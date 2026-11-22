//! LF-A1 (`docs/specs/worldgen-adapter.md` "Landforms (LF-A)"): the game's rock-unit table for the framework's
//! geology map (`sf_worldgen::geology`), `bevyrrowind/worldgen/geology.toml`.
//!
//! The table is the framework's [`GeologyTable`] TOML plus one adapter section, `[layer_names.<unit>]`
//! (`top`, `face`, `scree` as LTEX record ids), which the framework ignores. This module:
//!
//! - parses both halves and checks that every unit names its layers and every named unit exists
//!   ([`AdapterGeology::from_toml`]);
//! - resolves the layer names against a guide's `layers.json` palette (case-insensitive, as LTEX ids) into the
//!   numeric [`UnitLayers`] the framework reads ([`AdapterGeology::resolve_layers`], [`AdapterGeology::resolved_toml`]);
//! - reports which regions of a guide hold land and whether the table has a row for each
//!   ([`AdapterGeology::region_coverage`]): a region without a row falls back to the default shares, which the
//!   adapter spec allows only for land outside every region (legend index 0).

// Sample counts stay far below 2^52.
#![allow(clippy::cast_precision_loss)]

use std::collections::BTreeMap;

use serde::Deserialize;
use sf_worldgen::geology::{GeologyTable, UnitLayers};
use sf_worldgen::guide::Guide;
use sf_worldgen::{Error, Result};

/// The adapter-file name of the table (`bevyrrowind/worldgen/`).
// tunables: const (file name)
pub const TABLE_FILE: &str = "geology.toml";

/// Material layers of one unit, as LTEX record ids.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayerNames {
    pub top: String,
    pub face: String,
    pub scree: String,
}

/// The adapter half of the TOML (everything else is the framework's).
#[derive(Deserialize)]
struct AdapterPart {
    #[serde(default)]
    layer_names: BTreeMap<String, LayerNames>,
}

/// The game's geology table: the framework table and the unit layer names.
#[derive(Debug, Clone, PartialEq)]
pub struct AdapterGeology {
    pub table: GeologyTable,
    /// Unit name -> layer names.
    pub layer_names: BTreeMap<String, LayerNames>,
}

/// Land of one legend region in a guide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionCoverage {
    /// Legend index (0 = no region).
    pub index: u8,
    /// Legend name (`regions.json` `name`, the key of `[region."<name>"]`).
    pub name: String,
    /// Guide samples above sea level in this region.
    pub land_samples: usize,
    /// The table has a `[region."<name>"]` row.
    pub row: bool,
}

impl AdapterGeology {
    /// Parses and checks the table: the framework's checks, then every unit has layer names and every
    /// `[layer_names]` key is a unit.
    pub fn from_toml(text: &str) -> Result<Self> {
        let table = GeologyTable::from_toml(text)?;
        let part: AdapterPart = toml::from_str(text)?;
        let bad = |m: String| Err(Error::Invalid(format!("geology (adapter): {m}")));
        if let Some(u) = table.units.iter().find(|u| !part.layer_names.contains_key(&u.name)) {
            return bad(format!("unit {} has no [layer_names.{}]", u.name, u.name));
        }
        if let Some(k) = part.layer_names.keys().find(|k| !table.units.iter().any(|u| &u.name == *k)) {
            return bad(format!("[layer_names.{k}] names no unit"));
        }
        Ok(Self {
            table,
            layer_names: part.layer_names,
        })
    }

    /// The table with every unit's layer ids resolved against `palette` (a guide's `layers.json` palette;
    /// LTEX ids compare case-insensitively). Errors list every name the palette lacks.
    pub fn resolve_layers(&self, palette: &[String]) -> Result<GeologyTable> {
        let index: BTreeMap<String, usize> = palette
            .iter()
            .enumerate()
            .map(|(i, p)| (p.to_ascii_lowercase(), i))
            .collect();
        let mut missing = Vec::new();
        let mut find = |unit: &str, slot: &str, name: &str| -> u16 {
            let id = index.get(&name.to_ascii_lowercase()).and_then(|&i| u16::try_from(i).ok());
            if id.is_none() {
                missing.push(format!("{unit}.{slot} = {name}"));
            }
            id.unwrap_or(0)
        };
        let mut table = self.table.clone();
        for u in &mut table.units {
            let n = &self.layer_names[&u.name];
            u.layers = UnitLayers {
                top: find(&u.name, "top", &n.top),
                face: find(&u.name, "face", &n.face),
                scree: find(&u.name, "scree", &n.scree),
            };
        }
        if missing.is_empty() {
            Ok(table)
        } else {
            Err(Error::Invalid(format!(
                "geology (adapter): layers not in the palette: {}",
                missing.join(", ")
            )))
        }
    }

    /// [`Self::resolve_layers`] as the framework TOML (`scrib-worldgen geology --table`).
    pub fn resolved_toml(&self, palette: &[String]) -> Result<String> {
        let table = self.resolve_layers(palette)?;
        toml::to_string(&table).map_err(|e| Error::Invalid(format!("geology (adapter): toml: {e}")))
    }

    /// Land samples per legend region of `guide` (only regions holding land), in legend order. Without a
    /// region map, one entry for index 0.
    pub fn region_coverage(&self, guide: &Guide) -> Vec<RegionCoverage> {
        let mut land = [0usize; 256];
        for row in 0..guide.h {
            for col in 0..guide.w {
                if f64::from(guide.heights[row * guide.w + col]) <= guide.sea_level_m {
                    continue;
                }
                let r = guide
                    .regions
                    .as_ref()
                    .map_or(0, |rm| rm.at_sample(col, row, guide.spacing_m));
                land[usize::from(r)] += 1;
            }
        }
        land.iter()
            .enumerate()
            .filter(|&(_, &n)| n > 0)
            .map(|(i, &n)| {
                let index = u8::try_from(i).unwrap_or(u8::MAX);
                let name = guide
                    .regions
                    .as_ref()
                    .map(|rm| rm.names[i].clone())
                    .unwrap_or_default();
                let row = !name.is_empty() && self.table.region.contains_key(&name);
                RegionCoverage {
                    index,
                    name,
                    land_samples: n,
                    row,
                }
            })
            .collect()
    }
}

/// Named land regions of `coverage` without a table row (land outside every region, index 0, is allowed).
pub fn uncovered(coverage: &[RegionCoverage]) -> Vec<&RegionCoverage> {
    coverage.iter().filter(|c| c.index != 0 && !c.row).collect()
}

/// One line per region: index, name, land share, row or `DEFAULT`.
pub fn coverage_report(coverage: &[RegionCoverage]) -> String {
    let total: usize = coverage.iter().map(|c| c.land_samples).sum();
    coverage
        .iter()
        .map(|c| {
            format!(
                "{:>3} {:<34} {:>6.2} % land  {}",
                c.index,
                if c.name.is_empty() { "(no region)" } else { &c.name },
                100.0 * c.land_samples as f64 / total.max(1) as f64,
                if c.row { "row" } else { "DEFAULT" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
