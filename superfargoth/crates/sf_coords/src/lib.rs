//! World coordinates for huge worlds.
//!
//! A position is an integer grid cell plus an f32 offset from that cell's
//! center ([`WorldPos`]). This stays exact to micrometers anywhere in a
//! world hundreds of thousands of kilometers across, while every hot path
//! keeps using f32. Rendering converts to positions relative to a floating
//! origin (`big_space` on the Bevy side); records, saves, and bake products
//! store `WorldPos` and never a raw f32 world position.
//!
//! Engine space: right-handed, Y up, -Z forward, meters.
//! Spec: `docs/specs/world-coordinates.md`.

mod partition;
mod serde_impl;

pub use partition::{partition_extent, partition_origin};

use glam::{DVec3, IVec3, Quat, Vec3};

/// Edge length of a grid cell in meters. One constant for every worldspace,
/// so a stored position means the same thing everywhere. A power of two, so
/// every derived partition (streaming cells, terrain tiles) nests exactly.
pub const GRID_CELL: f32 = 256.0;

const HALF_CELL: f32 = GRID_CELL / 2.0;
const CELL_F64: f64 = GRID_CELL as f64;

/// A position in one worldspace.
///
/// Canonical form: each `local` axis is in `[-128, 128)`, an offset from the
/// center of `cell`. Constructors and arithmetic always return canonical
/// values, so equal positions compare equal.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldPos {
    pub cell: IVec3,
    pub local: Vec3,
}

impl WorldPos {
    pub const ORIGIN: Self = Self { cell: IVec3::ZERO, local: Vec3::ZERO };

    /// Builds a canonical position from any cell and offset.
    pub fn new(cell: IVec3, local: Vec3) -> Self {
        let (cx, x) = split(f64::from(local.x));
        let (cy, y) = split(f64::from(local.y));
        let (cz, z) = split(f64::from(local.z));
        Self { cell: cell + to_i32(cx, cy, cz), local: Vec3::new(x, y, z) }
    }

    /// From absolute meters. Exact to well under a micrometer for any
    /// position the f64 itself can hold that precisely.
    pub fn from_f64(p: DVec3) -> Self {
        let (cx, x) = split(p.x);
        let (cy, y) = split(p.y);
        let (cz, z) = split(p.z);
        Self { cell: to_i32(cx, cy, cz), local: Vec3::new(x, y, z) }
    }

    /// Absolute meters. Loses precision only past f64's ~15 significant digits.
    pub fn to_f64(self) -> DVec3 {
        self.cell.as_dvec3() * CELL_F64 + self.local.as_dvec3()
    }

    /// Moves by `d` meters, staying canonical.
    #[must_use]
    pub fn offset(self, d: Vec3) -> Self {
        let sum = self.local.as_dvec3() + d.as_dvec3();
        let (cx, x) = split(sum.x);
        let (cy, y) = split(sum.y);
        let (cz, z) = split(sum.z);
        Self { cell: self.cell + to_i32(cx, cy, cz), local: Vec3::new(x, y, z) }
    }

    /// Exact vector from `self` to `to` in meters, at any distance.
    pub fn delta(self, to: Self) -> DVec3 {
        let cells = to.cell.as_i64vec3() - self.cell.as_i64vec3();
        cells.as_dvec3() * CELL_F64 + (to.local.as_dvec3() - self.local.as_dvec3())
    }

    /// `delta` narrowed to f32: for positions known to be near each other.
    pub fn delta_f32(self, to: Self) -> Vec3 {
        self.delta(to).as_vec3()
    }

    /// Index of the partition cell containing this position, in the grid of
    /// edge `GRID_CELL · 2^shift` (shift may be negative: -1 is 128 m).
    /// Partitions share the grid's cell corners, so they nest exactly.
    pub fn partition(self, shift: i32) -> IVec3 {
        partition::index(self, shift)
    }

    /// The 24-byte little-endian binary form used by saves and bake products:
    /// `i32 × 3` cell, then `f32 × 3` local.
    pub fn to_bytes(self) -> [u8; 24] {
        let mut out = [0; 24];
        for (i, v) in self.cell.to_array().into_iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        for (i, v) in self.local.to_array().into_iter().enumerate() {
            out[12 + i * 4..16 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        out
    }

    /// Reads [`WorldPos::to_bytes`] output; re-canonicalizes defensively.
    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        let word = |i: usize| -> [u8; 4] { bytes[i * 4..i * 4 + 4].try_into().expect("4-byte slice") };
        let cell = IVec3::new(i32::from_le_bytes(word(0)), i32::from_le_bytes(word(1)), i32::from_le_bytes(word(2)));
        let local = Vec3::new(f32::from_le_bytes(word(3)), f32::from_le_bytes(word(4)), f32::from_le_bytes(word(5)));
        Self::new(cell, local)
    }
}

/// A placed object's full transform in a worldspace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldTransform {
    pub pos: WorldPos,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl WorldTransform {
    pub const IDENTITY: Self = Self { pos: WorldPos::ORIGIN, rotation: Quat::IDENTITY, scale: Vec3::ONE };

    pub fn from_pos(pos: WorldPos) -> Self {
        Self { pos, ..Self::IDENTITY }
    }
}

/// Splits one absolute coordinate (meters) into a cell index and a
/// canonical f32 offset in `[-128, 128)`.
// The f64 -> f32 and f64 -> i64 casts are the point: an f32 offset, an
// integer cell (range-checked by `to_i32`).
#[allow(clippy::cast_possible_truncation)]
fn split(x: f64) -> (i64, f32) {
    let mut cell = ((x + f64::from(HALF_CELL)) / CELL_F64).floor();
    let mut local = (x - cell * CELL_F64) as f32;
    // The f32 rounding can land exactly on the upper bound (or a hair below
    // the lower one); renormalize so the result stays canonical.
    if local >= HALF_CELL {
        cell += 1.0;
        local -= GRID_CELL;
    } else if local < -HALF_CELL {
        cell -= 1.0;
        local += GRID_CELL;
    }
    // Normalize -0.0 so equal positions are bit-identical.
    if local == 0.0 {
        local = 0.0;
    }
    (cell as i64, local)
}

fn to_i32(x: i64, y: i64, z: i64) -> IVec3 {
    let narrow = |v: i64| i32::try_from(v).expect("position outside the i32 grid (±5·10¹¹ m)");
    IVec3::new(narrow(x), narrow(y), narrow(z))
}

#[cfg(test)]
mod tests;
