//! `IN11b`: moving platforms carry characters in a Bevy app, headless, on real physics. A
//! kinematic box (a sliding platform, a lift going up then down) is driven by per-tick targets
//! before the physics step; a mover-driven character (the NPC path, `ControllerSystems::Step`)
//! and an action-driven body (the player path, `ActionSystems::Move`) stand idle on it. The
//! physics step runs before both (`crate::character`), and their ground velocity is re-read
//! after it ([`crate::refresh_base_velocity`]), so each keeps its platform-local spot through
//! the base starting from rest, moving and reversing: no tick of lead or lag, no sinking into
//! the lift, no hop off it. Run with `--nocapture` for the measured drift.

#![allow(clippy::cast_precision_loss)]

use std::time::Duration;

use bevy_app::{App, FixedPostUpdate, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{ActionState, BASE_RULES, CharacterId, CharacterParams, RuleKey, RuleSet, SlotInfo, SlotTable};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionRules, ActionSystems, SfActionPlugin};
use sf_controller::{GroundBody, Locomotion};
use sf_controller_bevy::{MoveIntent, Mover, MoverParams, SfControllerPlugin};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, BodyId, BodyKind, Layer, ShapeRef};
use sf_world::WorldspaceId;
use sf_world_bevy::ActiveWorldspace;

use crate::{Physics, PhysicsPlugin, PhysicsSystems, refresh_base_velocity};

const HZ: f64 = 60.0;
/// The platform's half extents, m: its top is `HALF.y` above its centre.
const HALF: Vec3 = Vec3::new(3.0, 0.25, 3.0);
/// At rest this long first (the characters land and settle), s.
const REST: f32 = 0.5;

fn centre() -> WorldPos {
    WorldPos::new(IVec3::new(1, 0, -1), Vec3::new(4.0, 2.0, -6.0))
}

/// The platform's displacement from [`centre`] at time `t` (s).
type Path = fn(f32) -> Vec3;

/// Slides at (2, 0, 1) m/s after [`REST`].
fn slide(t: f32) -> Vec3 {
    Vec3::new(2.0, 0.0, 1.0) * (t - REST).max(0.0)
}

/// Lift speed, m/s, and the time it goes up, then down, s.
const LIFT: f32 = 1.5;
const LEG: f32 = 1.5;

/// Up at [`LIFT`] for [`LEG`] after [`REST`], then down as long, then still.
fn lift(t: f32) -> Vec3 {
    let s = (t - REST).clamp(0.0, 2.0 * LEG);
    Vec3::Y * LIFT * if s < LEG { s } else { 2.0 * LEG - s }
}

#[derive(Resource)]
struct Platform {
    ws: WorldspaceId,
    id: BodyId,
    path: Path,
}

/// The host's base driver: this step's end pose, before the step.
#[allow(clippy::needless_pass_by_value)]
fn drive(time: Res<Time<Fixed>>, platform: Res<Platform>, mut physics: ResMut<Physics>) {
    #[allow(clippy::cast_possible_truncation)]
    let t = time.elapsed_secs_f64() as f32;
    let w = physics.world_mut(&platform.ws).expect("world");
    assert!(w.set_kinematic_target(platform.id, WorldTransform::from_pos(centre().offset((platform.path)(t)))));
}

/// The host's part for action bodies (the framework refreshes the movers).
#[allow(clippy::needless_pass_by_value)]
fn action_bases(physics: Res<Physics>, mut bodies: Query<&mut ActionBody>) {
    let Some(w) = physics.worlds.active_world() else { return };
    for mut b in &mut bodies {
        if b.loco.grounded.is_some_and(|g| g.body.is_some()) {
            refresh_base_velocity(w, &mut b.loco);
        }
    }
}

/// One tick: time, the platform's top centre, the mover's and the action body's feet, and
/// whether each stood on the platform.
type Sample = (f32, WorldPos, [(WorldPos, bool); 2]);

#[derive(Resource, Default)]
struct Samples(Vec<Sample>);

#[derive(Resource)]
struct Characters([Entity; 2]);

fn on(loco: &Locomotion, id: BodyId) -> (WorldPos, bool) {
    (loco.position, loco.grounded.is_some_and(|g| g.body == Some(GroundBody::from(id))))
}

#[allow(clippy::needless_pass_by_value)]
fn record(
    time: Res<Time<Fixed>>,
    platform: Res<Platform>,
    physics: Res<Physics>,
    who: Res<Characters>,
    movers: Query<&Mover>,
    bodies: Query<&ActionBody>,
    mut samples: ResMut<Samples>,
) {
    let pose = physics.world(&platform.ws).and_then(|w| w.pose(platform.id)).expect("platform");
    let mover = on(&movers.get(who.0[0]).expect("mover").loco, platform.id);
    let body = on(&bodies.get(who.0[1]).expect("action body").loco, platform.id);
    #[allow(clippy::cast_possible_truncation)]
    let t = time.elapsed_secs_f64() as f32;
    samples.0.push((t, pose.pos.offset(Vec3::Y * HALF.y), [mover, body]));
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

/// Runs `seconds` with both characters idle on the platform following `path`.
fn ride(path: Path, seconds: f32) -> Vec<Sample> {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let ws = WorldspaceId::new("platform");
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default(), SfControllerPlugin::<Physics>::default(), SfActionPlugin::<Physics>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .insert_resource(rules())
        .init_resource::<Samples>()
        // The host's order for its action bodies (the movers' is the framework's).
        .configure_sets(FixedUpdate, PhysicsSystems::Step.before(ActionSystems::Sample))
        .add_systems(FixedUpdate, drive.before(PhysicsSystems::Hands))
        .add_systems(FixedUpdate, action_bases.after(PhysicsSystems::Step).before(ActionSystems::Move))
        // At the tick's end, whatever the order inside it.
        .add_systems(FixedPostUpdate, record);
    let desc = BodyDesc::new(BodyKind::Kinematic, WorldTransform::from_pos(centre()), ShapeRef::Box(HALF), Layer::STATIC);
    let id = app.world_mut().resource_mut::<Physics>().insert(&ws, &desc);
    app.insert_resource(Platform { ws, id, path });
    let top = centre().offset(Vec3::Y * HALF.y);
    // Dropped from 5 cm onto opposite halves of the platform, facing different ways.
    let feet = |x: f32| top.offset(Vec3::new(x, 0.05, 0.5));
    let mover = app.world_mut().spawn((Mover::new(feet(-1.2), 0.3, MoverParams::default(), centre()), MoveIntent::default(), Transform::default())).id();
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(feet(1.2), -0.8), MoverParams::default(), centre());
    let body = app.world_mut().spawn((character, body, Transform::default())).id();
    app.insert_resource(Characters([mover, body]));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    for _ in 0..(f64::from(seconds) * HZ).round() as usize {
        app.update();
    }
    let samples = std::mem::take(&mut app.world_mut().resource_mut::<Samples>().0);
    let &(t, top, _) = samples.last().expect("ticks ran");
    let want = centre().offset(path(t) + Vec3::Y * HALF.y);
    assert!(top.delta_f32(want).length() < 1e-4, "the platform is {} m off its path", top.delta_f32(want));
    samples
}

/// Per character: the largest horizontal and vertical departure from its platform-local spot
/// (as it stood just before the platform moved), and the lowest and highest feet height over
/// the top, from [`REST`] on, m.
fn drift(samples: &[Sample]) -> [(f32, f32, f32, f32); 2] {
    let settled = samples.iter().rposition(|s| s.0 < REST - 0.05).expect("rest samples");
    let (_, top0, ref0) = samples[settled];
    let mut out = [(0.0_f32, 0.0_f32, f32::MAX, f32::MIN); 2];
    for (t, top, chars) in &samples[settled..] {
        for (i, (feet, on)) in chars.iter().enumerate() {
            let local = top.delta_f32(*feet);
            let local0 = top0.delta_f32(ref0[i].0);
            assert!(local.is_finite(), "t {t}: character {i} NaN");
            assert!(*on, "t {t}: character {i} not standing on the platform ({local})");
            let d = local - local0;
            let o = &mut out[i];
            *o = (o.0.max(d.with_y(0.0).length()), o.1.max(d.y.abs()), o.2.min(local.y), o.3.max(local.y));
        }
    }
    out
}

fn report(name: &str, d: &[(f32, f32, f32, f32); 2]) {
    for (who, (h, v, lo, hi)) in ["mover", "action body"].iter().zip(d) {
        println!("{name} {who}: horizontal drift {h:.2e} m, vertical {v:.2e} m, feet over the top {lo:.2e}..{hi:.2e} m");
    }
}

/// Sliding at 2.2 m/s from rest for 3 s: both keep their spot to well under a tick of travel
/// (3.7 cm: a mover stepped before the physics step, or riding the last cast's velocity, is
/// off by one or two of those from the start on).
#[test]
fn idle_characters_keep_their_spot_on_a_sliding_platform() {
    let samples = ride(slide, REST + 3.0);
    let travel = slide(REST + 1.0 / 60.0).length();
    let d = drift(&samples);
    report("slide", &d);
    for (i, (h, v, lo, _)) in d.iter().enumerate() {
        assert!(*h < travel / 4.0, "character {i} drifted {h} m on the platform (a tick of travel is {travel} m)");
        assert!(*v < 0.01 && *lo > -0.01, "character {i} off the deck: {v} m, lowest {lo} m");
    }
}

/// A lift up at 1.5 m/s for 1.5 s, then down as long: both ride it without sinking into it at
/// the start, the turn or the stop, nor lifting off it when it drops.
#[test]
fn idle_characters_ride_a_lift_up_and_down() {
    let samples = ride(lift, REST + 2.0 * LEG + 0.5);
    let travel = LIFT / 60.0;
    let d = drift(&samples);
    report("lift", &d);
    for (i, (h, _, lo, hi)) in d.iter().enumerate() {
        assert!(*h < travel / 4.0, "character {i} slid {h} m on the lift");
        assert!(*lo > -travel / 2.0, "character {i} sank {lo} m into the lift (a tick of travel is {travel} m)");
        assert!(*hi < travel / 2.0, "character {i} left the lift by {hi} m");
    }
}

// IN11c2a: a moving wall pushes idle characters standing on static ground.

/// The wall's half extents, m: thin (0.2 m), so a 10 m/s wall at 30 Hz (0.33 m a tick)
/// ends a tick past the capsule's centre and must still push from the face it came in by.
const WALL: Vec3 = Vec3::new(0.1, 1.0, 3.0);

/// The wall: its start pose, its kind and its speed along +x after [`REST`].
#[derive(Resource)]
struct Wall {
    ws: WorldspaceId,
    id: BodyId,
    start: WorldPos,
    speed: f32,
    kinematic: bool,
}

#[allow(clippy::needless_pass_by_value)]
fn drive_wall(time: Res<Time<Fixed>>, wall: Res<Wall>, mut physics: ResMut<Physics>) {
    if !wall.kinematic {
        return;
    }
    #[allow(clippy::cast_possible_truncation)]
    let t = time.elapsed_secs_f64() as f32;
    let at = wall.start.offset(Vec3::X * wall.speed * (t - REST).max(0.0));
    let w = physics.world_mut(&wall.ws).expect("world");
    assert!(w.set_kinematic_target(wall.id, WorldTransform::from_pos(at)));
}

/// One tick: time, a point on the wall's +x face, and per character its feet and whether
/// it stood on the ground.
type WallSample = (f32, WorldPos, [(WorldPos, bool); 2]);

#[derive(Resource, Default)]
struct WallSamples(Vec<WallSample>);

#[allow(clippy::needless_pass_by_value)]
fn record_wall(
    time: Res<Time<Fixed>>,
    wall: Res<Wall>,
    physics: Res<Physics>,
    who: Res<Characters>,
    movers: Query<&Mover>,
    bodies: Query<&ActionBody>,
    mut samples: ResMut<WallSamples>,
) {
    let pose = physics.world(&wall.ws).and_then(|w| w.pose(wall.id)).expect("wall");
    let stood = |l: &Locomotion| (l.position, l.grounded.is_some());
    let mover = stood(&movers.get(who.0[0]).expect("mover").loco);
    let body = stood(&bodies.get(who.0[1]).expect("action body").loco);
    #[allow(clippy::cast_possible_truncation)]
    let t = time.elapsed_secs_f64() as f32;
    samples.0.push((t, pose.pos.offset(Vec3::X * WALL.x), [mover, body]));
}

/// Runs `seconds` at `hz` with both characters idle on static ground and a wall whose +x
/// face is `behind` m behind their feet (−x), kinematic at `speed` along +x or fixed.
fn walled(hz: f64, speed: f32, kinematic: bool, behind: f32, seconds: f32) -> Vec<WallSample> {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let ws = WorldspaceId::new("wall");
    #[allow(clippy::cast_possible_truncation)]
    let params = sf_physics::PhysicsParams { dt: (1.0 / hz) as f32, ..sf_physics::PhysicsParams::default() };
    let settings = crate::PhysicsSettings { hz, ..crate::PhysicsSettings::default() };
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin { params, settings }, SfControllerPlugin::<Physics>::default(), SfActionPlugin::<Physics>::default()))
        // The action plugin sets its own 60 Hz tick: one fixed step a frame at `hz` instead.
        .insert_resource(Time::<Fixed>::from_hz(hz))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / hz)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .insert_resource(rules())
        .init_resource::<WallSamples>()
        .configure_sets(FixedUpdate, PhysicsSystems::Step.before(ActionSystems::Sample))
        .add_systems(FixedUpdate, drive_wall.before(PhysicsSystems::Hands))
        .add_systems(FixedPostUpdate, record_wall);
    // Ground: a fixed slab 40 m along x, its top at the centre's height.
    let slab = WorldTransform::from_pos(centre().offset(Vec3::new(17.0, -0.25, 0.0)));
    app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(BodyKind::Fixed, slab, ShapeRef::Box(Vec3::new(20.0, 0.25, 6.0)), Layer::STATIC));
    // Feet at x = 0, the characters at z = ∓1.2; the wall's bottom 5 cm over the ground.
    let start = centre().offset(Vec3::new(-behind - WALL.x, WALL.y + 0.05, 0.0));
    let kind = if kinematic { BodyKind::Kinematic } else { BodyKind::Fixed };
    let id = app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(kind, WorldTransform::from_pos(start), ShapeRef::Box(WALL), Layer::STATIC));
    app.insert_resource(Wall { ws, id, start, speed, kinematic });
    let feet = |z: f32| centre().offset(Vec3::new(0.0, 0.05, z));
    let mover = app.world_mut().spawn((Mover::new(feet(-1.2), 0.3, MoverParams::default(), centre()), MoveIntent::default(), Transform::default())).id();
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(feet(1.2), -0.8), MoverParams::default(), centre());
    let body = app.world_mut().spawn((character, body, Transform::default())).id();
    app.insert_resource(Characters([mover, body]));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    for _ in 0..(f64::from(seconds) * hz).round() as usize {
        app.update();
    }
    std::mem::take(&mut app.world_mut().resource_mut::<WallSamples>().0)
}

/// Per character, over the ticks after the wall reached it (its face within the skin of
/// the capsule, plus two ticks): the lowest and highest gap between the face and the
/// capsule, m, and the character's mean speed along +x, m/s. Asserts it stayed finite and
/// ahead of the wall throughout (no tunnelling), and grounded once landed.
fn pushed(samples: &[WallSample], radius: f32, skin: f32) -> [(f32, f32, f32); 2] {
    let mut out = [(0.0_f32, 0.0_f32, 0.0_f32); 2];
    for (i, o) in out.iter_mut().enumerate() {
        let gap = |s: &WallSample| s.1.delta_f32(s.2[i].0).x - radius;
        for s in samples {
            assert!(s.1.delta_f32(s.2[i].0).is_finite(), "t {}: character {i} NaN", s.0);
            assert!(s.0 < REST || s.2[i].1, "t {}: character {i} left the ground", s.0);
            assert!(gap(s) > -radius, "t {}: character {i} tunnelled into the wall (gap {} m)", s.0, gap(s));
        }
        let reached = samples.iter().position(|s| s.0 > REST && gap(s) <= skin + 1e-3).expect("the wall reached the character") + 2;
        let window = &samples[reached..];
        let (lo, hi) = window.iter().map(gap).fold((f32::MAX, f32::MIN), |(lo, hi), g| (lo.min(g), hi.max(g)));
        let (first, last) = (&window[0], window.last().expect("ticks after contact"));
        *o = (lo, hi, first.2[i].0.delta_f32(last.2[i].0).x / (last.0 - first.0));
    }
    out
}

/// A kinematic wall at 2 and 10 m/s, at 30 and 144 Hz, pushes both idle characters (the
/// NPC mover and the player's action body) ahead of it at its speed, at the skin from its
/// face; at 10 m/s and 30 Hz it moves more than its thickness a tick, so without the face
/// it came in by it would pass through them.
#[test]
fn a_moving_wall_pushes_idle_characters() {
    let params = MoverParams::default();
    for hz in [30.0, 144.0] {
        for speed in [2.0, 10.0] {
            let samples = walled(hz, speed, true, 1.0, REST + 1.5);
            for (who, (lo, hi, v)) in ["mover", "action body"].iter().zip(pushed(&samples, params.radius, params.skin)) {
                println!("{hz} Hz, {speed} m/s, {who}: gap {lo:.5}..{hi:.5} m (skin {}), speed {v:.4} m/s", params.skin);
                assert!((v - speed).abs() < 0.01 * speed, "{hz} Hz {speed} m/s: {who} pushed at {v} m/s");
                assert!(lo > -1e-3 && hi < params.skin + 1e-3, "{hz} Hz {speed} m/s: {who} gap {lo}..{hi} m");
            }
        }
    }
}

/// A fixed wall 5 mm from both capsules (inside the skin) never pushes them.
#[test]
fn a_static_wall_never_pushes() {
    let params = MoverParams::default();
    let samples = walled(60.0, 0.0, false, params.radius + 0.005, 0.5);
    let (first, last) = (&samples[0], samples.last().expect("ticks ran"));
    for i in 0..2 {
        let d = first.2[i].0.delta_f32(last.2[i].0).with_y(0.0).length();
        println!("static wall, character {i}: moved {d:.2e} m");
        assert!(d < 1e-4, "character {i} moved {d} m by a static wall");
    }
}
