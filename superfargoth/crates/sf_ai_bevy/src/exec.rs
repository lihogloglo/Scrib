//! Fixed-tick execution glue for [`sf_ai::exec`] (`docs/specs/ai.md` "Executors", slice AI6):
//! [`AiExecPlugin`] steps each [`AiExecutor`] in `FixedUpdate` inside [`ControllerSystems::Drive`]
//! (after [`NavSystems::Publish`] and before [`NavSystems::Corridor`]), drives [`NavCorridor`] and
//! [`MoveIntent`], and records operator / path transitions in [`AiTraces`].

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_ai::{
    ExecConfig, ExecContext, ExecGait, ExecOutput, KeySet, NavCommand, NavMoveState,
    OperatorBindings, OperatorId, OperatorKind, OperatorStatus, PathState, PlanRunner, StateKey,
    StateKeys, TaskId, TraceEntry, WorldState,
};
use sf_controller::{Gait, LocomotionIntent};
use sf_controller_bevy::{ControllerSettings, ControllerSystems, Dormant, Held, MoveIntent, Mover};
use sf_nav_bevy::{NavCorridor, NavQueries, NavSystems};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::{Location, WorldspaceId};

use crate::{AiActor, AiLibrary, AiTraces};

/// `ai.exec.stuck_timeout_s`: [`ExecConfig::stuck_timeout_s`] (`ai.md` "Executors": 3 s without progress).
pub const EXEC_STUCK_TIMEOUT_S: Def<f32> = def!(float "ai.exec.stuck_timeout_s" = 3.0)
    .range(0.5, 30.0)
    .unit(Unit::Seconds)
    .help("Seconds without planar progress toward the goal before MoveTo fails with NoProgress")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.exec.progress_min_m`: [`ExecConfig::progress_min_m`].
pub const EXEC_PROGRESS_MIN_M: Def<f32> = def!(float "ai.exec.progress_min_m" = 0.25)
    .range(0.01, 5.0)
    .unit(Unit::Meters)
    .help("Minimum planar XZ distance closure toward the goal that resets the MoveTo stuck timer")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.exec.face_tolerance_rad`: [`ExecConfig::face_tolerance_rad`].
pub const EXEC_FACE_TOLERANCE_RAD: Def<f32> = def!(float "ai.exec.face_tolerance_rad" = 0.05)
    .range(0.005, 0.5)
    .unit(Unit::None)
    .help("Default angular tolerance in radians within which a Face operator succeeds")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.exec.arrive_dist_m`: [`ExecConfig::arrive_dist_m`] (matches `PathFollower::stop_radius`).
pub const EXEC_ARRIVE_DIST_M: Def<f32> = def!(float "ai.exec.arrive_dist_m" = 0.15)
    .range(0.01, 2.0)
    .unit(Unit::Meters)
    .help("Planar XZ distance within which a MoveTo in the same worldspace counts as already arrived")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// This module's live tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &EXEC_STUCK_TIMEOUT_S,
    &EXEC_PROGRESS_MIN_M,
    &EXEC_FACE_TOLERANCE_RAD,
    &EXEC_ARRIVE_DIST_M,
];

/// Fixed-tick AI system sets.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiSystems {
    /// Steps [`AiExecutor`] components in `FixedUpdate` inside [`ControllerSystems::Drive`],
    /// after [`NavSystems::Publish`] and before [`NavSystems::Corridor`].
    Execute,
}

/// Live executor settings bound from [`TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct AiExecSettings {
    pub config: ExecConfig,
}

impl Default for AiExecSettings {
    fn default() -> Self {
        Self {
            config: ExecConfig {
                stuck_timeout_s: EXEC_STUCK_TIMEOUT_S.default_value(),
                progress_min_m: EXEC_PROGRESS_MIN_M.default_value(),
                face_tolerance_rad: EXEC_FACE_TOLERANCE_RAD.default_value(),
                arrive_dist_m: EXEC_ARRIVE_DIST_M.default_value(),
            },
        }
    }
}

/// Fixed simulation tick counter for [`AiTraces`] entries recorded by [`AiExecPlugin`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiExecTick(pub u64);

/// Maps [`ExecGait`] to [`sf_controller::Gait`].
pub fn to_controller_gait(gait: ExecGait) -> Gait {
    match gait {
        ExecGait::Sneak => Gait::Sneak,
        ExecGait::Walk => Gait::Walk,
        ExecGait::Run => Gait::Run,
        ExecGait::Sprint => Gait::Sprint,
    }
}

/// Per-actor plan runner and primitive operator executor (`AI6`).
#[derive(Component, Clone, Debug)]
pub struct AiExecutor {
    pub runner: PlanRunner,
    pub root: Option<TaskId>,
    pub world_state: WorldState,
    pub bindings: OperatorBindings,
    pub worldspace: WorldspaceId,
    pub tier: u8,
    pub idle_variant: Option<u8>,
    pub last_output: Option<ExecOutput>,
}

impl AiExecutor {
    /// Creates an idle executor in `worldspace` at tier 2.
    pub fn new(worldspace: WorldspaceId) -> Self {
        Self {
            runner: PlanRunner::new(),
            root: None,
            world_state: WorldState::new(&StateKeys::new()),
            bindings: OperatorBindings::new(),
            worldspace,
            // tunables: const (AI tier 2 full simulation identifier per ai.md)
            tier: 2,
            idle_variant: None,
            last_output: None,
        }
    }

    /// Creates an executor driven by HTN `root` over `world_state` and `bindings`.
    pub fn with_htn(
        worldspace: WorldspaceId,
        root: TaskId,
        world_state: WorldState,
        bindings: OperatorBindings,
    ) -> Self {
        let mut this = Self::new(worldspace);
        this.root = Some(root);
        this.world_state = world_state;
        this.bindings = bindings;
        this
    }

    /// Creates an executor with a queued sequence of direct operators.
    pub fn with_operators(
        worldspace: WorldspaceId,
        ops: impl IntoIterator<Item = (OperatorId, OperatorKind)>,
    ) -> Self {
        let mut this = Self::new(worldspace);
        for (op, kind) in ops {
            this.runner.queue_operator(op, kind);
        }
        this
    }

    pub fn set_root(&mut self, root: TaskId) {
        self.root = Some(root);
        self.runner.request_replan();
    }

    pub fn run_operator(&mut self, operator: OperatorId, kind: OperatorKind) {
        self.root = None;
        self.runner.run_operator(operator, kind);
    }

    pub fn queue_operator(&mut self, operator: OperatorId, kind: OperatorKind) {
        self.runner.queue_operator(operator, kind);
    }

    pub fn mark_dirty(&mut self, key: StateKey) {
        self.runner.mark_dirty(key);
    }

    pub fn mark_dirty_set(&mut self, keys: &KeySet) {
        self.runner.mark_dirty_set(keys);
    }

    pub fn request_replan(&mut self) {
        self.runner.request_replan();
    }

    pub fn is_complete(&self) -> bool {
        self.runner.is_complete()
    }

    pub fn is_failed(&self) -> bool {
        self.runner.is_failed()
    }

    pub fn last_status(&self) -> Option<OperatorStatus> {
        self.runner.last_status()
    }

    pub fn last_path(&self) -> PathState {
        self.runner.last_path()
    }
}

/// Steps [`AiExecutor`] components in `FixedUpdate` inside [`ControllerSystems::Drive`].
pub struct AiExecPlugin;

impl Plugin for AiExecPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiExecSettings::default())
            .init_resource::<AiExecTick>()
            .configure_sets(
                FixedUpdate,
                AiSystems::Execute
                    .in_set(ControllerSystems::Drive)
                    .after(NavSystems::Publish)
                    .before(NavSystems::Corridor),
            )
            .add_systems(FixedUpdate, drive_executors.in_set(AiSystems::Execute))
            .add_systems(
                FixedUpdate,
                apply_idle_intent
                    .in_set(ControllerSystems::Drive)
                    .after(NavSystems::Corridor),
            );
        add_exec_tunables(app);
    }
}

/// Registers the `ai.exec.*` [`TUNABLES`] and binds them to [`AiExecSettings`] (the caller
/// inserts the resource). [`AiExecPlugin`] calls it; a game that drives its own executor loop on
/// [`AiExecSettings`] without the plugin calls it once instead.
pub fn add_exec_tunables(app: &mut App) {
    let _stuck = app
        .add_tunable(&EXEC_STUCK_TIMEOUT_S)
        .bind_resource(|s: &mut AiExecSettings, v| s.config.stuck_timeout_s = v);
    let _prog = app
        .add_tunable(&EXEC_PROGRESS_MIN_M)
        .bind_resource(|s: &mut AiExecSettings, v| s.config.progress_min_m = v);
    let _face = app
        .add_tunable(&EXEC_FACE_TOLERANCE_RAD)
        .bind_resource(|s: &mut AiExecSettings, v| s.config.face_tolerance_rad = v);
    let _arrive = app
        .add_tunable(&EXEC_ARRIVE_DIST_M)
        .bind_resource(|s: &mut AiExecSettings, v| s.config.arrive_dist_m = v);
}

fn nav_move_state(nav: &NavCorridor) -> NavMoveState {
    if nav.failed.is_some() {
        NavMoveState::Failed
    } else if nav.is_arrived() {
        NavMoveState::Arrived
    } else if nav.corridor().is_some() {
        NavMoveState::Following
    } else if nav.pending.is_some() || nav.goal.is_some() {
        NavMoveState::Pending
    } else {
        NavMoveState::Idle
    }
}

fn clear_corridor_goal(nav: &mut NavCorridor, queries: Option<&mut NavQueries>) {
    if let Some(id) = nav.pending.take()
        && let Some(q) = queries
    {
        q.cancel(id);
    }
    nav.clear_goal();
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
fn drive_executors(
    time: Res<Time>,
    fixed_time: Option<Res<Time<Fixed>>>,
    ctrl: Option<Res<ControllerSettings>>,
    settings: Res<AiExecSettings>,
    mut tick: ResMut<AiExecTick>,
    ai_lib: Option<Res<AiLibrary>>,
    mut traces: Option<ResMut<AiTraces>>,
    mut queries: Option<ResMut<NavQueries>>,
    mut actors: Query<(
        Option<&AiActor>,
        &Mover,
        &mut MoveIntent,
        Option<&mut NavCorridor>,
        &mut AiExecutor,
        Has<Dormant>,
        Has<Held>,
    )>,
) {
    tick.0 += 1;
    let current_tick = tick.0;
    let raw_dt = if time.delta_secs() > 0.0 {
        time.delta_secs()
    } else {
        // tunables: const (fallback 60 Hz fixed timestep when Time::delta_secs is 0)
        fixed_time.map_or(1.0 / 60.0, |t| t.timestep().as_secs_f32())
    };
    let dt = ctrl.map_or(raw_dt, |c| raw_dt.min(c.max_dt));

    for (actor, mover, mut intent, mut corridor, mut exec, dormant, held) in &mut actors {
        if dormant || held {
            continue;
        }
        step_one_actor(
            actor,
            mover,
            &mut intent,
            corridor.as_deref_mut(),
            &mut exec,
            current_tick,
            dt,
            &settings.config,
            ai_lib.as_deref(),
            traces.as_deref_mut(),
            queries.as_deref_mut(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn step_one_actor(
    actor: Option<&AiActor>,
    mover: &Mover,
    intent: &mut MoveIntent,
    corridor: Option<&mut NavCorridor>,
    exec: &mut AiExecutor,
    current_tick: u64,
    dt: f32,
    cfg: &ExecConfig,
    ai_lib: Option<&AiLibrary>,
    mut traces: Option<&mut AiTraces>,
    queries: Option<&mut NavQueries>,
) {
    if exec.root.is_none() && exec.runner.is_idle() {
        return;
    }
    if (exec.is_complete() || exec.is_failed()) && exec.runner.is_idle() && exec.root.is_none() {
        return;
    }

    let ws = corridor
        .as_ref()
        .map_or_else(|| exec.worldspace.clone(), |c| c.worldspace.clone());
    exec.worldspace = ws.clone();
    let nav_state = corridor
        .as_deref()
        .map_or(NavMoveState::Idle, nav_move_state);
    let ctx = ExecContext {
        location: Location::new(ws, mover.loco.position),
        yaw: mover.loco.motion.yaw,
        dt,
        nav: nav_state,
    };

    let prev_op = exec.last_output.as_ref().and_then(|o| o.operator);
    let prev_path = exec
        .last_output
        .as_ref()
        .map_or(PathState::None, |o| o.path);

    let wants_trace = actor
        .zip(traces.as_ref())
        .is_some_and(|(a, t)| t.wants(a.0, exec.tier));
    let mut entry = wants_trace.then(|| TraceEntry::new(current_tick, exec.tier));

    let out = if let Some(root) = exec.root {
        let Some(lib_res) = ai_lib else {
            return;
        };
        let AiExecutor {
            ref mut runner,
            ref world_state,
            ref bindings,
            ..
        } = *exec;
        runner.step_htn(
            root,
            world_state,
            &lib_res.library,
            bindings,
            &ctx,
            cfg,
            entry.as_mut(),
        )
    } else {
        exec.runner.step_direct(&ctx, cfg, entry.as_mut())
    };

    if let (Some(a), Some(t), Some(e)) = (actor, traces.as_mut(), entry)
        && (out.plan_outcome.is_some() || out.operator != prev_op || out.path != prev_path)
    {
        t.push(a.0, e);
    }

    exec.idle_variant = out.idle_variant;
    apply_exec_output(&out, intent, corridor, queries);
    exec.last_output = Some(out);
}

fn apply_exec_output(
    out: &ExecOutput,
    intent: &mut MoveIntent,
    corridor: Option<&mut NavCorridor>,
    queries: Option<&mut NavQueries>,
) {
    match &out.nav {
        NavCommand::MoveTo { goal, gait, speed } => {
            if let Some(nav) = corridor {
                let needs_new_goal =
                    nav.goal.as_ref() != Some(goal) || nav.is_arrived() || nav.failed.is_some();
                if needs_new_goal {
                    clear_corridor_goal(nav, queries);
                    nav.set_goal(goal.clone());
                }
                nav.follower.gait = to_controller_gait(*gait);
                nav.follower.speed = *speed;
            }
        }
        NavCommand::Stop => {
            if let Some(nav) = corridor {
                clear_corridor_goal(nav, queries);
            }
            intent.0 = LocomotionIntent {
                desired_velocity: Vec3::ZERO,
                desired_facing: out.facing,
                gait: Gait::Walk,
            };
        }
        NavCommand::None => {
            if corridor.is_none_or(|c| c.goal.is_none()) {
                intent.0 = LocomotionIntent {
                    desired_velocity: Vec3::ZERO,
                    desired_facing: out.facing,
                    gait: Gait::Walk,
                };
            }
        }
    }
}

/// Preserves [`ExecOutput::facing`] on [`MoveIntent`] when [`NavCorridor`] has no active goal
/// (`NavCorridor` resets `MoveIntent` to `IDLE` when `goal` is `None`).
#[allow(clippy::type_complexity)]
fn apply_idle_intent(
    mut actors: Query<
        (&AiExecutor, Option<&NavCorridor>, &mut MoveIntent),
        (Without<Dormant>, Without<Held>),
    >,
) {
    for (exec, corridor, mut intent) in &mut actors {
        if corridor.is_some_and(|c| c.goal.is_some()) {
            continue;
        }
        if let Some(ref out) = exec.last_output
            && matches!(out.nav, NavCommand::Stop | NavCommand::None)
        {
            intent.0 = LocomotionIntent {
                desired_velocity: Vec3::ZERO,
                desired_facing: out.facing,
                gait: Gait::Walk,
            };
        }
    }
}
