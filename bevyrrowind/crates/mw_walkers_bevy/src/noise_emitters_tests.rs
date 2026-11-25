//! Headless App tests of the player's noise (`AI8c2`, [`crate::noise_emitters`]) through the game's
//! sight wiring ([`crate::sight::add`] on `sf_physics_bevy::Physics`): the player body (`PlayerBody`
//! with its `ActionBody` and `ActionCharacter`) is moved by hand between frames (the mover isn't in
//! the app; the emitter reads the body after `ActionSystems::Move`), its gait set on the chooser's
//! output. Per-tick moves are exact binary fractions of the stride, so step counts are exact.

use std::time::Duration;

use bevy_app::{App, FixedUpdate, TaskPoolPlugin};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::{HOST_KEY, PlayerBody};
use sf_action::{ActionState, CharacterId, CharacterParams, RuleKey};
use sf_action_bevy::{ActionBody, ActionCharacter};
use sf_ai::PerceptTarget;
use sf_ai_bevy::{AiActor, AiAwareness, AiPerceptionSystems, NoiseEvent};
use sf_controller::{Gait, GroundHit, Locomotion, SurfaceId};
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorlds, ShapeRef};
use sf_physics_bevy::Physics;
use sf_world::WorldspaceId;

use super::{NoiseSettings, PlayerNoiseKinds};

const HZ: f64 = 60.0;
const PLAYER: PerceptTarget = PerceptTarget::Player(HOST_KEY);
/// The player walks across +X this far ahead of the guard (+Z).
const DIST_M: f32 = 10.0;

fn at(x: f32, y: f32, z: f32) -> WorldTransform {
    WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(x, y, z)), ..WorldTransform::IDENTITY }
}

/// Every noise written, collected after hearing.
#[derive(Resource, Default)]
struct Heard(Vec<NoiseEvent>);

fn collect(mut reader: MessageReader<NoiseEvent>, mut heard: ResMut<Heard>) {
    heard.0.extend(reader.read().cloned());
}

/// The game's sight wiring over a floor (top at y = 0) and a 6 m x 3 m wall halfway between the
/// guard and the player's line (sight blocked: only hearing can raise the guard's meter).
fn room() -> App {
    let ws = WorldspaceId::new("noise_room");
    let mut physics = Physics { worlds: PhysicsWorlds::default(), params: PhysicsParams::default(), terrain: None };
    physics.insert(&ws, &BodyDesc::fixed(at(0.0, -0.5, 0.0), ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC));
    physics.insert(&ws, &BodyDesc::fixed(at(0.0, 1.5, DIST_M / 2.0), ShapeRef::Box(Vec3::new(3.0, 1.5, 0.25)), Layer::STATIC));
    physics.worlds.set_active(Some(ws));
    physics.worlds.active_world_mut().expect("active world").refresh_queries();

    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), TimePlugin))
        .insert_resource(Time::<Fixed>::from_hz(HZ))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(physics)
        .init_resource::<Heard>()
        .add_systems(FixedUpdate, collect.after(AiPerceptionSystems::Hearing));
    crate::sight::add::<Physics>(&mut app);
    app.update();
    app
}

/// A placed guard at the origin facing +Z, built as `attach` builds it (awake).
fn guard(app: &mut App) -> Entity {
    let params = MoverParams::default();
    app.world_mut()
        .spawn((Mover::new(WorldPos::ORIGIN, 0.0, params, WorldPos::ORIGIN), AiActor(ActorKey(7)), crate::sight::observer(params.height)))
        .id()
}

fn ground(feet: WorldPos) -> GroundHit {
    GroundHit::new(feet, Vec3::Y, 0.0, SurfaceId::DEFAULT)
}

/// The player's body, grounded at `x` on the line `DIST_M` ahead of the guard, its stride clock
/// set up by one frame.
fn player(app: &mut App, x: f32, gait: Gait) -> Entity {
    let feet = WorldPos::ORIGIN.offset(Vec3::new(x, 0.0, DIST_M));
    let mut loco = Locomotion::new(feet, 0.0);
    loco.grounded = Some(ground(feet));
    let body = ActionBody::new(loco, MoverParams::default(), WorldPos::ORIGIN);
    let mut character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    character.output.intent.gait = gait;
    let p = app.world_mut().spawn((PlayerBody, body, character)).id();
    app.update();
    app.world_mut().resource_mut::<Heard>().0.clear();
    p
}

/// Moves the grounded body `dx` along +X, then runs one frame.
fn step(app: &mut App, p: Entity, dx: f32) {
    let mut body = app.world_mut().get_mut::<ActionBody>(p).expect("player body");
    let feet = body.loco.position.offset(Vec3::new(dx, 0.0, 0.0));
    body.loco.position = feet;
    body.loco.grounded = Some(ground(feet));
    body.loco.vertical_speed = 0.0;
    app.update();
}

fn heard(app: &mut App) -> Vec<NoiseEvent> {
    std::mem::take(&mut app.world_mut().resource_mut::<Heard>().0)
}

fn level(app: &App, guard: Entity) -> f32 {
    app.world().get::<AiAwareness>(guard).expect("observer brings its meter").0.get(PLAYER).map_or(0.0, |a| a.level)
}

/// Walks `ticks` frames at `stride / per_stride` meters a tick; returns the noises.
fn walk(app: &mut App, p: Entity, stride: f32, per_stride: f32, ticks: usize) -> Vec<NoiseEvent> {
    for _ in 0..ticks {
        step(app, p, stride / per_stride);
    }
    heard(app)
}

#[test]
fn running_emits_one_step_per_run_stride_with_the_run_reach() {
    let mut app = room();
    let s = NoiseSettings::default();
    let p = player(&mut app, -7.5, Gait::Run);
    // 1/16 stride a tick (5.6 m/s): 160 ticks = 10 strides = 15 m.
    let noises = walk(&mut app, p, s.run.stride_m, 16.0, 160);
    assert_eq!(noises.len(), 10, "{noises:?}");
    let kind = PlayerNoiseKinds::default().run_step;
    for n in &noises {
        assert_eq!(n.kind, kind);
        assert!((n.loudness_m - s.run.reach_m).abs() < 1e-6, "{n:?}");
        assert_eq!(n.source, Some(PLAYER));
        assert!(n.reveals_source && n.notify.is_none() && n.space.is_none());
    }
    // Footfalls 1.5 m apart, at the feet.
    let xs: Vec<f32> = noises.iter().map(|n| n.at.delta_f32(WorldPos::ORIGIN).x).collect();
    assert!(xs.windows(2).all(|w| ((w[0] - w[1]).abs() - s.run.stride_m).abs() < 1e-3), "{xs:?}");
    println!("run: {} steps over 15 m, reach {} m, stride {} m", noises.len(), s.run.reach_m, s.run.stride_m);
}

#[test]
fn sneaking_steps_are_quieter_and_fewer_per_second_than_running() {
    let s = NoiseSettings::default();
    let ticks = 160;
    let mut app = room();
    let p = player(&mut app, -7.5, Gait::Run);
    let run = walk(&mut app, p, s.run.stride_m, 16.0, ticks);

    let mut app = room();
    let p = player(&mut app, -1.0, Gait::Sneak);
    // 1/32 sneak stride a tick (0.94 m/s, about the sneak speed): 160 ticks = 5 strides.
    let sneak = walk(&mut app, p, s.sneak.stride_m, 32.0, ticks);
    assert_eq!((run.len(), sneak.len()), (10, 5));
    let kind = PlayerNoiseKinds::default().sneak_step;
    assert!(sneak.iter().all(|n| n.kind == kind && (n.loudness_m - s.sneak.reach_m).abs() < 1e-6), "{sneak:?}");
    assert!(s.sneak.reach_m < s.walk.reach_m && s.walk.reach_m < s.run.reach_m && s.run.reach_m < s.sprint.reach_m);
    println!("over {ticks} ticks: run {} steps at {} m, sneak {} steps at {} m", run.len(), s.run.reach_m, sneak.len(), s.sneak.reach_m);
}

#[test]
fn a_gait_change_keeps_the_stride_phase() {
    let s = NoiseSettings::default();
    let mut app = room();
    let p = player(&mut app, 0.0, Gait::Run);
    // Half a run stride, then walk: the walk step comes after half a walk stride more.
    assert!(walk(&mut app, p, s.run.stride_m, 16.0, 8).is_empty());
    app.world_mut().get_mut::<ActionCharacter>(p).unwrap().output.intent.gait = Gait::Walk;
    assert!(walk(&mut app, p, s.walk.stride_m, 16.0, 7).is_empty());
    let one = walk(&mut app, p, s.walk.stride_m, 16.0, 1);
    assert_eq!(one.len(), 1);
    assert_eq!(one[0].kind, PlayerNoiseKinds::default().walk_step);
}

#[test]
fn standing_still_emits_nothing() {
    let mut app = room();
    let p = player(&mut app, 0.0, Gait::Run);
    for _ in 0..120 {
        step(&mut app, p, 0.0);
    }
    assert!(heard(&mut app).is_empty());
}

#[test]
fn a_teleport_is_not_a_footstep() {
    let mut app = room();
    let p = player(&mut app, 0.0, Gait::Run);
    step(&mut app, p, 20.0);
    assert!(heard(&mut app).is_empty());
}

/// Falls (airborne, `vspeed` the last airborne speed) for 10 ticks, lands and stands a second.
fn fall_and_land(vspeed: f32) -> Vec<NoiseEvent> {
    let mut app = room();
    let p = player(&mut app, 0.0, Gait::Run);
    for _ in 0..10 {
        let mut body = app.world_mut().get_mut::<ActionBody>(p).unwrap();
        body.loco.grounded = None;
        body.loco.vertical_speed = vspeed;
        app.update();
    }
    for _ in 0..60 {
        step(&mut app, p, 0.0);
    }
    heard(&mut app)
}

#[test]
fn a_landing_emits_one_noise_scaled_by_the_fall() {
    let s = NoiseSettings::default();
    let g = CharacterParams::default().gravity;
    let expect = |v: f32| {
        let drop = v * v / (2.0 * g);
        (s.land_base_m + s.land_per_drop * (drop - s.land_min_drop_m)).min(s.land_max_m)
    };
    let kind = PlayerNoiseKinds::default().land;
    let mut reaches = Vec::new();
    for v in [4.0_f32, 7.0, 15.0] {
        let noises = fall_and_land(-v);
        assert_eq!(noises.len(), 1, "impact {v} m/s: {noises:?}");
        let n = &noises[0];
        assert_eq!((n.kind, n.source, n.reveals_source), (kind, Some(PLAYER), true));
        assert!((n.loudness_m - expect(v)).abs() < 1e-4, "impact {v} m/s: {} vs {}", n.loudness_m, expect(v));
        reaches.push(n.loudness_m);
    }
    assert!(reaches[0] < reaches[1] && reaches[2] <= s.land_max_m, "{reaches:?}");
    // A hop down a step (2 m/s, 0.2 m drop) is silent.
    assert!(fall_and_land(-2.0).is_empty());
    println!("landing reach at 4 / 7 / 15 m/s: {reaches:?} m");
}

#[test]
fn a_walker_in_run_reach_hears_the_running_player_not_the_sneaking_one() {
    let s = NoiseSettings::default();
    let cap = sf_ai::AwarenessConfig::default().noise_cap;
    // Behind the wall 10 m ahead: inside the run reach (12 m), outside the sneak reach (2 m).
    let mut app = room();
    let g = guard(&mut app);
    let p = player(&mut app, -2.25, Gait::Run);
    let run = walk(&mut app, p, s.run.stride_m, 16.0, 48);
    let heard_run = level(&app, g);
    assert_eq!(run.len(), 3);
    assert!(heard_run > 0.0 && heard_run <= cap, "running player unheard: {heard_run}");

    let mut app = room();
    let g = guard(&mut app);
    let p = player(&mut app, -0.5, Gait::Sneak);
    let sneak = walk(&mut app, p, s.sneak.stride_m, 32.0, 96);
    assert_eq!(sneak.len(), 3);
    assert!(level(&app, g) <= 0.0, "sneaking player heard: {}", level(&app, g));
    println!("guard 10 m away behind a wall: run level {heard_run} (cap {cap}), sneak level 0");
}
