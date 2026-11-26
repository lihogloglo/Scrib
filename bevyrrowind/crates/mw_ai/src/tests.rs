use std::collections::BTreeSet;

use glam::{DVec3, Vec3};
use mw_world::convert::{UNIT_METERS_F64, meters};
use sf_controller::{
    HeightSurface, KinematicMover, Locomotion, MoveInput, MoverParams, RootMotion, TerrainProbe,
};
use sf_coords::WorldPos;
use tes3::esp::{PathGrid, PathGridPoint};

use crate::*;

/// A pathgrid record from points (units) and directed edges.
fn grid(points: &[[i32; 3]], edges: &[(usize, usize)]) -> PathGrid {
    let mut pts: Vec<PathGridPoint> = points
        .iter()
        .map(|&location| PathGridPoint {
            location,
            auto_generated: 0,
            connection_count: 0,
        })
        .collect();
    let mut connections = Vec::new();
    for (i, p) in pts.iter_mut().enumerate() {
        for &(a, b) in edges.iter().filter(|e| e.0 == i) {
            debug_assert_eq!(a, i);
            connections.push(u32::try_from(b).unwrap());
            p.connection_count += 1;
        }
    }
    PathGrid {
        cell: "Test".into(),
        points: pts,
        connections,
        ..PathGrid::default()
    }
}

/// An `n` x `n` lattice spaced `step` units, 4-neighbour edges both ways.
fn lattice(n: i32, step: i32) -> PathGrid {
    let mut points = Vec::new();
    let mut edges = Vec::new();
    let idx = |x: i32, y: i32| usize::try_from(y * n + x).unwrap();
    for y in 0..n {
        for x in 0..n {
            points.push([x * step, y * step, 0]);
            if x + 1 < n {
                edges.extend([(idx(x, y), idx(x + 1, y)), (idx(x + 1, y), idx(x, y))]);
            }
            if y + 1 < n {
                edges.extend([(idx(x, y), idx(x, y + 1)), (idx(x, y + 1), idx(x, y))]);
            }
        }
    }
    grid(&points, &edges)
}

fn interior(g: &PathGrid) -> PathGraph {
    PathGraph::from_record(g, &CellKey::interior("Test")).unwrap()
}

/// Morrowind units to a `WorldPos`.
fn mw(x: f64, y: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, z, -y) * UNIT_METERS_F64)
}

fn flat_dist(a: WorldPos, b: WorldPos) -> f32 {
    let d = a.delta_f32(b);
    Vec3::new(d.x, 0.0, d.z).length()
}

fn no_idles(distance_units: f32) -> Wander {
    Wander {
        distance: meters(distance_units),
        duration_hours: 0.0,
        game_hour: 0,
        idle_chances: [0; 8],
        repeat: true,
    }
}

#[test]
fn graph_offsets_exteriors_and_drops_bad_edges() {
    let g = grid(
        &[[0, 0, 0], [100, 0, 0], [100, 200, 50]],
        &[(0, 1), (1, 0), (1, 2), (2, 2), (0, 1)],
    );
    let ext = PathGraph::from_record(&g, &CellKey::Exterior(1, -2)).unwrap();
    assert_eq!(ext.len(), 3);
    assert!(ext.position(0).delta(mw(8192.0, -16384.0, 0.0)).length() < 1e-5);
    assert!(ext.position(2).delta(mw(8292.0, -16184.0, 50.0)).length() < 1e-5);
    assert_eq!(
        CellKey::exterior_at(ext.position(2)),
        CellKey::Exterior(1, -2)
    );
    // The duplicate (0,1) and the self-loop (2,2) are dropped; 1->2 is one-way.
    assert_eq!(ext.edge_count(), 3);
    assert_eq!(ext.one_way_edges(), 1);
    let int = interior(&g);
    assert!(int.position(1).delta(mw(100.0, 0.0, 0.0)).length() < 1e-5);

    let mut bad = g.clone();
    bad.connections.pop();
    assert!(matches!(
        PathGraph::from_record(&bad, &CellKey::Exterior(0, 0)),
        Err(PathGridError::Connections { .. })
    ));
    let mut bad = g;
    bad.connections[0] = 7;
    assert!(matches!(
        PathGraph::from_record(&bad, &CellKey::Exterior(0, 0)),
        Err(PathGridError::BadTarget {
            point: 0,
            target: 7,
            ..
        })
    ));
}

#[test]
fn astar_follows_directed_edges() {
    // A square 0-1-2-3 with a one-way diagonal 0->2.
    let g = grid(
        &[[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]],
        &[
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
            (2, 3),
            (3, 2),
            (3, 0),
            (0, 3),
            (0, 2),
        ],
    );
    let graph = interior(&g);
    assert_eq!(graph.path(0, 2), Some(vec![0, 2]));
    let back = graph.path(2, 0).unwrap();
    assert_eq!(back.len(), 3, "{back:?}");
    assert!((graph.path_length(&back) - meters(1000.0)).abs() < 1e-3);
    assert_eq!(graph.path(1, 1), Some(vec![1]));
    assert_eq!(graph.path(0, 9), None);
    let island = interior(&grid(&[[0, 0, 0], [100, 0, 0]], &[]));
    assert_eq!(island.path(0, 1), None);

    assert_eq!(graph.nearest(mw(480.0, 30.0, 0.0)), Some(1));
    assert_eq!(
        graph.within(mw(0.0, 0.0, 0.0), meters(510.0)),
        vec![0, 1, 3]
    );
}

/// `AiWander::getRandomIdle` (`OpenMW` `aiwander.cpp` at `b8285cc9`) on scripted draws: the gate
/// draw against `fIdleChanceMultiplier`, then `roll = 100 * draw` per idle, `roll <= chance`
/// and `roll > best` (best starts at 0, ties keep the earlier idle).
#[test]
fn idle_roll_is_openmw_get_random_idle_exactly() {
    fn scripted(chances: [u8; 8], mult: f32, draws: &[f32]) -> (Option<u8>, usize) {
        let mut i = 0;
        let pick = roll_idle_with(&chances, mult, || {
            let d = draws[i];
            i += 1;
            d
        });
        (pick, i)
    }
    let c = [10, 20, 30, 40, 50, 60, 70, 80];
    // Gate: a first draw above the multiplier means no idle, and nothing else is drawn.
    assert_eq!(scripted(c, 0.75, &[0.76]), (None, 1));
    // At the multiplier exactly the gate passes (`>`, not `>=`).
    assert_eq!(scripted(c, 0.75, &[0.75, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]), (None, 9));
    // A zero roll never wins (`roll > best`, best starts at 0), even with chance 100.
    assert_eq!(scripted([100; 8], 1.0, &[0.0; 9]), (None, 9));
    // `roll <= chance` is inclusive: 50.0 wins Idle6 (chance 50); 50.5 doesn't.
    assert_eq!(scripted(c, 0.75, &[0.5, 0.9, 0.9, 0.9, 0.9, 0.5, 0.9, 0.9, 0.9]), (Some(6), 9));
    assert_eq!(scripted(c, 0.75, &[0.5, 0.9, 0.9, 0.9, 0.9, 0.505, 0.9, 0.9, 0.9]), (None, 9));
    // The highest qualifying roll wins: Idle3 rolls 15 (<= 20), Idle7 rolls 55 (<= 60), Idle9
    // rolls 81 (> 80, out).
    assert_eq!(scripted(c, 0.75, &[0.1, 0.9, 0.15, 0.9, 0.9, 0.9, 0.55, 0.9, 0.81]), (Some(7), 9));
    // A tie keeps the earlier idle (strict `>`).
    assert_eq!(scripted([50; 8], 1.0, &[0.0, 0.4, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]), (Some(2), 9));
}

#[test]
fn idle_roll_matches_openmw_statistics() {
    let mut rng = fastrand::Rng::with_seed(7);
    let n = 200_000;
    // One idle at 100%: the gate passes with p = 0.75, then any roll in (0, 100] wins.
    let mut one = [0; 8];
    one[0] = 100;
    let hits = (0..n)
        .filter(|_| roll_idle(&one, 0.75, &mut rng) == Some(2))
        .count();
    let p = f64::from(u32::try_from(hits).unwrap()) / f64::from(n);
    assert!((p - 0.75).abs() < 0.005, "{p}");
    // One idle at 40%: 0.75 * 0.40.
    one[0] = 40;
    let hits = (0..n)
        .filter(|_| roll_idle(&one, 0.75, &mut rng) == Some(2))
        .count();
    let p = f64::from(u32::try_from(hits).unwrap()) / f64::from(n);
    assert!((p - 0.3).abs() < 0.005, "{p}");
    assert!((0..1000).all(|_| roll_idle(&[0; 8], 0.75, &mut rng).is_none()));
    // Equal chances pick every group about equally often.
    let mut counts = [0u32; 10];
    for _ in 0..n {
        if let Some(g) = roll_idle(&[30; 8], 0.75, &mut rng) {
            counts[usize::from(g)] += 1;
        }
    }
    let total: u32 = counts.iter().sum();
    for &c in &counts[2..] {
        let share = f64::from(c) / f64::from(total);
        assert!((share - 0.125).abs() < 0.01, "{counts:?}");
    }
}

#[test]
fn wander_goals_stay_in_range_and_cover_allowed_nodes() {
    let graph = interior(&lattice(7, 128));
    let home = graph.position(24); // the centre
    let planner = WanderPlanner::new(no_idles(300.0), home);
    let allowed: BTreeSet<usize> = graph
        .within(home, planner.wander.distance)
        .into_iter()
        .collect();
    assert!(allowed.len() > 5 && allowed.len() < 49);
    let mut rng = fastrand::Rng::with_seed(1);
    let mut goals = BTreeSet::new();
    for i in 0..2000 {
        // Start anywhere on the grid, also outside the disc.
        let pos = graph
            .position(i % graph.len())
            .offset(Vec3::new(0.3, 0.0, -0.2));
        let WanderStep::Walk {
            nodes,
            via_grid: true,
        } = planner.next(Some(&graph), pos, &mut rng)
        else {
            panic!("expected a grid walk");
        };
        let goal = nodes.last().unwrap().pos;
        assert!(home.delta_f32(goal).length() <= planner.wander.distance + 1e-4);
        goals.insert(graph.nearest(goal).unwrap());
        // Consecutive nodes are grid neighbours.
        for w in nodes.windows(2) {
            assert!((w[0].pos.delta_f32(w[1].pos).length() - meters(128.0)).abs() < 1e-3);
        }
    }
    assert_eq!(goals, allowed);

    // Idle chances win before walking: at 100 % each, every roll past the
    // `fIdleChanceMultiplier` gate (0.75) is an idle, the rest walk.
    let mut idler = planner.clone();
    idler.wander.idle_chances = [100; 8];
    idler.idle_seconds[3] = 2.5;
    let steps: Vec<_> = (0..500)
        .map(|_| idler.next(Some(&graph), home, &mut rng))
        .collect();
    let fidgets = steps
        .iter()
        .filter(|s| matches!(s, WanderStep::Idle { variant: 2..=9, .. }))
        .count();
    assert!((330..=420).contains(&fidgets), "{fidgets} idles of 500");
    assert!(steps.iter().all(|s| matches!(s, WanderStep::Idle { variant: 2..=9, .. } | WanderStep::Walk { .. })));
    assert!(steps.contains(&WanderStep::Idle {
        variant: 5,
        seconds: 2.5
    }));

    // Distance 0 stands; no grid walks straight to a point in the disc.
    let stander = WanderPlanner::new(no_idles(0.0), home);
    for _ in 0..100 {
        let WanderStep::Idle {
            variant: 0,
            seconds,
        } = stander.next(Some(&graph), home, &mut rng)
        else {
            panic!()
        };
        assert!((2.0..=5.0).contains(&seconds));
        let WanderStep::Walk {
            nodes,
            via_grid: false,
        } = planner.next(None, home, &mut rng)
        else {
            panic!()
        };
        assert_eq!(nodes.len(), 1);
        assert!(flat_dist(home, nodes[0].pos) <= planner.wander.distance + 1e-4);
    }
}

/// Flat ground at height 0.
struct Flat;

impl HeightSurface for Flat {
    fn height(&self, _p: WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    #[allow(clippy::cast_possible_truncation)]
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        if dir.y.abs() < 1e-6 {
            return None;
        }
        let t = -(origin.to_f64().y as f32) / dir.y;
        (0.0..=max).contains(&t).then_some((t, Vec3::Y))
    }
}

/// Distance in the ground plane from `pos` to the polyline.
fn polyline_dist(pos: WorldPos, line: &[WorldPos]) -> f32 {
    let flat = |w: WorldPos| {
        let d = line[0].delta_f32(w);
        Vec3::new(d.x, 0.0, d.z)
    };
    let q = flat(pos);
    line.windows(2)
        .map(|w| {
            let (a, b) = (flat(w[0]), flat(w[1]));
            let ab = b - a;
            let t = ((q - a).dot(ab) / ab.length_squared().max(1e-9)).clamp(0.0, 1.0);
            q.distance(a + ab * t)
        })
        .fold(f32::MAX, f32::min)
}

#[test]
fn mover_walks_scaffold_paths_on_flat_ground() {
    let graph = interior(&lattice(6, 256));
    let home = graph.position(14);
    let speed = walk_speed(50.0, wander::MIN_WALK_SPEED, wander::MAX_WALK_SPEED);
    assert!((speed - meters(150.0)).abs() < 1e-5);
    let mut scaffold = WanderDriver::new(WanderPlanner::new(no_idles(2000.0), home), speed, 11);
    let probe = TerrainProbe(Flat);
    let params = MoverParams::default();
    let mut loco = Locomotion::new(home, 0.0);
    let mut mover = KinematicMover::new();
    let dt = 1.0 / 60.0;
    let mut line: Vec<WorldPos> = Vec::new();
    let (mut walks, mut worst_track, mut worst_end) = (0, 0.0f32, 0.0f32);
    let mut was_walking = false;
    for _ in 0..(60 * 90) {
        let out = scaffold.update(Some(&graph), loco.position, dt);
        if out.walking && !was_walking {
            line = std::iter::once(loco.position)
                .chain(scaffold.follower().unwrap().nodes.iter().map(|n| n.pos))
                .collect();
        }
        if out.walking {
            worst_track = worst_track.max(polyline_dist(loco.position, &line));
        } else if was_walking {
            walks += 1;
            worst_end = worst_end.max(flat_dist(loco.position, *line.last().unwrap()));
        }
        was_walking = out.walking;
        mover.step(
            MoveInput {
                loco: &mut loco,
                intent: out.intent,
                params: &params,
                root: RootMotion::default(),
            },
            &probe,
            dt,
        );
        assert!(loco.position.to_f64().y.abs() < 0.05, "left the ground");
    }
    eprintln!("{walks} walks, worst track {worst_track:.3} m, worst end {worst_end:.3} m");
    assert!(walks >= 5, "{walks} walks in 90 s");
    assert!(worst_track < 0.75, "strayed {worst_track} m from the path");
    assert!(worst_end < 0.3, "stopped {worst_end} m from the goal");
}

/// Stuck handling: an actor that makes no headway gives the walk up after
/// `STUCK_SECONDS` (well before the walk's time backstop), stands, then
/// picks another goal.
#[test]
fn stuck_walks_are_given_up_and_replanned() {
    let graph = interior(&lattice(5, 256));
    let home = graph.position(12);
    let mut s = WanderDriver::new(WanderPlanner::new(no_idles(1024.0), home), 1.5, 7);
    let dt = 0.1;
    // Walking, but held in place (a wall the path doesn't know about).
    let mut walked = 0;
    let mut first_give_up = None;
    for i in 0..200 {
        let out = s.update(Some(&graph), home, dt);
        walked += usize::from(out.walking);
        if s.stuck_count() == 1 && first_give_up.is_none() {
            first_give_up = Some(i);
            assert!(!out.walking && out.intent == sf_controller::LocomotionIntent::IDLE);
        }
    }
    let first = first_give_up.expect("a stuck walk was given up");
    // 3 s without progress (+ a frame or two), not the 2x-nominal backstop.
    assert!((29..=32).contains(&first), "gave up after {first} frames");
    assert!(walked > 0);
    // It keeps trying other goals after standing a moment.
    assert!(s.stuck_count() >= 3, "replanned: {}", s.stuck_count());
}

/// A walk given up on an edge the mover found unwalkable (too steep, a
/// wall) resumes at once around it: the edge is remembered both ways and
/// A* routes over the rest of the grid to the same goal.
#[test]
#[allow(clippy::many_single_char_names)]
fn blocked_edges_are_routed_around() {
    // A square 0-1-2-3, both ways round.
    let g = grid(
        &[[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]],
        &[(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2), (3, 0), (0, 3)],
    );
    let graph = interior(&g);
    assert_eq!(graph.path_avoiding(0, 1, &[(0, 1)]), Some(vec![0, 3, 2, 1]));
    assert_eq!(graph.path_avoiding(0, 1, &[(0, 1), (3, 2)]), None);

    let (a, b) = (graph.position(0), graph.position(1));
    let mut s = WanderDriver::new(WanderPlanner::new(no_idles(1024.0), a), 1.5, 3);
    // Walk until a give-up whose walk heads from node 0 to node 1, held in place at node 0.
    let mut resumed = false;
    for _ in 0..2000 {
        let edge = s.follower().map(|f| (f.target(), f.nodes.clone()));
        s.update(Some(&graph), a, 0.1);
        if let Some((t, nodes)) = edge.filter(|_| s.follower().is_none() && s.stuck_count() > 0)
            && t < nodes.len()
            && nodes[t].pos == b
        {
            assert!(s.replan_around(Some(&graph), a, b, a), "a route around exists");
            let f = s.follower().expect("walking again");
            let route: Vec<usize> = f.nodes.iter().map(|n| graph.node_at(n.pos).unwrap()).collect();
            assert!(!route.windows(2).any(|w| w == [0, 1] || w == [1, 0]), "{route:?}");
            resumed = true;
            break;
        }
    }
    assert!(resumed, "never gave up on 0 -> 1");
    assert_eq!(s.replan_count(), 1);
    assert_eq!(s.planner.blocked, vec![(a, b), (b, a)]);
}

/// A first leg given up (from where the actor stood to its nearest node,
/// into ground it can't climb; walkers5: all of Balmora's give-ups) starts
/// the walk at the next-nearest node instead, and later plans from about
/// there skip that node as a start too.
#[test]
fn first_leg_give_ups_start_at_the_next_nearest_node() {
    // A square 0-1-2-3, both ways round; the actor stands just off node 0.
    let g = grid(
        &[[0, 0, 0], [500, 0, 0], [500, 500, 0], [0, 500, 0]],
        &[(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2), (3, 0), (0, 3)],
    );
    let graph = interior(&g);
    let a = graph.position(0);
    // Behind node 0 from both its neighbours, so no plan skips node 0 as
    // already passed.
    let away = -(a.delta_f32(graph.position(1)).normalize() + a.delta_f32(graph.position(3)).normalize()).normalize();
    let pos = a.offset(away * 0.8);
    let mut s = WanderDriver::new(WanderPlanner::new(no_idles(1024.0), pos), 1.5, 5);
    let mut resumed = false;
    for _ in 0..2000 {
        let edge = s.follower().map(|f| (f.target(), f.nodes.clone()));
        s.update(Some(&graph), pos, 0.1);
        if let Some((0, nodes)) = edge.filter(|_| s.follower().is_none() && s.stuck_count() > 0)
            && nodes[0].pos == a
        {
            assert!(s.replan_around(Some(&graph), pos, a, pos), "a walk from the next-nearest node exists");
            let f = s.follower().expect("walking again");
            assert_ne!(f.nodes[0].pos, a, "heads for the blocked start again");
            resumed = true;
            break;
        }
    }
    assert!(resumed, "never gave up on the first leg to node 0");
    assert_eq!(s.planner.blocked_starts, vec![(pos, a)]);
    assert!(s.planner.blocked.is_empty(), "a first leg is no pathgrid edge");
    // Later plans from about here start elsewhere; from afar, node 0 is fine again.
    let mut rng = fastrand::Rng::with_seed(9);
    for _ in 0..50 {
        if let WanderStep::Walk { nodes, .. } = s.planner.next(Some(&graph), pos, &mut rng) {
            assert_ne!(nodes[0].pos, a);
        }
    }
    let far = a.offset(away * 3.0);
    let starts_at_a = (0..50).any(|_| matches!(s.planner.next(Some(&graph), far, &mut rng), WanderStep::Walk { nodes, .. } if nodes[0].pos == a));
    assert!(starts_at_a);
}

/// Off the pathgrid (no node in reach: a straight walk to a random point),
/// a give-up idles as before, and the next rolls from about there avoid the
/// blocked heading (walkers5: Balmora's repeated give-ups at spawn were
/// these, re-rolled into the same slope).
#[test]
fn off_grid_give_ups_avoid_the_blocked_heading() {
    let home = WorldPos::ORIGIN;
    let mut s = WanderDriver::new(WanderPlanner::new(no_idles(1024.0), home), 1.5, 11);
    let mut blocked: Vec<Vec3> = Vec::new();
    let mut walks = 0;
    for _ in 0..6000 {
        let edge = s.follower().map(|f| f.nodes[0].pos);
        let given_up = s.stuck_count();
        s.update(None, home, 0.1);
        if let (None, Some(f)) = (edge, s.follower()) {
            // A new walk: away from every heading given up here (that it remembers).
            let heading = home.delta_f32(f.nodes[0].pos).with_y(0.0).normalize();
            for b in blocked.iter().rev().take(wander::BLOCKED_EDGES) {
                assert!(b.dot(heading) <= std::f32::consts::FRAC_1_SQRT_2, "heads within 45° of a blocked heading");
            }
            walks += 1;
        }
        if let Some(to) = edge.filter(|_| s.stuck_count() > given_up) {
            assert!(!s.replan_around(None, home, to, home), "off the grid there is no route to resume");
            blocked.push(home.delta_f32(to).with_y(0.0).normalize());
        }
    }
    assert!(blocked.len() >= 3 && walks >= blocked.len(), "{} give-ups, {walks} walks", blocked.len());
    assert_eq!(s.planner.blocked_starts.len(), blocked.len().min(wander::BLOCKED_EDGES));
}

#[test]
fn an_idle_never_repeats_back_to_back() {
    // Distance 0 stands: every step is an idle, a fidget when the roll hits.
    let mut wander = no_idles(0.0);
    wander.idle_chances = [100, 100, 0, 0, 0, 0, 0, 0];
    let planner = WanderPlanner::new(wander, WorldPos::from_f64(DVec3::ZERO));
    let mut rng = fastrand::Rng::with_seed(11);
    let (mut last, mut fidgets) = (None, 0);
    for _ in 0..5000 {
        let WanderStep::Idle { variant, .. } = planner.next_after(None, WorldPos::from_f64(DVec3::ZERO), &mut rng, last) else { panic!() };
        let now = (variant >= 2).then_some(variant);
        assert!(now.is_none() || now != last, "{variant} twice in a row");
        fidgets += usize::from(now.is_some());
        last = now;
    }
    assert!(fidgets > 1000, "{fidgets}");
}

#[test]
#[allow(clippy::too_many_lines)]
fn htn_wander_on_navmesh_stays_within_radius_and_rolls_exact_idles() {
    use sf_ai::{ExecConfig, ExecContext, NavCommand, NavMoveState, PlanOutcome, TraceEntry};
    use sf_controller::{PathFollower, PathMode};
    use sf_nav::{
        AgentClassId, AreaId, CorridorConfig, Goal, GridBackend, GridCell, GridConfig, JobState,
        NavBackend, PathCorridor, PathJob, PathQuery, PolyFlags, QueryFilter, SliceBudget,
    };
    use sf_world::{Location, WorldspaceId};

    let vocab = WanderVocabulary::build();
    let ws = WorldspaceId::new("test_room");
    let class = AgentClassId(0);
    let area = AreaId(1);
    let filter = QueryFilter::default();

    // 48 x 48 m flat walkable room on GridBackend (covers the 6x6 lattice in engine space:
    // x in 0..=18.29 m, z in -18.29..=0 m; shift lattice into positive X/Z so it sits inside tile (0,0)).
    let mut backend = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 48,
    })
    .unwrap();
    let row = ".".repeat(48);
    let rows: Vec<&str> = (0..48).map(|_| row.as_str()).collect();
    backend
        .insert_rows(&ws, class, 0.5, &rows, |c| {
            (c == '.').then_some(GridCell::new(0.0, area, PolyFlags::WALK))
        })
        .unwrap();

    // Build a 5x5 lattice shifted into positive engine X/Z (`y <= 0` in Morrowind units -> `z >= 0` in engine).
    let mut pts = Vec::new();
    let mut edges = Vec::new();
    let n = 5i32;
    let step = 256i32;
    let idx = |x: i32, y: i32| usize::try_from(y * n + x).unwrap();
    for y in 0..n {
        for x in 0..n {
            pts.push([(x + 1) * step, -(y + 1) * step, 0]);
            if x + 1 < n {
                edges.extend([(idx(x, y), idx(x + 1, y)), (idx(x + 1, y), idx(x, y))]);
            }
            if y + 1 < n {
                edges.extend([(idx(x, y), idx(x, y + 1)), (idx(x, y + 1), idx(x, y))]);
            }
        }
    }
    let graph = interior(&grid(&pts, &edges));
    let home = graph.position(12); // center of the 5x5 grid

    let wander = Wander {
        distance: meters(400.0),
        duration_hours: 0.0,
        game_hour: 0,
        idle_chances: [40, 40, 40, 40, 0, 0, 0, 0],
        repeat: true,
    };
    let speed = 2.0;
    let mut driver = WanderDriver::with_vocab(
        WanderPlanner::new(wander, home),
        speed,
        42,
        ws.clone(),
        std::sync::Arc::new(vocab.clone()),
    );

    let cfg = ExecConfig::default();
    let corridor_cfg = CorridorConfig::default();
    let dt = 0.1;

    let mut pos = home;
    let mut corridor: Option<(Location, PathCorridor, PathFollower)> = None;
    let mut nav_state = NavMoveState::Idle;
    let mut walk_plans = 0usize;
    let mut idle_plans = 0usize;
    let mut stand_plans = 0usize;
    let mut fidget_variants = Vec::new();
    let mut last_fidget: Option<u8> = None;

    for tick in 0..4000u64 {
        let ctx = ExecContext {
            location: Location::new(ws.clone(), pos),
            yaw: 0.0,
            dt,
            nav: nav_state,
        };
        let mut trace = TraceEntry::new(tick, 2);
        let out = driver.step_htn(Some(&graph), &ctx, &cfg, Some(&mut trace));

        if out.plan_outcome == Some(PlanOutcome::Planned) {
            assert_eq!(trace.methods.len(), 1);
            let chosen_method = trace.methods[0].method;
            assert_eq!(trace.plan.steps.len(), 1);
            let chosen_task = trace.plan.steps[0].task;
            // Exact decomposition: walk -> wander_walk/do_walk (a MoveTo), fidget Idle2..=9 ->
            // wander_idle/do_idle (a Wait with that group), plain stand -> wander_stand/do_idle.
            if let NavCommand::MoveTo { .. } = out.nav {
                assert_eq!((chosen_method, chosen_task), (vocab.wander_walk, vocab.do_walk));
                assert_eq!(out.idle_variant, None);
                walk_plans += 1;
                last_fidget = None;
            } else {
                assert_eq!(chosen_task, vocab.do_idle);
                idle_plans += 1;
                if let Some(v @ 2..=9) = out.idle_variant {
                    assert_eq!(chosen_method, vocab.wander_idle, "fidget {v}");
                    assert_ne!(Some(v), last_fidget, "back-to-back fidget {v}");
                    last_fidget = Some(v);
                    fidget_variants.push(v);
                } else {
                    assert_eq!(out.idle_variant, Some(0));
                    assert_eq!(chosen_method, vocab.wander_stand);
                    stand_plans += 1;
                    last_fidget = None;
                }
            }
        }

        match out.nav {
            NavCommand::MoveTo { goal, speed, .. } => {
                let dist_from_home = flat_dist(home, goal.pos);
                assert!(
                    dist_from_home <= wander.distance + 1e-3,
                    "MoveTo goal {dist_from_home} m exceeds wander radius {}",
                    wander.distance
                );
                let needs_load = corridor.as_ref().is_none_or(|(g, _, _)| g != &goal);
                if needs_load {
                    let q = PathQuery::new(
                        class,
                        Location::new(ws.clone(), pos),
                        Goal::Point(goal.clone()),
                        filter.clone(),
                    );
                    let mut job = PathJob::new(q);
                    let mut budget = SliceBudget::new(2048);
                    let JobState::Done(path) = backend.find_path(&mut job, &mut budget) else {
                        panic!("expected complete path on GridBackend");
                    };
                    let mut c = PathCorridor::new(class, filter.clone());
                    c.load(goal.clone(), &path).unwrap();
                    let f = PathFollower::new(Vec::new(), PathMode::Once, speed);
                    corridor = Some((goal, c, f));
                }
                let (_, c, f) = corridor.as_mut().unwrap();
                c.step(&backend, Location::new(ws.clone(), pos), dt, &corridor_cfg)
                    .unwrap();
                let stop_at_end = c.stop_at_end();
                let mut corners: Vec<WorldPos> = c.corners().iter().map(|l| l.pos).collect();
                if corners.is_empty() && stop_at_end {
                    corners.push(c.target().pos);
                }
                f.set_corridor_corners(&corners, stop_at_end);
                let fout = f.update(pos, dt);
                pos = pos.offset(fout.intent.desired_velocity * dt);
                let local = home.delta_f32(pos);
                assert!(local.is_finite(), "NaN position at tick {tick}");
                assert!(
                    flat_dist(home, pos) <= wander.distance + 1.0,
                    "walked {} m from home (radius {})",
                    flat_dist(home, pos),
                    wander.distance
                );
                nav_state = if fout.finished && stop_at_end {
                    NavMoveState::Arrived
                } else {
                    NavMoveState::Following
                };
            }
            NavCommand::Stop | NavCommand::None => {
                corridor = None;
                nav_state = NavMoveState::Idle;
            }
        }
    }

    assert!(walk_plans >= 10, "expected >= 10 HTN walk plans, got {walk_plans}");
    assert!(idle_plans >= 10, "expected >= 10 HTN idle plans, got {idle_plans}");
    assert!(
        fidget_variants.len() >= 8,
        "expected >= 8 fidget idles, got {}",
        fidget_variants.len()
    );
    // Every walk plan but the one still running at the end reached its goal (comes to rest).
    let arrived = usize::try_from(driver.arrival_count()).unwrap();
    assert!(arrived + 1 >= walk_plans && arrived >= 10, "{arrived} arrivals of {walk_plans} walks");
    assert_eq!(driver.stuck_count(), 0, "open room: no walk given up");
    let vocab_stand = vocab.wander_stand;

    // Verify 3.0 s NoProgress stuck timeout on HTN MoveTo: hold the actor in place, confirm stuck
    // increments after 3.0 s, recovery idle runs, and the next walk picks a different goal.
    let mut stuck_driver = WanderDriver::with_vocab(
        WanderPlanner::new(no_idles(600.0), home),
        speed,
        7,
        ws.clone(),
        std::sync::Arc::new(vocab),
    );
    let mut first_goal = None;
    let mut give_up_tick = None;
    let mut recovery_idled = false;
    let mut second_goal = None;

    for i in 0..80 {
        let ctx = ExecContext {
            location: Location::new(ws.clone(), home),
            yaw: 0.0,
            dt,
            nav: NavMoveState::Following,
        };
        let mut trace = TraceEntry::new(i, 2);
        let out = stuck_driver.step_htn(Some(&graph), &ctx, &cfg, Some(&mut trace));
        // The pause after a give-up is a plain stand: `wander_stand`.
        if give_up_tick.is_some() && out.plan_outcome == Some(PlanOutcome::Planned) && out.nav == NavCommand::Stop {
            assert_eq!(trace.methods[0].method, vocab_stand);
            stand_plans += 1;
        }
        if let NavCommand::MoveTo { ref goal, .. } = out.nav {
            if first_goal.is_none() {
                first_goal = Some(goal.pos);
            } else if give_up_tick.is_some() && second_goal.is_none() {
                second_goal = Some(goal.pos);
                break;
            }
        }
        if stuck_driver.stuck_count() == 1 && give_up_tick.is_none() {
            give_up_tick = Some(i);
            assert_eq!(out.nav, NavCommand::Stop);
            assert_eq!(out.idle_variant, Some(0));
        } else if give_up_tick.is_some() && out.nav == NavCommand::Stop {
            recovery_idled = true;
        }
    }

    let give_up = give_up_tick.expect("HTN MoveTo should fail with NoProgress after 3.0 s");
    assert!((29..=31).contains(&give_up), "expected give-up around 30 ticks (3.0 s), got {give_up}");
    assert!(recovery_idled, "expected recovery Wait after stuck give-up");
    assert!(stand_plans >= 1, "the recovery pause decomposes through wander_stand");
    assert_ne!(
        first_goal.expect("first goal"),
        second_goal.expect("second goal after recovery"),
        "replanned walk should avoid the abandoned goal"
    );
}

/// A walker that leaves the resident navmesh mid-walk (its tile evicted, or walked off it) drops
/// the navmesh `MoveTo` and picks a pathgrid walk in fallback mode instead of standing forever.
#[test]
fn navmesh_walk_left_mid_way_continues_on_the_pathgrid() {
    use sf_ai::{ExecConfig, ExecContext, NavCommand, NavMoveState};
    use sf_world::{Location, WorldspaceId};

    let graph = interior(&lattice(5, 256));
    let home = graph.position(12);
    let ws = WorldspaceId::new("test_room");
    let mut driver = WanderDriver::with_vocab(
        WanderPlanner::new(no_idles(600.0), home),
        2.0,
        3,
        ws.clone(),
        std::sync::Arc::new(WanderVocabulary::build()),
    );
    let ctx = ExecContext {
        location: Location::new(ws, home),
        yaw: 0.0,
        dt: 0.1,
        nav: NavMoveState::Idle,
    };
    let out = driver.step_htn(Some(&graph), &ctx, &ExecConfig::default(), None);
    assert!(matches!(out.nav, NavCommand::MoveTo { .. }), "{:?}", out.nav);
    assert!(driver.is_walking());
    // Off the navmesh from here on: the next fallback step walks a pathgrid route at once.
    let out = driver.update_fallback(Some(&graph), home, 0.1, &ExecConfig::default());
    assert!(out.walking, "stalled on the navmesh MoveTo");
    assert!(driver.follower().is_some());
    assert!(out.intent.desired_velocity.length() > 0.1, "{:?}", out.intent.desired_velocity);
}

/// A navmesh `MoveTo` that fails (`NoPath`: goal off the mesh; `NoProgress`: held in place) is the
/// goal failing, not a pathgrid edge or first leg (the corridor walked neither): the driver keeps
/// the abandoned goal, the planner's pathgrid memory stays empty, and the next walk rolls away
/// from it (`OpenMW` `AiWander` re-chooses and records no edge, `aiwander.cpp` at `b8285cc9`).
#[test]
#[allow(clippy::too_many_lines)]
fn navmesh_give_ups_remember_the_goal_not_pathgrid_edges() {
    use sf_ai::{
        ExecConfig, ExecContext, NavCommand, NavMoveState, PathFailure, PathState,
    };
    use sf_nav::{
        AgentClassId, AreaId, Goal, GridBackend, GridCell, GridConfig, JobState, NavBackend,
        PathCorridor, PathJob, PathQuery, PolyFlags, QueryFilter, SliceBudget,
    };
    use sf_world::{Location, WorldspaceId};

    let ws = WorldspaceId::new("test_room");
    let (class, area, filter) = (AgentClassId(0), AreaId(1), QueryFilter::default());
    // Walkable for z < 13 m only: the lattice row at z = 14.6 m snaps onto the mesh (2 m extent)
    // and is walked toward (held in place: `NoProgress`), the row at z = 18.3 m is off it (`NoPath`).
    let mut backend = GridBackend::new(GridConfig { cell_size: 1.0, tile_cells: 48 }).unwrap();
    let (open, wall) = (".".repeat(48), "#".repeat(48));
    let rows: Vec<&str> = (0..48).map(|z| if z < 13 { open.as_str() } else { wall.as_str() }).collect();
    backend
        .insert_rows(&ws, class, 0.5, &rows, |c| (c == '.').then_some(GridCell::new(0.0, area, PolyFlags::WALK)))
        .unwrap();
    // The 5x5 lattice of the navmesh test, in positive engine X/Z; the actor stands on node 12.
    let (n, step) = (5i32, 256i32);
    let idx = |x: i32, y: i32| usize::try_from(y * n + x).unwrap();
    let mut pts = Vec::new();
    let mut edges = Vec::new();
    for y in 0..n {
        for x in 0..n {
            pts.push([(x + 1) * step, -(y + 1) * step, 0]);
            if x + 1 < n {
                edges.extend([(idx(x, y), idx(x + 1, y)), (idx(x + 1, y), idx(x, y))]);
            }
            if y + 1 < n {
                edges.extend([(idx(x, y), idx(x, y + 1)), (idx(x, y + 1), idx(x, y))]);
            }
        }
    }
    let graph = interior(&grid(&pts, &edges));
    let home = graph.position(12);
    assert_eq!(graph.node_at(home), Some(12), "standing on a node: the old code blocked pathgrid edges");

    let mut driver = WanderDriver::with_vocab(
        WanderPlanner::new(no_idles(800.0), home),
        2.0,
        5,
        ws.clone(),
        std::sync::Arc::new(WanderVocabulary::build()),
    );
    let cfg = ExecConfig::default();
    let same_goal_m = driver.tuning.same_goal_m;
    let mut nav_state = NavMoveState::Idle;
    let mut loaded: Option<WorldPos> = None;
    let mut walking_to: Option<WorldPos> = None;
    let mut abandoned: Option<WorldPos> = None;
    let (mut no_path, mut no_progress, mut checked) = (0, 0, 0);
    for tick in 0..20_000u64 {
        let ctx = ExecContext {
            location: Location::new(ws.clone(), home),
            yaw: 0.0,
            dt: 0.1,
            nav: nav_state,
        };
        let stuck = driver.stuck_count();
        let out = driver.step_htn(Some(&graph), &ctx, &cfg, None);
        if driver.stuck_count() > stuck {
            match out.path {
                PathState::Failed(PathFailure::NoPath) => no_path += 1,
                PathState::Failed(PathFailure::NoProgress) => no_progress += 1,
                ref p => panic!("tick {tick}: a give-up without a path failure ({p:?})"),
            }
            let goal = walking_to.take().expect("a walk was running");
            assert_eq!(driver.abandoned(), Some(goal), "tick {tick}: the failed goal is remembered");
            assert!(driver.planner.blocked.is_empty(), "tick {tick}: {:?}", driver.planner.blocked);
            assert!(driver.planner.blocked_starts.is_empty(), "tick {tick}: {:?}", driver.planner.blocked_starts);
            abandoned = Some(goal);
        }
        match out.nav {
            NavCommand::MoveTo { goal, .. } => {
                if loaded != Some(goal.pos) {
                    loaded = Some(goal.pos);
                    if let Some(gone) = abandoned.take() {
                        let d = flat_dist(goal.pos, gone);
                        assert!(d >= same_goal_m, "tick {tick}: next goal {d} m from the abandoned one");
                        checked += 1;
                    }
                    let q = PathQuery::new(class, Location::new(ws.clone(), home), Goal::Point(goal.clone()), filter.clone());
                    let mut job = PathJob::new(q);
                    nav_state = match backend.find_path(&mut job, &mut SliceBudget::new(4096)) {
                        JobState::Done(path) => {
                            PathCorridor::new(class, filter.clone()).load(goal.clone(), &path).unwrap();
                            NavMoveState::Following
                        }
                        JobState::Failed(_) => NavMoveState::Failed,
                        JobState::InProgress => panic!("budget too small"),
                    };
                }
                walking_to = Some(goal.pos);
            }
            NavCommand::Stop | NavCommand::None => {
                loaded = None;
                nav_state = NavMoveState::Idle;
            }
        }
        if no_path >= 3 && no_progress >= 3 && abandoned.is_none() {
            break;
        }
    }
    assert!(no_path >= 3 && no_progress >= 3, "{no_path} NoPath, {no_progress} NoProgress give-ups");
    assert!(checked >= 5, "{checked} walks after a give-up");
    assert_eq!(driver.replan_count(), 0, "navmesh give-ups never resume a pathgrid route");
}
