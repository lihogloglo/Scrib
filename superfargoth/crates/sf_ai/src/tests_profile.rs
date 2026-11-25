//! ai.md "Perception and stimuli" → "Perception profile (per individual)" (AI8g): the identity
//! resolve (bit-equal to the globals), multiplying states, clamps and the 180° cone cap, refresh
//! without stacking, tick expiry and `next_expiry`, the full-array replacement rule, conditions,
//! `validate` by name, and serde (json + postcard). Ratios are dyadic, so products are exact.
#![allow(clippy::float_cmp)]

use std::collections::HashMap;

use sf_records::RecordId;

use crate::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn globals() -> (AwarenessConfig, SightConfig, SearchConfig) {
    (AwarenessConfig::default(), SightConfig::default(), SearchConfig::default())
}

fn resolve_with(
    profile: &PerceptionProfile,
    states: &PerceptionStates,
    db: &HashMap<RecordId, PerceptionState>,
    now: u64,
) -> EffectivePerception {
    let (cfg, sight, search) = globals();
    resolve(profile, states, |k| db.get(k), &cfg, &sight, &search, now)
}

fn db() -> HashMap<RecordId, PerceptionState> {
    HashMap::from([
        (id("drunk"), PerceptionState { sight_range: Some(0.5), alertness: Some(2.0), decay: Some(0.5), ..Default::default() }),
        (id("tired"), PerceptionState { alertness: Some(0.75), sight_gain: Some(1.5), ..Default::default() }),
        (id("eagle"), PerceptionState { sight_range: Some(4.0), sight_cone: Some(2.0), darkness: Some(2.0), hearing: Some(4.0), ..Default::default() }),
        (id("sharp"), PerceptionState { sight_range: Some(2.0), ..Default::default() }),
    ])
}

#[test]
fn identity_profile_resolves_bit_equal_to_globals() {
    // A non-default config too, so nothing is hard-wired to the defaults.
    let custom = AwarenessConfig { decay_curious: 0.3, noise_gain: 0.7, ..AwarenessConfig::default() };
    let sight = SightConfig { sight_range_m: 41.3, rate: 1.7, ..SightConfig::default() };
    for (cfg, sight) in [(AwarenessConfig::default(), SightConfig::default()), (custom, sight)] {
        let search = SearchConfig::default();
        let eff = resolve(&PerceptionProfile::default(), &PerceptionStates::new(), |_| None, &cfg, &sight, &search, 0);
        // postcard writes f32 as its raw bytes: equal bytes = bit-equal fields.
        assert_eq!(postcard::to_stdvec(&eff.cfg).unwrap(), postcard::to_stdvec(&cfg).unwrap());
        let bits = |x: f32| x.to_bits();
        assert_eq!(bits(eff.sight.range_m), bits(sight.sight_range_m));
        assert_eq!(bits(eff.sight.lose_m), bits(sight.lose_sight_range_m));
        assert_eq!(bits(eff.sight.central_half_deg), bits(sight.central_half_deg));
        assert_eq!(bits(eff.sight.peripheral_half_deg), bits(sight.peripheral_half_deg));
        assert_eq!(bits(eff.sight.rate), bits(sight.rate));
        assert_eq!(bits(eff.sight.darkness), bits(1.0));
        assert_eq!(bits(eff.hearing_mul), bits(1.0));
        assert_eq!(bits(eff.search.ttl_s), bits(search.search_ttl_s));
        assert_eq!(bits(eff.search.error_m), bits(search.search_error_m));
        assert_eq!(bits(eff.search.widen_mps), bits(search.search_widen_mps));
        assert_eq!(eff.next_expiry, None);
        assert!(!eff.needs_rebuild(u64::MAX));
    }
}

#[test]
fn global_defaults_match_the_spec_table() {
    let s = SightConfig::default();
    assert_eq!(
        (s.sight_range_m, s.lose_sight_range_m, s.near_m, s.central_half_deg, s.peripheral_half_deg, s.peripheral_mul, s.rate),
        (30.0, 36.0, 3.0, 30.0, 80.0, 0.35, 2.0)
    );
    let q = SearchConfig::default();
    assert_eq!((q.search_ttl_s, q.search_error_m, q.search_widen_mps), (30.0, 3.0, 0.5));
}

#[test]
fn profile_and_two_states_multiply_and_scale_the_hooks() {
    let (cfg, sight, search) = globals();
    let profile = PerceptionProfile { sight_range: 2.0, search_ttl: 0.5, search_error: 2.0, ..Default::default() };
    let mut st = PerceptionStates::new();
    st.push(id("drunk"), None, None);
    st.push(id("tired"), Some(100), Some(id("long_day")));
    let eff = resolve_with(&profile, &st, &db(), 0);
    // sight_range 2 × 0.5; alertness 2 × 0.75; sight_gain 1.5; decay 0.5.
    assert_eq!(eff.sight.range_m, sight.sight_range_m);
    assert_eq!(eff.sight.lose_m, sight.lose_sight_range_m);
    assert_eq!(eff.sight.rate, sight.rate * 1.5 * 1.5);
    assert_eq!(eff.cfg.noise_gain, cfg.noise_gain * 1.5);
    assert_eq!(eff.cfg.decay_curious, cfg.decay_curious * 0.5);
    assert_eq!(eff.cfg.decay_suspicious, cfg.decay_suspicious * 0.5);
    assert_eq!(eff.cfg.decay_searching, cfg.decay_searching * 0.5);
    assert_eq!(eff.search.ttl_s, search.search_ttl_s * 0.5);
    assert_eq!(eff.search.error_m, search.search_error_m * 2.0);
    assert_eq!(eff.search.widen_mps, search.search_widen_mps * 2.0);
    assert_eq!(eff.next_expiry, Some(100));
    // Never scaled.
    let unscaled = |c: &AwarenessConfig| {
        (c.curious, c.suspicious, c.searching, c.band_hysteresis, c.alert_hold_s, c.noise_cap, c.max_entries)
    };
    assert_eq!(unscaled(&eff.cfg), unscaled(&cfg));
}

#[test]
fn fields_clamp_at_their_range_ends() {
    let (_, sight, _) = globals();
    let profile = PerceptionProfile { sight_range: 4.0, sight_cone: 2.0, darkness: 2.0, hearing: 4.0, ..Default::default() };
    let mut st = PerceptionStates::new();
    st.push(id("eagle"), None, None);
    let eff = resolve_with(&profile, &st, &db(), 0);
    assert_eq!(eff.sight.range_m, sight.sight_range_m * 4.0);
    assert_eq!(eff.sight.darkness, 2.0);
    assert_eq!(eff.hearing_mul, 4.0);
    assert_eq!(eff.sight.central_half_deg, sight.central_half_deg * 2.0);
    // The low end: a 0 profile field stays 0 whatever multiplies it.
    let blind = PerceptionProfile { sight_range: 0.0, darkness: 0.0, ..Default::default() };
    let eff = resolve_with(&blind, &st, &db(), 0);
    assert_eq!((eff.sight.range_m, eff.sight.darkness), (0.0, 0.0));
}

#[test]
fn cone_half_angles_cap_at_180() {
    let (cfg, _, search) = globals();
    let sight = SightConfig { peripheral_half_deg: 120.0, ..SightConfig::default() };
    let profile = PerceptionProfile { sight_cone: 2.0, ..Default::default() };
    let eff = resolve(&profile, &PerceptionStates::new(), |_| None, &cfg, &sight, &search, 0);
    assert_eq!(eff.sight.central_half_deg, 60.0);
    assert_eq!(eff.sight.peripheral_half_deg, 180.0);
}

#[test]
fn refresh_never_stacks_and_later_until_wins() {
    let mut st = PerceptionStates::new();
    assert_eq!(st.push(id("sharp"), Some(10), Some(id("potion_a"))), PushOutcome::Added);
    assert_eq!(st.push(id("Sharp"), Some(5), Some(id("potion_b"))), PushOutcome::Refreshed);
    let e = st.get(&id("sharp")).unwrap();
    assert_eq!((e.until, e.source.clone()), (Some(10), Some(id("potion_a"))));
    assert_eq!(st.push(id("sharp"), Some(20), Some(id("potion_c"))), PushOutcome::Refreshed);
    let e = st.get(&id("sharp")).unwrap();
    assert_eq!((e.until, e.source.clone()), (Some(20), Some(id("potion_c"))));
    // A condition is the latest of all: it wins over a timed entry, and a timed push can't undo it.
    assert_eq!(st.push(id("sharp"), None, None), PushOutcome::Refreshed);
    assert_eq!(st.push(id("sharp"), Some(99), None), PushOutcome::Refreshed);
    assert_eq!(st.get(&id("sharp")).unwrap().until, None);
    assert_eq!(st.len(), 1);
    // Applied once: × 2, not × 2⁴.
    let eff = resolve_with(&PerceptionProfile::default(), &st, &db(), 0);
    assert_eq!(eff.sight.range_m, SightConfig::default().sight_range_m * 2.0);
}

#[test]
fn expiry_at_tick_removes_and_next_expiry_moves() {
    let mut st = PerceptionStates::new();
    st.push(id("sharp"), Some(10), None);
    st.push(id("drunk"), Some(20), None);
    let eff = resolve_with(&PerceptionProfile::default(), &st, &db(), 0);
    assert_eq!(eff.next_expiry, Some(10));
    assert_eq!(st.next_expiry(), Some(10));
    assert!(!eff.needs_rebuild(9));
    assert!(eff.needs_rebuild(10));
    // At tick 10 the entry is already gone from the resolve, even before `expire`.
    let at10 = resolve_with(&PerceptionProfile::default(), &st, &db(), 10);
    assert_eq!(at10.sight.range_m, SightConfig::default().sight_range_m * 0.5);
    assert_eq!(st.expire(9), 0);
    assert_eq!(st.expire(10), 1);
    assert!(st.get(&id("sharp")).is_none());
    let eff = resolve_with(&PerceptionProfile::default(), &st, &db(), 10);
    assert_eq!(eff, at10);
    assert_eq!(eff.next_expiry, Some(20));
    assert_eq!(st.expire(20), 1);
    assert!(st.is_empty());
    assert_eq!(st.next_expiry(), None);
}

#[test]
fn full_array_replaces_the_entry_expiring_first() {
    let mut st = PerceptionStates::new();
    for (s, until) in [("a", Some(30)), ("b", Some(10)), ("c", None), ("d", Some(20))] {
        assert_eq!(st.push(id(s), until, None), PushOutcome::Added);
    }
    assert_eq!(st.len(), PERCEPTION_STATES_CAPACITY);
    let out = st.push(id("e"), Some(40), None);
    assert_eq!(out, PushOutcome::Replaced(StateEntry { state: id("b"), until: Some(10), source: None }));
    // A new condition replaces a timed entry too (the earliest: d at 20).
    let PushOutcome::Replaced(old) = st.push(id("f"), None, None) else { panic!("expected a replacement") };
    assert_eq!(old.state, id("d"));
    // Tie on `until`: the lowest slot goes.
    let mut tie = PerceptionStates::new();
    for s in ["p", "q", "r", "s"] {
        tie.push(id(s), Some(5), None);
    }
    let PushOutcome::Replaced(old) = tie.push(id("t"), Some(1), None) else { panic!("expected a replacement") };
    assert_eq!(old.state, id("p"));
    // All conditions: refused, nothing changed.
    let mut held = PerceptionStates::new();
    for s in ["w", "x", "y", "z"] {
        held.push(id(s), None, None);
    }
    let before = held.clone();
    assert_eq!(held.push(id("v"), Some(1), None), PushOutcome::Full);
    assert_eq!(held, before);
}

#[test]
fn conditions_never_expire_and_clear_removes() {
    let mut st = PerceptionStates::new();
    st.push(id("drunk"), None, Some(id("tavern")));
    st.push(id("sharp"), Some(3), None);
    assert_eq!(st.expire(u64::MAX), 1);
    assert!(st.get(&id("drunk")).is_some());
    assert_eq!(st.next_expiry(), None);
    let eff = resolve_with(&PerceptionProfile::default(), &st, &db(), u64::MAX);
    assert_eq!(eff.next_expiry, None);
    assert_eq!(eff.sight.range_m, SightConfig::default().sight_range_m * 0.5);
    assert!(st.clear(&id("DRUNK")));
    assert!(!st.clear(&id("drunk")));
    assert!(st.is_empty());
}

#[test]
fn unknown_state_ids_count_as_one() {
    let mut st = PerceptionStates::new();
    st.push(id("no_such_state"), Some(7), None);
    let eff = resolve_with(&PerceptionProfile::default(), &st, &db(), 0);
    let base = resolve_with(&PerceptionProfile::default(), &PerceptionStates::new(), &db(), 0);
    assert_eq!(eff, base);
    assert_eq!(eff.next_expiry, None);
}

#[test]
fn state_until_from_duration() {
    let rec = PerceptionState { duration_s: Some(1.0), ..Default::default() };
    assert_eq!(rec.until(PerceptClock::new(100, 0.125)), Some(108));
    let rec = PerceptionState { duration_s: Some(0.3), ..Default::default() };
    assert_eq!(rec.until(PerceptClock::new(0, 0.125)), Some(3));
    assert_eq!(PerceptionState::default().until(PerceptClock::new(5, 0.125)), None);
}

#[test]
fn validate_reports_out_of_range_fields_by_name() {
    assert!(PerceptionProfile::default().validate().is_ok());
    let edges = PerceptionProfile { sight_range: 4.0, sight_cone: 2.0, darkness: 0.0, decay: 0.0, ..Default::default() };
    assert!(edges.validate().is_ok());
    let bad = PerceptionProfile { sight_range: 4.5, sight_cone: 2.5, darkness: f32::NAN, search_error: -0.5, ..Default::default() };
    let err = bad.validate().unwrap_err();
    assert_eq!(err.names(), ["sight_range", "sight_cone", "darkness", "search_error"]);
    assert!(err.to_string().contains("sight_cone = 2.5 (0-2)"));
    let bad_state = PerceptionState { hearing: Some(4.5), alertness: Some(1.0), duration_s: Some(-1.0), ..Default::default() };
    assert_eq!(bad_state.validate().unwrap_err().names(), ["hearing", "duration_s"]);
    assert!(PerceptionState { darkness: Some(2.0), duration_s: Some(0.0), ..Default::default() }.validate().is_ok());
}

#[test]
fn serde_round_trips_profile_and_states() {
    let profile = PerceptionProfile { sight_range: 1.25, darkness: 0.0, search_ttl: 3.0, ..Default::default() };
    let json = serde_json::to_string(&profile).unwrap();
    assert_eq!(serde_json::from_str::<PerceptionProfile>(&json).unwrap(), profile);
    let bytes = postcard::to_stdvec(&profile).unwrap();
    assert_eq!(postcard::from_bytes::<PerceptionProfile>(&bytes).unwrap(), profile);
    // Records may omit fields (1.0); a typo is a load error.
    let partial: PerceptionProfile = serde_json::from_str(r#"{ "hearing": 2.0 }"#).unwrap();
    assert_eq!(partial, PerceptionProfile { hearing: 2.0, ..Default::default() });
    assert!(serde_json::from_str::<PerceptionProfile>(r#"{ "hearin": 2.0 }"#).is_err());

    let rec: PerceptionState = serde_json::from_str(r#"{ "alertness": 0.5, "duration_s": 4.0 }"#).unwrap();
    assert_eq!(rec, PerceptionState { alertness: Some(0.5), duration_s: Some(4.0), ..Default::default() });
    assert_eq!(serde_json::from_str::<PerceptionState>(&serde_json::to_string(&rec).unwrap()).unwrap(), rec);
    assert_eq!(postcard::from_bytes::<PerceptionState>(&postcard::to_stdvec(&rec).unwrap()).unwrap(), rec);

    let mut st = PerceptionStates::new();
    st.push(id("drunk"), None, Some(id("tavern")));
    st.push(id("sharp"), Some(42), None);
    let json = serde_json::to_string(&st).unwrap();
    assert_eq!(serde_json::from_str::<PerceptionStates>(&json).unwrap(), st);
    let bytes = postcard::to_stdvec(&st).unwrap();
    assert_eq!(postcard::from_bytes::<PerceptionStates>(&bytes).unwrap(), st);
}
