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

#[test]
fn idle_roll_matches_openmw_statistics() {
    let mut rng = fastrand::Rng::with_seed(7);
    let n = 200_000;
    // One idle at 100%: r in 0..133, wins when 0 < r < 75.
    let mut one = [0; 8];
    one[0] = 100;
    let hits = (0..n)
        .filter(|_| roll_idle(&one, 0.75, &mut rng) == Some(2))
        .count();
    let p = f64::from(u32::try_from(hits).unwrap()) / f64::from(n);
    assert!((p - 74.0 / 133.0).abs() < 0.005, "{p}");
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

    // Idle chances win before walking.
    let mut idler = planner.clone();
    idler.wander.idle_chances = [100; 8];
    idler.idle_seconds[3] = 2.5;
    let steps: Vec<_> = (0..500)
        .map(|_| idler.next(Some(&graph), home, &mut rng))
        .collect();
    assert!(
        steps
            .iter()
            .filter(|s| matches!(s, WanderStep::Idle { variant: 2..=9, .. }))
            .count()
            > 450
    );
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

#[test]
fn scaffold_expires_and_repeats_in_game_hours() {
    let graph = interior(&lattice(3, 128));
    let home = graph.position(4);
    let mut wander = no_idles(0.0);
    wander.duration_hours = 2.0;
    wander.repeat = false;
    let mut s = PathWalkScaffold::new(WanderPlanner::new(wander, home), 2.0, 3);
    assert!(!s.update(Some(&graph), home, 0.1, 1.5).expired);
    let out = s.update(Some(&graph), home, 0.1, 1.0);
    assert!(out.expired && out.intent == sf_controller::LocomotionIntent::IDLE);
    wander.repeat = true;
    let mut s = PathWalkScaffold::new(WanderPlanner::new(wander, home), 2.0, 3);
    assert!((0..10).all(|_| !s.update(Some(&graph), home, 0.1, 1.0).expired));
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
    let mut scaffold = PathWalkScaffold::new(WanderPlanner::new(no_idles(2000.0), home), speed, 11);
    let probe = TerrainProbe(Flat);
    let params = MoverParams::default();
    let mut loco = Locomotion::new(home, 0.0);
    let mut mover = KinematicMover::new();
    let dt = 1.0 / 60.0;
    let mut line: Vec<WorldPos> = Vec::new();
    let (mut walks, mut worst_track, mut worst_end) = (0, 0.0f32, 0.0f32);
    let mut was_walking = false;
    for _ in 0..(60 * 90) {
        let out = scaffold.update(Some(&graph), loco.position, dt, 0.0);
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
    let mut s = PathWalkScaffold::new(WanderPlanner::new(no_idles(1024.0), home), 1.5, 7);
    let dt = 0.1;
    // Walking, but held in place (a wall the path doesn't know about).
    let mut walked = 0;
    let mut first_give_up = None;
    for i in 0..200 {
        let out = s.update(Some(&graph), home, dt, 0.0);
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
