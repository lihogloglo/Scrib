//! The effects glue headless (no install, [`super::test_records`]): `Effects` beside every
//! `Stats`, and a Fortify Attribute from the MGEF table landing through a command on a creature.

#![allow(clippy::float_cmp)]

use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::PlayerCharacter;
use sf_character_bevy::Character;
use sf_effects_bevy::sf_effects::{
    ArgRef, EffectEntry, EffectOp, InstanceFlags, Range, RollSeed, SourceClass, SourceRef,
};
use sf_effects_bevy::{EffectCommand, EffectTableRes, Effects};
use sf_records::RecordId;
use sf_stats_bevy::{StatTableRes, Stats};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(Time::<Fixed>::from_hz(60.0));
    crate::stats::add(&mut app, Some(super::test_records::db(&[("rat", 20)])));
    app.update();
    app
}

fn strength(app: &App, e: Entity) -> f32 {
    let table = &app.world().resource::<StatTableRes>().0;
    let str = table.find("strength").unwrap();
    app.world()
        .get::<Stats>(e)
        .unwrap()
        .view(table)
        .modified(str)
}

#[test]
fn effects_attach_beside_stats_and_a_fortify_lands_and_expires() {
    let mut app = app();
    let fx = app.world().resource::<EffectTableRes>().0.clone();
    let fortify = mw_effects::effect_key(79).unwrap();
    assert!(
        fx.get(&fortify).is_some(),
        "the MGEF table has Fortify Attribute"
    );

    let mut q = app
        .world_mut()
        .query_filtered::<(&Stats, &Effects), With<PlayerCharacter>>();
    let (s, e) = q.single(app.world()).unwrap();
    assert_eq!(s.owner, e.owner, "the player character has effects");

    let rat = app
        .world_mut()
        .spawn(Character::record("CREA", RecordId::new("rat")))
        .id();
    app.update();
    let owner = app
        .world()
        .get::<Effects>(rat)
        .expect("effects attached with the sheet")
        .owner;
    assert_eq!(app.world().get::<Stats>(rat).unwrap().owner, owner);
    assert_eq!(strength(&app, rat), 20.0);

    let entry = EffectEntry {
        effect: fortify,
        arg: Some(ArgRef::Stat("strength".into())),
        range: Range::OnSelf,
        area: 0.0,
        duration: 0.5,
        magnitude: (10.0, 10.0),
    };
    let op = EffectOp::Inflict {
        source: SourceRef {
            record: RecordId::new("p_fortify_strength"),
            item: None,
        },
        class: SourceClass::Consumable,
        caster: None,
        entries: [entry].into_iter().collect(),
        flags: InstanceFlags::default(),
        rolls: RollSeed(1),
    };
    app.world_mut()
        .write_message(EffectCommand::new(owner, vec![op]));
    app.update();
    assert_eq!(strength(&app, rat), 30.0, "fortified on the landing tick");
    for _ in 1..30 {
        app.update();
    }
    assert_eq!(strength(&app, rat), 20.0, "ended after 30 fixed ticks");
}

// Adapter A3: start spells (effects-adapter.md "Tests", [`super::test_records::spell_db`]).

use super::test_records::{
    NPC, NPC_ABILITY, NPC_SPELL, POOL_ABILITY, RACE_ABILITY, RACE_POWER, SIGN, SIGN_ABILITY,
};
use crate::stats::PlayerBirthsign;

fn spell_app(sign: Option<&str>) -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerBirthsign(sign.map(RecordId::new)));
    crate::stats::add(&mut app, Some(super::test_records::spell_db(&[])));
    app.update();
    app
}

fn player(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<PlayerCharacter>>()
        .single(app.world())
        .unwrap()
}

/// `(base, base + Permanent layer, modified or a pool's max, a pool's current)` of `key`.
fn stat(app: &App, e: Entity, key: &str) -> (f32, f32, f32, f32) {
    let table = &app.world().resource::<StatTableRes>().0;
    let idx = table.find(key).unwrap();
    let v = app.world().get::<Stats>(e).unwrap().view(table);
    (
        v.base(idx),
        v.effective_base(idx),
        v.modified(idx),
        v.current(idx),
    )
}

fn ids(list: &[&str]) -> Vec<RecordId> {
    list.iter().map(|i| RecordId::new(i)).collect()
}

/// The records of the active sources, in order, all innate.
fn innate(app: &App, e: Entity) -> Vec<RecordId> {
    let fx = app.world().get::<Effects>(e).unwrap();
    assert!(
        fx.active
            .sources()
            .iter()
            .all(|s| s.class == SourceClass::Innate)
    );
    fx.active
        .sources()
        .iter()
        .map(|s| s.source.record.clone())
        .collect()
}

#[test]
fn a_new_player_starts_with_race_and_birthsign_abilities() {
    let mut plain = spell_app(None);
    let mut app = spell_app(Some(SIGN));
    let (p0, p) = (player(&mut plain), player(&mut app));

    let book = app.world().get::<Effects>(p).unwrap().book.known().to_vec();
    assert_eq!(
        book,
        ids(&[RACE_ABILITY, RACE_POWER, SIGN_ABILITY]),
        "race powers, then the birthsign's; not the record's own list"
    );
    assert!(!book.contains(&RecordId::new(NPC_SPELL)));
    assert_eq!(
        innate(&app, p),
        ids(&[RACE_ABILITY, SIGN_ABILITY]),
        "abilities run, the power waits"
    );
    assert_eq!(
        plain.world().get::<Effects>(p0).unwrap().book.known(),
        ids(&[RACE_ABILITY, RACE_POWER])
    );

    // On the Permanent layer: base unchanged, effective base and modified raised.
    assert_eq!(stat(&plain, p0, "strength"), (40.0, 40.0, 40.0, 0.0));
    assert_eq!(stat(&app, p, "strength"), (40.0, 50.0, 50.0, 0.0));
    assert_eq!(stat(&app, p, "resistfire"), (0.0, 50.0, 50.0, 0.0));

    // Fortify Maximum Magicka 5 from the birthsign: (1 + 0.5) x Int 40, 20 over the plain 40,
    // and a new character starts full (`buildPlayer` sets current = modified).
    // (A derived pool's base is its formula value.)
    assert_eq!(stat(&plain, p0, "magicka"), (40.0, 40.0, 40.0, 40.0));
    assert_eq!(stat(&app, p, "magicka"), (60.0, 60.0, 60.0, 60.0));
    assert_eq!(stat(&app, p, "fortifymaximummagicka"), (0.0, 5.0, 5.0, 0.0));
}

#[test]
fn an_npc_learns_its_race_powers_then_its_record_spells_on_spawn() {
    let mut app = spell_app(None);
    let npc = app
        .world_mut()
        .spawn(Character::record("NPC_", RecordId::new(NPC)))
        .id();
    app.update();
    let fx = app.world().get::<Effects>(npc).expect("effects attached");
    assert_eq!(
        fx.book.known(),
        ids(&[RACE_ABILITY, RACE_POWER, NPC_ABILITY, NPC_SPELL]),
        "a repeated id once, an unknown one skipped"
    );
    assert_eq!(innate(&app, npc), ids(&[RACE_ABILITY, NPC_ABILITY]));
    assert_eq!(stat(&app, npc, "strength"), (40.0, 45.0, 45.0, 0.0));
    assert_eq!(stat(&app, npc, "resistfire").2, 50.0);
}

#[test]
fn forgetting_an_ability_removes_its_effect_and_learning_brings_it_back() {
    let mut app = spell_app(Some(SIGN));
    let p = player(&mut app);
    let owner = app.world().get::<Effects>(p).unwrap().owner;
    let full = stat(&app, p, "magicka");
    let forget = EffectOp::Forget {
        source: RecordId::new(SIGN_ABILITY),
    };
    app.world_mut()
        .write_message(EffectCommand::new(owner, vec![forget]));
    app.update();
    assert_eq!(stat(&app, p, "strength"), (40.0, 40.0, 40.0, 0.0));
    assert_eq!(stat(&app, p, "magicka").2, full.2 - 20.0);
    assert_eq!(innate(&app, p), ids(&[RACE_ABILITY]));
    assert!(
        !app.world()
            .get::<Effects>(p)
            .unwrap()
            .book
            .knows(&RecordId::new(SIGN_ABILITY))
    );

    let learn = EffectOp::Learn {
        source: RecordId::new(SIGN_ABILITY),
    };
    app.world_mut()
        .write_message(EffectCommand::new(owner, vec![learn]));
    app.update();
    assert_eq!(stat(&app, p, "strength").2, 50.0);
    assert_eq!(stat(&app, p, "magicka").2, full.2);
    assert_eq!(innate(&app, p), ids(&[RACE_ABILITY, SIGN_ABILITY]));
}

/// Logged bug (effects-adapter.md "As built (A3)"): an ability's Fortify Magicka is
/// `PoolModMax` (`OpenMW` `modDynamicStat` with `Flag_AffectsBaseValues`), but magicka's base
/// and max are derived (`mw_stats::magicka_max`) and every recompute overwrites the delta: the
/// sheet reads 40 / 40 before, after `Learn` and after `Forget`. Fortify Health and Fatigue
/// abilities likely the same where the pool is derived.
#[test]
#[ignore = "bug: an innate Fortify Magicka is lost on the derived magicka pool (E1 / stats)"]
fn an_ability_fortify_magicka_raises_the_maximum() {
    let mut app = spell_app(None);
    let p = player(&mut app);
    let owner = app.world().get::<Effects>(p).unwrap().owner;
    let before = stat(&app, p, "magicka");
    let learn = EffectOp::Learn {
        source: RecordId::new(POOL_ABILITY),
    };
    app.world_mut()
        .write_message(EffectCommand::new(owner, vec![learn]));
    app.update();
    assert_eq!(innate(&app, p), ids(&[RACE_ABILITY, POOL_ABILITY]));
    let after = stat(&app, p, "magicka");
    assert_eq!(
        after.2,
        before.2 + 20.0,
        "before {before:?}, after {after:?}"
    );
}
