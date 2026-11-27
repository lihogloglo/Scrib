//! `IN19a` light as state: initial state from the def, snuff / relight and their refusals, rule
//! sets (event only on a change), the state tag, the one-byte rest state and serde.

use glam::Vec3;
use sf_ids::ActorKey;
use sf_world::PlacementId;

use crate::*;

const LAMP: PlacementId = PlacementId(11);
const AT: Vec3 = Vec3::new(1.0, 2.0, 3.0);

fn actor(n: u128) -> UserKey {
    UserKey::Actor(ActorKey(n))
}

fn def(lit_default: bool) -> LightDef {
    LightDef { radius_m: 4.5, intensity: 0.8, lit_default }
}

#[test]
fn initial_state_follows_lit_default() {
    assert_eq!(LightState::initial(&def(true)), LightState { lit: true });
    assert_eq!(LightState::initial(&def(false)), LightState { lit: false });
    assert_eq!(LightState::initial(&def(true)).tag(), Some(LIT));
    assert_eq!(LightState::initial(&def(false)).tag(), None);
}

#[test]
fn snuff_then_relight_emit_one_change_each() {
    let mut s = LightState::initial(&def(true));
    let out = s.use_verb(VERB_SNUFF, LAMP, AT, Some(actor(1)), Tick(5)).unwrap();
    assert_eq!(
        out,
        LightChanged { tick: Tick(5), placement: LAMP, lit: false, at: AT, culprit: Some(actor(1)) }
    );
    assert!(!s.lit);
    let back = s
        .use_verb(VERB_RELIGHT, LAMP, AT, Some(actor(2)), Tick(9))
        .unwrap();
    assert_eq!(
        back,
        LightChanged { tick: Tick(9), placement: LAMP, lit: true, at: AT, culprit: Some(actor(2)) }
    );
    assert!(s.lit);
}

#[test]
fn refusals_change_nothing() {
    let mut out = LightState { lit: false };
    assert_eq!(
        out.use_verb(VERB_SNUFF, LAMP, AT, Some(actor(1)), Tick(1)),
        Err(LightRefusal::AlreadyUnlit)
    );
    assert_eq!(out, LightState { lit: false });
    let mut on = LightState { lit: true };
    assert_eq!(
        on.use_verb(VERB_RELIGHT, LAMP, AT, Some(actor(1)), Tick(1)),
        Err(LightRefusal::AlreadyLit)
    );
    assert_eq!(on, LightState { lit: true });
    assert_eq!(
        on.use_verb(VERB_OPEN, LAMP, AT, Some(actor(1)), Tick(1)),
        Err(LightRefusal::NotALightVerb)
    );
    assert_eq!(on, LightState { lit: true });
}

#[test]
fn rule_set_emits_only_on_a_change() {
    let mut s = LightState { lit: true };
    assert_eq!(s.set(true, LAMP, AT, None, Tick(3)), None);
    assert_eq!(
        s.set(false, LAMP, AT, None, Tick(4)),
        Some(LightChanged { tick: Tick(4), placement: LAMP, lit: false, at: AT, culprit: None })
    );
    assert_eq!(s.set(false, LAMP, AT, None, Tick(5)), None);
    assert!(!s.lit);
}

#[test]
fn rest_byte_round_trips_and_rejects_unknown_bytes() {
    for lit in [false, true] {
        let s = LightState { lit };
        assert_eq!(LightState::from_rest_byte(&[s.rest_byte()]), Some(s));
    }
    assert_eq!(LightState { lit: true }.rest_byte(), 1);
    assert_eq!(LightState { lit: false }.rest_byte(), 0);
    for bad in [&[2u8][..], &[0xFF], &[], &[1, 1], &[0, 0]] {
        assert_eq!(LightState::from_rest_byte(bad), None, "{bad:?}");
    }
}

#[test]
fn light_kind_and_state_serde_round_trip() {
    let kind = SlotKind::Light(def(true));
    let text = ron::to_string(&kind).unwrap();
    assert_eq!(ron::from_str::<SlotKind>(&text).unwrap(), kind);
    assert_eq!(KindTag::of(&kind), KindTag::Light);
    let s = LightState { lit: true };
    let text = ron::to_string(&s).unwrap();
    assert_eq!(ron::from_str::<LightState>(&text).unwrap(), s);
}
