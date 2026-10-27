//! Spring-driven horizontal motion and facing (Holden, *Spring-It-On*,
//! 2021, and *Code vs Data Driven Displacement*, 2022). Every update is the
//! exact closed-form solution, so stepping at any frame rate lands on the
//! same curve, and the trajectory prediction is the same function
//! evaluated further ahead: prediction and simulation agree by
//! construction for a constant intent.

use std::f32::consts::{PI, TAU};

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::spring::{critical_spring, halflife_to_damping};

use crate::intent::{Gait, LocomotionIntent};
use crate::trajectory::{TRAJECTORY_TIMES, Trajectory, TrajectoryPoint};

/// Wraps an angle into `[-π, π)`.
pub fn wrap_angle(a: f32) -> f32 {
    (a + PI).rem_euclid(TAU) - PI
}

/// The unit forward vector of a yaw (yaw 0 faces +Z).
pub fn forward(yaw: f32) -> Vec3 {
    Vec3::new(yaw.sin(), 0.0, yaw.cos())
}

/// A critically damped velocity spring integrated into position: `x`
/// moves with `v`, `v` springs toward `v_goal` with acceleration `a`.
/// Exact for any `dt` (Holden 2022, "spring character").
pub fn spring_character(x: &mut Vec3, v: &mut Vec3, a: &mut Vec3, v_goal: Vec3, halflife: f32, dt: f32) {
    if halflife <= 0.0 {
        // Snap (no smoothing, e.g. a classic preset): the goal at once.
        *x += v_goal * dt;
        *v = v_goal;
        *a = Vec3::ZERO;
        return;
    }
    let y = halflife_to_damping(halflife) / 2.0;
    let j0 = *v - v_goal;
    let j1 = *a + j0 * y;
    let e = (-y * dt).exp();
    *x += e * (-j1 / (y * y) + (-j0 - j1 * dt) / y) + j1 / (y * y) + j0 / y + v_goal * dt;
    *v = e * (j0 + j1 * dt) + v_goal;
    *a = e * (*a - j1 * (y * dt));
}

/// A critically damped yaw spring toward `goal` the short way round
/// (`w` is yaw rate, rad/s). The result is wrapped to `[-π, π)`.
pub fn spring_yaw(yaw: &mut f32, w: &mut f32, goal: f32, halflife: f32, dt: f32) {
    let goal = *yaw + wrap_angle(goal - *yaw);
    critical_spring(yaw, w, goal, halflife, dt);
    *yaw = wrap_angle(*yaw);
}

/// Turns `from` toward `to` the short way round by at most `max` radians
/// (an infinite `max` snaps). The result is wrapped to `[-π, π)`.
pub fn turn_toward(from: f32, to: f32, max: f32) -> f32 {
    wrap_angle(from + wrap_angle(to - from).clamp(-max, max))
}

/// Tuning of the motion springs: half-lives in seconds, and the optional
/// rate-capped steering (locomotion.md "Player feel").
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SpringParams {
    /// Velocity half-life while speeding up (and for the whole velocity
    /// when there is no [`Self::halflife_brake`]). 0 snaps.
    pub halflife_velocity: f32,
    /// Facing half-life when there are no [`Self::turn`] rates.
    pub halflife_facing: f32,
    /// Velocity half-life while slowing down (the goal speed is below the
    /// current one); `None` uses [`Self::halflife_velocity`]. Characters
    /// that stop faster than they start (a player) set it shorter.
    pub halflife_brake: Option<f32>,
    /// Rate-capped steering: the facing and the direction of travel turn at
    /// these rates instead of springing, and the speed springs on its own,
    /// so a reversal turns at speed instead of braking through zero.
    /// `None` is Holden's 2D velocity spring and facing spring.
    pub turn: Option<TurnRates>,
}

impl Default for SpringParams {
    /// Holden's defaults (0.27 s both, no brake half-life, springs only);
    /// adapters tune per character.
    fn default() -> Self {
        Self { halflife_velocity: 0.27, halflife_facing: 0.27, halflife_brake: None, turn: None }
    }
}

impl SpringParams {
    /// The velocity half-life for a speed change from `speed` toward `goal_speed`.
    pub fn halflife_for(&self, speed: f32, goal_speed: f32) -> f32 {
        match self.halflife_brake {
            Some(brake) if goal_speed < speed => brake,
            _ => self.halflife_velocity,
        }
    }

    /// Whether one closed-form step over any `dt` equals many small ones
    /// (no brake switch, no rate cap): the trajectory prediction then needs
    /// no sub-steps.
    fn closed_form(&self) -> bool {
        self.turn.is_none() && self.halflife_brake.is_none_or(|b| (b - self.halflife_velocity).abs() < 1e-6)
    }
}

/// Top turn rates per gait, rad/s ([`SpringParams::turn`]). `f32::INFINITY`
/// snaps (a classic, mouse-direct facing).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnRates {
    pub walk: f32,
    pub run: f32,
    pub sprint: f32,
    pub sneak: f32,
}

impl TurnRates {
    /// The same rate for every gait, rad/s.
    pub const fn uniform(rate: f32) -> Self {
        Self { walk: rate, run: rate, sprint: rate, sneak: rate }
    }

    pub fn get(&self, gait: Gait) -> f32 {
        match gait {
            Gait::Walk => self.walk,
            Gait::Run => self.run,
            Gait::Sprint => self.sprint,
            Gait::Sneak => self.sneak,
        }
    }
}

/// Speed below which a velocity has no direction of its own, m/s.
const STILL_SPEED: f32 = 1e-3;

/// Sub-step of the trajectory prediction when the motion has no closed
/// form (brake half-life or turn rates), seconds.
const PREDICT_STEP: f32 = 1.0 / 60.0;

/// The spring state of a character's horizontal motion and facing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MotionState {
    /// Velocity, m/s. Y stays 0 on the ground; flying and swimming
    /// (`MoverMode`) spring it in 3D.
    pub velocity: Vec3,
    /// Acceleration, m/s², the velocity spring's state.
    pub acceleration: Vec3,
    pub yaw: f32,
    /// Yaw rate, rad/s.
    pub angular_velocity: f32,
    /// The facing goal held while the intent names none and is still.
    pub facing_goal: f32,
}

impl MotionState {
    /// A character standing still facing `yaw`.
    pub fn facing(yaw: f32) -> Self {
        Self { yaw, facing_goal: yaw, ..Self::default() }
    }

    /// The spring goals for `intent`: horizontal velocity and yaw (the
    /// mover adds the intent's Y for 3D modes). Updates
    /// the held facing goal.
    pub fn goals(&mut self, intent: &LocomotionIntent) -> (Vec3, f32) {
        let v = Vec3::new(intent.desired_velocity.x, 0.0, intent.desired_velocity.z);
        if let Some(yaw) = intent.desired_facing {
            self.facing_goal = yaw;
        } else if v.length_squared() > 1e-6 {
            self.facing_goal = v.x.atan2(v.z);
        }
        (v, self.facing_goal)
    }

    /// Advances the springs by `dt` at `gait` (it picks the turn rate);
    /// returns the displacement (horizontal on the ground).
    pub fn advance(&mut self, v_goal: Vec3, yaw_goal: f32, gait: Gait, springs: &SpringParams, dt: f32) -> Vec3 {
        if let Some(rates) = springs.turn {
            return self.advance_capped(v_goal, yaw_goal, rates.get(gait), springs, dt);
        }
        let mut x = Vec3::ZERO;
        let halflife = springs.halflife_for(self.velocity.length(), v_goal.length());
        spring_character(&mut x, &mut self.velocity, &mut self.acceleration, v_goal, halflife, dt);
        spring_yaw(&mut self.yaw, &mut self.angular_velocity, yaw_goal, springs.halflife_facing, dt);
        x
    }

    /// Rate-capped steering ([`SpringParams::turn`]): the facing turns
    /// toward its goal at `rate`; the horizontal velocity is a speed along a
    /// heading that turns toward the goal's direction at the same rate while
    /// the speed springs toward the goal's. Starting from rest, the heading
    /// starts along the facing when the facing goal is the direction of
    /// travel (no `desired_facing`), so the body leads the motion.
    fn advance_capped(&mut self, v_goal: Vec3, yaw_goal: f32, rate: f32, springs: &SpringParams, dt: f32) -> Vec3 {
        if dt <= 0.0 {
            return Vec3::ZERO;
        }
        let max = rate * dt;
        let yaw0 = self.yaw;
        self.yaw = turn_toward(yaw0, yaw_goal, max);
        self.angular_velocity = wrap_angle(self.yaw - yaw0) / dt;

        let flat = Vec3::new(self.velocity.x, 0.0, self.velocity.z);
        let goal = Vec3::new(v_goal.x, 0.0, v_goal.z);
        let (speed0, goal_speed) = (flat.length(), goal.length());
        let goal_heading = (goal_speed > STILL_SPEED).then(|| goal.x.atan2(goal.z));
        // The facing goal is the direction of travel (no `desired_facing`).
        let led = goal_heading.is_some_and(|g| wrap_angle(g - yaw_goal).abs() <= 1e-4);
        let heading0 = match (speed0 > STILL_SPEED, goal_heading) {
            // Travelling along the facing: take the facing's exact angle, so
            // an exact reversal turns both the same way round.
            (true, _) if led && wrap_angle(flat.x.atan2(flat.z) - yaw0).abs() < 1e-3 => yaw0,
            (true, _) => flat.x.atan2(flat.z),
            (false, Some(g)) if !led => g,
            (false, _) => yaw0,
        };
        let heading1 = goal_heading.map_or(heading0, |g| turn_toward(heading0, g, max));
        let turn = wrap_angle(heading1 - heading0);

        // The speed as a 1D velocity spring (state: the speed and its rate).
        let mut dist = Vec3::ZERO;
        let mut speed = Vec3::X * speed0;
        let mut speed_rate = Vec3::X * self.acceleration.dot(forward(heading0));
        let halflife = springs.halflife_for(speed0, goal_speed);
        spring_character(&mut dist, &mut speed, &mut speed_rate, Vec3::X * goal_speed, halflife, dt);
        if speed.x < 0.0 {
            speed = Vec3::ZERO;
            speed_rate = Vec3::ZERO;
        }
        // The chord of an arc at a constant turn rate: along the mid heading,
        // shortened by sinc(turn / 2).
        let half = 0.5 * turn;
        let chord = if half.abs() > 1e-4 { half.sin() / half } else { 1.0 };
        let mut d = forward(heading0 + half) * (dist.x.max(0.0) * chord);
        // Centripetal acceleration (speed times heading rate) beside the
        // tangential spring rate; only the tangential part is spring state.
        let side = Vec3::new(heading1.cos(), 0.0, -heading1.sin());
        let accel = forward(heading1) * speed_rate.x + side * (speed.x * turn / dt);

        // Vertical (flying, swimming): its own spring.
        let (mut y, mut vy, mut ay) = (Vec3::ZERO, Vec3::Y * self.velocity.y, Vec3::Y * self.acceleration.y);
        let halflife_y = springs.halflife_for(self.velocity.y.abs(), v_goal.y.abs());
        spring_character(&mut y, &mut vy, &mut ay, Vec3::Y * v_goal.y, halflife_y, dt);
        d.y = y.y;
        self.velocity = forward(heading1) * speed.x + vy;
        self.acceleration = accel + ay;
        d
    }

    /// The future of this state under constant goals at [`TRAJECTORY_TIMES`],
    /// relative to the current position.
    pub fn predict(&self, v_goal: Vec3, yaw_goal: f32, gait: Gait, springs: &SpringParams) -> Trajectory {
        if springs.closed_form() {
            return TRAJECTORY_TIMES.map(|t| {
                let mut s = *self;
                let offset = s.advance(v_goal, yaw_goal, gait, springs, t);
                TrajectoryPoint { offset, yaw: s.yaw, velocity: s.velocity }
            });
        }
        let (mut s, mut offset, mut t) = (*self, Vec3::ZERO, 0.0);
        TRAJECTORY_TIMES.map(|target| {
            while t < target - 1e-6 {
                let dt = PREDICT_STEP.min(target - t);
                offset += s.advance(v_goal, yaw_goal, gait, springs, dt);
                t += dt;
            }
            TrajectoryPoint { offset, yaw: s.yaw, velocity: s.velocity }
        })
    }

    /// The velocity in the character's frame (X side, Z forward).
    pub fn local_velocity(&self) -> Vec3 {
        Quat::from_rotation_y(-self.yaw) * self.velocity
    }
}
