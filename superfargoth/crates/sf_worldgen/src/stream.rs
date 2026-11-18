//! Macro fluvial erosion by the **stream power law**, `dh/dt = U - K Aᵐ Sⁿ`, with n = 1.
//!
//! - Solver: Braun, J. & Willett, S. D., "A very efficient O(n), implicit and parallel method to solve the stream
//!   power equation governing fluvial incision and landscape evolution", Geomorphology 180-181, 2013. D8 receivers
//!   (O'Callaghan & Mark 1984), the stack order of the receiver tree, drainage area accumulated in reverse stack
//!   order, then the implicit update `hᵢ ← (hᵢ + F hᵣ) / (1 + F)` with `F = K dt Aᵐ / Δx`, swept from base level
//!   upward so `hᵣ` is already the new value. Unconditionally stable for n = 1.
//! - Depressions: routed over the Priority-Flood+ε surface ([`crate::flood`], Barnes et al. 2014), so every dry
//!   sample drains to base level. Where a sample sits below its receiver (inside a closed depression) it is not
//!   eroded (no deposition model at this stage); the heights are never filled.
//! - Uplift: authored landscapes are driven by an uplift map (Cordonnier, Braun, Cani, Benes, Galin, Peytavie,
//!   Guérin, "Large Scale Terrain Generation from Tectonic Uplift and Fluvial Erosion", CGF 35(2), 2016). Here the
//!   uplift comes from the intent: each step raises a dry sample toward the intent by a fraction `restore` of the
//!   gap (`U = (I - h)⁺ / τ`), so ridges stay near the intent while channels cut below it; at equilibrium the
//!   incision depth is `τ K Aᵐ S`.
//! - Base level: wet samples, held at the sea level (the water surface). They never move, and since every
//!   receiver chain ends at ≥ sea level and the update is a convex mix, dry samples stay dry.
//!
//! - Drainage area ([`Flow`]): D8 (single flow, the original solver) or **multiple-flow-direction** accumulation
//!   ([`crate::mfd`], Freeman 1991 / Quinn et al. 1991 / Holmgren 1994 slope-power weights). With MFD the
//!   implicit update still runs along the steepest (D8) receiver, so it keeps its stack order and stays
//!   unconditionally stable for n = 1; only `A` changes. This is what Schott et al. 2023 do (MFD area, incision
//!   relative to the steepest receiver; code `H-Schott/StreamPowerErosion`). The fully implicit MFD update
//!   (Braun, Cordonnier, Bovy, Yuan, EGU 2018-17723; Anand, Hooshyar, Porporato, "Linear layout of multiple
//!   flow-direction networks for landscape-evolution simulations", Env. Modelling & Software 133, 2020) mixes
//!   all receivers in a topological order; it is the next step if the steepest-receiver update shows grid bias.
//! - Symmetry breaking: an optional perturbation ([`crate::perturb`]) added to the uplift target and the initial
//!   height of dry samples (clamped to stay dry).
//! - Convergence: every step measures the residual `|Δh|` per step over dry samples (max and p99); with
//!   `steady_dh_m` set the run stops once the p99 drops below it (steady state, uplift balancing incision), else
//!   it runs `iterations` steps (the cap either way).
//!
//! Deterministic: routing, the MFD accumulation and the sweeps are single-threaded in a fixed order; only the D8
//! receiver map and the MFD in-degree and raw weights (pure functions of the routing surface) run in parallel.

use rayon::prelude::*;

use crate::flood::priority_flood_eps;
use crate::grid::{NEIGHBOUR_DIST, neighbour};
use crate::mfd::mfd_area;
use crate::{Error, Grid, Result};

/// How drainage area is accumulated.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum Flow {
    /// Single flow to the steepest receiver (O'Callaghan & Mark 1984).
    D8,
    /// Multiple flow to all lower neighbours, weights `tanβᵖ` ([`crate::mfd`]).
    Mfd { exponent: f64 },
}

/// Steps between two residual trace entries (report sampling, not a tunable).
// tunables: const (report layout)
pub const RESIDUAL_TRACE_EVERY: usize = 50;

/// Per-sample inputs of the stream power stage.
pub struct StreamParams<'a> {
    /// Drainage-area exponent m (n is 1).
    pub m: f64,
    /// Number of implicit steps (the cap when `steady_dh_m` is set).
    pub iterations: usize,
    /// `K dt` per sample, units m^(1-2m) (dimensionless for m = 0.5).
    pub k_dt: &'a [f64],
    /// Fraction of the gap to the intent restored per step, per sample, in [0, 1] (`dt / τ`).
    pub restore: &'a [f64],
    pub sea_level: f64,
    pub flow: Flow,
    /// Added to the uplift target and the initial height of dry samples, metres (clamped to stay dry).
    pub perturb: Option<&'a [f64]>,
    /// Stop once the p99 of `|Δh|` per step over dry samples is below this, metres per step.
    pub steady_dh_m: Option<f64>,
}

/// Receiver tree of one routing pass.
#[derive(Debug, Clone, Default)]
pub struct Routing {
    /// Receiver index per sample; a base-level sample is its own receiver.
    pub receiver: Vec<u32>,
    /// Stack order: every receiver before its donors, base level first.
    pub order: Vec<u32>,
    /// Drainage area per sample, m².
    pub area: Vec<f64>,
    /// Dry, non-border samples without a lower receiver (pits after routing). 0 by construction.
    pub pits: usize,
    /// Dry samples the stack never reached. 0 by construction.
    pub unrouted: usize,
}

/// Numbers of one erosion run.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct StreamStats {
    /// Steps run.
    pub iterations: usize,
    /// Residual of the last step: largest and 99th-percentile `|Δh|` over dry samples, metres per step.
    pub residual_max_m: f64,
    pub residual_p99_m: f64,
    /// True when a `steady_dh_m` criterion stopped the run before the cap.
    pub converged: bool,
    /// MFD senders without a lower neighbour, largest over all steps (0 expected).
    pub mfd_stuck: usize,
    /// Residual p99 every [`RESIDUAL_TRACE_EVERY`] steps and at the last step: (step, metres per step).
    pub residual_trace: Vec<(usize, f64)>,
    /// Largest pits-after-routing count over all steps (0 expected).
    pub pits_after_routing: usize,
    /// Largest drainage area at the last step, km².
    pub max_area_km2: f64,
    /// Largest and mean incision below the intent over dry samples, metres.
    pub max_incision_m: f64,
    pub mean_incision_m: f64,
    /// 90th and 99th percentiles of the incision over dry samples, metres.
    pub p90_incision_m: f64,
    pub p99_incision_m: f64,
    /// Dry interior local minima of the final heights (closed depressions left as they are).
    pub raw_pits_final: usize,
}

/// Routes `h`: Priority-Flood+ε surface, D8 steepest descent on it, stack order, drainage area.
pub fn route(g: &Grid, wet: &[bool], sea_level: f64, z: &mut Vec<f64>) -> Routing {
    let (w, hgt, n) = (g.w, g.h, g.len());
    priority_flood_eps(w, hgt, &g.data, wet, sea_level, z);
    let zr: &[f64] = z;
    let mut receiver = vec![0u32; n];
    receiver.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, r) in row.iter_mut().enumerate() {
            let i = y * w + x;
            *r = i as u32;
            if wet[i] || x == 0 || y == 0 || x + 1 == w || y + 1 == hgt {
                continue;
            }
            let mut best = 0.0;
            for (k, dist) in NEIGHBOUR_DIST.iter().enumerate() {
                let Some(j) = neighbour(w, hgt, i, k) else {
                    continue;
                };
                let s = (zr[i] - zr[j]) / dist;
                if s > best {
                    best = s;
                    *r = j as u32;
                }
            }
        }
    });
    let pits = (0..n)
        .filter(|&i| receiver[i] as usize == i && !wet[i] && !g.is_border(i))
        .count();

    // Donors as CSR, then the stack by depth-first search from each base-level sample in index order.
    let mut count = vec![0u32; n + 1];
    for (i, &r) in receiver.iter().enumerate() {
        if r as usize != i {
            count[r as usize + 1] += 1;
        }
    }
    for i in 0..n {
        count[i + 1] += count[i];
    }
    let mut fill = count.clone();
    let mut donors = vec![0u32; count[n] as usize];
    for (i, &r) in receiver.iter().enumerate() {
        if r as usize != i {
            donors[fill[r as usize] as usize] = i as u32;
            fill[r as usize] += 1;
        }
    }
    let mut order = Vec::with_capacity(n);
    let mut dfs = Vec::new();
    for b in 0..n {
        if receiver[b] as usize != b || count[b] == count[b + 1] {
            continue;
        }
        dfs.push(b as u32);
        while let Some(c) = dfs.pop() {
            order.push(c);
            let c = c as usize;
            dfs.extend_from_slice(&donors[count[c] as usize..count[c + 1] as usize]);
        }
    }
    let mut seen = vec![false; n];
    for &i in &order {
        seen[i as usize] = true;
    }
    let unrouted = (0..n)
        .filter(|&i| receiver[i] as usize != i && !seen[i])
        .count();

    let cell = g.spacing * g.spacing;
    let mut area = vec![cell; n];
    for &i in order.iter().rev() {
        let r = receiver[i as usize] as usize;
        if r != i as usize {
            area[r] += area[i as usize];
        }
    }
    Routing {
        receiver,
        order,
        area,
        pits,
        unrouted,
    }
}

/// Runs `p.iterations` stream power steps on `h` toward `intent`. Wet samples (`h < sea_level` on entry) are
/// base level and never change.
pub fn erode(h: &mut Grid, intent: &[f64], p: &StreamParams) -> Result<StreamStats> {
    let n = h.len();
    if intent.len() != n || p.k_dt.len() != n || p.restore.len() != n {
        return Err(Error::Invalid("stream power fields differ in size".into()));
    }
    if p.perturb.is_some_and(|d| d.len() != n) {
        return Err(Error::Invalid("perturbation field differs in size".into()));
    }
    let wet: Vec<bool> = h.data.iter().map(|&v| v < p.sea_level).collect();
    // Uplift target: the intent plus the perturbation, never below sea level on dry samples.
    let target: Vec<f64> = match p.perturb {
        Some(d) => (0..n)
            .map(|i| {
                if wet[i] {
                    intent[i]
                } else {
                    (intent[i] + d[i]).max(p.sea_level)
                }
            })
            .collect(),
        None => intent.to_vec(),
    };
    if let Some(d) = p.perturb {
        for i in 0..n {
            if !wet[i] {
                h.data[i] = (h.data[i] + d[i]).max(p.sea_level);
            }
        }
    }
    let dry_count = wet.iter().filter(|v| !**v).count();
    let mut z = Vec::with_capacity(n);
    let mut prev = h.data.clone();
    let mut dh = Vec::with_capacity(dry_count);
    let mut stats = StreamStats::default();
    let mut routing = Routing::default();
    for _ in 0..p.iterations {
        prev.copy_from_slice(&h.data);
        // Uplift toward the target (explicit; restore ≤ 1 so it never overshoots the target).
        for i in 0..n {
            if !wet[i] {
                let gap = target[i] - h.data[i];
                if gap > 0.0 {
                    h.data[i] += p.restore[i] * gap;
                }
            }
        }
        routing = route(h, &wet, p.sea_level, &mut z);
        stats.pits_after_routing = stats.pits_after_routing.max(routing.pits);
        if routing.unrouted != 0 {
            return Err(Error::Invalid(format!(
                "{} dry samples not reached by the stack",
                routing.unrouted
            )));
        }
        if let Flow::Mfd { exponent } = p.flow {
            let (area, stuck) = mfd_area(h.w, h.h, h.spacing, &z, &wet, exponent);
            routing.area = area;
            stats.mfd_stuck = stats.mfd_stuck.max(stuck);
        }
        sweep(h, &routing, &wet, p);
        stats.iterations += 1;
        dh.clear();
        dh.extend(
            (0..n)
                .filter(|&i| !wet[i])
                .map(|i| (h.data[i] - prev[i]).abs()),
        );
        (stats.residual_max_m, stats.residual_p99_m) = max_and_p99(&mut dh);
        if stats.iterations % RESIDUAL_TRACE_EVERY == 0 {
            stats
                .residual_trace
                .push((stats.iterations, stats.residual_p99_m));
        }
        if p.steady_dh_m.is_some_and(|t| stats.residual_p99_m < t) {
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
            .push((stats.iterations, stats.residual_p99_m));
    }
    stats.max_area_km2 = routing.area.iter().copied().fold(0.0, f64::max) * 1e-6;
    let mut inc: Vec<f64> = (0..n)
        .filter(|&i| !wet[i])
        .map(|i| (intent[i] - h.data[i]).max(0.0))
        .collect();
    if !inc.is_empty() {
        stats.mean_incision_m = inc.iter().sum::<f64>() / inc.len() as f64;
        inc.sort_by(f64::total_cmp);
        let q = |f: f64| inc[((inc.len() - 1) as f64 * f).round() as usize];
        stats.p90_incision_m = q(0.9);
        stats.p99_incision_m = q(0.99);
        stats.max_incision_m = inc[inc.len() - 1];
    }
    stats.raw_pits_final = raw_pits(h, &wet);
    Ok(stats)
}

/// Largest value and 99th percentile (nearest rank, as the incision stats) of `v`; (0, 0) when empty. Reorders `v`.
pub fn max_and_p99(v: &mut [f64]) -> (f64, f64) {
    if v.is_empty() {
        return (0.0, 0.0);
    }
    let k = ((v.len() - 1) as f64 * 0.99).round() as usize;
    let (_, p99, upper) = v.select_nth_unstable_by(k, f64::total_cmp);
    let p99 = *p99;
    let max = upper.iter().copied().fold(p99, f64::max);
    (max, p99)
}

/// Dry samples strictly below all 8 neighbours, none of them wet and not on the border.
pub fn raw_pits(g: &Grid, wet: &[bool]) -> usize {
    (0..g.len())
        .filter(|&i| {
            !wet[i]
                && !g.is_border(i)
                && (0..8).all(|k| {
                    g.neighbour(i, k)
                        .is_some_and(|j| !wet[j] && g.data[j] > g.data[i])
                })
        })
        .count()
}

/// One implicit Braun-Willett sweep along the steepest receivers, from base level upward (stack order), with the
/// drainage area of `routing` (D8 or MFD).
fn sweep(h: &mut Grid, routing: &Routing, wet: &[bool], p: &StreamParams) {
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
        h.data[i] = (hi + f * hr) / (1.0 + f);
    }
}
