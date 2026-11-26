//! Pose mirroring across the character's mirror plane (UE mirror data
//! tables, done in model space), for the graph's `Mirror` node and the
//! cook's [`crate::ClipRules::mirror`] (a left-side take cooked as the
//! right-side one).
//!
//! Joint `j` takes its counterpart `m(j)`'s model transform reflected by
//! `S`, times a per-joint correction `C_j = (S · Rest_m(j))⁻¹ · Rest_j`
//! that maps the counterpart's frame onto `j`'s, so the rest pose mirrors
//! onto itself whatever the joints' axis conventions. The two reflections
//! cancel, so every result is a proper rotation.
//!
//! A mirrored clip ([`mirror_frame_clip`]) also carries its root track
//! reflected (as [`mirror_root`] turns a delta), its side-named curves,
//! markers and events renamed to the counterpart joint (`contact.foot_l`
//! → `contact.foot_r`), its contacts on the other side and its velocities
//! reflected.

use glam::{Affine3A, Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::{Contact, FrameClip, LocalPose, ModelPose, RigProfile, RootDelta, Side, Skeleton, Trs, local_to_model};

/// The mirror plane's normal axis in character space (X: the character's
/// left and right).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    #[default]
    X,
    Y,
    Z,
}

/// Mirror data: per joint, its counterpart and the correction that maps
/// the counterpart's reflected model transform back onto this joint's
/// frame (so the rest pose mirrors onto itself).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MirrorTable {
    pub axis: Axis,
    pub pairs: Vec<u16>,
    pub corrections: Vec<Affine3A>,
}

impl MirrorTable {
    /// From every joint's counterpart (itself when unpaired).
    #[must_use]
    pub fn new(skeleton: &Skeleton, pairs: Vec<u16>, axis: Axis) -> Self {
        let corrections = corrections(skeleton, &pairs, axis);
        Self { axis, pairs, corrections }
    }

    /// From joint pairs (a rig profile's [`RigProfile::mirror`]); unpaired
    /// joints mirror onto themselves.
    #[must_use]
    pub fn from_pairs(skeleton: &Skeleton, pairs: &[(u16, u16)], axis: Axis) -> Self {
        let mut all: Vec<u16> = (0..skeleton.len()).map(|j| u16::try_from(j).expect("joint index fits u16")).collect();
        for &(a, b) in pairs {
            all[usize::from(a)] = b;
            all[usize::from(b)] = a;
        }
        Self::new(skeleton, all, axis)
    }

    /// The counterpart of `joint`.
    #[must_use]
    pub fn pair(&self, joint: usize) -> usize {
        usize::from(self.pairs[joint])
    }
}

/// The reflection across the plane normal to `axis`.
#[must_use]
pub fn reflection(axis: Axis) -> Affine3A {
    Affine3A::from_scale(match axis {
        Axis::X => Vec3::new(-1.0, 1.0, 1.0),
        Axis::Y => Vec3::new(1.0, -1.0, 1.0),
        Axis::Z => Vec3::new(1.0, 1.0, -1.0),
    })
}

/// Per-joint corrections for a mirror table, from every joint's
/// counterpart `pairs` (module docs).
#[must_use]
pub fn corrections(skeleton: &Skeleton, pairs: &[u16], axis: Axis) -> Vec<Affine3A> {
    let rest = skeleton.rest_model();
    let s = reflection(axis);
    (0..skeleton.len()).map(|j| (s * rest[usize::from(pairs[j])]).inverse() * rest[j]).collect()
}

/// Writes the mirror of `input` into `out` (`model` and `mirrored` are
/// scratch).
pub fn mirror_pose(skeleton: &Skeleton, table: &MirrorTable, input: &LocalPose, out: &mut LocalPose, model: &mut ModelPose, mirrored: &mut ModelPose) {
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
/// The same rule mirrors an absolute root track (a pose relative to frame
/// 0, [`crate::RootMotionTrack`]).
#[must_use]
pub fn mirror_root(axis: Axis, root: RootDelta) -> RootDelta {
    let s = reflection(axis);
    let flip = |keep: bool, v: f32| if keep { v } else { -v };
    RootDelta {
        translation: s.transform_vector3(root.translation),
        yaw: flip(axis == Axis::Y, root.yaw),
        tilt: Vec2::new(flip(axis == Axis::X, root.tilt.x), flip(axis == Axis::Z, root.tilt.y)),
    }
}

/// A limb side across the plane: left and right swap across the X plane
/// (`+X` is left), nothing else changes. Callers carrying a source clip's
/// contacts onto a mirrored target clip ([`crate::transfer_contacts`])
/// swap their sides with this first.
#[must_use]
pub fn mirror_side(axis: Axis, side: Side) -> Side {
    match (axis, side) {
        (Axis::X, Side::Left) => Side::Right,
        (Axis::X, Side::Right) => Side::Left,
        (_, s) => s,
    }
}

/// [`crate::ClipRules::mirror`]: cook the clip mirrored across the plane
/// normal to `axis` (the default X plays a left-side take as the right-side
/// one), on the target rig with its profile's mirror pairs
/// ([`crate::Retargeter::clip_post`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MirrorRules {
    pub axis: Axis,
}

/// Why [`crate::ClipRules::mirror`] was not applied (the clip is then
/// cooked unmirrored).
#[derive(Clone, Copy, Debug, thiserror::Error, PartialEq, Eq)]
pub enum MirrorError {
    #[error("mirroring needs the target rig profile (its mirror pairs)")]
    NoProfile,
    #[error("additive clips are not mirrored")]
    Additive,
    #[error("frame data size mismatch")]
    BadFrames,
}

/// Mirrors `clip` (frames of `skeleton`) in place with `profile`'s mirror
/// pairs: every frame's local pose ([`mirror_pose`]), the root track
/// ([`mirror_root`] on each frame's absolute pose), curve, marker and event
/// names naming a paired joint (by `.`-separated segment), the contacts
/// and the root velocities. Applying it twice gives the clip back.
pub fn mirror_frame_clip(skeleton: &Skeleton, profile: &RigProfile, rules: &MirrorRules, clip: &mut FrameClip) -> Result<(), MirrorError> {
    let j = skeleton.len();
    if clip.meta.additive {
        return Err(MirrorError::Additive);
    }
    if clip.frames.len() != clip.frame_count * j {
        return Err(MirrorError::BadFrames);
    }
    let table = MirrorTable::from_pairs(skeleton, &profile.mirror, rules.axis);
    let (mut input, mut out) = (LocalPose::new(j), LocalPose::new(j));
    let (mut model, mut mirrored) = (ModelPose::new(j), ModelPose::new(j));
    for frame in clip.frames.chunks_exact_mut(j) {
        for (k, t) in frame.iter().enumerate() {
            input.set(k, *t);
        }
        mirror_pose(skeleton, &table, &input, &mut out, &mut model, &mut mirrored);
        for (k, t) in frame.iter_mut().enumerate() {
            *t = out.get(k);
        }
    }
    let axis = rules.axis;
    if let Some(track) = &mut clip.root_motion {
        for (f, frame) in track.frames.iter_mut().enumerate() {
            let tilt = track.attitude.get(f).map_or(Vec2::ZERO, |a| Vec2::from(*a));
            let m = mirror_root(axis, RootDelta { translation: Vec3::new(frame[0], frame[1], frame[2]), yaw: frame[3], tilt });
            *frame = [m.translation.x, m.translation.y, m.translation.z, m.yaw];
            if let Some(a) = track.attitude.get_mut(f) {
                *a = m.tilt.into();
            }
        }
    }
    let rename = |name: &mut String| {
        if name.split('.').any(|seg| skeleton.find(seg).is_some_and(|k| table.pair(k) != k)) {
            let parts: Vec<&str> = name.split('.').map(|seg| skeleton.find(seg).map_or(seg, |k| skeleton.name(table.pair(k)))).collect();
            *name = parts.join(".");
        }
    };
    clip.curves.iter_mut().for_each(|c| rename(&mut c.name));
    clip.markers.iter_mut().for_each(|m| rename(&mut m.name));
    clip.events.iter_mut().for_each(|e| rename(&mut e.name));
    let s = reflection(axis);
    let meta = &mut clip.meta;
    meta.average_velocity = s.transform_vector3(meta.average_velocity);
    meta.stance_velocity = s.transform_vector3(meta.stance_velocity);
    meta.turn_rate = mirror_root(axis, RootDelta::new(Vec3::ZERO, meta.turn_rate)).yaw;
    for c in &mut meta.contacts {
        mirror_contact(skeleton, profile, &table, c);
    }
    Ok(())
}

/// One contact across the plane: its side, effector and limb swap to the
/// counterparts, its positions reflect.
fn mirror_contact(skeleton: &Skeleton, profile: &RigProfile, table: &MirrorTable, c: &mut Contact) {
    let s = reflection(table.axis);
    c.side = mirror_side(table.axis, c.side);
    c.position = s.transform_point3(c.position);
    c.local = s.transform_point3(c.local);
    let Some(joint) = skeleton.find(&c.joint) else {
        return;
    };
    let pair = table.pair(joint);
    skeleton.name(pair).clone_into(&mut c.joint);
    if let Some(limb) = profile.limbs.iter().find(|l| usize::from(l.effector) == pair) {
        c.limb.clone_from(&limb.name);
    }
}
