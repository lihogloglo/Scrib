//! NPC head tracking (`AN8b`): actors turn their heads to the player, Morrowind's way. The
//! humanoid graph's `LookAt` op (`AN8`) aims at [`sf_anim_bevy::AnimDrive::look_target`]; this
//! module is its only writer for NPCs.
//!
//! **Port.** `OpenMW` `apps/openmw/mwmechanics/actors.cpp` at commit
//! `b8285cc95d19e080fe0532f9f5154b0c3f727735` (`gitlab.com/OpenMW/openmw`): the two
//! `updateHeadTracking` overloads (lines 270-321, the per-target test; 323-363, the caller), their
//! refresh in `Actors::update` (1529-1530, 1638-1639: every 0.3 s for each actor whose AI is active
//! and in processing range) and `isTargetMagicallyHidden` (`actorutil.cpp:38-43`). The rule
//! ([`tracks`]):
//! - The tracker is not knocked down (nor unconscious), else it tracks nobody.
//! - The target is alive and not magically hidden (Invisibility above 0, or Chameleon at
//!   [`CHAMELEON_HIDES`] or more).
//! - In range: feet to feet within `fMaxHeadTrackDistance` (game units, [`HeadTrackRules`]), times
//!   `fInteriorHeadTrackMult` in an interior that does not behave like an exterior.
//! - In front: the target is in the tracker's front half-space, horizontally (facing dot offset > 0).
//! - Seen: `getLOS` and `awarenessCheck` (the target is not sneaking unseen).
//! - In combat or pursuit, the tracker only tracks its package's target, and range and the
//!   in-front test are skipped (LOS and awareness still apply).
//!
//! Both rule sets (Modern and Classic): this is Morrowind's own presentation, no seam.
//!
//! **Game mapping** ([`track_heads`], `PostUpdate` in `AnimSystems::Probe` after the controller's
//! feed). Trackers: every awake AI actor's mover (`AiActor` + `Mover`, not `Dormant`: `OpenMW`'s
//! `aiActive && inProcessingRange`) with an animator. The one candidate target is the host player
//! (no actor-to-actor tracking yet: other targets come with the AI lane), so `OpenMW`'s
//! nearest-wins fold over candidates (`sqrHeadTrackDistance`) is moot. "Seen" (LOS + awareness) is
//! the NPC's sight contact with the player within the last two sight periods
//! (`sf_ai_bevy::seen_recently`: our sight check casts the LOS ray and applies the crime stealth
//! rules, i.e. sneak, light and facing). "In combat or pursuit" is the Alerted band toward the
//! player (the latched band whose drives pick combat, report or flee) until NPCs carry combat
//! packages. Knocked down: [`crate::physics::KnockedDown`]; the target dead:
//! [`crate::physics::Dead`] on the player body. Magically hidden: the player sheet's
//! [`crate::visibility::CHAMELEON`] and [`INVISIBILITY`] magnitudes (absent from the stat table
//! until the effects adapter adds them: never hidden then). Interior: the NPC's worldspace in
//! [`crate::light_inputs::LightRecords::interiors`]. The choice refreshes every [`REFRESH_S`]
//! (one shared timer, as `OpenMW`'s); the aim follows the player's drawn head every frame.
//!
//! **Target and frame.** The player's head is its drawn feet (body `Transform` over its frame
//! origin, as the avatar feed draws it) raised by its `Perceivable::head_m` (the eye line,
//! [`crate::sight::HEAD_RATIO`] of the capsule height). It goes into the NPC's character space as
//! every other animation input does ([`to_character_space`]): the drawn root's frame (Y up, facing
//! +Z, [`CharacterFrame`]) divided by the body's scale (the unscaled skeleton space, as the feed
//! scales velocity and ground samples).

use std::sync::Arc;

use bevy_app::{App, PostUpdate};
use bevy_ecs::entity::EntityHashSet;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::Time;
use bevy_transform::components::Transform;
use mw_ai::wander::gmst_float;
use mw_game_core::player::{HOST_KEY, PlayerBody, PlayerCharacter};
use sf_action_bevy::ActionBody;
use sf_ai::{AwarenessBand, PerceptTarget};
use sf_ai_bevy::{AiActor, AiAwareness, AiPerceptClock, AiPerceptionSettings, Perceivable, seen_recently};
use sf_anim_bevy::{AnimSystems, Animator};
use sf_controller::CharacterFrame;
use sf_controller_bevy::{ControllerSystems, Dormant, Mover};
use sf_coords::WorldPos;
use sf_records::RecordDb;
use sf_stats_bevy::{StatTableRes, Stats};
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};

use crate::light_inputs::LightRecords;
use crate::physics::{Dead, KnockedDown};
use crate::walkers::ActorSpace;

/// `fMaxHeadTrackDistance` when the record is missing, game units (Morrowind.esm: 400).
pub const MAX_HEAD_TRACK_DISTANCE: f32 = 400.0; // tunables: const (vanilla GMST fallback; the game reads its records)
/// `fInteriorHeadTrackMult` when the record is missing (Morrowind.esm: 0.5).
pub const INTERIOR_HEAD_TRACK_MULT: f32 = 0.5; // tunables: const (vanilla GMST fallback; the game reads its records)
/// The stat table key of Invisibility's magnitude (the MGEF effect's name, as
/// [`crate::visibility::CHAMELEON`]; absent until the effects adapter adds magnitude stats).
pub const INVISIBILITY: &str = "invisibility";

// --- Tunables ---------------------------------------------------------------------------------

/// `mw.anim.head_track.on`: [`HeadTrackSettings::on`].
pub const ON: Def<bool> = def!(bool "mw.anim.head_track.on" = true)
    .help("NPCs turn their heads to the player (OpenMW head tracking); off clears every NPC's look target")
    .source(Source::Reference("OpenMW apps/openmw/mwmechanics/actors.cpp:323-363 (b8285cc9): always on"))
    .scope(Scope::Dev)
    .net(Net::Presentation)
    .lifetime(Lifetime::Live);

/// `mw.anim.head_track.refresh_s`: [`HeadTrackSettings::refresh_s`].
pub const REFRESH_S: Def<f32> = def!(float "mw.anim.head_track.refresh_s" = 0.3)
    .range(0.0, 5.0)
    .unit(Unit::Seconds)
    .help("How often each NPC re-chooses whom it head-tracks (the aim follows the target every frame)")
    .source(Source::Reference("OpenMW apps/openmw/mwmechanics/actors.cpp:1529 (b8285cc9) mTimerUpdateHeadTrack 0.3 s"))
    .scope(Scope::Dev)
    .net(Net::Presentation)
    .lifetime(Lifetime::Live);

/// `mw.anim.head_track.chameleon_hides`: [`HeadTrackRules::chameleon_hides`].
pub const CHAMELEON_HIDES: Def<f32> = def!(float "mw.anim.head_track.chameleon_hides" = 75.0)
    .range(0.0, 100.0)
    .unit(Unit::None)
    .help("Chameleon magnitude from which a target is magically hidden from head tracking")
    .source(Source::Reference("OpenMW apps/openmw/mwmechanics/actorutil.cpp:42 (b8285cc9) isTargetMagicallyHidden"))
    .scope(Scope::Dev)
    .net(Net::Presentation)
    .lifetime(Lifetime::Live);

/// The head tracking's live tunables (`mw.anim.head_track.*`).
pub const TUNABLES: &[&dyn AnyDef] = &[&ON, &REFRESH_S, &CHAMELEON_HIDES];

// --- Rule (Bevy-free) -------------------------------------------------------------------------

/// The rule's numbers: the two GMSTs (in meters) and the Chameleon threshold.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeadTrackRules {
    /// `fMaxHeadTrackDistance` in meters.
    pub max_distance_m: f32,
    /// `fInteriorHeadTrackMult`.
    pub interior_mult: f32,
    /// [`CHAMELEON_HIDES`].
    pub chameleon_hides: f32,
}

impl Default for HeadTrackRules {
    fn default() -> Self {
        Self {
            max_distance_m: mw_world::convert::meters(MAX_HEAD_TRACK_DISTANCE),
            interior_mult: INTERIOR_HEAD_TRACK_MULT,
            chameleon_hides: CHAMELEON_HIDES.default_value(),
        }
    }
}

impl HeadTrackRules {
    /// The rules with the content's GMSTs (vanilla values when a record is missing).
    pub fn from_records(db: &RecordDb) -> Self {
        Self {
            max_distance_m: mw_world::convert::meters(gmst_float(db, "fMaxHeadTrackDistance", MAX_HEAD_TRACK_DISTANCE)),
            interior_mult: gmst_float(db, "fInteriorHeadTrackMult", INTERIOR_HEAD_TRACK_MULT),
            ..Self::default()
        }
    }

    /// The tracking range of a tracker indoors or out, meters (`actors.cpp:296-299`).
    pub fn max_distance(&self, interior: bool) -> f32 {
        if interior { self.max_distance_m * self.interior_mult } else { self.max_distance_m }
    }

    /// `isTargetMagicallyHidden` (`actorutil.cpp:38-43`): Invisibility above 0 or Chameleon at the
    /// threshold or more.
    pub fn magically_hidden(&self, invisibility: f32, chameleon: f32) -> bool {
        invisibility > 0.0 || chameleon >= self.chameleon_hides
    }
}

/// The tracking actor this refresh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tracker {
    /// Its drawn root (feet) and facing.
    pub frame: CharacterFrame,
    /// Knocked down or unconscious.
    pub knocked_down: bool,
    /// In an interior that does not behave like an exterior.
    pub interior: bool,
    /// In combat or pursuit with this target (range and facing are skipped).
    pub engaged: bool,
}

/// A candidate target this refresh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeadCandidate {
    pub feet: WorldPos,
    pub dead: bool,
    /// Invisibility magnitude.
    pub invisibility: f32,
    /// Chameleon magnitude.
    pub chameleon: f32,
    /// Line of sight and awareness: the tracker perceives it (it is not sneaking unseen).
    pub seen: bool,
}

/// Whether `npc` head-tracks `target` (`actors.cpp:270-321` with the caller's knockdown gate,
/// 323-338; module docs).
pub fn tracks(npc: &Tracker, target: &HeadCandidate, rules: &HeadTrackRules) -> bool {
    if npc.knocked_down || target.dead || rules.magically_hidden(target.invisibility, target.chameleon) {
        return false;
    }
    let offset = npc.frame.position.delta_f32(target.feet);
    if !npc.engaged {
        let max = rules.max_distance(npc.interior);
        if offset.length_squared() > max * max {
            return false;
        }
        // In front, horizontally (`actors.cpp:308-313`).
        let forward = npc.frame.rotation() * Vec3::Z;
        if forward.with_y(0.0).dot(offset.with_y(0.0)) <= 0.0 {
            return false;
        }
    }
    target.seen
}

/// A world point in the character space of the root drawn at `frame` with body scale `scale`: the
/// space of [`sf_anim_bevy::AnimDrive`] (Y up, facing +Z, unscaled skeleton).
pub fn to_character_space(frame: &CharacterFrame, scale: Vec3, p: WorldPos) -> Vec3 {
    frame.to_local(p) * scale.recip()
}

/// `npc`'s look target at `head` (the target's head, world) if it tracks `target`, in its
/// character space (body scale `scale`); `None` if not.
pub fn head_target(npc: &Tracker, target: &HeadCandidate, head: WorldPos, scale: Vec3, rules: &HeadTrackRules) -> Option<Vec3> {
    tracks(npc, target, rules).then(|| to_character_space(&npc.frame, scale, head))
}

// --- Bevy -------------------------------------------------------------------------------------

/// The live settings: the rules from the records plus the tunables.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct HeadTrackSettings {
    /// [`ON`].
    pub on: bool,
    /// [`REFRESH_S`].
    pub refresh_s: f32,
    pub rules: HeadTrackRules,
}

impl Default for HeadTrackSettings {
    fn default() -> Self {
        Self { on: ON.default_value(), refresh_s: REFRESH_S.default_value(), rules: HeadTrackRules::default() }
    }
}

/// Who tracks the player, as of the last refresh, and the shared refresh timer.
#[derive(Default)]
pub struct HeadTrackState {
    timer_s: f32,
    tracking: EntityHashSet,
}

/// Adds head tracking with the content's GMSTs (called by `walkers::add`).
pub fn add(app: &mut App, db: &Arc<RecordDb>) {
    let rules = HeadTrackRules::from_records(db);
    app.insert_resource(HeadTrackSettings { rules, ..HeadTrackSettings::default() });
    let _on = app.add_tunable(&ON).bind_resource(|s: &mut HeadTrackSettings, v| s.on = v);
    let _refresh = app.add_tunable(&REFRESH_S).bind_resource(|s: &mut HeadTrackSettings, v| s.refresh_s = v);
    let _chameleon = app
        .add_tunable(&CHAMELEON_HIDES)
        .bind_resource(|s: &mut HeadTrackSettings, v| s.rules.chameleon_hides = v);
    app.add_systems(PostUpdate, track_heads.in_set(AnimSystems::Probe).after(ControllerSystems::Feed));
}

/// The host player as a candidate: its feet, its drawn head, dead, hidden magnitudes.
struct PlayerHead {
    feet: WorldPos,
    head: WorldPos,
    dead: bool,
    invisibility: f32,
    chameleon: f32,
}

/// Sets every awake NPC's look target (module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)] // system parameters
fn track_heads(
    time: Res<Time>,
    settings: Res<HeadTrackSettings>,
    clock: Option<Res<AiPerceptClock>>,
    perception: Option<Res<AiPerceptionSettings>>,
    lights: Option<Res<LightRecords>>,
    table: Option<Res<StatTableRes>>,
    mut state: Local<HeadTrackState>,
    players: Query<(&ActionBody, &Transform, &Perceivable, Has<Dead>), With<PlayerBody>>,
    sheets: Query<(&PlayerCharacter, &Stats)>,
    npcs: Query<(Entity, &Mover, Option<&AiAwareness>, Option<&ActorSpace>, Has<KnockedDown>, Has<Dormant>), With<AiActor>>,
    mut animators: Query<&mut Animator>,
) {
    let player = players.iter().next().map(|(body, transform, p, dead)| {
        let feet = body.frame_origin.offset(transform.translation);
        let (mut invisibility, mut chameleon) = (0.0, 0.0);
        let sheet = sheets.iter().find(|(c, _)| c.key == HOST_KEY).map(|(_, s)| s);
        if let (Some(t), Some(sheet)) = (table.as_deref(), sheet) {
            let read = sheet.view(&t.0);
            invisibility = t.0.find(INVISIBILITY).map_or(0.0, |i| read.modified(i));
            chameleon = t.0.find(crate::visibility::CHAMELEON).map_or(0.0, |i| read.modified(i));
        }
        PlayerHead { feet, head: feet.offset(Vec3::Y * p.head_m), dead, invisibility, chameleon }
    });
    // `Actors::update`: the timer wraps at the refresh, and the frame it reads 0 refreshes.
    if state.timer_s >= settings.refresh_s {
        state.timer_s = 0.0;
    }
    let refresh = state.timer_s == 0.0;
    state.timer_s += time.delta_secs();
    let target = PerceptTarget::Player(HOST_KEY);
    if refresh {
        let period = clock.as_deref().zip(perception.as_deref()).map(|(c, s)| (c.tick, s.budget.sight_period_ticks(c.tick_s)));
        let HeadTrackState { tracking, .. } = &mut *state;
        tracking.clear();
        if let Some(p) = player.as_ref().filter(|_| settings.on) {
            for (entity, mover, awareness, space, knocked_down, dormant) in &npcs {
                if dormant {
                    continue;
                }
                let seen = awareness
                    .zip(period)
                    .is_some_and(|(a, (tick, ticks))| seen_recently(&a.0, target, tick, ticks));
                let npc = Tracker {
                    frame: mover.drawn_frame(),
                    knocked_down,
                    interior: lights.as_deref().zip(space).is_some_and(|(l, s)| l.interiors.contains_key(&s.0)),
                    engaged: awareness.is_some_and(|a| a.0.band(target) == AwarenessBand::Alerted),
                };
                let candidate = HeadCandidate {
                    feet: p.feet,
                    dead: p.dead,
                    invisibility: p.invisibility,
                    chameleon: p.chameleon,
                    seen,
                };
                if tracks(&npc, &candidate, &settings.rules) {
                    tracking.insert(entity);
                }
            }
        }
    }
    for (entity, mover, _, _, _, dormant) in &npcs {
        let Some(mut animator) = mover.animator.and_then(|a| animators.get_mut(a).ok()) else { continue };
        let want = match &player {
            Some(p) if settings.on && !dormant && state.tracking.contains(&entity) => {
                Some(to_character_space(&mover.drawn_frame(), mover.body_scale, p.head))
            }
            _ => None,
        };
        if animator.drive.look_target != want {
            animator.drive.look_target = want;
        }
    }
}

#[cfg(test)]
#[path = "head_track_tests.rs"]
mod tests;
