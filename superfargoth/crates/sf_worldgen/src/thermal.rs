//! Thermal erosion by talus-angle relaxation (Musgrave, Kolb, Mace, "The synthesis and rendering of eroded
//! fractal terrains", SIGGRAPH 1989; the 8-neighbour form of Olsen, "Realtime procedural terrain generation",
//! 2004).
//!
//! Per step, a dry sample whose drop to a dry neighbour exceeds the talus height `tan(θ) Δx` moves
//! `c (d_max - T)` of material to those neighbours, shared by their excess. Wet samples neither give nor take
//! (sea is fixed base level). Mass is conserved among dry samples, and with `c ≤ 0.5` a donor never ends below
//! its lowest receiver, so dry stays dry.
//!
//! Jacobi form in two gather passes (each sample computes what it gives, then each sample sums what it gets
//! from its neighbours in a fixed order), so the parallel map is deterministic.

use rayon::prelude::*;

use crate::grid::{NEIGHBOUR_DIST, neighbour};
use crate::{Error, Grid, Result};

/// Inputs of the thermal stage.
pub struct ThermalParams<'a> {
    pub iterations: usize,
    /// Fraction of the excess moved per step, (0, 0.5].
    pub rate: f64,
    /// `tan(talus angle)` per sample.
    pub tan_talus: &'a [f64],
    pub sea_level: f64,
}

/// Excess of the drop from sample `i` to neighbour `k` over the talus height, or 0.
#[inline]
fn excess(g: &Grid, wet: &[bool], tan_talus: &[f64], i: usize, k: usize) -> f64 {
    let Some(j) = neighbour(g.w, g.h, i, k) else {
        return 0.0;
    };
    if wet[j] {
        return 0.0;
    }
    let dist = NEIGHBOUR_DIST[k] * g.spacing;
    (g.data[i] - g.data[j] - tan_talus[i] * dist).max(0.0)
}

/// Runs the thermal stage in place. Returns the material moved over all steps, m³.
pub fn relax(h: &mut Grid, p: &ThermalParams) -> Result<f64> {
    if !(p.rate > 0.0 && p.rate <= 0.5) || p.tan_talus.len() != h.len() {
        return Err(Error::Invalid(
            "thermal rate must be in (0, 0.5] and fields match the grid".into(),
        ));
    }
    let wet: Vec<bool> = h.data.iter().map(|&v| v < p.sea_level).collect();
    let (w, n) = (h.w, h.len());
    let mut give = vec![0.0f64; n];
    let mut sum_excess = vec![0.0f64; n];
    let mut next = vec![0.0f64; n];
    let mut moved = 0.0;
    for _ in 0..p.iterations {
        let g: &Grid = h;
        give.par_chunks_mut(w)
            .zip(sum_excess.par_chunks_mut(w))
            .enumerate()
            .for_each(|(y, (gr, sr))| {
                for x in 0..w {
                    let i = y * w + x;
                    gr[x] = 0.0;
                    sr[x] = 0.0;
                    if wet[i] {
                        continue;
                    }
                    let mut sum = 0.0;
                    let mut max = 0.0f64;
                    for k in 0..8 {
                        let e = excess(g, &wet, p.tan_talus, i, k);
                        sum += e;
                        max = max.max(e);
                    }
                    if max > 0.0 {
                        gr[x] = p.rate * max;
                        sr[x] = sum;
                    }
                }
            });
        let (give_r, sum_r) = (&give, &sum_excess);
        next.par_chunks_mut(w).enumerate().for_each(|(y, nr)| {
            for (x, out) in nr.iter_mut().enumerate() {
                let i = y * w + x;
                let mut v = g.data[i];
                if !wet[i] {
                    v -= give_r[i];
                    for k in 0..8 {
                        let Some(j) = neighbour(g.w, g.h, i, k) else {
                            continue;
                        };
                        if give_r[j] > 0.0 {
                            // Neighbour j gives toward i along the opposite direction of k.
                            let e = excess(g, &wet, p.tan_talus, j, (k + 4) % 8);
                            if e > 0.0 {
                                v += give_r[j] * e / sum_r[j];
                            }
                        }
                    }
                }
                *out = v;
            }
        });
        moved += give.iter().sum::<f64>() * h.spacing * h.spacing;
        std::mem::swap(&mut h.data, &mut next);
    }
    Ok(moved)
}
