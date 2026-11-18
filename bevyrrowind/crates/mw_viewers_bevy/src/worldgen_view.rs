//! `worldgen-view <dir> [<dir> ...]` (worldgen.md, slices WG-V, WG-V2): generated terrain in 3D, to judge guide
//! grids (`sf_worldgen::guide`, format v1) at scale where hillshades are hard to read. Each folder is an arm on a
//! number key; switching keeps the camera, so arms are compared from one viewpoint.
//!
//! Terrain: chunked meshes at the guide's native spacing ([`mesh`]), three LODs picked by camera distance with
//! skirts against cracks, neutral slope/altitude vertex colours with a low-frequency tint, an opaque sea plane at
//! the meta's sea level (chunks entirely under it aren't built). Light: one sun with cascaded shadows over tens of
//! km, low ambient. Arms load on a background thread, coarse LODs first, so the island appears within a second and
//! sharpens; the HUD shows progress.
//!
//! Scale references (WG-V2, [`refs`]): town markers from `mw_worldgen`'s `markers.json` (`--markers`) at their
//! scaled positions but true size (one 117 m block per source cell, labelled), the source summit as a red pole,
//! 1.8 m figures dropped in front of the camera, tree cones (8-20 m) on gentle ground below a treeline, aerial
//! perspective (Bevy's atmosphere with a denser aerosol term, or the WG-V distance fog, or none), contour lines
//! (`terrain.wesl`), and the distance to the nearest marker in the HUD.
//!
//! Keys: WASD move, mouse look (Esc frees the mouse, a left click takes it back), Space/E up, Ctrl/Q down, Shift
//! fast, wheel fly speed, G walk mode (on the ground at eye height, Shift runs), 1-9 arm, [ ] sun azimuth,
//! , . sun elevation, M markers, T trees, C contours, F haze (atmosphere / distance fog / off), P drops a figure,
//! O clears the figures, H hides the HUD and labels, F10 quits.
//!
//! Frame: x east, y up, z south (north = -z), metres, centred on the first arm's grid; arms are placed by their
//! `origin_m` and spacing, so arms of one region line up.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "grid sizes, counts and metres shown or placed in f32"
)]

pub mod arms;
pub mod mesh;
pub mod refs;
#[cfg(test)]
mod tests;

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use bevy::asset::{AssetPath, LoadState as AssetLoadState, RenderAssetUsages, embedded_asset, embedded_path};
use bevy::camera::Hdr;
use bevy::camera::primitives::Aabb;
use bevy::camera::visibility::NoAutoAabb;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::light::atmosphere::{Falloff, ScatteringMedium};
use bevy::light::{Atmosphere, CascadeShadowConfigBuilder, DirectionalLightShadowMap, GlobalAmbientLight, NotShadowCaster};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::{AtmosphereSettings, DistanceFog, ExtendedMaterial, FogFalloff, MaterialExtension, MaterialPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowMode, WindowResolution};

pub use arms::{Arm, MAX_ARMS};
use mesh::{ChunkMesh, ChunkRect, LOD_STRIDES, Terrain, Tint};
use mw_worldgen::MarkersFile;
use refs::{Contours, Placed, TreeParams};

/// Viewer settings: camera feel, light, fog, LOD distances, scale references. Command-line flags of
/// `worldgen-view`, defaults documented on each (Guess values for a look-test viewer, not game tunables).
#[derive(Debug, Clone, clap::Args, Resource)]
pub struct WorldgenViewSettings {
    /// Window size `WxH` (a centred window); default borderless fullscreen.
    #[arg(long, value_parser = parse_size)]
    pub resolution: Option<(u32, u32)>,
    /// Chunk edge, samples (a multiple of 16, the coarsest LOD stride).
    #[arg(long, default_value_t = 256)]
    pub chunk: usize,
    /// Camera distances, metres, past which a chunk drops to LOD 1 (every 4th sample) and LOD 2 (every 16th).
    #[arg(long, default_value = "6000,24000", value_parser = parse_pair)]
    pub lod_m: [f32; 2],
    /// Extra skirt depth below the measured LOD gap, metres.
    #[arg(long, default_value_t = 2.0)]
    pub skirt_margin_m: f32,
    /// Chunk meshes uploaded per frame while loading (more loads faster, hitches more).
    #[arg(long, default_value_t = 6)]
    pub uploads_per_frame: usize,
    /// Mesh-building threads (0: half the logical CPUs).
    #[arg(long, default_value_t = 0)]
    pub threads: usize,
    /// Fly speed at start, m/s (the wheel scales it).
    #[arg(long, default_value_t = 150.0)]
    pub fly_speed: f32,
    /// Fly speed range the wheel stays in, m/s.
    #[arg(long, default_value = "2,2000", value_parser = parse_pair)]
    pub fly_speed_range: [f32; 2],
    /// Wheel factor per notch.
    #[arg(long, default_value_t = 1.25)]
    pub wheel_step: f32,
    /// Shift multiplies the fly speed by this.
    #[arg(long, default_value_t = 5.0)]
    pub fast: f32,
    /// Walk mode speeds, m/s: walking, running (Shift).
    #[arg(long, default_value = "1.5,6", value_parser = parse_pair)]
    pub walk_run: [f32; 2],
    /// Eye height above the ground in walk mode, metres (also the fly mode's lowest clearance).
    #[arg(long, default_value_t = 1.7)]
    pub eye_height_m: f32,
    /// Mouse look, degrees per pixel of motion.
    #[arg(long, default_value_t = 0.08)]
    pub look_deg_per_px: f32,
    /// Vertical field of view, degrees.
    #[arg(long, default_value_t = 60.0)]
    pub fov_deg: f32,
    /// Sun azimuth at start, degrees clockwise from north (240: west-south-west, afternoon).
    #[arg(long, default_value_t = 240.0, allow_hyphen_values = true)]
    pub sun_azimuth_deg: f32,
    /// Sun elevation at start, degrees above the horizon (low sun shows relief).
    #[arg(long, default_value_t = 25.0)]
    pub sun_elevation_deg: f32,
    /// Sun keys turn it at this rate, degrees per second.
    #[arg(long, default_value_t = 30.0)]
    pub sun_rate_deg_s: f32,
    /// Sun illuminance, lux (Bevy's default exposure).
    #[arg(long, default_value_t = 15_000.0)]
    pub sun_lux: f32,
    /// Ambient light brightness, cd/m² (low, so shaded slopes read).
    #[arg(long, default_value_t = 300.0)]
    pub ambient: f32,
    /// Haze at start (F cycles): `atmosphere` (Bevy's atmosphere: sky + height-dependent aerial perspective),
    /// `distance` (WG-V's distance fog, `--fog-visibility-m`), `off`.
    #[arg(long, value_enum, default_value_t = Haze::Atmosphere)]
    pub haze: Haze,
    /// Aerosol (Mie) density of the atmosphere, times Earth's clear-day value (Hillaire 2020: scattering 3.996e-6,
    /// absorption 0.444e-6 per metre); 15 is a hazy summer day, ~50 km visibility at sea level.
    #[arg(long, default_value_t = 15.0)]
    pub aerosol: f32,
    /// Aerosol scale height, metres (density falls by e per this height: summits rise out of the haze).
    #[arg(long, default_value_t = 1200.0)]
    pub aerosol_height_m: f32,
    /// Reach of the atmosphere's aerial-perspective volume, metres (beyond it the in-scattering stays at its value
    /// there; transmittance stays exact).
    #[arg(long, default_value_t = 80_000.0)]
    pub aerial_max_m: f32,
    /// Distance fog visibility (haze `distance`), metres (Koschmieder distance where contrast drops to 5%).
    #[arg(long, default_value_t = 70_000.0)]
    pub fog_visibility_m: f32,
    /// Far bound of the first shadow cascade and the shadow distance, metres.
    #[arg(long, default_value = "800,40000", value_parser = parse_pair)]
    pub shadow_m: [f32; 2],
    /// Shadow map size per cascade, texels (a power of two).
    #[arg(long, default_value_t = 4096)]
    pub shadow_map: usize,
    /// Start pose: horizontal distance south of the first arm's highest point, metres...
    #[arg(long, default_value_t = 15_000.0)]
    pub start_distance_m: f32,
    /// ...and height above the ground there, metres.
    #[arg(long, default_value_t = 500.0)]
    pub start_height_m: f32,
    /// Vertex-colour variation amplitude, fraction of the colour (0: flat colours).
    #[arg(long, default_value_t = 0.08)]
    pub tint: f32,
    /// Vertex-colour variation wavelength, metres (a second octave at a quarter of it).
    #[arg(long, default_value_t = 600.0)]
    pub tint_wavelength_m: f32,
    /// Town markers: a `markers.json` from `bevyrrowind worldgen-dump` (source metres).
    #[arg(long)]
    pub markers: Option<PathBuf>,
    /// Arm frame per source metre: where a marker goes (S2/S2b arms: the guide frame x 10). The marker's own
    /// size is never scaled.
    #[arg(long, default_value_t = 10.0)]
    pub marker_scale: f64,
    /// Markers with fewer named cells are not drawn (1: every named place, forts and ruins included).
    #[arg(long, default_value_t = 1)]
    pub marker_min_cells: usize,
    /// Marker block height above the highest ground under it, metres (two to four storeys).
    #[arg(long, default_value_t = 15.0)]
    pub marker_height_m: f32,
    /// Source summit pole: height above the ground and width, metres.
    #[arg(long, default_value = "60,4", value_parser = parse_pair)]
    pub summit_pole_m: [f32; 2],
    /// Labels show for markers closer than this, metres.
    #[arg(long, default_value_t = 25_000.0)]
    pub label_distance_m: f32,
    /// Label anchor above the marker's top, metres.
    #[arg(long, default_value_t = 30.0)]
    pub label_lift_m: f32,
    /// Figure (P): height, metres, and how far in front of the camera it lands.
    #[arg(long, default_value = "1.8,12", value_parser = parse_pair)]
    pub figure_m: [f32; 2],
    /// Tree candidate grid, metres (at most one tree per cell).
    #[arg(long, default_value_t = 40.0)]
    pub tree_spacing_m: f32,
    /// Share of tree candidates kept, 0..1 (0: no trees built).
    #[arg(long, default_value_t = 0.35)]
    pub tree_density: f32,
    /// Tree cone heights, metres: lowest, highest.
    #[arg(long, default_value = "8,20", value_parser = parse_pair)]
    pub tree_height_m: [f32; 2],
    /// Tree base radius per metre of height.
    #[arg(long, default_value_t = 0.22)]
    pub tree_radius_ratio: f32,
    /// Steepest ground with trees, degrees.
    #[arg(long, default_value_t = 25.0)]
    pub tree_max_slope_deg: f32,
    /// Treeline, metres above the sea.
    #[arg(long, default_value_t = 1800.0)]
    pub tree_max_m: f32,
    /// Forest-patch noise wavelength, metres (0: uniform).
    #[arg(long, default_value_t = 1500.0)]
    pub tree_clump_m: f32,
    /// Trees show within this camera distance of their chunk, metres.
    #[arg(long, default_value_t = 8000.0)]
    pub tree_distance_m: f32,
    /// Contour intervals, metres: minor, major.
    #[arg(long, default_value = "100,500", value_parser = parse_pair)]
    pub contour_m: [f32; 2],
    /// Contour line widths, pixels: minor, major.
    #[arg(long, default_value = "1,2.5", value_parser = parse_pair)]
    pub contour_px: [f32; 2],
    /// Contours on at start (C toggles).
    #[arg(long)]
    pub contours: bool,
    /// Plain `StandardMaterial` terrain, no contour shader (a fallback if `terrain.wesl` fails to compile).
    #[arg(long)]
    pub plain_terrain: bool,
}

/// Aerial perspective modes (F cycles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Haze {
    Atmosphere,
    Distance,
    Off,
}

impl Haze {
    fn next(self) -> Self {
        match self {
            Self::Atmosphere => Self::Distance,
            Self::Distance => Self::Off,
            Self::Off => Self::Atmosphere,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Atmosphere => "atmosphere",
            Self::Distance => "distance fog",
            Self::Off => "off",
        }
    }
}

fn parse_pair(s: &str) -> Result<[f32; 2], String> {
    let (a, b) = s.split_once(',').ok_or_else(|| format!("{s:?}: two numbers, `a,b`"))?;
    let a: f32 = a.trim().parse().map_err(|e| format!("{s:?}: {e}"))?;
    let b: f32 = b.trim().parse().map_err(|e| format!("{s:?}: {e}"))?;
    Ok([a, b])
}

fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let (w, h) = s.split_once('x').ok_or_else(|| format!("{s:?}: WxH"))?;
    let w: u32 = w.trim().parse().map_err(|e| format!("{s:?}: {e}"))?;
    let h: u32 = h.trim().parse().map_err(|e| format!("{s:?}: {e}"))?;
    Ok((w, h))
}

/// Opens the viewer over `arms`; returns when the window closes (false on a bad setting).
pub fn run(arms: Vec<Arm>, settings: WorldgenViewSettings) -> bool {
    if settings.chunk == 0 || !settings.chunk.is_multiple_of(LOD_STRIDES[LOD_STRIDES.len() - 1]) {
        eprintln!("worldgen-view: --chunk {} must be a positive multiple of {}", settings.chunk, LOD_STRIDES[2]);
        return false;
    }
    let markers = match settings.markers.as_deref().map(mw_worldgen::read_markers).transpose() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("worldgen-view: --markers {}: {e}", settings.markers.as_deref().map_or_else(String::new, |p| p.display().to_string()));
            return false;
        }
    };
    if let Some(m) = &markers {
        info!("worldgen-view: {} markers (x{} positions, true size), summit {}", m.markers.len(), settings.marker_scale, m.summit.is_some());
    }
    let names: Vec<String> = arms.iter().map(|a| a.name.clone()).collect();
    for (i, arm) in arms.iter().enumerate() {
        info!("worldgen-view: [{}] {} = {}", i + 1, arm.name, arm.dir.display());
    }
    let rx = spawn_loader(arms, &settings);
    let mut window = Window { title: "Bevyrrowind - worldgen view".into(), ..default() };
    if let Some((w, h)) = settings.resolution {
        window.resolution = WindowResolution::new(w, h).with_scale_factor_override(1.0);
        window.position = WindowPosition::Centered(MonitorSelection::Primary);
    } else {
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(window), ..default() }))
        .add_plugins(TerrainMaterialPlugin)
        .insert_resource(ClearColor(FOG))
        .insert_resource(GlobalAmbientLight { color: SKY_AMBIENT, brightness: settings.ambient, ..default() })
        .insert_resource(DirectionalLightShadowMap { size: settings.shadow_map.next_power_of_two() })
        .insert_resource(Loader(Mutex::new(rx)))
        .insert_resource(View::new(&names, &settings))
        .insert_resource(MarkerData(markers))
        .insert_resource(Cam {
            pos: Vec3::new(0.0, 2_000.0, 0.0),
            yaw: 0.0,
            pitch: -0.2,
            speed: settings.fly_speed,
            walk: false,
            grabbed: true,
            placed: false,
        })
        .insert_resource(Sun { azimuth_deg: settings.sun_azimuth_deg, elevation_deg: settings.sun_elevation_deg })
        .init_resource::<Labels>()
        .insert_resource(settings)
        .add_systems(Startup, setup)
        .add_systems(Update, (receive, keys, toggles, control, (pick_lods, sun, hud, labels)).chain())
        .add_systems(Update, terrain_shader_fallback.before(receive));
    if !app.world().resource::<WorldgenViewSettings>().plain_terrain {
        // The same handle `MaterialPlugin` holds: its load state drives the fallback to plain terrain.
        let shader = app.world().resource::<AssetServer>().load(terrain_shader_path());
        app.insert_resource(TerrainShader(shader));
        info!(
            "worldgen-view: contour terrain shader {}; a shader compile error in this log leaves the terrain \
             invisible: re-run with --plain-terrain",
            terrain_shader_path()
        );
    }
    app.run().is_success()
}

// Look-test colours (sRGB): a hazy sky for the fog and clear colour, a sky-blue ambient, the sea, the references.
const FOG: Color = Color::srgb(0.66, 0.73, 0.82);
const SKY_AMBIENT: Color = Color::srgb(0.75, 0.82, 1.0);
const SUN_GLOW: Color = Color::srgb(1.0, 0.92, 0.75);
const SEA: Color = Color::srgb(0.05, 0.11, 0.18);
/// Marker blocks (plaster), the source summit pole, figures, contour ink.
const MARKER: [f32; 3] = [0.90, 0.86, 0.78];
const SUMMIT_POLE: [f32; 3] = [0.85, 0.15, 0.10];
const FIGURE: Color = Color::srgb(0.95, 0.25, 0.10);
const CONTOUR_INK: [f32; 3] = [0.08, 0.06, 0.05];
/// How far the sea plane reaches past the grid, metres (fog hides its edge).
// tunables: const (past the fog: the sea's edge is never seen)
const SEA_MARGIN_M: f32 = 150_000.0;
/// Camera far plane, metres: the sea's edge and the whole grid from anywhere over it.
// tunables: const (past the fog)
const FAR_M: f32 = 250_000.0;
/// Relative band around a LOD switch distance where a chunk keeps its current LOD (no flicker at the edge).
// tunables: const (switch hysteresis of the look-test viewer)
const LOD_HYSTERESIS: f32 = 0.1;
/// Figures kept at once (P recycles the oldest).
// tunables: const (a pool size of the look-test viewer)
const MAX_FIGURES: usize = 32;
/// A figure's radius per metre of height (a 1.8 m figure is 0.5 m wide).
// tunables: const (human proportion of a scale cue)
const FIGURE_RADIUS_RATIO: f32 = 0.14;

/// The terrain material: Bevy's `StandardMaterial` plus contour lines (`terrain.wesl`).
pub type TerrainMaterial = ExtendedMaterial<StandardMaterial, ContourExt>;

/// `terrain.wesl`'s uniforms.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct ContourExt {
    /// (minor interval m, major interval m, minor width px, major width px).
    #[uniform(100)]
    pub style: Vec4,
    /// (ink rgb linear, strength; 0 = off).
    #[uniform(101)]
    pub colour: Vec4,
}

impl MaterialExtension for ContourExt {
    fn fragment_shader() -> ShaderRef {
        terrain_shader_path().into()
    }
}

/// The contour terrain material and its embedded shader. The shader is embedded *before* `MaterialPlugin` is
/// added: that plugin's `build` loads `fragment_shader()` at once on the IO task pool, so an `embedded_asset!`
/// registered after it races that read, and a lost race fails the load ("Path not found") for good: no terrain.
struct TerrainMaterialPlugin;

impl Plugin for TerrainMaterialPlugin {
    fn build(&self, app: &mut App) {
        embed_terrain_shader(app);
        app.add_plugins(MaterialPlugin::<TerrainMaterial>::default());
    }
}

/// Embeds `terrain.wesl` at [`terrain_shader_path`].
fn embed_terrain_shader(app: &mut App) {
    embedded_asset!(app, "worldgen_view/terrain.wesl");
}

/// `terrain.wesl`'s asset path, computed exactly as `embedded_asset!` registers it (`embedded_path!`, same file).
fn terrain_shader_path() -> AssetPath<'static> {
    AssetPath::from_path_buf(embedded_path!("worldgen_view/terrain.wesl")).with_source("embedded")
}

/// The contour shader's handle (contour terrain only).
#[derive(Resource)]
struct TerrainShader(Handle<Shader>);

/// The plain terrain material: the contour material's base, and the whole `--plain-terrain` material.
fn terrain_base() -> StandardMaterial {
    StandardMaterial { base_color: Color::WHITE, perceptual_roughness: 0.92, reflectance: 0.2, ..default() }
}

/// If the contour shader fails to load, logs it and swaps the terrain to plain `StandardMaterial` (as
/// `--plain-terrain`), so a broken shader never leaves the terrain invisible. Runs before `receive`, so chunks
/// spawned from then on get the plain material.
fn terrain_shader_fallback(
    mut commands: Commands,
    server: Res<AssetServer>,
    shader: Option<Res<TerrainShader>>,
    mut view: ResMut<View>,
    terrain_materials: Res<Assets<TerrainMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunks: Query<Entity, With<MeshMaterial3d<TerrainMaterial>>>,
) {
    let (Some(shader), TerrainMat::Contour(contour)) = (shader, &view.terrain_material) else {
        return;
    };
    let Some(AssetLoadState::Failed(err)) = server.get_load_state(shader.0.id()) else {
        return;
    };
    error!(
        "worldgen-view: the contour terrain shader {} failed to load ({err}); falling back to plain \
         StandardMaterial terrain, no contours (as --plain-terrain)",
        terrain_shader_path()
    );
    let base = terrain_materials.get(contour).map_or_else(terrain_base, |m| m.base.clone());
    let plain = materials.add(base);
    for e in &chunks {
        commands.entity(e).remove::<MeshMaterial3d<TerrainMaterial>>().insert(MeshMaterial3d(plain.clone()));
    }
    view.terrain_material = TerrainMat::Plain(plain);
    commands.remove_resource::<TerrainShader>();
}

// ---------------------------------------------------------------------------------------------------------------
// Loading.

/// What the loader thread reports.
enum Msg {
    /// The guide is read: the terrain, its world offset (sample (0, 0)), its arm-frame origin and the chunks that
    /// will be built.
    Read { arm: usize, terrain: Arc<Terrain>, offset: Vec3, origin_m: [f64; 2], chunks: Vec<ChunkRect>, secs: f32 },
    Chunk { arm: usize, chunk: usize, lod: usize, mesh: ChunkMesh },
    Trees { arm: usize, chunk: usize, mesh: ChunkMesh, count: usize },
    Done { arm: usize, secs: f32 },
    Failed { arm: usize, error: String },
}

#[derive(Resource)]
struct Loader(Mutex<Receiver<Msg>>);

/// What the loader builds with.
#[derive(Clone, Copy)]
struct Build {
    chunk: usize,
    margin: f32,
    threads: usize,
    tint: Tint,
    trees: TreeParams,
}

/// Starts the loader: arms in order, each read then built coarse LOD first, then trees, chunks on a few threads.
fn spawn_loader(arms: Vec<Arm>, settings: &WorldgenViewSettings) -> Receiver<Msg> {
    // A few chunks in flight: the main thread uploads at its own pace, the workers wait for it.
    let (tx, rx) = sync_channel(4 * settings.uploads_per_frame.max(1));
    let threads = if settings.threads == 0 {
        std::thread::available_parallelism().map_or(2, |n| (n.get() / 2).max(1))
    } else {
        settings.threads
    };
    let build = Build {
        chunk: settings.chunk,
        margin: settings.skirt_margin_m,
        threads,
        tint: Tint { amount: settings.tint, wavelength_m: settings.tint_wavelength_m },
        trees: tree_params(settings),
    };
    std::thread::Builder::new()
        .name("worldgen-view loader".into())
        .spawn(move || load_all(&arms, build, &tx))
        .expect("spawn the loader thread");
    rx
}

fn tree_params(s: &WorldgenViewSettings) -> TreeParams {
    TreeParams {
        spacing_m: s.tree_spacing_m,
        density: s.tree_density.clamp(0.0, 1.0),
        height_m: s.tree_height_m,
        radius_ratio: s.tree_radius_ratio,
        max_slope_deg: s.tree_max_slope_deg,
        max_above_sea_m: s.tree_max_m,
        clump_m: s.tree_clump_m,
    }
}

/// One loader job: a chunk at a LOD, or a chunk's trees.
#[derive(Clone, Copy)]
enum Job {
    Lod(usize, usize),
    Trees(usize),
}

fn load_all(arms: &[Arm], build: Build, tx: &SyncSender<Msg>) {
    let mut reference: Option<[f64; 2]> = None;
    for (arm, a) in arms.iter().enumerate() {
        let started = Instant::now();
        let guide = match sf_worldgen::guide::Guide::read(&a.dir) {
            Ok(g) => g,
            Err(e) => {
                if tx.send(Msg::Failed { arm, error: e.to_string() }).is_err() {
                    return;
                }
                continue;
            }
        };
        let [ox, oy] = guide.origin_m;
        let (w, h, sp) = (guide.w, guide.h, guide.spacing_m);
        // The frame's centre: the first arm's grid centre, in guide metres (x east, y north).
        let [rx, ry] = *reference.get_or_insert([ox + (w - 1) as f64 * sp / 2.0, oy - (h - 1) as f64 * sp / 2.0]);
        let offset = Vec3::new((ox - rx) as f32, 0.0, -(oy - ry) as f32);
        let mut terrain = Terrain::new(w, h, sp as f32, guide.sea_level_m as f32, guide.heights);
        terrain.tint = build.tint;
        let terrain = Arc::new(terrain);
        let dry: Vec<ChunkRect> =
            mesh::chunks(w, h, build.chunk).into_iter().filter(|r| !mesh::chunk_wet(&terrain, *r)).collect();
        let secs = started.elapsed().as_secs_f32();
        let read = Msg::Read { arm, terrain: Arc::clone(&terrain), offset, origin_m: guide.origin_m, chunks: dry.clone(), secs };
        if tx.send(read).is_err() {
            return;
        }
        // Coarse first: the whole arm shows after the cheap LOD 2 pass, then sharpens; trees last.
        let mut jobs: Vec<Job> =
            (0..LOD_STRIDES.len()).rev().flat_map(|lod| (0..dry.len()).map(move |k| Job::Lod(lod, k))).collect();
        if build.trees.density > 0.0 {
            jobs.extend((0..dry.len()).map(Job::Trees));
        }
        let next = AtomicUsize::new(0);
        let stop = std::sync::atomic::AtomicBool::new(false);
        std::thread::scope(|s| {
            for _ in 0..build.threads {
                s.spawn(|| {
                    while !stop.load(Ordering::Relaxed) {
                        let Some(&job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) else { break };
                        let msg = match job {
                            Job::Lod(lod, k) => {
                                let depth = mesh::skirt_depths(&terrain, dry[k], build.margin)[lod];
                                Msg::Chunk { arm, chunk: k, lod, mesh: mesh::build_chunk(&terrain, dry[k], LOD_STRIDES[lod], depth) }
                            }
                            Job::Trees(k) => {
                                let rect = dry[k];
                                let trees = refs::scatter_trees(&terrain, rect, guide.origin_m, &build.trees);
                                let origin = [rect.c0 as f32 * terrain.spacing, rect.r0 as f32 * terrain.spacing];
                                let mesh = refs::tree_mesh(&trees, origin, build.trees.max_slope_deg);
                                Msg::Trees { arm, chunk: k, mesh, count: trees.len() }
                            }
                        };
                        if tx.send(msg).is_err() {
                            stop.store(true, Ordering::Relaxed);
                        }
                    }
                });
            }
        });
        if stop.load(Ordering::Relaxed) || tx.send(Msg::Done { arm, secs: started.elapsed().as_secs_f32() }).is_err() {
            return;
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// State.

/// The terrain's material: with contours, or plain (`--plain-terrain`).
enum TerrainMat {
    Contour(Handle<TerrainMaterial>),
    Plain(Handle<StandardMaterial>),
}

/// The arms and what of them is loaded, plus the reference toggles.
#[derive(Resource)]
#[allow(clippy::struct_excessive_bools, reason = "independent key toggles")]
struct View {
    arms: Vec<ArmView>,
    current: usize,
    terrain_material: TerrainMat,
    sea_material: Handle<StandardMaterial>,
    /// Vertex-coloured, no contours: trees and marker blocks.
    props_material: Handle<StandardMaterial>,
    figure_material: Handle<StandardMaterial>,
    figure_mesh: Handle<Mesh>,
    figures: Vec<Entity>,
    hud: bool,
    markers: bool,
    trees: bool,
    contours: bool,
    haze: Haze,
}

struct ArmView {
    name: String,
    root: Option<Entity>,
    terrain: Option<Arc<Terrain>>,
    offset: Vec3,
    chunks: Vec<ChunkView>,
    built: usize,
    trees_built: usize,
    tree_count: usize,
    state: LoadState,
    /// Markers placed on this arm, and the entity holding their blocks.
    markers: Vec<Placed>,
    marker_root: Option<Entity>,
}

#[derive(Debug, Clone, PartialEq)]
enum LoadState {
    Reading,
    Building,
    Ready,
    Failed(String),
}

struct ChunkView {
    rect: ChunkRect,
    lods: [Option<Entity>; LOD_STRIDES.len()],
    trees: Option<Entity>,
    trees_shown: bool,
    /// World bounds (from the first mesh that arrived).
    min: Vec3,
    max: Vec3,
    shown: Option<usize>,
}

impl View {
    fn new(names: &[String], settings: &WorldgenViewSettings) -> Self {
        let arms = names
            .iter()
            .map(|name| ArmView {
                name: name.clone(),
                root: None,
                terrain: None,
                offset: Vec3::ZERO,
                chunks: Vec::new(),
                built: 0,
                trees_built: 0,
                tree_count: 0,
                state: LoadState::Reading,
                markers: Vec::new(),
                marker_root: None,
            })
            .collect();
        Self {
            arms,
            current: 0,
            terrain_material: TerrainMat::Plain(Handle::default()),
            sea_material: Handle::default(),
            props_material: Handle::default(),
            figure_material: Handle::default(),
            figure_mesh: Handle::default(),
            figures: Vec::new(),
            hud: true,
            markers: true,
            trees: true,
            contours: settings.contours,
            haze: settings.haze,
        }
    }

    /// The current arm's ground height under world (x, z): `None` off its grid or before it is read.
    fn ground(&self, pos: Vec3) -> Option<f32> {
        let arm = &self.arms[self.current];
        let t = arm.terrain.as_ref()?;
        t.height_at(pos.x - arm.offset.x, pos.z - arm.offset.z)
    }

    /// The surface the camera stands on: the ground, or the sea where it is higher or off the grid.
    fn surface(&self, pos: Vec3) -> f32 {
        let sea = self.sea_level();
        self.ground(pos).map_or(sea, |g| g.max(sea))
    }

    fn sea_level(&self) -> f32 {
        self.arms[self.current].terrain.as_ref().map_or(0.0, |t| t.sea_level)
    }
}

/// `--markers`, read at start.
#[derive(Resource)]
struct MarkerData(Option<MarkersFile>);

/// The camera pose and mode (the transform follows it).
#[derive(Resource)]
struct Cam {
    pos: Vec3,
    yaw: f32,
    pitch: f32,
    /// Fly speed, m/s.
    speed: f32,
    walk: bool,
    grabbed: bool,
    /// The start pose was set from the first arm.
    placed: bool,
}

impl Cam {
    fn rotation(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0)
    }

    /// Looks from `pos` at `target`.
    fn look_at(&mut self, target: Vec3) {
        let d = target - self.pos;
        self.yaw = (-d.x).atan2(-d.z);
        self.pitch = d.y.atan2(Vec2::new(d.x, d.z).length());
    }
}

#[derive(Resource)]
struct Sun {
    azimuth_deg: f32,
    elevation_deg: f32,
}

impl Sun {
    /// Unit vector toward the sun (azimuth clockwise from north = -z).
    fn toward(&self) -> Vec3 {
        let (az, el) = (self.azimuth_deg.to_radians(), self.elevation_deg.to_radians());
        Vec3::new(az.sin() * el.cos(), el.sin(), -az.cos() * el.cos())
    }
}

#[derive(Component)]
struct Hud;

/// A marker label (a screen-space text placed each frame).
#[derive(Component)]
struct Label;

/// The label pool, one per marker of the current arm (grown on demand).
#[derive(Resource, Default)]
struct Labels(Vec<Entity>);

// ---------------------------------------------------------------------------------------------------------------
// Systems.

#[allow(clippy::too_many_arguments, reason = "a Bevy system")]
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_materials: ResMut<Assets<TerrainMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut media: ResMut<Assets<ScatteringMedium>>,
    mut view: ResMut<View>,
    settings: Res<WorldgenViewSettings>,
    sun: Res<Sun>,
) {
    let base = terrain_base();
    view.terrain_material = if settings.plain_terrain {
        TerrainMat::Plain(materials.add(base.clone()))
    } else {
        TerrainMat::Contour(terrain_materials.add(TerrainMaterial { base: base.clone(), extension: contour_ext(&settings, view.contours) }))
    };
    view.props_material = materials.add(StandardMaterial { perceptual_roughness: 0.85, ..base });
    view.sea_material =
        materials.add(StandardMaterial { base_color: SEA, perceptual_roughness: 0.3, reflectance: 0.4, ..default() });
    view.figure_material = materials.add(StandardMaterial { base_color: FIGURE, perceptual_roughness: 0.7, ..default() });
    let [height, _] = settings.figure_m;
    let radius = height * FIGURE_RADIUS_RATIO;
    view.figure_mesh = meshes.add(Capsule3d::new(radius, (height - 2.0 * radius).max(0.0)));
    for (i, arm) in view.arms.iter_mut().enumerate() {
        let visibility = if i == 0 { Visibility::Inherited } else { Visibility::Hidden };
        arm.root = Some(commands.spawn((Name::new(format!("arm {}", arm.name)), Transform::IDENTITY, visibility)).id());
    }
    // The planet: Bevy's Earth with a denser, tunable aerosol term (Hillaire 2020's Mie coefficients).
    let shell_m = {
        let earth = Atmosphere::earth(Handle::default());
        earth.outer_radius - earth.inner_radius
    };
    let medium = media.add(haze_medium(settings.aerosol, settings.aerosol_height_m, shell_m));
    commands.spawn(Atmosphere::earth(medium));
    let camera = commands
        .spawn((
            Camera3d::default(),
            Hdr,
            Projection::Perspective(PerspectiveProjection {
                fov: settings.fov_deg.to_radians(),
                near: 0.3,
                far: FAR_M,
                ..PerspectiveProjection::default()
            }),
            Transform::from_xyz(0.0, 2_000.0, 0.0),
        ))
        .id();
    apply_haze(&mut commands.entity(camera), view.haze, &settings);
    let [first, far] = settings.shadow_m;
    commands.spawn((
        DirectionalLight { illuminance: settings.sun_lux, shadow_maps_enabled: true, ..default() },
        Transform::IDENTITY.looking_to(-sun.toward(), Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.3,
            first_cascade_far_bound: first,
            maximum_distance: far,
            overlap_proportion: 0.2,
        }
        .build(),
    ));
    commands.spawn((
        Hud,
        Text::new("worldgen view: reading..."),
        TextFont::from_font_size(15.0),
        TextColor(Color::srgb(0.97, 0.97, 0.9)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            padding: UiRect::all(Val::Px(6.0)),
            ..Node::default()
        },
    ));
}

/// The atmosphere's medium: Bevy's Earth terms (Rayleigh, Mie, ozone) with the Mie term replaced by Hillaire
/// 2020's aerosol coefficients times `aerosol`, falling off with its own scale height.
fn haze_medium(aerosol: f32, height_m: f32, shell_m: f32) -> ScatteringMedium {
    // Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020), per metre.
    const MIE_SCATTERING: f32 = 3.996e-6;
    const MIE_ABSORPTION: f32 = 0.444e-6;
    let mut medium = ScatteringMedium::earth(256, 256);
    if let Some(mie) = medium.terms.get_mut(1) {
        mie.scattering = Vec3::splat(MIE_SCATTERING * aerosol);
        mie.absorption = Vec3::splat(MIE_ABSORPTION * aerosol);
        mie.falloff = Falloff::Exponential { scale: height_m / shell_m };
    }
    medium.with_label("worldgen-view haze")
}

/// The camera's aerial perspective for `haze`.
fn apply_haze(camera: &mut EntityCommands, haze: Haze, settings: &WorldgenViewSettings) {
    match haze {
        Haze::Atmosphere => {
            camera.remove::<DistanceFog>().insert(AtmosphereSettings {
                aerial_view_lut_max_distance: settings.aerial_max_m,
                ..AtmosphereSettings::default()
            });
        }
        Haze::Distance => {
            camera.remove::<AtmosphereSettings>().insert(DistanceFog {
                color: FOG,
                directional_light_color: SUN_GLOW.with_alpha(0.5),
                directional_light_exponent: 24.0,
                falloff: FogFalloff::from_visibility(settings.fog_visibility_m),
            });
        }
        Haze::Off => {
            camera.remove::<(DistanceFog, AtmosphereSettings)>();
        }
    }
}

/// The contour uniforms for the settings, on or off.
fn contour_ext(settings: &WorldgenViewSettings, on: bool) -> ContourExt {
    let c = Contours { interval_m: settings.contour_m, width_px: settings.contour_px };
    let [r, g, b, _] = refs::srgb_linear(CONTOUR_INK);
    ContourExt {
        style: Vec4::new(c.interval_m[0], c.interval_m[1], c.width_px[0], c.width_px[1]),
        colour: Vec4::new(r, g, b, if on { 1.0 } else { 0.0 }),
    }
}

/// Takes the loader's messages: terrain, sea and markers when an arm is read, chunk and tree meshes (a few per
/// frame).
#[allow(clippy::too_many_lines, reason = "one match over the loader's messages")]
fn receive(
    mut commands: Commands,
    loader: Res<Loader>,
    mut view: ResMut<View>,
    mut cam: ResMut<Cam>,
    mut meshes: ResMut<Assets<Mesh>>,
    markers: Res<MarkerData>,
    settings: Res<WorldgenViewSettings>,
) {
    let Ok(rx) = loader.0.lock() else { return };
    let mut uploads = 0;
    while uploads < settings.uploads_per_frame.max(1) {
        let Ok(msg) = rx.try_recv() else { break };
        match msg {
            Msg::Read { arm, terrain, offset, origin_m, chunks, secs } => {
                info!(
                    "worldgen-view: {}: {}x{} @ {} m, sea {} m, peak {:.0} m, {} land chunks, read in {secs:.1} s",
                    view.arms[arm].name, terrain.w, terrain.h, terrain.spacing, terrain.sea_level, terrain.max_h, chunks.len()
                );
                let root = view.arms[arm].root.expect("arm roots spawn at startup");
                spawn_sea(&mut commands, &mut meshes, &view.sea_material, root, &terrain, offset);
                if !cam.placed && arm == 0 {
                    place_start(&mut cam, &terrain, offset, &settings);
                }
                let placed = markers.0.as_ref().map(|m| place_markers(m, &terrain, origin_m, &settings)).unwrap_or_default();
                let marker_root = (!placed.0.is_empty()).then(|| {
                    spawn_markers(&mut commands, &mut meshes, &view.props_material, root, offset, &placed, view.markers)
                });
                if let Some(m) = &markers.0 {
                    info!("worldgen-view: {}: {} of {} markers on the grid", view.arms[arm].name, placed.0.len(), m.markers.len());
                }
                let a = &mut view.arms[arm];
                a.chunks = chunks
                    .into_iter()
                    .map(|rect| ChunkView {
                        rect,
                        lods: [None; LOD_STRIDES.len()],
                        trees: None,
                        trees_shown: false,
                        min: Vec3::ZERO,
                        max: Vec3::ZERO,
                        shown: None,
                    })
                    .collect();
                a.terrain = Some(terrain);
                a.offset = offset;
                a.markers = placed.0;
                a.markers.extend(placed.1);
                a.marker_root = marker_root;
                a.state = LoadState::Building;
            }
            Msg::Chunk { arm, chunk, lod, mesh } => {
                uploads += 1;
                let a = &mut view.arms[arm];
                // Chunk meshes are local to their first sample.
                let spacing = a.terrain.as_ref().map_or(0.0, |t| t.spacing);
                let rect = a.chunks[chunk].rect;
                let origin = a.offset + Vec3::new(rect.c0 as f32 * spacing, 0.0, rect.r0 as f32 * spacing);
                let (min, max) = (Vec3::from(mesh.min), Vec3::from(mesh.max));
                let handle = meshes.add(to_mesh(mesh));
                let mut e = commands.spawn((
                    Mesh3d(handle),
                    Transform::from_translation(origin),
                    Aabb::from_min_max(min, max),
                    NoAutoAabb,
                    Visibility::Hidden,
                    ChildOf(a.root.expect("arm roots spawn at startup")),
                ));
                match &view.terrain_material {
                    TerrainMat::Contour(h) => e.insert(MeshMaterial3d(h.clone())),
                    TerrainMat::Plain(h) => e.insert(MeshMaterial3d(h.clone())),
                };
                let id = e.id();
                let a = &mut view.arms[arm];
                let c = &mut a.chunks[chunk];
                if c.lods.iter().all(Option::is_none) {
                    (c.min, c.max) = (origin + min, origin + max);
                }
                c.lods[lod] = Some(id);
                a.built += 1;
            }
            Msg::Trees { arm, chunk, mesh, count } => {
                let material = view.props_material.clone();
                let a = &mut view.arms[arm];
                a.trees_built += 1;
                a.tree_count += count;
                if count == 0 {
                    continue;
                }
                uploads += 1;
                let spacing = a.terrain.as_ref().map_or(0.0, |t| t.spacing);
                let rect = a.chunks[chunk].rect;
                let origin = a.offset + Vec3::new(rect.c0 as f32 * spacing, 0.0, rect.r0 as f32 * spacing);
                let (min, max) = (Vec3::from(mesh.min), Vec3::from(mesh.max));
                let id = commands
                    .spawn((
                        Mesh3d(meshes.add(to_mesh(mesh))),
                        MeshMaterial3d(material),
                        Transform::from_translation(origin),
                        Aabb::from_min_max(min, max),
                        NoAutoAabb,
                        Visibility::Hidden,
                        ChildOf(a.root.expect("arm roots spawn at startup")),
                    ))
                    .id();
                a.chunks[chunk].trees = Some(id);
            }
            Msg::Done { arm, secs } => {
                let a = &mut view.arms[arm];
                info!("worldgen-view: {}: {} chunk meshes, {} trees built in {secs:.1} s", a.name, a.built, a.tree_count);
                a.state = LoadState::Ready;
            }
            Msg::Failed { arm, error } => {
                error!("worldgen-view: {}: {error}", view.arms[arm].name);
                view.arms[arm].state = LoadState::Failed(error);
            }
        }
    }
}

/// The markers of `file` with at least `--marker-min-cells` cells on `t`, and the source summit.
fn place_markers(file: &MarkersFile, t: &Terrain, origin_m: [f64; 2], s: &WorldgenViewSettings) -> (Vec<Placed>, Option<Placed>) {
    let placed = file
        .markers
        .iter()
        .filter(|m| m.cells.len() >= s.marker_min_cells)
        .filter_map(|m| refs::place_marker(m, t, s.marker_scale, origin_m, file.cell_size_m, s.marker_height_m))
        .collect();
    let [pole_m, pole_w] = s.summit_pole_m;
    let summit = file.summit.as_ref().and_then(|top| refs::place_summit(top, t, s.marker_scale, origin_m, pole_w / 2.0, pole_m));
    (placed, summit)
}

/// The marker blocks and the summit pole of one arm, under one entity (M toggles it).
fn spawn_markers(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    root: Entity,
    offset: Vec3,
    (markers, summit): &(Vec<Placed>, Option<Placed>),
    on: bool,
) -> Entity {
    let visibility = if on { Visibility::Inherited } else { Visibility::Hidden };
    let holder = commands.spawn((Name::new("markers"), Transform::from_translation(offset), visibility, ChildOf(root))).id();
    let blocks: Vec<refs::Block> = markers.iter().flat_map(|m| m.blocks.iter().copied()).collect();
    let pole: Vec<refs::Block> = summit.iter().flat_map(|m| m.blocks.iter().copied()).collect();
    for (blocks, colour) in [(blocks, MARKER), (pole, SUMMIT_POLE)] {
        if blocks.is_empty() {
            continue;
        }
        let mesh = refs::blocks_mesh(&blocks, [0.0, 0.0], refs::srgb_linear(colour));
        commands.spawn((Mesh3d(meshes.add(to_mesh(mesh))), MeshMaterial3d(material.clone()), Transform::IDENTITY, ChildOf(holder)));
    }
    holder
}

/// The sea: one opaque plane at sea level under the arm's grid, reaching past the fog.
fn spawn_sea(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    root: Entity,
    t: &Terrain,
    offset: Vec3,
) {
    let size = Vec2::new((t.w - 1) as f32, (t.h - 1) as f32) * t.spacing;
    let centre = offset + Vec3::new(size.x / 2.0, t.sea_level, size.y / 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, size / 2.0 + Vec2::splat(SEA_MARGIN_M)))),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(centre),
        NotShadowCaster,
        ChildOf(root),
    ));
}

/// The start pose: `start_distance_m` south of the highest point, `start_height_m` above the surface, looking at it.
fn place_start(cam: &mut Cam, t: &Terrain, offset: Vec3, settings: &WorldgenViewSettings) {
    let (c, r, h) = t.peak();
    let peak = offset + Vec3::new(c as f32 * t.spacing, h, r as f32 * t.spacing);
    let mut pos = peak + Vec3::new(0.0, 0.0, settings.start_distance_m);
    let ground = t.height_at(pos.x - offset.x, pos.z - offset.z).unwrap_or(t.sea_level).max(t.sea_level);
    pos.y = ground + settings.start_height_m;
    cam.pos = pos;
    cam.look_at(peak);
    cam.placed = true;
}

fn to_mesh(m: ChunkMesh) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, m.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, m.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, m.colors)
        .with_inserted_indices(Indices::U32(m.indices))
}

/// Arm keys, walk toggle, HUD toggle, mouse capture, quit.
fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut view: ResMut<View>,
    mut cam: ResMut<Cam>,
    mut roots: Query<&mut Visibility>,
    mut exit: MessageWriter<AppExit>,
) {
    const DIGITS: [KeyCode; MAX_ARMS] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];
    for (i, key) in DIGITS.iter().enumerate().take(view.arms.len()) {
        if keys.just_pressed(*key) && i != view.current {
            view.current = i;
            for (k, arm) in view.arms.iter_mut().enumerate() {
                if let Some(mut v) = arm.root.and_then(|r| roots.get_mut(r).ok()) {
                    *v = if k == i { Visibility::Inherited } else { Visibility::Hidden };
                }
            }
        }
    }
    if keys.just_pressed(KeyCode::KeyG) {
        cam.walk = !cam.walk;
    }
    if keys.just_pressed(KeyCode::KeyH) {
        view.hud = !view.hud;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cam.grabbed = !cam.grabbed;
    }
    if buttons.just_pressed(MouseButton::Left) {
        cam.grabbed = true;
    }
    if keys.just_pressed(KeyCode::F10) {
        exit.write(AppExit::Success);
    }
}

/// Reference keys: M markers, T trees, C contours, F haze, P drops a figure, O clears the figures.
#[allow(clippy::too_many_arguments, reason = "a Bevy system")]
fn toggles(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut view: ResMut<View>,
    cam: Res<Cam>,
    settings: Res<WorldgenViewSettings>,
    mut terrain_materials: ResMut<Assets<TerrainMaterial>>,
    mut vis: Query<&mut Visibility>,
    camera: Query<Entity, With<Camera3d>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        view.markers = !view.markers;
        let want = if view.markers { Visibility::Inherited } else { Visibility::Hidden };
        for root in view.arms.iter().filter_map(|a| a.marker_root) {
            if let Ok(mut v) = vis.get_mut(root) {
                *v = want;
            }
        }
    }
    if keys.just_pressed(KeyCode::KeyT) {
        view.trees = !view.trees;
    }
    if keys.just_pressed(KeyCode::KeyC) {
        view.contours = !view.contours;
        if let TerrainMat::Contour(h) = &view.terrain_material
            && let Some(mut m) = terrain_materials.get_mut(h)
        {
            m.extension = contour_ext(&settings, view.contours);
        }
    }
    if keys.just_pressed(KeyCode::KeyF) {
        view.haze = view.haze.next();
        if let Ok(e) = camera.single() {
            apply_haze(&mut commands.entity(e), view.haze, &settings);
        }
    }
    if keys.just_pressed(KeyCode::KeyP) {
        let [height, ahead] = settings.figure_m;
        let forward = (Quat::from_rotation_y(cam.yaw) * Vec3::NEG_Z) * ahead;
        let mut at = cam.pos + forward;
        at.y = view.surface(at) + height / 2.0;
        if view.figures.len() >= MAX_FIGURES {
            let oldest = view.figures.remove(0);
            commands.entity(oldest).despawn();
        }
        let figure = commands
            .spawn((
                Name::new("figure"),
                Mesh3d(view.figure_mesh.clone()),
                MeshMaterial3d(view.figure_material.clone()),
                Transform::from_translation(at),
            ))
            .id();
        view.figures.push(figure);
    }
    if keys.just_pressed(KeyCode::KeyO) {
        for e in view.figures.drain(..) {
            commands.entity(e).despawn();
        }
    }
}

/// Windows clips a hidden `Locked` cursor oddly; `Confined` + hidden is right there (as `mw_player_bevy`).
const GRAB: CursorGrabMode = if cfg!(target_os = "windows") { CursorGrabMode::Confined } else { CursorGrabMode::Locked };

/// Mouse look, movement, the walk mode's ground snap, and the camera transform.
#[allow(clippy::too_many_arguments, reason = "a Bevy system")]
fn control(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mut windows: Query<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
    mut cam: ResMut<Cam>,
    view: Res<View>,
    settings: Res<WorldgenViewSettings>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let mut looking = false;
    if let Ok((window, mut cursor)) = windows.single_mut() {
        looking = cam.grabbed && window.focused;
        let (mode, visible) = if looking { (GRAB, false) } else { (CursorGrabMode::None, true) };
        if cursor.grab_mode != mode || cursor.visible != visible {
            cursor.grab_mode = mode;
            cursor.visible = visible;
        }
    }
    if looking {
        let d = motion.delta * settings.look_deg_per_px.to_radians();
        cam.yaw -= d.x;
        cam.pitch = (cam.pitch - d.y).clamp(-1.55, 1.55);
    }
    let notches = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 100.0,
    };
    if notches != 0.0 && !cam.walk {
        let [lo, hi] = settings.fly_speed_range;
        cam.speed = (cam.speed * settings.wheel_step.powf(notches)).clamp(lo, hi);
    }
    let axis = |pos: KeyCode, neg: KeyCode| f32::from(u8::from(keys.pressed(pos))) - f32::from(u8::from(keys.pressed(neg)));
    let (fwd, right) = (axis(KeyCode::KeyW, KeyCode::KeyS), axis(KeyCode::KeyD, KeyCode::KeyA));
    let up = f32::from(u8::from(keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::KeyE)))
        - f32::from(u8::from(keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::KeyQ)));
    let fast = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let dt = time.delta_secs();
    let rotation = cam.rotation();
    if cam.walk {
        let [walk, run] = settings.walk_run;
        let yaw = Quat::from_rotation_y(cam.yaw);
        let step = (yaw * Vec3::NEG_Z * fwd + yaw * Vec3::X * right).normalize_or_zero();
        cam.pos += step * if fast { run } else { walk } * dt;
        cam.pos.y = view.surface(cam.pos) + settings.eye_height_m;
    } else {
        let step = (rotation * Vec3::NEG_Z * fwd + rotation * Vec3::X * right + Vec3::Y * up).normalize_or_zero();
        let speed = if fast { cam.speed * settings.fast } else { cam.speed };
        cam.pos += step * speed * dt;
        let floor = view.surface(cam.pos) + settings.eye_height_m;
        cam.pos.y = cam.pos.y.max(floor);
    }
    if let Ok(mut tf) = camera.single_mut() {
        *tf = Transform::from_translation(cam.pos).with_rotation(rotation);
    }
}

/// Shows one LOD per chunk of the current arm, by camera distance to its bounds, and its trees when near.
fn pick_lods(mut view: ResMut<View>, cam: Res<Cam>, settings: Res<WorldgenViewSettings>, mut vis: Query<&mut Visibility>) {
    let current = view.current;
    let trees_on = view.trees;
    for c in &mut view.arms[current].chunks {
        let available = c.lods.map(|e| e.is_some());
        let d = (cam.pos.clamp(c.min, c.max) - cam.pos).length();
        if let Some(t) = c.trees {
            let want = trees_on && d <= settings.tree_distance_m;
            if want != c.trees_shown
                && let Ok(mut v) = vis.get_mut(t)
            {
                *v = if want { Visibility::Inherited } else { Visibility::Hidden };
                c.trees_shown = want;
            }
        }
        let Some(level) = pick_lod(d, c.shown, settings.lod_m, available) else { continue };
        if c.shown == Some(level) {
            continue;
        }
        for (l, e) in c.lods.iter().enumerate() {
            if let Some(mut v) = e.and_then(|e| vis.get_mut(e).ok()) {
                *v = if l == level { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
        c.shown = Some(level);
    }
}

/// The LOD to show at distance `d`: by `lod_m` (with [`LOD_HYSTERESIS`] around the current one), then the
/// nearest built one (coarser first: they arrive first). `None` when none is built.
pub fn pick_lod(d: f32, shown: Option<usize>, lod_m: [f32; 2], built: [bool; 3]) -> Option<usize> {
    let mut want = lod_m.iter().take_while(|&&m| d >= m).count();
    if let Some(s) = shown {
        let lo = if s == 0 { 0.0 } else { lod_m[s - 1] * (1.0 - LOD_HYSTERESIS) };
        let hi = if s >= lod_m.len() { f32::INFINITY } else { lod_m[s] * (1.0 + LOD_HYSTERESIS) };
        if built[s] && d >= lo && d < hi {
            want = s;
        }
    }
    (want..built.len()).chain((0..want).rev()).find(|&l| built[l])
}

/// Sun keys: [ ] azimuth, , . elevation.
fn sun(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<WorldgenViewSettings>,
    mut sun: ResMut<Sun>,
    mut light: Query<&mut Transform, With<DirectionalLight>>,
) {
    let rate = settings.sun_rate_deg_s * time.delta_secs();
    let axis = |pos: KeyCode, neg: KeyCode| f32::from(u8::from(keys.pressed(pos))) - f32::from(u8::from(keys.pressed(neg)));
    let (daz, del) = (axis(KeyCode::BracketRight, KeyCode::BracketLeft), axis(KeyCode::Period, KeyCode::Comma));
    if daz == 0.0 && del == 0.0 {
        return;
    }
    sun.azimuth_deg = (sun.azimuth_deg + daz * rate).rem_euclid(360.0);
    sun.elevation_deg = (sun.elevation_deg + del * rate).clamp(0.5, 89.5);
    if let Ok(mut tf) = light.single_mut() {
        *tf = Transform::IDENTITY.looking_to(-sun.toward(), Vec3::Y);
    }
}

/// The overlay: arm, keys, heights, speed, sun, FPS, references, nearest marker, loading progress.
fn hud(
    time: Res<Time>,
    view: Res<View>,
    cam: Res<Cam>,
    sun: Res<Sun>,
    mut fps: Local<f32>,
    mut text: Query<(&mut Text, &mut Visibility), With<Hud>>,
) {
    let dt = time.delta_secs();
    if dt > 0.0 {
        *fps = if *fps == 0.0 { 1.0 / dt } else { *fps * 0.95 + 0.05 / dt };
    }
    let Ok((mut text, mut visibility)) = text.single_mut() else { return };
    let want = if view.hud { Visibility::Inherited } else { Visibility::Hidden };
    if *visibility != want {
        *visibility = want;
    }
    if !view.hud {
        return;
    }
    let arm = &view.arms[view.current];
    let ground = view.ground(cam.pos).map_or_else(|| "off the grid".to_owned(), |g| format!("{:.0} m above ground", cam.pos.y - g));
    let mode = if cam.walk { "walk".to_owned() } else { format!("fly {:.0} m/s", cam.speed) };
    let mut s = format!(
        "[{}] {}   {ground}, {:.0} m above sea   {mode}   sun az {:.0} el {:.0}   {:.0} fps ({:.1} ms)\n",
        view.current + 1,
        arm.name,
        cam.pos.y - view.sea_level(),
        sun.azimuth_deg,
        sun.elevation_deg,
        *fps,
        1000.0 / fps.max(1.0),
    );
    for (i, a) in view.arms.iter().enumerate() {
        let total = a.chunks.len() * LOD_STRIDES.len();
        let state = match &a.state {
            LoadState::Reading => "reading".to_owned(),
            LoadState::Building if a.built < total => format!("meshes {}/{total}", a.built),
            LoadState::Building => format!("trees {}/{}", a.trees_built, a.chunks.len()),
            LoadState::Ready => "ready".to_owned(),
            LoadState::Failed(e) => format!("failed: {e}"),
        };
        let mark = if i == view.current { ">" } else { " " };
        let _ = write!(s, "{mark}{} {}: {state}   ", i + 1, a.name);
    }
    let on = |b: bool| if b { "on" } else { "off" };
    let local = cam.pos - arm.offset;
    let nearest = refs::nearest(&arm.markers, (local.x, local.z)).map_or_else(
        || "no marker".to_owned(),
        |(i, d)| {
            let m = &arm.markers[i];
            format!("nearest: {} {:.2} km {}", m.name, d / 1000.0, refs::compass(m.at.0 - local.x, m.at.1 - local.z))
        },
    );
    let _ = write!(
        s,
        "\nM markers {} ({})   T trees {} ({})   C contours {}   F haze: {}   P figure ({}), O clear   {nearest}",
        on(view.markers),
        arm.markers.len(),
        on(view.trees),
        arm.tree_count,
        on(view.contours),
        view.haze.label(),
        view.figures.len(),
    );
    s.push_str(
        "\nWASD move, mouse look (Esc frees the mouse, click takes it), Space/E up, Ctrl/Q down, Shift fast, wheel speed, \
         G walk, 1-9 arm, [ ] sun azimuth, , . sun elevation, H hide, F10 quit",
    );
    if text.0 != s {
        text.0 = s;
    }
}

/// Marker labels: one screen-space text per marker of the current arm within `--label-distance-m`, in front of the
/// camera (name and distance). Hidden with the markers (M) and the HUD (H).
fn labels(
    mut commands: Commands,
    view: Res<View>,
    cam: Res<Cam>,
    settings: Res<WorldgenViewSettings>,
    cameras: Query<&Camera, With<Camera3d>>,
    mut pool: ResMut<Labels>,
    mut nodes: Query<(&mut Text, &mut Node, &mut Visibility), With<Label>>,
) {
    let arm = &view.arms[view.current];
    while pool.0.len() < arm.markers.len() {
        let id = commands
            .spawn((
                Label,
                Text::new(""),
                TextFont::from_font_size(14.0),
                TextColor(Color::srgb(1.0, 0.97, 0.88)),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
                Node { position_type: PositionType::Absolute, padding: UiRect::axes(Val::Px(3.0), Val::Px(1.0)), ..Node::default() },
                Visibility::Hidden,
            ))
            .id();
        pool.0.push(id);
    }
    let Ok(camera) = cameras.single() else { return };
    // This frame's camera pose (the GlobalTransform propagates later).
    let eye = GlobalTransform::from(Transform::from_translation(cam.pos).with_rotation(cam.rotation()));
    for (i, &e) in pool.0.iter().enumerate() {
        let Ok((mut text, mut node, mut visibility)) = nodes.get_mut(e) else { continue };
        let shown = (view.hud && view.markers).then(|| arm.markers.get(i)).flatten().and_then(|m| {
            let anchor = arm.offset + Vec3::new(m.at.0, m.top + settings.label_lift_m, m.at.1);
            let d = Vec2::new(anchor.x - cam.pos.x, anchor.z - cam.pos.z).length();
            if d > settings.label_distance_m {
                return None;
            }
            let at = camera.world_to_viewport(&eye, anchor).ok()?;
            Some((format!("{} {:.1} km", m.name, d / 1000.0), at))
        });
        let want = if shown.is_some() { Visibility::Inherited } else { Visibility::Hidden };
        if *visibility != want {
            *visibility = want;
        }
        if let Some((label, at)) = shown {
            if text.0 != label {
                text.0 = label;
            }
            let (left, top) = (Val::Px(at.x), Val::Px(at.y));
            if node.left != left || node.top != top {
                node.left = left;
                node.top = top;
            }
        }
    }
}
