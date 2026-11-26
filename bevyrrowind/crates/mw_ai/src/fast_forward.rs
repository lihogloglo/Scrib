//! Morrowind's wander fast-forward (`MA7a`): where a wanderer stands after a skipped span (rest,
//! wait, travel, training, jail).
//!
//! Ported from `OpenMW` (GPL-3.0, fine in this repo) `apps/openmw/mwmechanics/aiwander.cpp` at
//! commit `b8285cc95d19e080fe0532f9f5154b0c3f727735` (`bevyrrowind/docs/specs/ai-adapter.md`
//! "Fast-forward (rest, wait)"), with `pathgrid.cpp` (`PathgridGraph::isPointConnected`,
//! `getNeighbouringPoints`) at the same commit:
//! - [`allowed_positions`] is `fillAllowedPositions` (:865-922) with `addNonPathGridAllowedPoints`,
//!   `addPositionBetweenPathgridPoints` and `setCurrentPositionToClosestAllowedPosition`, in
//!   metres: the nodes within the wander distance (3D) of the spawn and in the strongly connected
//!   component of the node nearest the spawn; exactly one such node adds the spawn and one point
//!   along each of its edges; then the position closest to the spawn is removed (it is "where the
//!   actor already is": the roll never picks it).
//! - [`MorrowindFastForward`] (`sf_ai::FastForwardRules`, its body [`MorrowindFastForward::place`]) is
//!   `AiWander::fastForward` (:770-849): the front directive
//!   only, a `StayNear` with a radius above 0; one `rollDice` over the allowed positions; when an
//!   actor stands within 60 units of the pick, a random neighbour of the pathgrid node nearest the
//!   pick and points 5 %, 15 %, 30 % of the way toward it (cumulative steps of 5 %, 10 %, 15 %),
//!   each tested; a neighbour with all three occupied is dropped and another rolled; none free (or
//!   no neighbour) leaves the actor where it is. As in `OpenMW`, the nudge point is not reset
//!   between neighbours: the next neighbour's steps start where the last one's ended.
//!
//! Divergences (stated, not ported):
//! - `mRemainingDuration--` per call (:773): our `Limit::For` expires a wander by game minutes.
//! - `OpenMW` calls `fastForward` once per waited hour (`Actors::rest`, `waitdialog.cpp:254`): one
//!   span here is one roll (same distribution of where the actor ends up).
//! - `dest.z() += 60` and `adjustPosition` (:841-847, a ground snap): a logical position has no
//!   body; promotion snaps the body on the navmesh.
//! - `isPureWaterCreature` (:893): no record access here; the `is_water_only` seam says it, and
//!   such an actor is not moved. Unconscious and paralysed actors (`Actors::fastForwardAi`) are
//!   filtered by the caller (`MA7b`).
//! - The pathgrid is the one of the cell the actor stands in, as in `OpenMW`; an actor standing in
//!   another worldspace than its home is not moved (its home's positions would not be in that
//!   cell's grid).
//! - `AiTravel::fastForward` is not ported: tier 0 already walked a `GoTo` for the span (`AI10a`).

use std::sync::Arc;

use mw_world::convert::meters;
use sf_ai::{
    ActivityRef, ActorKey, DirectiveDone, DirectiveKind, DirectiveStack, FastForwardRules, LogicalCtx, LogicalState,
};
use sf_coords::WorldPos;
use sf_ids::GameInstant;
use sf_world::Location;

use crate::pathgrid::{CellKey, PathGraph, PathGrids};

/// `OpenMW`'s `minimumWanderDistance` (`destinationTolerance * 2`), Morrowind units.
// tunables: const (OpenMW constant, ported rule)
pub const MINIMUM_WANDER_DISTANCE_UNITS: f32 = 128.0;
/// `OpenMW`'s `isAnyActorInRange(.., 60)` radius in `fastForward`, Morrowind units.
// tunables: const (OpenMW constant, ported rule)
pub const OCCUPIED_RADIUS_UNITS: f32 = 60.0;
/// One nudge step toward a neighbour: `j * 5 * length / 100` (`OpenMW`'s `fastForward`).
// tunables: const (OpenMW constant, ported rule)
const NUDGE_STEP: f32 = 0.05;
/// Nudge steps per neighbour (`for j in 1..=3`).
// tunables: const (OpenMW constant, ported rule)
const NUDGE_STEPS: u8 = 3;

/// "Is any actor within `r` metres of this point" (the game feeds it in `MA7b`).
pub type Occupancy = Arc<dyn Fn(&Location, f32) -> bool + Send + Sync>;
/// "Is this actor a pure water creature" (`OpenMW`'s `isPureWaterCreature`).
pub type WaterOnly = Arc<dyn Fn(ActorKey) -> bool + Send + Sync>;

/// `OpenMW`'s wander positions a fast-forward picks from (module docs). Empty when the graph has
/// fewer than 2 nodes or `distance_m <= 0`.
pub fn allowed_positions(graph: &PathGraph, spawn: WorldPos, distance_m: f32) -> Vec<WorldPos> {
    if graph.len() < 2 || distance_m.is_nan() || distance_m <= 0.0 {
        return Vec::new();
    }
    let Some(closest) = graph.nearest(spawn) else {
        return Vec::new();
    };
    let components = graph.components();
    let allowed: Vec<usize> = graph
        .within(spawn, distance_m)
        .into_iter()
        .filter(|&n| components[n] == components[closest])
        .collect();
    let mut out: Vec<WorldPos> = allowed.iter().map(|&n| graph.position(n)).collect();
    if let [only] = allowed[..] {
        out.push(spawn);
        let along = (distance_m / 2.0).max(meters(MINIMUM_WANDER_DISTANCE_UNITS));
        let start = graph.position(only);
        for to in graph.neighbours(only) {
            let delta = start.delta_f32(graph.position(to));
            out.push(start.offset(delta.normalize_or_zero() * along.min(delta.length())));
        }
    }
    let nearest = out
        .iter()
        .map(|p| spawn.delta_f32(*p).length_squared())
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(i, _)| i);
    if let Some(i) = nearest {
        out.remove(i);
    }
    out
}

/// Morrowind's fast-forward rule (module docs): wanderers are placed on one of their
/// [`allowed_positions`] after a skipped span. Deterministic per actor and span end.
#[derive(Clone)]
pub struct MorrowindFastForward {
    grids: Arc<PathGrids>,
    occupied: Occupancy,
    is_water_only: WaterOnly,
    seed: u64,
}

impl MorrowindFastForward {
    pub fn new(grids: Arc<PathGrids>, occupied: Occupancy, is_water_only: WaterOnly, seed: u64) -> Self {
        Self { grids, occupied, is_water_only, seed }
    }

    /// The pathgrid cell of a location: the exterior square under it, or the interior named by its
    /// worldspace.
    pub fn cell_of(loc: &Location) -> CellKey {
        if loc.worldspace == mw_world::ids::exterior() {
            CellKey::exterior_at(loc.pos)
        } else {
            CellKey::interior(loc.worldspace.as_str())
        }
    }

    /// The rule ([`FastForwardRules::after_fast_forward`]): call once after `state` was stepped to
    /// the span's end (`state.at`), as `sf_ai::fast_forward` does. A placed actor is left a `StayNear` waiter at its new point at
    /// `state.at` (no route, progress 0, activity `Wait`), its `until` rebuilt by a `step` to the
    /// same instant. Anything but a wanderer is left as tier 0 stepped it.
    pub fn place(
        &self,
        state: &mut LogicalState,
        stack: &mut DirectiveStack,
        ctx: &LogicalCtx<'_>,
        events: &mut Vec<DirectiveDone>,
    ) {
        let Some(top) = stack.top() else {
            return;
        };
        let DirectiveKind::StayNear { home, radius } = &top.spec.kind else {
            return;
        };
        let (home, radius, top_id) = (home.clone(), *radius, top.id);
        let actor = stack.actor();
        if radius.is_nan()
            || radius <= 0.0
            || state.directive != Some(top_id)
            || state.pos.worldspace != home.worldspace
            || (self.is_water_only)(actor)
        {
            return;
        }
        let Some(graph) = self.grids.get(&Self::cell_of(&state.pos)) else {
            return;
        };
        let allowed = allowed_positions(graph, home.pos, radius);
        if allowed.is_empty() {
            return;
        }
        let mut rng = fastrand::Rng::with_seed(self.seed_for(actor, state.at.0));
        let Some(dest) = self.unoccupied(graph, &home, allowed[rng.usize(..allowed.len())], &mut rng) else {
            return;
        };
        state.pos = Location::new(home.worldspace, dest);
        state.route = None;
        state.progress_m = 0.0;
        state.follow = None;
        state.arrive = None;
        state.arrives = None;
        state.activity = Some(ActivityRef::Wait);
        let at = state.at;
        state.step(stack, at, ctx, events);
    }

    /// The RNG seed of one actor's skip ending at `end_ms`.
    #[expect(clippy::cast_possible_truncation, reason = "folding the key's halves into 64 bits")]
    fn seed_for(&self, actor: ActorKey, end_ms: u64) -> u64 {
        self.seed ^ (actor.0 as u64) ^ ((actor.0 >> 64) as u64) ^ end_ms
    }

    /// `pick`, or `OpenMW`'s nudge toward a neighbour when an actor stands there (module docs);
    /// `None`: nowhere free.
    fn unoccupied(&self, graph: &PathGraph, home: &Location, pick: WorldPos, rng: &mut fastrand::Rng) -> Option<WorldPos> {
        let r = meters(OCCUPIED_RADIUS_UNITS);
        let occupied = |p: WorldPos| (self.occupied)(&Location::new(home.worldspace.clone(), p), r);
        if !occupied(pick) {
            return Some(pick);
        }
        let mut points: Vec<WorldPos> =
            graph.nearest(pick).map(|n| graph.neighbours(n)).unwrap_or_default().into_iter().map(|n| graph.position(n)).collect();
        let mut dest = pick;
        while !points.is_empty() {
            let i = rng.usize(..points.len());
            let delta = dest.delta_f32(points[i]);
            let (dir, length) = (delta.normalize_or_zero(), delta.length());
            for j in 1..=NUDGE_STEPS {
                dest = dest.offset(dir * (f32::from(j) * NUDGE_STEP * length));
                if !occupied(dest) {
                    return Some(dest);
                }
            }
            points.remove(i);
        }
        None
    }
}

impl FastForwardRules for MorrowindFastForward {
    fn after_fast_forward(
        &self,
        state: &mut LogicalState,
        stack: &mut DirectiveStack,
        _span: (GameInstant, GameInstant),
        ctx: &LogicalCtx<'_>,
        events: &mut Vec<DirectiveDone>,
    ) {
        self.place(state, stack, ctx, events);
    }
}
