//! The bench's CPU slot writer, a local copy of the groundcover pattern (`sf_groundcover_bevy/src/render.rs`): the
//! main world queues byte runs in [`CeilingUploads`], [`extract_ceiling_uploads`] moves them into the render world,
//! and [`publish_ceiling`] (`GpuInstanceBatchSystems::Publish`, `RenderGraph` schedule) writes them into each
//! batch's output buffer with `RenderQueue::write_buffer`, in queue order, in chunks of at most [`CHUNK_BYTES`]. A
//! batch with no reservation yet (the frame it spawns) keeps its runs for the next frame; runs for a despawned batch
//! are dropped at extract.

use bevy::pbr::gpu_instance_batch::{GpuInstanceBatchReservations, GpuInstanceBatchSystems};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::render::renderer::{RenderGraph, RenderQueue};
use bevy::render::sync_world::MainEntity;
use bevy::render::{ExtractSchedule, MainWorld, RenderApp};

use crate::fill::SLOT_BYTES;

/// Fixture: the largest single `write_buffer`, bytes (16 MB = 262,144 slots).
pub const CHUNK_BYTES: usize = 16 << 20;

/// One byte run into a batch's output buffer.
#[derive(Clone, Debug)]
pub struct CeilingWrite {
    /// The batch entity (`GpuBatchedMesh3d`).
    pub entity: Entity,
    pub first_slot: u32,
    /// Whole slots, 64 B each.
    pub bytes: Vec<u8>,
}

/// Main world: runs queued this frame.
#[derive(Resource, Default)]
pub struct CeilingUploads {
    pub writes: Vec<CeilingWrite>,
}

/// Render world: runs not yet published.
#[derive(Resource, Default)]
struct PendingCeilingWrites {
    writes: Vec<CeilingWrite>,
}

/// The writer: [`CeilingUploads`] in the main world, extract and publish in the render world. Needs
/// `GpuInstanceBatchPlugin`.
pub struct CeilingUploadPlugin;

impl Plugin for CeilingUploadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CeilingUploads>();
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .init_resource::<PendingCeilingWrites>()
            .add_systems(ExtractSchedule, extract_ceiling_uploads)
            .add_systems(
                RenderGraph,
                publish_ceiling
                    .in_set(GpuInstanceBatchSystems::Publish)
                    .run_if(resource_exists::<GpuInstanceBatchReservations>),
            );
    }
}

/// Render world, `ExtractSchedule`: takes the main world's runs and drops pending ones whose batch is gone.
fn extract_ceiling_uploads(mut main: ResMut<MainWorld>, mut pending: ResMut<PendingCeilingWrites>) {
    if let Some(mut uploads) = main.get_resource_mut::<CeilingUploads>() {
        let writes = std::mem::take(&mut uploads.writes);
        pending.writes.extend(writes);
    }
    if pending.writes.is_empty() {
        return;
    }
    pending.writes.retain(|w| main.get_entity(w.entity).is_ok());
}

/// Render world, `RenderGraph` schedule, `GpuInstanceBatchSystems::Publish`: every pending run whose batch has its
/// reservation, in order, split into chunks of at most [`CHUNK_BYTES`]. A batch without one keeps all of its runs
/// (and their order) for the next frame.
fn publish_ceiling(
    mut pending: ResMut<PendingCeilingWrites>,
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
        let slots = (w.bytes.len() / SLOT_BYTES) as u64;
        let end = u64::from(w.first_slot) + slots;
        if end > u64::from(reservation.max_capacity()) {
            error!(
                "forest_ceiling: batch {:?} slots {}..{end} are past its capacity {}; not written",
                w.entity,
                w.first_slot,
                reservation.max_capacity()
            );
            continue;
        }
        let start = u64::from(w.first_slot) * SLOT_BYTES as u64;
        for (i, chunk) in w.bytes.chunks(CHUNK_BYTES).enumerate() {
            queue.write_buffer(reservation.output_buffer(), start + (i * CHUNK_BYTES) as u64, chunk);
        }
    }
}
