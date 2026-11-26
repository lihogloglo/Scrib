//! Bevy-free world generation for superfargoth (spec `superfargoth/docs/specs/worldgen.md`).
//!
//! This crate is the CPU macro stage and its preview tools, built for the WG-S2 look test:
//!
//! - [`guide`]: the generic guide-grid reader (format v1: raw f32 heights + JSON, optional coarse region grid).
//! - [`resample`]: bicubic (Catmull-Rom) sampling, used to scale a guide by a horizontal factor S.
//! - [`flood`]: depression routing by Priority-Flood+ε (Barnes, Lehman, Mulla 2014).
//! - [`stream`]: macro fluvial erosion by the stream power law with the implicit O(n) solver of
//!   Braun & Willett (2013), routed over the flooded surface, uplift relaxing toward the intent
//!   (Cordonnier et al. 2016 drive the form by an uplift map).
//!   Drainage area by D8 or by multiple-flow-direction accumulation ([`mfd`]: Freeman 1991, Quinn et al. 1991,
//!   Holmgren 1994; as Schott et al. 2023), an optional steady-state stop on the per-step residual.
//! - [`perturb`]: integer-hash value noise in absolute world coordinates that breaks grid symmetry (uplift
//!   target + initial height).
//! - [`metrics`]: channel-network measurements (direction shares of channel length, main-stem sinuosity).
//! - [`thermal`]: a talus-angle relaxation pass (Musgrave, Kolb, Mace 1989; Olsen 2004).
//! - [`shade`]: a fixed-light hillshade renderer and contact sheets (8-bit PNG).
//! - [`recipe`] and [`run`]: the recipe file the `scrib-worldgen` binary runs.
//!
//! WG-S4, the landform-first macro (`pipeline = "landform"` recipes, [`landform`]): [`layout`] (the source reduced
//! to a mask, a Gaussian low-passed relief and the region map), [`landscape`] (uplift-driven stream power +
//! hillslope diffusion + threshold slope to steady state, Cordonnier et al. 2016 / Braun & Willett 2013), [`volcano`]
//! (parametric stratovolcano or exemplar stamp, C1 blend) and [`structure`] (channel orientation coherence,
//! drainage density, junction angles).
//!
//! WG-L2a, the landform census ([`landforms`], `scrib-worldgen landforms`): craters, basins, lakes, lagoons,
//! trenches, gorges, valleys, peaks, plateaus, bays, headlands and islands found in any guide by standard
//! geomorphometry, written as `features.json` + an overlay, names joined from a generic names file.
//!
//! WG-S5 (same runner, per-arm `solver`, `rock`, `uplift_noise`, `rough_start`, `finite`): [`lem`] (the landscape
//! evolution model ported from Veloren's `erosion.rs`, GPL-3.0: multi-receiver implicit stream power, lakes by
//! Cordonnier et al. 2019, fixed anchor cells) and [`lithology`] (rock-type Worley map for erodibility, uplift
//! noise); arms with equal landscape switches share one landscape run.
//!
//! G1, runtime terrain ([`tiles`]): canonical `sf_terrain` tiles generated on demand from a guide plus the
//! band-limited [`filter`], a `TerrainTiles` provider for the game renderer.
//!
//! Rules (worldgen.md "Constraints", "Stage rules"): deterministic (single-threaded where order matters,
//! `rayon` only for pure per-sample maps), sea cells are base level and never move, every stage audits the
//! wet/dry class against its own input, and every parameter is a recipe value with a unit, a range and a source.
//! Nothing here names a game: recipes name regions by the names the guide's legend gives.

// Grid maths: sample counts stay far below 2^31 and heights within f32 range, so index/float casts are exact
// enough by construction (checked once where a guide is read).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    // Grid maths reads best in the usual letters (w, h, i, x, y, z for the routing surface).
    clippy::many_single_char_names
)]

pub mod anchors;
pub mod cliffs;
pub mod climate;
pub mod climate_classes;
pub mod climate_map;
pub mod climate_override;
pub mod dem;
pub mod filter;
pub mod flood;
pub mod geology;
pub mod grid;
pub mod guide;
pub mod height_curve;
pub mod landform;
pub mod landforms;
pub mod landscape;
pub mod layers;
pub mod layout;
pub mod lem;
pub mod lithology;
pub mod measure;
pub mod metrics;
pub mod mfd;
pub mod perturb;
pub mod placed;
pub mod recipe;
pub mod resample;
pub mod roads;
pub mod run;
pub mod scatter;
pub mod scatter_fields;
pub mod scatter_species;
pub mod shade;
pub mod stamps;
pub mod stratovolcano;
pub mod stream;
pub mod structure;
pub mod thermal;
pub mod tiles;
pub mod valley_terrace;
pub mod volcano;
pub mod water;

pub use grid::Grid;

/// Errors of the worldgen tool.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("recipe: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("png decode: {0}")]
    PngDecode(#[from] png::DecodingError),
    #[error("png encode: {0}")]
    PngEncode(#[from] png::EncodingError),
    #[error("{0}")]
    Invalid(String),
}

/// Result alias of the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Wet/dry census of a grid against a sea level (worldgen.md "Stage rules": no stage creates or removes water).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize)]
pub struct WetDry {
    /// Samples at or above sea level.
    pub land: usize,
    /// Samples below sea level.
    pub sea: usize,
}

impl WetDry {
    /// Counts land and sea samples of `h`.
    pub fn count(h: &[f64], sea_level: f64) -> Self {
        let sea = h.iter().filter(|&&v| v < sea_level).count();
        Self {
            land: h.len() - sea,
            sea,
        }
    }
}

/// Audit of one stage: samples whose wet/dry class changed between `before` and `after`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize)]
pub struct Audit {
    /// Dry samples that became wet.
    pub new_wet: usize,
    /// Wet samples that became dry.
    pub new_dry: usize,
    /// Wet samples whose height changed at all (sea is fixed base level).
    pub sea_moved: usize,
}

impl Audit {
    /// Compares two height fields of the same grid.
    pub fn compare(before: &[f64], after: &[f64], sea_level: f64) -> Self {
        let mut a = Self::default();
        for (&b, &n) in before.iter().zip(after) {
            match (b < sea_level, n < sea_level) {
                (false, true) => a.new_wet += 1,
                (true, false) => a.new_dry += 1,
                _ => {}
            }
            if b < sea_level && b.to_bits() != n.to_bits() {
                a.sea_moved += 1;
            }
        }
        a
    }

    /// True when the stage kept every class and every sea height.
    pub fn is_clean(&self) -> bool {
        self.new_wet == 0 && self.new_dry == 0 && self.sea_moved == 0
    }
}

/// blake3 of a height field as little-endian f32 (the product hash; worldgen.md "Deterministic").
pub fn hash_f32(h: &[f64]) -> String {
    let mut hasher = blake3::Hasher::new();
    for &v in h {
        hasher.update(&(v as f32).to_le_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

/// blake3 of a height field as little-endian f64 (exact equality of the working state).
pub fn hash_f64(h: &[f64]) -> String {
    let mut hasher = blake3::Hasher::new();
    for &v in h {
        hasher.update(&v.to_le_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_s4;
#[cfg(test)]
mod tests_s5;
#[cfg(test)]
mod tests_measure;
#[cfg(test)]
mod tests_geology;
#[cfg(test)]
mod tests_placed;
#[cfg(test)]
mod tests_valley_terrace;
