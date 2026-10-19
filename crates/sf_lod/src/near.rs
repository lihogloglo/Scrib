//! Near-cell precombine (precombine.md): a streaming cell's eligible statics
//! merged per material and LOD level into a few cooked models in cell space,
//! one per size bucket and vertex-attribute set. Unlike the far field's L1
//! (simplified, vertex-colored), nothing is simplified or re-textured: the
//! merged model draws exactly what the separate objects drew, as a handful of
//! entities instead of one per object, LOD level and material part.
//! Bevy-free; the `lod.near` bake step wraps [`build_near`].

use std::collections::BTreeMap;
use std::sync::Arc;

use glam::{Affine3A, IVec3, Mat3, Vec3};
use serde::{Deserialize, Serialize};
use sf_model::{Aabb, AlphaMode, CookedLod, CookedModel, CookedSubmesh, Material, Vertex};
use sf_vfs::VfsPath;
use sf_world::{IndexedPlacement, Partition, Placement, PlacementClass, PlacementFlags, PlacementId};

use crate::FormatError;
use crate::build::node_center;
use crate::cluster::encode_submesh;
use crate::node::NodeKey;

/// Bucket edges by placement radius, meters: `[0, a)`, `[a, b)`, `[b, c)`, `[c, ∞)`.
/// Size culling gives each bucket one draw distance (its biggest member's),
/// so small clutter keeps its short range (industry-audit.md J1).
pub const BUCKET_EDGES: [f32; 3] = [0.5, 2.0, 8.0];

/// One merged model of a cell: members of one size class and attribute set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NearBucket {
    /// Largest member radius (its draw distance is the bucket's).
    pub max_radius: f32,
    /// Distance from the cell center to the farthest member point (LOD0
    /// bounds): the runtime adds it to every switch distance, so no member
    /// coarsens earlier than it would alone.
    pub reach: f32,
    /// Members, by placement id (sorted): the runtime spawns no visuals for them.
    pub members: Vec<PlacementId>,
    /// The merged model, positions relative to [`NearCombine::center`].
    pub model: CookedModel,
}

/// A cell's precombine (`lod.near` product).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NearCombine {
    /// The buckets' shared origin (world, f64): the far field's L1 node center.
    pub center: [f64; 3],
    pub buckets: Vec<NearBucket>,
}

impl NearCombine {
    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("precombine serializes")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }

    /// Placements the buckets draw.
    pub fn member_count(&self) -> usize {
        self.buckets.iter().map(|b| b.members.len()).sum()
    }
}

/// Whether a placement may join a precombine (before the adapter's veto):
/// a plain static with a cooked model whose materials are all opaque or
/// alpha-masked (sorted translucents stay separate entities).
pub fn near_eligible(p: &Placement, model: &CookedModel) -> bool {
    let barred = PlacementFlags::NO_LOD | PlacementFlags::NEAR_ONLY | PlacementFlags::EDITOR_ONLY | PlacementFlags::DISABLED;
    p.class == PlacementClass::Static
        && p.model.is_some()
        && !p.flags.intersects(barred)
        && !model.lods.is_empty()
        && model.materials.iter().all(|m| matches!(m.alpha, AlphaMode::Opaque | AlphaMode::Mask(_)))
}

/// The size class of a radius.
pub fn bucket_of(radius: f32) -> usize {
    BUCKET_EDGES.iter().take_while(|&&e| radius >= e).count()
}

/// Merged geometry of one level, per material.
#[derive(Default)]
struct LevelBuffers {
    error: f32,
    by_material: BTreeMap<u32, (Vec<Vertex>, Vec<u32>)>,
}

/// One bucket being built.
struct BucketBuilder {
    max_radius: f32,
    members: Vec<PlacementId>,
    materials: Vec<Material>,
    levels: Vec<LevelBuffers>,
    signature: (bool, bool, bool),
}

impl BucketBuilder {
    fn material_index(&mut self, m: &Material) -> u32 {
        let i = self.materials.iter().position(|x| x == m).unwrap_or_else(|| {
            self.materials.push(m.clone());
            self.materials.len() - 1
        });
        u32::try_from(i).expect("materials fit u32")
    }

    fn add(&mut self, placement: &IndexedPlacement, model: &CookedModel, to_cell: Affine3A) -> Result<(), FormatError> {
        let scale = placement.placement.transform.scale.abs().max_element();
        let normal_m = Mat3::from(to_cell.matrix3).inverse().transpose();
        let remap: Vec<u32> = model.materials.iter().map(|m| self.material_index(m)).collect();
        let fallback = self.material_index(&Material::default());
        let levels = model.lods.len().max(self.levels.len());
        while self.levels.len() < levels {
            // A new level starts as a copy of every earlier member's coarsest.
            let copy = self.levels.last().map(|l| LevelBuffers { error: l.error, by_material: l.by_material.clone() });
            self.levels.push(copy.unwrap_or_default());
        }
        for (k, level) in self.levels.iter_mut().enumerate() {
            let lod: &CookedLod = &model.lods[k.min(model.lods.len() - 1)];
            level.error = level.error.max(lod.error * scale);
            for sub in &lod.submeshes {
                let index = remap.get(sub.material as usize).copied().unwrap_or(fallback);
                append(level.by_material.entry(index).or_default(), sub, to_cell, normal_m)?;
            }
        }
        self.max_radius = self.max_radius.max(placement.radius);
        self.members.push(placement.placement.id);
        Ok(())
    }

    fn finish(mut self) -> Result<NearBucket, FormatError> {
        let (has_normals, has_uvs, has_colors) = self.signature;
        let mut bounds = Aabb { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] };
        if let Some(first) = self.levels.first() {
            for (v, _) in first.by_material.values() {
                for p in v {
                    for a in 0..3 {
                        bounds.min[a] = bounds.min[a].min(p.position[a]);
                        bounds.max[a] = bounds.max[a].max(p.position[a]);
                    }
                }
            }
        }
        let reach = Vec3::from_array(bounds.min).abs().max(Vec3::from_array(bounds.max).abs()).length();
        let mut lods = Vec::with_capacity(self.levels.len());
        for level in &self.levels {
            let submeshes: Vec<CookedSubmesh> =
                level.by_material.iter().map(|(&m, (v, i))| encode_submesh(m, v, i)).collect::<Result<_, _>>()?;
            lods.push(CookedLod { error: level.error, submeshes });
        }
        self.members.sort_unstable();
        Ok(NearBucket {
            max_radius: self.max_radius,
            reach,
            members: self.members,
            model: CookedModel { bounds, has_normals, has_uvs, has_colors, materials: self.materials, lods },
        })
    }
}

/// Appends a member's submesh, transformed into cell space.
fn append(into: &mut (Vec<Vertex>, Vec<u32>), sub: &CookedSubmesh, to_cell: Affine3A, normal_m: Mat3) -> Result<(), FormatError> {
    let (vertices, indices) = sub.decode().map_err(|e| FormatError::Mesh(e.to_string()))?;
    let base = u32::try_from(into.0.len()).map_err(|_| FormatError::Mesh("precombine too large".into()))?;
    into.0.extend(vertices.iter().map(|v| {
        let n = (normal_m * Vec3::from_array(v.normal)).normalize_or_zero();
        Vertex { position: to_cell.transform_point3(Vec3::from_array(v.position)).to_array(), normal: n.to_array(), ..*v }
    }));
    into.1.extend(indices.iter().map(|i| base + i));
    Ok(())
}

/// Builds the precombine of one streaming cell from its index placements.
/// `models` resolves a cooked model (`None`: not cooked, skipped, skinned);
/// `exclude` is the content pack's veto (e.g. statics a script can disable).
/// `None` when no placement qualifies.
pub fn build_near(
    partition: Partition,
    cell: IVec3,
    placements: &[IndexedPlacement],
    models: &dyn Fn(&VfsPath) -> Option<Arc<CookedModel>>,
    exclude: &dyn Fn(&Placement) -> bool,
) -> Result<Option<NearCombine>, FormatError> {
    let mut members: Vec<(&IndexedPlacement, Arc<CookedModel>)> = placements
        .iter()
        .filter_map(|p| {
            let model = models(p.placement.model.as_ref()?)?;
            (near_eligible(&p.placement, &model) && !exclude(&p.placement)).then_some((p, model))
        })
        .collect();
    if members.is_empty() {
        return Ok(None);
    }
    members.sort_by_key(|(p, _)| p.placement.id);
    let center = node_center(NodeKey::of_cell(cell), partition, members.iter().map(|(p, _)| p.placement.transform.pos));
    // Bucket key: size class, then the attribute set (a member without vertex
    // colors must not join a colored mesh: its zeros would draw black).
    let mut buckets: BTreeMap<(usize, (bool, bool, bool)), BucketBuilder> = BTreeMap::new();
    for (p, model) in &members {
        let t = &p.placement.transform;
        let to_cell = Affine3A::from_scale_rotation_translation(t.scale, t.rotation, center.delta_f32(t.pos));
        let signature = (model.has_normals, model.has_uvs, model.has_colors);
        let b = buckets.entry((bucket_of(p.radius), signature)).or_insert_with(|| BucketBuilder {
            max_radius: 0.0,
            members: Vec::new(),
            materials: Vec::new(),
            levels: Vec::new(),
            signature,
        });
        b.add(p, model, to_cell)?;
    }
    let buckets = buckets.into_values().map(BucketBuilder::finish).collect::<Result<Vec<_>, _>>()?;
    Ok(Some(NearCombine { center: center.to_f64().to_array(), buckets }))
}
