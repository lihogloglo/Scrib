//! The predicted trajectory and the per-frame parameters the animation
//! graph reads (locomotion.md "Data model").

use std::f32::consts::FRAC_PI_3;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::motion::wrap_angle;
use crate::mover::Locomotion;

/// Seconds ahead of each trajectory point.
pub const TRAJECTORY_TIMES: [f32; 5] = [0.2, 0.4, 0.6, 0.8, 1.0];

/// One predicted future sample.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    /// From the character's current position, world axes (horizontal on
    /// the ground, 3D when flying or swimming).
    pub offset: Vec3,
    pub yaw: f32,
    /// Velocity there, m/s.
    pub velocity: Vec3,
}

/// Predicted samples at [`TRAJECTORY_TIMES`].
pub type Trajectory = [TrajectoryPoint; 5];

/// Graph inputs of one character for one update.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LocomotionParams {
    /// Horizontal speed, m/s.
    pub speed: f32,
    /// Velocity in the character frame (X side, Z forward): 2D blend spaces.
    pub local_velocity: Vec3,
    /// Angle of the velocity from the facing, radians, positive toward +X;
    /// 0 when still.
    pub direction: f32,
    /// Yaw rate, rad/s.
    pub turn_rate: f32,
    pub grounded: bool,
    /// Signed yaw change from now to the last trajectory point (turn in
    /// place, turn starts).
    pub future_heading_change: f32,
    /// Predicted speed at the last trajectory point (start and stop states).
    pub future_speed: f32,
    /// Vertical speed, m/s: the 3D velocity's Y when flying or swimming,
    /// the fall speed when airborne on foot.
    pub vertical_speed: f32,
    /// Flight-path angle, radians, positive climbing (0 below 0.1 m/s).
    pub pitch: f32,
    /// Coordinated-turn bank, radians, positive rolling left into a left
    /// turn: `atan(speed · turn rate / g)` with g = 9.81, clamped to ±60°
    /// (flyers bank by it; ground characters may lean by it).
    pub bank: f32,
}

impl LocomotionParams {
    pub fn of(loco: &Locomotion) -> Self {
        let m = &loco.motion;
        let local_velocity = Quat::from_rotation_y(-m.yaw) * m.velocity;
        let speed = Vec3::new(m.velocity.x, 0.0, m.velocity.z).length();
        let total = m.velocity.length();
        let last = loco.trajectory[TRAJECTORY_TIMES.len() - 1];
        Self {
            speed,
            local_velocity,
            direction: if speed > 1e-3 { local_velocity.x.atan2(local_velocity.z) } else { 0.0 },
            turn_rate: m.angular_velocity,
            grounded: loco.grounded.is_some(),
            future_heading_change: wrap_angle(last.yaw - m.yaw),
            future_speed: last.velocity.length(),
            vertical_speed: if m.velocity.y == 0.0 { loco.vertical_speed } else { m.velocity.y },
            pitch: if total > 0.1 { m.velocity.y.atan2(speed) } else { 0.0 },
            bank: (speed * m.angular_velocity / 9.81).atan().clamp(-FRAC_PI_3, FRAC_PI_3),
        }
    }
}
