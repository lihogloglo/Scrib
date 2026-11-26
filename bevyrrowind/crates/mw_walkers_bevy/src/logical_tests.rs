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
