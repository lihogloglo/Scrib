//! Tests of `worldgen_roads`: a hand-written `roads.json` spawns one mesh entity per ribbon chunk, draped on the
//! height function in the engine frame; no `roads.json`, no roads; a mod's network from the VFS draws next to the
//! guide's, each chunk tagged with its source, and a bad mod file is skipped.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::camera::primitives::Aabb;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use sf_coords::WorldPos;
use sf_world::WorldspaceId;
use sf_world_bevy::WorldspaceRegistry;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_worldgen::roads::{RibbonParams, RoadNetwork, ribbon};

use super::{GUIDE_SOURCE, RoadTextures, WorldgenRoad, WorldgenRoads, add, networks};

/// One dirt segment, ~600 m (three 256 m chunks), guide frame.
const ROADS: &str = r#"{
  "format": 1,
  "segments": [
    { "id": 7, "class": "local", "surface": "dirt", "width_m": 5.0, "a": 0, "b": 1,
      "points": [[1000, 2000], [1150, 2060], [1300, 2000], [1450, 1950], [1590, 2010]], "crossings": [] }
  ],
  "junctions": [{ "id": 0, "pos": [1000, 2000] }, { "id": 1, "pos": [1590, 2010] }]
}"#;

/// Engine-frame ground: a slope and a wave.
fn ground(x: f64, z: f64) -> f64 {
    12.0 + 0.04 * x + 3.0 * (z / 25.0).sin()
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mw_exterior_roads_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn app(dir: &Path) -> App {
    app_with(dir, None)
}

/// The VFS folder the mod networks of these tests live in.
const MOD_DIR: &str = "worldgen/roads";

/// A test app; `vfs`: the files the app's VFS serves (mods' road networks under [`MOD_DIR`]).
fn app_with(dir: &Path, vfs: Option<&[(&str, &str)]>) -> App {
    let mut app = App::new();
    if let Some(files) = vfs {
        let source = files.iter().fold(sf_vfs::MemSource::new("test-mod"), |s, (path, text)| s.with(path, text.as_bytes().to_vec()));
        app.insert_resource(mw_assets::SharedVfs(Arc::new(sf_vfs::Vfs::builder().mount(source).build())));
    }
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), sf_tunables_bevy::TunablesPlugin))
        .init_asset::<Mesh>()
        .init_asset::<Image>()
        .init_asset::<StandardMaterial>()
        .init_resource::<WorldspaceRegistry>();
    let ws = WorldspaceId::new("worldgen-roads-test");
    WorldspaceRegistry::ensure_in(app.world_mut(), &ws);
    let roads = WorldgenRoads {
        guide_dir: dir.to_owned(),
        mod_roads_dir: Some(MOD_DIR.into()),
        height: Arc::new(ground),
        textures: RoadTextures::default(),
    };
    add(&mut app, &ws, Some(&roads));
    app
}

/// Updates until the build is gone (spawned or skipped), at most a few seconds.
fn run(app: &mut App) {
    for _ in 0..2000 {
        app.update();
        if !app.world().contains_resource::<super::RoadBuild>() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    panic!("the road build never finished");
}

#[test]
fn roads_json_spawns_draped_chunks() {
    let dir = temp_dir("spawn");
    std::fs::write(dir.join("roads.json"), ROADS).unwrap();
    let mut app = app(&dir);
    run(&mut app);

    let net = RoadNetwork::read(&dir).unwrap().unwrap();
    let params = RibbonParams::default();
    let expected = ribbon(&net.segments[0], &params, &|x, y| ground(x, -y)).chunks.len();
    assert!(expected >= 2, "the fixture spans several chunks ({expected})");

    let world = app.world_mut();
    let mut q = world.query::<(&WorldgenRoad, &Mesh3d, &MeshMaterial3d<StandardMaterial>, &Aabb, &CellCoord, &Transform, &ChildOf)>();
    let rows: Vec<_> = q.iter(world).map(|(r, m, mat, a, c, t, p)| (r.clone(), m.0.clone(), mat.0.clone(), *a, *c, *t, p.parent())).collect();
    assert_eq!(rows.len(), expected, "one entity per chunk");
    let root = world.resource::<WorldspaceRegistry>().get(&WorldspaceId::new("worldgen-roads-test")).unwrap().root;
    let meshes = world.resource::<Assets<Mesh>>();
    let materials = world.resource::<Assets<StandardMaterial>>();
    let mut checked = 0;
    for (road, mesh, material, aabb, cell, transform, parent) in rows {
        assert_eq!((&*road.source, road.segment), (GUIDE_SOURCE, 7));
        assert_eq!(parent, root, "a child of the worldspace root");
        let material = materials.get(&material).expect("the road material");
        assert_eq!(material.alpha_mode, AlphaMode::Blend);
        assert!(material.depth_bias > 0.0);
        let mesh = meshes.get(&mesh).expect("the chunk mesh");
        let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("positions")
        };
        assert!(mesh.attribute(Mesh::ATTRIBUTE_COLOR).is_some(), "the edge fade");
        let origin = WorldPos::new(IVec3::new(cell.x, cell.y, cell.z), transform.translation);
        for p in positions {
            let local = Vec3::from(*p);
            let slack = Vec3::splat(1e-3);
            assert!((Vec3::from(aabb.min()) - slack).cmple(local).all() && (Vec3::from(aabb.max()) + slack).cmpge(local).all(), "inside its Aabb");
            let w = origin.to_f64() + local.as_dvec3();
            let want = ground(w.x, w.z) + params.lift_m;
            assert!((w.y - want).abs() < 2e-3, "vertex at ({:.2}, {:.2}): y {:.4}, ground + lift {want:.4}", w.x, w.z, w.y);
            checked += 1;
        }
    }
    assert!(checked > 100, "{checked} vertices checked");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn no_roads_json_spawns_nothing() {
    let dir = temp_dir("none");
    let mut app = app(&dir);
    run(&mut app);
    for _ in 0..5 {
        app.update();
    }
    let world = app.world_mut();
    assert_eq!(world.query::<&WorldgenRoad>().iter(world).count(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

/// A mod's two-segment street network (guide frame), near the guide's road.
const STREETS: &str = r#"{
  "format": 1,
  "segments": [
    { "id": 0, "class": "local", "surface": "flagstone", "width_m": 4.0, "a": 0, "b": 1,
      "points": [[1100, 2100], [1140, 2110]], "crossings": [] },
    { "id": 3, "class": "local", "surface": "dirt", "width_m": 3.0, "a": 1, "b": 2,
      "points": [[1140, 2110], [1170, 2150]], "crossings": [] }
  ],
  "junctions": [{ "id": 0, "pos": [1100, 2100] }, { "id": 1, "pos": [1140, 2110] }, { "id": 2, "pos": [1170, 2150] }]
}"#;

#[test]
fn guide_and_mod_networks_spawn_with_their_sources() {
    let dir = temp_dir("mods");
    std::fs::write(dir.join("roads.json"), ROADS).unwrap();
    let files = [
        ("worldgen/roads/town-a.json", STREETS),
        ("worldgen/roads/broken.json", "{ \"format\": 1, \"segments\": ["),
        ("worldgen/roads/notes.txt", "not a network"),
        ("worldgen/other/elsewhere.json", STREETS),
    ];
    let mut app = app_with(&dir, Some(&files));
    run(&mut app);

    // The networks in draw order: the guide's, then the one valid mod file (the broken one is skipped).
    let vfs = app.world().resource::<mw_assets::SharedVfs>().0.clone();
    let roads = WorldgenRoads {
        guide_dir: dir.clone(),
        mod_roads_dir: Some(MOD_DIR.into()),
        height: Arc::new(ground),
        textures: RoadTextures::default(),
    };
    let nets = networks(&roads, Some(&vfs));
    let names: Vec<&str> = nets.iter().map(|(n, _)| &**n).collect();
    assert_eq!(names, [GUIDE_SOURCE, "worldgen/roads/town-a.json"]);

    let params = RibbonParams::default();
    let chunks_of = |net: &RoadNetwork| net.segments.iter().map(|s| ribbon(s, &params, &|x, y| ground(x, -y)).chunks.len()).sum::<usize>();
    let (guide_chunks, mod_chunks) = (chunks_of(&nets[0].1), chunks_of(&nets[1].1));
    assert!(mod_chunks >= 2, "both street segments draw ({mod_chunks} chunks)");

    let world = app.world_mut();
    let roads: Vec<WorldgenRoad> = world.query::<&WorldgenRoad>().iter(world).cloned().collect();
    let count = |source: &str| roads.iter().filter(|r| &*r.source == source).count();
    assert_eq!(count(GUIDE_SOURCE), guide_chunks, "the guide's chunks");
    assert_eq!(count("worldgen/roads/town-a.json"), mod_chunks, "the mod's chunks");
    assert_eq!(roads.len(), guide_chunks + mod_chunks, "nothing from the broken, non-json or other-folder files");
    let mut mod_segments: Vec<u32> = roads.iter().filter(|r| &*r.source != GUIDE_SOURCE).map(|r| r.segment).collect();
    mod_segments.sort_unstable();
    mod_segments.dedup();
    assert_eq!(mod_segments, [0, 3], "the mod's segment ids");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn mod_network_alone_draws_without_a_guide_roads_json() {
    let dir = temp_dir("modonly");
    let mut app = app_with(&dir, Some(&[("worldgen/roads/town-a.json", STREETS)]));
    run(&mut app);
    let world = app.world_mut();
    let roads: Vec<WorldgenRoad> = world.query::<&WorldgenRoad>().iter(world).cloned().collect();
    assert!(!roads.is_empty());
    assert!(roads.iter().all(|r| &*r.source == "worldgen/roads/town-a.json"));
    let _ = std::fs::remove_dir_all(&dir);
}
