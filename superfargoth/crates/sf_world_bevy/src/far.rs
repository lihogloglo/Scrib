//! The far-field hook: what the far ring (R2) loads when a far field is
//! installed (far-field.md, "Residency"). Without one, R2 spawns large
//! placements at their coarsest LOD (streaming.md's original behavior).
//!
//! The hook keeps this crate free of the far field's own types: the far
//! field plugin (`sf_lod_bevy`) implements [`CellProxies`] and inserts
//! [`FarProxies`]; streaming calls it from its stages, so proxy loads share
//! the in-flight caps, spawn budget and reveal-at-once rule of every other
//! level step.

use std::sync::Arc;

use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use sf_coords::WorldPos;
use sf_world::WorldspaceId;

/// What [`CellProxies::spawn`] gets.
pub struct ProxySpawnCx<'a> {
    pub worldspace: &'a WorldspaceId,
    pub cell: IVec3,
    /// Spawn under this entity (hidden until the level completes). Its
    /// frame is the cell's origin corner.
    pub parent: Entity,
    pub cell_origin: WorldPos,
    pub layers: &'a RenderLayers,
}

/// A far field's per-cell proxies (implemented by the far-field plugin).
/// Calls for one cell come in order: `acquire`, then `ready` until true,
/// then `spawn`, and eventually `release` (also after a cancelled load).
pub trait CellProxies: Send + Sync + 'static {
    /// The load of a cell's far level reached its assets stage: start
    /// loading its proxy.
    fn acquire(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3);
    /// Whether the proxy is loaded (or the cell has none).
    fn ready(&self, world: &World, worldspace: &WorldspaceId, cell: IVec3) -> bool;
    /// Spawns the proxy's entities; returns how many.
    fn spawn(&self, world: &mut World, cx: &ProxySpawnCx<'_>) -> u32;
    /// The cell's far level is gone (despawned or cancelled).
    fn release(&self, world: &mut World, worldspace: &WorldspaceId, cell: IVec3);
    /// The smallest bounds radius of a far-eligible placement the cell's
    /// proxy represents (industry-audit.md J1, "L1-contributor clamp"):
    /// such placements stay drawn up to the handoff whatever their size,
    /// or a hole opens before the proxy fades in. Called when the cell's
    /// near level spawns, after its far level loaded the proxy. The
    /// default, 0, keeps every far-eligible placement (safe, no size culling
    /// for them).
    fn kept_radius(&self, _world: &World, _worldspace: &WorldspaceId, _cell: IVec3) -> f32 {
        0.0
    }
    /// Upload estimate of the cell's proxy, bytes.
    fn bytes(&self, _worldspace: &WorldspaceId, _cell: IVec3) -> u64 {
        0
    }
}

/// The installed far field, if any.
#[derive(Resource, Clone, Default)]
pub struct FarProxies {
    pub proxies: Option<Arc<dyn CellProxies>>,
    /// Where far-eligible placements hand over to the proxies: their LOD
    /// chain ends at `handoff` with a fade band of `band` meters.
    pub handoff: Option<(f32, f32)>,
}

impl FarProxies {
    pub fn is_installed(&self) -> bool {
        self.proxies.is_some()
    }
}
