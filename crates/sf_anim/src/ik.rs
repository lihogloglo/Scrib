//! IK solvers on model-space poses.
//!
//! Solvers compute model-space rotation deltas; [`rotate_joint`] folds a
//! delta into the local pose and refreshes the joint's subtree in the model
//! pose, so solvers chain (foot placement, then toes; spine, then head)
//! without a full local-to-model pass in between.
//!
//! - [`two_bone`]: analytic (law of cosines), knee toward a pole point,
//!   hinge-preserving, with a reach clamp (Holden, *Simple Two Joint IK*).
//! - [`aim_chain`]: look-at spread over a chain with per-joint weights and
//!   limits, plus a total cone limit.
//! - [`fabrik`] / [`fabrik_chain`]: Aristidou & Lasenby 2011.
//! - [`ccd_chain`]: cyclic coordinate descent (Wang & Chen 1991), as a fallback.
//! - [`two_bone_ankle_ik`]: digitigrade and unguligrade legs (thigh, shin,
//!   metatarsal, toe): the metatarsal keeps its animated angle to the shin,
//!   bent further as the leg compresses, and the rest solves as two bones.
//! - [`solve_limb`]: any limb by its [`LimbSolver`], with an optional
//!   effector rotation (rig-profile.md).
//! - [`FullBody`]: XPBD full-body IK over a rig profile: several effectors
//!   at once, the pelvis and spine giving way (animation.md "Layer 2").

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::math::{affine_rotation, any_perpendicular, rotation_between_frames};
use crate::{LocalPose, ModelPose, Skeleton, update_subtree};

mod full_body;
pub use full_body::{BodyJoint, BodyRole, Effector, FullBody, FullBodyScratch, FullBodySettings, Hinge, NO_JOINT};

/// Applies a model-space rotation `delta` to `joint` about its own pivot,
/// updating its local rotation and its subtree's model transforms. Parent
/// scale is assumed uniform.
pub fn rotate_joint(skeleton: &Skeleton, local: &mut LocalPose, model: &mut ModelPose, root: &Affine3A, joint: usize, delta: Quat) {
    let parent_rot = match skeleton.parent(joint) {
        Some(p) => model.rotation(p),
        None => affine_rotation(root),
    };
    let current = local.get(joint).rotation;
    let rotated = (parent_rot.inverse() * delta * parent_rot * current).normalize();
    local.set_rotation(joint, rotated);
    update_subtree(skeleton, local, model, root, joint);
}

/// Scales a rotation's angle by `w` (0 = identity, 1 = `q`).
pub fn weighted(q: Quat, w: f32) -> Quat {
    if w >= 1.0 { q } else { Quat::IDENTITY.slerp(q, w.max(0.0)) }
}

/// Limits a rotation's angle to `max_angle` radians.
pub fn clamp_angle(q: Quat, max_angle: f32) -> Quat {
    let (axis, angle) = q.to_axis_angle();
    if angle.abs() <= max_angle { q } else { Quat::from_axis_angle(axis, max_angle.copysign(angle)) }
}

/// Two-bone IK settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoBone {
    /// Largest extension as a fraction of the chain length. Below 1, the
    /// chain never locks fully straight, which keeps the knee plane
    /// defined.
    pub max_extension: f32,
    /// Blend toward the solution, 0–1.
    pub weight: f32,
}

impl Default for TwoBone {
    fn default() -> Self {
        Self { max_extension: 0.9999, weight: 1.0 }
    }
}

/// Model-space rotation deltas for a two-bone chain `a → b → c` (hip,
/// knee, ankle). Apply `root` to `a` about `a`, then `mid` to `b` about its
/// new position.
///
/// The knee lands in the half plane through `a` and the target that
/// contains `pole`, so it can never flip to the other side; the solution is
/// continuous as long as the pole is not on the line to the target.
/// Unreachable targets are clamped to the chain's reach along the target
/// direction. The upper bone keeps its hinge plane aligned with the knee's
/// bend plane, and the lower bone rotates about the hinge axis only.
pub fn two_bone(a: Vec3, b: Vec3, c: Vec3, target: Vec3, pole: Vec3, max_extension: f32) -> (Quat, Quat) {
    let lab = a.distance(b);
    let lbc = b.distance(c);
    if lab < 1e-6 || lbc < 1e-6 {
        return (Quat::IDENTITY, Quat::IDENTITY);
    }
    let to_target = target - a;
    let dist = to_target.length();
    let u = if dist > 1e-6 { to_target / dist } else { (c - a).normalize_or(Vec3::Y) };
    let min_reach = (lab - lbc).abs() + 1e-5;
    let max_reach = ((lab + lbc) * max_extension).max(min_reach);
    let lat = dist.clamp(min_reach, max_reach);
    // Knee direction: the pole, else the current knee, projected off the reach line.
    let project = |p: Vec3| {
        let d = p - a;
        d - u * d.dot(u)
    };
    let mut v = project(pole);
    if v.length_squared() < 1e-10 {
        v = project(b);
    }
    let v = v.normalize_or(any_perpendicular(u));
    let cos_a = ((lab * lab + lat * lat - lbc * lbc) / (2.0 * lab * lat)).clamp(-1.0, 1.0);
    let sin_a = (1.0 - cos_a * cos_a).max(0.0).sqrt();
    let b_new = a + (u * cos_a + v * sin_a) * lab;
    let c_new = a + u * lat;
    // Bend-plane normals: (b - a) × (c - b) before and after.
    let n_new = v.cross(u);
    let n_old = (b - a).cross(c - b);
    let n_old = if n_old.length_squared() > 1e-12 * lab * lab * lbc * lbc { n_old } else { n_new };
    let root = rotation_between_frames(b - a, n_old, b_new - a, n_new);
    let c_moved = a + root * (c - a);
    let mid = Quat::from_rotation_arc((c_moved - b_new).normalize(), (c_new - b_new).normalize());
    (root, mid)
}

/// Solves a two-bone chain on a pose: `joints = [upper, mid, end]`, each a
/// descendant of the previous. Returns the remaining distance to `target`.
#[allow(clippy::too_many_arguments)]
pub fn two_bone_ik(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    joints: [usize; 3],
    target: Vec3,
    pole: Vec3,
    settings: TwoBone,
) -> f32 {
    let [ja, jb, jc] = joints;
    let (a, b, c) = (model.translation(ja), model.translation(jb), model.translation(jc));
    let (r0, r1) = two_bone(a, b, c, target, pole, settings.max_extension);
    rotate_joint(skeleton, local, model, root, ja, weighted(r0, settings.weight));
    rotate_joint(skeleton, local, model, root, jb, weighted(r1, settings.weight));
    model.translation(jc).distance(target)
}

/// One joint of an aim chain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AimJoint {
    pub joint: usize,
    /// Share of the remaining correction this joint takes (the last joint
    /// usually takes 1).
    pub weight: f32,
    /// Largest rotation this joint may add, radians.
    pub limit: f32,
}

/// Aim settings: which axis of the end joint aims, and limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aim {
    /// Aim axis in the end joint's local frame (e.g. the eyes' forward).
    pub forward: Vec3,
    /// Largest angle between the animated aim and the corrected one.
    pub total_limit: f32,
    /// Blend toward the solution, 0–1.
    pub weight: f32,
}

/// Turns a chain (spine → neck → head, root first) so that `end`'s
/// `forward` axis points at `target`. Each joint takes its weight's share
/// of what remains, clamped to its limit. Returns the remaining angle.
#[allow(clippy::too_many_arguments)]
pub fn aim_chain(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    chain: &[AimJoint],
    end: usize,
    target: Vec3,
    aim: Aim,
) -> f32 {
    let aim_dir = |model: &ModelPose| (model.rotation(end) * aim.forward).normalize_or(Vec3::Z);
    let origin = model.translation(end);
    let animated = aim_dir(model);
    let wanted = (target - origin).normalize_or(animated);
    // Total cone limit around the animated aim.
    let full = Quat::from_rotation_arc(animated, wanted);
    let goal_dir = clamp_angle(full, aim.total_limit) * animated;
    let goal_point = origin + goal_dir * (target - origin).length().max(1e-3);
    for j in chain {
        let from = aim_dir(model);
        let to = (goal_point - model.translation(end)).normalize_or(from);
        let r = clamp_angle(weighted(Quat::from_rotation_arc(from, to), j.weight), j.limit);
        rotate_joint(skeleton, local, model, root, j.joint, weighted(r, aim.weight));
    }
    let to = (target - model.translation(end)).normalize_or(Vec3::Z);
    aim_dir(model).angle_between(to)
}

/// FABRIK on joint positions (`positions[0]` stays fixed). Returns the
/// final distance from the end to `target`.
pub fn fabrik(positions: &mut [Vec3], target: Vec3, tolerance: f32, max_iterations: usize) -> f32 {
    let n = positions.len();
    if n < 2 {
        return positions.first().map_or(0.0, |p| p.distance(target));
    }
    let lengths: Vec<f32> = positions.windows(2).map(|w| w[0].distance(w[1])).collect();
    let total: f32 = lengths.iter().sum();
    let base = positions[0];
    if base.distance(target) >= total {
        // Out of reach: straighten toward the target.
        let dir = (target - base).normalize_or(Vec3::Y);
        for i in 1..n {
            positions[i] = positions[i - 1] + dir * lengths[i - 1];
        }
        return positions[n - 1].distance(target);
    }
    for _ in 0..max_iterations {
        if positions[n - 1].distance(target) <= tolerance {
            break;
        }
        // Backward: end to target, then toward the base.
        positions[n - 1] = target;
        for i in (0..n - 1).rev() {
            let dir = (positions[i] - positions[i + 1]).normalize_or(Vec3::Y);
            positions[i] = positions[i + 1] + dir * lengths[i];
        }
        // Forward: base back in place, then out to the end.
        positions[0] = base;
        for i in 1..n {
            let dir = (positions[i] - positions[i - 1]).normalize_or(Vec3::Y);
            positions[i] = positions[i - 1] + dir * lengths[i - 1];
        }
    }
    positions[n - 1].distance(target)
}

/// FABRIK on a pose: `chain` lists joints from the fixed one to the end
/// effector, each a descendant of the previous. Returns the remaining distance.
#[allow(clippy::too_many_arguments)]
pub fn fabrik_chain(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    chain: &[usize],
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
) -> f32 {
    let mut solved: Vec<Vec3> = chain.iter().map(|&j| model.translation(j)).collect();
    fabrik(&mut solved, target, tolerance, max_iterations);
    for i in 0..chain.len().saturating_sub(1) {
        let from = model.translation(chain[i + 1]) - model.translation(chain[i]);
        let to = solved[i + 1] - model.translation(chain[i]);
        if from.length_squared() > 1e-12 && to.length_squared() > 1e-12 {
            let delta = Quat::from_rotation_arc(from.normalize(), to.normalize());
            rotate_joint(skeleton, local, model, root, chain[i], delta);
        }
    }
    chain.last().map_or(0.0, |&e| model.translation(e).distance(target))
}

/// CCD on a pose: `chain` from the fixed joint to the end effector. Each
/// joint turns at most `max_step` radians per iteration. Returns the
/// remaining distance.
#[allow(clippy::too_many_arguments)]
pub fn ccd_chain(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    chain: &[usize],
    target: Vec3,
    tolerance: f32,
    max_iterations: usize,
    max_step: f32,
) -> f32 {
    let Some(&end) = chain.last() else { return 0.0 };
    for _ in 0..max_iterations {
        if model.translation(end).distance(target) <= tolerance {
            break;
        }
        for &j in chain[..chain.len() - 1].iter().rev() {
            let pivot = model.translation(j);
            let from = model.translation(end) - pivot;
            let to = target - pivot;
            if from.length_squared() < 1e-12 || to.length_squared() < 1e-12 {
                continue;
            }
            let delta = clamp_angle(Quat::from_rotation_arc(from.normalize(), to.normalize()), max_step);
            rotate_joint(skeleton, local, model, root, j, delta);
        }
    }
    model.translation(end).distance(target)
}

/// How a limb is solved (rig-profile.md "Solvers"). Each solver works on
/// the last joints of the limb's chain ([`LimbSolver::span`]); joints
/// before them (a clavicle, a scapula) stay as animated.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimbSolver {
    /// Analytic two-bone: upper, mid, end (hip, knee, ankle).
    #[default]
    TwoBone,
    /// Digitigrade or unguligrade leg: thigh, shin, metatarsal, toe.
    TwoBoneAnkle,
    /// FABRIK over the whole chain (insect legs, tentacles, long necks).
    Fabrik,
    /// CCD over the whole chain.
    Ccd,
    /// Decorative: never solved.
    None,
}

impl LimbSolver {
    /// How many trailing joints of a chain of `len` joints the solver moves.
    pub fn span(self, len: usize) -> usize {
        match self {
            Self::TwoBone => len.min(3),
            Self::TwoBoneAnkle => len.min(4),
            Self::Fabrik | Self::Ccd => len,
            Self::None => 0,
        }
    }

    /// The default solver for a chain of `len` joints.
    pub fn for_len(len: usize) -> Self {
        match len {
            0 | 1 => Self::None,
            2 => Self::Ccd,
            3 => Self::TwoBone,
            4 | 5 => Self::TwoBoneAnkle,
            _ => Self::Fabrik,
        }
    }
}

/// Settings of the metatarsal in [`two_bone_ankle_ik`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnkleShare {
    /// How much extra hock bend a compression adds: at 1 the hock bend
    /// grows by the same fraction the leg is shortened by. 0 keeps the
    /// metatarsal rigid to the shin.
    pub share: f32,
}

impl Default for AnkleShare {
    fn default() -> Self {
        Self { share: 0.5 }
    }
}

/// Four-joint leg IK: `joints = [thigh, shin, hock, toe]`. The metatarsal
/// (hock → toe) keeps its animated angle to the shin, bent further by
/// `ankle.share` × the compression ratio when the target is closer than the
/// animated toe, then thigh and shin solve as two bones to the toe with the
/// shin and metatarsal as one rigid virtual bone (the common game approach
/// for dog and horse legs). The knee goes toward `pole`. Returns the
/// remaining distance.
#[allow(clippy::too_many_arguments)]
pub fn two_bone_ankle_ik(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    joints: [usize; 4],
    target: Vec3,
    pole: Vec3,
    settings: TwoBone,
    ankle: AnkleShare,
) -> f32 {
    let [ja, jb, jc, jd] = joints;
    let (a, b, c, d) = (model.translation(ja), model.translation(jb), model.translation(jc), model.translation(jd));
    let r0 = a.distance(d);
    let r = a.distance(target);
    if r0 > 1e-6 && r < r0 && ankle.share > 0.0 {
        // Compression: bend the hock further about its own hinge axis.
        let (shin, meta) = (c - b, d - c);
        let axis = shin.cross(meta);
        if axis.length_squared() > 1e-12 * shin.length_squared() * meta.length_squared() {
            let bend = shin.angle_between(meta);
            let extra = bend * ankle.share * ((r0 - r) / r0).min(1.0);
            let delta = Quat::from_axis_angle(axis.normalize(), extra * settings.weight.clamp(0.0, 1.0));
            rotate_joint(skeleton, local, model, root, jc, delta);
        }
    }
    let d = model.translation(jd);
    let (r0, r1) = two_bone(a, b, d, target, pole, settings.max_extension);
    rotate_joint(skeleton, local, model, root, ja, weighted(r0, settings.weight));
    rotate_joint(skeleton, local, model, root, jb, weighted(r1, settings.weight));
    model.translation(jd).distance(target)
}

/// A limb IK goal (model space).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LimbGoal {
    pub position: Vec3,
    /// Rotation of the effector, if it should be matched.
    pub rotation: Option<Quat>,
    /// Point the mid joint (knee, elbow) bends toward ([`limb_pole`]).
    pub pole: Vec3,
    /// Blend toward the solution, 0–1.
    pub weight: f32,
}

/// Solves a limb chain (`joints`, root first, effector last) with `solver`
/// on its last [`LimbSolver::span`] joints, then turns the effector to the
/// goal's rotation. A two-bone solver on a chain too short for it falls
/// back to FABRIK; FABRIK runs up to 16 iterations and CCD up to 64, to 1 mm.
/// Returns the remaining distance.
pub fn solve_limb(
    skeleton: &Skeleton,
    local: &mut LocalPose,
    model: &mut ModelPose,
    root: &Affine3A,
    joints: &[usize],
    solver: LimbSolver,
    goal: &LimbGoal,
) -> f32 {
    let Some(&end) = joints.last() else {
        return 0.0;
    };
    let span = solver.span(joints.len());
    let chain = &joints[joints.len() - span..];
    let settings = TwoBone { weight: goal.weight.clamp(0.0, 1.0), ..TwoBone::default() };
    let w = settings.weight;
    if w <= 0.0 {
        return model.translation(end).distance(goal.position);
    }
    match (solver, chain) {
        (LimbSolver::TwoBone, &[a, b, c]) => {
            two_bone_ik(skeleton, local, model, root, [a, b, c], goal.position, goal.pole, settings);
        }
        (LimbSolver::TwoBoneAnkle, &[a, b, c, d]) => {
            two_bone_ankle_ik(skeleton, local, model, root, [a, b, c, d], goal.position, goal.pole, settings, AnkleShare::default());
        }
        (LimbSolver::Fabrik | LimbSolver::TwoBone | LimbSolver::TwoBoneAnkle, _) if chain.len() >= 2 => {
            let target = model.translation(end).lerp(goal.position, w);
            fabrik_chain(skeleton, local, model, root, chain, target, 1e-3, 16);
        }
        (LimbSolver::Ccd, _) if chain.len() >= 2 => {
            let target = model.translation(end).lerp(goal.position, w);
            ccd_chain(skeleton, local, model, root, chain, target, 1e-3, 64, 0.5);
        }
        _ => {}
    }
    if let Some(q) = goal.rotation {
        let current = model.rotation(end);
        let delta = weighted((crate::math::align_hemisphere(q, current) * current.inverse()).normalize(), w);
        rotate_joint(skeleton, local, model, root, end, delta);
    }
    model.translation(end).distance(goal.position)
}

/// The pole point of a limb's solved span (`chain`: upper, knee, …, end):
/// the knee (`chain[1]`) pushed out along its animated bend, or along
/// `axis` (model space) when the chain is nearly straight (bend under 5%
/// of its length).
pub fn limb_pole(model: &ModelPose, chain: &[usize], axis: Option<Vec3>) -> Vec3 {
    let (Some(&first), Some(&last)) = (chain.first(), chain.last()) else {
        return Vec3::ZERO;
    };
    let mid = chain[1.min(chain.len() - 1)];
    let (hp, kp, fp) = (model.translation(first), model.translation(mid), model.translation(last));
    let length = chain.windows(2).map(|w| model.translation(w[0]).distance(model.translation(w[1]))).sum::<f32>().max(0.1);
    let line = fp - hp;
    let off = kp - hp;
    let side = off - line * (off.dot(line) / line.length_squared().max(1e-12));
    let animated = side.length() > 0.05 * length;
    let dir = match axis {
        Some(a) if !animated => a.normalize_or(Vec3::Z),
        _ => side.normalize_or(axis.unwrap_or(Vec3::Z)),
    };
    kp + dir * length
}
