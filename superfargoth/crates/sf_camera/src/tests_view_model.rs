//! The view-model projection (first-person.md "View model", FP-2a), exactly.

use glam::camera::rh::proj::directx::perspective_infinite_reverse;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::{ComfortSettings, ViewModelProjection, fov_scale};

const EPS: f32 = 1e-5;

fn close(a: f32, b: f32, what: &str) {
    assert!((a - b).abs() <= EPS * (1.0 + a.abs().max(b.abs())), "{what}: {a} vs {b}");
}

fn close3(a: Vec3, b: Vec3, what: &str) {
    assert!(a.abs_diff_eq(b, EPS * (1.0 + a.abs().max_element().max(b.abs().max_element()))), "{what}: {a} vs {b}");
}

/// Bevy's camera projection (`PerspectiveProjection::get_clip_from_view`) and TAA/DLSS jitter
/// (`TemporalJitter::jitter_projection`, perspective case).
fn projection(fov_deg: f32, aspect: f32, near: f32, jitter: Vec2, view_size: Vec2) -> Mat4 {
    let mut m = perspective_infinite_reverse(fov_deg.to_radians(), aspect, near);
    let j = jitter * Vec2::new(2.0, -2.0) / view_size;
    m.z_axis.x += j.x;
    m.z_axis.y += j.y;
    m
}

/// A camera's `view_from_world` (Bevy: the inverse of its rigid `GlobalTransform`).
fn view(at: Vec3, rotation: Quat) -> Mat4 {
    Mat4::from_rotation_translation(rotation, at).inverse()
}

fn ndc(clip: Vec4) -> Vec3 {
    clip.xyz() / clip.w
}

fn views() -> [Mat4; 4] {
    [
        Mat4::IDENTITY,
        view(Vec3::new(3.0, 1.7, -2.0), Quat::from_rotation_y(0.7)),
        view(Vec3::new(-120.5, 40.0, 900.25), Quat::from_euler(glam::EulerRot::YXZ, 2.9, -0.6, 0.05)),
        glam::camera::rh::view::look_at_mat4(Vec3::new(5.0, 2.0, 5.0), Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ]
}

#[test]
fn fov_scale_is_the_tangent_ratio() {
    close(fov_scale(60f32.to_radians(), 50f32.to_radians()), 30f32.to_radians().tan() / 25f32.to_radians().tan(), "60/50");
    close(fov_scale(60f32.to_radians(), 50f32.to_radians()), 1.238_131_6, "60/50 value");
    assert!((fov_scale(1.2, 1.2) - 1.0).abs() < 1e-7, "equal FOVs: no scale");
    assert!(fov_scale(1.0, 1.5) < 1.0, "a wider view model is smaller");
    for (w, v) in [(0.0, 1.0), (1.0, 0.0), (std::f32::consts::PI, 1.0), (f32::NAN, 1.0), (1.0, -0.5)] {
        assert!((fov_scale(w, v) - 1.0).abs() < f32::EPSILON, "degenerate ({w}, {v}): no scale");
    }
    let p = ViewModelProjection::new(60.0, 50.0, 0.8);
    close(p.k, 1.238_131_6, "new takes degrees");
    assert!((p.depth - 0.8).abs() < f32::EPSILON);
    assert!((ViewModelProjection::new(60.0, 50.0, 1.5).depth - ViewModelProjection::MAX_DEPTH).abs() < f32::EPSILON);
    assert!(ViewModelProjection::new(60.0, 50.0, -1.0).depth.abs() < f32::EPSILON);
    assert!(ViewModelProjection::new(60.0, 50.0, f32::NAN).depth.abs() < f32::EPSILON);
    assert_eq!(ViewModelProjection::default(), ViewModelProjection::IDENTITY);
}

#[test]
fn settings_carry_the_view_model_off_by_default() {
    let s = ComfortSettings::default();
    assert!(!s.view_model, "off until the view-model material lands (FP-2b)");
    let p = s.view_model_projection();
    close(p.k, fov_scale(s.fov_world.to_radians(), s.fov_body.to_radians()), "k from fov_world/fov_body");
    assert!((p.depth - s.view_model_depth).abs() < f32::EPSILON);
    let same = ComfortSettings { fov_body: s.fov_world, ..s }.view_model_projection();
    assert!((same.k - 1.0).abs() < 1e-7, "fov_body = fov_world: the arms at the world's FOV");
}

/// In view space the pseudo-world position is the real one with x and y scaled by `k`, depth
/// kept; it equals the inverse-matrix form `world_from_view * S_k * view_from_world * p`.
#[test]
fn pseudo_world_scales_view_x_and_y_and_keeps_depth() {
    let p = ViewModelProjection::new(60.0, 50.0, 0.8);
    let points = [Vec3::new(0.3, -0.2, -0.5), Vec3::new(4.0, 2.5, -1.0), Vec3::new(-110.0, 39.0, 899.0)];
    for v in views() {
        let world_from_view = v.inverse();
        let s_k = Mat4::from_scale(Vec3::new(p.k, p.k, 1.0));
        for w in points {
            let pw = p.pseudo_world(w, v);
            let real = v.transform_point3(w);
            close3(v.transform_point3(pw), Vec3::new(real.x * p.k, real.y * p.k, real.z), "view space");
            close3(pw, (world_from_view * s_k * v).transform_point3(w), "inverse form");
        }
    }
    for v in views() {
        let w = Vec3::new(1.0, 2.0, 3.0);
        close3(ViewModelProjection::IDENTITY.pseudo_world(w, v), w, "k = 1: the real position");
    }
}

/// Through the camera's own (jittered) projection, the pseudo-world position lands where a
/// `fov_body` camera with the same aspect, near plane and jitter draws the real one: the view
/// model's own FOV in one render.
#[test]
fn clip_is_the_view_model_fov_with_the_cameras_jitter() {
    let (aspect, near, size) = (16.0 / 9.0, 0.1, Vec2::new(2560.0, 1440.0));
    let p = ViewModelProjection { depth: 0.0, ..ViewModelProjection::new(60.0, 50.0, 0.8) };
    for jitter in [Vec2::ZERO, Vec2::new(0.25, -0.375)] {
        let world_proj = projection(60.0, aspect, near, jitter, size);
        let body_proj = projection(50.0, aspect, near, jitter, size);
        for v in views() {
            let w = v.inverse().transform_point3(Vec3::new(0.25, -0.3, -0.6));
            let got = ndc(p.clip(w, v, world_proj * v));
            let want = ndc(body_proj * v * w.extend(1.0));
            close3(got, want, "view model = fov_body projection");
        }
    }
}

/// Reverse Z: `ndc' = R + (1 - R) ndc`; depth order kept, the near plane kept, and only
/// geometry closer than `near / R` can cover the view model.
#[test]
fn depth_slice_keeps_order_and_the_near_plane() {
    let near = 0.1;
    let proj = perspective_infinite_reverse(60f32.to_radians(), 1.5, near);
    let p = ViewModelProjection::new(60.0, 60.0, 0.8);
    let depth_at = |d: f32| ndc(p.clip(Vec3::new(0.0, 0.0, -d), Mat4::IDENTITY, proj)).z;
    let world_at = |d: f32| ndc(proj * Vec4::new(0.0, 0.0, -d, 1.0)).z;
    for d in [0.1, 0.3, 0.7, 2.0, 50.0] {
        close(depth_at(d), 0.8 + 0.2 * world_at(d), "ndc' = R + (1 - R) ndc");
    }
    assert!(depth_at(0.3) > depth_at(0.7), "nearer stays in front (reverse Z)");
    close(depth_at(near), 1.0, "the near plane stays the near plane");
    close(p.cover_distance(near), 0.125, "near / R");
    assert!(world_at(0.124) > depth_at(10.0), "geometry inside near / R can cover the arms");
    assert!(world_at(0.126) < depth_at(10.0), "geometry past near / R never covers them");
    assert!(world_at(0.126) < depth_at(0.126), "not even at equal distance");
    // Clipping (WebGPU: 0 <= z <= w) is unchanged: z' <= w exactly when z <= w.
    for z in [0.5f32, 1.0, 1.5] {
        let c = p.depth_slice(Vec4::new(0.0, 0.0, z, 1.0));
        assert_eq!(c.z <= c.w, z <= 1.0, "near clip at z = {z}");
    }
    assert_eq!(ViewModelProjection::IDENTITY.depth_slice(Vec4::new(1.0, 2.0, 0.3, 4.0)), Vec4::new(1.0, 2.0, 0.3, 4.0), "R = 0: none");
    assert!(ViewModelProjection::IDENTITY.cover_distance(near).is_infinite());
}

/// Bevy's stock prepass motion vector (`prepass.wesl` fragment: unjittered `clip_from_world`
/// times `world_position`, previous view times `previous_world_position`) fed the pseudo-world
/// positions of both frames equals the motion of the drawn (`fov_body`) image; an arm held
/// still in front of a turning, moving camera has none.
#[test]
fn stock_motion_vectors_from_pseudo_world_positions_match_the_drawn_motion() {
    let (aspect, near) = (16.0 / 9.0, 0.1);
    let p = ViewModelProjection::new(60.0, 50.0, 0.8);
    let unjittered = perspective_infinite_reverse(60f32.to_radians(), aspect, near);
    let body = perspective_infinite_reverse(50f32.to_radians(), aspect, near);
    let motion = |a: Vec4, b: Vec4| (ndc(a).truncate() - ndc(b).truncate()) * Vec2::new(0.5, -0.5);
    let (cam_prev, cam_now) = (
        (Vec3::new(0.0, 1.6, 0.0), Quat::from_rotation_y(0.0)),
        (Vec3::new(0.05, 1.62, -0.1), Quat::from_euler(glam::EulerRot::YXZ, 0.08, -0.03, 0.0)),
    );
    let (v_prev, v_now) = (view(cam_prev.0, cam_prev.1), view(cam_now.0, cam_now.1));
    // A hand point fixed in the camera frame, and one swinging in the world.
    let held = Vec3::new(0.2, -0.25, -0.5);
    let fixed = (v_prev.inverse().transform_point3(held), v_now.inverse().transform_point3(held));
    let swinging = (Vec3::new(0.3, 1.3, -0.4), Vec3::new(0.1, 1.45, -0.55));
    for (prev, now) in [fixed, swinging] {
        let stock =
            motion(unjittered * v_now * p.pseudo_world(now, v_now).extend(1.0), unjittered * v_prev * p.pseudo_world(prev, v_prev).extend(1.0));
        let drawn = motion(body * v_now * now.extend(1.0), body * v_prev * prev.extend(1.0));
        assert!(stock.abs_diff_eq(drawn, 1e-5), "stock {stock} vs drawn {drawn}");
    }
    let still =
        motion(unjittered * v_now * p.pseudo_world(fixed.1, v_now).extend(1.0), unjittered * v_prev * p.pseudo_world(fixed.0, v_prev).extend(1.0));
    assert!(still.abs_diff_eq(Vec2::ZERO, 1e-5), "an arm still on screen has no motion: {still}");
}
