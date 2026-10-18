//! Water wiring (M6 W2; water-adapter.md "What replaces `SeaPlaneScaffold`"): the exterior's ocean
//! from `mw_water::MwWaterSource`, the CPU water query (`sf_water::WaterWorld`, stepped in
//! `FixedUpdate` before physics, bed = the resident terrain heights) and the GPU ocean
//! (`sf_water_bevy::WaterPlugin`) on the same seed, cascades and clock. `--water scaffold` keeps
//! the old flat sea (`SeaPlaneScaffold`) for A/B until WA-1 passes; `--water off` draws none.
//!
//! W3 (water-adapter.md "Openness and fetch on one sea plane", "Optics by region", "Interiors"):
//! the `water.shore` product baked at startup on a worker thread from the `LAND` wet mask (CPU
//! query and GPU waves calm toward shores and in enclosed water), the region optics map, interior
//! water as a `WaterSurface` pool at the cell's `WHGT`, and the analytic height fog in front of the
//! water when the froxels are off. The FFT tier follows `--quality`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use bevy::camera::visibility::RenderLayers;
use bevy::light::{AmbientLight, GlobalAmbientLight};
use bevy::prelude::*;
use mw_water::{optics, optics_records, MwWaterSource, WaterIni};
use mw_world::ids;
use sf_records::RecordDb;
use sf_sky_bevy::SunLight;
use sf_terrain_bevy::{TerrainHeightsMap, TerrainOrigin, TerrainSystems};
use sf_water::{BedHeights, Cascades, OpticsMap, ShoreField, WaterSource, WaterWorld, WaveTwin, WetMask, WorldspaceWater, TWIN_RESOLUTION};
use sf_water_bevy::{
    OceanSpectrum, OceanSurface, WaterCamera, WaterFogInput, WaterFrame, WaterLook, WaterPlugin, WaterQuality, WaterRegionsInput, WaterShoreInput,
    WaterSurface, WaterSystems,
};
use sf_world::{Worldspace, WorldspaceId};
use sf_world_bevy::{ActiveWorldspace, WorldspaceRegistry};
use tes3::esp::{Cell, Landscape};

use crate::exterior::{ExteriorCamera, Quality};

/// `--water`: which sea the exterior draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub(crate) enum WaterMode {
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
pub(crate) fn set_mode(mode: WaterMode, wind: Option<f32>) {
    MODE.store(mode as u8, Ordering::Relaxed);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    WIND.store(wind.map_or(0, |w| (w.clamp(0.0, 60.0) * 1000.0) as u32), Ordering::Relaxed);
}

pub(crate) fn mode() -> WaterMode {
    match MODE.load(Ordering::Relaxed) {
        1 => WaterMode::Scaffold,
        2 => WaterMode::Off,
        _ => WaterMode::On,
    }
}

/// Scaffold `WindInput` until `sf_wind`'s `WindState` (WD1): a steady sea breeze, m/s, blowing
/// toward the north-east (Morrowind has no wind data; WE2 maps weathers to wind later).
const DEFAULT_WIND: f32 = 7.0;
/// One seed for the GPU ocean and the CPU twin (the twin is the GPU's cascade 0 by construction).
const SEED: u32 = 0x5EA0_0001;

/// The CPU water query and its clock.
#[derive(Resource)]
pub(crate) struct Water {
    pub world: WaterWorld,
    pub time: f64,
}

/// What the water needs from the records, gathered while the `RecordDb` is at hand (main.rs).
#[derive(Default)]
pub(crate) struct WaterInputs {
    /// The exterior's wet mask at sea level (the shore bake's input).
    mask: Option<WetMask>,
    /// Optics record index by exterior cell.
    regions: Option<OpticsMap>,
}

impl WaterInputs {
    /// The wet mask from every `LAND` and the region map from the exterior cells (~0.1 s; nothing
    /// unless the FFT ocean is on).
    pub(crate) fn build(db: &RecordDb) -> Self {
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

pub(crate) fn add(app: &mut App, inputs: WaterInputs, quality: Quality, worldspaces: &std::collections::BTreeMap<WorldspaceId, Worldspace>) {
    if mode() != WaterMode::On {
        return;
    }
    let wind = match WIND.load(Ordering::Relaxed) {
        0 => DEFAULT_WIND,
        #[allow(clippy::cast_precision_loss)]
        mm => mm as f32 / 1000.0,
    };
    let exterior = ids::exterior();
    let source = MwWaterSource;
    let mut params = source.ocean_spectrum(&exterior);
    params.wind.speed = wind;
    params.wind.toward = Vec2::new(0.6, 0.8);
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
    let bake = inputs.mask.map(|mask| {
        let handle = std::thread::Builder::new()
            .name("water-shore-bake".into())
            .spawn(move || ShoreField::bake(&mask, mw_water::shore_settings()))
            .expect("spawn the shore bake thread");
        (handle, Instant::now())
    });
    app.add_plugins(WaterPlugin)
        .insert_resource(OceanSpectrum { params, cascades, seed: SEED })
        .insert_resource(look)
        .insert_resource(water_quality(quality))
        .insert_resource(WaterRegionsInput { map: inputs.regions.map(Arc::new), looks })
        .insert_resource(ShoreBake(bake))
        .insert_resource(Pools { levels, look: look_of(optics::INTERIOR), current: None })
        .insert_resource(Water { world, time: 0.0 })
        .add_systems(FixedUpdate, step.before(sf_physics_bevy::PhysicsSystems::Step))
        .add_systems(Update, (spawn_ocean, mark_camera, bed, shore_done, pools))
        .add_systems(PostUpdate, (sync, fog_input).after(TerrainSystems::Update).before(WaterSystems::Drive));
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

/// The query's state for this physics step (water.md "The water query": same 60 Hz step).
fn step(time: Res<Time<Fixed>>, mut water: ResMut<Water>) {
    let t = time.elapsed_secs_f64();
    water.time = t;
    water.world.step(t);
}

fn spawn_ocean(mut commands: Commands, registry: Res<WorldspaceRegistry>, mut done: Local<bool>) {
    if *done {
        return;
    }
    let Some(entry) = registry.get(&ids::exterior()) else { return };
    commands.spawn((OceanSurface, Transform::IDENTITY, Visibility::Visible, RenderLayers::layer(entry.layer)));
    *done = true;
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

/// The fog in front of the water: the froxel volume, and the analytic height fog layer (used when
/// the froxels are off, `--off froxels`). Its in-scattering is the CPU closed form of the fog
/// composite's: albedo × (sun / 4π (isotropic) + sky fill × ambient), the sky fill taken from the
/// camera's ambient light. Exterior only (interiors keep their `AMBI` fog).
#[allow(clippy::type_complexity)]
fn fog_input(
    (origin, active): (Res<TerrainOrigin>, Res<ActiveWorldspace>),
    (volume, atmos): (Option<Res<sf_atmos_bevy::FroxelVolume>>, Option<Res<sf_atmos_bevy::Fog>>),
    sun: Query<&DirectionalLight, With<SunLight>>,
    ambient: (Query<&AmbientLight, With<ExteriorCamera>>, Res<GlobalAmbientLight>),
    mut fog: ResMut<WaterFogInput>,
) {
    let mut want = volume.map_or_else(WaterFogInput::default, |v| WaterFogInput { froxels: Some(v.image.clone()), params: v.params, ..WaterFogInput::default() });
    let in_exterior = active.0.as_ref().is_none_or(|ws| *ws == ids::exterior());
    if let Some(a) = atmos.filter(|a| a.enabled && in_exterior && a.settings.height.density > 0.0) {
        let layer = a.settings.height;
        #[allow(clippy::cast_precision_loss)]
        let sea = -(origin.0.cell.y as f32) * sf_coords::GRID_CELL;
        let key: Vec3 = sun.iter().map(|l| l.color.to_linear().to_vec3() * l.illuminance).sum::<Vec3>() / (4.0 * std::f32::consts::PI);
        let (color, brightness) = ambient.0.iter().next().map_or((ambient.1.color, ambient.1.brightness), |l| (l.color, l.brightness));
        let fill = color.to_linear().to_vec3() * brightness * a.settings.ambient;
        want.height = Vec4::new(layer.density, layer.falloff, layer.base + sea, 1.0);
        want.height_color = a.settings.albedo * (key + fill);
    }
    if *fog != want {
        *fog = want;
    }
}
