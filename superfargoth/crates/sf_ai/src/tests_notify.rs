//! ai.md "Perception and stimuli" → "Awareness" → Notify (AI8e): a shout over a fixed 3-room
//! graph (exact arrival ticks and confidences, a relay with hops + 1, the hop limit, the
//! Searching cap, the asleep rule), the merge and ordering rules, serde of a meter with a pending
//! delivery, and 30 vs 144 Hz timing. The test config is dyadic (1/8 s tick, 8 m/s, 0.5 s, 0.5),
//! so arrival ticks and confidences are exact.
#![allow(clippy::float_cmp)]

use glam::{IVec3, Vec3};
use sf_coords::WorldPos;
use sf_ids::{ActorKey, PlayerKey};

use crate::*;

const TICK_S: f32 = 0.125;
const PLAYER: PerceptTarget = PerceptTarget::Player(PlayerKey(7));
/// The shout's reach at the listeners' ears (the caller's `loudness_m × hearing_mul`), meters.
const SHOUT_M: f32 = 40.0;
/// Fixed path distances from room A, closed doors included (`closed_door_m` = 8 per door):
/// B is one door away (4 m + 8), C two doors away (12 m + 16), D beyond the shout's reach.
const AB_M: f32 = 12.0;
const AC_M: f32 = 28.0;
const AD_M: f32 = 44.0;
/// C to D (one door), for C's relay.
const CD_M: f32 = 20.0;

fn cfg() -> AwarenessConfig {
    AwarenessConfig {
        curious: 0.125,
        suspicious: 0.375,
        searching: 0.625,
        band_hysteresis: 0.0625,
        asleep_wake_m: 8.0,
        max_entries: 3,
        notify_speed_mps: 8.0,
        notify_react_s: 0.5,
        notify_confidence_mul: 0.5,
        notify_max_hops: 3,
        ..AwarenessConfig::default()
    }
}

fn at(tick: u64) -> PerceptClock {
    PerceptClock::new(tick, TICK_S)
}

fn pos(x: f32) -> WorldPos {
    WorldPos::new(IVec3::ZERO, Vec3::new(x, 0.0, 0.0))
}

fn alarm(about: PerceptTarget, confidence: f32, hops: u8) -> Notify {
    Notify { about, lkp: pos(5.0), lkp_tick: 96, confidence, hops }
}

fn shout(notify: Notify, distance_m: f32) -> Stimulus {
    let noise = HeardNoise { source: None, reveals_source: true, at: pos(0.0), loudness_m: SHOUT_M, distance_m };
    Stimulus::Notify { noise, notify }
}

/// Runs `update_with` at `tick`, returning the deliveries.
fn tick(m: &mut AwarenessMeter, c: &AwarenessConfig, tick: u64) -> Vec<NotifyDelivered> {
    let mut out = Vec::new();
    m.update_with(c, at(tick), |d| out.push(*d));
    out
}

fn level(m: &AwarenessMeter, t: PerceptTarget) -> (f32, AwarenessBand) {
    let e = m.get(t).expect("entry");
    (e.level, e.band)
}

#[test]
fn a_shout_reaches_b_then_c_at_exact_ticks_and_c_relays_with_one_more_hop() {
    let c = cfg();
    let sent = alarm(PLAYER, 1.0, 0);
    let (mut b, mut cc, mut d) = (AwarenessMeter::new(), AwarenessMeter::new(), AwarenessMeter::new());
    // A shouts at tick 100. B: 12 / 8 + 0.5 = 2 s = 16 ticks; C: 28 / 8 + 0.5 = 4 s = 32 ticks.
    assert_eq!(b.apply(&c, &shout(sent, AB_M), at(100)), StimulusOutcome::Queued { due: 116 });
    assert_eq!(cc.apply(&c, &shout(sent, AC_M), at(100)), StimulusOutcome::Queued { due: 132 });
    // D is beyond the shout's reach: no telepathy, nothing queued.
    assert_eq!(d.apply(&c, &shout(sent, AD_M), at(100)), StimulusOutcome::Unheard);
    assert!(d.pending().is_empty());

    // Nothing lands early; B lands exactly on 116.
    assert!(tick(&mut b, &c, 115).is_empty());
    assert!(b.get(PLAYER).is_none());
    let got = tick(&mut b, &c, 116);
    let received = Notify { confidence: 0.5, ..sent };
    assert_eq!(got, vec![NotifyDelivered { due: 116, notify: received, outcome: StimulusOutcome::Raised(AwarenessBand::Suspicious) }]);
    assert_eq!(level(&b, PLAYER), (0.5, AwarenessBand::Suspicious));
    let lkp = b.get(PLAYER).unwrap().last_seen;
    assert_eq!((lkp.pos, lkp.tick, lkp.velocity, lkp.confidence), (pos(5.0), 96, Vec3::ZERO, 0.5));
    assert!(b.pending().is_empty());

    // C lands on 132 and (the caller decides) relays it: hops 1, its received confidence.
    assert!(tick(&mut cc, &c, 131).is_empty());
    let got = tick(&mut cc, &c, 132);
    assert_eq!(got.len(), 1);
    assert_eq!(level(&cc, PLAYER), (0.5, AwarenessBand::Suspicious));
    let relay = got[0].notify.relay();
    assert_eq!((relay.hops, relay.confidence, relay.lkp_tick), (1, 0.5, 96));
    // D hears C's relay 20 m away: 20 / 8 + 0.5 = 3 s = 24 ticks, confidence 0.5 × 0.5.
    assert_eq!(d.apply(&c, &shout(relay, CD_M), at(132)), StimulusOutcome::Queued { due: 156 });
    let got = tick(&mut d, &c, 156);
    assert_eq!((got[0].notify.hops, got[0].notify.confidence), (1, 0.25));
    assert_eq!(level(&d, PLAYER), (0.25, AwarenessBand::Curious));
}

#[test]
fn a_notify_past_the_hop_limit_is_dropped() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 1.0, 4), AB_M), at(0)), StimulusOutcome::Ignored);
    assert!(m.pending().is_empty());
    // At the limit it still lands; one relay later it no longer does.
    let last = alarm(PLAYER, 1.0, 3);
    assert_eq!(m.apply(&c, &shout(last, AB_M), at(0)), StimulusOutcome::Queued { due: 16 });
    let mut other = AwarenessMeter::new();
    assert_eq!(other.apply(&c, &shout(last.relay(), AB_M), at(0)), StimulusOutcome::Ignored);
}

#[test]
fn a_notify_raises_at_most_to_searching_even_at_full_confidence() {
    let c = AwarenessConfig { notify_confidence_mul: 1.0, ..cfg() };
    let mut m = AwarenessMeter::new();
    for t in [0, 20, 40] {
        m.apply(&c, &shout(alarm(PLAYER, 1.0, 0), AB_M), at(t));
        tick(&mut m, &c, t + 16);
        assert_eq!(level(&m, PLAYER), (0.625, AwarenessBand::Searching));
    }
    assert_eq!(m.get(PLAYER).unwrap().last_seen.confidence, 1.0);
}

#[test]
fn an_existing_entry_keeps_the_higher_level_and_its_fresher_lkp() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    // Seen at tick 98 (fresher than the alarm's tick 96 LKP) and fully alerted.
    let seen = Stimulus::Sight { target: PLAYER, gain: 1.0, pos: pos(9.0), velocity: Vec3::X };
    assert_eq!(m.apply(&c, &seen, at(98)), StimulusOutcome::Raised(AwarenessBand::Alerted));
    m.apply(&c, &shout(alarm(PLAYER, 1.0, 0), AB_M), at(100));
    let got = tick(&mut m, &c, 116);
    assert_eq!(got[0].outcome, StimulusOutcome::Raised(AwarenessBand::Alerted));
    assert_eq!(level(&m, PLAYER), (1.0, AwarenessBand::Alerted));
    assert_eq!(m.get(PLAYER).unwrap().last_seen.pos, pos(9.0));
}

#[test]
fn one_pending_per_target_higher_confidence_wins_and_equal_ticks_deliver_in_target_order() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    let (a1, a2) = (PerceptTarget::Actor(ActorKey(1)), PerceptTarget::Actor(ActorKey(2)));
    // A weaker, earlier relay is replaced by the stronger direct shout; a weaker one after is ignored.
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 0.5, 1), 4.0), at(0)), StimulusOutcome::Queued { due: 8 });
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 1.0, 0), AB_M), at(0)), StimulusOutcome::Queued { due: 16 });
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 0.5, 0), 4.0), at(1)), StimulusOutcome::Ignored);
    assert_eq!(m.pending().len(), 1);
    // Same due tick, applied in reverse target order: delivered in (due, target) order.
    m.apply(&c, &shout(alarm(a2, 1.0, 0), AB_M), at(0));
    m.apply(&c, &shout(alarm(a1, 1.0, 0), AB_M), at(0));
    // max_entries (3) targets pending: a fourth is refused (bounded).
    let a3 = PerceptTarget::Actor(ActorKey(3));
    assert_eq!(m.apply(&c, &shout(alarm(a3, 1.0, 0), AB_M), at(0)), StimulusOutcome::Ignored);
    let order: Vec<_> = tick(&mut m, &c, 16).iter().map(|d| d.notify.about).collect();
    assert_eq!(order, vec![PLAYER, a1, a2]);
}

#[test]
fn a_sleeper_gets_a_notify_only_from_a_shout_loud_enough_to_wake() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    m.fall_asleep();
    // 40 - 36 = 4 m of reach left < asleep_wake_m (8): heard by nobody awake, so nothing.
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 1.0, 0), 36.0), at(0)), StimulusOutcome::Unheard);
    assert!(m.asleep() && m.pending().is_empty());
    // 40 - 24 = 16 m ≥ 8: wakes, and the news lands 24 / 8 + 0.5 = 3.5 s = 28 ticks later.
    assert_eq!(m.apply(&c, &shout(alarm(PLAYER, 1.0, 0), 24.0), at(0)), StimulusOutcome::Queued { due: 28 });
    assert!(!m.asleep());
}

#[test]
fn a_meter_with_a_pending_delivery_round_trips_through_serde() {
    let c = cfg();
    let mut m = AwarenessMeter::new();
    m.apply(&c, &Stimulus::Sight { target: PLAYER, gain: 0.25, pos: pos(1.0), velocity: Vec3::X }, at(10));
    m.apply(&c, &shout(alarm(PerceptTarget::Actor(ActorKey(4)), 1.0, 2), AB_M), at(10));
    assert_eq!(m.pending().len(), 1);
    let json: AwarenessMeter = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
    assert_eq!(json, m);
    let bin: AwarenessMeter = postcard::from_bytes(&postcard::to_allocvec(&m).unwrap()).unwrap();
    assert_eq!(bin, m);
    // And it still delivers after the round trip.
    let mut back = json;
    assert_eq!(tick(&mut back, &c, 26).len(), 1);
}

#[test]
fn arrival_time_agrees_at_30_and_144_hz_within_one_tick() {
    #[allow(clippy::cast_precision_loss)]
    fn arrival_s(c: &AwarenessConfig, distance_m: f32, tick_s: f32) -> f32 {
        let mut m = AwarenessMeter::new();
        m.apply(c, &shout(alarm(PLAYER, 1.0, 0), distance_m), PerceptClock::new(0, tick_s));
        // Update every tick until it lands.
        let landed = (0..10_000u64)
            .find(|&t| {
                let mut hit = false;
                m.update_with(c, PerceptClock::new(t, tick_s), |_| hit = true);
                hit
            })
            .expect("delivered");
        landed as f32 * tick_s
    }
    for c in [cfg(), AwarenessConfig::default()] {
        for distance_m in [0.0, AB_M, AC_M, 37.0] {
            let exact = distance_m / c.notify_speed_mps + c.notify_react_s;
            let (s30, s144) = (arrival_s(&c, distance_m, 1.0 / 30.0), arrival_s(&c, distance_m, 1.0 / 144.0));
            // ceil: never early, late by less than one tick.
            for (s, tick_s) in [(s30, 1.0 / 30.0), (s144, 1.0 / 144.0)] {
                assert!(s >= exact - 1e-4 && s < exact + tick_s + 1e-4, "{distance_m} m at {tick_s}: {s} vs {exact}");
            }
            assert!((s30 - s144).abs() < 1.0 / 30.0 + 1e-4);
        }
    }
}

#[test]
fn notify_defaults_match_the_spec_table() {
    let d = AwarenessConfig::default();
    assert_eq!((d.notify_speed_mps, d.notify_react_s, d.notify_confidence_mul, d.notify_max_hops), (6.0, 0.8, 0.7, 3));
}
