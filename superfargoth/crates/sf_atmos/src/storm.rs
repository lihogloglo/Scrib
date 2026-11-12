//! Storm shape (CW-1, `weather/distant-weather.md` "CW-1 Storm shape"): the
//! distant precipitation medium shaped by the clouds and integrated analytically.
//! CPU twin of `far_weather_shaped` in `sf_atmos_bevy`'s `cloud_trace.wesl`.
//!
//! - Rain falls under the clouds' thick cores ([`rain_core`]), reaches up into the cloud's
//!   visible base and thins toward sea level ([`curtain_profile`]).
//! - The map's border blend (a smoothstep over the band) is re-blended over a narrow,
//!   noise-displaced edge ([`sharpen_border_width`]), pre-filtered to the march's sample
//!   spacing along the ray and widened back to the map's band near the camera
//!   ([`edge_half_width`]; S2: no pop walking in).
//! - The medium is integrated per segment in closed form, density linear between fixed
//!   samples ([`integrate`]), so the result is a continuous function of the ray.

use crate::density::ramp;

/// Segments of the shaped far-weather march (the WGSL `STORM_SEGMENTS`).
pub const STORM_SEGMENTS: u32 = 16;
/// Weather-noise field floor of the rain cores (a full deck still rains in cells, not everywhere).
pub const CORE_FIELD: f32 = 0.4;
/// Width of the core ramp, in field units.
pub const CORE_RAMP: f32 = 0.25;
/// Start of the shaped march, m from the camera (the WGSL `STORM_T0`).
pub const STORM_T0: f32 = 32.0;
/// Distances (m) over which the edge sharpens from the map's band (camera side) to the narrow
/// edge: the near precipitation FX ramp over the band, so the far rain hands off on that ramp.
pub const NEAR_SHARP: (f32, f32) = (128.0, 256.0);

/// The storm-shape settings (framework defaults in metres; a game's adapter overrides them).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StormShape {
    /// `false`: the `WE4d` far weather (`--off stormshape`, the A/B).
    pub enabled: bool,
    /// Width of the far curtain's region edge, m (re-blend of the map's band).
    pub edge_m: f32,
    /// Amplitude of the world-anchored noise that displaces the edge, m.
    pub edge_noise_m: f32,
    /// Rain density left at sea level, fraction of the density under the base.
    pub ground_fraction: f32,
}

impl Default for StormShape {
    /// 64 m edge (the weather map's texel at a 256 m band: the narrowest edge the map places
    /// without aliasing) and ±48 m noise (|noise| + edge/2 stays inside half the band).
    fn default() -> Self {
        Self { enabled: true, edge_m: 64.0, edge_noise_m: 48.0, ground_fraction: 0.35 }
    }
}

impl StormShape {
    /// The settings clamped so the displaced edge stays inside the band (`band_m`).
    #[must_use]
    pub fn fitted(self, band_m: f32) -> Self {
        let half = 0.5 * band_m.max(1.0);
        let edge_m = self.edge_m.clamp(1.0, band_m.max(1.0));
        let edge_noise_m = self.edge_noise_m.clamp(0.0, (half - 0.5 * edge_m).max(0.0));
        Self { edge_m, edge_noise_m, ground_fraction: self.ground_fraction.clamp(0.0, 1.0), ..self }
    }
}

/// Inverse of `3t² - 2t³` on [0, 1] (closed form).
#[must_use]
pub fn inverse_smoothstep(y: f32) -> f32 {
    let y = y.clamp(0.0, 1.0);
    0.5 - ((1.0 - 2.0 * y).asin() / 3.0).sin()
}

/// Signed distance (m, positive on the wetter side) to the border of a point whose value sits
/// at fraction `s` between the local min and max of a map blended by `sf_weather`'s
/// `border_weight` (smoothstep over ±half the band).
#[must_use]
pub fn border_distance(s: f32, band_m: f32) -> f32 {
    let half = 0.5 * band_m.max(1.0);
    let x = 2.0 * s.clamp(0.0, 1.0) - 1.0;
    half * x.signum() * inverse_smoothstep(x.abs())
}

/// The border's blend profile at signed distance `d` for half-width `h`: `border_weight`'s
/// shape (0.5 on the border, a smoothstep of `|d| / h` either side), so `h` = half the band
/// with no noise gives the map's own blend back.
#[must_use]
pub fn edge_profile(d: f32, h: f32) -> f32 {
    let t = (d.abs() / h.max(1e-3)).min(1.0);
    0.5 + 0.5 * d.signum() * t * t * (3.0 - 2.0 * t)
}

/// Spacing (m) of the march's fixed quadratic samples around distance `t` on
/// `[STORM_T0, STORM_T0 + span]`: `dt/di = 2·span·u/n`, `u = sqrt((t - t0) / span)`.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn sample_spacing(t: f32, span: f32, n: u32) -> f32 {
    2.0 * (span.max(0.0) * (t - STORM_T0).max(0.0)).sqrt() / n.max(1) as f32
}

/// Half-width (m) of the far edge at distance `t` of a march over `span`: the configured edge,
/// widened to the sample spacing × `along` (how fast the ray crosses the border, |horizontal
/// ray · border normal|, 1 worst case) so density linear between samples resolves it (a
/// pre-filter: no step as the edge crosses a sample), and back to half the band within
/// [`NEAR_SHARP`] of the camera. Capped at half the band (the widest edge the map encodes).
#[must_use]
pub fn edge_half_width(t: f32, span: f32, along: f32, band_m: f32, shape: &StormShape) -> f32 {
    let half = 0.5 * band_m.max(1.0);
    let edge = 0.5 * shape.edge_m.max(1e-3);
    let near = half * (1.0 - ramp(NEAR_SHARP.0, NEAR_SHARP.1, t));
    let spacing = sample_spacing(t, span, STORM_SEGMENTS) * along.clamp(0.0, 1.0);
    edge.max(spacing).max(near).min(half.max(edge))
}

/// The precipitation `p` re-blended over an edge of half-width `h`: `lo`/`hi` the local
/// min/max (map taps at ±band), `noise` in [0, 1] (world-anchored) displacing the edge by up
/// to `noise_m`, shrunk so the displaced edge stays inside the band (none at `h` = half the
/// band: the map's own blend). Uniform map (`hi - lo` tiny): `p` unchanged.
#[must_use]
pub fn sharpen_border_width(p: f32, lo: f32, hi: f32, noise: f32, band_m: f32, h: f32, noise_m: f32) -> f32 {
    let range = hi - lo;
    if range <= 1e-6 * hi.abs().max(1e-6) {
        return p;
    }
    let amp = noise_m.min((0.5 * band_m.max(1.0) - h).max(0.0));
    let d = border_distance((p - lo) / range, band_m) + amp * (2.0 * noise - 1.0);
    lo + range * edge_profile(d, h)
}

/// [`sharpen_border_width`] at the configured edge (`edge_m`, `edge_noise_m`): far from the
/// camera, the ray grazing the border.
#[must_use]
pub fn sharpen_border(p: f32, lo: f32, hi: f32, noise: f32, band_m: f32, shape: &StormShape) -> f32 {
    sharpen_border_width(p, lo, hi, noise, band_m, 0.5 * shape.edge_m.max(1e-3), shape.edge_noise_m)
}

/// Rain cores from the clouds' weather noise: `field` (R) from just under the coverage
/// threshold (where the deck passes half cover) and above [`CORE_FIELD`], max the storm cells
/// (`cells`, B) × `storm`. 0 under thin cloud and gaps, 1 under ordinary thick cloud and cells.
#[must_use]
pub fn rain_core(field: f32, cells: f32, coverage: f32, storm: f32) -> f32 {
    let threshold = 1.25 - 1.5 * coverage.clamp(0.0, 1.0);
    let lo = (threshold - 0.05).max(CORE_FIELD);
    let core = ramp(lo, lo + CORE_RAMP, field);
    core.max(storm.clamp(0.0, 1.0) * ramp(0.55, 0.85, cells))
}

/// Altitude of the cloud's visible base: halfway up the bottom ramp of the height gradient
/// (`mix(0.07, 0.22, kind)` of the thickness, `density::height_gradient`), where the coverage
/// remap has cut the thin bottom away.
#[must_use]
pub fn visible_base(base: f32, thickness: f32, kind: f32) -> f32 {
    let k = kind.clamp(0.0, 1.0);
    // Half of mix(0.07, 0.22, k).
    base + (0.035 + 0.075 * k) * thickness.max(0.0)
}

/// Vertical profile of the curtain at altitude `alt` (above sea level) under a layer at `base`
/// of `thickness` and type `kind`: full up to the base, fading out inside the cloud up to its
/// [`visible_base`] (the curtain meets the cloud the eye sees), thinning to `ground_fraction`
/// at sea level.
#[must_use]
pub fn curtain_profile(alt: f32, base: f32, thickness: f32, kind: f32, ground_fraction: f32) -> f32 {
    let top_m = visible_base(base, thickness, kind).max(base + 1.0);
    if alt >= top_m {
        return 0.0;
    }
    let top = 1.0 - ramp(base, top_m, alt);
    let lower = ground_fraction + (1.0 - ground_fraction) * ramp(0.0, 0.7 * base.max(1.0), alt);
    top * lower
}

/// Sample distance `i` of `n` along `[t0, t1]`: quadratic (fine near the camera), fixed.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn sample_t(i: u32, n: u32, t0: f32, t1: f32) -> f32 {
    let u = i as f32 / n.max(1) as f32;
    t0 + (t1 - t0) * u * u
}

/// Transmittance of `[t0, t1]` through extinction `sigma(t)`, integrated over `n` segments in
/// closed form with `sigma` linear between the fixed samples. Monotone in the extinction.
#[must_use]
pub fn integrate(sigma: impl Fn(f32) -> f32, t0: f32, t1: f32, n: u32) -> f32 {
    if t1 <= t0 {
        return 1.0;
    }
    let mut tau = 0.0;
    let mut prev = (t0, sigma(t0).max(0.0));
    for i in 1..=n.max(1) {
        let t = sample_t(i, n, t0, t1);
        let s = sigma(t).max(0.0);
        tau += f32::midpoint(prev.1, s) * (t - prev.0);
        prev = (t, s);
    }
    (-tau).exp()
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::float_cmp, clippy::many_single_char_names, reason = "small test counters; exact values are the point; maths names")]
mod tests {
    use super::*;

    /// `sf_weather::border_weight`'s blend of the wet side (`d` m inside it, negative outside).
    fn wet_fraction(d: f32, band: f32) -> f32 {
        let t = (d.abs() / (0.5 * band)).clamp(0.0, 1.0);
        0.5 + 0.5 * d.signum() * t * t * (3.0 - 2.0 * t)
    }

    #[test]
    fn inverse_smoothstep_inverts() {
        for k in 0..=100 {
            let t = k as f32 / 100.0;
            let y = t * t * (3.0 - 2.0 * t);
            assert!((inverse_smoothstep(y) - t).abs() < 1e-3, "t {t}");
        }
    }

    #[test]
    fn border_distance_recovers_the_band_blend() {
        for k in -12..=12 {
            let d = k as f32 * 10.0;
            let got = border_distance(wet_fraction(d, 256.0), 256.0);
            assert!((got - d).abs() < 0.5, "d {d}: {got}");
        }
    }

    #[test]
    fn border_width_is_honoured() {
        let shape = StormShape { edge_noise_m: 0.0, ..StormShape::default() };
        let (band, lo, hi) = (256.0, 0.0, 3e-3);
        let at = |d: f32| (sharpen_border(lo + (hi - lo) * wet_fraction(d, band), lo, hi, 0.5, band, &shape) - lo) / (hi - lo);
        // Full inside half the edge, none outside it; ½ on the border.
        assert!(at(0.5 * shape.edge_m + 1.0) > 0.999 && at(-0.5 * shape.edge_m - 1.0) < 1e-3);
        assert!((at(0.0) - 0.5).abs() < 1e-3);
        // The map alone is still 25 % / 75 % there (the band is 4× wider).
        assert!(wet_fraction(-0.5 * shape.edge_m - 1.0, band) > 0.1);
        // Noise displaces the edge by its amplitude at most.
        let noisy = StormShape::default();
        let n = |d: f32, noise: f32| (sharpen_border(hi * wet_fraction(d, band), lo, hi, noise, band, &noisy) - lo) / (hi - lo);
        assert!(n(-noisy.edge_noise_m + 0.5 * noisy.edge_m + 1.0, 1.0) > 0.999);
        assert!(n(noisy.edge_noise_m - 0.5 * noisy.edge_m - 1.0, 0.0) < 1e-3);
    }

    #[test]
    fn uniform_map_is_unchanged() {
        let s = StormShape::default();
        assert_eq!(sharpen_border(2e-3, 2e-3, 2e-3, 0.3, 256.0, &s), 2e-3);
        assert_eq!(sharpen_border(0.0, 0.0, 0.0, 0.3, 256.0, &s), 0.0);
    }

    #[test]
    fn fitted_keeps_the_edge_inside_the_band() {
        let s = StormShape { edge_m: 200.0, edge_noise_m: 500.0, ..StormShape::default() }.fitted(256.0);
        assert!(s.edge_noise_m + 0.5 * s.edge_m <= 128.0 + 1e-3);
        assert_eq!(StormShape::default().fitted(256.0), StormShape::default());
    }

    #[test]
    fn rain_falls_under_thick_cloud_only() {
        // A storm deck covers everything, yet rain falls only under its cores and cells.
        assert_eq!(rain_core(0.2, 0.0, 1.0, 1.0), 0.0);
        assert!(rain_core(0.9, 0.0, 1.0, 1.0) > 0.99);
        assert!(rain_core(0.2, 0.95, 1.0, 1.0) > 0.99);
        // Thin cover: no core where the deck is under half cover (below its own threshold).
        let thr = 1.25 - 1.5 * 0.4;
        assert_eq!(rain_core(thr - 0.06, 0.0, 0.4, 0.0), 0.0);
    }

    #[test]
    fn rain_falls_under_ordinary_thick_storm_cloud() {
        // Map coverage 0.45 (a thunderstorm region's deck): the old gate needed field > 0.775,
        // leaving only the storm cells. Typical thick-cloud noise (0.65-0.8) now rains without cells.
        let cov = 0.45;
        assert!(rain_core(0.7, 0.0, cov, 0.0) > 0.5);
        assert!(rain_core(0.8, 0.0, cov, 0.0) > 0.95);
        // The gaps stay dry: field 0.5 is a deck under a third of cover here.
        assert_eq!(rain_core(0.5, 0.0, cov, 0.0), 0.0);
        let thr = 1.25 - 1.5 * cov;
        assert!(ramp(thr - 0.2, thr + 0.2, 0.5) < 0.35);
    }

    #[test]
    fn curtain_reaches_the_visible_base_and_thins_to_the_ground() {
        let (base, thick, g) = (1200.0, 4000.0, 0.35);
        for kind in [0.0, 0.5, 1.0] {
            let vis = visible_base(base, thick, kind);
            assert!(vis > base + 100.0, "kind {kind}: the visible base sits inside the layer");
            // Full at the geometric base, rain still falling just under the visible base, none at it.
            assert!((curtain_profile(base, base, thick, kind, g) - 1.0).abs() < 1e-6);
            assert!(curtain_profile(vis - 1.0, base, thick, kind, g) > 0.0);
            assert_eq!(curtain_profile(vis, base, thick, kind, g), 0.0);
        }
        // The storm deck (kind 1, the 0.22 × 4 km bottom ramp): the curtain top is 440 m up.
        assert!((visible_base(base, thick, 1.0) - (base + 440.0)).abs() < 1e-3);
        assert!((curtain_profile(0.0, base, thick, 1.0, g) - g).abs() < 1e-6);
        let mut last = curtain_profile(0.0, base, thick, 1.0, g);
        for k in 1..=24 {
            let v = curtain_profile(k as f32 * 50.0, base, thick, 1.0, g);
            assert!(v.is_finite() && v + 1e-6 >= last, "monotone up to the base");
            last = v;
        }
    }

    #[test]
    fn edge_is_prefiltered_to_the_sample_spacing() {
        let (s, band, span) = (StormShape::default(), 256.0, 16_000.0);
        for k in 0..=40 {
            let t = 300.0 + k as f32 * 400.0;
            let spacing = sample_spacing(t, span, STORM_SEGMENTS);
            let h = edge_half_width(t, span, 1.0, band, &s);
            // A ray crossing the border: the edge spans the spacing (up to the map's own band).
            assert!(h + 1e-3 >= spacing.min(0.5 * band) && h >= 0.5 * s.edge_m, "t {t}: {h} vs {spacing}");
            // A ray grazing it keeps the narrow edge (the silhouette across pixels).
            assert_eq!(edge_half_width(t, span, 0.0, band, &s), 0.5 * s.edge_m);
        }
        // The spacing is the march's own: the gap between consecutive samples around `t`.
        let (a, b) = (sample_t(8, STORM_SEGMENTS, STORM_T0, STORM_T0 + span), sample_t(9, STORM_SEGMENTS, STORM_T0, STORM_T0 + span));
        let mid = sample_spacing(f32::midpoint(a, b), span, STORM_SEGMENTS);
        assert!((mid - (b - a)).abs() < 0.02 * (b - a), "{mid} vs {}", b - a);
        // At the band's half-width the edge is the map's blend exactly (noise faded out).
        for k in -12..=12 {
            let d = k as f32 * 10.0;
            let p = wet_fraction(d, band);
            assert!((sharpen_border_width(p, 0.0, 1.0, 1.0, band, 0.5 * band, s.edge_noise_m) - p).abs() < 1e-3, "d {d}");
        }
        // Near the camera the edge is the band.
        assert_eq!(edge_half_width(100.0, span, 0.0, band, &s), 0.5 * band);
    }

    const HI: f32 = 0.15;

    /// Far-rain extinction (per km) at distance `t` looking straight into a storm region (border
    /// at world x = 0, wide past the march) with the camera at `c`: the S2 edge widths, or the
    /// CW-1 fixed edge (`cw1`). No noise, no cores: the edge and the camera term alone.
    fn far_sigma(c: f32, t: f32, cw1: bool) -> f32 {
        let (s, band, span) = (StormShape { edge_noise_m: 0.0, ..StormShape::default() }, 256.0, 16_000.0);
        let raw = |x: f32| HI * wet_fraction(x, band);
        let h = if cw1 { 0.5 * s.edge_m } else { edge_half_width(t, span, 1.0, band, &s) };
        let p = sharpen_border_width(raw(c + t), 0.0, HI, 0.5, band, h, 0.0);
        (p - raw(c)).max(0.0) * ramp(32.0, 128.0, t)
    }

    fn far_opacity_n(c: f32, cw1: bool, n: u32) -> f32 {
        1.0 - integrate(|t| far_sigma(c, t, cw1) * 1e-3, STORM_T0, STORM_T0 + 16_000.0, n)
    }

    #[test]
    fn walking_into_the_storm_has_no_step_larger_than_the_near_fx() {
        // The near FX intensity ramps over the band (`border_weight`): its largest change per
        // step. The march's density at each of its fixed samples changes no faster as the
        // camera walks in (CW-1's narrow edge jumped 0 -> full at a sample in 64 m).
        let step = 2.0;
        let near_step = (0..400).map(|k| -400.0 + k as f32 * step).map(|c| (wet_fraction(c + step, 256.0) - wet_fraction(c, 256.0)).abs()).fold(0.0, f32::max);
        let worst = |cw1: bool| {
            let mut w = 0.0_f32;
            for k in 0..1000 {
                let c = -1800.0 + k as f32 * step;
                for i in 0..=STORM_SEGMENTS {
                    let t = sample_t(i, STORM_SEGMENTS, STORM_T0, STORM_T0 + 16_000.0);
                    w = w.max((far_sigma(c + step, t, cw1) - far_sigma(c, t, cw1)).abs() / HI);
                }
            }
            w
        };
        let (s2, cw1) = (worst(false), worst(true));
        println!("near FX step {near_step:.4}, far density step at the samples: S2 {s2:.4}, CW-1 {cw1:.4}");
        assert!(s2.is_finite() && s2 <= near_step + 1e-4, "far rain pops: {s2} > {near_step}");
        assert!(cw1 > 2.0 * near_step, "the test sees CW-1's pop: {cw1}");
    }

    #[test]
    fn the_march_resolves_the_edge_wherever_the_camera_stands() {
        // The pop seen as opacity: the 16-segment march missing the edge between samples, its
        // error against a fine march of the same medium swinging as the edge crosses a sample.
        // Near (edge within 400 m) the edge spans the spacing; far, the spacing outgrows the
        // map's band (the widest edge it encodes), so some error is left, under CW-1's.
        let err = |cw1: bool, from: f32, to: f32| {
            (0..=200)
                .map(|k| from + (to - from) * k as f32 / 200.0)
                .map(|c| (far_opacity_n(c, cw1, STORM_SEGMENTS) - far_opacity_n(c, cw1, 4096)).abs())
                .fold(0.0, f32::max)
        };
        let (near_s2, near_cw1) = (err(false, -400.0, 200.0), err(true, -400.0, 200.0));
        let (far_s2, far_cw1) = (err(false, -3000.0, -400.0), err(true, -3000.0, -400.0));
        println!("march error vs fine reference: near S2 {near_s2:.4} CW-1 {near_cw1:.4}; far S2 {far_s2:.4} CW-1 {far_cw1:.4}");
        assert!(near_s2.is_finite() && near_s2 < 0.5 * near_cw1, "near S2 {near_s2}, CW-1 {near_cw1}");
        assert!(far_s2.is_finite() && far_s2 < far_cw1, "far S2 {far_s2}, CW-1 {far_cw1}");
    }
}
