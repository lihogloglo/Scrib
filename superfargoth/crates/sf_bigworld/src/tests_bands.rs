//! Tier band tests (spec big-world-rendering.md "Tiers"): tunables golden, view scale, switch distances, fades.

use crate::tunables::{AGGREGATE_SWITCH_PX, FADE_BAND, MESH_SWITCH_PX, TILE_M, TUNABLES};
use crate::{BandSettings, Bands, SpeciesDims, TierFades, ViewScale};

// tunables: const (test fixture: the High preset's internal render height)
const HIGH_H: f32 = 960.0;
// tunables: const (test fixture: the High preset's vertical field of view, degrees)
const HIGH_FOV_DEG: f32 = 60.0;
// tunables: const (test fixture: float tolerance for values computed through a band division)
const EPS: f32 = 1e-5;

fn high() -> ViewScale {
    ViewScale::new(HIGH_H, HIGH_FOV_DEG.to_radians())
}

fn tree() -> SpeciesDims {
    SpeciesDims { height_m: 12.0, crown_m: 8.0 }
}

fn bands() -> Bands {
    Bands::new(tree(), high(), BandSettings::default())
}

fn near(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

fn assert_fades(f: TierFades, mesh: f32, impostor: f32, at: &str) {
    assert!(near(f.mesh, mesh, EPS) && near(f.impostor, impostor, EPS), "{at}: {f:?}, want {mesh} / {impostor}");
}

/// `count + 1` evenly spaced distances from `from` to `to`.
#[allow(clippy::cast_precision_loss, reason = "sweep indices stay far below 2^24")]
fn sweep(from: f32, to: f32, count: u32) -> impl Iterator<Item = f32> {
    (0..=count).map(move |k| from + (to - from) * (k as f32 / count as f32))
}

#[test]
#[allow(clippy::float_cmp, reason = "the defaults and bounds are the exact literals of the table")]
fn tunables_golden() {
    let table = [
        (&TILE_M, "bigworld.tile_m", 256.0, (64.0, 1024.0)),
        (&MESH_SWITCH_PX, "bigworld.mesh.switch_px", 80.0, (20.0, 400.0)),
        (&AGGREGATE_SWITCH_PX, "bigworld.aggregate.switch_px", 3.0, (1.0, 16.0)),
        (&FADE_BAND, "bigworld.fade_band", 0.1, (0.0, 0.5)),
    ];
    for (def, name, default, bounds) in table {
        assert_eq!(def.name(), name);
        assert_eq!(def.default_value(), default, "{name}");
        assert_eq!(def.bounds(), Some(bounds), "{name}");
    }
    // The fifth def (`bigworld.ring.hysteresis_tiles`) is checked in `tests_ring.rs`.
    assert_eq!(TUNABLES.len(), 5);
    let mut names: Vec<_> = TUNABLES.iter().map(|d| d.name()).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 5, "names unique");
    for (def, ..) in table {
        assert!(names.contains(&def.name()), "{} listed in TUNABLES", def.name());
    }
    let s = BandSettings::default();
    assert_eq!(s.mesh_switch_px, 80.0);
    assert_eq!(s.aggregate_switch_px, 3.0);
    assert_eq!(s.fade_band, 0.1);
}

#[test]
fn view_scale_high() {
    let v = high();
    assert!(near(v.px_per_rad, 831.38, 0.01), "{}", v.px_per_rad);
    for size in [0.5_f32, 12.0, 30.0] {
        for d in [1.0_f32, 100.0, 2500.0] {
            let back = v.distance(size, v.px(size, d));
            assert!(near(back, d, d * 1e-6), "size {size} d {d}: {back}");
        }
    }
}

#[test]
fn switch_distances_high() {
    let v = high();
    let b = bands();
    assert!(near(b.mesh_end_m, 12.0 * v.px_per_rad / 80.0, b.mesh_end_m * 1e-6), "{b:?}");
    assert!(near(b.impostor_end_m, 8.0 * v.px_per_rad / 3.0, b.impostor_end_m * 1e-6), "{b:?}");
    assert!(near(b.mesh_end_m, 124.7, 0.1), "{b:?}");
    assert!(near(b.impostor_end_m, 2217.0, 1.0), "{b:?}");
    assert!(near(b.fade_band, 0.1, 0.0), "{b:?}");
    assert!(near(b.aggregate_lead_m(1.0), b.impostor_end_m * 0.9, 1e-3), "{b:?}");
}

#[test]
fn fades_at_named_distances() {
    let b = bands();
    let (m, i, w) = (b.mesh_end_m, b.impostor_end_m, b.fade_band);
    assert_fades(b.fades(0.0, 1.0), 1.0, 0.0, "viewer");
    assert_fades(b.fades(50.0, 1.0), 1.0, 0.0, "inside the mesh tier");
    assert_fades(b.fades(m * (1.0 - w), 1.0), 1.0, 0.0, "mesh band start");
    assert_fades(b.fades(m * (1.0 - 0.5 * w), 1.0), 0.5, 0.5, "mesh band middle");
    assert_fades(b.fades(m, 1.0), 0.0, 1.0, "mesh end");
    assert_fades(b.fades(1000.0, 1.0), 0.0, 1.0, "between bands");
    assert_fades(b.fades(i * (1.0 - w), 1.0), 0.0, 1.0, "impostor band start");
    assert_fades(b.fades(i * (1.0 - 0.5 * w), 1.0), 0.0, 0.5, "impostor band middle");
    assert_fades(b.fades(i, 1.0), 0.0, 0.0, "impostor end");
    assert_fades(b.fades(3000.0, 1.0), 0.0, 0.0, "past the impostor");
    assert_fades(b.fades(f32::INFINITY, 1.0), 0.0, 0.0, "infinitely far");
}

#[test]
fn first_band_sums_to_one() {
    let b = bands();
    let m = b.mesh_end_m;
    for d in sweep(m * (1.0 - b.fade_band), m, 1000) {
        let f = b.fades(d, 1.0);
        assert!(near(f.mesh + f.impostor, 1.0, 1e-6), "d {d}: {f:?}");
    }
}

#[test]
fn fades_monotonic_and_in_range() {
    let b = bands();
    let m = b.mesh_end_m;
    let (mut prev, mut prev_d) = (b.fades(0.0, 1.0), 0.0);
    for d in sweep(0.0, b.impostor_end_m * 1.2, 10_000) {
        let f = b.fades(d, 1.0);
        assert!((0.0..=1.0).contains(&f.mesh) && (0.0..=1.0).contains(&f.impostor), "d {d}: {f:?}");
        assert!(f.mesh <= prev.mesh, "mesh rises at d {d}: {prev:?} -> {f:?}");
        // The step that straddles the mesh end may go either way; each side is monotonic.
        if d <= m {
            assert!(f.impostor >= prev.impostor, "impostor falls before the mesh end at d {d}: {prev:?} -> {f:?}");
        } else if prev_d >= m {
            assert!(f.impostor <= prev.impostor, "impostor rises past the mesh end at d {d}: {prev:?} -> {f:?}");
        }
        (prev, prev_d) = (f, d);
    }
}

/// Power-of-two scales are exact in f32 (every product and quotient is the scale-1 one times a power of two), so
/// those compare bit for bit; scale 3 rounds each product and is held to 1e-5.
#[test]
#[allow(clippy::float_cmp, reason = "power-of-two scales are exact in f32")]
fn scale_moves_the_bands() {
    let b = bands();
    for d in sweep(0.0, b.impostor_end_m * 1.2, 2000) {
        let base = b.fades(d, 1.0);
        for s in [0.25_f32, 2.0, 4.0] {
            assert_eq!(b.fades(d * s, s), base, "d {d} s {s}");
        }
        let f = b.fades(d * 3.0, 3.0);
        assert!(near(f.mesh, base.mesh, EPS) && near(f.impostor, base.impostor, EPS), "d {d} s 3: {f:?} {base:?}");
    }
    assert!(near(b.aggregate_lead_m(2.0), 2.0 * b.aggregate_lead_m(1.0), 0.0));
}

#[test]
fn zero_band_is_a_hard_switch() {
    let b = Bands::new(tree(), high(), BandSettings { fade_band: 0.0, ..BandSettings::default() });
    let (m, i) = (b.mesh_end_m, b.impostor_end_m);
    assert_fades(b.fades(m * 0.999, 1.0), 1.0, 0.0, "just before the mesh end");
    assert_fades(b.fades(m, 1.0), 0.0, 1.0, "at the mesh end");
    assert_fades(b.fades(i * 0.999, 1.0), 0.0, 1.0, "just before the impostor end");
    assert_fades(b.fades(i, 1.0), 0.0, 0.0, "at the impostor end");
    assert_fades(b.fades(i * 2.0, 1.0), 0.0, 0.0, "past the impostor end");
    assert!(near(b.aggregate_lead_m(1.0), i, 0.0));
}

/// A small crown on a tall trunk: the crown rule ends before the mesh rule, the impostor end clamps to the mesh end,
/// and the bands touch. Before `aggregate_lead_m` the instance is full mesh; in the shared band the impostor rises
/// as the mesh fades until both are 0.5, then follows the mesh out (the aggregate is fully drawn there).
#[test]
fn touching_bands_never_leave_a_hole() {
    let dims = SpeciesDims { height_m: 12.0, crown_m: 0.2 };
    let b = Bands::new(dims, high(), BandSettings::default());
    assert!(near(b.impostor_end_m, b.mesh_end_m, 0.0), "{b:?}");
    let lead = b.aggregate_lead_m(1.0);
    assert!(near(lead, b.mesh_end_m * (1.0 - b.fade_band), 1e-4), "{lead}");
    for d in sweep(0.0, b.mesh_end_m * 1.5, 3000) {
        let f = b.fades(d, 1.0);
        let sum = f.mesh + f.impostor;
        assert!(sum <= 1.0 + 1e-6, "d {d}: {f:?}");
        if d < lead {
            assert!(near(sum, 1.0, 1e-6), "hole before the aggregate lead at d {d}: {f:?}");
        }
    }
    let mid = b.fades(b.mesh_end_m * (1.0 - 0.5 * b.fade_band), 1.0);
    assert_fades(mid, 0.5, 0.5, "shared band middle");
}

#[test]
fn degenerate_inputs_give_no_nan() {
    let odd = [f32::NAN, -1.0, 0.0, f32::INFINITY, f32::NEG_INFINITY, f32::MAX, 1e-30];
    let views = [high(), ViewScale::new(f32::NAN, 1.0), ViewScale::new(HIGH_H, 0.0), ViewScale::new(HIGH_H, -1.0)];
    let views = views.into_iter().chain([ViewScale::new(HIGH_H, f32::INFINITY), ViewScale { px_per_rad: f32::NAN }]);
    for view in views {
        for &size in &odd {
            for &px in &odd {
                assert!(!view.px(size, px).is_nan() && !view.distance(size, px).is_nan(), "{view:?} {size} {px}");
            }
        }
        for &height in &odd {
            for &px in &odd {
                for &band in &odd {
                    let dims = SpeciesDims { height_m: height, crown_m: 8.0 };
                    let set = BandSettings { mesh_switch_px: px, aggregate_switch_px: px, fade_band: band };
                    let b = Bands::new(dims, view, set);
                    assert!(!b.mesh_end_m.is_nan() && !b.impostor_end_m.is_nan() && !b.fade_band.is_nan(), "{b:?}");
                    for &d in &odd {
                        for &s in &odd {
                            let f = b.fades(d, s);
                            let ok = (0.0..=1.0).contains(&f.mesh) && (0.0..=1.0).contains(&f.impostor);
                            assert!(ok, "{b:?} d {d} s {s}: {f:?}");
                            assert!(!b.aggregate_lead_m(s).is_nan(), "{b:?} s {s}");
                        }
                    }
                }
            }
        }
    }
    // The documented choices on a normal species.
    let b = bands();
    assert_fades(b.fades(f32::NAN, 1.0), 1.0, 0.0, "NaN distance is at the viewer");
    assert_fades(b.fades(-5.0, 1.0), 1.0, 0.0, "negative distance is at the viewer");
    assert_fades(b.fades(10.0, f32::NAN), 0.0, 0.0, "NaN scale draws nothing");
    assert_fades(b.fades(10.0, 0.0), 0.0, 0.0, "zero scale draws nothing");
    let never = Bands::new(tree(), high(), BandSettings { mesh_switch_px: 0.0, ..BandSettings::default() });
    assert!(near(never.mesh_end_m, f32::MAX, 0.0), "0 px never switches: {never:?}");
}
