//! clock.md "Calendar (C3)" tests (exact) on the builtin mod's Tamrielic calendar: the instant <->
//! date round trip at every month edge, and [`globals`] equal to [`advance_calendar`]'s roll.

use sf_calendar::{CalendarView, DateTime};
use sf_ids::GameInstant;
use sf_ids::clock::hours_ms;
use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::calendar::{apply_written, globals};
use crate::{Calendar, advance_calendar};

/// The builtin mod's record file (`mw_mods` `BASE_RECORDS`; no dev-dependency on `mw_mods`).
const CALENDAR_RON: &str = include_str!("../../mw_mods/data/records/calendar.ron");

/// `DaysPassed` on the start day: Morrowind.esm's start global (`mw_weather::moons`: day 1 is
/// 16 Last Seed).
const START_DAYS_PASSED: i32 = 1;

fn tamrielic() -> CalendarView {
    let mut db = RecordDb::new();
    sf_calendar::register_kinds(&mut db).unwrap();
    let src = db.add_source("mw", &[]);
    let outcome = apply_record_file(&mut db, src, CALENDAR_RON, RecordFormat::Ron).unwrap();
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    CalendarView::new(&sf_calendar::main_calendar(&db)).unwrap()
}

const fn at(year: i64, month: u32, day: u32, hour: u32, minute: u32, ms: u32) -> DateTime {
    DateTime { year, month, day, hour, minute, ms }
}

#[test]
fn tamrielic_dates_round_trip_at_every_month_edge() {
    let view = tamrielic();
    assert_eq!(view.days_in_year(427), 365);
    // Before the epoch (16 Last Seed 427, 00:00) there is no instant.
    assert_eq!(view.instant_at(&at(427, 7, 15, 23, 59, 59_999)), None);
    assert_eq!(view.instant_at(&at(427, 7, 16, 0, 0, 0)), Some(GameInstant(0)));
    assert_eq!(view.start_instant(), GameInstant(hours_ms(9.0)));
    let mut edges = 0;
    for year in 428..433 {
        for month in 0..12 {
            let days = view.days_in_month(year, month).unwrap();
            let first = at(year, month, 1, 0, 0, 0);
            let last = at(year, month, days, 23, 59, 59_999);
            for dt in [first, last] {
                let instant = view.instant_at(&dt).unwrap();
                assert_eq!(view.date(instant).date_time(), dt);
            }
            // The next ms is the next month's first day (Evening Star 31 -> Morning Star 1 next year).
            let next = view.date(view.instant_at(&last).unwrap().plus(1));
            let (y, m) = if month == 11 { (year + 1, 0) } else { (year, month + 1) };
            assert_eq!(next.date_time(), at(y, m, 1, 0, 0, 0));
            edges += 1;
        }
    }
    assert_eq!(edges, 60);
    assert_eq!(view.days_in_month(430, 11), Some(31));
    assert_eq!(view.month_name(11), Some("Evening Star"));
}

/// `SplitMix64` (Steele, Lea, Flood 2014): seeded, no `rand` dependency.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[test]
#[expect(clippy::cast_precision_loss, reason = "quarter hours under 2^24: exact in f32")]
fn globals_agree_with_advance_calendar() {
    let view = tamrielic();
    let mut instant = view.start_instant();
    // The vanilla start globals: GameHour 9, Day 16, Month 7 (Last Seed), Year 427.
    let mut rolled = Calendar { hour: 9.0, days_passed: START_DAYS_PASSED, day: 16, month: 7, year: 427 };
    assert_eq!(globals(&view, instant, START_DAYS_PASSED), rolled);
    let mut rng = SplitMix64(0x5C21_B0C3);
    let mut years = 0;
    for step in 0..1000 {
        // Whole quarter hours (exact in f32 and in ms): mostly under 2 days, some weeks, some years.
        let r = rng.next();
        let quarters = match r % 8 {
            0 => rng.next() % (4 * 24 * 400),
            1 | 2 => rng.next() % (4 * 24 * 40),
            _ => rng.next() % (4 * 48),
        };
        let hours = quarters as f32 / 4.0;
        rolled = advance_calendar(rolled, hours);
        instant = instant.plus(hours_ms(f64::from(hours)));
        let got = globals(&view, instant, START_DAYS_PASSED);
        assert_eq!(
            (got.day, got.month, got.year, got.days_passed),
            (rolled.day, rolled.month, rolled.year, rolled.days_passed),
            "step {step}: +{hours} h"
        );
        assert!((got.hour - rolled.hour).abs() < 1e-3, "step {step}: {} vs {}", got.hour, rolled.hour);
        years = got.year - 427;
    }
    assert!(years > 5, "the advances cross several years ({years})");
}

/// A script writes the globals `edit` changes at `instant` (start global `start`): the globals
/// read back after [`apply_written`], and the new instant.
fn write(view: &CalendarView, instant: GameInstant, start: i32, edit: impl FnOnce(&mut Calendar)) -> (Calendar, GameInstant) {
    let synced = globals(view, instant, start);
    let mut written = synced;
    edit(&mut written);
    let (new, start) = apply_written(view, instant, start, &synced, &written);
    (globals(view, new, start), new)
}

const fn cal(hour: f32, days_passed: i32, day: i32, month: i32, year: i32) -> Calendar {
    Calendar { hour, days_passed, day, month, year }
}

/// `dt` on the Tamrielic calendar (panics when there is no such instant).
fn instant(view: &CalendarView, dt: DateTime) -> GameInstant {
    view.instant_at(&dt).unwrap()
}

#[test]
fn written_game_hour_sets_the_hour_of_the_current_day() {
    let view = tamrielic();
    let start = view.start_instant();
    let (got, now) = write(&view, start, START_DAYS_PASSED, |c| c.hour = 20.0);
    assert_eq!(got, cal(20.0, 1, 16, 7, 427), "forward within the day");
    assert_eq!(now, GameInstant(hours_ms(20.0)));
    let (got, now) = write(&view, now, START_DAYS_PASSED, |c| c.hour = 3.5);
    assert_eq!(got, cal(3.5, 1, 16, 7, 427), "back within the day");
    assert_eq!(now, GameInstant(hours_ms(3.5)));
    let (got, _) = write(&view, now, START_DAYS_PASSED, |c| c.hour = -2.0);
    assert_eq!(got, cal(0.0, 1, 16, 7, 427), "below 0 is 0");
    // 30 h: 6:00 the next day; DaysPassed doesn't count a written hour (OpenMW `setHour`).
    let (got, now) = write(&view, start, START_DAYS_PASSED, |c| c.hour = 30.0);
    assert_eq!(got, cal(6.0, 1, 17, 7, 427), "whole days roll forward");
    assert_eq!(now, GameInstant(hours_ms(30.0)));
}

#[test]
fn written_month_clamps_the_day_and_carries_years() {
    let view = tamrielic();
    let eve = instant(&view, at(428, 11, 31, 10, 0, 0));
    let dp = globals(&view, eve, START_DAYS_PASSED).days_passed;
    let (got, now) = write(&view, eve, START_DAYS_PASSED, |c| c.month = 1);
    assert_eq!(got, cal(10.0, dp, 28, 1, 428), "Evening Star 31 -> Sun's Dawn 28 (setMonth)");
    assert_eq!(now, instant(&view, at(428, 1, 28, 10, 0, 0)));
    let (got, _) = write(&view, eve, START_DAYS_PASSED, |c| c.month = 13);
    assert_eq!(got, cal(10.0, dp, 28, 1, 429), "month 13: a year on, Sun's Dawn");
    let (got, _) = write(&view, eve, START_DAYS_PASSED, |c| c.month = -4);
    assert_eq!(got, cal(10.0, dp, 31, 0, 428), "below 0 is Morning Star");
}

#[test]
fn written_day_rolls_through_the_months() {
    let view = tamrielic();
    let dawn = instant(&view, at(428, 1, 10, 8, 0, 0));
    let dp = globals(&view, dawn, START_DAYS_PASSED).days_passed;
    let (got, _) = write(&view, dawn, START_DAYS_PASSED, |c| c.day = 31);
    assert_eq!(got, cal(8.0, dp, 3, 2, 428), "day 31 in a 28-day month: 3 First Seed (setDay)");
    let (got, _) = write(&view, dawn, START_DAYS_PASSED, |c| c.day = 0);
    assert_eq!(got, cal(8.0, dp, 1, 1, 428), "below 1 is 1");
    let eve = instant(&view, at(428, 11, 2, 8, 0, 0));
    let eve_dp = globals(&view, eve, START_DAYS_PASSED).days_passed;
    let (got, _) = write(&view, eve, START_DAYS_PASSED, |c| c.day = 400);
    assert_eq!(got, cal(8.0, eve_dp, 4, 0, 430), "across two new years");
    // Year, Month and Day together: that date at the current time.
    let (got, _) = write(&view, dawn, START_DAYS_PASSED, |c| (c.year, c.month, c.day) = (430, 5, 20));
    assert_eq!(got, cal(8.0, dp, 20, 5, 430));
}

#[test]
fn written_days_passed_shifts_the_count_not_the_clock() {
    let view = tamrielic();
    let now = instant(&view, at(427, 8, 3, 12, 0, 0));
    let synced = globals(&view, now, START_DAYS_PASSED);
    let written = Calendar { days_passed: 50, ..synced };
    let (new, start) = apply_written(&view, now, START_DAYS_PASSED, &synced, &written);
    assert_eq!(new, now, "the clock doesn't move");
    assert_eq!(globals(&view, new, start), written);
    // The next day counts on from the written value.
    assert_eq!(globals(&view, new.plus(view.day_ms()), start).days_passed, 51);
    // Nothing written: nothing moves.
    assert_eq!(apply_written(&view, now, START_DAYS_PASSED, &synced, &synced), (now, START_DAYS_PASSED));
}

#[test]
fn a_date_before_the_epoch_clamps_to_the_epoch_day() {
    let view = tamrielic();
    let now = instant(&view, at(428, 3, 5, 14, 0, 0));
    let dp = globals(&view, now, START_DAYS_PASSED).days_passed;
    let (got, _) = write(&view, now, START_DAYS_PASSED, |c| c.year = 400);
    assert_eq!(got, cal(14.0, dp, 16, 7, 427), "year 400: 16 Last Seed 427");
    let (got, _) = write(&view, now, START_DAYS_PASSED, |c| (c.year, c.day) = (427, 2));
    assert_eq!(got, cal(14.0, dp, 16, 7, 427), "Rain's Hand 427 is before the epoch");
    let (got, _) = write(&view, now, START_DAYS_PASSED, |c| c.year = i32::MAX);
    assert_eq!(got.year, 428, "past the last instant: the clock stays");
}
