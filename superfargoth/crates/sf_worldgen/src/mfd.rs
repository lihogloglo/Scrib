//! Drainage area by **multiple-flow-direction** (MFD) accumulation.
//!
//! Each dry interior sample splits its area among all its strictly lower 8-neighbours on the routing surface,
//! with weights `tanβₖᵖ / Σ tanβⱼᵖ` (Freeman, "Calculating catchment area with divergent flow based on a regular
//! grid", Computers & Geosciences 17(3), 1991, recommends p = 1.1; Quinn, Beven, Chevallier, Planchon, "The
//! prediction of hillslope flow paths for distributed hydrological modelling using digital terrain models",
//! Hydrological Processes 5, 1991, use p = 1 with contour-length factors; Holmgren, "Multiple flow direction
//! algorithms for runoff modelling in grid based elevation models", Hydrological Processes 8, 1994, shows large p
//! tends to D8). Schott et al. (ACM TOG 42(5), 2023; code `H-Schott/StreamPowerErosion`, `GetFlowLpCoeff`) use
//! exactly these slope-power weights (p = 1) for the drainage area of stream power erosion, while the incision
//! itself follows the steepest receiver. D-infinity (Tarboton, Water Resources Research 33(2), 1997) splits
//! between at most two neighbours (less dispersion); it was not chosen because the slope-power MFD is what the
//! erosion-for-terrain literature above uses and needs no facet geometry.
//!
//! Order: Kahn's topological sort of the flow DAG (every flow edge goes strictly down the routing surface, so it
//! is acyclic), with a FIFO seeded in index order: single-threaded and deterministic. Only the in-degree count and
//! the raw weights, pure per-sample maps, run in parallel. Wet samples and dry border samples are sinks (base level), as in
//! [`crate::flood`].
//!
//! Flats: on the ε-raised flats of the routing surface the drops can underflow to zero slope (a sample one ulp
//! above a sea level of 0 sits at a subnormal height). A sender whose weights all vanish sends everything to its
//! lowest neighbour, the direction the flood came from (D8 on flats, as Priority-Flood+ε routing intends).

use rayon::prelude::*;

use crate::grid::{NEIGHBOUR_DIST, neighbour};

/// True when sample `i` passes flow on (dry, not on the border).
#[inline]
fn sends(w: usize, h: usize, wet: &[bool], i: usize) -> bool {
    let (x, y) = (i % w, i / w);
    !wet[i] && x != 0 && y != 0 && x + 1 != w && y + 1 != h
}

/// Raw flow weights `tanβₖᵖ` of sender `c` (0 for neighbours not strictly lower), stored as f32 so the parallel
/// pass can keep all of them (32 bytes per sample); normalised in f64 by [`normalise`].
#[inline]
fn raw_weights(w: usize, h: usize, spacing: f64, z: &[f64], c: usize, exponent: f64) -> [f32; 8] {
    let mut out = [0.0f32; 8];
    for (k, wk) in out.iter_mut().enumerate() {
        if let Some(j) = neighbour(w, h, c, k)
            && z[j] < z[c]
        {
            let s = (z[c] - z[j]) / (NEIGHBOUR_DIST[k] * spacing);
            *wk = if exponent.to_bits() == 1.0f64.to_bits() {
                s
            } else {
                s.powf(exponent)
            } as f32;
        }
    }
    out
}

/// Normalised weights (sum 1) from the raw ones; when every raw weight vanished (flat), all flow goes to the
/// lowest strictly lower neighbour. `None` when no neighbour is lower.
#[inline]
fn normalise(w: usize, h: usize, z: &[f64], c: usize, raw: &[f32; 8]) -> Option<[f64; 8]> {
    let sum: f64 = raw.iter().map(|&v| f64::from(v)).sum();
    if sum > 0.0 && sum.is_finite() {
        return Some(raw.map(|v| f64::from(v) / sum));
    }
    let mut lowest: Option<(usize, f64)> = None;
    for k in 0..8 {
        if let Some(j) = neighbour(w, h, c, k)
            && z[j] < z[c]
            && lowest.is_none_or(|(_, zl)| z[j] < zl)
        {
            lowest = Some((k, z[j]));
        }
    }
    lowest.map(|(kl, _)| {
        let mut out = [0.0; 8];
        out[kl] = 1.0;
        out
    })
}

/// MFD drainage area per sample, m², on routing surface `z` (strictly lower neighbours only). `exponent` is p.
/// Returns the area and the number of senders that found no lower neighbour (0 on a Priority-Flood+ε surface).
pub fn mfd_area(
    w: usize,
    h: usize,
    spacing: f64,
    z: &[f64],
    wet: &[bool],
    exponent: f64,
) -> (Vec<f64>, usize) {
    let n = w * h;
    // Pure per-sample maps in parallel: in-degree (senders above) and raw outgoing weights.
    let mut indeg = vec![0u8; n];
    let mut raw = vec![[0.0f32; 8]; n];
    indeg
        .par_chunks_mut(w)
        .zip(raw.par_chunks_mut(w))
        .enumerate()
        .for_each(|(y, (drow, wrow))| {
            for (x, (d, wk)) in drow.iter_mut().zip(wrow.iter_mut()).enumerate() {
                let j = y * w + x;
                *d = (0..8)
                    .filter(|&k| {
                        neighbour(w, h, j, k).is_some_and(|i| sends(w, h, wet, i) && z[i] > z[j])
                    })
                    .count() as u8;
                if sends(w, h, wet, j) {
                    *wk = raw_weights(w, h, spacing, z, j, exponent);
                }
            }
        });
    let mut area = vec![spacing * spacing; n];
    let mut queue: Vec<u32> = (0..n as u32).filter(|&i| indeg[i as usize] == 0).collect();
    queue.reserve(n - queue.len());
    let mut head = 0;
    let mut stuck = 0;
    while head < queue.len() {
        let c = queue[head] as usize;
        head += 1;
        if !sends(w, h, wet, c) {
            continue;
        }
        let Some(wts) = normalise(w, h, z, c, &raw[c]) else {
            stuck += 1;
            continue;
        };
        let a = area[c];
        // Every strictly lower neighbour counted this sender in its in-degree, weight or not.
        for (k, &wk) in wts.iter().enumerate() {
            if let Some(j) = neighbour(w, h, c, k)
                && z[j] < z[c]
            {
                area[j] += a * wk;
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    queue.push(j as u32);
                }
            }
        }
    }
    (area, stuck)
}

/// Area passed from senders into sinks (wet or dry border samples) by the MFD network. When the accumulation
/// conserves area it equals the summed cell area of all senders (the area conservation test).
pub fn outflow_into_sinks(
    w: usize,
    h: usize,
    spacing: f64,
    z: &[f64],
    wet: &[bool],
    exponent: f64,
    area: &[f64],
) -> f64 {
    let mut out = 0.0;
    for (c, &ac) in area.iter().enumerate() {
        if !sends(w, h, wet, c) {
            continue;
        }
        let Some(wts) = normalise(w, h, z, c, &raw_weights(w, h, spacing, z, c, exponent)) else {
            continue;
        };
        for (k, &wk) in wts.iter().enumerate() {
            if let Some(j) = neighbour(w, h, c, k)
                && wk > 0.0
                && !sends(w, h, wet, j)
            {
                out += ac * wk;
            }
        }
    }
    out
}
