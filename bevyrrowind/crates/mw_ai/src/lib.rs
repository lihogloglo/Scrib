//! Morrowind NPC AI adapter on `sf_ai` (`docs/specs/ai-adapter.md`), Bevy-free.
//!
//! - [`pathgrid`]: PGRD records as per-cell directed graphs in engine space
//!   (meters, Y up), with nearest/within/A* queries.
//! - [`wander`]: the `AI_W` package (distance, duration, Idle2–9 chances),
//!   `OpenMW`'s idle roll, and the wander goal planner.
//! - [`methods`]: [`WanderVocabulary`] and [`WanderDriver`], the HTN wander/idle
//!   method set on `sf_ai` (`MA3`, replacing `PathWalkScaffold`).
//! - [`packages`]: record AI packages (`AI_W`/`AI_T`/`AI_E`/`AI_F`/`AI_A`) as `sf_ai`
//!   directives (`MA2a`).
//! - [`policy`]: `OpenMW`'s `AiSequence` stack rules as [`MorrowindPolicy`] (`MA2b`).
//! - [`settings`]: AI settings considerations: the Fight / aggression rating (`MA4d`) and the Flee
//!   rating and flee decision (`MA4f`), the greeting state machine (`MA4c1`).

pub mod methods;
pub mod packages;
pub mod pathgrid;
pub mod policy;
pub mod settings;
pub mod wander;

pub use methods::{
    METHOD_WANDER_IDLE, METHOD_WANDER_STAND, METHOD_WANDER_WALK, STATE_HAS_IDLE,
    STATE_HAS_WALK_GOAL, STATE_IDLE_VARIANT, TASK_DO_IDLE, TASK_DO_WALK, TASK_WANDER,
    METHOD_WANDER_FACE, STATE_HAS_FACE, TASK_DO_FACE,
    WanderDriver, WanderKeys, WanderOutput, WanderTuning, WanderVocabulary, wander_library,
};
pub use pathgrid::{CellKey, PathGraph, PathGridError, PathGrids};
pub use policy::{MorrowindPolicy, Options, is_actual_package, options};
pub use wander::{
    Wander, WanderPlanner, WanderStep, gmst_float, roll_idle, roll_idle_with, walk_speed,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_packages;
#[cfg(test)]
mod tests_policy;
#[cfg(test)]
mod tests_settings;
