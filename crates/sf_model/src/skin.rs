//! Deforming models: skins and morph targets, cooked without flattening.
//!
//! A model with any skinned mesh or morph target cooks to a
//! [`CookedSkinned`]: the usual [`CookedModel`] (bind-space geometry,
//! materials, LOD chain) plus, per submesh, a joint/weight stream and
//! optional morph deltas, a joint palette (names + inverse binds) and the
//! model's morph target list. Unskinned meshes of such a model become
//! single-joint skins of the nearest joint above them, so a character is
//! one kind of mesh. Spec: `docs/specs/bake.md` ("Skinned and morphed models").
//!
//! The layout maps straight onto Bevy's: `SkinVertex::joints` is
//! `Mesh::ATTRIBUTE_JOINT_INDEX` (`Uint16x4`), `weights` is
//! `ATTRIBUTE_JOINT_WEIGHT` (`Float32x4`), inverse binds are
//! `SkinnedMeshInverseBindposes` (`Mat4`, column-major), and morph deltas
//! are target-major like `Mesh::set_morph_targets` takes them.
//!
//! Morph layout (bake.md "Morph layout"): each submesh carries only the
//! targets that move one of its vertices ([`CookedSkinStreams::targets`]),
//! targets that move nothing anywhere are dropped from the model, and
//! [`MorphKind::Expression`] targets are dropped from LODs at and past
//! [`RUNTIME_MORPH_LODS`]. Bevy stores morphs densely (48 bytes per vertex
//! per target of the whole mesh) and loops over every target per vertex in
//! every pass, so zero blocks would cost real GPU memory and time.

use std::collections::{BTreeMap, HashMap};

use glam::{Affine3A, Mat3, Mat4, Vec3};
use serde::{Deserialize, Serialize};

use crate::cook::{CookError, CookSettings, Flags, Group, base_weights, lod_chain, optimize_indices, validate, vertex};
use crate::cooked::{Cooked, CookedLod, CookedModel, CookedSubmesh, FormatError, Vertex};
use crate::{Aabb, LodPolicy, MeshData, Model, MorphKind};

/// Influences per vertex in cooked data (glTF's one `JOINTS_0`/`WEIGHTS_0`
/// set; Bevy's skinning reads exactly four).
pub const MAX_INFLUENCES: usize = 4;

/// LODs `0..RUNTIME_MORPH_LODS` keep [`MorphKind::Expression`] targets;
/// coarser levels drop them (a blink is invisible at that distance, and UE
/// likewise lets each reduced LOD carry fewer morphs). Shape targets stay on
/// every level because assembly bakes them. Changing it changes cooked
/// bytes: bump [`crate::cooked::KEY_FORMAT`].
pub const RUNTIME_MORPH_LODS: usize = 2;

/// A target is "all zero" on a submesh when no position or normal delta
/// component exceeds this (meters for positions; 1 µm is below f32
/// resolution at 10 m, so it only catches importer noise).
pub const MORPH_EPSILON: f32 = 1e-6;

/// Simplifier weight of a vertex's "joint center" (its weights' blend of the
/// bind-space joint origins, in meters): collapsing an edge between vertices
/// driven by different joints costs about `weight × joint distance`, so hard
/// joint boundaries survive LODs and smooth blends still simplify. A
/// continuous form of UE's "Enforce Bone Boundaries". Changing it changes
/// cooked bytes: bump `mesh.cook`'s step version.
const JOINT_CENTER_WEIGHT: f32 = 0.5;
/// Simplifier weight of a vertex's largest morph displacement (meters), so
/// regions that morph keep their vertices longer.
const MORPH_WEIGHT: f32 = 1.0;

/// Per-vertex skin stream. 24 bytes, no padding (meshopt codec).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SkinVertex {
    /// Indices into [`CookedSkinned::joints`].
    pub joints: [u16; MAX_INFLUENCES],
    /// Sum to 1, largest first; unused slots are joint 0 with weight 0.
    pub weights: [f32; MAX_INFLUENCES],
}

/// One vertex's delta in one morph target. 24 bytes, no padding.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MorphDelta {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// One entry of the joint palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedJoint {
    /// The skeleton joint (node name) that drives it.
    pub name: String,
    /// Column-major 4×4: bind space → joint space. The same name may appear
    /// twice with different inverse binds (meshes bound in different poses).
    pub inverse_bind: [f32; 16],
}

/// A morph target of the whole model. Submeshes name the ones they carry
/// by index ([`CookedSkinStreams::targets`]); every entry moves at least one
/// vertex of some submesh.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MorphTargetInfo {
    pub name: String,
    pub default_weight: f32,
    pub kind: MorphKind,
}

/// One source mesh. Skinned models are not merged across meshes, so a
/// runtime can pick parts (a body part's node filter) before merging.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedPart {
    /// Node names from the root down to the mesh's node.
    pub node_path: Vec<String>,
}

/// The extra streams of one submesh, parallel to its [`CookedSubmesh`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedSkinStreams {
    /// Index into [`CookedSkinned::parts`].
    pub part: u32,
    /// meshopt-encoded [`SkinVertex`] buffer, one per vertex.
    pub skin: Vec<u8>,
    /// The targets this submesh carries, as ascending indices into
    /// [`CookedSkinned::morph_targets`]: those that move one of its vertices
    /// by more than [`MORPH_EPSILON`], minus expression targets past
    /// [`RUNTIME_MORPH_LODS`]. Empty: the submesh doesn't morph.
    pub targets: Vec<u16>,
    /// meshopt-encoded [`MorphDelta`] buffer, target-major, one block per
    /// entry of [`Self::targets`] in that order (all vertices of the first,
    /// then the second, …); `None` exactly when `targets` is empty.
    pub morphs: Option<Vec<u8>>,
}

/// A cooked deforming model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookedSkinned {
    /// Geometry in bind space; `bounds` are the rest pose.
    pub model: CookedModel,
    pub joints: Vec<CookedJoint>,
    pub morph_targets: Vec<MorphTargetInfo>,
    pub parts: Vec<CookedPart>,
    /// `streams[level][i]` belongs to `model.lods[level].submeshes[i]`.
    pub streams: Vec<Vec<CookedSkinStreams>>,
}

impl CookedSkinned {
    /// Decodes one submesh's skin stream and morph deltas (target-major, one
    /// block per entry of its [`CookedSkinStreams::targets`]; empty when it
    /// doesn't morph).
    pub fn decode_streams(&self, level: usize, submesh: usize) -> Result<(Vec<SkinVertex>, Vec<MorphDelta>), FormatError> {
        let missing = || FormatError::Body(format!("no submesh {submesh} in level {level}"));
        let sub = self.model.lods.get(level).and_then(|l| l.submeshes.get(submesh)).ok_or_else(missing)?;
        let streams = self.streams.get(level).and_then(|l| l.get(submesh)).ok_or_else(missing)?;
        let codec = |e: meshopt::Error| FormatError::Codec(e.to_string());
        let count = sub.vertex_count as usize;
        let skin = meshopt::decode_vertex_buffer::<SkinVertex>(&streams.skin, count).map_err(codec)?;
        if skin.iter().flat_map(|s| s.joints).any(|j| usize::from(j) >= self.joints.len()) {
            return Err(FormatError::Codec("joint index out of range".into()));
        }
        if streams.targets.iter().any(|&t| usize::from(t) >= self.morph_targets.len()) || streams.targets.is_empty() != streams.morphs.is_none() {
            return Err(FormatError::Codec("morph target list doesn't match the model".into()));
        }
        let morphs = match &streams.morphs {
            Some(bytes) => meshopt::decode_vertex_buffer::<MorphDelta>(bytes, count * streams.targets.len()).map_err(codec)?,
            None => Vec::new(),
        };
        Ok((skin, morphs))
    }

    /// Decodes one submesh with all its streams.
    pub fn decode_submesh(&self, level: usize, submesh: usize) -> Result<DecodedSubmesh, FormatError> {
        let (skin, morphs) = self.decode_streams(level, submesh)?;
        let sub = &self.model.lods[level].submeshes[submesh];
        let (vertices, indices) = sub.decode()?;
        Ok(DecodedSubmesh {
            material: sub.material,
            part: self.streams[level][submesh].part,
            vertices,
            indices,
            skin,
            targets: self.streams[level][submesh].targets.clone(),
            morphs,
        })
    }

    /// A rigid body part as a single-joint skin: every vertex follows
    /// `joint`, placed by `offset` (the part's transform relative to the
    /// joint, a socket offset). A negative determinant mirrors the part; the
    /// triangle winding is flipped here so front faces stay front faces,
    /// because renderers don't flip culling per instance (Bevy 0.20 culls
    /// back faces with a fixed front face; its determinant-sign flag only
    /// fixes tangent frames). Morph targets are kept. `None` for a model the
    /// bake skipped.
    pub fn rigid(cooked: &Cooked, joint: &str, offset: Affine3A) -> Result<Option<Self>, FormatError> {
        type Morphs = Vec<Vec<(Vec<u16>, Option<Vec<u8>>)>>;
        let (mut model, morph_targets, parts, morphs): (_, _, _, Morphs) = match cooked {
            Cooked::Skipped { .. } => return Ok(None),
            Cooked::Static(model) => {
                let morphs = model.lods.iter().map(|l| vec![(Vec::new(), None); l.submeshes.len()]).collect();
                (model.clone(), Vec::new(), vec![CookedPart { node_path: Vec::new() }], morphs)
            }
            Cooked::Skinned(s) => {
                let morphs = s.streams.iter().map(|l| l.iter().map(|st| (st.targets.clone(), st.morphs.clone())).collect()).collect();
                (s.model.clone(), s.morph_targets.clone(), s.parts.clone(), morphs)
            }
        };
        let part_of = |level: usize, i: usize| match cooked {
            Cooked::Skinned(s) => s.streams[level][i].part,
            _ => 0,
        };
        let mirrored = offset.matrix3.determinant() < 0.0;
        let codec = |e: meshopt::Error| FormatError::Codec(e.to_string());
        let mut streams = Vec::with_capacity(model.lods.len());
        for (level, (lod, morphs)) in model.lods.iter_mut().zip(morphs).enumerate() {
            let mut out = Vec::with_capacity(lod.submeshes.len());
            for (i, (sub, (targets, morphs))) in lod.submeshes.iter_mut().zip(morphs).enumerate() {
                if mirrored {
                    let (vertices, mut indices) = sub.decode()?;
                    for tri in indices.as_chunks_mut::<3>().0 {
                        tri.swap(1, 2);
                    }
                    *sub = CookedSubmesh::encode(sub.material, &vertices, &indices)?;
                }
                let skin = vec![SkinVertex { joints: [0; MAX_INFLUENCES], weights: [1.0, 0.0, 0.0, 0.0] }; sub.vertex_count as usize];
                out.push(CookedSkinStreams {
                    part: part_of(level, i),
                    skin: meshopt::encode_vertex_buffer(&skin).map_err(codec)?,
                    targets,
                    morphs,
                });
            }
            streams.push(out);
        }
        let inverse_bind = Mat4::from(offset).to_cols_array();
        Ok(Some(Self { model, joints: vec![CookedJoint { name: joint.to_owned(), inverse_bind }], morph_targets, parts, streams }))
    }
}

/// One submesh of a [`CookedSkinned`], decoded: what a loader or character
/// assembly works on.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedSubmesh {
    /// Index into the model's materials.
    pub material: u32,
    /// Index into [`CookedSkinned::parts`].
    pub part: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    /// One per vertex; joints index the model's palette.
    pub skin: Vec<SkinVertex>,
    /// Indices into [`CookedSkinned::morph_targets`], one block each in `morphs`.
    pub targets: Vec<u16>,
    /// Target-major: `morphs[k * vertices.len() + v]` is target `targets[k]`'s
    /// delta for vertex `v`.
    pub morphs: Vec<MorphDelta>,
}

impl DecodedSubmesh {
    /// Adds `weight × delta` of block `k` to every vertex: what a morph
    /// runtime evaluates (glTF; Bevy's `morph_vertex` adds position and
    /// normal deltas the same way, in target order, before skinning).
    /// Normals are left unnormalized, as there: skinning normalizes them.
    pub fn apply_block(&mut self, k: usize, weight: f32) {
        let n = self.vertices.len();
        if weight == 0.0 {
            return;
        }
        for (v, d) in self.vertices.iter_mut().zip(&self.morphs[k * n..(k + 1) * n]) {
            for c in 0..3 {
                v.position[c] += weight * d.position[c];
                v.normal[c] += weight * d.normal[c];
            }
        }
    }

    /// Bakes every [`MorphKind::Shape`] target into the vertices at
    /// `weight(target)` (an index into `infos`, the model's
    /// [`CookedSkinned::morph_targets`]) and removes it, leaving only
    /// expression targets: assembly with per-character weights, or a loader
    /// with the default weights.
    pub fn bake_shapes(&mut self, infos: &[MorphTargetInfo], weight: impl Fn(usize) -> f32) {
        let n = self.vertices.len();
        let mut kept = Vec::with_capacity(self.targets.len());
        let mut kept_morphs = Vec::new();
        for k in 0..self.targets.len() {
            let t = usize::from(self.targets[k]);
            if infos.get(t).is_some_and(|i| i.kind == MorphKind::Shape) {
                self.apply_block(k, weight(t));
            } else {
                kept.push(self.targets[k]);
                kept_morphs.extend_from_slice(&self.morphs[k * n..(k + 1) * n]);
            }
        }
        self.targets = kept;
        self.morphs = kept_morphs;
    }
}

/// A deforming group's extra per-vertex data, parallel to `Group::vertices`.
#[derive(Default)]
struct Extra {
    part: u32,
    skin: Vec<SkinVertex>,
    /// Vertex-major while building (`targets` per vertex); empty when the
    /// part doesn't morph.
    morphs: Vec<MorphDelta>,
}

/// The joint palette being built: entries keyed by node and inverse bind.
#[derive(Default)]
struct Palette {
    joints: Vec<CookedJoint>,
    /// Bind-space joint origin per entry (for the joint-center attribute).
    origins: Vec<Vec3>,
    /// Rest-pose skinning matrix per entry (for bounds).
    rest: Vec<Affine3A>,
    index: HashMap<(u32, [u32; 16]), u16>,
}

impl Palette {
    fn entry(&mut self, model: &Model, rest: &[Affine3A], node: u32, inverse_bind: Mat4) -> Result<u16, CookError> {
        let cols = inverse_bind.to_cols_array();
        let key = (node, cols.map(f32::to_bits));
        if let Some(&i) = self.index.get(&key) {
            return Ok(i);
        }
        let i = u16::try_from(self.joints.len()).map_err(|_| CookError::Malformed("more than 65536 joints".into()))?;
        let name = model.nodes.get(node as usize).map(|n| n.name.clone()).ok_or_else(|| {
            CookError::Malformed(format!("skin names missing node {node}"))
        })?;
        self.joints.push(CookedJoint { name, inverse_bind: cols });
        self.origins.push(inverse_bind.inverse().w_axis.truncate());
        self.rest.push(rest[node as usize] * Affine3A::from_mat4(inverse_bind));
        self.index.insert(key, i);
        Ok(i)
    }

    /// Rest-pose position of a bind-space vertex.
    fn rest_position(&self, v: &Vertex, s: &SkinVertex) -> Vec3 {
        let p = Vec3::from_array(v.position);
        s.joints.iter().zip(s.weights).map(|(&j, w)| self.rest[usize::from(j)].transform_point3(p) * w).sum()
    }
}

/// Cooks a model with skins or morph targets ([`crate::cook`] dispatches here).
pub(crate) fn cook_skinned(model: &Model, settings: &CookSettings) -> Result<Cooked, CookError> {
    if model.meshes.iter().any(|m| m.skinned && m.skin.is_none()) {
        return Ok(Cooked::Skipped { reason: "skinned mesh without skin data".into() });
    }
    let rest = model.node_transforms();
    // Rigid meshes bind to the nearest node that moves on its own: a skin
    // joint or an animated node.
    let mut movers = vec![false; model.nodes.len()];
    for node in model.meshes.iter().filter_map(|m| m.skin.as_ref()).flat_map(|s| &s.joints) {
        if let Some(m) = movers.get_mut(node.node as usize) {
            *m = true;
        }
    }
    for (m, node) in movers.iter_mut().zip(&model.nodes) {
        *m |= node.animated;
    }

    // Model-wide morph targets, matched by name in first-seen order.
    let mut targets: Vec<MorphTargetInfo> = Vec::new();
    for mesh in &model.meshes {
        for (i, t) in mesh.morph_targets.iter().enumerate() {
            let name = target_name(&t.name, i);
            if !targets.iter().any(|x| x.name == name) {
                targets.push(MorphTargetInfo { name, default_weight: t.default_weight, kind: t.kind });
            }
        }
    }

    let mut palette = Palette::default();
    let mut parts = Vec::new();
    let mut groups: BTreeMap<u32, Group> = BTreeMap::new();
    let mut extras: BTreeMap<u32, Extra> = BTreeMap::new();
    let mut flags = Flags::default();
    for (node_index, node) in model.nodes.iter().enumerate() {
        let Some(mesh_index) = node.mesh else { continue };
        let mesh = model
            .meshes
            .get(mesh_index as usize)
            .ok_or_else(|| CookError::Malformed(format!("node {:?} names missing mesh {mesh_index}", node.name)))?;
        validate(mesh, mesh_index)?;
        flags.normals |= mesh.normals.is_some();
        flags.uvs |= mesh.uvs.is_some();
        flags.colors |= mesh.colors.is_some();

        let part = u32::try_from(parts.len()).map_err(|_| CookError::Malformed("too many parts".into()))?;
        parts.push(CookedPart { node_path: node_path(model, node_index) });
        let (group, extra) = deforming_group(model, &rest, &movers, node_index, mesh, &targets, &mut palette, part)?;
        groups.insert(part, group);
        extras.insert(part, extra);
    }

    let morph_count = targets.len();
    let bounds = attributes_and_bounds(&mut groups, &extras, &palette, morph_count);
    let mut weights = base_weights(settings, flags);
    weights.extend([JOINT_CENTER_WEIGHT; 3]);
    weights.push(if morph_count > 0 { MORPH_WEIGHT } else { 0.0 });

    let mut lods = vec![CookedLod { error: 0.0, submeshes: Vec::new() }];
    let mut streams = vec![Vec::new()];
    for (key, group) in &groups {
        if !group.indices.is_empty() {
            let (sub, st) = finish_skinned(group, &extras[key], &targets, 0, group.indices.clone(), settings)?;
            lods[0].submeshes.push(sub);
            streams[0].push(st);
        }
    }
    let inputs: BTreeMap<u32, Vec<u32>> = groups.iter().map(|(&k, g)| (k, g.lod_input())).collect();
    // The chain's error is measured in bind space: the pose the weights were
    // painted in, and the one LOD switch distances assume.
    for (level, (level_indices, error)) in lod_chain(&groups, &inputs, &weights, bounds.radius(), lods[0].triangle_count(), settings)?
        .into_iter()
        .enumerate()
    {
        let (mut submeshes, mut level_streams) = (Vec::new(), Vec::new());
        for (key, indices) in level_indices {
            if !indices.is_empty() {
                let (sub, st) = finish_skinned(&groups[&key], &extras[&key], &targets, level + 1, indices, settings)?;
                submeshes.push(sub);
                level_streams.push(st);
            }
        }
        lods.push(CookedLod { error, submeshes });
        streams.push(level_streams);
    }
    let targets = drop_unused_targets(targets, &mut streams);

    Ok(Cooked::Skinned(CookedSkinned {
        model: CookedModel {
            bounds,
            has_normals: flags.normals,
            has_uvs: flags.uvs,
            has_colors: flags.colors,
            materials: model.materials.clone(),
            lods,
        },
        joints: palette.joints,
        morph_targets: targets,
        parts,
        streams,
    }))
}

/// Fills each group's simplifier attributes (normal/uv/color, joint center,
/// largest morph displacement) and returns the rest-pose bounds.
fn attributes_and_bounds(
    groups: &mut BTreeMap<u32, Group>,
    extras: &BTreeMap<u32, Extra>,
    palette: &Palette,
    morph_count: usize,
) -> Aabb {
    let mut bounds = Aabb::EMPTY;
    for (key, group) in groups {
        let extra = &extras[key];
        group.attributes = Vec::with_capacity(group.vertices.len() * (crate::cook::ATTRS + 4));
        for (i, v) in group.vertices.iter().enumerate() {
            let s = &extra.skin[i];
            let center: Vec3 = s.joints.iter().zip(s.weights).map(|(&j, w)| palette.origins[usize::from(j)] * w).sum();
            let morph = if extra.morphs.is_empty() {
                0.0
            } else {
                extra.morphs[i * morph_count..(i + 1) * morph_count]
                    .iter()
                    .map(|d| Vec3::from_array(d.position).length())
                    .fold(0.0, f32::max)
            };
            group.attributes.extend(Group::base_attributes(v));
            group.attributes.extend(center.to_array());
            group.attributes.push(morph);
            bounds.grow(palette.rest_position(v, s).to_array());
        }
    }
    bounds
}

/// Removes targets no submesh carries (they move nothing) from the model
/// list and renumbers the submeshes' lists.
fn drop_unused_targets(targets: Vec<MorphTargetInfo>, streams: &mut [Vec<CookedSkinStreams>]) -> Vec<MorphTargetInfo> {
    let mut used = vec![false; targets.len()];
    for t in streams.iter().flatten().flat_map(|s| &s.targets) {
        used[usize::from(*t)] = true;
    }
    let mut renumber = vec![u16::MAX; targets.len()];
    let mut kept = Vec::new();
    for (i, target) in targets.into_iter().enumerate() {
        if used[i] {
            renumber[i] = u16::try_from(kept.len()).unwrap_or(u16::MAX);
            kept.push(target);
        }
    }
    for t in streams.iter_mut().flatten().flat_map(|s| &mut s.targets) {
        *t = renumber[usize::from(*t)];
    }
    kept
}

fn target_name(name: &str, index: usize) -> String {
    if name.is_empty() { format!("target{index}") } else { name.to_owned() }
}

/// Node names from the root down to `node` (cycles cut).
fn node_path(model: &Model, node: usize) -> Vec<String> {
    let mut path = Vec::new();
    let mut at = Some(node);
    while let Some(i) = at.filter(|_| path.len() <= model.nodes.len()) {
        path.push(model.nodes[i].name.clone());
        at = model.nodes[i].parent.map(|p| p as usize).filter(|&p| p < model.nodes.len());
    }
    path.reverse();
    path
}

/// The node a rigid mesh at `node` follows: the nearest mover at or above
/// it, else its root.
fn rigid_joint(model: &Model, movers: &[bool], node: usize) -> usize {
    let mut at = node;
    for _ in 0..=model.nodes.len() {
        if movers[at] {
            return at;
        }
        match model.nodes[at].parent.map(|p| p as usize).filter(|&p| p < model.nodes.len()) {
            Some(p) => at = p,
            None => return at,
        }
    }
    at
}

/// One mesh as a deforming group: bind-space vertices, skin stream, morphs.
#[allow(clippy::too_many_arguments)]
fn deforming_group(
    model: &Model,
    rest: &[Affine3A],
    movers: &[bool],
    node_index: usize,
    mesh: &MeshData,
    targets: &[MorphTargetInfo],
    palette: &mut Palette,
    part: u32,
) -> Result<(Group, Extra), CookError> {
    let node = &model.nodes[node_index];
    let n = mesh.positions.len();
    // Skinned meshes are already in bind space (their node's transform is
    // ignored); rigid ones are flattened into model space and bound to
    // their joint at its rest transform.
    let (transform, rigid_slot) = if mesh.skin.is_some() {
        (Affine3A::IDENTITY, None)
    } else {
        let joint = rigid_joint(model, movers, node_index);
        let joint_u32 = u32::try_from(joint).map_err(|_| CookError::Malformed("too many nodes".into()))?;
        (rest[node_index], Some(palette.entry(model, rest, joint_u32, Mat4::from(rest[joint]).inverse())?))
    };
    let mut slots: Vec<u16> = Vec::new();
    if let Some(skin) = &mesh.skin {
        for j in &skin.joints {
            slots.push(palette.entry(model, rest, j.node, Mat4::from_cols_array(&j.inverse_bind))?);
        }
    }
    let normal_matrix = Mat3::from(transform.matrix3).inverse().transpose();
    let linear = Mat3::from(transform.matrix3);
    let mirrored = transform.matrix3.determinant() < 0.0;

    let mut group = Group { material: mesh.material, ..Group::default() };
    let mut extra = Extra { part, ..Extra::default() };
    let target_slots: Vec<usize> = mesh
        .morph_targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let name = target_name(&t.name, i);
            targets.iter().position(|x| x.name == name).expect("targets were collected from every mesh")
        })
        .collect();
    for i in 0..n {
        group.vertices.push(vertex(mesh, i, &transform, &normal_matrix));
        group.locked.push(node.lod == LodPolicy::Keep);
        extra.skin.push(match (&mesh.skin, rigid_slot) {
            (Some(skin), _) => influences(&skin.influences[i], &slots)?,
            (None, slot) => SkinVertex { joints: [slot.unwrap_or(0), 0, 0, 0], weights: [1.0, 0.0, 0.0, 0.0] },
        });
        if !mesh.morph_targets.is_empty() {
            let start = extra.morphs.len();
            extra.morphs.resize(start + targets.len(), MorphDelta::default());
            for (t, &slot) in mesh.morph_targets.iter().zip(&target_slots) {
                let normal = t.normals.as_ref().map_or(Vec3::ZERO, |ns| normal_matrix * Vec3::from_array(ns[i]));
                extra.morphs[start + slot] = MorphDelta {
                    position: (linear * Vec3::from_array(t.positions[i])).to_array(),
                    normal: normal.to_array(),
                };
            }
        }
    }
    for tri in mesh.indices.as_chunks::<3>().0 {
        // A mirroring transform flips winding; restore it.
        group.indices.extend(if mirrored { [tri[0], tri[2], tri[1]] } else { *tri });
        group.lod0_only.push(node.lod == LodPolicy::Drop);
    }
    weld(&mut group, &mut extra, targets.len());
    Ok((group, extra))
}

/// The [`MAX_INFLUENCES`] largest influences, mapped to palette slots and
/// renormalized; repeated joints are summed. No influence at all binds the
/// vertex fully to the mesh's first joint.
fn influences(source: &[(u16, f32)], slots: &[u16]) -> Result<SkinVertex, CookError> {
    let mut merged: Vec<(u16, f32)> = Vec::with_capacity(source.len());
    for &(joint, weight) in source {
        if !(weight.is_finite() && weight > 0.0) {
            continue;
        }
        let slot = *slots
            .get(usize::from(joint))
            .ok_or_else(|| CookError::Malformed(format!("influence names joint {joint} of {}", slots.len())))?;
        match merged.iter_mut().find(|(j, _)| *j == slot) {
            Some((_, w)) => *w += weight,
            None => merged.push((slot, weight)),
        }
    }
    merged.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    merged.truncate(MAX_INFLUENCES);
    let total: f32 = merged.iter().map(|(_, w)| w).sum();
    let mut out = SkinVertex::default();
    if total <= 0.0 {
        out.joints[0] = slots.first().copied().unwrap_or(0);
        out.weights[0] = 1.0;
        return Ok(out);
    }
    for (k, (joint, weight)) in merged.into_iter().enumerate() {
        out.joints[k] = joint;
        out.weights[k] = weight / total;
    }
    Ok(out)
}

/// Merges vertices identical in every stream.
fn weld(group: &mut Group, extra: &mut Extra, targets: usize) {
    let morphs = !extra.morphs.is_empty();
    let mut index: HashMap<Vec<u8>, u32> = HashMap::new();
    let mut remap = Vec::with_capacity(group.vertices.len());
    let mut kept = Group { material: group.material, ..Group::default() };
    let mut kept_extra = Extra { part: extra.part, ..Extra::default() };
    for i in 0..group.vertices.len() {
        let mut key = Vec::with_capacity(64);
        key.extend_from_slice(meshopt::typed_to_bytes(std::slice::from_ref(&group.vertices[i])));
        key.extend_from_slice(meshopt::typed_to_bytes(std::slice::from_ref(&extra.skin[i])));
        if morphs {
            key.extend_from_slice(meshopt::typed_to_bytes(&extra.morphs[i * targets..(i + 1) * targets]));
        }
        let next = u32::try_from(kept.vertices.len()).unwrap_or(u32::MAX);
        let new = *index.entry(key).or_insert_with(|| {
            kept.vertices.push(group.vertices[i]);
            kept.locked.push(false);
            kept_extra.skin.push(extra.skin[i]);
            if morphs {
                kept_extra.morphs.extend_from_slice(&extra.morphs[i * targets..(i + 1) * targets]);
            }
            next
        });
        kept.locked[new as usize] |= group.locked[i];
        remap.push(new);
    }
    kept.indices = group.indices.iter().map(|&i| remap[i as usize]).collect();
    kept.lod0_only = std::mem::take(&mut group.lod0_only);
    *group = kept;
    *extra = kept_extra;
}

/// Index optimization, a compact per-level vertex order shared by every
/// stream, then encoding.
fn finish_skinned(
    group: &Group,
    extra: &Extra,
    infos: &[MorphTargetInfo],
    level: usize,
    mut indices: Vec<u32>,
    settings: &CookSettings,
) -> Result<(CookedSubmesh, CookedSkinStreams), CookError> {
    let targets = infos.len();
    optimize_indices(group, &mut indices, settings)?;
    // Vertex fetch order: first use in the index buffer (what
    // `meshopt::optimize_vertex_fetch` does, applied to every stream).
    let mut remap = vec![u32::MAX; group.vertices.len()];
    let mut order = Vec::new();
    for i in &mut indices {
        let slot = &mut remap[*i as usize];
        if *slot == u32::MAX {
            *slot = u32::try_from(order.len()).map_err(|_| CookError::Malformed("too many vertices".into()))?;
            order.push(*i as usize);
        }
        *i = *slot;
    }
    let vertices: Vec<Vertex> = order.iter().map(|&o| group.vertices[o]).collect();
    let skin: Vec<SkinVertex> = order.iter().map(|&o| extra.skin[o]).collect();
    // The targets this level of the part carries: those that move one of
    // its vertices, minus expression targets on coarse levels.
    let moves = |d: &MorphDelta| d.position.iter().chain(&d.normal).any(|c| c.abs() > MORPH_EPSILON);
    let carried: Vec<usize> = if extra.morphs.is_empty() {
        Vec::new()
    } else {
        (0..targets)
            .filter(|&t| level < RUNTIME_MORPH_LODS || infos[t].kind == MorphKind::Shape)
            .filter(|&t| order.iter().any(|&o| moves(&extra.morphs[o * targets + t])))
            .collect()
    };
    // Vertex-major while cooking, target-major on disk.
    let morphs: Vec<MorphDelta> = carried.iter().flat_map(|&t| order.iter().map(move |&o| extra.morphs[o * targets + t])).collect();
    let codec = |e: meshopt::Error| CookError::Format(FormatError::Codec(e.to_string()));
    let streams = CookedSkinStreams {
        part: extra.part,
        skin: meshopt::encode_vertex_buffer(&skin).map_err(codec)?,
        targets: carried.iter().map(|&t| u16::try_from(t).map_err(|_| CookError::Malformed("more than 65536 morph targets".into()))).collect::<Result<_, _>>()?,
        morphs: (!carried.is_empty()).then(|| meshopt::encode_vertex_buffer(&morphs)).transpose().map_err(codec)?,
    };
    Ok((CookedSubmesh::encode(group.material, &vertices, &indices)?, streams))
}
