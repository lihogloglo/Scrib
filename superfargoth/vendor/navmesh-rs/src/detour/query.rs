//! Navmesh queries (`dtNavMeshQuery`): filters, nearest polygon, A* paths (plain
//! and sliced), straight paths and random points. Raycast, surface moves and the
//! Dijkstra-style local queries live in `query_local.rs`.

use alloc::vec::Vec;

use super::math::*;
use super::navmesh::{NavMesh, closest_point_on_poly_in_tile, poly_height};
use super::node::{NODE_CLOSED, NODE_OPEN, NODE_PARENT_DETACHED, NodePool, NodeQueue, no_better};
use super::tile::{MeshTile, Poly};
use super::{
    EdgeKind, FINDPATH_ANY_ANGLE, MAX_AREAS, NULL_LINK, POLYTYPE_GROUND,
    POLYTYPE_OFFMESH_CONNECTION, PolyRef, RAY_CAST_LIMIT_PROPORTIONS, RAYCAST_USE_COSTS,
    STRAIGHTPATH_ALL_CROSSINGS, STRAIGHTPATH_AREA_CROSSINGS, STRAIGHTPATH_END,
    STRAIGHTPATH_OFFMESH_CONNECTION, STRAIGHTPATH_START, Status, TileKey, VERTS_PER_POLYGON,
};
use crate::budget::spend_one;
use crate::{Budget, Budgeted, Error, UpstreamFixes};

pub(crate) const H_SCALE: f32 = 0.999;

#[inline]
fn path_heuristic(a: &[f32; 3], b: &[f32; 3], scale: f32) -> f32 {
    // Dijkstra must stay zero even when squared coordinate distances overflow.
    if scale == 0.0 {
        0.0
    } else {
        vdist(a, b) * scale
    }
}
const DETAIL_MASK: u32 = 0x0ff_ffff;

/// Polygon filter and traversal cost (`dtQueryFilter`).
pub trait QueryFilter {
    /// Whether the polygon may be visited.
    fn pass_filter(&self, r: PolyRef, tile: &MeshTile, poly: &Poly) -> bool;
    /// Cost of moving from `pa` to `pb` across `cur` (between `prev` and `next`,
    /// which are absent at the ends of a path).
    #[allow(clippy::too_many_arguments)]
    fn cost(
        &self,
        pa: &[f32; 3],
        pb: &[f32; 3],
        prev: Option<(PolyRef, &MeshTile, &Poly)>,
        cur: (PolyRef, &MeshTile, &Poly),
        next: Option<(PolyRef, &MeshTile, &Poly)>,
    ) -> f32;
}

/// Upstream's default filter: include and exclude flags and a cost per area.
#[derive(Clone, Debug, PartialEq)]
pub struct DefaultFilter {
    /// Cost multiplier per area id.
    pub area_cost: [f32; MAX_AREAS],
    /// A polygon passes if it has any of these flags...
    pub include_flags: u16,
    /// ...and none of these.
    pub exclude_flags: u16,
}

impl Default for DefaultFilter {
    fn default() -> Self {
        DefaultFilter {
            area_cost: [1.0; MAX_AREAS],
            include_flags: 0xffff,
            exclude_flags: 0,
        }
    }
}

impl QueryFilter for DefaultFilter {
    #[inline]
    fn pass_filter(&self, _r: PolyRef, _tile: &MeshTile, poly: &Poly) -> bool {
        (poly.flags & self.include_flags) != 0 && (poly.flags & self.exclude_flags) == 0
    }
    #[inline]
    fn cost(
        &self,
        pa: &[f32; 3],
        pb: &[f32; 3],
        _prev: Option<(PolyRef, &MeshTile, &Poly)>,
        cur: (PolyRef, &MeshTile, &Poly),
        _next: Option<(PolyRef, &MeshTile, &Poly)>,
    ) -> f32 {
        vdist(pa, pb) * self.area_cost[cur.2.area() as usize]
    }
}

/// A raycast result (`dtRaycastHit`).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct RaycastHit {
    /// Why the ray stopped (not in recastnavigation): [`EdgeKind::None`] when it
    /// reached its end, otherwise a wall, a world edge, or a border facing a known,
    /// unloaded column.
    pub edge: EdgeKind,
    /// Hit parameter along the ray (`f32::MAX` when the end was reached).
    pub t: f32,
    /// Normal of the hit wall.
    pub hit_normal: [f32; 3],
    /// Edge index of the hit wall on the last polygon (-1 if none).
    pub hit_edge_index: i32,
    /// Visited polygons.
    pub path: Vec<PolyRef>,
    /// Path cost (with [`RAYCAST_USE_COSTS`]).
    pub path_cost: f32,
}

/// One vertex of a straight path.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StraightPathPoint {
    /// Position.
    pub pos: [f32; 3],
    /// `STRAIGHTPATH_*` flags.
    pub flags: u8,
    /// Polygon entered at this vertex (0 at the end).
    pub r#ref: PolyRef,
}

/// One wall or portal segment from [`NavMeshQuery::poly_wall_segments`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SegmentWall {
    /// Segment start and end.
    pub seg: [f32; 6],
    /// Neighbour polygon across a portal segment (0 for a wall).
    pub r#ref: PolyRef,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct QueryData {
    pub status: u32,
    pub last_best_node: Option<usize>,
    pub last_best_node_cost: f32,
    pub start_ref: PolyRef,
    pub end_ref: PolyRef,
    pub end_pos: [f32; 3],
    pub options: u32,
    pub raycast_limit_sqr: f32,
    /// [`NavMesh::epoch`] when the sliced search began.
    pub epoch: u64,
}

/// Whether a tile that a search's node pool touched was unloaded or removed since
/// `epoch` (see [`NavMesh::changed_since`]).
pub(crate) fn pool_changed(nav: &NavMesh, epoch: u64, pool: &NodePool) -> bool {
    if nav.epoch() == epoch {
        return false;
    }
    let mut slots: Vec<u32> = pool
        .used()
        .iter()
        .map(|n| nav.decode_poly_id_tile(n.id))
        .collect();
    slots.sort_unstable();
    slots.dedup();
    nav.changed_since(epoch, &mut |s| slots.binary_search(&s).is_ok())
}

/// Query workspace over a navmesh (`dtNavMeshQuery`): it owns the node pools and
/// the open list, so one query object serves one thread.
pub struct NavMeshQuery<'a> {
    pub(crate) nav: &'a NavMesh,
    pub(crate) node_pool: NodePool,
    pub(crate) tiny_node_pool: NodePool,
    pub(crate) open_list: NodeQueue,
    pub(crate) query: QueryData,
    fixes: UpstreamFixes,
}

pub(crate) type Tp<'a> = (PolyRef, &'a MeshTile, &'a Poly);

impl<'a> NavMeshQuery<'a> {
    /// Creates a query with `max_nodes` search nodes (1..=65,535).
    pub fn new(nav: &'a NavMesh, max_nodes: usize) -> Result<NavMeshQuery<'a>, Error> {
        if max_nodes == 0 || max_nodes > 0xffff {
            return Err(Error::InvalidParameter("max_nodes must be in 1..=65535"));
        }
        Ok(NavMeshQuery {
            nav,
            node_pool: NodePool::new(max_nodes, next_pow2((max_nodes / 4) as u32).max(1) as usize)?,
            tiny_node_pool: NodePool::new(64, 32)?,
            open_list: NodeQueue::new(max_nodes)?,
            query: QueryData::default(),
            fixes: UpstreamFixes::NONE,
        })
    }

    /// Selects the upstream defect corrections this query applies; the default
    /// is [`UpstreamFixes::NONE`]. Only [`UpstreamFixes::SLICED_CROSS_SIDE`]
    /// concerns a query. Change it between searches, not during a sliced one.
    pub fn set_fixes(&mut self, fixes: UpstreamFixes) {
        self.fixes = fixes;
    }

    /// Configured search node capacity.
    pub fn max_nodes(&self) -> u16 {
        self.node_pool.nodes.len() as u16
    }

    /// The corrections this query applies.
    pub fn fixes(&self) -> UpstreamFixes {
        self.fixes
    }

    /// The navmesh this query reads.
    pub fn nav(&self) -> &'a NavMesh {
        self.nav
    }

    #[inline]
    pub(crate) fn tp(&self, r: PolyRef) -> Tp<'a> {
        // Refs here come from links and node pools this navmesh built; anything else
        // reads as an empty polygon (no vertices, no links) instead of panicking.
        let (t, ip) = self.nav.tile_and_poly_unsafe(r);
        (r, t, t.polys.get(ip).unwrap_or(&super::tile::EMPTY_POLY))
    }

    pub(crate) fn valid_tp(&self, r: PolyRef) -> Result<Tp<'a>, Error> {
        let (t, p) = self.nav.tile_and_poly_by_ref(r)?;
        Ok((r, t, p))
    }

    #[inline]
    pub(crate) fn ip_of(&self, r: PolyRef) -> usize {
        self.nav.decode_poly_id_poly(r) as usize
    }

    /// Random point on a random polygon, area-weighted (`findRandomPoint`). `frand`
    /// returns values in [0, 1).
    pub fn find_random_point(
        &self,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
    ) -> Result<(PolyRef, [f32; 3]), Error> {
        self.find_random_point_impl(filter, frand, &mut None)
    }

    /// [`NavMeshQuery::find_random_point`] spending one unit of `budget` per
    /// resident tile considered and per polygon of the chosen tile (not in
    /// recastnavigation). The pick is a reservoir sample over every polygon, so a
    /// partial scan has no meaningful result: when the budget runs out the outcome
    /// is `Exhausted(None)`. With a sufficient budget it is `Complete(Some(..))` and
    /// equals the unbudgeted call for the same `frand` sequence; a mesh with no
    /// eligible polygon is [`Error::Failed`], as unbudgeted.
    #[allow(clippy::type_complexity)]
    pub fn find_random_point_budgeted(
        &self,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
        budget: &mut Budget,
    ) -> Result<Budgeted<Option<(PolyRef, [f32; 3])>>, Error> {
        let (r, exhausted) =
            budget.scoped(|b| self.find_random_point_impl(filter, frand, &mut Some(b)));
        match r {
            Ok(v) => Ok(Budgeted::of(Some(v), exhausted)),
            Err(Error::Failed) if exhausted => Ok(Budgeted::Exhausted(None)),
            Err(e) => Err(e),
        }
    }

    fn find_random_point_impl(
        &self,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
        budget: &mut Option<&mut Budget>,
    ) -> Result<(PolyRef, [f32; 3]), Error> {
        let mut tile: Option<usize> = None;
        let mut tsum = 0.0f32;
        for i in 0..self.nav.max_tiles() as usize {
            if self.nav.tile(i).is_none() {
                continue;
            }
            if !spend_one(budget) {
                return Err(Error::Failed);
            }
            let area = 1.0f32;
            tsum += area;
            let u = frand();
            if u * tsum <= area {
                tile = Some(i);
            }
        }
        let ti = tile.ok_or(Error::Failed)?;
        let t = self.nav.tile(ti).ok_or(Error::Failed)?;
        let base = self.nav.base_ref(ti);
        let mut found: Option<(usize, PolyRef)> = None;
        let mut area_sum = 0.0f32;
        for (i, p) in t.polys.iter().enumerate() {
            if !spend_one(budget) {
                return Err(Error::Failed);
            }
            if p.poly_type() != POLYTYPE_GROUND {
                continue;
            }
            let r = base | i as PolyRef;
            if !filter.pass_filter(r, t, p) {
                continue;
            }
            let area = poly_area_2d(t, p);
            area_sum += area;
            let u = frand();
            if u * area_sum <= area {
                found = Some((i, r));
            }
        }
        let (ip, r) = found.ok_or(Error::Failed)?;
        let pt = self.random_point_in_poly(t, ip, frand);
        let (pt, _) = closest_point_on_poly_in_tile(t, ip, &pt);
        Ok((r, pt))
    }

    fn random_point_in_poly(
        &self,
        t: &MeshTile,
        ip: usize,
        frand: &mut dyn FnMut() -> f32,
    ) -> [f32; 3] {
        let p = &t.polys[ip];
        let mut verts = [0.0f32; 3 * VERTS_PER_POLYGON];
        let mut areas = [0.0f32; VERTS_PER_POLYGON];
        for j in 0..p.vert_count as usize {
            let v = p.verts[j] as usize * 3;
            verts[j * 3..j * 3 + 3].copy_from_slice(&t.verts[v..v + 3]);
        }
        let s = frand();
        let tt = frand();
        random_point_in_convex_poly(&verts, p.vert_count as usize, &mut areas, s, tt)
    }

    /// Random point within reach of `center` (`findRandomPointAroundCircle`).
    pub fn find_random_point_around_circle(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        max_radius: f32,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
    ) -> Result<(PolyRef, [f32; 3], Status), Error> {
        self.find_random_point_around_circle_impl(
            start_ref, center, max_radius, filter, frand, &mut None,
        )
    }

    /// [`NavMeshQuery::find_random_point_around_circle`] spending one unit of
    /// `budget` per polygon expanded (not in recastnavigation). When the budget runs
    /// out the point is drawn from the polygons expanded so far: still a valid point
    /// within reach, uniform over that subset (`Exhausted(Some(..))`), or
    /// `Exhausted(None)` if none was ground. With a sufficient budget the result
    /// equals the unbudgeted call for the same `frand` sequence.
    #[allow(clippy::type_complexity)]
    pub fn find_random_point_around_circle_budgeted(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        max_radius: f32,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
        budget: &mut Budget,
    ) -> Result<Budgeted<Option<(PolyRef, [f32; 3], Status)>>, Error> {
        let (r, exhausted) = budget.scoped(|b| {
            self.find_random_point_around_circle_impl(
                start_ref,
                center,
                max_radius,
                filter,
                frand,
                &mut Some(b),
            )
        });
        match r {
            Ok(v) => Ok(Budgeted::of(Some(v), exhausted)),
            Err(Error::Failed) if exhausted => Ok(Budgeted::Exhausted(None)),
            Err(e) => Err(e),
        }
    }

    fn find_random_point_around_circle_impl(
        &mut self,
        start_ref: PolyRef,
        center: &[f32; 3],
        max_radius: f32,
        filter: &dyn QueryFilter,
        frand: &mut dyn FnMut() -> f32,
        budget: &mut Option<&mut Budget>,
    ) -> Result<(PolyRef, [f32; 3], Status), Error> {
        if !self.nav.is_valid_poly_ref(start_ref)
            || !visfinite(center)
            || max_radius < 0.0
            || !max_radius.is_finite()
        {
            return Err(Error::InvalidParameter("start_ref, center or radius"));
        }
        let start = self.tp(start_ref);
        if !filter.pass_filter(start_ref, start.1, start.2) {
            return Err(Error::InvalidParameter("start polygon filtered out"));
        }
        self.node_pool.clear();
        self.open_list.clear();
        let sn = self
            .node_pool
            .get_node(start_ref, 0)
            .ok_or(Error::AllocationFailed)?;
        {
            let n = &mut self.node_pool.nodes[sn];
            n.pos = *center;
            n.pidx = 0;
            n.cost = 0.0;
            n.total = 0.0;
            n.id = start_ref;
            n.flags = NODE_OPEN;
        }
        self.open_list.push(sn, &self.node_pool.nodes);
        let mut status = Status::SUCCESS;
        let radius_sqr = sqr(max_radius);
        let mut area_sum = 0.0f32;
        let mut random: Option<Tp<'a>> = None;
        while !self.open_list.empty() {
            if !spend_one(budget) {
                break;
            }
            let bn = self.open_list.pop(&self.node_pool.nodes);
            self.node_pool.nodes[bn].flags &= !NODE_OPEN;
            self.node_pool.nodes[bn].flags |= NODE_CLOSED;
            let best = self.tp(self.node_pool.nodes[bn].id);
            if best.2.poly_type() == POLYTYPE_GROUND {
                let area = poly_area_2d(best.1, best.2);
                area_sum += area;
                let u = frand();
                if u * area_sum <= area {
                    random = Some(best);
                }
            }
            let parent_ref = NodePool::at_idx(self.node_pool.nodes[bn].pidx)
                .map_or(0, |i| self.node_pool.nodes[i].id);
            let mut li = best.2.first_link;
            while li != NULL_LINK {
                let link = best.1.links[li as usize];
                li = link.next;
                let nref = link.r#ref;
                if nref == 0 || nref == parent_ref {
                    continue;
                }
                let nei = self.tp(nref);
                if !filter.pass_filter(nref, nei.1, nei.2) {
                    continue;
                }
                let Some((va, vb)) = self.portal_points_tp(best, nei) else {
                    continue;
                };
                let (dist_sqr, _) = distance_pt_seg_sqr_2d(center, &va, &vb);
                if dist_sqr > radius_sqr {
                    continue;
                }
                let Some(nn) = self.node_pool.get_node(nref, 0) else {
                    status |= Status::OUT_OF_NODES;
                    continue;
                };
                if self.node_pool.nodes[nn].flags & NODE_CLOSED != 0 {
                    continue;
                }
                if self.node_pool.nodes[nn].flags == 0 {
                    self.node_pool.nodes[nn].pos = vlerp(&va, &vb, 0.5);
                }
                let bpos = self.node_pool.nodes[bn].pos;
                let total =
                    self.node_pool.nodes[bn].total + vdist(&bpos, &self.node_pool.nodes[nn].pos);
                let n = &mut self.node_pool.nodes[nn];
                if n.flags & NODE_OPEN != 0 && no_better(total, n.total) {
                    continue;
                }
                n.id = nref;
                n.flags &= !NODE_CLOSED;
                n.pidx = NodePool::idx(bn);
                n.total = total;
                if n.flags & NODE_OPEN != 0 {
                    self.open_list.modify(nn, &self.node_pool.nodes);
                } else {
                    n.flags = NODE_OPEN;
                    self.open_list.push(nn, &self.node_pool.nodes);
                }
            }
        }
        let (rref, rt, rp) = random.ok_or(Error::Failed)?;
        let ip = self.ip_of(rref);
        let _ = rp;
        let pt = self.random_point_in_poly(rt, ip, frand);
        let (pt, _) = closest_point_on_poly_in_tile(rt, ip, &pt);
        Ok((rref, pt, Status(status)))
    }

    /// Closest point on a polygon, using its detail mesh for height
    /// (`closestPointOnPoly`); the flag says whether `pos` lies over the polygon.
    pub fn closest_point_on_poly(
        &self,
        r: PolyRef,
        pos: &[f32; 3],
    ) -> Result<([f32; 3], bool), Error> {
        if !self.nav.is_valid_poly_ref(r) || !visfinite(pos) {
            return Err(Error::InvalidParameter("ref or pos"));
        }
        let (t, ip) = self.nav.tile_and_poly_unsafe(r);
        Ok(closest_point_on_poly_in_tile(t, ip, pos))
    }

    /// Closest point on a polygon's boundary in xz, or `pos` when inside
    /// (`closestPointOnPolyBoundary`).
    pub fn closest_point_on_poly_boundary(
        &self,
        r: PolyRef,
        pos: &[f32; 3],
    ) -> Result<[f32; 3], Error> {
        let (_, tile, poly) = self.valid_tp(r).map_err(|_| Error::InvalidRef)?;
        if !visfinite(pos) {
            return Err(Error::InvalidParameter("pos"));
        }
        Ok(closest_point_on_boundary(tile, poly, pos))
    }

    /// Height of the polygon's detail surface at `pos` (`getPolyHeight`).
    pub fn poly_height(&self, r: PolyRef, pos: &[f32; 3]) -> Result<f32, Error> {
        let (_, tile, poly) = self.valid_tp(r).map_err(|_| Error::InvalidRef)?;
        if !visfinite_2d(pos) {
            return Err(Error::InvalidParameter("pos"));
        }
        if poly.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
            let v0 = &tile.verts[poly.verts[0] as usize * 3..];
            let v1 = &tile.verts[poly.verts[1] as usize * 3..];
            let (_, t) = distance_pt_seg_sqr_2d(pos, v0, v1);
            return Ok(v0[1] + (v1[1] - v0[1]) * t);
        }
        poly_height(tile, self.ip_of(r), pos)
            .ok_or(Error::InvalidParameter("pos outside the polygon"))
    }

    /// Calls `f(tile, poly index, ref)` for every polygon passing `filter` whose
    /// bounds overlap the box (`queryPolygons` with a `dtPolyQuery`).
    pub fn query_polygons_with(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &dyn QueryFilter,
        f: &mut dyn FnMut(&MeshTile, usize, PolyRef),
    ) -> Result<(), Error> {
        self.query_polygons_until(
            center,
            half_extents,
            filter,
            &mut |t, ip, r| {
                f(t, ip, r);
                true
            },
            &mut None,
        )
    }

    /// [`NavMeshQuery::query_polygons_with`] that stops as soon as `f` returns
    /// false. The scan covers only the resident tiles inside the box's tile range,
    /// so its cost is bounded by the navmesh, not by the box size.
    fn query_polygons_until<F: QueryFilter + ?Sized>(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &F,
        f: &mut dyn FnMut(&MeshTile, usize, PolyRef) -> bool,
        budget: &mut Option<&mut Budget>,
    ) -> Result<(), Error> {
        if !visfinite(center) || !visfinite(half_extents) {
            return Err(Error::InvalidParameter("center or half extents"));
        }
        let bmin = vsub(center, half_extents);
        let bmax = vadd(center, half_extents);
        let lo = self.nav.calc_tile_loc(&bmin);
        let hi = self.nav.calc_tile_loc(&bmax);
        self.nav.for_tiles_in(lo, hi, &mut |ti| {
            let Some(tile) = self.nav.tile(ti) else {
                return true;
            };
            if !spend_one(budget) {
                return false;
            }
            let base = self.nav.base_ref(ti);
            super::navmesh::query_polygons_in_tile_with(tile, &bmin, &bmax, &mut |ip| {
                if !spend_one(budget) {
                    return false;
                }
                let r = base | ip as PolyRef;
                let Some(poly) = tile.polys.get(ip as usize) else {
                    return true;
                };
                if filter.pass_filter(r, tile, poly) && !f(tile, ip as usize, r) {
                    return false;
                }
                true
            })
        });
        Ok(())
    }

    /// Polygons overlapping a box (`queryPolygons`), at most `max`; the status
    /// carries `BUFFER_TOO_SMALL` when more were found. The scan stops at the first
    /// polygon that does not fit (with `max == 0`, at the first polygon found):
    /// the result and status are upstream's, without upstream's full scan.
    pub fn query_polygons(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &dyn QueryFilter,
        out: &mut Vec<PolyRef>,
        max: usize,
    ) -> Result<Status, Error> {
        self.query_polygons_impl(center, half_extents, filter, out, max, &mut None)
    }

    /// [`NavMeshQuery::query_polygons`] spending one unit of `budget` per tile
    /// scanned and per candidate polygon tested (not in recastnavigation). A tile's
    /// bounding-volume walk visits at most that tile's polygon count of nodes and is
    /// charged as the one unit for the tile. When the budget runs out `out` holds the
    /// polygons found so far.
    pub fn query_polygons_budgeted(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &dyn QueryFilter,
        out: &mut Vec<PolyRef>,
        max: usize,
        budget: &mut Budget,
    ) -> Result<Budgeted<Status>, Error> {
        let (r, exhausted) = budget.scoped(|b| {
            self.query_polygons_impl(center, half_extents, filter, out, max, &mut Some(b))
        });
        Ok(Budgeted::of(r?, exhausted))
    }

    fn query_polygons_impl(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &dyn QueryFilter,
        out: &mut Vec<PolyRef>,
        max: usize,
        budget: &mut Option<&mut Budget>,
    ) -> Result<Status, Error> {
        out.clear();
        let mut overflow = false;
        self.query_polygons_until(
            center,
            half_extents,
            filter,
            &mut |_, _, r| {
                if out.len() < max {
                    out.push(r);
                    true
                } else {
                    overflow = true;
                    false
                }
            },
            budget,
        )?;
        Ok(Status(
            Status::SUCCESS
                | if overflow {
                    Status::BUFFER_TOO_SMALL
                } else {
                    0
                },
        ))
    }

    /// Nearest polygon to `center` within the box (`findNearestPoly`): ref (0 when
    /// none), nearest point (`center` copy semantics as upstream: only set when
    /// found) and whether `center` lies over the polygon.
    pub fn find_nearest_poly<F: QueryFilter + ?Sized>(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &F,
    ) -> Result<(PolyRef, [f32; 3], bool), Error> {
        self.find_nearest_poly_impl(center, half_extents, filter, &mut None)
    }

    /// [`NavMeshQuery::find_nearest_poly`] charged like
    /// [`NavMeshQuery::query_polygons_budgeted`] (not in recastnavigation). When
    /// the budget runs out the result is the nearest polygon among those tested.
    pub fn find_nearest_poly_budgeted<F: QueryFilter + ?Sized>(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &F,
        budget: &mut Budget,
    ) -> Result<Budgeted<(PolyRef, [f32; 3], bool)>, Error> {
        let (r, exhausted) = budget
            .scoped(|b| self.find_nearest_poly_impl(center, half_extents, filter, &mut Some(b)));
        Ok(Budgeted::of(r?, exhausted))
    }

    fn find_nearest_poly_impl<F: QueryFilter + ?Sized>(
        &self,
        center: &[f32; 3],
        half_extents: &[f32; 3],
        filter: &F,
        budget: &mut Option<&mut Budget>,
    ) -> Result<(PolyRef, [f32; 3], bool), Error> {
        let mut nearest_d = f32::MAX;
        let mut nearest: (PolyRef, [f32; 3], bool) = (0, [0.0; 3], false);
        self.query_polygons_until(
            center,
            half_extents,
            filter,
            &mut |tile, ip, r| {
                let (cp, over) = closest_point_on_poly_in_tile(tile, ip, center);
                let diff = vsub(center, &cp);
                let d = if over {
                    let d = diff[1].abs() - tile.header.walkable_climb;
                    if d > 0.0 { d * d } else { 0.0 }
                } else {
                    vlen_sqr(&diff)
                };
                if d < nearest_d {
                    nearest_d = d;
                    nearest = (r, cp, over);
                }
                true
            },
            budget,
        )?;
        Ok(nearest)
    }

    /// Portal between two adjacent polygons (`getPortalPoints`): left and right.
    pub fn portal_points(&self, from: PolyRef, to: PolyRef) -> Result<([f32; 3], [f32; 3]), Error> {
        let f = self.valid_tp(from).map_err(|_| Error::InvalidRef)?;
        let t = self.valid_tp(to).map_err(|_| Error::InvalidRef)?;
        self.portal_points_tp(f, t)
            .ok_or(Error::InvalidParameter("polygons are not linked"))
    }

    pub(crate) fn portal_points_tp(&self, from: Tp, to: Tp) -> Option<([f32; 3], [f32; 3])> {
        let (_, ft, fp) = from;
        let (_, tt, tp) = to;
        let find = |tile: &MeshTile, poly: &Poly, r: PolyRef| {
            let mut i = poly.first_link;
            while i != NULL_LINK {
                let l = &tile.links[i as usize];
                if l.r#ref == r {
                    return Some(*l);
                }
                i = l.next;
            }
            None
        };
        let link = find(ft, fp, to.0)?;
        // A link's edge index comes from the tile bytes: a landing link (edge 255)
        // on a polygon that is not the connection's ground polygon, or a polygon of
        // the wrong type, must end the lookup instead of indexing past the vertices.
        if fp.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
            let v = v3(&ft.verts[*fp.verts.get(link.edge as usize)? as usize * 3..]);
            return Some((v, v));
        }
        if tp.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
            let l = find(tt, tp, from.0)?;
            let v = v3(&tt.verts[*tp.verts.get(l.edge as usize)? as usize * 3..]);
            return Some((v, v));
        }
        let e = link.edge as usize;
        if e >= fp.vert_count as usize {
            return None;
        }
        let v0 = fp.verts[e] as usize * 3;
        let v1 = fp.verts[(e + 1) % fp.vert_count as usize] as usize * 3;
        let mut left = v3(&ft.verts[v0..]);
        let mut right = v3(&ft.verts[v1..]);
        if link.side != 0xff && (link.bmin != 0 || link.bmax != 255) {
            let s = 1.0f32 / 255.0;
            let tmin = link.bmin as f32 * s;
            let tmax = link.bmax as f32 * s;
            left = vlerp(&ft.verts[v0..], &ft.verts[v1..], tmin);
            right = vlerp(&ft.verts[v0..], &ft.verts[v1..], tmax);
        }
        Some((left, right))
    }

    pub(crate) fn edge_mid_point_tp(&self, from: Tp, to: Tp) -> Option<[f32; 3]> {
        let (l, r) = self.portal_points_tp(from, to)?;
        Some([
            (l[0] + r[0]) * 0.5,
            (l[1] + r[1]) * 0.5,
            (l[2] + r[2]) * 0.5,
        ])
    }

    /// A* path from `start_ref` to `end_ref` (`findPath`), at most `max_path`
    /// polygons. The status carries `PARTIAL_RESULT`, `OUT_OF_NODES` and
    /// `BUFFER_TOO_SMALL` like upstream.
    #[allow(clippy::too_many_arguments)]
    pub fn find_path<F: QueryFilter + ?Sized>(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &F,
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) -> Result<Status, Error> {
        self.find_path_impl(
            start_ref, end_ref, start_pos, end_pos, filter, path, max_path, H_SCALE, &mut None,
        )
    }

    /// A* with a caller-selected Euclidean heuristic scale (not in recastnavigation).
    /// The scale must be finite and nonnegative. Use 0 for Dijkstra, including
    /// zero-cost links. For optimal search, the scale must lower-bound every
    /// traversal cost per unit Euclidean distance. The default API uses 0.999.
    #[allow(clippy::too_many_arguments)]
    pub fn find_path_with_heuristic_scale<F: QueryFilter + ?Sized>(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &F,
        path: &mut Vec<PolyRef>,
        max_path: usize,
        heuristic_scale: f32,
    ) -> Result<Status, Error> {
        if !heuristic_scale.is_finite() || heuristic_scale < 0.0 {
            return Err(Error::InvalidParameter("heuristic scale"));
        }
        self.find_path_impl(
            start_ref,
            end_ref,
            start_pos,
            end_pos,
            filter,
            path,
            max_path,
            heuristic_scale,
            &mut None,
        )
    }

    /// [`NavMeshQuery::find_path`] spending one unit of `budget` per polygon
    /// expanded (not in recastnavigation). When the budget runs out the search
    /// stops where it is: the path leads to the closest polygon found so far, as
    /// for upstream's out-of-nodes case, with `PARTIAL_RESULT` set, and the outcome
    /// is [`Budgeted::Exhausted`]. A budget that suffices gives
    /// [`find_path`](NavMeshQuery::find_path)'s result. For a search that continues
    /// across frames use the sliced forms, whose iteration count is their budget.
    #[allow(clippy::too_many_arguments)]
    pub fn find_path_budgeted<F: QueryFilter + ?Sized>(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &F,
        path: &mut Vec<PolyRef>,
        max_path: usize,
        budget: &mut Budget,
    ) -> Result<Budgeted<Status>, Error> {
        let (r, exhausted) = budget.scoped(|b| {
            self.find_path_impl(
                start_ref,
                end_ref,
                start_pos,
                end_pos,
                filter,
                path,
                max_path,
                H_SCALE,
                &mut Some(b),
            )
        });
        Ok(Budgeted::of(r?, exhausted))
    }

    #[allow(clippy::too_many_arguments)]
    fn find_path_impl<F: QueryFilter + ?Sized>(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &F,
        path: &mut Vec<PolyRef>,
        max_path: usize,
        heuristic_scale: f32,
        budget: &mut Option<&mut Budget>,
    ) -> Result<Status, Error> {
        path.clear();
        if !self.nav.is_valid_poly_ref(start_ref)
            || !self.nav.is_valid_poly_ref(end_ref)
            || !visfinite(start_pos)
            || !visfinite(end_pos)
            || max_path == 0
        {
            return Err(Error::InvalidParameter("refs, positions or max_path"));
        }
        if start_ref == end_ref {
            path.push(start_ref);
            return Ok(Status::OK);
        }
        self.node_pool.clear();
        self.open_list.clear();
        let sn = self
            .node_pool
            .get_node(start_ref, 0)
            .ok_or(Error::AllocationFailed)?;
        {
            let n = &mut self.node_pool.nodes[sn];
            n.pos = *start_pos;
            n.pidx = 0;
            n.cost = 0.0;
            n.total = path_heuristic(start_pos, end_pos, heuristic_scale);
            n.id = start_ref;
            n.flags = NODE_OPEN;
        }
        self.open_list.push(sn, &self.node_pool.nodes);
        let mut last_best = sn;
        let mut last_best_cost = self.node_pool.nodes[sn].total;
        let mut out_of_nodes = false;
        while !self.open_list.empty() {
            if !spend_one(budget) {
                break;
            }
            let bn = self.open_list.pop(&self.node_pool.nodes);
            self.node_pool.nodes[bn].flags &= !NODE_OPEN;
            self.node_pool.nodes[bn].flags |= NODE_CLOSED;
            if self.node_pool.nodes[bn].id == end_ref {
                last_best = bn;
                break;
            }
            let best = self.tp(self.node_pool.nodes[bn].id);
            let parent = NodePool::at_idx(self.node_pool.nodes[bn].pidx)
                .map(|i| self.node_pool.nodes[i].id)
                .filter(|&r| r != 0)
                .map(|r| self.tp(r));
            let parent_ref = parent.map_or(0, |p| p.0);
            let mut li = best.2.first_link;
            while li != NULL_LINK {
                let link = best.1.links[li as usize];
                li = link.next;
                let nref = link.r#ref;
                if nref == 0 || nref == parent_ref {
                    continue;
                }
                let nei = self.tp(nref);
                if !filter.pass_filter(nref, nei.1, nei.2) {
                    continue;
                }
                let cross_side = if link.side != 0xff { link.side >> 1 } else { 0 };
                let Some(nn) = self.node_pool.get_node(nref, cross_side) else {
                    out_of_nodes = true;
                    continue;
                };
                if self.node_pool.nodes[nn].flags == 0 {
                    if let Some(mid) = self.edge_mid_point_tp(best, nei) {
                        self.node_pool.nodes[nn].pos = mid;
                    }
                }
                let bpos = self.node_pool.nodes[bn].pos;
                let npos = self.node_pool.nodes[nn].pos;
                let bcost = self.node_pool.nodes[bn].cost;
                let (cost, heuristic) = if nref == end_ref {
                    let cur = filter.cost(&bpos, &npos, parent, best, Some(nei));
                    let end = filter.cost(&npos, end_pos, Some(best), nei, None);
                    (bcost + cur + end, 0.0)
                } else {
                    let cur = filter.cost(&bpos, &npos, parent, best, Some(nei));
                    (bcost + cur, path_heuristic(&npos, end_pos, heuristic_scale))
                };
                let total = cost + heuristic;
                let n = &mut self.node_pool.nodes[nn];
                if n.flags & NODE_OPEN != 0 && no_better(total, n.total) {
                    continue;
                }
                if n.flags & NODE_CLOSED != 0 && no_better(total, n.total) {
                    continue;
                }
                n.pidx = NodePool::idx(bn);
                n.id = nref;
                n.flags &= !NODE_CLOSED;
                n.cost = cost;
                n.total = total;
                if n.flags & NODE_OPEN != 0 {
                    self.open_list.modify(nn, &self.node_pool.nodes);
                } else {
                    n.flags |= NODE_OPEN;
                    self.open_list.push(nn, &self.node_pool.nodes);
                }
                if heuristic < last_best_cost {
                    last_best_cost = heuristic;
                    last_best = nn;
                }
            }
        }
        let mut status = self.path_to_node(last_best, path, max_path);
        if self.node_pool.nodes[last_best].id != end_ref {
            status |= Status::PARTIAL_RESULT;
            if self.nav.has_unloaded() && !self.search_frontier().is_empty() {
                status |= Status::UNLOADED_BORDER;
            }
        }
        if out_of_nodes {
            status |= Status::OUT_OF_NODES;
        }
        Ok(Status(status))
    }

    /// Unloaded columns facing the polygons the last A* search expanded, sorted.
    pub(crate) fn search_frontier(&self) -> Vec<TileKey> {
        self.nav.unloaded_frontier(
            self.node_pool
                .used()
                .iter()
                .filter(|n| n.flags & NODE_CLOSED != 0)
                .map(|n| n.id),
        )
    }

    pub(crate) fn diagnostic_frontier(&self, budget: &mut Budget) -> Option<Vec<TileKey>> {
        if !self.nav.has_unloaded() {
            return Some(Vec::new());
        }
        let mut out = Vec::new();
        for node in self.node_pool.used() {
            if !budget.spend(1) {
                return None;
            }
            if node.flags & NODE_CLOSED == 0 {
                continue;
            }
            let Ok((tile, poly)) = self.nav.tile_and_poly_by_ref(node.id) else {
                continue;
            };
            let ip = self.nav.decode_poly_id_poly(node.id) as usize;
            for edge in 0..usize::from(poly.vert_count) {
                if !budget.spend(1) {
                    return None;
                }
                if let EdgeKind::UnloadedBorder(key) = self.nav.edge_kind(tile, ip, edge) {
                    out.push(key);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        Some(out)
    }

    /// [`NavMeshQuery::find_path`] that also returns the frontier (not in
    /// recastnavigation): when the goal was not reached, the sorted, deduplicated
    /// columns that are known but not resident and face a polygon the search
    /// expanded (see [`NavMesh::mark_known`] and [`NavMesh::unload_tile`]). The
    /// status then carries [`Status::UNLOADED_BORDER`]. Empty when the goal was
    /// reached.
    #[allow(clippy::too_many_arguments)]
    pub fn find_path_report<F: QueryFilter + ?Sized>(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &F,
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) -> Result<(Status, Vec<TileKey>), Error> {
        let st = self.find_path(
            start_ref, end_ref, start_pos, end_pos, filter, path, max_path,
        )?;
        let frontier = if st.has_detail(Status::UNLOADED_BORDER) {
            self.search_frontier()
        } else {
            Vec::new()
        };
        Ok((st, frontier))
    }

    fn path_to_node(&self, end: usize, path: &mut Vec<PolyRef>, max_path: usize) -> u32 {
        let nodes = &self.node_pool.nodes;
        let mut length = 0usize;
        let mut cur = Some(end);
        while let Some(c) = cur {
            length += 1;
            cur = NodePool::at_idx(nodes[c].pidx);
        }
        let mut cur = Some(end);
        let mut write = length;
        while write > max_path {
            cur = cur.and_then(|c| NodePool::at_idx(nodes[c].pidx));
            write -= 1;
        }
        path.clear();
        path.resize(write, 0);
        for i in (0..write).rev() {
            let Some(c) = cur else { break };
            path[i] = nodes[c].id;
            cur = NodePool::at_idx(nodes[c].pidx);
        }
        if length > max_path {
            Status::SUCCESS | Status::BUFFER_TOO_SMALL
        } else {
            Status::SUCCESS
        }
    }

    /// Starts a sliced A* search (`initSlicedFindPath`). The filter given here must
    /// be passed again to every update and finalize call. Returns the query status
    /// (`IN_PROGRESS`, or success when start and end are the same polygon).
    #[allow(clippy::too_many_arguments)]
    pub fn init_sliced_find_path(
        &mut self,
        start_ref: PolyRef,
        end_ref: PolyRef,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        filter: &dyn QueryFilter,
        options: u32,
    ) -> Result<Status, Error> {
        let _ = filter;
        self.query = QueryData {
            status: Status::FAILURE,
            start_ref,
            end_ref,
            end_pos: *end_pos,
            options,
            raycast_limit_sqr: f32::MAX,
            ..QueryData::default()
        };
        if !self.nav.is_valid_poly_ref(start_ref)
            || !self.nav.is_valid_poly_ref(end_ref)
            || !visfinite(start_pos)
            || !visfinite(end_pos)
        {
            return Err(Error::InvalidParameter("refs or positions"));
        }
        if options & FINDPATH_ANY_ANGLE != 0 {
            let r = self
                .nav
                .tile_by_ref(start_ref)
                .map_or(0.0, |t| t.header.walkable_radius);
            self.query.raycast_limit_sqr = sqr(r * RAY_CAST_LIMIT_PROPORTIONS);
        }
        self.query.epoch = self.nav.epoch();
        if start_ref == end_ref {
            self.query.status = Status::SUCCESS;
            return Ok(Status::OK);
        }
        self.node_pool.clear();
        self.open_list.clear();
        let sn = self
            .node_pool
            .get_node(start_ref, 0)
            .ok_or(Error::AllocationFailed)?;
        {
            let n = &mut self.node_pool.nodes[sn];
            n.pos = *start_pos;
            n.pidx = 0;
            n.cost = 0.0;
            n.total = vdist(start_pos, end_pos) * H_SCALE;
            n.id = start_ref;
            n.flags = NODE_OPEN;
        }
        self.open_list.push(sn, &self.node_pool.nodes);
        self.query.status = Status::IN_PROGRESS;
        self.query.last_best_node = Some(sn);
        self.query.last_best_node_cost = self.node_pool.nodes[sn].total;
        Ok(Status(self.query.status))
    }

    fn sliced_fail(&mut self, iters: usize) -> Result<(Status, usize), Error> {
        self.query.status = Status::FAILURE;
        let _ = iters;
        Err(Error::Failed)
    }

    /// Runs up to `max_iter` iterations of the sliced search
    /// (`updateSlicedFindPath`): the query status and the iterations done.
    pub fn update_sliced_find_path(
        &mut self,
        max_iter: usize,
        filter: &dyn QueryFilter,
    ) -> Result<(Status, usize), Error> {
        let st = self.query.status;
        if st & Status::IN_PROGRESS == 0 {
            return if st & Status::FAILURE != 0 {
                Err(failure_of(st))
            } else {
                Ok((Status(st), 0))
            };
        }
        if pool_changed(self.nav, self.query.epoch, &self.node_pool) {
            self.query.status = Status::FAILURE | Status::MESH_CHANGED;
            return Err(Error::MeshChanged);
        }
        if !self.nav.is_valid_poly_ref(self.query.start_ref)
            || !self.nav.is_valid_poly_ref(self.query.end_ref)
        {
            return self.sliced_fail(0);
        }
        let mut hit = RaycastHit::default();
        let mut iter = 0usize;
        while iter < max_iter && !self.open_list.empty() {
            iter += 1;
            let bn = self.open_list.pop(&self.node_pool.nodes);
            self.node_pool.nodes[bn].flags &= !NODE_OPEN;
            self.node_pool.nodes[bn].flags |= NODE_CLOSED;
            if self.node_pool.nodes[bn].id == self.query.end_ref {
                self.query.last_best_node = Some(bn);
                self.query.status = Status::SUCCESS | (self.query.status & DETAIL_MASK);
                return Ok((Status(self.query.status), iter));
            }
            let best_ref = self.node_pool.nodes[bn].id;
            let Ok(best) = self.valid_tp(best_ref) else {
                return self.sliced_fail(iter);
            };
            let parent_node = NodePool::at_idx(self.node_pool.nodes[bn].pidx);
            let parent_ref = parent_node.map_or(0, |p| self.node_pool.nodes[p].id);
            let grandpa_ref = parent_node
                .and_then(|p| NodePool::at_idx(self.node_pool.nodes[p].pidx))
                .map_or(0, |g| self.node_pool.nodes[g].id);
            let mut parent: Option<Tp<'a>> = None;
            if parent_ref != 0 {
                match self.valid_tp(parent_ref) {
                    Ok(p) if grandpa_ref == 0 || self.nav.is_valid_poly_ref(grandpa_ref) => {
                        parent = Some(p)
                    }
                    _ => return self.sliced_fail(iter),
                }
            }
            let mut try_los = false;
            if self.query.options & FINDPATH_ANY_ANGLE != 0 {
                if let Some(pn) = parent_node {
                    if parent_ref != 0
                        && vdist_sqr(&self.node_pool.nodes[pn].pos, &self.node_pool.nodes[bn].pos)
                            < self.query.raycast_limit_sqr
                    {
                        try_los = true;
                    }
                }
            }
            let mut li = best.2.first_link;
            while li != NULL_LINK {
                let link = best.1.links[li as usize];
                li = link.next;
                let nref = link.r#ref;
                if nref == 0 || nref == parent_ref {
                    continue;
                }
                let nei = self.tp(nref);
                if !filter.pass_filter(nref, nei.1, nei.2) {
                    continue;
                }
                let cross_side =
                    if self.fixes.contains(UpstreamFixes::SLICED_CROSS_SIDE) && link.side != 0xff {
                        link.side >> 1
                    } else {
                        0
                    };
                let Some(nn) = self.node_pool.get_node(nref, cross_side) else {
                    self.query.status |= Status::OUT_OF_NODES;
                    continue;
                };
                if self.node_pool.nodes[nn].pidx != 0
                    && self.node_pool.nodes[nn].pidx == self.node_pool.nodes[bn].pidx
                {
                    continue;
                }
                if self.node_pool.nodes[nn].flags == 0 {
                    if let Some(mid) = self.edge_mid_point_tp(best, nei) {
                        self.node_pool.nodes[nn].pos = mid;
                    }
                }
                let mut found_short_cut = false;
                hit.path_cost = 0.0;
                hit.t = 0.0;
                let npos = self.node_pool.nodes[nn].pos;
                let mut cost;
                if try_los {
                    if let Some(pn) = parent_node {
                        let ppos = self.node_pool.nodes[pn].pos;
                        hit.path.clear();
                        if let Ok((h, _)) = self.raycast_with(
                            parent_ref,
                            &ppos,
                            &npos,
                            filter,
                            RAYCAST_USE_COSTS,
                            0,
                            grandpa_ref,
                        ) {
                            hit.t = h.t;
                            hit.path_cost = h.path_cost;
                        }
                        found_short_cut = hit.t >= 1.0;
                    }
                }
                if found_short_cut {
                    let pcost = parent_node.map_or(0.0, |p| self.node_pool.nodes[p].cost);
                    cost = pcost + hit.path_cost;
                } else {
                    let bpos = self.node_pool.nodes[bn].pos;
                    cost = self.node_pool.nodes[bn].cost
                        + filter.cost(&bpos, &npos, parent, best, Some(nei));
                }
                let heuristic;
                if nref == self.query.end_ref {
                    let end_pos = self.query.end_pos;
                    cost += filter.cost(&npos, &end_pos, Some(best), nei, None);
                    heuristic = 0.0;
                } else {
                    heuristic = vdist(&npos, &self.query.end_pos) * H_SCALE;
                }
                let total = cost + heuristic;
                let bpidx = self.node_pool.nodes[bn].pidx;
                let n = &mut self.node_pool.nodes[nn];
                if n.flags & NODE_OPEN != 0 && no_better(total, n.total) {
                    continue;
                }
                if n.flags & NODE_CLOSED != 0 && no_better(total, n.total) {
                    continue;
                }
                n.pidx = if found_short_cut {
                    bpidx
                } else {
                    NodePool::idx(bn)
                };
                n.id = nref;
                n.flags &= !(NODE_CLOSED | NODE_PARENT_DETACHED);
                n.cost = cost;
                n.total = total;
                if found_short_cut {
                    n.flags |= NODE_PARENT_DETACHED;
                }
                if n.flags & NODE_OPEN != 0 {
                    self.open_list.modify(nn, &self.node_pool.nodes);
                } else {
                    n.flags |= NODE_OPEN;
                    self.open_list.push(nn, &self.node_pool.nodes);
                }
                if heuristic < self.query.last_best_node_cost {
                    self.query.last_best_node_cost = heuristic;
                    self.query.last_best_node = Some(nn);
                }
            }
        }
        if self.open_list.empty() {
            self.query.status = Status::SUCCESS | (self.query.status & DETAIL_MASK);
        }
        Ok((Status(self.query.status), iter))
    }

    /// Finishes a sliced search (`finalizeSlicedFindPath`).
    pub fn finalize_sliced_find_path(
        &mut self,
        filter: &dyn QueryFilter,
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) -> Result<Status, Error> {
        path.clear();
        if max_path == 0 {
            return Err(Error::InvalidParameter("max_path"));
        }
        if self.query.status & Status::FAILURE != 0 {
            self.query = QueryData::default();
            return Err(Error::Failed);
        }
        if self.query.start_ref == self.query.end_ref {
            path.push(self.query.start_ref);
        } else {
            let lb = self.query.last_best_node.ok_or(Error::Failed)?;
            if self.node_pool.nodes[lb].id != self.query.end_ref {
                self.query.status |= Status::PARTIAL_RESULT;
                if self.nav.has_unloaded() && !self.search_frontier().is_empty() {
                    self.query.status |= Status::UNLOADED_BORDER;
                }
            }
            self.write_sliced_path(lb, filter, path, max_path);
        }
        let details = self.query.status & DETAIL_MASK;
        self.query = QueryData::default();
        Ok(Status(Status::SUCCESS | details))
    }

    /// Finishes a sliced search towards the furthest polygon of `existing` that the
    /// search visited (`finalizeSlicedFindPathPartial`).
    pub fn finalize_sliced_find_path_partial(
        &mut self,
        filter: &dyn QueryFilter,
        existing: &[PolyRef],
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) -> Result<Status, Error> {
        path.clear();
        if existing.is_empty() || max_path == 0 {
            return Err(Error::InvalidParameter("existing or max_path"));
        }
        if self.query.status & Status::FAILURE != 0 {
            self.query = QueryData::default();
            return Err(Error::Failed);
        }
        if self.query.start_ref == self.query.end_ref {
            path.push(self.query.start_ref);
        } else {
            let mut node = None;
            let mut found = Vec::new();
            for &r in existing.iter().rev() {
                self.node_pool.find_nodes(r, &mut found, 1);
                if let Some(&n) = found.first() {
                    node = Some(n);
                    break;
                }
            }
            let node = match node {
                Some(n) => n,
                None => {
                    self.query.status |= Status::PARTIAL_RESULT;
                    self.query.last_best_node.ok_or(Error::Failed)?
                }
            };
            if self.node_pool.nodes[node].id != self.query.end_ref
                && self.nav.has_unloaded()
                && !self.search_frontier().is_empty()
            {
                self.query.status |= Status::UNLOADED_BORDER;
            }
            self.write_sliced_path(node, filter, path, max_path);
        }
        let details = self.query.status & DETAIL_MASK;
        self.query = QueryData::default();
        Ok(Status(Status::SUCCESS | details))
    }

    fn write_sliced_path(
        &mut self,
        start: usize,
        filter: &dyn QueryFilter,
        path: &mut Vec<PolyRef>,
        max_path: usize,
    ) {
        // Reverse the parent chain, moving the detached flags with it.
        let mut prev: Option<usize> = None;
        let mut node = Some(start);
        let mut prev_ray = 0u8;
        while let Some(n) = node {
            let next = NodePool::at_idx(self.node_pool.nodes[n].pidx);
            let nd = &mut self.node_pool.nodes[n];
            nd.pidx = prev.map_or(0, NodePool::idx);
            prev = Some(n);
            let next_ray = nd.flags & NODE_PARENT_DETACHED;
            nd.flags = (nd.flags & !NODE_PARENT_DETACHED) | prev_ray;
            prev_ray = next_ray;
            node = next;
        }
        let mut node = prev;
        let mut tmp = Vec::new();
        while let Some(n) = node {
            let next = NodePool::at_idx(self.node_pool.nodes[n].pidx);
            let mut status = 0u32;
            if self.node_pool.nodes[n].flags & NODE_PARENT_DETACHED != 0 {
                let (id, pos) = (self.node_pool.nodes[n].id, self.node_pool.nodes[n].pos);
                let next_n =
                    next.map(|x| (self.node_pool.nodes[x].id, self.node_pool.nodes[x].pos));
                if let Some((nid, npos)) = next_n {
                    match self.raycast_with(id, &pos, &npos, filter, 0, max_path - path.len(), 0) {
                        Ok((h, st)) => {
                            status = st.0;
                            tmp.clear();
                            tmp.extend_from_slice(&h.path);
                        }
                        Err(e) => {
                            status = super::status_of(&e).0;
                            tmp.clear();
                        }
                    }
                    path.extend_from_slice(&tmp);
                    if path.last() == Some(&nid) {
                        path.pop();
                    }
                }
            } else {
                path.push(self.node_pool.nodes[n].id);
                if path.len() >= max_path {
                    status = Status::BUFFER_TOO_SMALL;
                }
            }
            if status & DETAIL_MASK != 0 {
                self.query.status |= status & DETAIL_MASK;
                break;
            }
            node = next;
        }
    }

    /// Straight path (string pulling) through a polygon corridor
    /// (`findStraightPath`), at most `max` vertices, with
    /// `STRAIGHTPATH_*_CROSSINGS` options.
    pub fn find_straight_path(
        &self,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        path: &[PolyRef],
        out: &mut Vec<StraightPathPoint>,
        max: usize,
        options: i32,
    ) -> Result<Status, Error> {
        self.find_straight_path_impl(start_pos, end_pos, path, out, max, options, &mut None)
    }

    /// [`NavMeshQuery::find_straight_path`] spending one unit of `budget` per portal
    /// examined by the funnel, restarts included (not in recastnavigation). When
    /// the budget runs out `out` holds the corners found so far, without the end
    /// vertex, and the status is `SUCCESS | PARTIAL_RESULT`.
    #[allow(clippy::too_many_arguments)]
    pub fn find_straight_path_budgeted(
        &self,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        path: &[PolyRef],
        out: &mut Vec<StraightPathPoint>,
        max: usize,
        options: i32,
        budget: &mut Budget,
    ) -> Result<Budgeted<Status>, Error> {
        let (r, exhausted) = budget.scoped(|b| {
            self.find_straight_path_impl(start_pos, end_pos, path, out, max, options, &mut Some(b))
        });
        Ok(Budgeted::of(r?, exhausted))
    }

    #[allow(clippy::too_many_arguments)]
    fn find_straight_path_impl(
        &self,
        start_pos: &[f32; 3],
        end_pos: &[f32; 3],
        path: &[PolyRef],
        out: &mut Vec<StraightPathPoint>,
        max: usize,
        options: i32,
        budget: &mut Option<&mut Budget>,
    ) -> Result<Status, Error> {
        out.clear();
        if !visfinite(start_pos)
            || !visfinite(end_pos)
            || path.is_empty()
            || path[0] == 0
            || max == 0
        {
            return Err(Error::InvalidParameter("positions, path or max"));
        }
        let closest_start = self
            .closest_point_on_poly_boundary(path[0], start_pos)
            .map_err(|_| Error::InvalidParameter("path start"))?;
        let mut closest_end = self
            .closest_point_on_poly_boundary(path[path.len() - 1], end_pos)
            .map_err(|_| Error::InvalidParameter("path end"))?;
        let stat = append_vertex(&closest_start, STRAIGHTPATH_START, path[0], out, max);
        if stat != Status::IN_PROGRESS {
            return Ok(Status(stat));
        }
        let crossings = options & (STRAIGHTPATH_AREA_CROSSINGS | STRAIGHTPATH_ALL_CROSSINGS) != 0;
        let n = path.len();
        if n > 1 {
            let mut apex = closest_start;
            let mut pleft = apex;
            let mut pright = apex;
            let (mut apex_index, mut left_index, mut right_index) = (0usize, 0usize, 0usize);
            let (mut left_type, mut right_type) = (0u8, 0u8);
            let (mut left_ref, mut right_ref) = (path[0], path[0]);
            let mut i = 0usize;
            while i < n {
                if !spend_one(budget) {
                    return Ok(Status(Status::SUCCESS | Status::PARTIAL_RESULT));
                }
                let (mut left, mut right);
                let to_type;
                if i + 1 < n {
                    let pp = self.valid_tp(path[i]).and_then(|f| {
                        let t = self.valid_tp(path[i + 1])?;
                        Ok((t.2.poly_type(), self.portal_points_tp(f, t)))
                    });
                    match pp {
                        Ok((tt, Some((l, r)))) => {
                            left = l;
                            right = r;
                            to_type = tt;
                        }
                        _ => {
                            closest_end = self
                                .closest_point_on_poly_boundary(path[i], end_pos)
                                .map_err(|_| Error::InvalidParameter("path"))?;
                            if crossings {
                                let _ = self.append_portals(
                                    apex_index,
                                    i,
                                    &closest_end,
                                    path,
                                    out,
                                    max,
                                    options,
                                );
                            }
                            append_vertex(&closest_end, 0, path[i], out, max);
                            return Ok(Status(
                                Status::SUCCESS
                                    | Status::PARTIAL_RESULT
                                    | if out.len() >= max {
                                        Status::BUFFER_TOO_SMALL
                                    } else {
                                        0
                                    },
                            ));
                        }
                    }
                    if i == 0 {
                        let (d, _) = distance_pt_seg_sqr_2d(&apex, &left, &right);
                        if d < sqr(0.001) {
                            i += 1;
                            continue;
                        }
                    }
                } else {
                    left = closest_end;
                    right = closest_end;
                    to_type = POLYTYPE_GROUND;
                }
                if tri_area_2d(&apex, &pright, &right) <= 0.0 {
                    if vequal(&apex, &pright) || tri_area_2d(&apex, &pleft, &right) > 0.0 {
                        pright = right;
                        right_ref = if i + 1 < n { path[i + 1] } else { 0 };
                        right_type = to_type;
                        right_index = i;
                    } else {
                        if crossings {
                            let stat = self.append_portals(
                                apex_index, left_index, &pleft, path, out, max, options,
                            )?;
                            if stat != Status::IN_PROGRESS {
                                return Ok(Status(stat));
                            }
                        }
                        apex = pleft;
                        apex_index = left_index;
                        let flags = if left_ref == 0 {
                            STRAIGHTPATH_END
                        } else if left_type == POLYTYPE_OFFMESH_CONNECTION {
                            STRAIGHTPATH_OFFMESH_CONNECTION
                        } else {
                            0
                        };
                        let stat = append_vertex(&apex, flags, left_ref, out, max);
                        if stat != Status::IN_PROGRESS {
                            return Ok(Status(stat));
                        }
                        pleft = apex;
                        pright = apex;
                        left_index = apex_index;
                        right_index = apex_index;
                        i = apex_index + 1;
                        continue;
                    }
                }
                if tri_area_2d(&apex, &pleft, &left) >= 0.0 {
                    if vequal(&apex, &pleft) || tri_area_2d(&apex, &pright, &left) < 0.0 {
                        pleft = left;
                        left_ref = if i + 1 < n { path[i + 1] } else { 0 };
                        left_type = to_type;
                        left_index = i;
                    } else {
                        if crossings {
                            let stat = self.append_portals(
                                apex_index,
                                right_index,
                                &pright,
                                path,
                                out,
                                max,
                                options,
                            )?;
                            if stat != Status::IN_PROGRESS {
                                return Ok(Status(stat));
                            }
                        }
                        apex = pright;
                        apex_index = right_index;
                        let flags = if right_ref == 0 {
                            STRAIGHTPATH_END
                        } else if right_type == POLYTYPE_OFFMESH_CONNECTION {
                            STRAIGHTPATH_OFFMESH_CONNECTION
                        } else {
                            0
                        };
                        let stat = append_vertex(&apex, flags, right_ref, out, max);
                        if stat != Status::IN_PROGRESS {
                            return Ok(Status(stat));
                        }
                        pleft = apex;
                        pright = apex;
                        left_index = apex_index;
                        right_index = apex_index;
                        i = apex_index + 1;
                        continue;
                    }
                }
                let _ = (&mut left, &mut right);
                i += 1;
            }
            if crossings {
                let stat =
                    self.append_portals(apex_index, n - 1, &closest_end, path, out, max, options)?;
                if stat != Status::IN_PROGRESS {
                    return Ok(Status(stat));
                }
            }
        }
        append_vertex(&closest_end, STRAIGHTPATH_END, 0, out, max);
        Ok(Status(
            Status::SUCCESS
                | if out.len() >= max {
                    Status::BUFFER_TOO_SMALL
                } else {
                    0
                },
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn append_portals(
        &self,
        start_idx: usize,
        end_idx: usize,
        end_pos: &[f32; 3],
        path: &[PolyRef],
        out: &mut Vec<StraightPathPoint>,
        max: usize,
        options: i32,
    ) -> Result<u32, Error> {
        let start_pos = out.last().map_or([0.0; 3], |p| p.pos);
        for i in start_idx..end_idx {
            let from = self
                .valid_tp(path[i])
                .map_err(|_| Error::InvalidParameter("path"))?;
            let to = self
                .valid_tp(path[i + 1])
                .map_err(|_| Error::InvalidParameter("path"))?;
            let Some((left, right)) = self.portal_points_tp(from, to) else {
                break;
            };
            if options & STRAIGHTPATH_AREA_CROSSINGS != 0 && from.2.area() == to.2.area() {
                continue;
            }
            if let Some((_, t)) = intersect_seg_seg_2d(&start_pos, end_pos, &left, &right) {
                let pt = vlerp(&left, &right, t);
                let stat = append_vertex(&pt, 0, path[i + 1], out, max);
                if stat != Status::IN_PROGRESS {
                    return Ok(stat);
                }
            }
        }
        Ok(Status::IN_PROGRESS)
    }
}

fn failure_of(st: u32) -> Error {
    match st & DETAIL_MASK {
        0 => Error::Failed,
        Status::MESH_CHANGED => Error::MeshChanged,
        _ => Error::InvalidParameter("sliced query"),
    }
}

fn append_vertex(
    pos: &[f32; 3],
    flags: u8,
    r: PolyRef,
    out: &mut Vec<StraightPathPoint>,
    max: usize,
) -> u32 {
    if let Some(last) = out.last_mut().filter(|l| vequal(&l.pos, pos)) {
        last.flags = flags;
        last.r#ref = r;
    } else {
        out.push(StraightPathPoint {
            pos: *pos,
            flags,
            r#ref: r,
        });
        if out.len() >= max {
            return Status::SUCCESS | Status::BUFFER_TOO_SMALL;
        }
        if flags == STRAIGHTPATH_END {
            return Status::SUCCESS;
        }
    }
    Status::IN_PROGRESS
}

pub(crate) fn poly_area_2d(t: &MeshTile, p: &Poly) -> f32 {
    let mut area = 0.0f32;
    let va = &t.verts[p.verts[0] as usize * 3..];
    for j in 2..p.vert_count as usize {
        let vb = &t.verts[p.verts[j - 1] as usize * 3..];
        let vc = &t.verts[p.verts[j] as usize * 3..];
        area += tri_area_2d(va, vb, vc);
    }
    area
}

pub(crate) fn closest_point_on_boundary(tile: &MeshTile, poly: &Poly, pos: &[f32; 3]) -> [f32; 3] {
    let mut verts = [0.0f32; VERTS_PER_POLYGON * 3];
    let mut ed = [0.0f32; VERTS_PER_POLYGON];
    let mut et = [0.0f32; VERTS_PER_POLYGON];
    let nv = poly.vert_count as usize;
    for i in 0..nv {
        let v = poly.verts[i] as usize * 3;
        verts[i * 3..i * 3 + 3].copy_from_slice(&tile.verts[v..v + 3]);
    }
    if distance_pt_poly_edges_sqr(pos, &verts, nv, &mut ed, &mut et) {
        return *pos;
    }
    let mut dmin = ed[0];
    let mut imin = 0;
    for i in 1..nv {
        if ed[i] < dmin {
            dmin = ed[i];
            imin = i;
        }
    }
    vlerp(
        &verts[imin * 3..],
        &verts[((imin + 1) % nv) * 3..],
        et[imin],
    )
}

// A detached query retains sliced-search state while the caller mutates tiles
// between crowd updates. Attaching borrows the navmesh only for that update.
//
// superfargoth patch (own: public detached query): `DetachedQuery` was crate-private and
// built only with `crowd` (for `PathQueue`). Public and ungated here, so a caller can own
// sliced A* state (one detached query per search, poolable: no lifetime) and attach it to
// a borrowed navmesh only for each step, like the resumable searches. To offer upstream.
/// A [`NavMeshQuery`] without its navmesh borrow (not in recastnavigation): the node
/// pools, the open list and the sliced-search state, kept across steps while the
/// caller mutates tiles. [`DetachedQuery::attach`] borrows a navmesh for one step and
/// [`NavMeshQuery::detach`] gives the state back. Attach it to the navmesh it was
/// created for: a sliced search continued on another navmesh fails or reads unrelated
/// polygons (never out of bounds). Tile changes between steps behave as for
/// [`NavMeshQuery::update_sliced_find_path`]: removing a tile the search touched fails
/// it with [`Error::MeshChanged`]; added tiles do not.
pub struct DetachedQuery {
    node_pool: NodePool,
    tiny_node_pool: NodePool,
    open_list: NodeQueue,
    query: QueryData,
    fixes: UpstreamFixes,
}
impl DetachedQuery {
    /// A detached query with `max_nodes` search nodes, as [`NavMeshQuery::new`].
    pub fn new(nav: &NavMesh, max_nodes: usize) -> Result<Self, Error> {
        Ok(NavMeshQuery::new(nav, max_nodes)?.detach())
    }
    /// Selects the upstream defect corrections, as [`NavMeshQuery::set_fixes`].
    pub fn set_fixes(&mut self, fixes: UpstreamFixes) {
        self.fixes = fixes;
    }
    /// A query over `nav` that carries this state.
    pub fn attach(self, nav: &NavMesh) -> NavMeshQuery<'_> {
        NavMeshQuery {
            nav,
            node_pool: self.node_pool,
            tiny_node_pool: self.tiny_node_pool,
            open_list: self.open_list,
            query: self.query,
            fixes: self.fixes,
        }
    }
}
impl NavMeshQuery<'_> {
    /// Releases the navmesh borrow and keeps the state (see [`DetachedQuery`]).
    pub fn detach(self) -> DetachedQuery {
        DetachedQuery {
            node_pool: self.node_pool,
            tiny_node_pool: self.tiny_node_pool,
            open_list: self.open_list,
            query: self.query,
            fixes: self.fixes,
        }
    }
}
