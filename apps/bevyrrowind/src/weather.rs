//! `bevyrrowind weather ...`: inspect the weather records (`mw_weather`,
//! loaded through the `morrowind-weather` mod) without running the game.

use clap::Subcommand;
use mw_install::MorrowindInstall;
use mw_weather::{climate_id, weather_type_of};
use sf_records::RecordId;
use sf_weather::{WeatherBook, WeatherState};

use crate::Result;

#[derive(Subcommand)]
pub enum WeatherCommand {
    /// The weather changes a region will go through (`WeatherState::forecast`), one row per change slot.
    Forecast {
        /// `REGN` id, e.g. "Bitter Coast Region".
        region: String,
        /// Game days to list.
        #[arg(long, default_value_t = 3.0)]
        days: f64,
        /// World weather seed (the save's `WeatherState::seed`).
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Start, in game hours since the weather clock's zero.
        #[arg(long, default_value_t = 0.0)]
        from: f64,
    },
}

pub fn run(install: &MorrowindInstall, command: WeatherCommand) -> Result {
    match command {
        WeatherCommand::Forecast {
            region,
            days,
            seed,
            from,
        } => forecast(install, &region, days, seed, from),
    }
}

fn forecast(install: &MorrowindInstall, region: &str, days: f64, seed: u64, from: f64) -> Result {
    let (_, content) = crate::load(install, false)?;
    let book = WeatherBook::from_db(&content.db);
    let Some(climate) = book.climate(&climate_id(region)) else {
        let known: Vec<String> = content
            .db
            .iter::<tes3::esp::Region>()
            .map(|(_, r)| r.id.clone())
            .collect();
        return Err(format!(
            "no climate for region {region:?}; regions: {}",
            known.join(", ")
        )
        .into());
    };
    let record = book.climate_record(climate);
    let until = from + days.max(0.0) * 24.0;
    // Enough entries to reach `until` at the shortest change interval.
    let shortest = f64::from(record.change_interval_hours.0.max(0.1));
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "small positive count"
    )]
    let count = ((until - from) / shortest).ceil() as usize + 2;
    let entries = WeatherState::new(seed).forecast(&book, climate, from, count);

    let weights: Vec<String> = record
        .weights
        .iter()
        .map(|(id, w)| format!("{} {w}", preset_name(id)))
        .collect();
    println!(
        "{} (seed {seed}): {}",
        book.climate_id(climate).as_str(),
        weights.join(", ")
    );
    println!(
        "changes every {:.1}-{:.1} h, transitions {:.2} h\n",
        record.change_interval_hours.0, record.change_interval_hours.1, record.transition_hours
    );
    println!("{:<12} {:<14} {:>12}", "starts", "weather", "transition");
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for e in entries.iter().take_while(|e| e.start < until) {
        let name = preset_name(book.preset_id(e.preset));
        // The first entry is the weather already running at `from`.
        let start = e.start.max(from);
        println!(
            "{:<12} {:<14} {:>10.2} h{}",
            clock(start),
            name,
            e.transition_hours,
            if e.forced { "  (forced)" } else { "" }
        );
        *counts.entry(name).or_default() += 1;
    }
    let summary: Vec<String> = counts.iter().map(|(n, c)| format!("{n} {c}")).collect();
    println!("\n{}", summary.join(", "));
    Ok(())
}

/// The weather type's name (`Thunderstorm`), else the preset id (mod-added presets).
fn preset_name(id: &RecordId) -> String {
    weather_type_of(id).map_or_else(|| id.as_str().to_owned(), |t| t.name().to_owned())
}

/// `day N hh:mm` from game hours.
fn clock(hours: f64) -> String {
    #[expect(clippy::cast_possible_truncation, reason = "game hours fit an i64")]
    let minutes = (hours * 60.0).round() as i64;
    let (day, rem) = (minutes.div_euclid(24 * 60), minutes.rem_euclid(24 * 60));
    format!("day {day} {:02}:{:02}", rem / 60, rem % 60)
}

// -- live weather (M6 WE3, weather-adapter.md "App") --------------------------------

/// Climate when the camera's cell has none (synthetic levels, cells
/// without a region): Seyda Neen's.
const FALLBACK_REGION: &str = "Bitter Coast Region";

/// `--weather` values: the Morrowind weather types, any case.
pub fn parse_weather_type(s: &str) -> std::result::Result<mw_weather::WeatherType, String> {
    mw_weather::WeatherType::ALL.into_iter().find(|t| t.name().eq_ignore_ascii_case(s.trim())).ok_or_else(|| {
        let names: Vec<_> = mw_weather::WeatherType::ALL.iter().map(|t| t.name().to_ascii_lowercase()).collect();
        format!("unknown weather {s:?}: {}", names.join(", "))
    })
}

/// What an exterior run needs for `sf_weather_bevy::WeatherPlugin`: the
/// records, the state (seed, `--weather` force), and the exterior grid's
/// climates (`mw_region/<CELL region>`).
pub struct WeatherSetup {
    pub book: std::sync::Arc<WeatherBook>,
    pub state: WeatherState,
    pub cells: std::collections::HashMap<(i32, i32), sf_weather::ClimateIx>,
    pub fallback: Option<sf_weather::ClimateIx>,
    /// The weather at the start (forced, or the start cell's forecast): the
    /// cloud genus and fog look until WE3 part 2 drives them from the frame.
    pub start: Option<mw_weather::WeatherType>,
}

impl WeatherSetup {
    /// `eye`: start camera (engine meters); `hours`: the clock in game hours.
    pub fn build(db: &sf_records::RecordDb, eye: glam::DVec3, hours: f64, force: Option<mw_weather::WeatherType>, seed: u64) -> Self {
        let book = WeatherBook::from_db(db);
        let cells: std::collections::HashMap<(i32, i32), sf_weather::ClimateIx> = db
            .iter::<tes3::esp::Cell>()
            .filter(|(_, c)| !c.is_interior())
            .filter_map(|(_, c)| Some((c.data.grid, book.climate(&climate_id(c.region.as_deref()?))?)))
            .collect();
        let fallback = book.climate(&climate_id(FALLBACK_REGION));
        let mut state = WeatherState::new(seed);
        if let Some(t) = force {
            // Every climate, from long before the start, held for good: a
            // debug flag, not a scripted change (no transition).
            let mut climates: Vec<_> = cells.values().copied().chain(fallback).collect();
            climates.sort_unstable();
            climates.dedup();
            for c in climates {
                state.force(book.climate_id(c), &t.preset_id(), hours - 1.0e4, Some(0.0), Some(1.0e7));
            }
        }
        let start_climate = cells.get(&grid_at(eye)).copied().or(fallback);
        let start = force.or_else(|| start_climate.and_then(|c| state.current(&book, c, hours)).and_then(|p| weather_type_of(book.preset_id(p))));
        Self { book: std::sync::Arc::new(book), state, cells, fallback, start }
    }
}

/// The exterior grid square under an engine position (meters, +Z south).
#[allow(clippy::cast_possible_truncation)]
pub fn grid_at(p: glam::DVec3) -> (i32, i32) {
    let cell = mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64;
    ((p.x / cell).floor() as i32, (-p.z / cell).floor() as i32)
}
