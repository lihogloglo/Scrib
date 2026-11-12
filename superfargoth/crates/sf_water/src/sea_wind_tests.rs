//! W-WIND tests (`sea_wind.rs`): phases kept, continuous monotone blend, rate limit, override, calm.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use glam::{DVec3, Vec2};
use sf_coords::WorldPos;

use crate::*;

fn params(speed: f32, toward: Vec2) -> SpectrumParams {
    SpectrumParams { wind: WindInput { speed, toward }, ..SpectrumParams::default() }
}

const STEP: f64 = 1.0 / 60.0;

/// Runs `sea` on a constant weather wind for `seconds` from `from`: (time, speed) per frame and
/// the re-init times.
fn run(sea: &mut SeaWind, weather: Vec2, from: f64, seconds: f64) -> (Vec<(f64, f32)>, Vec<f64>) {
    let (mut samples, mut inits) = (Vec::new(), Vec::new());
    for i in 0..(seconds / STEP) as usize {
        let t = from + i as f64 * STEP;
        if sea.update(Some(weather), t) {
            inits.push(t);
        }
        samples.push((t, sea.speed(t)));
    }
    (samples, inits)
}

#[test]
fn a_wind_change_keeps_every_phase() {
    let c = Cascades::default();
    for cascade in 0..c.len() {
        let a = H0::generate(&params(4.0, Vec2::new(0.6, 0.8)), &c, cascade, 64, 7);
        let b = H0::generate(&params(18.0, Vec2::new(-0.8, 0.6)), &c, cascade, 64, 7);
        let mut shared = 0;
        for (x, y) in a.h0.iter().zip(&b.h0) {
            if x.norm() > 1e-12 && y.norm() > 1e-12 {
                shared += 1;
                assert!((x.arg() - y.arg()).abs() < 1e-4, "cascade {cascade}: phase {} vs {}", x.arg(), y.arg());
            }
        }
        assert!(shared > 0, "cascade {cascade} shares bins");
    }
}

#[test]
fn the_sea_snaps_to_the_first_weather_then_follows_smoothly_and_monotonically() {
    let mut sea = SeaWind::new(WindInput { speed: 7.0, toward: Vec2::X }, SeaWindSettings::default());
    assert!(sea.update(Some(Vec2::new(2.0, 0.0)), 0.0), "the first weather is a re-init");
    assert_eq!(sea.wind().speed, 2.0);
    assert!(sea.transition().is_none(), "no ramp at load");
    let (rise, inits) = run(&mut sea, Vec2::new(20.0, 0.0), STEP, 400.0);
    assert!(rise.windows(2).all(|w| w[1].1 >= w[0].1), "speed only rises");
    let max_step = rise.windows(2).map(|w| w[1].1 - w[0].1).fold(0.0, f32::max);
    assert!(max_step < 0.05, "continuous: largest per-frame step {max_step} m/s");
    let last = rise.last().expect("frames");
    assert!((last.1 - 20.0).abs() <= 20.0 * sea.settings.speed_step + 1e-3, "reaches the storm ({})", last.1);
    assert!(inits.len() > 5, "many small transitions, not one jump ({})", inits.len());
    let (fall, _) = run(&mut sea, Vec2::new(1.0, 0.0), last.0 + STEP, 400.0);
    assert!(fall.windows(2).all(|w| w[1].1 <= w[0].1), "speed only falls");
    assert!(fall.last().expect("frames").1 < 1.5);
}

#[test]
fn re_inits_are_rate_limited() {
    let s = SeaWindSettings::default();
    let mut sea = SeaWind::new(WindInput { speed: 0.0, toward: Vec2::X }, s);
    sea.update(Some(Vec2::ZERO), 0.0);
    let (_, inits) = run(&mut sea, Vec2::new(0.0, 30.0), STEP, 300.0);
    assert!(inits.windows(2).all(|w| w[1] - w[0] >= f64::from(s.blend_s) - 1e-9), "{inits:?}");
    // A steady wind costs nothing once the sea has caught up.
    let late = run(&mut sea, Vec2::new(0.0, 30.0), 300.0, 600.0).1.into_iter().filter(|t| *t > 500.0).count();
    assert_eq!(late, 0);
}

#[test]
fn the_override_beats_the_weather() {
    let pinned = WindInput { speed: 7.0, toward: Vec2::new(0.6, 0.8) };
    let mut sea = SeaWind::new(WindInput { speed: 3.0, toward: Vec2::X }, SeaWindSettings::default()).with_fixed(Some(pinned));
    assert!(sea.update(Some(Vec2::new(25.0, 0.0)), 0.0));
    for i in 1..600 {
        assert!(!sea.update(Some(Vec2::new(25.0, 0.0)), f64::from(i) * STEP));
    }
    assert_eq!(sea.wind(), pinned);
    assert!(sea.transition().is_none());
}

#[test]
fn no_nan_at_zero_wind() {
    let w = wind_input(Vec2::ZERO, Vec2::ZERO, 0.5);
    assert!(w.speed == 0.0 && (w.toward.length() - 1.0).abs() < 1e-6);
    let p = SpectrumParams { wind: w, ..SpectrumParams::default() };
    let c = Cascades::default();
    for cascade in 0..c.len() {
        let h = H0::generate(&p, &c, cascade, 64, 1);
        assert!(h.h0.iter().all(|z| z.re.is_finite() && z.im.is_finite()) && h.variance.is_finite());
    }
    let sw = ShoreWaves::new(&p, &c, &ShoreSettings::default());
    let g = sw.eval(5.0, 1.0, Vec2::X, Vec2::splat(0.3), 10.0);
    assert!(g.offset.is_finite() && g.slope.is_finite() && sw.amplitude.is_finite() && sw.swash.is_finite());
    let mut twin = WaveTwin::blended(&p, &params(12.0, Vec2::X), &c, 1, 1, 32);
    twin.set_mix(0.5);
    twin.evaluate(3.0);
    assert!(twin.sample(WorldPos::from_f64(DVec3::new(1.0, 0.0, 2.0))).height.is_finite());
}

#[test]
fn a_blend_starts_on_the_old_sea_and_ends_on_the_new_one() {
    let (a, b) = (params(5.0, Vec2::X), params(15.0, Vec2::new(0.0, 1.0)));
    let c = Cascades::default();
    let at = WorldPos::from_f64(DVec3::new(13.0, 0.0, -7.0));
    let height = |twin: &mut WaveTwin| {
        twin.evaluate(42.0);
        twin.sample(at).height
    };
    let (ha, hb) = (height(&mut WaveTwin::new(&a, &c, 9, 1, 64)), height(&mut WaveTwin::new(&b, &c, 9, 1, 64)));
    assert!((ha - hb).abs() > 1e-3, "the two seas differ");
    let mut blend = WaveTwin::blended(&a, &b, &c, 9, 1, 64);
    blend.set_mix(0.0);
    assert!((height(&mut blend) - ha).abs() < 1e-5);
    blend.set_mix(1.0);
    assert!((height(&mut blend) - hb).abs() < 1e-5);
    // Linear in the weight (the FFT is linear): the midpoint grid is the mean grid.
    let grid = |twin: &mut WaveTwin| {
        twin.evaluate(42.0);
        twin.cascades[0].grids().height.clone()
    };
    let (ga, gb) = (grid(&mut WaveTwin::new(&a, &c, 9, 1, 64)), grid(&mut WaveTwin::new(&b, &c, 9, 1, 64)));
    blend.set_mix(0.5);
    let gm = grid(&mut blend);
    assert!(gm.iter().zip(ga.iter().zip(&gb)).all(|(m, (x, y))| (m - 0.5 * (x + y)).abs() < 1e-5));
    // Shore waves: the same ends, and a continuous crossfade between.
    let s = ShoreSettings::default();
    let (sa, sb) = (ShoreWaves::new(&a, &c, &s), ShoreWaves::new(&b, &c, &s));
    let eval = |w: &ShoreWaves| w.eval(4.0, 1.0, Vec2::X, Vec2::splat(0.25), 17.0);
    assert_eq!(eval(&ShoreWaves::blend(&sa, &sb, 0.0)).offset, eval(&sa).offset);
    assert_eq!(eval(&ShoreWaves::blend(&sa, &sb, 1.0)).offset, eval(&sb).offset);
    let ys: Vec<f32> = (0..=100).map(|i| eval(&ShoreWaves::blend(&sa, &sb, i as f32 / 100.0)).offset.y).collect();
    let jump = ys.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0, f32::max);
    assert!(jump < 0.05, "crossfade step {jump} m per 1 % of weight");
}

#[test]
#[ignore = "measurement: CPU cost of one spectrum re-init (h0 for 4 cascades at 256²)"]
fn measure_reinit_cost() {
    let c = Cascades::default();
    let p = params(12.0, Vec2::new(0.6, 0.8));
    let runs = 5;
    let start = std::time::Instant::now();
    for _ in 0..runs {
        for cascade in 0..c.len() {
            let h = H0::generate(&p, &c, cascade, 256, 1);
            std::hint::black_box(h.gpu_texels(&p));
        }
    }
    println!("re-init: {:.2} ms per sea state (4 cascades x 256², with texels)", start.elapsed().as_secs_f64() * 1e3 / f64::from(runs));
    let start = std::time::Instant::now();
    for _ in 0..runs {
        for cascade in 0..c.len() {
            std::hint::black_box(H0::generate(&p, &c, cascade, 256, 1));
        }
    }
    println!("  of which h0 alone: {:.2} ms", start.elapsed().as_secs_f64() * 1e3 / f64::from(runs));
    let start = std::time::Instant::now();
    for _ in 0..runs {
        std::hint::black_box(WaveTwin::blended(&p, &params(14.0, Vec2::X), &c, 1, 1, 64));
    }
    println!("CPU twin rebuild: {:.3} ms", start.elapsed().as_secs_f64() * 1e3 / f64::from(runs));
}
