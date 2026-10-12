//! Shared helpers for animation importers: the source file's coordinate
//! space ([`SourceSpace`]: units, up and forward axes, handedness) and
//! root-motion joint detection ([`detect_root_motion_joint`]).
//!
//! The framework's space is glTF's: meters, +Y up, right-handed, a
//! character facing +Z with its left at +X (character.md, locomotion.md).

use glam::{Mat3, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::{SourceClip, SourceJoint, Trs};

/// A signed coordinate axis of a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Axis {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Axis {
    pub fn vector(self) -> Vec3 {
        match self {
            Self::PosX => Vec3::X,
            Self::NegX => Vec3::NEG_X,
            Self::PosY => Vec3::Y,
            Self::NegY => Vec3::NEG_Y,
            Self::PosZ => Vec3::Z,
            Self::NegZ => Vec3::NEG_Z,
        }
    }
}

/// Where a source file's data lives: its length unit (in meters), which of
/// its axes is up and which one the character faces, and its handedness.
/// [`SourceSpace::default`] is the framework's own space (identity).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceSpace {
    /// Meters per source unit (0.01 for centimeters, 0.0254 for inches).
    pub unit_scale: f32,
    pub up: Axis,
    /// The direction the character faces in the file's rest pose.
    pub forward: Axis,
    pub right_handed: bool,
}

impl Default for SourceSpace {
    fn default() -> Self {
        Self { unit_scale: 1.0, up: Axis::PosY, forward: Axis::PosZ, right_handed: true }
    }
}

impl SourceSpace {
    /// Y up, facing +Z, right-handed, in centimeters: the usual mocap
    /// layout of BVH datasets.
    pub fn centimeters() -> Self {
        Self { unit_scale: 0.01, ..Self::default() }
    }

    /// The axis-change matrix `C` (source → framework). Its rows are the
    /// source's left, up and forward axes; `det C = −1` for a left-handed
    /// source. `None` if `up` and `forward` are parallel.
    pub fn basis(&self) -> Option<Mat3> {
        let (up, forward) = (self.up.vector(), self.forward.vector());
        if up.dot(forward).abs() > 0.5 {
            return None;
        }
        let left = if self.right_handed { up.cross(forward) } else { forward.cross(up) };
        Some(Mat3::from_cols(left, up, forward).transpose())
    }

    /// The converter for this space, or `None` for parallel axes.
    pub fn converter(&self) -> Option<SpaceConverter> {
        let c = self.basis()?;
        // A rotation R becomes C·R·Cᵀ. For an improper C (a mirror) that
        // equals P·R·Pᵀ with the proper P = −C, so one quaternion does it.
        let proper = if c.determinant() > 0.0 { c } else { -c };
        Some(SpaceConverter { basis: c, rotation: Quat::from_mat3(&proper).normalize(), unit_scale: self.unit_scale })
    }
}

/// Converts transforms from a [`SourceSpace`] into the framework's space.
/// Every local transform is conjugated by the axis change, so a joint's
/// model-space position `p` becomes `s·C·p` and its orientation `C·R·Cᵀ`.
#[derive(Clone, Copy, Debug)]
pub struct SpaceConverter {
    basis: Mat3,
    rotation: Quat,
    unit_scale: f32,
}

impl SpaceConverter {
    pub fn translation(&self, t: Vec3) -> Vec3 {
        self.basis * t * self.unit_scale
    }

    pub fn rotation(&self, r: Quat) -> Quat {
        (self.rotation * r * self.rotation.conjugate()).normalize()
    }

    /// Axis-aligned scale: the components follow the axes they moved to.
    pub fn scale(&self, s: Vec3) -> Vec3 {
        (self.basis * s).abs()
    }

    pub fn trs(&self, t: Trs) -> Trs {
        Trs::new(self.translation(t.translation), self.rotation(t.rotation), self.scale(t.scale))
    }
}

/// The joint whose motion is the character's root motion, as importers
/// detect it (stored in [`crate::AnimSource::root_motion`]; `skeleton.cook`
/// uses it when [`crate::SkeletonRules::detect_root_motion`] is set and the
/// rules name no joint):
///
/// 1. a joint named `root` (any case, after a `ns:` or `a|b` prefix) whose
///    translation moves in some clip: the UE / game-rig convention;
/// 2. else the shallowest joint whose translation moves (a BVH `ROOT`,
///    Mixamo's `Hips`, or an exporter's armature node carrying the travel):
///    `anim.cook` projects it to the ground;
/// 3. else a `root`-named joint, else none.
///
/// "Moves" means a translation key more than 1 mm from the first one.
pub fn detect_root_motion_joint(joints: &[SourceJoint], clips: &[SourceClip]) -> Option<usize> {
    let moving = |name: &str| {
        clips.iter().flat_map(|c| &c.tracks).filter(|t| t.joint == name).filter_map(|t| t.translation.as_ref()).any(|c| {
            let per_key = if c.interpolation == crate::Interpolation::CubicSpline { 3 } else { 1 };
            let first = c.values.get(per_key / 2).copied().unwrap_or(Vec3::ZERO);
            c.values.iter().skip(per_key / 2).step_by(per_key).any(|v| v.distance(first) > 1e-3)
        })
    };
    let is_root_name = |name: &str| {
        let base = name.rsplit([':', '|']).next().unwrap_or(name).to_ascii_lowercase();
        matches!(base.as_str(), "root" | "root_motion" | "rootmotion")
    };
    let depth = |mut j: usize| {
        let mut d = 0usize;
        while let Some(p) = joints[j].parent {
            if d > joints.len() {
                break;
            }
            d += 1;
            j = p;
        }
        d
    };
    let has_children = |j: usize| joints.iter().any(|c| c.parent == Some(j));
    let named = (0..joints.len()).find(|&j| is_root_name(&joints[j].name) && has_children(j));
    if let Some(r) = named.filter(|&r| moving(&joints[r].name)) {
        return Some(r);
    }
    (0..joints.len()).filter(|&j| moving(&joints[j].name)).min_by_key(|&j| (depth(j), j)).or(named)
}
