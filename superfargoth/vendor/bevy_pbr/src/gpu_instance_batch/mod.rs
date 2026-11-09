// superfargoth patch (bevyengine/bevy#25978): new file.
//! GPU-authored mesh instances.
//!
//! A [`GpuBatchedMesh3d`] reserves an output buffer of `max_capacity`
//! [`GpuMeshInstance`]s that a compute shader fills in
//! [`GpuInstanceBatchSystems::Publish`]. The batch then draws through the
//! standard mesh pipeline as a single indirect draw.

mod materialize;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetEvent, AssetEventSystems, AssetId, Assets};
use bevy_camera::{
    primitives::{Aabb, MeshAabb},
    visibility::{
        add_visibility_class, NoCpuCulling, NoFrustumCulling, Visibility, VisibilityClass,
    },
};
use bevy_core_pipeline::core_3d::{Transparent3d, TransparentSortingInfo3d};
use bevy_diagnostic::FrameCount;
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowReceiver, TransmittedShadowReceiver};
// superfargoth patch (bevyengine/bevy#25978): light visibility for batches.
use bevy_camera::{
    visibility::{
        CascadesVisibleEntities, CubemapVisibleEntities, InheritedVisibility, RenderLayers,
        SetViewVisibility, ViewVisibility, VisibilityRange, VisibilitySystems, VisibleEntities,
        VisibleEntityRanges,
        VisibleMeshEntities,
    },
    Camera, RenderTarget, ShadowLodOrigin,
};
use bevy_ecs::entity::EntityHashSet;
use bevy_light::{
    cluster::ClusterVisibilityClass, get_shadow_lod_origin, DirectionalLight, NotShadowCaster,
    PointLight, SimulationLightSystems, SpotLight,
};
use core::any::TypeId;
use bevy_log::{warn, warn_once};
use bevy_math::{Vec3, Vec4};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_platform::collections::{HashMap, HashSet};
use bevy_render::{
    // superfargoth patch (own, GS: ranges): `InstanceBatchSpan`,
    // `MaterialBindingId` and `MeshSlabs` for the per-range resolution.
    batching::{
        gpu_preprocessing::{BatchedInstanceBuffers, GpuPreprocessingSupport},
        InstanceBatchSpan,
    },
    material_bind_groups::{MaterialBindingId, RenderMaterialBindings},
    mesh::allocator::{MeshAllocator, MeshSlabs},
    render_phase::{sort_phase_system, ViewSortedRenderPhases},
    render_resource::{Buffer, BufferDescriptor, BufferUsages, PipelineCache, ShaderType},
    renderer::{RenderDevice, RenderGraph, RenderGraphSystems, RenderQueue},
    sync_world::{MainEntity, MainEntityHashMap},
    Extract, ExtractSchedule, Render, RenderApp, RenderSystems,
};
use bevy_transform::components::{GlobalTransform, Transform};
use bytemuck::{Pod, Zeroable};

use crate::{
    collect_gpu_culled_meshes, collect_meshes_for_gpu_building, MeshCullingData,
    MeshCullingDataBuffer, MeshInputUniform, MeshUniform, RenderMaterialInstances,
    RenderMeshInstanceBatch, RenderMeshInstanceBatches,
};
use crate::{
    // superfargoth patch (own, GS: range unpack in preprocess): the range dispatch moved inside
    // `early_gpu_preprocess`, so this plugin no longer registers a system into any schedule
    // (`unpack_ranges`, `unpack_bins` and `early_gpu_preprocess` are no longer named here).
    prepare_preprocess_pipelines, prepare_range_unpacking_bind_groups,
    prepare_range_unpacking_pipeline, RangeUnpackingPipeline,
};
use bevy_render::{render_resource::SpecializedComputePipelines, GpuResourceAppExt};
use materialize::GpuInstanceMaterialization;
// superfargoth patch (own, GS: ranges): the GPU range record is part of the
// batch's public contract (a producer sees it in `gpu_batch_tests`).
pub use materialize::GpuBatchGeometryRange;

/// One instance slot published by a GPU producer. Zeroed slots are inactive.
#[derive(Clone, Copy, Default, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct GpuMeshInstance {
    pub world_from_local: [Vec4; 3],
    pub is_active: u32,
    pub tag: u32,
    pub pad: [u32; 2],
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GpuInstanceBatchSystems {
    /// Producer work. Commands recorded through `RenderContext` here are
    /// submitted before materialization.
    Publish,
    /// The renderer's copy of active slots into mesh inputs, before any view
    /// renders.
    Materialize,
}

/// Up to `max_capacity` GPU-authored instances of a mesh, drawn as one indirect
/// draw. Pair with `MeshMaterial3d<M>`, not [`Mesh3d`].
///
/// # Hardware floor
///
/// superfargoth patch (own, GS: guards). **GPU instance batches require
/// `GpuPreprocessingMode::Culling`: an adapter limited to `PreprocessingOnly`,
/// and a camera (or an inherited view) carrying `NoIndirectDrawing`, draw
/// none.**
///
/// This is a floor, not a fallback. `batch_and_prepare_binned_render_phase`
/// and its sorted twin skip every instance batch of a `no_indirect_drawing`
/// view, and `PreprocessWorkItemBuffers::push_range` returns `None` on a
/// `Direct` work-item buffer, so such a view reserves nothing, queues no draw
/// record and shows no batch - the statics are simply **absent** there, not
/// drawn without culling. `prepare_gpu_batched_mesh_reservations` warns once
/// for the adapter case. A producer that must still show something on such an
/// adapter has to fall back to ordinary [`Mesh3d`] entities itself.
///
/// superfargoth: upstream gives this (and `Mesh3d`) a new `Mesh3dVisibility`
/// class in `bevy_mesh`; this port keeps `Mesh3d` as the class key (no
/// vendored `bevy_mesh`/`bevy_camera`/`bevy_light`): the batch pushes the
/// `Mesh3d` class in an `on_add` hook and joins light visibility in
/// [`add_gpu_batched_meshes_to_light_visibility`].
#[derive(Component, Clone)]
#[require(Transform, NoFrustumCulling, Visibility, VisibilityClass)]
#[component(on_add = add_visibility_class::<Mesh3d>)]
pub struct GpuBatchedMesh3d {
    pub mesh: bevy_asset::Handle<Mesh>,
    pub max_capacity: u32,
    // superfargoth patch (own, GS: ranges)
    /// Geometry ranges, sorted by [`GpuBatchedMeshRange::first_slot`] and
    /// partitioning `0..max_capacity` without gaps or overlap.
    ///
    /// **Empty = exactly upstream #25978 behaviour**: the whole batch draws
    /// `mesh`'s own index range, no range table is bound and `materialize`
    /// takes the same branch it took before this patch.
    ///
    /// Each range must resolve into the batch's own `MeshAllocator` vertex
    /// slab, index slab and material bind group; a range that does not is
    /// rejected per frame in `prepare_gpu_batched_mesh_draws` and the batch
    /// then draws nothing that frame (see [`validate_ranges`] for the
    /// slot-layout half, which is checked once per change on extraction).
    pub ranges: Vec<GpuBatchedMeshRange>,
}

// superfargoth patch (own, GS: ranges)
/// One geometry range of a [`GpuBatchedMesh3d`]: a contiguous span of instance
/// slots that all draw the same index range of the same mesh allocation with
/// the same material slot.
///
/// Every range must resolve into the same `MeshAllocator` vertex slab, index
/// slab and material bind group as the batch's own `mesh`: one indirect draw
/// binds exactly one of each.
#[derive(Clone, Debug)]
pub struct GpuBatchedMeshRange {
    /// The mesh whose allocation this range indexes into.
    pub mesh: bevy_asset::Handle<Mesh>,
    /// First index relative to `mesh`'s own index allocation; first vertex for
    /// a non-indexed mesh.
    pub first_index: u32,
    /// Index count (vertex count when non-indexed); 0 = the mesh's own count.
    pub index_count: u32,
    /// First instance slot of this range in the batch's output buffer.
    pub first_slot: u32,
    /// Number of instance slots in this range.
    pub slot_count: u32,
    /// Material to bind, or `None` for the batch's own `MeshMaterial3d<M>`.
    /// Must share the batch's bind group index. Untyped because
    /// `RenderMaterialBindings::get` is keyed by `UntypedAssetId`.
    pub material: Option<bevy_asset::UntypedAssetId>,
}

// superfargoth patch (own, GS: ranges)
/// Why a [`GpuBatchedMesh3d`]'s [`ranges`](GpuBatchedMesh3d::ranges) do not
/// describe a slot partition. Checked by [`validate_ranges`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeError {
    /// A range covers no instance slot. Spans are halves of a partition, so an
    /// empty one would make two ranges start at the same slot and break the
    /// "last range whose `first_slot <= slot`" search.
    EmptySpan { range: usize },
    /// `first_slot` is not where the previous range ended: ranges must be
    /// sorted by `first_slot` and tile `0..max_capacity` with no gap and no
    /// overlap.
    NotContiguous {
        range: usize,
        expected_first_slot: u32,
        first_slot: u32,
    },
    /// `first_slot + slot_count` overflows `u32`.
    SlotOverflow { range: usize },
    /// The ranges together cover `end` slots, not the batch's `max_capacity`.
    CapacityMismatch { end: u32, max_capacity: u32 },
}

// superfargoth patch (own, GS: ranges)
/// Checks that `ranges` is a slot partition of `0..max_capacity`: sorted by
/// `first_slot`, every span non-empty, each span starting exactly where the
/// previous one ended, and the last ending at `max_capacity`.
///
/// Empty `ranges` is always valid: it is the upstream "one mesh, one index
/// range" batch.
///
/// This is the contract [`range_of`] and its WGSL twin rely on; it does not
/// look at the index ranges, the slabs or the materials, which need a
/// `MeshAllocator` and are re-checked every frame in
/// `prepare_gpu_batched_mesh_draws`.
pub fn validate_ranges(ranges: &[GpuBatchedMeshRange], max_capacity: u32) -> Result<(), RangeError> {
    if ranges.is_empty() {
        return Ok(());
    }
    let mut expected_first_slot = 0u32;
    for (index, range) in ranges.iter().enumerate() {
        if range.slot_count == 0 {
            return Err(RangeError::EmptySpan { range: index });
        }
        if range.first_slot != expected_first_slot {
            return Err(RangeError::NotContiguous {
                range: index,
                expected_first_slot,
                first_slot: range.first_slot,
            });
        }
        expected_first_slot = range
            .first_slot
            .checked_add(range.slot_count)
            .ok_or(RangeError::SlotOverflow { range: index })?;
    }
    if expected_first_slot != max_capacity {
        return Err(RangeError::CapacityMismatch {
            end: expected_first_slot,
            max_capacity,
        });
    }
    Ok(())
}

// superfargoth patch (own, GS: ranges)
/// The index in a [`validate_ranges`]-valid range table of the range that owns
/// instance `slot`: the last range whose `first_slot <= slot`.
///
/// `first_slots` are the table's `first_slot`s in order. Mirrored line for line
/// by `range_of` in `materialize.wesl`, which is the copy that runs; this one
/// is the oracle the tests check, so keep the two in lockstep.
///
/// Returns 0 for an empty table: callers check the range count first (the WGSL
/// takes the whole range branch only when `metadata.range_count > 0`).
pub fn range_of(first_slots: &[u32], slot: u32) -> usize {
    let mut low = 0usize;
    let mut high = first_slots.len();
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if first_slots[mid] <= slot {
            low = mid;
        } else {
            high = mid;
        }
    }
    low
}

#[derive(Clone)]
pub(crate) struct ExtractedGpuBatchedMesh {
    pub mesh_asset_id: AssetId<Mesh>,
    pub max_capacity: u32,
    pub local_bounds: Option<Aabb>,
    pub flags: u32,
    pub world_center: Vec3,
    // superfargoth patch (own, GS: ranges)
    /// The batch's geometry ranges, slot-validated on extraction. Empty is
    /// upstream behaviour, whether the batch had no ranges or its ranges were
    /// rejected.
    pub ranges: Vec<ExtractedRange>,
}

// superfargoth patch (own, GS: ranges)
/// A [`GpuBatchedMeshRange`] as the render world sees it.
#[derive(Clone)]
pub(crate) struct ExtractedRange {
    pub mesh: AssetId<Mesh>,
    pub first_index: u32,
    pub index_count: u32,
    pub first_slot: u32,
    pub slot_count: u32,
    pub material: Option<bevy_asset::UntypedAssetId>,
    /// Model-space bounds of `mesh`, for this range's culling record.
    pub local_bounds: Option<Aabb>,
}

#[derive(Resource, Default)]
pub(crate) struct ExtractedGpuBatchedMeshes(pub(crate) MainEntityHashMap<ExtractedGpuBatchedMesh>);

/// Persistent published output for one draw. The producer owns the contents.
#[derive(Clone)]
pub struct GpuInstanceBatchReservation {
    output_buffer: Buffer,
    max_capacity: u32,
}

impl GpuInstanceBatchReservation {
    /// The buffer of [`max_capacity`](Self::max_capacity) [`GpuMeshInstance`]s.
    /// Its identity changes when the capacity changes
    pub fn output_buffer(&self) -> &Buffer {
        &self.output_buffer
    }

    /// The number of instance slots in the output buffer.
    pub fn max_capacity(&self) -> u32 {
        self.max_capacity
    }
}

/// Every live [`GpuBatchedMesh3d`]'s output, keyed by main-world entity.
#[derive(Resource, Default)]
pub struct GpuInstanceBatchReservations {
    by_entity: MainEntityHashMap<GpuInstanceBatchReservation>,
}

impl GpuInstanceBatchReservations {
    pub fn get(&self, entity: MainEntity) -> Option<&GpuInstanceBatchReservation> {
        self.by_entity.get(&entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (MainEntity, &GpuInstanceBatchReservation)> {
        self.by_entity
            .iter()
            .map(|(entity, output)| (*entity, output))
    }
}

pub struct GpuInstanceBatchPlugin;

impl Plugin for GpuInstanceBatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            mark_gpu_batched_meshes_as_changed_if_their_assets_changed.after(AssetEventSystems),
        );
        // superfargoth patch (bevyengine/bevy#25978): see `add_gpu_batched_meshes_to_light_visibility`.
        app.add_systems(
            PostUpdate,
            add_gpu_batched_meshes_to_light_visibility
                .after(SimulationLightSystems::CheckLightVisibility)
                .before(VisibilitySystems::MarkNewlyHiddenEntitiesInvisible),
        );
        bevy_shader::load_shader_library!(app, "gpu_instance.wesl");
        materialize::build(app);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_gpu_resource::<SpecializedComputePipelines<RangeUnpackingPipeline>>()
            .add_systems(
                Render,
                prepare_range_unpacking_pipeline
                    .in_set(RenderSystems::PrepareMeshes)
                    .before(prepare_gpu_batched_mesh_draws)
                    .run_if(
                        resource_exists::<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
                    ),
            )
            .add_systems(
                Render,
                prepare_range_unpacking_bind_groups
                    .in_set(RenderSystems::PrepareBindGroups)
                    .after(prepare_preprocess_pipelines)
                    .run_if(
                        resource_exists::<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
                    ),
            )
            // superfargoth patch (own, GS: range unpack in preprocess): #25978 registered
            // `unpack_ranges` here, into `bevy_core_pipeline::Core3d` only, so the point/spot
            // shadow schedule (`Core3dShadows`, patch #26149) and `sf_shadow_cache_bevy`'s
            // `LampShadowView` never unpacked and read uninitialised work items. The dispatch
            // now lives inside `early_gpu_preprocess`, which every such schedule runs, so there
            // is nothing to register per schedule. `prepare_range_unpacking_pipeline` and
            // `prepare_range_unpacking_bind_groups` stay here: with this plugin absent the
            // pipeline id stays `None` and the dispatch is skipped, so the port stays opt-in.
            .init_resource::<ExtractedGpuBatchedMeshes>()
            .init_gpu_resource::<GpuInstanceBatchReservations>()
            .add_systems(ExtractSchedule, extract_gpu_batched_meshes)
            .add_systems(
                Render,
                (
                    prepare_gpu_batched_mesh_reservations,
                    prepare_gpu_batched_mesh_draws
                        .after(collect_meshes_for_gpu_building)
                        .after(collect_gpu_culled_meshes)
                        .before(crate::prepass::specialize_prepass_material_meshes),
                )
                    // superfargoth patch (own, GS: guards): #25978 left these two in a bare
                    // tuple, i.e. unordered. `prepare_gpu_batched_mesh_draws` does not read
                    // `GpuInstanceBatchReservations` today, so nothing is wrong yet - but it
                    // resolves the batch that `prepare_gpu_batched_mesh_reservations` sized, and
                    // step 2's per-range validation will read the reservation, so pin the order
                    // now rather than find out as a frame-order bug.
                    .chain()
                    .in_set(RenderSystems::PrepareMeshes)
                    .run_if(
                        resource_exists::<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
                    ),
            )
            .add_systems(
                Render,
                refresh_gpu_batched_mesh_sort_centers
                    .in_set(RenderSystems::PhaseSort)
                    .before(sort_phase_system::<Transparent3d>),
            )
            .configure_sets(
                RenderGraph,
                (
                    GpuInstanceBatchSystems::Publish,
                    GpuInstanceBatchSystems::Materialize,
                )
                    .chain()
                    .after(RenderGraphSystems::Begin)
                    .before(RenderGraphSystems::Render),
            );
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app(RenderApp)
            && !matches!(
                render_app
                    .world()
                    .get_resource::<crate::RenderMeshInstances>(),
                Some(crate::RenderMeshInstances::GpuBuilding(_))
            )
        {
            warn!("GpuInstanceBatchPlugin requires GPU instance buffer building");
        }
    }
}

pub(crate) fn mark_gpu_batched_meshes_as_changed_if_their_assets_changed(
    mut batches: Query<&mut GpuBatchedMesh3d>,
    mut events: MessageReader<AssetEvent<Mesh>>,
) {
    let changed: HashSet<_> = events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .collect();
    if changed.is_empty() {
        return;
    }
    for mut batch in &mut batches {
        if changed.contains(&batch.mesh.id()) {
            batch.set_changed();
        }
    }
}

pub(crate) fn extract_gpu_batched_meshes(
    mut extracted: ResMut<ExtractedGpuBatchedMeshes>,
    meshes: Extract<Res<Assets<Mesh>>>,
    query: Extract<
        Query<(
            Entity,
            &GpuBatchedMesh3d,
            Option<&Aabb>,
            &GlobalTransform,
            Has<NotShadowReceiver>,
            Has<TransmittedShadowReceiver>,
            Has<Mesh3d>,
            Has<NoCpuCulling>,
        )>,
    >,
    mut events: Extract<MessageReader<AssetEvent<Mesh>>>,
    mut mesh_bounds: Local<HashMap<AssetId<Mesh>, Option<Aabb>>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } | AssetEvent::Removed { id } => {
                mesh_bounds.remove(id);
            }
            _ => {}
        }
    }
    extracted.0.clear();
    for (
        entity,
        batch,
        bounds,
        transform,
        not_receiver,
        transmitted_receiver,
        has_mesh,
        no_cpu_culling,
    ) in &query
    {
        if has_mesh {
            warn_once!("GpuBatchedMesh3d and Mesh3d must not coexist on {entity:?}");
        }
        if no_cpu_culling {
            warn_once!("NoCpuCulling removes GpuBatchedMesh3d {entity:?} from every view");
        }
        if batch.max_capacity == 0 {
            continue;
        }
        // superfargoth patch (own, GS: ranges): the slot layout is checked
        // here, once per change of the component (extraction only runs for
        // live entities and the whole map is rebuilt each frame, so "once per
        // change" is as cheap as the validation is). A bad layout clears the
        // ranges, which is exactly upstream behaviour rather than a dropped
        // batch: the batch keeps drawing its own mesh.
        let mut ranges = Vec::new();
        match validate_ranges(&batch.ranges, batch.max_capacity) {
            Ok(()) => {
                for range in &batch.ranges {
                    let local_bounds = *mesh_bounds
                        .entry(range.mesh.id())
                        .or_insert_with(|| meshes.get(&range.mesh).and_then(Mesh::get_aabb));
                    ranges.push(ExtractedRange {
                        mesh: range.mesh.id(),
                        first_index: range.first_index,
                        index_count: range.index_count,
                        first_slot: range.first_slot,
                        slot_count: range.slot_count,
                        material: range.material,
                        local_bounds,
                    });
                }
            }
            Err(error) => {
                warn_once!(
                    "GpuBatchedMesh3d {entity:?} has geometry ranges that do not partition its \
                     {} instance slots ({error:?}); drawing its own mesh instead",
                    batch.max_capacity
                );
            }
        }
        extracted.0.insert(
            entity.into(),
            ExtractedGpuBatchedMesh {
                mesh_asset_id: batch.mesh.id(),
                max_capacity: batch.max_capacity,
                // superfargoth patch (own, GS: ranges)
                ranges,
                local_bounds: *mesh_bounds
                    .entry(batch.mesh.id())
                    .or_insert_with(|| meshes.get(&batch.mesh).and_then(Mesh::get_aabb)),
                flags: u16::MAX as u32
                    | if not_receiver {
                        0
                    } else {
                        crate::MeshFlags::SHADOW_RECEIVER.bits()
                    }
                    | if transmitted_receiver {
                        crate::MeshFlags::TRANSMITTED_SHADOW_RECEIVER.bits()
                    } else {
                        0
                    },
                world_center: transform
                    .transform_point(bounds.map_or(Vec3::ZERO, |b| b.center.into())),
            },
        );
    }
}

fn prepare_gpu_batched_mesh_reservations(
    extracted: Res<ExtractedGpuBatchedMeshes>,
    mut reservations: ResMut<GpuInstanceBatchReservations>,
    support: Res<GpuPreprocessingSupport>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    reservations
        .by_entity
        .retain(|entity, _| extracted.0.contains_key(entity));
    if !support.is_culling_supported() {
        if !extracted.0.is_empty() {
            warn_once!("GpuBatchedMesh3d requires GPU culling support; nothing will be drawn");
        }
        return;
    }
    for (&entity, draw) in &extracted.0 {
        reservations
            .by_entity
            .entry(entity)
            .or_insert_with(|| GpuInstanceBatchReservation::new(draw.max_capacity, &device))
            .resize(draw.max_capacity, &device, &queue);
    }
}

fn prepare_gpu_batched_mesh_draws(
    extracted: Res<ExtractedGpuBatchedMeshes>,
    mut buffers: ResMut<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    mut culling: ResMut<MeshCullingDataBuffer>,
    mut batches: ResMut<RenderMeshInstanceBatches>,
    mut materialization: ResMut<GpuInstanceMaterialization>,
    allocator: Res<MeshAllocator>,
    materials: Res<RenderMaterialInstances>,
    bindings: Res<RenderMaterialBindings>,
    frame: Res<FrameCount>,
    support: Res<GpuPreprocessingSupport>,
    device: Res<RenderDevice>,
    cache: Res<PipelineCache>,
    pipeline: Option<Res<materialize::MaterializationPipeline>>,
    preprocess: Res<crate::PreprocessPipelines>,
) {
    batches.clear();
    materialization
        .jobs
        .retain(|entity, _| extracted.0.contains_key(entity));
    for job in materialization.jobs.values_mut() {
        job.active = false;
    }
    if !support.is_culling_supported() || extracted.0.is_empty() {
        return;
    }

    let writers_ready = materialize::pipelines_ready(
        pipeline.as_deref(),
        preprocess.range_unpacking.pipeline_id,
        &cache,
    );
    let input = &mut buffers.current_input_buffer;
    input.ensure_nonempty();
    let mut scratch_end = input.len().max(culling.len() as usize) as u32;
    for (&entity, draw) in &extracted.0 {
        let batch = batches.entry(entity).or_insert(RenderMeshInstanceBatch {
            asset_id: draw.mesh_asset_id,
            input_range: 0..0,
            world_center: draw.world_center,
            // superfargoth patch (own, GS: ranges)
            spans: Vec::new(),
        });
        if !writers_ready {
            continue;
        }
        let Some(material) = materials.instances.get(&entity) else {
            continue;
        };
        let Some(binding) = bindings.get(&material.asset_id) else {
            continue;
        };
        let Some(mut template) =
            MeshInputUniform::for_mesh(&allocator, draw.mesh_asset_id, *binding, u16::MAX)
        else {
            continue;
        };
        template.flags = draw.flags;
        template.timestamp = frame.0;
        // superfargoth patch (own, GS: ranges): resolve every range against
        // this frame's slabs and material bindings. Slabs move
        // (`meshes_displaced_by_slab_growth`), so this is a per-frame check,
        // not a once-per-change one; a rejected batch leaves `input_range` and
        // `spans` empty, the port's existing "do not draw this frame" signal,
        // and its materialization job inactive.
        let Some(gpu_ranges) = resolve_gpu_ranges(entity, draw, &allocator, &bindings, *binding)
        else {
            continue;
        };
        let base = scratch_end;
        scratch_end = scratch_end
            .checked_add(draw.max_capacity)
            .expect("GPU mesh input range overflow");
        materialization.jobs.entry(entity).or_default().update(
            template,
            MeshCullingData::new(draw.local_bounds.as_ref()),
            base,
            draw.max_capacity,
            // superfargoth patch (own, GS: ranges)
            &gpu_ranges,
        );
        batch.input_range = base..scratch_end;
        // superfargoth patch (own, GS: ranges): one span per range, in slot
        // order, tiling `input_range`; one span for the whole batch when it
        // has no ranges (upstream #25978).
        batch.spans = if draw.ranges.is_empty() {
            vec![InstanceBatchSpan {
                base_input_index: base,
                count: draw.max_capacity,
            }]
        } else {
            draw.ranges
                .iter()
                .map(|range| InstanceBatchSpan {
                    base_input_index: base + range.first_slot,
                    count: range.slot_count,
                })
                .collect()
        };
    }
    if !materialization.jobs.is_empty() {
        input.reserve_gpu(scratch_end, &device);
        culling.reserve_amortized(scratch_end as usize, &device);
    }
}

// superfargoth patch (own, GS: ranges)
/// Resolves a batch's geometry ranges into this frame's GPU range records, or
/// `None` (warning once) if any range cannot be drawn by the batch's single
/// indirect draw.
///
/// Returns an empty vector for a batch without ranges: it draws its own mesh,
/// exactly as upstream #25978 drew it.
fn resolve_gpu_ranges(
    entity: MainEntity,
    draw: &ExtractedGpuBatchedMesh,
    allocator: &MeshAllocator,
    bindings: &RenderMaterialBindings,
    batch_binding: MaterialBindingId,
) -> Option<Vec<GpuBatchGeometryRange>> {
    if draw.ranges.is_empty() {
        return Some(Vec::new());
    }
    let batch_slabs = allocator.mesh_slabs(&draw.mesh_asset_id);
    let mut records = Vec::with_capacity(draw.ranges.len());
    for (index, range) in draw.ranges.iter().enumerate() {
        match resolve_gpu_range(range, allocator, bindings, batch_binding, batch_slabs.as_ref()) {
            Ok(record) => records.push(record),
            Err(why) => {
                warn_once!(
                    "GpuBatchedMesh3d {entity:?} geometry range {index} {why}; the batch draws \
                     nothing this frame"
                );
                return None;
            }
        }
    }
    Some(records)
}

// superfargoth patch (own, GS: ranges)
/// One range's geometry, in the batch's own slabs and material bind group: the
/// draw binds exactly one vertex buffer, one index buffer and one material
/// bind group for all of its sub-draws.
fn resolve_gpu_range(
    range: &ExtractedRange,
    allocator: &MeshAllocator,
    bindings: &RenderMaterialBindings,
    batch_binding: MaterialBindingId,
    batch_slabs: Option<&MeshSlabs>,
) -> Result<GpuBatchGeometryRange, &'static str> {
    let geometry = crate::render::mesh::MeshInputGeometry::new(allocator, range.mesh)
        .ok_or("has no allocated mesh data yet")?;
    // `index_count` 0 means "the rest of the mesh"; `index_count` is the
    // vertex count for a non-indexed mesh.
    let index_count = if range.index_count == 0 {
        geometry
            .index_count
            .checked_sub(range.first_index)
            .ok_or("starts past the end of its mesh")?
    } else {
        range.index_count
    };
    if range
        .first_index
        .checked_add(index_count)
        .is_none_or(|end| end > geometry.index_count)
    {
        return Err("draws past the end of its mesh");
    }
    let slabs = allocator.mesh_slabs(&range.mesh);
    let (Some(slabs), Some(batch_slabs)) = (slabs.as_ref(), batch_slabs) else {
        return Err("has no allocated slabs yet");
    };
    if slabs.vertex_slab_id != batch_slabs.vertex_slab_id
        || slabs.index_slab_id != batch_slabs.index_slab_id
    {
        return Err("is not in its batch's vertex and index slabs");
    }
    // A non-indexed range offsets the first vertex instead: that is what the
    // draw reads (`build_indirect_params.wesl`).
    let (first_vertex_index, first_index_index) = if slabs.index_slab_id.is_some() {
        (
            geometry.first_vertex_index,
            geometry
                .first_index_index
                .checked_add(range.first_index)
                .ok_or("overflows its index slab")?,
        )
    } else {
        (
            geometry
                .first_vertex_index
                .checked_add(range.first_index)
                .ok_or("overflows its vertex slab")?,
            0,
        )
    };
    let binding = match range.material {
        Some(material) => *bindings.get(&material).ok_or("has an unprepared material")?,
        None => batch_binding,
    };
    if binding.group != batch_binding.group {
        return Err("is not in its batch's material bind group");
    }
    let bounds = MeshCullingData::new(range.local_bounds.as_ref());
    Ok(GpuBatchGeometryRange {
        first_slot: range.first_slot,
        first_vertex_index,
        first_index_index,
        index_count,
        metadata_index: geometry.metadata_index,
        material_and_lightmap_bind_group_slot:
            crate::render::mesh::material_and_lightmap_bind_group_slot(binding, u16::MAX),
        flags: 0,
        pad0: 0,
        aabb_center: bounds.aabb_center,
        pad1: 0.0,
        aabb_half_extents: bounds.aabb_half_extents,
        pad2: 0.0,
    })
}

fn refresh_gpu_batched_mesh_sort_centers(
    batches: Res<RenderMeshInstanceBatches>,
    mut phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
) {
    if batches.is_empty() {
        return;
    }
    for phase in phases.0.values_mut() {
        for (entity, batch) in batches.iter() {
            // `queue_material_meshes` keys mesh items by main entity only.
            let Some(item) = phase.items.get_mut(&(Entity::PLACEHOLDER, *entity)) else {
                continue;
            };
            if let TransparentSortingInfo3d::Sorted { mesh_center, .. } = &mut item.sorting_info {
                *mesh_center = batch.world_center;
            }
        }
    }
}

impl GpuInstanceBatchReservation {
    fn new(capacity: u32, device: &RenderDevice) -> Self {
        Self {
            output_buffer: device.create_buffer(&BufferDescriptor {
                label: Some("persistent GPU mesh instances"),
                size: u64::from(capacity) * size_of::<GpuMeshInstance>() as u64,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            max_capacity: capacity,
        }
    }

    fn resize(&mut self, capacity: u32, device: &RenderDevice, queue: &RenderQueue) {
        if self.max_capacity == capacity {
            return;
        }
        let replacement = Self::new(capacity, device);
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(
            &self.output_buffer,
            0,
            &replacement.output_buffer,
            0,
            u64::from(capacity.min(self.max_capacity)) * size_of::<GpuMeshInstance>() as u64,
        );
        queue.submit([encoder.finish()]);
        *self = replacement;
    }
}

// superfargoth patch (bevyengine/bevy#25978): upstream's light visibility
// checks (`bevy_light::check_*_light_mesh_visibility`) query
// `With<Mesh3dVisibility>`, which the batch shares with `Mesh3d`. Our 0.20
// `bevy_light` queries `With<Mesh3d>` and is not vendored, so this adds each
// batch to the light lists afterwards with the same rules those checks apply
// to a `NoFrustumCulling` entity: every cascade of every view of a visible,
// shadow-mapped directional light; every face of a shadow-mapped point light
// and every shadow-mapped spot light that a camera's clusters see. Render
// layers, `InheritedVisibility`, `NotShadowCaster` and `VisibilityRange` are
// honoured as there.
#[allow(clippy::type_complexity)]
pub(crate) fn add_gpu_batched_meshes_to_light_visibility(
    mut batches: Query<
        (
            Entity,
            &InheritedVisibility,
            &mut ViewVisibility,
            Option<&RenderLayers>,
            Has<VisibilityRange>,
        ),
        (
            With<GpuBatchedMesh3d>,
            Without<NotShadowCaster>,
            Without<NoCpuCulling>,
            Without<Mesh3d>,
        ),
    >,
    mut directional_lights: Query<
        (
            &DirectionalLight,
            &mut CascadesVisibleEntities,
            Option<&RenderLayers>,
            &ViewVisibility,
        ),
        (Without<SpotLight>, Without<GpuBatchedMesh3d>),
    >,
    camera_visible_lights: Query<&VisibleEntities>,
    mut point_lights: Query<
        (&PointLight, &mut CubemapVisibleEntities, Option<&RenderLayers>),
        Without<GpuBatchedMesh3d>,
    >,
    mut spot_lights: Query<
        (&SpotLight, &mut VisibleMeshEntities, Option<&RenderLayers>),
        Without<GpuBatchedMesh3d>,
    >,
    mut camera_query: Query<(Entity, &RenderTarget), With<Camera>>,
    mut shadow_lod_origin_query: Query<Entity, With<ShadowLodOrigin>>,
    mut point_and_spot_light_query: Query<Entity, Or<(With<PointLight>, With<SpotLight>)>>,
    visible_entity_ranges: Option<Res<VisibleEntityRanges>>,
    mut checked_lights: Local<EntityHashSet>,
) {
    if batches.is_empty() {
        return;
    }
    let ranges = visible_entity_ranges.as_deref();
    let in_range = |entity: Entity, has_range: bool, view: Option<Entity>| {
        !has_range
            || ranges.is_none_or(|ranges| {
                view.is_some_and(|view| ranges.entity_is_in_range_of_view(entity, view))
            })
    };
    fn insert_sorted(list: &mut VisibleMeshEntities, entity: Entity) {
        if let Err(index) = list.entities.binary_search(&entity) {
            list.entities.insert(index, entity);
        }
    }

    for (light, mut cascades, light_layers, light_visibility) in &mut directional_lights {
        if !light.shadow_maps_enabled || !light_visibility.get() {
            continue;
        }
        let light_layers = light_layers.unwrap_or_default();
        for (view, cascade_lists) in cascades.entities.iter_mut() {
            for (entity, inherited, mut view_visibility, layers, has_range) in &mut batches {
                if !inherited.get()
                    || !light_layers.intersects(layers.unwrap_or_default())
                    || !in_range(entity, has_range, Some(*view))
                {
                    continue;
                }
                for list in cascade_lists.iter_mut() {
                    insert_sorted(list, entity);
                }
                if !cascade_lists.is_empty() {
                    view_visibility.set_visible();
                }
            }
        }
    }

    let shadow_lod_origin = get_shadow_lod_origin(
        camera_query.transmute_lens_filtered(),
        shadow_lod_origin_query.transmute_lens_filtered(),
        point_and_spot_light_query.transmute_lens_filtered(),
    );
    checked_lights.clear();
    for visible in &camera_visible_lights {
        for &light_entity in visible.get(TypeId::of::<ClusterVisibilityClass>()) {
            if !checked_lights.insert(light_entity) {
                continue;
            }
            if let Ok((light, mut faces, light_layers)) = point_lights.get_mut(light_entity)
                && light.shadow_maps_enabled
            {
                let light_layers = light_layers.unwrap_or_default();
                for (entity, inherited, mut view_visibility, layers, has_range) in &mut batches {
                    if inherited.get()
                        && light_layers.intersects(layers.unwrap_or_default())
                        && in_range(entity, has_range, shadow_lod_origin)
                    {
                        view_visibility.set_visible();
                        for face in faces.iter_mut() {
                            insert_sorted(face, entity);
                        }
                    }
                }
            }
            if let Ok((light, mut list, light_layers)) = spot_lights.get_mut(light_entity)
                && light.shadow_maps_enabled
            {
                let light_layers = light_layers.unwrap_or_default();
                for (entity, inherited, mut view_visibility, layers, has_range) in &mut batches {
                    if inherited.get()
                        && light_layers.intersects(layers.unwrap_or_default())
                        && in_range(entity, has_range, shadow_lod_origin)
                    {
                        view_visibility.set_visible();
                        insert_sorted(&mut list, entity);
                    }
                }
            }
        }
    }
}
