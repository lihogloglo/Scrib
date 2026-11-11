//! Path queries: goals, the sliced path job and its budget, results (nav.md "Interfaces",
//! "Routing"). The shape follows Detour's sliced A* (`initSlicedFindPath`,
//! `updateSlicedFindPath(maxIter)`, `finalizeSlicedFindPath`), with the search state carried by the
//! job instead of the query object so one backend serves many workers.

use std::any::Any;
use std::fmt;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_world::{Location, WorldspaceId};

use crate::{AgentClassId, NavError, PolyRef, QueryFilter};

/// Where a path should end.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Goal {
    /// The navmesh point nearest this location.
    Point(Location),
    /// Any polygon whose position is within `radius` meters of `at` (follow, escort).
    Near { at: Location, radius: f32 },
    /// The cheapest polygon at least `min_dist` meters from `from` (flee; Detour-style Dijkstra).
    Away { from: Location, min_dist: f32 },
    /// The cheapest to reach of several points (the nearest free slot).
    AnyOf(Vec<Location>),
}

impl Goal {
    /// Every location the goal names, for worldspace checks.
    pub fn locations(&self) -> impl Iterator<Item = &Location> {
        let (one, many): (Option<&Location>, &[Location]) = match self {
            Self::Point(l) | Self::Near { at: l, .. } | Self::Away { from: l, .. } => {
                (Some(l), &[])
            }
            Self::AnyOf(v) => (None, v),
        };
        one.into_iter().chain(many)
    }

    /// The query's worldspace must be the goal's: crossing spaces is the worldspace graph's job
    /// (NV11), not a navmesh query's.
    pub fn check_worldspace(&self, ws: &WorldspaceId) -> Result<(), NavError> {
        match self.locations().find(|l| &l.worldspace != ws) {
            Some(l) => Err(NavError::WrongWorldspace {
                query: ws.clone(),
                goal: l.worldspace.clone(),
            }),
            None => Ok(()),
        }
    }
}

/// Detour sample's half extents for snapping a location onto the mesh, meters.
pub const DEFAULT_EXTENT: Vec3 = Vec3::new(2.0, 4.0, 2.0);

/// One path request (the query service adds the request id, priority and tick, NV6).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PathQuery {
    pub class: AgentClassId,
    pub from: Location,
    pub goal: Goal,
    pub filter: QueryFilter,
    /// Half extents for snapping `from` and point goals onto the mesh.
    pub extent: Vec3,
}

impl PathQuery {
    pub fn new(class: AgentClassId, from: Location, goal: Goal, filter: QueryFilter) -> Self {
        Self {
            class,
            from,
            goal,
            filter,
            extent: DEFAULT_EXTENT,
        }
    }
}

/// A work allowance in expansions (Detour's `maxIter`: one polygon taken off the open list). One
/// budget can be shared by several jobs (a tick's allowance) or spent by one job's steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceBudget {
    limit: u32,
    used: u32,
}

impl SliceBudget {
    pub fn new(limit: u32) -> Self {
        Self { limit, used: 0 }
    }

    pub fn unlimited() -> Self {
        Self::new(u32::MAX)
    }

    /// Takes one unit; false (and nothing taken) when none is left.
    pub fn spend(&mut self) -> bool {
        if self.used < self.limit {
            self.used += 1;
            true
        } else {
            false
        }
    }

    /// Takes `n` units a backend already spent in one call (Detour's iteration count), at most
    /// what is left.
    pub fn charge(&mut self, n: u32) {
        self.used += n.min(self.remaining());
    }

    pub fn remaining(&self) -> u32 {
        self.limit - self.used
    }

    pub fn used(&self) -> u32 {
        self.used
    }
}

/// Whether the path reaches the goal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PathStatus {
    Complete,
    /// The goal is unreachable: the path ends at the visited polygon closest to it (Detour's
    /// `DT_PARTIAL_RESULT`; for [`Goal::Away`], the farthest from the threat).
    Partial,
}

/// A finished path. Not serde: it holds [`PolyRef`]s; the AI re-requests paths after a load.
#[derive(Clone, Debug, PartialEq)]
pub struct PathResult {
    pub status: PathStatus,
    /// The polygon corridor, start to end (Detour `findPath` output).
    pub polys: Vec<PolyRef>,
    /// Points to walk through: the snapped start, one point per later polygon, the snapped goal
    /// point last when the goal is a point. Backends may string-pull (Detour `findStraightPath`).
    pub waypoints: Vec<Location>,
    /// Summed `meters × area cost` along the corridor.
    pub cost: f32,
}

/// Where a sliced job stands after a step.
#[derive(Clone, Debug, PartialEq)]
pub enum JobState {
    /// Out of budget: step again.
    InProgress,
    Done(PathResult),
    Failed(NavError),
}

/// A sliced path search: the query plus the backend's search state between steps.
pub struct PathJob {
    query: PathQuery,
    iterations: u32,
    finished: bool,
    scratch: Option<Box<dyn Any + Send>>,
}

impl PathJob {
    pub fn new(query: PathQuery) -> Self {
        Self {
            query,
            iterations: 0,
            finished: false,
            scratch: None,
        }
    }

    pub fn query(&self) -> &PathQuery {
        &self.query
    }

    /// Expansions spent so far, over all steps.
    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    // Backend side.

    /// The backend's search state (open list, node pool), whatever type it chooses.
    pub fn scratch_mut(&mut self) -> &mut Option<Box<dyn Any + Send>> {
        &mut self.scratch
    }

    pub fn add_iterations(&mut self, n: u32) {
        self.iterations += n;
    }

    /// Ends the job: drops the search state; later steps fail with [`NavError::JobFinished`].
    pub fn finish(&mut self) {
        self.finished = true;
        self.scratch = None;
    }
}

impl fmt::Debug for PathJob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathJob")
            .field("query", &self.query)
            .field("iterations", &self.iterations)
            .field("finished", &self.finished)
            .field("scratch", &self.scratch.is_some())
            .finish()
    }
}

/// A walkability ray along the mesh surface (Detour `raycast`).
#[derive(Clone, Debug, PartialEq)]
pub struct NavRay {
    /// False when the ray reached the target.
    pub hit: bool,
    /// Fraction of the way to the target reached, `0..=1` (`1` when clear).
    pub t: f32,
    /// Where the ray stopped, on the mesh.
    pub end: Location,
    /// Polygons crossed, start first.
    pub polys: Vec<PolyRef>,
}
