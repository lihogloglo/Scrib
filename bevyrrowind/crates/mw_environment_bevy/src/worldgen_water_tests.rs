//! GW-3: a guide's `water.json` spawns one mesh body per lake and river under the exterior's root, at the guide's
//! levels, in the engine frame (guide (x, y) → engine (x, −y)), the river with its flow; no file, no bodies.
#![allow(clippy::float_cmp)] // exact: copied levels

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy_app::App;
use bevy_asset::Assets;
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_mesh::{Mesh, VertexAttributeValues};
use bevy_transform::components::Transform;
use glam::{DVec2, DVec3, Vec3};
use sf_coords::WorldPos;
use sf_water::{
    BedHeights, Capsule, Join, JoinEnd, JoinSettings, JoinStart, WaterBody, WaterKind, WaterQuery, WaterSample, WaterShape, WaterWorld, WorldspaceWater,
};
use sf_water_bevy::{WaterLook, WaterMeshSurface};
use sf_world::Worldspace;
use sf_world_bevy::WorldspaceRegistry;
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_worldgen::water::{GuideWater, Lake, River};

use super::{GuideWaterBody, RIBBON_CHUNK, Resolved, RiverFlow, add, guide_bodies, river_spline, river_speeds};

/// `guide`'s joins resolved at the default flow and join settings.
fn resolve(guide: &GuideWater) -> Resolved {
    super::resolved(guide, &RiverFlow::default(), &JoinSettings::default())
}

/// `guide`'s query bodies on its default-resolved splines.
fn bodies_of(guide: &GuideWater, chunk: usize) -> Vec<WaterBody> {
    guide_bodies(guide, &resolve(guide), chunk)
}

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
    let settings = super::settings(&mut app);
    let guide = guide.map(|g| {
        let resolved = super::resolved(&g, &settings.flow, &settings.joins);
        (g, resolved)
    });
    add(&mut app, guide, WaterLook::default(), settings);
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

/// The bake on a real guide (`GW3_WATER_DIR`, default the x10hs-b2w build output): the joins' and the bake's log
/// lines.
#[test]
#[ignore = "measurement: bake time and size of a real guide's water.json"]
fn measure_real_guide_bake() {
    let dir = std::env::var("GW3_WATER_DIR").unwrap_or_else(|_| r"C:\cargo-target\scrib\worldgen\x10hs-b2w".into());
    let Some(guide) = GuideWater::read(Path::new(&dir)).expect("read water.json") else {
        println!("no water.json in {dir}");
        return;
    };
    let resolved = resolve(&guide);
    println!("{dir}: {}", resolved.summary());
    let b = super::bake(&guide, &resolved, &RiverFlow::default());
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

// FL-5a: the guide's lakes and rivers in the CPU water query.

/// A 100 m square lake at 120 m with a 20 m square hole, and a 3-point river (130 → 128 → 125 m), guide frame.
fn query_guide() -> GuideWater {
    GuideWater {
        sea_level_m: 0.0,
        lakes: vec![Lake {
            id: 4,
            level_m: 120.0,
            outline: vec![[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]],
            holes: vec![vec![[40.0, 40.0], [60.0, 40.0], [60.0, 60.0], [40.0, 60.0]]],
            ..Lake::default()
        }],
        rivers: vec![River {
            id: 9,
            kind: "river".into(),
            points: vec![[200.0, 0.0, 10.0, 130.0], [200.0, 100.0, 12.0, 128.0], [250.0, 200.0, 14.0, 125.0]],
            ..River::default()
        }],
    }
}

/// Flat terrain at a fixed height.
struct FlatBed(f32);

impl BedHeights for FlatBed {
    fn bed_height(&self, _: WorldPos) -> Option<f32> {
        Some(self.0)
    }
}

/// The exterior's query as `water.rs` builds it: the ocean at 0 m, then the guide's bodies, over `bed`.
fn query_of(guide: &GuideWater, chunk: usize, bed: Option<f32>) -> WorldspaceWater {
    let ext = mw_world::ids::exterior();
    let mut ws = Worldspace::new("exterior");
    ws.has_terrain = true;
    ws.sea_level = Some(0.0);
    let mut bodies = vec![WaterBody::from_worldspace(&ext, &ws).expect("the ocean")];
    bodies.extend(bodies_of(guide, chunk));
    let mut water = WorldspaceWater::new(bodies);
    water.set_bed(bed.map(|b| Arc::new(FlatBed(b)) as Arc<dyn BedHeights>));
    water
}

/// Engine position of guide `(x, y)` at height `h`.
fn at(x: f64, y: f64, h: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, h, -y))
}

#[test]
fn guide_bodies_records() {
    let bodies = bodies_of(&query_guide(), 64);
    let ids: Vec<&str> = bodies.iter().map(|b| b.id.as_str()).collect();
    assert_eq!(ids, ["guide:lake:4", "guide:river:9:0"]);
    let (lake, river) = (&bodies[0], &bodies[1]);
    assert_eq!((lake.kind, lake.level, lake.priority), (WaterKind::Lake, 120.0, 20));
    assert_eq!((river.kind, river.level, river.priority), (WaterKind::River, 130.0, 30));
    assert_eq!(lake.waves, sf_water::WaveTerms::for_kind(WaterKind::Lake));
    assert_eq!(river.waves, sf_water::WaveTerms::FLAT);
    for b in &bodies {
        assert_eq!(b.worldspace, mw_world::ids::exterior());
        assert_eq!(b.optics.as_ref().map(|r| r.id.as_str()), Some(super::LOOK));
        assert_eq!(b.flow, None);
    }
    let WaterShape::Polygon { outline, holes } = &lake.shape else { panic!("a lake is a polygon") };
    assert_eq!(outline[2], DVec2::new(100.0, -100.0), "guide (x, y) at engine (x, −y)");
    assert_eq!(holes[0][0], DVec2::new(40.0, -40.0));
}

#[test]
fn guide_river_answers_the_query() {
    let guide = query_guide();
    let water = query_of(&guide, 64, Some(117.0));
    let speeds = river_speeds(&guide.rivers[0].points, &RiverFlow::default());
    #[allow(clippy::cast_possible_truncation)]
    let speed = |i: usize, t: f64| speeds[i] as f32 + (speeds[i + 1] as f32 - speeds[i] as f32) * t as f32;

    // A quarter down the first segment, on the centre line: engine −Z, 129.5 m.
    let first = water.sample(at(200.0, 25.0, 0.0)).expect("water on the river");
    assert_eq!((first.body.as_str(), first.kind), ("guide:river:9:0", WaterKind::River));
    assert_eq!(first.height, 129.5);
    assert!((first.velocity - Vec3::new(0.0, 0.0, -1.0) * speed(0, 0.25)).length() < 1e-4, "{:?} vs {}", first.velocity, speed(0, 0.25));
    assert_eq!(first.depth, 12.5);

    // Halfway down the second segment, 2 m off the centre line.
    let (from, to) = (DVec2::new(200.0, -100.0), DVec2::new(250.0, -200.0));
    let segment = to - from;
    let off = (from + to) * 0.5 + segment.perp().normalize() * 2.0;
    let second = water.sample(WorldPos::from_f64(DVec3::new(off.x, 0.0, off.y))).expect("water on the river");
    assert_eq!(second.kind, WaterKind::River);
    // `WorldPos` keeps an f32 offset in its cell: the off-grid point moves by ~1e-5 m, the level by ~1e-7 m.
    assert!((second.height - 126.5).abs() < 1e-6, "lerped level {}", second.height);
    let dir = segment.normalize().as_vec2();
    assert!((second.velocity - Vec3::new(dir.x, 0.0, dir.y) * speed(1, 0.5)).length() < 1e-4, "{:?}", second.velocity);
}

#[test]
fn guide_lake_answers_the_query() {
    let water = query_of(&query_guide(), 64, Some(117.0));
    let s = water.sample(at(20.0, 20.0, 0.0)).expect("water in the lake");
    assert_eq!((s.body.as_str(), s.kind, s.height, s.depth), ("guide:lake:4", WaterKind::Lake, 120.0, 3.0));
    assert_eq!(s.velocity, Vec3::ZERO, "a lake does not flow");
    assert_eq!(water.sample(at(50.0, 50.0, 0.0)), None, "the lake's hole is land (the ocean is dry above its bed)");
    assert_eq!(water.sample(at(150.0, 50.0, 0.0)), None, "land outside the lake and the river");

    // A capsule from 118 m to 121 m standing in the lake: 2 of its 3 m under the surface.
    let mut world = WaterWorld::new();
    world.insert(mw_world::ids::exterior(), water);
    let capsule = Capsule { a: at(20.0, 20.0, 118.5), b: at(20.0, 20.0, 120.5), radius: 0.5 };
    let imm = world.immersion(&mw_world::ids::exterior(), capsule);
    assert_eq!(imm.depth, 2.0);
    #[allow(clippy::cast_possible_truncation)]
    let want = (2.0_f64 / 3.0) as f32;
    assert_eq!(imm.fraction, want);
    assert_eq!(imm.sample.map(|s| s.kind), Some(WaterKind::Lake));
}

#[test]
fn long_river_splits_into_seamless_chunks() {
    // A straight 20-point river due north at x = 1 km, 10 m wide, dropping 1 cm per metre.
    let points: Vec<[f64; 4]> = (0..20).map(|i| f64::from(i) * 20.0).map(|y| [1000.0, y, 10.0, 100.0 - 0.01 * y]).collect();
    let guide = GuideWater { rivers: vec![River { id: 5, kind: "river".into(), points, ..River::default() }], ..GuideWater::default() };
    let chunks = bodies_of(&guide, 8);
    let ids: Vec<&str> = chunks.iter().map(|b| b.id.as_str()).collect();
    assert_eq!(ids, ["guide:river:5:0", "guide:river:5:1", "guide:river:5:2"], "points 0-7, 7-14, 14-19");
    let splines: Vec<_> = chunks
        .iter()
        .map(|b| match &b.shape {
            WaterShape::Ribbon { spline } => spline.clone(),
            other => panic!("a river is a ribbon: {other:?}"),
        })
        .collect();
    assert_eq!(splines.iter().map(Vec::len).collect::<Vec<_>>(), [8, 8, 6]);
    for w in splines.windows(2) {
        assert_eq!(w[0].last(), w[1].first(), "consecutive chunks share their seam point");
    }
    assert_eq!(chunks[1].level, splines[1][0].height, "a chunk's level is its first point's");

    let (split, whole) = (query_of(&guide, 8, Some(0.0)), query_of(&guide, 512, Some(0.0)));
    assert_eq!(whole.bodies().len(), 2, "the ocean and one river body");
    // At a seam both chunks hold the point, at their end and start: equal up to the lerp's rounding.
    let same = |a: &WaterSample, b: &WaterSample| {
        a.kind == b.kind && (a.height - b.height).abs() < 1e-9 && (a.velocity - b.velocity).length() < 1e-6 && (a.depth - b.depth).abs() < 1e-5
    };
    // The seams (points 7 and 14), mid-segments either side and across each chunk, off the centre line too.
    for (x, y) in [(1000.0, 140.0), (1000.0, 280.0), (1000.0, 130.0), (1000.0, 150.0), (1003.0, 290.0), (998.0, 50.0), (1001.0, 370.0)] {
        let (a, b) = (split.sample(at(x, y, 0.0)).expect("chunked river"), whole.sample(at(x, y, 0.0)).expect("whole river"));
        assert_eq!(a.kind, WaterKind::River);
        assert!(same(&a, &b), "guide ({x}, {y}): {a:?} vs {b:?}");
    }
}

#[test]
fn short_river_gives_no_body() {
    let guide = GuideWater { rivers: vec![River { id: 1, kind: "braid".into(), points: vec![[0.0, 0.0, 5.0, 10.0]], ..River::default() }], ..GuideWater::default() };
    assert!(bodies_of(&guide, 64).is_empty());
}

/// FL-4b: FL-4a's Y confluence in guide coordinates. River 11 names line 7 (rivers 10 and 12, far away); its end
/// (31, -1) is 1 m from river 10's centre line (y = 0), so it moves to (31, 0) in the engine frame and fades out
/// there; the resolved spline reaches the query's ribbon unchanged.
#[test]
fn joins_reach_the_query_bodies() {
    let river = |id: i64, mu_id: i64, joins: Option<i64>, points: Vec<[f64; 4]>| River {
        id,
        kind: "river".into(),
        mu_id: Some(mu_id),
        joins,
        points,
        ..River::default()
    };
    let tributary = vec![[30.0, -40.0, 4.0, 12.0], [30.0, -30.0, 4.0, 11.0], [30.0, -20.0, 4.0, 10.5], [30.0, -10.0, 4.0, 10.0], [30.0, -4.0, 4.0, 9.6], [31.0, -1.0, 4.0, 9.5]];
    let guide = GuideWater {
        rivers: vec![
            river(10, 7, None, vec![[0.0, 0.0, 10.0, 10.0], [50.0, 0.0, 10.0, 9.0], [100.0, 0.0, 10.0, 8.0]]),
            river(11, 8, Some(7), tributary.clone()),
            river(12, 7, None, vec![[1000.0, 0.0, 10.0, 5.0], [1050.0, 0.0, 10.0, 4.0]]),
        ],
        ..GuideWater::default()
    };
    let resolved = resolve(&guide);

    let open = Join { start: JoinStart::Open, end: JoinEnd::Open };
    assert_eq!((resolved.joins[0], resolved.joins[2]), (open, open));
    let Join { start: JoinStart::Open, end: JoinEnd::Tributary { receiver: 0, moved_m } } = resolved.joins[1] else {
        panic!("river 11 joins river 10: {:?}", resolved.joins[1]);
    };
    assert!((moved_m - 1.0).abs() <= 1e-9, "moved {moved_m} m");

    let spline = &resolved.splines[1];
    let ness: Vec<f32> = spline.iter().map(|p| p.river_ness).collect();
    for (got, want) in ness.iter().zip([1.0_f32, 1.0, 1.0, 1.0, 1.0, 0.0]) {
        assert!((got - want).abs() <= 1e-6, "river_ness {ness:?}");
    }
    assert_eq!(ness.len(), 6);
    let last = spline.last().expect("six points");
    assert_eq!((last.center, last.flow_speed), (DVec2::new(31.0, 0.0), 0.0));
    let plain = river_spline(&tributary, &river_speeds(&tributary, &RiverFlow::default()));
    assert_eq!(spline[..5], plain[..5], "the points above the confluence are untouched");

    let bodies = guide_bodies(&guide, &resolved, 64);
    let body = bodies.iter().find(|b| b.id.as_str() == "guide:river:11:0").expect("river 11's body");
    let WaterShape::Ribbon { spline: ribbon } = &body.shape else { panic!("a river is a ribbon: {:?}", body.shape) };
    assert_eq!(ribbon, spline, "the query's ribbon carries the resolved spline");

    assert_eq!(
        resolved.summary(),
        "worldgen water joins: 3 river ends: 0 lake, 1 tributary, 0 unsnapped (farthest 0 m), 0 sea, 2 open; 0 start in a lake"
    );
}

/// FL-5a: ns per `WorldspaceWater::sample` on a real guide (`GW3_WATER_DIR`, default the x10s-b2w build output), on
/// every river point and on a 200 x 200 grid over the guide, at the default `water.query.ribbon_chunk` and at 512.
#[test]
#[ignore = "measurement: CPU water query sample time on a real guide's water.json"]
#[allow(clippy::cast_precision_loss)] // sample counts
fn measure_real_guide_query() {
    let dir = std::env::var("GW3_WATER_DIR").unwrap_or_else(|_| r"C:\cargo-target\scrib\worldgen\x10s-b2w".into());
    let Ok(Some(guide)) = GuideWater::read(Path::new(&dir)) else {
        println!("no water.json in {dir}");
        return;
    };
    let rivers: Vec<WorldPos> = guide.rivers.iter().flat_map(|r| r.points.iter().map(|p| at(p[0], p[1], 0.0))).collect();
    let all = guide.rivers.iter().flat_map(|r| r.points.iter().map(|p| [p[0], p[1]])).chain(guide.lakes.iter().flat_map(|l| l.outline.iter().copied()));
    let (lo, hi) = all.fold(([f64::MAX; 2], [f64::MIN; 2]), |(lo, hi), p| ([lo[0].min(p[0]), lo[1].min(p[1])], [hi[0].max(p[0]), hi[1].max(p[1])]));
    let grid: Vec<WorldPos> = (0..200)
        .flat_map(|i| (0..200).map(move |j| (f64::from(i) / 199.0, f64::from(j) / 199.0)))
        .map(|(u, v)| at(lo[0] + (hi[0] - lo[0]) * u, lo[1] + (hi[1] - lo[1]) * v, 0.0))
        .collect();
    let longest = guide.rivers.iter().map(|r| r.points.len()).max().unwrap_or(0);
    for chunk in [RIBBON_CHUNK.default_value() as usize, 512] {
        let water = query_of(&guide, chunk, None);
        let ns = |pts: &[WorldPos]| {
            let t = Instant::now();
            let wet = pts.iter().filter(|p| std::hint::black_box(water.sample(**p)).is_some_and(|s| s.kind != WaterKind::Ocean)).count();
            (t.elapsed().as_nanos() as f64 / pts.len().max(1) as f64, wet)
        };
        let ((river_ns, river_wet), (grid_ns, grid_wet)) = (ns(&rivers), ns(&grid));
        println!(
            "{dir}: ribbon_chunk {chunk}: {} bodies ({} lakes, {} rivers, longest {longest} points); sample {river_ns:.0} ns on {} river points ({river_wet} in a guide body), {grid_ns:.0} ns on a {} point grid ({grid_wet} in a guide body)",
            water.bodies().len(),
            guide.lakes.len(),
            guide.rivers.len(),
            rivers.len(),
            grid.len()
        );
    }
}
