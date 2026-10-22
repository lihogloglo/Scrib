//! Skeletal animation core, Bevy-free: skeletons, cooked clips, sampling,
//! pose ops, root motion, events and curves, IK solvers, retarget math,
//! and the `skeleton.cook` / `anim.cook` bake steps.
//!
//! The runtime follows ozz-animation's job model: clips are sampled into
//! contiguous local-space pose buffers ([`LocalPose`], four joints per SIMD
//! lane), whole buffers are blended, masked and layered, one forward loop
//! takes them to model space ([`local_to_model`]), and IK edits the
//! model-space pose before it is written to the engine once.
//!
//! Spec: `docs/specs/animation.md` (retargeting: `docs/specs/character.md`).

// Pose and IK math reads best with its textbook names (a, b, c, q, t, u, w).
#![allow(clippy::many_single_char_names)]

#[cfg(feature = "bvh")]
mod bvh_import;
mod clip;
pub mod cloth;
mod cook;
#[cfg(feature = "fbx")]
mod fbx_import;
pub mod foot_metrics;
#[cfg(any(test, feature = "fixtures"))]
#[doc(hidden)]
pub mod fixtures;
#[cfg(feature = "gltf")]
mod gltf_import;
pub mod ik;
mod import;
mod math;
mod pose;
mod retarget;
pub mod rig;
mod skeleton;
mod source;
pub mod spring;
pub mod steps;

#[cfg(feature = "bvh")]
pub use bvh_import::BvhAnimImporter;
pub use clip::{
    Advance, CLIP_FORMAT, Clip, ClipMeta, CurveTrack, Event, EventEdge, FiredEvent, RootDelta, RootMotionMode, RootMotionTrack, SyncMarker,
    attitude_rotation,
};
pub use cook::{
    AdditiveBase, ClipRules, ContactRules, CookError, CookSettings, FrameClip, Looping, RootMotionRules, cook_clip, cook_clip_for, cook_frames,
};
#[cfg(feature = "fbx")]
pub use fbx_import::FbxAnimImporter;
#[cfg(feature = "gltf")]
pub use gltf_import::{GltfAnimImporter, list_gltf_clips};
pub use ik::LimbSolver;
pub use import::{Axis, SourceSpace, SpaceConverter, detect_root_motion_joint};
pub use math::{Trs, angle_between, nlerp, twist_angle};
pub use pose::{
    BlendLayer, JointWeights, LocalPose, ModelPose, SoaTrs, apply_additive, blend, blend_in_place, blend_layers, local_to_model, make_additive,
    update_subtree,
};
pub use retarget::{RetargetChain, RetargetError, RetargetPair, RetargetRig, Retargeter, retarget_clip};
pub use rig::{
    Chain, FootSpec, Girdle, LegKind, Limb, LimbDesc, LimbHint, LimbKind, Measures, NeckHead, PoleHint, ProfileParts, RIG_PROFILE_FORMAT, RigError,
    RigHints, RigProfile, RigTemplate, Side, detect_profile,
};
pub use skeleton::{
    JointMask, LodRules, NO_PARENT, SKELETON_FORMAT, Skeleton, SkeletonError, SkeletonId, SkeletonRules, SourceJoint, guess_tag, tag,
};
pub use source::{AnimSource, BoundSource, Curve, CurveValue, Interpolation, SourceClip, SourceCurve, SourceTrack};

/// A cooked file that can't be read.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FormatError {
    #[error("not a cooked file of this kind (bad magic)")]
    Magic,
    #[error("cooked format version {found}, expected {expected}: re-bake")]
    Version { found: u32, expected: u32 },
    #[error("corrupt cooked data: {0}")]
    Decode(String),
}

fn check_header(bytes: &[u8], magic: [u8; 4], version: u32) -> Result<&[u8], FormatError> {
    if bytes.len() < 8 || bytes[..4] != magic {
        return Err(FormatError::Magic);
    }
    let found = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
    if found != version {
        return Err(FormatError::Version { found, expected: version });
    }
    Ok(&bytes[8..])
}

/// A source file an importer couldn't read.
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("{path}: {message}")]
pub struct ImportError {
    pub path: String,
    pub message: String,
}

/// Reads skeletons and clips from a file format. The framework ships glTF
/// (feature `gltf`), BVH (`bvh`) and FBX (`fbx`), all bake only;
/// adapters add legacy formats.
pub trait AnimImporter: Send + Sync {
    fn extensions(&self) -> &[&str];
    /// Bump when the importer's output changes (part of the bake key).
    fn version(&self) -> u32;
    /// Bytes of the settings `path` is imported with (units, axes), part
    /// of the bake key next to [`AnimImporter::version`]. Empty by default.
    fn settings_key(&self, _path: &str) -> Vec<u8> {
        Vec::new()
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError>;
    /// A sibling file read with `path` when it exists: curves that share the
    /// animation's timeline but live in another file (morph weights keyed
    /// in the model while the bones are keyed in the animation file). The
    /// bake records it as an input. `None` by default.
    fn companion(&self, _path: &str) -> Option<String> {
        None
    }
    /// [`AnimImporter::import`] with the [`AnimImporter::companion`]'s bytes
    /// when that file exists. Defaults to `import`.
    fn import_with(&self, path: &str, bytes: &[u8], companion: Option<&[u8]>) -> Result<AnimSource, ImportError> {
        let _ = companion;
        self.import(path, bytes)
    }
}

#[cfg(test)]
mod fbik_tests;
#[cfg(test)]
mod import_tests;
#[cfg(test)]
mod rig_tests;
#[cfg(test)]
mod tests;
