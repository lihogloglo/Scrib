//! Headless lock-on shell test (combat.md "C1c"): a press locks the lockable character ahead and
//! mirrors it (`ActionState::lock`, `LockAim`, the camera's `LockOn`); a wall between them
//! refuses it (the line-of-sight cast); past 20 m and on a traversal the lock breaks, and
//! touchdown offers it back.

use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{ActionState, CharacterId, CharacterParams, Command, LockRequest, RuleKey, TargetId};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionSystems, ActionTick, LockAim, SfActionPlugin};
use sf_camera::BreakReason;
use sf_combat::LockEvent;
use sf_controller::{GroundHit, GroundProbe, Locomotion, MoverParams, ProbeQuery, SurfaceId};
use sf_coords::WorldPos;

use crate::lock::{LockOn, Lockable};
use crate::{LockOwner, SfLockOnPlugin};

const PLAYER: CharacterId = CharacterId(1);
const DUMMY: CharacterId = CharacterId(2);

/// No ground for the mover (bodies stay where the test puts them).
#[derive(Resource)]
struct NoGround;

impl GroundProbe for NoGround {
    fn probe(&self, _: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        out.fill(None);
    }
}

/// The lock's line-of-sight probe: a wall across `z = 4` when `up`.
#[derive(Resource)]
struct Wall {
    up: bool,
}

impl GroundProbe for Wall {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out) {
            let z0 = WorldPos::ORIGIN.delta_f32(q.origin).z;
            let reach = z0 + q.dir.z * q.max;
            *o = (self.up && z0 < 4.0 && reach >= 4.0 && q.dir.z > 0.0).then(|| {
                let distance = (4.0 - z0) / q.dir.z;
                GroundHit { point: q.origin.offset(q.dir * distance), normal: Vec3::NEG_Z, distance, surface: SurfaceId::default() }
            });
        }
    }
}

/// A lock press for the next tick.
#[derive(Resource, Default)]
struct Press(bool);

/// The player's command: empty, looking down +Z, with the pending press.
fn script(tick: Res<ActionTick>, mut press: ResMut<Press>, mut chars: Query<&mut ActionCharacter>) {
    for mut c in &mut chars {
        let mut cmd = Command::empty(tick.0);
        if c.id == PLAYER && std::mem::take(&mut press.0) {
            cmd.lock_request = LockRequest::Toggle;
        }
        c.command = cmd;
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<NoGround>::default(), SfLockOnPlugin::<Wall>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Wall { up: false })
        .init_resource::<Press>()
        .add_systems(FixedUpdate, script.in_set(ActionSystems::Command));
    app
}

fn spawn(app: &mut App, id: CharacterId, z: f32) -> Entity {
    let params = MoverParams::default();
    let character = ActionCharacter::new(id, ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(Vec3::Z * z), 0.0);
    loco.grounded = Some(GroundHit { point: loco.position, normal: Vec3::Y, distance: 0.0, surface: SurfaceId::default() });
    let body = ActionBody::new(loco, params, WorldPos::ORIGIN);
    app.world_mut().spawn((character, body, Transform::default())).id()
}

fn run(app: &mut App, ticks: usize) {
    for _ in 0..ticks {
        app.update();
    }
}

fn press(app: &mut App) {
    app.world_mut().resource_mut::<Press>().0 = true;
    run(app, 2);
}

fn lock_of(app: &App, e: Entity) -> Option<TargetId> {
    app.world().get::<ActionCharacter>(e).unwrap().state.lock
}

fn last(app: &App, e: Entity) -> Option<LockEvent> {
    app.world().get::<LockOwner>(e).unwrap().last
}

fn set_z(app: &mut App, e: Entity, z: f32) {
    app.world_mut().get_mut::<ActionBody>(e).unwrap().loco.position = WorldPos::ORIGIN.offset(Vec3::Z * z);
}

#[test]
fn a_press_locks_the_dummy_ahead_and_mirrors_it_to_the_camera() {
    let mut app = app();
    let player = spawn(&mut app, PLAYER, 0.0);
    app.world_mut().entity_mut(player).insert(LockOwner::default());
    let dummy = spawn(&mut app, DUMMY, 8.0);
    app.world_mut().entity_mut(dummy).insert(Lockable::default());
    run(&mut app, 3);
    assert_eq!(lock_of(&app, player), None, "no lock without a press");
    assert!(app.world().get::<LockOn>(player).is_none());

    press(&mut app);
    assert_eq!(lock_of(&app, player), Some(TargetId(DUMMY.0)));
    assert_eq!(app.world().get::<LockOn>(player), Some(&LockOn(dummy)));
    let aim = app.world().get::<LockAim>(player).unwrap().0.expect("the step senses the target");
    assert!((aim - Vec3::new(0.0, Lockable::default().aim_height, 8.0)).length() < 1e-4, "{aim}");

    // Held to 19 m; past 20 m it breaks and the mirrors clear.
    set_z(&mut app, dummy, 19.0);
    run(&mut app, 2);
    assert_eq!(lock_of(&app, player), Some(TargetId(DUMMY.0)));
    set_z(&mut app, dummy, 21.0);
    run(&mut app, 2);
    assert_eq!(lock_of(&app, player), None);
    assert_eq!(last(&app, player), Some(LockEvent::Broke(BreakReason::Range)));
    assert!(app.world().get::<LockOn>(player).is_none());
    assert_eq!(app.world().get::<LockAim>(player).unwrap().0, None);
}

#[test]
fn a_wall_refuses_the_lock_and_a_traversal_breaks_it_until_touchdown() {
    let mut app = app();
    let player = spawn(&mut app, PLAYER, 0.0);
    app.world_mut().entity_mut(player).insert(LockOwner::default());
    let dummy = spawn(&mut app, DUMMY, 8.0);
    app.world_mut().entity_mut(dummy).insert(Lockable::default());
    run(&mut app, 2);

    app.world_mut().resource_mut::<Wall>().up = true;
    press(&mut app);
    assert_eq!(lock_of(&app, player), None, "a wall between: no lock");
    assert_eq!(last(&app, player), Some(LockEvent::Recentre));
    app.world_mut().resource_mut::<Wall>().up = false;

    press(&mut app);
    assert_eq!(lock_of(&app, player), Some(TargetId(DUMMY.0)));
    // A mantle: off the ground, traversing.
    {
        let mut body = app.world_mut().get_mut::<ActionBody>(player).unwrap();
        body.loco.traversing = true;
        body.loco.grounded = None;
    }
    run(&mut app, 2);
    assert_eq!(lock_of(&app, player), None);
    assert_eq!(last(&app, player), Some(LockEvent::Broke(BreakReason::Traversal)));
    assert!(app.world().get::<LockOn>(player).is_none());
    // Touchdown on the ledge: offered back without a press.
    {
        let mut body = app.world_mut().get_mut::<ActionBody>(player).unwrap();
        body.loco.traversing = false;
        body.loco.grounded = Some(GroundHit { point: body.loco.position, normal: Vec3::Y, distance: 0.0, surface: SurfaceId::default() });
    }
    run(&mut app, 2);
    assert_eq!(lock_of(&app, player), Some(TargetId(DUMMY.0)));
    assert_eq!(last(&app, player), Some(LockEvent::Reacquired(TargetId(DUMMY.0))));
    assert_eq!(app.world().get::<LockOn>(player), Some(&LockOn(dummy)));
}
