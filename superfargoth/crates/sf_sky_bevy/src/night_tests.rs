//! NS-3c: each moon's glow in the night uniform (exact, CPU side; the look is the owner's).
#![allow(clippy::float_cmp, reason = "exact values: the uniform copies the glow as computed")]

use super::*;
use bevy_math::Vec3;
use sf_sky::{MoonGlow, SkyPosition};

/// A full, visible moon high in the sky.
fn state(direction: Vec3, illuminance: f32) -> MoonState {
    MoonState {
        position: SkyPosition { direction, ..SkyPosition::default() },
        phase: 0.5,
        illuminated: 1.0,
        brightness: 1.0,
        frame: None,
        illuminance,
        full_illuminance: illuminance,
        transmittance: Vec3::new(0.9, 0.8, 0.6),
        angular_size: 0.2,
        tint: Vec3::new(1.0, 0.9, 0.8),
        occluder: 0.85,
        alpha: 1.0,
        texture_blend: 1.0,
    }
}

fn look(glow: f32, glow_size: f32) -> Moon {
    Moon { glow, glow_size, ..Moon::default() }
}

#[test]
fn the_uniform_carries_the_moon_glow_formula() {
    let s = state(Vec3::Y, 2.0);
    let m = gpu_moon(&s, Some(&look(0.04, 0.5)), true);
    let edge = 0.5 * 0.2 * 0.85;
    let falloff = 0.5 * edge;
    let expected = s.tint * s.transmittance * (0.04 * 2.0 / (std::f32::consts::TAU * falloff * (edge + falloff)));
    assert_eq!(m.glow_edge, edge);
    assert_eq!(m.glow_falloff, falloff);
    assert!((m.glow_radiance - expected).abs().max_element() < 1e-6 * expected.max_element(), "{} vs {expected}", m.glow_radiance);
    // Reddened by the atmosphere: the transmittance's hue carries into the glow.
    assert!(m.glow_radiance.x > m.glow_radiance.z);
}

#[test]
fn a_hidden_or_set_moon_has_no_glow() {
    let lit = look(0.04, 0.5);
    let mut hidden = state(Vec3::Y, 2.0);
    hidden.alpha = 0.0;
    let m = gpu_moon(&hidden, Some(&lit), true);
    assert_eq!((m.glow_radiance, m.glow_falloff), (Vec3::ZERO, 0.0));

    // Below the horizon the transmittance (times the visible part of the disk) is 0.
    let mut set = state(Vec3::NEG_Y, 2.0);
    set.transmittance = Vec3::ZERO;
    let m = gpu_moon(&set, Some(&lit), true);
    assert_eq!((m.glow_radiance, m.glow_falloff), (Vec3::ZERO, 0.0));

    // New moon: no light, no glow; glow 0: off; no settings: none.
    let new = state(Vec3::Y, 0.0);
    assert_eq!(gpu_moon(&new, Some(&lit), true).glow_falloff, 0.0);
    assert_eq!(gpu_moon(&state(Vec3::Y, 2.0), Some(&look(0.0, 0.5)), true).glow_falloff, 0.0);
    assert_eq!(gpu_moon(&state(Vec3::Y, 2.0), None, true).glow_falloff, 0.0);
}

#[test]
fn the_glow_follows_the_phase_and_the_opacity() {
    let lit = look(0.04, 0.5);
    let full = state(Vec3::Y, 2.0);
    // A quarter moon: the Lambert brightness 1/π of full (phase_light), full illuminance kept.
    let (_, brightness) = sf_sky::phase_light(0.25);
    let quarter = MoonState { phase: 0.25, illuminance: 2.0 * brightness, ..full };
    let g_full = gpu_moon(&full, Some(&lit), true).glow_radiance;
    let g_quarter = gpu_moon(&quarter, Some(&lit), true).glow_radiance;
    assert!((g_quarter - g_full * brightness).abs().max_element() < 1e-6 * g_full.max_element());
    // The disk itself is drawn from the full illuminance (the texture draws the phase).
    assert_eq!(gpu_moon(&quarter, Some(&lit), true).radiance, gpu_moon(&full, Some(&lit), true).radiance);
    // Half opacity (a fading driven moon), half glow.
    let fading = MoonState { alpha: 0.5, ..full };
    let g_fading = gpu_moon(&fading, Some(&lit), true).glow_radiance;
    assert!((g_fading - g_full * 0.5).abs().max_element() < 1e-6 * g_full.max_element());
}

#[test]
fn two_moons_get_two_independent_glows() {
    let a = state(Vec3::new(0.0, 0.8, 0.6).normalize(), 3.0);
    let b = MoonState { angular_size: 0.08, tint: Vec3::new(0.9, 0.93, 1.0), ..state(Vec3::new(0.6, 0.8, 0.0).normalize(), 0.5) };
    let looks = [look(0.04, 0.5), look(0.1, 1.5)];
    let ga = gpu_moon(&a, Some(&looks[0]), true);
    let gb = gpu_moon(&b, Some(&looks[1]), false);
    assert_eq!(ga.glow_radiance, looks[0].glow(&a).radiance);
    assert_eq!(gb.glow_radiance, looks[1].glow(&b).radiance);
    assert_eq!(gb.glow_falloff, 1.5 * (0.5 * 0.08 * 0.85));
    assert_ne!(ga.glow_radiance, gb.glow_radiance);
    // Changing one moon's settings leaves the other's glow alone.
    let ga2 = gpu_moon(&a, Some(&looks[0]), true);
    let gb2 = gpu_moon(&b, Some(&look(0.2, 0.3)), false);
    assert_eq!(ga2, ga);
    assert_ne!(gb2.glow_radiance, gb.glow_radiance);
    assert_eq!(gb2.radiance, gb.radiance, "the glow never changes the disk");
}

#[test]
fn glow_settings_clamp_to_their_tunable_ranges() {
    let s = state(Vec3::Y, 2.0);
    let wild = look(5.0, 100.0).glow(&s);
    let max = look(1.0, 8.0).glow(&s);
    assert_eq!(wild, max);
    assert_eq!(look(-1.0, 0.5).glow(&s), MoonGlow { radiance: Vec3::ZERO, ..look(0.0, 0.5).glow(&s) });
}
