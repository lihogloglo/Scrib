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
