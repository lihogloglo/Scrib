//! Water wiring (M6 W2; water-adapter.md "What replaces `SeaPlaneScaffold`"): the exterior's ocean
//! from `mw_water::MwWaterSource`, the CPU water query (`sf_water::WaterWorld`, stepped in
//! `FixedUpdate` before physics, bed = the resident terrain heights) and the GPU ocean
//! (`sf_water_bevy::WaterPlugin`) on the same seed, cascades and clock. `--water scaffold` keeps
//! the old flat sea (`SeaPlaneScaffold`) for A/B until WA-1 passes; `--water off` draws none.

use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use mw_water::{optics, optics_records, MwWaterSource, WaterIni};
use mw_world::ids;
use sf_terrain_bevy::{TerrainHeightsMap, TerrainOrigin, TerrainSystems};
use sf_water::{BedHeights, Cascades, WaterSource, WaterWorld, WaveTwin, WorldspaceWater, TWIN_RESOLUTION};
use sf_water_bevy::{OceanSpectrum, OceanSurface, WaterCamera, WaterFogInput, WaterFrame, WaterLook, WaterPlugin, WaterSystems};
use sf_world::Worldspace;
use sf_world_bevy::{ActiveWorldspace, WorldspaceRegistry};

use crate::exterior::ExteriorCamera;

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

pub(crate) fn add(app: &mut App) {
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
    world.insert(exterior, water);
    let look = optics_records(&WaterIni::default())
        .into_iter()
        .find(|(id, _)| *id == optics::SEA)
        .map_or_else(WaterLook::default, |(_, o)| WaterLook::from_optics(&o));
    app.add_plugins(WaterPlugin)
        .insert_resource(OceanSpectrum { params, cascades, seed: SEED })
        .insert_resource(look)
        .insert_resource(Water { world, time: 0.0 })
        .add_systems(FixedUpdate, step.before(sf_physics_bevy::PhysicsSystems::Step))
        .add_systems(Update, (spawn_ocean, mark_camera, bed))
        .add_systems(PostUpdate, sync.after(TerrainSystems::Update).before(WaterSystems::Drive));
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
/// with the origin in the same frame), the sea level in render space, the froxel volume.
fn sync(
    time: Res<Time<Virtual>>,
    origin: Res<TerrainOrigin>,
    active: Res<ActiveWorldspace>,
    volume: Option<Res<sf_atmos_bevy::FroxelVolume>>,
    mut frame: ResMut<WaterFrame>,
    mut fog: ResMut<WaterFogInput>,
) {
    let cell = origin.0.cell;
    let exterior = ids::exterior();
    let in_exterior = active.0.as_ref().is_none_or(|ws| *ws == exterior);
    #[allow(clippy::cast_precision_loss)]
    let next = WaterFrame { enabled: in_exterior, time: time.elapsed_secs_f64(), origin_cell: cell, level: -(cell.y as f32) * sf_coords::GRID_CELL };
    if *frame != next {
        *frame = next;
    }
    let want = volume.map_or_else(WaterFogInput::default, |v| WaterFogInput { froxels: Some(v.image.clone()), params: v.params });
    if *fog != want {
        *fog = want;
    }
}
