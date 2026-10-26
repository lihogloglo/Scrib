//! Auto-align: the retarget pose that brings a rig's rest pose to the
//! canonical T-pose (Y up, facing +Z, left = +X), as UE5's IK Retargeter
//! "Auto Align" does per chain, but against fixed canonical directions
//! instead of another rig, so each rig is aligned once (N maps, not N²).
//!
//! 1. **Facing**: the rig's left (left minus right limbs) and up (hips to
//!    head) are snapped to the nearest axes; if they are not +X and +Y,
//!    the topmost joint above the hips is turned so they are. A rig that
//!    faces −Z (or is Z up) still lines up with the hub.
//! 2. **Swing alignment**, parents first, by the shortest rotation that
//!    points the bone at its child: hips, spine and neck joints up (+Y);
//!    upper and lower legs down (−Y); feet keep their original (facing
//!    corrected) direction to the toes, so they stay flat, unless that
//!    direction is degenerate (within 30° of the bone above's line, as in
//!    a mocap zero pose with every bone in one line), when they point
//!    forward (+Z, flat) instead; upper arms,
//!    lower arms and hands along ±X; non-thumb finger segments along ±X.
//!    Shoulders, thumbs, head, eyes and jaw keep their rest rotation.
//!
//! Twist about the bone axis is left alone: only directions are aligned.

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
}

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
    // Torso: hips, the spine path, the neck path, each pointing at the next joint.
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
                push(Some(w[0]), Some(w[1]), Dir::Fixed(Vec3::Y));
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
            push(Some(w[0]), Some(w[1]), Dir::Fixed(Vec3::Y));
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

impl RigMap {
    /// Fills [`RigMap::retarget_pose`] with [`auto_align`].
    pub fn auto_align(&mut self, skeleton: &Skeleton) -> AlignReport {
        let (pose, report) = auto_align(skeleton, self);
        self.retarget_pose = pose;
        report
    }
}
