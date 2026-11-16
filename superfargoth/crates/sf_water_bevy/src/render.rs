//! Render world: the FFT compute chain (before the main pass), the snapshot of the opaque scene
//! (after the opaque pass, before the water draws in the transmissive pass, W-ORDER), the SSPR
//! compute of that snapshot (W-SSPR, between the two: `sf_water::sspr`), and the velocity redraw
//! (after the transparent pass, before post-processing).

use std::collections::HashMap;

use bevy_app::App;
use bevy_asset::{load_embedded_asset, AssetId, Handle};
use bevy_camera::{MainPassResolutionOverride, Viewport};
use bevy_core_pipeline::core_3d::{main_transparent_pass_3d, CORE_3D_DEPTH_FORMAT};
use bevy_core_pipeline::prepass::ViewPrepassTextures;
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::IntoSystemSet;
use bevy_image::Image;
use bevy_math::{Mat4, UVec4, Vec4};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{
    sampler, storage_buffer_read_only_sized, storage_buffer_sized, texture_2d, texture_2d_array, texture_depth_2d, texture_depth_2d_multisampled, texture_storage_2d,
    texture_storage_2d_array, uniform_buffer,
};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer, BufferDescriptor, BufferUsages, CachedComputePipelineId, CompareFunction, ComputePipeline, DepthBiasState, DepthStencilState, PrimitiveState, PrimitiveTopology, RenderPipeline, StencilState,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, SamplerBindingType,
    ShaderStages, ShaderType, StorageTextureAccess, StoreOp, TextureFormat, TextureSampleType, TextureViewDescriptor, TextureViewDimension, UniformBuffer,
    VertexState,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::GpuImage;
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms};
use sf_render_order_bevy::PassStage;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;

use crate::{WaterCamera, WaterGate, WaterImages, WaterParams, MAX_CASCADES, RESOLUTION};

/// The FFT chain's uniform (`water_fft.wesl` `FftParams`).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub(crate) struct FftUniform {
    pub sizes: Vec4,
    pub dts: Vec4,
    pub n: u32,
    pub log_n: u32,
    pub skip: u32,
    pub count: u32,
    pub cycles_t: f32,
    pub choppiness: f32,
    /// Weight of the sea state's `h0` layers against a transition's `from` layers (W-WIND; 1 = none).
    pub blend: f32,
    pub foam_decay: f32,
    pub foam_threshold: f32,
    pub foam_gain: f32,
}
/// The SSPR passes' uniform (`water_sspr.wesl` `Sspr`).
#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub(crate) struct SsprUniform {
    /// (plane level, texel scale, reflection width, reflection height).
    plane: Vec4,
}

/// SSPR this frame: the mirrored level (render-space Y) and the texel scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SsprState {
    pub level: f32,
    pub scale: u32,
}

#[derive(Clone, Copy, Debug, Default, ShaderType)]
struct VelocityUniform {
    clip_from_world: Mat4,
    prev_clip_from_world: Mat4,
    camera: Vec4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ImageIds {
    h0: AssetId<Image>,
    displacement: AssetId<Image>,
    prev_displacement: AssetId<Image>,
    slopes: AssetId<Image>,
    snapshot_color: AssetId<Image>,
    snapshot_depth: AssetId<Image>,
    reflection: AssetId<Image>,
    shore: AssetId<Image>,
    shore_bed: AssetId<Image>,
}

impl ImageIds {
    pub(crate) fn new(i: &WaterImages) -> Self {
        Self {
            h0: i.h0.id(),
            displacement: i.displacement.id(),
            prev_displacement: i.prev_displacement.id(),
            slopes: i.slopes.id(),
            snapshot_color: i.snapshot_color.id(),
            snapshot_depth: i.snapshot_depth.id(),
            reflection: i.reflection.id(),
            shore: i.shore.id(),
            shore_bed: i.shore_bed.id(),
        }
    }
}

/// Main world → render world, once per frame.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct WaterRender {
    pub active: bool,
    /// The FFT and mips run this frame (the visibility gate idles them, CW-10a; always without it).
    pub fft_run: bool,
    /// Copy the displacement to `prev_displacement` before the FFT (`gate::FftPlan::copy_before`).
    pub fft_copy_before: bool,
    pub fft: FftUniform,
    /// Mip levels of the wave images, and the cascades (array layers) written.
    pub mips: u32,
    pub layers: u32,
    pub params: WaterParams,
    /// SSPR plane and scale (`None`: off, or no body below the camera).
    pub sspr: Option<SsprState>,
    pub images: Option<ImageIds>,
}

/// Render-world marker: this view draws the water.
#[derive(Component)]
struct WaterView;

#[derive(Component)]
struct WaterViewBindings {
    samples: u32,
    snapshot: BindGroup,
    velocity: BindGroup,
    /// The SSPR bind group and its dispatch size (reflection texels).
    sspr: Option<(BindGroup, [u32; 2])>,
    /// The visibility gate's probe (CW-10a): `None` without the gate, inside a water box, or while
    /// its pipelines compile; the passes then draw and dispatch directly.
    gate: Option<GateBindings>,
}

/// Render-world copy of the camera's [`WaterGate`] (present while the gate is on).
#[derive(Component)]
struct ViewGate(WaterGate);

/// One view's probe this frame: its pipelines, bind groups and buffers.
struct GateBindings {
    probe: RenderPipeline,
    args_pipeline: ComputePipeline,
    probe_group: BindGroup,
    args_group: BindGroup,
    query: wgpu::QuerySet,
    resolve: Buffer,
    args: Buffer,
    boxes: u32,
}

/// One view's persistent probe objects (re-created when the box count outgrows `boxes`).
struct GateGpu {
    query: wgpu::QuerySet,
    resolve: Buffer,
    args: Buffer,
    boxes: Buffer,
    capacity: u64,
}

/// The `args` buffer (u32 offsets ×4 = bytes): snapshot draw, velocity draw, SSPR dispatch (`water_gate.wesl`).
pub(crate) const GATE_ARGS_SNAPSHOT: u64 = 0;
pub(crate) const GATE_ARGS_VELOCITY: u64 = 16;
pub(crate) const GATE_ARGS_SSPR: u64 = 32;
const GATE_ARGS_BYTES: u64 = 48;
/// One box in the probe's storage buffer (`GateBox`: two `vec4<f32>`).
const GATE_BOX_BYTES: u64 = 32;
/// Vertices per box (one triangle strip, `water_gate.wesl` `strip_corner`).
pub(crate) const GATE_STRIP: u32 = 14;

#[derive(Clone, Copy, Debug, Default, ShaderType)]
struct GateCounts {
    counts: UVec4,
}

#[derive(Resource, Default)]
struct WaterGpu {
    buffers: Option<(Buffer, Buffer)>,
    /// SSPR hash (one u32 key per reflection texel) and its size in bytes.
    hash: Option<(Buffer, u64)>,
    fft: Option<BindGroup>,
    /// Per wave image and level: (bind group, dispatch size).
    mips: Vec<(BindGroup, [u32; 3])>,
    /// Last frame's unjittered `clip_from_world` per view (motion vectors).
    prev: HashMap<Entity, Mat4>,
    /// The visibility gate's probe objects per view (CW-10a).
    gates: HashMap<Entity, GateGpu>,
}

#[derive(Resource)]
struct WaterPipelines {
    fft: BindGroupLayoutDescriptor,
    snapshot: [BindGroupLayoutDescriptor; 2],
    velocity: [BindGroupLayoutDescriptor; 2],
    sspr: BindGroupLayoutDescriptor,
    sspr_shader: Handle<Shader>,
    fft_shader: Handle<Shader>,
    snapshot_shader: Handle<Shader>,
    mips: BindGroupLayoutDescriptor,
    mips_shader: Handle<Shader>,
    velocity_shader: Handle<Shader>,
    /// The visibility gate (CW-10a): probe (view, boxes) and arguments (query, args, counts).
    gate_probe: BindGroupLayoutDescriptor,
    gate_args: BindGroupLayoutDescriptor,
    gate_shader: Handle<Shader>,
    fullscreen: FullscreenShader,
}

impl FromWorld for WaterPipelines {
    fn from_world(world: &mut World) -> Self {
        let unfiltered = TextureSampleType::Float { filterable: false };
        let filtered = TextureSampleType::Float { filterable: true };
        let snapshot = |ms: bool| {
            BindGroupLayoutDescriptor::new(
                "water_snapshot",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (texture_2d(unfiltered), if ms { texture_depth_2d_multisampled() } else { texture_depth_2d() }),
                ),
            )
        };
        let velocity = |ms: bool| {
            BindGroupLayoutDescriptor::new(
                "water_velocity",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::VERTEX_FRAGMENT,
                    (
                        uniform_buffer::<WaterParams>(false),
                        uniform_buffer::<VelocityUniform>(false),
                        texture_2d_array(filtered),
                        texture_2d_array(filtered),
                        sampler(SamplerBindingType::Filtering),
                        if ms { texture_depth_2d_multisampled() } else { texture_depth_2d() },
                        texture_2d(filtered),
                        sampler(SamplerBindingType::Filtering),
                        // The bed slope (W-GERSTNER-DEPTH), sampled with the shore sampler.
                        texture_2d(filtered),
                    ),
                ),
            )
        };
        Self {
            fft: BindGroupLayoutDescriptor::new(
                "water_fft",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (
                        uniform_buffer::<FftUniform>(false),
                        texture_2d_array(unfiltered),
                        storage_buffer_sized(false, None),
                        storage_buffer_sized(false, None),
                        texture_storage_2d_array(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly),
                        texture_storage_2d_array(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly),
                    ),
                ),
            ),
            snapshot: [snapshot(false), snapshot(true)],
            sspr: BindGroupLayoutDescriptor::new(
                "water_sspr",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (
                        uniform_buffer::<ViewUniform>(true),
                        uniform_buffer::<SsprUniform>(false),
                        texture_2d(unfiltered),
                        texture_2d(unfiltered),
                        storage_buffer_sized(false, None),
                        texture_storage_2d(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly),
                    ),
                ),
            ),
            sspr_shader: load_embedded_asset!(world, "shaders/water_sspr.wesl"),
            velocity: [velocity(false), velocity(true)],
            fft_shader: load_embedded_asset!(world, "shaders/water_fft.wesl"),
            snapshot_shader: load_embedded_asset!(world, "shaders/water_snapshot.wesl"),
            mips: BindGroupLayoutDescriptor::new(
                "water_mips",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (texture_2d_array(unfiltered), texture_storage_2d_array(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly)),
                ),
            ),
            mips_shader: load_embedded_asset!(world, "shaders/water_mips.wesl"),
            velocity_shader: load_embedded_asset!(world, "shaders/water_velocity.wesl"),
            gate_probe: BindGroupLayoutDescriptor::new(
                "water_gate_probe",
                &BindGroupLayoutEntries::sequential(ShaderStages::VERTEX, (uniform_buffer::<ViewUniform>(true), storage_buffer_read_only_sized(false, None))),
            ),
            gate_args: BindGroupLayoutDescriptor::new(
                "water_gate_args",
                &BindGroupLayoutEntries::with_indices(
                    ShaderStages::COMPUTE,
                    ((2, storage_buffer_read_only_sized(false, None)), (3, storage_buffer_sized(false, None)), (4, uniform_buffer::<GateCounts>(false))),
                ),
            ),
            gate_shader: load_embedded_asset!(world, "shaders/water_gate.wesl"),
            fullscreen: world.resource::<FullscreenShader>().clone(),
        }
    }
}

const FFT_ENTRIES: [&str; 2] = ["fft_rows", "fft_cols"];
pub(crate) const SSPR_ENTRIES: [&str; 2] = ["project", "resolve"];
/// The SSPR passes' workgroup side (`water_sspr.wesl`).
pub(crate) const SSPR_GROUP: u32 = 8;

#[derive(Resource, Default)]
struct WaterPipelineIds {
    fft: Option<[CachedComputePipelineId; 2]>,
    mips: Option<CachedComputePipelineId>,
    sspr: Option<[CachedComputePipelineId; 2]>,
    snapshot: HashMap<u32, CachedRenderPipelineId>,
    velocity: HashMap<u32, CachedRenderPipelineId>,
    gate_probe: HashMap<u32, CachedRenderPipelineId>,
    gate_args: Option<CachedComputePipelineId>,
}

pub(crate) fn build(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<WaterRender>()
        .init_resource::<WaterGpu>()
        .init_resource::<WaterPipelineIds>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<WaterPipelines>)
        .add_systems(Render, (queue_pipelines.in_set(RenderSystems::Queue), prepare.in_set(RenderSystems::PrepareBindGroups)))
        .add_systems(
            Core3d,
            (
                fft_pass.after(Core3dSystems::Prepass).before(Core3dSystems::MainPass),
                velocity_pass.after(main_transparent_pass_3d).before(Core3dSystems::EarlyPostProcess),
            ),
        );
}

/// The snapshot ([`PassStage::Snapshot`]) and the SSPR ([`PassStage::Reflection`]) go after the
/// opaque pass and before the water draws in Bevy's transmissive pass; the sky, clouds and fog
/// run after the water (render-order.md "Frame graph", W-ORDER).
pub(crate) fn finish(app: &mut App) {
    let mine = [fft_pass.into_system_set().intern(), velocity_pass.into_system_set().intern()];
    sf_render_order_bevy::add_pass(app, PassStage::Snapshot, snapshot_pass.into_system_set().intern(), &mine, snapshot_pass);
    // SSPR (W-SSPR): after the snapshot it reads, before the water samples it.
    sf_render_order_bevy::add_pass(app, PassStage::Reflection, sspr_pass.into_system_set().intern(), &mine, sspr_pass);
}

#[allow(clippy::type_complexity)]
fn extract(
    mut commands: Commands,
    state: Extract<Res<WaterRender>>,
    cameras: Extract<Query<(RenderEntity, Has<WaterCamera>, Option<&WaterGate>), With<bevy_camera::Camera3d>>>,
) {
    commands.insert_resource(state.clone());
    for (entity, water, gate) in &cameras {
        let mut e = commands.entity(entity);
        if water && state.active {
            e.insert(WaterView);
        } else {
            e.remove::<(WaterView, WaterViewBindings)>();
        }
        match gate {
            Some(g) if water => e.insert(ViewGate(g.clone())),
            _ => e.remove::<ViewGate>(),
        };
    }
}

impl WaterPipelines {
    #[allow(clippy::unused_self)] // a method so pipelines read alike
    fn compute(&self, layout: &BindGroupLayoutDescriptor, shader: &Handle<Shader>, entry: &'static str) -> ComputePipelineDescriptor {
        ComputePipelineDescriptor {
            label: Some(format!("water_{entry}").into()),
            layout: vec![layout.clone()],
            shader: shader.clone(),
            entry_point: Some(entry.into()),
            zero_initialize_workgroup_memory: false,
            ..ComputePipelineDescriptor::default()
        }
    }

    fn snapshot_descriptor(&self, samples: u32) -> RenderPipelineDescriptor {
        let ms = samples > 1;
        let target = |format| Some(ColorTargetState { format, blend: None, write_mask: ColorWrites::ALL });
        RenderPipelineDescriptor {
            label: Some(format!("water_snapshot_{samples}").into()),
            layout: vec![self.snapshot[usize::from(ms)].clone()],
            vertex: self.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.snapshot_shader.clone(),
                shader_defs: if ms { vec!["MULTISAMPLED".into()] } else { Vec::new() },
                entry_point: Some("snapshot".into()),
                targets: vec![target(TextureFormat::Rgba16Float), target(TextureFormat::R32Float)],
                ..FragmentState::default()
            }),
            ..RenderPipelineDescriptor::default()
        }
    }

    /// The gate's probe: boxes depth-tested (reverse Z, no write) against the view's depth, no colour.
    fn gate_probe_descriptor(&self, samples: u32) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some(format!("water_gate_probe_{samples}").into()),
            layout: vec![self.gate_probe.clone()],
            vertex: VertexState { shader: self.gate_shader.clone(), shader_defs: Vec::new(), entry_point: Some("probe".into()), buffers: Vec::new(), constants: Vec::default() },
            primitive: PrimitiveState { topology: PrimitiveTopology::TriangleStrip, cull_mode: None, ..PrimitiveState::default() },
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: Some(false),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState { count: samples, ..MultisampleState::default() },
            fragment: None,
            ..RenderPipelineDescriptor::default()
        }
    }

    fn velocity_descriptor(&self, samples: u32) -> RenderPipelineDescriptor {
        let ms = samples > 1;
        let defs = if ms { vec!["MULTISAMPLED".into()] } else { Vec::new() };
        RenderPipelineDescriptor {
            label: Some(format!("water_velocity_{samples}").into()),
            layout: vec![self.velocity[usize::from(ms)].clone()],
            vertex: VertexState {
                shader: self.velocity_shader.clone(),
                shader_defs: defs.clone(),
                entry_point: Some("vertex".into()),
                buffers: Vec::new(),
                constants: Vec::default(),
            },
            fragment: Some(FragmentState {
                shader: self.velocity_shader.clone(),
                shader_defs: defs,
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState { format: TextureFormat::Rg16Float, blend: None, write_mask: ColorWrites::ALL })],
                ..FragmentState::default()
            }),
            multisample: MultisampleState { count: samples, ..MultisampleState::default() },
            ..RenderPipelineDescriptor::default()
        }
    }
}

fn queue_pipelines(views: Query<&Msaa, With<WaterView>>, pipelines: Option<Res<WaterPipelines>>, cache: Res<PipelineCache>, mut ids: ResMut<WaterPipelineIds>) {
    let Some(p) = pipelines else { return };
    // Warm-up: the compute pipelines and the single-sample render pipelines are queued at startup,
    // not when a water view first appears, so they have compiled (1-2 s on DX12) by the time the
    // world has loaded and the sea is never flat on arrival.
    if ids.fft.is_none() {
        ids.fft = Some(FFT_ENTRIES.map(|e| cache.queue_compute_pipeline(p.compute(&p.fft, &p.fft_shader, e))));
    }
    if ids.mips.is_none() {
        ids.mips = Some(cache.queue_compute_pipeline(p.compute(&p.mips, &p.mips_shader, "downsample")));
    }
    if ids.sspr.is_none() {
        ids.sspr = Some(SSPR_ENTRIES.map(|e| cache.queue_compute_pipeline(p.compute(&p.sspr, &p.sspr_shader, e))));
    }
    let warm = std::iter::once(1);
    for s in views.iter().map(Msaa::samples).chain(warm) {
        ids.snapshot.entry(s).or_insert_with(|| cache.queue_render_pipeline(p.snapshot_descriptor(s)));
        ids.velocity.entry(s).or_insert_with(|| cache.queue_render_pipeline(p.velocity_descriptor(s)));
        ids.gate_probe.entry(s).or_insert_with(|| cache.queue_render_pipeline(p.gate_probe_descriptor(s)));
    }
    if ids.gate_args.is_none() {
        ids.gate_args = Some(cache.queue_compute_pipeline(p.compute(&p.gate_args, &p.gate_shader, "gate_args")));
    }
}

// FLICKER-1: `OriginRebase` (O_new - O_old, render space) on origin-shift frames; zero otherwise.
type ViewItem<'a> = (Entity, &'a ExtractedView, &'a ViewTarget, &'a ViewDepthStencilTexture, &'a Msaa, Option<&'a bevy_render::view::OriginRebase>, Option<&'a ViewGate>);

#[allow(clippy::too_many_arguments)]
fn prepare(
    mut commands: Commands,
    views: Query<ViewItem, With<WaterView>>,
    state: Res<WaterRender>,
    pipelines: Option<Res<WaterPipelines>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    images: Res<RenderAssets<GpuImage>>,
    view_uniforms: Res<ViewUniforms>,
    mut gpu: ResMut<WaterGpu>,
    pipeline_ids: Res<WaterPipelineIds>,
) {
    gpu.fft = None;
    let Some(pipelines) = pipelines else { return };
    let Some(ids) = state.images.filter(|_| state.active) else { return };
    let (Some(h0), Some(disp), Some(prev), Some(slopes), Some(snap_color), Some(snap_depth)) = (
        images.get(ids.h0),
        images.get(ids.displacement),
        images.get(ids.prev_displacement),
        images.get(ids.slopes),
        images.get(ids.snapshot_color),
        images.get(ids.snapshot_depth),
    ) else {
        return;
    };
    let (Some(shore), Some(shore_bed)) = (images.get(ids.shore), images.get(ids.shore_bed)) else { return };
    let n = RESOLUTION as u64;
    let (spec, foam) = gpu.buffers.get_or_insert_with(|| {
        let buffer = |label, size| device.create_buffer(&BufferDescriptor { label: Some(label), size, usage: BufferUsages::STORAGE, mapped_at_creation: false });
        (buffer("water_spectrum", MAX_CASCADES as u64 * n * n * 32), buffer("water_foam", MAX_CASCADES as u64 * n * n * 4))
    });
    let (spec, foam) = (spec.clone(), foam.clone());
    let mut fft_uniform = UniformBuffer::from(state.fft);
    fft_uniform.write_buffer(&device, &queue);
    let Some(fft_binding) = fft_uniform.binding() else { return };
    // Single-level views: storage bindings take one mip; the mip chain reads level k−1, writes k.
    let level = |t: &GpuImage, mip: u32| {
        t.texture.create_view(&TextureViewDescriptor {
            label: Some("water_wave_level"),
            dimension: Some(TextureViewDimension::D2Array),
            base_mip_level: mip,
            mip_level_count: Some(1),
            ..TextureViewDescriptor::default()
        })
    };
    let (disp0, slopes0) = (level(disp, 0), level(slopes, 0));
    gpu.fft = Some(device.create_bind_group(
        "water_fft",
        &cache.get_bind_group_layout(&pipelines.fft),
        &BindGroupEntries::sequential((fft_binding, &h0.texture_view, spec.as_entire_binding(), foam.as_entire_binding(), &disp0, &slopes0)),
    ));
    gpu.mips.clear();
    let levels = state.mips.min(slopes.texture.mip_level_count());
    for t in [slopes] {
        let size = t.texture.width();
        for mip in 1..levels {
            let group = device.create_bind_group(
                "water_mips",
                &cache.get_bind_group_layout(&pipelines.mips),
                &BindGroupEntries::sequential((&level(t, mip - 1), &level(t, mip))),
            );
            let s = (size >> mip).max(1);
            gpu.mips.push((group, [s.div_ceil(8), s.div_ceil(8), state.layers.max(1)]));
        }
    }
    let mut params = UniformBuffer::from(state.params);
    params.write_buffer(&device, &queue);
    let sspr = sspr_bindings(&state, &pipelines, &cache, &device, &queue, &images, &view_uniforms, &mut gpu, (snap_color, snap_depth));
    let ids_gate = (pipeline_ids.gate_probe.clone(), pipeline_ids.gate_args);
    for (entity, view, target, depth, msaa, rebase, gate) in &views {
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else { continue };
        let ms = usize::from(msaa.samples() > 1);
        let snapshot = device.create_bind_group(
            "water_snapshot",
            &cache.get_bind_group_layout(&pipelines.snapshot[ms]),
            &BindGroupEntries::sequential((target.main_texture_view(), depth_view)),
        );
        let clip_from_world = view.clip_from_view * view.world_from_view.to_matrix().inverse();
        let prev_clip = gpu.prev.insert(entity, clip_from_world).unwrap_or(clip_from_world);
        // Water writes previous = current in this frame's render space; last frame's matrix is in the old
        // one, so shift by the origin delta on a shift frame (FLICKER-1) instead of resetting history.
        let prev_clip = prev_clip * Mat4::from_translation(rebase.map_or(bevy_math::Vec3::ZERO, |r| r.0));
        let mut velocity_uniform =
            UniformBuffer::from(VelocityUniform { clip_from_world, prev_clip_from_world: prev_clip, camera: view.world_from_view.translation().extend(1.0) });
        velocity_uniform.write_buffer(&device, &queue);
        let (Some(params_binding), Some(velocity_binding)) = (params.binding(), velocity_uniform.binding()) else { continue };
        let velocity = device.create_bind_group(
            "water_velocity",
            &cache.get_bind_group_layout(&pipelines.velocity[ms]),
            &BindGroupEntries::sequential((params_binding, velocity_binding, &disp.texture_view, &prev.texture_view, &disp.sampler, depth_view, &shore.texture_view, &shore.sampler, &shore_bed.texture_view)),
        );
        let gate = match gate {
            // The frustum term closed this camera: none of its passes is queued (`prev` stays current).
            Some(g) if !g.0.in_frustum() => {
                commands.entity(entity).remove::<WaterViewBindings>();
                continue;
            }
            Some(g) if !g.0.forced => gate_bindings(entity, &g.0, msaa.samples(), &state, sspr.as_ref().map(|s| s.1), &pipelines, &ids_gate, &cache, &device, &queue, &view_uniforms, &mut gpu),
            _ => None,
        };
        commands.entity(entity).insert(WaterViewBindings { samples: msaa.samples(), snapshot, velocity, sspr: sspr.clone(), gate });
    }
}

/// One view's probe (CW-10a, `gate.rs`): its boxes uploaded, the query and argument buffers, and
/// both bind groups. `None` while a gate pipeline compiles (the passes then run directly).
#[allow(clippy::too_many_arguments)]
fn gate_bindings(
    entity: Entity,
    gate: &WaterGate,
    samples: u32,
    state: &WaterRender,
    sspr_groups: Option<[u32; 2]>,
    pipelines: &WaterPipelines,
    (probe_ids, args_id): &(HashMap<u32, CachedRenderPipelineId>, Option<CachedComputePipelineId>),
    cache: &PipelineCache,
    device: &RenderDevice,
    queue: &RenderQueue,
    view_uniforms: &ViewUniforms,
    gpu: &mut WaterGpu,
) -> Option<GateBindings> {
    let probe = cache.get_render_pipeline(*probe_ids.get(&samples)?)?.clone();
    let args_pipeline = cache.get_compute_pipeline((*args_id)?)?.clone();
    let count = u32::try_from(gate.boxes.len()).ok().filter(|n| *n > 0)?;
    let bytes = u64::from(count) * GATE_BOX_BYTES;
    if gpu.gates.get(&entity).is_none_or(|g| g.capacity < bytes) {
        let capacity = bytes.next_power_of_two().max(64 * GATE_BOX_BYTES);
        let buffer = |label, size, usage| device.create_buffer(&BufferDescriptor { label: Some(label), size, usage, mapped_at_creation: false });
        let query = device.wgpu_device().create_query_set(&wgpu::QuerySetDescriptor { label: Some("water_gate"), ty: wgpu::QueryType::Occlusion, count: 1 });
        gpu.gates.insert(
            entity,
            GateGpu {
                query,
                resolve: buffer("water_gate_resolve", wgpu::QUERY_SIZE.into(), BufferUsages::QUERY_RESOLVE | BufferUsages::STORAGE),
                args: buffer("water_gate_args", GATE_ARGS_BYTES, BufferUsages::STORAGE | BufferUsages::INDIRECT),
                boxes: buffer("water_gate_boxes", capacity, BufferUsages::STORAGE | BufferUsages::COPY_DST),
                capacity,
            },
        );
    }
    let g = gpu.gates.get(&entity)?;
    let data: Vec<u8> = gate.boxes.iter().flat_map(|[lo, hi]| [lo.x, lo.y, lo.z, 1.0, hi.x, hi.y, hi.z, 1.0]).flat_map(f32::to_le_bytes).collect();
    queue.write_buffer(&g.boxes, 0, &data);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (quads, rings) = (state.params.clip.y as u32, state.params.clip.z as u32);
    let [gx, gy] = sspr_groups.unwrap_or([0, 0]);
    let mut counts = UniformBuffer::from(GateCounts { counts: UVec4::new(quads * quads * 6, rings, gx, gy) });
    counts.write_buffer(device, queue);
    let probe_group = device.create_bind_group(
        "water_gate_probe",
        &cache.get_bind_group_layout(&pipelines.gate_probe),
        &BindGroupEntries::sequential((view_uniforms.uniforms.binding()?, g.boxes.as_entire_binding())),
    );
    let args_group = device.create_bind_group(
        "water_gate_args",
        &cache.get_bind_group_layout(&pipelines.gate_args),
        &BindGroupEntries::with_indices(((2, g.resolve.as_entire_binding()), (3, g.args.as_entire_binding()), (4, counts.binding()?))),
    );
    crate::gate::PROBES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Some(GateBindings { probe, args_pipeline, probe_group, args_group, query: g.query.clone(), resolve: g.resolve.clone(), args: g.args.clone(), boxes: count })
}

/// The SSPR bind group (one for every water view: the bindings are the view-independent
/// snapshot, hash and output; the view uniform comes by dynamic offset), and its dispatch size.
#[allow(clippy::too_many_arguments)]
fn sspr_bindings(
    state: &WaterRender,
    pipelines: &WaterPipelines,
    cache: &PipelineCache,
    device: &RenderDevice,
    queue: &RenderQueue,
    images: &RenderAssets<GpuImage>,
    view_uniforms: &ViewUniforms,
    gpu: &mut WaterGpu,
    (color, depth): (&GpuImage, &GpuImage),
) -> Option<(BindGroup, [u32; 2])> {
    let sspr = state.sspr?;
    let out = images.get(state.images?.reflection)?;
    let (w, h) = (out.texture.width(), out.texture.height());
    if w <= 1 && h <= 1 {
        return None;
    }
    let bytes = u64::from(w) * u64::from(h) * 4;
    if gpu.hash.as_ref().is_none_or(|(_, size)| *size != bytes) {
        let buffer = device.create_buffer(&BufferDescriptor { label: Some("water_sspr_hash"), size: bytes, usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, mapped_at_creation: false });
        gpu.hash = Some((buffer, bytes));
    }
    let (hash, _) = gpu.hash.as_ref()?;
    #[allow(clippy::cast_precision_loss)]
    let mut uniform = UniformBuffer::from(SsprUniform { plane: Vec4::new(sspr.level, sspr.scale as f32, w as f32, h as f32) });
    uniform.write_buffer(device, queue);
    let group = device.create_bind_group(
        "water_sspr",
        &cache.get_bind_group_layout(&pipelines.sspr),
        &BindGroupEntries::sequential((view_uniforms.uniforms.binding()?, uniform.binding()?, &depth.texture_view, &color.texture_view, hash.as_entire_binding(), &out.texture_view)),
    );
    Some((group, [w.div_ceil(SSPR_GROUP), h.div_ceil(SSPR_GROUP)]))
}

/// SSPR (water.md "Water pass" step 2, `water_sspr.wesl`): clear the hash, project the snapshot's
/// pixels above the plane to their mirror texels, resolve colour + confidence. GPU-timed as
/// `render/water_sspr/elapsed_gpu`.
fn sspr_pass(view: ViewQuery<(&WaterViewBindings, &ViewUniformOffset)>, gpu: Res<WaterGpu>, ids: Res<WaterPipelineIds>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let (bindings, offset) = view.into_inner();
    let Some((group, groups)) = bindings.sspr.as_ref() else { return };
    let Some((hash, _)) = gpu.hash.as_ref() else { return };
    let Some(pipelines) = ids.sspr.and_then(|ids| {
        let p: Option<Vec<_>> = ids.iter().map(|id| cache.get_compute_pipeline(*id)).collect();
        p
    }) else {
        return;
    };
    // 0 = empty (keys are inverted heights, `sf_water::sspr::height_key`).
    ctx.command_encoder().clear_buffer(hash, 0, None);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("water_sspr"), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, "water_sspr");
    pass.set_bind_group(0, group, &[offset.offset]);
    // Each dispatch is its own usage scope: the resolve sees every projection's atomics.
    for pipeline in &pipelines {
        pass.set_pipeline(pipeline);
        match &bindings.gate {
            Some(g) => pass.dispatch_workgroups_indirect(&g.args, GATE_ARGS_SSPR),
            None => pass.dispatch_workgroups(groups[0], groups[1], 1),
        }
    }
    span.end(&mut pass);
}

/// GPU-timed compute pass (`render/<label>/elapsed_gpu`).
#[allow(clippy::cast_possible_truncation)]
fn fft_pass(_view: ViewQuery<&WaterView>, gpu: Res<WaterGpu>, state: Res<WaterRender>, ids: Res<WaterPipelineIds>, cache: Res<PipelineCache>, images: Res<RenderAssets<GpuImage>>, mut ctx: RenderContext) {
    // The visibility gate idles the FFT while no camera sees water (CW-10a; always runs without it).
    if !state.fft_run {
        return;
    }
    let Some(group) = gpu.fft.as_ref() else { return };
    let Some(pipelines) = ids.fft.and_then(|ids| {
        let p: Option<Vec<_>> = ids.iter().map(|id| cache.get_compute_pipeline(*id)).collect();
        p
    }) else {
        return;
    };
    let Some(image_ids) = state.images else { return };
    let (Some(disp), Some(prev)) = (images.get(image_ids.displacement), images.get(image_ids.prev_displacement)) else { return };
    // Last frame's displacement (level 0), for the motion vectors: what was drawn, so an
    // amortised cascade that is not re-evaluated this frame correctly has no motion.
    let n = state.fft.n;
    if disp.texture.width() != n || prev.texture.width() != n {
        return;
    }
    let extent = Extent3d { width: n, height: n, depth_or_array_layers: MAX_CASCADES as u32 };
    // After a frame without FFT (the gate idled it) `prev` would be older than a frame: copy after
    // the FFT instead, so the first frame back has no wave motion rather than a too-long one.
    if state.fft_copy_before {
        ctx.command_encoder().copy_texture_to_texture(disp.texture.as_image_copy(), prev.texture.as_image_copy(), extent);
    }
    FFT_FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let cascades = state.fft.count;
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("water_fft"), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, "water_fft");
    if cascades > 0 {
        pass.set_bind_group(0, group, &[]);
        for pipeline in &pipelines {
            pass.set_pipeline(pipeline);
            pass.dispatch_workgroups(n, cascades, 1);
        }
    }
    span.end(&mut pass);
    drop(pass);
    if !state.fft_copy_before {
        ctx.command_encoder().copy_texture_to_texture(disp.texture.as_image_copy(), prev.texture.as_image_copy(), extent);
    }
    // Mip chains of both wave images (level k from k−1).
    if let Some(mips) = ids.mips.and_then(|id| cache.get_compute_pipeline(id)) {
        let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("water_mips"), timestamp_writes: None });
        let span = diagnostics.pass_span(&mut pass, "water_mips");
        pass.set_pipeline(mips);
        for (group, size) in &gpu.mips {
            pass.set_bind_group(0, group, &[]);
            pass.dispatch_workgroups(size[0], size[1], size[2]);
        }
        span.end(&mut pass);
    }
}

fn snapshot_pass(
    view: ViewQuery<(&WaterViewBindings, &ExtractedCamera, Option<&MainPassResolutionOverride>, &ViewDepthStencilTexture, &ViewUniformOffset)>,
    state: Res<WaterRender>,
    ids: Res<WaterPipelineIds>,
    cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) {
    let (bindings, camera, resolution, view_depth, offset) = view.into_inner();
    // The gate's probe first: the SSPR and velocity passes read the arguments it writes.
    if let Some(g) = bindings.gate.as_ref() {
        gate_probe(&mut ctx, g, view_depth, offset, camera, resolution);
    }
    let Some(image_ids) = state.images else { return };
    let (Some(color), Some(depth)) = (images.get(image_ids.snapshot_color), images.get(image_ids.snapshot_depth)) else { return };
    let Some(pipeline) = ids.snapshot.get(&bindings.samples).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    #[allow(clippy::default_trait_access)]
    let load = if camera.viewport.is_none() {
        LoadOp::Clear(Default::default())
    } else {
        LoadOp::Load
    };
    let attach = |view| Some(RenderPassColorAttachment { view, depth_slice: None, resolve_target: None, ops: Operations { load, store: StoreOp::Store } });
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("water_snapshot"),
        color_attachments: &[attach(&color.texture_view), attach(&depth.texture_view)],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "water_snapshot");
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bindings.snapshot, &[]);
    match &bindings.gate {
        Some(g) => pass.draw_indirect(&g.args, GATE_ARGS_SNAPSHOT),
        None => pass.draw(0..3, 0..1),
    }
    span.end(&mut pass);
}

/// The gate's probe (CW-10a, `water_gate.wesl`): the view's water boxes depth-tested against its
/// depth after the opaque pass in an occlusion query, the query resolved on the GPU and turned into
/// the indirect arguments of the snapshot, SSPR and velocity passes. GPU-timed as `water_gate`.
fn gate_probe(ctx: &mut RenderContext, g: &GateBindings, depth: &ViewDepthStencilTexture, offset: &ViewUniformOffset, camera: &ExtractedCamera, resolution: Option<&MainPassResolutionOverride>) {
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("water_gate"),
        color_attachments: &[],
        depth_stencil_attachment: Some(depth.get_attachment(StoreOp::Store)),
        timestamp_writes: None,
        occlusion_query_set: Some(&g.query),
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "water_gate");
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
    }
    pass.set_pipeline(&g.probe);
    pass.set_bind_group(0, &g.probe_group, &[offset.offset]);
    pass.begin_occlusion_query(0);
    pass.draw(0..GATE_STRIP, 0..g.boxes);
    pass.end_occlusion_query();
    span.end(&mut pass);
    drop(pass);
    let encoder = ctx.command_encoder();
    encoder.resolve_query_set(&g.query, 0..1, &g.resolve, 0);
    let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: Some("water_gate_args"), timestamp_writes: None });
    pass.set_pipeline(&g.args_pipeline);
    pass.set_bind_group(0, &g.args_group, &[]);
    pass.dispatch_workgroups(1, 1, 1);
}

fn velocity_pass(
    view: ViewQuery<(&WaterViewBindings, &ExtractedCamera, Option<&MainPassResolutionOverride>, Option<&ViewPrepassTextures>)>,
    state: Res<WaterRender>,
    ids: Res<WaterPipelineIds>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (bindings, camera, resolution, prepass) = view.into_inner();
    let Some(motion) = prepass.and_then(|p| p.motion_vectors.as_ref()) else { return };
    let Some(pipeline) = ids.velocity.get(&bindings.samples).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (quads, rings) = (state.params.clip.y as u32, state.params.clip.z as u32);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("water_velocity"),
        color_attachments: &[Some(motion.get_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "water_velocity");
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bindings.velocity, &[]);
    match &bindings.gate {
        Some(g) => pass.draw_indirect(&g.args, GATE_ARGS_VELOCITY),
        None => pass.draw(0..quads * quads * 6, 0..rings),
    }
    span.end(&mut pass);
}

/// FFT dispatches so far (diagnostics; the golden waits for the pipelines to compile).
pub(crate) static FFT_FRAMES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
