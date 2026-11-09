//! The shared HTN method library and its planner (ai.md "Behaviours", slice AI2).
//!
//! **Canonical technique:** total-order forward decomposition (Humphreys, "Exploring HTN
//! Planners through Example", Game AI Pro ch. 12; Fluid HTN; Guerrilla's Decima planner):
//! decompose the root task left to right, each compound task through the first of its
//! priority-ordered methods whose preconditions hold on the planning state, primitive tasks'
//! effects applied to that copy as the plan grows. Design (not code) from Fluid HTN (MIT,
//! github.com/ptrefall/fluid-hierarchical-task-network, `Selector.cs`/`Sequence.cs`/`Domain.cs`):
//!
//! - **Backtracking is method-local** (Fluid's sequence/selector semantics): a method whose
//!   subtask can't be planned is rejected ([`RejectReason::SubtaskFailed`]), its effects are
//!   undone, and the same task's next method is tried. Alternatives of sibling subtasks already
//!   planned are not re-tried (Humphreys' chronological restore and Decima's Prolog-style search
//!   would): one pass per method keeps the cost linear in the library and the trace readable;
//!   designers order methods instead.
//! - **Partial plans:** a [`Subtask::Pause`] marker in a method stops decomposition once the plan
//!   has at least one step; [`resume`] decomposes the rest later from the then-current state.
//!   Paused work is committed: if the rest can't be planned, [`resume`] fails and the caller
//!   replans from the root.
//! - **MTR:** every plan carries its method traversal record (each choice's rank among its task's
//!   methods, in decomposition order). [`replan`] adopts a new plan only when its MTR is
//!   lexicographically better on the common prefix ([`mtr_better`]), and stops early when it can
//!   no longer be (Fluid's `BeatsLastMTR`), so a replan of an NPC that keeps doing the same thing
//!   costs a few precondition checks.
//!
//! Effects live on primitive tasks (the operator's expected outcome, as in Humphreys, Fluid and
//! SHOP), not on methods as the spec's illustrative `Method` had it: a method-level effect has no
//! defined time in a total-order plan with pause markers. Effects are planning-only (Fluid's
//! `PlanOnly`); executors and perception write the real state (AI6, AI8).
//!
//! Records (mod-patchable, `sf_records` serde kinds): [`TASK_KIND`] ([`TaskDef`]) and
//! [`METHOD_KIND`] ([`MethodDef`]). Names resolve once at [`Library::build`] against the
//! [`StateKeys`] and [`Operators`] vocabularies; every unknown name is reported ([`LoadReport`]).
//! Methods of a task are ordered by priority (high first), ties by record id, so the order does
//! not depend on load order. Plans are never saved (ai.md "Saves").

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError, RecordId};

use crate::ids::{MethodId, OperatorId, TaskId};
use crate::state::{BitKey, IntKey, KeyKind, KeySet, RefKey, StateKey, StateKeys, Target, WorldState};
use crate::trace::{MethodChoice, PlanOutcome, PlanStep, RejectReason, Rejection, TraceEntry, TraceNames};

/// Record kind of HTN tasks ([`TaskDef`]); the record id is the task's name.
pub const TASK_KIND: &str = "sf.ai.task";
/// Record kind of HTN methods ([`MethodDef`]); the record id is the method's name.
pub const METHOD_KIND: &str = "sf.ai.method";

/// Deepest decomposition (nested compound tasks) before a branch is cut ([`RejectReason::Limit`]).
pub const SEARCH_DEPTH: usize = 64;
/// Most method tries per decomposition before it gives up ([`NoPlan::Limit`]): a guard against
/// content whose search explodes, far above what a sane library needs (tens).
pub const SEARCH_TRIES: u32 = 4096;

/// Registers [`TASK_KIND`] and [`METHOD_KIND`]. Call once before loading mods.
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<TaskDef>(TASK_KIND)?;
    db.register_serde::<MethodDef>(METHOD_KIND)?;
    Ok(())
}

// --- Records ---------------------------------------------------------------------------------

/// Integer comparison of a predicate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    fn test(self, a: i32, b: i32) -> bool {
        match self {
            Self::Eq => a == b,
            Self::Ne => a != b,
            Self::Lt => a < b,
            Self::Le => a <= b,
            Self::Gt => a > b,
            Self::Ge => a >= b,
        }
    }
}

/// A precondition as content writes it: a test of named state keys.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateDef {
    /// The bit key equals `value`.
    Bit { key: String, value: bool },
    /// `int(key) op value`.
    Int { key: String, op: CmpOp, value: i32 },
    /// The ref key holds a target (`set: true`) or none (`set: false`).
    Ref { key: String, set: bool },
    /// Both ref keys hold the same target (both set).
    SameRef { a: String, b: String },
}

/// A primitive task's planning effect, as content writes it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectDef {
    SetBit { key: String, value: bool },
    SetInt { key: String, value: i32 },
    /// Saturating add.
    AddInt { key: String, by: i32 },
    /// Copies the ref `from` into `to`.
    CopyRef { from: String, to: String },
    ClearRef { key: String },
}

/// An `sf.ai.task` record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskDef {
    /// Decomposed through the `sf.ai.method` records naming it.
    Compound,
    /// Runs an operator (executor) registered in [`Operators`].
    Primitive {
        operator: String,
        /// Checked when planned and, by the running operator, every step (ai.md "Behaviours").
        #[serde(default)]
        pre: Vec<PredicateDef>,
        #[serde(default)]
        effects: Vec<EffectDef>,
    },
}

/// One entry of a method's subtask list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubtaskDef {
    Task(String),
    /// Stop planning here; the rest is planned when the plan before it has run.
    Pause,
}

/// An `sf.ai.method` record: one way to decompose `task`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodDef {
    pub task: String,
    /// Higher first; ties by record id.
    #[serde(default)]
    pub priority: u16,
    /// Checked in order; the first false one is the rejection's index.
    #[serde(default)]
    pub pre: Vec<PredicateDef>,
    pub subtasks: Vec<SubtaskDef>,
}

// --- Vocabulary ------------------------------------------------------------------------------

/// The operator vocabulary: names primitive tasks may run, registered by the executors' owner
/// (AI6) before the library loads. Names fold to ASCII lowercase; ids follow registration order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Operators {
    names: Vec<String>,
    by_name: HashMap<String, OperatorId>,
}

impl Operators {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or finds) `name`.
    ///
    /// # Panics
    /// Past `u16::MAX` operators.
    pub fn add(&mut self, name: &str) -> OperatorId {
        let name = name.to_ascii_lowercase();
        if let Some(&id) = self.by_name.get(&name) {
            return id;
        }
        let id = OperatorId(u16::try_from(self.names.len()).expect("fewer than 65536 operators"));
        self.names.push(name.clone());
        self.by_name.insert(name, id);
        id
    }

    pub fn get(&self, name: &str) -> Option<OperatorId> {
        self.by_name.get(&name.to_ascii_lowercase()).copied()
    }

    /// `"?"` for an id from another vocabulary.
    pub fn name(&self, id: OperatorId) -> &str {
        self.names.get(usize::from(id.0)).map_or("?", String::as_str)
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

// --- Compiled library ------------------------------------------------------------------------

/// A resolved precondition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Predicate {
    Bit(BitKey, bool),
    Int(IntKey, CmpOp, i32),
    Ref(RefKey, bool),
    SameRef(RefKey, RefKey),
}

impl Predicate {
    pub fn eval(self, ws: &WorldState) -> bool {
        match self {
            Self::Bit(k, v) => ws.bit(k) == v,
            Self::Int(k, op, v) => op.test(ws.int(k), v),
            Self::Ref(k, set) => ws.target(k).is_some() == set,
            Self::SameRef(a, b) => ws.target(a).is_some() && ws.target(a) == ws.target(b),
        }
    }

    fn read(self, reads: &mut KeySet) {
        match self {
            Self::Bit(k, _) => reads.insert(StateKey::Bit(k)),
            Self::Int(k, ..) => reads.insert(StateKey::Int(k)),
            Self::Ref(k, _) => reads.insert(StateKey::Ref(k)),
            Self::SameRef(a, b) => {
                reads.insert(StateKey::Ref(a));
                reads.insert(StateKey::Ref(b));
            }
        }
    }
}

/// A resolved effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    SetBit(BitKey, bool),
    SetInt(IntKey, i32),
    AddInt(IntKey, i32),
    CopyRef { from: RefKey, to: RefKey },
    ClearRef(RefKey),
}

/// A resolved subtask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Subtask {
    Task(TaskId),
    Pause,
}

/// A primitive task: an operator with its planning preconditions and effects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Primitive {
    pub operator: OperatorId,
    pub pre: Vec<Predicate>,
    pub effects: Vec<Effect>,
}

/// A task, compound or primitive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Task {
    /// Its methods, highest priority first (the index is the method's MTR rank).
    Compound(Vec<MethodId>),
    Primitive(Primitive),
}

/// A resolved method.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub task: TaskId,
    pub priority: u16,
    /// Position in its task's method list (0 = tried first).
    pub rank: u16,
    pub pre: Vec<Predicate>,
    pub subtasks: Vec<Subtask>,
}

/// The one shared method library every NPC plans with (ai.md "Behaviours").
#[derive(Clone, Debug)]
pub struct Library {
    tasks: Vec<Task>,
    task_names: Vec<RecordId>,
    task_ids: HashMap<RecordId, TaskId>,
    methods: Vec<Method>,
    method_names: Vec<RecordId>,
    method_ids: HashMap<RecordId, MethodId>,
    keys: StateKeys,
    operators: Operators,
}

/// Where a load error is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct At {
    pub kind: &'static str,
    pub id: RecordId,
}

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} `{}`", self.kind, self.id)
    }
}

/// One problem found while building a [`Library`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum LoadError {
    #[error("{at}: unknown task `{name}`")]
    UnknownTask { at: At, name: String },
    #[error("{at}: unknown state key `{name}`")]
    UnknownKey { at: At, name: String },
    #[error("{at}: state key `{name}` is of kind {found}, expected {expected}")]
    KeyKind { at: At, name: String, expected: KeyKind, found: KeyKind },
    #[error("{at}: unknown operator `{name}`")]
    UnknownOperator { at: At, name: String },
    #[error("{at}: task `{task}` is primitive and can't have methods")]
    MethodOnPrimitive { at: At, task: String },
    #[error("too many {0} records")]
    TooMany(&'static str),
}

/// Every problem found while building a [`Library`], in record order (tasks, then methods).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadReport {
    pub errors: Vec<LoadError>,
}

impl fmt::Display for LoadReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTN library: {} error(s)", self.errors.len())?;
        for e in &self.errors {
            write!(f, "\n  {e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for LoadReport {}

/// Resolves names for one record, collecting errors.
struct Resolver<'a> {
    keys: &'a StateKeys,
    errors: &'a mut Vec<LoadError>,
    at: At,
}

impl Resolver<'_> {
    fn key(&mut self, name: &str, expected: KeyKind) -> Option<StateKey> {
        match self.keys.get(name) {
            Some(k) if k.kind() == expected => Some(k),
            Some(k) => {
                let (at, name, found) = (self.at.clone(), name.to_owned(), k.kind());
                self.errors.push(LoadError::KeyKind { at, name, expected, found });
                None
            }
            None => {
                self.errors.push(LoadError::UnknownKey { at: self.at.clone(), name: name.to_owned() });
                None
            }
        }
    }

    fn bit(&mut self, name: &str) -> Option<BitKey> {
        match self.key(name, KeyKind::Bit)? {
            StateKey::Bit(k) => Some(k),
            _ => None,
        }
    }

    fn int(&mut self, name: &str) -> Option<IntKey> {
        match self.key(name, KeyKind::Int)? {
            StateKey::Int(k) => Some(k),
            _ => None,
        }
    }

    fn reference(&mut self, name: &str) -> Option<RefKey> {
        match self.key(name, KeyKind::Ref)? {
            StateKey::Ref(k) => Some(k),
            _ => None,
        }
    }

    fn predicates(&mut self, defs: &[PredicateDef]) -> Vec<Predicate> {
        defs.iter()
            .filter_map(|d| match d {
                PredicateDef::Bit { key, value } => self.bit(key).map(|k| Predicate::Bit(k, *value)),
                PredicateDef::Int { key, op, value } => self.int(key).map(|k| Predicate::Int(k, *op, *value)),
                PredicateDef::Ref { key, set } => self.reference(key).map(|k| Predicate::Ref(k, *set)),
                PredicateDef::SameRef { a, b } => {
                    let (a, b) = (self.reference(a), self.reference(b));
                    Some(Predicate::SameRef(a?, b?))
                }
            })
            .collect()
    }

    fn effects(&mut self, defs: &[EffectDef]) -> Vec<Effect> {
        defs.iter()
            .filter_map(|d| match d {
                EffectDef::SetBit { key, value } => self.bit(key).map(|k| Effect::SetBit(k, *value)),
                EffectDef::SetInt { key, value } => self.int(key).map(|k| Effect::SetInt(k, *value)),
                EffectDef::AddInt { key, by } => self.int(key).map(|k| Effect::AddInt(k, *by)),
                EffectDef::CopyRef { from, to } => {
                    let (from, to) = (self.reference(from), self.reference(to));
                    Some(Effect::CopyRef { from: from?, to: to? })
                }
                EffectDef::ClearRef { key } => self.reference(key).map(Effect::ClearRef),
            })
            .collect()
    }
}

impl Library {
    /// Builds the library from the live [`TASK_KIND`] and [`METHOD_KIND`] records of `db`.
    pub fn build(db: &RecordDb, keys: &StateKeys, operators: &Operators) -> Result<Self, LoadReport> {
        Self::from_defs(
            db.iter::<TaskDef>().map(|(id, d)| (id.clone(), d)),
            db.iter::<MethodDef>().map(|(id, d)| (id.clone(), d)),
            keys,
            operators,
        )
    }

    /// Builds the library from task and method definitions (ids are the names). Task ids follow
    /// `tasks` order; method ids follow `methods` order. Collects every error before failing.
    pub fn from_defs<'d>(
        tasks: impl IntoIterator<Item = (RecordId, &'d TaskDef)>,
        methods: impl IntoIterator<Item = (RecordId, &'d MethodDef)>,
        keys: &StateKeys,
        operators: &Operators,
    ) -> Result<Self, LoadReport> {
        let mut errors = Vec::new();
        let tasks: Vec<_> = tasks.into_iter().collect();
        let methods: Vec<_> = methods.into_iter().collect();
        if u32::try_from(tasks.len()).is_err() {
            return Err(LoadReport { errors: vec![LoadError::TooMany(TASK_KIND)] });
        }
        if u32::try_from(methods.len()).is_err() {
            return Err(LoadReport { errors: vec![LoadError::TooMany(METHOD_KIND)] });
        }
        let task_ids: HashMap<RecordId, TaskId> =
            (0u32..).zip(&tasks).map(|(i, (id, _))| (id.clone(), TaskId(i))).collect();

        let mut lib_tasks = Vec::with_capacity(tasks.len());
        for (id, def) in &tasks {
            let mut r = Resolver { keys, errors: &mut errors, at: At { kind: TASK_KIND, id: id.clone() } };
            lib_tasks.push(match def {
                TaskDef::Compound => Task::Compound(Vec::new()),
                TaskDef::Primitive { operator, pre, effects } => {
                    let op = operators.get(operator);
                    if op.is_none() {
                        r.errors.push(LoadError::UnknownOperator { at: r.at.clone(), name: operator.clone() });
                    }
                    let pre = r.predicates(pre);
                    let effects = r.effects(effects);
                    Task::Primitive(Primitive { operator: op.unwrap_or(OperatorId(0)), pre, effects })
                }
            });
        }

        let mut lib_methods = Vec::with_capacity(methods.len());
        for (id, def) in &methods {
            let at = At { kind: METHOD_KIND, id: id.clone() };
            let task = task_ids.get(&RecordId::new(&def.task)).copied();
            match task.map(|t| &lib_tasks[t.0 as usize]) {
                None => errors.push(LoadError::UnknownTask { at: at.clone(), name: def.task.clone() }),
                Some(Task::Primitive(_)) => {
                    errors.push(LoadError::MethodOnPrimitive { at: at.clone(), task: def.task.clone() });
                }
                Some(Task::Compound(_)) => {}
            }
            let mut subtasks = Vec::with_capacity(def.subtasks.len());
            for s in &def.subtasks {
                match s {
                    SubtaskDef::Pause => subtasks.push(Subtask::Pause),
                    SubtaskDef::Task(name) => match task_ids.get(&RecordId::new(name)) {
                        Some(&t) => subtasks.push(Subtask::Task(t)),
                        None => errors.push(LoadError::UnknownTask { at: at.clone(), name: name.clone() }),
                    },
                }
            }
            let mut r = Resolver { keys, errors: &mut errors, at };
            let pre = r.predicates(&def.pre);
            lib_methods.push(Method {
                task: task.unwrap_or(TaskId(0)),
                priority: def.priority,
                rank: 0,
                pre,
                subtasks,
            });
        }
        if !errors.is_empty() {
            return Err(LoadReport { errors });
        }

        let method_names: Vec<RecordId> = methods.iter().map(|(id, _)| id.clone()).collect();
        let mut order: Vec<u32> = (0..).take(lib_methods.len()).collect();
        // Priority high first, then record id: independent of load order.
        order.sort_by(|&a, &b| {
            let (ma, mb) = (&lib_methods[a as usize], &lib_methods[b as usize]);
            mb.priority.cmp(&ma.priority).then_with(|| method_names[a as usize].cmp(&method_names[b as usize]))
        });
        for m in order {
            let task = lib_methods[m as usize].task;
            if let Task::Compound(list) = &mut lib_tasks[task.0 as usize] {
                let Ok(rank) = u16::try_from(list.len()) else {
                    return Err(LoadReport { errors: vec![LoadError::TooMany(METHOD_KIND)] });
                };
                lib_methods[m as usize].rank = rank;
                list.push(MethodId(m));
            }
        }
        let method_ids = (0u32..).zip(&method_names).map(|(i, id)| (id.clone(), MethodId(i))).collect();
        Ok(Self {
            tasks: lib_tasks,
            task_names: tasks.into_iter().map(|(id, _)| id).collect(),
            task_ids,
            methods: lib_methods,
            method_names,
            method_ids,
            keys: keys.clone(),
            operators: operators.clone(),
        })
    }

    pub fn task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(id.0 as usize)
    }

    pub fn method(&self, id: MethodId) -> Option<&Method> {
        self.methods.get(id.0 as usize)
    }

    /// The task named `name` (any spelling).
    pub fn task_id(&self, name: &str) -> Option<TaskId> {
        self.task_ids.get(&RecordId::new(name)).copied()
    }

    pub fn method_id(&self, name: &str) -> Option<MethodId> {
        self.method_ids.get(&RecordId::new(name)).copied()
    }

    pub fn task_name(&self, id: TaskId) -> &str {
        self.task_names.get(id.0 as usize).map_or("?", RecordId::as_str)
    }

    pub fn method_name(&self, id: MethodId) -> &str {
        self.method_names.get(id.0 as usize).map_or("?", RecordId::as_str)
    }

    pub fn keys(&self) -> &StateKeys {
        &self.keys
    }

    pub fn operators(&self) -> &Operators {
        &self.operators
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn method_count(&self) -> usize {
        self.methods.len()
    }

    /// Whether primitive `task`'s preconditions hold on `ws` (the running operator's per-step
    /// monitoring check, ai.md "Behaviours"); `false` for a compound or unknown task.
    pub fn holds(&self, task: TaskId, ws: &WorldState) -> bool {
        match self.task(task) {
            Some(Task::Primitive(p)) => p.pre.iter().all(|c| c.eval(ws)),
            _ => false,
        }
    }

    fn methods_of(&self, task: TaskId) -> &[MethodId] {
        match self.task(task) {
            Some(Task::Compound(list)) => list,
            _ => &[],
        }
    }
}

impl TraceNames for Library {
    fn task(&self, out: &mut String, id: TaskId) {
        out.push_str(self.task_name(id));
    }
    fn method(&self, out: &mut String, id: MethodId) {
        out.push_str(self.method_name(id));
    }
    fn operator(&self, out: &mut String, id: OperatorId) {
        out.push_str(self.operators.name(id));
    }
}

// --- Planning --------------------------------------------------------------------------------

/// Where a paused decomposition continues: a committed method and its next subtask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Resume {
    pub method: MethodId,
    pub next: u16,
}

/// A plan: the primitive steps to run, how they were chosen, and what they read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposition {
    pub root: TaskId,
    /// This segment's steps (after [`resume`], only the newly planned ones).
    pub steps: Vec<PlanStep>,
    /// The chosen methods, root first, accumulated across resumes.
    pub methods: Vec<MethodChoice>,
    /// The method traversal record, parallel to `methods`.
    pub mtr: Vec<u16>,
    /// Every key a precondition read while searching, rejected branches included (a change to
    /// any of them may change the outcome: re-validate).
    pub reads: KeySet,
    /// Committed methods still to finish, outermost first; empty unless paused.
    pub rest: Vec<Resume>,
}

impl Decomposition {
    /// Stopped at a pause marker; [`resume`] plans the rest.
    pub fn is_partial(&self) -> bool {
        !self.rest.is_empty()
    }

    /// Whether this plan's MTR beats `other`'s ([`mtr_better`]).
    pub fn beats(&self, other: &Self) -> bool {
        mtr_better(&self.mtr, &other.mtr)
    }
}

/// Whether MTR `new` beats `old`: at the first position where they differ within their common
/// prefix, `new` chose a higher-priority (lower-ranked) method. Equal on the common prefix (same
/// plan, or one stopped at a pause the other passed) is not better.
pub fn mtr_better(new: &[u16], old: &[u16]) -> bool {
    new.iter().zip(old).find(|(a, b)| a != b).is_some_and(|(a, b)| a < b)
}

/// Why no plan was found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum NoPlan {
    #[error("no method of the root task applies")]
    Failed,
    #[error("the search hit SEARCH_TRIES")]
    Limit,
    #[error("the root task is not in the library")]
    UnknownTask,
}

/// What [`replan`] decided.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Replan {
    /// A higher-priority MTR: run this plan instead.
    Replace(Decomposition),
    /// Equal or lower MTR: keep the running plan.
    KeepNotBetter,
    /// No plan at all: keep the running plan (its own monitoring decides).
    KeepNoPlan(NoPlan),
}

/// Plans `root` on a copy of `ws` (ai.md "Behaviours"). Writes the search into `trace` when
/// given: `rejected` appended in the order tried, `methods`, `plan` and `mtr` set to the result,
/// `outcome` [`PlanOutcome::Planned`] or [`PlanOutcome::NoPlan`]. Same result with `None`.
pub fn decompose(
    ws: &WorldState,
    root: TaskId,
    lib: &Library,
    mut trace: Option<&mut TraceEntry>,
) -> Result<Decomposition, NoPlan> {
    let result = Search::new(ws, lib, None, trace.as_deref_mut()).plan_root(root).map_err(NoPlan::from);
    if let Some(e) = trace {
        match &result {
            Ok(d) => write_plan(e, d, PlanOutcome::Planned),
            Err(_) => write_none(e, PlanOutcome::NoPlan),
        }
    }
    result
}

/// Plans the rest of a paused `plan` on a copy of `ws`: the committed methods continue after
/// their pause markers (no alternatives: they already ran in part). The result's `steps` are the
/// new segment; `methods`, `mtr` and `reads` accumulate. A plan that isn't partial yields an
/// empty, finished plan. Trace outcome [`PlanOutcome::Continued`] or [`PlanOutcome::NoPlan`].
pub fn resume(
    ws: &WorldState,
    plan: &Decomposition,
    lib: &Library,
    mut trace: Option<&mut TraceEntry>,
) -> Result<Decomposition, NoPlan> {
    let result = Search::new(ws, lib, None, trace.as_deref_mut()).plan_rest(plan).map_err(NoPlan::from);
    if let Some(e) = trace {
        match &result {
            Ok(d) => write_plan(e, d, PlanOutcome::Continued),
            Err(_) => write_none(e, PlanOutcome::NoPlan),
        }
    }
    result
}

/// Plans `root` again while `running` runs, and adopts the new plan only when its MTR beats the
/// running one's (ai.md "Behaviours"; Fluid HTN's replan rule). The search stops as soon as no
/// better plan is possible ([`RejectReason::NotBetter`]). In the trace, `kept_*` outcomes leave
/// `methods`, `plan` and `mtr` describing `running`.
pub fn replan(
    ws: &WorldState,
    root: TaskId,
    lib: &Library,
    running: &Decomposition,
    mut trace: Option<&mut TraceEntry>,
) -> Replan {
    let result = Search::new(ws, lib, Some(running.mtr.as_slice()), trace.as_deref_mut()).plan_root(root);
    let (decision, outcome) = match result {
        Ok(d) if mtr_better(&d.mtr, &running.mtr) => (Replan::Replace(d), PlanOutcome::Replaced),
        Ok(_) | Err(Stop::NotBetter) => (Replan::KeepNotBetter, PlanOutcome::KeptNotBetter),
        Err(Stop::No(e)) => (Replan::KeepNoPlan(e), PlanOutcome::KeptNoPlan),
    };
    if let Some(e) = trace {
        match &decision {
            Replan::Replace(d) => write_plan(e, d, outcome),
            Replan::KeepNotBetter | Replan::KeepNoPlan(_) => write_plan(e, running, outcome),
        }
    }
    decision
}

fn write_plan(e: &mut TraceEntry, d: &Decomposition, outcome: PlanOutcome) {
    e.methods.clear();
    e.methods.extend_from_slice(&d.methods);
    e.mtr.clear();
    e.mtr.extend_from_slice(&d.mtr);
    e.plan.steps.clear();
    e.plan.steps.extend_from_slice(&d.steps);
    e.plan.cursor = 0;
    e.plan.partial = d.is_partial();
    e.outcome = Some(outcome);
}

fn write_none(e: &mut TraceEntry, outcome: PlanOutcome) {
    e.methods.clear();
    e.mtr.clear();
    e.plan.steps.clear();
    e.plan.cursor = 0;
    e.plan.partial = false;
    e.outcome = Some(outcome);
}

/// Why a search stopped without a plan.
#[derive(Debug)]
enum Stop {
    No(NoPlan),
    /// Replan only: no plan from here can beat the running one.
    NotBetter,
}

impl From<Stop> for NoPlan {
    fn from(stop: Stop) -> Self {
        match stop {
            Stop::No(e) => e,
            // Only a replan compares MTRs.
            Stop::NotBetter => NoPlan::Failed,
        }
    }
}

/// The value an effect overwrote, to undo it on backtrack.
#[derive(Clone, Copy, Debug)]
enum Undo {
    Bit(BitKey, bool),
    Int(IntKey, i32),
    Ref(RefKey, Option<Target>),
}

/// Lengths to restore when a method is rejected.
#[derive(Clone, Copy, Debug)]
struct Marks {
    journal: usize,
    steps: usize,
    methods: usize,
}

/// A method being decomposed.
#[derive(Clone, Copy, Debug)]
struct Frame {
    method: MethodId,
    /// Next subtask index.
    next: u16,
    /// Resumed after a pause: no alternatives.
    committed: bool,
    marks: Marks,
}

/// One decomposition: the working state, the method stack and the plan so far.
struct Search<'a, 't> {
    lib: &'a Library,
    ws: WorldState,
    journal: Vec<Undo>,
    stack: Vec<Frame>,
    steps: Vec<PlanStep>,
    methods: Vec<MethodChoice>,
    mtr: Vec<u16>,
    reads: KeySet,
    /// The running plan's MTR (replan only).
    against: Option<&'a [u16]>,
    tries: u32,
    trace: Option<&'t mut TraceEntry>,
}

impl<'a, 't> Search<'a, 't> {
    fn new(ws: &WorldState, lib: &'a Library, against: Option<&'a [u16]>, trace: Option<&'t mut TraceEntry>) -> Self {
        Self {
            lib,
            ws: ws.clone(),
            journal: Vec::new(),
            stack: Vec::new(),
            steps: Vec::new(),
            methods: Vec::new(),
            mtr: Vec::new(),
            reads: KeySet::new(),
            against,
            tries: 0,
            trace,
        }
    }

    fn plan_root(mut self, root: TaskId) -> Result<Decomposition, Stop> {
        let lib = self.lib;
        match lib.task(root) {
            None => return Err(Stop::No(NoPlan::UnknownTask)),
            Some(Task::Primitive(p)) => {
                if self.holds(&p.pre).is_some() {
                    return Err(Stop::No(NoPlan::Failed));
                }
                self.apply(&p.effects);
                self.steps.push(PlanStep { task: root, operator: p.operator });
            }
            Some(Task::Compound(_)) => {
                if !self.expand(root, 0)? {
                    return Err(Stop::No(NoPlan::Failed));
                }
                self.run()?;
            }
        }
        Ok(self.finish(root))
    }

    fn plan_rest(mut self, plan: &Decomposition) -> Result<Decomposition, Stop> {
        self.methods.clone_from(&plan.methods);
        self.mtr.clone_from(&plan.mtr);
        self.reads.clone_from(&plan.reads);
        let marks = self.marks();
        self.stack.extend(plan.rest.iter().map(|r| Frame { method: r.method, next: r.next, committed: true, marks }));
        self.run()?;
        Ok(self.finish(plan.root))
    }

    fn finish(self, root: TaskId) -> Decomposition {
        let rest = self.stack.iter().map(|f| Resume { method: f.method, next: f.next }).collect();
        Decomposition { root, steps: self.steps, methods: self.methods, mtr: self.mtr, reads: self.reads, rest }
    }

    fn marks(&self) -> Marks {
        Marks { journal: self.journal.len(), steps: self.steps.len(), methods: self.methods.len() }
    }

    /// Processes subtasks until the stack empties (done) or a pause stops it (the stack is the
    /// rest).
    fn run(&mut self) -> Result<(), Stop> {
        let lib = self.lib;
        while let Some(top) = self.stack.last_mut() {
            let Some(method) = lib.method(top.method) else {
                return Err(Stop::No(NoPlan::UnknownTask));
            };
            let Some(&sub) = method.subtasks.get(usize::from(top.next)) else {
                self.stack.pop();
                continue;
            };
            top.next += 1;
            match sub {
                // Nothing planned yet in this segment: there is nothing to run first, so go on.
                Subtask::Pause if self.steps.is_empty() => {}
                Subtask::Pause => return Ok(()),
                Subtask::Task(t) => match lib.task(t) {
                    Some(Task::Primitive(p)) => {
                        if self.holds(&p.pre).is_none() {
                            self.apply(&p.effects);
                            self.steps.push(PlanStep { task: t, operator: p.operator });
                        } else {
                            self.fail(RejectReason::SubtaskFailed(t))?;
                        }
                    }
                    Some(Task::Compound(_)) => {
                        if !self.expand(t, 0)? {
                            self.fail(RejectReason::SubtaskFailed(t))?;
                        }
                    }
                    None => self.fail(RejectReason::SubtaskFailed(t))?,
                },
            }
        }
        Ok(())
    }

    /// Pushes the first applicable method of compound `task` from rank `from`; `false` if none.
    fn expand(&mut self, task: TaskId, from: usize) -> Result<bool, Stop> {
        let lib = self.lib;
        let list = lib.methods_of(task);
        if from >= list.len() {
            return Ok(false);
        }
        if self.stack.len() >= SEARCH_DEPTH {
            self.reject(task, list[from], RejectReason::Limit);
            return Ok(false);
        }
        for (rank, &mid) in list.iter().enumerate().skip(from) {
            self.tries += 1;
            if self.tries > SEARCH_TRIES {
                self.reject(task, mid, RejectReason::Limit);
                return Err(Stop::No(NoPlan::Limit));
            }
            if let Some(old) = self.against {
                let layer = self.mtr.len();
                if layer < old.len() && rank > usize::from(old[layer]) && self.mtr[..] == old[..layer] {
                    self.reject(task, mid, RejectReason::NotBetter);
                    return Err(Stop::NotBetter);
                }
            }
            let Some(method) = lib.method(mid) else { continue };
            if let Some(i) = self.holds(&method.pre) {
                self.reject(task, mid, RejectReason::Precondition(i));
                continue;
            }
            let marks = self.marks();
            self.stack.push(Frame { method: mid, next: 0, committed: false, marks });
            self.methods.push(MethodChoice { task, method: mid });
            self.mtr.push(method.rank);
            return Ok(true);
        }
        Ok(false)
    }

    /// Rejects the method on top of the stack for `reason`, undoes it and tries its task's next
    /// method, failing upward until one applies (`Ok`) or the root runs out (`Err`).
    fn fail(&mut self, mut reason: RejectReason) -> Result<(), Stop> {
        let lib = self.lib;
        loop {
            let Some(frame) = self.stack.pop() else {
                return Err(Stop::No(NoPlan::Failed));
            };
            let Some(method) = lib.method(frame.method) else {
                return Err(Stop::No(NoPlan::UnknownTask));
            };
            let task = method.task;
            self.reject(task, frame.method, reason);
            self.undo(frame.marks);
            if frame.committed {
                return Err(Stop::No(NoPlan::Failed));
            }
            if self.expand(task, usize::from(method.rank) + 1)? {
                return Ok(());
            }
            reason = RejectReason::SubtaskFailed(task);
        }
    }

    /// The index of the first false precondition (reading keys up to it), or `None`.
    fn holds(&mut self, pre: &[Predicate]) -> Option<u16> {
        for (i, c) in (0u16..).zip(pre) {
            c.read(&mut self.reads);
            if !c.eval(&self.ws) {
                return Some(i);
            }
        }
        None
    }

    fn apply(&mut self, effects: &[Effect]) {
        for e in effects {
            match *e {
                Effect::SetBit(k, v) => {
                    self.journal.push(Undo::Bit(k, self.ws.bit(k)));
                    self.ws.set_bit(k, v);
                }
                Effect::SetInt(k, v) => {
                    self.journal.push(Undo::Int(k, self.ws.int(k)));
                    self.ws.set_int(k, v);
                }
                Effect::AddInt(k, by) => {
                    let old = self.ws.int(k);
                    self.journal.push(Undo::Int(k, old));
                    self.ws.set_int(k, old.saturating_add(by));
                }
                Effect::CopyRef { from, to } => {
                    self.journal.push(Undo::Ref(to, self.ws.target(to)));
                    let t = self.ws.target(from);
                    self.ws.set_target(to, t);
                }
                Effect::ClearRef(k) => {
                    self.journal.push(Undo::Ref(k, self.ws.target(k)));
                    self.ws.set_target(k, None);
                }
            }
        }
    }

    fn undo(&mut self, marks: Marks) {
        while self.journal.len() > marks.journal {
            match self.journal.pop() {
                Some(Undo::Bit(k, v)) => self.ws.set_bit(k, v),
                Some(Undo::Int(k, v)) => self.ws.set_int(k, v),
                Some(Undo::Ref(k, v)) => self.ws.set_target(k, v),
                None => break,
            }
        }
        self.steps.truncate(marks.steps);
        self.methods.truncate(marks.methods);
        self.mtr.truncate(marks.methods);
    }

    fn reject(&mut self, task: TaskId, method: MethodId, reason: RejectReason) {
        if let Some(e) = self.trace.as_deref_mut() {
            e.rejected.push(Rejection { task, method, reason });
        }
    }
}
