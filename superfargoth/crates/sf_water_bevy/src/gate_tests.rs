//! The visibility gate (CW-10a): the frustum term per camera, the FFT plan, the probe's shader side.
#![allow(clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_sign_loss, deprecated)]

use std::sync::Arc;

use bevy_camera::primitives::Frustum;
use bevy_math::{DVec2, Mat4, Vec2, Vec3};
use sf_water::{ShoreField, ShoreSettings, WetMask, WetTiles};

use crate::gate::{camera_gate, Body, FftClock, FftPlan};
use crate::{ClipmapSettings, WaterFrame, WaterGateSettings, WaterLook, WaterShoreInput, WaterSurface};

/// A camera at `eye` looking along `dir` (60° vertical FOV, reverse-Z infinite like Bevy's).
fn frustum(eye: Vec3, dir: Vec3) -> Frustum {
    let up = if dir.normalize().y.abs() > 0.99 { Vec3::Z } else { Vec3::Y };
    let view = Mat4::look_to_rh(eye, dir, up);
    let proj = Mat4::perspective_infinite_reverse_rh(60f32.to_radians(), 16.0 / 9.0, 0.1);
    Frustum(<<Frustum as std::ops::Deref>::Target>::from_clip_from_world(&(proj * view)))
}

fn on() -> WaterGateSettings {
    WaterGateSettings { enabled: true, ..WaterGateSettings::default() }
}

fn frame() -> WaterFrame {
    WaterFrame { enabled: true, ..WaterFrame::default() }
}

/// A 512 × 256 m field at 4 m texels from world (0, 0): land, with a 12 m water strip along x = 0.
fn coast() -> WaterShoreInput {
    let (w, h, texel) = (128, 64, 4.0);
    let mut mask = WetMask::new(w, h, texel, DVec2::splat(texel * 0.5));
    for z in 0..h {
        for x in 0..w {
            mask.wet[z * w + x] = x < 3;
        }
    }
    WaterShoreInput { field: Some(Arc::new(ShoreField::bake(&mask, ShoreSettings::default()))) }
}

fn gate_at(shore: &WaterShoreInput, eye: Vec3, dir: Vec3, bodies: &[Body]) -> crate::WaterGate {
    let s = on();
    let tiles = shore.field.as_ref().map(|f| WetTiles::from_field(f, -s.margin_m));
    camera_gate(&s, &frame(), &ClipmapSettings::default(), shore, tiles.as_ref(), eye, &frustum(eye, dir), bodies)
}

/// No water in view: looking at the sky over the open sea, or straight down on land well inside
/// the field, the frustum term closes (no box, not forced).
#[test]
fn no_water_in_view_closes_the_frustum_term() {
    let open = WaterShoreInput::default();
    let sky = gate_at(&open, Vec3::new(0.0, 100.0, 0.0), Vec3::new(0.2, 1.0, 0.0), &[Body::Ocean]);
    assert!(!sky.in_frustum(), "{sky:?}");
    // The view's footprint at the slab's floor (136 m below, 91° × 60°): x 131-409 m, z 50-206 m.
    let land = gate_at(&coast(), Vec3::new(270.0, 120.0, 128.0), Vec3::NEG_Y, &[Body::Ocean]);
    assert!(land.boxes.is_empty() && !land.forced, "{:?}", land.boxes);
}

/// Water in view: the coast's strip below the camera, the open sea at the horizon, a bounded lake;
/// all give boxes, none forced from 200 m up.
#[test]
fn water_in_view_opens_it() {
    let coast = coast();
    let strip = gate_at(&coast, Vec3::new(40.0, 200.0, 128.0), Vec3::NEG_Y, &[Body::Ocean]);
    assert!(strip.in_frustum() && !strip.forced);
    assert!(strip.boxes.iter().all(|[lo, hi]| lo.y == -16.0 && hi.y == 16.0));
    let horizon = gate_at(&coast, Vec3::new(300.0, 200.0, 128.0), Vec3::X, &[Body::Ocean]);
    assert!(horizon.in_frustum(), "the open sea past x = 512 m is in view");
    let lake = WaterSurface { level: 50.0, bounds: Some([Vec2::new(280.0, 100.0), Vec2::new(320.0, 150.0)]), swell: 0.0, chop: 0.1, look: WaterLook::default() };
    let g = gate_at(&WaterShoreInput::default(), Vec3::new(300.0, 200.0, 128.0), Vec3::NEG_Y, &[Body::Surface(lake)]);
    assert_eq!(g.boxes.len(), 1);
    // The ocean not shown (an interior): no ocean box, whatever the view.
    assert!(!gate_at(&coast, Vec3::new(40.0, 200.0, 128.0), Vec3::NEG_Y, &[]).in_frustum());
}

/// The camera inside a water box (standing on the shore, swimming) skips the probe: every pass runs.
#[test]
fn camera_inside_a_box_forces_every_pass() {
    let g = gate_at(&coast(), Vec3::new(6.0, 1.7, 128.0), Vec3::X, &[Body::Ocean]);
    assert!(g.forced && g.in_frustum());
    let pool = WaterSurface::pool(-1.0, WaterLook::default());
    assert!(gate_at(&WaterShoreInput::default(), Vec3::new(0.0, 1.7, 0.0), Vec3::NEG_Y, &[Body::Surface(pool)]).forced);
}

/// Gate off: the FFT runs every frame, amortised, copying `prev` first (as before CW-10a).
#[test]
fn gate_off_is_todays_fft() {
    let mut c = FftClock::default();
    for i in 0..10 {
        assert_eq!(c.plan(false, i % 2 == 0, f64::from(i) / 60.0, 0.125), FftPlan { run: true, full: false, copy_before: true });
    }
}

/// Gate on: in view every frame (amortised after the first); out of view at the idle period with
/// every cascade; back in view, every cascade with `prev` copied after the FFT (no stale motion).
#[test]
fn fft_idles_out_of_view_and_reprimes_on_reentry() {
    let mut c = FftClock::default();
    let dt = 1.0 / 120.0;
    let mut t = 0.0;
    let mut step = |open: bool| {
        t += dt;
        c.plan(true, open, t, 0.125)
    };
    assert_eq!(step(true), FftPlan { run: true, full: true, copy_before: false });
    assert_eq!(step(true), FftPlan { run: true, full: false, copy_before: true });
    // Out of view for one second: ~8 full runs, none back to back.
    let runs: Vec<FftPlan> = (0..120).map(|_| step(false)).filter(|p| p.run).collect();
    assert!((7..=9).contains(&runs.len()), "{} idle runs", runs.len());
    assert!(runs.iter().all(|p| p.full && !p.copy_before));
    // Re-entry after a frame without FFT.
    let back = step(true);
    assert_eq!(back, FftPlan { run: true, full: true, copy_before: false });
    assert_eq!(step(true), FftPlan { run: true, full: false, copy_before: true });
}

/// The probe's triangle strip (`water_gate.wesl` `strip_corner`) closes the unit cube: its 12
/// non-degenerate triangles lie on the 6 faces, two per face, so a box is covered from any side.
#[test]
fn probe_strip_covers_the_cube() {
    let corner = |v: u32| {
        let b = 1u32 << v;
        Vec3::new(f32::from(u8::from(0x287a & b != 0)), f32::from(u8::from(0x02af & b != 0)), f32::from(u8::from(0x31e3 & b != 0)))
    };
    let mut faces = std::collections::HashMap::new();
    for i in 0..crate::render::GATE_STRIP - 2 {
        let [a, b, c] = [corner(i), corner(i + 1), corner(i + 2)];
        let n = (b - a).cross(c - a);
        assert!(n.length() > 0.5, "degenerate triangle {i}");
        // On a face: one axis constant at 0 or 1.
        let axis = (0..3).find(|k| a[*k] == b[*k] && b[*k] == c[*k]).expect("triangle off the cube's faces");
        *faces.entry((axis, a[axis] as u8)).or_insert(0) += 1;
    }
    assert_eq!(faces.len(), 6);
    assert!(faces.values().all(|n| *n == 2), "{faces:?}");
    let wgsl = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/shaders/water_gate.wesl")).expect("shader");
    for mask in ["0x287au", "0x02afu", "0x31e3u"] {
        assert!(wgsl.contains(mask), "strip mask {mask} differs from the shader");
    }
}

/// The switch flips live both ways: on, each `WaterCamera` carries its `WaterGate` (closed while it
/// looks at the sky); off, the component goes and every pass runs as before CW-10a.
#[test]
fn switch_flips_live_both_ways() {
    use bevy_ecs::prelude::*;
    use bevy_transform::components::GlobalTransform;
    let mut world = World::new();
    world.insert_resource(WaterGateSettings::default());
    world.insert_resource(frame());
    world.insert_resource(ClipmapSettings::default());
    world.insert_resource(WaterShoreInput::default());
    world.init_resource::<crate::gate::WetTilesCache>();
    world.spawn((crate::OceanSurface, bevy_camera::visibility::InheritedVisibility::VISIBLE));
    let eye = Vec3::new(0.0, 100.0, 0.0);
    let camera = world.spawn((crate::WaterCamera, GlobalTransform::from_translation(eye), frustum(eye, Vec3::new(0.2, 1.0, 0.0)))).id();
    let mut schedule = Schedule::default();
    schedule.add_systems((crate::gate::update_tiles, crate::gate::gate_views).chain());
    schedule.run(&mut world);
    assert!(world.get::<crate::WaterGate>(camera).is_none(), "off by default");
    world.resource_mut::<WaterGateSettings>().enabled = true;
    schedule.run(&mut world);
    let gate = world.get::<crate::WaterGate>(camera).expect("on: the camera is gated");
    assert!(!gate.in_frustum(), "looking at the sky");
    world.resource_mut::<WaterGateSettings>().enabled = false;
    schedule.run(&mut world);
    assert!(world.get::<crate::WaterGate>(camera).is_none(), "off again");
}
