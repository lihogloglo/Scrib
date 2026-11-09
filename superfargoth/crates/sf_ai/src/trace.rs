//! The per-actor decision trace (ai.md "Debugging", slice AI1): "why did this NPC do that?".
//!
//! Canonical technique: UE's Visual Logger (a per-actor, per-tick snapshot recorded as it
//! happens, replayed later) and the `StateTree` debugger (every condition's result, not only the
//! winner), Guerrilla's in-game HTN view (the decomposition and the methods it skipped), and the
//! IAUS habit of showing each consideration's curve output next to the drive's score. A
//! [`TraceEntry`] is one decision step: the drive scores and their breakdown, the chosen method
//! path, **every rejected method with why** (the failing precondition's index, or the subtask that
//! could not decompose), the plan, the running operator, path state, and the time it took.
//!
//! - [`Trace`]: one actor's last [`TRACE_LEN`] entries, a ring over std `VecDeque` (reuse: no crate
//!   adds what it lacks; the entries own `Vec`s, so the allocation-free part is [`Trace::record`],
//!   which recycles the oldest entry in place, keeping its buffers).
//! - [`TraceStore`]: [`ActorKey`] → [`Trace`] under a cap, with an off switch checked before an
//!   entry is built ([`TraceStore::record`] returns `None`), so tracing costs one branch for actors
//!   nobody traces.
//! - JSON dumps ([`Trace::to_json`], [`TraceStore::to_json`]) in declaration order, actors in
//!   [`ActorKey`] order, for agents and BRP; [`format_trace`] for the console (`ai.trace`, AI3).
//!
//! Traces are never saved (ai.md "Saves") and never feed gameplay: the decision code writes them,
//! nothing reads them back. `budget_us` is the only wall-clock field; deterministic runs (the tier
//! invariance test, golden traces) leave it `None` by turning timings off on the store.

use std::collections::{BTreeMap, VecDeque};
use std::fmt::Write as _;

use serde::{Deserialize, Serialize, Serializer};
use sf_ids::ActorKey;

use crate::ids::{ConsiderationId, DirectiveId, DriveId, MethodId, OperatorId, TaskId};

/// Entries one [`Trace`] keeps (ai.md "Debugging": 64 decisions).
pub const TRACE_LEN: usize = 64;

/// Default most actors a [`TraceStore`] traces: the largest tier-2 cap in ai.md "Tiers and budget"
/// (Dedicated, 256), so by default every tier-2 actor fits. The host sets the real CPU tier's cap.
pub const DEFAULT_TRACE_CAP: usize = 256;

/// What started this decision step (ai.md "Scheduling": staggered steps; events jump the queue).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    /// The actor's staggered slot at its tier's rate.
    #[default]
    Scheduled,
    /// An event: damage, stimulus, directive change, fact.
    Event,
    /// Plan monitoring: a read key changed or the running operator's preconditions failed.
    Replan,
}

/// One consideration's output inside a drive's score (IAUS: input through a response curve).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsiderationScore {
    pub consideration: ConsiderationId,
    /// The curve's output, 0..=1.
    pub value: f32,
}

/// A drive's score this step and its breakdown.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriveScore {
    pub drive: DriveId,
    /// Compensated product of the considerations times the drive's weight.
    pub raw: f32,
    /// What arbitration compared: `raw` after momentum, commit time and the `Directive` floor.
    /// `raw != score` is the usual answer to "why didn't it switch?".
    pub score: f32,
    pub considerations: Vec<ConsiderationScore>,
}

/// One step of the chosen decomposition: the method that expanded a compound task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MethodChoice {
    pub task: TaskId,
    pub method: MethodId,
}

/// Why a candidate method was skipped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectReason {
    /// Its precondition at this index (into the method's `pre` list) was false; the first false
    /// one, as preconditions are checked in order.
    Precondition(u16),
    /// Its preconditions held, but this subtask of its own (directly, or as the task that failed
    /// inside it) could not be planned, so the search backtracked out of it.
    SubtaskFailed(TaskId),
    /// It had been planned in full, but this later task could not be planned after it, so the
    /// search reopened this choice point (full chronological backtracking, `AI2b`): state, plan and
    /// agenda restored to just before the method was chosen, the task's next method tried.
    Backtracked(TaskId),
    /// A replan reached it with every earlier choice equal to the running plan's, and it ranks
    /// below the running plan's method here: no plan from here on can beat the running one, so
    /// the search stopped (Fluid HTN's MTR early-out; the entry's `outcome` is `kept_not_better`).
    NotBetter,
    /// A search limit stopped it: decomposition depth or the method-try budget (`htn`
    /// `SEARCH_DEPTH`, `SEARCH_TRIES`), usually a recursive method without a way out.
    Limit,
}

/// What this step's planning did (AI2; `None` when the step didn't plan). With a running plan,
/// a replan wins only with a higher-priority method traversal record (ai.md "Behaviours").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanOutcome {
    /// A fresh decomposition from the root (no running plan).
    Planned,
    /// A partial plan's rest, decomposed after its pause marker.
    Continued,
    /// A replan found a higher-priority MTR and replaced the running plan.
    Replaced,
    /// A replan's MTR was equal or lower: the running plan stays (the AI1 open item).
    KeptNotBetter,
    /// A replan found no plan at all: the running plan stays (its monitoring decides).
    KeptNoPlan,
    /// No plan, and none running.
    NoPlan,
}

/// A method decomposition tried and skipped, in the order tried.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rejection {
    /// The compound task it was a candidate for.
    pub task: TaskId,
    pub method: MethodId,
    pub reason: RejectReason,
}

/// One primitive step of the plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanStep {
    pub task: TaskId,
    pub operator: OperatorId,
}

/// The plan after this step.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub steps: Vec<PlanStep>,
    /// Index of the running step.
    pub cursor: u16,
    /// Decomposition stopped at a pause marker; the rest is planned later (ai.md "Behaviours").
    pub partial: bool,
}

/// The running operator's state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorStatus {
    #[default]
    Running,
    Succeeded,
    Failed,
}

/// The running operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OperatorState {
    pub operator: OperatorId,
    pub status: OperatorStatus,
}

/// Why a path failed (ai.md "Executors": `MoveTo` fails on `NoPath` or 3 s without progress).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathFailure {
    NoPath,
    NoProgress,
    Cancelled,
}

/// The actor's path request state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathState {
    #[default]
    None,
    Pending,
    Following,
    Failed(PathFailure),
}

/// One decision step of one actor. Fields serialize in this order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TraceEntry {
    /// Fixed simulation tick.
    pub tick: u64,
    /// The actor's AI tier: 2 full, 1 reduced, 0 logical.
    pub tier: u8,
    pub trigger: Trigger,
    /// The directive the actor is serving (top of its stack after arbitration).
    pub directive: Option<DirectiveId>,
    /// Every drive scored this step (empty when drives were not re-arbitrated).
    pub drives: Vec<DriveScore>,
    /// The winning drive.
    pub drive: Option<DriveId>,
    /// The chosen decomposition, root task first.
    pub methods: Vec<MethodChoice>,
    /// Methods tried and skipped, in the order tried.
    pub rejected: Vec<Rejection>,
    pub plan: Plan,
    pub operator: Option<OperatorState>,
    pub path: PathState,
    /// Wall time this actor's step took, in µs; `None` when the store's timings are off.
    pub budget_us: Option<u32>,
    /// The method traversal record of the plan in force: each choice's rank among its task's
    /// methods (0 = highest priority), parallel to `methods` (AI2).
    pub mtr: Vec<u16>,
    /// What planning did this step (AI2). `methods`, `plan` and `mtr` describe the plan in force
    /// after the step (for `kept_*`, the running plan); `rejected` is this step's search.
    pub outcome: Option<PlanOutcome>,
}

impl TraceEntry {
    /// An empty entry for `tick` at `tier`.
    pub fn new(tick: u64, tier: u8) -> Self {
        Self { tick, tier, ..Self::default() }
    }

    /// Empties the entry for a new step, keeping its buffers' capacity.
    pub fn reset(&mut self, tick: u64, tier: u8) {
        self.tick = tick;
        self.tier = tier;
        self.trigger = Trigger::default();
        self.directive = None;
        self.drives.clear();
        self.drive = None;
        self.methods.clear();
        self.rejected.clear();
        self.plan.steps.clear();
        self.plan.cursor = 0;
        self.plan.partial = false;
        self.operator = None;
        self.path = PathState::None;
        self.budget_us = None;
        self.mtr.clear();
        self.outcome = None;
    }
}

/// One actor's last [`TRACE_LEN`] entries, oldest first. Serializes as a JSON array.
#[derive(Clone, Debug, PartialEq)]
pub struct Trace {
    pub(crate) entries: VecDeque<TraceEntry>,
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Trace {
    /// An empty trace with room for [`TRACE_LEN`] entries (the only ring allocation).
    pub fn new() -> Self {
        Self { entries: VecDeque::with_capacity(TRACE_LEN) }
    }

    /// Appends `entry`, dropping the oldest when full.
    pub fn push(&mut self, entry: TraceEntry) {
        if self.entries.len() == TRACE_LEN {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Appends an empty entry for `tick` at `tier` and returns it to fill. When full, the oldest
    /// entry is reset and reused, so a warm trace records without allocating.
    pub fn record(&mut self, tick: u64, tier: u8) -> &mut TraceEntry {
        let entry = if self.entries.len() == TRACE_LEN {
            let mut old = self.entries.pop_front().unwrap_or_default();
            old.reset(tick, tier);
            old
        } else {
            TraceEntry::new(tick, tier)
        };
        self.entries.push_back(entry);
        self.entries.back_mut().expect("just pushed")
    }

    /// Oldest to newest.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &TraceEntry> + ExactSizeIterator {
        self.entries.iter()
    }

    /// The newest entry.
    pub fn last(&self) -> Option<&TraceEntry> {
        self.entries.back()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Drops every entry (keeps the ring's allocation).
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Pretty JSON: an array of entries, oldest first.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Serialize for Trace {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(&self.entries)
    }
}

/// Which actors a [`TraceStore`] admits on its own, besides watched ones.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Admit {
    /// Actors recording at tier 2 (the default: every full-detail actor).
    #[default]
    Tier2,
    /// Every actor that records, any tier.
    All,
    /// Only watched actors ([`TraceStore::watch`]).
    Watched,
}

#[derive(Clone, Debug)]
struct Slot {
    trace: Trace,
    /// Admission order (first admitted = 0); ranks unwatched traces.
    admitted: u64,
}

/// Every traced actor's [`Trace`], under a cap.
///
/// **Eviction rule.** The traced set is the first `cap` actors of one ranking: watched actors,
/// newest watch first, then the others in admission order (first admitted first). So:
/// - an unwatched actor is admitted at its first [`record`](Self::record) only while there is
///   room, and is never pushed out by a later unwatched one (the traced set doesn't churn with
///   update order; within a tick, batches apply in [`ActorKey`] order, so admission is
///   deterministic);
/// - watching an actor when full evicts the most recently admitted unwatched trace; watching more
///   than `cap` actors drops the oldest watch together with its trace.
///
/// A trace stays until [`forget`](Self::forget) (the allocator calls it on despawn or unload),
/// [`unwatch`](Self::unwatch), [`clear`](Self::clear) or a smaller [`set_cap`](Self::set_cap),
/// including after its actor leaves tier 2 (the tier change is part of the story).
#[derive(Clone, Debug)]
pub struct TraceStore {
    enabled: bool,
    timings: bool,
    admit: Admit,
    cap: usize,
    /// Oldest watch first.
    watched: Vec<ActorKey>,
    traces: BTreeMap<ActorKey, Slot>,
    next_admission: u64,
}

impl Default for TraceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceStore {
    /// Enabled, timings on, [`Admit::Tier2`], cap [`DEFAULT_TRACE_CAP`].
    pub fn new() -> Self {
        Self {
            enabled: true,
            timings: true,
            admit: Admit::default(),
            cap: DEFAULT_TRACE_CAP,
            watched: Vec::new(),
            traces: BTreeMap::new(),
            next_admission: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The off switch: when off, [`record`](Self::record) returns `None` at the first branch.
    /// Turning it off keeps existing traces (inspect them, or [`clear`](Self::clear)).
    pub fn set_enabled(&mut self, on: bool) {
        self.enabled = on;
    }

    /// Whether callers should measure `budget_us` (off for deterministic runs).
    pub fn timings(&self) -> bool {
        self.timings
    }

    pub fn set_timings(&mut self, on: bool) {
        self.timings = on;
    }

    pub fn admit(&self) -> Admit {
        self.admit
    }

    /// Changes which actors are admitted from now on; existing traces stay.
    pub fn set_admit(&mut self, admit: Admit) {
        self.admit = admit;
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    /// Sets the cap (at least 1), evicting by the ranking (see the type docs) when it shrinks.
    pub fn set_cap(&mut self, cap: usize) {
        self.cap = cap.max(1);
        while self.watched.len() > self.cap {
            let oldest = self.watched.remove(0);
            self.traces.remove(&oldest);
        }
        while self.traces.len() > self.cap {
            if !self.evict_newest_unwatched() {
                break;
            }
        }
    }

    /// Always trace `actor` from its next step (ai.md `ai.select`). See the eviction rule.
    pub fn watch(&mut self, actor: ActorKey) {
        if let Some(i) = self.watched.iter().position(|&a| a == actor) {
            self.watched.remove(i);
        } else if self.watched.len() == self.cap {
            let oldest = self.watched.remove(0);
            self.traces.remove(&oldest);
        }
        self.watched.push(actor);
    }

    /// Stops watching `actor` and drops its trace (it may be admitted again like any actor).
    pub fn unwatch(&mut self, actor: ActorKey) {
        if let Some(i) = self.watched.iter().position(|&a| a == actor) {
            self.watched.remove(i);
            self.traces.remove(&actor);
        }
    }

    pub fn is_watched(&self, actor: ActorKey) -> bool {
        self.watched.contains(&actor)
    }

    /// Watched actors, oldest watch first.
    pub fn watched(&self) -> &[ActorKey] {
        &self.watched
    }

    /// Whether a step of `actor` at `tier` would be recorded. The cheap check to make before
    /// gathering anything for an entry; [`record`](Self::record) makes it too.
    #[inline]
    pub fn wants(&self, actor: ActorKey, tier: u8) -> bool {
        if !self.enabled {
            return false;
        }
        if self.traces.contains_key(&actor) || self.is_watched(actor) {
            return true;
        }
        let admits = match self.admit {
            Admit::Tier2 => tier == 2,
            Admit::All => true,
            Admit::Watched => false,
        };
        admits && self.traces.len() < self.cap
    }

    /// Starts `actor`'s entry for `tick` at `tier` and returns it to fill, or `None` when tracing
    /// is off or the actor isn't traced. Admits the actor on its first step.
    pub fn record(&mut self, actor: ActorKey, tick: u64, tier: u8) -> Option<&mut TraceEntry> {
        if !self.wants(actor, tier) {
            return None;
        }
        if !self.traces.contains_key(&actor) {
            if self.traces.len() >= self.cap {
                // Only a watched actor gets here (see `wants`); the watch cap leaves an unwatched one.
                self.evict_newest_unwatched();
            }
            let admitted = self.next_admission;
            self.next_admission += 1;
            self.traces.insert(actor, Slot { trace: Trace::new(), admitted });
        }
        self.traces.get_mut(&actor).map(|slot| slot.trace.record(tick, tier))
    }

    /// Records a whole entry (tests and replays); `false` when not traced.
    pub fn push(&mut self, actor: ActorKey, entry: TraceEntry) -> bool {
        match self.record(actor, entry.tick, entry.tier) {
            Some(slot) => {
                *slot = entry;
                true
            }
            None => false,
        }
    }

    /// `actor`'s trace.
    pub fn get(&self, actor: ActorKey) -> Option<&Trace> {
        self.traces.get(&actor).map(|slot| &slot.trace)
    }

    /// Traced actors in [`ActorKey`] order.
    pub fn iter(&self) -> impl Iterator<Item = (ActorKey, &Trace)> {
        self.traces.iter().map(|(&actor, slot)| (actor, &slot.trace))
    }

    /// Number of traced actors.
    pub fn len(&self) -> usize {
        self.traces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }

    /// Drops `actor`'s trace (it may be admitted again); a watch stays.
    pub fn forget(&mut self, actor: ActorKey) {
        self.traces.remove(&actor);
    }

    /// Drops every trace; watches and settings stay.
    pub fn clear(&mut self) {
        self.traces.clear();
    }

    /// Pretty JSON: `{"actors": [{"actor": key, "entries": [...]}, ...]}`, actors in key order.
    pub fn to_json(&self) -> serde_json::Result<String> {
        #[derive(Serialize)]
        struct ActorDump<'a> {
            actor: ActorKey,
            entries: &'a Trace,
        }
        #[derive(Serialize)]
        struct StoreDump<'a> {
            actors: Vec<ActorDump<'a>>,
        }
        let actors = self.iter().map(|(actor, entries)| ActorDump { actor, entries }).collect();
        serde_json::to_string_pretty(&StoreDump { actors })
    }

    fn evict_newest_unwatched(&mut self) -> bool {
        let newest = self
            .traces
            .iter()
            .filter(|(actor, _)| !self.watched.contains(actor))
            .max_by_key(|(_, slot)| slot.admitted)
            .map(|(&actor, _)| actor);
        match newest {
            Some(actor) => {
                self.traces.remove(&actor);
                true
            }
            None => false,
        }
    }
}

/// How [`format_trace_with`] prints ids. The defaults print indices (`t1`, `m7`, `o4`);
/// `htn::Library` prints record names.
pub trait TraceNames {
    fn task(&self, out: &mut String, id: TaskId) {
        let _ = write!(out, "{id}");
    }
    fn method(&self, out: &mut String, id: MethodId) {
        let _ = write!(out, "{id}");
    }
    fn operator(&self, out: &mut String, id: OperatorId) {
        let _ = write!(out, "{id}");
    }
}

/// Ids as indices.
struct Indices;
impl TraceNames for Indices {}

/// The last `n` entries of `trace`, oldest first, one line each, for the console (`ai.trace`):
///
/// `#<tick> t<tier> <trigger> drive <d> via <task:method>... rej <first rejection> [(+k)] [plan <outcome>] op <o> <status> path <state> [<µs>us]`
///
/// `-` marks an empty field; `plan <outcome>` only appears when the step planned. Ids print as
/// their indices (`d3`, `t1:m7`); [`format_trace_with`] takes a name resolver.
pub fn format_trace(trace: &Trace, n: usize) -> String {
    format_trace_with(trace, n, &Indices)
}

/// [`format_trace`] with tasks, methods and operators printed through `names`.
pub fn format_trace_with(trace: &Trace, n: usize, names: &dyn TraceNames) -> String {
    let mut out = String::new();
    let skip = trace.len().saturating_sub(n);
    for e in trace.iter().skip(skip) {
        let _ = write!(out, "#{} t{} {}", e.tick, e.tier, trigger_name(e.trigger));
        match e.drive {
            Some(d) => {
                let _ = write!(out, " drive {d}");
            }
            None => out.push_str(" drive -"),
        }
        out.push_str(" via ");
        if e.methods.is_empty() {
            out.push('-');
        }
        for (i, m) in e.methods.iter().enumerate() {
            if i > 0 {
                out.push('>');
            }
            names.task(&mut out, m.task);
            out.push(':');
            names.method(&mut out, m.method);
        }
        match e.rejected.first() {
            Some(r) => {
                out.push_str(" rej ");
                names.task(&mut out, r.task);
                out.push(':');
                names.method(&mut out, r.method);
                out.push(' ');
                match r.reason {
                    RejectReason::Precondition(i) => {
                        let _ = write!(out, "pre{i}");
                    }
                    RejectReason::SubtaskFailed(t) => {
                        out.push_str("sub ");
                        names.task(&mut out, t);
                    }
                    RejectReason::Backtracked(t) => {
                        out.push_str("back ");
                        names.task(&mut out, t);
                    }
                    RejectReason::NotBetter => out.push_str("not_better"),
                    RejectReason::Limit => out.push_str("limit"),
                }
                if e.rejected.len() > 1 {
                    let _ = write!(out, " (+{})", e.rejected.len() - 1);
                }
            }
            None => out.push_str(" rej -"),
        }
        if let Some(outcome) = e.outcome {
            out.push_str(" plan ");
            out.push_str(match outcome {
                PlanOutcome::Planned => "planned",
                PlanOutcome::Continued => "continued",
                PlanOutcome::Replaced => "replaced",
                PlanOutcome::KeptNotBetter => "kept_not_better",
                PlanOutcome::KeptNoPlan => "kept_no_plan",
                PlanOutcome::NoPlan => "no_plan",
            });
        }
        match e.operator {
            Some(op) => {
                let status = match op.status {
                    OperatorStatus::Running => "running",
                    OperatorStatus::Succeeded => "succeeded",
                    OperatorStatus::Failed => "failed",
                };
                out.push_str(" op ");
                names.operator(&mut out, op.operator);
                let _ = write!(out, " {status}");
            }
            None => out.push_str(" op -"),
        }
        out.push_str(" path ");
        out.push_str(match e.path {
            PathState::None => "-",
            PathState::Pending => "pending",
            PathState::Following => "following",
            PathState::Failed(PathFailure::NoPath) => "failed(no_path)",
            PathState::Failed(PathFailure::NoProgress) => "failed(no_progress)",
            PathState::Failed(PathFailure::Cancelled) => "failed(cancelled)",
        });
        if let Some(us) = e.budget_us {
            let _ = write!(out, " {us}us");
        }
        out.push('\n');
    }
    out
}

fn trigger_name(trigger: Trigger) -> &'static str {
    match trigger {
        Trigger::Scheduled => "scheduled",
        Trigger::Event => "event",
        Trigger::Replan => "replan",
    }
}
