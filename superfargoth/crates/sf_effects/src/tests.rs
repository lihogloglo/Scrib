//! effects.md "Tests" for E1: lifecycle, stacking, absorb, resist, determinism. Exact assertions
//! on a small stat table, synthetic defs and [`TestRules`]; the host is a loop over
//! `sf_stats::apply` per emitted op.

// Exact float assertions are the point (CONTRIBUTING "Tests": logic and data are exact).
#![allow(clippy::float_cmp)]

use std::collections::BTreeMap;

use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Inputs, Layer, Rebase, Role, Rules, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp, StatOwner,
    StatTable, TestModel,
};
use smallvec::smallvec;

use crate::*;

const STR: StatIdx = StatIdx(0);
const BLADE: StatIdx = StatIdx(1);
const HEALTH: StatIdx = StatIdx(2);
const MANA: StatIdx = StatIdx(3);

const TARGET: StatOwner = StatOwner::Actor(ActorKey(1));
const CASTER: StatOwner = StatOwner::Actor(ActorKey(2));
/// Four ticks a second: rates per tick are exact in f32.
const DT: f32 = 0.25;

fn stat(key: &str, group: &str, kind: StatKind) -> StatDef {
    StatDef { key: key.into(), kind, group: group.into(), public: false, label: format!("stat.{key}") }
}

fn stats() -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let pool = StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepCurrent };
    let defs = vec![
        stat("str", "attribute", value),
        stat("blade", "skill", value),
        stat("health", "pool", pool),
        stat("mana", "pool", pool),
    ];
    StatTable::new(defs, &[(Role::Health, "health".into()), (Role::Mana, "mana".into())]).unwrap()
}

fn fixed(key: &str) -> StatTarget {
    StatTarget::Fixed(key.into())
}

fn defs() -> EffectTable {
    let mut attr = EffectDef::new("fortify_attr", Archetype::Modify { stat: StatTarget::Arg, layer: LayerRule::BySource });
    attr.arg = ArgKind::Stat("attribute".into());
    let mut refresh = EffectDef::new("refresh_str", Archetype::Modify { stat: fixed("str"), layer: LayerRule::BySource });
    refresh.stack = Some(StackPolicy::Refresh);
    let mut stack2 = EffectDef::new("stack_str", Archetype::Modify { stat: fixed("str"), layer: LayerRule::BySource });
    stack2.stack = Some(StackPolicy::Stack { limit: 2 });
    let mut ignore = EffectDef::new("ignore_str", Archetype::Modify { stat: fixed("str"), layer: LayerRule::BySource });
    ignore.stack = Some(StackPolicy::Ignore);
    let mut once = EffectDef::new("restore_str_once", Archetype::Restore { stat: fixed("str") });
    once.flags.applied_once = true;
    let mut lonely = EffectDef::new("lonely", Archetype::MagnitudeStat { stat: fixed("blade") });
    lonely.flags.non_recastable = true;
    let mut linked = EffectDef::new("linked", Archetype::MagnitudeStat { stat: fixed("blade") });
    linked.flags.caster_linked = true;
    let hurt = Archetype::Pool { pool: fixed("health"), mode: PoolMode::Hurt };
    EffectTable::new(vec![
        EffectDef::new("fortify_str", Archetype::Modify { stat: fixed("str"), layer: LayerRule::BySource }),
        attr,
        refresh,
        stack2,
        ignore,
        EffectDef::new("drain_str", Archetype::Drain { stat: fixed("str") }),
        EffectDef::new("restore_str", Archetype::Restore { stat: fixed("str") }),
        once,
        EffectDef::new("damage_health", hurt.clone()),
        EffectDef::new("drain_health", Archetype::Pool { pool: fixed("health"), mode: PoolMode::Drain }),
        EffectDef::new("fortify_health", Archetype::Pool { pool: fixed("health"), mode: PoolMode::Fortify }),
        EffectDef::new("fortify_mana", Archetype::Pool { pool: fixed("mana"), mode: PoolMode::Fortify }),
        EffectDef::new("boost_health", Archetype::Pool { pool: fixed("health"), mode: PoolMode::Boost }),
        EffectDef::new("absorb_health", Archetype::Absorb { inner: Box::new(hurt) }),
        EffectDef::new("absorb_str", Archetype::Absorb { inner: Box::new(Archetype::Drain { stat: fixed("str") }) }),
        lonely,
        linked,
        EffectDef::new("resist_fire", Archetype::MagnitudeStat { stat: fixed("blade") }),
        EffectDef::new("bad_stat", Archetype::Modify { stat: fixed("nope"), layer: LayerRule::BySource }),
    ])
    .unwrap()
}

/// One target and one caster, each str 50, blade 0, health 100, mana 100; pools full.
struct World {
    stats: StatTable,
    rules: Rules,
    defs: EffectTable,
    land: TestRules,
    set: ActiveSet,
    ids: EffectIdAlloc,
    sheets: BTreeMap<StatOwner, Sheet>,
    tick: u32,
    events: Vec<EffectEvent>,
    out: EffectOut,
}

impl World {
    fn new() -> Self {
        Self::with(stats())
    }

    fn with(stats: StatTable) -> Self {
        let rules = TestModel::default().rules();
        let mut sheets = BTreeMap::new();
        for owner in [TARGET, CASTER] {
            let mut s = Sheet::new(&stats);
            let init = [
                StatOp::SetBase { stat: STR, value: 50.0 },
                StatOp::SetBase { stat: HEALTH, value: 100.0 },
                StatOp::SetBase { stat: MANA, value: 100.0 },
            ];
            let inputs = Inputs::default();
            let cause = sf_stats::Cause { kind: sf_stats::CauseKind::Debug, source: None, tick: 0 };
            let ctx = ApplyCtx { owner, cause, inputs: &inputs };
            assert_eq!(sf_stats::apply(&stats, &rules, &mut s, &init, &ctx, &mut Vec::new()), 0);
            s.settle(&stats, &rules, &inputs, Settle::NewCharacter);
            sheets.insert(owner, s);
        }
        Self {
            stats,
            rules,
            defs: defs(),
            land: TestRules::default(),
            set: ActiveSet::new(),
            ids: EffectIdAlloc::default(),
            sheets,
            tick: 0,
            events: Vec::new(),
            out: EffectOut::default(),
        }
    }

    /// Runs the stat ops of the last step on their owners' sheets, one apply per op.
    fn host(&mut self, out: EffectOut) {
        let inputs = Inputs::default();
        for (owner, op, cause) in &out.stat_ops {
            let sheet = self.sheets.get_mut(owner).unwrap();
            let ctx = ApplyCtx { owner: *owner, cause: *cause, inputs: &inputs };
            assert_eq!(sf_stats::apply(&self.stats, &self.rules, sheet, std::slice::from_ref(op), &ctx, &mut Vec::new()), 0);
        }
        self.events.extend(out.events.iter().cloned());
        self.out = out;
    }

    fn run(&mut self, ops: &[EffectOp]) {
        let mut out = EffectOut::default();
        let mut ctx = EffectCtx { target: TARGET, tick: self.tick, dt: DT, ids: &mut self.ids };
        apply(&self.defs, &self.stats, &self.land, &mut self.set, ops, &mut ctx, &mut out);
        self.host(out);
    }

    /// One fixed tick (effects then stats, as the per-tick pipeline).
    fn step(&mut self) {
        let mut out = EffectOut::default();
        let ctx = EffectCtx { target: TARGET, tick: self.tick, dt: DT, ids: &mut self.ids };
        tick(&self.defs, &self.stats, &mut self.set, &ctx, &mut out);
        self.host(out);
        self.tick += 1;
    }

    fn steps(&mut self, n: u32) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Sets a pool's current directly (spending it), outside any effect.
    fn set_current(&mut self, owner: StatOwner, pool: StatIdx, value: f32) {
        let inputs = Inputs::default();
        let cause = sf_stats::Cause { kind: sf_stats::CauseKind::Debug, source: None, tick: self.tick };
        let ctx = ApplyCtx { owner, cause, inputs: &inputs };
        let op = StatOp::PoolSetCurrent { pool, value };
        let sheet = self.sheets.get_mut(&owner).unwrap();
        assert_eq!(sf_stats::apply(&self.stats, &self.rules, sheet, &[op], &ctx, &mut Vec::new()), 0);
    }

    fn pool(&self, owner: StatOwner, idx: StatIdx) -> (f32, f32) {
        (self.current(owner, idx), self.max(owner, idx))
    }

    fn modified(&self, owner: StatOwner, idx: StatIdx) -> f32 {
        self.sheets[&owner].view(&self.stats).modified(idx)
    }

    fn current(&self, owner: StatOwner, idx: StatIdx) -> f32 {
        self.sheets[&owner].view(&self.stats).current(idx)
    }

    fn max(&self, owner: StatOwner, idx: StatIdx) -> f32 {
        self.sheets[&owner].view(&self.stats).max(idx)
    }

    fn bytes(&self, owner: StatOwner) -> Vec<u8> {
        postcard::to_allocvec(&self.sheets[&owner]).unwrap()
    }

    fn ended(&self) -> Vec<EndReason> {
        self.events.iter().filter_map(|e| if let EffectEvent::Ended { reason, .. } = e { Some(*reason) } else { None }).collect()
    }
}

fn entry(effect: &str, magnitude: f32, seconds: f32) -> EffectEntry {
    EffectEntry {
        effect: effect.into(),
        arg: None,
        range: Range::OnSelf,
        area: 0.0,
        duration: seconds,
        magnitude: (magnitude, magnitude),
    }
}

fn land_from(record: &str, class: SourceClass, caster: Option<StatOwner>, entries: EffectList) -> EffectOp {
    EffectOp::Inflict {
        source: SourceRef { record: RecordId::new(record), item: None },
        class,
        caster,
        entries,
        flags: InstanceFlags::default(),
        rolls: RollSeed(42),
    }
}

fn cast(record: &str, entries: EffectList) -> EffectOp {
    land_from(record, SourceClass::Cast, Some(CASTER), entries)
}

#[test]
fn fortify_adds_by_source_and_expiry_restores_the_exact_sheet() {
    let mut w = World::new();
    let before = w.bytes(TARGET);
    w.run(&[cast("sp", smallvec![entry("fortify_str", 10.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 60.0);
    let id = w.set.sources()[0].id;
    assert_eq!(id, InstanceId(1));
    assert_eq!(w.out.stat_ops[0].2.source, Some(id.source_id(0)));
    assert!(matches!(w.events[0], EffectEvent::Applied { magnitude, .. } if magnitude == 10.0));
    w.steps(3);
    assert_eq!(w.modified(TARGET, STR), 60.0);
    assert!(w.ended().is_empty());
    w.step();
    assert_eq!(w.modified(TARGET, STR), 50.0);
    assert_eq!(w.ended(), [EndReason::Expired]);
    assert!(w.set.is_empty() && !w.set.is_timed());
    assert_eq!(w.bytes(TARGET), before, "removal by source restores the prior sheet");
}

#[test]
fn arg_effect_resolves_its_stat_and_ends_by_source() {
    let mut w = World::new();
    let mut e = entry("fortify_attr", 5.0, 10.0);
    e.arg = Some(ArgRef::Stat("STR".into()));
    w.run(&[cast("sp", smallvec![e.clone()])]);
    assert_eq!(w.modified(TARGET, STR), 55.0);
    assert_eq!(w.set.magnitude(&"fortify_attr".into(), e.arg.as_ref()), 5.0);
    w.run(&[EffectOp::EndBySource { record: RecordId::new("sp") }]);
    assert_eq!(w.modified(TARGET, STR), 50.0);
    assert_eq!(w.ended(), [EndReason::SourceRemoved]);
    // A skill is not of the def's argument group.
    e.arg = Some(ArgRef::Stat("blade".into()));
    w.run(&[cast("sp", smallvec![e])]);
    assert!(matches!(w.events.last(), Some(EffectEvent::Rejected { reason: Rejection::UnknownStat, .. })));
}

#[test]
fn innate_modify_is_permanent_and_lives_with_its_source() {
    let mut w = World::new();
    w.run(&[land_from("ability", SourceClass::Innate, None, smallvec![entry("fortify_str", 10.0, 0.0)])]);
    let view = w.sheets[&TARGET].view(&w.stats);
    assert_eq!((view.effective_base(STR), view.modified(STR)), (60.0, 60.0));
    assert!(view.modifiers_on(STR).all(|m| m.layer == Layer::Permanent));
    assert_eq!(w.set.sources()[0].effects[0].duration, None);
    assert!(!w.set.is_timed(), "nothing to tick");
    w.steps(100);
    assert_eq!(w.modified(TARGET, STR), 60.0);
    w.run(&[EffectOp::EndBySource { record: RecordId::new("ability") }]);
    assert_eq!(w.modified(TARGET, STR), 50.0);
}

#[test]
fn drain_restores_exactly_what_it_damaged_even_after_a_restore() {
    let mut w = World::new();
    w.run(&[cast("drain", smallvec![entry("drain_str", 10.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 40.0);
    w.steps(4);
    assert_eq!(w.modified(TARGET, STR), 50.0);
    assert_eq!(w.sheets[&TARGET].view(&w.stats).damage(STR), 0.0);

    w.run(&[cast("drain", smallvec![entry("drain_str", 10.0, 1.0)])]);
    w.run(&[cast("heal", smallvec![entry("restore_str_once", 10.0, 0.0)])]);
    assert_eq!(w.modified(TARGET, STR), 50.0);
    w.steps(4);
    assert_eq!(w.modified(TARGET, STR), 50.0, "the drain's restore stops at no damage");
    assert_eq!(w.sheets[&TARGET].view(&w.stats).damage(STR), 0.0);
}

#[test]
fn rate_damage_totals_magnitude_times_duration() {
    let mut w = World::new();
    w.run(&[cast("burn", smallvec![entry("damage_health", 8.0, 1.0)])]);
    assert_eq!(w.current(TARGET, HEALTH), 100.0, "rates deliver on ticks");
    w.steps(4);
    assert_eq!(w.current(TARGET, HEALTH), 92.0);
    assert_eq!(w.ended(), [EndReason::Expired]);
    assert_eq!(w.set.sources().len(), 0);
    w.steps(4);
    assert_eq!(w.current(TARGET, HEALTH), 92.0);
}

#[test]
fn instant_effects_land_and_end_the_same_step() {
    let mut w = World::new();
    w.run(&[cast("bolt", smallvec![entry("damage_health", 30.0, 0.0), entry("fortify_str", 10.0, 0.0)])]);
    assert_eq!(w.current(TARGET, HEALTH), 70.0);
    assert_eq!(w.modified(TARGET, STR), 50.0);
    assert!(w.set.is_empty());
    assert_eq!(w.ended(), [EndReason::Expired, EndReason::Expired]);
}

#[test]
fn skip_equals_ticking() {
    let ops = [cast("burn", smallvec![entry("damage_health", 8.0, 2.0), entry("fortify_str", 10.0, 1.5)])];
    let mut ticked = World::new();
    ticked.run(&ops);
    ticked.steps(6);
    let mut skipped = World::new();
    skipped.run(&ops);
    skipped.run(&[EffectOp::Skip { seconds: 1.5 }]);
    assert_eq!(skipped.current(TARGET, HEALTH), 88.0);
    assert_eq!(skipped.modified(TARGET, STR), 50.0);
    assert_eq!(skipped.set, ticked.set);
    assert_eq!(skipped.bytes(TARGET), ticked.bytes(TARGET));
    skipped.run(&[EffectOp::Skip { seconds: 60.0 }]);
    assert_eq!(skipped.current(TARGET, HEALTH), 84.0, "the remaining 2 ticks only");
    assert!(skipped.set.is_empty());
}

#[test]
fn replace_ends_the_old_effect() {
    let mut w = World::new();
    w.run(&[cast("sp", smallvec![entry("fortify_str", 10.0, 1.0)])]);
    w.run(&[cast("sp", smallvec![entry("fortify_str", 20.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 70.0);
    assert_eq!(w.ended(), [EndReason::Replaced]);
    assert_eq!(w.set.sources().len(), 1);
    // Another caster's landing is another stack.
    w.run(&[land_from("sp", SourceClass::Cast, None, smallvec![entry("fortify_str", 5.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 75.0);
}

#[test]
fn refresh_restarts_the_duration() {
    let mut w = World::new();
    w.run(&[cast("sp", smallvec![entry("refresh_str", 10.0, 1.0)])]);
    w.steps(3);
    w.run(&[cast("sp", smallvec![entry("refresh_str", 99.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 60.0, "kept, not replaced");
    w.steps(3);
    assert_eq!(w.modified(TARGET, STR), 60.0);
    w.step();
    assert_eq!(w.modified(TARGET, STR), 50.0);
}

#[test]
fn stack_limit_and_ignore() {
    let mut w = World::new();
    let stack = || cast("sp", smallvec![entry("stack_str", 10.0, 1.0)]);
    w.run(&[stack(), stack(), stack()]);
    assert_eq!(w.modified(TARGET, STR), 70.0, "limit 2");
    assert_eq!(w.set.sources().len(), 2);
    let ignore = || cast("ig", smallvec![entry("ignore_str", 5.0, 1.0)]);
    w.run(&[ignore(), ignore()]);
    assert_eq!(w.modified(TARGET, STR), 75.0);
    assert_eq!(w.set.sources().len(), 3);
}

#[test]
fn non_recastable_refuses_while_active() {
    let mut w = World::new();
    w.run(&[cast("calm", smallvec![entry("lonely", 1.0, 1.0)])]);
    let ids = w.ids;
    w.run(&[cast("calm", smallvec![entry("lonely", 1.0, 1.0)])]);
    assert!(w.out.stat_ops.is_empty());
    assert!(matches!(w.events.last(), Some(EffectEvent::CastRefused { .. })));
    assert_eq!(w.ids, ids, "no instance allocated");
    w.steps(4);
    w.run(&[cast("calm", smallvec![entry("lonely", 1.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, BLADE), 1.0);
}

#[test]
fn absorb_gives_two_owners_one_cause() {
    let mut w = World::new();
    let inputs = Inputs::default();
    let cause = sf_stats::Cause { kind: sf_stats::CauseKind::Debug, source: None, tick: 0 };
    let ctx = ApplyCtx { owner: CASTER, cause, inputs: &inputs };
    let hurt = [StatOp::PoolSetCurrent { pool: HEALTH, value: 50.0 }];
    sf_stats::apply(&w.stats, &w.rules, w.sheets.get_mut(&CASTER).unwrap(), &hurt, &ctx, &mut Vec::new());
    w.run(&[cast("leech", smallvec![entry("absorb_health", 8.0, 1.0)])]);
    w.step();
    assert_eq!((w.current(TARGET, HEALTH), w.current(CASTER, HEALTH)), (98.0, 52.0));
    let [(a, _, ca), (b, _, cb)] = &w.out.stat_ops[..] else { panic!("two ops: {:?}", w.out.stat_ops) };
    assert_eq!((*a, *b), (TARGET, CASTER));
    assert_eq!(ca, cb);

    w.run(&[cast("steal", smallvec![entry("absorb_str", 10.0, 1.0)])]);
    assert_eq!((w.modified(TARGET, STR), w.modified(CASTER, STR)), (40.0, 60.0));
    w.steps(4);
    assert_eq!((w.modified(TARGET, STR), w.modified(CASTER, STR)), (50.0, 50.0));
}

/// [`stats`] with mana rebasing by ratio (Morrowind's magicka and fatigue).
fn ratio_stats() -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let pool = |rebase| StatKind::Pool { depleted_below: 0.0, rebase };
    let defs = vec![
        stat("str", "attribute", value),
        stat("blade", "skill", value),
        stat("health", "pool", pool(Rebase::KeepCurrent)),
        stat("mana", "pool", pool(Rebase::KeepRatio)),
    ];
    StatTable::new(defs, &[(Role::Health, "health".into()), (Role::Mana, "mana".into())]).unwrap()
}

/// E4 bug: a temporary pool fortify on a `KeepRatio` pool counted twice (the modifier rebased
/// the current, then an adjust added the magnitude again: 100 / 100 read 180 / 140). Max and
/// current now move together, and the end takes the same amount back.
#[test]
fn a_pool_fortify_on_a_ratio_pool_counts_once() {
    let mut w = World::with(ratio_stats());
    w.run(&[cast("sp", smallvec![entry("fortify_mana", 40.0, 1.0)])]);
    assert_eq!(w.pool(TARGET, MANA), (140.0, 140.0));
    w.steps(4);
    assert_eq!(w.pool(TARGET, MANA), (100.0, 100.0));
    // Partly spent while active: 90 / 140 ends at 50 / 100 (the ratio would give 64.3).
    w.run(&[cast("sp", smallvec![entry("fortify_mana", 40.0, 1.0)])]);
    w.set_current(TARGET, MANA, 90.0);
    w.steps(4);
    assert_eq!(w.pool(TARGET, MANA), (50.0, 100.0));
    // Innate: on the Permanent layer, removed with its source.
    w.run(&[land_from("ability", SourceClass::Innate, None, smallvec![entry("fortify_mana", 20.0, 0.0)])]);
    assert_eq!(w.pool(TARGET, MANA), (70.0, 120.0));
    w.run(&[EffectOp::EndBySource { record: RecordId::new("ability") }]);
    assert_eq!(w.pool(TARGET, MANA), (50.0, 100.0));
}

/// `Boost` (Morrowind's Fortify Health / Magicka / Fatigue, `OpenMW` `adjustDynamicStat`): the
/// current over a max that stays, the same amount back at the end, below 0 allowed; an innate
/// source raises max and current together (`modDynamicStat`).
#[test]
fn a_boost_raises_the_current_over_the_max_and_takes_it_back() {
    let mut w = World::with(ratio_stats());
    w.run(&[cast("sp", smallvec![entry("boost_health", 40.0, 1.0)])]);
    assert_eq!(w.pool(TARGET, HEALTH), (140.0, 100.0));
    w.steps(4);
    assert_eq!(w.pool(TARGET, HEALTH), (100.0, 100.0));
    w.run(&[cast("sp", smallvec![entry("boost_health", 40.0, 1.0)])]);
    w.set_current(TARGET, HEALTH, 20.0);
    w.steps(4);
    assert_eq!(w.pool(TARGET, HEALTH), (-20.0, 100.0), "the end can kill");
    w.set_current(TARGET, HEALTH, 100.0);
    w.run(&[land_from("ability", SourceClass::Innate, None, smallvec![entry("boost_health", 20.0, 0.0)])]);
    assert_eq!(w.pool(TARGET, HEALTH), (120.0, 120.0));
    w.run(&[EffectOp::EndBySource { record: RecordId::new("ability") }]);
    assert_eq!(w.pool(TARGET, HEALTH), (100.0, 100.0));
}

#[test]
fn pool_fortify_and_drain_revert_exactly() {
    let mut w = World::new();
    w.run(&[cast("sp", smallvec![entry("fortify_health", 20.0, 1.0), entry("drain_health", 30.0, 1.0)])]);
    assert_eq!((w.max(TARGET, HEALTH), w.current(TARGET, HEALTH)), (120.0, 90.0));
    w.steps(4);
    assert_eq!((w.max(TARGET, HEALTH), w.current(TARGET, HEALTH)), (100.0, 100.0));
    w.run(&[land_from("ability", SourceClass::Innate, None, smallvec![entry("fortify_health", 20.0, 0.0)])]);
    assert_eq!((w.max(TARGET, HEALTH), w.current(TARGET, HEALTH)), (120.0, 120.0));
    w.run(&[EffectOp::EndBySource { record: RecordId::new("ability") }]);
    assert_eq!((w.max(TARGET, HEALTH), w.current(TARGET, HEALTH)), (100.0, 100.0));
}

#[test]
fn resist_scales_and_zero_drops() {
    let mut w = World::new();
    w.land.resist = vec![("fortify_str".into(), 0.5), ("resist_fire".into(), 0.0)];
    w.run(&[cast("sp", smallvec![entry("fortify_str", 10.0, 1.0)])]);
    assert_eq!(w.modified(TARGET, STR), 55.0);
    let ids = w.ids;
    w.run(&[cast("fire", smallvec![entry("resist_fire", 10.0, 1.0)])]);
    assert!(w.out.stat_ops.is_empty());
    assert!(matches!(w.events.last(), Some(EffectEvent::Resisted { .. })));
    assert_eq!(w.ids, ids);
    let mut op = cast("fire", smallvec![entry("resist_fire", 10.0, 1.0)]);
    if let EffectOp::Inflict { flags, .. } = &mut op {
        flags.ignore_resist = true;
    }
    w.run(&[op]);
    assert_eq!(w.modified(TARGET, BLADE), 10.0);
}

#[test]
fn unknown_keys_and_missing_casters_are_reported() {
    let mut w = World::new();
    w.run(&[land_from("sp", SourceClass::Cast, None, smallvec![
        entry("nothing", 1.0, 1.0),
        entry("bad_stat", 1.0, 1.0),
        entry("linked", 1.0, 1.0),
    ])]);
    let reasons: Vec<_> = w
        .events
        .iter()
        .filter_map(|e| if let EffectEvent::Rejected { reason, .. } = e { Some(*reason) } else { None })
        .collect();
    assert_eq!(reasons, [Rejection::UnknownEffect, Rejection::UnknownStat, Rejection::NoCaster]);
    assert!(w.set.is_empty());
    w.run(&[EffectOp::End { instance: InstanceId(99), reason: EndReason::Dispelled }]);
    assert!(w.out.stat_ops.is_empty() && w.out.events.is_empty(), "unknown instance: no-op");
    assert!(EffectTable::new(vec![
        EffectDef::new("x", Archetype::Restore { stat: fixed("str") }),
        EffectDef::new("X", Archetype::Restore { stat: fixed("str") }),
    ])
    .is_err());
}

#[test]
fn same_seed_same_bytes() {
    let script = |w: &mut World| {
        let mut e = entry("fortify_str", 0.0, 2.0);
        e.magnitude = (1.0, 20.0);
        w.run(&[
            cast("a", smallvec![e.clone(), entry("damage_health", 8.0, 1.0)]),
            cast("b", smallvec![entry("drain_str", 5.0, 0.5)]),
        ]);
        w.steps(3);
        w.run(&[cast("a", smallvec![e]), EffectOp::Skip { seconds: 1.0 }]);
        w.steps(2);
        (postcard::to_allocvec(&w.set).unwrap(), postcard::to_allocvec(&w.events).unwrap(), w.bytes(TARGET))
    };
    let (a, b) = (script(&mut World::new()), script(&mut World::new()));
    assert_eq!(a, b);
    let m = World::new().land.magnitude(&RollQuery {
        def: defs().get(&"fortify_str".into()).unwrap(),
        entry: &EffectEntry { magnitude: (1.0, 20.0), ..entry("fortify_str", 0.0, 1.0) },
        rolls: [roll(RollSeed(42), 0, 0), roll(RollSeed(42), 0, 1)],
    });
    assert!((1.0..20.0).contains(&m));
    assert_eq!(roll(RollSeed(42), 0, 0), roll(RollSeed(42), 0, 0));
    assert_ne!(roll(RollSeed(42), 0, 0), roll(RollSeed(43), 0, 0));
}
