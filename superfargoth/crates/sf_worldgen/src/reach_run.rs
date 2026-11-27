//! **`scrib-worldgen reach`** (spec `sites.md` "Design" flow step 3, slice WG-V1.3): a guide folder's final ground +
//! a road network + a site list → `sites.valid.json` and a census. The run around [`super::verdicts`].
//!
//! **Ground.** The final ground is the guide's runtime terrain ([`GuideTiles::open`]: guide heights, the erosion
//! filter, height stamps), sampled at every reachability cell centre at the coarsest terrain level whose spacing is
//! at most `reach.cell_m` (filter octaves under two cells are off there: a slope measured between cell centres
//! never sees detail the grid cannot hold). The grid covers the guide's extent from its sample (0, 0).
//!
//! **Water** ([`WaterMask`] with no margin): ground below the sea level, inside a lake, or inside a river band of
//! `water.json`. Its surface: the sea level, the lake's level (never below the ground), the ground in a river band
//! (a river is a shallow trough; stepping out of it is judged on the bank as on land). A river narrower than a cell
//! may fall between cell centres: it is then waded, which a character controller does too.
//!
//! **Anchors.** `anchors.json` footprints moved into the guide frame (each disc by its anchor's
//! `world_centre_m − source_centre_m`), for the re-seat anchor gap.
//!
//! **Census** ([`Census`]): per group (the site's row; kept sites by kind) placed / re-seated / dropped, kept sites
//! no path reaches, travel time p50 / p95 (nearest rank, over the sites with a travel time), and the share of land
//! cells reached on foot and only by swimming.
//!
//! **Rules** ([`read_rules`], slice WG-V1.4): the site table (`sites.toml`, spec `sites.md` "Site table") gives each
//! `[[row]]`'s post-road and re-seat rules: `slope_max_deg`, `distance_m.road` `[min, max]`, the minimum of
//! `distance_m.water` `[min, max]`, `distance_m.anchor` (a minimum gap). A key a row omits asks nothing
//! ([`SiteRule::NONE`]'s value). Every other key belongs to the placer (SW `worldgen-sites` checks the schema) and is
//! ignored here; a row with no `id`, a repeated id, a negative or reversed range are refused.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use std::time::Instant;

use sf_terrain::TileCoord;
use sf_world::WorldspaceId;

use super::{ReachGround, ReachMap, ReachValues, SiteRule, VerdictInput, declared, road_lines, sources, verdicts};
use crate::anchors::{AnchorTransform, Footprint};
use crate::filter::set;
use crate::roads::RoadNetwork;
use crate::scatter::WaterMask;
use crate::sites::{SiteKind, SitesFile, Status};
use crate::tiles::GuideTiles;
use crate::water::GuideWater;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_reach_run.rs"]
mod tests;

/// Water is the water itself: no margin around it (a definition, not a value).
// tunables: const (definition: the water edge itself)
const WATER_MARGIN_M: f64 = 0.0;
/// Finest to coarsest terrain level tried for the ground sampling.
// tunables: const (terrain level range, a layout)
const MAX_LEVEL: u8 = 15;

/// One `reach` run.
#[derive(Debug, Clone)]
pub struct ReachRequest {
    /// Guide folder (v1): heights, stamps, `water.json`, `anchors.json`.
    pub guide: PathBuf,
    /// Site list (format 1 or 2).
    pub sites: PathBuf,
    /// Road network (`roads.json` format 1).
    pub roads: PathBuf,
    /// Output site list (format 2), by convention [`super::VALID_FILE`] in the guide folder.
    pub out: PathBuf,
    /// `reach.*` overrides `(name, value)`, source Guess.
    pub sets: Vec<(String, f64)>,
    /// Rules per site row (every site that is not kept must name one).
    pub rules: BTreeMap<String, SiteRule>,
}

/// One census group's counts.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GroupCensus {
    pub placed: usize,
    pub reseated: usize,
    pub dropped: usize,
    /// Kept sites no path reaches (placed, no travel time).
    pub unreached: usize,
    /// Travel time p50 / p95, s (`None` when no site of the group has one).
    pub travel_p50_s: Option<f64>,
    pub travel_p95_s: Option<f64>,
}

/// What a run found (module docs, "Census").
#[derive(Debug, Clone, PartialEq)]
pub struct Census {
    /// Reachability grid `[w, h]` and its cell edge, m.
    pub grid: [usize; 2],
    pub cell_m: f64,
    /// Terrain level the ground was sampled at.
    pub level: u8,
    pub sources: usize,
    pub land_cells: usize,
    /// Land cells reached on foot, and reached only by swimming.
    pub land_foot: usize,
    pub land_swim_only: usize,
    /// Per group, sorted by name.
    pub groups: BTreeMap<String, GroupCensus>,
}

/// Wall-clock seconds per step (not part of the result).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Timings {
    pub ground_s: f64,
    pub flood_s: f64,
    pub verdicts_s: f64,
}

/// The site table as `reach` reads it (module docs, "Rules"): other keys are the placer's.
#[derive(serde::Deserialize)]
struct TableFile {
    #[serde(default)]
    row: Vec<TableRow>,
}

#[derive(serde::Deserialize)]
struct TableRow {
    id: Option<String>,
    slope_max_deg: Option<f64>,
    #[serde(default)]
    distance_m: TableDistances,
}

#[derive(serde::Deserialize, Default)]
struct TableDistances {
    road: Option<[f64; 2]>,
    water: Option<[f64; 2]>,
    anchor: Option<f64>,
}

/// Reads the site table at `path` into one [`SiteRule`] per row id (module docs, "Rules").
pub fn read_rules(path: &std::path::Path) -> Result<BTreeMap<String, SiteRule>> {
    rules_from_str(&std::fs::read_to_string(path)?)
        .map_err(|e| Error::Invalid(format!("site table {}: {e}", path.display())))
}

/// [`read_rules`] on the table's text.
pub fn rules_from_str(text: &str) -> Result<BTreeMap<String, SiteRule>> {
    let file: TableFile = toml::from_str(text)?;
    let mut out = BTreeMap::new();
    for (i, r) in file.row.into_iter().enumerate() {
        let id = r.id.ok_or_else(|| Error::Invalid(format!("row {}: no id", i + 1)))?;
        let bad = |what: &str| Error::Invalid(format!("row {id}: {what}"));
        let nonneg = |v: f64| v >= 0.0; // also false for NaN
        let mut rule = SiteRule::NONE;
        if let Some(s) = r.slope_max_deg {
            if !nonneg(s) {
                return Err(bad("slope_max_deg below 0"));
            }
            rule.slope_max_deg = s;
        }
        let range = |name: &str, v: [f64; 2]| {
            if nonneg(v[0]) && v[1] >= v[0] {
                Ok(v)
            } else {
                Err(bad(&format!("distance_m.{name} is not [min, max] >= 0")))
            }
        };
        if let Some(v) = r.distance_m.road {
            rule.road_m = range("road", v)?;
        }
        if let Some(v) = r.distance_m.water {
            rule.water_gap_m = range("water", v)?[0];
        }
        if let Some(a) = r.distance_m.anchor {
            if !nonneg(a) {
                return Err(bad("distance_m.anchor below 0"));
            }
            rule.anchor_gap_m = a;
        }
        if out.insert(id.clone(), rule).is_some() {
            return Err(bad("id repeated"));
        }
    }
    Ok(out)
}

/// The coarsest terrain level whose sample spacing is at most `cell_m` (level 0 when even that is coarser).
#[must_use]
pub fn ground_level(cell_m: f64) -> u8 {
    (0..=MAX_LEVEL).take_while(|&l| f64::from(TileCoord::new(l, 0, 0).spacing()) <= cell_m).last().unwrap_or(0)
}

/// The guide frame footprints of a guide's anchors (each disc moved by its anchor's rigid offset).
#[must_use]
pub fn anchor_footprints(t: &AnchorTransform) -> Vec<Footprint> {
    t.anchors
        .iter()
        .map(|a| {
            let d = [a.world_centre_m[0] - a.source_centre_m[0], a.world_centre_m[1] - a.source_centre_m[1]];
            Footprint { discs: a.footprint.discs.iter().map(|c| [c[0] + d[0], c[1] + d[1], c[2]]).collect() }
        })
        .collect()
}

/// Samples the final ground of `tiles` + `water` on the reachability grid (module docs, "Ground", "Water").
pub fn final_ground(tiles: &GuideTiles, water: &WaterMask, cell_m: f64) -> Result<(ReachGround, u8)> {
    let g = tiles.guide();
    let span = |n: usize| (n.saturating_sub(1)) as f64 * g.spacing_m;
    let cells = |n: usize| (span(n) / cell_m).floor() as usize + 1;
    let level = ground_level(cell_m);
    let ground = ReachGround::sample(g.origin_m, cells(g.w), cells(g.h), cell_m, |x, y| {
        let h = f64::from(tiles.height(x, -y, level));
        if !water.wet(x, y, h) {
            return (h, false);
        }
        let surface = match water.lake_level(x, y) {
            Some(level) => level.max(h),
            None if h < g.sea_level_m => g.sea_level_m,
            None => h,
        };
        (surface, true)
    })?;
    Ok((ground, level))
}

/// Nearest-rank percentile `q` (0..1] of sorted `v`.
fn percentile(v: &[f64], q: f64) -> Option<f64> {
    let n = v.len();
    (n > 0).then(|| v[((q * n as f64).ceil() as usize).clamp(1, n) - 1])
}

/// The census group of a site: its row, or its kind for a site with none.
fn group(s: &crate::sites::Site) -> String {
    if !s.row.is_empty() {
        return s.row.clone();
    }
    let kind = match s.kind {
        SiteKind::Town => "town",
        SiteKind::Hamlet => "hamlet",
        SiteKind::Farm => "farm",
        SiteKind::Activity => "activity",
        SiteKind::Remote => "remote",
        SiteKind::Node => "node",
    };
    format!("{}{kind}", if s.kept { "kept " } else { "" })
}

/// The census of a verdict file over its grid and floods.
#[must_use]
pub fn census(valid: &SitesFile, g: &ReachGround, map: &ReachMap, level: u8, sources: usize) -> Census {
    let mut groups: BTreeMap<String, (GroupCensus, Vec<f64>)> = BTreeMap::new();
    for s in &valid.sites {
        let (c, t) = groups.entry(group(s)).or_default();
        match s.status {
            Some(Status::Placed) => c.placed += 1,
            Some(Status::Reseated) => c.reseated += 1,
            Some(Status::Dropped) | None => c.dropped += 1,
        }
        match s.travel_s {
            Some(v) => t.push(v),
            None if s.status != Some(Status::Dropped) => c.unreached += 1,
            None => {}
        }
    }
    let groups = groups
        .into_iter()
        .map(|(k, (mut c, mut t))| {
            t.sort_by(f64::total_cmp);
            c.travel_p50_s = percentile(&t, 0.5);
            c.travel_p95_s = percentile(&t, 0.95);
            (k, c)
        })
        .collect();
    let land = (0..g.water.len()).filter(|&i| !g.water[i]);
    let (mut land_cells, mut land_foot, mut land_swim_only) = (0, 0, 0);
    for i in land {
        land_cells += 1;
        if map.foot.reached(i) {
            land_foot += 1;
        } else if map.swim.reached(i) {
            land_swim_only += 1;
        }
    }
    Census { grid: [g.w, g.h], cell_m: g.cell_m, level, sources, land_cells, land_foot, land_swim_only, groups }
}

/// A share in percent, one decimal.
fn pct(n: usize, of: usize) -> f64 {
    if of == 0 { 0.0 } else { (1000.0 * n as f64 / of as f64).round() / 10.0 }
}

impl fmt::Display for Census {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "reach grid {} x {} at {} m (terrain level {}), {} source cells; land cells {}: {} % on foot, {} % only swimming",
            self.grid[0],
            self.grid[1],
            self.cell_m,
            self.level,
            self.sources,
            self.land_cells,
            pct(self.land_foot, self.land_cells),
            pct(self.land_swim_only, self.land_cells),
        )?;
        let t = |v: Option<f64>| v.map_or_else(|| "-".to_string(), |s| format!("{s:.0}"));
        for (k, c) in &self.groups {
            writeln!(
                f,
                "  {k}: placed {}, reseated {}, dropped {}, unreached {}; travel p50 {} s, p95 {} s",
                c.placed,
                c.reseated,
                c.dropped,
                c.unreached,
                t(c.travel_p50_s),
                t(c.travel_p95_s)
            )?;
        }
        Ok(())
    }
}

/// Runs `req`: writes `req.out` and returns the verdict file, its census and the step timings.
pub fn run_reach(req: &ReachRequest) -> Result<(SitesFile, Census, Timings)> {
    let mut d = declared();
    for (k, v) in &req.sets {
        if !k.starts_with("reach.") {
            return Err(Error::Invalid(format!("--set {k}: not a reach value")));
        }
        set(&mut d, k, *v, None)?;
    }
    let v = ReachValues::from_declared(&d)?;
    let file = SitesFile::read(&req.sites)?;
    let net = RoadNetwork::from_reader(std::io::BufReader::new(std::fs::File::open(&req.roads)?))?;
    let anchors = AnchorTransform::read(&req.guide)?.map(|t| anchor_footprints(&t)).unwrap_or_default();
    let tiles = GuideTiles::open(&req.guide, WorldspaceId::new("reach"))?;
    let water = match GuideWater::read(&req.guide)? {
        Some(w) => WaterMask::new(&w, WATER_MARGIN_M),
        None => WaterMask::sea(tiles.guide().sea_level_m, WATER_MARGIN_M),
    };

    let t0 = Instant::now();
    let (ground, level) = final_ground(&tiles, &water, v.cell_m)?;
    let t1 = Instant::now();
    let roads = road_lines(&net, v.cell_m);
    let src = sources(&ground, &file.sites, &roads);
    let map = ReachMap::flood(&ground, &src, &v)?;
    let t2 = Instant::now();
    let inp = VerdictInput { ground: &ground, map: &map, roads: &roads, anchors: &anchors, rules: &req.rules };
    let valid = verdicts(&inp, &v, &file)?;
    let t3 = Instant::now();
    if let Some(dir) = req.out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir)?;
    }
    valid.write(&req.out)?;
    let census = census(&valid, &ground, &map, level, src.len());
    let s = |a: Instant, b: Instant| (b - a).as_secs_f64();
    Ok((valid, census, Timings { ground_s: s(t0, t1), flood_s: s(t1, t2), verdicts_s: s(t2, t3) }))
}
