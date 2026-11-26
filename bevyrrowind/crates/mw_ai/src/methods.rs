//! Morrowind wander/idle HTN method set (`docs/specs/ai-adapter.md` "Wander and idle", slice MA3)
//! and [`WanderDriver`], replacing the M3 `PathWalkScaffold`.
//!
//! Decomposes the root compound task [`TASK_WANDER`] (`"wander"`) through [`sf_ai::htn::Library`]
//! into exactly one of three methods, executed by [`sf_ai::exec::PlanRunner`]:
//! - [`METHOD_WANDER_IDLE`] when [`STATE_HAS_IDLE`] (an `Idle2..=9` fidget was rolled) ->
//!   [`TASK_DO_IDLE`] (`"wait"` with the fidget's group);
//! - [`METHOD_WANDER_WALK`] when [`STATE_HAS_WALK_GOAL`] -> [`TASK_DO_WALK`] (`"move_to"`);
//! - [`METHOD_WANDER_STAND`] otherwise (plain `Idle`, distance 0, or the pause after a give-up) ->
//!   [`TASK_DO_IDLE`] (`"wait"`, group 0).
//!
//! [`METHOD_WANDER_FACE`] -> [`TASK_DO_FACE`] (`"face"`) runs only when a caller asks for it
//! ([`WanderDriver::face`], HTN mode): no wander roll picks it.
//!
//! The choice between them is `OpenMW`'s `AiWander` (`apps/openmw/mwmechanics/aiwander.cpp` at
//! commit `b8285cc95d19e080fe0532f9f5154b0c3f727735`, `onChooseActionStatePerFrameActions` and
//! `getRandomIdle`; see [`crate::wander`]): roll the idles first, walk when none is picked and the
//! wander distance is non-zero, else stand.
//!
//! Two execution modes share the same HTN vocabulary, world state, and `PlanRunner`:
//! 1. **HTN + `sf_nav` mode ([`WanderDriver::step_htn`])**: for ground movers on a resident
//!    navmesh tile. Emits [`ExecOutput`] (`NavCommand::MoveTo` / `NavCommand::Stop`) consumed by
//!    `sf_nav_bevy::NavCorridor`.
//! 2. **Pathgrid fallback mode ([`WanderDriver::update_fallback`])**: for fliers/swimmers or cells
//!    without a resident navmesh tile (`ai-adapter.md`: "Fallback when a worldspace has no
//!    navmesh: the pathgrid route"; `ai.md`: "Fliers skip the navmesh"). Runs `Wait` through
//!    `PlanRunner` and steps a [`PathFollower`] along the pathgrid polyline while feeding
//!    `NavMoveState` into `PlanRunner`.

use std::sync::{Arc, OnceLock};

use sf_ai::{
    BitKey, ExecConfig, ExecContext, ExecGait, ExecOutput, IntKey, Library, MethodDef, MethodId,
    FaceTarget, MoveOperatorIds, NavMoveState, OP_FACE, OP_MOVE_TO, OP_WAIT,
    OperatorBindings, OperatorKind,
    OperatorStatus, Operators, PlanRunner, StateKeys, TaskDef, TaskId, TraceEntry, WorldState,
    htn::{EffectDef, PredicateDef, SubtaskDef},
    register_move_operators,
};
use sf_controller::{LocomotionIntent, PathFollower, PathMode, PathNode};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_world::{Location, WorldspaceId};

use crate::pathgrid::PathGraph;
use crate::wander::{WanderPlanner, WanderStep};

/// World-state bit key: `true` when the next wander step is an `Idle2..=9` fidget (a plain stand,
/// group 0, leaves it `false` and decomposes through [`METHOD_WANDER_STAND`]).
pub const STATE_HAS_IDLE: &str = "has_idle";
/// World-state bit key: `true` when the next wander step is a walk to a chosen destination.
pub const STATE_HAS_WALK_GOAL: &str = "has_walk_goal";
/// World-state bit key: `true` while a requested turn in place ([`WanderDriver::face`]) is pending.
pub const STATE_HAS_FACE: &str = "has_face";
/// World-state int key: active idle group (`0` for plain stand, `2..=9` for `Idle2..Idle9`).
pub const STATE_IDLE_VARIANT: &str = "idle_variant";

/// Root compound HTN task name for Morrowind `AI_W` wander.
pub const TASK_WANDER: &str = "wander";
/// Primitive HTN task name for standing / playing an idle fidget (`wait` operator).
pub const TASK_DO_IDLE: &str = "do_idle";
/// Primitive HTN task name for walking to a wander destination (`move_to` operator).
pub const TASK_DO_WALK: &str = "do_walk";
/// Primitive HTN task name for turning in place toward a target (`face` operator).
pub const TASK_DO_FACE: &str = "do_face";

/// HTN method selecting [`TASK_DO_IDLE`] when [`STATE_HAS_IDLE`] holds.
pub const METHOD_WANDER_IDLE: &str = "wander_idle";
/// HTN method selecting [`TASK_DO_WALK`] when [`STATE_HAS_WALK_GOAL`] holds.
pub const METHOD_WANDER_WALK: &str = "wander_walk";
/// Fallback HTN method selecting [`TASK_DO_IDLE`] when standing in place.
pub const METHOD_WANDER_STAND: &str = "wander_stand";
/// HTN method selecting [`TASK_DO_FACE`] when [`STATE_HAS_FACE`] holds.
pub const METHOD_WANDER_FACE: &str = "wander_face";

// tunables: const (HTN method priority ordering: face > idle > walk > stand)
const PRIORITY_WANDER_FACE: u16 = 30;
// tunables: const (HTN method priority ordering: face > idle > walk > stand)
const PRIORITY_WANDER_IDLE: u16 = 20;
// tunables: const (HTN method priority ordering: face > idle > walk > stand)
const PRIORITY_WANDER_WALK: u16 = 10;
// tunables: const (HTN method priority ordering: face > idle > walk > stand)
const PRIORITY_WANDER_STAND: u16 = 1;

/// Feel values of [`WanderDriver`] (`MA3`). Plain config: the declared tunables
/// (`mw.ai.wander.*`) and their binding live in `mw_walkers_bevy`, which copies the live values
/// into [`WanderDriver::tuning`] each tick. The stuck timeout and progress threshold of the
/// pathgrid fallback are the executor's (`ai.exec.*`), passed as the [`ExecConfig`] of
/// [`WanderDriver::update_fallback`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WanderTuning {
    /// Pathgrid fallback walk backstop: a walk is given up after this multiple of its nominal
    /// time (route length / walk speed) plus [`Self::walk_slack_s`].
    pub walk_backstop_mult: f32,
    /// Seconds added to the fallback walk backstop.
    pub walk_slack_s: f32,
    /// Planar distance (m) within which a newly rolled walk goal counts as the goal just given
    /// up (rolled once more).
    pub same_goal_m: f32,
    /// Seconds the actor stands after giving up a walk: this plus a uniform
    /// `0..`[`Self::give_up_pause_jitter_s`].
    pub give_up_pause_min_s: f32,
    /// Random extra seconds added to [`Self::give_up_pause_min_s`].
    pub give_up_pause_jitter_s: f32,
}

impl Default for WanderTuning {
    fn default() -> Self {
        Self {
            // tunables: const (bound from mw.ai.wander.walk_backstop_mult in mw_walkers_bevy)
            walk_backstop_mult: 2.0,
            // tunables: const (bound from mw.ai.wander.walk_slack_s in mw_walkers_bevy)
            walk_slack_s: 5.0,
            // tunables: const (bound from mw.ai.wander.same_goal_m in mw_walkers_bevy)
            same_goal_m: 0.5,
            // tunables: const (bound from mw.ai.wander.give_up_pause_min_s in mw_walkers_bevy)
            give_up_pause_min_s: 1.0,
            // tunables: const (bound from mw.ai.wander.give_up_pause_jitter_s in mw_walkers_bevy)
            give_up_pause_jitter_s: 1.0,
        }
    }
}

impl WanderTuning {
    fn give_up_pause(&self, rng: &mut fastrand::Rng) -> f32 {
        self.give_up_pause_min_s + self.give_up_pause_jitter_s * rng.f32()
    }
}

/// The [`ExecConfig`] the pathgrid fallback feeds its [`PlanRunner`]: `cfg` with the runner's own
/// stuck check and arrival radius switched off, since the fallback tracks per-node progress itself
/// (with `cfg`'s stuck timeout and progress threshold) and signals arrival through
/// [`NavMoveState::Arrived`].
fn fallback_exec_cfg(cfg: &ExecConfig) -> ExecConfig {
    ExecConfig {
        // tunables: const (sentinel: the fallback walk runs its own per-node stuck timer)
        stuck_timeout_s: f32::INFINITY,
        // tunables: const (sentinel: the fallback signals completion via NavMoveState::Arrived)
        arrive_dist_m: 0.0,
        ..*cfg
    }
}

/// Typed state-key handles for the Morrowind wander HTN domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WanderKeys {
    pub has_idle: BitKey,
    pub has_walk_goal: BitKey,
    pub has_face: BitKey,
    pub idle_variant: IntKey,
}

/// Compiled HTN [`Library`] and resolved task/method/operator/key handles for Morrowind wander.
#[derive(Clone, Debug)]
pub struct WanderVocabulary {
    pub keys: WanderKeys,
    pub state_keys: StateKeys,
    pub operators: MoveOperatorIds,
    pub library: Library,
    pub root: TaskId,
    pub do_idle: TaskId,
    pub do_walk: TaskId,
    pub do_face: TaskId,
    pub wander_idle: MethodId,
    pub wander_walk: MethodId,
    pub wander_stand: MethodId,
    pub wander_face: MethodId,
}

impl WanderVocabulary {
    /// Builds the Morrowind wander HTN vocabulary and library.
    ///
    /// # Panics
    /// Cannot panic: the built-in task and method definitions reference only keys and operators
    /// registered in the same function.
    #[allow(clippy::too_many_lines)]
    pub fn build() -> Self {
        let mut state_keys = StateKeys::new();
        let has_idle = state_keys
            .add_bit(STATE_HAS_IDLE)
            .expect("unique bit key has_idle");
        let has_walk_goal = state_keys
            .add_bit(STATE_HAS_WALK_GOAL)
            .expect("unique bit key has_walk_goal");
        let has_face = state_keys
            .add_bit(STATE_HAS_FACE)
            .expect("unique bit key has_face");
        let idle_variant = state_keys
            .add_int(STATE_IDLE_VARIANT)
            .expect("unique int key idle_variant");
        let keys = WanderKeys {
            has_idle,
            has_walk_goal,
            has_face,
            idle_variant,
        };

        let mut ops = Operators::new();
        let operators = register_move_operators(&mut ops);

        let tasks = [
            (RecordId::new(TASK_WANDER), TaskDef::Compound),
            (
                RecordId::new(TASK_DO_IDLE),
                TaskDef::Primitive {
                    operator: OP_WAIT.into(),
                    // Shared by the fidget and the plain stand: the methods hold the condition.
                    pre: Vec::new(),
                    effects: vec![EffectDef::SetBit {
                        key: STATE_HAS_IDLE.into(),
                        value: false,
                    }],
                },
            ),
            (
                RecordId::new(TASK_DO_WALK),
                TaskDef::Primitive {
                    operator: OP_MOVE_TO.into(),
                    pre: vec![PredicateDef::Bit {
                        key: STATE_HAS_WALK_GOAL.into(),
                        value: true,
                    }],
                    effects: vec![EffectDef::SetBit {
                        key: STATE_HAS_WALK_GOAL.into(),
                        value: false,
                    }],
                },
            ),
            (
                RecordId::new(TASK_DO_FACE),
                TaskDef::Primitive {
                    operator: OP_FACE.into(),
                    pre: vec![PredicateDef::Bit {
                        key: STATE_HAS_FACE.into(),
                        value: true,
                    }],
                    effects: vec![EffectDef::SetBit {
                        key: STATE_HAS_FACE.into(),
                        value: false,
                    }],
                },
            ),
        ];

        let methods = [
            (
                RecordId::new(METHOD_WANDER_IDLE),
                MethodDef {
                    task: TASK_WANDER.into(),
                    priority: PRIORITY_WANDER_IDLE,
                    pre: vec![PredicateDef::Bit {
                        key: STATE_HAS_IDLE.into(),
                        value: true,
                    }],
                    subtasks: vec![SubtaskDef::Task(TASK_DO_IDLE.into())],
                    cut: false,
                },
            ),
            (
                RecordId::new(METHOD_WANDER_WALK),
                MethodDef {
                    task: TASK_WANDER.into(),
                    priority: PRIORITY_WANDER_WALK,
                    pre: vec![PredicateDef::Bit {
                        key: STATE_HAS_WALK_GOAL.into(),
                        value: true,
                    }],
                    subtasks: vec![SubtaskDef::Task(TASK_DO_WALK.into())],
                    cut: false,
                },
            ),
            (
                RecordId::new(METHOD_WANDER_STAND),
                MethodDef {
                    task: TASK_WANDER.into(),
                    priority: PRIORITY_WANDER_STAND,
                    pre: Vec::new(),
                    subtasks: vec![SubtaskDef::Task(TASK_DO_IDLE.into())],
                    cut: false,
                },
            ),
            (
                RecordId::new(METHOD_WANDER_FACE),
                MethodDef {
                    task: TASK_WANDER.into(),
                    priority: PRIORITY_WANDER_FACE,
                    pre: vec![PredicateDef::Bit {
                        key: STATE_HAS_FACE.into(),
                        value: true,
                    }],
                    subtasks: vec![SubtaskDef::Task(TASK_DO_FACE.into())],
                    cut: false,
                },
            ),
        ];

        let library = Library::from_defs(
            tasks.iter().map(|(id, d)| (id.clone(), d)),
            methods.iter().map(|(id, d)| (id.clone(), d)),
            &state_keys,
            &ops,
        )
        .expect("Morrowind wander HTN library is valid");

        let root = library.task_id(TASK_WANDER).expect("wander task");
        let do_idle = library.task_id(TASK_DO_IDLE).expect("do_idle task");
        let do_walk = library.task_id(TASK_DO_WALK).expect("do_walk task");
        let do_face = library.task_id(TASK_DO_FACE).expect("do_face task");
        let wander_idle = library
            .method_id(METHOD_WANDER_IDLE)
            .expect("wander_idle method");
        let wander_walk = library
            .method_id(METHOD_WANDER_WALK)
            .expect("wander_walk method");
        let wander_stand = library
            .method_id(METHOD_WANDER_STAND)
            .expect("wander_stand method");
        let wander_face = library
            .method_id(METHOD_WANDER_FACE)
            .expect("wander_face method");

        Self {
            keys,
            state_keys,
            operators,
            library,
            root,
            do_idle,
            do_walk,
            do_face,
            wander_idle,
            wander_walk,
            wander_stand,
            wander_face,
        }
    }
}

/// Returns the shared process-wide [`WanderVocabulary`].
pub fn wander_library() -> Arc<WanderVocabulary> {
    static VOCAB: OnceLock<Arc<WanderVocabulary>> = OnceLock::new();
    VOCAB.get_or_init(|| Arc::new(WanderVocabulary::build())).clone()
}

/// One tick of fallback wander output, for the mover and the animation graph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WanderOutput {
    pub intent: LocomotionIntent,
    /// Idle group while standing (0 = plain `Idle`, 2..=9 = `IdleN`).
    pub idle_variant: Option<u8>,
    pub walking: bool,
}

#[derive(Clone, Debug)]
struct FallbackWalk {
    follower: PathFollower,
    left: f32,
    node: usize,
    best: f32,
    still: f32,
}

impl FallbackWalk {
    fn new(nodes: Vec<PathNode>, pos: WorldPos, speed: f32, tuning: &WanderTuning) -> Self {
        let mut length = 0.0;
        let mut prev = pos;
        for n in &nodes {
            length += prev.delta_f32(n.pos).length();
            prev = n.pos;
        }
        Self {
            follower: PathFollower::new(nodes, PathMode::Once, speed),
            left: tuning.walk_backstop_mult * length / speed.max(0.1) + tuning.walk_slack_s,
            node: usize::MAX,
            best: f32::MAX,
            still: 0.0,
        }
    }
}

/// Morrowind `AI_W` wander driver over [`sf_ai::exec::PlanRunner`] (`MA3`).
///
/// It counts no duration (`MA2d4`): the `AI_W` duration is the directive's `Limit` on the actor's
/// stack, the one count `OpenMW` keeps too (`aiwander.cpp` @ 5f3aafb, 233-234 and 378: the
/// package's `mRemainingDuration`); the stack ending it makes the walker stand.
#[derive(Clone, Debug)]
pub struct WanderDriver {
    pub planner: WanderPlanner,
    /// Walk speed, m/s (see [`crate::wander::walk_speed`]).
    pub walk_speed: f32,
    /// Feel values (the caller copies in the live `mw.ai.wander.*` tunables).
    pub tuning: WanderTuning,
    worldspace: WorldspaceId,
    rng: fastrand::Rng,
    /// The goal of the last walk given up as stuck.
    abandoned: Option<WorldPos>,
    current_goal: Option<WorldPos>,
    current_first_target: Option<WorldPos>,
    stuck: u32,
    arrivals: u32,
    /// Requested turns ([`WanderDriver::face`]) that reached their target.
    faces: u32,
    replans: u32,
    /// The fidget (`Idle2..=9`) of the last step, if it was one.
    last_idle: Option<u8>,
    vocab: Arc<WanderVocabulary>,
    state: WorldState,
    bindings: OperatorBindings,
    runner: PlanRunner,
    choosing: bool,
    fallback_walk: Option<FallbackWalk>,
}

impl WanderDriver {
    /// Creates a wander driver using the shared [`wander_library`].
    pub fn new(planner: WanderPlanner, walk_speed: f32, seed: u64) -> Self {
        Self::with_vocab(
            planner,
            walk_speed,
            seed,
            mw_world::ids::exterior(),
            wander_library(),
        )
    }

    /// Creates a wander driver in `worldspace` with an explicit [`WanderVocabulary`].
    pub fn with_vocab(
        planner: WanderPlanner,
        walk_speed: f32,
        seed: u64,
        worldspace: WorldspaceId,
        vocab: Arc<WanderVocabulary>,
    ) -> Self {
        let state = WorldState::new(&vocab.state_keys);
        Self {
            planner,
            walk_speed,
            tuning: WanderTuning::default(),
            worldspace,
            rng: fastrand::Rng::with_seed(seed),
            abandoned: None,
            current_goal: None,
            current_first_target: None,
            stuck: 0,
            arrivals: 0,
            faces: 0,
            replans: 0,
            last_idle: None,
            vocab,
            state,
            bindings: OperatorBindings::new(),
            runner: PlanRunner::new(),
            choosing: true,
            fallback_walk: None,
        }
    }

    pub fn vocab(&self) -> &Arc<WanderVocabulary> {
        &self.vocab
    }

    pub fn state(&self) -> &WorldState {
        &self.state
    }

    pub fn bindings(&self) -> &OperatorBindings {
        &self.bindings
    }

    pub fn runner(&self) -> &PlanRunner {
        &self.runner
    }

    /// Walks given up for lack of progress so far.
    pub fn stuck_count(&self) -> u32 {
        self.stuck
    }

    /// Walks resumed around a blocked edge ([`Self::replan_around`]).
    pub fn replan_count(&self) -> u32 {
        self.replans
    }

    /// The goal of the last walk given up as stuck, if any.
    pub fn abandoned(&self) -> Option<WorldPos> {
        self.abandoned
    }

    /// The destination of the currently active walk, if any.
    pub fn current_goal(&self) -> Option<WorldPos> {
        self.current_goal
    }

    /// Walks that reached their destination so far (either mode).
    pub fn arrival_count(&self) -> u32 {
        self.arrivals
    }

    /// Requested turns ([`Self::face`]) that reached their target so far.
    pub fn face_count(&self) -> u32 {
        self.faces
    }

    /// Turns in place toward `target` (within `tolerance_rad`; `0` uses
    /// `ExecConfig::face_tolerance_rad`) as the next step, ending the current one; wander resumes
    /// choosing once it is reached. HTN mode ([`Self::step_htn`]) only: the pathgrid fallback has
    /// no yaw input. No wander roll asks for it: the hook for callers (a greeting turn).
    pub fn face(&mut self, target: FaceTarget, tolerance_rad: f32) {
        self.current_goal = None;
        self.current_first_target = None;
        self.fallback_walk = None;
        self.state.set_bit(self.vocab.keys.has_idle, false);
        self.state.set_bit(self.vocab.keys.has_walk_goal, false);
        self.state.set_bit(self.vocab.keys.has_face, true);
        self.bindings.bind_task(
            self.vocab.do_face,
            OperatorKind::Face {
                target,
                tolerance_rad,
            },
        );
        self.runner.clear();
        self.runner.request_replan();
        self.choosing = false;
    }

    /// Whether the driver is currently executing a walk step (bound and not yet finished).
    pub fn is_walking(&self) -> bool {
        self.fallback_walk.is_some()
            || (!self.choosing && self.state.bit(self.vocab.keys.has_walk_goal))
    }

    /// The current fallback walk's [`PathFollower`], if walking in pathgrid fallback mode.
    pub fn follower(&self) -> Option<&PathFollower> {
        self.fallback_walk.as_ref().map(|w| &w.follower)
    }

    /// The pathgrid or direct edge the actor is currently heading along (`(from, to)`).
    pub fn current_edge(&self, pos: WorldPos) -> Option<(WorldPos, WorldPos)> {
        if let Some(f) = self.follower() {
            let t = f.target();
            let to = f.nodes.get(t)?.pos;
            return Some((if t > 0 { f.nodes[t - 1].pos } else { pos }, to));
        }
        let to = self.current_first_target.or(self.current_goal)?;
        Some((pos, to))
    }

    /// After a give-up on the pathgrid edge `from` → `to` that the mover
    /// found unwalkable (too steep, walled): remembers the edge and, instead
    /// of idling, walks on at once to the abandoned goal around every edge
    /// it remembers, when such a route exists. Returns whether it did.
    pub fn replan_around(
        &mut self,
        graph: Option<&PathGraph>,
        from: WorldPos,
        to: WorldPos,
        pos: WorldPos,
    ) -> bool {
        let Some(goal) = self
            .abandoned
            .filter(|_| !self.is_walking())
        else {
            return false;
        };
        let Some(nodes) = self.planner.replan_around(graph, (from, to), pos, goal) else {
            return false;
        };
        self.bind_walk_htn(&nodes);
        self.fallback_walk = Some(FallbackWalk::new(nodes, pos, self.walk_speed, &self.tuning));
        self.abandoned = None;
        self.replans += 1;
        true
    }

    fn choose_next_step(&mut self, graph: Option<&PathGraph>, pos: WorldPos) -> WanderStep {
        let mut step = self
            .planner
            .next_after(graph, pos, &mut self.rng, self.last_idle);
        // Don't head straight back to the goal just given up.
        if let (Some(gone), WanderStep::Walk { nodes, .. }) = (self.abandoned, &step)
            && nodes.last().is_some_and(|n| flat(n.pos, gone) < self.tuning.same_goal_m)
        {
            step = self
                .planner
                .next_after(graph, pos, &mut self.rng, self.last_idle);
        }
        self.last_idle = match &step {
            WanderStep::Idle {
                variant: v @ 2..=9, ..
            } => Some(*v),
            _ => None,
        };
        step
    }

    fn bind_idle(&mut self, variant: u8, seconds: f32) {
        self.current_goal = None;
        self.current_first_target = None;
        self.fallback_walk = None;
        self.state
            .set_bit(self.vocab.keys.has_idle, (2..=9).contains(&variant));
        self.state.set_bit(self.vocab.keys.has_walk_goal, false);
        self.state.set_bit(self.vocab.keys.has_face, false);
        self.state
            .set_int(self.vocab.keys.idle_variant, i32::from(variant));
        self.bindings.bind_task(
            self.vocab.do_idle,
            OperatorKind::Wait {
                duration_s: seconds,
                facing: None,
                idle_variant: Some(variant),
            },
        );
        self.runner.clear();
        self.runner.request_replan();
        self.choosing = false;
    }

    fn bind_walk_htn(&mut self, nodes: &[PathNode]) {
        let goal_pos = nodes.last().expect("walk has at least one node").pos;
        let first_pos = nodes.first().map(|n| n.pos);
        self.current_goal = Some(goal_pos);
        self.current_first_target = first_pos;
        self.state.set_bit(self.vocab.keys.has_idle, false);
        self.state.set_bit(self.vocab.keys.has_walk_goal, true);
        self.state.set_bit(self.vocab.keys.has_face, false);
        self.state.set_int(self.vocab.keys.idle_variant, 0);
        self.bindings.bind_task(
            self.vocab.do_walk,
            OperatorKind::MoveTo {
                goal: Location::new(self.worldspace.clone(), goal_pos),
                gait: ExecGait::Walk,
                speed: self.walk_speed,
            },
        );
        self.runner.clear();
        self.runner.request_replan();
        self.choosing = false;
    }

    /// Steps the wander driver in HTN + `sf_nav` mode (`NavCorridor` executes `MoveTo`).
    pub fn step_htn(
        &mut self,
        graph: Option<&PathGraph>,
        ctx: &ExecContext,
        cfg: &ExecConfig,
        trace: Option<&mut TraceEntry>,
    ) -> ExecOutput {
        self.fallback_walk = None;
        self.worldspace = ctx.location.worldspace.clone();

        let was_choosing = self.choosing;
        if was_choosing {
            match self.choose_next_step(graph, ctx.location.pos) {
                WanderStep::Walk { nodes, .. } => self.bind_walk_htn(&nodes),
                WanderStep::Idle { variant, seconds } => self.bind_idle(variant, seconds),
            }
        }

        let step_ctx = if was_choosing && ctx.nav == NavMoveState::Arrived {
            ExecContext {
                nav: NavMoveState::Idle,
                ..ctx.clone()
            }
        } else {
            ctx.clone()
        };

        let mut out = self.runner.step_htn(
            self.vocab.root,
            &self.state,
            &self.vocab.library,
            &self.bindings,
            &step_ctx,
            cfg,
            trace,
        );

        if let Some(op_state) = out.operator {
            match op_state.status {
                OperatorStatus::Running => {}
                OperatorStatus::Succeeded => {
                    if op_state.operator == self.vocab.operators.move_to {
                        self.arrivals += 1;
                        self.abandoned = None;
                        self.current_goal = None;
                        self.current_first_target = None;
                    } else if op_state.operator == self.vocab.operators.face {
                        self.faces += 1;
                    }
                    self.choosing = true;
                }
                OperatorStatus::Failed => {
                    // `NoPath` / `NoProgress` on the navmesh: the goal is what failed (the
                    // corridor never walked a pathgrid edge or first leg), so only the goal is
                    // remembered and [`Self::choose_next_step`] rolls away from it. No pathgrid
                    // `blocked` / `blocked_starts` entry: those are the fallback walk's memory
                    // ([`Self::replan_around`]). `OpenMW` `AiWander::evadeObstacles` likewise
                    // stops, re-chooses and records no edge (`aiwander.cpp` at `b8285cc9`).
                    self.stuck += 1;
                    self.abandoned = self.current_goal.take();
                    self.current_first_target = None;
                    let pause = self.tuning.give_up_pause(&mut self.rng);
                    self.bind_idle(0, pause);
                    out.idle_variant = Some(0);
                }
            }
        }

        out
    }

    fn standing(variant: Option<u8>) -> WanderOutput {
        WanderOutput {
            intent: LocomotionIntent::IDLE,
            idle_variant: variant,
            walking: false,
        }
    }

    /// Advances by `dt` real seconds in pathgrid fallback mode
    /// (for fliers/swimmers or when no navmesh tile is resident at `pos`). `cfg` is the executor
    /// config (`ai.exec.*`): its stuck timeout and progress threshold drive the fallback walk's
    /// per-node stuck check, its face tolerance the fallback's [`PlanRunner`].
    #[allow(clippy::too_many_lines)]
    pub fn update_fallback(
        &mut self,
        graph: Option<&PathGraph>,
        pos: WorldPos,
        dt: f32,
        cfg: &ExecConfig,
    ) -> WanderOutput {
        let fallback = fallback_exec_cfg(cfg);
        // A navmesh walk carried over from `step_htn` (the actor left the resident navmesh
        // mid-walk) has no pathgrid route to follow here: choose again instead of standing on a
        // `MoveTo` nothing drives.
        if !self.choosing && self.fallback_walk.is_none() && self.state.bit(self.vocab.keys.has_walk_goal) {
            self.runner.clear();
            self.current_goal = None;
            self.current_first_target = None;
            self.choosing = true;
        }

        // At most two passes: a finished state picks the next step at once.
        for _ in 0..2 {
            if self.choosing {
                match self.choose_next_step(graph, pos) {
                    WanderStep::Walk { nodes, .. } => {
                        self.bind_walk_htn(&nodes);
                        let walk = FallbackWalk::new(nodes, pos, self.walk_speed, &self.tuning);
                        self.fallback_walk = Some(walk);
                    }
                    WanderStep::Idle { variant, seconds } => {
                        self.bind_idle(variant, seconds);
                    }
                }
                continue;
            }

            if let Some(walk) = &mut self.fallback_walk {
                walk.left -= dt;
                let out = walk.follower.update(pos, dt);
                if out.finished {
                    self.arrivals += 1;
                    self.abandoned = None;
                    self.current_goal = None;
                    self.current_first_target = None;
                    self.fallback_walk = None;
                    self.choosing = true;
                    let ctx = ExecContext {
                        location: Location::new(self.worldspace.clone(), pos),
                        yaw: 0.0,
                        dt,
                        nav: NavMoveState::Arrived,
                        decide: true,
                    };
                    let _ = self.runner.step_htn(
                        self.vocab.root,
                        &self.state,
                        &self.vocab.library,
                        &self.bindings,
                        &ctx,
                        &fallback,
                        None,
                    );
                    continue;
                }
                let t = walk.follower.target();
                let d = walk.follower.nodes.get(t).map_or(0.0, |n| flat(pos, n.pos));
                if t != walk.node || d < walk.best - cfg.progress_min_m {
                    (walk.node, walk.best, walk.still) = (t, d, 0.0);
                } else {
                    walk.still += dt;
                }
                if walk.still >= cfg.stuck_timeout_s || walk.left <= 0.0 {
                    self.stuck += 1;
                    self.abandoned = walk.follower.nodes.last().map(|n| n.pos);
                    self.fallback_walk = None;
                    let ctx = ExecContext {
                        location: Location::new(self.worldspace.clone(), pos),
                        yaw: 0.0,
                        dt,
                        nav: NavMoveState::Failed,
                        decide: true,
                    };
                    let _ = self.runner.step_htn(
                        self.vocab.root,
                        &self.state,
                        &self.vocab.library,
                        &self.bindings,
                        &ctx,
                        &fallback,
                        None,
                    );
                    let pause = self.tuning.give_up_pause(&mut self.rng);
                    self.bind_idle(0, pause);
                    return Self::standing(Some(0));
                }
                let ctx = ExecContext {
                    location: Location::new(self.worldspace.clone(), pos),
                    yaw: 0.0,
                    dt,
                    nav: NavMoveState::Following,
                    decide: true,
                };
                let _ = self.runner.step_htn(
                    self.vocab.root,
                    &self.state,
                    &self.vocab.library,
                    &self.bindings,
                    &ctx,
                    &fallback,
                    None,
                );
                return WanderOutput {
                    intent: out.intent,
                    idle_variant: out.idle_variant,
                    walking: true,
                };
            }

            let ctx = ExecContext {
                location: Location::new(self.worldspace.clone(), pos),
                yaw: 0.0,
                dt,
                nav: NavMoveState::Idle,
                decide: true,
            };
            let exec_out = self.runner.step_htn(
                self.vocab.root,
                &self.state,
                &self.vocab.library,
                &self.bindings,
                &ctx,
                &fallback,
                None,
            );
            if exec_out
                .operator
                .is_some_and(|o| o.status == OperatorStatus::Running)
            {
                return Self::standing(exec_out.idle_variant);
            }
            self.choosing = true;
        }
        Self::standing(Some(0))
    }

    /// [`Self::update_fallback`] with the default executor config ([`ExecConfig::default`], the
    /// `ai.exec.*` defaults), for headless callers without the live settings.
    pub fn update(
        &mut self,
        graph: Option<&PathGraph>,
        pos: WorldPos,
        dt: f32,
    ) -> WanderOutput {
        self.update_fallback(graph, pos, dt, &ExecConfig::default())
    }
}

/// Horizontal distance, m.
fn flat(a: WorldPos, b: WorldPos) -> f32 {
    let d = a.delta_f32(b);
    d.x.hypot(d.z)
}
