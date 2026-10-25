//! `MWScript` weather functions as `sf_weather` calls (weather-adapter.md
//! "Scripting"). The `MWScript` → Luau layer (M5)
//! calls these with the game clock (`now`, game hours); they act on the
//! save state ([`WeatherState`]), so a script's change is saved and replays
//! deterministically.
//!
//! `GetMasserPhase`, `GetSecundaPhase`, `TurnMoonRed` and `TurnMoonWhite`
//! are sky functions (the moons), not weather: they don't map here.

use sf_weather::{WeatherBook, WeatherFrame, WeatherState};

use crate::visual::WIND_SPEED_RANGE;
use crate::{WeatherType, climate_id};

/// `ChangeWeather "<region>" <type>`: the region turns to weather `type`
/// (`REGN` index order) from `now`, over the climate's transition, and
/// stays until the first natural change one minimum interval later.
/// `false` (no change) for an index outside the ten types.
pub fn change_weather(state: &mut WeatherState, region: &str, weather: i32, now: f64) -> bool {
    let Some(ty) = usize::try_from(weather)
        .ok()
        .and_then(WeatherType::from_index)
    else {
        return false;
    };
    state.force(&climate_id(region), &ty.preset_id(), now, None, None);
    true
}

/// `ModRegion "<region>" c0 c1 …`: the region's weather chances (percent,
/// `REGN` index order; missing trailing ones are 0, as with 8 values before
/// Bloodmoon) for weather changes from `now` on. The weather in progress
/// stays until the next change. All zero means Clear, like a `REGN` with
/// no chances.
pub fn mod_region(state: &mut WeatherState, region: &str, chances: &[f32], now: f64) {
    let mut weights: Vec<_> = WeatherType::ALL
        .iter()
        .zip(chances)
        .filter(|(_, c)| **c > 0.0)
        .map(|(ty, c)| (ty.preset_id(), *c))
        .collect();
    if weights.is_empty() {
        weights.push((WeatherType::Clear.preset_id(), 1.0));
    }
    state.set_weights(&climate_id(region), now, weights);
}

/// `GetCurrentWeather`: the weather type the region is in, or turning into
/// (so a script sees its own `ChangeWeather` at once; the original reports
/// the old weather until the transition ends). `None` for a region without
/// a climate or a preset that isn't one of the ten types (a mod's).
pub fn current_weather(
    state: &WeatherState,
    book: &WeatherBook,
    region: &str,
    now: f64,
) -> Option<WeatherType> {
    let climate = book.climate(&climate_id(region))?;
    let preset = book.preset_id(state.current(book, climate, now)?);
    WeatherType::ALL
        .into_iter()
        .find(|ty| ty.preset_id() == *preset)
}

/// `GetWindSpeed`: the ini's `Wind Speed` scale (0..1 for vanilla weathers)
/// recovered from the frame's ground wind, the inverse of the preset mapping
/// ([`WIND_SPEED_RANGE`]). The wander (±25 %) and blends show through, as
/// they do in the wind itself.
pub fn wind_speed(frame: &WeatherFrame) -> f32 {
    let [lo, hi] = WIND_SPEED_RANGE;
    ((frame.wind().velocity.length() - lo) / (hi - lo)).max(0.0)
}
