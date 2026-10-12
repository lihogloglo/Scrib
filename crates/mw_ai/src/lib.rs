//! Morrowind NPC life before `sf_ai` (M10), Bevy-free.
//!
//! - [`pathgrid`]: PGRD records as per-cell directed graphs in engine space
//!   (meters, Y up), with nearest/within/A* queries.
//! - [`wander`]: the `AI_W` package (distance, duration, Idle2–9 chances),
//!   `OpenMW`'s idle roll, and the wander goal planner.
//! - [`scaffold`]: [`PathWalkScaffold`], the named scaffold turning wander
//!   steps into `sf_controller` [`PathFollower`](sf_controller::PathFollower)
//!   walks and [`LocomotionIntent`](sf_controller::LocomotionIntent)s.
//!
//! Spec: `docs/specs/actors-adapter.md` "NPC life in M3".

pub mod pathgrid;
pub mod scaffold;
pub mod wander;

pub use pathgrid::{CellKey, PathGraph, PathGridError, PathGrids};
pub use scaffold::{PathWalkScaffold, ScaffoldOutput};
pub use wander::{Wander, WanderPlanner, WanderStep, gmst_float, roll_idle, walk_speed};

#[cfg(test)]
mod tests;
