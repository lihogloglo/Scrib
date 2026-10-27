//! Locomotion core, Bevy-free: intent, spring-driven velocity and facing,
//! trajectory prediction, batched ground and context probes (traversal
//! affordances, warp targets) over terrain and a static
//! query world, the kinematic mover, the path follower, and the
//! `collision.cook` bake step.
//!
//! The pipeline per frame (locomotion.md):
//! - Something writes a [`LocomotionIntent`]: player input, AI, scripts, or
//!   the [`PathFollower`] that drives NPCs until `sf_ai` exists.
//! - [`KinematicMover::step_batch`] springs every character's velocity and
//!   facing toward its intent (Holden's spring character, exact closed
//!   forms), blends in last update's `Drive` root motion, sweeps the capsule
//!   against statics, follows the ground (step up/down, slope limit), and
//!   predicts the [`Trajectory`] with the same springs. Probes go through
//!   one batched [`GroundProbe`] call per stage.
//! - [`Avoidance::solve`] (ORCA) turns the intents' desired velocities
//!   into collision-free ones between characters before the mover runs.
//! - The animation graph reads [`LocomotionParams`], the trajectory, the
//!   character's motion since its last update ([`Locomotion::moved`]) and
//!   per-leg ground samples ([`CharacterFrame`] converts probe hits into
//!   character space). Root motion flows back into the next mover step.
//!
//! Conventions: Y up, meters, yaw in radians about +Y with yaw 0 facing +Z
//! (the character space of `sf_anim`). Positions are [`sf_coords::WorldPos`].
//!
//! Spec: `docs/specs/locomotion.md`.

// Spring and geometry math reads best with its textbook names (a, d, e, n, v, x, y).
#![allow(clippy::many_single_char_names)]

mod avoid;
mod context;
mod correction;
mod frame;
mod intent;
mod motion;
mod mover;
mod path;
mod probe;
mod statics;
#[cfg(feature = "bake")]
pub mod steps;
#[cfg(feature = "terrain")]
mod terrain;
mod trajectory;

pub use avoid::{AvoidAgent, AvoidParams, AvoidRole, Avoidance};
pub use context::{Affordance, ContextHit, ContextParams, ContextProbes, ContextQuery, ContextRequest, Side};
pub use correction::{CorrectionParams, RenderCorrection};
pub use frame::CharacterFrame;
pub use intent::{Gait, GaitSpeeds, LocomotionIntent};
pub use motion::{MotionState, SpringParams, TurnRates, forward, spring_character, spring_yaw, turn_toward, wrap_angle};
pub use mover::{KinematicMover, Locomotion, MoveInput, MoverMode, MoverParams, RootMotion, separate, separate_moved};
pub use path::{ArrivalParams, PathFollower, PathMode, PathNode, PathOutput};
pub use probe::{GroundHit, GroundProbe, HeightSurface, Nearest, ProbeQuery, SurfaceId, TerrainProbe};
pub use statics::{COLLISION_FORMAT, CollisionBuilder, CollisionMesh, StaticId, StaticShape, StaticWorld};
pub use trajectory::{LocomotionParams, TRAJECTORY_TIMES, Trajectory, TrajectoryPoint};

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

#[cfg(test)]
mod tests;
