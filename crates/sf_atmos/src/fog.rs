//! Height fog, ground-fog banks and the far fog (weather.md "Fog", F2),
//! without Bevy: the settings the weather drives, the bank drift, and the
//! CPU twin of the analytic fog in `sf_atmos_bevy`'s `fog.wesl`.
//!
//! The model is two exponential layers over sea level (altitude, meters):
//! - **height fog**: `density · e^(−(h − base) / falloff)`, the analytic
//!   exponential height fog of Unreal (Wenzel 2006; UE `ExponentialHeightFog`):
//!   its optical depth along a ray has a closed form, so every pixel gets the
//!   exact amount for its own depth at full resolution;
//! - **ground-fog banks**: a thin layer (`falloff` a few tens of meters) whose
//!   density is also modulated by thresholded, domain-warped noise drifting
//!   with the wind (the bank function of the owner's Megalophobia
//!   `ground_fog.wgsl`, MIT OR Apache-2.0, ARCHITECTURE §10 2026-10-12).
//!
//! The renderer marches the banks at reduced resolution inside the cloud
//! pass (the "far fog", out to [`FogSettings::far`]) and stores, per traced
//! pixel, how much of the layer's analytic optical depth the noise lets
//! through ([`FogBanks`] modulation, 0..1). The full-resolution composite
//! multiplies the bank layer's closed form by it, so depth edges stay exact
//! while the banks keep their shapes. F3's froxels replace the near part of
//! the march and reuse the same density terms.

use glam::{Vec2, Vec3};

use crate::scroll::SCROLL_PERIOD;

/// Koschmieder's constant: visibility (2 % contrast) is `3.912 / σ`.
pub const KOSCHMIEDER: f32 = 3.912;

/// Extinction (1/m) that gives `visibility` meters (0 or less: none).
pub fn extinction_for_visibility(visibility: f32) -> f32 {
    if visibility > 0.0 { KOSCHMIEDER / visibility } else { 0.0 }
}

/// One exponential layer: extinction `density · e^(−(h − base) / falloff)`
/// per meter at altitude `h`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogLayer {
    /// Extinction at `base`, 1/m (0: no layer).
    pub density: f32,
    /// Height over which the density falls to 1/e, meters.
    pub falloff: f32,
    /// Altitude of `density`, meters above sea level.
    pub base: f32,
}

impl Default for FogLayer {
    fn default() -> Self {
        Self { density: 0.0, falloff: 100.0, base: 0.0 }
    }
}

/// Below the base the density stops growing past `e^MAX_EXPONENT ×` (a
/// valley under the layer's base is foggy, not opaque).
pub const MAX_EXPONENT: f32 = 2.0;

impl FogLayer {
    /// Extinction at altitude `h`, 1/m.
    pub fn density_at(&self, h: f32) -> f32 {
        self.density * (-(h - self.base) / self.falloff.max(1e-3)).min(MAX_EXPONENT).exp()
    }

    /// Optical depth along a straight ray from altitude `h0`, with vertical
    /// cosine `mu` (`dot(up, dir)`), over `d` meters: the closed form
    /// `ρ(h0)·(1 − e^(−k·μ·d)) / (k·μ)` (planar; the curvature drop is 5 m
    /// at 8 km). Written as a difference of the two end densities so a ray
    /// that dives far below the layer's top never overflows. The GPU twin is
    /// `layer_depth` in `fog.wesl`.
    pub fn optical_depth(&self, h0: f32, mu: f32, d: f32) -> f32 {
        if self.density <= 0.0 || d <= 0.0 {
            return 0.0;
        }
        let k = 1.0 / self.falloff.max(1e-3);
        let e0 = (-(h0 - self.base) * k).min(MAX_EXPONENT).exp();
        let x = k * mu * d;
        if x.abs() < 1e-3 {
            return self.density * d * e0 * (1.0 - 0.5 * x);
        }
        let e1 = (-(h0 + mu * d - self.base) * k).min(MAX_EXPONENT).exp();
        self.density * (e0 - e1) / (k * mu)
    }

    /// Altitude above which the layer is under 1/400 of its base density.
    pub fn top(&self) -> f32 {
        self.base + 6.0 * self.falloff
    }
}

/// The ground-fog banks: a [`FogLayer`] modulated by drifting noise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogBanks {
    pub layer: FogLayer,
    /// Noise value under which there is no bank (gaps, not thinning), 0..1.
    pub coverage: f32,
    /// Width of the coverage ramp.
    pub sharpness: f32,
    /// Tile of the bank noise (the weather map's G channel), meters; must
    /// divide 16 384 m. Banks are about half of it across.
    pub tile: f32,
    /// Tile of the second, finer octave (wisps), meters.
    pub detail_tile: f32,
    /// Tile of the warp field, meters.
    pub warp_tile: f32,
    /// How far the warp displaces the bank field, meters (the swirl).
    pub warp_amount: f32,
}

impl Default for FogBanks {
    fn default() -> Self {
        Self {
            layer: FogLayer { density: 0.0, falloff: 20.0, base: 0.0 },
            coverage: 0.42,
            sharpness: 0.18,
            // The G channel's coarsest octave has 2 cells per tile: 512 m cells,
            // 128 m wisps, a 1 km warp (Megalophobia: 480 m, 140 m, 1.1 km).
            tile: 1_024.0,
            detail_tile: 256.0,
            warp_tile: 2_048.0,
            warp_amount: 220.0,
        }
    }
}

/// The weather map's G channel is a smooth FBM with a narrow spread (its
/// 5-95 % range is 0.29-0.43 for the blend of two octaves): the mask
/// stretches it around its median before thresholding.
pub const NOISE_MID: f32 = 0.357;
pub const NOISE_GAIN: f32 = 6.0;

/// Smoothstep with edges `a < b`.
fn ramp(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a).max(1e-5)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl FogBanks {
    /// The bank mask from the two noise octaves (each 0..1): 0 between
    /// banks, 1 inside. The GPU twin is `bank_mask` in `cloud_trace.wesl`.
    pub fn mask(&self, coarse: f32, fine: f32) -> f32 {
        let n = (coarse * 0.65 + fine * 0.35 - NOISE_MID) * NOISE_GAIN + 0.5;
        ramp(self.coverage, self.coverage + self.sharpness, n)
    }
}

/// Everything the fog renders with (the weather writes it).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogSettings {
    pub height: FogLayer,
    pub banks: FogBanks,
    /// Single-scattering albedo per channel (water fog ~1, ash grey-brown).
    pub albedo: Vec3,
    /// Henyey-Greenstein g of the sun/moon in-scattering (fog droplets
    /// scatter forward: 0.6-0.8; ash less).
    pub phase_g: f32,
    /// Sky fill multiplier.
    pub ambient: f32,
    /// How far the banks are marched, meters (the far fog's reach).
    pub far: f32,
    /// The distance sky pixels are fogged to, meters.
    pub sky_distance: f32,
    /// Ground wind the banks drift with (world X, Z), m/s.
    pub wind: Vec2,
}

impl Default for FogSettings {
    fn default() -> Self {
        Self {
            height: FogLayer::default(),
            banks: FogBanks::default(),
            albedo: Vec3::ONE,
            phase_g: 0.6,
            ambient: 1.0,
            far: 6_000.0,
            sky_distance: 20_000.0,
            wind: Vec2::new(2.0, 0.5),
        }
    }
}

impl FogSettings {
    /// Any fog at all (off: the passes skip it, no cost).
    pub fn any(&self) -> bool {
        self.height.density > 0.0 || self.banks.layer.density > 0.0
    }

    /// Total extinction at altitude `h` with the banks at `bank_mask`.
    pub fn extinction_at(&self, h: f32, bank_mask: f32) -> f32 {
        self.height.density_at(h) + self.banks.layer.density_at(h) * bank_mask
    }

    /// Transmittance along a ray (see [`FogLayer::optical_depth`]) with the
    /// banks modulated by `bank_modulation` (what the march stores).
    pub fn transmittance(&self, h0: f32, mu: f32, d: f32, bank_modulation: f32) -> f32 {
        let tau = self.height.optical_depth(h0, mu, d) + self.banks.layer.optical_depth(h0, mu, d) * bank_modulation;
        (-tau).exp()
    }

    /// Meteorological visibility at altitude `h` inside a bank (meters;
    /// `f32::INFINITY` without fog). `sf_weather`'s `visibility_m` reads it.
    pub fn visibility_at(&self, h: f32) -> f32 {
        let s = self.extinction_at(h, 1.0);
        if s > 0.0 { KOSCHMIEDER / s } else { f32::INFINITY }
    }
}

/// The banks' drift: wind travel (coarse octave and warp), wrapped at
/// [`SCROLL_PERIOD`] like the clouds' scroll, turned into per-texture
/// offsets with the floating origin's phase.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FogDrift {
    pub travel: [f64; 2],
    pub detail: [f64; 2],
    pub warp: [f64; 2],
}

/// The warp field drifts slower than the banks, so a bank deforms as it
/// travels instead of sliding rigidly (Megalophobia's 0.35).
pub const WARP_DRIFT: f64 = 0.35;
/// The fine octave drifts a little faster (wisps run ahead).
pub const DETAIL_DRIFT: f64 = 1.35;

/// Texture offsets (UV, 0..1) for the bank noise.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FogOffsets {
    pub bank: Vec2,
    pub detail: Vec2,
    pub warp: Vec2,
}

impl FogDrift {
    pub fn advance(&mut self, wind: Vec2, dt: f64) {
        let wrap = |v: f64| v.rem_euclid(SCROLL_PERIOD);
        let (wx, wz) = (f64::from(wind.x), f64::from(wind.y));
        self.travel = [wrap(self.travel[0] + wx * dt), wrap(self.travel[1] + wz * dt)];
        self.detail = [wrap(self.detail[0] + DETAIL_DRIFT * wx * dt), wrap(self.detail[1] + DETAIL_DRIFT * wz * dt)];
        self.warp = [wrap(self.warp[0] + WARP_DRIFT * wx * dt), wrap(self.warp[1] + WARP_DRIFT * wz * dt)];
    }

    /// Offsets for an origin at world `(x, z)` meters.
    pub fn offsets(&self, origin_xz: [f64; 2], banks: &FogBanks) -> FogOffsets {
        let off = |tile: f32, travel: [f64; 2]| {
            let t = f64::from(tile.abs().max(1e-3));
            #[allow(clippy::cast_possible_truncation)]
            let f = |o: f64, w: f64| ((o - w) / t).rem_euclid(1.0) as f32;
            Vec2::new(f(origin_xz[0], travel[0]), f(origin_xz[1], travel[1]))
        };
        FogOffsets {
            bank: off(banks.tile, self.travel),
            detail: off(banks.detail_tile, self.detail),
            warp: off(banks.warp_tile, self.warp),
        }
    }
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::float_cmp)]
mod tests {
    use super::*;

    fn numeric(layer: &FogLayer, h0: f32, mu: f32, d: f32) -> f32 {
        let n = 20_000;
        let ds = d / n as f32;
        (0..n).map(|i| layer.density_at(h0 + mu * (i as f32 + 0.5) * ds) * ds).sum()
    }

    /// The closed form matches a fine numeric integral for rays up, down and level.
    #[test]
    fn optical_depth_matches_numeric() {
        let layer = FogLayer { density: 0.01, falloff: 50.0, base: 10.0 };
        for (h0, mu, d) in [(20.0, 0.0, 500.0), (20.0, 0.3, 2_000.0), (300.0, -0.2, 1_400.0), (15.0, 1e-5, 800.0), (60.0, -0.05, 900.0)] {
            let a = layer.optical_depth(h0, mu, d);
            let b = numeric(&layer, h0, mu, d);
            assert!((a - b).abs() <= 2e-3 * b.max(1e-3), "h0 {h0} mu {mu} d {d}: {a} vs {b}");
        }
    }

    /// Looking down from high above a thin layer never overflows.
    #[test]
    fn optical_depth_is_finite_from_above() {
        let layer = FogLayer { density: 0.03, falloff: 15.0, base: 0.0 };
        let tau = layer.optical_depth(2_500.0, -0.9, 2_700.0);
        assert!(tau.is_finite() && tau > 0.0, "{tau}");
    }

    #[test]
    fn visibility_is_koschmieder() {
        let fog = FogSettings { height: FogLayer { density: extinction_for_visibility(500.0), falloff: 100.0, base: 0.0 }, ..FogSettings::default() };
        assert!((fog.visibility_at(0.0) - 500.0).abs() < 0.5);
        assert!(FogSettings::default().visibility_at(0.0).is_infinite());
        assert!(!FogSettings::default().any());
    }

    /// The mask is a threshold: nothing under coverage, full well above.
    #[test]
    fn bank_mask_thresholds() {
        let b = FogBanks::default();
        assert_eq!(b.mask(0.2, 0.2), 0.0);
        assert_eq!(b.mask(1.0, 1.0), 1.0);
    }

    /// Calibration: at the default coverage the banks cover a fair share of
    /// the ground (the weather map's G channel, sampled as the shader does,
    /// without the warp), with real gaps.
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn banks_cover_part_of_the_ground() {
        let map = crate::noise::weather_noise(0x5EED_C10D);
        let n = crate::WEATHER_SIZE as usize;
        let g = |u: f32, v: f32| {
            let (x, y) = ((u.rem_euclid(1.0) * n as f32) as usize % n, (v.rem_euclid(1.0) * n as f32) as usize % n);
            f32::from(map[(y * n + x) * 4 + 1]) / 255.0
        };
        let banks = FogBanks::default();
        let (mut inside, mut gaps, total) = (0, 0, 64 * 64);
        for i in 0..64 {
            for j in 0..64 {
                let (x, z) = (i as f32 * 32.0, j as f32 * 32.0);
                let m = banks.mask(g(x / banks.tile, z / banks.tile), g(z / banks.detail_tile, x / banks.detail_tile));
                inside += usize::from(m > 0.5);
                gaps += usize::from(m == 0.0);
            }
        }
        let (inside, gaps) = (inside as f32 / total as f32, gaps as f32 / total as f32);
        assert!((0.25..0.75).contains(&inside) && gaps > 0.1, "inside {inside}, gaps {gaps}");
    }

    /// Origin moves don't move the banks: offset(origin + Δ) = offset(origin) + Δ/tile.
    #[test]
    fn drift_offsets_are_origin_phased() {
        let banks = FogBanks::default();
        let mut drift = FogDrift::default();
        drift.advance(Vec2::new(3.0, -1.0), 37.0);
        let a = drift.offsets([1_000.0, 2_000.0], &banks);
        let b = drift.offsets([1_000.0 + 512.0, 2_000.0], &banks);
        let expect = (a.bank.x + 512.0 / banks.tile).rem_euclid(1.0);
        assert!((b.bank.x - expect).abs() < 1e-5);
        assert!((b.bank.y - a.bank.y).abs() < 1e-6);
    }
}
