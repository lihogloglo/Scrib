//! `markers.json` (WG-V2): the hand-made anchors of the exterior, as scale references for generated terrain.
//!
//! A marker is one connected group of **named exterior cells** with the same name (case-insensitive,
//! 8-connected, so two separate places sharing a name stay two markers): Balmora, Caldera, Vivec, Ghostgate,
//! Seyda Neen, but also forts, ruins and shrines. The data has no "settlement" flag, so every named group is
//! listed with its cell count; a viewer filters (`worldgen-view --marker-min-cells`). Only cells inside the
//! dump's `LAND` box are used. The file also holds the mosaic's summit (the highest `LAND` sample).
//!
//! **Frame: source metres**, x east, y north, (0, 0) = Morrowind's world origin: Morrowind units ×
//! `mw_world::convert::UNIT_METERS_F64`. It is the guide frame of a recipe whose `[guide] origin_m` is the dump's
//! `origin_mw_units` × that factor (`bevyrrowind/worldgen/vvardenfell-s2b.toml`). A generated arm at scale S puts
//! a marker at `S × centroid_m` in its frame; the anchor itself keeps its source size (cells of `cell_size_m`).
//!
//! ```json
//! { "format": 1, "frame": "...", "cell_size_m": 117.0432,
//!   "summit": { "x_m": .., "y_m": .., "height_m": .., "cell": [3, 9] },
//!   "markers": [ { "name": "Balmora", "cells": [[-3, -2], ...], "centroid_m": [x, y],
//!                  "min_m": [x, y], "max_m": [x, y] }, ... ] }
//! ```
//!
//! `cells` are Morrowind grid coordinates, sorted; `centroid_m` is the mean of the cell centres; `min_m`/`max_m`
//! bound the cells. Markers are sorted by cell count (largest first), then name.

use std::collections::BTreeMap;
use std::path::Path;

use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use serde::{Deserialize, Serialize};
use sf_records::RecordDb;
use tes3::esp::Cell;

use crate::{DumpError, Mosaic};

/// Version of `markers.json`.
pub const MARKERS_FORMAT: u32 = 1;

/// The frame text stored in `markers.json`.
pub const FRAME: &str = "source metres: Morrowind units x 1.8288/128 m, x east, y north, (0, 0) = Morrowind's world \
     origin (a guide frame whose origin_m = the dump's origin_mw_units x 1.8288/128)";

/// `markers.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkersFile {
    pub format: u32,
    pub frame: String,
    /// Edge of one source cell, metres.
    pub cell_size_m: f64,
    pub summit: Option<SummitMarker>,
    pub markers: Vec<Marker>,
}

/// The highest `LAND` sample, source metres.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummitMarker {
    pub x_m: f64,
    pub y_m: f64,
    pub height_m: f32,
    pub cell: [i32; 2],
}

/// One named group of exterior cells.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    /// The name as the first cell (lowest grid) spells it.
    pub name: String,
    /// Morrowind grid `[x, y]`, sorted.
    pub cells: Vec<[i32; 2]>,
    /// Mean of the cell centres, source metres `[x, y]`.
    pub centroid_m: [f64; 2],
    /// Bounds of the cells, source metres.
    pub min_m: [f64; 2],
    pub max_m: [f64; 2],
}

/// Edge of a cell, source metres.
pub fn cell_size_m() -> f64 {
    CELL_UNITS * UNIT_METERS_F64
}

/// Source-metre centre of cell `grid`.
#[allow(clippy::cast_precision_loss)]
pub fn cell_centre_m((gx, gy): (i32, i32)) -> [f64; 2] {
    let s = cell_size_m();
    [(f64::from(gx) + 0.5) * s, (f64::from(gy) + 0.5) * s]
}

/// A named cell: its grid and the name as it spells it.
type Spelt = ((i32, i32), String);

/// The markers of `db`'s named exterior cells inside `mosaic`'s box, plus its summit.
#[allow(clippy::cast_precision_loss)]
pub fn markers(db: &RecordDb, mosaic: &Mosaic) -> MarkersFile {
    let (lo, hi) = (mosaic.cells_min, mosaic.cells_max);
    let inside = |(x, y): (i32, i32)| (lo.0..=hi.0).contains(&x) && (lo.1..=hi.1).contains(&y);
    // Lower-cased name → (cell, spelling) per cell.
    let mut named: BTreeMap<String, Vec<Spelt>> = BTreeMap::new();
    for (_, cell) in db.iter::<Cell>() {
        let name = cell.name.trim();
        if cell.is_interior() || name.is_empty() || !inside(cell.data.grid) {
            continue;
        }
        named.entry(name.to_lowercase()).or_default().push((cell.data.grid, name.to_owned()));
    }
    let size = cell_size_m();
    let mut out = Vec::new();
    for (_, mut spelt) in named {
        spelt.sort();
        spelt.dedup_by_key(|(g, _)| *g);
        let cells: Vec<(i32, i32)> = spelt.iter().map(|(g, _)| *g).collect();
        for group in components(&cells) {
            // The spelling of the group's lowest cell (records iterate in no fixed order).
            let spelling = spelt.iter().find(|(g, _)| *g == group[0]).map_or_else(String::new, |(_, s)| s.clone());
            let n = group.len() as f64;
            let centres: Vec<[f64; 2]> = group.iter().map(|&g| cell_centre_m(g)).collect();
            let centroid_m = [centres.iter().map(|c| c[0]).sum::<f64>() / n, centres.iter().map(|c| c[1]).sum::<f64>() / n];
            let (min_x, min_y) = group.iter().fold((i32::MAX, i32::MAX), |(x, y), g| (x.min(g.0), y.min(g.1)));
            let (max_x, max_y) = group.iter().fold((i32::MIN, i32::MIN), |(x, y), g| (x.max(g.0), y.max(g.1)));
            out.push(Marker {
                name: spelling,
                cells: group.iter().map(|&(x, y)| [x, y]).collect(),
                centroid_m,
                min_m: [f64::from(min_x) * size, f64::from(min_y) * size],
                max_m: [f64::from(max_x + 1) * size, f64::from(max_y + 1) * size],
            });
        }
    }
    out.sort_by(|a, b| b.cells.len().cmp(&a.cells.len()).then_with(|| a.name.cmp(&b.name)).then_with(|| a.cells.cmp(&b.cells)));
    MarkersFile { format: MARKERS_FORMAT, frame: FRAME.into(), cell_size_m: size, summit: summit(mosaic), markers: out }
}

/// 8-connected groups of sorted `cells`, each sorted.
fn components(cells: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    let mut seen = vec![false; cells.len()];
    let mut groups = Vec::new();
    for start in 0..cells.len() {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        let mut group = vec![cells[start]];
        let mut k = 0;
        while k < group.len() {
            let (x, y) = group[k];
            for (i, &(cx, cy)) in cells.iter().enumerate() {
                if !seen[i] && (cx - x).abs() <= 1 && (cy - y).abs() <= 1 {
                    seen[i] = true;
                    group.push((cx, cy));
                }
            }
            k += 1;
        }
        group.sort_unstable();
        groups.push(group);
    }
    groups
}

/// The highest sample with `LAND` (the first one in row-major order on a tie).
fn summit(mosaic: &Mosaic) -> Option<SummitMarker> {
    let (index, height_m) = mosaic
        .heights
        .iter()
        .zip(&mosaic.mask)
        .enumerate()
        .filter(|(_, (_, mask))| **mask == 1)
        .fold(None, |best: Option<(usize, f32)>, (i, (&here, _))| match best {
            Some((_, top)) if top >= here => best,
            _ => Some((i, here)),
        })?;
    let (col, row) = (index % mosaic.width, index / mosaic.width);
    let (x, y) = mosaic.mw_units(col, row);
    let cell = mosaic.cell_of(col, row);
    Some(SummitMarker { x_m: x * UNIT_METERS_F64, y_m: y * UNIT_METERS_F64, height_m, cell: [cell.0, cell.1] })
}

/// Reads a `markers.json`.
pub fn read_markers(path: &Path) -> Result<MarkersFile, DumpError> {
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}
