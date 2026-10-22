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
    /// Inertia conditioning of the core (spine) bodies: principal moments at least
    /// `inertia_floor · mass · reach²`, `reach` = the body's farthest joint anchor. Lifts
    /// a small hub (pelvis) under heavy limbs, where joint limits otherwise give way and
    /// the torso spins (AN-18). 0 keeps the shapes' own inertia.
    pub inertia_floor: f32,
    /// The same reach floor for end bodies on a limb (hands, feet, tail tips): a light end
    /// whipping on its chain flips through its cone in a hard limp fall otherwise
    /// (AN-20, 300 N·s: hand 115° past at 0.25, held at 0.4). Mid-limb bodies get no reach
    /// floor: AN-18's floor of 1 on every body made limbs 3-12x harder to swing than
    /// their shapes (wooden; AN-20 test `hits_leave_limbs_loose`).
    pub end_inertia_floor: f32,
    /// The reach floor for end bodies jointed straight to a core body (a head on the
    /// chest): looser than the hands and feet (0.4), which whip on a longer chain and
    /// flip through their cones at 0.3 (AN-20 sweep: head 0.3 flops 0.94 of its shape at
    /// 120 N·s, hard death holds its limits).
    pub head_inertia_floor: f32,
    /// Long-axis conditioning: every principal moment is at least this share of the
    /// body's own largest (a thin capsule's long-axis moment is near zero), which lifts
    /// only the twist axis and leaves swing as the shape has it.
    pub axial_inertia: f32,
}

/// Default [`RagdollSettings::inertia_floor`] (AN-18 test `falls_keep_joints_in_limits_and_the_torso_from_spinning`).
pub const INERTIA_FLOOR: f32 = 1.0;

impl Default for RagdollSettings {
    fn default() -> Self {
        Self {
            // 16 (was 8): a limp chain of shape-inertia limbs needs them to hold its limits
            // under a 300 N·s hit (AN-20); only limp ragdolls pay it.
            extra_iterations: 16,
            damping: (0.05, 0.6),
            layer: Layer::RAGDOLL,
            user: 0,
            solver: JointSolver::Impulse,
            ccd: true,
            inertia_floor: INERTIA_FLOOR,
            end_inertia_floor: 0.4,
            head_inertia_floor: 0.3,
            axial_inertia: 0.5,
        }
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
    /// Extra solver iterations while limp ([`RagdollSettings::extra_iterations`]);
    /// powered bodies use [`crate::powered::DriveParams::powered_iterations`].
    pub iterations: usize,
    /// The extra iterations the bodies run with now.
    pub applied_iterations: usize,
    /// Each body's principal-moment floor as spawned (kg·m²), which the drive gains
    /// count in (a floored body is heavier to turn than its shape).
    pub inertia: Vec<f32>,
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

/// A capsule's largest principal moment (across its axis), kg·m².
pub(crate) fn capsule_inertia(mass: f32, radius: f32, half: f32) -> f32 {
    let length = 2.0 * (half + radius);
    mass * (3.0 * radius * radius + length * length) / 12.0
}

/// Each body's principal-moment floor (kg·m², at scale `s`): the reach floor
/// ([`RagdollSettings::inertia_floor`] on core bodies, `end_inertia_floor` on end bodies)
/// and the long-axis share of its own largest moment ([`RagdollSettings::axial_inertia`]).
fn conditioned_inertia(def: &RagdollDef, s: f32, reach: &[f32], settings: &RagdollSettings) -> Vec<f32> {
    let core = def.regions.iter().position(|r| r == crate::cook::CORE_REGION);
    def.bodies
        .iter()
        .enumerate()
        .zip(reach)
        .map(|((i, b), r)| {
            let m = b.mass * s * s * s;
            let leaf = !def.joints.iter().any(|j| usize::from(j.parent) == i);
            let k = if core == Some(usize::from(b.region)) {
                settings.inertia_floor
            } else if leaf && def.joints.iter().any(|j| usize::from(j.child) == i && core == Some(usize::from(def.bodies[usize::from(j.parent)].region))) {
                settings.head_inertia_floor
            } else if leaf {
                settings.end_inertia_floor
            } else {
                0.0
            };
            let own = capsule_inertia(m, b.shape.radius * s, b.shape.half * s);
            (k * m * r * r).max(settings.axial_inertia * own)
        })
        .collect()
}

impl RagdollInstance {
    /// Inserts the ragdoll's bodies and joints at `pose`.
    pub fn spawn(world: &mut PhysicsWorld, def: &RagdollDef, pose: &SpawnPose<'_>, settings: &RagdollSettings) -> Self {
        let s = pose.scale;
        let mass = def.total_mass * s * s * s;
        let mut reach = vec![0.0f32; def.bodies.len()];
        for j in &def.joints {
            reach[usize::from(j.parent)] = reach[usize::from(j.parent)].max(j.frame_parent.0.length() * s);
            reach[usize::from(j.child)] = reach[usize::from(j.child)].max(j.frame_child.0.length() * s);
        }
        let inertia = conditioned_inertia(def, s, &reach, settings);
        let mut bodies = Vec::with_capacity(def.bodies.len());
        for (b, &min_inertia) in def.bodies.iter().zip(&inertia) {
            let j = usize::from(b.joint);
            let world_pose = body_world(b.local, &pose.model[j], &pose.root, s);
            let mut desc = BodyDesc::new(
                BodyKind::Dynamic { asleep: false },
                world_pose,
                ShapeRef::Capsule { r: b.shape.radius * s, half: b.shape.half * s },
                settings.layer,
            );
            desc.mass = Some(b.mass * s * s * s);
            desc.min_inertia = min_inertia;
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
        Self { bodies, joints, scale: s, mass, anchor: None, dips: Vec::new(), powered: false, iterations: settings.extra_iterations, applied_iterations: settings.extra_iterations, inertia }
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
    /// root body follow it rigidly. Body joints take their positions from
    /// their bodies (so each bone sits exactly on its body); other bones
    /// keep their frozen lengths. Hosts show every joint at physics weight
    /// 1, including the joints above the root body: they carry the whole
    /// body's placement in `root`. False if a body is gone.
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
        let mut bodied = vec![false; n];
        for (b, &id) in def.bodies.iter().zip(&self.bodies) {
            let Some(w) = world.pose_at(id, alpha) else { return false };
            let q = (inv_root * w.rotation * b.local.1.inverse()).normalize();
            let p = inv_root * root.pos.delta_f32(w.pos) / s - q * b.local.0;
            model[usize::from(b.joint)] = Some((q, p));
            bodied[usize::from(b.joint)] = true;
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
                // A body joint takes its position from its body too (R0): the
                // bone lands exactly on the body, absorbing the joints'
                // sub-centimetre stretch instead of drifting down the chain.
                Some((qp, pp)) if bodied[j] => Trs { translation: qp.inverse() * (p - pp), rotation: (qp.inverse() * q).normalize(), scale: f.scale },
                Some((qp, _)) => Trs { translation: f.translation, rotation: (qp.inverse() * q).normalize(), scale: f.scale },
                None => Trs { translation: p, rotation: q, scale: f.scale },
            }
        }));
        true
    }
}
