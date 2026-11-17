//! The sea state the weather gives (W-SEASTATE; `superfargoth/docs/research/storm-sea-diag-2026-11-17.md`).
//!
//! Morrowind's `Wind Speed` is a ground-wind scale for foliage and particles: `mw_weather::visual`
//! makes it a 10 m wind of `1 + 14·s` m/s (clear 2.4, rain 5.2, thunderstorm 8, Beaufort 4-5), and
//! copied verbatim to the sea on a 20 km fetch it gave a storm Hs of 0.72 m ("super tame", owner
//! 2026-11-17). Shipped games author the sea state per weather and exaggerate it (Sea of Thieves, AC IV,
//! RDR2's weather-keyed sea sets; Crest documents raising wind and fetch for storms): the weather's
//! 10 m wind goes through an authored, piecewise-linear [`SeaStateCurve`] to the spectrum's wind, on
//! a longer fetch ([`SEA_FETCH_M`]). Thunderstorm 8 → 16 m/s at 50 km: Hs 2.2 m, peak wavelength 52 m
//! (inside cascade 0's 256 m tile). `--off seastate` keeps the verbatim copy and [`CLASSIC_FETCH_M`]
//! (the A/B arm). The framework takes wind and fetch as plain inputs (`sf_water::SpectrumParams`):
//! the weather convention stays here.

use glam::Vec2;
use sf_tunables::{def, AnyDef, Def, Group, Source, Unit};

/// Where the curve and the fetch come from.
const SOURCE: Source = Source::Industry("per-weather authored sea state (storm-sea-diag-2026-11-17)");

/// `water.sea_fetch_m`: open water upwind of the sea state (JONSWAP fetch). 50 km lets the storm's
/// 16 m/s reach Hs 2.2 m, λp 52 m; the clear sea stays ~0.4 m.
pub const SEA_FETCH_M: Def<f32> = def!(float "water.sea_fetch_m" = 50_000.0)
    .range(5_000.0, 200_000.0)
    .unit(Unit::Meters)
    .help("Open-sea fetch of the wave spectrum (JONSWAP), m: longer = higher, longer waves for the same wind")
    .source(SOURCE)
    .group(Group::Water);

/// The fetch before W-SEASTATE (water-adapter.md "Openness and fetch"), kept for `--off seastate`.
pub const CLASSIC_FETCH_M: f32 = 20_000.0; // tunables: const (the `--off seastate` A/B arm: the W2 fetch)

/// Points of [`SeaStateCurve`].
pub const CURVE_POINTS: usize = 6;

/// `water.sea_state_curve.p1` .. `p6`: the curve's points, `[weather 10 m wind, sea wind]`, both m/s.
/// Defaults at the weather classes' mean winds (clear, cloudy, rain, thunderstorm, ~ash/blight
/// storm, ~blizzard and beyond); the cap of 26 m/s keeps Hs ≤ ~3 m at 50 km.
pub const SEA_STATE_P1: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p1" = [2.4, 2.6])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 1 (clear): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);
pub const SEA_STATE_P2: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p2" = [3.8, 4.5])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 2 (cloudy): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);
pub const SEA_STATE_P3: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p3" = [5.2, 7.0])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 3 (rain): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);
pub const SEA_STATE_P4: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p4" = [8.0, 16.0])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 4 (thunderstorm): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);
pub const SEA_STATE_P5: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p5" = [12.0, 22.0])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 5 (ash storm): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);
pub const SEA_STATE_P6: Def<[f32; 2]> = def!(float2 "water.sea_state_curve.p6" = [16.0, 26.0])
    .range([0.0, 0.0], [60.0, 60.0])
    .help("Sea state curve point 6 (blizzard, and the cap beyond): [weather 10 m wind, sea wind], m/s")
    .source(SOURCE)
    .group(Group::Water);

/// The curve's point tunables, in order (`SeaStateCurve::points[i]` is bound to `SEA_STATE_POINTS[i]`).
pub const SEA_STATE_POINTS: [&Def<[f32; 2]>; CURVE_POINTS] =
    [&SEA_STATE_P1, &SEA_STATE_P2, &SEA_STATE_P3, &SEA_STATE_P4, &SEA_STATE_P5, &SEA_STATE_P6];

/// This module's tunables, for the app's tunables catalog.
pub const TUNABLES: &[&dyn AnyDef] =
    &[&SEA_FETCH_M, &SEA_STATE_P1, &SEA_STATE_P2, &SEA_STATE_P3, &SEA_STATE_P4, &SEA_STATE_P5, &SEA_STATE_P6];

/// The weather's 10 m wind → the sea's spectrum wind, m/s: linear between points, clamped beyond the
/// first and last, never decreasing. Points are taken in order of their weather wind and each point's
/// sea wind is raised to at least the one before it, so a live edit can't make a stronger wind give a
/// calmer sea.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeaStateCurve {
    /// `[weather wind, sea wind]`, m/s.
    pub points: [[f32; 2]; CURVE_POINTS],
}

impl Default for SeaStateCurve {
    fn default() -> Self {
        Self { points: SEA_STATE_POINTS.map(Def::default_value) }
    }
}

impl SeaStateCurve {
    /// The points sorted by weather wind, with non-decreasing (and finite, ≥ 0) sea winds.
    fn monotone(&self) -> [[f32; 2]; CURVE_POINTS] {
        let mut points = self.points;
        points.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let mut top = 0.0f32;
        for p in &mut points {
            // `max` ignores a NaN operand: a NaN sea wind takes the one before it.
            top = top.max(p[1]);
            p[1] = top;
        }
        points
    }

    /// The sea wind for a weather wind `u`, m/s (a non-finite or negative `u` reads as calm).
    pub fn sea_wind(&self, u: f32) -> f32 {
        let points = self.monotone();
        let u = if u.is_finite() { u.max(0.0) } else { 0.0 };
        let [x0, y0] = points[0];
        if u <= x0 || x0.is_nan() {
            return y0;
        }
        for w in points.windows(2) {
            let ([xa, ya], [xb, yb]) = (w[0], w[1]);
            if u <= xb {
                let span = xb - xa;
                return if span > 1e-6 { ya + (yb - ya) * (u - xa) / span } else { yb };
            }
        }
        points[CURVE_POINTS - 1][1]
    }

    /// A weather wind velocity (m/s, the way it blows) with its speed mapped by [`Self::sea_wind`] and
    /// its direction kept. A calm (zero or non-finite velocity) has no direction and stays zero (the
    /// weather never gives one: its slowest mean wind is 2.4 m/s).
    pub fn sea_velocity(&self, weather: Vec2) -> Vec2 {
        let speed = weather.length();
        match weather.try_normalize() {
            Some(dir) if speed.is_finite() => dir * self.sea_wind(speed),
            _ => Vec2::ZERO,
        }
    }
}

/// The weather wind the sea follows (WA-10 item 1): `None` until the weather is valid, so the sea's
/// first wind (`sf_water::SeaWind` snaps to its first value) is the weather's, never the pre-weather
/// default; then the weather's direction (`direction`, its wandering mean wind) at its mid speed
/// (`sf_weather::WeatherFrame::wind_mid_speed`, the presets' speed range without the slow wander,
/// which spans the curve's steepest part: thunderstorm 6-10 m/s gave a sea of 9.6-19 m/s). The
/// wander stays a direction and gust term for the rest of the world. A zero `direction` blows +X.
pub fn weather_sea_wind(valid: bool, direction: Vec2, mid_speed: f32) -> Option<Vec2> {
    if !valid {
        return None;
    }
    let speed = if mid_speed.is_finite() { mid_speed.max(0.0) } else { 0.0 };
    Some(direction.try_normalize().unwrap_or(Vec2::X) * speed)
}

/// Expected significant wave height of a fully developed sea of the spectrum's (JONSWAP) at the 10 m
/// wind `wind` (m/s) over `fetch` (m): Hs = 4·√m0 over the band from cascade 0's longest wave
/// (`sf_water::DEFAULT_CASCADES[0]`) to k = 50 rad/m. Diagnostics (the sea state log line), m.
pub fn expected_hs(wind: f32, fetch: f32) -> f32 {
    let p = sf_water::SpectrumParams {
        wind: sf_water::WindInput { speed: wind, toward: Vec2::X },
        fetch,
        ..sf_water::SpectrumParams::default()
    };
    let k_lo = std::f64::consts::TAU / f64::from(sf_water::DEFAULT_CASCADES[0]);
    #[allow(clippy::cast_possible_truncation)]
    let hs = (4.0 * p.band_variance(k_lo, 50.0).max(0.0).sqrt()) as f32;
    hs
}
