//! `IN38a` exact tests: the breakable set on synthetic records (each class, chests and hard
//! materials out, organic and scripted refused), the per-class def, the damage rows and the switch.

// Healths and factors are compared exactly: they are the declarations' literals, copied unchanged.
#![allow(clippy::float_cmp)]

use sf_interaction::{BreakAction, TagId};
use sf_records::RecordId;

use crate::breakable::{
    BreakClass as C, BreakFacts, Breakable, Eligibility as E, Refusal, Source, class_of, classify, damage_rules,
    destructible_def, kind, material,
};
use crate::tunables::BreakParams;

fn facts(source: Source, id: &str) -> BreakFacts<'_> {
    BreakFacts { source, id, organic: false, activate_script: false, named_by_script: false }
}

fn ok(class: C, spills: bool) -> E {
    E::Breakable(Breakable { class, spills })
}

#[test]
fn each_class_from_its_record() {
    for (source, id, want) in [
        (Source::Container, "contain_crate_01", ok(C::Crate, true)),
        (Source::Container, "Contain_Barrel_10", ok(C::Barrel, true)),
        (Source::Container, "contain_urn_05", ok(C::Urn, true)),
        (Source::Container, "contain_sack_02", ok(C::Sack, true)),
        (Source::Misc, "misc_com_redware_cup", ok(C::Pottery, false)),
        (Source::Misc, "misc_de_pot_redware_03", ok(C::Pottery, false)),
        (Source::Misc, "misc_com_bottle_07", ok(C::Glass, false)),
        (Source::Misc, "misc_de_glass_green_01", ok(C::Glass, false)),
        (Source::Static, "furn_de_urn_01", ok(C::Urn, false)),
        (Source::Static, "furn_com_pot_02", ok(C::Pottery, false)),
    ] {
        assert_eq!(classify(&facts(source, id)), want, "{id}");
    }
}

#[test]
fn chests_hard_materials_and_architecture_stay_out() {
    for (source, id) in [
        (Source::Container, "contain_chest_small_01"),
        (Source::Container, "com_chest_02"),
        (Source::Container, "contain_corpse00"),
        (Source::Container, "contain_closet_01"),
        (Source::Misc, "misc_imp_silverware_cup"),
        (Source::Misc, "misc_com_wood_bowl_01"),
        (Source::Misc, "misc_dwrv_bowl00"),
        (Source::Misc, "misc_com_broom_01"),
        (Source::Static, "ex_de_urn_tower_01"),
        (Source::Static, "in_redware_pot_wall"),
        (Source::Static, "furn_de_table_01"),
        // A `STAT` id with a pot token but a hard material.
        (Source::Static, "furn_dwrv_pot_00"),
    ] {
        assert_eq!(classify(&facts(source, id)), E::Not, "{id}");
        assert_eq!(class_of(source, id), None, "{id}");
    }
}

#[test]
fn organic_and_scripted_records_are_refused() {
    let crate_id = "contain_crate_01";
    let organic = BreakFacts { organic: true, ..facts(Source::Container, crate_id) };
    assert_eq!(classify(&organic), E::Refused(C::Crate, Refusal::Organic));
    let scripted = BreakFacts { activate_script: true, ..facts(Source::Container, crate_id) };
    assert_eq!(classify(&scripted), E::Refused(C::Crate, Refusal::ActivateScript));
    let named = BreakFacts { named_by_script: true, ..facts(Source::Misc, "misc_com_bottle_01") };
    assert_eq!(classify(&named), E::Refused(C::Glass, Refusal::NamedByScript));
    // The organic flag is a container's; a refused record's class is still reported.
    let all = BreakFacts { organic: true, activate_script: true, named_by_script: true, ..facts(Source::Container, "contain_urn_01") };
    assert_eq!(classify(&all), E::Refused(C::Urn, Refusal::Organic));
    // The organic flag means nothing outside `CONT`.
    let misc = BreakFacts { organic: true, ..facts(Source::Misc, "misc_com_redware_plate") };
    assert_eq!(classify(&misc), ok(C::Pottery, false));
}

#[test]
fn def_per_class_removes_at_zero_with_its_debris() {
    let p = BreakParams::default();
    for (b, hp, mat, debris, spill) in [
        (Breakable { class: C::Crate, spills: true }, 15.0, material::WOOD, "mw_debris_wood_planks", BreakAction::SpillContents),
        (Breakable { class: C::Barrel, spills: true }, 20.0, material::WOOD, "mw_debris_wood_planks", BreakAction::SpillContents),
        (Breakable { class: C::Urn, spills: true }, 10.0, material::CLAY, "mw_debris_clay_shards", BreakAction::SpillContents),
        (Breakable { class: C::Urn, spills: false }, 10.0, material::CLAY, "mw_debris_clay_shards", BreakAction::None),
        (Breakable { class: C::Sack, spills: true }, 8.0, material::CLOTH, "mw_debris_cloth_scraps", BreakAction::SpillContents),
        (Breakable { class: C::Pottery, spills: false }, 2.0, material::CLAY, "mw_debris_clay_shards", BreakAction::None),
        (Breakable { class: C::Glass, spills: false }, 1.0, material::GLASS, "mw_debris_glass_shards", BreakAction::None),
    ] {
        assert_eq!(b.class.material(), mat, "{b:?}");
        let d = destructible_def(b, &p).expect("a def");
        assert_eq!(d.health, hp, "{b:?}");
        assert_eq!(d.stages.len(), 1, "{b:?}");
        let s = &d.stages[0];
        assert_eq!((s.at, s.model.clone(), s.chunks.clone(), s.effect.clone()), (0.0, None, None, None), "{b:?}");
        assert_eq!(s.debris.as_slice(), &[RecordId::new(debris)], "{b:?}");
        assert_eq!(s.on_break, spill, "{b:?}");
        assert!(!d.ignore_external && d.loot.is_none() && d.salvage.is_empty(), "{b:?}");
    }
    // A tuned health is read from the params.
    let tuned = BreakParams { barrel_hp: 33.0, ..p };
    assert_eq!(destructible_def(Breakable { class: C::Barrel, spills: true }, &tuned).map(|d| d.health), Some(33.0));
}

#[test]
fn damage_rows() {
    let rows = damage_rules(&BreakParams::default());
    let factor = |k: TagId, m: TagId| rows.iter().find(|r| (r.0, r.1) == (k, m)).map(|r| r.2);
    assert_eq!(rows.len(), 4 * 4 + 1);
    for m in material::BREAKABLE {
        assert_eq!(factor(kind::BLUNT, m), Some(1.0));
        assert_eq!(factor(kind::BLADE, m), Some(0.5));
        assert_eq!(factor(kind::ARROW, m), Some(0.25));
        assert_eq!(factor(kind::HAND, m), Some(0.25));
    }
    assert_eq!(factor(kind::FIRE, material::WOOD), Some(1.5));
    // Fire on the other materials is the shell's default (1): no row.
    for m in [material::CLAY, material::GLASS, material::CLOTH] {
        assert_eq!(factor(kind::FIRE, m), None);
    }
    // No pair twice.
    let mut keys: Vec<_> = rows.iter().map(|r| (r.0, r.1)).collect();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(keys.len(), rows.len());
}

#[test]
fn classic_yields_no_def_and_no_rows() {
    let classic = BreakParams { enabled: false, ..BreakParams::default() };
    for class in [C::Crate, C::Barrel, C::Urn, C::Sack, C::Pottery, C::Glass] {
        assert_eq!(destructible_def(Breakable { class, spills: true }, &classic), None, "{class:?}");
    }
    assert!(damage_rules(&classic).is_empty());
    // The verdict itself is the same (the cook's counts do not change).
    assert_eq!(classify(&facts(Source::Container, "contain_crate_01")), ok(C::Crate, true));
}
