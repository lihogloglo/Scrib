//! Cluster proxies: merge many meshes into node space, weld, simplify to an
//! absolute error budget with meshoptimizer, optimize, encode as a one-level
//! `.sfmesh` model (far-field.md, "Bake steps").

use glam::{Affine3A, Mat3, Vec3};
use meshopt::{SimplifyOptions, VertexDataAdapter};
use sf_model::{Aabb, AlphaMode, CookedLod, CookedModel, CookedSubmesh, Material, Vertex};

use crate::FormatError;

/// Geometry gathered for one proxy, already in node space.
#[derive(Clone, Debug, Default)]
pub struct ClusterBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    /// The largest error of any source added (meters, node scale).
    source_error: f32,
}

/// Simplifier tuning (part of the bake key through `FarBakeSettings`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimplifySettings {
    /// Rebuilt normals are smoothed across edges whose faces are within
    /// this angle (cosine; 0.77 ≈ 40°) and kept hard beyond it.
    pub crease_cos: f32,
    /// Cost of a full color change, as a fraction of the error budget.
    pub color_weight: f32,
    /// Remove disconnected parts smaller than the error (meshopt `Prune`).
    pub prune: bool,
}

impl Default for SimplifySettings {
    fn default() -> Self {
        Self { crease_cos: 0.77, color_weight: 0.5, prune: true }
    }
}

/// A finished proxy mesh and its measured error.
#[derive(Clone, Debug, PartialEq)]
pub struct ClusterMesh {
    pub model: CookedModel,
    /// Source error plus simplification error (an upper bound), meters.
    pub error: f32,
    pub triangles: u32,
}

impl ClusterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Adds triangles transformed by `to_node`. `albedo` multiplies the
    /// vertex colors (material base color × texture average); the result
    /// is the proxy's vertex-color albedo. `error` is the source's own
    /// deviation (already scaled to meters).
    pub fn add(&mut self, vertices: &[Vertex], indices: &[u32], to_node: Affine3A, albedo: [f32; 4], error: f32) {
        let base = u32::try_from(self.vertices.len()).expect("proxy vertex count fits u32");
        let normal_matrix = Mat3::from(to_node.matrix3).inverse().transpose();
        let mirrored = to_node.matrix3.determinant() < 0.0;
        for v in vertices {
            let position = to_node.transform_point3(Vec3::from_array(v.position)).to_array();
            let normal = (normal_matrix * Vec3::from_array(v.normal)).normalize_or_zero().to_array();
            let color = [0, 1, 2, 3].map(|c| unorm8(f32::from(v.color[c]) / 255.0 * albedo[c]));
            self.vertices.push(Vertex { position, normal, uv: [0.0; 2], color });
        }
        for t in indices.as_chunks::<3>().0 {
            let [a, b, c] = t.map(|i| i + base);
            self.indices.extend(if mirrored { [a, c, b] } else { [a, b, c] });
        }
        self.source_error = self.source_error.max(error);
    }

    /// Welds, simplifies to `budget` (total error, source error included),
    /// optimizes and encodes. `None` when nothing was added.
    pub fn build(self, budget: f32, settings: &SimplifySettings) -> Result<Option<ClusterMesh>, FormatError> {
        if self.indices.is_empty() {
            return Ok(None);
        }
        // Topology: weld by position and color only. Hard-edged (faceted)
        // sources duplicate vertices per face normal, which would leave
        // every face a separate component the simplifier can't collapse;
        // normals are rebuilt after simplification (crease angle), as is
        // usual for HLOD proxies.
        let keyed: Vec<Vertex> = self.vertices.iter().map(|v| Vertex { normal: [0.0; 3], uv: [0.0; 2], ..*v }).collect();
        let (count, remap) = meshopt::generate_vertex_remap(&keyed, Some(&self.indices));
        let mut indices = meshopt::remap_index_buffer(Some(&self.indices), count, &remap);
        let vertices = meshopt::remap_vertex_buffer(&keyed, count, &remap);

        let target = (budget - self.source_error).max(0.0);
        let mut simplify_error = 0.0f32;
        if target > 0.0 {
            let adapter = VertexDataAdapter::new(meshopt::typed_to_bytes(&vertices), size_of::<Vertex>(), 0)
                .map_err(|e| FormatError::Mesh(e.to_string()))?;
            let attributes: Vec<f32> = vertices
                .iter()
                .flat_map(|v| {
                    let [r, g, b, _] = v.color.map(|c| f32::from(c) / 255.0);
                    [r, g, b]
                })
                .collect();
            // meshopt adds weighted attribute error to the geometric error
            // (meters here): a full 0→1 color change costs `color_weight` ×
            // the budget, so color seams shape the result without blocking it.
            let w = [settings.color_weight * target; 3];
            let mut options = SimplifyOptions::ErrorAbsolute;
            if settings.prune {
                options |= SimplifyOptions::Prune;
            }
            let locks = vec![false; vertices.len()];
            indices = meshopt::simplify_with_attributes_and_locks(
                &indices,
                &adapter,
                &attributes,
                &w,
                3 * size_of::<f32>(),
                &locks,
                3,
                target,
                options,
                Some(&mut simplify_error),
            );
            // Measured on Morrowind (2026-10-11): the result is error-bound
            // (it reaches the budget), and a vertex-clustering pass
            // (`simplify_sloppy`) on the remainder never paid off, so there
            // is none.
        }
        let (vertices, mut indices) = crease_normals(&vertices, &indices, settings.crease_cos);
        if indices.is_empty() {
            return Ok(None);
        }
        indices = meshopt::optimize_vertex_cache(&indices, vertices.len());
        let adapter = VertexDataAdapter::new(meshopt::typed_to_bytes(&vertices), size_of::<Vertex>(), 0)
            .map_err(|e| FormatError::Mesh(e.to_string()))?;
        meshopt::optimize_overdraw_in_place(&mut indices, &adapter, 1.05);
        let vertices = meshopt::optimize_vertex_fetch(&mut indices, &vertices);

        let mut bounds = Aabb::EMPTY;
        for v in &vertices {
            bounds.grow(v.position);
        }
        let triangles = u32::try_from(indices.len() / 3).map_err(|_| FormatError::TooLarge)?;
        let submesh = encode_submesh(0, &vertices, &indices)?;
        let model = CookedModel {
            bounds,
            has_normals: true,
            has_uvs: false,
            has_colors: true,
            materials: vec![proxy_material()],
            lods: vec![CookedLod { error: self.source_error + simplify_error, submeshes: vec![submesh] }],
        };
        Ok(Some(ClusterMesh { model, error: self.source_error + simplify_error, triangles }))
    }
}

/// Rebuilds normals: each triangle corner averages (area-weighted) the
/// normals of the faces around its position that are within the crease
/// angle of its own face; then identical vertices are shared again.
/// Degenerate triangles are dropped.
#[allow(clippy::many_single_char_names)]
pub fn crease_normals(vertices: &[Vertex], indices: &[u32], crease_cos: f32) -> (Vec<Vertex>, Vec<u32>) {
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| v.position).collect();
    let (count, pos_id) = meshopt::generate_vertex_remap(&positions, None);
    let tris: Vec<[u32; 3]> = indices.as_chunks::<3>().0.to_vec();
    let face: Vec<Vec3> = tris
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| Vec3::from_array(vertices[i as usize].position));
            (b - a).cross(c - a)
        })
        .collect();
    // Faces around each position (CSR).
    let mut start = vec![0u32; count + 1];
    for t in &tris {
        for &i in t {
            start[pos_id[i as usize] as usize + 1] += 1;
        }
    }
    for i in 0..count {
        start[i + 1] += start[i];
    }
    let mut fill = start.clone();
    let mut around = vec![0u32; tris.len() * 3];
    for (f, t) in tris.iter().enumerate() {
        for &i in t {
            let p = pos_id[i as usize] as usize;
            around[fill[p] as usize] = u32::try_from(f).expect("face count fits u32");
            fill[p] += 1;
        }
    }
    let mut corners = Vec::with_capacity(tris.len() * 3);
    for (f, t) in tris.iter().enumerate() {
        let own = face[f].normalize_or_zero();
        if own == Vec3::ZERO {
            continue;
        }
        for &i in t {
            let p = pos_id[i as usize] as usize;
            let mut n = Vec3::ZERO;
            for &g in &around[start[p] as usize..start[p + 1] as usize] {
                let other = face[g as usize];
                if other.normalize_or_zero().dot(own) >= crease_cos {
                    n += other;
                }
            }
            let v = vertices[i as usize];
            corners.push(Vertex { normal: n.normalize_or(own).to_array(), ..v });
        }
    }
    let (count, remap) = meshopt::generate_vertex_remap(&corners, None);
    let out_indices = remap.clone();
    let out_vertices = meshopt::remap_vertex_buffer(&corners, count, &remap);
    (out_vertices, out_indices)
}

/// The proxies' one material: white, opaque, vertex colors carry albedo.
pub fn proxy_material() -> Material {
    Material { name: "far.proxy".into(), alpha: AlphaMode::Opaque, ..Material::default() }
}

/// meshopt-encodes one submesh (`sf_model`'s container fields are public).
pub fn encode_submesh(material: u32, vertices: &[Vertex], indices: &[u32]) -> Result<CookedSubmesh, FormatError> {
    let codec = |e: meshopt::Error| FormatError::Mesh(e.to_string());
    Ok(CookedSubmesh {
        material,
        vertex_count: u32::try_from(vertices.len()).map_err(|_| FormatError::TooLarge)?,
        index_count: u32::try_from(indices.len()).map_err(|_| FormatError::TooLarge)?,
        vertices: meshopt::encode_vertex_buffer(vertices).map_err(codec)?,
        indices: meshopt::encode_index_buffer(indices, vertices.len()).map_err(codec)?,
    })
}

/// Decodes a proxy's (or any cooked model's) first level into one vertex
/// and index list.
pub fn decode_level(model: &CookedModel, level: usize) -> Result<(Vec<Vertex>, Vec<u32>), FormatError> {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let Some(lod) = model.lods.get(level) else { return Ok((vertices, indices)) };
    for sub in &lod.submeshes {
        let (v, i) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
        let base = u32::try_from(vertices.len()).map_err(|_| FormatError::TooLarge)?;
        vertices.extend(v);
        indices.extend(i.into_iter().map(|k| k + base));
    }
    Ok((vertices, indices))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn unorm8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}
