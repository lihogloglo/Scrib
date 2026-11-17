//! W-SEASTATE: the weather → sea wind curve and the storm's sea state on the new fetch.

use glam::Vec2;
use sf_water::{SpectrumParams, WindInput};

use crate::sea_state::{SeaStateCurve, CLASSIC_FETCH_M, CURVE_POINTS, SEA_FETCH_M, SEA_STATE_POINTS, TUNABLES};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn curve_is_exact_at_its_points_and_linear_between() {
    let c = SeaStateCurve::default();
    let expected: [[f32; 2]; CURVE_POINTS] = [[2.4, 2.6], [3.8, 4.5], [5.2, 7.0], [8.0, 16.0], [12.0, 22.0], [16.0, 26.0]];
    for (have, [u, sea]) in c.points.iter().zip(expected) {
        assert!(close(have[0], u) && close(have[1], sea), "default point {have:?} vs {:?}", [u, sea]);
        assert!(close(c.sea_wind(u), sea), "at {u}: {} vs {sea}", c.sea_wind(u));
    }
    // Midpoints and a quarter point.
    assert!(close(c.sea_wind(6.6), 11.5), "{}", c.sea_wind(6.6));
    assert!(close(c.sea_wind(10.0), 19.0), "{}", c.sea_wind(10.0));
    assert!(close(c.sea_wind(9.0), 17.5), "{}", c.sea_wind(9.0));
}

#[test]
fn curve_clamps_at_its_ends_and_has_no_nan() {
    let c = SeaStateCurve::default();
    for u in [0.0, 1.0, 2.4] {
        assert!(close(c.sea_wind(u), 2.6), "below the first point: {}", c.sea_wind(u));
    }
    for u in [16.0, 20.0, 60.0, 1e6] {
        assert!(close(c.sea_wind(u), 26.0), "above the last point: {}", c.sea_wind(u));
    }
    for u in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -3.0] {
        let s = c.sea_wind(u);
        assert!(s.is_finite() && close(s, 2.6), "{u} -> {s}");
    }
    // The velocity keeps its direction; a calm stays a calm, nothing becomes NaN.
    let v = c.sea_velocity(Vec2::new(0.0, 8.0));
    assert!(close(v.x, 0.0) && close(v.y, 16.0), "{v}");
    let d = c.sea_velocity(Vec2::new(3.0, 4.0));
    assert!(close(d.length(), c.sea_wind(5.0)) && close(d.normalize().dot(Vec2::new(0.6, 0.8)), 1.0), "{d}");
    assert_eq!(c.sea_velocity(Vec2::ZERO), Vec2::ZERO);
    assert_eq!(c.sea_velocity(Vec2::new(f32::NAN, 1.0)), Vec2::ZERO);
}

#[test]
fn curve_is_monotone_even_when_edited_out_of_order() {
    let c = SeaStateCurve::default();
    let mut last = c.sea_wind(0.0);
    for i in 1..=400u16 {
        let u = f32::from(i) * 0.05;
        let s = c.sea_wind(u);
        assert!(s >= last - 1e-5, "default curve drops at {u}: {s} < {last}");
        last = s;
    }
    // A live edit out of order (p2 before p1 by weather wind, p5's sea wind below p4's, a NaN).
    let edited = SeaStateCurve { points: [[3.8, 4.5], [2.4, 2.6], [5.2, f32::NAN], [8.0, 16.0], [12.0, 10.0], [16.0, 26.0]] };
    assert!(close(edited.sea_wind(2.4), 2.6) && close(edited.sea_wind(3.8), 4.5), "sorted by weather wind");
    assert!(close(edited.sea_wind(5.2), 4.5), "a NaN sea wind holds the point before: {}", edited.sea_wind(5.2));
    assert!(close(edited.sea_wind(12.0), 16.0), "a lower sea wind is raised: {}", edited.sea_wind(12.0));
    let mut last = edited.sea_wind(0.0);
    for i in 1..=400u16 {
        let s = edited.sea_wind(f32::from(i) * 0.05);
        assert!(s.is_finite() && s >= last - 1e-5, "edited curve: {s} after {last}");
        last = s;
    }
    // Two points at one weather wind: no division by zero.
    let step = SeaStateCurve { points: [[2.0, 2.0], [4.0, 4.0], [4.0, 9.0], [8.0, 16.0], [12.0, 22.0], [16.0, 26.0]] };
    assert!([3.9, 4.0, 4.1].iter().all(|u| step.sea_wind(*u).is_finite()));
}

#[test]
fn fetch_tunable_and_classic_arm() {
    assert!((SEA_FETCH_M.default_value() - 50_000.0).abs() < 1e-3);
    let (lo, hi) = SEA_FETCH_M.bounds().expect("a ranged number");
    assert!((lo - 5_000.0).abs() < 1e-3 && (hi - 200_000.0).abs() < 1e-3);
    assert!((CLASSIC_FETCH_M - 20_000.0).abs() < 1e-3);
    // Every tunable of the module is in the catalog list, the points in curve order.
    assert_eq!(TUNABLES.len(), 1 + CURVE_POINTS);
    assert_eq!(SEA_STATE_POINTS[0].name(), "water.sea_state_curve.p1");
    assert_eq!(SEA_STATE_POINTS[CURVE_POINTS - 1].name(), "water.sea_state_curve.p6");
    let mut store = sf_tunables::Tunables::default();
    store.register_all(TUNABLES).expect("valid names, defaults in range");
}

/// Hs = 4·sqrt(m0) over the spectrum's band from the 256 m tile's longest wave (the diagnosis note's band).
fn hs(wind: f32, fetch: f32) -> f64 {
    let p = SpectrumParams { wind: WindInput { speed: wind, toward: Vec2::X }, fetch, ..SpectrumParams::default() };
    4.0 * p.band_variance(std::f64::consts::TAU / 256.0, 50.0).sqrt()
}

#[test]
fn storm_sea_state_reaches_about_two_metres() {
    let c = SeaStateCurve::default();
    // The thunderstorm's mean 10 m wind (`mw_weather::visual`: 1 + 14·0.5).
    let sea = c.sea_wind(8.0);
    assert!(close(sea, 16.0));
    let storm = hs(sea, SEA_FETCH_M.default_value());
    assert!((storm / 2.2 - 1.0).abs() < 0.1, "storm Hs {storm:.2} m, expected ~2.2 m");
    // The old storm (verbatim 8 m/s, 20 km) was ~0.72 m: the curve and fetch triple it.
    let classic = hs(8.0, CLASSIC_FETCH_M);
    assert!((classic / 0.72 - 1.0).abs() < 0.1, "classic storm Hs {classic:.2} m");
    // Clear stays a small sea.
    let clear = hs(c.sea_wind(2.4), SEA_FETCH_M.default_value());
    assert!(clear < 0.6, "clear Hs {clear:.2} m");
    // The peak wavelength stays inside cascade 0's 256 m tile with 3+ waves per tile.
    let p = SpectrumParams { wind: WindInput { speed: c.sea_wind(1e3), toward: Vec2::X }, fetch: SEA_FETCH_M.default_value(), ..SpectrumParams::default() };
    let wp = p.omega_peak();
    let lambda = std::f64::consts::TAU * f64::from(sf_water::GRAVITY) / (wp * wp);
    assert!(lambda < 256.0 / 3.0, "λp {lambda:.0} m at the curve's cap");
}

/// WA-10 item 1: before the weather is valid the sea gets no wind (no stale pre-weather default to
/// lock onto), and its first wind after is the curve of the weather's mid speed, taken as-is (snap,
/// no low-pass), whatever the wandering wind's speed; only its direction is kept.
#[test]
fn first_sea_wind_is_the_curve_of_the_weathers_mid_speed() {
    use crate::sea_state::{expected_hs, weather_sea_wind};
    use sf_water::{SeaWind, SeaWindSettings};
    let curve = SeaStateCurve::default();
    // The pre-weather default wind (`sf_wind::Wind::default`, 2.06 m/s) the old feed copied.
    let stale = Vec2::new(2.0, 0.5);
    let mut sea = SeaWind::new(WindInput { speed: 0.0, toward: Vec2::X }, SeaWindSettings::default());
    for frame in 0..10u8 {
        let t = f64::from(frame) / 60.0;
        assert_eq!(weather_sea_wind(false, stale, 0.0), None);
        assert!(!sea.update(weather_sea_wind(false, stale, 0.0).map(|v| curve.sea_velocity(v)), t));
    }
    // The thunderstorm: mid speed 8 m/s (`mw_weather::visual`), wandering at 9.7 m/s toward -Y.
    let wandering = Vec2::new(0.0, -9.7);
    let feed = weather_sea_wind(true, wandering, 8.0).map(|v| curve.sea_velocity(v));
    assert!(sea.update(feed, 0.5), "the first valid wind commits");
    assert!(sea.transition().is_none(), "the first wind snaps");
    let w = sea.wind();
    assert!(close(w.speed, curve.sea_wind(8.0)) && close(w.speed, 16.0), "sea wind {}", w.speed);
    assert!(close(w.toward.x, 0.0) && close(w.toward.y, -1.0), "direction kept: {:?}", w.toward);
    // The wander alone (same mid speed, another speed and direction nearby) commits no new sea state.
    let feed = weather_sea_wind(true, Vec2::new(0.3, -6.1), 8.0).map(|v| curve.sea_velocity(v));
    assert!(!sea.update(feed, 0.6));
    // The log line's expected Hs is the spectrum's own band height (~2.2 m for the storm).
    let hs16 = f64::from(expected_hs(16.0, SEA_FETCH_M.default_value()));
    assert!((hs16 / hs(16.0, SEA_FETCH_M.default_value()) - 1.0).abs() < 1e-4, "Hs {hs16:.3} m");
}
