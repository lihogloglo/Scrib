//! Hands on the lip (action.md B2b), the host side: a mantle's grip targets
//! ([`sf_action::LedgeGrip`], planned with its warp) fed to the avatar's graph as the
//! [`HAND_L`] / [`HAND_R`] IK targets and [`HAND_IK_L`] / [`HAND_IK_R`] weights, every frame
//! after [`crate::avatar::feed`]. A graph without those slots or parameters ignores them.
//!
//! The targets are fixed world points (the warp anchor plus the plan's offsets); each frame they
//! go into the drawn body's character space (its feet and yaw, unscaled by the body's
//! scale, as [`crate::avatar::feed`] converts), weighted by the warp progress lerped like the
//! drawn body (`ActionBody::warp_progress`). Off a gripping mantle the weights are 0 and the
//! targets cleared: these slots belong to this feed.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_time::Fixed;
use bevy_time::prelude::*;
use bevy_transform::prelude::*;

use sf_action::{LEFT, LedgeGrip, RIGHT, Tick};
use sf_action_bevy::{ActionBody, ActionCharacter};
use sf_anim_bevy::Animator;
use sf_controller::CharacterFrame;
use sf_controller_bevy::Mover;

use mw_game_core::player::PlayerBody;

use crate::avatar::PlayerFeed;
use crate::dummy::TrainingDummy;

/// The graph's left-hand IK target slot.
pub const HAND_L: &str = mw_graphs::build::ik::HAND_L;
/// The graph's right-hand IK target slot.
pub const HAND_R: &str = mw_graphs::build::ik::HAND_R;
/// The left-hand IK weight parameter (float, 0-1).
pub const HAND_IK_L: &str = mw_graphs::build::params::HAND_IK_L;
/// The right-hand IK weight parameter (float, 0-1).
pub const HAND_IK_R: &str = mw_graphs::build::params::HAND_IK_R;

/// Feeds each body's grip targets and weights to its animator (after [`crate::avatar::feed`]).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn feed(
    fixed: Res<Time<Fixed>>,
    players: Query<(Entity, &ActionBody, &ActionCharacter, &Transform, &PlayerFeed), Or<(With<PlayerBody>, With<TrainingDummy>)>>,
    mut animators: Query<(&mut Animator, Option<&Transform>)>,
    mut logged: Local<HashMap<Entity, Tick>>,
) {
    let alpha = fixed.overstep_fraction().clamp(0.0, 1.0);
    for (entity, body, character, transform, feed) in &players {
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
        let inv = body_transform.map_or(Vec3::ONE, |t| t.scale).recip();
        for (i, slot, param) in [(LEFT, HAND_L, HAND_IK_L), (RIGHT, HAND_R, HAND_IK_R)] {
            let hold = holds[i].filter(|(_, w)| *w > 0.0);
            if let Some(k) = a.asset.graph.ik_target(slot)
                && let Some(target) = a.drive.ik_targets.get_mut(k)
            {
                *target = hold.map(|(world, _)| (drawn.to_local(world) * inv).into());
            }
            a.set_float(param, hold.map_or(0.0, |(_, w)| w));
        }
    }
}

/// Logs the hands a new mantle's grip skipped (their baked contact too far from the lip).
fn log_skipped(grip: &LedgeGrip) {
    for (i, name) in [(LEFT, "left"), (RIGHT, "right")] {
        if let Some(h) = grip.skipped[i] {
            info!("mantle grip: {name} hand skipped, its clip contact lands {:.2} m from the lip target", h.gap);
        }
    }
}
