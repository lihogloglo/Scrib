//! The weather records: presets, climates and precipitation kinds. Plain
//! serde data, registered as `sf_records` kinds ([`crate::register_kinds`]),
//! so adapters generate them and mods patch them field by field.
//!
//! Durations are in **game hours** (the sky clock), so time scale and
//! sleeping work. Speeds are m/s, lengths meters, densities 1/m.

use glam::{Vec2, Vec3};
use serde::{Deserialize, Serialize};
use sf_atmos::{CloudWeather, FogBanks, FogLayer, FogSettings};
use sf_records::RecordId;
use sf_sky::KeyCurve;
use sf_wind::{Wind, WindSource};

/// Record kind names.
pub const PRESET_KIND: &str = "weather.preset";
pub const CLIMATE_KIND: &str = "weather.climate";
pub const PRECIP_KIND: &str = "weather.precip";

/// A part of a transition, as fractions `[start, end]` of its duration;
/// the channel's progress is a smoothstep across it (lead/lag).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Window {
    pub start: f32,
    pub end: f32,
}

impl Window {
    pub const fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    /// Progress of the channel at transition fraction `s` (0..1, C1).
    pub fn ramp(self, s: f32) -> f32 {
        let (a, b) = (self.start.clamp(0.0, 1.0), self.end.clamp(0.0, 1.0));
        if s >= b {
            return 1.0;
        }
        if s <= a {
            return 0.0;
        }
        let t = (s - a) / (b - a);
        t * t * (3.0 - 2.0 * t)
    }
}

/// When each channel changes during a transition *into* this preset
/// (`*_out`: while it leaves, for the discrete channels). Defaults
/// (weather.md): clouds lead (0-60 %), wind in the middle (20-80 %),
/// precipitation and fog lag (40-100 %); precipitation and its sound stop
/// early when leaving (0-60 %), so rain clears before the sky does.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransitionTiming {
    pub clouds: Window,
    pub sky: Window,
    pub wind: Window,
    pub fog: Window,
    /// Precipitation, lightning, surface rates and tags fading in.
    pub precipitation: Window,
    /// The same fading out when this preset is the one leaving.
    pub precipitation_out: Window,
    pub cues: Window,
    pub cues_out: Window,
}

impl Default for TransitionTiming {
    fn default() -> Self {
        Self {
            clouds: Window::new(0.0, 0.6),
            sky: Window::new(0.0, 0.6),
            wind: Window::new(0.2, 0.8),
            fog: Window::new(0.4, 1.0),
            precipitation: Window::new(0.4, 1.0),
            precipitation_out: Window::new(0.0, 0.6),
            cues: Window::new(0.4, 1.0),
            cues_out: Window::new(0.0, 0.6),
        }
    }
}

/// The volumetric cloud layer and the high layer (`sf_atmos::CloudWeather`
/// as data; the direction comes from the wind).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CloudTerms {
    pub coverage: f32,
    /// 0 flat stratus, 1 piled cumulus.
    pub cloud_type: f32,
    pub density: f32,
    pub wetness: f32,
    pub storm: f32,
    pub erosion: f32,
    /// Layer base, meters above sea level.
    pub base: f32,
    pub thickness: f32,
    /// Wind speed at cloud altitude, m/s (direction: the ground wind's).
    pub speed: f32,
    pub cirrus: f32,
    pub cirrus_height: f32,
}

impl CloudTerms {
    /// From a cloud weather (e.g. an `sf_atmos::CloudPreset` genus).
    pub fn from_weather(w: &CloudWeather) -> Self {
        Self {
            coverage: w.coverage,
            cloud_type: w.cloud_type,
            density: w.density,
            wetness: w.wetness,
            storm: w.storm,
            erosion: w.erosion,
            base: w.base,
            thickness: w.thickness,
            speed: w.wind.length(),
            cirrus: w.cirrus,
            cirrus_height: w.cirrus_height,
        }
    }

    /// The renderer's cloud weather, moving along `direction` (unit X, Z).
    pub fn to_weather(&self, direction: Vec2) -> CloudWeather {
        CloudWeather {
            coverage: self.coverage,
            cloud_type: self.cloud_type,
            density: self.density,
            wetness: self.wetness,
            storm: self.storm,
            erosion: self.erosion,
            base: self.base,
            thickness: self.thickness,
            wind: direction * self.speed,
            cirrus: self.cirrus,
            cirrus_height: self.cirrus_height,
        }
    }
}

impl Default for CloudTerms {
    fn default() -> Self {
        Self::from_weather(&CloudWeather::default())
    }
}

/// What the weather does to the sky medium (`sf_sky::Weather` minus the
/// cloud cover, which comes from the clouds).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkyTerms {
    pub haze: f32,
    pub dust: f32,
    pub dust_tint: Vec3,
    pub overcast_dimming: f32,
    /// Multiplier on `haze` over the hour of day (empty: 1), e.g. morning haze.
    pub haze_by_hour: KeyCurve,
}

impl Default for SkyTerms {
    fn default() -> Self {
        let w = sf_sky::Weather::default();
        Self {
            haze: w.haze,
            dust: w.dust,
            dust_tint: w.dust_tint,
            overcast_dimming: w.overcast_dimming,
            haze_by_hour: KeyCurve::default(),
        }
    }
}

/// Height fog and ground-fog banks (`sf_atmos::FogSettings`' weather part).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FogTerms {
    /// Height fog extinction at `height_base`, 1/m (0: none).
    pub height_density: f32,
    pub height_falloff: f32,
    pub height_base: f32,
    /// Ground-fog bank extinction at its base, 1/m (0: none).
    pub bank_density: f32,
    pub bank_falloff: f32,
    pub bank_coverage: f32,
    pub albedo: Vec3,
    pub phase_g: f32,
    pub ambient: f32,
    /// Multiplier on the height fog over the hour of day (empty: 1).
    pub height_by_hour: KeyCurve,
    /// Multiplier on the banks over the hour of day (empty: 1), e.g. dawn banks.
    pub banks_by_hour: KeyCurve,
}

impl Default for FogTerms {
    fn default() -> Self {
        let f = FogSettings::default();
        Self {
            height_density: 0.0,
            height_falloff: f.height.falloff,
            height_base: f.height.base,
            bank_density: 0.0,
            bank_falloff: f.banks.layer.falloff,
            bank_coverage: f.banks.coverage,
            albedo: f.albedo,
            phase_g: f.phase_g,
            ambient: f.ambient,
            height_by_hour: KeyCurve::default(),
            banks_by_hour: KeyCurve::default(),
        }
    }
}

impl FogTerms {
    /// The fog settings with these terms; the look fields (bank tiles,
    /// march reach, sky distance) keep `FogSettings::default()`.
    pub fn to_settings(&self, wind: Vec2) -> FogSettings {
        let d = FogSettings::default();
        FogSettings {
            height: FogLayer { density: self.height_density, falloff: self.height_falloff, base: self.height_base },
            banks: FogBanks {
                layer: FogLayer { density: self.bank_density, falloff: self.bank_falloff, base: self.height_base },
                coverage: self.bank_coverage,
                ..d.banks
            },
            albedo: self.albedo,
            phase_g: self.phase_g,
            ambient: self.ambient,
            wind,
            ..d
        }
    }
}

/// One precipitation kind at an intensity (0..1 nominal; more is heavier).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrecipTerm {
    pub kind: RecordId,
    pub intensity: f32,
}

/// Ground wind of a preset. Speed and direction wander slowly inside their
/// ranges (smooth value noise, [`WindTerms::WANDER_HOURS`]), so a long
/// spell of one weather isn't a frozen wind.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WindTerms {
    /// Mean speed range at the reference height (10 m), m/s.
    pub speed: [f32; 2],
    pub gustiness: f32,
    /// Gust cell size, meters. Discrete in blends (the heavier preset's),
    /// since `sf_wind` snaps it to lattice divisors.
    pub gust_length: f32,
    /// Direction the wind blows toward, degrees from +X toward +Z.
    pub direction_deg: f32,
    /// Wander around `direction_deg`, ± degrees (0 locks it).
    pub direction_jitter_deg: f32,
    /// How much of the climate's storm source applies, 0..1 (e.g. a storm
    /// blowing away from a volcano).
    pub storm: f32,
}

impl WindTerms {
    /// Lattice spacing of the speed and direction wander, game hours.
    pub const WANDER_HOURS: f64 = 3.0;
}

impl Default for WindTerms {
    fn default() -> Self {
        let w = Wind::default();
        Self {
            speed: [1.5, 3.5],
            gustiness: w.gustiness,
            gust_length: w.gust_length,
            direction_deg: 15.0,
            direction_jitter_deg: 60.0,
            storm: 0.0,
        }
    }
}

/// Surface accumulation per game hour (fractions of full coverage); the
/// surface system integrates them. Rates add across blended presets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SurfaceRates {
    pub wet: f32,
    pub puddle: f32,
    pub snow: f32,
    pub ash: f32,
    /// Drying (wet and puddles) per hour.
    pub drying: f32,
}

/// Lightning of a preset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lightning {
    /// Mean strikes per game hour (capped at [`crate::LIGHTNING_CAP_PER_HOUR`]).
    pub per_hour: f32,
    /// Flash intensity scale.
    pub flash: f32,
    /// Flash fade-out, real seconds (a visual: it doesn't follow the game clock).
    pub fade_s: f32,
    /// Strike distance range from the camera, meters.
    pub distance_m: [f32; 2],
    /// Thunder sounds (one is picked per strike).
    pub thunder: Vec<RecordId>,
}

impl Default for Lightning {
    fn default() -> Self {
        Self { per_hour: 12.0, flash: 1.0, fade_s: 0.25, distance_m: [300.0, 6_000.0], thunder: Vec::new() }
    }
}

/// A looping sound while the weather (or climate) is present.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cue {
    pub sound: RecordId,
    pub volume: f32,
}

/// One weather type.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WeatherPreset {
    pub clouds: CloudTerms,
    pub sky: SkyTerms,
    pub fog: FogTerms,
    pub precipitation: Vec<PrecipTerm>,
    pub wind: WindTerms,
    pub surface: SurfaceRates,
    pub lightning: Option<Lightning>,
    pub cues: Vec<Cue>,
    /// Gameplay tags with a strength (rules read them).
    pub tags: Vec<(RecordId, f32)>,
    pub transition: TransitionTiming,
}

/// Region weather: which presets occur, how often the weather changes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Climate {
    /// Relative chance of each preset at a change (needn't sum to 1).
    pub weights: Vec<(RecordId, f32)>,
    /// Range of time between changes, game hours.
    pub change_interval_hours: (f32, f32),
    /// Duration of a transition, game hours (clamped to the minimum interval).
    pub transition_hours: f32,
    /// A wind source that presets with `wind.storm > 0` switch on.
    pub storm_source: Option<WindSource>,
    /// Ambient loops of the region (played under the weather's own cues).
    pub cues: Vec<Cue>,
}

impl Default for Climate {
    fn default() -> Self {
        Self {
            weights: Vec::new(),
            change_interval_hours: (8.0, 16.0),
            transition_hours: 1.5,
            storm_source: None,
            cues: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecipLook {
    #[default]
    Streak,
    Flake,
    Dust,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceChannel {
    #[default]
    None,
    Wet,
    Snow,
    Ash,
}

/// A precipitation kind (rain, snow, ash, ...). Kinds crossfade, never morph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrecipKind {
    pub look: PrecipLook,
    /// m/s.
    pub fall_speed: f32,
    /// Streak/flake size range, meters.
    pub size: [f32; 2],
    /// VFS path of the texture.
    pub texture: String,
    pub splash: Option<String>,
    pub accumulates: SurfaceChannel,
    /// Extinction at intensity 1, 1/m (heavy rain ≈ 2e-3: about 2 km
    /// visibility; snow and ash much more). Feeds `visibility_m`.
    pub extinction: f32,
}

impl Default for PrecipKind {
    fn default() -> Self {
        Self {
            look: PrecipLook::Streak,
            fall_speed: 7.0,
            size: [0.002, 0.004],
            texture: String::new(),
            splash: None,
            accumulates: SurfaceChannel::Wet,
            extinction: 2e-3,
        }
    }
}
