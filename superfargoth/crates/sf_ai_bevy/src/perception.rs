//! Perception data for the sight system ([`crate::sight`], slice `AI8b2`; `docs/specs/ai.md`
//! "Perception and stimuli"): the `ai.perception.*` / `ai.awareness.*` tunables bound into
//! [`AiPerceptionSettings`] (the light exposure's too, `AI8d`), the per-actor components and the
//! `sf_records` kinds of the perception profile presets and states.
//!
//! Ownership (ai.md "Perception profile", "Ownership"): the adapter writes [`AiPerceptionProfile`]
//! (spawn, load, when its inputs change) and pushes or clears [`AiPerceptionStates`]; this crate
//! resolves them into the cached [`AiEffectivePerception`]. An actor with neither has no
//! [`AiEffectivePerception`] and reads the globals.

use std::collections::BTreeMap;

use bevy_app::App;
use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_ai::{
    AwarenessConfig, AwarenessMeter, EffectivePerception, LightConfig, PerceptTarget, PerceptionProfile, PerceptionState,
    PerceptionStates, SearchConfig, SearchPatternConfig, SightBudget, SightConfig, SightTimer,
};
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::hearing::HearingConfig;

// --- Tunables ---------------------------------------------------------------------------------

/// Gameplay metadata shared by every perception tunable (ai.md "Tunables": net `Gameplay`, scope
/// `Dev`, source `Guess`, live).
const fn gp(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

/// [`gp`] for counts.
const fn gp_count(d: Def<u32>) -> Def<u32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live).unit(Unit::Count)
}

pub const SIGHT_RANGE_M: Def<f32> = gp(def!(float "ai.perception.sight_range_m" = 30.0)
    .range(1.0, 200.0)
    .unit(Unit::Meters)
    .help("Sight range (UE SightRadius); a profile's sight_range scales it"));
pub const LOSE_SIGHT_RANGE_M: Def<f32> = gp(def!(float "ai.perception.lose_sight_range_m" = 36.0)
    .range(1.0, 200.0)
    .unit(Unit::Meters)
    .help("A target seen at the previous check is lost beyond this (UE LoseSightRadius)"));
pub const NEAR_M: Def<f32> = gp(def!(float "ai.perception.near_m" = 3.0)
    .range(1.0, 200.0)
    .unit(Unit::Meters)
    .help("Full sight gain up to this distance, then linear to 0 at the range (never scaled by a profile)"));
pub const CENTRAL_HALF_DEG: Def<f32> = gp(def!(float "ai.perception.central_half_deg" = 30.0)
    .range(0.0, 180.0)
    .unit(Unit::Degrees)
    .help("Central sight cone half-angle (full gain)"));
pub const PERIPHERAL_HALF_DEG: Def<f32> = gp(def!(float "ai.perception.peripheral_half_deg" = 80.0)
    .range(0.0, 180.0)
    .unit(Unit::Degrees)
    .help("Peripheral sight cone half-angle (gain x peripheral_mul); nothing seen behind it"));
pub const PERIPHERAL_MUL: Def<f32> = gp(def!(float "ai.perception.peripheral_mul" = 0.35)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Sight gain multiplier in the peripheral band"));
pub const RATE: Def<f32> = gp(def!(float "ai.perception.rate" = 2.0)
    .range(0.05, 20.0)
    .help("Sight gain per second at full visibility within near_m (1/s)"));
pub const SKIP_GAIN: Def<f32> = gp(def!(float "ai.perception.skip_gain" = 0.01)
    .range(0.0, 1.0)
    .help("A sight check whose gain rate (1/s) is below this casts no rays and gains nothing"));
pub const SIGHT_PERIOD_S: Def<f32> = gp(def!(float "ai.perception.sight_period_s" = 0.1)
    .range(0.02, 10.0)
    .unit(Unit::Seconds)
    .help("Tier 2 sight check period, staggered per actor"));
pub const TIER1_PERIOD_S: Def<f32> = gp(def!(float "ai.perception.tier1_period_s" = 1.0)
    .range(0.02, 10.0)
    .unit(Unit::Seconds)
    .help("Tier 1 sight check period (no rays; read by the AI9 allocator)"));
pub const MAX_RAYS_PER_TICK: Def<u32> = gp_count(def!(count "ai.perception.max_rays_per_tick" = 24)
    .range(4, 512)
    .help("Rays cast per fixed tick across sight, light and concern rays (UE MaxTracesPerTick); Floor tier value"));
pub const SIGHT_CACHE_S: Def<f32> = gp(def!(float "ai.perception.sight_cache_s" = 0.25)
    .range(0.02, 10.0)
    .unit(Unit::Seconds)
    .help("How long a line-of-sight result is reused"));
pub const CACHE_MOVE_M: Def<f32> = gp(def!(float "ai.perception.cache_move_m" = 0.5)
    .range(0.0, 5.0)
    .unit(Unit::Meters)
    .help("A cached line-of-sight result is dropped once either side moved more than this"));
pub const SEARCH_TTL_S: Def<f32> = gp(def!(float "ai.perception.search_ttl_s" = 30.0)
    .range(0.0, 300.0)
    .unit(Unit::Seconds)
    .help("How long a search persists (AI8f; a profile's search_ttl scales it)"));
pub const SEARCH_ERROR_M: Def<f32> = gp(def!(float "ai.perception.search_error_m" = 3.0)
    .range(0.0, 30.0)
    .unit(Unit::Meters)
    .help("Initial error of a searched position (AI8f; a profile's search_error scales it)"));
pub const SEARCH_WIDEN_MPS: Def<f32> = gp(def!(float "ai.perception.search_widen_mps" = 0.5)
    .range(0.0, 5.0)
    .help("How fast a search area widens, m/s (AI8f)"));
pub const SEARCH_RING_POINTS: Def<u32> = gp_count(def!(count "ai.perception.search_ring_points" = 6)
    .range(1, 16)
    .help("Points per ring of a widening search around the last known position (AI8f)"));
pub const SEARCH_LOOK_HALF_DEG: Def<f32> = gp(def!(float "ai.perception.search_look_half_deg" = 60.0)
    .range(0.0, 180.0)
    .unit(Unit::Degrees)
    .help("Half-arc a searcher sweeps when it looks around at a point (AI8f)"));
pub const SEARCH_UNCERTAIN_MUL: Def<f32> = gp(def!(float "ai.perception.search_uncertain_mul" = 2.0)
    .range(1.0, 4.0)
    .unit(Unit::Ratio)
    .help("Search radius multiplier for a last known position of confidence 0 (a vague noise; AI8f)"));
pub const SEARCH_INNER_FRAC: Def<f32> = gp(def!(float "ai.perception.search_inner_frac" = 0.5)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("A ring point's nearest distance as a fraction of the ring radius (AI8f)"));

pub const CURIOUS: Def<f32> = gp(def!(float "ai.awareness.curious" = 0.1)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Awareness level that enters Curious"));
pub const SUSPICIOUS: Def<f32> = gp(def!(float "ai.awareness.suspicious" = 0.35)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Awareness level that enters Suspicious"));
pub const SEARCHING: Def<f32> = gp(def!(float "ai.awareness.searching" = 0.65)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Awareness level that enters Searching (1.0 enters Alerted)"));
pub const BAND_HYSTERESIS: Def<f32> = gp(def!(float "ai.awareness.band_hysteresis" = 0.05)
    .range(0.0, 0.3)
    .unit(Unit::Ratio)
    .help("A band is left below its threshold minus this"));
pub const ALERT_HOLD_S: Def<f32> = gp(def!(float "ai.awareness.alert_hold_s" = 6.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("Alerted holds this long after the last contact, then drops to Searching"));
pub const DECAY_DELAY_S: Def<f32> = gp(def!(float "ai.awareness.decay_delay_s" = 3.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("Awareness starts decaying this long after the last gain"));
pub const DECAY_CURIOUS: Def<f32> = gp(def!(float "ai.awareness.decay_curious" = 0.10)
    .range(0.0, 1.0)
    .help("Decay per second in Curious (and Unaware), 1/s"));
pub const DECAY_SUSPICIOUS: Def<f32> = gp(def!(float "ai.awareness.decay_suspicious" = 0.05)
    .range(0.0, 1.0)
    .help("Decay per second in Suspicious, 1/s"));
pub const DECAY_SEARCHING: Def<f32> = gp(def!(float "ai.awareness.decay_searching" = 0.02)
    .range(0.0, 1.0)
    .help("Decay per second in Searching, 1/s"));
pub const NOISE_GAIN: Def<f32> = gp(def!(float "ai.awareness.noise_gain" = 0.5)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Awareness a noise at the listener adds (scaled down with distance over its reach)"));
pub const NOISE_CAP: Def<f32> = gp(def!(float "ai.awareness.noise_cap" = 0.65)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Noise alone raises awareness at most to this"));
pub const NOISE_CONFIDENCE: Def<f32> = gp(def!(float "ai.awareness.noise_confidence" = 0.5)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Confidence of a last-known position placed by a noise"));
pub const ASLEEP_WAKE_M: Def<f32> = gp(def!(float "ai.awareness.asleep_wake_m" = 12.0)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("A sleeper wakes to a noise whose remaining reach is at least this"));
pub const GROGGY_S: Def<f32> = gp(def!(float "ai.awareness.groggy_s" = 4.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("Seconds after waking during which sight gain uses groggy_alertness"));
pub const MAX_ENTRIES: Def<u32> = gp_count(def!(count "ai.awareness.max_entries" = 8)
    .range(1, 64)
    .help("Awareness entries per actor (the lowest is evicted)"));
pub const DISTRACTION_TTL_S: Def<f32> = gp(def!(float "ai.awareness.distraction_ttl_s" = 10.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("How long a distraction stays in the actor's slot"));
pub const ALERT_ALERTNESS: Def<f32> = gp(def!(float "ai.awareness.alert_alertness" = 1.5)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Sight gain multiplier toward a target from Suspicious up"));
pub const GROGGY_ALERTNESS: Def<f32> = gp(def!(float "ai.awareness.groggy_alertness" = 0.5)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Sight gain multiplier while groggy"));
pub const NOTIFY_SPEED_MPS: Def<f32> = gp(def!(float "ai.awareness.notify_speed_mps" = 6.0)
    .range(0.5, 50.0)
    .help("Speed a shout or alarm's news travels, m/s (delivery delay = distance / this + notify_react_s)"));
pub const NOTIFY_REACT_S: Def<f32> = gp(def!(float "ai.awareness.notify_react_s" = 0.8)
    .range(0.0, 5.0)
    .unit(Unit::Seconds)
    .help("Reaction time added to a notify's delivery"));
pub const NOTIFY_CONFIDENCE_MUL: Def<f32> = gp(def!(float "ai.awareness.notify_confidence_mul" = 0.7)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Confidence multiplier per notify hop"));
pub const NOTIFY_MAX_HOPS: Def<u32> = gp_count(def!(count "ai.awareness.notify_max_hops" = 3)
    .range(0, 8)
    .help("A notify relayed more than this many times is ignored"));

pub const CLOSED_DOOR_M: Def<f32> = gp(def!(float "ai.perception.closed_door_m" = 8.0)
    .range(0.0, 50.0)
    .unit(Unit::Meters)
    .help("Path penalty per closed door between a noise and a listener (AI8c)"));

pub const LIGHT_SEARCH_M: Def<f32> = gp(def!(float "ai.perception.light_search_m" = 15.0)
    .range(1.0, 60.0)
    .unit(Unit::Meters)
    .help("Light sources farther than this from a target add nothing (AI8d); also the roof ray's length"));
pub const LIGHT_RAYS: Def<u32> = gp_count(def!(count "ai.perception.light_rays" = 2)
    .range(0, 4)
    .help("Strongest light sources per target checked by one line-of-sight ray each (AI8d)"));
pub const DARK_THRESHOLD: Def<f32> = gp(def!(float "ai.perception.dark_threshold" = 0.15)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Light exposure below this at an actor is dark (the dark world-state key)"));
pub const ROOF_SKY_MUL: Def<f32> = gp(def!(float "ai.perception.roof_sky_mul" = 0.25)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Share of the sky term left under a roof (outdoors, the upward ray hit)"));
pub const LIGHT_FALLOFF_EXP: Def<f32> = gp(def!(float "ai.perception.light_falloff_exp" = 2.0)
    .range(1.0, 8.0)
    .help("Exponent k of a light's falloff window saturate(1 - (d/r)^k)^2 (UE4/Frostbite use 4)"));

/// The perception tunables (`ai.perception.*`, `ai.awareness.*`), bound by [`add_perception_tunables`].
pub const PERCEPTION_TUNABLES: &[&dyn AnyDef] = &[
    &SIGHT_RANGE_M,
    &LOSE_SIGHT_RANGE_M,
    &NEAR_M,
    &CENTRAL_HALF_DEG,
    &PERIPHERAL_HALF_DEG,
    &PERIPHERAL_MUL,
    &RATE,
    &SKIP_GAIN,
    &SIGHT_PERIOD_S,
    &TIER1_PERIOD_S,
    &MAX_RAYS_PER_TICK,
    &SIGHT_CACHE_S,
    &CACHE_MOVE_M,
    &SEARCH_TTL_S,
    &SEARCH_ERROR_M,
    &SEARCH_WIDEN_MPS,
    &SEARCH_RING_POINTS,
    &SEARCH_LOOK_HALF_DEG,
    &SEARCH_UNCERTAIN_MUL,
    &SEARCH_INNER_FRAC,
    &CURIOUS,
    &SUSPICIOUS,
    &SEARCHING,
    &BAND_HYSTERESIS,
    &ALERT_HOLD_S,
    &DECAY_DELAY_S,
    &DECAY_CURIOUS,
    &DECAY_SUSPICIOUS,
    &DECAY_SEARCHING,
    &NOISE_GAIN,
    &NOISE_CAP,
    &NOISE_CONFIDENCE,
    &ASLEEP_WAKE_M,
    &GROGGY_S,
    &MAX_ENTRIES,
    &DISTRACTION_TTL_S,
    &ALERT_ALERTNESS,
    &GROGGY_ALERTNESS,
    &NOTIFY_SPEED_MPS,
    &NOTIFY_REACT_S,
    &NOTIFY_CONFIDENCE_MUL,
    &NOTIFY_MAX_HOPS,
    &CLOSED_DOOR_M,
    &LIGHT_SEARCH_M,
    &LIGHT_RAYS,
    &DARK_THRESHOLD,
    &ROOF_SKY_MUL,
    &LIGHT_FALLOFF_EXP,
];

/// The global perception config, bound from [`PERCEPTION_TUNABLES`]. A change (a tunable reload)
/// rebuilds every [`AiEffectivePerception`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct AiPerceptionSettings {
    pub awareness: AwarenessConfig,
    pub sight: SightConfig,
    pub budget: SightBudget,
    pub search: SearchConfig,
    /// The search pattern (`AI8f`, `sf_ai::search`).
    pub search_pattern: SearchPatternConfig,
    /// Light exposure (`AI8d`, [`crate::light`]).
    pub light: LightConfig,
    /// Hearing (`AI8c`, [`crate::hearing`]).
    pub hearing: HearingConfig,
}

macro_rules! bind {
    ($app:ident, $def:expr, |$s:ident, $v:ident| $body:expr) => {
        let _ = $app.add_tunable(&$def).bind_resource(|$s: &mut AiPerceptionSettings, $v| $body);
    };
}

/// Registers [`PERCEPTION_TUNABLES`] and binds them to [`AiPerceptionSettings`] (the caller
/// inserts the resource; [`crate::AiSightPlugin`] does).
pub fn add_perception_tunables(app: &mut App) {
    bind!(app, SIGHT_RANGE_M, |s, v| s.sight.sight_range_m = v);
    bind!(app, LOSE_SIGHT_RANGE_M, |s, v| s.sight.lose_sight_range_m = v);
    bind!(app, NEAR_M, |s, v| s.sight.near_m = v);
    bind!(app, CENTRAL_HALF_DEG, |s, v| s.sight.central_half_deg = v);
    bind!(app, PERIPHERAL_HALF_DEG, |s, v| s.sight.peripheral_half_deg = v);
    bind!(app, PERIPHERAL_MUL, |s, v| s.sight.peripheral_mul = v);
    bind!(app, RATE, |s, v| s.sight.rate = v);
    bind!(app, SKIP_GAIN, |s, v| s.budget.skip_gain = v);
    bind!(app, SIGHT_PERIOD_S, |s, v| s.budget.sight_period_s = v);
    bind!(app, TIER1_PERIOD_S, |s, v| s.budget.tier1_period_s = v);
    bind!(app, MAX_RAYS_PER_TICK, |s, v| s.budget.max_rays_per_tick = v);
    bind!(app, SIGHT_CACHE_S, |s, v| s.budget.sight_cache_s = v);
    bind!(app, CACHE_MOVE_M, |s, v| s.budget.cache_move_m = v);
    bind!(app, SEARCH_TTL_S, |s, v| s.search.search_ttl_s = v);
    bind!(app, SEARCH_ERROR_M, |s, v| s.search.search_error_m = v);
    bind!(app, SEARCH_WIDEN_MPS, |s, v| s.search.search_widen_mps = v);
    bind!(app, SEARCH_RING_POINTS, |s, v| s.search_pattern.ring_points = u8::try_from(v).unwrap_or(u8::MAX));
    bind!(app, SEARCH_LOOK_HALF_DEG, |s, v| s.search_pattern.look_half_deg = v);
    bind!(app, SEARCH_UNCERTAIN_MUL, |s, v| s.search_pattern.uncertain_mul = v);
    bind!(app, SEARCH_INNER_FRAC, |s, v| s.search_pattern.inner_frac = v);
    bind!(app, CURIOUS, |s, v| s.awareness.curious = v);
    bind!(app, SUSPICIOUS, |s, v| s.awareness.suspicious = v);
    bind!(app, SEARCHING, |s, v| s.awareness.searching = v);
    bind!(app, BAND_HYSTERESIS, |s, v| s.awareness.band_hysteresis = v);
    bind!(app, ALERT_HOLD_S, |s, v| s.awareness.alert_hold_s = v);
    bind!(app, DECAY_DELAY_S, |s, v| s.awareness.decay_delay_s = v);
    bind!(app, DECAY_CURIOUS, |s, v| s.awareness.decay_curious = v);
    bind!(app, DECAY_SUSPICIOUS, |s, v| s.awareness.decay_suspicious = v);
    bind!(app, DECAY_SEARCHING, |s, v| s.awareness.decay_searching = v);
    bind!(app, NOISE_GAIN, |s, v| s.awareness.noise_gain = v);
    bind!(app, NOISE_CAP, |s, v| s.awareness.noise_cap = v);
    bind!(app, NOISE_CONFIDENCE, |s, v| s.awareness.noise_confidence = v);
    bind!(app, ASLEEP_WAKE_M, |s, v| s.awareness.asleep_wake_m = v);
    bind!(app, GROGGY_S, |s, v| s.awareness.groggy_s = v);
    bind!(app, MAX_ENTRIES, |s, v| s.awareness.max_entries = usize::try_from(v).unwrap_or(usize::MAX));
    bind!(app, DISTRACTION_TTL_S, |s, v| s.awareness.distraction_ttl_s = v);
    bind!(app, ALERT_ALERTNESS, |s, v| s.awareness.alert_alertness = v);
    bind!(app, GROGGY_ALERTNESS, |s, v| s.awareness.groggy_alertness = v);
    bind!(app, NOTIFY_SPEED_MPS, |s, v| s.awareness.notify_speed_mps = v);
    bind!(app, NOTIFY_REACT_S, |s, v| s.awareness.notify_react_s = v);
    bind!(app, NOTIFY_CONFIDENCE_MUL, |s, v| s.awareness.notify_confidence_mul = v);
    bind!(app, NOTIFY_MAX_HOPS, |s, v| s.awareness.notify_max_hops = u8::try_from(v).unwrap_or(u8::MAX));
    bind!(app, CLOSED_DOOR_M, |s, v| s.hearing.closed_door_m = v);
    bind!(app, LIGHT_SEARCH_M, |s, v| s.light.light_search_m = v);
    bind!(app, LIGHT_RAYS, |s, v| s.light.light_rays = v);
    bind!(app, DARK_THRESHOLD, |s, v| s.light.dark_threshold = v);
    bind!(app, ROOF_SKY_MUL, |s, v| s.light.roof_sky_mul = v);
    bind!(app, LIGHT_FALLOFF_EXP, |s, v| s.light.light_falloff_exp = v);
}

// --- Components -------------------------------------------------------------------------------

/// An actor that sees: its eye height above the mover's feet (the adapter sets it from the body).
/// Needs [`crate::AiActor`] and a `Mover`; brings its [`AiAwareness`] and [`AiSightTimer`].
#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[require(AiAwareness, AiSightTimer)]
pub struct AiObserver {
    /// Eye above the feet, meters.
    pub eye_m: f32,
}

/// The actor's awareness memory (`AI8a`): sight applies here, noise and notify ([`crate::hearing`], `AI8c`).
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct AiAwareness(pub AwarenessMeter);

/// The actor's staggered sight check (`SightTimer`): damage, noise or notify call
/// `.0.force()` for a check next tick. A fresh timer counts from the tick it is first seen.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct AiSightTimer(pub SightTimer);

/// The adapter-written perception profile (ai.md "Perception profile"). Optional.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct AiPerceptionProfile(pub PerceptionProfile);

/// The actor's active perception states (pushed and cleared by the adapter). Optional.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct AiPerceptionStates(pub PerceptionStates);

/// The cached resolve of profile × states (rebuilt on change, state expiry or a tunable reload).
/// Present only while the actor has a profile or states.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AiEffectivePerception(pub EffectivePerception);

/// Something observers can see: its perception target id and the chest and head heights above
/// its feet (its `Mover`, else its [`PerceivablePose`]; the adapter sets them from the body). Chest
/// ray first, head if blocked.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Perceivable {
    pub target: PerceptTarget,
    /// Chest above the feet, meters.
    pub chest_m: f32,
    /// Head above the feet, meters.
    pub head_m: f32,
}

/// Where a [`Perceivable`] without a `Mover` stands (a player's action body, a remote player, a
/// prop): the adapter writes it on the fixed tick before [`crate::AiPerceptionSystems::Sight`].
/// A target with a `Mover` reads the mover instead.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PerceivablePose {
    /// The feet, world position.
    pub feet: WorldPos,
    /// Velocity, m/s (the last-seen estimate).
    pub velocity: Vec3,
}

// --- Records ----------------------------------------------------------------------------------

// The kinds and their registration live in the core (a Bevy-free mod loader registers them).
pub use sf_ai::{PERCEPTION_PROFILE_KIND, PERCEPTION_STATE_KIND, register_perception_kinds};

/// The live [`PERCEPTION_STATE_KIND`] records the resolve looks state ids up in (an unknown id
/// counts as 1). Filled by [`load_perception_states`]; a change rebuilds every
/// [`AiEffectivePerception`].
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct AiPerceptionStateDefs(pub BTreeMap<RecordId, PerceptionState>);

/// Copies `db`'s perception states into [`AiPerceptionStateDefs`] after validating each, and
/// checks every profile preset; an out-of-range record logs one error line (target `sf_ai`) and
/// is skipped. Returns the number of records refused.
pub fn load_perception_states(world: &mut World, db: &RecordDb) -> usize {
    let mut refused = 0;
    for (id, p) in db.iter::<PerceptionProfile>() {
        if let Err(e) = p.validate() {
            log::error!(target: "sf_ai", "{PERCEPTION_PROFILE_KIND} {id}: out of range: {:?}", e.names());
            refused += 1;
        }
    }
    let mut states = BTreeMap::new();
    for (id, s) in db.iter::<PerceptionState>() {
        match s.validate() {
            Ok(()) => {
                states.insert(id.clone(), *s);
            }
            Err(e) => {
                log::error!(target: "sf_ai", "{PERCEPTION_STATE_KIND} {id}: out of range: {:?}; ignored", e.names());
                refused += 1;
            }
        }
    }
    world.insert_resource(AiPerceptionStateDefs(states));
    refused
}
