//! One sun visibility for all haze (`docs/specs/aerial-shadow.md` "Architecture review", ROADMAP
//! 1.27 A2, FG-12): the CPU twin of `sf_atmos_bevy`'s sun-visibility pass
//! (`shaders/sun_vis_march.wesl`, `sun_vis_resolve.wesl`, `sun_vis_common.wesl`; change both
//! together).
//!
//! The canonical technique: shadow the key light's *single scattering* per pixel along the view
//! ray (Hillaire 2020 "A scalable and production ready sky and atmosphere rendering technique",
//! the per-pixel march for mountain shadows; UE "Cast Shadows on Atmosphere"; Yusov, GPU Pro 5,
//! epipolar sampling, the full-resolution form). Each pixel marches from the camera toward its
//! scene depth (sky pixels: to the reach) and reads one `sun_visibility(p)` per sample (the
//! cascades, the terrain's far map, the cloud shadow: `sun_visibility.wesl`, the froxels' rule).
//! One march yields two weighted means of that visibility:
//!
//! - `V_atmo`: weights = the atmosphere medium's scattering × its transmittance from the camera.
//!   Over a segment of constant medium the weight is exact in closed form:
//!   `∫ σs e^(−σt s) ds = T · (σs / σt) · (1 − e^(−σt Δ))` ([`atmo_segment`]).
//! - `V_fog`: weights = the height fog's density × its transmittance, only from `fog_start` (the
//!   froxels' far: they shadow the near fog themselves). Exact for any density profile:
//!   `∫ σ e^(−τ) = e^(−τ_before) · (1 − e^(−τ_segment))` ([`fog_weight`]).
//!
//! Past the march's reach (`max_distance_m`) the samples count as lit: one closed-form segment from
//! the reach to the end of the ray (the medium at its midpoint, the fog's own closed form). No
//! weight at all (no fog, no medium) = 1.
//!
//! **Sample distribution** ([`sample_distance`]): `t = reach · x²`, x the stratified sample index
//! in `[0, 1]` (jittered inside each stratum, [`jitter`]): segment lengths grow linearly, so the
//! near field (cascades, where shadows are sharp and the transmittance highest) gets the dense
//! samples, the far map's 24 m texels the sparse ones. Quadratic rather than exponential: it
//! starts at the camera with no near-plane constant and needs no tuning.
//!
//! **Temporal** ([`resolve`]): last frame's value reprojected through the pixel's world position
//! (sky: direction only), clipped to this frame's 3×3 min/max (the froxels' Karis 2014 rule,
//! [`clip_history`]), blended with the history weight; off screen = this frame alone.

/// R2 (Roberts 2018, "The unreasonable effectiveness of quasirandom sequences"): the plastic
/// constant's two inverse powers, the most uniform 2D additive recurrence.
// tunables: const (maths: 1/φ₂ and 1/φ₂² of the plastic number)
const R2_A: [f32; 2] = [0.754_877_7, 0.569_840_3];
/// R1 over the frames: the golden ratio's fractional part.
// tunables: const (maths: 1/φ, golden ratio)
const R1_A: f32 = 0.618_034;
/// The frame index's period in [`jitter`] (f32 keeps `frame · R1_A` exact enough below it).
// tunables: const (maths: f32 precision guard of the frame recurrence)
const FRAME_PERIOD: u32 = 4096;
/// Below this total weight a mean counts as "no weight" (= 1, lit).
// tunables: const (maths: division guard)
pub const MIN_WEIGHT: f32 = 1e-12;
/// Below this extinction (per unit length) a segment's weight is its linear limit `σs · Δ`.
// tunables: const (maths: 0/0 guard of the closed form)
const MIN_EXTINCTION: f32 = 1e-12;

/// The march's distance at `x` in `[0, 1]` of the way through its samples: `reach · x²`.
#[must_use]
pub fn sample_distance(x: f32, reach: f32) -> f32 {
    reach * x * x
}

/// Segment `i` of `n` (its stratum), `(t0, t1)`: `n` segments tile `[0, reach]` exactly.
#[must_use]
#[allow(clippy::cast_precision_loss, reason = "sample counts are small")]
pub fn segment(i: u32, n: u32, reach: f32) -> (f32, f32) {
    let n = n.max(1) as f32;
    (sample_distance(i as f32 / n, reach), sample_distance((i + 1) as f32 / n, reach))
}

/// The jittered sample of segment `i` of `n` (`jitter` in `[0, 1)`), inside its segment.
#[must_use]
#[allow(clippy::cast_precision_loss, reason = "sample counts are small")]
pub fn sample_at(i: u32, n: u32, jitter: f32, reach: f32) -> f32 {
    sample_distance((i as f32 + jitter) / n.max(1) as f32, reach)
}

/// This frame's jitter of the half-resolution pixel `px`, in `[0, 1)`: R2 over the pixel (so
/// neighbours err independently, noise the history and TAA average) plus R1 over the frame
/// (each pixel walks its stratum evenly over time).
#[must_use]
#[allow(clippy::cast_precision_loss, reason = "pixel coordinates and the frame period are far below 2^24")]
pub fn jitter(px: [u32; 2], frame: u32) -> f32 {
    let v = 0.5 + px[0] as f32 * R2_A[0] + px[1] as f32 * R2_A[1] + (frame % FRAME_PERIOD) as f32 * R1_A;
    v - v.floor()
}

/// The atmosphere medium at a sample: scattering and absorption coefficients (per unit length of
/// the march, the mean over the channels).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Medium {
    pub scattering: f32,
    pub absorption: f32,
}

/// One segment of length `len` through `m`, seen through `transmittance` from the camera: its
/// weight (the light it scatters toward the camera per unit of source) and its own transmittance.
#[must_use]
pub fn atmo_segment(transmittance: f32, m: Medium, len: f32) -> (f32, f32) {
    let s = m.scattering.max(0.0);
    let e = s + m.absorption.max(0.0);
    let len = len.max(0.0);
    if e <= MIN_EXTINCTION {
        return (transmittance * s * len, 1.0);
    }
    let t = (-e * len).exp();
    (transmittance * (s / e) * (1.0 - t), t)
}

/// The fog's weight of a segment: optical depth `tau_before` from the camera to the segment's
/// start, `tau_front` of its part before `fog_start` (no weight), `tau_back` of the rest.
#[must_use]
pub fn fog_weight(tau_before: f32, tau_front: f32, tau_back: f32) -> f32 {
    (-(tau_before + tau_front)).exp() * (1.0 - (-tau_back.max(0.0)).exp())
}

/// A weighted mean of visibility; no weight = 1 (lit).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mean {
    pub weight: f32,
    pub sum: f32,
}

impl Mean {
    pub fn add(&mut self, weight: f32, visibility: f32) {
        self.weight += weight;
        self.sum += weight * visibility;
    }

    #[must_use]
    pub fn value(self) -> f32 {
        if self.weight > MIN_WEIGHT { (self.sum / self.weight).clamp(0.0, 1.0) } else { 1.0 }
    }
}

/// One pixel's march.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct March {
    pub samples: u32,
    /// Where the samples stop: `min(scene distance, max distance)` (sky: the max distance), m.
    pub reach: f32,
    /// Where the weights stop: the scene distance (sky: the fog's sky distance, at least `reach`).
    /// Past `reach`: lit.
    pub end: f32,
    /// The fog's weights start here (the froxels' far; 0 without froxels), m.
    pub fog_start: f32,
    /// This frame's jitter in `[0, 1)` ([`jitter`]).
    pub jitter: f32,
    /// Atmosphere units per meter along the ray (the atmosphere's `world_to_atmosphere` scale).
    pub atmo_scale: f32,
}

/// The march: `vis(t)` the sun's visibility at distance `t`, `atmo(t)` the atmosphere medium there,
/// `fog_tau(t0, t1)` the fog's optical depth over `[t0, t1]` (`None`: no fog). Returns
/// `(V_atmo, V_fog)`.
pub fn march(march: &March, vis: impl Fn(f32) -> f32, atmo: impl Fn(f32) -> Medium, fog_tau: Option<&dyn Fn(f32, f32) -> f32>) -> (f32, f32) {
    let n = march.samples.max(1);
    let (mut va, mut vf) = (Mean::default(), Mean::default());
    let (mut ta, mut tau) = (1.0_f32, 0.0_f32);
    let fog_segment = |t0: f32, t1: f32, v: f32, tau: &mut f32, vf: &mut Mean| {
        if let Some(fog_tau) = fog_tau {
            let a0 = march.fog_start.clamp(t0, t1);
            let front = fog_tau(t0, a0);
            let back = fog_tau(a0, t1);
            if t1 > a0 {
                vf.add(fog_weight(*tau, front, back), v);
            }
            *tau += front + back;
        }
    };
    for i in 0..n {
        let (t0, t1) = segment(i, n, march.reach);
        let t = sample_at(i, n, march.jitter, march.reach);
        let v = vis(t).clamp(0.0, 1.0);
        let (w, seg) = atmo_segment(ta, atmo(t), (t1 - t0) * march.atmo_scale);
        va.add(w, v);
        ta *= seg;
        fog_segment(t0, t1, v, &mut tau, &mut vf);
    }
    // The lit remainder: one closed-form segment, the medium at its midpoint.
    if march.end > march.reach {
        let (w, _) = atmo_segment(ta, atmo(f32::midpoint(march.reach, march.end)), (march.end - march.reach) * march.atmo_scale);
        va.add(w, 1.0);
        fog_segment(march.reach, march.end, 1.0, &mut tau, &mut vf);
    }
    (va.value(), vf.value())
}

/// Clips `prev` to the box `lo..hi` along the line to its centre (Karis 2014): the two-channel
/// form of [`crate::froxel::clip_history`]. CPU twin of `sun_vis_resolve.wesl`'s `clip_history`.
#[must_use]
pub fn clip_history(prev: [f32; 2], lo: [f32; 2], hi: [f32; 2]) -> [f32; 2] {
    let c = crate::froxel::clip_history([prev[0], prev[1], 0.0, 0.0], [lo[0], lo[1], 0.0, 0.0], [hi[0], hi[1], 0.0, 0.0]);
    [c[0], c[1]]
}

/// This frame's `current` blended with the reprojected history `prev` (`None`: off screen, the
/// first frame or a reset), clipped to this frame's 3×3 box `lo..hi`, history weight `history`.
#[must_use]
pub fn resolve(current: [f32; 2], prev: Option<[f32; 2]>, lo: [f32; 2], hi: [f32; 2], history: f32) -> [f32; 2] {
    let Some(prev) = prev else { return current };
    let p = clip_history(prev, lo, hi);
    let w = history.clamp(0.0, 1.0);
    [current[0] + (p[0] - current[0]) * w, current[1] + (p[1] - current[1]) * w]
}

#[cfg(test)]
#[allow(clippy::float_cmp, reason = "exact assertions on exact cases")]
mod tests {
    use super::*;

    const AIR: Medium = Medium { scattering: 1.2e-5, absorption: 0.3e-5 };

    fn base() -> March {
        March { samples: 16, reach: 5000.0, end: 5000.0, fog_start: 0.0, jitter: 0.37, atmo_scale: 1.0 }
    }

    /// A uniform fog of extinction `k` per meter: τ over `[t0, t1]`.
    fn uniform_fog(k: f32) -> impl Fn(f32, f32) -> f32 {
        move |t0, t1| k * (t1 - t0).max(0.0)
    }

    #[test]
    fn segments_tile_the_reach_denser_near_the_camera() {
        let (n, reach) = (16, 5000.0);
        assert_eq!(segment(0, n, reach).0, 0.0);
        assert_eq!(segment(n - 1, n, reach).1, reach);
        let mut last_len = 0.0;
        for i in 0..n {
            let (t0, t1) = segment(i, n, reach);
            if i > 0 {
                assert_eq!(t0, segment(i - 1, n, reach).1, "contiguous");
            }
            let len = t1 - t0;
            assert!(len > last_len, "segment {i} longer than the one before");
            last_len = len;
            for j in [0.0, 0.5, 0.999] {
                let t = sample_at(i, n, j, reach);
                assert!(t >= t0 && t <= t1, "sample {t} inside segment {i} [{t0}, {t1}]");
            }
        }
        // Quadratic: the first segment is reach/n², the last (2n − 1)·reach/n².
        assert_eq!(segment(0, 4, 16.0), (0.0, 1.0));
        assert_eq!(segment(3, 4, 16.0), (9.0, 16.0));
    }

    #[test]
    fn jitter_stays_in_the_unit_interval_and_moves_each_frame() {
        let mut seen = Vec::new();
        for frame in 0..8 {
            let j = jitter([17, 5], frame);
            assert!((0.0..1.0).contains(&j), "{j}");
            seen.push(j);
        }
        seen.dedup();
        assert_eq!(seen.len(), 8, "a new offset every frame");
        assert_ne!(jitter([0, 0], 3), jitter([1, 0], 3), "neighbours differ");
        assert_eq!(jitter([3, 4], 7), jitter([3, 4], 7 + FRAME_PERIOD), "frame period");
    }

    #[test]
    fn atmosphere_weights_are_the_closed_form() {
        // A uniform medium: the segments' weights add up to (σs/σt)(1 − e^(−σt·L)) for any split.
        let (s, e) = (AIR.scattering, AIR.scattering + AIR.absorption);
        let (mut t, mut w) = (1.0, 0.0);
        for i in 0..16 {
            let (t0, t1) = segment(i, 16, 5000.0);
            let (dw, dt) = atmo_segment(t, AIR, t1 - t0);
            w += dw;
            t *= dt;
        }
        let exact = s / e * (1.0 - (-e * 5000.0_f32).exp());
        assert!((w - exact).abs() < 1e-6 * exact.max(1e-6) + 1e-9, "{w} vs {exact}");
        assert!((t - (-e * 5000.0_f32).exp()).abs() < 1e-6);
        // No extinction: the linear limit, transmittance 1.
        assert_eq!(atmo_segment(0.5, Medium { scattering: 0.0, absorption: 0.0 }, 100.0), (0.0, 1.0));
        // Fog: e^(−τ_before − τ_front)(1 − e^(−τ_back)).
        assert!((fog_weight(0.5, 0.25, 1.0) - (-0.75_f32).exp() * (1.0 - (-1.0_f32).exp())).abs() < 1e-7);
        assert_eq!(fog_weight(0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn no_occluder_is_lit() {
        let fog = uniform_fog(1e-3);
        assert_eq!(march(&base(), |_| 1.0, |_| AIR, Some(&fog)), (1.0, 1.0));
    }

    #[test]
    fn every_sample_shadowed_is_dark() {
        let fog = uniform_fog(1e-3);
        assert_eq!(march(&base(), |_| 0.0, |_| AIR, Some(&fog)), (0.0, 0.0));
    }

    #[test]
    fn no_fog_and_no_medium_are_lit() {
        let (va, vf) = march(&base(), |_| 0.0, |_| AIR, None);
        assert_eq!((va, vf), (0.0, 1.0), "no fog: V_fog = 1 whatever the shadow");
        let zero = uniform_fog(0.0);
        assert_eq!(march(&base(), |_| 0.0, |_| Medium::default(), Some(&zero)), (1.0, 1.0), "no weight anywhere: lit");
    }

    #[test]
    fn the_lit_remainder_past_the_reach_counts() {
        // Shadowed inside the reach, a uniform medium out to `end`: V = the remainder's share of the
        // closed-form weight, e^(−σR)(1 − e^(−σ(E−R))) / (1 − e^(−σE)).
        let m = March { end: 20_000.0, ..base() };
        let e = AIR.scattering + AIR.absorption;
        let (r, end) = (m.reach, m.end);
        let exact = (-e * r).exp() * (1.0 - (-e * (end - r)).exp()) / (1.0 - (-e * end).exp());
        let k = 2e-4;
        let fog = uniform_fog(k);
        let fog_exact = (-k * r).exp() * (1.0 - (-k * (end - r)).exp()) / (1.0 - (-k * end).exp());
        let (va, vf) = march(&m, |_| 0.0, |_| AIR, Some(&fog));
        assert!((va - exact).abs() < 1e-5, "{va} vs {exact}");
        assert!((vf - fog_exact).abs() < 1e-5, "{vf} vs {fog_exact}");
        assert!(va > 0.0 && va < 1.0);
    }

    #[test]
    fn fog_weights_start_at_the_froxels_far() {
        // Shadowed only before the froxels' far (on a segment boundary, 5000·(4/16)² = 312.5 m):
        // the analytic fog behind it is all lit.
        let m = March { fog_start: 312.5, ..base() };
        let fog = uniform_fog(1e-3);
        let (va, vf) = march(&m, |t| if t < 312.5 { 0.0 } else { 1.0 }, |_| AIR, Some(&fog));
        assert_eq!(vf, 1.0);
        assert!(va < 1.0, "the atmosphere counts the near shadow: {va}");
        // A fog start past the end: no fog weight, lit.
        let far = March { fog_start: 1e6, ..base() };
        assert_eq!(march(&far, |_| 0.0, |_| AIR, Some(&fog)).1, 1.0);
    }

    #[test]
    fn half_shadowed_is_the_weighted_share() {
        // No medium extinction to speak of (σ·L ≪ 1): the atmosphere's weights follow segment length,
        // so a shadow over the first 1/4 of the ray (by distance) leaves about 3/4 lit.
        let thin = Medium { scattering: 1e-9, absorption: 0.0 };
        let m = March { samples: 64, jitter: 0.5, ..base() };
        let (va, _) = march(&m, |t| if t < 1250.0 { 0.0 } else { 1.0 }, |_| thin, None);
        assert!((va - 0.75).abs() < 0.01, "{va}");
    }

    #[test]
    fn history_clip_and_resolve() {
        let (lo, hi) = ([0.2, 0.4], [0.6, 0.8]);
        assert_eq!(clip_history([0.3, 0.5], lo, hi), [0.3, 0.5], "inside: kept");
        let c = clip_history([1.0, 0.6], lo, hi);
        assert!((c[0] - 0.6).abs() < 1e-5 && (c[1] - 0.6).abs() < 1e-5, "outside: pulled to the box's surface {c:?}");
        assert_eq!(resolve([0.5, 0.5], None, lo, hi, 0.9), [0.5, 0.5], "off screen: this frame");
        let r = resolve([0.2, 0.4], Some([0.6, 0.8]), lo, hi, 0.9);
        assert!((r[0] - 0.56).abs() < 1e-5 && (r[1] - 0.76).abs() < 1e-5, "{r:?}");
        assert_eq!(resolve([0.2, 0.4], Some([0.6, 0.8]), lo, hi, 0.0), [0.2, 0.4], "history 0");
    }
}
