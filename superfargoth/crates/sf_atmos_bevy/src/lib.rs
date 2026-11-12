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
//!   clouds off too ([`Clouds::enabled`]); costs nothing without fog. The
//!   fog's isotropic fill adds the camera's ambient light ([`view_ambient`],
//!   [`Fog::view_ambient`]) to the sky's, so a host's night ambient floor
//!   lights the fog as it lights the ground (WE-9).
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

mod fog_translucents;
mod render;
mod transparent;
mod weather_map;
#[cfg(test)]
mod shader_tests;
#[cfg(test)]
mod tests;

pub use fog_translucents::FogTranslucentsPlugin;
pub use transparent::{FroxelFogExt, FroxelFogMaterial, FroxelVolume};
pub use weather_map::CloudWeatherMap;

use std::sync::{Arc, Mutex};

use bevy_app::{App, Plugin, PostUpdate, Startup, Update};
use bevy_asset::{embedded_asset, Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::{CompressedImageFormats, Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor, ImageType};
use bevy_camera::visibility::ViewVisibility;
use bevy_light::{AmbientLight, DirectionalLight, DirectionalLightTexture, GlobalAmbientLight, PointLight};
use bevy_math::{DVec3, Vec2, Vec3, Vec4};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, TextureViewDimension};
use bevy_shader::load_shader_library;
use bevy_time::Time;
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use sf_atmos::scroll::TileOffsets;
use sf_atmos::{CloudScroll, CloudSettings, CloudTemporal, FogDrift, FogOffsets, FogSettings, FroxelSettings, NoiseVolumes, ShadowRows, ShadowSlicer, DETAIL_SIZE, SHAPE_SIZE, WEATHER_SIZE};

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

/// Strike glows the cloud march lights at once.
pub const GLOWS: usize = 4;

/// What the clouds render with this frame. Weather writes `settings`.
#[derive(Resource, Clone, Debug)]
pub struct Clouds {
    pub settings: CloudSettings,
    /// Draw the clouds (off: nothing renders, no cost).
    pub enabled: bool,
    /// Cloud shadows on the [`CloudShadowLight`].
    pub shadows: bool,
    /// In-cloud lightning glows (`WE4e`): render-space position, intensity (0: unused).
    pub glows: [Vec4; GLOWS],
    /// How the resolve accumulates the cloud history (WE-10 fix 1; the `cloud_history` A/B).
    pub temporal: CloudTemporal,
}

/// Height fog, ground-fog banks and the far fog (weather.md "Fog", F2), drawn
/// by the same node as the clouds: the banks are marched in the cloud trace,
/// the analytic layers applied per pixel in the composite. Weather writes
/// `settings`; with no fog (or `enabled` off) the fog terms cost nothing.
#[derive(Resource, Clone, Copy, Debug)]
pub struct Fog {
    pub settings: FogSettings,
    pub enabled: bool,
    /// The froxel volume near the camera (F3: god rays, shadowed and point-lit
    /// fog); `None` is the Low preset (analytic layers only). High by default.
    pub froxels: Option<FroxelSettings>,
    /// How much of the camera's ambient light ([`view_ambient`]) the fog
    /// in-scatters on top of the sky fill: [`sf_atmos::VIEW_AMBIENT_IN_FOG`]
    /// (1, the same light as the surfaces) by default, 0 the sky alone.
    pub view_ambient: f32,
}

/// The fog bank modulation is depth-aware at depth edges (FOG-BANK, FG-7; on by default): the
/// trace records it at the nearest and furthest depth of its window and the composite
/// interpolates by each pixel's depth ([`sf_atmos::bank_edges`]). `false`: the furthest-depth
/// value alone, picked by population (the old path, an A/B).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FogBankEdges(pub bool);

impl Default for FogBankEdges {
    fn default() -> Self {
        Self(true)
    }
}

/// The clouds sit inside the fog column (WE12-FIX, WE-12; on by default): the composite
/// evaluates the fog to the cloud's depth too and puts only that part in front of the clouds
/// ([`sf_atmos::compose_fog_split`]), and the trace composes the far weather in front of the
/// deck or behind it by distance. `false`: the old order, the whole column and the far weather
/// in front of the clouds (wrong from above or inside the deck; the A/B).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FogCloudSplit(pub bool);

impl Default for FogCloudSplit {
    fn default() -> Self {
        Self(true)
    }
}

/// Stable cloud silhouettes (S1, WE-14; on by default): under the checkerboard the trace takes
/// its depth window over the whole block ([`sf_atmos::temporal::depth_window`]) and classes the
/// block along its centre ray, so a block's open/cut class doesn't flip with the checkerboard
/// phase; a texel whose history was rejected shows the mean of its same-population neighbours
/// ([`sf_atmos::temporal::mismatch_fallback`]) and the stored cloud distance is accumulated like
/// the colour (the composite's occlusion test reads it). `false` (`--off cloudedge`): the old
/// window centred on the traced pixel, the raw sample and the last sample's distance (the A/B).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloudEdgeStable(pub bool);

impl Default for CloudEdgeStable {
    fn default() -> Self {
        Self(true)
    }
}

/// The storm cloud shape (S3, WE-15; on by default): in a storm the shape noise runs in world
/// altitude, the top follows coverage (domed cells), an anvil spreads near the top, the base is
/// ragged and cells shrink by threshold toward the region edge (`sf_atmos::combine_density_shaped`).
/// Calm decks never change. `false` (`--off cbshape`): the old storm profile (the A/B).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CumulonimbusShape(pub bool);

impl Default for CumulonimbusShape {
    fn default() -> Self {
        Self(true)
    }
}

/// The far storm's shape (CW-1, [`sf_atmos::StormShape`]; on by default): distant rain hangs
/// under the clouds' thick cores, its region edge is re-blended over `edge_m` with noise, and
/// the far medium is integrated analytically. `enabled: false` (`--off stormshape`): the `WE4d`
/// far weather (the region column, the A/B). A game sets the edge in metres through its adapter.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FarStormShape(pub sf_atmos::StormShape);

/// A height-field shadow for the froxels' sun light past the cascades (the
/// terrain's shadow-height map, handed over by the app): `Rg16Float` texels of
/// (shadow height − base, occluder distance), meters; `params` = render-space
/// (min x, min z, 1 / extent, base height), extent 0 = none.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct FogShadowHeights {
    pub image: Option<Handle<Image>>,
    pub params: Vec4,
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
    /// Spatio-temporal blue noise for the march jitter ([`STBN_KTX2`]).
    pub blue_noise: Handle<Image>,
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
#[allow(clippy::struct_excessive_bools, reason = "independent per-frame switches")]
pub(crate) struct CloudFrame {
    pub active: bool,
    /// The clouds draw (`active` may hold for the fog alone).
    pub clouds: bool,
    pub glows: [Vec4; GLOWS],
    pub settings: CloudSettings,
    pub temporal: CloudTemporal,
    /// The fog, when it draws.
    pub fog: Option<FogSettings>,
    /// [`FogBankEdges`].
    pub fog_bank_edges: bool,
    /// [`FogCloudSplit`].
    pub fog_cloud_split: bool,
    /// [`CloudEdgeStable`].
    pub cloud_edge_stable: bool,
    /// [`CumulonimbusShape`].
    pub cb_shape: bool,
    /// [`FarStormShape`].
    pub storm_shape: sf_atmos::StormShape,
    /// The camera's ambient radiance × [`Fog::view_ambient`] (linear RGB),
    /// added to the fog's sky fill.
    pub fog_ambient: Vec3,
    pub fog_offsets: FogOffsets,
    pub offsets: TileOffsets,
    /// Wind displacement this frame (world X, Z), meters.
    pub wind_step: Vec2,
    pub origin: DVec3,
    pub mean_cover: f32,
    pub images: CloudImages,
    /// Shadow map basis: center, light-plane axes (× half extent), toward the light.
    pub shadow: Option<ShadowBasis>,
    /// The froxel grid, when the froxels run (F3).
    pub froxels: Option<FroxelSettings>,
    /// The point lights lit into the froxels: (position, range), (colour × cd).
    pub fog_lights: Vec<[Vec4; 2]>,
    /// The terrain's shadow-height map and its params ([`FogShadowHeights`]).
    pub far_map: Option<(Handle<Image>, Vec4)>,
    /// The image the froxels integrate into ([`FroxelVolume`]: transparent materials read it).
    pub volume: Option<(Handle<Image>, [u32; 3])>,
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
        load_shader_library!(app, "shaders/fog_bank.wesl");
        load_shader_library!(app, "shaders/bank_edges.wesl");
        load_shader_library!(app, "shaders/froxel_common.wesl");
        embedded_asset!(app, "shaders/transparent_fog.wesl");
        embedded_asset!(app, "shaders/cloud_trace.wesl");
        embedded_asset!(app, "shaders/cloud_resolve.wesl");
        embedded_asset!(app, "shaders/cloud_composite.wesl");
        embedded_asset!(app, "shaders/cloud_shadow.wesl");
        embedded_asset!(app, "shaders/fog_light.wesl");
        embedded_asset!(app, "shaders/froxel_inject.wesl");
        embedded_asset!(app, "shaders/froxel_integrate.wesl");

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

        if !app.is_plugin_added::<sf_render_order_bevy::RenderOrderPlugin>() {
            // `TranslucentFog` and the `translucent_fog` shader module `transparent_fog.wesl` imports.
            app.add_plugins(sf_render_order_bevy::RenderOrderPlugin);
        }
        app.insert_resource(Clouds { settings: self.settings, enabled: true, shadows: true, glows: [Vec4::ZERO; GLOWS], temporal: CloudTemporal::default() })
            .insert_resource(Fog { settings: self.fog, enabled: true, froxels: Some(FroxelSettings::high()), view_ambient: sf_atmos::VIEW_AMBIENT_IN_FOG })
            .init_resource::<FogBankEdges>()
            .init_resource::<FogCloudSplit>()
            .init_resource::<CloudEdgeStable>()
            .init_resource::<CumulonimbusShape>()
            .init_resource::<FarStormShape>()
            .init_resource::<FogShadowHeights>()
            .init_resource::<FroxelVolume>()
            .add_plugins(bevy_pbr::MaterialPlugin::<FroxelFogMaterial>::default())
            .add_systems(PostUpdate, (transparent::update_volume, transparent::fix_new_materials).chain().before(CloudSystems::Drive))
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
        blue_noise: images.add(blue_noise_image()),
    });
}

/// Spatio-temporal blue noise (NVIDIA STBN, 128² × 32 layers, Wolfe et al. 2022) for the cloud
/// march jitter: the texture Bevy 0.20 ships in `bevy_pbr` behind its `bluenoise_texture`
/// feature (MIT/Apache with Bevy), read from our vendored copy so that feature's other users
/// (contact shadows, SSR) keep their noise. The shaders read `.x`.
const STBN_KTX2: &[u8] = include_bytes!("../../../vendor/bevy_pbr/src/bluenoise/stbn.ktx2");

/// [`STBN_KTX2`] as a 2D array image; a 1×1 one (the shaders then fall back to gradient noise)
/// if this build can't decode KTX2.
fn blue_noise_image() -> Image {
    let decoded = Image::from_buffer(STBN_KTX2, ImageType::Extension("ktx2"), CompressedImageFormats::NONE, false, ImageSampler::Default, RenderAssetUsages::RENDER_WORLD);
    let mut image = decoded.unwrap_or_else(|e| {
        log::error!("cloud blue noise: {e}; the march jitter falls back to gradient noise");
        noise_image(Extent3d { width: 1, height: 1, depth_or_array_layers: 2 }, TextureDimension::D2, vec![0; 8], TextureFormat::Rgba8Unorm)
    });
    image.texture_view_descriptor = Some(TextureViewDescriptor { dimension: Some(TextureViewDimension::D2Array), ..TextureViewDescriptor::default() });
    image
}

type ShadowLightItem = (Entity, &'static mut GlobalTransform, Has<DirectionalLightTexture>);
type ShadowLightFilter = (With<CloudShadowLight>, With<DirectionalLight>, Without<CloudCamera>);
/// Point lights the froxels may light (disjoint from the shadow light's mutable transform).
type FogLightQuery<'w, 's> = Query<'w, 's, (&'static PointLight, &'static GlobalTransform, &'static ViewVisibility), Without<DirectionalLight>>;
/// The fog and its A/B switches, as one [`drive`] parameter (Bevy's 16-parameter limit).
type FogSwitches<'w> = (Res<'w, Fog>, Res<'w, FogBankEdges>, Res<'w, FogCloudSplit>, Res<'w, FarStormShape>, Res<'w, CloudEdgeStable>, Res<'w, CumulonimbusShape>);

/// Advances the wind, computes the texture offsets, places the shadow
/// light's texture around the camera, and fills [`CloudFrame`]. Runs after
/// transform propagation so the camera and origin are this frame's.
#[allow(clippy::too_many_arguments)]
fn drive(
    mut commands: Commands,
    time: Res<Time>,
    clouds: Res<Clouds>,
    (fog, bank_edges, cloud_split, storm_shape, edge_stable, cb_shape): FogSwitches,
    origin: Res<CloudsOrigin>,
    images: Res<CloudImages>,
    mut scroll: ResMut<Scroll>,
    mut slicer: ResMut<Slicer>,
    mut drift: ResMut<Drift>,
    mut frame: ResMut<CloudFrame>,
    cameras: Query<(&GlobalTransform, &Camera, Option<&AmbientLight>), With<CloudCamera>>,
    global_ambient: Option<Res<GlobalAmbientLight>>,
    mut lights: Query<ShadowLightItem, ShadowLightFilter>,
    point_lights: FogLightQuery,
    heights: Res<FogShadowHeights>,
    volume: Res<FroxelVolume>,
) {
    let s = clouds.settings;
    let dt = time.delta_secs_f64();
    scroll.0.advance(s.weather.wind, s.look.evolve, dt, s.look.shape_tile);
    let active_camera = cameras.iter().find(|(_, c, _)| c.is_active);
    let camera = active_camera.map(|(t, _, _)| t.translation());
    let ambient = view_ambient(active_camera.and_then(|(_, _, a)| a), global_ambient.as_deref());
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
        fog_bank_edges: bank_edges.0,
        fog_cloud_split: cloud_split.0,
        cloud_edge_stable: edge_stable.0,
        cb_shape: cb_shape.0,
        storm_shape: storm_shape.0,
        fog_ambient: sf_atmos::fog_fill(Vec3::ZERO, ambient, fog.view_ambient),
        fog_offsets: drift.0.offsets([origin.0.x, origin.0.z], &fog.settings.banks),
        glows: clouds.glows,
        settings: s,
        temporal: clouds.temporal,
        offsets: scroll.0.offsets([origin.0.x, origin.0.z], &s.look),
        #[allow(clippy::cast_possible_truncation)]
        wind_step: s.weather.wind * dt as f32,
        origin: origin.0,
        mean_cover: s.weather.mean_cover(),
        froxels: fog.froxels.filter(|_| fog_on),
        fog_lights: camera.zip(fog.froxels.filter(|_| fog_on)).map(|(cam, f)| fog_lights(cam, &f, &point_lights)).unwrap_or_default(),
        far_map: heights.image.clone().map(|i| (i, heights.params)),
        volume: Some((volume.image.clone(), volume.size)),
        images: images.clone(),
        shadow,
    };
}

/// The flat ambient radiance that lights the camera's surfaces (linear RGB,
/// `color × brightness`, as Bevy's PBR shades `AmbientLight`): the camera's
/// own [`AmbientLight`] when it has one, else the [`GlobalAmbientLight`]
/// (Bevy's rule). The fog adds it to its sky fill ([`Fog::view_ambient`]).
pub fn view_ambient(camera: Option<&AmbientLight>, global: Option<&GlobalAmbientLight>) -> Vec3 {
    let (color, brightness) = match (camera, global) {
        (Some(a), _) => (a.color, a.brightness),
        (None, Some(g)) => (g.color, g.brightness),
        (None, None) => return Vec3::ZERO,
    };
    let c = color.to_linear();
    Vec3::new(c.red, c.green, c.blue) * brightness
}

/// The point lights the froxels light (weather.md "Fog", F3: lanterns in fog):
/// visible ones whose range reaches into the froxel range, nearest first, at
/// most `max_lights`. Colour × luminous intensity (lm / 4π, as Bevy shades
/// point lights); unshadowed (a named limit: Bevy's cube shadow maps aren't
/// sampled in the fog).
fn fog_lights(camera: Vec3, froxels: &FroxelSettings, lights: &FogLightQuery) -> Vec<[Vec4; 2]> {
    let mut found: Vec<(f32, [Vec4; 2])> = lights
        .iter()
        .filter(|(l, _, v)| v.get() && l.intensity > 0.0 && l.range > 0.0)
        .filter_map(|(l, t, _)| {
            let p = t.translation();
            let d = p.distance(camera);
            (d < froxels.far + l.range).then(|| {
                let c = l.color.to_linear();
                let cd = l.intensity / (4.0 * std::f32::consts::PI);
                (d, [p.extend(l.range), Vec4::new(c.red * cd, c.green * cd, c.blue * cd, 0.0)])
            })
        })
        .collect();
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    found.truncate((froxels.max_lights as usize).min(sf_atmos::MAX_FOG_LIGHTS));
    found.into_iter().map(|(_, l)| l).collect()
}
