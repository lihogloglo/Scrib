//! `IN18a` noise core: no noise under `interact.noise.min_speed_mps`, the speed multiplier clamps at
//! `interact.noise.speed_mul_max`, an object's merge window keeps the loudest, the per-tick cap keeps the
//! loudest N (a cut noise opens no window), and any input order gives the same output.

use glam::Vec3;

use crate::*;

fn params() -> InteractParams {
    InteractParams {
        noise_min_speed_mps: 1.0,
        noise_ref_speed_mps: 5.0,
        noise_speed_mul_max: 2.0,
        noise_merge_s: 0.25,
        noise_per_tick_max: 16,
        tick_hz: 60.0,
        ..InteractParams::default()
    }
}

fn noise(source: u64, loudness_m: f32) -> Noise {
    Noise {
        source,
        at: Vec3::new(1.0, 2.0, 3.0),
        kind: TagId::of("wood"),
        loudness_m,
        maker: None,
        reveals_maker: false,
        case: NoiseCase::Impact,
    }
}

fn louds(heard: &[Noise]) -> Vec<(u64, f32)> {
    heard.iter().map(|n| (n.source, n.loudness_m)).collect()
}

#[test]
fn under_min_speed_none() {
    let p = params();
    assert_eq!(loudness_m(4.0, 0.999, &p), None);
    assert_eq!(loudness_m(4.0, 0.0, &p), None);
    assert_eq!(loudness_m(4.0, -3.0, &p), None);
    // At the gate it counts: 4 m x (1 / 5).
    assert_eq!(loudness_m(4.0, 1.0, &p).map(f32::to_bits), Some((4.0_f32 * (1.0_f32 / 5.0)).to_bits()));
    // At the reference speed the material's own reach.
    assert_eq!(loudness_m(4.0, 5.0, &p).map(f32::to_bits), Some(4.0_f32.to_bits()));
    // Silent material, non-finite input, NaN tunable: none.
    assert_eq!(loudness_m(0.0, 5.0, &p), None);
    assert_eq!(loudness_m(f32::NAN, 5.0, &p), None);
    assert_eq!(loudness_m(4.0, f32::INFINITY, &p), None);
    assert_eq!(loudness_m(4.0, 5.0, &InteractParams { noise_ref_speed_mps: f32::NAN, ..p }), None);
}

#[test]
fn speed_multiplier_clamps_at_max() {
    let p = params();
    assert_eq!(loudness_m(4.0, 7.5, &p).map(f32::to_bits), Some(6.0_f32.to_bits()));
    assert_eq!(loudness_m(4.0, 10.0, &p).map(f32::to_bits), Some(8.0_f32.to_bits()));
    assert_eq!(loudness_m(4.0, 20.0, &p).map(f32::to_bits), Some(8.0_f32.to_bits()), "clamped at 2x");
    assert_eq!(loudness_m(4.0, 1.0e30, &p).map(f32::to_bits), Some(8.0_f32.to_bits()), "clamped at 2x");
}

#[test]
fn merge_window_keeps_the_loudest() {
    let p = params();
    // 0.25 s at 60 Hz = 15 ticks.
    assert_eq!(p.ticks(p.noise_merge_s), 15);
    let mut merge = NoiseMerge::default();
    // One object twice in a tick: its loudest.
    assert_eq!(louds(&merge.step(Tick(100), [noise(1, 3.0), noise(1, 5.0)], &p)), vec![(1, 5.0)]);
    // Inside the window a quieter or equal noise is not heard (one rattle, one noise).
    assert!(merge.step(Tick(101), [noise(1, 4.0)], &p).is_empty());
    assert!(merge.step(Tick(102), [noise(1, 5.0)], &p).is_empty());
    // A louder one is, and raises the bar without reopening the window.
    assert_eq!(louds(&merge.step(Tick(110), [noise(1, 6.0)], &p)), vec![(1, 6.0)]);
    assert!(merge.step(Tick(114), [noise(1, 5.5)], &p).is_empty(), "tick 114 is still inside 100 + 15");
    // Another object has its own window.
    assert_eq!(louds(&merge.step(Tick(114), [noise(2, 0.5)], &p)), vec![(2, 0.5)]);
    // The window opened at 100 ends at 115: any noise is heard again.
    assert_eq!(louds(&merge.step(Tick(115), [noise(1, 1.0)], &p)), vec![(1, 1.0)]);
    // merge_s = 0: one tick, the same tick only.
    let p0 = InteractParams { noise_merge_s: 0.0, ..p };
    let mut merge = NoiseMerge::default();
    assert_eq!(louds(&merge.step(Tick(0), [noise(1, 2.0), noise(1, 1.0)], &p0)), vec![(1, 2.0)]);
    assert_eq!(louds(&merge.step(Tick(1), [noise(1, 1.0)], &p0)), vec![(1, 1.0)]);
    // A world change forgets every window.
    merge.clear();
    assert_eq!(merge.open_windows(), 0);
}

#[test]
fn cap_keeps_the_loudest_n() {
    let p = InteractParams { noise_per_tick_max: 3, ..params() };
    let mut merge = NoiseMerge::default();
    let heard = merge.step(Tick(0), (1..=5).map(|s| noise(s, f32::from(u8::try_from(s).unwrap()))), &p);
    assert_eq!(louds(&heard), vec![(5, 5.0), (4, 4.0), (3, 3.0)], "loudest first");
    assert_eq!(merge.open_windows(), 3);
    // The cut noises opened no window: the next tick they are heard, even quieter.
    assert_eq!(louds(&merge.step(Tick(1), [noise(1, 0.5), noise(2, 0.5)], &p)), vec![(1, 0.5), (2, 0.5)]);
    // Not a positive finite loudness: dropped, no window.
    assert!(merge.step(Tick(2), [noise(9, 0.0), noise(9, f32::NAN), noise(8, f32::INFINITY)], &p).is_empty());
}

#[test]
fn any_input_order_same_output() {
    let p = InteractParams { noise_per_tick_max: 4, ..params() };
    let base = vec![
        noise(1, 3.0),
        // Same object, same loudness, other fields differ: the total order picks one.
        Noise { case: NoiseCase::Break, ..noise(1, 3.0) },
        Noise { at: Vec3::new(-1.0, 0.0, 0.0), ..noise(1, 3.0) },
        noise(2, 3.0),
        Noise { maker: Some(UserKey::Player(sf_ids::PlayerKey(0))), reveals_maker: true, ..noise(3, 3.0) },
        noise(4, 7.0),
        noise(5, 3.0),
        noise(6, 1.0),
    ];
    let expected = NoiseMerge::default().step(Tick(0), base.clone(), &p);
    assert_eq!(expected.len(), 4);
    assert_eq!(expected[0].source, 4, "loudest first");
    for shift in 0..base.len() {
        let mut v = base.clone();
        v.rotate_left(shift);
        assert_eq!(NoiseMerge::default().step(Tick(0), v.clone(), &p), expected, "rotation {shift}");
        v.reverse();
        assert_eq!(NoiseMerge::default().step(Tick(0), v, &p), expected, "reversed rotation {shift}");
    }
}
