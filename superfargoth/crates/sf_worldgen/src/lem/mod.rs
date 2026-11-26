//! **Landscape evolution model (LEM) ported from Veloren** (WG-S5): implicit stream power over height-ordered nodes
//! with multiple receivers, closed basins (lakes) resolved and drained over their spill, nonlinear hillslope
//! transport, per-cell erodibility and uplift.
//!
//! **Provenance.** Ported from Veloren, `world/src/sim/erosion.rs` (gitlab.com/veloren/veloren, GPL-3.0-or-later),
//! read at master commit `6c99b292e0c3cbb5ab9c76e2c2f83a3ce73b295c` (2026-11-10; the file was last changed in
//! `07840f06fa49df612eccb9258099393bdd900eac`, 2025-09-10). Veloren's solver is itself a Rust port of `FastScape`
//! (fastscapelib-fortran, GPL-3.0: `StreamPowerLaw.f90`, `find_mult_rec`) and of Cordonnier, Bovy, Braun, "A
//! versatile, linear complexity algorithm for flow routing in topographies with depressions", Earth Surf. Dynam. 7,
//! 2019. Notice: `superfargoth/THIRD_PARTY_NOTICES.md` "Veloren (erosion)".
//!
//! Kept (Veloren `erode`, `get_lakes`, `get_multi_rec`, `get_multi_drainage`):
//!
//! - per step: route the current surface ([`route`]: steepest receivers, basins, lowest-pass tree, fill
//!   re-routing of flooded nodes, a water surface 1 ulp above each receiver over lakes, multiple receivers on that
//!   surface with weights `slopeᵖ`, `p = base + coef × mean slope`, a topological multi stack, drainage area);
//! - the implicit stream-power update swept bottom-up along the multi stack (Braun & Willett 2013, multi-receiver
//!   form of fastscapelib-fortran): `hᵢ = (hᵢ + Uᵢ + Σₖ Fₖ hₖ) / (1 + Σₖ Fₖ)`, `Fₖ = K dt (A wₖ)ᵐ / lₖⁿ`, over the
//!   receivers below the uplifted height; for n ≠ 1 the local Newton iteration (ω = 0.875 / n, tolerance 1 mm);
//! - lakes: a node at or below its receiver's water level is under a lake and only takes uplift (no incision); a
//!   node that would incise below its receiver's water level stops at that level (Veloren's "deposit" case); the
//!   water level then propagates upward as `max(receiver level, own height)`;
//! - hillslopes: Veloren's tangent diffusivity of the Roering, Kirchner, Dietrich (WRR 35(3), 1999) nonlinear flux
//!   `q = D S / (1 − (S/Sc)²)`: `D_eff = D (1 + r(3 − r)/(1 − r)²)`, `r = (S/Sc)²`, on the slope to the receiver,
//!   with the boost capped (Veloren caps it at its explicit stability limit; ours is a recipe ratio of D because the
//!   solve is implicit);
//! - per-cell erodibility (K dt) and uplift arrays, as Veloren's `kf(posi)` / `uplift(posi)` closures.
//!
//! Changed:
//!
//! - **Determinism:** no parallel reductions; basins, the tree, the stacks, the area sums and the sweep run in a
//!   fixed order on one thread; only pure per-node maps run on `rayon`. Bitwise equal for any thread count.
//! - **Sink handling:** both Veloren and fastscapelib (`mst_sink_resolver`, `basin_graph.hpp`, commit `b85cc6b1`)
//!   implement Cordonnier et al. 2019. fastscapelib offers the `basic` (pit → pass, non-planar) and `carve`
//!   (reversed path, a canyon cut from pit to pass) routes; Veloren adds the paper's **fill** route (flooded nodes
//!   drain across the lake toward the outflow), which keeps lakes as lakes until they fill or are breached. We use
//!   Veloren's (fill), with fastscapelib's 1-ulp sloped water surface; details in [`route`].
//! - **Boundary conditions:** sea (below sea level) and the grid edge are base level and never move (Veloren's
//!   ocean `wh = max(0, h)`, at our sea level, so land never incises below it); **anchors** are optional fixed-height
//!   cells (worldgen.md stage 2: boundary conditions), routed like land but never uplifted, eroded or diffused.
//! - **Hillslope solve:** Veloren's ADI (Peaceman-Rachford, from fastscapelib-fortran `Diffusion.f90`) has an
//!   explicit half-step that is not monotone at our `D dt / Δx²` (10²-10³ per step); the diffusion is the
//!   monotone line-split backward Euler of [`crate::landscape`], with sea Dirichlet at sea level and fixed cells
//!   Dirichlet at their height. Veloren's boost is zero at and above Sc (a jump); here it holds the cap there,
//!   and the threshold-slope clamp of WG-S4 (`h ≤ h_r + tan θc · l`, Montgomery & Brandon 2002) stays in the sweep.
//! - Not ported: sediment transport and deposition (`G`, `kdsed`), soil production and the bedrock layer, debris-flow
//!   terms, rock-strength noise (erodibility comes from [`crate::lithology`] instead).

// Grid loops index several parallel arrays by node and neighbour slot; iterators would hide that.
#![allow(clippy::needless_range_loop)]

pub mod route;

pub use route::{BOUNDARY, MfdLaw, Routing, route};

use crate::grid::NEIGHBOUR_DIST;
use crate::landscape::{LandscapeStats, diffuse_lines};
use crate::stream::{RESIDUAL_TRACE_EVERY, max_and_p99};
use crate::{Error, Grid, Result};

/// When a run stops (besides its step cap).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stop {
    /// Steady state: p99 `|Δh|` per step over land / mean uplift per step below `rel`.
    Steady { rel: f64 },
    /// Finite time: the p95 land height above sea reaches `target_m`.
    LandP95 { target_m: f64 },
    /// Run the step cap.
    Steps,
}

/// Per-cell inputs of one LEM run (all slices have the grid's length).
pub struct LemParams<'a> {
    /// Drainage-area exponent m.
    pub m: f64,
    /// Slope exponent n (1 = linear implicit update; otherwise local Newton).
    pub n: f64,
    /// `K dt` per step.
    pub k_dt: &'a [f64],
    /// Uplift per step, metres (ignored on sea, edge and fixed cells).
    pub uplift_dt: &'a [f64],
    /// Base `D dt` per step, m².
    pub diffusion_dt: &'a [f64],
    /// Critical slope `tan θc` per cell.
    pub tan_talus: &'a [f64],
    /// Optional fixed-height cells (anchors as boundary conditions).
    pub fixed: Option<&'a [bool]>,
    pub mfd: MfdLaw,
    /// Cap of the nonlinear diffusivity boost, as a multiple of the base D.
    pub nonlinear_max_ratio: f64,
    pub sea_level: f64,
    pub max_iterations: usize,
    pub stop: Stop,
}

// tunables: const (numerics of the local Newton solve: Veloren / fastscapelib-fortran tolerance and relaxation)
const NEWTON_TOL_M: f64 = 1.0e-3;
const NEWTON_OMEGA: f64 = 0.875;
const NEWTON_MAX_ITER: usize = 100; // tunables: const (numerics of the local Newton solve: a divergence guard, it stops on NEWTON_TOL_M)

/// p95 of land height above sea (`land` cells only).
pub fn land_p95(h: &[f64], land: &[bool], sea: f64, buf: &mut Vec<f64>) -> f64 {
    buf.clear();
    buf.extend(h.iter().zip(land).filter(|(_, l)| **l).map(|(v, _)| v - sea));
    if buf.is_empty() {
        return 0.0;
    }
    let k = ((buf.len() - 1) as f64 * 0.95).round() as usize;
    *buf.select_nth_unstable_by(k, f64::total_cmp).1
}

/// Runs the LEM on `h` until the stop rule or the cap. Sea (`h < sea_level` on entry), grid-edge and fixed cells
/// never change.
#[allow(clippy::too_many_lines)]
pub fn evolve(h: &mut Grid, p: &LemParams) -> Result<LandscapeStats> {
    let n = h.len();
    let (w, hh) = (h.w, h.h);
    if [p.k_dt.len(), p.uplift_dt.len(), p.diffusion_dt.len(), p.tan_talus.len()]
        .iter()
        .chain(p.fixed.map(<[bool]>::len).as_ref())
        .any(|&l| l != n)
    {
        return Err(Error::Invalid("lem fields differ in size".into()));
    }
    let sea = p.sea_level;
    let wet: Vec<bool> = h.data.iter().map(|&v| v < sea).collect();
    let boundary: Vec<bool> = (0..n).map(|i| wet[i] || h.is_border(i)).collect();
    let fixed = |i: usize| p.fixed.is_some_and(|f| f[i]);
    // Diffusion line kinds: 0 free, 1 sea (Dirichlet at sea level), 2 fixed at own height.
    let kind: Vec<u8> = (0..n)
        .map(|i| {
            if wet[i] {
                1
            } else if boundary[i] || fixed(i) {
                2
            } else {
                0
            }
        })
        .collect();
    let free: Vec<bool> = kind.iter().map(|&k| k == 0).collect();
    let land: Vec<bool> = wet.iter().map(|w| !w).collect();
    let free_count = free.iter().filter(|f| **f).count();
    let mut stats = LandscapeStats::default();
    let mut buf = Vec::new();
    if free_count == 0 {
        return Ok(stats);
    }
    stats.mean_uplift_dt_m = (0..n).filter(|&i| free[i]).map(|i| p.uplift_dt[i]).sum::<f64>()
        / free_count as f64;
    let dx2 = h.spacing * h.spacing;
    let d_max = (0..n)
        .filter(|&i| free[i])
        .map(|i| p.diffusion_dt[i] * (1.0 + p.nonlinear_max_ratio))
        .fold(0.0, f64::max);
    stats.diffusion_ratio_max = d_max / dx2;
    let linear = (p.n - 1.0).abs() < 1e-12;
    let sqrt_m = p.m.to_bits() == 0.5f64.to_bits();
    let mut prev = h.data.clone();
    let mut wl = vec![0.0; n];
    let mut d_eff = vec![0.0; n];
    let mut dh = Vec::with_capacity(free_count);
    let spacing = h.spacing;
    let dist = move |i: usize, j: usize| {
        let diag = (i % w != j % w) && (i / w != j / w);
        (if diag { std::f64::consts::SQRT_2 } else { 1.0 }) * spacing
    };
    for _ in 0..p.max_iterations {
        prev.copy_from_slice(&h.data);
        let r = route(w, hh, h.spacing, &h.data, &boundary, p.mfd)?;
        stats.lakes_max = stats.lakes_max.max(r.lakes);
        stats.lake_nodes_max = stats.lake_nodes_max.max(r.lake_nodes);
        // Implicit stream power, bottom-up along the multi stack.
        for &i in r.mstack.iter().rev() {
            let i = i as usize;
            if boundary[i] {
                wl[i] = h.data[i].max(sea);
                continue;
            }
            let j = r.receiver[i] as usize;
            let wj = wl[j];
            if fixed(i) {
                wl[i] = wj.max(h.data[i]);
                continue;
            }
            let elev = h.data[i] + p.uplift_dt[i];
            let new = if elev > wj {
                let a = r.area[i];
                let bits = r.mrec[i];
                let mut facts = [(0.0f64, 0.0f64); 8];
                let mut cnt = 0;
                for k in 0..8 {
                    if bits >> k & 1 == 0 {
                        continue;
                    }
                    let Some(jk) = crate::grid::neighbour(w, hh, i, k) else {
                        continue;
                    };
                    let hk = if wet[jk] { sea } else { h.data[jk] };
                    if elev > hk {
                        let aw = a * f64::from(r.mwrec[i][k]);
                        let am = if sqrt_m { aw.sqrt() } else { aw.powf(p.m) };
                        let l = NEIGHBOUR_DIST[k] * h.spacing;
                        let ln = if linear { l } else { l.powf(p.n) };
                        facts[cnt] = (p.k_dt[i] * am / ln, hk);
                        cnt += 1;
                    }
                }
                let mut v = if linear {
                    let (mut f, mut df) = (elev, 1.0);
                    for &(fk, hk) in &facts[..cnt] {
                        f += fk * hk;
                        df += fk;
                    }
                    f / df
                } else {
                    let omega = NEWTON_OMEGA / p.n;
                    let mut v = elev;
                    for _ in 0..NEWTON_MAX_ITER {
                        let (mut f, mut df) = (v - elev, 1.0);
                        for &(fk, hk) in &facts[..cnt] {
                            let d = v - hk;
                            if d > 0.0 {
                                f += fk * d.powf(p.n);
                                df += p.n * fk * d.powf(p.n - 1.0);
                            }
                        }
                        let hn = v - f / df;
                        let err = (hn - v).abs();
                        v = v * (1.0 - omega) + hn * omega;
                        if err <= NEWTON_TOL_M {
                            break;
                        }
                    }
                    v
                };
                v = v.min(wj + p.tan_talus[i] * dist(i, j));
                if v <= wj { wj } else { v }
            } else {
                elev
            };
            h.data[i] = new;
            wl[i] = wj.max(new);
        }
        // Nonlinear hillslope diffusivity on the slope to the receiver, then the implicit line solves.
        for i in 0..n {
            let d = p.diffusion_dt[i];
            d_eff[i] = if free[i] {
                let j = r.receiver[i] as usize;
                let s = (h.data[i] - h.data[j]).max(0.0) / dist(i, j);
                let sc = p.tan_talus[i];
                let cap = p.nonlinear_max_ratio * d;
                let ratio = if sc > 0.0 { s / sc } else { f64::INFINITY };
                let boost = if ratio >= 1.0 {
                    cap
                } else {
                    let r2 = ratio * ratio;
                    (d * r2 * (3.0 - r2) / ((1.0 - r2) * (1.0 - r2))).min(cap)
                };
                d + boost
            } else {
                d
            };
        }
        if d_max > 0.0 {
            diffuse_lines(h, &kind, &d_eff, sea);
        }
        stats.iterations += 1;
        dh.clear();
        dh.extend((0..n).filter(|&i| free[i]).map(|i| (h.data[i] - prev[i]).abs()));
        (stats.residual_max_m, stats.residual_p99_m) = max_and_p99(&mut dh);
        stats.residual_rel = if stats.mean_uplift_dt_m > 0.0 {
            stats.residual_p99_m / stats.mean_uplift_dt_m
        } else {
            0.0
        };
        if stats.iterations % RESIDUAL_TRACE_EVERY == 0 {
            stats.residual_trace.push((stats.iterations, stats.residual_rel));
        }
        let done = match p.stop {
            Stop::Steady { rel } => stats.residual_rel < rel,
            Stop::LandP95 { target_m } => {
                stats.land_p95_m = land_p95(&h.data, &land, sea, &mut buf);
                stats.land_p95_m >= target_m
            }
            Stop::Steps => false,
        };
        if done {
            stats.converged = true;
            break;
        }
    }
    stats.land_p95_m = land_p95(&h.data, &land, sea, &mut buf);
    if stats.residual_trace.last().is_none_or(|t| t.0 != stats.iterations) {
        stats.residual_trace.push((stats.iterations, stats.residual_rel));
    }
    Ok(stats)
}
