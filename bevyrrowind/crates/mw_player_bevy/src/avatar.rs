//! The player's visible body (superfargoth `docs/specs/action.md` A2 slice 2): the dressed
//! `player` NPC record (race, sex, head, hair, outfit) as an `sf_character_bevy` character under
//! the [`PlayerBody`], playing the humanoid graph. The capsule stays the gameplay shape, unseen.
//!
//! - **Placement.** The avatar is a child of the body entity, whose `Transform` is the fixed-tick
//!   capsule pose interpolated to the frame and offset by the render-body correction spring
//!   (`sf_action_bevy` `ActionSystems::Interpolate`): the drawn character follows the capsule
//!   with no per-frame jitter at any frame cap, and never feeds back into gameplay.
//! - **Feed** ([`feed`], `PostUpdate` in `AnimSystems::Probe`): the same locomotion parameters
//!   the NPC walkers' feed writes (`sf_controller_bevy::FeedValues`, the walkers'
//!   `ControllerSettings` thresholds), measured from the **drawn** root frame to frame (so the
//!   graph sees smooth motion between fixed ticks), plus `sneaking` and the air pair:
//!   `jumping` is the action layer's launch flag (`ActionState::body.launched`, set on the
//!   launch tick, cleared at touchdown), and `on_ground` is the mover's ground **debounced**
//!   ([`AirDebounce`]): a launch leaves the ground at once, a fall only after [`AIR_GRACE`] in
//!   the air or below [`FALL_SPEED`], so step-down misses on rocks and stairs don't flicker the
//!   fall pose (UE keeps Walking within the step height; Lyra/ALS gate the fall on air time).
//!   Foot probes are not fed yet: foot placement holds the animated pose.
//! - **Traversal and combat moves** (action.md B2a, combat.md C1b): while the action layer's Warp
//!   or Clip owner moves the body, the move's slot clip (`traverse.mantle.waist`, an attack, a roll,
//!   a hit reaction; [`AvatarSlots`]) plays in the graph's `full` montage slot with
//!   `RootMotionMode::Progress`; its playhead is the move's chain progress, interpolated between
//!   the last two ticks like the body. When the move ends the slot stops and the graph
//!   inertializes back to locomotion.
//! - **First person**: the camera fades the followed body (`CameraPose::fade`); the avatar is
//!   hidden while it is mostly faded (no first-person arms yet).

use bevy_camera::prelude::Visibility;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_time::prelude::*;
use bevy_time::Fixed;
use bevy_transform::prelude::*;
use std::collections::HashMap;

use sf_action::{RuleKey, SlotKey};
use sf_action_bevy::{ActionBody, ActionCharacter};
use sf_anim::{RootDelta, RootMotionMode};
use sf_anim_bevy::{AnimLod, AnimTier, Animator};
use sf_anim_graph::{MontageClip, SlotId, SlotRequest};
use sf_camera_bevy::PlayerCamera;
use sf_character_bevy::CharacterBody;
use sf_controller::{CharacterFrame, Gait};
use sf_controller_bevy::{ControllerSettings, FeedGait, FeedValues, Mover};

use mw_game_core::player::{PlayerAvatar, PlayerBody};

use crate::dummy::{DummyAvatar, TrainingDummy};
use mw_graphs::build::params as gp;

/// The montage slot traversal clips play in.
const TRAVERSAL_SLOT: &str = mw_graphs::build::slots::FULL;
/// Inertialization into a traversal clip and back to locomotion, seconds.
const TRAVERSAL_BLEND_IN: f32 = 0.12;
const TRAVERSAL_BLEND_OUT: f32 = 0.25;
/// Inertialization into a combat clip (sheet `defaults.blend_in`, combat.md "C1b"), seconds.
const CLIP_BLEND_IN: f32 = 0.10;
/// A drawn root that moved further than this in a frame was teleported (door, load): the feed
/// restarts instead of reporting the jump as motion, meters.
const TELEPORT_M: f32 = 3.0;
/// The character kind of `NPC_` records (`mw_actors::source::NPC_KIND`).
const NPC_KIND: &str = "NPC_";
/// The avatar hides while the camera fades the body more than this (first person).
const HIDE_FADE: f32 = 0.5;
/// Unlaunched air time the animation ignores (a step-down miss), seconds.
pub const AIR_GRACE: f32 = 0.1;
/// A vertical speed below this is a real fall even within [`AIR_GRACE`], m/s.
pub const FALL_SPEED: f32 = -2.0;

/// The animation's view of the ground: the mover's fixed-tick ground with the short airtimes of
/// a step-down miss filtered out. Counted in frame time, so frame-rate independent.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AirDebounce {
    /// Seconds airborne since the mover last stood.
    air: f32,
    /// Reported airborne; holds until touchdown.
    airborne: bool,
}

impl AirDebounce {
    /// `on_ground` for the graph this frame. `grounded`: the mover stands (or a traversal holds
    /// it); `launched`: this airtime began with a launch; `vertical_speed`, m/s; `dt`, seconds.
    /// Touchdown reports grounded at once, a launch airborne at once; any other airtime reports
    /// airborne once longer than [`AIR_GRACE`] or faster down than [`FALL_SPEED`].
    pub fn on_ground(&mut self, grounded: bool, launched: bool, vertical_speed: f32, dt: f32) -> bool {
        if grounded {
            *self = Self::default();
            return true;
        }
        self.air += dt.max(0.0);
        self.airborne |= launched || self.air > AIR_GRACE || vertical_speed < FALL_SPEED;
        !self.airborne
    }
}

/// On the player body: its animator and the feed's state between frames.
#[derive(Component, Debug, Default)]
pub struct PlayerFeed {
    /// The avatar's `Animator` entity (`CharacterBody::body`), once dressed.
    pub animator: Option<Entity>,
    last: Option<CharacterFrame>,
    measured: Vec3,
    turn: f32,
    gait: FeedGait,
    traversal: Option<Traversal>,
    air: AirDebounce,
}

/// A move's slot clip playing in the montage slot (traversal or combat).
#[derive(Clone, Copy, Debug)]
struct Traversal {
    key: SlotKey,
    slot: SlotId,
    /// The move and its chain index the clip belongs to.
    rule: RuleKey,
    chain: u8,
    /// Clip seconds of the move's earlier chain slots (subtracted from its chain progress).
    offset: f32,
}

/// The action slots the avatar plays from a Warp or Clip move: their graph slot names and
/// clip lengths (clip seconds; a chain's next slot starts after them). Default: every player
/// slot ([`crate::player_slots`]) with no lengths.
#[derive(Resource, Clone, Debug)]
pub struct AvatarSlots {
    names: HashMap<SlotKey, String>,
    durations: HashMap<SlotKey, f32>,
}

impl Default for AvatarSlots {
    fn default() -> Self {
        Self::new(crate::player_slots(), [])
    }
}

impl AvatarSlots {
    pub fn new(names: impl IntoIterator<Item = String>, durations: impl IntoIterator<Item = (String, f32)>) -> Self {
        Self {
            names: names.into_iter().map(|n| (SlotKey::of(&n), n)).collect(),
            durations: durations.into_iter().map(|(n, d)| (SlotKey::of(&n), d)).collect(),
        }
    }

    fn name(&self, key: SlotKey) -> Option<&str> {
        self.names.get(&key).map(String::as_str)
    }

    fn duration(&self, key: SlotKey) -> f32 {
        self.durations.get(&key).copied().unwrap_or(0.0)
    }
}

impl PlayerFeed {
    /// The traversal slot playing (its action slot key), if any.
    pub fn traversal(&self) -> Option<SlotKey> {
        self.traversal.map(|t| t.key)
    }
}

/// The dressed character under the body (`player` record), tagged [`PlayerAvatar`].
pub(crate) fn avatar_bundle() -> impl Bundle {
    (
        Name::new("player avatar"),
        sf_character_bevy::Character::record(NPC_KIND, sf_records::RecordId::new(crate::hud::PLAYER_RECORD)),
        PlayerAvatar,
        Transform::IDENTITY,
        Visibility::Inherited,
    )
}

/// Points the body's feed at its avatar's animator once the character is built (and after an
/// outfit swap), and pins the player's animation tier to every-frame updates.
#[allow(clippy::type_complexity)]
pub(crate) fn link(
    avatars: Query<(&ChildOf, &CharacterBody), (Or<(With<PlayerAvatar>, With<DummyAvatar>)>, Changed<CharacterBody>)>,
    mut feeds: Query<&mut PlayerFeed>,
    mut lods: Query<&mut AnimLod>,
) {
    for (parent, body) in &avatars {
        if let Ok(mut feed) = feeds.get_mut(parent.parent()) {
            feed.animator = Some(body.body);
        }
        if let Ok(mut lod) = lods.get_mut(body.body) {
            lod.forced = Some(AnimTier::A0);
        }
    }
}

/// The graph parameters, drive and traversal slot from the body (see the module docs).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn feed(
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    settings: Option<Res<ControllerSettings>>,
    mut players: Query<(&ActionBody, &ActionCharacter, &Transform, &mut PlayerFeed), Or<(With<PlayerBody>, With<TrainingDummy>)>>,
    slots: Option<Res<AvatarSlots>>,
    builtin: Local<AvatarSlots>,
    mut animators: Query<(&mut Animator, Option<&Transform>)>,
) {
    let slots = slots.as_deref().unwrap_or(&builtin);
    let fallback = ControllerSettings::default();
    let settings = settings.as_deref().unwrap_or(&fallback);
    let dt = time.delta_secs();
    let alpha = fixed.overstep_fraction().clamp(0.0, 1.0);
    for (body, character, transform, mut feed) in &mut players {
        let feed = &mut *feed;
        let Some((mut animator, body_transform)) = feed.animator.and_then(|e| animators.get_mut(e).ok()) else {
            feed.last = None;
            continue;
        };
        // Drawn root this frame, and how it moved since the last one (its frame then).
        let drawn = CharacterFrame::new(body.frame_origin.offset(transform.translation), Mover::yaw_of(transform.rotation));
        let moved = match feed.last {
            Some(last) if dt > 0.0 => {
                let d = last.delta_to(&drawn);
                if d.translation.length() > TELEPORT_M { RootDelta::ZERO } else { d }
            }
            _ => RootDelta::ZERO,
        };
        feed.last = Some(drawn);
        if dt > 0.0 {
            let raw = (Quat::from_rotation_y(-moved.yaw) * moved.translation / dt).with_y(0.0);
            let turn = moved.yaw / dt;
            let keep = 0.5f32.powf(dt / crate::feel::MEASURE_HALFLIFE);
            if raw.is_finite() && turn.is_finite() {
                feed.measured = raw.lerp(feed.measured, keep);
                feed.turn = turn + (feed.turn - turn) * keep;
            }
        }
        let intent = &character.output.intent;
        let braking = intent.desired_velocity.length_squared() < 1e-6;
        let v = FeedValues::measured(feed.measured, feed.turn, braking, intent.gait, settings, feed.gait);
        feed.gait = v.gait();
        // Mover space -> the body's unscaled skeleton space (race height and weight).
        let inv = body_transform.map_or(Vec3::ONE, |t| t.scale).recip();
        let a = &mut *animator;
        a.drive.velocity = v.velocity * inv;
        a.drive.acceleration = drawn.dir_to_local(body.loco.motion.acceleration) * inv;
        a.drive.turn_rate = v.turn;
        let mut moved = moved;
        moved.translation *= inv;
        a.drive.moved = a.drive.moved.then(moved);
        a.set_bool(gp::MOVING, v.moving);
        a.set_vec2(gp::MOVE, v.direction);
        a.set_bool(gp::RUNNING, v.running);
        a.set_float(gp::TURN, v.turn);
        a.set_bool(gp::SNEAKING, intent.gait == Gait::Sneak);
        // A warped traversal holds the body off the ground on purpose: not a jump.
        let loco = &body.loco;
        let launched = character.state.body.launched;
        let on_ground = feed.air.on_ground(loco.grounded.is_some() || loco.traversing, launched, loco.vertical_speed, dt);
        a.set_bool(gp::ON_GROUND, on_ground);
        // The flag clears on the tick after touchdown: report it with the ground, not after.
        a.set_bool(gp::JUMPING, launched && !on_ground);
        a.drive.ground.fill(None);
        traversal(feed, a, character, slots, body.warp_progress(), alpha);
    }
}

/// Starts, advances or ends the slot clip of the current Warp or Clip move. `ticks` is the
/// body's move progress at its previous and current tick (`ActionBody::warp_progress`): lerped
/// by the same `alpha` as the drawn body, the clip stays on it however many ticks ran. A Clip
/// move's progress runs over its whole slot chain (attack, then recovery): each slot plays from
/// the chain progress minus the clips before it (`slots`' durations).
pub(crate) fn traversal(
    feed: &mut PlayerFeed,
    a: &mut Animator,
    character: &ActionCharacter,
    slots: &AvatarSlots,
    ticks: Option<(f32, f32)>,
    alpha: f32,
) {
    let out = &character.output;
    let (moving, blend_in) = match (out.warp, out.clip) {
        (Some(w), _) => (Some(w.progress), TRAVERSAL_BLEND_IN),
        (None, Some(c)) => (Some(c.progress), CLIP_BLEND_IN),
        (None, None) => (None, CLIP_BLEND_IN),
    };
    let progress = |fallback: f32, offset: f32| {
        let (prev, curr) = ticks.unwrap_or((fallback, fallback));
        (prev + (curr - prev) * alpha - offset).max(0.0)
    };
    match (moving.zip(out.slot), feed.traversal.as_mut()) {
        (Some((p, key)), Some(t)) if t.key == key && t.rule == out.rule => {
            a.instance.set_slot_progress(t.slot, progress(p, t.offset));
        }
        (Some((p, key)), current) => {
            let current = current.map(|t| *t);
            // The next slot of the same move's chain starts where the previous clip ended.
            let offset = match current {
                Some(t) if t.rule == out.rule && !out.started && out.chain > t.chain => t.offset + slots.duration(t.key),
                _ => 0.0,
            };
            let previous = current.map(|t| t.slot);
            feed.traversal = start(a, slots, key, blend_in).map(|slot| {
                a.instance.set_slot_progress(slot, progress(p, offset));
                Traversal { key, slot, rule: out.rule, chain: out.chain, offset }
            });
            if feed.traversal.is_none()
                && let Some(slot) = previous
            {
                stop(a, slot);
            }
        }
        (None, Some(t)) => {
            let slot = t.slot;
            stop(a, slot);
            feed.traversal = None;
        }
        (None, None) => {}
    }
}

/// Plays the graph's clip for action slot `key` (one of [`AvatarSlots`]) in the full-body
/// montage slot, driven by progress. A new clip in the same montage slot inertializes from the
/// one playing.
fn start(a: &mut Animator, slots: &AvatarSlots, key: SlotKey, blend_in: f32) -> Option<SlotId> {
    let name = slots.name(key)?;
    let asset = a.asset.clone();
    let graph = &asset.graph;
    let (Some(slot), Some(clip)) = (graph.slot(TRAVERSAL_SLOT), graph.slot_clip(name)) else {
        bevy_log::warn_once!("player avatar: the graph has no {TRAVERSAL_SLOT} slot or no clip for {name}");
        return None;
    };
    let req = SlotRequest {
        root_motion: RootMotionMode::Progress,
        blend_in,
        blend_out: TRAVERSAL_BLEND_OUT,
        priority: 1,
        ..SlotRequest::new(MontageClip::Bound(clip))
    };
    match a.instance.play_slot(&asset.binding(), slot, req) {
        Ok(_) => Some(slot),
        Err(e) => {
            bevy_log::warn!("player avatar: {name} did not start ({e})");
            None
        }
    }
}

fn stop(a: &mut Animator, slot: SlotId) {
    let asset = a.asset.clone();
    a.instance.stop_slot(&asset.binding(), slot, TRAVERSAL_BLEND_OUT);
}

/// Hides the avatar while the camera fades the body (first person).
pub(crate) fn first_person(cams: Query<&PlayerCamera>, mut avatars: Query<&mut Visibility, With<PlayerAvatar>>) {
    let fade = cams.iter().find_map(|c| c.pose().map(|p| p.fade)).unwrap_or(0.0);
    let want = if fade > HIDE_FADE { Visibility::Hidden } else { Visibility::Inherited };
    for mut v in &mut avatars {
        v.set_if_neq(want);
    }
}
