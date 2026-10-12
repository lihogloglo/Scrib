//! Bevy plugin for [`sf_sky`]. Spec: `docs/specs/sky-lighting.md`.
//!
//! - [`SkyPlugin`]: advances the [`SkyTime`] clock, evaluates the sky into
//!   [`CurrentSky`], and drives Bevy's `Atmosphere`, a sun and a moon
//!   (`DirectionalLight` + `SunDisk`), and the exposure and ambient floor
//!   of every [`SkyCamera`].
//! - Render path ([`RenderPathSetting`] → [`ActiveRenderPath`]): raster or
//!   ray-traced (Bevy Solari, cargo feature `solari`), switchable at
//!   runtime, falling back to raster when the GPU lacks ray queries.
//! - Upscaling ([`UpscalingSetting`] → [`ActiveUpscaling`]): DLSS Super
//!   Resolution on raster and Ray Reconstruction on the ray-traced path
//!   (cargo feature `dlss`, NVIDIA RTX), off elsewhere.
//! - [`PointLightProxy`]: punctual lights become emissive spheres for the
//!   ray tracer, which doesn't support point lights yet.
//! - Night sky (`night.rs`): stars, moons (textured per phase or shaded)
//!   and the sun's glare, drawn into the sky before any cloud composite.
//!   The sun disk's drawn size is [`sf_sky::SunDiskLook`], not the light's.
//! - [`PointShadowBudget`]: shadow maps for the few most important
//!   [`BudgetedShadow`] point lights, faded by distance and over time.

mod night;
mod path;
mod proxy;
mod shadow_budget;
mod upscale;

// Headless: the `solari` feature pulls in render-world plugins.
#[cfg(all(test, not(feature = "solari")))]
mod tests;

use bevy_app::{App, Plugin, PostUpdate, Startup, Update};
use bevy_asset::{Assets, Handle};
use bevy_camera::{Camera3d, Exposure};
use bevy_color::{Color, LinearRgba};
use bevy_ecs::prelude::*;
use bevy_light::atmosphere::{Falloff, PhaseFunction, ScatteringMedium, ScatteringTerm};
use bevy_light::{
    AmbientLight, Atmosphere, AtmosphereEnvironmentMapLight, CascadeShadowConfig,
    CascadeShadowConfigBuilder, DirectionalLight, SunDisk,
};
use bevy_math::{Dir3, Vec3};
use bevy_pbr::AtmosphereSettings;
use bevy_time::Time;
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_transform::TransformSystems;

pub use sf_sky;
use sf_sky::{RenderPath, SkyClock, SkyLight, SkySettings, SkyState, Weather};

pub use path::{NoRayTracing, RayTracingMesh, RayTracingSupport, RenderPathCamera};
pub use proxy::{LightProxy, NoLightProxy, PointLightProxy};
pub use shadow_budget::{BudgetedShadow, PointShadowBudget, PointShadowPlugin, ShadowSlot, ShadowTwin};
pub use upscale::{ActiveUpscaling, TemporalHistory, UpscalingSetting, UpscalingSupport};

/// The sky's settings (orbit, atmosphere, curves). Change at will; the
/// atmosphere medium is rebuilt when its description changes.
#[derive(Resource, Clone, Debug, Default)]
pub struct SkyConfig(pub SkySettings);

/// The game clock. Saves store `SkyTime.0.days`.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SkyTime(pub SkyClock);

/// Current weather (M2: cloud dimming, haze, wind; M6 blends presets here).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WeatherState(pub Weather);

/// The sky evaluated this frame. Read-only for everyone but the plugin.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CurrentSky(pub SkyState);

/// The render path the player asked for (a graphics setting).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderPathSetting(pub RenderPath);

/// The render path actually running: [`RenderPathSetting`] if the GPU and
/// build support it, otherwise raster.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActiveRenderPath(pub RenderPath);

/// Engine-side options that aren't sky data.
#[derive(Resource, Clone)]
pub struct SkyRenderOptions {
    /// Render-space Y of the atmosphere's ground (sea level). The world
    /// glue updates it when the floating origin changes cell vertically.
    pub sea_level: f32,
    /// Shadow cascades of the sun and moon on the raster path.
    pub cascades: CascadeShadowConfigBuilder,
    /// Tint of the ambient floor ([`SkySettings::ambient`]).
    pub ambient_color: Color,
    /// Resolution of the medium's density and phase LUTs.
    pub medium_resolution: u32,
    /// Every `PointLight` gets a [`PointLightProxy`] unless it has
    /// [`NoLightProxy`].
    pub auto_light_proxies: bool,
    /// Proxy radius for point lights whose `radius` is 0, meters.
    pub default_proxy_radius: f32,
    /// On the ray-traced path, every `Mesh3d` with a `StandardMaterial`
    /// joins the ray-tracing scene unless it has [`NoRayTracing`].
    pub auto_ray_tracing_meshes: bool,
    /// Solari's `ReSTIR` resampling. `None` (default): on without a denoiser,
    /// where it is the difference between a usable image and heavy noise;
    /// off under DLSS Ray Reconstruction, which copes with raw samples (Bevy's
    /// own advice) and saves its cost.
    pub ray_tracing_restir: Option<bool>,
    /// DLSS Ray Reconstruction on the ray-traced path when the build has
    /// the `dlss` feature and the GPU supports it (NVIDIA RTX).
    pub ray_tracing_denoiser: bool,
}

impl Default for SkyRenderOptions {
    fn default() -> Self {
        Self {
            sea_level: 0.0,
            cascades: CascadeShadowConfigBuilder {
                num_cascades: 4,
                minimum_distance: 0.1,
                first_cascade_far_bound: 12.0,
                maximum_distance: 300.0,
                overlap_proportion: 0.2,
            },
            ambient_color: Color::linear_rgb(0.55, 0.65, 1.0),
            medium_resolution: 256,
            auto_light_proxies: true,
            default_proxy_radius: 0.08,
            auto_ray_tracing_meshes: true,
            ray_tracing_restir: None,
            ray_tracing_denoiser: true,
        }
    }
}

/// Put this on the 3D camera(s) that see the sky. The plugin adds Bevy's
/// `AtmosphereSettings` (sky and aerial perspective), optionally an
/// `AtmosphereEnvironmentMapLight` (sky ambient and reflections), and
/// drives `Exposure` and `AmbientLight`. Remove it (e.g. in interiors) and
/// the atmosphere components go away; the sun and moon go dark once no
/// sky camera is left.
#[derive(Component, Clone, Copy, Debug)]
#[require(Camera3d, RenderPathCamera)]
pub struct SkyCamera {
    /// Drive `Exposure` from [`SkySettings::exposure`].
    pub exposure: bool,
    /// Drive `AmbientLight` from [`SkySettings::ambient`].
    pub ambient: bool,
    /// Light the scene from the sky (`AtmosphereEnvironmentMapLight`).
    pub environment_light: bool,
}

impl Default for SkyCamera {
    fn default() -> Self {
        Self {
            exposure: true,
            ambient: true,
            environment_light: true,
        }
    }
}

/// The sun's `DirectionalLight` (spawned by the plugin).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SunLight;

/// The moon's `DirectionalLight` (spawned by the plugin).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MoonLight;

/// The `Atmosphere` entity (spawned by the plugin).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SkyAtmosphere;

/// Ordering of the plugin's systems (in `Update`, except `Origin`).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SkySystems {
    /// Advance [`SkyTime`].
    Clock,
    /// Resolve and apply the render path.
    RenderPath,
    /// Evaluate [`CurrentSky`] and apply it to lights and cameras.
    Apply,
    /// `PostUpdate`, after transform propagation: place the planet under
    /// [`SkyRenderOptions::sea_level`]. A floating origin that moves the sea
    /// level runs before this set, so the atmosphere follows in the same frame.
    Origin,
}

/// The sky plugin. Insert [`SkyConfig`], [`SkyTime`], etc. before adding it
/// to override the defaults, or set the fields here.
#[derive(Default)]
pub struct SkyPlugin {
    pub settings: SkySettings,
    pub clock: SkyClock,
    /// Requested render path at startup.
    pub render_path: RenderPath,
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<SkyConfig>() {
            app.insert_resource(SkyConfig(self.settings.clone()));
        }
        if !app.world().contains_resource::<SkyTime>() {
            app.insert_resource(SkyTime(self.clock));
        }
        if !app.world().contains_resource::<RenderPathSetting>() {
            app.insert_resource(RenderPathSetting(self.render_path));
        }
        app.init_resource::<WeatherState>()
            .init_resource::<SkyRenderOptions>()
            .init_resource::<CurrentSky>()
            .init_resource::<ActiveRenderPath>()
            .init_resource::<MediumState>()
            .configure_sets(
                Update,
                (SkySystems::Clock, SkySystems::RenderPath, SkySystems::Apply).chain(),
            )
            .add_systems(Startup, spawn_sky);
        if !app.is_plugin_added::<PointShadowPlugin>() {
            app.add_plugins(PointShadowPlugin);
        }
        app.add_systems(Update, advance_clock.in_set(SkySystems::Clock))
            .add_systems(
                Update,
                (
                    evaluate_sky,
                    (
                        update_medium,
                        setup_sky_cameras,
                        apply_lights,
                        apply_sun_disk,
                        apply_cameras,
                    ),
                )
                    .chain()
                    .in_set(SkySystems::Apply),
            )
            .add_systems(
                PostUpdate,
                place_atmosphere.in_set(SkySystems::Origin).after(TransformSystems::Propagate),
            );
        night::build(app);
        path::build(app);
        proxy::build(app);
        upscale::build(app);
    }

    fn finish(&self, app: &mut App) {
        night::finish(app);
        path::finish(app);
        upscale::finish(app);
    }
}

/// The medium asset and the inputs it was built from.
#[derive(Resource, Default)]
struct MediumState {
    handle: Handle<ScatteringMedium>,
    built_from: Option<(sf_sky::AtmosphereModel, f32)>,
}

/// Haze changes smaller than this don't rebuild the medium's LUTs.
const HAZE_REBUILD_STEP: f32 = 0.02;

fn build_medium(model: &sf_sky::AtmosphereModel, haze: f32, resolution: u32) -> ScatteringMedium {
    let terms = model.terms.iter().map(|t| {
        let scale = if t.weather_scaled { haze } else { 1.0 };
        ScatteringTerm {
            absorption: t.absorption * scale,
            scattering: t.scattering * scale,
            falloff: match t.falloff {
                sf_sky::Falloff::Linear => Falloff::Linear,
                sf_sky::Falloff::Exponential { scale } => Falloff::Exponential { scale },
                sf_sky::Falloff::Tent { center, width } => Falloff::Tent { center, width },
            },
            phase: match t.phase {
                sf_sky::Phase::Isotropic => PhaseFunction::Isotropic,
                sf_sky::Phase::Rayleigh => PhaseFunction::Rayleigh,
                sf_sky::Phase::Mie { asymmetry } => PhaseFunction::Mie { asymmetry },
            },
        }
    });
    ScatteringMedium::new(resolution, resolution, terms).with_label("sf_sky")
}

fn spawn_sky(
    mut commands: Commands,
    config: Res<SkyConfig>,
    options: Res<SkyRenderOptions>,
    weather: Res<WeatherState>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut medium: ResMut<MediumState>,
    existing: Query<(Has<SunLight>, Has<SkyAtmosphere>)>,
) {
    // Lights or an atmosphere the app spawned itself are kept.
    let (has_sun, has_atmosphere) = existing
        .iter()
        .fold((false, false), |(s, a), (sun, atm)| (s || sun, a || atm));
    let settings = &config.0;
    let model = &settings.atmosphere;
    medium.handle = media.add(build_medium(
        model,
        weather.0.haze,
        options.medium_resolution,
    ));
    medium.built_from = Some((model.clone(), weather.0.haze));
    if !has_atmosphere {
        commands.spawn((
            SkyAtmosphere,
            Atmosphere {
                inner_radius: model.inner_radius,
                outer_radius: model.outer_radius,
                ground_albedo: model.ground_albedo,
                medium: medium.handle.clone(),
            },
            atmosphere_transform(model.inner_radius, options.sea_level),
        ));
    }
    if !has_sun {
        let cascades: CascadeShadowConfig = options.cascades.build();
        commands.spawn((
            SunLight,
            DirectionalLight {
                shadow_maps_enabled: true,
                ..DirectionalLight::default()
            },
            sun_disk(settings),
            cascades.clone(),
            Transform::default(),
        ));
        commands.spawn((
            MoonLight,
            DirectionalLight {
                color: Color::linear_rgb(0.9, 0.93, 1.0),
                shadow_maps_enabled: false,
                ..DirectionalLight::default()
            },
            // Moons are drawn by the night pass (phases, textures); the
            // light keeps a zero-intensity disk (no disk means Bevy's default).
            SunDisk {
                angular_size: settings.moon_angular_size,
                intensity: 0.0,
            },
            cascades,
            Transform::default(),
        ));
    }
}

fn sun_disk(settings: &SkySettings) -> SunDisk {
    SunDisk {
        angular_size: settings.sun_disk.angular_size,
        intensity: settings.sun_disk.intensity,
    }
}

/// The drawn sun disk follows [`SkySettings::sun_disk`].
fn apply_sun_disk(config: Res<SkyConfig>, mut disks: Query<&mut SunDisk, With<SunLight>>) {
    if config.is_changed() {
        for mut disk in &mut disks {
            *disk = sun_disk(&config.0);
        }
    }
}

fn atmosphere_transform(inner_radius: f32, sea_level: f32) -> GlobalTransform {
    GlobalTransform::from_translation(Vec3::new(0.0, sea_level - inner_radius, 0.0))
}

fn advance_clock(time: Res<Time>, config: Res<SkyConfig>, mut clock: ResMut<SkyTime>) {
    let calendar = config.0.calendar;
    clock.0.advance(time.delta_secs_f64(), &calendar);
}

fn evaluate_sky(
    config: Res<SkyConfig>,
    clock: Res<SkyTime>,
    weather: Res<WeatherState>,
    mut sky: ResMut<CurrentSky>,
) {
    sky.0 = config.0.evaluate(&clock.0, &weather.0);
}

/// Rebuilds the medium when the atmosphere description or the haze change.
fn update_medium(
    config: Res<SkyConfig>,
    options: Res<SkyRenderOptions>,
    weather: Res<WeatherState>,
    mut medium: ResMut<MediumState>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut atmospheres: Query<&mut Atmosphere, With<SkyAtmosphere>>,
) {
    let model = &config.0.atmosphere;
    let haze = weather.0.haze;
    let stale = match &medium.built_from {
        Some((built, built_haze)) => {
            built != model || (built_haze - haze).abs() >= HAZE_REBUILD_STEP
        }
        None => true,
    };
    if stale && let Some(mut asset) = media.get_mut(&medium.handle) {
        *asset = build_medium(model, haze, options.medium_resolution);
        medium.built_from = Some((model.clone(), haze));
    }
    if config.is_changed() {
        for mut atmosphere in &mut atmospheres {
            atmosphere.inner_radius = model.inner_radius;
            atmosphere.outer_radius = model.outer_radius;
            atmosphere.ground_albedo = model.ground_albedo;
        }
    }
}

/// Keeps the planet under the sea level. In `PostUpdate` so a sea level set
/// by a floating-origin shift that frame (after propagation) applies before
/// extraction; a frame late, the atmosphere and the cloud shell jump by a cell.
fn place_atmosphere(
    config: Res<SkyConfig>,
    options: Res<SkyRenderOptions>,
    mut atmospheres: Query<&mut GlobalTransform, With<SkyAtmosphere>>,
) {
    if config.is_changed() || options.is_changed() {
        for mut transform in &mut atmospheres {
            *transform = atmosphere_transform(config.0.atmosphere.inner_radius, options.sea_level);
        }
    }
}

/// Adds the atmosphere components to new sky cameras and removes them from
/// cameras that stopped being one.
fn setup_sky_cameras(
    mut commands: Commands,
    sky: Res<CurrentSky>,
    added: Query<(Entity, &SkyCamera), Added<SkyCamera>>,
    mut removed: RemovedComponents<SkyCamera>,
    cameras: Query<(), With<Camera3d>>,
) {
    for (entity, sky_camera) in &added {
        let mut e = commands.entity(entity);
        // The scheduled exposure right away: `apply_cameras` runs before
        // these commands apply, and a default `Exposure` (EV100 9.7) would
        // show for one frame (a flash at noon when a camera becomes a sky
        // camera, e.g. walking out of an interior).
        e.insert(AtmosphereSettings::default());
        if sky_camera.exposure {
            e.insert(Exposure { ev100: sky.0.ev100 });
        } else {
            e.insert_if_new(Exposure::default());
        }
        if sky_camera.environment_light {
            e.insert(AtmosphereEnvironmentMapLight::default());
        }
    }
    for entity in removed.read() {
        if cameras.contains(entity) {
            commands
                .entity(entity)
                .try_remove::<(AtmosphereSettings, AtmosphereEnvironmentMapLight)>();
        }
    }
}

fn light_transform(direction: Vec3) -> Transform {
    // A directional light shines along its forward (−Z).
    let up = if direction.y.abs() > 0.999 {
        Dir3::Z
    } else {
        Dir3::Y
    };
    Transform::default().looking_to(-direction, up)
}

/// Writes one sky light into its `DirectionalLight`.
#[expect(clippy::float_cmp, reason = "exact: only skips redundant writes")]
fn write_light(
    light: &mut DirectionalLight,
    transform: &mut Transform,
    sky: &SkyLight,
    shader_applies_transmittance: bool,
    active: bool,
) {
    *transform = light_transform(sky.direction);
    let (tint, illuminance) = if !active {
        (Vec3::ONE, 0.0)
    } else if shader_applies_transmittance {
        // The raster PBR shader applies the atmosphere's transmittance per
        // pixel, and the sky needs the unattenuated light (twilight).
        (Vec3::ONE, sky.illuminance)
    } else {
        sky.ground_light()
    };
    let shadows = active && sky.casts_shadows && shader_applies_transmittance;
    if light.illuminance != illuminance {
        light.illuminance = illuminance;
    }
    if shader_applies_transmittance {
        // Keep the authored tint (the moon's is bluish).
    } else {
        light.color = Color::LinearRgba(LinearRgba::rgb(tint.x, tint.y, tint.z));
    }
    if light.shadow_maps_enabled != shadows {
        light.shadow_maps_enabled = shadows;
    }
}

type LightItem = (&'static mut DirectionalLight, &'static mut Transform);

fn apply_lights(
    sky: Res<CurrentSky>,
    path: Res<ActiveRenderPath>,
    sky_cameras: Query<(), With<SkyCamera>>,
    mut suns: Query<LightItem, (With<SunLight>, Without<MoonLight>)>,
    mut moons: Query<LightItem, (With<MoonLight>, Without<SunLight>)>,
) {
    let active = !sky_cameras.is_empty();
    // Solari's directional lights ignore the atmosphere: give them the
    // transmitted light (and no shadow maps: ray tracing replaces them).
    let raster = path.0 == RenderPath::Raster;
    for (mut light, mut transform) in &mut suns {
        write_light(&mut light, &mut transform, &sky.0.sun, raster, active);
    }
    for (mut light, mut transform) in &mut moons {
        write_light(&mut light, &mut transform, &sky.0.moon, raster, active);
    }
}

#[expect(clippy::float_cmp, reason = "exact: only skips redundant writes")]
fn apply_cameras(
    sky: Res<CurrentSky>,
    options: Res<SkyRenderOptions>,
    mut cameras: Query<(
        &SkyCamera,
        Option<&mut Exposure>,
        Option<&mut AmbientLight>,
        Entity,
    )>,
    mut commands: Commands,
) {
    for (sky_camera, exposure, ambient, entity) in &mut cameras {
        if sky_camera.exposure
            && let Some(mut exposure) = exposure
        {
            exposure.ev100 = sky.0.ev100;
        }
        if sky_camera.ambient {
            let brightness = sky.0.ambient;
            match ambient {
                Some(mut ambient) => {
                    if ambient.brightness != brightness {
                        ambient.brightness = brightness;
                        ambient.color = options.ambient_color;
                    }
                }
                None => {
                    commands.entity(entity).insert(AmbientLight {
                        color: options.ambient_color,
                        brightness,
                        affects_lightmapped_meshes: true,
                    });
                }
            }
        }
    }
}
