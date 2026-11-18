//! Unit tests for [`PathCorridor`] on [`GridBackend`] (`docs/specs/nav.md`, NV8).

use glam::DVec3;
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::{
    AgentClassId, AreaDef, AreaId, AreaTable, CorridorConfig, Goal, GridBackend, GridCell,
    GridConfig, MAX_CLASSES, NavBackend, PathCorridor, PathQuery, PathResult, PathStatus,
    PolyFlags, PolyState, QueryFilter, TileKey,
};

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn loc(x: f32, z: f32) -> Location {
    Location::new(
        ws(),
        WorldPos::from_f64(DVec3::new(f64::from(x), 0.0, f64::from(z))),
    )
}

fn table() -> AreaTable {
    AreaTable::new(vec![AreaDef {
        id: GROUND,
        name: "ground".into(),
        cost: [1.0; MAX_CLASSES],
        flags: PolyFlags::WALK,
    }])
    .unwrap()
}

fn filter() -> QueryFilter {
    QueryFilter::for_class(&table(), H)
}

fn grid_from_rows(rows: &[&str]) -> GridBackend {
    let mut g = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    let t = table();
    g.insert_rows(&ws(), H, 0.5, rows, |c| {
        (c == '.').then(|| GridCell::new(0.0, GROUND, t.flags(GROUND)))
    })
    .unwrap();
    g
}

#[test]
fn stepping_along_corridor_trims_visited_polygons_and_updates_corners() {
    // A U-shaped corridor around a wall block: 8x8 grid with a wall in rows 0..5.
    let g = grid_from_rows(&[
        ".######.",
        ".######.",
        ".######.",
        ".######.",
        ".######.",
        "........",
        "........",
        "........",
    ]);
    let start = loc(0.5, 0.5);
    let goal = loc(7.5, 0.5);
    let path = g
        .find_path_now(PathQuery::new(H, start.clone(), Goal::Point(goal.clone()), filter()))
        .unwrap();
    assert_eq!(path.status, PathStatus::Complete);
    let initial_len = path.polys.len();
    assert!(initial_len >= 6, "{initial_len}");

    let mut corridor = PathCorridor::from_result(H, filter(), goal.clone(), &path).unwrap();
    let cfg = CorridorConfig {
        max_corners: 1,
        ..CorridorConfig::default()
    };

    // With `max_corners = 1`, the first corner around the wall does not reach the final target, so
    // `stop_at_end` is false.
    let step0 = corridor.step(&g, start, 1.0 / 60.0, &cfg).unwrap();
    assert!(!step0.needs_replan);
    assert!(!step0.stop_at_end);
    assert_eq!(step0.corners.len(), 1);

    // Walk up the left column from z = 0.5 to z = 5.5 in 0.5 m steps: visited cells at the start
    // are trimmed from the front of `corridor.polys()`.
    let cfg = CorridorConfig::default();
    for k in 1..=10u16 {
        let p = loc(0.5, 0.5 + f32::from(k) * 0.5);
        let step = corridor.step(&g, p, 1.0 / 60.0, &cfg).unwrap();
        assert!(!step.needs_replan);
    }
    assert!(
        corridor.polys().len() < initial_len,
        "visited polygons not trimmed: {} vs {initial_len}",
        corridor.polys().len()
    );

    // Step across the top row and down the right column to the goal.
    for x in 1..=7u16 {
        corridor
            .step(&g, loc(0.5 + f32::from(x), 5.5), 1.0 / 60.0, &cfg)
            .unwrap();
    }
    let near_end = corridor.step(&g, loc(7.5, 1.5), 1.0 / 60.0, &cfg).unwrap();
    assert!(!near_end.needs_replan);
    assert!(near_end.stop_at_end);
    assert_eq!(near_end.corners, vec![goal]);
}

#[test]
fn lateral_push_repairs_locally_while_distant_or_unloaded_raises_replan() {
    // 12x8 open floor with adisconnected island on the far right.
    let g = grid_from_rows(&[
        "........#...",
        "........#...",
        "........#...",
        "........#...",
        "........#...",
        "........#...",
        "........#...",
        "........#...",
    ]);
    let start = loc(1.5, 1.5);
    let goal = loc(6.5, 1.5);
    let path = g
        .find_path_now(PathQuery::new(H, start, Goal::Point(goal.clone()), filter()))
        .unwrap();
    let mut corridor = PathCorridor::from_result(H, filter(), goal.clone(), &path).unwrap();
    let cfg = CorridorConfig::default();

    // 1. Lateral push of 2.0 m (> repair_drift_m = 0.75 m) onto an adjacent walkable row:
    // repairs locally via bounded A* back to the corridor without raising `needs_replan`.
    let pushed = loc(1.5, 3.5);
    let step = corridor.step(&g, pushed.clone(), 1.0 / 60.0, &cfg).unwrap();
    assert!(step.repaired, "expected local repair after lateral push");
    assert!(!step.needs_replan, "adjacent push should not need full replan");
    assert_eq!(corridor.pos(), &pushed);
    assert_eq!(corridor.last_poly(), path.polys.last().copied());

    // 2. Push across the wall onto the disconnected island at x = 10.5:
    // local repair cannot reach the corridor, so `needs_replan` is raised.
    let mut disconnected = PathCorridor::from_result(H, filter(), goal.clone(), &path).unwrap();
    let step_disc = disconnected
        .step(&g, loc(10.5, 1.5), 1.0 / 60.0, &cfg)
        .unwrap();
    assert!(!step_disc.repaired);
    assert!(step_disc.needs_replan);

    // 3. Unloading a lookahead tile trims the corridor to the valid prefix and raises `needs_replan`.
    let mut g_unload = g;
    let mut c_unload = PathCorridor::from_result(H, filter(), goal, &path).unwrap();
    assert!(g_unload.remove_tile(&TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x: 1,
        z: 0,
    }));
    let step_unload = c_unload
        .step(&g_unload, loc(1.5, 1.5), 1.0 / 60.0, &cfg)
        .unwrap();
    assert!(step_unload.needs_replan);
    assert!(!c_unload.polys().is_empty());
    assert!(
        c_unload
            .polys()
            .iter()
            .all(|&p| g_unload.is_valid_poly(p, &filter()))
    );
}

#[test]
fn visibility_shortcutting_straightens_open_dogleg_and_respects_walls() {
    // Open 8x8 floor.
    let open = grid_from_rows(&[
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
    ]);
    let start = loc(0.5, 0.5);
    let corner = loc(5.5, 0.5);
    let goal = loc(5.5, 5.5);

    // Construct an artificial L-shaped dogleg corridor `(0.5, 0.5) -> (5.5, 0.5) -> (5.5, 5.5)`.
    let leg1 = open
        .find_path_now(PathQuery::new(H, start.clone(), Goal::Point(corner.clone()), filter()))
        .unwrap();
    let leg2 = open
        .find_path_now(PathQuery::new(H, corner, Goal::Point(goal.clone()), filter()))
        .unwrap();
    let mut dogleg_polys = leg1.polys;
    dogleg_polys.extend_from_slice(&leg2.polys[1..]);
    let dogleg_result = PathResult {
        status: PathStatus::Complete,
        polys: dogleg_polys.clone(),
        waypoints: vec![start.clone(), goal.clone()],
        cost: leg1.cost + leg2.cost,
    };

    // Without visibility optimization (`optimize_range_m = 0`), the L-shaped corridor has an
    // intermediate corner at the bend before the goal.
    let mut unopt =
        PathCorridor::from_result(H, filter(), goal.clone(), &dogleg_result).unwrap();
    let unopt_step = unopt
        .step(
            &open,
            start.clone(),
            1.0 / 60.0,
            &CorridorConfig {
                optimize_range_m: 0.0,
                ..CorridorConfig::default()
            },
        )
        .unwrap();
    assert!(
        unopt_step.corners.len() >= 2,
        "expected bend corner before visibility optimization: {:?}",
        unopt_step.corners
    );

    // With visibility optimization enabled on the open floor, the raycast toward the 2nd corner
    // succeeds and straightens the corridor directly to `goal`.
    let mut opt = PathCorridor::from_result(H, filter(), goal.clone(), &dogleg_result).unwrap();
    let opt_step = opt
        .step(&open, start.clone(), 1.0 / 60.0, &CorridorConfig::default())
        .unwrap();
    assert_eq!(opt_step.corners, vec![goal.clone()]);

    // Now block the interior of the L with disabled cells (a wall): visibility shortcutting must
    // NOT cut across the wall.
    let mut walled = open;
    let wall_poly = walled.poly_at(&ws(), H, glam::IVec2::new(3, 3)).unwrap();
    walled
        .set_poly_state(
            &[wall_poly],
            PolyState {
                area: GROUND,
                flags: PolyFlags::DISABLED,
            },
        )
        .unwrap();
    let mut blocked =
        PathCorridor::from_result(H, filter(), goal, &dogleg_result).unwrap();
    let blocked_step = blocked
        .step(&walled, start, 1.0 / 60.0, &CorridorConfig::default())
        .unwrap();
    assert!(
        blocked_step.corners.len() >= 2,
        "visibility shortcutting cut through a wall: {:?}",
        blocked_step.corners
    );
    assert!(!blocked.polys().contains(&wall_poly));
}

#[test]
fn merge_shortcut_keeps_path_when_furthest_common_is_visited_start() {
    use crate::{PolyRef, merge_shortcut};
    let p = |v: &[u64]| v.iter().map(|&r| PolyRef(r)).collect::<Vec<_>>();

    // Furthest common polygon is `visited[0]` at path index 2 (j == 0, i > 0): Detour
    // `dtMergeCorridorStartShortcut` returns on `req <= 0`, so the path keeps its prefix.
    let mut path = p(&[1, 2, 3, 4, 5]);
    merge_shortcut(&mut path, &p(&[3, 9]), 16);
    assert_eq!(path, p(&[1, 2, 3, 4, 5]));

    // No common polygon: unchanged.
    let mut path = p(&[1, 2, 3]);
    merge_shortcut(&mut path, &p(&[7, 8]), 16);
    assert_eq!(path, p(&[1, 2, 3]));

    // Regular shortcut: furthest common is 4 (path index 3, visited index 2), so the result is
    // `visited[..2]` followed by `path[3..]`.
    let mut path = p(&[1, 2, 3, 4, 5]);
    merge_shortcut(&mut path, &p(&[1, 9, 4]), 16);
    assert_eq!(path, p(&[1, 9, 4, 5]));

    // The polygon cap keeps the front: `max = 3` keeps `[1, 9]` and one path polygon.
    let mut path = p(&[1, 2, 3, 4, 5]);
    merge_shortcut(&mut path, &p(&[1, 9, 4]), 3);
    assert_eq!(path, p(&[1, 9, 4]));
}

#[test]
fn visibility_overshoot_shortcuts_past_a_near_corner_on_open_ground() {
    // Open 10x10 floor; an artificial staircase corridor up column 0, along row 2, up column 3 and
    // along row 5. From (0.5, 0.5) the string-pulled corners are (1, 2), (3, 3), (4, 5), goal: the
    // 2nd corner (3, 3) is 3.54 m away, nearer than the 8 m range. Detour pushes the ray along the
    // same diagonal to 8 m, where it crosses (4, 4) and the corridor's (5, 5), so the corridor is
    // shortcut to (5, 5); a ray stopped at the corner would only reach (3, 3).
    let open = grid_from_rows(&[".........."; 10]);
    let cell = |x: i32, z: i32| open.poly_at(&ws(), H, glam::IVec2::new(x, z)).unwrap();
    let staircase = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 2),
        (2, 2),
        (3, 2),
        (3, 3),
        (3, 4),
        (3, 5),
        (4, 5),
        (5, 5),
        (6, 5),
        (7, 5),
        (8, 5),
    ];
    let start = loc(0.5, 0.5);
    let goal = loc(8.5, 5.5);
    let result = PathResult {
        status: PathStatus::Complete,
        polys: staircase.iter().map(|&(x, z)| cell(x, z)).collect(),
        waypoints: vec![start.clone(), goal.clone()],
        cost: 0.0,
    };
    let cfg = CorridorConfig {
        optimize_range_m: 8.0,
        ..CorridorConfig::default()
    };

    // Without the shortcut, the 2nd corner is the near one at (3, 3).
    let mut unopt = PathCorridor::from_result(H, filter(), goal.clone(), &result).unwrap();
    let unopt_step = unopt
        .step(
            &open,
            start.clone(),
            1.0 / 60.0,
            &CorridorConfig {
                optimize_range_m: 0.0,
                ..cfg
            },
        )
        .unwrap();
    assert_eq!(unopt_step.corners[1], loc(3.0, 3.0));

    let mut opt = PathCorridor::from_result(H, filter(), goal, &result).unwrap();
    opt.step(&open, start, 1.0 / 60.0, &cfg).unwrap();
    let expected: Vec<_> = [
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 5),
        (7, 5),
        (8, 5),
    ]
    .iter()
    .map(|&(x, z)| cell(x, z))
    .collect();
    assert_eq!(opt.polys(), expected.as_slice());
}
