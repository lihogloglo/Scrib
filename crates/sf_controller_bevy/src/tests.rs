//! Headless stage tests: avoidance + mover + write-back in a real `App`.
#![allow(clippy::many_single_char_names)]

use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::{Time, TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_controller::{GroundHit, GroundProbe, ProbeQuery, SurfaceId};
use sf_coords::WorldPos;

use crate::{
    AvoidObstacle, Avoider, ControllerSettings, ControllerStats, Dormant, MoveIntent, Mover,
    MoverParams, SfControllerPlugin,
};

/// Flat ground at y = 0 (rays and sweeps down only; nothing else to hit).
#[derive(Resource)]
struct Flat;

impl GroundProbe for Flat {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            let origin = WorldPos::ORIGIN.delta_f32(q.origin);
            let bottom = origin.y - q.half_height - q.radius;
            *o = (q.dir.y < -1e-6)
                .then(|| bottom / -q.dir.y)
                .filter(|t| (0.0..=q.max).contains(t))
                .map(|t| GroundHit {
                    point: q
                        .origin
                        .offset(q.dir * t - Vec3::Y * (q.half_height + q.radius)),
                    normal: Vec3::Y,
                    distance: t,
                    surface: SurfaceId::default(),
                });
        }
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
            1.0 / 60.0,
        )))
        .add_plugins(SfControllerPlugin::<Flat>::default())
        .insert_resource(Flat);
    app
}

fn walker(app: &mut App, x: f32, velocity: Vec3) -> Entity {
    let pos = WorldPos::ORIGIN.offset(Vec3::new(x, 0.0, 0.0));
    let mut intent = sf_controller::LocomotionIntent::IDLE;
    intent.desired_velocity = velocity;
    app.world_mut()
        .spawn((
            Mover::new(pos, 0.0, MoverParams::default(), WorldPos::ORIGIN),
            MoveIntent(intent),
            Avoider {
                max_speed: 2.0,
                ..Avoider::default()
            },
            Transform::default(),
        ))
        .id()
}

fn gap(app: &mut App, a: Entity, b: Entity) -> f32 {
    let w = app.world_mut();
    let pa = w.get::<Mover>(a).unwrap().loco.position;
    let pb = w.get::<Mover>(b).unwrap().loco.position;
    let d = pa.delta_f32(pb);
    Vec3::new(d.x, 0.0, d.z).length()
}

#[test]
fn head_on_walkers_pass_and_transforms_follow() {
    let mut app = app();
    let a = walker(&mut app, -4.0, Vec3::X * 1.2);
    let b = walker(&mut app, 4.0, -Vec3::X * 1.2);
    let mut closest = f32::MAX;
    for _ in 0..600 {
        app.update();
        closest = closest.min(gap(&mut app, a, b));
    }
    let r = MoverParams::default().radius;
    assert!(
        closest >= 2.0 * r - 0.02,
        "walkers overlapped: closest {closest} m vs {} m",
        2.0 * r
    );
    let w = app.world_mut();
    let (m, t) = (
        w.get::<Mover>(a).unwrap().clone(),
        *w.get::<Transform>(a).unwrap(),
    );
    assert!(m.loco.grounded.is_some());
    assert!(
        m.loco.position.delta_f32(WorldPos::ORIGIN).x < -3.0,
        "a walked past b"
    );
    assert!(
        (t.translation - m.root_transform().0).length() < 1e-5,
        "transform written"
    );
    let stats = w.resource::<ControllerStats>();
    assert_eq!((stats.movers, stats.reactive), (2, 2));
    let _ = w.resource::<Time>();
    // The measured velocity is the displacement over time (steady walk).
    let p0 = m.loco.position;
    app.update();
    let w = app.world_mut();
    let dt = w.resource::<Time>().delta_secs();
    let m = w.get::<Mover>(a).unwrap();
    let d = p0.delta_f32(m.loco.position) / dt;
    let travelled = Vec3::new(d.x, 0.0, d.z).length();
    assert!(
        travelled > 1.0 && (m.measured.length() - travelled).abs() < 0.05,
        "measured {} vs travelled {travelled}",
        m.measured
    );
}

#[test]
fn walkers_avoid_a_standing_obstacle_that_never_moves() {
    let mut app = app();
    let a = walker(&mut app, -4.0, Vec3::X * 1.2);
    let cam = app
        .world_mut()
        .spawn(AvoidObstacle {
            position: WorldPos::ORIGIN,
            velocity: Vec3::ZERO,
            radius: 0.4,
        })
        .id();
    let mut closest = f32::MAX;
    for _ in 0..600 {
        app.update();
        let p = app.world_mut().get::<Mover>(a).unwrap().loco.position;
        let d = WorldPos::ORIGIN.delta_f32(p);
        closest = closest.min(Vec3::new(d.x, 0.0, d.z).length());
    }
    assert!(
        closest >= 0.4 + MoverParams::default().radius - 0.02,
        "walked through the camera: {closest} m"
    );
    assert_eq!(
        app.world_mut().get::<AvoidObstacle>(cam).unwrap().position,
        WorldPos::ORIGIN
    );
}

#[test]
fn feed_values_are_in_the_character_frame() {
    // Facing +X (yaw 90°) and walking +X: forward in the graph's frame; then
    // strafing toward +Z, which is its right (forward × up).
    let mut m = Mover::new(
        WorldPos::ORIGIN,
        std::f32::consts::FRAC_PI_2,
        MoverParams::default(),
        WorldPos::ORIGIN,
    );
    let settings = ControllerSettings::default();
    let frame = m.loco.frame();
    let last = crate::FeedGait::default();
    m.measured = frame.dir_to_local(Vec3::X * 1.5);
    let v = crate::FeedValues::of(&m, &settings, last);
    assert!(v.moving && (v.speed - 1.5).abs() < 1e-5);
    assert!(
        (v.direction - bevy_math::Vec2::Y).length() < 1e-5,
        "{:?}",
        v.direction
    );
    m.measured = frame.dir_to_local(Vec3::Z);
    let v = crate::FeedValues::of(&m, &settings, last);
    assert!(
        (v.direction - bevy_math::Vec2::X).length() < 1e-5,
        "right: {:?}",
        v.direction
    );
    m.measured = Vec3::ZERO;
    assert!(!crate::FeedValues::of(&m, &settings, last).moving);
}

/// The graph gets the distance actually travelled: a walker pushing into a
/// wall (intent 1.5 m/s, no progress) is told it stands still, and the
/// moving / running flags switch with hysteresis on the measured speed.
#[test]
fn feed_uses_measured_speed_with_hysteresis() {
    let mut m = Mover::new(
        WorldPos::ORIGIN,
        0.0,
        MoverParams::default(),
        WorldPos::ORIGIN,
    );
    m.loco.motion.velocity = Vec3::Z * 1.5;
    let settings = ControllerSettings {
        moving_speed: 0.5,
        idle_speed: 0.25,
        run_speed: 2.9,
        run_exit: 2.6,
        ..ControllerSettings::default()
    };
    let feed = |m: &mut Mover, speed: f32| {
        m.measured = Vec3::Z * speed;
        let v = crate::FeedValues::of(m, &settings, m.gait);
        m.gait = v.gait();
        (v.moving, v.running)
    };
    assert_eq!(
        feed(&mut m, 0.0),
        (false, false),
        "intent alone moves nothing"
    );
    assert_eq!(feed(&mut m, 0.4), (false, false));
    assert_eq!(feed(&mut m, 0.6), (true, false));
    assert_eq!(feed(&mut m, 0.3), (true, false), "hysteresis keeps walking");
    assert_eq!(feed(&mut m, 2.8), (true, false));
    assert_eq!(feed(&mut m, 3.0), (true, true));
    assert_eq!(feed(&mut m, 2.7), (true, true), "hysteresis keeps running");
    assert_eq!(feed(&mut m, 2.5), (true, false));
    assert_eq!(feed(&mut m, 0.2), (false, false));
}

/// Processing range: a dormant mover never moves (whatever its intent) and
/// stands still; removing [`Dormant`] resumes it from where it stood.
#[test]
fn dormant_movers_never_move_and_resume_when_woken() {
    let mut app = app();
    let a = walker(&mut app, 0.0, Vec3::X * 1.2);
    for _ in 0..120 {
        app.update();
    }
    app.world_mut().entity_mut(a).insert(Dormant);
    app.update();
    let pos_of = |app: &App| app.world().get::<Mover>(a).unwrap().loco.position;
    let parked = pos_of(&app);
    assert!(
        parked.delta_f32(WorldPos::ORIGIN).length() > 0.3,
        "walked before sleeping"
    );
    for _ in 0..120 {
        app.update();
        assert_eq!(pos_of(&app), parked, "a dormant mover moved");
    }
    let m = app.world().get::<Mover>(a).unwrap();
    assert_eq!(m.loco.motion.velocity, Vec3::ZERO);
    assert_eq!(app.world().resource::<ControllerStats>().movers, 0);
    app.world_mut().entity_mut(a).remove::<Dormant>();
    for _ in 0..120 {
        app.update();
    }
    assert!(
        parked.delta_f32(pos_of(&app)).length() > 0.3,
        "woken mover resumed walking"
    );
}
