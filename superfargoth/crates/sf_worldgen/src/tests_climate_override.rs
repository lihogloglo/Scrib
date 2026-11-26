//! Tests of `climate_override`: 40 x 30 grids at 100 m, T = 10 + 0.1 * column, P = 800 + row; region 3 = columns
//! 20.., region 5 = rows 0..5 of columns 0..10, label 0 elsewhere. `blend_m` 400 m gives a box radius of 2 samples.

use super::*;

const W: usize = 40;
const H: usize = 30;
const SPACING: f64 = 100.0;

fn temperature() -> Grid {
    let data = (0..W * H).map(|i| 10.0 + 0.1 * f64::from(u32::try_from(i % W).unwrap())).collect();
    Grid { w: W, h: H, spacing: SPACING, data }
}

fn rain() -> Grid {
    let data = (0..W * H).map(|i| 800.0 + f64::from(u32::try_from(i / W).unwrap())).collect();
    Grid { w: W, h: H, spacing: SPACING, data }
}

fn labels() -> Vec<u8> {
    (0..W * H)
        .map(|i| {
            let (x, y) = (i % W, i / W);
            if x >= 20 {
                3
            } else if y < 5 && x < 10 {
                5
            } else {
                0
            }
        })
        .collect()
}

fn pin(v: f64) -> Adjust {
    Adjust { pin: Some(v), ..Adjust::default() }
}

fn temp_pin(region: u8, v: f64) -> RegionOverride {
    RegionOverride { region, temperature: pin(v), rain: Adjust::default() }
}

fn at(g: &Grid, x: usize, y: usize) -> f64 {
    g.data[y * g.w + x]
}

#[test]
fn pin_holds_inside_the_region() {
    let (t, p) = (temperature(), rain());
    let (t2, p2) = apply_overrides(&t, &p, &labels(), &[temp_pin(3, 25.0)], 400.0).unwrap();
    for y in 0..H {
        for x in 22..W {
            assert_eq!(at(&t2, x, y).to_bits(), 25.0f64.to_bits(), "({x}, {y})");
        }
    }
    assert_eq!(p2, p);
}

#[test]
fn scale_then_add() {
    let (t, p) = (temperature(), rain());
    let labels = labels();
    let o = RegionOverride {
        region: 3,
        temperature: Adjust::default(),
        rain: Adjust { scale: 0.5, add: 100.0, pin: None },
    };
    let (t2, p2) = apply_overrides(&t, &p, &labels, &[o], 0.0).unwrap();
    assert_eq!(t2, t);
    for (i, (&got, &input)) in p2.data.iter().zip(&p.data).enumerate() {
        let want = if labels[i] == 3 { input * 0.5 + 100.0 } else { input };
        assert_eq!(got.to_bits(), want.to_bits(), "sample {i}");
    }
}

#[test]
fn untouched_outside_the_blend() {
    let (t, p) = (temperature(), rain());
    let (t2, _) = apply_overrides(&t, &p, &labels(), &[temp_pin(3, 25.0)], 400.0).unwrap();
    for y in 0..H {
        for x in 0..=17 {
            assert_eq!(at(&t2, x, y).to_bits(), at(&t, x, y).to_bits(), "({x}, {y})");
        }
    }
}

#[test]
fn border_ramp_is_the_box_weight() {
    let (t, p) = (temperature(), rain());
    let (t2, _) = apply_overrides(&t, &p, &labels(), &[temp_pin(3, 25.0)], 400.0).unwrap();
    for y in [0, 15, 29] {
        for d in -3i32..=2 {
            let x = usize::try_from(20 + d).unwrap();
            let w = f64::from((3 + d).clamp(0, 5)) / 5.0;
            let want = (1.0 - w) * at(&t, x, y) + w * 25.0;
            let got = at(&t2, x, y);
            assert!((got - want).abs() < 1e-12, "({x}, {y}): {got} vs {want}");
        }
    }
}

#[test]
fn two_regions_blend_into_each_other() {
    let (t, p) = (temperature(), rain());
    let labels: Vec<u8> = (0..W * H).map(|i| if i % W >= 20 { 3 } else { 4 }).collect();
    let (t2, _) =
        apply_overrides(&t, &p, &labels, &[temp_pin(3, 30.0), temp_pin(4, 20.0)], 400.0).unwrap();
    for (i, &v) in t2.data.iter().enumerate() {
        assert!((20.0..=30.0).contains(&v), "sample {i}: {v}");
    }
    let want = 20.0 + 10.0 * 3.0 / 5.0;
    for y in 0..H {
        let got = at(&t2, 20, y);
        assert!((got - want).abs() < 1e-12, "row {y}: {got} vs {want}");
    }
}

#[test]
fn rain_never_goes_negative() {
    let (t, p) = (temperature(), rain());
    let o = RegionOverride {
        region: 5,
        temperature: Adjust::default(),
        rain: Adjust { add: -1e6, ..Adjust::default() },
    };
    let (_, p2) = apply_overrides(&t, &p, &labels(), &[o], 400.0).unwrap();
    for y in 0..=2 {
        for x in 0..=7 {
            assert_eq!(at(&p2, x, y).to_bits(), 0.0f64.to_bits(), "({x}, {y})");
        }
    }
    assert!(p2.data.iter().all(|&v| v >= 0.0));
}

#[test]
fn refuses_bad_input() {
    let (t, p) = (temperature(), rain());
    let labels = labels();
    let ok = [temp_pin(3, 25.0)];
    let refused = |t: &Grid, p: &Grid, labels: &[u8], o: &[RegionOverride], blend: f64, what: &str| {
        assert!(
            matches!(apply_overrides(t, p, labels, o, blend), Err(Error::Invalid(_))),
            "not refused: {what}"
        );
    };

    let narrow = Grid { w: W - 1, h: H, spacing: SPACING, data: vec![0.0; (W - 1) * H] };
    refused(&t, &narrow, &labels, &ok, 400.0, "different w");
    let short = Grid { w: W, h: H - 1, spacing: SPACING, data: vec![0.0; W * (H - 1)] };
    refused(&t, &short, &labels, &ok, 400.0, "different h");
    let coarse = Grid { spacing: 2.0 * SPACING, ..rain() };
    refused(&t, &coarse, &labels, &ok, 400.0, "different spacing");
    refused(&t, &p, &labels[1..], &ok, 400.0, "labels too short");
    refused(&t, &p, &labels, &[temp_pin(3, 25.0), temp_pin(3, 20.0)], 400.0, "region listed twice");
    refused(&t, &p, &labels, &ok, -1.0, "negative blend");
    refused(&t, &p, &labels, &ok, f64::NAN, "NaN blend");
    refused(&t, &p, &labels, &ok, f64::INFINITY, "infinite blend");
    let bad = |temperature: Adjust, rain: Adjust| [RegionOverride { region: 3, temperature, rain }];
    let nan_scale = Adjust { scale: f64::NAN, ..Adjust::default() };
    refused(&t, &p, &labels, &bad(nan_scale, Adjust::default()), 400.0, "NaN scale");
    let inf_add = Adjust { add: f64::INFINITY, ..Adjust::default() };
    refused(&t, &p, &labels, &bad(Adjust::default(), inf_add), 400.0, "infinite add");
    refused(&t, &p, &labels, &bad(pin(f64::NAN), Adjust::default()), 400.0, "NaN pin");

    // No override: the inputs come back unchanged.
    let (t2, p2) = apply_overrides(&t, &p, &labels, &[], 400.0).unwrap();
    assert_eq!((t2, p2), (t, p));
}

#[test]
fn same_result_on_any_thread_count() {
    let (t, p) = (temperature(), rain());
    let labels = labels();
    let run = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| apply_overrides(&t, &p, &labels, &[temp_pin(3, 25.0)], 400.0).unwrap())
    };
    let (t1, p1) = run(1);
    let (t8, p8) = run(8);
    let bits = |g: &Grid| g.data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(&t1), bits(&t8));
    assert_eq!(bits(&p1), bits(&p8));
}
