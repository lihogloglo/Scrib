//! Calendar and the time-of-day clock.

use serde::{Deserialize, Serialize};

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
    /// Game seconds in one day.
    pub fn seconds_per_day(&self) -> f64 {
        f64::from(self.hours_per_day) * 3600.0
    }
}

/// Game time as a continuous count of days since the world's epoch.
///
/// `days.fract()` is the time of day (0 = midnight, 0.5 = noon); the integer
/// part counts days. f64 keeps sub-millisecond resolution for millions of
/// days, so saves store this value as is.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkyClock {
    /// Days since the epoch (day 0, midnight).
    pub days: f64,
    /// Game seconds per real second. 1 is real time; many RPGs use 20–30.
    pub time_scale: f32,
    /// When set, [`SkyClock::advance`] does nothing.
    pub paused: bool,
}

impl Default for SkyClock {
    fn default() -> Self {
        Self {
            days: 0.5,
            time_scale: 30.0,
            paused: false,
        }
    }
}

impl SkyClock {
    /// A clock at `hour` of day `day` (both may be fractional).
    pub fn at(day: f64, hour: f64, calendar: &Calendar) -> Self {
        Self {
            days: day + hour / f64::from(calendar.hours_per_day),
            ..Self::default()
        }
    }

    /// Advances by `real_seconds` of real time, scaled by `time_scale`.
    pub fn advance(&mut self, real_seconds: f64, calendar: &Calendar) {
        if !self.paused {
            self.days += real_seconds * f64::from(self.time_scale) / calendar.seconds_per_day();
        }
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

    /// Jumps to `hour` of the current day, keeping the day count.
    pub fn set_hour(&mut self, hour: f64, calendar: &Calendar) {
        self.days = self.days.floor() + (hour / f64::from(calendar.hours_per_day)).rem_euclid(1.0);
    }

    /// Day within the year, fractional, in `[0, days_per_year)`.
    pub fn day_of_year(&self, calendar: &Calendar) -> f64 {
        self.days.rem_euclid(f64::from(calendar.days_per_year))
    }
}
