//! The render-body correction spring (locomotion.md "Correction springs", R8 adopt 2; Holden,
//! "Code vs Data Driven Displacement", 2022). The gameplay capsule is the truth; the drawn root
//! may sit off it by an offset (a co-op reconciliation, a snap, a clip catching up) that decays
//! toward zero with a critically damped spring, **velocity-limited** (at most a share of the
//! character's own speed, so feet don't skate at plants, starts and stops) and **clamped** (a
//! maximum distance and angle). The Bevy shell adds it to the render transform; it never feeds
//! back into gameplay.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::spring::critical_spring;

use crate::motion::wrap_angle;

/// Dials of the correction spring: position and rotation separately.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CorrectionParams {
    /// Position halflife, s.
    pub halflife: f32,
    /// Rotation halflife, s (slower than position).
    pub yaw_halflife: f32,
    /// The offset shrinks at most this share of the character's planar speed.
    pub speed_ratio: f32,
    /// Floor of that limit, m/s, so a standing character still converges.
    pub min_speed: f32,
    /// Rotation rate limit, rad/s.
    pub max_yaw_rate: f32,
    /// Clamp, m: the drawn root is never further off the capsule.
    pub max_distance: f32,
    /// Clamp, radians.
    pub max_yaw: f32,
}

impl Default for CorrectionParams {
    fn default() -> Self {
        Self {
            halflife: 0.1,
            yaw_halflife: 0.2,
            speed_ratio: 0.5,
            min_speed: 0.3,
            max_yaw_rate: 3.0,
            max_distance: 0.5,
            max_yaw: std::f32::consts::FRAC_PI_4,
        }
    }
}

/// The drawn root minus the gameplay root, with the spring's velocities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RenderCorrection {
    /// World offset, m.
    pub offset: Vec3,
    pub velocity: Vec3,
    /// Yaw offset, radians.
    pub yaw: f32,
    pub yaw_velocity: f32,
}

impl RenderCorrection {
    /// The gameplay body jumped by `delta` and turned by `yaw_delta` (a reconciliation, a
    /// snap): the drawn body stays where it was and catches up through [`Self::update`].
    pub fn absorb(&mut self, delta: Vec3, yaw_delta: f32, p: &CorrectionParams) {
        self.offset -= delta;
        self.yaw = wrap_angle(self.yaw - yaw_delta);
        self.clamp(p);
    }

    /// Decays the offset over `dt` (frame time: this is presentation) for a character moving
    /// at `speed` m/s on the plane.
    pub fn update(&mut self, p: &CorrectionParams, speed: f32, dt: f32) {
        if dt <= 0.0 || self.is_settled() {
            return;
        }
        let before = self.offset;
        critical_spring(&mut self.offset, &mut self.velocity, Vec3::ZERO, p.halflife, dt);
        let max_step = (p.speed_ratio * speed).max(p.min_speed) * dt;
        let step = self.offset - before;
        if step.length() > max_step {
            self.offset = before + step.clamp_length_max(max_step);
            self.velocity = self.velocity.clamp_length_max(max_step / dt);
        }
        let yaw_before = self.yaw;
        critical_spring(&mut self.yaw, &mut self.yaw_velocity, 0.0, p.yaw_halflife, dt);
        let max_turn = p.max_yaw_rate * dt;
        let turn = self.yaw - yaw_before;
        if turn.abs() > max_turn {
            self.yaw = yaw_before + turn.clamp(-max_turn, max_turn);
            self.yaw_velocity = self.yaw_velocity.clamp(-p.max_yaw_rate, p.max_yaw_rate);
        }
        self.clamp(p);
        if self.offset.length_squared() < 1e-8 && self.yaw.abs() < 1e-5 {
            *self = Self::default();
        }
    }

    pub fn is_settled(&self) -> bool {
        *self == Self::default()
    }

    /// The yaw offset as a rotation (pre-multiplied onto the gameplay rotation).
    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }

    fn clamp(&mut self, p: &CorrectionParams) {
        self.offset = self.offset.clamp_length_max(p.max_distance);
        self.yaw = self.yaw.clamp(-p.max_yaw, p.max_yaw);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_snap_decays_monotonically_within_the_speed_limit_and_clamps() {
        let p = CorrectionParams::default();
        let mut c = RenderCorrection::default();
        // A 2 m reconciliation is clamped to 0.5 m.
        c.absorb(Vec3::new(2.0, 0.0, 0.0), 1.0, &p);
        assert!((c.offset.length() - p.max_distance).abs() < 1e-6 && (c.yaw.abs() - p.max_yaw).abs() < 1e-6);
        let (dt, speed) = (1.0 / 144.0, 4.0);
        let mut last = c.offset.length();
        for frame in 1..=1000 {
            c.update(&p, speed, dt);
            let len = c.offset.length();
            assert!(len <= last + 1e-6, "monotonic");
            assert!(last - len <= p.speed_ratio * speed * dt + 1e-5, "velocity-limited");
            last = len;
            // 0.5 m at 2 m/s takes 0.25 s, then the spring's tail: under 1 cm at 0.5 s.
            if frame == 72 {
                assert!(len < 0.01, "{len} m left at 0.5 s");
            }
            // The rotation is slower (0.2 s halflife): under 1° at 1.2 s.
            if frame == 173 {
                assert!(c.yaw.abs() < 1f32.to_radians(), "{} rad left at 1.2 s", c.yaw);
            }
        }
        assert!(c.is_settled(), "never settles: {c:?}");
    }

    #[test]
    fn a_standing_character_still_converges_slowly() {
        let p = CorrectionParams::default();
        let mut c = RenderCorrection::default();
        c.absorb(Vec3::Z * 0.3, 0.0, &p);
        let mut t = 0.0;
        while !c.is_settled() && t < 5.0 {
            c.update(&p, 0.0, 1.0 / 60.0);
            t += 1.0 / 60.0;
        }
        assert!(c.is_settled() && t > 0.3 / p.min_speed - 0.05, "{t}");
    }
}
