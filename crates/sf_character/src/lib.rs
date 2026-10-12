//! Character core, Bevy-free: the standard humanoid rig (VRM 1.0 bone set),
//! rig maps with auto-map and auto-align, the body slot registry, the
//! `sf.body_part` / `sf.wearable` record kinds, slot resolution with the
//! outfit key, character recipes and the `CharacterSource` adapter trait.
//!
//! Retargeting math lives in `sf_anim::retarget`; a [`RigMap`] becomes its
//! `RetargetRig` through [`RigMap::retarget_rig`]. Outfit assembly (part
//! merging, morph-bearing parts, shape baking) is [`assemble`]; spawning
//! and views are `sf_character_bevy`'s.
//!
//! Spec: `docs/specs/character.md`.

mod align;
mod assemble;
mod automap;
mod humanoid;
mod names;
mod outfit;
mod records;
mod rig;
mod slots;
#[cfg(feature = "smooth")]
mod smooth;
#[cfg(feature = "smooth")]
mod smooth_outfit;
mod source;

pub use align::{AlignReport, auto_align};
pub use assemble::{
    AssembledMesh, AssembledOutfit, AssemblyPart, BEVY_JOINT_BYTES, BEVY_MORPH_BYTES, DecodedPart, OutfitBytes, ShapeWeights, assemble,
};
pub use automap::{AutoMapReport, auto_map};
pub use humanoid::{BoneSet, Finger, HumanoidBone, Limb, Side};
pub use outfit::{Outfit, OutfitKey, PartLookup, View, outfit_key, resolve, resolve_recipe};
pub use records::{
    AnimGraphRef, BodyPart, CharacterRecipe, FirstPersonRecipe, Hand, HeldItem, HeldLight, Layer, Mount, PartKind, PartRef, Readiness, RigMapRef,
    RigProfileRef, RigRef, Socket, TintParams, Wearable, WearableRef, register_kinds,
};
pub use rig::{Chain, JointName, RetargetOptions, RigMap, RigMapError};
pub use slots::{SlotDef, SlotId, SlotRegistry, slot};
#[cfg(feature = "smooth")]
pub use smooth::{
    GuardReport, SeamLink, SkinSurface, SmoothReport, SmoothSettings, TransferReport, TransferSettings, bind_palette, blend_seam_normals, find_seams,
    guard_seams, penetration, seam_openings, skinning_matrices, smooth_body, transfer_weights,
};
#[cfg(feature = "smooth")]
pub use smooth_outfit::{OutfitSmoothReport, OutfitSmoothing, SmoothRig, smooth_outfit};
pub use source::{CharacterError, CharacterSource, CharacterState};

#[cfg(test)]
mod tests;
