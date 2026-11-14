//! Cooked tangents read normal maps the way Bevy's PBR shader does: a
//! canonical (+Y up) normal-map texel tilted toward the image's top must tilt
//! the shaded normal toward the world direction of image-up, and cooked
//! tangents must agree with Bevy's own `Mesh::generate_tangents`.
//!
//! The shader side is a CPU copy of `bevy_pbr` 0.20
//! `src/render/pbr_functions.wesl` `calculate_tbn_mikktspace` (`B = w *
//! cross(N, T)`, line 162) and `apply_normal_mapping` (`Nt.x * T + Nt.y * B +
//! Nt.z * N`). Image-up is `-dP/dv`: Bevy (wgpu, glTF) puts UV (0, 0) at the
//! image's top-left, so rows (and V) run downward.

// Handedness is exactly ±1; the shader's own names (T, B, N, dP/du, dP/dv).
#![allow(clippy::float_cmp, clippy::many_single_char_names, clippy::similar_names)]

use bevy_asset::RenderAssetUsages;
use bevy_math::Vec3;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues};
use sf_model::{CookSettings, Cooked, Material, MeshData, Model, Node, Tangent, Vertex, cook_with};

/// A unit quad in the XZ plane facing +Y, U along +X (or −X when
/// `mirrored`), V along +Z.
fn quad(mirrored: bool) -> Model {
    let corners = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let mesh = MeshData {
        positions: corners.iter().map(|&[x, z]| [x, 0.0, z]).collect(),
        normals: Some(vec![[0.0, 1.0, 0.0]; 4]),
        uvs: Some(corners.iter().map(|&[x, z]| [if mirrored { 1.0 - x } else { x }, z]).collect()),
        indices: vec![0, 2, 1, 1, 2, 3],
        ..MeshData::default()
    };
    let mut node = Node::new("quad");
    node.mesh = Some(0);
    Model { nodes: vec![node], meshes: vec![mesh], materials: vec![Material { name: "bumpy".into(), ..Material::default() }] }
}

/// The quad cooked with tangents: its first LOD's vertices, indices, tangents.
fn cooked(mirrored: bool) -> (Vec<Vertex>, Vec<u32>, Vec<Tangent>) {
    let Cooked::Static(c) = cook_with(&quad(mirrored), &CookSettings::default(), &[true]).unwrap() else { panic!("static") };
    let sub = &c.lods[0].submeshes[0];
    let (vertices, indices) = sub.decode().unwrap();
    (vertices, indices, sub.decode_tangents().unwrap().expect("tangents"))
}

/// `dP/du` and `dP/dv` of the first triangle.
fn uv_gradients(vertices: &[Vertex], indices: &[u32]) -> (Vec3, Vec3) {
    let v = |k: usize| vertices[indices[k] as usize];
    let p = |k: usize| Vec3::from_array(v(k).position);
    let (e1, e2) = (p(1) - p(0), p(2) - p(0));
    let (du1, dv1) = (v(1).uv[0] - v(0).uv[0], v(1).uv[1] - v(0).uv[1]);
    let (du2, dv2) = (v(2).uv[0] - v(0).uv[0], v(2).uv[1] - v(0).uv[1]);
    let r = 1.0 / (du1 * dv2 - du2 * dv1);
    ((e1 * dv2 - e2 * dv1) * r, (e2 * du1 - e1 * du2) * r)
}

/// Bevy's tangent-space → world normal for one vertex.
fn shade(normal: [f32; 3], [x, y, z, w]: Tangent, nt: Vec3) -> Vec3 {
    let n = Vec3::from_array(normal);
    let t = Vec3::new(x, y, z);
    let b = w * n.cross(t);
    (nt.x * t + nt.y * b + nt.z * n).normalize()
}

#[test]
fn cooked_tangents_put_canonical_green_toward_image_up() {
    for mirrored in [false, true] {
        let (vertices, indices, tangents) = cooked(mirrored);
        let (dpdu, dpdv) = uv_gradients(&vertices, &indices);
        let (image_up, image_right) = (-dpdv.normalize(), dpdu.normalize());
        for (vertex, &tangent) in vertices.iter().zip(&tangents) {
            let up = shade(vertex.normal, tangent, Vec3::new(0.0, 0.6, 0.8));
            assert!(up.dot(image_up) > 0.59, "mirrored {mirrored}: +Y texel shades toward {up}, image up is {image_up}");
            assert!(up.dot(image_right).abs() < 1e-4, "mirrored {mirrored}: no sideways tilt ({up})");
            let right = shade(vertex.normal, tangent, Vec3::new(0.6, 0.0, 0.8));
            assert!(right.dot(image_right) > 0.59, "mirrored {mirrored}: +X texel shades toward {right}, image right is {image_right}");
        }
    }
}

#[test]
fn cooked_tangents_match_bevy_generate_tangents() {
    for mirrored in [false, true] {
        let (vertices, indices, tangents) = cooked(mirrored);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.iter().map(|v| v.position).collect::<Vec<_>>());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertices.iter().map(|v| v.normal).collect::<Vec<_>>());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vertices.iter().map(|v| v.uv).collect::<Vec<_>>());
        mesh.insert_indices(Indices::U32(indices));
        mesh.generate_tangents().unwrap();
        let Some(VertexAttributeValues::Float32x4(bevy)) = mesh.attribute(Mesh::ATTRIBUTE_TANGENT) else { panic!("tangents") };
        assert_eq!(bevy.len(), tangents.len());
        for (ours, theirs) in tangents.iter().zip(bevy) {
            assert_eq!(ours[3], theirs[3], "mirrored {mirrored}: handedness {ours:?} vs Bevy {theirs:?}");
            let delta = Vec3::from_slice(ours) - Vec3::from_slice(theirs);
            assert!(delta.length() < 1e-4, "mirrored {mirrored}: tangent {ours:?} vs Bevy {theirs:?}");
        }
    }
    // The two cases really differ in handedness.
    let w = |mirrored| cooked(mirrored).2.iter().map(|t| t[3]).collect::<Vec<_>>();
    let (plain, mirror) = (w(false), w(true));
    assert!(plain.iter().all(|&s| s == plain[0]) && mirror.iter().all(|&s| s == -plain[0]));
}
