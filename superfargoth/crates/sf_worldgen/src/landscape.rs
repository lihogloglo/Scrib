//! **Uplift-driven landscape to steady state** (WG-S4 macro form): the relief emerges from uplift against fluvial
//! incision and hillslope transport, from a low start, instead of eroding a finished field.
//!
//! Model, per land sample (n = 1):
//!
//! `∂h/∂t = U − K Aᵐ S + ∇·(D ∇h)`, with hillslopes bounded by a critical slope `tan θc`.
//!
//! - **Uplift-driven form:** Cordonnier, Braun, Cani, Benes, Galin, Peytavie, Guérin, "Large Scale Terrain
//!   Generation from Tectonic Uplift and Fluvial Erosion", CGF 35(2), 2016: the user paints an uplift map, the
//!   landscape is the steady state of uplift against stream power, started from a low surface. Here the uplift
//!   map comes from the layout ([`crate::layout`]) and the runner ([`crate::landform`]) calibrates it per region.
//! - **Implicit stream power:** Braun & Willett, Geomorphology 180-181, 2013 (O(n), unconditionally stable for
//!   n = 1): uplift is added, then `hᵢ ← (hᵢ + F hᵣ) / (1 + F)`, `F = K dt Aᵐ / Δx`, swept in stack order along the
//!   steepest receiver of the Priority-Flood+ε surface ([`crate::flood`]). Drainage area by multiple flow
//!   direction ([`crate::mfd`], Freeman 1991 slope-power weights), incision along the steepest receiver, as
//!   Schott et al. 2023 (code `H-Schott/StreamPowerErosion`). Large time steps are allowed: the steady state does
//!   not depend on dt, only the number of steps to reach it.
//! - **Hillslope diffusion:** linear diffusion `∇·(D∇h)` as in `FastScape` (Braun & Willett 2013, §2; they solve it
//!   implicitly by ADI) and Schott et al. 2023 (their stream power + hillslope model). Here: **implicit,
//!   line-split** (locally one-dimensional backward Euler: `(I − dt Lx)` along rows, then `(I − dt Ly)` along
//!   columns; Yanenko, "The Method of Fractional Steps", 1971), each line a tridiagonal Thomas solve,
//!   conservative fluxes with face-averaged D, sea samples Dirichlet at sea level, Neumann at the grid edge.
//!   Unconditionally stable and monotone (each line matrix is an M-matrix, so a line's solution stays between its
//!   right-hand side and the boundary values), so the large uplift steps that bring the landscape to steady state
//!   quickly need no substeps (an explicit scheme is bounded by `D dt / Δx² ≤ 1/4`, which made WG-S4's first
//!   probe need 30-200 substeps per step and still converge slowly: the diffusive response time is `L²/D`).
//!   At steady state a hillslope of half-width L between channels is a parabola, `h = U (L² − x²) / 2D`: the
//!   runner sets `D = (D/U) · U` per sample so the ridge shape is set by one length, independent of the
//!   calibrated uplift.
//! - **Threshold hillslope:** each sample is held to at most `tan θc` above its receiver (the critical-slope
//!   "threshold hillslope" of Montgomery & Brandon, "Topographic controls on erosion rates in tectonically active
//!   mountain ranges", EPSL 201, 2002; the talus clamp Cordonnier et al. 2016 use for hillslopes, U = unverified
//!   against their text). Applied in the same stack sweep, so the receiver is already final.
//! - **Steady state:** the residual is the p99 of `|Δh|` per step over land, relative to the mean uplift per step;
//!   the run stops once it drops below `steady_rel` (uplift balanced by erosion everywhere but the top 1 %).
//!
//! Sea samples (below sea level on entry) are base level and never move; uplift is only added on land and both the
//! implicit update (a convex mix with a receiver at ≥ sea level) and the diffusion (monotone line solves with
//! sea as a boundary at sea level, clamped at it against rounding) keep land at or above sea level, so no stage
//! creates or removes water.
//!
//! Deterministic: routing, MFD and the sweep are single-threaded in a fixed order; the diffusion line solves are
//! independent per row and per column (parallel lines, each solved sequentially).

use rayon::prelude::*;

use crate::mfd::mfd_area;
use crate::stream::{RESIDUAL_TRACE_EVERY, max_and_p99, route};
use crate::{Error, Grid, Result};


/// Per-sample inputs of one landscape run.
pub struct LandscapeParams<'a> {
    /// Drainage-area exponent m (n = 1).
    pub m: f64,
    /// `K dt` per sample per step, m^(1-2m).
    pub k_dt: &'a [f64],
    /// Uplift per step, metres, per sample (ignored on sea).
    pub uplift_dt: &'a [f64],
    /// `D dt` per step, m², per sample.
    pub diffusion_dt: &'a [f64],
    /// Critical slope `tan θc` per sample (∞ disables the clamp).
    pub tan_talus: &'a [f64],
    /// MFD slope exponent p.
    pub flow_exponent: f64,
    pub sea_level: f64,
    /// Step cap.
    pub max_iterations: usize,
    /// Steady state: p99 `|Δh|` per step over land / mean uplift per step below this.
    pub steady_rel: f64,
}

/// Numbers of one landscape run.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct LandscapeStats {
    pub iterations: usize,
    pub converged: bool,
    /// Last step: p99 and max `|Δh|` over land, metres per step.
    pub residual_p99_m: f64,
    pub residual_max_m: f64,
    /// Last step: p99 `|Δh|` / mean uplift per step.
    pub residual_rel: f64,
    /// Mean uplift per step over land, metres.
    pub mean_uplift_dt_m: f64,
    /// Largest `D dt / Δx²` over land (the implicit solve needs no substeps; an explicit one would need 4× this).
    pub diffusion_ratio_max: f64,
    /// (step, relative residual) every [`RESIDUAL_TRACE_EVERY`] steps and at the last one.
    pub residual_trace: Vec<(usize, f64)>,
    /// Largest MFD stuck-sender and pits-after-routing counts over all steps (0 expected).
    pub mfd_stuck: usize,
    pub pits_after_routing: usize,
    /// LEM runs ([`crate::lem`]): most closed basins resolved in one step and most flooded nodes re-routed.
    pub lakes_max: usize,
    pub lake_nodes_max: usize,
    /// LEM runs: p95 of land height above sea at the end, metres.
    pub land_p95_m: f64,
}

/// Runs the landscape on `h` until steady state or the cap. Wet samples (`h < sea_level` on entry) never change.
#[allow(clippy::too_many_lines)]
pub fn evolve(h: &mut Grid, p: &LandscapeParams) -> Result<LandscapeStats> {
    let n = h.len();
    if [
        p.k_dt.len(),
        p.uplift_dt.len(),
        p.diffusion_dt.len(),
        p.tan_talus.len(),
    ]
    .iter()
    .any(|&l| l != n)
    {
        return Err(Error::Invalid("landscape fields differ in size".into()));
    }
    let wet: Vec<bool> = h.data.iter().map(|&v| v < p.sea_level).collect();
    let land_count = wet.iter().filter(|v| !**v).count();
    let mut stats = LandscapeStats::default();
    if land_count == 0 {
        return Ok(stats);
    }
    stats.mean_uplift_dt_m = (0..n)
        .filter(|&i| !wet[i])
        .map(|i| p.uplift_dt[i])
        .sum::<f64>()
        / land_count as f64;
    let dx2 = h.spacing * h.spacing;
    let d_max = (0..n)
        .filter(|&i| !wet[i])
        .map(|i| p.diffusion_dt[i])
        .fold(0.0, f64::max);
    stats.diffusion_ratio_max = d_max / dx2;
    let kind: Vec<u8> = wet.iter().map(|&w| u8::from(w)).collect();
    let mut z = Vec::with_capacity(n);
    let mut prev = h.data.clone();
    let mut dh = Vec::with_capacity(land_count);
    for _ in 0..p.max_iterations {
        prev.copy_from_slice(&h.data);
        for ((v, &u), &wt) in h.data.iter_mut().zip(p.uplift_dt).zip(&wet) {
            if !wt {
                *v += u;
            }
        }
        let mut routing = route(h, &wet, p.sea_level, &mut z);
        stats.pits_after_routing = stats.pits_after_routing.max(routing.pits);
        if routing.unrouted != 0 {
            return Err(Error::Invalid(format!(
                "{} land samples not reached by the stack",
                routing.unrouted
            )));
        }
        let (area, stuck) = mfd_area(h.w, h.h, h.spacing, &z, &wet, p.flow_exponent);
        routing.area = area;
        stats.mfd_stuck = stats.mfd_stuck.max(stuck);
        // Implicit stream power + threshold hillslope, base level upward.
        let w = h.w;
        for &i in &routing.order {
            let i = i as usize;
            let r = routing.receiver[i] as usize;
            if r == i || wet[i] {
                continue;
            }
            let hr = if wet[r] { p.sea_level } else { h.data[r] };
            let hi = h.data[i];
            if hi <= hr {
                continue;
            }
            let diag = (i % w != r % w) && (i / w != r / w);
            let dx = if diag { std::f64::consts::SQRT_2 } else { 1.0 } * h.spacing;
            let a = routing.area[i];
            let am = if p.m.to_bits() == 0.5f64.to_bits() {
                a.sqrt()
            } else {
                a.powf(p.m)
            };
            let f = p.k_dt[i] * am / dx;
            let v = (hi + f * hr) / (1.0 + f);
            h.data[i] = v.min(hr + p.tan_talus[i] * dx);
        }
        if d_max > 0.0 {
            diffuse_lines(h, &kind, p.diffusion_dt, p.sea_level);
        }
        stats.iterations += 1;
        dh.clear();
        dh.extend(
            (0..n)
                .filter(|&i| !wet[i])
                .map(|i| (h.data[i] - prev[i]).abs()),
        );
        (stats.residual_max_m, stats.residual_p99_m) = max_and_p99(&mut dh);
        stats.residual_rel = if stats.mean_uplift_dt_m > 0.0 {
            stats.residual_p99_m / stats.mean_uplift_dt_m
        } else {
            0.0
        };
        if stats.iterations % RESIDUAL_TRACE_EVERY == 0 {
            stats
                .residual_trace
                .push((stats.iterations, stats.residual_rel));
        }
        if stats.residual_rel < p.steady_rel {
            stats.converged = true;
            break;
        }
    }
    if stats
        .residual_trace
        .last()
        .is_none_or(|t| t.0 != stats.iterations)
    {
        stats
            .residual_trace
            .push((stats.iterations, stats.residual_rel));
    }
    Ok(stats)
}

/// Thomas algorithm: solves the tridiagonal system (sub `a`, diagonal `b`, super `c`) in place in `r`.
/// Stable without pivoting for diagonally dominant systems (every land row here).
fn thomas(a: &[f64], b: &mut [f64], c: &[f64], r: &mut [f64]) {
    let n = b.len();
    for i in 1..n {
        let m = a[i] / b[i - 1];
        b[i] -= m * c[i - 1];
        r[i] -= m * r[i - 1];
    }
    r[n - 1] /= b[n - 1];
    for i in (0..n - 1).rev() {
        r[i] = (r[i] - c[i] * r[i + 1]) / b[i];
    }
}

/// Backward-Euler diffusion along one line: `vals` (in/out), each sample's kind (0 free, 1 sea: Dirichlet at
/// `sea`, 2 fixed: Dirichlet at its own value) and `D dt`; `inv_dx2` = 1/Δx². Non-free samples keep their value.
fn solve_line(vals: &mut [f64], kind: &[u8], d: &[f64], sea: f64, inv_dx2: f64) {
    let n = vals.len();
    let (mut a, mut b, mut c, mut r) = (vec![0.0; n], vec![1.0; n], vec![0.0; n], vec![0.0; n]);
    for j in 0..n {
        if kind[j] != 0 {
            r[j] = if kind[j] == 1 { sea } else { vals[j] };
            continue;
        }
        let face = |k: usize| f64::midpoint(d[j], if kind[k] != 0 { d[j] } else { d[k] }) * inv_dx2;
        let kl = if j > 0 { face(j - 1) } else { 0.0 };
        let kr = if j + 1 < n { face(j + 1) } else { 0.0 };
        a[j] = -kl;
        c[j] = -kr;
        b[j] = 1.0 + kl + kr;
        r[j] = vals[j];
    }
    thomas(&a, &mut b, &c, &mut r);
    for j in 0..n {
        if kind[j] == 0 {
            vals[j] = r[j].max(sea);
        }
    }
}

/// One implicit line-split diffusion step of `h` with `D dt` per sample: rows, then columns. `kind` as
/// [`solve_line`] (WG-S4 passes sea only; the LEM adds fixed cells).
pub(crate) fn diffuse_lines(h: &mut Grid, kind: &[u8], diffusion_dt: &[f64], sea: f64) {
    let (w, hh) = (h.w, h.h);
    let inv = 1.0 / (h.spacing * h.spacing);
    h.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let s = y * w..(y + 1) * w;
        solve_line(row, &kind[s.clone()], &diffusion_dt[s], sea, inv);
    });
    let cols: Vec<Vec<f64>> = (0..w)
        .into_par_iter()
        .map(|x| {
            let mut v: Vec<f64> = (0..hh).map(|y| h.data[y * w + x]).collect();
            let kl: Vec<u8> = (0..hh).map(|y| kind[y * w + x]).collect();
            let dl: Vec<f64> = (0..hh).map(|y| diffusion_dt[y * w + x]).collect();
            solve_line(&mut v, &kl, &dl, sea, inv);
            v
        })
        .collect();
    for (x, col) in cols.iter().enumerate() {
        for (y, &v) in col.iter().enumerate() {
            h.data[y * w + x] = v;
        }
    }
}
