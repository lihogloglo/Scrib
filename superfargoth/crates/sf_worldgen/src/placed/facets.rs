//! Eroded fault-scarp morphology (LF-2b, `landforms.md` "Operator catalog" `fault_scarp`): along-strike throw
//! variation, the face dissected into triangular facets between ravines (chutes), and a debris cone at every chute
//! mouth in place of the uniform talus apron.
//!
//! Geology (V = opened, S = secondary): range fronts of normal-fault blocks are a row of **triangular (spur)
//! facets** between ravines; the ravines cut back into the footwall (headward), so they widen upward and the facet
//! between two of them narrows to an apex (V: Wallace 1978, J. Res. USGS 6, "Geometry and rates of change of
//! fault-generated range fronts, north-central Nevada": spur facets 25-35 deg, basin headwalls 20-30 deg, fresh
//! scarps 50-90 deg degrading to 30-37 deg). Each ravine mouth builds a debris cone / alluvial fan on the basin floor;
//! neighbouring fans merge into a bajada (S: `Blair & McPherson 1994`, alluvial fans 1.5-25 deg, debris-flow fans the
//! steep end). Outlet spacing scales with the range width (S: Hovius 1996); the small facet ravines here follow a
//! truncated power law of spacing. Throw varies along strike and between segments, tapering at the tips (S:
//! Cowie & Scholz 1992; Peacock & Sanderson 1991). We keep faces steeper than Wallace's degraded facets (45-75 deg by
//! hardness, owner 2026-11-22: "dramatic"), the cliff mask then marks only what stays above `cliff.slope_deg`.
//!
//! **Surface of one segment** at arc length `s`, signed distance `d` (+ upthrown), local throw `H = throw_at(s)`:
//!
//! - block `B(d)`: the LF-2 tilted-block profile ([`super::scarp::Profile`]) with a thin facet talus;
//! - chute `k` at `s_k` (mouth height `z_a = apex_k H` on the face at `d_m = -(H - z_a) / tan face`): floor `F(d) =
//!   z_a + (d - d_m)(1 - depth_k) tan face` (so the chute is `depth_k (H - z_a)` deep at the lip and climbs back to the
//!   top surface behind the lip, the headward retreat), V walls `F(d) + |s - s_k| tan wall`, only above the mouth
//!   (`d >= d_m`; at `d_m` the V is never below the face, so the surface is continuous);
//! - cone `k`: `z_a - |(s, d) - (s_k, d_m)| tan cone`;
//! - `z = max(min(B, chutes), cones)`. Chute size = the mean of its two spacings: half-width at the lip `width_share x
//!   size / 2` sets `depth_k`, larger chutes get higher cone apexes (`fan_share x sqrt(size / chute_max_m)`).
//!
//! The delta is box-filtered over `2r x 2r` in `(s, d)` by a `taps x taps` midpoint rule (the min / max has no closed
//! form; the plain LF-2 profile keeps its exact antiderivative).

use super::scarp::{Segment, Shape, Zone};
use super::{power_law_length, u01};
use crate::filter::pcg3d;

/// Along-strike throw modulation: `1 + amp n(s)`, `n` two octaves of smooth value noise in [-1, 1].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Wobble {
    /// 0 = off (the factor is exactly 1).
    pub amp: f64,
    pub wavelength_m: f64,
    pub seed: u32,
}

impl Wobble {
    /// Throw factor at arc length `s`, >= 0.
    pub fn factor(&self, s: f64) -> f64 {
        if self.amp <= 0.0 || self.wavelength_m <= 0.0 {
            return 1.0;
        }
        let n = value_noise_1d(s / self.wavelength_m, self.seed) * (2.0 / 3.0)
            + value_noise_1d(2.0 * s / self.wavelength_m, self.seed ^ 0x6d2b_79f5) * (1.0 / 3.0);
        (1.0 + self.amp * n).max(0.0)
    }
}

/// Smooth (smoothstep-interpolated) value noise in [-1, 1] at `x` lattice units.
fn value_noise_1d(x: f64, seed: u32) -> f64 {
    let i = x.floor();
    let t = x - i;
    let k = i as i64;
    let v = |j: i64| -> f64 {
        let h = pcg3d(seed, (j as u64 & 0xffff_ffff) as u32, (j >> 32) as u32);
        2.0 * u01(h[0]) - 1.0
    };
    let e = t * t * (3.0 - 2.0 * t);
    v(k) + (v(k + 1) - v(k)) * e
}

/// One ravine of a dissected face.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chute {
    /// Arc length of its axis along the segment, metres.
    pub s: f64,
    /// Mouth (debris-cone apex) height, share of the local throw.
    pub apex: f64,
    /// Depth at the lip, share of `throw - apex height`.
    pub depth: f64,
}

/// Chute values (declared in [`super::declared_defaults`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChuteParams {
    pub min_m: f64,
    pub max_m: f64,
    pub b: f64,
    pub width_share: f64,
    pub depth_max: f64,
    pub fan_share: f64,
    pub facet_talus: f64,
}

/// Chutes along a segment of length `len` and nominal throw `h_max` (sorted by `s`): spacings from the truncated
/// power law on `[min_m, max_m]`, a chute between two spacings sized by their mean.
pub fn chutes(len: f64, h_max: f64, tan_wall: f64, cp: &ChuteParams, seed: u32) -> Vec<Chute> {
    if len <= 0.0 || h_max <= 0.0 || cp.max_m <= 0.0 {
        return Vec::new();
    }
    let mut gaps = Vec::new();
    let mut acc = 0.0;
    let mut k = 0u32;
    while acc < len {
        let h = pcg3d(seed ^ 0x3f1a_c0de, k, 7);
        let g = power_law_length(u01(h[0]), cp.min_m, cp.max_m, cp.b);
        gaps.push(g);
        acc += g;
        k += 1;
    }
    let mut out = Vec::with_capacity(gaps.len());
    let mut s = 0.0;
    for i in 0..gaps.len().saturating_sub(1) {
        s += gaps[i];
        if s >= len {
            break;
        }
        let size = f64::midpoint(gaps[i], gaps[i + 1]);
        let apex = (cp.fan_share * (size / cp.max_m).sqrt()).clamp(cp.facet_talus + 0.02, 0.9);
        let depth = (tan_wall * cp.width_share * 0.5 * size / ((1.0 - apex) * h_max)).min(cp.depth_max);
        out.push(Chute { s, apex, depth });
    }
    out
}

/// Along-strike reach of a segment's chutes and cones, metres (for the chute lookup window).
pub fn chute_reach(seg: &Segment, shape: &Shape) -> f64 {
    let h = seg.max_throw();
    seg.chutes
        .iter()
        .map(|c| (c.depth * (1.0 - c.apex) * h / shape.tan_wall).max(c.apex * h / shape.tan_cone))
        .fold(0.0, f64::max)
}

/// Unfiltered height of an eroded segment at `(s, d)` and its zone.
pub fn point(seg: &Segment, shape: &Shape, s: f64, d: f64) -> (f64, Zone) {
    let h = seg.throw_at(s);
    if h <= 0.0 {
        return (0.0, Zone::Plain);
    }
    let pr = seg.profile(shape, h);
    let block = pr.z(d);
    let tf = pr.tan_face;
    let mut carved = block;
    let mut chute = false;
    let mut cone = f64::NEG_INFINITY;
    let lo = seg.chutes.partition_point(|c| c.s < s - seg.chute_reach);
    for c in &seg.chutes[lo..] {
        if c.s > s + seg.chute_reach {
            break;
        }
        // The mouth sits on the face, never below the facet talus top (so the V meets the face at `d_m`).
        let za = c.apex.max(pr.fan) * h;
        let dm = -(h - za) / tf;
        let ds = (s - c.s).abs();
        if d >= dm {
            let v = za + (d - dm) * (1.0 - c.depth) * tf + ds * shape.tan_wall;
            if v < carved {
                carved = v;
                chute = true;
            }
        }
        cone = cone.max(za - ds.hypot(d - dm) * shape.tan_cone);
    }
    if cone > carved {
        (cone, Zone::Talus)
    } else if chute {
        (carved, Zone::Face)
    } else {
        (carved, pr.zone(d))
    }
}

/// [`point`] box-filtered over `[s - r, s + r] x [d - r, d + r]` (`taps^2` midpoints; `r = 0` or one tap: the point).
pub fn filtered(seg: &Segment, shape: &Shape, s: f64, d: f64) -> (f64, Zone) {
    let (z0, zone) = point(seg, shape, s, d);
    let (r, n) = (shape.filter_r, shape.taps);
    if r <= 0.0 || n <= 1 {
        return (z0, zone);
    }
    let nf = n as f64;
    let mut acc = 0.0;
    for i in 0..n {
        let os = ((i as f64 + 0.5) / nf * 2.0 - 1.0) * r;
        for j in 0..n {
            let od = ((j as f64 + 0.5) / nf * 2.0 - 1.0) * r;
            acc += point(seg, shape, s + os, d + od).0;
        }
    }
    (acc / (nf * nf), zone)
}
