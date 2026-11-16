//! The cache's tunables (`shadow.lamp.cache.*`, `docs/specs/tunables.md`) and the
//! [`LampCacheConfig`] they fill. Spec: `docs/specs/lamp-shadow-cache.md` "Tunables".

use sf_tunables::{AnyDef, Def, Group, RegisterError, Source, Tunables, Unit, def};

use super::MAX_TILES;

/// `shadow.lamp.cache.enabled`: the game switch reads it (S3); the core does not.
pub const ENABLED: Def<bool> = def!(bool "shadow.lamp.cache.enabled" = false)
    .help("Still lamps keep their static shadow depth and draw only movers over a copy of it")
    .source(Source::Owner(
        "2026-11-15: a change stays opt-in, off, until its interleaved A/B wins",
    ))
    .group(Group::Shadows);

/// `shadow.lamp.cache.capture_faces_per_frame`: [`LampCacheConfig::capture_faces_per_frame`].
/// The old cache (V1) captured 2 lights per frame; the spec spreads captures by face.
pub const CAPTURE_FACES_PER_FRAME: Def<u32> =
    def!(count "shadow.lamp.cache.capture_faces_per_frame" = 2)
        .range(1, 48)
        .unit(Unit::Count)
        .help("Lamp shadow faces captured (static casters drawn into the cache) per frame at most")
        .group(Group::Shadows);

/// `shadow.lamp.cache.recapture_interval_s`: [`LampCacheConfig::recapture_interval_s`] (V1 lesson 6).
pub const RECAPTURE_INTERVAL: Def<f32> = def!(float "shadow.lamp.cache.recapture_interval_s" = 2.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("A face whose static casters changed is captured again at most this often")
    .group(Group::Shadows);

/// `shadow.lamp.cache.settle_time_s`: [`LampCacheConfig::settle_time_s`] (V1's value).
pub const SETTLE_TIME: Def<f32> = def!(float "shadow.lamp.cache.settle_time_s" = 1.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("A rigid caster that moved counts as a mover until it has been still this long")
    .group(Group::Shadows);

/// `shadow.lamp.cache.store_faces_per_lamp`: [`LampCacheConfig::store_faces_per_lamp`] (a
/// walker dirties 1-3 faces of a cube).
pub const STORE_FACES_PER_LAMP: Def<u32> = def!(count "shadow.lamp.cache.store_faces_per_lamp" = 3)
    .range(0, MAX_TILES as u32)
    .unit(Unit::Count)
    .help("Store layers (static depth copies) pooled per cached lamp slot")
    .group(Group::Shadows);

/// `shadow.lamp.cache.mover_full_rate_m`: [`LampCacheConfig::mover_full_rate_m`] (the SH-20 rule:
/// movers near the viewer update every frame).
pub const MOVER_FULL_RATE: Def<f32> = def!(float "shadow.lamp.cache.mover_full_rate_m" = 16.0)
    .range(0.0, 500.0)
    .unit(Unit::Meters)
    .help("Lamp shadows of movers within this distance of a camera are redrawn every frame")
    .group(Group::Shadows);

/// `shadow.lamp.cache.mover_half_rate_m`: [`LampCacheConfig::mover_half_rate_m`].
pub const MOVER_HALF_RATE: Def<f32> = def!(float "shadow.lamp.cache.mover_half_rate_m" = 30.0)
    .range(0.0, 500.0)
    .unit(Unit::Meters)
    .help("Lamp shadows of movers out to this distance are redrawn every second frame, lamps alternating")
    .group(Group::Shadows);

/// `shadow.lamp.cache.mover_far_interval_frames`: [`LampCacheConfig::mover_far_interval_frames`].
/// Not in the spec's list (it names no rate past the half-rate distance); added with S1.
pub const MOVER_FAR_INTERVAL: Def<u32> =
    def!(count "shadow.lamp.cache.mover_far_interval_frames" = 4)
        .range(1, 60)
        .unit(Unit::Count)
        .help(
            "Lamp shadows of movers past the half-rate distance are redrawn every this many frames",
        )
        .group(Group::Shadows);

/// `shadow.lamp.cache.max_mover_faces_per_frame`: [`LampCacheConfig::max_mover_faces_per_frame`].
pub const MAX_MOVER_FACES_PER_FRAME: Def<u32> = def!(count "shadow.lamp.cache.max_mover_faces_per_frame" = 12)
    .range(1, 512)
    .unit(Unit::Count)
    .help("Cached lamp faces with movers drawn per frame at most, lamps that waited longest first")
    .group(Group::Shadows);

/// `shadow.lamp.cache.mover_linger_s`: [`LampCacheConfig::mover_linger_s`].
pub const MOVER_LINGER: Def<f32> = def!(float "shadow.lamp.cache.mover_linger_s" = 0.5)
    .range(0.0, 10.0)
    .unit(Unit::Seconds)
    .help("A face's view is kept this long after its last mover left, so a walker crossing back costs no respawn")
    .group(Group::Shadows);

/// `shadow.lamp.cache.static_tier_slots`: [`LampCacheConfig::static_tier_slots`]. Off until the
/// owner answers the spec's question 1 (S6).
pub const STATIC_TIER_SLOTS: Def<u32> = def!(count "shadow.lamp.cache.static_tier_slots" = 0)
    .range(0, 256)
    .unit(Unit::Count)
    .help(
        "Lamps past the budget that keep a captured cube and never draw movers (static-only tier)",
    )
    .group(Group::Shadows);

/// `shadow.lamp.cache.animated_casters_move`: [`LampCacheConfig::animated_casters_move`]. The
/// spec's default (frozen in the cache) until the owner answers question 2; UE leaves such
/// materials out of its cache (the `true` side).
pub const ANIMATED_CASTERS_MOVE: Def<bool> = def!(bool "shadow.lamp.cache.animated_casters_move" = false)
    .help("Casters animated in the vertex shader (wind) are drawn as movers instead of frozen in the cache")
    .group(Group::Shadows);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &ENABLED,
    &CAPTURE_FACES_PER_FRAME,
    &RECAPTURE_INTERVAL,
    &SETTLE_TIME,
    &STORE_FACES_PER_LAMP,
    &MOVER_FULL_RATE,
    &MOVER_HALF_RATE,
    &MOVER_FAR_INTERVAL,
    &MAX_MOVER_FACES_PER_FRAME,
    &MOVER_LINGER,
    &STATIC_TIER_SLOTS,
    &ANIMATED_CASTERS_MOVE,
];

/// The cache's policy values (one field per tunable but [`ENABLED`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LampCacheConfig {
    /// Faces captured per frame at most (a capture is the face's static casters, cleared first).
    pub capture_faces_per_frame: u32,
    /// Seconds between two captures of a face whose statics changed.
    pub recapture_interval_s: f32,
    /// Seconds a rigid caster must be still to count as static ([`LampCacheConfig::classify`]).
    pub settle_time_s: f32,
    /// Store layers per cached lamp slot: the pool holds `max_lamps * store_faces_per_lamp`.
    pub store_faces_per_lamp: u32,
    /// Movers within this distance of a camera: drawn every frame.
    pub mover_full_rate_m: f32,
    /// Movers out to this distance: every second frame, lamps alternating.
    pub mover_half_rate_m: f32,
    /// Movers farther away: every this many frames, lamps alternating.
    pub mover_far_interval_frames: u32,
    /// Mover faces drawn per frame at most (captures are counted separately).
    pub max_mover_faces_per_frame: u32,
    /// Seconds a face keeps its view after its last mover left.
    pub mover_linger_s: f32,
    /// Static-only lamps (S6) admitted at most.
    pub static_tier_slots: u32,
    /// Vertex-animated statics ([`super::CasterMobility::AnimatedStatic`]) are movers.
    pub animated_casters_move: bool,
}

impl Default for LampCacheConfig {
    /// The defs' defaults.
    fn default() -> Self {
        Self {
            capture_faces_per_frame: CAPTURE_FACES_PER_FRAME.default_value(),
            recapture_interval_s: RECAPTURE_INTERVAL.default_value(),
            settle_time_s: SETTLE_TIME.default_value(),
            store_faces_per_lamp: STORE_FACES_PER_LAMP.default_value(),
            mover_full_rate_m: MOVER_FULL_RATE.default_value(),
            mover_half_rate_m: MOVER_HALF_RATE.default_value(),
            mover_far_interval_frames: MOVER_FAR_INTERVAL.default_value(),
            max_mover_faces_per_frame: MAX_MOVER_FACES_PER_FRAME.default_value(),
            mover_linger_s: MOVER_LINGER.default_value(),
            static_tier_slots: STATIC_TIER_SLOTS.default_value(),
            animated_casters_move: ANIMATED_CASTERS_MOVE.default_value(),
        }
    }
}

impl LampCacheConfig {
    /// The effective values from a registry holding [`TUNABLES`].
    ///
    /// # Errors
    /// A def is not registered, or registered with another kind.
    pub fn from_tunables(t: &Tunables) -> Result<Self, RegisterError> {
        Ok(Self {
            capture_faces_per_frame: t.read(t.id_of(&CAPTURE_FACES_PER_FRAME)?),
            recapture_interval_s: t.read(t.id_of(&RECAPTURE_INTERVAL)?),
            settle_time_s: t.read(t.id_of(&SETTLE_TIME)?),
            store_faces_per_lamp: t.read(t.id_of(&STORE_FACES_PER_LAMP)?),
            mover_full_rate_m: t.read(t.id_of(&MOVER_FULL_RATE)?),
            mover_half_rate_m: t.read(t.id_of(&MOVER_HALF_RATE)?),
            mover_far_interval_frames: t.read(t.id_of(&MOVER_FAR_INTERVAL)?),
            max_mover_faces_per_frame: t.read(t.id_of(&MAX_MOVER_FACES_PER_FRAME)?),
            mover_linger_s: t.read(t.id_of(&MOVER_LINGER)?),
            static_tier_slots: t.read(t.id_of(&STATIC_TIER_SLOTS)?),
            animated_casters_move: t.read(t.id_of(&ANIMATED_CASTERS_MOVE)?),
        })
    }
}
