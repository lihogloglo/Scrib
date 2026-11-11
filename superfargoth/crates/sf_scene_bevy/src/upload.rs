//! The batch-slot writer (B4a): what makes a committed cell's statics draw.
//!
//! [`SceneHook::upload`](crate::SceneHook) resolves each of the plan's
//! `SlotWrite`s to a [`BatchSlotWrite`] (batch entity, slot, a ready
//! [`GpuMeshInstance`]) and queues it in [`SceneUploads`] beside the new
//! batches' zero-fills. [`extract_scene_uploads`] moves both into the render
//! world's [`PendingSceneWrites`], and [`publish_scene`]
//! (`GpuInstanceBatchSystems::Publish`, `RenderGraph` schedule) writes them
//! into the batches' output buffers with `RenderQueue::write_buffer`: the
//! zero-fills first, then the slot writes coalesced into one write per run of
//! consecutive slots (B3/B4 brief §5.3, writes 2 and 3).
//!
//! A batch whose reservation does not exist yet (the frame it is spawned,
//! before `PrepareMeshes` sized it) keeps its work for the next frame; work for
//! a batch entity that is gone is dropped at extract.
//!
//! **Origin shifts** (plan §5.4): the outputs are render-space, placed by the
//! floating origin at upload; when the origin moves,
//! [`rewrite_on_origin_shift`](crate::shift::rewrite_on_origin_shift) queues
//! every resident cell's slots again in the new frame, in the shift frame, and
//! the stable run sort makes that later write win. The per-space scene buffer
//! (`CellPlan::scene_bytes`, write 1) is B4b.

use std::ops::Range;

use bevy_ecs::prelude::*;
use bevy_math::{Vec3, Vec4};
use bevy_pbr::gpu_instance_batch::{GpuInstanceBatchReservations, GpuMeshInstance};
use bevy_render::MainWorld;
use bevy_render::render_resource::encase::StorageBuffer;
use bevy_render::renderer::RenderQueue;
use bevy_render::sync_world::MainEntity;
use sf_scene::BatchId;

use crate::residency::{BatchZeroFill, SceneUploads};

/// Bytes of one batch output slot.
pub const SLOT_BYTES: u64 = SLOT_LEN as u64;
const SLOT_LEN: usize = size_of::<GpuMeshInstance>();

/// One batch output slot to write, resolved for the render world.
#[derive(Clone, Copy)]
pub struct BatchSlotWrite {
    /// The batch's entity: the key of `GpuInstanceBatchReservations`.
    pub entity: MainEntity,
    pub batch: BatchId,
    pub slot: u32,
    /// What the slot gets, `is_active` included.
    pub instance: GpuMeshInstance,
}

impl BatchSlotWrite {
    /// Whether the slot draws (`SlotWrite::active`).
    pub fn active(&self) -> bool {
        self.instance.is_active != 0
    }
}

/// A table row's cell-relative affine rows (`StaticInstance::transform`, already the affine
/// transpose `GpuMeshInstance` takes, B3/B4 brief §2.3) placed in render space: `offset` is the
/// cell origin in the render frame (`sf_scene::cell_offset`), added to the translation lanes.
pub fn gpu_instance(rows: [[f32; 4]; 3], offset: Vec3, active: bool) -> GpuMeshInstance {
    let row = |r: [f32; 4], t: f32| Vec4::new(r[0], r[1], r[2], r[3] + t);
    let [r0, r1, r2] = rows;
    GpuMeshInstance {
        world_from_local: [row(r0, offset.x), row(r1, offset.y), row(r2, offset.z)],
        is_active: u32::from(active),
        tag: 0,
        pad: [0; 2],
    }
}

/// The GPU bytes of `instances`, back to back (64 B each, `is_active` at 48).
pub fn instance_bytes(instances: &[GpuMeshInstance]) -> Vec<u8> {
    let mut buffer = StorageBuffer::new(Vec::with_capacity(instances.len() * SLOT_LEN));
    if let Err(e) = buffer.write(instances) {
        log::error!("sf_scene_bevy: encoding {} batch slots: {e}", instances.len());
    }
    buffer.into_inner()
}

/// A run of consecutive slots of one batch: `writes[range]` go to `first_slot..`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotRun {
    pub entity: MainEntity,
    pub first_slot: u32,
    pub writes: Range<usize>,
}

/// Sorts `writes` by (batch entity, slot), stably (a later write to one slot stays after the
/// earlier one, so it wins), and coalesces consecutive slots of one batch into runs.
pub fn slot_runs(writes: &mut [BatchSlotWrite]) -> Vec<SlotRun> {
    writes.sort_by_key(|w| (w.entity, w.slot));
    let mut runs: Vec<SlotRun> = Vec::new();
    for (i, w) in writes.iter().enumerate() {
        match runs.last_mut() {
            Some(run) if run.entity == w.entity && u64::from(run.first_slot) + run.writes.len() as u64 == u64::from(w.slot) => {
                run.writes.end = i + 1;
            }
            _ => runs.push(SlotRun { entity: w.entity, first_slot: w.slot, writes: i..i + 1 }),
        }
    }
    runs
}

/// Render world: the scene's batch writes not yet published (a batch with no reservation yet
/// keeps its work here).
#[derive(Resource, Default)]
pub struct PendingSceneWrites {
    pub zero_fills: Vec<BatchZeroFill>,
    pub writes: Vec<BatchSlotWrite>,
}

/// Render world, `ExtractSchedule`: moves the main world's queued zero-fills and slot writes into
/// [`PendingSceneWrites`], and drops pending work whose batch entity was despawned. `cells` (the
/// scene buffer) stays for B4b.
pub fn extract_scene_uploads(mut main: ResMut<MainWorld>, mut pending: ResMut<PendingSceneWrites>) {
    if let Some(mut uploads) = main.get_resource_mut::<SceneUploads>() {
        let zero_fills = std::mem::take(&mut uploads.zero_fills);
        let writes = std::mem::take(&mut uploads.writes);
        pending.zero_fills.extend(zero_fills);
        pending.writes.extend(writes);
    }
    if pending.zero_fills.is_empty() && pending.writes.is_empty() {
        return;
    }
    let live = |e: Entity| main.get_entity(e).is_ok();
    pending.zero_fills.retain(|z| live(z.entity));
    pending.writes.retain(|w| live(w.entity.id()));
}

/// Render world, `RenderGraph` schedule, `GpuInstanceBatchSystems::Publish`: the zero-fills,
/// then the coalesced slot writes, into each batch's output buffer.
pub fn publish_scene(
    mut pending: ResMut<PendingSceneWrites>,
    reservations: Res<GpuInstanceBatchReservations>,
    queue: Res<RenderQueue>,
    mut zeros: Local<Vec<u8>>,
) {
    let pending = &mut *pending;
    pending.zero_fills.retain(|z| {
        let Some(reservation) = reservations.get(MainEntity::from(z.entity)) else { return true };
        let len = z.slots.min(reservation.max_capacity()) as usize * SLOT_LEN;
        if zeros.len() < len {
            zeros.resize(len, 0);
        }
        queue.write_buffer(reservation.output_buffer(), 0, &zeros[..len]);
        false
    });
    if pending.writes.is_empty() {
        return;
    }
    let mut writes = std::mem::take(&mut pending.writes);
    let mut instances = Vec::new();
    for run in slot_runs(&mut writes) {
        let run_writes = &writes[run.writes.clone()];
        // Kept while its reservation is missing or its zero-fill still waits (the fill must land first).
        let waiting = pending.zero_fills.iter().any(|z| MainEntity::from(z.entity) == run.entity);
        let reservation = reservations.get(run.entity);
        let Some(reservation) = reservation.filter(|_| !waiting) else {
            pending.writes.extend_from_slice(run_writes);
            continue;
        };
        let end = u64::from(run.first_slot) + run_writes.len() as u64;
        if end > u64::from(reservation.max_capacity()) {
            log::error!(
                "sf_scene_bevy: batch {} slots {}..{end} are past its capacity {}; not written",
                run_writes[0].batch,
                run.first_slot,
                reservation.max_capacity()
            );
            continue;
        }
        instances.clear();
        instances.extend(run_writes.iter().map(|w| w.instance));
        queue.write_buffer(reservation.output_buffer(), u64::from(run.first_slot) * SLOT_BYTES, &instance_bytes(&instances));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn le(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    /// Test 1 (B4a brief): a known row and cell origin give the exact slot bytes, the affine
    /// rows with the origin in the translation lanes and `is_active` at offset 48.
    #[test]
    fn a_table_row_becomes_the_exact_slot_bytes() {
        let rows = [[1.0, 0.0, 0.0, 10.0], [0.0, 2.0, 0.0, 20.0], [0.0, 0.0, 3.0, 30.0]];
        let offset = Vec3::new(100.0, -200.0, 0.5);
        let bytes = instance_bytes(&[gpu_instance(rows, offset, true)]);
        let f = f32::to_bits;
        #[rustfmt::skip]
        let expected = le(&[
            f(1.0), f(0.0), f(0.0), f(110.0),
            f(0.0), f(2.0), f(0.0), f(-180.0),
            f(0.0), f(0.0), f(3.0), f(30.5),
            1, 0, 0, 0,
        ]);
        assert_eq!(bytes, expected);
        assert_eq!(SLOT_BYTES, 64);
        assert_eq!(&bytes[48..52], &1u32.to_le_bytes(), "is_active at offset 48");
        let inactive = instance_bytes(&[gpu_instance(rows, offset, false)]);
        assert_eq!(&inactive[48..52], &0u32.to_le_bytes());
        assert_eq!(inactive[..48], bytes[..48]);
    }

    /// Test 2: slots `[3, 4, 5, 9]` of one batch and `[4]` of another, interleaved as table
    /// order may give them, are three runs.
    #[test]
    fn consecutive_slots_of_one_batch_coalesce() {
        let a = MainEntity::from(Entity::from_raw_u32(7).unwrap());
        let b = MainEntity::from(Entity::from_raw_u32(8).unwrap());
        let w = |entity, slot| BatchSlotWrite {
            entity,
            batch: 0,
            slot,
            instance: gpu_instance([[0.0; 4]; 3], Vec3::ZERO, true),
        };
        let mut writes = vec![w(a, 3), w(b, 4), w(a, 4), w(a, 9), w(a, 5)];
        let runs = slot_runs(&mut writes);
        // `Entity`'s order is not its index's, so compare as (entity, first slot, length).
        let mut got: Vec<(bool, u32, usize)> = runs.iter().map(|r| (r.entity == a, r.first_slot, r.writes.len())).collect();
        got.sort_unstable();
        assert_eq!(got, [(false, 4, 1), (true, 3, 3), (true, 9, 1)]);
        for run in &runs {
            let slots: Vec<u32> = writes[run.writes.clone()].iter().map(|w| w.slot).collect();
            let expected: Vec<u32> = (run.first_slot..).take(slots.len()).collect();
            assert_eq!(slots, expected);
            assert!(writes[run.writes.clone()].iter().all(|w| w.entity == run.entity));
        }
    }
}
