//! Footprint exclusion tests (exact) and the per-sample cost measurement.
#![allow(clippy::float_cmp, reason = "exact: an excluded point is exactly 0, an untouched one exactly the baseline")]
#![allow(clippy::many_single_char_names, reason = "geometry helpers: box, sine, cosine, corners, points")]

use std::time::Instant;

use mw_world::groundcover::GroundcoverData;
use sf_groundcover::{
    GroundSample, GroundSampler, Groundcover, OverrideSet, Overrides, ScatterSettings, Scatterer, SpeciesIndex, TileKey,
};

use super::*;

/// Flat dry ground at height 0, 10 m above the sea, every rule channel full.
struct Flat {
    channels: usize,
}

impl GroundSampler for Flat {
    fn sample(&self, _x: f64, _z: f64, out: &mut GroundSample) {
        out.height = 0.0;
        out.normal = bevy::math::Vec3::Y;
        out.water_distance = 10.0;
        out.valid = true;
        out.channels.clear();
        out.channels.resize(self.channels, 1.0);
    }
}

fn cover() -> (Groundcover, Flat) {
    let data = GroundcoverData::builtin().unwrap();
    (data.build().unwrap(), Flat { channels: data.channels.len() })
}

fn boxed(center: [f64; 2], half: [f64; 2], yaw: f64, bottom_y: f64) -> StaticBox {
    StaticBox { center, half, yaw, bottom_y, top_y: bottom_y + 2.0 }
}

/// Points on a 0.1 m lattice of `b`'s own frame, inside the box grown by `grow` (negative shrinks).
fn lattice(b: &StaticBox, grow: f64) -> Vec<[f64; 2]> {
    let (s, c) = b.yaw.sin_cos();
    let (ax, az) = ([c, -s], [s, c]);
    let (hx, hz) = (b.half[0] + grow, b.half[1] + grow);
    let n = |h: f64| {
        // A few hundred steps at most.
        #[allow(clippy::cast_possible_truncation)]
        let n = (h / 0.1).floor() as i32;
        n
    };
    let mut out = Vec::new();
    for i in -n(hx)..=n(hx) {
        for j in -n(hz)..=n(hz) {
            let (u, v) = (f64::from(i) * 0.1, f64::from(j) * 0.1);
            out.push([b.center[0] + u * ax[0] + v * az[0], b.center[1] + u * ax[1] + v * az[1]]);
        }
    }
    out
}

/// Points on the outline of `b` grown by `grow`, every 0.1 m.
fn ring(b: &StaticBox, grow: f64) -> Vec<[f64; 2]> {
    let g = StaticBox { half: [b.half[0] + grow, b.half[1] + grow], ..*b };
    let c = g.corners();
    let mut out = Vec::new();
    for k in 0..4 {
        let (p, q) = (c[k], c[(k + 1) % 4]);
        let len = ((q[0] - p[0]).powi(2) + (q[1] - p[1]).powi(2)).sqrt();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let steps = (len / 0.1).ceil() as u32;
        for t in 0..steps {
            let f = f64::from(t) / f64::from(steps);
            out.push([p[0] + (q[0] - p[0]) * f, p[1] + (q[1] - p[1]) * f]);
        }
    }
    out
}

#[test]
fn grounded_static_clears_its_footprint_and_a_floating_one_nothing() {
    let (cover, ground) = cover();
    let settings = FootprintSettings::default();
    let flat = |_: f64, _: f64| Some(0.0_f32);
    let grounded = boxed([10.0, 10.0], [2.0, 1.0], 0.0, -0.2);
    let turned = boxed([40.0, 10.0], [3.0, 1.5], 0.6, 0.0);
    // Bottom above the reach: a bridge deck, a hanging sign.
    let floating = boxed([70.0, 10.0], [2.0, 2.0], 0.0, f64::from(settings.reach_m) + 0.5);
    let (regions, report) = exclusions(&[grounded, turned, floating], &flat, settings);
    assert_eq!(report, FootprintReport { statics: 3, excluded: 2, floating: 1, ..FootprintReport::default() });

    let ov = Overrides::new(&cover, &OverrideSet { exclusions: regions, ..OverrideSet::default() }).unwrap();
    let none = Overrides::none();
    let with = Scatterer::new(&cover, &ov, ScatterSettings::default());
    let base = Scatterer::new(&cover, &none, ScatterSettings::default());
    let species: Vec<SpeciesIndex> =
        (0..cover.len()).map(|i| SpeciesIndex(u16::try_from(i).unwrap())).filter(|&s| base.density_at(&ground, 0.0, 0.0, s) > 0.0).collect();
    assert!(!species.is_empty(), "the builtin data plants something on full channels");

    // Inside the footprint grown by the margin: cleared; 1 cm past the margin: untouched.
    let margin = f64::from(settings.margin_m);
    let past = margin + 0.01;
    for &s in &species {
        for b in [&grounded, &turned] {
            for [x, z] in lattice(b, 0.0).into_iter().chain(ring(b, margin - 0.01)) {
                assert_eq!(with.density_at(&ground, x, z, s), 0.0, "inside the grown footprint at ({x}, {z})");
            }
            for [x, z] in ring(b, past) {
                assert_eq!(with.density_at(&ground, x, z, s), base.density_at(&ground, x, z, s), "past the margin at ({x}, {z})");
            }
        }
        for [x, z] in lattice(&floating, 0.0) {
            assert_eq!(with.density_at(&ground, x, z, s), base.density_at(&ground, x, z, s), "under a floating static");
        }
    }

    // The mesh scatter agrees: tile (0, 0) (16 m) holds the grounded box; no instance inside it.
    let size = f64::from(ScatterSettings::default().tile_size_m);
    let mut out = Vec::new();
    for &s in &species {
        with.scatter_tile(&ground, TileKey::new(0, 0), s, 0, &mut out);
        for i in &out {
            let (x, z) = (f64::from(i.local[0]), f64::from(i.local[2]));
            assert!(x > size || z > size || !grounded.contains(x, z, 0.0), "instance at ({x}, {z}) inside the static");
        }
    }
}

#[test]
fn no_terrain_and_buried_statics_exclude_nothing() {
    let settings = FootprintSettings::default();
    let b = boxed([0.0, 0.0], [1.0, 1.0], 0.0, 0.0);
    let (regions, report) = exclusions(&[b], &|_, _| None, settings);
    assert!(regions.is_empty());
    assert_eq!(report.no_ground, 1);
    // Top below the ground: buried.
    let (regions, report) = exclusions(&[b], &|_, _| Some(5.0), settings);
    assert!(regions.is_empty());
    assert_eq!(report.floating, 1);
}

/// A deterministic static field: `n` boxes of 1-10 m, any yaw, over a `side` m square at the origin.
fn field(n: usize, side: f64) -> Vec<StaticBox> {
    let mut seed = 0x9E37_79B9_7F4A_7C15_u64;
    let mut next = move || {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let v = (seed >> 11) as f64 / (1u64 << 53) as f64;
        v
    };
    (0..n)
        .map(|_| {
            let center = [next() * side, next() * side];
            let half = [0.5 + next() * 4.5, 0.5 + next() * 4.5];
            boxed(center, half, next() * std::f64::consts::TAU, 0.0)
        })
        .collect()
}

#[test]
#[ignore = "measurement: per-sample density cost with many static footprints (picks the representation)"]
fn per_sample_cost() {
    let (cover, ground) = cover();
    let none = Overrides::none();
    let base = Scatterer::new(&cover, &none, ScatterSettings::default());
    let s = (0..cover.len()).map(|i| SpeciesIndex(u16::try_from(i).unwrap())).find(|&s| base.density_at(&ground, 0.0, 0.0, s) > 0.0).unwrap();
    // A blade block's worth of samples (32 m at 0.5 m) in the middle of a 5 km world.
    let side = 5000.0;
    let points: Vec<[f64; 2]> =
        (0..64).flat_map(|j| (0..64).map(move |i| [2500.0 + f64::from(i) * 0.5, 2500.0 + f64::from(j) * 0.5])).collect();
    for n in [0, 1_000, 10_000, 40_000] {
        let boxes = field(n, side);
        let (regions, report) = exclusions(&boxes, &|_, _| Some(0.0), FootprintSettings::default());
        let built = Instant::now();
        let ov = Overrides::new(&cover, &OverrideSet { exclusions: regions, ..OverrideSet::default() }).unwrap();
        let build_ms = built.elapsed().as_secs_f64() * 1e3;
        let sc = Scatterer::new(&cover, &ov, ScatterSettings::default());
        let mut sum = 0.0;
        let t = Instant::now();
        let reps = 4;
        for _ in 0..reps {
            for &[x, z] in &points {
                sum += sc.density_at(&ground, x, z, s);
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let ns = t.elapsed().as_secs_f64() * 1e9 / (points.len() * reps) as f64;
        println!(
            "statics {n:>6}: {ns:>9.1} ns/sample, Overrides::new {build_ms:.1} ms, excluded {}, checksum {sum}",
            report.excluded
        );
    }
}
