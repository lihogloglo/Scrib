//! `IN11b`: moving platforms carry characters in a Bevy app, headless, on real physics. A
//! kinematic box (a sliding platform, a lift going up then down) is driven by per-tick targets
//! before the physics step; a mover-driven character (the NPC path, `ControllerSystems::Step`)
//! and an action-driven body (the player path, `ActionSystems::Move`) stand idle on it. The
//! physics step runs before both (`crate::character`), and their ground velocity is re-read
//! after it ([`crate::refresh_base_motion`]), so each keeps its platform-local spot through
//! the base starting from rest, moving and reversing: no tick of lead or lag, no sinking into
//! the lift, no hop off it. `IN11c2b`: on a sliding and turning base, a tick whose physics step
//! is dropped and one that steps twice leave them on their spot (the ride is the base's pose
//! delta, [`crate::refresh_base_motion`]). `IN11c2c1`: a lift rising into a ceiling leaves them
//! under it (the vertical ride is swept). `IN11c2c2`: a lift squeezing its riders into the ceiling
//! and a cart pushing characters into a wall crush each once ([`CharacterCrushed`]), their pose
//! held where the world lets them be. Run with `--nocapture` for the measured drift and depths.

#![allow(clippy::cast_precision_loss)]

use std::time::Duration;

use bevy_app::{App, FixedPostUpdate, FixedUpdate, Last};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Quat, Vec3};
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

use crate::{CharacterCrushed, Physics, PhysicsPlugin, PhysicsSettings, PhysicsStats, PhysicsSystems, crushed, refresh_base_motion};

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

/// The platform's yaw at time `t` (s), radians.
type Spin = fn(f32) -> f32;

fn still(_: f32) -> f32 {
    0.0
}

/// Turns at 0.6 rad/s after [`REST`].
fn turn(t: f32) -> f32 {
    0.6 * (t - REST).max(0.0)
}

/// How the platform moves, and the ticks (counted from 1) whose physics step is dropped or
/// runs twice.
#[derive(Clone, Copy)]
struct Motion {
    path: Path,
    spin: Spin,
    dropped: &'static [u32],
    doubled: &'static [u32],
}

impl Motion {
    fn along(path: Path) -> Self {
        Self { path, spin: still, dropped: &[], doubled: &[] }
    }

    fn pose(&self, t: f32) -> WorldTransform {
        WorldTransform { rotation: Quat::from_rotation_y((self.spin)(t)), ..WorldTransform::from_pos(centre().offset((self.path)(t))) }
    }
}

#[derive(Resource)]
struct Platform {
    ws: WorldspaceId,
    id: BodyId,
    motion: Motion,
}

#[allow(clippy::cast_possible_truncation)]
fn now(time: &Time<Fixed>) -> f32 {
    time.elapsed_secs_f64() as f32
}

/// The tick being run, from 1.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn tick(time: &Time<Fixed>) -> u32 {
    (time.elapsed_secs_f64() * HZ).round() as u32
}

/// The host's base driver: this step's end pose, before the step.
#[allow(clippy::needless_pass_by_value)]
fn drive(time: Res<Time<Fixed>>, platform: Res<Platform>, mut physics: ResMut<Physics>) {
    let w = physics.world_mut(&platform.ws).expect("world");
    assert!(w.set_kinematic_target(platform.id, platform.motion.pose(now(&time))));
}

/// A frame that already ran its steps: the plugin drops this tick's step (the movers run).
#[allow(clippy::needless_pass_by_value)]
fn drop_step(time: Res<Time<Fixed>>, platform: Res<Platform>, settings: Res<PhysicsSettings>, mut stats: ResMut<PhysicsStats>) {
    if platform.motion.dropped.contains(&tick(&time)) {
        stats.steps = settings.max_steps_per_frame;
    }
}

/// Steps the plugin dropped, over the run (its stats are per frame).
#[derive(Resource, Default)]
struct Dropped(u32);

#[allow(clippy::needless_pass_by_value)]
fn count_dropped(stats: Res<PhysicsStats>, mut dropped: ResMut<Dropped>) {
    dropped.0 += stats.dropped_steps;
}

/// A second step this tick, the base moving on to the next tick's pose (physics a tick ahead
/// of the movers; the next tick's step then leaves it there).
#[allow(clippy::needless_pass_by_value)]
fn double_step(time: Res<Time<Fixed>>, platform: Res<Platform>, mut physics: ResMut<Physics>) {
    if !platform.motion.doubled.contains(&tick(&time)) {
        return;
    }
    let next = platform.motion.pose(now(&time) + time.delta_secs());
    let w = physics.world_mut(&platform.ws).expect("world");
    assert!(w.set_kinematic_target(platform.id, next));
    w.step();
}

/// The host's part for action bodies (the framework refreshes the movers).
#[allow(clippy::needless_pass_by_value)]
fn action_bases(physics: Res<Physics>, time: Res<Time>, mut bodies: Query<&mut ActionBody>) {
    let Some(w) = physics.worlds.active_world() else { return };
    for mut b in &mut bodies {
        if b.loco.grounded.is_some_and(|g| g.body.is_some()) {
            refresh_base_motion(w, &mut b.loco, time.delta_secs());
        }
    }
}

/// One tick: time, the platform's top centre and turn, the mover's and the action body's
/// feet, and whether each stood on the platform.
type Sample = (f32, WorldPos, Quat, [(WorldPos, bool); 2]);

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
    samples.0.push((now(&time), pose.pos.offset(pose.rotation * Vec3::Y * HALF.y), pose.rotation, [mover, body]));
}

/// Crushes over the run: (time, character index, depth, m).
#[derive(Resource, Default)]
struct Crushes(Vec<(f32, usize, f32)>);

/// The movers' [`CharacterCrushed`] messages, and the host's part for the action body
/// ([`crushed`] after its step), at the tick's end.
#[allow(clippy::needless_pass_by_value)]
fn record_crushes(time: Res<Time<Fixed>>, who: Res<Characters>, bodies: Query<&ActionBody>, mut messages: MessageReader<CharacterCrushed>, mut out: ResMut<Crushes>) {
    for c in messages.read() {
        let i = who.0.iter().position(|e| *e == c.entity).expect("a test character");
        assert!(c.depth.is_finite() && c.direction.is_finite(), "crush {c:?}");
        out.0.push((now(&time), i, c.depth));
    }
    if let Some(c) = crushed(who.0[1], &bodies.get(who.0[1]).expect("action body").loco) {
        out.0.push((now(&time), 1, c.depth));
    }
}

/// Per character, the crushes of a run: (time, depth, m).
fn crushes_of(crushes: &[(f32, usize, f32)], i: usize) -> Vec<(f32, f32)> {
    crushes.iter().filter(|c| c.1 == i).map(|c| (c.0, c.2)).collect()
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

/// Runs `seconds` with both characters idle on the platform moving by `motion`.
fn ride(motion: Motion, seconds: f32) -> Vec<Sample> {
    ride_under(motion, None, seconds)
}

/// [`ride`], under a fixed ceiling slab whose underside is `ceiling` m above the platform's
/// starting top, when given.
fn ride_under(motion: Motion, ceiling: Option<f32>, seconds: f32) -> Vec<Sample> {
    ride_crushed(motion, ceiling, seconds).0
}

/// [`ride_under`], with the crushes ([`Crushes`]).
fn ride_crushed(motion: Motion, ceiling: Option<f32>, seconds: f32) -> (Vec<Sample>, Vec<(f32, usize, f32)>) {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let ws = WorldspaceId::new("platform");
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default(), SfControllerPlugin::<Physics>::default(), SfActionPlugin::<Physics>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / HZ)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .insert_resource(rules())
        .init_resource::<Samples>()
        .init_resource::<Crushes>()
        // The host's order for its action bodies (the movers' is the framework's).
        .configure_sets(FixedUpdate, PhysicsSystems::Step.before(ActionSystems::Sample))
        .add_systems(FixedUpdate, (drive, drop_step).chain().before(PhysicsSystems::Hands))
        .add_systems(FixedUpdate, double_step.in_set(PhysicsSystems::Step).after(crate::step))
        .add_systems(FixedUpdate, action_bases.after(PhysicsSystems::Step).before(ActionSystems::Move))
        // At the tick's end, whatever the order inside it.
        .init_resource::<Dropped>()
        .add_systems(Last, count_dropped)
        .add_systems(FixedPostUpdate, (record, record_crushes));
    let desc = BodyDesc::new(BodyKind::Kinematic, WorldTransform::from_pos(centre()), ShapeRef::Box(HALF), Layer::STATIC);
    let id = app.world_mut().resource_mut::<Physics>().insert(&ws, &desc);
    let top = centre().offset(Vec3::Y * HALF.y);
    if let Some(c) = ceiling {
        let slab = WorldTransform::from_pos(top.offset(Vec3::Y * (c + CEILING.y)));
        app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(BodyKind::Fixed, slab, ShapeRef::Box(CEILING), Layer::STATIC));
    }
    app.insert_resource(Platform { ws, id, motion });
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
    let dropped = app.world().resource::<Dropped>().0;
    assert_eq!(dropped as usize, motion.dropped.len(), "dropped steps");
    let samples = std::mem::take(&mut app.world_mut().resource_mut::<Samples>().0);
    let &(t, top, _, _) = samples.last().expect("ticks ran");
    let want = motion.pose(t);
    let want = want.pos.offset(want.rotation * Vec3::Y * HALF.y);
    assert!(top.delta_f32(want).length() < 1e-4, "the platform is {} m off its path", top.delta_f32(want));
    (samples, std::mem::take(&mut app.world_mut().resource_mut::<Crushes>().0))
}

/// Per character: the largest horizontal and vertical departure from its platform-local spot
/// (as it stood just before the platform moved), and the lowest and highest feet height over
/// the top, from [`REST`] on, m.
fn drift(samples: &[Sample]) -> [(f32, f32, f32, f32); 2] {
    let settled = samples.iter().rposition(|s| s.0 < REST - 0.05).expect("rest samples");
    let (_, top0, turn0, ref0) = samples[settled];
    let mut out = [(0.0_f32, 0.0_f32, f32::MAX, f32::MIN); 2];
    for (t, top, turn, chars) in &samples[settled..] {
        for (i, (feet, on)) in chars.iter().enumerate() {
            let local = turn.inverse() * top.delta_f32(*feet);
            let local0 = turn0.inverse() * top0.delta_f32(ref0[i].0);
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
    let samples = ride(Motion::along(slide), REST + 3.0);
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
    let samples = ride(Motion::along(lift), REST + 2.0 * LEG + 0.5);
    let travel = LIFT / 60.0;
    let d = drift(&samples);
    report("lift", &d);
    for (i, (h, _, lo, hi)) in d.iter().enumerate() {
        assert!(*h < travel / 4.0, "character {i} slid {h} m on the lift");
        assert!(*lo > -travel / 2.0, "character {i} sank {lo} m into the lift (a tick of travel is {travel} m)");
        assert!(*hi < travel / 2.0, "character {i} left the lift by {hi} m");
    }
}

/// `IN11c2b`: sliding at 2.2 m/s and turning at 0.6 rad/s from rest for 3 s, with two ticks
/// whose physics step is dropped and two that step twice (one of each back to back): both
/// characters keep their spot on the deck to the idle drift (riding the base's velocity, each
/// such tick shifts them by a tick of travel, 3.7 cm, for good).
#[test]
fn idle_characters_keep_their_spot_through_dropped_and_doubled_steps() {
    let motion = Motion { path: slide, spin: turn, dropped: &[60, 101], doubled: &[80, 100] };
    let samples = ride(motion, REST + 3.0);
    let d = drift(&samples);
    report("slide + turn, dropped and doubled steps", &d);
    for (i, (h, v, lo, _)) in d.iter().enumerate() {
        assert!(*h <= 1e-4, "character {i} drifted {h} m on the deck");
        assert!(*v <= 1e-4 && *lo > -1e-4, "character {i} off the deck: {v} m, lowest {lo} m");
    }
}

// IN11c2c1: a lift rises into a ceiling.

/// The ceiling slab's half extents, m: over the whole platform.
const CEILING: Vec3 = Vec3::new(4.0, 0.25, 4.0);
/// The room over the riders' heads at the start, m, and how far the lift goes on rising once
/// their heads reach the ceiling (it then stops), m.
const HEADROOM: f32 = 1.0;
const SQUEEZE: f32 = 0.3;

/// Up at `speed` m/s after [`REST`] by [`HEADROOM`] + [`SQUEEZE`], then still.
fn to_ceiling(t: f32, speed: f32) -> Vec3 {
    Vec3::Y * (speed * (t - REST).max(0.0)).min(HEADROOM + SQUEEZE)
}

fn lift_2(t: f32) -> Vec3 {
    to_ceiling(t, 2.0)
}

/// 10 m/s: a tick (17 cm) covers more than the last gap under the ceiling.
fn lift_10(t: f32) -> Vec3 {
    to_ceiling(t, 10.0)
}

/// A lift at 2 and 10 m/s carries both idle characters into a ceiling: until their heads reach
/// it they ride the deck as on an open lift; the capsule's top never passes the ceiling (no
/// tunnelling at 10 m/s, no pop through by the ground snap) and from contact on they stay under
/// it, while the lift rises [`SQUEEZE`] m more into them (the crush, slice `IN11c2c2`: printed).
#[test]
fn a_lift_into_a_ceiling_stops_its_riders_under_it() {
    let params = MoverParams::default();
    for (speed, path) in [(2.0_f32, lift_2 as Path), (10.0, lift_10)] {
        let samples = ride_under(Motion::along(path), Some(params.height + HEADROOM), REST + (HEADROOM + SQUEEZE) / speed + 0.5);
        let top0 = samples[0].1;
        let ceiling = top0.offset(Vec3::Y * (params.height + HEADROOM));
        let travel = speed / 60.0;
        for i in 0..2 {
            let who = ["mover", "action body"][i];
            // The capsule's top under the ceiling, m (negative: through it).
            let gap = |s: &Sample| s.3[i].0.delta_f32(ceiling).y - params.height;
            let mut lowest = f32::MAX;
            for s in &samples {
                assert!(s.1.delta_f32(s.3[i].0).is_finite(), "{speed} m/s, t {}: {who} NaN", s.0);
                lowest = lowest.min(gap(s));
                assert!(gap(s) > -1e-4, "{speed} m/s, t {}: {who}'s head {} m through the ceiling", s.0, -gap(s));
            }
            let contact = samples.iter().position(|s| s.0 > REST && gap(s) <= params.skin + 1e-3).expect("the heads reach the ceiling");
            let settled = samples.iter().rposition(|s| s.0 < REST - 0.05).expect("rest samples");
            for s in &samples[settled..contact] {
                let over = s.1.delta_f32(s.3[i].0).y;
                assert!(s.3[i].1, "{speed} m/s, t {}: {who} not on the lift before contact", s.0);
                assert!(over.abs() < travel / 2.0, "{speed} m/s, t {}: {who}'s feet {over} m over the deck before contact", s.0);
            }
            let stop = samples[contact].3[i].0;
            let after = &samples[contact..];
            let rose = after.iter().map(|s| stop.delta_f32(s.3[i].0).y.abs()).fold(0.0_f32, f32::max);
            let on = after.iter().filter(|s| s.3[i].1).count();
            let last = after.last().expect("ticks after contact");
            println!(
                "{speed} m/s {who}: contact at t {:.3} s, head gap {:.2e} m at contact, lowest {lowest:.2e} m (skin {}); after: moved {rose:.2e} m, on the lift {on}/{} ticks, feet {:.3} m over the deck at the end",
                samples[contact].0,
                gap(&samples[contact]),
                params.skin,
                after.len(),
                last.1.delta_f32(last.3[i].0).y,
            );
            assert!(rose < 1e-4, "{speed} m/s: {who} moved {rose} m after its head reached the ceiling");
        }
    }
}

// IN11c2c2: crushes.

/// 2 m/s, on 0.6 m after the heads reach the ceiling: past `step_up` (0.45 m) into the feet, so
/// the ground cast starts inside the deck.
const DEEP: f32 = 0.6;

fn lift_deep(t: f32) -> Vec3 {
    Vec3::Y * (2.0 * (t - REST).max(0.0)).min(HEADROOM + DEEP)
}

/// 2 m/s, stopping 5 cm after the heads reach the ceiling: under [`MoverParams::crush_depth`].
fn lift_short(t: f32) -> Vec3 {
    Vec3::Y * (2.0 * (t - REST).max(0.0)).min(HEADROOM + 0.05)
}

/// Per character over a lift run under the ceiling: asserts the pose stays finite, the head
/// under the ceiling and the feet never lower in the deck than the ceiling holds them (on top of
/// it, within the skin, while there is room; held where the head met the ceiling once there is
/// none). Returns the deepest the deck reached over the feet, m, and the ticks off the lift after
/// contact.
fn squeezed(samples: &[Sample], ceiling: WorldPos, params: &MoverParams, label: &str) -> [(f32, usize); 2] {
    let mut out = [(0.0_f32, 0_usize); 2];
    for (i, o) in out.iter_mut().enumerate() {
        let gap = |s: &Sample| s.3[i].0.delta_f32(ceiling).y - params.height;
        let contact = samples.iter().position(|s| s.0 > REST && gap(s) <= params.skin + 1e-3).expect("the heads reach the ceiling");
        let held = samples[contact].3[i].0;
        for (k, s) in samples.iter().enumerate() {
            let (t, feet) = (s.0, s.3[i].0);
            assert!(s.1.delta_f32(feet).is_finite(), "{label} t {t}: character {i} NaN");
            assert!(gap(s) > -1e-4, "{label} t {t}: character {i}'s head {} m through the ceiling", -gap(s));
            // The deck's top over the feet (positive: in them).
            let into = feet.delta_f32(s.1).y;
            let floor = if k < contact { params.skin } else { params.skin.max(held.delta_f32(s.1).y) };
            assert!(into <= floor + 1e-4, "{label} t {t}: character {i}'s feet {into} m into the deck (the ceiling holds them {floor} m)");
            if k >= contact {
                o.0 = o.0.max(into);
                o.1 += usize::from(!s.3[i].1);
            }
        }
    }
    out
}

/// A lift at 2 and 10 m/s squeezes both idle riders [`SQUEEZE`] m into the ceiling, and at 2 m/s
/// [`DEEP`] m (past the ground cast's origin): each is crushed exactly once, on the tick the deck
/// passes [`MoverParams::crush_depth`] into its feet, and is held under the ceiling meanwhile
/// (never through it, never deeper in the deck than the ceiling forces; off the lift from then on).
#[test]
fn a_lift_squeezing_riders_into_a_ceiling_crushes_each_once() {
    let params = MoverParams::default();
    for (speed, path, squeeze) in [(2.0_f32, lift_2 as Path, SQUEEZE), (10.0, lift_10, SQUEEZE), (2.0, lift_deep, DEEP)] {
        let (samples, crushes) = ride_crushed(Motion::along(path), Some(params.height + HEADROOM), REST + (HEADROOM + squeeze) / speed + 0.5);
        let ceiling = samples[0].1.offset(Vec3::Y * (params.height + HEADROOM));
        let label = format!("{speed} m/s, {squeeze} m");
        let deepest = squeezed(&samples, ceiling, &params, &label);
        for (i, (deep, off)) in deepest.iter().enumerate() {
            let c = crushes_of(&crushes, i);
            let after = samples.iter().filter(|s| s.0 >= c.first().map_or(f32::MAX, |c| c.0)).count();
            println!("{label} character {i}: crushes {c:?} (crush depth {}), deck up to {deep:.3} m into the feet, off the lift {off} ticks, {after} ticks from the crush", params.crush_depth);
            assert_eq!(c.len(), 1, "{label}: character {i} crushed {} times", c.len());
            assert!(c[0].1 > params.crush_depth, "{label}: character {i} crushed at {} m", c[0].1);
            assert!(*deep > squeeze - 0.05, "{label}: character {i}: the deck only reached {deep} m into it");
        }
    }
}

/// A lift that stops 5 cm into its riders' heads, and one with no ceiling over it, crush no one:
/// the riders under the ceiling are held there, finite, out of it.
#[test]
fn a_lift_that_stops_short_or_rises_free_crushes_no_one() {
    let params = MoverParams::default();
    let (samples, crushes) = ride_crushed(Motion::along(lift_short), Some(params.height + HEADROOM), REST + HEADROOM / 2.0 + 0.5);
    let ceiling = samples[0].1.offset(Vec3::Y * (params.height + HEADROOM));
    for (i, (deep, off)) in squeezed(&samples, ceiling, &params, "short").iter().enumerate() {
        println!("short character {i}: deck up to {deep:.3} m into the feet, off the lift {off} ticks");
    }
    assert!(crushes.is_empty(), "a lift stopping short crushed {crushes:?}");
    let (_, crushes) = ride_crushed(Motion::along(lift), None, REST + 2.0 * LEG + 0.5);
    assert!(crushes.is_empty(), "an open lift crushed {crushes:?}");
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
    /// How far it goes, m (then still).
    travel: f32,
}

#[allow(clippy::needless_pass_by_value)]
fn drive_wall(time: Res<Time<Fixed>>, wall: Res<Wall>, mut physics: ResMut<Physics>) {
    if !wall.kinematic {
        return;
    }
    #[allow(clippy::cast_possible_truncation)]
    let t = time.elapsed_secs_f64() as f32;
    let at = wall.start.offset(Vec3::X * (wall.speed * (t - REST).max(0.0)).min(wall.travel));
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
    walled_into(hz, speed, kinematic, behind, (f32::INFINITY, None), seconds).0
}

/// [`walled`], the moving wall stopping after `travel.0` m, and a fixed wall whose −x face is
/// `travel.1` m ahead of the feet (+x), when given; with the crushes ([`Crushes`]).
fn walled_into(hz: f64, speed: f32, kinematic: bool, behind: f32, travel: (f32, Option<f32>), seconds: f32) -> (Vec<WallSample>, Vec<(f32, usize, f32)>) {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let ws = WorldspaceId::new("wall");
    #[allow(clippy::cast_possible_truncation)]
    let params = sf_physics::PhysicsParams { dt: (1.0 / hz) as f32, ..sf_physics::PhysicsParams::default() };
    let settings = PhysicsSettings { hz, ..PhysicsSettings::default() };
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin { params, settings }, SfControllerPlugin::<Physics>::default(), SfActionPlugin::<Physics>::default()))
        // The action plugin sets its own 60 Hz tick: one fixed step a frame at `hz` instead.
        .insert_resource(Time::<Fixed>::from_hz(hz))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / hz)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .insert_resource(rules())
        .init_resource::<WallSamples>()
        .init_resource::<Crushes>()
        .configure_sets(FixedUpdate, PhysicsSystems::Step.before(ActionSystems::Sample))
        .add_systems(FixedUpdate, drive_wall.before(PhysicsSystems::Hands))
        .add_systems(FixedPostUpdate, (record_wall, record_crushes));
    // Ground: a fixed slab 40 m along x, its top at the centre's height.
    let slab = WorldTransform::from_pos(centre().offset(Vec3::new(17.0, -0.25, 0.0)));
    app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(BodyKind::Fixed, slab, ShapeRef::Box(Vec3::new(20.0, 0.25, 6.0)), Layer::STATIC));
    // Feet at x = 0, the characters at z = ∓1.2; the wall's bottom 5 cm over the ground.
    let start = centre().offset(Vec3::new(-behind - WALL.x, WALL.y + 0.05, 0.0));
    let kind = if kinematic { BodyKind::Kinematic } else { BodyKind::Fixed };
    let id = app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(kind, WorldTransform::from_pos(start), ShapeRef::Box(WALL), Layer::STATIC));
    if let Some(ahead) = travel.1 {
        let at = centre().offset(Vec3::new(ahead + WALL.x, WALL.y + 0.05, 0.0));
        app.world_mut().resource_mut::<Physics>().insert(&ws, &BodyDesc::new(BodyKind::Fixed, WorldTransform::from_pos(at), ShapeRef::Box(WALL), Layer::STATIC));
    }
    app.insert_resource(Wall { ws, id, start, speed, kinematic, travel: travel.0 });
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
    let crushes = std::mem::take(&mut app.world_mut().resource_mut::<Crushes>().0);
    (std::mem::take(&mut app.world_mut().resource_mut::<WallSamples>().0), crushes)
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

/// The room between the capsules and the fixed wall ahead of them, and how far the cart goes on
/// pushing once they touch it, m.
const AHEAD: f32 = 0.2;
const SHOVE: f32 = 0.25;

/// A cart (the kinematic wall) at 2 m/s pushes both idle characters [`AHEAD`] m into a fixed
/// wall and on [`SHOVE`] m into them, then stops: each is crushed exactly once, past
/// [`MoverParams::crush_depth`], and stays finite, grounded and out of the fixed wall, at its skin.
#[test]
fn a_cart_pushing_characters_into_a_wall_crushes_each_once() {
    let params = MoverParams::default();
    let behind = 1.0;
    let ahead = params.radius + AHEAD;
    // To the capsules' backs, across the room ahead, then the shove.
    let travel = behind - params.radius + AHEAD + SHOVE;
    let (samples, crushes) = walled_into(60.0, 2.0, true, behind, (travel, Some(ahead)), REST + travel / 2.0 + 0.5);
    let wall = centre().offset(Vec3::X * ahead);
    for i in 0..2 {
        let mut closest = f32::MAX;
        for s in &samples {
            let (t, feet) = (s.0, s.2[i].0);
            assert!(s.1.delta_f32(feet).is_finite(), "t {t}: character {i} NaN");
            assert!(t < REST || s.2[i].1, "t {t}: character {i} left the ground");
            // The capsule's side short of the fixed wall's face (negative: in it).
            let room = feet.delta_f32(wall).x - params.radius;
            closest = closest.min(room);
            assert!(room > -1e-4, "t {t}: character {i} {} m into the fixed wall", -room);
        }
        let last = samples.last().expect("ticks ran");
        let into = last.1.delta_f32(last.2[i].0).x - params.radius;
        let c = crushes_of(&crushes, i);
        println!("cart character {i}: crushes {c:?} (crush depth {}), closest to the wall {closest:.2e} m (skin {}), cart {:.3} m into it at the end", params.crush_depth, params.skin, -into);
        assert_eq!(c.len(), 1, "character {i} crushed {} times", c.len());
        assert!(c[0].1 > params.crush_depth, "character {i} crushed at {} m", c[0].1);
    }
}
