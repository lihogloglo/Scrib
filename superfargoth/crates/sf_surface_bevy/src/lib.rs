//! Surface weather on the GPU (surface-weather.md "GPU resources", "Shader integration"; slices
//! W2-W3). Thin: the window, shelter and noise logic is Bevy-free (`sf_surface::view`), the wet
//! math has a CPU twin (`sf_surface::layer`).
//!
//! - **Cell window** ([`cells`]): `Rgba8Unorm` 128², one texel per 32 m cell around the camera
//!   (R wet, G puddle, B snow, A ash), toroidal; strips on recenter, all of it when
//!   [`WorldSurface::generation`] moves.
//! - **Shelter map** ([`shelter`]): `Depth32Float` 1024² over 128 m, vertical, a subview of
//!   Bevy's shadow pass (`sf_sky_bevy::sun_view`).
//! - **Noise atlas**: `Rgba8Unorm` 256², repeat, generated at startup.
//! - **[`SurfaceUniform`]**: its `vec4` slots, written to a render-world uniform buffer when they
//!   change ([`render`]).
//! - **[`SurfaceWeatherCamera`]**: the exterior camera that gets surface weather (the game marks
//!   it). Its render-world view gets `bevy_pbr::SurfaceWeatherView`: the vendored `bevy_pbr`
//!   patch "SW" binds these resources at mesh-view bindings 43-48 and sets the
//!   `SURFACE_WEATHER` shader def, so every `StandardMaterial` (statics, characters, held items,
//!   fades), the terrain and the far proxies run the layer in the forward main pass.
//! - **WESL**: `sf_surface_bevy::layer` (`apply_surface_layer`, binding-agnostic) and
//!   `sf_surface_bevy::view` (the bindings and the entry points materials call).
//! - **[`SurfaceOrigin`]**, **[`SurfaceSettings`]**, [`NoSurfaceShelter`]: the game's glue (the
//!   floating origin, the A/B switches and preset, terrain patches out of the shelter map).
//!
//! W3 shows wetness (Lagarde's model, film normal flattening, wall streaks); W2's debug views
//! stay (`surface debug exposure|cells|porosity`).

mod cells;
mod render;
mod shelter;
mod uniform;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod shader_tests;

use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_render::{ExtractSchedule, RenderApp};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use sf_coords::WorldPos;
use sf_surface::view::{NOISE_SIZE, WINDOW_CELLS, noise_atlas};
use sf_weather_bevy::SurfaceDebug;

pub use cells::{CellWrite, CellWrites};
pub use shelter::{NoSurfaceShelter, SHELTER_SUBVIEW};
pub use uniform::{FLAG_ASH, FLAG_PUDDLE, FLAG_SNOW, FLAG_WET, SurfaceUniform, U_SLOTS};

/// Marks the camera that gets surface weather (one exterior camera; the game inserts and
/// removes it as the camera changes worldspace).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SurfaceWeatherCamera;

/// The floating origin render space is relative to (the game copies its world origin here,
/// as for `sf_wind_bevy::WindOrigin`). The window and the shelter map work in render space.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfaceOrigin(pub WorldPos);

/// A/B switches and the preset (surface-weather.md "A/B switches", "Budget"). Off parts keep
/// the hook compiled in and clear its flags (one uniform branch per pixel).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceSettings {
    /// `--off surface`: the whole layer.
    pub layer: bool,
    /// `--off wet`: wetness.
    pub wet: bool,
    /// The shelter map (off on Low and with `--off shelter`): exposure 1 everywhere.
    pub shelter: bool,
}

impl Default for SurfaceSettings {
    fn default() -> Self {
        Self {
            layer: true,
            wet: true,
            shelter: true,
        }
    }
}

impl SurfaceSettings {
    /// The layer flags these settings let through.
    pub fn flag_mask(self) -> u32 {
        if !self.layer {
            return 0;
        }
        let all = FLAG_WET | FLAG_PUDDLE | FLAG_SNOW | FLAG_ASH;
        if self.wet { all } else { all & !FLAG_WET }
    }
}

/// The GPU resources the layer reads (the uniform buffer is render-world, [`render`]).
#[derive(Resource, Clone, Debug)]
pub struct SurfaceGpu {
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
    /// Times the uniform's slots changed (each is one buffer write in the render world).
    pub uniform_uploads: u64,
}

/// System sets, in `PostUpdate` after transforms (order the game's origin and camera glue
/// before it).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceSystems;

pub struct SurfaceWeatherPlugin;

impl Plugin for SurfaceWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SurfaceUniform>()
            .init_resource::<SurfaceStats>()
            .init_resource::<CellWrites>()
            .init_resource::<SurfaceOrigin>()
            .init_resource::<SurfaceSettings>()
            .init_resource::<SurfaceDebug>()
            .configure_sets(
                PostUpdate,
                SurfaceSystems.after(TransformSystems::Propagate),
            )
            .add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                (
                    cells::update_cells,
                    debug_view,
                    shelter::update_shelter,
                    count_uniform,
                )
                    .chain()
                    .in_set(SurfaceSystems),
            );
        sf_sky_bevy::sun_view::add_sun_view::<shelter::ShelterFrame>(app);
        if app.get_sub_app(RenderApp).is_none() {
            return;
        }
        load_shader_library!(app, "layer.wesl");
        load_shader_library!(app, "view.wesl");
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<cells::PendingCellWrites>()
            .add_systems(ExtractSchedule, (cells::extract_writes, render::extract_view));
        render::build(render_app);
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

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(SurfaceGpu {
        cells: images.add(cells_image()),
        shelter: shelter::shelter_handle(&mut images),
        noise: images.add(noise_image()),
    });
}

fn debug_view(debug: Res<SurfaceDebug>, mut uniform: ResMut<SurfaceUniform>) {
    if uniform.debug != debug.0 {
        uniform.debug = debug.0;
    }
}

/// Counts uniform changes (the render world writes the buffer on the same change).
fn count_uniform(
    uniform: Res<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
    mut last: Local<Option<[[u32; 4]; U_SLOTS]>>,
) {
    let slots = uniform.slots();
    if *last != Some(slots) {
        *last = Some(slots);
        stats.uniform_uploads += 1;
    }
}
