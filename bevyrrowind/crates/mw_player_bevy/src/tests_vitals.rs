#![allow(clippy::float_cmp)] // exact by design: the sheet gets exactly what combat applied (stats.md S5)
//! Combat vitals from character stats (stats.md S5, [`crate::vitals`]): the player's and the
//! training dummy's combat pools are their sheets', and what combat applies goes back exactly.

use mw_game_core::player::{HOST_KEY, PlayerCharacter};
use sf_action_bevy::ActionSystems;
use sf_combat_bevy::{CombatSystems, Combatant};
use sf_stats_bevy::sf_stats::{ActorKey, CauseKind, StatEvent, StatOp, StatOwner};
use sf_stats_bevy::{StatMessage, StatSystems, StatTableRes, StatTick, Stats};

use super::*;
use crate::tests::{Statics, app_setup};
use crate::vitals::CombatSheet;
use crate::vitals::testing::{FATIGUE, HEALTH, add_stats, pool_of, sheet};

const PLAYER: StatOwner = StatOwner::Player(HOST_KEY);
/// Morrowind.esm's `player` record: health, magicka, fatigue.
const PLAYER_POOLS: [f32; 3] = [35.0, 50.0, 200.0];

/// Every published stat event.
#[derive(Resource, Default)]
struct Log(Vec<StatEvent>);

fn log(mut events: MessageReader<StatMessage>, mut out: ResMut<Log>) {
    out.0.extend(events.read().map(|m| m.0.clone()));
}

/// The armed player with the stats plugin, its character's sheet and the game's action rules
/// (the sheet's moves, so a roll starts). Returns the body and the character entity.
fn armed_with_stats() -> (App, Entity, Entity) {
    let sheet_def = sf_combat::load(mw_combat::SWORD_1H, &|_| Some(0.5)).expect("sheet compiles");
    let (mut app, player) = app_setup(60, &mw_controls_bevy::keymap(), |app| {
        add_stats(app);
        combat::add(app, Some(sheet_def.clone()), &PlayerClips::default());
        app.init_resource::<Log>().add_systems(Update, log);
    });
    app.insert_resource(rules(&[], Some(&sheet_def)));
    let stats = sheet(&app, PLAYER, PLAYER_POOLS);
    let character = app.world_mut().spawn((PlayerCharacter { key: HOST_KEY }, stats)).id();
    app.world_mut().entity_mut(player).insert(PlayerBody);
    frames(&mut app, 5);
    (app, player, character)
}

fn frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}

fn tick(app: &App) -> u32 {
    app.world().resource::<StatTick>().0
}

/// `(old, new)` of every `Changed` of `stat` on `owner` with cause `Combat`.
fn combat_changes(app: &App, owner: StatOwner, stat: sf_stats_bevy::sf_stats::StatIdx) -> Vec<(f32, f32)> {
    let changes = app.world().resource::<Log>().0.iter().filter_map(|e| match *e {
        StatEvent::Changed { owner: o, stat: s, old, new, cause } if o == owner && s == stat && cause.kind == CauseKind::Combat => Some((old, new)),
        _ => None,
    });
    changes.collect()
}

#[test]
fn the_player_body_links_its_characters_sheet_and_combat_sees_its_pools() {
    let (app, player, character) = armed_with_stats();
    assert_eq!(app.world().get::<CombatSheet>(player).map(|l| l.owner), Some(PLAYER));
    assert!(app.world().get::<Stats>(character).unwrap().inputs.combatant, "combat owns the stamina regen");
    let v = app.world().get::<Combatant>(player).unwrap().fighter.vitals;
    assert_eq!((v.health.cur, v.health.max), (35.0, 35.0), "health 1:1");
    assert_eq!((v.stamina.cur, v.stamina.max), (100.0, 100.0), "stamina = fatigue x 0.5");
}

/// Presses Dodge on the player's command once.
fn press_dodge(mut chars: Query<&mut ActionCharacter, With<PlayerBody>>, mut done: Local<bool>) {
    for mut c in &mut chars {
        if !*done {
            c.command.buttons.pressed.insert(ActionClass::Dodge);
            c.command.buttons.held.insert(ActionClass::Dodge);
            *done = true;
        }
    }
}

#[test]
fn a_roll_costs_the_players_fatigue_through_the_stamina_mapping() {
    let (mut app, player, character) = armed_with_stats();
    let roll = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().rule_key("roll");
    app.add_systems(FixedUpdate, press_dodge.in_set(ActionSystems::Command));
    let mut rolled = false;
    for _ in 0..10 {
        app.update();
        rolled |= app.world().get::<ActionCharacter>(player).unwrap().state.mov == roll;
    }
    assert!(rolled, "the roll started");
    // The sheet's roll costs 12 stamina: 24 fatigue, charged on the start tick.
    let changes = combat_changes(&app, PLAYER, FATIGUE);
    assert_eq!(changes.first(), Some(&(200.0, 176.0)), "{changes:?}");
    let (fatigue, max) = pool_of(&app, character, FATIGUE);
    assert!(max == 200.0 && fatigue < max, "fatigue {fatigue}/{max}");
    assert!(app.world().resource::<Log>().0.iter().all(|e| !matches!(e, StatEvent::Changed { stat, .. } if *stat == HEALTH)), "health untouched");
}

#[test]
fn combat_regen_replaces_the_stats_regen_for_a_combatant_and_hands_it_back_without_a_body() {
    let (mut app, player, character) = armed_with_stats();
    {
        let table = app.world().resource::<StatTableRes>().0.clone();
        let rules = app.world().resource::<sf_stats_bevy::StatRules>().0.clone();
        let mut stats = app.world_mut().get_mut::<Stats>(character).unwrap();
        let cause = sf_stats_bevy::sf_stats::Cause { kind: CauseKind::Debug, source: None, tick: 0 };
        assert_eq!(stats.apply(&table, &rules, &[StatOp::PoolSetCurrent { pool: FATIGUE, value: 140.0 }], cause, &mut Vec::new()), 0);
    }
    let t0 = tick(&app);
    frames(&mut app, 20);
    let ticks = tick(&app) - t0;
    assert!(ticks >= 10, "{ticks} fixed ticks");
    // Combat's 45 stamina/s at 60 Hz = 0.75 stamina = 1.5 fatigue a tick, and nothing from the
    // stats model's own fatigue regen (10/s) on top.
    #[allow(clippy::cast_precision_loss)]
    let want = 140.0 + 1.5 * ticks as f32;
    assert_eq!(pool_of(&app, character, FATIGUE).0, want);
    // The body goes (F11, free camera): the character keeps regenerating, by the stats model.
    app.world_mut().despawn(player);
    let before = pool_of(&app, character, FATIGUE).0;
    frames(&mut app, 6);
    assert!(!app.world().get::<Stats>(character).unwrap().inputs.combatant, "no combatant links it");
    let after = pool_of(&app, character, FATIGUE).0;
    assert!(after > before && after - before < 1.5 * 6.0, "stats regen {before} -> {after}");
}

/// A landed hit as the combat step applies it: the defender's health drops by `amount`.
#[derive(Resource, Default)]
struct Hit(Option<(Entity, f32)>);

fn land_hit(mut hit: ResMut<Hit>, mut fighters: Query<&mut Combatant>) {
    let Some((who, amount)) = hit.0.take() else { return };
    if let Ok(mut c) = fighters.get_mut(who) {
        c.fighter.vitals.apply(sf_combat::PoolDelta { health: -amount, ..sf_combat::PoolDelta::default() });
    }
}

/// The armed player and a standing training dummy whose dressed character carries a sheet (as
/// `mw_rpg_bevy::stats` gives every NPC character); returns the dummy body and avatar.
fn with_dummy() -> (App, Entity, Entity) {
    let (mut app, _player, _character) = armed_with_stats();
    dummy::systems::<Statics>(&mut app);
    app.init_resource::<Hit>().add_systems(FixedUpdate, land_hit.after(CombatSystems::Step).before(StatSystems::Combat));
    let root = app.world_mut().spawn_empty().id();
    let feet = WorldPos::ORIGIN.offset(Vec3::new(0.0, 1.5, 7.0));
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (feet, 0.0), root);
    app.world_mut().flush();
    let avatar = app.world().get::<Children>(d).and_then(|c| c.iter().find(|&e| app.world().entity(e).contains::<dummy::DummyAvatar>())).unwrap();
    let stats = sheet(&app, StatOwner::Actor(ActorKey(u128::from(avatar.to_bits()))), [28.0, 40.0, 150.0]);
    app.world_mut().entity_mut(avatar).insert(stats);
    frames(&mut app, 30);
    (app, d, avatar)
}

#[test]
fn a_hit_on_the_dummy_lowers_its_sheets_health_and_a_sheet_death_refills_it() {
    let (mut app, d, avatar) = with_dummy();
    let owner = app.world().get::<Stats>(avatar).unwrap().owner;
    assert_eq!(app.world().get::<CombatSheet>(d).map(|l| l.owner), Some(owner));
    assert_eq!(pool_of(&app, avatar, HEALTH), (dummy::DUMMY_HEALTH, dummy::DUMMY_HEALTH), "the dummy's health on its sheet");
    app.world_mut().resource_mut::<Hit>().0 = Some((d, 13.0));
    frames(&mut app, 3);
    assert_eq!(pool_of(&app, avatar, HEALTH).0, 87.0, "exactly the hit's damage");
    assert_eq!(combat_changes(&app, owner, HEALTH), [(100.0, 87.0)]);
    assert_eq!(app.world().get::<Combatant>(d).unwrap().fighter.vitals.health.cur, 87.0, "combat reads it back");
    // A hit leaving 0.5 health: alive to combat (> 0), dead to the sheet (< 1): `Depleted` is the
    // death, the dummy refills.
    app.world_mut().resource_mut::<Hit>().0 = Some((d, 86.5));
    frames(&mut app, 3);
    let depleted = app.world().resource::<Log>().0.iter().any(|e| matches!(e, StatEvent::Depleted { owner: o, pool, .. } if *o == owner && *pool == HEALTH));
    assert!(depleted, "Depleted(health)");
    assert_eq!(pool_of(&app, avatar, HEALTH), (dummy::DUMMY_HEALTH, dummy::DUMMY_HEALTH), "refilled");
}
