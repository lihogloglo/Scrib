//! Bevy plugin for [`sf_weather`] (weather.md "Canonical technique"; as
//! built: "WE3 as built").
//!
//! Each frame, in `Update` between `SkySystems::Clock` and
//! `SkySystems::Apply`:
//!
//! 1. [`WeatherSystems::Evaluate`]: the game clock ([`SkyTime`]) and the
//!    camera's climates ([`WeatherSite`], written by the app: it knows its
//!    world's regions) go through [`sf_weather::evaluate_into`] into
//!    [`CurrentWeather`]. The weather is a pure function of the records, the
//!    seed and overrides ([`WorldWeather`]), the clock and the site, so a
//!    host that replicates those three replicates the weather.
//! 2. [`WeatherSystems::Apply`]: the frame drives the sky medium (haze,
//!    dust, dust tint on `sf_sky_bevy::WeatherState`; [`WeatherOverrides`]
//!    pins either) and the wind field (`sf_wind_bevy::WindField`: the ground
//!    wind target and one radial source per storming climate).
//!
//! Cloud cover and dimming, fog, precipitation and lightning are read from
//! [`CurrentWeather`] by their own consumers (M6 WE3 part 2).

use std::sync::Arc;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_platform::time::Instant;
use sf_diag_bevy::DiagCounters;
use sf_sky_bevy::{SkyConfig, SkySystems, SkyTime};
use sf_weather::{ClimateIx, WeatherBook, WeatherFrame, WeatherInput, WeatherState, evaluate_into};
use sf_wind_bevy::WindField;
use sf_wind_bevy::sf_wind::WindSourceId;
use smallvec::SmallVec;

pub use sf_weather;

#[cfg(test)]
mod tests;

/// Longest stretch of game time one frame evaluates lightning over, game
/// hours: a clock jump (sleep, wait, time-lapse) skips strikes instead of
/// firing a backlog.
pub const MAX_STEP_HOURS: f64 = 0.25;

/// The plugin: the records and the starting [`WeatherState`] (seed and
/// overrides, from a new game or a save).
pub struct WeatherPlugin {
    pub book: Arc<WeatherBook>,
    pub state: WeatherState,
}

/// The resolved weather records.
#[derive(Resource, Clone)]
pub struct WeatherRecords(pub Arc<WeatherBook>);

/// The world's weather state: seed and explicit overrides (forced changes,
/// weight edits). The save data; scripts and `--weather` force through it.
#[derive(Resource, Clone, Debug, Default)]
pub struct WorldWeather(pub WeatherState);

/// Where the weather is evaluated: the camera's climates with their spatial
/// weights (one, or two across a border band) and its altitude (meters
/// above sea level). Written by the app; empty keeps the last weather.
#[derive(Resource, Clone, Debug, Default)]
pub struct WeatherSite {
    pub climates: SmallVec<[(ClimateIx, f32); 2]>,
    pub altitude: f32,
}

/// This frame's weather. `frame` is meaningful once `valid` (a site with a
/// climate has been evaluated).
#[derive(Resource, Clone, Debug, Default)]
pub struct CurrentWeather {
    pub frame: WeatherFrame,
    /// Game hours since the clock's epoch the frame was evaluated at.
    pub hours: f64,
    pub valid: bool,
}

/// Fixed sky terms instead of the weather's (debug flags, photo modes).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WeatherOverrides {
    pub haze: Option<f32>,
    pub dust: Option<f32>,
}

/// CPU cost of [`WeatherSystems`] (evaluate + apply), milliseconds.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WeatherStats {
    pub last_ms: f32,
    /// Exponential mean (α 0.05).
    pub mean_ms: f32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherSystems {
    /// `Update`, after `SkySystems::Clock`: fill [`CurrentWeather`].
    Evaluate,
    /// `Update`, before `SkySystems::Apply`: sky medium and wind.
    Apply,
}

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeatherRecords(self.book.clone()))
            .insert_resource(WorldWeather(self.state.clone()))
            .init_resource::<WeatherSite>()
            .init_resource::<CurrentWeather>()
            .init_resource::<WeatherOverrides>()
            .init_resource::<WeatherStats>()
            .configure_sets(
                Update,
                (WeatherSystems::Evaluate, WeatherSystems::Apply)
                    .chain()
                    .after(SkySystems::Clock)
                    .before(SkySystems::Apply),
            )
            .add_systems(Update, evaluate.in_set(WeatherSystems::Evaluate))
            .add_systems(
                Update,
                (apply_sky, apply_wind).in_set(WeatherSystems::Apply),
            );
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn evaluate(
    time: Res<SkyTime>,
    config: Res<SkyConfig>,
    records: Res<WeatherRecords>,
    world: Res<WorldWeather>,
    site: Res<WeatherSite>,
    mut current: ResMut<CurrentWeather>,
    mut stats: ResMut<WeatherStats>,
    counters: Option<Res<DiagCounters>>,
) {
    if site.climates.is_empty() {
        return;
    }
    let t0 = Instant::now();
    let calendar = &config.0.calendar;
    let hours = time.0.days * f64::from(calendar.hours_per_day);
    // Lightning fires for strikes in (prev, hours]: none on the first frame
    // or when the clock went back, at most MAX_STEP_HOURS after a jump.
    let prev = if current.valid && current.hours <= hours {
        current.hours.max(hours - MAX_STEP_HOURS)
    } else {
        hours
    };
    let input = WeatherInput {
        hours,
        prev_hours: prev,
        hour_of_day: time.0.hour(calendar),
        altitude: site.altitude,
        climates: &site.climates,
    };
    let current = &mut *current;
    evaluate_into(&records.0, &world.0, &input, &mut current.frame);
    current.hours = hours;
    current.valid = true;
    #[expect(clippy::cast_possible_truncation, reason = "milliseconds fit f32")]
    let ms = (t0.elapsed().as_secs_f64() * 1000.0) as f32;
    stats.last_ms = ms;
    stats.mean_ms += 0.05 * (ms - stats.mean_ms);
    if let Some(c) = counters {
        let unit = sf_diag_bevy::sf_diag::unit::MS;
        c.add_named("weather/eval/ms", unit, f64::from(ms));
        #[expect(clippy::cast_precision_loss, reason = "a handful of strikes")]
        c.add_named(
            "weather/strikes",
            "count",
            current.frame.lightning.len() as f64,
        );
    }
}

/// The sky medium: haze (with the preset's hour curve), dust and its tint.
/// Written only when they change, so the sky's rate-limited rebuild sees
/// no churn.
#[allow(clippy::needless_pass_by_value)]
fn apply_sky(
    current: Res<CurrentWeather>,
    overrides: Res<WeatherOverrides>,
    sky: Option<ResMut<sf_sky_bevy::WeatherState>>,
) {
    let Some(mut sky) = sky else { return };
    if !current.valid {
        return;
    }
    let f = &current.frame.sky;
    let haze = overrides.haze.unwrap_or(f.haze);
    let dust = overrides.dust.unwrap_or(f.dust);
    let w = &sky.0;
    if (w.haze, w.dust, w.dust_tint) != (haze, dust, f.dust_tint) {
        let w = &mut sky.0;
        w.haze = haze;
        w.dust = dust;
        w.dust_tint = f.dust_tint;
    }
}

/// The wind target and the storm sources (one per storming climate, its
/// strength retuned in place as the storm blends in and out).
#[allow(clippy::needless_pass_by_value)]
fn apply_wind(
    current: Res<CurrentWeather>,
    wind: Option<ResMut<WindField>>,
    mut storms: Local<SmallVec<[(ClimateIx, WindSourceId); 2]>>,
    counters: Option<Res<DiagCounters>>,
) {
    let Some(mut wind) = wind else { return };
    if !current.valid {
        return;
    }
    let t0 = Instant::now();
    let frame = &current.frame;
    if wind.state.wind != *frame.wind() {
        wind.state.wind = *frame.wind();
    }
    let state = &mut wind.state;
    storms.retain(|(c, id)| {
        let keep = frame.storms.iter().any(|s| s.climate == *c) && state.source_mut(*id).is_some();
        if !keep {
            state.remove_source(*id);
        }
        keep
    });
    for s in &frame.storms {
        let strength = s.source.strength * s.strength.clamp(0.0, 1.0);
        if let Some((_, id)) = storms.iter().find(|(c, _)| *c == s.climate) {
            if let Some(source) = state.source_mut(*id)
                && (source.strength - strength).abs() > 1e-4
            {
                source.strength = strength;
            }
        } else {
            let source = sf_wind_bevy::sf_wind::WindSource {
                strength,
                duration: None,
                ..s.source
            };
            if let Some(id) = state.add_source(source) {
                storms.push((s.climate, id));
            }
        }
    }
    if let Some(c) = counters {
        c.add_named(
            "weather/apply/ms",
            sf_diag_bevy::sf_diag::unit::MS,
            t0.elapsed().as_secs_f64() * 1000.0,
        );
    }
}
