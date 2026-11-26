//! `MA7a`: `OpenMW`'s wander fast-forward positions (`fast_forward.rs`) and the pathgrid helpers
//! they use (`PathGraph::neighbours`, `components`, `connected`).

use glam::DVec3;
use mw_world::convert::{UNIT_METERS_F64, meters};
use sf_coords::WorldPos;
use tes3::esp::{PathGrid, PathGridPoint};

use crate::fast_forward::allowed_positions;
use crate::{CellKey, PathGraph};

/// A pathgrid record from points (units) and directed edges (record order = the order given).
fn grid(points: &[[i32; 3]], edges: &[(usize, usize)]) -> PathGrid {
    let mut pts: Vec<PathGridPoint> = points
        .iter()
        .map(|&location| PathGridPoint { location, auto_generated: 0, connection_count: 0 })
        .collect();
    let mut connections = Vec::new();
    for (i, p) in pts.iter_mut().enumerate() {
        for &(_, b) in edges.iter().filter(|e| e.0 == i) {
            connections.push(u32::try_from(b).unwrap());
            p.connection_count += 1;
        }
    }
    PathGrid { cell: "Test".into(), points: pts, connections, ..PathGrid::default() }
}

fn graph(points: &[[i32; 3]], edges: &[(usize, usize)]) -> PathGraph {
    PathGraph::from_record(&grid(points, edges), &CellKey::interior("Test")).unwrap()
}

/// Morrowind units to a `WorldPos`.
fn mw(x: f64, y: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, z, -y) * UNIT_METERS_F64)
}

fn assert_positions(got: &[WorldPos], want: &[WorldPos]) {
    assert_eq!(got.len(), want.len(), "{got:?} != {want:?}");
    for (g, w) in got.iter().zip(want) {
        assert!(g.delta_f32(*w).length() < 1e-4, "{got:?} != {want:?}");
    }
}

/// 0-1-2 and 0-3 both ways, 5 off 2 both ways (far), 4 isolated, 0->6 one way.
fn mixed() -> PathGraph {
    graph(
        &[[0, 0, 0], [300, 0, 0], [600, 0, 0], [0, 300, 0], [200, 200, 0], [5000, 0, 0], [0, -300, 0]],
        &[(0, 1), (1, 0), (1, 2), (2, 1), (0, 3), (3, 0), (2, 5), (5, 2), (0, 6)],
    )
}

#[test]
fn neighbours_are_outgoing_edges_in_record_order() {
    let g = mixed();
    assert_eq!(g.neighbours(0), vec![1, 3, 6]);
    assert_eq!(g.neighbours(2), vec![1, 5]);
    assert_eq!(g.neighbours(4), Vec::<usize>::new());
    assert_eq!(g.neighbours(6), Vec::<usize>::new());
    assert_eq!(g.neighbours(99), Vec::<usize>::new());
}

#[test]
fn connected_is_one_strongly_connected_component() {
    let g = mixed();
    let c = g.components();
    assert_eq!(c.len(), 7);
    for n in [1, 2, 3, 5] {
        assert!(g.connected(0, n), "0 ~ {n}");
    }
    // Isolated, and reachable one way only: not connected (OpenMW's SCC test).
    assert!(!g.connected(0, 4));
    assert!(!g.connected(0, 6));
    assert!(!g.connected(4, 6));
    assert!(!g.connected(0, 99));
}

#[test]
fn allowed_are_near_and_connected_minus_the_closest() {
    let g = mixed();
    // 10 m (700 units) around a spawn next to node 0: nodes 0, 1, 2, 3, 4, 6 are near; 4 is
    // isolated and 6 only reachable one way, 5 is connected but far; 0 is the closest (removed).
    let got = allowed_positions(&g, mw(10.0, 0.0, 0.0), meters(700.0));
    assert_positions(&got, &[g.position(1), g.position(2), g.position(3)]);
}

#[test]
fn too_few_points_or_no_distance_allow_nothing() {
    let one = graph(&[[0, 0, 0]], &[]);
    assert!(allowed_positions(&one, mw(0.0, 0.0, 0.0), 50.0).is_empty());
    assert!(allowed_positions(&PathGraph::default(), mw(0.0, 0.0, 0.0), 50.0).is_empty());
    let g = mixed();
    assert!(allowed_positions(&g, mw(10.0, 0.0, 0.0), 0.0).is_empty());
    assert!(allowed_positions(&g, mw(10.0, 0.0, 0.0), -1.0).is_empty());
    assert!(allowed_positions(&g, mw(10.0, 0.0, 0.0), f32::NAN).is_empty());
}

/// Node 0 with edges to 1 (`x` units east) and to 2 (2000 units north), both ways.
fn star(x: i32) -> PathGraph {
    graph(&[[0, 0, 0], [x, 0, 0], [0, 2000, 0]], &[(0, 1), (0, 2), (1, 0), (2, 0)])
}

#[test]
fn one_allowed_node_adds_the_spawn_and_points_along_its_edges() {
    // Minimum wander distance wins (160 / 2 = 80 < 128), clamped to the 100-unit edge to node 1.
    let g = star(100);
    let spawn = mw(0.0, -150.0, 0.0);
    let got = allowed_positions(&g, spawn, meters(160.0));
    // [node 0, spawn, toward 1, toward 2] minus the spawn (closest to itself).
    assert_positions(&got, &[g.position(0), mw(100.0, 0.0, 0.0), mw(0.0, 128.0, 0.0)]);

    // Half the distance wins (320 / 2 = 160 > 128), shorter than both edges.
    let g = star(300);
    let spawn = mw(0.0, -300.0, 0.0);
    let got = allowed_positions(&g, spawn, meters(320.0));
    assert_positions(&got, &[g.position(0), mw(160.0, 0.0, 0.0), mw(0.0, 160.0, 0.0)]);
}

#[test]
fn one_allowed_node_without_edges_keeps_only_the_node() {
    // Node 0 has no outgoing edge (1 -> 0 only): node 0 and the spawn, minus the spawn.
    let g = graph(&[[0, 0, 0], [5000, 0, 0]], &[(1, 0)]);
    let got = allowed_positions(&g, mw(0.0, 40.0, 0.0), meters(100.0));
    assert_positions(&got, &[g.position(0)]);
}

// --- The rule (`MorrowindFastForward` through `sf_ai::fast_forward`) ---

mod rule {
    use std::sync::Arc;

    use sf_ai::{
        ActivityRef, ActorKey, DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, FastForwardRules,
        GameTime, LogicalCtx, LogicalEnv, LogicalState, LogicalTransitions, ModernFastForward, ModernPolicy, RouteRef,
        fast_forward,
    };
    use sf_coords::WorldPos;
    use sf_ids::GameInstant;
    use sf_world::{Location, PlacementId, WorldspaceId};

    use super::{assert_positions, graph, meters, mw};
    use crate::fast_forward::{MorrowindFastForward, allowed_positions};
    use crate::{CellKey, PathGraph, PathGrids};

    const WS: &str = "test";
    const NPC: ActorKey = ActorKey(0xA1);
    const OTHER: ActorKey = ActorKey(0xB2);

    /// Straight routes at 2 m/s.
    struct Env;

    impl LogicalEnv for Env {
        #[expect(clippy::cast_possible_truncation, reason = "test routes are short")]
        fn route(&self, _actor: ActorKey, from: &Location, to: &Location) -> Option<RouteRef> {
            let length_m = from.pos.delta(to.pos).length() as f32;
            Some(RouteRef { from: from.clone(), to: to.clone(), length_m, speed_mps: 2.0 })
        }

        fn point_on(&self, route: &RouteRef, progress_m: f32) -> Location {
            let d = route.from.pos.delta_f32(route.to.pos);
            let along = d.normalize_or_zero() * progress_m.min(d.length());
            Location::new(route.from.worldspace.clone(), route.from.pos.offset(along))
        }

        fn placement(&self, _id: PlacementId) -> Option<Location> {
            None
        }

        fn actor_position(&self, _actor: ActorKey, _at: GameInstant) -> Option<Location> {
            None
        }
    }

    fn at(p: WorldPos) -> Location {
        Location::new(WorldspaceId::new(WS), p)
    }

    fn minute(m: u64) -> GameInstant {
        GameTime::from_minutes(m).instant()
    }

    fn grids(g: PathGraph) -> Arc<PathGrids> {
        let mut grids = PathGrids::default();
        grids.grids.insert(CellKey::interior(WS), g);
        Arc::new(grids)
    }

    /// A chain of 5 nodes 200 units apart along x, both ways.
    fn chain() -> PathGraph {
        graph(
            &[[0, 0, 0], [200, 0, 0], [400, 0, 0], [600, 0, 0], [800, 0, 0]],
            &[(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2), (3, 4), (4, 3)],
        )
    }

    fn rule(g: PathGraph, occupied: impl Fn(&Location, f32) -> bool + Send + Sync + 'static) -> MorrowindFastForward {
        MorrowindFastForward::new(grids(g), Arc::new(occupied), Arc::new(|_| false), 7)
    }

    fn stay_near_in(actor: ActorKey, home: Location, radius: f32) -> DirectiveStack {
        let mut stack = DirectiveStack::new(actor, GameTime::from_minutes(0));
        let spec = DirectiveSpec::new(DirectiveKind::StayNear { home, radius }, DirectiveSource::Record);
        stack.push(spec, GameTime::from_minutes(0), &ModernPolicy, &mut Vec::new());
        stack
    }

    fn stay_near(actor: ActorKey, home: WorldPos, radius: f32) -> DirectiveStack {
        stay_near_in(actor, at(home), radius)
    }

    /// The actor at `start` fast-forwarded `hours` under `rules`: its state, the instant reached.
    fn run_from(
        stack: &mut DirectiveStack,
        start: Location,
        hours: f64,
        rules: &dyn FastForwardRules,
    ) -> (LogicalState, GameInstant) {
        let (env, tr) = (Env, LogicalTransitions::new());
        let ctx = LogicalCtx { env: &env, transitions: &tr, policy: &ModernPolicy, time_scale: 30.0 };
        let mut state = LogicalState::new(start, minute(0));
        let reached = fast_forward(&mut state, stack, hours, &ctx, rules, &mut Vec::new());
        (state, reached)
    }

    fn run(mut stack: DirectiveStack, start: WorldPos, hours: f64, rules: &dyn FastForwardRules) -> (LogicalState, GameInstant) {
        run_from(&mut stack, at(start), hours, rules)
    }

    #[test]
    fn a_wanderer_stands_on_an_allowed_position_at_the_span_end() {
        let home = mw(0.0, 0.0, 0.0);
        let allowed = allowed_positions(&chain(), home, 20.0);
        let nodes = [mw(200.0, 0.0, 0.0), mw(400.0, 0.0, 0.0), mw(600.0, 0.0, 0.0), mw(800.0, 0.0, 0.0)];
        assert_positions(&allowed, &nodes);
        let r = rule(chain(), |_, _| false);
        let (state, reached) = run(stay_near(NPC, home, 20.0), home, 8.0, &r);
        assert_eq!(reached, minute(8 * 60));
        assert_eq!(state.at, reached);
        assert!(allowed.contains(&state.pos.pos), "{:?} not in {allowed:?}", state.pos);
        assert_eq!(state.pos.worldspace, WorldspaceId::new(WS));
        assert_eq!(state.activity, Some(ActivityRef::Wait));
        assert_eq!(state.route, None);
        assert!(state.progress_m.abs() < f32::EPSILON);
        assert_eq!((state.arrive, state.arrives, state.until), (None, None, None));

        // Deterministic: the same actor and span give the same place.
        assert_eq!(run(stay_near(NPC, home, 20.0), home, 8.0, &r).0, state);
        // Another actor, or another span end, rolls again: these land on other nodes.
        let other = run(stay_near(OTHER, home, 20.0), home, 8.0, &r).0;
        assert!(allowed.contains(&other.pos.pos));
        assert_ne!(other.pos, state.pos);
        let longer = run(stay_near(NPC, home, 20.0), home, 10.0, &r).0;
        assert!(allowed.contains(&longer.pos.pos));
        assert_ne!(longer.pos, state.pos);

        // A later step keeps it where it was placed.
        let mut stack = stay_near(NPC, home, 20.0);
        let (mut s, _) = run_from(&mut stack, at(home), 8.0, &r);
        let placed = s.pos.clone();
        let (env, tr) = (Env, LogicalTransitions::new());
        let ctx = LogicalCtx { env: &env, transitions: &tr, policy: &ModernPolicy, time_scale: 30.0 };
        s.step(&mut stack, minute(12 * 60), &ctx, &mut Vec::new());
        assert_eq!(s.pos, placed);
        assert_eq!(s.at, minute(12 * 60));
    }

    /// What tier 0 alone gives (`ModernFastForward`).
    fn modern(stack: DirectiveStack, start: WorldPos, hours: f64) -> LogicalState {
        run(stack, start, hours, &ModernFastForward).0
    }

    #[test]
    fn non_wanderers_and_ungridded_actors_are_left_as_stepped() {
        let home = mw(0.0, 0.0, 0.0);
        let r = rule(chain(), |_, _| false);
        // Radius 0: stands where it is.
        let (state, _) = run(stay_near(NPC, home, 0.0), home, 8.0, &r);
        assert_eq!(state, modern(stay_near(NPC, home, 0.0), home, 8.0));
        assert_eq!(state.pos, at(home));

        // A `GoTo` on top of a wander: only the step moves it (36 game s, mid-walk).
        let go_to = || {
            let mut stack = stay_near(NPC, home, 20.0);
            let spec = DirectiveSpec::new(DirectiveKind::GoTo(at(mw(0.0, 5000.0, 0.0))), DirectiveSource::Script);
            stack.push(spec, GameTime::from_minutes(0), &ModernPolicy, &mut Vec::new());
            stack
        };
        let (state, _) = run(go_to(), home, 0.01, &r);
        assert_eq!(state, modern(go_to(), home, 0.01));
        assert!(state.route.is_some(), "still walking");

        // No pathgrid for its cell.
        let elsewhere = Location::new(WorldspaceId::new("elsewhere"), home);
        let mut stack = stay_near_in(NPC, elsewhere.clone(), 20.0);
        let (ours, _) = run_from(&mut stack, elsewhere.clone(), 8.0, &r);
        let mut stack = stay_near_in(NPC, elsewhere.clone(), 20.0);
        let (theirs, _) = run_from(&mut stack, elsewhere.clone(), 8.0, &ModernFastForward);
        assert_eq!(ours, theirs);
        assert_eq!(ours.pos, elsewhere);

        // A pure water creature.
        let water = MorrowindFastForward::new(grids(chain()), Arc::new(|_, _| false), Arc::new(|a| a == NPC), 7);
        let (state, _) = run(stay_near(NPC, home, 20.0), home, 8.0, &water);
        assert_eq!(state, modern(stay_near(NPC, home, 20.0), home, 8.0));
        assert_eq!(state.pos, at(home));
    }

    /// Nodes 0 (home) and 1 (`x` units east), both ways: the only allowed position is node 1, its
    /// only neighbour node 0.
    fn pair(x: i32) -> PathGraph {
        graph(&[[0, 0, 0], [x, 0, 0]], &[(0, 1), (1, 0)])
    }

    /// Occupied within `r` metres of any of `actors`.
    fn actors_at(actors: Vec<WorldPos>) -> impl Fn(&Location, f32) -> bool + Send + Sync + 'static {
        move |loc: &Location, r: f32| actors.iter().any(|a| a.delta_f32(loc.pos).length() < r)
    }

    fn placed(g: PathGraph, occupied: impl Fn(&Location, f32) -> bool + Send + Sync + 'static) -> WorldPos {
        let home = mw(0.0, 0.0, 0.0);
        run(stay_near(NPC, home, 40.0), home, 8.0, &rule(g, occupied)).0.pos.pos
    }

    #[test]
    fn an_occupied_pick_nudges_toward_a_neighbour_or_stays() {
        let node1 = mw(2000.0, 0.0, 0.0);
        assert_positions(&allowed_positions(&pair(2000), mw(0.0, 0.0, 0.0), 40.0), &[node1]);
        // Free: on the node.
        assert_positions(&[placed(pair(2000), |_, _| false)], &[node1]);
        // Someone on node 1: 5 % toward node 0 (100 units away, outside 60) is free.
        let five_pct = mw(1900.0, 0.0, 0.0);
        assert_positions(&[placed(pair(2000), actors_at(vec![node1]))], &[five_pct]);
        // Someone on the 5 % point as well: 15 %.
        let fifteen_pct = mw(1700.0, 0.0, 0.0);
        assert_positions(&[placed(pair(2000), actors_at(vec![node1, five_pct]))], &[fifteen_pct]);
        // And on the 15 % point: 30 %.
        assert_positions(&[placed(pair(2000), actors_at(vec![node1, five_pct, fifteen_pct]))], &[mw(1400.0, 0.0, 0.0)]);
        // A short edge: 5 % (50 units) is still within 60 of node 1, so 15 %.
        assert_positions(&[placed(pair(1000), actors_at(vec![mw(1000.0, 0.0, 0.0)]))], &[mw(850.0, 0.0, 0.0)]);
        // Everything occupied: stays home.
        assert_positions(&[placed(pair(2000), |_, _| true)], &[mw(0.0, 0.0, 0.0)]);
    }

    #[test]
    fn an_occupied_pick_without_neighbours_stays() {
        // Node 0 has no outgoing edge: its only allowed position is the node itself, which has no
        // neighbour to nudge toward.
        let g = graph(&[[0, 0, 0], [5000, 0, 0]], &[(1, 0)]);
        let (home, node0, radius) = (mw(0.0, 40.0, 0.0), mw(0.0, 0.0, 0.0), meters(100.0));
        assert_positions(&allowed_positions(&g, home, radius), &[node0]);
        let (state, _) = run(stay_near(NPC, home, radius), home, 8.0, &rule(g.clone(), actors_at(vec![node0])));
        assert_eq!(state.pos, at(home));
        // Free, it is placed on the node.
        let (state, _) = run(stay_near(NPC, home, radius), home, 8.0, &rule(g, |_, _| false));
        assert_positions(&[state.pos.pos], &[node0]);
    }
}
