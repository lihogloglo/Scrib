//! The path query service (nav.md "Routing", "Query service"; NV6): requests in a priority queue
//! feed sliced [`PathJob`]s under a node budget, and results land at the next tick boundary in
//! [`RequestId`] order. Bevy-free: `sf_nav_bevy` runs it on the fixed tick against `NavMeshes`.
//!
//! - **Requests** are `(RequestId = (ActorKey, seq), class, from, goal, filter, priority, tick)`:
//!   [`QueryService::request`] stamps the id; `seq` is the service's submission counter, unique
//!   across actors. Not saved, not sent (nav.md "Co-op and saves": the AI re-requests after a
//!   load; host-only).
//! - **Order** (each tick): urgency first (the [`PathPriority`], raised one level per
//!   [`QueryBudget::age_boost_ticks`] waited, so an old background request ends up at the top and
//!   never starves), then the request tick (oldest first), then [`RequestId`].
//! - **Slicing** is by expansions only (Detour's `maxIter`), never by wall time: at most
//!   [`QueryBudget::jobs_per_tick`] jobs get one slice each, of at most
//!   [`QueryBudget::nodes_per_slice`] expansions, from a tick allowance of
//!   [`QueryBudget::nodes_per_tick`]. Allotments are planned before any job runs
//!   ([`QueryService::plan`]) and an allotment a job leaves unused is not handed on, so the jobs
//!   of one tick are independent: they may run serially or in parallel with identical results.
//!   The spec's time budget (≤ 0.30 ms Floor, ≤ 0.50 ms High per tick, nav.md "Budgets") is met by
//!   calibrating the node budgets against the measured time (reported, never used to decide: net
//!   rule 4, a result depends only on tiles, obstacles and query).
//! - **Tick boundary**: a job that finishes in tick `t`'s run is ready at `t + 1`
//!   ([`QueryService::drain_ready`]), the tick a worker run spawned at `t` would be joined.
//! - **The ring edge**: when the start is on resident tiles but the goal is not (per
//!   [`NavCoverage`]; [`NavRing`] implements it), the job paths to the last covered point on the
//!   straight line towards the goal (a [`Goal::Near`] of [`QueryBudget::edge_radius_m`]) and
//!   reports [`PathStatus::Partial`] with [`PathReply::at_ring_edge`]. The coarse route beyond the
//!   edge (nav.md: "plus the coarse route") waits for the coarse graph (NV12). An [`Goal::AnyOf`]
//!   keeps only its covered candidates when some are covered. [`Goal::Away`] is never rewritten.
//! - **Mesh changes**: a tile a running job touched that is removed fails the job with
//!   [`NavError::MeshChanged`] (the backend's rule); the reply carries the error, the AI
//!   re-requests.
//! - **Inline mode** ([`QueryService::run_inline`]): the same planning and stepping with no caps,
//!   every result returned at once (tests, tools).

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use glam::Vec3;
use sf_ids::ActorKey;
use sf_world::Location;

use crate::{
    AgentClassId, Goal, JobState, NavBackend, NavError, NavRing, PathJob, PathQuery, PathResult,
    PathStatus, SliceBudget, TileState,
};

/// Bisection steps for the ring-edge point: 2^-20 of the start-to-goal segment (1 mm on 1 km).
// tunables: const (maths: bisection precision, far below any tile or snap size)
const EDGE_BISECT_STEPS: u32 = 20;

/// A request's id: the actor that asked and the service's submission counter. Results apply in
/// this order (actor, then submission).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestId {
    pub actor: ActorKey,
    pub seq: u64,
}

/// How urgent a request is, least first. nav.md: combat and player-visible first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathPriority {
    /// Off-screen, low-tier actors (schedules, wandering).
    Background,
    Normal,
    /// The player can see the actor.
    Visible,
    /// In or entering combat.
    Combat,
}

impl PathPriority {
    fn level(self) -> u64 {
        match self {
            Self::Background => 0,
            Self::Normal => 1,
            Self::Visible => 2,
            Self::Combat => 3,
        }
    }
}

/// The service's per-tick caps (the host binds them from tunables, `sf_nav_bevy`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueryBudget {
    /// Expansions per job slice at most (Detour's `maxIter`).
    pub nodes_per_slice: u32,
    /// Expansions per tick at most, over every job.
    pub nodes_per_tick: u32,
    /// Jobs stepped per tick at most (one slice each).
    pub jobs_per_tick: u32,
    /// Ticks waited per priority level gained (0: no ageing).
    pub age_boost_ticks: u32,
    /// The ring-edge goal's radius, meters: how far from the last covered point on the line to
    /// the goal the partial path may end (also the snap extent there).
    pub edge_radius_m: f32,
}

/// Which locations lie on tiles whose navmesh content is known: the ring edge's hook.
pub trait NavCoverage {
    fn covers(&self, class: AgentClassId, at: &Location) -> bool;
}

/// Everything is covered (no residency: tests, tools, a fully loaded mesh): no ring-edge rewrite.
#[derive(Clone, Copy, Debug, Default)]
pub struct FullCoverage;

impl NavCoverage for FullCoverage {
    fn covers(&self, _class: AgentClassId, _at: &Location) -> bool {
        true
    }
}

/// Closures (tests, hosts with their own residency).
impl<F: Fn(AgentClassId, &Location) -> bool> NavCoverage for F {
    fn covers(&self, class: AgentClassId, at: &Location) -> bool {
        self(class, at)
    }
}

/// The ring covers a location when the class's tile there was read: resident, or baked empty (a
/// goal there is off the mesh, not beyond the edge). Pending, unbaked and failed tiles are not
/// covered.
impl NavCoverage for NavRing {
    fn covers(&self, class: AgentClassId, at: &Location) -> bool {
        self.layout()
            .tiles_within(at, 0.0)
            .iter()
            .filter(|k| k.class == class)
            .any(|k| matches!(self.state(k), Some(TileState::Resident | TileState::Empty)))
    }
}

/// A finished request.
#[derive(Clone, Debug, PartialEq)]
pub struct PathReply {
    pub id: RequestId,
    pub result: Result<PathResult, NavError>,
    /// The tick it was requested at.
    pub requested: u64,
    /// The tick whose run finished it (ready at the next boundary).
    pub finished: u64,
    /// Expansions over every slice.
    pub iterations: u32,
    /// Slices it took.
    pub slices: u32,
    /// The goal was beyond the covered tiles: the path ends at the ring edge (`Partial`).
    pub at_ring_edge: bool,
}

/// What one tick's run did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TickReport {
    pub jobs: u32,
    pub nodes: u32,
    pub finished: u32,
}

/// Totals since the service was made.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QueryTotals {
    pub requests: u64,
    pub finished: u64,
    pub cancelled: u64,
    pub nodes: u64,
}

enum Work {
    Waiting(PathQuery),
    Running(PathJob),
}

struct Pending {
    priority: PathPriority,
    tick: u64,
    work: Work,
    iterations: u32,
    slices: u32,
    at_ring_edge: bool,
}

/// One job's slice in a [`TickBatch`]: run it once with [`Slice::run`], on any thread.
pub struct Slice {
    id: RequestId,
    pending: Pending,
    budget: SliceBudget,
    state: Option<JobState>,
}

impl Slice {
    pub fn id(&self) -> RequestId {
        self.id
    }

    /// Steps the job once within its allotment.
    pub fn run(&mut self, backend: &dyn NavBackend) {
        if self.state.is_some() {
            return;
        }
        let Work::Running(job) = &mut self.pending.work else {
            unreachable!("planned slices hold running jobs");
        };
        self.state = Some(backend.find_path(job, &mut self.budget));
    }
}

/// One tick's planned slices ([`QueryService::plan`]), independent of each other.
pub struct TickBatch {
    tick: u64,
    ready_at: u64,
    slices: Vec<Slice>,
}

impl TickBatch {
    /// The slices, in schedule order.
    pub fn slices_mut(&mut self) -> &mut [Slice] {
        &mut self.slices
    }

    pub fn len(&self) -> usize {
        self.slices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slices.is_empty()
    }

    /// Runs every slice in order on this thread.
    pub fn run_serial(&mut self, backend: &dyn NavBackend) {
        for s in &mut self.slices {
            s.run(backend);
        }
    }
}

/// The query service: the queue, the running jobs and the results waiting for their boundary.
pub struct QueryService {
    budget: QueryBudget,
    next_seq: u64,
    pending: BTreeMap<RequestId, Pending>,
    /// Ids planned into the open batch, and those cancelled while it was open.
    open: BTreeSet<RequestId>,
    cancelled_open: BTreeSet<RequestId>,
    ready: Vec<(u64, PathReply)>,
    totals: QueryTotals,
}

impl QueryService {
    pub fn new(budget: QueryBudget) -> Self {
        Self {
            budget,
            next_seq: 0,
            pending: BTreeMap::new(),
            open: BTreeSet::new(),
            cancelled_open: BTreeSet::new(),
            ready: Vec::new(),
            totals: QueryTotals::default(),
        }
    }

    pub fn budget(&self) -> QueryBudget {
        self.budget
    }

    /// New caps, from the next plan on (running jobs keep their search state).
    pub fn set_budget(&mut self, budget: QueryBudget) {
        self.budget = budget;
    }

    /// Queues a path request made at `tick`.
    pub fn request(
        &mut self,
        actor: ActorKey,
        query: PathQuery,
        priority: PathPriority,
        tick: u64,
    ) -> RequestId {
        let id = RequestId {
            actor,
            seq: self.next_seq,
        };
        self.next_seq += 1;
        self.totals.requests += 1;
        self.pending.insert(
            id,
            Pending {
                priority,
                tick,
                work: Work::Waiting(query),
                iterations: 0,
                slices: 0,
                at_ring_edge: false,
            },
        );
        id
    }

    /// Drops a queued or running request, or a result not yet drained: no reply will come.
    /// False when the id is unknown (already drained, or never made).
    pub fn cancel(&mut self, id: RequestId) -> bool {
        let found = if self.pending.remove(&id).is_some() {
            true
        } else if self.open.contains(&id) {
            self.cancelled_open.insert(id)
        } else if let Some(i) = self.ready.iter().position(|(_, r)| r.id == id) {
            self.ready.remove(i);
            true
        } else {
            false
        };
        if found {
            self.totals.cancelled += 1;
        }
        found
    }

    /// Cancels every request of `actor` (it despawned or changed its mind); how many.
    pub fn cancel_actor(&mut self, actor: ActorKey) -> usize {
        let mut ids: Vec<RequestId> = self
            .pending
            .keys()
            .chain(&self.open)
            .copied()
            .chain(self.ready.iter().map(|(_, r)| r.id))
            .filter(|id| id.actor == actor)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids.into_iter().filter(|id| self.cancel(*id)).count()
    }

    /// Requests not finished yet (queued or running).
    pub fn queue_len(&self) -> usize {
        self.pending.len() + self.open.len() - self.cancelled_open.len()
    }

    /// Requests with a running search.
    pub fn running(&self) -> usize {
        self.pending
            .values()
            .filter(|p| matches!(p.work, Work::Running(_)))
            .count()
            + self.open.len()
            - self.cancelled_open.len()
    }

    /// Finished results waiting to be drained.
    pub fn ready_len(&self) -> usize {
        self.ready.len()
    }

    pub fn is_pending(&self, id: RequestId) -> bool {
        self.pending.contains_key(&id)
            || (self.open.contains(&id) && !self.cancelled_open.contains(&id))
    }

    pub fn totals(&self) -> QueryTotals {
        self.totals
    }

    /// Picks this tick's jobs and their allotments (module docs, "Order" and "Slicing"), starting
    /// waiting ones (the ring-edge rewrite happens here, against `coverage` as it is now). Run
    /// the batch's slices, then [`QueryService::commit`] it in the same tick.
    pub fn plan(&mut self, tick: u64, coverage: &dyn NavCoverage) -> TickBatch {
        self.plan_with(tick, coverage, false)
    }

    fn plan_with(&mut self, tick: u64, coverage: &dyn NavCoverage, unbounded: bool) -> TickBatch {
        assert!(self.open.is_empty(), "a tick batch is still open");
        let b = self.budget;
        let mut order: Vec<(Reverse<u64>, u64, RequestId)> = self
            .pending
            .iter()
            .map(|(id, p)| (Reverse(Self::urgency(b, p, tick)), p.tick, *id))
            .collect();
        order.sort_unstable();
        let jobs = if unbounded {
            order.len()
        } else {
            usize::try_from(b.jobs_per_tick).unwrap_or(usize::MAX)
        };
        let mut left = b.nodes_per_tick;
        let mut slices = Vec::new();
        for (_, _, id) in order.into_iter().take(jobs) {
            let budget = if unbounded {
                SliceBudget::unlimited()
            } else {
                let n = b.nodes_per_slice.min(left);
                if n == 0 {
                    break;
                }
                left -= n;
                SliceBudget::new(n)
            };
            let mut pending = self.pending.remove(&id).expect("ordered ids are pending");
            if let Work::Waiting(q) = &pending.work {
                let (q, edge) = edge_query(q, coverage, b.edge_radius_m);
                pending.at_ring_edge = edge;
                pending.work = Work::Running(PathJob::new(q));
            }
            self.open.insert(id);
            slices.push(Slice {
                id,
                pending,
                budget,
                state: None,
            });
        }
        TickBatch {
            tick,
            ready_at: if unbounded { tick } else { tick + 1 },
            slices,
        }
    }

    fn urgency(b: QueryBudget, p: &Pending, tick: u64) -> u64 {
        let boost = match b.age_boost_ticks {
            0 => 0,
            n => tick.saturating_sub(p.tick) / u64::from(n),
        };
        (p.priority.level() + boost).min(PathPriority::Combat.level())
    }

    /// Takes a planned batch back: finished jobs become replies (ready at the next boundary),
    /// the others wait for their next slice.
    pub fn commit(&mut self, batch: TickBatch) -> TickReport {
        let mut report = TickReport::default();
        for s in batch.slices {
            self.open.remove(&s.id);
            if self.cancelled_open.remove(&s.id) {
                continue;
            }
            let Slice {
                id,
                mut pending,
                budget,
                state,
            } = s;
            let used = budget.used();
            report.nodes += used;
            self.totals.nodes += u64::from(used);
            let Some(state) = state else {
                // Never run: no slice spent, it keeps its place.
                self.pending.insert(id, pending);
                continue;
            };
            report.jobs += 1;
            pending.iterations += used;
            pending.slices += 1;
            let result = match state {
                JobState::InProgress => {
                    self.pending.insert(id, pending);
                    continue;
                }
                JobState::Done(mut r) => {
                    if pending.at_ring_edge {
                        r.status = PathStatus::Partial;
                    }
                    Ok(r)
                }
                JobState::Failed(e) => Err(e),
            };
            report.finished += 1;
            self.totals.finished += 1;
            self.ready.push((
                batch.ready_at,
                PathReply {
                    id,
                    result,
                    requested: pending.tick,
                    finished: batch.tick,
                    iterations: pending.iterations,
                    slices: pending.slices,
                    at_ring_edge: pending.at_ring_edge,
                },
            ));
        }
        self.cancelled_open.clear();
        report
    }

    /// One tick on this thread: plan, run every slice, commit.
    pub fn run(
        &mut self,
        backend: &dyn NavBackend,
        coverage: &dyn NavCoverage,
        tick: u64,
    ) -> TickReport {
        let mut batch = self.plan(tick, coverage);
        batch.run_serial(backend);
        self.commit(batch)
    }

    /// The replies ready at `tick` (finished in an earlier tick's run), in [`RequestId`] order.
    pub fn drain_ready(&mut self, tick: u64) -> Vec<PathReply> {
        let (mut out, keep): (Vec<_>, Vec<_>) =
            self.ready.drain(..).partition(|(at, _)| *at <= tick);
        self.ready = keep;
        out.sort_unstable_by_key(|(_, r)| r.id);
        out.into_iter().map(|(_, r)| r).collect()
    }

    /// Inline mode: every queued request to the end now, with no caps (one unbounded slice each,
    /// the same planning and stepping), and every reply ready at `tick`, in [`RequestId`] order.
    pub fn run_inline(
        &mut self,
        backend: &dyn NavBackend,
        coverage: &dyn NavCoverage,
        tick: u64,
    ) -> Vec<PathReply> {
        while !self.pending.is_empty() {
            let mut batch = self.plan_with(tick, coverage, true);
            batch.run_serial(backend);
            self.commit(batch);
        }
        self.drain_ready(tick)
    }
}

/// The query a job runs: the ring-edge rewrite when the start is covered and the goal is not.
fn edge_query(q: &PathQuery, cov: &dyn NavCoverage, edge_radius: f32) -> (PathQuery, bool) {
    let unchanged = (q.clone(), false);
    if q.goal.check_worldspace(&q.from.worldspace).is_err() || !cov.covers(q.class, &q.from) {
        return unchanged; // the backend reports it
    }
    let (target, radius) = match &q.goal {
        Goal::Point(l) => (l, 0.0),
        Goal::Near { at, radius } => (at, *radius),
        Goal::AnyOf(ls) if ls.is_empty() => return unchanged,
        Goal::AnyOf(ls) => {
            let covered: Vec<Location> = ls
                .iter()
                .filter(|l| cov.covers(q.class, l))
                .cloned()
                .collect();
            if covered.len() == ls.len() {
                return unchanged;
            }
            if !covered.is_empty() {
                let mut q = q.clone();
                q.goal = Goal::AnyOf(covered);
                return (q, false);
            }
            let nearest = ls
                .iter()
                .min_by(|a, b| {
                    let d = |l: &Location| q.from.pos.delta(l.pos).length_squared();
                    d(a).total_cmp(&d(b))
                })
                .expect("not empty");
            (nearest, 0.0)
        }
        Goal::Away { .. } => return unchanged,
    };
    if cov.covers(q.class, target) {
        return unchanged;
    }
    let mut out = q.clone();
    let r = edge_radius.max(0.0);
    out.goal = Goal::Near {
        at: edge_point(cov, q.class, &q.from, target),
        radius: r.max(radius),
    };
    out.extent = q.extent.max(Vec3::splat(r));
    (out, true)
}

/// The last covered point on the straight line from `from` (covered) towards `to` (not).
fn edge_point(
    cov: &dyn NavCoverage,
    class: AgentClassId,
    from: &Location,
    to: &Location,
) -> Location {
    let d = from.pos.delta_f32(to.pos);
    let at = |t: f32| Location::new(from.worldspace.clone(), from.pos.offset(d * t));
    let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
    for _ in 0..EDGE_BISECT_STEPS {
        let mid = f32::midpoint(lo, hi);
        if cov.covers(class, &at(mid)) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    at(lo)
}
