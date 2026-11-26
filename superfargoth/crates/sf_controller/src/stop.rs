//! Stop and pivot points predicted from the motion springs, for distance
//! matching (locomotion.md open question 4, animation.md open question 5:
//! a start/stop/pivot clip seeks by the distance left to the point).
//!
//! The velocity spring is critically damped ([`spring_character`]): with
//! `y = halflife_to_damping(h) / 2`, `j0 = v - v_goal` and `j1 = a + j0 y`,
//! `v(t) = e^{-yt} (j0 + j1 t) + v_goal`. With `v_goal = 0` the position
//! term tends to `x(∞) - x = j1 / y² + j0 / y = a / y² + 2 v / y`: the stop
//! point in closed form. The pivot (the velocity along the old direction of
//! travel reaching 0 under a reversing goal) is the root of
//! `e^{-yt} (c0 + c1 t) + g = 0` (the Lambert-W equation, `c0, c1, g` the
//! projections of `j0, j1, v_goal`); that function is unimodal (one turning
//! point at `1/y - c0/c1`), so the root is bracketed on a monotone piece and
//! bisected. Both answers match the mover's own stepping
//! ([`MotionState::advance`]) at any frame rate, including the brake
//! half-life and the rate-capped mode's 1D speed spring.

use glam::Vec3;
use sf_anim::spring::halflife_to_damping;

use crate::motion::{MotionState, STILL_SPEED, SpringParams, forward, spring_character};

/// Bisection steps of the root solvers: halves the bracket to below f32
/// resolution of any time under a minute.
const BISECT_ITERS: u32 = 48; // tunables: const (solver precision, not a feel or cost trade)

/// Doublings tried when bracketing a root on an unbounded monotone piece.
const BRACKET_DOUBLINGS: u32 = 64; // tunables: const (solver bound, not a feel or cost trade)

/// Samples of the scan for the brake-to-velocity half-life switch before a
/// pivot (that speed curve has no closed-form shape).
const SWITCH_SCAN_SAMPLES: u32 = 64; // tunables: const (solver resolution, not a feel or cost trade)

/// Where a reversal passes through zero speed along the old direction of
/// travel ([`pivot`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pivot {
    /// The pivot point relative to the current position, m.
    pub offset: Vec3,
    /// Time until the pivot, s.
    pub time: f32,
}

/// The remaining travel until the character comes to rest if its goal
/// velocity becomes zero now (the stop point relative to the current
/// position, m). Uses the half-life the mover picks for a stop
/// ([`SpringParams::halflife_for`], so the brake half-life when set). In the
/// rate-capped mode ([`SpringParams::turn`]) the speed springs along the
/// current heading and is clamped at zero, so a speed spring that would
/// undershoot stops at its zero crossing; the vertical (flying, swimming)
/// springs on its own. A snap half-life (0) stops at once: zero offset.
pub fn stop_offset(state: &MotionState, springs: &SpringParams) -> Vec3 {
    if springs.turn.is_none() {
        let h = springs.halflife_for(state.velocity.length(), 0.0);
        return travel(state.velocity, state.acceleration, h);
    }
    let speed = capped_speed(state, springs);
    let along = match speed.zero_time() {
        Some(t) => advance(Vec3::X * speed.s, Vec3::X * speed.rate, Vec3::ZERO, speed.halflife, t).0.x,
        None => travel(Vec3::X * speed.s, Vec3::X * speed.rate, speed.halflife).x,
    };
    let (vy, ay) = (Vec3::Y * state.velocity.y, Vec3::Y * state.acceleration.y);
    speed.dir * along + travel(vy, ay, springs.halflife_for(vy.y.abs(), 0.0))
}

/// Time until the speed stays below `rest_speed` (m/s) if the goal velocity
/// becomes zero now, s (the same springs as [`stop_offset`]; 0 when already
/// below or for a snap). In the rate-capped mode the horizontal speed and the
/// vertical each fall below `rest_speed` by then.
pub fn stop_time(state: &MotionState, springs: &SpringParams, rest_speed: f32) -> f32 {
    if springs.turn.is_none() {
        let h = springs.halflife_for(state.velocity.length(), 0.0);
        if h <= 0.0 {
            return 0.0;
        }
        let y = damping(h);
        return last_above(state.velocity, state.acceleration + state.velocity * y, y, rest_speed);
    }
    let speed = capped_speed(state, springs);
    let horizontal = if speed.halflife <= 0.0 || speed.s <= rest_speed {
        0.0
    } else {
        let y = damping(speed.halflife);
        let c1 = speed.rate + speed.s * y;
        match speed.zero_time() {
            // Monotone down to the zero crossing (the clamp): the first
            // time it reaches `rest_speed`.
            Some(t0) => bisect(|t| (-y * t).exp() * (speed.s + c1 * t) - rest_speed, 0.0, t0),
            None => last_above(Vec3::X * speed.s, Vec3::X * c1, y, rest_speed),
        }
    };
    let (vy, ay) = (state.velocity.y, state.acceleration.y);
    let hy = springs.halflife_for(vy.abs(), 0.0);
    let vertical = if hy <= 0.0 {
        0.0
    } else {
        let y = damping(hy);
        last_above(Vec3::Y * vy, Vec3::Y * (ay + vy * y), y, rest_speed)
    };
    horizontal.max(vertical)
}

/// The pivot of a turn toward `v_goal`: where the velocity component along
/// the current direction of travel first reaches zero, for Holden's 2D
/// velocity spring. `None` when the goal never reverses the travel (no root),
/// when the character is still (no direction of travel), and always in the
/// rate-capped mode ([`SpringParams::turn`]), where a reversal turns at speed
/// and never passes through zero speed. A brake half-life applies while the
/// speed is above the goal speed, then the velocity half-life, as in the
/// mover; for a goal that does not reverse the travel (an overshoot root
/// only) the switch is not followed past the brake phase. A snap reverses at
/// once (a pivot here and now when the goal points back).
pub fn pivot(state: &MotionState, v_goal: Vec3, springs: &SpringParams) -> Option<Pivot> {
    if springs.turn.is_some() {
        return None;
    }
    let (v, a) = (state.velocity, state.acceleration);
    let speed = v.length();
    if speed <= STILL_SPEED {
        return None;
    }
    let d = v / speed;
    let goal_speed = v_goal.length();
    let h = springs.halflife_for(speed, goal_speed);
    let t1 = zero_along(v, a, v_goal, h, d)?;
    let at = |t: f32| Pivot { offset: advance(v, a, v_goal, h, t).0, time: t };
    if (h - springs.halflife_velocity).abs() < 1e-6 {
        return Some(at(t1));
    }
    // Brake phase: until the speed falls to the goal speed.
    let y = damping(h);
    let (j0, g) = (v - v_goal, v_goal);
    let j1 = a + j0 * y;
    let above = |t: f32| ((-y * t).exp() * (j0 + j1 * t) + g).length_squared() - goal_speed * goal_speed;
    let Some(ts) = first_root(above, t1) else {
        return Some(at(t1));
    };
    let (x, v2, a2) = advance(v, a, v_goal, h, ts);
    let hv = springs.halflife_velocity;
    let t2 = zero_along(v2, a2, v_goal, hv, d)?;
    Some(Pivot { offset: x + advance(v2, a2, v_goal, hv, t2).0, time: ts + t2 })
}

/// The 1D speed spring of the rate-capped mode, as `advance_capped` sets it
/// up for a zero goal.
struct CappedSpeed {
    /// Direction of travel (the current heading).
    dir: Vec3,
    /// Speed, m/s.
    s: f32,
    /// Its rate, m/s².
    rate: f32,
    halflife: f32,
}

impl CappedSpeed {
    /// When the clamped speed reaches zero (the unclamped spring would
    /// undershoot), s; `None` when it only decays.
    fn zero_time(&self) -> Option<f32> {
        if self.halflife <= 0.0 {
            return None;
        }
        let c1 = self.rate + self.s * damping(self.halflife);
        (c1 < 0.0).then(|| self.s / -c1)
    }
}

fn capped_speed(state: &MotionState, springs: &SpringParams) -> CappedSpeed {
    let flat = Vec3::new(state.velocity.x, 0.0, state.velocity.z);
    let s = flat.length();
    // A zero goal has no heading: the heading stays along the travel (or
    // the facing when still), as in `advance_capped`.
    let heading = if s > STILL_SPEED { flat.x.atan2(flat.z) } else { state.yaw };
    let dir = forward(heading);
    CappedSpeed { dir, s, rate: state.acceleration.dot(dir), halflife: springs.halflife_for(s, 0.0) }
}

/// `y`, the spring's damping over two.
fn damping(halflife: f32) -> f32 {
    halflife_to_damping(halflife) / 2.0
}

/// The travel to rest of a velocity spring toward zero: `j1 / y² + j0 / y`.
fn travel(v: Vec3, a: Vec3, halflife: f32) -> Vec3 {
    if halflife <= 0.0 {
        return Vec3::ZERO;
    }
    let y = damping(halflife);
    (a + v * y) / (y * y) + v / y
}

/// The spring state after `t`: (displacement, velocity, acceleration).
fn advance(v: Vec3, a: Vec3, goal: Vec3, halflife: f32, t: f32) -> (Vec3, Vec3, Vec3) {
    let (mut x, mut v, mut a) = (Vec3::ZERO, v, a);
    spring_character(&mut x, &mut v, &mut a, goal, halflife, t);
    (x, v, a)
}

/// First time the velocity along `d` reaches zero, for the spring from
/// `(v, a)` toward `goal` (`d · v > 0`).
fn zero_along(v: Vec3, a: Vec3, goal: Vec3, halflife: f32, d: Vec3) -> Option<f32> {
    let g = d.dot(goal);
    if halflife <= 0.0 {
        return (g <= 0.0).then_some(0.0);
    }
    let y = damping(halflife);
    let c0 = d.dot(v) - g;
    let c1 = d.dot(a) + c0 * y;
    let h = |t: f32| (-y * t).exp() * (c0 + c1 * t) + g;
    // The one turning point; before it `h` is monotone, after it `h` tends
    // monotonically to `g`.
    let turn = if c1 == 0.0 { 0.0 } else { (1.0 / y - c0 / c1).max(0.0) };
    if turn > 0.0 && h(turn) <= 0.0 {
        return Some(bisect(h, 0.0, turn));
    }
    if g >= 0.0 {
        return None;
    }
    let mut span = 1.0 / y;
    for _ in 0..BRACKET_DOUBLINGS {
        if h(turn + span) <= 0.0 {
            return Some(bisect(h, turn, turn + span));
        }
        span *= 2.0;
    }
    None
}

/// The last time `|e^{-yt} (j0 + j1 t)| ≥ r` (0 if never). The squared
/// speed is `e^{-2yt} q(t)` with `q` quadratic; its slope has the sign of
/// `q' - 2y q`, a downward parabola, so the speed falls, may rise once, then
/// falls for good.
fn last_above(j0: Vec3, j1: Vec3, y: f32, r: f32) -> f32 {
    let (p0, p1, p2) = (j0.length_squared(), j0.dot(j1), j1.length_squared());
    let f = |t: f32| (-2.0 * y * t).exp() * (j0 + j1 * t).length_squared() - r * r;
    let (qa, qb, qc) = (-2.0 * y * p2, 2.0 * p2 - 4.0 * y * p1, 2.0 * p1 - 2.0 * y * p0);
    let disc = qb * qb - 4.0 * qa * qc;
    let (rise, peak) = if qa < 0.0 && disc > 0.0 {
        let s = disc.sqrt();
        let (r0, r1) = ((-qb + s) / (2.0 * qa), (-qb - s) / (2.0 * qa));
        (r0.min(r1).max(0.0), r0.max(r1).max(0.0))
    } else {
        (0.0, 0.0)
    };
    if f(peak) > 0.0 {
        let mut span = 1.0 / y;
        for _ in 0..BRACKET_DOUBLINGS {
            if f(peak + span) <= 0.0 {
                return bisect(f, peak, peak + span);
            }
            span *= 2.0;
        }
        return f32::INFINITY;
    }
    if f(0.0) > 0.0 { bisect(f, 0.0, rise) } else { 0.0 }
}

/// First root of `f` on `(0, end]` with `f(0) > 0` by a scan then bisection.
fn first_root(f: impl Fn(f32) -> f32, end: f32) -> Option<f32> {
    let mut lo = 0.0;
    for i in 1..=SWITCH_SCAN_SAMPLES {
        #[allow(clippy::cast_precision_loss)] // a small sample index
        let t = end * i as f32 / SWITCH_SCAN_SAMPLES as f32;
        if f(t) <= 0.0 {
            return Some(bisect(f, lo, t));
        }
        lo = t;
    }
    None
}

/// The crossing of `f` from `f(lo) > 0` to `f(hi) ≤ 0`.
fn bisect(f: impl Fn(f32) -> f32, mut lo: f32, mut hi: f32) -> f32 {
    for _ in 0..BISECT_ITERS {
        let mid = f32::midpoint(lo, hi);
        if f(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

#[cfg(test)]
#[path = "stop_tests.rs"]
mod tests;
