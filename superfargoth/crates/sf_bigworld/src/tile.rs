//! World-anchored square tiles on the ground plane (spec `superfargoth/docs/specs/big-world-rendering.md`
//! "Data flow"): the unit an instance source fills, the packer packs and the GPU pool holds as one block.
//!
//! Tile `(x, z)` covers world `[x·tile_m, (x+1)·tile_m) × [z·tile_m, (z+1)·tile_m)` in metres. Tile corners are
//! computed in `f64`, so they stay exact far from the origin.

/// A world-anchored tile: [x, x+1) × [z, z+1) in tile units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TileKey {
    /// Tile index along world x.
    pub x: i32,
    /// Tile index along world z.
    pub z: i32,
}

impl TileKey {
    /// The tile with indices `(x, z)`.
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// The tile holding world point `(x, z)` (floor of the tile coordinates; a point on an edge belongs to the
    /// tile on its positive side). Saturates at the `i32` range.
    #[must_use]
    pub fn containing(x: f64, z: f64, tile_m: f32) -> Self {
        let s = f64::from(tile_m);
        // `as` saturates; tile indices of any reachable world fit an i32 by far.
        #[allow(clippy::cast_possible_truncation)]
        let key = Self { x: (x / s).floor() as i32, z: (z / s).floor() as i32 };
        key
    }

    /// World `(x, z)` of the min corner. Exact: an `i32` times an `f32` fits the `f64` mantissa.
    #[must_use]
    pub fn min_corner(self, tile_m: f32) -> [f64; 2] {
        let s = f64::from(tile_m);
        [f64::from(self.x) * s, f64::from(self.z) * s]
    }

    /// Distance on the ground from world point `(x, z)` to this tile's closed square: 0 inside or on an edge, else
    /// the distance to the nearest edge or corner. Plain `sqrt` (correctly rounded), so it is the same on every
    /// machine. A NaN coordinate gives NaN.
    #[must_use]
    pub fn distance_to(self, x: f64, z: f64, tile_m: f32) -> f64 {
        if x.is_nan() || z.is_nan() {
            return f64::NAN;
        }
        let s = f64::from(tile_m);
        let [x0, z0] = self.min_corner(tile_m);
        let dx = (x0 - x).max(x - (x0 + s)).max(0.0);
        let dz = (z0 - z).max(z - (z0 + s)).max(0.0);
        (dx * dx + dz * dz).sqrt()
    }
}
