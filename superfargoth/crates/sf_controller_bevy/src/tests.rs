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
    AvoidObstacle, Avoider, ControllerSettings, ControllerStats, Dormant, Held, MoveIntent, Mover,
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

/// AN-2: a walker braking into a path's end hands the graph to idle once,
/// soon, and doesn't hold the walk loop at its rate floor through a creep
/// (the old exponential approach kept `moving` on the hysteresis edge at
/// 0.25 m/s for 1.5 s, the loop at half rate with shrunken strides).
#[test]
fn stopping_walkers_hand_over_to_idle_promptly() {
    let settings = ControllerSettings {
        moving_speed: 0.5,
        idle_speed: 0.25,
        stop_speed: 0.45,
        ..ControllerSettings::default()
    };
    let cruise = 1.5;
    let dt = 1.0 / 60.0;
    let node = sf_controller::PathNode::at(WorldPos::ORIGIN.offset(Vec3::Z * 12.0));
    let mut path = sf_controller::PathFollower::new(vec![node], sf_controller::PathMode::Once, cruise);
    let mut m = Mover::new(WorldPos::ORIGIN, 0.0, MoverParams::default(), WorldPos::ORIGIN);
    let springs = m.params.springs;
    let (mut braking, mut idle_at, mut switches, mut slow_walk) = (None, None, 0, 0.0);
    for frame in 0..600u16 {
        let t = f32::from(frame) * dt;
        let intent = path.update(m.loco.position, dt).intent;
        m.applied = intent;
        let (v_goal, yaw_goal) = m.loco.motion.goals(&intent);
        let d = m.loco.motion.advance(v_goal, yaw_goal, intent.gait, &springs, dt);
        m.loco.position = m.loco.position.offset(d);
        m.measured = m.loco.motion.velocity;
        let v = crate::FeedValues::of(&m, &settings, m.gait);
        switches += usize::from(v.moving != m.gait.moving);
        m.gait = v.gait();
        if t > 3.0 && braking.is_none() && v.speed < 0.98 * cruise {
            braking = Some(t);
        }
        if let Some(t0) = braking {
            if idle_at.is_none() && !v.moving {
                idle_at = Some(t - t0);
            }
            // The walk loop below half its cruise speed: SpeedMatch's rate floor.
            if v.moving && v.speed < 0.5 * cruise {
                slow_walk += dt;
            }
        }
    }
    let idle_at = idle_at.expect("never went idle");
    assert_eq!(switches, 2, "moving flickered");
    assert!(idle_at < 0.75, "idle {idle_at:.2} s after braking began");
    assert!(slow_walk < 0.3, "walk loop below half speed for {slow_walk:.2} s");
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

/// [`Held`]: an outside owner parks the mover (it brings [`Dormant`]): no
/// step, zero motion and no ground samples in the feed; removing both
/// resumes it.
#[test]
fn held_movers_are_dormant_and_unfed_until_released() {
    use sf_anim_bevy::{AnimLod, AnimTier, Animator};

    let mut app = app();
    app.add_plugins(sf_anim_bevy::SfAnimPlugin);
    let e = walker(&mut app, 0.0, Vec3::Z * 1.4);
    let anim = app
        .world_mut()
        .spawn((Animator::new(walk_asset()), AnimLod::forced(AnimTier::A0)))
        .id();
    app.world_mut().get_mut::<Mover>(e).unwrap().animator = Some(anim);
    for _ in 0..30 {
        app.update();
    }
    app.world_mut().entity_mut(e).insert(Held);
    app.update();
    assert!(app.world().get::<Dormant>(e).is_some(), "Held brings Dormant");
    let pos_of = |app: &App| app.world().get::<Mover>(e).unwrap().loco.position;
    let parked = pos_of(&app);
    for _ in 0..30 {
        app.update();
        assert_eq!(pos_of(&app), parked, "a held mover moved");
        let a = app.world().get::<Animator>(anim).unwrap();
        assert_eq!(a.drive.velocity, Vec3::ZERO, "a held mover fed motion");
        assert!(a.drive.ground.iter().all(Option::is_none), "a held mover fed ground samples");
    }
    app.world_mut().entity_mut(e).remove::<(Held, Dormant)>();
    for _ in 0..60 {
        app.update();
    }
    assert!(parked.delta_f32(pos_of(&app)).length() > 0.3, "released mover resumed walking");
}

/// The synthetic biped walk of `sf_anim`'s fixtures as a one-clip graph.
fn walk_asset() -> std::sync::Arc<sf_anim_bevy::AnimAsset> {
    use std::sync::Arc;

    use sf_anim::fixtures::{biped, gait};
    use sf_anim::{ClipRules, CookSettings, Looping, cook_clip};

    struct Walk(sf_anim::Clip);
    impl sf_anim_graph::CompileSource for Walk {
        fn clip(&mut self, reference: &str) -> Result<(sf_bake::ProductKey, sf_anim::Clip), String> {
            Ok((sf_bake::ProductKey::new("anim.cook", reference), self.0.clone()))
        }
        fn subgraph(&mut self, _: &str) -> Result<sf_anim_graph::asset::GraphAsset, String> {
            Err("none".into())
        }
    }
    let skeleton = biped();
    let src = gait(&skeleton, "walk", 1.0, 1.4, 0.5, 0.6, &[("thigh_l", "foot_l", 0.0), ("thigh_r", "foot_r", 0.5)]);
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let clip = cook_clip(&skeleton, &src, &rules, &CookSettings::default()).unwrap();
    let ron = r#"(nodes: {"walk": Clip(clip: "walk")}, layers: [(node: "walk")])"#;
    let graph = sf_anim_graph::compile_str("walk", ron, &skeleton, &mut Walk(clip.clone())).unwrap();
    let clips = graph.clips.iter().map(|_| Arc::new(clip.clone())).collect();
    Arc::new(sf_anim_bevy::AnimAsset::new(Arc::new(skeleton), Arc::new(graph), clips).unwrap())
}

/// What the graph got at each of its updates: the walker's position then and
/// `drive.moved`; `.1` holds this frame's until Evaluate shows whether it was consumed.
#[derive(Resource, Default)]
struct Fed(Vec<(WorldPos, Vec3)>, Option<(WorldPos, Vec3)>);

/// AN-1: an A2 animator updates every other frame, and each update must get
/// the motion of every frame since the last one (its foot locks undo exactly
/// that), not just the last frame's: else planted feet slide at half the
/// body's speed on every walker far enough away for A2.
#[test]
fn a2_updates_get_all_the_motion_since_their_last_update() {
    use bevy_app::PostUpdate;
    use sf_anim_bevy::{AnimLod, AnimSystems, AnimTier, Animator};

    let mut app = app();
    app.add_plugins(sf_anim_bevy::SfAnimPlugin).init_resource::<Fed>().add_systems(
        PostUpdate,
        (
            (|movers: Query<&Mover>, animators: Query<&Animator>, mut fed: ResMut<Fed>| {
                let m = movers.single().unwrap();
                let a = animators.get(m.animator.unwrap()).unwrap();
                fed.1 = Some((m.loco.position, a.drive.moved.translation));
            })
            .after(AnimSystems::Probe)
            .before(AnimSystems::Evaluate),
            // Evaluate resets what an update consumed.
            (|animators: Query<&Animator>, mut fed: ResMut<Fed>| {
                let consumed = animators.single().unwrap().drive.moved.translation == Vec3::ZERO;
                if let Some(f) = fed.1.take().filter(|f| consumed && f.1 != Vec3::ZERO) {
                    fed.0.push(f);
                }
            })
            .after(AnimSystems::Evaluate),
        ),
    );
    let e = walker(&mut app, 0.0, Vec3::Z * 1.4);
    let anim = app.world_mut().spawn((Animator::new(walk_asset()), AnimLod::forced(AnimTier::A2))).id();
    app.world_mut().get_mut::<Mover>(e).unwrap().animator = Some(anim);
    for _ in 0..30 {
        app.update();
    }
    app.world_mut().resource_mut::<Fed>().0.clear();
    for _ in 0..60 {
        app.update();
    }
    let fed = &app.world().resource::<Fed>().0;
    assert!(fed.len() >= 29, "A2: every other frame ({})", fed.len());
    let travelled = fed[0].0.delta_f32(fed[fed.len() - 1].0).length();
    let told: f32 = fed[1..].iter().map(|f| f.1.length()).sum();
    assert!(travelled > 1.0, "walking: {travelled} m");
    assert!((told - travelled).abs() < 0.01 * travelled, "the graph was told {told:.3} m of {travelled:.3} m");
}
