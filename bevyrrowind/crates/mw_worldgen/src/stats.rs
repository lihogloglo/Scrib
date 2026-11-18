//! WG-R2 (terrain half): the measurements of the source mosaic.
//!
//! Heights count `LAND` samples only (mask 1). "Above sea" is a sample higher than the sea surface; its area is
//! samples × spacing² (each vertex stands for one spacing² of ground). Per region, a cell's samples are the ones
//! it owns ([`Mosaic::cell_of`]: its north and west edges), so no sample counts twice. Percentiles are nearest
//! rank (`sorted[ceil(p·n) − 1]`); the median is the p = 0.5 rank.

use std::collections::BTreeMap;
use std::fmt;

use mw_world::convert::{CELL_UNITS, UNIT_METERS_F64};
use mw_world::terrain::VERTEX_UNITS;
use serde::{Deserialize, Serialize};

use crate::{Mosaic, RegionMap, SEA_SURFACE_UNITS, meters};

/// The measurements, printed by the command and stored in `meta.json`. Derived sizes are f32: they round-trip
/// through JSON exactly (`serde_json` without `float_roundtrip` may not for every f64).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// Cells with `LAND`.
    pub land_cells: usize,
    /// Their area, km² (cells × cell area).
    pub land_cells_km2: f32,
    /// The `LAND` box: cells across and down, and km.
    pub box_cells: [usize; 2],
    pub box_km: [f32; 2],
    pub sea_level_m: f32,
    /// Samples above sea level.
    pub above_sea_samples: usize,
    pub above_sea_km2: f32,
    /// Bounding box of the samples above sea: `[col_min, row_min, col_max, row_max]` and its size in km
    /// (east-west, north-south). `None` when nothing is above sea.
    pub above_sea_box: Option<[usize; 4]>,
    pub above_sea_box_km: Option<[f32; 2]>,
    pub histogram_bin_m: f32,
    /// `(bin start m, samples)`, bins of `histogram_bin_m` from `floor(h / bin)`, ascending.
    pub histogram: Vec<(f32, usize)>,
    pub summit: Option<Summit>,
    pub regions: Vec<RegionStats>,
}

/// The highest `LAND` sample.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summit {
    pub height_m: f32,
    /// Mosaic `[col, row]`.
    pub sample: [usize; 2],
    pub cell: [i32; 2],
    /// Morrowind units `[x, y]`.
    pub mw_units: [f64; 2],
    /// Region legend index of its cell.
    pub region: u8,
}

/// One legend entry's measurements (index 0: cells without a region).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionStats {
    pub index: u8,
    pub id: String,
    pub name: String,
    /// Cells of the box mapped to this region (with or without `LAND`).
    pub cells: usize,
    pub area_km2: f32,
    /// Owned `LAND` samples.
    pub land_samples: usize,
    /// `[min, median, p95, max]` m over those samples; `None` without any.
    pub heights_m: Option<[f32; 4]>,
}

/// Nearest-rank percentile of an ascending slice (non-empty).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn percentile(sorted: &[f32], p: f64) -> f32 {
    let rank = (p * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

/// Measures `mosaic` and `regions` (the same box); `bin_m` is the histogram bin width.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn measure(mosaic: &Mosaic, regions: &RegionMap, bin_m: f32) -> Stats {
    let spacing_km = f64::from(VERTEX_UNITS) * UNIT_METERS_F64 / 1000.0;
    let cell_km = CELL_UNITS * UNIT_METERS_F64 / 1000.0;
    let sea = meters(SEA_SURFACE_UNITS);
    let (cx, cy) = mosaic.cells();

    let mut above = 0;
    let mut above_box: Option<[usize; 4]> = None;
    let mut histogram: BTreeMap<i64, usize> = BTreeMap::new();
    let mut summit: Option<(f32, usize, usize)> = None;
    let mut per_region: Vec<Vec<f32>> = vec![Vec::new(); regions.legend.len()];
    for row in 0..mosaic.height {
        for col in 0..mosaic.width {
            let k = row * mosaic.width + col;
            if mosaic.mask[k] == 0 {
                continue;
            }
            let h = mosaic.heights[k];
            *histogram.entry((f64::from(h) / f64::from(bin_m)).floor() as i64).or_default() += 1;
            if summit.is_none_or(|(best, _, _)| h > best) {
                summit = Some((h, col, row));
            }
            if h > sea {
                above += 1;
                above_box = Some(match above_box {
                    None => [col, row, col, row],
                    Some([c0, r0, c1, r1]) => [c0.min(col), r0.min(row), c1.max(col), r1.max(row)],
                });
            }
            per_region[usize::from(regions.of(mosaic.cell_of(col, row)))].push(h);
        }
    }

    let mut cells_per_region = vec![0usize; regions.legend.len()];
    for &i in &regions.index {
        cells_per_region[usize::from(i)] += 1;
    }
    let regions_out = regions
        .legend
        .iter()
        .zip(per_region.iter_mut())
        .map(|(entry, heights)| {
            heights.sort_by(f32::total_cmp);
            let cells = cells_per_region[usize::from(entry.index)];
            RegionStats {
                index: entry.index,
                id: entry.id.clone(),
                name: entry.name.clone(),
                cells,
                area_km2: (cells as f64 * cell_km * cell_km) as f32,
                land_samples: heights.len(),
                heights_m: (!heights.is_empty()).then(|| {
                    [heights[0], percentile(heights, 0.5), percentile(heights, 0.95), heights[heights.len() - 1]]
                }),
            }
        })
        .collect();

    Stats {
        land_cells: mosaic.land_cells,
        land_cells_km2: (mosaic.land_cells as f64 * cell_km * cell_km) as f32,
        box_cells: [cx, cy],
        box_km: [(cx as f64 * cell_km) as f32, (cy as f64 * cell_km) as f32],
        sea_level_m: sea,
        above_sea_samples: above,
        above_sea_km2: (above as f64 * spacing_km * spacing_km) as f32,
        above_sea_box: above_box,
        above_sea_box_km: above_box.map(|[c0, r0, c1, r1]| [((c1 - c0) as f64 * spacing_km) as f32, ((r1 - r0) as f64 * spacing_km) as f32]),
        histogram_bin_m: bin_m,
        histogram: histogram.into_iter().map(|(bin, n)| ((bin as f64 * f64::from(bin_m)) as f32, n)).collect(),
        summit: summit.map(|(height_m, col, row)| {
            let cell = mosaic.cell_of(col, row);
            let (x, y) = mosaic.mw_units(col, row);
            Summit { height_m, sample: [col, row], cell: [cell.0, cell.1], mw_units: [x, y], region: regions.of(cell) }
        }),
        regions: regions_out,
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "LAND: {} cells ({:.1} km2), box {}x{} cells = {:.1} x {:.1} km",
            self.land_cells, self.land_cells_km2, self.box_cells[0], self.box_cells[1], self.box_km[0], self.box_km[1]
        )?;
        write!(f, "above sea ({} m): {:.1} km2", self.sea_level_m, self.above_sea_km2)?;
        if let (Some([c0, r0, c1, r1]), Some([ew, ns])) = (self.above_sea_box, self.above_sea_box_km) {
            write!(f, ", extent {ew:.1} x {ns:.1} km (cols {c0}..{c1}, rows {r0}..{r1})")?;
        }
        writeln!(f)?;
        if let Some(s) = &self.summit {
            let region = self.regions.get(usize::from(s.region)).map_or("", |r| r.id.as_str());
            writeln!(
                f,
                "summit: {:.1} m at cell ({}, {}), units ({:.0}, {:.0}), sample ({}, {}), {region}",
                s.height_m, s.cell[0], s.cell[1], s.mw_units[0], s.mw_units[1], s.sample[0], s.sample[1]
            )?;
        }
        writeln!(f, "histogram ({} m bins, LAND samples):", self.histogram_bin_m)?;
        for (from, n) in &self.histogram {
            writeln!(f, "  {:>6.0} .. {:>6.0} m  {n:>9}", from, from + self.histogram_bin_m)?;
        }
        writeln!(f, "regions:              cells    km2   samples    min   median    p95    max (m)")?;
        for r in &self.regions {
            let label = if r.index == 0 { "(none)" } else { r.id.as_str() };
            write!(f, "  {:>2} {:<26} {:>4} {:>7.1} {:>8}", r.index, label, r.cells, r.area_km2, r.land_samples)?;
            if let Some([lo, med, p95, hi]) = r.heights_m {
                write!(f, " {lo:>7.1} {med:>7.1} {p95:>7.1} {hi:>7.1}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
