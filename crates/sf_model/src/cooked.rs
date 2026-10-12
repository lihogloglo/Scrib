//! The `.sfmesh` container: a cooked model with its LOD chain.
//!
//! Layout: 8-byte magic, `u32` format version (little-endian), then a
//! postcard body. Geometry inside is meshoptimizer-encoded (vertex and
//! index codecs), which decodes at GB/s and compresses well.
//!
//! Versions: 1 holds [`Cooked::Static`] and [`Cooked::Skipped`]; 2 added
//! [`Cooked::Skinned`] as a new trailing variant, so a v1 body is a valid v2
//! body; 3 changed `Skinned`'s morph layout (per-submesh target lists), so v2
//! is retired (no reader takes it; skinned products re-cook) while v1 still
//! reads. Files are written with the lowest version that holds them (static
//! cooks stay byte-identical to v1).

use serde::{Deserialize, Serialize};

use crate::{Aabb, CookedSkinned, Material};

pub const MAGIC: [u8; 8] = *b"SFMESH\0\0";
/// The newest format version; readers accept [`MIN_FORMAT_VERSION`]..=this.
/// Bump on a layout change; when old bodies no longer decode, raise
/// [`MIN_FORMAT_VERSION`] too.
pub const FORMAT_VERSION: u32 = 3;
/// The oldest format version still read.
pub const MIN_FORMAT_VERSION: u32 = 1;
/// The format part of the `mesh.cook` / `mesh.rest` bake keys. Bump only when
/// cooking the same input writes different bytes (cached products are then
/// stale); a version that only adds new content (v2) leaves it alone, so
/// caches stay valid and no full re-cook is needed. 2: v3's morph layout.
pub const KEY_FORMAT: u32 = 2;
/// Versions in `MIN_FORMAT_VERSION..=FORMAT_VERSION` that are no longer read:
/// 2 (the first skinned layout, dense model-wide morph blocks).
pub const RETIRED_FORMAT_VERSIONS: [u32; 1] = [2];

/// The cooked vertex. Fixed layout, 36 bytes, no padding: the meshopt codec
/// and the vertex remap both work on raw bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    /// sRGB-agnostic 8-bit RGBA (linear values × 255).
    pub color: [u8; 4],
}

/// The result of cooking one source model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Cooked {
    Static(CookedModel),
    /// Not cookable yet (animated nodes without a skin, skinned meshes
    /// without skin data): the runtime imports it.
    Skipped { reason: String },
    /// Deforms at runtime (skins, morph targets): bind-space geometry plus
    /// joint and morph streams. Format version 3.
    Skinned(CookedSkinned),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedModel {
    pub bounds: Aabb,
    /// Which vertex attributes carry real data.
    pub has_normals: bool,
    pub has_uvs: bool,
    pub has_colors: bool,
    pub materials: Vec<Material>,
    /// LOD0 first. Each level is complete (not a delta).
    pub lods: Vec<CookedLod>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedLod {
    /// Worst geometric deviation from LOD0, in model-space meters.
    pub error: f32,
    pub submeshes: Vec<CookedSubmesh>,
}

impl CookedLod {
    pub fn triangle_count(&self) -> u32 {
        self.submeshes.iter().map(|s| s.index_count / 3).sum()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedSubmesh {
    /// Index into [`CookedModel::materials`].
    pub material: u32,
    pub vertex_count: u32,
    pub index_count: u32,
    /// meshopt-encoded `Vertex` buffer.
    pub vertices: Vec<u8>,
    /// meshopt-encoded `u32` index buffer.
    pub indices: Vec<u8>,
}

impl CookedSubmesh {
    pub fn decode(&self) -> Result<(Vec<Vertex>, Vec<u32>), FormatError> {
        let vertices = meshopt::decode_vertex_buffer::<Vertex>(&self.vertices, self.vertex_count as usize)
            .map_err(|e| FormatError::Codec(e.to_string()))?;
        let indices = meshopt::decode_index_buffer::<u32>(&self.indices, self.index_count as usize)
            .map_err(|e| FormatError::Codec(e.to_string()))?;
        if indices.iter().any(|&i| i >= self.vertex_count) {
            return Err(FormatError::Codec("index out of range".into()));
        }
        Ok((vertices, indices))
    }

    pub(crate) fn encode(material: u32, vertices: &[Vertex], indices: &[u32]) -> Result<Self, FormatError> {
        let codec = |e: meshopt::Error| FormatError::Codec(e.to_string());
        Ok(Self {
            material,
            vertex_count: u32::try_from(vertices.len()).map_err(|_| FormatError::TooLarge)?,
            index_count: u32::try_from(indices.len()).map_err(|_| FormatError::TooLarge)?,
            vertices: meshopt::encode_vertex_buffer(vertices).map_err(codec)?,
            indices: meshopt::encode_index_buffer(indices, vertices.len()).map_err(codec)?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("not an .sfmesh file")]
    BadMagic,
    #[error(".sfmesh format version {0}, expected {MIN_FORMAT_VERSION}..={FORMAT_VERSION}")]
    Version(u32),
    #[error("malformed .sfmesh: {0}")]
    Body(String),
    #[error("mesh codec: {0}")]
    Codec(String),
    #[error("mesh too large for 32-bit counts")]
    TooLarge,
}

impl CookedModel {
    /// Wavefront OBJ of every LOD (one object per level and submesh, LODs
    /// side by side along +X), for inspecting cooked output in any DCC tool.
    pub fn to_obj(&self) -> Result<String, FormatError> {
        use std::fmt::Write;
        let width = if self.bounds.is_empty() { 0.0 } else { (self.bounds.max[0] - self.bounds.min[0]) * 1.2 };
        let mut out = String::from("# superfargoth .sfmesh export\n");
        let mut base = 1u64;
        for (level, lod) in self.lods.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let shift = width * level as f32;
            for (i, sub) in lod.submeshes.iter().enumerate() {
                let (vertices, indices) = sub.decode()?;
                let material = self.materials.get(sub.material as usize).map_or("default", |m| m.name.as_str());
                let _ = writeln!(out, "o lod{level}_{i} # error {} m\nusemtl {material}", lod.error);
                for v in &vertices {
                    let [x, y, z] = v.position;
                    let _ = writeln!(out, "v {} {y} {z}", x + shift);
                }
                for v in &vertices {
                    let _ = writeln!(out, "vt {} {}", v.uv[0], 1.0 - v.uv[1]);
                }
                for v in &vertices {
                    let [x, y, z] = v.normal;
                    let _ = writeln!(out, "vn {x} {y} {z}");
                }
                for tri in indices.as_chunks::<3>().0 {
                    let [a, b, c] = tri.map(|k| base + u64::from(k));
                    let _ = writeln!(out, "f {a}/{a}/{a} {b}/{b}/{b} {c}/{c}/{c}");
                }
                base += vertices.len() as u64;
            }
        }
        Ok(out)
    }
}

impl Cooked {
    /// The lowest format version that holds this value (see the module docs).
    pub fn format_version(&self) -> u32 {
        match self {
            Self::Static(_) | Self::Skipped { .. } => 1,
            Self::Skinned(_) => 3,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend_from_slice(&self.format_version().to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("cooked models always serialize"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let rest = bytes.strip_prefix(&MAGIC).ok_or(FormatError::BadMagic)?;
        let (version, body) = rest.split_first_chunk::<4>().ok_or(FormatError::BadMagic)?;
        let version = u32::from_le_bytes(*version);
        if !(MIN_FORMAT_VERSION..=FORMAT_VERSION).contains(&version) || RETIRED_FORMAT_VERSIONS.contains(&version) {
            return Err(FormatError::Version(version));
        }
        let cooked: Self = postcard::from_bytes(body).map_err(|e| FormatError::Body(e.to_string()))?;
        if cooked.format_version() > version {
            return Err(FormatError::Body(format!("content needs format version {}", cooked.format_version())));
        }
        Ok(cooked)
    }
}
