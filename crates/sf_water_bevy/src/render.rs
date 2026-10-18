//! Render world: the FFT compute chain (before the main pass), the snapshot (after the sky and
//! the `sf_atmos` composite, before the transparent pass) with the aerial-view LUT copy, and the
//! velocity redraw (after the transparent pass, before post-processing).

use std::collections::HashMap;

use bevy_app::{App, SubApp};
use bevy_asset::{load_embedded_asset, AssetId, Handle};
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::prepass::ViewPrepassTextures;
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{InternedSystemSet, IntoSystemSet, NodeId, Schedules};
use bevy_image::Image;
use bevy_math::{Mat4, Vec4};
use bevy_pbr::resources::AtmosphereTextures;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{
    sampler, storage_buffer_sized, texture_2d, texture_2d_array, texture_3d, texture_depth_2d, texture_depth_2d_multisampled, texture_storage_2d_array,
    texture_storage_3d, uniform_buffer,
};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, Buffer, BufferDescriptor, BufferUsages, CachedComputePipelineId,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, SamplerBindingType,
    ShaderStages, ShaderType, StorageTextureAccess, StoreOp, TextureFormat, TextureSampleType, TextureViewDescriptor, TextureViewDimension, UniformBuffer,
    VertexState,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::GpuImage;
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture, ViewTarget};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;

use crate::{WaterCamera, WaterImages, WaterParams, MAX_CASCADES, RESOLUTION};

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
    pub foam_decay: f32,
    pub foam_threshold: f32,
    pub foam_gain: f32,
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
    aerial: AssetId<Image>,
    shore: AssetId<Image>,
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
            aerial: i.aerial.id(),
            shore: i.shore.id(),
        }
    }
}

/// Main world → render world, once per frame.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct WaterRender {
    pub active: bool,
    pub fft: FftUniform,
    /// Mip levels of the wave images, and the cascades (array layers) written.
    pub mips: u32,
    pub layers: u32,
    pub params: WaterParams,
    pub images: Option<ImageIds>,
}

/// Render-world marker: this view draws the water.
#[derive(Component)]
struct WaterView;

#[derive(Component)]
struct WaterViewBindings {
    samples: u32,
    snapshot: BindGroup,
    aerial: Option<(BindGroup, [u32; 3])>,
    velocity: BindGroup,
}

#[derive(Resource, Default)]
struct WaterGpu {
    buffers: Option<(Buffer, Buffer)>,
    fft: Option<BindGroup>,
    /// Per wave image and level: (bind group, dispatch size).
    mips: Vec<(BindGroup, [u32; 3])>,
    /// Last frame's unjittered `clip_from_world` per view (motion vectors).
    prev: HashMap<Entity, Mat4>,
}

#[derive(Resource)]
struct WaterPipelines {
    fft: BindGroupLayoutDescriptor,
    snapshot: [BindGroupLayoutDescriptor; 2],
    aerial: BindGroupLayoutDescriptor,
    velocity: [BindGroupLayoutDescriptor; 2],
    fft_shader: Handle<Shader>,
    snapshot_shader: Handle<Shader>,
    aerial_shader: Handle<Shader>,
    mips: BindGroupLayoutDescriptor,
    mips_shader: Handle<Shader>,
    velocity_shader: Handle<Shader>,
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
            aerial: BindGroupLayoutDescriptor::new(
                "water_aerial",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (texture_3d(unfiltered), texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly)),
                ),
            ),
            velocity: [velocity(false), velocity(true)],
            fft_shader: load_embedded_asset!(world, "shaders/water_fft.wesl"),
            snapshot_shader: load_embedded_asset!(world, "shaders/water_snapshot.wesl"),
            aerial_shader: load_embedded_asset!(world, "shaders/water_aerial.wesl"),
            mips: BindGroupLayoutDescriptor::new(
                "water_mips",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (texture_2d_array(unfiltered), texture_storage_2d_array(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly)),
                ),
            ),
            mips_shader: load_embedded_asset!(world, "shaders/water_mips.wesl"),
            velocity_shader: load_embedded_asset!(world, "shaders/water_velocity.wesl"),
            fullscreen: world.resource::<FullscreenShader>().clone(),
        }
    }
}

const FFT_ENTRIES: [&str; 2] = ["fft_rows", "fft_cols"];

#[derive(Resource, Default)]
struct WaterPipelineIds {
    fft: Option<[CachedComputePipelineId; 2]>,
    mips: Option<CachedComputePipelineId>,
    aerial: Option<CachedComputePipelineId>,
    snapshot: HashMap<u32, CachedRenderPipelineId>,
    velocity: HashMap<u32, CachedRenderPipelineId>,
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

/// The snapshot goes after everything ordered before the transparent pass (Bevy's
/// `render_sky`, the `sf_atmos` cloud/fog composite), as `sf_atmos_bevy` orders its composite.
/// Add `WaterPlugin` after the atmosphere plugins so their systems exist here.
pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    let sets = sets_before_transparent(render_app);
    let mut config = snapshot_pass.after(main_opaque_pass_3d).before(main_transparent_pass_3d).into_configs();
    for set in sets {
        config = config.after(set);
    }
    render_app.add_systems(Core3d, config);
}

fn sets_before_transparent(render_app: &mut SubApp) -> Vec<InternedSystemSet> {
    let mine = [fft_pass.into_system_set().intern(), velocity_pass.into_system_set().intern()];
    let transparent = main_transparent_pass_3d.into_system_set().intern();
    let world = render_app.world_mut();
    let schedules = world.resource::<Schedules>();
    let Some(schedule) = schedules.get(Core3d) else { return Vec::new() };
    let graph = schedule.graph();
    let Some(key) = graph.system_sets.get_key(transparent) else { return Vec::new() };
    let systems: HashMap<_, _> = graph.systems.iter().map(|(k, system, _)| (k, system)).collect();
    let mut out = Vec::new();
    for node in graph.dependency().graph().neighbors_directed(NodeId::Set(key), bevy_ecs::schedule::graph::Direction::Incoming) {
        if let NodeId::System(k) = node
            && let Some(system) = systems.get(&k)
        {
            for set in system.default_system_sets() {
                if !mine.contains(&set) && set != transparent {
                    out.push(set);
                }
            }
        }
    }
    out
}

#[allow(clippy::type_complexity)]
fn extract(mut commands: Commands, state: Extract<Res<WaterRender>>, cameras: Extract<Query<(RenderEntity, Has<WaterCamera>), With<bevy_camera::Camera3d>>>) {
    commands.insert_resource(state.clone());
    for (entity, water) in &cameras {
        let mut e = commands.entity(entity);
        if water && state.active {
            e.insert(WaterView);
        } else {
            e.remove::<(WaterView, WaterViewBindings)>();
        }
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
    if ids.aerial.is_none() {
        ids.aerial = Some(cache.queue_compute_pipeline(p.compute(&p.aerial, &p.aerial_shader, "copy_aerial")));
    }
    let warm = std::iter::once(1);
    for s in views.iter().map(Msaa::samples).chain(warm) {
        ids.snapshot.entry(s).or_insert_with(|| cache.queue_render_pipeline(p.snapshot_descriptor(s)));
        ids.velocity.entry(s).or_insert_with(|| cache.queue_render_pipeline(p.velocity_descriptor(s)));
    }
}

type ViewItem<'a> = (Entity, &'a ExtractedView, &'a ViewTarget, &'a ViewDepthStencilTexture, &'a Msaa, Option<&'a AtmosphereTextures>);

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
    mut gpu: ResMut<WaterGpu>,
) {
    gpu.fft = None;
    let Some(pipelines) = pipelines else { return };
    let Some(ids) = state.images.filter(|_| state.active) else { return };
    let (Some(h0), Some(disp), Some(prev), Some(slopes), Some(snap_color), Some(snap_depth), Some(aerial)) = (
        images.get(ids.h0),
        images.get(ids.displacement),
        images.get(ids.prev_displacement),
        images.get(ids.slopes),
        images.get(ids.snapshot_color),
        images.get(ids.snapshot_depth),
        images.get(ids.aerial),
    ) else {
        return;
    };
    let _ = (snap_color, snap_depth);
    let Some(shore) = images.get(ids.shore) else { return };
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
    for (entity, view, target, depth, msaa, atmosphere) in &views {
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else { continue };
        let ms = usize::from(msaa.samples() > 1);
        let snapshot = device.create_bind_group(
            "water_snapshot",
            &cache.get_bind_group_layout(&pipelines.snapshot[ms]),
            &BindGroupEntries::sequential((target.main_texture_view(), depth_view)),
        );
        let aerial_group = atmosphere.and_then(|a| {
            let size = a.aerial_view_lut.texture.size();
            (size == aerial.texture.size()).then(|| {
                (
                    device.create_bind_group(
                        "water_aerial",
                        &cache.get_bind_group_layout(&pipelines.aerial),
                        &BindGroupEntries::sequential((&a.aerial_view_lut.default_view, &aerial.texture_view)),
                    ),
                    [size.width, size.height, size.depth_or_array_layers],
                )
            })
        });
        let clip_from_world = view.clip_from_view * view.world_from_view.to_matrix().inverse();
        let prev_clip = gpu.prev.insert(entity, clip_from_world).unwrap_or(clip_from_world);
        let mut velocity_uniform =
            UniformBuffer::from(VelocityUniform { clip_from_world, prev_clip_from_world: prev_clip, camera: view.world_from_view.translation().extend(1.0) });
        velocity_uniform.write_buffer(&device, &queue);
        let (Some(params_binding), Some(velocity_binding)) = (params.binding(), velocity_uniform.binding()) else { continue };
        let velocity = device.create_bind_group(
            "water_velocity",
            &cache.get_bind_group_layout(&pipelines.velocity[ms]),
            &BindGroupEntries::sequential((params_binding, velocity_binding, &disp.texture_view, &prev.texture_view, &disp.sampler, depth_view, &shore.texture_view, &shore.sampler)),
        );
        commands.entity(entity).insert(WaterViewBindings { samples: msaa.samples(), snapshot, aerial: aerial_group, velocity });
    }
}

/// GPU-timed compute pass (`render/<label>/elapsed_gpu`).
#[allow(clippy::cast_possible_truncation)]
fn fft_pass(_view: ViewQuery<&WaterView>, gpu: Res<WaterGpu>, state: Res<WaterRender>, ids: Res<WaterPipelineIds>, cache: Res<PipelineCache>, images: Res<RenderAssets<GpuImage>>, mut ctx: RenderContext) {
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
    ctx.command_encoder().copy_texture_to_texture(
        disp.texture.as_image_copy(),
        prev.texture.as_image_copy(),
        Extent3d { width: n, height: n, depth_or_array_layers: MAX_CASCADES as u32 },
    );
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
    view: ViewQuery<&WaterViewBindings>,
    state: Res<WaterRender>,
    ids: Res<WaterPipelineIds>,
    cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) {
    let bindings = view.into_inner();
    let Some(image_ids) = state.images else { return };
    let (Some(color), Some(depth)) = (images.get(image_ids.snapshot_color), images.get(image_ids.snapshot_depth)) else { return };
    let Some(pipeline) = ids.snapshot.get(&bindings.samples).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    if let (Some((group, size)), Some(copy)) = (bindings.aerial.as_ref(), ids.aerial.and_then(|id| cache.get_compute_pipeline(id))) {
        let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some("water_aerial"), timestamp_writes: None });
        pass.set_pipeline(copy);
        pass.set_bind_group(0, group, &[]);
        pass.dispatch_workgroups(size[0].div_ceil(4), size[1].div_ceil(4), size[2].div_ceil(4));
    }
    let attach = |view| Some(RenderPassColorAttachment { view, depth_slice: None, resolve_target: None, ops: Operations { load: LoadOp::Load, store: StoreOp::Store } });
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("water_snapshot"),
        color_attachments: &[attach(&color.texture_view), attach(&depth.texture_view)],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "water_snapshot");
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bindings.snapshot, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}

fn velocity_pass(view: ViewQuery<(&WaterViewBindings, Option<&ViewPrepassTextures>)>, state: Res<WaterRender>, ids: Res<WaterPipelineIds>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let (bindings, prepass) = view.into_inner();
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
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bindings.velocity, &[]);
    pass.draw(0..quads * quads * 6, 0..rings);
    span.end(&mut pass);
}

/// FFT dispatches so far (diagnostics; the golden waits for the pipelines to compile).
pub(crate) static FFT_FRAMES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
