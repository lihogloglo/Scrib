//! The swim rule (FL-5c1): a capsule's [`Immersion`] becomes "swimming or not, where the surface
//! is, which current carries it". Pure and Bevy-free; a game system calls [`medium`] each fixed
//! tick with the capsule's last swim state (the hysteresis input) and feeds the result to its
//! mover.
//!
//! Industry: a swimmer moves relative to the water, so the water's flow is added to its velocity
//! (UE `CharacterMovementComponent` fluid volumes, `PhysicsVolume::FluidFriction`); entering and
//! leaving the swim state at different immersion depths is the usual guard against flicker on a
//! wave or a sloped bed.

use glam::Vec3;
use sf_tunables::{Def, Lifetime, Net, Scope, Source, Unit, def};

use crate::query::Immersion;

/// Gameplay metadata shared by every swim tunable (host-authoritative, dev scope, live).
const fn gp(d: Def<f32>) -> Def<f32> {
    d.scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

/// `swim.enter_fraction`: [`SwimParams::enter_fraction`].
pub const ENTER_FRACTION: Def<f32> = gp(def!("swim.enter_fraction" = 0.7_f32)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Swim: a wading capsule starts swimming once this share of its height is under water (a game adapter may set it from its own rule)")
    .source(Source::Guess));

/// `swim.exit_fraction`: [`SwimParams::exit_fraction`].
pub const EXIT_FRACTION: Def<f32> = gp(def!("swim.exit_fraction" = 0.6_f32)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Swim: a swimming capsule stops swimming once less than this share of its height is under water (hysteresis; above swim.enter_fraction it counts as that)")
    .source(Source::Guess));

/// `swim.current.gain`: [`SwimParams::current_gain`].
pub const CURRENT_GAIN: Def<f32> = gp(def!("swim.current.gain" = 1.0_f32)
    .range(0.0, 2.0)
    .unit(Unit::Ratio)
    .help("Swim: share of the water's horizontal velocity (flow + wave orbital) that carries a swimmer")
    .source(Source::Industry(
        "a swimmer's velocity is relative to the water (UE CharacterMovement fluid volumes)",
    )));

/// `swim.current.wade_gain`: [`SwimParams::wade_gain`].
pub const CURRENT_WADE_GAIN: Def<f32> = gp(def!("swim.current.wade_gain" = 0.3_f32)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Swim: a wading capsule gets the swimmer's current times this times its submerged share")
    .source(Source::Guess));

/// `swim.current.max_mps`: [`SwimParams::max_mps`].
pub const CURRENT_MAX_MPS: Def<f32> = gp(def!("swim.current.max_mps" = 3.0_f32)
    .range(0.0, 5.0)
    .unit(Unit::None)
    .help("Swim: cap on the current's speed, m/s (matches the guide-river speed cap, water.guide.river_speed_max_mps)")
    .source(Source::Guess));

/// The swim tunables (not yet in the crate's [`crate::TUNABLES`]: the wiring slice adds them).
pub const TUNABLES: &[&dyn sf_tunables::AnyDef] = &[
    &ENTER_FRACTION,
    &EXIT_FRACTION,
    &CURRENT_GAIN,
    &CURRENT_WADE_GAIN,
    &CURRENT_MAX_MPS,
];

/// The swim rule's settings. Every value is a tunable (`tunables.md`): unit and range in its doc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwimParams {
    /// `swim.enter_fraction`: submerged share at which a wading capsule starts swimming, ratio,
    /// range 0-1, default 0.7 (Guess; a game adapter may set it from its own rule).
    pub enter_fraction: f32,
    /// `swim.exit_fraction`: submerged share below which a swimmer stops, ratio, range 0-1,
    /// default 0.6 (Guess: hysteresis). Above `enter_fraction` it counts as `enter_fraction`.
    pub exit_fraction: f32,
    /// `swim.current.gain`: share of the water's horizontal velocity that carries a swimmer,
    /// ratio, range 0-2, default 1 (Industry: velocity relative to the water, UE fluid volumes).
    pub current_gain: f32,
    /// `swim.current.wade_gain`: a wader's current is the swimmer's times this times its
    /// submerged share, ratio, range 0-1, default 0.3 (Guess).
    pub wade_gain: f32,
    /// `swim.current.max_mps`: cap on the current's speed, m/s, range 0-5, default 3 (Guess:
    /// the guide-river speed cap).
    pub max_mps: f32,
}

impl Default for SwimParams {
    fn default() -> Self {
        Self {
            enter_fraction: ENTER_FRACTION.default_value(),
            exit_fraction: EXIT_FRACTION.default_value(),
            current_gain: CURRENT_GAIN.default_value(),
            wade_gain: CURRENT_WADE_GAIN.default_value(),
            max_mps: CURRENT_MAX_MPS.default_value(),
        }
    }
}

/// What the water does to a capsule this tick.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Medium {
    /// The capsule swims (otherwise it walks or wades).
    pub swim: bool,
    /// The water surface's height above the capsule's lowest point, metres
    /// ([`Immersion::depth`]); 0 when dry.
    pub surface: f32,
    /// Horizontal velocity the water adds to the capsule, m/s (y = 0).
    pub current: Vec3,
}

impl Medium {
    /// Out of the water.
    pub const DRY: Self = Self { swim: false, surface: 0.0, current: Vec3::ZERO };
}

/// The swim rule. `swimming` is the capsule's state last tick: a wader starts swimming at
/// `enter_fraction`, a swimmer stops below `exit_fraction`.
#[must_use]
pub fn medium(immersion: &Immersion, swimming: bool, params: &SwimParams) -> Medium {
    let Some(sample) = immersion.sample.as_ref() else {
        return Medium::DRY;
    };
    if immersion.depth.is_nan() || immersion.depth <= 0.0 {
        return Medium::DRY;
    }
    let fraction = immersion.fraction;
    let threshold = if swimming {
        params.exit_fraction.min(params.enter_fraction)
    } else {
        params.enter_fraction
    };
    let swim = fraction >= threshold;

    let mut gain = params.current_gain;
    if !swim {
        gain *= params.wade_gain * fraction;
    }
    let mut current = Vec3::new(sample.velocity.x, 0.0, sample.velocity.z) * gain;
    if !current.is_finite() {
        current = Vec3::ZERO;
    }
    let current = current.clamp_length_max(params.max_mps.max(0.0));

    Medium { swim, surface: immersion.depth, current }
}
