//! The visual fields of the presets (we2b): clouds, sky medium, fog, wind,
//! and the Red Mountain storm source (weather-adapter.md "As built: WE2
//! part 2").
//!
//! Clouds, haze, dust and fog carry the values the app's scaffold tables
//! (`exterior.rs` `CLOUD_CHOICES`, `SKY_*`, `FogLook`, `morning_haze`,
//! `ground_fog`) hard-code as of we2b, so the app's switch to these presets
//! (WE3) changes nothing on screen. `Land Fog Day/Night Depth` is not read:
//! its distances are fractions of the original's short view range, and the
//! fog here is calibrated on our 1-5 km views instead (weather-adapter.md
//! "Fog"); a mod that edits the ini fog depth changes nothing yet.

use glam::{DVec3, Vec3};
use sf_coords::WorldPos;
use sf_weather::sf_atmos::fog::extinction_for_visibility;
use sf_weather::sf_atmos::{CloudPreset, CloudWeather};
use sf_weather::sf_sky::KeyCurve;
use sf_weather::sf_wind::WindSource;
use sf_weather::{CloudTerms, FogTerms, SkyTerms, WeatherPreset, WindTerms};

use crate::ini::WeatherIni;
use crate::{UNIT_METERS, WeatherType};

/// Cloud layer heights × this: Morrowind's world is compressed, so the
/// genus layers come down with it (the app's `CLOUD_WORLD_SCALE`).
pub const CLOUD_WORLD_SCALE: f32 = 0.5;

/// Deepest cloud layer thickness on Morrowind's compressed island (~4.5 km across,
/// Red Mountain ~271 m): Earth-scale cumulonimbus halved (`4 000 m`) still towers
/// into the stratosphere and stretches the `WE4b` raymarch envelope shell (WE-5).
pub const MAX_CLOUD_THICKNESS_M: f32 = 1_600.0;

/// Genus preset scaled to Morrowind's world heights and storm ceiling.
#[must_use]
pub fn mw_clouds(genus: CloudPreset) -> CloudWeather {
    let mut w = CloudWeather::preset(genus).scaled_heights(CLOUD_WORLD_SCALE);
    w.thickness = w.thickness.min(MAX_CLOUD_THICKNESS_M);
    w
}

/// Cloud-altitude wind per unit of `Cloud Speed`, m/s (the app's
/// `CLOUD_SPEED_MPS`): real-world speeds read frozen beside a 30x clock.
pub const CLOUD_SPEED_MPS: f32 = 10.0;

/// Ground-fog bank extinction at the morning peak, 1/m (the app's `BANK_DENSITY`).
pub const BANK_DENSITY: f32 = 0.03;
/// Bank layer falloff, meters (the app's `BANK_FALLOFF`).
pub const BANK_FALLOFF: f32 = 12.0;

/// Ground wind at `Wind Speed` 0 and 1, m/s at 10 m (weather-adapter.md
/// "Wind Speed": linear, 1-15 m/s); the speed wanders ±25 % around it.
pub const WIND_SPEED_RANGE: [f32; 2] = [1.0, 15.0];

/// Red Mountain's summit (the highest `LAND` vertex, cell (3, 9), 271 m, in
/// `Morrowind.esm`), Morrowind units. The real-install test re-finds it.
pub const RED_MOUNTAIN_SUMMIT: [f32; 3] = [29_568.0, 73_728.0, 18_952.0];
/// The ash storm blowing off the mountain: peak speed (m/s), full inside
/// the crater, fading out across the Ashlands and Molag Amur (meters; the
/// island is ~5 km across).
const STORM_STRENGTH: f32 = 6.0;
const STORM_INNER_M: f32 = 300.0;
const STORM_RADIUS_M: f32 = 3_000.0;

/// The radial storm source around Red Mountain (`Climate::storm_source` of
/// every climate with ash or blight; presets with `wind.storm` switch it on).
pub fn red_mountain_storm() -> WindSource {
    let [x, y, z] = RED_MOUNTAIN_SUMMIT.map(f64::from);
    let center = WorldPos::from_f64(DVec3::new(x, z, -y) * f64::from(UNIT_METERS));
    WindSource::radial(center, STORM_STRENGTH, STORM_RADIUS_M).with_inner_radius(STORM_INNER_M)
}

/// One weather type's look (the app's `CloudChoice` genus and `SkyLook`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    pub genus: CloudPreset,
    pub haze: f32,
    pub dust: f32,
    pub dust_tint: Vec3,
    /// Height-fog visibility at sea level, meters (Koschmieder).
    pub visibility: f32,
    /// Height-fog falloff, meters.
    pub fog_height: f32,
    /// Ground-fog banks at their dawn peak (× [`BANK_DENSITY`]) and the
    /// fraction left by day.
    pub banks: f32,
    pub banks_day: f32,
    pub albedo: Vec3,
}

const fn look(genus: CloudPreset, haze: f32, visibility: f32, fog_height: f32, banks: f32) -> Look {
    Look {
        genus,
        haze,
        dust: 0.0,
        dust_tint: Vec3::ONE,
        visibility,
        fog_height,
        banks,
        banks_day: 0.0,
        albedo: Vec3::ONE,
    }
}

/// Grey-brown ash (the dust term's colour).
const ASH_ALBEDO: Vec3 = Vec3::new(0.55, 0.47, 0.38);

impl Look {
    /// Weather → genus follows each weather's sky texture (weather-adapter.md
    /// "Clouds"); haze, dust and fog are the F1/F2 calibration.
    pub const fn of(ty: WeatherType) -> Self {
        use CloudPreset as Cp;
        let ash = Look {
            dust: 4.0,
            albedo: ASH_ALBEDO,
            ..look(Cp::Stratocumulus, 24.0, 450.0, 500.0, 0.0)
        };
        match ty {
            WeatherType::Clear => look(Cp::Clear, 20.0, 40_000.0, 100.0, 0.0),
            WeatherType::Cloudy => look(Cp::Fair, 24.0, 30_000.0, 120.0, 0.0),
            WeatherType::Foggy => Look {
                banks_day: 0.35,
                ..look(Cp::Altostratus, 60.0, 3_000.0, 150.0, 1.0)
            },
            WeatherType::Overcast => look(Cp::Overcast, 32.0, 8_000.0, 300.0, 0.0),
            WeatherType::Rain | WeatherType::Snow => look(Cp::Overcast, 48.0, 3_000.0, 250.0, 0.25),
            WeatherType::Thunderstorm => look(Cp::Storm, 48.0, 2_000.0, 250.0, 0.3),
            WeatherType::Ashstorm => ash,
            // Ash tinted red: more green/blue absorption leaves red.
            WeatherType::Blight => Look {
                dust_tint: Vec3::new(0.6, 1.4, 1.9),
                albedo: Vec3::new(0.62, 0.42, 0.36),
                ..ash
            },
            WeatherType::Blizzard => look(Cp::Overcast, 80.0, 300.0, 400.0, 0.0),
        }
    }
}

/// Morning haze multiplier (the app's `morning_haze`): ×1.5 at 6:30, back
/// to 1 by 10:00.
pub fn morning_haze(hour: f32) -> f32 {
    let x = if hour < 6.5 {
        (hour - 4.0) / 2.5
    } else {
        (10.0 - hour) / 3.5
    };
    let x = x.clamp(0.0, 1.0);
    1.0 + 0.5 * x * x * (3.0 - 2.0 * x)
}

/// Ground fog by hour (the app's `ground_fog`): 1 at dawn, gone by 11 h,
/// half from 19 h through the night.
pub fn ground_fog(hour: f32) -> f32 {
    let s = |a: f32, b: f32, x: f32| {
        let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    };
    let g = if hour < 12.0 {
        0.5 + 0.5 * s(2.5, 4.5, hour) - s(8.5, 11.0, hour)
    } else {
        0.5 * s(19.0, 23.0, hour)
    };
    g.max(0.0)
}

/// `f` over the hour of day as a linear [`KeyCurve`]: sampled every quarter
/// hour, keys that their neighbours' interpolation reproduces dropped.
#[allow(clippy::many_single_char_names)]
pub fn hour_curve(f: impl Fn(f32) -> f32) -> KeyCurve {
    #[allow(clippy::cast_precision_loss)]
    let keys: Vec<(f32, f32)> = (0..=96)
        .map(|i| (i as f32 * 0.25, f(i as f32 * 0.25)))
        .collect();
    let mut out = vec![keys[0]];
    for pair in keys[1..].windows(2) {
        let (a, b, c) = (out[out.len() - 1], pair[0], pair[1]);
        let t = (b.0 - a.0) / (c.0 - a.0);
        if (a.1 + (c.1 - a.1) * t - b.1).abs() > 1e-4 {
            out.push(b);
        }
    }
    out.push(keys[keys.len() - 1]);
    KeyCurve::new(out)
}

/// Fills `p`'s clouds, sky, fog, wind and lightning fade for `ty`.
pub(crate) fn apply(ty: WeatherType, p: &mut WeatherPreset, w: &mut WeatherIni<'_>) {
    let section = format!("Weather {}", ty.name());
    let look = Look::of(ty);

    let cloud_speed = w
        .f32(&section, "Cloud Speed")
        .filter(|s| *s >= 0.0)
        .unwrap_or(1.5);
    p.clouds = CloudTerms {
        speed: cloud_speed * CLOUD_SPEED_MPS,
        ..CloudTerms::from_weather(&mw_clouds(look.genus))
    };

    p.sky = SkyTerms {
        haze: look.haze,
        dust: look.dust,
        dust_tint: look.dust_tint,
        haze_by_hour: hour_curve(morning_haze),
        sun_glare: w.f32(&section, "Glare View").map_or(1.0, |g| g.clamp(0.0, 1.0)),
        ..SkyTerms::default()
    };

    let mut fog = FogTerms {
        height_density: extinction_for_visibility(look.visibility),
        height_falloff: look.fog_height,
        height_base: 0.0,
        bank_density: BANK_DENSITY * look.banks,
        bank_falloff: BANK_FALLOFF,
        albedo: look.albedo,
        // Dawn and night fog thicken the height fog by half.
        height_by_hour: hour_curve(|h| 1.0 + 0.5 * ground_fog(h)),
        ..FogTerms::default()
    };
    if look.banks > 0.0 {
        fog.banks_by_hour = hour_curve(|h| look.banks_day + (1.0 - look.banks_day) * ground_fog(h));
    }
    p.fog = fog;

    let wind = w.f32(&section, "Wind Speed").unwrap_or(0.2).max(0.0);
    let mean = WIND_SPEED_RANGE[0] + (WIND_SPEED_RANGE[1] - WIND_SPEED_RANGE[0]) * wind;
    p.wind = WindTerms {
        speed: [mean * 0.75, mean * 1.25],
        storm: if matches!(ty, WeatherType::Ashstorm | WeatherType::Blight) {
            1.0
        } else {
            0.0
        },
        ..WindTerms::default()
    };

    // `Flash Decrement`: flash brightness lost per real second.
    if let Some(l) = &mut p.lightning
        && let Some(d) = w.f32(&section, "Flash Decrement").filter(|d| *d > 0.0)
    {
        l.fade_s = 1.0 / d;
    }
}
