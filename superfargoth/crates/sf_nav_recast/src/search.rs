//! Sliced path jobs: [`Goal`]s on Detour's searches. The job's scratch holds the search state;
//! every step re-finds its mesh (by slot and serial) and attaches the state for that step only.
//!
//! - `Point`: one sliced A* (`initSlicedFindPath` / `updateSlicedFindPath` /
//!   `finalizeSlicedFindPath`), then `findStraightPath` with a vertex at every area change.
//!   Unreachable: Detour's partial result (the visited polygon nearest the goal).
//! - `Near { at, radius }`: the `Point` search to `at` (snapped with the extent widened to the
//!   radius), its corridor trimmed where the string-pulled path first comes within `radius` of
//!   `at` (UE's acceptance radius, applied to the path). Complete when that point exists.
//! - `AnyOf`: one A* per candidate in order of straight distance, skipping candidates whose
//!   straight distance times the cheapest area cost already exceeds the best complete path:
//!   the industry's per-item path test (UE EQS pathing) with an admissible prune. Detour's A* has
//!   one end polygon, so a multi-goal A* would be a deeper patch for little gain at slot counts.
//! - `Away { from, min_dist }`: navmesh-rs's resumable Dijkstra (`findPolysAroundCircle`,
//!   self-owned) from the start, one polygon per budget unit, until the first polygon whose
//!   centroid is `min_dist` from the threat: the cheapest such polygon. Exhausted: the farthest.
//!
//! Path cost is the string-pulled path's length per segment times that segment's area cost.

use glam::Vec3;
use navmesh_rs::detour::{
    self, DetachedQuery, NavMeshQuery, PolysAroundSearch, STRAIGHTPATH_ALL_CROSSINGS,
    STRAIGHTPATH_AREA_CROSSINGS, Status, StraightPathPoint,
};
use navmesh_rs::{Budget, Progress};
use sf_nav::{
    AreaId, Goal, JobState, NavError, PathJob, PathQuery, PathResult, PathStatus, QueryFilter,
    SliceBudget,
};

use crate::filter::DtFilter;
use crate::{Mesh, RecastBackend, backend};

/// The Dijkstra's circle for `Away`: large enough to never be the limit (the node pool is).
const AWAY_RADIUS: f32 = 32_000.0;

struct Search {
    slot: usize,
    serial: u64,
    start_ref: detour::PolyRef,
    start_pos: [f32; 3],
    kind: Kind,
}

/// A point goal snapped onto the mesh.
#[derive(Clone, Copy)]
struct End {
    r: detour::PolyRef,
    pos: [f32; 3],
}

enum Kind {
    Point {
        q: Option<DetachedQuery>,
        end: End,
        near: Option<(Vec3, f32)>,
    },
    AnyOf {
        q: Option<DetachedQuery>,
        /// Candidates by straight distance, the next one, and whether its search is running.
        ends: Vec<(f32, End)>,
        next: usize,
        running: bool,
        best: Option<PathResult>,
        /// The best partial so far and its end's distance to its goal.
        fallback: Option<(f32, PathResult)>,
    },
    Away {
        s: Box<PolysAroundSearch>,
        from: Vec3,
        min_dist: f32,
        /// The farthest expanded polygon so far: (distance, ref, centroid).
        far: Option<(f32, detour::PolyRef, [f32; 3])>,
    },
}

pub(crate) fn find_path(
    b: &RecastBackend,
    job: &mut PathJob,
    budget: &mut SliceBudget,
) -> JobState {
    if job.is_finished() {
        return JobState::Failed(NavError::JobFinished);
    }
    let search = match job
        .scratch_mut()
        .take()
        .map(Box::<dyn std::any::Any + Send>::downcast::<Search>)
    {
        Some(Ok(s)) => Ok(*s),
        Some(Err(_)) => Err(NavError::Unsupported("a job started on another backend")),
        None => begin(b, job.query()),
    };
    let mut spent = 0;
    let out = match search {
        Ok(mut s) => {
            let r = step(b, &mut s, job.query(), budget, &mut spent);
            if matches!(r, Ok(None)) {
                *job.scratch_mut() = Some(Box::new(s));
            } else {
                release(b, s);
            }
            r
        }
        Err(e) => Err(e),
    };
    job.add_iterations(spent);
    match out {
        Ok(None) => JobState::InProgress,
        Ok(Some(r)) => {
            job.finish();
            JobState::Done(r)
        }
        Err(e) => {
            job.finish();
            JobState::Failed(e)
        }
    }
}

/// Returns a finished search's detached query to the pool.
fn release(b: &RecastBackend, s: Search) {
    match s.kind {
        Kind::Point { q: Some(q), .. } | Kind::AnyOf { q: Some(q), .. } => b.give_query(q),
        _ => {}
    }
}

fn begin(b: &RecastBackend, q: &PathQuery) -> Result<Search, NavError> {
    q.goal.check_worldspace(&q.from.worldspace)?;
    let (m, start_ref, start_pos) = b
        .nearest_in(&q.from, q.class, q.extent, &q.filter, None)
        .ok_or(NavError::StartOffMesh)?;
    let snap = |at: &sf_world::Location, extent: Vec3| {
        b.nearest_in(at, q.class, extent, &q.filter, Some(m.tag.slot))
            .map(|(_, r, pos)| End { r, pos })
    };
    let f = DtFilter(&q.filter);
    let kind = match &q.goal {
        Goal::Point(at) => {
            let end = snap(at, q.extent).ok_or(NavError::GoalOffMesh)?;
            Kind::Point {
                q: Some(init(b, m, start_ref, start_pos, end, &f)?),
                end,
                near: None,
            }
        }
        Goal::Near { at, radius } => {
            let r = radius.max(0.0);
            let wide = q.extent.max(Vec3::new(r, q.extent.y, r));
            let end = snap(at, wide).ok_or(NavError::GoalOffMesh)?;
            Kind::Point {
                q: Some(init(b, m, start_ref, start_pos, end, &f)?),
                end,
                near: Some((Vec3::from_array(m.local(at.pos)), r)),
            }
        }
        Goal::AnyOf(ls) if ls.is_empty() => return Err(NavError::EmptyGoal),
        Goal::AnyOf(ls) => {
            let s = Vec3::from_array(start_pos);
            let mut ends: Vec<(f32, End)> = ls
                .iter()
                .filter_map(|l| snap(l, q.extent))
                .map(|e| (s.distance(Vec3::from_array(e.pos)), e))
                .collect();
            if ends.is_empty() {
                return Err(NavError::GoalOffMesh);
            }
            ends.sort_by(|a, b| a.0.total_cmp(&b.0));
            Kind::AnyOf {
                q: Some(b.take_query(&m.nav)?),
                ends,
                next: 0,
                running: false,
                best: None,
                fallback: None,
            }
        }
        Goal::Away { from, min_dist } => {
            let s = b
                .with_query(m, |dq| {
                    dq.begin_polys_around_circle(
                        start_ref,
                        &start_pos,
                        AWAY_RADIUS,
                        usize::from(b.cfg.max_nodes),
                    )
                })?
                .map_err(backend)?;
            Kind::Away {
                s: Box::new(s),
                from: Vec3::from_array(m.local(from.pos)),
                min_dist: *min_dist,
                far: None,
            }
        }
    };
    Ok(Search {
        slot: m.tag.slot,
        serial: m.serial,
        start_ref,
        start_pos,
        kind,
    })
}

fn init(
    b: &RecastBackend,
    m: &Mesh,
    start_ref: detour::PolyRef,
    start_pos: [f32; 3],
    end: End,
    f: &DtFilter<'_>,
) -> Result<DetachedQuery, NavError> {
    let mut q = b.take_query(&m.nav)?.attach(&m.nav);
    let r = q.init_sliced_find_path(start_ref, end.r, &start_pos, &end.pos, f, 0);
    let dq = q.detach();
    match r {
        Ok(_) => Ok(dq),
        Err(e) => {
            b.give_query(dq);
            Err(backend(e))
        }
    }
}

/// `Ok(None)`: out of budget, step again.
fn step(
    b: &RecastBackend,
    s: &mut Search,
    query: &PathQuery,
    budget: &mut SliceBudget,
    spent: &mut u32,
) -> Result<Option<PathResult>, NavError> {
    let m = b
        .mesh_by_serial(s.slot, s.serial)
        .ok_or(NavError::MeshChanged)?;
    let f = DtFilter(&query.filter);
    let ctx = Ctx {
        b,
        m,
        filter: &query.filter,
        start_ref: s.start_ref,
        start_pos: s.start_pos,
    };
    match &mut s.kind {
        Kind::Point { q, end, near } => {
            if !slice(m, q, &f, budget, spent)? {
                return Ok(None);
            }
            ctx.finish(q, &f, *end, *near).map(Some)
        }
        Kind::AnyOf {
            q,
            ends,
            next,
            running,
            best,
            fallback,
        } => loop {
            if !*running {
                // Skip candidates that cannot beat the best complete path.
                let h = query.filter.min_cost();
                while *next < ends.len()
                    && best.as_ref().is_some_and(|p| ends[*next].0 * h >= p.cost)
                {
                    *next += 1;
                }
                if *next == ends.len() {
                    return match (best.take(), fallback.take()) {
                        (Some(p), _) | (None, Some((_, p))) => Ok(Some(p)),
                        (None, None) => Err(NavError::GoalOffMesh),
                    };
                }
                let end = ends[*next].1;
                let mut aq = q.take().expect("any-of keeps its query").attach(&m.nav);
                let r = aq.init_sliced_find_path(s.start_ref, end.r, &s.start_pos, &end.pos, &f, 0);
                *q = Some(aq.detach());
                r.map_err(backend)?;
                *running = true;
            }
            if !slice(m, q, &f, budget, spent)? {
                return Ok(None);
            }
            let end = ends[*next].1;
            let p = ctx.finish(q, &f, end, None)?;
            *running = false;
            *next += 1;
            if p.status == PathStatus::Complete {
                if best.as_ref().is_none_or(|o| p.cost < o.cost) {
                    *best = Some(p);
                }
            } else {
                let gap = p.waypoints.last().map_or(f32::INFINITY, |l| {
                    Vec3::from_array(m.local(l.pos)).distance(Vec3::from_array(end.pos))
                });
                if fallback.as_ref().is_none_or(|(g, _)| gap < *g) {
                    *fallback = Some((gap, p));
                }
            }
        },
        Kind::Away {
            s: dij,
            from,
            min_dist,
            far,
        } => b.with_query(m, |dq| {
            let Some((end_ref, end_pos, status)) =
                away_slice(dq, dij, *from, *min_dist, far, &f, budget, spent)?
            else {
                return Ok(None);
            };
            let mut corridor = Vec::new();
            dij.path_to(dq, end_ref, &mut corridor, b.cfg.max_path)
                .map_err(backend)?;
            ctx.result(dq, &corridor, end_pos, status, None).map(Some)
        })?,
    }
}

struct Ctx<'a> {
    b: &'a RecastBackend,
    m: &'a Mesh,
    filter: &'a QueryFilter,
    start_ref: detour::PolyRef,
    start_pos: [f32; 3],
}

/// Runs the sliced A* within the budget: true once it has finished.
fn slice(
    m: &Mesh,
    q: &mut Option<DetachedQuery>,
    f: &DtFilter<'_>,
    budget: &mut SliceBudget,
    spent: &mut u32,
) -> Result<bool, NavError> {
    let mut aq = q.take().expect("A* jobs keep their query").attach(&m.nav);
    let r = aq.update_sliced_find_path(budget.remaining() as usize, f);
    *q = Some(aq.detach());
    let (st, n) = r.map_err(backend)?;
    #[allow(
        clippy::cast_possible_truncation,
        reason = "at most the budget's u32 remaining"
    )]
    let n = n as u32;
    budget.charge(n);
    *spent += n;
    Ok(!st.is_in_progress())
}

impl Ctx<'_> {
    /// Finalizes the job's finished A* (Detour's partial result when the end was not reached)
    /// and builds the result with the same query.
    fn finish(
        &self,
        q: &mut Option<DetachedQuery>,
        f: &DtFilter<'_>,
        end: End,
        near: Option<(Vec3, f32)>,
    ) -> Result<PathResult, NavError> {
        let mut aq = q
            .take()
            .expect("A* jobs keep their query")
            .attach(&self.m.nav);
        let mut corridor = Vec::new();
        let r = aq
            .finalize_sliced_find_path(f, &mut corridor, self.b.cfg.max_path)
            .map_err(backend)
            .and_then(|st| {
                let partial = st.has_detail(Status::PARTIAL_RESULT)
                    || st.has_detail(Status::BUFFER_TOO_SMALL);
                let end_pos = match corridor.last() {
                    Some(&last) if partial => aq
                        .closest_point_on_poly(last, &end.pos)
                        .map_or(end.pos, |(p, _)| p),
                    _ => end.pos,
                };
                let status = if partial {
                    PathStatus::Partial
                } else {
                    PathStatus::Complete
                };
                self.result(&aq, &corridor, end_pos, status, near)
            });
        *q = Some(aq.detach());
        r
    }

    /// String-pulls `corridor` from the start to `end_pos` and prices it.
    fn result(
        &self,
        q: &NavMeshQuery<'_>,
        corridor: &[detour::PolyRef],
        end_pos: [f32; 3],
        status: PathStatus,
        near: Option<(Vec3, f32)>,
    ) -> Result<PathResult, NavError> {
        if corridor.is_empty() {
            return Err(NavError::Backend("an empty path corridor".into()));
        }
        let opts = if near.is_some() {
            STRAIGHTPATH_ALL_CROSSINGS
        } else {
            STRAIGHTPATH_AREA_CROSSINGS
        };
        let mut pts: Vec<StraightPathPoint> = Vec::new();
        let max = corridor.len() * 2 + 8;
        q.find_straight_path(&self.start_pos, &end_pos, corridor, &mut pts, max, opts)
            .map_err(backend)?;
        if pts.is_empty() {
            return Err(NavError::Backend("an empty straight path".into()));
        }
        let mut polys = corridor.to_vec();
        let mut status = status;
        if let Some((at, radius)) = near {
            if let Some(seg) = trim(&mut pts, at, radius) {
                // The cut lies in the polygon its segment starts in.
                let cut_poly = pts[seg].r#ref;
                if let Some(i) = polys.iter().position(|&p| p == cut_poly) {
                    polys.truncate(i + 1);
                }
                status = PathStatus::Complete;
            } else {
                status = PathStatus::Partial;
            }
        }
        let area = |r: detour::PolyRef| {
            self.m
                .nav
                .poly_area(r)
                .map_or(f32::INFINITY, |a| self.filter.area_cost(AreaId(a)))
        };
        let cost = pts
            .windows(2)
            .map(|w| {
                let r = if w[0].r#ref == 0 {
                    self.start_ref
                } else {
                    w[0].r#ref
                };
                Vec3::from_array(w[0].pos).distance(Vec3::from_array(w[1].pos)) * area(r)
            })
            .sum();
        let ws = &self.m.key.ws;
        Ok(PathResult {
            status,
            polys: polys.iter().map(|&r| self.m.poly(r)).collect(),
            waypoints: pts.iter().map(|p| self.m.world(ws, p.pos)).collect(),
            cost,
        })
    }
}

/// Cuts the path where it first comes within `radius` of `at`: returns the cut segment's start
/// index (the path then ends at the cut point), `None` when it never does.
fn trim(pts: &mut Vec<StraightPathPoint>, at: Vec3, radius: f32) -> Option<usize> {
    let p0 = Vec3::from_array(pts[0].pos);
    if p0.distance(at) <= radius {
        pts.truncate(1);
        return Some(0);
    }
    for i in 0..pts.len() - 1 {
        let (a, b) = (
            Vec3::from_array(pts[i].pos),
            Vec3::from_array(pts[i + 1].pos),
        );
        // Smallest t in [0, 1] with |a + t (b - a) - at| = radius.
        let d = b - a;
        let w = a - at;
        let (qa, qb, qc) = (d.dot(d), 2.0 * d.dot(w), w.dot(w) - radius * radius);
        let disc = qb * qb - 4.0 * qa * qc;
        if qa <= 0.0 || disc < 0.0 {
            continue;
        }
        let t = (-qb - disc.sqrt()) / (2.0 * qa);
        if (0.0..=1.0).contains(&t) {
            let cut = StraightPathPoint {
                pos: (a + d * t).to_array(),
                flags: detour::STRAIGHTPATH_END,
                r#ref: 0,
            };
            pts.truncate(i + 1);
            pts.push(cut);
            return Some(i);
        }
    }
    None
}

/// One Dijkstra slice for `Away`: `Some((end, point, status))` once a polygon is far enough or
/// the search ran dry.
#[allow(
    clippy::too_many_arguments,
    reason = "the search's parts, borrowed apart"
)]
fn away_slice(
    q: &mut NavMeshQuery<'_>,
    dij: &mut PolysAroundSearch,
    from: Vec3,
    min_dist: f32,
    far: &mut Option<(f32, detour::PolyRef, [f32; 3])>,
    f: &DtFilter<'_>,
    budget: &mut SliceBudget,
    spent: &mut u32,
) -> Result<Option<(detour::PolyRef, [f32; 3], PathStatus)>, NavError> {
    loop {
        if budget.remaining() == 0 {
            return Ok(None);
        }
        let before = dij.results().len();
        let progress = dij.resume(q, f, &mut Budget::new(1)).map_err(backend)?;
        if dij.results().len() > before {
            budget.charge(1);
            *spent += 1;
            let r = dij.results()[before].0;
            let c = centroid(q, r);
            let d = Vec3::from_array(c).distance(from);
            if d >= min_dist {
                return Ok(Some((r, c, PathStatus::Complete)));
            }
            if far.is_none_or(|(fd, ..)| d > fd) {
                *far = Some((d, r, c));
            }
        } else if progress == Progress::Done || dij.is_done() {
            let Some((_, r, c)) = *far else {
                return Err(NavError::StartOffMesh);
            };
            return Ok(Some((r, c, PathStatus::Partial)));
        }
    }
}

/// A polygon's vertex centroid, on its detail surface.
fn centroid(q: &NavMeshQuery<'_>, r: detour::PolyRef) -> [f32; 3] {
    let Ok((tile, poly)) = q.nav().tile_and_poly_by_ref(r) else {
        return [f32::NAN; 3];
    };
    let n = usize::from(poly.vert_count).max(1);
    let sum: Vec3 = poly.verts[..usize::from(poly.vert_count)]
        .iter()
        .map(|&v| {
            let i = usize::from(v) * 3;
            Vec3::new(tile.verts[i], tile.verts[i + 1], tile.verts[i + 2])
        })
        .sum();
    #[allow(clippy::cast_precision_loss, reason = "at most 6 vertices")]
    let c = (sum / n as f32).to_array();
    q.closest_point_on_poly(r, &c).map_or(c, |(p, _)| p)
}
