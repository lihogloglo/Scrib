//! Flora and rocks of the same map at 10× (spec `worldgen-adapter.md` "Same map at 10× (WG-X)" step 6; WG-X5):
//! the content's own nature references, learned per region and scattered again on the 10× land
//! (`sf_worldgen::scatter`).
//!
//! **What scatters:** the references [`crate::classify`] calls nature (the ones the 10× world drops): flora
//! (`/flora_`, `/f/flora` meshes, harvestable plants included) and rocks (`terrain_rock*` and large rocks), each base
//! record a species, two groups that keep their share of the budget.
//!
//! **Learning, on the source's own terrain:** land samples are the `LAND` mosaic's dry samples with a region (one
//! per 1.83 m vertex), classified by slope (central differences) and height; references by the same features at
//! their position (bilinear), their up axis from the reference rotation against the ground normal, their origin
//! height above the ground. References on wet ground (kelp, water plants) and outside any region are counted and
//! skipped: the scatter never places on water.
//!
//! **At 10×** ([`World`]): the guide's grid is the mosaic's (step 1: positions never change), so a guide point's
//! source point is the same `(col, row)`. Slope and normal come from the guide's own heights (the scaled source plus
//! the volcano uplift, the macro surface the occupancy was measured at), the height band from the source height at
//! that point (source-equivalent metres, so bands keep their meaning under the uplift), the region from the source
//! cell, the exclusion from the stamps (weight > 0: settlement footprints and bands), the guide's site and road
//! clearings (WG-V1b, [`guide_clearings`]) and the guide's own towns (no stamp: their placements' built-up area,
//! [`towns`]), the placement height from the
//! runtime tiles (`GuideTiles::height` at level 0, stamps and filter included: the ground the game draws). The
//! macro slope rather than the filtered one: the filter's detail is below the scale the source was measured at.

// Grid maths: sample counts stay far below 2^31 and heights within f32 range (the mosaic's own bounds).
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// Grid maths reads best in the usual letters (m mosaic, g grid, c col, r row, x, y).
#![allow(clippy::many_single_char_names, clippy::similar_names)]

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use glam::{DVec3, Quat, Vec3};
use mw_world::MwWorldSource;
use mw_world::convert::UNIT_METERS_F64;
use sf_coords::{WorldPos, WorldTransform};
use sf_records::RecordDb;
use sf_vfs::VfsPath;
use sf_world::{Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};
use sf_worldgen::guide::Guide;
use sf_worldgen::reach::VALID_FILE;
use sf_worldgen::roads::RoadNetwork;
use sf_worldgen::sites::SitesFile;
use sf_worldgen::sites::clearings::{ClearingValues, Clearings};
use sf_worldgen::scatter::{self, Item, Learner, Observed, Params, Scatter, Site, Species, Table, Terrain};
use sf_worldgen::tiles::GuideTiles;

use crate::classify::{Class, RefInfo, classify};

#[path = "flora_towns.rs"]
pub mod towns;
use towns::{TownCensus, TownMask, TownParams, WithTowns};
use crate::{DumpError, Mosaic, RegionMap, assemble, region_map};

/// Model bounds lookup: model VFS path → model-space `(min, max)` at unit scale (engine axes, metres).
pub type Bounds<'a> = &'a dyn Fn(&str) -> Option<([f32; 3], [f32; 3])>;

/// Group of a species (each keeps its share of the budget).
pub const GROUP_FLORA: u8 = 0;
pub const GROUP_ROCKS: u8 = 1;

/// Group names, by group index.
pub const GROUP_NAMES: [&str; 2] = ["flora", "rocks"];

/// Size of a species whose model has no bounds, metres.
// tunables: const (fallback: only the budget's priority reads it, and every Morrowind flora mesh has bounds)
const UNKNOWN_SIZE_M: f32 = 1.0;

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

/// What learning produced: the table and, per species, what its placements copy.
#[derive(Clone, Debug)]
pub struct Learned {
    pub table: Table,
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
}

/// Learns the per-region rules from `refs` on the source mosaic (`sea_m`: the source sea level).
pub fn learn(m: &Mosaic, regions: &RegionMap, refs: &[SourceRef], prm: &Params, sea_m: f64) -> Learned {
    let g = source_grid(m);
    let mut l = Learner::new(*prm, regions.legend.len(), g.spacing * g.spacing);
    for r in 0..m.height {
        for c in 0..m.width {
            let k = r * m.width + c;
            let region = region_at(m, regions, c, r);
            if m.mask[k] == 0 || region == 0 || f64::from(m.heights[k]) <= sea_m {
                continue;
            }
            let (slope, _) = g.slope_normal(c as f64, r as f64);
            l.add_land(u16::from(region), slope, m.heights[k]);
        }
    }
    let mut species: BTreeMap<&str, u32> = BTreeMap::new();
    let mut reps: Vec<SourceRef> = Vec::new();
    let mut scales: Vec<Vec<f32>> = Vec::new();
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
        if ground <= sea_m {
            wet[usize::from(s.group.min(1))] += 1;
            *wet_species.entry(s.key.clone()).or_default() += 1;
            continue;
        }
        let region = region_at(m, regions, c.floor() as usize, r.floor() as usize);
        if region == 0 {
            no_region += 1;
            continue;
        }
        let next = species.len() as u32;
        let id = *species.entry(s.key.as_str()).or_insert(next);
        if id == next {
            reps.push(s.clone());
            scales.push(Vec::new());
        }
        scales[id as usize].push(s.scale);
        learned[usize::from(s.group.min(1))] += 1;
        let (slope, normal) = g.slope_normal(c, r);
        l.add(Observed {
            species: id,
            region: u16::from(region),
            slope_deg: slope,
            band_m: ground as f32,
            scale: s.scale,
            up: s.up,
            normal,
            offset_m: (s.pos[2] - ground) as f32,
        });
    }
    let list = reps
        .iter()
        .zip(&mut scales)
        .map(|(rep, sc)| {
            sc.sort_by(f32::total_cmp);
            let typical = sc.get(sc.len() / 2).copied().unwrap_or(1.0);
            Species {
                name: rep.base.id.as_str().to_owned(),
                size_m: rep.extent_m.map_or(UNKNOWN_SIZE_M, |e| e * typical),
                group: rep.group,
            }
        })
        .collect();
    Learned {
        table: l.finish(list),
        reps,
        skipped_wet: wet,
        wet_species,
        skipped_outside: outside,
        skipped_no_region: no_region,
        learned,
    }
}

/// The 10× world as a scatter [`Terrain`] ([module docs](self)).
pub struct World<'a> {
    pub guide: Grid<'a>,
    pub source: &'a Mosaic,
    pub regions: &'a RegionMap,
    pub tiles: &'a GuideTiles,
    /// The guide's site and road clearings (WG-V1b), when it has any.
    pub clearings: Option<&'a Clearings>,
    /// Points only the clearings excluded (the stamps' are not counted).
    pub clearing_hits: AtomicUsize,
}

impl<'a> World<'a> {
    /// The 10× world on `tiles`' guide (`source` and `regions` the content's), outside `clearings` too.
    #[must_use]
    pub fn new(source: &'a Mosaic, regions: &'a RegionMap, tiles: &'a GuideTiles, clearings: Option<&'a Clearings>) -> Self {
        let g = tiles.guide();
        Self {
            guide: Grid { w: g.w, h: g.h, origin: g.origin_m, spacing: g.spacing_m, data: &g.heights },
            source,
            regions,
            tiles,
            clearings,
            clearing_hits: AtomicUsize::new(0),
        }
    }

    /// The clearings' count so far, reset to 0.
    pub fn take_clearing_hits(&self) -> usize {
        self.clearing_hits.swap(0, Ordering::Relaxed)
    }

    fn sample(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let (c, r) = self.guide.colrow(x, y);
        self.guide.inside(c, r).then_some((c, r))
    }
}

impl Terrain for World<'_> {
    fn regions_in(&self, rect: [f64; 4], out: &mut Vec<u16>) {
        let g = &self.guide;
        let (c0, r0) = g.colrow(rect[0], rect[3]);
        let (c1, r1) = g.colrow(rect[2], rect[1]);
        let clamp = |v: f64, n: usize| (v.floor().max(0.0) as usize).min(n - 1);
        let m = self.source;
        let (a, b) = (m.cell_of(clamp(c0, m.width), clamp(r0, m.height)), m.cell_of(clamp(c1, m.width), clamp(r1, m.height)));
        for gy in b.1..=a.1 {
            for gx in a.0..=b.0 {
                let r = self.regions.of((gx, gy));
                if r != 0 {
                    out.push(u16::from(r));
                }
            }
        }
    }

    fn region(&self, x: f64, y: f64) -> Option<u16> {
        let (c, r) = self.sample(x, y)?;
        let region = region_at(self.source, self.regions, c.floor() as usize, r.floor() as usize);
        (region != 0).then_some(u16::from(region))
    }

    fn site(&self, x: f64, y: f64) -> Option<Site> {
        let (c, r) = self.sample(x, y)?;
        let m = self.source;
        if m.mask[(r.round() as usize).min(m.height - 1) * m.width + (c.round() as usize).min(m.width - 1)] == 0 {
            return None;
        }
        let (slope_deg, normal) = self.guide.slope_normal(c, r);
        let band = Grid { data: &m.heights, ..self.guide }.at(c, r);
        Some(Site { slope_deg, band_m: band as f32, normal })
    }

    fn excluded(&self, x: f64, y: f64) -> bool {
        if self.tiles.stamps().and_then(|s| s.sample(x, y)).is_some_and(|(_, w)| w > 0.0) {
            return true;
        }
        let hit = self.clearings.is_some_and(|c| c.contains(x, y));
        if hit {
            self.clearing_hits.fetch_add(1, Ordering::Relaxed);
        }
        hit
    }

    fn height(&self, x: f64, y: f64) -> f64 {
        f64::from(self.tiles.height(x, -y, 0))
    }
}

/// A whole flora pass on a guide, with its census numbers.
#[derive(Clone, Debug)]
pub struct FloraRun {
    pub prm: Params,
    pub learned: Learned,
    /// Full-density expected count per `[region][entry]`.
    pub expected: Vec<Vec<f64>>,
    pub keep: Vec<f32>,
    pub items: Vec<Item>,
    pub region_names: Vec<String>,
    pub scale: f64,
    pub stamped: bool,
    /// The guide towns' exclusion.
    pub towns: TownCensus,
    /// The guide's site and road clearings.
    pub clearings: ClearingCensus,
    /// Seconds: source (mosaic, regions, references), learning, area census, scatter.
    pub seconds: [f64; 4],
}

/// The clearings of a flora run: what gave them, and the candidates only they excluded.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClearingCensus {
    /// Sites that gave a clearing (not dropped, not kept towns).
    pub sites: usize,
    /// Road segments that gave a clearing.
    pub roads: usize,
    /// Candidates the clearings excluded (not the stamps' nor the towns').
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

/// Learns from `db`'s exterior and scatters on `tiles`' guide, outside the stamps, `clearings` and `towns`.
pub fn scatter_world(
    db: &RecordDb,
    source: &MwWorldSource,
    bounds: Bounds<'_>,
    tiles: &GuideTiles,
    clearings: Option<&Clearings>,
    towns: &TownMask,
    prm: Params,
) -> Result<FloraRun, DumpError> {
    let t0 = Instant::now();
    let m = assemble(db)?;
    let regions = region_map(db, m.cells_min, m.cells_max)?;
    let refs = source_refs(source, bounds);
    let g = tiles.guide();
    let scale = guide_scale(g, &m)?;
    let prm = Params { sea_level_m: g.sea_level_m, ..prm };
    let t1 = Instant::now();
    let learned = learn(&m, &regions, &refs, &prm, crate::anchor::SEA_LEVEL_M);
    let t2 = Instant::now();
    let plain = World::new(&m, &regions, tiles, clearings);
    let rect = [
        g.origin_m[0],
        g.origin_m[1] - (g.h - 1) as f64 * g.spacing_m,
        g.origin_m[0] + (g.w - 1) as f64 * g.spacing_m,
        g.origin_m[1],
    ];
    let world = WithTowns::new(&plain, towns);
    let area = scatter::area_census(&world, &prm, rect, regions.legend.len());
    let expected = scatter::expected(&learned.table, &prm, &area);
    let keep = scatter::budget_keep(&learned.table, &prm, &expected);
    let t3 = Instant::now();
    // The census's land samples, not candidates.
    world.take_hits();
    plain.take_clearing_hits();
    let items = Scatter::new(&learned.table, keep.clone(), prm).run(&world, rect);
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
        expected,
        keep,
        items,
        region_names,
        scale,
        stamped: tiles.stamps().is_some(),
        towns: TownCensus { excluded: world.take_hits(), ..towns.census },
        clearings: ClearingCensus {
            sites: clearings.map_or(0, |c| c.sites),
            roads: clearings.map_or(0, |c| c.roads),
            excluded: plain.take_clearing_hits(),
        },
        seconds: [s(t0, t1), s(t1, t2), s(t2, t3), s(t3, t4)],
    })
}

/// Namespace of the scattered placements' ids (never a content key).
// tunables: const (format: an id namespace)
const ID_NAMESPACE: &str = "mw_worldgen.flora";

/// The run's items as placements in the engine frame (Y up, −Z north), each a copy of its species' first source
/// reference (base, model, class, flags without `GROUNDED`: the height is the final ground's already).
pub fn placements(run: &FloraRun) -> Vec<Placement> {
    run.items
        .iter()
        .map(|i| {
            let rep = &run.learned.reps[i.species as usize];
            let mut key = Vec::with_capacity(28);
            key.extend_from_slice(&i.species.to_le_bytes());
            for v in i.pos {
                key.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            let up = Vec3::new(i.up[0], i.up[2], -i.up[1]);
            let rotation = Quat::from_rotation_arc(Vec3::Y, up) * Quat::from_rotation_y(i.yaw);
            Placement {
                id: PlacementId::from_key(ID_NAMESPACE, &key),
                base: rep.base.clone(),
                model: rep.model.clone(),
                transform: WorldTransform {
                    pos: WorldPos::from_f64(DVec3::new(i.pos[0], i.pos[2], -i.pos[1])),
                    rotation,
                    scale: Vec3::splat(i.scale),
                },
                class: rep.class,
                flags: rep.flags,
            }
        })
        .collect()
}

/// The hook of `view world:<dir>` (`mw_exterior_bevy::worldgen::WorldgenContent::extra`): the flora pass at the
/// declared defaults (`budget` overrides `scatter.budget`, from `--set`), outside the clearings of the `guide` folder
/// ([`guide_clearings`]) and the guide's `towns` (one placement list per town, `bounds` covering their models),
/// logged; an error is logged and gives nothing.
pub fn extra(
    db: &RecordDb,
    source: &MwWorldSource,
    bounds: Bounds<'_>,
    guide: &Path,
    tiles: &GuideTiles,
    towns: &[&[Placement]],
    budget: Option<f64>,
) -> Vec<(String, Vec<Placement>)> {
    let mut prm = Params::default();
    if let Some(b) = budget {
        prm.budget = b;
    }
    let towns = TownMask::new(towns, bounds, &TownParams::default());
    let clearings = match guide_clearings(guide) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("worldgen flora: {e}");
            return Vec::new();
        }
    };
    match scatter_world(db, source, bounds, tiles, clearings.as_ref(), &towns, prm) {
        Ok(run) => {
            println!("worldgen flora: {}", summary_line(&run));
            vec![("flora".into(), placements(&run))]
        }
        Err(e) => {
            eprintln!("worldgen flora: {e}");
            Vec::new()
        }
    }
}

/// Items per group.
fn items_per_group(run: &FloraRun) -> [usize; 2] {
    let mut n = [0usize; 2];
    for i in &run.items {
        n[usize::from(run.learned.table.species[i.species as usize].group.min(1))] += 1;
    }
    n
}

/// Full-density expected count per group.
fn full_per_group(run: &FloraRun) -> [f64; 2] {
    let mut n = [0.0; 2];
    for (entries, exp) in run.learned.table.regions.iter().zip(&run.expected) {
        for (e, &x) in entries.iter().zip(exp) {
            n[usize::from(run.learned.table.species[e.species as usize].group.min(1))] += x;
        }
    }
    n
}

/// `212345 placements (flora 180000 of 3.1 M full, rocks ...), 312 species, budget 200000, stamps, clearings (...),
/// 1 towns (...), 4.2 s`.
pub fn summary_line(run: &FloraRun) -> String {
    let (n, full) = (items_per_group(run), full_per_group(run));
    let t = &run.towns;
    let towns = if t.towns == 0 {
        "no towns".to_owned()
    } else {
        format!(
            "{} towns ({} footprints, {} without bounds, {:.0} m² excluded, {} candidates excluded)",
            t.towns, t.footprints, t.unbounded, t.area_m2, t.excluded
        )
    };
    let c = &run.clearings;
    let clearings = if c.sites + c.roads == 0 {
        "no clearings".to_owned()
    } else {
        format!("clearings ({} sites, {} road segments, {} candidates excluded)", c.sites, c.roads, c.excluded)
    };
    format!(
        "{} placements (flora {} of {:.0} at full density, rocks {} of {:.0}), {} species, budget {:.0}, {}stamps, \
         {clearings}, {towns}, {:.1} s (source {:.1}, learn {:.1}, census {:.1}, scatter {:.1})",
        run.items.len(),
        n[0],
        full[0],
        n[1],
        full[1],
        run.learned.table.species.len(),
        run.prm.budget,
        if run.stamped { "" } else { "no " },
        run.seconds.iter().sum::<f64>(),
        run.seconds[0],
        run.seconds[1],
        run.seconds[2],
        run.seconds[3],
    )
}

/// `flora-census.md`: totals, then per region the top species by full-density count.
pub fn census_md(run: &FloraRun, guide: &Path, top: usize) -> String {
    let t = &run.learned.table;
    let mut md = String::new();
    let _ = writeln!(md, "# Flora census ({})\n", guide.display());
    let _ = writeln!(md, "{}\n", summary_line(run));
    let l = &run.learned;
    let _ = writeln!(
        md,
        "Source: {} flora + {} rocks references learned; skipped on wet ground {} flora + {} rocks (never placed: the \
         scatter keeps off water), {} outside LAND, {} without a region. Scale x{}. Seed {}, tile {} m, clump {} m x {}.\n",
        l.learned[0],
        l.learned[1],
        l.skipped_wet[0],
        l.skipped_wet[1],
        l.skipped_outside,
        l.skipped_no_region,
        run.scale,
        run.prm.seed,
        run.prm.tile_m,
        run.prm.clump_wavelength_m,
        run.prm.clump_strength
    );
    let mut wet: Vec<(&String, &usize)> = l.wet_species.iter().collect();
    wet.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let wet: Vec<String> = wet.iter().take(12).map(|(k, n)| format!("{k} {n}")).collect();
    let _ = writeln!(md, "Wet-skipped, most first: {}.
", wet.join(", "));
    let (n, full) = (items_per_group(run), full_per_group(run));
    let _ = writeln!(md, "| group | full density | placed | kept |\n|---|---:|---:|---:|");
    for g in 0..2 {
        let _ = writeln!(md, "| {} | {:.0} | {} | {:.1} % |", GROUP_NAMES[g], full[g], n[g], 100.0 * n[g] as f64 / full[g].max(1.0));
    }
    let mut placed: BTreeMap<(u16, u32), usize> = BTreeMap::new();
    for i in &run.items {
        *placed.entry((i.region, i.species)).or_default() += 1;
    }
    let _ = writeln!(
        md,
        "\nPer region: the top {top} species by full-density count. density = per m2 of source land (kept at 10x); \
         occ max = the strongest class preference; align 0 upright, 1 along the ground normal.\n"
    );
    for (r, entries) in t.regions.iter().enumerate() {
        if entries.is_empty() {
            continue;
        }
        let exp = &run.expected[r];
        let rfull: f64 = exp.iter().sum();
        let rplaced: usize = entries.iter().map(|e| placed.get(&(r as u16, e.species)).copied().unwrap_or(0)).sum();
        let _ = writeln!(
            md,
            "## {} ({} species, {} source refs, full {:.0}, placed {})\n",
            run.region_names.get(r).map_or("?", String::as_str),
            entries.len(),
            entries.iter().map(|e| e.count).sum::<usize>(),
            rfull,
            rplaced
        );
        let _ = writeln!(
            md,
            "| species | group | size m | src refs | density /km2 | occ max | max slope | align | tilt deg | full | keep | placed |\n\
             |---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
        let mut order: Vec<usize> = (0..entries.len()).collect();
        order.sort_by(|&a, &b| exp[b].total_cmp(&exp[a]));
        for &k in order.iter().take(top) {
            let e = &entries[k];
            let sp = &t.species[e.species as usize];
            let _ = writeln!(
                md,
                "| {} | {} | {:.1} | {} | {:.0} | {:.1} | {:.0} | {:.2} | {:.0} | {:.0} | {:.3} | {} |",
                sp.name,
                GROUP_NAMES[usize::from(sp.group.min(1))],
                sp.size_m,
                e.count,
                e.density_m2 * 1e6,
                e.occupancy.iter().copied().fold(0.0, f32::max),
                e.max_slope_deg,
                e.align,
                e.tilt_jitter_deg,
                exp[k],
                run.keep[e.species as usize],
                placed.get(&(r as u16, e.species)).copied().unwrap_or(0)
            );
        }
        let _ = writeln!(md);
    }
    md
}

/// The `worldgen-flora` command body: the census of a guide folder, written to `<dir>/flora-census.md`.
pub fn run_census(db: &RecordDb, bounds: Bounds<'_>, guide: &Path, budget: Option<f64>) -> Result<String, DumpError> {
    let tiles = GuideTiles::open(guide, mw_world::ids::exterior())
        .map_err(|e| DumpError::Invalid(format!("{}: {e}", guide.display())))?;
    let source = MwWorldSource::build(db, &mw_world::SourceOptions::default());
    let mut prm = Params::default();
    if let Some(b) = budget {
        prm.budget = b;
    }
    let clearings = guide_clearings(guide)?;
    let run = scatter_world(db, &source, bounds, &tiles, clearings.as_ref(), &TownMask::default(), prm)?;
    let md = census_md(&run, guide, 8);
    std::fs::write(guide.join("flora-census.md"), &md)?;
    Ok(md)
}
