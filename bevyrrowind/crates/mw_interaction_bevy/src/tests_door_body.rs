//! Headless tests of `door_body`: an opened swing door turns its kinematic body and its pivot, a
//! restored rest snaps both (also for a body inserted later), and without physics the pivot
//! follows the shell. Wiring checks with loose tolerances (look and feel: sanity only).

use std::sync::{Arc, PoisonError};
use std::time::Duration;

use bevy_app::App;
use bevy_math::Affine3A;
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use glam::Quat;
use sf_controller::{CollisionBuilder, StaticShape, SurfaceId};
use sf_coords::WorldTransform;
use sf_interaction::{DOOR_STATE_OWNER, DoorState, VERB_OPEN};
use sf_interaction_bevy::{DoorRest, Doors, InteractionSettings, UseApproved};
use sf_physics_bevy::sf_physics::BodyId;
use sf_physics_bevy::{Physics, PhysicsHook, PhysicsPlugin, PhysicsShapeSource, PhysicsStreaming};
use sf_script_bevy::sf_script::{Handle, ScriptRuntime};
use sf_script_bevy::{Activated, PLAYER, Scripts, ScriptsPlugin};
use sf_tunables_bevy::TunablesPlugin;
use sf_world::IndexedPlacement;
use sf_world_bevy::{ActiveWorldspace, WorldOverlay};

use crate::door::{DoorHook, DoorPivots};
use crate::door_body::leaf_pose;
use crate::tests::{Nothing, tick};
use crate::tests_door::{DOOR, door, residency, spawn, ws};
use crate::{FurnitureDefs, add_with_defs};

/// The interaction app on a manual 60 Hz clock with physics and a 2 x 2 m panel for every shape.
fn physics_app() -> App {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let mut app = App::new();
    app.insert_resource(Scripts::new(ScriptRuntime::default()))
        .add_plugins((TimePlugin, PhysicsPlugin::default(), TunablesPlugin, ScriptsPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws())))
        .init_resource::<WorldOverlay>();
    let mut b = CollisionBuilder::new(1e-3);
    let p = [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 2.0, 0.0], [-1.0, 2.0, 0.0]];
    b.add(&p, &[0, 1, 2, 0, 2, 3], Affine3A::IDENTITY, SurfaceId::DEFAULT);
    let shape = Arc::new(StaticShape::new(&b.finish()).expect("panel mesh"));
    app.insert_resource(PhysicsShapeSource(Arc::new(move |_: &str| Some(shape.clone()))));
    add_with_defs(&mut app, FurnitureDefs::default());
    app.update();
    app
}

/// The door placement with a model (a body needs one).
fn door_with_model() -> IndexedPlacement {
    let mut p = door();
    p.placement.model = Some(sf_vfs::VfsPath::new("door.nif").expect("a model path"));
    p
}

fn hook() -> DoorHook {
    DoorHook(Arc::new(PhysicsHook { inner: Arc::new(Nothing), statics: true }))
}

fn body_id(app: &App) -> Option<BodyId> {
    app.world().resource::<PhysicsStreaming>().body(&ws(), DOOR)
}

fn pose(app: &App, body: BodyId) -> WorldTransform {
    app.world().resource::<Physics>().world(&ws()).expect("a world").pose(body).expect("a live body")
}

/// Frames until the door's body is in; returns it.
fn wait_body(app: &mut App) -> BodyId {
    for _ in 0..500 {
        app.update();
        if let Some(b) = body_id(app) {
            return b;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("door body not inserted");
}

/// Spawns and activates the door; returns the closed placement.
fn stream_door(app: &mut App) -> WorldTransform {
    let hook = hook();
    let p = door_with_model();
    spawn(app, &hook, &p);
    residency(app, &hook, &p, true);
    p.placement.transform
}

fn approve_open(app: &mut App) {
    let activation = Activated { target: Handle(DOOR.0), actor: PLAYER, verb: Some(VERB_OPEN.0) };
    app.world_mut().write_message(UseApproved { activation, verb: Some(VERB_OPEN), slot: None });
}

/// The open pose the shell's leaf gives now.
fn shell_pose(app: &App, closed: &WorldTransform) -> WorldTransform {
    let shell = app.world().resource::<Doors>().get(DOOR).expect("registered");
    leaf_pose(closed, &shell.object, &shell.leaf())
}

fn pivot(app: &App) -> Transform {
    let e = app.world().resource::<DoorPivots>().get(DOOR).expect("a pivot");
    *app.world().get::<Transform>(e).expect("a pivot transform")
}

fn moving(app: &App) -> bool {
    app.world().resource::<Doors>().get(DOOR).is_some_and(|d| d.state.is_moving())
}

fn same_rotation(a: Quat, b: Quat) -> bool {
    a.angle_between(b) < 1e-3
}

/// Fixed ticks of one swing, from the live settings.
fn swing_ticks(app: &App) -> usize {
    let p = app.world().resource::<InteractionSettings>().0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (p.door_swing_s * p.tick_hz).ceil() as usize;
    n
}

#[test]
fn an_opened_door_turns_its_body_and_its_pivot() {
    let mut app = physics_app();
    let closed = stream_door(&mut app);
    let body = wait_body(&mut app);
    let pivot_closed = pivot(&app);
    approve_open(&mut app);
    for _ in 0..4 {
        app.update();
    }
    let mid = closed.rotation.angle_between(pose(&app, body).rotation).to_degrees();
    assert!(mid > 0.0 && mid < 90.0, "mid-swing angle {mid}");
    for _ in 0..swing_ticks(&app) + 10 {
        app.update();
    }
    assert!(!moving(&app), "the swing ended");
    let now = pose(&app, body);
    let leaf = app.world().resource::<Doors>().get(DOOR).expect("registered").leaf();
    assert!(same_rotation(now.rotation, leaf.rotation), "body {:?} leaf {:?}", now.rotation, leaf.rotation);
    let open = closed.rotation.angle_between(now.rotation).to_degrees();
    assert!((open - 90.0).abs() < 1.0, "open angle {open}");
    let off = closed.pos.delta_f32(now.pos).length();
    assert!(off < 1e-4, "the body moved {off} m");
    let t = pivot(&app);
    assert!(same_rotation(t.rotation, now.rotation), "pivot {:?} body {:?}", t.rotation, now.rotation);
    assert!(t.translation.distance(pivot_closed.translation) < 1e-4, "pivot moved {:?} -> {:?}", pivot_closed.translation, t.translation);
}

#[test]
fn a_restored_rest_snaps_the_body_and_it_stays() {
    let mut app = physics_app();
    let closed = stream_door(&mut app);
    let body = wait_body(&mut app);
    for _ in 0..2 {
        app.update();
    }
    app.world_mut().write_message(DoorRest { placement: DOOR, open: true, sign: 1 });
    for _ in 0..3 {
        app.update();
    }
    let open = shell_pose(&app, &closed);
    assert!(closed.rotation.angle_between(open.rotation) > 1.0, "the rest is open");
    let check = |app: &App, when: &str| {
        let now = pose(app, body);
        assert!(same_rotation(now.rotation, open.rotation), "{when}: body {:?} open {:?}", now.rotation, open.rotation);
        assert!(open.pos.delta_f32(now.pos).length() < 1e-4, "{when}: body at {:?}", now.pos);
        assert!(same_rotation(pivot(app).rotation, open.rotation), "{when}: pivot {:?}", pivot(app).rotation);
    };
    check(&app, "after the snap");
    for _ in 0..10 {
        app.update();
    }
    check(&app, "10 steps later");
}

#[test]
fn a_body_inserted_after_a_restored_rest_takes_the_open_pose() {
    let mut app = physics_app();
    let bytes = DoorState::resting(true, 1).rest_bytes().to_vec();
    app.world().resource::<WorldOverlay>().0.write().unwrap_or_else(PoisonError::into_inner).set_state(DOOR, DOOR_STATE_OWNER, bytes);
    let closed = stream_door(&mut app);
    assert!(body_id(&app).is_none(), "the body is not in yet");
    let body = wait_body(&mut app);
    for _ in 0..2 {
        app.update();
    }
    let open = shell_pose(&app, &closed);
    assert!(closed.rotation.angle_between(open.rotation) > 1.0, "the restored rest is open");
    let now = pose(&app, body);
    assert!(same_rotation(now.rotation, open.rotation), "body {:?} open {:?}", now.rotation, open.rotation);
    assert!(open.pos.delta_f32(now.pos).length() < 1e-4, "body at {:?}", now.pos);
}

#[test]
fn without_physics_the_pivot_follows_the_shell() {
    let mut app = crate::tests::app();
    let hook = DoorHook(Arc::new(Nothing));
    let p = door();
    spawn(&mut app, &hook, &p);
    residency(&mut app, &hook, &p, true);
    tick(&mut app, 1);
    approve_open(&mut app);
    app.update();
    let ticks = swing_ticks(&app) + 5;
    tick(&mut app, ticks);
    app.update();
    assert!(!moving(&app), "the swing ended");
    let leaf = app.world().resource::<Doors>().get(DOOR).expect("registered").leaf();
    assert!(p.placement.transform.rotation.angle_between(leaf.rotation) > 1.0, "the door opened");
    assert!(same_rotation(pivot(&app).rotation, leaf.rotation), "pivot {:?} leaf {:?}", pivot(&app).rotation, leaf.rotation);
}
