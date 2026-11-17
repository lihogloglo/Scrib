//! The underwater medium's colour (water.md "Underwater", slice U3): exact extinction, in-scatter
//! by depth, channel order, and no NaN at infinite distance or depth.
#![allow(clippy::float_cmp)]

use glam::Vec3;

use crate::underwater::{sun_term, underwater_colour, UnderwaterLight, UnderwaterOptics, MIN_EXTINCTION};
use crate::WaterOptics;

/// Clear sea water (the optics record's default: red absorbed first) with `Kd` = its extinction.
fn clear() -> UnderwaterOptics {
    UnderwaterOptics::from_optics(&WaterOptics::default(), 1.0)
}

const LIGHT: UnderwaterLight = UnderwaterLight { sun: Vec3::new(3.0, 3.0, 3.0), ambient: Vec3::new(0.5, 0.6, 0.7) };
const SCENE: Vec3 = Vec3::new(0.8, 0.4, 0.2);

#[test]
fn nothing_between_the_lens_and_the_scene_is_the_scene() {
    let o = clear();
    assert_eq!(o.transmittance(0.0), Vec3::ONE);
    assert_eq!(underwater_colour(SCENE, 0.0, 3.0, &o, LIGHT), SCENE);
    // A negative or NaN distance (a bad depth read) is no path either.
    assert_eq!(o.transmittance(-1.0), Vec3::ONE);
    assert_eq!(o.transmittance(f32::NAN), Vec3::ONE);
}

#[test]
fn far_and_infinite_paths_are_the_medium_colour() {
    let o = clear();
    let medium = o.albedo() * (LIGHT.ambient + LIGHT.sun) * o.light_at_depth(3.0);
    for d in [1.0e5, f32::INFINITY, f32::MAX] {
        assert_eq!(o.transmittance(d), Vec3::ZERO, "d = {d}");
        let c = underwater_colour(SCENE, d, 3.0, &o, LIGHT);
        assert!(c.is_finite(), "d = {d}: {c}");
        assert_eq!(c, medium, "d = {d}");
    }
}

#[test]
fn red_dies_first_in_a_red_absorbing_body() {
    let o = clear();
    let t = o.transmittance(5.0);
    assert!(t.x < t.y && t.y < t.z, "{t}");
    // Exactly Beer-Lambert per channel.
    let sigma = Vec3::from(WaterOptics::default().absorption) + Vec3::from(WaterOptics::default().scattering);
    assert_eq!(t, (-sigma * 5.0).exp());
    // The light at depth dims the same way.
    let l = o.light_at_depth(5.0);
    assert!(l.x < l.y && l.y < l.z, "{l}");
}

#[test]
fn the_colour_is_the_formula() {
    let o = UnderwaterOptics { absorption: Vec3::new(0.3, 0.1, 0.05), scattering: Vec3::new(0.02, 0.03, 0.04), kd_scale: 0.5 };
    let (d, z) = (7.0, 4.0);
    // As the code sums it (0.3 + 0.02 need not be the literal 0.32 in f32).
    let sigma = Vec3::new(0.3, 0.1, 0.05) + Vec3::new(0.02, 0.03, 0.04);
    let t = (-sigma * d).exp();
    let inscatter = Vec3::new(0.02, 0.03, 0.04) / sigma * (LIGHT.ambient + LIGHT.sun) * (-sigma * 0.5 * z).exp();
    assert_eq!(underwater_colour(SCENE, d, z, &o, LIGHT), SCENE * t + inscatter * (Vec3::ONE - t));
}

#[test]
fn light_falls_off_with_depth_and_ends_black() {
    let o = clear();
    assert_eq!(o.light_at_depth(0.0), Vec3::ONE);
    // Above the surface (a split frame's eye, a bad depth) is the full surface light.
    assert_eq!(o.light_at_depth(-2.0), Vec3::ONE);
    let (a, b) = (o.light_at_depth(2.0), o.light_at_depth(10.0));
    assert!(b.cmplt(a).all(), "{a} {b}");
    assert_eq!(o.light_at_depth(f32::INFINITY), Vec3::ZERO);
    // Infinitely deep and nothing hit: black, not NaN.
    assert_eq!(underwater_colour(SCENE, f32::INFINITY, f32::INFINITY, &o, LIGHT), Vec3::ZERO);
    // No falloff at all (Kd scale 0) keeps the light at any depth, infinite included.
    let flat = UnderwaterOptics { kd_scale: 0.0, ..o };
    assert_eq!(flat.light_at_depth(f32::INFINITY), Vec3::ONE);
}

#[test]
fn empty_optics_stay_finite() {
    let none = UnderwaterOptics { absorption: Vec3::ZERO, scattering: Vec3::ZERO, kd_scale: 1.0 };
    assert_eq!(none.extinction(), Vec3::splat(MIN_EXTINCTION));
    assert_eq!(none.albedo(), Vec3::ZERO);
    for d in [0.0, 10.0, f32::INFINITY] {
        for z in [0.0, 10.0, f32::INFINITY] {
            assert!(underwater_colour(SCENE, d, z, &none, LIGHT).is_finite(), "d {d} z {z}");
        }
    }
    // A pure scatterer has albedo 1.
    let white = UnderwaterOptics { absorption: Vec3::ZERO, scattering: Vec3::ONE, kd_scale: 1.0 };
    assert_eq!(white.albedo(), Vec3::ONE);
}

#[test]
fn the_sun_lights_from_above_the_horizon_only() {
    let e = Vec3::new(1000.0, 900.0, 800.0);
    assert_eq!(sun_term(e, Vec3::Y), e / std::f32::consts::PI);
    assert_eq!(sun_term(e, Vec3::new(1.0, 0.0, 0.0)), Vec3::ZERO);
    assert_eq!(sun_term(e, Vec3::new(0.0, -1.0, 0.0)), Vec3::ZERO);
    let low = sun_term(e, Vec3::new(0.8, 0.6, 0.0));
    assert_eq!(low, e * (0.6 / std::f32::consts::PI));
}
