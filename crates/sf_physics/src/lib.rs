//! Physics core, Bevy-free: one rigid-body world per worldspace behind our
//! own API (rapier3d inside; its types never leave this crate).
//!
//! - [`PhysicsWorld`]: bodies ([`BodyDesc`] → [`BodyId`]), layers, a fixed
//!   step with interpolation history ([`PhysicsWorld::pose_at`]), events
//!   ([`PhysicsEvent`]), joints with implicit PD drives, and the floating
//!   origin: an f32 frame around an anchor grid cell, rebased by whole cells
//!   ([`PhysicsWorld::rebase`], as `PxScene::shiftOrigin` does).
//! - [`PhysicsProbe`]: the controller's `GroundProbe` over the engine's
//!   query structure, filtered by [`Layer`].
//! - Shapes: `collision.cook` meshes ([`MeshShape`], one `Arc` per model),
//!   terrain chunks ([`HeightShape`]) and primitives.
//!
//! Placed items go in asleep ([`BodyDesc::item`]) and stay exactly where
//! content put them until something touches them.
//!
//! Spec: `docs/specs/physics.md`.

mod body;
mod carry;
mod character;
pub mod cook;
mod debug;
mod joint;
mod layer;
mod probe;
mod shape;
mod world;

pub use body::{BodyDesc, BodyId, BodyKind, ITEM_DAMPING, PhysicsEvent, PhysicsParams};
pub use sf_controller::SurfaceId;
pub use carry::{CarryEnd, CarryParams, Carrier, GrabMode, GrabResult, HolderLimits};
pub use character::{CharacterShape, PushParams, Pushed, impact_impulse};
pub use joint::{Drive, JointDesc, JointId, JointKind, JointSolver};
pub use layer::Layer;
pub use probe::PhysicsProbe;
pub use cook::{BodyShape, BodySource, FitSettings, PartShape, ShapePart};
pub use shape::{HeightShape, MeshShape, ShapeRef};

#[cfg(feature = "bake")]
pub mod steps;
pub use world::{PhysicsWorld, PhysicsWorlds};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod spike;
#[cfg(test)]
mod cook_tests;
#[cfg(test)]
mod character_tests;
#[cfg(test)]
mod carry_tests;
#[cfg(test)]
mod churn_tests;
#[cfg(test)]
mod contact_tests;
