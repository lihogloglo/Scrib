//! Agent simulation and local avoidance, translated from DetourCrowd at the
//! pinned upstream revision; see NOTICE. A crowd borrows the navmesh only during
//! each call, so tiles can be replaced between updates. Slot handles are reused
//! after removal, just as in upstream; retain them only while the agent is active.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::too_many_arguments
)]
use crate::common::sqrtf;
use crate::detour::math::*;
use crate::detour::{
    DefaultFilter, NavMesh, NavMeshQuery, PolyRef, QueryFilter, RefState, STRAIGHTPATH_END,
    STRAIGHTPATH_OFFMESH_CONNECTION, Status, StraightPathPoint, TileKey,
};
use crate::{Budget, Budgeted, Error, Limits, UpstreamFixes, Workspace};
use alloc::vec::Vec;
mod avoidance;
mod boundary;
mod orca;
// Public helper calls reset usage; internal `_in` calls compose within the
// manager's existing per-call budget.
mod corridor;
mod grid;
mod queue;
pub use avoidance::{ObstacleAvoidanceParams, ObstacleAvoidanceQuery};
pub use boundary::LocalBoundary;
pub use corridor::PathCorridor;
pub use grid::ProximityGrid;
pub use orca::{AvoidanceMode, OrcaAgent, OrcaParams, OrcaScratch, orca_velocity};
pub use queue::PathQueue;

/// Anticipate corners when steering.
pub const ANTICIPATE_TURNS: u8 = 1;
/// Plan collision-avoiding velocities.
pub const OBSTACLE_AVOIDANCE: u8 = 2;
/// Add neighbour separation to steering.
pub const SEPARATION: u8 = 4;
/// Shortcut visible corridor sections.
pub const OPTIMIZE_VIS: u8 = 8;
/// Periodically optimize corridor topology.
pub const OPTIMIZE_TOPO: u8 = 16;

/// Slot in a crowd's fixed agent pool, reused after removal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentId(pub usize);
/// Locomotion state, with upstream numeric values.
///
/// ```compile_fail,E0004
/// use navmesh_rs::crowd::AgentState;
/// fn code(state: AgentState) -> u8 {
///     match state {
///         AgentState::Invalid => 0,
///         AgentState::Walking => 1,
///         AgentState::Offmesh => 2,
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum AgentState {
    /// No valid position.
    Invalid = 0,
    /// Moving on the surface.
    Walking = 1,
    /// Animating across a connection.
    Offmesh = 2,
}
/// Move request state, with upstream numeric values.
///
/// ```compile_fail,E0004
/// use navmesh_rs::crowd::TargetState;
/// fn code(state: TargetState) -> u8 {
///     match state {
///         TargetState::None => 0,
///         TargetState::Failed => 1,
///         TargetState::Valid => 2,
///         TargetState::Requesting => 3,
///         TargetState::WaitingForQueue => 4,
///         TargetState::WaitingForPath => 5,
///         TargetState::Velocity => 6,
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum TargetState {
    /// No request.
    None = 0,
    /// Request failed.
    Failed = 1,
    /// Corridor loaded.
    Valid = 2,
    /// Quick search pending.
    Requesting = 3,
    /// Waiting for queue capacity; updates retry the request in upstream order.
    WaitingForQueue = 4,
    /// Sliced search pending.
    WaitingForPath = 5,
    /// Direct velocity control.
    Velocity = 6,
    /// The target polygon's tile is unloaded (not in recastnavigation). The agent
    /// follows the resident part of its corridor, whose end is moved to the point
    /// nearest the target on the last resident polygon, and replans when the tile
    /// is committed again; [`CrowdAgent::waiting_tile`] names the column.
    WaitingForTile = 7,
}
/// Physical dimensions and steering settings for an agent.
/// Construct with [`Self::default`] and assign the desired fields.
///
/// ```compile_fail,E0639
/// use navmesh_rs::crowd::AgentParams;
/// let params = AgentParams { radius: 0.5, ..AgentParams::default() };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct AgentParams {
    /// Positive radius, no larger than the crowd's maximum.
    pub radius: f32,
    /// Positive height.
    pub height: f32,
    /// Maximum acceleration per second.
    pub max_acceleration: f32,
    /// Positive maximum desired speed (zero is rejected).
    pub max_speed: f32,
    /// Neighbour and wall search range.
    pub collision_query_range: f32,
    /// Visibility optimization range.
    pub path_optimization_range: f32,
    /// Separation steering weight.
    pub separation_weight: f32,
    /// Combination of the update flag constants.
    pub update_flags: u8,
    /// Avoidance parameter slot (0..8).
    pub obstacle_avoidance_type: usize,
    /// Local velocity planner, gated by [`OBSTACLE_AVOIDANCE`].
    pub avoidance: AvoidanceMode,
    /// Filter slot (0..16).
    pub query_filter_type: usize,
}
impl Default for AgentParams {
    fn default() -> Self {
        Self {
            radius: 0.6,
            height: 2.,
            max_acceleration: 8.,
            max_speed: 3.5,
            collision_query_range: 7.2,
            path_optimization_range: 18.,
            separation_weight: 2.,
            update_flags: 31,
            obstacle_avoidance_type: 3,
            avoidance: AvoidanceMode::Sampled,
            query_filter_type: 0,
        }
    }
}
impl AgentParams {
    fn validate(&self, max_radius: f32) -> Result<(), Error> {
        for v in [
            self.radius,
            self.height,
            self.max_speed,
            self.collision_query_range,
            self.path_optimization_range,
        ] {
            positive(v)?;
        }
        for v in [self.max_acceleration, self.separation_weight] {
            nonnegative(v)?;
        }
        if self.radius > max_radius
            || self.obstacle_avoidance_type >= 8
            || self.query_filter_type >= 16
            || self.update_flags & !31 != 0
        {
            return Err(Error::InvalidParameter("agent parameters"));
        }
        if let AvoidanceMode::Orca(p) = self.avoidance {
            p.validate()?;
            for t in [p.time_horizon, p.time_horizon_obstacles] {
                if !(1.0e-5..=1.0e5).contains(&t) {
                    return Err(Error::InvalidParameter("ORCA time range"));
                }
            }
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
struct Animation {
    active: bool,
    init: V3,
    start: V3,
    end: V3,
    t: f32,
    tmax: f32,
}
/// Read-only state of an agent slot.
#[derive(Clone, Debug)]
pub struct CrowdAgent {
    /// Whether this slot is in use.
    pub active: bool,
    /// Locomotion state.
    pub state: AgentState,
    /// Move request state.
    pub target_state: TargetState,
    /// Current position.
    pub npos: V3,
    /// Actual velocity after acceleration limiting.
    pub vel: V3,
    /// Chosen avoidance velocity.
    pub nvel: V3,
    /// Desired steering velocity.
    pub dvel: V3,
    /// Desired speed.
    pub desired_speed: f32,
    /// Current path corridor.
    pub corridor: PathCorridor,
    /// Cached local walls.
    pub boundary: LocalBoundary,
    /// Steering corners.
    pub corners: Vec<StraightPathPoint>,
    /// Physical and steering configuration.
    pub params: AgentParams,
    /// Requested polygon (zero in velocity mode).
    pub target_ref: PolyRef,
    /// Requested position or velocity.
    pub target_pos: V3,
    /// Whether the full search returned a partial path.
    pub partial: bool,
    waiting_tile: Option<TileKey>,
    topology_time: f32,
    replan_time: f32,
    replan: bool,
    pathq: u32,
    neis: Vec<(usize, f32)>,
    nneis: usize,
    disp: V3,
    anim: Animation,
}
impl CrowdAgent {
    /// The neighbours kept by the last update (six for sampled mode, the configured
    /// cap for ORCA), nearest first, as
    /// the neighbour's agent slot and its squared planar distance.
    pub fn neighbours(&self) -> &[(usize, f32)] {
        &self.neis[..self.nneis]
    }
    /// The unloaded column holding the target polygon while the target state is
    /// [`TargetState::WaitingForTile`] (not in recastnavigation).
    pub fn waiting_tile(&self) -> Option<TileKey> {
        self.waiting_tile
            .filter(|_| self.target_state == TargetState::WaitingForTile)
    }
    // Preserve allocated storage, but no state belonging to a previous occupant.
    fn reset_slot(&mut self) {
        self.active = false;
        self.state = AgentState::Invalid;
        self.target_state = TargetState::None;
        self.npos = [0.; 3];
        self.vel = [0.; 3];
        self.nvel = [0.; 3];
        self.dvel = [0.; 3];
        self.desired_speed = 0.;
        self.corridor.path.clear();
        self.corridor.pos = [0.; 3];
        self.corridor.target = [0.; 3];
        self.boundary.reset();
        self.corners.clear();
        self.params = AgentParams::default();
        self.target_ref = 0;
        self.target_pos = [0.; 3];
        self.partial = false;
        self.topology_time = 0.;
        self.replan_time = 0.;
        self.replan = false;
        self.pathq = 0;
        self.neis.fill((0, 0.));
        self.nneis = 0;
        self.disp = [0.; 3];
        self.anim = Animation::default();
    }
}
/// Fixed-capacity single-threaded crowd manager.
pub struct Crowd {
    agents: Vec<CrowdAgent>,
    max_radius: f32,
    extents: V3,
    grid: ProximityGrid,
    avoidance: ObstacleAvoidanceQuery,
    avoidance_params: [ObstacleAvoidanceParams; 8],
    filters: [DefaultFilter; 16],
    pathq: PathQueue,
    active: Vec<usize>,
    velocity_samples: usize,
    fixes: UpstreamFixes,
}
impl Crowd {
    /// Allocates agents and all persistent crowd components under workspace limits.
    pub fn new(
        ws: &mut Workspace,
        nav: &NavMesh,
        max_agents: usize,
        max_radius: f32,
    ) -> Result<Self, Error> {
        ws.begin();
        positive(max_radius)?;
        if max_agents == 0 || max_agents > 16383 {
            return Err(Error::InvalidParameter("max_agents must be in 1..=16383"));
        }
        let grid = ProximityGrid::new_in(ws, max_agents * 4, max_radius * 3.)?;
        let avoidance = ObstacleAvoidanceQuery::new_in(ws, 6, 8)?;
        let pathq = PathQueue::new_in(ws, nav, 256, 4096)?;
        let mut agents = ws.vec_with_capacity(max_agents)?;
        for _ in 0..max_agents {
            agents.push(CrowdAgent {
                active: false,
                state: AgentState::Invalid,
                target_state: TargetState::None,
                npos: [0.; 3],
                vel: [0.; 3],
                nvel: [0.; 3],
                dvel: [0.; 3],
                desired_speed: 0.,
                corridor: PathCorridor::new_in(ws, 256)?,
                boundary: LocalBoundary::new_in(ws)?,
                corners: ws.vec_with_capacity(4)?,
                params: AgentParams::default(),
                target_ref: 0,
                target_pos: [0.; 3],
                partial: false,
                waiting_tile: None,
                topology_time: 0.,
                replan_time: 0.,
                replan: false,
                pathq: 0,
                neis: ws.vec(6, (0, 0.))?,
                nneis: 0,
                disp: [0.; 3],
                anim: Animation::default(),
            });
        }
        Ok(Self {
            agents,
            max_radius,
            extents: [max_radius * 2., max_radius * 1.5, max_radius * 2.],
            grid,
            avoidance,
            pathq,
            avoidance_params: [ObstacleAvoidanceParams::default(); 8],
            filters: core::array::from_fn(|_| DefaultFilter::default()),
            active: ws.vec_with_capacity(max_agents)?,
            velocity_samples: 0,
            fixes: UpstreamFixes::NONE,
        })
    }
    /// Selects the upstream defect corrections the crowd applies; the default is
    /// [`UpstreamFixes::NONE`]. The crowd uses [`UpstreamFixes::NEAREST_NEIGHBOURS`],
    /// [`UpstreamFixes::ADAPTIVE_PATTERN`] and, for its path queue,
    /// [`UpstreamFixes::SLICED_CROSS_SIDE`]. Set it before the first agent is
    /// given a target so one request is not planned under two settings.
    pub fn set_fixes(&mut self, fixes: UpstreamFixes) {
        self.fixes = fixes;
        self.pathq.set_fixes(fixes);
        self.avoidance.set_fixes(fixes);
    }
    /// The corrections the crowd applies.
    pub fn fixes(&self) -> UpstreamFixes {
        self.fixes
    }
    /// Maximum number of agent slots.
    pub fn max_agents(&self) -> usize {
        self.agents.len()
    }
    /// All slots, including inactive ones.
    pub fn agents(&self) -> &[CrowdAgent] {
        &self.agents
    }
    /// Reads an active slot.
    pub fn agent(&self, id: AgentId) -> Option<&CrowdAgent> {
        self.agents.get(id.0).filter(|a| a.active)
    }
    /// The active agents in slot order (`dtCrowd::getActiveAgents`, without the
    /// copy into a caller array).
    pub fn active_agents(&self) -> impl Iterator<Item = &CrowdAgent> {
        self.agents.iter().filter(|a| a.active)
    }
    /// Half extents of the box used to place agents on the navmesh
    /// (`dtCrowd::getQueryHalfExtents`).
    pub fn query_half_extents(&self) -> V3 {
        self.extents
    }
    /// The proximity grid the last update filled with the active agents
    /// (`dtCrowd::getGrid`).
    pub fn grid(&self) -> &ProximityGrid {
        &self.grid
    }
    /// The path queue that serves the agents' path requests (`dtCrowd::getPathQueue`).
    pub fn path_queue(&self) -> &PathQueue {
        &self.pathq
    }
    fn agent_mut(&mut self, id: AgentId) -> Result<&mut CrowdAgent, Error> {
        self.agents
            .get_mut(id.0)
            .filter(|a| a.active)
            .ok_or(Error::InvalidParameter("inactive agent handle"))
    }
    /// Number of sampled velocities in the last update.
    pub fn velocity_sample_count(&self) -> usize {
        self.velocity_samples
    }
    /// Sets one of eight avoidance presets.
    pub fn set_obstacle_avoidance_params(
        &mut self,
        index: usize,
        p: ObstacleAvoidanceParams,
    ) -> Result<(), Error> {
        p.validate()?;
        *self
            .avoidance_params
            .get_mut(index)
            .ok_or(Error::InvalidParameter("avoidance index"))? = p;
        Ok(())
    }
    /// Reads an avoidance preset.
    pub fn obstacle_avoidance_params(&self, index: usize) -> Option<&ObstacleAvoidanceParams> {
        self.avoidance_params.get(index)
    }
    /// Sets one of sixteen query filters. Pending searches read its current value.
    pub fn set_filter(&mut self, index: usize, f: DefaultFilter) -> Result<(), Error> {
        if f.area_cost.iter().any(|v| !v.is_finite() || *v < 0.) {
            return Err(Error::InvalidParameter("filter costs"));
        }
        *self
            .filters
            .get_mut(index)
            .ok_or(Error::InvalidParameter("filter index"))? = f;
        Ok(())
    }
    /// Reads a query filter.
    pub fn filter(&self, index: usize) -> Option<&DefaultFilter> {
        self.filters.get(index)
    }
    /// Places an agent on the nearest polygon, or marks it invalid if none is near.
    pub fn add_agent(
        &mut self,
        ws: &mut Workspace,
        nav: &NavMesh,
        pos: V3,
        p: AgentParams,
    ) -> Result<AgentId, Error> {
        ws.begin();
        finite(pos)?;
        p.validate(self.max_radius)?;
        ws.work(1)?;
        ws.charge_bytes::<u8>(512 * 128 + 16384)?;
        let idx = self
            .agents
            .iter()
            .position(|a| !a.active)
            .ok_or(Error::BufferTooSmall)?;
        let q = NavMeshQuery::new(nav, 512)?;
        let (r, nearest, _) = q
            .find_nearest_poly(&pos, &self.extents, &self.filters[p.query_filter_type])
            .unwrap_or((0, pos, false));
        let nearest = if r == 0 { pos } else { nearest };
        let a = &mut self.agents[idx];
        a.reset_slot();
        a.params = p;
        a.corridor.reset(r, nearest)?;
        a.npos = nearest;
        a.state = if r != 0 {
            AgentState::Walking
        } else {
            AgentState::Invalid
        };
        a.target_state = TargetState::None;
        a.active = true;
        Ok(AgentId(idx))
    }
    /// Releases a slot for reuse, clearing its animation, target and cached state.
    pub fn remove_agent(&mut self, id: AgentId) -> Result<(), Error> {
        self.agent_mut(id)?.reset_slot();
        Ok(())
    }
    /// Updates physical and steering settings for an active slot.
    pub fn update_agent_parameters(&mut self, id: AgentId, p: AgentParams) -> Result<(), Error> {
        p.validate(self.max_radius)?;
        self.agent_mut(id)?.params = p;
        Ok(())
    }
    /// Requests a new target polygon and position; processed on the next update.
    /// Returns [`Error::InvalidRef`] for zero, stale or nonexistent refs in `nav`.
    pub fn request_move_target(
        &mut self,
        nav: &NavMesh,
        id: AgentId,
        r: PolyRef,
        pos: V3,
    ) -> Result<(), Error> {
        finite(pos)?;
        // A polygon of an unloaded tile is accepted: the agent stops where it is
        // and waits for the tile (`TargetState::WaitingForTile`).
        let waiting = match nav.ref_state(r) {
            RefState::Valid => None,
            RefState::Unloaded(k) => Some(k),
            _ => return Err(Error::InvalidRef),
        };
        let a = self.agent_mut(id)?;
        a.target_ref = r;
        a.target_pos = pos;
        a.pathq = 0;
        a.replan = false;
        a.waiting_tile = waiting;
        if waiting.is_some() {
            a.target_state = TargetState::WaitingForTile;
            if a.state == AgentState::Walking {
                a.corridor.reset(a.corridor.first_poly(), a.npos)?;
            }
        } else {
            a.target_state = TargetState::Requesting;
        }
        Ok(())
    }
    /// Switches to direct velocity control.
    pub fn request_move_velocity(&mut self, id: AgentId, vel: V3) -> Result<(), Error> {
        finite(vel)?;
        let a = self.agent_mut(id)?;
        a.target_ref = 0;
        a.target_pos = vel;
        a.pathq = 0;
        a.replan = false;
        a.target_state = TargetState::Velocity;
        Ok(())
    }
    /// Clears a movement request and its desired velocity.
    pub fn reset_move_target(&mut self, id: AgentId) -> Result<(), Error> {
        let a = self.agent_mut(id)?;
        a.target_ref = 0;
        a.target_pos = [0.; 3];
        a.dvel = [0.; 3];
        a.pathq = 0;
        a.replan = false;
        a.target_state = TargetState::None;
        Ok(())
    }
}
fn finite(v: V3) -> Result<(), Error> {
    if v.iter().all(|v| v.is_finite()) {
        Ok(())
    } else {
        Err(Error::InvalidInput("non-finite crowd vector"))
    }
}
fn positive(v: f32) -> Result<(), Error> {
    if v.is_finite() && v > 0. {
        Ok(())
    } else {
        Err(Error::InvalidParameter("positive crowd scalar"))
    }
}
fn nonnegative(v: f32) -> Result<(), Error> {
    if v.is_finite() && v >= 0. {
        Ok(())
    } else {
        Err(Error::InvalidParameter("nonnegative crowd scalar"))
    }
}
fn vdist_2d_sqr(a: &[f32], b: &[f32]) -> f32 {
    sqr(a[0] - b[0]) + sqr(a[2] - b[2])
}
fn vdist_2d(a: &[f32], b: &[f32]) -> f32 {
    sqrtf(vdist_2d_sqr(a, b))
}
fn vlen(a: &[f32]) -> f32 {
    sqrtf(vlen_sqr(a))
}
// Upstream appends equal-to-last entries, but inserts other ties before peers.
fn insert_pos<T>(items: &[T], value: f32, key: impl Fn(&T) -> f32) -> usize {
    if items.last().is_none_or(|v| value >= key(v)) {
        items.len()
    } else {
        items
            .iter()
            .position(|v| value <= key(v))
            .unwrap_or(items.len())
    }
}
mod update;
