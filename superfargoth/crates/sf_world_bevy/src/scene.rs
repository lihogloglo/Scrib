//! The static-scene hook: what a cell's retained GPU instance table draws, so
//! streaming neither loads those placements' models nor spawns entities for
//! them (static-scene.md, "Streaming").
//!
//! The hook keeps this crate free of the static scene's own types: the scene
//! plugin (`sf_scene_bevy`) implements [`CellScene`] and inserts [`SceneCells`];
//! streaming calls it from its stages, so a cell's table load shares the
//! in-flight caps, the byte budget and the reveal-at-once rule of every other
//! level step. The dependency goes one way: `sf_world_bevy` must never depend on
//! `sf_scene_bevy`.
//!
//! **Interlock the game side must honour.** A scene cell and a near precombine
//! would draw the same placements twice, and `spawn_precombine` builds its
//! buckets' LOD ends from `Work::members` — which the scene's `covered` set
//! joins. So an app that installs a [`CellScene`] must also clear
//! `StreamingConfig::precombine` (`streaming.precombine &= !scene`, before
//! `add_plugins`): that is the one place the two are mutually exclusive, and
//! without it the precombine would take bucket ends from placements that are
//! not its own.
//!
//! Origin shifts need no hook method: the scene plugin watches the public
//! [`crate::WorldOrigin`] resource, whose `cell` changes exactly on a shift.

use std::collections::HashSet;
use std::sync::Arc;

use bevy_asset::UntypedHandle;
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use sf_coords::WorldPos;
use sf_world::{Level, PlacementId, WorldspaceId};

/// What [`CellScene::upload`] gets.
pub struct SceneCellCx<'a> {
    pub worldspace: &'a WorldspaceId,
    pub cell: IVec3,
    /// Centre of the cell's grid cell: the frame its table's values are
    /// relative to (`sf_scene::cell_offset`).
    pub cell_origin: WorldPos,
    /// The level this load is reaching (always [`Level::Near`] while only the
    /// near ring has a scene).
    pub level: Level,
}

/// A static scene's per-cell residency (implemented by the scene plugin).
///
/// Calls for one cell come in order: [`acquire`](Self::acquire), then the
/// cell's *fetch* waits for the handle it returned **and** for
/// [`ready`](Self::ready) (polled until true), then [`covered`](Self::covered)
/// once, then `ready` again in the assets stage (it re-confirms), then
/// [`upload`](Self::upload) once, and eventually [`release`](Self::release) —
/// also after a cancelled load.
///
/// `covered` feeds the fetch's model gather, so it must be the scene's final
/// answer: the fetch waiting on `ready` lets the hook resolve the table, the
/// chain geometry it names and (in the scene plugin) plan the cell's slots
/// first, so overflow and a failed chain are *excluded* from `covered` and
/// their models are gathered like any other placement's ("plan before
/// `covered`", scene-runtime-plan.md §5.3, ruled 2026-11-11: asset-gated stage
/// completion, the shape the near precombine already uses here).
pub trait CellScene: Send + Sync + 'static {
    /// A cell's near load started its fetch: start loading its table. The
    /// returned handle, if any, holds the fetch open until it is loaded or
    /// failed (and `ready` holds it until the hook is done with the cell).
    /// `None`: this cell (or this worldspace) has no scene.
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle>;
    /// Whether the hook has made its decision for the cell: everything the
    /// table names resolved and the cell planned, or the cell fell back to the
    /// entity path, or it has no scene. Read-only; polled every frame from the
    /// fetch until true.
    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool;
    /// The placements the table draws: their models are not loaded and they do
    /// not spawn. They are still *activated*, so colliders, scripts and AI are
    /// untouched (the activate stage is a separate request with its own work).
    ///
    /// `indexed` is the cell's placement ids in *index* order (the world
    /// index page's order, before any overlay): a baked table names its
    /// placements by that cell-local index, so this is how its ids become
    /// [`PlacementId`]s.
    ///
    /// The `Arc` is shared with `Work::members`, so the common case — a scene
    /// and no precombine — clones no set.
    fn covered(
        &self,
        world: &World,
        worldspace: &WorldspaceId,
        cell: IVec3,
        indexed: &[PlacementId],
    ) -> Option<Arc<HashSet<PlacementId>>>;
    /// Queues this cell's instance and chain-slot writes, before its level
    /// group is revealed; returns the bytes queued, for the counters.
    ///
    /// **One invocation per load, and it must complete** (owner 2026-11-10):
    /// the upload stage item is not resumable (`Streaming::reveal` removes the
    /// request's work), and a whole cell's table is one buffer write — an
    /// instance record is 80 bytes, so even a 2 000-instance cell is ~160 KiB.
    /// The scheduler's byte budget throttles how many *cells* upload per frame,
    /// through [`bytes`](Self::bytes). A cell big enough to need splitting is a
    /// measurement away, and resumable upload is its own slice.
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> u64;
    /// The cell's scene residency is gone (despawned, unloaded or cancelled).
    ///
    /// **Must be idempotent**: a no-op when the cell holds no scene residency.
    /// Streaming calls it from the near unload, from the unload sweep that
    /// tears down every remaining level at once, and from a cancelled load,
    /// and never proves the three are disjoint.
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3);
    /// Upload estimate of the cell's table, bytes, added to the near load's
    /// budget. The default, 0, lets a hook skip the estimate (its uploads then
    /// ride on the placements' own estimate).
    fn bytes(&self, _worldspace: &WorldspaceId, _cell: IVec3) -> u64 {
        0
    }
}

/// The installed static scene, if any.
#[derive(Resource, Clone, Default)]
pub struct SceneCells(pub Option<Arc<dyn CellScene>>);

impl SceneCells {
    pub fn is_installed(&self) -> bool {
        self.0.is_some()
    }
}
