//! Canopy class census and calibration (spec `worldgen-scatter.md` "Design" 4; slice WG-E2o.2): the bake half of
//! the canopy's class targets ([`crate::scatter::CanopyTargets`]).
//!
//! **Census** ([`census`]): a grid at `scatter.census_step_m` over the guide's bounds, at cell centres; per class,
//! land only (not sea, lake, river or the caller's exclusion), a mosaic clearing counting as land with `w = 0`:
//! area `A` and the mean `m` of the canopy weight `sum_s w_s` ([`CandidateRules::canopy_weight`]). Rows are summed
//! in parallel, each row in order, then the rows in order: bitwise the same on any thread count. The census keeps
//! the same counts per parent tile (the calibration's sample).
//!
//! **Calibration** ([`calibrate`]): the competition scale `g` per class with `T > 0`, so the canopy winners per
//! hectare `W` meet `T` on a sample of the class's parent tiles ([`calib_tiles`], WG-E2o.e/f: one world order of the
//! census tiles by `hash(seed, tile key)` ([`tile_order`]), shared by every class; each class takes, in that order,
//! the tiles holding its land until `scatter.calib_area_ha` of it), rocks and canopy with the full halo
//! ([`CandidateRules::square_canopy`]). `W` = winners at class points / class land on those tiles, no rescale (the
//! forest-inventory ratio-of-means; the tiles' weight mean `m_t` is reported, not applied). First guess the Matern
//! II closed form `g0 = -ln(1 - x) / x`, `x = T pi R^2 / 10^4`, then the fixed
//! point `g := g T / W`, then the secant in log-log through the last two measurements; every iterate clamped to
//! `[1, intensity_scale_max x m]`, at most `scatter.calib_iters` updates, stop within `scatter.calib_tol`. A raise of `g` by a fraction `f` that raises `W` by less than
//! `scatter.calib_saturation x f`, or `g` at the upper end short of the target, is the crown ceiling: the iterate
//! with the largest `W` is kept and the class flagged. The classes iterate together (one run per step over every
//! active class's tiles, each class at its current `g`, a finished one at its kept `g`), so the result is one
//! function of the inputs. **Joint check**: once no class is active, one run over every class's sample at every
//! class's kept `g`; a converged class now outside `scatter.calib_tol` is reopened from that measurement (a class's
//! `W` on mixed tiles moves with its neighbours' `g`, E2o.d); at most `scatter.calib_iters` checks, the last one
//! reopens nothing. The file reports the last check's `g` and `W`: one consistent snapshot.
//!
//! **Sample size** (WG-E2o.h, two-stage sampling, Cochran 1977 ch. 4 and 6): the `calib_area_ha` prefix is the pilot,
//! at least `scatter.calib_tiles_min` tiles holding the class. When the per-class iteration first finishes, each
//! converged class is sized once from its last pilot run: per tile `x` = its class land, `y` = winners at class
//! points, `CV_r` = the ratio estimator's residual SD over `R x_mean` ([`ratio_cv`]), `n = (calib_z CV_r /
//! calib_tol)^2` with the finite-population correction over `N(c)`, the tiles holding the class, at most
//! `min(N(c), calib_tiles_max)` and at least the pilot ([`sized_tiles`]); the sample becomes the first `n` tiles of
//! the same order ([`first_tiles`]; the pilot is its prefix) and the joint checks run on it. Ceiling, out-of-steps
//! and `R = 0` classes keep the pilot. `rse = CV_r sqrt((1 - n / N) / n)` on the final sample is reported; a class
//! `calib_tiles_max` cut with `calib_z rse > calib_tol` warns.
//!
//! Industry: the bake does the "artist iterates the density" loop of UE PCG and the Foliage Spawner (research
//! `scatter-target-density-2026-11-27.md` Finding 3); Matern II thinning (Matern 1960, U) gives the first guess.
//! Nothing here names a game.

use std::collections::BTreeMap;
use std::path::Path;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::scatter::{CandidateRules, CanopyTargets, ClassTarget, M2_PER_HA, mix, size_mark_to};
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

/// Salt of the calibration's tile order (a hash domain, not a value).
// tunables: const (hash salt)
const ORDER_SALT: u64 = 0xca1b_0a7e_5a3b_1e00;

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
#[serde(default)]
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
    /// `W` at the kept `g` in the last joint check, winners per ha of the sample's class land; 0 when not calibrated.
    pub reached_per_ha: f64,
    /// Class land of the sample's tiles, ha.
    pub calib_area_ha: f64,
    /// Tiles in the sample (the sized one: the first `calib_tiles` of the world order holding the class).
    pub calib_tiles: u32,
    /// Tiles in the pilot (the `calib_area_ha` prefix, at least `calib_tiles_min`); a prefix of the sample.
    pub pilot_tiles: u32,
    /// `N(c)`, the census tiles holding the class.
    pub class_tiles: u32,
    /// `CV_r` of the pilot's last run, which sized the sample (0 when not sized: ceiling, out of steps, `R = 0`).
    pub pilot_cv_r: f64,
    /// `CV_r`, the ratio estimator's unit CV over the final sample at the last joint check (0 below two tiles).
    pub cv_r: f64,
    /// `rse`, the relative SE of `reached` on the final sample.
    pub rse: f64,
    /// `calib_tiles_max` cut the sample size the pilot asked for.
    pub capped: bool,
    /// `m_t`, the census weight mean over the sample's class land (reported, not applied).
    pub calib_weight_mean: f64,
    /// Updates of `g` after the first guess (a reopened class's included).
    pub steps: u32,
    /// Joint checks run (the same for every calibrated class).
    pub joint_checks: u32,
    /// Times a joint check reopened this class.
    pub reopened: u32,
    /// The target is above the crown ceiling (or the multiplier clamp): `W` stays short of `T`.
    pub ceiling: bool,
    /// `intensity_scale_max x m < 1`: the class's weight mean is too low for its target.
    pub low_weight: bool,
    /// `class.<name>.size_max_m`, m: the canopy size cap of the class (absent: none).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_max_m: Option<f64>,
}

/// The calibration of every class (module docs).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Calibration {
    #[serde(rename = "class")]
    pub classes: Vec<ClassCalibration>,
    /// Bake log lines (not written).
    #[serde(skip)]
    pub warnings: Vec<String>,
    /// Wall seconds of [`calibrate`] (not written; 0 when read).
    #[serde(skip)]
    pub seconds: f64,
}

impl Calibration {
    /// The canopy targets of the passes (`T`, `m`, `g` per class id).
    #[must_use]
    pub fn targets(&self) -> CanopyTargets {
        CanopyTargets {
            classes: self
                .classes
                .iter()
                .map(|c| ClassTarget {
                    stems_per_ha: c.stems_per_ha,
                    weight_mean: c.weight_mean,
                    scale: c.scale,
                    size_max_m: c.size_max_m,
                })
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
#[derive(Clone, Debug)]
struct Iterate {
    target: f64,
    lo: f64,
    hi: f64,
    g: f64,
    /// `(g, W)` per measurement.
    seen: Vec<(f64, f64)>,
    steps: u32,
    done: bool,
    ceiling: bool,
    converged: bool,
    reopened: u32,
    /// `(g, W)` of the last joint check.
    joint: Option<(f64, f64)>,
}

impl Iterate {
    fn new(target: f64, lo: f64, hi: f64, g: f64) -> Self {
        Self {
            target,
            lo,
            hi,
            g,
            seen: Vec::new(),
            steps: 0,
            done: false,
            ceiling: false,
            converged: false,
            reopened: 0,
            joint: None,
        }
    }

    /// The `g` a run measures this class at: its current one while active, its kept one once finished.
    fn run_g(&self) -> f64 {
        if self.done { kept(self).0 } else { self.g }
    }
}

/// A class's calibration sample ([`calib_tiles`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sample {
    /// The tiles, in the world order.
    pub tiles: Vec<(i32, i32)>,
    /// Their class land, ha.
    pub land_ha: f64,
    /// `m_t`, the census weight mean over that land.
    pub weight_mean: f64,
}

/// The census's parent tiles in the calibration's world order: by `hash(seed, tile key)`, ties by tile key.
#[must_use]
pub fn tile_order(census: &Census, seed: u64) -> Vec<(i32, i32)> {
    let base = mix(seed ^ ORDER_SALT);
    let mut keyed: Vec<(u64, (i32, i32))> = census
        .tiles
        .keys()
        .map(|&(x, y)| (mix(base ^ (u64::from(x.cast_unsigned()) << 32 | u64::from(y.cast_unsigned()))), (x, y)))
        .collect();
    keyed.sort_unstable();
    keyed.into_iter().map(|(_, k)| k).collect()
}

/// Class `c`'s pilot sample (module docs): the tiles of `order` ([`tile_order`]) holding `c` land, in that order,
/// until their `c` land reaches `area_ha` and they are at least `min_tiles` (all of them when the class has fewer).
#[must_use]
pub fn calib_tiles(census: &Census, order: &[(i32, i32)], c: usize, area_ha: f64, min_tiles: usize) -> Sample {
    let cell_ha = census.step_m * census.step_m / M2_PER_HA;
    prefix(census, order, c, |land, n| land.cells as f64 * cell_ha >= area_ha && n >= min_tiles)
}

/// The first `n` tiles of `order` holding class `c` land (all of them when the class has fewer): a sized sample.
#[must_use]
pub fn first_tiles(census: &Census, order: &[(i32, i32)], c: usize, n: usize) -> Sample {
    prefix(census, order, c, |_, k| k >= n)
}

/// The tiles of `order` holding `c` land, in that order, until `full(land so far, tiles so far)`.
fn prefix(census: &Census, order: &[(i32, i32)], c: usize, full: impl Fn(&ClassCount, usize) -> bool) -> Sample {
    let cell_ha = census.step_m * census.step_m / M2_PER_HA;
    let (mut tiles, mut land) = (Vec::new(), ClassCount::default());
    for k in order {
        if full(&land, tiles.len()) {
            break;
        }
        let cc = &census.tiles[k].1[c];
        if cc.cells > 0 {
            land.merge(cc);
            tiles.push(*k);
        }
    }
    Sample { tiles, land_ha: land.cells as f64 * cell_ha, weight_mean: land.mean() }
}

/// The tiles holding class `c` land, `N(c)`.
#[must_use]
pub fn class_tiles(census: &Census, c: usize) -> usize {
    census.tiles.values().filter(|(_, per)| per[c].cells > 0).count()
}

/// `CV_r` of the ratio estimator `R = sum y / sum x` over a sample's tiles (module docs "Sample size"): the residual
/// SD `s_d = sqrt(sum (y - R x)^2 / (n - 1))` over `R x_mean`. `None` below two tiles or at `R = 0`.
#[must_use]
pub fn ratio_cv(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len();
    let (sx, sy): (f64, f64) = (x.iter().sum(), y.iter().sum());
    if n < 2 || sx <= 0.0 || sy <= 0.0 {
        return None;
    }
    let r = sy / sx;
    let ss: f64 = x.iter().zip(y).map(|(&xi, &yi)| (yi - r * xi).powi(2)).sum();
    Some((ss / (n - 1) as f64).sqrt() / (r * sx / n as f64))
}

/// The sized sample's tiles (module docs "Sample size"): `n_inf = (z CV_r / tol)^2`, `n = n_inf / (1 + n_inf / N)`
/// rounded up, then at most `min(N, max)` and at least the pilot's `n0`. Also whether `max` cut the formula's `n`.
#[must_use]
pub fn sized_tiles(n0: usize, cv_r: f64, z: f64, tol: f64, class_tiles: usize, max: usize) -> (usize, bool) {
    let n_inf = (z * cv_r / tol).powi(2);
    let n_fpc = n_inf / (1.0 + n_inf / class_tiles as f64);
    // A count of tiles (at most `class_tiles`, a usize).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = n_fpc.ceil().max(0.0) as usize;
    let capped = n > max;
    (n.min(class_tiles.min(max)).max(n0), capped)
}

/// `rse = CV_r sqrt((1 - n / N) / n)`, the relative SE of `reached` on `n` of the class's `N` tiles (0 at `n = N`).
#[must_use]
pub fn relative_se(cv_r: f64, n: usize, class_tiles: usize) -> f64 {
    if n == 0 { 0.0 } else { cv_r * ((1.0 - n as f64 / class_tiles.max(n) as f64) / n as f64).sqrt() }
}

/// The Matern II first guess of class `c` (module docs), before the clamp; `cap` is the class's canopy size cap
/// (the mean height of the capped size law).
fn first_guess(rules: &CandidateRules<'_>, census: &Census, c: usize, target: f64, hi: f64, cap: f64) -> f64 {
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
            let h = (0..SIZE_QUANTILES).map(|i| size_mark_to(sp, (f64::from(i) + 0.5) / q, cap)).sum::<f64>() / q;
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
    let start = std::time::Instant::now();
    let v = &rules.file.values;
    let names = fields.class_names();
    let order = tile_order(census, rules.seed);
    let mut out = Calibration::default();
    let mut its: Vec<Iterate> = Vec::new();
    let mut samples: Vec<(usize, Sample)> = Vec::new();
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
            size_max_m: rules.file.class_size_max.get(name).map(|e| e.v),
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
            let sample = calib_tiles(census, &order, c, v.calib_area_ha, v.calib_tiles_min as usize);
            row.pilot_tiles = u32::try_from(sample.tiles.len()).unwrap_or(u32::MAX);
            row.class_tiles = u32::try_from(class_tiles(census, c)).unwrap_or(u32::MAX);
            let cap = row.size_max_m.unwrap_or(f64::INFINITY);
            let g = first_guess(rules, census, c, target, hi, cap).clamp(lo, hi);
            row.scale = g;
            its.push(Iterate::new(target, lo, hi, g));
            samples.push((c, sample));
        }
        out.classes.push(row);
    }
    let targets = out.targets();
    let (last_y, pilot) = (vec![Vec::new(); samples.len()], vec![(0.0, false); samples.len()]);
    let mut runs = SampleRuns { rules, stand, fields, excluded, census, order: &order, targets, samples, last_y, pilot };
    let checks = drive(&mut its, v.calib_tol, v.calib_saturation, v.calib_iters, &mut runs);
    let SampleRuns { samples, last_y, pilot, .. } = runs;
    let cell_ha = census.step_m * census.step_m / M2_PER_HA;
    for (((it, (c, sample)), y), &(pilot_cv, capped)) in its.iter().zip(&samples).zip(&last_y).zip(&pilot) {
        let row = &mut out.classes[*c];
        // The final sample, `CV_r` over its last run (the last joint check measured every class on it).
        let x: Vec<f64> = sample.tiles.iter().map(|k| census.tiles[k].1[*c].cells as f64 * cell_ha).collect();
        let y: Vec<f64> = y.iter().map(|&n| n as f64).collect();
        let n = sample.tiles.len();
        row.calib_area_ha = sample.land_ha;
        row.calib_tiles = u32::try_from(n).unwrap_or(u32::MAX);
        row.calib_weight_mean = sample.weight_mean;
        row.cv_r = ratio_cv(&x, &y).unwrap_or(0.0);
        row.rse = relative_se(row.cv_r, n, row.class_tiles as usize);
        row.pilot_cv_r = pilot_cv;
        row.capped = capped;
        if row.capped && v.calib_z * row.rse > v.calib_tol {
            out.warnings.push(format!(
                "class {}: sample capped at {n} tiles, reached {:.1} +-{:.1} %",
                row.name,
                it.joint.map_or(0.0, |j| j.1),
                100.0 * v.calib_z * row.rse
            ));
        }
        let (g, w) = it.joint.unwrap_or_else(|| kept(it));
        row.scale = g;
        row.reached_per_ha = w;
        row.steps = it.steps;
        row.ceiling = it.ceiling;
        row.joint_checks = checks;
        row.reopened = it.reopened;
        if it.ceiling {
            out.warnings.push(format!("class {}: target {} above the crown ceiling, reached {w:.1}", row.name, it.target));
        } else if it.converged && (w - it.target).abs() > v.calib_tol * it.target {
            out.warnings.push(format!(
                "class {}: target {} off at the last joint check ({checks} checks), reached {w:.1}",
                row.name, it.target
            ));
        }
    }
    out.seconds = start.elapsed().as_secs_f64();
    out
}

/// The runs [`drive`] asks for.
trait Runs {
    /// `W` of the asked iterates, in order, each class at its `g` of `gs` (one per iterate).
    fn measure(&mut self, gs: &[f64], ask: &[usize]) -> Vec<f64>;

    /// Once, when the per-class iteration first finishes, before the first joint check: the converged classes'
    /// samples may grow (module docs "Sample size").
    fn size(&mut self, _converged: &[bool]) {}
}

/// A plain measurement closure: the samples never grow.
impl<F: FnMut(&[f64], &[usize]) -> Vec<f64>> Runs for F {
    fn measure(&mut self, gs: &[f64], ask: &[usize]) -> Vec<f64> {
        self(gs, ask)
    }
}

/// The calibration's runs: canopy winners on the classes' samples, each class's winners per tile of its last run
/// kept for the sample size.
struct SampleRuns<'a, 'r> {
    rules: &'a CandidateRules<'r>,
    stand: &'a StandRules<'r>,
    fields: &'a dyn Fields,
    excluded: &'a (dyn Fn(f64, f64) -> bool + Sync),
    census: &'a Census,
    order: &'a [(i32, i32)],
    targets: CanopyTargets,
    /// Per iterate: its class id and sample.
    samples: Vec<(usize, Sample)>,
    /// Per iterate: its class's winners per sample tile in its last run.
    last_y: Vec<Vec<u64>>,
    /// Per iterate, set by the sizing: the pilot's `CV_r` (0 when not sized) and whether `calib_tiles_max` cut `n`.
    pilot: Vec<(f64, bool)>,
}

impl Runs for SampleRuns<'_, '_> {
    fn measure(&mut self, gs: &[f64], ask: &[usize]) -> Vec<f64> {
        for (&(c, _), &g) in self.samples.iter().zip(gs) {
            self.targets.classes[c].scale = g;
        }
        let step_rules = CandidateRules { canopy: &self.targets, ..*self.rules };
        let mut tiles: Vec<(i32, i32)> = ask.iter().flat_map(|&i| self.samples[i].1.tiles.iter().copied()).collect();
        tiles.sort_unstable();
        tiles.dedup();
        let n_class = self.fields.class_names().len();
        let won = winners_per_tile(&step_rules, self.stand, self.fields, self.excluded, &tiles, n_class);
        ask.iter()
            .map(|&i| {
                let (c, s) = &self.samples[i];
                let y: Vec<u64> = s.tiles.iter().map(|k| won[k][*c]).collect();
                let n: u64 = y.iter().sum();
                let w = if s.land_ha > 0.0 { n as f64 / s.land_ha } else { 0.0 };
                self.last_y[i] = y;
                w
            })
            .collect()
    }

    fn size(&mut self, converged: &[bool]) {
        let v = &self.rules.file.values;
        let cell_ha = self.census.step_m * self.census.step_m / M2_PER_HA;
        for (i, &conv) in converged.iter().enumerate() {
            let (c, pilot) = &self.samples[i];
            let (c, n0) = (*c, pilot.tiles.len());
            let x: Vec<f64> = pilot.tiles.iter().map(|k| self.census.tiles[k].1[c].cells as f64 * cell_ha).collect();
            let y: Vec<f64> = self.last_y[i].iter().map(|&n| n as f64).collect();
            // Ceiling, out-of-steps and `R = 0` classes keep the pilot.
            let Some(cv) = ratio_cv(&x, &y).filter(|_| conv) else { continue };
            let big = class_tiles(self.census, c);
            let (n, cut) = sized_tiles(n0, cv, v.calib_z, v.calib_tol, big, v.calib_tiles_max as usize);
            self.pilot[i] = (cv, cut);
            if n > n0 {
                self.samples[i].1 = first_tiles(self.census, self.order, c, n);
            }
        }
    }
}

/// The classes' iteration, then the joint checks (module docs): `runs.measure(g per iterate, the iterates asked)`
/// returns `W` of the asked ones, in order, a finished class measured at its kept `g`; `runs.size` once before the
/// first joint check (the sample size). Returns the joint checks run.
fn drive(its: &mut [Iterate], tol: f64, saturation: f64, iters: u32, runs: &mut dyn Runs) -> u32 {
    let mut checks = 0;
    while !its.is_empty() {
        loop {
            let ask: Vec<usize> = (0..its.len()).filter(|&i| !its[i].done).collect();
            if ask.is_empty() {
                break;
            }
            let gs: Vec<f64> = its.iter().map(Iterate::run_g).collect();
            for (&i, w) in ask.iter().zip(runs.measure(&gs, &ask)) {
                step(&mut its[i], w, tol, saturation, iters);
            }
        }
        if checks == 0 {
            runs.size(&its.iter().map(|it| it.converged).collect::<Vec<_>>());
        }
        let gs: Vec<f64> = its.iter().map(Iterate::run_g).collect();
        let all: Vec<usize> = (0..its.len()).collect();
        let ws = runs.measure(&gs, &all);
        checks += 1;
        let mut reopened = false;
        for ((it, &g), w) in its.iter_mut().zip(&gs).zip(ws) {
            it.joint = Some((g, w));
            if !it.converged {
                continue; // ceiling and out-of-steps classes keep their `g`
            }
            if (w - it.target).abs() <= tol * it.target || checks >= iters {
                it.seen.push((g, w)); // the converged class's last (kept) measurement, at the same `g`
                continue;
            }
            it.g = g;
            it.converged = false;
            it.done = false;
            it.reopened += 1;
            step(it, w, tol, saturation, iters);
            reopened = true;
        }
        if !reopened {
            break;
        }
    }
    checks
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
    // The other point is the latest earlier measurement at another `g`: the one before, but for a class a joint
    // check reopened, whose previous measurement was at the same `g`.
    let before = &it.seen[..it.seen.len() - 1];
    let b = match before.iter().rev().find(|&&(g0, _)| g0 != it.g) {
        Some(&(g0, w0)) if w0 > 0.0 => ((w / w0).ln() / (it.g / g0).ln()).clamp(saturation.max(SECANT_B_MIN), 1.0),
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
