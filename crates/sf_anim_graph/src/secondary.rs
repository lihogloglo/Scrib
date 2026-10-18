//! Kinematic secondary motion in the procedural op stack (animation.md
//! "Next-gen roadmap" layer 4; physics.md P9):
//!
//! - `HitReact`: a hit queued with [`crate::GraphInstance::hit`] kicks
//!   critically damped rotation springs (Holden, *Spring-It-On*, 2021, the
//!   closed form in `sf_anim::spring`) on the hit joint and a few reactive
//!   ancestors, about the axis `lever × direction`. A spring kicked from
//!   rest peaks after `halflife / (2 ln 2)` and settles back to the animated
//!   pose, so the reaction is additive and leaves no offset. This is what
//!   characters below A0 (or past the powered-ragdoll budget) answer hits
//!   with, as UE's and Unity's kinematic "hit reaction" spring setups do.
//! - `SpringBones`: Verlet particle chains for tails, ears, antennae and
//!   hair (the Dynamic Bone / VRM `springBone` / UE `AnimDynamics` family):
//!   each chain joint's child is a particle pulled toward its animated
//!   position relative to the simulated parent, damped, under gravity, kept
//!   at bone length and within `max_angle` of the animated direction. The
//!   particles live in character space and feel `inertia` of the
//!   character's world motion. A fixed substep (1/120 s, accumulated) makes
//!   the result independent of the frame rate; the pull is a natural
//!   frequency and a damping ratio, so tuning is physical, not per-step
//!   factors. This is the base the cloth op (`ClothLattice`, P10) extends with lattice constraints.

use glam::{Quat, Vec3};
use sf_anim::ik::rotate_joint;
use sf_anim::spring::{critical_spring, halflife_to_damping};
use sf_anim::{LocalPose, ModelPose, RootDelta, Skeleton};
use serde::{Deserialize, Serialize};

use crate::compiled::{HitReactOp, SpringBonesOp};

const ROOT: glam::Affine3A = glam::Affine3A::IDENTITY;

/// The `SpringBones` substep, s.
pub const SPRING_SUBSTEP: f32 = 1.0 / 120.0;
/// Most substeps one update runs (a long hitch drops the rest of the time).
pub const SPRING_MAX_SUBSTEPS: u32 = 12;

/// A hit for the `HitReact` op, character space (the graph's model space).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    /// Where it landed.
    pub point: Vec3,
    /// Which way it pushes (normalized by the op).
    pub direction: Vec3,
    /// Dimensionless: 1 is a solid blow (the op's `gain` radians at peak on
    /// the hit joint). Hosts map their impulse onto it.
    pub strength: f32,
    /// The joint hit; `None`: the reactive joint nearest `point`.
    pub joint: Option<u16>,
}

/// Per reactive joint: rotation offset (scaled axis, model space) and its rate.
#[derive(Clone, Debug, Default)]
pub(crate) struct HitState {
    offset: Vec<Vec3>,
    velocity: Vec<Vec3>,
}

impl HitState {
    pub(crate) fn new(op: &HitReactOp) -> Self {
        Self { offset: vec![Vec3::ZERO; op.joints.len()], velocity: vec![Vec3::ZERO; op.joints.len()] }
    }

    /// The largest offset angle, radians (tests, diagnostics).
    pub(crate) fn max_angle(&self) -> f32 {
        self.offset.iter().map(|o| o.length()).fold(0.0, f32::max)
    }

    /// Back to the animated pose (LOD cut).
    pub(crate) fn clear(&mut self) {
        self.offset.fill(Vec3::ZERO);
        self.velocity.fill(Vec3::ZERO);
    }
}

/// Kicks for `hits`, advances the springs by `dt` and applies the offsets.
pub(crate) fn hit_react(op: &HitReactOp, st: &mut HitState, hits: &[Hit], s: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, dt: f32) {
    if op.joints.is_empty() {
        return;
    }
    // Peak of x(t) = v0·t·e^(−yt) is v0 / (y·e) at t = 1/y.
    let y = halflife_to_damping(op.halflife) / 2.0;
    let kick_per_radian = y * std::f32::consts::E;
    for hit in hits {
        let direction = hit.direction.normalize_or_zero();
        if direction == Vec3::ZERO || hit.strength <= 0.0 {
            continue;
        }
        let start = hit.joint.and_then(|j| op.joints.iter().position(|r| r.0 == j)).or_else(|| {
            op.joints
                .iter()
                .enumerate()
                .min_by(|a, b| {
                    let da = model.translation(usize::from(a.1.0)).distance_squared(hit.point);
                    let db = model.translation(usize::from(b.1.0)).distance_squared(hit.point);
                    da.total_cmp(&db)
                })
                .map(|(i, _)| i)
        });
        let Some(mut at) = start else { continue };
        let mut scale = 1.0;
        for _ in 0..op.depth {
            let (joint, gain) = op.joints[at];
            let pivot = model.translation(usize::from(joint));
            let mut axis = (hit.point - pivot).cross(direction);
            if axis.length_squared() < 1e-8 {
                // A hit through the pivot: tip about the horizontal axis across it.
                axis = Vec3::Y.cross(direction);
            }
            let axis = axis.normalize_or_zero();
            st.velocity[at] += axis * (op.gain * gain * hit.strength * scale * kick_per_radian);
            scale *= op.falloff;
            // Up to the nearest reactive ancestor.
            let mut j = s.parent(usize::from(joint));
            let mut next = None;
            while let Some(p) = j {
                if let Some(i) = op.joints.iter().position(|r| usize::from(r.0) == p) {
                    next = Some(i);
                    break;
                }
                j = s.parent(p);
            }
            let Some(n) = next else { break };
            at = n;
        }
    }
    for (o, v) in st.offset.iter_mut().zip(&mut st.velocity) {
        critical_spring(o, v, Vec3::ZERO, op.halflife, dt);
        if o.length() > op.max_angle {
            *o = o.normalize() * op.max_angle;
        }
    }
    // Root first (joints are in skeleton order), so children compose.
    for (&(joint, _), o) in op.joints.iter().zip(&st.offset) {
        if o.length_squared() > 1e-10 {
            rotate_joint(s, local, model, &ROOT, usize::from(joint), Quat::from_scaled_axis(*o));
        }
    }
}

/// One `SpringBones` chain's particles (character space): the current and
/// previous substep positions of every chain joint after the first.
#[derive(Clone, Debug, Default)]
pub(crate) struct ChainState {
    pos: Vec<Vec3>,
    prev: Vec<Vec3>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SpringState {
    chains: Vec<ChainState>,
    /// Unsimulated time, s (less than one substep after an update).
    pub(crate) carry: f32,
    init: bool,
}

impl SpringState {
    /// Re-seeds from the animated pose at the next update (LOD cut, teleport).
    pub(crate) fn reset(&mut self) {
        self.init = false;
    }
}

/// Moves character-space points into the frame after `moved`, by `share`
/// (1: they stay put in the world; 0: they move with the character).
fn carry_points(points: &mut [Vec3], moved: &RootDelta, share: f32) {
    if share <= 0.0 || (moved.translation.length_squared() < 1e-12 && moved.yaw.abs() < 1e-7) {
        return;
    }
    let undo = Quat::from_rotation_y(-moved.yaw);
    for p in points {
        let world_fixed = undo * (*p - moved.translation);
        *p += (world_fixed - *p) * share;
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) fn spring_bones(op: &SpringBonesOp, st: &mut SpringState, moved: &RootDelta, s: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, dt: f32) {
    let animated = |model: &ModelPose, chain: &[u16]| chain.iter().map(|&j| model.translation(usize::from(j))).collect::<Vec<_>>();
    if !st.init || st.chains.len() != op.chains.len() {
        st.chains = op
            .chains
            .iter()
            .map(|c| {
                let a = animated(model, c);
                ChainState { pos: a[1..].to_vec(), prev: a[1..].to_vec() }
            })
            .collect();
        st.carry = 0.0;
        st.init = true;
        return;
    }
    let h = SPRING_SUBSTEP;
    // A damped spring per particle toward its goal (ω² pull, 2ζω drag on
    // its velocity), as per-substep factors.
    let w = std::f32::consts::TAU * op.frequency;
    let pull = (w * h) * (w * h);
    let keep = (1.0 - 2.0 * op.damping * w * h).max(0.0);
    let gravity = Vec3::NEG_Y * op.gravity * h * h;
    st.carry += dt.max(0.0);
    let mut steps = (st.carry / h).floor() as u32;
    if steps > SPRING_MAX_SUBSTEPS {
        steps = SPRING_MAX_SUBSTEPS;
        st.carry = h * steps as f32;
    }
    st.carry -= h * steps as f32;
    for (chain, cs) in op.chains.iter().zip(&mut st.chains) {
        // The character's motion since the last update, once before its
        // substeps (the same total displacement at any frame rate).
        carry_points(&mut cs.pos, moved, op.inertia);
        carry_points(&mut cs.prev, moved, op.inertia);
        let a = animated(model, chain);
        for _ in 0..steps {
            let mut parent = a[0];
            for i in 1..chain.len() {
                let k = i - 1;
                let rest = a[i] - a[i - 1];
                let length = rest.length();
                let goal = parent + rest;
                let p = cs.pos[k];
                let mut next = p + (p - cs.prev[k]) * keep + (goal - p) * pull + gravity;
                // Bone length, then the cone about the animated direction.
                let mut d = (next - parent).normalize_or(rest.normalize_or(Vec3::Y));
                let r = rest.normalize_or(d);
                let angle = r.angle_between(d);
                if angle > op.max_angle {
                    let axis = r.cross(d).normalize_or(r.any_orthonormal_vector());
                    d = Quat::from_axis_angle(axis, op.max_angle) * r;
                }
                next = parent + d * length;
                cs.prev[k] = p;
                cs.pos[k] = next;
                parent = next;
            }
        }
        // Aim each joint at its simulated child, root first.
        for i in 0..chain.len() - 1 {
            let j = usize::from(chain[i]);
            let from = model.translation(usize::from(chain[i + 1])) - model.translation(j);
            let to = cs.pos[i] - model.translation(j);
            if from.length_squared() < 1e-10 || to.length_squared() < 1e-10 {
                continue;
            }
            let arc = Quat::from_rotation_arc(from.normalize(), to.normalize());
            rotate_joint(s, local, model, &ROOT, j, arc);
        }
    }
}

// --- ClothLattice ------------------------------------------------------------------------

/// How far the hem row's animated target follows the authored (leg) skin
/// instead of riding the girdle (`ClothBinding::new`'s default).
pub const CLOTH_LEG_FOLLOW: f32 = 0.7;
/// The highest animation LOD the cloth simulates at (A0, A1); above it the
/// lattice joints show the animated targets.
pub const CLOTH_MAX_LOD: usize = 1;

/// One cloth part bound to a character's skeleton (physics.md "Cloth",
/// `ClothLattice` op): the cooked lattice and where each particle's
/// animated target comes from.
///
/// The target blends, per row, between riding the girdle joint (`attach` ×
/// `offsets[i]`: the cloth hangs from the pelvis like a bell) and the
/// part's authored skin at the particle (it follows the legs). Morrowind's
/// robes are far too narrow at the hem for a run stride (census: +13..+133 %),
/// so a pure girdle ride would make the capsules stretch the hem; following
/// the legs toward the hem is the bone-chain answer UE `AnimDynamics` and
/// Magica `BoneCloth` setups use (the simulation then adds the swing).
#[derive(Clone, Debug)]
pub struct ClothBinding {
    pub lattice: sf_anim::cloth::ClothLattice,
    pub params: sf_anim::cloth::ClothParams,
    /// The girdle joint the top row rides.
    pub attach: u16,
    /// Per particle: its rest position in `attach`'s rest model frame.
    pub offsets: Vec<Vec3>,
    /// Per particle: the authored skin at its rest position, as
    /// `(joint, weight, the rest position in that joint's rest model frame)`.
    pub skin: Vec<[(u16, f32, Vec3); 4]>,
    /// Per particle: 0 rides the girdle, 1 follows the skin.
    pub follow: Vec<f32>,
}

impl ClothBinding {
    /// `skin`: per particle, the authored skin's joints (skeleton indices)
    /// and weights at its rest position; `leg_follow`: the hem row's follow
    /// (rows in between interpolate linearly from 0 at the pinned top row).
    #[allow(clippy::cast_precision_loss)]
    pub fn new(
        lattice: sf_anim::cloth::ClothLattice,
        params: sf_anim::cloth::ClothParams,
        attach: u16,
        offsets: Vec<Vec3>,
        skin: &[([u16; 4], [f32; 4])],
        rest_model: &[glam::Affine3A],
        leg_follow: f32,
    ) -> Self {
        let cols = usize::from(lattice.cols).max(1);
        let last = usize::from(lattice.rows).saturating_sub(1).max(1);
        let mut skin_out = Vec::with_capacity(lattice.particles.len());
        let mut follow = Vec::with_capacity(lattice.particles.len());
        for (i, p) in lattice.particles.iter().enumerate() {
            let mut s = [(0u16, 0.0f32, Vec3::ZERO); 4];
            if let Some((joints, weights)) = skin.get(i) {
                for k in 0..4 {
                    if weights[k] > 0.0
                        && let Some(m) = rest_model.get(usize::from(joints[k]))
                    {
                        s[k] = (joints[k], weights[k], m.inverse().transform_point3(p.rest));
                    }
                }
            }
            let has_skin = s.iter().any(|x| x.1 > 0.0);
            skin_out.push(s);
            let row = i / cols;
            follow.push(if has_skin { leg_follow.clamp(0.0, 1.0) * (row.min(last) as f32 / last as f32) } else { 0.0 });
        }
        Self { lattice, params, attach, offsets, skin: skin_out, follow }
    }

    /// Each particle's animated target in the posed model space.
    fn animated(&self, model: &ModelPose, out: &mut Vec<Vec3>) {
        out.clear();
        let girdle = model.mats.get(usize::from(self.attach)).copied().unwrap_or_default();
        for ((offset, skin), &f) in self.offsets.iter().zip(&self.skin).zip(&self.follow) {
            let ride = girdle.transform_point3(*offset);
            if f <= 0.0 {
                out.push(ride);
                continue;
            }
            let (mut sum, mut w) = (Vec3::ZERO, 0.0);
            for &(j, wk, local) in skin {
                if wk > 0.0
                    && let Some(m) = model.mats.get(usize::from(j))
                {
                    sum += m.transform_point3(local) * wk;
                    w += wk;
                }
            }
            out.push(if w > 1e-6 { ride.lerp(sum / w, f) } else { ride });
        }
    }
}

/// One cloth on a graph instance (`GraphInstance::set_cloth`): the
/// simulation state and the lattice joints' pose of the last update.
#[derive(Clone, Debug)]
pub struct ClothRun {
    pub binding: std::sync::Arc<ClothBinding>,
    state: sf_anim::cloth::ClothState,
    animated: Vec<Vec3>,
    capsules: Vec<sf_anim::cloth::Capsule>,
    rotations: Vec<Quat>,
    /// Whether the last update simulated (false: the joints show the
    /// animated targets: LOD above [`CLOTH_MAX_LOD`] or disabled).
    pub simulated: bool,
    /// Off: the lattice joints follow the animated targets (A/B view).
    pub enabled: bool,
    /// The last update's time in the op, µs (diagnostics; the cost gate).
    pub last_us: f32,
}

impl ClothRun {
    pub fn new(binding: std::sync::Arc<ClothBinding>) -> Self {
        Self {
            binding,
            state: sf_anim::cloth::ClothState::new(),
            animated: Vec::new(),
            capsules: Vec::new(),
            rotations: Vec::new(),
            simulated: false,
            enabled: true,
            last_us: 0.0,
        }
    }

    /// The lattice joints' positions, model space (empty before the first posed update).
    pub fn positions(&self) -> &[Vec3] {
        if self.simulated { self.state.positions() } else { &self.animated }
    }

    /// The lattice joints' rotations from rest, model space.
    pub fn rotations(&self) -> &[Quat] {
        &self.rotations
    }

    /// Re-seeds from the animation at the next simulated update.
    pub fn reset(&mut self) {
        self.state.reset();
    }

    /// The op: animated targets, posed capsules, one solver step (or the
    /// targets alone when not simulating), then the joint rotations.
    pub(crate) fn update(&mut self, model: &ModelPose, moved: RootDelta, dt: f32, lod: usize) {
        let started = std::time::Instant::now();
        let b = &*self.binding;
        b.animated(model, &mut self.animated);
        let simulate = self.enabled && lod <= CLOTH_MAX_LOD;
        if !simulate {
            if self.simulated {
                self.state.reset();
            }
            self.simulated = false;
            sf_anim::cloth::lattice_rotations(&b.lattice, &self.animated, &mut self.rotations);
            self.last_us = started.elapsed().as_secs_f32() * 1e6;
            return;
        }
        self.capsules.clear();
        for c in &b.lattice.colliders {
            let m = model.mats.get(usize::from(c.joint)).copied().unwrap_or_default();
            self.capsules.push(sf_anim::cloth::Capsule {
                a: m.transform_point3(c.center + c.axis * c.half_length),
                b: m.transform_point3(c.center - c.axis * c.half_length),
                radius: c.radius,
            });
        }
        let inputs = sf_anim::cloth::ClothInputs { animated: &self.animated, capsules: &self.capsules, moved, dt, acceleration: Vec3::ZERO };
        self.state.step(&b.lattice, &b.params, &inputs);
        self.simulated = true;
        self.state.rotations(&b.lattice, &mut self.rotations);
        self.last_us = started.elapsed().as_secs_f32() * 1e6;
    }
}

#[cfg(test)]
mod cloth_op_tests {
    use std::sync::Arc;

    use glam::{Affine3A, Vec3};
    use sf_anim::cloth::{ClothLattice, ClothParams, GridSpec};
    use sf_anim::{ModelPose, RootDelta};

    use super::{CLOTH_MAX_LOD, ClothBinding, ClothRun};

    /// A 4 × 6 skirt from 1 m down to 0.4 m on joint 0 (pelvis), skinned
    /// to joint 1 (a "leg" at the origin) below the top row.
    #[allow(clippy::cast_precision_loss)]
    fn binding(follow: f32) -> ClothBinding {
        let spec = GridSpec { rows: 4, cols: 6, ..GridSpec::default() };
        let rest: Vec<Vec3> = (0..24)
            .map(|i| {
                let (r, c) = (i / 6, i % 6);
                let a = std::f32::consts::TAU * c as f32 / 6.0;
                Vec3::new(0.2 * a.sin(), 1.0 - 0.2 * r as f32, 0.2 * a.cos())
            })
            .collect();
        let lattice = ClothLattice::grid(&spec, &rest, &[0; 24]);
        let rest_model = [Affine3A::IDENTITY; 2];
        let skin: Vec<([u16; 4], [f32; 4])> = (0..24).map(|i| if i < 6 { ([0; 4], [1.0, 0.0, 0.0, 0.0]) } else { ([1, 0, 0, 0], [1.0, 0.0, 0.0, 0.0]) }).collect();
        ClothBinding::new(lattice, ClothParams::default(), 0, rest, &skin, &rest_model, follow)
    }

    #[test]
    fn follow_blends_by_row_and_lod_switches_to_targets() {
        let b = binding(0.5);
        assert!(b.follow[..6].iter().all(|&f| f == 0.0));
        assert!((b.follow[23] - 0.5).abs() < 1e-6);
        let mut run = ClothRun::new(Arc::new(b));
        // The "leg" swings 0.3 m forward: the hem target moves by follow × 0.3.
        let mut model = ModelPose::new(2);
        model.mats[1] = Affine3A::from_translation(Vec3::new(0.0, 0.0, 0.3));
        run.update(&model, RootDelta::default(), 1.0 / 60.0, CLOTH_MAX_LOD + 1);
        assert!(!run.simulated);
        let hem = run.positions()[18];
        assert!((hem.z - (0.2 + 0.15)).abs() < 1e-5, "hem target {hem}");
        assert!((run.positions()[0].z - 0.2).abs() < 1e-6, "top row rides the girdle");
        assert_eq!(run.rotations().len(), 24);
        // Simulated at A0: seeds, then stays within its max distance of the targets.
        for _ in 0..30 {
            run.update(&model, RootDelta::default(), 1.0 / 60.0, 0);
        }
        assert!(run.simulated);
        assert_eq!(run.positions().len(), 24);
        assert!(run.positions().iter().all(|p| p.is_finite()));
        assert!(run.last_us >= 0.0);
    }
}
