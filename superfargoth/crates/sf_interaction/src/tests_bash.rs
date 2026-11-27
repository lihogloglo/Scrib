//! `IN37a` bash core: under the material's threshold no lock damage, over it `floor(force x kind factor)`;
//! bare-handed self damage, armed weapon wear only; contents risk from the raw force, capped; a non-finite or
//! negative force a zero result; the seed fixed and different per tick and user; `from_bashed` field for field.
#![allow(clippy::float_cmp)] // the values are exact in f32 (powers of two apart)

use std::collections::BTreeMap;

use glam::Vec3;
use sf_ids::{ActorKey, PlayerKey};
use sf_world::PlacementId;

use crate::*;

const BLUNT: TagId = TagId::of("blunt");
const BLADE: TagId = TagId::of("blade");
const HAND: TagId = TagId::of("hand");
const IRON: TagId = TagId::of("iron");
const WOOD: TagId = TagId::of("wood");
const DOOR: PlacementId = PlacementId(9);

fn rules() -> BashRules {
    BashRules {
        bare_kind: Some(HAND),
        kind_factor: BTreeMap::from([(BLUNT, 1.5), (BLADE, 0.5)]),
        threshold: BTreeMap::from([(IRON, 10.0), (WOOD, 2.0)]),
        self_per_force: 0.25,
        wear_per_force: 0.5,
        contents_risk_per_force: 0.02,
        contents_risk_max: 0.3,
    }
}

fn blow(force: f32, kind: TagId) -> Blow {
    Blow { force, kind, bare_handed: kind == HAND }
}

#[test]
fn threshold_gates_and_over_it_is_floor_of_force_times_factor() {
    let r = rules();
    // Blade on iron: 14 x 0.5 = 7 < 10.
    assert_eq!(bash(&r, &blow(14.0, BLADE), Some(IRON)).lock_damage, 0);
    // Blunt on iron: 7 x 1.5 = 10.5 >= 10 -> 10.
    assert_eq!(bash(&r, &blow(7.0, BLUNT), Some(IRON)).lock_damage, 10);
    // Exactly at the threshold counts: 20 x 0.5 = 10.
    assert_eq!(bash(&r, &blow(20.0, BLADE), Some(IRON)).lock_damage, 10);
    // Blade on wood: 14 x 0.5 = 7 -> 7; unknown kind factor 1: 13.9 -> 13.
    assert_eq!(bash(&r, &blow(14.0, BLADE), Some(WOOD)).lock_damage, 7);
    assert_eq!(bash(&r, &blow(13.9, TagId::of("pierce")), Some(WOOD)).lock_damage, 13);
    // No material / unknown material: threshold 0.
    assert_eq!(bash(&r, &blow(0.9, BLUNT), None).lock_damage, 1);
    assert_eq!(bash(&r, &blow(0.5, BLADE), Some(TagId::of("glass"))).lock_damage, 0);
    assert_eq!(bash(&r, &blow(3.0, BLADE), Some(TagId::of("glass"))).lock_damage, 1);
}

#[test]
fn neutral_default_is_floor_of_force() {
    let r = BashRules::default();
    let out = bash(&r, &blow(14.0, BLUNT), Some(IRON));
    assert_eq!(out, BashResult { lock_damage: 14, self_damage: 0.0, weapon_wear: 0.0, contents_risk: 0.0 });
}

#[test]
fn fists_hurt_the_basher_and_weapons_wear() {
    let r = rules();
    let fist = bash(&r, &blow(8.0, HAND), Some(WOOD));
    assert_eq!(fist, BashResult { lock_damage: 8, self_damage: 2.0, weapon_wear: 0.0, contents_risk: 0.16 });
    let armed = bash(&r, &blow(8.0, BLUNT), Some(WOOD));
    assert_eq!(armed, BashResult { lock_damage: 12, self_damage: 0.0, weapon_wear: 4.0, contents_risk: 0.16 });
    // Costs come even when the blow does nothing to the lock.
    let weak = bash(&r, &blow(4.0, BLADE), Some(IRON));
    assert_eq!(weak, BashResult { lock_damage: 0, self_damage: 0.0, weapon_wear: 2.0, contents_risk: 0.08 });
}

#[test]
fn contents_risk_uses_raw_force_and_is_capped() {
    let r = rules();
    // Raw force 8 (the blade factor 0.5 does not lower it): 0.16.
    assert_eq!(bash(&r, &blow(8.0, BLADE), Some(WOOD)).contents_risk, 0.16);
    assert_eq!(bash(&r, &blow(100.0, BLUNT), Some(WOOD)).contents_risk, 0.3);
}

#[test]
fn non_finite_or_negative_force_is_zero() {
    let r = rules();
    for f in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -5.0, 0.0] {
        assert_eq!(bash(&r, &blow(f, HAND), None), BashResult::default(), "force {f}");
        assert_eq!(bash(&r, &blow(f, BLUNT), Some(WOOD)), BashResult::default(), "force {f}");
    }
    // A huge factor saturates instead of wrapping.
    let mut big = rules();
    big.kind_factor.insert(BLUNT, f32::MAX);
    assert_eq!(bash(&big, &blow(10.0, BLUNT), None).lock_damage, 0, "an infinite product is no blow");
    big.kind_factor.insert(BLUNT, 1.0e30);
    assert_eq!(bash(&big, &blow(1.0, BLUNT), None).lock_damage, u32::MAX);
}

#[test]
fn seed_is_fixed_and_differs_per_tick_and_user() {
    let p = UserKey::Player(PlayerKey(1));
    assert_eq!(bash_seed(DOOR, p, Tick(100)), 0x5f0e_1e03_e0a2_f805);
    assert_eq!(bash_seed(DOOR, p, Tick(101)), 0x7950_549c_5688_2c20);
    assert_eq!(bash_seed(DOOR, UserKey::Actor(ActorKey(1)), Tick(100)), 0x15a7_247e_0f3b_d452);
    assert_ne!(bash_seed(PlacementId(10), p, Tick(100)), bash_seed(DOOR, p, Tick(100)));
}

#[test]
fn from_bashed_field_for_field() {
    let culprit = UserKey::Player(PlayerKey(1));
    let result = bash(&rules(), &blow(14.7, BLUNT), Some(WOOD));
    let e = Bashed {
        tick: Tick(100),
        placement: DOOR,
        culprit,
        kind: BLUNT,
        force: 14.7,
        result,
        point: Vec3::new(1.0, 0.0, 2.0),
        seed: bash_seed(DOOR, culprit, Tick(100)),
    };
    let site = ChangeSite {
        at: Vec3::new(10.0, 0.0, -4.0),
        owner: Some(ChangeOwner::Actor(ActorKey(42))),
        home: Some(PlacementId(3)),
    };
    assert_eq!(
        from_bashed(&e, &site),
        WorldChange {
            tick: Tick(100),
            kind: CHANGE_BASHED,
            object: DOOR,
            at: site.at,
            culprit: Some(culprit),
            owner: site.owner,
            home: site.home,
            cause: Some(BLUNT),
            detail: ChangeDetail::Force(14),
        }
    );
    // The raw force, not the lock damage: a blow under the threshold carries its force too.
    let weak = Bashed { force: 4.0, result: BashResult::default(), ..e };
    assert_eq!(from_bashed(&weak, &site).detail, ChangeDetail::Force(4));
    let nan = Bashed { force: f32::NAN, ..e };
    assert_eq!(from_bashed(&nan, &site).detail, ChangeDetail::Force(0));
    // Two blows on one door in one tick fold to the first.
    let mut v = vec![from_bashed(&e, &site), from_bashed(&weak, &site)];
    fold_tick(&mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].detail, ChangeDetail::Force(14));
}
