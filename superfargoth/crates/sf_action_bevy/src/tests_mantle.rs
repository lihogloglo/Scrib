//! Headless mantle (action.md B2a): the shell's context probe senses a box ledge on a Jump press
//! and the Warp move places the body on top (waist and high clips); too high a wall refuses into a
//! plain jump.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Vec2, Vec3};
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{
    ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, Origin, RuleKey, RuleSetBuilder, SlotInfo, SlotTable,
    TRAVERSAL_RULES,
};
use sf_anim::{RootPath, RootPose};
use sf_controller::{
    CollisionBuilder, CollisionMesh, GroundHit, GroundProbe, Locomotion, MoverParams, Nearest, ProbeQuery, StaticShape, StaticWorld, SurfaceId,
    TerrainProbe,
};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::Plane;
use crate::*;

/// The plane plus one box ledge across the path.
#[derive(Resource)]
struct Course {
    terrain: TerrainProbe<Plane>,
    statics: StaticWorld,
}

impl GroundProbe for Course {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

/// A box ledge `height` m tall whose face is 1 m ahead of the start (+Z), 4 m deep.
fn course(height: f32) -> Course {
    course_box(Vec3::new(-3.0, 0.0, 2.0), Vec3::new(3.0, height, 6.0))
}

/// One box from `min` to `max` on the plane.
fn course_box(min: Vec3, max: Vec3) -> Course {
    let h = (max - min) / 2.0;
    let c = |x: f32, y: f32, z: f32| [x * h.x, y * h.y, z * h.z];
    let corners = [
        c(-1.0, -1.0, -1.0),
        c(1.0, -1.0, -1.0),
        c(1.0, 1.0, -1.0),
        c(-1.0, 1.0, -1.0),
        c(-1.0, -1.0, 1.0),
        c(1.0, -1.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(-1.0, 1.0, 1.0),
    ];
    let idx = [0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(3));
    let mesh: CollisionMesh = b.finish();
    let mut statics = StaticWorld::new(WorldPos::ORIGIN);
    statics.insert(&Arc::new(StaticShape::new(&mesh).expect("box shape")), WorldTransform::from_pos(WorldPos::ORIGIN.offset((min + max) * 0.5)));
    statics.rebuild();
    Course { terrain: TerrainProbe(Plane), statics }
}

/// The clip's root track stand-in: up 1.2 m and forward 2 m in 0.667 s (the retargeted UAL2
/// `ClimbUp_1m_RM` measures about that).
struct Climb;

impl RootPath for Climb {
    fn duration(&self) -> f32 {
        0.667
    }
    fn root_at(&self, t: f32) -> RootPose {
        RootPose::new(Vec3::new(0.0, 1.2, 2.0) * (t / 0.667).clamp(0.0, 1.0), 0.0)
    }
}

/// The high clip's stand-in: up 2.38 m and forward 2 m in 1.3 s (the retargeted UAL2
/// `ClimbUp_2m_RM` measures that).
struct HighClimb;

impl RootPath for HighClimb {
    fn duration(&self) -> f32 {
        1.3
    }
    fn root_at(&self, t: f32) -> RootPose {
        RootPose::new(Vec3::new(0.0, 2.38, 2.0) * (t / 1.3).clamp(0.0, 1.0), 0.0)
    }
}

fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with("traverse.mantle.waist", SlotInfo::once(Climb.duration()))
        .with_root("traverse.mantle.waist", Arc::new(Climb))
        .with("traverse.mantle.high", SlotInfo::once(HighClimb.duration()))
        .with_root("traverse.mantle.high", Arc::new(HighClimb));
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, TRAVERSAL_RULES);
    ActionRules { rules: b.build().expect("base + traversal rules"), slots: Box::new(slots) }
}

/// Sneak held from this tick on, if any.
#[derive(Resource)]
struct SneakFrom(Option<u32>);

/// Forward from the start; Jump pressed on tick 10.
fn forward_and_jump(tick: Res<ActionTick>, sneak: Res<SneakFrom>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        let mut cmd = Command::empty(tick.0);
        cmd.stick = if tick.0.0 < 60 { Vec2::Y } else { Vec2::ZERO };
        if tick.0.0 == 10 {
            cmd.buttons.pressed.insert(ActionClass::Jump);
            cmd.buttons.held.insert(ActionClass::Jump);
        }
        if sneak.0.is_some_and(|t| tick.0.0 >= t) {
            cmd.buttons.held.insert(ActionClass::Sneak);
        }
        c.command = cmd;
    }
}

/// The last feet position and the moves in order, over `ticks` ticks at a ledge `height` m tall.
fn run(height: f32, ticks: usize) -> (Vec3, Vec<RuleKey>) {
    run_with(height, ticks, None)
}

fn run_with(height: f32, ticks: usize, sneak_from: Option<u32>) -> (Vec3, Vec<RuleKey>) {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<Course>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(course(height))
        .insert_resource(rules())
        .insert_resource(SneakFrom(sneak_from))
        .add_systems(FixedUpdate, forward_and_jump.in_set(ActionSystems::Command));
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let start = WorldPos::ORIGIN.offset(Vec3::Z * 1.0);
    let body = ActionBody::new(Locomotion::new(start, 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let e = app.world_mut().spawn((character, body, Transform::default())).id();
    let mut rules_seen = Vec::new();
    for _ in 0..ticks {
        app.update();
        let c = app.world().get::<ActionCharacter>(e).unwrap();
        if rules_seen.last() != Some(&c.state.mov) {
            rules_seen.push(c.state.mov);
        }
        let p = WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(e).unwrap().loco.position);
        assert!(p.is_finite() && p.y > -0.05, "never through the floor: {p}");
    }
    let body = app.world().get::<ActionBody>(e).unwrap();
    (WorldPos::ORIGIN.delta_f32(body.loco.position), rules_seen)
}

#[test]
fn a_waist_high_ledge_is_mantled_and_the_body_walks_on_from_its_top() {
    let mantle = RuleKey::of("traverse.mantle.waist");
    let (feet, seen) = run(1.0, 150);
    println!("1 m ledge: feet {feet}, moves {seen:?}");
    assert!(seen.contains(&mantle), "the mantle fired: {seen:?}");
    assert!((feet.y - 1.0).abs() < 0.05 && feet.z > 2.0, "ends on top: {feet}");
    assert_eq!(seen.last(), Some(&RuleKey::of("loco")), "hands back to walking");
}

#[test]
fn a_high_ledge_is_mantled_with_the_high_clip() {
    let (waist, high) = (RuleKey::of("traverse.mantle.waist"), RuleKey::of("traverse.mantle.high"));
    let (feet, seen) = run(2.0, 200);
    println!("2 m ledge: feet {feet}, moves {seen:?}");
    assert!(seen.contains(&high) && !seen.contains(&waist), "the high mantle fired: {seen:?}");
    assert!((feet.y - 2.0).abs() < 0.05 && feet.z > 2.0, "ends on top: {feet}");
    assert_eq!(seen.last(), Some(&RuleKey::of("loco")), "hands back to walking");
}

#[test]
fn a_wall_above_the_bands_refuses_into_a_plain_jump() {
    let (feet, seen) = run(3.5, 150);
    println!("3.5 m wall: feet {feet}, moves {seen:?}");
    let mantles = [RuleKey::of("traverse.mantle.waist"), RuleKey::of("traverse.mantle.high")];
    assert!(!seen.iter().any(|m| mantles.contains(m)), "no mantle: {seen:?}");
    assert!(feet.y.abs() < 0.05 && feet.z < 2.0, "stays below, against the face: {feet}");
}

#[test]
fn sneaking_never_mantles() {
    let (feet, seen) = run_with(1.0, 150, Some(0));
    println!("sneak at a 1 m ledge: feet {feet}, moves {seen:?}");
    assert!(!seen.contains(&RuleKey::of("traverse.mantle.waist")), "no mantle while sneaking: {seen:?}");
    assert!(feet.y.abs() < 0.05 && feet.z < 2.0, "stays below: {feet}");
}

#[test]
fn sneak_pressed_during_a_mantle_does_not_leave_the_body_in_the_box() {
    let (feet, seen) = run_with(1.0, 150, Some(14));
    println!("sneak mid-mantle: feet {feet}, moves {seen:?}");
    let inside = feet.y < 0.95 && feet.z > 2.0 - 0.29;
    assert!(!inside, "the capsule is not left inside the ledge: {feet}");
}

/// Each fixed tick's warp progress (current) and drawn root position (current).
#[derive(Resource, Default)]
struct TickLog(Vec<(Option<f32>, Vec3)>);

fn log_ticks(mut log: ResMut<TickLog>, bodies: Query<&ActionBody>) {
    for body in &bodies {
        log.0.push((body.warp_progress().map(|p| p.1), WorldPos::ORIGIN.delta_f32(body.poses().1.position)));
    }
}

/// The root position the warp put the body at for chain progress `p`: piecewise linear over the
/// ticks' (progress, position) samples, from the anchor at progress 0 (the tick before the warp).
fn root_at_progress(log: &[(Option<f32>, Vec3)], p: f32) -> Option<Vec3> {
    let first = log.iter().position(|s| s.0.is_some())?;
    let mut samples = vec![(0.0, log.get(first.checked_sub(1)?)?.1)];
    samples.extend(log[first..].iter().map_while(|&(q, x)| q.map(|q| (q, x))));
    samples.windows(2).find(|w| w[0].0 <= p && p <= w[1].0 && w[1].0 > w[0].0).map(|w| {
        let t = (p - w[0].0) / (w[1].0 - w[0].0);
        w[0].1.lerp(w[1].1, t)
    })
}

#[test]
fn mantle_progress_interpolates_from_the_same_ticks_as_the_drawn_body() {
    // 60 Hz ticks under a slow frame clock: 2 ticks per frame at 30 Hz, 2 or 3 at 25 Hz.
    for (frame_hz, frame_count) in [(30.0, 60), (25.0, 50)] {
        let mut app = App::new();
        app.add_plugins((TimePlugin, SfActionPlugin::<Course>::default()))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / frame_hz)))
            .insert_resource(course(1.0))
            .insert_resource(rules())
            .insert_resource(SneakFrom(None))
            .init_resource::<TickLog>()
            .add_systems(FixedUpdate, forward_and_jump.in_set(ActionSystems::Command))
            .add_systems(FixedUpdate, log_ticks.after(ActionSystems::Move));
        let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
        let body = ActionBody::new(Locomotion::new(WorldPos::ORIGIN.offset(Vec3::Z), 0.0), MoverParams::default(), WorldPos::ORIGIN);
        let e = app.world_mut().spawn((character, body, Transform::default())).id();
        let (mut frames, mut worst) = (0, 0.0_f32);
        for _ in 0..frame_count {
            app.update();
            let body = app.world().get::<ActionBody>(e).unwrap();
            let Some((prev, curr)) = body.warp_progress() else { continue };
            let alpha = app.world().resource::<Time<Fixed>>().overstep_fraction().clamp(0.0, 1.0);
            let progress = prev + (curr - prev) * alpha;
            let (a, b) = body.poses();
            let drawn = WorldPos::ORIGIN.delta_f32(a.lerp(b, alpha).position);
            let log = &app.world().resource::<TickLog>().0;
            let expected = root_at_progress(log, progress).expect("progress within the warp's ticks");
            assert!(progress.is_finite() && drawn.is_finite());
            worst = worst.max(expected.distance(drawn));
            frames += 1;
        }
        println!("{frame_hz} Hz frames: {frames} mantle frames, worst clip/body gap {worst} m");
        assert!(frames > 2, "the mantle ran over several frames");
        assert!(worst < 1e-3, "the clip's progress stays on the drawn body: {worst} m");
    }
}

// Approach sweep: the press timing, angle and gait a mantle triggers from (owner 2026-10-27:
// "it's quite hard to get the high mantling to work").

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Gait {
    Walk,
    Run,
    Sprint,
}

/// When the press comes: 0.5 m of travel before the capsule touches the face, on the touching
/// tick, or after 0.3 s of pushing (sliding) along it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum When {
    Early,
    Contact,
    Slide,
}

#[derive(Clone, Copy, Debug)]
struct Approach {
    height: f32,
    /// The stick's angle off the face's inward normal (+Z), degrees.
    angle: f32,
    gait: Gait,
    when: When,
    class: ActionClass,
    sneak: bool,
}

/// What an approach did: the mantle that ran (and the facing it ended with, degrees off the
/// face's inward normal) and the press tick's refusals.
#[derive(Debug, Default)]
struct Outcome {
    mantle: Option<(RuleKey, f32)>,
    refusals: Vec<sf_action::Refusal>,
    pressed: bool,
    /// The press tick's raw context hit along the stick (kind, height, depth) and the obstacle the
    /// chooser senses.
    hit: Option<(Affordance, f32, f32)>,
    sensed: Option<(ObstacleClass, f32)>,
}

/// The box faces the start at this z (its front face).
const FACE_Z: f32 = 2.0;

#[derive(Resource)]
struct Drive {
    a: Approach,
    dir: Vec2,
    contact: Option<u32>,
    press: Option<u32>,
    out: Outcome,
}

fn drive(tick: Res<ActionTick>, course: Res<Course>, mut d: ResMut<Drive>, mut q: Query<(&mut ActionCharacter, &ActionBody)>) {
    let t = tick.0.0;
    for (mut c, body) in &mut q {
        let p = WorldPos::ORIGIN.delta_f32(body.loco.position);
        let gap = FACE_Z - c.params.radius - p.z;
        if d.contact.is_none() && gap <= 0.03 {
            d.contact = Some(t);
        }
        let due = match d.a.when {
            When::Early => gap <= 0.5 * d.a.angle.to_radians().cos(),
            When::Contact => d.contact.is_some(),
            When::Slide => d.contact.is_some_and(|c0| t >= c0 + 18),
        };
        if due && d.press.is_none() {
            d.press = Some(t);
            let params = ContextParams::from_mover(&body.params);
            let dir = Vec3::new(d.dir.x, 0.0, d.dir.y);
            let mut out = [None];
            let request = ContextRequest { feet: body.loco.position, dir, params: &params, query: ContextQuery::ObstacleAhead { range: OBSTACLE_RANGE_M } };
            ContextProbes::default().run(&[request], &*course, &mut out);
            d.out.hit = out[0].map(|h| (h.kind, h.height, h.depth));
            d.out.sensed = sense_obstacle(&mut ContextProbes::default(), &*course, body, d.dir).map(|o| (o.class, o.height));
        }
        let mut cmd = Command::empty(tick.0);
        cmd.stick = d.dir * if d.a.gait == Gait::Walk { 0.4 } else { 1.0 };
        if let Some(t0) = d.press.filter(|&t0| t < t0 + 6) {
            if t == t0 {
                cmd.buttons.pressed.insert(d.a.class);
            }
            cmd.buttons.held.insert(d.a.class);
        }
        if d.a.gait == Gait::Sprint {
            cmd.buttons.held.insert(ActionClass::Sprint);
        }
        if d.a.sneak {
            cmd.buttons.held.insert(ActionClass::Sneak);
        }
        c.command = cmd;
    }
}

fn watch(tick: Res<ActionTick>, mut d: ResMut<Drive>, q: Query<(&ActionCharacter, &ActionBody)>) {
    let mantles = [RuleKey::of("traverse.mantle.waist"), RuleKey::of("traverse.mantle.high")];
    for (c, body) in &q {
        if d.press == Some(tick.0.0) {
            d.out.pressed = true;
            d.out.refusals.clone_from(&c.output.refusals);
        }
        if mantles.contains(&c.state.mov) {
            d.out.mantle = Some((c.state.mov, sf_controller::wrap_angle(body.loco.motion.yaw).to_degrees()));
        }
    }
}

/// Runs one approach at a 16 m wide box `height` m tall: the stick held at `angle` toward the
/// face from 6 m back, aimed to touch it at x = 0.
fn approach(a: Approach) -> Outcome {
    let dir = Vec2::new(a.angle.to_radians().sin(), a.angle.to_radians().cos());
    let radius = CharacterParams::default().radius;
    let touch = Vec3::new(0.0, 0.0, FACE_Z - radius);
    let start = touch - Vec3::new(dir.x, 0.0, dir.y) * 6.0;
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<Course>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(course_box(Vec3::new(-8.0, 0.0, FACE_Z), Vec3::new(8.0, a.height, FACE_Z + 4.0)))
        .insert_resource(rules())
        .insert_resource(Drive { a, dir, contact: None, press: None, out: Outcome::default() })
        .add_systems(FixedUpdate, drive.in_set(ActionSystems::Command))
        .add_systems(FixedUpdate, watch.after(ActionSystems::Move));
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(WorldPos::ORIGIN.offset(start), dir.x.atan2(dir.y)), MoverParams::default(), WorldPos::ORIGIN);
    app.world_mut().spawn((character, body, Transform::default()));
    for _ in 0..900 {
        app.update();
        let d = app.world().resource::<Drive>();
        if d.press.is_some_and(|p| app.world().resource::<ActionTick>().0.0 > p + 120) {
            break;
        }
    }
    std::mem::take(&mut app.world_mut().resource_mut::<Drive>().out)
}

fn describe(a: &Approach, o: &Outcome) -> String {
    let what = match o.mantle {
        _ if !o.pressed => "NO PRESS (never reached the face)".to_string(),
        Some((k, yaw)) if k == RuleKey::of("traverse.mantle.waist") => format!("WAIST  yaw {yaw:+5.1}"),
        Some((_, yaw)) => format!("HIGH   yaw {yaw:+5.1}"),
        None => "refuse".to_string(),
    };
    let refusals = o.refusals.iter().map(|r| format!("{:?}:{:?}", r.rule, r.reason)).collect::<Vec<_>>().join(" ");
    let hit = o.hit.map_or("no hit".to_string(), |(k, h, dep)| format!("{k:?} h{h:.2} d{dep:.2}"));
    let refusals = format!("{hit} | sensed {:?} | {refusals}", o.sensed);
    format!("h{:.1} {:>2}deg {:<6} {:<7} {:<4} -> {what:<24} {refusals}", a.height, a.angle, format!("{:?}", a.gait), format!("{:?}", a.when), format!("{:?}", a.class))
}

#[test]
#[ignore = "measurement: mantle approach sweep (angle x gait x press timing), prints accept/refuse per case"]
fn mantle_approach_sweep() {
    let (mut ok, mut all) = (0, 0);
    for height in [1.2, 2.0] {
        for angle in [0.0, 20.0, 40.0, 60.0, 75.0] {
            for gait in [Gait::Walk, Gait::Run, Gait::Sprint] {
                let presses = [(When::Early, ActionClass::Jump), (When::Contact, ActionClass::Jump), (When::Slide, ActionClass::Jump), (When::Contact, ActionClass::Up)];
                for (when, class) in presses {
                    let a = Approach { height, angle, gait, when, class, sneak: false };
                    let o = approach(a);
                    all += 1;
                    ok += usize::from(o.mantle.is_some());
                    println!("{}", describe(&a, &o));
                }
            }
        }
    }
    println!("mantled {ok}/{all}");
}

#[test]
fn a_40_degree_run_with_jump_at_contact_mantles_both_heights_facing_the_face() {
    for (height, rule) in [(1.2, "traverse.mantle.waist"), (2.0, "traverse.mantle.high")] {
        let a = Approach { height, angle: 40.0, gait: Gait::Run, when: When::Contact, class: ActionClass::Jump, sneak: false };
        let o = approach(a);
        println!("{}", describe(&a, &o));
        let (k, yaw) = o.mantle.expect("mantled");
        assert_eq!(k, RuleKey::of(rule), "the band's mantle");
        assert!(yaw.abs() < 3.0, "ends facing the face's inward normal: {yaw} deg");
    }
}

#[test]
fn a_glancing_pass_along_a_face_never_mantles() {
    for height in [1.2, 2.0] {
        for when in [When::Contact, When::Slide] {
            let a = Approach { height, angle: 75.0, gait: Gait::Run, when, class: ActionClass::Jump, sneak: false };
            let o = approach(a);
            println!("{}", describe(&a, &o));
            assert!(o.pressed && o.mantle.is_none(), "no mantle at 75 deg");
        }
    }
}

#[test]
fn sneaking_at_an_angle_never_mantles() {
    for height in [1.2, 2.0] {
        let a = Approach { height, angle: 40.0, gait: Gait::Run, when: When::Contact, class: ActionClass::Jump, sneak: true };
        let o = approach(a);
        println!("{}", describe(&a, &o));
        assert!(o.pressed && o.mantle.is_none(), "no mantle while sneaking");
    }
}

#[test]
fn faces_above_the_bands_refuse_at_an_angle() {
    for height in [2.5, 3.5] {
        let a = Approach { height, angle: 40.0, gait: Gait::Run, when: When::Contact, class: ActionClass::Jump, sneak: false };
        let o = approach(a);
        println!("{}", describe(&a, &o));
        assert!(o.pressed && o.mantle.is_none(), "no mantle on a {height} m face");
    }
}
