//! Volumetric clouds for Bevy (weather.md "Clouds"): the thin render plugin
//! over [`sf_atmos`].
//!
//! - [`CloudsPlugin`] generates the tiling noise (background thread), keeps
//!   [`Clouds`] (the settings the weather drives) and the wind scroll, and
//!   renders the clouds for every camera with [`CloudCamera`] that also has
//!   Bevy's `AtmosphereSettings`:
//!   1. a raymarch at reduced resolution (one pixel of each 2×2 half-res
//!      block per frame, or every half-res pixel at Ultra), lit through the
//!      atmosphere LUTs, stopped by the main depth buffer;
//!   2. a temporal resolve into a half-resolution history (reprojection by
//!      cloud distance, wind and floating-origin shifts; neighborhood clamp);
//!   3. a depth-aware upsample composited over the sky and the opaque scene,
//!      after Bevy's `render_sky` and before transparents.
//! - Cloud shadows: a 1024² map along the sun direction, given to the light
//!   with [`CloudShadowLight`] as a tiled `DirectionalLightTexture` around
//!   the camera (every PBR material receives it). Toroidal (texels fixed in
//!   the world, the light anchored in 2 km steps) and time-sliced: one band
//!   of rows per frame (`CloudQuality::shadow_slices`, `sf_atmos::shadow`).
//! - Fog ([`Fog`], weather.md "Fog", F2): the same node draws the height fog
//!   and the ground-fog banks. The trace marches the banks' noise (16 steps,
//!   quadratic, to `FogSettings::far`) and stores a modulation per traced
//!   pixel; the composite applies both exponential layers' closed-form
//!   optical depth to each full-resolution pixel's own depth, lit by the key
//!   light and the sky-view LUT, in front of the clouds. Runs with the
//!   clouds off too ([`Clouds::enabled`]); costs nothing without fog.
//! - GPU timers: each pass is a `RenderDiagnosticsPlugin` span
//!   (`clouds_shadow`, `clouds_trace`, `clouds_resolve`, `clouds_composite`).
//! - Floating origin: the app reports the absolute position of the
//!   origin-relative frame in [`CloudsOrigin`]; texture offsets carry it as a
//!   phase (tiles divide 16 384 m), so clouds never move on origin shifts.
//!
//! Provenance: the density/lighting math in `shaders/` and the noise
//! generators in `sf_atmos` are ported from the owner's Megalophobia project
//! (MIT OR Apache-2.0, ARCHITECTURE §10, 2026-10-12); the architecture
//! (shell, reduced resolution, temporal, LUT lighting, shadow map) is new.

mod render;

use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, PostUpdate, Startup, Update};
use bevy_asset::{embedded_asset, Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_light::{DirectionalLight, DirectionalLightTexture};
use bevy_math::{DVec3, Vec2, Vec3};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_shader::load_shader_library;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use sf_atmos::scroll::TileOffsets;
use sf_atmos::{CloudScroll, CloudSettings, FogDrift, FogOffsets, FogSettings, NoiseVolumes, ShadowRows, ShadowSlicer, DETAIL_SIZE, SHAPE_SIZE, WEATHER_SIZE};

pub use sf_atmos;

/// Half the side of the cloud shadow map's coverage, meters (16 km square).
pub const SHADOW_HALF_EXTENT: f32 = 8_192.0;

/// The shadow light's world anchor moves in steps of this many meters
/// (the map is toroidal: texels stay put while the camera moves inside a cell).
pub const SHADOW_ANCHOR_CELL: f64 = 2_048.0;

/// The cloud renderer. `settings` seeds [`Clouds`]; `seed` the noise.
#[derive(Clone, Debug, Default)]
pub struct CloudsPlugin {
    pub settings: CloudSettings,
    /// Seeds [`Fog`] (default: no fog).
    pub fog: FogSettings,
    pub seed: u64,
}

/// What the clouds render with this frame. Weather writes `settings`.
#[derive(Resource, Clone, Debug)]
pub struct Clouds {
    pub settings: CloudSettings,
    /// Draw the clouds (off: nothing renders, no cost).
    pub enabled: bool,
    /// Cloud shadows on the [`CloudShadowLight`].
    pub shadows: bool,
}

/// Height fog, ground-fog banks and the far fog (weather.md "Fog", F2), drawn
/// by the same node as the clouds: the banks are marched in the cloud trace,
/// the analytic layers applied per pixel in the composite. Weather writes
/// `settings`; with no fog (or `enabled` off) the fog terms cost nothing.
#[derive(Resource, Clone, Copy, Debug)]
pub struct Fog {
    pub settings: FogSettings,
    pub enabled: bool,
}

/// The absolute world position (meters) of the origin of origin-relative
/// coordinates (`big_space`: the floating origin cell's corner). Apps without
/// a floating origin leave it at zero. Write it before [`CloudSystems::Drive`]
/// in the frame the origin moves, or the clouds jump by a cell for a frame.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CloudsOrigin(pub DVec3);

/// The plugin's systems.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CloudSystems {
    /// `PostUpdate`, after transform propagation: wind, texture offsets from
    /// [`CloudsOrigin`], the shadow light, and the frame's uniform data.
    Drive,
}

/// Render clouds for this camera (it also needs Bevy's `AtmosphereSettings`).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CloudCamera;

/// The directional light (the sun) that receives the cloud shadow map.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CloudShadowLight;

/// The noise and shadow-map images (the noise appears once generated).
#[derive(Resource, Clone, Debug, Default)]
pub struct CloudImages {
    pub noise: Option<NoiseImages>,
    pub shadow: Handle<Image>,
}

#[derive(Clone, Debug)]
pub struct NoiseImages {
    pub shape: Handle<Image>,
    pub detail: Handle<Image>,
    pub weather: Handle<Image>,
}

#[derive(Resource, Default)]
struct NoiseJob(Arc<Mutex<Option<NoiseVolumes>>>);

#[derive(Resource, Default)]
struct Scroll(CloudScroll);

#[derive(Resource, Default)]
struct Slicer(ShadowSlicer);

#[derive(Resource, Default)]
struct Drift(FogDrift);

/// Per-frame values the render world needs (extracted).
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct CloudFrame {
    pub active: bool,
    /// The clouds draw (`active` may hold for the fog alone).
    pub clouds: bool,
    pub settings: CloudSettings,
    /// The fog, when it draws.
    pub fog: Option<FogSettings>,
    pub fog_offsets: FogOffsets,
    pub offsets: TileOffsets,
    /// Wind displacement this frame (world X, Z), meters.
    pub wind_step: Vec2,
    pub origin: DVec3,
    pub mean_cover: f32,
    pub images: CloudImages,
    /// Shadow map basis: center, light-plane axes (× half extent), toward the light.
    pub shadow: Option<ShadowBasis>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ShadowBasis {
    pub center: Vec3,
    pub x: Vec3,
    pub y: Vec3,
    pub light_dir: Vec3,
    /// The camera (each texel holds its repeat nearest the camera).
    pub camera: Vec3,
    pub rows: ShadowRows,
}

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shaders/cloud_common.wesl");
        load_shader_library!(app, "shaders/cloud_density.wesl");
        load_shader_library!(app, "shaders/fog.wesl");
        load_shader_library!(app, "shaders/key_light.wesl");
        embedded_asset!(app, "shaders/cloud_trace.wesl");
        embedded_asset!(app, "shaders/cloud_resolve.wesl");
        embedded_asset!(app, "shaders/cloud_composite.wesl");
        embedded_asset!(app, "shaders/cloud_shadow.wesl");
        embedded_asset!(app, "shaders/fog_light.wesl");

        let job = NoiseJob::default();
        let slot = job.0.clone();
        let seed = self.seed;
        std::thread::Builder::new()
            .name("cloud noise".into())
            .spawn(move || {
                let started = std::time::Instant::now();
                let volumes = NoiseVolumes::generate(seed);
                log::info!("cloud noise generated in {:.0} ms", started.elapsed().as_secs_f64() * 1000.0);
                if let Ok(mut s) = slot.lock() {
                    *s = Some(volumes);
                }
            })
            .ok();

        app.insert_resource(Clouds { settings: self.settings, enabled: true, shadows: true })
            .insert_resource(Fog { settings: self.fog, enabled: true })
            .init_resource::<Drift>()
            .insert_resource(job)
            .init_resource::<Scroll>()
            .init_resource::<Slicer>()
            .init_resource::<CloudsOrigin>()
            .init_resource::<CloudImages>()
            .init_resource::<CloudFrame>()
            .add_systems(Startup, create_shadow_image)
            .add_systems(Update, poll_noise)
            .add_systems(PostUpdate, drive.in_set(CloudSystems::Drive).after(TransformSystems::Propagate));
        render::build(app);
    }

    fn finish(&self, app: &mut App) {
        render::finish(app);
    }
}

fn create_shadow_image(mut images: ResMut<Assets<Image>>, mut cloud_images: ResMut<CloudImages>, clouds: Res<Clouds>) {
    let size = clouds.settings.quality.shadow_size.max(16);
    let mut image = Image::new_target_texture(size, size, TextureFormat::R8Unorm, None);
    image.texture_descriptor.usage |= TextureUsages::TEXTURE_BINDING;
    image.data = Some(vec![255; (size * size) as usize]);
    // Bevy samples light textures with the first decal image's sampler:
    // bilinear (soft but crisp edges at 16 m texels) and repeating (tiled).
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..ImageSamplerDescriptor::default()
    });
    cloud_images.shadow = images.add(image);
}

fn noise_image(size: Extent3d, dimension: TextureDimension, data: Vec<u8>, format: TextureFormat) -> Image {
    Image::new(size, dimension, data, format, RenderAssetUsages::RENDER_WORLD)
}

fn poll_noise(job: Res<NoiseJob>, mut images: ResMut<Assets<Image>>, mut cloud_images: ResMut<CloudImages>) {
    if cloud_images.noise.is_some() {
        return;
    }
    let Some(volumes) = job.0.lock().ok().and_then(|mut s| s.take()) else { return };
    let cube = |n: u32| Extent3d { width: n, height: n, depth_or_array_layers: n };
    cloud_images.noise = Some(NoiseImages {
        shape: images.add(noise_image(cube(SHAPE_SIZE), TextureDimension::D3, volumes.shape, TextureFormat::Rg8Unorm)),
        detail: images.add(noise_image(cube(DETAIL_SIZE), TextureDimension::D3, volumes.detail, TextureFormat::R8Unorm)),
        weather: images.add(noise_image(
            Extent3d { width: WEATHER_SIZE, height: WEATHER_SIZE, depth_or_array_layers: 1 },
            TextureDimension::D2,
            volumes.weather,
            TextureFormat::Rgba8Unorm,
        )),
    });
}

type ShadowLightItem = (Entity, &'static mut GlobalTransform, Has<DirectionalLightTexture>);
type ShadowLightFilter = (With<CloudShadowLight>, With<DirectionalLight>, Without<CloudCamera>);

/// Advances the wind, computes the texture offsets, places the shadow
/// light's texture around the camera, and fills [`CloudFrame`]. Runs after
/// transform propagation so the camera and origin are this frame's.
#[allow(clippy::too_many_arguments)]
fn drive(
    mut commands: Commands,
    time: Res<Time>,
    clouds: Res<Clouds>,
    fog: Res<Fog>,
    origin: Res<CloudsOrigin>,
    images: Res<CloudImages>,
    mut scroll: ResMut<Scroll>,
    mut slicer: ResMut<Slicer>,
    mut drift: ResMut<Drift>,
    mut frame: ResMut<CloudFrame>,
    cameras: Query<(&GlobalTransform, &Camera), With<CloudCamera>>,
    mut lights: Query<ShadowLightItem, ShadowLightFilter>,
) {
    let s = clouds.settings;
    let dt = time.delta_secs_f64();
    scroll.0.advance(s.weather.wind, s.look.evolve, dt, s.look.shape_tile);
    let camera = cameras.iter().find(|(_, c)| c.is_active).map(|(t, _)| t.translation());
    let fog_on = fog.enabled && fog.settings.any();
    if fog_on {
        drift.0.advance(fog.settings.wind, dt);
    }
    let active = (clouds.enabled || fog_on) && camera.is_some() && images.noise.is_some();

    let mut shadow = None;
    for (entity, mut transform, has_texture) in &mut lights {
        let want = active && clouds.enabled && clouds.shadows;
        if want != has_texture {
            if want {
                commands.entity(entity).insert(DirectionalLightTexture { image: images.shadow.clone(), tiled: true });
            } else {
                commands.entity(entity).remove::<DirectionalLightTexture>();
            }
        }
        let Some(cam) = camera.filter(|_| want) else {
            slicer.0.invalidate();
            continue;
        };
        // Toroidal map (`sf_atmos::shadow`): the light texture repeats and the
        // light sits at a world anchor that moves in whole cells, so texels
        // stay fixed in the world (no swimming, across origin shifts too) and
        // can refresh a band of rows per frame.
        let absolute = origin.0 + cam.as_dvec3();
        #[allow(clippy::cast_possible_truncation)]
        let cell = [(absolute.x / SHADOW_ANCHOR_CELL).floor() as i64, (absolute.z / SHADOW_ANCHOR_CELL).floor() as i64];
        #[allow(clippy::cast_precision_loss)]
        let anchor = (DVec3::new(cell[0] as f64 * SHADOW_ANCHOR_CELL, 0.0, cell[1] as f64 * SHADOW_ANCHOR_CELL) - origin.0).as_vec3();
        let rotation = transform.rotation();
        *transform = GlobalTransform::from(bevy_transform::components::Transform {
            translation: anchor,
            rotation,
            scale: Vec3::new(SHADOW_HALF_EXTENT, SHADOW_HALF_EXTENT, 1.0),
        });
        let light_dir = rotation * Vec3::Z;
        // A turn of the light moves the pattern near the camera by about
        // (anchor distance + 1 km of near field) × angle.
        let lever = (cam - anchor).with_y(0.0).length() + 1_000.0;
        let rows = slicer.0.rows(s.quality.shadow_size.max(16), s.quality.shadow_slices, cell, light_dir, &s.weather, lever);
        shadow = Some(ShadowBasis {
            center: anchor,
            x: rotation * Vec3::X * SHADOW_HALF_EXTENT,
            y: rotation * Vec3::Y * SHADOW_HALF_EXTENT,
            light_dir,
            camera: cam,
            rows,
        });
    }

    *frame = CloudFrame {
        active,
        clouds: clouds.enabled,
        fog: fog_on.then_some(fog.settings),
        fog_offsets: drift.0.offsets([origin.0.x, origin.0.z], &fog.settings.banks),
        settings: s,
        offsets: scroll.0.offsets([origin.0.x, origin.0.z], &s.look),
        #[allow(clippy::cast_possible_truncation)]
        wind_step: s.weather.wind * dt as f32,
        origin: origin.0,
        mean_cover: s.weather.mean_cover(),
        images: images.clone(),
        shadow,
    };
}
