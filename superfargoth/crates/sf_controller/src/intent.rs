//! What a character wants to do this frame.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Movement style. Speeds per gait are adapter data ([`GaitSpeeds`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gait {
    #[default]
    Walk,
    Run,
    Sprint,
    Sneak,
}

/// Written by player input, AI, scripts or the path follower; read by the mover.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LocomotionIntent {
    /// World velocity, m/s. Only X and Z are used.
    pub desired_velocity: Vec3,
    /// Yaw to face (strafing, aiming). `None` faces the desired velocity,
    /// and keeps the last facing when standing still.
    pub desired_facing: Option<f32>,
    pub gait: Gait,
}

impl LocomotionIntent {
    pub const IDLE: Self = Self { desired_velocity: Vec3::ZERO, desired_facing: None, gait: Gait::Walk };
}

/// Direction-dependent top speeds of one gait, m/s: people move faster
/// forward than sideways or backward (Holden, *Code vs Data Driven
/// Displacement*, 2022). Adapters supply one per [`Gait`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GaitSpeeds {
    pub forward: f32,
    pub side: f32,
    pub back: f32,
}

impl GaitSpeeds {
    /// The desired world velocity for a move input (world XZ, length up to 1,
    /// e.g. a stick turned into camera space) on a character facing `yaw`.
    /// The speed is interpolated elliptically between the three axes.
    pub fn velocity(&self, input: Vec3, yaw: f32) -> Vec3 {
        let input = Vec3::new(input.x, 0.0, input.z).clamp_length_max(1.0);
        let local = Quat::from_rotation_y(-yaw) * input;
        let z = local.z * if local.z >= 0.0 { self.forward } else { self.back };
        Quat::from_rotation_y(yaw) * Vec3::new(local.x * self.side, 0.0, z)
    }
}
