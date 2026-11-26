//! FL-5c1: the swim rule's hysteresis, current, clamp, wading, dry cases and defaults.

use glam::Vec3;
use sf_records::RecordId;

use crate::query::{Immersion, WaterSample};
use crate::records::WaterKind;
use crate::swim::{SwimParams, TUNABLES, medium};

fn sample(velocity: Vec3) -> WaterSample {
    WaterSample {
        body: RecordId::new("test_river"),
        kind: WaterKind::River,
        height: 0.0,
        normal: Vec3::Y,
        velocity,
        depth: f32::INFINITY,
    }
}

fn immersed(fraction: f32, velocity: Vec3) -> Immersion {
    Immersion { depth: 1.5, fraction, sample: Some(sample(velocity)) }
}

fn hysteresis_params() -> SwimParams {
    SwimParams { enter_fraction: 0.9, exit_fraction: 0.6, ..SwimParams::default() }
}

fn assert_vec_near(a: Vec3, b: Vec3) {
    assert!((a - b).length() < 1e-6, "{a} != {b}");
}

#[test]
fn swim_hysteresis_enters_at_enter_and_leaves_below_exit() {
    let p = hysteresis_params();
    let swims = |fraction, swimming| medium(&immersed(fraction, Vec3::ZERO), swimming, &p).swim;
    assert!(swims(0.9, false));
    assert!(!swims(0.85, false));
    assert!(swims(0.85, true));
    assert!(swims(0.6, true));
    assert!(!swims(0.59, true));
    assert!(!swims(0.89, false));
}

#[test]
fn swim_exit_above_enter_counts_as_enter() {
    let p = SwimParams { enter_fraction: 0.5, exit_fraction: 0.8, ..SwimParams::default() };
    // A swimmer at 0.6 stays: the exit threshold is min(0.8, 0.5) = 0.5.
    assert!(medium(&immersed(0.6, Vec3::ZERO), true, &p).swim);
    assert!(!medium(&immersed(0.49, Vec3::ZERO), true, &p).swim);
}

#[test]
fn swim_current_is_the_horizontal_velocity_times_gain() {
    let v = Vec3::new(1.5, 0.4, -2.0);
    let p = SwimParams { current_gain: 1.0, ..SwimParams::default() };
    let m = medium(&immersed(1.0, v), true, &p);
    assert!(m.swim);
    assert_eq!(m.current, Vec3::new(1.5, 0.0, -2.0));

    let p = SwimParams { current_gain: 0.5, ..SwimParams::default() };
    let m = medium(&immersed(1.0, v), true, &p);
    assert_eq!(m.current, Vec3::new(0.75, 0.0, -1.0));
}

#[test]
fn swim_current_is_clamped_to_max_speed() {
    let p = SwimParams { current_gain: 1.0, max_mps: 3.0, ..SwimParams::default() };
    let m = medium(&immersed(1.0, Vec3::new(6.0, 0.0, 8.0)), true, &p);
    assert_vec_near(m.current, Vec3::new(1.8, 0.0, 2.4));
    assert!((m.current.length() - 3.0).abs() < 1e-6);
}

#[test]
fn swim_wading_scales_the_current_by_wade_gain_and_fraction() {
    let p = SwimParams { current_gain: 1.0, wade_gain: 0.3, ..SwimParams::default() };
    let imm = immersed(0.5, Vec3::new(2.0, 0.0, 0.0));
    let m = medium(&imm, false, &p);
    assert!(!m.swim);
    assert_vec_near(m.current, Vec3::new(0.3, 0.0, 0.0));
    assert!((m.surface - imm.depth).abs() < 1e-6, "{}", m.surface);
}

#[test]
fn swim_dry_without_sample_or_depth() {
    let p = SwimParams::default();
    let dry = |imm: &Immersion| {
        let m = medium(imm, true, &p);
        assert!(!m.swim);
        assert!(m.surface.abs() < 1e-6, "{}", m.surface);
        assert_eq!(m.current, Vec3::ZERO);
    };
    dry(&Immersion { depth: 1.0, fraction: 1.0, sample: None });
    dry(&Immersion { depth: 0.0, fraction: 1.0, sample: Some(sample(Vec3::X)) });
    dry(&Immersion { depth: -0.5, fraction: 1.0, sample: Some(sample(Vec3::X)) });
    dry(&Immersion { depth: f32::NAN, fraction: 1.0, sample: Some(sample(Vec3::X)) });
}

#[test]
fn swim_non_finite_velocity_gives_no_current() {
    let p = SwimParams::default();
    let m = medium(&immersed(1.0, Vec3::new(f32::NAN, 0.0, 1.0)), true, &p);
    assert_eq!(m.current, Vec3::ZERO);
    let m = medium(&immersed(1.0, Vec3::new(f32::INFINITY, 0.0, 1.0)), true, &p);
    assert_eq!(m.current, Vec3::ZERO);
}

#[test]
fn swim_defaults_match_the_defs() {
    let p = SwimParams::default();
    assert_eq!(
        p,
        SwimParams {
            enter_fraction: 0.7,
            exit_fraction: 0.6,
            current_gain: 1.0,
            wade_gain: 0.3,
            max_mps: 3.0,
        }
    );
    assert_eq!(TUNABLES.len(), 5);
    for d in TUNABLES {
        assert!(d.name().starts_with("swim."), "{}", d.name());
    }
}
