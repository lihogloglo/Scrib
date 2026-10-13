//! Joints between bodies: the base `sf_ragdoll` builds on (task P7) and the
//! carry hand (P6). Two solvers, chosen per joint: impulse joints (maximal
//! coordinates, sleep and island simply) and reduced-coordinate multibodies
//! (Featherstone: no stretch, take high gains). physics.md open question 1;
//! the P1 spike measures both.
//!
//! Frames: each joint has a frame on either body (body-local); its X axis is
//! the twist axis of a ball joint and the axis of a hinge.

use glam::{Quat, Vec3};
use rapier3d::dynamics::{
    GenericJoint, GenericJointBuilder, ImpulseJointHandle, JointAxesMask, JointAxis, MotorModel, MultibodyJointHandle,
};
use rapier3d::math::Pose;

use crate::body::BodyId;
use crate::world::PhysicsWorld;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointSolver {
    Impulse,
    Multibody,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JointKind {
    /// Rotation limited by swing half-angles about the frame's Y and Z axes
    /// and a twist range about X, radians.
    Ball { swing: [f32; 2], twist: [f32; 2] },
    /// Rotation about the frame's X axis within `range`, radians.
    Hinge { range: [f32; 2] },
    Fixed,
    /// No locked axis, motors only: a world-space drive from a kinematic
    /// anchor body (UE's world-space physical animation).
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointDesc {
    pub parent: BodyId,
    pub child: BodyId,
    /// Joint frame in the parent body's space.
    pub frame_parent: (Vec3, Quat),
    /// Joint frame in the child body's space.
    pub frame_child: (Vec3, Quat),
    pub kind: JointKind,
    /// Contacts between the two jointed bodies (ragdoll neighbors: off).
    pub contacts: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Handle {
    Impulse(ImpulseJointHandle),
    Multibody(MultibodyJointHandle),
}

/// A joint in a [`PhysicsWorld`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JointId(Handle);

/// Implicit (stable) PD drive gains: acceleration-based, so independent of
/// the bodies' masses (physics.md "Ragdolls and the animation blend").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drive {
    pub stiffness: f32,
    pub damping: f32,
}

impl Drive {
    /// Critically damped (ζ = 1) for an angular frequency `omega` (rad/s).
    pub fn critical(omega: f32) -> Self {
        Self { stiffness: omega * omega, damping: 2.0 * omega }
    }
}

const ANGULAR: [JointAxis; 3] = [JointAxis::AngX, JointAxis::AngY, JointAxis::AngZ];

fn build(desc: &JointDesc) -> GenericJoint {
    let mask = match desc.kind {
        JointKind::Ball { .. } => JointAxesMask::LOCKED_SPHERICAL_AXES,
        JointKind::Hinge { .. } => JointAxesMask::LOCKED_REVOLUTE_AXES,
        JointKind::Fixed => JointAxesMask::LOCKED_FIXED_AXES,
        JointKind::Free => JointAxesMask::empty(),
    };
    let mut b = GenericJointBuilder::new(mask)
        .local_frame1(Pose::from_parts(desc.frame_parent.0, desc.frame_parent.1))
        .local_frame2(Pose::from_parts(desc.frame_child.0, desc.frame_child.1))
        .contacts_enabled(desc.contacts);
    match desc.kind {
        JointKind::Ball { swing, twist } => {
            b = b
                .limits(JointAxis::AngX, twist)
                .limits(JointAxis::AngY, [-swing[0], swing[0]])
                .limits(JointAxis::AngZ, [-swing[1], swing[1]]);
        }
        JointKind::Hinge { range } => b = b.limits(JointAxis::AngX, range),
        JointKind::Fixed | JointKind::Free => {}
    }
    b.build()
}

impl PhysicsWorld {
    /// Joins two bodies. `None` if either is gone, or (multibody) the joint
    /// would close a loop.
    pub fn insert_joint(&mut self, desc: &JointDesc, solver: JointSolver) -> Option<JointId> {
        let parent = self.live(desc.parent)?.handle;
        let child = self.live(desc.child)?.handle;
        let joint = build(desc);
        let handle = match solver {
            JointSolver::Impulse => Handle::Impulse(self.engine.impulse_joints.insert(parent, child, joint, false)),
            JointSolver::Multibody => Handle::Multibody(self.engine.multibody_joints.insert(parent, child, joint, false)?),
        };
        Some(JointId(handle))
    }

    pub fn remove_joint(&mut self, id: JointId) {
        match id.0 {
            Handle::Impulse(h) => {
                self.engine.remove_impulse_joint(h);
            }
            Handle::Multibody(h) => self.engine.remove_multibody_joint(h),
        }
    }

    fn joint_data_mut(&mut self, id: JointId) -> Option<&mut GenericJoint> {
        match id.0 {
            Handle::Impulse(h) => self.engine.impulse_joints.get_mut(h, true).map(|j| &mut j.data),
            Handle::Multibody(h) => {
                let (mb, link) = self.engine.multibody_joints.get_mut(h)?;
                mb.link_mut(link).map(|l| &mut l.joint.data)
            }
        }
    }

    /// Drives the joint's angular axes toward `target` (angles about the
    /// frame's X, Y, Z axes, radians), or releases it (limp) with `None`.
    pub fn set_joint_drive(&mut self, id: JointId, target: [f32; 3], drive: Option<Drive>) -> bool {
        let Some(j) = self.joint_data_mut(id) else {
            return false;
        };
        for (axis, angle) in ANGULAR.into_iter().zip(target) {
            if j.locked_axes.contains(axis.into()) {
                continue;
            }
            j.set_motor_model(axis, MotorModel::AccelerationBased);
            match drive {
                Some(d) => j.set_motor(axis, angle, 0.0, d.stiffness, d.damping),
                None => j.set_motor(axis, 0.0, 0.0, 0.0, 0.0),
            };
        }
        true
    }

    /// Drives the joint's free angular axes toward the relative rotation
    /// `target` (the child's frame in the parent's frame), or releases them
    /// with `None`. Rapier measures an angular motor's error on axis `i` as
    /// `2·asin(q.v_i)` of the relative rotation (`w ≥ 0`), so a rotation
    /// maps to the per-axis targets `2·asin(target.v_i)` and the drive
    /// converges on the whole rotation, swing and twist together (per-axis
    /// angle targets left the P1 spike's 0.5 m/s residual).
    /// `velocity` is the target's relative angular velocity in the parent
    /// frame's axes (feed-forward: without it the damping term lags a
    /// moving target by `2ζ·v/ω`).
    pub fn set_joint_target(&mut self, id: JointId, target: Quat, velocity: Vec3, drive: Option<Drive>) -> bool {
        let q = target.normalize();
        let q = if q.w < 0.0 { -q } else { q };
        let angles = [q.x, q.y, q.z].map(|v| 2.0 * v.clamp(-1.0, 1.0).asin());
        let Some(j) = self.joint_data_mut(id) else {
            return false;
        };
        for ((axis, angle), vel) in ANGULAR.into_iter().zip(angles).zip(velocity.to_array()) {
            if j.locked_axes.contains(axis.into()) {
                continue;
            }
            j.set_motor_model(axis, MotorModel::AccelerationBased);
            match drive {
                Some(d) => j.set_motor(axis, angle, vel, d.stiffness, d.damping),
                None => j.set_motor(axis, 0.0, 0.0, 0.0, 0.0),
            };
        }
        true
    }

    /// Drives the joint's free linear axes toward zero offset between its
    /// two frames (acceleration-based), or releases them with `None`.
    pub fn set_joint_linear_drive(&mut self, id: JointId, drive: Option<Drive>) -> bool {
        let Some(j) = self.joint_data_mut(id) else {
            return false;
        };
        for axis in [JointAxis::LinX, JointAxis::LinY, JointAxis::LinZ] {
            if j.locked_axes.contains(axis.into()) {
                continue;
            }
            j.set_motor_model(axis, MotorModel::AccelerationBased);
            match drive {
                Some(d) => j.set_motor(axis, 0.0, 0.0, d.stiffness, d.damping),
                None => j.set_motor(axis, 0.0, 0.0, 0.0, 0.0),
            };
        }
        true
    }

    /// Distance between the joint's two frame origins, meters (stretch).
    pub fn joint_stretch(&self, desc: &JointDesc) -> Option<f32> {
        let a = self.pose(desc.parent)?;
        let b = self.pose(desc.child)?;
        let pa = a.pos.offset(a.rotation * desc.frame_parent.0);
        let pb = b.pos.offset(b.rotation * desc.frame_child.0);
        #[allow(clippy::cast_possible_truncation)]
        Some(pa.delta(pb).length() as f32)
    }
}
