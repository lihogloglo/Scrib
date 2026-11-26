//! The tier-0 envs (`AI10c2a`) on hand-built data: a two-worldspace door graph, a fixed view, the
//! grid navmesh the drive tests use.

use bevy_math::{DVec3, Quat, Vec2};
use sf_ai::{LogicalEnv, PlayerViews, PromoteEnv};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::{ActorKey, GameInstant};
use sf_nav::{DoorState, GridBackend, GridCell, GridConfig, LinkFlags, NavBackend, PolyFlags, QueryFilter, TraverseTimes, WorldGraph};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, PlacementId, WorldspaceId};

use super::{ViewsEnv, WorldGraphEnv, WorldView};
use crate::in_view::{View, ViewRules};

/// Walk speed, m/s.
const SPEED: f32 = 2.0;
/// Seamless traverse, s (6 m at [`SPEED`]).
const SEAMLESS_S: f64 = 3.0;
const ACTOR: ActorKey = ActorKey(7);

fn loc(ws: &str, x: f64, z: f64) -> Location {
    Location::new(WorldspaceId::new(ws), WorldPos::from_f64(DVec3::new(x, 0.0, z)))
}

fn near(a: &Location, b: &Location) -> bool {
    a.worldspace == b.worldspace && a.pos.delta(b.pos).length() < 1e-4
}

#[track_caller]
fn assert_near(a: &Location, b: &Location) {
    assert!(near(a, b), "{a:?} != {b:?}");
}

fn end(l: &Location) -> LinkEnd {
    let frame = WorldTransform::from_pos(l.pos);
    LinkEnd { worldspace: l.worldspace.clone(), frame, arrival: frame, door: None, extent: Vec2::ZERO }
}

/// One two-way seamless door: `e` (6, 8) <-> `a` (0, 0).
fn graph() -> (WorldGraph, LinkId) {
    let link = DoorLink { a: end(&loc("e", 6.0, 8.0)), b: end(&loc("a", 0.0, 0.0)), two_way: true };
    let id = link.id();
    let approach = |e: &LinkEnd| Some(Location::new(e.worldspace.clone(), e.frame.pos));
    (WorldGraph::build(&[link], approach, |_: &DoorLink| LinkFlags::SEAMLESS), id)
}

fn traverse() -> TraverseTimes {
    TraverseTimes { seamless_s: SEAMLESS_S, fade_s: 1.0 }
}

fn speed(actor: ActorKey) -> Option<f32> {
    (actor == ACTOR).then_some(SPEED)
}

fn none_at(_: PlacementId) -> Option<Location> {
    None
}

fn no_body(_: ActorKey) -> Option<Location> {
    None
}

fn open(_: LinkId) -> DoorState {
    DoorState::Open
}

fn env<'a>(graph: Option<&'a WorldGraph>, doors: &'a dyn Fn(LinkId) -> DoorState) -> WorldGraphEnv<'a> {
    WorldGraphEnv::new(graph, doors, traverse(), &speed, &none_at, &no_body)
}

#[test]
fn same_space_is_the_straight_walk() {
    let g = WorldGraph::default();
    let env = env(Some(&g), &open);
    let (from, to) = (loc("e", 0.0, 0.0), loc("e", 6.0, 8.0));
    let route = env.route(ACTOR, &from, &to).expect("a straight route");
    assert!((route.length_m - 10.0).abs() < 1e-5, "{}", route.length_m);
    assert!((route.speed_mps - SPEED).abs() < f32::EPSILON);
    assert_near(&env.point_on(&route, 0.0), &from);
    assert_near(&env.point_on(&route, 5.0), &loc("e", 3.0, 4.0));
    assert_near(&env.point_on(&route, 10.0), &to);
    assert_near(&env.point_on(&route, 15.0), &to);
    assert_near(&env.point_on(&route, -3.0), &from);
}

#[test]
fn door_route_walks_traverses_and_walks_on() {
    let (g, _) = graph();
    let env = env(Some(&g), &open);
    let (from, to) = (loc("e", 0.0, 0.0), loc("a", 3.0, 4.0));
    let route = env.route(ACTOR, &from, &to).expect("through the door");
    // 10 m to the door, 3 s x 2 m/s through it, 5 m on: cost 10.5 s.
    let cost_s = 10.0 / f64::from(SPEED) + SEAMLESS_S + 5.0 / f64::from(SPEED);
    assert!((f64::from(route.length_m) - cost_s * f64::from(SPEED)).abs() < 1e-3, "{}", route.length_m);
    assert_near(&env.point_on(&route, 9.0), &loc("e", 5.4, 7.2));
    assert_near(&env.point_on(&route, 10.5), &loc("a", 0.0, 0.0));
    assert_near(&env.point_on(&route, 15.9), &loc("a", 0.0, 0.0));
    assert_near(&env.point_on(&route, 18.5), &loc("a", 1.5, 2.0));
    assert_near(&env.point_on(&route, route.length_m), &to);
    assert_near(&env.point_on(&route, 100.0), &to);

    // Monotonic: never back into `e`; within each space, ever farther from its entry point.
    let (mut in_a, mut last) = (false, 0.0_f64);
    let mut p = 0.0_f32;
    while p <= route.length_m {
        let at = env.point_on(&route, p);
        let a = at.worldspace == WorldspaceId::new("a");
        assert!(a || !in_a, "back into e at {p}");
        if a && !in_a {
            (in_a, last) = (true, 0.0);
        }
        let entry = if a { loc("a", 0.0, 0.0) } else { from.clone() };
        let d = entry.pos.delta(at.pos).length();
        assert!(d + 1e-6 >= last, "went back at {p}: {d} < {last}");
        last = d;
        p += 0.25;
    }
    assert!(in_a);
}

#[test]
fn locked_door_no_graph_or_unknown_speed_hold() {
    let (g, id) = graph();
    let (from, to) = (loc("e", 0.0, 0.0), loc("a", 3.0, 4.0));
    let closed = |_: LinkId| DoorState::Closed;
    assert!(env(Some(&g), &closed).route(ACTOR, &from, &to).is_some(), "a closed door opens");
    let locked = move |l: LinkId| if l == id { DoorState::Locked } else { DoorState::Open };
    assert_eq!(env(Some(&g), &locked).route(ACTOR, &from, &to), None);
    assert_eq!(env(None, &open).route(ACTOR, &from, &loc("e", 1.0, 0.0)), None);
    assert_eq!(env(Some(&g), &open).route(ActorKey(8), &from, &to), None);

    // Planned open, locked since: across spaces the actor holds at its start; in one space it
    // goes straight.
    let route = env(Some(&g), &open).route(ACTOR, &from, &to).unwrap();
    assert_near(&env(Some(&g), &locked).point_on(&route, 12.0), &from);
    let g0 = WorldGraph::default();
    let flat = env(Some(&g0), &open).route(ACTOR, &from, &loc("e", 6.0, 8.0)).unwrap();
    assert_near(&env(None, &open).point_on(&flat, 5.0), &loc("e", 3.0, 4.0));
}

#[test]
fn placement_and_bodies_come_from_the_lookups() {
    let bench = PlacementId(42);
    let at = |id: PlacementId| (id == bench).then(|| loc("e", 1.0, 2.0));
    let body = |k: ActorKey| (k == ACTOR).then(|| loc("a", 3.0, 0.0));
    let env = WorldGraphEnv::new(None, &open, traverse(), &speed, &at, &body);
    assert_near(&env.placement(bench).unwrap(), &loc("e", 1.0, 2.0));
    assert_eq!(env.placement(PlacementId(43)), None);
    assert_near(&env.actor_position(ACTOR, GameInstant::default()).unwrap(), &loc("a", 3.0, 0.0));
    assert_eq!(env.actor_position(ActorKey(8), GameInstant::default()), None);
}

fn radius(_: ActorKey) -> f32 {
    0.9
}

/// A viewer at the origin of `e`, eye 1.6 m up, looking down -Z.
fn view() -> WorldView {
    let eye = WorldPos::from_f64(DVec3::new(0.0, 1.6, 0.0));
    WorldView { worldspace: WorldspaceId::new("e"), view: View { eye, rotation: Quat::IDENTITY, fov_y: 1.0, aspect: 1.5 } }
}

#[test]
fn views_see_the_frustum_in_their_own_space() {
    let views = [view()];
    let filter = QueryFilter::default();
    let env = ViewsEnv { views: &views, rules: ViewRules::default(), nav: None, filter: &filter, body_radius: &radius };
    assert!(env.seen_by_any(&loc("e", 0.0, -10.0), 0.9), "in front, in range");
    assert!(env.unseen(&loc("e", 0.0, 10.0), 0.9), "behind");
    assert!(env.unseen(&loc("a", 0.0, -10.0), 0.9), "another worldspace");
    assert!(env.unseen(&loc("e", 0.0, -200.0), 0.9), "past max_m");
    assert!((env.body_radius_m(ACTOR) - 0.9).abs() < f32::EPSILON);
    let none = ViewsEnv { views: &[], ..env };
    assert!(none.unseen(&loc("e", 0.0, -10.0), 0.9), "no views");
}

#[test]
fn snap_lands_on_the_navmesh() {
    let ws = WorldspaceId::new("nav_room");
    let mut backend = GridBackend::new(GridConfig { cell_size: 1.0, tile_cells: 40 }).unwrap();
    let row = ".".repeat(20);
    let rows: Vec<&str> = (0..20).map(|_| row.as_str()).collect();
    backend
        .insert_rows(&ws, mw_bake::nav::HUMANOID, 0.5, &rows, |c| {
            (c == '.').then_some(GridCell::new(0.0, mw_bake::nav::GROUND, PolyFlags::WALK))
        })
        .unwrap();
    let filter = QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID);
    let nav: &dyn NavBackend = &backend;
    let env = ViewsEnv { views: &[], rules: ViewRules::default(), nav: Some(nav), filter: &filter, body_radius: &radius };
    let at = Location::new(ws.clone(), WorldPos::from_f64(DVec3::new(5.3, 0.8, 5.7)));
    let snapped = env.snap(&at).expect("on the mesh");
    assert_eq!(snapped.worldspace, ws);
    let d = at.pos.delta(snapped.pos);
    assert!(d.y.abs() <= 0.8 + 1e-4 && d.x.hypot(d.z) < 1.0, "{snapped:?}");
    assert!(env.snap(&Location::new(ws, WorldPos::from_f64(DVec3::new(100.0, 0.0, 100.0)))).is_none(), "off the mesh");
    let bare = ViewsEnv { nav: None, ..env };
    assert!(bare.snap(&at).is_none(), "no navmesh");
}

// --- In the game (AI10c2b): the systems on the walkers' fixed-tick harness ---------------------

mod game {
    use bevy_app::{App, FixedUpdate};
    use bevy_ecs::prelude::*;
    use bevy_math::{DVec3, Quat, Vec3};
    use bevy_time::Time;
    use mw_ai::MorrowindPolicy;
    use mw_ai::packages::PackageExtra;
    use sf_ai::{
        DirectiveDone, DirectiveEnd, DirectiveId, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, DirectiveStatus,
        GameTime, Tier, TierCaps,
    };
    use sf_ai_bevy::{AiActor, AiAllocSettings, AiAllocSystems, AiDirectives, AiLogical, AiLogicalBody, AiLogicalSystems, AiTier};
    use sf_controller_bevy::{Dormant, Mover};
    use sf_coords::WorldPos;
    use sf_ids::{ActorKey, GameInstant};
    use sf_nav_bevy::NavWorldGraph;
    use sf_sky_bevy::GameClock;
    use sf_world::{Location, WorldspaceId};
    use sf_world_bevy::ActiveWorldspace;

    use crate::directive_drive::DirectiveDrive;
    use crate::directives::AiPackageExtras;
    use crate::in_view::{AiViewer, View};
    use crate::logical::{Tier0Counts, Tier0Tick, proof_line};
    use crate::walkers::Walker;
    use crate::walkers::test_harness::{app_with, spawn_wanderer};

    const BIG: TierCaps = TierCaps { full: 64, reduced: 64 };
    /// The harness's fixed ticks per second.
    const HZ: usize = 60;

    fn ws() -> WorldspaceId {
        WorldspaceId::new("nav_room")
    }

    fn at(x: f64, z: f64) -> WorldPos {
        WorldPos::from_f64(DVec3::new(x, 0.0, z))
    }

    fn planar(a: WorldPos, b: WorldPos) -> f32 {
        a.delta_f32(b).with_y(0.0).length()
    }

    /// Every tier-0 directive end, with the instant of the tick it was stepped on.
    #[derive(Resource, Default)]
    struct Ends(Vec<(DirectiveDone, GameInstant)>);

    /// The clock on the fixed tick (the sky plugin's job in the game): x30.
    #[allow(clippy::needless_pass_by_value)]
    fn advance_clock(time: Res<Time>, mut clock: ResMut<GameClock>) {
        clock.advance(time.delta());
    }

    #[allow(clippy::needless_pass_by_value)]
    fn collect_ends(tick: Res<Tier0Tick>, clock: Res<GameClock>, mut ends: ResMut<Ends>) {
        ends.0.extend(tick.events.iter().map(|e| (e.clone(), clock.now)));
    }

    /// The walkers' harness with the game clock, an empty worldspace graph (straight walks in one
    /// worldspace), the room as the active worldspace and `caps`.
    fn game(caps: TierCaps) -> App {
        let mut app = app_with(|app| {
            app.insert_resource(GameClock::default())
                .insert_resource(NavWorldGraph::default())
                .insert_resource(ActiveWorldspace(Some(ws())))
                .init_resource::<Ends>()
                .add_systems(
                    FixedUpdate,
                    (
                        advance_clock.before(AiAllocSystems::Allocate),
                        collect_ends.after(AiLogicalSystems::Step).before(AiLogicalSystems::Promote),
                    ),
                );
        });
        app.world_mut().resource_mut::<AiAllocSettings>().set_caps(caps);
        app
    }

    /// A wanderer at `home` as `attach` makes it, its stack seeded as `seed_directives` does:
    /// `before` (record order), then the record wander. Returns the entity and the ids.
    fn actor(app: &mut App, key: u128, home: WorldPos, before: Vec<DirectiveKind>) -> (Entity, Vec<DirectiveId>) {
        let e = spawn_wanderer(app, &ws(), home, key, 11);
        let wander = app.world().get::<Walker>(e).unwrap().driver.planner.wander;
        let now = app.world().get_resource::<GameClock>().map_or_else(GameTime::default, |c| GameTime::from(c.now));
        let mut stack = DirectiveStack::new(ActorKey(key), now);
        let mut events = Vec::new();
        let mut ids: Vec<_> = before
            .into_iter()
            .map(|k| stack.push(DirectiveSpec::new(k, DirectiveSource::Record), now, &MorrowindPolicy, &mut events))
            .collect();
        let kind = DirectiveKind::StayNear { home: Location::new(ws(), home), radius: wander.distance };
        let stay = stack.push(DirectiveSpec::new(kind, DirectiveSource::Record).repeating(), now, &MorrowindPolicy, &mut events);
        ids.push(stay);
        app.world_mut().resource_mut::<AiDirectives>().insert(stack);
        app.world_mut().entity_mut(e).insert((
            AiActor(ActorKey(key)),
            AiPackageExtras(vec![(stay, PackageExtra::Wander(wander))]),
            DirectiveDrive::default(),
        ));
        (e, ids)
    }

    fn go_to(x: f64, z: f64) -> DirectiveKind {
        DirectiveKind::GoTo(Location::new(ws(), at(x, z)))
    }

    fn stored(app: &App, key: u128) -> bool {
        app.world().resource::<AiLogical>().contains(ActorKey(key))
    }

    fn position(app: &App, e: Entity) -> WorldPos {
        app.world().get::<Mover>(e).unwrap().loco.position
    }

    fn status(app: &App, key: u128, id: DirectiveId) -> DirectiveStatus {
        app.world().resource::<AiDirectives>().get(ActorKey(key)).expect("a stack").status(id)
    }

    fn now(app: &App) -> GameInstant {
        app.world().resource::<GameClock>().now
    }

    /// Tier invariance in the game: the same 20 m `GoTo`, walked by a Full walker over the navmesh
    /// and by a stored one on the world graph, ends `Completed` within one game minute.
    #[test]
    fn a_full_walker_and_a_stored_one_complete_the_same_go_to_within_a_game_minute() {
        let mut app = game(BIG);
        let (full, full_ids) = actor(&mut app, 1, at(10.0, 10.0), vec![go_to(30.0, 10.0)]);
        let (logical, logical_ids) = actor(&mut app, 2, at(10.0, 30.0), vec![go_to(30.0, 30.0)]);
        app.world_mut().entity_mut(logical).insert(Dormant);
        let mut full_end = None;
        for _ in 0..(40 * HZ) {
            app.update();
            if full_end.is_none() && status(&app, 1, full_ids[0]) == DirectiveStatus::Ended {
                full_end = Some(now(&app));
            }
            if full_end.is_some() && status(&app, 2, logical_ids[0]) == DirectiveStatus::Ended {
                break;
            }
        }
        assert!(stored(&app, 2) && !stored(&app, 1));
        assert!(app.world().entity(logical).contains::<AiLogicalBody>());
        let full_end = full_end.expect("the Full walker never arrived");
        assert_eq!(app.world().get::<DirectiveDrive>(full).unwrap().stats.goto_finished, 1, "the Full walk arrived");
        let ends: Vec<_> = app.world().resource::<Ends>().0.iter().filter(|(e, _)| e.id == logical_ids[0]).cloned().collect();
        assert_eq!(ends.len(), 1, "{ends:?}");
        let (end, stepped_at) = &ends[0];
        assert_eq!((end.actor, end.end), (ActorKey(2), DirectiveEnd::Completed));
        let gap_ms = full_end.0.abs_diff(stepped_at.0);
        println!("AI10c2b: Full arrived at {} ms, tier 0 at {} ms, {gap_ms} ms apart", full_end.0, stepped_at.0);
        assert!(gap_ms <= 60_000, "{gap_ms} game ms apart");
        assert!(app.world().resource::<crate::script_ai::AiRuns>().get(ActorKey(2)).done, "GetAIPackageDone at tier 0");
        assert!(planar(position(&app, logical), at(10.0, 30.0)) < 1e-3, "the dormant body moved");
    }

    /// A viewer at (20, 1.6, 45) looking down -Z over the room's middle band.
    fn viewer(app: &mut App) {
        let eye = WorldPos::from_f64(DVec3::new(20.0, 1.6, 45.0));
        app.world_mut().spawn(AiViewer(View { eye, rotation: Quat::IDENTITY, fov_y: 1.0, aspect: 1.5 }));
    }

    #[test]
    fn a_dormant_walker_enters_then_is_promoted_awake_and_unseen() {
        let mut app = game(BIG);
        let (e, _) = actor(&mut app, 3, at(20.0, 20.0), Vec::new());
        app.world_mut().entity_mut(e).insert(Dormant);
        app.update();
        app.update();
        assert!(stored(&app, 3), "a dormant walker with a stack is tier 0");
        assert!(app.world().entity(e).contains::<AiLogicalBody>());
        app.world_mut().entity_mut(e).remove::<Dormant>();
        for _ in 0..10 {
            app.update();
        }
        assert!(!stored(&app, 3), "awake and unseen: promoted");
        assert!(!app.world().entity(e).contains::<AiLogicalBody>());
        assert_eq!(app.world().get::<AiTier>(e).map(|t| t.tier), Some(Tier::Full), "the allocated tier");
        assert!(planar(position(&app, e), at(20.0, 20.0)) < 0.5, "a waiter is placed on its body");
        assert!(app.world().resource::<Tier0Tick>().counts.placed >= 1);
    }

    #[test]
    fn awake_at_a_seen_spot_away_from_its_body_it_stays_stored_and_the_body_stays() {
        let mut app = game(BIG);
        viewer(&mut app);
        let start = at(10.0, 20.0);
        let (e, _) = actor(&mut app, 4, start, vec![go_to(30.0, 20.0)]);
        app.world_mut().entity_mut(e).insert(Dormant);
        app.update();
        app.update();
        assert!(stored(&app, 4));
        // 240 game s at x30 = 8 s of walking at 1.5 m/s: its state stands 12 m on, in view.
        app.world_mut().resource_mut::<GameClock>().skip(240_000);
        app.world_mut().entity_mut(e).remove::<Dormant>();
        for _ in 0..30 {
            app.update();
            assert!(stored(&app, 4), "promoted into view");
            assert!(planar(position(&app, e), start) < 0.05, "the body moved");
        }
        assert_eq!(app.world().resource::<Tier0Tick>().counts.queued, 1, "it waits in the queue");
        assert!(app.world().entity(e).contains::<AiLogicalBody>());
    }

    /// Where key 5's body stood right after this tick's placement, with its velocity.
    #[derive(Resource, Default)]
    struct Placed(Option<(Location, WorldPos, Vec3)>);

    #[allow(clippy::needless_pass_by_value)]
    fn record_placed(tick: Res<Tier0Tick>, bodies: Query<(&AiActor, &Mover)>, mut placed: ResMut<Placed>) {
        let Some((_, to)) = tick.placements.iter().find(|(k, _)| *k == ActorKey(5)) else { return };
        if let Some((_, m)) = bodies.iter().find(|(a, _)| a.0 == ActorKey(5)) {
            placed.0 = Some((to.clone(), m.loco.position, m.loco.motion.velocity));
        }
    }

    #[test]
    fn placement_moves_the_body_and_it_walks_on_from_there() {
        let mut app = game(BIG);
        app.init_resource::<Placed>()
            .add_systems(FixedUpdate, record_placed.after(AiLogicalSystems::Promote).before(AiLogicalSystems::Cleanup));
        let start = at(10.0, 20.0);
        let goal = at(30.0, 20.0);
        let (e, ids) = actor(&mut app, 5, start, vec![go_to(30.0, 20.0)]);
        app.world_mut().entity_mut(e).insert(Dormant);
        app.update();
        app.update();
        assert!(stored(&app, 5));
        app.world_mut().resource_mut::<GameClock>().skip(240_000);
        app.world_mut().entity_mut(e).remove::<Dormant>();
        for _ in 0..10 {
            app.update();
            if !stored(&app, 5) {
                break;
            }
        }
        assert!(!stored(&app, 5), "unseen: promoted");
        let (to, body, velocity) = app.world().resource::<Placed>().0.clone().expect("placed");
        assert_eq!(to.worldspace, ws());
        assert_eq!(body, to.pos, "the body stands on the snapped point");
        assert_eq!(velocity, Vec3::ZERO);
        let along = planar(to.pos, start);
        assert!((11.0..=13.5).contains(&along), "placed {along} m along");
        // It walks on from there: never back toward the start, and on toward the goal.
        let mut last = planar(position(&app, e), goal);
        for tick in 0..(4 * HZ) {
            app.update();
            let d = planar(position(&app, e), goal);
            assert!(d.is_finite() && d <= last + 0.05, "tick {tick}: walked back ({d} m > {last} m from the goal)");
            assert!(planar(position(&app, e), start) >= along - 0.3, "tick {tick}: back toward the old spot");
            last = d;
        }
        assert!(last < planar(to.pos, goal) - 1.0, "it never walked on ({last} m left)");
        assert_eq!(status(&app, 5, ids[0]), DirectiveStatus::Serving);
        assert!(app.world().get::<DirectiveDrive>(e).unwrap().owns(), "the GoTo walks it");
    }

    /// Caps 0: an awake walker the allocator puts at tier 0 enters the store; its `GoTo` ends once,
    /// from the tier-0 step, while `drive_directives` never walks it and its body stands.
    #[test]
    fn drive_directives_leaves_a_stored_stack_to_the_tier_zero_step() {
        let mut app = game(TierCaps { full: 0, reduced: 0 });
        let start = at(10.0, 20.0);
        let (e, ids) = actor(&mut app, 6, start, vec![go_to(30.0, 20.0)]);
        for _ in 0..(20 * HZ) {
            app.update();
            if status(&app, 6, ids[0]) == DirectiveStatus::Ended {
                break;
            }
            assert!(planar(position(&app, e), start) < 0.05, "the stored body walked");
        }
        assert_eq!(status(&app, 6, ids[0]), DirectiveStatus::Ended);
        assert!(stored(&app, 6));
        let ends: Vec<_> = app.world().resource::<Ends>().0.iter().map(|(d, _)| (d.actor, d.id, d.end)).collect();
        assert_eq!(ends, [(ActorKey(6), ids[0], DirectiveEnd::Completed)], "one end, the tier-0 step's");
        let drive = app.world().get::<DirectiveDrive>(e).unwrap();
        assert_eq!((drive.stats.goto_started, drive.stats.goto_finished), (0, 0), "drive_directives walked it");
        assert_eq!(status(&app, 6, ids[1]), DirectiveStatus::Serving, "the wander is the next top");
    }

    #[test]
    fn no_game_clock_no_tier_zero() {
        let mut app = app_with(|_| {});
        let (e, _) = actor(&mut app, 7, at(20.0, 20.0), Vec::new());
        app.world_mut().entity_mut(e).insert(Dormant);
        for _ in 0..10 {
            app.update();
        }
        assert!(app.world().resource::<AiLogical>().is_empty());
        assert!(!app.world().entity(e).contains::<AiLogicalBody>());
    }

    #[test]
    fn the_proof_line() {
        let line = proof_line(Tier0Counts { logical: 12, queued: 2, placed: 5, other_space: 1 });
        println!("{line}");
        assert_eq!(line, "tier0: 12 logical, 2 queued, 5 placed (1 other-space)");
    }
}
