use bevy_asset::io::Reader;
use bevy_asset::{Asset, AssetLoader, Handle, LoadContext, RenderAssetUsages};
use bevy_mesh::{
    Indices, Mesh, MeshAttributeCompressionFlags, MeshCompressionArgs, PrimitiveTopology,
};
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use sf_model::{Aabb, Cooked, CookedModel, CookedSkinned, CookedSubmesh, FormatError, Material};

use crate::skinned::{CookedSkin, inverse_bindposes, skinned_submesh_to_mesh};

/// A cooked model's meshes, grouped by LOD.
#[derive(Asset, TypePath, Debug)]
pub struct CookedModelAsset {
    /// Every mesh of every level; [`CookedLodMeshes::parts`] index into it.
    #[dependency]
    pub meshes: Vec<Handle<Mesh>>,
    /// LOD0 first.
    pub lods: Vec<CookedLodMeshes>,
    pub materials: Vec<Material>,
    pub bounds: Aabb,
    /// Set when the bake didn't cook this model (animated or skinned): the
    /// caller falls back to the source importer.
    pub skipped: Option<String>,
    /// Ray-tracing variants of one level, when
    /// [`SfMeshLoaderSettings::ray_tracing_lod`] asked for them (static
    /// models only: Solari doesn't trace skinned meshes).
    pub ray_tracing: Option<CookedRayTracing>,
    /// Set for a deforming model (`Cooked::Skinned`): the joint palette,
    /// morph targets and per-mesh target lists. Spawn with
    /// [`crate::spawn_skinned`].
    pub skin: Option<CookedSkin>,
}

/// Meshes in the layout a ray tracer takes (Bevy Solari: uncompressed
/// position/normal/uv0/tangent, u32 indices, nothing else), built at load
/// time from one LOD level. Ray tracers typically use a reduced level.
#[derive(Debug, Clone)]
pub struct CookedRayTracing {
    /// The level these were built from.
    pub level: usize,
    /// One per part of that level, in [`CookedLodMeshes::parts`] order.
    pub meshes: Vec<Handle<Mesh>>,
}

#[derive(Debug, Clone)]
pub struct CookedLodMeshes {
    /// Worst deviation from LOD0 in model-space meters (for LOD distances).
    pub error: f32,
    /// `(index into CookedModelAsset::meshes, material index)`.
    pub parts: Vec<(usize, u32)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfMeshLoaderSettings {
    /// Apply Bevy's vertex compression (`MeshCompressionArgs::regular`).
    pub compress: bool,
    /// Keep a CPU copy of the vertex data (off: GPU only, less memory).
    pub keep_cpu_copy: bool,
    /// Also build ray-tracing variants ([`CookedRayTracing`]) from this LOD
    /// level, clamped to the last one. `None` (default): no variants.
    pub ray_tracing_lod: Option<usize>,
}

impl Default for SfMeshLoaderSettings {
    fn default() -> Self {
        Self {
            compress: true,
            keep_cpu_copy: false,
            ray_tracing_lod: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SfMeshLoadError {
    #[error("reading .sfmesh: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] FormatError),
    /// The model exceeds a Bevy renderer limit (joints, morph targets).
    #[error("model exceeds a Bevy limit: {0}")]
    Limit(String),
}

/// Loads `.sfmesh` files.
#[derive(Default, TypePath)]
pub struct SfMeshLoader;

impl AssetLoader for SfMeshLoader {
    type Asset = CookedModelAsset;
    type Settings = SfMeshLoaderSettings;
    type Error = SfMeshLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let model = match Cooked::from_bytes(&bytes)? {
            Cooked::Static(model) => model,
            Cooked::Skinned(skinned) => return load_skinned(&skinned, settings, load_context),
            Cooked::Skipped { reason } => {
                return Ok(CookedModelAsset {
                    meshes: Vec::new(),
                    lods: Vec::new(),
                    materials: Vec::new(),
                    bounds: Aabb::EMPTY,
                    skipped: Some(reason),
                    ray_tracing: None,
                    skin: None,
                });
            }
        };
        static_model_asset(model, settings, "", load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["sfmesh"]
    }
}

/// A static cooked model's meshes as labeled assets of the current load
/// (`<label>lod<k>/<i>`, ray-tracing variants `<label>lod<k>/<i>/rt`). The
/// `.sfmesh` loader passes an empty label; products holding several models
/// (precombined cells, precombine.md) prefix each one.
pub fn static_model_asset(
    model: CookedModel,
    settings: &SfMeshLoaderSettings,
    label: &str,
    load_context: &mut LoadContext<'_>,
) -> Result<CookedModelAsset, SfMeshLoadError> {
    let usage = if settings.keep_cpu_copy {
        RenderAssetUsages::all()
    } else {
        RenderAssetUsages::RENDER_WORLD
    };
    let mut meshes = Vec::new();
    let mut lods = Vec::with_capacity(model.lods.len());
    for (level, lod) in model.lods.iter().enumerate() {
        let mut parts = Vec::with_capacity(lod.submeshes.len());
        for (i, sub) in lod.submeshes.iter().enumerate() {
            let mesh = submesh_to_mesh(&model, sub, usage, settings.compress)?;
            parts.push((meshes.len(), sub.material));
            meshes.push(load_context.add_labeled_asset(format!("{label}lod{level}/{i}"), mesh));
        }
        lods.push(CookedLodMeshes {
            error: lod.error,
            parts,
        });
    }
    let ray_tracing = match settings.ray_tracing_lod {
        Some(wanted) if !model.lods.is_empty() => {
            let level = wanted.min(model.lods.len() - 1);
            let mut rt = Vec::with_capacity(model.lods[level].submeshes.len());
            for (i, sub) in model.lods[level].submeshes.iter().enumerate() {
                let mesh = submesh_to_ray_tracing_mesh(&model, sub)?;
                rt.push(load_context.add_labeled_asset(format!("{label}lod{level}/{i}/rt"), mesh));
            }
            Some(CookedRayTracing { level, meshes: rt })
        }
        _ => None,
    };
    Ok(CookedModelAsset {
        meshes,
        lods,
        materials: model.materials,
        bounds: model.bounds,
        skipped: None,
        ray_tracing,
        skin: None,
    })
}

/// The `Cooked::Skinned` branch of [`SfMeshLoader`]: one mesh per LOD and
/// submesh (labeled `lod<k>/<i>` as for static models) and one shared
/// `SkinnedMeshInverseBindposes` (labeled `inverse_bindposes`).
fn load_skinned(
    skinned: &CookedSkinned,
    settings: &SfMeshLoaderSettings,
    load_context: &mut LoadContext<'_>,
) -> Result<CookedModelAsset, SfMeshLoadError> {
    let inverse_bindposes =
        load_context.add_labeled_asset("inverse_bindposes".to_owned(), inverse_bindposes(skinned)?);
    let usage = if settings.keep_cpu_copy {
        RenderAssetUsages::all()
    } else {
        RenderAssetUsages::RENDER_WORLD
    };
    let mut meshes = Vec::new();
    let mut infos = Vec::new();
    let mut lods = Vec::with_capacity(skinned.model.lods.len());
    for (level, lod) in skinned.model.lods.iter().enumerate() {
        let mut parts = Vec::with_capacity(lod.submeshes.len());
        for (i, sub) in lod.submeshes.iter().enumerate() {
            let (mesh, info) =
                skinned_submesh_to_mesh(skinned, level, i, usage, settings.compress)?;
            parts.push((meshes.len(), sub.material));
            meshes.push(load_context.add_labeled_asset(format!("lod{level}/{i}"), mesh));
            infos.push(info);
        }
        lods.push(CookedLodMeshes {
            error: lod.error,
            parts,
        });
    }
    Ok(CookedModelAsset {
        meshes,
        lods,
        materials: skinned.model.materials.clone(),
        bounds: skinned.model.bounds,
        skipped: None,
        ray_tracing: None,
        skin: Some(CookedSkin {
            inverse_bindposes,
            joints: skinned.joints.iter().map(|j| j.name.clone()).collect(),
            morph_targets: skinned.morph_targets.clone(),
            parts: skinned.parts.clone(),
            meshes: infos,
        }),
    })
}

/// Decodes one cooked submesh into a Bevy triangle-list `Mesh`.
pub fn submesh_to_mesh(
    model: &CookedModel,
    sub: &CookedSubmesh,
    usage: RenderAssetUsages,
    compress: bool,
) -> Result<Mesh, FormatError> {
    let (vertices, indices) = sub.decode()?;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, usage);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|v| v.position).collect::<Vec<_>>(),
    );
    if model.has_uvs {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vertices.iter().map(|v| v.uv).collect::<Vec<_>>(),
        );
    }
    if model.has_colors {
        let colors: Vec<[f32; 4]> = vertices
            .iter()
            .map(|v| v.color.map(|c| f32::from(c) / 255.0))
            .collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    mesh.insert_indices(Indices::U32(indices));
    if model.has_normals {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vertices.iter().map(|v| v.normal).collect::<Vec<_>>(),
        );
    } else {
        // Lit materials need normals; source without them gets smooth ones.
        mesh.compute_smooth_normals();
    }
    if compress {
        // A failed attribute leaves that attribute uncompressed; the mesh
        // stays valid either way.
        mesh = mesh
            .compressed_mesh(&compression_args(usage))
            .unwrap_or_else(|(mesh, _)| mesh);
    }
    Ok(mesh)
}

/// Decodes one cooked submesh into the layout Bevy Solari ray traces:
/// uncompressed position, normal, uv0 and tangent, u32 indices, no other
/// attributes (vertex colors are dropped), GPU only.
pub fn submesh_to_ray_tracing_mesh(
    model: &CookedModel,
    sub: &CookedSubmesh,
) -> Result<Mesh, FormatError> {
    let (vertices, indices) = sub.decode()?;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|v| v.position).collect::<Vec<_>>(),
    );
    let uvs: Vec<[f32; 2]> = if model.has_uvs {
        vertices.iter().map(|v| v.uv).collect()
    } else {
        vec![[0.0; 2]; vertices.len()]
    };
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    if model.has_normals {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vertices.iter().map(|v| v.normal).collect::<Vec<_>>(),
        );
    } else {
        mesh.compute_smooth_normals();
    }
    if mesh.generate_tangents().is_err() {
        // Degenerate UVs: any unit tangent perpendicular to the normal will
        // do, since such surfaces can't carry a usable normal map.
        let normals: Vec<[f32; 4]> = match mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
            Some(bevy_mesh::VertexAttributeValues::Float32x3(normals)) => normals
                .iter()
                .map(|&[x, y, z]| orthogonal_tangent([x, y, z]))
                .collect(),
            _ => vec![[1.0, 0.0, 0.0, 1.0]; vertices.len()],
        };
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, normals);
    }
    Ok(mesh)
}

fn orthogonal_tangent([x, y, z]: [f32; 3]) -> [f32; 4] {
    // Cross the normal with the axis it is least aligned with.
    let (tx, ty, tz) = if x.abs() < 0.9 {
        (0.0, z, -y)
    } else {
        (-z, 0.0, x)
    };
    let len = (tx * tx + ty * ty + tz * tz).sqrt().max(f32::EPSILON);
    [tx / len, ty / len, tz / len, 1.0]
}

/// Bevy's regular vertex compression, minus positions for GPU-only meshes.
///
/// Scaffold for a Bevy 0.20 bug: `Mesh::take_gpu_data` (run when a mesh
/// drops its CPU copy) recomputes the AABB with `compute_aabb`, which only
/// reads `Float32x3` positions. Compressed (`Snorm16x4`) positions are
/// decoded against that AABB, so GPU-only meshes with compressed positions
/// render collapsed. Removal: once `take_gpu_data` keeps the AABB computed
/// at compression time, use `MeshCompressionArgs::regular()` for every mesh.
/// Test: `tests::gpu_only_meshes_keep_their_bounds`.
pub fn compression_args(usage: RenderAssetUsages) -> MeshCompressionArgs {
    let mut args = MeshCompressionArgs::regular();
    if !usage.contains(RenderAssetUsages::MAIN_WORLD) {
        args.compress_attributes
            .remove(MeshAttributeCompressionFlags::COMPRESS_POSITION);
    }
    args
}
