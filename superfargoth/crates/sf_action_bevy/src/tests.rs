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
pub(crate) struct Plane;

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

#[test]
fn no_clip_floats_without_gravity_and_ignores_the_ground() {
    let mut app = app(60);
    let up = WorldPos::ORIGIN.offset(Vec3::Y * 2.0);
    let ghost = spawn(&mut app);
    let walker = spawn(&mut app);
    for e in [ghost, walker] {
        app.world_mut().get_mut::<ActionBody>(e).unwrap().teleport(up, 0.0);
    }
    app.world_mut().entity_mut(ghost).insert(NoClip { look: Vec3::NEG_Z });
    for _ in 0..60 {
        app.update();
    }
    let height = |app: &App, e| WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(e).unwrap().loco.position).y;
    assert!((height(&app, ghost) - 2.0).abs() < 1e-3, "no-clip floats: {}", height(&app, ghost));
    assert!(height(&app, walker) < 0.1, "the walker fell to the plane: {}", height(&app, walker));
    assert!((no_clip_climb(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, -1.0, -1.0)) + 3.0).abs() < 1e-4);
}

#[test]
fn a_body_awaiting_ground_holds_its_pose_then_falls_when_released() {
    let mut app = app(60);
    let up = WorldPos::ORIGIN.offset(Vec3::Y * 2.0);
    let body = spawn(&mut app);
    app.world_mut().get_mut::<ActionBody>(body).unwrap().teleport(up, 0.0);
    app.world_mut().entity_mut(body).insert(AwaitingGround);
    let height = |app: &App| WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(body).unwrap().loco.position).y;
    for _ in 0..30 {
        app.update();
        assert!((height(&app) - 2.0).abs() < 1e-4, "held body moved: {}", height(&app));
    }
    app.world_mut().entity_mut(body).remove::<AwaitingGround>();
    for _ in 0..60 {
        app.update();
    }
    let h = height(&app);
    assert!(h.is_finite() && h < 0.1 && h > -0.1, "released body lands on the plane: {h}");
}

/// Every sampled tick's (Dodge pressed, Sprint held).
#[derive(Resource, Default)]
struct Shared(Vec<(bool, bool)>);

fn record_shared(mut seen: ResMut<Shared>, characters: Query<&ActionCharacter>) {
    for c in &characters {
        seen.0.push((c.command.pressed(ActionClass::Dodge), c.command.held(ActionClass::Sprint)));
    }
}

/// The shared Dodge/Sprint button through a real key binding, held across frames (CB-1: Shift
/// held never sprinted).
fn shared_button_ticks(fps: u32, held_frames: usize) -> Vec<(bool, bool)> {
    use bevy_enhanced_input::prelude::*;
    use bevy_input::{ButtonInput, keyboard::KeyCode};
    let mut app = app(fps);
    app.add_plugins((bevy_input::InputPlugin, EnhancedInputPlugin, ActionInputPlugin))
        .init_resource::<Shared>()
        .add_systems(FixedUpdate, record_shared.after(ActionSystems::Sample).before(ActionSystems::Step));
    app.finish();
    app.cleanup();
    let e = spawn(&mut app);
    app.world_mut().entity_mut(e).insert((PlayerControls, PlayerInput::new(PlayerId::new(0).unwrap())));
    let actions = {
        let mut commands = app.world_mut().commands();
        spawn_player_actions(&mut commands, e)
    };
    let shared = actions.iter().find(|(t, _)| *t == PlayerAction::DodgeSprint).unwrap().1;
    app.world_mut().spawn((BindingOf(shared), Binding::from(KeyCode::ShiftLeft)));
    app.world_mut().flush();
    for _ in 0..5 {
        app.update();
    }
    app.world_mut().resource_mut::<Shared>().0.clear();
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ShiftLeft);
    for _ in 0..held_frames {
        app.update();
    }
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(KeyCode::ShiftLeft);
    for _ in 0..10 {
        app.update();
    }
    std::mem::take(&mut app.world_mut().resource_mut::<Shared>().0)
}

#[test]
fn a_held_dodge_sprint_key_sprints_and_a_tap_rolls() {
    for fps in [30, 60, 144] {
        // Held one second: Sprint, never a Dodge.
        let held = shared_button_ticks(fps, fps as usize);
        assert!(held.iter().all(|&(dodge, _)| !dodge), "{fps} fps: held key rolled: {held:?}");
        // Sprint stays down from the 18th tick until release (1 s = 60 ticks): one run, not a blip.
        let sprint_ticks = held.iter().filter(|&&(_, sprint)| sprint).count();
        let first = held.iter().position(|&(_, sprint)| sprint);
        let run = first.map_or(0, |i| held[i..].iter().take_while(|&&(_, sprint)| sprint).count());
        assert!(sprint_ticks >= 30 && run == sprint_ticks, "{fps} fps: held key didn't keep sprinting: {held:?}");
        // Tapped (~0.1 s): one Dodge, no Sprint.
        let tap = shared_button_ticks(fps, (fps as usize / 10).max(1));
        assert_eq!(tap.iter().filter(|&&(dodge, _)| dodge).count(), 1, "{fps} fps: tap: {tap:?}");
        assert!(tap.iter().all(|&(_, sprint)| !sprint), "{fps} fps: tap sprinted: {tap:?}");
    }
}

/// A sit-down stand-in for the headless seat test: half a metre forward while turning round.
struct SitDown;

impl sf_action::RootPath for SitDown {
    fn duration(&self) -> f32 {
        0.5
    }

    fn root_at(&self, t: f32) -> sf_anim::RootPose {
        let u = (t / 0.5).clamp(0.0, 1.0);
        sf_anim::RootPose::new(Vec3::new(0.0, 0.0, 0.5) * u, std::f32::consts::PI * u)
    }
}

/// Every seat event the character sent, by tick.
#[derive(Resource, Default)]
struct SeatEvents(Vec<sf_action::ActionEvent>, Vec<RuleKey>);

fn write_idle(tick: Res<ActionTick>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        c.command = Command::empty(tick.0);
    }
}

fn record_seat(mut seen: ResMut<SeatEvents>, characters: Query<&ActionCharacter>) {
    for c in &characters {
        seen.0.extend(c.output.events.iter().filter(|e| matches!(e.kind, sf_action::EventKind::SeatEntered(_) | sf_action::EventKind::SeatRefused(_))));
        seen.1.push(c.state.mov);
    }
}

/// `IN2b1`: the interaction shell's claimed slot (`SeatAim`) is the request: with no press, the
/// grounded character starts the enter the tick it appears and sits once the clip lands.
#[test]
fn a_seat_aim_sits_with_no_press() {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with("interact.sit.enter", SlotInfo::once(0.5))
        .with("interact.sit.loop", SlotInfo::looping())
        .with("interact.sit.exit", SlotInfo::once(0.5))
        .with_root("interact.sit.enter", std::sync::Arc::new(SitDown));
    let mut b = sf_action::RuleSetBuilder::new();
    b.add_ron(&sf_action::Origin::Framework, BASE_RULES).add_ron(&sf_action::Origin::Framework, sf_action::SEAT_RULES);
    let rules = ActionRules { rules: b.build().expect("base and seat rules"), slots: Box::new(slots) };
    let mut app = app(60);
    app.insert_resource(rules)
        .init_resource::<SeatEvents>()
        .add_systems(FixedUpdate, write_idle.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, record_seat.after(ActionSystems::Step));
    let e = spawn(&mut app);
    let pi = std::f32::consts::PI;
    let seat = SeatSense {
        target: sf_action::TargetId(9),
        slot: 2,
        enter: sf_anim::RootPose::new(Vec3::new(0.0, 0.0, 0.5), pi),
        exit: sf_anim::RootPose::new(Vec3::ZERO, pi),
    };
    // The claim lands once the body stands on the ground (the shell's pipeline runs after a press).
    while app.world().resource::<SeatEvents>().1.len() < 10 {
        app.update();
    }
    app.world_mut().entity_mut(e).insert(SeatAim(Some(seat)));
    let (sit, enter) = (RuleKey::of("interact.sit.loop"), RuleKey::of("interact.sit.enter"));
    for _ in 0..200 {
        app.update();
        if app.world().resource::<SeatEvents>().1.contains(&sit) {
            break;
        }
    }
    let seen = app.world().resource::<SeatEvents>();
    assert!(seen.1.contains(&enter), "never entered: {:?}", seen.1);
    let loop_at = seen.1.iter().position(|&k| k == sit).expect("sat within the ticks");
    assert!(loop_at <= 10 + 40, "sat after {loop_at} ticks");
    let entered: Vec<_> = seen.0.iter().filter(|e| e.kind == sf_action::EventKind::SeatEntered(2)).collect();
    assert_eq!(entered.len(), 1, "{:?}", seen.0);
    assert_eq!(entered[0].target, Some(seat.target));
    assert!(!seen.0.iter().any(|e| matches!(e.kind, sf_action::EventKind::SeatRefused(_))), "{:?}", seen.0);
}
