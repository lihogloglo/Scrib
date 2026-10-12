//! Outfit assembly with morph-bearing parts: merged meshes never morph,
//! shape targets bake to exactly the runtime morph, memory before/after.

use glam::{Mat4, Vec3};
use sf_model::{Cooked, CookSettings, CookedSkinned, Material, MeshData, MeshSkin, Model, MorphKind, MorphTarget, Node, SkinJoint, cook};

use crate::*;

/// A `rows × cols` grid in the XY plane starting at height `y0`, skinned to
/// the part's joints `spine` and `head` (blended between 1 and 1.5 m),
/// facing +Z.
fn grid(rows: u32, cols: u32, y0: f32, material: u32) -> MeshData {
    let mut mesh = MeshData { normals: Some(Vec::new()), uvs: Some(Vec::new()), material, ..MeshData::default() };
    let mut influences = Vec::new();
    for r in 0..=rows {
        for c in 0..=cols {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (c as f32 / cols as f32 - 0.5, y0 + r as f32 / rows as f32 * 0.5);
            mesh.positions.push([x, y, 0.0]);
            mesh.normals.as_mut().unwrap().push([0.0, 0.0, 1.0]);
            mesh.uvs.as_mut().unwrap().push([x, y]);
            let t = ((y - 1.0) / 0.5).clamp(0.0, 1.0);
            influences.push(vec![(0, 1.0 - t), (1, t)]);
        }
    }
    for r in 0..rows {
        for c in 0..cols {
            let i = r * (cols + 1) + c;
            mesh.indices.extend([i, i + 1, i + cols + 1, i + 1, i + cols + 2, i + cols + 1]);
        }
    }
    let joint = |node: u32, y: f32| SkinJoint { node, inverse_bind: Mat4::from_translation(Vec3::new(0.0, -y, 0.0)).to_cols_array() };
    mesh.skin = Some(MeshSkin { joints: vec![joint(1, 1.0), joint(2, 1.5)], influences });
    mesh
}

/// A part file: root → spine (1 m) → head (1.5 m), one mesh node per entry
/// of `meshes`, materials "skin" (0) and "cloth" (1).
fn part_model(meshes: Vec<MeshData>) -> CookedSkinned {
    let mut spine = Node::new("spine");
    spine.parent = Some(0);
    spine.translation = [0.0, 1.0, 0.0];
    let mut head = Node::new("head");
    head.parent = Some(1);
    head.translation = [0.0, 0.5, 0.0];
    let mut nodes = vec![Node::new("root"), spine, head];
    for i in 0..meshes.len() {
        let mut n = Node::new(format!("mesh{i}"));
        n.parent = Some(0);
        n.mesh = Some(u32::try_from(i).unwrap());
        nodes.push(n);
    }
    let materials = ["skin", "cloth"].map(|name| Material { name: name.into(), ..Material::default() }).to_vec();
    match cook(&Model { nodes, meshes, materials }, &CookSettings::default()).unwrap() {
        Cooked::Skinned(s) => s,
        other => panic!("expected a skinned cook, got {other:?}"),
    }
}

/// A head grid (skin material) with a shape target "jaw" (default weight
/// 0.25, with normal deltas) and, optionally, an expression target "blink".
fn head(blink: bool) -> MeshData {
    let mut mesh = grid(8, 8, 1.5, 0);
    let n = mesh.positions.len();
    #[allow(clippy::cast_precision_loss)]
    let jaw: Vec<[f32; 3]> = (0..n).map(|i| [0.0, -0.01 * (i % 9) as f32, 0.02]).collect();
    let jaw_normals = Some(vec![[0.0, 0.1, 0.0]; n]);
    mesh.morph_targets.push(MorphTarget { name: "jaw".into(), positions: jaw, normals: jaw_normals, default_weight: 0.25, kind: MorphKind::Shape });
    if blink {
        let lid = (0..n).map(|i| if i % 5 == 0 { [0.0, 0.0, 0.01] } else { [0.0; 3] }).collect();
        mesh.morph_targets.push(MorphTarget { name: "blink".into(), positions: lid, normals: None, default_weight: 0.0, kind: MorphKind::Expression });
    }
    mesh
}

fn whole(cooked: &CookedSkinned) -> AssemblyPart<'_> {
    AssemblyPart::new(cooked, None)
}

#[test]
fn merged_outfit_carries_no_morph_data() {
    // Body (skin) + shirt (cloth) in one file; a head (skin) with blink + jaw in another.
    let body = part_model(vec![grid(10, 6, 0.5, 0), grid(6, 6, 0.8, 1)]);
    let face = part_model(vec![head(true)]);
    let outfit = assemble(&[whole(&body), whole(&face)], 0, &ShapeWeights::default()).unwrap();

    assert_eq!(outfit.merged.len(), 2, "one mesh per material");
    for mesh in &outfit.merged {
        assert!(mesh.morph_targets.is_empty() && mesh.morphs.is_empty(), "{}: merged meshes never morph", mesh.material.name);
    }
    // The head shares the body's material but keeps its own mesh, with only its runtime target.
    assert_eq!(outfit.morphed.len(), 1);
    let head_mesh = &outfit.morphed[0];
    assert_eq!(head_mesh.material.name, "skin");
    let names: Vec<&str> = head_mesh.morph_targets.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names, ["blink"], "the shape target is baked, not a GPU morph");
    assert_eq!(head_mesh.morphs.len(), head_mesh.vertices.len());
    // One shared palette: both files bind spine and head in the same pose.
    let joints: Vec<&str> = outfit.joints.iter().map(|j| j.name.as_str()).collect();
    assert_eq!(joints, ["spine", "head"]);
    assert!(outfit.meshes().flat_map(|m| &m.skin).flat_map(|s| s.joints).all(|j| usize::from(j) < outfit.joints.len()));

    // A head with only shape targets merges into the body's skin mesh.
    let still_face = part_model(vec![head(false)]);
    let outfit = assemble(&[whole(&body), whole(&still_face)], 0, &ShapeWeights::default()).unwrap();
    assert!(outfit.morphed.is_empty());
    let skin = outfit.merged.iter().find(|m| m.material.name == "skin").unwrap();
    assert!(skin.indices.iter().all(|&i| (i as usize) < skin.vertices.len()));
    assert_eq!(skin.indices.len(), (10 * 6 + 8 * 8) * 6, "body and head triangles");
    let bytes = outfit.gpu_bytes();
    assert_eq!(bytes.morphs, 0);
    assert_eq!(bytes.palettes, 2 * 2 * BEVY_JOINT_BYTES, "one palette copy per mesh");

    // A node filter keeps only the body grid.
    let filter = BodyPart {
        model: sf_vfs::VfsPath::new("meshes/body.glb").unwrap(),
        slot: SlotId(0),
        kind: PartKind::Skinned,
        node_filter: Some("mesh0".into()),
    };
    let outfit = assemble(&[AssemblyPart::new(&body, Some(&filter))], 0, &ShapeWeights::default()).unwrap();
    assert_eq!(outfit.merged.iter().map(|m| m.material.name.as_str()).collect::<Vec<_>>(), ["skin"]);

    // Pre-decoded parts (what smooth bodies hand over) assemble exactly like the cooked ones, filter included.
    let decoded = DecodedPart::decode(&body, 0, Some(&filter)).unwrap();
    assert_eq!(decoded.submeshes.len(), 1, "the filter applies at decode");
    let pre = AssemblyPart { decoded: Some(&decoded), ..AssemblyPart::new(&body, Some(&filter)) };
    assert_eq!(assemble(&[pre], 0, &ShapeWeights::default()).unwrap(), outfit);
}

#[test]
fn baked_shape_equals_runtime_morph_at_the_same_weight() {
    let face = part_model(vec![head(true)]);
    let jaw = u16::try_from(face.morph_targets.iter().position(|t| t.name == "jaw").unwrap()).unwrap();
    let reference = face.decode_submesh(0, 0).unwrap();
    let k = reference.targets.iter().position(|&t| t == jaw).unwrap();
    let n = reference.vertices.len();
    for (weights, w) in [(ShapeWeights(vec![("jaw".into(), 0.7)]), 0.7), (ShapeWeights::default(), 0.25)] {
        let outfit = assemble(&[whole(&face)], 0, &weights).unwrap();
        let baked = &outfit.morphed[0].vertices;
        assert_eq!(baked.len(), n);
        // The runtime morph of the cooked part: base + w · delta (glTF; Bevy's `morph_vertex`).
        for (v, (base, d)) in baked.iter().zip(reference.vertices.iter().zip(&reference.morphs[k * n..(k + 1) * n])) {
            for c in 0..3 {
                assert!((v.position[c] - (base.position[c] + w * d.position[c])).abs() < 1e-5, "{v:?} vs {base:?} + {w}·{d:?}");
                assert!((v.normal[c] - (base.normal[c] + w * d.normal[c])).abs() < 1e-5);
            }
        }
        assert!(baked.iter().zip(&reference.vertices).any(|(a, b)| (a.position[2] - b.position[2]).abs() > 1e-3), "the shape moved something");
    }
    // Shape weights key the cache: empty → 0, order-independent, value-sensitive.
    let a = ShapeWeights(vec![("jaw".into(), 0.7), ("brow".into(), 0.1)]);
    let b = ShapeWeights(vec![("brow".into(), 0.1), ("jaw".into(), 0.7)]);
    assert_eq!(ShapeWeights::default().key(), 0);
    assert_eq!(a.key(), b.key());
    assert_ne!(a.key(), ShapeWeights(vec![("jaw".into(), 0.71), ("brow".into(), 0.1)]).key());
}

/// Memory per cached outfit, old layout against new, for the spec's library
/// example: a 12k-vertex body and a ~2k-vertex head with 30 targets (24
/// expressions, 6 shapes) sharing the skin material.
#[test]
fn outfit_memory_before_and_after() {
    let mut face = grid(40, 47, 1.5, 0); // 41 × 48 = 1,968 vertices
    let n = face.positions.len();
    for t in 0..30_usize {
        #[allow(clippy::cast_precision_loss)]
        let positions = (0..n).map(|i| if i % 3 == t % 3 { [0.0, 0.0, 0.001 * (t + 1) as f32] } else { [0.0; 3] }).collect();
        let kind = if t < 24 { MorphKind::Expression } else { MorphKind::Shape };
        face.morph_targets.push(MorphTarget { name: format!("t{t}"), positions, normals: None, default_weight: 0.0, kind });
    }
    let body = part_model(vec![grid(119, 99, 0.0, 0)]); // 120 × 100 = 12,000 vertices
    let face = part_model(vec![face]);
    let outfit = assemble(&[whole(&body), whole(&face)], 0, &ShapeWeights::default()).unwrap();
    let after = outfit.gpu_bytes();
    let head_vertices = outfit.morphed[0].vertices.len();
    // Before (v2 cook + plain per-material merge): one skin mesh, all 30 targets dense over it.
    let vertices: usize = outfit.meshes().map(|m| m.vertices.len()).sum();
    let before_morphs = vertices * 30 * BEVY_MORPH_BYTES;
    let before = after.vertices + after.indices + before_morphs + outfit.joints.len() * BEVY_JOINT_BYTES;
    eprintln!(
        "outfit {vertices} vertices: before {:.2} MB (morphs {:.2} MB, 1 mesh); after {:.2} MB (morphs {:.2} MB, {} meshes)",
        mb(before),
        mb(before_morphs),
        mb(after.total()),
        mb(after.morphs),
        outfit.meshes().count()
    );
    assert_eq!(after.morphs, head_vertices * 24 * BEVY_MORPH_BYTES, "only the head's expression targets stay on the GPU");
    assert!(after.total() * 4 < before);
}

#[allow(clippy::cast_precision_loss)]
fn mb(bytes: usize) -> f64 {
    bytes as f64 / 1e6
}
