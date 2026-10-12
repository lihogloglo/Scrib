//! Cooking: flatten a static model, merge per material, build the LOD chain.
//!
//! Each level is simplified from LOD0 (not from the previous level), so the
//! reported error is the true deviation from the full model. Errors are
//! absolute (meters): one model-wide cap per level, so every submesh of a
//! level degrades alike. Deforming models take the same chain through
//! `skin.rs` (no flatten; one group per part).

use std::collections::BTreeMap;

use glam::{Affine3A, Mat3, Vec3};
use meshopt::{SimplifyOptions, VertexDataAdapter};

use crate::cooked::{Cooked, CookedLod, CookedModel, CookedSubmesh, FormatError, Vertex};
use crate::{Aabb, LodPolicy, MeshData, Model};

/// Tunables for [`cook`]. Part of the bake key: changing them re-cooks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CookSettings {
    /// Levels including LOD0.
    pub max_lods: usize,
    /// No further level once the previous one is below twice this.
    pub min_triangles: u32,
    /// A level must remove at least this fraction of the previous level's
    /// triangles, or the chain stops.
    pub min_reduction: f32,
    /// LOD1's error cap as a fraction of the model's bounding radius; each
    /// further level doubles it.
    pub base_error: f32,
    pub normal_weight: f32,
    pub uv_weight: f32,
    pub color_weight: f32,
    /// Keep open borders in place. Off by default: each level's error is
    /// sub-pixel at its switch distance, so seams between modular pieces stay
    /// sub-pixel anyway, while locking freezes most of a model made of open
    /// pieces (measured on real content: most chains stalled at one level).
    pub lock_borders: bool,
    pub overdraw_threshold: f32,
}

impl Default for CookSettings {
    fn default() -> Self {
        Self {
            max_lods: 5,
            min_triangles: 64,
            min_reduction: 0.25,
            base_error: 0.01,
            normal_weight: 0.5,
            uv_weight: 0.5,
            color_weight: 0.25,
            lock_borders: false,
            overdraw_threshold: 1.05,
        }
    }
}

impl CookSettings {
    /// Stable bytes for the bake key.
    pub fn key_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend((self.max_lods as u64).to_le_bytes());
        out.extend(self.min_triangles.to_le_bytes());
        for f in [
            self.min_reduction,
            self.base_error,
            self.normal_weight,
            self.uv_weight,
            self.color_weight,
            self.overdraw_threshold,
        ] {
            out.extend(f.to_le_bytes());
        }
        out.push(u8::from(self.lock_borders));
        out
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CookError {
    #[error("malformed model: {0}")]
    Malformed(String),
    #[error(transparent)]
    Format(#[from] FormatError),
}

/// Floats per vertex in the simplifier's attribute stream: normal, uv, color.
pub(crate) const ATTRS: usize = 9;

/// One merged piece of geometry: a material's static geometry, or one
/// deforming part.
#[derive(Default)]
pub(crate) struct Group {
    pub material: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    /// Vertices that must not move (from `LodPolicy::Keep` nodes).
    pub locked: Vec<bool>,
    /// Per triangle: present in LOD0 only (`LodPolicy::Drop`).
    pub lod0_only: Vec<bool>,
    /// Simplifier attributes, one weight's worth of floats per vertex
    /// (filled after welding).
    pub attributes: Vec<f32>,
}

impl Group {
    /// Normal, uv and color as simplifier attributes ([`ATTRS`] floats).
    pub fn base_attributes(v: &Vertex) -> [f32; ATTRS] {
        let [red, green, blue, alpha] = v.color.map(|c| f32::from(c) / 255.0);
        [v.normal[0], v.normal[1], v.normal[2], v.uv[0], v.uv[1], red, green, blue, alpha]
    }

    /// LOD1+ input: the triangles not marked LOD0-only.
    pub fn lod_input(&self) -> Vec<u32> {
        self.indices
            .as_chunks::<3>()
            .0
            .iter()
            .zip(&self.lod0_only)
            .filter(|(_, drop)| !**drop)
            .flat_map(|(tri, _)| tri.iter().copied())
            .collect()
    }
}

/// Cooks a model. Models with skins or morph targets come back as
/// [`Cooked::Skinned`] (bake.md, "Skinned and morphed models"). Models with
/// other animated nodes, and skinned meshes without skin data, come back as
/// [`Cooked::Skipped`]; they keep the runtime import path.
pub fn cook(model: &Model, settings: &CookSettings) -> Result<Cooked, CookError> {
    if model.meshes.iter().any(MeshData::deforms) {
        return crate::skin::cook_skinned(model, settings);
    }
    if let Some(node) = model.nodes.iter().find(|n| n.animated) {
        return Ok(Cooked::Skipped { reason: format!("animated node {:?}", node.name) });
    }
    cook_rest(model, settings)
}

/// Cooks a model as a character body driven by a rig from outside the
/// model (animation files that name its nodes): **rigid skinning**, what
/// DCC importers do with rigid animation (UE's "Import Rigid Animation"
/// makes a skeletal mesh of it). Every transform node (a node without a
/// mesh) may move, so each rigid mesh becomes a single-joint part of its
/// nearest transform ancestor, merged per material like any skin; skinned
/// meshes and morphs cook as in [`cook`]. A model whose meshes are all
/// rigid (a "segmented" creature animated per node) thus comes back as a
/// [`Cooked::Skinned`] that character assembly binds like any other. Only
/// skinned meshes without skin data are still [`Cooked::Skipped`].
///
/// A mesh node is never a joint here, even when something animates it (a
/// morph, a material, visibility): rigs are made of transform nodes, so the
/// mesh follows its transform parent instead of a joint no rig has.
pub fn cook_rig(model: &Model, settings: &CookSettings) -> Result<Cooked, CookError> {
    let mut rig = model.clone();
    for node in &mut rig.nodes {
        node.animated = node.mesh.is_none();
    }
    crate::skin::cook_skinned(&rig, settings)
}

/// Cooks a model frozen in its rest pose, for far use (the `mesh.rest`
/// product): animated nodes are taken at their rest transforms, as the
/// importer wrote them. Material and UV animation don't change geometry;
/// importers leave out what has no static shape (particle systems carry no
/// mesh, inactive switch children are hidden). Skinned meshes are still
/// [`Cooked::Skipped`]: their rest pose isn't a pose anyone sees.
pub fn cook_rest(model: &Model, settings: &CookSettings) -> Result<Cooked, CookError> {
    if model.meshes.iter().any(MeshData::is_skinned) {
        return Ok(Cooked::Skipped { reason: "skinned mesh".into() });
    }

    let (mut groups, flags) = flatten(model)?;
    let mut bounds = Aabb::EMPTY;
    for group in groups.values_mut() {
        weld(group);
        group.attributes = group.vertices.iter().flat_map(Group::base_attributes).collect();
        for v in &group.vertices {
            bounds.grow(v.position);
        }
    }

    let mut lods = vec![CookedLod { error: 0.0, submeshes: Vec::new() }];
    for group in groups.values() {
        if !group.indices.is_empty() {
            lods[0].submeshes.push(finish(group, group.indices.clone(), settings)?);
        }
    }

    let weights = base_weights(settings, flags);
    let inputs: BTreeMap<u32, Vec<u32>> = groups.iter().map(|(&m, g)| (m, g.lod_input())).collect();
    for (level, error) in lod_chain(&groups, &inputs, &weights, bounds.radius(), lods[0].triangle_count(), settings)? {
        let mut submeshes = Vec::new();
        for (material, indices) in level {
            if !indices.is_empty() {
                submeshes.push(finish(&groups[&material], indices, settings)?);
            }
        }
        lods.push(CookedLod { error, submeshes });
    }

    Ok(Cooked::Static(CookedModel {
        bounds,
        has_normals: flags.normals,
        has_uvs: flags.uvs,
        has_colors: flags.colors,
        materials: model.materials.clone(),
        lods,
    }))
}

/// Attribute weights for [`Group::base_attributes`]; absent attributes weigh nothing.
pub(crate) fn base_weights(settings: &CookSettings, flags: Flags) -> Vec<f32> {
    [
        [settings.normal_weight * f32::from(u8::from(flags.normals)); 3].as_slice(),
        &[settings.uv_weight * f32::from(u8::from(flags.uvs)); 2],
        &[settings.color_weight * f32::from(u8::from(flags.colors)); 4],
    ]
    .concat()
}

/// Simplified levels: indices per group and the level's error.
pub(crate) type Levels = Vec<(BTreeMap<u32, Vec<u32>>, f32)>;

/// One simplified level: indices per group, worst error, triangle count.
type Level = (BTreeMap<u32, Vec<u32>>, f32, usize);

/// Builds LOD1+ from `groups` (`inputs`: their LOD1+ index buffers, same
/// keys) and returns each level's indices per group and its error.
pub(crate) fn lod_chain(
    groups: &BTreeMap<u32, Group>,
    inputs: &BTreeMap<u32, Vec<u32>>,
    weights: &[f32],
    radius: f32,
    lod0_triangles: u32,
    settings: &CookSettings,
) -> Result<Levels, CookError> {
    let mut levels = Vec::new();
    let mut prev_tris = lod0_triangles;
    let mut prev_error = 0.0f32;
    // Each level targets half the previous triangle count. Its error cap
    // starts at `base_error` (×2 per level) and doubles until the level
    // removes enough, up to the last level's cap: low-poly models often can't
    // lose 25% within 1% of their radius, but a coarser level shown farther
    // away still saves triangles (switch distances come from the real error).
    #[allow(clippy::cast_precision_loss)]
    let max_cap = radius * settings.base_error * 2f32.powi(i32::try_from(settings.max_lods.saturating_sub(2)).unwrap_or(0));
    let mut cap = radius * settings.base_error;
    for level in 1..settings.max_lods {
        if radius <= 0.0 || prev_tris < settings.min_triangles * 2 {
            break;
        }
        let ratio = 0.5f32.powi(i32::try_from(level).unwrap_or(i32::MAX));
        #[allow(clippy::cast_precision_loss)]
        let enough = |tris: usize| tris > 0 && (tris as f32) <= prev_tris as f32 * (1.0 - settings.min_reduction);
        let run = |cap: f32, sloppy: bool| -> Result<Level, CookError> {
            let mut indices = BTreeMap::new();
            let mut error = prev_error;
            for (&material, group) in groups {
                let input = &inputs[&material];
                if !input.is_empty() {
                    let (i, e) = simplify(group, input, ratio, cap, level, weights, settings, sloppy)?;
                    error = error.max(e);
                    indices.insert(material, i);
                }
            }
            let tris = indices.values().map(|i| i.len() / 3).sum();
            Ok((indices, error, tris))
        };

        let mut found = None;
        let mut try_cap = cap;
        loop {
            let (indices, error, tris) = run(try_cap, false)?;
            if enough(tris) {
                found = Some((indices, error, tris));
                break;
            }
            if try_cap >= max_cap {
                break;
            }
            try_cap = (try_cap * 2.0).min(max_cap);
        }
        // Still stalled (thin parts, attribute seams everywhere): one last
        // level from the sloppy simplifier, if it helps.
        let last = found.is_none();
        if last {
            let (indices, error, tris) = run(max_cap * 2.0, true)?;
            if enough(tris) {
                found = Some((indices, error, tris));
            }
        }
        let Some((indices, error, tris)) = found else { break };
        levels.push((indices, error));
        prev_tris = u32::try_from(tris).unwrap_or(u32::MAX);
        prev_error = error;
        cap = (try_cap * 2.0).min(max_cap);
        if last {
            break;
        }
    }

    Ok(levels)
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Flags {
    pub normals: bool,
    pub uvs: bool,
    pub colors: bool,
}

/// Checks a mesh's per-vertex streams and index buffer against its vertex count.
pub(crate) fn validate(mesh: &MeshData, mesh_index: u32) -> Result<(), CookError> {
    let n = mesh.positions.len();
    let check = |len: Option<usize>, what: &str| match len {
        Some(len) if len != n => Err(CookError::Malformed(format!("mesh {mesh_index} {what}: {len} values for {n} vertices"))),
        _ => Ok(()),
    };
    check(mesh.normals.as_ref().map(Vec::len), "normals")?;
    check(mesh.uvs.as_ref().map(Vec::len), "uvs")?;
    check(mesh.colors.as_ref().map(Vec::len), "colors")?;
    check(mesh.skin.as_ref().map(|s| s.influences.len()), "skin influences")?;
    for target in &mesh.morph_targets {
        check(Some(target.positions.len()), "morph positions")?;
        check(target.normals.as_ref().map(Vec::len), "morph normals")?;
    }
    if !mesh.indices.len().is_multiple_of(3) || mesh.indices.iter().any(|&i| i as usize >= n) {
        return Err(CookError::Malformed(format!("bad index buffer in mesh {mesh_index}")));
    }
    Ok(())
}

/// One mesh vertex with `transform` applied (normals by `normal_matrix`,
/// its inverse transpose).
pub(crate) fn vertex(mesh: &MeshData, i: usize, transform: &Affine3A, normal_matrix: &Mat3) -> Vertex {
    let position = transform.transform_point3(Vec3::from_array(mesh.positions[i])).to_array();
    let normal = mesh
        .normals
        .as_ref()
        .map_or([0.0; 3], |ns| (*normal_matrix * Vec3::from_array(ns[i])).normalize_or_zero().to_array());
    let uv = mesh.uvs.as_ref().map_or([0.0; 2], |uvs| uvs[i]);
    let color = mesh.colors.as_ref().map_or([255; 4], |cs| cs[i].map(unorm8));
    Vertex { position, normal, uv, color }
}

/// Transforms every mesh into model space and merges per material.
fn flatten(model: &Model) -> Result<(BTreeMap<u32, Group>, Flags), CookError> {
    let transforms = model.node_transforms();
    let mut groups: BTreeMap<u32, Group> = BTreeMap::new();
    let mut flags = Flags::default();
    for (node, transform) in model.nodes.iter().zip(&transforms) {
        let Some(mesh_index) = node.mesh else { continue };
        let mesh = model
            .meshes
            .get(mesh_index as usize)
            .ok_or_else(|| CookError::Malformed(format!("node {:?} names missing mesh {mesh_index}", node.name)))?;
        validate(mesh, mesh_index)?;
        let n = mesh.positions.len();
        flags.normals |= mesh.normals.is_some();
        flags.uvs |= mesh.uvs.is_some();
        flags.colors |= mesh.colors.is_some();

        let normal_matrix = Mat3::from(transform.matrix3).inverse().transpose();
        let mirrored = transform.matrix3.determinant() < 0.0;
        let group = groups.entry(mesh.material).or_insert_with(|| Group { material: mesh.material, ..Group::default() });
        let base = u32::try_from(group.vertices.len()).map_err(|_| CookError::Malformed("too many vertices".into()))?;
        for i in 0..n {
            group.vertices.push(vertex(mesh, i, transform, &normal_matrix));
            group.locked.push(node.lod == LodPolicy::Keep);
        }
        for tri in mesh.indices.as_chunks::<3>().0 {
            let [a, b, c] = tri.map(|i| i + base);
            // A mirroring transform flips winding; restore it.
            group.indices.extend(if mirrored { [a, c, b] } else { [a, b, c] });
            group.lod0_only.push(node.lod == LodPolicy::Drop);
        }
    }
    Ok((groups, flags))
}

/// Merges bit-identical vertices (shared positions with equal attributes).
fn weld(group: &mut Group) {
    let (count, remap) = meshopt::generate_vertex_remap(&group.vertices, Some(&group.indices));
    let mut locked = vec![false; count];
    for (old, &new) in remap.iter().enumerate() {
        if new != u32::MAX {
            locked[new as usize] |= group.locked[old];
        }
    }
    group.indices = meshopt::remap_index_buffer(Some(&group.indices), count, &remap);
    group.vertices = meshopt::remap_vertex_buffer(&group.vertices, count, &remap);
    group.locked = locked;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn simplify(
    group: &Group,
    input: &[u32],
    ratio: f32,
    cap: f32,
    level: usize,
    weights: &[f32],
    settings: &CookSettings,
    sloppy: bool,
) -> Result<(Vec<u32>, f32), CookError> {
    let bytes = meshopt::typed_to_bytes(&group.vertices);
    let adapter = VertexDataAdapter::new(bytes, size_of::<Vertex>(), 0).map_err(|e| CookError::Malformed(e.to_string()))?;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    let target = ((input.len() / 3) as f32 * ratio).round().max(1.0) as usize * 3;
    let mut error = 0.0f32;
    let indices = if sloppy {
        meshopt::simplify_sloppy_with_locks(
            input,
            &adapter,
            &group.locked,
            target,
            // The sloppy simplifier's error is relative to the mesh extent.
            cap / meshopt::simplify_scale(&adapter).max(f32::MIN_POSITIVE),
            Some(&mut error),
        )
    } else {
        let mut options = SimplifyOptions::ErrorAbsolute;
        if settings.lock_borders {
            options |= SimplifyOptions::LockBorder;
        }
        if level >= 2 {
            options |= SimplifyOptions::Prune;
        }
        meshopt::simplify_with_attributes_and_locks(
            input,
            &adapter,
            &group.attributes,
            weights,
            size_of_val(weights),
            &group.locked,
            target,
            cap,
            options,
            Some(&mut error),
        )
    };
    // ...and so is the error it reports.
    if sloppy {
        error *= meshopt::simplify_scale(&adapter);
    }
    Ok((indices, error))
}

/// Vertex cache, overdraw, and fetch optimization, then a compact vertex
/// buffer for this level, then encoding.
fn finish(group: &Group, mut indices: Vec<u32>, settings: &CookSettings) -> Result<CookedSubmesh, CookError> {
    optimize_indices(group, &mut indices, settings)?;
    let vertices = meshopt::optimize_vertex_fetch(&mut indices, &group.vertices);
    Ok(CookedSubmesh::encode(group.material, &vertices, &indices)?)
}

/// Vertex cache and overdraw optimization of one level's index buffer.
pub(crate) fn optimize_indices(group: &Group, indices: &mut Vec<u32>, settings: &CookSettings) -> Result<(), CookError> {
    *indices = meshopt::optimize_vertex_cache(indices, group.vertices.len());
    let bytes = meshopt::typed_to_bytes(&group.vertices);
    let adapter = VertexDataAdapter::new(bytes, size_of::<Vertex>(), 0).map_err(|e| CookError::Malformed(e.to_string()))?;
    meshopt::optimize_overdraw_in_place(indices, &adapter, settings.overdraw_threshold);
    Ok(())
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn unorm8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}
