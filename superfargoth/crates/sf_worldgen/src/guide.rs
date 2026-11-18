//! Guide grids, **format v1** (worldgen.md "Pipeline": guides are generic inputs; an adapter writes them).
//!
//! A guide is a folder:
//!
//! - `meta.json` (required). Keys read by v1:
//!   - `format`: `1`.
//!   - `dims`: `[width, height]` in samples.
//!   - `sample_spacing_m`: horizontal spacing between samples, metres.
//!   - `sea_level_m`: sea level, metres (samples strictly below are wet).
//!   - `fill_m` (optional): the height the adapter wrote where it had no data (used as the masked-sea height).
//!   - `origin_m` (optional): `[x, y]` metres of sample (0, 0) in the guide frame, x east, y north.
//!     A recipe may give it instead (`[guide] origin_m`); default `[0, 0]`.
//!   - `cell_size_m` (required when a region grid exists): edge of one region-grid pixel, metres.
//!   - Every other key is ignored.
//! - `heights.f32` (required): `width × height` little-endian f32, row-major, **row 0 = north edge**, column 0 =
//!   west edge, metres.
//! - `regions.png` + `regions.json` (optional, together): an 8-bit indexed (or 8-bit gray) PNG, one pixel per
//!   region cell, north up, pixel (0, 0) covering sample (0, 0); its value is a legend index. `regions.json` has
//!   `regions: [{ "index": u8, "name": string }, ...]` (other keys ignored). Index 0 with an empty name means
//!   "no region".
//!
//! Extra keys are allowed so an adapter can add its own provenance and statistics.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::{Error, Result};

/// A loaded guide (format v1).
#[derive(Debug, Clone)]
pub struct Guide {
    pub w: usize,
    pub h: usize,
    /// Sample spacing, metres.
    pub spacing_m: f64,
    pub sea_level_m: f64,
    pub fill_m: Option<f64>,
    /// Guide-frame metres of sample (0, 0): x east, y north.
    pub origin_m: [f64; 2],
    /// Heights, metres, row-major, row 0 = north.
    pub heights: Vec<f32>,
    pub regions: Option<RegionMap>,
}

/// Coarse region grid with its legend.
#[derive(Debug, Clone)]
pub struct RegionMap {
    pub w: usize,
    pub h: usize,
    /// Edge of one region pixel, metres.
    pub cell_m: f64,
    /// Legend index per pixel, row-major, north up.
    pub index: Vec<u8>,
    /// Name per legend index (empty when the legend has none).
    pub names: Vec<String>,
}

#[derive(serde::Deserialize)]
struct MetaV1 {
    format: u32,
    dims: [usize; 2],
    sample_spacing_m: f64,
    sea_level_m: f64,
    fill_m: Option<f64>,
    origin_m: Option<[f64; 2]>,
    cell_size_m: Option<f64>,
}

#[derive(serde::Deserialize)]
struct LegendV1 {
    regions: Vec<LegendEntry>,
}

#[derive(serde::Deserialize)]
struct LegendEntry {
    index: u8,
    #[serde(default)]
    name: String,
}

impl Guide {
    /// Reads a guide folder (format v1).
    pub fn read(dir: &Path) -> Result<Self> {
        let meta: MetaV1 =
            serde_json::from_reader(BufReader::new(File::open(dir.join("meta.json"))?))?;
        if meta.format != 1 {
            return Err(Error::Invalid(format!(
                "guide format {} (v1 expected)",
                meta.format
            )));
        }
        let [w, h] = meta.dims;
        if w < 4 || h < 4 || w.checked_mul(h).is_none_or(|n| n >= 1 << 31) {
            return Err(Error::Invalid(format!("guide dims {w}x{h} out of range")));
        }
        if meta.sample_spacing_m.is_nan() || meta.sample_spacing_m <= 0.0 {
            return Err(Error::Invalid("sample_spacing_m must be > 0".into()));
        }
        let raw = std::fs::read(dir.join("heights.f32"))?;
        if raw.len() != w * h * 4 {
            return Err(Error::Invalid(format!(
                "heights.f32 is {} bytes, {} expected",
                raw.len(),
                w * h * 4
            )));
        }
        let heights: Vec<f32> = raw
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| f32::from_le_bytes(*c))
            .collect();
        if heights.iter().any(|v| !v.is_finite()) {
            return Err(Error::Invalid("heights.f32 has non-finite samples".into()));
        }
        let regions = if dir.join("regions.png").exists() {
            let cell_m = meta.cell_size_m.ok_or_else(|| {
                Error::Invalid("regions.png present but meta.json has no cell_size_m".into())
            })?;
            Some(RegionMap::read(
                &dir.join("regions.png"),
                &dir.join("regions.json"),
                cell_m,
            )?)
        } else {
            None
        };
        Ok(Self {
            w,
            h,
            spacing_m: meta.sample_spacing_m,
            sea_level_m: meta.sea_level_m,
            fill_m: meta.fill_m,
            origin_m: meta.origin_m.unwrap_or([0.0, 0.0]),
            heights,
            regions,
        })
    }
}

impl RegionMap {
    /// Reads `regions.png` (8-bit indexed or gray) and its legend.
    pub fn read(png_path: &Path, legend_path: &Path, cell_m: f64) -> Result<Self> {
        let mut dec = png::Decoder::new(BufReader::new(File::open(png_path)?));
        dec.set_transformations(png::Transformations::IDENTITY);
        let mut reader = dec.read_info()?;
        let size = reader
            .output_buffer_size()
            .ok_or_else(|| Error::Invalid("regions.png too large".into()))?;
        let mut buf = vec![0u8; size];
        let info = reader.next_frame(&mut buf)?;
        if info.bit_depth != png::BitDepth::Eight
            || !matches!(
                info.color_type,
                png::ColorType::Indexed | png::ColorType::Grayscale
            )
        {
            return Err(Error::Invalid(
                "regions.png must be 8-bit indexed or gray".into(),
            ));
        }
        let (w, h) = (info.width as usize, info.height as usize);
        let mut index = Vec::with_capacity(w * h);
        for row in buf.chunks(info.line_size).take(h) {
            index.extend_from_slice(&row[..w]);
        }
        let legend: LegendV1 = serde_json::from_reader(BufReader::new(File::open(legend_path)?))?;
        let mut names = vec![String::new(); 256];
        for e in legend.regions {
            names[usize::from(e.index)] = e.name;
        }
        Ok(Self {
            w,
            h,
            cell_m,
            index,
            names,
        })
    }

    /// Legend index of the pixel at (px, py), clamped to the grid.
    pub fn at_clamped(&self, px: isize, py: isize) -> u8 {
        let x = px.clamp(0, self.w as isize - 1).cast_unsigned();
        let y = py.clamp(0, self.h as isize - 1).cast_unsigned();
        self.index[y * self.w + x]
    }

    /// Legend index of the region pixel holding guide sample (col, row).
    pub fn at_sample(&self, col: usize, row: usize, spacing_m: f64) -> u8 {
        let px = ((col as f64 + 0.5) * spacing_m / self.cell_m).floor() as isize;
        let py = ((row as f64 + 0.5) * spacing_m / self.cell_m).floor() as isize;
        self.at_clamped(px, py)
    }

    /// Legend index whose name equals `name`.
    pub fn find(&self, name: &str) -> Option<u8> {
        self.names
            .iter()
            .position(|n| !n.is_empty() && n == name)
            .map(|i| i as u8)
    }
}
