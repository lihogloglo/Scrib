//! `portal_query` (NV11c): exact through-door answers on a synthetic three-space graph.
//!
//! Spaces: exterior `e`, interiors `a` and `b`, Y up, points in the XZ plane (heights for the
//! sight crossing). Links (identity frames, +Z into each end's space): `ea` (e origin - a
//! origin, seamless, 2 x 2 m opening), `ab` (a (3, 0, 4) - b origin, seamless) and, in the
//! trapdoor fixture, `eb` (e (60, 0, 80) - b (0, 0, 20), fade-only). Leg lengths are integer
//! (3-4-5 triangles), so distances compare exactly; mapped points through the half-turn carry
//! float noise and compare to 1e-5 m.
#![allow(clippy::float_cmp, reason = "exact distances on integer legs are the point")]

use std::cell::RefCell;
use std::f32::consts::FRAC_PI_2;

use glam::{IVec3, Quat, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_world::{DoorLink, LinkEnd, LinkId, Location, WorldspaceId};

use crate::{DoorState, LinkFlags, LinkSide, Los, LosProvider, PathRules, WorldGraph, map, map_direction};

fn pos(v: Vec3) -> WorldPos {
    WorldPos::new(IVec3::ZERO, v)
}

fn loc(ws: &str, x: f32, y: f32, z: f32) -> Location {
    Location::new(WorldspaceId::new(ws), pos(Vec3::new(x, y, z)))
}

fn end_at(ws: &str, at: Vec3, rotation: Quat, extent: Vec2) -> LinkEnd {
    let frame = WorldTransform { rotation, ..WorldTransform::from_pos(pos(at)) };
    LinkEnd { worldspace: WorldspaceId::new(ws), frame, arrival: frame, door: None, extent }
}

fn end(ws: &str, at: Vec3, extent: Vec2) -> LinkEnd {
    end_at(ws, at, Quat::IDENTITY, extent)
}

struct Fixture {
    links: Vec<DoorLink>,
    ea: LinkId,
    ab: LinkId,
    eb: LinkId,
}

fn fixture(trapdoor: bool) -> Fixture {
    let ea = DoorLink {
        a: end("e", Vec3::ZERO, Vec2::splat(2.0)),
        b: end("a", Vec3::ZERO, Vec2::splat(2.0)),
        two_way: true,
    };
    let ab = DoorLink {
        a: end("a", Vec3::new(3.0, 0.0, 4.0), Vec2::new(1.0, 2.0)),
        b: end("b", Vec3::ZERO, Vec2::new(1.0, 2.0)),
        two_way: true,
    };
    let eb = DoorLink {
        a: end("e", Vec3::new(60.0, 0.0, 80.0), Vec2::ZERO),
        b: end("b", Vec3::new(0.0, 0.0, 20.0), Vec2::ZERO),
        two_way: true,
    };
    let ids = Fixture { links: Vec::new(), ea: ea.id(), ab: ab.id(), eb: eb.id() };
    let mut links = vec![ea, ab];
    if trapdoor {
        links.push(eb);
    }
    Fixture { links, ..ids }
}

#[allow(clippy::unnecessary_wraps, reason = "the WorldGraph::build callback's signature")]
fn approach(end: &LinkEnd) -> Option<Location> {
    Some(Location::new(end.worldspace.clone(), end.frame.pos))
}

/// Links in `fade` are fade-only, the others seamless.
fn graph(f: &Fixture, fade: &[LinkId]) -> WorldGraph {
    WorldGraph::build(&f.links, approach, |l: &DoorLink| {
        if fade.contains(&l.id()) { LinkFlags::FADE_ONLY } else { LinkFlags::SEAMLESS }
    })
}

fn near(got: &Location, want: &Location) {
    assert_eq!(got.worldspace, want.worldspace);
    let off = got.pos.delta(want.pos).length();
    assert!(off < 1e-5, "{got:?} is {off} m from {want:?}");
}

// --- map ---

/// e (10, 0, 0) facing +Z, to a (100, 0, 50) turned a quarter (its +Z is world +X).
fn turned_link() -> DoorLink {
    DoorLink {
        a: end("e", Vec3::new(10.0, 0.0, 0.0), Vec2::splat(2.0)),
        b: end_at("a", Vec3::new(100.0, 0.0, 50.0), Quat::from_rotation_y(FRAC_PI_2), Vec2::splat(2.0)),
        two_way: true,
    }
}

#[test]
fn map_lands_behind_the_other_end() {
    let link = turned_link();
    // 1 m in front of a's end -> 1 m behind b's (b's front is +X).
    near(&map(&loc("e", 10.0, 0.0, 1.0), &link, LinkSide::A), &loc("a", 99.0, 0.0, 50.0));
    // Offsets across and up the opening keep their size: local (0.5, 0.2, 1) -> (-0.5, 0.2, -1).
    near(&map(&loc("e", 10.5, 0.2, 1.0), &link, LinkSide::A), &loc("a", 99.0, 0.2, 50.5));
    // And back.
    let there = map(&loc("e", 10.5, 0.2, 1.0), &link, LinkSide::A);
    near(&map(&there, &link, LinkSide::B), &loc("e", 10.5, 0.2, 1.0));
}

#[test]
fn map_direction_walks_on_through() {
    let link = turned_link();
    // Walking into a's door (-Z, out of e) comes out walking into a's space (b's +Z = +X).
    let out = map_direction(Vec3::NEG_Z, &link, LinkSide::A);
    assert!((out - Vec3::X).length() < 1e-6, "{out:?}");
    let back = map_direction(Vec3::NEG_X, &link, LinkSide::B);
    assert!((back - Vec3::Z).length() < 1e-6, "{back:?}");
}

// --- path_distance ---

fn rules(max_hops: usize, closed_door_m: f32, door_state: &dyn Fn(LinkId) -> DoorState) -> PathRules<'_> {
    PathRules { max_hops, closed_door_m, door_state }
}

fn open(_: LinkId) -> DoorState {
    DoorState::Open
}

#[test]
fn path_distance_same_space_is_straight() {
    let f = fixture(false);
    let g = graph(&f, &[]);
    let p =
        g.path_distance(&loc("e", 0.0, 0.0, 3.0), &loc("e", 4.0, 0.0, 0.0), &rules(0, 0.0, &open)).unwrap();
    assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (5.0, &[][..], 0));
}

#[test]
fn path_distance_across_one_door_sums_the_legs() {
    let f = fixture(false);
    let g = graph(&f, &[]);
    // 10 m to ea's opening in e, then 3 m from its opening in a.
    let p =
        g.path_distance(&loc("e", 6.0, 0.0, 8.0), &loc("a", 0.0, 0.0, 3.0), &rules(1, 0.0, &open)).unwrap();
    assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (13.0, &[f.ea][..], 0));
}

#[test]
fn path_distance_respects_max_hops_and_counts_closed_doors() {
    let f = fixture(false);
    let g = graph(&f, &[]);
    let ab = f.ab;
    let (from, to) = (loc("e", 6.0, 0.0, 8.0), loc("b", 0.0, 0.0, 5.0));
    for shut in [DoorState::Closed, DoorState::Locked] {
        let state = move |id: LinkId| if id == ab { shut } else { DoorState::Open };
        // 10 to ea, 5 across a to ab, 5 in b; the shut door still carries sound.
        let p = g.path_distance(&from, &to, &rules(2, 0.0, &state)).unwrap();
        assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (20.0, &[f.ea, f.ab][..], 1), "{shut:?}");
        assert_eq!(g.path_distance(&from, &to, &rules(1, 0.0, &state)), None, "two links, one hop");
        assert_eq!(g.path_distance(&from, &to, &rules(0, 0.0, &state)), None);
    }
}

#[test]
fn path_distance_weighs_closed_doors_and_hops() {
    let f = fixture(true);
    let g = graph(&f, &[f.eb]);
    let ab = f.ab;
    let state = move |id: LinkId| if id == ab { DoorState::Closed } else { DoorState::Open };
    let (from, to) = (loc("e", 6.0, 0.0, 8.0), loc("b", 0.0, 0.0, 5.0));
    // One hop: the fade-only trapdoor, 90 m to it plus 15 m in b.
    let p = g.path_distance(&from, &to, &rules(1, 0.0, &state)).unwrap();
    assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (105.0, &[f.eb][..], 0));
    // Two hops, distance only: the 20 m way through the closed door.
    let p = g.path_distance(&from, &to, &rules(2, 0.0, &state)).unwrap();
    assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (20.0, &[f.ea, f.ab][..], 1));
    // A closed door weighing 100 m (120 > 105): the open trapdoor wins, metres stay unweighted.
    let p = g.path_distance(&from, &to, &rules(2, 100.0, &state)).unwrap();
    assert_eq!((p.metres, p.links.as_slice(), p.closed_doors), (105.0, &[f.eb][..], 0));
}

// --- line_of_sight ---

/// Rays are clear except in the `blocked` spaces; records every ray.
#[derive(Default)]
struct FakeLos {
    blocked: Vec<WorldspaceId>,
    rays: RefCell<Vec<(Location, Location)>>,
}

impl FakeLos {
    fn blocking(ws: &str) -> Self {
        Self { blocked: vec![WorldspaceId::new(ws)], ..Self::default() }
    }

    fn rays(&self) -> Vec<(Location, Location)> {
        self.rays.borrow().clone()
    }
}

impl LosProvider for FakeLos {
    fn ray(&self, from: &Location, to: &Location) -> bool {
        assert_eq!(from.worldspace, to.worldspace, "rays stay in one space");
        self.rays.borrow_mut().push((from.clone(), to.clone()));
        !self.blocked.contains(&from.worldspace)
    }
}

#[test]
fn line_of_sight_same_space_is_one_ray() {
    let f = fixture(false);
    let g = graph(&f, &[]);
    let (a, b) = (loc("e", 0.0, 0.0, 3.0), loc("e", 4.0, 0.0, 0.0));
    let clear = FakeLos::default();
    assert_eq!(g.line_of_sight(&a, &b, &open, &clear), Los::Clear);
    assert_eq!(clear.rays(), vec![(a.clone(), b.clone())]);
    assert_eq!(g.line_of_sight(&a, &b, &open, &FakeLos::blocking("e")), Los::Blocked);
}

#[test]
fn line_of_sight_through_an_open_seamless_link_needs_both_rays() {
    let f = fixture(false);
    let g = graph(&f, &[]);
    // From a, through ea's opening, to e: the target seen through the portal is a (-0.5, 0.5, -2),
    // so the line crosses the door plane halfway, at (0, 0.25, 0).
    let (viewer, target) = (loc("a", 0.5, 0.0, 2.0), loc("e", 0.5, 0.5, 2.0));
    let clear = FakeLos::default();
    assert_eq!(g.line_of_sight(&viewer, &target, &open, &clear), Los::Clear);
    let rays = clear.rays();
    assert_eq!(rays.len(), 2);
    near(&rays[0].0, &viewer);
    near(&rays[0].1, &loc("a", 0.0, 0.25, 0.0));
    near(&rays[1].0, &loc("e", 0.0, 0.25, 0.0));
    near(&rays[1].1, &target);
    for ws in ["a", "e"] {
        assert_eq!(g.line_of_sight(&viewer, &target, &open, &FakeLos::blocking(ws)), Los::Blocked, "{ws}");
    }
    // Same answer from the other side.
    assert_eq!(g.line_of_sight(&target, &viewer, &open, &FakeLos::default()), Los::Clear);
}

#[test]
fn line_of_sight_blocked_by_shut_fade_or_missed_links() {
    let f = fixture(false);
    let (viewer, target) = (loc("a", 0.5, 0.0, 2.0), loc("e", 0.5, 0.5, 2.0));
    let g = graph(&f, &[]);
    for shut in [DoorState::Closed, DoorState::Locked] {
        let los = FakeLos::default();
        assert_eq!(g.line_of_sight(&viewer, &target, &move |_| shut, &los), Los::BlockedByLink, "{shut:?}");
        assert!(los.rays().is_empty(), "no ray through a shut door");
    }
    // Fade-only, even open.
    let faded = graph(&f, &[f.ea]);
    assert_eq!(faded.line_of_sight(&viewer, &target, &open, &FakeLos::default()), Los::BlockedByLink);
    // The line crosses the plane at x = 1.25, outside the 2 m opening; or the viewer stands behind it.
    assert_eq!(
        g.line_of_sight(&loc("a", 3.0, 0.0, 2.0), &target, &open, &FakeLos::default()),
        Los::BlockedByLink
    );
    assert_eq!(
        g.line_of_sight(&loc("a", 0.0, 0.0, -1.0), &target, &open, &FakeLos::default()),
        Los::BlockedByLink
    );
    // e and b share no link: sight crosses one link at most.
    assert_eq!(
        g.line_of_sight(&target, &loc("b", 0.0, 0.0, 1.0), &open, &FakeLos::default()),
        Los::BlockedByLink
    );
}
