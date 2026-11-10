//! The host cast path headless ([`crate::effects::test_records::spell_db`], modern rules): a
//! self Restore Health spell cast through a [`CastRequest`] heals and costs magicka; a touch
//! spell lands on the contact, not on the caster; a request from another player does nothing.

#![allow(clippy::float_cmp)]

use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use mw_game_core::player::{HOST_KEY, PlayerCharacter};
use sf_character_bevy::Character;
use sf_effects_bevy::sf_effects::EffectOp;
use sf_effects_bevy::{EffectCommand, Effects};
use sf_records::RecordId;
use sf_rpg::PlayerId;
use sf_stats_bevy::sf_stats::{CauseKind, StatOp, StatOwner};
use sf_stats_bevy::{StatCommand, StatTableRes, Stats};

use super::CastRequest;
use crate::effects::test_records::{HEAL_SPELL, TOUCH_SPELL};

const HERO: StatOwner = StatOwner::Player(HOST_KEY);

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(Time::<Fixed>::from_hz(60.0));
    let db = crate::effects::test_records::spell_db(&[("rat", 20)]);
    crate::stats::add(&mut app, Some(db));
    app.update();
    app
}

fn player(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<PlayerCharacter>>()
        .single(app.world())
        .unwrap()
}

/// `(modified or max, current)` of `key` on `e`.
fn stat(app: &App, e: Entity, key: &str) -> (f32, f32) {
    let table = &app.world().resource::<StatTableRes>().0;
    let idx = table.find(key).unwrap();
    let v = app.world().get::<Stats>(e).unwrap().view(table);
    (v.modified(idx), v.current(idx))
}

fn learn(app: &mut App, spell: &str) {
    let op = EffectOp::Learn {
        source: RecordId::new(spell),
    };
    app.world_mut()
        .write_message(EffectCommand::new(HERO, vec![op]));
    app.update();
}

fn cast(app: &mut App, player: PlayerId, spell: &str, target: Option<StatOwner>) {
    app.world_mut().write_message(CastRequest {
        player,
        source: RecordId::new(spell),
        target,
    });
}

#[test]
fn a_self_heal_cast_heals_and_costs_magicka() {
    let mut app = app();
    let hero = player(&mut app);
    learn(&mut app, HEAL_SPELL);
    let health = app
        .world()
        .resource::<StatTableRes>()
        .0
        .find("health")
        .unwrap();
    let hurt = StatOp::PoolAdjust {
        pool: health,
        delta: -30.0,
        below_zero: false,
        above_max: false,
    };
    app.world_mut()
        .write_message(StatCommand::new(HERO, CauseKind::Debug, vec![hurt]));
    app.update();
    let (max, hurt) = stat(&app, hero, "health");
    let magicka = stat(&app, hero, "magicka");
    assert_eq!(hurt, max - 30.0);
    assert_eq!(magicka.1, magicka.0, "full magicka");

    cast(&mut app, PlayerId::HOST, HEAL_SPELL, None);
    for _ in 0..150 {
        app.update();
    }
    assert_eq!(stat(&app, hero, "magicka").1, magicka.1 - 5.0, "cost 5");
    let healed = stat(&app, hero, "health").1;
    assert!(
        (healed - (hurt + 20.0)).abs() < 1e-3,
        "10 a second for 2 s: {healed} from {hurt}"
    );
    let heal = RecordId::new(HEAL_SPELL);
    let fx = app.world().get::<Effects>(hero).unwrap();
    assert!(
        fx.active.sources().iter().all(|s| s.source.record != heal),
        "the heal ended"
    );

    // Another player's request: nothing cast.
    cast(&mut app, PlayerId::new(1).unwrap(), HEAL_SPELL, None);
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(stat(&app, hero, "magicka").1, magicka.1 - 5.0);
}

#[test]
fn a_touch_cast_lands_on_the_contact() {
    let mut app = app();
    let hero = player(&mut app);
    let rat = app
        .world_mut()
        .spawn(Character::record("CREA", RecordId::new("rat")))
        .id();
    app.update();
    let rat_owner = app.world().get::<Effects>(rat).unwrap().owner;
    learn(&mut app, TOUCH_SPELL);
    let hero_strength = stat(&app, hero, "strength").0;
    let magicka = stat(&app, hero, "magicka").1;

    // No contact: the touch entries are dropped, the cost is spent.
    cast(&mut app, PlayerId::HOST, TOUCH_SPELL, None);
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(stat(&app, rat, "strength").0, 20.0);
    assert_eq!(stat(&app, hero, "magicka").1, magicka - 5.0);

    cast(&mut app, PlayerId::HOST, TOUCH_SPELL, Some(rat_owner));
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(
        stat(&app, rat, "strength").0,
        30.0,
        "Fortify Strength 10 on the rat"
    );
    assert_eq!(
        stat(&app, hero, "strength").0,
        hero_strength,
        "not on the caster"
    );
    assert_eq!(stat(&app, hero, "magicka").1, magicka - 10.0);
    let rat_fx = app.world().get::<Effects>(rat).unwrap();
    assert_eq!(rat_fx.active.sources().len(), 1);
    assert_eq!(rat_fx.active.sources()[0].caster, Some(HERO));
}

#[test]
fn the_console_cast_learns_then_casts() {
    let mut app = app();
    let hero = player(&mut app);
    let magicka = stat(&app, hero, "magicka").1;
    let line = super::console_cast(app.world_mut(), HEAL_SPELL, None).unwrap();
    assert!(line.contains("learned first"), "{line}");
    for _ in 0..3 {
        app.update();
    }
    let fx = app.world().get::<Effects>(hero).unwrap();
    assert!(fx.book.knows(&RecordId::new(HEAL_SPELL)));
    assert_eq!(stat(&app, hero, "magicka").1, magicka - 5.0);
    assert!(super::console_cast(app.world_mut(), "no_such_spell", None).is_err());
}
