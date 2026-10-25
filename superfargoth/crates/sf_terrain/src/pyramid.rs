//! The tile pyramid, per-block bounds, and stored height encoding.

use serde::{Deserialize, Serialize};

use crate::tile::{BLOCK_QUADS, BLOCKS, HeightTile, TILE_QUADS, TILE_SAMPLES, TileCoord};

/// Builds level-k tile `coord` from its four children (`TileCoord::children`
/// order): sample (i, j) is the child sample at even indices `(2i, 2j)`
/// modulo the tile, so coarse lattices nest exactly inside fine ones.
/// Missing children read as `fill`.
pub fn downsample(coord: TileCoord, children: [Option<&HeightTile>; 4], fill: f32) -> HeightTile {
    assert!(coord.level > 0, "level-0 tiles come from `upsample`");
    let half = TILE_QUADS / 2;
    // Child and child sample index on one axis. The shared middle line is in
    // both children bit-identically; take the lower one.
    let pick = |i: usize| if i <= half { (0, i * 2) } else { (1, (i - half) * 2) };
    let mut heights = vec![fill; TILE_SAMPLES * TILE_SAMPLES];
    for j in 0..TILE_SAMPLES {
        let (cz, sj) = pick(j);
        for i in 0..TILE_SAMPLES {
            let (cx, si) = pick(i);
            if let Some(child) = children[cz * 2 + cx] {
                debug_assert_eq!(child.coord, coord.children()[cz * 2 + cx]);
                heights[j * TILE_SAMPLES + i] = child.get(si, sj);
            }
        }
    }
    HeightTile { coord, heights }
}

/// The top of the pyramid for `tiles` (level-0 coords): the lowest level
/// at which at most 2×2 tiles cover them all.
///
/// Not "one tile": the grid's corner at −128 m is a tile border at every
/// level (tiles −1 and 0 never share a parent), so a world around it keeps
/// up to four roots, one per quadrant.
pub fn top_level(tiles: impl IntoIterator<Item = TileCoord>) -> u8 {
    let mut it = tiles.into_iter();
    let Some(first) = it.next() else { return 0 };
    let (mut lo, mut hi) = ((first.x, first.z), (first.x, first.z));
    for t in it {
        lo = (lo.0.min(t.x), lo.1.min(t.z));
        hi = (hi.0.max(t.x), hi.1.max(t.z));
    }
    let span = |lo: i32, hi: i32, level: u8| (hi >> level) - (lo >> level);
    let mut level = 0;
    while span(lo.0, hi.0, level) > 1 || span(lo.1, hi.1, level) > 1 {
        level += 1;
    }
    level
}

/// Min/max height per 16×16-quad block (each block's 17×17 samples), for
/// CDLOD node bounds, culling, and raycast acceleration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockBounds {
    /// 32² blocks, row-major.
    pub min: Vec<f32>,
    pub max: Vec<f32>,
}

impl BlockBounds {
    pub fn of(tile: &HeightTile) -> Self {
        let mut min = vec![f32::INFINITY; BLOCKS * BLOCKS];
        let mut max = vec![f32::NEG_INFINITY; BLOCKS * BLOCKS];
        for (j, row) in tile.heights.as_chunks::<TILE_SAMPLES>().0.iter().enumerate() {
            // A sample on a block border belongs to both blocks.
            let bz_hi = (j / BLOCK_QUADS).min(BLOCKS - 1);
            let bz_lo = if j % BLOCK_QUADS == 0 && j > 0 { j / BLOCK_QUADS - 1 } else { bz_hi };
            for bx in 0..BLOCKS {
                let s = &row[bx * BLOCK_QUADS..=bx * BLOCK_QUADS + BLOCK_QUADS];
                let (lo, hi) = s.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(a, b), &h| (a.min(h), b.max(h)));
                for bz in bz_lo..=bz_hi {
                    let k = bz * BLOCKS + bx;
                    min[k] = min[k].min(lo);
                    max[k] = max[k].max(hi);
                }
            }
        }
        Self { min, max }
    }

    /// (min, max) of block (bx, bz).
    pub fn get(&self, bx: usize, bz: usize) -> (f32, f32) {
        let k = bz * BLOCKS + bx;
        (self.min[k], self.max[k])
    }
}

/// Largest quantization step for u16 storage, in meters.
pub const MAX_QUANT_STEP: f32 = 0.02;

/// Stored heights of one tile.
///
/// u16 with a per-tile base and step (the smallest that spans the tile's
/// range, at most [`MAX_QUANT_STEP`]), or f32 when the range is too large
/// (over ~1.3 km). Border samples are kept as f32 in both forms, so
/// neighboring tiles decode bit-identical shared edges whatever their steps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EncodedHeights {
    U16 {
        base: f32,
        step: f32,
        /// 513², row-major; border entries are unused (0).
        values: Vec<u16>,
        /// The border ring: row 0, row 512, then columns 0 and 512 of rows 1..512.
        border: Vec<f32>,
    },
    F32(Vec<f32>),
}

impl EncodedHeights {
    pub fn encode(tile: &HeightTile) -> Self {
        let (lo, hi) = tile.range();
        let step = (hi - lo) / f32::from(u16::MAX);
        if step.is_nan() || step > MAX_QUANT_STEP {
            return Self::F32(tile.heights.clone());
        }
        let mut values = vec![0u16; TILE_SAMPLES * TILE_SAMPLES];
        if step > 0.0 {
            for (v, &h) in values.iter_mut().zip(&tile.heights) {
                // In [0, 65535] by construction; the clamp guards rounding.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let q = ((h - lo) / step).round().clamp(0.0, f32::from(u16::MAX)) as u16;
                *v = q;
            }
        }
        let border = border_indices().map(|k| tile.heights[k]).collect();
        for k in border_indices() {
            values[k] = 0;
        }
        Self::U16 { base: lo, step, values, border }
    }

    pub fn decode(&self, coord: TileCoord) -> HeightTile {
        match self {
            Self::F32(h) => HeightTile { coord, heights: h.clone() },
            Self::U16 { base, step, values, border } => {
                let mut heights: Vec<f32> = values.iter().map(|&q| base + f32::from(q) * step).collect();
                for (k, &h) in border_indices().zip(border) {
                    heights[k] = h;
                }
                HeightTile { coord, heights }
            }
        }
    }

    /// Largest decode error the encoding allows (half a step).
    pub fn max_error(&self) -> f32 {
        match self {
            Self::F32(_) => 0.0,
            Self::U16 { step, .. } => step * 0.5,
        }
    }
}

/// Indices of the border ring: row 0, row 512, then columns 0 and 512 of
/// rows 1..512.
pub(crate) fn border_indices() -> impl Iterator<Item = usize> {
    let n = TILE_SAMPLES;
    let last = n - 1;
    (0..n)
        .chain((0..n).map(move |i| last * n + i))
        .chain((1..last).flat_map(move |j| [j * n, j * n + last]))
}
