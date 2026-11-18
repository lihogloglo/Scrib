//! Corridor following on the fixed tick (`docs/specs/nav.md`, NV8): [`NavCorridorPlugin`] steps
//! each mover's [`PathCorridor`] in `FixedUpdate` inside [`ControllerSystems::Drive`] (after
//! [`NavSystems::Publish`] and before [`NavSystems::Queries`]), feeds string-pulled corners into
//! [`PathFollower`], writes [`MoveIntent`], and requests a replan when the corridor is trimmed or
//! drifts beyond local repair.

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use sf_controller::{LocomotionIntent, PathFollower, PathMode};
use sf_controller_bevy::{ControllerSettings, ControllerSystems, Dormant, Held, MoveIntent, Mover};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_nav::{
    AgentClassId, CorridorConfig, Goal, NavError, PathCorridor, PathPriority, PathQuery,
    QueryFilter, RequestId,
};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::{Location, WorldspaceId};

use crate::{NavDebugPaths, NavMeshes, NavQueries, NavSystems};

/// `ai.nav.corridor.max_corners`: [`CorridorConfig::max_corners`]. `DetourCrowd`'s `DT_CROWDAGENT_MAX_CORNERS`
/// is 4.
pub const CORRIDOR_MAX_CORNERS: Def<u32> = def!(count "ai.nav.corridor.max_corners" = 4)
    .range(1, 64)
    .unit(Unit::Count)
    .help("String-pulled steering corners computed per corridor step at most")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.corridor.lookahead_polys`: [`CorridorConfig::lookahead_polys`]. `DetourCrowd`'s
/// `CHECK_LOOKAHEAD` is 10 polygons.
pub const CORRIDOR_LOOKAHEAD_POLYS: Def<u32> = def!(count "ai.nav.corridor.lookahead_polys" = 10)
    .range(1, 256)
    .unit(Unit::Count)
    .help("Corridor polygons ahead of the walker checked for validity each fixed tick")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.corridor.optimize_range_m`: [`CorridorConfig::optimize_range_m`]. `DetourCrowd`'s default
/// `pathOptimizationRange` is `30 * radius` (~18 m for a 0.6 m diameter humanoid).
pub const CORRIDOR_OPTIMIZE_RANGE_M: Def<f32> = def!(float "ai.nav.corridor.optimize_range_m" = 18.0)
    .range(1.0, 128.0)
    .unit(Unit::Meters)
    .help("Maximum XZ raycast distance used to shortcut visible corridor doglegs")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.corridor.topology_period_s`: [`CorridorConfig::topology_period_s`]. `DetourCrowd`'s
/// `OPT_TIME_THR` is 0.5 s.
pub const CORRIDOR_TOPOLOGY_PERIOD_S: Def<f32> = def!(float "ai.nav.corridor.topology_period_s" = 0.5)
    .range(0.05, 10.0)
    .unit(Unit::Seconds)
    .help("Period between bounded local searches that shortcut topological corridor detours")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.corridor.repair_drift_m`: [`CorridorConfig::repair_drift_m`]. Planar XZ drift threshold
/// between the constrained corridor position and the mover before snapping and repairing locally.
pub const CORRIDOR_REPAIR_DRIFT_M: Def<f32> = def!(float "ai.nav.corridor.repair_drift_m" = 0.75)
    .range(0.1, 10.0)
    .unit(Unit::Meters)
    .help("Planar XZ drift between mover and corridor above which a local repair search runs")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.corridor.repair_nodes`: [`CorridorConfig::repair_nodes`]. `DetourCrowd`'s local sliced
/// search cap (`updateSlicedFindPath` iterations).
pub const CORRIDOR_REPAIR_NODES: Def<u32> = def!(count "ai.nav.corridor.repair_nodes" = 32)
    .range(1, 512)
    .unit(Unit::Count)
    .help("Maximum node expansions for local corridor repair and topology optimization")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &CORRIDOR_MAX_CORNERS,
    &CORRIDOR_LOOKAHEAD_POLYS,
    &CORRIDOR_OPTIMIZE_RANGE_M,
    &CORRIDOR_TOPOLOGY_PERIOD_S,
    &CORRIDOR_REPAIR_DRIFT_M,
    &CORRIDOR_REPAIR_NODES,
];

/// Corridor maintenance settings (bound live from [`TUNABLES`]).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct NavCorridorSettings {
    pub config: CorridorConfig,
}

impl Default for NavCorridorSettings {
    fn default() -> Self {
        Self {
            config: CorridorConfig {
                // tunables: const (bound from CORRIDOR_MAX_CORNERS)
                max_corners: CORRIDOR_MAX_CORNERS.default_value() as usize,
                // tunables: const (bound from CORRIDOR_LOOKAHEAD_POLYS)
                lookahead_polys: CORRIDOR_LOOKAHEAD_POLYS.default_value() as usize,
                optimize_range_m: CORRIDOR_OPTIMIZE_RANGE_M.default_value(),
                topology_period_s: CORRIDOR_TOPOLOGY_PERIOD_S.default_value(),
                repair_drift_m: CORRIDOR_REPAIR_DRIFT_M.default_value(),
                repair_nodes: CORRIDOR_REPAIR_NODES.default_value(),
            },
        }
    }
}

/// Drives a [`Mover`] along a navmesh [`PathCorridor`] via [`PathFollower`] (`NV8`).
#[derive(Component, Clone, Debug)]
pub struct NavCorridor {
    pub actor: ActorKey,
    pub class: AgentClassId,
    pub filter: QueryFilter,
    pub priority: PathPriority,
    pub worldspace: WorldspaceId,
    pub goal: Option<Location>,
    pub corridor: Option<PathCorridor>,
    pub follower: PathFollower,
    pub pending: Option<RequestId>,
    pub arrived: bool,
    /// The last path request or corridor step failed. While set, no request is issued until a
    /// re-arm: a new goal ([`Self::set_goal`] clears it) or a [`NavMeshes`] change since the last
    /// request (one retry per change, like Detour's `DT_CROWDAGENT_TARGET_FAILED` sticking until a
    /// new target, plus UE's repath on navmesh update). A successful reply clears it.
    pub failed: Option<NavError>,
    /// [`NavMeshes`] changed since the last request was issued (re-arms one retry while
    /// [`Self::failed`] is set).
    mesh_changed_since_request: bool,
}

impl NavCorridor {
    pub fn new(actor: ActorKey, class: AgentClassId, worldspace: WorldspaceId, speed: f32) -> Self {
        Self {
            actor,
            class,
            filter: QueryFilter::default(),
            priority: PathPriority::Normal,
            worldspace,
            goal: None,
            corridor: None,
            follower: PathFollower::new(Vec::new(), PathMode::Once, speed),
            pending: None,
            arrived: false,
            failed: None,
            mesh_changed_since_request: false,
        }
    }

    #[must_use]
    pub fn with_filter(mut self, filter: QueryFilter) -> Self {
        self.filter = filter;
        self
    }

    #[must_use]
    pub fn with_priority(mut self, priority: PathPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets a new goal location, resetting arrival/failure state and requesting a path on the next
    /// fixed tick.
    pub fn set_goal(&mut self, goal: Location) {
        self.worldspace = goal.worldspace.clone();
        self.goal = Some(goal);
        self.corridor = None;
        self.pending = None;
        self.arrived = false;
        self.failed = None;
        self.follower.set_corridor_corners(&[], true);
    }

    /// Clears the active goal and stops following.
    pub fn clear_goal(&mut self) {
        self.goal = None;
        self.corridor = None;
        self.pending = None;
        self.arrived = false;
        self.failed = None;
        self.follower.set_corridor_corners(&[], true);
    }

    pub fn is_arrived(&self) -> bool {
        self.arrived
    }

    pub fn corridor(&self) -> Option<&PathCorridor> {
        self.corridor.as_ref()
    }

    /// No request in flight, and not failed (or a mesh change re-armed one retry).
    fn may_request(&self) -> bool {
        self.pending.is_none() && (self.failed.is_none() || self.mesh_changed_since_request)
    }

    /// Queues a path from `from` to the goal, replacing the actor's other requests.
    fn request_path(&mut self, queries: &mut NavQueries, from: Location) {
        let Some(goal) = self.goal.clone() else {
            return;
        };
        queries.cancel_actor(self.actor);
        let q = PathQuery::new(self.class, from, Goal::Point(goal), self.filter.clone());
        self.pending = Some(queries.request(self.actor, q, self.priority));
        self.mesh_changed_since_request = false;
    }
}

/// Steps [`NavCorridor`] components in `FixedUpdate` inside [`ControllerSystems::Drive`].
pub struct NavCorridorPlugin;

impl Plugin for NavCorridorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NavCorridorSettings::default())
            .configure_sets(
                FixedUpdate,
                NavSystems::Corridor
                    .in_set(ControllerSystems::Drive)
                    .after(NavSystems::Publish)
                    .before(NavSystems::Queries),
            )
            .add_systems(FixedUpdate, drive_corridors.in_set(NavSystems::Corridor));
        let _corners = app
            .add_tunable(&CORRIDOR_MAX_CORNERS)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.max_corners = v as usize);
        let _lookahead = app
            .add_tunable(&CORRIDOR_LOOKAHEAD_POLYS)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.lookahead_polys = v as usize);
        let _opt_range = app
            .add_tunable(&CORRIDOR_OPTIMIZE_RANGE_M)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.optimize_range_m = v);
        let _topo = app
            .add_tunable(&CORRIDOR_TOPOLOGY_PERIOD_S)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.topology_period_s = v);
        let _drift = app
            .add_tunable(&CORRIDOR_REPAIR_DRIFT_M)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.repair_drift_m = v);
        let _nodes = app
            .add_tunable(&CORRIDOR_REPAIR_NODES)
            .bind_resource(|s: &mut NavCorridorSettings, v| s.config.repair_nodes = v);
    }
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
fn drive_corridors(
    time: Res<Time>,
    fixed_time: Option<Res<Time<Fixed>>>,
    ctrl: Option<Res<ControllerSettings>>,
    settings: Res<NavCorridorSettings>,
    meshes: Option<Res<NavMeshes>>,
    mut queries: Option<ResMut<NavQueries>>,
    mut debug_paths: Option<ResMut<NavDebugPaths>>,
    mut movers: Query<(&Mover, &mut MoveIntent, &mut NavCorridor, Has<Dormant>, Has<Held>)>,
) {
    let Some(meshes) = meshes else { return };
    let Some(ref mut queries) = queries else {
        return;
    };
    // Tile loads/drops mutate `NavMeshes` through `ResMut` (residency, runtime patches).
    let mesh_changed = meshes.is_changed();
    let raw_dt = if time.delta_secs() > 0.0 {
        time.delta_secs()
    } else {
        fixed_time.map_or(1.0 / 60.0, |t| t.timestep().as_secs_f32())
    };
    let dt = ctrl.map_or(raw_dt, |c| raw_dt.min(c.max_dt));

    for (mover, mut intent, mut nav, dormant, held) in &mut movers {
        if mesh_changed && !nav.mesh_changed_since_request {
            nav.mesh_changed_since_request = true;
        }
        step_mover_corridor(
            mover,
            &mut intent,
            &mut nav,
            dormant || held,
            dt,
            &settings.config,
            &*meshes.0,
            queries,
            debug_paths.as_deref_mut(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn step_mover_corridor(
    mover: &Mover,
    intent: &mut MoveIntent,
    nav: &mut NavCorridor,
    inactive: bool,
    dt: f32,
    cfg: &CorridorConfig,
    backend: &dyn sf_nav::NavBackend,
    queries: &mut NavQueries,
    debug_paths: Option<&mut NavDebugPaths>,
) {
    if inactive {
        if let Some(id) = nav.pending.take() {
            queries.cancel(id);
        }
        return;
    }
    if nav.goal.is_none() {
        if let Some(id) = nav.pending.take() {
            queries.cancel(id);
        }
        nav.corridor = None;
        intent.0 = LocomotionIntent::IDLE;
        return;
    }

    if !poll_pending_reply(mover, intent, nav, queries, debug_paths) {
        return;
    }

    let from = Location::new(nav.worldspace.clone(), mover.loco.position);
    let needs_initial_or_replan = nav
        .corridor
        .as_ref()
        .is_none_or(PathCorridor::needs_replan);
    if !nav.arrived && needs_initial_or_replan && nav.may_request() {
        nav.request_path(queries, from.clone());
    }
    let may_replan = nav.may_request();

    if let Some(ref mut c) = nav.corridor {
        match c.step(backend, from.clone(), dt, cfg) {
            Ok(_) => {}
            Err(NavError::MeshChanged) => {
                c.mark_needs_replan();
            }
            Err(e) => {
                nav.failed = Some(e);
                nav.corridor = None;
                intent.0 = LocomotionIntent::IDLE;
                return;
            }
        }

        // Gated like the request above: a failed replan keeps following the valid prefix without
        // re-requesting every tick (request spam into the NV6 service) until a re-arm.
        if c.needs_replan()
            && may_replan
            && let Some(goal) = nav.goal.clone()
        {
            queries.cancel_actor(nav.actor);
            let q = PathQuery::new(nav.class, from, Goal::Point(goal), nav.filter.clone());
            nav.pending = Some(queries.request(nav.actor, q, nav.priority));
            nav.mesh_changed_since_request = false;
        }

        let stop_at_end = c.stop_at_end();
        let mut corners: Vec<WorldPos> = c.corners().iter().map(|l| l.pos).collect();
        if corners.is_empty() && stop_at_end {
            corners.push(c.target().pos);
        }
        nav.follower.set_corridor_corners(&corners, stop_at_end);
        let out = nav.follower.update(mover.loco.position, dt);
        if out.finished && stop_at_end {
            nav.arrived = true;
            nav.corridor = None;
            if let Some(id) = nav.pending.take() {
                queries.cancel(id);
            }
            intent.0 = LocomotionIntent::IDLE;
        } else {
            intent.0 = out.intent;
        }
    } else {
        intent.0 = LocomotionIntent::IDLE;
    }
}

fn poll_pending_reply(
    mover: &Mover,
    intent: &mut MoveIntent,
    nav: &mut NavCorridor,
    queries: &mut NavQueries,
    debug_paths: Option<&mut NavDebugPaths>,
) -> bool {
    let Some(id) = nav.pending else {
        return true;
    };
    let Some(reply) = queries.take(id) else {
        return true;
    };
    nav.pending = None;
    match reply.result {
        Ok(path) => {
            if let Some(goal) = nav.goal.clone() {
                let mut c = nav
                    .corridor
                    .take()
                    .unwrap_or_else(|| PathCorridor::new(nav.class, nav.filter.clone()));
                c.set_filter(nav.filter.clone());
                match c.load(goal, &path) {
                    Ok(()) => {
                        nav.corridor = Some(c);
                        nav.failed = None;
                        nav.follower = nav.follower.clone().for_mover(&mover.params);
                        if let Some(dbg) = debug_paths {
                            dbg.record(nav.actor.0.to_string(), &path);
                        }
                    }
                    Err(e) => {
                        nav.failed = Some(e);
                        intent.0 = LocomotionIntent::IDLE;
                        return false;
                    }
                }
            }
            true
        }
        Err(NavError::MeshChanged) => true,
        Err(e) => {
            nav.failed = Some(e);
            intent.0 = LocomotionIntent::IDLE;
            false
        }
    }
}
