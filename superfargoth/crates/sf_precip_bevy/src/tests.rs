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

#[test]
fn occlusion_view_follows_the_travel_direction() {
    use super::occlusion::{along, travel, view_dir, MIN_ELEVATION};
    // Rain falling straight down: the old top-down map (eye above, depth 1 at the top).
    assert_eq!(view_dir(Vec3::new(0.0, -8.0, 0.0)), Vec3::NEG_Y);
    assert_eq!(view_dir(Vec3::ZERO), Vec3::NEG_Y);
    let center = Vec3::new(10.0, 5.0, -3.0);
    let (_, clip) = along(center, Vec3::NEG_Y);
    let roof = clip.project_point3(center + Vec3::Y * 4.0);
    let drop = clip.project_point3(center);
    assert!(roof.truncate().distance(drop.truncate()) < 1e-5 && roof.z > drop.z, "{roof} {drop}");
    // Slanted rain (8 m/s down, 3 m/s wind) keeps its own direction.
    let rain = view_dir(Vec3::new(3.0, -8.0, 0.0));
    assert!((rain - Vec3::new(3.0, -8.0, 0.0).normalize()).length() < 1e-6, "{rain}");
    // An ash storm (1 m/s down, 15 m/s wind along +x) blows near flat: the
    // view is lowered to MIN_ELEVATION and a wall upwind shelters the drop,
    // while the same wall straight above it would not.
    let ash = view_dir(Vec3::new(15.0, -1.0, 0.0));
    assert!((ash.y + MIN_ELEVATION.sin()).abs() < 1e-6 && ash.x > 0.9 && ash.z.abs() < 1e-6, "{ash}");
    let (_, clip) = along(center, ash);
    let drop = clip.project_point3(center);
    let wall = clip.project_point3(center - ash * 20.0);
    assert!(wall.truncate().distance(drop.truncate()) < 1e-4 && wall.z > drop.z, "{wall} {drop}");
    let above = clip.project_point3(center + Vec3::Y * 4.0);
    assert!(above.truncate().distance(drop.truncate()) > 0.05, "{above} {drop}");
    // The whole far box (64 m) fits the map's square and depth range.
    for corner in [Vec3::new(32.0, 32.0, 32.0), Vec3::new(-32.0, -32.0, 32.0), Vec3::new(32.0, -32.0, -32.0)] {
        for dir in [Vec3::NEG_Y, ash, rain] {
            let c = along(center, dir).1.project_point3(center + corner);
            assert!(c.x.abs() <= 1.0 && c.y.abs() <= 1.0 && c.z > 0.0 && c.z <= 1.0, "{dir} {corner} {c}");
        }
    }
    // The strongest falling slot steers the view.
    let ash_slot = PrecipSlot { fall_speed: 1.0, intensity: 0.9, ..PrecipSlot::default() };
    let rain_slot = PrecipSlot { fall_speed: 8.0, intensity: 0.2, wind_follow: 0.0, ..PrecipSlot::default() };
    let precip = Precip { slots: [Some(rain_slot), Some(ash_slot)], wind: Vec2::new(15.0, 0.0), ..Precip::default() };
    assert_eq!(travel(&precip), Some(ash));
    assert_eq!(travel(&Precip { enabled: false, ..precip }), None);
}
