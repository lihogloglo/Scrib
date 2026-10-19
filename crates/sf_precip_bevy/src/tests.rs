#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn drop_mesh_has_four_vertices_per_drop_and_even_ranks() {
    let mesh = drop_mesh(1000, 1);
    assert_eq!(mesh.count_vertices(), 4000);
    let Some(bevy_mesh::VertexAttributeValues::Float32x4(drops)) = mesh.attribute(ATTRIBUTE_DROP) else {
        panic!("drop attribute");
    };
    // Ranks spread evenly: about 30 % of drops show at intensity 0.3.
    let shown = drops.iter().step_by(4).filter(|d| d[0] < 0.3).count();
    assert!((295..=305).contains(&shown), "{shown}");
}

#[test]
fn offset_wraps_and_follows_the_wind() {
    let slot = PrecipSlot { fall_speed: 7.0, ..PrecipSlot::default() };
    let v = velocity(&slot, Vec2::new(3.0, -1.0));
    assert_eq!(v, Vec3::new(3.0, -7.0, -1.0));
    let mut o = Vec3::ZERO;
    for _ in 0..1000 {
        o = advance(o, v, 1.0 / 60.0, 8.0);
        assert!(o.cmpge(Vec3::ZERO).all() && o.cmplt(Vec3::splat(8.0)).all(), "{o}");
    }
    // 1000 frames at 60 Hz: 50 m of x travel, 116.7 m of fall, wrapped.
    assert!((o.x - (50.0_f32).rem_euclid(8.0)).abs() < 1e-2, "{o}");
}

#[test]
fn layer_params_nest_and_light() {
    let flake = PrecipSlot {
        shape: PrecipShape::Flake,
        intensity: 1.5,
        flutter: 0.2,
        color: Vec4::new(1.0, 1.0, 1.0, 0.8),
        ..PrecipSlot::default()
    };
    let precip = Precip { brightness: 0.5, ..Precip::default() };
    let p0 = layer_params(&flake, 0, &precip, Vec3::ONE, 2.0);
    let p2 = layer_params(&flake, 2, &precip, Vec3::ONE, 2.0);
    assert_eq!(p0.volume, Vec4::new(8.0, 0.0, STREAK_S, 1.0));
    assert_eq!(p2.volume.y, 12.0, "far box skips the mid box");
    assert_eq!(p0.look.z, 1.0, "intensity clamped");
    assert_eq!(p0.color.truncate(), Vec3::splat(0.5));
    assert!(p2.color.w < p0.color.w, "far box more transparent");
    assert_eq!(p0.offset.w, 2.0);
    assert_eq!(p0.velocity.w, 0.2);
}
