//! Exact tests on synthetic grids (nav.md "Build plan" NV1).
#![allow(
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    clippy::too_many_lines,
    reason = "exact grid arithmetic on small synthetic grids: halves and eighths of a meter"
)]

use std::f32::consts::SQRT_2;

use glam::{DVec3, IVec2, Vec2, Vec3};
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::*;

const H: AgentClassId = AgentClassId(0);
const L: AgentClassId = AgentClassId(1);
const GROUND: AreaId = AreaId(1);
const WATER: AreaId = AreaId(2);
const DOOR: AreaId = AreaId(3);

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(
        ws(),
        WorldPos::from_f64(DVec3::new(x.into(), y.into(), z.into())),
    )
}

/// The centre of cell (x, z) at height 0.
fn at(x: i32, z: i32) -> Location {
    loc(x as f32 + 0.5, 0.0, z as f32 + 0.5)
}

fn table() -> AreaTable {
    let mut water = [1.0; MAX_CLASSES];
    water[H.index()] = 10.0;
    AreaTable::new(vec![
        AreaDef {
            id: GROUND,
            name: "ground".into(),
            cost: [1.0; MAX_CLASSES],
            flags: PolyFlags::WALK,
        },
        AreaDef {
            id: WATER,
            name: "water".into(),
            cost: water,
            flags: PolyFlags::SWIM,
        },
        AreaDef {
            id: DOOR,
            name: "door".into(),
            cost: [1.0; MAX_CLASSES],
            flags: PolyFlags::DOOR,
        },
    ])
    .unwrap()
}

/// `.` ground, `~` water, `d` door, a digit `k` ground at `k × 0.25` m, anything else solid.
fn legend(c: char) -> Option<GridCell> {
    let t = table();
    let cell = |y, a| Some(GridCell::new(y, a, t.flags(a)));
    match c {
        '.' => cell(0.0, GROUND),
        '~' => cell(0.0, WATER),
        'd' => cell(0.0, DOOR),
        '0'..='9' => cell(c.to_digit(10).unwrap() as f32 * 0.25, GROUND),
        _ => None,
    }
}

fn grid_for(classes: &[AgentClassId], rows: &[&str]) -> GridBackend {
    let mut g = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    for &c in classes {
        g.insert_rows(&ws(), c, 0.5, rows, legend).unwrap();
    }
    g
}

fn grid(rows: &[&str]) -> GridBackend {
    grid_for(&[H], rows)
}

fn query(class: AgentClassId, from: Location, goal: Goal) -> PathQuery {
    PathQuery::new(class, from, goal, QueryFilter::for_class(&table(), class))
}

fn cells(g: &GridBackend, r: &PathResult) -> Vec<(i32, i32)> {
    r.polys
        .iter()
        .map(|p| g.cell_of(*p).map(|c| (c.x, c.y)).unwrap())
        .collect()
}

fn row(xs: std::ops::RangeInclusive<i32>, z: i32) -> Vec<(i32, i32)> {
    xs.map(|x| (x, z)).collect()
}

fn rev_row(xs: std::ops::RangeInclusive<i32>, z: i32) -> Vec<(i32, i32)> {
    xs.rev().map(|x| (x, z)).collect()
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn straight_path_across_tiles() {
    let g = grid(&["......"]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(at(5, 0))))
        .unwrap();
    assert_eq!(r.status, PathStatus::Complete);
    assert_eq!(cells(&g, &r), row(0..=5, 0));
    assert!(close(r.cost, 5.0));
    assert_eq!(r.waypoints, (0..=5).map(|x| at(x, 0)).collect::<Vec<_>>());
}

#[test]
fn snake_maze_is_exact() {
    let g = grid(&[".....", "####.", ".....", ".####", "....."]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(at(4, 4))))
        .unwrap();
    let mut want = row(0..=4, 0);
    want.push((4, 1));
    want.extend(rev_row(0..=4, 2));
    want.push((0, 3));
    want.extend(row(0..=4, 4));
    assert_eq!(cells(&g, &r), want);
    assert!(close(r.cost, 16.0));
}

#[test]
fn wall_gap_without_corner_cutting() {
    let g = grid(&["....#....", "....#....", ".........", "....#...."]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(at(8, 0))))
        .unwrap();
    let c = cells(&g, &r);
    assert_eq!(c.len(), 9);
    assert!(c.windows(3).any(|w| w == [(3, 2), (4, 2), (5, 2)]), "{c:?}");
    assert!(close(r.cost, 4.0 * SQRT_2 + 4.0), "{}", r.cost);
}

#[test]
fn area_costs_per_class_change_the_route() {
    let rows = ["..........", "~~~~~~~~~.", "~~~~~~~~~.", ".........."];
    let g = grid_for(&[H, L], &rows);
    // Humanoid: water costs 10, the bridge at x = 9 is cheaper.
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(at(0, 3))))
        .unwrap();
    let mut want = row(0..=8, 0);
    want.extend([(9, 1), (9, 2)]);
    want.extend(rev_row(0..=8, 3));
    assert_eq!(cells(&g, &r), want);
    assert!(close(r.cost, 17.0 + 2.0 * SQRT_2), "{}", r.cost);
    // Large: water costs 1, straight across.
    let r = g
        .find_path_now(query(L, at(0, 0), Goal::Point(at(0, 3))))
        .unwrap();
    assert_eq!(cells(&g, &r), vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    assert!(close(r.cost, 3.0));
    // A non-swimmer excludes SWIM: around, and no diagonal past a water corner.
    let walker = AgentClass {
        id: L,
        radius: 0.5,
        height: 2.0,
        max_climb: 0.5,
        max_slope_deg: 45.0,
        can_swim: false,
    };
    let q = PathQuery::new(
        L,
        at(0, 0),
        Goal::Point(at(0, 3)),
        QueryFilter::for_agent(&table(), &walker),
    );
    let r = g.find_path_now(q).unwrap();
    let mut want = row(0..=9, 0);
    want.extend([(9, 1), (9, 2)]);
    want.extend(rev_row(0..=9, 3));
    assert_eq!(cells(&g, &r), want);
    assert!(close(r.cost, 21.0));
    // An excluded area (infinite cost) is the same as excluding its flag.
    let mut f = QueryFilter::for_class(&table(), L);
    f.set_area_cost(WATER, f32::INFINITY);
    let r2 = g
        .find_path_now(PathQuery::new(L, at(0, 0), Goal::Point(at(0, 3)), f))
        .unwrap();
    assert_eq!(r2.polys, r.polys);
}

#[test]
fn exclude_and_include_flags() {
    let g = grid(&["..........", "##d####.##", ".........."]);
    let base = query(H, at(2, 0), Goal::Point(at(2, 2)));
    let r = g.find_path_now(base.clone()).unwrap();
    assert_eq!(cells(&g, &r), vec![(2, 0), (2, 1), (2, 2)]);
    let mut q = base;
    q.filter = q.filter.excluding(PolyFlags::DOOR);
    let r = g.find_path_now(q).unwrap();
    let mut want = row(2..=7, 0);
    want.push((7, 1));
    want.extend(rev_row(2..=7, 2));
    assert_eq!(cells(&g, &r), want);
    assert!(close(r.cost, 12.0));

    // Include SWIM only: the ground cell at (2, 0) is out, and so are diagonals past it.
    let g = grid_for(&[L], &["~~.~~", "~~~~~"]);
    let all = g
        .find_path_now(query(L, at(0, 0), Goal::Point(at(4, 0))))
        .unwrap();
    assert_eq!(cells(&g, &all), row(0..=4, 0));
    let mut q = query(L, at(0, 0), Goal::Point(at(4, 0)));
    q.filter = q.filter.including(PolyFlags::SWIM);
    let r = g.find_path_now(q).unwrap();
    assert_eq!(cells(&g, &r), vec![(0, 0), (1, 1), (2, 1), (3, 1), (4, 0)]);
    assert!(close(r.cost, 2.0 + 2.0 * SQRT_2));
}

#[test]
fn unreachable_goal_gives_partial_path() {
    let g = grid(&["...#..."]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(at(6, 0))))
        .unwrap();
    assert_eq!(r.status, PathStatus::Partial);
    assert_eq!(cells(&g, &r), row(0..=2, 0));
    assert!(close(r.cost, 2.0));
    assert_eq!(r.waypoints.last(), Some(&at(2, 0)));
    // Off the mesh altogether.
    assert_eq!(
        g.find_path_now(query(H, at(0, 0), Goal::Point(at(30, 30)))),
        Err(NavError::GoalOffMesh)
    );
    assert_eq!(
        g.find_path_now(query(H, at(30, 30), Goal::Point(at(0, 0)))),
        Err(NavError::StartOffMesh)
    );
    let other = Location::new(WorldspaceId::new("elsewhere"), WorldPos::ORIGIN);
    assert!(matches!(
        g.find_path_now(query(H, at(0, 0), Goal::Point(other))),
        Err(NavError::WrongWorldspace { .. })
    ));
}

#[test]
fn climb_limits_steps() {
    let g = grid(&["0020"]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(loc(3.5, 0.0, 0.5))))
        .unwrap();
    assert_eq!(r.status, PathStatus::Complete);
    assert!(close(r.cost, 1.0 + 2.0 * 1.25_f32.sqrt()), "{}", r.cost);
    let g = grid(&["0030"]);
    let r = g
        .find_path_now(query(H, at(0, 0), Goal::Point(loc(3.5, 0.0, 0.5))))
        .unwrap();
    assert_eq!(r.status, PathStatus::Partial);
    assert_eq!(cells(&g, &r), vec![(0, 0), (1, 0)]);
}

#[test]
fn nearest_snaps_within_extent() {
    let g = grid(&["..", ".#"]);
    let f = QueryFilter::for_class(&table(), H);
    let p = g
        .nearest(&loc(1.7, 0.3, 1.6), H, DEFAULT_EXTENT, &f)
        .unwrap();
    assert_eq!(g.cell_of(p.poly), Some(IVec2::new(1, 0)));
    assert_eq!(p.loc, loc(1.7, 0.0, 1.0));
    // Over a walkable cell: straight down onto it.
    let p = g
        .nearest(&loc(0.25, 1.0, 0.75), H, DEFAULT_EXTENT, &f)
        .unwrap();
    assert_eq!(
        (g.cell_of(p.poly), p.loc),
        (Some(IVec2::ZERO), loc(0.25, 0.0, 0.75))
    );
    assert_eq!(
        g.nearest(&loc(10.0, 0.0, 10.0), H, DEFAULT_EXTENT, &f),
        None
    );
    assert_eq!(g.nearest(&loc(0.5, 5.0, 0.5), H, DEFAULT_EXTENT, &f), None);
    assert_eq!(
        g.nearest(&loc(0.5, 0.0, 0.5), L, DEFAULT_EXTENT, &f),
        None,
        "no tiles for that class"
    );
    assert_eq!(
        g.nearest(
            &loc(0.5, 0.0, 0.5),
            H,
            DEFAULT_EXTENT,
            &f.excluding(PolyFlags::WALK)
        ),
        None
    );
}

#[test]
fn sliced_search_equals_one_shot() {
    let g = grid(&[".....", "####.", ".....", ".####", "....."]);
    let q = query(H, at(0, 0), Goal::Point(at(4, 4)));
    let mut one = PathJob::new(q.clone());
    let JobState::Done(want) = g.find_path(&mut one, &mut SliceBudget::unlimited()) else {
        panic!()
    };
    let n = one.iterations();
    assert!(n > 10);

    let mut job = PathJob::new(q.clone());
    assert_eq!(
        g.find_path(&mut job, &mut SliceBudget::new(0)),
        JobState::InProgress
    );
    assert_eq!(job.iterations(), 0);
    let mut steps = 0;
    let got = loop {
        steps += 1;
        let mut b = SliceBudget::new(1);
        match g.find_path(&mut job, &mut b) {
            JobState::InProgress => assert_eq!(b.remaining(), 0),
            JobState::Done(r) => break r,
            JobState::Failed(e) => panic!("{e}"),
        }
    };
    assert_eq!(got, want);
    assert_eq!((steps, job.iterations()), (n, n));
    assert_eq!(
        g.find_path(&mut job, &mut SliceBudget::unlimited()),
        JobState::Failed(NavError::JobFinished)
    );

    // One tick budget shared by several jobs.
    let mut tick = SliceBudget::new(n + 3);
    let (mut a, mut b) = (PathJob::new(q.clone()), PathJob::new(q));
    assert!(matches!(g.find_path(&mut a, &mut tick), JobState::Done(_)));
    assert_eq!(g.find_path(&mut b, &mut tick), JobState::InProgress);
    assert_eq!((tick.remaining(), b.iterations()), (0, 3));
}

#[test]
fn removing_a_tile_fails_inflight_jobs_adding_does_not() {
    let mut g = grid(&["........"]);
    let q = query(H, at(0, 0), Goal::Point(at(7, 0)));
    let mut job = PathJob::new(q.clone());
    assert_eq!(
        g.find_path(&mut job, &mut SliceBudget::new(1)),
        JobState::InProgress
    );
    g.insert_rows(&WorldspaceId::new("other"), H, 0.5, &["...."], legend)
        .unwrap();
    assert_eq!(
        g.find_path(&mut job, &mut SliceBudget::new(1)),
        JobState::InProgress
    );
    let key = TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x: 1,
        z: 0,
    };
    assert!(g.tile(&key).is_some());
    assert!(g.remove_tile(&key));
    assert!(!g.remove_tile(&key));
    assert_eq!(g.tile(&key), None);
    assert_eq!(
        g.find_path(&mut job, &mut SliceBudget::unlimited()),
        JobState::Failed(NavError::MeshChanged)
    );
    // A fresh query sees the shorter mesh: the goal's tile is gone.
    assert_eq!(g.find_path_now(q), Err(NavError::GoalOffMesh));
}

#[test]
fn goal_kinds() {
    let g = grid(&["........."]);
    let near = g
        .find_path_now(query(
            H,
            at(0, 0),
            Goal::Near {
                at: at(8, 0),
                radius: 3.0,
            },
        ))
        .unwrap();
    assert_eq!(
        (near.status, cells(&g, &near)),
        (PathStatus::Complete, row(0..=5, 0))
    );
    assert_eq!(near.waypoints.last(), Some(&at(5, 0)));

    let away = g
        .find_path_now(query(
            H,
            at(4, 0),
            Goal::Away {
                from: at(3, 0),
                min_dist: 3.0,
            },
        ))
        .unwrap();
    assert_eq!(
        (away.status, cells(&g, &away)),
        (PathStatus::Complete, row(4..=6, 0))
    );
    assert!(close(away.cost, 2.0));

    let any = g
        .find_path_now(query(H, at(5, 0), Goal::AnyOf(vec![at(0, 0), at(7, 0)])))
        .unwrap();
    assert_eq!(cells(&g, &any), row(5..=7, 0));
    assert_eq!(any.waypoints.last(), Some(&at(7, 0)));
    assert_eq!(
        g.find_path_now(query(H, at(5, 0), Goal::AnyOf(vec![]))),
        Err(NavError::EmptyGoal)
    );

    // Already there: one polygon, start and goal points.
    let here = g
        .find_path_now(query(H, at(2, 0), Goal::Point(loc(2.25, 0.0, 0.75))))
        .unwrap();
    assert_eq!(cells(&g, &here), vec![(2, 0)]);
    assert_eq!(here.waypoints, vec![at(2, 0), loc(2.25, 0.0, 0.75)]);

    // Nowhere far enough: the farthest cell, partial.
    let g = grid(&["..."]);
    let r = g
        .find_path_now(query(
            H,
            at(1, 0),
            Goal::Away {
                from: at(1, 0),
                min_dist: 10.0,
            },
        ))
        .unwrap();
    assert_eq!(
        (r.status, cells(&g, &r)),
        (PathStatus::Partial, vec![(1, 0), (0, 0)])
    );
}

#[test]
fn door_state_patch_reroutes_without_rebuild() {
    let mut g = grid(&["..........", "##d####.##", ".........."]);
    let door = g.poly_at(&ws(), H, IVec2::new(2, 1)).unwrap();
    assert_eq!(
        g.poly_state(door),
        Some(PolyState {
            area: DOOR,
            flags: PolyFlags::DOOR
        })
    );
    let q = query(H, at(2, 0), Goal::Point(at(2, 2)));
    let mut inflight = PathJob::new(q.clone());
    assert_eq!(
        g.find_path(&mut inflight, &mut SliceBudget::new(1)),
        JobState::InProgress
    );

    let locked = PolyState {
        area: DOOR,
        flags: PolyFlags::DOOR | PolyFlags::DISABLED,
    };
    g.set_poly_state(&[door], locked).unwrap();
    assert_eq!(g.poly_state(door), Some(locked));
    let r = g.find_path_now(q.clone()).unwrap();
    assert!(close(r.cost, 12.0));
    assert!(matches!(
        g.find_path(&mut inflight, &mut SliceBudget::unlimited()),
        JobState::Done(_)
    ));

    // An admitted agent (a filter that doesn't exclude DISABLED) still uses it.
    let mut admitted = q;
    admitted.filter.exclude = PolyFlags::empty();
    assert_eq!(g.find_path_now(admitted).unwrap().polys.len(), 3);

    assert!(g.remove_tile(&TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x: 0,
        z: 0
    }));
    assert_eq!(
        g.set_poly_state(&[door], locked),
        Err(NavError::UnknownPoly(door))
    );
    assert_eq!(g.poly_state(door), None);
}

#[test]
fn raycast_clear_hit_and_corner() {
    let f = QueryFilter::for_class(&table(), H);
    let g = grid(&["........."]);
    let from = g.nearest(&at(0, 0), H, DEFAULT_EXTENT, &f).unwrap();
    let ray = g.raycast(&from, &at(8, 0), &f).unwrap();
    assert_eq!((ray.hit, ray.t, ray.polys.len()), (false, 1.0, 9));
    assert_eq!(ray.end, at(8, 0));

    let g = grid(&["....#...."]);
    let from = g.nearest(&at(0, 0), H, DEFAULT_EXTENT, &f).unwrap();
    let ray = g.raycast(&from, &at(8, 0), &f).unwrap();
    assert_eq!((ray.hit, ray.t, ray.polys.len()), (true, 0.4375, 4));
    assert_eq!(ray.end, loc(4.0, 0.0, 0.5));

    let g = grid(&["...", "#..", "..."]);
    let from = g.nearest(&at(0, 0), H, DEFAULT_EXTENT, &f).unwrap();
    let ray = g.raycast(&from, &at(2, 2), &f).unwrap();
    assert_eq!(
        (ray.hit, ray.t),
        (true, 0.25),
        "a corner past a blocked side stops the ray"
    );
    assert_eq!(ray.end, loc(1.0, 0.0, 1.0));
    let ray = g.raycast(&from, &at(2, 0), &f).unwrap();
    assert!(!ray.hit);
}

fn quad(
    verts: &mut Vec<Vec3>,
    tris: &mut Vec<[u32; 3]>,
    areas: &mut Vec<AreaId>,
    x: [f32; 2],
    z: [f32; 2],
    y: f32,
    area: AreaId,
) {
    let b = verts.len() as u32;
    verts.extend([
        Vec3::new(x[0], y, z[0]),
        Vec3::new(x[1], y, z[0]),
        Vec3::new(x[1], y, z[1]),
        Vec3::new(x[0], y, z[1]),
    ]);
    tris.extend([[b, b + 1, b + 2], [b, b + 2, b + 3]]);
    areas.extend([area, area]);
}

#[test]
fn build_tile_from_triangles() {
    let class = AgentClass {
        id: H,
        radius: 0.3,
        height: 1.8,
        max_climb: 0.5,
        max_slope_deg: 45.0,
        can_swim: true,
    };
    let cfg = BuildConfig {
        class,
        cell_size: 1.0,
        cell_height: 0.1,
        tile_cells: 4,
    };
    let (mut verts, mut tris, mut areas) = (vec![], vec![], vec![]);
    quad(
        &mut verts,
        &mut tris,
        &mut areas,
        [0.0, 4.0],
        [0.0, 4.0],
        0.0,
        GROUND,
    ); // floor
    quad(
        &mut verts,
        &mut tris,
        &mut areas,
        [3.0, 4.0],
        [0.0, 4.0],
        2.0,
        GROUND,
    ); // a ledge at x = 3
    quad(
        &mut verts,
        &mut tris,
        &mut areas,
        [0.0, 1.0],
        [0.0, 1.0],
        0.1,
        AreaId::NULL,
    ); // unwalkable lid
    let b = verts.len() as u32; // a 63° slope over cell (2, 3)
    verts.extend([
        Vec3::new(2.0, 0.0, 3.0),
        Vec3::new(3.0, 0.0, 3.0),
        Vec3::new(2.5, 2.0, 4.0),
    ]);
    tris.push([b, b + 1, b + 2]);
    areas.push(GROUND);
    let door = AreaVolume {
        footprint: vec![
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(1.0, 2.0),
        ],
        min_y: -1.0,
        max_y: 1.0,
        area: DOOR,
    };
    let input = TileInput {
        tile_x: 0,
        tile_z: 0,
        verts,
        tris,
        tri_areas: areas,
        volumes: vec![door],
    };
    let mut g = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    let blob = g.build_tile(&input, &cfg, &table()).unwrap();
    assert_eq!(
        blob,
        g.build_tile(&input, &cfg, &table()).unwrap(),
        "deterministic"
    );

    let key = TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x: 0,
        z: 0,
    };
    let wrong = TileKey {
        x: 1,
        ..key.clone()
    };
    assert!(matches!(
        g.add_tile(wrong, &blob),
        Err(NavError::InvalidTile { .. })
    ));
    assert!(matches!(
        g.add_tile(key.clone(), &[1, 2, 3]),
        Err(NavError::InvalidTile { .. })
    ));
    let t = g.add_tile(key.clone(), &blob).unwrap();
    assert_eq!(g.tile(&key), Some(t));
    assert_eq!(
        g.add_tile(key.clone(), &blob),
        Err(NavError::TileLoaded(Box::new(key)))
    );

    let state = |x, z| {
        g.poly_at(&ws(), H, IVec2::new(x, z))
            .and_then(|p| g.poly_state(p))
    };
    assert_eq!(state(0, 0), None, "NULL area on top");
    assert_eq!(state(2, 3), None, "too steep");
    assert_eq!(
        state(1, 0),
        Some(PolyState {
            area: GROUND,
            flags: PolyFlags::WALK
        })
    );
    assert_eq!(
        state(1, 1),
        Some(PolyState {
            area: DOOR,
            flags: PolyFlags::DOOR
        }),
        "area volume"
    );
    let f = QueryFilter::for_class(&table(), H);
    let ledge = g
        .nearest(&loc(3.5, 2.0, 0.5), H, DEFAULT_EXTENT, &f)
        .unwrap();
    assert_eq!(ledge.loc, loc(3.5, 2.0, 0.5));
    // The ledge is 2 m up: unreachable with a 0.5 m climb.
    let r = g
        .find_path_now(query(H, at(1, 0), Goal::Point(loc(3.5, 2.0, 0.5))))
        .unwrap();
    assert_eq!(r.status, PathStatus::Partial);

    let bad = BuildConfig {
        tile_cells: 8,
        ..cfg
    };
    assert!(matches!(
        g.build_tile(&input, &bad, &table()),
        Err(NavError::InvalidConfig(_))
    ));
}

#[test]
fn area_table_validates() {
    let def = |id, cost| AreaDef {
        id: AreaId(id),
        name: format!("a{id}"),
        cost: [cost; MAX_CLASSES],
        flags: PolyFlags::WALK,
    };
    assert!(AreaTable::new(vec![def(0, 1.0)]).is_err(), "NULL area");
    assert!(AreaTable::new(vec![def(64, 1.0)]).is_err());
    assert!(AreaTable::new(vec![def(2, 1.0), def(2, 3.0)]).is_err());
    assert!(AreaTable::new(vec![def(2, 0.5)]).is_err());
    assert!(AreaTable::new(vec![def(2, f32::NAN)]).is_err());
    let t = AreaTable::new(vec![def(5, 2.0), def(2, 1.0)]).unwrap();
    assert_eq!(t.iter().map(|a| a.id.0).collect::<Vec<_>>(), vec![2, 5]);
    assert_eq!(t.flags(AreaId(9)), PolyFlags::empty());
    assert_eq!(QueryFilter::for_class(&t, H).area_cost(AreaId(5)), 2.0);
    let json = serde_json::to_string(&vec![def(2, 0.5)]).unwrap();
    assert!(
        serde_json::from_str::<AreaTable>(&json).is_err(),
        "deserialization validates too"
    );
}

#[test]
fn filter_rules() {
    let f = QueryFilter::default();
    assert!(f.passes(GROUND, PolyFlags::WALK));
    assert!(!f.passes(GROUND, PolyFlags::empty()), "no flag in include");
    let f = f
        .including(PolyFlags::WALK | PolyFlags::DOOR)
        .excluding(PolyFlags::DISABLED);
    assert!(f.passes(DOOR, PolyFlags::DOOR));
    assert!(!f.passes(DOOR, PolyFlags::DOOR | PolyFlags::DISABLED));
    assert!(!f.passes(WATER, PolyFlags::SWIM));
    assert!(ClassMask::single(L).contains(L) && !ClassMask::single(L).contains(H));
    assert!(ClassMask::ALL.contains(AgentClassId(7)));
}

fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug>(
    v: &T,
    json: bool,
) {
    let bin = postcard::to_allocvec(v).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&bin).unwrap(), v);
    if json {
        let s = serde_json::to_string(v).unwrap();
        assert_eq!(&serde_json::from_str::<T>(&s).unwrap(), v, "{s}");
    }
}

#[test]
fn serde_round_trips() {
    let class = AgentClass {
        id: L,
        radius: 1.0,
        height: 3.0,
        max_climb: 0.6,
        max_slope_deg: 40.0,
        can_swim: false,
    };
    round_trip(&class, true);
    round_trip(&table(), true);
    round_trip(
        &TileKey {
            ws: ws(),
            class: H,
            region: 3,
            x: -4,
            z: 9,
        },
        true,
    );
    round_trip(
        &PolyState {
            area: DOOR,
            flags: PolyFlags::DOOR | PolyFlags::from_bits_retain(0x0100),
        },
        true,
    );
    round_trip(&GridCell::new(1.5, GROUND, PolyFlags::WALK), true);
    round_trip(
        &OffMeshLink {
            id: NavLinkId(0xdead_beef),
            a: at(1, 2),
            b: loc(3.0, -1.0, 2.0),
            radius: 0.5,
            two_way: false,
            kind: TraverseKind::Drop,
            area: GROUND,
            classes: ClassMask::single(H),
        },
        true,
    );
    for goal in [
        Goal::Point(at(1, 1)),
        Goal::Near {
            at: at(2, 2),
            radius: 1.5,
        },
        Goal::Away {
            from: at(0, 0),
            min_dist: 8.0,
        },
        Goal::AnyOf(vec![at(0, 1), at(4, 4)]),
    ] {
        round_trip(&query(H, at(0, 0), goal), true);
    }
    let mut f = QueryFilter::for_class(&table(), L).excluding(PolyFlags::AVOID);
    f.set_area_cost(WATER, f32::INFINITY);
    round_trip(&f, false); // JSON has no infinity; saves and the wire use postcard
    let cfg = BuildConfig {
        class,
        cell_size: 0.5,
        cell_height: 0.1,
        tile_cells: 128,
    };
    round_trip(&cfg, true);
    let volume = AreaVolume {
        footprint: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        min_y: 0.0,
        max_y: 2.0,
        area: DOOR,
    };
    round_trip(
        &TileInput {
            tile_x: 1,
            tile_z: -2,
            verts: vec![Vec3::ZERO, Vec3::X, Vec3::Z],
            tris: vec![[0, 1, 2]],
            tri_areas: vec![GROUND],
            volumes: vec![volume],
        },
        true,
    );
}
