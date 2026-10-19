use super::*;
use bevy_math::{Affine3A, Quat};
use bevy_render::render_phase::ViewRangefinder3d;

fn sorted(center: Vec3) -> TransparentSortingInfo3d {
    TransparentSortingInfo3d::Sorted { mesh_center: center, depth_bias: 0.0 }
}

/// Water and precipitation both sit at the origin (shader-placed). A camera
/// 1 km away, at any yaw and pitch, must still draw the water, then the
/// world-placed translucents, then the precipitation boxes far to near.
#[test]
fn bands_order_water_translucents_overlays_for_any_view() {
    let boxes = [DrawBand::Overlay { order: 0 }, DrawBand::Overlay { order: -1 }, DrawBand::Overlay { order: -2 }];
    let eye = Vec3::new(-234.0, 12.0, 1053.0);
    for yaw in 0..8_u8 {
        for pitch in [-1.2_f32, -0.3, 0.0, 0.4, 1.2] {
            let rotation = Quat::from_rotation_y(f32::from(yaw) * 0.785) * Quat::from_rotation_x(pitch);
            let range = ViewRangefinder3d::from_world_from_view(&Affine3A::from_rotation_translation(rotation, eye));
            let key = |band: DrawBand, center: Vec3| band.sorting_info(sorted(center), eye).sort_distance(&range);
            // The water's and the boxes' AABB centres as built (grid space, seed space).
            let water = key(DrawBand::Refractive, Vec3::new(0.0, 4.0, 0.0));
            let drawn: Vec<f32> = boxes.iter().map(|band| key(*band, Vec3::splat(0.5))).collect();
            let near = [eye + rotation * Vec3::NEG_Z * 30.0, eye + Vec3::new(2000.0, -50.0, 0.0), eye + rotation * Vec3::Z * 5.0, Vec3::ZERO];
            for p in near {
                let t = key(DrawBand::Sorted, p);
                assert!(water < t, "water after a translucent (yaw {yaw}, pitch {pitch})");
                assert!(drawn.iter().all(|x| t < *x), "precip before a translucent (yaw {yaw}, pitch {pitch})");
            }
            assert!(drawn[2] < drawn[1] && drawn[1] < drawn[0], "far box first, near box last");
        }
    }
}

/// The bug this crate fixes: with Bevy's own keys (AABB centres: water
/// (0, 4, 0) in grid space, rain (0.5, 0.5, 0.5) in seed space, both at the
/// origin), looking down at the sea draws the rain *before* the water, which
/// then overwrites it; looking up draws it after.
#[test]
fn bevy_keys_alone_flip_water_and_rain_with_pitch() {
    let eye = Vec3::new(-234.0, 12.0, 1053.0);
    let order = |pitch: f32| {
        let range = ViewRangefinder3d::from_world_from_view(&Affine3A::from_rotation_translation(Quat::from_rotation_x(pitch), eye));
        sorted(Vec3::splat(0.5)).sort_distance(&range) > sorted(Vec3::new(0.0, 4.0, 0.0)).sort_distance(&range)
    };
    assert!(!order(-0.3), "looking down: rain drawn first, water erases it");
    assert!(order(0.3), "looking up: rain drawn last");
}

#[test]
fn sorted_band_keeps_bevys_key() {
    let info = TransparentSortingInfo3d::Sorted { mesh_center: Vec3::new(1.0, 2.0, 3.0), depth_bias: 0.5 };
    let range = ViewRangefinder3d::from_world_from_view(&Affine3A::IDENTITY);
    let ours = DrawBand::Sorted.sorting_info(info, Vec3::ZERO).sort_distance(&range);
    assert!((ours - info.sort_distance(&range)).abs() < 1e-6);
}
