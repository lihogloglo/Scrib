//! The tint of the sky's ambient floor from `Morrowind.ini` (RO-16 fix 2):
//! each weather's `Ambient Sunrise/Day/Sunset/Night Color`, interpolated over
//! the time of day, as `SkyTerms::ambient_tint_by_hour`.
//!
//! Port of `OpenMW` `apps/openmw/mwworld/weather.cpp` (github.com/OpenMW/openmw,
//! commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`):
//! - `TimeOfDayInterpolator::getValue` ([`TimeOfDay::value`]): night, a fade
//!   night → sunrise → day around sunrise, day, a fade day → sunset → night
//!   around sunset, each fade peaking at its middle;
//! - the `TimeOfDaySettings` of `WeatherManager`'s constructor ([`DayTimes`]):
//!   night ends at `Sunrise Time`, day starts `Sunrise Duration` later, day
//!   ends at `Sunset Time`, night starts `Sunset Duration` later;
//! - `TimeOfDaySettings::addSetting("Ambient")` ([`Fades`]): the
//!   `Ambient Pre/Post-Sunrise/Sunset Time` keys widen each fade.
//!
//! The weather transition (`calculateTransitionResult`:
//! `lerp(current.mAmbientColor, other.mAmbientColor, factor)`) is `sf_weather`'s
//! sky-channel blend of the presets' tints (weights summing to 1; a smoothstep
//! over the first 60 % of the transition where `OpenMW`'s factor is linear).
//!
//! Not a 1:1 port: `OpenMW` lights in gamma space with the colour as is. Here
//! the time-of-day lerp runs on the ini's gamma-space values as `OpenMW`'s does,
//! the result is decoded to linear (sRGB transfer function), and only its
//! chromaticity is kept (Rec. 709 luminance 1): the floor's brightness stays
//! the calibrated `LightCalibration::night_ambient_nits` (owner: lit, readable
//! nights, never darker without an A/B). [`WEATHER_AMBIENT_BRIGHTNESS`] can let
//! the weathers' relative brightness through; it defaults to 0.

use glam::Vec3;
use sf_weather::sf_sky::{KeyCurve, luminance};

use crate::WeatherType;
use crate::ini::WeatherIni;
use crate::visual::hour_curve_at;

/// How much of the weathers' relative ambient brightness shows: the exponent
/// on (this weather's luminance / Clear's at the same hour). 0 keeps only the
/// colour (every weather as bright as the calibrated floor); 1 is Morrowind's
/// full ratio (a vanilla Overcast night is ~2.4x a Clear one).
pub const WEATHER_AMBIENT_BRIGHTNESS: f32 = 0.0;

/// Samples across each sunrise/sunset fade for the tint curves (plus the
/// quarter-hour grid); keys that their neighbours reproduce are dropped.
const FADE_STEPS: u16 = 96;

/// The day's corners, game hours (`OpenMW` `TimeOfDaySettings`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DayTimes {
    pub night_end: f32,
    pub day_start: f32,
    pub day_end: f32,
    pub night_start: f32,
}

impl DayTimes {
    /// From `[Weather]` `Sunrise Time`, `Sunrise Duration`, `Sunset Time`,
    /// `Sunset Duration`.
    pub fn new(sunrise_time: f32, sunrise_duration: f32, sunset_time: f32, sunset_duration: f32) -> Self {
        Self {
            night_end: sunrise_time,
            day_start: sunrise_time + sunrise_duration,
            day_end: sunset_time,
            night_start: sunset_time + sunset_duration,
        }
    }
}

/// How far one quantity's fades reach before and after the day's corners,
/// hours (`OpenMW` `WeatherSetting`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fades {
    pub pre_sunrise: f32,
    pub post_sunrise: f32,
    pub pre_sunset: f32,
    pub post_sunset: f32,
}

/// Four values over the day (`OpenMW` `TimeOfDayInterpolator`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeOfDay {
    pub sunrise: Vec3,
    pub day: Vec3,
    pub sunset: Vec3,
    pub night: Vec3,
}

/// `OpenMW`'s `lerp`, in its operation order.
fn lerp(x: Vec3, y: Vec3, factor: f32) -> Vec3 {
    x * (1.0 - factor) + y * factor
}

impl TimeOfDay {
    /// The value at `hour` (`TimeOfDayInterpolator::getValue`).
    pub fn value(&self, hour: f32, t: &DayTimes, f: &Fades) -> Vec3 {
        let sunrise_from = t.night_end - f.pre_sunrise;
        let sunrise_to = t.day_start + f.post_sunrise;
        let sunset_from = t.day_end - f.pre_sunset;
        let sunset_to = t.night_start + f.post_sunset;
        // Each fade: from its outer value at the start to its peak at the
        // middle, then on to the next outer value at its end.
        let fade = |from: f32, to: f32, before: Vec3, peak: Vec3, after: Vec3| {
            let duration = to - from;
            let middle = from + duration / 2.0;
            if hour <= middle {
                let factor = if duration > 0.0 { (middle - hour) / duration * 2.0 } else { 0.0 };
                lerp(peak, before, factor)
            } else {
                let factor = if duration > 0.0 { (hour - middle) / duration * 2.0 } else { 1.0 };
                lerp(peak, after, factor)
            }
        };
        if hour < sunrise_from || hour > sunset_to {
            self.night
        } else if hour <= sunrise_to {
            fade(sunrise_from, sunrise_to, self.night, self.sunrise, self.day)
        } else if hour < sunset_from {
            self.day
        } else if hour <= sunset_to {
            fade(sunset_from, sunset_to, self.day, self.sunset, self.night)
        } else {
            // `OpenMW` returns `T()` here (unreachable with ordered times).
            Vec3::ZERO
        }
    }
}

/// The sRGB transfer function's inverse (IEC 61966-2-1): a gamma-space value
/// in `[0, 1]` to linear.
pub fn srgb_to_linear(u: f32) -> f32 {
    if u <= 0.040_45 {
        u / 12.92
    } else {
        ((u + 0.055) / 1.055).powf(2.4)
    }
}

/// An ini colour's bytes as gamma-space `[0, 1]` (`OpenMW`'s `getColour`).
pub fn srgb8(c: [u8; 3]) -> Vec3 {
    Vec3::from(c.map(f32::from)) / 255.0
}

/// A gamma-space colour decoded to linear.
pub fn decode(c: Vec3) -> Vec3 {
    Vec3::from(c.to_array().map(srgb_to_linear))
}

/// `c` (linear) scaled to Rec. 709 luminance 1; grey for black or non-finite.
pub fn chroma(c: Vec3) -> Vec3 {
    let l = luminance(c);
    if l > 0.0 && l.is_finite() { c / l } else { Vec3::ONE }
}

/// The ambient tint at `hour` (linear, luminance 1 times the relative
/// brightness `(luminance / Clear's luminance) ^ brightness`).
pub fn tint_at(colors: &TimeOfDay, clear: &TimeOfDay, hour: f32, t: &DayTimes, f: &Fades, brightness: f32) -> Vec3 {
    let c = decode(colors.value(hour, t, f));
    let tint = chroma(c);
    if brightness <= 0.0 {
        return tint;
    }
    let ratio = luminance(c) / luminance(decode(clear.value(hour, t, f)));
    if ratio > 0.0 && ratio.is_finite() { tint * ratio.powf(brightness) } else { tint }
}

/// `[Weather]` day corners and the ambient fades (vanilla where absent).
pub(crate) fn day_times(w: &mut WeatherIni<'_>) -> (DayTimes, Fades) {
    let mut get = |key: &str, vanilla: f32| w.f32("Weather", key).unwrap_or(vanilla);
    let times = DayTimes::new(
        get("Sunrise Time", 6.0),
        get("Sunrise Duration", 2.0),
        get("Sunset Time", 18.0),
        get("Sunset Duration", 2.0),
    );
    let fades = Fades {
        pre_sunrise: get("Ambient Pre-Sunrise Time", 0.5),
        post_sunrise: get("Ambient Post-Sunrise Time", 2.0),
        pre_sunset: get("Ambient Pre-Sunset Time", 1.0),
        post_sunset: get("Ambient Post-Sunset Time", 1.25),
    };
    (times, fades)
}

/// `ty`'s four ambient colours (gamma space); `None` if one is missing.
pub(crate) fn colors(ty: WeatherType, w: &mut WeatherIni<'_>) -> Option<TimeOfDay> {
    let section = format!("Weather {}", ty.name());
    let mut get = |when: &str| w.color(&section, &format!("Ambient {when} Color")).map(srgb8);
    Some(TimeOfDay { sunrise: get("Sunrise")?, day: get("Day")?, sunset: get("Sunset")?, night: get("Night")? })
}

/// `ty`'s ambient tint over the hour of day as linear R, G, B curves (empty
/// when the ini has no colours for it: the renderer's default tint).
pub(crate) fn tint_curves(ty: WeatherType, w: &mut WeatherIni<'_>) -> [KeyCurve; 3] {
    tint_curves_with(ty, w, WEATHER_AMBIENT_BRIGHTNESS)
}

pub(crate) fn tint_curves_with(ty: WeatherType, w: &mut WeatherIni<'_>, brightness: f32) -> [KeyCurve; 3] {
    let (times, fades) = day_times(w);
    let (Some(own), Some(clear)) = (colors(ty, w), colors(WeatherType::Clear, w)) else {
        return Default::default();
    };
    // The fades' corners and peaks, so the curves hit them exactly, and a
    // denser grid inside them (the luminance normalisation bends the lerp).
    let mut extra = Vec::new();
    for (from, to) in [
        (times.night_end - fades.pre_sunrise, times.day_start + fades.post_sunrise),
        (times.day_end - fades.pre_sunset, times.night_start + fades.post_sunset),
    ] {
        extra.extend([from, from + (to - from) / 2.0, to]);
        extra.extend((1..FADE_STEPS).map(|i| from + (to - from) * f32::from(i) / f32::from(FADE_STEPS)));
    }
    [0, 1, 2].map(|i| hour_curve_at(|h| tint_at(&own, &clear, h, &times, &fades, brightness)[i], &extra))
}

#[cfg(test)]
mod tests {
    // Exact float compares are intended: the interpolation's corners are exact.
    #![allow(clippy::float_cmp, clippy::many_single_char_names)]

    use mw_install::MorrowindIni;

    use super::*;
    use crate::ini::parse_color;

    fn vanilla_times() -> (DayTimes, Fades) {
        day_times(&mut WeatherIni::new(None))
    }

    fn sample(curves: &[KeyCurve; 3], h: f32) -> Vec3 {
        Vec3::new(curves[0].sample(h), curves[1].sample(h), curves[2].sample(h))
    }

    #[test]
    fn ini_colours_parse_and_fall_back_to_vanilla() {
        assert_eq!(parse_color("047,066,096"), Some([47, 66, 96]));
        assert_eq!(parse_color(" 1, 2 ,3"), Some([1, 2, 3]));
        for bad in ["1,2", "1,2,3,4", "256,0,0", "a,b,c", ""] {
            assert_eq!(parse_color(bad), None, "{bad:?}");
        }
        let mut w = WeatherIni::new(None);
        assert_eq!(w.color("Weather Clear", "Ambient Night Color"), Some([32, 35, 42]));
        assert_eq!(w.color("weather ashstorm", "ambient night color"), Some([36, 42, 49]));
        assert_eq!(w.color("Weather Clear", "Ambient Noon Color"), None);
        let ini = MorrowindIni::parse(
            b"[Weather Clear]\nAmbient Night Color=010,020,030\n[Weather Rain]\nAmbient Night Color=oops\n",
        );
        let mut w = WeatherIni::new(Some(&ini));
        assert_eq!(w.color("Weather Clear", "Ambient Night Color"), Some([10, 20, 30]));
        assert_eq!(w.color("Weather Rain", "Ambient Night Color"), Some([50, 55, 67]), "unparsable: vanilla");
        let (times, fades) = vanilla_times();
        assert_eq!(times, DayTimes { night_end: 6.0, day_start: 8.0, day_end: 18.0, night_start: 20.0 });
        assert_eq!(fades, Fades { pre_sunrise: 0.5, post_sunrise: 2.0, pre_sunset: 1.0, post_sunset: 1.25 });
    }

    #[test]
    fn time_of_day_hits_the_four_colours_and_fades_linearly() {
        let (t, f) = vanilla_times();
        let c = TimeOfDay { sunrise: Vec3::X, day: Vec3::Y, sunset: Vec3::Z, night: Vec3::ONE };
        // Sunrise fade 5.5 .. 10 (peak 7.75); sunset fade 17 .. 21.25 (peak 19.125).
        for (h, want) in [
            (0.0, c.night),
            (5.4, c.night),
            (5.5, c.night),
            (7.75, c.sunrise),
            (10.0, c.day),
            (12.0, c.day),
            (17.0, c.day),
            (19.125, c.sunset),
            (21.25, c.night),
            (23.9, c.night),
        ] {
            assert_eq!(c.value(h, &t, &f), want, "{h} h");
        }
        assert_eq!(c.value(6.625, &t, &f), (c.sunrise + c.night) / 2.0);
        assert_eq!(c.value(8.875, &t, &f), (c.sunrise + c.day) / 2.0);
        assert_eq!(c.value(18.0625, &t, &f), (c.sunset + c.day) / 2.0);
        assert_eq!(c.value(20.1875, &t, &f), (c.sunset + c.night) / 2.0);
    }

    #[test]
    fn srgb_decodes_and_luminance_normalises() {
        assert_eq!(srgb_to_linear(0.0), 0.0);
        assert!((srgb_to_linear(1.0) - 1.0).abs() < 1e-6);
        assert_eq!(srgb_to_linear(0.04), 0.04 / 12.92);
        assert!((srgb_to_linear(0.5) - 0.214_041).abs() < 1e-6);
        assert!((decode(srgb8([255, 128, 0])) - Vec3::new(1.0, 0.215_861, 0.0)).abs().max_element() < 1e-5);
        let c = Vec3::new(0.02, 0.03, 0.05);
        let n = chroma(c);
        assert!((luminance(n) - 1.0).abs() < 1e-6);
        assert!((n / n.y - c / c.y).abs().max_element() < 1e-6, "hue kept");
        assert_eq!(chroma(Vec3::ZERO), Vec3::ONE);
    }

    #[test]
    fn tint_curves_carry_the_chroma_at_full_brightness() {
        let mut w = WeatherIni::new(None);
        let (t, f) = day_times(&mut w);
        let clear = colors(WeatherType::Clear, &mut w).unwrap();
        for ty in [WeatherType::Clear, WeatherType::Overcast, WeatherType::Ashstorm] {
            let own = colors(ty, &mut w).unwrap();
            let curves = tint_curves(ty, &mut w);
            for (h, want) in [(0.0, own.night), (7.75, own.sunrise), (12.0, own.day), (19.125, own.sunset), (23.0, own.night)] {
                let got = sample(&curves, h);
                assert!((got - chroma(decode(want))).abs().max_element() < 1e-4, "{ty:?} {h} h: {got}");
                assert!((luminance(got) - 1.0).abs() < 1e-4, "{ty:?} {h} h: brightness unchanged");
            }
            // Between the keys the curve follows the gamma-space lerp closely.
            for i in 0..=2400_u16 {
                let h = f32::from(i) / 100.0;
                let exact = tint_at(&own, &clear, h, &t, &f, 0.0);
                let got = sample(&curves, h);
                assert!((got - exact).abs().max_element() < 5e-3, "{ty:?} {h} h: {got} vs {exact}");
            }
        }
        // Full relative brightness: Overcast's night over Clear's.
        let overcast = tint_curves_with(WeatherType::Overcast, &mut w, 1.0);
        let o = colors(WeatherType::Overcast, &mut w).unwrap();
        let ratio = luminance(decode(o.night)) / luminance(decode(clear.night));
        assert!((luminance(sample(&overcast, 0.0)) - ratio).abs() < 1e-3, "{ratio}");
        assert!(ratio > 2.0, "vanilla overcast nights are brighter: {ratio}");
    }

    #[test]
    #[ignore = "measurement: the vanilla night tints, linear at luminance 1"]
    fn print_night_tints() {
        let mut w = WeatherIni::new(None);
        for ty in WeatherType::ALL {
            let n = sample(&tint_curves(ty, &mut w), 0.0);
            println!("{ty:?}: ({:.3}, {:.3}, {:.3})", n.x, n.y, n.z);
        }
    }
}
