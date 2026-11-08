//! NPC AI core, Bevy-free (`docs/specs/ai.md`). Layers, built slice by slice: directives →
//! utility drives → one shared HTN method library → executors; host-only, on the fixed tick.
//!
//! Built so far (AI1, debugger first): [`ids`], the load-time indices every layer names, and
//! [`trace`], the per-actor decision trace ([`Trace`], [`TraceStore`], JSON dumps,
//! [`format_trace`]). The console command `ai.trace` and BRP live in `sf_ai_bevy` (AI3): actor
//! references and the console context are the shell's.

pub mod ids;
pub mod trace;

pub use ids::{ConsiderationId, DirectiveId, DriveId, MethodId, OperatorId, TaskId};
pub use sf_ids::ActorKey;
pub use trace::{
    Admit, ConsiderationScore, DEFAULT_TRACE_CAP, DriveScore, MethodChoice, OperatorState, OperatorStatus, PathFailure,
    PathState, Plan, PlanStep, RejectReason, Rejection, TRACE_LEN, Trace, TraceEntry, TraceStore, Trigger, format_trace,
};

#[cfg(test)]
mod tests;
