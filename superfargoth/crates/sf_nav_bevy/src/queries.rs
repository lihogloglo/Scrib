//! The path query service on the fixed tick (nav.md "Routing", "Query service"; NV6):
//! [`NavQueryPlugin`] runs [`sf_nav::QueryService`] against [`NavMeshes`] once per fixed tick.
//!
//! - Callers (the AI's `MoveTo`, AI6) hold [`NavQueries`]: [`NavQueries::request`] stamps the
//!   current tick and returns the [`RequestId`]; [`NavQueries::take`] hands the reply over once it
//!   is published. Requests made before [`NavSystems::Queries`] are planned in the same tick.
//! - `FixedFirst`, [`NavSystems::Publish`]: the tick advances and the replies of the previous
//!   tick's run are published (the tick boundary), so every `FixedUpdate` system sees them.
//! - `FixedUpdate`, [`NavSystems::Queries`]: one run of the service, its planned slices on this
//!   thread, or with [`QUERY_PARALLEL`] across the compute pool (`ComputeTaskPool::scope`;
//!   the slices of a tick are independent, so the results are identical). The coverage hook is
//!   the baked ring ([`BakedNavRing`]) when [`crate::NavRingPlugin`] is in the app, else full
//!   coverage (no ring-edge rewrite).
//! - [`NavCounters`] get the queue, running jobs, this tick's jobs, expansions and replies, and
//!   the run's wall time (reported only, never used to decide).
//!
//! **Replies are a resource map, not a `Message`.** A caller polls its own [`RequestId`]; a reply
//! stays until taken or cancelled, so an actor whose AI runs on a staggered period cannot miss it,
//! whereas messages are dropped two updates after they are written and the fixed loop runs zero or
//! more times a frame. The map iterates in [`RequestId`] order, the order results apply in.
//!
//! **Scaffold `NavQueryOnMain`**: the slices run inside the fixed tick (on the compute pool at
//! most), not as a worker task overlapping the tick. Removal path: a worker task spawned at tick
//! `t` and joined at the boundary of `t + 1` (the service's ready rule already models that), once
//! the backend is shareable across ticks (`NavMeshes` is mutated by residency between ticks).

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use bevy_app::{App, FixedFirst, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use bevy_tasks::ComputeTaskPool;
use sf_ids::ActorKey;
use sf_nav::{
    FullCoverage, NavCoverage, PathPriority, PathQuery, PathReply, QueryBudget, QueryService,
    RequestId, TickBatch,
};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::{BakedNavRing, NavCounters, NavMeshes, NavSystems};

/// `ai.nav.query.nodes_per_slice`: [`QueryBudget::nodes_per_slice`]. Detour's sample steps a
/// sliced query by a few hundred iterations; a quarter of the tick allowance keeps four jobs
/// moving at once. Not measured.
pub const QUERY_NODES_PER_SLICE: Def<u32> = def!(count "ai.nav.query.nodes_per_slice" = 128)
    .range(1, 65_536)
    .unit(Unit::Count)
    .help("Path search expansions one job may take per tick at most")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.query.nodes_per_tick`: [`QueryBudget::nodes_per_tick`]. nav.md "Budgets": path
/// queries ≤ 0.30 ms (Floor) / 0.50 ms (High) per tick; at roughly 0.5-1 µs per Detour expansion
/// that is about 512. Not measured: calibrate from the `nav.stats` tick time.
pub const QUERY_NODES_PER_TICK: Def<u32> = def!(count "ai.nav.query.nodes_per_tick" = 512)
    .range(1, 1_048_576)
    .unit(Unit::Count)
    .help("Path search expansions per tick at most, over every job")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.query.jobs_per_tick`: [`QueryBudget::jobs_per_tick`]. The tick allowance over the
/// slice size. Not measured.
pub const QUERY_JOBS_PER_TICK: Def<u32> = def!(count "ai.nav.query.jobs_per_tick" = 4)
    .range(1, 1024)
    .unit(Unit::Count)
    .help("Path jobs stepped per tick at most (one slice each)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.query.age_boost_ticks`: [`QueryBudget::age_boost_ticks`]. Half a second at 60 Hz per
/// priority level: a background request reaches the top within 1.5 s. Not measured.
pub const QUERY_AGE_BOOST_TICKS: Def<u32> = def!(count "ai.nav.query.age_boost_ticks" = 30)
    .range(0, 3600)
    .unit(Unit::Count)
    .help("Fixed ticks a path request waits per priority level it gains (0: no ageing)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.query.edge_radius_m`: [`QueryBudget::edge_radius_m`]. Half a typical 8 m tile: a
/// partial path ends within this distance of where the line to the goal leaves the resident
/// tiles. Not measured.
pub const QUERY_EDGE_RADIUS_M: Def<f32> = def!(float "ai.nav.query.edge_radius_m" = 4.0)
    .range(0.5, 64.0)
    .unit(Unit::Meters)
    .help("A path to a goal beyond the resident navmesh ends within this distance of its edge")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `ai.nav.query.parallel`: run a tick's slices across the compute pool. Off until an
/// interleaved A/B says it wins (a scope per tick costs more than a few short slices save).
pub const QUERY_PARALLEL: Def<bool> = def!(bool "ai.nav.query.parallel" = false)
    .help("Path query slices of a tick run in parallel on the compute pool (identical results)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .lifetime(Lifetime::Live);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &QUERY_NODES_PER_SLICE,
    &QUERY_NODES_PER_TICK,
    &QUERY_JOBS_PER_TICK,
    &QUERY_AGE_BOOST_TICKS,
    &QUERY_EDGE_RADIUS_M,
    &QUERY_PARALLEL,
];

/// The service's caps and execution (bound from the tunables).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct NavQuerySettings {
    pub budget: QueryBudget,
    pub parallel: bool,
}

impl Default for NavQuerySettings {
    fn default() -> Self {
        Self {
            budget: QueryBudget {
                nodes_per_slice: QUERY_NODES_PER_SLICE.default_value(),
                nodes_per_tick: QUERY_NODES_PER_TICK.default_value(),
                jobs_per_tick: QUERY_JOBS_PER_TICK.default_value(),
                age_boost_ticks: QUERY_AGE_BOOST_TICKS.default_value(),
                edge_radius_m: QUERY_EDGE_RADIUS_M.default_value(),
            },
            parallel: QUERY_PARALLEL.default_value(),
        }
    }
}

/// The query service, the fixed tick it is at, and the published replies.
#[derive(Resource)]
pub struct NavQueries {
    /// A `Mutex` only to make the resource `Sync` (a job's search state is `Send`, not
    /// `Sync`): `&mut` access goes through `get_mut`, never locking.
    service: Mutex<QueryService>,
    tick: u64,
    ready: BTreeMap<RequestId, PathReply>,
}

impl NavQueries {
    pub fn new(budget: QueryBudget) -> Self {
        Self {
            service: Mutex::new(QueryService::new(budget)),
            tick: 0,
            ready: BTreeMap::new(),
        }
    }

    /// The current fixed tick (advanced at each tick's start; 0 before the first).
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// Queues a path request at the current tick.
    pub fn request(&mut self, actor: ActorKey, query: PathQuery, priority: PathPriority) -> RequestId {
        let tick = self.tick;
        self.svc().request(actor, query, priority, tick)
    }

    /// Takes a published reply (`None`: not published yet, or unknown).
    pub fn take(&mut self, id: RequestId) -> Option<PathReply> {
        self.ready.remove(&id)
    }

    /// A published reply, left in place.
    pub fn get(&self, id: RequestId) -> Option<&PathReply> {
        self.ready.get(&id)
    }

    /// Published replies not taken yet, in [`RequestId`] order.
    pub fn published(&self) -> impl Iterator<Item = &PathReply> {
        self.ready.values()
    }

    /// Queued or running (not published yet).
    pub fn is_pending(&self, id: RequestId) -> bool {
        self.service
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .is_pending(id)
    }

    /// Drops a request or its reply; false when unknown (taken already).
    pub fn cancel(&mut self, id: RequestId) -> bool {
        self.ready.remove(&id).is_some() || self.svc().cancel(id)
    }

    /// Drops every request and reply of `actor` (despawned, or a new plan); how many.
    pub fn cancel_actor(&mut self, actor: ActorKey) -> usize {
        let before = self.ready.len();
        self.ready.retain(|id, _| id.actor != actor);
        before - self.ready.len() + self.svc().cancel_actor(actor)
    }

    fn svc(&mut self) -> &mut QueryService {
        self.service.get_mut().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Runs the query service on the fixed tick (module docs). Add with [`crate::NavPlugin`] and a
/// [`NavMeshes`] backend; the app's fixed timestep drives it.
pub struct NavQueryPlugin;

impl Plugin for NavQueryPlugin {
    fn build(&self, app: &mut App) {
        let settings = NavQuerySettings::default();
        app.insert_resource(NavQueries::new(settings.budget))
            .insert_resource(settings)
            .add_systems(FixedFirst, publish.in_set(NavSystems::Publish))
            .add_systems(FixedUpdate, run_queries.in_set(NavSystems::Queries));
        let _slice = app
            .add_tunable(&QUERY_NODES_PER_SLICE)
            .bind_resource(|s: &mut NavQuerySettings, v| s.budget.nodes_per_slice = v);
        let _tick = app
            .add_tunable(&QUERY_NODES_PER_TICK)
            .bind_resource(|s: &mut NavQuerySettings, v| s.budget.nodes_per_tick = v);
        let _jobs = app
            .add_tunable(&QUERY_JOBS_PER_TICK)
            .bind_resource(|s: &mut NavQuerySettings, v| s.budget.jobs_per_tick = v);
        let _age = app
            .add_tunable(&QUERY_AGE_BOOST_TICKS)
            .bind_resource(|s: &mut NavQuerySettings, v| s.budget.age_boost_ticks = v);
        let _edge = app
            .add_tunable(&QUERY_EDGE_RADIUS_M)
            .bind_resource(|s: &mut NavQuerySettings, v| s.budget.edge_radius_m = v);
        let _par = app
            .add_tunable(&QUERY_PARALLEL)
            .bind_resource(|s: &mut NavQuerySettings, v| s.parallel = v);
    }
}

/// The tick boundary: advance the tick, publish the replies of the previous tick's run.
fn publish(mut queries: ResMut<NavQueries>) {
    let q = &mut *queries;
    q.tick += 1;
    let tick = q.tick;
    let replies = q.svc().drain_ready(tick);
    q.ready.extend(replies.into_iter().map(|r| (r.id, r)));
}

fn run_queries(
    settings: Res<NavQuerySettings>,
    mut queries: ResMut<NavQueries>,
    meshes: Option<Res<NavMeshes>>,
    ring: Option<Res<BakedNavRing>>,
    mut counters: ResMut<NavCounters>,
) {
    let tick = queries.tick;
    let service = queries.svc();
    service.set_budget(settings.budget);
    let Some(meshes) = meshes else {
        // No backend: requests wait.
        counters.queue_depth = Some(service.queue_len());
        return;
    };
    let backend = &*meshes.0;
    let coverage: &dyn NavCoverage = match &ring {
        Some(r) => r.ring(),
        None => &FullCoverage,
    };
    let started = Instant::now();
    let mut batch = service.plan(tick, coverage);
    if settings.parallel && batch.len() > 1 {
        run_parallel(&mut batch, backend);
    } else {
        batch.run_serial(backend);
    }
    let report = service.commit(batch);
    counters.tick_ms = Some(started.elapsed().as_secs_f32() * 1000.0);
    counters.queue_depth = Some(service.queue_len());
    counters.running = Some(service.running());
    counters.jobs = Some(report.jobs);
    counters.nodes = Some(report.nodes);
    counters.results = Some(report.finished);
}

/// One task per slice on the compute pool (serial when there is no pool).
fn run_parallel(batch: &mut TickBatch, backend: &dyn sf_nav::NavBackend) {
    let Some(pool) = ComputeTaskPool::try_get() else {
        batch.run_serial(backend);
        return;
    };
    pool.scope(|s| {
        for slice in batch.slices_mut() {
            s.spawn(async move { slice.run(backend) });
        }
    });
}
