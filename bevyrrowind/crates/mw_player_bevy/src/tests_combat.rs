//! Combat glue (combat.md "C1b"): the player body is armed with the sword sheet, F and an attack
//! press draw the weapon (the press itself doesn't attack), and the HUD shows the stamina.

use sf_action_bevy::PlayerAction;
use sf_combat_bevy::Combatant;

use super::*;
use crate::combat::{ReadyWeaponAction, Stance};
use crate::hud::PlayerVitals;
use crate::tests::{Statics, action, app_setup, mock};

fn armed() -> (App, Entity) {
    let sheet = sf_combat::load(mw_combat::SWORD_1H, &|_| Some(0.5)).expect("sheet compiles");
    let (mut app, player) = app_setup(60, &mw_controls_bevy::keymap(), |app| {
        app.insert_resource(PlayerVitals::from_record(None)).add_systems(Update, hud::sync_vitals);
        combat::add(app, Some(sheet), &PlayerClips::default());
    });
    app.world_mut().entity_mut(player).insert(PlayerBody);
    app.update();
    (app, player)
}

fn stance(app: &App, player: Entity) -> (Stance, bool) {
    let c = app.world().get::<Combatant>(player).expect("armed");
    (*app.world().get::<Stance>(player).unwrap(), c.withheld.contains(ActionClass::Light))
}

#[test]
fn the_player_is_armed_sheathed_and_the_hud_shows_its_stamina() {
    let (mut app, player) = armed();
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Sheathed, true));
    let key = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().key;
    assert_eq!(app.world().get::<ActionCharacter>(player).unwrap().state.mode.moveset, Some(key));
    let vitals = app.world().resource::<PlayerVitals>();
    let stamina = mw_combat::stamina_from_fatigue(PlayerVitals::PLACEHOLDER[2]);
    assert!((vitals.fatigue.max - stamina).abs() < 1e-3 && vitals.fatigue.current <= vitals.fatigue.max, "{vitals:?}");
    assert!(vitals.health.current > 0.0 && vitals.health.current <= vitals.health.max);
}

#[test]
fn an_attack_press_while_sheathed_draws_and_f_sheathes() {
    let (mut app, player) = armed();
    let attack = action(&mut app, PlayerAction::Button(ActionClass::Light));
    mock(&mut app, attack, true);
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Drawn, false));
    let light1 = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().rule_key("light1");
    assert_ne!(app.world().get::<ActionCharacter>(player).unwrap().state.mov, light1, "the drawing press doesn't attack");
    app.world_mut().entity_mut(attack).remove::<ActionMock>();
    let mut q = app.world_mut().query_filtered::<Entity, With<Action<ReadyWeaponAction>>>();
    let ready = q.single(app.world()).expect("one ready-weapon action");
    mock(&mut app, ready, true);
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Sheathed, true));
}

/// Scaffold `TrainingDummy` (sanity): spawned in the air it waits, then stands on the floor,
/// armed with the sheet; at 0 health it refills.
#[test]
fn a_training_dummy_lands_on_the_floor_armed_and_refills_when_down() {
    let (mut app, _player) = armed();
    dummy::systems::<Statics>(&mut app);
    let root = app.world_mut().spawn_empty().id();
    let feet = WorldPos::ORIGIN.offset(Vec3::new(0.0, 1.5, 7.0));
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (feet, 0.0), root);
    app.world_mut().flush();
    for _ in 0..30 {
        app.update();
    }
    assert!(!app.world().entity(d).contains::<AwaitingGround>(), "released");
    let body = app.world().get::<ActionBody>(d).unwrap();
    let y = WorldPos::ORIGIN.delta_f32(body.loco.position).y;
    assert!(y.is_finite() && y.abs() < 0.1 && body.loco.grounded.is_some(), "dummy feet at y {y}");
    let mut c = app.world_mut().get_mut::<Combatant>(d).expect("armed");
    assert!(c.fighter.hostile && (c.fighter.vitals.health.max - dummy::DUMMY_HEALTH).abs() < 1e-3);
    c.fighter.vitals.health.cur = 0.0;
    app.world_mut().write_message(sf_combat_bevy::CombatantDied { who: d });
    for _ in 0..3 {
        app.update();
    }
    let health = app.world().get::<Combatant>(d).unwrap().fighter.vitals.health;
    assert!((health.cur - health.max).abs() < 1e-3, "refilled: {health:?}");
}

/// CB-1 (owner 2026-10-27): the player walking into a standing training dummy stops against it,
/// capsules never overlapping, and doesn't shove it (sanity: actor-vs-actor blocking).
#[test]
fn the_player_stops_against_a_training_dummy_without_shoving_it() {
    let (mut app, player) = armed();
    dummy::systems::<Statics>(&mut app);
    let root = app.world_mut().spawn_empty().id();
    // The start spawn is at z 10 on the flat floor; the dummy stands 3 m ahead along -Z.
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, 7.0)), 0.0), root);
    app.world_mut().flush();
    for _ in 0..30 {
        app.update();
    }
    let feet = |app: &App, e: Entity| app.world().get::<ActionBody>(e).unwrap().loco.position;
    let flat = |v: Vec3| Vec2::new(v.x, v.z).length();
    let reach = app.world().get::<ActionBody>(player).unwrap().params.radius + app.world().get::<ActionBody>(d).unwrap().params.radius;
    let dummy_at = feet(&app, d);
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::NEG_Y);
    let mut closest = f32::INFINITY;
    for _ in 0..180 {
        app.update();
        closest = closest.min(flat(feet(&app, player).delta_f32(feet(&app, d))));
    }
    assert!(closest.is_finite() && closest >= reach - 0.01, "capsules overlap: {closest:.3} m apart, reach {reach:.3} m");
    assert!(closest < reach + 0.1, "the player reached the dummy: {closest:.3} m");
    let shoved = flat(dummy_at.delta_f32(feet(&app, d)));
    assert!(shoved < 0.01, "the dummy was shoved {shoved:.3} m");
}
