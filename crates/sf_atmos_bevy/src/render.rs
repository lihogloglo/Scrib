//! Render world: pipelines, per-view textures and bind groups, and the
//! Core3d passes (shadow map before the main pass; trace + resolve after
//! the opaque pass; composite after Bevy's `render_sky`, before transparents).

use std::collections::HashMap;

use bevy_app::{App, SubApp};
use bevy_asset::{load_embedded_asset, AssetId, Handle};
use bevy_camera::{MainPassResolutionOverride, Viewport};
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::{Core3d, Core3dSystems};
use bevy_core_pipeline::FullscreenShader;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{InternedSystemSet, IntoSystemSet, NodeId, Schedules, SystemSet};
use bevy_image::Image;
use bevy_math::{DVec2, IVec2, Mat4, UVec2, Vec2, Vec3, Vec4};
use bevy_pbr::resources::{AtmosphereSampler, AtmosphereTextures, AtmosphereTransform, AtmosphereTransforms, AtmosphereTransformsOffset, GpuAtmosphere};
use bevy_pbr::{ExtractedAtmosphere, GpuAtmosphereSettings, GpuLights, GpuScatteringMedium, LightMeta, ScatteringMediumSampler, ShadowSamplers, ViewLightsUniformOffset, ViewShadowBindings};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{sampler, texture_2d, texture_2d_array, texture_2d_multisampled, texture_3d, texture_storage_3d, uniform_buffer};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation, BlendState,
    CachedComputePipelineId, CachedRenderPipelineId, ColorTargetState, ComputePassDescriptor, ComputePipelineDescriptor, StorageTextureAccess, ColorWrites, FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    ShaderType, StoreOp, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, UniformBuffer,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::texture::{CachedTexture, GpuImage, TextureCache};
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms};
use bevy_render::sync_world::RenderEntity;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;

use crate::{CloudCamera, CloudFrame};

pub(crate) fn build(app: &mut App) {
    crate::weather_map::build(app);
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<CloudHistory>()
        .init_resource::<CloudPipelineIds>()
        .init_resource::<FroxelPipelineIds>()
        .init_resource::<CloudFrame>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<CloudPipelines>)
        .add_systems(Render, (queue_pipelines.in_set(RenderSystems::Queue), prepare_views.in_set(RenderSystems::PrepareBindGroups)))
        .add_systems(
            Core3d,
            (
                crate::weather_map::map_pass.after(Core3dSystems::Prepass).before(shadow_pass),
                shadow_pass.after(Core3dSystems::Prepass).before(Core3dSystems::MainPass),
                trace_pass.after(main_opaque_pass_3d).before(main_transparent_pass_3d),
            ),
        );
}

/// Adds the composite after everything that runs between the opaque and the
/// transparent pass, which includes Bevy's `render_sky`.
///
/// Scaffold `SkyOrderScaffold`: `render_sky` is private to `bevy_pbr`, so its
/// system set can't be named. We find the systems ordered before
/// `main_transparent_pass_3d` in the Core3d graph and order the composite
/// after their sets. Removal path: a public `render_sky` set upstream
/// (or Bevy's own cloud hook).
pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    let sets = sets_before_transparent(render_app);
    if sets.is_empty() {
        log::warn!("clouds: render_sky not found in Core3d; the composite may run before the sky");
    }
    let mut config = composite_pass.after(trace_pass).after(main_opaque_pass_3d).before(main_transparent_pass_3d).into_configs();
    for set in sets {
        config = config.after(set);
    }
    render_app.add_systems(Core3d, config);
}

fn sets_before_transparent(render_app: &mut SubApp) -> Vec<InternedSystemSet> {
    let mine = [trace_pass.into_system_set().intern(), shadow_pass.into_system_set().intern()];
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

// -- uniform -------------------------------------------------------------------

/// GPU layout of `Clouds` in `cloud_common.wesl` (field order is the layout).
#[derive(Clone, Copy, Default, ShaderType)]
struct CloudUniform {
    planet_center: Vec3,
    inner_radius: f32,
    ground_albedo: Vec3,
    base: f32,
    wind_step: Vec3,
    thickness: f32,
    origin_shift: Vec3,
    weather_tile: f32,
    weather_off: Vec2,
    shape_off: Vec2,
    detail_off: Vec2,
    shape_tile: f32,
    detail_tile: f32,
    coverage: f32,
    cloud_type: f32,
    density: f32,
    wetness: f32,
    storm: f32,
    erosion: f32,
    extinction: f32,
    ambient: f32,
    ground_bounce: f32,
    powder: f32,
    phase_g: f32,
    ms_falloff: f32,
    far: f32,
    shadow_strength: f32,
    mean_cover: f32,
    min_steps: u32,
    max_steps: u32,
    light_steps: u32,
    ms_octaves: u32,
    frame: u32,
    checkerboard: u32,
    history_valid: u32,
    shadow_steps: u32,
    full_size: Vec2,
    half_size: Vec2,
    trace_size: Vec2,
    shadow_center: Vec3,
    shadow_x: Vec3,
    shadow_y: Vec3,
    light_dir: Vec3,
    prev_clip_from_world: Mat4,
    weather2_off: Vec2,
    cirrus_off: Vec2,
    cirrus_tile: f32,
    cirrus: f32,
    cirrus_height: f32,
    cirrus_depth: f32,
    rise: f32,
    shadow_camera: Vec3,
    fog: FogUniform,
    /// 1: the clouds draw; 0: the node runs for the fog alone.
    clouds_on: u32,
    /// World weather map (WE4b): 0 off (uniform weather), 1 per sample
    /// (High/Ultra), 2 per ray (3 lookups along the in-layer segment).
    map_on: u32,
    map_size: f32,
    map_inv_texel: f32,
    /// Window-relative texel coordinate of the origin-relative zero.
    map_offset: Vec2,
    /// Storage texel of the window's minimum corner (window mod size).
    map_wrap: Vec2,
    /// The envelope shell the trace and shadow map march: the uniform layer,
    /// grown to the map's lowest base and highest top (`WE4b`).
    shell_base: f32,
    shell_thickness: f32,
    /// Most cirrus anywhere (the map's layer C gives the local cover).
    cirrus_max: f32,
    /// 1: march the distant precipitation shafts and far fog by location
    /// (WE4d): the map is sampled per sample (High/Ultra) and some climate
    /// precipitates or has other fog than another.
    far_weather: u32,
    glows: [Vec4; 4],
}

/// GPU layout of `Fog` in `fog.wesl` (weather.md "Fog", F2).
#[derive(Clone, Copy, Default, ShaderType)]
struct FogUniform {
    albedo: Vec3,
    phase_g: f32,
    height_density: f32,
    height_falloff: f32,
    height_base: f32,
    bank_density: f32,
    bank_falloff: f32,
    bank_base: f32,
    bank_coverage: f32,
    bank_sharpness: f32,
    bank_off: Vec2,
    detail_off: Vec2,
    warp_off: Vec2,
    bank_tile: f32,
    detail_tile: f32,
    warp_tile: f32,
    warp_amount: f32,
    ambient: f32,
    far: f32,
    sky_distance: f32,
    steps: u32,
    /// 1: fog draws (0: every fog term is skipped).
    enabled: u32,
}

impl FogUniform {
    fn new(fog: Option<&sf_atmos::FogSettings>, offsets: &sf_atmos::FogOffsets, steps: u32) -> Self {
        let Some(f) = fog else { return Self::default() };
        Self {
            albedo: f.albedo,
            phase_g: f.phase_g,
            height_density: f.height.density,
            height_falloff: f.height.falloff.max(1e-3),
            height_base: f.height.base,
            bank_density: f.banks.layer.density,
            bank_falloff: f.banks.layer.falloff.max(1e-3),
            bank_base: f.banks.layer.base,
            bank_coverage: f.banks.coverage,
            bank_sharpness: f.banks.sharpness,
            bank_off: offsets.bank,
            detail_off: offsets.detail,
            warp_off: offsets.warp,
            bank_tile: f.banks.tile,
            detail_tile: f.banks.detail_tile,
            warp_tile: f.banks.warp_tile,
            warp_amount: f.banks.warp_amount,
            ambient: f.ambient,
            far: f.far,
            sky_distance: f.sky_distance,
            steps,
            enabled: 1,
        }
    }
}

/// GPU layout of `FogLight` in `froxel_common.wesl`.
#[derive(Clone, Copy, Default, ShaderType)]
struct FogLightGpu {
    position_range: Vec4,
    color: Vec4,
}

/// GPU layout of `Froxels` in `froxel_common.wesl` (field order is the layout).
#[derive(Clone, Copy, Default, ShaderType)]
struct FroxelUniform {
    world_from_clip: Mat4,
    prev_clip_from_world: Mat4,
    camera: Vec3,
    exposure: f32,
    prev_camera: Vec3,
    near: f32,
    forward: Vec3,
    far: f32,
    origin_shift: Vec3,
    history: f32,
    size: Vec3,
    frame: u32,
    planet_center: Vec3,
    inner_radius: f32,
    shadow_center: Vec3,
    cloud_shadow: u32,
    shadow_x: Vec3,
    history_valid: u32,
    shadow_y: Vec3,
    n_lights: u32,
    far_map: Vec4,
    jitter: Vec3,
    enabled: u32,
    shadow_light: Vec3,
    /// 1: `FroxelJitter::PerFroxel` (per-froxel rotation, history at the froxel centre).
    jitter_mode: u32,
    fog: FogUniform,
    lights: [FogLightGpu; sf_atmos::MAX_FOG_LIGHTS],
}

// -- pipelines -----------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pass {
    Shadow,
    Trace,
    Resolve,
    Composite,
    FogLight,
}

#[derive(Resource)]
struct CloudPipelines {
    atmosphere: BindGroupLayoutDescriptor,
    atmosphere_msaa: BindGroupLayoutDescriptor,
    density: BindGroupLayoutDescriptor,
    resolve: BindGroupLayoutDescriptor,
    composite: BindGroupLayoutDescriptor,
    fog_light: BindGroupLayoutDescriptor,
    fullscreen: FullscreenShader,
    shaders: [Handle<Shader>; 5],
    froxel_inject: BindGroupLayoutDescriptor,
    froxel_integrate: BindGroupLayoutDescriptor,
    froxel_shaders: [Handle<Shader>; 2],
    noise_sampler: Sampler,
    linear_sampler: Sampler,
}

fn atmosphere_layout(label: &'static str, msaa: bool) -> BindGroupLayoutDescriptor {
    let depth = if msaa { texture_2d_multisampled(TextureSampleType::Depth) } else { texture_2d(TextureSampleType::Depth) };
    BindGroupLayoutDescriptor::new(
        label,
        &BindGroupLayoutEntries::with_indices(
            ShaderStages::FRAGMENT,
            (
                (0, uniform_buffer::<GpuAtmosphere>(true)),
                (1, uniform_buffer::<GpuAtmosphereSettings>(true)),
                (2, uniform_buffer::<AtmosphereTransform>(true)),
                (3, uniform_buffer::<ViewUniform>(true)),
                (4, uniform_buffer::<GpuLights>(true)),
                (5, texture_2d(TextureSampleType::default())),
                (6, texture_2d(TextureSampleType::default())),
                (7, sampler(SamplerBindingType::Filtering)),
                (8, texture_2d(TextureSampleType::default())),
                (9, texture_2d(TextureSampleType::default())),
                (10, texture_2d(TextureSampleType::default())),
                (11, texture_3d(TextureSampleType::default())),
                (12, sampler(SamplerBindingType::Filtering)),
                (13, depth),
            ),
        ),
    )
}

impl FromWorld for CloudPipelines {
    #[allow(clippy::too_many_lines)] // one layout per pass, declarative
    fn from_world(world: &mut World) -> Self {
        let unfiltered = TextureSampleType::Float { filterable: false };
        let device = world.resource::<RenderDevice>();
        let linear = |address| SamplerDescriptor {
            label: Some("clouds"),
            address_mode_u: address,
            address_mode_v: address,
            address_mode_w: address,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..SamplerDescriptor::default()
        };
        let noise_sampler = device.create_sampler(&linear(bevy_render::render_resource::AddressMode::Repeat));
        let linear_sampler = device.create_sampler(&linear(bevy_render::render_resource::AddressMode::ClampToEdge));
        Self {
            atmosphere: atmosphere_layout("clouds_atmosphere", false),
            atmosphere_msaa: atmosphere_layout("clouds_atmosphere_msaa", true),
            density: BindGroupLayoutDescriptor::new(
                "clouds_density",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<CloudUniform>(false),
                        texture_3d(TextureSampleType::default()),
                        texture_3d(TextureSampleType::default()),
                        texture_2d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                        // The world weather map (WE4b), layers A, B, C.
                        texture_2d_array(TextureSampleType::default()),
                    ),
                ),
            ),
            resolve: BindGroupLayoutDescriptor::new(
                "clouds_resolve",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<CloudUniform>(false),
                        texture_2d(TextureSampleType::default()),
                        texture_2d(unfiltered),
                        texture_2d(TextureSampleType::default()),
                        texture_2d(unfiltered),
                        sampler(SamplerBindingType::Filtering),
                    ),
                ),
            ),
            composite: BindGroupLayoutDescriptor::new(
                "clouds_composite",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<CloudUniform>(false),
                        texture_2d(unfiltered),
                        texture_2d(unfiltered),
                        texture_2d(unfiltered),
                        uniform_buffer::<FroxelUniform>(false),
                        texture_3d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                    ),
                ),
            ),
            fog_light: BindGroupLayoutDescriptor::new(
                "clouds_fog_light",
                &BindGroupLayoutEntries::single(ShaderStages::FRAGMENT, uniform_buffer::<CloudUniform>(false)),
            ),
            froxel_inject: BindGroupLayoutDescriptor::new(
                "atmos_froxel_inject",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (
                        uniform_buffer::<FroxelUniform>(false),
                        uniform_buffer::<GpuLights>(true),
                        texture_2d(unfiltered),
                        texture_2d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                        texture_2d(TextureSampleType::default()),
                        texture_2d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                        texture_2d_array(TextureSampleType::Depth),
                        sampler(SamplerBindingType::Comparison),
                        texture_3d(TextureSampleType::default()),
                        texture_storage_3d(FROXEL, StorageTextureAccess::WriteOnly),
                    ),
                ),
            ),
            froxel_integrate: BindGroupLayoutDescriptor::new(
                "atmos_froxel_integrate",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (uniform_buffer::<FroxelUniform>(false), texture_3d(unfiltered), texture_storage_3d(FROXEL, StorageTextureAccess::WriteOnly)),
                ),
            ),
            froxel_shaders: [load_embedded_asset!(world, "shaders/froxel_inject.wesl"), load_embedded_asset!(world, "shaders/froxel_integrate.wesl")],
            fullscreen: world.resource::<FullscreenShader>().clone(),
            shaders: [
                load_embedded_asset!(world, "shaders/cloud_shadow.wesl"),
                load_embedded_asset!(world, "shaders/cloud_trace.wesl"),
                load_embedded_asset!(world, "shaders/cloud_resolve.wesl"),
                load_embedded_asset!(world, "shaders/cloud_composite.wesl"),
                load_embedded_asset!(world, "shaders/fog_light.wesl"),
            ],
            noise_sampler,
            linear_sampler,
        }
    }
}

const HDR: TextureFormat = TextureFormat::Rgba16Float;
/// Froxel volumes (weather.md: RGBA16F scattering/extinction, and the integrated light/transmittance).
const FROXEL: TextureFormat = TextureFormat::Rgba16Float;
/// The fog light texture (`fog_light.wesl`): 64 sky-fill azimuths, the key
/// light's colour and direction. 32-bit: the sun's illuminance overflows f16.
const FOG_AZIMUTHS: u32 = 64;
const FOG_LIGHT: TextureFormat = TextureFormat::Rgba32Float;
/// Bank-march samples per traced pixel (weather.md: the far fog, 16 steps).
const FOG_STEPS: u32 = 16;
/// R: cloud distance; G: the bank modulation of the fog (`fog.wesl`).
const DIST: TextureFormat = TextureFormat::Rg32Float;

impl CloudPipelines {
    fn descriptor(&self, pass: Pass, samples: u32) -> RenderPipelineDescriptor {
        let atmosphere = if samples > 1 { self.atmosphere_msaa.clone() } else { self.atmosphere.clone() };
        let mut shader_defs = Vec::new();
        if samples > 1 {
            shader_defs.push("MULTISAMPLED".into());
        }
        let target = |format| Some(ColorTargetState { format, blend: None, write_mask: ColorWrites::ALL });
        let (layout, entry, targets, sample_count, shader) = match pass {
            Pass::Shadow => (vec![atmosphere, self.density.clone()], "shadow", vec![target(TextureFormat::R8Unorm)], 1, &self.shaders[0]),
            Pass::Trace => (vec![atmosphere, self.density.clone()], "trace", vec![target(HDR), target(DIST)], 1, &self.shaders[1]),
            Pass::Resolve => (vec![atmosphere, self.resolve.clone()], "resolve", vec![target(HDR), target(DIST)], 1, &self.shaders[2]),
            Pass::FogLight => (vec![atmosphere, self.fog_light.clone()], "fog_light", vec![target(FOG_LIGHT)], 1, &self.shaders[4]),
            Pass::Composite => (
                vec![atmosphere, self.composite.clone()],
                "composite",
                vec![Some(ColorTargetState {
                    format: HDR,
                    blend: Some(BlendState {
                        color: BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::SrcAlpha, operation: BlendOperation::Add },
                        alpha: BlendComponent { src_factor: BlendFactor::Zero, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
                samples,
                &self.shaders[3],
            ),
        };
        RenderPipelineDescriptor {
            label: Some(format!("clouds_{entry}_{samples}").into()),
            layout,
            vertex: self.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: shader.clone(),
                shader_defs,
                entry_point: Some(entry.into()),
                targets,
                ..FragmentState::default()
            }),
            multisample: MultisampleState { count: sample_count, ..MultisampleState::default() },
            ..RenderPipelineDescriptor::default()
        }
    }
}

#[derive(Resource, Default)]
struct CloudPipelineIds(HashMap<(Pass, u32), CachedRenderPipelineId>);

/// The froxel compute pipelines (inject, integrate), queued with the first cloud camera.
#[derive(Resource, Default)]
struct FroxelPipelineIds(Option<(CachedComputePipelineId, CachedComputePipelineId)>);

impl CloudPipelines {
    #[allow(clippy::unused_self)] // a method so pipelines read alike
    fn compute(&self, layout: &BindGroupLayoutDescriptor, shader: &Handle<Shader>, entry: &'static str) -> ComputePipelineDescriptor {
        ComputePipelineDescriptor {
            label: Some(format!("atmos_froxel_{entry}").into()),
            layout: vec![layout.clone()],
            shader: shader.clone(),
            entry_point: Some(entry.into()),
            zero_initialize_workgroup_memory: false,
            ..ComputePipelineDescriptor::default()
        }
    }
}

fn queue_pipelines(
    views: Query<&Msaa, With<CloudCamera>>,
    pipelines: Option<Res<CloudPipelines>>,
    cache: Res<PipelineCache>,
    mut ids: ResMut<CloudPipelineIds>,
    mut froxel_ids: ResMut<FroxelPipelineIds>,
) {
    let Some(pipelines) = pipelines else { return };
    if froxel_ids.0.is_none() && !views.is_empty() {
        froxel_ids.0 = Some((
            cache.queue_compute_pipeline(pipelines.compute(&pipelines.froxel_inject, &pipelines.froxel_shaders[0], "inject")),
            cache.queue_compute_pipeline(pipelines.compute(&pipelines.froxel_integrate, &pipelines.froxel_shaders[1], "integrate")),
        ));
    }
    for msaa in &views {
        let samples = msaa.samples();
        for pass in [Pass::Shadow, Pass::Trace, Pass::Resolve, Pass::Composite, Pass::FogLight] {
            ids.0.entry((pass, samples)).or_insert_with(|| cache.queue_render_pipeline(pipelines.descriptor(pass, samples)));
        }
    }
}

// -- per view ------------------------------------------------------------------

struct History {
    prev_clip_from_world: Mat4,
    origin: bevy_math::DVec3,
    frame: u32,
    size: UVec2,
    /// Last frame's camera position and froxel grid (none: no froxel history).
    camera: Vec3,
    froxels: Option<[u32; 3]>,
}

#[derive(Resource, Default)]
struct CloudHistory(HashMap<Entity, History>);

#[derive(Component)]
struct CloudView {
    trace: (CachedTexture, CachedTexture),
    /// The fog light texture and its bind group, when fog draws.
    fog_light: Option<(CachedTexture, BindGroup)>,
    history: (CachedTexture, CachedTexture),
    atmosphere: BindGroup,
    density: BindGroup,
    resolve: BindGroup,
    composite: BindGroup,
    samples: u32,
    shadow_image: Option<AssetId<Image>>,
    /// Rows of the shadow map to render this frame (time slicing).
    /// The froxel passes, when the froxels run.
    froxels: Option<FroxelPass>,
    shadow_rows: (u32, u32),
}

/// The froxel passes' bind groups and dispatch sizes (F3).
struct FroxelPass {
    inject: BindGroup,
    integrate: BindGroup,
    size: [u32; 3],
}

fn cached(cache: &mut TextureCache, device: &RenderDevice, label: &'static str, size: UVec2, format: TextureFormat) -> CachedTexture {
    cache.get(
        device,
        TextureDescriptor {
            label: Some(label),
            size: bevy_render::render_resource::Extent3d { width: size.x, height: size.y, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
    )
}

/// A froxel volume: written by a compute pass, sampled by the next.
fn cached_3d(cache: &mut TextureCache, device: &RenderDevice, label: &'static str, size: [u32; 3]) -> CachedTexture {
    cache.get(
        device,
        TextureDescriptor {
            label: Some(label),
            size: bevy_render::render_resource::Extent3d { width: size[0], height: size[1], depth_or_array_layers: size[2] },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D3,
            format: FROXEL,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
    )
}

/// Per-frame values the froxel uniform shares with the clouds'.
struct FroxelFrame {
    origin_shift: Vec3,
    planet_center: Vec3,
    inner_radius: f32,
    frame: u32,
    /// The far shadow map's image is bound (not a stand-in).
    far_map: bool,
}

/// The froxel uniform for one view; `settings` None: the froxels are off
/// (`enabled` 0, the composite fogs everything analytically). `prev`: last
/// frame's clip-from-world, camera and froxel grid.
#[allow(clippy::cast_precision_loss)]
fn froxel_uniform(frame: &CloudFrame, view: &ExtractedView, exposure: f32, settings: Option<sf_atmos::FroxelSettings>, prev: (Mat4, Vec3, Option<[u32; 3]>), f: FroxelFrame) -> FroxelUniform {
    let s = settings.unwrap_or_default();
    let mut lights = [FogLightGpu::default(); sf_atmos::MAX_FOG_LIGHTS];
    let n_lights = frame.fog_lights.len().min(sf_atmos::MAX_FOG_LIGHTS);
    for (gpu, l) in lights.iter_mut().zip(&frame.fog_lights) {
        *gpu = FogLightGpu { position_range: l[0], color: l[1] };
    }
    let shadow = frame.shadow.filter(|_| frame.clouds);
    FroxelUniform {
        world_from_clip: view.world_from_view.to_matrix() * view.clip_from_view.inverse(),
        prev_clip_from_world: prev.0,
        camera: view.world_from_view.translation(),
        exposure,
        prev_camera: prev.1,
        near: s.near,
        forward: view.world_from_view.forward().as_vec3(),
        far: s.far,
        origin_shift: f.origin_shift,
        history: s.history,
        size: Vec3::new(s.size[0] as f32, s.size[1] as f32, s.size[2] as f32),
        frame: f.frame,
        planet_center: f.planet_center,
        inner_radius: f.inner_radius,
        shadow_center: shadow.map_or(Vec3::ZERO, |b| b.center),
        cloud_shadow: u32::from(shadow.is_some()),
        shadow_x: shadow.map_or(Vec3::X, |b| b.x),
        history_valid: u32::from(settings.is_some() && prev.2 == Some(s.size) && f.frame > 0),
        shadow_y: shadow.map_or(Vec3::Y, |b| b.y),
        #[allow(clippy::cast_possible_truncation)]
        n_lights: n_lights as u32,
        far_map: frame.far_map.as_ref().filter(|_| f.far_map).map_or(Vec4::ZERO, |m| m.1),
        // The sample's place inside its froxel this frame (weather.md "Temporal sampling").
        jitter: Vec3::from_array(sf_atmos::froxel::frame_jitter(s.jitter, f.frame)),
        enabled: u32::from(settings.is_some()),
        // The sun the cloud and terrain shadow maps were made for (none: neither applies).
        shadow_light: frame.shadow.map_or(Vec3::ZERO, |b| b.light_dir),
        jitter_mode: u32::from(s.jitter == sf_atmos::FroxelJitter::PerFroxel),
        fog: FogUniform::new(frame.fog.as_ref(), &frame.fog_offsets, FOG_STEPS),
        lights,
    }
}

/// The atmosphere resources the clouds' group 0 is built from (the same
/// bindings as Bevy's `render_sky`).
type AtmosphereResources<'w> = (
    Res<'w, ComponentUniforms<GpuAtmosphere>>,
    Res<'w, ComponentUniforms<GpuAtmosphereSettings>>,
    Res<'w, AtmosphereTransforms>,
    Res<'w, ViewUniforms>,
    Res<'w, LightMeta>,
    Res<'w, RenderAssets<GpuScatteringMedium>>,
    Res<'w, ScatteringMediumSampler>,
    Res<'w, AtmosphereSampler>,
);

type ViewItem = (
    Entity,
    &'static ExtractedView,
    &'static ExtractedAtmosphere,
    &'static AtmosphereTextures,
    &'static ViewDepthStencilTexture,
    &'static Msaa,
    Option<&'static MainPassResolutionOverride>,
    &'static ExtractedCamera,
    Option<&'static ViewShadowBindings>,
);

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn prepare_views(
    mut commands: Commands,
    views: Query<ViewItem, With<CloudCamera>>,
    frame: Res<CloudFrame>,
    pipelines: Option<Res<CloudPipelines>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut textures: ResMut<TextureCache>,
    mut history: ResMut<CloudHistory>,
    images: Res<RenderAssets<GpuImage>>,
    shadow_samplers: Option<Res<ShadowSamplers>>,
    map: Option<Res<crate::weather_map::WeatherMapGpu>>,
    atmos: AtmosphereResources,
) {
    let (atmosphere_uniforms, settings_uniforms, transforms, view_uniforms, lights, media, medium_sampler, atmosphere_sampler) = atmos;
    let Some(pipelines) = pipelines else { return };
    let Some(noise) = frame.images.noise.as_ref().filter(|_| frame.active) else { return };
    let (Some(shape), Some(detail), Some(weather)) = (images.get(&noise.shape), images.get(&noise.detail), images.get(&noise.weather)) else { return };
    let (Some(a0), Some(a1), Some(a2), Some(a3), Some(a4)) = (
        atmosphere_uniforms.binding(),
        settings_uniforms.binding(),
        transforms.uniforms().binding(),
        view_uniforms.uniforms.binding(),
        lights.view_gpu_lights.binding(),
    ) else {
        return;
    };
    let s = frame.settings;
    let Some(map) = map else { return };
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    let (map_on, map_size, map_inv_texel, map_offset, map_wrap) = match map.active {
        Some(v) => {
            let o = DVec2::new(frame.origin.x, frame.origin.z) / v.texel_m - v.window.as_dvec2();
            let wrap = IVec2::new(v.window.x.rem_euclid(v.size), v.window.y.rem_euclid(v.size));
            (if s.quality.map_per_sample { 1 } else { 2 }, v.size as f32, (1.0 / v.texel_m) as f32, o.as_vec2(), wrap.as_vec2())
        }
        None => (0, 1.0, 0.0, Vec2::ZERO, Vec2::ZERO),
    };
    let mut shell = (s.weather.base, s.weather.base + s.weather.thickness.max(1.0), s.weather.cirrus);
    let mut far_weather = 0;
    if let Some(v) = map.active {
        shell = (shell.0.min(v.envelope[0]), shell.1.max(v.envelope[1]), shell.2.max(v.envelope[2]));
        far_weather = u32::from(map_on == 1 && s.quality.far_weather && v.envelope[3] > 0.5);
    }
    for (entity, view, atmosphere, luts, depth, msaa, resolution, camera, shadow_maps) in &views {
        let Some(medium) = media.get(atmosphere.medium) else { continue };
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else { continue };
        let full = resolution.map_or(UVec2::new(view.viewport.z, view.viewport.w), |r| r.0).max(UVec2::ONE);
        let half = (full + 1) / 2;
        let checker = s.quality.checkerboard;
        let trace_size = if checker { (half + 1) / 2 } else { half };

        let clip_from_world = view.clip_from_view * view.world_from_view.to_matrix().inverse();
        let h = history.0.entry(entity).or_insert(History { prev_clip_from_world: clip_from_world, origin: frame.origin, frame: 0, size: UVec2::ZERO, camera: Vec3::ZERO, froxels: None });
        let valid = h.size == half && h.frame > 0;
        #[allow(clippy::cast_possible_truncation)]
        let origin_shift = (frame.origin - h.origin).as_vec3();
        let frame_index = h.frame;
        let prev = (h.prev_clip_from_world, h.camera, h.froxels);
        let planet_center = atmosphere.world_to_atmosphere.inverse().transform_point3(Vec3::ZERO);
        let shadow = frame.shadow;
        #[allow(clippy::cast_precision_loss)]
        let uniform = CloudUniform {
            planet_center,
            inner_radius: atmosphere.inner_radius,
            ground_albedo: atmosphere.ground_albedo,
            base: s.weather.base,
            wind_step: Vec3::new(frame.wind_step.x, 0.0, frame.wind_step.y),
            thickness: s.weather.thickness.max(1.0),
            origin_shift,
            weather_tile: s.look.weather_tile,
            weather_off: frame.offsets.weather,
            shape_off: frame.offsets.shape,
            detail_off: frame.offsets.detail,
            shape_tile: s.look.shape_tile,
            detail_tile: s.look.detail_tile,
            coverage: s.weather.coverage,
            cloud_type: s.weather.cloud_type,
            density: s.weather.density,
            wetness: s.weather.wetness,
            storm: s.weather.storm,
            erosion: s.weather.erosion,
            extinction: s.look.extinction,
            ambient: s.look.ambient,
            ground_bounce: s.look.ground_bounce,
            powder: s.look.powder,
            phase_g: s.look.phase_g,
            ms_falloff: s.look.ms_falloff,
            far: s.look.far,
            shadow_strength: s.look.shadow_strength,
            mean_cover: frame.mean_cover,
            min_steps: s.quality.min_steps,
            max_steps: s.quality.max_steps.max(s.quality.min_steps),
            light_steps: s.quality.light_steps,
            ms_octaves: s.quality.ms_octaves,
            frame: frame_index,
            checkerboard: u32::from(checker),
            history_valid: u32::from(valid),
            shadow_steps: s.quality.shadow_steps,
            full_size: full.as_vec2(),
            half_size: half.as_vec2(),
            trace_size: trace_size.as_vec2(),
            shadow_center: shadow.map_or(Vec3::ZERO, |b| b.center),
            shadow_x: shadow.map_or(Vec3::X, |b| b.x),
            shadow_y: shadow.map_or(Vec3::Y, |b| b.y),
            light_dir: shadow.map_or(Vec3::Y, |b| b.light_dir),
            prev_clip_from_world: h.prev_clip_from_world,
            weather2_off: frame.offsets.weather2,
            cirrus_off: frame.offsets.cirrus,
            cirrus_tile: s.look.cirrus_tile,
            cirrus: s.weather.cirrus,
            cirrus_height: s.weather.cirrus_height,
            cirrus_depth: s.look.cirrus_depth,
            rise: frame.offsets.rise,
            shadow_camera: shadow.map_or(Vec3::ZERO, |b| b.camera),
            fog: FogUniform::new(frame.fog.as_ref(), &frame.fog_offsets, FOG_STEPS),
            clouds_on: u32::from(frame.clouds),
            map_on,
            map_size,
            map_inv_texel,
            map_offset,
            map_wrap,
            shell_base: shell.0,
            shell_thickness: shell.1 - shell.0,
            cirrus_max: shell.2,
            far_weather,
            glows: frame.glows,
        };
        h.prev_clip_from_world = clip_from_world;
        h.origin = frame.origin;
        h.frame = h.frame.wrapping_add(1);
        h.size = half;
        h.camera = view.world_from_view.translation();

        let mut buffer = UniformBuffer::from(uniform);
        buffer.write_buffer(&device, &queue);
        let Some(uniform_binding) = buffer.binding() else { continue };

        let trace = (cached(&mut textures, &device, "clouds_trace", trace_size, HDR), cached(&mut textures, &device, "clouds_trace_dist", trace_size, DIST));
        let even = frame_index.is_multiple_of(2);
        let (a_color, a_dist, b_color, b_dist) = (
            cached(&mut textures, &device, "clouds_history_a", half, HDR),
            cached(&mut textures, &device, "clouds_history_a_dist", half, DIST),
            cached(&mut textures, &device, "clouds_history_b", half, HDR),
            cached(&mut textures, &device, "clouds_history_b_dist", half, DIST),
        );
        // (write, read) by frame parity.
        let ((w_color, w_dist), (r_color, r_dist)) = if even { ((a_color, a_dist), (b_color, b_dist)) } else { ((b_color, b_dist), (a_color, a_dist)) };

        let samples = msaa.samples();
        let atmosphere_layout = if samples > 1 { &pipelines.atmosphere_msaa } else { &pipelines.atmosphere };
        let atmosphere_group = device.create_bind_group(
            "clouds_atmosphere",
            &cache.get_bind_group_layout(atmosphere_layout),
            &BindGroupEntries::with_indices((
                (0, a0.clone()),
                (1, a1.clone()),
                (2, a2.clone()),
                (3, a3.clone()),
                (4, a4.clone()),
                (5, &medium.density_lut_view),
                (6, &medium.scattering_lut_view),
                (7, medium_sampler.sampler()),
                (8, &luts.transmittance_lut.default_view),
                (9, &luts.multiscattering_lut.default_view),
                (10, &luts.sky_view_lut.default_view),
                (11, &luts.aerial_view_lut.default_view),
                (12, &**atmosphere_sampler),
                (13, depth_view),
            )),
        );
        let density = device.create_bind_group(
            "clouds_density",
            &cache.get_bind_group_layout(&pipelines.density),
            &BindGroupEntries::sequential((uniform_binding.clone(), &shape.texture_view, &detail.texture_view, &weather.texture_view, &pipelines.noise_sampler, &map.view)),
        );
        let resolve = device.create_bind_group(
            "clouds_resolve",
            &cache.get_bind_group_layout(&pipelines.resolve),
            &BindGroupEntries::sequential((
                uniform_binding.clone(),
                &trace.0.default_view,
                &trace.1.default_view,
                &r_color.default_view,
                &r_dist.default_view,
                &pipelines.linear_sampler,
            )),
        );
        let fog_texture = cached(&mut textures, &device, "clouds_fog_light", UVec2::new(FOG_AZIMUTHS + 2, 1), FOG_LIGHT);
        // The froxels (F3): run when fog draws and the sun's shadow maps are bound.
        let cloud_shadow = images.get(&frame.images.shadow);
        let settings = frame.froxels.filter(|_| frame.fog.is_some() && shadow_maps.is_some() && shadow_samplers.is_some() && cloud_shadow.is_some());
        let froxel_uniform = froxel_uniform(&frame, view, camera.exposure, settings, prev, FroxelFrame {
            origin_shift,
            planet_center,
            inner_radius: atmosphere.inner_radius,
            frame: frame_index,
            far_map: frame.far_map.as_ref().is_some_and(|(image, _)| images.get(image).is_some()),
        });
        h.froxels = settings.map(|f| f.size);
        let mut froxel_buffer = UniformBuffer::from(froxel_uniform);
        froxel_buffer.write_buffer(&device, &queue);
        let Some(froxel_binding) = froxel_buffer.binding() else { continue };
        let size = settings.map_or([1, 1, 1], |f| f.size);
        // Integrated into the shared `FroxelVolume` image (transparent materials
        // read it) once it exists at this size; a private texture until then.
        let shared = frame.volume.as_ref().filter(|(_, s)| *s == size).and_then(|(image, _)| images.get(image));
        let private = shared.is_none().then(|| cached_3d(&mut textures, &device, "atmos_froxels_integrated", size));
        let integrated_view = shared.map_or_else(|| &private.as_ref().expect("private volume").default_view, |g| &g.texture_view);
        let froxels = match (settings, shadow_maps, shadow_samplers.as_deref(), cloud_shadow) {
            (Some(_), Some(maps), Some(shadow_samplers), Some(cloud_shadow)) => {
                let (a, b) = (cached_3d(&mut textures, &device, "atmos_froxels_a", size), cached_3d(&mut textures, &device, "atmos_froxels_b", size));
                let (write, read) = if even { (a, b) } else { (b, a) };
                let far_map = frame.far_map.as_ref().and_then(|(image, _)| images.get(image)).map_or(&cloud_shadow.texture_view, |g| &g.texture_view);
                let inject = device.create_bind_group(
                    "atmos_froxel_inject",
                    &cache.get_bind_group_layout(&pipelines.froxel_inject),
                    &BindGroupEntries::sequential((
                        froxel_binding.clone(),
                        a4.clone(),
                        &fog_texture.default_view,
                        &weather.texture_view,
                        &pipelines.noise_sampler,
                        &cloud_shadow.texture_view,
                        far_map,
                        &pipelines.linear_sampler,
                        &maps.directional_light_depth_texture_view,
                        &shadow_samplers.directional_light_comparison_sampler,
                        &read.default_view,
                        &write.default_view,
                    )),
                );
                let integrate = device.create_bind_group(
                    "atmos_froxel_integrate",
                    &cache.get_bind_group_layout(&pipelines.froxel_integrate),
                    &BindGroupEntries::sequential((froxel_binding.clone(), &write.default_view, integrated_view)),
                );
                Some(FroxelPass { inject, integrate, size })
            }
            _ => None,
        };
        let composite = device.create_bind_group(
            "clouds_composite",
            &cache.get_bind_group_layout(&pipelines.composite),
            &BindGroupEntries::sequential((
                uniform_binding.clone(),
                &w_color.default_view,
                &w_dist.default_view,
                &fog_texture.default_view,
                froxel_binding,
                integrated_view,
                &pipelines.linear_sampler,
            )),
        );
        let fog_light = frame.fog.is_some().then(|| {
            let group = device.create_bind_group(
                "clouds_fog_light",
                &cache.get_bind_group_layout(&pipelines.fog_light),
                &BindGroupEntries::single(uniform_binding),
            );
            (fog_texture, group)
        });
        commands.entity(entity).insert(CloudView {
            trace,
            fog_light,
            history: (w_color, w_dist),
            atmosphere: atmosphere_group,
            density,
            resolve,
            composite,
            samples,
            shadow_image: shadow.map(|_| frame.images.shadow.id()),
            shadow_rows: shadow.map_or((0, 0), |b| (b.rows.start, b.rows.count)),
            froxels,
        });
    }
}

// -- passes --------------------------------------------------------------------

type AtmosphereOffsets = (
    &'static DynamicUniformIndex<GpuAtmosphere>,
    &'static DynamicUniformIndex<GpuAtmosphereSettings>,
    &'static AtmosphereTransformsOffset,
    &'static ViewUniformOffset,
    &'static ViewLightsUniformOffset,
);

fn offsets(o: (&DynamicUniformIndex<GpuAtmosphere>, &DynamicUniformIndex<GpuAtmosphereSettings>, &AtmosphereTransformsOffset, &ViewUniformOffset, &ViewLightsUniformOffset)) -> [u32; 5] {
    [o.0.index(), o.1.index(), o.2.index(), o.3.offset, o.4.offset]
}

#[allow(clippy::unnecessary_wraps)] // color attachment slots are options
fn attachment(view: &bevy_render::render_resource::TextureView) -> Option<RenderPassColorAttachment<'_>> {
    Some(RenderPassColorAttachment { view, depth_slice: None, resolve_target: None, ops: Operations { load: LoadOp::Load, store: StoreOp::Store } })
}

/// One fullscreen triangle; `rows` (start, count) limits it to a band of the target.
fn fullscreen(
    ctx: &mut RenderContext,
    label: &'static str,
    targets: &[Option<RenderPassColorAttachment<'_>>],
    pipeline: &bevy_render::render_resource::RenderPipeline,
    groups: [(&BindGroup, &[u32]); 2],
    rows: Option<(u32, u32, u32)>,
) {
    // GPU timer per pass (`render/<label>/elapsed_gpu` under Bevy's `RenderDiagnosticsPlugin`).
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: targets,
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, label);
    if let Some((width, start, count)) = rows {
        pass.set_scissor_rect(0, start, width, count);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, groups[0].0, groups[0].1);
    pass.set_bind_group(1, groups[1].0, groups[1].1);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}

/// One compute dispatch with its GPU timer (`render/<label>/elapsed_gpu`).
pub(crate) fn compute(ctx: &mut RenderContext, label: &'static str, pipeline: &bevy_render::render_resource::ComputePipeline, group: (&BindGroup, &[u32]), groups: [u32; 3]) {
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_compute_pass(&ComputePassDescriptor { label: Some(label), timestamp_writes: None });
    let span = diagnostics.pass_span(&mut pass, label);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, group.0, group.1);
    pass.dispatch_workgroups(groups[0], groups[1], groups[2]);
    span.end(&mut pass);
}

fn shadow_pass(
    view: ViewQuery<(&CloudView, AtmosphereOffsets)>,
    ids: Res<CloudPipelineIds>,
    cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) {
    let (cloud, offs) = view.into_inner();
    let Some(id) = cloud.shadow_image else { return };
    let Some(image) = images.get(id) else { return };
    let Some(pipeline) = ids.0.get(&(Pass::Shadow, cloud.samples)).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    let (start, count) = cloud.shadow_rows;
    let size = image.texture_descriptor.size;
    if count == 0 || start + count > size.height {
        return;
    }
    let offsets = offsets(offs);
    let groups = [(&cloud.atmosphere, &offsets[..]), (&cloud.density, &[][..])];
    fullscreen(&mut ctx, "clouds_shadow", &[attachment(&image.texture_view)], pipeline, groups, Some((size.width, start, count)));
}

fn trace_pass(
    view: ViewQuery<(&CloudView, AtmosphereOffsets)>,
    ids: Res<CloudPipelineIds>,
    froxel_ids: Res<FroxelPipelineIds>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (cloud, offs) = view.into_inner();
    let get = |pass| ids.0.get(&(pass, cloud.samples)).and_then(|id| cache.get_render_pipeline(*id));
    let (Some(trace), Some(resolve)) = (get(Pass::Trace), get(Pass::Resolve)) else { return };
    let offsets = offsets(offs);
    // The fog's per-frame lighting (66 texels) for the composite.
    if let (Some((texture, group)), Some(pipeline)) = (cloud.fog_light.as_ref(), get(Pass::FogLight)) {
        fullscreen(&mut ctx, "clouds_fog_light", &[attachment(&texture.default_view)], pipeline, [(&cloud.atmosphere, &offsets), (group, &[])], None);
    }
    // The froxels (F3), after the fog light they read: inject, then integrate.
    let froxel_pipelines = froxel_ids.0.and_then(|(a, b)| cache.get_compute_pipeline(a).zip(cache.get_compute_pipeline(b)));
    if let (Some(froxels), Some((inject, integrate))) = (cloud.froxels.as_ref(), froxel_pipelines) {
        let [x, y, z] = froxels.size;
        compute(&mut ctx, "atmos_froxel_inject", inject, (&froxels.inject, &offsets[4..5]), [x.div_ceil(8), y.div_ceil(8), z]);
        compute(&mut ctx, "atmos_froxel_integrate", integrate, (&froxels.integrate, &[]), [x.div_ceil(8), y.div_ceil(8), 1]);
    }
    fullscreen(
        &mut ctx,
        "clouds_trace",
        &[attachment(&cloud.trace.0.default_view), attachment(&cloud.trace.1.default_view)],
        trace,
        [(&cloud.atmosphere, &offsets), (&cloud.density, &[])],
        None,
    );
    fullscreen(
        &mut ctx,
        "clouds_resolve",
        &[attachment(&cloud.history.0.default_view), attachment(&cloud.history.1.default_view)],
        resolve,
        [(&cloud.atmosphere, &offsets), (&cloud.resolve, &[])],
        None,
    );
}

fn composite_pass(
    view: ViewQuery<(&CloudView, &ViewTarget, &ExtractedCamera, Option<&MainPassResolutionOverride>, AtmosphereOffsets)>,
    ids: Res<CloudPipelineIds>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (cloud, target, camera, resolution, offs) = view.into_inner();
    let Some(pipeline) = ids.0.get(&(Pass::Composite, cloud.samples)).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    let offsets = offsets(offs);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("clouds_composite"),
        color_attachments: &[Some(target.get_color_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "clouds_composite");
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &cloud.atmosphere, &offsets);
    pass.set_bind_group(1, &cloud.composite, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}


// -- extraction ----------------------------------------------------------------

type CameraItem = (RenderEntity, Has<CloudCamera>);

fn extract(
    mut commands: Commands,
    frame: Extract<Res<CloudFrame>>,
    cameras: Extract<Query<CameraItem, With<bevy_camera::Camera3d>>>,
) {
    commands.insert_resource(frame.clone());
    for (entity, cloud) in &cameras {
        let mut e = commands.entity(entity);
        if cloud && frame.active {
            e.insert(CloudCamera);
        } else {
            e.remove::<(CloudCamera, CloudView)>();
        }
    }
}
