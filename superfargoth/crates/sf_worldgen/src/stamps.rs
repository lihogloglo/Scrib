//! **Height stamps, format v1** (spec `worldgen-adapter.md` "Same map at 10× (WG-X)" step 4 and "Stamp format
//! v1"): authored height patches an adapter writes into a guide folder, so a generated world keeps chosen places
//! (settlements, landmarks) at their own heights. Generic: a stamp is a named grid of heights and blend weights.
//!
//! A guide folder may hold:
//!
//! - `stamps.json`: `{ "format": 1, "stamps": [{ "name", "origin_m": [x, y], "dims": [w, h], "spacing_m",
//!   "offset" }] }`. `origin_m` is the guide-frame position (x east, y north, metres) of the stamp's sample
//!   (0, 0); row 0 is the north edge, so sample (c, r) sits at `(x + c·s, y − r·s)`. `offset` is an f32 index
//!   into `stamps.f32`. Extra keys are ignored.
//! - `stamps.f32`: per stamp `w·h` heights (final metres) then `w·h` weights (0..1), little-endian f32,
//!   row-major, north up.
//!
//! **Sample.** Bilinear in both grids (a grid point returns its own values bit for bit). Stamps never overlap in
//! weight > 0 (the adapter merges them); where rectangles overlap, the covering stamp of largest weight wins
//! (first in the file on a tie). The runtime blend (`crate::tiles`): `mix(h_gen, h_stamp, w)` with the runtime
//! detail scaled by `1 − w`.
//!
//! **Index.** [`StampIndex`] bins the stamps on a uniform grid aligned to the level-0 terrain lattice (0.5 m,
//! `sf_terrain::sample_meters`), so a lookup tests only the stamps of one bin, and the GPU twin (`tiles.wgsl`
//! `wg_stamp`) finds the same bin from the integer sample index. [`StampIndex::gpu_words`] packs the stamps for
//! the GPU field.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::{Error, Result};

/// One height stamp.
#[derive(Debug, Clone, PartialEq)]
pub struct Stamp {
    pub name: String,
    /// Guide-frame metres (x east, y north) of sample (0, 0), the north-west corner.
    pub origin_m: [f64; 2],
    /// `[w, h]` samples, each at least 2.
    pub dims: [usize; 2],
    /// Sample spacing, metres.
    pub spacing_m: f64,
    /// Final heights, metres, `w·h` row-major, row 0 = north.
    pub heights: Vec<f32>,
    /// Blend weights 0..1, same layout.
    pub weights: Vec<f32>,
}

/// The stamps of a guide (format v1).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StampSet {
    pub stamps: Vec<Stamp>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileV1 {
    format: u32,
    stamps: Vec<EntryV1>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EntryV1 {
    name: String,
    origin_m: [f64; 2],
    dims: [usize; 2],
    spacing_m: f64,
    offset: u64,
}

impl Stamp {
    /// Stamp-grid coordinates (column, row) of a guide-frame point.
    fn grid(&self, x_east: f64, y_north: f64) -> (f64, f64) {
        ((x_east - self.origin_m[0]) / self.spacing_m, (self.origin_m[1] - y_north) / self.spacing_m)
    }

    /// Bilinear (height, weight) at grid coordinates (u, v), `None` outside `[0, w−1] × [0, h−1]` (widened by
    /// [`EDGE_EPS`], so an edge sample found through f64 metres still counts).
    pub fn sample_grid(&self, u: f64, v: f64) -> Option<(f32, f32)> {
        let [w, h] = self.dims;
        let (wm, hm) = ((w - 1) as f64, (h - 1) as f64);
        if !(u >= -EDGE_EPS && v >= -EDGE_EPS && u <= wm + EDGE_EPS && v <= hm + EDGE_EPS) {
            return None;
        }
        let (u, v) = (u.clamp(0.0, wm), v.clamp(0.0, hm));
        let (i, j) = ((u.floor() as usize).min(w - 1), (v.floor() as usize).min(h - 1));
        Some(bilinear(&self.heights, &self.weights, w, h, i, j, (u - i as f64) as f32, (v - j as f64) as f32))
    }

    /// Bilinear (height, weight) at a guide-frame point, `None` outside the stamp's rectangle.
    pub fn sample(&self, x_east: f64, y_north: f64) -> Option<(f32, f32)> {
        let (u, v) = self.grid(x_east, y_north);
        self.sample_grid(u, v)
    }

    /// The stamp's rectangle, guide frame: `[x0, y0, x1, y1]` with x0 west, y0 south.
    pub fn rect_m(&self) -> [f64; 4] {
        let s = self.spacing_m;
        let [ox, oy] = self.origin_m;
        [ox, oy - (self.dims[1] - 1) as f64 * s, ox + (self.dims[0] - 1) as f64 * s, oy]
    }

    fn validate(&self) -> Result<()> {
        let [w, h] = self.dims;
        let bad = |m: String| Err(Error::Invalid(format!("stamp {:?}: {m}", self.name)));
        if w < 2 || h < 2 || w.checked_mul(h).is_none_or(|n| n >= 1 << 28) {
            return bad(format!("dims {w}x{h} out of range"));
        }
        if !(self.spacing_m.is_finite() && self.spacing_m > 0.0) {
            return bad("spacing_m must be > 0".into());
        }
        if !(self.origin_m[0].is_finite() && self.origin_m[1].is_finite()) {
            return bad("origin_m must be finite".into());
        }
        if self.heights.len() != w * h || self.weights.len() != w * h {
            return bad("heights/weights length is not w·h".into());
        }
        if self.heights.iter().any(|v| !v.is_finite()) {
            return bad("non-finite height".into());
        }
        if self.weights.iter().any(|v| !(0.0..=1.0).contains(v)) {
            return bad("weight outside 0..1".into());
        }
        Ok(())
    }
}

/// Bilinear in f32 at cell (i, j) + (tx, ty); `a + (b − a)·t` keeps grid points and flat patches exact.
#[allow(clippy::too_many_arguments)]
fn bilinear(hs: &[f32], ws: &[f32], w: usize, h: usize, i: usize, j: usize, tx: f32, ty: f32) -> (f32, f32) {
    let (i1, j1) = ((i + 1).min(w - 1), (j + 1).min(h - 1));
    let lerp2 = |g: &[f32]| {
        let (a, b) = (g[j * w + i], g[j * w + i1]);
        let (c, d) = (g[j1 * w + i], g[j1 * w + i1]);
        let top = a + (b - a) * tx;
        let bot = c + (d - c) * tx;
        top + (bot - top) * ty
    };
    (lerp2(hs), lerp2(ws))
}

/// Picks the covering stamp of largest weight (first on a tie).
fn pick(best: &mut Option<(f32, f32)>, s: Option<(f32, f32)>) {
    if let Some(s) = s
        && best.is_none_or(|b| s.1 > b.1)
    {
        *best = Some(s);
    }
}

impl StampSet {
    /// Reads `stamps.json` + `stamps.f32` from a guide folder; `None` when `stamps.json` is absent.
    pub fn read(dir: &Path) -> Result<Option<Self>> {
        let json = dir.join("stamps.json");
        if !json.exists() {
            return Ok(None);
        }
        let file: FileV1 = serde_json::from_reader(BufReader::new(File::open(json)?))?;
        if file.format != 1 {
            return Err(Error::Invalid(format!("stamps format {} (v1 expected)", file.format)));
        }
        let raw = std::fs::read(dir.join("stamps.f32"))?;
        if raw.len() % 4 != 0 {
            return Err(Error::Invalid("stamps.f32 length is not a multiple of 4".into()));
        }
        let floats: Vec<f32> = raw.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect();
        let mut stamps = Vec::with_capacity(file.stamps.len());
        for e in file.stamps {
            let n = e.dims[0].saturating_mul(e.dims[1]);
            let off = usize::try_from(e.offset).map_err(|_| Error::Invalid("stamp offset too large".into()))?;
            let end = off.checked_add(n.saturating_mul(2)).filter(|&end| end <= floats.len());
            let Some(end) = end else {
                return Err(Error::Invalid(format!("stamp {:?}: offset/dims past stamps.f32", e.name)));
            };
            let s = Stamp {
                name: e.name,
                origin_m: e.origin_m,
                dims: e.dims,
                spacing_m: e.spacing_m,
                heights: floats[off..off + n].to_vec(),
                weights: floats[off + n..end].to_vec(),
            };
            s.validate()?;
            stamps.push(s);
        }
        Ok(Some(Self { stamps }))
    }

    /// Writes `stamps.json` + `stamps.f32` into `dir` (stamps packed in order).
    pub fn write(&self, dir: &Path) -> Result<()> {
        let mut entries = Vec::with_capacity(self.stamps.len());
        let mut bytes = Vec::new();
        for s in &self.stamps {
            s.validate()?;
            entries.push(EntryV1 {
                name: s.name.clone(),
                origin_m: s.origin_m,
                dims: s.dims,
                spacing_m: s.spacing_m,
                offset: (bytes.len() / 4) as u64,
            });
            for v in s.heights.iter().chain(&s.weights) {
                bytes.extend_from_slice(&v.to_le_bytes());
            }
        }
        std::fs::create_dir_all(dir)?;
        std::fs::write(dir.join("stamps.f32"), bytes)?;
        let json = serde_json::to_string_pretty(&FileV1 { format: 1, stamps: entries })?;
        std::fs::write(dir.join("stamps.json"), json)?;
        Ok(())
    }

    /// Sets every weight of stamp `index` to 0, so it no longer changes the ground; its heights, rectangle and
    /// every stamp index stay (other files key stamps by index).
    pub fn zero_weights(&mut self, index: usize) -> Result<()> {
        let n = self.stamps.len();
        let s = self.stamps.get_mut(index).ok_or_else(|| Error::Invalid(format!("stamp {index} of {n}: no such stamp")))?;
        s.weights.fill(0.0);
        Ok(())
    }

    /// Appends `other`'s stamps after these (existing stamps keep their index and, once written, their offset).
    /// Refuses, changing nothing, when an appended stamp is invalid or a sample of weight > 0 of one stamp lies
    /// where another (existing or appended earlier) has weight > 0: the format's no-overlap rule.
    pub fn append(&mut self, other: StampSet) -> Result<()> {
        let n = self.stamps.len();
        for (k, b) in other.stamps.iter().enumerate() {
            b.validate()?;
            for a in self.stamps.iter().chain(&other.stamps[..k]) {
                if let Some([x, y]) = overlap(a, b).or_else(|| overlap(b, a)) {
                    return Err(Error::Invalid(format!(
                        "appended stamp {:?} (index {}) overlaps stamp {:?} at weight > 0 near ({x:.2}, {y:.2}) m",
                        b.name,
                        n + k,
                        a.name
                    )));
                }
            }
        }
        self.stamps.extend(other.stamps);
        Ok(())
    }

    /// Guide frame (x east, y north) metres → (height m, weight 0..1) of the stamp covering the point, bilinear;
    /// `None` outside every stamp. Tests every stamp: hot paths use [`StampIndex::sample`] (same result).
    pub fn sample(&self, x_east: f64, y_north: f64) -> Option<(f32, f32)> {
        let mut best = None;
        for s in &self.stamps {
            pick(&mut best, s.sample(x_east, y_north));
        }
        best
    }
}

/// The first sample of weight > 0 of `a` (guide frame metres) where `b` samples a weight > 0.
fn overlap(a: &Stamp, b: &Stamp) -> Option<[f64; 2]> {
    let (ra, rb) = (a.rect_m(), b.rect_m());
    let e = EDGE_EPS * a.spacing_m.max(b.spacing_m);
    if ra[2] < rb[0] - e || rb[2] < ra[0] - e || ra[3] < rb[1] - e || rb[3] < ra[1] - e {
        return None;
    }
    let w = a.dims[0];
    a.weights.iter().enumerate().find_map(|(k, &wa)| {
        let p = [a.origin_m[0] + (k % w) as f64 * a.spacing_m, a.origin_m[1] - (k / w) as f64 * a.spacing_m];
        (wa > 0.0 && b.sample(p[0], p[1]).is_some_and(|(_, wb)| wb > 0.0)).then_some(p)
    })
}

/// Grid units a point may sit past a stamp's edge and still sample it (f64 rounding of metres → grid).
// tunables: const (maths: rounding tolerance, far below any sample spacing)
const EDGE_EPS: f64 = 1e-6;

/// Samples per min/max cell of a stamp (bounds queries).
// tunables: const (layout: block size of the per-stamp min/max grid, no quality trade)
const MM_CELL: usize = 8;

/// Most bins per axis of the index grid.
// tunables: const (capacity: caps the bin grid at 256² for far-apart small stamps)
const MAX_BINS_AXIS: f64 = 256.0;

/// Level-0 lattice coordinates (half metres, `sf_terrain::sample_meters` inverse) of a guide-frame point:
/// engine X = x, Z = −y, sample g = 2·m + 256.
fn lattice(x_east: f64, y_north: f64) -> (f64, f64) {
    (2.0 * x_east + 256.0, 256.0 - 2.0 * y_north)
}

/// A bin-grid index over a [`StampSet`] (lookups test one bin's stamps), plus per-stamp min/max cells.
#[derive(Debug, Clone)]
pub struct StampIndex {
    set: StampSet,
    /// Lattice coordinates of bin (0, 0)'s corner (integers).
    g0: [i64; 2],
    /// Bin edge in lattice samples (≥ 1).
    edge: i64,
    nx: usize,
    ny: usize,
    /// CSR: bin `b`'s stamps are `items[start[b]..start[b + 1]]`, ascending.
    start: Vec<u32>,
    items: Vec<u32>,
    /// Per stamp: `(cells_w, min, max)` over `MM_CELL`-sample cells (shared edges included).
    minmax: Vec<(usize, Vec<f32>, Vec<f32>)>,
}

impl StampIndex {
    pub fn new(set: StampSet) -> Self {
        let rects: Vec<[f64; 4]> = set
            .stamps
            .iter()
            .map(|s| {
                // Widened by the edge tolerance (in lattice samples, 2 per metre) so the bins agree with it.
                let e = 2.0 * EDGE_EPS * s.spacing_m + 1e-9;
                let [x0, y0, x1, y1] = s.rect_m();
                let (gx0, gy0) = lattice(x0, y1);
                let (gx1, gy1) = lattice(x1, y0);
                [gx0 - e, gy0 - e, gx1 + e, gy1 + e]
            })
            .collect();
        let (mut lo, mut hi, mut big) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2], 1.0f64);
        for r in &rects {
            lo = [lo[0].min(r[0]), lo[1].min(r[1])];
            hi = [hi[0].max(r[2]), hi[1].max(r[3])];
            big = big.max(r[2] - r[0]).max(r[3] - r[1]);
        }
        let (g0, edge, nx, ny) = if rects.is_empty() {
            ([0, 0], 1, 0, 0)
        } else {
            let g0 = [lo[0].floor() as i64, lo[1].floor() as i64];
            let span = (hi[0] - g0[0] as f64).max(hi[1] - g0[1] as f64);
            let edge = big.max(span / MAX_BINS_AXIS).ceil().max(1.0) as i64;
            let n = |a: usize| ((hi[a] - g0[a] as f64) / edge as f64).floor() as usize + 1;
            (g0, edge, n(0), n(1))
        };
        let bin = |v: f64, a: usize| ((v - g0[a] as f64) / edge as f64).floor() as usize;
        let mut lists = vec![Vec::new(); nx * ny];
        for (k, r) in rects.iter().enumerate() {
            for by in bin(r[1], 1)..=bin(r[3], 1).min(ny - 1) {
                for bx in bin(r[0], 0)..=bin(r[2], 0).min(nx - 1) {
                    lists[by * nx + bx].push(k as u32);
                }
            }
        }
        let mut start = Vec::with_capacity(lists.len() + 1);
        let mut items = Vec::new();
        start.push(0);
        for l in lists {
            items.extend(l);
            start.push(items.len() as u32);
        }
        let minmax = set.stamps.iter().map(minmax_cells).collect();
        Self { set, g0, edge, nx, ny, start, items, minmax }
    }

    pub fn set(&self) -> &StampSet {
        &self.set
    }

    /// The stamps of the bin holding lattice point (gx, gy).
    fn bin_items(&self, gx: f64, gy: f64) -> &[u32] {
        let bx = ((gx - self.g0[0] as f64) / self.edge as f64).floor();
        let by = ((gy - self.g0[1] as f64) / self.edge as f64).floor();
        if !(bx >= 0.0 && by >= 0.0 && bx < self.nx as f64 && by < self.ny as f64) {
            return &[];
        }
        let b = by as usize * self.nx + bx as usize;
        &self.items[self.start[b] as usize..self.start[b + 1] as usize]
    }

    /// [`StampSet::sample`] through the bins.
    pub fn sample(&self, x_east: f64, y_north: f64) -> Option<(f32, f32)> {
        let (gx, gy) = lattice(x_east, y_north);
        let mut best = None;
        for &k in self.bin_items(gx, gy) {
            pick(&mut best, self.set.stamps[k as usize].sample(x_east, y_north));
        }
        best
    }

    /// Min/max stamp height over the guide-frame rectangle `[x0, y0, x1, y1]` (x0 ≤ x1, y0 ≤ y1), conservative
    /// (whole min/max cells); `None` when no stamp reaches it.
    pub fn height_range(&self, rect: [f64; 4]) -> Option<(f32, f32)> {
        let mut out: Option<(f32, f32)> = None;
        for (s, (cw, mn, mx)) in self.set.stamps.iter().zip(&self.minmax) {
            let r = s.rect_m();
            if rect[2] < r[0] || rect[0] > r[2] || rect[3] < r[1] || rect[1] > r[3] {
                continue;
            }
            let (u0, v1) = s.grid(rect[0], rect[1]);
            let (u1, v0) = s.grid(rect[2], rect[3]);
            let ch = mn.len() / cw;
            let cell = |t: f64, n: usize| ((t.max(0.0) / MM_CELL as f64).floor() as usize).min(n - 1);
            for cj in cell(v0, ch)..=cell(v1, ch) {
                for ci in cell(u0, *cw)..=cell(u1, *cw) {
                    let k = cj * cw + ci;
                    out = Some(out.map_or((mn[k], mx[k]), |(a, b)| (a.min(mn[k]), b.max(mx[k]))));
                }
            }
        }
        out
    }

    /// Length of [`Self::gpu_words`].
    pub fn gpu_word_count(&self) -> usize {
        let grids: usize = self.set.stamps.iter().map(|s| 2 * s.heights.len()).sum();
        9 + 8 * self.set.stamps.len() + self.start.len() + self.items.len() + grids
    }

    /// The stamps as GPU field words starting at word `base` (`tiles.wgsl` `wg_stamp`): a 9-word header
    /// (count, bins x/y, bin corner x/y, bin edge, then the absolute offsets of the table, the bin starts and
    /// the bin items), one 8-word record per stamp (lattice reference x/y, grid offsets u/v, grid units per
    /// lattice sample, w, h, absolute offset of its heights; weights follow), bin starts, items, grids.
    pub fn gpu_words(&self, base: usize) -> Vec<u32> {
        let n = self.set.stamps.len();
        let table = base + 9;
        let starts = table + 8 * n;
        let items = starts + self.start.len();
        let mut grid = items + self.items.len();
        let mut d = Vec::with_capacity(grid - base);
        let w32 = |v: usize| u32::try_from(v).expect("stamp field offset fits u32");
        d.extend([
            w32(n),
            w32(self.nx),
            w32(self.ny),
            (self.g0[0] as i32).cast_unsigned(),
            (self.g0[1] as i32).cast_unsigned(),
            self.edge as u32,
            w32(table),
            w32(starts),
            w32(items),
        ]);
        for s in &self.set.stamps {
            // u = (x − ox) / s with x = g / 2 − 128: u = (g − ref)·(0.5 / s) + ((ref / 2 − 128 − ox) / s), the
            // integer difference exact on the GPU; v likewise with y = 128 − g / 2.
            let (lx, ly) = lattice(s.origin_m[0], s.origin_m[1]);
            let (rx, ry) = (lx.round(), ly.round());
            let cu = ((rx * 0.5 - 128.0 - s.origin_m[0]) / s.spacing_m) as f32;
            let cv = ((s.origin_m[1] - 128.0 + ry * 0.5) / s.spacing_m) as f32;
            d.extend([
                (rx as i32).cast_unsigned(),
                (ry as i32).cast_unsigned(),
                cu.to_bits(),
                cv.to_bits(),
                ((0.5 / s.spacing_m) as f32).to_bits(),
                w32(s.dims[0]),
                w32(s.dims[1]),
                w32(grid),
            ]);
            grid += 2 * s.heights.len();
        }
        d.extend(&self.start);
        d.extend(&self.items);
        for s in &self.set.stamps {
            d.extend(s.heights.iter().chain(&s.weights).map(|v| v.to_bits()));
        }
        d
    }
}

fn minmax_cells(s: &Stamp) -> (usize, Vec<f32>, Vec<f32>) {
    let [w, h] = s.dims;
    let (cw, ch) = ((w - 1).div_ceil(MM_CELL), (h - 1).div_ceil(MM_CELL));
    let (mut mn, mut mx) = (vec![f32::INFINITY; cw * ch], vec![f32::NEG_INFINITY; cw * ch]);
    for cj in 0..ch {
        for ci in 0..cw {
            let k = cj * cw + ci;
            for r in cj * MM_CELL..=((cj + 1) * MM_CELL).min(h - 1) {
                for c in ci * MM_CELL..=((ci + 1) * MM_CELL).min(w - 1) {
                    mn[k] = mn[k].min(s.heights[r * w + c]);
                    mx[k] = mx[k].max(s.heights[r * w + c]);
                }
            }
        }
    }
    (cw, mn, mx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stamp(name: &str, origin: [f64; 2], dims: [usize; 2], sp: f64, seed: u32) -> Stamp {
        let n = dims[0] * dims[1];
        let mut x = seed.wrapping_mul(2_654_435_761).max(1);
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x >> 8) as f32 / 16_777_216.0
        };
        let heights = (0..n).map(|_| 100.0 * next() - 20.0).collect();
        let weights = (0..n).map(|_| next()).collect();
        Stamp { name: name.into(), origin_m: origin, dims, spacing_m: sp, heights, weights }
    }

    #[test]
    fn round_trip_exact() {
        let set = StampSet {
            stamps: vec![stamp("a", [10.25, 300.0], [5, 4], 1.8288, 1), stamp("b", [-500.0, -20.5], [3, 7], 2.0, 2)],
        };
        let dir = std::env::temp_dir().join(format!("sf_worldgen_stamps_rt_{}", std::process::id()));
        set.write(&dir).unwrap();
        let back = StampSet::read(&dir).unwrap().unwrap();
        assert_eq!(back, set);
        std::fs::remove_dir_all(&dir).ok();
        assert!(StampSet::read(&dir).unwrap().is_none());
    }

    #[test]
    fn sample_exact_at_grid_points_and_none_outside() {
        let s = stamp("a", [10.25, 300.0], [5, 4], 1.8288, 3);
        let set = StampSet { stamps: vec![s.clone()] };
        for r in 0..4 {
            for c in 0..5 {
                let (x, y) = (10.25 + c as f64 * 1.8288, 300.0 - r as f64 * 1.8288);
                // The grid maths in f64 may land an ulp beside the point; sample the grid directly too.
                assert_eq!(s.sample_grid(c as f64, r as f64), Some((s.heights[r * 5 + c], s.weights[r * 5 + c])));
                let (h, _) = set.sample(x, y).unwrap();
                assert!((h - s.heights[r * 5 + c]).abs() < 1e-3);
            }
        }
        // Midpoint of a cell is the mean of its 4 corners (bilinear).
        let m = s.sample_grid(1.5, 2.5).unwrap().0;
        let mean = (s.heights[2 * 5 + 1] + s.heights[2 * 5 + 2] + s.heights[3 * 5 + 1] + s.heights[3 * 5 + 2]) / 4.0;
        assert!((m - mean).abs() < 1e-4);
        assert_eq!(set.sample(10.0, 299.0), None);
        assert_eq!(set.sample(10.25 + 4.0 * 1.8288 + 0.01, 299.0), None);
        assert_eq!(set.sample(12.0, 300.1), None);
        assert_eq!(set.sample(12.0, 300.0 - 3.0 * 1.8288 - 0.01), None);
    }

    #[test]
    fn index_equals_brute_force() {
        let mut stamps = Vec::new();
        let mut x = 12_345u32;
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            f64::from(x >> 8) / 16_777_216.0
        };
        for k in 0..300 {
            let o = [next() * 20_000.0 - 10_000.0, next() * 20_000.0 - 10_000.0];
            let dims = [2 + (next() * 60.0) as usize, 2 + (next() * 60.0) as usize];
            stamps.push(stamp(&format!("s{k}"), o, dims, 0.5 + next() * 4.0, k));
        }
        // One big one and a far-away tiny one.
        stamps.push(stamp("big", [-3000.0, 3000.0], [400, 300], 10.0, 999));
        stamps.push(stamp("far", [90_000.0, -80_000.0], [2, 2], 1.0, 1000));
        let set = StampSet { stamps };
        let idx = StampIndex::new(set.clone());
        assert!(idx.nx <= 257 && idx.ny <= 257);
        let mut hits = 0;
        for _ in 0..200_000 {
            let (x, y) = (next() * 24_000.0 - 12_000.0, next() * 24_000.0 - 12_000.0);
            let a = set.sample(x, y);
            assert_eq!(idx.sample(x, y), a, "at ({x}, {y})");
            hits += usize::from(a.is_some());
        }
        assert!(hits > 1000, "{hits}");
        assert_eq!(idx.sample(90_000.5, -80_000.5), set.sample(90_000.5, -80_000.5));
        assert!(idx.sample(90_000.5, -80_000.5).is_some());
        // Lattice points (what the tiles ask for).
        for gx in (-2000..2000).step_by(7) {
            for gy in (-2000..2000).step_by(11) {
                let (x, y) = (f64::from(gx) * 0.5 - 128.0, 128.0 - f64::from(gy) * 0.5);
                assert_eq!(idx.sample(x, y), set.sample(x, y));
            }
        }
    }

    fn bits(s: &Stamp) -> (Vec<u32>, Vec<u32>) {
        (s.heights.iter().map(|v| v.to_bits()).collect(), s.weights.iter().map(|v| v.to_bits()).collect())
    }

    fn offsets(dir: &Path) -> Vec<u64> {
        let f: FileV1 = serde_json::from_slice(&std::fs::read(dir.join("stamps.json")).unwrap()).unwrap();
        f.stamps.iter().map(|e| e.offset).collect()
    }

    #[test]
    fn zero_weights_touches_only_that_stamp() {
        let set = StampSet {
            stamps: vec![
                stamp("a", [0.0, 0.0], [5, 4], 2.0, 1),
                stamp("b", [100.0, 0.0], [3, 7], 1.5, 2),
                stamp("c", [200.0, 0.0], [4, 4], 1.0, 3),
            ],
        };
        let mut z = set.clone();
        z.zero_weights(1).unwrap();
        assert_eq!(z.stamps.len(), 3);
        for k in [0, 2] {
            assert_eq!(z.stamps[k], set.stamps[k]);
            assert_eq!(bits(&z.stamps[k]), bits(&set.stamps[k]));
        }
        let (b, b0) = (&z.stamps[1], &set.stamps[1]);
        assert_eq!((&b.name, b.origin_m, b.dims, b.spacing_m.to_bits()), (&b0.name, b0.origin_m, b0.dims, b0.spacing_m.to_bits()));
        assert_eq!(bits(b).0, bits(b0).0);
        assert!(b.weights.iter().all(|w| w.to_bits() == 0));
        // Every sample of the zeroed stamp reads weight 0.
        for r in 0..7 {
            for c in 0..3 {
                assert_eq!(b.sample_grid(f64::from(c), f64::from(r)).unwrap().1.to_bits(), 0);
            }
        }
        assert!(z.zero_weights(3).is_err());
    }

    #[test]
    fn append_keeps_offsets_and_reads_back_exact() {
        let set = StampSet { stamps: vec![stamp("a", [0.0, 0.0], [5, 4], 2.0, 1), stamp("b", [100.0, 0.0], [3, 7], 1.5, 2)] };
        let base = std::env::temp_dir().join(format!("sf_worldgen_stamps_app_{}", std::process::id()));
        let (d0, d1) = (base.join("before"), base.join("after"));
        set.write(&d0).unwrap();
        let add = stamp("new", [-300.25, 40.0], [6, 3], 0.75, 9);
        let mut more = set.clone();
        more.append(StampSet { stamps: vec![add.clone()] }).unwrap();
        more.write(&d1).unwrap();
        let (o0, o1) = (offsets(&d0), offsets(&d1));
        assert_eq!(o1.len(), 3);
        assert_eq!(o1[..2], o0[..]);
        let (f0, f1) = (std::fs::read(d0.join("stamps.f32")).unwrap(), std::fs::read(d1.join("stamps.f32")).unwrap());
        assert_eq!(f1[..f0.len()], f0[..]);
        let back = StampSet::read(&d1).unwrap().unwrap();
        assert_eq!(back.stamps[..2], set.stamps[..]);
        assert_eq!((&back.stamps[2].name, back.stamps[2].origin_m, back.stamps[2].dims), (&add.name, add.origin_m, add.dims));
        assert_eq!(back.stamps[2].spacing_m.to_bits(), add.spacing_m.to_bits());
        assert_eq!(bits(&back.stamps[2]), bits(&add));
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn append_refuses_overlap_accepts_zero_weight_touch() {
        let ones = |name: &str, origin: [f64; 2], dims: [usize; 2]| {
            let n = dims[0] * dims[1];
            Stamp { name: name.into(), origin_m: origin, dims, spacing_m: 1.0, heights: vec![5.0; n], weights: vec![1.0; n] }
        };
        let set = StampSet { stamps: vec![ones("old", [0.0, 0.0], [5, 5])] };
        // Overlaps old's interior.
        let mut s = set.clone();
        let e = s.append(StampSet { stamps: vec![ones("new", [3.0, -1.0], [4, 4])] }).unwrap_err().to_string();
        assert!(e.contains("\"new\"") && e.contains("\"old\""), "{e}");
        assert_eq!(s, set, "a refused append changes nothing");
        // Two appended stamps overlapping each other are refused too.
        let far = |name: &str| ones(name, [50.0, 0.0], [3, 3]);
        let e = s.append(StampSet { stamps: vec![far("p"), far("q")] }).unwrap_err().to_string();
        assert!(e.contains("\"q\"") && e.contains("\"p\""), "{e}");
        // Shares old's east column (x = 4) where the new stamp's weight is 0: accepted.
        let mut touch = ones("touch", [4.0, 0.0], [4, 5]);
        for r in 0..5 {
            touch.weights[r * 4] = 0.0;
        }
        s.append(StampSet { stamps: vec![touch.clone()] }).unwrap();
        assert_eq!(s.stamps.len(), 2);
        assert_eq!(s.stamps[1], touch);
        // The same with weight > 0 on the shared column: refused.
        let mut s = set.clone();
        assert!(s.append(StampSet { stamps: vec![ones("touch", [4.0, 0.0], [4, 5])] }).is_err());
    }

    #[test]
    fn height_range_is_conservative() {
        let s = stamp("a", [0.0, 0.0], [37, 21], 1.5, 7);
        let idx = StampIndex::new(StampSet { stamps: vec![s.clone()] });
        let rect = [10.0, -20.0, 30.0, -5.0];
        let (lo, hi) = idx.height_range(rect).unwrap();
        for k in 0..=100 {
            for l in 0..=100 {
                let x = rect[0] + (rect[2] - rect[0]) * f64::from(k) / 100.0;
                let y = rect[1] + (rect[3] - rect[1]) * f64::from(l) / 100.0;
                let (h, _) = idx.sample(x, y).unwrap();
                assert!(h >= lo && h <= hi);
            }
        }
        assert!(idx.height_range([100.0, 0.0, 200.0, 10.0]).is_none());
    }
}
