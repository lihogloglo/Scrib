//! effects.md "Tests" for E2: the `sf.effects.*` rule formulas (exact, within f32 rounding),
//! [`StandardEffectRules`] reading real sheets through the queries, [`TestRules`] and the
//! [`EffectRules`] bundle, and every rule input and output surviving a postcard round trip.

use serde::Serialize;
use serde::de::DeserializeOwned;
use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, Inputs, Rebase, Role, Rules, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp,
    StatOwner, StatTable, TestModel,
};
use smallvec::smallvec;

use crate::*;

const SKILL: StatIdx = StatIdx(0);
const SILENCE: StatIdx = StatIdx(1);
const REFLECT: StatIdx = StatIdx(2);
const ABSORB: StatIdx = StatIdx(3);
const RESIST_FIRE: StatIdx = StatIdx(4);
const WEAK_FIRE: StatIdx = StatIdx(5);
const MANA: StatIdx = StatIdx(6);

const OWNER: StatOwner = StatOwner::Actor(ActorKey(1));
const CASTER: StatOwner = StatOwner::Actor(ActorKey(2));

/// Equal within f32 rounding of a few operations on numbers below 1000.
fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[track_caller]
fn assert_near(a: f32, b: f32) {
    assert!(near(a, b), "{a} != {b}");
}

/// `base 2, magnitude 5..15, 10 s, no area`: costs 2 · 10 · 10 · 0.1 = 20 at the default scale.
fn cost_entry() -> CostEntryIn {
    CostEntryIn { base_cost: 2.0, magnitude: (5.0, 15.0), duration: 10.0, area: 0.0 }
}

fn cast_in() -> CastIn {
    CastIn {
        fixed_cost: None,
        entries: smallvec![cost_entry()],
        free: false,
        skill: 0.0,
        mana: 1000.0,
        silenced: false,
        recharging: false,
        cooldown: 7,
        k: CostConsts::default(),
    }
}

fn ready_cost(plan: CastPlan) -> f32 {
    match plan {
        CastPlan::Ready { cost, chance, cooldown } => {
            assert_near(chance, 1.0);
            assert_eq!(cooldown, 7);
            cost
        }
        CastPlan::Refused(r) => panic!("refused: {r:?}"),
    }
}

#[test]
fn entry_cost_formula() {
    let k = CostConsts::default();
    assert_near(entry_cost(&cost_entry(), &k), 20.0);
    // Area: 10 m at 5 m per unit triples the cost.
    assert_near(entry_cost(&CostEntryIn { area: 10.0, ..cost_entry() }, &k), 60.0);
    // No magnitude, instant: base · 1 · 1 · scale.
    assert_near(entry_cost(&CostEntryIn { magnitude: (0.0, 0.0), duration: 0.0, ..cost_entry() }, &k), 0.2);
    // A zero area unit makes area free; a negative area counts as none.
    let free_area = CostConsts { area_unit: 0.0, ..k };
    assert_near(entry_cost(&CostEntryIn { area: 10.0, ..cost_entry() }, &free_area), 20.0);
    assert_near(entry_cost(&CostEntryIn { area: -10.0, ..cost_entry() }, &k), 20.0);
}

#[test]
fn cast_rule_cost_skill_and_refusals() {
    assert_near(ready_cost(cast_rule(&cast_in())), 20.0);
    // Skill 50: a quarter off; the cap is 100 (half off); below 0 counts as 0.
    assert_near(ready_cost(cast_rule(&CastIn { skill: 50.0, ..cast_in() })), 15.0);
    assert_near(ready_cost(cast_rule(&CastIn { skill: 250.0, ..cast_in() })), 10.0);
    assert_near(ready_cost(cast_rule(&CastIn { skill: -5.0, ..cast_in() })), 20.0);
    // A fixed cost replaces the entries' sum and is lowered by the skill too.
    assert_near(ready_cost(cast_rule(&CastIn { fixed_cost: Some(40.0), ..cast_in() })), 40.0);
    assert_near(ready_cost(cast_rule(&CastIn { fixed_cost: Some(40.0), skill: 100.0, ..cast_in() })), 20.0);
    // Exactly enough mana casts; a little less is refused; free costs 0 with no mana.
    let cost = ready_cost(cast_rule(&CastIn { skill: 50.0, ..cast_in() }));
    assert_near(ready_cost(cast_rule(&CastIn { skill: 50.0, mana: cost, ..cast_in() })), cost);
    assert_eq!(
        cast_rule(&CastIn { skill: 50.0, mana: cost - 0.01, ..cast_in() }),
        CastPlan::Refused(CastFailReason::NoMana)
    );
    assert_near(ready_cost(cast_rule(&CastIn { free: true, mana: 0.0, ..cast_in() })), 0.0);
    // Refusal order: recharging, silenced, no mana.
    let all = CastIn { recharging: true, silenced: true, mana: 0.0, ..cast_in() };
    assert_eq!(cast_rule(&all), CastPlan::Refused(CastFailReason::Recharging));
    assert_eq!(cast_rule(&CastIn { recharging: false, ..all.clone() }), CastPlan::Refused(CastFailReason::Silenced));
    assert_eq!(
        cast_rule(&CastIn { recharging: false, silenced: false, ..all }),
        CastPlan::Refused(CastFailReason::NoMana)
    );
}

#[test]
fn magnitude_rule_is_uniform_in_min_max() {
    assert_near(magnitude_rule(&MagnitudeIn { min: 2.0, max: 10.0, roll: 0.0 }), 2.0);
    assert_near(magnitude_rule(&MagnitudeIn { min: 2.0, max: 10.0, roll: 0.5 }), 6.0);
    assert_near(magnitude_rule(&MagnitudeIn { min: 2.0, max: 10.0, roll: 0.75 }), 8.0);
    assert_near(magnitude_rule(&MagnitudeIn { min: 4.0, max: 4.0, roll: 0.9 }), 4.0);
}

#[test]
fn resist_rule_is_deterministic_and_clamped() {
    let r = |resist, weakness| resist_rule(&ResistIn { resist, weakness });
    assert_near(r(0.0, 0.0), 1.0);
    assert_near(r(50.0, 0.0), 0.5);
    assert_near(r(100.0, 0.0), 0.0);
    assert_near(r(150.0, 0.0), 0.0);
    assert_near(r(0.0, 25.0), 1.25);
    assert_near(r(20.0, 50.0), 1.3);
    assert_near(r(0.0, 300.0), 2.0);
}

#[test]
fn protect_rule_rolls_absorb_then_reflect() {
    let base = ProtectIn { reflect: 0.0, absorb: 0.0, can_reflect: true, can_absorb: true, cost: 12.0, rolls: [0.0, 0.0] };
    // A chance of 0 never triggers, even on a roll of 0.
    assert_eq!(protect_rule(&base), Protection::Applied);
    let absorb = ProtectIn { absorb: 50.0, ..base };
    assert_eq!(protect_rule(&ProtectIn { rolls: [0.49, 0.0], ..absorb }), Protection::Absorbed { gain: 12.0 });
    assert_eq!(protect_rule(&ProtectIn { rolls: [0.5, 0.0], ..absorb }), Protection::Applied);
    assert_eq!(protect_rule(&ProtectIn { can_absorb: false, ..absorb }), Protection::Applied);
    // Absorb wins over reflect; reflect rolls its own number.
    let both = ProtectIn { reflect: 100.0, absorb: 100.0, rolls: [0.999, 0.999], ..base };
    assert_eq!(protect_rule(&both), Protection::Absorbed { gain: 12.0 });
    assert_eq!(protect_rule(&ProtectIn { can_absorb: false, ..both }), Protection::Reflected);
    assert_eq!(protect_rule(&ProtectIn { can_absorb: false, can_reflect: false, ..both }), Protection::Applied);
    let reflect = ProtectIn { reflect: 30.0, can_absorb: false, ..base };
    assert_eq!(protect_rule(&ProtectIn { rolls: [0.0, 0.29], ..reflect }), Protection::Reflected);
    assert_eq!(protect_rule(&ProtectIn { rolls: [0.0, 0.3], ..reflect }), Protection::Applied);
}

#[test]
fn item_charge_rule_cost_and_max() {
    let item = ItemIn {
        fixed_cost: None,
        fixed_max: None,
        entries: smallvec![cost_entry()],
        skill: 50.0,
        uses: 10.0,
        k: CostConsts::default(),
    };
    let out = item_charge_rule(&item);
    assert_near(out.cost, 15.0);
    assert_near(out.max, 200.0);
    let out = item_charge_rule(&ItemIn { fixed_max: Some(50.0), ..item.clone() });
    assert_near(out.max, 50.0);
    let out = item_charge_rule(&ItemIn { fixed_cost: Some(8.0), skill: 0.0, ..item });
    assert_near(out.cost, 8.0);
    assert_near(out.max, 80.0);
}

// --- StandardEffectRules through the queries, on real sheets ---

fn stat(key: &str, kind: StatKind) -> StatDef {
    StatDef { key: key.into(), kind, group: "test".into(), public: false, label: format!("stat.{key}") }
}

fn table() -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let pool = StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepCurrent };
    let defs = vec![
        stat("skill", value),
        stat("silence", value),
        stat("reflect", value),
        stat("absorb", value),
        stat("resist_fire", value),
        stat("weak_fire", value),
        stat("mana", pool),
    ];
    StatTable::new(defs, &[(Role::Mana, "mana".into())]).unwrap()
}

/// A settled sheet with these bases (pools full).
fn sheet(table: &StatTable, rules: &Rules, owner: StatOwner, base: &[(StatIdx, f32)]) -> Sheet {
    let mut s = Sheet::new(table);
    let ops: Vec<StatOp> = base.iter().map(|&(stat, value)| StatOp::SetBase { stat, value }).collect();
    let inputs = Inputs::default();
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Debug, source: None, tick: 0 }, inputs: &inputs };
    assert_eq!(sf_stats::apply(table, rules, &mut s, &ops, &ctx, &mut Vec::new()), 0);
    s.settle(table, rules, &inputs, Settle::NewCharacter);
    s
}

fn defs() -> EffectTable {
    let mut fire = EffectDef::new("fire", Archetype::Pool { pool: StatTarget::Fixed("mana".into()), mode: PoolMode::Hurt });
    fire.base_cost = 2.0;
    fire.flags.harmful = true;
    fire.resisted_by = smallvec!["resist_fire".into()];
    fire.weakened_by = smallvec!["weak_fire".into()];
    let mut heal = EffectDef::new("heal", Archetype::Restore { stat: StatTarget::Fixed("skill".into()) });
    heal.base_cost = 1.0;
    let mut pure = EffectDef::new("pure", Archetype::Restore { stat: StatTarget::Fixed("skill".into()) });
    pure.flags.harmful = true;
    pure.flags.unreflectable = true;
    EffectTable::new(vec![fire, heal, pure]).unwrap()
}

fn entry(effect: &str) -> EffectEntry {
    EffectEntry { effect: effect.into(), arg: None, range: Range::Target, area: 0.0, duration: 10.0, magnitude: (5.0, 15.0) }
}

fn standard() -> StandardEffectRules {
    StandardEffectRules {
        silence: Some("silence".into()),
        reflect: Some("reflect".into()),
        spell_absorb: Some("absorb".into()),
        ..StandardEffectRules::default()
    }
}

#[test]
fn standard_cast_plan_reads_the_caster() {
    let (table, rules, defs, std_rules) = (table(), TestModel::default().rules(), defs(), standard());
    let caster = sheet(&table, &rules, CASTER, &[(SKILL, 50.0), (MANA, 100.0)]);
    let silenced = sheet(&table, &rules, CASTER, &[(SKILL, 50.0), (MANA, 100.0), (SILENCE, 1.0)]);
    let broke = sheet(&table, &rules, CASTER, &[(SKILL, 50.0), (MANA, 10.0)]);
    let source = RecordId::new("firebolt");
    let entries = [entry("fire"), entry("unknown")];
    let q = CastQuery {
        defs: &defs,
        source: &source,
        class: SourceClass::Cast,
        entries: &entries,
        cost: None,
        always_succeeds: false,
        cooldown: 7,
        ready_at: None,
        tick: 100,
        caster: caster.view(&table),
        skill: Some(SKILL),
    };
    // Fire costs 20, a quarter off at skill 50; the unknown entry costs nothing.
    assert_near(ready_cost(std_rules.plan(&q)), 15.0);
    assert_near(ready_cost(std_rules.plan(&CastQuery { skill: None, ..q })), 20.0);
    assert_near(ready_cost(std_rules.plan(&CastQuery { class: SourceClass::Power, ..q })), 0.0);
    assert_near(ready_cost(std_rules.plan(&CastQuery { ready_at: Some(100), ..q })), 15.0);
    assert_eq!(std_rules.plan(&CastQuery { ready_at: Some(101), ..q }), CastPlan::Refused(CastFailReason::Recharging));
    let silenced_q = CastQuery { caster: silenced.view(&table), ..q };
    assert_eq!(std_rules.plan(&silenced_q), CastPlan::Refused(CastFailReason::Silenced));
    // No silence stat configured: the same caster casts.
    let deaf = StandardEffectRules { silence: None, ..standard() };
    assert_near(ready_cost(deaf.plan(&silenced_q)), 15.0);
    assert_eq!(std_rules.plan(&CastQuery { caster: broke.view(&table), ..q }), CastPlan::Refused(CastFailReason::NoMana));
    // The bundle answers the same.
    let bundle = EffectRules::from_model(&std_rules);
    assert_eq!(bundle.cast.plan(&q), std_rules.plan(&q));
}

#[test]
fn standard_land_magnitude_and_resist() {
    let (table, rules, defs, std_rules) = (table(), TestModel::default().rules(), defs(), standard());
    let target = sheet(&table, &rules, OWNER, &[(RESIST_FIRE, 30.0), (WEAK_FIRE, 10.0)]);
    let (fire, heal) = (defs.get(&"fire".into()).unwrap(), defs.get(&"heal".into()).unwrap());
    let e = entry("fire");
    assert_near(std_rules.magnitude(&RollQuery { def: fire, entry: &e, rolls: [0.25, 0.9] }), 7.5);
    let q = ResistQuery { def: fire, entry: &e, caster: Some(CASTER), rolls: [0.0, 0.0], target: Some(target.view(&table)) };
    assert_near(std_rules.resist(&q), 0.8);
    // No sheet, or a def no stat resists: no resistance.
    assert_near(std_rules.resist(&ResistQuery { target: None, ..q }), 1.0);
    assert_near(std_rules.resist(&ResistQuery { def: heal, ..q }), 1.0);
}

#[test]
fn standard_protect_reflects_once_and_absorbs() {
    let (table, rules, defs, std_rules) = (table(), TestModel::default().rules(), defs(), standard());
    let mirror = sheet(&table, &rules, OWNER, &[(REFLECT, 100.0)]);
    let sponge = sheet(&table, &rules, OWNER, &[(ABSORB, 100.0), (REFLECT, 100.0)]);
    let source = RecordId::new("firebolt");
    let fire = [entry("fire")];
    let q = ProtectQuery {
        defs: &defs,
        source: &source,
        class: SourceClass::Cast,
        entries: &fire,
        caster: Some(CASTER),
        target: mirror.view(&table),
        flags: InstanceFlags::default(),
        cost: 15.0,
        rolls: [0.5, 0.5],
    };
    assert_eq!(std_rules.protect(&q), Protection::Reflected);
    let reflected = InstanceFlags { from_reflect: true, ..InstanceFlags::default() };
    assert_eq!(std_rules.protect(&ProtectQuery { flags: reflected, ..q }), Protection::Applied, "no second reflect");
    let ignore = InstanceFlags { ignore_reflect: true, ..InstanceFlags::default() };
    assert_eq!(std_rules.protect(&ProtectQuery { flags: ignore, ..q }), Protection::Applied);
    assert_eq!(std_rules.protect(&ProtectQuery { caster: None, ..q }), Protection::Applied);
    // Only harmful sources reflect, and none with an unreflectable entry.
    let heal = [entry("heal")];
    assert_eq!(std_rules.protect(&ProtectQuery { entries: &heal, ..q }), Protection::Applied);
    let mixed = [entry("fire"), entry("pure")];
    assert_eq!(std_rules.protect(&ProtectQuery { entries: &mixed, ..q }), Protection::Applied);
    // Absorb comes first and gains the cost; any source can be absorbed.
    let sq = ProtectQuery { target: sponge.view(&table), ..q };
    assert_eq!(std_rules.protect(&sq), Protection::Absorbed { gain: 15.0 });
    assert_eq!(std_rules.protect(&ProtectQuery { entries: &heal, ..sq }), Protection::Absorbed { gain: 15.0 });
    let no_absorb = InstanceFlags { ignore_absorb: true, ..InstanceFlags::default() };
    assert_eq!(std_rules.protect(&ProtectQuery { flags: no_absorb, ..sq }), Protection::Reflected);
}

#[test]
fn standard_item_charge_reads_the_user() {
    let (table, rules, defs, std_rules) = (table(), TestModel::default().rules(), defs(), standard());
    let user = sheet(&table, &rules, OWNER, &[(SKILL, 50.0)]);
    let source = RecordId::new("ring");
    let fire = [entry("fire")];
    let q = ItemQuery {
        defs: &defs,
        source: &source,
        entries: &fire,
        cost: None,
        capacity: None,
        user: Some(user.view(&table)),
        skill: Some(SKILL),
    };
    assert_near(std_rules.charge_cost(&q), 15.0);
    assert_near(std_rules.max_charge(&q), 200.0);
    assert_near(std_rules.charge_cost(&ItemQuery { user: None, ..q }), 20.0);
    assert_near(std_rules.max_charge(&ItemQuery { capacity: Some(400.0), ..q }), 400.0);
    let bundle = EffectRules::standard();
    assert_near(bundle.items.charge_cost(&ItemQuery { skill: None, ..q }), 20.0);
}

#[test]
fn test_rules_are_fixed() {
    let (table, rules, defs) = (table(), TestModel::default().rules(), defs());
    let s = sheet(&table, &rules, OWNER, &[]);
    let source = RecordId::new("x");
    let fire = [entry("fire")];
    let mut t = TestRules::default();
    let cq = CastQuery {
        defs: &defs,
        source: &source,
        class: SourceClass::Cast,
        entries: &fire,
        cost: Some(9.0),
        always_succeeds: false,
        cooldown: 3,
        ready_at: None,
        tick: 0,
        caster: s.view(&table),
        skill: None,
    };
    assert_eq!(t.plan(&cq), CastPlan::Ready { cost: 9.0, chance: 1.0, cooldown: 3 });
    t.cast = Some(CastPlan::Refused(CastFailReason::Silenced));
    assert_eq!(t.plan(&cq), CastPlan::Refused(CastFailReason::Silenced));
    let pq = ProtectQuery {
        defs: &defs,
        source: &source,
        class: SourceClass::Cast,
        entries: &fire,
        caster: Some(CASTER),
        target: s.view(&table),
        flags: InstanceFlags::default(),
        cost: 0.0,
        rolls: [0.0, 0.0],
    };
    assert_eq!(t.protect(&pq), Protection::Applied);
    t.protect = Protection::Reflected;
    assert_eq!(t.protect(&pq), Protection::Reflected);
    let iq = ItemQuery { defs: &defs, source: &source, entries: &fire, cost: Some(4.0), capacity: None, user: None, skill: None };
    assert_near(t.charge_cost(&iq), 4.0);
    assert_near(t.max_charge(&iq), 0.0);
}

fn round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let bytes = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&bytes).unwrap(), value);
}

#[test]
fn rule_inputs_and_outputs_round_trip() {
    let mut cast = cast_in();
    cast.fixed_cost = Some(3.5);
    round_trip(&cast);
    round_trip(&CastPlan::Ready { cost: 15.0, chance: 1.0, cooldown: 7 });
    round_trip(&CastPlan::Refused(CastFailReason::Recharging));
    round_trip(&MagnitudeIn { min: 2.0, max: 10.0, roll: 0.5 });
    round_trip(&ProtectIn { reflect: 30.0, absorb: 5.0, can_reflect: true, can_absorb: false, cost: 12.0, rolls: [0.1, 0.2] });
    round_trip(&Protection::Absorbed { gain: 12.0 });
    round_trip(&Protection::Reflected);
    round_trip(&ResistIn { resist: 20.0, weakness: 50.0 });
    round_trip(&ItemIn {
        fixed_cost: None,
        fixed_max: Some(50.0),
        entries: smallvec![cost_entry(), CostEntryIn { area: 3.0, ..cost_entry() }],
        skill: 12.0,
        uses: 10.0,
        k: CostConsts::default(),
    });
    round_trip(&ItemCharge { cost: 15.0, max: 200.0 });
    round_trip(&standard());
}
