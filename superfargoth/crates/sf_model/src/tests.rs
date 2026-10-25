// Exact float equality is intended: cooked data must reproduce inputs bit for bit.
#![allow(clippy::float_cmp)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use glam::Mat4;
use sf_bake::{BakeError, Baker, Digest, Inputs, ProductKey, Store};

use super::steps::{Importers, MeshCookStep, ModelInfoStep};

use super::*;

/// A wavy grid: `n × n` quads, enough triangles for a real LOD chain.
fn terrain_like(n: u32, material: u32) -> MeshData {
    let mut mesh = MeshData { material, normals: Some(Vec::new()), uvs: Some(Vec::new()), ..MeshData::default() };
    for z in 0..=n {
        for x in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            let (fx, fz) = (x as f32 / n as f32, z as f32 / n as f32);
            let y = 0.2 * (fx * 9.0).sin() * (fz * 7.0).cos();
            mesh.positions.push([fx * 4.0 - 2.0, y, fz * 4.0 - 2.0]);
            mesh.normals.as_mut().unwrap().push([0.0, 1.0, 0.0]);
            mesh.uvs.as_mut().unwrap().push([fx, fz]);
        }
    }
    let row = n + 1;
    for z in 0..n {
        for x in 0..n {
            let i = z * row + x;
            mesh.indices.extend([i, i + row, i + 1, i + 1, i + row, i + row + 1]);
        }
    }
    mesh
}

fn triangle(material: u32) -> MeshData {
    MeshData {
        positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        normals: Some(vec![[0.0, 0.0, 1.0]; 3]),
        indices: vec![0, 1, 2],
        material,
        ..MeshData::default()
    }
}

fn sample_model() -> Model {
    let mut root = Node::new("root");
    root.mesh = Some(0);
    let mut detail = Node::new("detail");
    detail.parent = Some(0);
    detail.translation = [0.0, 1.0, 0.0];
    detail.mesh = Some(1);
    detail.lod = LodPolicy::Drop;
    let mut sign = Node::new("sign");
    sign.parent = Some(0);
    sign.translation = [5.0, 0.0, 0.0];
    sign.mesh = Some(2);
    sign.lod = LodPolicy::Keep;
    Model {
        nodes: vec![root, detail, sign],
        meshes: vec![terrain_like(64, 0), terrain_like(4, 1), triangle(1)],
        materials: vec![Material { name: "ground".into(), ..Material::default() }, Material::default()],
    }
}

fn cooked(model: &Model) -> CookedModel {
    match cook(model, &CookSettings::default()).unwrap() {
        Cooked::Static(c) => c,
        Cooked::Skipped { reason } => panic!("skipped: {reason}"),
        Cooked::Skinned(_) => panic!("skinned"),
    }
}

#[test]
fn lod_chain_shrinks_and_errors_grow() {
    let model = sample_model();
    let c = cooked(&model);
    let source_tris: usize = model.meshes.iter().map(MeshData::triangle_count).sum();
    assert_eq!(c.lods[0].triangle_count() as usize, source_tris);
    assert!(c.lods.len() >= 3, "expected a real chain, got {} levels", c.lods.len());
    for pair in c.lods.windows(2) {
        #[allow(clippy::cast_precision_loss)]
        let ratio = pair[1].triangle_count() as f32 / pair[0].triangle_count() as f32;
        assert!(ratio <= 0.75, "each level removes ≥ 25%: {ratio}");
        assert!(pair[1].error >= pair[0].error);
    }
    assert_eq!(c.lods[0].error, 0.0);
    // Errors stay small relative to the model (radius ≈ 3.6 m).
    assert!(c.lods[1].error < 0.2, "{}", c.lods[1].error);
}

#[test]
fn every_level_decodes_to_valid_geometry() {
    let c = cooked(&sample_model());
    for lod in &c.lods {
        for sub in &lod.submeshes {
            let (vertices, indices) = sub.decode().unwrap();
            assert_eq!(indices.len() % 3, 0);
            assert!(indices.iter().all(|&i| (i as usize) < vertices.len()));
            // Compact: every vertex is referenced.
            let mut used = vec![false; vertices.len()];
            for &i in &indices {
                used[i as usize] = true;
            }
            assert!(used.iter().all(|u| *u));
        }
    }
}

#[test]
fn keep_nodes_survive_and_drop_nodes_vanish() {
    let c = cooked(&sample_model());
    let sign = [[5.0, 0.0, 0.0], [6.0, 0.0, 0.0], [5.0, 1.0, 0.0]];
    for (level, lod) in c.lods.iter().enumerate() {
        let mut positions = Vec::new();
        for sub in lod.submeshes.iter().filter(|s| s.material == 1) {
            positions.extend(sub.decode().unwrap().0.into_iter().map(|v| v.position));
        }
        for p in sign {
            assert!(positions.contains(&p), "LOD{level} lost the kept triangle");
        }
        if level > 0 {
            // Only the kept triangle remains in material 1.
            assert_eq!(positions.len(), 3, "LOD{level} still has dropped detail");
        }
    }
}

#[test]
fn cooking_is_deterministic() {
    let model = sample_model();
    let a = cook(&model, &CookSettings::default()).unwrap().to_bytes();
    let b = cook(&model, &CookSettings::default()).unwrap().to_bytes();
    assert_eq!(a, b);
}

#[test]
fn animated_and_skinned_models_are_skipped() {
    let mut model = sample_model();
    model.nodes[1].animated = true;
    assert!(matches!(cook(&model, &CookSettings::default()).unwrap(), Cooked::Skipped { .. }));
    let mut model = sample_model();
    model.meshes[0].skinned = true;
    assert!(matches!(cook(&model, &CookSettings::default()).unwrap(), Cooked::Skipped { .. }));
}

/// A segmented body: `body` and `leg` transform nodes, a rigid mesh under
/// each (the leg's 1 m below its node), no skin.
fn segmented_model() -> Model {
    let body = Node::new("body");
    let mut leg = Node::new("leg");
    leg.parent = Some(0);
    leg.translation = [0.0, 2.0, 0.0];
    let mut torso = Node::new("torso shape");
    torso.parent = Some(0);
    torso.mesh = Some(0);
    let mut shin = Node::new("shin shape");
    shin.parent = Some(1);
    shin.translation = [0.0, -1.0, 0.0];
    shin.mesh = Some(1);
    Model { nodes: vec![body, leg, torso, shin], meshes: vec![triangle(0), triangle(0)], materials: vec![Material::default()] }
}

#[test]
fn rig_cook_binds_rigid_meshes_to_their_transform_nodes() {
    let model = segmented_model();
    assert!(matches!(cook(&model, &CookSettings::default()).unwrap(), Cooked::Static(_)), "the world cook stays static");
    let Cooked::Skinned(rig) = cook_rig(&model, &CookSettings::default()).unwrap() else { panic!("rig cook must skin") };
    let names: Vec<&str> = rig.joints.iter().map(|j| j.name.as_str()).collect();
    assert_eq!(names, ["body", "leg"], "one joint per transform node holding a mesh");
    assert_eq!(rig.model.lods[0].submeshes.len(), 2, "one part per mesh, one material");
    // The shin's vertices sit in model space (leg at y 2, shin 1 m below);
    // the leg joint's inverse bind undoes its rest transform.
    let leg = rig.joints.iter().position(|j| j.name == "leg").unwrap();
    let inverse = Mat4::from_cols_array(&rig.joints[leg].inverse_bind);
    assert!((inverse.w_axis.y + 2.0).abs() < 1e-6);
    let sub = (0..2).map(|i| rig.decode_submesh(0, i).unwrap()).find(|s| s.skin.iter().all(|v| usize::from(v.joints[0]) == leg)).unwrap();
    assert!(sub.vertices.iter().all(|v| (v.position[1] - 1.0).abs() < 1e-6 || (v.position[1] - 2.0).abs() < 1e-6));
    assert!(sub.skin.iter().all(|v| v.weights[0] == 1.0), "rigid: full weight on one joint");
}

#[test]
fn rest_cook_freezes_animated_models_but_not_skinned_ones() {
    let mut model = sample_model();
    model.nodes[1].animated = true;
    let rest = cook_rest(&model, &CookSettings::default()).unwrap();
    let still = sample_model();
    let Cooked::Static(frozen) = rest else { panic!("rest cook skipped an animated model") };
    let Cooked::Static(plain) = cook(&still, &CookSettings::default()).unwrap() else { panic!("skipped") };
    assert_eq!(frozen.lods.len(), plain.lods.len(), "the rest pose cooks like the static model");
    assert_eq!(frozen.lods[0].triangle_count(), plain.lods[0].triangle_count());
    model.meshes[0].skinned = true;
    assert!(matches!(cook_rest(&model, &CookSettings::default()).unwrap(), Cooked::Skipped { .. }));
}

#[test]
fn mirrored_nodes_keep_their_winding() {
    let mut node = Node::new("mirror");
    node.mesh = Some(0);
    node.scale = [-1.0, 1.0, 1.0];
    let model = Model { nodes: vec![node], meshes: vec![triangle(0)], materials: vec![Material::default()] };
    let c = cooked(&model);
    let (verts, tri) = c.lods[0].submeshes[0].decode().unwrap();
    let [p0, p1, p2] = [0, 1, 2].map(|k| Vec3::from_array(verts[tri[k] as usize].position));
    let face = (p1 - p0).cross(p2 - p0);
    // Front face still points along the vertex normals (+Z).
    assert!(face.z > 0.0, "{face:?}");
    assert!(verts.iter().all(|vert| vert.normal == [0.0, 0.0, 1.0]));
}

#[test]
fn container_round_trips_and_rejects_garbage() {
    let c = Cooked::Static(cooked(&sample_model()));
    let bytes = c.to_bytes();
    assert_eq!(Cooked::from_bytes(&bytes).unwrap(), c);
    assert!(matches!(Cooked::from_bytes(b"not a mesh"), Err(FormatError::BadMagic)));
    let mut wrong = bytes.clone();
    wrong[8] = 99;
    assert!(matches!(Cooked::from_bytes(&wrong), Err(FormatError::Version(_))));
}

#[test]
fn model_info_reports_bounds_and_counts() {
    let info = ModelInfo::of(&sample_model());
    assert_eq!(info.bounds.min[0], -2.0);
    assert_eq!(info.bounds.max[0], 6.0);
    assert_eq!(info.triangles, 64 * 64 * 2 + 4 * 4 * 2 + 1);
    assert!(!info.animated && !info.skinned);
}

/// Throughput: the first bake must cook every model within its budget.
#[test]
fn cook_throughput() {
    let model = Model {
        nodes: {
            let mut n = Node::new("m");
            n.mesh = Some(0);
            vec![n]
        },
        meshes: vec![terrain_like(32, 0)],
        materials: vec![Material::default()],
    };
    let count = 50;
    let start = Instant::now();
    for _ in 0..count {
        cooked(&model);
    }
    let per_model = start.elapsed() / count;
    // ~2k triangles is a typical prop; report for the budget table.
    println!("cook: {per_model:?} per 2k-triangle model (single thread)");
    assert!(per_model.as_millis() < 200, "{per_model:?}");
}

// --- bake integration ---------------------------------------------------

/// Test importer: the file is a postcard-serialized `Model`.
struct PostcardImporter;
impl ModelImporter for PostcardImporter {
    fn extensions(&self) -> &[&str] {
        &["pcm"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        postcard::from_bytes(bytes).map_err(|e| ImportError { path: path.into(), message: e.to_string() })
    }
}

struct Files(HashMap<String, Vec<u8>>);
impl Inputs for Files {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        Ok(self.0.get(path).map(|b| Digest::of(b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        self.0.get(path).cloned().ok_or_else(|| BakeError::Other(format!("missing {path}")))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

#[test]
fn steps_bake_cook_and_info_through_sf_bake() {
    let dir = tempfile::tempdir().unwrap();
    let mut files = HashMap::new();
    files.insert("meshes/rock.pcm".to_owned(), postcard::to_stdvec(&sample_model()).unwrap());
    let inputs = Arc::new(Files(files));
    let importers = Importers::new().with(Arc::new(PostcardImporter));
    let make = || {
        let mut b = Baker::new(Store::open(dir.path()).unwrap(), inputs.clone());
        b.register(ModelInfoStep { importers: importers.clone() })
            .register(MeshCookStep { importers: importers.clone(), settings: CookSettings::default() });
        b
    };
    let keys = [ProductKey::new("mesh.cook", "meshes/rock.pcm"), ProductKey::new("model.info", "meshes/rock.pcm")];
    let baker = make();
    let report = baker.bake(&keys);
    assert_eq!(report.failures().count(), 0, "{:?}", report.products);
    let cooked_digest = baker.product(&keys[0]).unwrap();
    let bytes = baker.store().get(&cooked_digest).unwrap();
    assert!(matches!(Cooked::from_bytes(&bytes).unwrap(), Cooked::Static(_)));
    let info = ModelInfoStep::decode(&baker.store().get(&baker.product(&keys[1]).unwrap()).unwrap()).unwrap();
    assert_eq!(info.bounds.max[0], 6.0);

    let again = make();
    again.bake(&keys);
    assert_eq!(again.step_stats()["mesh.cook"].hits, 1);
}

#[test]
fn obj_export_lists_every_level() {
    let c = cooked(&sample_model());
    let obj = c.to_obj().unwrap();
    assert_eq!(obj.matches("\no lod").count(), c.lods.iter().map(|l| l.submeshes.len()).sum::<usize>());
    let faces = obj.lines().filter(|l| l.starts_with("f ")).count();
    assert_eq!(faces, c.lods.iter().map(|l| l.triangle_count() as usize).sum::<usize>());
    let dir = std::env::temp_dir().join("sf_model_obj_export");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("sample.obj"), obj).unwrap();
}

#[test]
fn lod_ranges_tile_the_distance_axis() {
    let c = cooked(&sample_model());
    let screen = LodScreen::default();
    let errors: Vec<f32> = c.lods.iter().map(|l| l.error).collect();
    let ranges = lod_ranges(&errors, &screen, 3000.0);
    assert_eq!(ranges.len(), c.lods.len());
    assert_eq!(ranges[0].start, 0.0);
    assert_eq!(ranges.last().unwrap().end, 3000.0);
    for pair in ranges.windows(2) {
        assert_eq!(pair[0].end, pair[1].start, "no gaps or overlaps");
        assert!(pair[0].end >= pair[0].start);
    }
    // 1 cm of error at 1440p / 70° / 1 px switches at about 10 m.
    let d = screen.distance(0.01);
    assert!((d - 10.28).abs() < 0.05, "{d}");
    let biased = LodScreen { bias: 2.0, ..screen };
    assert_eq!(biased.distance(0.01), d * 2.0);
}

/// Size-based draw distance (industry-audit.md J1): the High preset (2 px,
/// 24 m floor) puts a 6 cm cup at ~62 m and a 45 cm barrel at ~463 m.
#[test]
fn draw_distance_from_screen_size() {
    let high = LodScreen::default();
    assert!((high.focal_px() - 1028.3).abs() < 0.1, "{}", high.focal_px());
    let cup = high.draw_distance(0.06);
    assert!((cup - 61.7).abs() < 0.1, "{cup}");
    assert!((high.draw_distance(0.45) - 462.7).abs() < 0.2);
    // The floor, and bias scaling both.
    assert_eq!(high.draw_distance(0.001), 24.0);
    let low = LodScreen { min_pixels: 4.0, min_draw_distance: 16.0, bias: 0.75, ..high };
    assert!((low.draw_distance(0.06) - 23.1).abs() < 0.1);
    assert_eq!(low.draw_distance(0.0), 16.0);
    // Off.
    let off = LodScreen { min_pixels: 0.0, ..high };
    assert_eq!(off.draw_distance(0.06), f32::INFINITY);
}

// ---- Skinned and morphed models (bake.md, "Skinned and morphed models") ----

/// An arm: a cylinder along +X (0..2 m) skinned to `upper` (0..1 m) and
/// `lower` (1..2 m) with a blend around the elbow, a "bulge" morph target on
/// the upper arm, and a rigid, mirrored triangle under `hand`.
fn arm_model(rings: u32, segments: u32) -> Model {
    let mut root = Node::new("root");
    root.translation = [0.0, 1.0, 0.0];
    let mut upper = Node::new("upper");
    upper.parent = Some(0);
    let mut lower = Node::new("lower");
    lower.parent = Some(1);
    lower.translation = [1.0, 0.0, 0.0];
    let mut hand = Node::new("hand");
    hand.parent = Some(2);
    hand.translation = [1.0, 0.0, 0.0];
    hand.animated = true;
    let mut arm = Node::new("arm_mesh");
    arm.parent = Some(0);
    arm.translation = [5.0, 5.0, 5.0]; // Ignored: skinned meshes live in bind space.
    arm.mesh = Some(0);
    let mut ring = Node::new("ring");
    ring.parent = Some(3);
    ring.scale = [-1.0, 1.0, 1.0];
    ring.mesh = Some(1);

    // Bind space = model space at rest: the root lifts everything by 1 m.
    let mut mesh = MeshData { normals: Some(Vec::new()), uvs: Some(Vec::new()), ..MeshData::default() };
    let mut influences = Vec::new();
    let mut bulge = Vec::new();
    for r in 0..=rings {
        #[allow(clippy::cast_precision_loss)]
        let x = 2.0 * r as f32 / rings as f32;
        let blend = ((x - 0.8) / 0.4).clamp(0.0, 1.0);
        for s in 0..segments {
            #[allow(clippy::cast_precision_loss)]
            let angle = std::f32::consts::TAU * s as f32 / segments as f32;
            let n = [0.0, angle.cos(), angle.sin()];
            mesh.positions.push([x, 1.0 + 0.2 * n[1], 0.2 * n[2]]);
            mesh.normals.as_mut().unwrap().push(n);
            #[allow(clippy::cast_precision_loss)]
            mesh.uvs.as_mut().unwrap().push([x / 2.0, s as f32 / segments as f32]);
            // A zero weight, a split duplicate, and up to six influences:
            // the cook merges, drops, keeps four and renormalizes.
            let mut inf = vec![(0u16, (1.0 - blend) * 0.5), (0, (1.0 - blend) * 0.5), (1, blend), (1, 0.0)];
            if r % 7 == 3 {
                inf.extend([(0, 0.01), (1, 0.02), (0, 0.03)]);
            }
            influences.push(inf);
            let k = if (0.3..0.7).contains(&x) { 0.05 } else { 0.0 };
            bulge.push(n.map(|c| c * k));
        }
    }
    for r in 0..rings {
        for s in 0..segments {
            let i = r * segments + s;
            let j = r * segments + (s + 1) % segments;
            mesh.indices.extend([i, j, i + segments, j, j + segments, i + segments]);
        }
    }
    mesh.skin = Some(MeshSkin {
        joints: vec![
            SkinJoint { node: 1, inverse_bind: Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0)).to_cols_array() },
            SkinJoint { node: 2, inverse_bind: Mat4::from_translation(Vec3::new(-1.0, -1.0, 0.0)).to_cols_array() },
        ],
        influences,
    });
    mesh.morph_targets = vec![MorphTarget { name: "bulge".into(), positions: bulge, normals: None, default_weight: 0.25, kind: MorphKind::Expression }];

    Model {
        nodes: vec![root, upper, lower, hand, arm, ring],
        meshes: vec![mesh, triangle(1)],
        materials: vec![Material { name: "skin".into(), ..Material::default() }, Material { name: "metal".into(), ..Material::default() }],
    }
}

fn skinned(model: &Model) -> CookedSkinned {
    match cook(model, &CookSettings::default()).unwrap() {
        Cooked::Skinned(s) => s,
        other => panic!("expected a skinned cook, got {other:?}"),
    }
}

fn header_version(bytes: &[u8]) -> u32 {
    u32::from_le_bytes(bytes[8..12].try_into().unwrap())
}

#[test]
fn skinned_container_round_trips_as_v3_and_static_stays_v1() {
    let s = Cooked::Skinned(skinned(&arm_model(24, 12)));
    let bytes = s.to_bytes();
    assert_eq!(header_version(&bytes), 3);
    assert_eq!(Cooked::from_bytes(&bytes).unwrap(), s);

    // Static cooks keep the v1 header and body, so v1 caches stay valid.
    let st = Cooked::Static(cooked(&sample_model()));
    let v1 = st.to_bytes();
    assert_eq!(header_version(&v1), 1);
    assert_eq!(Cooked::from_bytes(&v1).unwrap(), st);
    // A v1 body under a v3 header is the same value.
    let mut as_v3 = v1.clone();
    as_v3[8..12].copy_from_slice(&3u32.to_le_bytes());
    assert_eq!(Cooked::from_bytes(&as_v3).unwrap(), st);
    // v2 (the dense morph layout) is retired, whatever the body.
    as_v3[8..12].copy_from_slice(&2u32.to_le_bytes());
    assert!(matches!(Cooked::from_bytes(&as_v3), Err(FormatError::Version(2))));
    // A skinned body under a v1 header, and unknown versions, are rejected cleanly.
    let mut lying = bytes.clone();
    lying[8..12].copy_from_slice(&1u32.to_le_bytes());
    assert!(matches!(Cooked::from_bytes(&lying), Err(FormatError::Body(_))));
    let mut future = bytes;
    future[8..12].copy_from_slice(&4u32.to_le_bytes());
    assert!(matches!(Cooked::from_bytes(&future), Err(FormatError::Version(4))));
}

#[test]
fn skinned_cook_keeps_bind_space_palette_parts_and_morphs() {
    let s = skinned(&arm_model(24, 12));
    let names: Vec<&str> = s.joints.iter().map(|j| j.name.as_str()).collect();
    assert_eq!(names, ["upper", "lower", "hand"], "skin joints, then the rigid part's joint");
    // The rigid joint's inverse bind inverts `hand`'s rest transform (root +1 y, +2 x).
    let hand = Mat4::from_cols_array(&s.joints[2].inverse_bind);
    assert!(hand.abs_diff_eq(Mat4::from_translation(Vec3::new(-2.0, -1.0, 0.0)), 1e-6), "{hand}");
    assert_eq!(s.morph_targets, [MorphTargetInfo { name: "bulge".into(), default_weight: 0.25, kind: MorphKind::Expression }]);
    assert_eq!(s.parts.len(), 2);
    assert_eq!(s.parts[0].node_path, ["root", "arm_mesh"]);
    assert_eq!(s.parts[1].node_path, ["root", "upper", "lower", "hand", "ring"]);
    // Bind space: the skinned mesh's node transform (5, 5, 5) is ignored.
    let b = s.model.bounds;
    assert!(b.max[0] <= 2.0 + 1e-5 && b.min[1] >= 0.79 && b.max[2] <= 0.21, "{b:?}");

    for (i, (sub, streams)) in s.model.lods[0].submeshes.iter().zip(&s.streams[0]).enumerate() {
        let (vertices, indices) = sub.decode().unwrap();
        let (skin, morphs) = s.decode_streams(0, i).unwrap();
        assert_eq!(skin.len(), vertices.len());
        if streams.part == 1 {
            // A single-joint skin on `hand`, mirrored winding restored, no morphs.
            assert!(skin.iter().all(|v| v.joints[0] == 2 && v.weights == [1.0, 0.0, 0.0, 0.0]));
            assert!(morphs.is_empty());
            let [p0, p1, p2] = [0, 1, 2].map(|k| Vec3::from_array(vertices[indices[k] as usize].position));
            assert!((p1 - p0).cross(p2 - p0).z > 0.0, "front face follows the normals");
            // Flattened to model space at rest (hand at x = 2, y = 1; mirrored in x).
            assert!(vertices.iter().any(|v| v.position == [1.0, 1.0, 0.0]), "{vertices:?}");
        } else {
            assert_eq!(morphs.len(), vertices.len(), "one target, target-major");
            for (v, d) in vertices.iter().zip(&morphs) {
                let expect = if (0.3..0.7).contains(&v.position[0]) { 0.05 } else { 0.0 };
                assert!((Vec3::from_array(d.position).length() - expect).abs() < 1e-5, "{v:?} {d:?}");
            }
        }
    }
}

#[test]
fn skinned_lods_keep_weights_normalized_and_joints_valid() {
    let s = skinned(&arm_model(64, 32));
    assert!(s.model.lods.len() >= 3, "expected a real chain, got {} levels", s.model.lods.len());
    assert_eq!(s.streams.len(), s.model.lods.len());
    for pair in s.model.lods.windows(2) {
        assert!(pair[1].triangle_count() < pair[0].triangle_count());
    }
    for (level, lod) in s.model.lods.iter().enumerate() {
        assert_eq!(s.streams[level].len(), lod.submeshes.len());
        for (i, sub) in lod.submeshes.iter().enumerate() {
            let (vertices, _) = sub.decode().unwrap();
            let (skin, morphs) = s.decode_streams(level, i).unwrap();
            assert_eq!(skin.len(), vertices.len());
            assert_eq!(morphs.len(), vertices.len() * s.streams[level][i].targets.len());
            for v in &skin {
                let sum: f32 = v.weights.iter().sum();
                assert!((sum - 1.0).abs() < 1e-5, "level {level}: weights {v:?}");
                assert!(v.weights.windows(2).all(|w| w[0] >= w[1]), "largest first: {v:?}");
                assert!(v.joints.iter().all(|&j| usize::from(j) < s.joints.len()));
            }
        }
    }
    // Rigid regions on both sides of the elbow survive to the last level.
    let last = s.model.lods.len() - 1;
    let arm = s.streams[last].iter().position(|st| st.part == 0).unwrap();
    let (skin, _) = s.decode_streams(last, arm).unwrap();
    assert!(skin.iter().any(|v| v.joints[0] == 0 && v.weights[0] == 1.0));
    assert!(skin.iter().any(|v| v.joints[0] == 1 && v.weights[0] == 1.0));
}

#[test]
fn skinned_cooking_is_deterministic_and_skins_without_data_skip() {
    let model = arm_model(24, 12);
    let a = cook(&model, &CookSettings::default()).unwrap().to_bytes();
    let b = cook(&model, &CookSettings::default()).unwrap().to_bytes();
    assert_eq!(a, b);
    let mut model = arm_model(8, 6);
    model.meshes[1].skinned = true; // Known skinned, skin not read (yet).
    assert!(matches!(cook(&model, &CookSettings::default()).unwrap(), Cooked::Skipped { .. }));
    // A skinned model has no rest-pose cook.
    assert!(matches!(cook_rest(&arm_model(8, 6), &CookSettings::default()).unwrap(), Cooked::Skipped { .. }));
    let info = ModelInfo::of(&arm_model(8, 6));
    assert!(info.skinned && info.bounds.max[0] < 2.5, "{info:?}");
}

#[test]
fn rigid_parts_become_single_joint_skins_and_mirrors_flip_winding() {
    let st = Cooked::Static(cooked(&sample_model()));
    let mirror = Affine3A::from_scale(Vec3::new(-1.0, 1.0, 1.0)) * Affine3A::from_translation(Vec3::new(0.0, 0.1, 0.0));
    let part = CookedSkinned::rigid(&st, "Hand.L", mirror).unwrap().unwrap();
    assert_eq!(part.joints.len(), 1);
    assert_eq!(part.joints[0].name, "Hand.L");
    assert_eq!(part.joints[0].inverse_bind, Mat4::from(mirror).to_cols_array());
    let Cooked::Static(plain) = &st else { unreachable!() };
    for (level, lod) in part.model.lods.iter().enumerate() {
        for (i, sub) in lod.submeshes.iter().enumerate() {
            let (_, flipped) = sub.decode().unwrap();
            let (_, original) = plain.lods[level].submeshes[i].decode().unwrap();
            // The index codec may rotate triangles; compare them up to rotation.
            let canon = |t: [u32; 3]| (0..3).map(|r| [t[r], t[(r + 1) % 3], t[(r + 2) % 3]]).min().unwrap();
            for (f, o) in flipped.as_chunks::<3>().0.iter().zip(original.as_chunks::<3>().0) {
                assert_eq!(canon(*f), canon([o[0], o[2], o[1]]));
            }
            let (skin, _) = part.decode_streams(level, i).unwrap();
            assert!(skin.iter().all(|v| v.joints == [0; 4] && v.weights == [1.0, 0.0, 0.0, 0.0]));
        }
    }
    // Unmirrored: geometry untouched; a skinned cook keeps its morphs and parts.
    let s = Cooked::Skinned(skinned(&arm_model(16, 8)));
    let part = CookedSkinned::rigid(&s, "Head", Affine3A::IDENTITY).unwrap().unwrap();
    let Cooked::Skinned(src) = &s else { unreachable!() };
    assert_eq!(part.model, src.model);
    assert_eq!(part.parts, src.parts);
    assert_eq!(part.morph_targets, src.morph_targets);
    assert!(CookedSkinned::rigid(&Cooked::Skipped { reason: String::new() }, "x", Affine3A::IDENTITY).unwrap().is_none());
}

/// The arm with a shape target ("fat", on the arm), an expression target
/// that is zero on the ring ("bulge" there) and one that moves nothing
/// anywhere ("still").
fn morph_layout_model(rings: u32, segments: u32) -> Model {
    let mut model = arm_model(rings, segments);
    let arm = &mut model.meshes[0];
    let fat = arm.normals.as_ref().unwrap().iter().map(|n| n.map(|c| c * 0.03)).collect();
    let still = vec![[0.0; 3]; arm.positions.len()];
    arm.morph_targets.push(MorphTarget { name: "fat".into(), positions: fat, normals: None, default_weight: 0.5, kind: MorphKind::Shape });
    arm.morph_targets.push(MorphTarget { name: "still".into(), positions: still, normals: None, default_weight: 0.0, kind: MorphKind::Expression });
    // Below MORPH_EPSILON: importer noise, stripped like zeros.
    let noise = vec![[1e-7, 0.0, 0.0]; 3];
    model.meshes[1].morph_targets = vec![MorphTarget { name: "bulge".into(), positions: noise, ..MorphTarget::default() }];
    model
}

#[test]
fn morph_layout_is_per_part_strips_zero_targets_and_drops_expressions_past_lod1() {
    let s = skinned(&morph_layout_model(64, 32));
    let names: Vec<(&str, MorphKind)> = s.morph_targets.iter().map(|t| (t.name.as_str(), t.kind)).collect();
    assert_eq!(names, [("bulge", MorphKind::Expression), ("fat", MorphKind::Shape)], "\"still\" moves nothing: dropped");
    assert!(s.model.lods.len() > RUNTIME_MORPH_LODS, "need a coarse level, got {}", s.model.lods.len());
    for (level, streams) in s.streams.iter().enumerate() {
        for (i, st) in streams.iter().enumerate() {
            let (skin, morphs) = s.decode_streams(level, i).unwrap();
            assert_eq!(morphs.len(), skin.len() * st.targets.len());
            if st.part == 1 {
                assert!(st.targets.is_empty() && st.morphs.is_none(), "the ring's bulge is zero: no block");
            } else if level < RUNTIME_MORPH_LODS {
                assert_eq!(st.targets, [0, 1], "level {level}");
            } else {
                assert_eq!(st.targets, [1], "level {level}: only the shape target past LOD {}", RUNTIME_MORPH_LODS - 1);
            }
        }
    }
    // Baking the shape leaves the expression target; the shape's deltas land in the vertices.
    let arm = s.streams[0].iter().position(|st| st.part == 0).unwrap();
    let mut sub = s.decode_submesh(0, arm).unwrap();
    let before = sub.clone();
    sub.bake_shapes(&s.morph_targets, |_| 0.6);
    assert_eq!(sub.targets, [0]);
    let n = sub.vertices.len();
    assert_eq!(sub.morphs, before.morphs[..n]);
    for (v, (b, d)) in sub.vertices.iter().zip(before.vertices.iter().zip(&before.morphs[n..])) {
        for c in 0..3 {
            assert!((v.position[c] - (b.position[c] + 0.6 * d.position[c])).abs() < 1e-6);
        }
    }
}
