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
    pub rules: RuleTable,
    pub channels: Vec<ChannelNames>,
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
    /// ruled, no empty model key). Species validation is [`Self::build`]'s.
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
        assert_eq!(cover.len(), 13);
        assert_eq!(data.rules.rules.len(), 7);
        assert_eq!(data.channels.len(), 7);
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
}
