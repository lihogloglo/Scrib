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
        ViewVisibility, VisibilityRange, VisibilitySystems, VisibleEntities, VisibleEntityRanges,
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
    batching::gpu_preprocessing::{BatchedInstanceBuffers, GpuPreprocessingSupport},
    material_bind_groups::RenderMaterialBindings,
    mesh::allocator::MeshAllocator,
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
    early_gpu_preprocess, prepare_preprocess_pipelines, prepare_range_unpacking_bind_groups,
    prepare_range_unpacking_pipeline, unpack_bins, unpack_ranges, RangeUnpackingPipeline,
};
use bevy_render::{render_resource::SpecializedComputePipelines, GpuResourceAppExt};
use materialize::GpuInstanceMaterialization;

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
}

#[derive(Clone)]
pub(crate) struct ExtractedGpuBatchedMesh {
    pub mesh_asset_id: AssetId<Mesh>,
    pub max_capacity: u32,
    pub local_bounds: Option<Aabb>,
    pub flags: u32,
    pub world_center: Vec3,
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
            .add_systems(
                bevy_core_pipeline::Core3d,
                unpack_ranges
                    .after(unpack_bins)
                    .before(early_gpu_preprocess)
                    .run_if(
                        resource_exists::<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
                    ),
            )
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
        extracted.0.insert(
            entity.into(),
            ExtractedGpuBatchedMesh {
                mesh_asset_id: batch.mesh.id(),
                max_capacity: batch.max_capacity,
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
        let base = scratch_end;
        scratch_end = scratch_end
            .checked_add(draw.max_capacity)
            .expect("GPU mesh input range overflow");
        materialization.jobs.entry(entity).or_default().update(
            template,
            MeshCullingData::new(draw.local_bounds.as_ref()),
            base,
            draw.max_capacity,
        );
        batch.input_range = base..scratch_end;
    }
    if !materialization.jobs.is_empty() {
        input.reserve_gpu(scratch_end, &device);
        culling.reserve_amortized(scratch_end as usize, &device);
    }
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
