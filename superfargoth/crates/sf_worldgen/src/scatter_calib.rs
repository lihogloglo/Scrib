//! Canopy class census and calibration (spec `worldgen-scatter.md` "Design" 4; slice WG-E2o.2): the bake half of
//! the canopy's class targets ([`crate::scatter::CanopyTargets`]).
//!
//! **Census** ([`census`]): a grid at `scatter.census_step_m` over the guide's bounds, at cell centres; per class,
//! land only (not sea, lake, river or the caller's exclusion), a mosaic clearing counting as land with `w = 0`:
//! area `A` and the mean `m` of the canopy weight `sum_s w_s` ([`CandidateRules::canopy_weight`]). Rows are summed
//! in parallel, each row in order, then the rows in order: bitwise the same on any thread count. The census keeps
//! the same counts per parent tile (the calibration's tile choice).
//!
//! **Calibration** ([`calibrate`]): the competition scale `g` per class with `T > 0`, so the canopy winners per
//! hectare `W` meet `T` on the class's purest parent tiles (up to `scatter.calib_area_ha` of its land), rocks and
//! canopy with the full halo ([`CandidateRules::square_canopy`]); `W` is rescaled by `m / m_t` (the tiles' own
//! weight mean). First guess the Matern II closed form `g0 = -ln(1 - x) / x`, `x = T pi R^2 / 10^4`, then the fixed
//! point `g := g T / W`, then the secant in log-log through the last two measurements; every iterate clamped to
//! `[1, intensity_scale_max x m]`, at most `scatter.calib_iters` updates, stop within `scatter.calib_tol`. A raise of `g` by a fraction `f` that raises `W` by less than
//! `scatter.calib_saturation x f`, or `g` at the upper end short of the target, is the crown ceiling: the iterate
//! with the largest `W` is kept and the class flagged. The classes iterate together (one run per step over every
//! active class's tiles, each class at its current `g`), so the result is one function of the inputs.
//!
//! Industry: the bake does the "artist iterates the density" loop of UE PCG and the Foliage Spawner (research
//! `scatter-target-density-2026-11-27.md` Finding 3); Matern II thinning (Matern 1960, U) gives the first guess.
//! Nothing here names a game.

use std::collections::BTreeMap;
use std::path::Path;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::scatter::{CandidateRules, CanopyTargets, ClassTarget, M2_PER_HA, size_mark};
use crate::scatter_fields::Fields;
use crate::scatter_species::Layer;
use crate::scatter_stand::{StandClass, StandMap, StandRules};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_scatter_calib.rs"]
mod tests;

/// File name of the calibration beside the product (`<guide>/scatter/calibration.toml`).
pub const FILE: &str = "calibration.toml";

/// Quantiles of the size mark's mean (`E[h]` of the first guess's crown).
// tunables: const (quadrature)
const SIZE_QUANTILES: u32 = 64;

/// Matern II's `x` past which the closed form is used no more: the first guess is the upper end.
// tunables: const (maths: `-ln(1 - x)` diverges at 1)
const MATERN_X_MAX: f64 = 0.9;

/// Floor of the secant's exponent `b` (a guard: `1 / b` scales the step).
// tunables: const (maths guard)
const SECANT_B_MIN: f64 = 0.05;

/// The census counts of one class (or one class in one parent tile).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassCount {
    /// Land cells.
    pub cells: u64,
    /// Sum of `w = sum_s w_s` over them.
    pub weight_sum: f64,
}

impl ClassCount {
    fn add(&mut self, w: f64) {
        self.cells += 1;
        self.weight_sum += w;
    }

    fn merge(&mut self, o: &Self) {
        self.cells += o.cells;
        self.weight_sum += o.weight_sum;
    }

    /// `m = sum w / N` (0 without land).
    #[must_use]
    pub fn mean(&self) -> f64 {
        if self.cells == 0 { 0.0 } else { self.weight_sum / self.cells as f64 }
    }
}

/// The class census over a guide (module docs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Census {
    /// `scatter.census_step_m`, m.
    pub step_m: f64,
    /// Per class id ([`Fields::class_names`]).
    pub classes: Vec<ClassCount>,
    /// Per class id, the sum of each species' `w_s` (indexed like the file's species; the first guess's mix).
    pub species_weight: Vec<Vec<f64>>,
    /// Per parent tile: every census cell in it (land or not), then per class id its land counts.
    pub tiles: BTreeMap<(i32, i32), (u64, Vec<ClassCount>)>,
}

impl Census {
    /// `A(c)`, ha.
    #[must_use]
    pub fn area_ha(&self, class: usize) -> f64 {
        self.classes.get(class).map_or(0.0, |c| c.cells as f64 * self.step_m * self.step_m / M2_PER_HA)
    }
}

/// One row's sums (cells in x order).
#[derive(Default)]
struct RowSums {
    classes: Vec<ClassCount>,
    species: Vec<Vec<f64>>,
    tiles: BTreeMap<(i32, i32), (u64, Vec<ClassCount>)>,
}

/// The class census of `bounds = [x0, y0, x1, y1]` (module docs): cells of `scatter.census_step_m` from `(x0, y0)`,
/// the whole cells inside the bounds, sampled at their centres. `excluded` marks forced clearings (not land).
#[must_use]
pub fn census(
    rules: &CandidateRules<'_>,
    stand: &StandRules<'_>,
    fields: &dyn Fields,
    excluded: &(dyn Fn(f64, f64) -> bool + Sync),
    bounds: [f64; 4],
) -> Census {
    let step = rules.file.values.census_step_m;
    let n_class = fields.class_names().len();
    let n_species = rules.file.species.len();
    let nx = ((bounds[2] - bounds[0]) / step).floor().max(0.0) as u64;
    let ny = ((bounds[3] - bounds[1]) / step).floor().max(0.0) as u64;
    let t = rules.tile_m;
    let rows: Vec<RowSums> = (0..ny)
        .into_par_iter()
        .map(|j| {
            let y = bounds[1] + (j as f64 + 0.5) * step;
            let ty = (y / t).floor() as i32;
            let mut row = RowSums {
                classes: vec![ClassCount::default(); n_class],
                species: vec![vec![0.0; n_species]; n_class],
                tiles: BTreeMap::new(),
            };
            let mut ws = vec![0.0; n_species];
            for i in 0..nx {
                let x = bounds[0] + (i as f64 + 0.5) * step;
                let tile = row.tiles.entry(((x / t).floor() as i32, ty)).or_insert_with(|| (0, vec![ClassCount::default(); n_class]));
                tile.0 += 1;
                let Some(c) = fields.sample(x, y).class.map(usize::from).filter(|&c| c < n_class) else { continue };
                if excluded(x, y) || rules.water.wet(x, y, (rules.height)(x, y)) {
                    continue;
                }
                let d = stand.density(fields, x, y);
                let w = if stand.class_of(d) == StandClass::Clearing {
                    ws.fill(0.0);
                    0.0
                } else {
                    rules.canopy_weight(fields, x, y, d, &mut ws)
                };
                row.classes[c].add(w);
                tile.1[c].add(w);
                for (acc, w) in row.species[c].iter_mut().zip(&ws) {
                    *acc += w;
                }
            }
            row
        })
        .collect();
    let mut out = Census {
        step_m: step,
        classes: vec![ClassCount::default(); n_class],
        species_weight: vec![vec![0.0; n_species]; n_class],
        tiles: BTreeMap::new(),
    };
    for row in rows {
        for (acc, c) in out.classes.iter_mut().zip(&row.classes) {
            acc.merge(c);
        }
        for (acc, s) in out.species_weight.iter_mut().zip(&row.species) {
            acc.iter_mut().zip(s).for_each(|(a, b)| *a += b);
        }
        for (key, (cells, per)) in row.tiles {
            let tile = out.tiles.entry(key).or_insert_with(|| (0, vec![ClassCount::default(); n_class]));
            tile.0 += cells;
            for (acc, c) in tile.1.iter_mut().zip(&per) {
                acc.merge(c);
            }
        }
    }
    out
}

/// One class's calibration, as written to [`FILE`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClassCalibration {
    /// The class name ([`Fields::class_names`]).
    pub name: String,
    /// `T`, winners per ha.
    pub stems_per_ha: f64,
    /// `A`, ha of land.
    pub area_ha: f64,
    /// `m`, the census weight mean.
    pub weight_mean: f64,
    /// `g`, the competition scale kept.
    pub scale: f64,
    /// `W` at the kept `g`, winners per ha (rescaled by `m / m_t`); 0 when not calibrated.
    pub reached_per_ha: f64,
    /// Land of the calibration tiles, ha.
    pub calib_area_ha: f64,
    /// Updates of `g` after the first guess.
    pub steps: u32,
    /// The target is above the crown ceiling (or the multiplier clamp): `W` stays short of `T`.
    pub ceiling: bool,
    /// `intensity_scale_max x m < 1`: the class's weight mean is too low for its target.
    pub low_weight: bool,
}

/// The calibration of every class (module docs).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(rename = "class")]
    pub classes: Vec<ClassCalibration>,
    /// Bake log lines (not written).
    #[serde(skip)]
    pub warnings: Vec<String>,
}

impl Calibration {
    /// The canopy targets of the passes (`T`, `m`, `g` per class id).
    #[must_use]
    pub fn targets(&self) -> CanopyTargets {
        CanopyTargets {
            classes: self
                .classes
                .iter()
                .map(|c| ClassTarget { stems_per_ha: c.stems_per_ha, weight_mean: c.weight_mean, scale: c.scale })
                .collect(),
        }
    }

    /// Writes `dir/scatter/calibration.toml`.
    pub fn write(&self, dir: &Path) -> Result<()> {
        let out = dir.join(crate::scatter_product::DIR);
        std::fs::create_dir_all(&out)?;
        let text = toml::to_string(self).map_err(|e| Error::Invalid(format!("calibration: {e}")))?;
        std::fs::write(out.join(FILE), text)?;
        Ok(())
    }

    /// Reads `dir/scatter/calibration.toml` (written by [`Self::write`]; no warnings).
    pub fn read(dir: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(dir.join(crate::scatter_product::DIR).join(FILE))?;
        Ok(toml::from_str(&text)?)
    }
}

/// A class's iteration state.
struct Iterate {
    class: usize,
    target: f64,
    lo: f64,
    hi: f64,
    g: f64,
    /// `(g, W)` per measurement.
    seen: Vec<(f64, f64)>,
    steps: u32,
    tiles: Vec<(i32, i32)>,
    land_ha: f64,
    /// `m / m_t`.
    bias: f64,
    done: bool,
    ceiling: bool,
    converged: bool,
}

/// The parent tiles of class `c` in decreasing census share (ties by tile key) until their `c` land reaches
/// `area_ha` (all of them when the class has less), with that land, ha, and its weight mean `m_t`.
fn calib_tiles(census: &Census, c: usize, area_ha: f64) -> (Vec<(i32, i32)>, f64, f64) {
    let cell_ha = census.step_m * census.step_m / M2_PER_HA;
    let mut ranked: Vec<(f64, (i32, i32))> = census
        .tiles
        .iter()
        .filter(|(_, (_, per))| per[c].cells > 0)
        .map(|(&k, (all, per))| (per[c].cells as f64 / *all as f64, k))
        .collect();
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let (mut tiles, mut land) = (Vec::new(), ClassCount::default());
    for (_, k) in ranked {
        if land.cells as f64 * cell_ha >= area_ha {
            break;
        }
        land.merge(&census.tiles[&k].1[c]);
        tiles.push(k);
    }
    (tiles, land.cells as f64 * cell_ha, land.mean())
}

/// The Matern II first guess of class `c` (module docs), before the clamp.
fn first_guess(rules: &CandidateRules<'_>, census: &Census, c: usize, target: f64, hi: f64) -> f64 {
    let w = &census.species_weight[c];
    let total: f64 = w.iter().sum();
    let r2 = rules
        .file
        .species
        .iter()
        .zip(w)
        .filter(|(sp, _)| sp.layer == Layer::Canopy)
        .map(|(sp, &ws)| {
            let q = f64::from(SIZE_QUANTILES);
            let h = (0..SIZE_QUANTILES).map(|i| size_mark(sp, (f64::from(i) + 0.5) / q)).sum::<f64>() / q;
            let r = sp.crown_k * 2.0 * (sp.crown_a_m + sp.crown_b * h);
            if total > 0.0 { ws / total * r * r } else { 0.0 }
        })
        .sum::<f64>();
    let x = target * std::f64::consts::PI * r2 / M2_PER_HA;
    if x >= MATERN_X_MAX {
        hi
    } else if x <= 0.0 {
        1.0
    } else {
        -(1.0 - x).ln() / x
    }
}

/// The competition scale per class (module docs) from a [`census`] of the same inputs; `rules.canopy` is not
/// read (each step builds its own targets). Classes with land and no canopy (`T` or `m` at 0) keep `g` = 1.
#[must_use]
pub fn calibrate(
    rules: &CandidateRules<'_>,
    stand: &StandRules<'_>,
    fields: &dyn Fields,
    excluded: &(dyn Fn(f64, f64) -> bool + Sync),
    census: &Census,
) -> Calibration {
    let v = &rules.file.values;
    let names = fields.class_names();
    let mut out = Calibration::default();
    let mut its: Vec<Iterate> = Vec::new();
    for (c, name) in names.iter().enumerate() {
        let target = rules.file.classes.get(name).map_or(0.0, |e| e.v);
        let count = &census.classes[c];
        let m = count.mean();
        let mut row = ClassCalibration {
            name: name.clone(),
            stems_per_ha: target,
            area_ha: census.area_ha(c),
            weight_mean: m,
            scale: 1.0,
            ..ClassCalibration::default()
        };
        if count.cells > 0 && (target <= 0.0 || m <= 0.0) {
            out.warnings.push(format!("class {name}: no canopy (T {target}, m {m:.3})"));
        }
        if target > 0.0 && m > 0.0 {
            let hi = v.intensity_scale_max * m;
            let lo = hi.min(1.0);
            if hi < 1.0 {
                row.low_weight = true;
                out.warnings.push(format!("class {name}: weight mean m too low ({m:.3})"));
            }
            let (tiles, land_ha, m_t) = calib_tiles(census, c, v.calib_area_ha);
            row.calib_area_ha = land_ha;
            let g = first_guess(rules, census, c, target, hi).clamp(lo, hi);
            row.scale = g;
            its.push(Iterate {
                class: c,
                target,
                lo,
                hi,
                g,
                seen: Vec::new(),
                steps: 0,
                tiles,
                land_ha,
                bias: if m_t > 0.0 { m / m_t } else { 0.0 },
                done: false,
                ceiling: false,
                converged: false,
            });
        }
        out.classes.push(row);
    }
    while its.iter().any(|it| !it.done) {
        for it in &its {
            out.classes[it.class].scale = it.g;
        }
        let targets = out.targets();
        let step_rules = CandidateRules { canopy: &targets, ..*rules };
        let mut tiles: Vec<(i32, i32)> = its.iter().filter(|it| !it.done).flat_map(|it| it.tiles.iter().copied()).collect();
        tiles.sort_unstable();
        tiles.dedup();
        let won = winners_per_tile(&step_rules, stand, fields, excluded, &tiles, names.len());
        for it in its.iter_mut().filter(|it| !it.done) {
            let n: u64 = it.tiles.iter().map(|k| won[k][it.class]).sum();
            let w = if it.land_ha > 0.0 { n as f64 / it.land_ha * it.bias } else { 0.0 };
            step(it, w, v.calib_tol, v.calib_saturation, v.calib_iters);
        }
    }
    for it in &its {
        let row = &mut out.classes[it.class];
        let kept = kept(it);
        row.scale = kept.0;
        row.reached_per_ha = kept.1;
        row.steps = it.steps;
        row.ceiling = it.ceiling;
        if it.ceiling {
            out.warnings
                .push(format!("class {}: target {} above the crown ceiling, reached {:.1}", row.name, it.target, kept.1));
        }
    }
    out
}

/// The iterate a finished class keeps: converged, its last; at the ceiling, the largest `W` (ties: the smaller
/// `g`); else the nearest `W` to `T` (ties: the smaller `g`).
fn kept(it: &Iterate) -> (f64, f64) {
    let last = *it.seen.last().expect("a finished class was measured");
    if it.converged {
        return last;
    }
    let key = |&(g, w): &(f64, f64)| if it.ceiling { (-w, g) } else { ((w - it.target).abs(), g) };
    *it.seen.iter().min_by(|a, b| key(a).partial_cmp(&key(b)).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&last)
}

/// One measurement `w` of a class at its `g` (module docs): converged, ceiling, out of steps, or the next `g`.
#[allow(clippy::float_cmp)] // exact: the same `g` twice (a clamp end) measures the same `W`
fn step(it: &mut Iterate, w: f64, tol: f64, saturation: f64, iters: u32) {
    let t = it.target;
    let saturated = it.seen.last().is_some_and(|&(g0, w0)| it.g > g0 && w0 > 0.0 && w / w0 - 1.0 < saturation * (it.g / g0 - 1.0));
    it.seen.push((it.g, w));
    if (w - t).abs() <= tol * t {
        it.converged = true;
        it.done = true;
        return;
    }
    if saturated || (it.g >= it.hi && w < (1.0 - tol) * t) {
        it.ceiling = true;
        it.done = true;
        return;
    }
    // The secant in log-log (`W ~ a g^b` through the last two measurements; `b` = 1, the plain fixed point
    // `g T / W`, on the first): `W` flattens as crowns pack, so the fixed point alone crawls (E2o.2).
    let b = match it.seen.len().checked_sub(2).map(|i| it.seen[i]) {
        Some((g0, w0)) if g0 != it.g && w0 > 0.0 => ((w / w0).ln() / (it.g / g0).ln()).clamp(saturation.max(SECANT_B_MIN), 1.0),
        _ => 1.0,
    };
    let next = if w <= 0.0 { it.hi } else { (it.g * (t / w).powf(1.0 / b)).clamp(it.lo, it.hi) };
    if it.steps >= iters || next == it.g {
        it.done = true;
        return;
    }
    it.g = next;
    it.steps += 1;
}

/// Canopy winners per tile and class id (the class at the winner's point), tiles in parallel, each with its own
/// stand map over [`CandidateRules::tile_rect`] (exact winners).
fn winners_per_tile(
    rules: &CandidateRules<'_>,
    stand: &StandRules<'_>,
    fields: &dyn Fields,
    excluded: &(dyn Fn(f64, f64) -> bool + Sync),
    tiles: &[(i32, i32)],
    n_class: usize,
) -> BTreeMap<(i32, i32), Vec<u64>> {
    tiles
        .par_iter()
        .map(|&tile| {
            let sr = StandRules { values: stand.values, seed: stand.seed, canopy: stand.canopy };
            let map = StandMap::build(sr, fields, excluded, rules.tile_rect(tile));
            let mut n = vec![0u64; n_class];
            for c in rules.square_canopy(fields, &map, rules.tile_square(tile)) {
                if let Some(k) = fields.sample(c.pos[0], c.pos[1]).class.map(usize::from).filter(|&k| k < n_class) {
                    n[k] += 1;
                }
            }
            (tile, n)
        })
        .collect()
}
