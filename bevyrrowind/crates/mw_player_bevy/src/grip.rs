//! The hands' IK feed, the host side of two sources sharing the graph's [`HAND_L`] / [`HAND_R`]
//! IK targets and [`HAND_IK_L`] / [`HAND_IK_R`] weights, every frame after
//! [`crate::avatar::feed`]. A graph without those slots or parameters ignores them.
//!
//! - **Hands on the lip** (action.md B2b): a mantle's grip targets ([`sf_action::LedgeGrip`],
//!   planned with its warp) are fixed world points (the warp anchor plus the plan's offsets),
//!   weighted by the warp progress lerped like the drawn body (`ActionBody::warp_progress`).
//! - **Wall brace** (action.md B2c): [`sf_action::WallBrace`] probes the physics world from the
//!   drawn body's shoulders (last frame's model pose; estimated from the mover's height before
//!   the first pose) and plants a free hand on a wall within reach. Free: not the weapon hand
//!   while the weapon is drawn ([`Stance::Drawn`]: the one-handed sword is in the right hand);
//!   torches and carried items join when they exist. Runs only grounded in plain locomotion (no
//!   action move clip, no warp: [`brace_active`]).
//!
//! Per hand the ledge grip wins while its weight is above 0; otherwise the brace. Targets go into
//! the drawn body's character space (its feet and yaw, unscaled by the body's scale, as
//! [`crate::avatar::feed`] converts). With neither, the weight is 0 and the target cleared: these
//! slots belong to this feed.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_time::Fixed;
use bevy_time::prelude::*;
use bevy_transform::prelude::*;

use sf_action::{BraceBody, LEFT, LedgeGrip, RIGHT, Tick, WallBrace};
use sf_action_bevy::{ActionBody, ActionCharacter};
use sf_anim_bevy::Animator;
use sf_anim_graph::Op;
use sf_controller::CharacterFrame;
use sf_controller_bevy::Mover;
use sf_physics_bevy::Physics;

use mw_game_core::player::PlayerBody;

use crate::avatar::PlayerFeed;
use crate::combat::Stance;
use crate::dummy::TrainingDummy;

/// The graph's left-hand IK target slot.
pub const HAND_L: &str = mw_graphs::build::ik::HAND_L;
/// The graph's right-hand IK target slot.
pub const HAND_R: &str = mw_graphs::build::ik::HAND_R;
/// The left-hand IK weight parameter (float, 0-1).
pub const HAND_IK_L: &str = mw_graphs::build::params::HAND_IK_L;
/// The right-hand IK weight parameter (float, 0-1).
pub const HAND_IK_R: &str = mw_graphs::build::params::HAND_IK_R;

/// Feeds each body's grip or brace targets and weights to its animator (after [`crate::avatar::feed`]).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn feed(
    fixed: Res<Time<Fixed>>,
    time: Res<Time>,
    physics: Option<Res<Physics>>,
    players: Query<
        (Entity, &ActionBody, &ActionCharacter, &Transform, &PlayerFeed, Option<&Stance>),
        Or<(With<PlayerBody>, With<TrainingDummy>)>,
    >,
    mut animators: Query<(&mut Animator, Option<&Transform>)>,
    mut logged: Local<HashMap<Entity, Tick>>,
    mut braces: Local<HashMap<Entity, WallBrace>>,
) {
    let alpha = fixed.overstep_fraction().clamp(0.0, 1.0);
    braces.retain(|e, _| players.contains(*e));
    for (entity, body, character, transform, feed, stance) in &players {
        let Some((mut animator, body_transform)) = feed.animator.and_then(|e| animators.get_mut(e).ok()) else { continue };
        let a = &mut *animator;
        let grip = character.state.warp.and_then(|w| w.grip);
        // Once per mantle (frames outnumber ticks).
        if let Some(g) = grip
            && logged.insert(entity, character.state.started) != Some(character.state.started)
        {
            log_skipped(&g);
        }
        // This frame's warp progress (clip seconds), lerped like the drawn body.
        let t = body.warp_progress().map(|(prev, curr)| prev + (curr - prev) * alpha);
        let holds = match (grip, body.warp_anchor(), t) {
            (Some(g), Some(anchor), Some(t)) => g.at(t).map(|h| h.map(|(target, w)| (anchor.offset(target), w))),
            _ => [None, None],
        };
        let drawn = CharacterFrame::new(body.frame_origin.offset(transform.translation), Mover::yaw_of(transform.rotation));
        let scale = body_transform.map_or(Vec3::ONE, |t| t.scale);
        let inv = scale.recip();
        // The wall brace (character space, meters).
        let brace = physics.as_deref().map(|physics| {
            let mut b = brace_body(a, body, scale);
            b.desired = drawn.dir_to_local(character.output.intent.desired_velocity * Vec3::new(1.0, 0.0, 1.0));
            b.velocity = drawn.dir_to_local(body.loco.motion.velocity * Vec3::new(1.0, 0.0, 1.0));
            b.free = [true, stance != Some(&Stance::Drawn)];
            b.active = brace_active(body, character);
            braces.entry(entity).or_default().update(&drawn, physics, &b, time.delta_secs())
        });
        for (i, slot, param) in [(LEFT, HAND_L, HAND_IK_L), (RIGHT, HAND_R, HAND_IK_R)] {
            let hold = holds[i].filter(|(_, w)| *w > 0.0).map(|(world, w)| (drawn.to_local(world), w));
            let on_wall = brace.and_then(|b| b[i].target.map(|t| (t, b[i].weight))).filter(|(_, w)| *w > 0.0);
            let feed = hold.or(on_wall);
            if let Some(k) = a.asset.graph.ik_target(slot)
                && let Some(target) = a.drive.ik_targets.get_mut(k)
            {
                *target = feed.map(|(local, _)| (local * inv).into());
            }
            a.set_float(param, feed.map_or(0.0, |(_, w)| w));
        }
    }
}

/// The wall brace may run: grounded in plain locomotion, no action move's clip (a Clip move:
/// attack, roll, ...) and no warp (a traversal) playing. Not `ActionOutput::slot`: the base
/// locomotion and air rules carry slots too (`loco.move`, `air.fall`, ...), so it is always set;
/// the moves whose clip owns the body are the ones with a clip or warp output, as
/// [`crate::avatar`] plays them.
pub(crate) fn brace_active(body: &ActionBody, character: &ActionCharacter) -> bool {
    let out = &character.output;
    body.loco.grounded.is_some() && !body.loco.traversing && out.clip.is_none() && out.warp.is_none() && character.state.warp.is_none()
}

/// The brace's body from the animator's last model pose: each hand IK op's upper-arm joint as
/// the shoulder and its two bones as the arm (character space, times the body's `scale`), else
/// the estimate from the mover's height.
fn brace_body(a: &Animator, body: &ActionBody, scale: Vec3) -> BraceBody {
    let mut b = BraceBody::estimate(body.params.height);
    let model = &a.output.model;
    if !a.output.posed {
        return b;
    }
    for (i, slot) in [(LEFT, HAND_L), (RIGHT, HAND_R)] {
        let Some(k) = a.asset.graph.ik_target(slot) else { continue };
        let arm = a.asset.graph.ops.iter().find_map(|e| match &e.op {
            Op::LimbIk { limb, target, .. } if usize::from(*target) == k => Some(limb.solved()),
            _ => None,
        });
        if let Some(&[upper, lower, hand]) = arm
            && [upper, lower, hand].iter().all(|&j| usize::from(j) < model.mats.len())
        {
            let p = |j: u16| model.translation(usize::from(j)) * scale;
            b.shoulders[i] = p(upper);
            b.arms[i] = p(upper).distance(p(lower)) + p(lower).distance(p(hand));
        }
    }
    b
}

/// Logs the hands a new mantle's grip skipped (their baked contact too far from the lip).
fn log_skipped(grip: &LedgeGrip) {
    for (i, name) in [(LEFT, "left"), (RIGHT, "right")] {
        if let Some(h) = grip.skipped[i] {
            info!("mantle grip: {name} hand skipped, its clip contact lands {:.2} m from the lip target", h.gap);
        }
    }
}
