//! `sf_character::AssembledMesh` → Bevy `Mesh`: the same attribute mapping
//! as `sf_model_bevy::skinned_submesh_to_mesh` (bake.md "Bevy mapping"), for
//! a merged outfit mesh instead of one cooked submesh.

use bevy_asset::RenderAssetUsages;
use bevy_math::{Mat4, Vec3};
use bevy_mesh::morph::{MAX_MORPH_WEIGHTS, MorphAttributes};
use bevy_mesh::skinning::SkinnedMeshInverseBindposes;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues};
use sf_character::{AssembledMesh, AssembledOutfit};
use sf_model_bevy::{MAX_JOINTS, compression_args};

/// Why an assembled outfit can't become Bevy meshes.
#[derive(Debug, thiserror::Error)]
pub enum OutfitMeshError {
    /// Bevy would silently drop the extra joints.
    #[error("outfit palette has {0} joints, Bevy supports {MAX_JOINTS} per skinned mesh")]
    TooManyJoints(usize),
    #[error("{0} morph targets on one mesh, Bevy supports {MAX_MORPH_WEIGHTS}")]
    TooManyTargets(usize),
}

/// The outfit palette as Bevy inverse bindposes (one asset shared by every
/// mesh of the outfit).
pub fn outfit_inverse_bindposes(outfit: &AssembledOutfit) -> Result<SkinnedMeshInverseBindposes, OutfitMeshError> {
    if outfit.joints.len() > MAX_JOINTS {
        return Err(OutfitMeshError::TooManyJoints(outfit.joints.len()));
    }
    Ok(SkinnedMeshInverseBindposes::from(outfit.joints.iter().map(|j| Mat4::from_cols_array(&j.inverse_bind)).collect::<Vec<_>>()))
}

/// One merged (or morph-bearing) outfit mesh as a skinned Bevy `Mesh`:
/// positions, normals, UVs, colors when the outfit has them, joint indices
/// (`Uint16x4`, outfit palette) and weights, its own morph targets, and
/// `SkinnedMeshBounds` for `DynamicSkinnedMeshBounds` (generated before
/// compression, as the loader does).
pub fn assembled_mesh_to_mesh(m: &AssembledMesh, has_colors: bool, usage: RenderAssetUsages, compress: bool) -> Result<Mesh, OutfitMeshError> {
    if m.morph_targets.len() > MAX_MORPH_WEIGHTS {
        return Err(OutfitMeshError::TooManyTargets(m.morph_targets.len()));
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, usage);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, m.vertices.iter().map(|v| v.position).collect::<Vec<_>>());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, m.vertices.iter().map(|v| v.normal).collect::<Vec<_>>());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, m.vertices.iter().map(|v| v.uv).collect::<Vec<_>>());
    if has_colors {
        let colors: Vec<[f32; 4]> = m.vertices.iter().map(|v| v.color.map(|c| f32::from(c) / 255.0)).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_INDEX, VertexAttributeValues::Uint16x4(m.skin.iter().map(|s| s.joints).collect()));
    mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, m.skin.iter().map(|s| s.weights).collect::<Vec<_>>());
    mesh.insert_indices(Indices::U32(m.indices.clone()));
    if !m.morph_targets.is_empty() {
        // Target-major, as `bevy_gltf` and the loader build it.
        let attributes =
            m.morphs.iter().map(|d| MorphAttributes::new(Vec3::from_array(d.position), Vec3::from_array(d.normal), Vec3::ZERO)).collect();
        mesh.set_morph_targets(attributes);
        mesh.set_morph_target_names(m.morph_targets.iter().map(|t| t.name.clone()).collect());
    }
    // Every assembled vertex is weighted (the cook guarantees it); a mesh
    // without bounds only skips the component.
    let _ = mesh.generate_skinned_mesh_bounds();
    if compress {
        mesh = mesh.compressed_mesh(&compression_args(usage)).unwrap_or_else(|(mesh, _)| mesh);
    }
    Ok(mesh)
}
