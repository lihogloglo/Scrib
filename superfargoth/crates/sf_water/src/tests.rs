#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::many_single_char_names
)]

use std::sync::Arc;

use glam::{DVec2, IVec3, Vec2, Vec3};
use sf_coords::WorldPos;
use sf_world::{Worldspace, WorldspaceId};

use crate::twin::reference_height;
use crate::*;

fn sea(wind: f32) -> SpectrumParams {
    SpectrumParams {
        wind: WindInput {
            speed: wind,
            toward: Vec2::new(0.8, 0.6),
        },
        ..SpectrumParams::default()
    }
}

/// Deterministic pseudo-random points in a tile.
fn points(count: usize, size: f32) -> Vec<Vec2> {
    (0..count as u32)
        .map(|i| {
            let a = spectrum::hash(i * 2 + 1) as f32 / u32::MAX as f32;
            let b = spectrum::hash(i * 2 + 2) as f32 / u32::MAX as f32;
            Vec2::new(a, b) * size
        })
        .collect()
}

#[test]
fn spectrum_energy_matches_the_analytic_jonswap_integral() {
    let p = sea(10.0);
    let cascades = Cascades::new(&[1024, 64]).unwrap();
    let (lo, hi) = cascades.band(0);
    assert_eq!(lo, 0.0);
    let h0 = H0::generate(&p, &cascades, 0, 256, 7);
    let analytic = p.band_variance(1e-4, hi);
    let rel = (h0.variance - analytic).abs() / analytic;
    assert!(
        rel < 0.03,
        "discrete {} vs analytic {analytic} ({:.1} %)",
        h0.variance,
        rel * 100.0
    );
    // The realization's variance follows the expectation (many bins, chi-square).
    let realized = h0.realized_variance();
    assert!(
        (realized / h0.variance - 1.0).abs() < 0.15,
        "realized {realized} vs expected {}",
        h0.variance
    );
    // Fully developed-ish sea: significant height near the JONSWAP fetch law 1.6e-3·U·sqrt(F/g).
    let hs = 4.0 * p.band_variance(1e-4, 50.0).sqrt();
    let law = 1.6e-3 * 10.0 * (100_000.0f64 / f64::from(GRAVITY)).sqrt();
    // The JONSWAP peak and alpha fits and the energy fit are separate regressions: ~30 % apart.
    assert!((hs / law - 1.0).abs() < 0.4, "Hs {hs} vs fetch law {law}");
    // Spreading integrates to one over the circle.
    let n = 3600;
    let step = std::f64::consts::TAU / f64::from(n);
    let sum: f64 = (0..n)
        .map(|i| {
            p.spreading(
                p.omega_peak() * 1.3,
                -std::f64::consts::PI + step * f64::from(i),
            ) * step
        })
        .sum();
    assert!((sum - 1.0).abs() < 1e-3, "spreading integral {sum}");
}

#[test]
fn cascades_are_power_of_two_and_band_limited() {
    let c = Cascades::default();
    assert_eq!(c.len(), 4);
    assert!(Cascades::new(&[256, 100]).is_none());
    assert!(Cascades::new(&[64, 256]).is_none());
    // Bands tile the wave numbers with no overlap.
    for i in 0..c.len() - 1 {
        assert_eq!(c.band(i).1, c.band(i + 1).0);
    }
    // The twin's 64² grid holds cascade 0's whole band.
    assert!(
        c.min_resolution(0) <= TWIN_RESOLUTION,
        "{}",
        c.min_resolution(0)
    );
    // Every tile divides the 256 m grid cell, so cells carry no phase.
    for i in 0..c.len() {
        assert_eq!(origin_phase(IVec3::new(7, 0, -13), c.size(i)), Vec2::ZERO);
    }
}

#[test]
fn spectrum_and_twin_are_deterministic() {
    let p = sea(12.0);
    let c = Cascades::default();
    let a = H0::generate(&p, &c, 0, 64, 42);
    assert_eq!(a, H0::generate(&p, &c, 0, 64, 42));
    assert_ne!(a.h0, H0::generate(&p, &c, 0, 64, 43).h0, "the seed matters");
    // A twin's state is a function of time only (no history).
    let mut t1 = WaveTwin::new(&p, &c, 42, 1, 64);
    let mut t2 = WaveTwin::new(&p, &c, 42, 1, 64);
    t1.evaluate(3.0);
    t1.evaluate(1234.5);
    t2.evaluate(1234.5);
    assert_eq!(t1.cascades[0].grids().height, t2.cascades[0].grids().height);
    assert_eq!(t1.cascades[0].grids().dx, t2.cascades[0].grids().dx);
}

#[test]
fn small_and_large_grids_share_every_coefficient() {
    // The CPU twin (64²) and the GPU grid (256²) draw the same h0 for the same wave vector.
    let p = sea(9.0);
    let c = Cascades::default();
    let small = H0::generate(&p, &c, 0, 64, 5);
    let large = H0::generate(&p, &c, 0, 256, 5);
    let mut nonzero = 0;
    for iz in 0..64 {
        for ix in 0..64 {
            let (sx, sz) = (spectrum::signed(ix, 64), spectrum::signed(iz, 64));
            let (lx, lz) = (sx.rem_euclid(256) as usize, sz.rem_euclid(256) as usize);
            assert_eq!(
                small.h0[iz * 64 + ix],
                large.h0[lz * 256 + lx],
                "bin {sx},{sz}"
            );
            nonzero += usize::from(small.h0[iz * 64 + ix].norm_sqr() > 0.0);
        }
    }
    assert!(nonzero > 100);
    // The band fits the small grid: the large grid has no energy the small one lacks.
    assert!((small.variance - large.variance).abs() < 1e-9 * large.variance.max(1e-12));
}

#[test]
fn twin_matches_the_reference_dft() {
    let p = sea(10.0);
    let c = Cascades::default();
    let h0 = H0::generate(&p, &c, 0, 64, 11);
    let mut twin = CascadeTwin::new(&h0, &p);
    let t = 17.25;
    twin.evaluate(t);
    // On grid nodes: the FFT equals the direct sum (float error only).
    let mut worst_node = 0.0f32;
    for (ix, iz) in [(0, 0), (5, 9), (31, 2), (63, 63), (17, 40)] {
        let x0 = Vec2::new(ix as f32, iz as f32) * 4.0;
        let (h, d) = reference_height(&h0, &p, t, x0);
        let g = twin.grids();
        worst_node = worst_node.max((g.height[ix * 64 + iz] - h).abs());
        worst_node = worst_node
            .max((g.dx[ix * 64 + iz] - d.x).abs())
            .max((g.dz[ix * 64 + iz] - d.y).abs());
    }
    assert!(worst_node < 1e-3, "node error {worst_node} m");
    // Between nodes: bilinear 64² vs the 256² grid (what the GPU samples) and vs the exact field.
    let h0_large = H0::generate(&p, &c, 0, 256, 11);
    let mut large = CascadeTwin::new(&h0_large, &p);
    large.evaluate(t);
    let (mut vs_gpu, mut vs_exact) = (0.0f32, 0.0f32);
    for x in points(200, 256.0) {
        let a = twin.lagrangian(x).0.y;
        let b = large.lagrangian(x).0.y;
        let (e, _) = reference_height(&h0, &p, t, x);
        vs_gpu = vs_gpu.max((a - b).abs());
        vs_exact = vs_exact.max((a - e).abs());
    }
    eprintln!(
        "Hs {:.2} m: node {worst_node:.2e} m, 64² vs 256² {vs_gpu:.4} m, vs exact {vs_exact:.4} m",
        4.0 * h0.variance.sqrt()
    );
    assert!(
        vs_gpu < 0.01 && vs_exact < 0.01,
        "64² vs 256² {vs_gpu} m, vs exact {vs_exact} m"
    );
}

#[test]
fn displaced_height_inversion_converges_in_three_iterations() {
    // A steep, choppy sea.
    let p = SpectrumParams {
        choppiness: 1.0,
        ..sea(18.0)
    };
    let mut twin = WaveTwin::new(&p, &Cascades::default(), 3, 2, 64);
    twin.evaluate(40.0);
    let mut worst = 0.0f32;
    let mut worst_fit = 0.0f32;
    for x in points(300, 256.0) {
        let at = WorldPos::new(
            IVec3::new(3, 0, -2),
            Vec3::new(x.x - 128.0, 0.0, x.y - 128.0),
        );
        let fast = twin.sample(at);
        let exact = twin.sample_iterations(at, 40);
        worst = worst.max((fast.height - exact.height).abs());
        // The converged undisplaced point maps back onto the query point.
        let base = tile_coords(at, 256);
        let mut off = Vec2::ZERO;
        for _ in 0..40 {
            off = -(twin.cascades[0].displacement(base + off)
                + twin.cascades[1].displacement(tile_coords(at, 64) + off));
        }
        let back = off
            + twin.cascades[0].displacement(base + off)
            + twin.cascades[1].displacement(tile_coords(at, 64) + off);
        worst_fit = worst_fit.max(back.length());
    }
    eprintln!("3-iteration height error {worst:.4} m, converged fit {worst_fit:.2e} m");
    assert!(worst < 0.01, "3 iterations off by {worst} m");
    assert!(worst_fit < 1e-3);
}

#[test]
fn origin_shifts_and_far_positions_move_no_wave() {
    let p = sea(11.0);
    let mut twin = WaveTwin::new(&p, &Cascades::default(), 9, 2, 64);
    twin.evaluate(5.0);
    let local = Vec3::new(37.123_45, 2.0, -101.987_6);
    let near = twin.sample(WorldPos::new(IVec3::new(2, 0, 3), local));
    // One cell (256 m, a multiple of every tile) further, or a hundred thousand cells away: the same
    // bits, so streaming never moves a wave.
    for cell in [
        IVec3::new(3, 0, 3),
        IVec3::new(2, 5, 4),
        IVec3::new(100_002, 0, -99_997),
    ] {
        assert_eq!(twin.sample(WorldPos::new(cell, local)), near, "{cell}");
    }
    // The GPU formula (origin phase + camera-relative XZ) lands on the CPU's tile coordinates for
    // any render origin (to f32 rounding of the relative position).
    let at = WorldPos::new(IVec3::new(2, 0, 3), local);
    for origin in [
        IVec3::ZERO,
        IVec3::new(1, 0, 1),
        IVec3::new(-3, 0, 6),
        IVec3::new(9, 0, -4),
    ] {
        for size in [256u32, 64, 16, 4] {
            let rel = WorldPos::new(origin, Vec3::ZERO).delta(at).as_vec3();
            let gpu = (origin_phase(origin, size) + Vec2::new(rel.x, rel.z))
                .rem_euclid(Vec2::splat(size as f32));
            let cpu = tile_coords(at, size);
            let d = (gpu - cpu).abs();
            let d = d.min(Vec2::splat(size as f32) - d);
            assert!(
                d.max_element() < 1e-3,
                "origin {origin} size {size}: {gpu} vs {cpu}"
            );
        }
    }
    // Tiles longer than a cell (1024 m swell) carry an exact integer phase.
    assert_eq!(
        origin_phase(IVec3::new(5, 0, -1), 1024),
        Vec2::new(256.0, 768.0)
    );
}

#[test]
fn weather_blend_is_continuous() {
    let c = Cascades::default();
    let calm = H0::generate(&sea(3.0), &c, 0, 64, 1);
    let storm = H0::generate(&sea(20.0), &c, 0, 64, 1);
    assert_eq!(H0::blend(&calm, &storm, 0.0).h0, calm.h0);
    assert_eq!(H0::blend(&calm, &storm, 1.0).h0, storm.h0);
    let p = sea(10.0);
    let heights = |w: f32| {
        let mut t = WaveTwin::from_h0(&[H0::blend(&calm, &storm, w)], &p);
        t.evaluate(8.0);
        points(50, 256.0)
            .iter()
            .map(|x| t.cascades[0].lagrangian(*x).0.y)
            .collect::<Vec<_>>()
    };
    let (a, b) = (heights(0.50), heights(0.52));
    let jump = a
        .iter()
        .zip(&b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max);
    let range = a.iter().fold(0.0f32, |m, x| m.max(x.abs()));
    assert!(
        jump < 0.05 * range.max(0.1),
        "2 % blend step moved the surface {jump} m (range {range})"
    );
}

struct Island;

impl BedHeights for Island {
    fn bed_height(&self, at: WorldPos) -> Option<f32> {
        let x = at.to_f64().x;
        // Sea floor at −10 m, land rising past x = 1 000 m.
        #[allow(clippy::cast_possible_truncation)]
        Some(if x > 1000.0 {
            ((x - 1000.0) * 0.1) as f32
        } else {
            -10.0
        })
    }
}

fn square(
    id: &str,
    center: DVec2,
    half: f64,
    level: f64,
    priority: i32,
    kind: WaterKind,
) -> WaterBody {
    WaterBody {
        id: id.into(),
        worldspace: WorldspaceId::new("w"),
        kind,
        shape: WaterShape::Polygon {
            outline: vec![
                center + DVec2::new(-half, -half),
                center + DVec2::new(half, -half),
                center + DVec2::new(half, half),
                center + DVec2::new(-half, half),
            ],
            holes: Vec::new(),
        },
        level,
        priority,
        optics: None,
        waves: WaveTerms::for_kind(kind),
        flow: None,
    }
}

#[test]
fn bodies_resolve_by_priority_through_the_bvh() {
    let ws = WorldspaceId::new("w");
    let mut w = Worldspace::new("w");
    w.has_terrain = true;
    w.sea_level = Some(0.0);
    let mut bodies = vec![WaterBody::from_worldspace(&ws, &w).unwrap()];
    assert_eq!(bodies[0].kind, WaterKind::Ocean);
    // A lake above the sea on the island, a pool inside it, and 60 scattered ponds.
    bodies.push(square(
        "lake",
        DVec2::new(1500.0, 0.0),
        200.0,
        80.0,
        10,
        WaterKind::Lake,
    ));
    bodies.push(square(
        "pool",
        DVec2::new(1500.0, 50.0),
        10.0,
        81.0,
        20,
        WaterKind::Pool,
    ));
    for i in 0..60 {
        bodies.push(square(
            &format!("pond{i}"),
            DVec2::new(3000.0 + 50.0 * f64::from(i), 500.0),
            5.0,
            120.0,
            5,
            WaterKind::Pool,
        ));
    }
    bodies.push(WaterBody {
        shape: WaterShape::Ribbon {
            spline: vec![
                RiverPoint {
                    center: DVec2::new(2000.0, -300.0),
                    width: 20.0,
                    height: 60.0,
                    flow_speed: 1.0,
                },
                RiverPoint {
                    center: DVec2::new(2000.0, 300.0),
                    width: 20.0,
                    height: 40.0,
                    flow_speed: 2.0,
                },
            ],
        },
        ..square("river", DVec2::ZERO, 1.0, 50.0, 10, WaterKind::River)
    });
    let mut water = WorldspaceWater::new(bodies);
    water.set_bed(Some(Arc::new(Island)));
    let mut world = WaterWorld::new();
    world.insert(ws.clone(), water);
    let at = |x: f64, z: f64| WorldPos::from_f64(glam::DVec3::new(x, 0.0, z));
    let body = |x: f64, z: f64| {
        world
            .sample(&ws, at(x, z))
            .map(|s| s.body.as_str().to_owned())
    };

    assert_eq!(body(0.0, 0.0).as_deref(), Some("w#water"));
    let s = world.sample(&ws, at(0.0, 0.0)).unwrap();
    assert!((s.depth - 10.0).abs() < 1e-4 && s.height == 0.0 && s.normal == Vec3::Y);
    assert_eq!(body(1200.0, 0.0), None, "dry land cuts the ocean");
    assert_eq!(body(1500.0, 0.0).as_deref(), Some("lake"));
    assert_eq!(
        body(1500.0, 50.0).as_deref(),
        Some("pool"),
        "higher priority wins"
    );
    assert_eq!(
        body(3000.0 + 50.0 * 37.0 + 2.0, 498.0).as_deref(),
        Some("pond37")
    );
    assert_eq!(body(3025.0, 500.0), None, "between ponds is land");
    let r = world.sample(&ws, at(2005.0, 150.0)).unwrap();
    assert_eq!(r.body.as_str(), "river");
    assert!((r.height - 45.0).abs() < 1e-9, "{}", r.height);
    assert!(
        (r.velocity - Vec3::new(0.0, 0.0, 1.75)).length() < 1e-5,
        "{}",
        r.velocity
    );

    // Immersion: a 2 m capsule standing on the sea floor at 1 m depth is half in.
    let mut out = vec![None; 2];
    world.sample_many(&ws, &[at(0.0, 0.0), at(1200.0, 0.0)], &mut out);
    assert!(out[0].is_some() && out[1].is_none());
    let cap = Capsule {
        a: WorldPos::from_f64(glam::DVec3::new(0.0, -0.7, 0.0)),
        b: WorldPos::from_f64(glam::DVec3::new(0.0, 0.7, 0.0)),
        radius: 0.3,
    };
    let im = world.immersion(&ws, cap);
    assert!(
        (im.depth - 1.0).abs() < 1e-5 && (im.fraction - 0.5).abs() < 1e-5,
        "{im:?}"
    );
}

#[test]
fn ocean_samples_carry_the_waves() {
    let ws = WorldspaceId::new("sea");
    let mut w = Worldspace::new("sea");
    w.has_terrain = true;
    w.sea_level = Some(-2.0);
    let mut water = WorldspaceWater::new(vec![WaterBody::from_worldspace(&ws, &w).unwrap()]);
    let p = sea(14.0);
    water.set_waves(Some(WaveTwin::new(&p, &Cascades::default(), 1, 1, 64)));
    let mut world = WaterWorld::new();
    world.insert(ws.clone(), water);
    world.step(30.0);
    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
    let mut moving = 0;
    for x in points(100, 256.0) {
        let s = world
            .sample(
                &ws,
                WorldPos::new(IVec3::ZERO, Vec3::new(x.x - 128.0, 0.0, x.y - 128.0)),
            )
            .unwrap();
        lo = lo.min(s.height);
        hi = hi.max(s.height);
        moving += usize::from(s.velocity.length() > 0.01);
        assert!(s.normal.y > 0.5);
    }
    assert!(lo < -2.2 && hi > -1.8, "waves around the level: {lo}..{hi}");
    assert!(moving > 80);
    // Interiors: a flat pool everywhere.
    let mut room = Worldspace::new("room");
    room.sea_level = Some(1.5);
    let pool = WaterBody::from_worldspace(&WorldspaceId::new("room"), &room).unwrap();
    assert_eq!(
        (pool.kind, &pool.shape),
        (WaterKind::Pool, &WaterShape::Bounds)
    );
    assert!(WaterBody::from_worldspace(&ws, &Worldspace::new("dry")).is_none());
}

#[test]
fn records_round_trip_and_optics_are_physical() {
    let o = WaterOptics::default();
    let v = o.visibility();
    assert!((40.0..300.0).contains(&v), "clear water visibility {v} m");
    let body = square("lake", DVec2::ZERO, 10.0, 3.0, 1, WaterKind::Lake);
    let json = serde_json::to_string(&body).unwrap();
    assert_eq!(serde_json::from_str::<WaterBody>(&json).unwrap(), body);
}

#[test]
fn waves_repeat_after_the_repeat_period() {
    // W2: the GPU evolves from `t mod T` in f32; the twin must be the same T-periodic field.
    let params = sea(14.0);
    let cascades = Cascades::default();
    let h0 = H0::generate(&params, &cascades, 0, 64, 7);
    let mut a = CascadeTwin::new(&h0, &params);
    let mut b = CascadeTwin::new(&h0, &params);
    a.evaluate(37.25);
    b.evaluate(37.25 + REPEAT_PERIOD * 3.0);
    let err = a.grids().height.iter().zip(&b.grids().height).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max);
    assert!(err < 1e-4, "not periodic: {err} m");
    // The GPU texels carry whole cycles per period.
    let texels = h0.gpu_texels(&params);
    assert_eq!(texels.len(), 64 * 64);
    assert!(texels.iter().all(|t| t[2].fract() == 0.0));
    // Rounding moves no frequency by more than half a cycle per period.
    let k = 0.1;
    assert!((params.omega_repeat(k) - params.omega(k)).abs() <= std::f64::consts::PI / REPEAT_PERIOD + 1e-12);
}

// ---- water.shore (W3) ----

#[test]
fn shore_edt_matches_brute_force() {
    let (w, h) = (37usize, 29usize);
    let mut state = 0x1234_5678u32;
    let feature: Vec<bool> = (0..w * h)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            state >> 28 == 0
        })
        .collect();
    let fast = shore::edt_squared(&feature, w, h);
    for y in 0..h {
        for x in 0..w {
            let mut best = f64::MAX;
            for j in 0..h {
                for i in 0..w {
                    if feature[j * w + i] {
                        let (dx, dy) = (x as f64 - i as f64, y as f64 - j as f64);
                        best = best.min(dx * dx + dy * dy);
                    }
                }
            }
            assert!((fast[y * w + x] - best).abs() < 1e-6, "({x}, {y}): {} vs {best}", fast[y * w + x]);
        }
    }
}

/// An island with a long bay cut into it and an enclosed lake, in an 800 m sea.
fn island() -> WetMask {
    let mut m = WetMask::new(200, 200, 4.0, DVec2::new(-400.0, -400.0));
    for y in 50..150 {
        for x in 50..150 {
            let bay = (100..150).contains(&x) && (95..105).contains(&y);
            let lake = (65..85).contains(&x) && (65..85).contains(&y);
            m.wet[y * 200 + x] = bay || lake;
        }
    }
    m
}

#[test]
fn shore_openness_falls_into_bays_and_lakes_use_their_own_fetch() {
    let settings = ShoreSettings { open_core: 60.0, open_falloff: 150.0, ..ShoreSettings::default() };
    let f = ShoreField::bake(&island(), settings);
    assert_eq!((f.stats.components, f.stats.open_components), (2, 1));
    let at = |x: usize, y: usize| (f.distance[y * 200 + x], f.openness[y * 200 + x]);
    // Open sea far from the island: fully open.
    assert!(at(5, 5).1 > 0.99);
    // Into the bay openness falls with the geodesic distance from the open region.
    let (mouth, inner) = (at(148, 100).1, at(102, 100).1);
    assert!(mouth > inner && inner < 0.6, "mouth {mouth}, inner {inner}");
    // The lake (20 × 20 texels = 80 m) is enclosed: openness from its own size, far below the sea.
    let (d, o) = at(75, 75);
    assert!((d - 38.0).abs() < 4.5, "lake centre distance {d}");
    let want = settings.enclosed_max * (2.0 * 38.0 / settings.enclosed_fetch);
    assert!((o - want).abs() < 0.01 && o > 0.0, "lake openness {o} vs {want}");
    // Land is negative, and swell is gone at the shore while chop keeps its floor offshore.
    assert!(at(60, 120).0 < -30.0);
    assert!(f.swell_scale(0.0, 1.0) == 0.0 && f.chop_scale(10.0, 0.0) >= settings.chop_floor);
    // Outside the baked area: open deep water.
    assert_eq!(f.sample(1e6, 0.0), (settings.range, 1.0));
    // The GPU encoding (half floats) round-trips distance, openness and direction to ~cm.
    let enc = f.encode_rgba16f();
    assert_eq!(enc.len(), f.distance.len() * 8);
    for i in [100 * 200 + 148, 100 * 200 + 160, 5 * 200 + 5, 120 * 200 + 60] {
        let half = |c: usize| f16_value(u16::from_le_bytes([enc[i * 8 + c * 2], enc[i * 8 + c * 2 + 1]]));
        let want = [f.distance[i], f.openness[i], f.direction[i][0], f.direction[i][1]];
        for (c, w) in want.iter().enumerate() {
            let tol = 1e-3 + w.abs() * 1e-3;
            assert!((half(c) - w).abs() <= tol, "texel {i} channel {c}: {} vs {w}", half(c));
        }
    }
}

/// Decodes IEEE half-float bits (test side of `shore::f16_bits`).
fn f16_value(b: u16) -> f32 {
    let sign = if b & 0x8000 == 0 { 1.0 } else { -1.0 };
    let exp = i32::from((b >> 10) & 0x1f);
    let man = f32::from(b & 0x3ff);
    match exp {
        0 => sign * man * 2f32.powi(-24),
        31 => sign * f32::INFINITY,
        _ => sign * (1.0 + man / 1024.0) * 2f32.powi(exp - 15),
    }
}

#[test]
fn half_floats_round_to_nearest() {
    for x in [0.0f32, 1.0, -1.0, 0.5, 2.6, 255.9, -256.0, 1e-5, 6e-8, 65504.0, 0.1, 40.0, -0.0] {
        let back = f16_value(shore::f16_bits(x));
        assert!((back - x).abs() <= x.abs() / 2048.0 + 3e-8, "{x} -> {back}");
    }
    assert_eq!(shore::f16_bits(1e6), 0x7c00);
}

// ---- shore waves (W-GERSTNER) ----

/// Land west of x = 0, water east, 2 m texels: a straight north-south shore.
fn straight_shore() -> ShoreField {
    let mut mask = WetMask::new(128, 64, 2.0, DVec2::new(-64.0, -64.0));
    for y in 0..64 {
        for x in 0..32 {
            mask.wet[y * 128 + x] = false;
        }
    }
    ShoreField::bake(&mask, ShoreSettings::default())
}

fn shore_waves(field: &ShoreField, wind: f32) -> ShoreWaves {
    ShoreWaves::new(&sea(wind), &Cascades::default(), &field.settings)
}

/// The waves at world XZ (x, z) and time t on `field`.
fn wave_at(field: &ShoreField, w: &ShoreWaves, x: f64, z: f64, t: f32) -> ShoreWave {
    let (d, o) = field.sample(x, z);
    w.eval(d, o, field.sample_direction(x, z), ShoreWaves::noise_coords(DVec2::new(x, z)), t)
}

#[test]
fn shore_waves_follow_the_sea_state_and_point_shoreward() {
    let field = straight_shore();
    let (calm, windy) = (shore_waves(&field, 5.0), shore_waves(&field, 14.0));
    assert!(calm.amplitude > 0.0 && windy.amplitude > 2.0 * calm.amplitude, "{} vs {}", calm.amplitude, windy.amplitude);
    assert!(windy.omega[0] < calm.omega[0], "a stronger wind has a longer peak period");
    // Whole cycles per repeat period, distinct and increasing.
    for w in [calm, windy] {
        for i in 0..3 {
            let cycles = f64::from(w.omega[i]) * REPEAT_PERIOD / std::f64::consts::TAU;
            assert!((cycles - cycles.round()).abs() < 1e-3, "{cycles}");
        }
        assert!(w.omega[0] < w.omega[1] && w.omega[1] < w.omega[2]);
        assert!(w.swash > 0.0 && (2.0..=8.0).contains(&w.land_margin));
    }
    // The baked direction points at the shore (−x), unit on a straight shore.
    let dir = field.sample_direction(20.0, 0.0);
    assert!((dir - Vec2::new(-1.0, 0.0)).length() < 1e-3, "{dir}");
    // Off: no amplitude, no swash, nothing anywhere.
    let off = ShoreWaves::new(&sea(14.0), &Cascades::default(), &ShoreSettings { shore_waves: 0.0, ..ShoreSettings::default() });
    for d in [-3.0, 0.0, 5.0, 20.0] {
        assert_eq!(off.eval(d, 1.0, Vec2::NEG_X, Vec2::ZERO, 3.0), ShoreWave::default());
    }
}

#[test]
fn shore_waves_are_zero_beyond_the_band_and_finite_everywhere() {
    let field = straight_shore();
    let w = shore_waves(&field, 14.0);
    for t in [0.0, 1.3, 77.7, 255.9] {
        for d in [w.band, w.band + 0.01, 100.0, 1e4, -w.land_margin, -w.land_margin - 1.0, -50.0, f32::NAN] {
            assert_eq!(w.eval(d, 1.0, Vec2::NEG_X, Vec2::new(0.3, 0.7), t), ShoreWave::default(), "d {d}");
        }
        // No NaN at or behind the shoreline, nor at a zero direction (medial axis) or openness.
        for i in 0..=400 {
            let d = -w.land_margin + (w.band + w.land_margin) * i as f32 / 400.0;
            for (dir, o) in [(Vec2::NEG_X, 1.0), (Vec2::ZERO, 1.0), (Vec2::NEG_X, 0.0)] {
                let g = w.eval(d, o, dir, Vec2::new(0.1, 0.9), t);
                assert!(g.offset.is_finite() && g.slope.is_finite() && g.crest.is_finite(), "d {d}: {g:?}");
            }
        }
        let g = w.eval(-0.0, 1.0, Vec2::NEG_X, Vec2::ZERO, t);
        assert!(g.offset.is_finite());
    }
}

/// The analytic waves take over exactly as the FFT swell fades: their weight is the swell's
/// complement, and both the amplitude and the surface are continuous across the band (no pop).
#[test]
fn shore_waves_blend_continuously_with_the_fft_fade() {
    let field = straight_shore();
    let w = ShoreWaves { variation: 0.0, ..shore_waves(&field, 12.0) };
    // Envelope near the band's edge goes to zero smoothly (C1: smoothstep), under the open swell.
    let peak = |d: f32| (0..64).map(|i| w.eval(d, 1.0, Vec2::NEG_X, Vec2::ZERO, i as f32 * 0.37).offset.y.abs()).fold(0.0, f32::max);
    assert!(peak(w.band - 0.05) < 1e-3, "{}", peak(w.band - 0.05));
    assert!(peak(0.5 * w.band) > 0.02, "{}", peak(0.5 * w.band));
    for d in [5.0, 15.0, 25.0, 35.0] {
        let fft = field.swell_scale(d, 1.0);
        assert!(peak(d) <= w.amplitude * (1.0 - fft) * 1.25 + 1e-4, "d {d}: {} vs {}", peak(d), w.amplitude);
    }
    // Lipschitz along the shore normal at a fine step: no seam or step anywhere in the band.
    for t in [0.0, 3.1, 100.0] {
        let mut last = w.eval(-w.land_margin + 1e-3, 1.0, Vec2::NEG_X, Vec2::ZERO, t).offset;
        for i in 1..=2000 {
            let d = -w.land_margin + (w.band + w.land_margin + 1.0) * i as f32 / 2000.0;
            let g = w.eval(d, 1.0, Vec2::NEG_X, Vec2::ZERO, t).offset;
            assert!((g - last).length() < 0.05, "step at d {d}: {last} -> {g}");
            last = g;
        }
    }
    // The clock wraps without a pop (whole cycles per repeat period).
    let a = w.eval(10.0, 1.0, Vec2::NEG_X, Vec2::new(0.2, 0.4), 0.0).offset;
    let b = w.eval(10.0, 1.0, Vec2::NEG_X, Vec2::new(0.2, 0.4), REPEAT_PERIOD as f32).offset;
    assert!((a - b).length() < 2e-3, "{a} vs {b}");
}

/// On a straight shore, uniform waves have their crests parallel to the shoreline (the surface is
/// the same all along it) and travel shoreward; the noise bends them only within its bound.
#[test]
fn shore_wave_crests_parallel_the_shoreline_and_roll_in() {
    let field = straight_shore();
    let uniform = ShoreWaves { variation: 0.0, ..shore_waves(&field, 10.0) };
    for x in [2.0, 9.0, 17.0, 30.0] {
        // (Openness varies along this small test field: hold it at 1 to isolate the geometry.)
        let open = |z: f64| uniform.eval(field.sample(x, z).0, 1.0, field.sample_direction(x, z), ShoreWaves::noise_coords(DVec2::new(x, z)), 4.2).offset;
        let first = open(-40.0);
        for z in [-30.0, -11.0, 0.0, 13.0, 40.0] {
            let g = open(z);
            assert!((g - first).length() < 1e-4, "x {x} z {z}: {g} vs {first}");
        }
    }
    // Phase increases away from the shore and the local wave number is the dispersion's.
    let beta = uniform.beach_slope;
    for omega in uniform.omega {
        let mut last = f32::NEG_INFINITY;
        for i in 0..=200 {
            let d = uniform.band * i as f32 / 200.0;
            let (k, phi) = shore_waves::local_wave(omega, beta * d, beta);
            assert!(phi > last, "phase not increasing at d {d}");
            last = phi;
            if beta * d >= shore_waves::MIN_DEPTH {
                // Exact root of ω² = g k tanh(k h) by Newton; Eckart is within 5 %.
                let h = beta * d;
                let mut ke = k;
                for _ in 0..30 {
                    let f = GRAVITY * ke * (ke * h).tanh() - omega * omega;
                    let df = GRAVITY * ((ke * h).tanh() + ke * h / (ke * h).cosh().powi(2));
                    ke -= f / df;
                }
                assert!((k - ke).abs() / ke < 0.05, "k {k} vs {ke} at h {h}");
                let (_, ahead) = shore_waves::local_wave(omega, beta * (d + 0.01), beta);
                assert!(((ahead - phi) / 0.01 - k).abs() / k < 0.02, "dΦ/dd vs k at d {d}");
            }
        }
    }
    // A crest moves shoreward: track the main train's crest (θ0 = π/2 mod 2π) between two times.
    let crest_d = |t: f32, around: f32, half: f32| {
        let mut best = (f32::MAX, 0.0);
        for i in 0..4000 {
            let d = around - half + 2.0 * half * i as f32 / 4000.0;
            let (_, phi) = shore_waves::local_wave(uniform.omega[0], beta * d, beta);
            let theta = -(phi + uniform.omega[0] * t);
            let off = (theta - std::f32::consts::FRAC_PI_2).rem_euclid(std::f32::consts::TAU);
            let dist = off.min(std::f32::consts::TAU - off);
            if dist < best.0 {
                best = (dist, d);
            }
        }
        best.1
    };
    // (A ±12 m window holds a crest (λ ≈ 20 m there); in 0.1 s it moves ~0.3 m, inside ±2 m.)
    let a = crest_d(10.0, 20.0, 12.0);
    let b = crest_d(10.1, a, 2.0);
    assert!(b < a - 0.01, "crest at {a} m then {b} m");
    // With the noise the shore line still bounds the bend: the surface varies along the shore but
    // stays within the envelope.
    let varied = shore_waves(&field, 10.0);
    let ys: Vec<f32> = (0..64u32).map(|i| wave_at(&field, &varied, 12.0, -60.0 + 1.8 * f64::from(i), 4.2).offset.y).collect();
    let spread = ys.iter().fold(f32::MIN, |m, y| m.max(*y)) - ys.iter().fold(f32::MAX, |m, y| m.min(*y));
    assert!(spread > 1e-3, "no variation along the shore");
}

/// The Gerstner map never folds (no loops on crests): along the shore normal the displaced
/// position is strictly increasing for every sea state, time and noise position, and the
/// steepness sum holds its limit.
#[test]
fn shore_waves_never_loop() {
    let field = straight_shore();
    for wind in [3.0, 8.0, 14.0, 25.0] {
        let w = shore_waves(&field, wind);
        for t in [0.0, 0.7, 2.9, 31.0, 140.5] {
            for z in [-50.0, -7.0, 22.0] {
                let mut last = f64::NEG_INFINITY;
                for i in 0..=1500 {
                    let x = -f64::from(w.land_margin) + f64::from(w.band + w.land_margin) * f64::from(i) / 1500.0;
                    let g = wave_at(&field, &w, x, z, t);
                    assert!(g.steepness <= shore_waves::STEEPNESS_LIMIT + 1e-4, "{}", g.steepness);
                    let moved = x + f64::from(g.offset.x);
                    assert!(moved > last, "wind {wind} t {t}: fold at x {x}");
                    last = moved;
                }
            }
        }
    }
}

#[test]
fn query_lifts_the_surface_with_the_shore_waves() {
    let id = WorldspaceId::from("ws");
    let mut ws = Worldspace::new("ws");
    ws.has_terrain = true;
    ws.sea_level = Some(0.0);
    let params = sea(12.0);
    let cascades = Cascades::default();
    let field = Arc::new(straight_shore());
    let mut water = WorldspaceWater::new(WaterBody::from_worldspace(&id, &ws).into_iter().collect());
    water.set_waves(Some(WaveTwin::new(&params, &cascades, 7, 1, 32)));
    water.set_shore(Some(field.clone()));
    let mut world = WaterWorld::new();
    world.insert(id.clone(), water);
    world.step(12.0);
    let at = WorldPos::from_f64(glam::DVec3::new(6.0, 0.0, 3.0));
    let before = world.get(&id).unwrap().sample(at).unwrap();
    let waves = ShoreWaves::new(&params, &cascades, &field.settings);
    world.get_mut(&id).unwrap().set_shore_waves(Some(waves));
    let mut moved = false;
    for t in [12.0, 13.1, 14.7, 16.2] {
        world.step(t);
        let s = world.get(&id).unwrap().sample(at).unwrap();
        assert!(s.height.is_finite() && s.normal.is_finite() && s.velocity.is_finite());
        moved |= (s.height - before.height).abs() > 1e-3;
    }
    assert!(moved, "the shore waves never lifted the query's surface");
    // Far offshore (beyond the band) the query is the FFT twin alone.
    world.step(12.0);
    let far = WorldPos::from_f64(glam::DVec3::new(60.0, 0.0, 3.0));
    let mut plain = WorldspaceWater::new(WaterBody::from_worldspace(&id, &ws).into_iter().collect());
    plain.set_waves(Some(WaveTwin::new(&params, &cascades, 7, 1, 32)));
    plain.set_shore(Some(field));
    let mut reference = WaterWorld::new();
    reference.insert(id.clone(), plain);
    reference.step(12.0);
    assert_eq!(world.get(&id).unwrap().sample(far).unwrap().height, reference.get(&id).unwrap().sample(far).unwrap().height);
}

#[test]
fn query_swell_is_attenuated_by_the_shore_product() {
    let id = WorldspaceId::from("ws");
    let mut ws = Worldspace::new("ws");
    ws.has_terrain = true;
    ws.sea_level = Some(0.0);
    let params = sea(14.0);
    let cascades = Cascades::default();
    let mut water = WorldspaceWater::new(WaterBody::from_worldspace(&id, &ws).into_iter().collect());
    water.set_waves(Some(WaveTwin::new(&params, &cascades, 7, 1, 32)));
    let mut world = WaterWorld::new();
    world.insert(id.clone(), water);
    world.step(12.0);
    let at = WorldPos::from_f64(glam::DVec3::new(-2.0, 0.0, 3.0));
    let open = world.get(&id).unwrap().sample(at).unwrap().height;
    assert!(open.abs() > 1e-3);
    // Land west of x = 0: the point is ashore (no swell; no bed, so the query still answers).
    let mut mask = WetMask::new(64, 64, 2.0, DVec2::new(-64.0, -64.0));
    for y in 0..64 {
        for x in 0..32 {
            mask.wet[y * 64 + x] = false;
        }
    }
    let field = ShoreField::bake(&mask, ShoreSettings::default());
    world.get_mut(&id).unwrap().set_shore(Some(Arc::new(field)));
    let calm = world.get(&id).unwrap().sample(at).unwrap();
    assert!(calm.height.abs() < 1e-5, "{} vs {open}", calm.height);
}

/// W-SHORE: the chop's shading normal keeps `normal_floor` of its open-water strength at the
/// shoreline while the displacement settles to zero; far out both agree; floor 0 is the old look.
#[test]
fn shore_normals_keep_wavelets_where_the_displacement_settles() {
    let settings = ShoreSettings::default();
    let field = ShoreField::bake(&WetMask::new(4, 4, 2.0, DVec2::ZERO), settings);
    let open = |o: f32| settings.chop_floor + (1.0 - settings.chop_floor) * o;
    // At the shoreline: no chop displacement, the normal at its floor.
    assert_eq!(field.chop_scale(0.0, 1.0), 0.0);
    assert_eq!(field.normal_chop_scale(0.0, 1.0), settings.normal_floor);
    assert_eq!(field.normal_chop_scale(0.0, 0.0), settings.chop_floor * settings.normal_floor);
    // Past the chop ramp both are the open-water chop (never above it).
    for o in [0.0, 0.4, 1.0] {
        assert_eq!(field.normal_chop_scale(settings.chop_ramp, o), open(o));
        assert_eq!(field.normal_chop_scale(settings.chop_ramp, o), field.chop_scale(settings.chop_ramp, o));
    }
    // In between: the larger of the ramp and the floor, continuous, never below the displacement's.
    for i in 0..=30 {
        let d = settings.chop_ramp * i as f32 / 30.0;
        let n = field.normal_chop_scale(d, 0.7);
        assert!(n >= field.chop_scale(d, 0.7) && n >= settings.normal_floor * open(0.7) && n <= open(0.7));
    }
    // `--off shorenormals`: floor 0 is exactly the displacement's scale.
    let old = ShoreField::bake(&WetMask::new(4, 4, 2.0, DVec2::ZERO), ShoreSettings { normal_floor: 0.0, ..settings });
    for d in [0.0, 0.5, 1.5, 2.9, 10.0] {
        assert_eq!(old.normal_chop_scale(d, 0.6), old.chop_scale(d, 0.6));
    }
}

/// Prints the shore-wave model for a few winds: offshore amplitude, periods, swash, and the peak
/// crest height and local wavelength across the band.
#[test]
#[ignore = "measurement: shore wave model numbers (W-GERSTNER)"]
fn shore_wave_model_numbers() {
    let field = straight_shore();
    for wind in [4.0, 7.0, 12.0, 20.0] {
        let w = ShoreWaves { variation: 0.0, ..shore_waves(&field, wind) };
        let periods: Vec<String> = w.omega.iter().map(|o| format!("{:.2}", std::f32::consts::TAU / o)).collect();
        println!("wind {wind:4.1} m/s: A0 {:.2} m, T [{}] s, swash ±{:.2} m, land margin {:.1} m", w.amplitude, periods.join(", "), w.swash, w.land_margin);
        for d in [0.0, 2.0, 5.0, 10.0, 20.0, 30.0, 38.0] {
            let peak = (0..256).map(|i| w.eval(d, 1.0, Vec2::NEG_X, Vec2::ZERO, i as f32 * 0.25).offset.y).fold(f32::MIN, f32::max);
            let (k, _) = shore_waves::local_wave(w.omega[0], w.beach_slope * d, w.beach_slope);
            println!("  d {d:4.1} m: crest {peak:5.2} m, main λ {:5.1} m", std::f32::consts::TAU / k);
        }
    }
}
