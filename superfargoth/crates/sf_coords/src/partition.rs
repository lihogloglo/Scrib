//! Power-of-two partitions of the grid (streaming cells, terrain tiles,
//! HLOD clusters).
//!
//! Partition `shift` has edge `GRID_CELL · 2^shift`. Every partition shares
//! the grid's cell corners (at `256·k − 128` m), so a partition cell of one
//! shift is exactly a union of partition cells of any smaller shift, and
//! the math is integer shifts on the grid cell index.

use glam::{IVec3, Vec3};

use crate::{GRID_CELL, HALF_CELL, WorldPos};

/// Edge length in meters of partition `shift`.
pub fn partition_extent(shift: i32) -> f32 {
    GRID_CELL * 2f32.powi(shift)
}

pub(crate) fn index(pos: WorldPos, shift: i32) -> IVec3 {
    if shift >= 0 {
        // Arithmetic shift is floor division, also for negative cells.
        IVec3::new(pos.cell.x >> shift, pos.cell.y >> shift, pos.cell.z >> shift)
    } else {
        let sub_bits = shift.unsigned_abs();
        let per_cell = 1i32 << sub_bits;
        let edge = partition_extent(shift);
        let sub = |local: f32| -> i32 {
            #[allow(clippy::cast_possible_truncation)]
            let i = ((local + HALF_CELL) / edge).floor() as i32;
            i.clamp(0, per_cell - 1)
        };
        IVec3::new(
            (pos.cell.x << sub_bits) + sub(pos.local.x),
            (pos.cell.y << sub_bits) + sub(pos.local.y),
            (pos.cell.z << sub_bits) + sub(pos.local.z),
        )
    }
}

/// The minimum corner of partition cell `index` at `shift`.
pub fn partition_origin(index: IVec3, shift: i32) -> WorldPos {
    if shift >= 0 {
        let cell = IVec3::new(index.x << shift, index.y << shift, index.z << shift);
        WorldPos { cell, local: Vec3::splat(-HALF_CELL) }
    } else {
        let sub_bits = shift.unsigned_abs();
        let per_cell = 1i32 << sub_bits;
        let edge = partition_extent(shift);
        let cell = IVec3::new(index.x >> sub_bits, index.y >> sub_bits, index.z >> sub_bits);
        let rem = index - cell * per_cell;
        WorldPos::new(cell, Vec3::splat(-HALF_CELL) + rem.as_vec3() * edge)
    }
}
