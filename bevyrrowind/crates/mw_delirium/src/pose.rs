//! Delirium parts on skeleton bones: the table and the two copies (parts to bones, bones to parts).
//!
//! Each of the 21 parts owns one bone; the bone's frame is rigid in the part's frame (`offset`, measured at the bind
//! pose). Two parts share the chest bone (`Bip01 Spine2`): Spine3 writes it, Spine2 only reads it.
//! [`pose_to_bones`] writes the owned bones' rotations (and the pelvis' position) into a local pose and leaves the
//! other bones (fingers, toes, sockets, the root) with their incoming, animated values; children keep their incoming
//! local translations, so bone lengths never stretch.

use delirium::prelude::Transform;
use glam::{Affine3A, Quat, Vec3};
use sf_anim::{Skeleton, Trs};

/// One part's bone.
#[derive(Clone, Copy, Debug)]
pub struct PartBone {
    /// Skeleton joint index.
    pub joint: usize,
    /// Whether the part writes the bone ([`pose_to_bones`]); every part reads it ([`bones_to_parts`]).
    pub write: bool,
    /// The bone's frame in the part's frame.
    pub offset: Transform,
}

/// The 21 parts' bones, in Delirium's part order.
#[derive(Clone, Debug)]
pub struct PartTable {
    pub parts: Vec<PartBone>,
}

impl PartTable {
    /// The writing part of each skeleton joint.
    pub fn writers(&self, joints: usize) -> Vec<Option<usize>> {
        let mut out = vec![None; joints];
        for (k, p) in self.parts.iter().enumerate() {
            if p.write {
                out[p.joint] = Some(k);
            }
        }
        out
    }
}

fn rigid(m: &Affine3A) -> (Quat, Vec3) {
    let (_, r, t) = m.to_scale_rotation_translation();
    (r.normalize(), t)
}

/// Model-space transforms of a local pose.
pub fn model_pose(skeleton: &Skeleton, local: &[Trs]) -> Vec<Affine3A> {
    let mut out: Vec<Affine3A> = Vec::with_capacity(local.len());
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        out.push(skeleton.parent(j).map_or(m, |p| out[p] * m));
    }
    out
}

/// Writes Delirium part frames (`parts`, sim space) into the bones they own in `local` (the incoming local pose,
/// e.g. the clip's), through `model_from_sim` (sim space to the skeleton's model space; may carry the actor's
/// uniform scale). Owned bones take the part's rotation; the pelvis bone also its position. Other bones keep their
/// local transforms.
pub fn pose_to_bones(table: &PartTable, skeleton: &Skeleton, parts: &[Transform], model_from_sim: &Affine3A, local: &mut [Trs]) {
    let writers = table.writers(local.len());
    let mut model: Vec<Affine3A> = Vec::with_capacity(local.len());
    for j in 0..local.len() {
        let parent = skeleton.parent(j).map_or(Affine3A::IDENTITY, |p| model[p]);
        if let Some(k) = writers[j] {
            let target = *model_from_sim * parts[k].compute_affine() * table.parts[k].offset.compute_affine();
            let (r, t) = rigid(&target);
            let (pr, _) = rigid(&parent);
            local[j].rotation = (pr.inverse() * r).normalize();
            if k == 0 {
                local[j].translation = parent.inverse().transform_point3(t);
            }
        }
        model.push(parent * local[j].to_affine());
    }
}

/// Delirium part frames (sim space) from a local pose, through `sim_from_model` (the inverse of
/// [`pose_to_bones`]'s `model_from_sim`): the start pose of a takeover.
pub fn bones_to_parts(table: &PartTable, skeleton: &Skeleton, local: &[Trs], sim_from_model: &Affine3A) -> Vec<Transform> {
    let model = model_pose(skeleton, local);
    table
        .parts
        .iter()
        .map(|p| {
            let m = *sim_from_model * model[p.joint] * p.offset.compute_affine().inverse();
            let (r, t) = rigid(&m);
            Transform::from_translation(t).with_rotation(r)
        })
        .collect()
}
