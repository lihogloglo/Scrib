//! Morrowind → worldgen adapter (`docs/specs/worldgen-adapter.md`), Bevy-free.
//!
//! WG-S1: the exterior worldspace's source terrain as one height mosaic plus a per-cell region map, written as
//! raw f32, PNGs and JSON for the generator prototype. WG-R2 (terrain half): the measurements of that mosaic
//! ([`stats`]). Nothing here is shipped or committed: the outputs are derived from the player's own data.
//!
//! The format facts reuse `mw_world::terrain` (`LandData::decode`, `fill_height`) and `mw_world::convert`
//! (`UNIT_METERS_F64`, `CELL_UNITS`); no conversion is restated here.
//!
//! **Mosaic layout:** cells `cells_min..=cells_max` (Morrowind grid, x east, y north), 65² vertices per cell at
//! 128 units (1.8288 m), shared edges merged once (64 samples per cell plus one closing row and column):
//! `width = 64·cells_x + 1`, `height = 64·cells_y + 1`. Row-major, **row 0 = the north edge** (north up),
//! column 0 = the west edge. Sample `(col, row)` is at Morrowind `x = 8192·cells_min.x + 128·col`,
//! `y = 8192·(cells_max.y + 1) − 128·row` units. Cells without `LAND` hold [`fill_height`] and mask 0.

pub mod anchor;
pub mod classify;
pub mod flora;
pub mod layers;
pub mod markers;
mod output;
pub mod stats;
pub mod x10;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_flora;
#[cfg(test)]
mod tests_x10;

use std::collections::BTreeMap;
use std::fmt;

use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::{LAND_VERTS, LandData, fill_height};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{Cell, Landscape, Region};

pub use markers::{Marker, MarkersFile, SummitMarker, read_markers};
pub use output::{Meta, RegionsFile, read_meta, run, write};
pub use stats::Stats;

/// Quads along a cell edge: a cell's samples start every this many mosaic samples.
pub const CELL_QUADS: usize = LAND_VERTS - 1;

/// Morrowind's exterior sea surface, in units.
// tunables: const (format fact: the exterior water plane is at z = 0 in Morrowind's data and OpenMW)
pub const SEA_SURFACE_UNITS: f32 = 0.0;

/// Units to meters through `mw_world`'s conversion (f64, then rounded once).
#[allow(clippy::cast_possible_truncation)]
pub fn meters(units: f32) -> f32 {
    (f64::from(units) * UNIT_METERS_F64) as f32
}

/// Errors of a dump.
#[derive(Debug)]
pub enum DumpError {
    /// The database holds no exterior `LAND`.
    NoLand,
    /// More regions than an 8-bit index map holds.
    TooManyRegions(usize),
    Io(std::io::Error),
    Png(png::EncodingError),
    Json(serde_json::Error),
    /// A bad input or a failed framework write (`worldgen-x10`).
    Invalid(String),
}

impl fmt::Display for DumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoLand => write!(f, "no exterior LAND records loaded"),
            Self::TooManyRegions(n) => write!(f, "{n} regions do not fit an 8-bit region map (255 max)"),
            Self::Io(e) => write!(f, "{e}"),
            Self::Png(e) => write!(f, "png: {e}"),
            Self::Json(e) => write!(f, "json: {e}"),
            Self::Invalid(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for DumpError {}

impl From<std::io::Error> for DumpError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<png::EncodingError> for DumpError {
    fn from(e: png::EncodingError) -> Self {
        Self::Png(e)
    }
}
impl From<serde_json::Error> for DumpError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

/// The exterior's `LAND` as one height field (module docs: layout).
#[derive(Clone, Debug, PartialEq)]
pub struct Mosaic {
    /// South-west-most cell of the box (min x, min y).
    pub cells_min: (i32, i32),
    /// North-east-most cell (max x, max y).
    pub cells_max: (i32, i32),
    pub width: usize,
    pub height: usize,
    /// Meters, `[row * width + col]`, row 0 north.
    pub heights: Vec<f32>,
    /// 1 where a `LAND` provides the sample, 0 where the cell has none (height = [`fill_height`]).
    pub mask: Vec<u8>,
    /// Cells with a `LAND` record.
    pub land_cells: usize,
    /// `LAND` records without the heights flag (flat at `OpenMW`'s default height).
    pub without_heights: usize,
    /// Shared-edge samples where the second cell disagreed with the first (the first is kept).
    pub edge_mismatches: usize,
    pub max_edge_mismatch_m: f32,
}

impl Mosaic {
    /// Cells across and down the box.
    #[allow(clippy::cast_sign_loss)]
    pub fn cells(&self) -> (usize, usize) {
        ((self.cells_max.0 - self.cells_min.0 + 1) as usize, (self.cells_max.1 - self.cells_min.1 + 1) as usize)
    }

    /// Mosaic `(col, row)` of the north-west sample of cell `grid` (inside the box).
    #[allow(clippy::cast_sign_loss)]
    pub fn cell_origin(&self, (gx, gy): (i32, i32)) -> (usize, usize) {
        ((gx - self.cells_min.0) as usize * CELL_QUADS, (self.cells_max.1 - gy) as usize * CELL_QUADS)
    }

    /// Height at `(col, row)`, meters.
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.heights[row * self.width + col]
    }

    /// The cell that owns sample `(col, row)`: each cell owns its north and west edges, so the mosaic's last
    /// row and column (the box's south and east edges) belong to the cell they close.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn cell_of(&self, col: usize, row: usize) -> (i32, i32) {
        let (cx, cy) = self.cells();
        let x = (col / CELL_QUADS).min(cx - 1) as i32;
        let y = (row / CELL_QUADS).min(cy - 1) as i32;
        (self.cells_min.0 + x, self.cells_max.1 - y)
    }

    /// Morrowind units `(x, y)` of sample `(col, row)`.
    #[allow(clippy::cast_precision_loss)]
    pub fn mw_units(&self, col: usize, row: usize) -> (f64, f64) {
        let cell = mw_world::convert::CELL_UNITS;
        let vertex = f64::from(mw_world::terrain::VERTEX_UNITS);
        (
            f64::from(self.cells_min.0) * cell + vertex * col as f64,
            f64::from(self.cells_max.1 + 1) * cell - vertex * row as f64,
        )
    }
}

/// Assembles every exterior `LAND` of `db` into a [`Mosaic`] over their bounding box.
pub fn assemble(db: &RecordDb) -> Result<Mosaic, DumpError> {
    let mut lands: Vec<&Landscape> = db.iter::<Landscape>().map(|(_, l)| l).collect();
    lands.sort_by_key(|l| l.grid);
    let first = lands.first().ok_or(DumpError::NoLand)?.grid;
    let (mut lo, mut hi) = (first, first);
    for l in &lands {
        lo = (lo.0.min(l.grid.0), lo.1.min(l.grid.1));
        hi = (hi.0.max(l.grid.0), hi.1.max(l.grid.1));
    }
    let mut mosaic = Mosaic {
        cells_min: lo,
        cells_max: hi,
        width: 0,
        height: 0,
        heights: Vec::new(),
        mask: Vec::new(),
        land_cells: lands.len(),
        without_heights: 0,
        edge_mismatches: 0,
        max_edge_mismatch_m: 0.0,
    };
    let (cx, cy) = mosaic.cells();
    mosaic.width = cx * CELL_QUADS + 1;
    mosaic.height = cy * CELL_QUADS + 1;
    mosaic.heights = vec![fill_height(); mosaic.width * mosaic.height];
    mosaic.mask = vec![0; mosaic.width * mosaic.height];
    for land in lands {
        let flags = land.landscape_flags;
        mosaic.without_heights +=
            usize::from(!flags.contains(tes3::esp::LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS));
        let data = LandData::decode(land);
        let (c0, r0) = mosaic.cell_origin(land.grid);
        // Morrowind vertex (i east, j north) → mosaic (c0 + i, r0 + 64 − j).
        for j in 0..LAND_VERTS {
            for i in 0..LAND_VERTS {
                let k = (r0 + CELL_QUADS - j) * mosaic.width + c0 + i;
                let h = meters(data.heights[j * LAND_VERTS + i]);
                if mosaic.mask[k] == 1 {
                    let d = (mosaic.heights[k] - h).abs();
                    if d > 0.0 {
                        mosaic.edge_mismatches += 1;
                        mosaic.max_edge_mismatch_m = mosaic.max_edge_mismatch_m.max(d);
                    }
                } else {
                    mosaic.heights[k] = h;
                    mosaic.mask[k] = 1;
                }
            }
        }
    }
    Ok(mosaic)
}

/// One region of the legend. Index 0 is "no region".
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegionEntry {
    pub index: u8,
    /// `REGN` record id (as the cells name it when the record is missing; empty for index 0).
    pub id: String,
    /// `REGN` display name (empty when the record is missing).
    pub name: String,
    /// Distinct per entry: the region's own map colour unless another entry already has it.
    pub color: [u8; 3],
    /// The cells name a region with no `REGN` record.
    pub missing: bool,
}

/// Per-cell region index over a cell box, north up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionMap {
    pub cells_min: (i32, i32),
    pub cells_max: (i32, i32),
    /// Cells across.
    pub width: usize,
    /// Cells down.
    pub height: usize,
    /// `[row * width + col]`, row 0 = the northern-most cell row.
    pub index: Vec<u8>,
    /// `legend[i].index == i`.
    pub legend: Vec<RegionEntry>,
}

impl RegionMap {
    /// The region index of cell `grid` (inside the box).
    #[allow(clippy::cast_sign_loss)]
    pub fn of(&self, (gx, gy): (i32, i32)) -> u8 {
        self.index[(self.cells_max.1 - gy) as usize * self.width + (gx - self.cells_min.0) as usize]
    }
}

/// The region map of the exterior `CELL` records inside `cells_min..=cells_max`.
#[allow(clippy::cast_sign_loss)]
pub fn region_map(db: &RecordDb, cells_min: (i32, i32), cells_max: (i32, i32)) -> Result<RegionMap, DumpError> {
    let inside = |(x, y): (i32, i32)| (cells_min.0..=cells_max.0).contains(&x) && (cells_min.1..=cells_max.1).contains(&y);
    let mut cells: Vec<((i32, i32), String)> = Vec::new();
    let mut ids: BTreeMap<String, String> = BTreeMap::new();
    for (_, cell) in db.iter::<Cell>() {
        if cell.is_interior() || !inside(cell.data.grid) {
            continue;
        }
        if let Some(region) = cell.region.as_deref().filter(|r| !r.is_empty()) {
            let key = region.to_ascii_lowercase();
            ids.entry(key.clone()).or_insert_with(|| region.to_owned());
            cells.push((cell.data.grid, key));
        }
    }
    if ids.len() > usize::from(u8::MAX) {
        return Err(DumpError::TooManyRegions(ids.len()));
    }
    let mut legend = vec![RegionEntry { index: 0, id: String::new(), name: String::new(), color: [0; 3], missing: false }];
    let mut by_key = BTreeMap::new();
    for (key, spelling) in &ids {
        let record = db.get::<Region>(&RecordId::new(spelling));
        let index = u8::try_from(legend.len()).expect("checked above");
        let wanted = record.map_or([0; 3], |r| [r.map_color[0], r.map_color[1], r.map_color[2]]);
        let color = distinct_color(wanted, &legend, usize::from(index));
        legend.push(RegionEntry {
            index,
            id: record.map_or_else(|| spelling.clone(), |r| r.id.clone()),
            name: record.map(|r| r.name.clone()).unwrap_or_default(),
            color,
            missing: record.is_none(),
        });
        by_key.insert(key.clone(), index);
    }
    let width = (cells_max.0 - cells_min.0 + 1) as usize;
    let height = (cells_max.1 - cells_min.1 + 1) as usize;
    let mut map = RegionMap { cells_min, cells_max, width, height, index: vec![0; width * height], legend };
    for (grid, key) in cells {
        let row = (cells_max.1 - grid.1) as usize;
        let col = (grid.0 - cells_min.0) as usize;
        map.index[row * width + col] = by_key[&key];
    }
    Ok(map)
}

/// `wanted` unless black (index 0's colour) or already taken; else golden-angle hues until one is free.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn distinct_color(wanted: [u8; 3], taken: &[RegionEntry], seed: usize) -> [u8; 3] {
    let free = |c: [u8; 3]| c != [0; 3] && taken.iter().all(|e| e.color != c);
    if free(wanted) {
        return wanted;
    }
    (0..=u16::MAX as usize).map(|k| hue((seed + k) as f32 * 0.618_034)).find(|c| free(*c)).expect("at most 256 regions")
}

/// A saturated colour at `turns` around the hue wheel (HSV with S = 0.75, V = 0.95).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn hue(turns: f32) -> [u8; 3] {
    let sector = turns.fract() * 6.0;
    let (sat, val) = (0.75_f32, 0.95_f32);
    let chroma = val * sat;
    let mid = chroma * (1.0 - (sector % 2.0 - 1.0).abs());
    let rgb = match sector as u32 {
        0 => [chroma, mid, 0.0],
        1 => [mid, chroma, 0.0],
        2 => [0.0, chroma, mid],
        3 => [0.0, mid, chroma],
        4 => [mid, 0.0, chroma],
        _ => [chroma, 0.0, mid],
    };
    let floor = val - chroma;
    rgb.map(|ch| ((ch + floor) * 255.0).round() as u8)
}

/// Everything WG-S1 exports, computed from the record database alone.
#[derive(Clone, Debug)]
pub struct Dump {
    pub mosaic: Mosaic,
    pub regions: RegionMap,
    pub stats: Stats,
    /// Named exterior cell groups and the summit (`markers.json`, [`markers`]).
    pub markers: MarkersFile,
    /// The record sources in load order (content files and mods).
    pub plugins: Vec<String>,
}

/// Builds the [`Dump`] of `db`'s exterior; `bin_m` is the height histogram's bin width.
pub fn dump(db: &RecordDb, bin_m: f32) -> Result<Dump, DumpError> {
    let mosaic = assemble(db)?;
    let regions = region_map(db, mosaic.cells_min, mosaic.cells_max)?;
    let stats = stats::measure(&mosaic, &regions, bin_m);
    let plugins = db.sources().iter().map(|s| s.name.clone()).collect();
    let markers = markers::markers(db, &mosaic);
    Ok(Dump { mosaic, regions, stats, markers, plugins })
}
