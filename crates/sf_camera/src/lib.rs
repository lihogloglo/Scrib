//! Bevy-free camera rig math (`docs/specs/camera.md`). Presentation only: it runs at frame
//! rate with the frame `dt`, reads the render body and the player's look, and never writes
//! gameplay state. Aim reaches gameplay as `sf_action::Command::view`, filled by the input
//! shell from its [`Look`], never from the final camera pose.
//!
//! - [`orbit`]: third-person orbit rig: pivot height, shoulder offset and side swap, pitch
//!   limits, distance per [`RigMode`], a critically damped pivot spring with no rotation lag,
//!   and the optional auto-recentre (comfort, off by default).
//! - [`collision`]: the arm's sphere cast through `sf_controller`'s [`GroundProbe`]: push in at
//!   once, ease out slower, never inside geometry, near-character fade 0..1.
//! - [`lockon`]: acquisition scoring, target switching, break hysteresis, line-of-sight
//!   timeout and re-acquire after traversal as pure functions over candidate lists (the
//!   target itself lives in `sf_action`), plus the locked framing.
//! - [`first_person`]: per-move look limits around the entry yaw, eased in and out, and
//!   `ignore_look`.
//! - [`comfort`]: the settings list and a rotation-only trauma shake.
//!
//! [`GroundProbe`]: sf_controller::GroundProbe

pub mod collision;
pub mod comfort;
pub mod first_person;
pub mod lockon;
pub mod orbit;

#[cfg(test)]
mod tests;

use std::f32::consts::PI;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
pub use sf_controller::wrap_angle;

pub use collision::{ArmCollision, ArmResult, CollisionParams};
pub use comfort::{ComfortSettings, ShakeParams, TraumaShake, ViewChoice};
pub use first_person::{FirstPersonLook, LookLimits};
pub use lockon::{BreakReason, Candidate, LockParams, LockTimers, acquire, locked_look, switch_target};
pub use orbit::{ModeDistances, OrbitParams, OrbitPose, OrbitRig, RecentreParams, RigMode, Shoulder, recentre_behind};

/// A look direction in radians, same convention as `sf_action::ViewDir`: yaw about +Y (0 faces
/// +Z), pitch up positive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Look {
    pub yaw: f32,
    pub pitch: f32,
}

impl Look {
    pub const fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }

    /// Unit view direction.
    pub fn forward(self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        Vec3::new(sy * cp, sp, cy * cp)
    }

    /// Unit right vector on the horizontal plane (screen right for this yaw).
    pub fn right(self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        Vec3::new(-cy, 0.0, sy)
    }

    /// Camera rotation that maps the camera's local -Z (Bevy's convention) onto [`Self::forward`]
    /// and keeps +Y up.
    pub fn rotation(self) -> Quat {
        Quat::from_rotation_y(self.yaw + PI) * Quat::from_rotation_x(self.pitch)
    }

    /// The look from a direction (any length > 0).
    pub fn from_dir(dir: Vec3) -> Self {
        let flat = Vec3::new(dir.x, 0.0, dir.z).length();
        Self { yaw: dir.x.atan2(dir.z), pitch: dir.y.atan2(flat) }
    }

    /// Adds look input (radians), wraps yaw to ±π and clamps pitch.
    #[must_use]
    pub fn turned(self, delta: Self, pitch_min: f32, pitch_max: f32) -> Self {
        Self { yaw: wrap_angle(self.yaw + delta.yaw), pitch: (self.pitch + delta.pitch).clamp(pitch_min, pitch_max) }
    }

    /// Frame-rate independent ease toward `goal` (exponential, half-life in seconds), yaw the short
    /// way round.
    #[must_use]
    pub fn eased(self, goal: Self, halflife: f32, dt: f32) -> Self {
        let t = ease_factor(halflife, dt);
        Self {
            yaw: wrap_angle(self.yaw + wrap_angle(goal.yaw - self.yaw) * t),
            pitch: self.pitch + (goal.pitch - self.pitch) * t,
        }
    }
}

/// Fraction of the remaining gap an exponential damper covers in `dt` (1 for a zero half-life).
pub fn ease_factor(halflife: f32, dt: f32) -> f32 {
    if halflife <= 0.0 { 1.0 } else { 1.0 - (-(std::f32::consts::LN_2 * dt) / halflife).exp() }
}

/// Hermite smoothstep of `x` clamped to 0..1.
pub fn smoothstep(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}
