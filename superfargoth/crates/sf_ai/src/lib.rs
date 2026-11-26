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
//! - AI4 (directives): [`directive`], the per-actor [`DirectiveStack`] of content orders, the
//!   adapter's [`StackPolicy`] seam with the modern default [`ModernPolicy`], and the
//!   [`DirectiveDone`] completion events, all on one logical clock.
//! - AI5 (saves): [`save`], the `sf/ai` section's shape ([`AiSave`]: per actor, its
//!   [`SavedStack`]), key and version, and the checks a loaded section must pass.
//! - AI7 (drives): [`drive`], IAUS utility arbitration: record-built [`Drives`] (considerations
//!   with response [`Curve`]s over an adapter [`Inputs`] vocabulary), the pure [`arbitrate`]
//!   (momentum, minimum commit, the `Directive` floor, ties by record id), [`serving_for`] onto
//!   AI4's [`Serving`], and the save-ready [`ArbiterState`].
//! - `AI8a` (awareness core): [`perception`], the per-actor [`AwarenessMeter`] ([`Awareness`]
//!   entries, five [`AwarenessBand`]s with hysteresis, decay and the Alerted hold, the
//!   [`Distraction`] slot, asleep and groggy), fed [`Stimulus`] inputs, and the HUD's
//!   [`player_detection`].
//! - `AI8e` (shared knowledge): a shout's [`Notify`] queued per hearer as a [`PendingNotify`] and
//!   delivered after travel and reaction time, at most to Searching ([`NotifyDelivered`]).
//! - `AI8g` (perception profile): [`profile`], the per-individual [`PerceptionProfile`], its
//!   timed and condition [`PerceptionStates`], and [`resolve`] into the cached [`EffectivePerception`].
//! - `AI8b1` (sight maths): [`sight`], the per-check [`sight_gain`], [`apply_darkness`], the
//!   staggered [`SightTimer`], the shared [`RayQueue`] behind the caller's [`LineOfSight`], and the
//!   [`SightCache`].
//! - `AI8f1` (search core): [`search`], the seeded widening [`SearchState`] at the last known
//!   position, its [`SearchGoal`]s for the move executors, and [`OP_SEARCH`].
//! - `AI8h1` (band reactions core): [`react`], the Curious / Suspicious [`ReactState`] (double
//!   take, held goal, spent placement) and its [`ReactGoal`]s for the move executors.
//! - `AI9a` (tier allocator core): [`alloc`], the [`Allocator`] of tiers 2/1/0 under the fixed
//!   [`CpuTier`] caps (pins, visibility, nearest-player distance, hysteresis, staggered first decide).
//! - `AI10a` (tier-0 core): [`logical`], the event-stepped [`LogicalState`] (route, progress,
//!   activity, until) every logical actor runs, the per-kind [`LogicalTransitions`] (custom kinds
//!   registered, [`LogicalTransitions::validate`] at load), the [`LogicalEnv`] seam and
//!   [`fast_forward`] with the adapter's [`FastForwardRules`].
//! - `AI10b` (promotion core): [`promote`], the nearest-first [`PromoteQueue`] under a fixed count
//!   per tick, [`resolve_promotion`] (unseen placement: spot, back along the route, approaches) on
//!   the [`PlayerViews`] / [`PromoteEnv`] seams, and [`demote`].
//!
//! The console command `ai.trace` and BRP live in `sf_ai_bevy` (AI3): actor references and the
//! console context are the shell's.

pub mod alloc;
pub mod directive;
pub mod drive;
pub mod exec;
pub mod htn;
pub mod ids;
pub mod light;
pub mod logical;
pub mod perception;
pub mod profile;
pub mod promote;
pub mod react;
pub mod save;
pub mod search;
pub mod sight;
pub mod state;
pub mod trace;

pub use alloc::{
    AllocConfig, AllocInput, AllocStats, Allocation, Allocator, CpuTier, Pin, Tier, TierCaps, TierChange,
    decide_due, first_decide_after,
};
pub use directive::{
    Directive, DirectiveDone, DirectiveEnd, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, DirectiveStatus,
    Limit, MAX_REACTIONS, ModernPolicy, SavedStack, Serving, StackEdit, StackError, StackPolicy, format_directives,
};
pub use drive::{
    ArbiterState, Arbitration, CONSIDERATION_KIND, Consideration, ConsiderationDef, Curve, DRIVE_KIND, Drive, DriveDef,
    DriveLoadError, DriveLoadReport, Drives, Inputs, SavedArbiter, Scored, arbitrate, register_drive_kinds, serving_for,
};
pub use exec::{
    ExecConfig, ExecContext, ExecGait, ExecOutput, FaceTarget, MoveAction, MoveOperatorIds, NavCommand, NavMoveState,
     OP_FACE, OP_MOVE_TO, OP_WAIT, OperatorBindings, OperatorExec, OperatorKind, OperatorResolver, PlanRunner,
    register_move_operators, wrap_angle, yaw_toward,
};
pub use htn::{
    Decomposition, LoadError, LoadReport, Library, METHOD_KIND, MethodDef, NoPlan, Operators, Replan, Resume,
    SEARCH_DEPTH, SEARCH_TRIES, SubtaskDef, TASK_KIND, TaskDef, decompose, mtr_better, register_kinds, replan, resume,
};
pub use ids::{ConsiderationId, DirectiveId, DriveId, InputId, MethodId, NoiseKindId, OperatorId, StatId, TaskId};
pub use light::{
    LightConfig, LightSource, MAX_LIGHT_RAYS, Stance, VISIBILITY_MODIFIERS_CAPACITY, VisibilityFactors, is_dark,
    light_exposure, source_contribution,
};
pub use logical::{
    ActivityRef, Arrival, CustomTransition, FastForwardRules, LogicalCtx, LogicalEnv, LogicalState, LogicalTransitions,
    ModernFastForward, RouteRef, Transition, TransitionInput, UnregisteredKind, fast_forward,
};
pub use perception::{
    Awareness, AwarenessBand, AwarenessConfig, AwarenessMeter, Distraction, HeardNoise, LastSeen, Notify,
    NotifyDelivered, PendingNotify, PerceptClock, PerceptTarget, PlayerDetection, Stimulus, StimulusOutcome,
    player_detection,
};
pub use profile::{
    EffectivePerception, OutOfRange, PERCEPTION_PROFILE_KIND, PERCEPTION_STATE_KIND, PERCEPTION_STATES_CAPACITY,
    PerceptionField, PerceptionProfile, PerceptionRangeError, PerceptionState, PerceptionStates, PushOutcome,
    SearchConfig, SearchParams, SightConfig, SightParams, StateEntry, register_perception_kinds, resolve,
};
pub use promote::{
    EnteredFrom, Placement, PlayerViews, PromoteConfig, PromoteEnv, PromoteQueue, PromoteRequest, PromoteTick, Resolve,
    demote, resolve_promotion,
};
pub use react::{ReactConfig, ReactGoal, ReactInput, ReactState, investigate_point};
pub use save::{AI_SECTION, AI_VERSION, ActorSave, AiSave, AiSaveError};
pub use search::{
    OP_SEARCH, SearchGoal, SearchParamError, SearchPatternConfig, SearchState, register_search_operator, search_seed,
};
pub use sight::{
    LineOfSight, LosOutcome, LosRay, RayKind, RayQueue, RayQueueFull, RayRequest, RayResult, RaySubject, RayTickStats,
    SightBudget, SightCache, SightCheck, SightTimer, actor_hash, apply_darkness, due, seconds_to_ticks, sight_gain,
    sight_rate, stagger_phase,
};
pub use sf_ids::ActorKey;
pub use sf_rpg::GameTime;
pub use state::{BitKey, IntKey, KeyKind, KeySet, RefKey, StateKey, StateKeyError, StateKeys, Target, WorldState};
pub use trace::{
    Admit, ConsiderationScore, DEFAULT_TRACE_CAP, DriveReason, DriveScore, MethodChoice, OperatorState, OperatorStatus, PathFailure,
    PathState, Plan, PlanOutcome, PlanStep, RejectReason, Rejection, TRACE_LEN, Trace, TraceEntry, TraceNames,
    TraceStore, Trigger, format_trace, format_trace_with,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_alloc;
#[cfg(test)]
mod tests_directive;
#[cfg(test)]
mod tests_drive;
#[cfg(test)]
mod tests_exec;
#[cfg(test)]
mod tests_htn;
#[cfg(test)]
mod tests_light;
#[cfg(test)]
mod tests_logical;
#[cfg(test)]
mod tests_notify;
#[cfg(test)]
mod tests_perception;
#[cfg(test)]
mod tests_profile;
#[cfg(test)]
mod tests_promote;
#[cfg(test)]
mod tests_react;
#[cfg(test)]
mod tests_save;
#[cfg(test)]
mod tests_search;
#[cfg(test)]
mod tests_sight;
