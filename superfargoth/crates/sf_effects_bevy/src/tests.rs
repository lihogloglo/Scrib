//! Headless app (effects.md build plan E4): commands in, the sheet's modified values follow the
//! active set and expire on the fixed tick, events once; a measurement of the per-tick cost.

#![allow(clippy::float_cmp)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy_app::App;
use bevy_ecs::message::Messages;
use bevy_ecs::schedule::Schedule;
use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
use sf_effects::{
    Archetype, ArgKind, ArgRef, EffectDef, EffectEntry, EffectEvent, EffectList, EffectOp,
    EndReason, InstanceFlags, LayerRule, PoolMode, Range, RollSeed, SourceClass, SourceRef,
    StatTarget,
};
use sf_stats_bevy::sf_stats::{
    ActorKey, ApplyCtx, CauseKind, Inputs, PlayerKey, Rebase, Role, Rules, Settle, Sheet, StatDef,
    StatIdx, StatKind, StatTable, TestModel,
};
use sf_stats_bevy::{StatRules, StatTableRes};

use crate::*;

const STR: StatIdx = StatIdx(0);
const HEALTH: StatIdx = StatIdx(1);

const HERO: StatOwner = StatOwner::Player(PlayerKey(1));
const RAT: StatOwner = StatOwner::Actor(ActorKey(7));

fn def(key: &str, group: &str, kind: StatKind) -> StatDef {
    StatDef {
        key: key.into(),
        kind,
        group: group.into(),
        public: false,
        label: String::new(),
    }
}

/// Str (an attribute) and health.
fn table() -> StatTable {
    let defs = vec![
        def(
            "str",
            "attribute",
            StatKind::Value {
                min: 0.0,
                cap: None,
            },
        ),
        def(
            "health",
            "pool",
            StatKind::Pool {
                depleted_below: 1.0,
                rebase: Rebase::KeepCurrent,
            },
        ),
    ];
    StatTable::new(defs, &[(Role::Health, "health".into())]).expect("table builds")
}

fn rules() -> Rules {
    TestModel::default().rules()
}

/// Fortify / drain an attribute (by argument), and a rate health damage.
pub(crate) fn effect_table() -> EffectTable {
    let mut fortify = EffectDef::new(
        "fortify_attr",
        Archetype::Modify {
            stat: StatTarget::Arg,
            layer: LayerRule::BySource,
        },
    );
    fortify.arg = ArgKind::Stat("attribute".into());
    let mut drain = EffectDef::new(
        "drain_attr",
        Archetype::Drain {
            stat: StatTarget::Arg,
        },
    );
    drain.arg = ArgKind::Stat("attribute".into());
    let hurt = EffectDef::new(
        "damage_health",
        Archetype::Pool {
            pool: StatTarget::Fixed("health".into()),
            mode: PoolMode::Hurt,
        },
    );
    EffectTable::new(vec![fortify, drain, hurt]).expect("effects build")
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfEffectsPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(StatTableRes(Arc::new(table())))
        .insert_resource(StatRules(Arc::new(rules())))
        .insert_resource(EffectTableRes(Arc::new(effect_table())));
    // The first frame has no elapsed time: no fixed tick yet.
    app.update();
    app
}

/// Str 50, health 100, pools full.
fn sheet(owner: StatOwner) -> Sheet {
    let (table, rules) = (table(), rules());
    let mut sheet = Sheet::new(&table);
    let cause = Cause {
        kind: CauseKind::Rules,
        source: None,
        tick: 0,
    };
    let ctx = ApplyCtx {
        owner,
        cause,
        inputs: &Inputs::default(),
    };
    let ops = [
        StatOp::SetBase {
            stat: STR,
            value: 50.0,
        },
        StatOp::SetBase {
            stat: HEALTH,
            value: 100.0,
        },
    ];
    assert_eq!(
        sf_stats_bevy::sf_stats::apply(&table, &rules, &mut sheet, &ops, &ctx, &mut Vec::new()),
        0
    );
    sheet.settle(&table, &rules, &Inputs::default(), Settle::NewCharacter);
    sheet
}

fn spawn(app: &mut App, owner: StatOwner) -> Entity {
    app.world_mut()
        .spawn((
            Stats::new(owner, sheet(owner), Inputs::default()),
            Effects::new(owner),
        ))
        .id()
}

pub(crate) fn entry(effect: &str, arg: Option<&str>, seconds: f32, magnitude: f32) -> EffectEntry {
    EffectEntry {
        effect: effect.into(),
        arg: arg.map(|a| ArgRef::Stat(a.into())),
        range: Range::OnSelf,
        area: 0.0,
        duration: seconds,
        magnitude: (magnitude, magnitude),
    }
}

pub(crate) fn inflict(record: &str, class: SourceClass, entries: EffectList) -> EffectOp {
    EffectOp::Inflict {
        source: SourceRef {
            record: record.into(),
            item: None,
        },
        class,
        caster: None,
        entries,
        flags: InstanceFlags::default(),
        rolls: RollSeed(7),
    }
}

fn modified(app: &App, e: Entity, stat: StatIdx) -> f32 {
    let table = &app.world().resource::<StatTableRes>().0;
    app.world()
        .get::<Stats>(e)
        .unwrap()
        .view(table)
        .modified(stat)
}

fn drain(app: &mut App) -> Vec<EffectEvent> {
    app.world_mut()
        .resource_mut::<Messages<EffectMessage>>()
        .drain()
        .map(|m| m.0)
        .collect()
}

/// A fortify (+10 Str, 1 s) and a drain (-5 Str, 0.5 s) through one command: the sheet's
/// modified Str follows on the landing tick, each ends after its duration in fixed ticks (the
/// landing tick counts), and every event comes once.
#[test]
fn fortify_and_drain_land_through_commands_and_expire_on_the_tick() {
    let mut app = app();
    let hero = spawn(&mut app, HERO);
    let rat = spawn(&mut app, RAT);
    let ops = vec![
        inflict(
            "potion_str",
            SourceClass::Consumable,
            [entry("fortify_attr", Some("str"), 1.0, 10.0)]
                .into_iter()
                .collect(),
        ),
        inflict(
            "curse_str",
            SourceClass::Cast,
            [entry("drain_attr", Some("str"), 0.5, 5.0)]
                .into_iter()
                .collect(),
        ),
    ];
    app.world_mut().write_message(EffectCommand::new(HERO, ops));
    app.update();
    assert_eq!(
        modified(&app, hero, STR),
        55.0,
        "fortify and drain on the landing tick"
    );
    assert_eq!(
        modified(&app, rat, STR),
        50.0,
        "the other character is untouched"
    );
    let landed = drain(&mut app);
    let applied: Vec<_> = landed
        .iter()
        .filter(|e| matches!(e, EffectEvent::Applied { .. }))
        .collect();
    assert_eq!(applied.len(), 2, "{landed:?}");
    assert!(
        matches!(applied[0], EffectEvent::Applied { target: HERO, magnitude, .. } if *magnitude == 10.0)
    );
    assert!(
        matches!(applied[1], EffectEvent::Applied { target: HERO, magnitude, .. } if *magnitude == 5.0)
    );

    // The drain lasts 30 ticks, the landing tick the first.
    let mut events = Vec::new();
    for _ in 1..29 {
        app.update();
        events.extend(drain(&mut app));
    }
    assert_eq!(
        modified(&app, hero, STR),
        55.0,
        "29 ticks: both still running"
    );
    assert!(events.is_empty(), "{events:?}");
    app.update();
    assert_eq!(modified(&app, hero, STR), 60.0, "30 ticks: the drain ended");
    let ended = drain(&mut app);
    assert!(
        matches!(ended.as_slice(), [EffectEvent::Ended { target: HERO, reason: EndReason::Expired, effect, .. }] if effect.as_str() == "drain_attr"),
        "{ended:?}"
    );

    for _ in 30..59 {
        app.update();
        events.extend(drain(&mut app));
    }
    assert_eq!(modified(&app, hero, STR), 60.0);
    assert!(events.is_empty(), "{events:?}");
    app.update();
    assert_eq!(
        modified(&app, hero, STR),
        50.0,
        "60 ticks: the fortify ended"
    );
    let ended = drain(&mut app);
    assert!(
        matches!(
            ended.as_slice(),
            [EffectEvent::Ended {
                reason: EndReason::Expired,
                ..
            }]
        ),
        "{ended:?}"
    );
    assert!(app.world().get::<Effects>(hero).unwrap().active.is_empty());

    for _ in 0..10 {
        app.update();
    }
    assert!(drain(&mut app).is_empty(), "nothing after the end");
    assert_eq!(modified(&app, hero, STR), 50.0);
}

/// The stat ops reach the sheet as `StatCommand`s with the effect's cause and source, one
/// command per effect stage.
#[test]
fn stat_ops_go_out_as_one_command_per_effect_stage() {
    let mut app = app();
    spawn(&mut app, HERO);
    let entries = [
        entry("fortify_attr", Some("str"), 1.0, 10.0),
        entry("drain_attr", Some("str"), 1.0, 5.0),
    ];
    app.world_mut().write_message(EffectCommand::new(
        HERO,
        vec![inflict(
            "potion",
            SourceClass::Consumable,
            entries.into_iter().collect(),
        )],
    ));
    app.update();
    let sent: Vec<_> = app
        .world_mut()
        .resource_mut::<Messages<StatCommand>>()
        .drain()
        .collect();
    assert_eq!(sent.len(), 2, "{sent:?}");
    assert!(
        sent.iter()
            .all(|c| c.owner == HERO && c.cause == CauseKind::Effect && c.ops.len() == 1)
    );
    assert_ne!(
        sent[0].source, sent[1].source,
        "each effect has its own stats source"
    );
}

/// A command for an owner without effects is dropped; a set without timed effects is not
/// ticked; nothing runs without the effect table.
#[test]
fn unknown_targets_and_missing_tables_do_nothing() {
    let mut app = app();
    let hero = spawn(&mut app, HERO);
    app.world_mut().write_message(EffectCommand::new(
        RAT,
        vec![inflict("x", SourceClass::Cast, EffectList::new())],
    ));
    app.update();
    assert!(drain(&mut app).is_empty());
    assert_eq!(app.world().resource::<EffectIndex>().len(), 1);

    app.world_mut().remove_resource::<EffectTableRes>();
    let fortify = [entry("fortify_attr", Some("str"), 1.0, 10.0)]
        .into_iter()
        .collect();
    app.world_mut().write_message(EffectCommand::new(
        HERO,
        vec![inflict("potion", SourceClass::Consumable, fortify)],
    ));
    app.update();
    assert!(drain(&mut app).is_empty());
    assert_eq!(
        modified(&app, hero, STR),
        50.0,
        "no table: the command is cleared, not queued"
    );
    app.insert_resource(EffectTableRes(Arc::new(effect_table())));
    app.update();
    assert_eq!(modified(&app, hero, STR), 50.0);
}

/// effects.md "Performance budget": 500 resident characters, 2 000 active effects (450 with 4
/// innate, untimed; 50 with 3 timed modifiers and 1 rate), the effect tick alone and the whole
/// fixed tick (with the stats tick on 500 sheets).
#[test]
#[ignore = "measurement: effects per-tick cost, 500 characters / 2000 effects / 50 rate"]
fn measure_tick_cost() {
    let mut app = app();
    let mut ops_for = Vec::new();
    for i in 0..500u64 {
        let owner = StatOwner::Actor(ActorKey(u128::from(i) + 100));
        spawn(&mut app, owner);
        let ops = if i < 450 {
            let entries = (0..4)
                .map(|_| entry("fortify_attr", Some("str"), 0.0, 1.0))
                .collect();
            vec![inflict("ability", SourceClass::Innate, entries)]
        } else {
            let mut entries: EffectList = (0..3)
                .map(|_| entry("fortify_attr", Some("str"), 3600.0, 1.0))
                .collect();
            entries.push(entry("damage_health", None, 3600.0, 0.001));
            vec![inflict("spell", SourceClass::Consumable, entries)]
        };
        ops_for.push(EffectCommand::new(owner, ops));
    }
    app.world_mut().write_message_batch(ops_for);
    app.update();
    let live: usize = {
        let mut q = app.world_mut().query::<&Effects>();
        q.iter(app.world())
            .map(|fx| {
                fx.active
                    .sources()
                    .iter()
                    .map(|s| s.effects.len())
                    .sum::<usize>()
            })
            .sum()
    };
    assert_eq!(live, 2000);

    let n: u32 = 600;
    let mut tick_only = Schedule::default();
    tick_only.add_systems(tick_sets);
    let mut spent = Duration::ZERO;
    for _ in 0..n {
        let t = Instant::now();
        tick_only.run(app.world_mut());
        spent += t.elapsed();
        app.world_mut().resource_mut::<PendingEffectOut>().0.clear();
    }
    let whole = Instant::now();
    for _ in 0..n {
        app.update();
    }
    let whole = whole.elapsed();
    println!(
        "effects tick: {:.4} ms per fixed tick (effect step only); whole app update with stats: {:.4} ms",
        spent.as_secs_f64() * 1e3 / f64::from(n),
        whole.as_secs_f64() * 1e3 / f64::from(n)
    );
}
