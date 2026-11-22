//! What the scatter asks of the ground. The caller implements [`GroundSampler`] over its terrain (or a
//! generated world); tests use a synthetic one.

use glam::Vec3;

/// The ground at one world (x, z) point. The scatter reuses one value per tile, so a sampler fills it in place
/// and `channels` keeps its allocation.
#[derive(Clone, Debug)]
pub struct GroundSample {
    /// Ground height (world Y), metres.
    pub height: f32,
    /// Unit surface normal (Y up).
    pub normal: Vec3,
    /// Signed distance to the water surface, metres: positive above or away from water, negative submerged. A
    /// world with no water nearby reports a large positive value.
    pub water_distance: f32,
    /// Channel weights, indexed by the rule table's `channel` (terrain layer weights, material fields, ...).
    pub channels: Vec<f32>,
    /// `false` where there is no ground (a hole, outside the terrain): the point gets no instances.
    pub valid: bool,
}

impl GroundSample {
    /// Flat, dry ground at height 0 with no channels: a blank for a sampler to fill.
    pub const fn blank() -> Self {
        Self { height: 0.0, normal: Vec3::Y, water_distance: f32::MAX, channels: Vec::new(), valid: true }
    }
}

impl Default for GroundSample {
    fn default() -> Self {
        Self::blank()
    }
}

/// The caller's ground. Implementations must be pure functions of (x, z): the scatter's determinism (same
/// instances on every peer, every run, any tile order) rests on it.
pub trait GroundSampler {
    /// Fill `out` for world point (x, z). `out` holds the previous point's values; overwrite every field (clear
    /// and refill `channels`).
    fn sample(&self, x: f64, z: f64, out: &mut GroundSample);
}
