//! Surface weather on the GPU (surface-weather.md "GPU resources", "Shader integration"; slice
//! W2). Thin: the window, shelter and noise logic is Bevy-free (`sf_surface::view`).
//!
//! - **Cell window** ([`cells`]): `Rgba8Unorm` 128², one texel per 32 m cell around the camera
//!   (R wet, G puddle, B snow, A ash), toroidal; strips on recenter, all of it when
//!   [`WorldSurface::generation`] moves.
//! - **Shelter map** ([`shelter`]): `Depth32Float` 1024² over 128 m, vertical, a subview of
//!   Bevy's shadow pass (`sf_sky_bevy::sun_view`).
//! - **Noise atlas**: `Rgba8Unorm` 256², repeat, generated at startup.
//! - **[`SurfaceUniform`]**: uploaded as `vec4` slots when it changes.
//! - **[`SurfaceWeatherView`]**: the exterior camera that gets surface weather (W2 follows
//!   `TerrainCamera`); extracted to the render world for W3's mesh-view bindings.
//! - **WESL** `sf_surface_bevy::layer`: `apply_surface_layer` and its helpers; binding-agnostic
//!   (the caller passes the uniform and textures).
//! - **Terrain hook** (W2 scaffold, removed by W3's `bevy_pbr` patch "SW"): the terrain
//!   material's bindings 113-118 get these resources and its `SURFACE_WEATHER` def turns on.
//!
//! W2 shows the layer only through the debug views (`surface debug exposure|cells`).

mod cells;
mod shelter;
mod uniform;

#[cfg(test)]
mod tests;

use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_render::storage::ShaderBuffer;
use bevy_render::sync_world::RenderEntity;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use sf_surface::view::{NOISE_SIZE, WINDOW_CELLS, noise_atlas};
use sf_terrain_bevy::{TerrainCamera, TerrainMaterial, TerrainOrigin, TerrainSystems};
use sf_weather_bevy::SurfaceDebug;

pub use cells::{CellWrite, CellWrites};
pub use shelter::{NoSurfaceShelter, SHELTER_SUBVIEW};
pub use uniform::{FLAG_ASH, FLAG_PUDDLE, FLAG_SNOW, FLAG_WET, SurfaceUniform, U_SLOTS};

/// Marks the camera that gets surface weather (one exterior camera). W2: follows
/// `TerrainCamera` (the exterior camera; doors move it).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SurfaceWeatherView;

/// The GPU resources the layer reads.
#[derive(Resource, Clone, Debug)]
pub struct SurfaceGpu {
    /// [`SurfaceUniform`] as `U_SLOTS` `vec4<u32>`.
    pub uniform: Handle<ShaderBuffer>,
    pub cells: Handle<Image>,
    pub shelter: Handle<Image>,
    pub noise: Handle<Image>,
}

/// Counters for status tools and tests.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SurfaceStats {
    pub shelter_redraws: u64,
    pub shelter_casters: usize,
    /// Cell texels written (strips and full rewrites).
    pub cell_texels: u64,
    pub full_uploads: u64,
    pub uniform_uploads: u64,
}

/// System sets, in `PostUpdate` after transforms and the terrain.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceSystems;

pub struct SurfaceWeatherPlugin;

impl Plugin for SurfaceWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SurfaceUniform>()
            .init_resource::<SurfaceStats>()
            .init_resource::<CellWrites>()
            .init_resource::<TerrainOrigin>()
            .init_resource::<SurfaceDebug>()
            .configure_sets(
                PostUpdate,
                SurfaceSystems
                    .after(TransformSystems::Propagate)
                    .after(TerrainSystems::Update),
            )
            .add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                (
                    (follow_terrain_camera, shelter::tag_terrain),
                    cells::update_cells,
                    debug_view,
                    shelter::update_shelter,
                    (write_uniform, hook_terrain),
                )
                    .chain()
                    .in_set(SurfaceSystems),
            );
        sf_sky_bevy::sun_view::add_sun_view::<shelter::ShelterFrame>(app);
        if app.get_sub_app(RenderApp).is_none() {
            return;
        }
        load_shader_library!(app, "layer.wesl");
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<cells::PendingCellWrites>()
            .add_systems(ExtractSchedule, (cells::extract_writes, extract_view))
            .add_systems(
                Render,
                cells::write_cells.in_set(RenderSystems::PrepareResources),
            );
    }
}

/// The cell window image (zeros until the first upload), filtered and repeating (the torus).
fn cells_image() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: WINDOW_CELLS.unsigned_abs(),
            height: WINDOW_CELLS.unsigned_abs(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = repeat_linear();
    image
}

fn noise_image() -> Image {
    let data: Vec<u8> = noise_atlas().into_iter().flatten().collect();
    let mut image = Image::new(
        Extent3d {
            width: NOISE_SIZE,
            height: NOISE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = repeat_linear();
    image
}

fn repeat_linear() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..ImageSamplerDescriptor::default()
    })
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    uniform: Res<SurfaceUniform>,
) {
    commands.insert_resource(SurfaceGpu {
        uniform: buffers.add(ShaderBuffer::new(
            uniform.slots().to_vec(),
            RenderAssetUsages::default(),
        )),
        cells: images.add(cells_image()),
        shelter: shelter::shelter_handle(&mut images),
        noise: images.add(noise_image()),
    });
}

/// W2: the surface view follows the terrain camera.
#[allow(clippy::type_complexity)]
fn follow_terrain_camera(
    mut commands: Commands,
    add: Query<
        Entity,
        (
            With<Camera>,
            With<TerrainCamera>,
            Without<SurfaceWeatherView>,
        ),
    >,
    remove: Query<Entity, (With<SurfaceWeatherView>, Without<TerrainCamera>)>,
) {
    for e in &add {
        commands.entity(e).insert(SurfaceWeatherView);
    }
    for e in &remove {
        commands.entity(e).remove::<SurfaceWeatherView>();
    }
}

/// Mirrors [`SurfaceWeatherView`] onto the cameras' render entities (W3's mesh-view key reads
/// it there), and takes it off when the main world did.
fn extract_view(
    mut commands: Commands,
    main: Extract<Query<&RenderEntity, With<SurfaceWeatherView>>>,
    marked: Query<Entity, With<SurfaceWeatherView>>,
) {
    let now: Vec<Entity> = main.iter().map(|r| **r).collect();
    for e in &marked {
        if !now.contains(&e) {
            commands.entity(e).remove::<SurfaceWeatherView>();
        }
    }
    for e in now {
        if let Ok(mut c) = commands.get_entity(e) {
            c.insert(SurfaceWeatherView);
        }
    }
}
fn debug_view(debug: Res<SurfaceDebug>, mut uniform: ResMut<SurfaceUniform>) {
    if uniform.debug != debug.0 {
        uniform.debug = debug.0;
    }
}

/// Uploads the uniform when it changed.
fn write_uniform(
    gpu: Option<Res<SurfaceGpu>>,
    uniform: Res<SurfaceUniform>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut stats: ResMut<SurfaceStats>,
    mut last: Local<Option<[[u32; 4]; U_SLOTS]>>,
) {
    let Some(gpu) = gpu else { return };
    let slots = uniform.slots();
    if *last == Some(slots) {
        return;
    }
    if let Some(mut buffer) = buffers.get_mut(&gpu.uniform)
        && let Some(dst) = buffer.cast_slice_mut::<[u32; 4]>()
        && dst.len() == U_SLOTS
    {
        dst.copy_from_slice(&slots);
        *last = Some(slots);
        stats.uniform_uploads += 1;
    }
}

/// W2 scaffold: hands the resources to every terrain material not hooked yet (W3 replaces this
/// with mesh-view bindings).
fn hook_terrain(gpu: Option<Res<SurfaceGpu>>, mut materials: ResMut<Assets<TerrainMaterial>>) {
    let Some(gpu) = gpu else { return };
    let ids: Vec<_> = materials
        .iter()
        .filter(|(_, m)| !m.surface)
        .map(|(id, _)| id)
        .collect();
    for id in ids {
        if let Some(mut m) = materials.get_mut(id) {
            m.surface_uniform = gpu.uniform.clone();
            m.surface_cells = gpu.cells.clone();
            m.surface_shelter = gpu.shelter.clone();
            m.surface_noise = gpu.noise.clone();
            m.surface = true;
        }
    }
}
