//! `WorldPos` ↔ `big_space` grid coordinates (world-coordinates.md).
//!
//! Every worldspace root is a `BigSpace` with one [`Grid`] of
//! [`GRID_CELL`] meters, and `big_space`'s convention matches `sf_coords`
//! exactly: a [`CellCoord`] plus a `Transform` translation measured from
//! the cell's center. So a canonical [`WorldPos`] maps 1:1, with no
//! arithmetic. `big_space` lets a moving entity's translation grow to the
//! switching threshold (`GRID_CELL / 2 + GRID_SWITCH`) before recentering,
//! so the way back renormalizes.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_math::{IVec3, Vec3};
use bevy_transform::components::Transform;
use big_space::prelude::{CellCoord, Grid};
use sf_coords::{GRID_CELL, WorldPos, WorldTransform};
use sf_world::{Location, WorldspaceId};

use crate::WorldspaceRoot;

/// Hysteresis before a moving entity changes grid cell (meters).
pub const GRID_SWITCH: f32 = 32.0;

/// The grid every worldspace root uses.
pub fn world_grid() -> Grid {
    Grid::new(GRID_CELL, GRID_SWITCH)
}

pub fn cell_coord(cell: IVec3) -> CellCoord {
    CellCoord::new(cell.x, cell.y, cell.z)
}

pub fn cell_ivec(cell: &CellCoord) -> IVec3 {
    IVec3::new(cell.x, cell.y, cell.z)
}

/// A position as grid coordinates (translation only).
pub fn to_grid(pos: WorldPos) -> (CellCoord, Transform) {
    (cell_coord(pos.cell), Transform::from_translation(pos.local))
}

/// A full transform as grid coordinates.
pub fn transform_to_grid(t: &WorldTransform) -> (CellCoord, Transform) {
    (cell_coord(t.pos.cell), Transform { translation: t.pos.local, rotation: t.rotation, scale: t.scale })
}

/// Grid coordinates back to a canonical position.
pub fn from_grid(cell: &CellCoord, transform: &Transform) -> WorldPos {
    WorldPos::new(cell_ivec(cell), transform.translation)
}

/// Where the floating origin's cell is: positions relative to it are what
/// `GlobalTransform` holds. `rel(p) = origin_rel(fo_cell, p)`.
pub fn origin_relative(origin_cell: IVec3, pos: WorldPos) -> Vec3 {
    WorldPos { cell: origin_cell, local: Vec3::ZERO }.delta_f32(pos)
}

/// Reads the [`WorldPos`] (and [`Location`]) of high-precision entities:
/// direct children of a worldspace root with a [`CellCoord`]. For their
/// low-precision descendants it composes the `Transform` chain up to the
/// nearest high-precision ancestor.
#[derive(SystemParam)]
pub struct WorldPosOf<'w, 's> {
    spatial: Query<'w, 's, (Option<&'static CellCoord>, &'static Transform, &'static ChildOf)>,
    roots: Query<'w, 's, &'static WorldspaceRoot>,
}

impl WorldPosOf<'_, '_> {
    /// The entity's position in its worldspace.
    pub fn get(&self, entity: Entity) -> Option<WorldPos> {
        self.location(entity).map(|l| l.pos)
    }

    /// The entity's worldspace and position.
    pub fn location(&self, entity: Entity) -> Option<Location> {
        let mut local = Transform::IDENTITY;
        let mut current = entity;
        // Bounded walk: low-precision chains are shallow.
        for _ in 0..64 {
            let (cell, transform, parent) = self.spatial.get(current).ok()?;
            let composed = transform.mul_transform(local);
            if let Some(cell) = cell {
                let root = self.roots.get(parent.parent()).ok()?;
                return Some(Location::new(root.id.clone(), from_grid(cell, &composed)));
            }
            local = composed;
            current = parent.parent();
        }
        None
    }

    /// The worldspace an entity lives in.
    pub fn worldspace(&self, entity: Entity) -> Option<WorldspaceId> {
        self.location(entity).map(|l| l.worldspace)
    }
}
