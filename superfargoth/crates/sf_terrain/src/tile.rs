//! Canonical tile addressing and the in-memory height grid.

use glam::{DVec2, IVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::{GRID_CELL, WorldPos, partition_extent, partition_origin};

use crate::source::Diagonal;

/// Quads along a tile edge, at every level.
pub const TILE_QUADS: usize = 512;
/// Samples along a tile edge. Neighbors share their border samples.
pub const TILE_SAMPLES: usize = TILE_QUADS + 1;
/// `TILE_QUADS` as i64 and f64, for global index and coordinate math.
pub(crate) const QUADS_I64: i64 = 512;
pub(crate) const QUADS_F64: f64 = 512.0;
const _: () = assert!(TILE_QUADS == 512);
/// Sample spacing of level 0, in meters.
pub const BASE_SPACING: f32 = 0.5;
/// Quads along the edge of a min/max block.
pub const BLOCK_QUADS: usize = 16;
/// Blocks along a tile edge.
pub const BLOCKS: usize = TILE_QUADS / BLOCK_QUADS;
/// Layer-map samples along a tile edge (1 m at level 0).
pub const LAYER_SAMPLES: usize = TILE_QUADS / 2 + 1;
/// Tint samples along a tile edge (2 m at level 0).
pub const TINT_SAMPLES: usize = TILE_QUADS / 4 + 1;

/// How canonical tiles split their quads into triangles: along the diagonal
/// from sample (i, j) to (i+1, j+1). CPU queries, colliders, and the render
/// patch all use it, so they describe the same surface.
pub const CANONICAL_DIAGONAL: Diagonal = Diagonal::Main;

/// A canonical tile: level `level`, index (`x`, `z`) in the partition grid
/// of shift `level` (`sf_coords`). Level 0 is 256 m.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TileCoord {
    pub level: u8,
    pub x: i32,
    pub z: i32,
}

impl TileCoord {
    pub const fn new(level: u8, x: i32, z: i32) -> Self {
        Self { level, x, z }
    }

    /// The level-`level` tile containing `pos` (its XZ; Y is ignored).
    pub fn containing(pos: WorldPos, level: u8) -> Self {
        let p = pos.partition(i32::from(level));
        Self { level, x: p.x, z: p.z }
    }

    /// Sample spacing in meters.
    pub fn spacing(self) -> f32 {
        BASE_SPACING * 2f32.powi(i32::from(self.level))
    }

    /// Edge length in meters.
    pub fn extent(self) -> f32 {
        partition_extent(i32::from(self.level))
    }

    /// Position of sample (0, 0): the tile's minimum XZ corner, at Y = 0.
    pub fn origin(self) -> WorldPos {
        let o = partition_origin(IVec3::new(self.x, 0, self.z), i32::from(self.level));
        WorldPos::new(IVec3::new(o.cell.x, 0, o.cell.z), Vec3::new(o.local.x, 0.0, o.local.z))
    }

    /// The level k+1 tile this one is a quarter of.
    #[must_use]
    pub fn parent(self) -> Self {
        Self { level: self.level + 1, x: self.x >> 1, z: self.z >> 1 }
    }

    /// The four level k−1 tiles, in order (−x −z), (+x −z), (−x +z), (+x +z).
    pub fn children(self) -> [Self; 4] {
        assert!(self.level > 0, "level-0 tiles have no children");
        let (l, x, z) = (self.level - 1, self.x * 2, self.z * 2);
        [Self::new(l, x, z), Self::new(l, x + 1, z), Self::new(l, x, z + 1), Self::new(l, x + 1, z + 1)]
    }

    /// Level-0 global index of sample (0, 0), per axis (x, z).
    pub fn first_sample(self) -> (i64, i64) {
        let q = QUADS_I64 << self.level;
        (i64::from(self.x) * q, i64::from(self.z) * q)
    }

    /// Level-0 global index of sample (i, j).
    pub fn global_sample(self, i: usize, j: usize) -> (i64, i64) {
        let (x0, z0) = self.first_sample();
        let step = |n: usize| i64::try_from(n).expect("small index") << self.level;
        (x0 + step(i), z0 + step(j))
    }
}

/// Continuous level-0 global sample coordinates (x, z) of a position:
/// `g = (absolute meters + 128) / 0.5`. Exact in f64 for any position.
pub fn global_coords(pos: WorldPos) -> DVec2 {
    let axis = |cell: i32, local: f32| (f64::from(cell) * f64::from(GRID_CELL) + f64::from(local) + 128.0) * 2.0;
    DVec2::new(axis(pos.cell.x, pos.local.x), axis(pos.cell.z, pos.local.z))
}

/// Absolute meters of level-0 global sample index `g` on one axis.
pub(crate) fn sample_meters(g: i64) -> f64 {
    // i64 -> f64 is exact below 2^53 samples (far beyond the i32 cell grid).
    #[allow(clippy::cast_precision_loss)]
    let g = g as f64;
    g * 0.5 - 128.0
}

/// A tile's heights in meters (absolute world Y), row-major, x fastest,
/// rows stepping +Z.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeightTile {
    pub coord: TileCoord,
    pub heights: Vec<f32>,
}

impl HeightTile {
    /// A tile filled with one height.
    pub fn flat(coord: TileCoord, height: f32) -> Self {
        Self { coord, heights: vec![height; TILE_SAMPLES * TILE_SAMPLES] }
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.heights[j * TILE_SAMPLES + i]
    }

    pub fn set(&mut self, i: usize, j: usize, h: f32) {
        self.heights[j * TILE_SAMPLES + i] = h;
    }

    /// Minimum and maximum height.
    pub fn range(&self) -> (f32, f32) {
        self.heights.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), &h| (lo.min(h), hi.max(h)))
    }
}
