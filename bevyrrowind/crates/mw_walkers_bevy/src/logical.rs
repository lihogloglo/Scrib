//! The game's tier-0 world (`AI10c2`; `ai.md` "Tiers and budget"): the two envs `sf_ai_bevy`'s
//! tier-0 shell runs `sf_ai`'s cores against. Plain types over borrowed data and closures (no
//! `Query`, no `Res`), so the systems build them from their params each tick and the tests by
//! hand. Neither keeps state across ticks.
//!
//! - [`WorldGraphEnv`] (`sf_ai::LogicalEnv`): routes over the worldspace graph
//!   (`sf_nav::WorldGraph::route`, the door links `NavWorldGraph` holds) at the actor's walk speed;
//!   placements and bodies through lookups. The shell layers its store over it, so
//!   [`LogicalEnv::actor_position`] here answers for bodies only.
//! - [`ViewsEnv`] (`sf_ai::PlayerViews` + `sf_ai::PromoteEnv`): the `in_view` frustum test on a
//!   snapshot of the tick's player views ([`view_snapshot`]), each in its viewer's worldspace, and
//!   the navmesh's nearest point for placing a promoted body.
//!
//! **Gap (named):** inside a worldspace a route is a straight leg (start to door, door to door, door
//! to goal) until `NV12f` loads the baked route graph (`docs/roadmaps/nav.md`): tier 0 then walks
//! through walls in its timing and in the points it places a body at (the navmesh snap takes the
//! nearest walkable point).

use std::cell::RefCell;

use bevy_math::Vec3;
use sf_ai::{LogicalEnv, PlayerViews, PromoteEnv, RouteRef};
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;
use sf_ids::{ActorKey, GameInstant};
use sf_nav::{DEFAULT_EXTENT, DoorState, NavBackend, QueryFilter, RouteRules, TraverseTimes, WorldGraph};
use sf_world::{LinkId, Location, PlacementId, WorldspaceId};

use crate::in_view::{AiViewer, View, ViewRules, actor_sphere, sees};

// --- Routes -----------------------------------------------------------------------------------

/// One straight piece of a route: walked from `from` to `to` over `len_m` metres, or (a door's
/// traverse) held at `from == to`, the far side's approach point, for its seconds at walk speed.
#[derive(Clone, Debug, PartialEq)]
struct Segment {
    from: Location,
    to: Location,
    len_m: f64,
}

/// A route's segments, per `(from, to, speed)` asked this tick: a linear list (the due states of one
/// tick are few; `Location` is not hashable). `None` = no route.
type Cached = (Location, Location, f32, Option<Vec<Segment>>);

/// `sf_ai::LogicalEnv` over the worldspace graph.
///
/// - [`route`](LogicalEnv::route): `WorldGraph::route` at the actor's walk speed with the host's
///   door states (a `Locked` link refuses: tier 0 opens no lock; `Closed` passes), as a
///   [`RouteRef`] with `length_m = cost_s x speed` (every leg's walk, the traverse seconds counted
///   as metres at walk speed, the final walk), so the arrival time is the graph's `cost_s`. In one
///   worldspace the graph gives the straight walk. No graph, an unknown walk speed or no passable
///   chain of links: `None` (the actor holds).
/// - [`point_on`](LogicalEnv::point_on): re-routes `from -> to` (cached for the tick) and walks the
///   straight segments `from -> leg[0].depart`, the traverse held at `leg[0].arrive` (the far
///   worldspace) for `traverse_s x speed` metres, `leg[0].arrive -> leg[1].depart`, ..., the last
///   `arrive -> to`, clamped to `[0, length_m]`; the point is in the worldspace of its segment.
///   **The route is gone** (a door locked since it was planned): straight from `from` to `to` by
///   `progress_m / length_m` when both are in one worldspace, else `from` (the actor holds at its
///   start).
pub struct WorldGraphEnv<'a> {
    graph: Option<&'a WorldGraph>,
    door_state: &'a dyn Fn(LinkId) -> DoorState,
    traverse: TraverseTimes,
    walk_speed: &'a dyn Fn(ActorKey) -> Option<f32>,
    placement: &'a dyn Fn(PlacementId) -> Option<Location>,
    body: &'a dyn Fn(ActorKey) -> Option<Location>,
    cache: RefCell<Vec<Cached>>,
}

impl<'a> WorldGraphEnv<'a> {
    /// The env for one tick: `graph` (`NavWorldGraph`'s; `None` = no route anywhere), `door_state`
    /// (`NavDoorStates::state`, `Closed` without the resource, as hearing reads it), `traverse`
    /// (`TraverseTimes::from_tunables`), `walk_speed` (m/s by actor; `None` = no route),
    /// `placement` (where a placement stands, for `Use`), `body` (where an actor's body is).
    pub fn new(
        graph: Option<&'a WorldGraph>,
        door_state: &'a dyn Fn(LinkId) -> DoorState,
        traverse: TraverseTimes,
        walk_speed: &'a dyn Fn(ActorKey) -> Option<f32>,
        placement: &'a dyn Fn(PlacementId) -> Option<Location>,
        body: &'a dyn Fn(ActorKey) -> Option<Location>,
    ) -> Self {
        Self { graph, door_state, traverse, walk_speed, placement, body, cache: RefCell::new(Vec::new()) }
    }

    /// The segments of the route `from -> to` at `speed`, routed once per tick, given to `f`.
    fn with_segments<R>(&self, from: &Location, to: &Location, speed: f32, f: impl FnOnce(Option<&[Segment]>) -> R) -> R {
        let mut cache = self.cache.borrow_mut();
        let i = if let Some(i) = cache.iter().position(|(a, b, s, _)| a == from && b == to && s.to_bits() == speed.to_bits()) {
            i
        } else {
            cache.push((from.clone(), to.clone(), speed, self.segments(from, to, speed)));
            cache.len() - 1
        };
        f(cache[i].3.as_deref())
    }

    fn segments(&self, from: &Location, to: &Location, speed: f32) -> Option<Vec<Segment>> {
        let graph = self.graph?;
        let unlock = |_: LinkId| false;
        let rules = RouteRules { speed_mps: f64::from(speed), traverse: self.traverse, unlock: &unlock };
        let route = graph.route(from, to, self.door_state, &rules)?;
        let walk = |a: &Location, b: &Location| Segment { from: a.clone(), to: b.clone(), len_m: a.pos.delta(b.pos).length() };
        let mut out = Vec::with_capacity(route.legs.len() * 2 + 1);
        let mut at = from;
        for leg in &route.legs {
            out.push(walk(at, &leg.depart));
            out.push(Segment { from: leg.arrive.clone(), to: leg.arrive.clone(), len_m: leg.traverse_s * f64::from(speed) });
            at = &leg.arrive;
        }
        out.push(walk(at, to));
        Some(out)
    }
}

/// The point `t` (0-1) of the way from `a` to `b`, in `a`'s worldspace.
fn lerp(a: &Location, b: &Location, t: f64) -> Location {
    Location::new(a.worldspace.clone(), WorldPos::from_f64(a.pos.to_f64().lerp(b.pos.to_f64(), t.clamp(0.0, 1.0))))
}

/// The point `progress_m` metres along `segments` (clamped; the last point past the end).
fn along(segments: &[Segment], progress_m: f64) -> Option<Location> {
    let mut left = progress_m.max(0.0);
    for s in segments {
        if left <= s.len_m {
            let t = if s.len_m > 0.0 { left / s.len_m } else { 1.0 };
            return Some(lerp(&s.from, &s.to, t));
        }
        left -= s.len_m;
    }
    segments.last().map(|s| s.to.clone())
}

impl LogicalEnv for WorldGraphEnv<'_> {
    #[expect(clippy::cast_possible_truncation, reason = "route lengths are well inside f32")]
    fn route(&self, actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
        let speed = (self.walk_speed)(actor).filter(|s| *s > 0.0 && s.is_finite())?;
        let length_m = self.with_segments(from, to, speed, |segs| segs.map(|s| s.iter().map(|s| s.len_m).sum::<f64>()))?;
        Some(RouteRef { from: from.clone(), to: to.clone(), length_m: length_m as f32, speed_mps: speed })
    }

    fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
        let progress = f64::from(progress_m.clamp(0.0, route.length_m.max(0.0)));
        let found = self.with_segments(&route.from, &route.to, route.speed_mps, |segs| segs.and_then(|s| along(s, progress)));
        found.unwrap_or_else(|| {
            if route.from.worldspace == route.to.worldspace && route.length_m > 0.0 {
                lerp(&route.from, &route.to, progress / f64::from(route.length_m))
            } else if route.from.worldspace == route.to.worldspace {
                route.to.clone()
            } else {
                route.from.clone()
            }
        })
    }

    fn placement(&self, id: PlacementId) -> Option<Location> {
        (self.placement)(id)
    }

    fn actor_position(&self, actor: ActorKey, _at: GameInstant) -> Option<Location> {
        (self.body)(actor)
    }
}

// --- Views and placing ------------------------------------------------------------------------

/// One player's view this tick, in the worldspace its viewer stands in.
#[derive(Clone, Debug, PartialEq)]
pub struct WorldView {
    pub worldspace: WorldspaceId,
    pub view: View,
}

/// The tick's views of the host's own viewers (every `AiViewer`: the local player's body), all in
/// the active worldspace (`ActiveWorldspace`'s; `None` = the exterior, as `processing_range` reads
/// it). A remote player's view will carry its own body's worldspace.
pub fn view_snapshot<'v>(viewers: impl IntoIterator<Item = &'v AiViewer>, active: Option<&WorldspaceId>) -> Vec<WorldView> {
    let worldspace = active.cloned().unwrap_or_else(mw_world::ids::exterior);
    viewers.into_iter().map(|v| WorldView { worldspace: worldspace.clone(), view: v.0 }).collect()
}

/// The radius the view test gives a mover's body: [`actor_sphere`]'s (half its height, or its
/// capsule radius when wider). Feed it to [`ViewsEnv::body_radius`].
pub fn body_radius_m(mover: &Mover) -> f32 {
    actor_sphere(mover).1
}

/// `sf_ai::PlayerViews` + `sf_ai::PromoteEnv` for promotion.
///
/// - `seen_by_any(at, radius)`: [`sees`] with `rules` against every view in `at`'s worldspace (a
///   viewer in another worldspace never sees it), on the sphere centred `radius` above the feet
///   point `at` ([`actor_sphere`]'s centre for every body at least as tall as it is wide).
/// - `snap(at)`: `nav.nearest(at, HUMANOID, DEFAULT_EXTENT, filter)`; `None` with no navmesh or
///   no resident tile there (the actor waits).
/// - `body_radius_m(actor)`: `body_radius` (the mover's [`body_radius_m`]).
/// - `claim`, `approaches`: the trait defaults (`AI12`).
pub struct ViewsEnv<'a> {
    /// This tick's views ([`view_snapshot`]).
    pub views: &'a [WorldView],
    /// `InViewSettings::rules`.
    pub rules: ViewRules,
    /// `NavMeshes.0`; `None` without the resource.
    pub nav: Option<&'a dyn NavBackend>,
    /// The walkers' corridor filter (`QueryFilter::for_class(&mw_bake::nav::areas(), HUMANOID)`).
    pub filter: &'a QueryFilter,
    pub body_radius: &'a dyn Fn(ActorKey) -> f32,
}

impl PlayerViews for ViewsEnv<'_> {
    fn seen_by_any(&self, at: &Location, radius_m: f32) -> bool {
        let centre = at.pos.offset(Vec3::Y * radius_m);
        self.views.iter().filter(|v| v.worldspace == at.worldspace).any(|v| sees(&v.view, centre, radius_m, self.rules))
    }
}

impl PromoteEnv for ViewsEnv<'_> {
    fn snap(&self, at: &Location) -> Option<Location> {
        Some(self.nav?.nearest(at, mw_bake::nav::HUMANOID, DEFAULT_EXTENT, self.filter)?.loc)
    }

    fn body_radius_m(&self, actor: ActorKey) -> f32 {
        (self.body_radius)(actor)
    }
}

#[cfg(test)]
#[path = "logical_tests.rs"]
mod tests;
