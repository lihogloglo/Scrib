#![allow(clippy::cast_precision_loss, clippy::float_cmp)] // Small test counts, power-of-two map sizes, exact whole-meter values.

use super::*;

const MAP: u32 = 2048;

/// Bevy's perspective `get_frustum_corners` (fov 60 degrees, 16:9) for the
/// slice `near..far` meters in front of the camera.
fn slice(near: f32, far: f32, map_size: u32) -> SliceShape {
    let tan = (60f32.to_radians() / 2.0).tan();
    let aspect = 16.0 / 9.0;
    let plane = |d: f32| {
        let a = d * tan;
        let b = a * aspect;
        [Vec3A::new(b, -a, -d), Vec3A::new(b, a, -d), Vec3A::new(-b, a, -d), Vec3A::new(-b, -a, -d)]
    };
    let (n, f) = (plane(near), plane(far));
    SliceShape { corners: [n[0], n[1], n[2], n[3], f[0], f[1], f[2], f[3]], map_size }
}

fn sun() -> Quat {
    Quat::from_rotation_y(0.7) * Quat::from_rotation_x(-0.9)
}

fn view(position: Vec3, rotation: Quat, light: Quat) -> HoldView {
    HoldView { world_from_view: Mat4::from_rotation_translation(rotation, position), light_rotation: light }
}

fn start() -> HoldView {
    view(Vec3::new(1000.0, 30.0, -2000.0), Quat::from_rotation_y(0.3), sun())
}

/// Captures at `start()`, then returns the decision at `next`.
fn after_capture(config: &CascadeHoldConfig, next: &HoldView) -> HoldDecision {
    let s = slice(70.0, 205.0, MAP);
    let mut state = None;
    assert_eq!(step(config, &mut state, &start(), &s).1, HoldDecision::Redraw(Redraw::First));
    step(config, &mut state, next, &s).1
}

#[test]
fn first_capture_redraws_then_a_still_camera_holds() {
    let config = CascadeHoldConfig::default();
    let s = slice(70.0, 205.0, MAP);
    let mut state = None;
    let (drawn, first) = step(&config, &mut state, &start(), &s);
    assert_eq!(first, HoldDecision::Redraw(Redraw::First));
    let (held, second) = step(&config, &mut state, &start(), &s);
    assert_eq!(second, HoldDecision::Hold);
    assert_eq!(held, drawn, "a hold reuses the drawn fit");
    assert_eq!(state.map(|c| c.age), Some(1));
}

#[test]
fn moving_four_meters_releases() {
    let config = CascadeHoldConfig::default();
    let at = |d: f32| {
        let mut v = start();
        v.world_from_view.w_axis += Vec4::new(d, 0.0, 0.0, 0.0);
        v
    };
    assert_eq!(after_capture(&config, &at(3.9)), HoldDecision::Hold);
    assert_eq!(after_capture(&config, &at(4.1)), HoldDecision::Redraw(Redraw::Moved));
}

#[test]
fn turning_three_degrees_releases() {
    let config = CascadeHoldConfig::default();
    let turned = |deg: f32, axis: Vec3| {
        let rotation = Quat::from_axis_angle(axis, deg.to_radians()) * Quat::from_rotation_y(0.3);
        view(start().position(), rotation, sun())
    };
    for axis in [Vec3::Y, Vec3::X, Vec3::Z] {
        assert_eq!(after_capture(&config, &turned(2.9, axis)), HoldDecision::Hold, "{axis}");
        assert_eq!(after_capture(&config, &turned(3.1, axis)), HoldDecision::Redraw(Redraw::Turned), "{axis}");
    }
}

#[test]
fn sun_turning_releases() {
    let config = CascadeHoldConfig::default();
    let sun_by = |deg: f32| {
        let mut v = start();
        v.light_rotation = Quat::from_rotation_x(deg.to_radians()) * sun();
        v
    };
    assert_eq!(after_capture(&config, &sun_by(0.05)), HoldDecision::Hold);
    assert_eq!(after_capture(&config, &sun_by(0.2)), HoldDecision::Redraw(Redraw::SunMoved));
}

#[test]
fn max_age_redraws_every_n_frames() {
    let config = CascadeHoldConfig::default();
    let s = slice(70.0, 205.0, MAP);
    let mut state = None;
    let decisions: Vec<bool> = (0..9).map(|_| step(&config, &mut state, &start(), &s).1 == HoldDecision::Hold).collect();
    assert_eq!(decisions, [false, true, true, true, false, true, true, true, false]);
    let mut state = None;
    let s4 = step(&config, &mut state, &start(), &s);
    assert_eq!(s4.1, HoldDecision::Redraw(Redraw::First));
    for _ in 0..3 {
        step(&config, &mut state, &start(), &s);
    }
    assert_eq!(step(&config, &mut state, &start(), &s).1, HoldDecision::Redraw(Redraw::MaxAge));

    for max_age in [0, 1] {
        let config = CascadeHoldConfig { max_age, ..CascadeHoldConfig::default() };
        let mut state = None;
        assert!((0..4).all(|_| step(&config, &mut state, &start(), &s).1 != HoldDecision::Hold), "{max_age} never holds");
    }
}

#[test]
fn slice_shape_or_map_size_change_redraws() {
    let config = CascadeHoldConfig::default();
    let mut state = None;
    step(&config, &mut state, &start(), &slice(70.0, 205.0, MAP));
    assert_eq!(step(&config, &mut state, &start(), &slice(70.0, 210.0, MAP)).1, HoldDecision::Redraw(Redraw::Changed));
    assert_eq!(step(&config, &mut state, &start(), &slice(70.0, 210.0, 4096)).1, HoldDecision::Redraw(Redraw::Changed));
    assert_eq!(step(&config, &mut state, &start(), &slice(70.0, 210.0, 4096)).1, HoldDecision::Hold);
}

/// The padded fit holds the slice anywhere within the thresholds: every move
/// under 4 m combined with every turn under 3 degrees stays covered.
#[test]
fn padded_fit_covers_the_slice_within_the_thresholds() {
    let config = CascadeHoldConfig { max_age: u32::MAX, ..CascadeHoldConfig::default() };
    for (near, far) in [(0.1, 30.0), (24.0, 80.0), (70.0, 205.0), (180.0, 280.0)] {
        let s = slice(near, far, MAP);
        let mut captured = None;
        step(&config, &mut captured, &start(), &s);
        let fit = captured.expect("captured").fit;
        for i in 0..64 {
            let a = i as f32 * 0.7;
            let dir = Vec3::new(a.cos(), (a * 1.3).sin() * 0.5, a.sin()).normalize();
            let axis = Vec3::new((a * 0.5).sin(), (a * 0.9).cos(), (a * 1.7).sin()).normalize();
            let rotation = Quat::from_axis_angle(axis, 2.95f32.to_radians()) * Quat::from_rotation_y(0.3);
            let v = view(start().position() + dir * 3.95, rotation, sun());
            assert!(fit.covers(&world_corners(v.world_from_view, &s.corners)), "slice {near}..{far}, sample {i}");
            let mut state = captured;
            assert_eq!(step(&config, &mut state, &v, &s).1, HoldDecision::Hold, "slice {near}..{far}, sample {i}");
        }
    }
}

/// The pad comes from the thresholds at the draw: loosened live, the coverage
/// check is the safety net (a 60 m jump leaves a box padded for 1 m).
#[test]
fn leaving_the_held_box_redraws() {
    let tight = CascadeHoldConfig { move_threshold: 1.0, turn_threshold_deg: 0.0, max_age: u32::MAX, ..CascadeHoldConfig::default() };
    let loose = CascadeHoldConfig { move_threshold: 100.0, turn_threshold_deg: 3.0, ..tight };
    let s = slice(70.0, 205.0, MAP);
    let mut state = None;
    step(&tight, &mut state, &start(), &s);
    let mut v = start();
    v.world_from_view.w_axis += Vec4::new(60.0, 0.0, 0.0, 0.0);
    assert_eq!(step(&loose, &mut state, &v, &s).1, HoldDecision::Redraw(Redraw::Uncovered));
}

#[test]
fn pad_grows_the_texel_and_keeps_snapping() {
    let s = slice(70.0, 205.0, MAP);
    let light = Mat4::from_quat(sun());
    let camera = start().world_from_view;
    let bare = fit_cascade(&s.corners, MAP as f32, light, camera, 0.0);
    let pad = hold_pad(&CascadeHoldConfig::default(), &s.corners);
    let padded = fit_cascade(&s.corners, MAP as f32, light, camera, pad);
    let diameter = |f: &CascadeFit| f.texel_size * MAP as f32;
    assert_eq!(diameter(&bare), diameter(&bare).round(), "whole-meter diameter");
    assert_eq!(diameter(&padded), diameter(&padded).round(), "whole-meter diameter");
    assert!((diameter(&padded) - diameter(&bare) - 2.0 * pad).abs() <= 1.0);
    assert!(padded.texel_size > bare.texel_size);
    // The box centre sits on whole texels in light space.
    let centre = light.transpose().transform_point3(padded.world_from_cascade.w_axis.truncate());
    for c in [centre.x, centre.y] {
        let texels = c / padded.texel_size;
        assert!((texels - texels.round()).abs() < 1e-2, "{texels}");
    }
}

#[test]
fn rotation_angle_is_precise_for_small_angles() {
    let a = sun();
    let b = Quat::from_rotation_z(0.01f32.to_radians()) * a;
    assert!((rotation_angle(a, b).to_degrees() - 0.01).abs() < 1e-3);
    assert!(rotation_angle(a, a).abs() < 1e-7);
    assert!(rotation_angle(a, -a) < 1e-6, "q and -q are the same rotation");
}
