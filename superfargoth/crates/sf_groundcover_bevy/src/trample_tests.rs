//! The trample's CPU twin (GC-M3, [`crate::trample`]): exact checks of the push maths the vertex stages run.

use bevy_math::{Vec2, Vec3};

use crate::trample::{pressed, push, tilt};

/// `sf_surface::trail::PRESS_FLOOR` (`layer.wesl` `TRAIL_PRESS_FLOOR`).
const FLOOR: f32 = 1.0 / 256.0;
/// `sf_surface::trail::RING_TEXEL_M`: the forward differences' step.
const TEXEL_M: f32 = 1.0 / 16.0;

fn close(a: Vec3, b: Vec3) -> bool {
    (a - b).abs().max_element() < 1e-5
}

/// No press (or a trample amount of 0): no trample, and the vertex is the wind-bent one, unchanged.
#[test]
fn zero_press_leaves_the_vertex_unchanged() {
    let max = 75_f32.to_radians();
    assert_eq!(push(Vec3::ZERO, 1.0, 0.3, FLOOR), Vec3::ZERO);
    assert_eq!(push(Vec3::new(0.8, -0.1, 0.05), 0.0, 0.3, FLOOR), Vec3::ZERO, "trample_amount 0 turns it off");
    let (root, world, bend) = (Vec3::new(3.0, 1.0, -2.0), Vec3::new(3.2, 1.7, -2.1), Vec3::new(0.05, -0.01, 0.02));
    assert_eq!(pressed(world, root, Vec3::ZERO, Vec3::ZERO, max), world);
    assert_eq!(pressed(world, root, bend, Vec3::ZERO, max), world + bend);
    assert_eq!(tilt(Vec3::new(0.1, 0.9, 0.2), Vec3::ZERO, max), Vec3::new(0.1, 0.9, 0.2));
}

/// A full press at 75° tilts a vertex 1 m above the root to (sin 75°, cos 75°) along the push direction, with
/// its length kept; the wind bend is gone (faded by the press); any offset keeps its length.
#[test]
fn full_press_tilts_the_plant_about_its_root() {
    let max = 75_f32.to_radians();
    let root = Vec3::new(10.0, 2.0, -4.0);
    for dir in [Vec2::X, Vec2::new(-0.6, 0.8), Vec2::new(0.0, -1.0)] {
        let t = Vec3::new(1.0, dir.x, dir.y);
        let up = pressed(root + Vec3::Y, root, Vec3::new(0.3, 0.0, 0.1), t, max) - root;
        let (s, c) = max.sin_cos();
        assert!(close(up, Vec3::new(dir.x * s, c, dir.y * s)), "{dir}: {up}");
        assert!((up.length() - 1.0).abs() < 1e-6);
        for o in [Vec3::new(0.2, 0.5, -0.1), Vec3::new(-0.3, 0.05, 0.4), Vec3::new(0.0, -0.1, 0.0)] {
            assert!((tilt(o, t, max).length() - o.length()).abs() < 1e-6, "{o} keeps its length");
        }
    }
    // Half a press: half the angle.
    let up = tilt(Vec3::Y, Vec3::new(0.5, 1.0, 0.0), max);
    let (s, c) = (0.5 * max).sin_cos();
    assert!(close(up, Vec3::new(s, c, 0.0)), "{up}");
}

/// The push runs away from a press peak: the ring's forward differences one texel away, read at a root near a
/// foot's press (`sf_surface::trail::press_profile`-like cone), point down the gradient; on a flat press (under a
/// body) the plant falls its own way, by its wind phase; the press is capped at 1.
#[test]
#[allow(clippy::float_cmp)]
fn push_points_away_from_the_press_peak() {
    let peak = Vec2::new(5.0, 7.0);
    let radius_m = 0.35;
    let press = |xz: Vec2| (1.0 - xz.distance(peak) / radius_m).max(0.0);
    let slope = |xz: Vec2| {
        let p = press(xz);
        Vec3::new(p, press(xz + Vec2::new(TEXEL_M, 0.0)) - p, press(xz + Vec2::new(0.0, TEXEL_M)) - p)
    };
    for off in [Vec2::new(0.15, 0.0), Vec2::new(-0.1, 0.12), Vec2::new(0.02, -0.2), Vec2::new(-0.18, -0.05)] {
        let t = push(slope(peak + off), 1.0, 0.7, FLOOR);
        assert!(t.x > 0.0 && t.x < 1.0);
        let d = Vec2::new(t.y, t.z);
        assert!((d.length() - 1.0).abs() < 1e-5);
        assert!(d.dot(off.normalize()) > 0.95, "{off}: {d}");
    }
    // Flat: the phase's direction.
    let phase = 0.3_f32;
    let t = push(Vec3::new(1.0, 0.0, 0.0), 1.0, phase, FLOOR);
    let a = phase * std::f32::consts::TAU;
    assert!(close(t, Vec3::new(1.0, a.cos(), a.sin())), "{t}");
    // A press over 1 (amount 2) stays a full press.
    assert_eq!(push(Vec3::new(0.8, 0.0, 0.0), 2.0, phase, FLOOR).x, 1.0);
}
