//! `settings`: the Fight rating, hand-computed from `OpenMW`'s formula with vanilla GMSTs
//! (`iFightDistanceBase` 20, `fFightDistanceMultiplier` 0.005, `fFightDispMult` 0.2, `iWerewolfFightMod` 100).

use glam::Vec3;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{GameSetting, GameSettingValue};

use crate::settings::{
    EngageGates, FightGmst, FightInput, aggro_distance_units, engages_on_sight, fight_rating,
    is_aggressive,
};

/// An NPC observer of an NPC player, capable, not in combat.
fn npc(fight: i32, disposition: i32, distance_units: f32) -> FightInput {
    FightInput {
        fight,
        observer_npc: true,
        disposition,
        distance_units,
        in_combat_with_target: false,
        aggression_capable: true,
        target_npc: true,
        target_werewolf: false,
        target_is_player: true,
        pc_known_werewolf: false,
    }
}

fn rate(input: &FightInput) -> (i32, bool) {
    let g = FightGmst::default();
    (fight_rating(input, &g), is_aggressive(input, &g))
}

#[test]
fn fight_30_disposition_50_near() {
    // 30 + (20 - 0) + 0
    assert_eq!(rate(&npc(30, 50, 0.0)), (50, false));
    // 30 + (20 - 0.005 * 200) = 30 + 19
    assert_eq!(rate(&npc(30, 50, 200.0)), (49, false));
}

#[test]
fn fight_90_near_and_far() {
    assert_eq!(rate(&npc(90, 50, 0.0)), (110, true));
    // 20 - 0.005 * 2000 = 10: exactly at the threshold.
    assert_eq!(rate(&npc(90, 50, 2000.0)), (100, true));
    // 20 - 10.5 = 9.5, truncated to 9.
    assert_eq!(rate(&npc(90, 50, 2100.0)), (99, false));
    assert_eq!(rate(&npc(90, 50, 2200.0)), (99, false));
    // 20 - 20 = 0.
    assert_eq!(rate(&npc(90, 50, 4000.0)), (90, false));
}

#[test]
fn fight_0() {
    assert_eq!(rate(&npc(0, 50, 0.0)), (20, false));
    // (50 - 0) * 0.2 = 10 more for a hated target.
    assert_eq!(rate(&npc(0, 0, 0.0)), (30, false));
}

#[test]
fn disposition_100_lowers_it() {
    assert_eq!(rate(&npc(85, 50, 0.0)), (105, true));
    // (50 - 100) * 0.2 = -10
    assert_eq!(rate(&npc(85, 100, 0.0)), (95, false));
    assert_eq!(rate(&npc(50, 100, 4000.0)), (40, false));
}

#[test]
fn distance_bias_truncates_toward_zero() {
    // 20 - 0.005 * 4100 = -0.5 truncates to 0, not -1: Fight 100 stays aggressive.
    assert_eq!(rate(&npc(100, 50, 4100.0)), (100, true));
    // 20 - 0.005 * 4300 = -1.5 truncates to -1.
    assert_eq!(rate(&npc(100, 50, 4300.0)), (99, false));
}

#[test]
fn creatures_count_disposition_as_50() {
    let mut c = npc(85, 100, 0.0);
    c.observer_npc = false;
    assert_eq!(rate(&c), (105, true));
}

#[test]
fn werewolf_targets_raise_npc_aggression() {
    let mut i = npc(30, 50, 0.0);
    i.target_werewolf = true;
    // The bonus is in isAggressive, not in the rating.
    assert_eq!(rate(&i), (50, true));
    i.target_werewolf = false;
    i.pc_known_werewolf = true;
    assert_eq!(rate(&i), (50, true));
    // Known werewolf only counts for the player.
    i.target_is_player = false;
    assert_eq!(rate(&i), (50, false));
    // Creatures get no bonus, nor do NPCs eyeing a creature.
    let mut c = npc(30, 50, 0.0);
    c.target_werewolf = true;
    c.observer_npc = false;
    assert_eq!(rate(&c), (50, false));
    c.observer_npc = true;
    c.target_npc = false;
    assert_eq!(rate(&c), (50, false));
}

#[test]
fn combat_and_capability_short_circuit() {
    let mut i = npc(0, 100, 10_000.0);
    i.in_combat_with_target = true;
    assert!(rate(&i).1);
    let mut i = npc(100, 0, 0.0);
    i.aggression_capable = false;
    assert_eq!(rate(&i), (130, false));
}

#[test]
fn engages_on_sight_gates() {
    let g = FightGmst::default();
    let hostile = npc(90, 50, 0.0);
    let open = EngageGates {
        either_dead: false,
        target_hidden: false,
        can_fight: true,
        target_player_side: true,
        observer_player_side: false,
        sees_target: true,
    };
    assert!(engages_on_sight(&hostile, &open, &g));
    assert!(!engages_on_sight(&npc(30, 50, 0.0), &open, &g));
    for gates in [
        EngageGates { sees_target: false, ..open },
        EngageGates { observer_player_side: true, ..open },
        EngageGates { target_player_side: false, ..open },
        EngageGates { target_hidden: true, ..open },
        EngageGates { can_fight: false, ..open },
        EngageGates { either_dead: true, ..open },
    ] {
        assert!(!engages_on_sight(&hostile, &gates, &g), "{gates:?}");
    }
    // Already fighting: engageCombat returns before starting anything.
    let mut fighting = hostile;
    fighting.in_combat_with_target = true;
    assert!(!engages_on_sight(&fighting, &open, &g));
}

#[test]
fn aggro_distance_ignores_height_for_swimmers_and_flyers() {
    let unit = 0.9144_f32 / 64.0;
    let (a, b) = (Vec3::ZERO, Vec3::new(300.0 * unit, 400.0 * unit, 0.0));
    assert!((aggro_distance_units(a, b, false) - 500.0).abs() < 1e-2);
    assert!((aggro_distance_units(a, b, true) - 300.0).abs() < 1e-2);
}

fn gmst(db: &mut RecordDb, src: sf_records::SourceId, id: &str, value: GameSettingValue) {
    db.define(src, RecordId::new(id), GameSetting { id: id.into(), value, ..Default::default() })
        .unwrap();
}

#[test]
fn gmsts_read_from_records() {
    let mut db = RecordDb::new();
    db.register::<GameSetting>("GMST").unwrap();
    let src = db.add_source("test", &[]);
    assert_eq!(FightGmst::from_db(&db), FightGmst::default());

    // The vanilla records give the vanilla values.
    gmst(&mut db, src, "iFightDistanceBase", GameSettingValue::Integer(20));
    gmst(&mut db, src, "fFightDistanceMultiplier", GameSettingValue::Float(0.005));
    gmst(&mut db, src, "fFightDispMult", GameSettingValue::Float(0.2));
    gmst(&mut db, src, "iWerewolfFightMod", GameSettingValue::Integer(100));
    assert_eq!(FightGmst::from_db(&db), FightGmst::default());

    // A mod's values win and feed the rating: 30 + (40 - 0.01 * 1000) + (50 - 0) * 0.5 = 85.
    let m = db.add_source("mod", &[src]);
    gmst(&mut db, m, "iFightDistanceBase", GameSettingValue::Integer(40));
    gmst(&mut db, m, "fFightDistanceMultiplier", GameSettingValue::Float(0.01));
    gmst(&mut db, m, "fFightDispMult", GameSettingValue::Float(0.5));
    gmst(&mut db, m, "iWerewolfFightMod", GameSettingValue::Float(15.9));
    let g = FightGmst::from_db(&db);
    assert_eq!(g.werewolf_mod, 15);
    assert_eq!(fight_rating(&npc(30, 0, 1000.0), &g), 85);
}

// Flee (`MA4f`): hand-computed from `OpenMW`'s formulas with vanilla GMSTs (`fAIFleeHealthMult` 7,
// `fAIFleeFleeMult` 0.3, `iWereWolfLevelToAttack` 20, `iWereWolfFleeMod` 100, `fFleeDistance` 3000, and
// the Fight distance bias `20 - 0.005 * d`).

use crate::settings::{
    FLEE_ACTION_COOLDOWN_S, FleeDecision, FleeGmst, FleeInput, FleeMode, FleeRunInput, FleeState,
    FleeStepInput, HeldAction, flee_rating, flee_run_step, flee_step, health_ratio, wants_to_flee,
};

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

/// An NPC actor at `health` of 100 facing an NPC enemy at `distance_units`.
fn fleer(flee: i32, health: f32, distance_units: f32) -> FleeInput {
    FleeInput {
        flee,
        health_current: health,
        health_max: 100.0,
        level: 10,
        observer_npc: true,
        enemy_npc: true,
        enemy_werewolf: false,
        distance_units,
    }
}

fn flee_of(input: &FleeInput) -> f32 {
    flee_rating(input, &FleeGmst::default())
}

#[test]
fn flee_rating_full_health_low_flee() {
    // Full health and Flee 0: a rating of exactly 0 gets no distance bias.
    assert!(close(flee_of(&fleer(0, 100.0, 0.0)), 0.0));
    // 0 * 7 + 30 * 0.3 + 20 = 29.
    assert!(close(flee_of(&fleer(30, 100.0, 0.0)), 29.0));
    assert!(!wants_to_flee(flee_of(&fleer(30, 100.0, 0.0)), 0.0, false));
}

#[test]
fn flee_rating_low_health() {
    // (1 - 0) * 7 + 50 * 0.3 + 20 = 42; quarter health: 0.75 * 7 + 15 + 20 = 40.25.
    assert!(close(flee_of(&fleer(50, 0.0, 0.0)), 42.0));
    assert!(close(flee_of(&fleer(50, 25.0, 0.0)), 40.25));
    // The best below Flee 100: 7 + 99 * 0.3 + 20 = 56.7, still under the 100 that counts.
    let r = flee_of(&fleer(99, 0.0, 0.0));
    assert!(close(r, 56.7));
    assert!(!wants_to_flee(r, 0.0, false));
    // Far away the bias goes negative: 7 + 15 + (20 - 0.005 * 6000) = 12.
    assert!(close(flee_of(&fleer(50, 0.0, 6000.0)), 12.0));
    // No maximum Health: the ratio is 1, so 0 * 7 + 15 + 20 = 35.
    assert!(close(health_ratio(5.0, 0.0), 1.0));
    let mut empty = fleer(50, 5.0, 0.0);
    empty.health_max = 0.0;
    assert!(close(flee_of(&empty), 35.0));
}

#[test]
fn flee_100_is_the_rating() {
    // Flee 100 or more returns the stat, with no distance bias.
    assert!(close(flee_of(&fleer(100, 100.0, 4000.0)), 100.0));
    assert!(close(flee_of(&fleer(120, 0.0, 0.0)), 120.0));
    // It must beat the best action's anti-flee rating.
    assert!(wants_to_flee(100.0, 0.0, false));
    assert!(wants_to_flee(100.0, 99.5, false));
    assert!(!wants_to_flee(100.0, 100.0, false));
    assert!(!wants_to_flee(100.0, f32::MAX, false));
}

#[test]
fn flee_with_fists_and_a_summon() {
    assert!(wants_to_flee(0.0, 0.0, true));
    assert!(!wants_to_flee(0.0, 5.0, true));
    assert!(!wants_to_flee(0.0, 0.0, false));
}

#[test]
fn flee_from_a_werewolf() {
    let w = FleeInput { enemy_werewolf: true, ..fleer(0, 100.0, 0.0) };
    // iWereWolfFleeMod 100 + 20; at 2000 units 100 + 10; at 6000 units 100 - 10 = 90, too low.
    assert!(close(flee_of(&w), 120.0));
    assert!(close(flee_of(&FleeInput { distance_units: 2000.0, ..w }), 110.0));
    let far = flee_of(&FleeInput { distance_units: 6000.0, ..w });
    assert!(close(far, 90.0));
    assert!(!wants_to_flee(far, 0.0, false));
    // Level 20 stands its ground; so does a creature.
    assert!(close(flee_of(&FleeInput { level: 20, ..w }), 0.0));
    assert!(close(flee_of(&FleeInput { level: 19, ..w }), 120.0));
    assert!(close(flee_of(&FleeInput { observer_npc: false, ..w }), 0.0));
}

fn run(los: bool, distance_minus_extents: f32, distance_units: f32) -> FleeRunInput {
    FleeRunInput {
        los,
        enemy_reach: 100.0,
        distance_minus_extents,
        distance_units,
        has_destination: false,
        destination_reached: false,
    }
}

#[test]
fn flee_run_idle_waits_for_the_enemy() {
    let g = FleeGmst::default();
    let step = |input: &FleeRunInput| flee_run_step(FleeMode::Idle, input, &g, 0.25);
    assert_eq!(step(&run(true, 150.0, 200.0)), FleeMode::Idle);
    assert_eq!(step(&run(false, 50.0, 100.0)), FleeMode::Idle);
    // Within reach exactly: run blindly, or to a pathgrid point when there is one.
    assert_eq!(step(&run(true, 100.0, 150.0)), FleeMode::RunBlindly { timer: 0.0 });
    let with_point = FleeRunInput { has_destination: true, ..run(true, 100.0, 150.0) };
    assert_eq!(step(&with_point), FleeMode::RunToDestination);
    // A reach of 1000 or more triggers at any distance.
    let ranged = FleeRunInput { enemy_reach: 1000.0, ..run(true, 5000.0, 5000.0) };
    assert_eq!(step(&ranged), FleeMode::RunBlindly { timer: 0.0 });
    assert_eq!(step(&FleeRunInput { enemy_reach: 999.0, ..ranged }), FleeMode::Idle);
}

#[test]
fn flee_run_blindly_for_one_second() {
    let g = FleeGmst::default();
    let input = run(true, 0.0, 0.0);
    let mut mode = FleeMode::RunBlindly { timer: 0.0 };
    for expected in [0.25, 0.5, 0.75, 1.0] {
        mode = flee_run_step(mode, &input, &g, 0.25);
        assert_eq!(mode, FleeMode::RunBlindly { timer: expected });
    }
    assert_eq!(flee_run_step(mode, &input, &g, 0.25), FleeMode::Idle);
    // The timer is tested before it grows: 0.875 still runs (to 1.125), then stops.
    let mode = flee_run_step(FleeMode::RunBlindly { timer: 0.875 }, &input, &g, 0.25);
    assert_eq!(mode, FleeMode::RunBlindly { timer: 1.125 });
    assert_eq!(flee_run_step(mode, &input, &g, 0.25), FleeMode::Idle);
}

#[test]
fn flee_run_to_destination_stops() {
    let g = FleeGmst::default();
    let step = |input: &FleeRunInput| flee_run_step(FleeMode::RunToDestination, input, &g, 0.25);
    // Past fFleeDistance and out of sight.
    assert_eq!(step(&run(false, 2950.0, 3001.0)), FleeMode::Idle);
    assert_eq!(step(&run(false, 2950.0, 3000.0)), FleeMode::RunToDestination);
    assert_eq!(step(&run(true, 2950.0, 3001.0)), FleeMode::RunToDestination);
    // Arrived.
    let arrived = FleeRunInput { destination_reached: true, ..run(true, 0.0, 10.0) };
    assert_eq!(step(&arrived), FleeMode::Idle);
    assert_eq!(flee_run_step(FleeMode::None, &run(true, 0.0, 0.0), &g, 0.25), FleeMode::None);
}

/// A mobile actor reconsidering this tick, armed (anti-flee 50, cooldown 1 s), enemy out of sight.
fn tick(flee: i32, health: f32) -> FleeStepInput {
    FleeStepInput {
        mobile: true,
        incapacitated: false,
        reaction_due: true,
        can_fight: true,
        force_flee_allowed: false,
        ready_to_prepare: true,
        werewolf: false,
        rating: fleer(flee, health, 0.0),
        best_action: HeldAction::Attack,
        anti_flee_rating: 50.0,
        best_action_cooldown: 1.0,
        has_summons: false,
        path_failed: false,
        run: run(false, 500.0, 600.0),
    }
}

fn state(mode: FleeMode, action: HeldAction, action_cooldown: f32) -> FleeState {
    FleeState { mode, action, action_cooldown }
}

#[test]
fn flee_step_healthy_fights() {
    let g = FleeGmst::default();
    let mut s = FleeState::default();
    assert_eq!(flee_step(&mut s, &tick(0, 100.0), &g, 0.25), FleeDecision::NotFleeing);
    assert_eq!(s, state(FleeMode::None, HeldAction::Attack, 1.0));
    // A wounded Flee-99 actor still fights: 56.7 counts as 0.
    let mut s = FleeState::default();
    assert_eq!(flee_step(&mut s, &tick(99, 0.0), &g, 0.25), FleeDecision::NotFleeing);
    // So does a werewolf with Flee 100.
    let mut s = FleeState::default();
    let wolf = FleeStepInput { werewolf: true, ..tick(100, 0.0) };
    assert_eq!(flee_step(&mut s, &wolf, &g, 0.25), FleeDecision::NotFleeing);
}

#[test]
fn flee_step_starts_keeps_and_stops() {
    let g = FleeGmst::default();
    let mut s = FleeState::default();
    // Flee 100 beats anti-flee 50.
    assert_eq!(flee_step(&mut s, &tick(100, 10.0), &g, 1.0), FleeDecision::Start);
    assert_eq!(s, state(FleeMode::Idle, HeldAction::Flee, FLEE_ACTION_COOLDOWN_S));
    // Calmed (Flee 0) but the 3 s cooldown holds: still fleeing with 2 s and 1 s left.
    let calm = tick(0, 100.0);
    assert_eq!(flee_step(&mut s, &calm, &g, 1.0), FleeDecision::Keep);
    assert_eq!(flee_step(&mut s, &calm, &g, 1.0), FleeDecision::Keep);
    assert!(close(s.action_cooldown, 1.0));
    // At 0 s it reconsiders and fights again.
    assert_eq!(flee_step(&mut s, &calm, &g, 1.0), FleeDecision::Stop);
    assert_eq!(s, state(FleeMode::None, HeldAction::Attack, 1.0));
    // Still Flee 100 at reconsideration: keeps fleeing, the cooldown back to 3 s.
    let mut s = state(FleeMode::RunToDestination, HeldAction::Flee, 0.25);
    let scared = FleeStepInput { run: run(true, 50.0, 100.0), ..tick(100, 10.0) };
    assert_eq!(flee_step(&mut s, &scared, &g, 0.25), FleeDecision::Keep);
    assert_eq!(s, state(FleeMode::RunToDestination, HeldAction::Flee, 3.0));
}

#[test]
fn flee_step_gates() {
    let g = FleeGmst::default();
    // Cooldown left after the tick: no decision.
    let mut s = state(FleeMode::None, HeldAction::Attack, 0.5);
    assert_eq!(flee_step(&mut s, &tick(100, 0.0), &g, 0.25), FleeDecision::NotFleeing);
    assert!(close(s.action_cooldown, 0.25));
    // Reaction not due: the cooldown runs, nothing is decided.
    let mut s = FleeState::default();
    let wait = FleeStepInput { reaction_due: false, ..tick(100, 0.0) };
    assert_eq!(flee_step(&mut s, &wait, &g, 0.25), FleeDecision::NotFleeing);
    assert!(close(s.action_cooldown, -0.25));
    // Paralysed: nothing moves, not even the cooldown.
    let mut s = FleeState::default();
    let down = FleeStepInput { incapacitated: true, ..tick(100, 0.0) };
    assert_eq!(flee_step(&mut s, &down, &g, 0.25), FleeDecision::NotFleeing);
    assert_eq!(s, FleeState::default());
    // Mid-hit with no action yet: nothing.
    let mut s = FleeState::default();
    let busy = FleeStepInput { ready_to_prepare: false, ..tick(100, 0.0) };
    assert_eq!(flee_step(&mut s, &busy, &g, 0.25), FleeDecision::NotFleeing);
    assert_eq!(s.action, HeldAction::None);
}

#[test]
fn flee_step_immobile_parks_in_idle() {
    let g = FleeGmst::default();
    let mut s = state(FleeMode::None, HeldAction::Attack, 0.0);
    let stuck = FleeStepInput { mobile: false, ..tick(0, 100.0) };
    assert_eq!(flee_step(&mut s, &stuck, &g, 0.25), FleeDecision::Start);
    assert_eq!(s.mode, FleeMode::Idle);
    // Mobile again, mid-hit with an attack held: the flee state no longer matches it and stops.
    let busy = FleeStepInput { ready_to_prepare: false, ..tick(0, 100.0) };
    assert_eq!(flee_step(&mut s, &busy, &g, 0.25), FleeDecision::Stop);
    assert_eq!(s.mode, FleeMode::None);
}

#[test]
fn flee_step_unreachable_enemy() {
    let g = FleeGmst::default();
    // Out of reach and no hit attempted: combat ends.
    let mut s = state(FleeMode::None, HeldAction::Attack, 2.0);
    let away = FleeStepInput { can_fight: false, ..tick(0, 100.0) };
    assert_eq!(flee_step(&mut s, &away, &g, 0.25), FleeDecision::EndCombat);
    assert!(close(s.action_cooldown, 0.0));
    // The player out of reach after a hit attempt: flee, whatever the cooldown.
    let mut s = state(FleeMode::None, HeldAction::Attack, 2.0);
    let forced = FleeStepInput { force_flee_allowed: true, ..away };
    assert_eq!(flee_step(&mut s, &forced, &g, 0.25), FleeDecision::Start);
    assert_eq!(s, state(FleeMode::Idle, HeldAction::Flee, 3.0));
    // An attack with no path to the enemy flees too; a potion never paths.
    let mut s = FleeState::default();
    let lost = FleeStepInput { path_failed: true, ..tick(0, 100.0) };
    assert_eq!(flee_step(&mut s, &lost, &g, 0.25), FleeDecision::Start);
    assert_eq!(s, state(FleeMode::Idle, HeldAction::Flee, 3.0));
    let mut s = FleeState::default();
    let potion = FleeStepInput { best_action: HeldAction::Potion, anti_flee_rating: f32::MAX, ..lost };
    assert_eq!(flee_step(&mut s, &potion, &g, 0.25), FleeDecision::NotFleeing);
}

#[test]
fn flee_step_runs_while_fleeing() {
    let g = FleeGmst::default();
    // Fleeing, cooldown pending; the enemy comes into sight within reach: a blind run starts.
    let mut s = state(FleeMode::Idle, HeldAction::Flee, 2.0);
    let near = FleeStepInput { run: run(true, 80.0, 120.0), ..tick(100, 10.0) };
    assert_eq!(flee_step(&mut s, &near, &g, 0.25), FleeDecision::Keep);
    assert_eq!(s.mode, FleeMode::RunBlindly { timer: 0.0 });
    assert_eq!(flee_step(&mut s, &near, &g, 0.25), FleeDecision::Keep);
    assert_eq!(s.mode, FleeMode::RunBlindly { timer: 0.25 });
}

#[test]
fn flee_gmsts_read_from_records() {
    let mut db = RecordDb::new();
    db.register::<GameSetting>("GMST").unwrap();
    let src = db.add_source("test", &[]);
    assert_eq!(FleeGmst::from_db(&db), FleeGmst::default());

    // The vanilla records give the vanilla values.
    gmst(&mut db, src, "fAIFleeHealthMult", GameSettingValue::Float(7.0));
    gmst(&mut db, src, "fAIFleeFleeMult", GameSettingValue::Float(0.3));
    gmst(&mut db, src, "iWereWolfLevelToAttack", GameSettingValue::Integer(20));
    gmst(&mut db, src, "iWereWolfFleeMod", GameSettingValue::Integer(100));
    gmst(&mut db, src, "fFleeDistance", GameSettingValue::Float(3000.0));
    assert_eq!(FleeGmst::from_db(&db), FleeGmst::default());

    // A mod's values win and feed the rating: (1 - 0.5) * 20 + 50 * 1 + (10 - 0.01 * 500) = 65.
    let m = db.add_source("mod", &[src]);
    gmst(&mut db, m, "fAIFleeHealthMult", GameSettingValue::Float(20.0));
    gmst(&mut db, m, "fAIFleeFleeMult", GameSettingValue::Integer(1));
    gmst(&mut db, m, "iFightDistanceBase", GameSettingValue::Integer(10));
    gmst(&mut db, m, "fFightDistanceMultiplier", GameSettingValue::Float(0.01));
    gmst(&mut db, m, "iWereWolfLevelToAttack", GameSettingValue::Float(30.7));
    gmst(&mut db, m, "fFleeDistance", GameSettingValue::Float(500.0));
    let g = FleeGmst::from_db(&db);
    assert_eq!(g.werewolf_level_to_attack, 30);
    assert!(close(g.flee_distance, 500.0));
    assert!(close(flee_rating(&fleer(50, 50.0, 500.0), &g), 65.0));
}

// Greeting (`MA4c1`): `updateGreetingState` step by step with vanilla GMSTs
// (`iGreetDistanceMultiplier` 6, `iGreetDuration` 4 -> 16 updates, `fGreetDistanceReset` 512).

use crate::settings::{
    GREET_TURN_EPSILON_RAD, GreetAction, GreetGmst, GreetInput, GreetState, GreetingState, HelloTimer,
    greet_geometry, greeting_step, normalize_angle,
};

/// Hello 30 (reach 180 units), player straight ahead at `distance_units`, every gate open, a full update.
fn greeter(distance_units: f32) -> GreetInput {
    GreetInput {
        hello: 30,
        distance_units,
        angle_to_player: 0.0,
        heading: 0.0,
        in_combat: false,
        swimming: false,
        package_allows: true,
        player_dead: false,
        paralyzed: false,
        player_hidden: false,
        los: true,
        aware: true,
        can_say_hello: true,
        movement_forced: false,
        smooth_movement: false,
        turn_only: false,
    }
}

fn greet(s: &mut GreetState, input: &GreetInput) -> GreetAction {
    greeting_step(s, input, &GreetGmst::default())
}

/// Full updates until one says Hello: the number of updates it took, or `None` within `max`.
fn updates_to_hello(s: &mut GreetState, input: &GreetInput, max: usize) -> Option<usize> {
    (1..=max).find(|_| greet(s, input).say_hello)
}

#[test]
fn greet_out_of_range_does_nothing() {
    let mut s = GreetState::default();
    assert_eq!(updates_to_hello(&mut s, &greeter(181.0), 50), None);
    assert_eq!(s, GreetState::default());
    // Exactly at the reach counts (`distSquared <= helloDistance^2`).
    assert_eq!(updates_to_hello(&mut s, &greeter(180.0), 50), Some(2));
}

#[test]
fn greet_once_then_done_holds() {
    let mut s = GreetState::default();
    let near = greeter(100.0);
    let a = greet(&mut s, &near);
    assert_eq!(a, GreetAction::default());
    assert_eq!((s.state, s.timer), (GreetingState::None, 1));

    let a = greet(&mut s, &near);
    assert_eq!(a, GreetAction { say_hello: true, play_idle2: false, hold_still: true, turn_to: None });
    assert_eq!((s.state, s.timer, s.turning), (GreetingState::InProgress, 1, true));

    // Facing the player for 16 updates in all (timer 1..=16), idle2 kept playing.
    for timer in 2..=16 {
        let a = greet(&mut s, &near);
        assert_eq!(a, GreetAction { say_hello: false, play_idle2: true, hold_still: true, turn_to: None });
        assert_eq!((s.state, s.timer), (GreetingState::InProgress, timer));
    }
    let a = greet(&mut s, &near);
    assert_eq!(a, GreetAction { say_hello: false, play_idle2: true, hold_still: false, turn_to: None });
    assert_eq!((s.state, s.timer), (GreetingState::Done, 0));

    for _ in 0..50 {
        assert_eq!(greet(&mut s, &near), GreetAction::default());
    }
    assert_eq!((s.state, s.timer), (GreetingState::Done, 0));
}

#[test]
fn greet_walking_away_rearms() {
    let mut s = GreetState { state: GreetingState::Done, ..GreetState::default() };
    greet(&mut s, &greeter(511.0));
    assert_eq!(s.state, GreetingState::Done);
    // `distSquared >= fGreetDistanceReset^2`.
    greet(&mut s, &greeter(512.0));
    assert_eq!((s.state, s.timer), (GreetingState::None, 0));
    assert_eq!(updates_to_hello(&mut s, &greeter(100.0), 10), Some(2));

    // Leaving mid-greeting faces once more, then resets in the same update.
    let a = greet(&mut s, &greeter(600.0));
    assert!(a.hold_still && a.play_idle2);
    assert_eq!((s.state, s.timer), (GreetingState::None, 0));
}

#[test]
fn greet_hello_0_never_greets() {
    let mut s = GreetState::default();
    let input = GreetInput { hello: 0, ..greeter(1.0) };
    assert_eq!(updates_to_hello(&mut s, &input, 100), None);
    assert_eq!(s, GreetState::default());
}

#[test]
fn greet_count_never_decays_out_of_range() {
    let mut s = GreetState::default();
    greet(&mut s, &greeter(100.0));
    for _ in 0..20 {
        greet(&mut s, &greeter(300.0));
    }
    assert_eq!((s.state, s.timer), (GreetingState::None, 1));
    assert!(greet(&mut s, &greeter(100.0)).say_hello);
}

#[test]
fn greet_count_gates() {
    let base = greeter(100.0);
    for input in [
        GreetInput { los: false, ..base },
        GreetInput { aware: false, ..base },
        GreetInput { player_hidden: true, ..base },
        GreetInput { player_dead: true, ..base },
        GreetInput { paralyzed: true, ..base },
    ] {
        let mut s = GreetState::default();
        assert_eq!(updates_to_hello(&mut s, &input, 20), None);
        assert_eq!(s, GreetState::default());
    }
}

#[test]
fn greet_combat_swimming_or_package_clear_the_state() {
    let busy = GreetState { state: GreetingState::InProgress, timer: 5, turning: true, angle_to_player: 1.0 };
    let base = greeter(100.0);
    for input in [
        GreetInput { in_combat: true, ..base },
        GreetInput { swimming: true, ..base },
        GreetInput { package_allows: false, ..base },
        GreetInput { in_combat: true, turn_only: true, ..base },
    ] {
        let mut s = busy;
        assert_eq!(greet(&mut s, &input), GreetAction::default());
        assert_eq!(s, GreetState { angle_to_player: 1.0, ..GreetState::default() });
    }
}

#[test]
fn greet_turn_only_only_turns() {
    // In range, but between full updates nothing counts.
    let mut s = GreetState::default();
    let between = GreetInput { turn_only: true, ..greeter(100.0) };
    for _ in 0..20 {
        assert_eq!(greet(&mut s, &between), GreetAction::default());
    }
    assert_eq!(s, GreetState::default());

    // A started turn runs every frame until within 5 degrees.
    let mut s = GreetState { state: GreetingState::InProgress, timer: 3, turning: true, angle_to_player: 1.0 };
    let a = greet(&mut s, &between);
    assert_eq!(a, GreetAction { say_hello: false, play_idle2: true, hold_still: false, turn_to: Some(1.0) });
    assert_eq!((s.timer, s.turning), (3, true));
    let a = greet(&mut s, &GreetInput { heading: 1.0 - GREET_TURN_EPSILON_RAD * 0.9, ..between });
    assert_eq!(a.turn_to, None);
    assert!(!s.turning);
    assert_eq!(s.timer, 3);
}

#[test]
fn greet_turn_wraps_and_smooth_movement_threshold() {
    // Heading 3.1 vs target -3.1 is 0.083 rad apart across the wrap: within 5 degrees.
    let mut s = GreetState { turning: true, angle_to_player: -3.1, ..GreetState::default() };
    let a = greet(&mut s, &GreetInput { heading: 3.1, turn_only: true, ..greeter(1000.0) });
    assert_eq!(a.turn_to, None);
    assert!(!s.turning);

    // Smooth movement: a turn starts only past 60 degrees.
    let smooth = |angle| GreetInput { smooth_movement: true, angle_to_player: angle, ..greeter(100.0) };
    let mut s = GreetState::default();
    updates_to_hello(&mut s, &smooth(1.0), 5);
    assert!(!s.turning);
    let mut s = GreetState::default();
    updates_to_hello(&mut s, &smooth(1.1), 5);
    assert!(s.turning);
    assert!(close(s.angle_to_player, 1.1));
    assert_eq!(greet(&mut s, &GreetInput { turn_only: true, ..smooth(1.1) }).turn_to, Some(1.1));
}

#[test]
fn greet_without_a_line_or_while_forced_to_move() {
    // Nothing to say: straight to Done, no facing.
    let mut s = GreetState::default();
    let mute = GreetInput { can_say_hello: false, ..greeter(100.0) };
    greet(&mut s, &mute);
    assert_eq!(greet(&mut s, &mute), GreetAction::default());
    assert_eq!((s.state, s.timer), (GreetingState::Done, 0));

    // ForceJump / ForceSneak: greets but never stops or turns.
    let mut s = GreetState::default();
    let forced = GreetInput { movement_forced: true, ..greeter(100.0) };
    greet(&mut s, &forced);
    let a = greet(&mut s, &forced);
    assert!(a.say_hello && !a.hold_still);
    assert_eq!((s.state, s.turning), (GreetingState::InProgress, false));
}

#[test]
fn greet_duration_edges() {
    let g = |duration| GreetGmst { duration, ..GreetGmst::default() };
    assert_eq!(GreetGmst::default().duration_updates(), 16);
    assert_eq!(g(1).duration_updates(), 4);

    // iGreetDuration 1: the Hello update and 3 more face, the 5th is Done.
    let near = greeter(100.0);
    let mut s = GreetState::default();
    greeting_step(&mut s, &near, &g(1));
    assert!(greeting_step(&mut s, &near, &g(1)).say_hello);
    for _ in 0..3 {
        assert!(greeting_step(&mut s, &near, &g(1)).hold_still);
    }
    assert!(!greeting_step(&mut s, &near, &g(1)).hold_still);
    assert_eq!(s.state, GreetingState::Done);

    // iGreetDuration 0: says Hello and is Done in the same update.
    let mut s = GreetState::default();
    greeting_step(&mut s, &near, &g(0));
    let a = greeting_step(&mut s, &near, &g(0));
    assert!(a.say_hello && !a.hold_still);
    assert_eq!((s.state, s.timer), (GreetingState::Done, 0));
}

#[test]
fn hello_timer_full_update_every_quarter_second() {
    let mut t = HelloTimer::default();
    let frames: Vec<bool> = (0..6).map(|_| t.frame(0.1)).collect();
    // turn_only: full at 0, then 0.1, 0.2 turn only, 0.3 resets to a full update.
    assert_eq!(frames, [false, true, true, false, true, true]);
    for dt in [0.25, 0.3] {
        let mut t = HelloTimer::default();
        assert!((0..5).all(|_| !t.frame(dt)));
    }
}

#[test]
fn greet_geometry_and_normalize_angle() {
    // Engine -Z is Morrowind north (heading 0), +X east (heading pi/2); 64 units per 0.9144 m.
    let (d, a) = greet_geometry(Vec3::ZERO, Vec3::new(0.0, 0.0, -0.9144));
    assert!(close(d, 64.0) && close(a, 0.0));
    let (d, a) = greet_geometry(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.9144, 2.0, 3.0));
    assert!((d - 64.0).abs() < 1e-3 && close(a, std::f32::consts::FRAC_PI_2));
    assert!((normalize_angle(3.0 * std::f64::consts::PI) + std::f64::consts::PI).abs() < 1e-9);
    assert!((normalize_angle(-std::f64::consts::FRAC_PI_2) + std::f64::consts::FRAC_PI_2).abs() < 1e-12);
}

#[test]
fn greet_gmsts_read_from_records() {
    let mut db = RecordDb::new();
    db.register::<GameSetting>("GMST").unwrap();
    let src = db.add_source("test", &[]);
    assert_eq!(GreetGmst::from_db(&db), GreetGmst::default());

    // The vanilla records give the vanilla values.
    gmst(&mut db, src, "iGreetDistanceMultiplier", GameSettingValue::Integer(6));
    gmst(&mut db, src, "iGreetDuration", GameSettingValue::Integer(4));
    gmst(&mut db, src, "fGreetDistanceReset", GameSettingValue::Float(512.0));
    assert_eq!(GreetGmst::from_db(&db), GreetGmst::default());

    // A mod's values win: reach 30 * 10 = 300, 2 s (8 updates), reset at 300.
    let m = db.add_source("mod", &[src]);
    gmst(&mut db, m, "iGreetDistanceMultiplier", GameSettingValue::Integer(10));
    gmst(&mut db, m, "iGreetDuration", GameSettingValue::Float(2.9));
    gmst(&mut db, m, "fGreetDistanceReset", GameSettingValue::Integer(300));
    let g = GreetGmst::from_db(&db);
    assert_eq!(g, GreetGmst { distance_multiplier: 10, duration: 2, distance_reset: 300.0 });
    assert_eq!(g.duration_updates(), 8);
    let mut s = GreetState::default();
    greeting_step(&mut s, &greeter(250.0), &g);
    assert!(greeting_step(&mut s, &greeter(250.0), &g).say_hello);
    greeting_step(&mut s, &greeter(300.0), &g);
    assert_eq!(s.state, GreetingState::None);
}
