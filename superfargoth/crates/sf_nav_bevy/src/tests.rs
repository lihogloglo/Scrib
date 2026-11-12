//! nav.md "Build plan" NV3: headless plugin smoke over a small test bake (synthetic floor, box and
//! ramp through `sf_nav_recast`), gizmo layers on for a few frames. Exact: the `nav.stats` tile
//! counts, the command lines, the BRP answer, the tile and path-point counts the gizmos drew.
//! No visual judgement (rule: owner checks visuals).
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "synthetic layouts in small whole meters"
)]

use bevy::MinimalPlugins;
use glam::{DVec3, Vec3};
use serde_json::json;
use sf_console_bevy::sf_console::Level;
use sf_console_bevy::{ConsolePlugin, ConsoleSubmit, DevConsole};
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, BuildConfig, Goal, GridBackend, GridCell,
    GridConfig, MAX_CLASSES, NavBackend, PathQuery, PathStatus, PolyFlags, QueryFilter, TileInput,
    TileKey,
};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_world::{Location, WorldspaceId};

use super::*;

const H: AgentClassId = AgentClassId(0);
const L: AgentClassId = AgentClassId(1);
const GROUND: AreaId = AreaId(1);

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn loc(x: f32, y: f32, z: f32) -> Location {
    Location::new(
        ws(),
        WorldPos::from_f64(DVec3::new(x.into(), y.into(), z.into())),
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

/// 0.2 m voxels, 40 per tile: 8 m tiles (as `sf_nav_recast`'s tests).
fn cfg(id: AgentClassId) -> BuildConfig {
    BuildConfig {
        class: AgentClass {
            id,
            radius: if id == H { 0.3 } else { 0.8 },
            height: if id == H { 1.8 } else { 2.5 },
            max_climb: 0.4,
            max_slope_deg: 45.0,
            can_swim: false,
        },
        cell_size: 0.2,
        cell_height: 0.1,
        tile_cells: 40,
    }
}

/// A triangle soup: floor `0..16 × 0..16`, a 2 m box at x 3..5, z 3..5 and a ramp up to a
/// platform at x 10..15, z 10..14.
fn soup() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let mut verts = Vec::new();
    let mut tris = Vec::new();
    let mut quad = |q: [Vec3; 4]| {
        let n = verts.len() as u32;
        verts.extend(q);
        tris.extend([[n, n + 1, n + 2], [n, n + 2, n + 3]]);
    };
    // Corners (x0,z0) (x0,z1) (x1,z1) (x1,z0): Recast's +Y winding.
    let flat = |x0: f32, z0: f32, x1: f32, z1: f32, y0: f32, y1: f32| {
        [
            Vec3::new(x0, y0, z0),
            Vec3::new(x0, y0, z1),
            Vec3::new(x1, y1, z1),
            Vec3::new(x1, y1, z0),
        ]
    };
    quad(flat(0.0, 0.0, 16.0, 16.0, 0.0, 0.0));
    // The box: top and four walls.
    let (a, b) = (Vec3::new(3.0, 0.0, 3.0), Vec3::new(5.0, 2.0, 5.0));
    quad(flat(a.x, a.z, b.x, b.z, b.y, b.y));
    let v = Vec3::new;
    quad([
        v(a.x, a.y, a.z),
        v(a.x, b.y, a.z),
        v(b.x, b.y, a.z),
        v(b.x, a.y, a.z),
    ]);
    quad([
        v(a.x, a.y, b.z),
        v(b.x, a.y, b.z),
        v(b.x, b.y, b.z),
        v(a.x, b.y, b.z),
    ]);
    quad([
        v(a.x, a.y, a.z),
        v(a.x, a.y, b.z),
        v(a.x, b.y, b.z),
        v(a.x, b.y, a.z),
    ]);
    quad([
        v(b.x, a.y, a.z),
        v(b.x, b.y, a.z),
        v(b.x, b.y, b.z),
        v(b.x, a.y, b.z),
    ]);
    // Ramp 10..13 rising to 1 m, platform 13..15.
    quad(flat(10.0, 10.0, 13.0, 14.0, 0.0, 1.0));
    quad(flat(13.0, 10.0, 15.0, 14.0, 1.0, 1.0));
    (verts, tris)
}

/// The test bake: class H tiles `0..2 × 0..2`, class L tile (0, 0). Returns the backend.
fn bake() -> RecastBackend {
    let mut b = RecastBackend::new(RecastConfig::new(vec![cfg(H), cfg(L)])).unwrap();
    let (verts, tris) = soup();
    let tiles = [(H, 0, 0), (H, 1, 0), (H, 0, 1), (H, 1, 1), (L, 0, 0)];
    for (class, x, z) in tiles {
        let input = TileInput {
            tile_x: x,
            tile_z: z,
            verts: verts.clone(),
            tris: tris.clone(),
            tri_areas: vec![GROUND; tris.len()],
            volumes: Vec::new(),
        };
        let blob = b.build_tile(&input, &cfg(class), &table()).unwrap();
        assert!(!blob.is_empty(), "tile {class:?} ({x}, {z}) has polygons");
        b.add_tile(
            TileKey {
                ws: ws(),
                class,
                region: 0,
                x,
                z,
            },
            &blob,
        )
        .unwrap();
    }
    b
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin, NavPlugin));
    #[cfg(feature = "debug")]
    {
        // The default gizmo group alone (GizmoPlugin's AABB, frustum and skinned-bounds plugins
        // want mesh assets a headless app doesn't have).
        use bevy::asset::AssetApp;
        use bevy::gizmos::AppGizmoBuilder;
        app.add_plugins(bevy::asset::AssetPlugin::default())
            .init_asset::<bevy::gizmos::GizmoAsset>()
            .init_gizmo_group::<bevy::gizmos::config::DefaultGizmoConfigGroup>();
    }
    app.update();
    app
}

fn run(app: &mut App, line: &str) -> Vec<(Level, String)> {
    let before = app.world().resource::<DevConsole>().log().lines().len();
    app.world_mut().write_message(ConsoleSubmit::exec(line));
    app.update();
    app.world()
        .resource::<DevConsole>()
        .log()
        .lines()
        .skip(before)
        .map(|l| (l.level, l.text.clone()))
        .collect()
}

fn info(text: &str) -> Vec<(Level, String)> {
    vec![(Level::Info, text.to_owned())]
}

#[test]
#[allow(clippy::too_many_lines, reason = "one smoke scenario, step by step")]
fn smoke_over_a_test_bake() {
    let backend = bake();
    let polys: usize = backend.tile_summaries().iter().map(|t| t.polys).sum();
    let path = backend
        .find_path_now(PathQuery::new(
            H,
            loc(1.0, 0.0, 1.0),
            Goal::Point(loc(14.0, 1.0, 12.0)),
            QueryFilter::for_class(&table(), H),
        ))
        .unwrap();
    assert_eq!(path.status, PathStatus::Complete);
    let mut app = app();

    // Before a backend: a plain line, and nothing drawn.
    assert_eq!(
        run(&mut app, "nav.stats"),
        info("nav: no navmesh backend (0 tiles)")
    );

    app.insert_resource(NavMeshes(Box::new(backend)));
    app.world_mut()
        .resource_mut::<NavDebugPaths>()
        .record("guard", &path);
    let stats = NavStats::of(app.world());
    assert_eq!((stats.tiles, stats.polys), (5, polys));
    assert_eq!(
        stats
            .classes
            .iter()
            .map(|c| (c.class, c.tiles))
            .collect::<Vec<_>>(),
        [(0, 4), (1, 1)]
    );
    let mib = format!("{:.2}", stats.mib);
    assert_eq!(
        run(&mut app, "nav.stats"),
        info(&format!(
            "nav: 5 tiles (class 0: 4, class 1: 1), {polys} polys, {mib} MiB, queue n/a, tick n/a ms, rebuilds n/a"
        ))
    );

    // Layers on, a few frames.
    let note = if cfg!(feature = "debug") {
        ""
    } else {
        " (no gizmos: sf_nav_bevy is built without feature `debug`)"
    };
    assert_eq!(
        run(&mut app, "nav.draw tiles"),
        info(&format!("nav.draw tiles on (5 resident){note}"))
    );
    assert_eq!(
        run(&mut app, "nav.draw paths guard"),
        info(&format!("nav.draw paths on (actor guard){note}"))
    );
    for _ in 0..3 {
        app.update();
    }
    let drawn = *app.world().resource::<NavDrawn>();
    if cfg!(feature = "debug") {
        assert_eq!(
            drawn,
            NavDrawn {
                tiles: 5,
                polys,
                path_points: path.waypoints.len()
            }
        );
        assert!(drawn.path_points >= 2);
    } else {
        assert_eq!(drawn, NavDrawn::default());
    }

    // Another actor's path: nothing recorded, so no points; the tiles stay.
    run(&mut app, "nav.draw paths nobody");
    app.update();
    assert_eq!(app.world().resource::<NavDrawn>().path_points, 0);

    // Only another worldspace shown: nothing drawn.
    app.world_mut().resource_mut::<NavDrawFrame>().worldspace =
        Some(WorldspaceId::new("elsewhere"));
    app.update();
    assert_eq!(app.world().resource::<NavDrawn>().tiles, 0);

    // Layers not built yet name their slice.
    for (layer, slice) in [
        ("links", "NV9"),
        ("doors", "NV4"),
        ("obstacles", "NV10"),
        ("boundary", "NV7b"),
        ("coarse", "NV12"),
        ("graph", "NV11"),
    ] {
        assert_eq!(
            run(&mut app, &format!("nav.draw {layer}")),
            [(
                Level::Warn,
                format!("nav.draw {layer}: not built yet ({slice})")
            )]
        );
    }

    // Off clears every layer; the next frame draws nothing.
    assert_eq!(
        run(&mut app, "nav.draw off"),
        info("nav.draw: every layer off")
    );
    app.update();
    assert_eq!(*app.world().resource::<NavDraw>(), NavDraw::default());
    assert_eq!(*app.world().resource::<NavDrawn>(), NavDrawn::default());

    // Counters a later slice fills replace n/a.
    *app.world_mut().resource_mut::<NavCounters>() = NavCounters {
        queue_depth: Some(3),
        tick_ms: Some(0.25),
        rebuilds: Some(0),
    };
    let line = run(&mut app, "nav.stats");
    assert_eq!(line.len(), 1);
    assert!(
        line[0].1.ends_with(", queue 3, tick 0.25 ms, rebuilds 0"),
        "{line:?}"
    );

    // BRP: the same as JSON.
    let answer = brp_stats_system(In(None), app.world()).unwrap();
    assert_eq!(answer["backend"], json!(true));
    assert_eq!(answer["tiles"], json!(5));
    assert_eq!(answer["polys"], json!(polys));
    assert_eq!(answer["queue_depth"], json!(3));
    assert_eq!(answer["classes"][0]["class"], json!(0));
    assert_eq!(answer["classes"][0]["tiles"], json!(4));
    assert_eq!(answer["classes"][1]["tiles"], json!(1));
}

#[test]
fn tile_geometry_is_finite_and_in_the_tile() {
    let backend = bake();
    for t in backend.tile_summaries() {
        let geo = backend.tile_geometry(&t.key).unwrap();
        assert_eq!(geo.polys.len(), t.polys);
        assert!(geo.polys.iter().all(|p| (3..=6).contains(&p.verts.len())));
        assert!(
            geo.polys.iter().any(|p| p.walls != 0),
            "a tile has outer edges"
        );
        let (x0, z0) = (t.key.x as f32 * 8.0, t.key.z as f32 * 8.0);
        for v in &geo.verts {
            assert!(v.is_finite());
            assert!(
                (x0 - 0.01..=x0 + 8.01).contains(&v.x) && (z0 - 0.01..=z0 + 8.01).contains(&v.z),
                "{v}"
            );
            assert!((-0.5..=2.5).contains(&v.y), "{v}");
        }
    }
}

#[test]
fn grid_double_lists_and_draws_its_cells() {
    let mut grid = GridBackend::new(GridConfig {
        cell_size: 1.0,
        tile_cells: 4,
    })
    .unwrap();
    // Two tiles along x; `#` is a hole.
    grid.insert_rows(&ws(), H, 0.4, &["......", ".#....", "......"], |c| {
        (c == '.').then(|| GridCell::new(0.0, GROUND, PolyFlags::WALK))
    })
    .unwrap();
    let mut app = app();
    app.insert_resource(NavMeshes(Box::new(grid)));
    let stats = NavStats::of(app.world());
    assert_eq!((stats.tiles, stats.polys), (2, 17));
    run(&mut app, "nav.draw tiles");
    app.update();
    if cfg!(feature = "debug") {
        assert_eq!(
            *app.world().resource::<NavDrawn>(),
            NavDrawn {
                tiles: 2,
                polys: 17,
                path_points: 0
            }
        );
    }
    let meshes = app.world().resource::<NavMeshes>();
    let key = TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x: 0,
        z: 0,
    };
    let geo = meshes.0.tile_geometry(&key).unwrap();
    // Cell (0, 0): walls on -x and -z (outside the rows), open on +z and +x.
    assert_eq!(geo.polys[0].walls, 0b1001);
    // Cell (0, 1) borders the hole at (1, 1) on +x.
    assert_eq!(geo.polys[4].walls, 0b0101);
}
