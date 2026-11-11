//! Resumable forms of the Dijkstra-style searches and of the raycast (not in
//! recastnavigation).
//!
//! Each search owns its node pool, open list or stack and the locals of upstream's
//! loop, so it survives other queries on the same [`NavMeshQuery`] and can be
//! continued over several frames. A step ([`PolysAroundSearch::resume`] and its
//! siblings) expands polygons until the search ends or the [`Budget`] has no unit
//! left; one unit is one polygon expanded. When the budget is large enough for the
//! whole search, the results equal the unbudgeted query's, which runs the same
//! per-polygon code.
//!
//! A search belongs to the navmesh it began on and must be resumed with a query over
//! that navmesh. Tiles may be committed between steps (the search may then miss the
//! new area: its result reflects the navmesh at its start plus additions). If a
//! tile holding a polygon the search has visited was unloaded or removed since the
//! last step, or the navmesh's removal log no longer reaches back that far, the step
//! fails with [`Error::MeshChanged`] instead of reading stale refs; restart the
//! search. The raycast has no node pool and fails on any unload or removal.

use alloc::vec::Vec;

use super::math::*;
use super::node::{NodePool, NodeQueue};
use super::query::{NavMeshQuery, QueryFilter, RaycastHit, pool_changed};
use super::query_local::{NeighbourhoodState, RaycastState, Region, SurfaceState, WallState};
use super::{PolyRef, Status};
use crate::{Budget, Error, Progress};

/// The navmesh a search began on (its address) and the epoch it was last checked
/// at.
#[derive(Clone, Copy)]
struct Stamp {
    id: usize,
    epoch: u64,
}

fn nav_id(q: &NavMeshQuery) -> Stamp {
    Stamp {
        id: core::ptr::from_ref(q.nav()).addr(),
        epoch: q.nav().epoch(),
    }
}

/// Checks that `q` is over the search's navmesh and that no tile the search
/// touched (`pool`; any tile without one) lost its tile since the stamp's epoch,
/// then moves the stamp to the current epoch.
fn check_nav(q: &NavMeshQuery, stamp: &mut Stamp, pool: Option<&NodePool>) -> Result<(), Error> {
    let now = nav_id(q);
    if now.id != stamp.id {
        return Err(Error::InvalidParameter(
            "search resumed with a query over another navmesh",
        ));
    }
    let changed = match pool {
        Some(p) => pool_changed(q.nav(), stamp.epoch, p),
        None => q.nav().changed_since(stamp.epoch, &mut |_| true),
    };
    if changed {
        return Err(Error::MeshChanged);
    }
    stamp.epoch = now.epoch;
    Ok(())
}

impl NavMeshQuery<'_> {
    /// Runs `f` with `pool` and `open` standing in for the query's own.
    fn with_search_pools<R>(
        &mut self,
        pool: &mut NodePool,
        open: &mut NodeQueue,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        core::mem::swap(&mut self.node_pool, pool);
        core::mem::swap(&mut self.open_list, open);
        let r = f(self);
        core::mem::swap(&mut self.node_pool, pool);
        core::mem::swap(&mut self.open_list, open);
        r
    }

    /// Runs `f` with `pool` standing in for the query's small node pool.
    fn with_tiny_pool<R>(&mut self, pool: &mut NodePool, f: impl FnOnce(&mut Self) -> R) -> R {
        core::mem::swap(&mut self.tiny_node_pool, pool);
        let r = f(self);
        core::mem::swap(&mut self.tiny_node_pool, pool);
        r
    }

    fn new_search_pools(&self) -> Result<(NodePool, NodeQueue), Error> {
        Ok((
            self.node_pool.same_shape()?,
            NodeQueue::new(self.node_pool.max_nodes())?,
        ))
    }

    fn new_tiny_pool(&self) -> Result<NodePool, Error> {
        self.tiny_node_pool.same_shape()
    }

    /// Starts a resumable `findPolysAroundCircle` (not in recastnavigation).
    ///
    /// Results of the finished search equal
    /// [`NavMeshQuery::find_polys_around_circle`]'s; `max` caps the stored polygons.
    pub fn begin_polys_around_circle(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        radius: f32,
        max: usize,
    ) -> Result<PolysAroundSearch, Error> {
        if !self.nav.is_valid_poly_ref(start_ref)
            || !visfinite(center)
            || radius < 0.0
            || !radius.is_finite()
        {
            return Err(Error::InvalidParameter("ref, center or radius"));
        }
        PolysAroundSearch::begin(self, start_ref, *center, None, sqr(radius), max)
    }

    /// Starts a resumable `findPolysAroundShape` (not in recastnavigation); `verts`
    /// holds 3 floats per vertex.
    pub fn begin_polys_around_shape(
        &mut self,
        start_ref: PolyRef,
        verts: &[f32],
        max: usize,
    ) -> Result<PolysAroundSearch, Error> {
        let nverts = verts.len() / 3;
        if !self.nav.is_valid_poly_ref(start_ref)
            || nverts < 3
            || !verts.iter().all(|v| v.is_finite())
        {
            return Err(Error::InvalidParameter("ref or shape"));
        }
        let mut c = [0.0f32; 3];
        for i in 0..nverts {
            c = vadd(&c, &verts[i * 3..]);
        }
        let c = vscale(&c, 1.0 / nverts as f32);
        PolysAroundSearch::begin(self, start_ref, c, Some(verts.to_vec()), 0.0, max)
    }

    /// Starts a resumable `findDistanceToWall` (not in recastnavigation).
    pub fn begin_distance_to_wall(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        max_radius: f32,
    ) -> Result<DistanceToWallSearch, Error> {
        self.check_circle(start_ref, center, max_radius)?;
        let (mut pool, mut open) = self.new_search_pools()?;
        self.with_search_pools(&mut pool, &mut open, |q| {
            q.dijkstra_start(start_ref, center)
        })?;
        Ok(DistanceToWallSearch {
            nav: nav_id(self),
            pool,
            open,
            center: *center,
            state: WallState::new(max_radius),
            done: false,
        })
    }

    /// Starts a resumable `findLocalNeighbourhood` (not in recastnavigation); `max`
    /// caps the stored polygons.
    pub fn begin_local_neighbourhood(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        radius: f32,
        max: usize,
    ) -> Result<LocalNeighbourhoodSearch, Error> {
        self.check_circle(start_ref, center, radius)?;
        let mut pool = self.new_tiny_pool()?;
        let mut out = Vec::new();
        let state = self.with_tiny_pool(&mut pool, |q| {
            q.neighbourhood_start(start_ref, &mut out, max)
        })?;
        Ok(LocalNeighbourhoodSearch {
            nav: nav_id(self),
            pool,
            center: *center,
            radius_sqr: sqr(radius),
            state,
            out,
            max,
            done: false,
        })
    }

    /// Starts a resumable `moveAlongSurface` (not in recastnavigation); `max_visited`
    /// caps the visited polygons reported by [`SurfaceMoveSearch::result`].
    pub fn begin_move_along_surface(
        &mut self,
        start_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        max_visited: usize,
    ) -> Result<SurfaceMoveSearch, Error> {
        self.check_surface_move(start_ref, start_pos, end_pos, max_visited)?;
        let mut pool = self.new_tiny_pool()?;
        let state = self.with_tiny_pool(&mut pool, |q| {
            q.surface_start(start_ref, start_pos, end_pos)
        })?;
        Ok(SurfaceMoveSearch {
            nav: nav_id(self),
            pool,
            state,
            max_visited,
        })
    }

    /// Starts a resumable raycast (not in recastnavigation) with the arguments of
    /// [`NavMeshQuery::raycast_with`].
    #[allow(clippy::too_many_arguments)]
    pub fn begin_raycast(
        &self,
        start_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        options: u32,
        max_path: usize,
        prev_ref: PolyRef,
    ) -> Result<RaycastSearch, Error> {
        Ok(RaycastSearch {
            nav: nav_id(self),
            state: self
                .raycast_start(start_ref, start_pos, end_pos, options, max_path, prev_ref)?,
        })
    }
}

/// A resumable `findPolysAroundCircle` or `findPolysAroundShape` (not in
/// recastnavigation).
pub struct PolysAroundSearch {
    nav: Stamp,
    pool: NodePool,
    open: NodeQueue,
    center: [f32; 3],
    shape: Option<Vec<f32>>,
    radius_sqr: f32,
    out: Vec<(PolyRef, PolyRef, f32)>,
    max: usize,
    status: u32,
    done: bool,
}

impl PolysAroundSearch {
    fn begin(
        q: &mut NavMeshQuery,
        start_ref: PolyRef,
        center: [f32; 3],
        shape: Option<Vec<f32>>,
        radius_sqr: f32,
        max: usize,
    ) -> Result<Self, Error> {
        let (mut pool, mut open) = q.new_search_pools()?;
        q.with_search_pools(&mut pool, &mut open, |q| {
            q.dijkstra_start(start_ref, &center)
        })?;
        Ok(PolysAroundSearch {
            nav: nav_id(q),
            pool,
            open,
            center,
            shape,
            radius_sqr,
            out: Vec::new(),
            max,
            status: Status::SUCCESS,
            done: false,
        })
    }

    /// Expands polygons until the search ends or `budget` has no unit left. Resuming
    /// a finished search does nothing.
    pub fn resume(
        &mut self,
        q: &mut NavMeshQuery,
        filter: &dyn QueryFilter,
        budget: &mut Budget,
    ) -> Result<Progress, Error> {
        check_nav(q, &mut self.nav, Some(&self.pool))?;
        if self.done {
            return Ok(Progress::Done);
        }
        let region = match &self.shape {
            Some(v) => Region::Shape(v, v.len() / 3),
            None => Region::Circle(self.center, self.radius_sqr),
        };
        let (out, max, status) = (&mut self.out, self.max, &mut self.status);
        let exhausted = q.with_search_pools(&mut self.pool, &mut self.open, |q| {
            while !q.open_list.empty() {
                if !budget.spend(1) {
                    return true;
                }
                q.polys_around_step(&region, filter, out, max, status);
            }
            false
        });
        self.done = !exhausted;
        Ok(if exhausted {
            Progress::Exhausted
        } else {
            Progress::Done
        })
    }

    /// Whether the search has ended.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// The polygons expanded so far as (ref, parent, cost), at most the `max` given
    /// to `begin_*`; final once the search is done.
    pub fn results(&self) -> &[(PolyRef, PolyRef, f32)] {
        &self.out
    }

    /// The status so far (`BUFFER_TOO_SMALL`, `OUT_OF_NODES`).
    pub fn status(&self) -> Status {
        Status(self.status)
    }

    /// Path from the start polygon to `end_ref`, which the search must have
    /// expanded (`getPathFromDijkstraSearch`).
    pub fn path_to(
        &self,
        q: &NavMeshQuery,
        end_ref: PolyRef,
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) -> Result<Status, Error> {
        check_nav(q, &mut { self.nav }, Some(&self.pool))?;
        q.path_from_pool(&self.pool, end_ref, path, max_path)
    }
}

/// A resumable `findDistanceToWall` (not in recastnavigation).
pub struct DistanceToWallSearch {
    nav: Stamp,
    pool: NodePool,
    open: NodeQueue,
    center: [f32; 3],
    state: WallState,
    done: bool,
}

impl DistanceToWallSearch {
    /// Expands polygons until the search ends or `budget` has no unit left.
    pub fn resume(
        &mut self,
        q: &mut NavMeshQuery,
        filter: &dyn QueryFilter,
        budget: &mut Budget,
    ) -> Result<Progress, Error> {
        check_nav(q, &mut self.nav, Some(&self.pool))?;
        if self.done {
            return Ok(Progress::Done);
        }
        let (state, center) = (&mut self.state, &self.center);
        let exhausted = q.with_search_pools(&mut self.pool, &mut self.open, |q| {
            while !q.open_list.empty() {
                if !budget.spend(1) {
                    return true;
                }
                q.wall_step(state, center, filter);
            }
            false
        });
        self.done = !exhausted;
        Ok(if exhausted {
            Progress::Exhausted
        } else {
            Progress::Done
        })
    }

    /// Whether the search has ended.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Distance, hit point and normal found so far (the `max_radius` and the origin
    /// until a wall is found); final once the search is done.
    pub fn result(&self) -> (f32, [f32; 3], [f32; 3]) {
        self.state.result(&self.center)
    }

    /// The status so far (`OUT_OF_NODES`).
    pub fn status(&self) -> Status {
        Status(self.state.status)
    }
}

/// A resumable `findLocalNeighbourhood` (not in recastnavigation).
pub struct LocalNeighbourhoodSearch {
    nav: Stamp,
    pool: NodePool,
    center: [f32; 3],
    radius_sqr: f32,
    state: NeighbourhoodState,
    out: Vec<(PolyRef, PolyRef)>,
    max: usize,
    done: bool,
}

impl LocalNeighbourhoodSearch {
    /// Expands polygons until the search ends or `budget` has no unit left.
    pub fn resume(
        &mut self,
        q: &mut NavMeshQuery,
        filter: &dyn QueryFilter,
        budget: &mut Budget,
    ) -> Result<Progress, Error> {
        check_nav(q, &mut self.nav, Some(&self.pool))?;
        if self.done {
            return Ok(Progress::Done);
        }
        let (state, out, max) = (&mut self.state, &mut self.out, self.max);
        let (center, radius_sqr) = (&self.center, self.radius_sqr);
        let exhausted = q.with_tiny_pool(&mut self.pool, |q| {
            while !state.stack.is_empty() {
                if !budget.spend(1) {
                    return true;
                }
                q.neighbourhood_step(state, center, radius_sqr, filter, out, max);
            }
            false
        });
        self.done = !exhausted;
        Ok(if exhausted {
            Progress::Exhausted
        } else {
            Progress::Done
        })
    }

    /// Whether the search has ended.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// The polygons found so far as (ref, parent); final once the search is done.
    pub fn results(&self) -> &[(PolyRef, PolyRef)] {
        &self.out
    }

    /// The status so far (`BUFFER_TOO_SMALL`).
    pub fn status(&self) -> Status {
        Status(self.state.status)
    }
}

/// A resumable `moveAlongSurface` (not in recastnavigation).
pub struct SurfaceMoveSearch {
    nav: Stamp,
    pool: NodePool,
    state: SurfaceState,
    max_visited: usize,
}

impl SurfaceMoveSearch {
    /// Expands polygons until the move ends or `budget` has no unit left.
    pub fn resume(
        &mut self,
        q: &mut NavMeshQuery,
        filter: &dyn QueryFilter,
        budget: &mut Budget,
    ) -> Result<Progress, Error> {
        check_nav(q, &mut self.nav, Some(&self.pool))?;
        let state = &mut self.state;
        let exhausted = q.with_tiny_pool(&mut self.pool, |q| {
            while !state.finished() {
                if !budget.spend(1) {
                    return true;
                }
                q.surface_step(state, filter);
            }
            false
        });
        Ok(if exhausted {
            Progress::Exhausted
        } else {
            Progress::Done
        })
    }

    /// Whether the move has ended.
    pub fn is_done(&self) -> bool {
        self.state.finished()
    }

    /// The position reached so far and the polygons leading to it into `visited`;
    /// final once the move is done. Taking a result does not disturb the search.
    pub fn result(&self, q: &NavMeshQuery, visited: &mut Vec<PolyRef>) -> ([f32; 3], Status) {
        q.surface_result(&self.state, &self.pool, self.max_visited, visited)
    }
}

/// A resumable raycast (not in recastnavigation).
pub struct RaycastSearch {
    nav: Stamp,
    state: RaycastState,
}

impl RaycastSearch {
    /// Crosses polygons until the ray ends or `budget` has no unit left.
    pub fn resume(
        &mut self,
        q: &NavMeshQuery,
        filter: &dyn QueryFilter,
        budget: &mut Budget,
    ) -> Result<Progress, Error> {
        check_nav(q, &mut self.nav, None)?;
        while !self.state.finished {
            if !budget.spend(1) {
                return Ok(Progress::Exhausted);
            }
            q.raycast_step(&mut self.state, filter);
        }
        Ok(Progress::Done)
    }

    /// Whether the ray has ended.
    pub fn is_done(&self) -> bool {
        self.state.finished
    }

    /// The hit so far (polygons crossed, the parameter at the last crossing) and
    /// status; final once the ray is done.
    pub fn result(&self) -> (&RaycastHit, Status) {
        (&self.state.hit, Status(self.state.status))
    }
}
