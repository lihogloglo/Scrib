//! LF-1 geology map tests: determinism, beds parallel to a cone under `Surface`, the bed count of a `Fixed` dip on a
//! plane, bed boundaries ordered and within their thickness range, binary16 encoding.

// Exact values are the point: hardness is one of two products, binary16 values are exact.
#![allow(clippy::float_cmp)]

use std::collections::BTreeMap;

use crate::geology::{
    BaseGrid, DipMode, GeologyParams, GeologyTable, RegionShares, RockUnit, UnitLayers, bed, build,
    declared_defaults, f16_bits, f16_value,
};
use crate::guide::RegionMap;

fn params() -> GeologyParams {
    GeologyParams::from_declared(&declared_defaults()).unwrap()
}

fn unit(name: &str, hardness: f64, bed_m: [f64; 2], dip_deg: f64, dip_mode: DipMode) -> RockUnit {
    RockUnit {
        name: name.into(),
        hardness,
        k: 1.0,
        layered: true,
        bed_m,
        soft_bed_share: 0.3,
        dip_deg,
        dip_mode,
        repose_hard_deg: 70.0,
        repose_soft_deg: 33.0,
        outcrop: false,
        layers: UnitLayers::default(),
    }
}

fn one_unit_table(u: RockUnit) -> GeologyTable {
    GeologyTable {
        shares: BTreeMap::from([(u.name.clone(), 1.0)]),
        units: vec![u],
        ..GeologyTable::default()
    }
}

/// Sum of |bed index steps| between 4-neighbours of a `w` x `h` field whose samples pass `keep`.
fn crossings(
    g: &crate::geology::GeologyMap,
    z: &[f64],
    keep: impl Fn(usize, usize) -> bool,
) -> i64 {
    let (w, h, dx, o) = (g.w, g.h, g.spacing_m, g.origin_m);
    let idx = |c: usize, r: usize| g.bed_at(o[0] + c as f64 * dx, o[1] - r as f64 * dx, z[r * w + c]).index;
    let mut n = 0;
    for r in 0..h {
        for c in 0..w {
            if !keep(c, r) {
                continue;
            }
            if c + 1 < w && keep(c + 1, r) {
                n += (idx(c + 1, r) - idx(c, r)).abs();
            }
            if r + 1 < h && keep(c, r + 1) {
                n += (idx(c, r + 1) - idx(c, r)).abs();
            }
        }
    }
    n
}

#[test]
fn geology_map_is_deterministic_and_seeded() {
    // 200 x 150 at 20 m, two region pixels of 2 km (west "a", east "b") with different unit shares.
    let (w, h, dx) = (200, 150, 20.0);
    let z: Vec<f64> = (0..w * h)
        .map(|i| 50.0 + 0.1 * (i % w) as f64 * dx + 30.0 * ((i / w) as f64 * 0.05).sin())
        .collect();
    let rm = RegionMap {
        w: 2,
        h: 2,
        cell_m: 2000.0,
        index: vec![1, 2, 1, 2],
        names: vec![String::new(), "a".into(), "b".into()],
    };
    let mut plain = unit("plain", 0.2, [1.0, 1.0], 0.0, DipMode::Surface);
    plain.layered = false;
    let table = GeologyTable {
        units: vec![
            unit("layered", 0.8, [3.0, 9.0], 0.0, DipMode::Surface),
            unit("tilted", 0.6, [5.0, 15.0], 12.0, DipMode::Fixed { azimuth_deg: 40.0 }),
            unit("cone", 0.9, [2.0, 6.0], 8.0, DipMode::Radial { centre_m: [2000.0, -1500.0] }),
            plain,
        ],
        shares: BTreeMap::from([("layered".into(), 1.0), ("plain".into(), 1.0)]),
        region: BTreeMap::from([(
            "b".into(),
            RegionShares {
                shares: BTreeMap::from([("tilted".into(), 1.0), ("cone".into(), 2.0)]),
            },
        )]),
        ..GeologyTable::default()
    };
    let mut p = params();
    p.unit_cell_m = 600.0;
    p.unit_blend_m = 120.0;
    let base = BaseGrid {
        heights: &z,
        w,
        h,
        spacing_m: dx,
        origin_m: [0.0, 0.0],
        regions: Some(&rm),
    };
    let a = build(&base, &table, &p).unwrap();
    let b = build(&base, &table, &p).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.hash(), b.hash());
    p.seed += 1;
    let c = build(&base, &table, &p).unwrap();
    assert_ne!(a.hash(), c.hash());
    // Every unit shows up; empty products until placement writes them; lip at +lip_band_m.
    assert!(a.unit_shares().iter().all(|&s| s > 0.0), "{:?}", a.unit_shares());
    assert!(a.cliff.iter().chain(&a.talus).chain(&a.human).all(|&v| v == 0));
    assert!(a.feature.iter().all(|&v| v == 0));
    assert!(a.lip.iter().all(|&v| f16_value(v) == 120.0));
    // Blended hardness stays inside the units' range.
    assert!(a.hardness.iter().all(|&v| (51..=230).contains(&v)));
}

#[test]
fn surface_dip_keeps_a_cone_inside_one_bed() {
    // A 30 deg cone over a 6 km grid at 20 m. Under `Surface` the datum is the low-passed cone itself, so the flank
    // (apex and the grid border excluded, where a local linear fit of the cone's tangential curvature is not exact)
    // lies in one bed: 5 m beds start at s = 0 and the residual s = z - L of a 200 m low-pass is under 0.3 m.
    let (w, h, dx) = (301, 301, 20.0);
    let t30 = 30f64.to_radians().tan();
    let r_of = |c: usize, r: usize| (c as f64 - 150.0).hypot(r as f64 - 150.0) * dx;
    let z: Vec<f64> = (0..w * h).map(|i| 2000.0 - t30 * r_of(i % w, i / w)).collect();
    let base = BaseGrid {
        heights: &z,
        w,
        h,
        spacing_m: dx,
        origin_m: [-3000.0, 3000.0],
        regions: None,
    };
    let keep = |c: usize, r: usize| r_of(c, r) > 300.0 && (10..w - 10).contains(&c) && (10..h - 10).contains(&r);
    let mut p = params();
    p.surface_lowpass_m = 200.0;
    let g = build(&base, &one_unit_table(unit("u", 0.8, [5.0, 5.0], 0.0, DipMode::Surface)), &p).unwrap();
    assert_eq!(crossings(&g, &z, keep), 0);
    // The same cone with beds dipping 10 deg east crosses many beds.
    let fixed = unit("u", 0.8, [5.0, 5.0], 10.0, DipMode::Fixed { azimuth_deg: 90.0 });
    let g = build(&base, &one_unit_table(fixed), &p).unwrap();
    assert!(crossings(&g, &z, keep) > 1000);
}

#[test]
fn fixed_dip_crosses_the_expected_beds_on_a_plane() {
    // Flat plane at 100 m, 2 km west-east inside one 20 km unit cell (no jitter, warp or blend): its centre is
    // (10 km, 10 km), so the datum is 100 - tan(10 deg)(x - 10 km) and s = tan(10 deg)(x - 10 km). With 5 m beds
    // (a = 0) bed i holds s in [5 i, 5 i + 5): a row from x = 9 km to 11 km steps floor(s/5) from -36 to 35.
    let (w, h, dx) = (101, 11, 20.0);
    let z = vec![100.0; w * h];
    let base = BaseGrid {
        heights: &z,
        w,
        h,
        spacing_m: dx,
        origin_m: [9000.0, 11_000.0],
        regions: None,
    };
    let mut p = params();
    p.unit_cell_m = 20_000.0;
    p.unit_jitter = 0.0;
    p.unit_warp_m = 0.0;
    p.unit_blend_m = 0.0;
    let u = unit("u", 0.8, [5.0, 5.0], 10.0, DipMode::Fixed { azimuth_deg: 90.0 });
    let g = build(&base, &one_unit_table(u), &p).unwrap();
    let tan = 10f64.to_radians().tan();
    let e = |x: f64| (tan * (x - 10_000.0) / 5.0).floor() as i64;
    let expected = e(11_000.0) - e(9000.0);
    assert_eq!(expected, 71);
    let row: Vec<i64> = (0..w).map(|c| g.bed_at(9000.0 + c as f64 * dx, 10_900.0, 100.0).index).collect();
    assert_eq!((row[0], row[w - 1]), (e(9000.0), e(11_000.0)));
    assert!(row.windows(2).all(|p| p[1] - p[0] == 0 || p[1] - p[0] == 1));
    // Every row and column of the grid: 71 per row, none north-south.
    assert_eq!(crossings(&g, &z, |_, _| true), expected * h as i64);
}

#[test]
fn bed_boundaries_are_ordered_and_within_the_thickness_range() {
    let u = unit("u", 0.8, [3.0, 12.0], 0.0, DipMode::Surface);
    let (ratio, step) = (0.5, 0.01);
    let mut last = bed(&u, 3, -500.0, 9, ratio);
    let mut start: Option<f64> = None;
    let (mut beds, mut soft) = (0, 0);
    for k in 1..=100_000 {
        let s = -500.0 + f64::from(k) * step;
        let b = bed(&u, 3, s, 9, ratio);
        assert!((0.0..=1.0).contains(&b.frac));
        assert!(b.hardness == 0.8 || b.hardness == 0.4);
        if b.index != last.index {
            assert_eq!(b.index, last.index + 1, "at s = {s}");
            if let Some(s0) = start {
                let t = s - s0;
                assert!((3.0 - 2.0 * step..=12.0 + 2.0 * step).contains(&t), "bed {} is {t} m", last.index);
                beds += 1;
                soft += usize::from(last.hardness < 0.5);
            }
            start = Some(s);
        }
        last = b;
    }
    assert!(beds > 100, "{beds} beds");
    assert!(soft > 0 && soft < beds);
    // An unlayered unit is one bed of its own hardness.
    let mut plain = u.clone();
    plain.layered = false;
    let b = bed(&plain, 0, 123.0, 9, ratio);
    assert_eq!((b.index, b.hardness), (0, 0.8));
}

#[test]
fn binary16_round_trips_and_rounds() {
    assert_eq!(f16_bits(120.0), 0x5780);
    for v in [120.0f32, 1.5, -0.0625, 65504.0, 0.0, -2.0, 6.103_515_6e-5] {
        assert_eq!(f16_value(f16_bits(v)), v, "{v}");
    }
    assert_eq!(f16_value(f16_bits(65520.0)), f32::INFINITY);
    assert_eq!(f16_bits(1e-7), 2);
    assert_eq!(f16_bits(1.000_4), 0x3c00);
    assert!(f16_value(f16_bits(f32::NAN)).is_nan());
}
