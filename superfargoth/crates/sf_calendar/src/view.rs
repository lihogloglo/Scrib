//! [`CalendarView`]: a validated [`Calendar`] with its date math precomputed (clock.md
//! "Calendar (C3)" Functions). Day counts are exact integers (`i128`, so no realistic calendar
//! overflows); the year of a day is estimated from the mean year and corrected with the
//! closed-form leap count, and weekdays come from the closed-form count of in-week days: O(1) in
//! the instant (O(months) inside a year).

use sf_ids::GameInstant;
use sf_ids::clock::{MS_PER_HOUR, MS_PER_MINUTE, day_ms, hours_ms};

use crate::Calendar;

/// Why a [`Calendar`] record is malformed ([`CalendarView::new`]).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CalendarError {
    #[error("hours_per_day must be finite and > 0")]
    HoursPerDay,
    #[error("a calendar needs at least one month")]
    NoMonths,
    #[error("month {month} has no days, not even in leap years")]
    EmptyMonth { month: usize },
    #[error("a common year has no days")]
    EmptyYear,
    #[error("a year has more than u32::MAX days")]
    YearTooLong,
    #[error("leap.every must be at least 1")]
    LeapEvery,
    #[error("the epoch date does not exist in the epoch year")]
    EpochDate,
    #[error("epoch weekday {weekday} is not below the {weekdays} weekdays")]
    EpochWeekday { weekday: u32, weekdays: usize },
    #[error("the epoch has a weekday but falls on a day out of the week")]
    EpochOutOfWeek,
    #[error("epoch.start_hour must be in 0..hours_per_day")]
    StartHour,
    #[error("era {era} does not start after the previous era")]
    ErasUnsorted { era: usize },
    #[error("season {season} starts on a date that no year has")]
    SeasonDate { season: usize },
    #[error("season {season} does not start after the previous season")]
    SeasonsUnsorted { season: usize },
}

/// A date and time of day ([`CalendarView::date`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Date {
    /// The absolute year (eras only name spans of it).
    pub year: i64,
    /// Index into the calendar's eras: the last with `start_year <= year`; `None` before the
    /// first era (or without eras).
    pub era: Option<usize>,
    /// `year - era.start_year + 1`, or `year` without an era.
    pub year_of_era: i64,
    /// 0-based.
    pub month: u32,
    /// 1-based.
    pub day: u32,
    /// 0-based.
    pub day_of_year: u32,
    /// `None` without weekdays, when the epoch's weekday is unknown, or on a day out of the week.
    pub weekday: Option<u32>,
    /// Index into the calendar's seasons; `None` without seasons.
    pub season: Option<usize>,
    pub hour: u32,
    pub minute: u32,
    /// Milliseconds into the minute (0..60 000): keeps `date` -> `instant_at` exact.
    pub ms: u32,
}

impl Date {
    /// The fields [`CalendarView::instant_at`] reads.
    pub const fn date_time(&self) -> DateTime {
        DateTime { year: self.year, month: self.month, day: self.day, hour: self.hour, minute: self.minute, ms: self.ms }
    }
}

/// A date and time of day to convert to an instant ([`CalendarView::instant_at`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub year: i64,
    /// 0-based.
    pub month: u32,
    /// 1-based.
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    /// Milliseconds into the minute.
    pub ms: u32,
}

/// The leap rule as divisors of `year - offset`: leap years counted as multiples of `every`,
/// minus multiples of `lcm(every, skip)`, plus multiples of `lcm(every, skip, restore)`.
#[derive(Clone, Copy, Debug)]
struct LeapTerms {
    offset: i128,
    every: i128,
    skip: i128,
    restore: i128,
    skip_lcm: Option<i128>,
    restore_lcm: Option<i128>,
}

/// A validated calendar and its precomputed counts. A Bevy `Resource` behind the `bevy` feature
/// (the one source of the sky's day and year length; clock.md "Sky in C3").
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::resource::Resource))]
pub struct CalendarView {
    cal: Calendar,
    day_ms: u64,
    /// Days of a common year, and what a leap year adds.
    common_days: i128,
    leap_extra: i128,
    /// The same for in-week days.
    common_week: i128,
    leap_week_extra: i128,
    leap: Option<LeapTerms>,
    /// Days, and in-week days, from the start of the epoch year to the epoch day.
    epoch_day: i128,
    epoch_week: i128,
}

impl CalendarView {
    /// Validates `cal` and precomputes its counts.
    pub fn new(cal: &Calendar) -> Result<Self, CalendarError> {
        if !cal.hours_per_day.is_finite() || cal.hours_per_day <= 0.0 {
            return Err(CalendarError::HoursPerDay);
        }
        if cal.months.is_empty() {
            return Err(CalendarError::NoMonths);
        }
        if let Some(month) = cal.months.iter().position(|m| m.days == 0 && m.leap_days == 0) {
            return Err(CalendarError::EmptyMonth { month });
        }
        let sum = |f: &dyn Fn(&crate::Month) -> u32| cal.months.iter().map(|m| i128::from(f(m))).sum::<i128>();
        let common_days = sum(&|m| m.days);
        let leap_extra = sum(&|m| m.leap_days);
        let common_week = sum(&|m| if m.in_week { m.days } else { 0 });
        let leap_week_extra = sum(&|m| if m.in_week { m.leap_days } else { 0 });
        if common_days == 0 {
            return Err(CalendarError::EmptyYear);
        }
        if common_days + leap_extra > i128::from(u32::MAX) {
            return Err(CalendarError::YearTooLong);
        }
        let leap = match cal.leap {
            None => None,
            Some(l) if l.every == 0 => return Err(CalendarError::LeapEvery),
            Some(l) => {
                let every = i128::from(l.every);
                let skip_lcm = (l.skip_every > 0).then(|| lcm(every, i128::from(l.skip_every)));
                let restore_lcm = skip_lcm.filter(|_| l.restore_every > 0).map(|s| lcm(s, i128::from(l.restore_every)));
                Some(LeapTerms {
                    offset: i128::from(l.offset),
                    every,
                    skip: i128::from(l.skip_every),
                    restore: i128::from(l.restore_every),
                    skip_lcm,
                    restore_lcm,
                })
            }
        };
        let mut view = Self {
            cal: cal.clone(),
            day_ms: day_ms(cal.hours_per_day),
            common_days,
            leap_extra,
            common_week,
            leap_week_extra,
            leap,
            epoch_day: 0,
            epoch_week: 0,
        };
        view.validate_epoch()?;
        view.validate_eras_seasons()?;
        Ok(view)
    }

    fn validate_epoch(&mut self) -> Result<(), CalendarError> {
        let e = self.cal.epoch;
        let len = self.days_in_month(e.year, e.month).ok_or(CalendarError::EpochDate)?;
        if e.day == 0 || e.day > len {
            return Err(CalendarError::EpochDate);
        }
        let leap = self.is_leap(e.year);
        let (before, week_before) = self.before_month(leap, e.month);
        let in_week = self.cal.months[e.month as usize].in_week;
        self.epoch_day = before + i128::from(e.day - 1);
        self.epoch_week = week_before + if in_week { i128::from(e.day - 1) } else { 0 };
        if let Some(weekday) = e.weekday {
            let weekdays = self.cal.weekdays.len();
            if weekday as usize >= weekdays {
                return Err(CalendarError::EpochWeekday { weekday, weekdays });
            }
            if !in_week {
                return Err(CalendarError::EpochOutOfWeek);
            }
        }
        let start = e.start_hour;
        if !start.is_finite() || start < 0.0 || hours_ms(f64::from(start)) >= self.day_ms {
            return Err(CalendarError::StartHour);
        }
        Ok(())
    }

    fn validate_eras_seasons(&self) -> Result<(), CalendarError> {
        if let Some(i) = self.cal.eras.windows(2).position(|w| w[1].start_year <= w[0].start_year) {
            return Err(CalendarError::ErasUnsorted { era: i + 1 });
        }
        for (season, s) in self.cal.seasons.iter().enumerate() {
            let month = self.cal.months.get(s.month as usize);
            if !month.is_some_and(|m| s.day >= 1 && s.day <= m.days + m.leap_days) {
                return Err(CalendarError::SeasonDate { season });
            }
        }
        if let Some(i) = self.cal.seasons.windows(2).position(|w| (w[1].month, w[1].day) <= (w[0].month, w[0].day)) {
            return Err(CalendarError::SeasonsUnsorted { season: i + 1 });
        }
        Ok(())
    }

    /// The calendar record this view was built from.
    pub fn calendar(&self) -> &Calendar {
        &self.cal
    }

    /// Hours in a day (the sky's `Calendar` takes it from here).
    pub fn hours_per_day(&self) -> f32 {
        self.cal.hours_per_day
    }

    /// Game milliseconds in a day ([`sf_ids::clock::day_ms`] of [`Self::hours_per_day`]).
    pub fn day_ms(&self) -> u64 {
        self.day_ms
    }

    /// The mean year length in days over a full leap cycle (the sky's year length).
    #[expect(clippy::cast_precision_loss, reason = "day counts and divisors far below 2^53")]
    pub fn mean_year_days(&self) -> f64 {
        let frac = self.leap.map_or(0.0, |l| {
            let inv = |d: Option<i128>| d.map_or(0.0, |d| 1.0 / d as f64);
            1.0 / l.every as f64 - inv(l.skip_lcm) + inv(l.restore_lcm)
        });
        self.common_days as f64 + self.leap_extra as f64 * frac
    }

    /// Whether `year` is a leap year (always `false` without a leap rule).
    pub fn is_leap(&self, year: i64) -> bool {
        self.leap.is_some_and(|l| {
            let r = i128::from(year) - l.offset;
            let m = |d: i128| d > 0 && r.rem_euclid(d) == 0;
            m(l.every) && (l.skip == 0 || !m(l.skip) || m(l.restore))
        })
    }

    /// Days in `year`.
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "validated: 1..=u32::MAX")]
    pub fn days_in_year(&self, year: i64) -> u64 {
        (self.common_days + if self.is_leap(year) { self.leap_extra } else { 0 }) as u64
    }

    /// Days in `month` (0-based) of `year`; `None` if there is no such month.
    pub fn days_in_month(&self, year: i64, month: u32) -> Option<u32> {
        let m = self.cal.months.get(month as usize)?;
        Some(m.days + if self.is_leap(year) { m.leap_days } else { 0 })
    }

    /// Whole days since the epoch day (day 0), the count moons and day-passed counters use.
    pub fn days_since_epoch(&self, instant: GameInstant) -> u64 {
        instant.0 / self.day_ms
    }

    /// A new game's instant: the epoch day at `epoch.start_hour`.
    pub fn start_instant(&self) -> GameInstant {
        GameInstant(hours_ms(f64::from(self.cal.epoch.start_hour)))
    }

    pub fn month_name(&self, month: u32) -> Option<&str> {
        self.cal.months.get(month as usize).map(|m| m.name.as_str())
    }

    pub fn weekday_name(&self, weekday: u32) -> Option<&str> {
        self.cal.weekdays.get(weekday as usize).map(String::as_str)
    }

    pub fn era_name(&self, era: usize) -> Option<&str> {
        self.cal.eras.get(era).map(|e| e.name.as_str())
    }

    pub fn season_name(&self, season: usize) -> Option<&str> {
        self.cal.seasons.get(season).map(|s| s.name.as_str())
    }

    /// The date and time of day of `instant`.
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "bounded by the year/day lengths")]
    pub fn date(&self, instant: GameInstant) -> Date {
        let since = instant.0 / self.day_ms;
        let in_day = instant.0 % self.day_ms;
        let day_index = i128::from(since) + self.epoch_day;
        let year = self.year_of_day(day_index);
        let leap = self.is_leap(year);
        let day_of_year = (day_index - self.days_before_year(year)) as u32;
        let (mut month, mut day, mut week_before) = (0, 1, 0);
        let mut left = day_of_year;
        for (i, m) in self.cal.months.iter().enumerate() {
            let len = m.days + if leap { m.leap_days } else { 0 };
            if left < len {
                (month, day) = (i as u32, left + 1);
                break;
            }
            left -= len;
            if m.in_week {
                week_before += i128::from(len);
            }
        }
        let weekday = match self.cal.epoch.weekday {
            Some(w) if self.cal.months[month as usize].in_week => {
                let index = self.week_before_year(year) + week_before + i128::from(day - 1) - self.epoch_week;
                Some((i128::from(w) + index).rem_euclid(self.cal.weekdays.len() as i128) as u32)
            }
            _ => None,
        };
        let eras = &self.cal.eras;
        let era = eras.partition_point(|e| e.start_year <= year).checked_sub(1);
        let year_of_era = era.map_or(year, |e| year - eras[e].start_year + 1);
        let seasons = &self.cal.seasons;
        let season = (!seasons.is_empty()).then(|| {
            let n = seasons.partition_point(|s| (s.month, s.day) <= (month, day));
            n.checked_sub(1).unwrap_or(seasons.len() - 1)
        });
        Date {
            year,
            era,
            year_of_era,
            month,
            day,
            day_of_year,
            weekday,
            season,
            hour: (in_day / MS_PER_HOUR) as u32,
            minute: (in_day % MS_PER_HOUR / MS_PER_MINUTE) as u32,
            ms: (in_day % MS_PER_MINUTE) as u32,
        }
    }

    /// The instant of a date and time (the inverse of [`Self::date`]); `None` when the date
    /// doesn't exist (no such month or day that year, minute >= 60, ms >= 60 000, a time past
    /// the day's end), precedes the epoch, or is past the last instant.
    pub fn instant_at(&self, at: &DateTime) -> Option<GameInstant> {
        let len = self.days_in_month(at.year, at.month)?;
        if at.day == 0 || at.day > len || u64::from(at.minute) >= 60 || u64::from(at.ms) >= MS_PER_MINUTE {
            return None;
        }
        let in_day = u64::from(at.hour) * MS_PER_HOUR + u64::from(at.minute) * MS_PER_MINUTE + u64::from(at.ms);
        if in_day >= self.day_ms {
            return None;
        }
        let (before, _) = self.before_month(self.is_leap(at.year), at.month);
        let day_index = self.days_before_year(at.year) + before + i128::from(at.day - 1);
        let since = u64::try_from(day_index - self.epoch_day).ok()?;
        since.checked_mul(self.day_ms)?.checked_add(in_day).map(GameInstant)
    }

    /// Days and in-week days of the months before `month` in a (leap) year.
    fn before_month(&self, leap: bool, month: u32) -> (i128, i128) {
        let mut days = 0;
        let mut week = 0;
        for m in &self.cal.months[..month as usize] {
            let len = i128::from(m.days + if leap { m.leap_days } else { 0 });
            days += len;
            if m.in_week {
                week += len;
            }
        }
        (days, week)
    }

    /// Leap years in `[epoch.year, year)` (negative before the epoch year): closed form.
    fn leaps_before(&self, year: i64) -> i128 {
        let Some(l) = self.leap else { return 0 };
        // Multiples of the rule's divisors in [0, n), signed so that M(b) - M(a) counts [a, b).
        let count = |n: i128| {
            let ceil = |d: i128| -(-n).div_euclid(d);
            ceil(l.every) - l.skip_lcm.map_or(0, ceil) + l.restore_lcm.map_or(0, ceil)
        };
        count(i128::from(year) - l.offset) - count(i128::from(self.cal.epoch.year) - l.offset)
    }

    /// Days from the start of the epoch year to the start of `year`.
    fn days_before_year(&self, year: i64) -> i128 {
        (i128::from(year) - i128::from(self.cal.epoch.year)) * self.common_days + self.leaps_before(year) * self.leap_extra
    }

    /// In-week days from the start of the epoch year to the start of `year`.
    fn week_before_year(&self, year: i64) -> i128 {
        (i128::from(year) - i128::from(self.cal.epoch.year)) * self.common_week
            + self.leaps_before(year) * self.leap_week_extra
    }

    /// The year holding day `day_index` (days from the start of the epoch year, >= 0): an
    /// estimate by the mean year, corrected with the exact count (one step for sane calendars).
    #[expect(clippy::cast_possible_truncation, clippy::cast_precision_loss, reason = "an estimate, corrected below")]
    fn year_of_day(&self, day_index: i128) -> i64 {
        let mut year = self.cal.epoch.year.saturating_add((day_index as f64 / self.mean_year_days()).floor() as i64);
        while self.days_before_year(year) > day_index {
            year -= 1;
        }
        while self.days_before_year(year + 1) <= day_index {
            year += 1;
        }
        year
    }
}

impl Default for CalendarView {
    /// The view of the framework's default [`Calendar`] (24-hour days, 365-day years).
    fn default() -> Self {
        Self::new(&Calendar::default()).expect("the default calendar is valid")
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: i128, b: i128) -> i128 {
    a / gcd(a, b) * b
}
