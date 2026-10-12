//! Pose mirroring across the character's mirror plane (UE mirror data
//! tables, done in model space).
//!
//! Joint `j` takes its counterpart `m(j)`'s model transform reflected by
//! `S`, times a per-joint correction `C_j = (S · Rest_m(j))⁻¹ · Rest_j`
//! that maps the counterpart's frame onto `j`'s, so the rest pose mirrors
//! onto itself whatever the joints' axis conventions. The two reflections
//! cancel, so every result is a proper rotation.

use glam::{Affine3A, Vec3};
use sf_anim::{LocalPose, ModelPose, RootDelta, Skeleton, Trs, local_to_model};

use crate::asset::Axis;
use crate::compiled::MirrorTable;

pub(crate) fn reflection(axis: Axis) -> Affine3A {
    Affine3A::from_scale(match axis {
        Axis::X => Vec3::new(-1.0, 1.0, 1.0),
        Axis::Y => Vec3::new(1.0, -1.0, 1.0),
        Axis::Z => Vec3::new(1.0, 1.0, -1.0),
    })
}

/// Per-joint corrections for a mirror table (compile time).
#[cfg(feature = "bake")]
pub(crate) fn corrections(skeleton: &Skeleton, pairs: &[u16], axis: Axis) -> Vec<Affine3A> {
    let rest = skeleton.rest_model();
    let s = reflection(axis);
    (0..skeleton.len()).map(|j| (s * rest[usize::from(pairs[j])]).inverse() * rest[j]).collect()
}

/// Writes the mirror of `input` into `out`.
pub(crate) fn mirror_pose(
    skeleton: &Skeleton,
    table: &MirrorTable,
    input: &LocalPose,
    out: &mut LocalPose,
    model: &mut ModelPose,
    mirrored: &mut ModelPose,
) {
    let n = skeleton.len();
    local_to_model(skeleton, input, model, &Affine3A::IDENTITY);
    let s = reflection(table.axis);
    mirrored.mats.resize(n, Affine3A::IDENTITY);
    for j in 0..n {
        mirrored.mats[j] = s * model.mats[usize::from(table.pairs[j])] * table.corrections[j];
    }
    if out.len() != n {
        *out = LocalPose::new(n);
    }
    for j in 0..n {
        let local = match skeleton.parent(j) {
            Some(p) => mirrored.mats[p].inverse() * mirrored.mats[j],
            None => mirrored.mats[j],
        };
        out.set(j, Trs::from_affine(&local));
    }
}

/// Root motion mirrored across the plane: the translation reflects, and a
/// turn about an axis in the plane reverses (one along the plane's normal
/// keeps): yaw (about Y) reverses unless the plane is horizontal, pitch
/// (about X) unless the plane is the X one, roll (about Z) unless it is Z.
pub(crate) fn mirror_root(axis: Axis, root: RootDelta) -> RootDelta {
    let s = reflection(axis);
    let flip = |keep: bool, v: f32| if keep { v } else { -v };
    RootDelta {
        translation: s.transform_vector3(root.translation),
        yaw: flip(axis == Axis::Y, root.yaw),
        tilt: glam::Vec2::new(flip(axis == Axis::X, root.tilt.x), flip(axis == Axis::Z, root.tilt.y)),
    }
}
