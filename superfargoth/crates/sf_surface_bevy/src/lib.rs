//! Surface weather on the GPU (surface-weather.md "GPU resources", "Shader integration"; slices
//! W2-W3). Thin: the window, shelter and noise logic is Bevy-free (`sf_surface::view`), the wet
//! math has a CPU twin (`sf_surface::layer`).
//!
//! - **Cell window** ([`cells`]): `Rgba8Unorm` 128², one texel per 32 m cell around the camera
//!   (R wet, G puddle, B snow, A ash), toroidal; strips on recenter, all of it when
//!   [`WorldSurface::generation`] moves.
//! - **Shelter and canopy maps** ([`shelter`]): `Depth32Float` 1024² over 128 m, vertical, two
//!   extra views of Bevy's shadow pass (`sf_sky_bevy::sun_view`) owned by an anchor of their
//!   own (drawn at night too); solid casters in one, [`ShelterFoliage`] leaves (partial
//!   shelter) in the other.
//! - **Exposure pack** ([`exposure`], SW-5): `Rgba16Float` 1024² (ping-pong), the maps'
//!   heights plus the eased exposure's lag, world-anchored; the layer's shelter binding.
//! - **Far shelter level** (SW-5e, [`ShelterFarSettings`]): the same capture and pack over
//!   ±512 m at 1 m texels, casters at a coarser fixed LOD; the layer blends near into far over
//!   the near map's band, far into open sky over its own. [`ShelterReach`] says how far shelter
//!   is known, for the terrain's raise fade.
//! - **Noise atlas**: `Rgba8Unorm` 256², repeat, generated at startup.
//! - **Cover layer array** ([`cover`], W5a): the snow and ash looks the host decoded
//!   ([`CoverLayers`], from `sf_surface::cover::SurfaceLayer` records), `Rgba8Unorm` 512² × 4
//!   slices with mips; a 1×1 neutral placeholder while none is set.
//! - **[`SurfaceUniform`]**: its `vec4` slots, written to a render-world uniform buffer when they
//!   change ([`render`]).
//! - **[`SurfaceWeatherCamera`]**: the exterior camera that gets surface weather (the game marks
//!   it). Its render-world view gets `bevy_pbr::SurfaceWeatherView`: the vendored `bevy_pbr`
//!   patch "SW" binds these resources at mesh-view bindings 43-51 and sets the
//!   `SURFACE_WEATHER` shader def, so every `StandardMaterial` (statics, characters, held items,
//!   fades), the terrain and the far proxies run the layer in the forward main pass.
//! - **WESL**: `sf_surface_bevy::layer` (`apply_surface_layer`, binding-agnostic) and
//!   `sf_surface_bevy::view` (the bindings and the entry points materials call).
//! - **[`SurfaceOrigin`]**, **[`SurfaceSettings`]**, [`NoSurfaceShelter`]: the game's glue (the
//!   floating origin, the A/B switches and preset, terrain patches out of the shelter map).
//!
//! W3 shows wetness (Lagarde's model, film normal flattening, wall streaks), W4 puddles (water
//! level against the height map, flat water, rain rings while it rains), W5b snow and ash cover
//! (slope × shelter × cell, height blend, the look's normals, snow glints); W2's debug views stay
//! (`surface debug exposure|cells|porosity|trails`). W6a: footprints cut the cover ([`trail`]: the
//! trail ring at mesh-view binding 52, stamps from the host's [`TrailStamp`] messages). S1a: whole
//! [`Deformer`] meshes carve too ([`deform`]: a depth view from below, merged into the ring's B).

mod cells;
pub mod deform;
pub mod cover;
mod exposure;
mod probe;
pub mod kickup;
mod render;
mod shelter;
pub mod trail;
mod uniform;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod shader_tests;
#[cfg(test)]
mod deform_tests;
#[cfg(test)]
mod shelter_offset_tests;
#[cfg(test)]
mod probe_tests;

use bevy_app::{App, Plugin, PostUpdate, Startup};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::visibility::{VisibilitySystems, check_visibility_ranges};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_render::{ExtractSchedule, RenderApp};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use sf_coords::WorldPos;
use sf_surface::view::{NOISE_SIZE, WINDOW_CELLS, noise_atlas_mips};
use bevy_time::Time;
use sf_surface::layer::RIPPLE_TIME_WRAP_S;
use sf_tunables::{AnyDef, Def, Source, Unit, def};
use sf_weather_bevy::{CurrentWeather, SurfaceDebug, WorldSurface};
/// `surface probe`: the request and its text (`probe` reads the shelter maps back for it).
pub use sf_weather_bevy::{ProbeRequest, ProbeStatus, SurfaceProbe};

pub use cells::{CellWrite, CellWrites};
pub use cover::{CoverLayers, CoverSource};
pub use deform::{DEFORM_SUBVIEW, DeformGap, DeformGaps, DeformGround, DeformSettings, Deformer, NoDeform};
pub use kickup::{KickupSettings, KickupStats};
pub use shelter::{CANOPY_SUBVIEW, FAR_CANOPY_SUBVIEW, FAR_SHELTER_SUBVIEW, NoSurfaceShelter, SHELTER_SUBVIEW, ShelterFoliage};
pub use sf_surface;
pub use trail::{TrailFrame, TrailStamp};
pub use uniform::{FLAG_ASH, FLAG_PUDDLE, FLAG_RAISE, FLAG_SNOW, FLAG_WET, SurfaceUniform, U_SLOTS};

/// Marks the camera that gets surface weather (one exterior camera; the game inserts and
/// removes it as the camera changes worldspace).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SurfaceWeatherCamera;

/// The floating origin render space is relative to (the game copies its world origin here,
/// as for `sf_wind_bevy::WindOrigin`). The window and the shelter map work in render space.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfaceOrigin(pub WorldPos);

/// `surface.shelter.far_half_m`: [`ShelterFarSettings::half_m`]. It must hold the raised cover's
/// reach on the widest preset (casters end at the near-field handoff, 400 m less its 10 % band on
/// High: 360 m) plus a recentre step and the hand-over band: 512 m x 0.875 - 64 m = 384 m. 1024²
/// texels over it: 1 m, well under a tree crown or a roof.
pub const SHELTER_FAR_HALF_M: Def<f32> = def!(float "surface.shelter.far_half_m" = 512.0)
    .range(128.0, 2048.0)
    .unit(Unit::Meters)
    .help("The far shelter level covers this far around the camera each way (1024² texels): shelter under far trees and roofs is known out to it")
    .source(Source::Industry("stable shadow cascades / clipmaps: the outer level covers the effect's reach (raise to the 400 m handoff)"));

/// `surface.shelter.far_recenter_m`: [`ShelterFarSettings::recenter_m`]. 64 m = 8 near steps;
/// with the 512 m level it keeps 384 m known (above High's 360 m caster reach).
pub const SHELTER_FAR_RECENTER_M: Def<f32> = def!(float "surface.shelter.far_recenter_m" = 64.0)
    .range(8.0, 512.0)
    .unit(Unit::Meters)
    .help("The far shelter level redraws once the camera moved this far from its centre")
    .source(Source::Guess);

/// `surface.shelter.far_lod_distance_m`: [`ShelterFarSettings::lod_distance_m`], the distance every
/// caster's LOD level is picked at for the far maps. At 150 m a reference pixel (1440p, 70°) is
/// ~0.15 m, so that level's error is far under the 1 m texel; it stays under the shortest preset's
/// handoff less its band (Extremely low: 200 m - 20 m), so every static's last near level holds it
/// and proxies (from the handoff) never cast.
pub const SHELTER_FAR_LOD_DISTANCE_M: Def<f32> = def!(float "surface.shelter.far_lod_distance_m" = 150.0)
    .range(0.0, 400.0)
    .unit(Unit::Meters)
    .help("Casters draw into the far shelter maps at the LOD level they show at this distance")
    .source(Source::Industry("fixed content LOD per cascade (texel 1 m vs ~0.15 m pixel error at 150 m)"));

/// This crate's tunables (live binding: tunables slice T2; the far level reads them each frame).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &SHELTER_FAR_HALF_M,
    &SHELTER_FAR_RECENTER_M,
    &SHELTER_FAR_LOD_DISTANCE_M,
    &deform::DEFORM_ENABLED,
    &deform::DEFORM_WINDOW_M,
    &deform::DEFORM_TEXEL_M,
    &deform::DEFORM_RANGE_M,
    &deform::DEFORM_MAX_ACTORS,
    &deform::DEFORM_LOD_DISTANCE_M,
    &deform::DEFORM_SLAB_BELOW_M,
    &deform::DEFORM_GAP_TEXELS,
    &deform::DEFORM_GROUND_STEP_M,
    &deform::DEFORM_LOG_S,
];

/// The far shelter level (SW-5e; `sf_surface::view::ShelterLevel::far`): a second capture of
/// the shelter and canopy maps around the camera, wider and coarser, so shelter is known
/// (deterministically: fixed caster LOD, world-snapped texels) wherever the raised cover stands.
/// Off: the near level alone, open sky past it, and the raise ends where it ends.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ShelterFarSettings {
    pub on: bool,
    /// [`SHELTER_FAR_HALF_M`].
    pub half_m: f32,
    /// [`SHELTER_FAR_RECENTER_M`].
    pub recenter_m: f32,
    /// [`SHELTER_FAR_LOD_DISTANCE_M`].
    pub lod_distance_m: f32,
}

impl Default for ShelterFarSettings {
    fn default() -> Self {
        Self {
            on: SurfaceSettings::default().shelter,
            half_m: SHELTER_FAR_HALF_M.default_value(),
            recenter_m: SHELTER_FAR_RECENTER_M.default_value(),
            lod_distance_m: SHELTER_FAR_LOD_DISTANCE_M.default_value(),
        }
    }
}

impl ShelterFarSettings {
    /// The far level's geometry.
    pub fn level(&self) -> sf_surface::view::ShelterLevel {
        sf_surface::view::ShelterLevel::far(self.half_m, self.recenter_m)
    }
}

/// How far from the camera shelter is known whatever the camera's place (SW-5e, written every
/// frame from the settings): the far level's unfaded reach, the near level's with the far level
/// off, `None` with the shelter off (exposure 1 everywhere: nothing changes with distance). The
/// host passes it with its casters' reach to `sf_surface::view::raise_reach_m` and gives the
/// result to the terrain (`TerrainSettings::raise_reach_m`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShelterReach(pub Option<f32>);

impl ShelterReach {
    /// The reach for these settings.
    pub fn of(surface: &SurfaceSettings, far: &ShelterFarSettings) -> Self {
        Self(surface.shelter.then(|| if far.on { far.level().known_m() } else { sf_surface::view::NEAR_LEVEL.known_m() }))
    }
}

/// A/B switches and the preset (surface-weather.md "A/B switches", "Budget"). Off parts keep
/// the hook compiled in and clear its flags (one uniform branch per pixel).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools, reason = "independent A/B switches, one per --off part")]
pub struct SurfaceSettings {
    /// `--off surface`: the whole layer.
    pub layer: bool,
    /// `--off wet`: wetness.
    pub wet: bool,
    /// `--off puddles` (and the Low preset): standing water and its rain rings (W4).
    pub puddles: bool,
    /// The shelter map (off on Low and with `--off shelter`): exposure 1 everywhere.
    pub shelter: bool,
    /// `--off cover`: snow and ash cover (W5).
    pub cover: bool,
    /// `--off trails`: prints in snow and ash (W6; no stamps, no ring pass, the cover uncut).
    pub trails: bool,
    /// `--off raise`: the raised cover (W6b displacement; SW-5c A/B): off, snow and ash lie flat
    /// (shading only), on the same amounts and look.
    pub raise: bool,
}

impl Default for SurfaceSettings {
    fn default() -> Self {
        Self {
            layer: true,
            wet: true,
            puddles: true,
            shelter: true,
            cover: true,
            trails: true,
            raise: true,
        }
    }
}

impl SurfaceSettings {
    /// The layer flags these settings let through.
    pub fn flag_mask(self) -> u32 {
        if !self.layer {
            return 0;
        }
        let mut mask = FLAG_WET | FLAG_PUDDLE | FLAG_SNOW | FLAG_ASH;
        if !self.wet {
            mask &= !FLAG_WET;
        }
        if !self.puddles {
            mask &= !FLAG_PUDDLE;
        }
        if !self.cover {
            mask &= !(FLAG_SNOW | FLAG_ASH);
        }
        mask
    }

    /// The uniform's layer flags for a cell window with channel maxima `max` (R wet, G puddle, B
    /// snow, A ash): the channels present that these settings let through, plus [`FLAG_RAISE`]
    /// while a cover channel is and the raise is on (SW-5c).
    pub fn flags(self, max: [u8; 4]) -> u32 {
        let f = SurfaceUniform::flags_of(max) & self.flag_mask();
        if self.raise && f & (FLAG_SNOW | FLAG_ASH) != 0 { f | FLAG_RAISE } else { f }
    }
}

/// The GPU resources the layer reads (the uniform buffer is render-world, [`render`]).
#[derive(Resource, Clone, Debug)]
pub struct SurfaceGpu {
    pub cells: Handle<Image>,
    pub shelter: Handle<Image>,
    /// Leaf casters only (partial shelter).
    pub canopy: Handle<Image>,
    /// The far level's maps (SW-5e), same formats.
    pub far_shelter: Handle<Image>,
    pub far_canopy: Handle<Image>,
    pub noise: Handle<Image>,
    /// The cover layer array ([`cover`]): a neutral placeholder until the host inserts
    /// [`CoverLayers`].
    pub cover: Handle<Image>,
}

/// Counters for status tools and tests.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SurfaceStats {
    pub shelter_redraws: u64,
    pub shelter_casters: usize,
    /// Leaf casters in the last canopy redraw.
    pub canopy_casters: usize,
    /// The far level's redraws and its last redraw's solid and leaf casters (SW-5e).
    pub far_redraws: u64,
    pub far_casters: usize,
    pub far_canopy_casters: usize,
    /// Cell texels written (strips and full rewrites).
    pub cell_texels: u64,
    pub full_uploads: u64,
    /// Times the uniform's slots changed (each is one buffer write in the render world).
    pub uniform_uploads: u64,
    /// Slices in the cover layer array (0 before the first pack, 1 = the placeholder).
    pub cover_slices: u32,
    /// Trail stamps taken (W6a; within range, capped per frame).
    pub trail_stamps: u64,
    /// Deformer captures requested (S1a) and the last one's casters.
    pub deform_captures: u64,
    pub deform_casters: usize,
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
            .init_resource::<ShelterFarSettings>()
            .init_resource::<ShelterReach>()
            .init_resource::<SurfaceDebug>()
            .init_resource::<SurfaceProbe>()
            .init_resource::<probe::ProbeState>()
            .init_resource::<shelter::ShelterPending>()
            .init_resource::<TrailFrame>()
            .init_resource::<DeformSettings>()
            .init_resource::<DeformGaps>()
            .add_message::<TrailStamp>()
            .configure_sets(
                PostUpdate,
                // The shelter redraw reads this frame's `InheritedVisibility` and `Aabb` (a caster
                // streamed in this frame is in the redraw its arrival triggers).
                SurfaceSystems
                    .after(TransformSystems::Propagate)
                    .after(VisibilitySystems::VisibilityPropagate)
                    .after(VisibilitySystems::CalculateBounds),
            )
            .add_systems(Startup, (setup, deform::setup))
            .add_systems(
                PostUpdate,
                (
                    cells::update_cells,
                    cover::update_cover,
                    trail::update_trails,
                    deform::update_deform,
                    debug_view,
                    shelter::update_shelter,
                    probe::run_probe,
                    ripple_clock,
                    count_uniform,
                )
                    .chain()
                    .in_set(SurfaceSystems),
            )
            .add_systems(
                PostUpdate,
                (shelter::mark_casters_visible, deform::mark_casters_visible)
                    .in_set(VisibilitySystems::CheckVisibility)
                    .after(SurfaceSystems)
                    .after(check_visibility_ranges),
            );
        kickup::build(app);
        exposure::build(app);
        sf_sky_bevy::sun_view::add_sun_view::<shelter::ShelterFrame>(app);
        sf_sky_bevy::sun_view::add_sun_view::<shelter::CanopyFrame>(app);
        sf_sky_bevy::sun_view::add_sun_view::<shelter::FarShelterFrame>(app);
        sf_sky_bevy::sun_view::add_sun_view::<shelter::FarCanopyFrame>(app);
        sf_sky_bevy::sun_view::add_sun_view::<deform::DeformerFrame>(app);
        if app.get_sub_app(RenderApp).is_none() {
            return;
        }
        load_shader_library!(app, "layer.wesl");
        load_shader_library!(app, "view.wesl");
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<cells::PendingCellWrites>()
            .add_systems(ExtractSchedule, (cells::extract_writes, deform::extract_lod_distance, (trail::extract, exposure::extract, render::extract_view).chain()));
        render::build(render_app);
        trail::build(app);
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

/// The noise atlas with its full mip chain (SW-6: the raised cover reads the drift prefiltered to
/// its filter width; every other read is at level 0). The render world's sampler (47) filters
/// between mips linearly.
fn noise_image() -> Image {
    let extent = Extent3d { width: NOISE_SIZE, height: NOISE_SIZE, depth_or_array_layers: 1 };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, TextureFormat::Rgba8Unorm, RenderAssetUsages::RENDER_WORLD);
    image.texture_descriptor.mip_level_count = NOISE_SIZE.trailing_zeros() + 1;
    image.data = Some(noise_atlas_mips().into_iter().flatten().collect());
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
        canopy: shelter::shelter_handle(&mut images),
        far_shelter: shelter::shelter_handle(&mut images),
        far_canopy: shelter::shelter_handle(&mut images),
        noise: images.add(noise_image()),
        cover: images.add(cover::cover_image(&cover::pack(&[None, None]))),
    });
    shelter::spawn_anchor(&mut commands);
}

fn debug_view(debug: Res<SurfaceDebug>, mut uniform: ResMut<SurfaceUniform>) {
    if uniform.debug != debug.0 {
        uniform.debug = debug.0;
    }
}

/// The puddles' rain rings: the rain at the camera (the weather frame's wetting rate over the
/// storm's, `SurfaceTuning::storm_wet_rate`) and the animation seconds (`Time`, not the game
/// clock, wrapped on whole ring cycles). Both stay put while no rings can show (no puddles in the
/// window or no rain), so a dry frame writes no uniform; rain 0 stops the rings at once.
fn ripple_clock(
    weather: Option<Res<CurrentWeather>>,
    surface: Option<Res<WorldSurface>>,
    time: Option<Res<Time>>,
    mut uniform: ResMut<SurfaceUniform>,
) {
    let rain = match (&weather, &surface) {
        (Some(w), Some(s)) if w.valid && uniform.flags & FLAG_PUDDLE != 0 => {
            rain_of(w.frame.surface.wet, s.tuning.storm_wet_rate)
        }
        _ => 0.0,
    };
    if uniform.time.y.to_bits() != rain.to_bits() {
        uniform.time.y = rain;
    }
    if rain > 0.0
        && let Some(time) = time
    {
        #[allow(clippy::cast_possible_truncation)]
        let seconds = time.elapsed_secs_f64().rem_euclid(f64::from(RIPPLE_TIME_WRAP_S)) as f32;
        uniform.time.x = seconds;
    }
}

/// Rain intensity (0..1) from a wetting rate and the storm's.
pub(crate) fn rain_of(wet_rate: f32, storm_wet_rate: f32) -> f32 {
    if storm_wet_rate > 0.0 {
        (wet_rate / storm_wet_rate).clamp(0.0, 1.0)
    } else {
        0.0
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
