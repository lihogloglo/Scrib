//! A generic, data-driven game calendar (clock.md "Calendar (C3)").
//!
//! The calendar is content: one [`Calendar`] record (kind [`CALENDAR_KIND`], id [`CALENDAR_ID`],
//! a later definition replaces it, so a mod can change the calendar) with the day length, months
//! (festival and intercalary days are short months kept out of the week), an optional leap rule
//! (the Gregorian rule generalised), weekdays, eras, seasons and the epoch (the date of instant 0
//! at 00:00). The clock's [`GameInstant`] stays the only state: [`CalendarView`] turns an instant
//! into a [`Date`] and back in O(1) of the instant (closed-form leap and week counts).
//!
//! Nothing here names a game: every game's calendar is a record. The framework [`Default`] is a
//! plain 12-month, 365-day, 7-day-week calendar from year 1, used only when no content defines
//! `main` ([`main_calendar`]).
//!
//! Non-goals (clock.md): hours other than 60 minutes of 60 s, several calendars at once, instants
//! before the epoch (`GameInstant` is unsigned).
//!
//! [`GameInstant`]: sf_ids::GameInstant

mod format;
mod view;

#[cfg(test)]
mod tests_record;
#[cfg(test)]
mod tests_view;

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordError, RecordId};

pub use view::{CalendarError, CalendarView, Date, DateTime};

/// Record kind of the calendar ([`Calendar`]).
pub const CALENDAR_KIND: &str = "sf.calendar";
/// Record id of the calendar the clock uses.
pub const CALENDAR_ID: &str = "main";

/// Registers [`CALENDAR_KIND`]. Call once before loading mods.
pub fn register_kinds(db: &mut RecordDb) -> Result<(), RecordError> {
    db.register_serde::<Calendar>(CALENDAR_KIND)
}

/// The content's `main` calendar, or the framework [`Default`] when no content defines it (or
/// the kind isn't registered).
pub fn main_calendar(db: &RecordDb) -> Calendar {
    db.get::<Calendar>(&RecordId::new(CALENDAR_ID)).cloned().unwrap_or_default()
}

/// A game calendar (record [`CALENDAR_KIND`]). Validated by [`CalendarView::new`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Calendar {
    /// Hours in a day (hours are 60 minutes of 60 s); need not be whole.
    #[serde(default = "default_hours_per_day")]
    pub hours_per_day: f32,
    /// The months in year order (at least one).
    pub months: Vec<Month>,
    /// The leap rule; `None`: every year has the same length and `leap_days` are unused.
    #[serde(default)]
    pub leap: Option<Leap>,
    /// Weekday names in week order; empty: no weeks.
    #[serde(default)]
    pub weekdays: Vec<String>,
    /// Named spans of the absolute year, sorted by strictly increasing `start_year`; empty: plain
    /// years.
    #[serde(default)]
    pub eras: Vec<Era>,
    /// Season start dates in year order; the last runs into the first of the next year. Empty:
    /// no seasons.
    #[serde(default)]
    pub seasons: Vec<Season>,
    /// The date of instant 0 at 00:00, and where a new game's clock starts.
    pub epoch: Epoch,
}

fn default_hours_per_day() -> f32 {
    24.0
}

fn default_true() -> bool {
    true
}

/// A month, or a festival / intercalary day as a short month.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Month {
    pub name: String,
    /// Days in a common year. May be 0 when `leap_days` > 0: a month only leap years have.
    pub days: u32,
    /// Days a leap year adds to this month.
    #[serde(default)]
    pub leap_days: u32,
    /// `false`: its days are out of the week (no weekday; the week resumes after it).
    #[serde(default = "default_true")]
    pub in_week: bool,
}

/// The leap rule, the Gregorian one generalised: with `r = year - offset`, a year is leap when
/// `r % every == 0 && (r % skip_every != 0 || r % restore_every == 0)`. A 0 `skip_every` (or
/// `restore_every`) means no such exception. Gregorian: `every: 4, skip_every: 100,
/// restore_every: 400, offset: 0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leap {
    /// At least 1.
    pub every: u32,
    #[serde(default)]
    pub skip_every: u32,
    #[serde(default)]
    pub restore_every: u32,
    #[serde(default)]
    pub offset: i64,
}

/// A named span of the absolute year, from `start_year` to the next era's start.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Era {
    pub name: String,
    pub start_year: i64,
}

/// A season starting on `day` (1-based) of `month` (0-based) every year.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Season {
    pub name: String,
    pub month: u32,
    pub day: u32,
}

/// The date of instant 0 at 00:00.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Epoch {
    pub year: i64,
    /// 0-based.
    pub month: u32,
    /// 1-based.
    pub day: u32,
    /// The epoch day's weekday (an index into `weekdays`); `None` when the content doesn't know
    /// it, and then no date has a weekday.
    #[serde(default)]
    pub weekday: Option<u32>,
    /// The hour a new game's clock starts at on the epoch day ([`CalendarView::start_instant`]):
    /// content, not calendar state.
    #[serde(default)]
    pub start_hour: f32,
}

impl Default for Calendar {
    /// A plain calendar: 12 months, 365 days (no leap), a 7-day week, 24-hour days, no eras or
    /// seasons, epoch 1 of the first month of year 1 (the first weekday), new games at 00:00.
    fn default() -> Self {
        const MONTHS: [(&str, u32); 12] = [
            ("January", 31),
            ("February", 28),
            ("March", 31),
            ("April", 30),
            ("May", 31),
            ("June", 30),
            ("July", 31),
            ("August", 31),
            ("September", 30),
            ("October", 31),
            ("November", 30),
            ("December", 31),
        ];
        const WEEKDAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        Self {
            hours_per_day: default_hours_per_day(),
            months: MONTHS
                .iter()
                .map(|&(name, days)| Month { name: name.to_owned(), days, leap_days: 0, in_week: true })
                .collect(),
            leap: None,
            weekdays: WEEKDAYS.iter().map(|&w| w.to_owned()).collect(),
            eras: Vec::new(),
            seasons: Vec::new(),
            epoch: Epoch { year: 1, month: 0, day: 1, weekday: Some(0), start_hour: 0.0 },
        }
    }
}
