//! A held body's shapes for the physics world (superfargoth character-shapes.md slice S3, PH-15): Delirium's own
//! parts as the framework's [`ShapeSet`], so balls and props thrown at a body Delirium holds hit its limbs (kinematic
//! body proxies) and every contact start pushes the struck part again.
//!
//! The parts are what Delirium simulates and what the copy-back shows ([`crate::pose_to_bones`]), so the proxies are
//! where the body is drawn. Each part is a capsule along its frame's Y axis, or a box, which becomes the capsule along
//! its longest side (as [`crate::takeover::pick`] tests it). Scaffold for S1: the cooked per-rig set from
//! `RagdollDef` (skin fit, regions) replaces this per-takeover set; the poses stay the parts' while Delirium holds the
//! body.

use delirium::prelude::Transform;
use delirium::ragdoll::Build;
use glam::{Affine3A, Quat, Vec3};
use sf_anim::Skeleton;
use sf_ragdoll::{BoneShape, Capsule, ShapeSet};

use crate::PartTable;

/// A takeover's shapes: the set (one shape per part, in part order) and each part's turn from its frame to its
/// capsule's (Y along the capsule).
#[derive(Clone, Debug)]
pub struct PartShapes {
    pub set: ShapeSet,
    pub turns: Vec<Quat>,
}

/// The capsule of a part (`radius`, `half`, and the turn from the part frame to the capsule frame).
fn capsule(radius: f32, half: f32, boxed: Option<Vec3>) -> (Capsule, Quat) {
    match boxed {
        Some(e) => {
            let mut sides = [(Vec3::X, e.x), (Vec3::Y, e.y), (Vec3::Z, e.z)];
            sides.sort_by(|a, b| b.1.total_cmp(&a.1));
            (Capsule { radius: sides[1].1, half: sides[0].1 - sides[1].1 }, Quat::from_rotation_arc(Vec3::Y, sides[0].0))
        }
        None => (Capsule { radius, half }, Quat::IDENTITY),
    }
}

/// The shapes of `build`'s parts (a takeover's build: the actor's size) on `skeleton`, each following its part's bone
/// (`table`): `local` is the capsule frame in the bone's model frame at the skeleton's own scale.
#[must_use]
pub fn part_shapes(table: &PartTable, build: &Build, skeleton: &Skeleton) -> PartShapes {
    let (shapes, turns) = table
        .parts
        .iter()
        .zip(&build.parts)
        .map(|(bone, part)| {
            let (capsule, turn) = capsule(part.radius, part.half, part.boxed);
            // The part frame in the bone's frame is the inverse of the bone's in the part's.
            let (_, r, t) = bone.offset.compute_affine().inverse().to_scale_rotation_translation();
            #[allow(clippy::cast_possible_truncation)]
            let shape = BoneShape { joint: bone.joint as u16, capsule, local: (t, (r * turn).normalize()), region: 0, hurt_scale: 1.0 };
            (shape, turn)
        })
        .unzip();
    PartShapes { set: ShapeSet::new(skeleton, shapes), turns }
}

/// Each part's capsule frame in sim space: (centre, rotation).
#[must_use]
pub fn shape_frames(parts: &[Transform], turns: &[Quat]) -> Vec<(Vec3, Quat)> {
    parts.iter().zip(turns).map(|(p, t)| (p.translation, (p.rotation * *t).normalize())).collect()
}

/// The capsule frames of a shown model pose (`model`: model-space joint transforms, e.g. [`crate::pose::model_pose`]),
/// through `sim_from_model`: what the shapes' `local` gives on the drawn skeleton (S2's posing, for checks).
#[must_use]
pub fn posed_frames(set: &ShapeSet, model: &[Affine3A], sim_from_model: &Affine3A) -> Vec<(Vec3, Quat)> {
    set.shapes
        .iter()
        .map(|s| {
            let m = *sim_from_model * model[usize::from(s.joint)] * Affine3A::from_rotation_translation(s.local.1, s.local.0);
            let (_, r, t) = m.to_scale_rotation_translation();
            (t, r.normalize())
        })
        .collect()
}
