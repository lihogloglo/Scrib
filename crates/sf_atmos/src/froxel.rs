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
    /// Where each frame's sample sits inside its froxel.
    pub jitter: FroxelJitter,
}

/// The froxels' per-frame sample placement (weather.md "Fog", "Temporal
/// sampling"). A jittered sample per frame blended into a reprojected
/// history is the standard estimator (Wronski 2014, Hillaire 2015); what
/// matters is that the error it leaves behind is noise, not a signal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FroxelJitter {
    /// Each froxel walks its own low-discrepancy sequence (R3, Roberts 2018)
    /// rotated by a per-froxel hash (Cranley-Patterson rotation), and the
    /// history is read at the froxel's centre. The residual is spatially
    /// uncorrelated noise, which the history and the screen's TAA/DLSS
    /// average away.
    #[default]
    PerFroxel,
    /// One Halton (2, 3, 5) offset for the whole volume, cycling every 16
    /// frames, history read at the jittered point (before 2026-10-19; A/B
    /// only). Every froxel errs in step: an exponential history of weight
    /// 0.9 keeps ~26 % of the cycle's fundamental, so the whole fog pulses
    /// at frame rate / 16, and screen TAA cannot clamp a coherent change.
    Shared,
    /// The froxel centre every frame (diagnosis: no temporal sampling, so
    /// shadow and density edges alias but hold still).
    Off,
}

/// Radical inverse of `i` in `base` (Halton).
#[allow(clippy::cast_precision_loss)]
fn halton(mut i: u32, base: u32) -> f32 {
    let (mut f, mut r) = (1.0, 0.0);
    while i > 0 {
        f /= base as f32;
        r += f * (i % base) as f32;
        i /= base;
    }
    r
}

/// The whole volume's sample offset for `frame`, 0..1 per axis (the
/// uniform's `jitter`); [`FroxelJitter::PerFroxel`] rotates it per froxel
/// by [`froxel_hash`] in the shader.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn frame_jitter(mode: FroxelJitter, frame: u32) -> [f32; 3] {
    match mode {
        FroxelJitter::PerFroxel => {
            // R3: powers of 1/g, g the 3D generalised golden ratio; never
            // repeats and every prefix is evenly spread (Roberts 2018).
            const G: f64 = 1.220_744_084_605_759_5;
            let n = f64::from(frame);
            [1.0 / G, 1.0 / (G * G), 1.0 / (G * G * G)].map(|a| (0.5 + a * n).fract() as f32)
        }
        FroxelJitter::Shared => {
            let j = frame % 16 + 1;
            [halton(j, 2), halton(j, 3), halton(j, 5)]
        }
        FroxelJitter::Off => [0.5; 3],
    }
}

/// A froxel's fixed offset, 0..1 per axis: pcg3d (Jarzynski and Olano
/// 2020). CPU twin of `froxel_inject.wesl`'s `froxel_hash` (keep in step).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn froxel_hash(cell: [u32; 3]) -> [f32; 3] {
    fn mix(v: &mut [u32; 3]) {
        v[0] = v[0].wrapping_add(v[1].wrapping_mul(v[2]));
        v[1] = v[1].wrapping_add(v[2].wrapping_mul(v[0]));
        v[2] = v[2].wrapping_add(v[0].wrapping_mul(v[1]));
    }
    let mut v = cell.map(|c| c.wrapping_mul(1_664_525).wrapping_add(1_013_904_223));
    mix(&mut v);
    v = v.map(|x| x ^ (x >> 16));
    mix(&mut v);
    v.map(|x| (x >> 8) as f32 / 16_777_216.0)
}

/// Where froxel `cell` samples in `frame`, 0..1 inside it (the shader's rule).
#[must_use]
pub fn sample_offset(mode: FroxelJitter, frame: u32, cell: [u32; 3]) -> [f32; 3] {
    let j = frame_jitter(mode, frame);
    if mode != FroxelJitter::PerFroxel {
        return j;
    }
    let h = froxel_hash(cell);
    [(j[0] + h[0]).fract(), (j[1] + h[1]).fract(), (j[2] + h[2]).fract()]
}

/// The froxel shader's light array length (`froxel_common.wesl`).
pub const MAX_FOG_LIGHTS: usize = 32;

impl FroxelSettings {
    /// weather.md Medium: 128×72×48 to 256 m.
    #[must_use]
    pub const fn medium() -> Self {
        Self { size: [128, 72, 48], near: 0.5, far: 256.0, history: 0.9, max_lights: 16, jitter: FroxelJitter::PerFroxel }
    }
    /// weather.md High: 160×90×64 (16 px tiles at 1440p) to 384 m.
    #[must_use]
    pub const fn high() -> Self {
        Self { size: [160, 90, 64], near: 0.5, far: 384.0, history: 0.9, max_lights: 32, jitter: FroxelJitter::PerFroxel }
    }
    /// weather.md Ultra: 160×90×128 to 512 m (inside the 504 m cascades' reach, give or take).
    #[must_use]
    pub const fn ultra() -> Self {
        Self { size: [160, 90, 128], near: 0.5, far: 512.0, history: 0.9, max_lights: 32, jitter: FroxelJitter::PerFroxel }
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

    /// Std over frames of a region's mean after the shader's history (weight
    /// 0.9): 256 froxels, each straddling a shadow edge across x at its own
    /// place and a density ramp along z (the pulse the owner saw, 2026-10-19).
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn region_flicker(mode: FroxelJitter) -> f32 {
        let n = 256_u32;
        let mut hist = vec![0.0_f32; n as usize];
        let mut means = Vec::new();
        for frame in 0..600 {
            for (i, h) in hist.iter_mut().enumerate() {
                let o = sample_offset(mode, frame, [i as u32 % 16, i as u32 / 16, 7]);
                let edge = (i as f32 + 0.5) / n as f32;
                let s = f32::from(u8::from(o[0] < edge)) + o[2];
                *h = if frame == 0 { s } else { s + (*h - s) * 0.9 };
            }
            if frame >= 300 {
                means.push(hist.iter().sum::<f32>() / n as f32);
            }
        }
        let m = means.iter().sum::<f32>() / means.len() as f32;
        (means.iter().map(|x| (x - m) * (x - m)).sum::<f32>() / means.len() as f32).sqrt()
    }

    #[test]
    fn per_froxel_jitter_leaves_noise_not_a_pulse() {
        let shared = region_flicker(FroxelJitter::Shared);
        let per = region_flicker(FroxelJitter::PerFroxel);
        println!("region flicker (std of the mean): shared {shared:.4}, per froxel {per:.4}");
        assert!(shared > 0.02, "the old shared jitter pulses the region: {shared}");
        assert!(per < shared / 4.0 && per < 0.01, "per froxel {per} vs shared {shared}");
        // Unbiased: each froxel's samples spread evenly over its cell.
        for cell in [[0, 0, 0], [3, 9, 1], [159, 89, 63]] {
            let mut sum = [0.0_f32; 3];
            for frame in 0..256 {
                let o = sample_offset(FroxelJitter::PerFroxel, frame, cell);
                for (s, v) in sum.iter_mut().zip(o) {
                    assert!((0.0..1.0).contains(&v));
                    *s += v / 256.0;
                }
            }
            assert!(sum.iter().all(|s| (s - 0.5).abs() < 0.02), "{cell:?}: {sum:?}");
        }
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
