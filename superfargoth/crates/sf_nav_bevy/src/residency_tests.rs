//! nav.md "Build plan" `NV4c`: [`NavRingPlugin`] over a synthetic bake in a temporary `sf_bake`
//! store. A flat 56 × 8 m floor baked into 8 m tiles `(0..7, 0)` through `sf_nav_recast`, tile
//! `(7, 0)` stored empty, the anchor walked along it. Exact: which tiles are resident per frame
//! (cap 2), the drops past the unload radius, a worldspace change, the counters; a `nearest` and
//! a `find_path` query on the loaded tiles.

use std::sync::Arc;

use bevy::MinimalPlugins;
use glam::{DVec3, Vec3};
use sf_bake::{Manifest, ProductKey, Store};
use sf_console_bevy::ConsolePlugin;
use sf_coords::WorldPos;
use sf_nav::{
    AgentClass, AgentClassId, AreaDef, AreaId, AreaTable, BakedTile, BuildConfig, ClassTiling,
    DEFAULT_EXTENT, Goal, MAX_CLASSES, NavBackend, NavError, PathQuery, PathStatus, PolyFlags,
    QueryFilter, RegionFrame, RingLayout, TileInput, TileKey,
};
use sf_nav_bake::{TILES, tile_subject};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_tunables_bevy::{SetBy, Tunables};
use sf_world::{Location, WorldspaceId};

use super::*;

const H: AgentClassId = AgentClassId(0);
const GROUND: AreaId = AreaId(1);
const STRIP_TILES: i32 = 7;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn loc(x: f64, z: f64) -> Location {
    Location::new(ws(), WorldPos::from_f64(DVec3::new(x, 0.0, z)))
}

fn key(x: i32) -> TileKey {
    TileKey {
        ws: ws(),
        class: H,
        region: 0,
        x,
        z: 0,
    }
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

/// 0.2 m voxels, 40 per tile: 8 m tiles.
fn cfg() -> BuildConfig {
    BuildConfig {
        class: AgentClass {
            id: H,
            radius: 0.3,
            height: 1.8,
            max_climb: 0.4,
            max_slope_deg: 45.0,
            can_swim: false,
        },
        cell_size: 0.2,
        cell_height: 0.1,
        tile_cells: 40,
    }
}

/// Bakes the strip into a fresh store: tiles `(0..7, 0)` with polygons, `(7, 0)` empty.
fn bake(dir: &std::path::Path) -> (Store, Manifest) {
    let store = Store::open(dir).unwrap();
    let mut manifest = Manifest::default();
    let builder = RecastBackend::new(RecastConfig::new(vec![cfg()])).unwrap();
    #[allow(clippy::cast_precision_loss, reason = "a few whole tiles")]
    let end = STRIP_TILES as f32 * 8.0;
    let verts = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 8.0),
        Vec3::new(end, 0.0, 8.0),
        Vec3::new(end, 0.0, 0.0),
    ];
    let tris = vec![[0, 1, 2], [0, 2, 3]];
    for x in 0..=STRIP_TILES {
        let bytes = if x < STRIP_TILES {
            let input = TileInput {
                tile_x: x,
                tile_z: 0,
                verts: verts.clone(),
                tris: tris.clone(),
                tri_areas: vec![GROUND; tris.len()],
                volumes: Vec::new(),
            };
            let blob = builder.build_tile(&input, &cfg(), &table()).unwrap();
            assert!(!blob.is_empty(), "tile ({x}, 0) has polygons");
            BakedTile {
                blob,
                doors: Vec::new(),
            }
            .encode()
            .unwrap()
        } else {
            Vec::new()
        };
        let digest = store.put(&bytes, false).unwrap();
        manifest
            .products
            .insert(ProductKey::new(TILES, tile_subject(&key(x))), digest);
    }
    (store, manifest)
}

fn app(store: Store, manifest: Manifest) -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ConsolePlugin,
        NavPlugin,
        NavRingPlugin {
            store,
            manifest: Arc::new(manifest),
            layout: RingLayout {
                classes: vec![ClassTiling::of(&cfg())],
                regions: BTreeMap::default(),
                default_regions: vec![RegionFrame {
                    region: 0,
                    origin: WorldPos::ORIGIN,
                }],
            },
        },
    ));
    app.insert_resource(NavMeshes(Box::new(
        RecastBackend::new(RecastConfig::new(vec![cfg()])).unwrap(),
    )));
    // Small radii and cap for an 8 m tile strip, through the tunables (the bindings own
    // NavRingSettings).
    {
        let mut t = app.world_mut().resource_mut::<Tunables>();
        let load = t.id_of(&RING_LOAD_M).unwrap();
        let unload = t.id_of(&RING_UNLOAD_M).unwrap();
        let cap = t.id_of(&RING_TILES_PER_FRAME).unwrap();
        t.write(load, 16.0, SetBy::CommandLine).unwrap();
        t.write(unload, 24.0, SetBy::CommandLine).unwrap();
        t.write(cap, 2, SetBy::CommandLine).unwrap();
    }
    app.update();
    app
}

fn step(app: &mut App, at: Option<Location>) {
    app.world_mut().resource_mut::<NavAnchor>().0 = at;
    app.update();
}

/// Resident `x`s of row 0, ascending.
fn resident(app: &App) -> Vec<i32> {
    let mut xs: Vec<i32> = app
        .world()
        .resource::<NavMeshes>()
        .0
        .tile_summaries()
        .iter()
        .map(|t| t.key.x)
        .collect();
    xs.sort_unstable();
    xs
}

fn ring(app: &App) -> RingStats {
    app.world().resource::<BakedNavRing>().stats()
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "one walk along the strip, step by step"
)]
fn baked_tiles_follow_the_anchor() {
    let dir = tempfile::tempdir().unwrap();
    let (store, manifest) = bake(dir.path());
    let mut app = app(store, manifest);
    let settings = *app.world().resource::<NavRingSettings>();
    assert_eq!(
        (
            settings.radii.load_m,
            settings.radii.unload_m,
            settings.tiles_per_frame
        ),
        (16.0, 24.0, 2)
    );

    // No anchor yet: nothing loads.
    assert!(resident(&app).is_empty());
    assert_eq!(ring(&app).pending, 0);

    // x = 12 (tile 1), load 16 m: tiles -1..=3 of row 0 (12 m to the farthest; -1 is unbaked),
    // x -1..=3 at z = +-1 (4 m off the row) and x 0..=2 at z = +-2 (12 m off), all unbaked: 21
    // tiles. Cap 2, nearest first: tile 1, then tile 0 (4 m, first in key order).
    step(&mut app, Some(loc(12.0, 4.0)));
    assert_eq!(resident(&app), vec![0, 1]);
    let planned = ring(&app).pending + 2;
    assert_eq!(planned, 21);
    for _ in 0..planned {
        app.update();
    }
    assert_eq!(resident(&app), vec![0, 1, 2, 3]);
    let r = ring(&app);
    assert_eq!(
        (r.pending, r.empty, r.failed, r.loads, r.drops),
        (0, 0, 0, 4, 0)
    );
    assert_eq!(r.resident + r.unbaked, planned);

    // Queries on the loaded tiles.
    {
        let meshes = &app.world().resource::<NavMeshes>().0;
        let filter = QueryFilter::for_class(&table(), H);
        let p = meshes
            .nearest(&loc(12.0, 4.0), H, DEFAULT_EXTENT, &filter)
            .expect("the anchor stands on the mesh");
        assert!(p.loc.pos.delta_f32(loc(12.0, 4.0).pos).length() < 0.5);
        let path = meshes
            .find_path_now(PathQuery::new(
                H,
                loc(2.0, 4.0),
                Goal::Point(loc(30.0, 4.0)),
                filter.clone(),
            ))
            .unwrap();
        assert_eq!(path.status, PathStatus::Complete);
        // Tile 5 isn't resident: a goal there is off the mesh.
        assert!(matches!(
            meshes.find_path_now(PathQuery::new(
                H,
                loc(2.0, 4.0),
                Goal::Point(loc(44.0, 4.0)),
                filter,
            )),
            Err(NavError::GoalOffMesh)
        ));
    }

    // x = 28 (tile 3): tiles 4 and 5 join (4 m, 12 m); tile 0 is 20 m away, past the load
    // radius but within the unload radius: it stays (hysteresis).
    step(&mut app, Some(loc(28.0, 4.0)));
    for _ in 0..16 {
        app.update();
    }
    assert_eq!(resident(&app), vec![0, 1, 2, 3, 4, 5]);

    // x = 44 (tile 5): tile 0 (36 m) and tile 1 (28 m) go; tile 6 joins; tile 7 is empty.
    step(&mut app, Some(loc(44.0, 4.0)));
    for _ in 0..16 {
        app.update();
    }
    assert_eq!(resident(&app), vec![2, 3, 4, 5, 6]);
    assert!(
        app.world()
            .resource::<NavMeshes>()
            .0
            .tile(&key(0))
            .is_none()
    );
    let r = ring(&app);
    assert_eq!((r.empty, r.loads, r.drops, r.pending), (1, 7, 2, 0));

    // nav.stats carries the ring.
    let line = NavStats::of(app.world()).line();
    assert!(line.starts_with("nav: 5 tiles (class 0: 5), "), "{line}");
    assert!(
        line.contains("; ring: 0 pending, 1 empty, ")
            && line.contains(", 7 loaded, 2 dropped, last "),
        "{line}"
    );

    // Another worldspace: every tile of this one goes.
    step(
        &mut app,
        Some(Location::new(WorldspaceId::new("inside"), WorldPos::ORIGIN)),
    );
    assert!(resident(&app).is_empty());
    let r = ring(&app);
    assert_eq!((r.resident, r.empty, r.drops), (0, 0, 7));
}
