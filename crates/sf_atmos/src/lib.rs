//! The cloud model, without Bevy (weather.md, "Clouds"; crate split rule 7).
//!
//! - [`settings`]: [`CloudSettings`] (what the weather drives: coverage,
//!   type, density, wetness, storm cells, layer heights, wind) with a few
//!   presets ([`CloudPreset`]) and blending, so the weather state machine can
//!   drive the clouds by changing scalars only.
//! - [`noise`]: the tiling noise volumes (Perlin-Worley shape, Worley detail)
//!   and the weather-noise map, generated on the CPU, deterministic per seed.
//! - [`density`]: the Nubis density model and the weather-map lookup, the
//!   CPU twin of the WGSL in `sf_atmos_bevy` (tests, gameplay queries).
//! - [`scroll`]: wind scroll and floating-origin phase, wrapped per texture.
//! - [`shadow`]: time slicing of the toroidal cloud shadow map.
//! - [`fog`]: height fog and ground-fog banks (F2), the CPU twin of the
//!   fog that the same render node draws.
//!
//! **Provenance.** The density model (`combine_density`, `erode_density`,
//! `height_gradient`), the lighting math in the WGSL twin, the noise
//! generators (lattice value noise, tiling Worley, Perlin-Worley) and the
//! per-texture scroll rule are ported from the owner's Megalophobia project
//! (`src/clouds.rs`, `src/volumetrics.rs`, `assets/shaders/cloud_density.wgsl`,
//! `cloud_deck.wgsl`), licensed by the owner as MIT OR Apache-2.0
//! (ARCHITECTURE §10, 2026-10-12). Those files cite papers only (Schneider
//! 2015/2017, Hillaire 2016, Wrenninge); no third-party code is included.

pub mod density;
pub mod fog;
pub mod noise;
pub mod scroll;
pub mod settings;
pub mod shadow;

#[cfg(test)]
mod tests;

pub use density::{cirrus_at, combine_density, erode_density, evolve_field, height_gradient, weather_at, WeatherSample};
pub use fog::{FogBanks, FogDrift, FogLayer, FogOffsets, FogSettings};
pub use noise::{NoiseVolumes, DETAIL_SIZE, SHAPE_SIZE, WEATHER_SIZE};
pub use scroll::CloudScroll;
pub use settings::{CloudLook, CloudPreset, CloudQuality, CloudSettings, CloudWeather};
pub use shadow::{ShadowRows, ShadowSlicer};
