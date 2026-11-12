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

/// Directions the fog's sky fill averages (`fog_light.wesl` `SKY_FILL_SAMPLES`, keep in step).
pub const SKY_FILL_SAMPLES: u32 = 64;

/// The golden angle, radians: successive spiral points' azimuth step.
const GOLDEN_ANGLE: f32 = 2.399_963_2;

/// The `i`-th of `n` sky-fill directions (y up, the sky-view LUT's frame): a
/// Fibonacci spiral over the upper hemisphere with `cos θ` uniform in (0, 1),
/// so every direction stands for the same solid angle, turned about y by
/// `azimuth` (radians, from +x toward +z).
#[allow(clippy::cast_precision_loss)]
pub fn sky_fill_direction(i: u32, n: u32, azimuth: f32) -> Vec3 {
    let mu = 1.0 - (i as f32 + 0.5) / n as f32;
    let phi = i as f32 * GOLDEN_ANGLE + azimuth;
    let s = (1.0 - mu * mu).max(0.0).sqrt();
    Vec3::new(s * phi.cos(), mu, s * phi.sin())
}

/// The fog's sky fill (CPU twin of `fog_light.wesl`): the sky dome's mean
/// radiance, the isotropic sky ambient that every froxel and far-fog pixel
/// in-scatters whatever its view direction (the SH L0 band of the sky light,
/// as UE's volumetric fog and Frostbite's (Hillaire 2015) light their fog
/// with). One value per frame, so a bright aureole around a low sun
/// brightens the fill a little everywhere instead of lighting a column of fog
/// toward the sun's azimuth (FG-9); the sun's own forward lobe comes from the
/// key light's phase function.
///
/// The spiral turns with the key light (`key`, y up), so a sky symmetric about
/// the sun's vertical plane gives the same fill toward any sun azimuth, and
/// its error (~3 % against the true dome mean for a sharp g 0.8 aureole, test
/// `sky_fill_spread`) changes smoothly as the sun rises instead of jumping as
/// the aureole crosses sample points.
#[allow(clippy::cast_precision_loss)]
pub fn sky_fill(key: Vec3, radiance: impl Fn(Vec3) -> Vec3) -> Vec3 {
    let azimuth = key.z.atan2(key.x);
    let sum: Vec3 = (0..SKY_FILL_SAMPLES).map(|i| radiance(sky_fill_direction(i, SKY_FILL_SAMPLES, azimuth))).sum();
    sum / SKY_FILL_SAMPLES as f32
}

/// How much of the view's ambient light (the flat ambient that lights the
/// surfaces, on top of the sky's own light) the fog in-scatters, 1 = all of it
/// (weather.md "Fog", night fill). The fog is then lit by the same light as
/// the ground it veils, as UE lights its height and volumetric fog with the
/// sky light and Frostbite (Hillaire 2015) its media with the scene's ambient
/// probes: without it a host's night ambient floor lights the ground and not
/// the fog, which becomes a black absorber you see far through (WE-9).
/// 0 is the sky-only fill (the A/B "off").
pub const VIEW_AMBIENT_IN_FOG: f32 = 1.0;

/// The fog's isotropic fill (CPU twin of `fog_light.wesl` texel 0): the sky
/// fill plus the view's ambient radiance × `scale` ([`VIEW_AMBIENT_IN_FOG`]).
/// Both are radiance in the same units (an isotropic medium in uniform
/// radiance `L` in-scatters `L`, as a white Lambert surface reflects `L`).
pub fn fog_fill(sky_fill: Vec3, view_ambient: Vec3, scale: f32) -> Vec3 {
    sky_fill + view_ambient.max(Vec3::ZERO) * scale.max(0.0)
}

/// Fog along a view ray from the camera to some distance: the radiance it in-scatters toward the
/// eye (premultiplied) and its transmittance, as `clouds_composite` evaluates it per pixel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogSegment {
    /// In-scattered radiance reaching the eye.
    pub light: Vec3,
    /// Transmittance, 0..1.
    pub transmittance: f32,
}

impl FogSegment {
    /// No fog.
    pub const CLEAR: Self = Self { light: Vec3::ZERO, transmittance: 1.0 };
}

/// The composite's output over a cloud layer (CPU twin of `cloud_composite.wesl`, WE12-FIX):
/// `(rgb, alpha)` for the blend `rgb + alpha · scene`. `cloud` is the cloud's premultiplied
/// radiance and `cloud_t` its transmittance; `full` is the fog to the pixel's depth, `front`
/// the fog to the cloud's transmittance-weighted depth (`tc`, never past the pixel's depth).
///
/// The cloud sits inside the fog column (UE5 Volumetric Cloud, Hillaire 2016/2020): the fog in
/// front of it dims and lights it, and only the scene behind gets the rest of the column, seen
/// through both. With `full = front + T_f · back`, the three layers compose to
/// `L_f + T_f · C + T_c · (L − L_f)`, alpha `T_c · T`. At `tc ≥ d` (`front == full`) this is
/// [`compose_fog_in_front`] exactly.
pub fn compose_fog_split(cloud: Vec3, cloud_t: f32, full: FogSegment, front: FogSegment) -> (Vec3, f32) {
    let behind = (full.light - front.light).max(Vec3::ZERO);
    (front.light + cloud * front.transmittance + behind * cloud_t, cloud_t * full.transmittance)
}

/// The old order (`--off cloudfog`): the whole fog column in front of the cloud,
/// `T · C + L`, alpha `T_c · T`. Wrong from above the deck: the fog behind the cloud hides it.
pub fn compose_fog_in_front(cloud: Vec3, cloud_t: f32, full: FogSegment) -> (Vec3, f32) {
    (cloud * full.transmittance + full.light, cloud_t * full.transmittance)
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

    /// The spiral covers the upper hemisphere evenly: unit directions, all
    /// above the horizon, mean height 1/2 (uniform in solid angle).
    #[test]
    fn sky_fill_directions_are_uniform() {
        let n = SKY_FILL_SAMPLES;
        let dirs: Vec<Vec3> = (0..n).map(|i| sky_fill_direction(i, n, 0.7)).collect();
        assert!(dirs.iter().all(|d| (d.length() - 1.0).abs() < 1e-5 && d.y > 0.0));
        let mean_y = dirs.iter().map(|d| d.y).sum::<f32>() / n as f32;
        assert!((mean_y - 0.5).abs() < 1e-5, "{mean_y}");
        let mean_xz = dirs.iter().map(|d| Vec2::new(d.x, d.z)).sum::<Vec2>() / n as f32;
        assert!(mean_xz.length() < 0.02, "{mean_xz}");
        let c = sky_fill(Vec3::Y, |_| Vec3::new(1.0, 2.0, 3.0));
        assert!((c - Vec3::new(1.0, 2.0, 3.0)).abs().max_element() < 1e-5, "{c}");
    }

    /// A low sun's sky: 1 + an HG g 0.8 aureole peaking at 20x the sky.
    fn aureole_sky(sun: Vec3) -> impl Fn(Vec3) -> Vec3 {
        let hg = |c: f32| (1.0 - 0.64) / (1.64 - 1.6 * c).powf(1.5);
        let peak = hg(1.0);
        move |d: Vec3| Vec3::splat(1.0 + 20.0 * hg(d.dot(sun)) / peak)
    }

    /// The dome's true mean radiance, by a fine midpoint integral in (cos θ, φ).
    fn dome_mean(radiance: impl Fn(Vec3) -> Vec3) -> f32 {
        let (nm, np) = (200, 400);
        let mut t = 0.0;
        for a in 0..nm {
            let mu = (a as f32 + 0.5) / nm as f32;
            let s = (1.0 - mu * mu).sqrt();
            for b in 0..np {
                let phi = (b as f32 + 0.5) / np as f32 * std::f32::consts::TAU;
                t += radiance(Vec3::new(s * phi.cos(), mu, s * phi.sin())).x;
            }
        }
        t / (nm * np) as f32
    }

    fn sun_at(azimuth: f32, elevation: f32) -> Vec3 {
        Vec3::new(azimuth.cos() * elevation.cos(), elevation.sin(), azimuth.sin() * elevation.cos())
    }

    /// FG-9: with a low sun's aureole the fill is the dome's mean (one value,
    /// whatever the view direction), the same toward any sun azimuth, and far
    /// below what the old per-azimuth fill read toward the sun.
    #[test]
    fn sky_fill_has_no_sun_spike() {
        let truth = dome_mean(aureole_sky(sun_at(0.0, 0.05)));
        let fills: Vec<f32> = (0..64)
            .map(|k| {
                let sun = sun_at(k as f32 / 64.0 * std::f32::consts::TAU, 0.05);
                sky_fill(sun, aureole_sky(sun)).x
            })
            .collect();
        let (lo, hi) = fills.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &f| (lo.min(f), hi.max(f)));
        assert!(hi - lo < 1e-4 * truth, "fill {lo}..{hi} across sun azimuths");
        assert!((lo - truth).abs() < 0.05 * truth, "fill {lo}, dome mean {truth}");
        // The old fill toward the sun: (sky 0.05 rad over the horizon + zenith) / 2.
        let sky = aureole_sky(sun_at(0.0, 0.05));
        let old_toward_sun = f32::midpoint(sky(Vec3::new(1.0, 0.05, 0.0).normalize()).x, sky(Vec3::Y).x);
        assert!(hi < 0.25 * old_toward_sun, "fill {hi}, old {old_toward_sun}");
    }

    /// The fill's error against the dome mean as a low sun rises (sun-aligned spiral).
    #[test]
    #[ignore = "measurement: sky fill quadrature error over sun elevation"]
    fn sky_fill_spread() {
        let (mut worst, mut step, mut prev) = (0.0f32, 0.0f32, 0.0f32);
        for k in 0..=100 {
            let sun = sun_at(0.3, k as f32 * 0.005);
            let t = dome_mean(aureole_sky(sun));
            let e = (sky_fill(sun, aureole_sky(sun)).x - t) / t;
            if k > 0 {
                step = step.max((e - prev).abs());
            }
            (prev, worst) = (e, worst.max(e.abs()));
        }
        println!("sun 0-0.5 rad: worst error {:.1} %, worst change per 0.005 rad {:.2} %", worst * 100.0, step * 100.0);
    }

    /// Three explicit layers (fog to the cloud, the cloud, fog behind it) composed back to
    /// front: the reference for the split.
    fn layered(front: FogSegment, cloud: Vec3, cloud_t: f32, back: FogSegment) -> (Vec3, f32) {
        let light = front.light + front.transmittance * (cloud + cloud_t * back.light);
        (light, front.transmittance * cloud_t * back.transmittance)
    }

    fn joined(front: FogSegment, back: FogSegment) -> FogSegment {
        FogSegment { light: front.light + front.transmittance * back.light, transmittance: front.transmittance * back.transmittance }
    }

    /// No cloud in front of the pixel's depth (`tc >= d`, so `front == full`): the old order exactly.
    #[test]
    fn split_reduces_to_the_old_order_without_a_cloud_in_front() {
        let fogs = [
            FogSegment::CLEAR,
            FogSegment { light: Vec3::new(0.2, 0.25, 0.3), transmittance: 0.4 },
            FogSegment { light: Vec3::splat(0.8), transmittance: 0.0 },
        ];
        for (cloud, cloud_t) in [(Vec3::new(0.4, 0.5, 0.6), 0.3), (Vec3::ZERO, 1.0), (Vec3::splat(2.0), 0.0)] {
            for full in fogs {
                assert_eq!(compose_fog_split(cloud, cloud_t, full, full), compose_fog_in_front(cloud, cloud_t, full));
            }
        }
    }

    /// From above the deck (WE-12): opaque fog behind the cloud keeps the cloud, dimmed only by
    /// the fog in front of it; the old order lost it entirely.
    #[test]
    fn opaque_fog_behind_the_cloud_keeps_it() {
        let cloud = Vec3::new(0.9, 0.9, 1.0);
        let front = FogSegment { light: Vec3::splat(0.05), transmittance: 0.9 };
        let back = FogSegment { light: Vec3::splat(0.6), transmittance: 0.0 };
        let full = joined(front, back);
        let (rgb, alpha) = compose_fog_split(cloud, 0.0, full, front);
        assert_eq!(alpha, 0.0);
        assert!((rgb - (front.light + front.transmittance * cloud)).abs().max_element() <= 1e-6, "{rgb}");
        let (old, _) = compose_fog_in_front(cloud, 0.0, full);
        assert!((old - full.light).abs().max_element() <= 1e-6, "old order: the cloud is gone");
    }

    /// The split equals the three layers composed explicitly, for any segments.
    #[test]
    fn split_matches_the_layered_composite() {
        for i in 0..200_u32 {
            let u = |k: u32| ((i * 37 + k * 101) % 97) as f32 / 96.0;
            let front = FogSegment { light: Vec3::new(u(1), u(2), u(3)) * 0.5, transmittance: u(4) };
            let back = FogSegment { light: Vec3::new(u(5), u(6), u(7)), transmittance: u(8) };
            let (cloud, cloud_t) = (Vec3::new(u(9), u(10), u(11)) * (1.0 - u(12)), u(12));
            let got = compose_fog_split(cloud, cloud_t, joined(front, back), front);
            let want = layered(front, cloud, cloud_t, back);
            assert!((got.0 - want.0).abs().max_element() <= 1e-6 && (got.1 - want.1).abs() <= 1e-6, "{got:?} vs {want:?}");
            assert!(got.0.is_finite() && (0.0..=1.0).contains(&got.1));
        }
    }

    /// Sanity: a front pair a hair brighter than the full one (shader rounding) never gives
    /// negative light behind the cloud, and nothing is NaN.
    #[test]
    fn split_is_finite_and_never_negative() {
        let full = FogSegment { light: Vec3::splat(0.3), transmittance: 0.5 };
        let front = FogSegment { light: Vec3::splat(0.300_01), transmittance: 0.5 };
        let (rgb, alpha) = compose_fog_split(Vec3::splat(0.2), 0.5, full, front);
        assert!(rgb.is_finite() && rgb.min_element() >= 0.0 && (0.0..=1.0).contains(&alpha));
    }
}
