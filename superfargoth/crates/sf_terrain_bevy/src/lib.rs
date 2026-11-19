//! Bevy plugin for `sf_terrain`. Spec: `docs/specs/terrain.md`.
//!
//! - **Residency**: canonical tiles stream from the bake store
//!   ([`TerrainWorld`]) on the async compute pool, around the
//!   [`TerrainCamera`], and also feed the CPU [`TerrainHeightsMap`].
//! - **Clipmaps**: 10 × 512² `R32Float` heights (0.5 m … 256 m), 3 levels of
//!   splat layers (1–4 m) and 10 levels of macro albedo, updated
//!   toroidally in strips under a per-frame upload budget
//!   ([`TerrainSettings::upload_budget`]).
//! - **Geometry**: CDLOD ([`sf_terrain::cdlod`]) selects patches each
//!   frame; each patch is an entity sharing one grid mesh and one
//!   [`TerrainMaterial`], so Bevy batches them into indirect draws, culls
//!   them per view (main and shadow cascades) and draws them in the depth
//!   prepass. The vertex shader morphs and displaces.
//! - **Origin**: patch transforms are relative to [`TerrainOrigin`], the
//!   floating origin; hosts on `big_space` implement [`RenderOrigin`] for
//!   their origin resource and add [`TerrainOriginPlugin`].
//! - **GPU fill** (worldgen G2, `gpu_fill`): a provider with a GPU field
//!   ([`sf_terrain::GpuField`]) fills the height, normal and macro clipmaps
//!   on the GPU; no tile is needed to draw (CDLOD bounds are analytic,
//!   `TerrainTiles::block_bounds`) and CPU tiles are generated only for
//!   height queries near the camera and at coarse levels (the CPU twin,
//!   [`TwinSettings`]). [`TerrainSettings::gpu_fill`] off: the tile path.

mod far_map;
mod gpu_fill;
mod material;
mod params;
mod render;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_image::{CompressedImageFormatSupport, CompressedImageFormats, Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_math::{DVec2, Vec3};
use bevy_mesh::{Indices, Mesh, Mesh3d, MeshTag, PrimitiveTopology};
use bevy_pbr::{MaterialPlugin, MeshMaterial3d};
use bevy_render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use bevy_render::renderer::{RenderGraph, RenderGraphSystems};
use bevy_render::storage::ShaderBuffer;
use bevy_render::{ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shader::{Shader, load_shader_library};
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use bevy_transform::TransformSystems;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_coords::WorldPos;
use sf_terrain::LayerCap;
use sf_terrain::TerrainTiles;
use sf_terrain::cdlod::{self, CdlodSettings, MIN_LOD, NodeKey, SelectedNode};
use sf_terrain::clipmap::{self, CLIPMAP_SIZE, ClipWindow, SampleRect};
use sf_terrain::gpu_fill::{FillKind, FillRequest};
use sf_terrain::{
    BLOCKS, BlockBounds, LayerId, LayerMap, LayerTable, ResidentTile, TerrainHeights, TileCoord, global_coords,
};
use sf_tunables::{AnyDef, Def, Group, Source, Unit, def};
use sf_world::{GateQuanta, GateSource, SelectGate, WorldspaceId};

pub use far_map::{FAR_MAP_SIZE, FAR_MAP_TEXEL, FarShadowMap};
pub use gpu_fill::GpuFillCounters;
pub use material::TerrainMaterial;
use params::Params;
pub use params::{HEIGHT_LEVELS, LAYER_LEVELS, MACRO_LEVELS};
pub use render::{ClipmapWrites, TextureWrite};
pub use sf_terrain;

/// Default ground reflectance (Bevy scale, F0 = 0.16 x r²): 0.3, F0 0.0144, as the usual
/// default for legacy assets without PBR maps. Lower than a measured dielectric's 0.5 (F0 0.04)
/// because the ground has no normal or cavity maps for small-scale occlusion (UE lowers
/// Specular for that); Bevy's F0-based specular occlusion `saturate(16.5 x F0)` (0.24 here,
/// 0.66 at 0.5) then also damps the ambient and env-map lobes' grazing B term. Rough ground
/// (0.85-0.9) at N.V 0.1-0.2 under a flat ambient: the albedo-independent specular film drops
/// from ~25% of the pixel at 0.5 to ~7% at albedo 0.15 (12% at 0.08); RO-16 measurement test.
pub const DEFAULT_REFLECTANCE: f32 = 0.3;

/// Default memory budget of the terrain layer arrays, bytes: 1 GiB, an eighth of the 8 GB frame
/// target GPU (terrain.md "Layer arrays"). A world with many layers keeps a smaller level 0
/// rather than all of them at full size (until per-layer residency, textures.md S3).
pub const DEFAULT_LAYER_BUDGET: usize = 1 << 30;

/// Tunables. Defaults follow terrain.md and the streaming budgets.
#[derive(Resource, Clone, Debug)]
pub struct TerrainSettings {
    pub cdlod: CdlodSettings,
    /// GPU clipmap bytes per frame (terrain.md: ≤ 2 MiB/frame at 60 m/s).
    pub upload_budget: usize,
    /// Budget while nothing is resident yet (startup, teleport), as
    /// streaming's burst mode.
    pub burst_budget: usize,
    /// Tile loads in flight.
    pub max_loads: usize,
    /// Clipmap windows move in steps of this many samples.
    pub snap: i64,
    /// Distance where splatting hands over to the macro albedo, meters.
    pub splat_range: f32,
    /// Height-blend transition depth (Mishkinis), in height units.
    pub blend_depth: f32,
    /// Layer arrays to load (`None`: the largest baked).
    pub layer_array_size: Option<u32>,
    /// Top levels of the layer arrays left out at load (T6): the preset's texture cap
    /// (`max_edge`) and a memory budget for the three arrays. Default: no cap,
    /// [`DEFAULT_LAYER_BUDGET`].
    pub layer_cap: LayerCap,
    /// The ground's dielectric reflectance, Bevy's `StandardMaterial::reflectance` scale
    /// (F0 = 0.16 x r², so 0.5 is F0 0.04), applied before surface weather mixes wet areas
    /// toward water. Default [`DEFAULT_REFLECTANCE`]; games may override it, live too.
    pub reflectance: f32,
    /// How high another system raises the near terrain above its ground, meters (W6b: snow cover
    /// drawn as geometry by `sf_surface_bevy` through the patch "SW" bindings). The two finest
    /// CDLOD levels' bounds grow upward by it so culling stays conservative. Default 0.
    pub raise_m: f32,
    /// The raised cover's steepest slope, degrees (SW-6, [`TERRAIN_RAISE_REPOSE_DEG`]): the raise's
    /// tops are low-passed wide enough (`sf_surface::layer::raise_filter_m`) that the cover never
    /// stands steeper whatever the tops. Shader slot `P_RAISE`.
    pub raise_repose_deg: f32,
    /// The raise's narrowest filter width, meters (SW-6, [`TERRAIN_RAISE_FILTER_MIN_M`]).
    pub raise_filter_min_m: f32,
    /// How far from the camera the raised cover may stand (SW-5e), meters: the host sets it to where
    /// the cover's inputs stop being known whatever the camera's place (surface weather's shelter
    /// levels and the casters that fill them, `sf_surface::view::raise_reach_m`), so no texel's raise
    /// changes as the camera nears. The fade ends there or at its own sub-pixel end, whichever is
    /// nearer (`CdlodSettings::raise_fade_range`). Infinite (default): no limit.
    pub raise_reach_m: f32,
    /// CW-2 selection gate (`sf_world::select_gate`): CDLOD selection and the patch diff re-run only
    /// once the camera moved [`TERRAIN_SELECT_MOVE_M`] (3D) from where the last selection saw it, or a
    /// tile arrived or left, an edit, an origin shift, or a settings or debug change; the shader's morph
    /// zones move that much closer to stay crack-free ([`params::morph_zone`]). Selection is by distance
    /// only (no frustum, `sf_terrain::cdlod`), so turning never re-runs it. Kept per frame: load polls,
    /// requests and eviction, clipmap uploads and the shader parameters (camera, morph). Off by default
    /// until the owner's A/B.
    pub select_gate: GateQuanta,
    /// Fill the clipmaps on the GPU when the provider has a GPU field (`gpu_fill`, worldgen G2); off, every
    /// provider goes through CPU tiles (the G1 path, an A/B). Default on.
    pub gpu_fill: bool,
    /// Which CPU tiles a GPU-filled world generates for height queries.
    pub twin: TwinSettings,
}

/// The CPU twin of a GPU-filled world (`gpu_fill`): tiles generated only for height queries (physics, camera,
/// AI, the far shadow), the finest levels near the camera and the coarse levels' whole windows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwinSettings {
    /// Levels 0 .. this many get the tiles around the camera's ([`TERRAIN_TWIN_NEAR_LEVELS`]).
    pub near_levels: u8,
    /// ... this many tiles either side of the camera's ([`TERRAIN_TWIN_NEAR_RADIUS`]).
    pub near_radius: i32,
    /// Levels from this one up get every tile their height window covers ([`TERRAIN_TWIN_COARSE_LEVEL`]).
    pub coarse_level: u8,
    /// Twin tile loads in flight ([`TERRAIN_TWIN_LOADS`]).
    pub max_loads: usize,
}

impl Default for TwinSettings {
    fn default() -> Self {
        Self {
            near_levels: u8::try_from(TERRAIN_TWIN_NEAR_LEVELS.default_value()).unwrap_or(2),
            near_radius: i32::try_from(TERRAIN_TWIN_NEAR_RADIUS.default_value()).unwrap_or(1),
            coarse_level: u8::try_from(TERRAIN_TWIN_COARSE_LEVEL.default_value()).unwrap_or(5),
            max_loads: usize::try_from(TERRAIN_TWIN_LOADS.default_value()).unwrap_or(1),
        }
    }
}

/// `terrain.twin.near_levels`: [`TwinSettings::near_levels`]. Two levels (0.5 m and 1 m) carry the full detail
/// a walking or riding player and the physics around them touch; past them queries read the coarse levels.
pub const TERRAIN_TWIN_NEAR_LEVELS: Def<u32> = def!(count "terrain.twin.near_levels" = 2)
    .range(0, 10)
    .unit(Unit::Count)
    .help("GPU-filled terrain: CPU query tiles of the finest this many levels are generated around the camera")
    .source(Source::Guess);

/// `terrain.twin.near_radius`: [`TwinSettings::near_radius`]: 1 = the 3 x 3 tiles around the camera's (level 0:
/// at least 256 m of full detail around the player, level 1 512 m).
pub const TERRAIN_TWIN_NEAR_RADIUS: Def<u32> = def!(count "terrain.twin.near_radius" = 1)
    .range(0, 4)
    .unit(Unit::Count)
    .help("GPU-filled terrain: CPU query tiles this many tiles either side of the camera's, on the near levels")
    .source(Source::Guess);

/// `terrain.twin.coarse_level`: [`TwinSettings::coarse_level`]. Level 5 (16 m samples) keeps the octaves of
/// 32 m and up, so far queries are off by at most the finer octaves' share of the detail (a few metres).
pub const TERRAIN_TWIN_COARSE_LEVEL: Def<u32> = def!(count "terrain.twin.coarse_level" = 5)
    .range(0, 9)
    .unit(Unit::Count)
    .help("GPU-filled terrain: CPU query tiles of every level from this one up cover their whole clipmap window")
    .source(Source::Guess);

/// `terrain.twin.max_loads`: [`TwinSettings::max_loads`]. One at a time: each runs on the provider's own small
/// pool (`sf_worldgen` `twin_threads`), so the game's threads keep their cores.
pub const TERRAIN_TWIN_LOADS: Def<u32> = def!(count "terrain.twin.max_loads" = 1)
    .range(1, 16)
    .unit(Unit::Count)
    .help("GPU-filled terrain: CPU query tiles generated at once")
    .source(Source::Guess);

/// `terrain.select.move_m`: [`TerrainSettings::select_gate`] location quantum, also the morph slack.
/// Nobody measured it: 0.5 m is ~4 % of LOD -2's 12 m range and ~8 frames of a 4 m/s walk at 60 fps.
pub const TERRAIN_SELECT_MOVE_M: Def<f32> = def!(float "terrain.select.move_m" = 0.5)
    .range(0.0, 4.0)
    .unit(Unit::Meters)
    .help("Terrain re-selects its CDLOD patches once the camera moved this far; the morph zones move this much closer")
    .source(Source::Guess)
    .group(Group::ViewDistance);

/// `terrain.raise.repose_deg`: [`TerrainSettings::raise_repose_deg`]. Settled snow's angle of repose
/// is 30-35 deg (DEM studies), fresh powder ~25 (`superfargoth/docs/research/deep-snow-2026-11-16.md`
/// section 4 S2); 30 is the conservative end for the accumulated cover. Steeper reads sharper edges
/// (a narrower filter), flatter softer ones.
pub const TERRAIN_RAISE_REPOSE_DEG: Def<f32> = def!(float "terrain.raise.repose_deg" = 30.0)
    .range(10.0, 60.0)
    .unit(Unit::Degrees)
    .help("Raised snow and ash never stand steeper than this at their edges (the tops are low-passed to fit)")
    .source(Source::Industry("snow angle of repose 30-35 deg, DEM (research deep-snow-2026-11-16 S2)"));

/// `terrain.raise.filter_min_m`: [`TerrainSettings::raise_filter_min_m`], the floor under the repose
/// bound's width (which is ~4.3 m for 0.45 m of snow at 30 deg); 0.5 m was SW-5c's fixed prefilter.
pub const TERRAIN_RAISE_FILTER_MIN_M: Def<f32> = def!(float "terrain.raise.filter_min_m" = 0.5)
    .range(0.25, 64.0)
    .unit(Unit::Meters)
    .help("The raised cover's tops are low-passed at least this wide (the repose angle may need wider)")
    .source(Source::Guess);

/// This crate's tunables (live binding: tunables slice T2).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &TERRAIN_SELECT_MOVE_M,
    &TERRAIN_RAISE_REPOSE_DEG,
    &TERRAIN_RAISE_FILTER_MIN_M,
    &TERRAIN_TWIN_NEAR_LEVELS,
    &TERRAIN_TWIN_NEAR_RADIUS,
    &TERRAIN_TWIN_COARSE_LEVEL,
    &TERRAIN_TWIN_LOADS,
];

/// The `P_RAISE` slot (SW-6, SW-5e): tan of the repose angle, the filter's minimum width (m), the
/// raise fade's start and end (m; `CdlodSettings::raise_fade_range` at the reach limit).
fn raise_params(settings: &TerrainSettings) -> [f32; 4] {
    let (start, end) = settings.cdlod.raise_fade_range(settings.raise_reach_m);
    [settings.raise_repose_deg.to_radians().tan(), settings.raise_filter_min_m, start, end]
}

impl TerrainSettings {
    /// The terrain gate with its declared quantum, on or off.
    pub fn select_gate_quanta(enabled: bool) -> GateQuanta {
        GateQuanta { enabled, move_m: TERRAIN_SELECT_MOVE_M.default_value(), turn_deg: 0.0 }
    }

    /// The shader's morph slack: the gate's quantum while it is on.
    fn morph_slack(&self) -> f32 {
        if self.select_gate.enabled { self.select_gate.move_m } else { 0.0 }
    }
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            cdlod: CdlodSettings::default(),
            upload_budget: 2 << 20,
            burst_budget: 64 << 20,
            max_loads: 16,
            snap: 16,
            splat_range: 1000.0,
            blend_depth: 0.2,
            layer_array_size: None,
            layer_cap: LayerCap { max_edge: None, budget: Some(DEFAULT_LAYER_BUDGET) },
            reflectance: DEFAULT_REFLECTANCE,
            raise_m: 0.0,
            raise_repose_deg: TERRAIN_RAISE_REPOSE_DEG.default_value(),
            raise_filter_min_m: TERRAIN_RAISE_FILTER_MIN_M.default_value(),
            raise_reach_m: f32::INFINITY,
            select_gate: Self::select_gate_quanta(sf_world::SELECT_GATE.default_value()),
            gpu_fill: true,
            twin: TwinSettings::default(),
        }
    }
}

/// Debug views (terrain.md "Verification").
#[derive(Resource, Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct TerrainDebug {
    /// Color patches by LOD (fractional: morphing).
    pub lod_colors: bool,
    /// Overlay the triangle edges.
    pub wireframe: bool,
    /// Stop updating the selection (inspect it from elsewhere).
    pub freeze: bool,
    /// Spawn patches without frustum culling (to tell culling bugs from
    /// selection bugs).
    pub no_culling: bool,
    /// Turn off the terrain shadow past the shadow cascades (A/B; see
    /// `shaders/far_shadow.wesl`).
    pub no_far_shadows: bool,
    /// RO-16 veil diagnosis (scaffold: remove these three with the veil fix): no specular on the
    /// terrain (reflectance 0).
    pub no_specular: bool,
    /// RO-16: no indirect light on the terrain (env map, flat ambient, GI).
    pub no_indirect: bool,
    /// RO-16: the raw splat layer albedo (no macro tint or variation).
    pub raw_layers: bool,
    /// RO-16: no env-map specular on the terrain alone (diffuse and the other lobes stay).
    pub no_env_specular: bool,
}

/// The view that drives selection, morphing and residency. One per app.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TerrainCamera;

/// The worldspace whose terrain is shown: its tiles from the bake store
/// (`sf_terrain::bake::BakedTerrain`) or any other [`TerrainTiles`] provider.
#[derive(Resource, Clone)]
pub struct TerrainWorld {
    pub tiles: Arc<dyn TerrainTiles>,
}

impl TerrainWorld {
    pub fn new(tiles: Arc<dyn TerrainTiles>) -> Self {
        Self { tiles }
    }
}

/// The world position of render-space (0, 0, 0): the floating origin.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TerrainOrigin(pub WorldPos);

/// The floating-origin glue: implement for the resource that holds the
/// origin (e.g. `sf_world_bevy`'s `WorldOrigin`: `WorldPos::new(cell, 0)`)
/// and add [`TerrainOriginPlugin`]. Without it, [`TerrainOrigin`] is a
/// fixed local origin the app may set.
pub trait RenderOrigin: Resource {
    fn render_origin(&self) -> WorldPos;
}

/// Copies an origin resource into [`TerrainOrigin`] every frame.
pub struct TerrainOriginPlugin<O: RenderOrigin>(std::marker::PhantomData<O>);

impl<O: RenderOrigin> Default for TerrainOriginPlugin<O> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<O: RenderOrigin> Plugin for TerrainOriginPlugin<O> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_origin::<O>.before(TerrainSystems::Update));
    }
}

fn sync_origin<O: RenderOrigin>(source: Option<Res<O>>, mut origin: ResMut<TerrainOrigin>) {
    if let Some(source) = source {
        let o = source.render_origin();
        if origin.0 != o {
            origin.0 = o;
        }
    }
}

/// CPU height queries per worldspace (base + edits), filled from the tiles
/// the renderer streams. Physics, AI and foliage read these.
#[derive(Resource, Default)]
pub struct TerrainHeightsMap(pub HashMap<WorldspaceId, TerrainHeights>);

/// Counters for `sf_diag` (the app forwards them).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainStats {
    pub patches: usize,
    pub resident_tiles: usize,
    pub loads_in_flight: usize,
    /// Clipmap bytes uploaded this frame.
    pub upload_bytes: usize,
    /// CDLOD selection + entity diff, milliseconds.
    pub select_ms: f32,
    /// Whole terrain update, milliseconds.
    pub update_ms: f32,
    /// Height levels resident on the GPU.
    pub height_levels: u32,
    /// The clipmaps fill on the GPU (`gpu_fill`); then the tiles above are the CPU twin's.
    pub gpu_fill: bool,
    /// GPU fill so far: pieces and texels dispatched, frames that dispatched (cumulative).
    pub gpu_pieces: u64,
    pub gpu_texels: u64,
    pub gpu_frames: u64,
}

/// Fire after editing a worldspace's `HeightEdits`: GPU clipmap levels
/// covering the rect are rewritten.
#[derive(Message, Clone, Copy, Debug)]
pub struct TerrainEdited(pub sf_terrain::TerrainChanged);

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainSystems {
    Update,
}

/// Marks patch entities.
#[derive(Component, Clone, Copy, Debug)]
pub struct TerrainPatch(pub NodeKey);

#[derive(Default)]
pub struct TerrainPlugin {
    pub settings: TerrainSettings,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shaders/common.wesl");
        load_shader_library!(app, "shaders/far_shadow.wesl");
        load_shader_library!(app, "shaders/surface.wesl");
        embedded_asset!(app, "shaders/terrain.wesl");
        embedded_asset!(app, "shaders/terrain_prepass.wesl");

        app.add_plugins(MaterialPlugin::<TerrainMaterial>::default())
            .insert_resource(self.settings.clone())
            .init_resource::<TerrainDebug>()
            .init_resource::<TerrainOrigin>()
            .init_resource::<TerrainHeightsMap>()
            .init_resource::<TerrainStats>()
            .init_resource::<ClipmapWrites>()
            .init_resource::<gpu_fill::GpuFillQueue>()
            .add_message::<TerrainEdited>()
            .configure_sets(
                PostUpdate,
                TerrainSystems::Update
                    .after(TransformSystems::Propagate)
                    .before(VisibilitySystems::VisibilityPropagate)
                    .before(VisibilitySystems::CalculateBounds),
            )
            .add_systems(bevy_app::Startup, far_map::setup_far_map)
            .add_systems(
                PostUpdate,
                // `update_far_map` runs before `update_terrain` so `FarShadowMap::params`
                // tracks any `TerrainOrigin` shift before `update_terrain` writes `P_FAR` (SH-11).
                (setup_terrain, far_map::update_far_map, update_terrain).chain().in_set(TerrainSystems::Update),
            );

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<render::PendingWrites>()
                .init_resource::<gpu_fill::GpuFillRender>()
                .add_systems(ExtractSchedule, (render::extract_writes, gpu_fill::extract))
                .add_systems(
                    Render,
                    (render::write_clipmaps, gpu_fill::prepare).in_set(RenderSystems::PrepareResources),
                )
                // Before any view renders (the camera driver runs in `Render`).
                .add_systems(RenderGraph, gpu_fill::run.in_set(RenderGraphSystems::Begin));
        }
    }
}

/// A GPU-filled world's state (`gpu_fill`): analytic bounds per tile instead of resident tiles, and whether the
/// render world can fill yet.
struct GpuMode {
    bounds: HashMap<TileCoord, Arc<BlockBounds>>,
    ready: Arc<AtomicBool>,
    counters: Arc<GpuFillCounters>,
    /// The near layer levels' layer maps (`TerrainTiles::layer_map`), when the provider has them
    /// (`TerrainTiles::has_layer_maps`): heights stay on the GPU, layers still come from the CPU.
    layers: Option<HashMap<TileCoord, LayerSlot>>,
}

/// A GPU-filled world's layer map: loading, resident, or none for that tile (the fill slot).
enum LayerSlot {
    Loading(Task<Option<LayerMap>>),
    Ready(Arc<LayerMap>),
    Absent,
}

/// A resident tile.
struct Loaded {
    resident: Arc<ResidentTile>,
    layers: Option<LayerMap>,
    macro_rgba: Option<Vec<[u8; 4]>>,
}

enum Slot {
    Loading(Task<Option<Loaded>>),
    Ready(Arc<Loaded>),
    Absent,
}

/// GPU-side state of the active terrain.
#[derive(Resource)]
struct TerrainState {
    ws: WorldspaceId,
    /// Where tiles come from (the bake store or a generator).
    src: Arc<dyn TerrainTiles>,
    material: Handle<TerrainMaterial>,
    params: Handle<ShaderBuffer>,
    heights: Handle<Image>,
    normals: Handle<Image>,
    layer_ids: Handle<Image>,
    layer_weights: Handle<Image>,
    macro_image: Handle<Image>,
    patch: Handle<Mesh>,
    quadrant: Handle<Mesh>,
    table: LayerTable,
    slots: HashMap<LayerId, u8>,
    tiles: HashMap<TileCoord, Slot>,
    height_windows: [Option<ClipWindow>; HEIGHT_LEVELS],
    layer_windows: [Option<ClipWindow>; LAYER_LEVELS],
    macro_windows: [Option<ClipWindow>; MACRO_LEVELS],
    /// Patch entities and the height bounds their `Aabb` was built from.
    nodes: HashMap<NodeKey, (Entity, (f32, f32))>,
    origin: Option<WorldPos>,
    /// Frames left in which clipmap uploads wait after an origin shift.
    shift_settle: u8,
    fill: f32,
    fill_slot: u8,
    fill_rgba: [u8; 4],
    /// Highest height of any tile loaded so far (world meters): the far
    /// shadow's march stops once its ray is above it.
    max_height: f32,
    /// Bumped by everything besides the camera that changes the CDLOD selection or its patches: a
    /// tile's slot changed (loaded, absent, evicted), an edit, a settings or debug change.
    select_rev: u64,
    /// The GPU fill of a provider with a GPU field (`None`: the tile path).
    gpu: Option<GpuMode>,
    /// The selection gate (CW-2), keyed on (`select_rev`, origin).
    gate: SelectGate<(u64, WorldPos)>,
}

/// The 33² (or 17²) grid patch: integer grid coordinates in POSITION,
/// quads split along the canonical diagonal, counter-clockwise from +Y.
fn patch_mesh(quads: u32) -> Mesh {
    let n = quads + 1;
    let mut positions = Vec::with_capacity((n * n) as usize);
    for j in 0..n {
        for i in 0..n {
            #[allow(clippy::cast_precision_loss)]
            positions.push([i as f32, 0.0, j as f32]);
        }
    }
    let v = |i: u32, j: u32| u16::try_from(j * n + i).expect("small patch");
    let mut indices = Vec::with_capacity((quads * quads * 6) as usize);
    for j in 0..quads {
        for i in 0..quads {
            indices.extend_from_slice(&[v(i, j), v(i + 1, j + 1), v(i + 1, j)]);
            indices.extend_from_slice(&[v(i, j), v(i, j + 1), v(i + 1, j + 1)]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(Indices::U16(indices))
}

fn array_view(image: &mut Image) {
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..TextureViewDescriptor::default()
    });
}

/// An uninitialized clipmap array, written in strips from the render world.
fn clipmap_image(layers: usize, format: TextureFormat, filtered: bool) -> Image {
    let size = u32::try_from(CLIPMAP_SIZE).expect("small");
    let extent = Extent3d { width: size, height: size, depth_or_array_layers: u32::try_from(layers).expect("small") };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    array_view(&mut image);
    let filter = if filtered { ImageFilterMode::Linear } else { ImageFilterMode::Nearest };
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: filter,
        min_filter: filter,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// GPU formats of the three layer arrays, [`sf_terrain::LayerArrays`] order (T6): albedo +
/// height BC7 sRGB, normal X/Y BC5, AO + roughness BC5.
pub const LAYER_ARRAY_FORMATS: [TextureFormat; 3] =
    [TextureFormat::Bc7RgbaUnormSrgb, TextureFormat::Bc5RgUnorm, TextureFormat::Bc5RgUnorm];

/// The three layer array images of baked data (formats [`LAYER_ARRAY_FORMATS`]).
fn layer_images(size: u32, mips: u32, layers: usize, data: [Vec<u8>; 3]) -> [Image; 3] {
    let mut data = data.into_iter();
    LAYER_ARRAY_FORMATS.map(|f| layer_array(size, mips, layers, f, data.next().expect("three arrays")))
}

/// The load line: `terrain layers: N x S², X MiB (L levels)`.
fn layer_arrays_line(a: &sf_terrain::LayerArrays) -> String {
    #[allow(clippy::cast_precision_loss)]
    let mib = a.gpu_bytes() as f64 / f64::from(1u32 << 20);
    format!("terrain layers: {} x {}², {mib:.1} MiB ({} levels)", a.table.layers.len(), a.size, a.mips)
}

/// A layer texture array with its mip chain (bake layout: layer-major).
fn layer_array(size: u32, mips: u32, layers: usize, format: TextureFormat, data: Vec<u8>) -> Image {
    let extent =
        Extent3d { width: size, height: size, depth_or_array_layers: u32::try_from(layers.max(1)).expect("small") };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    image.texture_descriptor.mip_level_count = mips;
    image.data = Some(data);
    array_view(&mut image);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 16,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// Creates the GPU resources when a [`TerrainWorld`] appears or changes.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn setup_terrain(
    mut commands: Commands,
    world: Option<Res<TerrainWorld>>,
    state: Option<ResMut<TerrainState>>,
    settings: Res<TerrainSettings>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    far_map: Option<Res<FarShadowMap>>,
    compressed: Option<Res<CompressedImageFormatSupport>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut heights_map: ResMut<TerrainHeightsMap>,
    mut field_rev: Local<u64>,
) {
    // The old state's patches and the query tiles it put in `TerrainHeightsMap` (edits stay).
    let mut retire = |commands: &mut Commands, state: &TerrainState| {
        for (e, _) in state.nodes.values() {
            commands.entity(*e).despawn();
        }
        if let Some(heights) = heights_map.0.get_mut(&state.ws) {
            for c in state.tiles.keys() {
                heights.remove(*c);
            }
        }
    };
    let Some(world) = world else {
        if let Some(state) = state {
            retire(&mut commands, &state);
            commands.remove_resource::<TerrainState>();
            commands.remove_resource::<gpu_fill::GpuFillMain>();
        }
        return;
    };
    if !world.is_changed() && state.is_some() {
        return;
    }
    if let Some(state) = state {
        retire(&mut commands, &state);
    }
    let tiles = world.tiles.clone();
    // Headless runs have no device (nothing uploads); a device without BC can't take the arrays.
    let bc = compressed.is_none_or(|c| c.0.contains(CompressedImageFormats::BC));
    let (table, [albedo, normal, ao_roughness]) = match tiles.layers_capped(settings.layer_array_size, &settings.layer_cap) {
        Some(Ok(a)) if bc => {
            log::info!("{}", layer_arrays_line(&a));
            let images = layer_images(a.size, a.mips, a.table.layers.len(), [a.albedo_height, a.normal, a.ao_roughness]);
            (a.table, images)
        }
        other => {
            match other {
                Some(Err(e)) => log::error!("terrain layer arrays: {e}"),
                Some(Ok(_)) => log::error!(
                    "terrain layer arrays need BC texture compression (TEXTURE_COMPRESSION_BC), which this GPU lacks:                      drawing flat default layers"
                ),
                None => {}
            }
            let one = |c: [u8; 4], f| layer_array(1, 1, 1, f, c.to_vec());
            let [a, n, r] = sf_terrain::bake::LAYER_DEFAULTS;
            (
                default_layer_table(a, tiles.fill_height()),
                [
                    one(a, TextureFormat::Rgba8UnormSrgb),
                    one(n, TextureFormat::Rgba8Unorm),
                    one(r, TextureFormat::Rgba8Unorm),
                ],
            )
        }
    };
    if table.layers.len() > params::MAX_LAYERS {
        log::error!("terrain: {} layers, only {} are drawn", table.layers.len(), params::MAX_LAYERS);
    }
    let slots = table.layers.iter().enumerate().filter_map(|(n, l)| u8::try_from(n).ok().map(|s| (l.id, s))).collect();
    let heights = images.add(clipmap_image(HEIGHT_LEVELS, TextureFormat::R32Float, false));
    let normals = images.add(clipmap_image(HEIGHT_LEVELS, TextureFormat::Rg8Snorm, true));
    let layer_ids = images.add(clipmap_image(LAYER_LEVELS, TextureFormat::Rgba8Uint, false));
    let layer_weights = images.add(clipmap_image(LAYER_LEVELS, TextureFormat::Rgba8Unorm, false));
    let macro_image = images.add(clipmap_image(MACRO_LEVELS, TextureFormat::Rgba8UnormSrgb, true));
    let params = buffers.add(ShaderBuffer::new(Params::default().0, RenderAssetUsages::default()));
    let material = materials.add(TerrainMaterial {
        params: params.clone(),
        heights: heights.clone(),
        normals: normals.clone(),
        layer_ids: layer_ids.clone(),
        layer_weights: layer_weights.clone(),
        macro_albedo: macro_image.clone(),
        albedo_height: images.add(albedo),
        normal: images.add(normal),
        ao_roughness: images.add(ao_roughness),
        far_map: far_map.map(|m| m.image.clone()).unwrap_or_default(),
    });
    // What the bake used where there is no source, for tiles that don't exist.
    let fill = table.fill_height;
    let fill_slot = table.fill_layer.and_then(|id| table.slice_of(id)).map_or(0, |s| u8::try_from(s).unwrap_or(0));
    let fill_rgba =
        table.fill_layer.and_then(|id| table.layers.iter().find(|l| l.id == id)).map_or([70, 70, 70, 255], |l| {
            let c = l.mean_albedo.map(sf_terrain::bake::linear_to_srgb);
            [c[0], c[1], c[2], 255]
        });
    // The GPU fill when the provider has a field (and it isn't switched off).
    let field = if settings.gpu_fill { tiles.gpu_field() } else { None };
    let gpu = field.map(|field| {
        *field_rev += 1;
        let source = sf_terrain::gpu_fill::compose(&field.wgsl);
        let shader = shaders.add(Shader::from_wgsl(source, format!("sf_terrain_bevy/gpu_fill/{}.wgsl", *field_rev)));
        let ready = Arc::new(AtomicBool::new(false));
        let counters = Arc::new(GpuFillCounters::default());
        #[allow(clippy::cast_precision_loss)]
        let mib = (field.data.len() * 4) as f64 / f64::from(1u32 << 20);
        log::info!("terrain: clipmaps fill on the GPU from {} ({mib:.1} MiB of field data)", field.label);
        commands.insert_resource(gpu_fill::GpuFillMain {
            shader,
            data: field.data.clone(),
            rev: *field_rev,
            heights: heights.id(),
            normals: normals.id(),
            macro_image: macro_image.id(),
            ready: ready.clone(),
            counters: counters.clone(),
        });
        GpuMode { bounds: HashMap::new(), ready, counters, layers: tiles.has_layer_maps().then(HashMap::new) }
    });
    if gpu.is_none() {
        commands.remove_resource::<gpu_fill::GpuFillMain>();
    }
    commands.insert_resource(TerrainState {
        ws: tiles.worldspace().clone(),
        src: tiles.clone(),
        material,
        params,
        heights,
        normals,
        layer_ids,
        layer_weights,
        macro_image,
        patch: meshes.add(patch_mesh(cdlod::PATCH_QUADS)),
        quadrant: meshes.add(patch_mesh(cdlod::PATCH_QUADS / 2)),
        table,
        slots,
        tiles: HashMap::new(),
        height_windows: [None; HEIGHT_LEVELS],
        layer_windows: [None; LAYER_LEVELS],
        macro_windows: [None; MACRO_LEVELS],
        nodes: HashMap::new(),
        origin: None,
        shift_settle: 0,
        fill,
        fill_slot,
        fill_rgba,
        max_height: fill,
        select_rev: 0,
        gpu,
        gate: SelectGate::default(),
    });
}

/// Without layer arrays: one flat default layer (the 1×1 `albedo` slice) whose mean is its own
/// colour, so the shader's tint (macro / layer mean) gives back the macro albedo near the camera
/// as well as far (an empty table left the mean at 0 and the tint at its ×4 clamp).
fn default_layer_table(albedo: [u8; 4], fill_height: f32) -> LayerTable {
    let lin = |c: u8| {
        let s = f32::from(c) / 255.0;
        if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    let id = sf_terrain::layer_id("sf.terrain.default");
    let height = f32::from(albedo[3]) / 255.0;
    LayerTable {
        layers: vec![sf_terrain::LayerInfo {
            record: "sf.terrain.default".to_owned(),
            id,
            tiling_m: 4.0,
            triplanar: false,
            displacement_m: 0.0,
            mean_albedo: [lin(albedo[0]), lin(albedo[1]), lin(albedo[2])],
            mean_height: height,
            p90_height: height,
            source_edge: 1,
        }],
        fill_layer: Some(id),
        fill_height,
    }
}

fn level_u8(l: usize) -> u8 {
    u8::try_from(l).expect("small level")
}

/// Tiles at `level` overlapping a window (plus `margin` samples) whose
/// lattice has `per` samples per tile.
fn window_tiles(w: ClipWindow, level: u8, per: i64, margin: i64, out: &mut HashSet<TileCoord>) {
    let lo = |a: usize| (w.min[a] - margin).div_euclid(per);
    let hi = |a: usize| (w.min[a] + CLIPMAP_SIZE - 1 + margin).div_euclid(per);
    for z in lo(1)..=hi(1) {
        for x in lo(0)..=hi(0) {
            if let (Ok(x), Ok(z)) = (i32::try_from(x), i32::try_from(z)) {
                out.insert(TileCoord::new(level, x, z));
            }
        }
    }
}

/// What each clipmap kind needs: (heights, layers, macro) windows' tiles.
struct Wanted {
    heights: [ClipWindow; HEIGHT_LEVELS],
    layers: [ClipWindow; LAYER_LEVELS],
    macros: [ClipWindow; MACRO_LEVELS],
}

impl Wanted {
    fn new(cam: DVec2, snap: i64) -> Self {
        let s = |shift: usize| u32::try_from(shift).expect("small");
        Self {
            heights: std::array::from_fn(|l| ClipWindow::centered(cam, s(l), snap)),
            layers: std::array::from_fn(|l| ClipWindow::centered(cam, s(l + 1), snap)),
            macros: std::array::from_fn(|l| ClipWindow::centered(cam, s(l + 1), snap)),
        }
    }

    fn tiles(&self, margin: i64) -> HashSet<TileCoord> {
        let mut out = HashSet::new();
        let per_h = i64::try_from(sf_terrain::TILE_QUADS).expect("small");
        let per_l = per_h / 2;
        for (l, w) in self.heights.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_h, margin, &mut out);
        }
        for (l, w) in self.layers.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_l, margin, &mut out);
        }
        for (l, w) in self.macros.iter().enumerate() {
            window_tiles(*w, level_u8(l), per_l, margin, &mut out);
        }
        out
    }
}

fn load_tile(tiles: Arc<dyn TerrainTiles>, c: TileCoord, wants_layers: bool, wants_macro: bool) -> Option<Loaded> {
    let product = match tiles.tile(c)? {
        Ok(p) => p,
        Err(e) => {
            log::error!("terrain tile {c:?}: {e}");
            return None;
        }
    };
    let macro_rgba = if wants_macro { tiles.macro_albedo(c).and_then(Result::ok).map(|m| m.rgba) } else { None };
    let tile = product.height_tile();
    // A provider whose products carry no layers may give them alone (a generator, `TerrainTiles::layer_map`).
    let layers = match product.layers {
        Some(l) if wants_layers => Some(l),
        None if wants_layers => load_layer_map(tiles.as_ref(), c),
        _ => None,
    };
    Some(Loaded { resident: Arc::new(ResidentTile { tile, bounds: product.bounds }), layers, macro_rgba })
}

/// Tile `c`'s layer map alone (`TerrainTiles::layer_map`); errors are logged.
fn load_layer_map(tiles: &dyn TerrainTiles, c: TileCoord) -> Option<LayerMap> {
    match tiles.layer_map(c)? {
        Ok(m) => Some(m),
        Err(e) => {
            log::error!("terrain layer map {c:?}: {e}");
            None
        }
    }
}

/// The tiles a GPU-filled world loads layer maps for: every tile its near layer windows read, plus `margin`
/// samples (hysteresis, as the tile path's).
fn gpu_layer_tiles(wanted: &Wanted, margin: i64) -> HashSet<TileCoord> {
    let mut out = HashSet::new();
    let per_l = i64::try_from(sf_terrain::TILE_QUADS).expect("small") / 2;
    for (l, w) in wanted.layers.iter().enumerate() {
        window_tiles(*w, level_u8(l), per_l, margin, &mut out);
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Height,
    Layer,
    Macro,
}

/// Queues the strips a window move exposes (CPU texture writes, or GPU fill pieces). Returns the bytes queued.
#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
fn upload_level(
    state: &TerrainState,
    edits: Option<&sf_terrain::HeightEdits>,
    kind: Kind,
    level: usize,
    old: Option<ClipWindow>,
    new: ClipWindow,
    writes: &mut Vec<TextureWrite>,
    gpu_queue: &mut Vec<FillRequest>,
) -> usize {
    let lv = level_u8(level);
    let ready = |c: TileCoord| match state.tiles.get(&c) {
        Some(Slot::Ready(t)) => Some(t.clone()),
        _ => None,
    };
    let mut bytes = 0;
    for rect in clipmap::exposed(old, new) {
        for (piece, texel) in clipmap::toroidal_pieces(rect) {
            let n = piece.count();
            let size = [u32::try_from(piece.size[0]).expect("small"), u32::try_from(piece.size[1]).expect("small")];
            let layer = u32::try_from(level).expect("small");
            // A GPU-filled world: the render world evaluates the piece (heights + normals, or macro); the budget
            // counts the bytes it writes.
            let gpu_kind = match kind {
                Kind::Height => Some((FillKind::Heights, 6)),
                Kind::Macro => Some((FillKind::Macro, 4)),
                Kind::Layer => None,
            };
            if let (Some(_), Some((fill, per))) = (&state.gpu, gpu_kind) {
                gpu_queue.push(FillRequest { kind: fill, level: lv, rect: piece, texel });
                bytes += n * per;
                continue;
            }
            match kind {
                Kind::Height => {
                    // One sample of margin: normals difference their neighbors.
                    let grown = SampleRect {
                        min: [piece.min[0] - 1, piece.min[1] - 1],
                        size: [piece.size[0] + 2, piece.size[1] + 2],
                    };
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(grown, lv, 512).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    let tile = |c: TileCoord| held.get(&c).map(|t| &t.resident.tile);
                    let mut out = vec![0.0f32; n];
                    clipmap::fill_heights(lv, piece, &tile, edits, state.fill, &mut out);
                    let data: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
                    let mut normals = vec![[0i8; 2]; n];
                    clipmap::fill_normals(lv, piece, &tile, edits, state.fill, &mut normals);
                    let normals: Vec<u8> = normals.iter().flat_map(|v| v.map(i8::cast_unsigned)).collect();
                    bytes += data.len() + normals.len();
                    writes.push(TextureWrite {
                        image: state.heights.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data,
                    });
                    writes.push(TextureWrite {
                        image: state.normals.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 2,
                        data: normals,
                    });
                }
                Kind::Layer => {
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(piece, lv, 256).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    // A GPU-filled world's layer maps live apart from its twin's height tiles.
                    let gpu_layers = state.gpu.as_ref().and_then(|g| g.layers.as_ref());
                    let tile = |c: TileCoord| match gpu_layers {
                        Some(maps) => match maps.get(&c) {
                            Some(LayerSlot::Ready(m)) => Some(m.as_ref()),
                            _ => None,
                        },
                        None => held.get(&c).and_then(|t| t.layers.as_ref()),
                    };
                    let slot_of = |id: LayerId| state.slots.get(&id).copied().unwrap_or(0);
                    let mut ids = vec![[0u8; 4]; n];
                    let mut weights = vec![[0u8; 4]; n];
                    clipmap::fill_layers(lv, piece, &tile, &slot_of, state.fill_slot, &mut ids, &mut weights);
                    let ids: Vec<u8> = ids.concat();
                    let weights: Vec<u8> = weights.concat();
                    bytes += ids.len() + weights.len();
                    writes.push(TextureWrite {
                        image: state.layer_ids.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data: ids,
                    });
                    writes.push(TextureWrite {
                        image: state.layer_weights.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data: weights,
                    });
                }
                Kind::Macro => {
                    let held: HashMap<TileCoord, Arc<Loaded>> =
                        tiles_of(piece, lv, 256).filter_map(|c| ready(c).map(|t| (c, t))).collect();
                    let tile = |c: TileCoord| held.get(&c).and_then(|t| t.macro_rgba.as_deref());
                    let mut out = vec![[0u8; 4]; n];
                    clipmap::fill_rgba(lv, piece, &tile, state.fill_rgba, &mut out);
                    let data: Vec<u8> = out.concat();
                    bytes += data.len();
                    writes.push(TextureWrite {
                        image: state.macro_image.id(),
                        layer,
                        origin: texel,
                        size,
                        bytes_per_texel: 4,
                        data,
                    });
                }
            }
        }
    }
    bytes
}

fn tiles_of(rect: SampleRect, level: u8, per: i64) -> impl Iterator<Item = TileCoord> {
    let (x0, z0) = (rect.min[0].div_euclid(per), rect.min[1].div_euclid(per));
    let (x1, z1) = ((rect.min[0] + rect.size[0] - 1).div_euclid(per), (rect.min[1] + rect.size[1] - 1).div_euclid(per));
    (z0..=z1).flat_map(move |z| {
        (x0..=x1).filter_map(move |x| Some(TileCoord::new(level, i32::try_from(x).ok()?, i32::try_from(z).ok()?)))
    })
}

/// Whether every tile a window of `kind` reads is loaded (or known absent).
fn window_ready(state: &TerrainState, kind: Kind, w: ClipWindow, level: usize, per: i64, margin: i64) -> bool {
    let mut needed = HashSet::new();
    window_tiles(w, level_u8(level), per, margin, &mut needed);
    match &state.gpu {
        // A GPU-filled world reads no height tile; its layers wait for their maps when the provider has them
        // (else they fill with the default slot).
        Some(g) => match (&g.layers, kind) {
            (Some(maps), Kind::Layer) => {
                needed.iter().all(|c| matches!(maps.get(c), Some(LayerSlot::Ready(_) | LayerSlot::Absent)))
            }
            _ => true,
        },
        None => needed.iter().all(|c| matches!(state.tiles.get(c), Some(Slot::Ready(_) | Slot::Absent))),
    }
}

/// Conservative node bounds from the finest resident level (bounds are
/// merged up the pyramid in the bake).
#[allow(clippy::many_single_char_names)]
fn node_bounds(state: &TerrainState, lod: i8, x: i64, z: i64) -> (f32, f32) {
    let blocks = i64::try_from(BLOCKS).expect("small");
    let lookup = |k: u8, bx: i64, bz: i64| -> Option<(f32, f32)> {
        let c =
            TileCoord::new(k, i32::try_from(bx.div_euclid(blocks)).ok()?, i32::try_from(bz.div_euclid(blocks)).ok()?);
        let block = |b: &BlockBounds| -> Option<(f32, f32)> {
            Some(b.get(usize::try_from(bx.rem_euclid(blocks)).ok()?, usize::try_from(bz.rem_euclid(blocks)).ok()?))
        };
        // A GPU-filled world: the provider's analytic bounds.
        if let Some(gpu) = &state.gpu {
            return block(gpu.bounds.get(&c)?);
        }
        match state.tiles.get(&c)? {
            Slot::Ready(t) => {
                let b: &BlockBounds = &t.resident.bounds;
                Some(b.get(usize::try_from(bx.rem_euclid(blocks)).ok()?, usize::try_from(bz.rem_euclid(blocks)).ok()?))
            }
            Slot::Absent => Some((state.fill, state.fill)),
            Slot::Loading(_) => None,
        }
    };
    let first = u8::try_from(lod.max(0)).expect("non-negative");
    for k in first..=level_u8(HEIGHT_LEVELS - 1) {
        let level_k = i8::try_from(k).expect("small");
        let found = if level_k == lod {
            let mut acc: Option<(f32, f32)> = None;
            for (dx, dz) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let Some(b) = lookup(k, x * 2 + dx, z * 2 + dz) else {
                    acc = None;
                    break;
                };
                acc = Some(acc.map_or(b, |a| (a.0.min(b.0), a.1.max(b.1))));
            }
            acc
        } else {
            let shift = u32::try_from(i32::from(level_k) - i32::from(lod) - 1).expect("k > lod");
            lookup(k, x >> shift, z >> shift)
        };
        if let Some(b) = found {
            return b;
        }
    }
    (state.fill, state.fill)
}

/// Frames (the shift's and the next) in which clipmap uploads wait after an
/// origin shift.
const SHIFT_SETTLE_FRAMES: u8 = 2;

/// Height margins (below, above) a CDLOD node's bounds grow by, meters: the layer detail
/// displacement either way on the two finest levels (LOD < 0), the raise
/// ([`TerrainSettings::raise_m`], W6b) upward on the levels it displaces (SW-5c: LOD <=
/// [`cdlod::RAISE_MAX_LOD`], whose range holds its distance fade's end); 5 cm of rounding slack
/// everywhere.
fn node_margins(lod: i8, detail_m: f32, raise_m: f32) -> (f32, f32) {
    const SLACK_M: f32 = 0.05;
    let detail = if lod < 0 { detail_m } else { 0.0 };
    let raise = if lod <= cdlod::RAISE_MAX_LOD { raise_m.max(0.0) } else { 0.0 };
    (detail + SLACK_M, detail + raise + SLACK_M)
}

/// The `P_DETAIL` slot: splat range, height-blend depth, ground reflectance (clamped to
/// Bevy's 0..1), max terrain height.
fn detail_params(settings: &TerrainSettings, max_height: f32) -> [f32; 4] {
    [settings.splat_range, settings.blend_depth, settings.reflectance.clamp(0.0, 1.0), max_height]
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn update_terrain(
    mut commands: Commands,
    state: Option<ResMut<TerrainState>>,
    settings: Res<TerrainSettings>,
    debug: Res<TerrainDebug>,
    origin: Res<TerrainOrigin>,
    far_map: Option<Res<FarShadowMap>>,
    mut heights_map: ResMut<TerrainHeightsMap>,
    mut terrain_stats: ResMut<TerrainStats>,
    mut writes: ResMut<ClipmapWrites>,
    mut gpu_queue: ResMut<gpu_fill::GpuFillQueue>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut edited: MessageReader<TerrainEdited>,
    camera: Query<&GlobalTransform, With<TerrainCamera>>,
) {
    let Some(mut state) = state else { return };
    let Some(cam) = camera.iter().next() else { return };
    let started = Instant::now();
    let state = &mut *state;
    // Selection inputs besides the camera (CW-2 gate key).
    let mut tiles_changed = settings.is_changed() || debug.is_changed();
    let cam_render = cam.translation();
    let cam_world = origin.0.offset(cam_render);
    let cam_g = global_coords(cam_world);
    let origin_g = global_coords(origin.0);
    let origin_moved = state.origin != Some(origin.0);
    state.shift_settle = if origin_moved { SHIFT_SETTLE_FRAMES } else { state.shift_settle.saturating_sub(1) };

    // A GPU-filled world: the tiles are the CPU twin's (queries only), they don't change the selection.
    let gpu = state.gpu.is_some();
    // 1. Finished loads.
    let heights = heights_map.0.entry(state.ws.clone()).or_default();
    let mut in_flight = 0;
    for (c, slot) in &mut state.tiles {
        if let Slot::Loading(task) = slot {
            match block_on(poll_once(task)) {
                Some(Some(t)) => {
                    let top = t.resident.bounds.max.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    state.max_height = state.max_height.max(top);
                    heights.insert(t.resident.clone());
                    *slot = Slot::Ready(Arc::new(t));
                    tiles_changed |= !gpu;
                }
                Some(None) => {
                    *slot = Slot::Absent;
                    tiles_changed |= !gpu;
                }
                None => in_flight += 1,
            }
        }
        let _ = c;
    }

    // 2. Requests (coarse levels first, so far terrain is never missing) and eviction. A GPU-filled world asks
    // only for its twin's tiles, the nearest first within a level.
    let wanted = Wanted::new(cam_g, settings.snap);
    let keep = if gpu { twin_tiles(cam_g, &wanted, &settings.twin) } else { wanted.tiles(128) };
    let mut requests: Vec<TileCoord> = keep.iter().copied().filter(|c| !state.tiles.contains_key(c)).collect();
    if gpu {
        // The camera's own level-0 tile first (the ground clamp and physics under the player read it), then the
        // cheap coarse levels, then the rest nearest first.
        let home = |c: &TileCoord| !(c.level == 0 && tile_distance2(*c, cam_g) <= 2 * 256 * 256);
        requests.sort_by_key(|c| (home(c), std::cmp::Reverse(c.level), tile_distance2(*c, cam_g)));
    } else {
        requests.sort_by_key(|c| (std::cmp::Reverse(c.level), c.z, c.x));
    }
    let max_loads = if gpu { settings.twin.max_loads } else { settings.max_loads };
    let pool = AsyncComputeTaskPool::get();
    for c in requests {
        if !state.src.has_tile(c) {
            state.tiles.insert(c, Slot::Absent);
            tiles_changed |= !gpu;
            continue;
        }
        if in_flight >= max_loads {
            break;
        }
        let src = state.src.clone();
        let wants_layers = !gpu && usize::from(c.level) < LAYER_LEVELS;
        state.tiles.insert(c, Slot::Loading(pool.spawn(async move { load_tile(src, c, wants_layers, !gpu) })));
        in_flight += 1;
    }
    let evict: Vec<TileCoord> = state
        .tiles
        .iter()
        .filter(|(c, s)| !keep.contains(c) && !matches!(s, Slot::Loading(_)))
        .map(|(c, _)| *c)
        .collect();
    tiles_changed |= !gpu && !evict.is_empty();
    for c in evict {
        state.tiles.remove(&c);
        heights.remove(c);
    }

    // 2b. A GPU-filled world's analytic bounds for every tile its windows cover (cheap: the provider's min/max
    // pyramid, no sample evaluated); new bounds change the selection.
    if let Some(g) = &mut state.gpu {
        let covered = wanted.tiles(0);
        g.bounds.retain(|c, _| covered.contains(c));
        for c in covered {
            if g.bounds.contains_key(&c) {
                continue;
            }
            let b = state.src.block_bounds(c).unwrap_or_else(|| BlockBounds {
                min: vec![state.fill; BLOCKS * BLOCKS],
                max: vec![state.fill; BLOCKS * BLOCKS],
            });
            let top = b.max.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            state.max_height = state.max_height.max(top);
            g.bounds.insert(c, Arc::new(b));
            tiles_changed = true;
        }
    }

    // 2c. A GPU-filled world's near layer maps (the provider's `layer_map`): finished loads, then requests for the
    // layer windows' tiles (coarse levels first, the nearest first), eviction outside them.
    if let Some(maps) = state.gpu.as_mut().and_then(|g| g.layers.as_mut()) {
        let mut loading = 0;
        for slot in maps.values_mut() {
            if let LayerSlot::Loading(task) = slot {
                match block_on(poll_once(task)) {
                    Some(Some(m)) => *slot = LayerSlot::Ready(Arc::new(m)),
                    Some(None) => *slot = LayerSlot::Absent,
                    None => loading += 1,
                }
            }
        }
        let keep = gpu_layer_tiles(&wanted, 128);
        maps.retain(|c, s| keep.contains(c) || matches!(s, LayerSlot::Loading(_)));
        let mut requests: Vec<TileCoord> = keep.iter().copied().filter(|c| !maps.contains_key(c)).collect();
        requests.sort_by_key(|c| (std::cmp::Reverse(c.level), tile_distance2(*c, cam_g)));
        for c in requests {
            if loading >= settings.max_loads {
                break;
            }
            let src = state.src.clone();
            maps.insert(c, LayerSlot::Loading(pool.spawn(async move { load_layer_map(src.as_ref(), c) })));
            loading += 1;
        }
        in_flight += loading;
    }

    // 3. Edits invalidate the clipmap levels they touch.
    for TerrainEdited(_) in edited.read() {
        state.height_windows = [None; HEIGHT_LEVELS];
        tiles_changed = true;
    }

    // 4. Clipmap uploads, one whole level move at a time, under the budget.
    let nothing_resident = state.height_windows.iter().all(Option::is_none);
    // Uploads wait out the frames around an origin shift (streaming.md,
    // "Spikes"): the render world is busy re-collecting every instance.
    let budget = if state.gpu.as_ref().is_some_and(|g| !g.ready.load(Ordering::Relaxed)) {
        // The GPU fill's pipelines are still compiling: nothing would write the windows yet.
        0
    } else if nothing_resident {
        settings.burst_budget
    } else if state.shift_settle > 0 {
        0
    } else {
        settings.upload_budget
    };
    let mut used = 0;
    let edits = Some(heights.edits());
    let per_h = i64::try_from(sf_terrain::TILE_QUADS).expect("small");
    for level in (0..HEIGHT_LEVELS).rev() {
        for kind in [Kind::Height, Kind::Macro, Kind::Layer] {
            if used >= budget {
                break;
            }
            let (old, new, per) = match kind {
                Kind::Height => (state.height_windows[level], wanted.heights[level], per_h),
                Kind::Macro => (state.macro_windows[level], wanted.macros[level], per_h / 2),
                Kind::Layer if level < LAYER_LEVELS => (state.layer_windows[level], wanted.layers[level], per_h / 2),
                Kind::Layer => continue,
            };
            if old == Some(new) || !window_ready(state, kind, new, level, per, i64::from(kind == Kind::Height)) {
                continue;
            }
            used += upload_level(state, edits, kind, level, old, new, &mut writes.0, &mut gpu_queue.0);
            match kind {
                Kind::Height => state.height_windows[level] = Some(new),
                Kind::Macro => state.macro_windows[level] = Some(new),
                Kind::Layer => state.layer_windows[level] = Some(new),
            }
        }
    }

    // 5. CDLOD selection and the patch entities.
    let select_start = Instant::now();
    if tiles_changed {
        state.select_rev += 1;
    }
    // An origin shift re-expresses the live patches in place (new transform
    // and bounds on the same entities) rather than respawning them: no entity
    // churn in the shift frame, which already rewrites every transform
    // (streaming.md, "Spikes"). A frozen selection still starts over.
    state.origin = Some(origin.0);
    if origin_moved && debug.freeze {
        for (_, (e, _)) in state.nodes.drain() {
            commands.entity(e).despawn();
        }
    }
    // The selection gate (CW-2, `TerrainSettings::select_gate`): skipped while the camera stays within the
    // quantum of the last selection and nothing in its key changed; the patches stay as they are.
    let select = !debug.freeze
        && state.gate.check(&settings.select_gate, &[GateSource { pos: cam_world.to_f64(), dir: None }], (state.select_rev, origin.0));
    if select {
        let mut selected: Vec<SelectedNode> = Vec::with_capacity(1024);
        let camera3 = bevy_math::DVec3::new(cam_g.x, cam_world.to_f64().y, cam_g.y);
        let bounds = |lod: i8, x: i64, z: i64| node_bounds(state, lod, x, z);
        cdlod::select(camera3, &settings.cdlod, &bounds, &mut selected);
        let oy = origin.0.to_f64().y;
        let mut live: HashSet<NodeKey> = HashSet::with_capacity(selected.len());
        let displacement = state.table.layers.iter().map(|l| l.displacement_m.abs()).fold(0.0f32, f32::max);
        for node in &selected {
            live.insert(node.key);
            let key = node.key;
            let (margin_below, margin_above) = node_margins(key.lod, displacement, settings.raise_m);
            // The raise is in the key so a changed raise rebuilds the bounds.
            let b = (node.min_height - margin_below, node.max_height + margin_above);
            let existing = state.nodes.get(&key).copied();
            if !origin_moved && existing.is_some_and(|(_, old)| old == b) {
                continue;
            }
            let g = key.first_sample();
            let spacing = NodeKey::spacing(key.lod);
            #[allow(clippy::cast_possible_truncation)]
            let translation = Vec3::new(((g.x - origin_g.x) * 0.5) as f32, 0.0, ((g.y - origin_g.y) * 0.5) as f32);
            let transform = Transform::from_translation(translation).with_scale(Vec3::new(spacing, 1.0, spacing));
            #[allow(clippy::cast_possible_truncation)]
            let (lo, hi) = ((f64::from(b.0) - oy) as f32, (f64::from(b.1) - oy) as f32);
            #[allow(clippy::cast_precision_loss)]
            let quads = key.quads() as f32;
            let aabb = Aabb::from_min_max(Vec3::new(0.0, lo, 0.0), Vec3::new(quads, hi.max(lo + 0.01), quads));
            if let Some((e, _)) = existing {
                // Bounds were coarser (or a fill guess) when it spawned, or
                // the origin moved. Replacing components keeps the archetype.
                commands.entity(e).insert((aabb, transform, GlobalTransform::from(transform)));
                state.nodes.insert(key, (e, b));
                continue;
            }
            let mesh = if key.quadrant.is_some() { state.quadrant.clone() } else { state.patch.clone() };
            let tag = u32::try_from(i32::from(key.lod) - i32::from(MIN_LOD)).expect("lod >= MIN_LOD");
            let e = commands
                .spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(state.material.clone()),
                    transform,
                    GlobalTransform::from(transform),
                    aabb,
                    NoAutoAabb,
                    MeshTag::new(tag),
                    TerrainPatch(key),
                ))
                .id();
            if debug.no_culling {
                commands.entity(e).insert(bevy_camera::visibility::NoFrustumCulling);
            }
            state.nodes.insert(key, (e, b));
        }
        state.nodes.retain(|k, (e, _)| {
            let keep = live.contains(k);
            if !keep {
                commands.entity(*e).despawn();
            }
            keep
        });
    }
    let select_ms = select_start.elapsed().as_secs_f32() * 1e3;

    // 6. Shader parameters.
    let mut p = Params::default();
    let oi = [origin_g.x.floor(), origin_g.y.floor()];
    #[allow(clippy::cast_possible_truncation)]
    let oi_i = oi.map(|v| v as i64);
    let o_abs = origin.0.to_f64();
    #[allow(clippy::cast_possible_truncation)]
    {
        p.set_f(params::P_CAMERA, [cam_render.x, cam_render.y, cam_render.z, o_abs.y as f32]);
        let low = |v: i64| i32::try_from(v.rem_euclid(1024)).expect("small");
        p.0[params::P_ORIGIN] = [
            ((origin_g.x - oi[0]) as f32).to_bits(),
            ((origin_g.y - oi[1]) as f32).to_bits(),
            low(oi_i[0]).cast_unsigned(),
            low(oi_i[1]).cast_unsigned(),
        ];
        let debug_bits =
            (u32::from(debug.lod_colors) * params::DEBUG_LOD) | (u32::from(debug.wireframe) * params::DEBUG_WIREFRAME)
                | (u32::from(debug.no_far_shadows) * params::DEBUG_NO_FAR_SHADOWS)
                | (u32::from(debug.no_specular) * params::DEBUG_NO_SPECULAR)
                | (u32::from(debug.no_indirect) * params::DEBUG_NO_INDIRECT)
                | (u32::from(debug.raw_layers) * params::DEBUG_RAW_LAYERS)
                | (u32::from(debug.no_env_specular) * params::DEBUG_NO_ENV_SPECULAR);
        p.0[params::P_MISC] = [
            (o_abs.x.rem_euclid(1024.0) as f32).to_bits(),
            (o_abs.z.rem_euclid(1024.0) as f32).to_bits(),
            state.fill.to_bits(),
            debug_bits,
        ];
    }
    let mask =
        |w: &[Option<ClipWindow>]| w.iter().enumerate().fold(0u32, |m, (l, w)| m | (u32::from(w.is_some()) << l));
    p.0[params::P_VALID] = [
        mask(&state.height_windows),
        mask(&state.layer_windows),
        mask(&state.macro_windows),
        u32::try_from(state.table.layers.len().min(params::MAX_LAYERS)).expect("small"),
    ];
    p.set_morph(&settings.cdlod, settings.morph_slack());
    let level_slot = |w: Option<ClipWindow>, shift: u32| -> [i32; 4] {
        let o = [oi_i[0] >> shift, oi_i[1] >> shift];
        let base = |a: usize| i32::try_from(o[a].rem_euclid(CLIPMAP_SIZE)).expect("small");
        let rel = |a: usize| w.map_or(0, |w| i32::try_from(w.min[a] - o[a]).unwrap_or(0));
        [base(0), base(1), rel(0), rel(1)]
    };
    for l in 0..HEIGHT_LEVELS {
        let s = u32::try_from(l).expect("small");
        p.set_i(params::P_HEIGHT + l, level_slot(state.height_windows[l], s));
        p.set_i(params::P_MACRO + l, level_slot(state.macro_windows[l], s + 1));
    }
    for l in 0..LAYER_LEVELS {
        p.set_i(params::P_LAYER + l, level_slot(state.layer_windows[l], u32::try_from(l).expect("small") + 1));
    }
    p.set_f(params::P_DETAIL, detail_params(&settings, state.max_height));
    p.set_f(params::P_RAISE, raise_params(&settings));
    for (n, l) in state.table.layers.iter().take(params::MAX_LAYERS).enumerate() {
        p.set_layer(n, l);
    }
    if let Some(far) = &far_map {
        p.set_f(params::P_FAR, far.params.to_array());
    }
    if let Some(mut buffer) = buffers.get_mut(&state.params)
        && let Some(slots) = buffer.cast_slice_mut::<[u32; 4]>()
    {
        slots.copy_from_slice(p.as_slice());
    }

    *terrain_stats = TerrainStats {
        patches: state.nodes.len(),
        resident_tiles: state.tiles.values().filter(|s| matches!(s, Slot::Ready(_))).count(),
        loads_in_flight: in_flight,
        upload_bytes: used,
        select_ms,
        update_ms: started.elapsed().as_secs_f32() * 1e3,
        height_levels: mask(&state.height_windows).count_ones(),
        gpu_fill: state.gpu.is_some(),
        gpu_pieces: state.gpu.as_ref().map_or(0, |g| g.counters.pieces.load(Ordering::Relaxed)),
        gpu_texels: state.gpu.as_ref().map_or(0, |g| g.counters.texels.load(Ordering::Relaxed)),
        gpu_frames: state.gpu.as_ref().map_or(0, |g| g.counters.frames.load(Ordering::Relaxed)),
    };
}

/// The CPU twin's tiles of a GPU-filled world ([`TwinSettings`]): the finest levels' tiles around the camera's
/// (`cam`, level-0 global sample coordinates) and every tile of the coarse levels' height windows.
fn twin_tiles(cam: DVec2, wanted: &Wanted, twin: &TwinSettings) -> HashSet<TileCoord> {
    let mut out = HashSet::new();
    let per = i64::try_from(sf_terrain::TILE_QUADS).expect("small");
    for level in 0..twin.near_levels.min(level_u8(HEIGHT_LEVELS)) {
        let size = f64::from(512u32 << level);
        #[allow(clippy::cast_possible_truncation)]
        let (tx, tz) = ((cam.x / size).floor() as i32, (cam.y / size).floor() as i32);
        for dz in -twin.near_radius..=twin.near_radius {
            for dx in -twin.near_radius..=twin.near_radius {
                out.insert(TileCoord::new(level, tx + dx, tz + dz));
            }
        }
    }
    for level in usize::from(twin.coarse_level)..HEIGHT_LEVELS {
        window_tiles(wanted.heights[level], level_u8(level), per, 0, &mut out);
    }
    out
}

/// Squared distance (level-0 samples²) from `cam` to tile `c`'s centre.
fn tile_distance2(c: TileCoord, cam: DVec2) -> i64 {
    let size = i64::from(512u32 << c.level);
    let (x0, z0) = c.first_sample();
    #[allow(clippy::cast_possible_truncation)]
    let (dx, dz) = (x0 + size / 2 - cam.x as i64, z0 + size / 2 - cam.y as i64);
    dx.saturating_mul(dx).saturating_add(dz.saturating_mul(dz))
}

#[cfg(test)]
#[path = "tests_gpu.rs"]
mod tests_gpu;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_mesh::VertexAttributeValues;

    /// T6: the layer arrays upload as BC7 sRGB + 2 x BC5 `2d_array` images with the baked
    /// size, levels and layers, and the load line names them.
    #[test]
    fn layer_arrays_upload_block_compressed() {
        let (size, mips, layers) = (16, 5, 3);
        let len = 3 * sf_terrain::LayerArrays::slice_len(size, mips);
        let images = layer_images(size, mips, layers, [vec![0; len], vec![0; len], vec![0; len]]);
        let formats: Vec<TextureFormat> = images.iter().map(|i| i.texture_descriptor.format).collect();
        assert_eq!(formats, [TextureFormat::Bc7RgbaUnormSrgb, TextureFormat::Bc5RgUnorm, TextureFormat::Bc5RgUnorm]);
        for image in &images {
            let d = &image.texture_descriptor;
            assert_eq!((d.size.width, d.size.height, d.size.depth_or_array_layers, d.mip_level_count), (16, 16, 3, 5));
            assert_eq!(image.data.as_ref().map(Vec::len), Some(len));
            let view = image.texture_view_descriptor.as_ref().and_then(|v| v.dimension);
            assert_eq!(view, Some(TextureViewDimension::D2Array));
        }
        let arrays = sf_terrain::LayerArrays {
            table: LayerTable { layers: Vec::new(), ..LayerTable::default() },
            size: 1024,
            mips: 11,
            albedo_height: vec![0; 1 << 20],
            normal: vec![0; 1 << 20],
            ao_roughness: vec![0; 1 << 20],
        };
        assert_eq!(layer_arrays_line(&arrays), "terrain layers: 0 x 1024², 3.0 MiB (11 levels)");
    }

    /// W6b/SW-5c: the raised cover grows the bounds of the levels it displaces (LOD <=
    /// `RAISE_MAX_LOD`), upward only; the detail displacement only the two finest.
    #[test]
    fn node_bounds_grow_by_the_raise_on_the_displaced_levels() {
        let near = |a: (f32, f32), b: (f32, f32)| (a.0 - b.0).abs() < 1e-6 && (a.1 - b.1).abs() < 1e-6;
        assert!(near(node_margins(-2, 0.1, 0.45), (0.15, 0.6)));
        assert!(near(node_margins(-1, 0.0, 0.45), (0.05, 0.5)));
        assert!(near(node_margins(0, 0.1, 0.45), (0.05, 0.5)));
        assert!(near(node_margins(1, 0.1, 0.45), (0.05, 0.5)));
        assert!(near(node_margins(cdlod::RAISE_MAX_LOD, 0.1, 0.45), (0.05, 0.5)));
        assert!(near(node_margins(cdlod::RAISE_MAX_LOD + 1, 0.1, 0.45), (0.05, 0.05)));
        assert!(near(node_margins(-1, 0.0, -1.0), (0.05, 0.05)));
    }

    /// W6b/SW-5c/SW-6: the shader's raised-cover fade and displaced levels are its CPU twin's
    /// (`CdlodSettings::raise_weight` / `vertex_raise`); `terrain_raise` prefilters to the one
    /// filter width of the repose bound (`surface_raise_filter` from `P_RAISE` and the coarsest
    /// layer level's 4 m texel, `sf_surface::layer::raise_filter_m`'s `base_texel_m`), reads the
    /// layers' bilinear p90 (`layer_raise_top`, no camera distance) and the finest normal level,
    /// never the vertex's spacing or LOD; the fade ends inside the splat range.
    #[test]
    fn raise_constants_match_the_twin() {
        let wgsl = include_str!("shaders/common.wesl");
        // SW-5e: the fade's range comes from P_RAISE (zw), the CPU's `raise_fade_range`.
        assert!(wgsl.contains("return select(1.0 - smoothstep(r.z, r.w, d), 0.0, d >= r.w);"));
        assert!(wgsl.contains(&format!("const RAISE_MAX_LOD: i32 = {};", cdlod::RAISE_MAX_LOD)));
        assert!(wgsl.contains("if lod <= RAISE_MAX_LOD {\n        out.raise_w = raise_fade("));
        let body = wgsl.split("fn terrain_raise(").nth(1).unwrap().split("\n}\n").next().unwrap();
        assert!(body.contains("let r_params = pf(P_RAISE);"), "{body}");
        assert!(body.contains("surface_raise_filter(r_params.x, r_params.y, raise_top_texel_m())"), "{body}");
        assert!(body.contains("let top = layer_raise_top(t.position.xz);"), "{body}");
        assert!(body.contains("terrain_normal(t.position.xz, 0.0), top, filter_m)"), "{body}");
        assert!(!body.contains("t.lod") && !body.contains("spacing") && !body.contains("camera_position"), "{body}");
        assert!(body.contains("!surface_raise_on()"), "{body}");
        // The high point reads the coarsest layer level first, 4 m texels (0.5 m x 2^LAYER_LEVELS).
        let top = wgsl.split("fn layer_raise_top(").nth(1).unwrap().split("\n}\n").next().unwrap();
        assert!(top.contains("let l = LAYER_LEVELS - k;") && !top.contains("dist"), "{top}");
        assert!(wgsl.contains("return 0.5 * exp2(f32(LAYER_LEVELS));"));
        assert_eq!(1usize << LAYER_LEVELS, 8, "the 4 m texel sf_surface's raise_tests assume");
        // P_RAISE carries tan(repose) and the minimum width from the tunables' defaults.
        let p = raise_params(&TerrainSettings::default());
        assert!((p[0] - 30f32.to_radians().tan()).abs() < 1e-6, "{p:?}");
        assert_eq!(p[1].to_bits(), TERRAIN_RAISE_FILTER_MIN_M.default_value().to_bits());
        let unlimited = TerrainSettings::default().cdlod.raise_fade_range(f32::INFINITY);
        assert_eq!((p[2], p[3]), unlimited, "no reach limit: the sub-pixel fade");
        let limited = raise_params(&TerrainSettings { raise_reach_m: 300.0, ..TerrainSettings::default() });
        assert_eq!((limited[2], limited[3]), (200.0, 300.0));
        let s = TerrainSettings::default();
        assert!(cdlod::RAISE_FADE_END * s.cdlod.lod0_range <= 0.7 * s.splat_range);
    }

    /// The WGSL slot constants must match `params.rs`.
    #[test]
    fn shader_param_slots_match_rust() {
        let wgsl = include_str!("shaders/common.wesl");
        let slot = |name: &str| -> usize {
            let line = wgsl.lines().find(|l| l.starts_with(&format!("const {name}: u32 = "))).expect(name);
            let value = line.split('=').nth(1).unwrap().trim();
            value.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap()
        };
        assert_eq!(slot("P_CAMERA"), params::P_CAMERA);
        assert_eq!(slot("P_ORIGIN"), params::P_ORIGIN);
        assert_eq!(slot("P_MISC"), params::P_MISC);
        assert_eq!(slot("P_VALID"), params::P_VALID);
        assert_eq!(slot("P_MORPH"), params::P_MORPH);
        assert_eq!(slot("P_HEIGHT"), params::P_HEIGHT);
        assert_eq!(slot("P_LAYER"), params::P_LAYER);
        assert_eq!(slot("P_MACRO"), params::P_MACRO);
        assert_eq!(slot("P_DETAIL"), params::P_DETAIL);
        assert_eq!(slot("P_LAYERS"), params::P_LAYERS);
        assert_eq!(slot("P_FAR"), params::P_FAR);
        assert_eq!(slot("P_RAISE"), params::P_RAISE);
        assert_eq!(slot("HEIGHT_LEVELS"), HEIGHT_LEVELS);
        assert_eq!(slot("LAYER_LEVELS"), LAYER_LEVELS);
        assert_eq!(params::P_MORPH + params::MORPH_LODS, params::P_HEIGHT);
        assert_eq!(params::P_MACRO + MACRO_LEVELS, params::P_DETAIL);
        for (name, bit) in [
            ("DEBUG_LOD", params::DEBUG_LOD),
            ("DEBUG_WIREFRAME", params::DEBUG_WIREFRAME),
            ("DEBUG_NO_FAR_SHADOWS", params::DEBUG_NO_FAR_SHADOWS),
            ("DEBUG_NO_SPECULAR", params::DEBUG_NO_SPECULAR),
            ("DEBUG_NO_INDIRECT", params::DEBUG_NO_INDIRECT),
            ("DEBUG_RAW_LAYERS", params::DEBUG_RAW_LAYERS),
            ("DEBUG_NO_ENV_SPECULAR", params::DEBUG_NO_ENV_SPECULAR),
        ] {
            assert_eq!(slot(name), usize::try_from(bit).unwrap(), "{name}");
        }
    }

    /// Patches split quads along the canonical diagonal, counter-clockwise from +Y.
    #[test]
    fn patch_mesh_matches_the_canonical_triangulation() {
        let mesh = patch_mesh(cdlod::PATCH_QUADS);
        let Some(VertexAttributeValues::Float32x3(p)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!() };
        let Some(Indices::U16(idx)) = mesh.indices() else { panic!() };
        assert_eq!(p.len(), 33 * 33);
        assert_eq!(idx.len(), 32 * 32 * 6);
        for t in idx.as_chunks::<3>().0 {
            let [a, b, c] = [t[0], t[1], t[2]].map(|i| Vec3::from(p[usize::from(i)]));
            assert!((b - a).cross(c - a).y > 0.0, "clockwise triangle {t:?}");
        }
        // First quad: (0,0)-(1,1) is the shared edge.
        assert_eq!(&idx[..6], &[0, 34, 1, 0, 33, 34]);
    }
    /// The ground reflectance reaches the shader's `P_DETAIL.z` (clamped to Bevy's 0..1), and the
    /// terrain fragment sets it before surface weather mixes wet areas toward water, with the
    /// RO-16 `DEBUG_NO_SPECULAR` override after both.
    #[test]
    #[allow(clippy::float_cmp)] // exact copies, no arithmetic
    fn ground_reflectance_reaches_the_detail_slot_before_surface_weather() {
        let mut settings = TerrainSettings::default();
        assert_eq!(detail_params(&settings, 100.0)[2], DEFAULT_REFLECTANCE);
        settings.reflectance = 0.2;
        assert_eq!(detail_params(&settings, 100.0), [settings.splat_range, settings.blend_depth, 0.2, 100.0]);
        settings.reflectance = 3.0;
        assert_eq!(detail_params(&settings, 100.0)[2], 1.0);
        settings.reflectance = -1.0;
        assert_eq!(detail_params(&settings, 100.0)[2], 0.0);

        // LF line ends: a Windows checkout with `core.autocrlf` has CRLF.
        let common = include_str!("shaders/common.wesl").replace("\r\n", "\n");
        assert!(common.contains("fn base_reflectance() -> f32 {\n    return pf(P_DETAIL).z;"));
        let terrain = include_str!("shaders/terrain.wesl");
        let at = |s: &str| terrain.find(s).unwrap_or_else(|| panic!("{s}"));
        let set = at("pbr.material.reflectance = vec3<f32>(base_reflectance());");
        let weather = at("apply_surface_weather(&pbr, si);");
        let off = at("if (debug & DEBUG_NO_SPECULAR) != 0u {");
        assert!(set < weather && weather < off, "order: reflectance {set}, weather {weather}, no-specular {off}");
    }

    /// Split-sum DFG terms (A, B) for GGX with height-correlated Smith (Bevy's DFG LUT and
    /// direct lighting), importance-sampled over the Hammersley set.
    #[allow(clippy::many_single_char_names)] // the paper's symbols
    fn dfg(perceptual_roughness: f32, n_dot_v: f32) -> (f64, f64) {
        const N: u32 = 4096;
        let a = f64::from(perceptual_roughness.max(0.089)).powi(2);
        let a2 = a * a;
        let nv = f64::from(n_dot_v);
        let v = [(1.0 - nv * nv).sqrt(), 0.0, nv];
        let (mut sum_a, mut sum_b) = (0.0, 0.0);
        for i in 0..N {
            let u = f64::from(i) / f64::from(N);
            let w = f64::from(i.reverse_bits()) / 4_294_967_296.0;
            let phi = std::f64::consts::TAU * u;
            let ct = ((1.0 - w) / (1.0 + (a2 - 1.0) * w)).sqrt();
            let st = (1.0 - ct * ct).sqrt();
            let h = [st * phi.cos(), st * phi.sin(), ct];
            let vh = v[0] * h[0] + v[2] * h[2];
            let nl = 2.0 * vh * h[2] - v[2];
            if nl <= 0.0 || vh <= 0.0 {
                continue;
            }
            let gv = nl * (nv * nv * (1.0 - a2) + a2).sqrt();
            let gl = nv * (nl * nl * (1.0 - a2) + a2).sqrt();
            let g = 0.5 / (gv + gl) * 4.0 * nl * vh / h[2];
            let fc = (1.0 - vh).powi(5);
            sum_a += (1.0 - fc) * g;
            sum_b += fc * g;
        }
        (sum_a / f64::from(N), sum_b / f64::from(N))
    }

    /// RO-16: how much of a night pixel under Bevy's flat ambient (`pbr_ambient.wesl`) is the
    /// albedo-independent specular film, per ground reflectance. Pixel = diffuse
    /// (albedo x A(1, N.V) + B(1, N.V)) + specular ((F0 x A + B) x saturate(16.5 x F0)), F0 =
    /// 0.16 x r²; the env map is black at night. Printed, never a limit (owner judges).
    #[test]
    #[ignore = "measurement: RO-16 flat-ambient specular film share per terrain reflectance"]
    fn measure_night_flat_ambient_specular_share() {
        let nvs = [0.1_f32, 0.15, 0.2, 0.5];
        for p in [0.85_f32, 0.9, 1.0] {
            let row: Vec<String> = nvs
                .iter()
                .map(|&nv| {
                    let (a, b) = dfg(p, nv);
                    format!("N.V {nv}: A {a:.3} B {b:.4}")
                })
                .collect();
            println!("roughness {p}: {}", row.join(" | "));
        }
        println!("spec share of the pixel, % (albedo 0.15 / 0.08), roughness 0.85 then 0.9, N.V 0.1 0.15 0.2 0.5");
        for r in [0.5_f64, 0.4, 0.35, 0.3, 0.25, 0.2, 0.15, 0.1, 0.0] {
            let f0 = 0.16 * r * r;
            let so = (16.5 * f0).clamp(0.0, 1.0);
            let mut cells = Vec::new();
            for p in [0.85_f32, 0.9] {
                for &nv in &nvs {
                    let (a, b) = dfg(p, nv);
                    let (a1, b1) = dfg(1.0, nv);
                    let spec = (f0 * a + b) * so;
                    let share = |albedo: f64| 100.0 * spec / (spec + albedo * a1 + b1);
                    cells.push(format!("{:4.1}/{:4.1}", share(0.15), share(0.08)));
                }
            }
            println!("r {r:.2} F0 {f0:.4} occl {so:.3} spec {:.4}@.15: {}", {
                let (a, b) = dfg(0.85, 0.15);
                (f0 * a + b) * so
            }, cells.join(" "));
        }
    }

    /// CW-2: a CDLOD selection kept while the camera moves less than the gate's quantum stays crack-free
    /// with the morph slack: wherever a patch borders a coarser one, its edge vertices have finished
    /// morphing (k = 1) and the coarser patch's haven't started (k = 0). Without the slack, the same
    /// staleness cracks; a fresh selection without it doesn't (the ungated baseline).
    #[test]
    fn stale_selection_stays_crack_free_with_the_morph_slack() {
        use bevy_math::DVec3;
        let cdlod = CdlodSettings::default();
        let q = TERRAIN_SELECT_MOVE_M.default_value();
        let c0 = DVec3::new(1000.3, 2.0, 2000.7); // meters
        let mut nodes = Vec::new();
        cdlod::select(DVec3::new(c0.x * 2.0, c0.y, c0.z * 2.0), &cdlod, &|_, _, _| (0.0, 0.0), &mut nodes);
        assert!(nodes.len() > 100);
        let drawn: HashSet<NodeKey> = nodes.iter().map(|n| n.key).collect();
        // The LOD of the drawn patch over level-0 sample point `g`.
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let lod_at = |g: DVec2| -> Option<i8> {
            (cdlod.min_lod..=cdlod.max_lod).find(|&lod| {
                let size = NodeKey::node_samples(lod);
                let (x, z) = ((g.x / size).floor() as i64, (g.y / size).floor() as i64);
                let quad = u8::from(g.x / size - x as f64 >= 0.5) | (u8::from(g.y / size - z as f64 >= 0.5) << 1);
                drawn.contains(&NodeKey { lod, x, z, quadrant: None }) || drawn.contains(&NodeKey { lod, x, z, quadrant: Some(quad) })
            })
        };
        // The shader's morph (`common.wesl` `terrain_vertex`) on flat ground at height 0.
        #[allow(clippy::cast_possible_truncation)]
        let morph = |lod: i8, g: DVec2, cam: DVec3, slack: f32| -> f32 {
            let (start, end) = params::morph_zone(&cdlod, lod, slack);
            let d = (DVec3::new(g.x * 0.5, 0.0, g.y * 0.5) - cam).length() as f32;
            ((d - start) / (end - start)).clamp(0.0, 1.0)
        };
        // Worst crack over every border with a coarser patch: max of (1 - fine k, coarse k).
        let worst = |cam: DVec3, slack: f32| -> f32 {
            let mut worst = 0.0f32;
            for n in &nodes {
                let k = n.key;
                let o = k.first_sample();
                let step = NodeKey::node_samples(k.lod) / f64::from(cdlod::PATCH_QUADS);
                let edge = f64::from(k.quads()) * step;
                for i in 0..=k.quads() {
                    let t = f64::from(i) * step;
                    for (g, out) in [
                        (DVec2::new(o.x + t, o.y), DVec2::new(0.0, -1.0)),
                        (DVec2::new(o.x + t, o.y + edge), DVec2::new(0.0, 1.0)),
                        (DVec2::new(o.x, o.y + t), DVec2::new(-1.0, 0.0)),
                        (DVec2::new(o.x + edge, o.y + t), DVec2::new(1.0, 0.0)),
                    ] {
                        let Some(other) = lod_at(g + out * 0.01) else { continue };
                        if other > k.lod {
                            assert_eq!(other, k.lod + 1, "neighbors differ by one LOD");
                            worst = worst.max(1.0 - morph(k.lod, g, cam, slack)).max(morph(other, g, cam, slack));
                        }
                    }
                }
            }
            worst
        };
        assert!(worst(c0, 0.0) < 1e-3, "fresh selection, no slack");
        let stale: Vec<DVec3> = (0..16)
            .map(|i| {
                let a = f64::from(i) * std::f64::consts::TAU / 16.0;
                DVec3::new(a.cos(), 0.0, a.sin())
            })
            .chain([DVec3::Y, DVec3::NEG_Y, DVec3::new(1.0, 1.0, 0.0).normalize()])
            .map(|dir| c0 + dir * f64::from(q) * 0.99)
            .collect();
        for cam in &stale {
            let w = worst(*cam, q);
            assert!(w < 1e-3, "camera {cam}: a crack of {w} with the slack");
        }
        let unslacked = stale.iter().map(|c| worst(*c, 0.0)).fold(0.0f32, f32::max);
        assert!(unslacked > 1e-2, "without the slack a stale selection cracks ({unslacked})");
    }

    /// CW-2: the terrain gate's key. A still camera skips, a quantum of travel, a tile change (`select_rev`)
    /// or an origin shift re-selects; off, it always selects.
    #[test]
    fn terrain_gate_keys_on_camera_tiles_and_origin() {
        let q = TerrainSettings::select_gate_quanta(true);
        let off = TerrainSettings::default().select_gate;
        assert!(!off.enabled, "off by default until the owner's A/B");
        assert!(TerrainSettings::default().morph_slack() == 0.0, "no slack while off");
        let mut gate = SelectGate::<(u64, WorldPos)>::default();
        let cam = |x: f64| [GateSource { pos: bevy_math::DVec3::new(x, 10.0, 0.0), dir: None }];
        let o = WorldPos::from_f64(bevy_math::DVec3::ZERO);
        assert!(gate.check(&q, &cam(0.0), (0, o)));
        assert!(!gate.check(&q, &cam(0.0), (0, o)), "still");
        assert!(!gate.check(&q, &cam(f64::from(q.move_m) * 0.9), (0, o)));
        assert!(gate.check(&q, &cam(f64::from(q.move_m) * 1.01), (0, o)), "moved a quantum");
        assert!(gate.check(&q, &cam(f64::from(q.move_m) * 1.01), (1, o)), "a tile changed");
        let shifted = WorldPos::from_f64(bevy_math::DVec3::new(256.0, 0.0, 0.0));
        assert!(gate.check(&q, &cam(f64::from(q.move_m) * 1.01), (1, shifted)), "origin shift");
        assert!(!gate.check(&q, &cam(f64::from(q.move_m) * 1.01), (1, shifted)));
        assert!(gate.check(&off, &cam(0.0), (1, shifted)) && gate.check(&off, &cam(0.0), (1, shifted)));
    }
}
