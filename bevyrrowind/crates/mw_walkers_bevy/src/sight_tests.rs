//! Headless App tests of the game's sight wiring (`MA14`, [`crate::sight::add`]) on the game's ground,
//! `sf_physics_bevy::Physics`: a placed actor as `attach` makes it (mover, `AiActor`, the observer
//! from [`crate::sight::observer`]) faces the player body (`PlayerBody` + `ActionBody`, made a target
//! by the wiring). Its awareness of the host player rises over a second of fixed ticks; a static wall
//! collider between them keeps it at zero.

use std::time::Duration;

use bevy_app::{App, TaskPoolPlugin};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action_bevy::ActionBody;
use sf_ai::PerceptTarget;
use sf_ai_bevy::{AiActor, AiAwareness, Perceivable};
use sf_controller::Locomotion;
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorlds, ShapeRef};
use sf_physics_bevy::Physics;
use sf_world::WorldspaceId;

const HZ: f64 = 60.0;
const PLAYER: PerceptTarget = PerceptTarget::Player(HOST_KEY);
/// The player stands this far ahead of the guard (+Z, inside the default 30 m sight range).
const DIST_M: f32 = 10.0;

fn at(x: f32, y: f32, z: f32) -> WorldTransform {
    WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(x, y, z)), ..WorldTransform::IDENTITY }
}

/// The game's sight wiring over a physics world: a floor (top at y = 0) and, with `wall`, a 6 m x
/// 3 m static wall halfway between the guard and the player.
fn app(wall: bool) -> App {
    let ws = WorldspaceId::new("sight_room");
    let mut physics = Physics { worlds: PhysicsWorlds::default(), params: PhysicsParams::default(), terrain: None };
    physics.insert(&ws, &BodyDesc::fixed(at(0.0, -0.5, 0.0), ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC));
    if wall {
        physics.insert(&ws, &BodyDesc::fixed(at(0.0, 1.5, DIST_M / 2.0), ShapeRef::Box(Vec3::new(3.0, 1.5, 0.25)), Layer::STATIC));
    }
    physics.worlds.set_active(Some(ws));
    physics.worlds.active_world_mut().expect("active world").refresh_queries();

    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
        .insert_resource(Time::<Fixed>::from_hz(HZ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(physics);
    crate::sight::add::<Physics>(&mut app);
    app.update();
    app
}

/// A placed guard at the origin facing +Z, built as `attach` builds it (awake: in processing range).
fn guard(app: &mut App) -> Entity {
    let params = MoverParams::default();
    app.world_mut()
        .spawn((
            Mover::new(WorldPos::ORIGIN, 0.0, params, WorldPos::ORIGIN),
            AiActor(ActorKey(7)),
            crate::sight::observer(params.height),
        ))
        .id()
}

/// The player's body as `mw_player_bevy` spawns it (its motion half), `DIST_M` ahead of the guard.
fn player(app: &mut App) -> Entity {
    let feet = WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, DIST_M));
    let body = ActionBody::new(Locomotion::new(feet, std::f32::consts::PI), MoverParams::default(), WorldPos::ORIGIN);
    app.world_mut().spawn((PlayerBody, body)).id()
}

fn level(app: &App, guard: Entity) -> f32 {
    app.world().get::<AiAwareness>(guard).expect("observer brings its meter").0.get(PLAYER).map_or(0.0, |a| a.level)
}

/// One second of frames, one fixed tick each: the guard's awareness after each.
fn run_one_second(app: &mut App, guard: Entity) -> Vec<f32> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let frames = HZ as usize;
    (0..frames)
        .map(|_| {
            app.update();
            level(app, guard)
        })
        .collect()
}

#[test]
fn a_guard_facing_the_player_grows_aware_over_a_second() {
    let mut app = app(false);
    let g = guard(&mut app);
    let p = player(&mut app);
    let levels = run_one_second(&mut app, g);

    let target = app.world().get::<Perceivable>(p).expect("the wiring made the player body a target");
    assert_eq!(target.target, PLAYER);
    assert!(target.chest_m > 0.0 && target.chest_m < target.head_m, "{target:?}");
    let last = *levels.last().unwrap();
    assert!(last > 0.0, "no awareness after 1 s: {levels:?}");
    assert!(levels.windows(2).all(|w| w[1] >= w[0]), "awareness fell while in plain view: {levels:?}");
}

#[test]
fn a_wall_between_keeps_the_guard_unaware() {
    let mut app = app(true);
    let g = guard(&mut app);
    player(&mut app);
    let levels = run_one_second(&mut app, g);
    assert!(levels.iter().all(|&l| l <= 0.0), "the guard saw through the wall: {levels:?}");
}
