//! Headless check of the water core (ROADMAP M6 W1): builds an ocean worldspace with a lake, steps
//! the CPU wave twin at 60 Hz, samples the query, and prints the costs against water.md's CPU
//! budget (twin ≤ 0.05 ms per step, ≤ 2 µs per sample).
//!
//! `cargo run --release -p sf_water --example water_bench`
#![allow(clippy::cast_precision_loss)]

use std::hint::black_box;
use std::time::Instant;

use glam::{DVec2, IVec3, Vec2, Vec3};
use sf_coords::WorldPos;
use sf_water::{
    Cascades, SpectrumParams, TWIN_RESOLUTION, WaterBody, WaterKind, WaterQuery, WaterShape,
    WaterWorld, WaveTerms, WaveTwin, WindInput, WorldspaceWater,
};
use sf_world::{Worldspace, WorldspaceId};

fn main() {
    let ws = WorldspaceId::new("bench");
    let mut w = Worldspace::new("bench");
    w.has_terrain = true;
    w.sea_level = Some(0.0);
    let mut bodies = vec![WaterBody::from_worldspace(&ws, &w).unwrap()];
    for i in 0..200 {
        let c = DVec2::new(
            f64::from(i % 20) * 300.0 + 5000.0,
            f64::from(i / 20) * 300.0,
        );
        bodies.push(WaterBody {
            id: format!("lake{i}").as_str().into(),
            worldspace: ws.clone(),
            kind: WaterKind::Lake,
            shape: WaterShape::Polygon {
                outline: (0..16)
                    .map(|k| {
                        c + DVec2::from_angle(f64::from(k) * std::f64::consts::TAU / 16.0) * 100.0
                    })
                    .collect(),
                holes: Vec::new(),
            },
            level: 50.0,
            priority: 10,
            optics: None,
            waves: WaveTerms::for_kind(WaterKind::Lake),
            flow: None,
        });
    }
    for (wind, count) in [(5.0, 1), (12.0, 1), (20.0, 1), (20.0, 2)] {
        let params = SpectrumParams {
            wind: WindInput {
                speed: wind,
                toward: Vec2::new(0.6, 0.8),
            },
            ..SpectrumParams::default()
        };
        let t = Instant::now();
        let twin = WaveTwin::new(&params, &Cascades::default(), 1, count, TWIN_RESOLUTION);
        let build = t.elapsed();
        let bins: usize = twin
            .cascades
            .iter()
            .map(sf_water::CascadeTwin::active_bins)
            .sum();
        let mut water = WorldspaceWater::new(bodies.clone());
        water.set_waves(Some(twin));
        let mut world = WaterWorld::new();
        world.insert(ws.clone(), water);

        let steps = 2000;
        world.step(0.0);
        let t = Instant::now();
        for i in 0..steps {
            world.step(f64::from(i) / 60.0);
        }
        let step_ms = t.elapsed().as_secs_f64() * 1e3 / f64::from(steps);

        let pts: Vec<WorldPos> = (0..10_000)
            .map(|i| {
                let f = i as f32;
                WorldPos::new(
                    IVec3::new(i % 40, 0, i / 400),
                    Vec3::new((f * 7.31) % 256.0 - 128.0, 0.0, (f * 3.17) % 256.0 - 128.0),
                )
            })
            .collect();
        let mut out = vec![None; pts.len()];
        let t = Instant::now();
        let rounds = 20;
        for _ in 0..rounds {
            world.sample_many(&ws, black_box(&pts), &mut out);
        }
        let sample_us = t.elapsed().as_secs_f64() * 1e6 / (rounds * pts.len()) as f64;
        let wet = out.iter().filter(|s| s.is_some()).count();
        let (lo, hi) = out
            .iter()
            .flatten()
            .filter(|s| s.kind == WaterKind::Ocean)
            .fold((f64::MAX, f64::MIN), |(a, b), s| {
                (a.min(s.height), b.max(s.height))
            });
        println!(
            "wind {wind:>4} m/s, {count} cascade(s), {bins:>4} bins: build {build:.2?}, step {step_ms:.4} ms, sample {sample_us:.3} µs, \
             {wet} of {} wet, ocean height {lo:.2}..{hi:.2} m",
            pts.len()
        );
        assert!(
            step_ms <= 0.05 || count > 1,
            "twin step over budget: {step_ms} ms"
        );
        assert!(sample_us <= 2.0, "sample over budget: {sample_us} µs");
    }
}
