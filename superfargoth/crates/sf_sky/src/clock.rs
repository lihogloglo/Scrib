//! Calendar and the sky's view of the game clock.

use serde::{Deserialize, Serialize};
use sf_ids::GameInstant;
use sf_ids::clock::day_ms;

/// Length of a day and a year, in game time. Games pick their own; the
/// defaults are Earth's.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Calendar {
    /// Hours in one day. Solar noon is at half of this.
    pub hours_per_day: f32,
    /// Days in one year (the period of the seasons).
    pub days_per_year: f32,
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            hours_per_day: 24.0,
            days_per_year: 365.2422,
        }
    }
}

impl Calendar {
    /// Game milliseconds in one day (`sf_ids::clock::day_ms`): the day the game clock's views use.
    pub fn day_ms(&self) -> u64 {
        day_ms(self.hours_per_day)
    }
}

/// The sky's view of game time: a continuous count of days since the world's epoch.
///
/// `days.fract()` is the time of day (0 = midnight, 0.5 = noon); the integer part counts days.
/// Not a clock: the one game clock is `sf_ids::GameClock` (clock.md), and the sky builds this
/// view from its instant each frame ([`SkyClock::from_instant`]), exact to the millisecond.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkyClock {
    /// Days since the epoch (day 0, midnight).
    pub days: f64,
}

impl Default for SkyClock {
    /// Noon of day 0 (`GameClock::default`'s instant).
    fn default() -> Self {
        Self { days: 0.5 }
    }
}

impl SkyClock {
    /// `hour` of day `day` (both may be fractional).
    pub fn at(day: f64, hour: f64, calendar: &Calendar) -> Self {
        Self { days: day + hour / f64::from(calendar.hours_per_day) }
    }

    /// The view of the game clock's `instant` for `calendar`'s day length.
    pub fn from_instant(instant: GameInstant, calendar: &Calendar) -> Self {
        Self { days: instant.days(calendar.day_ms()) }
    }

    /// Time of day as a fraction in `[0, 1)`: 0 = midnight, 0.5 = noon.
    pub fn day_fraction(&self) -> f64 {
        self.days.rem_euclid(1.0)
    }

    /// Hour of the day in `[0, hours_per_day)`.
    #[expect(clippy::cast_possible_truncation, reason = "an hour fits f32")]
    pub fn hour(&self, calendar: &Calendar) -> f32 {
        (self.day_fraction() * f64::from(calendar.hours_per_day)) as f32
    }

    /// Day within the year, fractional, in `[0, days_per_year)`.
    pub fn day_of_year(&self, calendar: &Calendar) -> f64 {
        self.days.rem_euclid(f64::from(calendar.days_per_year))
    }
}
