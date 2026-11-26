//! `RouteGraph` (NV12a): exact tests on synthetic graphs (nav.md "Route graph", its test list).
//!
//! Graphs lie in the XZ plane (y = 0 unless a test needs height); distances are integers or
//! 3-4-5 triangles where costs compare exactly. Node ids are hashes, so tests that depend on id
//! order compute it (`min`) rather than assume it.
#![allow(clippy::float_cmp, reason = "exact costs on integer distances are the point")]
#![allow(clippy::many_single_char_names, reason = "nodes named after the figure")]

use glam::DVec3;
use sf_coords::WorldPos;
use sf_world::{LinkId, Location, WorldspaceId};

use crate::route_graph::RouteLeg;
use crate::{
    AgentClassId, EdgeKind, EdgeKinds, LinkSide, RouteBuildRules, RouteEdgeId, RouteError,
    RouteGraph, RouteGraphBuilder, RouteKind, RouteNodeId, RouteNodeKind, RoutePath, TravelRules,
};

const SPEED: f64 = 2.0;

fn ws() -> WorldspaceId {
    WorldspaceId::new("w")
}

fn p3(x: f64, y: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, y, z))
}

fn p(x: f64, z: f64) -> WorldPos {
    p3(x, 0.0, z)
}

fn loc(x: f64, z: f64) -> Location {
    Location::new(ws(), p(x, z))
}

fn rules() -> TravelRules {
    TravelRules::new(SPEED, AgentClassId(0))
}

fn rules_join(join_max_m: f64) -> TravelRules {
    TravelRules { join_max_m, ..rules() }
}

fn exact() -> RouteBuildRules {
    RouteBuildRules { simplify_m: 0.0, ..RouteBuildRules::default() }
}

fn close(a: WorldPos, b: WorldPos, tol: f64) -> bool {
    a.delta(b).length() <= tol
}

fn anchor(b: &mut RouteGraphBuilder, x: f64, z: f64, tag: &str) -> RouteNodeId {
    b.add_node(p(x, z), RouteNodeKind::Anchor(tag.into()))
}

fn end(b: &mut RouteGraphBuilder, x: f64, z: f64) -> RouteNodeId {
    b.add_node(p(x, z), RouteNodeKind::End)
}

fn road(b: &mut RouteGraphBuilder, a: RouteNodeId, c: RouteNodeId, cost: Option<f32>) {
    b.add_edge(a, c, EdgeKind::Road, true, &[], cost);
}

/// The nodes a path passes, in order (start and end of every edge leg that reaches one).
fn node_seq(g: &RouteGraph, path: &RoutePath) -> Vec<RouteNodeId> {
    let mut out = Vec::new();
    for leg in &path.legs {
        if let RouteLeg::Edge { edge, from_m, to_m, .. } = *leg {
            let e = &g.edges[g.edge_index(edge).unwrap()];
            let (len, (a, b)) = (e.length_m, (g.nodes[e.a as usize].id, g.nodes[e.b as usize].id));
            let at = |m: f32| if m == 0.0 { Some(a) } else if m == len { Some(b) } else { None };
            for n in [at(from_m), at(to_m)].into_iter().flatten() {
                if out.last() != Some(&n) {
                    out.push(n);
                }
            }
        }
    }
    out
}

fn leg_sum(path: &RoutePath) -> f64 {
    path.legs.iter().map(RouteLeg::cost_m).sum()
}

// ---- route ----------------------------------------------------------------------------------

/// Square A(0,0) B(10,0) C(10,10) D(0,10) + diagonal A-C; anchors so nothing collapses.
fn square(ab: Option<f32>, diag: Option<f32>) -> (RouteGraph, [RouteNodeId; 4]) {
    let mut b = RouteGraphBuilder::new(ws());
    let n = [anchor(&mut b, 0.0, 0.0, "A"), anchor(&mut b, 10.0, 0.0, "B"), anchor(&mut b, 10.0, 10.0, "C"), anchor(&mut b, 0.0, 10.0, "D")];
    road(&mut b, n[0], n[1], ab);
    road(&mut b, n[1], n[2], None);
    road(&mut b, n[2], n[3], None);
    road(&mut b, n[3], n[0], None);
    road(&mut b, n[0], n[2], diag);
    (b.build(&exact()).0, n)
}

#[test]
fn shortest_route_on_square_plus_diagonal() {
    // Via B 11 + 10 = 21, diagonal 21, via D 10 + 10 = 20.
    let (g, [a, _, c, d]) = square(Some(11.0), Some(21.0));
    let r = rules_join(1.0);
    let path = g.route(&loc(0.0, 0.0), &loc(10.0, 10.0), &r).unwrap();
    assert_eq!(path.kind, RouteKind::Graph);
    assert_eq!(node_seq(&g, &path), vec![a, d, c]);
    assert!((path.cost_m - 20.0).abs() < 1e-9);
    assert!((leg_sum(&path) - path.cost_m).abs() < 1e-9);
    assert!((path.duration_s() - 10.0).abs() < 1e-9);

    // A cheap measured diagonal wins.
    let (g, [a, _, c, _]) = square(Some(11.0), Some(15.0));
    let path = g.route(&loc(0.0, 0.0), &loc(10.0, 10.0), &r).unwrap();
    assert_eq!(node_seq(&g, &path), vec![a, c]);
    assert!((path.cost_m - 15.0).abs() < 1e-9);
}

/// T(-3,-4) - S(0,0); S - X(10,0) - G(10,10); S - Y(0,10) - G; G - H(13,13). X and Y tie exactly
/// (same g, same straight distance to H).
fn tie_graph(reverse_input: bool) -> (RouteGraph, [RouteNodeId; 6]) {
    let mut b = RouteGraphBuilder::new(ws());
    let spec = [(-3.0, -4.0, "T"), (0.0, 0.0, "S"), (10.0, 0.0, "X"), (0.0, 10.0, "Y"), (10.0, 10.0, "G"), (13.0, 13.0, "H")];
    let edges = [(0, 1), (1, 2), (2, 4), (1, 3), (3, 4), (4, 5)];
    let mut ids = [RouteNodeId(0); 6];
    let order: Vec<usize> = if reverse_input { (0..6).rev().collect() } else { (0..6).collect() };
    for &i in &order {
        let (x, z, tag) = spec[i];
        ids[i] = if tag == "X" || tag == "Y" { anchor(&mut b, x, z, tag) } else { end(&mut b, x, z) };
    }
    let mut es: Vec<(usize, usize)> = edges.to_vec();
    if reverse_input {
        es.reverse();
        es = es.into_iter().map(|(a, c)| (c, a)).collect();
    }
    for (a, c) in es {
        road(&mut b, ids[a], ids[c], None);
    }
    (b.build(&exact()).0, ids)
}

#[test]
fn equal_cost_tie_takes_lower_node_id_and_ignores_input_order() {
    let (g, [t, s, x, y, gn, h]) = tie_graph(false);
    let r = rules_join(1.0);
    let path = g.route(&loc(-3.0, -4.0), &loc(13.0, 13.0), &r).unwrap();
    assert_eq!(node_seq(&g, &path), vec![t, s, x.min(y), gn, h]);
    assert!((path.cost_m - (25.0 + 18.0_f64.sqrt())).abs() < 1e-5);

    let (g2, _) = tie_graph(true);
    assert_eq!(g.encode().unwrap(), g2.encode().unwrap());
    assert_eq!(g2.route(&loc(-3.0, -4.0), &loc(13.0, 13.0), &r).unwrap(), path);
}

#[test]
fn unreachable_across_components() {
    let mut b = RouteGraphBuilder::new(ws());
    let (a, c) = (end(&mut b, 0.0, 0.0), end(&mut b, 10.0, 0.0));
    road(&mut b, a, c, None);
    let (d, e) = (end(&mut b, 1000.0, 0.0), end(&mut b, 1010.0, 0.0));
    road(&mut b, d, e, None);
    let g = b.build(&exact()).0;
    assert_eq!(g.route(&loc(0.0, 0.0), &loc(1010.0, 0.0), &rules()), Err(RouteError::Unreachable));
    let other = Location::new(WorldspaceId::new("elsewhere"), p(0.0, 0.0));
    assert_eq!(g.route(&other, &loc(0.0, 0.0), &rules()), Err(RouteError::WrongSpace));
}

/// A(0,0) - B(100,0) anchor - C(100,100): two 100 m edges.
fn ell() -> RouteGraph {
    let mut b = RouteGraphBuilder::new(ws());
    let (a, bb, c) = (end(&mut b, 0.0, 0.0), anchor(&mut b, 100.0, 0.0, "B"), end(&mut b, 100.0, 100.0));
    road(&mut b, a, bb, None);
    road(&mut b, bb, c, None);
    b.build(&exact()).0
}

#[test]
fn off_graph_ends_join_with_straight_and_partial_edge_legs() {
    let g = ell();
    let r = rules_join(50.0); // no Direct: the ends are 99 m apart
    let (from, to) = (loc(20.0, 5.0), loc(103.0, 60.0));
    let path = g.route(&from, &to, &r).unwrap();
    assert_eq!(path.kind, RouteKind::Graph);
    assert_eq!(path.legs.len(), 4);
    let RouteLeg::Straight { a, b, cost_m } = path.legs[0] else { panic!("start join") };
    assert_eq!(a, from.pos);
    assert!((cost_m - 5.0 * 1.5).abs() < 1e-9);
    assert!(close(b, p(20.0, 0.0), 1e-5));
    let pieces: Vec<(f32, f64)> = path.legs[1..3]
        .iter()
        .map(|l| match *l {
            RouteLeg::Edge { from_m, to_m, cost_m, .. } => ((to_m - from_m).abs(), cost_m),
            RouteLeg::Straight { .. } => panic!("edge leg"),
        })
        .collect();
    assert_eq!(pieces, vec![(80.0, 80.0), (60.0, 60.0)]);
    let RouteLeg::Straight { b, cost_m, .. } = path.legs[3] else { panic!("goal join") };
    assert_eq!(b, to.pos);
    assert!((cost_m - 3.0 * 1.5).abs() < 1e-9);
    assert!((path.cost_m - 152.0).abs() < 1e-9);
    assert!((leg_sum(&path) - path.cost_m).abs() < 1e-9);

    // Past join_max_m.
    assert_eq!(g.route(&loc(20.0, 60.0), &to, &r), Err(RouteError::NoJoin(crate::RouteEnd::From)));
    assert_eq!(g.route(&from, &loc(160.0, 60.0), &r), Err(RouteError::NoJoin(crate::RouteEnd::To)));
}

#[test]
fn direct_wins_when_cheaper() {
    let g = ell();
    let (from, to) = (loc(20.0, 5.0), loc(60.0, 10.0));
    // Graph 7.5 + 40 + 15 = 62.5; straight 40.31 m × 1.5 = 60.47.
    let path = g.route(&from, &to, &rules()).unwrap();
    assert_eq!(path.kind, RouteKind::Direct);
    let want = 1625.0_f64.sqrt() * 1.5;
    assert_eq!(path.legs, vec![RouteLeg::Straight { a: from.pos, b: to.pos, cost_m: want }]);
    assert_eq!(path.cost_m, want);
    // No join at either end but the ends are close: Direct too.
    let path = g.route(&loc(20.0, 400.0), &loc(20.0, 420.0), &rules()).unwrap();
    assert_eq!(path.kind, RouteKind::Direct);
}

#[test]
fn same_edge_forward_reverse_and_one_way() {
    let build = |two_way: bool| {
        let mut b = RouteGraphBuilder::new(ws());
        let (a, c) = (end(&mut b, 0.0, 0.0), end(&mut b, 100.0, 0.0));
        b.add_edge(a, c, EdgeKind::Road, two_way, &[], None);
        b.build(&exact()).0
    };
    let r = rules_join(30.0); // ends 50 m apart: no Direct
    let one = |g: &RouteGraph, x0: f64, x1: f64| {
        let path = g.route(&loc(x0, 0.0), &loc(x1, 0.0), &r)?;
        let [RouteLeg::Edge { from_m, to_m, cost_m, .. }] = path.legs[..] else { panic!("one edge leg") };
        assert_eq!(((to_m - from_m).abs(), cost_m, path.cost_m), (50.0, 50.0, 50.0));
        Ok::<_, RouteError>(())
    };
    let two = build(true);
    one(&two, 20.0, 70.0).unwrap();
    one(&two, 70.0, 20.0).unwrap();
    let oneway = build(false);
    one(&oneway, 20.0, 70.0).unwrap();
    assert_eq!(one(&oneway, 70.0, 20.0), Err(RouteError::Unreachable));
    // With Direct allowed the reverse walk goes straight.
    let path = oneway.route(&loc(70.0, 0.0), &loc(20.0, 0.0), &rules()).unwrap();
    assert_eq!((path.kind, path.cost_m), (RouteKind::Direct, 75.0));
}

#[test]
fn budget_at_tiny_max_expansions() {
    let g = grid(6, 10.0);
    let r = TravelRules { max_expansions: 3, ..rules_join(1.0) };
    assert_eq!(g.route(&loc(0.0, 0.0), &loc(50.0, 50.0), &r), Err(RouteError::Budget));
    assert!(g.route(&loc(0.0, 0.0), &loc(50.0, 50.0), &rules_join(1.0)).is_ok());
}

// ---- position_at ----------------------------------------------------------------------------

#[test]
fn position_at_ends_mid_polyline_and_pacing() {
    let g = ell();
    let path = g.route(&loc(20.0, 5.0), &loc(103.0, 60.0), &rules_join(50.0)).unwrap();
    let at = |cost_m: f64| g.position_at(&path, cost_m / SPEED);

    let start = g.position_at(&path, -1.0);
    assert_eq!((start.loc.pos, start.leg, start.done), (p(20.0, 5.0), 0, false));
    assert_eq!(start.heading, glam::Vec2::new(0.0, -1.0));
    let last = g.position_at(&path, path.duration_s());
    assert_eq!((last.loc.pos, last.leg, last.done), (p(103.0, 60.0), 3, true));
    assert!(g.position_at(&path, 1e9).done);

    // 30 m into the first edge, then 25 m into the second.
    let mid = at(7.5 + 30.0);
    assert!(close(mid.loc.pos, p(50.0, 0.0), 1e-5), "{:?}", mid.loc.pos);
    assert_eq!((mid.leg, mid.heading), (1, glam::Vec2::new(1.0, 0.0)));
    let mid = at(7.5 + 80.0 + 25.0);
    assert!(close(mid.loc.pos, p(100.0, 25.0), 1e-5));
    assert_eq!((mid.leg, mid.heading), (2, glam::Vec2::new(0.0, 1.0)));

    // One polyline edge P(0,0) (30,0) (30,40) Q(60,40): 30 + 40 + 30 m.
    let mut b = RouteGraphBuilder::new(ws());
    let (pn, qn) = (end(&mut b, 0.0, 0.0), end(&mut b, 60.0, 40.0));
    b.add_edge(pn, qn, EdgeKind::Path, true, &[p(30.0, 0.0), p(30.0, 40.0)], None);
    let g = b.build(&exact()).0;
    let path = g.route(&loc(0.0, 0.0), &loc(60.0, 40.0), &rules_join(10.0)).unwrap();
    assert_eq!((path.legs.len(), path.cost_m), (1, 100.0));
    let mid = g.position_at(&path, 50.0 / SPEED);
    assert!(close(mid.loc.pos, p(30.0, 20.0), 1e-5), "{:?}", mid.loc.pos);
    assert_eq!(mid.heading, glam::Vec2::new(0.0, 1.0));
    assert!((mid.heading.length() - 1.0).abs() < 1e-6);

    // Measured cost 200 on a 100 m edge: uniform pacing, half the time = half the edge.
    let mut b = RouteGraphBuilder::new(ws());
    let (a, c) = (end(&mut b, 0.0, 0.0), end(&mut b, 100.0, 0.0));
    road(&mut b, a, c, Some(200.0));
    let g = b.build(&exact()).0;
    let path = g.route(&loc(0.0, 0.0), &loc(100.0, 0.0), &rules_join(10.0)).unwrap();
    assert_eq!(path.cost_m, 200.0);
    for (cost, x) in [(50.0, 25.0), (100.0, 50.0), (150.0, 75.0)] {
        let pose = g.position_at(&path, cost / SPEED);
        assert!(close(pose.loc.pos, p(x, 0.0), 1e-5), "{cost}: {:?}", pose.loc.pos);
    }
}

// ---- nearest_point --------------------------------------------------------------------------

#[test]
fn nearest_point_projection_ties_reach_and_kinds() {
    let mut b = RouteGraphBuilder::new(ws());
    let (a, c) = (end(&mut b, 0.0, 5.0), end(&mut b, 100.0, 5.0));
    b.add_edge(a, c, EdgeKind::Road, true, &[], None);
    let (d, e) = (end(&mut b, 0.0, -5.0), end(&mut b, 100.0, -5.0));
    b.add_edge(d, e, EdgeKind::Path, true, &[], None);
    let g = b.build(&exact()).0;
    let r = rules();

    // Projection in 3D: (30, 4, 8) is 4 above and 3 beside (30, 0, 5): 5 m.
    let at = Location::new(ws(), p3(30.0, 4.0, 8.0));
    let hit = g.nearest_point(&at, &r, 100.0).unwrap();
    assert!(close(hit.pos, p(30.0, 5.0), 1e-5));
    assert!((hit.dist_m - 5.0).abs() < 1e-5);
    let e = &g.edges[g.edge_index(hit.edge).unwrap()];
    assert_eq!(e.kind, EdgeKind::Road);
    let along = if g.nodes[e.a as usize].pos == p(0.0, 5.0) { 30.0 } else { 70.0 };
    assert_eq!(hit.along_m, along);

    // Equidistant: the lower edge id.
    let hit = g.nearest_point(&loc(50.0, 0.0), &r, 100.0).unwrap();
    let low = g.edges.iter().map(|e| e.id).min().unwrap();
    assert_eq!((hit.edge, hit.dist_m), (low, 5.0));
    // Past max_m.
    assert_eq!(g.nearest_point(&loc(50.0, 0.0), &r, 4.9), None);
    // Kind filter.
    let paths = TravelRules { kinds: EdgeKinds::PATH, ..rules() };
    let hit = g.nearest_point(&loc(50.0, 4.0), &paths, 100.0).unwrap();
    assert_eq!((g.edges[g.edge_index(hit.edge).unwrap()].kind, hit.dist_m), (EdgeKind::Path, 9.0));
    // Class filter.
    let other = TravelRules { class: AgentClassId(3), ..rules() };
    assert!(g.nearest_point(&loc(50.0, 0.0), &other, 100.0).is_some()); // edges default to all classes
}

/// splitmix64.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    #[allow(clippy::cast_precision_loss)]
    fn f(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }
}

/// `n` random polylines in a 2 km square, with height and 0-3 interior points.
fn random_graph(seed: u64, n: usize) -> RouteGraphBuilder {
    let mut rng = Rng(seed);
    let mut b = RouteGraphBuilder::new(ws());
    for _ in 0..n {
        let pt = |rng: &mut Rng| p3(rng.f(0.0, 2000.0), rng.f(-20.0, 20.0), rng.f(0.0, 2000.0));
        let (a, c) = (pt(&mut rng), pt(&mut rng));
        let interior: Vec<WorldPos> = (0..rng.next() % 4).map(|_| pt(&mut rng)).collect();
        let kind = if rng.next().is_multiple_of(2) { EdgeKind::Road } else { EdgeKind::Path };
        let (na, nc) = (b.add_node(a, RouteNodeKind::End), b.add_node(c, RouteNodeKind::End));
        b.add_edge(na, nc, kind, !rng.next().is_multiple_of(3), &interior, None);
    }
    b
}

#[test]
fn nearest_point_equals_linear_scan() {
    let mut g = random_graph(7, 150).build(&RouteBuildRules::default()).0;
    let mut rng = Rng(99);
    for cell in [None, Some(17.0), Some(500.0)] {
        if let Some(c) = cell {
            g.set_grid_cell(c);
        }
        for i in 0..1000_u32 {
            let at = Location::new(ws(), p3(rng.f(-200.0, 2200.0), rng.f(-30.0, 30.0), rng.f(-200.0, 2200.0)));
            let r = if i.is_multiple_of(3) { TravelRules { kinds: EdgeKinds::ROAD, ..rules() } } else { rules() };
            let max_m = if i.is_multiple_of(2) { 120.0 } else { f64::INFINITY };
            assert_eq!(g.nearest_point(&at, &r, max_m), g.nearest_linear(&at, &r, max_m), "cell {cell:?} query {i}");
        }
    }
}

// ---- builder --------------------------------------------------------------------------------

#[test]
fn builder_collapses_chains_keeping_anchors_and_doors() {
    // A - n1 - D(door) - n2 - K(anchor) - n3 - B, all roads, then B -path- n4 -path- n5 -road- E.
    let mut b = RouteGraphBuilder::new(ws());
    let a = end(&mut b, 0.0, 0.0);
    let n1 = end(&mut b, 10.0, 1.0);
    let d = b.add_door(LinkId(7), LinkSide::A, p(20.0, 0.0));
    let n2 = end(&mut b, 30.0, 1.0);
    let k = anchor(&mut b, 40.0, 0.0, "town");
    let n3 = end(&mut b, 50.0, 1.0);
    let bb = end(&mut b, 60.0, 0.0);
    let n4 = end(&mut b, 70.0, 1.0);
    let n5 = end(&mut b, 80.0, 0.0);
    let e = end(&mut b, 90.0, 1.0);
    for (x, y) in [(a, n1), (n1, d), (d, n2), (n2, k), (k, n3), (n3, bb)] {
        road(&mut b, x, y, None);
    }
    b.add_edge(bb, n4, EdgeKind::Path, true, &[], None);
    b.add_edge(n4, n5, EdgeKind::Path, true, &[], None);
    road(&mut b, n5, e, None);
    let (g, stats) = b.build(&exact());
    // n1, n2, n3 and n4 collapse; B and n5 sit between two kinds.
    assert_eq!(stats.collapsed_nodes, 4);
    assert_eq!(g.nodes.len(), 6);
    assert_eq!(g.edges.len(), 5);
    let kind = |id: RouteNodeId| g.nodes[g.node_index(id).unwrap()].kind.clone();
    assert_eq!(kind(d), RouteNodeKind::Door { link: LinkId(7), side: LinkSide::A });
    assert_eq!(kind(k), RouteNodeKind::Anchor("town".into()));
    assert_eq!((kind(bb), kind(n5)), (RouteNodeKind::Junction, RouteNodeKind::Junction));
    assert_eq!(kind(a), RouteNodeKind::End);
    assert!(g.node_index(n1).is_none());
    assert_eq!(g.anchor("town"), Some(k));
    assert_eq!(g.anchor("nowhere"), None);
    // B - n4 - n5 is one polyline of 3 points, like each collapsed road.
    let pe = g.edges.iter().find(|e| e.kind == EdgeKind::Path).unwrap();
    assert_eq!(pe.points.len(), 3);
    assert_eq!(g.edges.iter().filter(|e| e.points.len() == 3).count(), 4);
    let costs: f32 = g.edges.iter().map(|e| e.cost_m).sum();
    let lens: f32 = g.edges.iter().map(|e| e.length_m).sum();
    assert_eq!(costs, lens);

    // Douglas-Peucker drops a collinear point (and keeps the bends).
    let mut b = RouteGraphBuilder::new(ws());
    let (a, c) = (end(&mut b, 0.0, 0.0), end(&mut b, 20.0, 10.0));
    b.add_edge(a, c, EdgeKind::Road, true, &[p(5.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)], None);
    let (g, stats) = b.build(&RouteBuildRules::default());
    assert_eq!((stats.simplified_points, g.points.len()), (1, 4));
}

#[test]
fn builder_merges_clamps_splits_and_joins_doors() {
    let mut b = RouteGraphBuilder::new(ws());
    let a = end(&mut b, 0.0, 0.0);
    let a2 = end(&mut b, 0.002, 0.0); // same 1 cm key
    assert_eq!(a, a2);
    let c = end(&mut b, 100.0, 0.0);
    road(&mut b, a, c, None);
    b.add_edge(c, a, EdgeKind::Road, true, &[], None); // the same two-way edge, reversed
    let unknown = RouteNodeId(1);
    road(&mut b, a, unknown, None);
    let near = b.add_door(LinkId(9), LinkSide::B, p(40.0, 20.0));
    let far = b.add_door(LinkId(10), LinkSide::A, p(40.0, 100.0));
    // A short edge with too low a measured cost, far away.
    let (s0, s1) = (end(&mut b, 0.0, 500.0), end(&mut b, 10.0, 500.0));
    road(&mut b, s0, s1, Some(4.0));
    // A 10 km edge.
    let (l0, l1) = (end(&mut b, 0.0, 1000.0), end(&mut b, 10_000.0, 1000.0));
    road(&mut b, l0, l1, None);
    let (g, stats) = b.build(&exact());

    assert_eq!((stats.merged_nodes, stats.merged_edges, stats.dropped_edges), (1, 1, 1));
    assert_eq!(stats.clamped_costs, 1);
    let short = g.edges.iter().find(|e| g.nodes[e.a as usize].pos.delta(p(5.0, 500.0)).length() < 6.0).unwrap();
    assert_eq!((short.length_m, short.cost_m), (10.0, 10.0));
    assert_eq!((stats.doors_joined, stats.doors_isolated), (1, 1));
    assert!(g.edges.iter().all(|e| e.length_m <= crate::MAX_EDGE_M));
    assert_eq!(stats.split_long, 1);
    assert_eq!(g.edges.iter().filter(|e| g.nodes[e.a as usize].pos.delta(p(5000.0, 1000.0)).length() < 5001.0).count(), 3);

    // The door joins (40, 0) by a 20 m connector; the road is split there.
    let join = RouteNodeId::at(&ws(), p(40.0, 0.0));
    let ji = g.node_index(join).unwrap();
    assert_eq!(g.nodes[ji].kind, RouteNodeKind::Junction);
    let at_join: Vec<_> = g.edges.iter().filter(|e| e.a as usize == ji || e.b as usize == ji).collect();
    assert_eq!(at_join.len(), 3);
    let conn = at_join.iter().find(|e| e.kind == EdgeKind::Connector).unwrap();
    let ni = g.node_index(near).unwrap();
    assert!(conn.a as usize == ni || conn.b as usize == ni);
    assert_eq!((conn.length_m, conn.two_way), (20.0, true));
    assert!(g.node_index(far).is_some()); // isolated, still a node
    // Route from the door to the far end of the road: 20 + 60.
    let door_loc = Location::new(ws(), p(40.0, 20.0));
    let path = g.route(&door_loc, &loc(100.0, 0.0), &rules_join(1.0)).unwrap();
    assert!((path.cost_m - 80.0).abs() < 1e-4);
}

#[test]
fn golden_ids_and_shuffled_input_bytes() {
    // Golden: the id hash is a save format (changing it changes every saved route).
    assert_eq!(format!("{}", RouteNodeId::at(&ws(), p3(1.0, 2.0, 3.0))), GOLDEN_NODE);
    assert_eq!(format!("{}", RouteNodeId::door(&ws(), LinkId(42), LinkSide::B)), GOLDEN_DOOR);
    // A door id ignores its position; a position within the 1 cm quantum keeps the id.
    let mut b = RouteGraphBuilder::new(ws());
    assert_eq!(b.add_door(LinkId(42), LinkSide::B, p(5.0, 5.0)), RouteNodeId::door(&ws(), LinkId(42), LinkSide::B));
    assert_eq!(RouteNodeId::at(&ws(), p3(1.001, 2.0, 3.0)), RouteNodeId::at(&ws(), p3(1.0, 2.0, 3.0)));
    assert_ne!(RouteNodeId::at(&WorldspaceId::new("v"), p3(1.0, 2.0, 3.0)), RouteNodeId::at(&ws(), p3(1.0, 2.0, 3.0)));

    // Shuffled input (reversed node and edge order) = identical bytes.
    let forward = random_graph(3, 80);
    let bytes = forward.clone().build(&RouteBuildRules::default()).0.encode().unwrap();
    let mut rev = RouteGraphBuilder::new(ws());
    let mut rng = Rng(3);
    let mut items = Vec::new();
    for _ in 0..80 {
        let pt = |rng: &mut Rng| p3(rng.f(0.0, 2000.0), rng.f(-20.0, 20.0), rng.f(0.0, 2000.0));
        let (a, c) = (pt(&mut rng), pt(&mut rng));
        let interior: Vec<WorldPos> = (0..rng.next() % 4).map(|_| pt(&mut rng)).collect();
        let kind = if rng.next().is_multiple_of(2) { EdgeKind::Road } else { EdgeKind::Path };
        items.push((a, c, interior, kind, !rng.next().is_multiple_of(3)));
    }
    for (a, c, interior, kind, two_way) in items.into_iter().rev() {
        let (nc, na) = (rev.add_node(c, RouteNodeKind::End), rev.add_node(a, RouteNodeKind::End));
        if two_way {
            let back: Vec<WorldPos> = interior.iter().rev().copied().collect();
            rev.add_edge(nc, na, kind, true, &back, None);
        } else {
            rev.add_edge(na, nc, kind, false, &interior, None);
        }
    }
    assert_eq!(rev.build(&RouteBuildRules::default()).0.encode().unwrap(), bytes);
}

const GOLDEN_NODE: &str = "aa2974fcb8f728e6";
const GOLDEN_DOOR: &str = "6a81f80dafab55dd";

#[test]
fn postcard_and_json_round_trips() {
    let (g, _) = tie_graph(false);
    let bytes = g.encode().unwrap();
    let back = RouteGraph::decode(&bytes).unwrap().unwrap();
    assert_eq!(back, g);
    let json = serde_json::to_string(&g).unwrap();
    let back_json: RouteGraph = serde_json::from_str(&json).unwrap();
    assert_eq!(back_json, g);
    // A decoded graph builds its index on first use and answers the same.
    let r = rules_join(1.0);
    let path = g.route(&loc(-3.0, -4.0), &loc(13.0, 13.0), &r).unwrap();
    assert_eq!(back.route(&loc(-3.0, -4.0), &loc(13.0, 13.0), &r).unwrap(), path);
    assert_eq!(back_json.route(&loc(-3.0, -4.0), &loc(13.0, 13.0), &r).unwrap(), path);
    // Another format is a rebuild, not an error.
    let mut other = bytes.clone();
    other[0] = other[0].wrapping_add(1);
    assert_eq!(RouteGraph::decode(&other).unwrap(), None);

    let bin: RoutePath = postcard::from_bytes(&postcard::to_allocvec(&path).unwrap()).unwrap();
    assert_eq!(bin, path);
    let txt: RoutePath = serde_json::from_str(&serde_json::to_string(&path).unwrap()).unwrap();
    assert_eq!(txt, path);
    let id: RouteEdgeId = serde_json::from_str(&serde_json::to_string(&g.edges[0].id).unwrap()).unwrap();
    assert_eq!(id, g.edges[0].id);
}

// ---- measurement ----------------------------------------------------------------------------

/// An `n` × `n` grid of roads, `step` metres apart, from (0, 0).
fn grid(n: usize, step: f64) -> RouteGraph {
    let mut b = RouteGraphBuilder::new(ws());
    #[allow(clippy::cast_precision_loss)]
    let at = |i: usize, j: usize| p(i as f64 * step, j as f64 * step);
    let ids: Vec<Vec<RouteNodeId>> =
        (0..n).map(|i| (0..n).map(|j| b.add_node(at(i, j), RouteNodeKind::Junction)).collect()).collect();
    for i in 0..n {
        for j in 0..n {
            if i + 1 < n {
                road(&mut b, ids[i][j], ids[i + 1][j], None);
            }
            if j + 1 < n {
                road(&mut b, ids[i][j], ids[i][j + 1], None);
            }
        }
    }
    b.build(&exact()).0
}

#[test]
#[ignore = "measurement: 50k-node grid route / nearest_point / position_at µs"]
fn measure_50k_grid() {
    use std::time::Instant;
    let (n, step) = (224, 50.0);
    let t0 = Instant::now();
    let g = grid(n, step);
    println!("build {} nodes {} edges: {:.0} ms", g.nodes.len(), g.edges.len(), t0.elapsed().as_secs_f64() * 1e3);
    #[allow(clippy::cast_precision_loss)]
    let side = (n - 1) as f64 * step;
    let mut rng = Rng(5);
    let r = TravelRules { join_max_m: 100.0, ..rules() };
    let _ = g.nearest_point(&loc(1.0, 1.0), &r, 250.0); // build the index
    let routes = 20_u32;
    let mut paths = Vec::new();
    let t0 = Instant::now();
    for i in 0..routes {
        let (from, to) = if i == 0 {
            (loc(0.0, 0.0), loc(side, side))
        } else {
            (loc(rng.f(0.0, side), rng.f(0.0, side)), loc(rng.f(0.0, side), rng.f(0.0, side)))
        };
        paths.push(g.route(&from, &to, &r).unwrap());
    }
    #[allow(clippy::cast_precision_loss)]
    let per_route = t0.elapsed().as_secs_f64() * 1e6 / f64::from(routes);
    let t0 = Instant::now();
    let corner = g.route(&loc(0.0, 0.0), &loc(side, side), &r).unwrap();
    let corner_us = t0.elapsed().as_secs_f64() * 1e6;
    let queries = 20_000;
    let pts: Vec<Location> = (0..queries).map(|_| loc(rng.f(0.0, side), rng.f(0.0, side))).collect();
    let t0 = Instant::now();
    let mut hits = 0;
    for at in &pts {
        hits += usize::from(g.nearest_point(at, &r, 250.0).is_some());
    }
    #[allow(clippy::cast_precision_loss)]
    let per_nearest = t0.elapsed().as_secs_f64() * 1e6 / queries as f64;
    let t0 = Instant::now();
    let mut done = 0;
    for i in 0..queries {
        let path = &paths[i % paths.len()];
        #[allow(clippy::cast_precision_loss)]
        let t = path.duration_s() * (i % 97) as f64 / 97.0;
        done += usize::from(g.position_at(path, t).done);
    }
    #[allow(clippy::cast_precision_loss)]
    let per_pos = t0.elapsed().as_secs_f64() * 1e6 / queries as f64;
    let legs = corner.legs.len();
    println!(
        "route {per_route:.1} us avg over {routes} (corner-to-corner {corner_us:.1} us, {legs} legs); \
         nearest_point {per_nearest:.3} us ({hits} hits); position_at {per_pos:.3} us ({done} done)"
    );
}
