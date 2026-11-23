//! Morrowind's `MWScript` calendar globals (`GameHour`, `Day`, `Month`, `Year`, `DaysPassed`) read
//! off the one game clock (clock.md "Calendar (C3)", "Morrowind"): the month table is content, the
//! builtin mod's `sf.calendar` record `main` (`mw_mods` `records/calendar.ron`), seen through the
//! framework's [`CalendarView`].
//!
//! Mirrors `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`,
//! `apps/openmw/mwworld/datetimemanager.cpp`: the globals `advanceTime` keeps (`setHour` wraps the
//! hour at 24, `setDay` rolls `Day` over `getDaysPerMonth` into `Month` and `Year`, `DaysPassed`
//! counts whole days). `OpenMW` advances them step by step ([`crate::advance_calendar`]); here they
//! are a function of the instant, with the same values: `GameHour` = the hour of day, `Day` /
//! `Month` / `Year` = the view's date (`Month` 0-based, `Day` from 1), `DaysPassed` = the start
//! global (1 in Morrowind.esm, on 16 Last Seed) + whole days since the epoch.

use sf_calendar::CalendarView;
use sf_ids::GameInstant;

use crate::training::Calendar;

/// The calendar globals at `instant`; `start_days_passed` is the content's `DaysPassed` start
/// global (the value on the epoch day). Values past `i32` saturate.
#[expect(clippy::cast_possible_truncation, reason = "the hour of day, under 24: f32 like the global")]
pub fn globals(view: &CalendarView, instant: GameInstant, start_days_passed: i32) -> Calendar {
    let date = view.date(instant);
    let days = i32::try_from(view.days_since_epoch(instant)).unwrap_or(i32::MAX);
    Calendar {
        hour: instant.hour_of_day(view.day_ms()) as f32,
        days_passed: start_days_passed.saturating_add(days),
        day: i32::try_from(date.day).unwrap_or(i32::MAX),
        month: i32::try_from(date.month).unwrap_or(i32::MAX),
        year: i32::try_from(date.year).unwrap_or(if date.year < 0 { i32::MIN } else { i32::MAX }),
    }
}

/// The clock after a script (or the console) wrote calendar globals (clock.md "Calendar (C3)",
/// C3c): `synced` is what the last sync wrote, `written` what the globals hold now; each field
/// that differs was written. Returns the new instant and the new `DaysPassed` start global (the
/// offset [`globals`] adds), so that the next [`globals`] reads the written values back.
///
/// Mirrors `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`,
/// `apps/openmw/mwworld/datetimemanager.cpp` 66-143 (`setHour`, `setDay`, `setMonth`) and
/// 196-222 (`updateGlobalFloat`, which routes a written global to them), read at that commit:
/// - `Year`: stored as is; the date moves to that year at the same month, day and time.
/// - `Month`: below 0 is 0; whole years past the last month carry into the year; the day clamps
///   to the new month's length (`setMonth`).
/// - `Day`: below 1 is 1; a day past the month's length rolls through the following months
///   into the next years (`setDay`).
/// - `GameHour`: below 0 (or NaN) is 0; the hour of day is set, so it may go back within the
///   day; whole days past `hours_per_day` roll the day forward (`setHour` -> `setDay`).
/// - `DaysPassed`: `OpenMW` only stores it; the clock doesn't move and the start offset shifts
///   so the written value holds. The date writes above don't change `DaysPassed` either (as in
///   `OpenMW`, they are not time passing): the start offset shifts by the days jumped.
///
/// Several globals written in one frame apply in the order `Year`, `Month`, `Day`, `GameHour`,
/// `DaysPassed` (`OpenMW` applies each as the script sets it). A date before the calendar's
/// epoch (no instant there; also a day rolled on from a month before the epoch month) clamps to
/// the epoch day at the same time of day; a date past the last representable instant leaves the
/// clock where it was. Never panics.
#[expect(clippy::float_cmp, reason = "exact: the global was written or not")]
pub fn apply_written(
    view: &CalendarView,
    instant: GameInstant,
    start_days_passed: i32,
    synced: &Calendar,
    written: &Calendar,
) -> (GameInstant, i32) {
    let day_ms = view.day_ms().max(1);
    let date = view.date(instant);
    let (mut year, mut month) = (date.year, date.month);
    let mut day = i128::from(date.day);
    let mut in_day = instant.0 % day_ms;
    if written.year != synced.year {
        year = i64::from(written.year);
    }
    if written.month != synced.month {
        let months = i64::try_from(view.calendar().months.len()).unwrap_or(i64::MAX).max(1);
        let m = i64::from(written.month.max(0));
        year = year.saturating_add(m / months);
        month = u32::try_from(m % months).unwrap_or(0);
    }
    let len = view.days_in_month(year, month).unwrap_or(1).max(1);
    day = if written.day == synced.day {
        // An unwritten day stays, clamped to the (new) month's length.
        day.min(i128::from(len))
    } else {
        i128::from(written.day.max(1))
    };
    if written.hour != synced.hour {
        let ms = sf_ids::clock::hours_ms(f64::from(written.hour));
        in_day = ms % day_ms;
        day += i128::from(ms / day_ms);
    }
    let new = month_end(view, year, month, len).map_or_else(
        || before_epoch(view, year, month).then_some(GameInstant(in_day)),
        |end| {
            let ms = i128::from(end) + (day - i128::from(len)) * i128::from(day_ms) + i128::from(in_day);
            Some(GameInstant(u64::try_from(ms.max(i128::from(in_day))).ok()?))
        },
    );
    let new = new.unwrap_or(instant);
    let days_before = i64::from(start_days_passed) + days_i64(view, instant);
    let days_passed = if written.days_passed == synced.days_passed {
        days_before
    } else {
        i64::from(written.days_passed)
    };
    let start = days_passed.saturating_sub(days_i64(view, new));
    let start = i32::try_from(start).unwrap_or(if start < 0 { i32::MIN } else { i32::MAX });
    (new, start)
}

/// The instant (ms) of 00:00 on the last day (`len`) of `month` of `year`: `None` when that day
/// is before the epoch or past `u64`.
fn month_end(view: &CalendarView, year: i64, month: u32, len: u32) -> Option<u64> {
    let at = sf_calendar::DateTime { year, month, day: len, hour: 0, minute: 0, ms: 0 };
    view.instant_at(&at).map(|i| i.0)
}

/// `month` of `year` is before the calendar's epoch month.
fn before_epoch(view: &CalendarView, year: i64, month: u32) -> bool {
    let epoch = &view.calendar().epoch;
    (year, month) < (epoch.year, epoch.month)
}

fn days_i64(view: &CalendarView, instant: GameInstant) -> i64 {
    i64::try_from(view.days_since_epoch(instant)).unwrap_or(i64::MAX)
}
