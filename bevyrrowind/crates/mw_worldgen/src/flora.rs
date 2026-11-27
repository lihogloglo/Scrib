//! Flora of the same map at 10× (spec `worldgen-adapter.md` "Same map at 10× (WG-X)" step 6; WG-X5; scatter v2,
//! spec `worldgen-scatter.md` "Adapter data", WG-E2i.3): the content's own nature references, learned per region,
//! scattered again on the 10× land by the scatter v2 passes (`sf_worldgen::scatter`: stand mosaic, Thomas clusters,
//! bounded competition, layers by light, water, rank budget).
//!
//! **What scatters:** the references [`crate::classify`] calls nature (the ones the 10× world drops): flora
//! (`/flora_`, `/f/flora` meshes, harvestable plants included) and rocks (`terrain_rock*` and large rocks). Flora
//! base records become the **models** of the species file's rows ([`SPECIES_FILE`]: one canopy tree, understorey,
//! ground); rock base records become learned rock models ([`Learned::rocks`]), pack-model entries of the rock units
//! next to the library classes ([`rocks`], WG-E3f.1; the passes run the rock layer from WG-E3f.4).
//!
//! **Learning, on the source's own terrain** ([`learn`]): land samples are the `LAND` mosaic's dry samples with a
//! region (one per 1.83 m vertex); references by their position (bilinear), slope by central differences, their up
//! axis from the reference rotation against the ground normal, their origin height above the ground. References on
//! wet ground (kelp, water plants) and outside any region are counted and skipped: the scatter never places on
//! water. Per base record a [`Slot`]: references per region, scale range, model extent, pose (align, tilt jitter,
//! offset; the v1 rules) and steepest slope. A base record joins the row whose size range is nearest its typical
//! size (model extent x median scale; geometric range centres, log distance). Per row the learned columns replace
//! the authored ones: size bounds (the slots' scale range x extent, inside the column range), pose (count-weighted
//! means, for the record: placement uses the slot's own), the slope hat's `e_max` (steepest slot + the margin). A
//! row that learned no model is dropped.
//!
//! **Region prior** ([`region_prior`]): per row, region `r` and climate class `c`, the learned density `D(r)`
//! (references per m² of the region's dry source land) over its class mean `M(c) = sum_r D(r) A(r, c) / sum_r
//! A(r, c)`, `A` the guide land per region and class. It keeps each class's mean at the authored intensity and
//! moves stems between regions the way the source has them; the candidates clamp it to `scatter.region_prior_max`.
//!
//! **At 10×** ([`World`]): the guide's grid is the mosaic's (step 1: positions never change), so a guide point's
//! source point is the same `(col, row)`. The fields are the guide's products ([`product_fields`]: climate rasters,
//! slope, aspect, curvature, altitude from the guide heights), the region from the source cell, the forced clearings
//! the stamps (weight > 0: settlement footprints and bands), the guide's site and road clearings (WG-V1b,
//! [`guide_clearings`]) and the guide's own towns (no stamp: their placements' built-up area, [`towns`]), the water
//! the guide's `water.json` (else the sea), the placement height from the runtime tiles (`GuideTiles::height` at
//! level 0, stamps and filter included: the ground the game draws), the up axis from the guide's macro normal (the
//! filter's detail is below the scale the source was measured at). Each kept instance picks its model among its
//! row's slots whose size range holds its size mark, weighted by their references in its region (a hash of its key).

// Grid maths: sample counts stay far below 2^31 and heights within f32 range (the mosaic's own bounds).
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// Grid maths reads best in the usual letters (m mosaic, g grid, c col, r row, x, y).
#![allow(clippy::many_single_char_names, clippy::similar_names)]

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use glam::{DVec3, Quat, Vec3};
use mw_world::MwWorldSource;
use mw_world::convert::UNIT_METERS_F64;
use sf_coords::{WorldPos, WorldTransform};
use sf_records::RecordDb;
use sf_vfs::VfsPath;
use sf_world::{Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};
use sf_worldgen::climate_map::ClimateMap;
use sf_worldgen::guide::Guide;
use sf_worldgen::reach::VALID_FILE;
use sf_worldgen::roads::RoadNetwork;
use sf_worldgen::scatter::{CandidateKey, CandidateRules, CanopyTargets, LayerWinners, Params, WaterMask, budget_cut_load};
use sf_worldgen::scatter_calib::{self, Calibration};
use sf_worldgen::scatter_product::{self, InputsHash};
use sf_worldgen::scatter_fields::{Fields, GridFields, Suitability};
use sf_worldgen::scatter_species::{Hat, Layer, SpeciesFile, species_declared};
use sf_worldgen::scatter_stand::StandRules;
use sf_worldgen::sites::SitesFile;
use sf_worldgen::sites::clearings::{ClearingValues, Clearings};
use sf_worldgen::tiles::{GuideTiles, WORLD_FILTER_FILE};
use sf_worldgen::water::GuideWater;

use crate::classify::{Class, RefInfo, classify};

#[path = "flora_towns.rs"]
pub mod towns;
#[path = "flora_rocks.rs"]
pub mod rocks;
use towns::{TownCensus, TownMask, TownParams};
use crate::{DumpError, Mosaic, RegionMap, assemble, region_map};

/// Model bounds lookup: model VFS path → model-space `(min, max)` at unit scale (engine axes, metres).
pub type Bounds<'a> = &'a dyn Fn(&str) -> Option<([f32; 3], [f32; 3])>;

/// Source group of a nature reference: flora (the plant rows' models) or rocks (the rock layer, WG-E3f).
pub const GROUP_FLORA: u8 = 0;
pub const GROUP_ROCKS: u8 = 1;

/// The budget groups of pass 10, in [`FloraRun::full`] order.
pub const BUDGET_GROUPS: [&str; 4] = ["rocks", "canopy", "understorey", "ground"];

/// The adapter-file name of the scatter v2 species file (`bevyrrowind/worldgen/`; WG-E2i.2); a guide folder's own
/// copy wins over the shipped one ([`species_file`]).
// tunables: const (file name)
pub const SPECIES_FILE: &str = "scatter_species.toml";

/// The shipped species file (`bevyrrowind/worldgen/scatter_species.toml`).
pub const SHIPPED_SPECIES: &str = include_str!("../../../worldgen/scatter_species.toml");

/// Size of a model without bounds, metres.
// tunables: const (fallback: every Morrowind flora mesh has bounds)
const UNKNOWN_SIZE_M: f32 = 1.0;

/// Salt of the pose stream (model pick, yaw, tilt) in the scatter seed's hash space.
// tunables: const (hash salt, not a value)
const POSE_SALT: u64 = 0x9f1a_2d00_0000_4e2d;

/// One nature reference of the source. Frame: x east, y north, z up, metres.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceRef {
    /// Base record id, lower case (the species key).
    pub key: String,
    pub base: RecordRef,
    pub model: Option<VfsPath>,
    pub class: PlacementClass,
    pub flags: PlacementFlags,
    pub group: u8,
    pub pos: [f64; 3],
    /// The reference's up axis (unit).
    pub up: [f32; 3],
    pub scale: f32,
    /// Largest edge of the model's bounds at unit scale, metres.
    pub extent_m: Option<f32>,
}

/// The source's nature references ([module docs](self): what scatters).
pub fn source_refs(source: &MwWorldSource, bounds: Bounds<'_>) -> Vec<SourceRef> {
    let rock_min_m = crate::anchor::Params::default().rock_min_m;
    source
        .all_placements(&mw_world::ids::exterior())
        .filter_map(|p| {
            let e = p.transform.pos.to_f64();
            let pos = [e.x, -e.z, e.y];
            let b = p.model.as_ref().and_then(|m| bounds(m.as_str()));
            let info = RefInfo {
                key: p.id.to_string(),
                kind: p.base.kind.to_string(),
                base: p.base.id.as_str().to_owned(),
                model: p.model.as_ref().map(|m| m.as_str().to_ascii_lowercase().replace('\\', "/")),
                pos,
                scale: p.transform.scale.x,
                bounds: b,
            };
            let (class, rule) = classify(&info, rock_min_m);
            if class != Class::Nature {
                return None;
            }
            let u = p.transform.rotation * Vec3::Y;
            Some(SourceRef {
                key: info.base.to_ascii_lowercase(),
                base: p.base.clone(),
                model: p.model.clone(),
                class: p.class,
                flags: p.flags - PlacementFlags::GROUNDED,
                group: if rule.starts_with("flora") { GROUP_FLORA } else { GROUP_ROCKS },
                pos,
                up: [u.x, -u.z, u.y],
                scale: info.scale,
                extent_m: b.map(|(lo, hi)| (0..3).map(|i| hi[i] - lo[i]).fold(0.0, f32::max)),
            })
        })
        .collect()
}

/// A height grid: row 0 north, column 0 west; `origin` = (x east, y north) of sample (0, 0).
#[derive(Clone, Copy, Debug)]
pub struct Grid<'a> {
    pub w: usize,
    pub h: usize,
    pub origin: [f64; 2],
    pub spacing: f64,
    pub data: &'a [f32],
}

impl Grid<'_> {
    /// Continuous `(col, row)` of a point.
    pub fn colrow(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.origin[0]) / self.spacing, (self.origin[1] - y) / self.spacing)
    }

    pub fn inside(&self, c: f64, r: f64) -> bool {
        c >= 0.0 && r >= 0.0 && c <= (self.w - 1) as f64 && r <= (self.h - 1) as f64
    }

    /// Bilinear height at `(col, row)`, clamped to the grid.
    pub fn at(&self, c: f64, r: f64) -> f64 {
        let c = c.clamp(0.0, (self.w - 1) as f64);
        let r = r.clamp(0.0, (self.h - 1) as f64);
        let (c0, r0) = ((c.floor() as usize).min(self.w - 2), (r.floor() as usize).min(self.h - 2));
        let (tc, tr) = (c - c0 as f64, r - r0 as f64);
        let v = |cc: usize, rr: usize| f64::from(self.data[rr * self.w + cc]);
        let top = v(c0, r0) + (v(c0 + 1, r0) - v(c0, r0)) * tc;
        let bot = v(c0, r0 + 1) + (v(c0 + 1, r0 + 1) - v(c0, r0 + 1)) * tc;
        top + (bot - top) * tr
    }

    /// Gradient `(d/dx east, d/dy north)` at `(col, row)`: central differences one sample apart.
    pub fn gradient(&self, c: f64, r: f64) -> (f64, f64) {
        let gx = (self.at(c + 1.0, r) - self.at(c - 1.0, r)) / (2.0 * self.spacing);
        let gy = (self.at(c, r - 1.0) - self.at(c, r + 1.0)) / (2.0 * self.spacing);
        (gx, gy)
    }

    /// Slope (degrees) and unit normal at `(col, row)`.
    pub fn slope_normal(&self, c: f64, r: f64) -> (f32, [f32; 3]) {
        let (gx, gy) = self.gradient(c, r);
        let l = (gx * gx + gy * gy + 1.0).sqrt();
        (gx.hypot(gy).atan().to_degrees() as f32, [(-gx / l) as f32, (-gy / l) as f32, (1.0 / l) as f32])
    }
}

/// The mosaic as a [`Grid`] in source metres.
pub fn source_grid(m: &Mosaic) -> Grid<'_> {
    let (ox, oy) = m.mw_units(0, 0);
    Grid {
        w: m.width,
        h: m.height,
        origin: [ox * UNIT_METERS_F64, oy * UNIT_METERS_F64],
        spacing: f64::from(mw_world::terrain::VERTEX_UNITS) * UNIT_METERS_F64,
        data: &m.heights,
    }
}

/// Region index of mosaic sample `(col, row)` (0: none).
fn region_at(m: &Mosaic, regions: &RegionMap, c: usize, r: usize) -> u8 {
    regions.of(m.cell_of(c.min(m.width - 1), r.min(m.height - 1)))
}


/// One learned model of a species row: a source base record and what its instances copy ([module docs](self)).
#[derive(Clone, Debug, PartialEq)]
pub struct Slot {
    /// Index of its source reference in [`Learned::reps`].
    pub rep: usize,
    /// Source references per region index.
    pub per_region: Vec<usize>,
    /// Scale range of its references.
    pub scale: [f32; 2],
    /// Largest edge of the model's bounds at unit scale, m.
    pub extent_m: f32,
    /// Share of the way from vertical to the ground normal of the up axis (0 upright, 1 along the normal).
    pub align: f32,
    /// Median tilt of the up axis off its aligned axis, degrees.
    pub tilt_jitter_deg: f32,
    /// Median origin height above the ground per unit scale, m.
    pub offset_m: f32,
    /// Steepest ground under one of its references, degrees.
    pub max_slope_deg: f32,
}

impl Slot {
    /// Source references.
    #[must_use]
    pub fn count(&self) -> usize {
        self.per_region.iter().sum()
    }

    /// Size range (scale range x extent), m.
    #[must_use]
    pub fn size_m(&self) -> [f64; 2] {
        [f64::from(self.scale[0] * self.extent_m), f64::from(self.scale[1] * self.extent_m)]
    }
}

/// What learning produced: the adapted species file and its rows' models.
#[derive(Clone, Debug)]
pub struct Learned {
    /// The species file with the learned columns replaced; rows that learned no model are dropped.
    pub file: SpeciesFile,
    /// Per row of `file`: its models.
    pub slots: Vec<Vec<Slot>>,
    /// Per row of `file`, per region: source references per m² of the region's dry source land.
    pub density: Vec<Vec<f64>>,
    /// The slots' source references.
    pub reps: Vec<SourceRef>,
    /// References skipped: on wet ground (per group: water plants, sea-floor rocks), outside the mosaic or `LAND`,
    /// without a region.
    pub skipped_wet: [usize; 2],
    /// The wet-skipped references per base record.
    pub wet_species: BTreeMap<String, usize>,
    pub skipped_outside: usize,
    pub skipped_no_region: usize,
    /// Source references learned, per group.
    pub learned: [usize; 2],
    /// The learned rock models (WG-E3f.1): one per rock base record, its reference in [`Self::reps`] (after the
    /// rows' models); [`rocks::rock_entries`] makes them pack-model entries of the rock units.
    pub rocks: Vec<Slot>,
    /// Per region: rock references per m² of the region's dry source land ([`rocks::rock_prior`]).
    pub rock_density: Vec<f64>,
}

/// One source reference on dry land in a region.
struct Obs {
    region: usize,
    slope_deg: f32,
    scale: f32,
    up: [f32; 3],
    normal: [f32; 3],
    offset_m: f32,
}

/// A base record's first reference and its observations.
type Seen<'a> = (&'a SourceRef, Vec<Obs>);

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
#[must_use]
pub fn aligned_up(normal: [f32; 3], align: f32) -> [f32; 3] {
    let a = align.clamp(0.0, 1.0);
    norm3([a * normal[0], a * normal[1], (1.0 - a) + a * normal[2]])
}

/// `up` tilted by `deg` towards the tangent direction `turn` (a fraction of a turn).
fn tilt(up: [f32; 3], deg: f32, turn: f64) -> [f32; 3] {
    if deg <= 0.0 {
        return up;
    }
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

/// The slot of one base record's observations (v1's pose rules: align = median of the up axis's share of the
/// normal over the references on ground tilted at least `align_min_tilt_deg`).
fn slot(rep: usize, extent_m: f32, obs: &[Obs], regions: usize, prm: &Params) -> Slot {
    let mut per_region = vec![0; regions];
    for o in obs {
        per_region[o.region] += 1;
    }
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
    Slot {
        rep,
        per_region,
        scale: obs.iter().fold([f32::INFINITY, f32::NEG_INFINITY], |a, o| [a[0].min(o.scale), a[1].max(o.scale)]),
        extent_m,
        align,
        tilt_jitter_deg: median(&mut jitter),
        offset_m: median(&mut offsets),
        max_slope_deg: obs.iter().map(|o| o.slope_deg).fold(0.0, f32::max),
    }
}

/// The row of `file` whose size range is nearest `size_m` (log distance to the range's geometric centre; the
/// first on a tie).
#[must_use]
pub fn row_of(file: &SpeciesFile, size_m: f64) -> Option<usize> {
    let d = |s: &sf_worldgen::scatter_species::Species| {
        (size_m.max(1e-3).ln() - f64::midpoint(s.size_min_m.max(1e-3).ln(), s.size_max_m.max(1e-3).ln())).abs()
    };
    (0..file.species.len()).min_by(|&a, &b| d(&file.species[a]).total_cmp(&d(&file.species[b])))
}

/// The declared range of a species column: the file's `[columns]` entry, else the declared default's.
fn column_range(file: &SpeciesFile, name: &str) -> [f64; 2] {
    if let Some(c) = file.columns.get(name) {
        return [c.min, c.max];
    }
    species_declared().iter().find(|d| d.name == name).map_or([f64::NEG_INFINITY, f64::INFINITY], |d| [d.min, d.max])
}

/// The learned columns of `file`'s rows from their slots ([module docs](self)): size bounds inside the column
/// ranges, count-weighted pose means, the slope hat's `e_max`. A row without slots keeps its authored values.
fn fill_columns(file: &mut SpeciesFile, slots: &[Vec<Slot>], prm: &Params) {
    let (lo, hi) = (column_range(file, "size_min_m"), column_range(file, "size_max_m"));
    let slope = column_range(file, "hat_slope_deg");
    for (sp, sl) in file.species.iter_mut().zip(slots) {
        if sl.is_empty() {
            continue;
        }
        let size = sl.iter().fold([f64::INFINITY, f64::NEG_INFINITY], |a, s| [a[0].min(s.size_m()[0]), a[1].max(s.size_m()[1])]);
        sp.size_min_m = size[0].clamp(lo[0], lo[1]);
        sp.size_max_m = size[1].clamp(hi[0], hi[1]).max(sp.size_min_m);
        let n = sl.iter().map(Slot::count).sum::<usize>() as f64;
        let mean = |f: &dyn Fn(&Slot) -> f32| sl.iter().map(|s| f64::from(f(s)) * s.count() as f64).sum::<f64>() / n;
        sp.align = mean(&|s| s.align);
        sp.tilt_jitter_deg = mean(&|s| s.tilt_jitter_deg);
        sp.offset_m = mean(&|s| s.offset_m);
        let steep = sl.iter().map(|s| s.max_slope_deg).fold(0.0, f32::max);
        let e_max = f64::from(steep + prm.slope_margin_deg).clamp(slope[0], slope[1]);
        sp.hats.slope_deg = Some(match sp.hats.slope_deg {
            Some(h) => Hat { e_min: h.e_min.min(e_max), i_min: h.i_min.min(e_max), i_max: h.i_max.min(e_max), e_max },
            None => Hat { e_min: 0.0, i_min: 0.0, i_max: e_max, e_max },
        });
    }
}

/// Learns the models and densities of `authored`'s rows from `refs` on the source mosaic (`sea_m`: the source sea
/// level; `prm`: `align_min_tilt_deg`, `slope_margin_deg`) ([module docs](self)).
pub fn learn(m: &Mosaic, regions: &RegionMap, refs: &[SourceRef], authored: &SpeciesFile, prm: &Params, sea_m: f64) -> Learned {
    let g = source_grid(m);
    let nreg = regions.legend.len();
    let mut land = vec![0u64; nreg];
    for r in 0..m.height {
        for c in 0..m.width {
            let k = r * m.width + c;
            let region = usize::from(region_at(m, regions, c, r));
            if m.mask[k] != 0 && region != 0 && f64::from(m.heights[k]) > sea_m && region < nreg {
                land[region] += 1;
            }
        }
    }
    // Flora and rock base records, each in first-seen order.
    let (mut index, mut rock_index): (BTreeMap<&str, usize>, BTreeMap<&str, usize>) = (BTreeMap::new(), BTreeMap::new());
    let (mut seen, mut rock_seen): (Vec<Seen<'_>>, Vec<Seen<'_>>) = (Vec::new(), Vec::new());
    let (mut wet, mut outside, mut no_region, mut learned) = ([0usize; 2], 0, 0, [0usize; 2]);
    let mut wet_species = BTreeMap::new();
    for s in refs {
        let (c, r) = g.colrow(s.pos[0], s.pos[1]);
        let (ci, ri) = (c.round() as usize, r.round() as usize);
        if !g.inside(c, r) || m.mask[ri.min(m.height - 1) * m.width + ci.min(m.width - 1)] == 0 {
            outside += 1;
            continue;
        }
        let ground = g.at(c, r);
        let group = usize::from(s.group.min(1));
        if ground <= sea_m {
            wet[group] += 1;
            *wet_species.entry(s.key.clone()).or_default() += 1;
            continue;
        }
        let region = usize::from(region_at(m, regions, c.floor() as usize, r.floor() as usize));
        if region == 0 || region >= nreg {
            no_region += 1;
            continue;
        }
        learned[group] += 1;
        let (index, seen) = if s.group == GROUP_FLORA { (&mut index, &mut seen) } else { (&mut rock_index, &mut rock_seen) };
        let next = seen.len();
        let id = *index.entry(s.key.as_str()).or_insert(next);
        if id == next {
            seen.push((s, Vec::new()));
        }
        let (slope_deg, normal) = g.slope_normal(c, r);
        seen[id].1.push(Obs { region, slope_deg, scale: s.scale, up: s.up, normal, offset_m: (s.pos[2] - ground) as f32 });
    }
    // Base records to rows.
    let mut file = authored.clone();
    let mut slots: Vec<Vec<Slot>> = vec![Vec::new(); file.species.len()];
    let mut reps = Vec::with_capacity(seen.len());
    for (rep, obs) in &seen {
        let extent = rep.extent_m.unwrap_or(UNKNOWN_SIZE_M);
        let mut sc: Vec<f32> = obs.iter().map(|o| o.scale).collect();
        let Some(row) = row_of(&file, f64::from(extent * median(&mut sc))) else { continue };
        slots[row].push(slot(reps.len(), extent, obs, nreg, prm));
        reps.push((*rep).clone());
    }
    fill_columns(&mut file, &slots, prm);
    let kept: Vec<bool> = slots.iter().map(|s| !s.is_empty()).collect();
    let mut k = kept.iter();
    file.species.retain(|_| *k.next().unwrap_or(&false));
    slots.retain(|s| !s.is_empty());
    let rocks: Vec<Slot> = rock_seen
        .iter()
        .map(|(rep, obs)| {
            let s = slot(reps.len(), rep.extent_m.unwrap_or(UNKNOWN_SIZE_M), obs, nreg, prm);
            reps.push((*rep).clone());
            s
        })
        .collect();
    let sample_m2 = g.spacing * g.spacing;
    let per_land = |n: usize, r: usize| if land[r] == 0 { 0.0 } else { n as f64 / (land[r] as f64 * sample_m2) };
    let rock_density = (0..nreg).map(|r| per_land(rocks.iter().map(|s| s.per_region[r]).sum(), r)).collect();
    let density = slots
        .iter()
        .map(|sl| {
            (0..nreg)
                .map(|r| per_land(sl.iter().map(|s| s.per_region[r]).sum(), r))
                .collect()
        })
        .collect();
    Learned {
        file,
        slots,
        density,
        reps,
        skipped_wet: wet,
        wet_species,
        skipped_outside: outside,
        skipped_no_region: no_region,
        learned,
        rocks,
        rock_density,
    }
}

/// The region prior of every row ([module docs](self)): `prior[row][region][class]` = `D(row, region) /
/// M(row, class)` (0 where `M` is 0), `area[region][class]` the guide land samples (class index `classes` = none).
#[must_use]
pub fn region_prior(density: &[Vec<f64>], area: &[Vec<u64>]) -> Vec<Vec<Vec<f64>>> {
    let classes = area.first().map_or(0, Vec::len);
    density
        .iter()
        .map(|d| {
            let mean: Vec<f64> = (0..classes)
                .map(|c| {
                    let (mut num, mut den) = (0.0, 0.0);
                    for (r, a) in area.iter().enumerate() {
                        let a = a[c] as f64;
                        num += d.get(r).copied().unwrap_or(0.0) * a;
                        den += a;
                    }
                    if den > 0.0 { num / den } else { 0.0 }
                })
                .collect();
            (0..area.len())
                .map(|r| {
                    let dr = d.get(r).copied().unwrap_or(0.0);
                    mean.iter().map(|&m| if m > 0.0 { dr / m } else { 0.0 }).collect()
                })
                .collect()
        })
        .collect()
}

/// The 10× world: what the passes read besides the fields ([module docs](self)).
pub struct World<'a> {
    pub guide: Grid<'a>,
    pub source: &'a Mosaic,
    pub regions: &'a RegionMap,
    pub tiles: &'a GuideTiles,
    /// The guide's site and road clearings (WG-V1b), when it has any.
    pub clearings: Option<&'a Clearings>,
    /// The guide's own towns.
    pub towns: &'a TownMask,
}

impl<'a> World<'a> {
    /// The 10× world on `tiles`' guide (`source` and `regions` the content's), outside `clearings` and `towns` too.
    #[must_use]
    pub fn new(
        source: &'a Mosaic,
        regions: &'a RegionMap,
        tiles: &'a GuideTiles,
        clearings: Option<&'a Clearings>,
        towns: &'a TownMask,
    ) -> Self {
        let g = tiles.guide();
        Self {
            guide: Grid { w: g.w, h: g.h, origin: g.origin_m, spacing: g.spacing_m, data: &g.heights },
            source,
            regions,
            tiles,
            clearings,
            towns,
        }
    }

    fn sample(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let (c, r) = self.guide.colrow(x, y);
        self.guide.inside(c, r).then_some((c, r))
    }

    /// The region index at a guide point (`None`: outside the guide, `LAND` or every region).
    #[must_use]
    pub fn region(&self, x: f64, y: f64) -> Option<u16> {
        let (c, r) = self.sample(x, y)?;
        let m = self.source;
        if m.mask[(r.round() as usize).min(m.height - 1) * m.width + (c.round() as usize).min(m.width - 1)] == 0 {
            return None;
        }
        let region = region_at(m, self.regions, c.floor() as usize, r.floor() as usize);
        (region != 0).then_some(u16::from(region))
    }

    /// Which exclusion holds a point first: 1 the stamps, 2 the clearings, 3 the towns; 0 none.
    #[must_use]
    pub fn exclusion(&self, x: f64, y: f64) -> u8 {
        if self.tiles.stamps().and_then(|s| s.sample(x, y)).is_some_and(|(_, w)| w > 0.0) {
            1
        } else if self.clearings.is_some_and(|c| c.contains(x, y)) {
            2
        } else if self.towns.contains(x, y) {
            3
        } else {
            0
        }
    }

    /// A forced clearing: stamped, cleared or a town.
    #[must_use]
    pub fn excluded(&self, x: f64, y: f64) -> bool {
        self.exclusion(x, y) != 0
    }

    /// The final ground height at a guide point, m.
    #[must_use]
    pub fn height(&self, x: f64, y: f64) -> f64 {
        f64::from(self.tiles.height(x, -y, 0))
    }

    /// The guide's land rect `[x0, y0, x1, y1]`.
    #[must_use]
    pub fn rect(&self) -> [f64; 4] {
        let g = &self.guide;
        [g.origin[0], g.origin[1] - (g.h - 1) as f64 * g.spacing, g.origin[0] + (g.w - 1) as f64 * g.spacing, g.origin[1]]
    }
}

/// The clearings of a flora run: what gave them, and the guide land they took.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClearingCensus {
    /// Sites that gave a clearing (not dropped, not kept towns).
    pub sites: usize,
    /// Road segments that gave a clearing.
    pub roads: usize,
    /// Guide land samples only the clearings exclude (not the stamps').
    pub excluded: usize,
}

/// The site list of a guide folder before V1 judged it (format 1 or 2), read when there is no [`VALID_FILE`].
// tunables: const (file name)
pub const SITES_FILE: &str = "sites.json";

/// The clearings of a guide folder (spec `sites.md` "WG-V1b clearings"): its judged sites ([`VALID_FILE`], else
/// [`SITES_FILE`]) and its `roads.json`, each when present, at the declared `clearing.*` values; `None` when they give
/// no clearing (format-1 sites are kept towns, which clear nothing: only their roads do).
///
/// # Errors
/// A site list or road network that is present but unreadable.
pub fn guide_clearings(dir: &Path) -> Result<Option<Clearings>, DumpError> {
    let invalid = |what: &Path, e: sf_worldgen::Error| DumpError::Invalid(format!("{}: {e}", what.display()));
    let sites = match [dir.join(VALID_FILE), dir.join(SITES_FILE)].into_iter().find(|p| p.exists()) {
        Some(p) => SitesFile::read(&p).map_err(|e| invalid(&p, e))?.sites,
        None => Vec::new(),
    };
    let roads = RoadNetwork::read(dir).map_err(|e| invalid(&dir.join("roads.json"), e))?;
    let c = Clearings::new(&sites, roads.as_ref(), &ClearingValues::default());
    Ok((!c.is_empty()).then_some(c))
}

/// Checks the guide is the mosaic's grid scaled by some S (dims equal, origin = source origin × S); returns S.
pub fn guide_scale(guide: &Guide, m: &Mosaic) -> Result<f64, DumpError> {
    let src = source_grid(m);
    let s = guide.spacing_m / src.spacing;
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-6 * b.abs().max(1.0);
    if [guide.w, guide.h] != [m.width, m.height]
        || !close(guide.origin_m[0], src.origin[0] * s)
        || !close(guide.origin_m[1], src.origin[1] * s)
    {
        return Err(DumpError::Invalid(format!(
            "flora: the guide ({}x{} @ {} m, origin {:?}) is not the source mosaic ({}x{}) scaled",
            guide.w, guide.h, guide.spacing_m, guide.origin_m, m.width, m.height
        )));
    }
    Ok(s)
}

/// The species file of a guide folder: its own [`SPECIES_FILE`] when present, else the shipped one.
///
/// # Errors
/// A species file that does not parse or read.
pub fn species_file(dir: &Path) -> Result<SpeciesFile, DumpError> {
    let own = dir.join(SPECIES_FILE);
    let (text, what) = if own.exists() {
        (std::fs::read_to_string(&own)?, own.display().to_string())
    } else {
        (SHIPPED_SPECIES.to_owned(), format!("shipped {SPECIES_FILE}"))
    };
    SpeciesFile::parse_toml(&text).map_err(|e| DumpError::Invalid(format!("{what}: {e}")))
}

/// The guide's product fields ([module docs](self)): `dir/climate/` on the guide heights (written by
/// `scrib-worldgen climate --base DIR --out DIR`).
///
/// # Errors
/// No climate product, or one that does not match the guide.
pub fn product_fields(dir: &Path, guide: &Guide) -> Result<GridFields, DumpError> {
    let climate = ClimateMap::read(dir).map_err(|e| {
        DumpError::Invalid(format!(
            "flora needs the climate product ({e}): run `scrib-worldgen climate --base {0} --out {0}`",
            dir.display()
        ))
    })?;
    GridFields::from_products(guide, &climate).map_err(|e| DumpError::Invalid(format!("{}: {e}", dir.display())))
}

/// One placed plant: its row and model, the pose in the guide frame (x east, y north, z up, m).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plant {
    /// The candidate key (`species` = the row of [`Learned::file`]).
    pub key: CandidateKey,
    /// Index of its model in the row's [`Learned::slots`].
    pub slot: u32,
    pub region: u16,
    /// Size mark, m.
    pub size_m: f64,
    pub pos: [f64; 3],
    pub yaw: f32,
    pub scale: f32,
    /// Up axis (unit).
    pub up: [f32; 3],
}

/// A whole flora pass on a guide, with its census numbers.
#[derive(Clone, Debug)]
pub struct FloraRun {
    pub prm: Params,
    pub learned: Learned,
    /// [`region_prior`] of the run.
    pub prior: Vec<Vec<Vec<f64>>>,
    /// Guide land samples per region and class (the last class: none).
    pub area: Vec<Vec<u64>>,
    /// Area of one guide sample, m².
    pub sample_m2: f64,
    /// Instances before the budget, per budget group ([`BUDGET_GROUPS`]).
    pub full: [usize; 4],
    pub plants: Vec<Plant>,
    pub region_names: Vec<String>,
    pub scale: f64,
    pub stamped: bool,
    /// The guide towns' exclusion.
    pub towns: TownCensus,
    /// The guide's site and road clearings.
    pub clearings: ClearingCensus,
    /// Where the kept set came from (WG-E2j.2).
    pub product: ProductUse,
    /// The canopy calibration (WG-E2o.3): the passes' own, or the product's `calibration.toml` when the kept set was
    /// read (`None` when that file is missing or unreadable; a read one has no warnings).
    pub calibration: Option<Calibration>,
    /// Seconds: source (mosaic, regions, references), learning, area census, scatter (the product read or the passes).
    pub seconds: [f64; 4],
}

/// Where a pass got its kept set (the `scatter/` guide product, WG-E2j.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductUse {
    /// No product folder (a census run): the passes ran.
    Off,
    /// Read from the guide's product.
    Read,
    /// No product, or a stale one: the passes ran and wrote it.
    Wrote,
    /// The product of these inputs failed its checks: the passes ran and rewrote it.
    Rebuilt(String),
    /// The passes ran; the product could not be written.
    Unwritten(String),
}

impl ProductUse {
    /// A short phrase for the summary line.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::Off => "no product".into(),
            Self::Read => "product read".into(),
            Self::Wrote => "product written".into(),
            Self::Rebuilt(e) => format!("product rebuilt ({e})"),
            Self::Unwritten(e) => format!("product not written ({e})"),
        }
    }
}

/// The guide census of a pass: land samples per region and class (class `n` = none), and the samples only the
/// clearings and only the towns exclude.
fn guide_census(world: &World<'_>, fields: &dyn Fields, sea_m: f64) -> (Vec<Vec<u64>>, usize, usize) {
    let g = &world.guide;
    let classes = fields.class_names().len();
    let mut area = vec![vec![0u64; classes + 1]; world.regions.legend.len()];
    let (mut cleared, mut towns) = (0, 0);
    for r in 0..g.h {
        for c in 0..g.w {
            if f64::from(g.data[r * g.w + c]) <= sea_m {
                continue;
            }
            let (x, y) = (g.origin[0] + c as f64 * g.spacing, g.origin[1] - r as f64 * g.spacing);
            let Some(region) = world.region(x, y) else { continue };
            let class = fields.sample(x, y).class.map_or(classes, |k| usize::from(k).min(classes));
            if let Some(a) = area.get_mut(usize::from(region)) {
                a[class] += 1;
            }
            match world.exclusion(x, y) {
                2 => cleared += 1,
                3 => towns += 1,
                _ => {}
            }
        }
    }
    (area, cleared, towns)
}

/// splitmix64.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Uniform `[0, 1)` from the top 53 bits.
fn unit(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// The pose stream of a candidate key.
fn key_hash(seed: u64, k: &CandidateKey) -> u64 {
    let fields = [k.layer as u64, u64::from(k.species), k.tile.0 as u64, k.tile.1 as u64, u64::from(k.parent), u64::from(k.offspring)];
    fields.iter().fold(mix(seed ^ POSE_SALT), |h, &f| mix(h ^ f))
}

/// The model pick and pose of a winner ([module docs](self)); `None` when no model of its row lives in its region.
fn pose(run: &Learned, world: &World<'_>, seed: u64, c: &sf_worldgen::scatter::Candidate) -> Option<Plant> {
    let [x, y] = c.pos;
    let region = world.region(x, y)?;
    let slots = &run.slots[c.key.species as usize];
    let weight = |s: &Slot| s.per_region.get(usize::from(region)).copied().unwrap_or(0);
    let fits = |s: &Slot| {
        let r = s.size_m();
        c.size_m >= r[0] * (1.0 - 1e-6) && c.size_m <= r[1] * (1.0 + 1e-6)
    };
    let fit_total: usize = slots.iter().filter(|s| fits(s)).map(weight).sum();
    let use_fit = fit_total > 0;
    let total = if use_fit { fit_total } else { slots.iter().map(weight).sum() };
    if total == 0 {
        return None;
    }
    let h = key_hash(seed, &c.key);
    let mut pick = (unit(h) * total as f64) as usize;
    let mut chosen = 0;
    for (i, s) in slots.iter().enumerate() {
        if use_fit && !fits(s) {
            continue;
        }
        let w = weight(s);
        if pick < w {
            chosen = i;
            break;
        }
        pick -= w;
    }
    let s = &slots[chosen];
    let scale = ((c.size_m / f64::from(s.extent_m.max(1e-3))) as f32).clamp(s.scale[0], s.scale[1]);
    let (gc, gr) = world.guide.colrow(x, y);
    let (_, normal) = world.guide.slope_normal(gc, gr);
    let (h1, h2, h3) = (mix(h ^ 1), mix(h ^ 2), mix(h ^ 3));
    let up = tilt(aligned_up(normal, s.align), s.tilt_jitter_deg * 2.0 * unit(h2) as f32, unit(h3));
    Some(Plant {
        key: c.key,
        slot: chosen as u32,
        region,
        size_m: c.size_m,
        pos: [x, y, world.height(x, y) + f64::from(s.offset_m * scale)],
        yaw: (unit(h1) * std::f64::consts::TAU) as f32,
        scale,
        up,
    })
}

/// Calls `f` with the passes' rules on `world` (canopy at `targets`), the stand rules and the exclusion.
#[allow(clippy::too_many_arguments)] // the pass's inputs, each its own product
fn with_rules<R>(
    learned: &Learned,
    world: &World<'_>,
    fields: &dyn Fields,
    water: &WaterMask,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
    targets: &CanopyTargets,
    f: impl FnOnce(&CandidateRules<'_>, &StandRules<'_>, &(dyn Fn(f64, f64) -> bool + Sync)) -> R,
) -> R {
    let file = &learned.file;
    let names = fields.class_names();
    let suit: Vec<Suitability> = file.species.iter().map(|s| Suitability::new(s, names)).collect();
    let canopy: Vec<Suitability> =
        file.species.iter().filter(|s| s.layer == Layer::Canopy).map(|s| Suitability::new(s, names)).collect();
    let none = names.len();
    let prior_at = |s: usize, x: f64, y: f64| {
        let Some(r) = world.region(x, y) else { return 0.0 };
        let c = fields.sample(x, y).class.map_or(none, |k| usize::from(k).min(none));
        prior.get(s).and_then(|p| p.get(usize::from(r))).and_then(|p| p.get(c)).copied().unwrap_or(0.0)
    };
    let height = |x: f64, y: f64| world.height(x, y);
    let rules = CandidateRules {
        file,
        suitability: &suit,
        seed: prm.seed,
        tile_m: prm.tile_m,
        prior: &prior_at,
        water,
        height: &height,
        rocks: None,
        canopy: targets,
    };
    let stand = StandRules { values: &file.values, seed: prm.seed, canopy: &canopy };
    let excluded = |x: f64, y: f64| world.excluded(x, y);
    f(&rules, &stand, &excluded)
}

/// The canopy calibration of a pass on `world` (spec `worldgen-scatter.md` Design 4, WG-E2o.3): the class census over
/// the guide's land rect ([`World::rect`]), then the competition scale per class. The class ids are the fields'
/// class order, so [`Calibration::targets`] is the passes' [`CanopyTargets`]; its warnings go to the bake log.
#[must_use]
pub fn calibration(
    learned: &Learned,
    world: &World<'_>,
    fields: &dyn Fields,
    water: &WaterMask,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
) -> Calibration {
    // `calibrate` builds its own targets per step; the census does not read them.
    let none = CanopyTargets::uncalibrated(&learned.file, fields.class_names());
    with_rules(learned, world, fields, water, prior, prm, &none, |rules, stand, excluded| {
        let census = scatter_calib::census(rules, stand, fields, excluded, world.rect());
        scatter_calib::calibrate(rules, stand, fields, excluded, &census)
    })
}

/// The passes over `tiles` (every guide tile when `None`) on `world` with the canopy at `targets` (a
/// [`calibration`]'s): candidates, competition, layers; the kept set before the budget (what the `scatter/` product
/// holds).
#[allow(clippy::too_many_arguments)] // the pass's inputs, each its own product
pub fn winners(
    learned: &Learned,
    world: &World<'_>,
    fields: &dyn Fields,
    water: &WaterMask,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
    targets: &CanopyTargets,
    tiles: Option<&[(i32, i32)]>,
) -> LayerWinners {
    with_rules(learned, world, fields, water, prior, prm, targets, |rules, stand, excluded| {
        let all;
        let tiles = if let Some(t) = tiles {
            t
        } else {
            all = rules.tiles(world.rect());
            &all
        };
        rules.run_tiles(fields, stand, excluded, tiles)
    })
}

/// The load half of a pass on a kept set ([`budget_cut_load`]): the stand cap per tile (`prm.canopy_stems_per_ha_max`,
/// WG-E2q), then the budget (four groups; with `prm.focus`, every survivor within `prm.full_radius_m` of it placed and
/// one remainder outside, WG-E2p), then the pose. Returns
/// (plants, full count per budget group).
#[must_use]
pub fn place(learned: &Learned, world: &World<'_>, prm: &Params, won: &LayerWinners) -> (Vec<Plant>, [usize; 4]) {
    let mut full = [won.rocks.len(), 0, 0, 0];
    for c in &won.plants {
        full[1 + c.key.layer as usize] += 1;
    }
    let keep = budget_cut_load(won, prm).plants;
    let plants = won.plants.iter().zip(keep).filter(|(_, k)| *k).filter_map(|(c, _)| pose(learned, world, prm.seed, c)).collect();
    (plants, full)
}

/// The passes over `tiles` (every guide tile when `None`) on `world` at `targets`: [`winners`], then [`place`].
#[allow(clippy::too_many_arguments)] // the pass's inputs, each its own product
pub fn scatter_tiles(
    learned: &Learned,
    world: &World<'_>,
    fields: &dyn Fields,
    water: &WaterMask,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
    targets: &CanopyTargets,
    tiles: Option<&[(i32, i32)]>,
) -> (Vec<Plant>, [usize; 4]) {
    place(learned, world, prm, &winners(learned, world, fields, water, prior, prm, targets, tiles))
}

/// Version of the flora adapter's part of the product's inputs: bump it when the adapter feeds the passes
/// differently (changes to the passes' own output bump `scatter_product::FORMAT`).
// tunables: const (format)
pub const PRODUCT_VERSION: u32 = 1;

/// The guide files the pass reads besides the guide and its climate (stamps and layers: the ground height; the
/// filter; water; the clearings' sites and roads), hashed as bytes (absent and empty differ).
// tunables: const (file names)
pub const PRODUCT_FILES: [&str; 10] = [
    "stamps.json",
    "stamps.f32",
    "layers.json",
    "layers.u16",
    "layers.tint",
    WORLD_FILTER_FILE,
    "water.json",
    VALID_FILE,
    SITES_FILE,
    "roads.json",
];

/// The inputs hash of a pass on `dir`'s guide (spec `worldgen-scatter.md` "Product"): the adapter version, the guide,
/// the product fields, seed, tile size and water margin, the tile filter and settings, the learned rows and models,
/// the region prior, the source mosaic's land mask and region grid, the clearing values, the towns' mask, and the
/// [`PRODUCT_FILES`] of `dir` (so the clearings must be `dir`'s, [`guide_clearings`]).
///
/// # Errors
/// A product file that exists but cannot be read.
pub fn product_inputs(
    dir: &Path,
    world: &World<'_>,
    fields: &GridFields,
    learned: &Learned,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
) -> Result<String, DumpError> {
    let mut h = InputsHash::new();
    h.value("flora.version", f64::from(PRODUCT_VERSION))
        .guide(world.tiles.guide())
        .fields(fields)
        .seed(prm.seed)
        .value("tile_m", prm.tile_m)
        .value("water_margin_m", prm.water_margin_m)
        .bytes("tiles.filter", format!("{:?}", world.tiles.params()).as_bytes())
        .bytes("tiles.settings", format!("{:?}", world.tiles.settings()).as_bytes())
        .bytes("learned.file", format!("{:?}", learned.file).as_bytes())
        .bytes("learned.slots", format!("{:?}", learned.slots).as_bytes())
        .bytes("prior", format!("{prior:?}").as_bytes())
        .bytes("clearing.values", format!("{:?}", ClearingValues::default()).as_bytes())
        .bytes("towns", &world.towns.digest());
    let (m, r) = (world.source, world.regions);
    let mut head = Vec::with_capacity(48);
    for v in [m.cells_min.0, m.cells_min.1, r.cells_min.0, r.cells_min.1] {
        head.extend_from_slice(&v.to_le_bytes());
    }
    for v in [m.width, m.height, r.width, r.height] {
        head.extend_from_slice(&(v as u64).to_le_bytes());
    }
    h.bytes("source.head", &head).bytes("source.mask", &m.mask).bytes("source.regions", &r.index);
    for name in PRODUCT_FILES {
        match std::fs::read(dir.join(name)) {
            Ok(b) => h.bytes(&format!("file {name}"), &b),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => h.bytes(&format!("absent {name}"), &[]),
            Err(e) => return Err(DumpError::Invalid(format!("{}: {e}", dir.join(name).display()))),
        };
    }
    Ok(h.finish())
}

/// What [`scatter_cached`] returns: plants, full count per budget group, where the kept set came from, the calibration.
pub type Cached = (Vec<Plant>, [usize; 4], ProductUse, Option<Calibration>);

/// A pass on `dir`'s guide through its `scatter/` product (WG-E2j.2): the kept set read when the product is of these
/// inputs ([`product_inputs`]), else the [`calibration`] and the passes over every tile, the calibration written
/// beside the product (`scatter/calibration.toml`, WG-E2o.3) and the product (re)written; then [`place`]. Returns
/// (plants, full count per budget group, where the kept set came from, the calibration: the passes' own, or the file's
/// on a read).
///
/// # Errors
/// A product file that exists but cannot be read.
pub fn scatter_cached(
    dir: &Path,
    learned: &Learned,
    world: &World<'_>,
    fields: &GridFields,
    water: &WaterMask,
    prior: &[Vec<Vec<f64>>],
    prm: &Params,
) -> Result<Cached, DumpError> {
    let inputs = product_inputs(dir, world, fields, learned, prior, prm)?;
    let (won, used, cal) = match scatter_product::cached(dir, &inputs) {
        Ok(Some(won)) => (won, ProductUse::Read, Calibration::read(dir).ok()),
        cached => {
            let cal = calibration(learned, world, fields, water, prior, prm);
            let won = winners(learned, world, fields, water, prior, prm, &cal.targets(), None);
            // The calibration first: a product on disk always has its calibration beside it.
            let wrote = cal.write(dir).and_then(|()| scatter_product::write(dir, &inputs, &won).map(|_| ()));
            let used = match (wrote, cached) {
                (Err(e), _) => ProductUse::Unwritten(e.to_string()),
                (Ok(()), Err(e)) => ProductUse::Rebuilt(e.to_string()),
                (Ok(()), Ok(_)) => ProductUse::Wrote,
            };
            (won, used, Some(cal))
        }
    };
    let (plants, full) = place(learned, world, prm, &won);
    Ok((plants, full, used, cal))
}

/// Learns from `db`'s exterior and scatters on `tiles`' guide with `fields` and `water`, outside the stamps,
/// `clearings` and `towns`; through the guide folder `product`'s `scatter/` product when given ([`scatter_cached`];
/// `clearings` must then be its [`guide_clearings`]).
#[allow(clippy::too_many_arguments)] // the run's inputs, each its own product
pub fn scatter_world(
    db: &RecordDb,
    source: &MwWorldSource,
    bounds: Bounds<'_>,
    tiles: &GuideTiles,
    fields: &GridFields,
    water: &WaterMask,
    file: &SpeciesFile,
    clearings: Option<&Clearings>,
    towns: &TownMask,
    prm: Params,
    product: Option<&Path>,
) -> Result<FloraRun, DumpError> {
    let t0 = Instant::now();
    let m = assemble(db)?;
    let regions = region_map(db, m.cells_min, m.cells_max)?;
    let refs = source_refs(source, bounds);
    let g = tiles.guide();
    let scale = guide_scale(g, &m)?;
    let prm = Params { sea_level_m: g.sea_level_m, ..prm };
    let t1 = Instant::now();
    let learned = learn(&m, &regions, &refs, file, &prm, crate::anchor::SEA_LEVEL_M);
    let t2 = Instant::now();
    let world = World::new(&m, &regions, tiles, clearings, towns);
    let (area, cleared, in_towns) = guide_census(&world, fields, g.sea_level_m);
    let prior = region_prior(&learned.density, &area);
    let t3 = Instant::now();
    let (plants, full, product, calibration) = if let Some(dir) = product {
        scatter_cached(dir, &learned, &world, fields, water, &prior, &prm)?
    } else {
        let cal = calibration(&learned, &world, fields, water, &prior, &prm);
        let (plants, full) = scatter_tiles(&learned, &world, fields, water, &prior, &prm, &cal.targets(), None);
        (plants, full, ProductUse::Off, Some(cal))
    };
    let t4 = Instant::now();
    let region_names = regions
        .legend
        .iter()
        .map(|e| if e.name.is_empty() { e.id.clone() } else { e.name.clone() })
        .collect();
    let s = |a: Instant, b: Instant| (b - a).as_secs_f64();
    Ok(FloraRun {
        prm,
        learned,
        prior,
        area,
        sample_m2: g.spacing_m * g.spacing_m,
        full,
        plants,
        region_names,
        scale,
        stamped: tiles.stamps().is_some(),
        towns: TownCensus { excluded: in_towns, ..towns.census },
        clearings: ClearingCensus {
            sites: clearings.map_or(0, |c| c.sites),
            roads: clearings.map_or(0, |c| c.roads),
            excluded: cleared,
        },
        product,
        calibration,
        seconds: [s(t0, t1), s(t1, t2), s(t2, t3), s(t3, t4)],
    })
}

/// Namespace of the scattered placements' ids (never a content key).
// tunables: const (format: an id namespace)
const ID_NAMESPACE: &str = "mw_worldgen.flora";

/// The run's plants as placements in the engine frame (Y up, −Z north), each a copy of its model's source
/// reference (base, model, class, flags without `GROUNDED`: the height is the final ground's already); the id is
/// the candidate key's.
#[must_use]
pub fn placements(run: &FloraRun) -> Vec<Placement> {
    run.plants
        .iter()
        .map(|p| {
            let slot = &run.learned.slots[p.key.species as usize][p.slot as usize];
            let rep = &run.learned.reps[slot.rep];
            let k = &p.key;
            let mut key = Vec::with_capacity(21);
            key.push(k.layer as u8);
            for v in [k.species.to_le_bytes(), k.tile.0.to_le_bytes(), k.tile.1.to_le_bytes(), k.parent.to_le_bytes(), k.offspring.to_le_bytes()] {
                key.extend_from_slice(&v);
            }
            let up = Vec3::new(p.up[0], p.up[2], -p.up[1]);
            let rotation = Quat::from_rotation_arc(Vec3::Y, up) * Quat::from_rotation_y(p.yaw);
            Placement {
                id: PlacementId::from_key(ID_NAMESPACE, &key),
                base: rep.base.clone(),
                model: rep.model.clone(),
                transform: WorldTransform {
                    pos: WorldPos::from_f64(DVec3::new(p.pos[0], p.pos[2], -p.pos[1])),
                    rotation,
                    scale: Vec3::splat(p.scale),
                },
                class: rep.class,
                flags: rep.flags,
            }
        })
        .collect()
}

/// The inputs of a guide folder's pass: species file, product fields, water, clearings.
pub(crate) fn guide_inputs(guide: &Path, tiles: &GuideTiles, prm: &Params) -> Result<(SpeciesFile, GridFields, WaterMask, Option<Clearings>), DumpError> {
    let file = species_file(guide)?;
    let fields = product_fields(guide, tiles.guide())?;
    let water = match GuideWater::read(guide).map_err(|e| DumpError::Invalid(format!("{}: {e}", guide.display())))? {
        Some(w) => WaterMask::new(&w, prm.water_margin_m),
        None => WaterMask::sea(tiles.guide().sea_level_m, prm.water_margin_m),
    };
    Ok((file, fields, water, guide_clearings(guide)?))
}

/// The hook of `view world:<dir>` (`mw_exterior_bevy::worldgen::WorldgenContent::extra`): the flora pass at `prm`'s
/// load values (the declared defaults with the app's `--set scatter.budget`, `scatter.full_radius_m`,
/// `scatter.canopy_stems_per_ha_max` and its start
/// position as the focus, WG-E2p) on the `guide` folder's products
/// ([`species_file`], [`product_fields`], `water.json`, [`guide_clearings`]) outside the guide's `towns` (one
/// placement list per town, `bounds` covering their models), logged; an error is logged and gives nothing.
pub fn extra(
    db: &RecordDb,
    source: &MwWorldSource,
    bounds: Bounds<'_>,
    guide: &Path,
    tiles: &GuideTiles,
    towns: &[&[Placement]],
    prm: Params,
) -> Vec<(String, Vec<Placement>)> {
    let towns = TownMask::new(towns, bounds, &TownParams::default());
    let run = guide_inputs(guide, tiles, &prm).and_then(|(file, fields, water, clearings)| {
        scatter_world(db, source, bounds, tiles, &fields, &water, &file, clearings.as_ref(), &towns, prm, Some(guide))
    });
    match run {
        Ok(run) => {
            println!("worldgen flora: {}", summary_line(&run));
            if let Some(line) = full_radius_line(&run) {
                println!("worldgen flora: {line}");
            }
            for w in calibration_warnings(&run) {
                println!("worldgen flora: calibration: {w}");
            }
            vec![("flora".into(), placements(&run))]
        }
        Err(e) => {
            eprintln!("worldgen flora: {e}");
            Vec::new()
        }
    }
}

/// Placed plants per budget group ([`BUDGET_GROUPS`]; rocks 0 until the rock layer's adapter, WG-E3f).
fn placed_per_group(run: &FloraRun) -> [usize; 4] {
    let mut n = [0usize; 4];
    for p in &run.plants {
        n[1 + p.key.layer as usize] += 1;
    }
    n
}

/// The full radius's log line (WG-E2p): the placed plants within `full_radius_m` of the focus per budget group, and
/// the remainder they leave the outside set; `None` without a focus or with the radius off.
#[must_use]
pub fn full_radius_line(run: &FloraRun) -> Option<String> {
    let at = run.prm.focus.filter(|_| run.prm.full_radius_m > 0.0)?;
    let r2 = run.prm.full_radius_m * run.prm.full_radius_m;
    let mut n = [0usize; 4];
    for p in run.plants.iter().filter(|p| (p.pos[0] - at[0]).powi(2) + (p.pos[1] - at[1]).powi(2) <= r2) {
        n[1 + p.key.layer as usize] += 1;
    }
    let groups: Vec<String> = (1..4).map(|g| format!("{} {}", BUDGET_GROUPS[g], n[g])).collect();
    // Counts of instances: far under 2^52.
    #[allow(clippy::cast_precision_loss)]
    let remainder = (run.prm.budget - n.iter().sum::<usize>() as f64).max(0.0);
    Some(format!(
        "full radius {:.0} m round ({:.0}, {:.0}): inside {} ({}), remainder {remainder:.0} for the rest",
        run.prm.full_radius_m,
        at[0],
        at[1],
        n.iter().sum::<usize>(),
        groups.join(", "),
    ))
}

/// The run's calibration warnings (none when the kept set and its calibration were read).
#[must_use]
pub fn calibration_warnings(run: &FloraRun) -> &[String] {
    run.calibration.as_ref().map_or(&[], |c| &c.warnings)
}

/// `212345 placements (rocks 0 of 0, canopy 1200 of 1500, ...), 3 rows / 312 models, budget 200000, stamps,
/// clearings (...), 1 towns (...), 4.2 s`.
#[must_use]
pub fn summary_line(run: &FloraRun) -> String {
    let n = placed_per_group(run);
    let groups: Vec<String> = (0..4).map(|g| format!("{} {} of {}", BUDGET_GROUPS[g], n[g], run.full[g])).collect();
    let t = &run.towns;
    let towns = if t.towns == 0 {
        "no towns".to_owned()
    } else {
        format!(
            "{} towns ({} footprints, {} without bounds, {:.0} m² excluded)",
            t.towns, t.footprints, t.unbounded, t.excluded as f64 * run.sample_m2
        )
    };
    let c = &run.clearings;
    let clearings = if c.sites + c.roads == 0 {
        "no clearings".to_owned()
    } else {
        format!("clearings ({} sites, {} road segments, {:.0} m² excluded)", c.sites, c.roads, c.excluded as f64 * run.sample_m2)
    };
    format!(
        "{} placements ({}), {} rows / {} models, budget {:.0}, {}stamps, {clearings}, {towns}, {}, {:.1} s (source {:.1}, \
         learn {:.1}, census {:.1}, scatter {:.1})",
        run.plants.len(),
        groups.join(", "),
        run.learned.file.species.len(),
        run.learned.slots.iter().map(Vec::len).sum::<usize>(),
        run.prm.budget,
        if run.stamped { "" } else { "no " },
        run.product.label(),
        run.seconds.iter().sum::<f64>(),
        run.seconds[0],
        run.seconds[1],
        run.seconds[2],
        run.seconds[3],
    )
}

/// `flora-census.md`: totals, the rows, then per region each row's density and prior and its top models.
#[must_use]
pub fn census_md(run: &FloraRun, guide: &Path, top: usize) -> String {
    let l = &run.learned;
    let mut md = String::new();
    let _ = writeln!(md, "# Flora census ({})\n", guide.display());
    let _ = writeln!(md, "{}\n", summary_line(run));
    let _ = writeln!(
        md,
        "Source: {} flora + {} rocks references learned (rocks scatter with WG-E3f); skipped on wet ground {} flora + {} \
         rocks (never placed: the scatter keeps off water), {} outside LAND, {} without a region. Scale x{}. Seed {}, \
         tile {} m.\n",
        l.learned[0], l.learned[1], l.skipped_wet[0], l.skipped_wet[1], l.skipped_outside, l.skipped_no_region, run.scale,
        run.prm.seed, run.prm.tile_m
    );
    let mut wet: Vec<(&String, &usize)> = l.wet_species.iter().collect();
    wet.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let wet: Vec<String> = wet.iter().take(12).map(|(k, n)| format!("{k} {n}")).collect();
    let _ = writeln!(md, "Wet-skipped, most first: {}.\n", wet.join(", "));
    let n = placed_per_group(run);
    let _ = writeln!(md, "| group | before budget | placed |\n|---|---:|---:|");
    for g in 0..4 {
        let _ = writeln!(md, "| {} | {} | {} |", BUDGET_GROUPS[g], run.full[g], n[g]);
    }
    let _ = writeln!(md, "\n| row | layer | models | source refs | size m | slope e_max |\n|---|---|---:|---:|---|---:|");
    for (sp, sl) in l.file.species.iter().zip(&l.slots) {
        let e = sp.hats.slope_deg.map_or(f64::NAN, |h| h.e_max);
        let refs: usize = sl.iter().map(Slot::count).sum();
        let _ = writeln!(md, "| {} | {:?} | {} | {refs} | {:.1}-{:.1} | {e:.0} |", sp.name, sp.layer, sl.len(), sp.size_min_m, sp.size_max_m);
    }
    if let Some(cal) = &run.calibration {
        let _ = writeln!(
            md,
            "
| class | target /ha | land ha | weight mean | scale g | reached /ha | steps | ceiling |
|---|---:|---:|---:|---:|---:|---:|---|"
        );
        for c in cal.classes.iter().filter(|c| c.area_ha > 0.0) {
            let _ = writeln!(
                md,
                "| {} | {:.0} | {:.0} | {:.3} | {:.2} | {:.0} | {} | {} |",
                c.name, c.stems_per_ha, c.area_ha, c.weight_mean, c.scale, c.reached_per_ha, c.steps, c.ceiling
            );
        }
        for w in &cal.warnings {
            let _ = writeln!(md, "
Calibration: {w}.");
        }
    }
    let mut placed: BTreeMap<(u16, u32, u32), usize> = BTreeMap::new();
    for p in &run.plants {
        *placed.entry((p.region, p.key.species, p.slot)).or_default() += 1;
    }
    let _ = writeln!(
        md,
        "\nPer region: each row's learned density (per km2 of source land), its prior range over the region's classes, \
         then the top {top} models by placed count.\n"
    );
    for r in 1..run.region_names.len() {
        let land: u64 = run.area.get(r).map_or(0, |a| a.iter().sum());
        if land == 0 {
            continue;
        }
        let _ = writeln!(md, "## {} ({:.1} km2 of guide land)\n", run.region_names[r], land as f64 * run.sample_m2 / 1e6);
        let _ = writeln!(md, "| row | density /km2 | prior | placed |\n|---|---:|---|---:|");
        let mut models: Vec<(usize, usize, usize)> = Vec::new();
        for (row, sp) in l.file.species.iter().enumerate() {
            let present = |c: usize| run.area[r][c] > 0;
            let pr: Vec<f64> = (0..run.area[r].len()).filter(|&c| present(c)).map(|c| run.prior[row][r][c]).collect();
            let (lo, hi) = pr.iter().fold((f64::INFINITY, 0.0f64), |a, &p| (a.0.min(p), a.1.max(p)));
            let mut row_placed = 0;
            for (k, _) in l.slots[row].iter().enumerate() {
                let n = placed.get(&(r as u16, row as u32, k as u32)).copied().unwrap_or(0);
                row_placed += n;
                models.push((n, row, k));
            }
            let _ = writeln!(md, "| {} | {:.0} | {lo:.2}-{hi:.2} | {row_placed} |", sp.name, l.density[row][r] * 1e6);
        }
        models.sort_by_key(|m| std::cmp::Reverse(m.0));
        let _ = writeln!(md, "\n| model | row | source refs here | placed |\n|---|---|---:|---:|");
        for &(n, row, k) in models.iter().take(top) {
            let s = &l.slots[row][k];
            let _ = writeln!(md, "| {} | {} | {} | {n} |", l.reps[s.rep].base.id.as_str(), l.file.species[row].name, s.per_region[r]);
        }
        let _ = writeln!(md);
    }
    md
}

/// The `worldgen-flora` command body: the census of a guide folder, written to `<dir>/flora-census.md`.
///
/// # Errors
/// An unreadable guide folder or product, or no source mosaic.
pub fn run_census(db: &RecordDb, bounds: Bounds<'_>, guide: &Path, budget: Option<f64>) -> Result<String, DumpError> {
    let tiles = GuideTiles::open(guide, mw_world::ids::exterior())
        .map_err(|e| DumpError::Invalid(format!("{}: {e}", guide.display())))?;
    let source = MwWorldSource::build(db, &mw_world::SourceOptions::default());
    let mut prm = Params::default();
    if let Some(b) = budget {
        prm.budget = b;
    }
    let (file, fields, water, clearings) = guide_inputs(guide, &tiles, &prm)?;
    let run = scatter_world(db, &source, bounds, &tiles, &fields, &water, &file, clearings.as_ref(), &TownMask::default(), prm, None)?;
    let md = census_md(&run, guide, 8);
    std::fs::write(guide.join("flora-census.md"), &md)?;
    Ok(md)
}
