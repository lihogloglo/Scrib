//! Static scene core: the retained, GPU-driven static world's Bevy-free half.
//!
//! Spec: `superfargoth/docs/specs/static-scene.md` ("Crates": `sf_scene`).
//!
//! - [`table`]: the baked per-cell instance table ([`StaticCellTable`], §2), a versioned
//!   little-endian binary format (v2) with a borrowing reader ([`StaticCellTableView`]) whose instance
//!   section is the GPU layout (cell-local chain indices remapped at stream-in).
//! - [`eligibility`]: which placements live in the scene (§1), asked through an adapter trait
//!   ([`SceneEligibility`]), and the one-way per-cell [`Promotions`].
//! - [`alloc`]: the per-space GPU table's slot allocator (§3): one contiguous range per cell,
//!   free list, release deferred by the frames in flight.
//! - [`material`]: the material classes (§5) every scene material folds into.
//! - [`chain`]: how one model part's cooked LOD levels become one chain mesh ([`chain_layout`]),
//!   shared by the bake and the runtime.
//! - `bake` (feature `bake`): the `scene.cells` step writing one table per cell.
//! - [`cull`]: the CPU twin of the per-view pass (§4): layer mask, active flag, frustum, terrain
//!   horizon, sub-pixel cull and the LOD pick in the view's own projection ([`SceneProjection`]:
//!   the perspective pixel-error form, the orthographic cascade texel-density form), with the
//!   quality presets.
//!
//! Space: `sf_coords` engine space (right-handed, Y up, metres). Table contents are relative to
//! the cell origin, the centre of a `sf_coords` grid cell; [`cull`] works in any one f32 frame the
//! caller picks (camera-relative on the GPU), each cell placed by [`cell_offset`].

pub mod alloc;
#[cfg(feature = "bake")]
pub mod bake;
pub mod chain;
pub mod cull;
pub mod eligibility;
pub mod material;
pub mod table;

pub use alloc::{AllocError, SlotAllocator, SlotRange, SlotStats};
pub use chain::{ChainLayout, LayoutError, LevelSource, Segment, chain_layout};
pub use cull::{
    CullReason, CullStats, Frustum, HorizonGrid, HorizonParams, ScenePreset, SceneProjection, SceneView, Verdict,
    WorkItem, cull_cell, cull_instance, pick_level, pick_level_ortho, pixels_per_radian, texels_per_metre,
};
pub use eligibility::{ClassKind, Eligibility, EntityReason, MaterialKind, Promotions, SceneEligibility, classify};
pub use material::{MaterialClass, MaterialTraits};
pub use table::{
    Aabb, FORMAT_VERSION, InstanceFlags, LodChain, LodLevel, MAGIC, MAX_LOD_LEVELS, Sphere, StaticCellTable,
    StaticCellTableView, StaticInstance, TableError,
};

/// A placement's id, **cell-local**: with its table's cell it names one placement of the space
/// (`(cell, placement)` links the scene instance to records, physics and promotion).
pub type PlacementId = u32;

/// Where a cell's origin (the centre of grid cell `origin_cell`) sits in the cull frame whose
/// origin is `frame`: add it to cell-relative positions. Exact at any distance before the f32
/// narrowing, so an origin shift only changes this offset, never the table (§3 "Origin shifts").
pub fn cell_offset(origin_cell: glam::IVec3, frame: sf_coords::WorldPos) -> glam::Vec3 {
    frame.delta_f32(sf_coords::WorldPos::new(origin_cell, glam::Vec3::ZERO))
}
