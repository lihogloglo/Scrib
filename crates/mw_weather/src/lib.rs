//! Morrowind weather adapter (Bevy-free, weather-adapter.md).
//!
//! Turns the original data into `sf_weather` records:
//! - the ten weather types and `Morrowind.ini [Weather*]` → [`WeatherPreset`]s
//!   `mw_weather/<type>`;
//! - `REGN` regions → [`Climate`]s `mw_region/<region id>` (weights = the
//!   region's weather chances, in `REGN` index order);
//! - precipitation → [`PrecipKind`]s `mw_precip/{rain,snow,ash,blight}`.
//!
//! [`MwWeather::define_into`] puts them in a synthetic record source
//! ([`SOURCE_NAME`]), so record mods patch them field by field like any
//! other content (everything is a mod).
//!
//! As built (we2a, we2b, we2c1): weights, change interval, transition duration,
//! precipitation kinds and intensities with their start thresholds, sound
//! cues, lightning rate, flash fade and thunder sounds, the blight tag; clouds,
//! sky haze and dust, fog and wind ([`visual`]), the Red Mountain storm source;
//! region sounds as one-shot ambience; the `MWScript` weather functions
//! ([`script`]).

mod ini;
pub mod script;
pub mod visual;

use mw_install::MorrowindIni;
use sf_records::{RecordDb, RecordError, RecordId, SourceId};
use sf_weather::{
    Climate, Cue, Lightning, OneShotSet, PrecipKind, PrecipLook, PrecipTerm, SurfaceChannel,
    SurfaceRates, WeatherPreset, Window,
};
use tes3::esp::{GlobalValue, GlobalVariable, Region, WeatherChances};

use crate::ini::WeatherIni;

/// Name of the synthetic record source the generated records live in.
pub const SOURCE_NAME: &str = "Morrowind.ini weather";

/// Morrowind's default `timescale` global (game seconds per real second).
pub const DEFAULT_TIMESCALE: f32 = 30.0;

/// Meters per Morrowind unit (same as `mw_world::convert::UNIT_METERS_F64`).
const UNIT_METERS: f32 = 0.9144 / 64.0;

/// Gameplay tag for blight disease exposure (rules read it, M7).
pub const BLIGHT_EXPOSURE_TAG: &str = "mw_tag/blight_exposure";

/// The weather types, in `REGN` weather-chance index order (Snow and
/// Blizzard are Bloodmoon's).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Foggy,
    Overcast,
    Rain,
    Thunderstorm,
    Ashstorm,
    Blight,
    Snow,
    Blizzard,
}

impl WeatherType {
    pub const ALL: [Self; 10] = [
        Self::Clear,
        Self::Cloudy,
        Self::Foggy,
        Self::Overcast,
        Self::Rain,
        Self::Thunderstorm,
        Self::Ashstorm,
        Self::Blight,
        Self::Snow,
        Self::Blizzard,
    ];

    /// Index in `REGN` and in `MWScript` (`ChangeWeather`, `GetCurrentWeather`).
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// The name used by `Morrowind.ini` sections (`[Weather <name>]`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Cloudy => "Cloudy",
            Self::Foggy => "Foggy",
            Self::Overcast => "Overcast",
            Self::Rain => "Rain",
            Self::Thunderstorm => "Thunderstorm",
            Self::Ashstorm => "Ashstorm",
            Self::Blight => "Blight",
            Self::Snow => "Snow",
            Self::Blizzard => "Blizzard",
        }
    }

    /// The preset record id, `mw_weather/<name>` (lowercase).
    pub fn preset_id(self) -> RecordId {
        RecordId::new(&format!("mw_weather/{}", self.name()))
    }

    fn section(self) -> String {
        format!("Weather {}", self.name())
    }

    /// This type's chance in a region (percent).
    pub fn chance(self, c: &WeatherChances) -> u8 {
        match self {
            Self::Clear => c.clear,
            Self::Cloudy => c.cloudy,
            Self::Foggy => c.foggy,
            Self::Overcast => c.overcast,
            Self::Rain => c.rain,
            Self::Thunderstorm => c.thunder,
            Self::Ashstorm => c.ash,
            Self::Blight => c.blight,
            Self::Snow => c.snow,
            Self::Blizzard => c.blizzard,
        }
    }
}

/// The precipitation kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Precip {
    Rain,
    Snow,
    Ash,
    Blight,
}

impl Precip {
    pub const ALL: [Self; 4] = [Self::Rain, Self::Snow, Self::Ash, Self::Blight];

    /// The kind record id, `mw_precip/<name>`.
    pub fn id(self) -> RecordId {
        RecordId::new(match self {
            Self::Rain => "mw_precip/rain",
            Self::Snow => "mw_precip/snow",
            Self::Ash => "mw_precip/ash",
            Self::Blight => "mw_precip/blight",
        })
    }
}

/// The climate record id of a region, `mw_region/<region id>` (lowercase).
pub fn climate_id(region_id: &str) -> RecordId {
    RecordId::new(&format!("mw_region/{region_id}"))
}

/// Morrowind's `Thunder Frequency` (strikes per real minute) as strikes per
/// game hour (`sf_weather::Lightning::per_hour`): one game hour is
/// `60 / timescale` real minutes.
pub fn lightning_per_game_hour(per_real_minute: f32, timescale: f32) -> f32 {
    per_real_minute * 60.0 / timescale
}

/// Morrowind's `Transition Delta` (transitions per *real* second, the
/// reading of `OpenMW`'s `weather.cpp`) as a transition length in game hours.
pub fn transition_game_hours(delta_per_real_second: f32, timescale: f32) -> Option<f32> {
    (delta_per_real_second > 0.0).then(|| timescale / (delta_per_real_second * 3600.0))
}

/// The weather records generated from the content and the ini.
#[derive(Clone, Debug, PartialEq)]
pub struct MwWeather {
    /// One per [`WeatherType`], in index order.
    pub presets: Vec<(RecordId, WeatherPreset)>,
    /// One per `REGN` record, sorted by id.
    pub climates: Vec<(RecordId, Climate)>,
    /// One per [`Precip`].
    pub precip_kinds: Vec<(RecordId, PrecipKind)>,
    /// The `timescale` global used for the real-time conversions.
    pub timescale: f32,
    /// Hash of every ini value read (vanilla where absent) and the time
    /// scale: a bake step keyed on it re-runs only when they change.
    pub fingerprint: u64,
}

impl MwWeather {
    /// Builds the records from the loaded content (`REGN`, the `timescale`
    /// global; kinds registered by `mw_records`) and the ini (`None`, or
    /// missing keys: vanilla values).
    pub fn build(db: &RecordDb, ini: Option<&MorrowindIni>) -> Self {
        let mut w = WeatherIni::new(ini);
        let timescale = timescale(db);
        w.fold(&timescale.to_bits().to_le_bytes());

        let precip_kinds = precip_kinds(&mut w);
        let mut transitions = [None; WeatherType::ALL.len()];
        let presets = WeatherType::ALL
            .iter()
            .map(|&ty| {
                transitions[ty.index()] = w
                    .f32(&ty.section(), "Transition Delta")
                    .and_then(|d| transition_game_hours(d, timescale));
                (ty.preset_id(), preset(ty, &mut w, timescale))
            })
            .collect();

        let hours = w
            .f32("Weather", "Hours Between Weather Changes")
            .filter(|h| *h > 0.0)
            .unwrap_or(20.0);
        // Region sounds: one draw per environmental-sound interval (real seconds).
        let sound_interval_s = [
            w.f32("Weather", "Minimum Time Between Environmental Sounds")
                .unwrap_or(1.0),
            w.f32("Weather", "Maximum Time Between Environmental Sounds")
                .unwrap_or(5.0),
        ];
        let mut climates: Vec<_> = db
            .iter::<Region>()
            .map(|(_, r)| climate(r, hours, &transitions, sound_interval_s))
            .collect();
        climates.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            presets,
            climates,
            precip_kinds,
            timescale,
            fingerprint: w.fingerprint(),
        }
    }

    /// Registers the weather kinds and defines every record in a new source
    /// [`SOURCE_NAME`] (call it right after the content files, before
    /// record mods, which then patch these records). Returns the source.
    pub fn define_into(
        &self,
        db: &mut RecordDb,
        depends_on: &[SourceId],
    ) -> Result<SourceId, RecordError> {
        register_kinds(db)?;
        let source = db.add_source(SOURCE_NAME, depends_on);
        self.define_in(db, source)?;
        Ok(source)
    }

    /// Defines every record in an existing `source` whose kinds are already
    /// registered ([`register_kinds`]): the mod pipeline's path, where
    /// `mw_mods`' synthesized `morrowind-weather` mod owns the source.
    pub fn define_in(&self, db: &mut RecordDb, source: SourceId) -> Result<(), RecordError> {
        for (id, kind) in &self.precip_kinds {
            db.define(source, id.clone(), kind.clone())?;
        }
        for (id, preset) in &self.presets {
            db.define(source, id.clone(), preset.clone())?;
        }
        for (id, climate) in &self.climates {
            db.define(source, id.clone(), climate.clone())?;
        }
        Ok(())
    }
}

/// Registers the `weather.*` record kinds (`sf_weather::register_kinds`).
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    sf_weather::register_kinds(db)
}

/// The weather type a preset record id names (`mw_weather/<type>`).
pub fn weather_type_of(preset: &RecordId) -> Option<WeatherType> {
    WeatherType::ALL
        .into_iter()
        .find(|t| t.preset_id() == *preset)
}

fn timescale(db: &RecordDb) -> f32 {
    let value = db
        .get::<GlobalVariable>(&RecordId::new("timescale"))
        .map(|g| match g.value {
            GlobalValue::Float(f) => f,
            #[allow(clippy::cast_precision_loss)]
            GlobalValue::Long(l) => l as f32,
            GlobalValue::Short(s) => f32::from(s),
        });
    value
        .filter(|t| t.is_finite() && *t > 0.0)
        .unwrap_or(DEFAULT_TIMESCALE)
}

fn precip_kinds(w: &mut WeatherIni<'_>) -> Vec<(RecordId, PrecipKind)> {
    // `Precip Gravity` read as the fall speed in units per second (575 ≈ 8.2 m/s,
    // a heavy raindrop's terminal velocity); snow falls `Snow Gravity Scale` as fast.
    let rain_fall = w
        .f32("Weather", "Precip Gravity")
        .filter(|g| *g > 0.0)
        .unwrap_or(575.0)
        * UNIT_METERS;
    let snow_scale = w
        .f32("Weather", "Snow Gravity Scale")
        .filter(|s| *s > 0.0)
        .unwrap_or(0.1);
    let dust = PrecipKind {
        look: PrecipLook::Dust,
        fall_speed: 1.0,
        size: [0.0005, 0.002],
        accumulates: SurfaceChannel::Ash,
        // ≈ 450 m visibility in a full storm (the F2 ash fog).
        extinction: 8e-3,
        ..PrecipKind::default()
    };
    Precip::ALL
        .iter()
        .map(|&p| {
            let kind = match p {
                Precip::Rain => PrecipKind {
                    fall_speed: rain_fall,
                    ..PrecipKind::default()
                },
                Precip::Snow => PrecipKind {
                    look: PrecipLook::Flake,
                    fall_speed: (rain_fall * snow_scale).max(0.3),
                    size: [0.004, 0.012],
                    accumulates: SurfaceChannel::Snow,
                    extinction: 6e-3,
                    ..PrecipKind::default()
                },
                // Blight is ash tinted red; the tint is fog albedo (we2b).
                Precip::Ash | Precip::Blight => dust.clone(),
            };
            (p.id(), kind)
        })
        .collect()
}

/// Thunderstorm's `Max Raindrops`: rain intensity 1.
const REFERENCE_RAINDROPS: f32 = 650.0;
/// Snowflakes at snow intensity 1: Blizzard (no `Max Snowflakes` key, a
/// particle mesh in the original) counts as twice vanilla Snow's 750.
const REFERENCE_SNOWFLAKES: f32 = 1500.0;

fn preset(ty: WeatherType, w: &mut WeatherIni<'_>, timescale: f32) -> WeatherPreset {
    let section = ty.section();
    let mut p = WeatherPreset::default();

    // Precipitation: kind, intensity, and the transition fraction it starts at.
    let using_precip = w.f32(&section, "Using Precip").is_some_and(|u| u != 0.0);
    let precip = match ty {
        WeatherType::Snow => {
            let flakes = w.f32(&section, "Max Snowflakes").unwrap_or(750.0);
            Some((
                Precip::Snow,
                flakes / REFERENCE_SNOWFLAKES,
                w.f32(&section, "Snow Threshold"),
            ))
        }
        WeatherType::Blizzard => Some((Precip::Snow, 1.0, w.f32(&section, "Storm Threshold"))),
        WeatherType::Ashstorm => Some((Precip::Ash, 1.0, w.f32(&section, "Storm Threshold"))),
        WeatherType::Blight => Some((Precip::Blight, 1.0, w.f32(&section, "Storm Threshold"))),
        _ if using_precip => {
            let reference = w
                .f32("Weather Thunderstorm", "Max Raindrops")
                .filter(|r| *r > 0.0)
                .unwrap_or(REFERENCE_RAINDROPS);
            let drops = w.f32(&section, "Max Raindrops").unwrap_or(reference);
            Some((
                Precip::Rain,
                drops / reference,
                w.f32(&section, "Rain Threshold"),
            ))
        }
        _ => None,
    };
    if let Some((kind, intensity, threshold)) = precip {
        let intensity = intensity.max(0.0);
        p.precipitation.push(PrecipTerm {
            kind: kind.id(),
            intensity,
        });
        if let Some(t) = threshold {
            p.transition.precipitation = Window::new(t.clamp(0.0, 0.95), 1.0);
        }
        p.surface = match kind {
            Precip::Rain => SurfaceRates {
                wet: 2.0 * intensity,
                puddle: 0.5 * intensity,
                ..SurfaceRates::default()
            },
            Precip::Snow => SurfaceRates {
                snow: 0.5 * intensity,
                ..SurfaceRates::default()
            },
            Precip::Ash | Precip::Blight => SurfaceRates {
                ash: 0.25 * intensity,
                ..SurfaceRates::default()
            },
        };
    } else {
        p.surface.drying = 0.25;
    }

    // Looping sounds (`SOUN` ids).
    for key in ["Ambient Loop Sound ID", "Rain Loop Sound ID"] {
        if let Some(sound) = w.str(&section, key) {
            let sound = RecordId::new(sound);
            if !p.cues.iter().any(|c| c.sound == sound) {
                p.cues.push(Cue { sound, volume: 1.0 });
            }
            // A weather with its own ambient loop (ash, blight, blizzard)
            // replaces the region's ambience (weather-adapter.md "Regions").
            if key == "Ambient Loop Sound ID" {
                p.climate_ambience = 0.0;
            }
        }
    }

    // Lightning: real-time frequency → per game hour.
    if let Some(f) = w.f32(&section, "Thunder Frequency").filter(|f| *f > 0.0) {
        let thunder = (0..4)
            .filter_map(|i| w.str(&section, &format!("Thunder Sound ID {i}")))
            .map(RecordId::new)
            .collect();
        p.lightning = Some(Lightning {
            per_hour: lightning_per_game_hour(f, timescale),
            thunder,
            ..Lightning::default()
        });
    }

    if let Some(chance) = w.f32(&section, "Disease Chance").filter(|c| *c > 0.0) {
        p.tags.push((RecordId::new(BLIGHT_EXPOSURE_TAG), chance));
    }
    visual::apply(ty, &mut p, w);
    p
}

/// A region's `REGN` sounds (`SOUN` id, chance in percent per draw) as one
/// one-shot set; `None` when it has no audible sound.
fn region_sounds(region: &Region, interval_s: [f32; 2]) -> Option<OneShotSet> {
    let sounds: Vec<_> = region
        .sounds
        .iter()
        .filter(|(id, chance)| *chance > 0 && !id.0.is_empty())
        .map(|(id, chance)| (RecordId::new(&id.0), f32::from(*chance) / 100.0))
        .collect();
    (!sounds.is_empty()).then_some(OneShotSet {
        sounds,
        interval_s,
        volume: 1.0,
    })
}

fn climate(
    region: &Region,
    hours: f32,
    transitions: &[Option<f32>; 10],
    sound_interval_s: [f32; 2],
) -> (RecordId, Climate) {
    let mut weights = Vec::new();
    let (mut weighted, mut total) = (0.0, 0.0);
    for ty in WeatherType::ALL {
        let chance = f32::from(ty.chance(&region.weather_chances));
        if chance > 0.0 {
            weights.push((ty.preset_id(), chance));
            if let Some(t) = transitions[ty.index()] {
                weighted += chance * t;
                total += chance;
            }
        }
    }
    if weights.is_empty() {
        weights.push((WeatherType::Clear.preset_id(), 1.0));
    }
    let defaults = Climate::default();
    let climate = Climate {
        weights,
        // ± 25 % so regions don't all change together.
        change_interval_hours: (hours * 0.75, hours * 1.25),
        // sf_weather has one transition length per climate: the chance-weighted
        // mean of the presets' (Morrowind's is per target weather).
        // Ash and blight blow off Red Mountain (the source fades with distance).
        storm_source: (f32::from(region.weather_chances.ash)
            + f32::from(region.weather_chances.blight)
            > 0.0)
            .then(visual::red_mountain_storm),
        transition_hours: if total > 0.0 {
            weighted / total
        } else {
            defaults.transition_hours
        },
        one_shots: region_sounds(region, sound_interval_s)
            .into_iter()
            .collect(),
        ..defaults
    };
    (climate_id(&region.id), climate)
}

#[cfg(test)]
mod tests;
