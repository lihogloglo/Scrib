//! Morrowind groundcover data (slice GC-2b2; spec `bevyrrowind/docs/specs/groundcover-adapter.md` "First species
//! set"): the species, rule table and LTEX → rule channel table in `bevyrrowind/data/groundcover.ron`, and their
//! resolution against a baked [`LayerTable`]. Bevy-free; the wiring slice wraps [`ChannelMap::pairs`] into the
//! renderer's layer → channel remap and loads [`GroundcoverData::model_keys`].
//!
//! Terrain layer records are the LTEX editor ids (`terrain::layer_for`), stored lowercase by `sf_records`; the data
//! names them as the ESMs spell them, so matching ignores ASCII case.

use std::collections::{BTreeSet, HashMap};

use serde::Deserialize;
use sf_groundcover::{BuildError, Groundcover, RenderKind, RuleTable, Species};
use sf_terrain::{LayerId, LayerTable};
use thiserror::Error;

use crate::model_path;

/// The shipped data file.
pub const BUILTIN: &str = include_str!("../../../data/groundcover.ron");

/// One rule channel and the land textures (LTEX record ids) that feed it.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ChannelNames {
    pub channel: u16,
    /// A readable name (logs, tools); not matched against anything.
    pub name: String,
    pub ltex: Vec<String>,
}

/// The parsed data file.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GroundcoverData {
    pub species: Vec<Species>,
    /// The looks blade species name (`Blade(style: ..)`), by [`BladeStyleDef::name`] (slice GC-3e1).
    pub blade_styles: Vec<BladeStyleDef>,
    pub rules: RuleTable,
    pub channels: Vec<ChannelNames>,
}

/// An sRGB colour, each channel 0 to 1.
pub type Srgb = [f32; 3];

/// One blade style: a renderer preset and the look fields the data changes on it (slice GC-3e1). Plain data: the
/// exterior turns it into the renderer's style (`mw_exterior_bevy::groundcover::blade_styles`). The preset's own
/// density is never used (the species density sets each field's), so there is no density here.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BladeStyleDef {
    /// What `Blade(style: ..)` names.
    pub name: String,
    /// The renderer preset it starts from, as the preset's lower-case name (`"meadow"`).
    pub base: String,
    /// Overrides of the blade's look; `None` keeps the preset's.
    #[serde(default)]
    pub blade: BladeOverrides,
    /// Overrides of the field's variation.
    #[serde(default)]
    pub variation: VariationOverrides,
    /// Overrides of the ground showing through at the foot.
    #[serde(default)]
    pub ground_tint: GroundTintOverrides,
    /// The rule channel (its [`ChannelNames::name`]) whose land textures set the ground tint colour: the linear mean
    /// of their layers' mean albedo (slice GC-3e2a). An explicit `ground_tint.color` wins; `None` keeps the preset's.
    #[serde(default)]
    pub ground_from: Option<String>,
    /// Flat toon shading bands; 0 shades physically.
    #[serde(default)]
    pub toon_bands: Option<u32>,
    /// Other plants among the style's own blades (slice GC-3e2c). A non-empty list replaces the preset's own species
    /// list; an empty one keeps it. At most 15 (the renderer mixes 16 with the style's own blades).
    #[serde(default)]
    pub species: Vec<SubSpeciesDef>,
}

/// One plant growing among a blade style's own blades (`bevy_grass::GrassSpecies`; slice GC-3e2c): a blade preset
/// with overrides and its share of the field.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubSpeciesDef {
    /// The blade it starts from: a `bevy_grass` species preset by its lower-case name (`"seed_heads"`), or
    /// `"style:<preset>"` for a style preset's own blade (`"style:pampas"`).
    pub base: String,
    /// Share of the field's blades, 0 to 1.
    pub share: f32,
    /// Growing in patches about a metre across, 0 (scattered) to 1 (only in patches).
    #[serde(default)]
    pub clustering: f32,
    /// Overrides of the base blade's look.
    #[serde(default)]
    pub blade: BladeOverrides,
}

/// Overrides of one blade's look (`bevy_grass::GrassBlade`); every `None` keeps the base preset's value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BladeOverrides {
    /// Shortest and tallest blade, metres.
    pub height_m: Option<[f32; 2]>,
    /// Blade width at the root, metres.
    pub width_m: Option<f32>,
    /// Tip lean at rest, fraction of the height.
    pub lean: Option<f32>,
    /// Resistance to wind, 0 to 1.
    pub stiffness: Option<f32>,
    pub root_color: Option<Srgb>,
    pub tip_color: Option<Srgb>,
    /// Root darkening, 0 to 1.
    pub root_occlusion: Option<f32>,
    /// Back-lit glow, 0 to 1.
    pub translucency: Option<f32>,
    /// Sheen on the upper half, 0 to 1.
    pub specular: Option<f32>,
    /// Twist along the blade, turns at the tip.
    pub twist: Option<f32>,
    /// Droop, 0 (upright) to 1.
    pub droop: Option<f32>,
    /// Fold along the midrib, 0 to 1.
    pub fold: Option<f32>,
    /// Seed head or flower length, share of the blade; 0 for none.
    pub head_length: Option<f32>,
    /// Head width, metres.
    pub head_width_m: Option<f32>,
    pub head_color: Option<Srgb>,
}

/// Overrides of a field's variation (`bevy_grass::GrassVariation`); every `None` keeps the base preset's value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct VariationOverrides {
    pub patch_color: Option<Srgb>,
    /// How far blades in a patch take on the patch colour, 0 to 1.
    pub patch_amount: Option<f32>,
    /// Colour patch size, metres.
    pub patch_size_m: Option<f32>,
    /// Drying toward straw at the tips, 0 to 1.
    pub dry_tips: Option<f32>,
    /// Share of dead blades, 0 to 1.
    pub dead_share: Option<f32>,
    /// Tuft gathering, 0 (even) to 1.
    pub tufts: Option<f32>,
    /// Distance between tufts, metres.
    pub tuft_spacing_m: Option<f32>,
    /// Share of bare ground, 0 to 1.
    pub bare_share: Option<f32>,
    /// Bare patch size, metres.
    pub bare_patch_size_m: Option<f32>,
}

/// Overrides of the ground tint (`bevy_grass::GroundTint`); every `None` keeps the base preset's value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GroundTintOverrides {
    pub color: Option<Srgb>,
    /// How far the lowest third of each blade blends toward the colour, 0 to 1.
    pub amount: Option<f32>,
}

#[derive(Debug, Error)]
pub enum GroundcoverDataError {
    #[error("groundcover data: {0}")]
    Parse(#[from] ron::error::SpannedError),
    #[error("groundcover data: {0}")]
    Build(#[from] BuildError),
    #[error("groundcover data: LTEX {0:?} is listed twice")]
    DuplicateLtex(String),
    #[error("groundcover data: channel {0} is named twice")]
    DuplicateChannel(u16),
    #[error("groundcover data: rule channel {0} has no LTEX names")]
    UnnamedChannel(u16),
    #[error("groundcover data: named channel {0} has no rule")]
    ChannelWithoutRule(u16),
    #[error("groundcover data: species {0:?} has an empty model key")]
    EmptyModel(String),
    #[error("groundcover data: blade style {0:?} is defined twice")]
    DuplicateBladeStyle(String),
    #[error("groundcover data: species {species:?} names blade style {style:?}, which blade_styles does not define")]
    MissingBladeStyle { species: String, style: String },
}

/// Terrain layers resolved to rule channels.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChannelMap {
    /// `(layer, channel)` for every layer whose record the data names, sorted by layer id. Layers left out feed no
    /// channel. Sparse because a [`LayerId`] is the record's 32-bit hash (`sf_terrain::layer_id`), not an index.
    pub pairs: Vec<(LayerId, u16)>,
    /// LTEX names the data lists that no layer in the table carries (a missing plugin or a typo), in data order.
    pub unmatched: Vec<String>,
}

impl ChannelMap {
    /// The channel of `id`, if the data names its record.
    pub fn channel(&self, id: LayerId) -> Option<u16> {
        self.pairs.binary_search_by_key(&id.0, |(l, _)| l.0).ok().map(|i| self.pairs[i].1)
    }
}

impl GroundcoverData {
    /// Parses and checks the channel table (no LTEX listed twice, every rule channel named and every named channel
    /// ruled, no empty model key, blade style names unique and every blade species' style defined). Species
    /// validation is [`Self::build`]'s; whether a style's `base` preset exists is the renderer's (the exterior).
    pub fn parse(text: &str) -> Result<Self, GroundcoverDataError> {
        let data: Self = ron::from_str(text)?;
        data.check()?;
        Ok(data)
    }

    /// The shipped `bevyrrowind/data/groundcover.ron`.
    pub fn builtin() -> Result<Self, GroundcoverDataError> {
        Self::parse(BUILTIN)
    }

    fn check(&self) -> Result<(), GroundcoverDataError> {
        let mut seen_ltex = BTreeSet::new();
        let mut named = BTreeSet::new();
        for c in &self.channels {
            if !named.insert(c.channel) {
                return Err(GroundcoverDataError::DuplicateChannel(c.channel));
            }
            for name in &c.ltex {
                if !seen_ltex.insert(name.to_ascii_lowercase()) {
                    return Err(GroundcoverDataError::DuplicateLtex(name.clone()));
                }
            }
        }
        let ruled: BTreeSet<u16> = self.rules.rules.iter().map(|r| r.channel).collect();
        if let Some(&c) = ruled.difference(&named).next() {
            return Err(GroundcoverDataError::UnnamedChannel(c));
        }
        if let Some(&c) = named.difference(&ruled).next() {
            return Err(GroundcoverDataError::ChannelWithoutRule(c));
        }
        if let Some(s) = self.species.iter().find(|s| model_keys_of(s).into_iter().any(|k| model_path(k).is_none())) {
            return Err(GroundcoverDataError::EmptyModel(s.id.clone()));
        }
        let mut styles = BTreeSet::new();
        for b in &self.blade_styles {
            if !styles.insert(b.name.as_str()) {
                return Err(GroundcoverDataError::DuplicateBladeStyle(b.name.clone()));
            }
        }
        for s in &self.species {
            if let RenderKind::Blade { style } = &s.render
                && !styles.contains(style.as_str())
            {
                return Err(GroundcoverDataError::MissingBladeStyle { species: s.id.clone(), style: style.clone() });
            }
        }
        Ok(())
    }

    /// The validated species set with its compiled rules.
    pub fn build(&self) -> Result<Groundcover, GroundcoverDataError> {
        Ok(Groundcover::new(self.species.clone(), &self.rules)?)
    }

    /// Maps every layer of `table` whose record the data names (ASCII case ignored) to its rule channel.
    pub fn resolve(&self, table: &LayerTable) -> ChannelMap {
        let by_name: HashMap<String, u16> = self
            .channels
            .iter()
            .flat_map(|c| c.ltex.iter().map(move |n| (n.to_ascii_lowercase(), c.channel)))
            .collect();
        let mut matched = BTreeSet::new();
        let mut pairs: Vec<(LayerId, u16)> = table
            .layers
            .iter()
            .filter_map(|l| {
                let key = l.record.to_ascii_lowercase();
                let channel = *by_name.get(&key)?;
                matched.insert(key);
                Some((l.id, channel))
            })
            .collect();
        pairs.sort_by_key(|(l, _)| l.0);
        let unmatched = self
            .channels
            .iter()
            .flat_map(|c| c.ltex.iter())
            .filter(|n| !matched.contains(&n.to_ascii_lowercase()))
            .cloned()
            .collect();
        ChannelMap { pairs, unmatched }
    }

    /// Every model the species draw (LODs included), as `meshes/...` VFS keys ([`model_path`]), deduplicated, in
    /// declaration order.
    pub fn model_keys(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        self.species
            .iter()
            .flat_map(model_keys_of)
            .filter_map(model_path)
            // `model_path` lowercases, so keys that differ only in case collapse here.
            .filter(|k| seen.insert(k.clone()))
            .collect()
    }
}

/// A species' model and LOD keys as written; none for blades.
fn model_keys_of(s: &Species) -> Vec<&str> {
    match &s.render {
        RenderKind::Mesh { model, lods } => std::iter::once(model.as_str()).chain(lods.iter().map(String::as_str)).collect(),
        RenderKind::Blade { .. } => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_terrain::{LayerInfo, layer_id};

    fn table(records: &[&str]) -> LayerTable {
        LayerTable {
            layers: records
                .iter()
                .map(|r| LayerInfo {
                    record: (*r).to_owned(),
                    id: layer_id(r),
                    tiling_m: 1.0,
                    triplanar: false,
                    displacement_m: 0.0,
                    mean_albedo: [0.5; 3],
                    mean_height: 0.5,
                    p90_height: 0.5,
                    source_edge: 256,
                })
                .collect(),
            fill_layer: None,
            fill_height: 0.0,
        }
    }

    #[test]
    fn builtin_data_parses_and_builds() {
        let data = GroundcoverData::builtin().unwrap();
        let cover = data.build().unwrap();
        assert_eq!(cover.len(), 20);
        assert_eq!(data.rules.rules.len(), 7);
        assert_eq!(data.channels.len(), 7);
    }

    #[test]
    fn a_blade_species_of_its_own_on_every_grass_channel() {
        let data = GroundcoverData::builtin().unwrap();
        data.build().unwrap();
        let blades: Vec<&Species> = data.species.iter().filter(|s| matches!(s.render, RenderKind::Blade { .. })).collect();
        assert_eq!(blades.len(), data.channels.len(), "one blade species per channel");
        let mut seen = BTreeSet::new();
        for c in &data.channels {
            let rule = data.rules.rules.iter().find(|r| r.channel == c.channel).unwrap();
            let on: Vec<&Species> = blades.iter().copied().filter(|b| rule.species.iter().any(|w| w.species == b.id && w.weight > 0.0)).collect();
            assert_eq!(on.len(), 1, "channel {} has one blade species", c.name);
            assert!(seen.insert(on[0].id.as_str()), "{} is on one channel only", on[0].id);
            let RenderKind::Blade { style } = &on[0].render else { unreachable!() };
            let def = data.blade_styles.iter().find(|d| d.name == *style).expect("its style is defined");
            assert_eq!(def.ground_from.as_deref(), Some(c.name.as_str()), "{style} takes the ground colour of its channel");
        }
        let keys = data.model_keys();
        assert!(!keys.iter().any(|k| blades.iter().any(|b| k.contains(&b.id))), "blades load no model: {keys:?}");
    }

    #[test]
    fn layers_resolve_to_channels_case_insensitively() {
        let data = GroundcoverData::builtin().unwrap();
        // Records as the bake stores them (lowercase) and as the ESM spells them; one unknown layer.
        let t = table(&["ai_grass", "GL_GRASS_03", "tx_bc_moss.tga", "Tx_BM_grass_02.dds", "ai_grass_cobbles"]);
        let map = data.resolve(&t);
        let ch = |r: &str| map.channel(layer_id(r));
        assert_eq!(ch("ai_grass"), Some(0));
        assert_eq!(ch("GL_GRASS_03"), Some(1));
        assert_eq!(ch("tx_bc_moss.tga"), Some(4));
        assert_eq!(ch("Tx_BM_grass_02.dds"), Some(5));
        assert_eq!(ch("ai_grass_cobbles"), None);
        assert_eq!(ch("not_in_the_table"), None);
        assert_eq!(map.pairs.len(), 4);
        // Every other listed name is reported, in data order.
        let total: usize = data.channels.iter().map(|c| c.ltex.len()).sum();
        assert_eq!(map.unmatched.len(), total - 4);
        assert_eq!(map.unmatched.first().map(String::as_str), Some("AI_Grass_02"));
        assert!(!map.unmatched.iter().any(|n| n.eq_ignore_ascii_case("ai_grass")));
        assert!(map.unmatched.iter().any(|n| n == "Tx_BC_grass.tga"));
    }

    #[test]
    fn model_keys_are_complete_and_unique() {
        let keys = GroundcoverData::builtin().unwrap().model_keys();
        assert_eq!(keys.len(), 13);
        assert_eq!(keys.iter().collect::<BTreeSet<_>>().len(), keys.len());
        assert_eq!(keys[0], "meshes/f/flora_grass_01.nif");
        assert!(keys.iter().all(|k| k.starts_with("meshes/f/flora_") && std::path::Path::new(k).extension() == Some("nif".as_ref())));
    }

    #[test]
    fn channel_table_mistakes_are_rejected() {
        let mut data = GroundcoverData::builtin().unwrap();
        data.channels[1].ltex.push("ai_GRASS".into());
        assert!(matches!(data.check(), Err(GroundcoverDataError::DuplicateLtex(_))));
        let mut data = GroundcoverData::builtin().unwrap();
        data.channels.pop();
        assert!(matches!(data.check(), Err(GroundcoverDataError::UnnamedChannel(6))));
        let mut data = GroundcoverData::builtin().unwrap();
        data.rules.rules.pop();
        assert!(matches!(data.check(), Err(GroundcoverDataError::ChannelWithoutRule(6))));
    }

    #[test]
    fn blade_style_overrides_parse_into_their_fields() {
        let def: BladeStyleDef = ron::from_str(
            "(name: \"dry\", base: \"meadow\", blade: (tip_color: Some((0.6, 0.5, 0.3)), height_m: Some((0.2, 0.4))), \
             ground_tint: (amount: Some(0.5)), ground_from: Some(\"bc_moss\"), toon_bands: Some(3))",
        )
        .unwrap();
        assert_eq!(def.blade, BladeOverrides { tip_color: Some([0.6, 0.5, 0.3]), height_m: Some([0.2, 0.4]), ..BladeOverrides::default() });
        assert_eq!(def.variation, VariationOverrides::default());
        assert_eq!(def.ground_tint, GroundTintOverrides { color: None, amount: Some(0.5) });
        assert_eq!(def.toon_bands, Some(3));
        assert_eq!(def.ground_from.as_deref(), Some("bc_moss"));
        assert!(def.species.is_empty(), "no species listed keeps the preset's");
        assert!(ron::from_str::<BladeStyleDef>("(name: \"x\", base: \"meadow\", blade: (tip_colour: None))").is_err(), "typos are errors");
    }

    #[test]
    fn blade_style_species_parse_into_their_fields() {
        let def: BladeStyleDef = ron::from_str(
            "(name: \"dry\", base: \"dry_savanna\", species: [             (base: \"style:pampas\", share: 0.15, clustering: 0.6, blade: (height_m: Some((0.9, 1.4)))),              (base: \"seed_heads\", share: 0.05)])",
        )
        .unwrap();
        assert_eq!(
            def.species,
            vec![
                SubSpeciesDef {
                    base: "style:pampas".into(),
                    share: 0.15,
                    clustering: 0.6,
                    blade: BladeOverrides { height_m: Some([0.9, 1.4]), ..BladeOverrides::default() },
                },
                SubSpeciesDef { base: "seed_heads".into(), share: 0.05, clustering: 0.0, blade: BladeOverrides::default() },
            ]
        );
        assert!(
            ron::from_str::<BladeStyleDef>("(name: \"x\", base: \"meadow\", species: [(base: \"moss\", shares: 0.1)])").is_err(),
            "typos are errors"
        );
    }

    #[test]
    fn blade_style_mistakes_are_rejected_by_name() {
        let mut data = GroundcoverData::builtin().unwrap();
        data.blade_styles.clear();
        let err = data.check().unwrap_err();
        assert!(
            matches!(&err, GroundcoverDataError::MissingBladeStyle { species, style }
                if species == "ascadian_meadow_blades" && style == "ascadian"),
            "{err}"
        );
        assert!(err.to_string().contains("\"ascadian\""), "{err}");
        let mut data = GroundcoverData::builtin().unwrap();
        data.blade_styles.push(data.blade_styles[0].clone());
        assert!(matches!(data.check(), Err(GroundcoverDataError::DuplicateBladeStyle(n)) if n == "ascadian"));
    }
}
