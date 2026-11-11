//! Exact checks for patch GS "change detection" in the vendored `GpuBatchedMesh3d` port: a
//! batch's geometry ranges are resolved against the mesh allocator and the material bindings
//! only when something they depend on changed (static-scene perf audit 2026-11-11, fixing plan
//! item 2; UE cached mesh draw commands).
//!
//! `prepare_gpu_batched_mesh_draws` itself needs a `RenderDevice`, so these drive the two
//! GPU-free halves it is built on, exactly as it calls them each frame:
//! - `MeshAllocationChanges` (vendored `bevy_render`): the allocator's generation bumps on a
//!   round that freed, allocated or displaced a mesh, and names those meshes, and only then;
//! - `GpuBatchRangeCache` (vendored `bevy_pbr`): steady frames hit, a re-extracted batch
//!   misses alone, an allocator round drops exactly the batches drawing a moved mesh, missed
//!   rounds or a recreated allocator drop everything, a binding change drops only the batches
//!   whose materials now bind elsewhere, and the per-frame resolve counters say so.
//!
//! The debug-build cross-check in `prepare_gpu_batched_mesh_draws` (every cached batch is
//! re-resolved and asserted equal) covers the render-world wiring when the game runs.

use bevy::asset::uuid::Uuid;
use bevy::asset::{AssetId, UntypedAssetId};
use bevy::ecs::world::World;
use bevy::mesh::Mesh;
use bevy::pbr::gpu_instance_batch::{
    BatchRangeInputs, GpuBatchGeometryRange, GpuBatchRangeCache, GpuInstanceBatchStats, RangeResolveError,
};
use bevy::platform::collections::HashSet;
use bevy::render::material_bind_groups::{MaterialBindGroupIndex, MaterialBindGroupSlot, MaterialBindingId};
use bevy::render::mesh::allocator::MeshAllocationChanges;
use bevy::render::mesh::RenderMesh;
use bevy::render::render_asset::ExtractedAssets;
use bevy::render::sync_world::MainEntity;

fn mesh(id: u128) -> AssetId<Mesh> {
    AssetId::Uuid { uuid: Uuid::from_u128(id) }
}

fn material(id: u128) -> UntypedAssetId {
    mesh(1000 + id).untyped()
}

fn binding(group: u32, slot: u32) -> MaterialBindingId {
    MaterialBindingId { group: MaterialBindGroupIndex(group), slot: MaterialBindGroupSlot(slot) }
}

fn entities(n: usize) -> Vec<MainEntity> {
    let mut world = World::new();
    (0..n).map(|_| MainEntity::from(world.spawn_empty().id())).collect()
}

fn records(n: u32) -> Vec<GpuBatchGeometryRange> {
    (0..n).map(|i| GpuBatchGeometryRange { first_slot: i * 4, ..Default::default() }).collect()
}

/// One batch as `prepare_gpu_batched_mesh_draws` would describe it on a miss.
fn inputs(version: u64, meshes: &[u128], materials: &[(u128, MaterialBindingId)]) -> BatchRangeInputs {
    BatchRangeInputs {
        extract_version: version,
        batch_binding: binding(0, 0),
        meshes: meshes.iter().map(|&m| mesh(m)).collect(),
        material_bindings: materials.iter().map(|&(m, b)| (material(m), Some(b))).collect(),
    }
}

/// The frame start `prepare_gpu_batched_mesh_draws` runs: allocator `generation` / `moved`,
/// bindings unchanged unless `rebind` says otherwise, every batch alive.
fn begin(cache: &mut GpuBatchRangeCache, generation: u64, moved: &[u128], rebind: Option<(u128, MaterialBindingId)>) {
    let moved: HashSet<AssetId<Mesh>> = moved.iter().map(|&m| mesh(m)).collect();
    let binding_of = |m: &UntypedAssetId| match rebind {
        Some((id, b)) if *m == material(id) => Some(b),
        _ => Some(binding(0, 7)),
    };
    cache.begin_frame(generation, false, &moved, rebind.is_some(), binding_of, |_| true);
}

/// A cache holding two resolved batches at allocator generation 0: `a` draws meshes 1-2,
/// `b` draws mesh 3 with material 1 bound at (0, 7).
fn two_batches() -> (GpuBatchRangeCache, MainEntity, MainEntity) {
    let [a, b] = entities(2)[..] else { unreachable!() };
    let mut cache = GpuBatchRangeCache::default();
    begin(&mut cache, 0, &[], None);
    cache.insert(a, inputs(1, &[1, 2], &[]), 3, Ok(records(3)));
    cache.insert(b, inputs(2, &[3], &[(1, binding(0, 7))]), 5, Ok(records(5)));
    assert_eq!(cache.stats(), GpuInstanceBatchStats { extracted_batches: 0, resolved_batches: 2, resolved_ranges: 8 });
    (cache, a, b)
}

#[test]
fn allocator_generation_bumps_only_on_a_round_that_moves_a_mesh() {
    let mut changes = MeshAllocationChanges::default();
    assert_eq!(changes.generation(), 0);
    changes.record_round(&ExtractedAssets::<RenderMesh>::default(), []);
    assert_eq!(changes.generation(), 0, "a round that touches nothing is not a generation");

    let mut round = ExtractedAssets::<RenderMesh>::default();
    round.added.insert(mesh(1));
    round.removed.insert(mesh(2));
    changes.record_round(&round, []);
    assert_eq!(changes.generation(), 1);
    assert_eq!(changes.meshes(), &[mesh(1), mesh(2)].into_iter().collect::<HashSet<_>>());

    changes.record_round(&ExtractedAssets::<RenderMesh>::default(), []);
    assert_eq!(changes.generation(), 1, "steady frames keep the generation");
    assert!(changes.meshes().contains(&mesh(1)), "and the meshes of the round that produced it");

    changes.record_round(&ExtractedAssets::<RenderMesh>::default(), [mesh(9)]);
    assert_eq!(changes.generation(), 2, "slab growth (displacement) is a generation too");
    assert_eq!(changes.meshes(), &[mesh(9)].into_iter().collect::<HashSet<_>>());
}

#[test]
fn steady_frames_resolve_nothing_and_a_new_extraction_misses_alone() {
    let (mut cache, a, b) = two_batches();
    for _ in 0..3 {
        begin(&mut cache, 0, &[], None);
        assert_eq!(cache.stats().resolved_batches, 0);
        assert_eq!(cache.stats().resolved_ranges, 0);
        assert_eq!(cache.current(a, 1, binding(0, 0)), Some(&Ok(records(3))));
        assert_eq!(cache.current(b, 2, binding(0, 0)), Some(&Ok(records(5))));
    }

    begin(&mut cache, 0, &[], None);
    assert_eq!(cache.current(a, 3, binding(0, 0)), None, "a re-extracted batch (new version) misses");
    assert!(cache.current(b, 2, binding(0, 0)).is_some(), "the other batch still hits");
    cache.insert(a, inputs(3, &[1, 2], &[]), 4, Ok(records(4)));
    assert_eq!(cache.stats().resolved_batches, 1, "one batch re-resolved");
    assert_eq!(cache.stats().resolved_ranges, 4);
    assert_eq!(cache.current(a, 3, binding(0, 0)), Some(&Ok(records(4))));

    assert_eq!(cache.current(b, 2, binding(0, 1)), None, "the batch's own material moved: miss");
}

#[test]
fn allocator_rounds_drop_exactly_the_batches_they_moved() {
    let (mut cache, a, b) = two_batches();
    begin(&mut cache, 1, &[2, 40], None);
    assert_eq!(cache.current(a, 1, binding(0, 0)), None, "mesh 2 moved: batch a re-resolves");
    assert!(cache.current(b, 2, binding(0, 0)).is_some(), "batch b draws none of the moved meshes");

    begin(&mut cache, 1, &[3], None);
    assert!(cache.current(b, 2, binding(0, 0)).is_some(), "same generation: its meshes were already handled");

    begin(&mut cache, 3, &[99], None);
    assert!(cache.is_empty(), "generations 2-3 missed: every offset may have moved");

    let (mut cache, _, _) = two_batches();
    cache.begin_frame(0, true, &HashSet::new(), false, |_| None, |_| true);
    assert!(cache.is_empty(), "a recreated allocator drops everything");
}

#[test]
fn binding_changes_drop_only_batches_whose_materials_moved() {
    let (mut cache, a, b) = two_batches();
    begin(&mut cache, 0, &[], Some((1, binding(0, 7))));
    assert_eq!(cache.len(), 2, "bindings changed elsewhere: material 1 still binds at (0, 7)");

    begin(&mut cache, 0, &[], Some((1, binding(2, 7))));
    assert!(cache.current(a, 1, binding(0, 0)).is_some(), "batch a has no range materials");
    assert_eq!(cache.current(b, 2, binding(0, 0)), None, "material 1 moved: batch b re-resolves");

    let mut world_gone = GpuBatchRangeCache::default();
    world_gone.begin_frame(0, false, &HashSet::new(), false, |_| None, |_| true);
    world_gone.insert(a, inputs(1, &[1], &[]), 1, Ok(records(1)));
    world_gone.begin_frame(0, false, &HashSet::new(), false, |_| None, |e| *e != a);
    assert!(world_gone.is_empty(), "a despawned batch drops out");
}

#[test]
fn a_failed_resolution_is_cached_until_its_mesh_allocates() {
    let [a] = entities(1)[..] else { unreachable!() };
    let mut cache = GpuBatchRangeCache::default();
    begin(&mut cache, 0, &[], None);
    let failed = Err(RangeResolveError { range: 0, why: "has no allocated mesh data yet" });
    cache.insert(a, inputs(1, &[5], &[]), 2, failed.clone());
    begin(&mut cache, 0, &[], None);
    assert_eq!(cache.current(a, 1, binding(0, 0)), Some(&failed), "no retry on a steady frame");
    begin(&mut cache, 1, &[5], None);
    assert_eq!(cache.current(a, 1, binding(0, 0)), None, "mesh 5 allocated: retry");
}
