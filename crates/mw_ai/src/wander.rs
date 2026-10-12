//! The `AI_W` wander package: idle rolls and wander goals.
//!
//! Behavior follows `OpenMW`'s `AiWander` (GPL reference, fine in this
//! repo): each decision first rolls the eight idle chances; with no idle
//! picked and a non-zero distance, the actor walks to a random pathgrid node
//! within `distance` of its home (spawn) position, along A* from the node
//! nearest to it. `game_hour` (time-of-day gating in the original editor's
//! data) is kept but inert, as in `OpenMW`.

use std::f32::consts::{FRAC_1_SQRT_2, TAU};

use glam::Vec3;
use mw_world::convert::meters;
use sf_controller::PathNode;
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{AiPackage, AiWanderPackage, Creature, GameSetting, GameSettingValue, Npc};

use crate::pathgrid::PathGraph;

/// Vanilla `fIdleChanceMultiplier`.
pub const IDLE_CHANCE_MULTIPLIER: f32 = 0.75;
/// Vanilla `fMinWalkSpeed` / `fMaxWalkSpeed`, units per second.
pub const MIN_WALK_SPEED: f32 = 100.0;
pub const MAX_WALK_SPEED: f32 = 200.0;
/// Idle length when the caller has no clip length for a variant, seconds.
pub const DEFAULT_IDLE_SECONDS: f32 = 5.0;
/// Goals tried per decision before standing instead.
const GOAL_TRIES: usize = 4;
/// Blocked pathgrid edges one actor remembers (the oldest is forgotten).
pub const BLOCKED_EDGES: usize = 8;
/// How far (m) an actor may stand from where it failed to reach a start
/// node for that node to stay skipped as a start.
pub const START_RADIUS: f32 = 1.0;

/// A float game setting, or `fallback` when missing or not a number.
#[allow(clippy::cast_precision_loss)]
pub fn gmst_float(db: &RecordDb, id: &str, fallback: f32) -> f32 {
    match db.get::<GameSetting>(&RecordId::new(id)).map(|g| &g.value) {
        Some(GameSettingValue::Float(v)) => *v,
        Some(GameSettingValue::Integer(v)) => *v as f32,
        _ => fallback,
    }
}

/// Walk speed in m/s for a Speed attribute (`fMinWalkSpeed` at 0,
/// `fMaxWalkSpeed` at 100; encumbrance not applied).
pub fn walk_speed(speed_attribute: f32, min_units: f32, max_units: f32) -> f32 {
    meters(min_units + 0.01 * speed_attribute * (max_units - min_units))
}

/// One actor's `AI_W` package, in engine units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wander {
    /// Wander radius around home, meters. 0 stands in place.
    pub distance: f32,
    /// Game hours until the package ends; 0 never ends.
    pub duration_hours: f32,
    /// Authored time of day; inert (as in `OpenMW`).
    pub game_hour: u8,
    /// Chances (0..=100) of Idle2..Idle9.
    pub idle_chances: [u8; 8],
    /// Restart when the duration runs out instead of ending.
    pub repeat: bool,
}

impl Wander {
    pub fn from_package(p: &AiWanderPackage) -> Self {
        Self {
            distance: meters(f32::from(p.distance)),
            duration_hours: f32::from(p.duration),
            game_hour: p.game_hour,
            idle_chances: [
                p.idle2, p.idle3, p.idle4, p.idle5, p.idle6, p.idle7, p.idle8, p.idle9,
            ],
            repeat: p.reset != 0,
        }
    }

    /// The first wander package in a package list.
    pub fn first(packages: &[AiPackage]) -> Option<Self> {
        packages.iter().find_map(|p| match p {
            AiPackage::Wander(w) => Some(Self::from_package(w)),
            _ => None,
        })
    }

    pub fn of_npc(npc: &Npc) -> Option<Self> {
        Self::first(&npc.ai_packages)
    }

    pub fn of_creature(creature: &Creature) -> Option<Self> {
        Self::first(&creature.ai_packages)
    }
}

/// Rolls the idle chances (`OpenMW`'s rule): for each idle, draw
/// `r` in `0..100/mult`; it wins if `r < mult * chance` and `r` beats the
/// best draw so far (which starts at 0). Returns the group number (2..=9).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn roll_idle(chances: &[u8; 8], mult: f32, rng: &mut fastrand::Rng) -> Option<u8> {
    let range = (100.0 / mult.max(1e-3)) as u32;
    let mut best = 0;
    let mut pick = None;
    for (group, &chance) in (2u8..).zip(chances) {
        let threshold = (mult * f32::from(chance)) as u32;
        let r = rng.u32(0..range.max(1));
        if r < threshold && r > best {
            best = r;
            pick = Some(group);
        }
    }
    pick
}

/// What to do next.
#[derive(Clone, Debug, PartialEq)]
pub enum WanderStep {
    /// Walk this polyline. `via_grid` is false for the no-pathgrid fallback
    /// (one straight leg to a random point in the wander disc).
    Walk {
        nodes: Vec<PathNode>,
        via_grid: bool,
    },
    /// Stand for `seconds`; `variant` is the idle group (2..=9), 0 for the
    /// plain `Idle`.
    Idle { variant: u8, seconds: f32 },
}

/// Chooses wander steps for one actor.
#[derive(Clone, Debug)]
pub struct WanderPlanner {
    pub wander: Wander,
    /// Spawn position; goals stay within `wander.distance` of it.
    pub home: WorldPos,
    /// `fIdleChanceMultiplier`.
    pub idle_mult: f32,
    /// Seconds per idle group 2..=9 (the wiring fills them from clip lengths).
    pub idle_seconds: [f32; 8],
    /// Pathgrid edges (from, to node positions) this actor gave up on as
    /// too steep or walled: plans route around them (newest last, at most
    /// [`BLOCKED_EDGES`]).
    pub blocked: Vec<(WorldPos, WorldPos)>,
    /// First legs given up (where the actor stood, the node or off-grid
    /// point it headed for): from about there, pathgrid walks start at the
    /// next-nearest node and off-grid walks avoid that heading (at most
    /// [`BLOCKED_EDGES`], newest last).
    pub blocked_starts: Vec<(WorldPos, WorldPos)>,
}

impl WanderPlanner {
    pub fn new(wander: Wander, home: WorldPos) -> Self {
        Self {
            wander,
            home,
            idle_mult: IDLE_CHANCE_MULTIPLIER,
            idle_seconds: [DEFAULT_IDLE_SECONDS; 8],
            blocked: Vec::new(),
            blocked_starts: Vec::new(),
        }
    }

    fn stand(rng: &mut fastrand::Rng) -> WanderStep {
        WanderStep::Idle {
            variant: 0,
            seconds: 2.0 + 3.0 * rng.f32(),
        }
    }

    /// The next step for an actor at `pos`, using its cell's pathgrid if any.
    pub fn next(
        &self,
        graph: Option<&PathGraph>,
        pos: WorldPos,
        rng: &mut fastrand::Rng,
    ) -> WanderStep {
        if let Some(variant) = roll_idle(&self.wander.idle_chances, self.idle_mult, rng) {
            return WanderStep::Idle {
                variant,
                seconds: self.idle_seconds[usize::from(variant - 2)],
            };
        }
        if self.wander.distance <= 0.0 {
            return Self::stand(rng);
        }
        let graph = graph.filter(|g| !g.is_empty());
        let Some(graph) = graph else {
            return self.direct(pos, rng);
        };
        let allowed = graph.within(self.home, self.wander.distance);
        if allowed.is_empty() {
            return self.direct(pos, rng);
        }
        let skip = self.skipped_starts(graph, pos);
        let Some(start) = graph.nearest_excluding(pos, &skip) else {
            return Self::stand(rng);
        };
        for _ in 0..GOAL_TRIES {
            let goal = allowed[rng.usize(0..allowed.len())];
            if goal == start && allowed.len() > 1 {
                continue;
            }
            let Some(nodes) = self.route(graph, pos, start, goal, &skip) else {
                continue;
            };
            return WanderStep::Walk { nodes, via_grid: true };
        }
        Self::stand(rng)
    }

    /// The blocked edges as `graph` node pairs.
    fn blocked_in(&self, graph: &PathGraph) -> Vec<(usize, usize)> {
        self.blocked.iter().filter_map(|&(a, b)| Some((graph.node_at(a)?, graph.node_at(b)?))).collect()
    }

    /// Nodes this actor failed to reach from about `pos` (first legs).
    fn skipped_starts(&self, graph: &PathGraph, pos: WorldPos) -> Vec<usize> {
        self.blocked_starts
            .iter()
            .filter(|(at, _)| at.delta_f32(pos).length() < START_RADIUS)
            .filter_map(|&(_, n)| graph.node_at(n))
            .collect()
    }

    /// The walk from `pos` along the pathgrid from node `start` to `goal`,
    /// around the blocked edges. `skip`: nodes not to head for straight
    /// from `pos` (first legs given up from about here).
    fn route(&self, graph: &PathGraph, pos: WorldPos, start: usize, goal: usize, skip: &[usize]) -> Option<Vec<PathNode>> {
        let path = graph.path_avoiding(start, goal, &self.blocked_in(graph))?;
        let mut points: Vec<WorldPos> = path.iter().map(|&n| graph.position(n)).collect();
        // Already between the first two nodes: don't walk back to the first.
        if points.len() >= 2 && !skip.contains(&path[1]) && pos.delta_f32(points[1]).length() < points[0].delta_f32(points[1]).length() {
            points.remove(0);
        }
        Some(points.into_iter().map(PathNode::at).collect())
    }

    /// Remembers the pathgrid edge `from` → `to` (and its reverse) as
    /// unwalkable for this actor and plans a walk from `pos` to the node at
    /// `goal` around every edge it remembers, if one exists. `from` must be
    /// a node; otherwise this was the first leg (from where the actor
    /// stood to its nearest node, walkers5): that node is skipped as a
    /// start from about here and the walk starts at the next-nearest one.
    /// Off the pathgrid (a straight walk to a point, [`Self::direct`]), the
    /// heading is remembered and avoided by the next rolls from about here.
    pub fn replan_around(
        &mut self,
        graph: Option<&PathGraph>,
        (from, to): (WorldPos, WorldPos),
        pos: WorldPos,
        goal: WorldPos,
    ) -> Option<Vec<PathNode>> {
        let Some(graph) = graph.filter(|g| g.node_at(to).is_some()) else {
            // Off the grid there is no route to try: the actor idles first
            // (the original keeps it in place), then rolls away from here.
            remember(&mut self.blocked_starts, (pos, to));
            return None;
        };
        if graph.node_at(from).is_none() {
            remember(&mut self.blocked_starts, (pos, to));
            let skip = self.skipped_starts(graph, pos);
            let (start, goal) = (graph.nearest_excluding(pos, &skip)?, graph.node_at(goal)?);
            return self.route(graph, pos, start, goal, &skip).filter(|nodes| !nodes.is_empty());
        }
        for edge in [(from, to), (to, from)] {
            remember(&mut self.blocked, edge);
        }
        let (start, goal) = (graph.nearest(pos)?, graph.node_at(goal)?);
        // Starting at the blocked edge's far end would lead back over it.
        let start = if start == graph.node_at(to)? { graph.node_at(from)? } else { start };
        self.route(graph, pos, start, goal, &[]).filter(|nodes| !nodes.is_empty())
    }

    /// A random point in the wander disc around home (uniform by area), a
    /// scaffold for actors with no pathgrid node in reach (the original
    /// keeps those in place). Headings given up from about `pos` (within
    /// 45°) are re-rolled; standing when every try is blocked.
    fn direct(&self, pos: WorldPos, rng: &mut fastrand::Rng) -> WanderStep {
        let blocked: Vec<Vec3> = self
            .blocked_starts
            .iter()
            .filter(|(at, _)| at.delta_f32(pos).length() < START_RADIUS)
            .map(|(at, to)| at.delta_f32(*to).with_y(0.0).normalize_or_zero())
            .collect();
        for _ in 0..2 * GOAL_TRIES {
            let r = self.wander.distance * rng.f32().sqrt();
            let a = TAU * rng.f32();
            let point = self.home.offset(Vec3::new(r * a.cos(), 0.0, r * a.sin()));
            let heading = pos.delta_f32(point).with_y(0.0).normalize_or_zero();
            if blocked.iter().any(|b| b.dot(heading) > FRAC_1_SQRT_2) {
                continue;
            }
            return WanderStep::Walk {
                nodes: vec![PathNode::at(point)],
                via_grid: false,
            };
        }
        Self::stand(rng)
    }
}

/// Adds `item` to a memory of at most [`BLOCKED_EDGES`] (the oldest goes).
fn remember(list: &mut Vec<(WorldPos, WorldPos)>, item: (WorldPos, WorldPos)) {
    if !list.contains(&item) {
        if list.len() >= BLOCKED_EDGES {
            list.remove(0);
        }
        list.push(item);
    }
}
