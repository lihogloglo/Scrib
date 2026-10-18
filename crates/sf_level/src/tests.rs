use glam::Vec3;

use crate::{Level, LevelError, Material, Shape};

const SAMPLE: &str = r#"
(
    name: "sample",
    description: "every shape once",
    cameras: [(name: "overview", eye: (0.0, 10.0, 20.0), target: (0.0, 0.0, 0.0))],
    spawns: [(name: "start", at: (0.0, 0.0, 5.0))],
    objects: [
        (shape: Ground(size: (40.0, 40.0))),
        (name: "box", shape: Box(size: (1.0, 2.0, 3.0)), at: (5.0, 0.0, 0.0), material: Orange),
        (name: "ramp-30", shape: Ramp(width: 3.0, rise: 2.0, angle: 30.0), yaw: 90.0),
        (name: "stairs", shape: Stairs(width: 2.0, steps: 10, rise: 0.18, run: 0.28), at: (-5.0, 0.0, 0.0)),
        (name: "pole", shape: Cylinder(radius: 0.1, height: 3.0), collide: false),
    ],
)
"#;

fn shapes() -> Vec<Shape> {
    Level::parse(SAMPLE).expect("sample parses").objects.into_iter().map(|o| o.shape).collect()
}

#[test]
fn sample_parses_with_defaults() {
    let level = Level::parse(SAMPLE).expect("sample parses");
    assert_eq!(level.objects.len(), 5);
    assert_eq!(level.objects[0].material, Material::Floor);
    assert!(level.objects[0].collide);
    assert!(!level.objects[4].collide);
    assert!((level.objects[2].rotation() * Vec3::NEG_Z - Vec3::NEG_X).length() < 1e-5, "yaw 90 turns -z to -x");
}

#[test]
fn unknown_fields_are_errors() {
    let bad = SAMPLE.replace("description:", "descripton:");
    assert!(matches!(Level::parse(&bad), Err(LevelError::Parse(_))));
}

#[test]
fn validation_lists_every_problem() {
    let text = r#"(
        name: "bad",
        cameras: [],
        objects: [
            (name: "a", shape: Box(size: (1.0, 0.0, 1.0))),
            (name: "a", shape: Ramp(width: 1.0, rise: 1.0, angle: 90.0)),
            (shape: Stairs(width: 1.0, steps: 0, rise: 0.2, run: 0.3)),
        ],
    )"#;
    let Err(LevelError::Invalid { problems, .. }) = Level::parse(text) else { panic!("invalid level accepted") };
    assert_eq!(problems.len(), 5, "{problems:#?}");
}

#[test]
fn normals_follow_the_winding() {
    for shape in shapes() {
        let mesh = shape.mesh();
        assert_eq!(mesh.positions.len(), mesh.normals.len());
        assert_eq!(mesh.positions.len(), mesh.uvs.len());
        for tri in mesh.triangles() {
            let [p0, p1, p2] = tri.map(|i| Vec3::from_array(mesh.positions[i as usize]));
            let normal = Vec3::from_array(mesh.normals[tri[0] as usize]);
            assert!((normal.length() - 1.0).abs() < 1e-4);
            assert!((p1 - p0).cross(p2 - p0).dot(normal) > 0.0, "{shape:?}: triangle faces inward");
        }
    }
}

#[test]
fn sizes_match_the_parameters() {
    let stairs = Shape::Stairs { width: 2.0, steps: 10, rise: 0.18, run: 0.28 };
    let (lo, hi) = stairs.mesh().bounds();
    assert!((hi.y - 1.8).abs() < 1e-5 && lo.y.abs() < 1e-6);
    assert!(((hi.z - lo.z) - 2.8).abs() < 1e-5);
    // One tread per step: two up-facing triangles each.
    let m = stairs.mesh();
    let up = m.triangles().iter().filter(|t| m.normals[t[0] as usize][1] > 0.99).count();
    assert_eq!(up, 20);

    let ground = Shape::Ground { size: (40.0, 30.0) };
    let (lo, hi) = ground.mesh().bounds();
    assert_eq!((hi.y, hi.x - lo.x, hi.z - lo.z), (0.0, 40.0, 30.0));

    for angle in [10.0_f32, 20.0, 30.0, 40.0] {
        let ramp = Shape::Ramp { width: 3.0, rise: 2.0, angle };
        let m = ramp.mesh();
        let slope = m.normals.iter().map(|n| Vec3::from_array(*n)).find(|n| n.y > 0.0 && n.y < 0.999).expect("sloped face");
        assert!((slope.y.acos().to_degrees() - angle).abs() < 1e-3, "ramp {angle}°");
        assert!(slope.z > 0.0, "rises toward -z, so the slope faces +z");
        let (lo, hi) = m.bounds();
        assert!(((hi.z - lo.z) - ramp.length()).abs() < 1e-4 && (hi.y - 2.0).abs() < 1e-5);
    }

    let (lo, hi) = Shape::Cylinder { radius: 0.5, height: 3.0 }.mesh().bounds();
    assert!((hi.x - 0.5).abs() < 1e-5 && (lo.x + 0.5).abs() < 1e-5 && (hi.y - 3.0).abs() < 1e-6);
}

#[test]
fn uvs_are_meters() {
    // A 4 m box face spans 4 texture tiles.
    let m = Shape::Box { size: Vec3::new(4.0, 1.0, 1.0) }.mesh();
    let top: Vec<[f32; 2]> = m.normals.iter().zip(&m.uvs).filter(|(n, _)| n[1] > 0.99).map(|(_, uv)| *uv).collect();
    let span = |k: usize| top.iter().map(|uv| uv[k]).fold(f32::MIN, f32::max) - top.iter().map(|uv| uv[k]).fold(f32::MAX, f32::min);
    assert!(((span(0).max(span(1))) - 4.0).abs() < 1e-5);
}

const ACTORS: &str = r#"
(
    name: "actors",
    cameras: [(name: "overview", eye: (0.0, 10.0, 20.0), target: (0.0, 0.0, 0.0))],
    spawns: [(name: "a", at: (0.0, 0.0, 5.0), yaw: 90.0), (name: "b", at: (2.0, 0.0, 5.0))],
    objects: [
        (shape: Ground(size: (40.0, 40.0))),
        (name: "crate", shape: Box(size: (0.6, 0.6, 0.6)), at: (3.0, 0.0, 0.0), dynamic: Some(20.0), interact: Some("open")),
    ],
    actors: [
        (id: "walker", spawn: "a", outfit: Some(["robe", "shoes"]),
         path: Some((points: [(0.0, 0.0, 5.0), (0.0, 0.0, -5.0)], mode: PingPong, speed: 1.2, wait: 2.0))),
        (id: "idler", spawn: "B", action: Some((trigger: "fidget", params: [("variant", 3.0)]))),
    ],
)
"#;

#[test]
fn actors_and_props_parse_with_defaults() {
    let level = Level::parse(ACTORS).expect("actors parse");
    assert_eq!(level.actors.len(), 2);
    let walker = &level.actors[0];
    let path = walker.path.as_ref().expect("path");
    assert_eq!((path.mode, path.run, path.points.len()), (crate::PathMode::PingPong, false, 2));
    assert_eq!(walker.outfit.as_deref().map(<[String]>::len), Some(2));
    let action = level.actors[1].action.as_ref().expect("action");
    assert!((action.every - 6.0).abs() < 1e-6, "default period");
    assert_eq!(level.spawn("b").map(|s| s.at.x), Some(2.0), "spawn names are case-insensitive");
    assert_eq!(level.objects[1].dynamic, Some(20.0));
    assert_eq!(level.objects[1].interact.as_deref(), Some("open"));
    let (lo, hi) = level.bounds().expect("bounds");
    assert!(lo.x <= -20.0 && hi.x >= 20.0 && lo.z <= -20.0 && hi.z >= 20.0);
}

#[test]
fn actor_and_prop_problems_are_listed() {
    let bad = ACTORS
        .replace("spawn: \"B\"", "spawn: \"nowhere\"")
        .replace("speed: 1.2", "speed: 0.0")
        .replace("Box(size: (0.6, 0.6, 0.6))", "Cylinder(radius: 0.3, height: 0.6)")
        .replace("trigger: \"fidget\"", "trigger: \"\"");
    let Err(LevelError::Invalid { problems, .. }) = Level::parse(&bad) else { panic!("invalid level accepted") };
    assert_eq!(problems.len(), 4, "{problems:#?}");
}
