//! Skins (`NiSkinInstance` + `NiSkinData`) and geometry morphs
//! (`NiGeomMorpherController` + `NiMorphData`) as engine-space data.
//!
//! The converter keeps baking skinned shapes into their rest pose
//! ([`crate::Mesh::positions`], what the runtime NIF path draws); this module
//! adds what a deforming renderer needs on top: bind-space vertices, the
//! bones (as node indices) with inverse bind matrices, per-vertex influences,
//! and morph target deltas with their weight curves.
//!
//! Gamebryo skins a vertex `v` of a shape `G` bound to the skeleton root `R` as
//! `world(G) · S · world(R)⁻¹ · Σ wᵢ · world(Bᵢ) · Dᵢ · v`, where `S` is
//! `NiSkinData`'s overall transform and `Dᵢ` its per-bone transform (the
//! skin → bone "inverse bind"); `OpenMW`'s `RigGeometry` and `tes3`'s
//! `apply_skin_deforms` compose the same chain. The glTF/Bevy form is
//! `Σ wᵢ · Jᵢ · IBᵢ · p` with joint model transforms `Jᵢ`. With
//! `M = world(G) · S · world(R)⁻¹` (identity whenever the shape and the
//! skeleton root sit where they were bound, the normal case),
//! `IBᵢ = Jᵢ⁻¹ · M · Jᵢ · Dᵢ` and `p = v` reproduce the rest pose exactly and
//! every pose exactly when `M` is the identity. Spec: `docs/specs/nif-runtime.md`
//! ("Skins and geometry morphs").

use tes3::nif::glam::{Affine3A, Vec3};
use tes3::nif::{NiGeomMorpherController, NiGeometry, NiKey, NiLink, NiMorphData, NiSkinData, NiSkinInstance, NiStream};

use crate::anim::{self, Curve, Timing};
use crate::coords;

/// The skin of one mesh, in engine space.
#[derive(Clone, Debug, Default)]
pub struct Skin {
    /// Bind-space positions (the shape's own vertex data): skinning input.
    pub positions: Vec<[f32; 3]>,
    /// Bind-space normals; empty when the shape has none.
    pub normals: Vec<[f32; 3]>,
    /// Bones that resolved to nodes, indexed by [`Self::influences`].
    pub bones: Vec<SkinBone>,
    /// Per vertex, `(bone index, weight)` pairs as authored: any number
    /// (vanilla content uses up to 4 per vertex in most shapes, more in a
    /// few), weights not renormalized.
    pub influences: Vec<Vec<(u16, f32)>>,
    /// The skin's `M` (see the module docs) was not the identity: poses other
    /// than the rest pose are approximate.
    pub offset_bind: bool,
}

/// One bone of a [`Skin`].
#[derive(Clone, Copy, Debug)]
pub struct SkinBone {
    /// Index into [`crate::Model::nodes`].
    pub node: usize,
    /// Bind space → the bone's space, so that
    /// `world(node) · inverse_bind · position` is the rest pose.
    pub inverse_bind: Affine3A,
}

/// Geometry morphs of one mesh (`NiGeomMorpherController`). The mesh's
/// positions (and its skin's bind positions) are the base, target 0.
#[derive(Clone, Debug, Default)]
pub struct Morph {
    pub timing: Timing,
    /// Targets 1.. of the morph data (target 0 is the base).
    pub targets: Vec<MorphTarget>,
}

/// One morph target: dense per-vertex position deltas and the weight curve.
#[derive(Clone, Debug, Default)]
pub struct MorphTarget {
    /// Engine-space deltas (meters), in the mesh's own space.
    pub deltas: Vec<[f32; 3]>,
    pub weight: Curve<f32>,
}

impl Morph {
    /// Weight of each target at the controller's start time (the pose shown
    /// while nothing drives it).
    pub fn rest_weights(&self) -> Vec<f32> {
        self.targets.iter().map(|t| t.weight.sample(self.timing.start).unwrap_or(0.0)).collect()
    }
}

/// A skinned shape's data before `apply_skin_deforms` replaces it.
#[derive(Clone, Debug)]
pub(crate) struct RawSkin {
    pub skin: NiLink<NiSkinInstance>,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
}

/// Raw skin data of every skinned shape, keyed by the shape.
pub(crate) fn raw_skins(stream: &NiStream) -> std::collections::HashMap<NiKey, RawSkin> {
    stream
        .objects_of_type_with_link::<NiGeometry>()
        .filter(|(_, g)| !g.skin_instance.is_null())
        .filter_map(|(link, g)| {
            let data = stream.get(g.geometry_data)?;
            Some((link.key, RawSkin { skin: g.skin_instance, vertices: data.vertices.clone(), normals: data.normals.clone() }))
        })
        .collect()
}

/// Resolves a raw skin against the converted model. `node_of` maps NIF
/// objects to node indices, `world` holds the model-space rest transforms,
/// `shape` is the mesh's node. `None` when the skin data or its root is
/// missing (the mesh then keeps only its baked rest pose).
pub(crate) fn resolve(
    stream: &NiStream,
    raw: &RawSkin,
    base: Option<&[Vec3]>,
    shape: usize,
    world: &[Affine3A],
    node_of: impl Fn(NiKey) -> Option<usize>,
) -> Option<Skin> {
    let instance = stream.get(raw.skin)?;
    let data: &NiSkinData = stream.get(instance.data)?;
    let root = node_of(instance.root.key)?;
    let vertices = base.unwrap_or(&raw.vertices);
    let offset = world[shape] * coords::affine(data.transform()) * world[root].inverse();
    let mut skin = Skin {
        positions: vertices.iter().map(|&v| coords::point(v).to_array()).collect(),
        normals: if raw.normals.len() == vertices.len() {
            raw.normals.iter().map(|&n| coords::direction(n).normalize_or_zero().to_array()).collect()
        } else {
            Vec::new()
        },
        bones: Vec::new(),
        influences: vec![Vec::new(); vertices.len()],
        offset_bind: !offset.abs_diff_eq(Affine3A::IDENTITY, 1e-4),
    };
    for (bone, bone_data) in instance.bones.iter().zip(&data.bone_data) {
        let Some(node) = node_of(bone.key) else { continue };
        let Ok(slot) = u16::try_from(skin.bones.len()) else { break };
        let joint = world[node];
        let inverse_bind = joint.inverse() * offset * joint * coords::affine(bone_data.transform());
        skin.bones.push(SkinBone { node, inverse_bind });
        for &(vertex, weight) in &bone_data.vertex_weights {
            if let Some(list) = skin.influences.get_mut(usize::from(vertex)) {
                list.push((slot, weight));
            }
        }
    }
    Some(skin)
}

/// The geometry morphs on a shape, if any, and the base positions they
/// imply (morph target 0) in NIF space.
pub(crate) fn morph(stream: &NiStream, geometry: &NiGeometry, vertex_count: usize) -> Option<(Morph, Vec<Vec3>)> {
    let controller = geometry.base.base.controllers_of_type::<NiGeomMorpherController>(stream).next()?;
    let data: &NiMorphData = stream.get(controller.base.data)?;
    let base = data.targets.first().filter(|t| t.vertices.len() == vertex_count)?.vertices.clone();
    let targets = data.targets[1..]
        .iter()
        .filter(|t| t.vertices.len() == vertex_count)
        .map(|t| MorphTarget {
            deltas: t
                .vertices
                .iter()
                .zip(&base)
                .map(|(&v, &b)| coords::point(if data.relative_targets { v } else { v - b }).to_array())
                .collect(),
            weight: anim::float_curve(&t.keys),
        })
        .collect();
    let mut timing = Timing::from_controller(&controller.base.base);
    if timing.stop <= timing.start
        && let Some((start, stop)) = morph_key_range(data)
    {
        timing.start = start;
        timing.stop = stop;
    }
    Some((Morph { timing, targets }, base))
}

fn morph_key_range(data: &NiMorphData) -> Option<(f32, f32)> {
    let ranges = data.targets.iter().filter_map(|t| anim::float_curve(&t.keys).time_range());
    ranges.reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))
}

impl Skin {
    /// Model-space rest pose of every vertex, given the model's rest
    /// transforms ([`crate::world_transforms`]): `Σ w · world(bone) ·
    /// inverse_bind · p`. Equals the baked [`crate::Mesh::positions`] placed by
    /// the mesh's node.
    pub fn rest_positions(&self, world: &[Affine3A]) -> Vec<Vec3> {
        self.positions
            .iter()
            .zip(&self.influences)
            .map(|(&p, influences)| {
                let p = Vec3::from_array(p);
                influences
                    .iter()
                    .map(|&(bone, weight)| {
                        let bone = &self.bones[usize::from(bone)];
                        (world[bone.node] * bone.inverse_bind).transform_point3(p) * weight
                    })
                    .sum()
            })
            .collect()
    }
}
