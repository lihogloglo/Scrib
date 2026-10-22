//! Headless app tests: the fixed tick at render rates 30/60/144 fps (action.md A2 verify).

use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, RuleKey, RuleSet, SlotInfo, SlotTable, Tick};
use sf_controller::{GroundHit, GroundProbe, HeightSurface, Locomotion, MoverParams, ProbeQuery, TerrainProbe};
use sf_coords::WorldPos;

use crate::*;

/// Flat ground at y = 0.
struct Plane;

impl HeightSurface for Plane {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let t = -WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
        (dir.y.abs() > 1e-6 && (0.0..=max).contains(&t)).then_some((t, Vec3::Y))
    }
}

#[derive(Resource)]
struct Flat(TerrainProbe<Plane>);

impl GroundProbe for Flat {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    ActionRules { rules: RuleSet::from_ron(BASE_RULES).expect("base rules"), slots: Box::new(slots) }
}

fn app(fps: u32) -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<Flat>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / f64::from(fps))))
        .insert_resource(Flat(TerrainProbe(Plane)))
        .insert_resource(rules());
    app
}

fn spawn(app: &mut App) -> Entity {
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(WorldPos::ORIGIN, 0.0), MoverParams::default(), WorldPos::ORIGIN);
    app.world_mut().spawn((character, body, Transform::default())).id()
}

/// The scripted command stream, by tick: forward, a jump at tick 40, right from tick 90, stop
/// at tick 150.
fn script(tick: Tick) -> Command {
    let mut cmd = Command::empty(tick);
    cmd.stick = match tick.0 {
        0..90 => Vec2::Y,
        90..150 => Vec2::X,
        _ => Vec2::ZERO,
    };
    if tick.0 == 40 {
        cmd.buttons.pressed.insert(ActionClass::Jump);
        cmd.buttons.held.insert(ActionClass::Jump);
    }
    if tick.0 == 41 {
        cmd.buttons.released.insert(ActionClass::Jump);
    }
    cmd
}

/// Writes the script for characters without a local player (AI, scripts, net: `Command` set).
fn write_script(tick: Res<ActionTick>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        c.command = script(tick.0);
    }
}

/// Every gameplay tick's result, bit for bit.
#[derive(Resource, Default)]
struct Ticks(Vec<(u32, [u32; 5], RuleKey)>);

fn record(tick: Res<ActionTick>, mut ticks: ResMut<Ticks>, bodies: Query<(&ActionCharacter, &ActionBody)>) {
    for (c, b) in &bodies {
        let p = WorldPos::ORIGIN.delta_f32(b.loco.position);
        let bits = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits(), b.loco.motion.yaw.to_bits(), b.loco.vertical_speed.to_bits()];
        ticks.0.push((tick.0.0, bits, c.state.mov));
    }
}

fn run_script(fps: u32, ticks: usize) -> Vec<(u32, [u32; 5], RuleKey)> {
    let mut app = app(fps);
    app.init_resource::<Ticks>()
        .add_systems(FixedUpdate, write_script.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, record.after(ActionSystems::Move));
    spawn(&mut app);
    let mut frames = 0;
    while app.world().resource::<Ticks>().0.len() < ticks {
        app.update();
        frames += 1;
        assert!(frames < 10_000, "the fixed loop doesn't run");
    }
    let mut out = std::mem::take(&mut app.world_mut().resource_mut::<Ticks>().0);
    out.truncate(ticks);
    out
}

#[test]
fn gameplay_ticks_are_identical_at_30_60_and_144_fps() {
    let reference = run_script(60, 180);
    // The script did something: it jumped and it moved.
    assert!(reference.iter().any(|(_, b, _)| f32::from_bits(b[4]) > 1.0), "no jump");
    let end = reference.last().unwrap().1;
    assert!(f32::from_bits(end[2]) > 3.0 && f32::from_bits(end[0]).abs() > 1.0, "end {:?}", end.map(f32::from_bits));
    for fps in [30, 144] {
        let other = run_script(fps, 180);
        let first = reference.iter().zip(&other).position(|(a, b)| a != b);
        assert!(first.is_none(), "{fps} fps diverges at tick {first:?}");
    }
}

/// A straight walk drawn at `fps`: the rendered transform per frame, after the warm-up.
fn rendered_walk(fps: u32) -> Vec<Vec3> {
    let mut app = app(fps);
    app.add_systems(FixedUpdate, write_forward.in_set(ActionSystems::Command));
    let e = spawn(&mut app);
    let warmup = fps as usize * 3 / 2;
    let measured = fps as usize;
    let mut out = Vec::with_capacity(measured);
    for frame in 0..warmup + measured {
        app.update();
        if frame >= warmup {
            out.push(app.world().get::<Transform>(e).unwrap().translation);
        }
    }
    out
}

fn write_forward(tick: Res<ActionTick>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        c.command = Command { stick: Vec2::Y, ..Command::empty(tick.0) };
    }
}

#[test]
fn the_rendered_walk_is_monotonic_and_jitter_free_at_30_60_and_144_fps() {
    for fps in [30_u16, 60, 144] {
        let points = rendered_walk(u32::from(fps));
        let dir = (*points.last().unwrap() - points[0]).normalize();
        let steps: Vec<f32> = points.windows(2).map(|w| (w[1] - w[0]).dot(dir)).collect();
        let n = f32::from(u16::try_from(steps.len()).unwrap());
        let mean = steps.iter().sum::<f32>() / n;
        let var = steps.iter().map(|s| (s - mean).powi(2)).sum::<f32>() / n;
        // Run speed (4.5 m/s) over one frame.
        let expected = 4.5 / f32::from(fps);
        assert!((mean - expected).abs() < 0.02 * expected, "{fps} fps: mean step {mean}, expected {expected}");
        assert!(steps.iter().all(|&s| s > 0.0), "{fps} fps: not monotonic");
        // Jitter gate: per-frame displacement within 1 % (coefficient of variation). Without
        // interpolation 144 fps draws 0 or 1 tick per frame and fails both gates (checked).
        let cv = var.sqrt() / mean;
        assert!(cv < 0.01, "{fps} fps: step CV {cv}");
    }
}

#[test]
fn reconciliation_glides_through_the_correction_spring() {
    let mut app = app(60);
    let e = spawn(&mut app);
    for _ in 0..10 {
        app.update();
    }
    let before = app.world().get::<Transform>(e).unwrap().translation;
    let mut body = app.world_mut().get_mut::<ActionBody>(e).unwrap();
    let mut snapped = body.loco.clone();
    snapped.position = snapped.position.offset(Vec3::X * 0.3);
    body.reconcile(snapped);
    app.update();
    let after = app.world().get::<Transform>(e).unwrap().translation;
    // The drawn body barely moved on the snap frame and has caught up a second later.
    assert!((after - before).length() < 0.05, "popped {:?}", after - before);
    for _ in 0..90 {
        app.update();
    }
    let settled = app.world().get::<Transform>(e).unwrap().translation;
    assert!((settled.x - 0.3).abs() < 1e-3, "{settled:?}");
}

#[test]
fn enhanced_input_actions_reach_the_command_and_jump() {
    use bevy_enhanced_input::prelude::*;
    let mut app = app(60);
    app.add_plugins((bevy_input::InputPlugin, EnhancedInputPlugin, ActionInputPlugin));
    // `EnhancedInputPlugin` creates its context registry in `finish` (`App::run` calls it).
    app.finish();
    app.cleanup();
    let e = spawn(&mut app);
    app.world_mut().entity_mut(e).insert((PlayerControls, PlayerInput::new(PlayerId::new(0).unwrap())));
    let actions = {
        let mut commands = app.world_mut().commands();
        spawn_player_actions(&mut commands, e)
    };
    app.world_mut().flush();
    for _ in 0..5 {
        app.update();
    }
    let find = |a: PlayerAction| actions.iter().find(|(t, _)| *t == a).unwrap().1;
    let stick = find(PlayerAction::Stick);
    let jump = find(PlayerAction::Button(ActionClass::Jump));
    app.world_mut().entity_mut(stick).insert(ActionMock::new(TriggerState::Fired, Vec2::Y, MockSpan::Manual));
    app.world_mut().entity_mut(jump).insert(ActionMock::once(TriggerState::Fired, true));
    let mut jumped = false;
    for _ in 0..30 {
        app.update();
        let body = app.world().get::<ActionBody>(e).unwrap();
        jumped |= body.loco.vertical_speed > 1.0;
    }
    let c = app.world().get::<ActionCharacter>(e).unwrap();
    assert_eq!(c.command.player, Some(PlayerId::new(0).unwrap()));
    assert!(c.command.stick.y > 0.99, "{:?}", c.command.stick);
    assert!(jumped, "the mocked Jump never launched");
}
