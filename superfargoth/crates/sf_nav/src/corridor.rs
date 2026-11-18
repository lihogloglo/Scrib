//! Local path maintenance (`docs/specs/nav.md`, NV8): [`PathCorridor`] tracks an agent's
//! constrained position and polygon sequence along a [`PathResult`], string-pulls steering
//! corners, shortcuts visible doglegs and topological detours, and repairs the start locally
//! after a push or tile/polygon change (Detour `dtPathCorridor`).
//!
//! The visibility shortcut and the shortcut merge follow recastnavigation
//! `DetourCrowd/Source/DetourPathCorridor.cpp` (zlib) at commit
//! `9f4ce64458dfae86e1239c525ddc219c4e9e06f1`: `dtPathCorridor::optimizePathVisibility` and
//! `dtMergeCorridorStartShortcut`.

use glam::{Vec2, Vec3};
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::{
    AgentClassId, DEFAULT_EXTENT, NavBackend, NavError, NavPoint, PathResult, PathStatus, PolyRef,
    QueryFilter,
};

// tunables: const (Detour dtPathCorridor findCorners 1 cm pruning epsilon)
pub const CORNER_PRUNE_DIST_M: f32 = 0.01;

// tunables: const (corridor polygon cap matching RecastConfig::max_path)
const MAX_CORRIDOR_POLYS: usize = 2048;

// tunables: const (Detour optimizePathVisibility 1 cm ray overshoot past the next corner)
const VISIBILITY_OVERSHOOT_M: f32 = 0.01;

// tunables: const (Detour optimizePathVisibility `t > 0.99` clear-ray acceptance)
const VISIBILITY_CLEAR_T: f32 = 0.99;

/// Corridor maintenance parameters (bound from `ai.nav.corridor.*` tunables in `sf_nav_bevy`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorridorConfig {
    /// Maximum string-pulled steering corners returned per step.
    pub max_corners: usize,
    /// Polygons ahead of the agent checked for validity each step.
    pub lookahead_polys: usize,
    /// Maximum XZ raycast distance for visibility shortcutting, meters.
    pub optimize_range_m: f32,
    /// Period between bounded topology optimization searches, seconds.
    pub topology_period_s: f32,
    /// Planar XZ drift above which the corridor snaps and repairs locally, meters.
    pub repair_drift_m: f32,
    /// Maximum node expansions for local repair and topology optimization searches.
    pub repair_nodes: u32,
}

impl Default for CorridorConfig {
    fn default() -> Self {
        Self {
            // tunables: const (bound from ai.nav.corridor.max_corners in sf_nav_bevy)
            max_corners: 4,
            // tunables: const (bound from ai.nav.corridor.lookahead_polys in sf_nav_bevy)
            lookahead_polys: 10,
            // tunables: const (bound from ai.nav.corridor.optimize_range_m in sf_nav_bevy)
            optimize_range_m: 18.0,
            // tunables: const (bound from ai.nav.corridor.topology_period_s in sf_nav_bevy)
            topology_period_s: 0.5,
            // tunables: const (bound from ai.nav.corridor.repair_drift_m in sf_nav_bevy)
            repair_drift_m: 0.75,
            // tunables: const (bound from ai.nav.corridor.repair_nodes in sf_nav_bevy)
            repair_nodes: 32,
        }
    }
}

/// Output of one [`PathCorridor::step`].
#[derive(Clone, Debug, PartialEq)]
pub struct CorridorStep {
    /// Constrained position on the first corridor polygon.
    pub pos: Location,
    /// Constrained target on the last corridor polygon.
    pub target: Location,
    /// String-pulled steering corners ahead of [`Self::pos`].
    pub corners: Vec<Location>,
    /// True when the corridor is complete, valid, and the last corner reaches [`Self::target`].
    pub stop_at_end: bool,
    /// True when the corridor was trimmed or drifted beyond local repair and needs a new path.
    pub needs_replan: bool,
    /// True when a local push/start repair search spliced a new start onto the corridor this step.
    pub repaired: bool,
}

/// A polygon corridor with constrained position, target, and string-pulled corners (Detour
/// `dtPathCorridor`).
#[derive(Clone, Debug)]
pub struct PathCorridor {
    class: AgentClassId,
    filter: QueryFilter,
    extent: Vec3,
    pos: Location,
    target: Location,
    goal: Location,
    polys: Vec<PolyRef>,
    corners: Vec<Location>,
    stop_at_end: bool,
    status: PathStatus,
    needs_replan: bool,
    topology_timer: f32,
}

impl PathCorridor {
    /// Creates an empty corridor for `class` and `filter`. Call [`Self::load`] with a
    /// [`PathResult`] before stepping.
    pub fn new(class: AgentClassId, filter: QueryFilter) -> Self {
        let origin = Location::new(WorldspaceId::new(""), WorldPos::ORIGIN);
        Self {
            class,
            filter,
            extent: DEFAULT_EXTENT,
            pos: origin.clone(),
            target: origin.clone(),
            goal: origin,
            polys: Vec::new(),
            corners: Vec::new(),
            stop_at_end: false,
            status: PathStatus::Partial,
            needs_replan: true,
            topology_timer: 0.0,
        }
    }

    /// Creates a corridor initialized from `result` toward `goal`.
    pub fn from_result(
        class: AgentClassId,
        filter: QueryFilter,
        goal: Location,
        result: &PathResult,
    ) -> Result<Self, NavError> {
        let mut c = Self::new(class, filter);
        c.load(goal, result)?;
        Ok(c)
    }

    /// Sets the `nearest` search half-extents used when snapping after a push.
    #[must_use]
    pub fn with_extent(mut self, extent: Vec3) -> Self {
        self.extent = extent;
        self
    }

    /// Updates the query filter used by surface moves, raycasts, and local repair.
    pub fn set_filter(&mut self, filter: QueryFilter) {
        self.filter = filter;
    }

    /// Marks the corridor as needing a full replan from the query service.
    pub fn mark_needs_replan(&mut self) {
        self.needs_replan = true;
        self.stop_at_end = false;
    }

    /// Loads a finished [`PathResult`] for `goal` into this corridor.
    pub fn load(&mut self, goal: Location, result: &PathResult) -> Result<(), NavError> {
        let Some(first) = result.waypoints.first().cloned() else {
            return Err(NavError::Backend("path result has no waypoints".into()));
        };
        let Some(last) = result.waypoints.last().cloned() else {
            return Err(NavError::Backend("path result has no waypoints".into()));
        };
        if result.polys.is_empty() {
            return Err(NavError::Backend("path result has no polygons".into()));
        }
        self.pos = first;
        self.target = last;
        self.goal = goal;
        self.polys.clear();
        self.polys
            .extend(result.polys.iter().copied().take(MAX_CORRIDOR_POLYS));
        self.corners.clear();
        self.status = result.status;
        self.needs_replan = false;
        self.stop_at_end = false;
        self.topology_timer = 0.0;
        Ok(())
    }

    pub fn class(&self) -> AgentClassId {
        self.class
    }

    pub fn filter(&self) -> &QueryFilter {
        &self.filter
    }

    pub fn extent(&self) -> Vec3 {
        self.extent
    }

    pub fn pos(&self) -> &Location {
        &self.pos
    }

    pub fn target(&self) -> &Location {
        &self.target
    }

    pub fn goal(&self) -> &Location {
        &self.goal
    }

    pub fn polys(&self) -> &[PolyRef] {
        &self.polys
    }

    pub fn first_poly(&self) -> Option<PolyRef> {
        self.polys.first().copied()
    }

    pub fn last_poly(&self) -> Option<PolyRef> {
        self.polys.last().copied()
    }

    pub fn corners(&self) -> &[Location] {
        &self.corners
    }

    pub fn stop_at_end(&self) -> bool {
        self.stop_at_end
    }

    pub fn status(&self) -> PathStatus {
        self.status
    }

    pub fn needs_replan(&self) -> bool {
        self.needs_replan
    }

    /// Advances the corridor to `mover_pos` over `dt` seconds, repairing after a push, validating
    /// lookahead polygons, optimizing visibility/topology, and refreshing corners.
    pub fn step(
        &mut self,
        backend: &dyn NavBackend,
        mover_pos: Location,
        dt: f32,
        cfg: &CorridorConfig,
    ) -> Result<CorridorStep, NavError> {
        // 1. Worldspace mismatch or empty corridor requires a replan.
        if mover_pos.worldspace != self.pos.worldspace || self.polys.is_empty() {
            self.needs_replan = true;
            self.corners.clear();
            self.stop_at_end = false;
            return Ok(CorridorStep {
                pos: self.pos.clone(),
                target: self.target.clone(),
                corners: Vec::new(),
                stop_at_end: false,
                needs_replan: true,
                repaired: false,
            });
        }

        let orig_last = *self.polys.last().expect("checked non-empty");

        // 2. Move along surface when the start polygon is valid and the step is within drift.
        let first_valid = backend.is_valid_poly(self.polys[0], &self.filter);
        let pre_drift = xz_dist(&self.pos, &mover_pos);
        if first_valid && pre_drift <= cfg.repair_drift_m {
            let from = NavPoint {
                loc: self.pos.clone(),
                poly: self.polys[0],
            };
            if let Ok(mv) = backend.move_along_surface(&from, &mover_pos, &self.filter) {
                merge_start_moved(&mut self.polys, &mv.visited, MAX_CORRIDOR_POLYS);
                self.pos = mv.pos;
            }
        }

        // 3. Repair after a push or invalid start polygon.
        let first_valid = self
            .polys
            .first()
            .is_some_and(|&p| backend.is_valid_poly(p, &self.filter));
        let drift = xz_dist(&self.pos, &mover_pos);
        let mut repaired = false;
        if !first_valid || drift > cfg.repair_drift_m {
            repaired = self.repair_start(backend, &mover_pos, orig_last, cfg.repair_nodes);
        }

        // 4. Lookahead validity check.
        let lookahead = cfg.lookahead_polys.max(1);
        if !self
            .polys
            .iter()
            .take(lookahead)
            .all(|&p| backend.is_valid_poly(p, &self.filter))
        {
            self.trim_to_valid_prefix(backend);
            self.needs_replan = true;
        }

        if self.polys.is_empty() {
            self.needs_replan = true;
            self.corners.clear();
            self.stop_at_end = false;
            return Ok(CorridorStep {
                pos: self.pos.clone(),
                target: self.target.clone(),
                corners: Vec::new(),
                stop_at_end: false,
                needs_replan: true,
                repaired,
            });
        }

        // 5a. Periodic topology optimization.
        self.optimize_topology(backend, dt, cfg);

        // 5b & 6. Find corners, try visibility shortcutting toward the 2nd corner, then refresh.
        let corners = self.optimize_visibility_and_corners(backend, cfg)?;

        let end_corner = corners.last().unwrap_or(&self.pos);
        let reaches_target = xz_dist(end_corner, &self.target) <= CORNER_PRUNE_DIST_M;
        self.stop_at_end =
            self.status == PathStatus::Complete && !self.needs_replan && reaches_target;
        self.corners.clone_from(&corners);

        Ok(CorridorStep {
            pos: self.pos.clone(),
            target: self.target.clone(),
            corners,
            stop_at_end: self.stop_at_end,
            needs_replan: self.needs_replan,
            repaired,
        })
    }

    fn optimize_topology(&mut self, backend: &dyn NavBackend, dt: f32, cfg: &CorridorConfig) {
        if dt > 0.0 && dt.is_finite() {
            self.topology_timer += dt;
        }
        if self.polys.len() >= 3
            && cfg.topology_period_s > 0.0
            && self.topology_timer >= cfg.topology_period_s
        {
            self.topology_timer = 0.0;
            let start = NavPoint {
                loc: self.pos.clone(),
                poly: self.polys[0],
            };
            if let Ok(res) = backend.repair_corridor(
                &start,
                &self.target,
                &self.polys,
                &self.filter,
                cfg.repair_nodes,
            ) && res.iter().any(|r| self.polys.contains(r))
            {
                merge_shortcut(&mut self.polys, &res, MAX_CORRIDOR_POLYS);
            }
        }
    }

    fn optimize_visibility_and_corners(
        &mut self,
        backend: &dyn NavBackend,
        cfg: &CorridorConfig,
    ) -> Result<Vec<Location>, NavError> {
        let max_corners = cfg.max_corners.max(1);
        let mut corners = backend.corridor_corners(
            &self.pos,
            &self.target,
            &self.polys,
            max_corners.max(2),
        )?;
        if cfg.optimize_range_m > 0.0 && corners.len() >= 2 {
            let next = &corners[1];
            let delta = self.pos.pos.delta_f32(next.pos);
            let dist_xz = Vec2::new(delta.x, delta.z).length();
            if dist_xz >= CORNER_PRUNE_DIST_M {
                // Detour overshoots: a corner nearer than the range is pushed out to the full
                // range (`goal = pos + delta * range / min(dist + 0.01, range)`), which shortcuts
                // open ground past the corner and across tiles. A corner beyond the range keeps
                // the ray at the range (Detour's own "clamp the ray to max distance" intent; its
                // code would cast all the way to that corner).
                let denom = (dist_xz + VISIBILITY_OVERSHOOT_M)
                    .min(cfg.optimize_range_m)
                    .max(dist_xz);
                let scale = cfg.optimize_range_m / denom;
                let goal_loc = Location::new(
                    self.pos.worldspace.clone(),
                    self.pos.pos.offset(delta * scale),
                );
                let start = NavPoint {
                    loc: self.pos.clone(),
                    poly: self.polys[0],
                };
                // Detour accepts a ray with `t > 0.99`. An overshot ray stopped by a wall or the
                // mesh edge after it passed the corner (`t * scale >= 1`) is accepted too, so the
                // visible-corner shortcut of an unscaled ray is never lost.
                if let Ok(ray) = backend.raycast(&start, &goal_loc, &self.filter)
                    && ray.polys.len() > 1
                    && (ray.t > VISIBILITY_CLEAR_T || ray.t * scale >= 1.0)
                {
                    merge_shortcut(&mut self.polys, &ray.polys, MAX_CORRIDOR_POLYS);
                    corners = backend.corridor_corners(
                        &self.pos,
                        &self.target,
                        &self.polys,
                        max_corners,
                    )?;
                }
            }
        }
        if corners.len() > max_corners {
            corners.truncate(max_corners);
        }
        Ok(corners)
    }

    /// Snaps `mover_pos` onto the mesh and runs a bounded local search back to the corridor.
    /// Returns `true` when the local repair reconnected to the corridor.
    fn repair_start(
        &mut self,
        backend: &dyn NavBackend,
        mover_pos: &Location,
        orig_last: PolyRef,
        repair_nodes: u32,
    ) -> bool {
        let Some(snapped) = backend.nearest(mover_pos, self.class, self.extent, &self.filter)
        else {
            self.trim_to_valid_prefix(backend);
            self.needs_replan = true;
            return false;
        };
        self.pos = snapped.loc.clone();

        let valid_corridor: Vec<PolyRef> = self
            .polys
            .iter()
            .copied()
            .filter(|&p| backend.is_valid_poly(p, &self.filter))
            .collect();
        if valid_corridor.is_empty() {
            self.polys.clear();
            self.polys.push(snapped.poly);
            if let Ok(t) = backend.closest_point_on_poly(snapped.poly, &self.target) {
                self.target = t;
            }
            self.needs_replan = true;
            return false;
        }

        match backend.repair_corridor(
            &snapped,
            &self.target,
            &valid_corridor,
            &self.filter,
            repair_nodes,
        ) {
            Ok(res) if !res.is_empty() => {
                // The repair reached the corridor only if `res` ends on a corridor polygon and
                // every polygon of `res` passes the filter.
                let reached_corridor = res.last().is_some_and(|r| valid_corridor.contains(r))
                    && res.iter().all(|&p| backend.is_valid_poly(p, &self.filter));
                if reached_corridor {
                    // `res` starts on the agent's snapped polygon, so a furthest common polygon
                    // at `res[0]` means the agent is back on the corridor there: restart the
                    // corridor at it (unlike `merge_shortcut`, which then keeps the path).
                    let mut merged = valid_corridor;
                    if let Some((i, j)) = furthest_common(&merged, &res) {
                        splice_front(&mut merged, i, &res[..j], MAX_CORRIDOR_POLYS);
                    }
                    self.polys = merged;
                    if self.polys.last() == Some(&orig_last) {
                        true
                    } else {
                        if let Some(&last) = self.polys.last()
                            && let Ok(t) = backend.closest_point_on_poly(last, &self.target)
                        {
                            self.target = t;
                        }
                        self.needs_replan = true;
                        false
                    }
                } else {
                    self.polys = res
                        .into_iter()
                        .take_while(|&p| backend.is_valid_poly(p, &self.filter))
                        .collect();
                    if self.polys.is_empty() {
                        self.polys.push(snapped.poly);
                    }
                    if let Some(&last) = self.polys.last()
                        && let Ok(t) = backend.closest_point_on_poly(last, &self.target)
                    {
                        self.target = t;
                    }
                    self.needs_replan = true;
                    false
                }
            }
            _ => {
                self.polys.clear();
                self.polys.push(snapped.poly);
                if let Ok(t) = backend.closest_point_on_poly(snapped.poly, &self.target) {
                    self.target = t;
                }
                self.needs_replan = true;
                false
            }
        }
    }

    fn trim_to_valid_prefix(&mut self, backend: &dyn NavBackend) {
        let valid_len = self
            .polys
            .iter()
            .take_while(|&&p| backend.is_valid_poly(p, &self.filter))
            .count();
        self.polys.truncate(valid_len);
        if let Some(&last) = self.polys.last()
            && let Ok(t) = backend.closest_point_on_poly(last, &self.target)
        {
            self.target = t;
        }
    }
}

/// Planar XZ distance between two locations in the same worldspace.
pub(crate) fn xz_dist(a: &Location, b: &Location) -> f32 {
    let d = a.pos.delta_f32(b.pos);
    Vec2::new(d.x, d.z).length()
}

/// Detour `dtMergeCorridorStartMoved` (`mode == 0` in `navmesh-rs`): merges `visited` (from
/// `move_along_surface`) into the front of `path`.
pub fn merge_start_moved(path: &mut Vec<PolyRef>, visited: &[PolyRef], max: usize) {
    let mut common = None;
    for i in (0..path.len()).rev() {
        for j in (0..visited.len()).rev() {
            if path[i] == visited[j] {
                common = Some((i, j));
            }
        }
        if common.is_some() {
            break;
        }
    }
    let Some((i, j)) = common else { return };
    let req = (visited.len() - j).min(max);
    if req == 0 {
        return;
    }
    let orig = i + 1;
    let size = (path.len() - orig).min(max - req);
    path.resize(max, PolyRef(0));
    path.copy_within(orig..orig + size, req);
    for k in 0..req {
        path[k] = visited[visited.len() - 1 - k];
    }
    path.truncate(req + size);
}

/// Detour `dtMergeCorridorStartShortcut` (`mode == 2` in `navmesh-rs`): merges a forward shortcut
/// path `visited` (from `raycast` or `repair_corridor`, starting on `path[0]`) into the front of
/// `path`. Leaves `path` unchanged when they share no polygon or when the furthest common polygon
/// is `visited[0]` (Detour `req <= 0`).
pub fn merge_shortcut(path: &mut Vec<PolyRef>, visited: &[PolyRef], max: usize) {
    let Some((i, j)) = furthest_common(path, visited) else {
        return;
    };
    if j == 0 {
        return;
    }
    splice_front(path, i, &visited[..j], max);
}

/// The furthest polygon of `path` also in `visited`, as `(path index, first visited index)` (the
/// search of Detour's `dtMergeCorridor*` functions).
fn furthest_common(path: &[PolyRef], visited: &[PolyRef]) -> Option<(usize, usize)> {
    (0..path.len())
        .rev()
        .find_map(|i| visited.iter().position(|&v| v == path[i]).map(|j| (i, j)))
}

/// Replaces `path[..orig]` with `prefix`, keeping at most `max` polygons.
fn splice_front(path: &mut Vec<PolyRef>, orig: usize, prefix: &[PolyRef], max: usize) {
    let req = prefix.len().min(max);
    let size = (path.len() - orig).min(max - req);
    path.resize(max.max(path.len()), PolyRef(0));
    path.copy_within(orig..orig + size, req);
    path[..req].copy_from_slice(&prefix[..req]);
    path.truncate(req + size);
}
