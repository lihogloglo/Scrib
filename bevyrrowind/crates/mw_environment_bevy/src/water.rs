//! Water wiring (M6 W2; water-adapter.md "What replaces `SeaPlaneScaffold`"): the exterior's ocean
//! from `mw_water::MwWaterSource`, the CPU water query (`sf_water::WaterWorld`, stepped in
//! `FixedUpdate` before physics, bed = the resident terrain heights) and the GPU ocean
//! (`sf_water_bevy::WaterPlugin`) on the same seed, cascades and clock. `--water scaffold` keeps
//! the old flat sea (`SeaPlaneScaffold`) for A/B until WA-1 passes; `--water off` draws none.
//!
//! W3 (water-adapter.md "Openness and fetch on one sea plane", "Optics by region", "Interiors"):
//! the `water.shore` product baked at startup on a worker thread from the `LAND` wet mask (CPU
//! query and GPU waves calm toward shores and in enclosed water), the region optics map, interior
//! water as a `WaterSurface` pool at the cell's `WHGT`. The water has no fog of its own: the sky,
//! clouds and fog cover it after its draw (W-ORDER). The FFT tier follows `--quality`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use bevy_app::{App, FixedUpdate, PostUpdate, Update};
use bevy_camera::visibility::{RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_log::{error, info};
use bevy_math::Vec2;
use bevy_time::{Fixed, Time, Virtual};
use bevy_transform::components::Transform;
use mw_water::{optics, optics_records, MwWaterSource, WaterIni};
use mw_world::ids;
use sf_records::RecordDb;
use sf_terrain_bevy::{TerrainHeightsMap, TerrainOrigin, TerrainSystems};
use sf_water::{BedHeights, Cascades, OpticsMap, SeaWind, SeaWindSettings, ShoreField, ShoreSettings, ShoreWaves, WaterSource, WaterWorld, WaveTwin, WetMask, WindInput, WorldspaceWater, TWIN_RESOLUTION};
use sf_water_bevy::{
    OceanSpectrum, OceanSurface, WaterCamera, WaterFrame, WaterLook, WaterPlugin, WaterQuality, WaterReflections, WaterRegionsInput, WaterShoreInput,
    WaterSurface, WaterSystems,
};
use sf_world::{Worldspace, WorldspaceId};
use sf_world_bevy::{ActiveWorldspace, WorldspaceRegistry};
use tes3::esp::{Cell, Landscape};

use mw_game_core::exterior::{ExteriorCamera, Quality};
/// The sea's wind input (W-WIND): the exterior copies the weather's `WindField` mean wind into it.
pub use sf_water_bevy::WaterWind;

/// `--water`: which sea the exterior draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum WaterMode {
    /// The FFT ocean (W2).
    #[default]
    On,
    /// The flat `SeaPlaneScaffold` (A/B until WA-1 passes).
    Scaffold,
    /// No sea.
    Off,
}

static MODE: AtomicU8 = AtomicU8::new(0);
/// `--water-wind` in mm/s (0 = the default).
static WIND: AtomicU32 = AtomicU32::new(0);

/// Set from the command line before the exterior app is built.
pub fn set_mode(mode: WaterMode, wind: Option<f32>) {
    MODE.store(mode as u8, Ordering::Relaxed);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    WIND.store(wind.map_or(0, |w| (w.clamp(0.0, 60.0) * 1000.0) as u32), Ordering::Relaxed);
}

pub fn mode() -> WaterMode {
    match MODE.load(Ordering::Relaxed) {
        1 => WaterMode::Scaffold,
        2 => WaterMode::Off,
        _ => WaterMode::On,
    }
}

/// The sea's wind before the weather's first frame, and always with `--off seawind` (W-WIND): a steady
/// sea breeze, m/s, blowing toward the north-east.
const DEFAULT_WIND: f32 = 7.0;
/// One seed for the GPU ocean and the CPU twin (the twin is the GPU's cascade 0 by construction).
const SEED: u32 = 0x5EA0_0001;

/// The CPU water query and its clock.
#[derive(Resource)]
pub struct Water {
    pub world: WaterWorld,
    pub time: f64,
}

/// What the water needs from the records, gathered while the `RecordDb` is at hand (main.rs).
#[derive(Default)]
pub struct WaterInputs {
    /// The exterior's wet mask at sea level (the shore bake's input).
    mask: Option<WetMask>,
    /// Optics record index by exterior cell.
    regions: Option<OpticsMap>,
}

impl WaterInputs {
    /// The wet mask from every `LAND` and the region map from the exterior cells (~0.1 s; nothing
    /// unless the FFT ocean is on).
    pub fn build(db: &RecordDb) -> Self {
        if mode() != WaterMode::On {
            return Self::default();
        }
        let t = Instant::now();
        let lands: Vec<((i32, i32), Vec<f32>)> = db.iter::<Landscape>().map(|(_, l)| (l.grid, mw_world::terrain::LandData::decode(l).heights)).collect();
        let regions: HashMap<(i32, i32), String> =
            db.iter::<Cell>().filter(|(_, c)| !c.is_interior()).filter_map(|(_, c)| Some((c.data.grid, c.region.clone()?))).collect();
        let Some(bounds) = mw_water::cell_bounds(lands.iter().map(|(g, _)| *g).chain(regions.keys().copied())) else {
            return Self::default();
        };
        let mask = mw_water::wet_mask(bounds, lands.iter().map(|(g, h)| (*g, h.as_slice())));
        let map = mw_water::optics_map(bounds, |x, y| regions.get(&(x, y)).cloned());
        info!("water: wet mask {}x{} ({} LAND cells), regions {}x{} in {:.2} s", mask.width, mask.height, lands.len(), map.width, map.height, t.elapsed().as_secs_f32());
        Self { mask: Some(mask), regions: Some(map) }
    }
}

/// The shore bake running on a worker thread (water-adapter.md: ~1 s, gate ≤ 10 s cold).
#[derive(Resource)]
struct ShoreBake(Option<(JoinHandle<ShoreField>, Instant)>);

/// Interior water levels (engine Y, metres) by worldspace (`HAS_WATER` + `WHGT`).
#[derive(Resource)]
struct Pools {
    levels: HashMap<WorldspaceId, f32>,
    look: WaterLook,
    /// The pool entity of the active interior.
    current: Option<(WorldspaceId, Entity)>,
}

/// The FFT tier of a `--quality` preset (water.md "Performance budgets").
fn water_quality(q: Quality) -> WaterQuality {
    match q {
        Quality::Low => WaterQuality::Low,
        Quality::Medium => WaterQuality::Medium,
        Quality::High => WaterQuality::High,
        Quality::Ultra => WaterQuality::Ultra,
    }
}

/// `sspr`: local reflections on the water (W-SSPR; `--off waterssr` = the environment map only).
/// `shore_normals`: wind wavelets in the shading normal up to the shoreline (W-SHORE; `--off
/// shorenormals` = the normal flattens with the displacement, `ShoreSettings::normal_floor` 0).
/// `shore_waves`: analytic Gerstner waves and swash in the shore band (W-GERSTNER; `--off shorewaves`
/// = the FFT alone, fading to flat at the shore, `ShoreSettings::shore_waves` 0).
/// `sea_wind`: the sea follows the weather's wind (W-WIND, `sf_water::SeaWind`; `--off seawind` =
/// the fixed [`DEFAULT_WIND`]); `--water-wind` overrides both.
pub fn add(
    app: &mut App,
    inputs: WaterInputs,
    quality: Quality,
    (sspr, shore_normals, shore_waves, sea_wind): (bool, bool, bool, bool),
    worldspaces: &std::collections::BTreeMap<WorldspaceId, Worldspace>,
) {
    if mode() != WaterMode::On {
        return;
    }
    let fixed = match WIND.load(Ordering::Relaxed) {
        0 => None,
        #[allow(clippy::cast_precision_loss)]
        mm => Some(mm as f32 / 1000.0),
    };
    let exterior = ids::exterior();
    let source = MwWaterSource;
    let mut params = source.ocean_spectrum(&exterior);
    params.wind = WindInput { speed: fixed.unwrap_or(DEFAULT_WIND), toward: Vec2::new(0.6, 0.8) };
    // W-WIND: the weather's 10 m wind drives the sea (the exterior copies `WindField` into
    // `WaterWind::weather`); the command-line wind pins it.
    let sea = sea_wind.then(|| SeaWind::new(params.wind, SeaWindSettings::default()).with_fixed(fixed.map(|_| params.wind)));
    let cascades = Cascades::default();
    let mut ws = Worldspace::new("exterior");
    ws.has_terrain = true;
    ws.sea_level = Some(0.0);
    let mut bodies = Vec::new();
    source.bodies(&exterior, &ws, &mut bodies);
    let mut water = WorldspaceWater::new(bodies);
    water.set_waves(Some(WaveTwin::new(&params, &cascades, SEED, 1, TWIN_RESOLUTION)));
    let mut world = WaterWorld::new();
    world.insert(exterior.clone(), water);
    // Optics table in `mw_water::optics_index` order (sea, coastal, swamp, ash, north, interior).
    let records = optics_records(&WaterIni::default());
    let look_of = |id: &str| records.iter().find(|(r, _)| *r == id).map_or_else(WaterLook::default, |(_, o)| WaterLook::from_optics(o));
    let look = look_of(optics::SEA);
    let looks: Vec<WaterLook> = records.iter().map(|(_, o)| WaterLook::from_optics(o)).collect();
    let levels: HashMap<WorldspaceId, f32> =
        worldspaces.iter().filter(|(id, ws)| !ws.has_terrain && **id != exterior).filter_map(|(id, ws)| Some((id.clone(), ws.sea_level?))).collect();
    let mut settings = mw_water::shore_settings();
    if !shore_normals {
        settings.normal_floor = 0.0;
    }
    if !shore_waves {
        settings.shore_waves = 0.0;
    }
    // The CPU query lifts its surface with the same shore waves as the GPU (W-GERSTNER).
    let shore_pair = ShoreWaves::new(&params, &cascades, &settings);
    if let Some(ws) = world.get_mut(&exterior) {
        ws.set_shore_waves(Some(shore_pair));
    }
    let spectrum = OceanSpectrum { params, cascades: cascades.clone(), seed: SEED, transition: None };
    let sea_sync = SeaSync { spectrum: spectrum.clone(), settings, shore: [shore_pair; 2] };
    let bake = inputs.mask.map(|mask| {
        let handle = std::thread::Builder::new()
            .name("water-shore-bake".into())
            .spawn(move || ShoreField::bake(&mask, settings))
            .expect("spawn the shore bake thread");
        (handle, Instant::now())
    });
    app.add_plugins(WaterPlugin)
        .insert_resource(spectrum)
        .insert_resource(WaterWind { weather: None, sea })
        .insert_resource(sea_sync)
        .insert_resource(look)
        .insert_resource(water_quality(quality))
        .insert_resource(WaterReflections { sspr, ..WaterReflections::default() })
        .insert_resource(WaterRegionsInput { map: inputs.regions.map(Arc::new), looks })
        .insert_resource(ShoreBake(bake))
        .insert_resource(Pools { levels, look: look_of(optics::INTERIOR), current: None })
        .insert_resource(Water { world, time: 0.0 })
        .add_systems(FixedUpdate, step.before(sf_physics_bevy::PhysicsSystems::Step))
        .add_systems(Update, (spawn_ocean, mark_camera, bed, shore_done, pools))
        .add_systems(PostUpdate, sync.after(TerrainSystems::Update).before(WaterSystems::Drive));
}

/// Hands the finished shore bake to the GPU waves and the CPU query.
fn shore_done(mut bake: ResMut<ShoreBake>, mut water: ResMut<Water>, mut shore: ResMut<WaterShoreInput>) {
    if !bake.0.as_ref().is_some_and(|(h, _)| h.is_finished()) {
        return;
    }
    let Some((handle, started)) = bake.0.take() else { return };
    let Ok(field) = handle.join() else {
        error!("water: the shore bake panicked; waves stay unattenuated");
        return;
    };
    let s = field.stats;
    info!(
        "water: shore field {}x{} at {:.2} m in {:.2} s ({} wet texels, {} components, {} open)",
        field.width,
        field.height,
        field.texel,
        started.elapsed().as_secs_f32(),
        s.wet_texels,
        s.components,
        s.open_components
    );
    let field = Arc::new(field);
    if let Some(ws) = water.world.get_mut(&ids::exterior()) {
        ws.set_shore(Some(field.clone()));
    }
    shore.field = Some(field);
}

/// The active interior's water (water-adapter.md "Interiors"): a still pool at the cell's level in
/// the interior's render layers, replaced on every worldspace change. The level follows the
/// floating origin in [`sync`].
fn pools(mut commands: Commands, active: Res<ActiveWorldspace>, registry: Res<WorldspaceRegistry>, mut pools: ResMut<Pools>) {
    let want = active.0.as_ref().filter(|ws| pools.levels.contains_key(*ws));
    if pools.current.as_ref().map(|(ws, _)| ws) == want {
        return;
    }
    if let Some((_, e)) = pools.current.take() {
        commands.entity(e).despawn();
    }
    let Some(ws) = want.cloned() else { return };
    let Some(entry) = registry.get(&ws) else { return };
    let e = commands
        .spawn((WaterSurface::pool(pools.levels[&ws], pools.look), Transform::IDENTITY, Visibility::Visible, RenderLayers::layer(entry.layer)))
        .id();
    info!("water: interior pool in {} at {:.2} m", ws.as_str(), pools.levels[&ws]);
    pools.current = Some((ws, e));
}

/// The CPU query's copy of the GPU sea state (W-WIND): rebuilt when [`OceanSpectrum`] changes,
/// blended at the twin's own time like the GPU.
#[derive(Resource)]
struct SeaSync {
    spectrum: OceanSpectrum,
    settings: ShoreSettings,
    /// Shore waves of the transition's `from` and of the sea state.
    shore: [ShoreWaves; 2],
}

/// The query's state for this physics step (water.md "The water query": same 60 Hz step). The
/// twin and the shore waves follow the GPU's sea state and transition (W-WIND).
fn step(time: Res<Time<Fixed>>, spectrum: Res<OceanSpectrum>, shore: Res<WaterShoreInput>, mut sync: ResMut<SeaSync>, mut water: ResMut<Water>) {
    let t = time.elapsed_secs_f64();
    let exterior = ids::exterior();
    let settings = shore.field.as_ref().map_or(sync.settings, |f| f.settings);
    if sync.spectrum != *spectrum || sync.settings != settings {
        let s = &*spectrum;
        let from = s.from_params().unwrap_or(s.params);
        let to = ShoreWaves::new(&s.params, &s.cascades, &settings);
        sync.shore = [s.from_params().map_or(to, |p| ShoreWaves::new(&p, &s.cascades, &settings)), to];
        sync.spectrum = s.clone();
        sync.settings = settings;
        if let Some(ws) = water.world.get_mut(&exterior) {
            ws.set_waves(Some(WaveTwin::blended(&from, &s.params, &s.cascades, s.seed, 1, TWIN_RESOLUTION)));
        }
    }
    let w = spectrum.weight(t);
    let [from, to] = sync.shore;
    if let Some(ws) = water.world.get_mut(&exterior) {
        if let Some(twin) = ws.waves_mut() {
            twin.set_mix(w);
        }
        ws.set_shore_waves(Some(ShoreWaves::blend(&from, &to, w)));
    }
    water.time = t;
    water.world.step(t);
}

/// The ocean once the exterior's root exists; again for each new world ([`despawn_world`]).
fn spawn_ocean(mut commands: Commands, registry: Res<WorldspaceRegistry>, oceans: Query<(), With<OceanSurface>>) {
    if !oceans.is_empty() {
        return;
    }
    let Some(entry) = registry.get(&ids::exterior()) else { return };
    commands.spawn((OceanSurface, Transform::IDENTITY, Visibility::Visible, RenderLayers::layer(entry.layer)));
}

/// Leaving the world (menus.md "App phases", F2a): the ocean and the interior pool go (they are
/// free-standing, on the old registry's layers); the next world spawns its own.
pub fn despawn_world(world: &mut World) {
    let surfaces: Vec<Entity> =
        world.query_filtered::<Entity, Or<(With<OceanSurface>, With<WaterSurface>)>>().iter(world).collect();
    for e in surfaces {
        world.despawn(e);
    }
    if let Some(mut pools) = world.get_resource_mut::<Pools>() {
        pools.current = None;
    }
}

fn mark_camera(mut commands: Commands, cameras: Query<Entity, (With<ExteriorCamera>, Without<WaterCamera>)>) {
    for e in &cameras {
        commands.entity(e).insert(WaterCamera);
    }
}

/// Bed = the resident terrain heights (refreshed at most once a second while streaming).
fn bed(heights: Option<Res<TerrainHeightsMap>>, mut water: ResMut<Water>, time: Res<Time>, mut last: Local<Option<Duration>>) {
    let Some(heights) = heights else { return };
    if !heights.is_changed() || last.is_some_and(|l| time.elapsed() < l + Duration::from_secs(1)) {
        return;
    }
    *last = Some(time.elapsed());
    let exterior = ids::exterior();
    let Some(h) = heights.0.get(&exterior) else { return };
    let bed: Arc<dyn BedHeights> = Arc::new(h.clone());
    if let Some(ws) = water.world.get_mut(&exterior) {
        ws.set_bed(Some(bed));
    }
}

/// The GPU ocean's frame inputs: the same clock as the twin (virtual time; the fixed steps follow
/// it), the floating origin's cell (as `sync_origin` set it this frame, so the waves' phase moves
/// with the origin in the same frame), the sea level in render space, and the interior pool's
/// level in render space.
fn sync(
    time: Res<Time<Virtual>>,
    origin: Res<TerrainOrigin>,
    active: Res<ActiveWorldspace>,
    pools: Res<Pools>,
    mut surfaces: Query<&mut WaterSurface>,
    mut frame: ResMut<WaterFrame>,
) {
    let cell = origin.0.cell;
    let exterior = ids::exterior();
    let in_exterior = active.0.as_ref().is_none_or(|ws| *ws == exterior);
    #[allow(clippy::cast_precision_loss)]
    let base = -(cell.y as f32) * sf_coords::GRID_CELL;
    let next = WaterFrame { enabled: in_exterior, time: time.elapsed_secs_f64(), origin_cell: cell, level: base };
    if *frame != next {
        *frame = next;
    }
    if let Some((ws, e)) = &pools.current
        && let Ok(mut surface) = surfaces.get_mut(*e)
    {
        let level = pools.levels[ws] + base;
        if (surface.level - level).abs() > f32::EPSILON {
            surface.level = level;
        }
    }
}

