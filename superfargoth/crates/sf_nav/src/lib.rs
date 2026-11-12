//! Navigation core, Bevy-free (`docs/specs/nav.md`): the types every navmesh consumer shares, the
//! [`NavBackend`] seam and a test double. Game-agnostic: the adapter names agent classes and areas.
//!
//! Built so far (NV1):
//! - [`types`]: agent classes ([`AgentClass`], [`ClassMask`]), the adapter's [`AreaTable`] of
//!   costs per class and [`PolyFlags`], [`TileKey`], off-mesh links, and the backend-opaque
//!   64-bit [`PolyRef`] / [`TileRef`] (never saved or sent).
//! - [`filter`]: [`QueryFilter`], Detour's include/exclude flags and per-area costs (UE's excluded
//!   areas as infinite cost).
//! - [`query`]: [`Goal`] (point, near, away, any of), [`PathQuery`], the sliced [`PathJob`] with
//!   its [`SliceBudget`] of expansions, [`PathResult`], [`NavRay`].
//! - [`backend`]: the [`NavBackend`] trait (bake a tile, tile residency, `nearest`, sliced
//!   `find_path`, `raycast`, polygon state patches) and [`NavError`]; NV3 adds the read-only
//!   [`TileSummary`] and [`TileGeometry`] accessors for stats and debug draw (`sf_nav_bevy`).
//! - [`baked`] (NV4a): the bake's products as the runtime reads them ([`BakedTile`] with its
//!   [`DoorPolys`], [`NavLinks`], [`CoarseGraph`]); the steps are `sf_nav_bake`. The trait's
//!   bake-side [`NavBackend::read_tile_blob`] decodes a blob into [`BlobTile`].
//! - [`grid`]: [`GridBackend`], an exact-A* 2.5D grid implementing [`NavBackend`] for tests.
//!
//! Later slices add to the trait: corridor and local boundary (NV7-NV8), `TileCache` obstacles
//! (NV10); the worldspace graph and `portal_query` (NV11), the coarse graph (NV12). The navmesh-rs
//! backend is `sf_nav_recast` (NV2).

pub mod backend;
pub mod baked;
pub mod filter;
pub mod grid;
pub mod query;
pub mod types;

pub use backend::{
    AreaVolume, BlobPoly, BlobTile, BuildConfig, DebugPoly, NavBackend, NavError, PolyEdge,
    TileGeometry, TileInput, TileSide, TileSummary,
};
pub use baked::{
    BakedTile, CoarseEdge, CoarseGraph, CoarseNode, CoarseNodeKind, DoorApproach, DoorPolys,
    NavLinks,
};
pub use filter::QueryFilter;
pub use grid::{GridBackend, GridCell, GridConfig};
pub use query::{
    DEFAULT_EXTENT, Goal, JobState, NavRay, PathJob, PathQuery, PathResult, PathStatus, SliceBudget,
};
pub use types::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, ClassMask, MAX_AREAS, MAX_CLASSES,
    NavDoorId, NavLinkId, NavPoint, OffMeshLink, PolyFlags, PolyRef, PolyState, TileKey, TileRef,
    TraverseKind,
};

#[cfg(test)]
mod tests;
