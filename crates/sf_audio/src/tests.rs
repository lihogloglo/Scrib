#![expect(clippy::float_cmp, reason = "clamped fades land exactly on their targets")]

use super::*;

#[test]
fn fade_moves_linearly_and_never_overshoots() {
    assert!((fade_toward(0.0, 1.0, 0.5, 2.0) - 0.25).abs() < 1e-6);
    assert_eq!(fade_toward(0.9, 1.0, 1.0, 2.0), 1.0);
    assert!((fade_toward(1.0, 0.0, 1.0, 2.0) - 0.5).abs() < 1e-6);
    assert_eq!(fade_toward(0.1, 0.0, 10.0, 2.0), 0.0);
    assert_eq!(fade_toward(0.3, 0.8, 0.1, 0.0), 0.8);
}

#[test]
fn crossfade_keeps_total_level_constant() {
    let (mut a, mut b) = (1.0, 0.0);
    for _ in 0..10 {
        a = fade_toward(a, 0.0, 0.1, 2.0);
        b = fade_toward(b, 1.0, 0.1, 2.0);
        assert!((a + b - 1.0f32).abs() < 1e-5);
    }
}

#[test]
fn mixer_gain_multiplies_bus_and_master() {
    let mut m = Mixer::default();
    m.set_volume(Bus::Master, 0.5);
    m.set_volume(Bus::Weather, 0.4);
    assert!((m.gain(Bus::Weather, 0.5, 0.8) - 0.5 * 0.8 * 0.4 * 0.5).abs() < 1e-6);
    assert!((m.bus_gain(Bus::Master) - 0.5).abs() < 1e-6);
    m.set_muted(Bus::Weather, true);
    assert_eq!(m.gain(Bus::Weather, 1.0, 1.0), 0.0);
    m.set_muted(Bus::Weather, false);
    m.set_muted(Bus::Master, true);
    assert_eq!(m.gain(Bus::Ambience, 1.0, 1.0), 0.0);
}

#[test]
fn scheduler_waits_the_interval_then_draws() {
    let mut s = OneShotScheduler::new(7);
    // Interval 4..6 s, chance 1 (always a sound): a draw within 4..6.1 s.
    let mut t = 0.0;
    let mut drawn = None;
    while t < 6.1 {
        t += 0.1;
        if let Some(i) = s.tick(0.1, [4.0, 6.0], &[1.0]) {
            drawn = Some(t);
            assert_eq!(i, 0);
            break;
        }
    }
    assert!(drawn.is_some_and(|t| (3.9..=6.2).contains(&t)), "{drawn:?}");
}

#[test]
fn scheduler_is_deterministic_and_respects_chances() {
    let run = |seed| {
        let mut s = OneShotScheduler::new(seed);
        (0..3000).filter_map(|_| s.tick(1.0, [1.0, 1.0], &[0.2, 0.3])).collect::<Vec<_>>()
    };
    assert_eq!(run(3), run(3));
    // A draw lands every tick (interval 1 s, dt 1 s): 3000 draws, 20 % and 30 % of them.
    let mut s = OneShotScheduler::new(3);
    let (mut a, mut b) = (0, 0);
    for _ in 0..3000 {
        match s.tick(1.0, [1.0, 1.0], &[0.2, 0.3]) {
            Some(0) => a += 1,
            Some(_) => b += 1,
            None => {}
        }
    }
    assert!((f64::from(a) / 3000.0 - 0.2).abs() < 0.04, "{a}");
    assert!((f64::from(b) / 3000.0 - 0.3).abs() < 0.04, "{b}");
}

#[test]
fn chances_above_one_normalize_to_no_silence() {
    let mut s = OneShotScheduler::new(1);
    let n = (0..200).filter_map(|_| s.tick(1.0, [1.0, 1.0], &[0.9, 0.9])).count();
    assert_eq!(n, 200);
}
