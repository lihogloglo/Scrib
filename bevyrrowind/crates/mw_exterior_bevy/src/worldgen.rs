//! A generated world as the exterior (worldgen slice G1, `view world:<guide dir>`): the terrain comes from a
//! [`TerrainTiles`] provider that generates tiles on demand (the app builds it, `sf_worldgen::tiles`) instead of
//! the bake store, and the exterior's streaming index is empty, so no cell, static or actor of the loaded content
//! streams in. Sky, atmosphere, clouds, weather, water and the camera stay the exterior's. The cloud layer rises
//! by [`CLOUD_BASE_LIFT_M`] so high summits can pierce it (`WeatherOverrides::cloud_base_lift_m`).
//!
//! Slice G2: the provider's GPU field fills the clipmaps (`sf_terrain_bevy` gpu fill; `--off gputerrain` goes back
//! to CPU tiles), and the erosion filter is live: the console's `worldgen.filter false` / `worldgen.octaves <n>`
//! (or `--set`) rebuild the provider with the new filter and the terrain refills ([`WORLDGEN_FILTER`],
//! [`WORLDGEN_OCTAVES`]).

use std::sync::Arc;
use std::time::Duration;

use bevy::math::IVec3;
use bevy::prelude::*;
use sf_terrain_bevy::sf_terrain::TerrainTiles;
use sf_terrain_bevy::{TerrainStats, TerrainWorld};
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, Source, Unit, def};
use sf_world::{IndexParams, IndexReader, MemoryIndex, WorldIndex, WorldspaceId, build_directory, build_pages};
use sf_world_bevy::WorldIndexSource;

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
    /// The same world with the filter at `n` octaves (0 = off): the live filter switch.
    pub rebuild: Arc<dyn Fn(u32) -> WorldgenTiles + Send + Sync>,
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

/// The live filter values ([`WORLDGEN_FILTER`], [`WORLDGEN_OCTAVES`]).
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
struct LiveFilter {
    on: bool,
    octaves: u32,
}

impl LiveFilter {
    fn octaves(self) -> u32 {
        if self.on { self.octaves } else { 0 }
    }
}

#[derive(Resource)]
struct Rebuild {
    rebuild: Arc<dyn Fn(u32) -> WorldgenTiles + Send + Sync>,
    /// The octaves the shown provider has.
    shown: u32,
}

/// Replaces the exterior's index with an empty one and logs the world; call after `WorldIndexSource` is
/// inserted.
pub(crate) fn add(app: &mut App, ext: &WorldspaceId, setup: &WorldgenSetup) {
    info!("{}", setup.label);
    let base = app.world().resource::<WorldIndexSource>().0.clone();
    let params = IndexParams::default();
    let (pages, stats) = build_pages(&params, Vec::new(), &|_| None);
    let directory = build_directory(ext.clone(), &params, &pages);
    let mut own = MemoryIndex::new();
    own.insert(&WorldIndex { directory, pages, stats });
    app.insert_resource(WorldIndexSource(Arc::new(WorldgenIndex { own, base })))
        .insert_resource(Timings(setup.timings.clone()))
        .insert_resource(LiveFilter { on: setup.octaves > 0, octaves: setup.octaves.max(1) })
        .insert_resource(Rebuild { rebuild: setup.rebuild.clone(), shown: setup.octaves })
        .add_systems(Update, apply_live_filter)
        .add_systems(Last, report_timings);
    let _on = app.add_tunable(&WORLDGEN_FILTER).bind_resource(|f: &mut LiveFilter, v| f.on = v);
    let _octaves = app.add_tunable(&WORLDGEN_OCTAVES).bind_resource(|f: &mut LiveFilter, v| f.octaves = v);
}

/// A live filter change rebuilds the provider (the guide is shared) and swaps the terrain world: the clipmaps
/// and the CPU twin refill from it.
fn apply_live_filter(mut commands: Commands, live: Res<LiveFilter>, mut rebuild: ResMut<Rebuild>) {
    let want = live.octaves();
    if !live.is_changed() || want == rebuild.shown {
        return;
    }
    let (tiles, timings) = (rebuild.rebuild)(want);
    info!("worldgen filter: {} (live), the terrain refills", if want == 0 { "off".to_owned() } else { format!("{want} octaves") });
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
