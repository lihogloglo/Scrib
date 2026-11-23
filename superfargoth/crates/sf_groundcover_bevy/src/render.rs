//! The batch-slot writer, the pattern of `sf_scene_bevy::upload` (`upload.rs:92-176`, `lib.rs:197-213`) for
//! groundcover's own batches: the main world queues byte runs in [`GroundcoverUploads`] (a new batch's whole-capacity
//! zero-fill, or a grown batch's new-tail zero-fill, first, then block runs; a grown batch's reservation is resized
//! with a copy of its old slots in `PrepareMeshes`, before this publish, in the same frame as its runs), [`extract_groundcover_uploads`] moves them into the render world, and
//! [`publish_groundcover`] (`GpuInstanceBatchSystems::Publish`, `RenderGraph` schedule) writes them into each
//! batch's output buffer with `RenderQueue::write_buffer`, in queue order, so for any slot the run queued last wins
//! (frees, moves and an origin-shift rewrite need no sort). A batch with no reservation yet (the frame it spawns)
//! keeps its runs, in order, for the next frame; runs for a despawned batch are dropped at extract.

use bevy_ecs::prelude::*;
use bevy_pbr::gpu_instance_batch::GpuInstanceBatchReservations;
use bevy_platform::collections::HashSet;
use bevy_render::MainWorld;
use bevy_render::renderer::RenderQueue;
use bevy_render::sync_world::MainEntity;

/// Bytes of one batch slot (`GpuMeshInstance`).
pub const SLOT_BYTES: u64 = 64;

/// One byte run into a batch's output buffer.
#[derive(Clone, Debug)]
pub struct GroundcoverWrite {
    /// The batch entity (`GpuBatchedMesh3d`).
    pub entity: Entity,
    pub first_slot: u32,
    /// Whole slots, 64 B each.
    pub bytes: Vec<u8>,
}

/// Main world: runs queued this frame.
#[derive(Resource, Default)]
pub struct GroundcoverUploads {
    pub writes: Vec<GroundcoverWrite>,
}

/// Render world: runs not yet published.
#[derive(Resource, Default)]
pub struct PendingGroundcoverWrites {
    pub writes: Vec<GroundcoverWrite>,
}

/// Render world, `ExtractSchedule`: takes the main world's runs and drops pending ones whose batch is gone.
pub fn extract_groundcover_uploads(mut main: ResMut<MainWorld>, mut pending: ResMut<PendingGroundcoverWrites>) {
    if let Some(mut uploads) = main.get_resource_mut::<GroundcoverUploads>() {
        let writes = std::mem::take(&mut uploads.writes);
        pending.writes.extend(writes);
    }
    if pending.writes.is_empty() {
        return;
    }
    pending.writes.retain(|w| main.get_entity(w.entity).is_ok());
}

/// Render world, `RenderGraph` schedule, `GpuInstanceBatchSystems::Publish`: every pending run whose batch has
/// its reservation, in order. A batch without one keeps all of its runs (and their order) for the next frame.
pub fn publish_groundcover(
    mut pending: ResMut<PendingGroundcoverWrites>,
    reservations: Res<GpuInstanceBatchReservations>,
    queue: Res<RenderQueue>,
) {
    if pending.writes.is_empty() {
        return;
    }
    let writes = std::mem::take(&mut pending.writes);
    let mut waiting: HashSet<Entity> = HashSet::default();
    for w in writes {
        let reservation = reservations.get(MainEntity::from(w.entity));
        let Some(reservation) = reservation.filter(|_| !waiting.contains(&w.entity)) else {
            waiting.insert(w.entity);
            pending.writes.push(w);
            continue;
        };
        let end = u64::from(w.first_slot) + w.bytes.len() as u64 / SLOT_BYTES;
        if end > u64::from(reservation.max_capacity()) {
            log::error!(
                "sf_groundcover_bevy: batch {:?} slots {}..{end} are past its capacity {}; not written",
                w.entity,
                w.first_slot,
                reservation.max_capacity()
            );
            continue;
        }
        queue.write_buffer(reservation.output_buffer(), u64::from(w.first_slot) * SLOT_BYTES, &w.bytes);
    }
}
