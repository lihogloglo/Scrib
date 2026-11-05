//! Character shape sets (character-shapes.md "Data model"): one capsule per
//! significant bone, the source of hurt volumes and of the kinematic body
//! proxies a physics layer keeps in the solver while it animates or holds a
//! character.
//!
//! As built (S3): the type and its rest-pose bound. The set is made by
//! whoever has shapes for the rig (S3: the game's active-ragdoll layer, from
//! its own parts); S1 adds the cooked set from [`crate::RagdollDef`] with the
//! skin fit and the joint-to-shape map, S2 the posing (`pose_shapes`).

use glam::{Quat, Vec3};
use sf_anim::{Skeleton, SkeletonId};

use crate::Capsule;

/// One bone's shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneShape {
    /// The skeleton joint it follows.
    pub joint: u16,
    /// Along the shape frame's Y axis.
    pub capsule: Capsule,
    /// The shape frame relative to the joint's model frame (translation, rotation).
    pub local: (Vec3, Quat),
    /// Region index (`RagdollDef::regions`); 0 when the source has none.
    pub region: u8,
    /// Hurt radius factor (hit and hurt are tuned as a pair), 1 by default.
    pub hurt_scale: f32,
}

/// A rig's shapes.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeSet {
    pub skeleton: SkeletonId,
    /// Parents before children; 0 is the pelvis.
    pub shapes: Vec<BoneShape>,
    /// Bounding upright capsule of the rest pose (radius, half height of its
    /// cylinder part), standing on the model origin: the broadphase bound.
    pub bound: (f32, f32),
}

impl ShapeSet {
    /// A set on `skeleton` with its rest-pose bound. Shapes on joints the
    /// skeleton doesn't have are left out of the bound.
    #[must_use]
    pub fn new(skeleton: &Skeleton, shapes: Vec<BoneShape>) -> Self {
        let rest = skeleton.rest_model();
        let (mut reach, mut top) = (0.0_f32, 0.0_f32);
        for s in &shapes {
            let Some(joint) = rest.get(usize::from(s.joint)) else { continue };
            let center = joint.transform_point3(s.local.0);
            let axis = joint.transform_vector3(s.local.1 * Vec3::Y).normalize_or_zero() * s.capsule.half;
            for end in [center - axis, center + axis] {
                reach = reach.max(Vec3::new(end.x, 0.0, end.z).length() + s.capsule.radius);
                top = top.max(end.y + s.capsule.radius);
            }
        }
        Self { skeleton: skeleton.id(), shapes, bound: (reach, (0.5 * top - reach).max(0.0)) }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// The joint shape `shape` follows.
    #[must_use]
    pub fn joint(&self, shape: u16) -> Option<u16> {
        self.shapes.get(usize::from(shape)).map(|s| s.joint)
    }
}
