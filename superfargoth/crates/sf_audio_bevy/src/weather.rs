//! The weather bridge (`weather` feature): `sf_weather_bevy`'s [`WeatherSound`] messages become
//! [`BusSound`]s on the Weather bus, and the frame's region one-shot sets become [`OneShotSets`].
//! Behind a feature so the audio plugin builds and tests without the render crates
//! `sf_weather_bevy` depends on; the app turns it on and adds [`WeatherAudioPlugin`] next to the
//! weather plugin.

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use sf_audio::Bus;
use sf_weather_bevy::{CurrentWeather, WeatherRecords, WeatherSound};

use crate::{AudioSystems, BusSound, OneShotSet, OneShotSets};

/// Feeds the weather's sounds to the audio plugin (add both).
pub struct WeatherAudioPlugin;

impl Plugin for WeatherAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WeatherSound>()
            .add_message::<BusSound>()
            .init_resource::<OneShotSets>()
            .add_systems(Update, (weather_bus_sounds, region_one_shot_sets).before(AudioSystems));
    }
}

/// Weather loops and thunder, non-positional on the Weather bus (as before the split).
pub fn weather_bus_sounds(mut weather: MessageReader<WeatherSound>, mut out: MessageWriter<BusSound>) {
    for s in weather.read() {
        out.write(match s {
            WeatherSound::Loop { sound, volume, .. } => BusSound::Loop { sound: sound.clone(), volume: *volume, bus: Bus::Weather },
            WeatherSound::OneShot { sound, volume, .. } => BusSound::OneShot { sound: sound.clone(), volume: *volume, bus: Bus::Weather },
        });
    }
}

/// The current frame's region one-shot sets (key = climate << 16 | set, the old scheduler seed).
pub fn region_one_shot_sets(current: Option<Res<CurrentWeather>>, records: Option<Res<WeatherRecords>>, mut sets: ResMut<OneShotSets>) {
    let (Some(current), Some(records)) = (current, records) else { return };
    if !current.valid {
        if !sets.0.is_empty() {
            sets.0.clear();
        }
        return;
    }
    let next: Vec<OneShotSet> = current
        .frame
        .one_shots
        .iter()
        .filter_map(|o| {
            let set = records.0.climate_record(o.climate).one_shots.get(o.set)?;
            Some(OneShotSet {
                key: u64::from(o.climate.0) << 16 | o.set as u64,
                interval_s: set.interval_s,
                sounds: set.sounds.clone(),
                volume: o.volume,
            })
        })
        .collect();
    if sets.0 != next {
        sets.0 = next;
    }
}
