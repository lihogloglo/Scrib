//! Inertialization (Bollo, *Inertialization: High-Performance Animation
//! Transitions in Gears of War*, GDC 2018), as a post-process on the
//! graph's output pose.
//!
//! At a switch, the offset between where the previous output was heading
//! and the new source decays to zero over the blend time along a quintic
//! with matched initial value and velocity and zero value, velocity and
//! acceleration at the end. Only the new source is evaluated afterwards.
//!
//! Two refinements over the talk's formulation (as UE implements it):
//! - **Target velocity.** The talk's initial offset velocity is the source's
//!   velocity alone, so the output's velocity right after the switch is
//!   off by the target's own velocity. Here the offset velocity is source
//!   minus target (Holden, *Spring-It-On*, 2021, "Inertialization"), with
//!   the target's velocity taken from one extra evaluation of the new
//!   source a frame ahead, on the switch frame only.
//! - **Vector offsets.** The talk decays each bone's offset along one fixed
//!   axis, which drops the velocity component across that axis. Here each
//!   channel's offset is a 3-vector (translation, rotation as a scaled
//!   axis, scale) with a quintic per component, so pose and velocity are
//!   both continuous. The talk's overshoot guards stay, measured along the
//!   offset direction: the blend shortens to `-5·x0/v0` when the offset
//!   closes fast, and the initial acceleration never points toward zero.
//!
//! The output on the switch frame is the previous output extrapolated by
//! its velocity, so there is no one-frame stall. An interrupting switch
//! simply starts again from the last two outputs, which already include
//! the old offset.

use glam::{Quat, Vec3};
use sf_anim::{LocalPose, Trs};

/// Offset of one channel of one joint.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Channel {
    x0: Vec3,
    v0: Vec3,
    a0: Vec3,
    t1: f32,
}

impl Channel {
    fn new(x0: Vec3, v0: Vec3, duration: f32) -> Self {
        let len = x0.length();
        let mut t1 = duration.max(1e-4);
        if len > 1e-9 {
            let dir = x0 / len;
            let closing = v0.dot(dir);
            if closing < 0.0 {
                t1 = t1.min(-5.0 * len / closing).max(1e-4);
            }
            let mut a0 = (-8.0 * t1 * v0 - 20.0 * x0) / (t1 * t1);
            let along = a0.dot(dir);
            if along < 0.0 {
                a0 -= dir * along;
            }
            return Self { x0, v0, a0, t1 };
        }
        let a0 = (-8.0 * t1 * v0 - 20.0 * x0) / (t1 * t1);
        Self { x0, v0, a0, t1 }
    }

    /// The offset `t` seconds after the switch.
    fn at(&self, t: f32) -> Vec3 {
        if t >= self.t1 {
            return Vec3::ZERO;
        }
        let (x0, v0, a0, t1) = (self.x0, self.v0, self.a0, self.t1);
        let t1_2 = t1 * t1;
        let a = -(a0 * t1_2 + 6.0 * t1 * v0 + 12.0 * x0) / (2.0 * t1_2 * t1_2 * t1);
        let b = (3.0 * t1_2 * a0 + 16.0 * t1 * v0 + 30.0 * x0) / (2.0 * t1_2 * t1_2);
        let c = -(3.0 * t1_2 * a0 + 12.0 * t1 * v0 + 20.0 * x0) / (2.0 * t1_2 * t1);
        (((((a * t + b) * t + c) * t + a0 * 0.5) * t + v0) * t) + x0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct JointOffset {
    translation: Channel,
    rotation: Channel,
    scale: Channel,
}

/// `log(a · b⁻¹)` as a scaled axis, short arc.
fn rot_diff(a: Quat, b: Quat) -> Vec3 {
    let mut d = a * b.conjugate();
    if d.w < 0.0 {
        d = -d;
    }
    d.normalize().to_scaled_axis()
}

/// Source state extrapolated `s` seconds by velocity `(v_t, ω, v_s)`.
fn extrapolate(p: Trs, vel: (Vec3, Vec3, Vec3), s: f32) -> Trs {
    Trs { translation: p.translation + vel.0 * s, rotation: (Quat::from_scaled_axis(vel.1 * s) * p.rotation).normalize(), scale: p.scale + vel.2 * s }
}

/// `p ⊖ t` per channel.
fn offset(p: Trs, t: Trs) -> (Vec3, Vec3, Vec3) {
    (p.translation - t.translation, rot_diff(p.rotation, t.rotation), p.scale - t.scale)
}

/// The decaying pose offset of one graph instance.
#[derive(Clone, Debug, Default)]
pub struct Inertializer {
    joints: Vec<JointOffset>,
    elapsed: f32,
    active: bool,
}

impl Inertializer {
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Starts (or restarts) a blend of `duration` seconds.
    ///
    /// `prev` and `prev2` are the last two outputs, `prev_dt` the time
    /// between them; `target` is the new source now and `ahead` the new
    /// source `dt` seconds later.
    #[allow(clippy::too_many_arguments)]
    pub fn start(&mut self, prev: &LocalPose, prev2: &LocalPose, prev_dt: f32, target: &LocalPose, ahead: &LocalPose, dt: f32, duration: f32) {
        let n = target.len();
        self.joints.resize(n, JointOffset::default());
        let inv_prev = if prev_dt > 1e-6 { 1.0 / prev_dt } else { 0.0 };
        let h = dt.max(1e-4);
        for j in 0..n {
            let (p1, p2) = (prev.get(j), prev2.get(j));
            let (dt_, dr, ds) = offset(p1, p2);
            let vel = (dt_ * inv_prev, dr * inv_prev, ds * inv_prev);
            // Where the previous motion is now, and a frame later.
            let s0 = extrapolate(p1, vel, dt);
            let s1 = extrapolate(s0, vel, h);
            let x0 = offset(s0, target.get(j));
            let x1 = offset(s1, ahead.get(j));
            let v = ((x1.0 - x0.0) / h, (x1.1 - x0.1) / h, (x1.2 - x0.2) / h);
            self.joints[j] = JointOffset {
                translation: Channel::new(x0.0, v.0, duration),
                rotation: Channel::new(x0.1, v.1, duration),
                scale: Channel::new(x0.2, v.2, duration),
            };
        }
        self.elapsed = 0.0;
        self.active = true;
    }

    /// Advances the blend clock (call once per update, before [`Self::apply`]).
    pub fn advance(&mut self, dt: f32) {
        if self.active {
            self.elapsed += dt;
            let longest = self.joints.iter().map(|j| j.translation.t1.max(j.rotation.t1).max(j.scale.t1)).fold(0.0, f32::max);
            if self.elapsed >= longest {
                self.active = false;
            }
        }
    }

    /// Adds the current offset to `pose`.
    pub fn apply(&self, pose: &mut LocalPose) {
        if !self.active {
            return;
        }
        let t = self.elapsed;
        for (j, o) in self.joints.iter().enumerate().take(pose.len()) {
            let mut p = pose.get(j);
            p.translation += o.translation.at(t);
            p.rotation = (Quat::from_scaled_axis(o.rotation.at(t)) * p.rotation).normalize();
            p.scale += o.scale.at(t);
            pose.set(j, p);
        }
    }

    pub fn cancel(&mut self) {
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quintic_meets_its_boundary_conditions() {
        let c = Channel::new(Vec3::new(0.3, -0.2, 0.1), Vec3::new(-0.5, 1.0, 0.0), 0.4);
        let h = 1e-3;
        assert!(c.at(0.0).distance(c.x0) < 1e-6);
        let v = (c.at(h) - c.at(0.0)) / h;
        assert!(v.distance(c.v0) < 1e-2, "{v} vs {}", c.v0);
        assert!(c.at(c.t1 - 1e-4).length() < 1e-4);
        assert_eq!(c.at(c.t1), Vec3::ZERO);
        // A fast-closing offset shortens the blend instead of overshooting.
        let fast = Channel::new(Vec3::X, Vec3::NEG_X * 50.0, 1.0);
        assert!((fast.t1 - 0.1).abs() < 1e-6);
        for k in 0..100u8 {
            assert!(fast.at(f32::from(k) * 0.001).x > -1e-3);
        }
    }
}
