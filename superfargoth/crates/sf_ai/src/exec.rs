//! Move operator executors (`MoveTo`, `Wait`, `Face`) and the HTN [`PlanRunner`] (`docs/specs/ai.md`
//! "Executors", slice AI6).
//!
//! Each primitive HTN task binds to a small hand-written state machine stepped on the fixed tick.
//! [`PlanRunner`] monitors the active step's preconditions (`Library::holds`) and dirty world-state
//! keys (`Decomposition::reads`), resumes partial decompositions across [`SubtaskDef::Pause`](crate::htn::SubtaskDef::Pause)
//! markers (`htn::resume`), and emits a Bevy-free [`ExecOutput`] that `sf_ai_bevy` translates into
//! `NavCorridor` goals and `MoveIntent`.

use std::collections::{BTreeMap, VecDeque};
use std::f32::consts::{PI, TAU};

use glam::Vec2;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;
use sf_world::Location;

use crate::htn::{Decomposition, Library, Operators, Replan, decompose, replan, resume};
use crate::ids::{OperatorId, TaskId};
use crate::state::{KeySet, StateKey, WorldState};
use crate::trace::{OperatorState, OperatorStatus, PathFailure, PathState, PlanOutcome, TraceEntry, Trigger};

/// Built-in operator name for `MoveTo`.
pub const OP_MOVE_TO: &str = "move_to";
/// Built-in operator name for `Wait`.
pub const OP_WAIT: &str = "wait";
/// Built-in operator name for `Face`.
pub const OP_FACE: &str = "face";

// tunables: const (degenerate zero-length planar vector epsilon)
const PLANAR_EPS_SQ: f32 = 1e-8;
// tunables: const (sub-microsecond timer epsilon for fixed-tick countdown)
const TIMER_EPS_S: f32 = 1e-6;

/// Ids of the three move operators registered by [`register_move_operators`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MoveOperatorIds {
    pub move_to: OperatorId,
    pub wait: OperatorId,
    pub face: OperatorId,
}

/// Registers [`OP_MOVE_TO`], [`OP_WAIT`] and [`OP_FACE`] in `ops` and returns their ids.
pub fn register_move_operators(ops: &mut Operators) -> MoveOperatorIds {
    MoveOperatorIds {
        move_to: ops.add(OP_MOVE_TO),
        wait: ops.add(OP_WAIT),
        face: ops.add(OP_FACE),
    }
}

/// Locomotion gait requested by an AI executor (`sf_ai_bevy` maps this to `sf_controller::Gait`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecGait {
    Sneak,
    #[default]
    Walk,
    Run,
    Sprint,
}

/// What a [`OperatorKind::Face`] operator turns the actor toward.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FaceTarget {
    /// Target yaw in radians about +Y (0 facing +Z).
    Yaw(f32),
    /// Target world position (`atan2(dx, dz)` on the XZ plane).
    Point(WorldPos),
}

/// Parameters of a primitive move operator (`MoveTo`, `Wait`, `Face`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatorKind {
    /// Follow a navmesh corridor to `goal` at `(gait, speed)`; fails on `NoPath` or after
    /// [`ExecConfig::stuck_timeout_s`] without [`ExecConfig::progress_min_m`] progress.
    MoveTo {
        goal: Location,
        gait: ExecGait,
        speed: f32,
    },
    /// Stand idle for `duration_s` seconds, optionally holding `facing` and `idle_variant`.
    Wait {
        duration_s: f32,
        facing: Option<f32>,
        idle_variant: Option<u8>,
    },
    /// Stand in place and turn toward `target` until within `tolerance_rad` radians.
    Face {
        target: FaceTarget,
        tolerance_rad: f32,
    },
}

/// Alias for [`OperatorKind`].
pub type MoveAction = OperatorKind;

/// Tunable thresholds for move executors (bound from `ai.exec.*` in `sf_ai_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecConfig {
    /// Seconds without planar progress toward the goal before `MoveTo` fails with
    /// [`PathFailure::NoProgress`] (`ai.md` "Executors": 3 s).
    pub stuck_timeout_s: f32,
    /// Minimum planar XZ distance closure toward the goal that resets the no-progress timer, meters.
    pub progress_min_m: f32,
    /// Default angular tolerance for `Face` when not overridden, radians.
    pub face_tolerance_rad: f32,
    /// Planar XZ distance within which a `MoveTo` in the same worldspace is already arrived, meters.
    pub arrive_dist_m: f32,
}

impl Default for ExecConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.exec.stuck_timeout_s in sf_ai_bevy)
            stuck_timeout_s: 3.0,
            // tunables: const (bound from ai.exec.progress_min_m in sf_ai_bevy)
            progress_min_m: 0.25,
            // tunables: const (bound from ai.exec.face_tolerance_rad in sf_ai_bevy)
            face_tolerance_rad: 0.05,
            // tunables: const (bound from ai.exec.arrive_dist_m in sf_ai_bevy)
            arrive_dist_m: 0.15,
        }
    }
}

/// State of the actor's navigation corridor reported to the executor each fixed tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavMoveState {
    #[default]
    Idle,
    Pending,
    Following,
    Arrived,
    Failed,
}

/// Per-tick kinematic and navigation context fed into [`OperatorExec::step`] and [`PlanRunner`].
#[derive(Clone, Debug, PartialEq)]
pub struct ExecContext {
    pub location: Location,
    pub yaw: f32,
    pub dt: f32,
    pub nav: NavMoveState,
    /// Whether this tick is one of the actor's decide ticks (`ai.md` "Tiers and budget", `AI9g`:
    /// [`crate::decide_due`] on its tier's staggered period; `true` for an actor without a tier).
    /// [`PlanRunner::step_htn`] runs its periodic planning (a fresh decomposition, a resume past a
    /// `Pause`, an explicit [`PlanRunner::request_replan`]) only on these ticks and holds
    /// ([`NavCommand::Stop`]) in between; events (a dirty read key, a failed precondition, the
    /// replan after an operator failure) and the operators themselves run on every tick.
    pub decide: bool,
}

/// Navigation command emitted by an executor step for the Bevy shell (`NavCorridor`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum NavCommand {
    #[default]
    None,
    MoveTo {
        goal: Location,
        gait: ExecGait,
        speed: f32,
    },
    Stop,
}

/// Output of one fixed-tick executor / plan-runner step.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExecOutput {
    pub nav: NavCommand,
    pub facing: Option<f32>,
    pub idle_variant: Option<u8>,
    pub operator: Option<OperatorState>,
    pub path: PathState,
    pub plan_outcome: Option<PlanOutcome>,
}

/// Wraps an angle in radians into `[-π, π)`.
pub fn wrap_angle(a: f32) -> f32 {
    (a + PI).rem_euclid(TAU) - PI
}

/// Yaw in radians (about +Y, 0 facing +Z) from `from` toward `to` on the XZ plane, or `None` when
/// the two points coincide in XZ.
pub fn yaw_toward(from: WorldPos, to: WorldPos) -> Option<f32> {
    let d = from.delta_f32(to);
    let len_sq = d.x * d.x + d.z * d.z;
    (len_sq > PLANAR_EPS_SQ).then(|| d.x.atan2(d.z))
}

fn planar_dist(a: &Location, b: &Location) -> f32 {
    if a.worldspace != b.worldspace {
        return f32::INFINITY;
    }
    let d = a.pos.delta_f32(b.pos);
    Vec2::new(d.x, d.z).length()
}

/// Running state of one primitive operator (`MoveTo`, `Wait`, `Face`).
#[derive(Clone, Debug, PartialEq)]
pub struct OperatorExec {
    operator: OperatorId,
    state: ExecState,
}

#[derive(Clone, Debug, PartialEq)]
struct MoveToState {
    goal: Location,
    gait: ExecGait,
    speed: f32,
    started: bool,
    best_dist: f32,
    no_progress_s: f32,
}

#[derive(Clone, Debug, PartialEq)]
enum ExecState {
    MoveTo(MoveToState),
    Wait {
        remaining_s: f32,
        facing: Option<f32>,
        idle_variant: Option<u8>,
    },
    Face {
        target: FaceTarget,
        tolerance_rad: f32,
    },
}

impl MoveToState {
    fn step(&mut self, op: OperatorId, ctx: &ExecContext, cfg: &ExecConfig) -> ExecOutput {
        let dt = ctx.dt.max(0.0);
        let dist = planar_dist(&ctx.location, &self.goal);
        if !self.started {
            if ctx.nav == NavMoveState::Arrived || dist <= cfg.arrive_dist_m {
                return stop_output(op, OperatorStatus::Succeeded, PathState::None);
            }
            if ctx.nav == NavMoveState::Failed {
                return stop_output(
                    op,
                    OperatorStatus::Failed,
                    PathState::Failed(PathFailure::NoPath),
                );
            }
            self.started = true;
            self.best_dist = dist;
            self.no_progress_s = 0.0;
            return self.running_output(op, ctx.nav);
        }

        match ctx.nav {
            NavMoveState::Failed => stop_output(
                op,
                OperatorStatus::Failed,
                PathState::Failed(PathFailure::NoPath),
            ),
            NavMoveState::Arrived => stop_output(op, OperatorStatus::Succeeded, PathState::None),
            NavMoveState::Idle | NavMoveState::Pending | NavMoveState::Following => {
                if ctx.nav == NavMoveState::Idle && dist <= cfg.arrive_dist_m {
                    return stop_output(op, OperatorStatus::Succeeded, PathState::None);
                }
                if self.best_dist - dist >= cfg.progress_min_m {
                    self.best_dist = dist;
                    self.no_progress_s = 0.0;
                } else {
                    self.no_progress_s += dt;
                }
                if self.no_progress_s + TIMER_EPS_S >= cfg.stuck_timeout_s {
                    return stop_output(
                        op,
                        OperatorStatus::Failed,
                        PathState::Failed(PathFailure::NoProgress),
                    );
                }
                self.running_output(op, ctx.nav)
            }
        }
    }

    fn running_output(&self, op: OperatorId, nav: NavMoveState) -> ExecOutput {
        let path = match nav {
            NavMoveState::Following => PathState::Following,
            _ => PathState::Pending,
        };
        ExecOutput {
            nav: NavCommand::MoveTo {
                goal: self.goal.clone(),
                gait: self.gait,
                speed: self.speed,
            },
            facing: None,
            idle_variant: None,
            operator: Some(OperatorState {
                operator: op,
                status: OperatorStatus::Running,
            }),
            path,
            plan_outcome: None,
        }
    }
}

fn stop_output(operator: OperatorId, status: OperatorStatus, path: PathState) -> ExecOutput {
    ExecOutput {
        nav: NavCommand::Stop,
        facing: None,
        idle_variant: None,
        operator: Some(OperatorState { operator, status }),
        path,
        plan_outcome: None,
    }
}

impl OperatorExec {
    pub fn new(operator: OperatorId, kind: OperatorKind) -> Self {
        let state = match kind {
            OperatorKind::MoveTo { goal, gait, speed } => ExecState::MoveTo(MoveToState {
                goal,
                gait,
                speed,
                started: false,
                best_dist: f32::INFINITY,
                no_progress_s: 0.0,
            }),
            OperatorKind::Wait {
                duration_s,
                facing,
                idle_variant,
            } => ExecState::Wait {
                remaining_s: duration_s.max(0.0),
                facing,
                idle_variant,
            },
            OperatorKind::Face {
                target,
                tolerance_rad,
            } => ExecState::Face {
                target,
                tolerance_rad: tolerance_rad.max(0.0),
            },
        };
        Self { operator, state }
    }

    pub fn operator(&self) -> OperatorId {
        self.operator
    }

    /// Steps this operator by one fixed tick (`ctx.dt`).
    pub fn step(&mut self, ctx: &ExecContext, cfg: &ExecConfig) -> ExecOutput {
        let dt = ctx.dt.max(0.0);
        match &mut self.state {
            ExecState::MoveTo(m) => m.step(self.operator, ctx, cfg),
            ExecState::Wait {
                remaining_s,
                facing,
                idle_variant,
            } => {
                *remaining_s -= dt;
                let status = if *remaining_s <= TIMER_EPS_S {
                    OperatorStatus::Succeeded
                } else {
                    OperatorStatus::Running
                };
                ExecOutput {
                    nav: NavCommand::Stop,
                    facing: *facing,
                    idle_variant: *idle_variant,
                    operator: Some(OperatorState {
                        operator: self.operator,
                        status,
                    }),
                    path: PathState::None,
                    plan_outcome: None,
                }
            }
            ExecState::Face {
                target,
                tolerance_rad,
            } => {
                let desired_yaw = match *target {
                    FaceTarget::Yaw(yaw) => wrap_angle(yaw),
                    FaceTarget::Point(pos) => yaw_toward(ctx.location.pos, pos).unwrap_or(ctx.yaw),
                };
                let tol = if *tolerance_rad > 0.0 {
                    *tolerance_rad
                } else {
                    cfg.face_tolerance_rad
                };
                let diff = wrap_angle(desired_yaw - ctx.yaw).abs();
                let status = if diff <= tol {
                    OperatorStatus::Succeeded
                } else {
                    OperatorStatus::Running
                };
                ExecOutput {
                    nav: NavCommand::Stop,
                    facing: Some(desired_yaw),
                    idle_variant: None,
                    operator: Some(OperatorState {
                        operator: self.operator,
                        status,
                    }),
                    path: PathState::None,
                    plan_outcome: None,
                }
            }
        }
    }
}

/// Resolves a planned primitive step `(task, operator)` on `ws` into its runtime [`OperatorKind`]
/// parameters.
pub trait OperatorResolver {
    fn resolve(&self, task: TaskId, operator: OperatorId, ws: &WorldState) -> Option<OperatorKind>;
}

impl<F> OperatorResolver for F
where
    F: Fn(TaskId, OperatorId, &WorldState) -> Option<OperatorKind>,
{
    fn resolve(&self, task: TaskId, operator: OperatorId, ws: &WorldState) -> Option<OperatorKind> {
        self(task, operator, ws)
    }
}

/// Per-task and per-operator parameter bindings usable as an [`OperatorResolver`] inside ECS
/// components. Task bindings take precedence over operator bindings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OperatorBindings {
    by_task: BTreeMap<TaskId, OperatorKind>,
    by_operator: BTreeMap<OperatorId, OperatorKind>,
}

impl OperatorBindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_task(&mut self, task: TaskId, kind: OperatorKind) -> &mut Self {
        self.by_task.insert(task, kind);
        self
    }

    pub fn bind_operator(&mut self, operator: OperatorId, kind: OperatorKind) -> &mut Self {
        self.by_operator.insert(operator, kind);
        self
    }

    pub fn get_task(&self, task: TaskId) -> Option<&OperatorKind> {
        self.by_task.get(&task)
    }

    pub fn get_operator(&self, operator: OperatorId) -> Option<&OperatorKind> {
        self.by_operator.get(&operator)
    }
}

impl OperatorResolver for OperatorBindings {
    fn resolve(&self, task: TaskId, operator: OperatorId, _ws: &WorldState) -> Option<OperatorKind> {
        self.by_task
            .get(&task)
            .or_else(|| self.by_operator.get(&operator))
            .cloned()
    }
}

/// Steps an HTN [`Decomposition`] (or a direct operator queue), monitoring preconditions and dirty
/// keys, resuming across [`SubtaskDef::Pause`](crate::htn::SubtaskDef::Pause) markers, and recording
/// [`OperatorState`] + [`PathState`] into [`TraceEntry`].
#[derive(Clone, Debug, Default)]
#[allow(clippy::struct_excessive_bools)] // independent runner flags (request, event, completed, failed), not a state enum
pub struct PlanRunner {
    plan: Option<Decomposition>,
    cursor: usize,
    active: Option<OperatorExec>,
    queue: VecDeque<(OperatorId, OperatorKind)>,
    dirty: KeySet,
    replan_requested: bool,
    /// An event since the last planning (`AI9g`: an operator failed): the next planning runs on
    /// any tick, not only on a decide tick. Survives [`Self::clear`] (it is a fact about what
    /// happened, not plan state: a caller that clears and re-requests after a failure still
    /// re-plans at once); planning consumes it.
    event_replan: bool,
    completed: bool,
    failed: bool,
    last_status: Option<OperatorStatus>,
    last_path: PathState,
}

impl PlanRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn plan(&self) -> Option<&Decomposition> {
        self.plan.as_ref()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn active_operator(&self) -> Option<&OperatorExec> {
        self.active.as_ref()
    }

    pub fn is_idle(&self) -> bool {
        self.plan.is_none() && self.active.is_none() && self.queue.is_empty()
    }

    pub fn is_complete(&self) -> bool {
        self.completed
    }

    pub fn is_failed(&self) -> bool {
        self.failed
    }

    pub fn last_status(&self) -> Option<OperatorStatus> {
        self.last_status
    }

    pub fn last_path(&self) -> PathState {
        self.last_path
    }

    /// Marks `key` dirty; if the active plan read `key`, the next [`Self::step_htn`] re-evaluates
    /// with [`replan`].
    pub fn mark_dirty(&mut self, key: StateKey) {
        self.dirty.insert(key);
    }

    /// Marks every key in `keys` dirty.
    pub fn mark_dirty_set(&mut self, keys: &KeySet) {
        self.dirty.union(keys);
    }

    /// Forces a replan (or fresh decomposition if idle/completed/failed) on the next
    /// [`Self::step_htn`] that is a decide tick ([`ExecContext::decide`]): a request is periodic
    /// work, not an event (`AI9g`; its callers hand over a step they chose, or a new root).
    pub fn request_replan(&mut self) {
        self.replan_requested = true;
        self.completed = false;
        self.failed = false;
    }

    /// Clears all running plan, operator and queue state.
    pub fn clear(&mut self) {
        self.plan = None;
        self.cursor = 0;
        self.active = None;
        self.queue.clear();
        self.dirty.clear();
        self.replan_requested = false;
        self.completed = false;
        self.failed = false;
        self.last_status = None;
        self.last_path = PathState::None;
    }

    /// Replaces the active HTN decomposition directly.
    pub fn set_plan(&mut self, plan: Decomposition) {
        self.plan = Some(plan);
        self.cursor = 0;
        self.active = None;
        self.queue.clear();
        self.dirty.clear();
        self.replan_requested = false;
        self.event_replan = false;
        self.completed = false;
        self.failed = false;
    }

    /// Starts a single direct operator, clearing any active plan or queue.
    pub fn run_operator(&mut self, operator: OperatorId, kind: OperatorKind) {
        self.clear();
        self.active = Some(OperatorExec::new(operator, kind));
    }

    /// Appends a direct operator to the runner's queue.
    pub fn queue_operator(&mut self, operator: OperatorId, kind: OperatorKind) {
        self.completed = false;
        self.failed = false;
        self.queue.push_back((operator, kind));
    }

    /// Steps a direct operator or queued operator sequence (without an HTN library).
    pub fn step_direct(
        &mut self,
        ctx: &ExecContext,
        cfg: &ExecConfig,
        trace: Option<&mut TraceEntry>,
    ) -> ExecOutput {
        if self.active.is_none()
            && let Some((op, kind)) = self.queue.pop_front()
        {
            self.active = Some(OperatorExec::new(op, kind));
        }
        let Some(ref mut active) = self.active else {
            return ExecOutput::default();
        };
        let out = active.step(ctx, cfg);
        let status = out.operator.map_or(OperatorStatus::Running, |o| o.status);
        self.last_status = Some(status);
        self.last_path = out.path;
        if let Some(e) = trace {
            e.operator = out.operator;
            e.path = out.path;
        }
        match status {
            OperatorStatus::Running => {}
            OperatorStatus::Succeeded => {
                self.active = None;
                if self.queue.is_empty() {
                    self.completed = true;
                }
            }
            OperatorStatus::Failed => {
                self.active = None;
                self.queue.clear();
                self.failed = true;
            }
        }
        out
    }

    fn decompose_fresh(
        &mut self,
        root: TaskId,
        ws: &WorldState,
        lib: &Library,
        trace: Option<&mut TraceEntry>,
    ) -> Result<PlanOutcome, ExecOutput> {
        self.event_replan = false;
        if let Ok(fresh) = decompose(ws, root, lib, trace) {
            self.plan = Some(fresh);
            self.cursor = 0;
            self.active = None;
            Ok(PlanOutcome::Planned)
        } else {
            self.plan = None;
            self.cursor = 0;
            self.active = None;
            self.failed = true;
            Err(ExecOutput {
                nav: NavCommand::Stop,
                plan_outcome: Some(PlanOutcome::NoPlan),
                ..ExecOutput::default()
            })
        }
    }

    /// Plans as needed before the current step. `decide` gates the periodic work (`AI9g`, see
    /// [`ExecContext::decide`]): steps 2 (resume / next segment) and 3 (fresh decomposition) and an
    /// explicit request wait for a decide tick unless an event is pending, and the actor holds
    /// ([`NavCommand::Stop`], no plan outcome) meanwhile; step 1's dirty read-key hit and step 4's
    /// failed precondition are events and run on any tick.
    fn ensure_plan(
        &mut self,
        root: TaskId,
        ws: &WorldState,
        lib: &Library,
        decide: bool,
        mut trace: Option<&mut TraceEntry>,
    ) -> Result<Option<PlanOutcome>, ExecOutput> {
        let mut planned_outcome: Option<PlanOutcome> = None;
        let periodic_ok = decide || self.event_replan;
        let hold = |plan_outcome| ExecOutput { nav: NavCommand::Stop, plan_outcome, ..ExecOutput::default() };

        // 1. Dirty read-key monitoring (an event: any tick) or an explicit replan request (periodic:
        //    a decide tick) while a plan is running.
        if let Some(ref current) = self.plan {
            let dirty_hit = current.reads.intersects(&self.dirty);
            if dirty_hit || (self.replan_requested && periodic_ok) {
                self.dirty.clear();
                self.replan_requested = false;
                if let Some(ref mut e) = trace {
                    e.trigger = Trigger::Replan;
                }
                match replan(ws, root, lib, current, trace.as_deref_mut()) {
                    Replan::Replace(new_plan) => {
                        self.plan = Some(new_plan);
                        self.cursor = 0;
                        self.active = None;
                        planned_outcome = Some(PlanOutcome::Replaced);
                    }
                    Replan::KeepNotBetter => {
                        planned_outcome = Some(PlanOutcome::KeptNotBetter);
                    }
                    Replan::KeepNoPlan(_) => {
                        planned_outcome = Some(PlanOutcome::KeptNoPlan);
                    }
                }
            }
        }

        // 2. Resume a partial plan when the previous segment's steps have all completed.
        if let Some(ref current) = self.plan
            && self.cursor >= current.steps.len()
        {
            if current.is_partial() {
                if !periodic_ok {
                    // Periodic: the next segment waits for the decide tick; stand meanwhile.
                    return Err(hold(planned_outcome));
                }
                if let Ok(resumed) = resume(ws, current, lib, trace.as_deref_mut()) {
                    self.plan = Some(resumed);
                    self.cursor = 0;
                    self.active = None;
                    planned_outcome = Some(PlanOutcome::Continued);
                } else {
                    if let Some(ref mut e) = trace {
                        e.trigger = Trigger::Replan;
                    }
                    planned_outcome = Some(self.decompose_fresh(root, ws, lib, trace.as_deref_mut())?);
                }
            } else {
                self.plan = None;
                self.cursor = 0;
                self.active = None;
                self.completed = true;
                return Err(ExecOutput {
                    nav: NavCommand::Stop,
                    plan_outcome: planned_outcome,
                    ..ExecOutput::default()
                });
            }
        }

        // 3. Initial decomposition when no plan is active (periodic: a decide tick, or an event).
        if self.plan.is_none() {
            if !periodic_ok {
                return Err(hold(planned_outcome));
            }
            self.dirty.clear();
            self.replan_requested = false;
            self.completed = false;
            self.failed = false;
            planned_outcome = Some(self.decompose_fresh(root, ws, lib, trace.as_deref_mut())?);
        }

        // 4. Per-step precondition monitoring on the current primitive task.
        if let Some(ref current) = self.plan
            && let Some(step) = current.steps.get(self.cursor).copied()
            && !lib.holds(step.task, ws)
        {
            self.active = None;
            if let Some(ref mut e) = trace {
                e.trigger = Trigger::Replan;
            }
            planned_outcome = Some(self.decompose_fresh(root, ws, lib, trace)?);
        }

        if planned_outcome.is_some() {
            self.event_replan = false;
        }
        Ok(planned_outcome)
    }

    /// Steps the runner under HTN `root` against `ws` and `lib`. Planning follows
    /// [`ExecContext::decide`] (periodic work on decide ticks, events on any tick); the active
    /// operator steps every tick.
    #[allow(clippy::too_many_arguments)]
    pub fn step_htn<R: OperatorResolver>(
        &mut self,
        root: TaskId,
        ws: &WorldState,
        lib: &Library,
        resolver: &R,
        ctx: &ExecContext,
        cfg: &ExecConfig,
        mut trace: Option<&mut TraceEntry>,
    ) -> ExecOutput {
        if self.plan.as_ref().is_some_and(|p| p.root != root) {
            self.plan = None;
            self.cursor = 0;
            self.active = None;
            self.dirty.clear();
            self.replan_requested = false;
            self.completed = false;
            self.failed = false;
        }

        if (self.completed || self.failed) && !self.replan_requested && self.dirty.is_empty() {
            return ExecOutput::default();
        }

        let planned_outcome = match self.ensure_plan(root, ws, lib, ctx.decide, trace.as_deref_mut()) {
            Ok(outcome) => outcome,
            Err(early) => return early,
        };

        let Some(ref plan) = self.plan else {
            return ExecOutput {
                plan_outcome: planned_outcome,
                ..ExecOutput::default()
            };
        };
        let Some(step) = plan.steps.get(self.cursor).copied() else {
            self.completed = true;
            self.plan = None;
            return ExecOutput {
                nav: NavCommand::Stop,
                plan_outcome: planned_outcome,
                ..ExecOutput::default()
            };
        };

        if self.active.is_none() {
            let Some(kind) = resolver.resolve(step.task, step.operator, ws) else {
                let fail_state = OperatorState {
                    operator: step.operator,
                    status: OperatorStatus::Failed,
                };
                if let Some(e) = trace {
                    sync_trace_plan(e, plan, self.cursor, planned_outcome);
                    e.operator = Some(fail_state);
                    e.path = PathState::None;
                }
                self.plan = None;
                self.cursor = 0;
                self.failed = true;
                self.event_replan = true;
                self.last_status = Some(OperatorStatus::Failed);
                return ExecOutput {
                    nav: NavCommand::Stop,
                    operator: Some(fail_state),
                    path: PathState::None,
                    plan_outcome: planned_outcome,
                    ..ExecOutput::default()
                };
            };
            self.active = Some(OperatorExec::new(step.operator, kind));
        }

        let step_cursor = self.cursor;
        let mut out = self
            .active
            .as_mut()
            .expect("active operator just initialized")
            .step(ctx, cfg);
        out.plan_outcome = planned_outcome;

        let status = out.operator.map_or(OperatorStatus::Running, |o| o.status);
        self.last_status = Some(status);
        self.last_path = out.path;

        if let Some(e) = trace {
            sync_trace_plan(e, plan, step_cursor, planned_outcome);
            e.operator = out.operator;
            e.path = out.path;
        }

        match status {
            OperatorStatus::Running => {}
            OperatorStatus::Succeeded => {
                self.active = None;
                self.cursor += 1;
                if self.cursor >= plan.steps.len() && !plan.is_partial() {
                    self.plan = None;
                    self.cursor = 0;
                    self.completed = true;
                }
            }
            OperatorStatus::Failed => {
                self.active = None;
                self.plan = None;
                self.cursor = 0;
                self.failed = true;
                self.event_replan = true;
            }
        }

        out
    }
}

fn sync_trace_plan(
    e: &mut TraceEntry,
    plan: &Decomposition,
    cursor: usize,
    outcome: Option<PlanOutcome>,
) {
    if outcome.is_none() {
        e.methods.clear();
        e.methods.extend_from_slice(&plan.methods);
        e.mtr.clear();
        e.mtr.extend_from_slice(&plan.mtr);
        e.plan.steps.clear();
        e.plan.steps.extend_from_slice(&plan.steps);
        e.plan.partial = plan.is_partial();
        e.outcome = None;
    }
    e.plan.cursor = u16::try_from(cursor).unwrap_or(u16::MAX);
}
