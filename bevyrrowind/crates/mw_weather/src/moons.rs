//! Masser and Secunda (NS-3): where each moon is, its phase and how visible it is, from the
//! `[Moons]` section of `Morrowind.ini` and the game clock.
//!
//! Port of `OpenMW` `MoonModel` (`apps/openmw/mwworld/weather.cpp` and `weather.hpp`,
//! github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`; the moon code is
//! the same on master as of 2026-11-17), f32 arithmetic in `OpenMW`'s order:
//! - [`MoonModel::new`]: `calculateSettings` (degrees per hour from the day length
//!   `Sunrise Time`..`Sunset Time`, at least 180/23 for a non-negative speed; the daily
//!   increment mod 24) and `buildCycle` (each cycle day's rise hour and handover hour);
//! - [`MoonModel::state`]: `calculateState` with `riseHour` (the 24-day skip), `handoverHour`,
//!   `angle`, `phase` (full on 16 Last Seed, 3 days per phase), `shadowBlend`, `hourlyAlpha` (with
//!   the one-minute early cull) and `earlyMoonShadowAlpha`;
//! - [`MoonState::direction`]: `Moon::setState` (`apps/openmw/mwrender/skyutil.cpp`): the moon
//!   turns `rotation_from_horizon` about the east axis from the northern horizon, then
//!   `rotation_from_north` (`Axis Offset`) about the vertical, so it rises north of west-north-west
//!   and sets south of east-south-east, passing overhead;
//! - [`quad_angular_size`]: the moon quad of `CelestialBody` (450 units times `Size / 115`,
//!   `apps/openmw/mwrender/sky.cpp`) seen from 1000 units;
//! - [`Phase::texture_suffix`]: `Moon::setPhase`'s texture names.
//!
//! Game days are `OpenMW`'s `TimeStamp` days (`DaysPassed`): day 1 is 16 Last Seed, the game's
//! start ([`game_day_hour`] maps the game clock to them).

use mw_install::MorrowindIni;

use crate::ini::WeatherIni;

/// `sOneMinute`: one game minute in hours, as `OpenMW` rounds it (Morrowind parity).
const ONE_MINUTE_H: f32 = 0.0167; // tunables: const (OpenMW data: Morrowind parity)
/// `sRiseHourEpsilon`.
const RISE_HOUR_EPSILON: f32 = 1e-4;
/// `sDaysPerPhase`.
const DAYS_PER_PHASE: i32 = 3; // tunables: const (Morrowind calendar data)
/// `sPhasesPerCycle`.
const PHASES_PER_CYCLE: i32 = 8; // tunables: const (Morrowind calendar data)
/// `sDaysPerCycle`.
const DAYS_PER_CYCLE: i32 = DAYS_PER_PHASE * PHASES_PER_CYCLE;
/// `sStartIncrements`: from the start date, 16 Last Seed 427.
const START_INCREMENTS: i32 = 17; // tunables: const (OpenMW data: the start date)

/// How a moon shows near the horizon (owner 2026-11-17, `DECISIONS.md` NS-3): the rules seam
/// between the modern default and the classic look. Paths, phases and the daytime fade
/// (`hourlyAlpha`) are `OpenMW`'s either way.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MoonHorizon {
    /// Modern default: the moon shows from the moment it rises, its surface textured all the way;
    /// the framework's atmosphere dims and reddens it low down, as its glow already was.
    #[default]
    Rise,
    /// `OpenMW` / vanilla: hidden below `Fade End Angle`, a plain star-hiding disk up to
    /// `Fade Start Angle` (`earlyMoonShadowAlpha`, `shadowBlend`). For the Classic preset.
    OpenMwFade,
}

/// The two moons, in the order `OpenMW` creates (and draws) them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoonName {
    Masser,
    Secunda,
}

impl MoonName {
    pub const ALL: [Self; 2] = [Self::Masser, Self::Secunda];

    /// The `[Moons]` key prefix (`Masser Size`) and `Moons_<name>_*` fallback name.
    pub fn ini_name(self) -> &'static str {
        match self {
            Self::Masser => "Masser",
            Self::Secunda => "Secunda",
        }
    }

    /// Lower-case name, as in the texture paths (`textures/tx_secunda_full.dds`).
    pub fn texture_name(self) -> &'static str {
        match self {
            Self::Masser => "masser",
            Self::Secunda => "secunda",
        }
    }

    /// The phase textures, frame `k` = phase `k / 8` from new (`Moon::setPhase` names).
    pub fn phase_textures(self) -> Vec<String> {
        Phase::FROM_NEW.iter().map(|p| format!("textures/tx_{}_{}.dds", self.texture_name(), p.texture_suffix())).collect()
    }
}

/// One moon's `[Moons]` settings (`OpenMW`'s `Moons_<name>_*` fallbacks).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoonSettings {
    pub size: f32,
    pub fade_in_start: f32,
    pub fade_in_finish: f32,
    pub fade_out_start: f32,
    pub fade_out_finish: f32,
    pub axis_offset: f32,
    pub speed: f32,
    pub daily_increment: f32,
    pub fade_start_angle: f32,
    pub fade_end_angle: f32,
    pub moon_shadow_early_fade_angle: f32,
}

impl MoonSettings {
    /// From the player's ini, each key falling back to the vanilla `Morrowind.ini` value.
    pub fn from_ini(moon: MoonName, ini: Option<&MorrowindIni>) -> Self {
        let mut w = WeatherIni::new(ini);
        Self::read(moon, &mut w)
    }

    fn read(moon: MoonName, w: &mut WeatherIni<'_>) -> Self {
        let name = moon.ini_name();
        let mut f = |key: &str| w.f32("Moons", &format!("{name} {key}")).unwrap_or(0.0);
        Self {
            size: f("Size"),
            fade_in_start: f("Fade In Start"),
            fade_in_finish: f("Fade In Finish"),
            fade_out_start: f("Fade Out Start"),
            fade_out_finish: f("Fade Out Finish"),
            axis_offset: f("Axis Offset"),
            speed: f("Speed"),
            daily_increment: f("Daily Increment"),
            fade_start_angle: f("Fade Start Angle"),
            fade_end_angle: f("Fade End Angle"),
            moon_shadow_early_fade_angle: f("Moon Shadow Early Fade Angle"),
        }
    }

    /// These settings under `horizon`. [`MoonHorizon::Rise`] sets both fade angles to the early
    /// fade angle (vanilla 0.5 deg), so `earlyMoonShadowAlpha` only ramps the first and last
    /// half degree of the arc and `shadowBlend` is textured above it. Applied before the cycle is
    /// built, so `handoverHour` (the switch to the next day's rise hour, a jump of the moon along
    /// its path) still falls only where the moon is invisible: below the horizon or faded out by
    /// day, never on a moon low in the sky. The player's `Fade Start/End Angle` keys only count
    /// under [`MoonHorizon::OpenMwFade`].
    #[must_use]
    pub fn with_horizon(mut self, horizon: MoonHorizon) -> Self {
        if horizon == MoonHorizon::Rise {
            let edge = self.moon_shadow_early_fade_angle.max(0.0);
            self.fade_start_angle = edge;
            self.fade_end_angle = edge;
        }
        self
    }
}

/// `Weather_Sunrise_Time` and `Weather_Sunset_Time` (`[Weather] Sunrise Time`, `Sunset Time`).
pub fn day_hours(ini: Option<&MorrowindIni>) -> (f32, f32) {
    let mut w = WeatherIni::new(ini);
    let sunrise = w.f32("Weather", "Sunrise Time").unwrap_or(0.0);
    let sunset = w.f32("Weather", "Sunset Time").unwrap_or(0.0);
    (sunrise, sunset)
}

/// `MWRender::MoonState::Phase`, in `OpenMW`'s order (0 is full).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Full,
    WaningGibbous,
    ThirdQuarter,
    WaningCrescent,
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
}

impl Phase {
    const ORDER: [Self; 8] = [
        Self::Full,
        Self::WaningGibbous,
        Self::ThirdQuarter,
        Self::WaningCrescent,
        Self::New,
        Self::WaxingCrescent,
        Self::FirstQuarter,
        Self::WaxingGibbous,
    ];

    /// The phase with `OpenMW` index `i` (mod 8).
    pub fn from_index(i: i32) -> Self {
        Self::ORDER[i.rem_euclid(PHASES_PER_CYCLE) as usize]
    }

    /// Fraction of the cycle from new (0 new, 0.25 first quarter, 0.5 full, 0.75 third quarter).
    pub fn cycle_fraction(self) -> f32 {
        let steps = match self {
            Self::New => 0.0,
            Self::WaxingCrescent => 1.0,
            Self::FirstQuarter => 2.0,
            Self::WaxingGibbous => 3.0,
            Self::Full => 4.0,
            Self::WaningGibbous => 5.0,
            Self::ThirdQuarter => 6.0,
            Self::WaningCrescent => 7.0,
        };
        steps / 8.0
    }

    /// `Moon::setPhase`'s texture suffix: `textures/tx_<moon>_<suffix>.dds`.
    pub fn texture_suffix(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::WaxingCrescent => "one_wax",
            Self::FirstQuarter => "half_wax",
            Self::WaxingGibbous => "three_wax",
            Self::WaningCrescent => "one_wan",
            Self::ThirdQuarter => "half_wan",
            Self::WaningGibbous => "three_wan",
            Self::Full => "full",
        }
    }

    /// The eight phases from new, one every eighth of the cycle ([`Self::cycle_fraction`]).
    pub const FROM_NEW: [Self; 8] = [
        Self::New,
        Self::WaxingCrescent,
        Self::FirstQuarter,
        Self::WaxingGibbous,
        Self::Full,
        Self::WaningGibbous,
        Self::ThirdQuarter,
        Self::WaningCrescent,
    ];
}

/// `MWRender::MoonState`: one moon at one instant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoonState {
    /// Degrees travelled from the rising horizon, `[0, 180)` (0 also while waiting to rise).
    pub rotation_from_horizon: f32,
    /// The path's turn about the vertical, degrees (`Axis Offset`).
    pub rotation_from_north: f32,
    pub phase: Phase,
    /// Textured surface (1) versus the plain sky-coloured disk (0) (`shadowBlend`).
    pub shadow_blend: f32,
    /// Opacity of the whole moon, `[0, 1]` (`earlyMoonShadowAlpha * hourlyAlpha`).
    pub alpha: f32,
}

impl MoonState {
    /// Unit vector toward the moon in (east, north, up) (`Moon::setState`: `rotX * rotZ * (0,1,0)`,
    /// `OpenMW`'s world frame is X east, Y north, Z up).
    pub fn direction(&self) -> glam::Vec3 {
        let (sin_x, cos_x) = self.rotation_from_horizon.to_radians().sin_cos();
        let (sin_z, cos_z) = self.rotation_from_north.to_radians().sin_cos();
        // osg `rotX * rotZ` applies rotX first: (0, cos x, sin x), then rotZ about up.
        glam::Vec3::new(-sin_z * cos_x, cos_z * cos_x, sin_x)
    }
}

/// Angular diameter (radians) of the moon's square texture: `CelestialBody`'s quad, 450 units per
/// side times `Size / 115` (`sky.cpp`), at 1000 units (`CelestialBody::mDistance`).
pub fn quad_angular_size(size: f32) -> f32 {
    2.0 * (0.5 * 450.0 * size / 115.0 / 1000.0).atan()
}

/// `OpenMW`'s game day (`DaysPassed`: 1 on 16 Last Seed) and hour from the game clock's days since
/// its epoch (day 0, midnight, is 16 Last Seed; `mw_saves_bevy` calendar).
pub fn game_day_hour(days: f64) -> (i32, f32) {
    let whole = days.floor();
    #[expect(clippy::cast_possible_truncation, reason = "game days and hours")]
    let (day, hour) = (whole as i32 + 1, ((days - whole) * 24.0) as f32);
    (day, hour.clamp(0.0, 23.999_9))
}

/// `CycleDay`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CycleDay {
    pub rise_hour: f32,
    pub handover_hour: f32,
}

/// `MoonModel`.
#[derive(Clone, Debug, PartialEq)]
pub struct MoonModel {
    s: MoonSettings,
    sunrise_hour: f32,
    sunset_hour: f32,
    degrees_per_hour: f32,
    cycle: [CycleDay; 24],
}

/// `safeDivide`: sign-safe and finite-safe.
fn safe_divide(numerator: f32, divisor: f32) -> f32 {
    if divisor != 0.0 {
        return numerator / divisor;
    }
    let magnitude = if numerator == 0.0 { 0.0 } else { f32::MAX };
    if numerator.is_sign_negative() == divisor.is_sign_negative() { magnitude } else { -magnitude }
}

impl MoonModel {
    /// `MoonModel(...)`: `calculateSettings` then `buildCycle`.
    pub fn new(settings: MoonSettings, sunrise_hour: f32, sunset_hour: f32) -> Self {
        let mut m = Self { s: settings, sunrise_hour, sunset_hour, degrees_per_hour: 0.0, cycle: [CycleDay::default(); 24] };
        m.calculate_settings();
        m.build_cycle();
        m
    }

    /// The moon from the player's ini (vanilla fallbacks), as `OpenMW` shows it
    /// ([`MoonHorizon::OpenMwFade`]).
    pub fn from_ini(moon: MoonName, ini: Option<&MorrowindIni>) -> Self {
        Self::from_ini_with(moon, ini, MoonHorizon::OpenMwFade)
    }

    /// [`Self::from_ini`] under `horizon` ([`MoonSettings::with_horizon`]).
    pub fn from_ini_with(moon: MoonName, ini: Option<&MorrowindIni>, horizon: MoonHorizon) -> Self {
        let (sunrise, sunset) = day_hours(ini);
        Self::new(MoonSettings::from_ini(moon, ini).with_horizon(horizon), sunrise, sunset)
    }

    pub fn settings(&self) -> &MoonSettings {
        &self.s
    }

    /// Degrees along the path per game hour.
    pub fn degrees_per_hour(&self) -> f32 {
        self.degrees_per_hour
    }

    /// The cached cycle (index = `dayOfCycle`).
    pub fn cycle(&self) -> &[CycleDay; 24] {
        &self.cycle
    }

    fn calculate_settings(&mut self) {
        let day_length = self.sunset_hour - self.sunrise_hour;
        self.degrees_per_hour = safe_divide(self.s.speed * 180.0, day_length);
        if !self.degrees_per_hour.is_sign_negative() {
            self.degrees_per_hour = self.degrees_per_hour.max(180.0 / 23.0);
        }
        self.s.daily_increment %= 24.0;
    }

    fn build_cycle(&mut self) {
        let first_day = 1 - DAYS_PER_CYCLE;
        let mut current_rise_hour = self.rise_hour(first_day);
        for day in first_day + 1..=DAYS_PER_CYCLE {
            let next_rise_hour = self.rise_hour(day);
            let handover = self.handover_hour(current_rise_hour, next_rise_hour);
            if day >= 1 {
                self.cycle[Self::day_of_cycle(day)] = CycleDay { rise_hour: current_rise_hour, handover_hour: handover };
            }
            if handover < 24.0 {
                current_rise_hour = next_rise_hour;
            }
        }
    }

    /// `calculateState` at `OpenMW` game day `game_day` (1 = 16 Last Seed) and `hour` in `[0, 24)`.
    pub fn state(&self, game_day: i32, hour: f32) -> MoonState {
        let today = self.cycle[Self::day_of_cycle(game_day)];
        let post_handover = hour >= today.handover_hour;
        let current_rise_hour =
            if post_handover { self.cycle[Self::day_of_cycle(game_day + 1)].rise_hour } else { today.rise_hour };
        let rotation_from_horizon = self.angle(hour, current_rise_hour);
        MoonState {
            rotation_from_horizon,
            rotation_from_north: self.s.axis_offset,
            phase: Self::phase(game_day, post_handover),
            shadow_blend: self.shadow_blend(rotation_from_horizon),
            alpha: self.early_moon_shadow_alpha(rotation_from_horizon) * self.hourly_alpha(hour),
        }
    }

    fn hours_until_visible(&self) -> f32 {
        safe_divide(self.s.fade_end_angle - self.s.moon_shadow_early_fade_angle, self.rotation(1.0))
    }

    fn hours_until_invisible(&self) -> f32 {
        safe_divide(180.0 - self.s.fade_end_angle + self.s.moon_shadow_early_fade_angle, self.rotation(1.0))
    }

    fn hour_of_day(hours: f32) -> f32 {
        let hour = hours % 24.0;
        if hour < 0.0 { hour + 24.0 } else { hour }
    }

    /// `dayOfCycle`: the day's place in the 24-day cycle (day 1 is 0).
    pub fn day_of_cycle(game_day: i32) -> usize {
        let day = (game_day - 1) % DAYS_PER_CYCLE;
        #[expect(clippy::cast_sign_loss, reason = "wrapped into [0, 24)")]
        let day = (if day < 0 { day + DAYS_PER_CYCLE } else { day }) as usize;
        day
    }

    /// `riseHour`: the hour the moon rises on `game_day`, before handovers.
    pub fn rise_hour(&self, game_day: i32) -> f32 {
        let inc = self.s.daily_increment;
        if inc == 0.0 {
            return 0.0;
        }
        #[expect(clippy::cast_precision_loss, reason = "small day counts, as OpenMW's float casts")]
        let (cycle, days) = (DAYS_PER_CYCLE as f32, (game_day + START_INCREMENTS - 1) as f32);
        let skipped_increments = (cycle - (cycle / inc).abs()) * (days / cycle).floor();
        let mut rise_hour = inc * (days - skipped_increments);
        rise_hour %= 24.0;
        if inc < 0.0 && rise_hour > RISE_HOUR_EPSILON {
            rise_hour -= 24.0;
        }
        if inc >= 0.0 && rise_hour <= RISE_HOUR_EPSILON {
            rise_hour += 24.0;
        }
        rise_hour
    }

    /// `handoverHour`: when the rise hour switches to the next day's, at a time the moon is
    /// invisible at both positions; 999 for none.
    pub fn handover_hour(&self, prev_rise_hour: f32, next_rise_hour: f32) -> f32 {
        let is_invisible_hour = |game_hour: f32, rise_hour: f32| {
            let travelled = Self::hour_of_day(game_hour - rise_hour);
            travelled < self.hours_until_visible() || travelled >= self.hours_until_invisible() - RISE_HOUR_EPSILON
        };
        let is_invisible = |game_hour: f32| {
            let transparent = self.hourly_alpha(game_hour) <= 0.0;
            (transparent || is_invisible_hour(game_hour, prev_rise_hour))
                && (is_invisible_hour(game_hour, next_rise_hour) || (transparent && game_hour < self.sunrise_hour))
        };
        if is_invisible(0.0) {
            return 0.0;
        }
        let daytime_fade = self.s.fade_out_finish - ONE_MINUTE_H;
        let prev_rise_invisible_hour = Self::hour_of_day(prev_rise_hour + self.hours_until_invisible());
        let next_rise_invisible_hour = Self::hour_of_day(next_rise_hour + self.hours_until_invisible());
        let mut handover = 999.0_f32;
        if daytime_fade >= 0.0 && is_invisible(daytime_fade) {
            handover = daytime_fade;
        }
        if is_invisible(prev_rise_invisible_hour) {
            handover = handover.min(prev_rise_invisible_hour);
        }
        if is_invisible(next_rise_invisible_hour) {
            handover = handover.min(next_rise_invisible_hour);
        }
        handover
    }

    fn rotation(&self, hours: f32) -> f32 {
        self.degrees_per_hour * hours
    }

    /// `angle`: degrees travelled from the rising horizon; 0 once past 180 (reset until it rises).
    pub fn angle(&self, game_hour: f32, rise_hour: f32) -> f32 {
        let mut travelled = game_hour - rise_hour;
        if travelled < 0.0 {
            travelled += 24.0;
        }
        let rotation_from_horizon = self.rotation(travelled);
        if rotation_from_horizon.abs() < 180.0 { rotation_from_horizon } else { 0.0 }
    }

    /// `phase`: full on day 1 (16 Last Seed), 3 days per phase; it changes with the rise hour.
    pub fn phase(game_day: i32, post_handover: bool) -> Phase {
        let phase_day = if post_handover { game_day + 1 } else { game_day };
        Phase::from_index((phase_day / DAYS_PER_PHASE) % PHASES_PER_CYCLE)
    }

    /// `shadowBlend`: the textured surface (1) versus the sky-coloured disk (0) by angle.
    pub fn shadow_blend(&self, angle: f32) -> f32 {
        let s = &self.s;
        let fade_angle = s.fade_start_angle - s.fade_end_angle;
        let fade_end_angle2 = 180.0 - s.fade_end_angle;
        let fade_start_angle2 = 180.0 - s.fade_start_angle;
        if angle >= s.fade_end_angle && angle < s.fade_start_angle {
            (angle - s.fade_end_angle) / fade_angle
        } else if angle >= s.fade_start_angle && angle < fade_start_angle2 {
            1.0
        } else if angle >= fade_start_angle2 && angle < fade_end_angle2 {
            (fade_end_angle2 - angle) / fade_angle
        } else {
            0.0
        }
    }

    /// `hourlyAlpha`: the daytime fade out and in, culled one minute early when the moon has a
    /// transparent hour range.
    pub fn hourly_alpha(&self, game_hour: f32) -> f32 {
        let s = &self.s;
        let adjusted_fade_out_finish =
            if s.fade_in_start > s.fade_out_finish { s.fade_out_finish - ONE_MINUTE_H } else { s.fade_out_finish };
        if game_hour >= s.fade_out_start && game_hour < adjusted_fade_out_finish {
            (adjusted_fade_out_finish - game_hour) / (adjusted_fade_out_finish - s.fade_out_start)
        } else if game_hour >= adjusted_fade_out_finish && game_hour < s.fade_in_start {
            0.0
        } else if game_hour >= s.fade_in_start && game_hour < s.fade_in_finish {
            (game_hour - s.fade_in_start) / (s.fade_in_finish - s.fade_in_start)
        } else {
            1.0
        }
    }

    /// `earlyMoonShadowAlpha`: hidden near the horizon, faded in over the early fade angle.
    pub fn early_moon_shadow_alpha(&self, angle: f32) -> f32 {
        let s = &self.s;
        if angle <= 0.0 && s.fade_end_angle >= s.moon_shadow_early_fade_angle {
            return 0.0;
        }
        let early1 = s.fade_end_angle - s.moon_shadow_early_fade_angle;
        let fade_end_angle2 = 180.0 - s.fade_end_angle;
        let early2 = fade_end_angle2 + s.moon_shadow_early_fade_angle;
        if angle >= early1 && angle < s.fade_end_angle {
            (angle - early1) / s.moon_shadow_early_fade_angle
        } else if angle >= s.fade_end_angle && angle < fade_end_angle2 {
            1.0
        } else if angle >= fade_end_angle2 && angle < early2 {
            (early2 - angle) / s.moon_shadow_early_fade_angle
        } else {
            0.0
        }
    }
}

#[cfg(test)]
#[path = "moons_tests.rs"]
mod tests;
