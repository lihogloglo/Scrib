//! Engine-neutral models and mesh cooking.
//!
//! Importers (the framework's own, or an adapter's for a legacy format)
//! produce a [`Model`]: a node tree with meshes and material descriptions,
//! in engine space (meters, Y up). The bake turns models into
//! [`CookedModel`]s (`.sfmesh`): merged per material, with a LOD chain made
//! by meshoptimizer, so the runtime never parses source formats and
//! decodes geometry at GB/s.
//!
//! Spec: `docs/specs/bake.md` ("Cooked meshes and LODs").

mod cook;
mod cooked;
mod lod;
mod skin;
#[cfg(feature = "gltf")]
mod gltf_import;
pub mod steps;

pub use cook::{CookError, CookSettings, cook, cook_rest, cook_rig};
pub use cooked::{Cooked, CookedLod, CookedModel, CookedSubmesh, FormatError, Vertex};
pub use skin::{
    CookedJoint, CookedPart, CookedSkinStreams, CookedSkinned, DecodedSubmesh, MAX_INFLUENCES, MORPH_EPSILON, MorphDelta,
    MorphTargetInfo, RUNTIME_MORPH_LODS, SkinVertex,
};
pub use lod::{LodRange, LodScreen, lod_ranges};
#[cfg(feature = "gltf")]
pub use gltf_import::GltfImporter;

use std::collections::BTreeMap;

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

/// A model as imported: nodes, meshes, materials.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub meshes: Vec<MeshData>,
    pub materials: Vec<Material>,
}

/// One node of the model's scene graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    /// Index into [`Model::nodes`]; parents need not come first.
    pub parent: Option<u32>,
    pub translation: [f32; 3],
    /// Quaternion `[x, y, z, w]`.
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    /// Index into [`Model::meshes`].
    pub mesh: Option<u32>,
    pub lod: LodPolicy,
    /// Moves at runtime (animated transform, billboard, particle system, …).
    /// Models with animated nodes keep the runtime path for now.
    pub animated: bool,
}

impl Node {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            translation: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
            mesh: None,
            lod: LodPolicy::Simplify,
            animated: false,
        }
    }

    pub fn local_transform(&self) -> Affine3A {
        Affine3A::from_scale_rotation_translation(
            Vec3::from_array(self.scale),
            Quat::from_array(self.rotation).normalize(),
            Vec3::from_array(self.translation),
        )
    }
}

/// How a node takes part in LOD generation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LodPolicy {
    /// Simplified with the rest of the model.
    #[default]
    Simplify,
    /// Kept unsimplified in every level (its vertices are locked).
    Keep,
    /// Present in LOD0 only (small details that vanish at distance).
    Drop,
}

/// Triangle geometry with optional attributes, all per vertex.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub uvs: Option<Vec<[f32; 2]>>,
    /// Linear RGBA, 0..=1.
    pub colors: Option<Vec<[f32; 4]>>,
    /// Skinned meshes deform at runtime and are never cooked statically.
    /// Importers may set this before they can read the mesh's [`Self::skin`]:
    /// a skinned mesh without skin data makes the cook skip the model.
    pub skinned: bool,
    /// Triangle list.
    pub indices: Vec<u32>,
    /// Index into [`Model::materials`].
    pub material: u32,
    /// Joint influences (implies [`Self::skinned`]). Positions and normals of
    /// a skinned mesh are in bind space and its node's transform is ignored
    /// (the glTF rule): the joints alone place it.
    pub skin: Option<MeshSkin>,
    /// Morph targets (blend shapes): per-vertex deltas, applied before skinning.
    pub morph_targets: Vec<MorphTarget>,
}

impl MeshData {
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Skinned, with or without skin data.
    pub fn is_skinned(&self) -> bool {
        self.skinned || self.skin.is_some()
    }

    /// Deforms at runtime (skin or morph targets): cooked as a [`CookedSkinned`].
    pub fn deforms(&self) -> bool {
        self.skin.is_some() || !self.morph_targets.is_empty()
    }
}

/// Per-vertex joint influences of one mesh, and the joints they name.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshSkin {
    /// The mesh's joint palette, indexed by [`Self::influences`].
    pub joints: Vec<SkinJoint>,
    /// Per vertex, any number of `(palette index, weight)` pairs. The cook
    /// keeps the [`MAX_INFLUENCES`] largest and renormalizes them (a second
    /// glTF `JOINTS_1`/`WEIGHTS_1` set just adds pairs).
    pub influences: Vec<Vec<(u16, f32)>>,
}

/// One joint of a mesh skin.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkinJoint {
    /// Index into [`Model::nodes`].
    pub node: u32,
    /// Maps bind-space positions into the joint's space (column-major 4×4,
    /// glTF `inverseBindMatrices`). Skinned position:
    /// `Σ weight · joint_model_transform · inverse_bind · position`.
    pub inverse_bind: [f32; 16],
}

/// One morph target of a mesh: dense deltas, one per vertex (importers
/// expand sparse sources).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MorphTarget {
    /// Targets are matched across a model's meshes by name.
    pub name: String,
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    /// Weight while nothing animates it (glTF `mesh.weights`).
    pub default_weight: f32,
    /// Runtime morph or per-instance shape. Set by the importer (an adapter
    /// knows which of its targets animate); glTF has no such flag, so its
    /// importer leaves the default.
    #[serde(default)]
    pub kind: MorphKind,
}

/// What a morph target is for: the cook and character assembly treat the
/// two differently (bake.md "Morph layout", character.md "Morph-bearing parts").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MorphKind {
    /// Animated at runtime (blink, mouth shapes, visemes): a GPU morph
    /// target, kept on LODs below [`RUNTIME_MORPH_LODS`] only.
    #[default]
    Expression,
    /// A per-instance shape that never animates (face or body sliders): kept
    /// on every LOD and baked into the vertices when a character is
    /// assembled, at the character's weight (else the default weight), so it
    /// never reaches the GPU as a morph target (UE Mutable, `FaceGen`).
    Shape,
}

/// A material description, engine-neutral. The runtime maps it to its own
/// material types; `extras` carries adapter-specific hints it may use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    /// VFS path of the base color texture.
    pub base_color_texture: Option<String>,
    /// Linear RGBA multiplier.
    pub base_color: [f32; 4],
    pub emissive: [f32; 3],
    pub alpha: AlphaMode,
    pub double_sided: bool,
    pub unlit: bool,
    pub extras: BTreeMap<String, String>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::new(),
            base_color_texture: None,
            base_color: [1.0; 4],
            emissive: [0.0; 3],
            alpha: AlphaMode::Opaque,
            double_sided: false,
            unlit: false,
            extras: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum AlphaMode {
    #[default]
    Opaque,
    /// Alpha test with this cutoff.
    Mask(f32),
    Blend,
}

/// Axis-aligned bounds in model space.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    pub const EMPTY: Self = Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] };

    pub fn is_empty(&self) -> bool {
        self.min[0] > self.max[0]
    }

    pub fn grow(&mut self, p: [f32; 3]) {
        for ((lo, hi), v) in self.min.iter_mut().zip(&mut self.max).zip(p) {
            *lo = lo.min(v);
            *hi = hi.max(v);
        }
    }

    /// Half the diagonal: the bounding-sphere radius around the center.
    pub fn radius(&self) -> f32 {
        if self.is_empty() {
            return 0.0;
        }
        (Vec3::from_array(self.max) - Vec3::from_array(self.min)).length() * 0.5
    }
}

/// What streaming and LOD selection need to know about a model without
/// loading it (bake product `model.info`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model-space bounds of everything the model renders.
    pub bounds: Aabb,
    pub triangles: u32,
    pub vertices: u32,
    pub animated: bool,
    pub skinned: bool,
}

impl ModelInfo {
    pub fn of(model: &Model) -> Self {
        let world = model.node_transforms();
        let mut bounds = Aabb::EMPTY;
        let (mut triangles, mut vertices) = (0usize, 0usize);
        for (node, transform) in model.nodes.iter().zip(&world) {
            let Some(mesh) = node.mesh.and_then(|m| model.meshes.get(m as usize)) else { continue };
            // Skinned positions are in bind space already (approximately the rest pose).
            let transform = if mesh.skin.is_some() { &Affine3A::IDENTITY } else { transform };
            for p in &mesh.positions {
                bounds.grow(transform.transform_point3(Vec3::from_array(*p)).to_array());
            }
            triangles += mesh.triangle_count();
            vertices += mesh.positions.len();
        }
        let count = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Self {
            bounds,
            triangles: count(triangles),
            vertices: count(vertices),
            animated: model.nodes.iter().any(|n| n.animated),
            skinned: model.meshes.iter().any(MeshData::is_skinned),
        }
    }
}

impl Model {
    /// Model-space transform of every node (parents composed in).
    pub fn node_transforms(&self) -> Vec<Affine3A> {
        let mut out: Vec<Option<Affine3A>> = vec![None; self.nodes.len()];
        for i in 0..self.nodes.len() {
            self.resolve_transform(i, &mut out, 0);
        }
        out.into_iter().map(|t| t.unwrap_or(Affine3A::IDENTITY)).collect()
    }

    fn resolve_transform(&self, i: usize, out: &mut [Option<Affine3A>], depth: usize) -> Affine3A {
        if let Some(t) = out[i] {
            return t;
        }
        let node = &self.nodes[i];
        let local = node.local_transform();
        // A malformed parent chain (cycle, bad index) is treated as a root.
        let parent = match node.parent {
            Some(p) if (p as usize) < self.nodes.len() && depth < self.nodes.len() => {
                self.resolve_transform(p as usize, out, depth + 1)
            }
            _ => Affine3A::IDENTITY,
        };
        let t = parent * local;
        out[i] = Some(t);
        t
    }
}

/// Turns a source file into a [`Model`]. Implemented by adapters for
/// legacy formats; the framework ships its own for modern ones.
pub trait ModelImporter: Send + Sync {
    /// Lowercase extensions without the dot, e.g. `["nif"]`.
    fn extensions(&self) -> &[&str];
    /// Bump when the importer's output changes, to invalidate cooked products.
    fn version(&self) -> u32;
    fn import(&self, path: &str, bytes: &[u8]) -> Result<Model, ImportError>;
}

#[derive(Debug, thiserror::Error)]
#[error("{path}: {message}")]
pub struct ImportError {
    pub path: String,
    pub message: String,
}

#[cfg(test)]
mod tests;
