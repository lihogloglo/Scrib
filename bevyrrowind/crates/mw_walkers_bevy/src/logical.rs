//! The game's tier 0 (`AI10c2`; `ai.md` "Tiers and budget"): the two envs `sf_ai_bevy`'s tier-0
//! shell runs `sf_ai`'s cores against (part a), and the fixed-tick systems that run the shell in
//! the game (part b, [`add`]).
//!
//! **Envs.** Plain types over borrowed data and closures (no `Query`, no `Res`), so the systems
//! build them from their params each tick and the tests by hand. Neither keeps state across ticks.
//!
//! - [`WorldGraphEnv`] (`sf_ai::LogicalEnv`): routes over the worldspace graph
//!   (`sf_nav::WorldGraph::route`, the door links `NavWorldGraph` holds) at the actor's walk speed;
//!   placements and bodies through lookups. The shell layers its store over it, so
//!   [`LogicalEnv::actor_position`] here answers for bodies only.
//! - [`ViewsEnv`] (`sf_ai::PlayerViews` + `sf_ai::PromoteEnv`): the `in_view` frustum test on a
//!   snapshot of the tick's player views ([`view_snapshot`]), each in its viewer's worldspace, and
//!   the navmesh's nearest point for placing a promoted body.
//!
//! **Systems** (`FixedUpdate`, only while a `GameClock` exists: without one nothing enters the
//! store and the walkers run as before):
//!
//! 1. **Enter** (after the allocator, before the shell's hold): an [`AiActor`] body with a
//!    `Mover` and a directive stack, not `Held`, not marked yet, whose `AiTier` is `Logical` or
//!    that has none because it is `Dormant` (out of the simulated set), is demoted at its body
//!    ([`demote_body`]: state + [`AiLogicalBody`]). A body spawned for a key already stored is
//!    re-marked the same way. An actor without a stack is never stored.
//! 2. **Step** (`AiLogicalSystems::Step`): the due states step against a [`WorldGraphEnv`] with
//!    `mw_ai`'s policy ([`Renewing`]); their directive ends feed [`AiRuns`] (`GetAIPackageDone`)
//!    as `drive_directives`' do, for the stored actors that were due this tick, and each repeat
//!    moves its extras and its `StayNear` home to the new id.
//! 3. **Promote** (`AiLogicalSystems::Promote`): the queue the shell's hold fills (a marked body
//!    the allocator grants tier 1 or 2: a body `processing_range` woke) resolves against a
//!    [`ViewsEnv`]; each placed body in its own worldspace moves to the placement (position,
//!    zero velocity, `Mover::snap`, corridor goal cleared, wander re-chosen, `GoTo` walk
//!    restarted). A placement in another worldspace than the body's puts the state back in the
//!    store and the body stays (`AI10c3` moves bodies across worldspaces).
//! 4. **Stand-down**: `drive` stands a marked body, `drive_directives` skips it.
//! 5. **Proof line** ([`proof_line`]): `tier0: N logical, Q queued, P placed (X other-space)`,
//!    when a count changed, at most once a second.
//!
//! **Gaps (named):** inside a worldspace a route is a straight leg (start to door, door to door,
//! door to goal) until `NV12f` loads the baked route graph (`docs/roadmaps/nav.md`): tier 0 then
//! walks through walls in its timing and in the points it places a body at (the navmesh snap takes
//! the nearest walkable point). Nothing in this crate knows where a placement stands
//! (`PlacedObjects` holds ids only), so tier 0 holds on a `Use` (tier 2 drives no `Use` either). A
//! script push maps its home from the body, not from the tier-0 state. A body whose state stands
//! in another worldspace is tried again each tick it is wanted (one promotion try each). An
//! actor woken by a push ends a directive at once without its `AiRuns` read (only due states are
//! noted).

use std::cell::RefCell;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_log::{debug, info};
use bevy_math::Vec3;
use bevy_time::Time;
use sf_ai::{DirectiveDone, DirectiveId, FaceTarget, LogicalCtx, LogicalEnv, LogicalState, LogicalTransitions, PlayerViews, PromoteEnv, RouteRef, Tier};
use sf_ai_bevy::{AiActor, AiAllocSystems, AiDirectives, AiLogical, AiLogicalBody, AiLogicalSystems, AiTier, AiTierZero, demote_body};
use sf_controller_bevy::{Avoider, Dormant, Held, Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, GameInstant};
use sf_nav::{DEFAULT_EXTENT, DoorState, NavBackend, QueryFilter, RouteRules, TraverseTimes, WorldGraph};
use sf_nav_bevy::{NavCorridor, NavDoorStates, NavMeshes, NavQueries, NavWorldGraph};
use sf_sky_bevy::GameClock;
use sf_world::{LinkId, Location, PlacementId, WorldspaceId};
use sf_world_bevy::ActiveWorldspace;

use crate::directive_drive::DirectiveDrive;
use crate::directives::{AiPackageExtras, Renewing};
use crate::in_view::{AiViewer, InViewSettings, View, ViewRules, actor_sphere, sees};
use crate::reaction::Reaction;
use crate::script_ai::{AiRuns, Served, record_runs};
use crate::walkers::{ActorSpace, Walker, clear_corridor_goal};

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

// --- Systems ----------------------------------------------------------------------------------

/// The tier-0 transitions of the game's directive kinds (the built-ins: `mw_ai` maps no custom
/// kind yet).
#[derive(Resource, Debug, Default)]
pub struct Tier0Transitions(pub LogicalTransitions);

/// The counts of the proof line ([`proof_line`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tier0Counts {
    /// States stored.
    pub logical: usize,
    /// Promotion queue entries after the tick.
    pub queued: usize,
    /// Placements since start, every worldspace.
    pub placed: usize,
    /// Of those, placements in another worldspace than the body's (put back in the store).
    pub other_space: usize,
}

/// `tier0: N logical, Q queued, P placed (X other-space)`.
pub fn proof_line(c: Tier0Counts) -> String {
    format!("tier0: {} logical, {} queued, {} placed ({} other-space)", c.logical, c.queued, c.placed, c.other_space)
}

/// One fixed tick of the game's tier 0 between its systems, and the proof line's counts.
#[derive(Resource, Debug, Default)]
pub struct Tier0Tick {
    /// What each stored actor due this tick served before the step ([`AiRuns`]).
    served: Vec<(ActorKey, Served)>,
    /// The step's directive ends this tick.
    pub events: Vec<DirectiveDone>,
    /// The repeats the step renewed, `(old, new)`, in step order.
    renewed: Vec<(DirectiveId, DirectiveId)>,
    /// This tick's placements, where each body went (its own worldspace) or would have gone.
    pub placements: Vec<(ActorKey, Location)>,
    /// Counts since start (`logical` and `queued`: this tick's).
    pub counts: Tier0Counts,
    /// The counts last logged, and when (fixed-clock seconds).
    logged: (Tier0Counts, Option<f64>),
}

/// The proof line's cadence: at most one line a second (also `crate::poses`' line).
// tunables: const (log cadence, not a gameplay or quality value)
pub(crate) const LOG_EVERY_S: f64 = 1.0;

/// The tier-0 systems on the fixed tick (module docs), only while a [`GameClock`] exists. Needs
/// `sf_ai_bevy::AiPlugin` (the store, the queue, the sets); `walkers::add_drive` calls it.
pub fn add(app: &mut App) {
    app.init_resource::<Tier0Transitions>()
        .init_resource::<Tier0Tick>()
        .init_resource::<crate::bodiless::BodilessSpawns>()
        .init_resource::<sf_ai_bevy::AiBodilessStats>()
        .add_systems(
            FixedUpdate,
            (
                enter_tier0.after(AiAllocSystems::Allocate).before(AiLogicalSystems::Hold),
                (
                    note_served,
                    step_tier0,
                    after_step,
                    crate::bodiless::follow_bodiless,
                    crate::bodiless::spawn_bodiless,
                    crate::bodiless::report_bodiless,
                )
                    .chain()
                    .in_set(AiLogicalSystems::Step),
                promote_tier0.in_set(AiLogicalSystems::Promote),
            )
                .run_if(resource_exists::<GameClock>),
        );
}

/// The world the tier-0 env reads: the graph, the doors, the traverse tunables, the transitions.
#[derive(SystemParam)]
pub struct Tier0World<'w> {
    clock: Res<'w, GameClock>,
    graph: Option<Res<'w, NavWorldGraph>>,
    doors: Option<Res<'w, NavDoorStates>>,
    tunables: Option<Res<'w, sf_tunables_bevy::Tunables>>,
    transitions: Res<'w, Tier0Transitions>,
}

impl Tier0World<'_> {
    /// The tick's game instant.
    pub(crate) fn now(&self) -> GameInstant {
        self.clock.now
    }

    /// The worldspace graph (`None` without `NavWorldGraph`).
    pub(crate) fn graph(&self) -> Option<&WorldGraph> {
        self.graph.as_deref().map(|g| &g.0)
    }

    pub(crate) fn traverse(&self) -> TraverseTimes {
        self.tunables.as_ref().map_or_else(TraverseTimes::default, |t| TraverseTimes::from_tunables(&t.0))
    }

    pub(crate) fn door(&self, id: LinkId) -> DoorState {
        self.doors.as_ref().map_or(DoorState::Closed, |d| d.state(id))
    }

    pub(crate) fn time_scale(&self) -> f64 {
        f64::from(self.clock.time_scale)
    }
}

/// The walk speed a `GoTo` uses (`drive_directives`): the reaction's, the wander's, the corridor's.
fn walk_speed_of(reaction: Option<&Reaction>, walker: Option<&Walker>, nav: Option<&NavCorridor>) -> Option<f32> {
    reaction.and_then(|r| r.walk_speed).or_else(|| walker.map(|w| w.driver.walk_speed)).or_else(|| nav.map(|n| n.follower.speed))
}

/// Every body's key, sorted, for the envs' lookups (built only on ticks with a stored state).
fn key_index(keys: impl Iterator<Item = (ActorKey, Entity)>) -> Vec<(ActorKey, Entity)> {
    let mut index: Vec<_> = keys.collect();
    index.sort_unstable();
    index
}

fn find(index: &[(ActorKey, Entity)], key: ActorKey) -> Option<Entity> {
    index.binary_search_by_key(&key, |p| p.0).ok().map(|i| index[i].1)
}

/// The view radius of an actor without a body: the default capsule's ([`actor_sphere`]'s rule).
pub(crate) fn default_radius_m() -> f32 {
    let p = MoverParams::default();
    (p.height * 0.5).max(p.radius)
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn enter_tier0(
    clock: Res<GameClock>,
    directives: Res<AiDirectives>,
    mut store: ResMut<AiLogical>,
    mut commands: Commands,
    bodies: Query<
        (Entity, &AiActor, &Mover, &ActorSpace, Option<&AiTier>, Has<Dormant>),
        (Without<AiLogicalBody>, Without<Held>),
    >,
) {
    for (entity, actor, mover, space, tier, dormant) in &bodies {
        let key = actor.0;
        let logical = tier.map_or(dormant, |t| t.tier == Tier::Logical);
        if !(logical || store.contains(key)) || directives.get(key).is_none() {
            continue;
        }
        let body = Location::new(space.0.clone(), mover.loco.position);
        demote_body(&mut commands, &mut store, entity, key, body, clock.now);
    }
}

/// Notes what each due state's stack serves before the step ([`AiRuns`]) and clears the tick.
#[allow(clippy::needless_pass_by_value)]
fn note_served(clock: Res<GameClock>, store: Res<AiLogical>, directives: Res<AiDirectives>, mut tick: ResMut<Tier0Tick>) {
    let tick = &mut *tick;
    tick.served.clear();
    tick.events.clear();
    tick.renewed.clear();
    let now = clock.now;
    for (key, state) in store.iter() {
        let due = state.directive.is_none() || state.until.is_some_and(|u| u <= now);
        if due && let Some(stack) = directives.get(key) {
            tick.served.push((key, Served::of(stack)));
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn step_tier0(
    mut t0: AiTierZero,
    world: Tier0World,
    mut tick: ResMut<Tier0Tick>,
    bodies: Query<(Entity, &AiActor, &Mover, &ActorSpace, Option<&Reaction>, Option<&Walker>, Option<&NavCorridor>)>,
) {
    if t0.store().is_empty() {
        return;
    }
    let index = key_index(bodies.iter().map(|(e, a, ..)| (a.0, e)));
    let lookup = |k: ActorKey| find(&index, k).and_then(|e| bodies.get(e).ok());
    let speed = |k: ActorKey| lookup(k).and_then(|(_, _, _, _, r, w, n)| walk_speed_of(r, w, n));
    let body = |k: ActorKey| lookup(k).map(|(_, _, m, s, ..)| Location::new(s.0.clone(), m.loco.position));
    let door = |id: LinkId| world.door(id);
    let nowhere = |_: PlacementId| None;
    let env = WorldGraphEnv::new(world.graph.as_deref().map(|g| &g.0), &door, world.traverse(), &speed, &nowhere, &body);
    let policy = Renewing::default();
    let ctx = LogicalCtx { env: &env, transitions: &world.transitions.0, policy: &policy, time_scale: world.time_scale() };
    let mut events = std::mem::take(&mut tick.events);
    t0.step(world.clock.now, &ctx, &mut events);
    tick.events = events;
    tick.renewed = policy.take();
}

/// The step's ends into [`AiRuns`] and the debug log (as `drive_directives` sends its own), and
/// each repeat's extras and home onto its new id.
#[allow(clippy::needless_pass_by_value)]
fn after_step(
    tick: Res<Tier0Tick>,
    directives: Res<AiDirectives>,
    runs: Option<ResMut<AiRuns>>,
    mut actors: Query<(&AiActor, Option<&mut DirectiveDrive>, Option<&mut AiPackageExtras>)>,
) {
    if let Some(mut runs) = runs
        && !tick.served.is_empty()
    {
        record_runs(&mut runs, &tick.served, &tick.events);
    }
    for e in &tick.events {
        debug!("directive {:?} of actor {:?} ended ({:?}) at tier 0", e.id, e.actor, e.end);
    }
    if tick.renewed.is_empty() {
        return;
    }
    // Each repeat ended on one actor's stack this tick (its end event, in the same order): the
    // first such actor whose stack now holds the new id.
    let mut from = 0;
    for &(old, new) in &tick.renewed {
        let hit = tick.events[from..].iter().position(|e| {
            e.id == old && directives.get(e.actor).is_some_and(|s| s.get(new).is_some() && s.get(old).is_none())
        });
        let Some(i) = hit else { continue };
        let actor = tick.events[from + i].actor;
        from += i + 1;
        let Some(stack) = directives.get(actor) else { continue };
        for (a, drive, extras) in &mut actors {
            if a.0 != actor {
                continue;
            }
            if let Some(mut drive) = drive {
                drive.follow(&[(old, new)], stack);
            }
            if let Some(mut extras) = extras {
                extras.follow(&[(old, new)]);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity, clippy::too_many_lines)]
fn promote_tier0(
    mut t0: AiTierZero,
    world: Tier0World,
    time: Res<Time>,
    views: (Query<&AiViewer>, Option<Res<InViewSettings>>, Option<Res<ActiveWorldspace>>),
    meshes: Option<Res<NavMeshes>>,
    mut queries: Option<ResMut<NavQueries>>,
    mut tick: ResMut<Tier0Tick>,
    mut filter: Local<Option<QueryFilter>>,
    mut bodies: Query<(
        (Entity, &AiActor, &mut Mover, &ActorSpace, Has<AiLogicalBody>, Has<Dormant>),
        (Option<&Reaction>, Option<&mut Walker>, Option<&mut NavCorridor>, Option<&mut Avoider>, Option<&mut DirectiveDrive>),
    )>,
) {
    let mut queued = 0;
    tick.placements.clear();
    if !t0.store().is_empty() {
        let index = key_index(bodies.iter().map(|((e, a, ..), _)| (a.0, e)));
        // A marked, awake body whose state stands (or walks) in another worldspace: its state, to
        // put back if the placement lands there.
        let held_back: Vec<(ActorKey, LogicalState)> = bodies
            .iter()
            .filter(|((_, _, _, _, marked, dormant), _)| *marked && !*dormant)
            .filter_map(|((_, a, _, space, ..), _)| {
                let state = t0.store().get(a.0)?;
                let elsewhere = |l: &Location| l.worldspace != space.0;
                let other = elsewhere(&state.pos) || state.route.as_ref().is_some_and(|r| elsewhere(&r.from) || elsewhere(&r.to));
                other.then(|| (a.0, state.clone()))
            })
            .collect();
        let filter = &*filter.get_or_insert_with(|| QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID));
        let out = {
            let lookup = |k: ActorKey| find(&index, k).and_then(|e| bodies.get(e).ok());
            let speed = |k: ActorKey| lookup(k).and_then(|(_, (r, w, n, ..))| walk_speed_of(r, w, n));
            let body = |k: ActorKey| lookup(k).map(|((_, _, m, s, ..), _)| Location::new(s.0.clone(), m.loco.position));
            let radius = |k: ActorKey| lookup(k).map_or_else(default_radius_m, |((_, _, m, ..), _)| body_radius_m(m));
            let door = |id: LinkId| world.door(id);
            let nowhere = |_: PlacementId| None;
            let graph = world.graph.as_deref().map(|g| &g.0);
            let env = WorldGraphEnv::new(graph, &door, world.traverse(), &speed, &nowhere, &body);
            let (viewers, settings, active) = &views;
            let snapshot = view_snapshot(viewers.iter(), active.as_ref().and_then(|a| a.0.as_ref()));
            let rules = settings.as_ref().map_or_else(ViewRules::default, |s| s.rules);
            let nav = meshes.as_deref().map(|m| &*m.0);
            let promote = ViewsEnv { views: &snapshot, rules, nav, filter, body_radius: &radius };
            t0.promote(world.clock.now, world.time_scale(), &env, &promote, &body)
        };
        queued = out.queued;
        for (key, _tier, p) in out.placed {
            tick.counts.placed += 1;
            tick.placements.push((key, p.at.clone()));
            let Some(e) = find(&index, key) else { continue };
            let Ok(((_, _, mut mover, space, ..), (_, walker, nav, avoider, drive))) = bodies.get_mut(e) else { continue };
            if p.at.worldspace != space.0 {
                tick.counts.other_space += 1;
                let state = held_back
                    .iter()
                    .find(|(k, _)| *k == key)
                    .map_or_else(|| LogicalState::new(p.at.clone(), world.clock.now), |(_, s)| s.clone());
                t0.store_mut().insert(key, state);
                continue;
            }
            if mover.loco.position != p.at.pos {
                mover.loco.position = p.at.pos;
                mover.loco.motion.velocity = Vec3::ZERO;
                mover.loco.motion.acceleration = Vec3::ZERO;
                mover.measured = Vec3::ZERO;
                mover.snap();
            }
            if let Some(mut nav) = nav {
                clear_corridor_goal(&mut nav, queries.as_deref_mut());
            }
            if let Some(mut w) = walker {
                w.release_held_intent();
                w.driver.face(FaceTarget::Yaw(mover.loco.motion.yaw), 0.0);
            }
            if let Some(mut a) = avoider {
                a.travel = Vec3::ZERO;
            }
            if let Some(mut d) = drive {
                d.restart_walk();
            }
        }
    }
    tick.counts.logical = t0.store().len();
    tick.counts.queued = queued;
    let (counts, now_s) = (tick.counts, time.elapsed_secs_f64());
    let (last, at) = tick.logged;
    if counts != last && at.is_none_or(|t| now_s - t >= LOG_EVERY_S) {
        info!("{}", proof_line(counts));
        tick.logged = (counts, Some(now_s));
    }
}

#[cfg(test)]
#[path = "logical_tests.rs"]
mod tests;
