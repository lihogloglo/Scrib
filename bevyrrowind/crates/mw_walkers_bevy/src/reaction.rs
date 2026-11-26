//! The perception-reaction layer (`AI8i`): a higher-priority driver over every AI actor's routine.
//!
//! **Industry standard.** UE Behavior Trees / `StateTree`, Decima and RDR2 keep routines (wander,
//! schedule, standing on post, a scripted path) and perception reactions apart: a reaction is a
//! higher-priority branch that *pre-empts* whatever routine runs, and when it ends the actor walks
//! back to its routine's anchor (the post or the point it left) and the routine resumes. Reactions
//! never depend on the actor having a wander package. This module is that branch for the walkers.
//!
//! **Who.** Every awake ground actor with a [`NavCorridor`] and a [`Reaction`] (which requires the
//! [`Searcher`]): wanderers ([`crate::walkers::Walker`] requires [`Reaction`]), stationary NPCs
//! (`AI_W` distance 0: guards on post, shopkeepers; `attach` gives them the corridor and a
//! [`Reaction`], no wander) and level actors whose script dropped the wander
//! (`mw_exterior_bevy::level::actors`).
//!
//! **Phases** (one per actor, stepped by [`react`] on the fixed tick before
//! [`NavSystems::Corridor`], ahead of every routine):
//! - **Routine**: the reaction owns nothing; the routine (the wander drive, a level path, or
//!   nobody: the corridor's idle) writes the intent.
//! - **Search** (`AI8f2`, [`crate::search`]): the strongest awareness entry is Searching. On the
//!   first reaction tick the actor's position and yaw become its **anchor** (where the routine was
//!   pre-empted: a stationary NPC's post, a path actor's place on its path, a wanderer's place in
//!   its disc); restarts and re-searches keep the first anchor.
//! - **React** (`AI8h2`, `sf_ai::react`): the strongest entry is Curious (turn toward the last known
//!   position) or Suspicious (step [`sf_ai::ReactConfig::step_m`] toward it, then turn), after the
//!   double take. Each actor's [`ReactState`] (on its [`Reaction`]) is updated every tick, and its
//!   goal's operators restart whenever its revision changes; a finished goal stands facing the
//!   stimulus until the band drops (or the goal re-targets). The search wins: React is only chosen
//!   when no search runs, and a Searching band idles the reaction. Same pre-emption as the search
//!   (the anchor is taken on entry); a goal that goes idle starts the Return.
//! - **Return**: the search gave up (ttl), the band left Searching, or a React goal went idle: walk back to the anchor
//!   (`MoveTo`, through the same corridor) and turn to its yaw (`Face`); a fresh Searching episode
//!   (or a new React goal) on the way pre-empts the return. When it finishes (or its walk fails) the routine gets the
//!   actor back: a wander re-chooses from there.
//!
//! While a reaction owns the actor ([`Reaction::owns`]) the routines stay off the intent: the
//! wander drive pauses (`walkers::drive`), level path actors skip their path step, and
//! [`apply_reaction_intent`] restores a stand's facing after the corridor's idle reset
//! ([`ReactionSystems::Apply`], after [`NavSystems::Corridor`]; level drives order after it).
//!
//! **Arbitration order** in [`react`] each tick: the search (`Run`: Search), then a search that
//! ended starts the Return, then React (any phase but Search; it pre-empts Routine and Return), then
//! the Return steps. The routines only read [`Reaction::owns`].
//!
//! Off the navmesh (fliers, swimmers, the pathgrid fallback, no resident tile) there is no reaction
//! (its moves need the corridor): a running one is dropped and the routine keeps the actor.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_time::prelude::*;
use sf_ai::{
    ExecConfig, ExecContext, ExecGait, ExecOutput, FaceTarget, MoveOperatorIds, NavCommand, OperatorExec, OperatorKind,
    OperatorStatus, ReactConfig, ReactGoal, ReactInput, ReactState,
};
use sf_ai_bevy::{AiActor, AiAwareness, AiEffectivePerception, AiExecSettings, AiPerceptionSettings};
use sf_controller::LocomotionIntent;
use sf_controller_bevy::{Avoider, ControllerSystems, Dormant, MoveIntent, Mover, MoverMode};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_nav::DEFAULT_EXTENT;
use sf_nav_bevy::{NavCorridor, NavMeshes, NavQueries, NavSystems};
use sf_world::Location;

use crate::search::{SearchMode, SearchWorld, Searcher};
use crate::walkers::{ActorSpace, Walker, clear_corridor_goal, nav_move_state};

/// Operator transitions one tick may run (a finished `Face` or an already-reached point hands over
/// to the next operator in the same tick).
// tunables: const (loop bound: bounds the work per tick; a goal is at most two operators)
const MAX_TRANSITIONS: usize = 3;

/// The reaction systems' order points.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReactionSystems {
    /// [`react`]: before [`NavSystems::Corridor`] and before every routine that drives through it
    /// (the wander drive orders after it).
    React,
    /// Restores an owning reaction's stand intent after [`NavSystems::Corridor`]; routines that
    /// write `MoveIntent` after the corridor (level paths) order after this and skip while
    /// [`Reaction::owns`].
    Apply,
}

/// Which layer drives the actor (module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    /// The routine owns the intent.
    #[default]
    Routine,
    /// A search runs ([`Searcher`]).
    Search,
    /// A Curious / Suspicious reaction runs (`sf_ai::react`).
    React,
    /// Walking back to the anchor, then turning to its yaw.
    Return,
}

/// What an actor's reactions did since spawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReactionStats {
    /// Times a reaction pre-empted the routine.
    pub preempted: u32,
    /// Returns to the anchor started.
    pub returns: u32,
    /// Returns that reached the anchor and turned to its yaw.
    pub returned: u32,
    /// Returns whose walk failed (no path, no progress): the routine resumed where the actor stood.
    pub return_failed: u32,
    /// Curious / Suspicious goals started (one per [`ReactState`] revision run).
    pub reacts: u32,
}

/// An actor's perception-reaction state (`AI8i`, module docs).
#[derive(Component, Debug, Default)]
#[require(Searcher)]
pub struct Reaction {
    /// Walk speed of the reaction's moves, m/s (`None`: the wander's, else the corridor's).
    pub walk_speed: Option<f32>,
    phase: Phase,
    /// Where the routine was pre-empted: position and yaw.
    anchor: Option<(WorldPos, f32)>,
    /// The React and Return phases' operators.
    runner: OpRunner,
    /// The Curious / Suspicious reaction (`AI8h2`), updated every tick from the awareness.
    react: ReactState,
    /// The [`ReactState::revision`] whose operators the runner holds (React phase only).
    react_rev: Option<u32>,
    /// A stand's intent, restored after the corridor's idle reset.
    held: Option<LocomotionIntent>,
    pub stats: ReactionStats,
}

impl Reaction {
    pub fn new(walk_speed: f32) -> Self {
        Self { walk_speed: Some(walk_speed), ..Self::default() }
    }

    /// Whether a reaction drives the actor (the routines stay off its intent).
    pub fn owns(&self) -> bool {
        self.phase != Phase::Routine
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Where the routine was pre-empted (position, yaw), while a reaction owns the actor.
    pub fn anchor(&self) -> Option<(WorldPos, f32)> {
        self.anchor
    }

    /// The Curious / Suspicious reaction state.
    pub fn react_state(&self) -> &ReactState {
        &self.react
    }

    fn preempt(&mut self, pos: WorldPos, yaw: f32) {
        if self.phase == Phase::Routine {
            self.anchor = Some((pos, yaw));
            self.stats.preempted += 1;
        }
    }

    fn start_return(&mut self, location: &Location, speed: f32, cfg: &ExecConfig) {
        let Some((pos, yaw)) = self.anchor else {
            self.phase = Phase::Routine;
            return;
        };
        self.phase = Phase::Return;
        self.react_rev = None;
        self.stats.returns += 1;
        self.runner.start([
            Some(OperatorKind::MoveTo {
                goal: Location::new(location.worldspace.clone(), pos),
                gait: ExecGait::Walk,
                speed,
            }),
            Some(OperatorKind::Face { target: FaceTarget::Yaw(yaw), tolerance_rad: cfg.face_tolerance_rad }),
        ]);
    }

    /// Gives the actor back to its routine.
    fn hand_back(&mut self) {
        self.phase = Phase::Routine;
        self.anchor = None;
        self.held = None;
        self.runner = OpRunner::default();
        self.react_rev = None;
    }

    /// Runs a non-idle React `goal` (pre-empting the routine or a return), restarting its operators
    /// when the reaction re-targeted, and steps them; `None` while standing after the goal ended.
    fn step_react(
        &mut self,
        goal: ReactGoal,
        ctx: &ExecContext,
        cfg: &ExecConfig,
        ops: MoveOperatorIds,
        speed: f32,
    ) -> Option<ExecOutput> {
        self.preempt(ctx.location.pos, ctx.yaw);
        let rev = self.react.revision();
        if self.phase != Phase::React || self.react_rev != Some(rev) {
            self.phase = Phase::React;
            self.react_rev = Some(rev);
            self.stats.reacts += 1;
            // A stand held from an earlier goal must not turn the actor back.
            self.held = None;
            self.runner.start(goal.operators(&ctx.location.worldspace, ExecGait::Walk, speed, cfg.face_tolerance_rad));
        }
        match self.runner.step(ctx, cfg, ops) {
            RunStep::Running(o) => Some(o),
            RunStep::Done { .. } => None,
        }
    }
}

/// Runs up to two operators in a row (a goal's `[first, second]`, as `sf_ai`'s search and react
/// goals give them) through the move executors.
#[derive(Debug, Default)]
pub(crate) struct OpRunner {
    queue: [Option<OperatorKind>; 2],
    active: Option<OperatorExec>,
    /// Whether `active` has not been stepped yet (the corridor's state belongs to the last goal).
    fresh: bool,
    walking: bool,
    /// An operator of this goal failed.
    failed: bool,
}

/// One [`OpRunner::step`].
pub(crate) enum RunStep {
    Running(ExecOutput),
    /// Every operator ended; `failed` if one did not succeed.
    Done { failed: bool },
}

impl OpRunner {
    pub(crate) fn start(&mut self, ops: [Option<OperatorKind>; 2]) {
        *self = Self { queue: ops, ..Self::default() };
    }

    pub(crate) fn step(&mut self, ctx: &ExecContext, cfg: &ExecConfig, ops: MoveOperatorIds) -> RunStep {
        let mut last = None;
        for _ in 0..MAX_TRANSITIONS {
            if self.active.is_none() {
                let Some(kind) = self.queue.iter_mut().find_map(Option::take) else {
                    return RunStep::Done { failed: self.failed };
                };
                let id = match kind {
                    OperatorKind::MoveTo { .. } => ops.move_to,
                    OperatorKind::Face { .. } => ops.face,
                    OperatorKind::Wait { .. } => ops.wait,
                };
                self.walking = id == ops.move_to;
                self.active = Some(OperatorExec::new(id, kind));
                self.fresh = true;
            }
            let active = self.active.as_mut().expect("set above");
            // A new operator's first step: the corridor's arrival or failure is the last goal's.
            let step_ctx;
            let ctx = if self.fresh && matches!(ctx.nav, sf_ai::NavMoveState::Arrived | sf_ai::NavMoveState::Failed) {
                step_ctx = ExecContext { nav: sf_ai::NavMoveState::Idle, ..ctx.clone() };
                &step_ctx
            } else {
                ctx
            };
            self.fresh = false;
            let out = active.step(ctx, cfg);
            let status = out.operator.map_or(OperatorStatus::Succeeded, |o| o.status);
            if status == OperatorStatus::Running {
                return RunStep::Running(out);
            }
            self.failed |= status != OperatorStatus::Succeeded;
            self.active = None;
            last = Some(out);
        }
        last.map_or(RunStep::Done { failed: self.failed }, RunStep::Running)
    }
}

/// Registers [`react`] before [`NavSystems::Corridor`] and ahead of the wander drive, and
/// [`apply_reaction_intent`] in [`ReactionSystems::Apply`] after it.
pub(crate) fn add(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        ReactionSystems::Apply.in_set(ControllerSystems::Drive).after(NavSystems::Corridor),
    )
    .add_systems(
        FixedUpdate,
        (
            react
                .in_set(ReactionSystems::React)
                .in_set(ControllerSystems::Drive)
                .after(NavSystems::Publish)
                .before(NavSystems::Corridor),
            apply_reaction_intent.in_set(ReactionSystems::Apply),
        ),
    );
}

/// Steps every awake reacting actor (module docs): arbitrates the search from its awareness,
/// pre-empts the routine, runs the search or the return through the corridor, and hands back.
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines, clippy::type_complexity)]
fn react(
    time: Res<Time>,
    exec_settings: Option<Res<AiExecSettings>>,
    perception: Option<Res<AiPerceptionSettings>>,
    meshes: Option<Res<NavMeshes>>,
    mut queries: Option<ResMut<NavQueries>>,
    search_world: SearchWorld,
    mut actors: Query<
        (
            (Entity, &Mover, &ActorSpace, Option<&AiActor>),
            (&mut MoveIntent, &mut NavCorridor, &mut Reaction, &mut Searcher),
            (Option<&mut AiAwareness>, Option<&AiEffectivePerception>, Option<&mut Walker>, Option<&mut Avoider>),
        ),
        Without<Dormant>,
    >,
) {
    let exec_cfg = exec_settings.map_or_else(ExecConfig::default, |s| s.config);
    let react_cfg = perception.map_or_else(ReactConfig::default, |s| s.react);
    let dt = time.delta_secs().min(0.1);
    for (
        (entity, mover, space, actor),
        (mut intent, mut nav, mut reaction, mut searcher),
        (mut awareness, eff, mut walker, avoider),
    ) in &mut actors
    {
        let key = actor.map_or(ActorKey(u128::from(entity.to_bits())), |a| a.0);
        let setup = search_world.setup(key, eff);
        let wants = awareness.as_deref().is_some_and(|aw| {
            aw.0.strongest().is_some_and(|s| s.band == sf_ai::AwarenessBand::Searching)
        });
        let pos = mover.loco.position;
        let yaw = mover.loco.motion.yaw;
        let loc = Location::new(space.0.clone(), pos);
        // AI8h2: the Curious / Suspicious reaction advances every tick (its double take and spent
        // placement need every band change), whatever runs.
        let react_goal = if let (Some(aw), Some(setup)) = (awareness.as_deref(), setup.as_ref()) {
            let top = aw.0.strongest().map(ReactInput::from);
            reaction.react.update(&react_cfg, top, pos, setup.now)
        } else {
            reaction.react = ReactState::new();
            ReactGoal::None
        };
        let reacts = react_goal != ReactGoal::None;
        // Fast path: nothing to react to and nothing running (no navmesh query).
        let on_navmesh = (wants || reacts || reaction.owns())
            && matches!(mover.loco.mode, MoverMode::Ground)
            && meshes.as_ref().is_some_and(|m| m.0.nearest(&loc, nav.class, DEFAULT_EXTENT, &nav.filter).is_some());
        let mut out = None;
        if on_navmesh {
            let speed = reaction
                .walk_speed
                .or_else(|| walker.as_ref().map(|w| w.driver.walk_speed))
                .unwrap_or(nav.follower.speed);
            let ctx = ExecContext { location: loc.clone(), yaw, dt, nav: nav_move_state(&nav), decide: true };
            let mode = awareness
                .as_deref()
                .zip(setup.as_ref())
                .map_or(SearchMode::Off, |(aw, setup)| searcher.arbitrate(&aw.0, setup));
            if mode == SearchMode::Run {
                reaction.preempt(pos, yaw);
                reaction.phase = Phase::Search;
                let now = setup.as_ref().expect("Run needs the setup").now;
                let (class, filter) = (nav.class, &nav.filter);
                let mut reachable = |p: WorldPos| {
                    meshes.as_ref().is_some_and(|m| {
                        m.0.nearest(&Location::new(space.0.clone(), p), class, DEFAULT_EXTENT, filter).is_some()
                    })
                };
                out = searcher.step(&ctx, &exec_cfg, search_world.vocab(), &space.0, speed, now, &mut reachable);
            }
            if out.is_none() && reaction.phase == Phase::Search {
                // The search gave up (ttl) or the band left Searching: back to the anchor.
                reaction.start_return(&loc, speed, &exec_cfg);
            }
            if reaction.phase != Phase::Search {
                if reacts {
                    out = reaction.step_react(react_goal, &ctx, &exec_cfg, search_world.vocab().0.operators, speed);
                    if out.is_none() && reaction.held.is_none() {
                        // The goal ended without a stand to hold (already facing): stand here.
                        clear_corridor_goal(&mut nav, queries.as_deref_mut());
                        let stand = LocomotionIntent { desired_facing: Some(yaw), ..LocomotionIntent::IDLE };
                        intent.0 = stand;
                        reaction.held = Some(stand);
                    }
                } else if reaction.phase == Phase::React {
                    // The band dropped or the hold is over: back to the anchor.
                    reaction.start_return(&loc, speed, &exec_cfg);
                }
            }
            if reaction.phase == Phase::Return {
                match reaction.runner.step(&ctx, &exec_cfg, search_world.vocab().0.operators) {
                    RunStep::Running(o) => out = Some(o),
                    RunStep::Done { failed } => {
                        if failed {
                            reaction.stats.return_failed += 1;
                        } else {
                            reaction.stats.returned += 1;
                        }
                        clear_corridor_goal(&mut nav, queries.as_deref_mut());
                        reaction.hand_back();
                        resume_wander(walker.as_deref_mut(), yaw);
                    }
                }
            }
        } else {
            if wants || searcher.is_searching() {
                // No search off the navmesh (its moves need the corridor); the episode resets.
                searcher.abort();
            }
            if !wants && let (Some(aw), Some(setup)) = (awareness.as_deref(), setup.as_ref()) {
                // The episode is over (resets a given-up search for the next one).
                let _ = searcher.arbitrate(&aw.0, setup);
            }
            if reaction.owns() {
                clear_corridor_goal(&mut nav, queries.as_deref_mut());
                reaction.hand_back();
                resume_wander(walker.as_deref_mut(), yaw);
            }
        }
        if let Some(out) = out {
            if let Some(w) = walker.as_deref_mut() {
                // The paused wander's held stand must not overwrite the reaction's intent.
                w.release_held_intent();
            }
            apply_output(&out, &mut nav, &mut intent, &mut reaction, queries.as_deref_mut());
            if let Some(mut avoider) = avoider {
                avoider.travel = nav
                    .corridor()
                    .and_then(|c| c.corners().first())
                    .map_or(Vec3::ZERO, |c| pos.delta_f32(c.pos).with_y(0.0).normalize_or_zero());
            }
        }
        // AI8f3: a running search holds its target's entry at the Searching floor (taken
        // mutably only on a change, so the meter's change ticks stay meaningful).
        if let Some(aw) = awareness.as_mut()
            && searcher.hold_stale(&aw.0)
            && let Some(setup) = setup.as_ref()
        {
            searcher.sync_hold(&mut aw.0, setup);
        }
    }
}

/// A paused wander re-chooses from where the reaction left the actor (a turn to the current yaw
/// ends its paused step at once).
fn resume_wander(walker: Option<&mut Walker>, yaw: f32) {
    if let Some(w) = walker {
        w.driver.face(FaceTarget::Yaw(yaw), 0.0);
    }
}

/// Feeds an executor step into the corridor (a move) or the intent (a stand, held past the
/// corridor's idle reset), as the wander drive does.
fn apply_output(
    out: &ExecOutput,
    nav: &mut NavCorridor,
    intent: &mut MoveIntent,
    reaction: &mut Reaction,
    queries: Option<&mut NavQueries>,
) {
    match &out.nav {
        NavCommand::MoveTo { goal, gait, speed } => {
            if nav.goal.as_ref() != Some(goal) || nav.is_arrived() || nav.failed.is_some() {
                clear_corridor_goal(nav, queries);
                nav.set_goal(goal.clone());
            }
            nav.follower.gait = sf_ai_bevy::to_controller_gait(*gait);
            nav.follower.speed = *speed;
            reaction.held = None;
        }
        NavCommand::Stop | NavCommand::None => {
            clear_corridor_goal(nav, queries);
            let stand = LocomotionIntent { desired_facing: out.facing, ..LocomotionIntent::IDLE };
            intent.0 = stand;
            reaction.held = Some(stand);
        }
    }
}

/// Restores an owning reaction's stand (its facing) after [`NavSystems::Corridor`] reset a
/// goal-less corridor's intent to `IDLE`.
#[allow(clippy::type_complexity)]
fn apply_reaction_intent(
    mut actors: Query<(&Reaction, &mut MoveIntent), (Without<Dormant>, Without<sf_controller_bevy::Held>)>,
) {
    for (reaction, mut intent) in &mut actors {
        if let Some(stand) = reaction.held.filter(|_| reaction.owns()) {
            intent.0 = stand;
        }
    }
}
