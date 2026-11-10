//! Combat glue (combat.md "C1b"): the player body is armed with the sword sheet, F draws and
//! sheathes the weapon (a sheathed attack press does nothing), and without stats it keeps its
//! own vitals; C1c: the lock key locks the dummy.

use sf_action_bevy::PlayerAction;
use sf_combat_bevy::Combatant;

use super::*;
use crate::combat::{ReadyWeaponAction, Stance};
use crate::tests::{Statics, action, app_setup, hold_key, mock};

fn armed() -> (App, Entity) {
    let sheet = sf_combat::load(mw_combat::SWORD_1H, &|_| Some(0.5)).expect("sheet compiles");
    let (mut app, player) = app_setup(60, &mw_controls_bevy::keymap(), |app| {
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
fn the_player_is_armed_sheathed_with_unsheeted_vitals_without_stats() {
    let (mut app, player) = armed();
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Sheathed, true));
    let key = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().key;
    assert_eq!(app.world().get::<ActionCharacter>(player).unwrap().state.mode.moveset, Some(key));
    let vitals = app.world().get::<Combatant>(player).unwrap().fighter.vitals;
    let stamina = mw_combat::stamina_from_fatigue(combat::UNSHEETED_VITALS[1]);
    assert!((vitals.stamina.max - stamina).abs() < 1e-3 && vitals.stamina.cur <= vitals.stamina.max, "{vitals:?}");
    assert!(vitals.health.cur > 0.0 && vitals.health.cur <= vitals.health.max);
    assert!(!app.world().entity(player).contains::<vitals::CombatSheet>(), "no stats, no sheet link");
}

/// CB-3 (owner 2026-11-02: attack only with the weapon drawn): sheathed, light and heavy presses
/// start nothing and don't draw; the F key (the real binding) draws, the avatar's readiness
/// follows (its held weapon mounts on the weapon bone), and F again sheathes.
#[test]
fn sheathed_attack_presses_do_nothing_and_the_f_key_draws_and_sheathes() {
    use sf_character::Readiness;
    use sf_character_bevy::CharacterReadiness;
    let (mut app, player) = armed();
    // The game's rules with the sheet's moves (the harness runs the base rules only).
    let sheet = app.world().resource::<combat::PlayerCombat>().sheet.clone();
    app.insert_resource(rules(&[], sheet.as_ref()));
    let avatar = app.world_mut().spawn((mw_game_core::player::PlayerAvatar, ChildOf(player))).id();
    // Landed and standing.
    for _ in 0..30 {
        app.update();
    }
    let shown = |app: &App| app.world().get::<CharacterReadiness>(avatar).map(|r| r.0);
    assert_eq!(shown(&app), Some(Readiness::Sheathed));
    let attacks: Vec<_> = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().moves.moves.keys().copied().collect();
    assert!(!attacks.is_empty());
    for class in [ActionClass::Light, ActionClass::Heavy] {
        let attack = action(&mut app, PlayerAction::Button(class));
        mock(&mut app, attack, true);
        for _ in 0..6 {
            app.update();
            let mov = app.world().get::<ActionCharacter>(player).unwrap().state.mov;
            assert!(!attacks.contains(&mov), "{class:?} started a sheet move sheathed");
        }
        app.world_mut().entity_mut(attack).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
        app.update();
        assert_eq!(stance(&app, player), (Stance::Sheathed, true), "{class:?} drew the weapon");
    }
    let mut q = app.world_mut().query_filtered::<Entity, With<Action<ReadyWeaponAction>>>();
    assert!(q.single(app.world()).is_ok(), "one ready-weapon action");
    let tap_f = |app: &mut App| {
        hold_key(app, KeyCode::KeyF, true);
        for _ in 0..2 {
            app.update();
        }
        hold_key(app, KeyCode::KeyF, false);
        for _ in 0..2 {
            app.update();
        }
    };
    tap_f(&mut app);
    assert_eq!(stance(&app, player), (Stance::Drawn, false));
    assert_eq!(shown(&app), Some(Readiness::Drawn));
    // Drawn, the same press attacks (the sheathed check above isn't vacuous).
    let attack = action(&mut app, PlayerAction::Button(ActionClass::Light));
    mock(&mut app, attack, true);
    let light1 = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().rule_key("light1");
    let mut seen = Vec::new();
    for _ in 0..6 {
        app.update();
        seen.push(app.world().get::<ActionCharacter>(player).unwrap().state.mov);
    }
    app.world_mut().entity_mut(attack).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
    assert!(seen.contains(&light1), "a drawn light press attacks: {seen:?}, light1 {light1:?}, sheet moves {attacks:?}");
    for _ in 0..120 {
        app.update();
    }
    tap_f(&mut app);
    assert_eq!(stance(&app, player), (Stance::Sheathed, true));
    assert_eq!(shown(&app), Some(Readiness::Sheathed));
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

/// C1c: the lock key locks the training dummy ahead (the camera's `LockOn` mirrors it) and a
/// second press releases it.
#[test]
fn the_lock_key_locks_the_training_dummy_and_a_second_press_releases_it() {
    let (mut app, player) = armed();
    dummy::systems::<Statics>(&mut app);
    let root = app.world_mut().spawn_empty().id();
    // The start spawn is at z 10 looking down +Z (no camera: view yaw 0); the dummy 3 m ahead.
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, 13.0)), 0.0), root);
    app.world_mut().flush();
    for _ in 0..30 {
        app.update();
    }
    let mut q = app.world_mut().query_filtered::<Entity, With<Action<sf_combat_bevy::LockOnAction>>>();
    let lock = q.single(app.world()).expect("one lock action");
    let press = |app: &mut App| {
        mock(app, lock, true);
        for _ in 0..3 {
            app.update();
        }
        app.world_mut().entity_mut(lock).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
        for _ in 0..3 {
            app.update();
        }
    };
    let target = sf_action::TargetId(app.world().get::<ActionCharacter>(d).unwrap().id.0);
    let locked = |app: &App| app.world().get::<ActionCharacter>(player).unwrap().state.lock;
    assert_eq!(locked(&app), None);
    press(&mut app);
    assert_eq!(locked(&app), Some(target));
    assert_eq!(app.world().get::<sf_camera_bevy::LockOn>(player), Some(&sf_camera_bevy::LockOn(d)));
    press(&mut app);
    assert_eq!(locked(&app), None);
    assert!(app.world().get::<sf_camera_bevy::LockOn>(player).is_none());
}

/// C1c part 2a: locked on, a sideways stick circles the training dummy while the body keeps
/// facing it (strafe locomotion; the strafe clips are C1c-2b).
#[test]
fn locked_on_a_sideways_stick_strafes_round_the_dummy_facing_it() {
    let (mut app, player) = armed();
    // The game player's mover (rate-capped turns), not the harness's generic springs.
    app.world_mut().get_mut::<ActionBody>(player).unwrap().params = feel::mover_params();
    dummy::systems::<Statics>(&mut app);
    let root = app.world_mut().spawn_empty().id();
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.0, 13.0)), 0.0), root);
    app.world_mut().flush();
    for _ in 0..30 {
        app.update();
    }
    let mut q = app.world_mut().query_filtered::<Entity, With<Action<sf_combat_bevy::LockOnAction>>>();
    let lock = q.single(app.world()).expect("one lock action");
    mock(&mut app, lock, true);
    for _ in 0..3 {
        app.update();
    }
    app.world_mut().entity_mut(lock).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
    app.update();
    assert!(app.world().get::<ActionCharacter>(player).unwrap().state.lock.is_some(), "locked");
    let feet = |app: &App, e: Entity| app.world().get::<ActionBody>(e).unwrap().loco.position;
    let start = feet(&app, player);
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::X);
    for _ in 0..60 {
        app.update();
    }
    assert!(app.world().get::<ActionCharacter>(player).unwrap().state.lock.is_some(), "still locked");
    let to = feet(&app, player).delta_f32(feet(&app, d));
    let bearing = to.x.atan2(to.z);
    let yaw = app.world().get::<ActionBody>(player).unwrap().loco.motion.yaw;
    assert!(sf_controller::wrap_angle(yaw - bearing).abs() < 0.15, "faces the dummy: yaw {yaw:.3}, bearing {bearing:.3}");
    let moved = start.delta_f32(feet(&app, player));
    let r = Vec2::new(to.x, to.z).length();
    assert!(moved.x.abs() > 1.0 && (2.0..4.5).contains(&r), "strafed round it: moved {moved}, {r:.2} m away");
}

/// A cooked draw or sheathe clip of `length` seconds with the point event `event` at `key`.
fn stance_clip(slot: &str, length: f32, event: &str, key: f32) -> SlotClip {
    use sf_anim::{ClipRules, CookSettings, Event, cook_clip, fixtures};
    let skeleton = fixtures::biped();
    let mut src = fixtures::gait(&skeleton, slot, length, 0.0, 0.1, 0.6, &[("thigh_l", "foot_l", 0.0)]);
    src.events = vec![Event::point(key, event)];
    let clip = cook_clip(&skeleton, &src, &ClipRules::default(), &CookSettings::default()).expect("cooks");
    SlotClip { slot: slot.into(), clip: Arc::new(clip) }
}

/// CB-3 draw and sheathe clips: F starts the draw clip and the avatar's readiness (the weapon
/// prop's mount) flips to Drawn at the clip's attach key, not at the press; attacks stay refused
/// until the draw has played out and are accepted after. F again sheathes: attacks are refused
/// from its first tick and the prop leaves the hand at the detach key. Times from the baked keys.
#[test]
fn the_draw_and_sheathe_clips_move_the_weapon_at_their_keys_and_gate_the_attacks() {
    use sf_character::Readiness;
    use sf_character_bevy::CharacterReadiness;
    let (draw_key, sheathe_key, length) = (0.25, 0.375, 0.5);
    let clips = PlayerClips {
        slots: vec![
            stance_clip(mw_combat::SWORD_1H_DRAW, length, mw_anim::events::EQUIP_ATTACH, draw_key),
            stance_clip(mw_combat::SWORD_1H_SHEATHE, length, mw_anim::events::UNEQUIP_DETACH, sheathe_key),
        ],
        skeleton: None,
    };
    let sheet = sf_combat::load(mw_combat::SWORD_1H, &|_| Some(0.5)).expect("sheet compiles");
    let (mut app, player) = app_setup(60, &mw_controls_bevy::keymap(), |app| {
        combat::add(app, Some(sheet.clone()), &clips);
    });
    app.world_mut().entity_mut(player).insert(PlayerBody);
    app.insert_resource(rules(&[], Some(&sheet)));
    let timing = *app.world().resource::<combat::StanceClips>();
    assert!((timing.draw.key - draw_key).abs() < 1e-4 && (timing.sheathe.key - sheathe_key).abs() < 1e-4, "{timing:?}");
    assert!(timing.draw.length > draw_key && timing.sheathe.length > sheathe_key, "{timing:?}");
    let avatar = app.world_mut().spawn((mw_game_core::player::PlayerAvatar, ChildOf(player))).id();
    for _ in 0..30 {
        app.update();
    }
    let shown = |app: &App| app.world().get::<CharacterReadiness>(avatar).map(|r| r.0);
    let light1 = sheet.rule_key("light1");
    let mov = |app: &App| app.world().get::<ActionCharacter>(player).unwrap().state.mov;
    assert_eq!(stance(&app, player), (Stance::Sheathed, true));

    // Press F, then attack while the draw plays: (elapsed, shown readiness, withheld) per frame.
    hold_key(&mut app, KeyCode::KeyF, true);
    app.update();
    app.update();
    hold_key(&mut app, KeyCode::KeyF, false);
    let attack = action(&mut app, PlayerAction::Button(ActionClass::Light));
    let mut drawing = Vec::new();
    for _ in 0..120 {
        if drawing.len() == 2 {
            mock(&mut app, attack, true);
        }
        app.update();
        let (s, withheld) = stance(&app, player);
        assert_ne!(mov(&app), light1, "attacked during the draw: {s:?}");
        let Stance::Changing(c) = s else { break };
        assert!(withheld && c.to == Readiness::Drawn);
        drawing.push((c.elapsed, shown(&app)));
    }
    assert_eq!(stance(&app, player), (Stance::Drawn, false), "the draw ended");
    assert!(drawing.len() > 4, "the draw played over several frames: {drawing:?}");
    // The prop moved exactly when the elapsed time crossed the attach key.
    for &(elapsed, readiness) in &drawing {
        let want = if elapsed >= timing.draw.key { Readiness::Drawn } else { Readiness::Sheathed };
        assert_eq!(readiness, Some(want), "at {elapsed:.3} s: {drawing:?}");
    }
    assert!(drawing.iter().any(|d| d.1 == Some(Readiness::Sheathed)) && drawing.iter().any(|d| d.1 == Some(Readiness::Drawn)), "{drawing:?}");
    assert_eq!(shown(&app), Some(Readiness::Drawn));
    // Drawn: a fresh press attacks.
    app.world_mut().entity_mut(attack).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
    app.update();
    mock(&mut app, attack, true);
    let mut seen = Vec::new();
    for _ in 0..6 {
        app.update();
        seen.push(mov(&app));
    }
    app.world_mut().entity_mut(attack).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
    assert!(seen.contains(&light1), "a drawn light press attacks: {seen:?}");
    for _ in 0..120 {
        app.update();
    }

    // F again: the sheathe withholds the attacks from its first frame; the prop leaves at the key.
    hold_key(&mut app, KeyCode::KeyF, true);
    app.update();
    app.update();
    hold_key(&mut app, KeyCode::KeyF, false);
    let mut sheathing = Vec::new();
    for _ in 0..120 {
        let (s, withheld) = stance(&app, player);
        let Stance::Changing(c) = s else {
            if sheathing.is_empty() {
                app.update();
                continue;
            }
            break;
        };
        assert!(withheld && c.to == Readiness::Sheathed, "attacks withheld while sheathing");
        sheathing.push((c.elapsed, shown(&app)));
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Sheathed, true), "the sheathe ended");
    assert!(sheathing.len() > 4, "{sheathing:?}");
    for &(elapsed, readiness) in &sheathing {
        let want = if elapsed >= timing.sheathe.key { Readiness::Sheathed } else { Readiness::Drawn };
        assert_eq!(readiness, Some(want), "at {elapsed:.3} s: {sheathing:?}");
    }
    assert_eq!(shown(&app), Some(Readiness::Sheathed));
}

/// First-person combat (combat.md C4 soft aim, camera.md "First person"): the player is armed and
/// drawn with the game's rules, the training dummy stands `dist` m off at `deg` degrees from the
/// view (yaw 0, +Z), and the camera following the player is in `view`. Settled.
fn first_person_arena(dist: f32, deg: f32, view: sf_camera::ViewChoice) -> (App, Entity, Entity, Entity) {
    let (mut app, player) = armed();
    let sheet = app.world().resource::<combat::PlayerCombat>().sheet.clone();
    app.insert_resource(rules(&[], sheet.as_ref()));
    dummy::systems::<Statics>(&mut app);
    for _ in 0..30 {
        app.update();
    }
    let feet = app.world().get::<ActionBody>(player).unwrap().loco.position;
    let (s, c) = deg.to_radians().sin_cos();
    let root = app.world_mut().spawn_empty().id();
    let d = dummy::spawn(&mut app.world_mut().commands(), 0, (feet.offset(Vec3::new(dist * s, 0.0, dist * c)), std::f32::consts::PI), root);
    let mut cam = PlayerCamera::new(player);
    cam.set_view(view);
    let cam = app.world_mut().spawn(cam).id();
    app.world_mut().flush();
    hold_key(&mut app, KeyCode::KeyF, true);
    app.update();
    hold_key(&mut app, KeyCode::KeyF, false);
    for _ in 0..30 {
        app.update();
    }
    assert_eq!(stance(&app, player), (Stance::Drawn, false));
    (app, player, d, cam)
}

fn press_lock(app: &mut App) {
    let mut q = app.world_mut().query_filtered::<Entity, With<Action<sf_combat_bevy::LockOnAction>>>();
    let lock = q.single(app.world()).expect("one lock action");
    mock(app, lock, true);
    for _ in 0..3 {
        app.update();
    }
    app.world_mut().entity_mut(lock).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
    for _ in 0..3 {
        app.update();
    }
}

/// First person: a light attack with the dummy 20 deg off the view and 2.5 m away turns the body
/// onto it inside the attack's `Turn` window (light1: 0-0.11 s at 360 deg/s), and not after:
/// once the window has closed, neither the view swinging 60 deg away nor the dummy stepping to
/// 40 deg moves the body (no dragging). The lock key locks nothing in first person.
#[test]
fn first_person_light_attack_soft_aims_at_the_dummy_in_its_turn_window_only() {
    let (mut app, player, d, _) = first_person_arena(2.5, 20.0, sf_camera::ViewChoice::FirstPerson);
    press_lock(&mut app);
    assert_eq!(app.world().get::<ActionCharacter>(player).unwrap().state.lock, None, "no hard lock in first person");
    assert!(app.world().get::<sf_camera_bevy::LockOn>(player).is_none());
    let feet = |app: &App, e: Entity| app.world().get::<ActionBody>(e).unwrap().loco.position;
    let to = feet(&app, player).delta_f32(feet(&app, d));
    let bearing = to.x.atan2(to.z);
    assert!((bearing.to_degrees() - 20.0).abs() < 1.0, "the dummy stands 20 deg off the view: {}", bearing.to_degrees());
    let yaw = |app: &App| app.world().get::<ActionBody>(player).unwrap().loco.motion.yaw;
    assert!(yaw(&app).abs() < 0.02, "the body faces the view before the attack: {}", yaw(&app));

    let light1 = app.world().resource::<combat::PlayerCombat>().sheet.as_ref().unwrap().rule_key("light1");
    let attack = action(&mut app, PlayerAction::Button(ActionClass::Light));
    mock(&mut app, attack, true);
    let state = |app: &App| app.world().get::<ActionCharacter>(player).unwrap().state.clone();
    let (mut in_window, mut after) = (Vec::new(), Vec::new());
    for _ in 0..60 {
        app.update();
        let s = state(&app);
        if s.mov != light1 {
            if in_window.is_empty() {
                continue;
            }
            break;
        }
        app.world_mut().entity_mut(attack).insert(ActionMock::new(TriggerState::None, false, MockSpan::Manual));
        let t = s.clock.elapsed.secs();
        if t <= 0.12 {
            in_window.push((t, yaw(&app)));
        } else {
            if after.is_empty() {
                // The window has closed: the player looks 60 deg away and the dummy steps to 40 deg.
                app.world_mut().get_mut::<PlayerInput>(player).unwrap().sampler.set_view(sf_action::ViewDir { yaw: 60f32.to_radians(), pitch: 0.0 });
                let (s, c) = 40f32.to_radians().sin_cos();
                let at = feet(&app, player).offset(Vec3::new(2.5 * s, 0.0, 2.5 * c));
                app.world_mut().get_mut::<ActionBody>(d).unwrap().loco.position = at;
            }
            after.push((t, yaw(&app)));
        }
    }
    assert!(!in_window.is_empty() && after.len() > 5, "light1 played: {in_window:?} then {after:?}");
    let target = app.world().get::<Combatant>(player).unwrap().fight.swing.as_ref().and_then(|s| s.aim);
    assert_eq!(target, Some(sf_action::TargetId(app.world().get::<ActionCharacter>(d).unwrap().id.0)), "the dummy is the swing's soft target");
    let turned = in_window.last().unwrap().1;
    assert!((turned - bearing).abs() < 0.02, "on the dummy by the window's end: {turned:.3} vs {bearing:.3} ({in_window:?})");
    for &(t, y) in &after {
        assert!((y - turned).abs() < 1e-3, "turned after the window at {t:.3} s: {y:.3} vs {turned:.3}");
    }
}

/// A lock taken in third person breaks when the camera goes to first person (no hard lock there).
#[test]
fn switching_to_first_person_breaks_a_held_lock() {
    let (mut app, player, _, cam) = first_person_arena(3.0, 0.0, sf_camera::ViewChoice::ThirdPerson);
    press_lock(&mut app);
    assert!(app.world().get::<ActionCharacter>(player).unwrap().state.lock.is_some(), "locked in third person");
    app.world_mut().get_mut::<PlayerCamera>(cam).unwrap().set_view(sf_camera::ViewChoice::FirstPerson);
    for _ in 0..3 {
        app.update();
    }
    assert_eq!(app.world().get::<ActionCharacter>(player).unwrap().state.lock, None);
    assert_eq!(app.world().get::<sf_combat_bevy::LockOwner>(player).unwrap().last, Some(sf_combat::LockEvent::Broke(sf_camera::BreakReason::View)));
    assert!(app.world().get::<sf_camera_bevy::LockOn>(player).is_none());
}
