//! effects-adapter.md "Tests" for A3: SPEL records as `SourceDef`s (class, recharge, cure tag,
//! school skill, cost, flags), the source table, and the start spell lists. Exact assertions on
//! synthetic records.

#![allow(clippy::float_cmp)]

use sf_effects::{GameMinutes, SourceClass, Tag};
use sf_records::RecordId;
use tes3::esp::{AttributeId2, EffectId2, EffectRange, SpellFlags, SpellType};

use crate::tests::{effects, entry, spell};
use crate::*;

#[test]
fn spel_records_map_to_source_defs() {
    let fx = effects();
    let g = MwEffectGmst::vanilla();
    let mut fortify = entry(
        EffectId2::FortifyAttribute,
        EffectRange::OnSelf,
        (10, 10),
        0,
        0,
    );
    fortify.attribute = AttributeId2::Strength;
    let resist = entry(EffectId2::ResistFire, EffectRange::OnSelf, (50, 50), 0, 0);
    let fire = entry(EffectId2::FireDamage, EffectRange::OnTarget, (10, 10), 1, 0);
    let def = |t, flags, cost, list: Vec<_>| {
        map_spell(&fx, &g, &spell("Src", t, flags, cost, list))
            .0
            .source_def(&fx)
    };

    let ability = def(
        SpellType::Ability,
        SpellFlags::empty(),
        0,
        vec![resist.clone(), fortify.clone()],
    );
    assert_eq!(ability.id, RecordId::new("src"));
    assert_eq!(ability.class, SourceClass::Innate);
    assert_eq!(
        (
            ability.skill.clone(),
            ability.recharge,
            ability.tags.len(),
            ability.cooldown
        ),
        (None, None, 0, 0)
    );
    assert_eq!(ability.effects.len(), 2);
    assert!(ability.lives_in_book() && !ability.is_castable());

    // A power: one game day on the world clock; the school of its first effect.
    let power = def(
        SpellType::Power,
        SpellFlags::empty(),
        5,
        vec![fortify.clone()],
    );
    assert_eq!(power.class, SourceClass::Power);
    assert_eq!(power.recharge, Some(GameMinutes::DAY));
    assert_eq!(power.recharge, Some(GameMinutes(1440)));
    assert_eq!(power.skill, Some("restoration".into()));
    assert_eq!(power.cost, Some(5.0));

    for (t, tag) in [
        (SpellType::Curse, "curse"),
        (SpellType::Disease, "disease"),
        (SpellType::Blight, "blight"),
    ] {
        let d = def(t, SpellFlags::empty(), 0, vec![fortify.clone()]);
        assert_eq!(d.class, SourceClass::Affliction);
        assert_eq!(d.tags.as_slice(), [Tag::new(tag)]);
        assert_eq!((d.skill.clone(), d.recharge), (None, None));
    }

    // A spell: autocalc cost 0.5·20 · 0.1·5 · 1 = 5, ×0.5 = 2.5, ×1.5 at target = 3.75 → 4.
    let s = def(
        SpellType::Spell,
        SpellFlags::AUTO_CALCULATE | SpellFlags::PC_START_SPELL,
        99,
        vec![fire, fortify],
    );
    assert_eq!(s.class, SourceClass::Cast);
    assert_eq!(s.cost, Some(4.0));
    assert_eq!(
        s.skill,
        Some("destruction".into()),
        "the first effect's school"
    );
    assert_eq!(
        s.flags,
        sf_effects::SourceFlags {
            always_succeeds: false,
            auto_cost: true,
            starts_known: true
        }
    );
    assert_eq!((s.recharge, s.tags.len()), (None, 0));
}

#[test]
fn the_source_table_holds_every_spell_once() {
    let fx = effects();
    let resist = entry(EffectId2::ResistFire, EffectRange::OnSelf, (50, 50), 0, 0);
    let unknown = entry(EffectId2::Levitate, EffectRange::OnSelf, (1, 1), 1, 0);
    let spells = [
        spell(
            "a",
            SpellType::Ability,
            SpellFlags::empty(),
            0,
            vec![resist.clone()],
        ),
        spell(
            "b",
            SpellType::Spell,
            SpellFlags::empty(),
            3,
            vec![resist.clone(), unknown],
        ),
        spell("A", SpellType::Power, SpellFlags::empty(), 0, vec![resist]),
    ];
    let (table, notes) = source_table(&fx, &MwEffectGmst::vanilla(), &spells);
    assert_eq!(table.len(), 2);
    assert_eq!(
        table.get(&RecordId::new("a")).unwrap().class,
        SourceClass::Innate,
        "the first record of an id"
    );
    assert_eq!(table.get(&RecordId::new("b")).unwrap().effects.len(), 1);
    assert_eq!(
        notes,
        [(RecordId::new("b"), MapNote::DroppedEffect { index: 10 })]
    );
}

#[test]
fn start_spells_in_list_order_once_known_only() {
    let race = ["Resist_Fire_75".to_owned(), "Ancestor_Guardian".to_owned()];
    let record = [
        "fireball".to_owned(),
        "resist_fire_75".to_owned(),
        "no_such_spell".to_owned(),
    ];
    let known = |id: &RecordId| id.as_str() != "no_such_spell";
    let s = start_spells([&race[..], &record[..]], known);
    let ids = |v: &[&str]| v.iter().map(|i| RecordId::new(i)).collect::<Vec<_>>();
    assert_eq!(
        s.known,
        ids(&["resist_fire_75", "ancestor_guardian", "fireball"])
    );
    assert_eq!(s.missing, ids(&["no_such_spell"]));
    assert_eq!(start_spells([], known), StartSpells::default());
}
