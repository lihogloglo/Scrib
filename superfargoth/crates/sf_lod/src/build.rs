//! Node builders: the pure functions the `lod.l1`–`lod.l3` bake steps wrap.
//!
//! L1 gathers a streaming cell's eligible placements (merge, impostor,
//! light); L2 and L3 merge their children's blobs hierarchically, so each
//! level reads a few blobs instead of every model again.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use glam::{Affine3A, DVec3, IVec3, Quat, Vec3};
use sf_model::{Aabb, AlphaMode, CookedLod, CookedModel};
use sf_vfs::VfsPath;
use sf_world::{IndexedPlacement, Partition, PlacementClass, PlacementFlags};

use crate::blob::{ImpostorInstance, ImpostorModel, LightPoint, NodeBlob, NodeHeader, PosRepr};
use crate::cluster::{
    ClusterBuilder, ClusterMesh, SimplifySettings, blend_proxy_material, fold_back_faces, proxy_material, quantise_emissive,
    unquantise_emissive,
};
use crate::impostor::ImpostorSettings;
use crate::levels::FarLevels;
use crate::node::{FIRST_LEVEL, NodeKey};
use crate::policy::{FarLight, FarPolicy, FarSource, ModelSummary, far_eligible};
use crate::FormatError;

/// Everything that changes node bytes (part of every `lod.*` bake key).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FarBakeSettings {
    pub levels: FarLevels,
    pub impostor: ImpostorSettings,
    pub simplify: SimplifySettings,
    /// Lights closer than this merge into one sprite, per level (L1..L3).
    pub light_merge: [f32; 3],
}

impl Default for FarBakeSettings {
    fn default() -> Self {
        Self {
            levels: FarLevels::default(),
            impostor: ImpostorSettings::default(),
            simplify: SimplifySettings::default(),
            light_merge: [0.0, 12.0, 48.0],
        }
    }
}

impl FarBakeSettings {
    /// Stable bytes for bake keys.
    pub fn key_bytes(&self) -> Vec<u8> {
        let l = &self.levels;
        let mut out = Vec::new();
        for f in l.handoff.iter().chain(&[l.band, l.screen.height_px, l.screen.fov_y, l.screen.pixel_error, l.screen.bias, l.min_pixels]) {
            out.extend(f.to_le_bytes());
        }
        out.extend(postcard::to_stdvec(&self.impostor).expect("settings serialize"));
        for f in [self.simplify.crease_cos, self.simplify.color_weight] {
            out.extend(f.to_le_bytes());
        }
        out.push(u8::from(self.simplify.prune));
        for f in self.light_merge {
            out.extend(f.to_le_bytes());
        }
        out
    }
}

/// What an L1 build knows about one model.
#[derive(Clone, Debug)]
pub struct FarModel {
    pub policy: FarPolicy,
    pub summary: ModelSummary,
    /// The cooked model (needed for `Merge`).
    pub cooked: Option<Arc<CookedModel>>,
    /// Per material: linear albedo (base color × texture average) and
    /// texture coverage in `w`. Empty: base colors only.
    pub albedo: Vec<[f32; 4]>,
}

/// The sphere an impostor of a model is captured in and drawn with.
pub fn impostor_sphere(s: &ModelSummary) -> (Vec3, f32) {
    let min = Vec3::from_array(s.min);
    let max = Vec3::from_array(s.max);
    ((min + max) * 0.5, ((max - min) * 0.5).length().max(1e-3))
}

/// Rotation about +Y that best matches `q` (impostors only turn about Y).
pub fn yaw_of(q: Quat) -> f32 {
    let f = q * Vec3::Z;
    f.x.atan2(f.z)
}

fn rel(center: sf_coords::WorldPos, p: sf_coords::WorldPos) -> Vec3 {
    center.delta_f32(p)
}

/// A node's center (entity origin): the middle of its box in X and Z, and
/// the middle of its children's origins in Y (cells are cubes, content
/// sits near the ground, so the box center can be far above or below it).
pub fn node_center(key: NodeKey, partition: Partition, children: impl Iterator<Item = sf_coords::WorldPos>) -> sf_coords::WorldPos {
    let center = key.center(partition);
    let (lo, hi) = children.map(|p| rel(center, p).y).fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), y| (lo.min(y), hi.max(y)));
    if lo > hi { center } else { center.offset(Vec3::Y * f32::midpoint(lo, hi)) }
}

/// A blended proxy batch: quantised emissive ([`quantise_emissive`]) and double-sidedness.
fn blend_key(emissive: [f32; 3], double_sided: bool) -> ([u16; 3], bool) {
    (quantise_emissive(emissive), double_sided)
}

/// A blended part's emissive as the near shader shows it on average: the
/// material emissive glows through the base texture (`albedo` = base colour ×
/// texture average, so the texture average is `albedo / base`).
fn blend_emissive(material: &sf_model::Material, albedo: Option<&[f32; 4]>) -> [f32; 3] {
    match albedo {
        Some(a) if material.base_color_texture.is_some() => [0, 1, 2].map(|c| {
            let b = material.base_color[c];
            let tex_rgb = if b > 1e-4 { (a[c] / b).clamp(0.0, 1.0) } else { 1.0 };
            material.emissive[c] * tex_rgb
        }),
        _ => material.emissive,
    }
}

/// Builds the opaque and blended proxy submeshes into one `CookedModel`.
fn assemble_cluster_mesh(
    opaque: ClusterBuilder,
    blends: BTreeMap<([u16; 3], bool), ClusterBuilder>,
    budget: f32,
    simplify: &SimplifySettings,
) -> Result<Option<ClusterMesh>, FormatError> {
    let mut materials = Vec::new();
    let mut submeshes = Vec::new();
    let mut bounds = Aabb::EMPTY;
    let mut error = 0.0f32;
    let mut triangles = 0u32;

    if let Some(built) = opaque.build_submesh(0, budget, simplify)? {
        bounds.grow(built.bounds.min);
        bounds.grow(built.bounds.max);
        error = error.max(built.error);
        triangles = triangles.checked_add(built.triangles).ok_or(FormatError::TooLarge)?;
        materials.push(proxy_material());
        submeshes.push(built.submesh);
    }
    for ((q_emissive, double_sided), builder) in blends {
        let mat_idx = u32::try_from(materials.len()).map_err(|_| FormatError::TooLarge)?;
        if let Some(built) = builder.build_submesh(mat_idx, budget, simplify)? {
            bounds.grow(built.bounds.min);
            bounds.grow(built.bounds.max);
            error = error.max(built.error);
            triangles = triangles.checked_add(built.triangles).ok_or(FormatError::TooLarge)?;
            materials.push(blend_proxy_material(unquantise_emissive(q_emissive), double_sided));
            submeshes.push(built.submesh);
        }
    }
    if submeshes.is_empty() {
        return Ok(None);
    }
    let model = CookedModel {
        bounds,
        has_normals: true,
        has_uvs: false,
        has_colors: true,
        materials,
        lods: vec![CookedLod { error, submeshes }],
    };
    Ok(Some(ClusterMesh { model, error, triangles }))
}

/// Builds the L1 node of one streaming cell from its index placements.
pub fn build_l1(
    partition: Partition,
    cell: IVec3,
    placements: &[IndexedPlacement],
    models: &dyn Fn(&VfsPath) -> Option<FarModel>,
    source: &dyn FarSource,
    settings: &FarBakeSettings,
) -> Result<NodeBlob, FormatError> {
    let key = NodeKey::of_cell(cell);
    let eligible: Vec<&IndexedPlacement> = placements.iter().filter(|p| far_eligible(&p.placement)).collect();
    let center = node_center(key, partition, eligible.iter().map(|p| p.placement.transform.pos));
    let lead = eligible.iter().map(|p| rel(center, p.placement.transform.pos).length()).fold(0.0, f32::max);
    let levels = &settings.levels;
    let budget = levels.error_budget(FIRST_LEVEL, lead);
    let min_radius = levels.min_radius(FIRST_LEVEL, lead);

    let mut cluster = ClusterBuilder::new();
    let mut blends: BTreeMap<([u16; 3], bool), ClusterBuilder> = BTreeMap::new();
    let mut impostor_models: Vec<ImpostorModel> = Vec::new();
    let mut impostors = Vec::new();
    let mut lights = Vec::new();
    // Every light in play glows from afar, with or without a model
    // (adapters may return no `FarLight` for lights that shouldn't).
    for p in placements {
        let excluded = PlacementFlags::EDITOR_ONLY | PlacementFlags::DISABLED;
        if p.placement.class == PlacementClass::Light && !p.placement.flags.intersects(excluded) {
            let t = &p.placement.transform;
            let light = source.light(&p.placement).unwrap_or_default();
            if light.intensity > 0.0 {
                lights.push(light_point(rel(center, t.pos), t.rotation, t.scale, &light));
            }
        }
    }
    // Models of eligible placements this node draws, and those it doesn't
    // (`NodeHeader::unrepresented`: they keep their own range, no handoff).
    let mut drawn: BTreeSet<&str> = BTreeSet::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for p in eligible {
        let t = &p.placement.transform;
        let pivot = rel(center, t.pos);
        let scale = t.scale.abs().max_element();
        let Some(path) = &p.placement.model else { continue };
        // Boundless placements (radius 0: particle-only models) escape size
        // culling, so they would end at the handoff with nothing taking over;
        // smaller-than-proxy ones stay out (size culling ends them first).
        if p.radius <= 0.0 {
            seen.insert(path.as_str());
        }
        if p.radius < min_radius {
            continue;
        }
        seen.insert(path.as_str());
        let Some(model) = models(path) else { continue };
        match model.policy {
            FarPolicy::Drop => {}
            FarPolicy::Impostor => {
                drawn.insert(path.as_str());
                let (c, r) = impostor_sphere(&model.summary);
                let index = impostor_models.iter().position(|m| m.model == path.as_str()).unwrap_or_else(|| {
                    impostor_models.push(ImpostorModel { model: path.as_str().to_owned(), radius: r });
                    impostor_models.len() - 1
                });
                impostors.push(ImpostorInstance {
                    pos: (pivot + t.rotation * (c * t.scale)).to_array(),
                    yaw: yaw_of(t.rotation),
                    scale,
                    model: u16::try_from(index).map_err(|_| FormatError::TooLarge)?,
                });
            }
            FarPolicy::Merge => {
                let Some(cooked) = &model.cooked else { continue };
                // LOD0, so the merged simplification gets the whole budget.
                // Measured on Morrowind (2026-10-11): starting from the
                // coarsest cooked level within half the budget left 6.2M L1
                // triangles, LOD0 4.1M, at the same bake time.
                let Some(lod) = cooked.lods.first() else { continue };
                let to_node = Affine3A::from_scale_rotation_translation(t.scale, t.rotation, pivot);
                for sub in &lod.submeshes {
                    let material = cooked.materials.get(sub.material as usize).cloned().unwrap_or_default();
                    match material.alpha {
                        AlphaMode::Opaque => {
                            let (v, i) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
                            let albedo = model
                                .albedo
                                .get(sub.material as usize)
                                .map_or(material.base_color, |a| [a[0], a[1], a[2], material.base_color[3]]);
                            cluster.add(&v, &i, to_node, albedo, lod.error * scale);
                            drawn.insert(path.as_str());
                        }
                        AlphaMode::Blend => {
                            // B5a: keep alpha-blended parts in a separate proxy submesh
                            // grouped by quantised emissive and double-sidedness.
                            let (v, i) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
                            let (i, folded) = fold_back_faces(&v, &i);
                            let tex_albedo = model.albedo.get(sub.material as usize);
                            let albedo = tex_albedo
                                .map_or(material.base_color, |a| [a[0], a[1], a[2], material.base_color[3] * a[3]]);
                            let emissive = blend_emissive(&material, tex_albedo);
                            let group = blends.entry(blend_key(emissive, material.double_sided || folded > 0)).or_default();
                            group.add(&v, &i, to_node, albedo, lod.error * scale);
                            drawn.insert(path.as_str());
                        }
                        // Phase 1: no far textures, so alpha-tested parts (leaf cards) stay out of
                        // the cluster mesh (tall foliage goes through impostors).
                        AlphaMode::Mask(_) => {}
                    }
                }
            }
        }
    }
    let mesh = assemble_cluster_mesh(cluster, blends, budget, &settings.simplify)?;
    let unrepresented = seen.difference(&drawn).map(|m| (*m).to_owned()).collect();
    let mut blob = finish(key, center, lead, mesh, impostor_models, impostors, lights);
    blob.header.unrepresented = unrepresented;
    Ok(blob)
}

fn light_point(pivot: Vec3, rotation: Quat, scale: Vec3, light: &FarLight) -> LightPoint {
    LightPoint {
        pos: (pivot + rotation * (Vec3::from_array(light.offset) * scale)).to_array(),
        color: light.color,
        intensity: light.intensity,
        radius: light.radius,
    }
}

fn finish(
    key: NodeKey,
    center: sf_coords::WorldPos,
    lead: f32,
    mesh: Option<ClusterMesh>,
    impostor_models: Vec<ImpostorModel>,
    impostors: Vec<ImpostorInstance>,
    lights: Vec<LightPoint>,
) -> NodeBlob {
    let (mut min, mut max) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    if let Some(m) = &mesh
        && !m.model.bounds.is_empty()
    {
        min = min.min(Vec3::from_array(m.model.bounds.min));
        max = max.max(Vec3::from_array(m.model.bounds.max));
    }
    for i in &impostors {
        let r = impostor_models[usize::from(i.model)].radius * i.scale;
        min = min.min(Vec3::from_array(i.pos) - r);
        max = max.max(Vec3::from_array(i.pos) + r);
    }
    for l in &lights {
        min = min.min(Vec3::from_array(l.pos));
        max = max.max(Vec3::from_array(l.pos));
    }
    let header = NodeHeader {
        key,
        center: PosRepr::from(center),
        min: min.to_array(),
        max: max.to_array(),
        error: mesh.as_ref().map_or(0.0, |m| m.error),
        lead,
        triangles: mesh.as_ref().map_or(0, |m| m.triangles),
        instances: u32::try_from(impostors.len()).unwrap_or(u32::MAX),
        lights: u32::try_from(lights.len()).unwrap_or(u32::MAX),
        impostor_models,
        unrepresented: Vec::new(),
    };
    NodeBlob { header, mesh: mesh.map(|m| m.model), impostors, lights }
}

/// Builds an L2 or L3 node from its children's blobs (any order; sorted
/// here by key for determinism).
pub fn build_parent(
    key: NodeKey,
    partition: Partition,
    mut children: Vec<NodeBlob>,
    settings: &FarBakeSettings,
) -> Result<NodeBlob, FormatError> {
    children.sort_by_key(|c| c.header.key);
    let center = node_center(key, partition, children.iter().map(|c| c.header.center()));
    let lead = children.iter().map(|c| rel(center, c.header.center()).length()).fold(0.0, f32::max);
    let levels = &settings.levels;
    let budget = levels.error_budget(key.level, lead);
    let min_radius = levels.min_radius(key.level, lead);
    let simplify = settings.simplify;

    let mut cluster = ClusterBuilder::new();
    let mut blends: BTreeMap<([u16; 3], bool), ClusterBuilder> = BTreeMap::new();
    let mut models: Vec<ImpostorModel> = Vec::new();
    let mut impostors = Vec::new();
    let mut lights: BTreeMap<[i64; 3], (DVec3, DVec3, f64, f32)> = BTreeMap::new();
    let merge = f64::from(settings.light_merge[usize::from(key.level.clamp(1, 3) - 1)]);
    let keep_blended = key.level < 3;
    for child in &children {
        let offset = rel(center, child.header.center());
        if let Some(mesh) = &child.mesh
            && let Some(lod) = mesh.lods.first()
        {
            for sub in &lod.submeshes {
                let mat = mesh.materials.get(sub.material as usize);
                let alpha = mat.map_or(AlphaMode::Opaque, |m| m.alpha);
                match alpha {
                    AlphaMode::Opaque => {
                        let (v, i) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
                        cluster.add(&v, &i, Affine3A::from_translation(offset), [1.0; 4], child.header.error);
                    }
                    AlphaMode::Blend if keep_blended => {
                        let (v, i) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
                        let (emissive, double_sided) = mat.map_or(([0.0; 3], false), |m| (m.emissive, m.double_sided));
                        let group = blends.entry(blend_key(emissive, double_sided)).or_default();
                        group.add(&v, &i, Affine3A::from_translation(offset), [1.0; 4], child.header.error);
                    }
                    AlphaMode::Blend | AlphaMode::Mask(_) => {}
                }
            }
        }
        for inst in &child.impostors {
            let m = &child.header.impostor_models[usize::from(inst.model)];
            if m.radius * inst.scale < min_radius {
                continue;
            }
            let index = models.iter().position(|x| x.model == m.model).unwrap_or_else(|| {
                models.push(m.clone());
                models.len() - 1
            });
            impostors.push(ImpostorInstance {
                pos: (Vec3::from_array(inst.pos) + offset).to_array(),
                model: u16::try_from(index).map_err(|_| FormatError::TooLarge)?,
                ..*inst
            });
        }
        for l in &child.lights {
            let p = (Vec3::from_array(l.pos) + offset).as_dvec3();
            #[allow(clippy::cast_possible_truncation)]
            let cell = if merge > 0.0 { (p / merge).floor().as_i64vec3().to_array() } else { [i64::try_from(lights.len()).unwrap_or(i64::MAX), 0, 0] };
            let w = f64::from(l.intensity.max(1e-3));
            let e = lights.entry(cell).or_insert((DVec3::ZERO, DVec3::ZERO, 0.0, 0.0));
            e.0 += p * w;
            e.1 += Vec3::from_array(l.color).as_dvec3() * w;
            e.2 += w;
            e.3 = e.3.max(l.radius);
        }
    }
    #[allow(clippy::cast_possible_truncation)]
    let lights = lights
        .into_values()
        .map(|(p, c, w, r)| LightPoint {
            pos: (p / w).as_vec3().to_array(),
            color: (c / w).as_vec3().to_array(),
            intensity: w as f32,
            radius: r,
        })
        .collect();
    let mesh = assemble_cluster_mesh(cluster, blends, budget, &simplify)?;
    Ok(finish(key, center, lead, mesh, models, impostors, lights))
}

/// The L1 cells (from a list of streaming cells) under a node, and the
/// child nodes one level down, sorted.
pub fn children_of(key: NodeKey, cells: &[IVec3]) -> Vec<NodeKey> {
    let mut out: Vec<NodeKey> = cells
        .iter()
        .filter(|c| key.contains_cell(**c))
        .map(|c| NodeKey::containing(*c, key.level - 1))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Every node of `level` that has at least one cell, sorted.
pub fn nodes_of(level: u8, cells: &[IVec3]) -> Vec<NodeKey> {
    let mut out: Vec<NodeKey> = cells.iter().map(|c| NodeKey::containing(*c, level)).collect();
    out.sort();
    out.dedup();
    out
}
