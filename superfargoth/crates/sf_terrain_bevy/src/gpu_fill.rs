//! GPU clipmap fill (worldgen.md "Pipeline" stage 7, slice G2): when the [`crate::TerrainWorld`]'s provider has a
//! [`GpuField`] (and [`crate::TerrainSettings::gpu_fill`] is on), the exposed height, normal and macro strips are
//! evaluated on the GPU instead of copied from CPU tiles. The main world queues [`FillRequest`]s (one per
//! toroidal piece, `upload_level`); the render world plans them (`sf_terrain::gpu_fill::plan`), runs the two
//! passes of the composed shader (`sf_terrain::gpu_fill::compose`) and copies the results into the clipmap
//! textures, in `RenderGraphSystems::Begin`, before any view draws (the pattern Bevy's Solari uses for its TLAS
//! build). Public render API only: the pipeline cache, render device and queue, the render context.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use bevy_asset::{AssetId, Handle};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_render::MainWorld;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{storage_buffer_read_only_sized, storage_buffer_sized};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer, BufferDescriptor,
    BufferInitDescriptor, BufferUsages, CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor,
    ComputePipelineDescriptor, Extent3d, Origin3d, PipelineCache, ShaderStages, TexelCopyBufferInfo,
    TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
};
use bevy_render::renderer::{RenderContext, RenderDevice};
use bevy_render::texture::GpuImage;
use bevy_shader::Shader;
use sf_terrain::gpu_fill::{self, FillPlan, FillRequest, FillTarget};

/// Cumulative counters (main world reads them for `TerrainStats` and logs).
#[derive(Default, Debug)]
pub struct GpuFillCounters {
    /// Pieces and texels dispatched, frames with a dispatch.
    pub pieces: AtomicU64,
    pub texels: AtomicU64,
    pub frames: AtomicU64,
}

/// The main world's half: the composed shader, the field's words, the target clipmaps.
#[derive(Resource, Clone)]
pub(crate) struct GpuFillMain {
    pub shader: Handle<Shader>,
    pub data: Arc<[u32]>,
    /// Bumped per field (a new `TerrainWorld`): the render world rebuilds its buffer and pipelines.
    pub rev: u64,
    pub heights: AssetId<Image>,
    pub normals: AssetId<Image>,
    pub macro_image: AssetId<Image>,
    /// Set by the render world once the pipelines compiled and the textures exist: until then the main
    /// world keeps its windows unfilled (nothing would write them).
    pub ready: Arc<AtomicBool>,
    pub counters: Arc<GpuFillCounters>,
}

/// Pieces queued by the main world this frame.
#[derive(Resource, Default)]
pub(crate) struct GpuFillQueue(pub Vec<FillRequest>);

/// The render world's half.
#[derive(Resource, Default)]
pub(crate) struct GpuFillRender {
    main: Option<GpuFillMain>,
    rev: Option<u64>,
    data: Option<Buffer>,
    pipelines: Option<[CachedComputePipelineId; 2]>,
    status: String,
    /// Pieces waiting for the pipelines (or queued this frame).
    pending: Vec<FillRequest>,
    scratch: Option<(Buffer, u64)>,
    out: Option<(Buffer, u64)>,
    /// This frame's dispatch.
    frame: Option<(FillPlan, BindGroup)>,
}

fn layout() -> BindGroupLayoutDescriptor {
    BindGroupLayoutDescriptor::new(
        "terrain_gpu_fill",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                storage_buffer_read_only_sized(false, None),
                storage_buffer_read_only_sized(false, None),
                storage_buffer_sized(false, None),
                storage_buffer_sized(false, None),
            ),
        ),
    )
}

pub(crate) fn extract(mut main: ResMut<MainWorld>, mut fill: ResMut<GpuFillRender>) {
    fill.main = main.get_resource::<GpuFillMain>().cloned();
    if let Some(mut queue) = main.get_resource_mut::<GpuFillQueue>() {
        if fill.main.is_some() {
            fill.pending.append(&mut queue.0);
        } else {
            queue.0.clear();
        }
    }
    if fill.main.is_none() {
        fill.pending.clear();
    }
}

/// A storage buffer of at least `words` u32, reused while big enough.
fn sized(device: &RenderDevice, slot: &mut Option<(Buffer, u64)>, label: &'static str, words: u32, usage: BufferUsages) -> Buffer {
    let bytes = u64::from(words.max(1)) * 4;
    match slot {
        Some((b, cap)) if *cap >= bytes => b.clone(),
        _ => {
            // Grow by half again so a creeping burst doesn't reallocate every frame.
            let cap = bytes + bytes / 2;
            let b = device.create_buffer(&BufferDescriptor { label: Some(label), size: cap, usage, mapped_at_creation: false });
            *slot = Some((b.clone(), cap));
            b
        }
    }
}

/// Field buffer and pipelines per field; this frame's plan, buffers and bind group once everything exists.
pub(crate) fn prepare(
    mut fill: ResMut<GpuFillRender>,
    device: Res<RenderDevice>,
    cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
) {
    let fill = &mut *fill;
    fill.frame = None;
    let Some(main) = fill.main.clone() else {
        fill.rev = None;
        fill.data = None;
        fill.pipelines = None;
        return;
    };
    if fill.rev != Some(main.rev) {
        let bytes: Vec<u8> = main.data.iter().flat_map(|w| w.to_le_bytes()).collect();
        fill.data = Some(device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("terrain_gpu_fill_field"),
            contents: &bytes,
            usage: BufferUsages::STORAGE,
        }));
        let pipeline = |entry: &'static str| {
            cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some(format!("terrain_gpu_{entry}").into()),
                layout: vec![layout()],
                shader: main.shader.clone(),
                entry_point: Some(entry.into()),
                ..ComputePipelineDescriptor::default()
            })
        };
        fill.pipelines = Some([pipeline("fill_eval"), pipeline("fill_pack")]);
        fill.rev = Some(main.rev);
        fill.status.clear();
    }
    let Some(ids) = fill.pipelines else { return };
    let status = match ids.map(|id| cache.get_compute_pipeline_state(id)) {
        [CachedPipelineState::Ok(_), CachedPipelineState::Ok(_)] => "ready",
        [CachedPipelineState::Err(e), _] | [_, CachedPipelineState::Err(e)] => {
            // Not loaded yet (the shader or one of its imports): the cache retries these every frame.
            if !matches!(e, bevy_shader::ShaderCacheError::ShaderNotLoaded(_) | bevy_shader::ShaderCacheError::ShaderImportNotYetAvailable) {
                let line = format!("terrain gpu fill: pipeline error: {e}");
                if fill.status != line {
                    log::error!("{line}");
                    fill.status = line;
                }
            }
            return;
        }
        _ => return,
    };
    let targets = [main.heights, main.normals, main.macro_image];
    if !targets.iter().all(|id| images.get(*id).is_some()) {
        return;
    }
    if fill.status != status {
        log::info!("terrain gpu fill: pipelines ready ({} field words)", main.data.len());
        fill.status = status.into();
    }
    main.ready.store(true, Ordering::Relaxed);
    if fill.pending.is_empty() {
        return;
    }
    let plan = gpu_fill::plan(&std::mem::take(&mut fill.pending));
    let Some(data) = fill.data.clone() else { return };
    let jobs = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("terrain_gpu_fill_jobs"),
        contents: &plan.job_bytes(),
        usage: BufferUsages::STORAGE,
    });
    let scratch = sized(&device, &mut fill.scratch, "terrain_gpu_fill_scratch", plan.scratch_len, BufferUsages::STORAGE);
    let out = sized(&device, &mut fill.out, "terrain_gpu_fill_out", plan.out_len, BufferUsages::STORAGE | BufferUsages::COPY_SRC);
    let group = device.create_bind_group(
        "terrain_gpu_fill",
        &cache.get_bind_group_layout(&layout()),
        &BindGroupEntries::sequential((
            data.as_entire_binding(),
            jobs.as_entire_binding(),
            scratch.as_entire_binding(),
            out.as_entire_binding(),
        )),
    );
    main.counters.pieces.fetch_add(plan.jobs.len() as u64, Ordering::Relaxed);
    main.counters.texels.fetch_add(plan.texels, Ordering::Relaxed);
    main.counters.frames.fetch_add(1, Ordering::Relaxed);
    fill.frame = Some((plan, group));
}

/// The two passes and the copies into the clipmaps, before any view renders (`RenderGraphSystems::Begin`).
pub(crate) fn run(
    mut fill: ResMut<GpuFillRender>,
    cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) {
    let Some((plan, group)) = fill.frame.take() else { return };
    let (Some(main), Some(ids), Some((out, _))) = (fill.main.as_ref(), fill.pipelines, fill.out.as_ref()) else { return };
    let [Some(eval), Some(pack)] = ids.map(|id| cache.get_compute_pipeline(id)) else { return };
    let (Some(heights), Some(normals), Some(macro_image)) =
        (images.get(main.heights), images.get(main.normals), images.get(main.macro_image))
    else {
        return;
    };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    // Two passes: `fill_pack` reads the scratch `fill_eval` wrote.
    for (label, pipeline, threads) in [("terrain_gpu_fill_eval", eval, plan.threads[0]), ("terrain_gpu_fill_pack", pack, plan.threads[1])] {
        let [x, y] = FillPlan::groups(threads);
        if x == 0 {
            continue;
        }
        let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some(label), timestamp_writes: None });
        let span = diagnostics.pass_span(&mut pass, label);
        pass.set_bind_group(0, &group, &[]);
        pass.set_pipeline(pipeline);
        pass.dispatch_workgroups(x, y, 1);
        span.end(&mut pass);
    }
    let encoder = ctx.command_encoder();
    for c in &plan.copies {
        let texture = match c.target {
            FillTarget::Heights => &heights.texture,
            FillTarget::Normals => &normals.texture,
            FillTarget::Macro => &macro_image.texture,
        };
        encoder.copy_buffer_to_texture(
            TexelCopyBufferInfo {
                buffer: out,
                layout: TexelCopyBufferLayout { offset: c.offset, bytes_per_row: Some(c.bytes_per_row), rows_per_image: Some(c.size[1]) },
            },
            TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: Origin3d { x: c.texel[0], y: c.texel[1], z: c.layer },
                aspect: TextureAspect::All,
            },
            Extent3d { width: c.size[0], height: c.size[1], depth_or_array_layers: 1 },
        );
    }
}
