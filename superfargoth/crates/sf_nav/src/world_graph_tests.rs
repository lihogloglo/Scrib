//! `WorldGraph` (NV11b): exact routes on a synthetic three-space graph.
//!
//! Spaces: exterior `e`, interiors `a` and `b`. Links: `ea` (e-a, seamless), `ab` (a-b,
//! seamless), `eb` (one-way b -> e, fade). Approach point = the end's frame position. All
//! distances are integer (3-4-5 triangles), so costs compare exactly.
#![allow(clippy::float_cmp, reason = "exact costs on integer distances are the point")]

use glam::{IVec3, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

use crate::{DoorState, LinkFlags, LinkKind, LinkSide, RouteRules, TraverseTimes, WorldGraph};

fn pos(x: f32, y: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, y, 0.0))
}

fn loc(ws: &str, x: f32, y: f32) -> Location {
    Location::new(WorldspaceId::new(ws), pos(x, y))
}

fn end(ws: &str, x: f32, y: f32) -> LinkEnd {
    let frame = WorldTransform::from_pos(pos(x, y));
    LinkEnd { worldspace: WorldspaceId::new(ws), frame, arrival: frame, door: None, extent: Vec2::ZERO }
}

fn link(a: LinkEnd, b: LinkEnd, two_way: bool) -> DoorLink {
    DoorLink { a, b, two_way }
}

struct Fixture {
    links: Vec<DoorLink>,
    ea: LinkId,
    ab: LinkId,
    eb: LinkId,
}

fn fixture() -> Fixture {
    let ea = link(end("e", 0.0, 0.0), end("a", 0.0, 0.0), true);
    let ab = link(end("a", 6.0, 8.0), end("b", 0.0, 0.0), true);
    // One-way b -> e: `a` is the b end.
    let eb = link(end("b", 80.0, 0.0), end("e", 0.0, 50.0), false);
    Fixture { ea: ea.id(), ab: ab.id(), eb: eb.id(), links: vec![ea, ab, eb] }
}

#[allow(clippy::unnecessary_wraps, reason = "the WorldGraph::build callback's signature")]
fn approach(end: &LinkEnd) -> Option<Location> {
    Some(Location::new(end.worldspace.clone(), end.frame.pos))
}

/// `eb` is a fade-only trapdoor, the others seamless.
fn graph(f: &Fixture) -> WorldGraph {
    let eb = f.eb;
    WorldGraph::build(&f.links, approach, move |l: &DoorLink| {
        if l.id() == eb { LinkFlags::FADE_ONLY } else { LinkFlags::SEAMLESS }
    })
}

/// 2 m/s, seamless 3 s, fade 1 s.
fn rules() -> RouteRules<'static> {
    RouteRules {
        traverse: TraverseTimes { seamless_s: 3.0, fade_s: 1.0 },
        ..RouteRules::walking(2.0)
    }
}

fn open(_: LinkId) -> DoorState {
    DoorState::Open
}

#[test]
fn build_indexes_every_exit() {
    let f = fixture();
    let g = graph(&f);
    assert_eq!((g.len(), g.skipped()), (3, 0));
    assert!(g.exit(f.ea, LinkSide::A).is_some() && g.exit(f.ea, LinkSide::B).is_some());
    assert!(g.exit(f.eb, LinkSide::A).is_some());
    assert!(g.exit(f.eb, LinkSide::B).is_none(), "one-way: no b exit");
    assert_eq!(g.exit(f.eb, LinkSide::A).unwrap().kind, LinkKind::Fade);
    assert_eq!(g.exit(f.ab, LinkSide::B).unwrap().depart, loc("b", 0.0, 0.0));
    assert_eq!(g.link(f.eb).unwrap().flags, LinkFlags::FADE_ONLY);
    let mut from_e: Vec<_> = g.exits_from(&WorldspaceId::new("e")).map(|x| x.link).collect();
    assert_eq!(from_e.len(), 1, "only ea leaves e (eb is one-way into e)");
    from_e.dedup();
    assert_eq!(from_e, vec![f.ea]);
}

#[test]
fn b_to_e_takes_the_cheaper_path_through_a() {
    let f = fixture();
    let g = graph(&f);
    let (from, to) = (loc("b", 40.0, 0.0), loc("e", 0.0, 10.0));
    let r = g.route(&from, &to, &open, &rules()).unwrap();
    // 40 m walk (20 s) + 3 + 10 m in a (5 s) + 3 + 10 m to the goal (5 s) = 36 s;
    // the direct trapdoor is 20 + 1 + 20 = 41 s.
    assert_eq!(r.links, vec![f.ab, f.ea]);
    assert_eq!(r.cost_s, 36.0);
    assert_eq!(r.final_walk_s, 5.0);
    let legs: Vec<_> = r.legs.iter().map(|l| (l.link, l.side, l.walk_s, l.traverse_s)).collect();
    assert_eq!(legs, vec![(f.ab, LinkSide::B, 20.0, 3.0), (f.ea, LinkSide::B, 5.0, 3.0)]);
    assert_eq!(r.legs[0].depart, loc("b", 0.0, 0.0));
    assert_eq!(r.legs[1].arrive, loc("e", 0.0, 0.0));
}

#[test]
fn locked_link_forces_the_other_path_unless_unlocked() {
    let f = fixture();
    let g = graph(&f);
    let (from, to) = (loc("b", 40.0, 0.0), loc("e", 0.0, 10.0));
    let ea = f.ea;
    let state = move |id: LinkId| if id == ea { DoorState::Locked } else { DoorState::Open };
    let r = g.route(&from, &to, &state, &rules()).unwrap();
    assert_eq!(r.links, vec![f.eb]);
    assert_eq!(r.cost_s, 41.0);
    assert_eq!(r.legs[0].kind, LinkKind::Fade);

    // A key for ea: back through a.
    let key = move |id: LinkId| id == ea;
    let with_key = RouteRules { unlock: &key, ..rules() };
    assert_eq!(g.route(&from, &to, &state, &with_key).unwrap().links, vec![f.ab, f.ea]);

    // Closed but unlocked is passable.
    let closed = |_: LinkId| DoorState::Closed;
    assert_eq!(g.route(&from, &to, &closed, &rules()).unwrap().cost_s, 36.0);
}

#[test]
fn one_way_link_refuses_the_wrong_direction() {
    let f = fixture();
    let g = graph(&f);
    // e -> b: eb only runs b -> e, so the route goes through a even with the goal at eb's b end.
    let (from, to) = (loc("e", 0.0, 10.0), loc("b", 80.0, 0.0));
    let r = g.route(&from, &to, &open, &rules()).unwrap();
    assert_eq!(r.links, vec![f.ea, f.ab]);
    // 5 + 3 + 5 + 3 + 80 m (40 s) = 56 s.
    assert_eq!(r.cost_s, 56.0);
    assert_eq!(r.legs.iter().map(|l| l.side).collect::<Vec<_>>(), vec![LinkSide::A, LinkSide::A]);

    // With ea locked, e has no way out.
    let ea = f.ea;
    let state = move |id: LinkId| if id == ea { DoorState::Locked } else { DoorState::Open };
    assert!(g.route(&from, &to, &state, &rules()).is_none());

    // The host's ONE_WAY flag does the same to a two-way record: ab only a -> b.
    let ab = f.ab;
    let g2 = WorldGraph::build(&f.links, approach, move |l: &DoorLink| {
        if l.id() == ab { LinkFlags::SEAMLESS | LinkFlags::ONE_WAY } else { LinkFlags::SEAMLESS }
    });
    assert!(g2.exit(f.ab, LinkSide::B).is_none());
    let back = g2.route(&loc("b", 40.0, 0.0), &loc("e", 0.0, 10.0), &open, &rules()).unwrap();
    assert_eq!(back.links, vec![f.eb]);
}

#[test]
fn same_space_route_is_empty() {
    let f = fixture();
    let g = graph(&f);
    let r = g.route(&loc("e", 0.0, 10.0), &loc("e", 0.0, 50.0), &open, &rules()).unwrap();
    assert!(r.links.is_empty() && r.legs.is_empty());
    assert_eq!((r.cost_s, r.final_walk_s), (20.0, 20.0));
}

#[test]
fn costs_scale_with_speed_and_traverse_times() {
    let f = fixture();
    let g = graph(&f);
    let (from, to) = (loc("b", 40.0, 0.0), loc("e", 0.0, 10.0));
    // 4 m/s, seamless 0.5 s: via a = 10 + 0.5 + 2.5 + 0.5 + 2.5 = 16; direct = 10 + 1 + 10 = 21.
    let fast = RouteRules {
        traverse: TraverseTimes { seamless_s: 0.5, fade_s: 1.0 },
        ..RouteRules::walking(4.0)
    };
    assert_eq!(g.route(&from, &to, &open, &fast).unwrap().cost_s, 16.0);
    // Seamless doors slow (20 s each): via a = 20 + 20 + 5 + 20 + 5 = 70 > direct 41.
    let slow_doors = RouteRules {
        traverse: TraverseTimes { seamless_s: 20.0, fade_s: 1.0 },
        ..RouteRules::walking(2.0)
    };
    let r = g.route(&from, &to, &open, &slow_doors).unwrap();
    assert_eq!((r.links, r.cost_s), (vec![f.eb], 41.0));
    // Unreachable space and a bad speed: no route.
    assert!(g.route(&from, &loc("z", 0.0, 0.0), &open, &rules()).is_none());
    assert!(g.route(&from, &to, &open, &RouteRules::walking(0.0)).is_none());
}

#[test]
fn equal_costs_take_the_lower_link_id() {
    // Two parallel e-a doors, mirror images around the start and the goal.
    let p = link(end("e", -5.0, 0.0), end("a", -5.0, 0.0), true);
    let q = link(end("e", 5.0, 0.0), end("a", 5.0, 0.0), true);
    let low = p.id().min(q.id());
    let flags = |_: &DoorLink| LinkFlags::SEAMLESS;
    let (from, to) = (loc("e", 0.0, 0.0), loc("a", 0.0, 0.0));
    for links in [vec![p.clone(), q.clone()], vec![q.clone(), p.clone()]] {
        let g = WorldGraph::build(&links, approach, flags);
        let r = g.route(&from, &to, &open, &rules()).unwrap();
        assert_eq!(r.links, vec![low]);
        assert_eq!(r.cost_s, 2.5 + 3.0 + 2.5);
    }
}

#[test]
fn unplaceable_and_duplicate_links_are_skipped() {
    let f = fixture();
    let mut links = f.links.clone();
    links.push(f.links[0].clone());
    let g = WorldGraph::build(&links, |e: &LinkEnd| {
        (e.worldspace.as_str() != "b").then(|| Location::new(e.worldspace.clone(), e.frame.pos))
    }, |_: &DoorLink| LinkFlags::SEAMLESS);
    // ab and eb have a b end without an approach point; the second ea is a duplicate.
    assert_eq!((g.len(), g.skipped()), (1, 3));
    assert!(g.link(f.ab).is_none());
}

#[test]
fn route_serde_round_trip() {
    let f = fixture();
    let g = graph(&f);
    let r = g.route(&loc("b", 40.0, 0.0), &loc("e", 0.0, 10.0), &open, &rules()).unwrap();
    let json = serde_json::to_string(&r).unwrap();
    assert_eq!(serde_json::from_str::<crate::WorldRoute>(&json).unwrap(), r);
    let bytes = postcard::to_allocvec(&r).unwrap();
    assert_eq!(postcard::from_bytes::<crate::WorldRoute>(&bytes).unwrap(), r);
}
