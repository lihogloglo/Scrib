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
