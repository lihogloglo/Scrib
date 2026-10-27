//! Brake half-life and rate-capped steering ([`SpringParams::halflife_brake`],
//! [`SpringParams::turn`]; locomotion.md "Player feel"). Sanity only: the tuned
//! numbers are measured in the adapters (`mw_player_bevy` feel tests).

use super::*;

/// A snappy player-like tuning: fast velocity, faster brake, capped turns.
fn capped() -> SpringParams {
    SpringParams {
        halflife_velocity: 0.07,
        halflife_facing: 0.08,
        halflife_brake: Some(0.05),
        turn: Some(TurnRates { walk: 12.0, run: 18.0, sprint: 8.0, sneak: 9.0 }),
    }
}

fn running_north(speed: f32) -> MotionState {
    MotionState { velocity: Vec3::Z * speed, ..MotionState::facing(0.0) }
}

#[test]
fn defaults_keep_the_plain_springs() {
    let s = SpringParams::default();
    assert_eq!((s.halflife_brake, s.turn), (None, None));
    assert_eq!(s.halflife_for(4.0, 0.0), s.halflife_velocity);
}

#[test]
fn a_brake_half_life_stops_faster_than_it_starts() {
    let slow = SpringParams::default();
    let braking = SpringParams { halflife_brake: Some(0.1), ..slow };
    let stop = |springs: &SpringParams| {
        let mut m = running_north(4.0);
        let mut dist = 0.0;
        for _ in 0..120 {
            dist += m.advance(Vec3::ZERO, 0.0, Gait::Run, springs, DT).z;
            assert!(m.velocity.z >= -1e-4 && m.velocity.is_finite());
        }
        dist
    };
    let (coast, brake) = (stop(&slow), stop(&braking));
    assert!(brake < coast, "brake {brake} m, coast {coast} m");
    // Speeding up is unchanged.
    let mut a = MotionState::facing(0.0);
    let mut b = a;
    for _ in 0..30 {
        a.advance(Vec3::Z * 4.0, 0.0, Gait::Run, &slow, DT);
        b.advance(Vec3::Z * 4.0, 0.0, Gait::Run, &braking, DT);
    }
    assert!((a.velocity - b.velocity).length() < 1e-6);
}

#[test]
fn a_capped_reversal_turns_at_speed_with_the_body_leading() {
    let springs = capped();
    let mut m = running_north(4.5);
    let goal = Vec3::NEG_Z * 4.5;
    let yaw_goal = goal.x.atan2(goal.z);
    let mut frames = 0;
    while wrap_angle(m.yaw - yaw_goal).abs() > 1e-4 {
        m.advance(goal, yaw_goal, Gait::Run, &springs, DT);
        let speed = m.velocity.length();
        assert!(speed > 4.4 && speed < 4.6, "speed {speed} at frame {frames}");
        // The velocity follows the facing.
        let heading = m.velocity.x.atan2(m.velocity.z);
        assert!(wrap_angle(heading - m.yaw).abs() < 1e-3, "heading {heading} yaw {}", m.yaw);
        assert!(m.angular_velocity.abs() <= 18.0 + 1e-3);
        frames += 1;
        assert!(frames < 60, "never arrived");
    }
    // 180° at 18 rad/s: 10.5 frames.
    assert!((10..=12).contains(&frames), "{frames} frames");
}

#[test]
fn capped_steering_starts_along_the_facing_unless_it_strafes() {
    let springs = capped();
    // Facing the motion: the first step moves along the old facing, turned by one frame's cap.
    let mut m = MotionState::facing(0.0);
    let east = Vec3::X * 4.5;
    m.advance(east, FRAC_PI_2, Gait::Run, &springs, DT);
    let heading = m.velocity.x.atan2(m.velocity.z);
    assert!((heading - 18.0 * DT).abs() < 1e-3 && (m.yaw - 18.0 * DT).abs() < 1e-4, "heading {heading} yaw {}", m.yaw);
    // Strafing (a facing of its own): the motion goes where asked at once.
    let mut m = MotionState::facing(0.0);
    m.advance(east, 0.0, Gait::Run, &springs, DT);
    assert!((m.velocity.normalize() - Vec3::X).length() < 1e-4 && m.yaw == 0.0);
}

#[test]
fn snap_settings_reach_the_goal_at_once_without_nans() {
    let snap = SpringParams { halflife_velocity: 0.0, halflife_facing: 0.0, halflife_brake: Some(0.0), turn: Some(TurnRates::uniform(f32::INFINITY)) };
    let mut m = MotionState::facing(0.0);
    let goal = Vec3::new(-3.0, 0.0, -1.0);
    let d = m.advance(goal, goal.x.atan2(goal.z), Gait::Run, &snap, DT);
    assert!((m.velocity - goal).length() < 1e-4, "{}", m.velocity);
    assert!((m.yaw - goal.x.atan2(goal.z)).abs() < 1e-5 && d.is_finite());
    m.advance(Vec3::ZERO, m.yaw, Gait::Run, &snap, DT);
    assert!(m.velocity.length() < 1e-5);
    // A zero step changes nothing and stays finite.
    let before = m;
    assert_eq!(m.advance(goal, 1.0, Gait::Run, &snap, 0.0), Vec3::ZERO);
    assert_eq!(m, before);
    // Zero half-lives on the plain springs snap too.
    let plain = SpringParams { turn: None, ..snap };
    let mut m = MotionState::facing(0.0);
    m.advance(goal, 0.0, Gait::Run, &plain, DT);
    assert!((m.velocity - goal).length() < 1e-4 && m.yaw.is_finite());
}

#[test]
fn capped_trajectory_matches_the_simulation() {
    let world = World::new();
    let params = MoverParams { springs: capped(), ..MoverParams::default() };
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    let run = |v: Vec3| LocomotionIntent { desired_velocity: v, gait: Gait::Run, ..LocomotionIntent::IDLE };
    walk(&world, &mut loco, run(Vec3::Z * 4.5), &params, 30, |_| {});
    // Mid-reversal toward -X.
    let intent = run(Vec3::new(-4.5, 0.0, -0.5));
    let mut mover = KinematicMover::new();
    mover.step(MoveInput { loco: &mut loco, intent, params: &params, root: RootMotion::default() }, &world, DT);
    let (start, prediction) = (loco.position, loco.trajectory);
    let mut t = 0.0;
    let mut checked = 0;
    while checked < TRAJECTORY_TIMES.len() {
        mover.step(MoveInput { loco: &mut loco, intent, params: &params, root: RootMotion::default() }, &world, DT);
        t += DT;
        if (t - TRAJECTORY_TIMES[checked]).abs() < 1e-4 {
            let err = (start.delta_f32(loco.position) - prediction[checked].offset).length();
            assert!(err < 0.02, "+{t} s: error {err} m");
            assert!(wrap_angle(loco.motion.yaw - prediction[checked].yaw).abs() < 1e-2);
            checked += 1;
        }
    }
}
