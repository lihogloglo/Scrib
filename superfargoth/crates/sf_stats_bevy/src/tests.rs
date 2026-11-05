//! Headless app (stats.md build plan S4): spawn, apply, regen, events in order.

#![allow(clippy::float_cmp)]

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::message::Messages;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use sf_stats::{
    ActorKey, CauseKind, Inputs, PlayerKey, Rebase, Role, Settle, Sheet, StatDef, StatEvent, StatIdx, StatKind, StatOp, StatOwner,
    StatTable, TestModel,
};

use crate::*;

const STR: StatIdx = StatIdx(0);
const HEALTH: StatIdx = StatIdx(1);
const STAMINA: StatIdx = StatIdx(2);
const LEVEL: StatIdx = StatIdx(3);

const HERO: StatOwner = StatOwner::Player(PlayerKey(1));
const RAT: StatOwner = StatOwner::Actor(ActorKey(7));
const GUARD: StatOwner = StatOwner::Actor(ActorKey(3));

fn def(key: &str, kind: StatKind) -> StatDef {
    StatDef { key: key.into(), kind, group: "test".into(), public: false, label: String::new() }
}

/// Str; health = 2 x Str; stamina = 3 x Str, 6 per second back; level.
fn table_with(extra: usize) -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let mut defs = vec![
        def("str", value),
        def("health", StatKind::Pool { depleted_below: 1.0, rebase: Rebase::KeepCurrent }),
        def("stamina", StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepRatio }),
        def("level", StatKind::Counter { min: 1.0, max: 100.0 }),
    ];
    defs.extend((0..extra).map(|i| def(&format!("skill{i}"), value)));
    StatTable::new(defs, &[(Role::Health, "health".into()), (Role::Stamina, "stamina".into()), (Role::Level, "level".into())])
        .expect("table builds")
}

fn rules() -> Rules {
    TestModel { pool_from: vec![(HEALTH, STR, 2.0), (STAMINA, STR, 3.0)], regen: vec![(STAMINA, 6.0)], ..TestModel::default() }.rules()
}

fn app(table: StatTable) -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfStatsPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(StatTableRes(Arc::new(table)))
        .insert_resource(StatRules(Arc::new(rules())));
    // The first frame has no elapsed time: no fixed tick yet.
    app.update();
    app
}

/// A settled sheet with Str `str` (pools full).
fn sheet(app: &App, str: f32) -> Sheet {
    let table = &app.world().resource::<StatTableRes>().0;
    let rules = &app.world().resource::<StatRules>().0;
    let mut sheet = Sheet::new(table);
    let ctx = ApplyCtx { owner: HERO, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs: &Inputs::default() };
    let ops = [StatOp::SetBase { stat: STR, value: str }, StatOp::CounterSet { stat: LEVEL, value: 1.0 }];
    assert_eq!(sf_stats::apply(table, rules, &mut sheet, &ops, &ctx, &mut Vec::new()), 0);
    sheet.settle(table, rules, &Inputs::default(), Settle::NewCharacter);
    sheet
}

fn spawn(app: &mut App, owner: StatOwner, str: f32) -> Entity {
    let s = sheet(app, str);
    app.world_mut().spawn(Stats::new(owner, s, Inputs::default())).id()
}

fn drain(app: &mut App) -> Vec<StatEvent> {
    app.world_mut().resource_mut::<Messages<StatMessage>>().drain().map(|m| m.0).collect()
}

fn view(app: &App, e: Entity, f: impl Fn(&SheetRead<'_>) -> f32) -> f32 {
    let table = &app.world().resource::<StatTableRes>().0;
    f(&app.world().get::<Stats>(e).expect("stats").view(table))
}

fn changed(e: &StatEvent) -> Option<(StatOwner, StatIdx, f32, f32)> {
    match *e {
        StatEvent::Changed { owner, stat, old, new, .. } => Some((owner, stat, old, new)),
        _ => None,
    }
}

#[test]
fn spawn_apply_regen_and_events_in_order() {
    let mut app = app(table_with(0));
    let hero = spawn(&mut app, HERO, 10.0);
    let rat = spawn(&mut app, RAT, 5.0);
    let guard = spawn(&mut app, GUARD, 20.0);
    app.update();
    assert_eq!(app.world().resource::<StatIndex>().get(RAT), Some(rat));
    assert_eq!(app.world().resource::<StatIndex>().len(), 3);
    assert!(drain(&mut app).is_empty(), "full sheets tick silently");
    let first = app.world().resource::<StatTick>().0;
    assert!(first >= 1, "the fixed tick ran");

    // Commands apply in write order; the tick's regen events follow, sorted by owner.
    let spend = |pool, delta| StatOp::PoolAdjust { pool, delta, below_zero: false, above_max: false };
    app.world_mut().write_message(StatCommand::new(RAT, CauseKind::Combat, vec![spend(STAMINA, -6.0)]));
    app.world_mut().write_message(StatCommand::new(HERO, CauseKind::Script, vec![StatOp::SetBase { stat: STR, value: 12.0 }]));
    app.world_mut().write_message(StatCommand::new(GUARD, CauseKind::Combat, vec![spend(STAMINA, -30.0), spend(HEALTH, -39.5)]));
    app.update();
    let events = drain(&mut app);
    let tick = app.world().resource::<StatTick>().0;
    assert_eq!(tick, first + 1);
    let applied: Vec<_> = events.iter().take_while(|e| !matches!(e, StatEvent::Changed { cause, .. } if cause.kind == CauseKind::Regen)).collect();
    let owners: Vec<_> = applied.iter().map(|e| e.owner()).collect();
    assert_eq!(owners.first(), Some(&RAT));
    assert_eq!(owners.last(), Some(&GUARD));
    assert!(owners.iter().position(|o| *o == HERO) < owners.iter().position(|o| *o == GUARD));
    assert!(applied.iter().any(|e| changed(e) == Some((RAT, STAMINA, 15.0, 9.0))));
    assert!(applied.iter().any(|e| changed(e) == Some((HERO, STR, 10.0, 12.0))));
    assert!(applied.iter().any(|e| matches!(e, StatEvent::Depleted { owner: GUARD, pool: HEALTH, .. })), "health 40 - 39.5 < 1");
    for e in &events {
        let StatEvent::Changed { cause, .. } = e else { continue };
        assert_eq!(cause.tick, tick);
    }
    // Regen in the same tick: one event per owner below max, by owner (GUARD 3 < RAT 7).
    let regen: Vec<_> = events[applied.len()..].iter().filter_map(changed).collect();
    assert_eq!(regen.iter().map(|r| r.0).collect::<Vec<_>>(), [GUARD, RAT]);
    assert_eq!(regen[1].2, 9.0);
    assert!((regen[1].3 - 9.1).abs() < 1e-5, "6 per second x 1/60 s: {regen:?}");
    // The hero's Str 12 moved both maxima; health keeps its current, stamina its ratio.
    assert_eq!(view(&app, hero, |v| v.max(HEALTH)), 24.0);
    assert_eq!(view(&app, hero, |v| v.current(HEALTH)), 20.0);
    assert_eq!(view(&app, hero, |v| v.current(STAMINA)), 36.0);
    assert_eq!(view(&app, guard, |v| v.current(HEALTH)), 0.5);

    // Regen runs until full, then stops.
    for _ in 0..70 {
        app.update();
    }
    assert_eq!(view(&app, rat, |v| v.current(STAMINA)), 15.0);
    drain(&mut app);
    for _ in 0..400 {
        app.update();
    }
    assert_eq!(view(&app, guard, |v| v.current(STAMINA)), 60.0);
    drain(&mut app);
    app.update();
    assert!(drain(&mut app).is_empty());
}

#[test]
fn a_combatant_keeps_its_stamina_regen_and_unknown_owners_are_dropped() {
    let mut app = app(table_with(0));
    let rat = spawn(&mut app, RAT, 5.0);
    app.world_mut().get_mut::<Stats>(rat).unwrap().inputs.combatant = true;
    app.world_mut().write_message(StatCommand::new(RAT, CauseKind::Combat, vec![StatOp::PoolSetCurrent { pool: STAMINA, value: 3.0 }]));
    app.world_mut().write_message(StatCommand::new(GUARD, CauseKind::Combat, vec![StatOp::PoolSetCurrent { pool: STAMINA, value: 3.0 }]));
    let mut events = Vec::new();
    for _ in 0..10 {
        app.update();
        events.extend(drain(&mut app));
    }
    assert_eq!(view(&app, rat, |v| v.current(STAMINA)), 3.0, "combat owns a combatant's stamina regen");
    assert_eq!(events.len(), 1, "{events:?}");
}

#[test]
fn the_index_follows_despawn_and_replace() {
    let mut app = app(table_with(0));
    let rat = spawn(&mut app, RAT, 5.0);
    app.update();
    let s = sheet(&app, 5.0);
    app.world_mut().entity_mut(rat).insert(Stats::new(GUARD, s, Inputs::default()));
    let index = app.world().resource::<StatIndex>();
    assert_eq!((index.get(RAT), index.get(GUARD)), (None, Some(rat)));
    app.world_mut().despawn(rat);
    assert!(app.world().resource::<StatIndex>().is_empty());
    // A synchronous host step publishes with the tick's events.
    let hero = spawn(&mut app, HERO, 10.0);
    let world = app.world_mut();
    let (table, rules) = (world.resource::<StatTableRes>().0.clone(), world.resource::<StatRules>().0.clone());
    let mut out = Vec::new();
    let cause = Cause { kind: CauseKind::Script, source: None, tick: 0 };
    world.get_mut::<Stats>(hero).unwrap().apply(&table, &rules, &[StatOp::SetBase { stat: STR, value: 11.0 }], cause, &mut out);
    world.resource_mut::<PendingStatEvents>().0.extend(out);
    app.update();
    assert!(drain(&mut app).iter().any(|e| changed(e) == Some((HERO, STR, 10.0, 11.0))));
}

/// 500 resident actors with a game-sized table, a quarter regenerating (budget stats.md
/// "Per-tick pipeline": under 0.05 ms per tick on the target laptop).
#[test]
#[ignore = "measurement: tick cost of 500 sheets"]
fn measure_tick_cost_500_sheets() {
    let mut app = app(table_with(110));
    let mut entities = Vec::new();
    for i in 0..500u128 {
        entities.push(spawn(&mut app, StatOwner::Actor(ActorKey(i)), 10.0));
    }
    app.update();
    let mut tick_ms = Vec::new();
    for round in 0..600 {
        if round % 60 == 0 {
            for e in entities.iter().step_by(4) {
                let owner = app.world().get::<Stats>(*e).unwrap().owner;
                app.world_mut().write_message(StatCommand::new(
                    owner,
                    CauseKind::Combat,
                    vec![StatOp::PoolAdjust { pool: STAMINA, delta: -20.0, below_zero: false, above_max: false }],
                ));
            }
        }
        let t = std::time::Instant::now();
        app.world_mut().run_schedule(FixedUpdate);
        tick_ms.push(t.elapsed().as_secs_f64() * 1e3);
        drain(&mut app);
    }
    tick_ms.sort_by(f64::total_cmp);
    println!(
        "stats tick, 500 sheets (125 regenerating, 4 + 110 stats): median {:.4} ms, p95 {:.4} ms",
        tick_ms[tick_ms.len() / 2],
        tick_ms[tick_ms.len() * 95 / 100]
    );
}
