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
use bevy_math::{Mat4, UVec2, Vec2, Vec3};
use bevy_pbr::resources::{AtmosphereSampler, AtmosphereTextures, AtmosphereTransform, AtmosphereTransforms, AtmosphereTransformsOffset, GpuAtmosphere};
use bevy_pbr::{ExtractedAtmosphere, GpuAtmosphereSettings, GpuLights, GpuScatteringMedium, LightMeta, ScatteringMediumSampler, ViewLightsUniformOffset};
use bevy_render::camera::ExtractedCamera;
use bevy_render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{sampler, texture_2d, texture_2d_multisampled, texture_3d, uniform_buffer};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation, BlendState,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache,
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
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<CloudHistory>()
        .init_resource::<CloudPipelineIds>()
        .init_resource::<CloudFrame>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<CloudPipelines>)
        .add_systems(Render, (queue_pipelines.in_set(RenderSystems::Queue), prepare_views.in_set(RenderSystems::PrepareBindGroups)))
        .add_systems(
            Core3d,
            (
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
}

// -- pipelines -----------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Pass {
    Shadow,
    Trace,
    Resolve,
    Composite,
}

#[derive(Resource)]
struct CloudPipelines {
    atmosphere: BindGroupLayoutDescriptor,
    atmosphere_msaa: BindGroupLayoutDescriptor,
    density: BindGroupLayoutDescriptor,
    resolve: BindGroupLayoutDescriptor,
    composite: BindGroupLayoutDescriptor,
    fullscreen: FullscreenShader,
    shaders: [Handle<Shader>; 4],
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
                    (uniform_buffer::<CloudUniform>(false), texture_2d(unfiltered), texture_2d(unfiltered)),
                ),
            ),
            fullscreen: world.resource::<FullscreenShader>().clone(),
            shaders: [
                load_embedded_asset!(world, "shaders/cloud_shadow.wesl"),
                load_embedded_asset!(world, "shaders/cloud_trace.wesl"),
                load_embedded_asset!(world, "shaders/cloud_resolve.wesl"),
                load_embedded_asset!(world, "shaders/cloud_composite.wesl"),
            ],
            noise_sampler,
            linear_sampler,
        }
    }
}

const HDR: TextureFormat = TextureFormat::Rgba16Float;
const DIST: TextureFormat = TextureFormat::R32Float;

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

fn queue_pipelines(
    views: Query<&Msaa, With<CloudCamera>>,
    pipelines: Option<Res<CloudPipelines>>,
    cache: Res<PipelineCache>,
    mut ids: ResMut<CloudPipelineIds>,
) {
    let Some(pipelines) = pipelines else { return };
    for msaa in &views {
        let samples = msaa.samples();
        for pass in [Pass::Shadow, Pass::Trace, Pass::Resolve, Pass::Composite] {
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
}

#[derive(Resource, Default)]
struct CloudHistory(HashMap<Entity, History>);

#[derive(Component)]
struct CloudView {
    trace: (CachedTexture, CachedTexture),
    history: (CachedTexture, CachedTexture),
    atmosphere: BindGroup,
    density: BindGroup,
    resolve: BindGroup,
    composite: BindGroup,
    samples: u32,
    shadow_image: Option<AssetId<Image>>,
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
    for (entity, view, atmosphere, luts, depth, msaa, resolution) in &views {
        let Some(medium) = media.get(atmosphere.medium) else { continue };
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else { continue };
        let full = resolution.map_or(UVec2::new(view.viewport.z, view.viewport.w), |r| r.0).max(UVec2::ONE);
        let half = (full + 1) / 2;
        let checker = s.quality.checkerboard;
        let trace_size = if checker { (half + 1) / 2 } else { half };

        let clip_from_world = view.clip_from_view * view.world_from_view.to_matrix().inverse();
        let h = history.0.entry(entity).or_insert(History { prev_clip_from_world: clip_from_world, origin: frame.origin, frame: 0, size: UVec2::ZERO });
        let valid = h.size == half && h.frame > 0;
        #[allow(clippy::cast_possible_truncation)]
        let origin_shift = (frame.origin - h.origin).as_vec3();
        let frame_index = h.frame;
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
        };
        h.prev_clip_from_world = clip_from_world;
        h.origin = frame.origin;
        h.frame = h.frame.wrapping_add(1);
        h.size = half;

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
            &BindGroupEntries::sequential((uniform_binding.clone(), &shape.texture_view, &detail.texture_view, &weather.texture_view, &pipelines.noise_sampler)),
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
        let composite = device.create_bind_group(
            "clouds_composite",
            &cache.get_bind_group_layout(&pipelines.composite),
            &BindGroupEntries::sequential((uniform_binding, &w_color.default_view, &w_dist.default_view)),
        );
        commands.entity(entity).insert(CloudView {
            trace,
            history: (w_color, w_dist),
            atmosphere: atmosphere_group,
            density,
            resolve,
            composite,
            samples,
            shadow_image: shadow.map(|_| frame.images.shadow.id()),
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

fn fullscreen(ctx: &mut RenderContext, label: &'static str, targets: &[Option<RenderPassColorAttachment<'_>>], pipeline: &bevy_render::render_resource::RenderPipeline, groups: [(&BindGroup, &[u32]); 2]) {
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: targets,
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, groups[0].0, groups[0].1);
    pass.set_bind_group(1, groups[1].0, groups[1].1);
    pass.draw(0..3, 0..1);
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
    let offsets = offsets(offs);
    fullscreen(&mut ctx, "clouds_shadow", &[attachment(&image.texture_view)], pipeline, [(&cloud.atmosphere, &offsets), (&cloud.density, &[])]);
}

fn trace_pass(view: ViewQuery<(&CloudView, AtmosphereOffsets)>, ids: Res<CloudPipelineIds>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let (cloud, offs) = view.into_inner();
    let get = |pass| ids.0.get(&(pass, cloud.samples)).and_then(|id| cache.get_render_pipeline(*id));
    let (Some(trace), Some(resolve)) = (get(Pass::Trace), get(Pass::Resolve)) else { return };
    let offsets = offsets(offs);
    fullscreen(
        &mut ctx,
        "clouds_trace",
        &[attachment(&cloud.trace.0.default_view), attachment(&cloud.trace.1.default_view)],
        trace,
        [(&cloud.atmosphere, &offsets), (&cloud.density, &[])],
    );
    fullscreen(
        &mut ctx,
        "clouds_resolve",
        &[attachment(&cloud.history.0.default_view), attachment(&cloud.history.1.default_view)],
        resolve,
        [(&cloud.atmosphere, &offsets), (&cloud.resolve, &[])],
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
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("clouds_composite"),
        color_attachments: &[Some(target.get_color_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &cloud.atmosphere, &offsets);
    pass.set_bind_group(1, &cloud.composite, &[]);
    pass.draw(0..3, 0..1);
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
