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
    /// The weather at the start (forced, or the start cell's forecast), printed.
    pub start: Option<mw_weather::WeatherType>,
    /// `SOUN` id -> VFS sound file for `sf_audio_bevy` (filled by `load_sounds`).
    pub sounds: sf_audio::SoundBank,
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
        Self { book: std::sync::Arc::new(book), state, cells, fallback, start, sounds: sf_audio::SoundBank::default() }
    }

    /// The exterior's region raster for the world weather map (WE4c): the
    /// grid's climates over their bounding box plus [`RASTER_MARGIN`]
    /// squares of sea; squares without a region (sea) take the nearest
    /// region's climate (`ClimateRaster` dilation, owner decision WE4 1).
    /// Without any region cell: one square of the fallback climate.
    pub fn raster(&self) -> Option<sf_weather::ClimateRaster> {
        let cell = mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64;
        let band = sf_weather::BORDER_BAND_M;
        if self.cells.is_empty() {
            let origin = glam::DVec2::new(0.0, -cell);
            return sf_weather::ClimateRaster::new(origin, cell, 1, 1, &[self.fallback], band);
        }
        let (mut lo, mut hi) = ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN));
        for &(x, y) in self.cells.keys() {
            (lo, hi) = ((lo.0.min(x), lo.1.min(y)), (hi.0.max(x), hi.1.max(y)));
        }
        let (lo, hi) = ((lo.0 - RASTER_MARGIN, lo.1 - RASTER_MARGIN), (hi.0 + RASTER_MARGIN, hi.1 + RASTER_MARGIN));
        #[expect(clippy::cast_sign_loss, reason = "hi >= lo")]
        let (w, h) = ((hi.0 - lo.0 + 1) as u32, (hi.1 - lo.1 + 1) as u32);
        // Raster rows run along world +Z (south), so row 0 is the northmost grid row.
        let mut cells = vec![None; (w as usize) * (h as usize)];
        for (&(x, y), &c) in &self.cells {
            #[expect(clippy::cast_sign_loss, reason = "inside the box")]
            let i = (x - lo.0) as usize + (hi.1 - y) as usize * w as usize;
            cells[i] = Some(c);
        }
        let origin = glam::DVec2::new(f64::from(lo.0) * cell, -f64::from(hi.1 + 1) * cell);
        sf_weather::ClimateRaster::new(origin, cell, w, h, &cells, band)
    }
}

/// Sea squares around the exterior's region cells in the weather raster
/// (the map clamps to the edge squares past it).
const RASTER_MARGIN: i32 = 4;

/// The exterior grid square under an engine position (meters, +Z south).
#[allow(clippy::cast_possible_truncation)]
pub fn grid_at(p: glam::DVec3) -> (i32, i32) {
    let cell = mw_world::convert::CELL_UNITS * mw_world::convert::UNIT_METERS_F64;
    ((p.x / cell).floor() as i32, (-p.z / cell).floor() as i32)
}

// -- live control (BRP `bevyrrowind/set weather`, the C key) ------------------------

/// How long a live forced weather holds, game hours (until `auto`).
const LIVE_HOLD_HOURS: f32 = 1.0e6;

/// Forces `ty` on every climate from `now` over `transition_hours` (`None`:
/// the climate's own; 0 skips), held until the next call; `None` returns to
/// the forecast: each climate glides to the weather its forecast has at
/// `now` (a force of that weather, released at the climate's next natural
/// change). Overrides are the save state, so the change replays.
pub(crate) fn force_live(
    book: &WeatherBook,
    state: &mut WeatherState,
    climates: &[sf_weather::ClimateIx],
    ty: Option<mw_weather::WeatherType>,
    now: f64,
    transition_hours: Option<f32>,
) {
    let natural = WeatherState {
        seed: state.seed,
        overrides: state.overrides.iter().filter(|o| !matches!(o, sf_weather::Override::Force { .. })).cloned().collect(),
    };
    for &c in climates {
        let climate = book.climate_id(c).clone();
        match ty {
            Some(t) => state.force(&climate, &t.preset_id(), now, transition_hours, Some(LIVE_HOLD_HOURS)),
            None => {
                if let Some(p) = natural.current(book, c, now) {
                    state.force(&climate, &book.preset_id(p).clone(), now, transition_hours, None);
                }
            }
        }
    }
}

/// The weather at a climate for `bevyrrowind/status` and the C key's note.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[cfg_attr(feature = "remote", derive(schemars::JsonSchema))]
pub(crate) struct LiveWeather {
    /// The weather in force or being turned into.
    pub current: String,
    /// How far the current weather has come in, per channel (1: done):
    /// clouds lead, fog follows.
    pub clouds: f32,
    pub fog: f32,
    /// A forced weather (`--weather`, a live force or a script).
    pub forced: bool,
    /// The next change: weather and game hours until it starts.
    pub next: Option<(String, f64)>,
}

pub(crate) fn live_weather(book: &WeatherBook, state: &WeatherState, climate: sf_weather::ClimateIx, now: f64) -> Option<LiveWeather> {
    let name = |p: sf_weather::PresetIx| {
        let id = book.preset_id(p);
        weather_type_of(id).map_or_else(|| id.as_str().to_owned(), |t| t.name().to_ascii_lowercase())
    };
    let current = state.current(book, climate, now)?;
    let blend = state.blend(book, climate, now);
    let w = blend.iter().find(|(p, _)| *p == current).map_or([1.0; sf_weather::CHANNELS], |(_, w)| *w);
    let forecast = state.forecast(book, climate, now, 3);
    let forced = forecast.iter().take_while(|e| e.start <= now).last().is_some_and(|e| e.forced);
    let next = forecast.iter().find(|e| e.start > now).map(|e| (name(e.preset), e.start - now));
    Some(LiveWeather { current: name(current), clouds: w[sf_weather::CLOUDS], fog: w[sf_weather::FOG], forced, next })
}

impl WeatherSetup {
    /// Resolves the `SOUN` records to sound files in the VFS (mods' replacements win; a `.wav`
    /// may be replaced by an `.mp3` of the same name).
    pub fn load_sounds(&mut self, db: &sf_records::RecordDb, vfs: &sf_vfs::Vfs) {
        self.sounds = mw_weather::sound_bank(db, |raw| {
            let path = sf_vfs::VfsPath::new(raw).ok()?;
            vfs.resolve_preferring(&path, &["mp3", "wav"]).map(|(p, _)| p.as_str().to_owned())
        });
    }
}
