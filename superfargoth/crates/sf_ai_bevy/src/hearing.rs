//! The hearing system (slice `AI8c1`; `docs/specs/ai.md` "Perception and stimuli" → "Noise",
//! "Noise gain", "Notify", "Asleep"): a [`NoiseEvent`] (UE `ReportNoiseEvent` shape) reaches every
//! listener whose path distance, plus `closed_door_m` per closed door, is within the noise's reach
//! at its ears (`loudness_m × hearing_mul × weather_mask`), as a `Stimulus::Noise` on its
//! [`AiAwareness`] meter; a noise carrying a [`Notify`] (a shout, an alarm) also queues that notify
//! through the core's delivery queue (`Stimulus::Notify`, delivered by the meter's update on its due
//! tick). The core decides what a heard noise does (`AwarenessMeter::hear`: a revealing noise raises
//! its source's entry, capped at `noise_cap`; any other fills the distraction slot; a sleeper wakes
//! only with `asleep_wake_m` of reach left).
//!
//! One system, `hear_noise`, in [`AiPerceptionSystems::Hearing`] on `FixedUpdate`, after
//! [`AiPerceptionSystems::Sight`] and before `ControllerSystems::Drive`. After Sight because it reads
//! what Sight's chain set up this tick: the clock `begin_tick` advanced and the
//! [`AiEffectivePerception`] `resolve_profiles` rebuilt (a hearing state pushed this tick counts this
//! tick). A noise applied after the meters' update is no later than one applied before it: the
//! update settles entries to `now` (no time passes until the next tick), and queued notifies carry
//! their own due tick.
//!
//! Listeners: entities with [`AiObserver`] + [`AiAwareness`] and a `Mover`, not `Dormant` and not
//! tier 0 ([`AiTier`] `Logical`; the actors sight polls), ears at the observer's eye height; the noise's own source actor is skipped.
//! Prefilter per listener: the straight line to the ears beyond its reach is unheard (a path is never
//! shorter than the straight line), so only listeners in reach ask [`NoisePropagation`]. Tier 1
//! ([`AiTier`] `Reduced`, ai.md "Tiers and budget": "distance + same space") never asks it: it hears
//! by the straight line, no doors, when it is in the noise's space (the listener's space is its
//! [`AiExecutor`] worldspace; a noise in the active worldspace (`space: None`) or a listener without
//! an executor counts as the same space, as sight takes every mover to share it). Tier 2 and
//! listeners without [`AiTier`] ask the propagation. A heard noise forces the listener's sight check
//! next tick (ai.md "Cost and budget").
//!
//! Emitters write [`NoiseEvent`]s on the fixed tick before [`AiPerceptionSystems::Hearing`] (heard
//! the same tick) or anywhere else (heard on the next perception tick; Bevy keeps messages until
//! `FixedUpdate` has run). The listener hears at the perception clock's tick; `tick` is the emitter's
//! stamp (traces, replication). Host only, like sight.
//!
//! Not modelled yet: paths through the navmesh and doors (`AI8c3` swaps [`NoisePropagation`] for
//! `portal_query::path_distance`); the default is straight line, no doors (UE `AIPerception` hearing);
//! spaces in the default propagation (it ignores [`NoiseEvent::space`], as sight takes every mover to
//! share the active worldspace); relaying a delivered notify (the receiver's behaviour decides,
//! `Notify::relay`).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_ai::{HeardNoise, NoiseKindId, Notify, PerceptTarget, Stimulus, StimulusOutcome, Tier};
use sf_controller_bevy::{ControllerSystems, Dormant, Mover};
use sf_coords::WorldPos;
use sf_world::WorldspaceId;

use crate::AiActor;
use crate::alloc::AiTier;
use crate::exec::AiExecutor;
use crate::perception::{AiAwareness, AiEffectivePerception, AiObserver, AiPerceptionSettings, AiSightTimer};
use crate::sight::{AiPerceptClock, AiPerceptionSystems};

/// A noise made in the world (ai.md "Noise"; UE `ReportNoiseEvent`): written by the game's emitters
/// (gait, landing, combat, action `noise: Loud`, interactions; `AI8c2`).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct NoiseEvent {
    /// The perception tick it was made on (the emitter's stamp; listeners hear at the clock's tick).
    pub tick: u64,
    /// Where it happened (where a thrown object landed).
    pub at: WorldPos,
    /// The space it was made in; `None`: the active worldspace. Passed to [`NoisePropagation`].
    pub space: Option<WorldspaceId>,
    pub kind: NoiseKindId,
    /// Open-air reach before the listener's `hearing_mul` and the weather mask, meters.
    pub loudness_m: f32,
    /// Who made it, if anyone (an actor never hears its own noise).
    pub source: Option<PerceptTarget>,
    /// Footsteps reveal their source; a thrown cup does not (`at` is where it landed).
    pub reveals_source: bool,
    /// A shout or alarm's shared knowledge: every listener that hears the noise also gets it
    /// (`Stimulus::Notify`, due after `notify_speed_mps` and `notify_react_s`).
    pub notify: Option<Notify>,
}

/// A noise's way to a listener: the path length and the closed doors on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoisePath {
    /// Path distance from the noise to the ears, meters (at least the straight line).
    pub path_m: f32,
    /// Closed doors the path goes through (`closed_door_m` each).
    pub closed_doors: u32,
}

/// How noise travels: the propagation seam the host replaces ([`NoisePropagation`]).
pub trait NoisePaths: Send + Sync + 'static {
    /// The way from `from` (the noise) to `to` (a listener's ears) in `space`; `None`: no way (an
    /// other space, no path), unheard. Asked only for listeners within straight-line reach.
    fn distance(&self, from: WorldPos, to: WorldPos, space: Option<&WorldspaceId>) -> Option<NoisePath>;
}

/// The default propagation: the straight line, no doors (UE `AIPerception` hearing).
#[derive(Clone, Copy, Debug, Default)]
pub struct StraightLine;

impl NoisePaths for StraightLine {
    fn distance(&self, from: WorldPos, to: WorldPos, _space: Option<&WorldspaceId>) -> Option<NoisePath> {
        // tunables: const (no door on a straight line)
        Some(NoisePath { path_m: from.delta_f32(to).length(), closed_doors: 0 })
    }
}

/// The host's noise propagation ([`StraightLine`] until replaced; `AI8c3`: nav paths and doors).
#[derive(Resource)]
pub struct NoisePropagation(pub Box<dyn NoisePaths>);

impl NoisePropagation {
    pub fn new(paths: impl NoisePaths) -> Self {
        Self(Box::new(paths))
    }
}

impl Default for NoisePropagation {
    fn default() -> Self {
        Self::new(StraightLine)
    }
}

/// The weather's noise mask (ai.md "Asleep": rain and storms mask noise), × every noise's reach.
/// Host-written from its weather (storm `weather_mask`); absent: 1, clear weather.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AiWeatherMask(pub f32);

impl Default for AiWeatherMask {
    fn default() -> Self {
        // tunables: const (input default: clear weather, the identity until the host writes it)
        Self(1.0)
    }
}

/// Hearing tunables (`ai.perception.closed_door_m`, bound in [`crate::perception`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HearingConfig {
    /// Path penalty per closed door, meters (range 0-50).
    pub closed_door_m: f32,
}

impl Default for HearingConfig {
    fn default() -> Self {
        // tunables: const (bound from ai.perception.closed_door_m)
        Self { closed_door_m: 8.0 }
    }
}

/// Adds the [`NoiseEvent`] message, the default [`NoisePropagation`] and the hearing system in
/// [`AiPerceptionSystems::Hearing`] (called by [`crate::AiSightPlugin`]; the clock, settings and
/// tunables are its).
pub(crate) fn add(app: &mut App) {
    app.add_message::<NoiseEvent>()
        .init_resource::<NoisePropagation>()
        .configure_sets(
            FixedUpdate,
            AiPerceptionSystems::Hearing.after(AiPerceptionSystems::Sight).before(ControllerSystems::Drive),
        )
        .add_systems(FixedUpdate, hear_noise.in_set(AiPerceptionSystems::Hearing));
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::cast_precision_loss)]
fn hear_noise(
    settings: Res<AiPerceptionSettings>,
    clock: Res<AiPerceptClock>,
    propagation: Res<NoisePropagation>,
    weather: Option<Res<AiWeatherMask>>,
    mut noises: MessageReader<NoiseEvent>,
    mut listeners: Query<
        (
            &AiActor,
            &AiObserver,
            &Mover,
            &mut AiAwareness,
            &mut AiSightTimer,
            Option<&AiEffectivePerception>,
            Option<&AiTier>,
            Option<&AiExecutor>,
        ),
        Without<Dormant>,
    >,
) {
    let now = clock.clock();
    let mask = weather.map_or(AiWeatherMask::default().0, |w| w.0);
    let door_m = settings.hearing.closed_door_m;
    for ev in noises.read() {
        for (actor, obs, mover, mut meter, mut timer, eff, tier, exec) in &mut listeners {
            // Tier 0 hears nothing (events only, ai.md "Tiers and budget").
            if ev.source == Some(PerceptTarget::Actor(actor.0)) || tier.is_some_and(|t| t.tier == Tier::Logical) {
                continue;
            }
            // An actor without a profile or states reads the globals (hearing × 1).
            let (cfg, hearing_mul) = eff.map_or((&settings.awareness, 1.0), |e| (&e.0.cfg, e.0.hearing_mul));
            let reach = ev.loudness_m * hearing_mul * mask;
            let ears = mover.loco.position.offset(Vec3::Y * obs.eye_m);
            // Prefilter; an invalid (NaN) reach hears nothing.
            let straight = ev.at.delta_f32(ears).length();
            if straight > reach || reach.is_nan() {
                continue;
            }
            let path = if tier.is_some_and(|t| t.tier == Tier::Reduced) {
                // Tier 1: distance + same space, no path query.
                if !same_space(ev.space.as_ref(), exec.map(|x| &x.worldspace)) {
                    continue;
                }
                // tunables: const (no door on a straight line)
                NoisePath { path_m: straight, closed_doors: 0 }
            } else {
                let Some(path) = propagation.0.distance(ev.at, ears, ev.space.as_ref()) else { continue };
                path
            };
            let noise = HeardNoise {
                source: ev.source,
                reveals_source: ev.reveals_source,
                at: ev.at,
                loudness_m: reach,
                distance_m: path.path_m + door_m * path.closed_doors as f32,
            };
            let heard = meter.0.apply(cfg, &Stimulus::Noise(noise), now);
            if let Some(notify) = ev.notify {
                let _ = meter.0.apply(cfg, &Stimulus::Notify { noise, notify }, now);
            }
            if matches!(heard, StimulusOutcome::Raised(_) | StimulusOutcome::Distraction | StimulusOutcome::Ignored) {
                timer.0.force();
            }
        }
    }
}

/// Whether a noise in `noise` reaches a listener in `listener` (tier 1's space test): different only
/// when both are known and differ (`None`: the active worldspace, or a listener without an executor).
fn same_space(noise: Option<&WorldspaceId>, listener: Option<&WorldspaceId>) -> bool {
    match (noise, listener) {
        (Some(a), Some(b)) => a == b,
        _ => true,
    }
}
