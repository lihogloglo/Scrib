//! Outfit assembly, Bevy-free: the visible parts' cooked meshes become one
//! skinned mesh per material, plus one per **morph-bearing** submesh, all on
//! one shared joint palette (character.md "Assembly at runtime" and
//! "Morph-bearing parts").
//!
//! The morph rule (gap G1 in industry-audit.md): Bevy 0.20 stores morph
//! targets densely, 48 bytes per vertex of the *whole mesh* per target, and
//! loops over every target per vertex in every pass, and UE's mesh merge
//! drops morphs for the same reason. So:
//! - [`MorphKind::Shape`](sf_model::MorphKind::Shape) targets (per-character sliders that never animate)
//!   are baked into the vertices here, at the recipe's [`ShapeWeights`]
//!   (else their default weight), as UE Mutable and `FaceGen` bake them;
//! - a submesh that still carries [`MorphKind::Expression`](sf_model::MorphKind::Expression) targets (blink,
//!   mouth) is never merged: it becomes its own mesh with only its own
//!   targets, on the same palette (one extra draw and palette copy);
//! - everything else merges per material and carries no morph data.
//!
//! `sf_character_bevy` turns the result into `Mesh` assets and one
//! `SkinnedMesh` entity per mesh, sharing the joint entities.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sf_model::{CookedJoint, CookedSkinned, DecodedSubmesh, FormatError, Material, MorphDelta, MorphTargetInfo, SkinVertex, Vertex};

use crate::records::BodyPart;

/// Bytes Bevy 0.20 stores per vertex per morph target (`MorphAttributes`:
/// position, normal and tangent deltas, each padded to 16 bytes).
pub const BEVY_MORPH_BYTES: usize = 48;
/// Bytes per joint of a `SkinnedMesh` palette (one `Mat4`; Bevy uploads a
/// palette copy per skinned mesh entity).
pub const BEVY_JOINT_BYTES: usize = 64;

/// Per-character weights of [`MorphKind::Shape`](sf_model::MorphKind::Shape) targets, by target name
/// (face or body sliders). A shape target not named here is baked at its
/// default weight. Part of the merged-mesh cache key ([`Self::key`]).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShapeWeights(pub Vec<(String, f32)>);

impl ShapeWeights {
    pub fn get(&self, name: &str) -> Option<f32> {
        self.0.iter().rev().find(|(n, _)| n == name).map(|(_, w)| *w)
    }

    /// Stable hash of the weights (order-independent, later entries win);
    /// 0 when empty, so characters without sliders share cache entries.
    pub fn key(&self) -> u64 {
        if self.0.is_empty() {
            return 0;
        }
        let mut named: Vec<(&str, u32)> = Vec::new();
        for (name, _) in &self.0 {
            if !named.iter().any(|(n, _)| *n == name.as_str()) {
                named.push((name, self.get(name).unwrap_or(0.0).to_bits()));
            }
        }
        named.sort_unstable();
        let mut hasher = blake3::Hasher::new();
        for (name, bits) in named {
            hasher.update(name.as_bytes()).update(&[0]).update(&bits.to_le_bytes());
        }
        u64::from_le_bytes(hasher.finalize().as_bytes()[..8].try_into().expect("8 bytes"))
    }
}

/// One visible part going into an outfit.
#[derive(Clone, Copy, Debug)]
pub struct AssemblyPart<'a> {
    /// The part's cooked model, already a single-joint skin for rigid
    /// parts (`CookedSkinned::rigid`).
    pub cooked: &'a CookedSkinned,
    /// Keeps only the submeshes whose node path passes
    /// [`BodyPart::keeps_node`]; `None` keeps them all.
    pub body_part: Option<&'a BodyPart>,
    /// The part already decoded and rewritten (smooth bodies: welded
    /// positions and blended weights on a rest-space palette). When set,
    /// its palette and submeshes replace the cooked ones; materials and
    /// morph target infos still come from `cooked`.
    pub decoded: Option<&'a DecodedPart>,
}

impl<'a> AssemblyPart<'a> {
    /// A cooked part as it is.
    pub fn new(cooked: &'a CookedSkinned, body_part: Option<&'a BodyPart>) -> Self {
        Self { cooked, body_part, decoded: None }
    }
}

/// One part's kept submeshes at one LOD, decoded: what an assembly step
/// that rewrites geometry or weights (smooth bodies) hands to [`assemble`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecodedPart {
    /// The palette the submeshes' skin joints index.
    pub joints: Vec<CookedJoint>,
    /// Only the submeshes [`BodyPart::keeps_node`] keeps, in model order.
    pub submeshes: Vec<DecodedSubmesh>,
}

impl DecodedPart {
    /// Decodes the submeshes `body_part` keeps at `lod` (a part with fewer
    /// levels uses its coarsest), in the order [`assemble`] reads them.
    pub fn decode(cooked: &CookedSkinned, lod: usize, body_part: Option<&BodyPart>) -> Result<Self, FormatError> {
        let mut out = Self { joints: cooked.joints.clone(), submeshes: Vec::new() };
        let Some(level) = cooked.model.lods.len().checked_sub(1).map(|last| lod.min(last)) else { return Ok(out) };
        for i in 0..cooked.model.lods[level].submeshes.len() {
            let sub = cooked.decode_submesh(level, i)?;
            let path = cooked.parts.get(sub.part as usize).map_or(&[][..], |p| p.node_path.as_slice());
            if body_part.is_none_or(|b| b.keeps_node(path.iter().map(String::as_str))) {
                out.submeshes.push(sub);
            }
        }
        Ok(out)
    }
}

/// One mesh of an assembled outfit.
#[derive(Clone, Debug, PartialEq)]
pub struct AssembledMesh {
    pub material: Material,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    /// One per vertex; joints index [`AssembledOutfit::joints`].
    pub skin: Vec<SkinVertex>,
    /// Runtime (expression) targets only; empty for merged meshes.
    pub morph_targets: Vec<MorphTargetInfo>,
    /// Target-major, one block of `vertices.len()` per entry of `morph_targets`.
    pub morphs: Vec<MorphDelta>,
}

/// Estimated GPU bytes of an outfit in Bevy 0.20's layouts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OutfitBytes {
    /// Vertex attributes: position, normal, uv (32 B) plus joint indices and
    /// weights (24 B), plus 16 B of color when present.
    pub vertices: usize,
    pub indices: usize,
    /// [`BEVY_MORPH_BYTES`] per vertex per target of each mesh.
    pub morphs: usize,
    /// One palette copy per mesh.
    pub palettes: usize,
}

impl OutfitBytes {
    pub fn total(&self) -> usize {
        self.vertices + self.indices + self.morphs + self.palettes
    }
}

/// An outfit ready for the renderer: what the merged-mesh cache holds,
/// keyed by (skeleton, outfit key, LOD, [`ShapeWeights::key`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AssembledOutfit {
    /// The shared palette: joint names (bound to skeleton entities by
    /// name) with inverse binds, deduplicated by (name, inverse bind).
    pub joints: Vec<CookedJoint>,
    /// One per material, never morphing.
    pub merged: Vec<AssembledMesh>,
    /// One per submesh that keeps expression targets.
    pub morphed: Vec<AssembledMesh>,
    /// Whether any part has colors (adds a color attribute).
    pub has_colors: bool,
}

impl AssembledOutfit {
    pub fn meshes(&self) -> impl Iterator<Item = &AssembledMesh> {
        self.merged.iter().chain(&self.morphed)
    }

    pub fn gpu_bytes(&self) -> OutfitBytes {
        let per_vertex = 56 + if self.has_colors { 16 } else { 0 };
        let mut out = OutfitBytes::default();
        for mesh in self.meshes() {
            out.vertices += mesh.vertices.len() * per_vertex;
            out.indices += mesh.indices.len() * 4;
            out.morphs += mesh.vertices.len() * mesh.morph_targets.len() * BEVY_MORPH_BYTES;
            out.palettes += self.joints.len() * BEVY_JOINT_BYTES;
        }
        out
    }
}

/// Assembles the visible parts at one LOD (a part with fewer levels uses
/// its coarsest). Parts and submeshes keep their order, so merged vertices
/// are the parts' vertices concatenated.
pub fn assemble(parts: &[AssemblyPart<'_>], lod: usize, shapes: &ShapeWeights) -> Result<AssembledOutfit, FormatError> {
    let mut out = AssembledOutfit::default();
    let mut palette: HashMap<(String, [u32; 16]), u16> = HashMap::new();
    for part in parts {
        let cooked = part.cooked;
        let Some(level) = cooked.model.lods.len().checked_sub(1).map(|last| lod.min(last)) else { continue };
        out.has_colors |= cooked.model.has_colors;
        let decoded = match part.decoded {
            Some(d) => std::borrow::Cow::Borrowed(d),
            None => std::borrow::Cow::Owned(DecodedPart::decode(cooked, level, part.body_part)?),
        };
        // This part's palette slots in the outfit palette.
        let mut slots = Vec::with_capacity(decoded.joints.len());
        for joint in &decoded.joints {
            let key = (joint.name.clone(), joint.inverse_bind.map(f32::to_bits));
            let next = u16::try_from(out.joints.len()).map_err(|_| FormatError::Body("outfit palette over 65536 joints".into()))?;
            slots.push(*palette.entry(key).or_insert_with(|| {
                out.joints.push(joint.clone());
                next
            }));
        }
        for mut sub in decoded.into_owned().submeshes {
            let infos = &cooked.morph_targets;
            sub.bake_shapes(infos, |t| shapes.get(&infos[t].name).unwrap_or(infos[t].default_weight));
            for s in &mut sub.skin {
                for j in &mut s.joints {
                    *j = slots[usize::from(*j)];
                }
            }
            let material = cooked.model.materials.get(sub.material as usize).cloned().unwrap_or_default();
            if sub.targets.is_empty() {
                let k = out.merged.iter().position(|m| m.material == material).unwrap_or_else(|| {
                    out.merged.push(AssembledMesh::empty(material.clone()));
                    out.merged.len() - 1
                });
                let mesh = &mut out.merged[k];
                let base = u32::try_from(mesh.vertices.len()).map_err(|_| FormatError::TooLarge)?;
                mesh.indices.extend(sub.indices.iter().map(|&i| i + base));
                mesh.vertices.extend(sub.vertices);
                mesh.skin.extend(sub.skin);
            } else {
                out.morphed.push(AssembledMesh {
                    material,
                    vertices: sub.vertices,
                    indices: sub.indices,
                    skin: sub.skin,
                    morph_targets: sub.targets.iter().map(|&t| infos[usize::from(t)].clone()).collect(),
                    morphs: sub.morphs,
                });
            }
        }
    }
    Ok(out)
}

impl AssembledMesh {
    fn empty(material: Material) -> Self {
        Self { material, vertices: Vec::new(), indices: Vec::new(), skin: Vec::new(), morph_targets: Vec::new(), morphs: Vec::new() }
    }
}
