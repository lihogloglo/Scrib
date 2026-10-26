//! Delirium (Euphoria-style active ragdoll, github.com/PVCPC/delirium-physics, GPL-3.0-only) characters built from
//! Morrowind's humanoid skeleton. Bevy-free; ROADMAP 3.2b slice 1.
//!
//! - [`character_from_skeleton`]: the Delirium character (asset + build, `nm-character-1`) from the `Bip01` bones.
//! - [`pose_to_bones`] / [`bones_to_parts`]: Delirium part frames to the bones each part owns and back, through the
//!   [`PartTable`] the character comes with.
//! - [`sim`]: a headless harness (floor world, one agent, pushes), used by the tests.
//! - [`takeover`]: an animated character taken over from its pose, and when to hand it back (slice 2a);
//!   [`takeover::pick`] finds the part under a ray.
//!
//! Spaces. *Model*: the skeleton's model space (`sf_anim`: Y up, facing +Z, metres). *Authoring*: Z up, facing +Y,
//! metres (Morrowind's own NIF axes), where the character is built ([`authoring_point`]). *Sim*: Delirium's world (Z
//! up); the harness puts the authoring pose there with a translation only ([`sim::Spawned::sim_from_authoring`]).
//!
//! The asset is process-global in Delirium (`nmrs::set`, first call wins), so one character per process.
//!
//! Licence: GPL-3.0-only (links delirium-physics; `body` ports skyrim-delirium code, see its module docs).

pub mod body;
pub mod pose;
pub mod sim;
pub mod takeover;

pub use body::{Body, Options, character_from_skeleton};
pub use delirium;
pub use delirium_physics as sdk;
pub use pose::{PartBone, PartTable, bones_to_parts, pose_to_bones};
pub use takeover::{HandBack, Rules, Takeover};

use glam::{Affine3A, Mat3, Quat, Vec3};

/// Authoring axes from model axes: (x, y, z) -> (-x, z, y). A proper rotation, its own inverse.
const MODEL_TO_AUTHORING: Mat3 = Mat3::from_cols(Vec3::NEG_X, Vec3::Z, Vec3::Y);

/// A model-space point (or direction) in authoring space.
pub fn authoring_point(v: Vec3) -> Vec3 {
    MODEL_TO_AUTHORING * v
}

/// A model-space rotation in authoring space.
pub fn authoring_rotation(q: Quat) -> Quat {
    let b = Quat::from_mat3(&MODEL_TO_AUTHORING);
    (b * q * b.inverse()).normalize()
}

/// The authoring-from-model transform (equal to its inverse, model-from-authoring).
pub fn authoring_from_model() -> Affine3A {
    Affine3A::from_mat3(MODEL_TO_AUTHORING)
}
