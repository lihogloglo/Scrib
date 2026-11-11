//! Pack profiles: a texture pack's conventions as data (textures.md §7, owner rule 2026-11-11:
//! the renderer is generic, never tuned to one pack).
//!
//! The runtime has one canonical layout (Bevy `StandardMaterial`: sRGB base colour, linear
//! normal with +Y up, glTF ORM = R occlusion / G roughness / B metallic, depth = 1 − height).
//! A [`PackProfile`] says how a pack differs from it: which file suffix holds which
//! [`PackRole`], each role's channel layout ([`ChannelMap`], canonical channel ← shipped
//! channel), the normals' Y convention and the height map's neutral level. [`apply`] converts
//! one shipped file into its canonical [`Product`]s at bake, block-exactly where
//! [`crate::remap`] can, else reporting the cook path. Profile *instances* (which packs follow
//! which convention) belong to the game adapter; this module names no pack and no game.

use serde::{Deserialize, Serialize};

use crate::cook::{AlphaUse, Channel, Role, SourceY};
use crate::remap::ChannelMap;
use crate::rewrap::{Rewrap, canonical, rewrap_mapped};

/// What a pack file is, and its canonical channels (the order of its [`SuffixRule::layout`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackRole {
    /// Base colour: R, G, B, A (cutout / blend alpha, when the material uses it).
    Albedo,
    /// Tangent-space normal: X, Y, Z (A unused).
    Normal,
    /// Normal plus height: X, Y, Z, height (white = high).
    NormalHeight,
    /// Occlusion, roughness, metallic (glTF ORM; A unused).
    Orm,
    /// A terrain layer: R, G, B albedo, A roughness (terrain layer arrays, T5/T6).
    TerrainAlbedoRoughness,
}

/// One file suffix of a pack: `<base><suffix>.<ext>` holds `role`, laid out as `layout`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuffixRule {
    /// Appended to the base texture's stem (`""` for the base texture itself), matched without
    /// regard to ASCII case.
    pub suffix: String,
    pub role: PackRole,
    /// Canonical channel `i` of `role` ← this shipped channel.
    pub layout: ChannelMap,
}

/// A known defect of a pack, fixed at bake. No variant yet: the first candidate (a pack whose
/// packed map has an empty channel at level 0 only) is an open owner question
/// (textures-adapter.md §7), so nothing is fixed silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quirk {}

/// A texture pack's conventions (module docs).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackProfile {
    /// The convention's name, which a mod's manifest refers to.
    pub name: String,
    /// In lookup order: the first rule whose file exists wins its slot (a height-normal before a
    /// plain normal).
    pub suffixes: Vec<SuffixRule>,
    /// Which way the pack's normal maps point their green channel.
    pub normal_y: SourceY,
    /// The height value of an undisplaced surface (`0.0..=1.0`, usually 0.5): for the material
    /// builder's parallax setup (T4); the remaps don't use it.
    pub height_neutral: f32,
    #[serde(default)]
    pub quirks: Vec<Quirk>,
}

impl PackProfile {
    /// The rule a texture stem (path without extension) falls under and the base stem it
    /// belongs to: the longest matching suffix, the `""` rule (the base itself) last.
    pub fn rule_for<'s>(&self, stem: &'s str) -> Option<(&'s str, &SuffixRule)> {
        self.suffixes
            .iter()
            .filter(|rule| {
                stem.len() >= rule.suffix.len()
                    && stem.is_char_boundary(stem.len() - rule.suffix.len())
                    && stem[stem.len() - rule.suffix.len()..].eq_ignore_ascii_case(&rule.suffix)
            })
            .max_by_key(|rule| rule.suffix.len())
            .map(|rule| (&stem[..stem.len() - rule.suffix.len()], rule))
    }

    /// The companion stems to look for beside base stem `base`, in lookup order, with their
    /// rules (the `""` rule excluded).
    pub fn companions<'p>(
        &'p self,
        base: &'p str,
    ) -> impl Iterator<Item = (String, &'p SuffixRule)> + 'p {
        self.suffixes
            .iter()
            .filter(|rule| !rule.suffix.is_empty())
            .map(move |rule| (format!("{base}{}", rule.suffix), rule))
    }
}

/// The canonical material slot a [`Product`] fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Slot {
    /// `base_color_texture`.
    BaseColor,
    /// `normal_map_texture`.
    Normal,
    /// `depth_map` (1 − height).
    Depth,
    /// `occlusion_texture` (R) and `metallic_roughness_texture` (G roughness, B metallic).
    Orm,
    /// A terrain layer's albedo + roughness.
    TerrainLayer,
}

/// One canonical texture made from a shipped file.
#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub slot: Slot,
    /// Output channel ← shipped channel, the whole conversion (the rule's layout then the
    /// role's canonical map): what the cook path applies to decoded texels
    /// ([`ChannelMap::apply_rgba`]) when `result` is a [`Rewrap::Cook`].
    pub map: ChannelMap,
    /// The canonical role `result` was made for, and the cook path cooks.
    pub role: Role,
    pub result: Rewrap,
}

/// The products of a `rule` file, without the bytes: `(slot, layout, role)` with `layout`
/// relative to the shipped channels and `role` in the pack's convention (a normal's
/// `source_y` is the profile's). `alpha`: the material's alpha use, for [`PackRole::Albedo`].
pub fn plan_products(
    profile: &PackProfile,
    rule: &SuffixRule,
    alpha: AlphaUse,
) -> Vec<(Slot, ChannelMap, Role)> {
    let layout = rule.layout;
    let normal = Role::Normal {
        source_y: profile.normal_y,
    };
    match rule.role {
        PackRole::Albedo => vec![(Slot::BaseColor, layout, Role::Color { alpha })],
        PackRole::Normal => vec![(Slot::Normal, layout, normal)],
        PackRole::NormalHeight => vec![
            (Slot::Normal, layout, normal),
            (Slot::Depth, layout, Role::height(Channel::A)),
        ],
        PackRole::Orm => vec![(Slot::Orm, layout, Role::Packed)],
        PackRole::TerrainAlbedoRoughness => vec![(
            Slot::TerrainLayer,
            layout,
            Role::Color {
                alpha: AlphaUse::Blend,
            },
        )],
    }
}

/// Converts one shipped `rule` file (DDS or KTX2 bytes) into its canonical products
/// ([`plan_products`]): each block-exactly remapped and rewrapped where possible
/// ([`rewrap_mapped`]), else a [`Rewrap::Cook`] / [`Rewrap::Skip`] with its reason.
/// `alpha`: the material's alpha use, for [`PackRole::Albedo`].
pub fn apply(
    profile: &PackProfile,
    rule: &SuffixRule,
    bytes: &[u8],
    alpha: AlphaUse,
) -> Vec<Product> {
    plan_products(profile, rule, alpha)
        .into_iter()
        .map(|(slot, layout, role)| {
            let (to_canonical, canonical_role) = canonical(role);
            Product {
                slot,
                map: layout.then(to_canonical),
                role: canonical_role,
                result: rewrap_mapped(bytes, &layout, role),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
