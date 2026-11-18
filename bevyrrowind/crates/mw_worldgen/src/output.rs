//! The WG-S1 files (`--out <dir>`):
//!
//! - `heights.f32`: raw little-endian f32 meters, row-major, `width × height`, row 0 north (crate docs).
//! - `heights.png`: 16-bit gray, `meta.min_m` → 0 and `meta.max_m` → 65535 (linear, rounded).
//! - `mask.png`: 8-bit gray, 255 where a `LAND` gives the sample, 0 where the cell has none.
//! - `regions.png`: 8-bit indexed, one pixel per cell, north up; the pixel is the legend index and the palette
//!   holds the legend colours (so it views as a coloured map).
//! - `regions.json`: the legend ([`RegionsFile`]).
//! - `meta.json`: layout, units, sources, a blake3 of `heights.f32`, and the [`Stats`] ([`Meta`]).
//! - `markers.json`: named exterior cell groups and the summit, source metres ([`crate::markers`]).

use std::fs::File;
use std::fmt::Write as _;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use mw_world::terrain::{LAND_VERTS, VERTEX_UNITS, fill_height};
use serde::{Deserialize, Serialize};

use crate::{Dump, DumpError, RegionEntry, Stats, dump};

/// Version of the file set; bumped on any layout change.
pub const FORMAT_VERSION: u32 = 1;

/// `meta.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub format: u32,
    pub worldspace: String,
    /// The cell whose north-west vertex is sample (0, 0).
    pub origin_cell: [i32; 2],
    /// Morrowind units `[x, y]` of sample (0, 0).
    pub origin_mw_units: [f64; 2],
    /// Engine meters `[x, z]` of sample (0, 0) (engine: x east, −z north).
    pub origin_engine_m: [f64; 2],
    /// The `LAND` box, Morrowind grid (x east, y north).
    pub cells_min: [i32; 2],
    pub cells_max: [i32; 2],
    pub cell_size_m: f64,
    pub sample_spacing_m: f64,
    /// Vertices along a source cell edge (shared with the neighbours).
    pub samples_per_cell_edge: usize,
    /// `[width, height]` of the height mosaic and mask, samples.
    pub dims: [usize; 2],
    /// `[width, height]` of the region map, cells.
    pub region_dims: [usize; 2],
    pub min_m: f32,
    pub max_m: f32,
    /// Height of samples in cells without `LAND` (mask 0).
    pub fill_m: f32,
    pub sea_level_m: f32,
    pub orientation: String,
    /// Record sources in load order.
    pub plugins: Vec<String>,
    /// blake3 of `heights.f32`, hex.
    pub heights_blake3: String,
    pub without_heights: usize,
    pub edge_mismatches: usize,
    pub max_edge_mismatch_m: f32,
    pub stats: Stats,
}

/// `regions.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionsFile {
    pub note: String,
    pub regions: Vec<RegionEntry>,
}

/// Orientation text stored in `meta.json`.
pub const ORIENTATION: &str = "row-major, row 0 = north edge (north up), column 0 = west edge; sample (col, row) at \
     Morrowind x = origin_mw_units.x + 128 col, y = origin_mw_units.y - 128 row units";

fn png_file<'a>(path: &Path, width: usize, height: usize) -> Result<png::Encoder<'a, BufWriter<File>>, DumpError> {
    let w = u32::try_from(width).map_err(|_| std::io::Error::other("image too wide"))?;
    let h = u32::try_from(height).map_err(|_| std::io::Error::other("image too tall"))?;
    Ok(png::Encoder::new(BufWriter::new(File::create(path)?), w, h))
}

/// Writes the file set of `dump` into `dir` (created) and returns the `meta.json` contents.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn write(dump: &Dump, dir: &Path) -> Result<Meta, DumpError> {
    std::fs::create_dir_all(dir)?;
    let m = &dump.mosaic;
    let mut raw = Vec::with_capacity(m.heights.len() * 4);
    for h in &m.heights {
        raw.extend_from_slice(&h.to_le_bytes());
    }
    std::fs::write(dir.join("heights.f32"), &raw)?;
    let hash = blake3::hash(&raw).to_hex().to_string();
    drop(raw);

    let (min_m, max_m) = m.heights.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), &h| (lo.min(h), hi.max(h)));
    let range = f64::from(max_m - min_m);
    let mut gray = Vec::with_capacity(m.heights.len() * 2);
    for &h in &m.heights {
        let v = if range > 0.0 { (f64::from(h - min_m) / range * 65535.0).round() as u16 } else { 0 };
        gray.extend_from_slice(&v.to_be_bytes());
    }
    let mut enc = png_file(&dir.join("heights.png"), m.width, m.height)?;
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::Sixteen);
    enc.write_header()?.write_image_data(&gray)?;
    drop(gray);

    let mask: Vec<u8> = m.mask.iter().map(|&v| if v == 0 { 0 } else { 255 }).collect();
    let mut enc = png_file(&dir.join("mask.png"), m.width, m.height)?;
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(&mask)?;

    let r = &dump.regions;
    let palette: Vec<u8> = r.legend.iter().flat_map(|e| e.color).collect();
    let mut enc = png_file(&dir.join("regions.png"), r.width, r.height)?;
    enc.set_color(png::ColorType::Indexed);
    enc.set_depth(png::BitDepth::Eight);
    enc.set_palette(palette);
    enc.write_header()?.write_image_data(&r.index)?;

    let legend = RegionsFile {
        note: "regions.png pixel = index; index 0 = no region; one pixel per cell, north up".into(),
        regions: r.legend.clone(),
    };
    std::fs::write(dir.join("regions.json"), serde_json::to_vec_pretty(&legend)?)?;

    let unit = UNIT_METERS_F64;
    let (ox, oy) = m.mw_units(0, 0);
    let meta = Meta {
        format: FORMAT_VERSION,
        worldspace: mw_world::ids::EXTERIOR.into(),
        origin_cell: [m.cells_min.0, m.cells_max.1],
        origin_mw_units: [ox, oy],
        origin_engine_m: [ox * unit, -oy * unit],
        cells_min: [m.cells_min.0, m.cells_min.1],
        cells_max: [m.cells_max.0, m.cells_max.1],
        cell_size_m: CELL_UNITS * unit,
        sample_spacing_m: f64::from(VERTEX_UNITS) * unit,
        samples_per_cell_edge: LAND_VERTS,
        dims: [m.width, m.height],
        region_dims: [r.width, r.height],
        min_m,
        max_m,
        fill_m: fill_height(),
        sea_level_m: dump.stats.sea_level_m,
        orientation: ORIENTATION.into(),
        plugins: dump.plugins.clone(),
        heights_blake3: hash,
        without_heights: m.without_heights,
        edge_mismatches: m.edge_mismatches,
        max_edge_mismatch_m: m.max_edge_mismatch_m,
        stats: dump.stats.clone(),
    };
    let mut file = BufWriter::new(File::create(dir.join("meta.json"))?);
    serde_json::to_writer_pretty(&mut file, &meta)?;
    file.flush()?;
    std::fs::write(dir.join("markers.json"), serde_json::to_vec_pretty(&dump.markers)?)?;
    Ok(meta)
}

/// Reads `dir/meta.json` back.
pub fn read_meta(dir: &Path) -> Result<Meta, DumpError> {
    Ok(serde_json::from_slice(&std::fs::read(dir.join("meta.json"))?)?)
}

/// The `worldgen-dump` command body: dump, write, and the printed report.
pub fn run(db: &sf_records::RecordDb, out: &Path, bin_m: f32) -> Result<String, DumpError> {
    let started = Instant::now();
    let dump = dump(db, bin_m)?;
    let meta = write(&dump, out)?;
    let size = |name: &str| std::fs::metadata(out.join(name)).map_or(0, |m| m.len());
    let mut report = format!(
        "mosaic {}x{} samples ({} m spacing), cells ({}, {})..({}, {}), heights {:.1}..{:.1} m, {} LAND without \
         heights, {} shared-edge mismatches (max {} m)\n",
        meta.dims[0],
        meta.dims[1],
        meta.sample_spacing_m,
        meta.cells_min[0],
        meta.cells_min[1],
        meta.cells_max[0],
        meta.cells_max[1],
        meta.min_m,
        meta.max_m,
        meta.without_heights,
        meta.edge_mismatches,
        meta.max_edge_mismatch_m
    );
    report.push_str(&dump.stats.to_string());
    for name in ["heights.f32", "heights.png", "mask.png", "regions.png", "regions.json", "meta.json", "markers.json"] {
        let _ = writeln!(report, "wrote {} ({} bytes)", out.join(name).display(), size(name));
    }
    let _ = writeln!(report, "worldgen-dump: {:.2} s (after content load)", started.elapsed().as_secs_f64());
    Ok(report)
}
