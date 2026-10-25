//! Persistent height edits (stack layer 2): sparse bricks of deltas.
//!
//! Only the data model for now (no gameplay edits until M10). A brick is a
//! 32 m square (partition shift −3) of f32 deltas at 0.5 m, one per level-0
//! sample it owns: brick `b` owns global samples `64·b .. 64·b + 63`, so no
//! sample belongs to two bricks. Consumers add the delta to the base sample
//! before interpolating, so an edit is exact at every lattice point and
//! coarser levels see it at their (even) lattice points.

use std::collections::BTreeMap;

use glam::Vec2;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

use crate::tile::{TileCoord, sample_meters};

/// Samples along a brick edge.
pub const BRICK_SAMPLES: usize = 64;
const BRICK_I64: i64 = 64;

/// A brick's index in the 32 m grid (`sf_coords` partition shift −3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BrickCoord {
    pub x: i32,
    pub z: i32,
}

impl BrickCoord {
    /// The brick owning level-0 global sample (gx, gz).
    pub fn of_sample(gx: i64, gz: i64) -> Self {
        let narrow = |g: i64| i32::try_from(g.div_euclid(BRICK_I64)).expect("brick index fits i32");
        Self { x: narrow(gx), z: narrow(gz) }
    }

    /// Level-0 global index of the brick's sample (0, 0).
    pub fn first_sample(self) -> (i64, i64) {
        (i64::from(self.x) * BRICK_I64, i64::from(self.z) * BRICK_I64)
    }
}

/// 64² height deltas in meters, row-major like tiles.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditBrick {
    pub deltas: Vec<f32>,
}

impl EditBrick {
    pub fn zero() -> Self {
        Self { deltas: vec![0.0; BRICK_SAMPLES * BRICK_SAMPLES] }
    }
}

/// Fired when edits change a region. Consumers (colliders, navmesh, grass,
/// shoreline, GPU clipmaps) rebuild `rect`; caches compare `revision`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainChanged {
    /// Minimum XZ corner; Y is ignored.
    pub min: WorldPos,
    /// XZ size in meters.
    pub size: Vec2,
    pub revision: u64,
}

/// The persistent edit layer of one worldspace.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HeightEdits {
    bricks: BTreeMap<BrickCoord, EditBrick>,
    revision: u64,
}

impl HeightEdits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.bricks.is_empty()
    }

    /// Bumped by every change.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn bricks(&self) -> impl Iterator<Item = (&BrickCoord, &EditBrick)> {
        self.bricks.iter()
    }

    pub fn brick(&self, coord: BrickCoord) -> Option<&EditBrick> {
        self.bricks.get(&coord)
    }

    /// The delta at level-0 global sample (gx, gz); 0 without a brick.
    pub fn delta(&self, gx: i64, gz: i64) -> f32 {
        if self.bricks.is_empty() {
            return 0.0;
        }
        let b = BrickCoord::of_sample(gx, gz);
        self.bricks.get(&b).map_or(0.0, |brick| {
            let (x0, z0) = b.first_sample();
            let i = usize::try_from(gx - x0).expect("in brick");
            let j = usize::try_from(gz - z0).expect("in brick");
            brick.deltas[j * BRICK_SAMPLES + i]
        })
    }

    /// Replaces (or adds) a brick.
    pub fn set_brick(&mut self, coord: BrickCoord, brick: EditBrick) -> TerrainChanged {
        assert_eq!(brick.deltas.len(), BRICK_SAMPLES * BRICK_SAMPLES, "brick size");
        self.bricks.insert(coord, brick);
        self.changed(coord)
    }

    pub fn remove_brick(&mut self, coord: BrickCoord) -> Option<TerrainChanged> {
        self.bricks.remove(&coord).map(|_| self.changed(coord))
    }

    /// Whether any brick overlaps `tile` (its samples, borders included).
    pub fn touches(&self, tile: TileCoord) -> bool {
        if self.bricks.is_empty() {
            return false;
        }
        let (x0, z0) = tile.global_sample(0, 0);
        let (x1, z1) = tile.global_sample(crate::TILE_QUADS, crate::TILE_QUADS);
        let (lo, hi) = (BrickCoord::of_sample(x0, z0), BrickCoord::of_sample(x1, z1));
        // Keys order by x, then z.
        self.bricks
            .range(BrickCoord { x: lo.x, z: i32::MIN }..=BrickCoord { x: hi.x, z: i32::MAX })
            .any(|(b, _)| (lo.z..=hi.z).contains(&b.z))
    }

    /// The region a brick affects: its samples plus the quads leading to the
    /// next brick's samples (interpolation reads both).
    fn changed(&mut self, coord: BrickCoord) -> TerrainChanged {
        self.revision += 1;
        let (x0, z0) = coord.first_sample();
        let corner = |g: i64| sample_meters(g - 1);
        let min = WorldPos::from_f64(glam::DVec3::new(corner(x0), 0.0, corner(z0)));
        // 64 samples span 31.5 m; one quad more on each side.
        TerrainChanged { min, size: Vec2::splat(32.5), revision: self.revision }
    }
}
