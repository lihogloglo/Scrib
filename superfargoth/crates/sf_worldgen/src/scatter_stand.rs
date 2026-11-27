//! Scatter v2 stand mosaic and edge band (spec `worldgen-scatter.md` "Design" 3; slice WG-E2c).
//!
//! - Stand density `D(x)` = value noise at `scatter.stand_wavelength_m` mapped to `[0, 1]`, times the best canopy
//!   suitability at the point ([`StandRules::density`]). No canopy species, or none suited, gives `D = 0`.
//! - Classes by the thresholds of `scatter.stand_*_threshold` ([`StandRules::class_of`]): `D < clearing` is a
//!   clearing, `< sparse` a sparse stand, `< inner` an edge stand, the rest an inner stand. A point the caller marks
//!   excluded (settlement footprints and their bands, later WG-V1b clearings) is a *forced* clearing whatever `D`.
//! - Edge band: the distance from a point to the nearest clearing (either kind), capped at `scatter.edge_band_m`,
//!   on a grid at `scatter.field_step_m` aligned to the world origin and grown past the requested square by the band
//!   plus one step ([`StandMap::build`]); exact Euclidean distance transform between grid nodes (Felzenszwalb and
//!   Huttenlocher 2012, `measure::distance_transform`), bilinear between nodes. A point is in the band when it is
//!   not a clearing and that distance is below `edge_band_m`: the regrowth fringe round every clearing.
//!
//! Determinism: every node is a pure function of (seed, fields, exclusion, its world position), and a node's capped
//! distance only reads nodes within the band, which every grid covering the query point holds. So a tile built alone
//! answers bit for bit what a larger build answers over that tile. Nothing here names a game.

use rayon::prelude::*;

use crate::measure::distance_transform;
use crate::perturb::value_noise;
use crate::scatter_fields::{Fields, Suitability};
use crate::scatter_species::ScatterValues;

#[cfg(test)]
#[path = "tests_scatter_stand.rs"]
mod tests;

/// Salt of the stand noise in the scatter seed's hash space.
// tunables: const (hash salt, not a value)
const STAND_SALT: u64 = 0x57a2_d000_0000_00e2;

/// The four stand classes (spec "Design" 3), in rising density.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StandClass {
    Clearing,
    Sparse,
    Edge,
    Inner,
}

/// What the stand answers at one point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StandPoint {
    /// Stand density `D` in `[0, 1]` (the mosaic value; a forced clearing keeps it, the class says clearing).
    pub d: f64,
    pub class: StandClass,
    /// Inside an exclusion (a forced clearing).
    pub forced: bool,
    /// Distance to the nearest clearing, m, capped at `scatter.edge_band_m` (0 in a clearing node's cell).
    pub clearing_distance_m: f64,
    /// Not a clearing and closer than `scatter.edge_band_m` to one.
    pub edge_band: bool,
}

/// The mosaic rules: values, seed and the canopy species' suitabilities.
pub struct StandRules<'a> {
    pub values: &'a ScatterValues,
    /// The scatter seed (`scatter.seed`).
    pub seed: u64,
    /// One suitability per canopy species; `D` scales by the best of them.
    pub canopy: &'a [Suitability],
}

impl StandRules<'_> {
    /// `D(x)`: the stand noise in `[0, 1]` times the best canopy suitability.
    #[must_use]
    pub fn density(&self, fields: &dyn Fields, x: f64, y: f64) -> f64 {
        let s = fields.sample(x, y);
        let best = self.canopy.iter().map(|c| c.at(&s)).fold(0.0, f64::max);
        if best <= 0.0 {
            return 0.0;
        }
        // Maths: value noise in [-1, 1] to [0, 1].
        let n = f64::midpoint(value_noise([x, y], self.values.stand_wavelength_m, self.seed ^ STAND_SALT), 1.0);
        (n * best).clamp(0.0, 1.0)
    }

    /// The class of a density by the stand thresholds.
    #[must_use]
    pub fn class_of(&self, d: f64) -> StandClass {
        let v = self.values;
        if d < v.stand_clearing_threshold {
            StandClass::Clearing
        } else if d < v.stand_sparse_threshold {
            StandClass::Sparse
        } else if d < v.stand_inner_threshold {
            StandClass::Edge
        } else {
            StandClass::Inner
        }
    }
}

/// The stand over a square and its band halo.
pub struct StandMap<'a> {
    rules: StandRules<'a>,
    fields: &'a dyn Fields,
    excluded: &'a (dyn Fn(f64, f64) -> bool + Sync),
    /// Global index of node (0, 0): node `(i, j)` sits at `((ix0 + i) step, (iy0 + j) step)`.
    ix0: i64,
    iy0: i64,
    w: usize,
    h: usize,
    step: f64,
    /// Capped distance to the nearest clearing node, m, row-major (row 0 = lowest y).
    dist: Vec<f64>,
}

impl<'a> StandMap<'a> {
    /// Builds the distance field over `rect = [x0, y0, x1, y1]` grown by `edge_band_m` plus one `field_step_m`.
    /// `excluded(x, y)` marks forced clearings. Answers are exact (and the same for any covering build) inside
    /// `rect`.
    #[must_use]
    // Grid indices of world metres: floor/ceil of a finite coordinate over a step >= 1 m.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn build(
        rules: StandRules<'a>,
        fields: &'a dyn Fields,
        excluded: &'a (dyn Fn(f64, f64) -> bool + Sync),
        rect: [f64; 4],
    ) -> Self {
        let step = rules.values.field_step_m;
        let band = rules.values.edge_band_m;
        let margin = (band / step).ceil() as i64 + 1;
        let ix0 = (rect[0] / step).floor() as i64 - margin;
        let iy0 = (rect[1] / step).floor() as i64 - margin;
        let ix1 = (rect[2] / step).ceil() as i64 + margin;
        let iy1 = (rect[3] / step).ceil() as i64 + margin;
        let w = (ix1 - ix0 + 1) as usize;
        let h = (iy1 - iy0 + 1) as usize;
        let mut seed = vec![false; w * h];
        seed.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            let y = (iy0 + j as i64) as f64 * step;
            for (i, s) in row.iter_mut().enumerate() {
                let x = (ix0 + i as i64) as f64 * step;
                *s = excluded(x, y) || rules.class_of(rules.density(fields, x, y)) == StandClass::Clearing;
            }
        });
        let dist = distance_transform(&seed, w, h).into_iter().map(|d| (d * step).min(band)).collect();
        Self { rules, fields, excluded, ix0, iy0, w, h, step, dist }
    }

    /// The capped node distance at grid node `(i, j)` of this map, m (tests and debug views).
    #[must_use]
    pub fn node_distance_m(&self, i: usize, j: usize) -> f64 {
        self.dist[j * self.w + i]
    }

    /// Grid size `(w, h)` and the world position of node (0, 0), m.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // grid indices of world metres
    pub fn grid(&self) -> (usize, usize, [f64; 2]) {
        (self.w, self.h, [self.ix0 as f64 * self.step, self.iy0 as f64 * self.step])
    }

    /// Distance to the nearest clearing at a point, m (bilinear between nodes, capped at the band).
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn clearing_distance_m(&self, x: f64, y: f64) -> f64 {
        let (u, v) = (x / self.step, y / self.step);
        let (fu, fv) = (u.floor(), v.floor());
        let (tu, tv) = (u - fu, v - fv);
        // Clamped to the grid; inside the built square the four nodes always exist.
        let i = ((fu as i64 - self.ix0).max(0) as usize).min(self.w - 2);
        let j = ((fv as i64 - self.iy0).max(0) as usize).min(self.h - 2);
        let d = |a: usize, b: usize| self.dist[b * self.w + a];
        let lo = d(i, j) + (d(i + 1, j) - d(i, j)) * tu;
        let hi = d(i, j + 1) + (d(i + 1, j + 1) - d(i, j + 1)) * tu;
        lo + (hi - lo) * tv
    }

    /// Everything the stand answers at a point inside the built square.
    #[must_use]
    pub fn at(&self, x: f64, y: f64) -> StandPoint {
        let d = self.rules.density(self.fields, x, y);
        let forced = (self.excluded)(x, y);
        let class = if forced { StandClass::Clearing } else { self.rules.class_of(d) };
        let clearing_distance_m = self.clearing_distance_m(x, y);
        let edge_band = class != StandClass::Clearing && clearing_distance_m < self.rules.values.edge_band_m;
        StandPoint { d, class, forced, clearing_distance_m, edge_band }
    }
}
