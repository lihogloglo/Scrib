//! The player makes noise NPCs can hear (`AI8c2`; ai.md "Stimuli" → "Noise"): footsteps by gait and
//! landings, written as `sf_ai_bevy` [`NoiseEvent`]s from gameplay state on the fixed tick, after
//! the player moves ([`ActionSystems::Move`]) and after the sight chain advanced the perception clock
//! ([`AiPerceptionSystems::Sight`]), before [`AiPerceptionSystems::Hearing`]: heard the same tick,
//! stamped with this tick. Nothing here reads audio or animation (`mw_audio_bevy` plays footsteps
//! from the clips' `SoundGen` keys: presentation, never fed back into gameplay).
//!
//! - **Signals.** The player body's (`PlayerBody`) action body after its mover step: feet
//!   (`Locomotion::position`), ground contact (`grounded`, `mode` Ground), the airborne vertical
//!   speed; and its chooser's gait this tick (`ActionCharacter::output.intent.gait`: Sneak in sneak
//!   mode, Sprint in free run, Walk or Run by stick length) and gravity (`params.gravity`).
//! - **Stride clock** ([`StrideClock`]): while grounded on foot, the horizontal distance the feet
//!   moved advances a stride phase by `distance / stride_m(gait)`; each whole stride (one footfall)
//!   emits one footstep at the feet with the gait's reach. A gait change keeps the phase (the
//!   stride's fraction), so it never bursts steps. A move longer than one stride in one tick is a
//!   teleport (a door, a load, a script), not a walk: no step, the phase restarts. Standing still
//!   emits nothing. Airborne, flying, swimming or no-clip: no steps.
//! - **Landing.** Airborne → grounded on foot emits one landing noise at the feet, its reach from the
//!   drop height the impact speed is worth (`v² / 2g`, the impact energy per unit weight; `v` the
//!   last airborne tick's vertical speed, as `sf_action`'s `BodyMemory::air_vspeed`): none below
//!   [`LAND_MIN_DROP_M`], else [`LAND_BASE_M`] plus [`LAND_PER_DROP`] per meter of drop above it,
//!   capped at [`LAND_MAX_M`]. The stride restarts on landing.
//! - **Reveal.** Footsteps and landings reveal their source: `source` the host player
//!   ([`PerceptTarget::Player`] of [`HOST_KEY`]), `reveals_source` (the core raises the player's
//!   entry, capped at `noise_cap`: hearing alone never alerts).
//! - **Kinds** ([`PlayerNoiseKinds`]): `NoiseKind` records don't exist yet; the kinds are a small
//!   table ([`KIND_NAMES`]) resolved to [`NoiseKindId`]s once at start (their index). Their reach is
//!   the `mw.ai.noise.*` tunables.
//! - **Surface and gear** ([`surface_gear_mul`]): the reach's surface (the ground's `SurfaceId`) and
//!   gear (boots, armour weight) factor is a seam, 1 today.
//!
//! Not covered: NPC footsteps (NPCs as sources, `Perceivable` actors), surface and gear factors,
//! combat and action-state noise (`noise: Loud`), object noises (IN18), swimming, the player's
//! sneak skill (the core's `DetectionRules`, `AI8d`).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems};
use sf_ai::{NoiseKindId, PerceptTarget};
use sf_ai_bevy::{AiPerceptClock, AiPerceptionSystems, NoiseEvent};
use sf_controller::{Gait, Locomotion, MoverMode, SurfaceId};
use sf_coords::WorldPos;
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};

// --- Tunables ---------------------------------------------------------------------------------

macro_rules! noise_def {
    ($name:literal = $v:expr, $lo:expr, $hi:expr, $unit:expr, $help:literal, $source:expr) => {
        def!(float $name = $v)
            .range($lo, $hi)
            .unit($unit)
            .help($help)
            .source($source)
            .scope(Scope::Dev)
            .net(Net::Gameplay)
            .lifetime(Lifetime::Live)
    };
}

/// `mw.ai.noise.sneak_step_m`: a sneaking footstep's open-air reach.
pub const SNEAK_STEP_M: Def<f32> = noise_def!("mw.ai.noise.sneak_step_m" = 2.0, 0.0, 50.0, Unit::Meters,
    "Open-air reach of a sneaking footstep (before the listener's hearing and the weather)", Source::Guess);
/// `mw.ai.noise.walk_step_m`: a walking footstep's open-air reach.
pub const WALK_STEP_M: Def<f32> = noise_def!("mw.ai.noise.walk_step_m" = 6.0, 0.0, 50.0, Unit::Meters,
    "Open-air reach of a walking footstep", Source::Guess);
/// `mw.ai.noise.run_step_m`: a running footstep's open-air reach.
pub const RUN_STEP_M: Def<f32> = noise_def!("mw.ai.noise.run_step_m" = 12.0, 0.0, 80.0, Unit::Meters,
    "Open-air reach of a running footstep", Source::Guess);
/// `mw.ai.noise.sprint_step_m`: a sprinting footstep's open-air reach.
pub const SPRINT_STEP_M: Def<f32> = noise_def!("mw.ai.noise.sprint_step_m" = 18.0, 0.0, 80.0, Unit::Meters,
    "Open-air reach of a sprinting (free-run) footstep", Source::Guess);

/// `mw.ai.noise.sneak_stride_m`: feet travel per sneaking footfall.
pub const SNEAK_STRIDE_M: Def<f32> = noise_def!("mw.ai.noise.sneak_stride_m" = 0.5, 0.1, 3.0, Unit::Meters,
    "Distance the feet travel per footfall while sneaking (crouched, short steps)", Source::Guess);
/// `mw.ai.noise.walk_stride_m`: feet travel per walking footfall.
pub const WALK_STRIDE_M: Def<f32> = noise_def!("mw.ai.noise.walk_stride_m" = 0.75, 0.1, 3.0, Unit::Meters,
    "Distance the feet travel per footfall while walking",
    Source::Reference("walking step length about 0.415 x stature (Dean 1965, Grieve and Gear 1966): 0.75 m at 1.8 m"));
/// `mw.ai.noise.run_stride_m`: feet travel per running footfall.
pub const RUN_STRIDE_M: Def<f32> = noise_def!("mw.ai.noise.run_stride_m" = 1.5, 0.1, 4.0, Unit::Meters,
    "Distance the feet travel per footfall while running",
    Source::Reference("running cadence about 170-180 steps/min (Heiderscheit 2011): 4.5 m/s / 2.9 Hz = 1.5 m"));
/// `mw.ai.noise.sprint_stride_m`: feet travel per sprinting footfall.
pub const SPRINT_STRIDE_M: Def<f32> = noise_def!("mw.ai.noise.sprint_stride_m" = 2.0, 0.1, 4.0, Unit::Meters,
    "Distance the feet travel per footfall while sprinting",
    Source::Reference("sprint step length about 2 m at 7 m/s, 3.5 Hz (Weyand 2000 sprint cadence)"));

/// `mw.ai.noise.land_min_drop_m`: the smallest drop that makes a landing noise.
pub const LAND_MIN_DROP_M: Def<f32> = noise_def!("mw.ai.noise.land_min_drop_m" = 0.5, 0.0, 5.0, Unit::Meters,
    "Drop height (from the impact speed, v^2 / 2g) below which a landing makes no noise (a hop down a step)",
    Source::Guess);
/// `mw.ai.noise.land_base_m`: a landing's reach at the minimum drop.
pub const LAND_BASE_M: Def<f32> = noise_def!("mw.ai.noise.land_base_m" = 8.0, 0.0, 80.0, Unit::Meters,
    "Open-air reach of a landing from the minimum drop", Source::Guess);
/// `mw.ai.noise.land_per_drop`: extra landing reach per meter of drop.
pub const LAND_PER_DROP: Def<f32> = noise_def!("mw.ai.noise.land_per_drop" = 4.0, 0.0, 30.0, Unit::Ratio,
    "Extra landing reach, meters per meter of drop above the minimum (impact energy grows with the drop)",
    Source::Guess);
/// `mw.ai.noise.land_max_m`: a landing's largest reach.
pub const LAND_MAX_M: Def<f32> = noise_def!("mw.ai.noise.land_max_m" = 30.0, 0.0, 120.0, Unit::Meters,
    "Open-air reach cap of a landing (a long fall)", Source::Guess);

/// The player noise emitters' live tunables (`mw.ai.noise.*`).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &SNEAK_STEP_M,
    &WALK_STEP_M,
    &RUN_STEP_M,
    &SPRINT_STEP_M,
    &SNEAK_STRIDE_M,
    &WALK_STRIDE_M,
    &RUN_STRIDE_M,
    &SPRINT_STRIDE_M,
    &LAND_MIN_DROP_M,
    &LAND_BASE_M,
    &LAND_PER_DROP,
    &LAND_MAX_M,
];

/// One gait's footstep: its reach and the feet travel per footfall, meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StepNoise {
    pub reach_m: f32,
    pub stride_m: f32,
}

/// Values bound from [`TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct NoiseSettings {
    pub sneak: StepNoise,
    pub walk: StepNoise,
    pub run: StepNoise,
    pub sprint: StepNoise,
    pub land_min_drop_m: f32,
    pub land_base_m: f32,
    pub land_per_drop: f32,
    pub land_max_m: f32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        let step = |r: &Def<f32>, s: &Def<f32>| StepNoise { reach_m: r.default_value(), stride_m: s.default_value() };
        Self {
            sneak: step(&SNEAK_STEP_M, &SNEAK_STRIDE_M),
            walk: step(&WALK_STEP_M, &WALK_STRIDE_M),
            run: step(&RUN_STEP_M, &RUN_STRIDE_M),
            sprint: step(&SPRINT_STEP_M, &SPRINT_STRIDE_M),
            land_min_drop_m: LAND_MIN_DROP_M.default_value(),
            land_base_m: LAND_BASE_M.default_value(),
            land_per_drop: LAND_PER_DROP.default_value(),
            land_max_m: LAND_MAX_M.default_value(),
        }
    }
}

impl NoiseSettings {
    /// The footstep of `gait`.
    pub fn step(&self, gait: Gait) -> StepNoise {
        match gait {
            Gait::Sneak => self.sneak,
            Gait::Walk => self.walk,
            Gait::Run => self.run,
            Gait::Sprint => self.sprint,
        }
    }

    /// A landing's reach at `impact_mps` (downward speed, m/s) under `gravity` (m/s²); `None`: too
    /// soft to be heard (module docs).
    pub fn landing_m(&self, impact_mps: f32, gravity: f32) -> Option<f32> {
        if !impact_mps.is_finite() || !gravity.is_finite() || gravity <= 0.0 {
            return None;
        }
        let drop_m = impact_mps * impact_mps / (2.0 * gravity);
        (drop_m >= self.land_min_drop_m)
            .then(|| (self.land_base_m + self.land_per_drop * (drop_m - self.land_min_drop_m)).min(self.land_max_m))
    }
}

// --- Kinds ------------------------------------------------------------------------------------

/// The player's noise kinds, in [`NoiseKindId`] order (the table [`PlayerNoiseKinds`] resolves at
/// start until `NoiseKind` records exist).
// tunables: const (an id table, not a trade-off)
pub const KIND_NAMES: [&str; 5] = ["step.sneak", "step.walk", "step.run", "step.sprint", "land"];

/// The player's noise kinds as [`NoiseKindId`]s.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PlayerNoiseKinds {
    pub sneak_step: NoiseKindId,
    pub walk_step: NoiseKindId,
    pub run_step: NoiseKindId,
    pub sprint_step: NoiseKindId,
    pub land: NoiseKindId,
}

impl PlayerNoiseKinds {
    /// The id of a [`KIND_NAMES`] entry.
    pub fn resolve(name: &str) -> Option<NoiseKindId> {
        let i = KIND_NAMES.iter().position(|&n| n == name)?;
        Some(NoiseKindId(u16::try_from(i).ok()?))
    }

    /// The footstep kind of `gait`.
    pub fn step(&self, gait: Gait) -> NoiseKindId {
        match gait {
            Gait::Sneak => self.sneak_step,
            Gait::Walk => self.walk_step,
            Gait::Run => self.run_step,
            Gait::Sprint => self.sprint_step,
        }
    }
}

impl Default for PlayerNoiseKinds {
    fn default() -> Self {
        let id = |name| Self::resolve(name).expect("a KIND_NAMES entry");
        Self {
            sneak_step: id("step.sneak"),
            walk_step: id("step.walk"),
            run_step: id("step.run"),
            sprint_step: id("step.sprint"),
            land: id("land"),
        }
    }
}

/// The reach factor of the ground's surface and the body's gear (the seam ai.md "Noise" names:
/// `loudness_m` = reach × surface and gear): 1 until surfaces and gear carry noise data.
pub fn surface_gear_mul(_surface: SurfaceId) -> f32 {
    // tunables: const (the seam's identity: no surface or gear noise data yet)
    1.0
}

// --- Emitter ----------------------------------------------------------------------------------

/// A body's gameplay stride clock and air memory (module docs).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct StrideClock {
    /// Fraction of the current stride walked, 0..1.
    pub phase: f32,
    /// Feet at the last tick.
    pub prev: Option<WorldPos>,
    /// Airborne at the last tick.
    pub airborne: bool,
    /// The last airborne tick's vertical speed, m/s (negative falling): the impact speed.
    pub air_vspeed: f32,
}

/// The noise a body made this tick (its reach comes beside it, before the surface factor).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Made {
    Step(Gait),
    Land,
}

impl StrideClock {
    /// Advances the clock to the body's feet after its mover step: on foot is grounded in
    /// `MoverMode::Ground`, airborne is not grounded in `Ground` (flying, swimming and no-clip are
    /// neither: the clock just follows). `gait` is the chooser's this tick, `gravity` the body's
    /// (m/s²). Returns the noise this tick makes and its reach.
    pub fn tick(&mut self, settings: &NoiseSettings, loco: &Locomotion, gait: Gait, gravity: f32) -> Option<(Made, f32)> {
        let feet = loco.position;
        let ground = matches!(loco.mode, MoverMode::Ground);
        let on_foot = ground && loco.grounded.is_some();
        let airborne = ground && loco.grounded.is_none();
        let prev = self.prev.replace(feet);
        if airborne {
            self.airborne = true;
            self.air_vspeed = loco.vertical_speed;
            return None;
        }
        let landed = std::mem::take(&mut self.airborne);
        if !on_foot {
            self.phase = 0.0;
            return None;
        }
        if landed {
            self.phase = 0.0;
            return settings.landing_m(-self.air_vspeed, gravity).map(|m| (Made::Land, m));
        }
        let step = settings.step(gait);
        let moved = prev.map_or(0.0, |p| {
            let d = p.delta_f32(feet);
            d.x.hypot(d.z)
        });
        if step.stride_m <= 0.0 || !moved.is_finite() || moved > step.stride_m {
            // A teleport (module docs) or a broken stride: restart.
            self.phase = 0.0;
            return None;
        }
        self.phase += moved / step.stride_m;
        if self.phase < 1.0 {
            return None;
        }
        self.phase = self.phase.fract();
        Some((Made::Step(gait), step.reach_m))
    }
}

/// Gives a new player body its stride clock.
#[allow(clippy::type_complexity)]
fn clock_player(mut commands: Commands, bodies: Query<Entity, (With<PlayerBody>, With<ActionBody>, Without<StrideClock>)>) {
    for e in &bodies {
        commands.entity(e).try_insert(StrideClock::default());
    }
}

/// The player's footsteps and landings, this tick (module docs).
fn emit_player_noise(
    settings: Res<NoiseSettings>,
    kinds: Res<PlayerNoiseKinds>,
    clock: Option<Res<AiPerceptClock>>,
    mut bodies: Query<(&ActionBody, &ActionCharacter, &mut StrideClock), With<PlayerBody>>,
    mut noise: MessageWriter<NoiseEvent>,
) {
    let tick = clock.map_or(0, |c| c.tick);
    for (body, character, mut stride) in &mut bodies {
        let loco = &body.loco;
        let made = stride.tick(&settings, loco, character.output.intent.gait, character.params.gravity);
        let Some((made, reach_m)) = made else { continue };
        let surface = loco.grounded.map_or(SurfaceId::DEFAULT, |g| g.surface);
        noise.write(NoiseEvent {
            tick,
            at: loco.position,
            space: None,
            kind: match made {
                Made::Step(g) => kinds.step(g),
                Made::Land => kinds.land,
            },
            loudness_m: reach_m * surface_gear_mul(surface),
            source: Some(PerceptTarget::Player(HOST_KEY)),
            reveals_source: true,
            notify: None,
        });
    }
}

/// Adds the player noise emitters (called by [`crate::sight::add`], after the hearing plugin):
/// settings and their tunables, the kinds table, the stride clock and the emitter.
pub fn add(app: &mut App) {
    app.init_resource::<NoiseSettings>().init_resource::<PlayerNoiseKinds>().add_systems(
        FixedUpdate,
        (clock_player, emit_player_noise)
            .chain()
            .after(ActionSystems::Move)
            .after(AiPerceptionSystems::Sight)
            .before(AiPerceptionSystems::Hearing),
    );
    macro_rules! bind {
        ($def:expr, |$s:ident, $v:ident| $set:expr) => {
            let _ = app.add_tunable(&$def).bind_resource(|$s: &mut NoiseSettings, $v| $set);
        };
    }
    bind!(SNEAK_STEP_M, |s, v| s.sneak.reach_m = v);
    bind!(WALK_STEP_M, |s, v| s.walk.reach_m = v);
    bind!(RUN_STEP_M, |s, v| s.run.reach_m = v);
    bind!(SPRINT_STEP_M, |s, v| s.sprint.reach_m = v);
    bind!(SNEAK_STRIDE_M, |s, v| s.sneak.stride_m = v);
    bind!(WALK_STRIDE_M, |s, v| s.walk.stride_m = v);
    bind!(RUN_STRIDE_M, |s, v| s.run.stride_m = v);
    bind!(SPRINT_STRIDE_M, |s, v| s.sprint.stride_m = v);
    bind!(LAND_MIN_DROP_M, |s, v| s.land_min_drop_m = v);
    bind!(LAND_BASE_M, |s, v| s.land_base_m = v);
    bind!(LAND_PER_DROP, |s, v| s.land_per_drop = v);
    bind!(LAND_MAX_M, |s, v| s.land_max_m = v);
}

#[cfg(test)]
#[path = "noise_emitters_tests.rs"]
mod tests;
