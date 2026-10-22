//! Animation graphs, Bevy-free: graph assets, the `animgraph.compile` bake
//! step, and the evaluator (layers with weighted masks, blend trees, 1D and
//! 2D blend spaces, state machines, sync groups with markers,
//! inertialization, mirroring, montage slots, the procedural op stack).
//!
//! The pipeline:
//! - Authoring: `*.animgraph.ron` ([`asset::GraphAsset`]), names everywhere.
//! - Bake (`bake` feature): [`compile`] resolves names against a skeleton
//!   and its cooked clips, weights masks, triangulates blend spaces, and
//!   checks pose kinds; [`steps::AnimGraphCompileStep`] caches it as the
//!   `animgraph.compile` product.
//! - Runtime: a [`Binding`] (compiled graph + skeleton + clips) and one
//!   [`GraphInstance`] per character. [`GraphInstance::update`] is a pure
//!   function of the instance, parameters, dt and [`UpdateInputs`], so
//!   characters update in parallel with identical results on any thread
//!   count.
//!
//! Spec: `docs/specs/animation.md`.

#![allow(clippy::many_single_char_names)]

pub mod asset;
mod blendspace;
#[cfg(feature = "bake")]
mod compile;
mod compiled;
mod inertial;
mod instance;
mod mirror;
mod procedural;
mod secondary;
pub mod slot;
#[cfg(feature = "bake")]
pub mod steps;
mod sync;
pub mod variety;

pub use blendspace::Weights;
#[cfg(feature = "bake")]
pub use blendspace::triangulate;
#[cfg(feature = "bake")]
pub use compile::{CompileError, CompileSource, compile, compile_str};
pub use compiled::{
    Blend, BlendSpace, BodyEffector, ClipResolution, ClipSlot, CompiledGraph, Condition, GRAPH_FORMAT, GirdleSeg, HitReactOp, Layer, Leg, Machine, MirrorTable,
    Node, NodeKind, Op, OpEntry, ParamDef, ParamId, ParamType, PlayerNode, Provenance, SlotDef, SpringBonesOp, State, Transition, Value, WarpMode, WarpRotation,
    WarpWindow, parse_warp_window,
};
pub use inertial::Inertializer;
pub use variety::{FidgetPicker, Variety};
pub use secondary::{CLOTH_LEG_FOLLOW, CLOTH_MAX_LOD, ClothBinding, ClothRun, Hit, SPRING_MAX_SUBSTEPS, SPRING_SUBSTEP};
pub use instance::{
    ActiveWarp, BindError, Binding, FootStats, GraphEvent, GraphInstance, GraphOutput, GroundSample, IkTarget, Params, UpdateInputs, WarpTarget,
};
pub use slot::{LateClip, MontageClip, MontageId, MontageStatus, NoticeKind, SlotError, SlotId, SlotNotice, SlotRequest, SlotStart};

#[cfg(all(test, feature = "bake"))]
mod creature_tests;
#[cfg(all(test, feature = "bake"))]
mod tests;
