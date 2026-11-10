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
