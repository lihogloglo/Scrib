// superfargoth patch (bevyengine/bevy#25978): new file.
use bevy_app::App;
use bevy_asset::{embedded_asset, load_embedded_asset};
use bevy_ecs::prelude::*;
// superfargoth patch (own, GS: ranges): `Vec3` and `Pod` for the range record.
use bevy_math::Vec3;
use bytemuck::{Pod, Zeroable};
use bevy_render::{
    batching::gpu_preprocessing::BatchedInstanceBuffers,
    diagnostic::RecordDiagnostics as _,
    render_resource::{
        binding_types::{storage_buffer, storage_buffer_read_only, uniform_buffer},
        BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BufferId,
        BufferUsages, CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor,
        PipelineCache, RawBufferVec, ShaderStages, ShaderType, UniformBuffer,
    },
    renderer::{RenderContext, RenderDevice, RenderGraph, RenderQueue},
    sync_world::MainEntityHashMap,
    GpuResourceAppExt, Render, RenderApp, RenderStartup, RenderSystems,
};

use super::{
    GpuInstanceBatchReservation, GpuInstanceBatchReservations, GpuInstanceBatchSystems,
    GpuMeshInstance,
};
use crate::{
    MeshCullingData, MeshCullingDataBuffer, MeshInputUniform, MeshUniform,
    // superfargoth patch (own, GS: counters)
    PreprocessBindGroupCount,
};

// superfargoth patch (own, GS: ranges)
/// One geometry range of a batch, as `materialize.wesl` reads it: the
/// `MeshInput` geometry fields that every instance in the slot span
/// `first_slot..` (up to the next range's `first_slot`) draws, plus that
/// span's model-space culling bounds.
///
/// 64 bytes, 16-aligned, identical in Rust and WGSL: the `vec3<f32>` align-16
/// rule puts `aabb_center` at 32 and `aabb_half_extents` at 48 with no
/// implicit padding. Asserted by `sf_world_bevy`'s `gpu_batch_tests`.
#[derive(Clone, Copy, Default, Pod, Zeroable, ShaderType, Debug, PartialEq)]
#[repr(C)]
pub struct GpuBatchGeometryRange {
    /// First instance slot of this range. Ranges are sorted by this field and
    /// partition the batch, so the range of a slot is the last one whose
    /// `first_slot <= slot`.
    pub first_slot: u32,
    pub first_vertex_index: u32,
    pub first_index_index: u32,
    pub index_count: u32,
    pub metadata_index: u32,
    pub material_and_lightmap_bind_group_slot: u32,
    /// `MeshFlags` to OR into the instance's flags. Reserved; 0 today.
    pub flags: u32,
    pub pad0: u32,
    pub aabb_center: Vec3,
    pub pad1: f32,
    pub aabb_half_extents: Vec3,
    pub pad2: f32,
}

#[derive(Clone, Copy, Default, ShaderType)]
struct MaterializationMetadata {
    template: MeshInputUniform,
    bounds: MeshCullingData,
    input_base: u32,
    capacity: u32,
    // superfargoth patch (own, GS: ranges): in place of one `pad` word, so the
    // struct stays 16-aligned. 0 = no range table: the shader then takes
    // exactly the path it took before this patch.
    range_count: u32,
    pad: u32,
}

pub(super) struct MaterializationJob {
    metadata: UniformBuffer<MaterializationMetadata>,
    bind_group: Option<BindGroup>,
    // superfargoth patch (own, GS: ranges): four ids now - the range buffer's
    // identity changes when it grows, and a stale bind group would then point
    // at the freed buffer.
    bound_buffers: Option<[BufferId; 4]>,
    // superfargoth patch (own, GS: ranges)
    /// This batch's geometry ranges, in slot order. Always holds at least one
    /// record (a storage binding cannot be empty); `range_count` says how many
    /// of them the shader may read.
    ranges: RawBufferVec<GpuBatchGeometryRange>,
    range_count: u32,
    /// Set when `ranges` changed, cleared once it has been uploaded:
    /// `RawBufferVec::write_buffer` re-uploads unconditionally, and the table
    /// only changes when a producer republishes or a slab moves.
    ranges_dirty: bool,
    pub(super) active: bool,
}

// superfargoth patch (own, GS: ranges): `RawBufferVec` has no `Default`.
impl Default for MaterializationJob {
    fn default() -> Self {
        Self {
            metadata: UniformBuffer::default(),
            bind_group: None,
            bound_buffers: None,
            ranges: RawBufferVec::new(BufferUsages::STORAGE),
            range_count: 0,
            ranges_dirty: false,
            active: false,
        }
    }
}

impl MaterializationJob {
    pub fn update(
        &mut self,
        template: MeshInputUniform,
        bounds: MeshCullingData,
        input_base: u32,
        capacity: u32,
        // superfargoth patch (own, GS: ranges): empty = draw the batch's own
        // mesh over every slot, which is upstream #25978.
        ranges: &[GpuBatchGeometryRange],
        // superfargoth patch (own, GS: change detection): `false` when `ranges`
        // is the same cached table as at the last update (`GpuBatchRangeCache`
        // hit), which skips the O(ranges) comparison below.
        ranges_resolved: bool,
    ) {
        self.metadata.set(MaterializationMetadata {
            template,
            bounds,
            input_base,
            capacity,
            range_count: ranges.len() as u32,
            pad: 0,
        });
        // superfargoth patch (own, GS: ranges): compare the records rather
        // than hash them - the table is short, the comparison is exact, and
        // `ranges` empty on the first update must still upload the one record
        // that keeps the binding non-empty.
        let unchanged = !self.ranges.is_empty()
            && self.range_count == ranges.len() as u32
            // superfargoth patch (own, GS: change detection)
            && (!ranges_resolved || self.ranges.values()[..ranges.len()] == *ranges);
        debug_assert!(
            !unchanged || self.ranges.values()[..ranges.len()] == *ranges,
            "a cached range table differs from the uploaded one"
        );
        if !unchanged {
            self.range_count = ranges.len() as u32;
            self.ranges.clear();
            self.ranges.values_mut().extend_from_slice(ranges);
            if self.ranges.is_empty() {
                self.ranges.push(GpuBatchGeometryRange::default());
            }
            self.ranges_dirty = true;
        }
        self.active = true;
    }
}

#[derive(Resource, Default)]
pub(crate) struct GpuInstanceMaterialization {
    pub(super) jobs: MainEntityHashMap<MaterializationJob>,
}

#[derive(Resource)]
pub(crate) struct MaterializationPipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId,
}

pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "materialize.wesl");
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };
    render_app
        .init_gpu_resource::<GpuInstanceMaterialization>()
        .add_systems(RenderStartup, init_pipeline)
        .add_systems(
            Render,
            prepare_bind_groups
                .in_set(RenderSystems::PrepareBindGroups)
                .run_if(resource_exists::<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>)
                .run_if(resource_exists::<MaterializationPipeline>),
        )
        .add_systems(
            RenderGraph,
            materialize
                .in_set(GpuInstanceBatchSystems::Materialize)
                .run_if(resource_exists::<MaterializationPipeline>),
        );
}

fn init_pipeline(
    mut commands: Commands,
    cache: Res<PipelineCache>,
    assets: Res<bevy_asset::AssetServer>,
    support: Res<bevy_render::batching::gpu_preprocessing::GpuPreprocessingSupport>,
) {
    if !support.is_culling_supported() {
        return;
    }
    let layout = BindGroupLayoutDescriptor::new(
        "GPU instance materialization",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<MaterializationMetadata>(false),
                storage_buffer_read_only::<GpuMeshInstance>(false),
                storage_buffer::<MeshInputUniform>(false),
                storage_buffer::<MeshCullingData>(false),
                // superfargoth patch (own, GS: ranges): the range table.
                storage_buffer_read_only::<GpuBatchGeometryRange>(false),
            ),
        ),
    );
    let pipeline = cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("GPU instance materialization".into()),
        layout: vec![layout.clone()],
        shader: load_embedded_asset!(assets.as_ref(), "materialize.wesl"),
        ..Default::default()
    });
    commands.insert_resource(MaterializationPipeline { layout, pipeline });
}

pub(super) fn pipelines_ready(
    pipeline: Option<&MaterializationPipeline>,
    range_pipeline: Option<CachedComputePipelineId>,
    cache: &PipelineCache,
) -> bool {
    pipeline.is_some_and(|pipeline| cache.get_compute_pipeline(pipeline.pipeline).is_some())
        && range_pipeline.is_some_and(|id| cache.get_compute_pipeline(id).is_some())
}

fn prepare_bind_groups(
    mut materialization: ResMut<GpuInstanceMaterialization>,
    reservations: Res<GpuInstanceBatchReservations>,
    buffers: Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    culling: Res<MeshCullingDataBuffer>,
    pipeline: Res<MaterializationPipeline>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    // superfargoth patch (own, GS: counters)
    mut bind_group_count: ResMut<PreprocessBindGroupCount>,
) {
    // superfargoth patch (own, GS: counters): assigned, not accumulated, so
    // the ordering against the other `RenderSystems::PrepareBindGroups`
    // systems that write their own field does not matter.
    bind_group_count.materialize = 0;
    let (Some(input), Some(culling)) = (
        buffers.current_input_buffer.buffer().buffer(),
        culling.buffer(),
    ) else {
        return;
    };
    for (entity, job) in &mut materialization.jobs {
        if !job.active {
            continue;
        }
        let Some(output) = reservations
            .get(*entity)
            .map(GpuInstanceBatchReservation::output_buffer)
        else {
            continue;
        };
        job.metadata.write_buffer(&device, &queue);
        // superfargoth patch (own, GS: ranges): upload the range table before
        // anything can bind it. `write_buffer` reserves, so the buffer's
        // identity can change here, which is why it joins `bound_buffers`.
        if job.ranges_dirty {
            job.ranges.write_buffer(&device, &queue);
            job.ranges_dirty = false;
        }
        let Some(ranges) = job.ranges.buffer() else {
            continue;
        };
        let buffers = [output.id(), input.id(), culling.id(), ranges.id()];
        if job.bound_buffers == Some(buffers) {
            continue;
        }
        job.bound_buffers = Some(buffers);
        // superfargoth patch (own, GS: counters)
        bind_group_count.materialize = bind_group_count.materialize.saturating_add(1);
        job.bind_group = Some(device.create_bind_group(
            "GPU instance materialization",
            &cache.get_bind_group_layout(&pipeline.layout),
            &BindGroupEntries::sequential((
                job.metadata.binding().unwrap(),
                output.as_entire_binding(),
                input.as_entire_binding(),
                culling.as_entire_binding(),
                // superfargoth patch (own, GS: ranges)
                ranges.as_entire_binding(),
            )),
        ));
    }
}

fn materialize(
    mut render_context: RenderContext,
    materialization: Res<GpuInstanceMaterialization>,
    pipeline: Res<MaterializationPipeline>,
    cache: Res<PipelineCache>,
) {
    if !materialization.jobs.values().any(|job| job.active) {
        return;
    }
    let Some(pipeline) = cache.get_compute_pipeline(pipeline.pipeline) else {
        return;
    };
    let diagnostics = render_context.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = render_context
        .command_encoder()
        .begin_compute_pass(&ComputePassDescriptor {
            label: Some("GPU instance materialization"),
            timestamp_writes: None,
        });
    let pass_span = diagnostics.pass_span(&mut pass, "gpu_instance_materialization");
    pass.set_pipeline(pipeline);
    for job in materialization.jobs.values() {
        if !job.active {
            continue;
        }
        let Some(bind_group) = &job.bind_group else {
            continue;
        };
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(job.metadata.get().capacity.div_ceil(64), 1, 1);
    }
    pass_span.end(&mut pass);
}
