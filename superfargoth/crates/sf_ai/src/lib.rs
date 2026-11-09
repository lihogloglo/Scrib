//! NPC AI core, Bevy-free (`docs/specs/ai.md`). Layers, built slice by slice: directives →
//! utility drives → one shared HTN method library → executors; host-only, on the fixed tick.
//!
//! Built so far:
//! - AI1 (debugger first): [`ids`], the load-time indices every layer names, and [`trace`], the
//!   per-actor decision trace ([`Trace`], [`TraceStore`], JSON dumps, [`format_trace`]).
//! - AI2 (HTN core): [`state`], the typed world state ([`StateKeys`], [`WorldState`]), and
//!   [`htn`], the record-built method [`Library`] and its planner ([`decompose`], [`resume`],
//!   [`replan`] with MTR comparison), every rejected method traced. `AI2b`: full chronological
//!   backtracking, bounded by [`SEARCH_TRIES`].
//!
//! The console command `ai.trace` and BRP live in `sf_ai_bevy` (AI3): actor references and the
//! console context are the shell's.

pub mod htn;
pub mod ids;
pub mod state;
pub mod trace;

pub use htn::{
    Decomposition, LoadError, LoadReport, Library, METHOD_KIND, MethodDef, NoPlan, Operators, Replan, Resume,
    SEARCH_DEPTH, SEARCH_TRIES, SubtaskDef, TASK_KIND, TaskDef, decompose, mtr_better, register_kinds, replan, resume,
};
pub use ids::{ConsiderationId, DirectiveId, DriveId, MethodId, OperatorId, TaskId};
pub use sf_ids::ActorKey;
pub use state::{BitKey, IntKey, KeyKind, KeySet, RefKey, StateKey, StateKeyError, StateKeys, Target, WorldState};
pub use trace::{
    Admit, ConsiderationScore, DEFAULT_TRACE_CAP, DriveScore, MethodChoice, OperatorState, OperatorStatus, PathFailure,
    PathState, Plan, PlanOutcome, PlanStep, RejectReason, Rejection, TRACE_LEN, Trace, TraceEntry, TraceNames,
    TraceStore, Trigger, format_trace, format_trace_with,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_htn;
