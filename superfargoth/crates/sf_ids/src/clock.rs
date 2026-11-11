//! The world's one game clock (clock.md, ROADMAP 1.17): [`GameInstant`], an integer count of
//! game milliseconds since the world's epoch (day 0, 00:00), and [`GameClock`], the host's
//! value that advances it. Every other view of time derives from the instant and rounds down:
//! the sky's fractional day ([`GameInstant::days`]), `sf_rpg::GameTime` minutes, `sf_effects`
//! `GameMinutes`, the AI tick's `now`, an adapter's calendar (`MWScript`'s `GameHour`...).
//!
//! Plain serde data (a Bevy `Resource` behind the `bevy` feature): the host owns the one value, saves it (`sf/clock`) and `sf_net`
//! replicates it (net.md "Net-ready rules" 1 and 4). Integer, so two machines given the same
//! fixed ticks reach the same instant and a save round trip is exact.

use core::time::Duration;

use serde::{Deserialize, Serialize};

/// Game milliseconds in one game second, minute and hour (fixed; a calendar only picks how many
/// hours a day has).
pub const MS_PER_SECOND: u64 = 1_000;
pub const MS_PER_MINUTE: u64 = 60 * MS_PER_SECOND;
pub const MS_PER_HOUR: u64 = 60 * MS_PER_MINUTE;
/// [`MS_PER_HOUR`] as `f64` (exact).
const MS_PER_HOUR_F64: f64 = 3_600_000.0;

/// Game milliseconds in a day of `hours_per_day` hours (`sf_sky::Calendar`), at least 1.
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 1..")]
pub fn day_ms(hours_per_day: f32) -> u64 {
    (f64::from(hours_per_day) * MS_PER_HOUR_F64).round().max(1.0) as u64
}

/// Game hours as whole game milliseconds, rounded down; negative and NaN give 0.
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 0..")]
pub fn hours_ms(hours: f64) -> u64 {
    let ms = (hours * MS_PER_HOUR_F64).floor();
    if ms.is_nan() || ms <= 0.0 { 0 } else { ms as u64 }
}

/// A point in game time: game milliseconds since the world's epoch (day 0, 00:00). Monotonic
/// in play (only [`GameClock::set_hour`], a dev tool, goes back). A `u64` spans ~584 million
/// years.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameInstant(pub u64);

impl GameInstant {
    pub const ZERO: Self = Self(0);

    /// `minutes` whole game minutes after the epoch (saturating).
    pub const fn from_minutes(minutes: u64) -> Self {
        Self(minutes.saturating_mul(MS_PER_MINUTE))
    }

    /// Whole game minutes since the epoch, rounded down (`GameTime`, `GameMinutes`, the AI's
    /// `now`: a reading never runs ahead of the clock).
    pub const fn minutes(self) -> u64 {
        self.0 / MS_PER_MINUTE
    }

    /// Game hours since the epoch, fractional (weather steps, `MWScript`'s hour-based timers,
    /// Lua's `core.game_time()`): independent of the calendar's day length.
    #[expect(clippy::cast_precision_loss, reason = "exact below 2^53 ms, ~285,000 years")]
    pub fn hours(self) -> f64 {
        self.0 as f64 / MS_PER_HOUR_F64
    }

    /// Days since the epoch for a day of `day_ms` ([`day_ms`]): the sky's view, `fract()` the time
    /// of day. The whole days and the remainder convert separately, so it stays exact to the
    /// millisecond for any realistic game.
    #[expect(clippy::cast_precision_loss, reason = "whole days < 2^53")]
    pub fn days(self, day_ms: u64) -> f64 {
        let day_ms = day_ms.max(1);
        (self.0 / day_ms) as f64 + (self.0 % day_ms) as f64 / day_ms as f64
    }

    /// The instant `days` days after the epoch, rounded down to the millisecond (the inverse of
    /// [`GameInstant::days`]; old `sf/clock` v1 saves stored days). Negative and NaN give the
    /// epoch.
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 0..")]
    #[expect(clippy::cast_precision_loss, reason = "day_ms < 2^53")]
    pub fn from_days(days: f64, day_ms: u64) -> Self {
        let days = if days.is_nan() { 0.0 } else { days.max(0.0) };
        let whole = days.floor();
        let part = ((days - whole) * day_ms.max(1) as f64).floor() as u64;
        Self((whole as u64).saturating_mul(day_ms.max(1)).saturating_add(part))
    }

    /// Time of day in `[0, 1)`: 0 = midnight, 0.5 = noon.
    #[expect(clippy::cast_precision_loss, reason = "under one day of ms")]
    pub fn day_fraction(self, day_ms: u64) -> f64 {
        let day_ms = day_ms.max(1);
        (self.0 % day_ms) as f64 / day_ms as f64
    }

    /// Hour of the day in `[0, hours per day)`.
    #[expect(clippy::cast_precision_loss, reason = "under one day of ms")]
    pub fn hour_of_day(self, day_ms: u64) -> f64 {
        (self.0 % day_ms.max(1)) as f64 / MS_PER_HOUR_F64
    }

    /// `ms` later (saturating).
    #[must_use]
    pub const fn plus(self, ms: u64) -> Self {
        Self(self.0.saturating_add(ms))
    }
}

/// The host's game clock: the instant, the time scale, and a dev freeze. One plain value,
/// owned by the host and advanced on the fixed tick ([`GameClock::advance`] with
/// `Time<Fixed>`'s delta); menus stop it by pausing Bevy's virtual time, which stops the fixed
/// tick. Serde carries `now` and `time_scale` (what is saved and replicated); `paused` and the
/// sub-millisecond carry are the host's session state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::resource::Resource))]
pub struct GameClock {
    pub now: GameInstant,
    /// Game seconds per real second: 1 is real time; Morrowind's `Timescale` global is 30.
    pub time_scale: f32,
    /// Dev freeze (screenshot and bench runs, the live A/B): [`GameClock::advance`] does nothing.
    /// Rest and wait still skip.
    #[serde(skip)]
    pub paused: bool,
    /// Game nanoseconds below the last whole millisecond, so a scale that doesn't divide a tick
    /// evenly loses nothing (64 ticks of 1/64 s at x30 are exactly 30 game seconds).
    #[serde(skip)]
    carry_ns: u32,
}

impl Default for GameClock {
    /// Noon of day 0 at x30.
    fn default() -> Self {
        Self::new(GameInstant(12 * MS_PER_HOUR), 30.0)
    }
}

impl GameClock {
    pub const fn new(now: GameInstant, time_scale: f32) -> Self {
        Self { now, time_scale, paused: false, carry_ns: 0 }
    }

    /// A clock at `hour` of day `day` (both may be fractional) for a day of `day_ms`, rounded
    /// down to the millisecond.
    #[must_use]
    pub fn at(day: f64, hour: f64, day_ms: u64) -> Self {
        Self { now: GameInstant::from_days(day, day_ms).plus(hours_ms(hour)), ..Self::default() }
    }

    /// Advances by `real` time scaled by `time_scale` (one fixed tick's delta); nothing while
    /// paused or at a scale that isn't positive.
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "non-negative, carry < 1e6")]
    #[expect(clippy::cast_precision_loss, reason = "a tick's nanoseconds are far below 2^53")]
    pub fn advance(&mut self, real: Duration) {
        let scale = f64::from(self.time_scale);
        if self.paused || scale.is_nan() || scale <= 0.0 {
            return;
        }
        let ns = (real.as_nanos() as f64 * scale).floor() + f64::from(self.carry_ns);
        let ms = (ns / 1e6).floor();
        self.carry_ns = (ns - ms * 1e6) as u32;
        self.now = self.now.plus(ms as u64);
    }

    /// Passes `ms` of game time at once (rest, wait, training, travel; a host command in co-op).
    /// Works while paused.
    pub fn skip(&mut self, ms: u64) {
        self.now = self.now.plus(ms);
    }

    /// Jumps to `hour` of the current day, keeping the day (dev tools and test scenes; may go
    /// back in time, which stalls timers set later than the new instant).
    pub fn set_hour(&mut self, hour: f64, day_ms: u64) {
        let day_ms = day_ms.max(1);
        let start = self.now.0 - self.now.0 % day_ms;
        self.now = GameInstant(start + hours_ms(hour) % day_ms);
        self.carry_ns = 0;
    }
}
