//! Auto-align: the retarget pose that brings a rig's rest pose to the
//! canonical pose (Y up, facing +Z, left = +X; limbs in a T-pose), as UE5's
//! IK Retargeter "Auto Align" does per chain, but against fixed canonical
//! directions instead of another rig, so each rig is aligned once (N maps,
//! not N²).
//!
//! 1. **Facing**: the rig's left (left minus right limbs) and up (hips to
//!    head) are snapped to the nearest axes; if they are not +X and +Y,
//!    the topmost joint above the hips is turned so they are. A rig that
//!    faces −Z (or is Z up) still lines up with the hub.
//! 2. **Matched trunk**: the hips, spine and neck joints keep their
//!    (facing-corrected) rest direction, so each rig keeps its own bind
//!    curvature and a source at rest lands on the target at its bind (UE5's
//!    matched retarget pose: both retarget poses show one posture). Every
//!    standing rig's rest trunk is that posture already; straightening it
//!    on both rigs made clips carry the source's rest curvature onto the
//!    target in place of its own. A trunk bone more than 45° from up (a rig
//!    not standing in its rest) points up (+Y) instead.
//! 3. **Swing alignment** of the limbs, whose rests really differ (T against
//!    A pose), parents first, by the shortest rotation that points the bone
//!    at its child: upper and lower legs down (−Y); feet keep their original
//!    (facing corrected) direction to the toes, so they stay flat, unless
//!    that direction is degenerate (within 30° of the bone above's line, as
//!    in a mocap zero pose with every bone in one line), when they point
//!    forward (+Z, flat) instead; upper arms, lower arms and hands along ±X;
//!    non-thumb finger segments along ±X.
//!    Shoulders, thumbs, head, eyes and jaw keep their rest rotation.
//!
//! Twist about the bone axis is left alone: only directions are aligned.
//!
//! **Retarget-pose offsets** ([`PoseOffset`], [`offset_retarget_pose`]):
//! afterwards a rig may edit its retarget pose per humanoid limb bone, as
//! UE5's IK Retargeter "Edit Retarget Pose" does per chain: a rotation in
//! the canonical frame, given for the left side and mirrored for the right,
//! that turns the bone and everything below it. On a source it moves the
//! posture its motion is measured from (a rig whose clips hold the
//! clavicles lower than its rest: deltas then start from the lowered
//! clavicle); on a target it moves where that posture lands. Offsets are
//! data (the adapter's per-source and per-target settings); none leaves the
//! retarget pose untouched.

use glam::{Affine3A, Mat3, Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::{Skeleton, Trs};

use crate::humanoid::{Finger, HumanoidBone, Limb, Side};
use crate::rig::{JointName, RigMap, model_of};

/// What auto-align changed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AlignReport {
    /// Rotation applied to the topmost joint to face +Z with Y up, degrees.
    pub facing_correction_deg: f32,
    /// Per aligned joint, how far it was rotated, degrees.
    pub rotated: Vec<(JointName, f32)>,
    /// Largest angle between an aligned bone and its canonical direction
    /// after alignment, degrees (≈ 0 unless joints have zero length).
    pub max_residual_deg: f32,
}

/// One alignment target: point `joint` at `child` along `dir` (model space).
struct Target {
    joint: usize,
    child: usize,
    dir: Dir,
}

enum Dir {
    Fixed(Vec3),
    /// Keep the facing-corrected rest direction, or this one when the rest
    /// direction is degenerate (collinear with the parent bone, either way).
    Rest(Vec3),
    /// A trunk bone: keep the facing-corrected rest direction (the matched
    /// retarget pose), or +Y when it lies more than 45° from up.
    Trunk,
}

/// A trunk bone within 45° of up keeps its rest direction.
const UPRIGHT_COS: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// A chain end whose rest direction lies within 30° of its parent bone's
/// line (parallel or opposite) has no usable rest direction.
const DEGENERATE_COS: f32 = 0.866;

fn rotation_of(m: &Affine3A) -> Quat {
    m.to_scale_rotation_translation().1.normalize()
}

fn snap(v: Vec3) -> Vec3 {
    let a = v.abs();
    if a.x >= a.y && a.x >= a.z {
        Vec3::X * v.x.signum()
    } else if a.y >= a.z {
        Vec3::Y * v.y.signum()
    } else {
        Vec3::Z * v.z.signum()
    }
}

/// Computes the retarget pose of `map` on `skeleton` (from the rest pose;
/// any retarget pose already in `map` is ignored).
#[allow(clippy::too_many_lines)]
pub fn auto_align(skeleton: &Skeleton, map: &RigMap) -> (Vec<(JointName, Quat)>, AlignReport) {
    let mut report = AlignReport::default();
    let joint = |b: HumanoidBone| map.joint(b).and_then(|n| skeleton.find(n));
    let mut local: Vec<Trs> = skeleton.rest().to_vec();
    let mut model = model_of(skeleton, &local);
    let pos = |model: &[Affine3A], j: usize| Vec3::from(model[j].translation);

    // 1. Facing.
    if let Some(hips) = joint(HumanoidBone::Hips) {
        let mut left = Vec3::ZERO;
        for l in [Limb::UpperLeg, Limb::UpperArm, Limb::Shoulder, Limb::Foot, Limb::Hand] {
            let (a, b) = (HumanoidBone::limb(Side::Left, l), HumanoidBone::limb(Side::Right, l));
            if let (Some(a), Some(b)) = (a.and_then(joint), b.and_then(joint)) {
                left += pos(&model, a) - pos(&model, b);
            }
        }
        let top_bone =
            [HumanoidBone::Head, HumanoidBone::Neck, HumanoidBone::UpperChest, HumanoidBone::Chest, HumanoidBone::Spine].into_iter().find_map(joint);
        if let Some(top_bone) = top_bone
            && left.length() > 1e-4
        {
            let up = snap(pos(&model, top_bone) - pos(&model, hips));
            let left = left - up * left.dot(up);
            if left.length() > 1e-4 {
                let left = snap(left);
                let forward = left.cross(up);
                // Rows: the detected axes, so it maps left → X, up → Y, forward → Z.
                let c = Quat::from_mat3(&Mat3::from_cols(left, up, forward).transpose());
                let angle = c.angle_between(Quat::IDENTITY);
                if angle > 1e-4 {
                    let mut top = hips;
                    while let Some(p) = skeleton.parent(top) {
                        top = p;
                    }
                    local[top].rotation = (c * local[top].rotation).normalize();
                    model = model_of(skeleton, &local);
                    report.facing_correction_deg = angle.to_degrees();
                }
            }
        }
    }
    let rest_model = model.clone();

    // 2. Targets, in joint order (parents first).
    let mut targets = Vec::new();
    let mut push = |j: Option<usize>, c: Option<usize>, dir: Dir| {
        if let (Some(joint), Some(child)) = (j, c) {
            targets.push(Target { joint, child, dir });
        }
    };
    // Trunk: hips, the spine path, the neck path, each keeping its rest
    // direction to the next joint (module docs, step 2).
    let torso_top = [HumanoidBone::Neck, HumanoidBone::Head].into_iter().find_map(joint);
    if let (Some(hips), Some(top)) = (joint(HumanoidBone::Hips), torso_top) {
        let mut path = vec![top];
        let mut j = top;
        while let Some(p) = skeleton.parent(j) {
            path.push(p);
            if p == hips {
                break;
            }
            j = p;
        }
        if path.last() == Some(&hips) {
            path.reverse();
            for w in path.windows(2) {
                push(Some(w[0]), Some(w[1]), Dir::Trunk);
            }
        }
    }
    if let (Some(neck), Some(head)) = (joint(HumanoidBone::Neck), joint(HumanoidBone::Head))
        && skeleton.is_ancestor(neck, head)
    {
        let mut path = vec![head];
        let mut j = head;
        while let Some(p) = skeleton.parent(j) {
            path.push(p);
            if p == neck {
                break;
            }
            j = p;
        }
        path.reverse();
        for w in path.windows(2) {
            push(Some(w[0]), Some(w[1]), Dir::Trunk);
        }
    }
    for s in [Side::Left, Side::Right] {
        let limb = |l: Limb| HumanoidBone::limb(s, l).and_then(joint);
        let out = Vec3::X * s.sign();
        push(limb(Limb::UpperLeg), limb(Limb::LowerLeg), Dir::Fixed(Vec3::NEG_Y));
        push(limb(Limb::LowerLeg), limb(Limb::Foot), Dir::Fixed(Vec3::NEG_Y));
        push(limb(Limb::Foot), limb(Limb::Toes), Dir::Rest(Vec3::Z));
        push(limb(Limb::UpperArm), limb(Limb::LowerArm), Dir::Fixed(out));
        push(limb(Limb::LowerArm), limb(Limb::Hand), Dir::Fixed(out));
        let finger = |f: Finger, k: usize| HumanoidBone::finger(s, f, k).and_then(joint);
        let knuckle = [Finger::Middle, Finger::Index, Finger::Ring, Finger::Little].into_iter().find_map(|f| finger(f, 0));
        push(limb(Limb::Hand), knuckle, Dir::Fixed(out));
        for f in [Finger::Index, Finger::Middle, Finger::Ring, Finger::Little] {
            push(finger(f, 0), finger(f, 1), Dir::Fixed(out));
            push(finger(f, 1), finger(f, 2), Dir::Fixed(out));
            // The distal segment points at its end site, when there is exactly one child.
            if let Some(d) = finger(f, 2) {
                let children: Vec<usize> = skeleton.subtree(d).skip(1).filter(|&k| skeleton.parent(k) == Some(d)).collect();
                if let [c] = children[..] {
                    push(Some(d), Some(c), Dir::Fixed(out));
                }
            }
        }
    }
    targets.sort_by_key(|t| t.joint);
    targets.dedup_by_key(|t| t.joint);

    let goal_of = |t: &Target| match t.dir {
        Dir::Fixed(v) => v,
        Dir::Rest(fallback) => {
            let d = (pos(&rest_model, t.child) - pos(&rest_model, t.joint)).normalize_or_zero();
            let above =
                skeleton.parent(t.joint).map_or(Vec3::ZERO, |p| (pos(&rest_model, t.joint) - pos(&rest_model, p)).normalize_or_zero());
            if d.dot(above).abs() > DEGENERATE_COS { fallback } else { d }
        }
        Dir::Trunk => {
            let d = (pos(&rest_model, t.child) - pos(&rest_model, t.joint)).normalize_or_zero();
            if d.dot(Vec3::Y) >= UPRIGHT_COS { d } else { Vec3::Y }
        }
    };
    for t in &targets {
        let goal = goal_of(t);
        let d = (pos(&model, t.child) - pos(&model, t.joint)).normalize_or_zero();
        if d == Vec3::ZERO || goal == Vec3::ZERO {
            continue;
        }
        let r = Quat::from_rotation_arc(d, goal);
        if r.angle_between(Quat::IDENTITY) < 1e-5 {
            continue;
        }
        let parent = skeleton.parent(t.joint).map_or(Quat::IDENTITY, |p| rotation_of(&model[p]));
        let world = r * rotation_of(&model[t.joint]);
        local[t.joint].rotation = (parent.inverse() * world).normalize();
        model = model_of(skeleton, &local);
    }

    // Residuals and the changed joints.
    for t in &targets {
        let goal = goal_of(t);
        let d = (pos(&model, t.child) - pos(&model, t.joint)).normalize_or_zero();
        if d != Vec3::ZERO && goal != Vec3::ZERO {
            report.max_residual_deg = report.max_residual_deg.max(d.angle_between(goal).to_degrees());
        }
    }
    let mut pose = Vec::new();
    for (j, (now, rest)) in local.iter().zip(skeleton.rest()).enumerate() {
        let angle = now.rotation.angle_between(rest.rotation);
        if angle > 1e-5 {
            pose.push((skeleton.name(j).to_owned(), now.rotation));
            report.rotated.push((skeleton.name(j).to_owned(), angle.to_degrees()));
        }
    }
    (pose, report)
}

/// One retarget-pose edit (module docs): a rotation of a humanoid limb bone
/// and its subtree in the canonical frame (Y up, facing +Z, left = +X), for
/// the left side; the right side gets its mirror image (about the YZ plane).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoseOffset {
    pub limb: Limb,
    /// Degrees about the forward axis, outer end up and out (+) or down and
    /// in (−): raises a clavicle or a T-pose arm, abducts a leg.
    #[serde(default)]
    pub raise_deg: f32,
    /// Degrees about the up axis, outer end forward (+) or back (−).
    #[serde(default)]
    pub forward_deg: f32,
}

impl PoseOffset {
    /// The model-space rotation on `side` (identity on [`Side::Center`]).
    /// Raise first, then forward.
    pub fn rotation(&self, side: Side) -> Quat {
        let s = side.sign();
        Quat::from_rotation_y(-s * self.forward_deg.to_radians()) * Quat::from_rotation_z(s * self.raise_deg.to_radians())
    }
}

/// The retarget pose of `map` with `offsets` applied (module docs), parents
/// first: each edited joint's local rotation becomes `P⁻¹ · O · P · local`
/// (`P` its parent's model rotation), so it and its subtree turn by `O` in
/// model space. Joints `map` lacks are skipped; no non-zero offset returns
/// `map.retarget_pose` unchanged.
pub fn offset_retarget_pose(skeleton: &Skeleton, map: &RigMap, offsets: &[PoseOffset]) -> Vec<(JointName, Quat)> {
    let mut edits: Vec<(usize, Quat)> = offsets
        .iter()
        .flat_map(|o| {
            [Side::Left, Side::Right].map(|s| {
                let j = HumanoidBone::limb(s, o.limb).and_then(|b| map.joint(b)).and_then(|n| skeleton.find(n))?;
                Some((j, o.rotation(s)))
            })
        })
        .flatten()
        .filter(|(_, q)| q.angle_between(Quat::IDENTITY) > 1e-6)
        .collect();
    if edits.is_empty() {
        return map.retarget_pose.clone();
    }
    edits.sort_by_key(|e| e.0);
    let mut local = map.retarget_pose_local(skeleton);
    for &(j, o) in &edits {
        let model = model_of(skeleton, &local);
        let parent = skeleton.parent(j).map_or(Quat::IDENTITY, |p| rotation_of(&model[p]));
        local[j].rotation = (parent.inverse() * o * parent * local[j].rotation).normalize();
    }
    let mut pose = map.retarget_pose.clone();
    for &(j, _) in &edits {
        let name = skeleton.name(j);
        match pose.iter_mut().find(|(n, _)| n == name) {
            Some(entry) => entry.1 = local[j].rotation,
            None => pose.push((name.to_owned(), local[j].rotation)),
        }
    }
    pose
}

impl RigMap {
    /// Fills [`RigMap::retarget_pose`] with [`auto_align`].
    pub fn auto_align(&mut self, skeleton: &Skeleton) -> AlignReport {
        let (pose, report) = auto_align(skeleton, self);
        self.retarget_pose = pose;
        report
    }

    /// Applies retarget-pose offsets ([`offset_retarget_pose`]) to
    /// [`RigMap::retarget_pose`]; call after [`RigMap::auto_align`].
    pub fn offset_retarget_pose(&mut self, skeleton: &Skeleton, offsets: &[PoseOffset]) {
        self.retarget_pose = offset_retarget_pose(skeleton, self, offsets);
    }
}
