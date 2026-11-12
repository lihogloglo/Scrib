//! Bevy plugin for `sf_water` (water.md "Ocean surface", "Water pass", "Shoreline and depth
//! colour", "Water bodies at any height"): GPU FFT ocean cascades from the same `h0` as the CPU
//! twin, a camera-centred clipmap surface with a horizon skirt, shore attenuation from the
//! `water.shore` field, optics by region, and the water pass (a colour + depth snapshot of the
//! opaque scene, the water draw in Bevy's transmissive phase, so the sky, clouds and fog cover it
//! afterwards by its depth, render-order.md "Frame graph"; SSPR local reflections of the snapshot,
//! W-SSPR; a motion-vector redraw for DLSS/TAA).
//!
//! The host (an app or adapter) owns the inputs: [`OceanSpectrum`] (sea state, seed, cascades:
//! the twin's), [`WaterFrame`] (time, floating-origin cell, rest level), [`WaterLook`] (optics),
//! [`WaterShoreInput`] and
//! [`WaterRegionsInput`] (bake products), [`WaterQuality`], [`WaterReflections`]; it marks the camera ([`WaterCamera`])
//! and surface entities ([`OceanSurface`] for the ocean, [`WaterSurface`] for lakes, pools and
//! interior water at their own level), with the camera's render layers. Systems in
//! [`WaterSystems::Drive`] (`PostUpdate`) turn them into GPU state; order the host's systems
//! before it.
//!
//! Wave and optics code follows bevy-aqua (MIT OR Apache-2.0; `THIRD_PARTY_NOTICES.md`).

mod material;
pub mod mesh;
mod render;
#[cfg(test)]
mod shader_tests;

use std::sync::Arc;

use bevy_app::{App, Plugin, PostUpdate, PreStartup};
use bevy_asset::{embedded_asset, Assets, Handle, RenderAssetUsages};
use bevy_camera::visibility::{NoFrustumCulling, Visibility};
use bevy_camera::{Camera, MainPassResolutionOverride, Viewport};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_light::NotShadowCaster;
use bevy_material::AlphaMode;
use bevy_math::{DVec2, IVec3, UVec2, Vec2, Vec3, Vec4};
use bevy_mesh::{Mesh, Mesh3d};
use sf_render_order_bevy::{DrawBand, RenderOrderPlugin};
use bevy_pbr::{MaterialPlugin, MeshMaterial3d, ScreenSpaceTransmission, StandardMaterial};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, TextureViewDimension};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use sf_water::{origin_phase, reflection_plane, Cascades, OpticsMap, PlaneCandidate, ShoreField, ShoreSettings, ShoreWaves, SpectrumParams, WaterOptics, H0, REPEAT_PERIOD};

pub use material::{WaterExt, WaterMaterial, WaterParams};

/// FFT grid per cascade at High/Ultra (water.md: 256²); Low and Medium use 128².
pub const RESOLUTION: usize = 256;
/// Cascades the GPU evaluates at most.
pub const MAX_CASCADES: usize = 4;
/// Optics table entries ([`WaterRegionsInput::looks`]).
pub const MAX_LOOKS: usize = 8;

/// The ocean's sea state. Use the **same** values for the CPU twin (`sf_water::WaveTwin::new`
/// with this seed and cascades): the twin is then the GPU's cascade 0 by construction.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct OceanSpectrum {
    pub params: SpectrumParams,
    pub cascades: Cascades,
    pub seed: u32,
}

impl Default for OceanSpectrum {
    fn default() -> Self {
        Self { params: SpectrumParams::default(), cascades: Cascades::default(), seed: 1 }
    }
}

/// The FFT's quality tier (water.md "Performance budgets"): Low 2 × 128², Medium 3 × 128², High
/// 4 × 256² with the longest cascade every second frame (amortised), Ultra 4 × 256² every frame.
/// Fewer cascades drop the shortest (chop); the CPU twin is cascade 0 at every tier.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WaterQuality {
    Low,
    Medium,
    #[default]
    High,
    Ultra,
}

impl WaterQuality {
    /// FFT grid per cascade.
    #[must_use]
    pub fn resolution(self) -> usize {
        match self {
            Self::Low | Self::Medium => 128,
            Self::High | Self::Ultra => RESOLUTION,
        }
    }

    /// Cascades evaluated (at most the spectrum's).
    #[must_use]
    pub fn cascades(self) -> usize {
        match self {
            Self::Low => 2,
            Self::Medium => 3,
            Self::High | Self::Ultra => MAX_CASCADES,
        }
    }

    /// The longest cascade runs every second frame.
    #[must_use]
    pub fn amortised(self) -> bool {
        self == Self::High
    }

    /// SSPR texel size in render pixels (water.md "Performance budgets"): none on Low (the
    /// environment map only), ½ resolution on Medium, full on High and Ultra.
    #[must_use]
    pub fn sspr_scale(self) -> Option<u32> {
        match self {
            Self::Low => None,
            Self::Medium => Some(2),
            Self::High | Self::Ultra => Some(1),
        }
    }
}

/// Local reflections on the water (water.md "Water pass, refraction, reflections"): screen-space
/// planar reflections (SSPR, Cichocki 2017) of the opaque snapshot in the plane of the nearest
/// body below the camera ([`sf_water::reflection_plane`]); the environment map where SSPR misses
/// (sky, off screen) and on surfaces at another level.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterReflections {
    /// SSPR on, where the [`WaterQuality`] tier has it; off: the environment map only.
    pub sspr: bool,
    /// How far the wave normal tilts the reflection lookup: 1 = the mirror law, 0 = a flat mirror.
    pub wave_distortion: f32,
}

impl Default for WaterReflections {
    fn default() -> Self {
        Self { sspr: true, wave_distortion: 0.6 }
    }
}

/// Per-frame inputs the host sets before [`WaterSystems::Drive`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WaterFrame {
    /// Draw the ocean this frame.
    pub enabled: bool,
    /// Water time, seconds (the CPU twin's `step(t)` clock).
    pub time: f64,
    /// The floating origin's grid cell (`sf_water::origin_phase`).
    pub origin_cell: IVec3,
    /// The ocean's rest level in render space (world Y relative to the origin cell).
    pub level: f32,
}

/// The ocean's look: optics per metre and the surface terms.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterLook {
    pub absorption: Vec3,
    pub scattering: Vec3,
    pub roughness: f32,
    /// Screen refraction offset, metres at the surface.
    pub refraction: f32,
    /// Whitecap brightness scale (0 = none).
    pub whitecaps: f32,
    /// Shore foam band intensity (0 = none).
    pub shore_foam: f32,
    /// Contact foam: water depth (m) below which foam shows at shores and objects (0 = none).
    pub contact_depth: f32,
    /// Jacobian below which the surface foams, and how fast foam saturates and decays (s).
    pub foam_threshold: f32,
    pub foam_gain: f32,
    pub foam_decay: f32,
}

impl Default for WaterLook {
    fn default() -> Self {
        Self::from_optics(&WaterOptics::default())
    }
}

impl WaterLook {
    #[must_use]
    pub fn from_optics(o: &WaterOptics) -> Self {
        Self {
            absorption: Vec3::from(o.absorption),
            scattering: Vec3::from(o.scattering),
            roughness: o.roughness,
            refraction: o.refraction_strength,
            whitecaps: o.foam.whitecaps,
            shore_foam: 0.8 * o.foam.shore,
            contact_depth: 0.35 * o.foam.contact,
            foam_threshold: 0.35,
            foam_gain: 1.5,
            foam_decay: 2.5,
        }
    }

    fn table(&self) -> [Vec4; 3] {
        [self.absorption.extend(self.whitecaps), self.scattering.extend(self.contact_depth), Vec4::new(self.roughness, self.shore_foam, 0.0, 0.0)]
    }
}


/// The `water.shore` bake product of the ocean (`sf_water::ShoreField`), world coordinates.
#[derive(Resource, Clone, Debug, Default)]
pub struct WaterShoreInput {
    pub field: Option<Arc<ShoreField>>,
}

/// Optics by area for the ocean: a cell map of indices into `looks` (≤ [`MAX_LOOKS`]).
#[derive(Resource, Clone, Debug, Default)]
pub struct WaterRegionsInput {
    pub map: Option<Arc<OpticsMap>>,
    pub looks: Vec<WaterLook>,
}

/// The clipmap's shape (water.md "Mesh"; High: 7 rings of 128², 0.25 m).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ClipmapSettings {
    pub base_spacing: f32,
    pub quads: u32,
    pub rings: u32,
    /// Horizon skirt radius, metres.
    pub skirt: f32,
    /// Wave displacement fades out by this distance (the skirt is flat).
    pub displacement_range: f32,
}

impl Default for ClipmapSettings {
    fn default() -> Self {
        Self { base_spacing: 0.25, quads: 128, rings: 7, skirt: 32_000.0, displacement_range: 900.0 }
    }
}

/// The camera that draws the water (snapshot, water pass, velocity). It requires Bevy's
/// [`ScreenSpaceTransmission`] with 0 steps: the water draws in the transmissive phase (W-ORDER)
/// without Bevy's colour copy (the water reads its own snapshot).
#[derive(Component, Clone, Copy, Debug, Default)]
#[require(ScreenSpaceTransmission = water_transmission())]
pub struct WaterCamera;

/// No transmission copy (`steps: 0`): only the phase placement is used.
fn water_transmission() -> ScreenSpaceTransmission {
    ScreenSpaceTransmission { steps: 0, ..ScreenSpaceTransmission::default() }
}

/// The ocean surface entity: the plugin adds the clipmap mesh and the material. Give it the
/// camera's render layers.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OceanSurface;

/// A water body at its own level (water.md "Water bodies at any height"): a lake (bounded,
/// fetch-scaled chop), a pool or an interior's water (`Bounds`: unbounded, flat, ripples). Drawn
/// with the ocean's material and cascades; visible while its entity is. Give it the camera's
/// render layers.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct WaterSurface {
    /// Rest level, render-space Y.
    pub level: f32,
    /// Render-space XZ rectangle (min, max); `None`: everywhere (an interior's water plane).
    pub bounds: Option<[Vec2; 2]>,
    /// Amplitude scale of the swell (cascades 0-1) and of the chop (2-3).
    pub swell: f32,
    pub chop: f32,
    pub look: WaterLook,
}

impl WaterSurface {
    /// A still pool or interior water plane: no swell, wind ripples only.
    #[must_use]
    pub fn pool(level: f32, look: WaterLook) -> Self {
        Self { level, bounds: None, swell: 0.0, chop: 0.12, look }
    }
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WaterSystems {
    /// `PostUpdate`, after transform propagation: spectrum upload, targets, material, render state.
    Drive,
}

/// The plugin's images (GPU-written ones too, so materials and readbacks can hold handles).
#[derive(Resource, Clone, Debug)]
pub struct WaterImages {
    /// `h0` upload (`sf_water::H0::gpu_texels`), one layer per cascade.
    pub h0: Handle<Image>,
    /// (Dx, h, Dz, foam) per cascade, written by the FFT pass every frame, with mips.
    pub displacement: Handle<Image>,
    /// Last frame's displacement (motion vectors), level 0 only.
    pub prev_displacement: Handle<Image>,
    /// (∂h/∂x, ∂h/∂z, ∂Dx/∂x, ∂Dz/∂z) per cascade, with mips.
    pub slopes: Handle<Image>,
    pub snapshot_color: Handle<Image>,
    pub snapshot_depth: Handle<Image>,
    /// SSPR output (`Rgba16Float`, premultiplied colour + confidence), at the snapshot size / the
    /// tier's [`WaterQuality::sspr_scale`].
    pub reflection: Handle<Image>,
    /// The shore field (`Rgba16Float`: distance, openness, shoreward direction) or a 1×1 open-sea stand-in.
    pub shore: Handle<Image>,
    /// The optics map (`R8Uint`) or a 1×1 zero.
    pub regions: Handle<Image>,
    /// FFT grid of the wave images.
    pub resolution: usize,
    snapshot_size: UVec2,
    reflection_size: UVec2,
}

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        // Water draws first among the translucents (water.md "Render order").
        if !app.is_plugin_added::<RenderOrderPlugin>() {
            app.add_plugins(RenderOrderPlugin);
        }
        load_shader_library!(app, "shaders/water_common.wesl");
        embedded_asset!(app, "shaders/water_material.wesl");
        embedded_asset!(app, "shaders/water_fft.wesl");
        embedded_asset!(app, "shaders/water_mips.wesl");
        embedded_asset!(app, "shaders/water_snapshot.wesl");
        embedded_asset!(app, "shaders/water_sspr.wesl");
        embedded_asset!(app, "shaders/water_velocity.wesl");
        app.init_resource::<OceanSpectrum>()
            .init_resource::<WaterFrame>()
            .init_resource::<WaterLook>()
            .init_resource::<WaterShoreInput>()
            .init_resource::<WaterRegionsInput>()
            .init_resource::<WaterQuality>()
            .init_resource::<WaterReflections>()
            .init_resource::<ClipmapSettings>()
            .init_resource::<render::WaterRender>()
            .add_plugins(MaterialPlugin::<WaterMaterial>::default())
            .configure_sets(PostUpdate, WaterSystems::Drive.after(TransformSystems::Propagate))
            .add_systems(PreStartup, create_images)
            .add_systems(
                PostUpdate,
                (depth_usages, size_waves, upload_spectrum, upload_bakes, size_targets, attach_surface, drive).chain().in_set(WaterSystems::Drive),
            );
        render::build(app);
    }

    fn finish(&self, app: &mut App) {
        render::finish(app);
    }
}

// -- images -----------------------------------------------------------------------------

fn sampler(mode: ImageAddressMode) -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: mode,
        address_mode_v: mode,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..ImageSamplerDescriptor::default()
    })
}

#[allow(clippy::cast_possible_truncation)]
fn image(size: Extent3d, dimension: TextureDimension, format: TextureFormat, usage: TextureUsages, data: Option<Vec<u8>>) -> Image {
    let mut image = Image::new_uninit(size, dimension, format, RenderAssetUsages::all());
    image.data = data;
    image.texture_descriptor.usage = usage;
    if dimension == TextureDimension::D2 && size.depth_or_array_layers > 1 {
        image.texture_view_descriptor = Some(TextureViewDescriptor { dimension: Some(TextureViewDimension::D2Array), ..TextureViewDescriptor::default() });
    }
    image
}

/// IEEE half from f32 (normal range; enough for constant fills).
#[must_use]
pub fn f16_bits(x: f32) -> u16 {
    let b = x.to_bits();
    let sign = ((b >> 16) & 0x8000) as u16;
    let exp = i32::try_from((b >> 23) & 0xFF).unwrap_or(0) - 127 + 15;
    if x == 0.0 || exp <= 0 {
        return sign;
    }
    if exp >= 31 {
        return sign | 0x7C00;
    }
    sign | (u16::try_from(exp).unwrap_or(0) << 10) | u16::try_from((b >> 13) & 0x3FF).unwrap_or(0)
}

/// f32 from IEEE half.
#[must_use]
pub fn f16_to_f32(h: u16) -> f32 {
    let sign = if h & 0x8000 == 0 { 1.0 } else { -1.0 };
    let exp = i32::from((h >> 10) & 0x1F);
    let frac = f32::from(h & 0x3FF);
    match exp {
        0 => sign * frac * 2f32.powi(-24),
        31 => sign * f32::INFINITY,
        _ => sign * (1.0 + frac / 1024.0) * 2f32.powi(exp - 15),
    }
}


/// Mip levels of the slopes image: down to 8×8 (each level is a dispatch).
#[must_use]
pub fn wave_mips(n: usize) -> u32 {
    (usize::BITS - n.max(1).leading_zeros()).min(6)
}

/// The FFT's images at grid `n`: (h0, displacement, prev displacement, slopes).
#[allow(clippy::cast_possible_truncation)]
fn wave_images(n: usize) -> [Image; 4] {
    let n32 = n as u32;
    let layers = MAX_CASCADES as u32;
    let array = Extent3d { width: n32, height: n32, depth_or_array_layers: layers };
    let h0 = image(array, TextureDimension::D2, TextureFormat::Rgba32Float, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, Some(vec![0; n * n * MAX_CASCADES * 16]));
    let gpu = TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC;
    let mut displacement = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, gpu, None);

    displacement.sampler = sampler(ImageAddressMode::Repeat);
    let mut prev = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, None);
    prev.sampler = sampler(ImageAddressMode::Repeat);
    let mut slopes = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, gpu, None);
    slopes.texture_descriptor.mip_level_count = wave_mips(n);
    slopes.sampler = sampler(ImageAddressMode::Repeat);
    [h0, displacement, prev, slopes]
}

/// One shore texel of open deep water (`sf_water::ShoreField::encode_rgba16f`: the range, open, no direction).
fn open_sea_texel() -> Vec<u8> {
    [ShoreSettings::default().range, 1.0, 0.0, 0.0].iter().flat_map(|v| sf_water::shore::f16_bits(*v).to_le_bytes()).collect()
}

fn shore_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    let mut i = image(Extent3d { width, height, depth_or_array_layers: 1 }, TextureDimension::D2, TextureFormat::Rgba16Float, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, Some(data));
    i.sampler = sampler(ImageAddressMode::ClampToEdge);
    i
}

fn regions_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    image(Extent3d { width, height, depth_or_array_layers: 1 }, TextureDimension::D2, TextureFormat::R8Uint, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, Some(data))
}

fn create_images(mut commands: Commands, mut images: ResMut<Assets<Image>>, quality: Res<WaterQuality>) {
    let n = quality.resolution();
    let [h0, displacement, prev, slopes] = wave_images(n);
    let water_images = WaterImages {
        h0: images.add(h0),
        displacement: images.add(displacement),
        prev_displacement: images.add(prev),
        slopes: images.add(slopes),
        snapshot_color: images.add(snapshot_image(UVec2::ONE, TextureFormat::Rgba16Float)),
        snapshot_depth: images.add(snapshot_image(UVec2::ONE, TextureFormat::R32Float)),
        reflection: images.add(reflection_image(UVec2::ONE)),
        shore: images.add(shore_image(1, 1, open_sea_texel())),
        regions: images.add(regions_image(1, 1, vec![0])),
        resolution: n,
        snapshot_size: UVec2::ONE,
        reflection_size: UVec2::ONE,
    };
    commands.insert_resource(water_images);
}

fn snapshot_image(size: UVec2, format: TextureFormat) -> Image {
    image(
        Extent3d { width: size.x, height: size.y, depth_or_array_layers: 1 },
        TextureDimension::D2,
        format,
        TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        None,
    )
}

/// The SSPR output: written by the resolve (storage), sampled by the water draw.
fn reflection_image(size: UVec2) -> Image {
    let mut i = image(
        Extent3d { width: size.x, height: size.y, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Rgba16Float,
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
        None,
    );
    i.sampler = sampler(ImageAddressMode::ClampToEdge);
    i
}


// -- systems ----------------------------------------------------------------------------

/// Re-creates the wave images when the quality tier changes their grid.
fn size_waves(quality: Res<WaterQuality>, images: Option<ResMut<WaterImages>>, mut assets: ResMut<Assets<Image>>) {
    let Some(mut images) = images else { return };
    let n = quality.resolution();
    if n == images.resolution {
        return;
    }
    let [h0, displacement, prev, slopes] = wave_images(n);
    images.h0 = assets.add(h0);
    images.displacement = assets.add(displacement);
    images.prev_displacement = assets.add(prev);
    images.slopes = assets.add(slopes);
    images.resolution = n;
}

/// Generates and uploads `h0` for every cascade when the sea state or the grid changes (water.md
/// "Weather changes": the host blends spectra with `H0::blend` and sets the result here later).
fn upload_spectrum(spectrum: Res<OceanSpectrum>, images: Option<Res<WaterImages>>, mut assets: ResMut<Assets<Image>>, mut done: Local<Option<(OceanSpectrum, usize)>>) {
    let Some(images) = images else { return };
    let n = images.resolution;
    if done.as_ref().is_some_and(|(s, r)| s == &*spectrum && *r == n) {
        return;
    }
    let mut data = vec![0u8; n * n * MAX_CASCADES * 16];
    for c in 0..spectrum.cascades.len().min(MAX_CASCADES) {
        let h0 = H0::generate(&spectrum.params, &spectrum.cascades, c, n, spectrum.seed);
        for (i, t) in h0.gpu_texels(&spectrum.params).iter().enumerate() {
            let at = (c * n * n + i) * 16;
            for (j, v) in t.iter().enumerate() {
                data[at + j * 4..at + j * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
        }
    }
    if let Some(mut image) = assets.get_mut(&images.h0) {
        image.data = Some(data);
    }
    *done = Some((spectrum.clone(), n));
}

/// Uploads the shore field and the optics map when the host changes them.
#[allow(clippy::cast_possible_truncation)]
fn upload_bakes(shore: Res<WaterShoreInput>, regions: Res<WaterRegionsInput>, images: Option<ResMut<WaterImages>>, mut assets: ResMut<Assets<Image>>) {
    let Some(mut images) = images else { return };
    if shore.is_changed() {
        images.shore = match &shore.field {
            Some(f) => assets.add(shore_image(f.width as u32, f.height as u32, f.encode_rgba16f())),
            None => assets.add(shore_image(1, 1, open_sea_texel())),
        };
    }
    if regions.is_changed() {
        images.regions = match &regions.map {
            Some(m) => assets.add(regions_image(m.width as u32, m.height as u32, m.index.clone())),
            None => assets.add(regions_image(1, 1, vec![0])),
        };
    }
}

/// Keeps the snapshot targets at the active main-pass size (matching DLSS sub-viewports), and the
/// SSPR image at that size / the tier's texel scale (1×1 without SSPR).
#[allow(clippy::type_complexity)]
fn size_targets(
    cameras: Query<(&Camera, Option<&MainPassResolutionOverride>), With<WaterCamera>>,
    (quality, reflections): (Res<WaterQuality>, Res<WaterReflections>),
    images: Option<ResMut<WaterImages>>,
    mut assets: ResMut<Assets<Image>>,
) {
    let Some(mut images) = images else { return };
    let Some((camera, resolution)) = cameras.iter().next() else { return };
    let target_size = camera.physical_target_size().unwrap_or(UVec2::ONE);
    let size = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution)
        .map_or(target_size, |v| (v.physical_position + v.physical_size).min(target_size))
        .max(UVec2::ONE);
    if size != images.snapshot_size {
        images.snapshot_color = assets.add(snapshot_image(size, TextureFormat::Rgba16Float));
        images.snapshot_depth = assets.add(snapshot_image(size, TextureFormat::R32Float));
        images.snapshot_size = size;
    }
    let reflection = sspr_scale(*quality, *reflections).map_or(UVec2::ONE, |s| UVec2::new(size.x.div_ceil(s), size.y.div_ceil(s)));
    if reflection != images.reflection_size {
        images.reflection = assets.add(reflection_image(reflection));
        images.reflection_size = reflection;
    }
}

/// The SSPR texel scale in use (`None`: off by setting or tier).
fn sspr_scale(quality: WaterQuality, reflections: WaterReflections) -> Option<u32> {
    quality.sspr_scale().filter(|_| reflections.sspr)
}

/// Gives [`OceanSurface`] and [`WaterSurface`] entities the clipmap mesh and the water material.
#[allow(clippy::type_complexity)]
fn attach_surface(
    mut commands: Commands,
    surfaces: Query<Entity, (Or<(With<OceanSurface>, With<WaterSurface>)>, Without<Mesh3d>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    images: Option<Res<WaterImages>>,
    clip: Res<ClipmapSettings>,
) {
    let Some(images) = images else { return };
    for e in &surfaces {
        let material = WaterMaterial {
            // `specular_transmission` > 0 puts the water in Bevy's transmissive phase on a
            // `WaterCamera` (W-ORDER: after the opaque pass, before the sky, clouds and fog); the
            // water shader never reads it. `DrawBand::Refractive` keeps it first among the
            // translucents on a camera without `ScreenSpaceTransmission` (the fallback phase).
            base: StandardMaterial { alpha_mode: AlphaMode::Blend, specular_transmission: 1.0, ..StandardMaterial::default() },
            extension: WaterExt::new(&images, WaterParams::default()),
        };
        commands.entity(e).try_insert((Mesh3d(meshes.add(mesh::clipmap(&clip))), MeshMaterial3d(materials.add(material)), NotShadowCaster, NoFrustumCulling, DrawBand::Refractive));
    }
}

/// Inputs of [`params`] shared by every surface this frame.
struct Shared<'a> {
    frame: &'a WaterFrame,
    spectrum: &'a OceanSpectrum,
    clip: &'a ClipmapSettings,
    shore: &'a WaterShoreInput,
    regions: &'a WaterRegionsInput,
    cascades: usize,
    /// SSPR this frame: (plane level, texel scale, wave distortion).
    reflection: Option<(f32, u32, f32)>,
    /// The shore waves of this sea state on the shore field (W-GERSTNER), and last frame's time.
    shore_waves: Option<ShoreWaves>,
    prev_time: f64,
}

/// Render-space XZ of world XZ `p` under the floating origin.
#[allow(clippy::cast_possible_truncation)]
fn render_xz(p: DVec2, cell: IVec3) -> Vec2 {
    let g = f64::from(sf_coords::GRID_CELL);
    Vec2::new((p.x - f64::from(cell.x) * g) as f32, (p.y - f64::from(cell.z) * g) as f32)
}

/// The water's parameters for one surface this frame (`surface`: `None` for the ocean).
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn params(s: &Shared, look: &WaterLook, surface: Option<&WaterSurface>) -> WaterParams {
    let frame = s.frame;
    let spectrum = s.spectrum;
    let mut phase = [Vec2::ZERO; MAX_CASCADES];
    let mut sizes = Vec4::ZERO;
    let mut lambda_min = Vec4::ZERO;
    for c in 0..s.cascades {
        let size = spectrum.cascades.size(c);
        phase[c] = origin_phase(frame.origin_cell, size);
        sizes[c] = size as f32;
        let nyquist = size as f32 / (RESOLUTION / 2) as f32;
        let (_, hi) = spectrum.cascades.band(c);
        let band = if hi.is_finite() && hi > 0.0 { (std::f64::consts::TAU / hi) as f32 } else { 0.0 };
        lambda_min[c] = band.max(nyquist);
    }
    let ocean = surface.is_none();
    let level = surface.map_or(frame.level, |w| w.level);
    let (swell, chop) = surface.map_or((1.0, 1.0), |w| (w.swell, w.chop));
    let bounds = surface.and_then(|w| w.bounds);
    let shore_field = s.shore.field.as_ref().filter(|_| ocean);
    let st = shore_field.map_or_else(ShoreSettings::default, |f| f.settings);
    let shore2 = Vec4::new(st.range, st.swell_ramp, st.chop_ramp, st.chop_floor);
    // Shore waves only on the ocean with a shore field, and only while on (`--off shorewaves`).
    let sw = s.shore_waves.filter(|w| shore_field.is_some() && (w.amplitude > 0.0 || w.swash > 0.0));
    let shore = shore_field.map_or(Vec4::ZERO, |f| {
        let corner = render_xz(f.origin - DVec2::splat(f.texel * 0.5), frame.origin_cell);
        let extent = Vec2::new((f.width as f64 * f.texel) as f32, (f.height as f64 * f.texel) as f32);
        corner.extend(1.0 / extent.x).extend(1.0 / extent.y)
    });
    let map = s.regions.map.as_ref().filter(|_| ocean && !s.regions.looks.is_empty());
    let regions = map.map_or(Vec4::ZERO, |m| render_xz(m.origin, frame.origin_cell).extend((1.0 / m.cell) as f32).extend(1.0));
    let mut looks = [Vec4::ZERO; MAX_LOOKS * 3];
    for (i, l) in s.regions.looks.iter().take(MAX_LOOKS).enumerate() {
        looks[i * 3..i * 3 + 3].copy_from_slice(&l.table());
    }
    WaterParams {
        phase01: Vec4::new(phase[0].x, phase[0].y, phase[1].x, phase[1].y),
        phase23: Vec4::new(phase[2].x, phase[2].y, phase[3].x, phase[3].y),
        sizes,
        lambda_min,
        absorption: look.absorption.extend(0.0),
        scattering: look.scattering.extend(0.0),
        clip: Vec4::new(s.clip.base_spacing, (s.clip.quads.max(8) & !3) as f32, s.clip.rings.max(1) as f32, level),
        foam: Vec4::new(look.whitecaps, look.contact_depth, look.refraction, look.roughness),
        misc: Vec4::new(s.cascades as f32, s.clip.skirt, s.clip.displacement_range, 1.0),
        waves: Vec4::new(swell, chop, f32::from(u8::from(bounds.is_some())), f32::from(u8::from(shore_field.is_some()))),
        bounds: bounds.map_or(Vec4::ZERO, |[lo, hi]| Vec4::new(lo.x, lo.y, hi.x, hi.y)),
        shore,
        shore2,
        misc2: Vec4::new((frame.time.rem_euclid(REPEAT_PERIOD)) as f32, 6.0, st.normal_floor, look.shore_foam),
        swash: sw.map_or(Vec4::ZERO, |w| Vec4::new(w.omega[0], w.omega[1], w.omega[2], s.prev_time.rem_euclid(REPEAT_PERIOD) as f32)),
        swash2: sw.map_or(Vec4::ZERO, |w| Vec4::new(w.amplitude, w.swash, w.beach_slope, w.band)),
        swash3: sw.map_or(Vec4::ZERO, |w| {
            let g = f64::from(sf_coords::GRID_CELL);
            let origin = |c: i32| (f64::from(c) * g).rem_euclid(sf_water::shore_waves::NOISE_PERIOD) as f32;
            Vec4::new(w.land_margin, w.variation, origin(frame.origin_cell.x), origin(frame.origin_cell.z))
        }),
        regions,
        reflection: match s.reflection {
            // Only the surface at the mirrored level reads the SSPR; others keep the environment map.
            Some((plane, scale, distortion)) if (level - plane).abs() < 0.01 => Vec4::new(distortion, scale as f32, plane, 0.0),
            _ => Vec4::ZERO,
        },
        looks,
    }
}

/// Material parameters, and the render world's per-frame state (FFT tier and amortisation).
#[allow(clippy::too_many_arguments, clippy::similar_names, clippy::type_complexity)]
fn drive(
    (frame, spectrum, look, clip, quality): (Res<WaterFrame>, Res<OceanSpectrum>, Res<WaterLook>, Res<ClipmapSettings>, Res<WaterQuality>),
    (shore, regions, reflections): (Res<WaterShoreInput>, Res<WaterRegionsInput>, Res<WaterReflections>),
    cameras: Query<&GlobalTransform, With<WaterCamera>>,
    images: Option<Res<WaterImages>>,
    mut surfaces: Query<(&MeshMaterial3d<WaterMaterial>, Option<&WaterSurface>, Has<OceanSurface>, &mut Visibility, &bevy_camera::visibility::InheritedVisibility)>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut state: ResMut<render::WaterRender>,
    mut clock: Local<(u64, [Option<f64>; MAX_CASCADES])>,
    (mut last_time, mut shore_waves): (Local<Option<f64>>, Local<Option<(OceanSpectrum, ShoreSettings, ShoreWaves)>>),
) {
    let Some(images) = images else { return };
    let cascades = spectrum.cascades.len().min(MAX_CASCADES).min(quality.cascades());
    // SSPR plane: the nearest shown body below the camera (sf_water::reflection_plane).
    let candidates: Vec<PlaneCandidate> = surfaces
        .iter()
        .filter(|(_, surface, ocean, _, inherited)| if *ocean && surface.is_none() { frame.enabled } else { inherited.get() })
        .map(|(_, surface, _, _, _)| surface.map_or(PlaneCandidate { level: frame.level, bounds: None }, |w| PlaneCandidate { level: w.level, bounds: w.bounds }))
        .collect();
    let plane = cameras.iter().next().and_then(|c| reflection_plane(c.translation(), &candidates));
    // The shore waves follow the sea state and the field's settings (recomputed only on a change).
    let sw = shore.field.as_ref().map(|f| {
        let fresh = shore_waves.as_ref().is_some_and(|(sp, st, _)| *sp == *spectrum && *st == f.settings);
        if !fresh {
            *shore_waves = Some((spectrum.clone(), f.settings, ShoreWaves::new(&spectrum.params, &spectrum.cascades, &f.settings)));
        }
        shore_waves.as_ref().map_or_else(ShoreWaves::default, |(_, _, w)| *w)
    });
    let prev_time = last_time.replace(frame.time).unwrap_or(frame.time);
    let reflection = plane.zip(sspr_scale(*quality, *reflections)).map(|(level, scale)| (level, scale, reflections.wave_distortion));
    let shared = Shared {
        reflection,
        shore_waves: sw,
        prev_time,
        frame: &frame,
        spectrum: &spectrum,
        clip: &clip,
        shore: &shore,
        regions: &regions,
        cascades,
    };
    let mut primary: Option<WaterParams> = None;
    let mut any = false;
    for (handle, surface, ocean, mut visibility, inherited) in &mut surfaces {
        let p = match surface {
            Some(w) => params(&shared, &w.look, Some(w)),
            None => params(&shared, &look, None),
        };
        if ocean && surface.is_none() {
            let want = if frame.enabled { Visibility::Inherited } else { Visibility::Hidden };
            if *visibility != want {
                *visibility = want;
            }
        }
        let shown = if ocean && surface.is_none() { frame.enabled } else { inherited.get() };
        if !shown {
            continue;
        }
        any = true;
        if primary.is_none() || (ocean && surface.is_none()) {
            primary = Some(p);
        }
        let stale = materials.get(&handle.0).is_some_and(|m| {
            let e = &m.extension;
            e.params != p
                || e.snapshot_color != images.snapshot_color
                || e.reflection != images.reflection
                || e.displacement != images.displacement
                || e.shore != images.shore
                || e.regions != images.regions
        });
        if stale && let Some(mut m) = materials.get_mut(&handle.0) {
            m.extension = WaterExt::new(&images, p);
        }
    }
    // FFT tier: the cascades this frame. On High the two swell cascades (256 m, 64 m) alternate,
    // so three of four run each frame; the chop runs every frame.
    let (count, k) = (u32::try_from(cascades).unwrap_or(0), clock.0);
    clock.0 += 1;
    let skip = if quality.amortised() && count >= 3 { u32::from(k % 2 == 0) } else { u32::MAX };
    let mut dts = Vec4::ZERO;
    #[allow(clippy::cast_possible_truncation)]
    for c in (0..cascades).filter(|c| u32::try_from(*c).ok() != Some(skip)) {
        dts[c] = clock.1[c].map_or(0.0, |l| (frame.time - l).clamp(0.0, 0.2)) as f32;
        clock.1[c] = Some(frame.time);
    }
    #[allow(clippy::cast_possible_truncation)]
    let cycles_t = (frame.time.rem_euclid(REPEAT_PERIOD) / REPEAT_PERIOD) as f32;
    let n = images.resolution;
    let p = primary.unwrap_or_else(|| params(&shared, &look, None));
    #[allow(clippy::cast_possible_truncation)]
    let next = render::WaterRender {
        active: any,
        fft: render::FftUniform {
            sizes: p.sizes,
            dts,
            n: n as u32,
            log_n: n.trailing_zeros(),
            skip,
            count: count - u32::from(skip < count),
            cycles_t,
            choppiness: spectrum.params.choppiness,
            foam_decay: look.foam_decay,
            foam_threshold: look.foam_threshold,
            foam_gain: look.foam_gain,
        },
        mips: wave_mips(n),
        layers: count,
        params: p,
        sspr: reflection.map(|(level, scale, _)| render::SsprState { level, scale }),
        images: Some(render::ImageIds::new(&images)),
    };
    *state = next;
}

/// The snapshot and velocity passes read the camera's depth buffer: make it bindable (as Bevy's
/// atmosphere does for `render_sky`).
fn depth_usages(mut cameras: Query<&mut bevy_camera::Camera3d, With<WaterCamera>>) {
    let bit = TextureUsages::TEXTURE_BINDING.bits();
    for mut camera in &mut cameras {
        if camera.depth_texture_usages.0 & bit == 0 {
            camera.depth_texture_usages.0 |= bit;
        }
    }
}

/// How many frames the GPU FFT has run (0 while its pipelines compile).
#[must_use]
pub fn fft_frames() -> u64 {
    render::FFT_FRAMES.load(std::sync::atomic::Ordering::Relaxed)
}
