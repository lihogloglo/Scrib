//! The drawn sky bodies (spec: `docs/specs/sky-lighting.md`, "Night sky"):
//! stars, moons and the sun's glare, in one fullscreen pass over the sky
//! pixels after Bevy's `render_sky` and before the transparent pass, so a
//! cloud composite that runs after it hides them behind clouds.
//!
//! Why our own pass: Bevy's atmosphere draws only `SunDisk`s (uniform disks,
//! no texture, no phase) and has no star field; a mesh in the transparent
//! pass would draw over the clouds, and an opaque one would replace the
//! sky's in-scattering (a dark moon by day). Drawing additively into the sky
//! keeps the physics: what lies in front (air, clouds) dims and hides them.

use std::collections::HashMap;

use bevy_app::{App, SubApp};
use bevy_asset::{AssetId, AssetServer, Handle, embedded_asset, load_embedded_asset};
use bevy_camera::{Camera3d, MainPassResolutionOverride, Viewport};
use bevy_core_pipeline::core_3d::{CORE_3D_DEPTH_FORMAT, main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::Core3d;
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{InternedSystemSet, IntoSystemSet, NodeId, Schedules};
use bevy_image::Image;
use bevy_math::{Vec3, Vec4};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation,
    BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
    FilterMode, FragmentState, MultisampleState, PipelineCache, RenderPassDescriptor, RenderPipelineDescriptor, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StencilFaceState, StencilState, StoreOp, TextureFormat,
    TextureSampleType, UniformBuffer,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::{FallbackImage, GpuImage};
use bevy_render::view::{Msaa, ViewDepthStencilTexture, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use bevy_time::Time;
use sf_sky::MAX_MOONS;

use crate::{CurrentSky, SkyCamera, SkyConfig, WeatherState};

/// Transmittance table size (by `sqrt(sin(elevation))`).
const T_ROWS: usize = 16;

/// GPU layout of `GpuMoon` in `night.wesl`.
#[derive(Clone, Copy, Default, ShaderType)]
struct GpuMoon {
    direction: Vec3,
    tan_radius: f32,
    right: Vec3,
    occluder: f32,
    up: Vec3,
    textured: u32,
    light: Vec3,
    _pad: f32,
    radiance: Vec3,
    _pad2: f32,
}

/// GPU layout of `Night` in `night.wesl`.
#[derive(Clone, Copy, Default, ShaderType)]
struct NightUniform {
    moons: [GpuMoon; MAX_MOONS],
    sun_direction: Vec3,
    glare_size: f32,
    glare_radiance: Vec3,
    star_density: f32,
    star_lux: f32,
    twinkle: f32,
    time: f32,
    moon_count: u32,
    transmittance: [Vec4; T_ROWS],
}

/// The moons' phase textures, loaded from [`sf_sky::Moon::textures`].
#[derive(Resource, Default)]
struct MoonImages {
    built_from: Vec<Vec<String>>,
    handles: Vec<Vec<Handle<Image>>>,
}

/// What the render world draws this frame.
#[derive(Resource, Clone, Default)]
struct NightFrame {
    uniform: NightUniform,
    images: [Option<AssetId<Image>>; MAX_MOONS],
    active: bool,
}

/// Render world: this view draws the night sky.
#[derive(Component)]
struct NightCamera;

pub(crate) fn build(app: &mut App) {
    embedded_asset!(app, "shaders/night.wesl");
    app.init_resource::<MoonImages>()
        .init_resource::<NightFrame>()
        .add_systems(
            bevy_app::Update,
            (load_moon_images, prepare_frame).chain().after(crate::SkySystems::Apply),
        );
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<NightFrame>()
        .init_resource::<NightPipelineIds>()
        .init_resource::<NightBindGroup>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<NightPipeline>)
        .add_systems(Render, (queue_pipelines, prepare_bind_group).chain().in_set(RenderSystems::PrepareBindGroups));
}

/// Orders the pass after Bevy's `render_sky`, before anything later
/// plugins add between the opaque and transparent passes (a cloud composite
/// that discovers the same way then runs after this pass).
///
/// Scaffold `SkyOrderScaffold` (shared with `sf_atmos_bevy`): `render_sky`
/// is private to `bevy_pbr`, so its set can't be named. Removal path: a
/// public `render_sky` set upstream. Add `SkyPlugin` before cloud plugins.
pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    let sets = sets_before_transparent(render_app);
    let mut config = night_pass.after(main_opaque_pass_3d).before(main_transparent_pass_3d).into_configs();
    for set in sets {
        config = config.after(set);
    }
    render_app.add_systems(Core3d, config);
}

fn sets_before_transparent(render_app: &mut SubApp) -> Vec<InternedSystemSet> {
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
            out.extend(system.default_system_sets().into_iter().filter(|set| *set != transparent));
        }
    }
    out
}

// -- main world ----------------------------------------------------------------

fn load_moon_images(config: Res<SkyConfig>, server: Option<Res<AssetServer>>, mut images: ResMut<MoonImages>) {
    let Some(server) = server else { return };
    let wanted: Vec<Vec<String>> = config.0.moon_list().iter().map(|m| m.textures.clone()).collect();
    if wanted != images.built_from {
        images.handles = wanted.iter().map(|paths| paths.iter().map(|p| server.load(p.clone())).collect()).collect();
        images.built_from = wanted;
    }
}

/// Builds the frame's uniform from [`CurrentSky`].
fn prepare_frame(
    sky: Res<CurrentSky>,
    config: Res<SkyConfig>,
    weather: Res<WeatherState>,
    images: Res<MoonImages>,
    time: Res<Time>,
    cameras: Query<(), With<SkyCamera>>,
    mut frame: ResMut<NightFrame>,
) {
    let settings = &config.0;
    let state = &sky.0;
    let mut u = NightUniform::default();
    let mut ids = [None; MAX_MOONS];
    for (i, moon) in state.night.moons().iter().enumerate() {
        let d = moon.position.direction.normalize_or(Vec3::NEG_Y);
        let reference = if d.y.abs() > 0.999 { Vec3::Z } else { Vec3::Y };
        let right = d.cross(reference).normalize();
        let up = right.cross(d);
        let radius = moon.angular_size * 0.5;
        let solid_angle = std::f32::consts::PI * radius * radius;
        let frame_image = moon.frame.and_then(|k| images.handles.get(i)?.get(k as usize)).map(Handle::id);
        ids[i] = frame_image;
        // Phase lighting for the procedural sphere: θ = 0 new (lit from
        // behind), π full (lit from the viewer), waxing lit on the right.
        let theta = std::f32::consts::TAU * moon.phase;
        u.moons[i] = GpuMoon {
            direction: d,
            tan_radius: radius.tan(),
            right,
            occluder: moon.occluder,
            up,
            textured: u32::from(frame_image.is_some()),
            light: Vec3::new(theta.sin(), 0.0, -theta.cos()),
            radiance: moon.tint * moon.transmittance * (moon.full_illuminance / solid_angle.max(1e-9)),
            ..GpuMoon::default()
        };
        u.moon_count += 1;
    }
    let look = &settings.sun_disk;
    u.sun_direction = state.sun.direction;
    u.glare_size = look.glare_size;
    let spread = 2.0 * std::f32::consts::PI * look.glare_size * look.glare_size;
    u.glare_radiance = state.sun.transmittance * (look.glare * weather.0.sun_glare.clamp(0.0, 1.0) * state.sun.illuminance / spread.max(1e-9));
    u.star_density = settings.stars.density.max(0.05);
    u.star_lux = settings.stars.brightest_lux * state.night.stars * weather.0.direct_light_factor();
    u.twinkle = settings.stars.twinkle;
    u.time = time.elapsed_secs_wrapped();
    if u.star_lux > 0.0 {
        #[expect(clippy::cast_precision_loss, reason = "16 rows")]
        for (row, t) in u.transmittance.iter_mut().enumerate() {
            let s = (row as f32 / (T_ROWS - 1) as f32).powi(2);
            let rgb = settings.atmosphere.transmittance(settings.observer_altitude, s.asin(), &weather.0.medium(), 24);
            *t = rgb.extend(1.0);
        }
    }
    *frame = NightFrame { uniform: u, images: ids, active: !cameras.is_empty() };
}

// -- render world --------------------------------------------------------------

type CameraItem = (RenderEntity, Has<SkyCamera>);

fn extract(mut commands: Commands, frame: Extract<Res<NightFrame>>, cameras: Extract<Query<CameraItem, With<Camera3d>>>) {
    commands.insert_resource(frame.clone());
    for (entity, sky) in &cameras {
        if sky && frame.active {
            commands.entity(entity).insert(NightCamera);
        } else {
            commands.entity(entity).remove::<NightCamera>();
        }
    }
}

#[derive(Resource)]
struct NightPipeline {
    layout: BindGroupLayoutDescriptor,
    fullscreen: FullscreenShader,
    shader: Handle<Shader>,
    sampler: Sampler,
}

impl FromWorld for NightPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let moon_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("night_sky"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..SamplerDescriptor::default()
        });
        let texture = || texture_2d(TextureSampleType::Float { filterable: true });
        Self {
            layout: BindGroupLayoutDescriptor::new(
                "night_sky",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<ViewUniform>(true),
                        uniform_buffer::<NightUniform>(false),
                        texture(),
                        texture(),
                        texture(),
                        texture(),
                        sampler(SamplerBindingType::Filtering),
                    ),
                ),
            ),
            fullscreen: world.resource::<FullscreenShader>().clone(),
            shader: load_embedded_asset!(world, "shaders/night.wesl"),
            sampler: moon_sampler,
        }
    }
}

impl NightPipeline {
    fn descriptor(&self, samples: u32, format: TextureFormat) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some(format!("night_sky_{samples}").into()),
            layout: vec![self.layout.clone()],
            vertex: self.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: Some("night_sky".into()),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState {
                        color: BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
                        alpha: BlendComponent { src_factor: BlendFactor::Zero, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
                ..FragmentState::default()
            }),
            // The fullscreen triangle sits at the far plane (reverse Z: 0),
            // so `GreaterEqual` passes only where nothing was drawn: the sky.
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: Some(false),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState { front: StencilFaceState::IGNORE, back: StencilFaceState::IGNORE, read_mask: 0, write_mask: 0 },
                bias: DepthBiasState { constant: 0, slope_scale: 0.0, clamp: 0.0 },
            }),
            multisample: MultisampleState { count: samples, ..MultisampleState::default() },
            ..RenderPipelineDescriptor::default()
        }
    }
}

#[derive(Resource, Default)]
struct NightPipelineIds(HashMap<(u32, TextureFormat), CachedRenderPipelineId>);

#[derive(Resource, Default)]
struct NightBindGroup(Option<BindGroup>);

fn queue_pipelines(
    views: Query<(&Msaa, &ViewTarget), With<NightCamera>>,
    pipeline: Option<Res<NightPipeline>>,
    cache: Res<PipelineCache>,
    mut ids: ResMut<NightPipelineIds>,
) {
    let Some(pipeline) = pipeline else { return };
    for (msaa, target) in &views {
        let key = (msaa.samples(), target.main_texture_format());
        ids.0.entry(key).or_insert_with(|| cache.queue_render_pipeline(pipeline.descriptor(key.0, key.1)));
    }
}

#[allow(clippy::too_many_arguments)]
fn prepare_bind_group(
    frame: Res<NightFrame>,
    pipeline: Option<Res<NightPipeline>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    view_uniforms: Res<ViewUniforms>,
    images: Res<RenderAssets<GpuImage>>,
    fallback: Res<FallbackImage>,
    mut group: ResMut<NightBindGroup>,
) {
    group.0 = None;
    let Some(pipeline) = pipeline else { return };
    if !frame.active {
        return;
    }
    let Some(view_binding) = view_uniforms.uniforms.binding() else { return };
    let mut uniform = frame.uniform;
    let views: Vec<_> = frame
        .images
        .iter()
        .enumerate()
        .map(|(i, id)| {
            if let Some(image) = id.and_then(|id| images.get(id)) {
                &image.texture_view
            } else {
                // Not loaded (yet): draw this moon procedurally.
                uniform.moons[i].textured = 0;
                &fallback.d2.texture_view
            }
        })
        .collect();
    let mut buffer = UniformBuffer::from(uniform);
    buffer.write_buffer(&device, &queue);
    let Some(binding) = buffer.binding() else { return };
    group.0 = Some(device.create_bind_group(
        "night_sky",
        &cache.get_bind_group_layout(&pipeline.layout),
        &BindGroupEntries::sequential((view_binding, binding, views[0], views[1], views[2], views[3], &pipeline.sampler)),
    ));
}

type NightViewItem = (
    &'static ViewTarget,
    &'static ViewDepthStencilTexture,
    &'static ViewUniformOffset,
    &'static Msaa,
    &'static ExtractedCamera,
    Option<&'static MainPassResolutionOverride>,
    Has<NightCamera>,
);

fn night_pass(
    view: ViewQuery<NightViewItem>,
    ids: Res<NightPipelineIds>,
    group: Res<NightBindGroup>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (target, depth, offset, msaa, camera, resolution, enabled) = view.into_inner();
    if !enabled {
        return;
    }
    let Some(group) = group.0.as_ref() else { return };
    let key = (msaa.samples(), target.main_texture_format());
    let Some(pipeline) = ids.0.get(&key).and_then(|id| cache.get_render_pipeline(*id)) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("night_sky"),
        color_attachments: &[Some(target.get_color_attachment())],
        depth_stencil_attachment: Some(depth.get_attachment(StoreOp::Store)),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let span = diagnostics.pass_span(&mut pass, "night_sky");
    if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(
            v.physical_position.x as f32,
            v.physical_position.y as f32,
            v.physical_size.x as f32,
            v.physical_size.y as f32,
            v.depth.start,
            v.depth.end,
        );
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, group, &[offset.offset]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}
