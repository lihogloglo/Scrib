//! clock.md "Calendar (C3)" tests: the record kind, RON with omitted defaults, `main` replaced by
//! a later source, the framework default, and each validation error.

#![expect(clippy::float_cmp, reason = "the compared values are exact in binary")]

use sf_ids::GameInstant;
use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::*;

const BASE: &str = r#"[
  define(kind: "sf.calendar", id: "main", value: (
    months: [
      (name: "Thaw", days: 30),
      (name: "Feast", days: 1, in_week: false),
      (name: "Frost", days: 30, leap_days: 1),
    ],
    leap: (every: 4),
    weekdays: ["One", "Two", "Three"],
    eras: [(name: "Age", start_year: 1)],
    epoch: (year: 5, month: 0, day: 1, weekday: 2),
  )),
]"#;

const MOD: &str = r#"[
  define(kind: "sf.calendar", id: "main", value: (
    hours_per_day: 20.0,
    months: [(name: "Only", days: 100)],
    epoch: (year: 1, month: 0, day: 1, start_hour: 6.5),
  )),
]"#;

fn db_with(files: &[&str]) -> RecordDb {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    register_kinds(&mut db).unwrap();
    for (i, text) in files.iter().enumerate() {
        let src = db.add_source(format!("src{i}"), &[]);
        let outcome = apply_record_file(&mut db, src, text, RecordFormat::Ron).unwrap();
        assert_eq!(outcome.errors, []);
        assert_eq!(outcome.defined, 1);
    }
    db
}

#[test]
fn ron_record_fills_defaults() {
    let db = db_with(&[BASE]);
    assert!(db.kind_names().contains(&CALENDAR_KIND));
    let cal = main_calendar(&db);
    assert_eq!(cal.hours_per_day, 24.0);
    assert_eq!(cal.months[0], Month { name: "Thaw".into(), days: 30, leap_days: 0, in_week: true });
    assert_eq!(cal.months[1], Month { name: "Feast".into(), days: 1, leap_days: 0, in_week: false });
    assert_eq!(cal.months[2].leap_days, 1);
    assert_eq!(cal.leap, Some(Leap { every: 4, skip_every: 0, restore_every: 0, offset: 0 }));
    assert_eq!(cal.seasons, []);
    assert_eq!(cal.epoch, Epoch { year: 5, month: 0, day: 1, weekday: Some(2), start_hour: 0.0 });
    let view = CalendarView::new(&cal).unwrap();
    assert_eq!(view.days_in_year(5), 61);
    assert_eq!(view.days_in_year(8), 62);
    assert_eq!(view.mean_year_days(), 61.25);
    assert_eq!(view.start_instant(), GameInstant::ZERO);
}

#[test]
fn later_source_replaces_main() {
    let cal = main_calendar(&db_with(&[BASE, MOD]));
    assert_eq!(cal.hours_per_day, 20.0);
    assert_eq!(cal.months.len(), 1);
    assert_eq!(cal.leap, None);
    assert_eq!(cal.weekdays, Vec::<String>::new());
    let view = CalendarView::new(&cal).unwrap();
    assert_eq!(view.day_ms(), 20 * sf_ids::clock::MS_PER_HOUR);
    assert_eq!(view.start_instant(), GameInstant(6 * sf_ids::clock::MS_PER_HOUR + 30 * sf_ids::clock::MS_PER_MINUTE));
}

#[test]
fn default_when_no_main() {
    assert_eq!(main_calendar(&RecordDb::new()), Calendar::default());
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let cal = main_calendar(&db);
    assert_eq!(cal, Calendar::default());
    let view = CalendarView::new(&cal).unwrap();
    assert_eq!(cal.months.len(), 12);
    assert_eq!(cal.weekdays.len(), 7);
    assert_eq!(view.days_in_year(1), 365);
    assert_eq!(view.days_in_year(4), 365);
    assert_eq!(view.mean_year_days(), 365.0);
    assert_eq!(view.hours_per_day(), 24.0);
    let d = view.date(GameInstant::ZERO);
    assert_eq!((d.year, d.month, d.day, d.weekday, d.era, d.year_of_era), (1, 0, 1, Some(0), None, 1));
}

fn base() -> Calendar {
    let mut cal = main_calendar(&db_with(&[BASE]));
    cal.seasons = vec![
        Season { name: "Warm".into(), month: 0, day: 10 },
        Season { name: "Cold".into(), month: 2, day: 31 },
    ];
    cal
}

fn rejects(edit: impl FnOnce(&mut Calendar), err: CalendarError) {
    let mut cal = base();
    edit(&mut cal);
    assert_eq!(CalendarView::new(&cal).unwrap_err(), err);
}

#[test]
fn validation_rejects_each_malformed_record() {
    CalendarView::new(&base()).unwrap();
    rejects(|c| c.hours_per_day = 0.0, CalendarError::HoursPerDay);
    rejects(|c| c.hours_per_day = -3.0, CalendarError::HoursPerDay);
    rejects(|c| c.hours_per_day = f32::NAN, CalendarError::HoursPerDay);
    rejects(|c| c.hours_per_day = f32::INFINITY, CalendarError::HoursPerDay);
    rejects(|c| c.months.clear(), CalendarError::NoMonths);
    rejects(|c| c.months[1].days = 0, CalendarError::EmptyMonth { month: 1 });
    rejects(
        |c| {
            c.months.truncate(1);
            c.months[0] = Month { name: "L".into(), days: 0, leap_days: 1, in_week: true };
            c.epoch.weekday = None;
        },
        CalendarError::EmptyYear,
    );
    rejects(|c| c.months[0].days = u32::MAX, CalendarError::YearTooLong);
    rejects(|c| c.leap = Some(Leap { every: 0, skip_every: 0, restore_every: 0, offset: 0 }), CalendarError::LeapEvery);
    rejects(|c| c.epoch.month = 3, CalendarError::EpochDate);
    rejects(|c| c.epoch.day = 0, CalendarError::EpochDate);
    rejects(|c| c.epoch.day = 31, CalendarError::EpochDate);
    // Day 31 of Frost exists only in leap years: year 5 isn't one, year 8 is.
    rejects(|c| (c.epoch.month, c.epoch.day) = (2, 31), CalendarError::EpochDate);
    let mut leap_epoch = base();
    (leap_epoch.epoch.year, leap_epoch.epoch.month, leap_epoch.epoch.day) = (8, 2, 31);
    CalendarView::new(&leap_epoch).unwrap();
    rejects(|c| c.epoch.weekday = Some(3), CalendarError::EpochWeekday { weekday: 3, weekdays: 3 });
    rejects(|c| c.weekdays.clear(), CalendarError::EpochWeekday { weekday: 2, weekdays: 0 });
    rejects(|c| c.epoch.month = 1, CalendarError::EpochOutOfWeek);
    rejects(|c| c.epoch.start_hour = 24.0, CalendarError::StartHour);
    rejects(|c| c.epoch.start_hour = -0.5, CalendarError::StartHour);
    rejects(|c| c.epoch.start_hour = f32::NAN, CalendarError::StartHour);
    rejects(|c| c.eras.push(Era { name: "Same".into(), start_year: 1 }), CalendarError::ErasUnsorted { era: 1 });
    rejects(|c| c.eras.insert(0, Era { name: "Late".into(), start_year: 9 }), CalendarError::ErasUnsorted { era: 1 });
    rejects(|c| c.seasons[0].month = 3, CalendarError::SeasonDate { season: 0 });
    rejects(|c| c.seasons[0].day = 0, CalendarError::SeasonDate { season: 0 });
    rejects(|c| c.seasons[1].day = 32, CalendarError::SeasonDate { season: 1 });
    rejects(|c| c.seasons.swap(0, 1), CalendarError::SeasonsUnsorted { season: 1 });
}
