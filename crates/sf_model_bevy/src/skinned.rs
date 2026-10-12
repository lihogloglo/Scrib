//! `Cooked::Skinned` → Bevy: the mapping in bake.md ("Morph layout" →
//! "Bevy mapping"). One `Mesh` per submesh: base attributes, joint indices
//! and weights, and the submesh's own expression targets as Bevy morph
//! targets (shape targets baked at their default weight). Bevy's own
//! skinning and morph runtime does the rest, as for `bevy_gltf` meshes.

use bevy_asset::RenderAssetUsages;
use bevy_math::{Mat4, Vec3};
use bevy_mesh::morph::{MAX_MORPH_WEIGHTS, MorphAttributes};
use bevy_mesh::skinning::SkinnedMeshInverseBindposes;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};
use sf_model::{CookedPart, CookedSkinned, DecodedSubmesh, MorphTargetInfo};

use crate::loader::{SfMeshLoadError, compression_args};

/// Joints per `SkinnedMesh` Bevy 0.20 supports (`bevy_pbr::MAX_JOINTS`; the
/// skin uniform/storage layout is sized for it).
pub const MAX_JOINTS: usize = 256;

/// Floats per morph target layer Bevy's texture fallback holds
/// (`MAX_TEXTURE_WIDTH²`).
const MAX_MORPH_TEXELS: usize = (bevy_mesh::morph::MAX_TEXTURE_WIDTH as usize).pow(2);

/// The skin side of a [`crate::CookedModelAsset`] loaded from a
/// `Cooked::Skinned` file.
#[derive(Debug, Clone)]
pub struct CookedSkin {
    /// The model-wide palette's inverse binds, shared by every mesh.
    pub inverse_bindposes: bevy_asset::Handle<SkinnedMeshInverseBindposes>,
    /// Palette joint names, in `SkinnedMesh::joints` order. A name may repeat
    /// (one joint bound in two poses).
    pub joints: Vec<String>,
    /// The model's morph targets. Shape targets are baked into the meshes at
    /// their default weight; only expression targets reach the GPU.
    pub morph_targets: Vec<MorphTargetInfo>,
    /// Source meshes (node paths), for body-part filtering.
    pub parts: Vec<CookedPart>,
    /// Parallel to [`crate::CookedModelAsset::meshes`].
    pub meshes: Vec<SkinnedMeshInfo>,
}

/// What a spawned mesh needs beyond its `Mesh` handle.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkinnedMeshInfo {
    /// Index into [`CookedSkin::parts`].
    pub part: u32,
    /// The mesh's Bevy morph targets, as indices into
    /// [`CookedSkin::morph_targets`] (Bevy target `k` is `targets[k]`).
    /// Empty: the mesh has no morph targets.
    pub targets: Vec<u16>,
}

impl CookedSkin {
    /// Default model-wide weights, one per [`Self::morph_targets`].
    pub fn default_weights(&self) -> Vec<f32> {
        self.morph_targets
            .iter()
            .map(|t| t.default_weight)
            .collect()
    }
}

/// One mesh's `MeshMorphWeights::Value` weights, gathered from the
/// model-wide vector through its target list (a missing entry is 0).
pub fn gather_weights(targets: &[u16], model_weights: &[f32]) -> Vec<f32> {
    targets
        .iter()
        .map(|&t| model_weights.get(usize::from(t)).copied().unwrap_or(0.0))
        .collect()
}

/// The palette as Bevy inverse bindposes. Fails above [`MAX_JOINTS`]:
/// Bevy would silently drop the extra joints.
pub fn inverse_bindposes(
    skinned: &CookedSkinned,
) -> Result<SkinnedMeshInverseBindposes, SfMeshLoadError> {
    if skinned.joints.len() > MAX_JOINTS {
        return Err(SfMeshLoadError::Limit(format!(
            "{} joints, Bevy supports {MAX_JOINTS} per skinned mesh",
            skinned.joints.len()
        )));
    }
    Ok(SkinnedMeshInverseBindposes::from(
        skinned
            .joints
            .iter()
            .map(|j| Mat4::from_cols_array(&j.inverse_bind))
            .collect::<Vec<_>>(),
    ))
}

/// Decodes submesh `i` of `level` into a skinned Bevy `Mesh` (and its
/// [`SkinnedMeshInfo`]). Shape targets are baked at their default weight.
/// The mesh carries `SkinnedMeshBounds` (Bevy's per-joint bind-space boxes)
/// for `DynamicSkinnedMeshBounds`, generated before compression because
/// Bevy reads them from `Float32x3` positions.
pub fn skinned_submesh_to_mesh(
    skinned: &CookedSkinned,
    level: usize,
    i: usize,
    usage: RenderAssetUsages,
    compress: bool,
) -> Result<(Mesh, SkinnedMeshInfo), SfMeshLoadError> {
    let mut sub = skinned.decode_submesh(level, i)?;
    sub.bake_shapes(&skinned.morph_targets, |t| {
        skinned.morph_targets[t].default_weight
    });
    if sub.targets.len() > MAX_MORPH_WEIGHTS {
        return Err(SfMeshLoadError::Limit(format!(
            "{} morph targets on one mesh, Bevy supports {MAX_MORPH_WEIGHTS}",
            sub.targets.len()
        )));
    }
    // Without storage buffers Bevy stores each target as one texture layer
    // of `vertices × 9` floats, at most 2048² (`MorphTargetImage::new`).
    let texels = sub.vertices.len() * MorphAttributes::COMPONENT_COUNT;
    if !sub.targets.is_empty() && texels > MAX_MORPH_TEXELS {
        return Err(SfMeshLoadError::Limit(format!(
            "{} morphing vertices on one mesh, Bevy supports {}",
            sub.vertices.len(),
            MAX_MORPH_TEXELS / 9
        )));
    }
    let model = &skinned.model;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, usage);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        sub.vertices.iter().map(|v| v.position).collect::<Vec<_>>(),
    );
    if model.has_uvs {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            sub.vertices.iter().map(|v| v.uv).collect::<Vec<_>>(),
        );
    }
    if model.has_colors {
        let colors: Vec<[f32; 4]> = sub
            .vertices
            .iter()
            .map(|v| v.color.map(|c| f32::from(c) / 255.0))
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_JOINT_INDEX,
        bevy_mesh::VertexAttributeValues::Uint16x4(sub.skin.iter().map(|s| s.joints).collect()),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_JOINT_WEIGHT,
        sub.skin.iter().map(|s| s.weights).collect::<Vec<_>>(),
    );
    mesh.insert_indices(Indices::U32(std::mem::take(&mut sub.indices)));
    if model.has_normals {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            sub.vertices.iter().map(|v| v.normal).collect::<Vec<_>>(),
        );
    } else {
        mesh.compute_smooth_normals();
    }
    if !sub.targets.is_empty() {
        mesh.set_morph_targets(morph_attributes(&sub));
        mesh.set_morph_target_names(
            sub.targets
                .iter()
                .map(|&t| skinned.morph_targets[usize::from(t)].name.clone())
                .collect(),
        );
    }
    // A mesh whose vertices all have zero weight has no bounds to give;
    // the cook never produces one, so this only skips the component.
    let _ = mesh.generate_skinned_mesh_bounds();
    if compress {
        mesh = mesh
            .compressed_mesh(&compression_args(usage))
            .unwrap_or_else(|(mesh, _)| mesh);
    }
    Ok((
        mesh,
        SkinnedMeshInfo {
            part: sub.part,
            targets: sub.targets,
        },
    ))
}

/// Bevy's flat target-major morph layout: block `k`, vertex `v` is
/// `morphs[k·n + v]` (the order `bevy_gltf` builds). No tangent deltas (the
/// cook keeps no tangents).
pub fn morph_attributes(sub: &DecodedSubmesh) -> Vec<MorphAttributes> {
    sub.morphs
        .iter()
        .map(|d| {
            MorphAttributes::new(
                Vec3::from_array(d.position),
                Vec3::from_array(d.normal),
                Vec3::ZERO,
            )
        })
        .collect()
}
