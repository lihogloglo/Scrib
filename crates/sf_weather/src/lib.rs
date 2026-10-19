//! Bevy-free weather core (weather.md "Weather: state machine and presets
//! as data"; as built: "Weather core as built").
//!
//! - **Records** ([`WeatherPreset`], [`Climate`], [`PrecipKind`]) are
//!   `sf_records` kinds ([`register_kinds`]): adapters generate them, mods
//!   patch them field by field. [`WeatherBook`] resolves them by index.
//! - **State machine** ([`WeatherState`]): per climate, slots with hashed
//!   start jitter and a hashed weighted preset draw. The weather at any time
//!   is a pure function of `(seed, climate, slot index)` plus explicit
//!   overrides (forced changes, weight edits), which is all a save keeps;
//!   forecasts are queries ([`WeatherState::forecast`]).
//! - **Blending**: per-channel lead/lag windows (clouds lead, precipitation
//!   lags); continuous channels interpolate, discrete ones (precipitation
//!   kinds, sounds, lightning, tags, surface rates) crossfade per preset,
//!   never morph. Climates blend spatially across a border band
//!   ([`border_blend`]).
//! - **Output**: [`evaluate`] → [`WeatherFrame`] (sky medium + wind target,
//!   clouds, fog, precipitation, surface rates, lightning strikes from a
//!   Poisson schedule, cues, tags, storm wind sources, `visibility_m`).

mod book;
mod frame;
mod hash;
mod records;
mod state;

/// The term types' crates, for adapters that build presets from cloud
/// genera, hour curves and wind sources without depending on them directly.
pub use {sf_atmos, sf_sky, sf_wind};

pub use book::{ClimateIx, PrecipIx, PresetIx, WeatherBook};
pub use frame::{
    BORDER_BAND_M, CueFrame, LIGHTNING_CAP_PER_HOUR, LightningStrike, MAX_VISIBILITY_M, OneShotFrame, PrecipFrame, SPEED_OF_SOUND, StormFrame,
    WeatherFrame, WeatherInput, border_blend, border_weight, evaluate, evaluate_into,
};
pub use records::{
    CLIMATE_KIND, Climate, CloudTerms, Cue, FogTerms, Lightning, OneShotSet, PRECIP_KIND, PRESET_KIND, PrecipKind, PrecipLook,
    PrecipTerm, SkyTerms, SurfaceChannel, SurfaceRates, TransitionTiming, WeatherPreset, WindTerms, Window,
};
pub use state::{Blend, CHANNELS, CLOUDS, CUES, FOG, ForecastEntry, Override, PRECIP, SKY, WIND, WeatherState};

use sf_records::{RecordDb, RecordError};

/// Registers the weather record kinds (serde, so record files and patches
/// can write them by name).
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<WeatherPreset>(PRESET_KIND)?;
    db.register_serde::<Climate>(CLIMATE_KIND)?;
    db.register_serde::<PrecipKind>(PRECIP_KIND)
}

#[cfg(test)]
mod tests;
