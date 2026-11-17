//! The camera medium (water.md "Underwater", slice U1): exact state, body, depth and cues.
#![allow(clippy::float_cmp)]

use glam::{DVec2, IVec3, Quat, Vec3};
use sf_coords::WorldPos;
use sf_world::{Worldspace, WorldspaceId};

use crate::*;

const H: f32 = 0.01; // the hysteresis tunable's default (water.medium.hysteresis_m)

/// A flat sea at world Y `level`.
fn calm(level: f64) -> impl Fn(WorldPos) -> Option<WaterSample> {
    move |_| Some(WaterSample { body: "sea".into(), kind: WaterKind::Ocean, height: level, normal: Vec3::Y, velocity: Vec3::ZERO, depth: f32::INFINITY })
}

fn at(x: f32, y: f32, z: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, y, z))
}

/// A level camera looking along −Z: near 0.1 m, 60° vertical, 16:9 (the near plane is ±5.8 cm tall).
fn camera(eye: WorldPos) -> NearPlane {
    NearPlane::perspective(eye, Quat::IDENTITY, 0.1, 60f32.to_radians(), 16.0 / 9.0)
}

#[test]
fn all_below_is_under_and_says_underwater() {
    let mut m = CameraMedium::default();
    let cues = m.update(&camera(at(0.0, -1.0, 0.0)), calm(0.0), H);
    assert_eq!(m.state, MediumState::Under);
    assert_eq!(m.body, Some("sea".into()));
    assert_eq!(m.kind, Some(WaterKind::Ocean));
    assert_eq!(m.depth, 1.0);
    assert_eq!(m.column, f32::INFINITY);
    assert!(m.submerged && m.changed);
    // The first frame starts under: `underwater`, but no crossing.
    assert_eq!(cues, MediumCues { underwater: Some(true), surface_break: None });
    // The switch off: above at once, `underwater` left.
    let off = m.force_above();
    assert_eq!(off, MediumCues { underwater: Some(false), surface_break: None });
    assert_eq!((m.state, m.changed, m.submerged, m.body.clone()), (MediumState::Above, true, false, None));
}

#[test]
fn straddling_a_calm_surface_is_split() {
    let mut m = CameraMedium::default();
    let cues = m.update(&camera(at(0.0, 0.0, 0.0)), calm(0.0), H);
    assert_eq!(m.state, MediumState::Split);
    assert_eq!(m.depth, 0.0);
    assert!(m.changed && !m.submerged);
    assert_eq!(cues, MediumCues::default());
    // 3 cm down: the top corners (+5.8 cm) are still out, the eye is under.
    let cues = m.update(&camera(at(0.0, -0.03, 0.0)), calm(0.0), H);
    assert_eq!(m.state, MediumState::Split);
    assert!(!m.changed && m.submerged);
    assert_eq!(cues, MediumCues { underwater: Some(true), surface_break: Some(true) });
}

#[test]
fn all_above_is_above() {
    let mut m = CameraMedium::default();
    let cues = m.update(&camera(at(0.0, 1.0, 0.0)), calm(0.0), H);
    assert_eq!(m.state, MediumState::Above);
    assert_eq!(m.depth, -1.0);
    assert_eq!(m.body, Some("sea".into()));
    assert!(!m.changed && !m.submerged);
    assert_eq!(cues, MediumCues::default());
}

#[test]
fn no_water_is_above() {
    let mut m = CameraMedium::default();
    let cues = m.update(&camera(at(0.0, -50.0, 0.0)), |_| None, H);
    assert_eq!(m.state, MediumState::Above);
    assert_eq!((m.body.clone(), m.kind), (None, None));
    assert_eq!(m.depth, f32::NEG_INFINITY);
    assert_eq!(m.column, 0.0);
    assert!(!m.changed);
    assert_eq!(cues, MediumCues::default());
}

/// A point riding the surface: a ±5 mm wobble never flips the state; 1.5 cm past it does, once.
#[test]
fn hysteresis_holds_a_one_centimetre_band() {
    let mut m = CameraMedium::default();
    let mut ride = |y: f32| {
        let cues = m.update(&NearPlane::point(at(0.0, y, 0.0)), calm(0.0), H);
        (m.state, m.changed, cues)
    };
    assert_eq!(ride(0.02).0, MediumState::Above);
    for _ in 0..10 {
        for y in [-0.005, 0.005] {
            assert_eq!(ride(y), (MediumState::Above, false, MediumCues::default()), "wobble at {y}");
        }
    }
    assert_eq!(ride(-0.015), (MediumState::Under, true, MediumCues { underwater: Some(true), surface_break: Some(true) }));
    for _ in 0..10 {
        for y in [0.005, -0.005] {
            assert_eq!(ride(y), (MediumState::Under, false, MediumCues::default()), "wobble at {y}");
        }
    }
    assert_eq!(ride(0.015), (MediumState::Above, true, MediumCues { underwater: Some(false), surface_break: Some(false) }));
}

/// A swell (0.5 m, 40 m, 8 s) lifts its surface through a level camera at the rest level and lets
/// it down again: Above → Split → Under → Split → Above, one change per transition, the eye
/// crossing once each way.
#[test]
fn a_wave_carries_the_surface_through_the_lens() {
    let (amp, wavenumber, omega) = (0.5, std::f64::consts::TAU / 40.0, std::f64::consts::TAU / 8.0);
    let plane = camera(at(0.0, 0.0, 0.0));
    let mut cam = CameraMedium::default();
    let mut states = Vec::new();
    let mut changes = 0;
    let mut underwater = Vec::new();
    let mut breaks = Vec::new();
    for frame in 0..=480 {
        let time = f64::from(frame) / 60.0;
        let wave = move |p: WorldPos| {
            let x = p.to_f64().x;
            Some(WaterSample { body: "sea".into(), kind: WaterKind::Ocean, height: -amp * (omega * time - wavenumber * x).cos(), normal: Vec3::Y, velocity: Vec3::ZERO, depth: f32::INFINITY })
        };
        let cues = cam.update(&plane, wave, H);
        if frame == 0 {
            states.push(cam.state);
            assert!(!cam.changed);
        } else if cam.changed {
            changes += 1;
            states.push(cam.state);
        }
        underwater.extend(cues.underwater);
        breaks.extend(cues.surface_break);
    }
    assert_eq!(states, [MediumState::Above, MediumState::Split, MediumState::Under, MediumState::Split, MediumState::Above]);
    assert_eq!(changes, 4);
    assert_eq!(underwater, [true, false]);
    assert_eq!(breaks, [true, false]);
}

fn pool(center: DVec2, half: f64, level: f64) -> WaterBody {
    WaterBody {
        id: "pool".into(),
        worldspace: WorldspaceId::new("w"),
        kind: WaterKind::Pool,
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
        priority: 10,
        optics: None,
        waves: WaveTerms::for_kind(WaterKind::Pool),
        flow: None,
    }
}

/// A pool above the sea level (a cave or an interior's water at its own height): a camera in it is
/// under the pool, not above the sea; beside it, the sea is the body.
#[test]
fn a_pool_over_the_sea_is_the_cameras_body() {
    let ws = WorldspaceId::new("w");
    let mut w = Worldspace::new("w");
    w.has_terrain = true;
    w.sea_level = Some(0.0);
    let water = WorldspaceWater::new(vec![WaterBody::from_worldspace(&ws, &w).unwrap(), pool(DVec2::new(100.0, 0.0), 10.0, 5.0)]);
    let sample = |p| water.sample(p);
    let mut m = CameraMedium::default();
    m.update(&camera(at(100.0, 4.0, 0.0)), sample, H);
    assert_eq!((m.state, m.body.clone(), m.kind), (MediumState::Under, Some("pool".into()), Some(WaterKind::Pool)));
    assert_eq!(m.depth, 1.0);
    let mut m = CameraMedium::default();
    m.update(&camera(at(200.0, 4.0, 0.0)), sample, H);
    assert_eq!((m.state, m.body.clone(), m.kind), (MediumState::Above, Some("w#water".into()), Some(WaterKind::Ocean)));
    assert_eq!(m.depth, -4.0);
}

#[test]
fn optics_index_is_the_nearest_cell() {
    let map = OpticsMap { width: 2, height: 2, cell: 100.0, origin: DVec2::new(50.0, 50.0), index: vec![1, 2, 3, 4] };
    assert_eq!(optics_index_at(&map, DVec2::new(40.0, 60.0)), 1);
    assert_eq!(optics_index_at(&map, DVec2::new(160.0, 20.0)), 2);
    assert_eq!(optics_index_at(&map, DVec2::new(30.0, 199.0)), 3);
    assert_eq!(optics_index_at(&map, DVec2::new(140.0, 140.0)), 4);
    assert_eq!(optics_index_at(&map, DVec2::new(-60.0, 50.0)), 0);
    assert_eq!(optics_index_at(&map, DVec2::new(50.0, 400.0)), 0);
}
