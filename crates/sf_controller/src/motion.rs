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

use crate::intent::LocomotionIntent;
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

/// Half-lives of the motion springs, seconds.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpringParams {
    pub halflife_velocity: f32,
    pub halflife_facing: f32,
}

impl Default for SpringParams {
    /// Holden's defaults (0.27 s both); adapters tune per character.
    fn default() -> Self {
        Self { halflife_velocity: 0.27, halflife_facing: 0.27 }
    }
}

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

    /// Advances the springs by `dt`; returns the horizontal displacement.
    pub fn advance(&mut self, v_goal: Vec3, yaw_goal: f32, springs: &SpringParams, dt: f32) -> Vec3 {
        let mut x = Vec3::ZERO;
        spring_character(&mut x, &mut self.velocity, &mut self.acceleration, v_goal, springs.halflife_velocity, dt);
        spring_yaw(&mut self.yaw, &mut self.angular_velocity, yaw_goal, springs.halflife_facing, dt);
        x
    }

    /// The future of this state under constant goals at [`TRAJECTORY_TIMES`],
    /// relative to the current position.
    pub fn predict(&self, v_goal: Vec3, yaw_goal: f32, springs: &SpringParams) -> Trajectory {
        TRAJECTORY_TIMES.map(|t| {
            let mut s = *self;
            let offset = s.advance(v_goal, yaw_goal, springs, t);
            TrajectoryPoint { offset, yaw: s.yaw, velocity: s.velocity }
        })
    }

    /// The velocity in the character's frame (X side, Z forward).
    pub fn local_velocity(&self) -> Vec3 {
        Quat::from_rotation_y(-self.yaw) * self.velocity
    }
}
