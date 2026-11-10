//! stats.md "Tests" for S1: exact assertions on a small content table and the [`TestModel`].

// Exact float assertions are the point (CONTRIBUTING "Tests": logic and data are exact).
#![allow(clippy::float_cmp, clippy::many_single_char_names)]

use std::path::Path;

use serde_reflection::{Samples, Tracer, TracerConfig};
use sf_records::RecordId;
use smallvec::smallvec;

use crate::*;

const STR: StatIdx = StatIdx(0);
const BLADE: StatIdx = StatIdx(1);
const HEALTH: StatIdx = StatIdx(2);
const MANA: StatIdx = StatIdx(3);
const STAMINA: StatIdx = StatIdx(4);
const LEVEL: StatIdx = StatIdx(5);
const PROGRESS: StatIdx = StatIdx(6);
const TALLY: StatIdx = StatIdx(7);
const CAPACITY: StatIdx = StatIdx(8);
const BOUNTY: StatIdx = StatIdx(9);

fn def(key: &str, kind: StatKind) -> StatDef {
    StatDef { key: key.into(), kind, group: "test".into(), public: false, label: format!("stat.{key}") }
}

fn table() -> StatTable {
    table_with(0)
}

/// The test table plus `extra` value stats at the end (a game-sized table for the measurement).
fn table_with(extra: usize) -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let mut defs = vec![
        def("Str", value),
        def("blade", StatKind::Value { min: 0.0, cap: Some(100.0) }),
        def("health", StatKind::Pool { depleted_below: 1.0, rebase: Rebase::KeepCurrent }),
        def("mana", StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepRatio }),
        def("stamina", StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepDeficit }),
        def("level", StatKind::Counter { min: 1.0, max: 100.0 }),
        def("blade_progress", StatKind::Counter { min: 0.0, max: 1000.0 }),
        def("tally", StatKind::Counter { min: 0.0, max: 100.0 }),
        def("capacity", StatKind::Derived),
        def("bounty", StatKind::Counter { min: 0.0, max: 1.0e9 }),
    ];
    let roles = [
        (Role::Health, "health".into()),
        (Role::Mana, "mana".into()),
        (Role::Stamina, "stamina".into()),
        (Role::Level, "level".into()),
        (Role::CarryCapacity, "capacity".into()),
        (Role::Bounty, "bounty".into()),
    ];
    defs.extend((0..extra).map(|i| def(&format!("extra{i}"), value)));
    StatTable::new(defs, &roles).unwrap()
}

fn model() -> TestModel {
    TestModel {
        pool_from: vec![(MANA, STR, 2.0)],
        derived_from: vec![(CAPACITY, STR, 5.0)],
        regen: vec![(HEALTH, 1.0), (STAMINA, 10.0)],
        rest: vec![(HEALTH, 10.0)],
        progress: vec![(BLADE, PROGRESS)],
        threshold: 3.0,
        skill_cap: 100.0,
        tally: Some(TALLY),
        tally_needed: 2.0,
        picks: 2,
        pickable: vec![STR, BLADE],
        pick_gain: 5.0,
    }
}

const OWNER: StatOwner = StatOwner::Actor(ActorKey(7));

fn cause(kind: CauseKind) -> Cause {
    Cause { kind, source: None, tick: 3 }
}

/// str 50, blade 20, health 100, mana 100 (2 × str), stamina 80, level 1; pools full.
fn setup() -> (StatTable, Rules, Sheet) {
    setup_with(table())
}

fn setup_with(t: StatTable) -> (StatTable, Rules, Sheet) {
    let r = model().rules();
    let mut s = Sheet::new(&t);
    let init = [
        StatOp::SetBase { stat: STR, value: 50.0 },
        StatOp::SetBase { stat: BLADE, value: 20.0 },
        StatOp::SetBase { stat: HEALTH, value: 100.0 },
        StatOp::SetBase { stat: STAMINA, value: 80.0 },
        StatOp::CounterSet { stat: LEVEL, value: 1.0 },
    ];
    let mut ev = Vec::new();
    assert_eq!(run(&t, &r, &mut s, &init, &mut ev), 0);
    s.settle(&t, &r, &Inputs::default(), Settle::NewCharacter);
    assert!(!s.changed_since_init());
    (t, r, s)
}

fn run(t: &StatTable, r: &Rules, s: &mut Sheet, ops: &[StatOp], out: &mut Vec<StatEvent>) -> usize {
    let inputs = Inputs::default();
    apply(t, r, s, ops, &ApplyCtx { owner: OWNER, cause: cause(CauseKind::Debug), inputs: &inputs }, out)
}

fn ops(t: &StatTable, r: &Rules, s: &mut Sheet, ops: &[StatOp]) -> Vec<StatEvent> {
    let mut out = Vec::new();
    assert_eq!(run(t, r, s, ops, &mut out), 0, "no op rejected");
    out
}

fn src(id: u64) -> SourceId {
    SourceId { kind: SourceKind::Effect, id }
}

fn modifier(stat: StatIdx, layer: Layer, op: ModOp, value: f32, id: u64) -> StatOp {
    StatOp::AddModifier(Modifier { stat, layer, op, value, source: src(id) })
}

fn bytes(s: &Sheet) -> Vec<u8> {
    postcard::to_allocvec(s).unwrap()
}

#[test]
fn table_folds_keys_and_checks_roles() {
    let t = table();
    assert_eq!(t.find("STR"), Some(STR));
    assert_eq!(t.def(STR).key.as_str(), "str");
    assert_eq!(t.role(Role::Mana), Some(MANA));
    assert_eq!(t.role(Role::Encumbrance), None);
    let dup = StatTable::new(vec![def("a", StatKind::Derived), def("A", StatKind::Derived)], &[]);
    assert_eq!(dup.unwrap_err(), TableError::Duplicate("a".into()));
    let bad = StatTable::new(vec![def("hp", StatKind::Derived)], &[(Role::Health, "hp".into())]);
    assert!(matches!(bad.unwrap_err(), TableError::RoleKind { role: Role::Health, .. }));
    let unknown = StatTable::new(vec![], &[(Role::Level, "lvl".into())]);
    assert!(matches!(unknown.unwrap_err(), TableError::UnknownRoleStat { .. }));
}

#[test]
fn new_character_settles_full_pools_and_derived() {
    let (t, _, s) = setup();
    let v = s.view(&t);
    assert_eq!((v.current(HEALTH), v.max(HEALTH)), (100.0, 100.0));
    assert_eq!((v.current(MANA), v.max(MANA)), (100.0, 100.0));
    assert_eq!((v.current(STAMINA), v.max(STAMINA)), (80.0, 80.0));
    assert_eq!(v.modified(CAPACITY), 250.0);
    assert_eq!(v.modified(LEVEL), 1.0);
    assert_eq!(v.ratio(MANA), 1.0);
}

#[test]
fn modified_formula_layers_and_mul_sum() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[
        modifier(STR, Layer::Permanent, ModOp::Add, 10.0, 1),
        modifier(STR, Layer::Temporary, ModOp::Add, 5.0, 2),
        modifier(STR, Layer::Temporary, ModOp::Mul, 0.5, 3),
        modifier(STR, Layer::Temporary, ModOp::Mul, 0.5, 4),
        StatOp::Damage { stat: STR, amount: 20.0, cap_at_modified: false },
    ]);
    let v = s.view(&t);
    // ((50 + 10) - 20 + 5) × (1 + 0.5 + 0.5): the Mul modifiers sum, they don't compound.
    assert_eq!(v.modified(STR), 90.0);
    assert_eq!((v.base(STR), v.effective_base(STR), v.damage(STR)), (50.0, 60.0, 20.0));
    assert_eq!(v.modifiers_on(STR).count(), 4);
    // Cap and min clamp the modified value.
    ops(&t, &r, &mut s, &[modifier(BLADE, Layer::Temporary, ModOp::Add, 200.0, 5)]);
    assert_eq!(s.view(&t).modified(BLADE), 100.0);
    ops(&t, &r, &mut s, &[StatOp::Damage { stat: BLADE, amount: 500.0, cap_at_modified: false }]);
    assert_eq!(s.view(&t).modified(BLADE), 0.0);
}

#[test]
fn restore_stops_at_zero_damage() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[StatOp::Damage { stat: STR, amount: 10.0, cap_at_modified: false }]);
    assert_eq!(s.view(&t).modified(STR), 40.0);
    ops(&t, &r, &mut s, &[StatOp::Restore { stat: STR, amount: 25.0 }]);
    assert_eq!((s.view(&t).damage(STR), s.view(&t).modified(STR)), (0.0, 50.0));
}

#[test]
fn damage_capped_at_modified_when_asked() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[
        modifier(STR, Layer::Temporary, ModOp::Add, 10.0, 1),
        StatOp::Damage { stat: STR, amount: 100.0, cap_at_modified: true },
    ]);
    assert_eq!((s.view(&t).damage(STR), s.view(&t).modified(STR)), (60.0, 0.0));
    ops(&t, &r, &mut s, &[StatOp::Damage { stat: BLADE, amount: 100.0, cap_at_modified: false }]);
    assert_eq!((s.view(&t).damage(BLADE), s.view(&t).modified(BLADE)), (100.0, 0.0));
}

/// `OpenMW` `damageAttribute`: `magnitude = min(modified, magnitude)`, so damage past the value
/// (an uncapped drain of 60 on 50) is kept, not lowered; a later restore of 5 leaves it at 0.
#[test]
fn capped_damage_keeps_damage_already_past_the_value() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[
        StatOp::Damage { stat: STR, amount: 60.0, cap_at_modified: false },
        StatOp::Damage { stat: STR, amount: 5.0, cap_at_modified: true },
    ]);
    assert_eq!((s.view(&t).damage(STR), s.view(&t).modified(STR)), (60.0, 0.0));
    ops(&t, &r, &mut s, &[StatOp::Restore { stat: STR, amount: 5.0 }]);
    assert_eq!((s.view(&t).damage(STR), s.view(&t).modified(STR)), (55.0, 0.0));
    ops(&t, &r, &mut s, &[StatOp::Damage { stat: STR, amount: 30.0, cap_at_modified: false }]);
    ops(&t, &r, &mut s, &[StatOp::Restore { stat: STR, amount: 80.0 }]);
    ops(&t, &r, &mut s, &[StatOp::Damage { stat: STR, amount: 20.0, cap_at_modified: true }]);
    assert_eq!((s.view(&t).damage(STR), s.view(&t).modified(STR)), (25.0, 25.0));
}

#[test]
fn add_then_remove_source_is_byte_identical() {
    let (t, r, mut s) = setup();
    let before = bytes(&s);
    // str feeds mana's max (KeepRatio); health is KeepCurrent.
    ops(&t, &r, &mut s, &[
        modifier(STR, Layer::Temporary, ModOp::Add, 10.0, 1),
        modifier(HEALTH, Layer::Temporary, ModOp::Add, 25.0, 1),
    ]);
    assert_eq!(s.view(&t).max(MANA), 120.0);
    assert_eq!(s.view(&t).current(MANA), 120.0);
    assert_eq!(s.view(&t).max(HEALTH), 125.0);
    ops(&t, &r, &mut s, &[StatOp::RemoveSource(src(1))]);
    assert_eq!(bytes(&s), before);
    assert!(!s.changed_since_init(), "modifiers re-register on load: not a saved change");
    // Removing a source that was never added changes nothing and says nothing.
    let ev = ops(&t, &r, &mut s, &[StatOp::RemoveSource(src(99))]);
    assert!(ev.is_empty());
    assert_eq!(bytes(&s), before);
}

#[test]
fn two_sources_on_one_stat_remove_independently() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[
        modifier(STR, Layer::Temporary, ModOp::Add, 10.0, 1),
        modifier(STR, Layer::Temporary, ModOp::Add, 4.0, 2),
    ]);
    assert_eq!(s.view(&t).modified(STR), 64.0);
    ops(&t, &r, &mut s, &[StatOp::RemoveSource(src(1))]);
    assert_eq!(s.view(&t).modified(STR), 54.0);
    ops(&t, &r, &mut s, &[StatOp::RemoveSource(src(2))]);
    assert_eq!(s.view(&t).modified(STR), 50.0);
}

/// `OpenMW` `DynamicStat::setCurrent`, its eight branches (max 100).
#[test]
fn set_current_matches_openmw_flags() {
    // Increase: within max; over max allowed; over max refused (clamped); already above max.
    assert_eq!(set_current(50.0, 100.0, 80.0, false, false), 80.0);
    assert_eq!(set_current(50.0, 100.0, 130.0, false, true), 130.0);
    assert_eq!(set_current(50.0, 100.0, 130.0, false, false), 100.0);
    assert_eq!(set_current(120.0, 100.0, 130.0, false, false), 120.0);
    // Decrease: above zero; below zero allowed; below zero refused (to 0); already below zero.
    assert_eq!(set_current(50.0, 100.0, 20.0, false, false), 20.0);
    assert_eq!(set_current(50.0, 100.0, -20.0, true, false), -20.0);
    assert_eq!(set_current(50.0, 100.0, -20.0, false, false), 0.0);
    assert_eq!(set_current(-5.0, 100.0, -20.0, false, false), -5.0);
}

#[test]
fn pool_adjust_uses_the_flags() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[StatOp::PoolAdjust { pool: HEALTH, delta: 30.0, below_zero: false, above_max: true }]);
    assert_eq!(s.view(&t).current(HEALTH), 130.0);
    ops(&t, &r, &mut s, &[StatOp::PoolAdjust { pool: HEALTH, delta: -200.0, below_zero: true, above_max: false }]);
    assert_eq!(s.view(&t).current(HEALTH), -70.0);
    ops(&t, &r, &mut s, &[StatOp::PoolModMax { pool: STAMINA, delta: 20.0 }]);
    assert_eq!((s.view(&t).current(STAMINA), s.view(&t).max(STAMINA)), (100.0, 100.0));
    assert!(s.changed_since_init());
}

#[test]
fn rebase_keeps_ratio_current_or_deficit() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[
        StatOp::PoolSetCurrent { pool: MANA, value: 50.0 },
        StatOp::PoolSetCurrent { pool: STAMINA, value: 60.0 },
        StatOp::PoolSetCurrent { pool: HEALTH, value: 90.0 },
    ]);
    ops(&t, &r, &mut s, &[
        StatOp::ModBase { stat: STR, delta: 25.0 },
        StatOp::ModBase { stat: STAMINA, delta: 20.0 },
        StatOp::ModBase { stat: HEALTH, delta: -20.0 },
    ]);
    let v = s.view(&t);
    assert_eq!((v.current(MANA), v.max(MANA)), (75.0, 150.0), "KeepRatio");
    assert_eq!((v.current(STAMINA), v.max(STAMINA)), (80.0, 100.0), "KeepDeficit");
    assert_eq!((v.current(HEALTH), v.max(HEALTH)), (80.0, 80.0), "KeepCurrent lowered to the new max");
}

#[test]
fn depleted_fires_once_per_crossing() {
    let (t, r, mut s) = setup();
    let adjust = |delta| StatOp::PoolAdjust { pool: HEALTH, delta, below_zero: false, above_max: false };
    let depleted = |ev: &[StatEvent]| ev.iter().filter(|e| matches!(e, StatEvent::Depleted { pool: HEALTH, .. })).count();
    assert_eq!(depleted(&ops(&t, &r, &mut s, &[adjust(-99.5)])), 1);
    assert_eq!(depleted(&ops(&t, &r, &mut s, &[adjust(-0.25)])), 0, "still below");
    assert_eq!(depleted(&ops(&t, &r, &mut s, &[adjust(50.0)])), 0);
    assert_eq!(depleted(&ops(&t, &r, &mut s, &[adjust(-60.0)])), 1, "a second crossing");
    assert_eq!(s.view(&t).current(HEALTH), 0.0);
}

#[test]
fn changed_events_coalesce_per_stat() {
    let (t, r, mut s) = setup();
    let ev = ops(&t, &r, &mut s, &[
        StatOp::ModBase { stat: STR, delta: 1.0 },
        StatOp::ModBase { stat: STR, delta: 1.0 },
        StatOp::CounterAdd { stat: BOUNTY, delta: 40.0 },
    ]);
    let c = cause(CauseKind::Debug);
    assert_eq!(ev, vec![
        StatEvent::Changed { owner: OWNER, stat: STR, old: 50.0, new: 52.0, cause: c },
        // mana is KeepRatio: full stays full as its max follows str.
        StatEvent::Changed { owner: OWNER, stat: MANA, old: 100.0, new: 104.0, cause: c },
        StatEvent::Changed { owner: OWNER, stat: BOUNTY, old: 0.0, new: 40.0, cause: c },
    ]);
}

#[test]
fn wrong_kind_ops_are_rejected_and_change_nothing() {
    let (t, r, mut s) = setup();
    let before = bytes(&s);
    let mut out = Vec::new();
    let bad = [
        StatOp::Damage { stat: HEALTH, amount: 5.0, cap_at_modified: false },
        StatOp::PoolAdjust { pool: STR, delta: 1.0, below_zero: false, above_max: false },
        StatOp::CounterAdd { stat: STR, delta: 1.0 },
        StatOp::SetBase { stat: CAPACITY, value: 1.0 },
        StatOp::AddModifier(Modifier { stat: LEVEL, layer: Layer::Temporary, op: ModOp::Add, value: 1.0, source: src(1) }),
        StatOp::SetBase { stat: StatIdx(999), value: 1.0 },
    ];
    assert_eq!(run(&t, &r, &mut s, &bad, &mut out), bad.len());
    assert!(out.is_empty());
    assert_eq!(bytes(&s), before);
}

#[test]
fn counters_clamp_to_their_range() {
    let (t, r, mut s) = setup();
    ops(&t, &r, &mut s, &[StatOp::CounterSet { stat: LEVEL, value: 0.0 }]);
    assert_eq!(s.view(&t).modified(LEVEL), 1.0);
    ops(&t, &r, &mut s, &[StatOp::CounterAdd { stat: LEVEL, delta: 500.0 }]);
    assert_eq!(s.view(&t).modified(LEVEL), 100.0);
}

#[test]
fn use_skill_reports_use_then_raise_at_threshold() {
    let (t, r, mut s) = setup();
    let use_blade = [StatOp::UseSkill { skill: BLADE, use_kind: 2, scale: 1.0 }];
    let c = cause(CauseKind::Debug);
    for _ in 0..2 {
        let ev = ops(&t, &r, &mut s, &use_blade);
        assert_eq!(ev[0], StatEvent::SkillUsed { owner: OWNER, skill: BLADE, use_kind: 2, gain: 1.0, cause: c });
        assert!(!ev.iter().any(|e| matches!(e, StatEvent::SkillRaised { .. })));
    }
    assert_eq!(s.view(&t).modified(PROGRESS), 2.0);
    let ev = ops(&t, &r, &mut s, &use_blade);
    assert_eq!(ev[0], StatEvent::SkillUsed { owner: OWNER, skill: BLADE, use_kind: 2, gain: 1.0, cause: c });
    assert_eq!(ev[1], StatEvent::SkillRaised {
        owner: OWNER,
        skill: BLADE,
        base: 21.0,
        source: IncreaseSource::Usage,
        cause: c
    });
    let v = s.view(&t);
    assert_eq!((v.base(BLADE), v.modified(PROGRESS), v.modified(TALLY)), (21.0, 0.0, 1.0));
    // At the cap the model refuses: no raise, no event, not rejected.
    ops(&t, &r, &mut s, &[StatOp::SetBase { stat: BLADE, value: 100.0 }]);
    let ev = ops(&t, &r, &mut s, &[StatOp::RaiseSkill { skill: BLADE, source: IncreaseSource::Training }]);
    assert!(ev.is_empty());
    let ev = ops(&t, &r, &mut s, &[StatOp::RaiseSkill { skill: BLADE, source: IncreaseSource::Penalty }]);
    assert!(ev.iter().any(|e| matches!(e, StatEvent::SkillRaised { base: 99.0, .. })));
}

#[test]
fn level_up_wrong_picks_refused_then_levels() {
    let (t, r, mut s) = setup();
    let train = StatOp::RaiseSkill { skill: BLADE, source: IncreaseSource::Training };
    let ev = ops(&t, &r, &mut s, std::slice::from_ref(&train));
    assert!(!ev.iter().any(|e| matches!(e, StatEvent::LevelReady { .. })));
    let ev = ops(&t, &r, &mut s, &[train]);
    assert_eq!(ev.iter().filter(|e| matches!(e, StatEvent::LevelReady { .. })).count(), 1);
    assert_eq!(r.leveling.offer(&s.view(&t)).picks, 2);

    let before = bytes(&s);
    let mut out = Vec::new();
    assert_eq!(run(&t, &r, &mut s, &[StatOp::LevelUp { picks: smallvec![STR] }], &mut out), 1, "wrong pick count");
    assert_eq!(run(&t, &r, &mut s, &[StatOp::LevelUp { picks: smallvec![STR, MANA] }], &mut out), 1, "bad pick");
    assert!(out.is_empty());
    assert_eq!(bytes(&s), before);

    let ev = ops(&t, &r, &mut s, &[StatOp::LevelUp { picks: smallvec![STR, BLADE] }]);
    assert!(ev.contains(&StatEvent::LevelChanged { owner: OWNER, level: 2.0, cause: cause(CauseKind::Debug) }));
    let v = s.view(&t);
    assert_eq!((v.base(STR), v.base(BLADE), v.modified(TALLY)), (55.0, 27.0, 0.0));
    assert!(!r.leveling.ready(&v));
}

#[test]
fn groups_join_rank_expel_clear_reputation() {
    let (t, r, mut s) = setup();
    let g = RecordId::new("Guild");
    let c = cause(CauseKind::Debug);
    let ev = ops(&t, &r, &mut s, &[StatOp::Join { group: g.clone() }, StatOp::Join { group: g.clone() }]);
    assert_eq!(ev, vec![StatEvent::RankChanged { owner: OWNER, group: g.clone(), old: None, new: Some(0), cause: c }]);
    let ev = ops(&t, &r, &mut s, &[StatOp::SetRank { group: g.clone(), rank: 3 }]);
    assert_eq!(ev, vec![StatEvent::RankChanged { owner: OWNER, group: g.clone(), old: Some(0), new: Some(3), cause: c }]);
    let ev = ops(&t, &r, &mut s, &[StatOp::Expel { group: g.clone() }, StatOp::Expel { group: g.clone() }]);
    assert_eq!(ev, vec![StatEvent::Expelled { owner: OWNER, group: g.clone(), cause: c }]);
    ops(&t, &r, &mut s, &[
        StatOp::ClearExpelled { group: g.clone() },
        StatOp::GroupRep { group: g.clone(), delta: 5, set: false },
        StatOp::GroupRep { group: g.clone(), delta: 2, set: false },
    ]);
    assert_eq!(s.view(&t).membership(&g), Some(&Membership { rank: Some(3), reputation: 7, expelled: false }));
    ops(&t, &r, &mut s, &[StatOp::GroupRep { group: g.clone(), delta: -1, set: true }]);
    assert_eq!(s.view(&t).membership(&g).unwrap().reputation, -1);
    // Reputation and the expelled flag exist without membership.
    let other = RecordId::new("other");
    ops(&t, &r, &mut s, &[StatOp::GroupRep { group: other.clone(), delta: 4, set: false }]);
    assert_eq!(s.view(&t).membership(&other).unwrap().rank, None);
}

#[test]
fn leave_drops_rank_and_expulsion_keeps_reputation() {
    let (t, r, mut s) = setup();
    let g = RecordId::new("Guild");
    let c = cause(CauseKind::Debug);
    ops(&t, &r, &mut s, &[
        StatOp::SetRank { group: g.clone(), rank: 2 },
        StatOp::Expel { group: g.clone() },
        StatOp::GroupRep { group: g.clone(), delta: 6, set: true },
    ]);
    let ev = ops(&t, &r, &mut s, &[StatOp::Leave { group: g.clone() }, StatOp::Leave { group: g.clone() }]);
    assert_eq!(ev, vec![StatEvent::RankChanged { owner: OWNER, group: g.clone(), old: Some(2), new: None, cause: c }]);
    assert_eq!(s.view(&t).membership(&g), Some(&Membership { rank: None, reputation: 6, expelled: false }));
    // Never a member: nothing to leave, no entry made.
    let other = RecordId::new("other");
    assert!(ops(&t, &r, &mut s, &[StatOp::Leave { group: other.clone() }]).is_empty());
    assert_eq!(s.view(&t).membership(&other), None);
}

#[test]
fn regen_tick_and_rest() {
    let (t, r, mut s) = setup();
    let inputs = Inputs::default();
    let ctx = TickCtx { owner: OWNER, tick: 9, dt: 0.5, inputs: &inputs };
    let mut ev = Vec::new();
    tick(&t, &r, &mut s, &ctx, &mut ev);
    assert!(ev.is_empty(), "full pools, clean sheet: nothing to do");
    ops(&t, &r, &mut s, &[
        StatOp::PoolAdjust { pool: HEALTH, delta: -10.0, below_zero: false, above_max: false },
        StatOp::PoolAdjust { pool: STAMINA, delta: -40.0, below_zero: false, above_max: false },
    ]);
    tick(&t, &r, &mut s, &ctx, &mut ev);
    let regen = Cause { kind: CauseKind::Regen, source: None, tick: 9 };
    assert_eq!(ev, vec![
        StatEvent::Changed { owner: OWNER, stat: HEALTH, old: 90.0, new: 90.5, cause: regen },
        StatEvent::Changed { owner: OWNER, stat: STAMINA, old: 40.0, new: 45.0, cause: regen },
    ]);
    // Combat owns a combatant's stamina regen.
    let fighting = Inputs { combatant: true, ..Inputs::default() };
    tick(&t, &r, &mut s, &TickCtx { inputs: &fighting, ..ctx }, &mut ev);
    assert_eq!((s.view(&t).current(HEALTH), s.view(&t).current(STAMINA)), (91.0, 45.0));
    // Regen stops at the max.
    ops(&t, &r, &mut s, &[StatOp::Rest { hours: 2.0, sleep: true }]);
    assert_eq!(s.view(&t).current(HEALTH), 100.0);
}

#[test]
fn derived_follows_inputs() {
    let t = table();
    let mut m = model();
    m.derived_from = vec![(CAPACITY, CAPACITY, 2.0)];
    let r = m.rules();
    let mut s = Sheet::new(&t);
    s.settle(&t, &r, &Inputs::default(), Settle::NewCharacter);
    let heavy = Inputs { carried: 30.0, seed: 5, ..Inputs::default() };
    tick(&t, &r, &mut s, &TickCtx { owner: OWNER, tick: 1, dt: 0.1, inputs: &heavy }, &mut Vec::new());
    assert_eq!(s.view(&t).modified(CAPACITY), 60.0);
}

#[test]
fn one_op_stream_twice_gives_identical_bytes() {
    let stream = [
        modifier(STR, Layer::Temporary, ModOp::Mul, 0.1, 1),
        StatOp::Damage { stat: STR, amount: 3.3, cap_at_modified: true },
        StatOp::PoolAdjust { pool: MANA, delta: -17.7, below_zero: false, above_max: false },
        StatOp::UseSkill { skill: BLADE, use_kind: 0, scale: 1.7 },
        StatOp::UseSkill { skill: BLADE, use_kind: 0, scale: 1.7 },
        StatOp::Join { group: RecordId::new("g") },
        StatOp::RemoveSource(src(1)),
    ];
    let play = || {
        let (t, r, mut s) = setup();
        let mut ev = ops(&t, &r, &mut s, &stream);
        let inputs = Inputs::default();
        for i in 0..10 {
            tick(&t, &r, &mut s, &TickCtx { owner: OWNER, tick: i, dt: 1.0 / 60.0, inputs: &inputs }, &mut ev);
        }
        (bytes(&s), postcard::to_allocvec(&s.to_save(&t)).unwrap(), postcard::to_allocvec(&ev).unwrap())
    };
    assert_eq!(play(), play());
}

#[test]
fn save_round_trip_keeps_a_fortified_current() {
    let (t, r, mut s) = setup();
    let fortify = modifier(HEALTH, Layer::Temporary, ModOp::Add, 50.0, 1);
    ops(&t, &r, &mut s, &[
        fortify.clone(),
        StatOp::PoolAdjust { pool: HEALTH, delta: 50.0, below_zero: false, above_max: false },
        StatOp::Damage { stat: STR, amount: 4.0, cap_at_modified: false },
        StatOp::SetRank { group: RecordId::new("g"), rank: 2 },
        StatOp::CounterSet { stat: BOUNTY, value: 40.0 },
    ]);
    assert_eq!(s.view(&t).current(HEALTH), 150.0);
    let save = s.to_save(&t);
    let save: SheetSave = postcard::from_bytes(&postcard::to_allocvec(&save).unwrap()).unwrap();
    let (mut loaded, report) = Sheet::from_save(&t, &save);
    assert_eq!(report, LoadReport::default());
    // The source re-registers, then the sheet settles: the current above the unfortified max survives.
    ops(&t, &r, &mut loaded, &[fortify]);
    loaded.settle(&t, &r, &Inputs::default(), Settle::Loaded);
    assert_eq!(bytes(&loaded), bytes(&s));
    assert_eq!(loaded.view(&t).current(HEALTH), 150.0);
}

#[test]
fn load_reports_dropped_and_missing_keys() {
    let t = table();
    let save = SheetSave {
        values: vec![
            ValueSave { key: "Str".into(), base: 40.0, damage: 0.0 },
            ValueSave { key: "gone".into(), base: 1.0, damage: 0.0 },
            ValueSave { key: "level".into(), base: 1.0, damage: 0.0 },
        ],
        ..SheetSave::default()
    };
    let (s, report) = Sheet::from_save(&t, &save);
    assert_eq!(s.view(&t).base(STR), 40.0);
    assert_eq!(report.dropped, vec![StatKey::new("gone"), StatKey::new("level")]);
    let missing: Vec<_> = report.missing.iter().map(StatKey::as_str).collect();
    assert_eq!(missing, ["blade", "health", "mana", "stamina", "level", "blade_progress", "tally", "bounty"]);
}

/// The saved shape of [`SheetSave`] (wired into `sf.character` v3 in S8). Bless a new golden with
/// `SF_SAVE_BLESS=1`; a changed shape needs a save-section version bump, never a re-bless.
#[test]
fn sheet_save_shape_matches_its_golden() {
    let mut tracer = Tracer::new(TracerConfig::default());
    let samples = Samples::new();
    tracer.trace_type::<SheetSave>(&samples).unwrap();
    let text = serde_json::to_string_pretty(&tracer.registry().unwrap()).unwrap() + "\n";
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/shapes/sf_stats.SheetSave.v1.json");
    match std::fs::read_to_string(&path) {
        Ok(golden) if golden == text => {}
        _ if std::env::var_os("SF_SAVE_BLESS").is_some() => {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, text).unwrap();
        }
        _ => panic!("the saved shape of SheetSave changed ({}); new shape:\n{text}", path.display()),
    }
}

#[test]
#[ignore = "measurement: 500 sheets of a 77-stat table, 10% dirty, one tick each"]
fn measure_tick_cost() {
    let (t, r, base) = setup_with(table_with(67));
    let mut sheets: Vec<Sheet> = (0..500).map(|_| base.clone()).collect();
    let inputs = Inputs::default();
    let mut ev = Vec::new();
    let rounds = 200u32;
    let start = std::time::Instant::now();
    for round in 0..rounds {
        for (i, s) in sheets.iter_mut().enumerate() {
            if (i + round as usize).is_multiple_of(10) {
                let op = StatOp::PoolAdjust { pool: HEALTH, delta: -1.0, below_zero: false, above_max: false };
                run(&t, &r, s, &[op], &mut ev);
            }
            tick(&t, &r, s, &TickCtx { owner: OWNER, tick: round, dt: 1.0 / 60.0, inputs: &inputs }, &mut ev);
        }
        ev.clear();
    }
    let per = start.elapsed() / rounds;
    println!("500 sheets x {} stats, 10% dirty, all regenerating: {:.4} ms per tick (apply + tick)", t.len(), per.as_secs_f64() * 1000.0);
}

fn shift(stat: StatIdx, layer: Layer, value: f32, id: u64) -> StatOp {
    StatOp::AddModifierShift(Modifier { stat, layer, op: ModOp::Add, value, source: src(id) })
}

/// A pool fortify on a `KeepRatio` pool (effects E4 bug): max and current move by the same
/// amount, never by the ratio (`OpenMW` `modDynamicStat`), and the inverse takes the same amount
/// back, clamped at 0 (`setCurrent` default flags).
#[test]
fn a_shifting_modifier_moves_max_and_current_together_on_a_ratio_pool() {
    let (t, r, mut s) = setup();
    let pool = |s: &Sheet| (s.view(&t).current(MANA), s.view(&t).max(MANA));
    assert_eq!(pool(&s), (100.0, 100.0));
    ops(&t, &r, &mut s, &[shift(MANA, Layer::Temporary, 40.0, 1)]);
    assert_eq!(pool(&s), (140.0, 140.0));
    assert!(s.changed_since_init(), "a shifted current is saved state");
    ops(&t, &r, &mut s, &[StatOp::RemoveSourceShift(src(1))]);
    assert_eq!(pool(&s), (100.0, 100.0));

    // Partly spent: 50 / 100 + 40 is 90 / 140 (a plain modifier rebases to 70 / 140).
    ops(&t, &r, &mut s, &[StatOp::PoolSetCurrent { pool: MANA, value: 50.0 }, shift(MANA, Layer::Temporary, 40.0, 1)]);
    assert_eq!(pool(&s), (90.0, 140.0));
    ops(&t, &r, &mut s, &[StatOp::RemoveSourceShift(src(1))]);
    assert_eq!(pool(&s), (50.0, 100.0));

    // Spent below the bonus: the removal stops at 0.
    ops(&t, &r, &mut s, &[shift(MANA, Layer::Temporary, 40.0, 1), StatOp::PoolSetCurrent { pool: MANA, value: 10.0 }]);
    ops(&t, &r, &mut s, &[StatOp::RemoveSourceShift(src(1))]);
    assert_eq!(pool(&s), (0.0, 100.0));
    assert!(s.modifiers().is_empty());
    // Removing an unknown source shifts nothing.
    assert!(ops(&t, &r, &mut s, &[StatOp::RemoveSourceShift(src(9))]).is_empty());
}

/// An innate pool fortify on a derived pool (effects adapter A3 bug): a `Permanent` shifting
/// modifier survives the derivation (the derived base no longer overwrites it) and rebases with
/// the pool when the derivation moves.
#[test]
fn a_permanent_shift_on_a_derived_pool_survives_a_recompute() {
    let (t, r, mut s) = setup();
    let pool = |s: &Sheet| (s.view(&t).current(MANA), s.view(&t).max(MANA), s.view(&t).effective_base(MANA));
    ops(&t, &r, &mut s, &[shift(MANA, Layer::Permanent, 20.0, 1)]);
    assert_eq!(pool(&s), (120.0, 120.0, 120.0));
    assert_eq!(s.view(&t).base(MANA), 100.0, "the derived base is the formula's");
    // Str 50 -> 60: base 120 + 20, current by the pool's ratio.
    ops(&t, &r, &mut s, &[StatOp::PoolSetCurrent { pool: MANA, value: 60.0 }, StatOp::ModBase { stat: STR, delta: 10.0 }]);
    assert_eq!(pool(&s), (70.0, 140.0, 140.0));
    ops(&t, &r, &mut s, &[StatOp::RemoveSourceShift(src(1))]);
    assert_eq!(pool(&s), (50.0, 120.0, 120.0));
}

/// While building (spawn, load) a shifting modifier is a plain one: settle fills a new
/// character, a load keeps its saved current.
#[test]
fn a_shifting_modifier_does_not_shift_while_building() {
    let t = table();
    let r = model().rules();
    let mut s = Sheet::new(&t);
    ops(&t, &r, &mut s, &[StatOp::SetBase { stat: STR, value: 50.0 }, shift(MANA, Layer::Permanent, 20.0, 1)]);
    s.settle(&t, &r, &Inputs::default(), Settle::NewCharacter);
    assert_eq!((s.view(&t).current(MANA), s.view(&t).max(MANA)), (120.0, 120.0));
    assert!(!s.changed_since_init());
}
