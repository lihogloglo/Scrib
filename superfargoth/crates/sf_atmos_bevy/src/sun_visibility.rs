//! One sun visibility for all haze (`docs/specs/aerial-shadow.md` "Architecture review", ROADMAP
//! 1.27 A2, FG-12): a low sun hidden behind a ridge stops lighting the haze and the fog in front
//! of the shaded terrain, on screen or off. The canonical technique: shadow the key light's single
//! scattering per pixel along the view ray (Hillaire 2020, the per-pixel march for mountain
//! shadows; UE "Cast Shadows on Atmosphere"; Yusov, GPU Pro 5).
//!
//! Per view with Bevy's `AtmosphereSettings` and a sun (the [`CloudShadowLight`](crate::CloudShadowLight), [`ShaftsSun`]),
//! while [`SUN_VIS_ENABLED`], at half the internal resolution (the `(internal + 1) / 2` size the
//! clouds and the light shafts use), `Core3d` after the opaque and transmissive passes, in
//! `RenderSkyInputs` (before `render_sky`), before the transparents. GPU timer `sun_visibility`.
//!
//! 1. `sun_vis_march`: each pixel marches its view ray to min(scene depth, [`SUN_VIS_MAX_DISTANCE`])
//!    (sky: to the max distance) with [`SUN_VIS_SAMPLES`] jittered samples (`reach · x²`, denser
//!    near the camera; R2 + R1 jitter, [`sf_atmos::sun_visibility`]). Every sample reads the
//!    shared `sun_visibility(p)` (`shaders/sun_visibility.wesl`: the cascades inside their range,
//!    the terrain's far map beyond, × the cloud shadow; the froxels' rule). Two weighted means:
//!    `V_atmo` (Bevy's medium scattering × its transmittance, the combined density LUT) and
//!    `V_fog` (our height fog × its transmittance, from the froxels' far on; the ground banks are
//!    left out). Past the reach the ray counts as lit (one closed-form segment to the scene; sky
//!    pixels to the fog's sky distance).
//! 2. `sun_vis_resolve`: last frame's result reprojected (the pixel's world point; sky: its
//!    direction), clipped to this frame's 3×3 min/max (the froxels' Karis rule), blended with
//!    [`SUN_VIS_HISTORY`]. Output `Rg16Float`: R = `V_atmo`, G = `V_fog`.
//!
//! Consumers: Bevy's aerial perspective through the vendored hook `bevy_pbr::AtmosphereLightShafts`
//! (texture = the output, whose red is `V_atmo`; `light_dir` = the sun, `darkness` 1, `lobe_g` 0:
//! no lobe), and the cloud composite's fog sun term (`V_fog`, `cloud_composite.wesl`). With the pass
//! off, the light shaft occlusion (`sky.shafts.enabled`, the A/B arm) keeps both hooks.

use std::collections::HashMap;

use bevy_app::App;
use bevy_asset::{load_embedded_asset, Handle};
use bevy_camera::MainPassResolutionOverride;
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::Core3d;
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, Mat4, UVec2, Vec2, Vec3, Vec4};
use bevy_pbr::resources::AtmosphereTextures;
use bevy_pbr::{main_transmissive_pass_3d, AtmosphereLightShafts, AtmosphereSettings, ExtractedAtmosphere, GpuAtmosphereLightShafts, RenderSkyInputs, ShadowSamplers, ViewShadowBindings};
use bevy_render::camera::TemporalJitter;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::binding_types::{sampler, texture_2d, texture_2d_array, uniform_buffer};
use bevy_render::render_resource::{
    AddressMode, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites, FilterMode,
    FragmentState, LoadOp, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StoreOp, TextureFormat, TextureSampleType, TextureView, UniformBuffer,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::{CachedTexture, GpuImage, TextureCache};
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use sf_tunables::{Def, Group, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::render::{AtmosphereOffsets, AtmosphereResources, FogUniform};
use crate::shafts::{LightShafts, ShaftsSun};
use crate::CloudFrame;

/// `sky.sun_vis.enabled`: the sun-visibility pass. `--off sunvis` sets it false at the
/// command-line layer.
pub const SUN_VIS_ENABLED: Def<bool> = def!(bool "sky.sun_vis.enabled" = true)
    .help("Per-pixel sun visibility along the view ray: terrain and clouds shadow the sun's light in the haze and the fog, on screen or off")
    .source(Source::Industry("UE SkyAtmosphere 'Cast Shadows on Atmosphere'; Hillaire 2020 per-pixel shadowed aerial perspective"))
    .group(Group::Sky);

/// `sky.sun_vis.samples`: samples per pixel along the view ray.
pub const SUN_VIS_SAMPLES: Def<u32> = def!(count "sky.sun_vis.samples" = 16)
    .range(1, 64)
    .unit(Unit::Count)
    .help("Samples of the sun's visibility per pixel along the view ray (half resolution, jittered, history-blended)")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.sun_vis.max_distance_m`: how far the samples reach; farther, the ray counts as lit (the
/// far map ends at about ±4.6 km).
pub const SUN_VIS_MAX_DISTANCE: Def<f32> = def!(float "sky.sun_vis.max_distance_m" = 5000.0)
    .range(100.0, 50_000.0)
    .unit(Unit::Meters)
    .help("How far along the view ray the sun's visibility is marched; farther the haze counts as lit")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.sun_vis.history`: the weight of last frame's reprojected, clipped result.
pub const SUN_VIS_HISTORY: Def<f32> = def!(float "sky.sun_vis.history" = 0.9)
    .range(0.0, 0.99)
    .unit(Unit::Ratio)
    .help("Weight of the reprojected history in the sun-visibility pass (0 = this frame only)")
    .source(Source::Guess)
    .group(Group::Sky);

/// The sun-visibility pass's settings (bound from the `sky.sun_vis.*` tunables).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SunVisibility {
    pub enabled: bool,
    pub samples: u32,
    /// Reach of the march, meters.
    pub max_distance_m: f32,
    /// History weight, `0..1`.
    pub history: f32,
}

impl Default for SunVisibility {
    fn default() -> Self {
        Self {
            enabled: SUN_VIS_ENABLED.default_value(),
            samples: SUN_VIS_SAMPLES.default_value(),
            max_distance_m: SUN_VIS_MAX_DISTANCE.default_value(),
            history: SUN_VIS_HISTORY.default_value(),
        }
    }
}

/// The most samples the march takes (the top of `sky.sun_vis.samples`' range).
// tunables: const (capacity: the top of sky.sun_vis.samples' range)
const MAX_SAMPLES: u32 = 64;
/// This frame's march: `V_atmo`, `V_fog`, the scene distance (f16 holds it to 60 km), spare.
const RAW: TextureFormat = TextureFormat::Rgba16Float;
/// The result and its history: R = `V_atmo` (`render_sky`), G = `V_fog` (the composite).
pub(crate) const OUTPUT: TextureFormat = TextureFormat::Rg16Float;

/// Registers the tunables, binds them and adds the render passes.
pub(crate) fn build(app: &mut App) {
    app.init_resource::<SunVisibility>();
    let _bound = app.add_tunable(&SUN_VIS_ENABLED).bind_resource(|s: &mut SunVisibility, v| s.enabled = v);
    let _bound = app.add_tunable(&SUN_VIS_SAMPLES).bind_resource(|s: &mut SunVisibility, v| s.samples = v);
    let _bound = app.add_tunable(&SUN_VIS_MAX_DISTANCE).bind_resource(|s: &mut SunVisibility, v| s.max_distance_m = v);
    let _bound = app.add_tunable(&SUN_VIS_HISTORY).bind_resource(|s: &mut SunVisibility, v| s.history = v);

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<SunVisFrame>()
        .init_resource::<SunVisHistory>()
        .init_resource::<SunVisPipelineIds>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<SunVisPipelines>)
        .add_systems(
            Render,
            (
                queue_pipelines.in_set(RenderSystems::Queue),
                prepare_views.in_set(RenderSystems::PrepareResources),
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            ),
        )
        // After the water (its depth stops the march, W-ORDER), before `render_sky` and the
        // composite that read the result.
        .add_systems(
            Core3d,
            sun_visibility_pass
                .after(main_opaque_pass_3d)
                .after(main_transmissive_pass_3d)
                .in_set(RenderSkyInputs)
                .before(main_transparent_pass_3d),
        );
}

// -- render world --------------------------------------------------------------

/// The main world's [`SunVisibility`] (while on, with a sun) and [`ShaftsSun`], extracted.
#[derive(Resource, Clone, Copy, Debug, Default)]
struct SunVisFrame {
    settings: Option<SunVisibility>,
    sun: Option<Vec3>,
}

/// A render-world camera that gets the sun-visibility pass (while [`SunVisibility::enabled`], with
/// a sun, on a camera with Bevy's `AtmosphereSettings`).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SunVisCamera;

/// GPU layout of `SunVisPass` in `sun_vis_common.wesl` (field order is the layout).
#[derive(Clone, Copy, Default, ShaderType)]
pub(crate) struct SunVisUniform {
    pub prev_clip_from_world: Mat4,
    pub camera: Vec3,
    pub max_distance: f32,
    pub forward: Vec3,
    pub samples: u32,
    pub sun_dir: Vec3,
    pub frame: u32,
    pub origin_shift: Vec3,
    pub history: f32,
    pub shadow_center: Vec3,
    pub cloud_shadow: u32,
    pub shadow_x: Vec3,
    pub history_valid: u32,
    pub shadow_y: Vec3,
    pub fog_start: f32,
    pub planet_center: Vec3,
    pub inner_radius: f32,
    pub far_map: Vec4,
    pub far_vis: Vec4,
    pub half_size: Vec2,
    pub full_size: Vec2,
    pub far_fade: f32,
    pub sky_end: f32,
    pub fog: FogUniform,
}

/// GPU layout of `SunVisUse` in `sun_vis_common.wesl`: what the composite reads.
#[derive(Clone, Copy, Default, ShaderType)]
pub(crate) struct SunVisUseUniform {
    /// Toward the sun (world space).
    pub light_dir: Vec3,
    /// 1: the pass runs this frame (the fog's sun term takes `V_fog`).
    pub enabled: u32,
}

/// The values the passes take this frame for one view (CPU side, testable without a GPU).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SunVisViewParams {
    /// Samples per pixel (clamped to the tunable's range).
    pub samples: u32,
    /// The march's reach, meters.
    pub max_distance: f32,
    /// Where a sky pixel's weights end, meters (at least the reach).
    pub sky_end: f32,
    /// The fog's weights start here (the froxels' far when they run, else 0).
    pub fog_start: f32,
    /// History weight.
    pub history: f32,
}

impl SunVisViewParams {
    /// From the settings, the fog's sky distance (none: no fog) and the froxels' far (none: the
    /// froxels don't run).
    #[must_use]
    pub fn new(settings: &SunVisibility, fog_sky_distance: Option<f32>, froxels_far: Option<f32>) -> Self {
        let max_distance = settings.max_distance_m.max(0.0);
        Self {
            samples: settings.samples.clamp(1, MAX_SAMPLES),
            max_distance,
            sky_end: fog_sky_distance.unwrap_or(0.0).max(max_distance),
            fog_start: froxels_far.unwrap_or(0.0).max(0.0),
            history: settings.history.clamp(0.0, 1.0),
        }
    }
}

/// The vendored aerial-perspective hook's uniform for the pass's result: the sun, no lobe
/// (`lobe_g` 0 makes `render_sky`'s lobe exactly 1), full darkness once the pass writes its
/// texture (`ready`), none before (an unwritten target would darken the haze).
#[must_use]
pub fn aerial_hook(sun: Vec3, ready: bool) -> GpuAtmosphereLightShafts {
    GpuAtmosphereLightShafts { light_dir: sun, darkness: if ready { 1.0 } else { 0.0 }, lobe_g: 0.0 }
}

/// A view's targets and uniforms (prepared in `PrepareResources`).
#[derive(Component)]
pub(crate) struct SunVisView {
    /// This frame's march.
    raw: CachedTexture,
    /// The resolve writes `write` from `read` (last frame's `write`).
    write: CachedTexture,
    read: CachedTexture,
    uniform: UniformBuffer<SunVisUniform>,
    /// The composite's switch.
    pub use_uniform: UniformBuffer<SunVisUseUniform>,
    /// The aerial-perspective hook's uniform ([`aerial_hook`]).
    occlusion: UniformBuffer<GpuAtmosphereLightShafts>,
    samples: u32,
    /// The pipelines are compiled and the bind groups made: the pass writes the result this frame.
    ready: bool,
}

impl SunVisView {
    /// The result this frame (R = `V_atmo`, G = `V_fog`).
    pub(crate) fn output_view(&self) -> &TextureView {
        &self.write.default_view
    }

    /// Nothing reads the result this frame (its inputs are missing): the hook masks nothing and
    /// the composite takes its other path. Rewrites the same buffers before the frame's commands.
    fn disable(&mut self, device: &RenderDevice, queue: &RenderQueue) {
        self.ready = false;
        self.use_uniform.get_mut().enabled = 0;
        self.use_uniform.write_buffer(device, queue);
        self.occlusion.get_mut().darkness = 0.0;
        self.occlusion.write_buffer(device, queue);
    }
}

/// The view's bind groups: the atmosphere (group 0, as `render_sky`'s), the march's and the
/// resolve's group 1.
#[derive(Component)]
pub(crate) struct SunVisBindGroups {
    atmosphere: BindGroup,
    march: BindGroup,
    resolve: BindGroup,
}

/// The pass's pipelines, by MSAA samples (the atmosphere group's depth binding).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pipe {
    March(u32),
    Resolve(u32),
}

#[derive(Resource)]
struct SunVisPipelines {
    atmosphere: BindGroupLayoutDescriptor,
    atmosphere_msaa: BindGroupLayoutDescriptor,
    march: BindGroupLayoutDescriptor,
    resolve: BindGroupLayoutDescriptor,
    /// march, resolve.
    shaders: [Handle<Shader>; 2],
    fullscreen: FullscreenShader,
    linear: Sampler,
    repeat: Sampler,
}

impl FromWorld for SunVisPipelines {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let sampler_of = |address| {
            device.create_sampler(&SamplerDescriptor {
                label: Some("sun_visibility"),
                address_mode_u: address,
                address_mode_v: address,
                address_mode_w: address,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                ..SamplerDescriptor::default()
            })
        };
        let linear = sampler_of(AddressMode::ClampToEdge);
        let repeat = sampler_of(AddressMode::Repeat);
        Self {
            atmosphere: crate::render::atmosphere_layout("sun_vis_atmosphere", false),
            atmosphere_msaa: crate::render::atmosphere_layout("sun_vis_atmosphere_msaa", true),
            march: BindGroupLayoutDescriptor::new(
                "sun_vis_march",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<SunVisUniform>(false),
                        texture_2d_array(TextureSampleType::Depth),
                        sampler(SamplerBindingType::Comparison),
                        texture_2d(TextureSampleType::default()),
                        texture_2d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                        sampler(SamplerBindingType::Filtering),
                    ),
                ),
            ),
            resolve: BindGroupLayoutDescriptor::new(
                "sun_vis_resolve",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<SunVisUniform>(false),
                        texture_2d(TextureSampleType::Float { filterable: false }),
                        texture_2d(TextureSampleType::default()),
                        sampler(SamplerBindingType::Filtering),
                    ),
                ),
            ),
            shaders: [load_embedded_asset!(world, "shaders/sun_vis_march.wesl"), load_embedded_asset!(world, "shaders/sun_vis_resolve.wesl")],
            fullscreen: world.resource::<FullscreenShader>().clone(),
            linear,
            repeat,
        }
    }
}

impl SunVisPipelines {
    fn descriptor(&self, pipe: Pipe) -> RenderPipelineDescriptor {
        let (samples, group1, shader, entry, format) = match pipe {
            Pipe::March(n) => (n, &self.march, 0, "march", RAW),
            Pipe::Resolve(n) => (n, &self.resolve, 1, "resolve", OUTPUT),
        };
        let atmosphere = if samples > 1 { &self.atmosphere_msaa } else { &self.atmosphere };
        RenderPipelineDescriptor {
            label: Some(format!("sun_visibility_{pipe:?}").into()),
            layout: vec![atmosphere.clone(), group1.clone()],
            vertex: self.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shaders[shader].clone(),
                shader_defs: if samples > 1 { vec!["MULTISAMPLED".into()] } else { Vec::new() },
                entry_point: Some(entry.into()),
                targets: vec![Some(ColorTargetState { format, blend: None, write_mask: ColorWrites::ALL })],
                ..FragmentState::default()
            }),
            ..RenderPipelineDescriptor::default()
        }
    }
}

/// The queued pipelines.
#[derive(Resource, Default)]
pub(crate) struct SunVisPipelineIds(HashMap<Pipe, CachedRenderPipelineId>);

impl SunVisPipelineIds {
    fn get<'a>(&self, cache: &'a PipelineCache, pipe: Pipe) -> Option<&'a RenderPipeline> {
        self.0.get(&pipe).and_then(|id| cache.get_render_pipeline(*id))
    }
}

/// Last frame's per-view state the reprojection needs.
struct ViewHistory {
    prev_clip_from_world: Mat4,
    origin: DVec3,
    /// Frames written in a row (0: no history).
    frame: u32,
    size: UVec2,
}

#[derive(Resource, Default)]
pub(crate) struct SunVisHistory(HashMap<Entity, ViewHistory>);

type CameraItem = (RenderEntity, Has<AtmosphereSettings>);

pub(crate) fn extract(
    mut commands: Commands,
    settings: Extract<Res<SunVisibility>>,
    sun: Extract<Res<ShaftsSun>>,
    shafts: Extract<Option<Res<LightShafts>>>,
    cameras: Extract<Query<CameraItem, With<bevy_camera::Camera3d>>>,
    mut history: ResMut<SunVisHistory>,
) {
    let on = settings.enabled && sun.0.is_some();
    let shafts_on = shafts.as_ref().is_some_and(|s| s.enabled) && sun.0.is_some();
    commands.insert_resource(SunVisFrame { settings: on.then_some(**settings), sun: sun.0 });
    for (entity, atmosphere) in &cameras {
        let mut e = commands.entity(entity);
        if on && atmosphere {
            e.insert(SunVisCamera);
        } else {
            e.remove::<(SunVisCamera, SunVisView, SunVisBindGroups)>();
            // The light shaft occlusion (the A/B arm) owns the vendored hook while it is on; else
            // the vendored `render_sky` falls back to its stock pipeline the same frame.
            if !(shafts_on && atmosphere) {
                e.remove::<AtmosphereLightShafts>();
            }
            history.0.remove(&entity);
        }
    }
}

fn queue_pipelines(views: Query<&Msaa, With<SunVisCamera>>, pipelines: Option<Res<SunVisPipelines>>, cache: Res<PipelineCache>, mut ids: ResMut<SunVisPipelineIds>) {
    let Some(pipelines) = pipelines else { return };
    for msaa in &views {
        let samples = msaa.samples();
        for pipe in [Pipe::March(samples), Pipe::Resolve(samples)] {
            ids.0.entry(pipe).or_insert_with(|| cache.queue_render_pipeline(pipelines.descriptor(pipe)));
        }
    }
}

type ViewItem = (
    Entity,
    &'static ExtractedView,
    &'static Msaa,
    Option<&'static MainPassResolutionOverride>,
    Option<&'static TemporalJitter>,
    &'static ExtractedAtmosphere,
    Has<bevy_pbr::SkyOccluded>,
);

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn prepare_views(
    mut commands: Commands,
    views: Query<ViewItem, With<SunVisCamera>>,
    frame: Res<SunVisFrame>,
    cloud: Res<CloudFrame>,
    images: Res<RenderAssets<GpuImage>>,
    (device, queue): (Res<RenderDevice>, Res<RenderQueue>),
    mut textures: ResMut<TextureCache>,
    mut history: ResMut<SunVisHistory>,
    (ids, cache): (Res<SunVisPipelineIds>, Res<PipelineCache>),
) {
    let (Some(settings), Some(sun)) = (frame.settings, frame.sun) else { return };
    let fog = cloud.fog.as_ref();
    let froxels_far = cloud.froxels.filter(|_| fog.is_some()).map(|f| f.far);
    let params = SunVisViewParams::new(&settings, fog.map(|f| f.sky_distance), froxels_far);
    let far_map = cloud.far_map.as_ref().filter(|(image, _)| images.get(image).is_some()).map_or(Vec4::ZERO, |m| m.1);
    let shadow = cloud.shadow.filter(|_| cloud.clouds);
    for (entity, view, msaa, resolution, jitter, atmosphere, occluded) in &views {
        // Rebuilt in `PrepareBindGroups` this frame, or the result is unused (`SunVisView::disable`).
        commands.entity(entity).remove::<SunVisBindGroups>();
        let full = resolution.map_or(UVec2::new(view.viewport.z, view.viewport.w), |r| r.0).max(UVec2::ONE);
        let half = (full + 1) / 2;
        // Jittered like the depth the march reads (Bevy's view uniform under TAA/DLSS), as the
        // clouds' history.
        let mut clip_from_view = view.clip_from_view;
        if let Some(jitter) = jitter {
            jitter.jitter_projection(&mut clip_from_view, full.as_vec2());
        }
        let clip_from_world = clip_from_view * view.world_from_view.to_matrix().inverse();
        let samples = msaa.samples();
        let ready = ids.get(&cache, Pipe::March(samples)).is_some() && ids.get(&cache, Pipe::Resolve(samples)).is_some();
        let h = history.0.entry(entity).or_insert(ViewHistory { prev_clip_from_world: clip_from_world, origin: cloud.origin, frame: 0, size: UVec2::ZERO });
        let valid = ready && !occluded && h.size == half && h.frame > 0;
        #[allow(clippy::cast_possible_truncation, reason = "a frame's origin shift is small")]
        let origin_shift = (cloud.origin - h.origin).as_vec3();
        let planet_center = atmosphere.world_to_atmosphere.inverse().transform_point3(Vec3::ZERO);
        let uniform = SunVisUniform {
            prev_clip_from_world: h.prev_clip_from_world,
            camera: view.world_from_view.translation(),
            max_distance: params.max_distance,
            forward: view.world_from_view.forward().as_vec3(),
            samples: params.samples,
            sun_dir: sun,
            frame: h.frame,
            origin_shift,
            history: params.history,
            shadow_center: shadow.map_or(Vec3::ZERO, |b| b.center),
            cloud_shadow: u32::from(shadow.is_some()),
            shadow_x: shadow.map_or(Vec3::X, |b| b.x),
            history_valid: u32::from(valid),
            shadow_y: shadow.map_or(Vec3::Y, |b| b.y),
            fog_start: params.fog_start,
            planet_center,
            inner_radius: atmosphere.inner_radius,
            far_map,
            far_vis: cloud.far_shape.froxel_vec(),
            half_size: half.as_vec2(),
            full_size: full.as_vec2(),
            far_fade: cloud.far_shape.fade_start,
            sky_end: params.sky_end,
            fog: FogUniform::new(fog, &cloud.fog_offsets, cloud.fog_ambient, crate::render::FOG_STEPS, cloud.fog_bank_edges),
        };
        let even = h.frame.is_multiple_of(2);
        h.prev_clip_from_world = clip_from_world;
        h.origin = cloud.origin;
        h.size = half;
        h.frame = if ready && !occluded { h.frame.wrapping_add(1).max(1) } else { 0 };

        let mut uniform = UniformBuffer::from(uniform);
        uniform.write_buffer(&device, &queue);
        let mut use_uniform = UniformBuffer::from(SunVisUseUniform { light_dir: sun, enabled: u32::from(ready) });
        use_uniform.write_buffer(&device, &queue);
        let mut occlusion = UniformBuffer::from(aerial_hook(sun, ready));
        occlusion.write_buffer(&device, &queue);
        let raw = crate::render::cached(&mut textures, &device, "sun_vis_raw", half, RAW);
        let a = crate::render::cached(&mut textures, &device, "sun_vis_a", half, OUTPUT);
        let b = crate::render::cached(&mut textures, &device, "sun_vis_b", half, OUTPUT);
        let (write, read) = if even { (a, b) } else { (b, a) };
        let Some(buffer) = occlusion.buffer().cloned() else { continue };
        let vendored = AtmosphereLightShafts { occlusion: write.default_view.clone(), uniform: buffer };
        commands.entity(entity).insert((SunVisView { raw, write, read, uniform, use_uniform, occlusion, samples, ready }, vendored));
    }
}

type BindViewItem = (Entity, &'static mut SunVisView, &'static ViewDepthStencilTexture, &'static ExtractedAtmosphere, &'static AtmosphereTextures, Option<&'static ViewShadowBindings>);

#[allow(clippy::too_many_arguments)]
fn prepare_bind_groups(
    mut commands: Commands,
    mut views: Query<BindViewItem>,
    pipelines: Option<Res<SunVisPipelines>>,
    cache: Res<PipelineCache>,
    (device, queue): (Res<RenderDevice>, Res<RenderQueue>),
    cloud: Res<CloudFrame>,
    images: Res<RenderAssets<GpuImage>>,
    shadow_samplers: Option<Res<ShadowSamplers>>,
    atmos: AtmosphereResources,
) {
    let (atmosphere_uniforms, settings_uniforms, transforms, view_uniforms, lights, media, medium_sampler, atmosphere_sampler) = atmos;
    let uniforms = atmosphere_uniforms
        .binding()
        .zip(settings_uniforms.binding())
        .zip(transforms.uniforms().binding())
        .zip(view_uniforms.uniforms.binding())
        .zip(lights.view_gpu_lights.binding())
        .map(|((((a0, a1), a2), a3), a4)| [a0, a1, a2, a3, a4]);
    // The far map's stand-in is the cloud shadow image (as the froxels'): `far_map.z` 0 skips it.
    let cloud_shadow = images.get(&cloud.images.shadow);
    for (entity, mut sv, depth, atmosphere, luts, shadow_maps) in &mut views {
        let inputs = (pipelines.as_deref(), shadow_samplers.as_deref(), uniforms.as_ref(), cloud_shadow, shadow_maps);
        let (Some(pipelines), Some(shadow_samplers), Some(uniforms), Some(cloud_shadow), Some(shadow_maps)) = inputs else {
            sv.disable(&device, &queue);
            continue;
        };
        let (Some(depth_view), Some(uniform), Some(_)) = (depth.attachment.depth_stencil_views().depth_only_view(), sv.uniform.binding(), media.get(atmosphere.medium)) else {
            sv.disable(&device, &queue);
            continue;
        };
        let far_map = cloud.far_map.as_ref().and_then(|(image, _)| images.get(image)).map_or(&cloud_shadow.texture_view, |g| &g.texture_view);
        let layout = if sv.samples > 1 { &pipelines.atmosphere_msaa } else { &pipelines.atmosphere };
        let atmosphere = crate::render::atmosphere_group(
            &device,
            &cache,
            ("sun_vis_atmosphere", layout),
            uniforms.clone(),
            luts,
            (&medium_sampler, &atmosphere_sampler),
            depth_view,
        );
        let march = device.create_bind_group(
            "sun_vis_march",
            &cache.get_bind_group_layout(&pipelines.march),
            &BindGroupEntries::sequential((
                uniform.clone(),
                &shadow_maps.directional_light_depth_texture_view,
                &shadow_samplers.directional_light_comparison_sampler,
                far_map,
                &cloud_shadow.texture_view,
                &pipelines.linear,
                &pipelines.repeat,
            )),
        );
        let resolve = device.create_bind_group(
            "sun_vis_resolve",
            &cache.get_bind_group_layout(&pipelines.resolve),
            &BindGroupEntries::sequential((uniform, &sv.raw.default_view, &sv.read.default_view, &pipelines.linear)),
        );
        commands.entity(entity).insert(SunVisBindGroups { atmosphere, march, resolve });
    }
}

/// One fullscreen draw of `pipeline` into `target` (groups 0 and 1).
fn draw(ctx: &mut RenderContext, label: &'static str, target: &TextureView, pipeline: &RenderPipeline, groups: [(&BindGroup, &[u32]); 2]) {
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: Operations { load: LoadOp::Load, store: StoreOp::Store },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    for (i, (group, offsets)) in (0u32..).zip(groups) {
        pass.set_bind_group(i, group, offsets);
    }
    pass.draw(0..3, 0..1);
}

/// What [`sun_visibility_pass`] reads per view.
type SunVisPassView = (&'static SunVisView, Option<&'static SunVisBindGroups>, Option<AtmosphereOffsets>);

/// The march and the resolve, under one GPU timer `sun_visibility`. Skipped on a view under water
/// (`render_sky` and the composite skip it too) and while the result is unused (pipelines still
/// compiling, inputs missing: [`SunVisView::disable`]).
pub(crate) fn sun_visibility_pass(
    view: ViewQuery<SunVisPassView, Without<bevy_pbr::SkyOccluded>>,
    ids: Res<SunVisPipelineIds>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (sv, groups, offs) = view.into_inner();
    let pipes = ids.get(&cache, Pipe::March(sv.samples)).zip(ids.get(&cache, Pipe::Resolve(sv.samples)));
    let (true, Some(groups), Some(offs), Some((march, resolve))) = (sv.ready, groups, offs, pipes) else { return };
    let offsets = crate::render::offsets(offs);
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let span = diagnostics.time_span(ctx.command_encoder(), "sun_visibility");
    draw(&mut ctx, "sun_vis_march", &sv.raw.default_view, march, [(&groups.atmosphere, &offsets), (&groups.march, &[])]);
    draw(&mut ctx, "sun_vis_resolve", &sv.write.default_view, resolve, [(&groups.atmosphere, &offsets), (&groups.resolve, &[])]);
    span.end(ctx.command_encoder());
}
