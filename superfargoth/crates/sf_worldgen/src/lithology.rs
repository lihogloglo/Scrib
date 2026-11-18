//! **Heterogeneous forcing fields** (WG-S5): an erodibility multiplier from a cellular rock-type map and a
//! low-frequency uplift modulation, both pure functions of absolute world position and a seed.
//!
//! Why: uniform erodibility and uplift run to a strict steady state self-organise into evenly spaced ridges and
//! valleys (Perron, Kirchner, Dietrich, "Formation of evenly spaced ridges and valleys", Nature 460, 2009), the
//! "motherboard" look of WG-S4 C1. Landscape-evolution practice breaks it with spatially varying rock strength and
//! uplift: Veloren drives its erosion with Worley-noise uplift and a rock-strength noise (`world/src/sim/mod.rs`,
//! `erosion.rs`); Landlab's `Lithology` component carries per-unit erodibility; Schott et al. 2023 multiply K by a
//! noise buffer.
//!
//! - **Rock map:** a jittered lattice of feature points every `cell_m` (Worley, "A cellular texture basis function",
//!   SIGGRAPH 1996), each point a rock type drawn by its hash from the recipe's shares; a sample takes the type of its
//!   nearest point (Voronoi cell). The lookup position is domain-warped by value noise so cell edges are not straight.
//!   **Edge bands:** contacts between rock units are weaker (fractured, faulted), so K rises toward a cell edge:
//!   `K × (1 + (edge_k − 1)(1 − smoothstep(d / band)))`, `d` the distance to the Voronoi edge (exact bisector
//!   distance to the second-nearest points of the 3 × 3 neighbourhood).
//! - **Uplift noise:** `U × max(0, 1 + amp × value_noise(p / wavelength))`.
//!
//! Deterministic: integer-hash lattice values ([`crate::perturb::lattice`]), no state; rows map in parallel.

use rayon::prelude::*;

use crate::perturb::{lattice, value_noise};

/// One rock type: K multiplier and share of the map (shares are normalised).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockType {
    pub k: f64,
    pub share: f64,
}

/// The rock-type map's parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockMap {
    /// Lattice spacing of the feature points, metres.
    pub cell_m: f64,
    /// Feature point jitter inside its lattice cell (0 = regular grid, 1 = anywhere in the cell).
    pub jitter: f64,
    /// Domain warp amplitude and wavelength, metres.
    pub warp_m: f64,
    pub warp_wavelength_m: f64,
    /// Width of the softer band along unit edges, metres (0 = none), and its K factor at the edge.
    pub edge_band_m: f64,
    pub edge_k: f64,
    pub seed: u64,
}

/// Rock sample: type index and K multiplier (edge band included).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockSample {
    pub kind: usize,
    pub k: f64,
}

#[inline]
fn smoothstep(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl RockMap {
    /// Feature point of lattice cell (ix, iy), in lattice units.
    fn point(&self, ix: i64, iy: i64) -> [f64; 2] {
        let j = 0.5 * self.jitter;
        [
            ix as f64 + 0.5 + j * lattice(ix, iy, self.seed),
            iy as f64 + 0.5 + j * lattice(ix, iy, self.seed ^ 0x5bd1_e995),
        ]
    }

    /// Rock type of lattice cell (ix, iy) by cumulative share.
    fn kind_of(ix: i64, iy: i64, seed: u64, cum: &[f64]) -> usize {
        let u = f64::midpoint(lattice(ix, iy, seed ^ 0x2545_f491), 1.0) * cum.last().copied().unwrap_or(1.0);
        cum.iter().position(|&c| u < c).unwrap_or(cum.len() - 1)
    }

    /// Samples the map at world position `p` (metres). `types` must not be empty.
    pub fn sample(&self, p: [f64; 2], types: &[RockType], cum: &[f64]) -> RockSample {
        let q = if self.warp_m > 0.0 {
            [
                p[0] + self.warp_m * value_noise(p, self.warp_wavelength_m, self.seed.wrapping_add(11)),
                p[1] + self.warp_m * value_noise(p, self.warp_wavelength_m, self.seed.wrapping_add(12)),
            ]
        } else {
            p
        };
        let (u, v) = (q[0] / self.cell_m, q[1] / self.cell_m);
        let (cx, cy) = (u.floor() as i64, v.floor() as i64);
        let mut pts = [([0.0f64; 2], 0i64, 0i64); 9];
        let mut best = (f64::INFINITY, 0usize);
        for (n, (dy, dx)) in (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dy, dx))).enumerate() {
            let (ix, iy) = (cx + dx, cy + dy);
            let f = self.point(ix, iy);
            pts[n] = (f, ix, iy);
            let d2 = (f[0] - u).powi(2) + (f[1] - v).powi(2);
            if d2 < best.0 {
                best = (d2, n);
            }
        }
        let (f1, ix, iy) = pts[best.1];
        let kind = Self::kind_of(ix, iy, self.seed, cum);
        let mut k = types[kind].k;
        if self.edge_band_m > 0.0 {
            // Distance (lattice units) to the bisector between the nearest point and each other point.
            let mut edge = f64::INFINITY;
            for (n, (f, _, _)) in pts.iter().enumerate() {
                if n == best.1 {
                    continue;
                }
                let (ex, ey) = (f[0] - f1[0], f[1] - f1[1]);
                let len = ex.hypot(ey);
                if len > 0.0 {
                    let d2a = (f[0] - u).powi(2) + (f[1] - v).powi(2);
                    edge = edge.min((d2a - best.0) / (2.0 * len));
                }
            }
            let t = edge * self.cell_m / self.edge_band_m;
            k *= 1.0 + (self.edge_k - 1.0) * (1.0 - smoothstep(t));
        }
        RockSample { kind, k }
    }

    /// K multiplier and type index of every sample of a `w` × `h` grid (`world` maps a sample to world metres).
    pub fn field(
        &self,
        w: usize,
        h: usize,
        world: impl Fn(usize, usize) -> [f64; 2] + Sync,
        types: &[RockType],
    ) -> (Vec<f64>, Vec<u8>) {
        let cum = cumulative(types);
        let mut k = vec![0.0; w * h];
        let mut kinds = vec![0u8; w * h];
        k.par_chunks_mut(w)
            .zip(kinds.par_chunks_mut(w))
            .enumerate()
            .for_each(|(y, (kr, tr))| {
                for x in 0..w {
                    let s = self.sample(world(x, y), types, &cum);
                    kr[x] = s.k;
                    tr[x] = s.kind as u8;
                }
            });
        (k, kinds)
    }
}

/// Cumulative shares of `types`.
pub fn cumulative(types: &[RockType]) -> Vec<f64> {
    let mut acc = 0.0;
    types
        .iter()
        .map(|t| {
            acc += t.share.max(0.0);
            acc
        })
        .collect()
}

/// Uplift modulation factor `max(0, 1 + amp × value_noise)` per sample of a `w` × `h` grid.
pub fn uplift_factor(
    w: usize,
    h: usize,
    world: impl Fn(usize, usize) -> [f64; 2] + Sync,
    amp: f64,
    wavelength_m: f64,
    seed: u64,
) -> Vec<f64> {
    let mut out = vec![0.0; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, v) in row.iter_mut().enumerate() {
            *v = (1.0 + amp * value_noise(world(x, y), wavelength_m, seed)).max(0.0);
        }
    });
    out
}
