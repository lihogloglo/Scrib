//! Nature scatter (spec `worldgen.md` "Pipeline" stage 8; slice WG-X5): flora and rocks placed by rules learned
//! from a source's own placements per region, then replayed deterministically on any terrain.
//!
//! **Learn** ([`Learner`]): per region, per species, the count and density per m² of the region's land, an
//! **occupancy table** over terrain classes at the placements (slope class × height band; the ratio of the
//! species' share of a class to the land's share, Laplace-smoothed toward the land, so 1 means "no preference"),
//! the steepest slope it was seen on, its scale range, how far it tilts toward the ground normal (`align`, from
//! the placements' own up axes) plus the residual tilt, and its height offset above the ground per unit scale.
//!
//! **Scatter** ([`Scatter`]): per world tile (edge `scatter.tile_m`) and per species present in the tile's regions,
//! `floor(λ·keep·A + u)` candidate points from an integer hash of (seed, tile, species, index), so any tile is
//! generated alone, in any order, on any thread count, bitwise equal. Each candidate is accepted by a Bernoulli
//! draw with probability `density · occupancy(class) · clump / λ` (λ the species' largest rate in the tile; clump
//! a value-noise factor of mean 1); a failed placement means absence. Candidates are rejected outside a region
//! the species lives in, on slopes beyond its observed maximum, inside an exclusion (settlement footprints and
//! bands) and on water (final height under sea + margin). Poses replay in the terrain frame: uniform yaw, scale in
//! the learned range, up axis blended toward the ground normal by `align` plus the learned jitter.
//!
//! **Budget** ([`budget_keep`]): the learned density is kept per m² (the look); a global count budget thins each
//! species by `keep = min(1, t·size^p)` (large species first), one `t` per group (flora, rocks) so each group
//! keeps its share. Raising the budget only adds candidates (`n` grows with `keep`, acceptance doesn't depend on
//! it), so a 1 M run contains the 200 k run.
//!
//! Industry: rule-based procedural placement from density × terrain-attribute rules is Horizon Zero Dawn's
//! (Guerrilla, "GPU-based procedural placement in Horizon Zero Dawn", GDC 2017) and UE5 PCG's density filters;
//! learning the rules from an existing map's own placements per region follows the WG-R3 prior art (Procedural
//! Tamriel's occupancy tables). Counter-based hashing (splitmix64, Steele et al. 2014) gives the per-tile
//! independence Horizon gets from evaluating placement per tile on the GPU. Nothing here names a game.

use std::collections::BTreeMap;

use rayon::prelude::*;

use crate::filter::Declared;
use crate::perturb::value_noise;
use crate::scatter_fields::{Fields, Suitability};
use crate::scatter_species::{Layer, SizeLaw, Species as SpeciesRow, SpeciesFile};
use crate::scatter_stand::{StandClass, StandMap};

#[cfg(test)]
#[path = "tests_scatter.rs"]
mod tests;

#[cfg(test)]
#[path = "tests_scatter_candidates.rs"]
mod tests_candidates;

#[cfg(test)]
#[path = "tests_scatter_competition.rs"]
mod tests_competition;

/// One-degree slope bins of the area census (0..90°).
// tunables: const (layout: one bin per degree of slope, the census resolution of the slope cut)
pub const SLOPE_BINS: usize = 90;

/// The declared scatter values (spec `tunables.md` "Definition": value, unit, range, source).
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("scatter.tile_m", 256.0, "m", 16.0, 4096.0, "Guess",
          "edge of a scatter tile: candidates are hashed per tile, so any tile is generated alone"),
        d("scatter.seed", 55.0, "count", 0.0, 4.294_967_295e9, "Guess", "seed of every scatter hash and clump noise"),
        d("scatter.slope_class_deg", 5.0, "deg", 1.0, 30.0, "Guess", "width of one slope class of the occupancy tables"),
        d("scatter.slope_classes", 10.0, "count", 1.0, 90.0, "Guess",
          "slope classes; the last one holds every steeper slope (10 x 5 deg: 45 deg and up)"),
        d("scatter.band_base_m", 2.0, "m", 0.1, 100.0, "Guess", "top of the lowest height band (source-equivalent metres)"),
        d("scatter.band_ratio", 2.0, "ratio", 1.1, 10.0, "Guess", "each height band is this many times the one below"),
        d("scatter.bands", 9.0, "count", 1.0, 32.0, "Guess", "height bands; the last one holds every higher sample"),
        d("scatter.occupancy_prior", 2.0, "count", 0.0, 100.0, "Industry(Laplace smoothing of a categorical estimate)",
          "pseudo-placements spread like the land: a species seen a few times doesn't own a class"),
        d("scatter.occupancy_max", 20.0, "ratio", 1.0, 1000.0, "Guess",
          "upper bound of an occupancy ratio (a species' class share over the land's)"),
        d("scatter.align_min_tilt_deg", 8.0, "deg", 1.0, 45.0, "Guess",
          "placements on ground flatter than this don't vote on how far a species tilts with the ground"),
        d("scatter.slope_margin_deg", 1.0, "deg", 0.0, 10.0, "Guess",
          "a placement may stand this much steeper than the steepest of its species seen in the source (the target's            slope differs from the source's by reshaping and rounding)"),
        d("scatter.clump_wavelength_m", 48.0, "m", 4.0, 2000.0, "Guess", "lattice spacing of the clumping value noise"),
        d("scatter.clump_strength", 0.6, "ratio", 0.0, 1.0, "Guess",
          "clumping: acceptance x (1 + k noise), noise in [-1, 1] of mean 0, so the mean density is kept"),
        d("scatter.water_margin_m", 0.3, "m", 0.0, 10.0, "Guess", "no placement on ground lower than sea level + this"),
        d("scatter.budget", 200_000.0, "count", 0.0, 1.0e8, "Owner(2026-11-18: counts raised in steps, 50k, 200k, 1M+)",
          "global placement budget; each group keeps its share, large species first; full density when above it"),
        d("scatter.size_priority", 1.0, "ratio", 0.0, 4.0, "Guess",
          "budget keep grows as size^p: 0 thins every species alike, 1 keeps a 10 m tree 10x more than a 1 m shrub"),
        d("scatter.census_step_m", 36.0, "m", 1.0, 1000.0, "Guess",
          "spacing of the area census that predicts the full-density count (and so the budget's thinning)"),
    ]
}

/// The scatter values ([`declared`]), plus the sea level the caller takes from its terrain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub tile_m: f64,
    pub seed: u64,
    pub slope_class_deg: f32,
    pub slope_classes: usize,
    pub band_base_m: f32,
    pub band_ratio: f32,
    pub bands: usize,
    pub occupancy_prior: f64,
    pub occupancy_max: f32,
    pub align_min_tilt_deg: f32,
    pub slope_margin_deg: f32,
    pub clump_wavelength_m: f64,
    pub clump_strength: f64,
    pub water_margin_m: f64,
    pub budget: f64,
    pub size_priority: f64,
    pub census_step_m: f64,
    /// Sea level of the terrain, metres (a fact of the terrain, not a tunable; 0 by default).
    pub sea_level_m: f64,
}

impl Default for Params {
    fn default() -> Self {
        let v = |name: &str| declared().into_iter().find(|d| d.name == name).map_or(f64::NAN, |d| d.v);
        Self {
            tile_m: v("scatter.tile_m"),
            seed: v("scatter.seed") as u64,
            slope_class_deg: v("scatter.slope_class_deg") as f32,
            slope_classes: v("scatter.slope_classes") as usize,
            band_base_m: v("scatter.band_base_m") as f32,
            band_ratio: v("scatter.band_ratio") as f32,
            bands: v("scatter.bands") as usize,
            occupancy_prior: v("scatter.occupancy_prior"),
            occupancy_max: v("scatter.occupancy_max") as f32,
            align_min_tilt_deg: v("scatter.align_min_tilt_deg") as f32,
            slope_margin_deg: v("scatter.slope_margin_deg") as f32,
            clump_wavelength_m: v("scatter.clump_wavelength_m"),
            clump_strength: v("scatter.clump_strength"),
            water_margin_m: v("scatter.water_margin_m"),
            budget: v("scatter.budget"),
            size_priority: v("scatter.size_priority"),
            census_step_m: v("scatter.census_step_m"),
            sea_level_m: 0.0, // tunables: const (a fact of the terrain the host passes, not a tunable)
        }
    }
}

impl Params {
    /// Terrain classes of an occupancy table.
    pub fn classes(&self) -> usize {
        self.slope_classes * self.bands
    }

    pub fn slope_class(&self, slope_deg: f32) -> usize {
        ((slope_deg.max(0.0) / self.slope_class_deg) as usize).min(self.slope_classes - 1)
    }

    /// Band 0 below `band_base_m`, then one band per factor `band_ratio`, the last open.
    pub fn band(&self, h_m: f32) -> usize {
        if h_m.is_nan() || h_m < self.band_base_m {
            return 0;
        }
        (1 + ((h_m / self.band_base_m).ln() / self.band_ratio.ln()) as usize).min(self.bands - 1)
    }

    pub fn class(&self, slope_deg: f32, band_m: f32) -> usize {
        self.slope_class(slope_deg) * self.bands + self.band(band_m)
    }
}

/// A species: one model of the source (the caller keeps what it maps to).
#[derive(Clone, Debug, PartialEq)]
pub struct Species {
    pub name: String,
    /// Largest edge of the model at its typical scale, metres (the budget's priority).
    pub size_m: f32,
    /// The caller's group (flora, rocks): each group keeps its share of the budget.
    pub group: u8,
}

/// One species in one region: its learned rules.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub species: u32,
    /// Placements seen in the region's source.
    pub count: usize,
    /// Per m² of the region's land.
    pub density_m2: f64,
    /// Per terrain class ([`Params::class`]): the species' share over the land's (1 = no preference), 0 on
    /// classes entirely steeper than `max_slope_deg`.
    pub occupancy: Vec<f32>,
    /// Steepest slope a placement was seen on, degrees.
    pub max_slope_deg: f32,
    /// Scale range seen.
    pub scale: [f32; 2],
    /// 0 = upright, 1 = along the ground normal.
    pub align: f32,
    /// Median residual tilt from the aligned up axis, degrees.
    pub tilt_jitter_deg: f32,
    /// Median height of the origin above the ground, metres per unit scale (negative: sunk).
    pub offset_m: f32,
}

impl Entry {
    fn occupancy_max(&self) -> f32 {
        self.occupancy.iter().copied().fold(0.0, f32::max)
    }
}

/// Learned rules: species and, per region index, its entries (ascending species).
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub species: Vec<Species>,
    pub regions: Vec<Vec<Entry>>,
}

/// One source placement, measured on the source's own terrain. Frame: x east, y north, z up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Observed {
    pub species: u32,
    pub region: u16,
    pub slope_deg: f32,
    /// The height the bands classify.
    pub band_m: f32,
    pub scale: f32,
    /// The placement's own up axis (unit).
    pub up: [f32; 3],
    /// The ground normal under it (unit).
    pub normal: [f32; 3],
    /// Origin height above the ground, metres.
    pub offset_m: f32,
}

/// Collects a source's land classes and placements; [`Learner::finish`] builds the [`Table`].
pub struct Learner {
    prm: Params,
    /// Per region, land samples per class.
    land: Vec<Vec<u64>>,
    land_sample_m2: f64,
    obs: Vec<Observed>,
}

fn median(v: &mut [f32]) -> f32 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(f32::total_cmp);
    let m = v.len() / 2;
    if v.len() % 2 == 1 { v[m] } else { f32::midpoint(v[m - 1], v[m]) }
}

fn norm3(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 0.0 { [v[0] / l, v[1] / l, v[2] / l] } else { [0.0, 0.0, 1.0] }
}

fn angle_deg(a: [f32; 3], b: [f32; 3]) -> f32 {
    (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]).clamp(-1.0, 1.0).acos().to_degrees()
}

/// The up axis `align` of the way from vertical to `normal`.
pub fn aligned_up(normal: [f32; 3], align: f32) -> [f32; 3] {
    let a = align.clamp(0.0, 1.0);
    norm3([a * normal[0], a * normal[1], (1.0 - a) + a * normal[2]])
}

impl Learner {
    /// `regions`: region indices are `0..regions`; `land_sample_m2`: the area one [`Learner::add_land`] stands for.
    pub fn new(prm: Params, regions: usize, land_sample_m2: f64) -> Self {
        Self { prm, land: vec![vec![0; prm.classes()]; regions], land_sample_m2, obs: Vec::new() }
    }

    /// One land sample of `region`.
    pub fn add_land(&mut self, region: u16, slope_deg: f32, band_m: f32) {
        let c = self.prm.class(slope_deg, band_m);
        if let Some(r) = self.land.get_mut(usize::from(region)) {
            r[c] += 1;
        }
    }

    pub fn add(&mut self, o: Observed) {
        self.obs.push(o);
    }

    /// The table: one entry per (region, species) seen on land the region has.
    pub fn finish(self, species: Vec<Species>) -> Table {
        let prm = self.prm;
        let mut groups: BTreeMap<(u16, u32), Vec<Observed>> = BTreeMap::new();
        for o in self.obs {
            groups.entry((o.region, o.species)).or_default().push(o);
        }
        let mut regions = vec![Vec::new(); self.land.len()];
        for ((r, s), obs) in groups {
            let Some(land) = self.land.get(usize::from(r)) else { continue };
            let total: u64 = land.iter().sum();
            if total == 0 {
                continue;
            }
            let n = obs.len();
            let mut counts = vec![0u64; prm.classes()];
            for o in &obs {
                counts[prm.class(o.slope_deg, o.band_m)] += 1;
            }
            let max_slope = obs.iter().map(|o| o.slope_deg).fold(0.0, f32::max);
            // Classes whose lower edge is above the steepest placement are never occupied.
            let first_too_steep = ((max_slope + prm.slope_margin_deg) / prm.slope_class_deg).floor() as usize + 1;
            let alpha = prm.occupancy_prior;
            let occupancy = (0..prm.classes())
                .map(|c| {
                    let share = land[c] as f64 / total as f64;
                    if share <= 0.0 || c / prm.bands >= first_too_steep {
                        return 0.0;
                    }
                    let seen = (counts[c] as f64 + alpha * share) / (n as f64 + alpha);
                    ((seen / share) as f32).min(prm.occupancy_max)
                })
                .collect();
            let mut votes: Vec<f32> = obs
                .iter()
                .filter(|o| angle_deg(o.normal, [0.0, 0.0, 1.0]) >= prm.align_min_tilt_deg)
                .map(|o| {
                    let nh2 = o.normal[0] * o.normal[0] + o.normal[1] * o.normal[1];
                    (o.up[0] * o.normal[0] + o.up[1] * o.normal[1]) / nh2.max(1e-12)
                })
                .collect();
            // `+ 0.0` turns a clamped −0 into 0.
            let align = median(&mut votes).clamp(0.0, 1.0) + 0.0;
            let mut jitter: Vec<f32> = obs.iter().map(|o| angle_deg(o.up, aligned_up(o.normal, align))).collect();
            let mut offsets: Vec<f32> = obs.iter().map(|o| o.offset_m / o.scale.max(1e-3)).collect();
            let scale = obs.iter().fold([f32::INFINITY, f32::NEG_INFINITY], |a, o| [a[0].min(o.scale), a[1].max(o.scale)]);
            regions[usize::from(r)].push(Entry {
                species: s,
                count: n,
                density_m2: n as f64 / (total as f64 * self.land_sample_m2),
                occupancy,
                max_slope_deg: max_slope,
                scale,
                align,
                tilt_jitter_deg: median(&mut jitter),
                offset_m: median(&mut offsets),
            });
        }
        Table { species, regions }
    }
}

/// What a terrain answers at a point (x east, y north, metres).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Site {
    pub slope_deg: f32,
    /// The height the bands classify (the learner's convention: e.g. a source-equivalent height).
    pub band_m: f32,
    /// Unit ground normal, z up.
    pub normal: [f32; 3],
}

/// The terrain a scatter runs on.
pub trait Terrain: Sync {
    /// Every region a point of `[x0, y0, x1, y1]` can have (a superset is fine, a missing one drops its species).
    fn regions_in(&self, rect: [f64; 4], out: &mut Vec<u16>);
    fn region(&self, x: f64, y: f64) -> Option<u16>;
    /// Slope, band height and normal; `None` where nothing may grow (outside the terrain).
    fn site(&self, x: f64, y: f64) -> Option<Site>;
    /// Inside an exclusion (settlement footprints and their bands).
    fn excluded(&self, x: f64, y: f64) -> bool;
    /// The final ground height, metres.
    fn height(&self, x: f64, y: f64) -> f64;
}

/// One placement. Frame: x east, y north, z up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    pub species: u32,
    pub region: u16,
    pub pos: [f64; 3],
    pub yaw: f32,
    pub scale: f32,
    /// The object's up axis (unit).
    pub up: [f32; 3],
}

/// Land samples per region × (one-degree slope bin, band) of a terrain, dry and not excluded.
#[derive(Clone, Debug, PartialEq)]
pub struct AreaBins {
    pub bands: usize,
    pub sample_m2: f64,
    /// `[region][slope_bin * bands + band]`.
    pub counts: Vec<Vec<u64>>,
}

/// The area census of `terrain` over `rect` at `prm.census_step_m` (sample centres), rows in parallel.
pub fn area_census(terrain: &dyn Terrain, prm: &Params, rect: [f64; 4], regions: usize) -> AreaBins {
    let step = prm.census_step_m;
    let nx = ((rect[2] - rect[0]) / step).floor().max(0.0) as usize;
    let ny = ((rect[3] - rect[1]) / step).floor().max(0.0) as usize;
    let keys: Vec<Vec<(u16, u32)>> = (0..ny)
        .into_par_iter()
        .map(|j| {
            let y = rect[1] + (j as f64 + 0.5) * step;
            (0..nx)
                .filter_map(|i| {
                    let x = rect[0] + (i as f64 + 0.5) * step;
                    let r = terrain.region(x, y)?;
                    let s = terrain.site(x, y)?;
                    if terrain.excluded(x, y) || terrain.height(x, y) < prm.sea_level_m + prm.water_margin_m {
                        return None;
                    }
                    let bin = (s.slope_deg.max(0.0) as usize).min(SLOPE_BINS - 1);
                    Some((r, (bin * prm.bands + prm.band(s.band_m)) as u32))
                })
                .collect()
        })
        .collect();
    let mut counts = vec![vec![0u64; SLOPE_BINS * prm.bands]; regions];
    for (r, k) in keys.into_iter().flatten() {
        if let Some(c) = counts.get_mut(usize::from(r)) {
            c[k as usize] += 1;
        }
    }
    AreaBins { bands: prm.bands, sample_m2: step * step, counts }
}

/// Full-density expected count per `[region][entry]` over `area` (clump mean 1).
pub fn expected(table: &Table, prm: &Params, area: &AreaBins) -> Vec<Vec<f64>> {
    table
        .regions
        .iter()
        .enumerate()
        .map(|(r, entries)| {
            let Some(counts) = area.counts.get(r) else { return vec![0.0; entries.len()] };
            entries
                .iter()
                .map(|e| {
                    let mut sum = 0.0;
                    for (k, &n) in counts.iter().enumerate() {
                        if n == 0 {
                            continue;
                        }
                        let (bin, band) = (k / area.bands, k % area.bands);
                        let slope = bin as f32 + 0.5;
                        if slope > e.max_slope_deg + prm.slope_margin_deg + 0.5 {
                            continue;
                        }
                        let c = prm.slope_class(slope) * prm.bands + band;
                        sum += f64::from(e.occupancy[c]) * n as f64;
                    }
                    e.density_m2 * sum * area.sample_m2
                })
                .collect()
        })
        .collect()
}

/// Per species, the share kept under `prm.budget` ([module docs](self): one `t` per group, `min(1, t·size^p)`).
pub fn budget_keep(table: &Table, prm: &Params, expected: &[Vec<f64>]) -> Vec<f32> {
    let ns = table.species.len();
    let mut full = vec![0.0f64; ns];
    for (entries, exp) in table.regions.iter().zip(expected) {
        for (e, &n) in entries.iter().zip(exp) {
            full[e.species as usize] += n;
        }
    }
    let total: f64 = full.iter().sum();
    let mut keep = vec![1.0f32; ns];
    if total <= prm.budget || total <= 0.0 {
        return keep;
    }
    let weight: Vec<f64> = table.species.iter().map(|s| f64::from(s.size_m.max(0.1)).powf(prm.size_priority)).collect();
    let mut groups: Vec<u8> = table.species.iter().map(|s| s.group).collect();
    groups.sort_unstable();
    groups.dedup();
    for g in groups {
        let members: Vec<usize> = (0..ns).filter(|&s| table.species[s].group == g).collect();
        let n_g: f64 = members.iter().map(|&s| full[s]).sum();
        let target = prm.budget * n_g / total;
        let kept = |t: f64| members.iter().map(|&s| (t * weight[s]).min(1.0) * full[s]).sum::<f64>();
        let (mut lo, mut hi) = (0.0f64, members.iter().map(|&s| 1.0 / weight[s]).fold(0.0, f64::max));
        for _ in 0..100 {
            let mid = f64::midpoint(lo, hi);
            if kept(mid) < target { lo = mid } else { hi = mid }
        }
        for &s in &members {
            keep[s] = (hi * weight[s]).min(1.0) as f32;
        }
    }
    keep
}

/// splitmix64 finaliser (Steele, Lea, Flood, OOPSLA 2014).
#[inline]
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[inline]
fn unit(h: u64) -> f64 {
    (h >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

/// The hash stream of (seed, tile, species).
fn stream(seed: u64, tx: i32, ty: i32, species: u32) -> u64 {
    let a = mix(seed ^ 0x5ca7_7e40_0000_0001);
    let b = mix(a ^ i64::from(tx).cast_unsigned().wrapping_mul(0x9e37_79b9_7f4a_7c15));
    let c = mix(b ^ i64::from(ty).cast_unsigned().wrapping_mul(0xc2b2_ae3d_27d4_eb4f));
    mix(c ^ u64::from(species).wrapping_mul(0x1656_67b1_9e37_79f9))
}

/// Draw `k` of candidate `i` of a stream, in [0, 1).
#[inline]
fn draw(base: u64, i: u64, k: u64) -> f64 {
    unit(mix(base ^ (i.wrapping_mul(16).wrapping_add(k).wrapping_add(1)).wrapping_mul(0x9e37_79b9_7f4a_7c15)))
}

/// A scatter of a [`Table`] with per-species keep shares ([`budget_keep`]).
pub struct Scatter<'a> {
    table: &'a Table,
    keep: Vec<f32>,
    prm: Params,
    /// `[region][species]` → entry index + 1 (0: the species doesn't live there).
    lookup: Vec<Vec<u32>>,
    /// `[region][entry]`: density × largest occupancy, per m².
    rate: Vec<Vec<f64>>,
}

impl<'a> Scatter<'a> {
    pub fn new(table: &'a Table, keep: Vec<f32>, prm: Params) -> Self {
        let ns = table.species.len();
        let lookup = table
            .regions
            .iter()
            .map(|entries| {
                let mut l = vec![0u32; ns];
                for (k, e) in entries.iter().enumerate() {
                    l[e.species as usize] = k as u32 + 1;
                }
                l
            })
            .collect();
        let rate = table
            .regions
            .iter()
            .map(|entries| entries.iter().map(|e| e.density_m2 * f64::from(e.occupancy_max())).collect())
            .collect();
        Self { table, keep, prm, lookup, rate }
    }

    fn entry(&self, region: u16, species: u32) -> Option<&Entry> {
        let k = *self.lookup.get(usize::from(region))?.get(species as usize)?;
        (k > 0).then(|| &self.table.regions[usize::from(region)][k as usize - 1])
    }

    /// The tiles covering `[x0, y0, x1, y1]`, row by row (south to north), west to east.
    pub fn tiles(&self, rect: [f64; 4]) -> Vec<(i32, i32)> {
        let t = self.prm.tile_m;
        let (tx0, ty0) = ((rect[0] / t).floor() as i32, (rect[1] / t).floor() as i32);
        let (tx1, ty1) = ((rect[2] / t).floor() as i32, (rect[3] / t).floor() as i32);
        (ty0..=ty1).flat_map(|ty| (tx0..=tx1).map(move |tx| (tx, ty))).collect()
    }

    /// Tile `(tx, ty)`'s placements (it spans `[tx·t, (tx+1)·t) × [ty·t, (ty+1)·t)`), appended to `out`.
    pub fn tile(&self, terrain: &dyn Terrain, (tx, ty): (i32, i32), out: &mut Vec<Item>) {
        let prm = &self.prm;
        let t = prm.tile_m;
        let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
        let mut regions = Vec::new();
        terrain.regions_in([x0, y0, x0 + t, y0 + t], &mut regions);
        regions.sort_unstable();
        regions.dedup();
        regions.retain(|&r| usize::from(r) < self.table.regions.len());
        let mut species: Vec<u32> =
            regions.iter().flat_map(|&r| self.table.regions[usize::from(r)].iter().map(|e| e.species)).collect();
        species.sort_unstable();
        species.dedup();
        let k = prm.clump_strength;
        for s in species {
            let keep = f64::from(self.keep.get(s as usize).copied().unwrap_or(0.0));
            if keep <= 0.0 {
                continue;
            }
            let lam = regions
                .iter()
                .filter_map(|&r| {
                    let e = self.lookup[usize::from(r)][s as usize];
                    (e > 0).then(|| self.rate[usize::from(r)][e as usize - 1])
                })
                .fold(0.0, f64::max)
                * (1.0 + k);
            if lam <= 0.0 {
                continue;
            }
            let base = stream(prm.seed, tx, ty, s);
            let n = (lam * keep * t * t + unit(mix(base))).floor() as u64;
            let clump_seed = mix(prm.seed ^ u64::from(s).wrapping_mul(0xd6e8_feb8_6659_fd93));
            for i in 0..n {
                let (x, y) = (x0 + draw(base, i, 0) * t, y0 + draw(base, i, 1) * t);
                let Some(r) = terrain.region(x, y) else { continue };
                let Some(e) = self.entry(r, s) else { continue };
                let Some(site) = terrain.site(x, y) else { continue };
                if site.slope_deg > e.max_slope_deg + prm.slope_margin_deg {
                    continue;
                }
                let occ = f64::from(e.occupancy[prm.class(site.slope_deg, site.band_m)]);
                let clump = 1.0 + k * value_noise([x, y], prm.clump_wavelength_m, clump_seed);
                if draw(base, i, 2) >= e.density_m2 * occ * clump / lam {
                    continue;
                }
                if terrain.excluded(x, y) {
                    continue;
                }
                let h = terrain.height(x, y);
                if h < prm.sea_level_m + prm.water_margin_m {
                    continue;
                }
                let scale = e.scale[0] + (e.scale[1] - e.scale[0]) * draw(base, i, 3) as f32;
                let up = tilt(aligned_up(site.normal, e.align), e.tilt_jitter_deg * 2.0 * draw(base, i, 4) as f32, draw(base, i, 5));
                out.push(Item {
                    species: s,
                    region: r,
                    pos: [x, y, h + f64::from(e.offset_m * scale)],
                    yaw: (draw(base, i, 6) * std::f64::consts::TAU) as f32,
                    scale,
                    up,
                });
            }
        }
    }

    /// Every tile over `rect` (whole tiles), in [`Scatter::tiles`] order; tiles in parallel, the result the
    /// same on any thread count.
    pub fn run(&self, terrain: &dyn Terrain, rect: [f64; 4]) -> Vec<Item> {
        self.tiles(rect)
            .into_par_iter()
            .map(|c| {
                let mut v = Vec::new();
                self.tile(terrain, c, &mut v);
                v
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect()
    }
}

/// `up` tilted by `deg` toward the azimuth `turn` (of a full turn) around it.
fn tilt(up: [f32; 3], deg: f32, turn: f64) -> [f32; 3] {
    if deg <= 0.0 {
        return up;
    }
    // A tangent basis of `up`.
    let helper = if up[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let t1 = norm3([
        up[1] * helper[2] - up[2] * helper[1],
        up[2] * helper[0] - up[0] * helper[2],
        up[0] * helper[1] - up[1] * helper[0],
    ]);
    let t2 = [up[1] * t1[2] - up[2] * t1[1], up[2] * t1[0] - up[0] * t1[2], up[0] * t1[1] - up[1] * t1[0]];
    let (sp, cp) = ((turn * std::f64::consts::TAU) as f32).sin_cos();
    let (st, ct) = deg.to_radians().sin_cos();
    norm3([0, 1, 2].map(|k| up[k] * ct + (t1[k] * cp + t2[k] * sp) * st))
}

// ---------------------------------------------------------------------------------------------------------------
// Scatter v2 candidates (spec `worldgen-scatter.md` "Design" 4-5 and "Determinism across tiles"; slice WG-E2d).
//
// Per species and per parent tile (edge `scatter.tile_m`), a truncated Thomas process: `n = floor(kappa_max A + u)`
// parents from the hash stream of (seed, parent tile, species), thinned at the parent by `kappa / kappa_max`
// (Lewis and Shedler 1979), `kappa = (1 - b) parent_per_ha x suitability x stand x prior`; each kept parent has
// Poisson(`offspring_mean x suitability`) offspring by inversion, offset by a Gaussian of `cluster_sigma_m`
// (Box-Muller) and rejected beyond `THOMAS_REACH_SIGMAS` sigma, so a parent's reach is exact. A uniform background
// stream per parent tile carries the share `b = background_fraction` of the candidates, thinned at the point by the
// same factors (suitability squared: the cluster's parent and offspring factors). `stand` is the stand density `D`
// for the canopy (0 in a clearing, forced or mosaic; a canopy candidate in a clearing is dropped) and 1 for lower
// layers (their light factor is WG-E2f's). `prior` is the caller's region prior, clamped to
// `[0, scatter.region_prior_max]`. Each candidate gets a size mark by inversion ([`size_mark`]; canopy quantiles
// scaled by `scatter.edge_size_ratio` in the edge band), its crown radius `crown_a_m + crown_b h`, and a tie-break
// hash for the competition's priority (size mark, then hash).
//
// Determinism: a candidate is a pure function of its own streams, the fields and the stand at its point; a rect
// keeps the candidates inside it, from every parent tile within the reach of a parent, so any covering build
// returns the same candidates over the rect, bit for bit. A tile's rect is its square grown by the competition
// reach `(2k - 1) R_max` ([`CandidateRules::tile_rect`]); its parents come from `3 sigma` further, the spec's
// halo `H = 3 sigma_max + (2k - 1) R_max` ([`CandidateRules::halo_m`]).

/// The parent index of a background candidate in its [`CandidateKey`].
// tunables: const (format: key marker of the background stream)
pub const BACKGROUND: u32 = u32::MAX;

/// Offspring beyond this many cluster sigmas are rejected (spec "Design" 4: the truncated Thomas process).
// tunables: const (maths: the truncation that defines the process and its exact reach, spec "Design" 4)
pub const THOMAS_REACH_SIGMAS: f64 = 3.0;

/// Salts of the v2 parent and background streams in the scatter seed's hash space.
// tunables: const (hash salt, not a value)
const PARENT_SALT: u64 = 0x7a0e_2d00_0000_0e2d;
// tunables: const (hash salt, not a value)
const BACKGROUND_SALT: u64 = 0xb6c0_2d00_0000_1e2d;

/// Square metres per hectare.
// tunables: const (units)
const M2_PER_HA: f64 = 10_000.0;

/// The integer key of a candidate; output order (spec "Determinism across tiles").
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CandidateKey {
    pub layer: Layer,
    /// Index of the species in its file.
    pub species: u32,
    /// The parent tile.
    pub tile: (i32, i32),
    /// Parent index in the tile, or [`BACKGROUND`].
    pub parent: u32,
    /// Offspring index of the parent, or the background point's index.
    pub offspring: u32,
}

/// One candidate before competition (spec "Design" 4-5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Candidate {
    pub key: CandidateKey,
    /// Position, m (x east, y north).
    pub pos: [f64; 2],
    /// Size mark (height), m.
    pub size_m: f64,
    /// Crown radius `crown_a_m + crown_b size`, m.
    pub crown_m: f64,
    /// Second key of the competition priority (after the size mark).
    pub tiebreak: u64,
}

/// The size mark of quantile `u` in `[0, 1)` (spec "Design" 5), m, in `[size_min_m, size_max_m]`. Reverse J: an
/// exponential in height truncated to the range, of rate `size_shape / (size_max_m - size_min_m)` (so `size_shape`
/// is the e-folds of the count across the range). Weibull: shape `size_shape`, scale the range, truncated at it.
#[must_use]
pub fn size_mark(sp: &SpeciesRow, u: f64) -> f64 {
    let (a, b) = (sp.size_min_m, sp.size_max_m);
    let range = b - a;
    if range <= 0.0 {
        return a;
    }
    let k = sp.size_shape;
    let x = match sp.size_law {
        SizeLaw::ReverseJ => -(1.0 - u * (1.0 - (-k).exp())).ln() * range / k,
        // Maths: F(range) = 1 - exp(-1) at scale = range.
        SizeLaw::Weibull => range * (-(1.0 - u * (1.0 - (-1.0f64).exp())).ln()).powf(1.0 / k),
    };
    (a + x).clamp(a, b)
}

/// Poisson(`mu`) by inversion of one uniform `u` (bounded: far past the tail the count stops).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a bound of a few hundred at most
fn poisson(mu: f64, u: f64) -> u32 {
    if mu <= 0.0 {
        return 0;
    }
    // Maths: mu + 20 sd + 20 is past any tail mass an f64 uniform can reach.
    let cap = (mu + 20.0 * mu.sqrt() + 20.0).ceil() as u32;
    let mut p = (-mu).exp();
    let mut f = p;
    let mut k = 0u32;
    while u >= f && k < cap {
        k += 1;
        p *= mu / f64::from(k);
        f += p;
    }
    k
}

/// Hash `k` of item `i` of a stream (the raw bits [`draw`] maps to `[0, 1)`).
#[inline]
fn hash_k(base: u64, i: u64, k: u64) -> u64 {
    mix(base ^ (i.wrapping_mul(16).wrapping_add(k).wrapping_add(1)).wrapping_mul(0x9e37_79b9_7f4a_7c15))
}

/// The rules of the v2 candidates.
pub struct CandidateRules<'a> {
    /// Values (`[params]`) and species rows.
    pub file: &'a SpeciesFile,
    /// One suitability per species of `file`, in order.
    pub suitability: &'a [Suitability],
    /// `scatter.seed`.
    pub seed: u64,
    /// `scatter.tile_m`: the parent tiles.
    pub tile_m: f64,
    /// Region prior on parent intensity at (species index, x, y), clamped to `[0, scatter.region_prior_max]`.
    pub prior: &'a (dyn Fn(usize, f64, f64) -> f64 + Sync),
}

impl CandidateRules<'_> {
    /// `R_max`: the largest conflict distance, `max crown_k x 2 (crown_a_m + crown_b size_max_m)`, m.
    #[must_use]
    pub fn r_max_m(&self) -> f64 {
        self.file.species.iter().map(|s| s.crown_k * 2.0 * (s.crown_a_m + s.crown_b * s.size_max_m)).fold(0.0, f64::max)
    }

    /// The competition reach `(2k - 1) R_max`, m (spec "Determinism across tiles", D).
    #[must_use]
    pub fn competition_reach_m(&self) -> f64 {
        (2.0 * f64::from(self.file.values.competition_rounds) - 1.0) * self.r_max_m()
    }

    /// The halo `H = 3 sigma_max + (2k - 1) R_max`, m: how far past its square a tile rebuilds parents.
    #[must_use]
    pub fn halo_m(&self) -> f64 {
        let sigma = self.file.species.iter().map(|s| s.cluster_sigma_m).fold(0.0, f64::max);
        THOMAS_REACH_SIGMAS * sigma + self.competition_reach_m()
    }

    /// The square of tile `(tx, ty)` grown by the competition reach: the rect its candidates (and its stand map)
    /// cover.
    #[must_use]
    pub fn tile_rect(&self, (tx, ty): (i32, i32)) -> [f64; 4] {
        let (t, g) = (self.tile_m, self.competition_reach_m());
        let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
        [x0 - g, y0 - g, x0 + t + g, y0 + t + g]
    }

    /// Every candidate inside `rect = [x0, y0, x1, y1]` (half open), sorted by key. `stand` must cover `rect`
    /// (built over it or a larger rect).
    #[must_use]
    pub fn candidates(&self, fields: &dyn Fields, stand: &StandMap, rect: [f64; 4]) -> Vec<Candidate> {
        let mut out = Vec::new();
        for (s, sp) in self.file.species.iter().enumerate() {
            self.species_candidates(fields, stand, rect, s, sp, &mut out);
        }
        out.sort_unstable_by_key(|c| c.key);
        out
    }

    /// The candidates of one species (unsorted).
    // Tile indices of world metres and hashed counts of a few thousand per tile.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn species_candidates(
        &self,
        fields: &dyn Fields,
        stand: &StandMap,
        rect: [f64; 4],
        s: usize,
        sp: &SpeciesRow,
        out: &mut Vec<Candidate>,
    ) {
        let v = &self.file.values;
        let suit = &self.suitability[s];
        let canopy = sp.layer == Layer::Canopy;
        let t = self.tile_m;
        let area = t * t;
        let pmax = v.region_prior_max;
        let b = sp.background_fraction;
        let sigma = sp.cluster_sigma_m;
        let reach = THOMAS_REACH_SIGMAS * sigma;
        let inside = |x: f64, y: f64| x >= rect[0] && x < rect[2] && y >= rect[1] && y < rect[3];
        // (stand factor, edge band) at a point; `None` drops a canopy candidate in a clearing.
        let stand_at = |x: f64, y: f64| -> Option<(f64, bool)> {
            if !canopy {
                return Some((1.0, false));
            }
            let p = stand.at(x, y);
            (p.class != StandClass::Clearing).then_some((p.d, p.edge_band))
        };
        let prior = |x: f64, y: f64| (self.prior)(s, x, y).clamp(0.0, pmax) / pmax;
        let mut emit = |key: CandidateKey, x: f64, y: f64, edge: bool, u: f64, tiebreak: u64| {
            let q = if edge { u * v.edge_size_ratio } else { u };
            let size_m = size_mark(sp, q);
            out.push(Candidate { key, pos: [x, y], size_m, crown_m: sp.crown_a_m + sp.crown_b * size_m, tiebreak });
        };
        let species = s as u32;
        let kappa_max = (1.0 - b) * sp.parent_per_ha / M2_PER_HA * pmax;
        let bg_max = b * sp.parent_per_ha / M2_PER_HA * sp.offspring_mean * pmax;
        let tx0 = ((rect[0] - reach) / t).floor() as i32;
        let ty0 = ((rect[1] - reach) / t).floor() as i32;
        let tx1 = ((rect[2] + reach) / t).floor() as i32;
        let ty1 = ((rect[3] + reach) / t).floor() as i32;
        for ty in ty0..=ty1 {
            for tx in tx0..=tx1 {
                let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
                let tile = (tx, ty);
                // Clusters.
                let base = stream(self.seed ^ PARENT_SALT, tx, ty, species);
                let n = (kappa_max * area + unit(mix(base))).floor() as u64;
                for j in 0..n {
                    let (px, py) = (x0 + draw(base, j, 0) * t, y0 + draw(base, j, 1) * t);
                    // Box distance to the rect: past the reach no offspring lands inside (an exact skip).
                    let dx = (rect[0] - px).max(px - rect[2]).max(0.0);
                    let dy = (rect[1] - py).max(py - rect[3]).max(0.0);
                    if dx.hypot(dy) > reach {
                        continue;
                    }
                    let su = suit.at(&fields.sample(px, py));
                    let Some((d, _)) = stand_at(px, py) else { continue };
                    if draw(base, j, 2) >= su * d * prior(px, py) {
                        continue;
                    }
                    let m = poisson(sp.offspring_mean * su, draw(base, j, 3));
                    let ob = mix(base ^ (j + 1).wrapping_mul(0xa076_1d64_78bd_642f));
                    for o in 0..u64::from(m) {
                        let r = sigma * (-2.0 * (1.0 - draw(ob, o, 0)).ln()).sqrt();
                        if r > reach {
                            continue;
                        }
                        let (sn, cs) = (draw(ob, o, 1) * std::f64::consts::TAU).sin_cos();
                        let (x, y) = (px + r * cs, py + r * sn);
                        if !inside(x, y) {
                            continue;
                        }
                        let Some((_, edge)) = stand_at(x, y) else { continue };
                        let key = CandidateKey { layer: sp.layer, species, tile, parent: j as u32, offspring: o as u32 };
                        emit(key, x, y, edge, draw(ob, o, 2), hash_k(ob, o, 3));
                    }
                }
                // Background.
                if bg_max <= 0.0 {
                    continue;
                }
                let bb = stream(self.seed ^ BACKGROUND_SALT, tx, ty, species);
                let n = (bg_max * area + unit(mix(bb))).floor() as u64;
                for i in 0..n {
                    let (x, y) = (x0 + draw(bb, i, 0) * t, y0 + draw(bb, i, 1) * t);
                    if !inside(x, y) {
                        continue;
                    }
                    let su = suit.at(&fields.sample(x, y));
                    let Some((d, edge)) = stand_at(x, y) else { continue };
                    if draw(bb, i, 2) >= su * su * d * prior(x, y) {
                        continue;
                    }
                    let key = CandidateKey { layer: sp.layer, species, tile, parent: BACKGROUND, offspring: i as u32 };
                    emit(key, x, y, edge, draw(bb, i, 3), hash_k(bb, i, 4));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Scatter v2 competition (spec `worldgen-scatter.md` "Design" 6 and "Determinism across tiles"; slice WG-E2e).
//
// Inside a layer, two candidates conflict when `dist < R_ij = k_i r_i + k_j r_j` (`crown_k` x crown radius per
// species; `crown_k (r_i + r_j)` for one species, and never past `R_max`). Priority: the larger size mark, then the
// larger tie-break hash, then the smaller key (a total order). A fixed-priority parallel maximal independent set,
// bounded at `scatter.competition_rounds` rounds (Luby 1986, U): every candidate starts eligible; in a round an
// eligible candidate with no higher-priority eligible conflict wins; an eligible candidate that conflicts with a
// winner is out. Round 1 is Matern II; later rounds re-admit the losers no winner blocks. Winners never conflict
// (two eligible conflicting candidates never both win; a later winner was eligible, so free of earlier ones). It
// never runs "until converged": the state after round r depends only on candidates within `(2r - 1) R_max`, so a
// tile's winners are exact when its candidates cover the tile grown by `(2k - 1) R_max`
// ([`CandidateRules::tile_rect`]).
//
// Neighbours come from a bucket grid of cell `max(R_max of the layer, sqrt(area / n))` (so the grid never has more
// cells than candidates; a 3 x 3 block holds every conflict). Each round is a pure map over the candidates (rayon,
// collected in order), so the result is the same on any thread count.

/// The competition priority: `Greater` when `a` outranks `b`.
fn priority(a: &Candidate, b: &Candidate) -> std::cmp::Ordering {
    a.size_m.total_cmp(&b.size_m).then(a.tiebreak.cmp(&b.tiebreak)).then(b.key.cmp(&a.key))
}

/// The conflicting neighbours of every candidate of one layer (indices into `cands`), by a bucket grid.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // cell indices
fn conflicts(file: &SpeciesFile, cands: &[Candidate]) -> Vec<Vec<u32>> {
    let k = |c: &Candidate| file.species[c.key.species as usize].crown_k;
    let r_max = cands.iter().map(|c| 2.0 * k(c) * c.crown_m).fold(0.0, f64::max);
    let n = cands.len();
    if n < 2 || r_max <= 0.0 {
        return vec![Vec::new(); n];
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for c in cands {
        (x0, y0, x1, y1) = (x0.min(c.pos[0]), y0.min(c.pos[1]), x1.max(c.pos[0]), y1.max(c.pos[1]));
    }
    let cell = r_max.max(((x1 - x0) * (y1 - y0) / n as f64).sqrt());
    let nx = ((x1 - x0) / cell).floor() as usize + 1;
    let ny = ((y1 - y0) / cell).floor() as usize + 1;
    let cell_of = |c: &Candidate| {
        let cx = (((c.pos[0] - x0) / cell).floor() as usize).min(nx - 1);
        let cy = (((c.pos[1] - y0) / cell).floor() as usize).min(ny - 1);
        (cx, cy)
    };
    // Counting sort into cells (CSR).
    let mut start = vec![0u32; nx * ny + 1];
    for c in cands {
        let (cx, cy) = cell_of(c);
        start[cy * nx + cx + 1] += 1;
    }
    for i in 0..nx * ny {
        start[i + 1] += start[i];
    }
    let mut next = start.clone();
    let mut items = vec![0u32; n];
    for (i, c) in cands.iter().enumerate() {
        let (cx, cy) = cell_of(c);
        let slot = &mut next[cy * nx + cx];
        items[*slot as usize] = i as u32;
        *slot += 1;
    }
    (0..n)
        .into_par_iter()
        .map(|i| {
            let a = &cands[i];
            let (cx, cy) = cell_of(a);
            let mut out = Vec::new();
            for gy in cy.saturating_sub(1)..=(cy + 1).min(ny - 1) {
                for gx in cx.saturating_sub(1)..=(cx + 1).min(nx - 1) {
                    let g = gy * nx + gx;
                    for &j in &items[start[g] as usize..start[g + 1] as usize] {
                        let b = &cands[j as usize];
                        if j as usize != i
                            && (a.pos[0] - b.pos[0]).hypot(a.pos[1] - b.pos[1]) < k(a) * a.crown_m + k(b) * b.crown_m
                        {
                            out.push(j);
                        }
                    }
                }
            }
            out.sort_unstable();
            out
        })
        .collect()
}

/// The winners of `rounds` competition rounds among the candidates of one layer: one flag per candidate.
fn compete_layer(file: &SpeciesFile, cands: &[Candidate], rounds: u32) -> Vec<bool> {
    let nbr = conflicts(file, cands);
    let n = cands.len();
    let mut won = vec![false; n];
    let mut eligible = vec![true; n];
    for _ in 0..rounds {
        let wins: Vec<bool> = (0..n)
            .into_par_iter()
            .map(|i| {
                eligible[i]
                    && !nbr[i].iter().any(|&j| {
                        let j = j as usize;
                        eligible[j] && priority(&cands[j], &cands[i]).is_gt()
                    })
            })
            .collect();
        for (w, &new) in won.iter_mut().zip(&wins) {
            *w |= new;
        }
        eligible = (0..n).into_par_iter().map(|i| eligible[i] && !won[i] && !nbr[i].iter().any(|&j| won[j as usize])).collect();
    }
    won
}

/// The winners of the competition (spec "Design" 6) among `cands` sorted by key ([`CandidateRules::candidates`]):
/// one flag per candidate. Layers compete apart; `rounds` is `scatter.competition_rounds`.
#[must_use]
pub fn compete(file: &SpeciesFile, cands: &[Candidate], rounds: u32) -> Vec<bool> {
    let mut won = Vec::with_capacity(cands.len());
    for layer in cands.chunk_by(|a, b| a.key.layer == b.key.layer) {
        won.extend(compete_layer(file, layer, rounds));
    }
    won
}

impl CandidateRules<'_> {
    /// The competition winners inside tile `(tx, ty)`'s square (half open), sorted by key: candidates over
    /// [`Self::tile_rect`], [`compete`] at `scatter.competition_rounds`, kept by membership. `stand` must cover the
    /// tile rect. Bitwise the same as the winners of any larger build, cut to the square.
    #[must_use]
    pub fn tile_winners(&self, fields: &dyn Fields, stand: &StandMap, (tx, ty): (i32, i32)) -> Vec<Candidate> {
        let cands = self.candidates(fields, stand, self.tile_rect((tx, ty)));
        let won = compete(self.file, &cands, self.file.values.competition_rounds);
        let t = self.tile_m;
        let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
        cands
            .into_iter()
            .zip(won)
            .filter(|(c, w)| *w && c.pos[0] >= x0 && c.pos[0] < x0 + t && c.pos[1] >= y0 && c.pos[1] < y0 + t)
            .map(|(c, _)| c)
            .collect()
    }
}
