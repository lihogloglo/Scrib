//! Sight visibility from crime's stealth rules (slice `LW4b1`; `docs/specs/crime.md` "Stealth and
//! detection", decision 6: crime owns the formula, AI owns detection): `queue_sight` builds one
//! [`DetectIn`] per sight check and sets `SightCheck::visibility` to
//! [`visibility_from_concealment`] of the [`AiDetectionRules`]' concealment.
//!
//! - [`AiDetectionRules`]: the game's [`DetectionRules`] (default [`StandardDetection`]: light,
//!   stance, speed, facing). [`crate::AiSightPlugin`] inserts the default only when the app has none,
//!   so a game or plugin replaces it by inserting its own, before or after the plugin.
//! - [`AiVisibilityFactors`]: the parts of a target's look the game writes (stance, size, disguise,
//!   stat modifiers; `MA18` fills it from sneak mode). Absent: upright, standard size, no disguise.
//! - Per check (allocation-free): `factors.light` is the target's [`crate::AiLightExposure`] after
//!   the observer's `darkness` ([`sf_ai::apply_darkness`]), [`crate::VISIBILITY_UNMODELLED`] without
//!   a sample; `factors.speed_mps` the length of the target's velocity; `distance_m` eye to chest;
//!   `angle_rad` between the observer's facing and the direction to the chest (0 when undefined).
//!   `DetectIn::target` / `observer` views stay default until `sf_crime_bevy` fills them (stats,
//!   `StealthState`).
//!
//! Upright, still and in front, [`StandardDetection`] gives exactly the `AI8d2` light-only
//! visibility (crime.md `LW4a`).

use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_ai::{Stance, StatId, VISIBILITY_MODIFIERS_CAPACITY, VisibilityFactors, apply_darkness};
use sf_crime::{DetectIn, DetectionRules, StandardDetection, visibility_from_concealment};

use crate::sight::VISIBILITY_UNMODELLED;

/// The stealth rules sight asks for concealment (default [`StandardDetection`]; module docs).
#[derive(Resource, Clone)]
pub struct AiDetectionRules(pub Arc<dyn DetectionRules>);

impl Default for AiDetectionRules {
    fn default() -> Self {
        Self(Arc::new(StandardDetection::default()))
    }
}

impl std::fmt::Debug for AiDetectionRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AiDetectionRules(..)")
    }
}

/// A target's look as the game sets it (module docs); absent = [`Self::default`].
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AiVisibilityFactors {
    pub stance: Stance,
    /// Target size, ratio (1 = a standard humanoid).
    pub size: f32,
    /// How well the target is disguised, 0 (none) to 1 (perfect).
    pub disguise: f32,
    /// Stat values the game's rules read (sneak skill, chameleon magnitude, ...); `sf_ai` never
    /// interprets them.
    pub modifiers: [Option<(StatId, f32)>; VISIBILITY_MODIFIERS_CAPACITY],
}

impl Default for AiVisibilityFactors {
    fn default() -> Self {
        Self::from(&VisibilityFactors::default())
    }
}

impl From<&VisibilityFactors> for AiVisibilityFactors {
    fn from(f: &VisibilityFactors) -> Self {
        Self { stance: f.stance, size: f.size, disguise: f.disguise, modifiers: f.modifiers }
    }
}

impl AiVisibilityFactors {
    /// The per-target part of [`VisibilityFactors`] (light and speed at their neutral defaults;
    /// [`check_visibility`] sets them per check).
    pub(crate) fn factors(&self) -> VisibilityFactors {
        VisibilityFactors {
            stance: self.stance,
            size: self.size,
            disguise: self.disguise,
            modifiers: self.modifiers,
            ..VisibilityFactors::default()
        }
    }
}

/// `SightCheck::visibility` for one check (module docs): the target's `base` factors
/// ([`AiVisibilityFactors::factors`]), light sample and velocity; the eye-to-chest offset `to`, the
/// observer's `facing` and `darkness`.
pub(crate) fn check_visibility(
    rules: &dyn DetectionRules,
    base: &VisibilityFactors,
    light: Option<f32>,
    velocity: Vec3,
    facing: Vec3,
    to: Vec3,
    darkness: f32,
) -> f32 {
    let mut factors = *base;
    factors.light = light.map_or(VISIBILITY_UNMODELLED, |l| apply_darkness(l, darkness));
    factors.speed_mps = velocity.length();
    let angle = facing.angle_between(to);
    let input = DetectIn {
        distance_m: to.length(),
        angle_rad: if angle.is_finite() { angle } else { 0.0 },
        factors,
        ..DetectIn::default()
    };
    visibility_from_concealment(rules.concealment(&input))
}
