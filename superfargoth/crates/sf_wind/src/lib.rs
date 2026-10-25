//! The wind field (Bevy-free). Spec: `docs/specs/weather.md` "Wind".
//!
//! One deterministic field sampled the same way by every consumer (grass,
//! trees, cloth, particles, clouds, fog, water, audio):
//!
//! - [`Wind`]: what weather sets and blends (mean velocity, gustiness, gust
//!   size). `sf_sky` re-exports it.
//! - [`WindState`]: the live wind: the gust field advected downwind
//!   (integrated, wrapped to [`PERIOD`]), local [`WindSource`]s (directional,
//!   radial, vortex; at most [`MAX_SOURCES`]) with lifetimes. Saveable.
//! - [`WindFrame`]: one frame's upload for a floating origin
//!   ([`WindParams`] + [`WindSourceGpu`]s) and its CPU sampler, the twin of
//!   the WGSL module [`WESL`] (`wind_global`, `wind_gust`, `wind_source`).
//!
//! Velocity = gust-field direction × mean speed × (1 ± gustiness by the gust
//! envelope) × a log altitude profile, plus the sources. The field repeats
//! every [`PERIOD`] meters, so it is sampled at `rel + (origin − drift) mod
//! PERIOD` (whole lattice cells + a fraction) and stays exact across
//! floating-origin shifts.
//!
//! The gust field is ported from the owner's Megalophobia project
//! (`assets/shaders/grass_wind.wgsl`), licensed by the owner as MIT OR
//! Apache-2.0 (ARCHITECTURE §10, 2026-10-12). That file cites no
//! third-party code. The lattice hash is PCG (Jarzynski and Olano 2020).

mod field;
mod frame;
mod source;
mod state;
mod wind;

#[cfg(test)]
mod tests;

/// The math types used in this API.
pub use glam;

pub use field::{GustSample, SOURCE_CORE};
pub use frame::{WindFrame, WindParams, WindSourceGpu};
pub use source::{MAX_SOURCES, WindSource, WindSourceId, WindSourceKind};
pub use state::{WindSettings, WindState};
pub use wind::Wind;

/// Spatial period of the gust field in meters (whole grid cells, so the
/// origin phase comes exactly from the origin's cell). Gust cells snap so
/// they divide it.
pub const PERIOD: f64 = 1024.0;

/// The WGSL (WESL, no imports) module: `WindParams`, `WindSource`,
/// `WindGust`, `wind_gust`, `wind_profile`, `wind_global`,
/// `wind_source_count`, `wind_source`. Pure functions, no bindings.
pub const WESL: &str = include_str!("shaders/sf_wind.wesl");
