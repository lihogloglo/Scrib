//! `IN4a` impact damage core: nothing under `interact.impact.speed_min_mps` at any impulse, nothing at
//! the dead zone, linear above it (exact bits at two points), non-finite input -> 0 (an overflow
//! saturates), same inputs same bits.

use crate::*;

fn params() -> InteractParams {
    InteractParams {
        impact_speed_min_mps: 3.0,
        impact_impulse_min_ns: 0.5,
        impact_damage_per_ns: 0.25,
        ..InteractParams::default()
    }
}

#[test]
fn under_speed_min_nothing_at_any_impulse() {
    let p = params();
    for impulse in [0.0, 0.6, 4.0, 1.0e3, 1.0e9] {
        assert_eq!(impact_damage(impulse, 2.999, &p).to_bits(), 0.0_f32.to_bits(), "impulse {impulse}");
        assert_eq!(impact_damage(impulse, 0.16, &p).to_bits(), 0.0_f32.to_bits(), "resting {impulse}");
    }
    // At the gate it counts.
    assert_eq!(impact_damage(4.5, 3.0, &p).to_bits(), 1.0_f32.to_bits());
}

#[test]
fn dead_zone_then_linear() {
    let p = params();
    assert_eq!(impact_damage(0.5, 10.0, &p).to_bits(), 0.0_f32.to_bits(), "at the dead zone");
    assert_eq!(impact_damage(0.25, 10.0, &p).to_bits(), 0.0_f32.to_bits(), "inside the dead zone");
    assert_eq!(impact_damage(-5.0, 10.0, &p).to_bits(), 0.0_f32.to_bits(), "negative impulse");
    // 0.25 x (4.5 - 0.5) = 1.0 and 0.25 x (10.5 - 0.5) = 2.5, exact in f32.
    assert_eq!(impact_damage(4.5, 5.0, &p).to_bits(), 1.0_f32.to_bits());
    assert_eq!(impact_damage(10.5, 5.0, &p).to_bits(), 2.5_f32.to_bits());
    // The speed past the gate does not scale it: impulse already carries the speed.
    assert_eq!(impact_damage(10.5, 50.0, &p).to_bits(), 2.5_f32.to_bits());
    // Defaults: the spec's worked bottle thrown at 10 m/s (4 N·s) takes 1.75.
    let d = InteractParams::default();
    assert_eq!(impact_damage(4.0, 10.0, &d).to_bits(), 1.75_f32.to_bits());
}

#[test]
fn non_finite_is_zero() {
    let p = params();
    for (impulse, speed) in [
        (f32::NAN, 5.0),
        (4.5, f32::NAN),
        (f32::INFINITY, 5.0),
        (f32::NEG_INFINITY, 5.0),
        (4.5, f32::INFINITY),
        (4.5, f32::NEG_INFINITY),
        (f32::NAN, f32::NAN),
    ] {
        assert_eq!(impact_damage(impulse, speed, &p).to_bits(), 0.0_f32.to_bits(), "{impulse} {speed}");
    }
    // A finite impulse whose damage overflows f32 saturates (still breaks), never infinity.
    let huge = InteractParams { impact_damage_per_ns: 10.0, ..p };
    assert_eq!(impact_damage(f32::MAX, 5.0, &huge).to_bits(), f32::MAX.to_bits());
}

#[test]
fn same_inputs_same_bits() {
    let p = params();
    let cases = [(0.7_f32, 3.2_f32), (1.7, 4.2), (7.7, 7.7), (10.0, 10.0), (123.456, 9.81)];
    let first: Vec<u32> = cases.iter().map(|&(i, s)| impact_damage(i, s, &p).to_bits()).collect();
    for _ in 0..3 {
        let again: Vec<u32> = cases.iter().map(|&(i, s)| impact_damage(i, s, &p).to_bits()).collect();
        assert_eq!(first, again);
    }
    for ((i, _), bits) in cases.iter().zip(&first) {
        assert_eq!(*bits, (0.25_f32 * (i - 0.5)).to_bits(), "linear formula at {i}");
    }
}
