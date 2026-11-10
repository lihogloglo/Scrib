//! effects-adapter.md "Tests" for A1: MGEF flags, keys, resistances, the A1 archetypes and their
//! stats, SPEL / ALCH mapping and cost, and the first ops through `sf_effects::apply` with
//! `TestRules`. Exact assertions on synthetic records.

// Exact float assertions are the point (CONTRIBUTING "Tests": logic and data are exact).
#![allow(clippy::float_cmp)]

use sf_effects::{
    ActiveSet, ArgKind, ArgRef, Archetype, EffectCtx, EffectIdAlloc, EffectKey, EffectOut, LayerRule, PoolMode, Range,
    RollSeed, SourceClass, StatTarget, TestRules, apply, tick,
};
use sf_stats::{ActorKey, Layer, ModOp, StatKey, StatOp, StatOwner, StatTable};
use tes3::esp::{
    Alchemy, AttributeId2, Effect, EffectId, EffectId2, EffectRange, EffectSchool, MagicEffect, MagicEffectData,
    MagicEffectFlags, SkillId2, Spell, SpellData, SpellFlags, SpellType,
};

use crate::*;

const TARGET: StatOwner = StatOwner::Actor(ActorKey(1));
const CASTER: StatOwner = StatOwner::Actor(ActorKey(2));
const DT: f32 = 0.25;

fn mgef(id: EffectId, school: EffectSchool, base_cost: f32, flags: u32) -> MagicEffect {
    MagicEffect {
        effect_id: id,
        data: MagicEffectData {
            school,
            base_cost,
            flags: MagicEffectFlags::from_bits_truncate(flags),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// MGEF records for the effects the tests use (every record flag bit set on Fortify Attribute).
pub(crate) fn effects() -> MwEffects {
    let d = EffectSchool::Destruction;
    let r = EffectSchool::Restoration;
    MwEffects::from_records(&[
        mgef(EffectId::FireDamage, d, 5.0, 0),
        mgef(EffectId::DrainSkill, d, 1.0, 0),
        mgef(EffectId::DamageAttribute, d, 8.0, 0),
        mgef(EffectId::RestoreHealth, r, 1.0, 0),
        mgef(EffectId::FortifyAttribute, r, 1.0, 0x3_FFFF),
        mgef(EffectId::FortifySkill, r, 1.0, 0),
        mgef(EffectId::ResistFire, r, 2.0, 0),
    ])
}

pub(crate) fn entry(effect: EffectId2, range: EffectRange, magnitude: (u32, u32), duration: u32, area: u32) -> Effect {
    Effect {
        magic_effect: effect,
        skill: SkillId2::None,
        attribute: AttributeId2::None,
        range,
        area,
        duration,
        min_magnitude: magnitude.0,
        max_magnitude: magnitude.1,
    }
}

pub(crate) fn spell(id: &str, spell_type: SpellType, flags: SpellFlags, cost: u32, effects: Vec<Effect>) -> Spell {
    Spell { id: id.to_owned(), effects, data: SpellData { spell_type, cost, flags }, ..Default::default() }
}

fn key(k: &str) -> StatKey {
    StatKey::new(k)
}

#[test]
fn keys_labels_and_indices() {
    assert_eq!(effect_key(79), Some(EffectKey::new("fortifyattribute")));
    assert_eq!(effect_name(84), Some("FortifyMaximumMagicka"));
    assert_eq!(effect_index("FORTIFYMAXIMUMMAGICKA"), Some(84));
    assert_eq!(effect_label(19), Some("sEffectDrainSpellpoints"));
    assert_eq!(effect_label(34), Some("sEffectWeaknesstoCorprusDisease"));
    assert_eq!(effect_name(142), Some("SummonCreature05"));
    assert_eq!(effect_name(EFFECT_COUNT), None);
    let mut names: Vec<&str> = (0..EFFECT_COUNT).filter_map(effect_name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EFFECT_COUNT);
}

#[test]
fn record_flags_are_masked_then_or_hardcoded() {
    assert_eq!(hardcoded_flags(79), 0x11C2);
    assert_eq!(effective_flags(79, 0x3_FFFF), 0x11C2 | flag::RECORD_MASK);
    let fx = effects();
    let info = fx.info(79).unwrap();
    assert!(info.has_record);
    assert_eq!(info.flags, 0x11C2 | 0x200 | 0x400 | 0x800);
    assert_eq!(info.school, key("restoration"));
    assert_eq!(info.ranges(), RangeFlags { on_self: true, touch: true, target: true });
    let def = fx.table().get(&EffectKey::new("fortifyattribute")).unwrap();
    // The record's harmful bit is not kept; applied-once is hardcoded.
    assert!(!def.flags.harmful);
    assert!(def.flags.applied_once);
    assert_eq!(def.base_cost, 1.0);
    assert_eq!(def.label, "sEffectFortifyAttribute");
    // Fire Damage: harmful, a rate (not applied once).
    let fire = fx.table().get(&EffectKey::new("firedamage")).unwrap();
    assert!(fire.flags.harmful);
    assert!(fire.is_rate());
    assert_eq!(fx.info(14).unwrap().school, key("destruction"));
}

#[test]
fn indices_without_record_and_bad_records() {
    let bad = mgef(EffectId::None, EffectSchool::Alteration, 0.0, 0);
    let fx = MwEffects::from_records([&bad]);
    assert_eq!(fx.bad_records(), &[-1]);
    assert_eq!(fx.missing().count(), EFFECT_COUNT);
    // Defs exist without records (hardcoded flags, cost 0).
    let def = fx.table().get(&EffectKey::new("drainskill")).unwrap();
    assert_eq!(def.base_cost, 0.0);
    assert!(def.flags.applied_once);
}

#[test]
fn resistances_per_effect() {
    let fx = effects();
    let fire = fx.table().get(&EffectKey::new("firedamage")).unwrap();
    assert_eq!(fire.resisted_by.as_slice(), &[key("resistfire"), key("fireshield")]);
    assert_eq!(fire.weakened_by.as_slice(), &[key("weaknesstofire")]);
    let frost = fx.table().get(&EffectKey::new("frostdamage")).unwrap();
    assert_eq!(frost.resisted_by.as_slice(), &[key("resistfrost"), key("frostshield")]);
    let shock = fx.table().get(&EffectKey::new("shockdamage")).unwrap();
    assert_eq!(shock.resisted_by.as_slice(), &[key("resistshock"), key("lightningshield")]);
    let drain = fx.table().get(&EffectKey::new("drainskill")).unwrap();
    assert_eq!(drain.resisted_by.as_slice(), &[key("resistmagicka")]);
    assert_eq!(drain.weakened_by.as_slice(), &[key("weaknesstomagicka")]);
    let poison = fx.table().get(&EffectKey::new("poison")).unwrap();
    assert_eq!(poison.resisted_by.as_slice(), &[key("resistpoison")]);
    assert_eq!(poison.weakened_by.as_slice(), &[key("weaknesstopoison")]);
    let restore = fx.table().get(&EffectKey::new("restorehealth")).unwrap();
    assert!(restore.resisted_by.is_empty() && restore.weakened_by.is_empty());
    let weakness = fx.table().get(&EffectKey::new("weaknesstofire")).unwrap();
    assert_eq!(weakness.resisted_by.as_slice(), &[key("resistmagicka")]);
    // Outside A1, from the same maps.
    assert_eq!(resistance_effect(37), Some(42));
    assert_eq!(resistance_effect(45), Some(99));
    assert_eq!(weakness_effect(45), None);
    assert_eq!(resistance_effect(133), Some(94));
    assert_eq!(weakness_effect(132), Some(34));
    assert_eq!(resistance_effect(75), None);
}

#[test]
fn a1_archetypes() {
    let attribute = ArgKind::Stat("attribute".to_owned());
    let skill = ArgKind::Stat("skill".to_owned());
    let pool = |k: &str, mode| Archetype::Pool { pool: StatTarget::Fixed(key(k)), mode };
    assert_eq!(
        archetype(79),
        Some((Archetype::Modify { stat: StatTarget::Arg, layer: LayerRule::BySource }, attribute.clone()))
    );
    assert_eq!(archetype(83), Some((Archetype::Modify { stat: StatTarget::Arg, layer: LayerRule::BySource }, skill.clone())));
    assert_eq!(archetype(17), Some((Archetype::Drain { stat: StatTarget::Arg }, attribute.clone())));
    assert_eq!(archetype(21), Some((Archetype::Drain { stat: StatTarget::Arg }, skill.clone())));
    assert_eq!(archetype(22), Some((Archetype::Damage { stat: StatTarget::Arg, cap_at_modified: true }, attribute.clone())));
    assert_eq!(archetype(26), Some((Archetype::Damage { stat: StatTarget::Arg, cap_at_modified: true }, skill.clone())));
    assert_eq!(archetype(74), Some((Archetype::Restore { stat: StatTarget::Arg }, attribute)));
    assert_eq!(archetype(78), Some((Archetype::Restore { stat: StatTarget::Arg }, skill)));
    assert_eq!(archetype(18), Some((pool("health", PoolMode::Drain), ArgKind::None)));
    assert_eq!(archetype(24), Some((pool("magicka", PoolMode::Hurt), ArgKind::None)));
    assert_eq!(archetype(77), Some((pool("fatigue", PoolMode::Heal), ArgKind::None)));
    assert_eq!(archetype(81), Some((pool("magicka", PoolMode::Boost), ArgKind::None)));
    assert_eq!(archetype(15), Some((pool("health", PoolMode::Hurt), ArgKind::None)));
    assert_eq!(archetype(27), Some((pool("health", PoolMode::Hurt), ArgKind::None)));
    let magnitude = |k: &str| Some((Archetype::MagnitudeStat { stat: StatTarget::Fixed(key(k)) }, ArgKind::None));
    assert_eq!(archetype(84), magnitude("fortifymaximummagicka"));
    assert_eq!(archetype(90), magnitude("resistfire"));
    assert_eq!(archetype(36), magnitude("weaknesstonormalweapons"));
    assert_eq!(archetype(5), magnitude("lightningshield"));
    // Not in A1: absorb, levitate, sun damage, summons.
    for i in [10, 85, 102, 135] {
        assert_eq!(archetype(i), None, "{i}");
    }
    assert_eq!(MwEffects::from_records(std::iter::empty()).table().len(), 47);
}

/// The game's stat table: `MwTable`'s (SKIL data is irrelevant to effects).
fn stat_table() -> StatTable {
    let skill = mw_stats::SkillInfo { governing: 0, specialization: 0, use_values: [0.0; 4] };
    mw_stats::MwTable::with_skills([skill; 27]).unwrap().table
}

#[test]
fn magnitude_stats_match_the_mgef_table() {
    for (n, &i) in MAGNITUDE_STAT_EFFECTS.iter().enumerate() {
        let (key, label) = mw_stats::MAGNITUDE_STATS[n];
        assert_eq!(mgef::magnitude_stat_key(i), StatKey::new(key), "MGEF {i}");
        assert_eq!(effect_label(i), Some(label), "MGEF {i}");
    }
}

#[test]
fn every_a1_stat_is_in_the_table() {
    let stats = stat_table();
    let resist = stats.find("resistfire").unwrap();
    assert_eq!(stats.def(resist).group, "magnitude");
    assert_eq!(stats.def(resist).label, "sEffectResistFire");
    let fx = MwEffects::from_records(std::iter::empty());
    for (_, def) in fx.table().defs() {
        if let StatTarget::Fixed(k) = def.archetype.stat() {
            assert!(stats.idx(k).is_some(), "{} writes {k:?}", def.key);
        }
        if let ArgKind::Stat(group) = &def.arg {
            assert!(stats.defs().any(|(_, s)| s.group == *group), "{}", def.key);
        }
        for k in def.resisted_by.iter().chain(&def.weakened_by) {
            assert!(stats.idx(k).is_some(), "{} reads {k:?}", def.key);
        }
    }
}

#[test]
fn spell_maps_with_autocalc_cost() {
    let fx = effects();
    let mut fortify = entry(EffectId2::FortifyAttribute, EffectRange::OnSelf, (5, 5), 30, 0);
    fortify.attribute = AttributeId2::Strength;
    let s = spell(
        "Fire_And_Might",
        SpellType::Spell,
        SpellFlags::AUTO_CALCULATE,
        7,
        vec![entry(EffectId2::FireDamage, EffectRange::OnTarget, (10, 20), 5, 10), fortify],
    );
    let (src, notes) = map_spell(&fx, &MwEffectGmst::vanilla(), &s);
    assert!(notes.is_empty());
    assert_eq!(src.id, sf_records::RecordId::new("fire_and_might"));
    assert_eq!(src.class, SourceClass::Cast);
    assert_eq!((src.recharge, src.affliction), (None, None));
    assert_eq!(src.flags, SourceFlags { always_succeeds: false, auto_cost: true, starts_known: false });
    // Fire: 0.5·30 · 0.1·5 · 5 + 0.05·10·5 = 40, ×0.5 = 20, ×1.5 at target = 30.
    // Fortify: 0.5·10 · 0.1·1 · 30 = 15, ×0.5 = 7.5. round(37.5) = 38.
    assert_eq!(src.cost, Some(38.0));
    assert_eq!(src.effects.len(), 2);
    let e = &src.effects[0];
    assert_eq!(e.effect, EffectKey::new("firedamage"));
    assert_eq!((e.arg.clone(), e.range, e.area, e.duration, e.magnitude), (None, Range::Target, 10.0 * FOOT_M, 5.0, (10.0, 20.0)));
    let e = &src.effects[1];
    assert_eq!(e.effect, EffectKey::new("fortifyattribute"));
    assert_eq!(e.arg, Some(ArgRef::Stat(key("strength"))));
    assert_eq!((e.range, e.area, e.duration, e.magnitude), (Range::OnSelf, 0.0, 30.0, (5.0, 5.0)));
    // Without autocalc the record's cost.
    let manual = spell("m", SpellType::Spell, SpellFlags::empty(), 7, s.effects.clone());
    assert_eq!(spell_cost(&fx, &MwEffectGmst::vanilla(), &manual), 7.0);
}

#[test]
fn spell_types_and_flags() {
    let fx = effects();
    let g = MwEffectGmst::vanilla();
    let one = || vec![entry(EffectId2::RestoreHealth, EffectRange::OnSelf, (1, 1), 1, 0)];
    let class = |t, flags| map_spell(&fx, &g, &spell("x", t, flags, 0, one())).0;
    assert_eq!(class(SpellType::Ability, SpellFlags::empty()).class, SourceClass::Innate);
    let power = class(SpellType::Power, SpellFlags::empty());
    assert_eq!((power.class, power.recharge), (SourceClass::Power, Some(Recharge::GameDays(1))));
    for (t, a) in
        [(SpellType::Disease, Affliction::Disease), (SpellType::Blight, Affliction::Blight), (SpellType::Curse, Affliction::Curse)]
    {
        let s = class(t, SpellFlags::empty());
        assert_eq!((s.class, s.affliction), (SourceClass::Affliction, Some(a)));
    }
    let s = class(SpellType::Spell, SpellFlags::ALWAYS_SUCCEEDS | SpellFlags::PC_START_SPELL);
    assert_eq!(s.flags, SourceFlags { always_succeeds: true, auto_cost: false, starts_known: true });
    assert_eq!(s.cost, Some(0.0));
}

#[test]
fn unknown_effects_dropped_and_args_checked() {
    let fx = effects();
    let mut stray = entry(EffectId2::RestoreHealth, EffectRange::OnSelf, (1, 1), 1, 0);
    stray.attribute = AttributeId2::Luck;
    let mut missing = entry(EffectId2::FortifySkill, EffectRange::OnSelf, (1, 1), 1, 0);
    missing.attribute = AttributeId2::Luck;
    let mut ok = entry(EffectId2::DrainSkill, EffectRange::OnTouch, (1, 1), 1, 0);
    ok.skill = SkillId2::LongBlade;
    let effects = vec![
        entry(EffectId2::Levitate, EffectRange::OnSelf, (1, 1), 1, 0), // no MGEF record in effects()
        entry(EffectId2::None, EffectRange::OnSelf, (1, 1), 1, 0),
        stray,
        missing,
        ok,
    ];
    let mut notes = Vec::new();
    let list = map_effects(&fx, &effects, false, &mut notes);
    assert_eq!(
        notes,
        vec![
            MapNote::DroppedEffect { index: 10 },
            MapNote::DroppedEffect { index: -1 },
            MapNote::MissingArg { effect: EffectKey::new("fortifyskill"), value: -1 },
        ]
    );
    assert_eq!(list.len(), 3);
    assert_eq!(list[0].arg, None); // the stray attribute is cleared
    assert_eq!(list[1].arg, None);
    assert_eq!(list[2].arg, Some(ArgRef::Stat(key("longblade"))));
    assert_eq!(list[2].range, Range::Touch);
}

#[test]
fn potion_maps_to_self_without_area() {
    let fx = effects();
    let potion = Alchemy {
        id: "p_fire".to_owned(),
        effects: vec![entry(EffectId2::FireDamage, EffectRange::OnTarget, (3, 4), 6, 5)],
        ..Default::default()
    };
    let (src, notes) = map_potion(&fx, &potion);
    assert!(notes.is_empty());
    assert_eq!((src.class, src.cost), (SourceClass::Consumable, None));
    let e = &src.effects[0];
    assert_eq!((e.range, e.area, e.duration, e.magnitude), (Range::OnSelf, 0.0, 6.0, (3.0, 4.0)));
}

/// Lands `ops` on [`TARGET`] at tick 0 and returns the output.
fn land(fx: &MwEffects, stats: &StatTable, set: &mut ActiveSet, ids: &mut EffectIdAlloc, ops: &[sf_effects::EffectOp]) -> EffectOut {
    let mut out = EffectOut::default();
    let mut ctx = EffectCtx { target: TARGET, tick: 0, dt: DT, ids };
    apply(fx.table(), stats, &TestRules::default(), set, ops, &mut ctx, &mut out);
    out
}

fn strength_potion() -> Alchemy {
    let mut e = entry(EffectId2::FortifyAttribute, EffectRange::OnSelf, (10, 10), 30, 0);
    e.attribute = AttributeId2::Strength;
    Alchemy { id: "p_str".to_owned(), effects: vec![e], ..Default::default() }
}

#[test]
fn fortify_layer_follows_the_source() {
    let (fx, stats) = (effects(), stat_table());
    let strength = stats.find("strength").unwrap();
    let (potion, _) = map_potion(&fx, &strength_potion());
    let ability = MwSource { class: SourceClass::Innate, ..potion.clone() };
    for (src, layer) in [(potion, Layer::Temporary), (ability, Layer::Permanent)] {
        let (mut set, mut ids) = (ActiveSet::new(), EffectIdAlloc::default());
        let out = land(&fx, &stats, &mut set, &mut ids, &[src.inflict(None, RollSeed(1))]);
        let first = out.stat_ops.first().map(|(owner, op, _)| (*owner, op.clone()));
        let Some((owner, StatOp::AddModifier(m))) = first else { panic!("{:?}", out.stat_ops) };
        assert_eq!((owner, m.stat, m.layer, m.op, m.value), (TARGET, strength, layer, ModOp::Add, 10.0));
    }
}

#[test]
fn drain_skill_damages_on_landing() {
    let (fx, stats) = (effects(), stat_table());
    let mut e = entry(EffectId2::DrainSkill, EffectRange::OnSelf, (5, 5), 10, 0);
    e.skill = SkillId2::LongBlade;
    let (src, _) = map_spell(&fx, &MwEffectGmst::vanilla(), &spell("d", SpellType::Spell, SpellFlags::empty(), 0, vec![e]));
    let (mut set, mut ids) = (ActiveSet::new(), EffectIdAlloc::default());
    let out = land(&fx, &stats, &mut set, &mut ids, &[src.inflict(Some(CASTER), RollSeed(2))]);
    let blade = stats.find("longblade").unwrap();
    assert!(
        out.stat_ops.iter().any(|(_, op, _)| *op == StatOp::Damage { stat: blade, amount: 5.0, cap_at_modified: false }),
        "{:?}",
        out.stat_ops
    );
}

#[test]
fn fire_damage_hurts_per_second() {
    let (fx, stats) = (effects(), stat_table());
    let e = entry(EffectId2::FireDamage, EffectRange::OnSelf, (8, 8), 2, 0);
    let (src, _) = map_spell(&fx, &MwEffectGmst::vanilla(), &spell("f", SpellType::Spell, SpellFlags::empty(), 0, vec![e]));
    let (mut set, mut ids) = (ActiveSet::new(), EffectIdAlloc::default());
    land(&fx, &stats, &mut set, &mut ids, &[src.inflict(Some(CASTER), RollSeed(3))]);
    let mut out = EffectOut::default();
    let ctx = EffectCtx { target: TARGET, tick: 1, dt: DT, ids: &mut ids };
    tick(fx.table(), &stats, &mut set, &ctx, &mut out);
    let health = stats.find("health").unwrap();
    let hurt = StatOp::PoolAdjust { pool: health, delta: -2.0, below_zero: false, above_max: false };
    assert!(out.stat_ops.iter().any(|(_, op, _)| *op == hurt), "{:?}", out.stat_ops);
}

#[test]
fn potions_stack_spells_replace() {
    let (fx, stats) = (effects(), stat_table());
    let (potion, _) = map_potion(&fx, &strength_potion());
    let (mut set, mut ids) = (ActiveSet::new(), EffectIdAlloc::default());
    land(&fx, &stats, &mut set, &mut ids, &[potion.inflict(None, RollSeed(1)), potion.inflict(None, RollSeed(2))]);
    assert_eq!(set.sources().len(), 2);

    let mut e = entry(EffectId2::FortifySkill, EffectRange::OnSelf, (5, 5), 30, 0);
    e.skill = SkillId2::Sneak;
    let (s, _) = map_spell(&fx, &MwEffectGmst::vanilla(), &spell("sneaky", SpellType::Spell, SpellFlags::empty(), 0, vec![e]));
    let (mut set, mut ids) = (ActiveSet::new(), EffectIdAlloc::default());
    land(&fx, &stats, &mut set, &mut ids, &[s.inflict(Some(CASTER), RollSeed(1)), s.inflict(Some(CASTER), RollSeed(2))]);
    assert_eq!(set.sources().len(), 1);
    // From a second caster it stacks.
    land(&fx, &stats, &mut set, &mut ids, &[s.inflict(Some(TARGET), RollSeed(3))]);
    assert_eq!(set.sources().len(), 2);
}
