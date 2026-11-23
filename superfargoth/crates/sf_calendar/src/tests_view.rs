//! clock.md "Calendar (C3)" Tests (exact), framework part: a weird calendar (13 months, 30-hour
//! days, leap rule with skip/restore, intercalary day) and a festival calendar walked day by day,
//! epoch weekdays, season wrap, non-integer days, a 12-month no-leap calendar's month edges,
//! `start_instant` and `format`.

#![expect(clippy::float_cmp, reason = "the compared values are exact in binary")]

use sf_ids::GameInstant;
use sf_ids::clock::{MS_PER_HOUR, MS_PER_MINUTE};

use crate::*;

fn month(name: &str, days: u32) -> Month {
    Month { name: name.into(), days, leap_days: 0, in_week: true }
}

fn festival(name: &str) -> Month {
    Month { name: name.into(), days: 1, leap_days: 0, in_week: false }
}

fn names(prefix: &str, n: u32) -> Vec<String> {
    (0..n).map(|i| format!("{prefix}{i}")).collect()
}

fn dt(year: i64, month: u32, day: u32, hour: u32, minute: u32) -> DateTime {
    DateTime { year, month, day, hour, minute, ms: 0 }
}

/// The first day of the first non-empty month at or after `month` in `year`, rolling years.
fn first_day_from(view: &CalendarView, mut year: i64, mut month: u32) -> (i64, u32) {
    let months = u32::try_from(view.calendar().months.len()).unwrap();
    loop {
        if month == months {
            (year, month) = (year + 1, 0);
        }
        if view.days_in_month(year, month).unwrap() > 0 {
            return (year, month);
        }
        month += 1;
    }
}

/// Walks `days` days from the epoch one at a time and checks each day against the previous one
/// (the closed-form date math against plain succession): round trip at a varying time of day,
/// `days_since_epoch`, day/month/year succession, `day_of_year`, and the weekday stepping by one
/// over in-week days (none out of the week). Returns the date after the last day.
fn walk(view: &CalendarView, days: u64) -> Date {
    let cal = view.calendar();
    let week = u32::try_from(cal.weekdays.len()).unwrap();
    let mut prev: Option<Date> = None;
    let mut last_weekday = cal.epoch.weekday.map(|w| (w + week - 1) % week);
    for d in 0..=days {
        let t = GameInstant(d * view.day_ms() + d.wrapping_mul(7_919_993) % view.day_ms());
        let date = view.date(t);
        assert_eq!(view.instant_at(&date.date_time()), Some(t), "{date:?}");
        assert_eq!(view.days_since_epoch(t), d);
        if let Some(p) = prev {
            let (year, month, day, doy) = if p.day < view.days_in_month(p.year, p.month).unwrap() {
                (p.year, p.month, p.day + 1, p.day_of_year + 1)
            } else {
                let (y, m) = first_day_from(view, p.year, p.month + 1);
                (y, m, 1, if y == p.year { p.day_of_year + 1 } else { 0 })
            };
            assert_eq!((date.year, date.month, date.day, date.day_of_year), (year, month, day, doy), "after {p:?}");
        }
        let in_week = cal.months[date.month as usize].in_week;
        match last_weekday {
            Some(w) if in_week => {
                assert_eq!(date.weekday, Some((w + 1) % week), "{date:?}");
                last_weekday = date.weekday;
            }
            _ => assert_eq!(date.weekday, None, "{date:?}"),
        }
        prev = Some(date);
    }
    prev.unwrap()
}

/// 13 months (one a 1-day intercalary month out of the week), 30-hour days, a leap rule with
/// skip and restore and an offset, leap days in three months, 9 weekdays, a negative epoch year
/// in mid-year.
fn weird() -> Calendar {
    let mut months: Vec<Month> = (0..12).map(|i| month(&format!("W{i}"), 28 + i % 3)).collect();
    months[0].leap_days = 1;
    months[9].leap_days = 2;
    months.insert(6, Month { name: "Mid".into(), days: 1, leap_days: 1, in_week: false });
    Calendar {
        hours_per_day: 30.0,
        months,
        leap: Some(Leap { every: 3, skip_every: 30, restore_every: 120, offset: 2 }),
        weekdays: names("D", 9),
        eras: vec![Era { name: "Old".into(), start_year: -100 }, Era { name: "New".into(), start_year: 100 }],
        seasons: vec![
            Season { name: "S0".into(), month: 2, day: 1 },
            Season { name: "S1".into(), month: 7, day: 15 },
            Season { name: "S2".into(), month: 11, day: 29 },
        ],
        epoch: Epoch { year: -5, month: 3, day: 10, weekday: Some(4), start_hour: 0.0 },
    }
}

fn leap_rule(year: i64) -> bool {
    let r = (year - 2).rem_euclid(120);
    r % 3 == 0 && (r % 30 != 0 || r == 0)
}

#[test]
fn weird_calendar_round_trips_over_400_years() {
    let view = CalendarView::new(&weird()).unwrap();
    assert_eq!(view.day_ms(), 30 * MS_PER_HOUR);
    for year in -1000..1000 {
        assert_eq!(view.is_leap(year), leap_rule(year), "{year}");
        let months = (0..13).map(|m| u64::from(view.days_in_month(year, m).unwrap())).sum::<u64>();
        assert_eq!(view.days_in_year(year), months);
    }
    // Common year: 4 x (28 + 29 + 30) + 1 = 349 days; leap: +4. 120-year cycle: 40 - 4 + 1 = 37.
    assert_eq!(view.days_in_year(-3), 349);
    assert_eq!(view.days_in_year(-4), 353);
    assert!((view.mean_year_days() - (349.0 + 4.0 * 37.0 / 120.0)).abs() < 1e-12);
    let epoch_day: u64 = (0..3).map(|m| u64::from(view.days_in_month(-5, m).unwrap())).sum::<u64>() + 9;
    let end = view.instant_at(&dt(395, 0, 1, 0, 0)).unwrap();
    let days = view.days_since_epoch(end);
    assert_eq!(days + epoch_day, (-5..395).map(|y| view.days_in_year(y)).sum::<u64>());
    let after = walk(&view, days);
    assert_eq!((after.year, after.month, after.day, after.day_of_year), (395, 0, 1, 0));
    assert_eq!((after.era, after.year_of_era), (Some(1), 296));
    // Far from the epoch the year estimate still lands (corrected by the exact count).
    for year in [9_998, 9_999, 10_000, 123_456] {
        for (m, d) in [(0, 1), (12, 30), (6, 1)] {
            let Some(t) = view.instant_at(&dt(year, m, d, 29, 59)) else { continue };
            let date = view.date(t);
            assert_eq!((date.year, date.month, date.day, date.hour, date.minute), (year, m, d, 29, 59));
        }
    }
    // Before the epoch, a missing day and a time past the 30-hour day don't exist.
    assert_eq!(view.instant_at(&dt(-5, 3, 9, 29, 59)), None);
    assert_eq!(view.instant_at(&dt(-5, 3, 10, 30, 0)), None);
    assert_eq!(view.instant_at(&dt(-3, 6, 2, 0, 0)), None);
    assert!(view.instant_at(&dt(-4, 6, 2, 0, 0)).is_some());
    assert_eq!(view.instant_at(&dt(-4, 13, 1, 0, 0)), None);
    assert_eq!(view.instant_at(&DateTime { ms: 60_000, ..dt(-4, 0, 1, 0, 0) }), None);
    assert_eq!(view.instant_at(&dt(-4, 0, 1, 0, 60)), None);
}

/// 12 x 30 days with five 1-day festivals out of the week, a 10-day week, two eras, seasons.
fn festivals() -> Calendar {
    let mut months = Vec::new();
    for i in 0..12 {
        months.push(month(&format!("M{}", i + 1), 30));
        if [0, 3, 6, 8, 11].contains(&i) {
            months.push(festival(&format!("F{}", months.len())));
        }
    }
    Calendar {
        hours_per_day: 24.0,
        months,
        leap: None,
        weekdays: names("Day", 10),
        eras: vec![Era { name: "First".into(), start_year: 1 }, Era { name: "Second".into(), start_year: 1000 }],
        seasons: vec![
            Season { name: "Spring".into(), month: 3, day: 1 },
            Season { name: "Summer".into(), month: 7, day: 1 },
            Season { name: "Autumn".into(), month: 11, day: 1 },
            Season { name: "Winter".into(), month: 14, day: 20 },
        ],
        epoch: Epoch { year: 998, month: 0, day: 1, weekday: Some(0), start_hour: 8.0 },
    }
}

#[test]
fn festival_calendar_weeks_eras_and_seasons() {
    let view = CalendarView::new(&festivals()).unwrap();
    assert_eq!(view.calendar().months.len(), 17);
    assert_eq!(view.days_in_year(998), 365);
    let end = view.instant_at(&dt(1003, 0, 1, 0, 0)).unwrap();
    walk(&view, view.days_since_epoch(end));
    let at = |y, m, d| view.date(view.instant_at(&dt(y, m, d, 12, 0)).unwrap());
    // 30 in-week days before the first festival: M1 30 is the 10th weekday, F1 has none, M2 1
    // takes the next weekday.
    assert_eq!(at(998, 0, 30).weekday, Some(9));
    assert_eq!(at(998, 1, 1).weekday, None);
    assert_eq!(at(998, 2, 1).weekday, Some(0));
    // A year has 360 in-week days: the week starts every year on the same weekday.
    assert_eq!(at(999, 0, 1).weekday, Some(0));
    assert_eq!(at(998, 16, 1).weekday, None);
    // Eras switch at start_year.
    let last = at(999, 16, 1);
    assert_eq!((last.era, last.year_of_era), (Some(0), 999));
    let first = at(1000, 0, 1);
    assert_eq!((first.era, first.year_of_era, first.day_of_year), (Some(1), 1, 0));
    // Seasons start on their date and the last wraps over the year end.
    assert_eq!(at(1000, 0, 1).season, Some(3));
    assert_eq!(at(1000, 2, 30).season, Some(3));
    assert_eq!(at(1000, 3, 1).season, Some(0));
    assert_eq!(at(1000, 7, 1).season, Some(1));
    assert_eq!(at(1000, 14, 19).season, Some(2));
    assert_eq!(at(1000, 14, 20).season, Some(3));
    assert_eq!(at(1000, 16, 1).season, Some(3));
    assert_eq!(view.start_instant(), GameInstant(8 * MS_PER_HOUR));
}

#[test]
fn epoch_weekday_some_and_none() {
    let mut cal = Calendar::default();
    cal.epoch.weekday = Some(5);
    let view = CalendarView::new(&cal).unwrap();
    let day = |d: u64| view.date(GameInstant(d * view.day_ms() + 3 * MS_PER_HOUR)).weekday;
    assert_eq!([day(0), day(1), day(2), day(6), day(7), day(8)], [5, 6, 0, 4, 5, 6].map(Some));
    walk(&view, 800);
    cal.epoch.weekday = None;
    let view = CalendarView::new(&cal).unwrap();
    walk(&view, 800);
    cal.weekdays.clear();
    let view = CalendarView::new(&cal).unwrap();
    assert_eq!(view.date(GameInstant(5 * view.day_ms())).weekday, None);
}

#[test]
fn leap_only_month_and_non_integer_day() {
    let cal = Calendar {
        hours_per_day: 26.5,
        months: vec![
            month("A", 10),
            Month { name: "Leapday".into(), days: 0, leap_days: 1, in_week: false },
            month("B", 10),
        ],
        leap: Some(Leap { every: 4, skip_every: 0, restore_every: 0, offset: 0 }),
        weekdays: names("D", 3),
        eras: Vec::new(),
        seasons: Vec::new(),
        epoch: Epoch { year: 1, month: 0, day: 1, weekday: Some(1), start_hour: 26.25 },
    };
    let view = CalendarView::new(&cal).unwrap();
    assert_eq!(view.day_ms(), 26 * MS_PER_HOUR + 30 * MS_PER_MINUTE);
    assert_eq!(view.mean_year_days(), 20.25);
    assert_eq!(view.days_in_month(3, 1), Some(0));
    assert_eq!(view.days_in_month(4, 1), Some(1));
    assert_eq!(view.instant_at(&dt(3, 1, 1, 0, 0)), None);
    let leap_day = view.date(view.instant_at(&dt(4, 1, 1, 0, 0)).unwrap());
    assert_eq!((leap_day.day_of_year, leap_day.weekday), (10, None));
    walk(&view, 20 * 40);
    // The last minute of the 26.5-hour day exists; the next one is the next day.
    let last = view.instant_at(&dt(2, 0, 5, 26, 29)).unwrap();
    assert_eq!(view.instant_at(&dt(2, 0, 5, 26, 30)), None);
    let next = view.date(GameInstant(last.0 + MS_PER_MINUTE));
    assert_eq!((next.day, next.hour, next.minute), (6, 0, 0));
    let start = view.date(view.start_instant());
    assert_eq!((start.year, start.month, start.day, start.hour, start.minute), (1, 0, 1, 26, 15));
}

/// A 12-month, no-leap, 7-weekday calendar with an unknown epoch weekday and one era, epoch in
/// mid-year at a start hour: the shape a Tamrielic record has (generic names here).
fn twelve() -> Calendar {
    const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    Calendar {
        hours_per_day: 24.0,
        months: DAYS.iter().enumerate().map(|(i, &d)| month(&format!("Month{i}"), d)).collect(),
        leap: None,
        weekdays: names("Wd", 7),
        eras: vec![Era { name: "E3".into(), start_year: 1 }],
        seasons: Vec::new(),
        epoch: Epoch { year: 427, month: 7, day: 16, weekday: None, start_hour: 9.0 },
    }
}

#[test]
fn twelve_month_calendar_month_edges_over_five_years() {
    let view = CalendarView::new(&twelve()).unwrap();
    let mut edges = 0;
    for year in 427..432 {
        for m in 0..12 {
            let len = view.days_in_month(year, m).unwrap();
            for (day, hour, minute) in [(1, 0, 0), (1, 23, 59), (len, 0, 0), (len, 23, 59)] {
                let at = dt(year, m, day, hour, minute);
                let before_epoch = year == 427 && (m < 7 || (m == 7 && day < 16));
                let Some(t) = view.instant_at(&at) else {
                    assert!(before_epoch, "{at:?}");
                    continue;
                };
                assert!(!before_epoch, "{at:?}");
                let d = view.date(t);
                assert_eq!(d.date_time(), at);
                assert_eq!((d.weekday, d.era, d.year_of_era, d.season), (None, Some(0), year, None));
                if (day, hour) == (len, 23) {
                    // One minute later is the next month's first day (the next year after the last).
                    let n = view.date(GameInstant(t.0 + MS_PER_MINUTE));
                    let next = if m == 11 { (year + 1, 0) } else { (year, m + 1) };
                    assert_eq!((n.year, n.month, n.day, n.hour, n.minute), (next.0, next.1, 1, 0, 0));
                }
                edges += 1;
            }
        }
    }
    assert_eq!(edges, 5 * 48 - 7 * 4 - 2);
    let start = view.date(view.start_instant());
    assert_eq!((start.year, start.month, start.day, start.hour, start.day_of_year), (427, 7, 16, 9, 227));
    let first_of_next = view.instant_at(&dt(427, 8, 1, 0, 0)).unwrap();
    assert_eq!(view.days_since_epoch(first_of_next), 16);
    assert_eq!(view.mean_year_days(), 365.0);
    let new_year = view.date(view.instant_at(&dt(428, 0, 1, 0, 0)).unwrap());
    assert_eq!(view.days_since_epoch(view.instant_at(&dt(428, 0, 1, 0, 0)).unwrap()), 365 - 227);
    assert_eq!(new_year.day_of_year, 0);
}

#[test]
fn format_placeholders() {
    let view = CalendarView::new(&festivals()).unwrap();
    let d = view.date(view.instant_at(&dt(1000, 3, 7, 9, 5)).unwrap());
    let text = view.format(&d, "{weekday}, {d} {month} ({dd}/{m}) {yoe} {era} [{year}] {season} {hh}:{mm}");
    assert_eq!(text, "Day6, 7 M3 (07/4) 1 Second [1000] Spring 09:05");
    let festival = view.date(view.instant_at(&dt(999, 1, 1, 23, 0)).unwrap());
    assert_eq!(view.format(&festival, "<{weekday}>{month}"), "<>F1");
    assert_eq!(view.format(&d, "{x} {d"), "{x} {d");
    assert_eq!(view.format(&d, "{{d}}"), "{7}");
    let plain = CalendarView::new(&twelve()).unwrap();
    let start = plain.date(plain.start_instant());
    assert_eq!(plain.format(&start, "{d} {month} {era} {yoe}, {season}{weekday}."), "16 Month7 E3 427, .");
}
