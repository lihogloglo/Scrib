//! Bevy-free groundcover core (spec `superfargoth/docs/specs/groundcover.md`, slice GC-1): species, the rule table
//! (input channels → weighted species), overrides (density paint, exclusion, "only these species" regions with
//! priority, explicit-placement regions), and the deterministic per-tile scatter with its density CPU twin.
//!
//! The renderer (`sf_groundcover_bevy`, GC-2) runs [`Scatterer::scatter_tile`] in a task per world tile and species
//! and copies the flat `Vec<Instance>` into a GPU batch. A game adapter fills the species and the rule table from its
//! own data and implements [`GroundSampler`] over its terrain; this crate names no game format and works in metres.
//!
//! ```
//! use sf_groundcover::*;
//!
//! struct Flat;
//! impl GroundSampler for Flat {
//!     fn sample(&self, _x: f64, _z: f64, out: &mut GroundSample) {
//!         *out = GroundSample { channels: vec![1.0], ..GroundSample::blank() };
//!     }
//! }
//!
//! let grass = Species {
//!     id: "grass".into(),
//!     render: RenderKind::Blade { style: "meadow".into() },
//!     footprint_m: 0.5,
//!     density: 2.0,
//!     slope_deg: Some(Span::new(0.0, 35.0)),
//!     altitude_m: None,
//!     water_m: Some(Span::new(0.2, 1.0e9)),
//!     scale: Span::new(0.8, 1.2),
//!     yaw_deg: Span::new(0.0, 360.0),
//!     tint: [[0.9, 1.0, 0.9], [1.0, 1.0, 0.8]],
//!     align_to_surface: Span::new(0.5, 1.0),
//!     wind: 1.0,
//!     trample: 1.0,
//!     z_offset_m: 0.0,
//!     shadow: ShadowClass::None,
//!     range: RangeClass::Near,
//! };
//! let rules = RuleTable {
//!     rules: vec![ChannelRule { channel: 0, species: vec![WeightedSpecies { species: "grass".into(), weight: 1.0 }] }],
//! };
//! let cover = Groundcover::new(vec![grass], &rules).unwrap();
//! let overrides = Overrides::none();
//! let scatter = Scatterer::new(&cover, &overrides, ScatterSettings::default());
//! let mut out = Vec::new();
//! let stats = scatter.scatter_tile(&Flat, TileKey::new(3, -7), SpeciesIndex(0), 0, &mut out);
//! assert_eq!(stats.accepted as usize, out.len());
//! ```

mod ground;
mod hash;
mod overrides;
mod rules;
mod scatter;
mod species;
pub mod tunables;

pub use ground::{GroundSample, GroundSampler};
pub use hash::{Stream, candidate_seed, mix64, species_hash};
pub use overrides::{DensityPaint, ExplicitRegion, Mask, OnlyRegion, OverrideSet, OverrideView, Overrides, Rect, Region, Shape};
pub use rules::{BuildError, ChannelRule, Groundcover, RuleTable, SpeciesIndex, WeightedSpecies};
pub use scatter::{Grid, Instance, ScatterSettings, Scatterer, Tag, TileKey, TileStats, accepts};
pub use species::{RangeClass, RenderKind, ShadowClass, Span, Species};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_overrides;
