//! ai.md "Perception and stimuli" → "Awareness" (AI8a): band hysteresis at the threshold edges,
//! decay over a known tick count (and tier invariance), the Alerted hold, noise gain and cap,
//! distraction, eviction, asleep and groggy, the `PlayerDetection` query, invalid input and
//! determinism. The test config is dyadic (thresholds, rates and the 1/8 s tick are powers of
//! two), so the arithmetic is exact and floats are compared exactly.
#![allow(clippy::float_cmp)]

use glam::{IVec3, Vec3};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};

use crate::*;

const TICK_S: f32 = 0.125;
const P: PlayerKey = PlayerKey(7);
const PLAYER: PerceptTarget = PerceptTarget::Player(P);

fn cfg() -> AwarenessConfig {
    AwarenessConfig {
        curious: 0.125,
        suspicious: 0.375,
        searching: 0.625,
        band_hysteresis: 0.0625,
        alert_hold_s: 1.0,
        decay_delay_s: 0.5,
        decay_curious: 0.125,
        decay_suspicious: 0.0625,
        decay_searching: 0.031_25,
        noise_gain: 0.5,
        noise_cap: 0.625,
        noise_confidence: 0.5,
        asleep_wake_m: 8.0,
        groggy_s: 1.0,
        max_entries: 3,
        distraction_ttl_s: 2.0,
        alert_alertness: 1.5,
        groggy_alertness: 0.5,
    }
}

fn at(tick: u64) -> PerceptClock {
    PerceptClock::new(tick, TICK_S)
}

fn pos(x: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, 0.0, 0.0))
}

fn actor(n: u128) -> PerceptTarget {
    PerceptTarget::Actor(ActorKey(n))
}

fn sight(target: PerceptTarget, gain: f32) -> Stimulus {
    Stimulus::Sight { target, gain, pos: pos(1.0), velocity: Vec3::X }
}

fn noise(source: Option<PerceptTarget>, reveals: bool, loudness_m: f32, distance_m: f32) -> Stimulus {
    Stimulus::Noise(HeardNoise { source, reveals_source: reveals, at: pos(2.0), loudness_m, distance_m })
}

fn level(m: &AwarenessMeter, t: PerceptTarget) -> Option<(f32, AwarenessBand)> {
    m.get(t).map(|e| (e.level, e.band))
}

use AwarenessBand::{Alerted, Curious, Searching, Suspicious, Unaware};

#[test]
fn bands_enter_at_threshold_and_leave_below_it_minus_hysteresis() {
    let c = cfg();
    // Up: entered exactly at the threshold.
    assert_eq!(c.next_band(Unaware, 0.124), Unaware);
    assert_eq!(c.next_band(Unaware, 0.125), Curious);
    assert_eq!(c.next_band(Curious, 0.375), Suspicious);
    assert_eq!(c.next_band(Suspicious, 0.625), Searching);
    assert_eq!(c.next_band(Searching, 0.999), Searching);
    assert_eq!(c.next_band(Searching, 1.0), Alerted);
    assert_eq!(c.next_band(Unaware, 1.0), Alerted);
    // Down: held exactly at threshold - hysteresis, left just below it.
    assert_eq!(c.next_band(Curious, 0.0625), Curious);
    assert_eq!(c.next_band(Curious, 0.0624), Unaware);
    assert_eq!(c.next_band(Suspicious, 0.3125), Suspicious);
    assert_eq!(c.next_band(Suspicious, 0.3124), Curious);
    assert_eq!(c.next_band(Searching, 0.5625), Searching);
    assert_eq!(c.next_band(Searching, 0.5624), Suspicious);
    // A big drop stops at the first band whose stay level holds (same as many small steps).
    assert_eq!(c.next_band(Searching, 0.3125), Suspicious);
    assert_eq!(c.next_band(Searching, 0.0625), Curious);
    assert_eq!(c.next_band(Searching, 0.0), Unaware);
    // Alerted is latched: only the hold drops it.
    assert_eq!(c.next_band(Alerted, 0.2), Alerted);

    // Through the meter: rising past an edge, then a small gain inside the hysteresis band.
    let mut m = AwarenessMeter::new();
    assert_eq!(m.apply(&c, &sight(PLAYER, 0.0625), at(0)), StimulusOutcome::Raised(Unaware));
    assert_eq!(m.apply(&c, &sight(PLAYER, 0.0625), at(1)), StimulusOutcome::Raised(Curious));
    assert_eq!(level(&m, PLAYER), Some((0.125, Curious)));
}

#[test]
fn decay_starts_after_delay_and_reaches_unaware_in_a_known_tick_count() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    m.apply(&c, &sight(PLAYER, 0.25), at(0));
    // Delay 0.5 s = 4 ticks; then 0.125/s x 0.125 s = 1/64 per tick, from 0.25 to 0 in 16 ticks.
    let mut seen = Vec::new();
    for t in 1..=20 {
        m.update(&c, at(t));
        seen.push(level(&m, PLAYER));
    }
    assert_eq!(seen[3], Some((0.25, Curious)), "tick 4: still in the delay");
    assert_eq!(seen[4], Some((0.25 - 1.0 / 64.0, Curious)));
    assert_eq!(seen[15], Some((0.0625, Curious)), "tick 16: exactly at the stay level");
    assert_eq!(seen[16], Some((0.046_875, Unaware)), "tick 17: left Curious");
    assert_eq!(seen[18], Some((1.0 / 64.0, Unaware)));
    assert_eq!(seen[19], None, "tick 20: decayed to 0 and forgotten");

    // Tier invariance: one coarse update lands on the same level and band.
    for (tick, want) in [(13, Some((0.109_375, Curious))), (17, Some((0.046_875, Unaware))), (20, None)] {
        let mut coarse = AwarenessMeter::new();
        coarse.apply(&c, &sight(PLAYER, 0.25), at(0));
        coarse.update(&c, at(tick));
        assert_eq!(level(&coarse, PLAYER), want, "single update at tick {tick}");
    }
}

#[test]
fn alerted_holds_while_in_contact_then_drops_to_the_searching_floor() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    assert_eq!(m.apply(&c, &sight(PLAYER, 1.0), at(0)), StimulusOutcome::Raised(Alerted));
    m.apply(&c, &sight(PLAYER, 0.0), at(4)); // still seen: refreshes the contact
    // Hold 1 s = 8 ticks from the last contact (tick 4).
    m.update(&c, at(11));
    assert_eq!(level(&m, PLAYER), Some((1.0, Alerted)));
    m.update(&c, at(12));
    assert_eq!(level(&m, PLAYER), Some((0.625, Searching)));
    m.update(&c, at(13));
    assert_eq!(level(&m, PLAYER), Some((0.625 - 0.031_25 / 8.0, Searching)));

    // One update at tick 20 decays only from the drop (tick 12): 1 s at 0.031 25/s.
    let mut coarse = AwarenessMeter::new();
    coarse.apply(&c, &sight(PLAYER, 1.0), at(0));
    coarse.apply(&c, &sight(PLAYER, 0.0), at(4));
    coarse.update(&c, at(20));
    assert_eq!(level(&coarse, PLAYER), Some((0.593_75, Searching)));
    // From Suspicious up, sight alertness is higher.
    assert_eq!(coarse.alertness(&c, PLAYER, at(20)), 1.5);
    assert_eq!(coarse.alertness(&c, actor(1), at(20)), 1.0);
}

#[test]
fn noise_gain_falls_off_with_distance_and_never_alerts() {
    let c = cfg();
    let heard = |d| HeardNoise { source: Some(PLAYER), reveals_source: true, at: pos(2.0), loudness_m: 16.0, distance_m: d };
    assert_eq!(c.noise_gain(&heard(0.0)), Some(0.5));
    assert_eq!(c.noise_gain(&heard(4.0)), Some(0.375));
    assert_eq!(c.noise_gain(&heard(16.0)), Some(0.0), "heard iff d <= reach");
    assert_eq!(c.noise_gain(&heard(16.5)), None);

    let mut m = AwarenessMeter::new();
    assert_eq!(m.apply(&c, &noise(Some(PLAYER), true, 16.0, 4.0), at(0)), StimulusOutcome::Raised(Suspicious));
    let e = *m.get(PLAYER).unwrap();
    assert_eq!((e.level, e.last_seen.pos, e.last_seen.confidence, e.last_seen.velocity), (0.375, pos(2.0), 0.5, Vec3::ZERO));
    assert_eq!(m.apply(&c, &noise(Some(PLAYER), true, 16.0, 16.5), at(1)), StimulusOutcome::Unheard);
    // Capped at noise_cap (= the Searching threshold here): hearing alone never alerts.
    m.apply(&c, &noise(Some(PLAYER), true, 16.0, 0.0), at(1));
    m.apply(&c, &noise(Some(PLAYER), true, 16.0, 0.0), at(2));
    assert_eq!(level(&m, PLAYER), Some((0.625, Searching)));
    // A noise never lowers a higher sight level.
    m.apply(&c, &sight(PLAYER, 0.25), at(3));
    m.apply(&c, &noise(Some(PLAYER), true, 16.0, 0.0), at(4));
    assert_eq!(level(&m, PLAYER), Some((0.875, Searching)));
}

#[test]
fn non_revealing_noise_fills_the_distraction_slot_loudest_recent_wins() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    assert_eq!(m.apply(&c, &noise(Some(PLAYER), false, 10.0, 1.0), at(0)), StimulusOutcome::Distraction);
    assert_eq!(m.get(PLAYER), None, "a thrown cup does not reveal its thrower");
    assert_eq!(m.apply(&c, &noise(None, false, 6.0, 1.0), at(1)), StimulusOutcome::Ignored);
    assert_eq!(m.apply(&c, &noise(None, true, 12.0, 1.0), at(2)), StimulusOutcome::Distraction);
    assert_eq!(m.distraction().map(|d| (d.tick, d.loudness_m)), Some((2, 12.0)));
    // ttl 2 s = 16 ticks: then a quieter noise replaces it, and update clears it.
    assert_eq!(m.apply(&c, &noise(None, false, 6.0, 1.0), at(17)), StimulusOutcome::Ignored);
    assert_eq!(m.apply(&c, &noise(None, false, 6.0, 1.0), at(18)), StimulusOutcome::Distraction);
    m.update(&c, at(33));
    assert!(m.distraction().is_some());
    m.update(&c, at(34));
    assert_eq!(m.distraction(), None);
    m.apply(&c, &noise(None, false, 6.0, 1.0), at(35));
    assert_eq!(m.take_distraction().map(|d| d.at), Some(pos(2.0)));
    assert_eq!(m.distraction(), None);
}

#[test]
fn a_full_table_evicts_the_lowest_then_the_oldest() {
    let c = cfg(); // max_entries 3; decay waits 4 ticks, so levels hold over ticks 0-3
    let mut m = AwarenessMeter::new();
    m.apply(&c, &sight(actor(1), 0.5), at(0));
    m.apply(&c, &sight(actor(2), 0.25), at(1));
    m.apply(&c, &sight(actor(3), 0.375), at(2));
    // A weaker newcomer is evicted at once.
    assert_eq!(m.apply(&c, &sight(actor(4), 0.125), at(3)), StimulusOutcome::Evicted);
    assert_eq!(m.get(actor(4)), None);
    // A tie on level: the older gain (actor 2, tick 1) goes, the most recent stays.
    assert_eq!(m.apply(&c, &sight(actor(5), 0.25), at(3)), StimulusOutcome::Raised(Curious));
    let kept: Vec<_> = m.entries().iter().map(|e| e.target).collect();
    assert_eq!(kept, [actor(1), actor(3), actor(5)]);
    // Levels are compared as of now: by tick 20 actor 5 has decayed to the lowest (0.046 875).
    m.update(&c, at(3));
    assert_eq!(m.apply(&c, &sight(actor(6), 0.375), at(20)), StimulusOutcome::Raised(Suspicious));
    assert_eq!(m.get(actor(5)), None, "actor 5 (0.25 at tick 3) was lowest at tick 20");
    assert_eq!(m.strongest().map(|e| e.target), Some(actor(6)));
}

#[test]
fn sleepers_ignore_sight_and_wake_to_loud_noise_touch_and_damage_then_are_groggy() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    m.fall_asleep();
    assert_eq!(m.apply(&c, &sight(PLAYER, 1.0), at(0)), StimulusOutcome::Asleep);
    // Wake needs 8 m of reach left at the ears: 16 - 10 = 6 m does not wake.
    assert_eq!(m.apply(&c, &noise(Some(PLAYER), true, 16.0, 10.0), at(1)), StimulusOutcome::Unheard);
    assert!(m.asleep());
    assert_eq!(m.get(PLAYER), None);
    // 16 - 8 = 8 m wakes, and the noise still counts: 0.5 x (1 - 8/16).
    assert_eq!(m.apply(&c, &noise(Some(PLAYER), true, 16.0, 8.0), at(2)), StimulusOutcome::Raised(Curious));
    assert!(!m.asleep());
    assert_eq!(level(&m, PLAYER), Some((0.25, Curious)));
    // Groggy for 1 s = 8 ticks after waking.
    assert!(m.groggy(&c, at(9)));
    assert_eq!(m.alertness(&c, PLAYER, at(9)), 0.5);
    assert!(!m.groggy(&c, at(10)));
    assert_eq!(m.alertness(&c, PLAYER, at(10)), 1.0);

    let mut hit = AwarenessMeter::new();
    hit.fall_asleep();
    assert_eq!(hit.apply(&c, &Stimulus::Damage { by: None, at: pos(0.0) }, at(0)), StimulusOutcome::Woke);
    assert!(!hit.asleep() && hit.entries().is_empty());
    assert_eq!(hit.apply(&c, &Stimulus::Damage { by: Some(actor(9)), at: pos(3.0) }, at(1)), StimulusOutcome::Raised(Alerted));
    assert_eq!(level(&hit, actor(9)), Some((1.0, Alerted)));

    let mut touched = AwarenessMeter::new();
    touched.fall_asleep();
    assert_eq!(touched.apply(&c, &Stimulus::Touch { by: PLAYER, at: pos(0.5) }, at(0)), StimulusOutcome::Raised(Searching));
    assert!(!touched.asleep());
    assert_eq!(touched.get(PLAYER).map(|e| (e.level, e.last_seen.confidence)), Some((0.625, 1.0)));
}

#[test]
fn raise_to_floor_seeds_a_loaded_directive_band() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    let lkp = LastSeen { pos: pos(4.0), tick: 0, velocity: Vec3::ZERO, confidence: 0.5 };
    assert_eq!(m.raise_to_floor(&c, PLAYER, Searching, lkp, at(0)), StimulusOutcome::Raised(Searching));
    assert_eq!(m.get(PLAYER).map(|e| (e.level, e.band, e.last_seen)), Some((0.625, Searching, lkp)));
}

#[test]
fn player_detection_is_the_max_over_counting_actors() {
    let c = cfg();
    assert_eq!(player_detection(std::iter::empty(), P), PlayerDetection { level: 0.0, band: Unaware });
    // A: Searching held by hysteresis at 0.593 75; B: Suspicious at 0.6; C: sees another player.
    let mut a = AwarenessMeter::new();
    a.apply(&c, &sight(PLAYER, 0.625), at(0));
    a.update(&c, at(12));
    let mut b = AwarenessMeter::new();
    b.apply(&c, &sight(PLAYER, 0.6), at(12));
    let mut other = AwarenessMeter::new();
    other.apply(&c, &sight(PerceptTarget::Player(PlayerKey(8)), 0.9), at(12));
    assert_eq!(level(&a, PLAYER), Some((0.593_75, Searching)));
    assert_eq!(level(&b, PLAYER), Some((0.6, Suspicious)));
    let d = player_detection([&a, &b, &other], P);
    assert_eq!(d, PlayerDetection { level: 0.6, band: Searching });
    // Replicated to the client: serde round trip.
    let json = serde_json::to_string(&d).unwrap();
    assert_eq!(serde_json::from_str::<PlayerDetection>(&json).unwrap(), d);
}

#[test]
fn nan_infinite_and_negative_inputs_are_rejected() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    m.apply(&c, &sight(PLAYER, 0.25), at(0));
    let before = m.clone();
    let bad_pos = WorldPos { cell: IVec3::ZERO, local: Vec3::new(f32::NAN, 0.0, 0.0) };
    let bad = [
        sight(PLAYER, f32::NAN),
        sight(PLAYER, -0.1),
        sight(PLAYER, f32::INFINITY),
        Stimulus::Sight { target: PLAYER, gain: 0.1, pos: bad_pos, velocity: Vec3::ZERO },
        Stimulus::Sight { target: PLAYER, gain: 0.1, pos: pos(0.0), velocity: Vec3::splat(f32::NAN) },
        noise(Some(PLAYER), true, f32::NAN, 1.0),
        noise(Some(PLAYER), true, -4.0, 1.0),
        noise(Some(PLAYER), true, 16.0, f32::NAN),
        noise(Some(PLAYER), true, 16.0, -1.0),
        Stimulus::Touch { by: PLAYER, at: bad_pos },
        Stimulus::Damage { by: Some(PLAYER), at: bad_pos },
    ];
    for s in &bad {
        assert_eq!(m.apply(&c, s, at(1)), StimulusOutcome::Invalid, "{s:?}");
    }
    assert_eq!(m, before);
}

#[test]
fn the_same_stimuli_give_the_same_meter() {
    let c = cfg();
    let run = || {
        let mut m = AwarenessMeter::new();
        let mut outcomes = Vec::new();
        for t in 0..60_u64 {
            let n = u128::from(t % 5);
            outcomes.push(m.apply(&c, &sight(actor(n), 0.05 * f32::from(u8::try_from(t % 7).unwrap())), at(t)));
            if t % 3 == 0 {
                outcomes.push(m.apply(&c, &noise(Some(actor(n + 1)), t % 2 == 0, 20.0, 3.0), at(t)));
            }
            m.update(&c, at(t));
        }
        (m, outcomes)
    };
    let (a, oa) = run();
    let (b, ob) = run();
    assert_eq!(oa, ob);
    assert_eq!(a, b);
    assert!(a.entries().len() <= c.max_entries);
    assert!(a.entries().windows(2).all(|w| w[0].target < w[1].target), "entries stay sorted by target");
}

#[test]
fn defaults_match_the_spec_table() {
    let d = AwarenessConfig::default();
    assert_eq!((d.curious, d.suspicious, d.searching, d.band_hysteresis), (0.1, 0.35, 0.65, 0.05));
    assert_eq!((d.alert_hold_s, d.decay_delay_s), (6.0, 3.0));
    assert_eq!((d.decay_curious, d.decay_suspicious, d.decay_searching), (0.10, 0.05, 0.02));
    assert_eq!((d.noise_gain, d.noise_cap, d.noise_confidence), (0.5, 0.65, 0.5));
    assert_eq!((d.asleep_wake_m, d.groggy_s, d.max_entries), (12.0, 4.0, 8));
}
