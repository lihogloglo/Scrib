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
fn indoors_silences_the_weather_bus_only() {
    let mut m = Mixer { outdoor: 0.0, ..Mixer::default() };
    assert_eq!(m.gain(Bus::Weather, 1.0, 1.0), 0.0);
    assert_eq!(m.gain(Bus::Ambience, 1.0, 1.0), 1.0);
    assert_eq!(m.gain(Bus::Sfx, 1.0, 1.0), 1.0);
    m.outdoor = 0.5;
    assert!((m.gain(Bus::Weather, 1.0, 0.8) - 0.4).abs() < 1e-6);
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

#[test]
fn slider_curve_is_linear_in_db_with_a_taper_to_silence() {
    assert_eq!(slider_to_gain(1.0), 1.0);
    assert_eq!(slider_to_gain(0.0), 0.0);
    assert_eq!(slider_to_gain(f32::NAN), 0.0);
    assert_eq!(slider_to_gain(2.0), 1.0);
    // Equal slider steps are equal dB steps: half travel is -30 dB, a quarter from the top -15 dB.
    assert!((slider_to_db(0.5) + 30.0).abs() < 1e-3);
    assert!((slider_to_db(0.75) + 15.0).abs() < 1e-3);
    assert!((slider_to_db(VOLUME_TAPER) + VOLUME_RANGE_DB * (1.0 - VOLUME_TAPER)).abs() < 1e-3);
    // Below the taper: a straight line to 0, continuous at the joint.
    let joint = slider_to_gain(VOLUME_TAPER);
    assert!((slider_to_gain(VOLUME_TAPER / 2.0) - joint / 2.0).abs() < 1e-7);
    assert!((slider_to_gain(VOLUME_TAPER - 1e-4) - joint).abs() < 1e-5);
    assert_eq!(slider_to_db(0.0), f32::NEG_INFINITY);
    // Strictly increasing, and the inverse round-trips.
    let mut last = -1.0;
    for i in 0..=100 {
        #[expect(clippy::cast_precision_loss, reason = "small counter")]
        let s = i as f32 / 100.0;
        let g = slider_to_gain(s);
        assert!(g > last, "{s}");
        last = g;
        assert!((gain_to_slider(g) - s).abs() < 1e-4, "{s}");
    }
    assert!((db_to_gain(-6.0) - 0.501_187).abs() < 1e-5);
    assert!((gain_to_db(0.5) + 6.0206).abs() < 1e-3);
}

#[test]
fn volume_settings_write_curved_gains_and_mutes() {
    let mut v = VolumeSettings::default();
    v.set_level(Bus::Music, 0.5);
    v.set_level(Bus::Sfx, -3.0);
    v.set_muted(Bus::Voice, true);
    assert_eq!(v.get(Bus::Sfx).level, 0.0);
    let mut m = Mixer::default();
    v.apply(&mut m);
    assert!((m.bus_gain(Bus::Music) - slider_to_gain(0.5)).abs() < 1e-7);
    assert_eq!(m.bus_gain(Bus::Sfx), 0.0);
    assert_eq!(m.bus_gain(Bus::Voice), 0.0);
    // Unmuting restores the kept level.
    assert_eq!(v.get(Bus::Voice).level, 1.0);
    v.set_muted(Bus::Voice, false);
    v.apply(&mut m);
    assert_eq!(m.bus_gain(Bus::Voice), 1.0);
    // Master scales every bus; muting it silences all.
    v.set_level(Bus::Master, 0.75);
    v.apply(&mut m);
    assert!((m.bus_gain(Bus::Music) - slider_to_gain(0.5) * slider_to_gain(0.75)).abs() < 1e-7);
    v.set_muted(Bus::Master, true);
    v.apply(&mut m);
    assert!(Bus::ALL.iter().all(|b| m.bus_gain(*b) == 0.0));
}

#[test]
fn the_ambience_bus_scales_the_weather() {
    assert_eq!(Bus::Weather.parent(), Some(Bus::Ambience));
    assert_eq!(Bus::Ambience.parent(), Some(Bus::Master));
    assert_eq!(Bus::Master.parent(), None);
    assert!(Bus::ALL.iter().all(|b| Bus::from_name(b.name()) == Some(*b)));
    assert!(!Bus::USER.contains(&Bus::Weather));
    let mut m = Mixer::default();
    m.set_volume(Bus::Ambience, 0.5);
    m.set_volume(Bus::Master, 0.5);
    assert!((m.bus_gain(Bus::Weather) - 0.25).abs() < 1e-7);
    m.set_muted(Bus::Ambience, true);
    assert_eq!(m.bus_gain(Bus::Weather), 0.0);
    assert_eq!(m.bus_gain(Bus::Sfx), 0.5);
}

fn duck(trigger: DuckTrigger, target: Bus, depth_db: f32) -> DuckRule {
    DuckRule { trigger, target, depth_db, attack_s: 0.5, release_s: 1.0 }
}

#[test]
fn speech_ducks_music_with_attack_and_release() {
    let mut d = Ducker::new([duck(DuckTrigger::Bus(Bus::Voice), Bus::Music, 8.0)]);
    let mut m = Mixer::default();
    let speaking = |b: Bus| b == Bus::Voice;
    d.tick(0.25, speaking);
    assert!((d.duck_db(Bus::Music) - 4.0).abs() < 1e-5, "half the attack");
    d.tick(0.25, speaking);
    d.apply(&mut m);
    assert!((m.bus_gain(Bus::Music) - db_to_gain(-8.0)).abs() < 1e-6);
    assert_eq!(m.bus_gain(Bus::Sfx), 1.0);
    d.tick(10.0, speaking);
    assert!((d.duck_db(Bus::Music) - 8.0).abs() < 1e-5, "holds at depth");
    // Speech ends: released over release_s (1 s).
    d.tick(0.5, |_| false);
    assert!((d.duck_db(Bus::Music) - 4.0).abs() < 1e-5);
    d.tick(0.5, |_| false);
    d.apply(&mut m);
    assert_eq!(m.bus_gain(Bus::Music), 1.0);
}

#[test]
fn ducks_do_not_stack_and_holds_duck_children() {
    let mut d = Ducker::new([
        duck(DuckTrigger::Bus(Bus::Voice), Bus::Ambience, 6.0),
        duck(DuckTrigger::Hold("menu".into()), Bus::Ambience, 12.0),
    ]);
    d.set_hold("menu", true);
    assert!(d.is_held("menu"));
    d.tick(1.0, |b| b == Bus::Voice);
    // Deepest wins (12 dB), not 18.
    assert!((d.duck_db(Bus::Ambience) - 12.0).abs() < 1e-5);
    let mut m = Mixer::default();
    d.apply(&mut m);
    // Weather sits under Ambience, so it ducks too; its own duck stays 1.
    assert_eq!(m.duck(Bus::Weather), 1.0);
    assert!((m.bus_gain(Bus::Weather) - db_to_gain(-12.0)).abs() < 1e-6);
    d.set_hold("menu", false);
    d.tick(2.0, |b| b == Bus::Voice);
    assert!((d.duck_db(Bus::Ambience) - 6.0).abs() < 1e-5);
    // Instant times (0 s) jump.
    let mut snap = Ducker::new([DuckRule { attack_s: 0.0, release_s: 0.0, ..duck(DuckTrigger::Bus(Bus::Ui), Bus::Sfx, 3.0) }]);
    snap.tick(0.0, |b| b == Bus::Ui);
    assert!((snap.duck_db(Bus::Sfx) - 3.0).abs() < 1e-6);
}
