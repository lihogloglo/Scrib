//! A generated world as the exterior (worldgen slice G1, `view world:<guide dir>`): the terrain comes from a
//! [`TerrainTiles`] provider that generates tiles on demand (the app builds it, `sf_worldgen::tiles`) instead of
//! the bake store, and the exterior's streaming index is built in memory from [`WorldgenSetup::content`]: empty
//! when the app gives none (nothing of the loaded content streams in), else the content's placements already at
//! their places in the generated world (WG-X4: anchor members through the anchor transform) plus any extra passes
//! ([`WorldgenContent::extra`], WG-X5 flora scatter). Sky, atmosphere, clouds, weather, water and the camera stay
//! the exterior's. The cloud layer rises by [`CLOUD_BASE_LIFT_M`] so high summits can pierce it
//! (`WeatherOverrides::cloud_base_lift_m`).
//!
//! Scaffold `WorldgenPlacementStatics` (WG-X4): the static scene's cell tables and the near precombines are baked
//! per **source** cell, so a generated world turns both off (`exterior.rs`) and its statics spawn one entity per
//! placement (the `--off scene` path); the far field is off too. Removal path: bake the generated world as its own
//! worldspace (index, scene, precombine, far field) in a one-time bake step.
//!
//! Slice G2: the provider's GPU field fills the clipmaps (`sf_terrain_bevy` gpu fill; `--off gputerrain` goes back
//! to CPU tiles), and the erosion filter is live: the console's `worldgen.filter false` / `worldgen.octaves <n>`
//! (or `--set`) rebuild the provider with the new filter and the terrain refills ([`WORLDGEN_FILTER`],
//! [`WORLDGEN_OCTAVES`]).
//!
//! RG-P5a (rockgen.md "(f)"): library rocks (`ROCK_LIBRARY`, model `rockgen/<class>/<index>`) index with the
//! variant bounds of [`WorldgenContent::models`] and draw through a [`RockSpawner`] registered at startup when the
//! content carries [`WorldgenContent::rock_libraries`] (`--off rocklibrary`: the app reads none).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy::math::{DVec3, IVec3};
use bevy::prelude::*;
use sf_bake::{Manifest, Store};
use sf_coords::WorldPos;
use sf_model::steps::ModelInfoStep;
use sf_terrain_bevy::sf_terrain::TerrainTiles;
use sf_terrain_bevy::{TerrainStats, TerrainWorld};
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, Source, Unit, def};
use sf_vfs::VfsPath;
use sf_world::{
    IndexParams, IndexReader, MemoryIndex, ModelInfo, Placement, WorldIndex, WorldspaceId, build_directory, build_pages,
};
use sf_world_bevy::{PlacementSpawners, WorldIndexSource};
use mw_assets::SharedVfs;
use mw_assets::cooked::TextureCompanions;
use mw_world::source::ROCK_LIBRARY;
use sf_rockgen::Libraries;
use sf_rockgen_bevy::{RockDebug, RockLodSettings, RockMaterial, RockMaterialSettings, RockSpawner, RockgenPlugin};
use sf_tunables_bevy::Tunables;

use crate::rock_materials::{RockTextureTable, material_set};

/// `worldgen.cloud_base_lift_m`: what a generated world adds to every cloud base (the weather's heights are
/// halved, `CLOUD_WORLD_SCALE`, so the layer sits at ~0.6-1.2 km; a 5.5 km summit needs it near 3 km).
pub const CLOUD_BASE_LIFT_M: Def<f32> = def!(float "worldgen.cloud_base_lift_m" = 2500.0)
    .range(0.0, 8000.0)
    .unit(Unit::Meters)
    .help("A generated world raises every cloud base by this, so its summits can pierce the layer")
    .source(Source::Guess);

/// `worldgen.filter`: the runtime erosion filter on a generated world (live: off shows the guide's bare
/// Catmull-Rom surface, so the owner sees what the filter adds).
pub const WORLDGEN_FILTER: Def<bool> = def!(bool "worldgen.filter" = true)
    .help("Generated world: the runtime erosion filter's gully and ridge detail (off: the guide's smooth surface)")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `worldgen.octaves`: the filter's octaves at runtime (live). 8 from 160 m at lacunarity 2 end at 1.25 m
/// (`sf_worldgen::tiles` `runtime_octaves`); fewer drop the finest.
pub const WORLDGEN_OCTAVES: Def<u32> = def!(count "worldgen.octaves" = 8)
    .range(0, 8)
    .unit(Unit::Count)
    .help("Generated world: erosion filter octaves (each halves the wavelength from 160 m; 8 ends at 1.25 m)")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// `worldgen.sharp`: multiplier on the Phase 3a runtime sharp-landform operators (strata benches, Voronoi
/// rock fracture blocks, ridged crests). Live: 0 = off (default, owner 2026-11-22 prefers the look without them),
/// 1 = the full sharp profile, > 1 = more aggressive cliffs and rocky outcrops.
pub const WORLDGEN_SHARP: Def<f32> = def!(float "worldgen.sharp" = 0.0)
    .range(0.0, 3.0)
    .unit(Unit::Ratio)
    .help("Generated world: runtime sharp-landform scale (strata, Voronoi rock blocks, ridged crests; 0 = off, the default; 1 = full sharp profile)")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

/// Seconds between the tile timing lines while tiles are being generated.
// tunables: const (log cadence: how often a diagnostic line prints, no look, feel or cost)
const REPORT_EVERY_S: u64 = 5;

/// A provider and its cumulative per-level timing summary.
pub type WorldgenTiles = (Arc<dyn TerrainTiles>, Arc<dyn Fn() -> String + Send + Sync>);

/// What the viewer needs to show a generated world.
pub struct WorldgenSetup {
    /// The terrain tiles (for the exterior worldspace).
    pub tiles: Arc<dyn TerrainTiles>,
    /// The start line: `worldgen world: <dir>, <w>x<h> @ <s> m, ...`.
    pub label: String,
    /// Added to every cloud base, meters ([`CLOUD_BASE_LIFT_M`] unless the app chose another).
    pub cloud_base_lift_m: f32,
    /// Per-level tile generation summary, cumulative (logged while it changes).
    pub timings: Arc<dyn Fn() -> String + Send + Sync>,
    /// The filter octaves `tiles` was built with.
    pub octaves: u32,
    /// The same world with the filter at `(octaves, sharp_scale)` (`(0, 0.0)` = bare guide): the live switches.
    pub rebuild: Arc<dyn Fn(u32, f32) -> WorldgenTiles + Send + Sync>,
    /// What streams in the exterior; `None`: nothing (the bare generated land).
    pub content: Option<WorldgenContent>,
    /// The guide's road ribbons (GR-1b, [`crate::worldgen_roads`]); `None`: no roads (`--off roads`).
    pub roads: Option<crate::worldgen_roads::WorldgenRoads>,
}

/// The exterior's placements in a generated world, already at their final places (engine frame).
#[derive(Default)]
pub struct WorldgenContent {
    /// The content's own references, moved (WG-X4: anchor members and what lies inside stamped footprints).
    pub placements: Vec<Placement>,
    /// Extra placements from later passes, each with a name for the log (WG-X5: flora scatter). Indexed with
    /// `placements`; their ids must not collide with them.
    pub extra: Vec<(String, Vec<Placement>)>,
    /// Model infos the bake lacks (RG-P5a: library rocks' `rockgen/<class>/<index>`, the variant's engine-frame
    /// bounds); looked up before the bake's `model.info` ([`content_models`]).
    pub models: HashMap<VfsPath, ModelInfo>,
    /// The rock variant libraries the library rocks name (RG-P5a); with a [`ROCK_LIBRARY`] placement, `add`
    /// registers their [`RockSpawner`].
    pub rock_libraries: Option<Arc<Libraries>>,
}

impl WorldgenContent {
    /// Every placement: the content's, then the extra lists'.
    pub fn all(&self) -> impl Iterator<Item = &Placement> {
        self.placements.iter().chain(self.extra.iter().flat_map(|(_, e)| e))
    }
}

/// The index's model lookup: `content`'s own [`WorldgenContent::models`] first, then the bake's `infos`.
pub fn content_models<'a, S: std::hash::BuildHasher>(
    content: Option<&'a WorldgenContent>,
    infos: &'a HashMap<VfsPath, ModelInfo, S>,
) -> impl Fn(&VfsPath) -> Option<ModelInfo> + 'a {
    move |m| content.and_then(|c| c.models.get(m)).or_else(|| infos.get(m)).copied()
}

/// The content index's log line (`+ <name> <count>` per extra list).
pub fn index_line(c: &WorldgenContent, index: &WorldIndex, with_info: usize, seconds: f32) -> String {
    use std::fmt::Write as _;
    let mut extra = String::new();
    for (name, e) in &c.extra {
        let _ = write!(extra, " + {name} {}", e.len());
    }
    format!(
        "worldgen content index: {} placements ({} content{extra}), {} cells, {} pages, {} models with model.info, \
         {} without (points), in {seconds:.2} s; static scene, precombine and far field off (source-cell products): \
         statics spawn per placement",
        index.stats.placements,
        c.placements.len(),
        index.stats.cells,
        index.stats.pages,
        with_info,
        index.stats.missing_models.len(),
    )
}

/// `p` moved to engine position `pos` (metres, absolute): rotation, scale, class and flags kept.
pub fn placed_at(mut p: Placement, pos: DVec3) -> Placement {
    p.transform.pos = WorldPos::from_f64(pos);
    p
}

/// `model.info` of `models` from the bake, as `sf_world::bake` indexes them (bounds and radii); a model without one
/// is indexed as a point.
pub fn model_infos<'a>(
    store: &Store,
    manifest: &Manifest,
    models: impl IntoIterator<Item = &'a VfsPath>,
) -> HashMap<VfsPath, ModelInfo> {
    let mut out = HashMap::new();
    let mut seen = std::collections::HashSet::new();
    for m in models {
        if !seen.insert(m) {
            continue;
        }
        let info = manifest
            .get(ModelInfoStep::ID, m.as_str())
            .and_then(|d| store.get(&d).ok())
            .and_then(|bytes| ModelInfoStep::decode(&bytes).ok())
            .and_then(|i| sf_world::bake::index_model_info(&i));
        if let Some(info) = info {
            out.insert(m.clone(), info);
        }
    }
    out
}

/// The exterior's in-memory index of `content` (empty without).
pub fn content_index(
    ext: &WorldspaceId,
    content: Option<&WorldgenContent>,
    models: &dyn Fn(&VfsPath) -> Option<ModelInfo>,
) -> WorldIndex {
    let params = IndexParams::default();
    let placements = content.into_iter().flat_map(WorldgenContent::all).cloned();
    let (pages, stats) = build_pages(&params, placements, models);
    let directory = build_directory(ext.clone(), &params, &pages);
    WorldIndex { directory, pages, stats }
}

/// The exterior's directory is empty (nothing streams); every other worldspace reads the bake's.
struct WorldgenIndex {
    own: MemoryIndex,
    base: Arc<dyn IndexReader>,
}

impl IndexReader for WorldgenIndex {
    fn directory(&self, ws: &WorldspaceId) -> Result<Option<Vec<u8>>, String> {
        match self.own.directory(ws)? {
            Some(d) => Ok(Some(d)),
            None => self.base.directory(ws),
        }
    }

    fn page(&self, ws: &WorldspaceId, page: IVec3) -> Result<Option<Vec<u8>>, String> {
        if self.own.directory(ws)?.is_some() {
            return self.own.page(ws, page);
        }
        self.base.page(ws, page)
    }
}

#[derive(Resource)]
struct Timings(Arc<dyn Fn() -> String + Send + Sync>);

/// The live filter values ([`WORLDGEN_FILTER`], [`WORLDGEN_OCTAVES`], [`WORLDGEN_SHARP`]).
#[derive(Resource, Clone, Copy, PartialEq)]
struct LiveFilter {
    on: bool,
    octaves: u32,
    sharp: f32,
}

impl LiveFilter {
    fn effective(self) -> (u32, f32) {
        if self.on { (self.octaves, self.sharp) } else { (0, 0.0) }
    }
}

#[derive(Resource)]
struct Rebuild {
    rebuild: Arc<dyn Fn(u32, f32) -> WorldgenTiles + Send + Sync>,
    /// The `(octaves, sharp)` the shown provider has.
    shown: (u32, f32),
}

/// Replaces the exterior's index with the in-memory one of [`WorldgenSetup::content`] and logs the world; call
/// after `WorldIndexSource` is inserted.
pub(crate) fn add(app: &mut App, ext: &WorldspaceId, setup: &WorldgenSetup, store: &Store, manifest: &Manifest) {
    info!("{}", setup.label);
    let base = app.world().resource::<WorldIndexSource>().0.clone();
    let started = Instant::now();
    let content = setup.content.as_ref();
    let models = content
        .into_iter()
        .flat_map(|c| c.all().filter_map(|p| p.model.as_ref()).filter(|m| !c.models.contains_key(*m)));
    let infos = model_infos(store, manifest, models);
    let index = content_index(ext, content, &content_models(content, &infos));
    if let Some(c) = content {
        let with_info = infos.len() + c.models.len();
        info!("{}", index_line(c, &index, with_info, started.elapsed().as_secs_f32()));
        if !index.stats.duplicate_ids.is_empty() {
            warn!("worldgen content index: {} duplicate placement ids", index.stats.duplicate_ids.len());
        }
        if let Some(libs) = c.rock_libraries.clone().filter(|_| c.all().any(|p| p.class == ROCK_LIBRARY)) {
            if !app.is_plugin_added::<RockgenPlugin>() {
                app.add_plugins(RockgenPlugin);
            }
            app.insert_resource(RockLibrarySource(libs))
                .add_systems(Startup, register_rock_spawner.after(crate::exterior::SpawnersSetup));
        }
    }
    let mut own = MemoryIndex::new();
    own.insert(&index);
    let sharp0 = WORLDGEN_SHARP.default_value();
    app.insert_resource(WorldIndexSource(Arc::new(WorldgenIndex { own, base })))
        .insert_resource(Timings(setup.timings.clone()))
        .insert_resource(LiveFilter { on: setup.octaves > 0, octaves: setup.octaves.max(1), sharp: sharp0 })
        .insert_resource(Rebuild { rebuild: setup.rebuild.clone(), shown: (setup.octaves, sharp0) })
        .add_systems(Update, apply_live_filter)
        .add_systems(Last, report_timings);
    let _on = app.add_tunable(&WORLDGEN_FILTER).bind_resource(|f: &mut LiveFilter, v| f.on = v);
    let _octaves = app.add_tunable(&WORLDGEN_OCTAVES).bind_resource(|f: &mut LiveFilter, v| f.octaves = v);
    let _sharp = app.add_tunable(&WORLDGEN_SHARP).bind_resource(|f: &mut LiveFilter, v| f.sharp = v);
    crate::worldgen_roads::add(app, ext, setup.roads.as_ref());
}

/// The libraries [`register_rock_spawner`] builds the library rocks' spawner from.
#[derive(Resource)]
struct RockLibrarySource(Arc<Libraries>);

/// Startup, after the exterior's spawners and before the `PostStartup` hooks that wrap them: the library rocks'
/// [`RockSpawner`] for [`ROCK_LIBRARY`] (RG-P5a): every variant's LOD meshes once, the materials of
/// `rock_textures.toml` ([`crate::rock_materials`]) at the live `rockgen.material.*` / `rockgen.lod.*` values.
#[allow(clippy::too_many_arguments)] // a system's parameters
fn register_rock_spawner(
    source: Res<RockLibrarySource>,
    spawners: Option<ResMut<PlacementSpawners>>,
    vfs: Option<Res<SharedVfs>>,
    server: Res<AssetServer>,
    tunables: Option<Res<Tunables>>,
    debug: Res<RockDebug>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RockMaterial>>,
) {
    let (Some(mut spawners), Some(vfs)) = (spawners, vfs) else {
        warn!("worldgen rocks library: no placement spawners or VFS; library rocks not drawn");
        return;
    };
    let libs = &source.0;
    let (settings, lod) = tunables.map_or_else(
        || (RockMaterialSettings::default(), RockLodSettings::default()),
        |t| (RockMaterialSettings::from_tunables(&t.0), RockLodSettings::from_tunables(&t.0)),
    );
    let levels = libs.classes().filter_map(|c| libs.get(c)).flat_map(|l| &l.variants).map(|v| v.lods.lods.len()).max();
    let levels = u32::try_from(levels.unwrap_or(1)).unwrap_or(u32::MAX);
    let companions = TextureCompanions::read(&vfs.0);
    let table = RockTextureTable::shipped();
    let set = material_set(&table, &vfs.0, &companions, &server, &settings, levels, debug.lod_tint, &mut materials);
    let spawner = RockSpawner::new(libs, &mut meshes, &set, lod);
    info!(
        "worldgen rocks library: {} classes, {} meshes ({levels} LOD levels), default texture {}",
        libs.len(),
        spawner.mesh_count(),
        table.default
    );
    spawners.set(ROCK_LIBRARY, Arc::new(spawner));
}

/// A live filter change rebuilds the provider (the guide is shared) and swaps the terrain world: the clipmaps
/// and the CPU twin refill from it.
fn apply_live_filter(mut commands: Commands, live: Res<LiveFilter>, mut rebuild: ResMut<Rebuild>) {
    let want = live.effective();
    if !live.is_changed() || want == rebuild.shown {
        return;
    }
    let (tiles, timings) = (rebuild.rebuild)(want.0, want.1);
    info!(
        "worldgen filter: {} (live), the terrain refills",
        if want.0 == 0 && want.1 <= 0.0 {
            "off".to_owned()
        } else {
            format!("{} octaves, sharp {:.2}", want.0, want.1)
        }
    );
    commands.insert_resource(TerrainWorld::new(tiles));
    commands.insert_resource(Timings(timings));
    rebuild.shown = want;
}

/// Logs the CPU tile summary (the CPU twin's on a GPU-filled world) and the GPU fill counts every few seconds
/// while they change.
fn report_timings(
    time: Res<Time<Real>>,
    timings: Res<Timings>,
    stats: Option<Res<TerrainStats>>,
    mut last: Local<(Duration, String, u64)>,
) {
    let now = time.elapsed();
    if now.saturating_sub(last.0) < Duration::from_secs(REPORT_EVERY_S) {
        return;
    }
    last.0 = now;
    let line = (timings.0)();
    if !line.is_empty() && line != last.1 {
        let what = if stats.as_ref().is_some_and(|s| s.gpu_fill) { "CPU twin tiles" } else { "tiles" };
        info!("worldgen {what}: {line}");
        last.1 = line;
    }
    if let Some(s) = stats.filter(|s| s.gpu_fill && s.gpu_frames != last.2) {
        #[allow(clippy::cast_precision_loss)]
        let mtex = s.gpu_texels as f64 / 1e6;
        info!("worldgen gpu fill: {} pieces, {mtex:.2} M texels in {} frames so far (GPU time: render/terrain_gpu_fill_*)", s.gpu_pieces, s.gpu_frames);
        last.2 = s.gpu_frames;
    }
}
