//! GW-3: a guide's `water.json` spawns one mesh body per lake and river under the exterior's root, at the guide's
//! levels, in the engine frame (guide (x, y) → engine (x, −y)), the river with its flow; no file, no bodies.
#![allow(clippy::float_cmp)] // exact: copied levels

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bevy_app::App;
use bevy_asset::Assets;
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_mesh::{Mesh, VertexAttributeValues};
use bevy_transform::components::Transform;
use glam::DVec2;
use sf_water_bevy::{WaterLook, WaterMeshSurface};
use sf_world_bevy::WorldspaceRegistry;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_worldgen::water::GuideWater;

use super::{GuideWaterBody, RiverFlow, add, river_speeds};

/// One lake (a 100 m square at 12.5 m with a 20 m square hole) and one 3-point river (20 → 18 → 15 m), guide frame.
const WATER_JSON: &str = r#"{
  "format": 1, "frame": "guide: x east, y north, metres", "sea_level_m": 0.0,
  "lakes": [{ "id": 1, "kind": "open", "level_m": 12.5,
              "outline": [[0, 0], [100, 0], [100, 100], [0, 100]],
              "holes": [[[40, 40], [60, 40], [60, 60], [40, 60]]], "area_m2": 9600 }],
  "rivers": [{ "id": 0, "kind": "river", "mu_id": 0, "joins": -1, "outlet": 7, "clipped": false,
               "points": [[200, 0, 10, 20], [200, 100, 12, 18], [250, 200, 14, 15]] }],
  "stats": { "lakes": 1, "rivers": 1 }
}"#;

/// A fresh folder under the temp dir, with `water.json` when `json` is given.
fn guide_dir(name: &str, json: Option<&str>) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("gw3-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp guide dir");
    if let Some(json) = json {
        std::fs::write(dir.join("water.json"), json).expect("write water.json");
    }
    dir
}

/// An app with the exterior's root and the guide's bodies added, updated until `want` bodies spawned (or 30 s).
fn run(dir: &Path, want: usize) -> App {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>().init_resource::<WorldspaceRegistry>();
    WorldspaceRegistry::ensure_in(app.world_mut(), &mw_world::ids::exterior());
    let guide = GuideWater::read(dir).expect("read water.json");
    add(&mut app, guide, WaterLook::default());
    let start = Instant::now();
    loop {
        app.update();
        let n = app.world_mut().query::<&GuideWaterBody>().iter(app.world()).count();
        if n >= want && (want > 0 || start.elapsed() > Duration::from_millis(200)) || start.elapsed() > Duration::from_secs(30) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    app
}

/// World XZ (engine) of every vertex of a body, with its Y.
fn world_vertices(app: &mut App, kind: GuideWaterBody) -> (Vec<(DVec2, f32)>, Mesh) {
    let world = app.world_mut();
    let (surface, coord, transform) = world
        .query::<(&GuideWaterBody, &WaterMeshSurface, &CellCoord, &Transform)>()
        .iter(world)
        .find(|(k, ..)| **k == kind)
        .map(|(_, s, c, t)| (s.clone(), *c, *t))
        .expect("a body of this kind");
    let origin = sf_world_bevy::from_grid(&coord, &transform).to_f64();
    let mesh = world.resource::<Assets<Mesh>>().get(&surface.mesh).expect("mesh asset").clone();
    let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!("positions") };
    let out = positions.iter().map(|p| (DVec2::new(origin.x + f64::from(p[0]), origin.z + f64::from(p[2])), p[1])).collect();
    (out, mesh)
}

fn near(v: &[(DVec2, f32)], at: DVec2) -> Option<f32> {
    v.iter().find(|(p, _)| p.distance(at) < 1e-3).map(|(_, y)| *y)
}

#[test]
fn guide_water_spawns_lakes_and_rivers() {
    let dir = guide_dir("bodies", Some(WATER_JSON));
    let mut app = run(&dir, 2);
    let world = app.world_mut();
    let entry = world.resource::<WorldspaceRegistry>().get(&mw_world::ids::exterior()).expect("exterior");
    let bodies: Vec<(GuideWaterBody, Entity, RenderLayers)> = world
        .query_filtered::<(&GuideWaterBody, &ChildOf, &RenderLayers), With<WaterMeshSurface>>()
        .iter(world)
        .map(|(k, p, l)| (*k, p.parent(), l.clone()))
        .collect();
    assert_eq!(bodies.iter().filter(|b| b.0 == GuideWaterBody::Lake).count(), 1, "one lake");
    assert_eq!(bodies.iter().filter(|b| b.0 == GuideWaterBody::River).count(), 1, "one river");
    for (_, parent, layers) in &bodies {
        assert_eq!(*parent, entry.root, "under the exterior's root (floating origin)");
        assert_eq!(*layers, RenderLayers::layer(entry.layer), "on the exterior's render layer");
    }

    // The lake: flat at its level, guide (x, y) at engine (x, −y), the hole cut out (area 100² − 20²).
    let (lake, mesh) = world_vertices(&mut app, GuideWaterBody::Lake);
    assert!(lake.iter().all(|(_, y)| *y == 12.5), "every lake vertex at the lake level");
    for (gx, gy) in [(0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0), (40.0, 40.0), (60.0, 60.0)] {
        assert!(near(&lake, DVec2::new(gx, -gy)).is_some(), "guide ({gx}, {gy}) lands at engine ({gx}, {})", -gy);
    }
    assert!(lake.iter().all(|(p, _)| p.y <= 1e-3), "guide y ≥ 0 is engine z ≤ 0");
    let Some(bevy_mesh::Indices::U32(idx)) = mesh.indices() else { panic!("u32 indices") };
    let area: f64 = idx
        .chunks(3)
        .map(|t| {
            let [a, b, c] = [lake[t[0] as usize].0, lake[t[1] as usize].0, lake[t[2] as usize].0];
            0.5 * (b - a).perp_dot(c - a).abs()
        })
        .sum();
    assert!((area - 9600.0).abs() < 1.0, "lake area {area} m² (the hole cut out)");

    // The river: its control points at their levels, never above the source or below the mouth, flow attached and
    // pointing downstream (guide north = engine −Z at the source).
    let (river, mesh) = world_vertices(&mut app, GuideWaterBody::River);
    assert_eq!(near(&river, DVec2::new(200.0, 0.0)), Some(20.0), "source centre at 20 m");
    assert_eq!(near(&river, DVec2::new(200.0, -100.0)), Some(18.0), "middle centre at 18 m");
    assert_eq!(near(&river, DVec2::new(250.0, -200.0)), Some(15.0), "mouth centre at 15 m");
    assert!(river.iter().all(|(_, y)| (15.0..=20.0).contains(y)), "river levels within the guide's");
    let Some(VertexAttributeValues::Float32x2(flow)) = mesh.attribute(sf_water_bevy::mesh::ATTRIBUTE_WATER_FLOW) else {
        panic!("river flow attribute")
    };
    assert_eq!(flow.len(), river.len(), "one flow per vertex");
    // The spline's end tangent leans a little toward the bend downstream.
    assert!(flow[0][1] < -0.99 && flow[0][0].abs() < 0.05, "flow at the source runs engine −Z: {:?}", flow[0]);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn no_water_json_no_bodies() {
    let dir = guide_dir("none", None);
    assert!(GuideWater::read(&dir).expect("read").is_none(), "no file reads as none");
    let mut app = run(&dir, 0);
    let world = app.world_mut();
    assert_eq!(world.query::<&GuideWaterBody>().iter(world).count(), 0, "no bodies without water.json");
    assert_eq!(world.query::<&WaterMeshSurface>().iter(world).count(), 0, "no mesh surfaces");
    let _ = std::fs::remove_dir_all(dir);
}

/// The bake on a real guide (`GW3_WATER_DIR`, default the x10s-b2w build output): the log line's numbers.
#[test]
#[ignore = "measurement: bake time and size of a real guide's water.json"]
fn measure_real_guide_bake() {
    let dir = std::env::var("GW3_WATER_DIR").unwrap_or_else(|_| r"C:\cargo-target\scrib\worldgen\x10s-b2w".into());
    let Some(guide) = GuideWater::read(Path::new(&dir)).expect("read water.json") else {
        println!("no water.json in {dir}");
        return;
    };
    let b = super::bake(&guide, &RiverFlow::default());
    let [p10, p50, p90] = b.speeds;
    println!(
        "{dir}: {} lakes, {} rivers, {:.1} km, river speed p10/p50/p90 {p10:.2}/{p50:.2}/{p90:.2} m/s, {} vertices, built in {:.0} ms",
        b.lakes, b.rivers, b.km, b.vertices, b.ms
    );
}

/// A straight 1 km river due north, 10 m wide, a point every 20 m, its level dropping `drop_per_m` per metre.
fn straight_river(drop_per_m: f64) -> Vec<[f64; 4]> {
    (0..=50).map(|i| f64::from(i) * 20.0).map(|y| [0.0, y, 10.0, 100.0 - drop_per_m * y]).collect()
}

/// GW-6: the speed follows the slope by Manning: steeper is faster, flat is the floor, all within the clamp.
#[test]
fn river_speed_follows_slope() {
    let flow = RiverFlow::default();
    let median = |v: &[f64]| {
        let mut v = v.to_vec();
        v.sort_by(f64::total_cmp);
        v[v.len() / 2]
    };
    let gentle = river_speeds(&straight_river(0.0005), &flow);
    let steep = river_speeds(&straight_river(0.005), &flow);
    let flat = river_speeds(&straight_river(0.0), &flow);
    assert!(median(&steep) > median(&gentle), "a faster drop flows faster: {} vs {}", median(&steep), median(&gentle));
    assert!(flat.iter().all(|&v| v == flow.min_mps), "a flat river flows at the minimum: {flat:?}");
    // Manning by hand at a mid point of the gentle river: d = 0.95 m, R = 9.5 / 11.9, S = 0.0005.
    let r: f64 = 10.0 * 0.95 / (10.0 + 1.9);
    let want = r.powf(2.0 / 3.0) * 0.0005_f64.sqrt() / flow.manning_n;
    assert!((gentle[25] - want).abs() < 1e-9, "Manning: {} vs {want}", gentle[25]);
    // Plain Manning (the steep roughness off) holds a cliff of a river at the maximum.
    let plain = RiverFlow { steep_n: false, ..flow };
    let rapids = river_speeds(&straight_river(0.2), &plain);
    assert!(rapids.iter().all(|&v| v == plain.max_mps), "a cliff of a river is held at the maximum");
}

/// G-W7: a flat reach drifts at the minimum, and a steep 10 % reach takes Jarrett's roughness: exactly his
/// `n = 0.39 S^0.38 R^-0.16`, below the cap and well below plain Manning on the same reach.
#[test]
fn steep_reach_takes_jarrett_roughness() {
    let flow = RiverFlow::default();
    assert!(flow.steep_n, "the steep roughness is the default");
    let flat = river_speeds(&straight_river(0.0), &flow);
    assert!(flat.iter().all(|&v| v == flow.min_mps), "a flat reach drifts at the minimum: {flat:?}");

    let steep = river_speeds(&straight_river(0.1), &flow);
    // By hand at a mid point: d = 0.95 m, R = 9.5 / 11.9, S = 0.1.
    let r: f64 = 10.0 * 0.95 / (10.0 + 1.9);
    let n = 0.39 * 0.1_f64.powf(0.38) * r.powf(-0.16);
    let want = r.powf(2.0 / 3.0) * 0.1_f64.sqrt() / n;
    assert!((steep[25] - want).abs() < 1e-9, "Jarrett: {} vs {want}", steep[25]);
    assert!(steep.iter().all(|&v| v < flow.max_mps), "a 10 % reach stays below the cap: {steep:?}");
    let manning = RiverFlow { steep_n: false, max_mps: 1e3, ..flow };
    let plain = river_speeds(&straight_river(0.1), &manning);
    assert!(steep.iter().zip(&plain).all(|(s, p)| s < p), "below plain Manning: {} vs {}", steep[25], plain[25]);
    assert!(steep[25] < plain[25] * 0.3, "Jarrett's steep bed slows it by more than 3x: {} vs {}", steep[25], plain[25]);
}

/// G-W7: the speeds of a real guide's rivers (`GW3_WATER_DIR`, default the x10s-b2w build output), with the share
/// of river points above the foam start, for Jarrett (the default) and plain Manning (the A/B arm).
#[test]
#[ignore = "measurement: river speed spread and foam share of a real guide's water.json"]
#[allow(clippy::cast_precision_loss)] // point counts
fn measure_real_river_speeds() {
    let dir = std::env::var("GW3_WATER_DIR").unwrap_or_else(|_| r"C:\cargo-target\scrib\worldgen\x10s-b2w".into());
    let Ok(Some(guide)) = GuideWater::read(Path::new(&dir)) else {
        println!("no water.json in {dir}");
        return;
    };
    let foam = sf_water_bevy::WaterFlowSettings::default();
    for (arm, flow) in [("jarrett", RiverFlow::default()), ("manning", RiverFlow { steep_n: false, ..RiverFlow::default() })] {
        let mut v: Vec<f64> = guide.rivers.iter().flat_map(|r| river_speeds(&r.points, &flow)).collect();
        let above = |t: f32| v.iter().filter(|&&s| s > f64::from(t)).count() as f64 / v.len().max(1) as f64 * 100.0;
        let (start, full) = (above(foam.foam_start_mps), above(foam.foam_full_mps - 1e-3));
        let [p10, p50, p90] = super::percentiles(&mut v);
        println!(
            "{arm}: {} points, river speed p10/p50/p90 {p10:.2}/{p50:.2}/{p90:.2} m/s (cap {:.1}), {start:.1} % above foam start {:.1} m/s, {full:.1} % at foam full {:.1} m/s",
            v.len(),
            flow.max_mps,
            foam.foam_start_mps,
            foam.foam_full_mps
        );
    }
}

/// GW-6: speeds stay within the clamp and change smoothly along a river whose slope breaks sharply.
#[test]
fn river_speed_clamped_and_continuous() {
    let flow = RiverFlow::default();
    // Flat for 500 m, then dropping 1 % (a sharp break), a point every 5 m, widening downstream.
    let points: Vec<[f64; 4]> = (0..=200)
        .map(|i| {
            let y = f64::from(i) * 5.0;
            [0.0, y, 8.0 + y * 0.01, 50.0 - 0.01 * (y - 500.0).max(0.0)]
        })
        .collect();
    let v = river_speeds(&points, &flow);
    assert_eq!(v.len(), points.len(), "one speed per point");
    assert!(v.iter().all(|s| (flow.min_mps..=flow.max_mps).contains(s)), "within the clamp: {v:?}");
    assert_eq!(v[0], flow.min_mps, "the flat head drifts at the minimum");
    assert!(v[200] > 1.0, "the steep tail runs: {}", v[200]);
    // Over the break the window spreads the change: no step between 5 m neighbours bigger than a tenth of the
    // whole change (a raw per-segment slope would jump from the minimum to the full speed in one step).
    let range = v[200] - v[0];
    let worst = v.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0, f64::max);
    assert!(worst < range * 0.2, "neighbouring speeds jump {worst} of {range}");
}

/// The flow shading's `water.flow.*` reach `WaterFlowSettings`, live.
#[test]
fn flow_tunables_bind_live() {
    use sf_tunables_bevy::{SetBy, SetTunable, TunablesPlugin};
    use sf_water_bevy::WaterFlowSettings;
    let mut app = App::new();
    app.add_plugins(TunablesPlugin).init_resource::<WaterFlowSettings>();
    crate::water::bind_flow(&mut app);
    app.update();
    assert_eq!(*app.world().resource::<WaterFlowSettings>(), WaterFlowSettings::default(), "defaults bound");
    for (name, value) in [
        ("water.flow.cycle_s", "3"),
        ("water.flow.advect_scale", "0.5"),
        ("water.flow.phase_noise", "0.25"),
        ("water.flow.noise_cell_m", "8"),
        ("water.flow.normal_gain", "1.5"),
        ("water.flow.foam_start_mps", "2"),
        ("water.flow.foam_full_mps", "6"),
        ("water.flow.foam", "0.3"),
    ] {
        app.world_mut().write_message(SetTunable::new(name, value, SetBy::Console));
    }
    app.update();
    let f = *app.world().resource::<WaterFlowSettings>();
    let want = WaterFlowSettings {
        cycle_s: 3.0,
        advect_scale: 0.5,
        phase_noise: 0.25,
        noise_cell_m: 8.0,
        normal_gain: 1.5,
        foam_start_mps: 2.0,
        foam_full_mps: 6.0,
        foam: 0.3,
    };
    assert_eq!(f, want, "every water.flow.* tunable reaches WaterFlowSettings");
}
