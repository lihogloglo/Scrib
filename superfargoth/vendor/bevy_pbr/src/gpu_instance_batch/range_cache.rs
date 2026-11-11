// superfargoth patch (own, GS: change detection): new file.
//! Per-batch cache of resolved geometry ranges, so that
//! `prepare_gpu_batched_mesh_draws` resolves a batch's ranges against the
//! `MeshAllocator` and `RenderMaterialBindings` only when something they depend
//! on changed, not every frame (UE's cached `FMeshDrawCommand`s, Unity's
//! persistent BRG draw commands: rebuilt on change).
//!
//! **What is cached**, per batch: its [`GpuBatchGeometryRange`] records (or the
//! reason a range could not be resolved). The records hold slab offsets, the
//! metadata index, material slots and model-space bounds; they do **not** hold
//! the batch's input base (`input_range`), which still moves every frame with
//! the number of ordinary mesh inputs and is applied per batch, not per range.
//!
//! **What invalidates an entry** (checked in [`GpuBatchRangeCache::begin_frame`]
//! for every entry, and in [`GpuBatchRangeCache::current`] for the one asked):
//! - the batch was re-extracted (its `extract_version` changed: new ranges,
//!   mesh, capacity, or an asset event on one of its meshes);
//! - its own material's binding changed (`batch_binding`);
//! - the allocator moved one of its meshes (a [`MeshAllocationChanges`] round
//!   naming it), or the cache missed allocator rounds, or the allocator was
//!   recreated: then every entry drops;
//! - `RenderMaterialBindings` changed and one of its range materials now binds
//!   elsewhere, or appeared, or went away;
//! - the batch is gone.
//!
//! A failed resolution is cached too: it is retried on the same triggers, which
//! are exactly the ones that can make it succeed (a mesh allocated, a material
//! prepared, the ranges republished).
//!
//! [`MeshAllocationChanges`]: bevy_render::mesh::allocator::MeshAllocationChanges

use bevy_asset::{AssetId, UntypedAssetId};
use bevy_ecs::resource::Resource;
use bevy_mesh::Mesh;
use bevy_platform::collections::HashSet;
use bevy_render::{
    material_bind_groups::MaterialBindingId,
    sync_world::{MainEntity, MainEntityHashMap},
};

use super::GpuBatchGeometryRange;

/// Why a batch's range table could not be resolved: the range's index and a
/// reason for the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeResolveError {
    pub range: usize,
    pub why: &'static str,
}

/// A batch's resolved range table, or why it does not draw.
pub type ResolvedRanges = Result<Vec<GpuBatchGeometryRange>, RangeResolveError>;

/// Everything a batch's resolved ranges were computed from, apart from the
/// allocator's offsets (tracked by generation instead).
#[derive(Clone, Debug)]
pub struct BatchRangeInputs {
    /// The extraction this resolution read (`ExtractedGpuBatchedMesh::version`).
    pub extract_version: u64,
    /// The binding of the batch's own `MeshMaterial3d`.
    pub batch_binding: MaterialBindingId,
    /// Every mesh the batch draws from: its own and each range's.
    pub meshes: HashSet<AssetId<Mesh>>,
    /// Each distinct range material and the binding it had when resolved.
    pub material_bindings: Vec<(UntypedAssetId, Option<MaterialBindingId>)>,
}

/// Counters for the port's per-frame work, for tests and diagnostics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuInstanceBatchStats {
    /// Batches re-extracted from the main world in the last `ExtractSchedule`.
    pub extracted_batches: u32,
    /// Batches whose ranges were resolved this frame (cache misses).
    pub resolved_batches: u32,
    /// Ranges resolved this frame, summed over `resolved_batches`.
    pub resolved_ranges: u32,
}

struct CachedBatch {
    inputs: BatchRangeInputs,
    resolved: ResolvedRanges,
}

fn same_binding(a: Option<MaterialBindingId>, b: Option<MaterialBindingId>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a.group == b.group && a.slot == b.slot,
        (None, None) => true,
        _ => false,
    }
}

/// The render world's cache of resolved batch ranges. See the module docs.
#[derive(Resource, Default)]
pub struct GpuBatchRangeCache {
    batches: MainEntityHashMap<CachedBatch>,
    seen_allocator_generation: Option<u64>,
    stats: GpuInstanceBatchStats,
}

impl GpuBatchRangeCache {
    /// Starts a frame: drops the entries of batches for which `live` is false
    /// and every entry this frame's allocator and binding changes invalidate,
    /// then resets the per-frame resolve counters.
    ///
    /// Call it **every frame**, before any batch is skipped for an unrelated
    /// reason: an allocator round or a binding change it does not see is lost
    /// for the entries it should have dropped.
    ///
    /// `allocator_generation` and `allocator_changed` are
    /// `MeshAllocator::changes()`; `allocator_recreated` is the allocator
    /// resource being (re)inserted. `bindings_changed` is
    /// `RenderMaterialBindings` changing since the last call, and `binding_of`
    /// its lookup.
    pub fn begin_frame(
        &mut self,
        allocator_generation: u64,
        allocator_recreated: bool,
        allocator_changed: &HashSet<AssetId<Mesh>>,
        bindings_changed: bool,
        binding_of: impl Fn(&UntypedAssetId) -> Option<MaterialBindingId>,
        live: impl Fn(&MainEntity) -> bool,
    ) {
        self.stats.resolved_batches = 0;
        self.stats.resolved_ranges = 0;
        let seen = self.seen_allocator_generation.replace(allocator_generation);
        let moved_meshes = match seen {
            Some(seen) if !allocator_recreated && seen == allocator_generation => None,
            Some(seen) if !allocator_recreated && seen.wrapping_add(1) == allocator_generation => {
                Some(allocator_changed)
            }
            // Missed rounds, a recreated allocator, or the first frame (no
            // entries yet): every offset may have moved.
            _ => {
                self.batches.clear();
                return;
            }
        };
        self.batches.retain(|entity, cached| {
            if !live(entity) {
                return false;
            }
            if let Some(moved) = moved_meshes
                && moved.iter().any(|mesh| cached.inputs.meshes.contains(mesh))
            {
                return false;
            }
            !bindings_changed
                || cached
                    .inputs
                    .material_bindings
                    .iter()
                    .all(|(material, binding)| same_binding(binding_of(material), *binding))
        });
    }

    /// The cached resolution of `entity`'s ranges, if it is still valid for
    /// this extraction and this batch material binding.
    pub fn current(
        &self,
        entity: MainEntity,
        extract_version: u64,
        batch_binding: MaterialBindingId,
    ) -> Option<&ResolvedRanges> {
        let cached = self.batches.get(&entity)?;
        (cached.inputs.extract_version == extract_version
            && same_binding(Some(cached.inputs.batch_binding), Some(batch_binding)))
        .then_some(&cached.resolved)
    }

    /// Stores a fresh resolution of `range_count` ranges and counts it in
    /// [`stats`](Self::stats).
    pub fn insert(
        &mut self,
        entity: MainEntity,
        inputs: BatchRangeInputs,
        range_count: usize,
        resolved: ResolvedRanges,
    ) -> &ResolvedRanges {
        self.stats.resolved_batches += 1;
        self.stats.resolved_ranges += range_count as u32;
        self.batches
            .insert(entity, CachedBatch { inputs, resolved });
        &self
            .batches
            .get(&entity)
            .expect("the entry was just inserted")
            .resolved
    }

    /// Drops `entity`'s entry, so its next frame resolves afresh.
    pub fn remove(&mut self, entity: MainEntity) {
        self.batches.remove(&entity);
    }

    /// Number of cached batches.
    pub fn len(&self) -> usize {
        self.batches.len()
    }

    /// Whether no batch is cached.
    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }

    /// This frame's counters.
    pub fn stats(&self) -> GpuInstanceBatchStats {
        self.stats
    }

    pub(crate) fn set_extracted_batches(&mut self, count: u32) {
        self.stats.extracted_batches = count;
    }
}
