//! A limp ragdoll in a physics world: spawned at an animated pose with the
//! pose's velocities (the death handoff), read back as a local pose.

use glam::{Affine3A, Quat, Vec3};
use sf_anim::{Skeleton, Trs};
use sf_coords::WorldTransform;
use sf_physics::{BodyDesc, BodyId, BodyKind, JointDesc, JointId, JointSolver, Layer, PhysicsWorld, ShapeRef};

use crate::RagdollDef;

/// How a ragdoll goes into the world.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RagdollSettings {
    /// Extra solver iterations per body (rapier solves a substep group with
    /// its bodies' largest count). The joint-stretch gate (≤ 1 cm) is
    /// measured with this (P7 tests).
    pub extra_iterations: usize,
    /// Linear and angular damping.
    pub damping: (f32, f32),
    pub layer: Layer,
    pub user: u64,
    pub solver: JointSolver,
    /// Continuous collision for the bodies (fast falls through thin floors).
    pub ccd: bool,
}

impl Default for RagdollSettings {
    fn default() -> Self {
        Self { extra_iterations: 8, damping: (0.05, 0.6), layer: Layer::RAGDOLL, user: 0, solver: JointSolver::Impulse, ccd: true }
    }
}

/// Where the character is when it ragdolls.
#[derive(Clone, Copy, Debug)]
pub struct SpawnPose<'a> {
    /// The character's model frame in the world (no scale).
    pub root: WorldTransform,
    /// Uniform scale of the model (race or creature scale).
    pub scale: f32,
    /// Model-space joint transforms now (the animated pose).
    pub model: &'a [Affine3A],
    /// The pose `dt` seconds earlier, for the bodies' initial velocities.
    pub previous: Option<(&'a [Affine3A], f32)>,
    /// The root's own velocity, m/s (world).
    pub root_velocity: Vec3,
}

/// A spawned ragdoll: its bodies (in [`RagdollDef::bodies`] order) and joints.
#[derive(Clone, Debug)]
pub struct RagdollInstance {
    pub bodies: Vec<BodyId>,
    pub joints: Vec<(JointDesc, JointId)>,
    pub scale: f32,
    /// Total simulated mass, kg (the mass a grab of any limb decides by).
    pub mass: f32,
    /// The world-space pelvis drive: a kinematic anchor body and its joint (P8).
    pub anchor: Option<(BodyId, JointId)>,
    /// Strength dips from hits, recovering (P8).
    pub dips: Vec<crate::powered::Dip>,
    /// Whether the bodies are on the powered layer.
    pub powered: bool,
}

pub(crate) fn rigid(m: &Affine3A) -> (Quat, Vec3) {
    let (_, r, t) = m.to_scale_rotation_translation();
    (r.normalize(), t)
}

/// A body's world pose for a model-space joint transform.
pub(crate) fn body_world(local: (Vec3, Quat), joint: &Affine3A, root: &WorldTransform, scale: f32) -> WorldTransform {
    let (r, t) = rigid(joint);
    let model_pos = t + r * local.0;
    WorldTransform {
        pos: root.pos.offset(root.rotation * (model_pos * scale)),
        rotation: (root.rotation * r * local.1).normalize(),
        scale: Vec3::ONE,
    }
}

impl RagdollInstance {
    /// Inserts the ragdoll's bodies and joints at `pose`.
    pub fn spawn(world: &mut PhysicsWorld, def: &RagdollDef, pose: &SpawnPose<'_>, settings: &RagdollSettings) -> Self {
        let s = pose.scale;
        let mass = def.total_mass * s * s * s;
        let mut bodies = Vec::with_capacity(def.bodies.len());
        for b in &def.bodies {
            let j = usize::from(b.joint);
            let world_pose = body_world(b.local, &pose.model[j], &pose.root, s);
            let mut desc = BodyDesc::new(
                BodyKind::Dynamic { asleep: false },
                world_pose,
                ShapeRef::Capsule { r: b.shape.radius * s, half: b.shape.half * s },
                settings.layer,
            );
            desc.mass = Some(b.mass * s * s * s);
            desc.damping = settings.damping;
            desc.extra_iterations = settings.extra_iterations;
            desc.ccd = settings.ccd;
            desc.user = settings.user;
            let id = world.insert(&desc);
            let (mut v, mut w) = (pose.root_velocity, Vec3::ZERO);
            if let Some((prev, dt)) = pose.previous
                && dt > 1e-4
            {
                let before = body_world(b.local, &prev[j], &pose.root, s);
                v += before.pos.delta_f32(world_pose.pos) / dt;
                let mut dq = world_pose.rotation * before.rotation.inverse();
                if dq.w < 0.0 {
                    dq = -dq;
                }
                let (axis, angle) = dq.to_axis_angle();
                w = axis * (angle / dt);
            }
            world.set_velocity(id, v, w);
            world.set_mode_mass(id, Some(mass));
            bodies.push(id);
        }
        let mut joints = Vec::with_capacity(def.joints.len());
        for j in &def.joints {
            let desc = JointDesc {
                parent: bodies[usize::from(j.parent)],
                child: bodies[usize::from(j.child)],
                frame_parent: (j.frame_parent.0 * s, j.frame_parent.1),
                frame_child: (j.frame_child.0 * s, j.frame_child.1),
                kind: j.kind.physics(),
                contacts: false,
            };
            if let Some(id) = world.insert_joint(&desc, settings.solver) {
                joints.push((desc, id));
            }
        }
        for &(a, b) in &def.no_collide {
            world.exclude_pair(bodies[usize::from(a)], bodies[usize::from(b)]);
        }
        Self { bodies, joints, scale: s, mass, anchor: None, dips: Vec::new(), powered: false }
    }

    /// Removes the bodies (and with them the joints).
    pub fn remove(&self, world: &mut PhysicsWorld) {
        if let Some((a, _)) = self.anchor {
            world.remove(a);
        }
        for &b in &self.bodies {
            world.remove(b);
        }
    }

    /// The largest joint separation, m.
    pub fn max_stretch(&self, world: &PhysicsWorld) -> f32 {
        self.joints.iter().filter_map(|(d, _)| world.joint_stretch(d)).fold(0.0, f32::max)
    }

    /// True when every body sleeps.
    pub fn asleep(&self, world: &PhysicsWorld) -> bool {
        self.bodies.iter().all(|&b| world.is_sleeping(b).unwrap_or(true))
    }

    /// The root body's interpolated pose.
    pub fn root_pose(&self, world: &PhysicsWorld, alpha: f32) -> Option<WorldTransform> {
        world.pose_at(*self.bodies.first()?, alpha)
    }

    /// Whether `body` is one of this ragdoll's.
    pub fn contains(&self, body: BodyId) -> bool {
        self.bodies.contains(&body)
    }

    /// The ragdoll as a local pose for the skeleton, relative to the
    /// character frame `root`: body joints take their rotations from the
    /// bodies (interpolated by `alpha`); joints without a body keep their
    /// `frozen` local transform (the pose at the handoff); joints above the
    /// root body follow it rigidly. Bones keep their frozen lengths; only
    /// the skeleton's top joint moves. False if a body is gone.
    #[allow(clippy::too_many_arguments, clippy::many_single_char_names)]
    pub fn local_pose(
        &self,
        world: &PhysicsWorld,
        def: &RagdollDef,
        skeleton: &Skeleton,
        root: &WorldTransform,
        alpha: f32,
        frozen: &[Trs],
        out: &mut Vec<Trs>,
    ) -> bool {
        let n = skeleton.len();
        if frozen.len() != n {
            return false;
        }
        let inv_root = root.rotation.inverse();
        let s = self.scale.max(1e-4);
        let mut model: Vec<Option<(Quat, Vec3)>> = vec![None; n];
        for (b, &id) in def.bodies.iter().zip(&self.bodies) {
            let Some(w) = world.pose_at(id, alpha) else { return false };
            let q = (inv_root * w.rotation * b.local.1.inverse()).normalize();
            let p = inv_root * root.pos.delta_f32(w.pos) / s - q * b.local.0;
            model[usize::from(b.joint)] = Some((q, p));
        }
        // Up from the root body: each parent from its child and the frozen local.
        let mut j = usize::from(def.bodies[0].joint);
        while let Some(p) = skeleton.parent(j) {
            let (q, pos) = model[j].expect("set");
            let f = frozen[j];
            let qp = (q * f.rotation.inverse()).normalize();
            model[p] = Some((qp, pos - qp * f.translation));
            j = p;
        }
        for j in 0..n {
            if model[j].is_none() {
                let (qp, pp) = skeleton.parent(j).and_then(|p| model[p]).unwrap_or((Quat::IDENTITY, Vec3::ZERO));
                let f = frozen[j];
                model[j] = Some(((qp * f.rotation).normalize(), pp + qp * f.translation));
            }
        }
        out.clear();
        out.extend((0..n).map(|j| {
            let (q, p) = model[j].expect("set");
            let f = frozen[j];
            match skeleton.parent(j).and_then(|p| model[p]) {
                Some((qp, _)) => Trs { translation: f.translation, rotation: (qp.inverse() * q).normalize(), scale: f.scale },
                None => Trs { translation: p, rotation: q, scale: f.scale },
            }
        }));
        true
    }
}
