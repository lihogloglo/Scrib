//! Headless App tests for the game's wander drive (`MA3`, [`add_drive`]): the real [`drive`]
//! system on the fixed tick with `sf_nav_bevy`'s queries and [`NavCorridor`] and the controller's
//! mover over a flat floor. A ground wanderer on a resident navmesh runs the HTN methods and walks
//! its `MoveTo` goals through the corridor; one in a worldspace without navmesh walks the
//! pathgrid fallback (its intent restored after the corridor's idle write).


use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use bevy_math::DVec3;
use mw_ai::{CellKey, PathGrids, Wander, WanderDriver, WanderPlanner};
use sf_controller::{GroundHit, GroundProbe, HeightSurface, ProbeQuery, TerrainProbe};
use sf_controller_bevy::{Avoider, MoveIntent, Mover, MoverParams, SfControllerPlugin};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_nav::{GridBackend, GridCell, GridConfig, PolyFlags, QueryFilter};
use sf_nav_bevy::{NavCorridor, NavCorridorPlugin, NavMeshes, NavPlugin, NavQueryPlugin};
use sf_world::WorldspaceId;

use super::{
    ActorSpace, GroundOnPhysics, MwGround, Pathgrids, Walker, WanderModes, add_drive,
};

/// The flat room: 40 m x 40 m of navmesh in `nav_room`, home in its middle.
const ROOM_CELLS: usize = 40;
const HOME: [f64; 3] = [20.0, 0.0, 20.0];
/// Wander radius, m.
const RADIUS: f32 = 6.0;
/// Walk speed, m/s.
const SPEED: f32 = 1.5;

struct FlatFloor;

impl HeightSurface for FlatFloor {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

#[derive(Resource)]
struct Floor(TerrainProbe<FlatFloor>);

impl GroundProbe for Floor {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

fn nav_ws() -> WorldspaceId {
    WorldspaceId::new("nav_room")
}

fn home() -> WorldPos {
    WorldPos::from_f64(DVec3::from_array(HOME))
}

fn app() -> App {
    let mut backend = GridBackend::new(GridConfig { cell_size: 1.0, tile_cells: 40 }).unwrap();
    let row = ".".repeat(ROOM_CELLS);
    let rows: Vec<&str> = (0..ROOM_CELLS).map(|_| row.as_str()).collect();
    backend
        .insert_rows(&nav_ws(), mw_bake::nav::HUMANOID, 0.5, &rows, |c| {
            (c == '.').then_some(GridCell::new(0.0, mw_bake::nav::GROUND, PolyFlags::WALK))
        })
        .unwrap();
    let mut app = App::new();
    app.add_plugins((
        TaskPoolPlugin::default(),
        TimePlugin,
        SfControllerPlugin::<Floor>::default(),
        NavPlugin,
        NavQueryPlugin,
        NavCorridorPlugin,
    ))
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
    .insert_resource(Floor(TerrainProbe(FlatFloor)))
    .insert_resource(NavMeshes(Box::new(backend)))
    .insert_resource(Pathgrids(PathGrids::default()))
    .init_resource::<MwGround>()
    .insert_resource(GroundOnPhysics(false));
    add_drive(&mut app, mw_ai::wander_library());
    app.update();
    app
}

/// A ground wanderer as `attach` makes it (no pathgrid: walks go to points in the wander disc).
fn spawn_wanderer(app: &mut App, ws: &WorldspaceId, home: WorldPos, key: u128, seed: u64) -> Entity {
    let wander = Wander {
        distance: RADIUS,
        duration_hours: 0.0,
        game_hour: 0,
        idle_chances: [20, 20, 0, 0, 0, 0, 0, 0],
        repeat: true,
    };
    let driver = WanderDriver::with_vocab(
        WanderPlanner::new(wander, home),
        SPEED,
        seed,
        ws.clone(),
        mw_ai::wander_library(),
    );
    let filter = QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID);
    app.world_mut()
        .spawn((
            Mover::new(home, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent::default(),
            ActorSpace(ws.clone()),
            Walker {
                driver,
                variant: None,
                key: CellKey::interior(ws.as_str()),
                held_intent: None,
                on_navmesh: None,
            },
            Avoider { max_speed: SPEED * 1.5, ..Avoider::default() },
            NavCorridor::new(ActorKey(key), mw_bake::nav::HUMANOID, ws.clone(), SPEED).with_filter(filter),
        ))
        .id()
}

#[derive(Default)]
struct Watch {
    modes: Vec<bool>,
    corridor_seen: bool,
    moved_max: f32,
    rested: bool,
    still_s: f32,
}

#[test]
fn wanderers_run_htn_over_the_navmesh_and_the_pathgrid_fallback_off_it() {
    let mut app = app();
    // Far apart: avoidance (ORCA) has no worldspaces, and two walkers sharing a home would
    // dodge each other (in the game only the active worldspace is awake).
    let homes = [home(), home().offset(Vec3::new(200.0, 0.0, 200.0))];
    let on_mesh = spawn_wanderer(&mut app, &nav_ws(), homes[0], 1, 11);
    let off_mesh = spawn_wanderer(&mut app, &WorldspaceId::new("no_navmesh"), homes[1], 2, 12);
    let dt = 1.0 / 60.0;
    let mut watch = [Watch::default(), Watch::default()];
    let mut last = homes;
    // 90 s of fixed ticks.
    for _ in 0..(90 * 60) {
        app.update();
        for (i, e) in [on_mesh, off_mesh].into_iter().enumerate() {
            let w = &mut watch[i];
            let world = app.world();
            let pos = world.get::<Mover>(e).unwrap().loco.position;
            let walker = world.get::<Walker>(e).unwrap();
            if let Some(m) = walker.on_navmesh {
                w.modes.push(m);
            }
            w.corridor_seen |= world.get::<NavCorridor>(e).unwrap().corridor().is_some();
            let off = homes[i].delta_f32(pos);
            assert!(off.is_finite(), "walker {i}: NaN position");
            assert!(off.y.abs() < 0.5, "walker {i}: left the floor ({off:?})");
            w.moved_max = w.moved_max.max(off.with_y(0.0).length());
            // Comes to rest: standing still for a second between walks.
            let speed = last[i].delta_f32(pos).with_y(0.0).length() / dt;
            w.still_s = if speed < 0.05 && !walker.driver.is_walking() { w.still_s + dt } else { 0.0 };
            w.rested |= w.still_s >= 1.0;
            last[i] = pos;
        }
    }
    let world = app.world();
    let walker = |e| world.get::<Walker>(e).unwrap();
    let (nav, fb) = (walker(on_mesh), walker(off_mesh));
    let modes = world.resource::<WanderModes>();

    // On the navmesh every tick: HTN + `NavCorridor`, whose path replies arrived.
    assert!(watch[0].modes.iter().all(|&m| m), "navmesh walker left HTN mode");
    assert!(watch[0].corridor_seen, "no corridor was ever loaded");
    assert!(nav.driver.arrival_count() >= 3, "{} navmesh arrivals", nav.driver.arrival_count());
    assert_eq!(nav.driver.stuck_count(), 0, "open room: no navmesh walk given up");
    // Off the navmesh every tick: the pathgrid fallback walks too (intent survives the corridor).
    assert!(watch[1].modes.iter().all(|&m| !m), "fallback walker took the navmesh path");
    assert!(!watch[1].corridor_seen);
    assert!(fb.driver.arrival_count() >= 3, "{} fallback arrivals", fb.driver.arrival_count());
    assert_eq!(modes.navmesh_arrivals, u64::from(nav.driver.arrival_count()));
    assert_eq!(modes.fallback_arrivals, u64::from(fb.driver.arrival_count()));
    for (i, w) in watch.iter().enumerate() {
        assert!(w.moved_max > 1.0, "walker {i} never walked");
        assert!(w.moved_max <= RADIUS + 1.0, "walker {i} went {} m from home (radius {RADIUS})", w.moved_max);
        assert!(w.rested, "walker {i} never came to rest");
    }
    // The 5 s report saw the split (one of each).
    assert_eq!((modes.navmesh, modes.fallback), (1, 1));
}

/// A navmesh wanderer asked to turn in place (`Face`) keeps the executor's facing past
/// [`NavCorridor`]'s idle reset: it turns to the target and the operator succeeds (before the fix
/// the drive wrote `IDLE`, dropping the facing, and the turn never finished).
#[test]
fn a_face_step_turns_the_navmesh_walker_and_succeeds() {
    use sf_ai::{ExecConfig, FaceTarget};

    let mut app = app();
    let e = spawn_wanderer(&mut app, &nav_ws(), home(), 1, 11);
    for _ in 0..30 {
        app.update();
    }
    let yaw0 = app.world().get::<Mover>(e).unwrap().loco.motion.yaw;
    let target = wrap(yaw0 + 2.0);
    app.world_mut().get_mut::<Walker>(e).unwrap().driver.face(FaceTarget::Yaw(target), 0.0);
    let tol = ExecConfig::default().face_tolerance_rad;
    let mut done_at = None;
    // At most 10 s of fixed ticks.
    for tick in 0..(10 * 60) {
        app.update();
        let world = app.world();
        let mover = world.get::<Mover>(e).unwrap();
        let walker = world.get::<Walker>(e).unwrap();
        assert!(mover.loco.motion.yaw.is_finite(), "tick {tick}: NaN yaw");
        assert!(home().delta_f32(mover.loco.position).is_finite(), "tick {tick}: NaN position");
        assert_eq!(walker.on_navmesh, Some(true), "tick {tick}: left the navmesh");
        if walker.driver.face_count() == 1 {
            done_at = Some(tick);
            let off = wrap(mover.loco.motion.yaw - target).abs();
            assert!(off <= tol + 1e-3, "succeeded {off} rad off the target (tolerance {tol})");
            break;
        }
    }
    let tick = done_at.expect("the Face operator never succeeded");
    assert!(tick > 0, "a 2 rad turn took no time");
}

/// `a` wrapped to (-PI, PI].
fn wrap(a: f32) -> f32 {
    let t = a.rem_euclid(std::f32::consts::TAU);
    if t > std::f32::consts::PI { t - std::f32::consts::TAU } else { t }
}
