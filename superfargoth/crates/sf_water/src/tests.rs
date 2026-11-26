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
                    river_ness: 1.0,
                },
                RiverPoint {
                    center: DVec2::new(2000.0, 300.0),
                    width: 20.0,
                    height: 40.0,
                    flow_speed: 2.0,
                    river_ness: 1.0,
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

/// WA-6: the shore foam band follows the local wave energy (`shore_foam_weight` of
/// `ShoreWave::breaking`): drawn on an open beach at 7 m/s, nothing where the trains have no height
/// (openness 0: rivers and pools far from the sea; no amplitude; shore waves off), and less on a lake.
#[test]
fn shore_foam_follows_the_wave_energy() {
    let field = straight_shore();
    let full = field.settings.foam_full_breaking;
    let w = shore_waves(&field, 7.0);
    let flat = ShoreWaves { amplitude: 0.0, ..w };
    let off = ShoreWaves::new(&sea(7.0), &Cascades::default(), &ShoreSettings { shore_waves: 0.0, ..ShoreSettings::default() });
    let lake = field.settings.enclosed_max;
    for t in [0.0, 1.7, 40.3, 200.9] {
        for noise in [Vec2::ZERO, Vec2::new(0.3, 0.7), Vec2::new(0.91, 0.12)] {
            // The 6 m band, the swash side included.
            for d in [-1.0, 0.0, 1.5, 3.0, 5.5] {
                let at = |w: &ShoreWaves, o: f32| shore_foam_weight(w.eval(d, o, Vec2::NEG_X, noise, t).breaking, full);
                assert_eq!(at(&w, 0.0), 0.0, "openness 0, d {d}");
                assert_eq!(at(&flat, 1.0), 0.0, "no amplitude, d {d}");
                assert_eq!(at(&off, 1.0), 0.0, "off, d {d}");
                let open = at(&w, 1.0);
                assert!(open > 0.0 && open <= 1.0, "open beach, d {d} t {t}: {open}");
                assert!(at(&w, lake) < open, "lake vs open, d {d} t {t}: {} vs {open}", at(&w, lake));
            }
        }
    }
    assert_eq!(shore_foam_weight(f32::NAN, full), 0.0);
    assert_eq!(shore_foam_weight(2.0, full), 1.0);
}

/// One fragment's breaker-foam input (SF-1) with the lace resolved and no waterline.
fn surf_input(breaking: f32, phase: f32, lace: f32) -> SurfFoamInput {
    SurfFoamInput { breaking, phase, swash: Vec2::ZERO, vertical: f32::INFINITY, contact: 0.35, lace, variation: 0.5, strength: 1.0, detail: 1.0 }
}

/// SF-1 (keeps W-FOAM-GATE): no bore foam and no glow where the trains do not break (breaking 0,
/// calm or enclosed water up to the largest lake's openness), and nothing at all when off. On an
/// open beach the shore waves do give bore foam.
#[test]
fn surf_foam_is_born_only_where_waves_break() {
    let s = SurfFoam::default();
    let off = SurfFoam { enabled: false, ..s };
    let lake = ShoreSettings::default().enclosed_max;
    for k in 0..64 {
        let phase = k as f32 * 0.37 - 9.0;
        for lace in [0.0, 0.1, 0.3, 0.6] {
            for breaking in [0.0, 0.2, lake, s.birth_start] {
                let t = surf_foam(&surf_input(breaking, phase, lace), &s);
                assert_eq!((t.bore, t.coverage, t.glow), (0.0, 0.0, 0.0), "breaking {breaking} phase {phase}");
            }
            let wet = SurfFoamInput { vertical: 0.0, swash: Vec2::new(1.0, 2.0), ..surf_input(1.0, phase, lace) };
            assert_eq!(surf_foam(&wet, &off), SurfFoamTerms::default(), "off");
        }
    }
    // The waves themselves: an enclosed water's trains never reach the birth; an open beach's do.
    let field = straight_shore();
    let mut open_bore = 0.0f32;
    for wind in [7.0, 14.0, 30.0] {
        let w = shore_waves(&field, wind);
        for t in [0.0, 3.1, 77.7] {
            for d in [0.5, 2.0, 5.0, 12.0, 25.0] {
                let at = |o: f32| w.eval(d, o, Vec2::NEG_X, Vec2::new(0.3, 0.7), t);
                let lakewave = at(lake);
                let calm = surf_foam(&surf_input(lakewave.breaking, lakewave.phase, 0.2), &s);
                assert_eq!(calm.bore, 0.0, "lake, wind {wind} d {d}: breaking {}", lakewave.breaking);
                let g = at(1.0);
                open_bore = open_bore.max(surf_birth(g.breaking, &s));
            }
        }
    }
    assert!(open_bore > 0.5, "no breaking on an open beach: {open_bore}");
}

/// SF-1: bore foam is densest on the crest (θ = π/2) and decays monotonically behind it (the
/// roller's trail, eroded by the narrowing lace), for any lace sample and detail.
#[test]
fn bore_foam_peaks_on_the_crest_and_decays_behind() {
    let s = SurfFoam::default();
    let crest = 0.5 * std::f32::consts::PI;
    for detail in [0.0, 0.5, 1.0] {
        for lace in [0.0, 0.05, 0.2, 0.45, 0.7] {
            let at = |theta: f32| surf_foam(&SurfFoamInput { detail, ..surf_input(1.0, theta, lace) }, &s).bore;
            let top = at(crest);
            assert!(top > 0.0, "detail {detail} lace {lace}");
            let mut last = top;
            // Behind the crest: θ falls with time at a point, so "behind" is θ < π/2.
            for k in 1..200 {
                let behind = k as f32 * (std::f32::consts::TAU - s.front_rad) / 200.0;
                let v = at(crest - behind);
                assert!(v <= last + 1e-6, "rises behind the crest: {v} after {last} at {behind} rad (lace {lace})");
                last = v;
            }
            assert!(last < 0.05 * top, "no decay: {last} vs {top}");
            // Ahead of the crest (the front feather) it is below the crest's.
            for k in 0..20 {
                let ahead = k as f32 * s.front_rad / 20.0;
                assert!(at(crest + ahead) <= top + 1e-6, "ahead {ahead}");
            }
        }
    }
    // Fresh lace covers the cells on the crest; aged foam keeps only the cell borders.
    assert!(surf_foam(&surf_input(1.0, crest, 0.45), &s).bore > 0.9999);
    assert_eq!(surf_foam(&surf_input(1.0, crest - 4.0, 0.45), &s).bore, 0.0);
    assert!(surf_foam(&surf_input(1.0, crest - 4.0, 0.0), &s).bore > 0.0);
}

/// SF-1: the waterline foam is a thin bright line at the very edge, 0 beyond its reach (the
/// contact depth, widened by the uprush), faint on calm water and full with turbulence.
#[test]
fn waterline_foam_ends_at_its_reach() {
    let s = SurfFoam::default();
    let contact = 0.35;
    for swash in [Vec2::ZERO, Vec2::new(1.0, 2.0), Vec2::new(0.6, -1.0), Vec2::new(1.0, 4.4)] {
        let reach = contact * (1.0 + s.swash_spread * swash_edge(swash, &s));
        for breaking in [0.0, 0.7, 1.0] {
            for lace in [0.0, 0.2, 0.5] {
                let at = |vertical: f32| surf_foam(&SurfFoamInput { swash, vertical, ..surf_input(breaking, 0.3, lace) }, &s).waterline;
                for v in [reach * 1.001, reach * 1.01, 2.0 * contact * (1.0 + s.swash_spread), 10.0] {
                    assert_eq!(at(v), 0.0, "beyond the reach {reach}: vertical {v}");
                }
                assert!(at(0.0) >= s.contact_calm, "edge line");
            }
        }
    }
    let calm = SurfFoamInput { vertical: 0.0, ..surf_input(0.0, 0.3, 0.3) };
    assert_eq!(surf_foam(&calm, &s).waterline, s.contact_calm);
    assert!(surf_foam(&SurfFoamInput { breaking: 1.0, ..calm }, &s).waterline > 0.9999);
    assert_eq!(surf_foam(&SurfFoamInput { contact: 0.0, ..calm }, &s).waterline, 0.0, "no contact depth");
    // The uprush (cos g < 0) carries the swash edge; the backwash keeps a fading residue.
    let edge = |g: f32| swash_edge(Vec2::new(1.0, g), &s);
    assert_eq!(edge(std::f32::consts::PI), 1.0, "mid-uprush");
    assert!(edge(0.0) < 1.0 && edge(0.0) > edge(-1.0), "residue fades through the backwash");
    assert_eq!(swash_edge(Vec2::new(0.0, 3.0), &s), 0.0);
}

/// SF-1: every term stays in range and finite over a grid of inputs, NaN ones included.
#[test]
fn surf_foam_stays_in_range() {
    let s = SurfFoam::default();
    let nan = f32::NAN;
    let mut n = 0;
    for breaking in [-1.0, 0.0, 0.6, 0.9, 1.0, 3.0, nan] {
        for phase in [-50.0, -1.0, 0.0, 1.0, 1.6, 3.0, 400.0, nan] {
            for swash in [Vec2::ZERO, Vec2::new(0.5, 1.0), Vec2::new(1.0, -2.0), Vec2::new(2.0, 9.0), Vec2::new(nan, 1.0), Vec2::new(1.0, nan)] {
                for vertical in [0.0, 0.03, 0.2, 1.0, f32::INFINITY, nan, -1.0] {
                    for lace in [-0.1, 0.0, 0.3, 0.9, nan] {
                        for detail in [0.0, 0.5, 1.0] {
                            let i = SurfFoamInput { breaking, phase, swash, vertical, contact: 0.35, lace, variation: 0.3, strength: 1.0, detail };
                            let t = surf_foam(&i, &s);
                            for (name, v) in [("bore", t.bore), ("waterline", t.waterline), ("coverage", t.coverage), ("fresh", t.fresh)] {
                                assert!((0.0..=1.0).contains(&v), "{name} {v} at {i:?}");
                            }
                            assert!((0.0..=s.glow).contains(&t.glow), "glow {} at {i:?}", t.glow);
                            assert!(t.d_lace.is_finite() && t.d_lace <= 0.0, "slope {} at {i:?}", t.d_lace);
                            n += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(n > 10_000);
}

/// SF-1: the shore waves export their main train's phase (crest = sin θ, `crest_cos` = cos θ) and a
/// swash weight in [0, 1], zero without swash.
#[test]
fn shore_waves_export_the_phase_and_swash() {
    let field = straight_shore();
    let w = shore_waves(&field, 14.0);
    let dry = ShoreWaves { swash: 0.0, ..w };
    for t in [0.0, 2.2, 130.4] {
        for d in [-1.0, 0.0, 1.0, 4.0, 20.0] {
            let g = w.eval(d, 1.0, Vec2::NEG_X, Vec2::new(0.4, 0.1), t);
            assert!((g.crest - g.phase.sin()).abs() < 1e-5 && (g.crest_cos - g.phase.cos()).abs() < 1e-5, "d {d}: {g:?}");
            assert!((0.0..=1.0).contains(&g.swash.x) && g.swash.y.is_finite(), "d {d}: {g:?}");
            assert_eq!(dry.eval(d, 1.0, Vec2::NEG_X, Vec2::new(0.4, 0.1), t).swash.x, 0.0);
        }
    }
    assert!(w.eval(0.0, 1.0, Vec2::NEG_X, Vec2::ZERO, 1.0).swash.x > 0.0, "no swash at the shoreline");
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
                let mut last_bore = false;
                for i in 0..=1500 {
                    let x = -f64::from(w.land_margin) + f64::from(w.band + w.land_margin) * f64::from(i) / 1500.0;
                    let g = wave_at(&field, &w, x, z, t);
                    // SF-2: inside the bore the front may stand vertical (`SurfShape::bore_loop`).
                    let bore = g.lifecycle.y > 0.0;
                    let cap = if bore { w.surf.bore_loop.max(shore_waves::STEEPNESS_LIMIT) } else { shore_waves::STEEPNESS_LIMIT };
                    assert!(g.steepness <= cap + 1e-4, "{}", g.steepness);
                    let moved = x + f64::from(g.offset.x);
                    assert!(moved > last || bore || last_bore, "wind {wind} t {t}: fold at x {x}");
                    last = moved;
                    last_bore = bore;
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

// ---- bed slope (W-GERSTNER-DEPTH) ----

/// The straight shore of [`straight_shore`] over a bed `depth(d, z)` (m below the rest level at shore
/// distance `d`; the field's shoreline is at x = −1, so `d = x + 1` exactly on both sides).
fn shore_on_bed(depth: impl Fn(f64, f64) -> f32) -> ShoreField {
    let mut mask = WetMask::new(128, 64, 2.0, DVec2::new(-64.0, -64.0));
    let mut z = vec![0.0f32; 128 * 64];
    for y in 0..64 {
        for x in 0..128 {
            mask.wet[y * 128 + x] = x >= 32;
            let (wx, wz) = (-64.0 + 2.0 * x as f64, -64.0 + 2.0 * y as f64);
            z[y * 128 + x] = depth(wx + 1.0, wz);
        }
    }
    mask.depth = Some(z);
    ShoreField::bake(&mask, ShoreSettings::default())
}

#[test]
fn bed_slope_bakes_the_real_bed_and_honours_its_clamp() {
    use crate::shore::{BED_SLOPE_MAX, BED_SLOPE_MIN};
    // A 1:10 bed: β = 0.1 everywhere, in the water and up the beach.
    let ten = shore_on_bed(|d, _| (0.1 * d) as f32);
    for x in [-7.0, -1.5, 0.0, 3.0, 10.0, 25.0, 39.0, 80.0] {
        for z in [-50.0, 0.0, 31.0] {
            let b = ten.sample_slope(x, z);
            assert!((b - 0.1).abs() <= 0.01, "x {x} z {z}: β {b}");
        }
    }
    let [p10, p50, p90] = ten.bed_slope_percentiles(40.0).expect("bathymetry");
    assert!((p10 - 0.1).abs() <= 0.01 && (p50 - 0.1).abs() <= 0.01 && (p90 - 0.1).abs() <= 0.01);
    // The clamp: a cliff (1:1.7) and a tidal flat (1:200).
    let cliff = shore_on_bed(|d, _| (0.6 * d) as f32);
    let flat = shore_on_bed(|d, _| (0.005 * d) as f32);
    for x in [0.0, 10.0, 30.0] {
        assert!((cliff.sample_slope(x, 0.0) - BED_SLOPE_MAX).abs() < 1e-6, "{}", cliff.sample_slope(x, 0.0));
        assert!((flat.sample_slope(x, 0.0) - BED_SLOPE_MIN).abs() < 1e-6, "{}", flat.sample_slope(x, 0.0));
    }
    // Unknown depths (NaN: no `LAND`) take the beach slope; the blur bends β smoothly between.
    let patchy = shore_on_bed(|d, z| if z > 0.0 { f32::NAN } else { (0.2 * d) as f32 });
    let beach = ShoreSettings::default().beach_slope;
    assert!((patchy.sample_slope(20.0, 40.0) - beach).abs() < 1e-6, "{}", patchy.sample_slope(20.0, 40.0));
    assert!((patchy.sample_slope(20.0, -40.0) - 0.2).abs() < 1e-6);
    let mut last = patchy.sample_slope(20.0, -40.0);
    for i in 1..=80 {
        let b = patchy.sample_slope(20.0, -40.0 + f64::from(i));
        assert!(b <= last + 1e-6 && (last - b) < 0.03, "β step at z {}: {last} -> {b}", -40 + i);
        last = b;
    }
    // Every texel finite and in range; the texture round-trips.
    for f in [&ten, &cliff, &flat, &patchy] {
        assert!(f.bed_slope.as_ref().unwrap().iter().all(|b| (BED_SLOPE_MIN..=BED_SLOPE_MAX).contains(b)));
        assert_eq!(f.encode_slope_r16f().unwrap().len(), f.width * f.height * 2);
    }
    let texel = ten.encode_slope_r16f().unwrap();
    assert!((f16_value(u16::from_le_bytes([texel[0], texel[1]])) - 0.1).abs() < 1e-3);
}

#[test]
fn bed_slope_falls_back_to_the_beach_without_bathymetry() {
    let field = straight_shore();
    assert!(field.bed_slope.is_none() && field.encode_slope_r16f().is_none() && field.bed_slope_percentiles(40.0).is_none());
    let beach = field.settings.beach_slope;
    assert_eq!(field.sample_slope(10.0, 0.0), beach);
    // Outside the field too, with bathymetry.
    let ten = shore_on_bed(|d, _| (0.1 * d) as f32);
    assert_eq!(ten.sample_slope(1e5, 0.0), beach);
    // A depth grid of the wrong size is ignored, not misread.
    let mut mask = WetMask::new(8, 8, 2.0, DVec2::ZERO);
    mask.depth = Some(vec![1.0; 7]);
    assert!(ShoreField::bake(&mask, ShoreSettings::default()).bed_slope.is_none());
    // The uniform evaluation is exactly the bed one at the beach slope.
    let w = shore_waves(&field, 12.0);
    for d in [-2.0, 0.0, 3.0, 17.0, 33.0] {
        assert_eq!(w.eval(d, 0.8, Vec2::NEG_X, Vec2::new(0.4, 0.1), 5.5), w.eval_on(d, 0.8, beach, Vec2::NEG_X, Vec2::new(0.4, 0.1), 5.5));
    }
    // A NaN slope falls back too (no NaN surface).
    let g = w.eval_on(5.0, 1.0, f32::NAN, Vec2::NEG_X, Vec2::ZERO, 1.0);
    assert_eq!(g, w.eval(5.0, 1.0, Vec2::NEG_X, Vec2::ZERO, 1.0));
}

/// The waves at world XZ (x, z) and time t on `field`'s bed.
fn wave_on_bed(field: &ShoreField, w: &ShoreWaves, x: f64, z: f64, t: f32) -> ShoreWave {
    let (d, o) = field.sample(x, z);
    w.eval_on(d, o, field.sample_slope(x, z), field.sample_direction(x, z), ShoreWaves::noise_coords(DVec2::new(x, z)), t)
}

/// No NaN and no loops (the displaced position strictly increases along the shore normal: the
/// Gerstner Jacobian stays > 0) over the whole bed-slope range, a bed that steepens offshore and
/// along the shore, and winds up to a storm.
#[test]
fn shore_waves_on_any_bed_stay_finite_and_never_loop() {
    let beds: Vec<ShoreField> = [0.02, 0.05, 0.1, 0.2, 0.3]
        .into_iter()
        .map(|b| shore_on_bed(move |d, _| (b * d) as f32))
        .chain(std::iter::once(shore_on_bed(|d, z| (d * (0.05 + 0.25 * (0.5 + 0.5 * (z / 20.0).sin())) * (1.0 + 0.02 * d.max(0.0))) as f32)))
        .collect();
    for field in &beds {
        for wind in [3.0, 8.0, 14.0, 25.0, 30.0] {
            let w = shore_waves(field, wind);
            for t in [0.0, 1.9, 47.3, 200.1] {
                for z in [-40.0, -9.0, 14.0, 37.0] {
                    let mut last = f64::NEG_INFINITY;
                    let mut last_bore = false;
                    for i in 0..=1200 {
                        let x = -1.0 - f64::from(w.land_margin) + f64::from(w.band + w.land_margin) * f64::from(i) / 1200.0;
                        let g = wave_on_bed(field, &w, x, z, t);
                        assert!(g.offset.is_finite() && g.slope.is_finite() && g.crest.is_finite(), "wind {wind}: {g:?}");
                        // SF-2: inside the bore the front may stand vertical (`SurfShape::bore_loop`).
                        let bore = g.lifecycle.y > 0.0;
                        let cap = if bore { w.surf.bore_loop.max(shore_waves::STEEPNESS_LIMIT) } else { shore_waves::STEEPNESS_LIMIT };
                        assert!(g.steepness <= cap + 1e-4);
                        let moved = x + f64::from(g.offset.x);
                        assert!(moved > last || bore || last_bore, "β {} wind {wind} t {t} z {z}: fold at x {x}", field.sample_slope(x, z));
                        last = moved;
                        last_bore = bore;
                    }
                }
            }
        }
    }
}

/// The CPU query lifts its surface over the baked bed, as the GPU does: a steep bed's crests differ
/// from the 1:20 proxy's at the same point and time.
#[test]
fn query_shore_waves_follow_the_bed_slope() {
    let id = WorldspaceId::from("ws");
    let mut ws = Worldspace::new("ws");
    ws.has_terrain = true;
    ws.sea_level = Some(0.0);
    let params = sea(14.0);
    let cascades = Cascades::default();
    let world_on = |field: ShoreField| {
        let mut water = WorldspaceWater::new(WaterBody::from_worldspace(&id, &ws).into_iter().collect());
        water.set_waves(Some(WaveTwin::new(&params, &cascades, 7, 1, 32)));
        water.set_shore_waves(Some(ShoreWaves::new(&params, &cascades, &field.settings)));
        water.set_shore(Some(Arc::new(field)));
        let mut world = WaterWorld::new();
        world.insert(id.clone(), water);
        world
    };
    let (mut proxy, mut steep) = (world_on(straight_shore()), world_on(shore_on_bed(|d, _| (0.2 * d) as f32)));
    let at = WorldPos::from_f64(glam::DVec3::new(9.0, 0.0, 3.0));
    let mut differs = false;
    for t in [3.0, 4.4, 6.1, 8.0] {
        proxy.step(t);
        steep.step(t);
        let (a, b) = (proxy.get(&id).unwrap().sample(at).unwrap(), steep.get(&id).unwrap().sample(at).unwrap());
        assert!(b.height.is_finite() && b.normal.is_finite() && b.velocity.is_finite());
        differs |= (a.height - b.height).abs() > 1e-3;
    }
    assert!(differs, "the query ignores the bed slope");
}

/// Prints the shore waves over real bed slopes (W-GERSTNER-DEPTH): the breaker cap, peak crest,
/// main wavelength and swash at a few depths, slopes and winds.
#[test]
#[ignore = "measurement: shore waves over the bed slope (W-GERSTNER-DEPTH)"]
fn shore_wave_bed_numbers() {
    let field = straight_shore();
    for wind in [7.0, 14.0, 30.0] {
        let w = ShoreWaves { variation: 0.0, ..shore_waves(&field, wind) };
        println!("wind {wind:4.1} m/s: A0 {:.2} m, T {:.2} s, swash at 1:20 ±{:.2} m", w.amplitude, std::f32::consts::TAU / w.omega[0], w.swash);
        for beta in [0.05, 0.1, 0.2] {
            let swash = (0..512).map(|i| w.eval_on(-0.5, 1.0, beta, Vec2::NEG_X, Vec2::ZERO, i as f32 * 0.25).offset.y.abs()).fold(0.0, f32::max);
            for d in [5.0, 10.0, 20.0] {
                let peak = (0..512).map(|i| w.eval_on(d, 1.0, beta, Vec2::NEG_X, Vec2::ZERO, i as f32 * 0.25).offset.y).fold(f32::MIN, f32::max);
                let (k, _) = shore_waves::local_wave(w.omega[0], beta * d, beta);
                let cap = 0.5 * shore_waves::BREAKER_INDEX * beta * d;
                println!(
                    "  β {beta:.2} d {d:4.1} m (h {:4.1} m): cap {cap:4.2} m, crest {peak:4.2} m, main λ {:5.1} m, swash ±{swash:4.2} m",
                    beta * d,
                    std::f32::consts::TAU / k
                );
            }
        }
    }
}

// ---- surf shape (SF-2) ----

fn smooth(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The shore waves as they were before SF-2 (`ShoreWaves::eval_trains` at variation 0: sets 1, no
/// phase bends, so no noise), op for op, for the `--off surfskew` arm's exactness.
fn pre_sf2(w: &ShoreWaves, d: f32, o: f32, beta: f32, dir: Vec2, t: f32) -> ShoreWave {
    use shore_waves::{BREAKER_INDEX, MIN_DEPTH, ORBIT_LIMIT, SHARES, STEEPNESS_LIMIT, TRAINS, local_wave};
    let beta = if beta.is_finite() { beta.max(1e-3) } else { w.beach_slope };
    if !(d < w.band && d > -w.land_margin) || (w.amplitude <= 0.0 && w.swash <= 0.0) {
        return ShoreWave::default();
    }
    let miche = 0.142 * std::f32::consts::PI;
    let h = beta * d.max(0.0);
    let h_edge = beta * w.band;
    let ww = o * (1.0 - smooth(0.0, w.band, d));
    let set = 1.0f32;
    let shoal = w.amplitude * (h_edge / h.max(MIN_DEPTH)).max(1.0).powf(0.25);
    let breaker = 0.5 * BREAKER_INDEX * h;
    let env = shoal * breaker / (shoal.powi(4) + breaker.powi(4) + 1e-20).powf(0.25);
    let common = env * ww * set;
    let breaker_r = 0.5 * BREAKER_INDEX * h.max(MIN_DEPTH);
    let breaking = ww * set * shoal / (shoal.powi(4) + breaker_r.powi(4) + 1e-20).powf(0.25);
    let mut trains = [(0.0f32, 0.0f32, 0.0f32, 0.0f32); TRAINS];
    let mut steep = 0.0;
    let coth_cap = ORBIT_LIMIT / (0.5 * BREAKER_INDEX * beta);
    for (i, train) in trains.iter_mut().enumerate() {
        let (k, phi) = local_wave(w.omega[i], h, beta);
        let th = (k * h.max(MIN_DEPTH)).tanh();
        let a = (SHARES[i] * common).min(miche * th / k);
        let theta = -(phi + w.omega[i] * t);
        let coth = (1.0 / th).min(coth_cap);
        steep += k * a * coth;
        *train = (a, k, coth, theta);
    }
    let q = (STEEPNESS_LIMIT / steep.max(1e-6)).min(1.0);
    let (mut y, mut along, mut dy, mut dalong) = (0.0, 0.0, 0.0, 0.0);
    for (a, k, coth, theta) in trains {
        let (s, c) = theta.sin_cos();
        y += a * s;
        along += q * a * coth * c;
        dy += a * k * c;
        dalong -= q * a * coth * k * s;
    }
    let (_, phi0) = local_wave(w.omega[0], 0.0, beta);
    let theta0 = -(phi0 + w.omega[0] * t);
    let swash_w = o * set * (1.0 - smooth(0.0, 0.3 * w.band, d)) * smooth(-w.land_margin, -0.5 * w.land_margin, d);
    let swash = w.swash * (beta / w.beach_slope.max(1e-3)) * swash_w;
    let g = theta0 - 0.5 * theta0.sin();
    y += swash * g.sin();
    ShoreWave {
        offset: Vec3::new(dir.x * along, y, dir.y * along),
        slope: glam::Vec4::new(dy * dir.x, dy * dir.y, dalong * dir.x * dir.x, dalong * dir.y * dir.y),
        crest: trains[0].3.sin(),
        crest_cos: trains[0].3.cos(),
        phase: trains[0].3,
        swash: Vec2::new(if w.swash > 0.0 { swash_w } else { 0.0 }, g),
        steepness: q * steep,
        breaking,
        lifecycle: Vec2::ZERO,
    }
}

/// `--off surfskew` is the old shape exactly (== on every field, a grid of distances, beds, times,
/// openness and winds), and the surf gain raises only the incident waves, not the swash.
#[test]
fn surf_skew_off_is_the_old_shape() {
    let off = ShoreSettings { surf_shape: SurfShape::off(), ..ShoreSettings::default() };
    for wind in [4.0, 7.0, 14.0, 25.0] {
        let w = ShoreWaves { variation: 0.0, ..ShoreWaves::new(&sea(wind), &Cascades::default(), &off) };
        let on = ShoreWaves::new(&sea(wind), &Cascades::default(), &ShoreSettings::default());
        let gain = SurfShape::default().height_gain;
        assert!((on.amplitude - gain * w.amplitude).abs() <= 1e-5 * on.amplitude.max(1.0), "{} vs {}", on.amplitude, w.amplitude);
        assert_eq!(on.swash, w.swash);
        for beta in [0.02, 0.05, 0.1, 0.3] {
            for i in 0..=160 {
                let d = -1.0 - w.land_margin + (w.band + w.land_margin + 2.0) * i as f32 / 160.0;
                for t in [0.0, 1.3, 47.9, 200.2] {
                    for o in [1.0, 0.35] {
                        let g = w.eval_on(d, o, beta, Vec2::NEG_X, Vec2::new(0.3, 0.7), t);
                        assert_eq!(g, pre_sf2(&w, d, o, beta, Vec2::NEG_X, t), "wind {wind} β {beta} d {d} t {t} o {o}");
                    }
                }
            }
        }
    }
}

/// With the defaults and with every tunable at its range's end: no NaN, the lifecycle in range, the
/// horizontal map never folds outside the bore (the displaced position strictly increases with shore
/// distance), and the lifecycle actually happens (skew and bore somewhere).
#[test]
fn surf_shape_stays_finite_and_folds_only_in_the_bore() {
    let extreme = SurfShape {
        height_gain: 2.5,
        breaker_gamma: 1.0,
        skew_steepen: 0.9,
        skew_bore: 0.9,
        break_sharpness: 16.0,
        break_crest_min_m: 3.0,
        bore_loop: 1.5,
        ..SurfShape::default()
    };
    for shape in [SurfShape::default(), extreme] {
        let settings = ShoreSettings { surf_shape: shape, ..ShoreSettings::default() };
        let (mut skewed, mut bored) = (0usize, 0usize);
        for wind in [3.0, 8.0, 14.0, 25.0] {
            let w = ShoreWaves::new(&sea(wind), &Cascades::default(), &settings);
            for beta in [0.02, 0.05, 0.1, 0.3] {
                for t in [0.0, 2.9, 31.0, 140.5] {
                    let (mut last, mut last_bore) = (f64::NEG_INFINITY, false);
                    for i in 0..=1500 {
                        let d = -w.land_margin + (w.band + w.land_margin) * i as f32 / 1500.0;
                        let g = w.eval_on(d, 1.0, beta, Vec2::NEG_X, Vec2::new(0.42, 0.17), t);
                        assert!(
                            g.offset.is_finite() && g.slope.is_finite() && g.lifecycle.is_finite() && g.crest.is_finite(),
                            "{shape:?} wind {wind} β {beta} d {d}: {g:?}"
                        );
                        assert!((0.0..1.0).contains(&g.lifecycle.x) && (0.0..=1.0).contains(&g.lifecycle.y), "{g:?}");
                        skewed += usize::from(g.lifecycle.x > 0.0);
                        bored += usize::from(g.lifecycle.y > 0.5);
                        let bore = g.lifecycle.y > 0.0;
                        // dir = −x: the displaced shore distance is d + offset.x.
                        let moved = f64::from(d) + f64::from(g.offset.x);
                        assert!(moved > last || bore || last_bore, "{shape:?} wind {wind} β {beta} t {t}: fold at d {d}");
                        last = moved;
                        last_bore = bore;
                    }
                }
            }
        }
        assert!(skewed > 0 && bored > 0, "no lifecycle: {skewed} skewed, {bored} in the bore");
    }
}

/// `surf_height_gain` raises the incident waves: seaward of the break the crest grows with the gain
/// (∝ it, less the smooth min), the break line moves seaward, and inside the surf zone the crest
/// stays depth-limited (H = γh whatever the offshore height, as on a real saturated beach). On a
/// 1:3 bed so the break falls well inside the band (on the default 1:20 bed a 7 m/s sea already
/// breaks at the band's edge: the whole band is surf zone, where the gain cannot show).
#[test]
fn surf_gain_raises_the_crest() {
    const BETA: f32 = 0.3;
    let waves = |gain: f32| {
        let settings = ShoreSettings { surf_shape: SurfShape { height_gain: gain, ..SurfShape::default() }, ..ShoreSettings::default() };
        ShoreWaves { variation: 0.0, ..ShoreWaves::new(&sea(7.0), &Cascades::default(), &settings) }
    };
    let peak = |w: &ShoreWaves, d: f32| {
        (0..512).map(|i| w.eval_on(d, 1.0, BETA, Vec2::NEG_X, Vec2::ZERO, i as f32 * 0.25).offset.y).fold(f32::MIN, f32::max)
    };
    // The break point: the shoaled amplitude meets the depth-limited one, A·(band/d)^¼ = ½γβd.
    let gamma = SurfShape::default().breaker_gamma;
    let break_at = |w: &ShoreWaves| (w.amplitude * w.band.powf(0.25) / (0.5 * gamma * BETA)).powf(0.8);
    let [w1, w15, w2] = [waves(1.0), waves(1.5), waves(2.0)];
    let (b1, b2) = (break_at(&w1), break_at(&w2));
    assert!(1.3 * b2 < 0.8 * w2.band && 0.6 * b1 > 2.0, "break at {b1} / {b2} m: pick another sea or bed");
    for i in 0..4 {
        let d = 1.3 * b2 + (0.8 * w2.band - 1.3 * b2) * i as f32 / 3.0;
        let (a, b, c) = (peak(&w1, d), peak(&w15, d), peak(&w2, d));
        assert!(a > 0.0 && b > 1.3 * a && c > 1.6 * a, "offshore d {d}: crest {a} / {b} / {c} m at gain 1 / 1.5 / 2");
    }
    // The bore's seaward edge moves out with the gain.
    let bore_edge = |w: &ShoreWaves| {
        (0..=400).map(|i| w.band * i as f32 / 400.0).filter(|&d| w.eval_on(d, 1.0, BETA, Vec2::NEG_X, Vec2::ZERO, 0.0).lifecycle.y > 0.0).fold(0.0, f32::max)
    };
    let (e1, e15, e2) = (bore_edge(&w1), bore_edge(&w15), bore_edge(&w2));
    assert!(e1 > 0.0 && e15 > e1 && e2 > e15, "bore edge {e1} / {e15} / {e2} m at gain 1 / 1.5 / 2");
    for d in [0.3 * b1, 0.6 * b1] {
        let (a, c) = (peak(&w1, d), peak(&w2, d));
        assert!(c >= a - 1e-4 && c <= 1.1 * a, "surf zone d {d}: crest {a} / {c} m at gain 1 / 2 (depth-limited)");
    }
}

/// The lifecycle: no skew while shoaling, the steepening's skew before the bore, the full bore's
/// skew past `bore_full`, nothing of the bore in a gap; and no skew far out on the twin.
#[test]
fn surf_skew_starts_at_the_steepening() {
    let s = SurfShape::default();
    let mut last = 0.0;
    for i in 0..=300 {
        let b = 3.0 * i as f32 / 300.0;
        let l = surf_life(&s, b, 0.1, 1.0, 0.0);
        if b < s.steepen_start {
            assert_eq!((l.skew, l.bore, l.skew_rate), (0.0, 0.0, 0.0), "b {b}");
        }
        if b < s.bore_start {
            assert!(l.bore == 0.0 && l.skew <= s.skew_steepen + 1e-6, "b {b}: {l:?}");
        }
        if b >= s.bore_full {
            assert!((l.bore - 1.0).abs() < 1e-6 && (l.skew - s.skew_bore).abs() < 1e-6, "b {b}: {l:?}");
        }
        assert!(l.skew >= last - 1e-6, "skew falls at b {b}");
        last = l.skew;
        let gap = surf_life(&s, b, 0.1, 0.0, 0.0);
        assert!(gap.bore == 0.0 && gap.skew <= s.skew_steepen + 1e-6, "gap at b {b}: {gap:?}");
    }
    let w = ShoreWaves { variation: 0.0, ..ShoreWaves::new(&sea(7.0), &Cascades::default(), &ShoreSettings::default()) };
    for t in [0.0, 5.0, 77.0] {
        assert_eq!(w.eval(0.95 * w.band, 1.0, Vec2::NEG_X, Vec2::ZERO, t).lifecycle, Vec2::ZERO);
    }
}

/// The break gaps: along a long crest deep in the surf zone some points break fully and some not at
/// all, and a point's state holds along its wave ray (the same shoreline foot at every distance).
#[test]
fn surf_gaps_leave_some_crests_unbroken() {
    let w = ShoreWaves::new(&sea(14.0), &Cascades::default(), &ShoreSettings::default());
    let (mut broken, mut gaps) = (0usize, 0usize);
    for i in 0..1024 {
        let foot = Vec2::new(0.25, i as f32 / 1024.0);
        // dir = −x: the foot is noise + d·dir / period, so the noise sits d / period seaward of it.
        let at = |d: f32| w.eval(d, 1.0, Vec2::NEG_X, foot + Vec2::new(d / shore_waves::NOISE_PERIOD as f32, 0.0), 10.0).lifecycle.y;
        let bore = at(2.0);
        broken += usize::from(bore > 0.95);
        gaps += usize::from(bore < 0.05);
        for d in [1.0, 3.0] {
            assert!((at(d) - bore).abs() < 1e-4, "foot {foot}: {} at d {d}, {bore} at 2 m", at(d));
        }
    }
    assert!(broken > 200 && gaps > 20, "{broken} broken, {gaps} in gaps of 1024");
}
