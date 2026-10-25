//! Toroidal clipmap windows (Losasso & Hoppe, *Geometry Clipmaps*,
//! SIGGRAPH 2004; Asirvatham & Hoppe, *GPU Gems 2* ch. 2): the CPU side of
//! the GPU height, layer, and macro clipmaps.
//!
//! A clipmap level holds a [`CLIPMAP_SIZE`]² window of one pyramid level's
//! samples around the camera. Sample index `s` lives at texel `s mod 512`,
//! so when the window moves only the newly exposed strips are written
//! ([`exposed`], [`toroidal_pieces`]); nothing is copied. The fill functions
//! read canonical tiles by global sample index, so shared tile borders and
//! levels agree exactly with the bake.

use glam::DVec2;

use crate::edits::HeightEdits;
use crate::layers::{LayerId, LayerMap};
use crate::tile::{HeightTile, LAYER_SAMPLES, TILE_QUADS, TILE_SAMPLES, TileCoord};

/// Texels along a clipmap level's edge.
pub const CLIPMAP_SIZE: i64 = 512;

/// A rectangle of samples `[min, min + size)` at one level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SampleRect {
    pub min: [i64; 2],
    pub size: [i64; 2],
}

impl SampleRect {
    pub fn count(&self) -> usize {
        usize::try_from(self.size[0] * self.size[1]).expect("non-negative rect")
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let lo = [self.min[0].max(other.min[0]), self.min[1].max(other.min[1])];
        let hi = [
            (self.min[0] + self.size[0]).min(other.min[0] + other.size[0]),
            (self.min[1] + self.size[1]).min(other.min[1] + other.size[1]),
        ];
        (hi[0] > lo[0] && hi[1] > lo[1]).then(|| Self { min: lo, size: [hi[0] - lo[0], hi[1] - lo[1]] })
    }
}

/// The resident window of one clipmap level: samples `min .. min + 512`
/// of the level's lattice (sample index = level-0 global index `>> shift`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClipWindow {
    pub min: [i64; 2],
}

impl ClipWindow {
    /// The window centered on `center` (level-0 global sample coordinates)
    /// for a lattice of level-0 index `>> shift`, its corner snapped to
    /// multiples of `snap` samples so it moves in strips, not every frame.
    pub fn centered(center: DVec2, shift: u32, snap: i64) -> Self {
        let scale = f64::from(1u32 << shift);
        #[allow(clippy::cast_possible_truncation)]
        let c = |v: f64| (v / scale).floor() as i64;
        let corner = |c: i64| (c - CLIPMAP_SIZE / 2).div_euclid(snap) * snap;
        Self { min: [corner(c(center.x)), corner(c(center.y))] }
    }

    pub fn rect(&self) -> SampleRect {
        SampleRect { min: self.min, size: [CLIPMAP_SIZE; 2] }
    }

    pub fn contains(&self, s: [i64; 2]) -> bool {
        (0..2).all(|a| s[a] >= self.min[a] && s[a] < self.min[a] + CLIPMAP_SIZE)
    }
}

/// Texel of sample index `s` (toroidal addressing).
pub fn texel(s: i64) -> u32 {
    u32::try_from(s.rem_euclid(CLIPMAP_SIZE)).expect("in 0..512")
}

/// Rectangles of `new` that `old` doesn't cover (all of `new` without
/// `old`): at most two, an L shape.
pub fn exposed(old: Option<ClipWindow>, new: ClipWindow) -> Vec<SampleRect> {
    let n = new.rect();
    let Some(old) = old else { return vec![n] };
    let Some(keep) = n.intersect(&old.rect()) else { return vec![n] };
    let mut out = Vec::new();
    // Full-height columns left and right of the kept part.
    if keep.min[0] > n.min[0] {
        out.push(SampleRect { min: n.min, size: [keep.min[0] - n.min[0], n.size[1]] });
    }
    let keep_right = keep.min[0] + keep.size[0];
    if keep_right < n.min[0] + n.size[0] {
        out.push(SampleRect { min: [keep_right, n.min[1]], size: [n.min[0] + n.size[0] - keep_right, n.size[1]] });
    }
    // Rows above and below, only across the kept columns.
    if keep.min[1] > n.min[1] {
        out.push(SampleRect { min: [keep.min[0], n.min[1]], size: [keep.size[0], keep.min[1] - n.min[1]] });
    }
    let keep_bottom = keep.min[1] + keep.size[1];
    if keep_bottom < n.min[1] + n.size[1] {
        out.push(SampleRect {
            min: [keep.min[0], keep_bottom],
            size: [keep.size[0], n.min[1] + n.size[1] - keep_bottom],
        });
    }
    out
}

/// Splits `rect` (at most 512 on a side) into pieces that are contiguous in
/// texel space, each with its first texel.
pub fn toroidal_pieces(rect: SampleRect) -> Vec<(SampleRect, [u32; 2])> {
    let split = |min: i64, size: i64| -> Vec<(i64, i64)> {
        let t = min.rem_euclid(CLIPMAP_SIZE);
        if t + size <= CLIPMAP_SIZE {
            vec![(min, size)]
        } else {
            let first = CLIPMAP_SIZE - t;
            vec![(min, first), (min + first, size - first)]
        }
    };
    let mut out = Vec::new();
    for (z, sz) in split(rect.min[1], rect.size[1]) {
        for (x, sx) in split(rect.min[0], rect.size[0]) {
            out.push((SampleRect { min: [x, z], size: [sx, sz] }, [texel(x), texel(z)]));
        }
    }
    out
}

/// Visits the tiles a rect touches: `per` samples per tile (shared border
/// excluded), calling `f(tile x, tile z, sub-rect in samples)`.
fn for_tiles(rect: SampleRect, per: i64, mut f: impl FnMut(i64, i64, SampleRect)) {
    let (tx0, tz0) = (rect.min[0].div_euclid(per), rect.min[1].div_euclid(per));
    let (tx1, tz1) =
        ((rect.min[0] + rect.size[0] - 1).div_euclid(per), (rect.min[1] + rect.size[1] - 1).div_euclid(per));
    for tz in tz0..=tz1 {
        for tx in tx0..=tx1 {
            let tile = SampleRect { min: [tx * per, tz * per], size: [per, per] };
            if let Some(sub) = rect.intersect(&tile) {
                f(tx, tz, sub);
            }
        }
    }
}

fn idx(v: i64) -> usize {
    usize::try_from(v).expect("non-negative index")
}

fn tile_coord(level: u8, tx: i64, tz: i64) -> Option<TileCoord> {
    Some(TileCoord::new(level, i32::try_from(tx).ok()?, i32::try_from(tz).ok()?))
}

/// Heights of `rect` at height level `level` (sample spacing `0.5·2^level`
/// m, sample index = level-0 global index `>> level`), row-major into
/// `out`. Reads level-`level` tiles through `tile`; samples of missing
/// tiles get `fill`. `edits` (base + edits, terrain.md) are added at the
/// level-0 index of every sample.
pub fn fill_heights<'a>(
    level: u8,
    rect: SampleRect,
    tile: &dyn Fn(TileCoord) -> Option<&'a HeightTile>,
    edits: Option<&HeightEdits>,
    fill: f32,
    out: &mut [f32],
) {
    assert_eq!(out.len(), rect.count(), "fill_heights: `out` has the wrong length");
    let w = idx(rect.size[0]);
    let per = i64::try_from(TILE_QUADS).expect("small");
    for_tiles(rect, per, |tx, tz, sub| {
        let data = tile_coord(level, tx, tz).and_then(tile);
        for z in sub.min[1]..sub.min[1] + sub.size[1] {
            let row = idx(z - rect.min[1]) * w;
            let start = row + idx(sub.min[0] - rect.min[0]);
            let dst = &mut out[start..start + idx(sub.size[0])];
            match data {
                Some(t) => {
                    let j = idx(z - tz * per);
                    let i0 = idx(sub.min[0] - tx * per);
                    dst.copy_from_slice(&t.heights[j * TILE_SAMPLES + i0..j * TILE_SAMPLES + i0 + dst.len()]);
                }
                None => dst.fill(fill),
            }
        }
    });
    if let Some(edits) = edits.filter(|e| !e.is_empty()) {
        for (n, h) in out.iter_mut().enumerate() {
            let (i, j) = (i64::try_from(n % w).expect("small"), i64::try_from(n / w).expect("small"));
            *h += edits.delta((rect.min[0] + i) << level, (rect.min[1] + j) << level);
        }
    }
}

/// Layer samples of `rect` at layer level `level` (spacing `2^level` m,
/// sample index = level-0 global index `>> (level + 1)`): four global
/// layer slots (`slot_of`, e.g. texture array slices) and four weights per
/// sample (summing to 255). Missing tiles get `fill_slot` at full weight.
pub fn fill_layers<'a>(
    level: u8,
    rect: SampleRect,
    tile: &dyn Fn(TileCoord) -> Option<&'a LayerMap>,
    slot_of: &dyn Fn(LayerId) -> u8,
    fill_slot: u8,
    ids: &mut [[u8; 4]],
    weights: &mut [[u8; 4]],
) {
    assert_eq!(ids.len(), rect.count(), "fill_layers: `ids` has the wrong length");
    assert_eq!(weights.len(), rect.count(), "fill_layers: `weights` has the wrong length");
    let w = idx(rect.size[0]);
    let per = i64::try_from(LAYER_SAMPLES - 1).expect("small");
    for_tiles(rect, per, |tx, tz, sub| {
        let data = tile_coord(level, tx, tz).and_then(tile);
        // Palette slot -> global slot, once per tile.
        let map: Vec<u8> = data.map(|m| m.palette.iter().map(|&id| slot_of(id)).collect()).unwrap_or_default();
        for z in sub.min[1]..sub.min[1] + sub.size[1] {
            for x in sub.min[0]..sub.min[0] + sub.size[0] {
                let o = idx(z - rect.min[1]) * w + idx(x - rect.min[0]);
                if let Some(m) = data {
                    let k = idx(z - tz * per) * LAYER_SAMPLES + idx(x - tx * per);
                    let (s, wt) = (m.slots[k], m.weights[k]);
                    ids[o] = s.map(|s| map[usize::from(s)]);
                    weights[o] = [wt[0], wt[1], wt[2], 255 - wt[0] - wt[1] - wt[2]];
                } else {
                    ids[o] = [fill_slot; 4];
                    weights[o] = [255, 0, 0, 0];
                }
            }
        }
    });
}

/// Normals of `rect` at height level `level`, from central differences one
/// sample apart (the same heights as [`fill_heights`], so edits and fill
/// apply), as signed-normalized X and Z (Y is reconstructed: normals point
/// up). The GPU filters these instead of differencing heights per pixel.
pub fn fill_normals<'a>(
    level: u8,
    rect: SampleRect,
    tile: &dyn Fn(TileCoord) -> Option<&'a HeightTile>,
    edits: Option<&HeightEdits>,
    fill: f32,
    out: &mut [[i8; 2]],
) {
    assert_eq!(out.len(), rect.count(), "fill_normals: `out` has the wrong length");
    let grown = SampleRect { min: [rect.min[0] - 1, rect.min[1] - 1], size: [rect.size[0] + 2, rect.size[1] + 2] };
    let mut h = vec![0.0f32; grown.count()];
    fill_heights(level, grown, tile, edits, fill, &mut h);
    let (w, gw) = (idx(rect.size[0]), idx(grown.size[0]));
    let span = 2.0 * 0.5 * f32::from(1u16 << level);
    #[allow(clippy::cast_possible_truncation)]
    let snorm = |v: f32| (v.clamp(-1.0, 1.0) * 127.0).round() as i8;
    for (n, o) in out.iter_mut().enumerate() {
        let (i, j) = (n % w + 1, n / w + 1);
        let dx = (h[j * gw + i + 1] - h[j * gw + i - 1]) / span;
        let dz = (h[(j + 1) * gw + i] - h[(j - 1) * gw + i]) / span;
        let len = (dx * dx + dz * dz + 1.0).sqrt();
        *o = [snorm(-dx / len), snorm(-dz / len)];
    }
}

/// RGBA samples of `rect` from per-tile 257² grids at layer resolution
/// (the macro albedo), level `level`. Missing tiles get `fill`.
pub fn fill_rgba<'a>(
    level: u8,
    rect: SampleRect,
    tile: &dyn Fn(TileCoord) -> Option<&'a [[u8; 4]]>,
    fill: [u8; 4],
    out: &mut [[u8; 4]],
) {
    assert_eq!(out.len(), rect.count(), "fill_rgba: `out` has the wrong length");
    let w = idx(rect.size[0]);
    let per = i64::try_from(LAYER_SAMPLES - 1).expect("small");
    for_tiles(rect, per, |tx, tz, sub| {
        let data = tile_coord(level, tx, tz).and_then(tile);
        for z in sub.min[1]..sub.min[1] + sub.size[1] {
            let start = idx(z - rect.min[1]) * w + idx(sub.min[0] - rect.min[0]);
            let dst = &mut out[start..start + idx(sub.size[0])];
            match data {
                Some(d) => {
                    let k = idx(z - tz * per) * LAYER_SAMPLES + idx(sub.min[0] - tx * per);
                    dst.copy_from_slice(&d[k..k + dst.len()]);
                }
                None => dst.fill(fill),
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposed_strips_cover_exactly_the_new_area() {
        let old = ClipWindow { min: [0, 0] };
        for new in [ClipWindow { min: [16, 0] }, ClipWindow { min: [-32, 48] }, ClipWindow { min: [600, 0] }] {
            let rects = exposed(Some(old), new);
            let mut covered = vec![0u8; 512 * 512];
            for r in &rects {
                assert_eq!(new.rect().intersect(r), Some(*r));
                for z in r.min[1]..r.min[1] + r.size[1] {
                    for x in r.min[0]..r.min[0] + r.size[0] {
                        assert!(!old.contains([x, z]));
                        covered[idx(z - new.min[1]) * 512 + idx(x - new.min[0])] += 1;
                    }
                }
            }
            for z in 0..512 {
                for x in 0..512 {
                    let s = [new.min[0] + x, new.min[1] + z];
                    let expect = u8::from(!old.contains(s));
                    assert_eq!(covered[idx(z) * 512 + idx(x)], expect, "{s:?}");
                }
            }
        }
    }

    #[test]
    fn toroidal_pieces_are_contiguous_in_texels() {
        let r = SampleRect { min: [500, -20], size: [40, 30] };
        let pieces = toroidal_pieces(r);
        assert_eq!(pieces.len(), 4);
        let total: i64 = pieces.iter().map(|(p, _)| p.size[0] * p.size[1]).sum();
        assert_eq!(total, 40 * 30);
        for (p, t) in pieces {
            assert_eq!(t, [texel(p.min[0]), texel(p.min[1])]);
            assert!(i64::from(t[0]) + p.size[0] <= CLIPMAP_SIZE && i64::from(t[1]) + p.size[1] <= CLIPMAP_SIZE);
        }
    }

    #[test]
    fn fill_reads_tiles_by_global_index() {
        let mut a = HeightTile::flat(TileCoord::new(1, 0, 0), 0.0);
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                #[allow(clippy::cast_precision_loss)]
                a.set(i, j, (i + 1000 * j) as f32);
            }
        }
        let tile = |c: TileCoord| (c == a.coord).then_some(&a);
        let rect = SampleRect { min: [510, 3], size: [4, 2] };
        let mut out = vec![0.0; 8];
        fill_heights(1, rect, &tile, None, -5.0, &mut out);
        assert_eq!(out, vec![3510.0, 3511.0, -5.0, -5.0, 4510.0, 4511.0, -5.0, -5.0]);
    }
}
