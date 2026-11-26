use glam::Vec3;

use super::*;
use crate::intent::Gait;
use crate::motion::TurnRates;

const DT: f32 = 1.0 / 60.0;
/// Steps simulated (30 s at [`DT`]).
const STEPS: u32 = 1800;
/// The roadmap's verify: stop (and pivot) within 1 cm.
const TOL_M: f32 = 0.01;

fn state(v: Vec3, a: Vec3) -> MotionState {
    let yaw = v.x.atan2(v.z);
    MotionState { velocity: v, acceleration: a, ..MotionState::facing(yaw) }
}

fn springs(velocity: f32, brake: Option<f32>, turn: Option<f32>) -> SpringParams {
    SpringParams {
        halflife_velocity: velocity,
        halflife_brake: brake,
        turn: turn.map(TurnRates::uniform),
        ..SpringParams::default()
    }
}

/// The mover's own stepping toward a zero goal: (end offset, last time the
/// speed was at or above `rest`).
fn simulate_stop(mut s: MotionState, p: &SpringParams, rest: f32) -> (Vec3, f32) {
    let (mut x, mut t, mut last) = (Vec3::ZERO, 0.0, 0.0);
    let yaw = s.yaw;
    for _ in 0..STEPS {
        if s.velocity.length() >= rest {
            last = t;
        }
        x += s.advance(Vec3::ZERO, yaw, Gait::Run, p, DT);
        t += DT;
    }
    (x, last)
}

/// The mover's stepping toward `goal` until the velocity along the start
/// direction crosses zero, interpolated inside the step: (point, time).
fn simulate_pivot(mut s: MotionState, goal: Vec3, p: &SpringParams) -> Option<(Vec3, f32)> {
    let d = s.velocity.normalize();
    let yaw = goal.x.atan2(goal.z);
    let (mut x, mut t) = (Vec3::ZERO, 0.0);
    for _ in 0..STEPS {
        let u0 = d.dot(s.velocity);
        let step = s.advance(goal, yaw, Gait::Run, p, DT);
        let u1 = d.dot(s.velocity);
        if u1 <= 0.0 {
            let f = u0 / (u0 - u1);
            // The displacement is near-linear across the crossing step.
            return Some((x + step * f, t + DT * f));
        }
        x += step;
        t += DT;
    }
    None
}

fn check_stop(v: Vec3, a: Vec3, p: &SpringParams) {
    let s = state(v, a);
    let (end, _) = simulate_stop(s, p, 0.0);
    let predicted = stop_offset(&s, p);
    assert!(predicted.is_finite());
    assert!((predicted - end).length() < TOL_M, "v {v} a {a} {p:?}: predicted {predicted}, simulated {end}");
}

#[test]
fn stop_matches_spring_2d() {
    let p = SpringParams::default();
    check_stop(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, &p);
    check_stop(Vec3::new(3.0, 0.0, 4.0), Vec3::new(-2.0, 0.0, 1.0), &p);
    check_stop(Vec3::new(1.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 6.0), &springs(0.5, None, None));
    // A hard deceleration: the 2D spring overshoots back past the rest point.
    check_stop(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -20.0), &springs(0.5, None, None));
    // Flying: the vertical springs with the rest.
    check_stop(Vec3::new(2.0, 1.5, 0.0), Vec3::new(0.0, -1.0, 0.0), &p);
}

#[test]
fn stop_uses_brake_halflife() {
    let p = springs(0.3, Some(0.12), None);
    check_stop(Vec3::new(0.0, 0.0, 6.0), Vec3::new(0.0, 0.0, 3.0), &p);
    check_stop(Vec3::new(-4.0, 0.0, 2.0), Vec3::ZERO, &p);
    // The brake is shorter, so the stop is nearer than with the velocity half-life.
    let s = state(Vec3::new(0.0, 0.0, 6.0), Vec3::ZERO);
    assert!(stop_offset(&s, &p).z < stop_offset(&s, &springs(0.3, None, None)).z);
}

#[test]
fn stop_matches_rate_capped_speed_spring() {
    check_stop(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0), &springs(0.27, None, Some(6.0)));
    check_stop(Vec3::new(3.0, 0.0, -3.0), Vec3::new(1.0, 0.0, 0.0), &springs(0.3, Some(0.15), Some(8.0)));
    // A hard deceleration: the speed is clamped at zero instead of reversing.
    let p = springs(0.5, None, Some(6.0));
    check_stop(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -20.0), &p);
    let s = state(Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -20.0));
    assert!(stop_offset(&s, &p).z > 0.0);
}

#[test]
fn snap_stops_at_once() {
    let s = state(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 2.0));
    for p in [springs(0.0, None, None), springs(0.3, Some(0.0), None), springs(0.0, None, Some(6.0))] {
        assert_eq!(stop_offset(&s, &p), Vec3::ZERO, "{p:?}");
        assert!(stop_time(&s, &p, 0.05) <= 0.0, "{p:?}");
        check_stop(s.velocity, s.acceleration, &p);
    }
    let p = springs(0.0, None, None);
    let back = pivot(&s, Vec3::new(0.0, 0.0, -5.0), &p).unwrap();
    assert_eq!(back, Pivot { offset: Vec3::ZERO, time: 0.0 });
}

#[test]
fn stop_time_matches_simulated_rest() {
    let rest = 0.05;
    let cases = [
        (Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, SpringParams::default()),
        (Vec3::new(3.0, 0.0, 4.0), Vec3::new(-2.0, 0.0, 1.0), springs(0.3, Some(0.12), None)),
        (Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -20.0), springs(0.5, None, None)),
        (Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0), springs(0.27, None, Some(6.0))),
        (Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -20.0), springs(0.5, None, Some(6.0))),
    ];
    for (v, a, p) in cases {
        let s = state(v, a);
        let (_, last) = simulate_stop(s, &p, rest);
        let t = stop_time(&s, &p, rest);
        // The simulation samples once per step.
        assert!((t - last).abs() <= DT + 1e-4, "v {v} a {a} {p:?}: predicted {t}, simulated {last}");
    }
    let slow = state(Vec3::new(0.0, 0.0, 0.01), Vec3::ZERO);
    assert!(stop_time(&slow, &SpringParams::default(), rest) <= 0.0);
}

fn check_pivot(v: Vec3, a: Vec3, goal: Vec3, p: &SpringParams) {
    let s = state(v, a);
    let (point, time) = simulate_pivot(s, goal, p).expect("the simulation pivots");
    let predicted = pivot(&s, goal, p).expect("a pivot is predicted");
    assert!(
        (predicted.offset - point).length() < TOL_M,
        "v {v} a {a} goal {goal} {p:?}: predicted {predicted:?}, simulated {point} at {time}"
    );
    assert!((predicted.time - time).abs() < DT, "time: predicted {}, simulated {time}", predicted.time);
}

#[test]
fn pivot_matches_simulated_zero_crossing() {
    let p = SpringParams::default();
    check_pivot(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 0.0, -5.0), &p);
    check_pivot(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(4.0, 0.0, -3.0), &p);
    check_pivot(Vec3::new(2.0, 0.0, 3.0), Vec3::new(1.0, 0.0, 2.0), Vec3::new(-3.0, 0.0, -1.0), &springs(0.4, None, None));
}

#[test]
fn pivot_follows_brake_then_velocity_halflife() {
    // A reversal to a slower goal brakes, then springs on the velocity
    // half-life once the speed is below the goal speed.
    let p = springs(0.3, Some(0.12), None);
    check_pivot(Vec3::new(0.0, 0.0, 6.0), Vec3::ZERO, Vec3::new(0.0, 0.0, -2.0), &p);
    check_pivot(Vec3::new(0.0, 0.0, 6.0), Vec3::new(0.0, 0.0, 2.0), Vec3::new(1.5, 0.0, -1.5), &p);
    // An equal-speed reversal never brakes.
    check_pivot(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 0.0, -5.0), &p);
}

#[test]
fn no_pivot_without_reversal_still_or_capped() {
    let p = SpringParams::default();
    let s = state(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO);
    // A right-angle or forward goal never brings the old direction to zero.
    assert_eq!(pivot(&s, Vec3::new(5.0, 0.0, 0.0), &p), None);
    assert_eq!(pivot(&s, Vec3::new(1.0, 0.0, 3.0), &p), None);
    assert_eq!(pivot(&MotionState::facing(0.0), Vec3::new(0.0, 0.0, -5.0), &p), None);
    // Rate-capped: a reversal turns at speed, with no zero-speed point.
    assert_eq!(pivot(&s, Vec3::new(0.0, 0.0, -5.0), &springs(0.27, None, Some(6.0))), None);
}
