//! FL-5c2: movers in a guide river over flat ground swim and drift with its flow; flyers, dry ground and
//! leaving the water behave; `fSwimHeightScale` reaches the bound rule.
//!
//! The ground probe is an empty `Physics` (no hits): the flat-ground probe of `sf_controller_bevy`'s tests
//! needs `sf_controller`'s probe types, which this crate does not depend on. Nothing here needs ground: a
//! swimmer and a flyer have no gravity, a dry walker just falls, still `Ground`.
#![allow(clippy::float_cmp)] // exact: tunable values copied through

use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use glam::{DVec3, Vec3};
use sf_controller_bevy::{LocomotionIntent, MoveIntent, Mover, MoverMode, MoverParams, SfControllerPlugin};
use sf_coords::WorldPos;
use sf_physics_bevy::Physics;
use sf_tunables_bevy::TunablesPlugin;
use sf_water::{WaterWorld, WorldspaceWater};
use sf_world_bevy::ActiveWorldspace;
use sf_worldgen::water::{GuideWater, River};

use super::{SwimSettings, add};
use crate::water::Water;
use crate::worldgen_water::{RiverFlow, guide_bodies, river_speeds};

/// Fixed ticks run so far.
#[derive(Resource, Default)]
struct Ticks(u32);

fn count(mut ticks: ResMut<Ticks>) {
    ticks.0 += 1;
}

/// A level 200 m river due north (guide frame) at 3 m, 20 m wide: engine flow along −Z over x = 0.
fn river_points() -> Vec<[f64; 4]> {
    (0..=10).map(|i| [0.0, f64::from(i) * 20.0, 20.0, 3.0]).collect()
}

/// The river's speed at its middle point, m/s.
fn river_speed() -> f32 {
    #[allow(clippy::cast_possible_truncation)]
    let speed = river_speeds(&river_points(), &RiverFlow::default())[5] as f32;
    speed
}

/// Engine position of guide `(x, y)` at y = 0.
fn ground(x: f64, y: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, -y))
}

/// The controller stage over no ground, the river in the exterior's water query, the swim system;
/// frames of 1/60 s on a 1/60 s tick.
fn app(swim_height_scale: Option<f32>) -> App {
    let frame = Duration::from_secs_f64(1.0 / 60.0);
    let guide = GuideWater { rivers: vec![River { id: 1, kind: "river".into(), points: river_points() }], ..GuideWater::default() };
    let mut world = WaterWorld::new();
    world.insert(mw_world::ids::exterior(), WorldspaceWater::new(guide_bodies(&guide, &RiverFlow::default(), 64)));
    let mut app = App::new();
    app.add_plugins((TimePlugin, TunablesPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(frame))
        .insert_resource(Time::<Fixed>::from_duration(frame))
        .add_plugins(SfControllerPlugin::<Physics>::default())
        .init_resource::<Physics>()
        .insert_resource(ActiveWorldspace::default())
        .insert_resource(Water { world, time: 0.0 })
        .init_resource::<Ticks>()
        .add_systems(FixedUpdate, count);
    add(&mut app, swim_height_scale);
    app
}

fn spawn(app: &mut App, at: WorldPos, mode: MoverMode) -> Entity {
    let mut mover = Mover::new(at, 0.0, MoverParams::default(), WorldPos::ORIGIN);
    mover.loco.mode = mode;
    app.world_mut().spawn((mover, MoveIntent(LocomotionIntent::IDLE))).id()
}

/// Updates until `n` more fixed ticks ran.
fn run_ticks(app: &mut App, n: u32) {
    let until = app.world().resource::<Ticks>().0 + n;
    while app.world().resource::<Ticks>().0 < until {
        app.update();
    }
    assert_eq!(app.world().resource::<Ticks>().0, until, "one tick per update");
}

fn mover(app: &App, e: Entity) -> &Mover {
    app.world().get::<Mover>(e).expect("the mover")
}

#[test]
fn river_carries_a_swimmer() {
    let mut app = app(None);
    let start = ground(0.0, 100.0);
    let e = spawn(&mut app, start, MoverMode::Ground);
    // The swim system runs in `Drive`, before the step, so the first tick already drifts: no lag tick.
    run_ticks(&mut app, 60);
    let m = mover(&app, e);
    assert!(matches!(m.loco.mode, MoverMode::Swim { .. }), "{:?}", m.loco.mode);
    let moved = start.delta_f32(m.loco.position);
    let speed = river_speed();
    assert!(speed > 0.0);
    assert!((-moved.z - speed).abs() <= 0.02, "along the flow {} m in 1 s at {speed} m/s", -moved.z);
    assert!(moved.x.abs() <= 0.01, "across the flow {} m", moved.x);
}

#[test]
fn fly_mover_is_untouched() {
    let mut app = app(None);
    let start = ground(0.0, 100.0).offset(Vec3::Y);
    let e = spawn(&mut app, start, MoverMode::Fly { hover: None });
    run_ticks(&mut app, 60);
    let m = mover(&app, e);
    assert_eq!(m.loco.mode, MoverMode::Fly { hover: None });
    assert_eq!(m.loco.current, Vec3::ZERO);
    assert!(start.delta_f32(m.loco.position).length() < 1e-6, "{:?}", start.delta_f32(m.loco.position));
}

#[test]
fn dry_mover_stays_ground() {
    let mut app = app(None);
    let e = spawn(&mut app, ground(100.0, 100.0), MoverMode::Ground);
    run_ticks(&mut app, 10);
    let m = mover(&app, e);
    assert_eq!(m.loco.mode, MoverMode::Ground);
    assert_eq!(m.loco.current, Vec3::ZERO);
}

#[test]
fn leaving_water_ends_swim() {
    let mut app = app(None);
    let e = spawn(&mut app, ground(100.0, 100.0), MoverMode::Swim { surface: 1.0 });
    run_ticks(&mut app, 1);
    let m = mover(&app, e);
    assert_eq!(m.loco.mode, MoverMode::Ground);
    assert_eq!(m.loco.current, Vec3::ZERO);
}

#[test]
fn swim_height_scale_sets_enter_fraction() {
    for (scale, want) in [(Some(0.9), 0.9), (None, 0.7)] {
        let mut a = app(scale);
        a.update();
        assert_eq!(a.world().resource::<SwimSettings>().0.enter_fraction, want, "{scale:?}");
    }
}
