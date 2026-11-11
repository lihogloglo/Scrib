//! The backend seam (nav.md "Interfaces"): what a navmesh implementation provides. The core, the
//! bake step and the query service use only this trait, so `sf_nav_recast` (navmesh-rs, NV2) and
//! the [`GridBackend`](crate::GridBackend) test double are interchangeable.

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_world::{Location, WorldspaceId};
use thiserror::Error;

use crate::{
    AgentClass, AgentClassId, AreaId, AreaTable, JobState, NavPoint, NavRay, PathJob, PathQuery,
    PathResult, PolyRef, PolyState, QueryFilter, SliceBudget, TileKey, TileRef,
};

#[derive(Clone, Debug, PartialEq, Error)]
pub enum NavError {
    #[error("invalid navigation config: {0}")]
    InvalidConfig(String),
    #[error("invalid tile {key:?}: {reason}")]
    InvalidTile { key: Box<TileKey>, reason: String },
    #[error("tile {0:?} is already loaded")]
    TileLoaded(Box<TileKey>),
    #[error("the start is not on the navmesh")]
    StartOffMesh,
    #[error("the goal is not on the navmesh")]
    GoalOffMesh,
    #[error("an empty goal")]
    EmptyGoal,
    #[error("query in {query} has a goal in {goal}: cross spaces through the worldspace graph")]
    WrongWorldspace {
        query: WorldspaceId,
        goal: WorldspaceId,
    },
    #[error("polygon {0:?} is not resident")]
    UnknownPoly(PolyRef),
    #[error("a tile the search used was removed: restart it")]
    MeshChanged,
    #[error("the job already finished")]
    JobFinished,
    #[error("unsupported by this backend: {0}")]
    Unsupported(&'static str),
    /// The backend's own failure (a tile bake stage, an exhausted search pool), as text: backend
    /// types never cross the seam.
    #[error("navmesh backend: {0}")]
    Backend(String),
}

/// A convex prism that overrides the area of the polygons inside it (Recast
/// `rcMarkConvexPolyArea`): doorway footprints, hazards.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaVolume {
    /// Footprint corners on the XZ plane (x, z), convex, either winding.
    pub footprint: Vec<Vec2>,
    pub min_y: f32,
    pub max_y: f32,
    pub area: AreaId,
}

impl AreaVolume {
    /// Whether `point` lies inside the prism (boundary included).
    pub fn contains(&self, point: Vec3) -> bool {
        if point.y < self.min_y || point.y > self.max_y || self.footprint.len() < 3 {
            return false;
        }
        let at = Vec2::new(point.x, point.z);
        let n = self.footprint.len();
        let (mut pos, mut neg) = (false, false);
        for i in 0..n {
            let (a, b) = (self.footprint[i], self.footprint[(i + 1) % n]);
            let side = (b - a).perp_dot(at - a);
            pos |= side > 0.0;
            neg |= side < 0.0;
        }
        !(pos && neg)
    }
}

/// The adapter's geometry for one tile (region-local meters, Y up): the same collision the mover
/// collides with, triangles with their areas, plus area volumes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TileInput {
    pub tile_x: i32,
    pub tile_z: i32,
    pub verts: Vec<Vec3>,
    pub tris: Vec<[u32; 3]>,
    /// One per triangle; [`AreaId::NULL`] for collision that is never walkable.
    pub tri_areas: Vec<AreaId>,
    pub volumes: Vec<AreaVolume>,
}

/// Bake settings for one navmesh (nav.md "Tiles": humanoid cs 0.15 m, ch 0.1 m, 128 cells).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub class: AgentClass,
    /// Voxel size on XZ, meters.
    pub cell_size: f32,
    /// Voxel size on Y, meters.
    pub cell_height: f32,
    /// Cells per tile side; a tile spans `tile_cells × cell_size` meters.
    pub tile_cells: u16,
}

impl BuildConfig {
    pub fn tile_size(&self) -> f32 {
        f32::from(self.tile_cells) * self.cell_size
    }

    pub fn validate(&self) -> Result<(), NavError> {
        if !(self.cell_size > 0.0 && self.cell_height > 0.0 && self.tile_cells > 0) {
            return Err(NavError::InvalidConfig(format!(
                "cell size {} m, cell height {} m, {} cells per tile",
                self.cell_size, self.cell_height, self.tile_cells
            )));
        }
        Ok(())
    }
}

/// A navmesh backend (Detour `dtNavMesh` + `dtNavMeshQuery`). Thread-safe for queries: every
/// query takes `&self`; tile changes take `&mut self` and apply between ticks.
///
/// Positions are [`Location`]s; a backend converts to its region-local frame. A sliced
/// [`PathJob`] survives tile additions between steps; a removed tile it touched fails it with
/// [`NavError::MeshChanged`]. Polygon state patches never fail a job.
pub trait NavBackend: Send + Sync {
    /// Bakes one tile from geometry (bake time only). Polygon flags come from `areas`.
    fn build_tile(
        &self,
        input: &TileInput,
        cfg: &BuildConfig,
        areas: &AreaTable,
    ) -> Result<Vec<u8>, NavError>;

    /// Makes a baked tile resident.
    fn add_tile(&mut self, key: TileKey, blob: &[u8]) -> Result<TileRef, NavError>;

    /// Drops a resident tile; false when it wasn't loaded. Its [`PolyRef`]s become invalid.
    fn remove_tile(&mut self, key: &TileKey) -> bool;

    fn tile(&self, key: &TileKey) -> Option<TileRef>;

    /// The passing polygon nearest `at` within `extent` (half extents), and the nearest point on
    /// it (Detour `findNearestPoly`).
    fn nearest(
        &self,
        at: &Location,
        class: AgentClassId,
        extent: Vec3,
        filter: &QueryFilter,
    ) -> Option<NavPoint>;

    /// Runs a sliced A* until it finishes or `budget` runs out (Detour `updateSlicedFindPath`).
    fn find_path(&self, job: &mut PathJob, budget: &mut SliceBudget) -> JobState;

    /// Walks the mesh surface in a straight line from `from` towards `to` (XZ), stopping at the
    /// first polygon the filter rejects or the mesh doesn't continue into.
    fn raycast(
        &self,
        from: &NavPoint,
        to: &Location,
        filter: &QueryFilter,
    ) -> Result<NavRay, NavError>;

    fn poly_state(&self, poly: PolyRef) -> Option<PolyState>;

    /// Sets polygons' area and flags without a rebuild (doors: Detour `setPolyArea` and
    /// `setPolyFlags`).
    fn set_poly_state(&mut self, polys: &[PolyRef], state: PolyState) -> Result<(), NavError>;

    /// The whole search at once (tests, tools): [`NavBackend::find_path`] with no budget.
    fn find_path_now(&self, query: PathQuery) -> Result<PathResult, NavError> {
        let mut job = PathJob::new(query);
        loop {
            match self.find_path(&mut job, &mut SliceBudget::unlimited()) {
                JobState::InProgress => {}
                JobState::Done(r) => return Ok(r),
                JobState::Failed(e) => return Err(e),
            }
        }
    }
}
