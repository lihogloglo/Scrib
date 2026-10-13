//! Ragdolls (physics.md "`sf_ragdoll`", "Ragdolls and the animation blend").
//!
//! - [`cook`] (`ragdoll.cook`): a [`RagdollDef`] from a skeleton and its
//!   [`RigProfile`](sf_anim::RigProfile): one capsule body per girdle and
//!   spine segment (at most 3), neck and head, and limb joint (fingers and
//!   toes merged into hands and feet), tails and long extra chains up to
//!   4 bodies; ball joints with swing/twist limits at girdles, spine, neck,
//!   wrists and ankles, hinges at knees, elbows and hocks; masses from the
//!   capsule volumes, balanced between neighbors and scaled to the
//!   adapter's total mass.
//! - [`RagdollInstance`]: a limp ragdoll in an `sf_physics` world, spawned
//!   at an animated pose with its velocities, and read back as a local pose
//!   for the animation's physics input.
//!
//! - [`powered`](StrengthProfile) (P8): stable-PD joint drives toward an
//!   animated pose (rapier's implicit acceleration-based motors) with
//!   per-region strength, a world-space pelvis drive, hit impulses with a
//!   strength dip and recovery, a balance check, and get-up root placement.
//!
//! Frames: the skeleton's model space (Y up, +Z forward, +X left).

mod cook;
mod instance;
mod powered;
#[cfg(feature = "bake")]
pub mod steps;

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_anim::SkeletonId;

pub use cook::{BodyOverride, CookParams, RagdollError, RagdollPolicy, cook};
pub use instance::{RagdollInstance, RagdollSettings, SpawnPose};
pub use powered::{Balance, Dip, Drive, DriveParams, DriveTarget, HitImpulse, StrengthProfile, balance, get_up_root, region_gain};

/// A capsule along the body frame's Y axis (a sphere when `half` is 0).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capsule {
    pub radius: f32,
    pub half: f32,
}

/// One rigid body of a ragdoll, attached to a skeleton joint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RagBody {
    pub joint: u16,
    pub shape: Capsule,
    /// The body frame relative to its joint's model frame (translation, rotation).
    pub local: (Vec3, Quat),
    /// kg, at the skeleton's own scale.
    pub mass: f32,
    /// Index into [`RagdollDef::regions`].
    pub region: u8,
    /// Parent body (`None` for the root body).
    pub parent: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RagJointKind {
    /// Swing half-angles about the frame's Y and Z axes, twist range about X (the bone), radians.
    Ball { swing: [f32; 2], twist: [f32; 2] },
    /// Rotation about the frame's X axis within `range`, radians (positive bends the limb).
    Hinge { range: [f32; 2] },
    Fixed,
}

impl RagJointKind {
    pub fn physics(self) -> sf_physics::JointKind {
        match self {
            Self::Ball { swing, twist } => sf_physics::JointKind::Ball { swing, twist },
            Self::Hinge { range } => sf_physics::JointKind::Hinge { range },
            Self::Fixed => sf_physics::JointKind::Fixed,
        }
    }
}

/// A joint between a body and its parent body; frames are body-local.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RagJoint {
    pub parent: u16,
    pub child: u16,
    pub frame_parent: (Vec3, Quat),
    pub frame_child: (Vec3, Quat),
    pub kind: RagJointKind,
}

/// A skeleton's ragdoll (the `ragdoll.cook` product).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RagdollDef {
    pub skeleton: SkeletonId,
    pub policy: RagdollPolicy,
    /// Parents before children; body 0 is the root (pelvis).
    pub bodies: Vec<RagBody>,
    /// One per non-root body.
    pub joints: Vec<RagJoint>,
    /// "core", "head", then limb and chain names.
    pub regions: Vec<String>,
    pub total_mass: f32,
    /// Body pairs that overlap in the rest pose and never collide (besides jointed neighbors).
    pub no_collide: Vec<(u16, u16)>,
    /// Per skeleton joint, the body that moves it: its own, or its nearest
    /// ancestor's; `None` above the root body.
    pub owner: Vec<Option<u16>>,
}

const MAGIC: &[u8; 4] = b"SFRD";
/// The bake step id (`steps::RagdollCookStep`, feature `bake`).
pub const RAGDOLL_COOK: &str = "ragdoll.cook";
/// Version of the cooked ragdoll format.
pub const RAGDOLL_FORMAT: u32 = 1;

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum FormatError {
    #[error("not a cooked ragdoll")]
    Magic,
    #[error("ragdoll format {0}, expected {RAGDOLL_FORMAT}")]
    Version(u32),
    #[error("ragdoll decode: {0}")]
    Decode(String),
}

impl RagdollDef {
    /// The cooked form: magic, format version, postcard.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend(RAGDOLL_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() < 8 || &bytes[..4] != MAGIC {
            return Err(FormatError::Magic);
        }
        let version = u32::from_le_bytes(bytes[4..8].try_into().expect("4 bytes"));
        if version != RAGDOLL_FORMAT {
            return Err(FormatError::Version(version));
        }
        postcard::from_bytes(&bytes[8..]).map_err(|e| FormatError::Decode(e.to_string()))
    }

    /// The region index of a name.
    pub fn region(&self, name: &str) -> Option<u8> {
        self.regions.iter().position(|r| r == name).and_then(|i| u8::try_from(i).ok())
    }

    /// The body attached to a skeleton joint, if that joint has one.
    pub fn body_of_joint(&self, joint: u16) -> Option<usize> {
        self.bodies.iter().position(|b| b.joint == joint)
    }
}

#[cfg(test)]
mod tests;
