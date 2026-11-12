//! Screen-space planar reflections, the CPU side (water.md "Water pass, refraction, reflections";
//! Cichocki, "Optimized pixel-projected reflections for planar reflectors", SIGGRAPH 2017):
//! which water plane the frame mirrors, and the reference math of the GPU passes
//! (`sf_water_bevy` `water_sspr.wesl`), so it is tested without a GPU.
//!
//! The projection pass mirrors every opaque pixel above the plane to its mirror pixel and keeps,
//! per target pixel, the candidate with the **smallest height above the plane**: every candidate
//! of one target lies on the same reflected ray, whose height grows with distance once it leaves
//! the plane, so the lowest is the nearest (the occluder in the mirror). The key is the height's
//! float bits (positive floats order like their bits), so one 32-bit atomic resolves the
//! visibility exactly at any resolution; Cichocki's screen-Y key needs an upright camera, and
//! packing source coordinates beside a depth leaves too few bits at 1440p and above. The resolve
//! pass rebuilds the source point from the target pixel's own ray and that height
//! ([`point_on_reflected_ray`]), then reads its colour.

use glam::{Mat4, Vec2, Vec3, Vec4Swizzles};

/// A water body the plane can come from: its rest level (render-space Y) and, for a bounded
/// body, its render-space XZ rectangle (min, max).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneCandidate {
    pub level: f32,
    pub bounds: Option<[Vec2; 2]>,
}

/// Sources lower than this above the plane are not mirrored (the shore line itself, and the
/// bed seen through the water, which is below it).
pub const MIN_HEIGHT: f32 = 0.01;

/// The level SSPR mirrors this frame: the nearest body below the camera (distance from the camera
/// to the body's rectangle at its level; an unbounded body, the ocean or an interior's water, is
/// at the vertical distance). `None` when the camera is under every body's level (underwater: the
/// environment map only).
#[must_use]
pub fn reflection_plane(camera: Vec3, bodies: &[PlaneCandidate]) -> Option<f32> {
    bodies
        .iter()
        .filter(|b| b.level < camera.y)
        .map(|b| {
            let xz = Vec2::new(camera.x, camera.z);
            let nearest = b.bounds.map_or(xz, |[lo, hi]| xz.clamp(lo, hi));
            let d = Vec3::new(nearest.x - xz.x, b.level - camera.y, nearest.y - xz.y).length();
            (d, b.level)
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, level)| level)
}

/// `p` mirrored in the plane `y = level`.
#[must_use]
pub fn mirror(p: Vec3, level: f32) -> Vec3 {
    Vec3::new(p.x, 2.0 * level - p.y, p.z)
}

/// The projection pass's key of a source `height` above the plane: larger wins under the GPU's
/// `atomicMax` (0 = empty), so the lowest source wins.
#[must_use]
pub fn height_key(height: f32) -> u32 {
    u32::MAX - height.max(0.0).to_bits()
}

/// The height a key stores (`None`: empty).
#[must_use]
pub fn key_height(key: u32) -> Option<f32> {
    (key != 0).then(|| f32::from_bits(u32::MAX - key))
}

/// Viewport UV (0..1, y down) of world point `p` under `clip_from_world`; `None` behind the
/// camera or off screen.
#[must_use]
pub fn project_uv(clip_from_world: Mat4, p: Vec3) -> Option<Vec2> {
    let clip = clip_from_world * p.extend(1.0);
    if clip.w <= 1e-6 {
        return None;
    }
    let ndc = clip.xy() / clip.w;
    let uv = Vec2::new(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    (uv.cmpge(Vec2::ZERO).all() && uv.cmple(Vec2::ONE).all()).then_some(uv)
}

/// Where the projection pass writes source point `p`: the viewport UV of its mirror image, with
/// its key; `None` for a source at or below the plane, or a mirror image off screen.
#[must_use]
pub fn mirror_target(clip_from_world: Mat4, p: Vec3, level: f32) -> Option<(Vec2, u32)> {
    let height = p.y - level;
    if height < MIN_HEIGHT {
        return None;
    }
    project_uv(clip_from_world, mirror(p, level)).map(|uv| (uv, height_key(height)))
}

/// The resolve's source point: the camera ray `dir` (from `camera`, above the plane) meets the
/// plane, reflects, and climbs to `height` above it. `None` when the ray never reaches the plane.
#[must_use]
pub fn point_on_reflected_ray(camera: Vec3, dir: Vec3, level: f32, height: f32) -> Option<Vec3> {
    if dir.y >= -1e-5 || camera.y <= level {
        return None;
    }
    let hit = camera + dir * ((level - camera.y) / dir.y);
    let up = Vec3::new(dir.x, -dir.y, dir.z);
    Some(hit + up * (height / up.y))
}

/// The direction whose flat-plane reflection is `reflected` (`(x, -y, z)`): the water draw looks
/// the wave-perturbed reflection up at the screen pixel of this direction.
#[must_use]
pub fn lookup_direction(reflected: Vec3) -> Vec3 {
    Vec3::new(reflected.x, -reflected.y, reflected.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camera() -> (Vec3, Mat4) {
        let eye = Vec3::new(3.0, 12.0, -4.0);
        let view = glam::camera::rh::view::look_at_mat4(eye, Vec3::new(40.0, 2.0, 60.0), Vec3::Y);
        let proj = glam::camera::rh::proj::directx::perspective_infinite_reverse(1.0, 16.0 / 9.0, 0.1);
        (eye, proj * view)
    }

    #[test]
    fn plane_is_the_nearest_body_below_the_camera() {
        let sea = PlaneCandidate { level: 0.0, bounds: None };
        let lake = PlaneCandidate { level: 40.0, bounds: Some([Vec2::new(100.0, 100.0), Vec2::new(200.0, 160.0)]) };
        let pool = PlaneCandidate { level: 70.0, bounds: Some([Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)]) };
        // Beside the lake, 5 m above it: the lake (5 m) beats the sea (45 m).
        assert_eq!(reflection_plane(Vec3::new(150.0, 45.0, 130.0), &[sea, lake]), Some(40.0));
        // Far from the lake at the same height: the sea is nearer.
        assert_eq!(reflection_plane(Vec3::new(-900.0, 45.0, 130.0), &[sea, lake]), Some(0.0));
        // A body above the camera is never mirrored; under every body: none.
        assert_eq!(reflection_plane(Vec3::new(0.0, 50.0, 0.0), &[sea, pool]), Some(0.0));
        assert_eq!(reflection_plane(Vec3::new(0.0, -2.0, 0.0), &[sea]), None);
        assert_eq!(reflection_plane(Vec3::ZERO, &[]), None);
    }

    #[test]
    fn keys_order_lowest_source_first_and_round_trip() {
        let heights = [0.01_f32, 0.5, 1.0, 3.25, 80.0, 2000.0];
        for pair in heights.windows(2) {
            assert!(height_key(pair[0]) > height_key(pair[1]), "{pair:?}");
        }
        for h in heights {
            assert_eq!(key_height(height_key(h)), Some(h));
            assert_ne!(height_key(h), 0, "a key is never the empty value");
        }
        assert_eq!(key_height(0), None);
    }

    #[test]
    fn sources_below_the_plane_are_not_mirrored() {
        let (_, clip) = camera();
        assert!(mirror_target(clip, Vec3::new(30.0, -1.0, 40.0), 0.0).is_none(), "bed under the water");
        assert!(mirror_target(clip, Vec3::new(30.0, 0.0, 40.0), 0.0).is_none(), "on the plane");
        assert!(mirror_target(clip, Vec3::new(30.0, 4.0, 40.0), 0.0).is_some());
    }

    /// The resolve rebuilds the source from the target pixel's ray and the stored height: the
    /// mirror image of a point lies on the target's camera ray, so the reflected ray at that
    /// height is the point itself.
    #[test]
    fn resolve_inverts_the_projection() {
        let (eye, clip) = camera();
        let world_from_clip = clip.inverse();
        for p in [Vec3::new(30.0, 4.0, 40.0), Vec3::new(10.0, 0.3, 25.0), Vec3::new(55.0, 18.0, 90.0)] {
            let (uv, key) = mirror_target(clip, p, 0.0).expect("mirror on screen");
            let ndc = Vec2::new(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);
            let near = world_from_clip.project_point3(ndc.extend(1.0));
            let dir = (near - eye).normalize();
            let height = key_height(key).expect("stored");
            let back = point_on_reflected_ray(eye, dir, 0.0, height).expect("ray reaches the plane");
            assert!(back.distance(p) < 1e-3 * p.distance(eye), "{p} -> {back}");
        }
    }

    /// Of two sources mirrored onto one pixel, the nearer one (the occluder) has the lower height.
    #[test]
    fn nearer_source_on_a_reflected_ray_is_lower() {
        let eye = Vec3::new(0.0, 10.0, 0.0);
        let dir = Vec3::new(0.0, -0.3, 1.0).normalize();
        let near = point_on_reflected_ray(eye, dir, 0.0, 2.0).expect("hit");
        let far = point_on_reflected_ray(eye, dir, 0.0, 9.0).expect("hit");
        assert!(near.distance(eye) < far.distance(eye));
        assert!(height_key(2.0) > height_key(9.0), "the GPU's atomicMax keeps the nearer");
        assert!(point_on_reflected_ray(eye, Vec3::new(0.0, 0.2, 1.0), 0.0, 2.0).is_none(), "above the horizon");
    }

    /// A flat surface looks up its own pixel: the reflected view ray maps back to the view ray.
    #[test]
    fn flat_lookup_is_the_pixel_itself() {
        let (eye, clip) = camera();
        let d = Vec3::new(0.4, -0.35, 0.8).normalize();
        let r = Vec3::new(d.x, -d.y, d.z);
        let a = project_uv(clip, eye + d).expect("on screen");
        let b = project_uv(clip, eye + lookup_direction(r)).expect("on screen");
        assert!(a.distance(b) < 1e-6);
    }
}
