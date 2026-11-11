//! Epoch-scoped, unfiltered connectivity and budgeted path explanations.
//!
//! Labels describe **weak** connectivity: a one-way connection joins two islands,
//! but does not prove travel in both directions. [`Connectivity::reachable_from`]
//! follows directed links. Labels are the smallest resident polygon reference in
//! the component; retain `(epoch, global_off_mesh_links, label)`, never a label alone. Call [`Connectivity::update`]
//! after structural edits, before consulting labels. Flag/area edits need no update:
//! this graph deliberately ignores filters. A refused update leaves the old snapshot intact.

use crate::detour::{
    EXT_LINK, NULL_LINK, NavMesh, NavMeshQuery, PolyRef, QueryFilter, Status, TileKey,
};
use crate::{Budget, Budgeted, Error};
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Version of the serialized report field contract.
pub const SCHEMA_VERSION: u32 = 1;

/// The minimum polygon reference in a weak component, meaningful at one epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IslandId(pub PolyRef);

/// Unfiltered component size and projected geometry.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct IslandSummary {
    /// Minimum member reference, scoped to the containing report's epoch.
    pub id: Option<IslandId>,
    /// Number of polygons, including off-mesh connection polygons.
    pub polys: u32,
    /// Sum of ground polygon areas projected onto xz; overlapping layers count separately.
    pub xz_area: f64,
    /// Detour side bits (0, 2, 4, 6) touched by ground polygon border edges.
    pub border_sides: u8,
}

/// Unfiltered world statistics over the resident mesh only.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Reachability {
    /// Field contract version.
    pub schema_version: u32,
    /// Structural mesh epoch of these labels.
    pub epoch: u64,
    /// Topology mode used to resolve far off-mesh endpoints.
    pub global_off_mesh_links: bool,
    /// Weak component summaries ordered by minimum reference.
    pub islands: Vec<IslandSummary>,
    /// Number of weak components.
    pub island_count: u32,
    /// Largest component's polygon count divided by all resident polygons (zero for empty).
    pub largest_island_share: f64,
    /// Components containing fewer than the requested polygon threshold.
    pub islands_below_threshold: u32,
    /// Resident polygon count, including off-mesh polygons.
    pub total_polys: u32,
}

/// Directed coverage from a caller's seed references.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReachableCoverage {
    /// Field contract version.
    pub schema_version: u32,
    /// Structural mesh epoch of this result.
    pub epoch: u64,
    /// Topology mode used by this snapshot.
    pub global_off_mesh_links: bool,
    /// Number of reached polygons (duplicates in the seed list count once).
    pub reachable_polys: u32,
    /// All resident polygons.
    pub total_polys: u32,
    /// Reached divided by total; zero for an empty world.
    pub share: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct Node {
    local: IslandId,
    edges: Vec<PolyRef>,
    summary: IslandSummary,
}

/// Cached tile-local and world labels for an unfiltered resident mesh.
///
/// Updates scan resident polygons/links to detect changes; only affected weak
/// components are relabeled with union-find. Removals invalidate the entire old
/// component, so splits are handled exactly. Work units charge polygons, links,
/// allocated slots, component members and union operations. No partial snapshot is published.
/// The cache is tied to the particular mesh supplied at construction; updates
/// on another mesh, including a clone, return [`Error::MeshChanged`]. Moving the
/// mesh between calls is fine.
#[derive(Clone, Debug)]
pub struct Connectivity {
    identity: Arc<()>,
    epoch: u64,
    global_off_mesh: bool,
    nodes: BTreeMap<PolyRef, Node>,
    labels: BTreeMap<PolyRef, IslandId>,
}

fn charge(budget: &mut Budget) -> Result<(), Error> {
    if budget.spend(1) {
        Ok(())
    } else {
        Err(Error::LimitExceeded)
    }
}

fn root(parent: &mut BTreeMap<PolyRef, PolyRef>, mut r: PolyRef) -> PolyRef {
    while let Some(&p) = parent.get(&r) {
        if p == r {
            break;
        }
        let next = parent.get(&p).copied().unwrap_or(p);
        parent.insert(r, next);
        r = next;
    }
    r
}

fn union(parent: &mut BTreeMap<PolyRef, PolyRef>, a: PolyRef, b: PolyRef) {
    let a = root(parent, a);
    let b = root(parent, b);
    if a != b {
        parent.insert(a.max(b), a.min(b));
    }
}

/// Polygon area and side summary, also shared by per-tile bake reports.
pub(crate) fn polygon_summary(
    tile: &crate::detour::MeshTile,
    poly: &crate::detour::Poly,
) -> IslandSummary {
    let mut area = 0.0f64;
    let n = usize::from(poly.vert_count);
    if poly.poly_type() == crate::detour::POLYTYPE_GROUND {
        // Triangulate about the first vertex to avoid cancellation far from the origin.
        if n >= 3 {
            let a = usize::from(poly.verts[0]) * 3;
            for i in 1..n - 1 {
                let b = usize::from(poly.verts[i]) * 3;
                let c = usize::from(poly.verts[i + 1]) * 3;
                area += ((f64::from(tile.verts[b]) - f64::from(tile.verts[a]))
                    * (f64::from(tile.verts[c + 2]) - f64::from(tile.verts[a + 2]))
                    - (f64::from(tile.verts[c]) - f64::from(tile.verts[a]))
                        * (f64::from(tile.verts[b + 2]) - f64::from(tile.verts[a + 2])))
                .abs()
                    * 0.5;
            }
        }
    }
    let mut sides = 0;
    for &nei in &poly.neis[..n] {
        if nei & EXT_LINK != 0 {
            sides |= 1 << (nei & 7);
        }
    }
    IslandSummary {
        id: None,
        polys: 1,
        xz_area: area,
        border_sides: sides,
    }
}

impl Connectivity {
    /// Builds a snapshot; exhaustion returns no snapshot. One unit per polygon/link/union.
    pub fn new(nav: &NavMesh, budget: &mut Budget) -> Result<Budgeted<Option<Self>>, Error> {
        let mut graph = Self {
            identity: Arc::clone(nav.report_identity()),
            epoch: nav.epoch(),
            global_off_mesh: nav.global_off_mesh_links(),
            nodes: BTreeMap::new(),
            labels: BTreeMap::new(),
        };
        match graph.update_inner(nav, budget) {
            Ok(()) => Ok(Budgeted::Complete(Some(graph))),
            Err(Error::LimitExceeded) if budget.is_exhausted() => Ok(Budgeted::Exhausted(None)),
            Err(e) => Err(e),
        }
    }

    /// Epoch for which labels are valid; compare with [`NavMesh::epoch`].
    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Refreshes after commits, unloads or removals; exhausted calls preserve the old snapshot.
    pub fn update(&mut self, nav: &NavMesh, budget: &mut Budget) -> Result<Budgeted<()>, Error> {
        match self.update_inner(nav, budget) {
            Ok(()) => Ok(Budgeted::Complete(())),
            Err(Error::LimitExceeded) if budget.is_exhausted() => Ok(Budgeted::Exhausted(())),
            Err(e) => Err(e),
        }
    }

    fn update_inner(&mut self, nav: &NavMesh, budget: &mut Budget) -> Result<(), Error> {
        charge(budget)?;
        if !Arc::ptr_eq(&self.identity, nav.report_identity()) {
            return Err(Error::MeshChanged);
        }
        let mut nodes = BTreeMap::new();
        for slot in nav.allocated_slots() {
            charge(budget)?;
            let Some(tile) = nav.tile(slot) else {
                continue;
            };
            let base = nav.tile_ref(slot);
            let mut local = BTreeMap::new();
            for (i, poly) in tile.polys.iter().enumerate() {
                charge(budget)?;
                let r = base | i as PolyRef;
                local.insert(r, r);
                let mut edges = Vec::new();
                let mut link = poly.first_link;
                while link != NULL_LINK {
                    charge(budget)?;
                    let l = &tile.links[link as usize];
                    if l.r#ref != 0 {
                        edges.push(l.r#ref);
                    }
                    link = l.next;
                }
                edges.sort_unstable();
                edges.dedup();
                nodes.insert(
                    r,
                    Node {
                        local: IslandId(r),
                        edges,
                        summary: polygon_summary(tile, poly),
                    },
                );
            }
            for (&r, node) in
                nodes.range(base..=(base | (tile.polys.len().saturating_sub(1) as PolyRef)))
            {
                for &to in &node.edges {
                    charge(budget)?;
                    if local.contains_key(&to) {
                        union(&mut local, r, to);
                    }
                }
            }
            for &r in local.clone().keys() {
                charge(budget)?;
                if let Some(node) = nodes.get_mut(&r) {
                    node.local = IslandId(root(&mut local, r));
                }
            }
        }
        // Mark changed components in the previous graph, plus all components newly
        // joined by a changed edge. Unchanged components keep their labels.
        let mut affected = BTreeSet::new();
        let mut changed = BTreeSet::new();
        for (&r, old) in &self.nodes {
            charge(budget)?;
            if nodes.get(&r) != Some(old) {
                changed.insert(r);
                if let Some(id) = self.labels.get(&r) {
                    affected.insert(*id);
                }
            }
        }
        for (&r, node) in &nodes {
            charge(budget)?;
            if self.nodes.get(&r) != Some(node) {
                changed.insert(r);
                if let Some(id) = self.labels.get(&r) {
                    affected.insert(*id);
                }
                for to in &node.edges {
                    charge(budget)?;
                    if let Some(id) = self.labels.get(to) {
                        affected.insert(*id);
                    }
                }
            }
        }
        let mut parent = BTreeMap::new();
        let mut labels = BTreeMap::new();
        for &r in nodes.keys() {
            charge(budget)?;
            match self.labels.get(&r) {
                Some(id) if !affected.contains(id) && !changed.contains(&r) => {
                    labels.insert(r, *id);
                }
                _ => {
                    parent.insert(r, r);
                }
            }
        }
        for (&r, node) in &nodes {
            charge(budget)?;
            for &to in &node.edges {
                charge(budget)?;
                if parent.contains_key(&r) && parent.contains_key(&to) {
                    union(&mut parent, r, to);
                }
            }
        }
        let keys: Vec<_> = parent.keys().copied().collect();
        for r in keys {
            charge(budget)?;
            labels.insert(r, IslandId(root(&mut parent, r)));
        }
        self.nodes = nodes;
        self.labels = labels;
        self.epoch = nav.epoch();
        self.global_off_mesh = nav.global_off_mesh_links();
        Ok(())
    }

    /// Whether owner identity, epoch and global-link mode still match this mesh.
    pub fn is_current(&self, nav: &NavMesh) -> bool {
        Arc::ptr_eq(&self.identity, nav.report_identity())
            && self.epoch == nav.epoch()
            && self.global_off_mesh == nav.global_off_mesh_links()
    }

    /// World weak component of a resident polygon, or `None` for refs absent at this epoch.
    pub fn island(&self, r: PolyRef) -> Option<IslandId> {
        self.labels.get(&r).copied()
    }

    /// Tile-local weak component; these labels are shared with hierarchy consumers.
    /// External links do not join local components, including external off-mesh endpoints.
    pub fn tile_island(&self, r: PolyRef) -> Option<IslandId> {
        self.nodes.get(&r).map(|n| n.local)
    }

    /// Statistics at this epoch; `small_threshold` is an exclusive polygon-count threshold.
    pub fn reachability(
        &self,
        small_threshold: u32,
        budget: &mut Budget,
    ) -> Result<Budgeted<Option<Reachability>>, Error> {
        let mut groups: BTreeMap<IslandId, IslandSummary> = BTreeMap::new();
        for (&r, node) in &self.nodes {
            if !budget.spend(1) {
                return Ok(Budgeted::Exhausted(None));
            }
            let id = self.labels[&r];
            let s = groups.entry(id).or_default();
            s.id = Some(id);
            s.polys = s.polys.checked_add(1).ok_or(Error::SizeOverflow)?;
            s.xz_area += node.summary.xz_area;
            s.border_sides |= node.summary.border_sides;
        }
        let total = u32::try_from(self.nodes.len()).map_err(|_| Error::SizeOverflow)?;
        let largest = groups.values().map(|s| s.polys).max().unwrap_or(0);
        Ok(Budgeted::Complete(Some(Reachability {
            schema_version: SCHEMA_VERSION,
            epoch: self.epoch,
            global_off_mesh_links: self.global_off_mesh,
            island_count: u32::try_from(groups.len()).map_err(|_| Error::SizeOverflow)?,
            largest_island_share: if total == 0 {
                0.0
            } else {
                f64::from(largest) / f64::from(total)
            },
            islands_below_threshold: groups
                .values()
                .filter(|s| s.polys < small_threshold)
                .count() as u32,
            total_polys: total,
            islands: groups.into_values().collect(),
        })))
    }

    /// Exact directed resident coverage, ignoring query filters. Invalid seeds return [`Error::InvalidRef`].
    pub fn reachable_from(
        &self,
        seeds: &[PolyRef],
        budget: &mut Budget,
    ) -> Result<Budgeted<Option<ReachableCoverage>>, Error> {
        let mut seen = BTreeSet::new();
        let mut stack = Vec::new();
        for &seed in seeds {
            if !budget.spend(1) {
                return Ok(Budgeted::Exhausted(None));
            }
            if !self.nodes.contains_key(&seed) {
                return Err(Error::InvalidRef);
            }
            if seen.insert(seed) {
                stack.push(seed);
            }
        }
        while let Some(r) = stack.pop() {
            if !budget.spend(1) {
                return Ok(Budgeted::Exhausted(None));
            }
            for &to in &self.nodes[&r].edges {
                if !budget.spend(1) {
                    return Ok(Budgeted::Exhausted(None));
                }
                if self.nodes.contains_key(&to) && seen.insert(to) {
                    stack.push(to);
                }
            }
        }
        let total = u32::try_from(self.nodes.len()).map_err(|_| Error::SizeOverflow)?;
        let reached = u32::try_from(seen.len()).map_err(|_| Error::SizeOverflow)?;
        Ok(Budgeted::Complete(Some(ReachableCoverage {
            schema_version: SCHEMA_VERSION,
            epoch: self.epoch,
            global_off_mesh_links: self.global_off_mesh,
            reachable_polys: reached,
            total_polys: total,
            share: if total == 0 {
                0.0
            } else {
                f64::from(reached) / f64::from(total)
            },
        })))
    }
}

/// A typed failure to establish a path. Precedence is documented on [`diagnose_path`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum NoPathReason {
    /// No start polygon in the supplied search box; optional nearest ref and Euclidean distance.
    StartOffMesh {
        /// Unfiltered candidate, if any.
        nearest: Option<(PolyRef, f32)>,
    },
    /// No goal polygon in the supplied search box.
    GoalOffMesh {
        /// Unfiltered candidate, if any.
        nearest: Option<(PolyRef, f32)>,
    },
    /// Start's unfiltered nearest polygon fails the query filter.
    StartExcluded {
        /// Polygon flags.
        flags: u16,
        /// Polygon area.
        area: u8,
    },
    /// Goal's unfiltered nearest polygon fails the query filter.
    GoalExcluded {
        /// Polygon flags.
        flags: u16,
        /// Polygon area.
        area: u8,
    },
    /// Distinct weak components with no touched unloaded frontier.
    DisconnectedIslands {
        /// Start component.
        start: IslandId,
        /// Goal component.
        goal: IslandId,
    },
    /// An all-pass search reaches the goal; excluded polygons on its corridor explain the cut.
    FilterCut {
        /// Sorted excluded refs, flags and areas; at most 64 entries.
        blocking: Vec<(PolyRef, u16, u8)>,
    },
    /// Expanded polygons face known nonresident columns, or an endpoint's column is unloaded.
    UnloadedTiles {
        /// Sorted unique columns to consider loading (not proof that loading suffices).
        frontier: Vec<TileKey>,
    },
    /// A search could not allocate another node; no negative reachability conclusion follows.
    NodePoolExhausted {
        /// Query's configured node count.
        max_nodes: u16,
    },
    /// Shared work allowance ran out; no negative reachability conclusion follows.
    BudgetExhausted,
    /// Connectivity belongs to a different structural epoch; update it first.
    MeshChanged,
    /// Weakly connected endpoints have no directed route (for example a reverse one-way link).
    DirectedUnreachable,
}

/// A path or a typed reason, with provenance and bounded explanation details.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PathDiagnosis {
    /// Field contract version.
    pub schema_version: u32,
    /// Mesh epoch used for the diagnosis.
    pub epoch: u64,
    /// Topology mode used by this diagnosis.
    pub global_off_mesh_links: bool,
    /// `None` exactly when a complete filtered corridor reaches the goal.
    pub reason: Option<NoPathReason>,
    /// Complete corridor on success; empty on failure.
    pub path: Vec<PolyRef>,
    /// Whether the excluded-polygon explanation was capped at 64 entries.
    pub blocking_truncated: bool,
}

struct AllPass;
impl QueryFilter for AllPass {
    fn pass_filter(
        &self,
        _: PolyRef,
        _: &crate::detour::MeshTile,
        _: &crate::detour::Poly,
    ) -> bool {
        true
    }
    fn cost(
        &self,
        pa: &[f32; 3],
        pb: &[f32; 3],
        _: Option<(PolyRef, &crate::detour::MeshTile, &crate::detour::Poly)>,
        _: (PolyRef, &crate::detour::MeshTile, &crate::detour::Poly),
        _: Option<(PolyRef, &crate::detour::MeshTile, &crate::detour::Poly)>,
    ) -> f32 {
        crate::common::sqrtf(
            (pa[0] - pb[0]) * (pa[0] - pb[0])
                + (pa[1] - pb[1]) * (pa[1] - pb[1])
                + (pa[2] - pb[2]) * (pa[2] - pb[2]),
        )
    }
}

/// Diagnoses a path using one shared budget; never reports a partial search as a proof.
///
/// Checks: epoch, endpoint residency, filtered nearest endpoints with unfiltered fallback, endpoint
/// exclusion, cheap fully resident weak disconnection, filtered A*, all-pass A*,
/// then weak/directed disconnection.
/// Exhaustion takes precedence over topology; a touched unloaded frontier takes
/// precedence over disconnection. Endpoint selection uses the same search box as
/// nearest-poly; off-mesh failures have no candidate when that box is empty.
/// `FilterCut` lists the first 64 excluded refs in sorted order on the complete
/// all-pass corridor and sets `blocking_truncated` if more existed. Costs and
/// directionality remain the query's responsibility; labels alone never prove a route.
/// Filters must give consistent inclusion answers throughout this call.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_path(
    q: &mut NavMeshQuery<'_>,
    conn: &Connectivity,
    start: [f32; 3],
    goal: [f32; 3],
    half_extents: [f32; 3],
    filter: &dyn QueryFilter,
    budget: &mut Budget,
) -> Result<PathDiagnosis, Error> {
    let mut out = PathDiagnosis {
        schema_version: SCHEMA_VERSION,
        epoch: q.nav().epoch(),
        global_off_mesh_links: q.nav().global_off_mesh_links(),
        reason: None,
        path: Vec::new(),
        blocking_truncated: false,
    };
    let reason = diagnose_inner(q, conn, start, goal, half_extents, filter, budget, &mut out)?;
    out.reason = reason;
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn diagnose_inner(
    q: &mut NavMeshQuery<'_>,
    conn: &Connectivity,
    start: [f32; 3],
    goal: [f32; 3],
    ext: [f32; 3],
    filter: &dyn QueryFilter,
    budget: &mut Budget,
    out: &mut PathDiagnosis,
) -> Result<Option<NoPathReason>, Error> {
    if !start.iter().chain(&goal).chain(&ext).all(|v| v.is_finite()) || ext.iter().any(|v| *v < 0.0)
    {
        return Err(Error::InvalidParameter("diagnostic positions or extents"));
    }
    if !budget.spend(1) {
        return Ok(Some(NoPathReason::BudgetExhausted));
    }
    if !conn.is_current(q.nav()) {
        return Ok(Some(NoPathReason::MeshChanged));
    }
    let mut unloaded = Vec::new();
    for pos in [&start, &goal] {
        if let Some(key) = q.nav().unloaded_column_at(pos) {
            unloaded.push(key);
        }
    }
    unloaded.sort_unstable();
    unloaded.dedup();
    if !unloaded.is_empty() {
        return Ok(Some(NoPathReason::UnloadedTiles { frontier: unloaded }));
    }
    let mut endpoints = [(0, [0.0; 3]); 2];
    for (index, pos) in [start, goal].iter().enumerate() {
        let (r, snapped, _) = match q.find_nearest_poly_budgeted(pos, &ext, filter, budget)? {
            Budgeted::Complete(v) => v,
            Budgeted::Exhausted(_) => return Ok(Some(NoPathReason::BudgetExhausted)),
        };
        if r != 0 {
            endpoints[index] = (r, snapped);
            continue;
        }
        let (candidate, _, _) = match q.find_nearest_poly_budgeted(pos, &ext, &AllPass, budget)? {
            Budgeted::Complete(v) => v,
            Budgeted::Exhausted(_) => return Ok(Some(NoPathReason::BudgetExhausted)),
        };
        if candidate == 0 {
            return Ok(Some(if index == 0 {
                NoPathReason::StartOffMesh { nearest: None }
            } else {
                NoPathReason::GoalOffMesh { nearest: None }
            }));
        }
        let (_, poly) = q.nav().tile_and_poly_by_ref(candidate)?;
        return Ok(Some(if index == 0 {
            NoPathReason::StartExcluded {
                flags: poly.flags,
                area: poly.area(),
            }
        } else {
            NoPathReason::GoalExcluded {
                flags: poly.flags,
                area: poly.area(),
            }
        }));
    }
    let [(sr, sp), (gr, gp)] = endpoints;
    if !budget.spend(1) {
        return Ok(Some(NoPathReason::BudgetExhausted));
    }
    if !q.nav().has_unloaded() {
        let a = conn.island(sr).ok_or(Error::MeshChanged)?;
        let b = conn.island(gr).ok_or(Error::MeshChanged)?;
        if a != b {
            return Ok(Some(NoPathReason::DisconnectedIslands {
                start: a,
                goal: b,
            }));
        }
    }
    let mut path = Vec::new();
    let max_nodes = q.max_nodes();
    let st = match q.find_path_budgeted(
        sr,
        gr,
        &sp,
        &gp,
        filter,
        &mut path,
        usize::from(max_nodes),
        budget,
    )? {
        Budgeted::Complete(s) => s,
        Budgeted::Exhausted(_) => return Ok(Some(NoPathReason::BudgetExhausted)),
    };
    if path.last() == Some(&gr) && !st.has_detail(Status::PARTIAL_RESULT) {
        out.path = path;
        return Ok(None);
    }
    if st.has_detail(Status::OUT_OF_NODES) {
        return Ok(Some(NoPathReason::NodePoolExhausted { max_nodes }));
    }
    let Some(filtered_frontier) = q.diagnostic_frontier(budget) else {
        return Ok(Some(NoPathReason::BudgetExhausted));
    };
    let st = match q.find_path_budgeted(
        sr,
        gr,
        &sp,
        &gp,
        &AllPass,
        &mut path,
        usize::from(max_nodes),
        budget,
    )? {
        Budgeted::Complete(s) => s,
        Budgeted::Exhausted(_) => return Ok(Some(NoPathReason::BudgetExhausted)),
    };
    if st.has_detail(Status::OUT_OF_NODES) {
        return Ok(Some(NoPathReason::NodePoolExhausted { max_nodes }));
    }
    if path.last() == Some(&gr) && !st.has_detail(Status::PARTIAL_RESULT) {
        let mut blocking = Vec::new();
        for r in path {
            if !budget.spend(1) {
                return Ok(Some(NoPathReason::BudgetExhausted));
            }
            let (tile, poly) = q.nav().tile_and_poly_by_ref(r)?;
            if !filter.pass_filter(r, tile, poly) {
                blocking.push((r, poly.flags, poly.area()));
            }
        }
        if blocking.is_empty() {
            return Err(Error::InvalidParameter("inconsistent diagnostic filter"));
        }
        blocking.sort_unstable();
        blocking.dedup();
        out.blocking_truncated = blocking.len() > 64;
        blocking.truncate(64);
        return Ok(Some(NoPathReason::FilterCut { blocking }));
    }
    let Some(mut frontier) = q.diagnostic_frontier(budget) else {
        return Ok(Some(NoPathReason::BudgetExhausted));
    };
    frontier.extend(filtered_frontier);
    frontier.sort_unstable();
    frontier.dedup();
    if !frontier.is_empty() {
        return Ok(Some(NoPathReason::UnloadedTiles { frontier }));
    }
    let a = conn.island(sr).ok_or(Error::MeshChanged)?;
    let b = conn.island(gr).ok_or(Error::MeshChanged)?;
    Ok(Some(if a != b {
        NoPathReason::DisconnectedIslands { start: a, goal: b }
    } else {
        NoPathReason::DirectedUnreachable
    }))
}
