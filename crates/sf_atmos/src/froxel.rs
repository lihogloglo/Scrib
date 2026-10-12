//! Froxel volumetric fog (weather.md "Fog", F3): the grid, its exponential
//! slice distribution and the integration step, CPU twins of
//! `froxel_common.wesl` (change both together).
//!
//! Technique: Wronski 2014 ("Volumetric fog", Assassin's Creed IV) and
//! Hillaire 2015 ("Physically based and unified volumetric rendering in
//! Frostbite"): a camera-aligned grid of screen tiles × depth slices holds
//! (in-scattered light × scattering, extinction); a second pass integrates
//! it front to back into (in-scatter, transmittance) that any pixel reads
//! with one trilinear fetch at its depth.

/// The froxel grid and its reach (one preset row of weather.md's budget table).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FroxelSettings {
    /// Tiles across, tiles down, depth slices.
    pub size: [u32; 3],
    /// First slice boundary, meters (the slices grow exponentially from here).
    pub near: f32,
    /// Where the froxels end, meters; the analytic layers take over past it.
    pub far: f32,
    /// Weight of the reprojected history, 0..1 (0: no temporal filter).
    pub history: f32,
    /// Point lights lit into the fog (the nearest ones), at most [`MAX_FOG_LIGHTS`].
    pub max_lights: u32,
}

/// The froxel shader's light array length (`froxel_common.wesl`).
pub const MAX_FOG_LIGHTS: usize = 32;

impl FroxelSettings {
    /// weather.md Medium: 128×72×48 to 256 m.
    #[must_use]
    pub const fn medium() -> Self {
        Self { size: [128, 72, 48], near: 0.5, far: 256.0, history: 0.9, max_lights: 16 }
    }
    /// weather.md High: 160×90×64 (16 px tiles at 1440p) to 384 m.
    #[must_use]
    pub const fn high() -> Self {
        Self { size: [160, 90, 64], near: 0.5, far: 384.0, history: 0.9, max_lights: 32 }
    }
    /// weather.md Ultra: 160×90×128 to 512 m (inside the 504 m cascades' reach, give or take).
    #[must_use]
    pub const fn ultra() -> Self {
        Self { size: [160, 90, 128], near: 0.5, far: 512.0, history: 0.9, max_lights: 32 }
    }

    /// Distance (along the view ray) of slice coordinate `z` in 0..=1:
    /// 0 at the camera's near boundary, 1 at `far`; exponential, so every
    /// slice is the same fraction of its distance (Frostbite's depth split).
    #[must_use]
    pub fn slice_distance(&self, z: f32) -> f32 {
        self.near * (self.far / self.near).powf(z)
    }

    /// Inverse of [`Self::slice_distance`], clamped to 0..=1.
    #[must_use]
    pub fn slice_coord(&self, d: f32) -> f32 {
        if d <= self.near {
            return 0.0;
        }
        ((d / self.near).ln() / (self.far / self.near).ln()).clamp(0.0, 1.0)
    }

    /// Thickness of slice `s`, meters (slice 0 also covers the camera's first `near` meters).
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn slice_thickness(&self, s: u32) -> f32 {
        let n = self.size[2] as f32;
        let d0 = if s == 0 { 0.0 } else { self.slice_distance(s as f32 / n) };
        self.slice_distance((s + 1) as f32 / n) - d0
    }
}

impl Default for FroxelSettings {
    fn default() -> Self {
        Self::high()
    }
}

/// One integration step (Hillaire 2015, energy-conserving): a slab of
/// `thickness` m with in-scattering `scatter` (σs × incoming light) and
/// extinction `sigma_t`, appended behind the running (light, transmittance).
#[must_use]
pub fn integrate_step(light: [f32; 3], transmittance: f32, scatter: [f32; 3], sigma_t: f32, thickness: f32) -> ([f32; 3], f32) {
    let t = (-sigma_t * thickness).exp();
    // ∫ S e^(−σt x) dx over the slab; S·thickness when σt is ~0.
    let k = if sigma_t > 1e-7 { (1.0 - t) / sigma_t } else { thickness };
    let out = [light[0] + transmittance * scatter[0] * k, light[1] + transmittance * scatter[1] * k, light[2] + transmittance * scatter[2] * k];
    (out, transmittance * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_round_trip_and_cover_the_range() {
        let f = FroxelSettings::high();
        assert!((f.slice_distance(0.0) - f.near).abs() < 1e-5);
        assert!((f.slice_distance(1.0) - f.far).abs() < 1e-2);
        for d in [1.0_f32, 10.0, 100.0, 383.0] {
            assert!((f.slice_distance(f.slice_coord(d)) - d).abs() < d * 1e-4, "{d}");
        }
        let total: f32 = (0..f.size[2]).map(|s| f.slice_thickness(s)).sum();
        assert!((total - f.far).abs() < 1e-2, "{total}");
        // Each slice is ~10 % of its distance at High: 38 m deep at the far end.
        assert!(f.slice_thickness(63) < 40.0 && f.slice_thickness(63) > 36.0);
    }

    #[test]
    fn integration_matches_the_closed_form_of_a_homogeneous_medium() {
        // Constant medium: L = S/σt (1 − e^(−σt d)), T = e^(−σt d), however it is sliced.
        let f = FroxelSettings::high();
        let (s, sigma) = (2.0_f32, 0.01_f32);
        let (mut l, mut t) = ([0.0; 3], 1.0);
        for i in 0..f.size[2] {
            (l, t) = integrate_step(l, t, [s * sigma; 3], sigma, f.slice_thickness(i));
        }
        let expect_t = (-sigma * f.far).exp();
        assert!((t - expect_t).abs() < 1e-4, "{t} vs {expect_t}");
        assert!((l[0] - s * (1.0 - expect_t)).abs() < 1e-3, "{}", l[0]);
        // Empty medium: nothing changes.
        let (l0, t0) = integrate_step([0.5; 3], 0.7, [0.0; 3], 0.0, 10.0);
        assert!((l0[0] - 0.5).abs() < 1e-6 && (t0 - 0.7).abs() < 1e-6);
    }
}
