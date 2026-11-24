//! `worldgen-site` (ST-1, scrib-worldgen `docs/specs/settlements.md` "Contract: files only", site folder format 1):
//! the vanilla exterior around one named settlement anchor as a **site folder** for the settlement tool, which reads
//! the files and links nothing of ours.
//!
//! - `heights.f32`: the `LAND` mosaic ([`crate::dump`]) cropped to the square of half edge `radius_m` around the
//!   anchor's centre, at the native vertex spacing (128 units), metres, little-endian, row-major, row 0 = north.
//!   Raster cell `(c, r)` is centred on mosaic vertex `(col0 + c, row0 + r)`, so `origin_m` (the north-west corner of
//!   cell (0,0)) is that vertex's position plus half a cell west and north.
//! - `water.u8`: 1 where the ground is below the exterior water level ([`anchor::SEA_LEVEL_M`]).
//! - `roads.u8`: 1 where the vertex's `VTEX` texel ([`crate::layers::source_texels`]) is a road or path `LTEX`,
//!   as listed in `bevyrrowind/data/worldgen-site.toml` ([`SiteTable`]).
//! - `site.json`: the header, `centre_m` = the anchor's centre ([`anchor::centre`]), and the claim: one polygon per
//!   human-made member of the anchor, the convex hull (counter-clockwise) of its model's visible vertices,
//!   world-transformed and projected on the ground; a member without vertices gets the square of its anchor disc.
//!   `kind` is `other` for every footprint: [`crate::classify`] tells human-made from nature, not building from wall
//!   or bridge. No doors (Milestone 2).
//!
//! The anchor is found as `worldgen-x10` builds it (same classification and clustering at
//! [`SiteParams::anchor_scale`]), so its name and members match `anchors-census.md`.
//!
//! **From a 10x guide (ST-2, `--guide <dir>`, [`run_guide`]):** a new town founded in the generated world, no claim.
//! - Heights: the game's own CPU tiles ([`GuideTiles::open`], level 0, the shown `worldgen.sharp` scale passed in),
//!   with the named anchor's stamp weights set to 0 ([`guide_tiles`]): its stamp (and lift) is gone, every other
//!   stamp stays, the stamp indices (and the layers' stamp grids) are unchanged. Sampled at cell centres
//!   ([`SiteParams::cell_m`]) on a square of half edge [`SiteParams::guide_radius_m`] around `centre_m`.
//! - `water.u8`: the generated world shows open water only, one plane at the guide's sea level (`meta.json`
//!   `sea_level_m`; `world.rs` sets the exterior's water to plain open water): 1 where the ground is below it.
//! - `centre_m` ([`find_river`]): the river reach nearest the anchor's mapped centre (`anchors.json`
//!   `world_centre_m`): the nearest wet sample of a water body of at least [`SiteParams::river_min_area_m2`] within
//!   [`SiteParams::river_search_m`] (guide spacing), moved up the in-water distance to the channel's middle.
//! - `roads.u8`: 1 where the ground layer the game draws there ([`sf_worldgen::layers`]: the grid of the stamp of
//!   weight over one half, else the global grid; the covering texel, no blend) is a road in `worldgen-site.toml`;
//!   no file when the guide has no layers.

// Grid maths: sample counts stay far below 2^31 and coordinates within a few hundred km (as in sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use glam::Vec3;
use sf_worldgen::anchors::AnchorTransform;
use sf_worldgen::layers::{GuideLayerGrid, STAMP_LAYER_WEIGHT};
use sf_worldgen::stamps::{Stamp, StampIndex, StampSet};
use sf_worldgen::tiles::GuideTiles;
use mw_world::convert::UNIT_METERS_F64;
use mw_world::{MwWorldSource, SourceOptions};
use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordId};
use sf_worldgen::filter::Declared;
use sf_worldgen::layers::NO_LAYER;
use tes3::esp::LandscapeTexture;

use crate::anchor::{self, Field, Member, Params};
use crate::x10::{self, Bounds};
use crate::{Dump, DumpError, dump};

/// The site folder format this module writes and reads.
// tunables: const (format version of the contract)
pub const FORMAT: u32 = 1;
/// File names of the site folder (contract).
pub const SITE_FILE: &str = "site.json";
pub const HEIGHTS_FILE: &str = "heights.f32";
pub const WATER_FILE: &str = "water.u8";
pub const ROADS_FILE: &str = "roads.u8";

/// The shipped road table, `bevyrrowind/data/worldgen-site.toml`.
pub const BUILTIN: &str = include_str!("../../../data/worldgen-site.toml");

/// Model visible vertices lookup: model VFS path → model-space points at unit scale (engine axes, metres).
pub type Points<'a> = &'a dyn Fn(&str) -> Option<Vec<[f32; 3]>>;

/// The declared site values (spec `tunables.md` "Definition").
pub fn declared() -> Vec<Declared> {
    vec![
        Declared {
            name: "site.radius_m",
            v: 1200.0,
            unit: "m",
            min: 50.0,
            max: 20_000.0,
            source: "Guess".into(),
            help: "half edge of the site crop around the anchor's centre (Balmora's footprint is ~700 m across; the \
                   crop keeps land around it to grow on)",
        },
        Declared {
            name: "site.anchor_scale",
            v: 10.0,
            unit: "ratio",
            min: 1.0,
            max: 100.0,
            source: "Reference(worldgen-x10's default scale: anchors cluster as in anchors-census.md)".into(),
            help: "scale the anchors are clustered at, so a site's anchor is the same set of members as in the guide",
        },
        Declared {
            name: "site.guide_radius_m",
            v: 1500.0,
            unit: "m",
            min: 50.0,
            max: 20_000.0,
            source: "Guess".into(),
            help: "half edge of a 10x-guide site crop around its centre: vanilla Balmora's claim spans 346 x 187 m, a town                    of 10x its built area ~1.1 x 0.6 km, so a 3 km square keeps ~1 km of ground around it on both banks",
        },
        Declared {
            name: "site.cell_m",
            v: 1.8288,
            unit: "m",
            min: 0.25,
            max: 50.0,
            source: "Reference(the vanilla LAND vertex spacing, 128 units: the spacing of ST-1's vanilla sites)".into(),
            help: "raster spacing of a 10x-guide site (heights are sampled at the cell centres)",
        },
        Declared {
            name: "site.river_search_m",
            v: 4000.0,
            unit: "m",
            min: 100.0,
            max: 50_000.0,
            source: "Guess".into(),
            help: "half edge of the square searched for the river nearest the anchor's mapped centre (10x guide)",
        },
        Declared {
            name: "site.river_min_area_m2",
            v: 50_000.0,
            unit: "m2",
            min: 0.0,
            max: 1.0e8,
            source: "Guess".into(),
            help: "smallest water body (in the search window) the site centre may sit on: smaller ones are ponds",
        },
    ]
}

/// The site values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SiteParams {
    pub radius_m: f64,
    pub anchor_scale: f64,
    pub anchor: Params,
    pub guide_radius_m: f64,
    pub cell_m: f64,
    pub river_search_m: f64,
    pub river_min_area_m2: f64,
}

impl Default for SiteParams {
    fn default() -> Self {
        let v = |name: &str| declared().into_iter().find(|d| d.name == name).map_or(f64::NAN, |d| d.v);
        Self {
            radius_m: v("site.radius_m"),
            anchor_scale: v("site.anchor_scale"),
            anchor: Params::default(),
            guide_radius_m: v("site.guide_radius_m"),
            cell_m: v("site.cell_m"),
            river_search_m: v("site.river_search_m"),
            river_min_area_m2: v("site.river_min_area_m2"),
        }
    }
}

/// Which ground textures are roads or paths (`worldgen-site.toml` `[roads]`).
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct RoadTable {
    /// `LTEX` editor ids, ASCII case ignored.
    #[serde(default)]
    pub ltex_ids: Vec<String>,
    /// Lower-case substrings of the `LTEX` texture file name.
    #[serde(default)]
    pub texture_patterns: Vec<String>,
}

impl RoadTable {
    /// Whether the `LTEX` `id` with texture `file` is a road or path.
    pub fn is_road(&self, id: &str, file: &str) -> bool {
        let file = file.to_ascii_lowercase();
        self.ltex_ids.iter().any(|i| i.eq_ignore_ascii_case(id))
            || self.texture_patterns.iter().any(|p| file.contains(&p.to_ascii_lowercase()))
    }
}

/// `worldgen-site.toml`.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct SiteTable {
    pub roads: RoadTable,
}

impl SiteTable {
    pub fn from_toml(text: &str) -> Result<Self, DumpError> {
        toml::from_str(text).map_err(|e| DumpError::Invalid(format!("worldgen-site.toml: {e}")))
    }

    /// The shipped table ([`BUILTIN`]).
    pub fn builtin() -> Result<Self, DumpError> {
        Self::from_toml(BUILTIN)
    }
}

/// A footprint's kind (contract).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FootprintKind {
    Building,
    Wall,
    Bridge,
    Stairs,
    Other,
}

/// One kept reference's ground footprint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FootprintPolygon {
    /// The reference key.
    pub id: String,
    pub kind: FootprintKind,
    /// Counter-clockwise, metres, x east, y north, not closed (last ≠ first).
    pub polygon: Vec<[f64; 2]>,
}

/// The kept town (arm A).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub footprints: Vec<FootprintPolygon>,
}

/// `site.json` (contract, format 1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteJson {
    pub format: u32,
    pub name: String,
    pub producer: String,
    /// World position of the north-west corner of raster cell (0, 0), metres.
    pub origin_m: [f64; 2],
    /// Raster spacing, metres.
    pub cell_m: f64,
    /// `[w, h]`.
    pub dims: [usize; 2],
    pub water_level_m: f64,
    pub centre_m: [f64; 2],
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<Claim>,
    /// blake3 (hex) of each raster file, by file name.
    pub blake3: BTreeMap<String, String>,
}

/// A site in memory: the header (its `blake3` is filled by [`write`]) and the rasters.
#[derive(Clone, Debug, PartialEq)]
pub struct Site {
    pub json: SiteJson,
    pub heights: Vec<f32>,
    pub water: Vec<u8>,
    pub roads: Option<Vec<u8>>,
}

fn le_bytes(h: &[f32]) -> Vec<u8> {
    h.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// Writes `site` into `dir` (created); returns the written header (with the rasters' blake3).
pub fn write(site: &Site, dir: &Path) -> Result<SiteJson, DumpError> {
    let [w, h] = site.json.dims;
    let n = w * h;
    let lens_ok = site.heights.len() == n && site.water.len() == n && site.roads.as_ref().is_none_or(|r| r.len() == n);
    if !lens_ok {
        return Err(DumpError::Invalid(format!("site rasters do not match dims {w} x {h}")));
    }
    std::fs::create_dir_all(dir)?;
    let mut json = site.json.clone();
    json.blake3.clear();
    let mut put = |name: &str, bytes: &[u8]| -> Result<(), DumpError> {
        std::fs::write(dir.join(name), bytes)?;
        json.blake3.insert(name.to_owned(), hash(bytes));
        Ok(())
    };
    put(HEIGHTS_FILE, &le_bytes(&site.heights))?;
    put(WATER_FILE, &site.water)?;
    if let Some(roads) = &site.roads {
        put(ROADS_FILE, roads)?;
    } else {
        let _ = std::fs::remove_file(dir.join(ROADS_FILE));
    }
    std::fs::write(dir.join(SITE_FILE), serde_json::to_vec_pretty(&json)?)?;
    Ok(json)
}

/// Reads a site folder, checking the format, each raster's length and its blake3.
pub fn read(dir: &Path) -> Result<Site, DumpError> {
    let json: SiteJson = serde_json::from_slice(&std::fs::read(dir.join(SITE_FILE))?)?;
    if json.format != FORMAT {
        return Err(DumpError::Invalid(format!("site format {} (expected {FORMAT})", json.format)));
    }
    let n = json.dims[0] * json.dims[1];
    let raw = |name: &str, bpp: usize| -> Result<Option<Vec<u8>>, DumpError> {
        let Some(want) = json.blake3.get(name) else { return Ok(None) };
        let bytes = std::fs::read(dir.join(name))?;
        if bytes.len() != n * bpp {
            return Err(DumpError::Invalid(format!("{name}: {} bytes, expected {}", bytes.len(), n * bpp)));
        }
        if &hash(&bytes) != want {
            return Err(DumpError::Invalid(format!("{name}: blake3 mismatch")));
        }
        Ok(Some(bytes))
    };
    let missing = |name: &str| DumpError::Invalid(format!("{name}: not in site.json blake3"));
    let heights = raw(HEIGHTS_FILE, 4)?.ok_or_else(|| missing(HEIGHTS_FILE))?;
    let heights = heights.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect();
    let water = raw(WATER_FILE, 1)?.ok_or_else(|| missing(WATER_FILE))?;
    let roads = raw(ROADS_FILE, 1)?;
    Ok(Site { json, heights, water, roads })
}

/// A square crop of a field: first column and row, width, height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    pub col0: usize,
    pub row0: usize,
    pub w: usize,
    pub h: usize,
}

/// The samples within `radius_m` (square, per axis) of the sample nearest `centre`, clamped to the field.
pub fn crop_around(f: &Field<'_>, centre: [f64; 2], radius_m: f64) -> Option<Crop> {
    let c = ((centre[0] - f.origin_m[0]) / f.spacing_m).round() as i64;
    let r = ((f.origin_m[1] - centre[1]) / f.spacing_m).round() as i64;
    let half = (radius_m / f.spacing_m).round().max(0.0) as i64;
    let (c0, c1) = ((c - half).max(0), (c + half).min(f.w as i64 - 1));
    let (r0, r1) = ((r - half).max(0), (r + half).min(f.h as i64 - 1));
    (c0 <= c1 && r0 <= r1).then(|| Crop { col0: c0 as usize, row0: r0 as usize, w: (c1 - c0 + 1) as usize, h: (r1 - r0 + 1) as usize })
}

/// Convex hull (Andrew's monotone chain), counter-clockwise, collinear points dropped; `None` below 3 corners.
pub fn convex_hull(points: &[[f64; 2]]) -> Option<Vec<[f64; 2]>> {
    let mut p: Vec<[f64; 2]> = points.to_vec();
    p.sort_by(|a, b| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])));
    p.dedup();
    if p.len() < 3 {
        return None;
    }
    let cross = |o: [f64; 2], a: [f64; 2], b: [f64; 2]| (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    let mut hull: Vec<[f64; 2]> = Vec::with_capacity(2 * p.len());
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &[f64; 2]>> = if pass == 0 { Box::new(p.iter()) } else { Box::new(p.iter().rev()) };
        for &q in iter {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    (hull.len() >= 3).then_some(hull)
}

/// The axis-aligned square of half edge `r` around `xy`, counter-clockwise.
fn square(xy: [f64; 2], r: f64) -> Vec<[f64; 2]> {
    let [x, y] = xy;
    vec![[x - r, y - r], [x + r, y - r], [x + r, y + r], [x - r, y + r]]
}

/// The command's arguments.
#[derive(Clone, Debug)]
pub struct SiteArgs {
    /// Anchor name as `anchors-census.md` gives it (ASCII case ignored).
    pub anchor: String,
    pub out: PathBuf,
    pub params: SiteParams,
    pub table: SiteTable,
    /// The `worldgen.sharp` scale the game shows a guide at (`FilterParams::with_sharp_scale`; [`run_guide`] only).
    pub sharp_scale: f32,
}

/// The named anchor as `worldgen-x10` builds it: its name, source-frame centre and member keys.
struct Found {
    name: String,
    centre: [f64; 2],
    keys: Vec<String>,
}

fn find_anchor(dumped: &Dump, src: &Field<'_>, members: &[Member], params: &SiteParams, query: &str) -> Result<Found, DumpError> {
    let s = params.anchor_scale;
    let scaled = x10::scale_heights(src.data, s);
    let fin = Field { origin_m: [src.origin_m[0] * s, src.origin_m[1] * s], spacing_m: src.spacing_m * s, data: &scaled, ..*src };
    let built = x10::build_anchors(members, src, &fin, s, &params.anchor, &dumped.markers);
    let Some(b) = built.iter().find(|b| b.place.name.eq_ignore_ascii_case(query)) else {
        let names: Vec<&str> = built.iter().take(20).map(|b| b.place.name.as_str()).collect();
        return Err(DumpError::Invalid(format!("no anchor named {query:?}; the largest: {}", names.join(", "))));
    };
    Ok(Found { name: b.place.name.clone(), centre: b.place.source_centre_m, keys: b.place.members.clone() })
}

/// One footprint per member of `keys` (module docs), sorted by id; and how many fell back to a disc square.
fn footprints_of(
    source: &MwWorldSource,
    keys: &[String],
    members: &[Member],
    points: Points<'_>,
    params: &Params,
) -> (Vec<FootprintPolygon>, usize) {
    let by_key: HashMap<&str, &Member> = members.iter().map(|m| (m.key.as_str(), m)).collect();
    let keys: HashSet<&str> = keys.iter().map(String::as_str).collect();
    let mut footprints = Vec::with_capacity(keys.len());
    let mut fallback = 0;
    for pl in source.all_placements(&mw_world::ids::exterior()) {
        let key = pl.id.to_string();
        if !keys.contains(key.as_str()) {
            continue;
        }
        let tf = &pl.transform;
        let pos = tf.pos.to_f64();
        let ground: Vec<[f64; 2]> = pl
            .model
            .as_ref()
            .and_then(|model| points(model.as_str()))
            .unwrap_or_default()
            .into_iter()
            .map(|v| {
                let world = tf.rotation * (tf.scale * Vec3::from_array(v));
                [pos.x + f64::from(world.x), -(pos.z + f64::from(world.z))]
            })
            .collect();
        let polygon = convex_hull(&ground).unwrap_or_else(|| {
            fallback += 1;
            let radius = by_key.get(key.as_str()).map_or(params.default_radius_m, |m| m.r.min(params.max_radius_m));
            square([pos.x, -pos.z], radius)
        });
        footprints.push(FootprintPolygon { id: key, kind: FootprintKind::Other, polygon });
    }
    footprints.sort_by(|a, b| a.id.cmp(&b.id));
    (footprints, fallback)
}

/// The cropped rasters and what the report prints about them.
struct Rasters {
    heights: Vec<f32>,
    water: Vec<u8>,
    roads: Vec<u8>,
    /// Wet cells west, east, north, south of the centre sample: over the crop, and inside the town box.
    sides: [usize; 4],
    town_sides: [usize; 4],
    /// Ground textures of the crop: cells, LTEX id, texture file, road.
    textures: Vec<(usize, String, String, bool)>,
}

/// `[min_x, min_y, max_x, max_y]` of the footprints.
fn bounds_of(footprints: &[FootprintPolygon]) -> [f64; 4] {
    footprints.iter().flat_map(|f| &f.polygon).fold(
        [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY],
        |b, p| [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])],
    )
}

fn rasters(db: &RecordDb, src: &Field<'_>, crop: Crop, centre: [f64; 2], town: [f64; 4], table: &RoadTable) -> Result<Rasters, DumpError> {
    let sp = src.spacing_m;
    let texels = crate::layers::source_texels(db)?;
    let ltex_file = |id: &str| db.get::<LandscapeTexture>(&RecordId::new(id)).map_or_else(String::new, |l| l.file_name.clone());
    let road_of: Vec<bool> = texels.palette.iter().map(|id| table.is_road(id, &ltex_file(id))).collect();
    let mut tex_count: BTreeMap<u16, usize> = BTreeMap::new();
    let n = crop.w * crop.h;
    let mut out = Rasters {
        heights: Vec::with_capacity(n),
        water: Vec::with_capacity(n),
        roads: Vec::with_capacity(n),
        sides: [0; 4],
        town_sides: [0; 4],
        textures: Vec::new(),
    };
    let centre_cell = [((centre[0] - src.origin_m[0]) / sp).round() as i64, ((src.origin_m[1] - centre[1]) / sp).round() as i64];
    // A vertex on a texel edge (the vanilla quarter-texel shift puts every 4th vertex there) takes the texel east
    // and south of it, consistently.
    let edge_bias = 1e-6;
    for row in crop.row0..crop.row0 + crop.h {
        for col in crop.col0..crop.col0 + crop.w {
            let height = src.data[row * src.w + col];
            out.heights.push(height);
            let wet = f64::from(height) < anchor::SEA_LEVEL_M;
            out.water.push(u8::from(wet));
            let (x, y) = (src.origin_m[0] + col as f64 * sp, src.origin_m[1] - row as f64 * sp);
            if wet {
                let (dc, dr) = (col as i64 - centre_cell[0], row as i64 - centre_cell[1]);
                let in_town = x >= town[0] && x <= town[2] && y >= town[1] && y <= town[3];
                for (i, on) in [dc < 0, dc > 0, dr < 0, dr > 0].into_iter().enumerate() {
                    out.sides[i] += usize::from(on);
                    out.town_sides[i] += usize::from(on && in_town);
                }
            }
            let tc = ((x - texels.min_xz[0]) / texels.spacing_m + edge_bias).floor() as i64;
            let tr = ((-y - texels.min_xz[1]) / texels.spacing_m + edge_bias).floor() as i64;
            let inside = tc >= 0 && tr >= 0 && (tc as usize) < texels.dims[0] && (tr as usize) < texels.dims[1];
            let tex = if inside { texels.texels[tr as usize * texels.dims[0] + tc as usize] } else { NO_LAYER };
            *tex_count.entry(tex).or_default() += 1;
            out.roads.push(u8::from(tex != NO_LAYER && road_of.get(usize::from(tex)).copied().unwrap_or(false)));
        }
    }
    out.textures = tex_count
        .into_iter()
        .map(|(tex, cells)| {
            if tex == NO_LAYER {
                (cells, "(no LAND)".to_owned(), String::new(), false)
            } else {
                let id = texels.palette[usize::from(tex)].clone();
                let file = ltex_file(&id);
                (cells, id, file, road_of[usize::from(tex)])
            }
        })
        .collect();
    out.textures.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    Ok(out)
}

fn ones(v: &[u8]) -> usize {
    v.iter().map(|&b| usize::from(b == 1)).sum()
}

/// The `worldgen-site` command body; returns the printed report.
pub fn run(db: &RecordDb, bounds: Bounds<'_>, points: Points<'_>, args: &SiteArgs) -> Result<String, DumpError> {
    let started = Instant::now();
    let params = &args.params;
    let dumped = dump(db, 100.0)?;
    let mosaic = &dumped.mosaic;
    let sp = f64::from(mw_world::terrain::VERTEX_UNITS) * UNIT_METERS_F64;
    let src = Field { w: mosaic.width, h: mosaic.height, origin_m: x10::source_origin_m(mosaic), spacing_m: sp, data: &mosaic.heights };
    let sea = anchor::open_sea(&src, params.anchor.open_water_radius_m);
    let source = MwWorldSource::build(db, &SourceOptions::default());
    let refs = x10::exterior_refs(&source, bounds);
    let (members, _) = x10::members_of(&refs, &src, &sea, &params.anchor);
    let found = find_anchor(&dumped, &src, &members, params, &args.anchor)?;
    let (footprints, fallback) = footprints_of(&source, &found.keys, &members, points, &params.anchor);
    let town = bounds_of(&footprints);
    let crop = crop_around(&src, found.centre, params.radius_m).ok_or_else(|| DumpError::Invalid("empty crop".into()))?;
    let ras = rasters(db, &src, crop, found.centre, town, &args.table.roads)?;
    let name = found.name;
    let nw = [src.origin_m[0] + crop.col0 as f64 * sp, src.origin_m[1] - crop.row0 as f64 * sp];
    let json = SiteJson {
        format: FORMAT,
        name: name.clone(),
        producer: "bevyrrowind worldgen-site (mw_worldgen::site, ST-1)".into(),
        origin_m: [nw[0] - 0.5 * sp, nw[1] + 0.5 * sp],
        cell_m: sp,
        dims: [crop.w, crop.h],
        water_level_m: anchor::SEA_LEVEL_M,
        centre_m: found.centre,
        source: format!("{name}, exterior of {}", dumped.plugins.join(" + ")),
        claim: Some(Claim { footprints }),
        blake3: BTreeMap::new(),
    };
    let (road_cells, water_cells) = (ones(&ras.roads), ones(&ras.water));
    let site = Site { json, heights: ras.heights, water: ras.water, roads: Some(ras.roads) };
    let json = write(&site, &args.out)?;

    let mut out = String::new();
    let fp = json.claim.as_ref().map_or(0, |c| c.footprints.len());
    let [cx, cy] = found.centre;
    let _ = writeln!(out, "worldgen-site: {name} ({} members, centre [{cx:.1}, {cy:.1}] m)", found.keys.len());
    let _ = writeln!(out, "  footprints: {fp} ({} convex hulls, {fallback} disc squares: no model vertices)", fp - fallback);
    let _ = writeln!(out, "  crop: {} x {} cells at {sp:.4} m, origin [{:.1}, {:.1}] m", crop.w, crop.h, json.origin_m[0], json.origin_m[1]);
    let [west, east, north, south] = ras.sides;
    let _ = writeln!(out, "  water cells: {water_cells} (west of the centre {west}, east {east}, north {north}, south {south})");
    let [west, east, north, south] = ras.town_sides;
    let _ = writeln!(
        out,
        "  water cells inside the claim's bounds [{:.1}, {:.1}]..[{:.1}, {:.1}] m: west of the centre {west}, east {east}, \
         north {north}, south {south}",
        town[0], town[1], town[2], town[3]
    );
    let _ = writeln!(out, "  road cells: {road_cells}");
    let _ = writeln!(out, "  ground textures in the crop (cells, road?):");
    for (cells, id, file, road) in &ras.textures {
        let _ = writeln!(out, "    {cells:>8}  {}  {id} ({file})", if *road { "road" } else { "    " });
    }
    let _ = writeln!(out, "wrote {} in {:.1} s", args.out.display(), started.elapsed().as_secs_f64());
    Ok(out)
}

// ---- ST-2: a site from a 10x guide (module docs) ----

/// Radius of the reported slope median: ST-3d measured vanilla Balmora's within 300 m (0.49).
// tunables: const (report: the ST-3d measurement's radius, so the numbers compare)
pub const SLOPE_REPORT_RADIUS_M: f64 = 300.0;

/// 8-neighbourhood, orthogonal steps first (the river ascent prefers them on a tie).
// tunables: const (maths: grid neighbourhood)
const NEIGHBOURS: [(i64, i64); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

/// The guide's tiles as the game shows them (`world.rs`: [`GuideTiles::open`], then the `worldgen.sharp` scale),
/// with the weights of stamp `drop` set to 0: that stamp no longer moves the ground, every other stamp and every
/// stamp index stay. Returns the dropped stamp as it was.
pub fn guide_tiles(dir: &Path, drop: Option<usize>, sharp_scale: f32) -> Result<(GuideTiles, Option<Stamp>), DumpError> {
    let err = |e: sf_worldgen::Error| DumpError::Invalid(format!("{}: {e}", dir.display()));
    let opened = GuideTiles::open(dir, mw_world::ids::exterior()).map_err(err)?;
    let tiles = opened.with_params(opened.params().with_sharp_scale(sharp_scale));
    let Some(i) = drop else { return Ok((tiles, None)) };
    let mut set = StampSet::read(dir).map_err(err)?.unwrap_or_default();
    let Some(stamp) = set.stamps.get_mut(i) else {
        return Err(DumpError::Invalid(format!("{}: no stamp {i} in stamps.json", dir.display())));
    };
    let dropped = stamp.clone();
    stamp.weights.fill(0.0);
    Ok((tiles.with_stamps(set), Some(dropped)))
}

/// A square raster of `2k + 1` cells a side centred on a point, guide frame (x east, y north), row 0 = north.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub centre: [f64; 2],
    pub cell_m: f64,
    pub k: usize,
}

impl Square {
    /// The square of half edge about `radius_m` (rounded to whole cells) around `centre`.
    pub fn new(centre: [f64; 2], radius_m: f64, cell_m: f64) -> Self {
        Self { centre, cell_m, k: (radius_m / cell_m).round().max(0.0) as usize }
    }

    /// Cells a side.
    pub fn n(&self) -> usize {
        2 * self.k + 1
    }

    /// Centre of cell `(c, r)`.
    pub fn at(&self, c: usize, r: usize) -> [f64; 2] {
        let k = self.k as f64;
        [self.centre[0] + (c as f64 - k) * self.cell_m, self.centre[1] - (r as f64 - k) * self.cell_m]
    }

    /// North-west corner of cell (0, 0).
    pub fn origin_m(&self) -> [f64; 2] {
        let e = (self.k as f64 + 0.5) * self.cell_m;
        [self.centre[0] - e, self.centre[1] + e]
    }
}

/// Level-0 tile heights (the finest the game draws) at `at(c, r)` (guide frame) for a `w x h` raster, row-major,
/// rows split over the cores.
fn heights_at(tiles: &GuideTiles, w: usize, h: usize, at: impl Fn(usize, usize) -> [f64; 2] + Sync) -> Vec<f32> {
    let mut out = vec![0.0f32; w * h];
    if out.is_empty() {
        return out;
    }
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let rows = h.div_ceil(threads).max(1);
    let at = &at;
    std::thread::scope(|scope| {
        for (part, chunk) in out.chunks_mut(rows * w).enumerate() {
            scope.spawn(move || {
                for (i, v) in chunk.iter_mut().enumerate() {
                    let [x, y] = at(i % w, part * rows + i / w);
                    *v = tiles.height(x, -y, 0);
                }
            });
        }
    });
    out
}

/// Where a site centres on a river ([`find_river`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct River {
    /// The channel's middle: the site centre (guide frame).
    pub centre_m: [f64; 2],
    /// The wet sample nearest the point searched from.
    pub nearest_m: [f64; 2],
    /// Its water body's area within the search window, m2.
    pub area_m2: f64,
    /// Whether that body reaches the window's edge (it goes on: a river or the sea, not a closed lake).
    pub open: bool,
    /// In-water distance to the nearest dry sample at the centre (about half the channel width), metres.
    pub half_width_m: f64,
}

/// Chamfer (3-4) distance of each wet sample to the nearest dry one, in thirds of a sample; dry = 0. Samples past
/// the window's edge don't count as dry.
fn chamfer(wet: &[bool], n: usize) -> Vec<u32> {
    let mut d: Vec<u32> = wet.iter().map(|&w| if w { u32::MAX / 2 } else { 0 }).collect();
    let step = |d: &mut [u32], k: usize, taps: &[(i64, i64, u32)]| {
        let (c, r) = ((k % n) as i64, (k / n) as i64);
        for &(dc, dr, cost) in taps {
            let (cc, rr) = (c + dc, r + dr);
            if cc >= 0 && rr >= 0 && (cc as usize) < n && (rr as usize) < n {
                d[k] = d[k].min(d[rr as usize * n + cc as usize] + cost);
            }
        }
    };
    let fwd = [(-1, 0, 3), (0, -1, 3), (-1, -1, 4), (1, -1, 4)];
    let bwd = [(1, 0, 3), (0, 1, 3), (1, 1, 4), (-1, 1, 4)];
    for k in 0..n * n {
        step(&mut d, k, &fwd);
    }
    for k in (0..n * n).rev() {
        step(&mut d, k, &bwd);
    }
    d
}

/// The river reach nearest `near` (guide frame): over the square of half edge `search_m` at the guide's spacing,
/// water as the game shows it (below the sea level), 8-connected bodies; the nearest wet sample of a body of at
/// least `min_area_m2`, then up the in-water distance (orthogonal steps first) to the channel's middle. `None`
/// when no such body is in the window.
pub fn find_river(tiles: &GuideTiles, near: [f64; 2], search_m: f64, min_area_m2: f64) -> Option<River> {
    let guide = tiles.guide();
    let sq = Square::new(near, search_m, guide.spacing_m);
    let n = sq.n();
    let wet: Vec<bool> = heights_at(tiles, n, n, |c, r| sq.at(c, r)).iter().map(|&h| f64::from(h) < guide.sea_level_m).collect();
    let inside = |c: i64, r: i64| c >= 0 && r >= 0 && (c as usize) < n && (r as usize) < n;
    let mut label = vec![usize::MAX; n * n];
    let (mut areas, mut open, mut stack) = (Vec::new(), Vec::new(), Vec::new());
    for start in 0..n * n {
        if !wet[start] || label[start] != usize::MAX {
            continue;
        }
        let id = areas.len();
        let (mut count, mut edge) = (0usize, false);
        label[start] = id;
        stack.push(start);
        while let Some(k) = stack.pop() {
            count += 1;
            let (c, r) = ((k % n) as i64, (k / n) as i64);
            edge |= c == 0 || r == 0 || c as usize == n - 1 || r as usize == n - 1;
            for (dc, dr) in NEIGHBOURS {
                let (cc, rr) = (c + dc, r + dr);
                if inside(cc, rr) {
                    let kk = rr as usize * n + cc as usize;
                    if wet[kk] && label[kk] == usize::MAX {
                        label[kk] = id;
                        stack.push(kk);
                    }
                }
            }
        }
        areas.push(count as f64 * guide.spacing_m * guide.spacing_m);
        open.push(edge);
    }
    let k0 = sq.k as f64;
    let d2 = |k: usize| ((k % n) as f64 - k0).powi(2) + ((k / n) as f64 - k0).powi(2);
    let nearest = (0..n * n)
        .filter(|&k| label[k] != usize::MAX && areas[label[k]] >= min_area_m2)
        .min_by(|&a, &b| d2(a).total_cmp(&d2(b)))?;
    let depth = chamfer(&wet, n);
    let mut k = nearest;
    loop {
        let (c, r) = ((k % n) as i64, (k / n) as i64);
        let best = NEIGHBOURS
            .iter()
            .map(|&(dc, dr)| (c + dc, r + dr))
            .filter(|&(cc, rr)| inside(cc, rr))
            .map(|(cc, rr)| rr as usize * n + cc as usize)
            .fold(k, |b, kk| if depth[kk] > depth[b] { kk } else { b });
        if best == k {
            break;
        }
        k = best;
    }
    let id = label[nearest];
    Some(River {
        centre_m: sq.at(k % n, k / n),
        nearest_m: sq.at(nearest % n, nearest / n),
        area_m2: areas[id],
        open: open[id],
        half_width_m: f64::from(depth[k]) / 3.0 * guide.spacing_m,
    })
}

/// The ground layer grid the game draws at a point: the grid of the covering stamp of weight over one half
/// (`sf_worldgen::layers` "Tiles"), else the global grid.
fn layer_grid_at<'a>(
    grids: &[Option<&'a GuideLayerGrid>],
    global: &'a GuideLayerGrid,
    stamps: &[(usize, &Stamp)],
    p: [f64; 2],
) -> &'a GuideLayerGrid {
    let best = stamps
        .iter()
        .filter_map(|&(i, s)| s.sample(p[0], p[1]).map(|(_, w)| (i, w)))
        .fold(None, |b: Option<(usize, f32)>, s| if b.is_none_or(|b| s.1 > b.1) { Some(s) } else { b });
    match best {
        Some((i, w)) if w > STAMP_LAYER_WEIGHT => grids.get(i).copied().flatten().unwrap_or(global),
        _ => global,
    }
}

/// The palette index of the texel of `grid` covering `p` ([`NO_LAYER`] outside it).
fn texel_at(grid: &GuideLayerGrid, p: [f64; 2]) -> u16 {
    let col = ((p[0] - grid.origin_m[0]) / grid.spacing_m).floor();
    let row = ((grid.origin_m[1] - p[1]) / grid.spacing_m).floor();
    let [gw, gh] = grid.dims;
    if col < 0.0 || row < 0.0 || col >= gw as f64 || row >= gh as f64 {
        NO_LAYER
    } else {
        grid.texels[row as usize * gw + col as usize]
    }
}

/// A 10x-guide site in memory and the ground layers under it (palette index -> cells; [`NO_LAYER`] = default).
pub struct GuideSite {
    pub site: Site,
    pub textures: BTreeMap<u16, usize>,
}

/// The site of half edge `radius_m` at `cell_m` around `centre` from `tiles` (module docs, ST-2): heights, water,
/// and roads where the guide has layers (`road_of`: whether each palette entry is a road). No claim. Errs when the
/// square leaves the guide.
pub fn guide_site(
    tiles: &GuideTiles,
    name: &str,
    centre: [f64; 2],
    radius_m: f64,
    cell_m: f64,
    road_of: &[bool],
) -> Result<GuideSite, DumpError> {
    let g = tiles.guide();
    let sq = Square::new(centre, radius_m, cell_m);
    let n = sq.n();
    let [x0, z0, x1, z1] = tiles.extent_m();
    let (sw, ne) = (sq.at(0, n - 1), sq.at(n - 1, 0));
    if sw[0] < x0 || ne[0] > x1 || -ne[1] < z0 || -sw[1] > z1 {
        return Err(DumpError::Invalid(format!("site square around [{:.1}, {:.1}] m leaves the guide", centre[0], centre[1])));
    }
    let heights = heights_at(tiles, n, n, |c, r| sq.at(c, r));
    let water = heights.iter().map(|&h| u8::from(f64::from(h) < g.sea_level_m)).collect();
    let mut textures = BTreeMap::new();
    let roads = tiles.layers().map(|field| {
        let layers = field.layers();
        let set = tiles.stamps().map(StampIndex::set);
        let mut grids: Vec<Option<&GuideLayerGrid>> = vec![None; set.map_or(0, |s| s.stamps.len())];
        for (i, grid) in &layers.stamps {
            if let Some(slot) = grids.get_mut(*i) {
                *slot = Some(grid);
            }
        }
        // Only the stamps whose rectangle meets the square can cover a cell.
        let nw = sq.origin_m();
        let se = [nw[0] + n as f64 * cell_m, nw[1] - n as f64 * cell_m];
        let near: Vec<(usize, &Stamp)> = set
            .map(|s| s.stamps.iter().enumerate().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
            .filter(|(_, s)| {
                let r = s.rect_m();
                r[0] <= se[0] && r[2] >= nw[0] && r[1] <= nw[1] && r[3] >= se[1]
            })
            .collect();
        let mut out = Vec::with_capacity(n * n);
        for r in 0..n {
            for c in 0..n {
                let p = sq.at(c, r);
                let tex = texel_at(layer_grid_at(&grids, &layers.global, &near, p), p);
                *textures.entry(tex).or_default() += 1;
                out.push(u8::from(tex != NO_LAYER && road_of.get(usize::from(tex)).copied().unwrap_or(false)));
            }
        }
        out
    });
    let json = SiteJson {
        format: FORMAT,
        name: name.to_owned(),
        producer: "bevyrrowind worldgen-site --guide (mw_worldgen::site, ST-2)".into(),
        origin_m: sq.origin_m(),
        cell_m,
        dims: [n, n],
        water_level_m: g.sea_level_m,
        centre_m: centre,
        source: String::new(),
        claim: None,
        blake3: BTreeMap::new(),
    };
    Ok(GuideSite { site: Site { json, heights, water, roads }, textures })
}

fn median_of(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() { f64::NAN } else { v[v.len() / 2] }
}

/// Slope `|grad h|` (central differences) of the cells within `radius_m` of the centre cell: (dry cells, all cells).
fn slopes_near_centre(site: &Site, radius_m: f64) -> (Vec<f64>, Vec<f64>) {
    let [w, h] = site.json.dims;
    let s = site.json.cell_m;
    let (cc, cr) = ((w / 2) as f64, (h / 2) as f64);
    let rr = (radius_m / s).powi(2);
    let at = |c: usize, r: usize| f64::from(site.heights[r * w + c]);
    let (mut dry, mut all) = (Vec::new(), Vec::new());
    for r in 1..h.saturating_sub(1) {
        for c in 1..w.saturating_sub(1) {
            if (c as f64 - cc).powi(2) + (r as f64 - cr).powi(2) > rr {
                continue;
            }
            let gx = (at(c + 1, r) - at(c - 1, r)) / (2.0 * s);
            let gy = (at(c, r + 1) - at(c, r - 1)) / (2.0 * s);
            let slope = gx.hypot(gy);
            all.push(slope);
            if site.water[r * w + c] == 0 {
                dry.push(slope);
            }
        }
    }
    (dry, all)
}

/// 4-connected dry regions of the site, largest first (cells).
fn dry_regions(site: &Site) -> Vec<usize> {
    let [w, h] = site.json.dims;
    let mut seen = vec![false; w * h];
    let (mut sizes, mut stack) = (Vec::new(), Vec::new());
    for start in 0..w * h {
        if site.water[start] != 0 || seen[start] {
            continue;
        }
        seen[start] = true;
        stack.push(start);
        let mut count = 0;
        while let Some(k) = stack.pop() {
            count += 1;
            let (c, r) = (k % w, k / w);
            let next = [(c > 0).then(|| k - 1), (c + 1 < w).then(|| k + 1), (r > 0).then(|| k - w), (r + 1 < h).then(|| k + w)];
            for kk in next.into_iter().flatten() {
                if site.water[kk] == 0 && !seen[kk] {
                    seen[kk] = true;
                    stack.push(kk);
                }
            }
        }
        sizes.push(count);
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes
}

/// The ledger's dried rivers (rivers under lifted towns ran dry): the crop's cells under the dropped `stamp`'s
/// rectangle, and how many are water without it (the site) and with it (the guide as shipped).
fn water_under(guide: &Path, args: &SiteArgs, site: &Site, stamp: &Stamp, centre: [f64; 2]) -> Result<(usize, usize, usize), DumpError> {
    let (with, _) = guide_tiles(guide, None, args.sharp_scale)?;
    let rect = stamp.rect_m();
    let sq = Square::new(centre, args.params.guide_radius_m, args.params.cell_m);
    let cells_a_side = sq.n();
    let cells: Vec<usize> = (0..cells_a_side * cells_a_side)
        .filter(|&k| {
            let q = sq.at(k % cells_a_side, k / cells_a_side);
            q[0] >= rect[0] && q[0] <= rect[2] && q[1] >= rect[1] && q[1] <= rect[3]
        })
        .collect();
    let hw = heights_at(&with, cells.len(), 1, |i, _| sq.at(cells[i] % cells_a_side, cells[i] / cells_a_side));
    let wet_with = hw.iter().filter(|&&v| f64::from(v) < site.json.water_level_m).count();
    let wet_without = cells.iter().filter(|&&k| site.water[k] == 1).count();
    Ok((cells.len(), wet_without, wet_with))
}

/// The road cells and the ground layers of the crop (the 12 largest and every road).
fn roads_report(out: &mut String, site: &Site, textures: &BTreeMap<u16, usize>, road_of: &[bool], palette: &[String]) {
    let Some(roads) = &site.roads else {
        let _ = writeln!(out, "  roads: the guide has no ground layers, no roads.u8");
        return;
    };
    let _ = writeln!(out, "  road cells: {}", ones(roads));
    let mut by: Vec<(usize, u16)> = textures.iter().map(|(&t, &n)| (n, t)).collect();
    by.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let _ = writeln!(out, "  ground layers in the crop (cells, road?), the first 12 and every road:");
    for (i, (cells, tex)) in by.iter().enumerate() {
        let road = *tex != NO_LAYER && road_of[usize::from(*tex)];
        if i < 12 || road {
            let id = if *tex == NO_LAYER { "(default)" } else { palette[usize::from(*tex)].as_str() };
            let _ = writeln!(out, "    {cells:>9}  {}  {id}", if road { "road" } else { "    " });
        }
    }
}

/// The `worldgen-site --guide` command body (ST-2); returns the printed report.
pub fn run_guide(db: &RecordDb, guide: &Path, args: &SiteArgs) -> Result<String, DumpError> {
    let started = Instant::now();
    let prm = &args.params;
    let err = |e: sf_worldgen::Error| DumpError::Invalid(format!("{}: {e}", guide.display()));
    let anchors = AnchorTransform::read(guide)
        .map_err(err)?
        .ok_or_else(|| DumpError::Invalid(format!("{}: no anchors.json", guide.display())))?;
    let Some(anchor) = anchors.anchors.iter().find(|a| a.name.eq_ignore_ascii_case(&args.anchor)) else {
        return Err(DumpError::Invalid(format!("{}: no anchor named {:?}", guide.display(), args.anchor)));
    };
    let (tiles, dropped) = guide_tiles(guide, anchor.stamp, args.sharp_scale)?;
    let open_s = started.elapsed().as_secs_f64();
    let mapped = anchor.world_centre_m;
    let river = find_river(&tiles, mapped, prm.river_search_m, prm.river_min_area_m2).ok_or_else(|| {
        DumpError::Invalid(format!(
            "no water body of {} m2 within {} m of [{:.1}, {:.1}]",
            prm.river_min_area_m2, prm.river_search_m, mapped[0], mapped[1]
        ))
    })?;
    let ltex_file = |id: &str| db.get::<LandscapeTexture>(&RecordId::new(id)).map_or_else(String::new, |l| l.file_name.clone());
    let palette: Vec<String> = tiles.layers().map(|f| f.layers().palette.clone()).unwrap_or_default();
    let road_of: Vec<bool> = palette.iter().map(|id| args.table.roads.is_road(id, &ltex_file(id))).collect();
    let GuideSite { mut site, textures } = guide_site(&tiles, &anchor.name, river.centre_m, prm.guide_radius_m, prm.cell_m, &road_of)?;
    let dropped_name = dropped.as_ref().map_or_else(|| "none (a point anchor)".to_owned(), |s| format!("{:?}", s.name));
    site.json.source = format!(
        "{} guide, around the river reach nearest {}'s mapped centre, without its stamp {dropped_name} (worldgen.sharp {})",
        guide.display(),
        anchor.name,
        args.sharp_scale
    );
    let json = write(&site, &args.out)?;

    let dist = |a: [f64; 2], b: [f64; 2]| (a[0] - b[0]).hypot(a[1] - b[1]);
    let [w, h] = json.dims;
    let mut out = String::new();
    let _ = writeln!(
        out,
        "worldgen-site --guide {}: {} (stamp {dropped_name} dropped, lift {:.1} m), mapped centre [{:.1}, {:.1}] m",
        guide.display(),
        anchor.name,
        anchor.lift_m,
        mapped[0],
        mapped[1]
    );
    let _ = writeln!(
        out,
        "  river: nearest water {:.0} m from the mapped centre at [{:.1}, {:.1}]; centre_m [{:.1}, {:.1}] ({:.0} m from the \
         mapped centre), half width ~{:.0} m; its body {:.2} km2 in the {:.0} m window, {}",
        dist(river.nearest_m, mapped),
        river.nearest_m[0],
        river.nearest_m[1],
        river.centre_m[0],
        river.centre_m[1],
        dist(river.centre_m, mapped),
        river.half_width_m,
        river.area_m2 / 1e6,
        prm.river_search_m,
        if river.open { "open (reaches the window edge)" } else { "closed in the window" }
    );
    let _ = writeln!(out, "  crop: {w} x {h} cells at {:.4} m, origin [{:.1}, {:.1}] m", json.cell_m, json.origin_m[0], json.origin_m[1]);
    let (cc, cr) = (w / 2, h / 2);
    let mut sides = [0usize; 4];
    for (k, &v) in site.water.iter().enumerate() {
        if v == 1 {
            let (c, r) = (k % w, k / w);
            for (i, on) in [c < cc, c > cc, r < cr, r > cr].into_iter().enumerate() {
                sides[i] += usize::from(on);
            }
        }
    }
    let [west, east, north, south] = sides;
    let _ = writeln!(out, "  water cells: {} (west of the centre {west}, east {east}, north {north}, south {south})", ones(&site.water));
    let regions = dry_regions(&site);
    let km2 = json.cell_m * json.cell_m / 1e6;
    let (r0, r1) = (regions.first().copied().unwrap_or(0), regions.get(1).copied().unwrap_or(0));
    let _ = writeln!(
        out,
        "  dry regions: {} (largest {r0} cells = {:.2} km2, next {r1} = {:.2} km2)",
        regions.len(),
        r0 as f64 * km2,
        r1 as f64 * km2
    );
    if let Some(stamp) = &dropped {
        let (cells, wet_without, wet_with) = water_under(guide, args, &site, stamp, river.centre_m)?;
        let _ = writeln!(
            out,
            "  under the dropped stamp's rectangle ({cells} cells in the crop): water cells {wet_without} without it, {wet_with} with it"
        );
    }
    let (dry, all) = slopes_near_centre(&site, SLOPE_REPORT_RADIUS_M);
    let (nd, na) = (dry.len(), all.len());
    let _ = writeln!(
        out,
        "  slope median within {SLOPE_REPORT_RADIUS_M} m of the centre: {:.3} over dry cells ({nd}), {:.3} over all ({na})",
        median_of(dry),
        median_of(all)
    );
    roads_report(&mut out, &site, &textures, &road_of, &palette);
    let _ = writeln!(out, "wrote {} in {:.1} s (guide opened in {open_s:.1} s)", args.out.display(), started.elapsed().as_secs_f64());
    Ok(out)
}
