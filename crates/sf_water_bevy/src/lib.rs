//! Bevy plugin for `sf_water` (water.md "Ocean surface", "Water pass"): GPU FFT ocean cascades
//! from the same `h0` as the CPU twin, a camera-centred clipmap surface with a horizon skirt, and
//! the water pass (a colour + depth snapshot after the sky, clouds and fog, the water draw, a
//! motion-vector redraw for DLSS/TAA).
//!
//! The host (an app or adapter) owns the inputs: [`OceanSpectrum`] (sea state, seed, cascades:
//! the twin's), [`WaterFrame`] (time, floating-origin cell, rest level), [`WaterLook`] (optics),
//! [`WaterFogInput`] (the froxel volume), and marks the camera ([`WaterCamera`]) and a surface
//! entity ([`OceanSurface`], with the camera's render layers). Systems in [`WaterSystems::Drive`]
//! (`PostUpdate`) turn them into GPU state; order the host's systems before it.
//!
//! Wave and optics code follows bevy-aqua (MIT OR Apache-2.0; `THIRD_PARTY_NOTICES.md`).

mod material;
pub mod mesh;
mod render;

use bevy_app::{App, Plugin, PostUpdate, PreStartup};
use bevy_asset::{embedded_asset, Assets, Handle, RenderAssetUsages};
use bevy_camera::visibility::NoFrustumCulling;
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_light::NotShadowCaster;
use bevy_material::AlphaMode;
use bevy_math::{IVec3, UVec2, UVec3, Vec2, Vec3, Vec4};
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{AtmosphereSettings, MaterialPlugin, MeshMaterial3d, StandardMaterial};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, TextureViewDimension};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use sf_water::{origin_phase, Cascades, SpectrumParams, WaterOptics, H0, REPEAT_PERIOD};

pub use material::{WaterExt, WaterMaterial, WaterParams};

/// FFT grid per cascade (water.md High: 256²). The shaders are compiled for it.
pub const RESOLUTION: usize = 256;
/// Cascades the GPU evaluates at most.
pub const MAX_CASCADES: usize = 4;

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
    /// Contact foam: water depth (m) below which foam shows at shores and objects (0 = none).
    pub contact_depth: f32,
    /// Jacobian below which the surface foams, and how fast foam saturates and decays (s).
    pub foam_threshold: f32,
    pub foam_gain: f32,
    pub foam_decay: f32,
    /// Air extinction per metre for the aerial transmittance (Bevy's Earth-like sea level).
    pub aerial_extinction: Vec3,
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
            contact_depth: 0.35 * o.foam.contact,
            foam_threshold: 0.35,
            foam_gain: 1.5,
            foam_decay: 2.5,
            aerial_extinction: Vec3::new(9.8e-6, 1.75e-5, 3.7e-5),
        }
    }
}

/// The froxel fog volume the water samples (`sf_atmos_bevy::FroxelVolume`, wired by the host).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct WaterFogInput {
    pub froxels: Option<Handle<Image>>,
    /// (near, far, slices, enabled).
    pub params: Vec4,
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

/// The camera that draws the water (snapshot, water pass, velocity).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WaterCamera;

/// The ocean surface entity: the plugin adds the clipmap mesh and the material. Give it the
/// camera's render layers.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OceanSurface;

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
    /// (Dx, h, Dz, foam) per cascade, written by the FFT pass every frame.
    pub displacement: Handle<Image>,
    /// Last frame's displacement (motion vectors).
    pub prev_displacement: Handle<Image>,
    /// (∂h/∂x, ∂h/∂z, ∂Dx/∂x, ∂Dz/∂z) per cascade.
    pub slopes: Handle<Image>,
    pub snapshot_color: Handle<Image>,
    pub snapshot_depth: Handle<Image>,
    /// Copy of the camera's aerial-view LUT.
    pub aerial: Handle<Image>,
    /// A 1³ clear volume standing in when there are no froxels.
    pub froxels: Handle<Image>,
    snapshot_size: UVec2,
    aerial_size: UVec3,
}

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shaders/water_common.wesl");
        embedded_asset!(app, "shaders/water_material.wesl");
        embedded_asset!(app, "shaders/water_fft.wesl");
        embedded_asset!(app, "shaders/water_snapshot.wesl");
        embedded_asset!(app, "shaders/water_aerial.wesl");
        embedded_asset!(app, "shaders/water_velocity.wesl");
        app.init_resource::<OceanSpectrum>()
            .init_resource::<WaterFrame>()
            .init_resource::<WaterLook>()
            .init_resource::<WaterFogInput>()
            .init_resource::<ClipmapSettings>()
            .init_resource::<render::WaterRender>()
            .add_plugins(MaterialPlugin::<WaterMaterial>::default())
            .configure_sets(PostUpdate, WaterSystems::Drive.after(TransformSystems::Propagate))
            .add_systems(PreStartup, create_images)
            .add_systems(
                PostUpdate,
                (depth_usages, upload_spectrum, size_targets, attach_surface, drive).chain().in_set(WaterSystems::Drive),
            );
        render::build(app);
    }

    fn finish(&self, app: &mut App) {
        render::finish(app);
    }
}

// -- images -----------------------------------------------------------------------------

fn wave_sampler() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
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

fn fill_rgba16(texels: usize, value: [f32; 4]) -> Vec<u8> {
    let one: Vec<u8> = value.iter().flat_map(|v| f16_bits(*v).to_le_bytes()).collect();
    one.repeat(texels)
}

#[allow(clippy::cast_possible_truncation)]
fn create_images(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let n = RESOLUTION as u32;
    let layers = MAX_CASCADES as u32;
    let array = Extent3d { width: n, height: n, depth_or_array_layers: layers };
    let h0 = image(
        array,
        TextureDimension::D2,
        TextureFormat::Rgba32Float,
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        Some(vec![0; (n * n * layers * 16) as usize]),
    );
    let gpu = TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC;
    let mut displacement = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, gpu, None);
    displacement.sampler = wave_sampler();
    let mut prev = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, None);
    prev.sampler = wave_sampler();
    let mut slopes = image(array, TextureDimension::D2, TextureFormat::Rgba16Float, gpu, None);
    slopes.sampler = wave_sampler();
    let one = Extent3d { width: 1, height: 1, depth_or_array_layers: 1 };
    let mut froxels = image(one, TextureDimension::D3, TextureFormat::Rgba16Float, TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST, Some(fill_rgba16(1, [0.0, 0.0, 0.0, 1.0])));
    froxels.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    let water_images = WaterImages {
        h0: images.add(h0),
        displacement: images.add(displacement),
        prev_displacement: images.add(prev),
        slopes: images.add(slopes),
        snapshot_color: images.add(snapshot_image(UVec2::ONE, TextureFormat::Rgba16Float)),
        snapshot_depth: images.add(snapshot_image(UVec2::ONE, TextureFormat::R32Float)),
        aerial: images.add(aerial_image(UVec3::ONE)),
        froxels: images.add(froxels),
        snapshot_size: UVec2::ONE,
        aerial_size: UVec3::ONE,
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

/// Filled with log(1e-6) (no in-scatter) until the first copy, so an unwritten LUT adds nothing.
fn aerial_image(size: UVec3) -> Image {
    let texels = (size.x * size.y * size.z) as usize;
    let mut image = image(
        Extent3d { width: size.x, height: size.y, depth_or_array_layers: size.z },
        TextureDimension::D3,
        TextureFormat::Rgba16Float,
        TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        Some(fill_rgba16(texels, [-13.8, -13.8, -13.8, 0.0])),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    image
}

// -- systems ----------------------------------------------------------------------------

/// Generates and uploads `h0` for every cascade when the sea state changes (water.md
/// "Weather changes": the host blends spectra with `H0::blend` and sets the result here later).
fn upload_spectrum(spectrum: Res<OceanSpectrum>, images: Option<Res<WaterImages>>, mut assets: ResMut<Assets<Image>>, mut done: Local<Option<OceanSpectrum>>) {
    let Some(images) = images else { return };
    if done.as_ref() == Some(&*spectrum) {
        return;
    }
    let n = RESOLUTION;
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
    *done = Some(spectrum.clone());
}

/// Keeps the snapshot targets at the water camera's size and the aerial copy at its LUT size.
fn size_targets(cameras: Query<(&Camera, Option<&AtmosphereSettings>), With<WaterCamera>>, images: Option<ResMut<WaterImages>>, mut assets: ResMut<Assets<Image>>) {
    let Some(mut images) = images else { return };
    let Some((camera, atmosphere)) = cameras.iter().next() else { return };
    let size = camera.physical_target_size().unwrap_or(UVec2::ONE).max(UVec2::ONE);
    if size != images.snapshot_size {
        images.snapshot_color = assets.add(snapshot_image(size, TextureFormat::Rgba16Float));
        images.snapshot_depth = assets.add(snapshot_image(size, TextureFormat::R32Float));
        images.snapshot_size = size;
    }
    let lut = atmosphere.map_or(UVec3::ONE, |a| a.aerial_view_lut_size).max(UVec3::ONE);
    if lut != images.aerial_size {
        images.aerial = assets.add(aerial_image(lut));
        images.aerial_size = lut;
    }
}

/// Gives [`OceanSurface`] entities the clipmap mesh and the water material.
fn attach_surface(
    mut commands: Commands,
    surfaces: Query<Entity, (With<OceanSurface>, Without<Mesh3d>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    images: Option<Res<WaterImages>>,
    clip: Res<ClipmapSettings>,
) {
    let Some(images) = images else { return };
    for e in &surfaces {
        let material = WaterMaterial {
            base: StandardMaterial { alpha_mode: AlphaMode::Blend, ..StandardMaterial::default() },
            extension: WaterExt::new(&images, WaterParams::default()),
        };
        commands.entity(e).insert((Mesh3d(meshes.add(mesh::clipmap(&clip))), MeshMaterial3d(materials.add(material)), NotShadowCaster, NoFrustumCulling));
    }
}

/// The water's parameters for this frame.
#[allow(clippy::cast_precision_loss)]
fn params(frame: &WaterFrame, spectrum: &OceanSpectrum, look: &WaterLook, fog: &WaterFogInput, clip: &ClipmapSettings, aerial_range: f32) -> WaterParams {
    let mut phase = [Vec2::ZERO; MAX_CASCADES];
    let mut sizes = Vec4::ZERO;
    let mut lambda_min = Vec4::ZERO;
    let count = spectrum.cascades.len().min(MAX_CASCADES);
    for c in 0..count {
        let size = spectrum.cascades.size(c);
        phase[c] = origin_phase(frame.origin_cell, size);
        sizes[c] = size as f32;
        let nyquist = size as f32 / (RESOLUTION / 2) as f32;
        let (_, hi) = spectrum.cascades.band(c);
        #[allow(clippy::cast_possible_truncation)]
        let band = if hi.is_finite() && hi > 0.0 { (std::f64::consts::TAU / hi) as f32 } else { 0.0 };
        lambda_min[c] = band.max(nyquist);
    }
    WaterParams {
        phase01: Vec4::new(phase[0].x, phase[0].y, phase[1].x, phase[1].y),
        phase23: Vec4::new(phase[2].x, phase[2].y, phase[3].x, phase[3].y),
        sizes,
        lambda_min,
        absorption: look.absorption.extend(0.0),
        scattering: look.scattering.extend(0.0),
        aerial: look.aerial_extinction.extend(aerial_range),
        froxel: if fog.froxels.is_some() { fog.params } else { Vec4::new(0.5, 1.0, 1.0, 0.0) },
        clip: Vec4::new(clip.base_spacing, (clip.quads.max(8) & !3) as f32, clip.rings.max(1) as f32, frame.level),
        foam: Vec4::new(look.whitecaps, look.contact_depth, look.refraction, look.roughness),
        misc: Vec4::new(count as f32, clip.skirt, clip.displacement_range, 1.0),
    }
}

/// Material parameters, and the render world's per-frame state.
#[allow(clippy::too_many_arguments, clippy::similar_names)]
fn drive(
    frame: Res<WaterFrame>,
    spectrum: Res<OceanSpectrum>,
    look: Res<WaterLook>,
    fog: Res<WaterFogInput>,
    clip: Res<ClipmapSettings>,
    images: Option<Res<WaterImages>>,
    cameras: Query<Option<&AtmosphereSettings>, With<WaterCamera>>,
    mut surfaces: Query<(&MeshMaterial3d<WaterMaterial>, &mut bevy_camera::visibility::Visibility), With<OceanSurface>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut state: ResMut<render::WaterRender>,
    mut last: Local<Option<f64>>,
) {
    let Some(images) = images else { return };
    let atmosphere = cameras.iter().next().flatten();
    let aerial_range = atmosphere.map_or(0.0, |a| a.aerial_view_lut_max_distance);
    let p = params(&frame, &spectrum, &look, &fog, &clip, aerial_range);
    let froxels = fog.froxels.clone().unwrap_or_else(|| images.froxels.clone());
    for (handle, mut visibility) in &mut surfaces {
        let want = if frame.enabled { bevy_camera::visibility::Visibility::Inherited } else { bevy_camera::visibility::Visibility::Hidden };
        if *visibility != want {
            *visibility = want;
        }
        let stale = materials.get(&handle.0).is_some_and(|m| {
            let e = &m.extension;
            e.params != p || e.snapshot_color != images.snapshot_color || e.aerial != images.aerial || e.froxels != froxels
        });
        if stale && let Some(mut m) = materials.get_mut(&handle.0) {
            m.extension = WaterExt { froxels: froxels.clone(), ..WaterExt::new(&images, p) };
        }
    }
    let dt = last.map_or(0.0, |l| (frame.time - l).clamp(0.0, 0.1));
    *last = Some(frame.time);
    #[allow(clippy::cast_possible_truncation)]
    let cycles_t = (frame.time.rem_euclid(REPEAT_PERIOD) / REPEAT_PERIOD) as f32;
    #[allow(clippy::cast_possible_truncation)]
    let next = render::WaterRender {
        active: frame.enabled,
        fft: render::FftUniform {
            sizes: p.sizes,
            n: RESOLUTION as u32,
            cascades: spectrum.cascades.len().min(MAX_CASCADES) as u32,
            cycles_t,
            dt: dt as f32,
            choppiness: spectrum.params.choppiness,
            foam_decay: look.foam_decay,
            foam_threshold: look.foam_threshold,
            foam_gain: look.foam_gain,
        },
        params: p,
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
