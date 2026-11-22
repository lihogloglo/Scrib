//! Procedural Atmospheric Scattering.
//!
//! This plugin implements [Hillaire's 2020 paper](https://sebh.github.io/publications/egsr2020.pdf)
//! on real-time atmospheric scattering. While it *will* work simply as a
//! procedural skybox, it also does much more. It supports dynamic time-of-
//! -day, multiple directional lights, and since it's applied as a post-processing
//! effect *on top* of the existing skybox, a starry skybox would automatically
//! show based on the time of day. Scattering in front of terrain (similar
//! to distance fog, but more complex) is handled as well, and takes into
//! account the directional light color and direction.
//!
//! To add the atmosphere to your scene, spawn an entity with the [`Atmosphere`] component and
//! [`bevy_transform::components::GlobalTransform`], and add [`AtmosphereSettings`] to each
//! 3D camera that should render it. Detailed documentation is on the [`Atmosphere`] component.
//!
//! Placement and scene scale come from the entity's transform. With several atmospheres in one
//! scene, each camera picks the atmosphere whose origin is closest in world space.
//!
//! Performance-wise, the effect should be fairly cheap since the LUTs (Look
//! Up Tables) that encode most of the data are small, and take advantage of the
//! fact that the atmosphere is symmetric. Performance is also proportional to
//! the number of directional lights in the scene. In order to tune
//! performance more finely, the [`AtmosphereSettings`] camera component
//! manages the size of each LUT and the sample count for each ray.
//!
//! Given how similar it is to [`crate::volumetric_fog`], it might be expected
//! that these two modules would work together well. However for now using both
//! at once is untested, and might not be physically accurate. These may be
//! integrated into a single module in the future.
//!
//! On web platforms, atmosphere rendering will look slightly different. Specifically, when calculating how light travels
//! through the atmosphere, we use a simpler averaging technique instead of the more
//! complex blending operations. This difference will be resolved for WebGPU in a future release.
//!
//! [Shadertoy]: https://www.shadertoy.com/view/slSXRW
//!
//! [Unreal Engine Implementation]: https://github.com/sebh/UnrealEngineSkyAtmosphere

mod environment;
mod node;
pub mod resources;

use bevy_app::{App, Plugin};
use bevy_asset::{embedded_asset, AssetId, Handle};
use bevy_camera::{Camera3d, Hdr};
use bevy_core_pipeline::{
    core_3d::{main_opaque_pass_3d, main_transparent_pass_3d},
    schedule::{Core3d, Core3dSystems},
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    schedule::IntoScheduleConfigs,
    // superfargoth patch (own, LSO: light shaft occlusion)
    schedule::SystemSet,
    system::{Commands, Query},
};
use bevy_image::Image;
use bevy_light::{atmosphere::ScatteringMedium, Atmosphere};
use bevy_math::{Mat4, Quat, UVec2, UVec3, Vec3, Vec4};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_render::{
    extract_component::{DynamicUniformIndex, ExtractComponentPlugin, UniformComponentPlugin},
    render_resource::{DownlevelFlags, ShaderType, SpecializedRenderPipelines},
    renderer::RenderDevice,
    sync_component::{SyncComponent, SyncComponentPlugin},
    sync_world::RenderEntity,
    Extract, ExtractSchedule, RenderStartup,
};
use bevy_render::{
    render_resource::{TextureFormat, TextureUsages},
    renderer::RenderAdapter,
    GpuResourceAppExt, Render, RenderApp, RenderSystems,
};
use bevy_transform::components::GlobalTransform;

use bevy_shader::load_shader_library;
use environment::{
    atmosphere_environment, init_atmosphere_probe_layout, init_atmosphere_probe_pipeline,
    on_insert_atmosphere_environment_map_light, on_remove_atmosphere_environment_map_light,
    prepare_atmosphere_probe_bind_groups, prepare_probe_textures, AtmosphereEnvironmentMap,
};
use node::{atmosphere_luts, render_sky};
use resources::{
    prepare_atmosphere_transforms, prepare_atmosphere_uniforms, queue_render_sky_pipelines,
    AtmosphereTransforms, GpuAtmosphere, RenderSkyBindGroupLayouts,
};
use tracing::warn;

use crate::resources::prepare_atmosphere_buffers;

use self::resources::{
    prepare_atmosphere_bind_groups, prepare_atmosphere_textures, AtmosphereBindGroupLayouts,
    AtmosphereBindGroups, AtmosphereLutPipelines, AtmosphereSampler, AtmosphereStaticLutCache,
    AtmosphereTextures, AtmosphereTransformsOffset, RenderSkyPipelineId,
};

#[doc(hidden)]
pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "types.wesl");
        load_shader_library!(app, "functions.wesl");
        load_shader_library!(app, "bruneton_functions.wesl");
        load_shader_library!(app, "bindings.wesl");

        embedded_asset!(app, "transmittance_lut.wesl");
        embedded_asset!(app, "multiscattering_lut.wesl");
        embedded_asset!(app, "sky_view_lut.wesl");
        embedded_asset!(app, "aerial_view_lut.wesl");
        embedded_asset!(app, "render_sky.wesl");
        embedded_asset!(app, "environment.wesl");
        // superfargoth patch (own, P1b): combined medium LUTs on the GPU.
        embedded_asset!(app, "medium_lut.wesl");

        app.add_plugins((
            ExtractComponentPlugin::<AtmosphereEnvironmentMap>::default(),
            SyncComponentPlugin::<AtmosphereSettings>::default(),
            UniformComponentPlugin::<GpuAtmosphere>::default(),
            UniformComponentPlugin::<GpuAtmosphereSettings>::default(),
            // superfargoth patch (own, AS: aerial shadow)
            UniformComponentPlugin::<GpuAtmosphereHeightShadow>::default(),
        ))
        .add_observer(on_insert_atmosphere_environment_map_light)
        .add_observer(on_remove_atmosphere_environment_map_light);

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(ExtractSchedule, extract_atmosphere);
        }
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let render_adapter = render_app.world().resource::<RenderAdapter>();

        if !render_adapter
            .get_downlevel_capabilities()
            .flags
            .contains(DownlevelFlags::COMPUTE_SHADERS)
        {
            warn!("AtmospherePlugin not loaded. GPU lacks support for compute shaders.");
            return;
        }

        if !render_adapter
            .get_texture_format_features(TextureFormat::Rgba16Float)
            .allowed_usages
            .contains(TextureUsages::STORAGE_BINDING)
        {
            warn!("AtmospherePlugin not loaded. GPU lacks support: TextureFormat::Rgba16Float does not support TextureUsages::STORAGE_BINDING.");
            return;
        }

        // Check the `RenderDevice` in addition to the `RenderAdapter`. The
        // former takes the `WGPU_SETTINGS_PRIO` environment variable into
        // account, and the latter doesn't.
        let render_device = render_app.world().resource::<RenderDevice>();
        if render_device.limits().max_storage_textures_per_shader_stage == 0 {
            warn!("AtmospherePlugin not loaded. GPU lacks support: `max_storage_textures_per_shader_stage` is 0");
            return;
        }

        render_app
            .insert_resource(AtmosphereBindGroupLayouts::new())
            .init_resource::<AtmosphereStaticLutCache>()
            .init_gpu_resource::<RenderSkyBindGroupLayouts>()
            .init_gpu_resource::<AtmosphereSampler>()
            .init_gpu_resource::<AtmosphereLutPipelines>()
            .init_gpu_resource::<AtmosphereTransforms>()
            .init_gpu_resource::<SpecializedRenderPipelines<RenderSkyBindGroupLayouts>>()
            .add_systems(
                RenderStartup,
                (init_atmosphere_probe_layout, init_atmosphere_probe_pipeline).chain(),
            )
            .add_systems(
                Render,
                (
                    configure_camera_depth_usages.in_set(RenderSystems::PrepareViews),
                    queue_render_sky_pipelines.in_set(RenderSystems::Queue),
                    prepare_atmosphere_textures.in_set(RenderSystems::PrepareResources),
                    prepare_probe_textures
                        .in_set(RenderSystems::PrepareResources)
                        .after(prepare_atmosphere_textures),
                    prepare_atmosphere_uniforms
                        .in_set(RenderSystems::Prepare)
                        .before(RenderSystems::PrepareResources),
                    prepare_atmosphere_probe_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_atmosphere_transforms.in_set(RenderSystems::PrepareResources),
                    prepare_atmosphere_bind_groups.in_set(RenderSystems::PrepareBindGroups),
                    prepare_atmosphere_buffers.in_set(RenderSystems::PrepareResources),
                ),
            )
            .add_systems(
                Core3d,
                (
                    (atmosphere_luts, atmosphere_environment)
                        .chain()
                        .after(Core3dSystems::Prepass)
                        .before(Core3dSystems::MainPass),
                    render_sky
                        .after(main_opaque_pass_3d)
                        // superfargoth patch (own, LSO: light shaft occlusion)
                        .after(RenderSkyInputs)
                        // superfargoth patch (own: sky after water): the sky and the aerial
                        // perspective read the depth of transmissive draws too (the water, W-ORDER).
                        .after(crate::transmission::main_transmissive_pass_3d)
                        .before(main_transparent_pass_3d),
                ),
            );
    }
}

/// superfargoth patch (own, U3: sky occluded): a render-world view component saying that no sky
/// and no air lie between this view's camera and what it sees (its lens is fully under water:
/// `sf_water_bevy` inserts it from the camera's medium each frame). `render_sky` skips such a view
/// (neither the sky nor the aerial perspective is drawn: the underwater post fogs it instead, so
/// it is not fogged twice); other passes that fog by depth may read it too. Absent = stock.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SkyOccluded;

/// superfargoth patch (own, AS: aerial shadow), `superfargoth/docs/specs/aerial-shadow.md`: a
/// camera component giving the aerial view LUT a top-down height-field shadow for one directional
/// light, so haze in front of terrain the light cannot reach stops glowing. Only that light's
/// single scattering is shadowed (the multiple-scattering term is the sky's isotropic fill and stays
/// unshadowed); the sky-view LUT and the raymarched sky are unchanged. Absent (or `params.z` 0) =
/// the stock aerial view LUT pipeline.
///
/// Per LUT sample at render-space point `p` and camera `c`: `uv = (p.xz - params.xy) * params.z`,
/// texel `t = (shadow height - params.w, occluder distance)`; lit = `smoothstep(h - w, h + w, p.y +
/// bias + |p - c| * bias_per_m)` with `h = t.x + params.w`, `w = t.y * penumbra_per_m + penumbra_min`,
/// faded to lit from `fade_start` (fraction of the half extent) to the map's border.
#[derive(Clone, Component, Debug, PartialEq)]
pub struct AtmosphereHeightShadow {
    /// A filterable two-channel float texture (`Rg16Float`): (shadow height - base height,
    /// occluder distance), meters; texel (0, 0) at the map's min corner.
    pub image: Handle<Image>,
    /// Render space: (min x, min z, 1 / extent (1/m; 0 = none), base height).
    pub params: Vec4,
    /// Toward the light the map was made for (world space, unit). The directional light whose
    /// `direction_to_light` is within `dot > 0.99` of it is the one shadowed.
    pub light_dir: Vec3,
    /// Penumbra half width per meter of occluder distance.
    pub penumbra_per_m: f32,
    /// Penumbra half width floor, meters.
    pub penumbra_min: f32,
    /// Height bias, meters.
    pub bias: f32,
    /// Height bias growth per meter of distance from the camera.
    pub bias_per_m: f32,
    /// Where the fade to lit starts, as a fraction of the map's half extent (`0..1`).
    pub fade_start: f32,
}

/// superfargoth patch (own, AS: aerial shadow): the render-world uniform of
/// [`AtmosphereHeightShadow`] (`aerial_view_lut.wesl` `AtmosphereHeightShadow`; field order is
/// the layout).
#[derive(Clone, Component, ShaderType, Debug, PartialEq)]
pub struct GpuAtmosphereHeightShadow {
    pub params: Vec4,
    pub light_dir: Vec3,
    pub penumbra_per_m: f32,
    pub penumbra_min: f32,
    pub bias: f32,
    pub bias_per_m: f32,
    pub fade_start: f32,
}

impl From<&AtmosphereHeightShadow> for GpuAtmosphereHeightShadow {
    fn from(s: &AtmosphereHeightShadow) -> Self {
        Self {
            params: s.params,
            light_dir: s.light_dir,
            penumbra_per_m: s.penumbra_per_m,
            penumbra_min: s.penumbra_min,
            bias: s.bias,
            bias_per_m: s.bias_per_m,
            fade_start: s.fade_start,
        }
    }
}

/// superfargoth patch (own, AS: aerial shadow): the render-world image of
/// [`AtmosphereHeightShadow`] (bound with its own sampler; not yet uploaded = stock pass).
#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub struct AtmosphereHeightShadowImage(pub AssetId<Image>);

/// superfargoth patch (own, AS: aerial shadow): the shader def of the shadowed aerial view LUT.
pub const ATMOSPHERE_HEIGHT_SHADOW_DEF: &str = "ATMOSPHERE_HEIGHT_SHADOW";

/// superfargoth patch (own, LSO: light shaft occlusion), `superfargoth/docs/specs/aerial-shadow.md`
/// "S3": a render-world view component giving `render_sky` a screen-space light shaft occlusion
/// (UE "Light Shaft Occlusion", Mitchell 2007): `occlusion` is a filterable float texture over the
/// view's (internal-resolution) viewport whose red channel `o` in `[0, 1]` says how much of the
/// way from the pixel to the light is open sky; `uniform` a buffer holding one
/// [`GpuAtmosphereLightShafts`]. A3 key split ("Architecture review"): the aerial view LUT pass
/// also writes [`AtmosphereTextures::aerial_view_key_lut`](resources::AtmosphereTextures), the
/// single scattering of the directional light within `dot > 0.99` of `light_dir` alone, and on
/// geometry pixels `render_sky` computes `total - (1 - v) * min(key, total)` with
/// `v = 1 - darkness * (1 - o) * lobe`, `lobe` = Henyey-Greenstein (`lobe_g`) toward `light_dir`
/// normalised to 1 at the light (exactly 1 at `lobe_g` 0). Multiple scattering, the Rayleigh fill
/// and the other lights are never masked; sky pixels are unchanged. Inserted by the app's own
/// render systems before [`RenderSkyInputs`] runs (`RenderSystems::PrepareResources`, so the bind
/// group sees it); absent = the stock aerial view LUT and `render_sky` pipelines and bind groups.
#[derive(Clone, Component)]
pub struct AtmosphereLightShafts {
    /// The occlusion texture, sampled at the fullscreen UV with the atmosphere LUT sampler.
    pub occlusion: bevy_render::render_resource::TextureView,
    /// A uniform buffer of one [`GpuAtmosphereLightShafts`].
    pub uniform: bevy_render::render_resource::Buffer,
}

/// superfargoth patch (own, LSO: light shaft occlusion): the uniform of
/// [`AtmosphereLightShafts`] (`render_sky.wesl` `AtmosphereLightShafts`; field order is the
/// layout).
#[derive(Clone, Copy, ShaderType, Debug, Default, PartialEq)]
pub struct GpuAtmosphereLightShafts {
    /// Toward the light whose forward lobe is masked (world space, unit).
    pub light_dir: Vec3,
    /// Occlusion strength in `[0, 1]`: 0 = stock, 1 = a fully closed pixel looking at the light
    /// gets no in-scattering.
    pub darkness: f32,
    /// The lobe's Henyey-Greenstein asymmetry (the haze's Mie asymmetry).
    pub lobe_g: f32,
}

/// superfargoth patch (own, LSO: light shaft occlusion): the shader def of `render_sky` with the
/// light shaft occlusion.
pub const ATMOSPHERE_LIGHT_SHAFTS_DEF: &str = "ATMOSPHERE_LIGHT_SHAFTS";

/// superfargoth patch (own, LSO: light shaft occlusion, A3 key split): the shader def of the
/// aerial view LUT that also writes the key light's single scattering (views with an
/// [`AtmosphereLightShafts`]; combines with [`ATMOSPHERE_HEIGHT_SHADOW_DEF`], whose height shadow
/// the key LUT then carries too, so `total - key` stays the unmasked rest).
pub const ATMOSPHERE_KEY_SINGLE_DEF: &str = "ATMOSPHERE_KEY_SINGLE";

/// superfargoth patch (own, LSO: light shaft occlusion): `Core3d` systems that write what
/// `render_sky` reads (an [`AtmosphereLightShafts`] texture) go in this set; `render_sky` runs
/// after it. A public name for that order (`render_sky` itself is private).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderSkyInputs;

/// For each camera with [`AtmosphereSettings`], picks the nearest [`Atmosphere`] by world-space
/// distance to its origin, copies it as [`ExtractedAtmosphere`], and builds [`GpuAtmosphereSettings`].
pub fn extract_atmosphere(
    mut commands: Commands,
    atmosphere_entities: Extract<Query<(Entity, &Atmosphere, &GlobalTransform)>>,
    cameras: Extract<
        Query<
            (
                RenderEntity,
                Option<&AtmosphereSettings>,
                &GlobalTransform,
                // superfargoth patch (own, AS: aerial shadow)
                Option<&AtmosphereHeightShadow>,
            ),
            With<Camera3d>,
        >,
    >,
    // superfargoth patch (own, P1: live medium scales)
    medium_scales: Extract<Query<&crate::ScatteringMediumScales>>,
) {
    let candidates: Vec<(Entity, &Atmosphere, &GlobalTransform)> =
        atmosphere_entities.iter().collect();

    for (render_entity, settings, cam_global, height_shadow) in &cameras {
        // superfargoth patch (own, AS: aerial shadow): the view's height shadow, or none (a map
        // with no extent is none); a stale one (and its uniform index) never outlives it.
        match height_shadow.filter(|s| s.params.z > 0.0 && settings.is_some()) {
            Some(s) => {
                commands.entity(render_entity).insert((
                    GpuAtmosphereHeightShadow::from(s),
                    AtmosphereHeightShadowImage(s.image.id()),
                ));
            }
            None => {
                commands.entity(render_entity).try_remove::<(
                    GpuAtmosphereHeightShadow,
                    AtmosphereHeightShadowImage,
                    DynamicUniformIndex<GpuAtmosphereHeightShadow>,
                )>();
            }
        }
        // Remove any stale render-world state when AtmosphereSettings is removed or candidates is empty
        let (Some(settings), false) = (settings, candidates.is_empty()) else {
            commands.entity(render_entity).try_remove::<(
                ExtractedAtmosphere,
                GpuAtmosphereSettings,
                GpuAtmosphere,
                AtmosphereTextures,
                AtmosphereTransformsOffset,
                RenderSkyPipelineId,
                AtmosphereBindGroups,
            )>();
            continue;
        };

        let cam_world = cam_global.translation();
        let selected = candidates
            .iter()
            .min_by(|(ea, _, gt_a), (eb, _, gt_b)| {
                let da = cam_world.distance(gt_a.translation());
                let db = cam_world.distance(gt_b.translation());
                da.total_cmp(&db).then_with(|| ea.cmp(eb))
            })
            .expect("checked non-empty above");
        let atmo = selected.1;
        let gt = selected.2;
        // Atmospheres are spherically symmetric, so ignore their orientation.
        let (scale, _, translation) = gt.to_scale_rotation_translation();

        let extracted = ExtractedAtmosphere {
            inner_radius: atmo.inner_radius,
            outer_radius: atmo.outer_radius,
            ground_albedo: atmo.ground_albedo,
            medium: atmo.medium.id(),
            world_to_atmosphere: Mat4::from_scale_rotation_translation(
                scale,
                Quat::IDENTITY,
                translation,
            )
            .inverse(),
            // superfargoth patch (own, P1: live medium scales)
            medium_scales: medium_scales.get(selected.0).copied().unwrap_or_default(),
        };
        commands.entity(render_entity).insert(extracted);
        commands
            .entity(render_entity)
            .insert(GpuAtmosphereSettings::from(settings.clone()));
    }
}

/// The render-world representation of an `Atmosphere`, but which
/// hasn't been converted into shader uniforms yet.
#[derive(Clone, Component)]
pub struct ExtractedAtmosphere {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub ground_albedo: Vec3,
    pub medium: AssetId<ScatteringMedium>,
    pub world_to_atmosphere: Mat4,
    /// superfargoth patch (own, P1: live medium scales): the atmosphere entity's
    /// [`crate::ScatteringMediumScales`] (all one when it has none).
    pub medium_scales: crate::ScatteringMediumScales,
}

/// This component controls the resolution of the atmosphere LUTs, and
/// how many samples are used when computing them.
///
/// The transmittance LUT stores the transmittance from a point in the
/// atmosphere to the outer edge of the atmosphere in any direction,
/// parametrized by the point's radius and the cosine of the zenith angle
/// of the ray.
///
/// The multiscattering LUT stores the factor representing luminance scattered
/// towards the camera with scattering order >2, parametrized by the point's radius
/// and the cosine of the zenith angle of the sun.
///
/// The sky-view lut is essentially the actual skybox, storing the light scattered
/// towards the camera in every direction with a cubemap.
///
/// The aerial-view lut is a 3d LUT fit to the view frustum, which stores the luminance
/// scattered towards the camera at each point (RGB channels), alongside the average
/// transmittance to that point (A channel).
#[derive(Clone, Component, Reflect)]
#[reflect(Clone, Default)]
#[require(Hdr)]
pub struct AtmosphereSettings {
    /// The size of the transmittance LUT
    pub transmittance_lut_size: UVec2,

    /// The size of the multiscattering LUT
    pub multiscattering_lut_size: UVec2,

    /// The size of the sky-view LUT.
    pub sky_view_lut_size: UVec2,

    /// The size of the aerial-view LUT.
    pub aerial_view_lut_size: UVec3,

    /// The number of points to sample along each ray when
    /// computing the transmittance LUT
    pub transmittance_lut_samples: u32,

    /// The number of rays to sample when computing each
    /// pixel of the multiscattering LUT
    pub multiscattering_lut_dirs: u32,

    /// The number of points to sample when integrating along each
    /// multiscattering ray
    pub multiscattering_lut_samples: u32,

    /// The number of points to sample along each ray when
    /// computing the sky-view LUT.
    pub sky_view_lut_samples: u32,

    /// The number of points to sample for each slice along the z-axis
    /// of the aerial-view LUT.
    pub aerial_view_lut_samples: u32,

    /// The maximum distance from the camera to evaluate the
    /// aerial view LUT. The slices along the z-axis of the
    /// texture will be distributed linearly from the camera
    /// to this value.
    ///
    /// units: m
    pub aerial_view_lut_max_distance: f32,

    /// The number of points to sample for each fragment when the using
    /// ray marching to render the sky
    pub sky_max_samples: u32,

    /// The rendering method to use for the atmosphere.
    pub rendering_method: AtmosphereMode,

    /// superfargoth patch (own, N2: sky glow): the sky's own radiance at the zenith, linear
    /// RGB in cd/m² (airglow, integrated starlight and scattered moonlight as one artistic
    /// term; a few mcd/m² physically, games raise it). Added to every sky ray of the sky-view
    /// LUT and of the ray-marched sky, so the sky, its environment map and everything that
    /// samples the LUT (fog, clouds) see it. Zero (default) is stock Bevy.
    pub airglow: Vec3,
    /// superfargoth patch (own, N2): horizon over zenith radiance of [`Self::airglow`]
    /// (a van Rhijn profile: a thin emitting layer seen at a slant).
    pub airglow_horizon: f32,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self {
            transmittance_lut_size: UVec2::new(256, 128),
            transmittance_lut_samples: 40,
            multiscattering_lut_size: UVec2::new(32, 32),
            multiscattering_lut_dirs: 64,
            multiscattering_lut_samples: 20,
            sky_view_lut_size: UVec2::new(400, 200),
            sky_view_lut_samples: 16,
            aerial_view_lut_size: UVec3::new(32, 32, 32),
            aerial_view_lut_samples: 10,
            aerial_view_lut_max_distance: 3.2e4,
            sky_max_samples: 16,
            rendering_method: AtmosphereMode::LookupTexture,
            // superfargoth patch (own, N2: sky glow)
            airglow: Vec3::ZERO,
            airglow_horizon: 3.0,
        }
    }
}

#[derive(Clone, Component, Reflect, ShaderType)]
#[reflect(Default)]
pub struct GpuAtmosphereSettings {
    pub transmittance_lut_size: UVec2,
    pub multiscattering_lut_size: UVec2,
    pub sky_view_lut_size: UVec2,
    pub aerial_view_lut_size: UVec3,
    pub transmittance_lut_samples: u32,
    pub multiscattering_lut_dirs: u32,
    pub multiscattering_lut_samples: u32,
    pub sky_view_lut_samples: u32,
    pub aerial_view_lut_samples: u32,
    pub aerial_view_lut_max_distance: f32,
    pub sky_max_samples: u32,
    pub rendering_method: u32,
    // superfargoth patch (own, N2: sky glow)
    pub airglow: Vec3,
    pub airglow_horizon: f32,
}

impl Default for GpuAtmosphereSettings {
    fn default() -> Self {
        AtmosphereSettings::default().into()
    }
}

impl From<AtmosphereSettings> for GpuAtmosphereSettings {
    fn from(s: AtmosphereSettings) -> Self {
        Self {
            transmittance_lut_size: s.transmittance_lut_size,
            multiscattering_lut_size: s.multiscattering_lut_size,
            sky_view_lut_size: s.sky_view_lut_size,
            aerial_view_lut_size: s.aerial_view_lut_size,
            transmittance_lut_samples: s.transmittance_lut_samples,
            multiscattering_lut_dirs: s.multiscattering_lut_dirs,
            multiscattering_lut_samples: s.multiscattering_lut_samples,
            sky_view_lut_samples: s.sky_view_lut_samples,
            aerial_view_lut_samples: s.aerial_view_lut_samples,
            aerial_view_lut_max_distance: s.aerial_view_lut_max_distance,
            sky_max_samples: s.sky_max_samples,
            rendering_method: s.rendering_method as u32,
            // superfargoth patch (own, N2: sky glow)
            airglow: s.airglow,
            airglow_horizon: s.airglow_horizon,
        }
    }
}

impl SyncComponent<RenderApp> for AtmosphereSettings {
    type Target = GpuAtmosphereSettings;
}

fn configure_camera_depth_usages(
    mut cameras: Query<&mut Camera3d, (Changed<Camera3d>, With<ExtractedAtmosphere>)>,
) {
    for mut camera in &mut cameras {
        camera.depth_texture_usages.0 |= TextureUsages::TEXTURE_BINDING.bits();
    }
}

/// Selects how the atmosphere is rendered. Choose based on scene scale and
/// volumetric shadow quality, and based on performance needs.
#[repr(u32)]
#[derive(Clone, Default, Reflect, Copy)]
pub enum AtmosphereMode {
    /// High-performance solution tailored to scenes that are mostly inside of the atmosphere.
    /// Uses a set of lookup textures to approximate scattering integration.
    /// Slightly less accurate for very long-distance/space views (lighting precision
    /// tapers as the camera moves far from the scene origin) and for sharp volumetric
    /// (cloud/fog) shadows.
    #[default]
    LookupTexture = 0,
    /// Slower, more accurate rendering method for any type of scene.
    /// Integrates the scattering numerically with raymarching and produces sharp volumetric
    /// (cloud/fog) shadows.
    /// Best for cinematic shots, planets seen from orbit, and scenes requiring
    /// accurate long-distance lighting.
    Raymarched = 1,
}
