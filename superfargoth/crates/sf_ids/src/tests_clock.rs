//! The game clock (clock.md "Tests"): exact assertions.
#![allow(clippy::float_cmp, reason = "the views are exact by construction")]

use core::time::Duration;

use crate::clock::{GameClock, GameInstant, MS_PER_HOUR, MS_PER_MINUTE, day_ms, hours_ms};

const DAY: u64 = 24 * MS_PER_HOUR;

#[test]
fn day_lengths_and_hours_convert_exactly() {
    assert_eq!(day_ms(24.0), 86_400_000);
    assert_eq!(day_ms(30.0), 108_000_000);
    assert_eq!(day_ms(0.0), 1, "never a zero-length day");
    assert_eq!(hours_ms(1.5), 5_400_000);
    assert_eq!(hours_ms(-2.0), 0);
    assert_eq!(hours_ms(f64::NAN), 0);
}

#[test]
fn minutes_round_down() {
    assert_eq!(GameInstant(59_999).minutes(), 0);
    assert_eq!(GameInstant(60_000).minutes(), 1);
    assert_eq!(GameInstant(DAY + 61 * MS_PER_MINUTE - 1).minutes(), 1440 + 60);
    assert_eq!(GameInstant::from_minutes(1500).minutes(), 1500);
    assert!((GameInstant(DAY + 90 * MS_PER_MINUTE).hours() - 25.5).abs() < 1e-12);
    assert_eq!(GameInstant::from_minutes(u64::MAX).0, u64::MAX, "saturates");
}

#[test]
fn days_views_and_their_inverse() {
    let i = GameInstant(3 * DAY + 18 * MS_PER_HOUR); // day 3, 18:00
    assert_eq!(i.days(DAY), 3.75);
    assert_eq!(i.day_fraction(DAY), 0.75);
    assert_eq!(i.hour_of_day(DAY), 18.0);
    assert_eq!(GameInstant::from_days(3.75, DAY), i);
    assert_eq!(GameInstant::from_days(-1.0, DAY), GameInstant::ZERO);
    assert_eq!(GameInstant::from_days(f64::NAN, DAY), GameInstant::ZERO);
    // A 30-hour calendar: noon is hour 15, and the day count follows its own length.
    let long = day_ms(30.0);
    let j = GameInstant(2 * long + 15 * MS_PER_HOUR);
    assert_eq!(j.days(long), 2.5);
    assert_eq!(j.hour_of_day(long), 15.0);
    assert_eq!(GameInstant::from_days(2.5, long), j);
    // Rounds down to the millisecond: 1.5 days and 0.9 ms.
    assert_eq!(GameInstant::from_days(1.5 + 0.9 / 86_400_000.0, DAY).0, DAY + DAY / 2);
}

#[test]
fn every_whole_millisecond_round_trips_through_days() {
    for ms in [0, 1, 999, DAY - 1, DAY, 400 * DAY + 12_345_678, 1_000_000 * DAY + 1] {
        let i = GameInstant(ms);
        // days() then from_days() may only lose to rounding down, and never more than 1 ms.
        let back = GameInstant::from_days(i.days(DAY), DAY);
        assert!(back <= i && i.0 - back.0 <= 1, "{ms}: {back:?}");
    }
}

#[test]
fn fixed_ticks_advance_exactly_and_carry_the_remainder() {
    // 64 Hz ticks at x30: 468.75 ms each; 64 of them are exactly 30 game seconds.
    let tick = Duration::from_micros(15_625);
    let mut c = GameClock::new(GameInstant::ZERO, 30.0);
    c.advance(tick);
    assert_eq!(c.now, GameInstant(468));
    for _ in 1..64 {
        c.advance(tick);
    }
    assert_eq!(c.now, GameInstant(30_000));
    // Many small steps equal one big one.
    let mut big = GameClock::new(GameInstant::ZERO, 30.0);
    big.advance(Duration::from_secs(1));
    assert_eq!(big.now, c.now);
    // A scale that doesn't divide evenly loses nothing: x7/3 for 3000 ticks of 1 ms = 7000 ms.
    let mut odd = GameClock::new(GameInstant::ZERO, 7.0 / 3.0);
    for _ in 0..3000 {
        odd.advance(Duration::from_millis(1));
    }
    assert!((6999..=7000).contains(&odd.now.0), "{:?}", odd.now);
}

#[test]
fn paused_and_zero_scale_hold_but_skip_still_passes_time() {
    let mut c = GameClock::new(GameInstant(DAY), 30.0);
    c.paused = true;
    c.advance(Duration::from_secs(100));
    assert_eq!(c.now, GameInstant(DAY));
    c.skip(8 * MS_PER_HOUR);
    assert_eq!(c.now, GameInstant(DAY + 8 * MS_PER_HOUR));
    let mut z = GameClock::new(GameInstant::ZERO, 0.0);
    z.advance(Duration::from_secs(100));
    let mut n = GameClock::new(GameInstant::ZERO, -5.0);
    n.advance(Duration::from_secs(100));
    assert_eq!((z.now, n.now), (GameInstant::ZERO, GameInstant::ZERO));
}

#[test]
fn set_hour_keeps_the_day_and_at_builds_a_clock() {
    let mut c = GameClock::at(10.0, 6.0, DAY);
    assert_eq!(c.now, GameInstant(10 * DAY + 6 * MS_PER_HOUR));
    c.set_hour(23.5, DAY);
    assert_eq!(c.now.days(DAY), 10.0 + 23.5 / 24.0);
    c.set_hour(25.0, DAY);
    assert_eq!(c.now, GameInstant(10 * DAY + MS_PER_HOUR), "wraps within the day");
    assert_eq!(GameClock::default().now, GameInstant(12 * MS_PER_HOUR));
}

#[test]
fn serde_carries_the_instant_and_scale_only() {
    let mut c = GameClock::new(GameInstant(123_456_789), 30.0);
    c.paused = true;
    c.advance(Duration::from_nanos(1)); // paused: no carry either
    let bytes = postcard::to_allocvec(&c).unwrap();
    let back: GameClock = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back.now, c.now);
    assert_eq!(back.time_scale.to_bits(), 30.0_f32.to_bits());
    assert!(!back.paused, "the dev freeze is session state");
    let instant = postcard::to_allocvec(&GameInstant(300)).unwrap();
    assert_eq!(postcard::from_bytes::<u64>(&instant).unwrap(), 300, "transparent");
}
