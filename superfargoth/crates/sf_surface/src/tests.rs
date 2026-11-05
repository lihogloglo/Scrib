// Exact float compares are intended where both sides run the same operations.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names, clippy::similar_names)]

use glam::DVec2;
use sf_weather::{Climate, PrecipKind, PrecipTerm, WeatherPreset};

use super::*;

/// Constant rates per climate.
struct Const(Vec<SurfaceRates>);

impl RateSource for Const {
    fn climate_count(&self) -> usize {
        self.0.len()
    }

    fn rates(&self, c: ClimateIx, _t_h: f64) -> SurfaceRates {
        self.0[c.0 as usize]
    }
}

const STEP: f64 = 1.0 / 6.0;

fn tuning() -> SurfaceTuning {
    SurfaceTuning::default()
}

fn rain(wet: f32, puddle: f32) -> SurfaceRates {
    SurfaceRates { wet, puddle, ..SurfaceRates::default() }
}

fn ws() -> Worldspace {
    RecordId::new("test/world")
}

/// Two 256 m squares side by side (climate 0 west, 1 east) from the origin, a 128 m band.
fn raster() -> ClimateRaster {
    ClimateRaster::new(DVec2::ZERO, 256.0, 2, 1, &[Some(ClimateIx(0)), Some(ClimateIx(1))], 128.0).unwrap()
}

/// A state stepped once at `t = 0`, so later advances step exactly to their target.
fn started(n: usize) -> SurfaceState {
    SurfaceState { t_h: 0.0, climates: vec![ClimateSurface::default(); n], ..SurfaceState::default() }
}

#[test]
fn wetness_reaches_one_at_one_over_rate_and_dries_in_dry_h() {
    let t = tuning();
    for r in [0.5_f32, 1.0, 2.0] {
        let mut s = started(1);
        let src = Const(vec![rain(r, 0.0)]);
        let full = 1.0 / f64::from(r);
        let mut k = 0;
        while f64::from(k) * STEP < full - 1e-9 {
            k += 1;
            advance(&mut s, f64::from(k) * STEP, &src, &[], &t);
            let closed = (f64::from(k) * STEP * f64::from(r)).min(1.0) as f32;
            assert!((s.climates[0].wet - closed).abs() < 1e-5, "r {r} step {k}: {} vs {closed}", s.climates[0].wet);
        }
        assert!(s.climates[0].wet > 1.0 - 1e-5, "r {r}: wet at 1/r h");
        // Dry (no preset sets drying): 1 → 0 in dry_h.
        let dry = Const(vec![SurfaceRates::default()]);
        s.climates[0].wet = 1.0;
        let t0 = s.t_h;
        let steps = (f64::from(t.dry_h) / STEP).round() as i32;
        advance(&mut s, t0 + f64::from(steps - 1) * STEP, &dry, &[], &t);
        assert!(s.climates[0].wet > 0.0, "still damp one step before dry_h");
        advance(&mut s, t0 + f64::from(steps) * STEP, &dry, &[], &t);
        assert!(s.climates[0].wet < 1e-5, "dry at dry_h: {}", s.climates[0].wet);
    }
}

#[test]
fn a_preset_drying_rate_replaces_dry_h() {
    let mut s = started(1);
    s.climates[0].wet = 1.0;
    let src = Const(vec![SurfaceRates { drying: 0.25, ..SurfaceRates::default() }]);
    advance(&mut s, 2.0, &src, &[], &tuning());
    assert!((s.climates[0].wet - 0.5).abs() < 1e-5, "{}", s.climates[0].wet);
}

#[test]
fn one_long_advance_equals_its_steps_bit_for_bit() {
    let g = CellGrid::new(ws(), &raster());
    let src = Const(vec![SurfaceRates { wet: 0.7, puddle: 0.4, snow: 0.3, ..SurfaceRates::default() }, SurfaceRates { ash: 0.2, ..SurfaceRates::default() }]);
    let t = tuning();
    let mut a = started(2);
    let mut b = started(2);
    assert_eq!(advance(&mut a, 24.0, &src, &[&g], &t), 144);
    for k in 1..=144 {
        assert_eq!(advance(&mut b, f64::from(k) * STEP, &src, &[&g], &t), 1);
    }
    assert_eq!(a, b);
    // A target between grid points steps to the last one before it; the remainder waits.
    assert_eq!(advance(&mut a, 24.0 + 0.5 * STEP, &src, &[&g], &t), 0);
    assert_eq!(a.t_h, b.t_h);
}

#[test]
fn catch_up_is_capped_and_a_new_state_warms_up_over_the_cap() {
    let src = Const(vec![rain(1.0, 0.0)]);
    let t = SurfaceTuning { catch_up_h: 1.0, ..tuning() };
    let mut s = started(1);
    assert_eq!(advance(&mut s, 100.0, &src, &[], &t), 6);
    assert_eq!(s.t_h, 100.0);
    let mut fresh = SurfaceState::default();
    assert!(!fresh.started());
    assert_eq!(advance(&mut fresh, 100.0, &src, &[], &t), 6);
    assert_eq!(fresh, s);
}

#[test]
fn a_clock_that_goes_back_re_anchors_without_stepping() {
    let src = Const(vec![rain(1.0, 0.0)]);
    let mut s = started(1);
    advance(&mut s, 1.0, &src, &[], &tuning());
    let wet = s.climates[0].wet;
    assert_eq!(advance(&mut s, 0.5, &src, &[], &tuning()), 0);
    assert_eq!((s.t_h, s.climates[0].wet), (0.5, wet));
}

#[test]
fn puddles_stay_empty_below_puddle_from_wet() {
    let t = tuning();
    let src = Const(vec![rain(1.0, 0.5)]);
    let mut s = started(1);
    let mut k = 0;
    while s.climates[0].wet < t.puddle_from_wet {
        assert_eq!(s.climates[0].puddle, 0.0, "step {k}");
        k += 1;
        advance(&mut s, f64::from(k) * STEP, &src, &[], &t);
    }
    // The first step that ends saturated fills.
    assert!(s.climates[0].puddle > 0.0);
    // Puddles dry at half the drying rate: 0.5 of fill lasts 2 × 0.5 × dry_h.
    s.climates[0] = ClimateSurface { wet: 1.0, puddle: 0.5 };
    let t0 = s.t_h;
    advance(&mut s, t0 + f64::from(t.dry_h) * 0.5 / f64::from(t.puddle_dry_mult) - STEP, &Const(vec![SurfaceRates::default()]), &[], &t);
    assert!(s.climates[0].puddle > 0.0);
    advance(&mut s, t0 + f64::from(t.dry_h) * 0.5 / f64::from(t.puddle_dry_mult), &Const(vec![SurfaceRates::default()]), &[], &t);
    assert!(s.climates[0].puddle < 1e-5, "{}", s.climates[0].puddle);
}

#[test]
fn snow_decays_three_and_ash_four_times_slower_than_wetness() {
    let t = tuning();
    let g = CellGrid::new(ws(), &raster());
    let mut s = started(2);
    let cell = IVec2::new(1, 1);
    s.set_cell(&ws(), cell, Channel::Snow, 1.0);
    s.set_cell(&ws(), cell, Channel::Ash, 1.0);
    let dry = Const(vec![SurfaceRates::default(); 2]);
    // Wetness takes dry_h; snow 3 dry_h, ash 4 dry_h.
    let snow_h = (f64::from(t.dry_h) / f64::from(t.snow_melt_mult)).round();
    assert_eq!(snow_h, 6.0);
    advance(&mut s, snow_h - STEP, &dry, &[&g], &t);
    let c = s.cell(&ws(), cell);
    assert!(c.snow > 0.0 && c.ash > 0.0);
    advance(&mut s, snow_h, &dry, &[&g], &t);
    let c = s.cell(&ws(), cell);
    assert!(c.snow < 1e-5, "snow gone at 3 dry_h: {}", c.snow);
    assert!((c.ash - 0.25).abs() < 1e-4, "ash at 3/4: {}", c.ash);
    advance(&mut s, (f64::from(t.dry_h) / f64::from(t.ash_clear_mult)).round(), &dry, &[&g], &t);
    assert_eq!(s.active_cells(), 0, "empty cells drop");
}

#[test]
fn cell_rates_equal_the_raster_blend_at_cell_centres() {
    let r = raster();
    let g = CellGrid::new(ws(), &r);
    let (min, size) = g.extent();
    assert_eq!((min, size), (IVec2::ZERO, IVec2::new(16, 8)));
    let rates = vec![SurfaceRates { snow: 0.6, ..SurfaceRates::default() }, SurfaceRates { snow: 0.2, ash: 0.3, ..SurfaceRates::default() }];
    let mut s = started(2);
    advance(&mut s, STEP, &Const(rates.clone()), &[&g], &tuning());
    let dt = STEP as f32;
    let mut blended = 0;
    for ix in 0..g.len() {
        let cell = g.cell(ix);
        let site = r.site_at((cell.as_dvec2() + 0.5) * CELL_M);
        blended += usize::from(site.len() > 1);
        let (mut snow, mut ash) = (0.0, 0.0);
        for (c, w) in &site {
            snow += rates[c.0 as usize].snow * w;
            ash += rates[c.0 as usize].ash * w;
        }
        let got = s.cell(&ws(), cell);
        assert_eq!(got, CellSurface { snow: snow * dt, ash: ash * dt }, "cell {cell}");
    }
    assert!(blended > 0, "the band crosses some cells");
}

#[test]
fn no_snow_climates_allocate_no_cells() {
    let g = CellGrid::new(ws(), &raster());
    let mut s = started(2);
    advance(&mut s, 24.0, &Const(vec![rain(1.0, 0.5), SurfaceRates { drying: 0.5, ..SurfaceRates::default() }]), &[&g], &tuning());
    assert_eq!(s.active_cells(), 0);
    assert!(s.cells.is_empty());
    // Snow only in the west climate: only cells it weighs are stored.
    advance(&mut s, 25.0, &Const(vec![SurfaceRates { snow: 0.5, ..SurfaceRates::default() }, SurfaceRates::default()]), &[&g], &tuning());
    let n = (0..g.len()).filter(|&ix| g.site(ix).iter().any(|e| e.climate == ClimateIx(0))).count();
    assert_eq!(s.active_cells(), n);
    assert!(n < g.len());
}

#[test]
fn the_snow_line_scales_snowfall_by_altitude() {
    let mut g = CellGrid::new(ws(), &raster());
    // Height rises 100 m per cell eastward; snow line 300 m in both climates.
    g.set_altitude(|_| Some(300.0), |c| Some(c.x as f32 * 100.0));
    let mut s = started(2);
    let snow = SurfaceRates { snow: 1.0, ..SurfaceRates::default() };
    advance(&mut s, STEP, &Const(vec![snow; 2]), &[&g], &tuning());
    let at = |x: i32| s.cell(&ws(), IVec2::new(x, 0)).snow;
    assert_eq!(at(2), 0.0, "below the line's blend: no cell");
    assert!((at(3) - 0.5 * STEP as f32).abs() < 1e-6, "half at the line");
    assert_eq!(at(4), STEP as f32);
    assert_eq!(smoothstep(250.0, 350.0, 300.0), 0.5);
}

fn book() -> WeatherBook {
    let id = RecordId::new;
    let presets = vec![
        (id("clear"), WeatherPreset { surface: SurfaceRates { drying: 0.5, ..SurfaceRates::default() }, ..WeatherPreset::default() }),
        (
            id("rain"),
            WeatherPreset {
                precipitation: vec![PrecipTerm { kind: id("rain"), intensity: 1.0 }],
                surface: SurfaceRates { wet: 1.0, puddle: 0.5, ..SurfaceRates::default() },
                ..WeatherPreset::default()
            },
        ),
        (
            id("snow"),
            WeatherPreset {
                precipitation: vec![PrecipTerm { kind: id("rain"), intensity: 1.0 }],
                surface: SurfaceRates { snow: 0.4, ..SurfaceRates::default() },
                ..WeatherPreset::default()
            },
        ),
    ];
    let climate = |w: &[(&str, f32)]| Climate {
        weights: w.iter().map(|(p, w)| (id(p), *w)).collect(),
        change_interval_hours: (2.0, 6.0),
        ..Climate::default()
    };
    let climates = vec![(id("wet"), climate(&[("clear", 1.0), ("rain", 2.0)])), (id("cold"), climate(&[("clear", 1.0), ("snow", 2.0)]))];
    let b = WeatherBook::new(presets, climates, vec![(id("rain"), PrecipKind::default())]);
    assert!(b.warnings.is_empty(), "{:?}", b.warnings);
    b
}

#[test]
fn the_weather_drives_it_and_equal_inputs_give_equal_states() {
    let b = book();
    let weather = WeatherState::new(42);
    let src = WeatherRates { book: &b, state: &weather };
    let g = CellGrid::new(ws(), &raster());
    let run = || {
        let mut s = SurfaceState::default();
        for h in 1..=200 {
            advance(&mut s, f64::from(h) * 0.37, &src, &[&g], &tuning());
        }
        s
    };
    let (a, c) = (run(), run());
    assert_eq!(a, c);
    // Some of it rained and snowed over 74 h.
    assert!(a.climates.iter().any(|c| c.wet > 0.0) || a.active_cells() > 0, "{a:?}");
    // The rates are the preset blend's: rain alone gives its rates.
    let cw = b.climate(&RecordId::new("wet")).unwrap();
    let rain_h = (0..2000).map(|i| f64::from(i) * 0.25).find(|&t| src.rates(cw, t).wet >= 1.0).expect("it rains in 500 h");
    assert_eq!(src.rates(cw, rain_h), SurfaceRates { wet: 1.0, puddle: 0.5, ..SurfaceRates::default() });
}

fn sample_state(b: &WeatherBook) -> SurfaceState {
    let mut s = SurfaceState { t_h: 10.0, climates: vec![ClimateSurface::default(); b.climate_count()], ..SurfaceState::default() };
    s.climates[0] = ClimateSurface { wet: 0.8, puddle: 0.3 };
    s.set_cell(&ws(), IVec2::new(2, 3), Channel::Snow, 0.4);
    s.set_cell(&ws(), IVec2::new(-1, 0), Channel::Ash, 0.123_456);
    s
}

#[test]
fn save_round_trip_is_within_one_quantum_and_commutes_with_advance() {
    let b = book();
    let weather = WeatherState::new(9);
    let src = WeatherRates { book: &b, state: &weather };
    let g = CellGrid::new(ws(), &raster());
    let s = sample_state(&b);
    let save = s.to_save(&b);
    let bytes = postcard::to_stdvec(&save).unwrap();
    let back: SurfaceSave = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, save);
    let loaded = SurfaceState::from_save(&back, &b);
    assert_eq!(loaded.to_save(&b), save, "a loaded state saves to the same bytes");
    let close = |a: &SurfaceState, c: &SurfaceState| {
        assert_eq!(a.t_h, c.t_h);
        for (x, y) in a.climates.iter().zip(&c.climates) {
            assert!((x.wet - y.wet).abs() <= QUANTUM && (x.puddle - y.puddle).abs() <= QUANTUM, "{x:?} {y:?}");
        }
        assert_eq!(a.active_cells(), c.active_cells());
        for (w, m) in &a.cells {
            for (cell, x) in m {
                let y = c.cell(w, *cell);
                assert!((x.snow - y.snow).abs() <= QUANTUM && (x.ash - y.ash).abs() <= QUANTUM, "{cell}: {x:?} {y:?}");
            }
        }
    };
    close(&s, &loaded);
    // save → load → advance vs advance → save → load.
    let mut a = loaded;
    advance(&mut a, 13.0, &src, &[&g], &tuning());
    let mut c = s.clone();
    advance(&mut c, 13.0, &src, &[&g], &tuning());
    let c = SurfaceState::from_save(&c.to_save(&b), &b);
    close(&a, &c);
}

#[test]
fn unknown_climates_drop_on_load_and_zero_entries_are_not_saved() {
    let b = book();
    let mut save = sample_state(&b).to_save(&b);
    assert_eq!(save.climates.len(), 1, "the zero climate is not saved");
    save.climates.push((RecordId::new("removed by a mod"), 9, 9));
    let s = SurfaceState::from_save(&save, &b);
    assert_eq!(s.climates.len(), b.climate_count());
    assert_eq!(s.to_save(&b), sample_state(&b).to_save(&b));
    // An unstarted state saves and loads unstarted.
    let fresh = SurfaceState::default().to_save(&b);
    assert!(!SurfaceState::from_save(&fresh, &b).started());
}

#[test]
fn sample_blends_climates_and_reads_the_cell() {
    let b = book();
    let r = raster();
    let mut s = sample_state(&b);
    s.climates[1] = ClimateSurface { wet: 0.2, puddle: 0.0 };
    // Deep in the west square: climate 0 alone; at the border: half and half.
    let p = DVec2::new(2.5 * CELL_M, 3.5 * CELL_M);
    let got = sample(&s, &r, &ws(), p);
    assert_eq!(got, SurfaceSample { wet: 0.8, puddle: 0.3, snow: 0.4, ash: 0.0 });
    let border = sample(&s, &r, &ws(), DVec2::new(256.0, 10.0));
    assert!((border.wet - 0.5).abs() < 1e-5, "{border:?}");
}

#[test]
fn console_setters_clamp_and_clear_keeps_the_time() {
    let mut s = started(1);
    s.set_climate(ClimateIx(2), Channel::Wet, 3.0);
    assert_eq!(s.climate(ClimateIx(2)).wet, 1.0);
    s.set_cell(&ws(), IVec2::ZERO, Channel::Snow, 0.5);
    s.set_cell(&ws(), IVec2::ZERO, Channel::Snow, 0.0);
    assert!(s.cells.is_empty(), "a zeroed cell drops");
    s.set_cell(&ws(), IVec2::ZERO, Channel::Ash, 0.5);
    s.t_h = 5.0;
    s.clear();
    assert_eq!((s.t_h, s.active_cells(), s.climate(ClimateIx(2)).wet), (5.0, 0, 0.0));
    assert_eq!(Channel::parse("PUDDLE"), Some(Channel::Puddle));
}
