//! The framework's glTF 2.0 importer (binary `.glb`, or `.gltf` whose
//! buffers are all embedded in a GLB `BIN` chunk).
//!
//! glTF is already Y up, right-handed, in meters, like engine space, so
//! nothing is converted. External buffer files aren't read: the importer
//! sees one file's bytes, and modders ship `.glb` anyway.
//!
//! Skins: `JOINTS_n`/`WEIGHTS_n` (every set) become [`MeshSkin`]
//! influences, `skin.joints` + `inverseBindMatrices` its palette. Morph
//! targets: position/normal displacements (sparse accessors expand through
//! the `gltf` reader), names from `mesh.extras.targetNames`, default
//! weights from `mesh.weights`. Tangent displacements are dropped (the cook
//! keeps no tangents).

use std::collections::HashMap;

use gltf::buffer::Source as BufferSource;
use gltf::image::Source as ImageSource;
use gltf::mesh::Mode;

use crate::{
    AlphaMode, ImportError, LodPolicy, Material, MeshData, MeshSkin, Model, ModelImporter, MorphTarget, Node, SkinJoint,
};

/// Imports `.glb` / `.gltf` into a [`Model`].
#[derive(Clone, Copy, Debug, Default)]
pub struct GltfImporter;

impl ModelImporter for GltfImporter {
    fn extensions(&self) -> &[&str] {
        &["glb", "gltf"]
    }

    fn version(&self) -> u32 {
        // 2: skins and morph targets.
        2
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError> {
        let err = |message: String| ImportError { path: path.to_owned(), message };
        let gltf = gltf::Gltf::from_slice(bytes).map_err(|e| err(e.to_string()))?;
        let mut buffers = Vec::new();
        for buffer in gltf.document.buffers() {
            match buffer.source() {
                BufferSource::Bin => buffers.push(gltf.blob.clone().ok_or_else(|| err("missing BIN chunk".into()))?),
                BufferSource::Uri(uri) => return Err(err(format!("external buffer {uri:?}: ship a .glb instead"))),
            }
        }

        let mut model = Model {
            materials: gltf.document.materials().map(|m| material(path, &m)).collect(),
            ..Model::default()
        };
        // Primitives without a material use a trailing default material.
        let default_material = u32::try_from(model.materials.len()).map_err(|_| err("too many materials".into()))?;
        model.materials.push(Material { name: "default".into(), ..Material::default() });

        let animated: std::collections::HashSet<usize> = gltf
            .document
            .animations()
            .flat_map(|a| a.channels().map(|c| c.target().node().index()).collect::<Vec<_>>())
            .collect();
        let scene = gltf.document.default_scene().or_else(|| gltf.document.scenes().next());
        let Some(scene) = scene else { return Ok(model) };
        let mut stack: Vec<(gltf::Node<'_>, Option<u32>)> = scene.nodes().map(|n| (n, None)).collect::<Vec<_>>().into_iter().rev().collect();
        // glTF node index → model node index, and skinned meshes whose
        // palettes wait for every joint node to exist.
        let mut node_map: HashMap<usize, u32> = HashMap::new();
        let mut pending_skins: Vec<(usize, gltf::Skin<'_>)> = Vec::new();
        while let Some((node, parent)) = stack.pop() {
            let index = u32::try_from(model.nodes.len()).map_err(|_| err("too many nodes".into()))?;
            node_map.insert(node.index(), index);
            let (translation, rotation, scale) = node.transform().decomposed();
            model.nodes.push(Node {
                name: node.name().unwrap_or_default().to_owned(),
                parent,
                translation,
                rotation,
                scale,
                mesh: None,
                lod: LodPolicy::Simplify,
                animated: animated.contains(&node.index()),
            });
            if let Some(mesh) = node.mesh() {
                let skinned = node.skin().is_some();
                let names = target_names(&mesh);
                let default_weights = mesh.weights().unwrap_or_default();
                // One child node per primitive: a node carries one mesh.
                for (p, primitive) in mesh.primitives().enumerate() {
                    if primitive.mode() != Mode::Triangles {
                        continue;
                    }
                    let reader = primitive.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
                    let Some(positions) = reader.read_positions() else { continue };
                    let positions: Vec<[f32; 3]> = positions.collect();
                    let count = u32::try_from(positions.len()).map_err(|_| err("too many vertices".into()))?;
                    let indices = reader.read_indices().map_or_else(|| (0..count).collect(), |i| i.into_u32().collect());
                    let material = primitive
                        .material()
                        .index()
                        .map_or(Ok(default_material), u32::try_from)
                        .map_err(|_| err("material index".into()))?;
                    let data = MeshData {
                        normals: reader.read_normals().map(Iterator::collect),
                        uvs: reader.read_tex_coords(0).map(|t| t.into_f32().collect()),
                        colors: reader.read_colors(0).map(|c| c.into_rgba_f32().collect()),
                        skinned,
                        skin: skinned.then(|| MeshSkin { joints: Vec::new(), influences: influences(&reader, positions.len()) }),
                        morph_targets: morph_targets(&reader, positions.len(), &names, default_weights),
                        indices,
                        material,
                        positions,
                    };
                    if let Some(skin) = node.skin() {
                        pending_skins.push((model.meshes.len(), skin));
                    }
                    let mesh_index = u32::try_from(model.meshes.len()).map_err(|_| err("too many meshes".into()))?;
                    model.meshes.push(data);
                    let mut child = Node::new(format!("{}#{p}", mesh.name().unwrap_or("mesh")));
                    child.parent = Some(index);
                    child.mesh = Some(mesh_index);
                    model.nodes.push(child);
                }
            }
            for child in node.children().collect::<Vec<_>>().into_iter().rev() {
                stack.push((child, Some(index)));
            }
        }

        for (mesh_index, skin) in pending_skins {
            let joints = skin_joints(&skin, &buffers, &node_map).map_err(err)?;
            if let Some(mesh_skin) = model.meshes[mesh_index].skin.as_mut() {
                mesh_skin.joints = joints;
            }
        }
        Ok(model)
    }
}

/// A skin's palette: joint nodes (as model node indices) and inverse binds.
fn skin_joints(skin: &gltf::Skin<'_>, buffers: &[Vec<u8>], node_map: &HashMap<usize, u32>) -> Result<Vec<SkinJoint>, String> {
    let reader = skin.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
    let inverse_binds: Vec<[[f32; 4]; 4]> = reader.read_inverse_bind_matrices().map(Iterator::collect).unwrap_or_default();
    let mut joints = Vec::new();
    for (i, joint) in skin.joints().enumerate() {
        let node = *node_map
            .get(&joint.index())
            .ok_or_else(|| format!("skin {} joint {} is not in the scene", skin.index(), joint.index()))?;
        // Absent inverse binds are identity (glTF 2.0, "Skins").
        let m = inverse_binds.get(i).copied().unwrap_or([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);
        let mut inverse_bind = [0.0; 16];
        inverse_bind.copy_from_slice(m.as_flattened());
        joints.push(SkinJoint { node, inverse_bind });
    }
    Ok(joints)
}

/// Every `JOINTS_n`/`WEIGHTS_n` pair as `(joint, weight)` lists; zero weights dropped.
fn influences<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>, count: usize) -> Vec<Vec<(u16, f32)>>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    let mut out = vec![Vec::new(); count];
    for set in 0.. {
        let (Some(joints), Some(weights)) = (reader.read_joints(set), reader.read_weights(set)) else { break };
        for ((v, j), w) in out.iter_mut().zip(joints.into_u16()).zip(weights.into_f32()) {
            v.extend(j.into_iter().zip(w).filter(|(_, w)| *w > 0.0));
        }
    }
    out
}

/// Position and normal displacements of every target, dense.
fn morph_targets<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>, count: usize, names: &[String], defaults: &[f32]) -> Vec<MorphTarget>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    reader
        .read_morph_targets()
        .enumerate()
        .map(|(i, (positions, normals, _tangents))| MorphTarget {
            name: names.get(i).cloned().unwrap_or_default(),
            positions: positions.map_or_else(|| vec![[0.0; 3]; count], Iterator::collect),
            normals: normals.map(Iterator::collect),
            default_weight: defaults.get(i).copied().unwrap_or(0.0),
            // glTF can't say whether a target animates: runtime by default.
            kind: crate::MorphKind::Expression,
        })
        .collect()
}

/// `mesh.extras.targetNames`, the de facto convention for morph target names.
fn target_names(mesh: &gltf::Mesh<'_>) -> Vec<String> {
    let Some(raw) = mesh.extras() else { return Vec::new() };
    let Ok(value) = gltf::json::deserialize::from_str::<gltf::json::Value>(raw.get()) else { return Vec::new() };
    value
        .get("targetNames")
        .and_then(|n| n.as_array())
        .map(|names| names.iter().map(|n| n.as_str().unwrap_or_default().to_owned()).collect())
        .unwrap_or_default()
}

fn material(model_path: &str, m: &gltf::Material<'_>) -> Material {
    let pbr = m.pbr_metallic_roughness();
    let base_color_texture = pbr.base_color_texture().and_then(|info| match info.texture().source().source() {
        ImageSource::Uri { uri, .. } if !uri.starts_with("data:") => Some(resolve_uri(model_path, uri)),
        // Embedded images would need decoding here; HD packs ship textures
        // as files the VFS (and texture overrides) can see.
        _ => None,
    });
    let mut extras = std::collections::BTreeMap::new();
    extras.insert("metallic".to_owned(), pbr.metallic_factor().to_string());
    extras.insert("roughness".to_owned(), pbr.roughness_factor().to_string());
    Material {
        name: m.name().unwrap_or_default().to_owned(),
        base_color_texture,
        base_color: pbr.base_color_factor(),
        emissive: m.emissive_factor(),
        alpha: match m.alpha_mode() {
            gltf::material::AlphaMode::Opaque => AlphaMode::Opaque,
            gltf::material::AlphaMode::Mask => AlphaMode::Mask(m.alpha_cutoff().unwrap_or(0.5)),
            gltf::material::AlphaMode::Blend => AlphaMode::Blend,
        },
        double_sided: m.double_sided(),
        unlit: m.unlit(),
        extras,
    }
}

/// A texture URI relative to the model file, as a normalized VFS path.
fn resolve_uri(model_path: &str, uri: &str) -> String {
    let uri = uri.replace("%20", " ");
    let dir = model_path.rsplit_once('/').map_or("", |(d, _)| d);
    let mut parts: Vec<&str> = if dir.is_empty() { Vec::new() } else { dir.split('/').collect() };
    for segment in uri.split('/') {
        match segment {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    parts.join("/").to_ascii_lowercase()
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // Imported values must match the file exactly.
mod tests {
    use super::*;

    /// A minimal GLB: one node with a triangle and one material.
    fn triangle_glb() -> Vec<u8> {
        let mut bin = Vec::new();
        for v in [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
            bin.extend(v.to_le_bytes());
        }
        for i in [0u16, 1, 2, 0] {
            // Last index pads the view to 4-byte alignment.
            bin.extend(i.to_le_bytes());
        }
        let json = r#"{
            "asset": {"version": "2.0"},
            "scene": 0,
            "scenes": [{"nodes": [0]}],
            "nodes": [{"name": "root", "translation": [1, 2, 3], "mesh": 0}],
            "meshes": [{"name": "tri", "primitives": [{"attributes": {"POSITION": 0}, "indices": 1, "material": 0}]}],
            "materials": [{"name": "stone", "alphaMode": "MASK", "alphaCutoff": 0.3, "doubleSided": true,
                "pbrMetallicRoughness": {"baseColorFactor": [1, 0.5, 0.5, 1], "baseColorTexture": {"index": 0}}}],
            "textures": [{"source": 0}],
            "images": [{"uri": "../textures/Stone%20Wall.png"}],
            "buffers": [{"byteLength": 44}],
            "bufferViews": [{"buffer": 0, "byteOffset": 0, "byteLength": 36}, {"buffer": 0, "byteOffset": 36, "byteLength": 6}],
            "accessors": [
                {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0, 0, 0], "max": [1, 1, 0]},
                {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"}
            ]
        }"#;
        glb(json, bin)
    }

    /// Packs a JSON chunk and a BIN chunk into a GLB.
    fn glb(json: &str, bin: Vec<u8>) -> Vec<u8> {
        let mut json = json.as_bytes().to_vec();
        while !json.len().is_multiple_of(4) {
            json.push(b' ');
        }
        let total = 12 + 8 + json.len() + 8 + bin.len();
        let mut out = Vec::new();
        out.extend(b"glTF");
        out.extend(2u32.to_le_bytes());
        out.extend(u32::try_from(total).unwrap().to_le_bytes());
        out.extend(u32::try_from(json.len()).unwrap().to_le_bytes());
        out.extend(b"JSON");
        out.extend(json);
        out.extend(u32::try_from(bin.len()).unwrap().to_le_bytes());
        out.extend(b"BIN\0");
        out.extend(bin);
        out
    }

    #[test]
    fn imports_nodes_meshes_and_materials() {
        let model = GltfImporter.import("meshes/props/tri.glb", &triangle_glb()).unwrap();
        assert_eq!(model.nodes.len(), 2, "root + one primitive child");
        assert_eq!(model.nodes[0].translation, [1.0, 2.0, 3.0]);
        assert_eq!(model.nodes[1].parent, Some(0));
        let mesh = &model.meshes[0];
        assert_eq!(mesh.indices, [0, 1, 2]);
        assert_eq!(mesh.positions[1], [1.0, 0.0, 0.0]);
        let mat = &model.materials[0];
        assert_eq!(mat.name, "stone");
        assert_eq!(mat.alpha, AlphaMode::Mask(0.3));
        assert!(mat.double_sided);
        assert_eq!(mat.base_color_texture.as_deref(), Some("meshes/textures/stone wall.png"));
        // The imported model cooks.
        assert!(matches!(crate::cook(&model, &crate::CookSettings::default()).unwrap(), crate::Cooked::Static(_)));
    }

    /// A two-joint skinned triangle with a sparse morph target.
    fn skinned_glb() -> Vec<u8> {
        let mut bin = Vec::new();
        let f32s = |bin: &mut Vec<u8>, values: &[f32]| bin.extend(values.iter().flat_map(|v| v.to_le_bytes()));
        f32s(&mut bin, &[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 2.0, 0.0]); // 0: positions
        bin.extend([0u8, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0]); // 36: JOINTS_0 (u8)
        f32s(&mut bin, &[1.0, 0.0, 0.0, 0.0, 0.25, 0.75, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]); // 48: WEIGHTS_0
        bin.extend([0u16, 1, 2, 0].iter().flat_map(|i| i.to_le_bytes())); // 96: indices (+ pad)
        for y in [-1.0, -2.0] {
            // 104: inverse binds, column-major: translation (0, y, 0).
            f32s(&mut bin, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, y, 0.0, 1.0]);
        }
        bin.extend([2u16, 0].iter().flat_map(|i| i.to_le_bytes())); // 232: sparse index (+ pad)
        f32s(&mut bin, &[0.0, 0.0, 0.5]); // 236: sparse value
        let json = r#"{
            "asset": {"version": "2.0"},
            "scene": 0,
            "scenes": [{"nodes": [0]}],
            "nodes": [
                {"name": "Armature", "children": [1, 3]},
                {"name": "Bone", "translation": [0, 1, 0], "children": [2]},
                {"name": "Tip", "translation": [0, 1, 0]},
                {"name": "Body", "mesh": 0, "skin": 0}
            ],
            "skins": [{"joints": [1, 2], "inverseBindMatrices": 4}],
            "meshes": [{"name": "body", "weights": [0.5], "extras": {"targetNames": ["Smile"]},
                "primitives": [{"attributes": {"POSITION": 0, "JOINTS_0": 1, "WEIGHTS_0": 2}, "indices": 3,
                    "targets": [{"POSITION": 5}]}]}],
            "buffers": [{"byteLength": 248}],
            "bufferViews": [
                {"buffer": 0, "byteOffset": 0, "byteLength": 36},
                {"buffer": 0, "byteOffset": 36, "byteLength": 12},
                {"buffer": 0, "byteOffset": 48, "byteLength": 48},
                {"buffer": 0, "byteOffset": 96, "byteLength": 6},
                {"buffer": 0, "byteOffset": 104, "byteLength": 128},
                {"buffer": 0, "byteOffset": 232, "byteLength": 2},
                {"buffer": 0, "byteOffset": 236, "byteLength": 12}
            ],
            "accessors": [
                {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0, 0, 0], "max": [1, 2, 0]},
                {"bufferView": 1, "componentType": 5121, "count": 3, "type": "VEC4"},
                {"bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC4"},
                {"bufferView": 3, "componentType": 5123, "count": 3, "type": "SCALAR"},
                {"bufferView": 4, "componentType": 5126, "count": 2, "type": "MAT4"},
                {"componentType": 5126, "count": 3, "type": "VEC3", "min": [0, 0, 0], "max": [0, 0, 0.5],
                    "sparse": {"count": 1, "indices": {"bufferView": 5, "componentType": 5123}, "values": {"bufferView": 6}}}
            ]
        }"#;
        glb(json, bin)
    }

    #[test]
    fn imports_skins_and_sparse_morph_targets() {
        let model = GltfImporter.import("meshes/actors/body.glb", &skinned_glb()).unwrap();
        let names: Vec<&str> = model.nodes.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(names, ["Armature", "Bone", "Tip", "Body", "body#0"]);
        let mesh = &model.meshes[0];
        assert!(mesh.skinned);
        let skin = mesh.skin.as_ref().unwrap();
        assert_eq!(skin.joints.iter().map(|j| j.node).collect::<Vec<_>>(), [1, 2]);
        assert_eq!(skin.joints[1].inverse_bind[13], -2.0, "column-major translation");
        assert_eq!(skin.influences, [vec![(0, 1.0)], vec![(0, 0.25), (1, 0.75)], vec![(1, 1.0)]]);
        let target = &mesh.morph_targets[0];
        assert_eq!((target.name.as_str(), target.default_weight), ("Smile", 0.5));
        assert_eq!(target.positions, [[0.0; 3], [0.0; 3], [0.0, 0.0, 0.5]], "sparse expands to dense");

        // It cooks to a skinned model whose joints are the skin's nodes.
        let crate::Cooked::Skinned(cooked) = crate::cook(&model, &crate::CookSettings::default()).unwrap() else {
            panic!("expected a skinned cook");
        };
        assert_eq!(cooked.joints.iter().map(|j| j.name.as_str()).collect::<Vec<_>>(), ["Bone", "Tip"]);
        assert_eq!(cooked.morph_targets[0].name, "Smile");
        let (vertices, _) = cooked.model.lods[0].submeshes[0].decode().unwrap();
        let (skin, morphs) = cooked.decode_streams(0, 0).unwrap();
        let tip = vertices.iter().position(|v| v.position == [0.0, 2.0, 0.0]).unwrap();
        assert_eq!((skin[tip].joints[0], skin[tip].weights), (1, [1.0, 0.0, 0.0, 0.0]));
        assert_eq!(morphs[tip].position, [0.0, 0.0, 0.5]);
        let mid = vertices.iter().position(|v| v.position == [1.0, 1.0, 0.0]).unwrap();
        assert_eq!((skin[mid].joints, skin[mid].weights), ([1, 0, 0, 0], [0.75, 0.25, 0.0, 0.0]));
    }

    #[test]
    fn rejects_external_buffers() {
        let json = br#"{"asset":{"version":"2.0"},"buffers":[{"byteLength":4,"uri":"data.bin"}]}"#;
        let err = GltfImporter.import("a.gltf", json).unwrap_err();
        assert!(err.message.contains("external buffer"), "{err}");
    }

    #[test]
    fn resolves_relative_uris() {
        assert_eq!(resolve_uri("a/b/c.glb", "./t.png"), "a/b/t.png");
        assert_eq!(resolve_uri("a/b/c.glb", "../../T.png"), "t.png");
        assert_eq!(resolve_uri("c.glb", "x/y.png"), "x/y.png");
    }
}
