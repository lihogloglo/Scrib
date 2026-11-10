//! effects.md "Tests" for E4's Bevy-free half: a save round trip with a fortify, a drain and a
//! pool fortify active re-registers modifiers only (sheet values equal, remaining durations equal,
//! the same expiry), fixed-tick fields rebase, drift is reported.

#![allow(clippy::float_cmp)]

use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, Inputs, Rebase, Role, Rules, Settle, Sheet, StatDef,
    StatIdx, StatKind, StatOp, StatOwner, StatTable, TestModel,
};
use smallvec::smallvec;

use crate::*;

const STR: StatIdx = StatIdx(0);
const HEALTH: StatIdx = StatIdx(1);
const TARGET: StatOwner = StatOwner::Actor(ActorKey(1));
const DT: f32 = 0.25;

fn stats() -> StatTable {
    let def = |key: &str, kind| StatDef {
        key: key.into(),
        kind,
        group: "g".into(),
        public: false,
        label: String::new(),
    };
    let defs = vec![
        def(
            "str",
            StatKind::Value {
                min: 0.0,
                cap: None,
            },
        ),
        def(
            "health",
            StatKind::Pool {
                depleted_below: 0.0,
                rebase: Rebase::KeepCurrent,
            },
        ),
    ];
    StatTable::new(defs, &[(Role::Health, "health".into())]).unwrap()
}

fn defs() -> EffectTable {
    let fixed = |k: &str| StatTarget::Fixed(k.into());
    EffectTable::new(vec![
        EffectDef::new(
            "fortify_str",
            Archetype::Modify {
                stat: fixed("str"),
                layer: LayerRule::BySource,
            },
        ),
        EffectDef::new("drain_str", Archetype::Drain { stat: fixed("str") }),
        EffectDef::new(
            "fortify_health",
            Archetype::Pool {
                pool: fixed("health"),
                mode: PoolMode::Fortify,
            },
        ),
    ])
    .unwrap()
}

fn entry(effect: &str, seconds: f32, magnitude: f32) -> EffectEntry {
    EffectEntry {
        effect: effect.into(),
        arg: None,
        range: Range::OnSelf,
        area: 0.0,
        duration: seconds,
        magnitude: (magnitude, magnitude),
    }
}

fn run(stats: &StatTable, rules: &Rules, sheet: &mut Sheet, out: &mut EffectOut) {
    for (owner, op, cause) in out.stat_ops.drain(..) {
        assert_eq!(owner, TARGET);
        let ctx = ApplyCtx {
            owner,
            cause,
            inputs: &Inputs::default(),
        };
        assert_eq!(
            sf_stats::apply(stats, rules, sheet, &[op], &ctx, &mut Vec::new()),
            0
        );
    }
    out.events.clear();
}

fn fresh_sheet(stats: &StatTable, rules: &Rules) -> Sheet {
    let mut sheet = Sheet::new(stats);
    let ctx = ApplyCtx {
        owner: TARGET,
        cause: Cause {
            kind: CauseKind::Debug,
            source: None,
            tick: 0,
        },
        inputs: &Inputs::default(),
    };
    let init = [
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
        sf_stats::apply(stats, rules, &mut sheet, &init, &ctx, &mut Vec::new()),
        0
    );
    sheet.settle(stats, rules, &Inputs::default(), Settle::NewCharacter);
    sheet
}

fn remaining(set: &ActiveSet) -> Vec<Option<u32>> {
    set.sources()
        .iter()
        .flat_map(|s| &s.effects)
        .map(|e| e.duration.map(|d| d - e.elapsed))
        .collect()
}

#[test]
#[allow(clippy::too_many_lines, reason = "one round trip, step by step")]
fn a_round_trip_with_a_fortify_and_a_drain_reregisters_modifiers_only() {
    let (stats, rules, defs) = (stats(), TestModel::default().rules(), defs());
    let land = TestRules::default();
    let mut sheet = fresh_sheet(&stats, &rules);
    let (mut set, mut ids, mut out) = (
        ActiveSet::new(),
        EffectIdAlloc::default(),
        EffectOut::default(),
    );
    let op = EffectOp::Inflict {
        source: SourceRef {
            record: RecordId::new("potion"),
            item: None,
        },
        class: SourceClass::Consumable,
        caster: None,
        entries: smallvec![
            entry("fortify_str", 4.0, 10.0),
            entry("drain_str", 2.0, 5.0),
            entry("fortify_health", 4.0, 20.0)
        ],
        flags: InstanceFlags::default(),
        rolls: RollSeed(1),
    };
    let mut ctx = EffectCtx {
        target: TARGET,
        tick: 100,
        dt: DT,
        ids: &mut ids,
    };
    apply(&defs, &stats, &land, &mut set, &[op], &mut ctx, &mut out);
    run(&stats, &rules, &mut sheet, &mut out);
    for t in 101..104 {
        tick(
            &defs,
            &stats,
            &mut set,
            &EffectCtx {
                target: TARGET,
                tick: t,
                dt: DT,
                ids: &mut ids,
            },
            &mut out,
        );
        run(&stats, &rules, &mut sheet, &mut out);
    }
    let read = |s: &Sheet| {
        (
            s.view(&stats).modified(STR),
            s.view(&stats).current(HEALTH),
            s.view(&stats).max(HEALTH),
        )
    };
    assert_eq!(read(&sheet), (55.0, 120.0, 120.0));

    // Save at tick 103; a new session loads at tick 7.
    let sheet_save = sheet.to_save(&stats);
    let fx_save = EffectsSave::capture(&set, &SpellBook::new(), 103);
    let bytes = postcard::to_stdvec(&fx_save).unwrap();
    let fx_save: EffectsSave = postcard::from_bytes(&bytes).unwrap();
    let mut new_ids = EffectIdAlloc::default();
    let (mut loaded, _, report) = fx_save.restore(&defs, &SourceTable::default(), 7, &mut new_ids);
    assert!(report.is_empty());
    assert_eq!(new_ids, ids, "the counter moves past the restored instance");
    assert_eq!(
        remaining(&loaded),
        remaining(&set),
        "remaining durations equal"
    );
    assert_eq!(
        loaded.sources()[0].started,
        4,
        "started rebased: landed 3 ticks before the save"
    );
    assert!(loaded.is_timed());

    let (mut back, sheet_report) = Sheet::from_save(&stats, &sheet_save);
    assert!(sheet_report.dropped.is_empty() && sheet_report.missing.is_empty());
    let mut ops = Vec::new();
    reregister(&defs, &stats, &loaded, TARGET, 7, &mut ops);
    assert_eq!(ops.len(), 2, "the str and health modifiers only: {ops:?}");
    assert!(ops.iter().all(|(o, op, c)| *o == TARGET
        && matches!(op, StatOp::AddModifier(_))
        && c.kind == CauseKind::Effect));
    let mut ops_out = EffectOut {
        stat_ops: ops,
        ..EffectOut::default()
    };
    run(&stats, &rules, &mut back, &mut ops_out);
    back.settle(&stats, &rules, &Inputs::default(), Settle::Loaded);
    assert_eq!(
        read(&back),
        read(&sheet),
        "modified values equal after the load"
    );
    assert_eq!(back.to_save(&stats), sheet_save);

    // Both run to the end the same way.
    let mut ids2 = new_ids;
    for k in 0..16 {
        tick(
            &defs,
            &stats,
            &mut set,
            &EffectCtx {
                target: TARGET,
                tick: 104 + k,
                dt: DT,
                ids: &mut ids,
            },
            &mut out,
        );
        run(&stats, &rules, &mut sheet, &mut out);
        tick(
            &defs,
            &stats,
            &mut loaded,
            &EffectCtx {
                target: TARGET,
                tick: 8 + k,
                dt: DT,
                ids: &mut ids2,
            },
            &mut out,
        );
        run(&stats, &rules, &mut back, &mut out);
        assert_eq!(read(&back), read(&sheet), "tick {k}");
        assert_eq!(remaining(&loaded), remaining(&set));
    }
    assert!(set.is_empty() && loaded.is_empty());
    assert_eq!(read(&back), (50.0, 100.0, 100.0));
}

#[test]
fn restore_drops_unknown_effects_and_sources_and_rebases_cooldowns() {
    let (stats, defs) = (stats(), defs());
    let mut set = ActiveSet::new();
    let mut ids = EffectIdAlloc { next: 40 };
    let mut out = EffectOut::default();
    let op = EffectOp::Inflict {
        source: SourceRef {
            record: RecordId::new("potion"),
            item: None,
        },
        class: SourceClass::Consumable,
        caster: None,
        entries: smallvec![
            entry("fortify_str", 4.0, 10.0),
            entry("drain_str", 4.0, 5.0)
        ],
        flags: InstanceFlags::default(),
        rolls: RollSeed(1),
    };
    let mut ctx = EffectCtx {
        target: TARGET,
        tick: 0,
        dt: DT,
        ids: &mut ids,
    };
    apply(
        &defs,
        &stats,
        &TestRules::default(),
        &mut set,
        &[op],
        &mut ctx,
        &mut out,
    );
    let mut book = SpellBook::new();
    book.known = vec![RecordId::new("gone"), RecordId::new("kept")];
    book.selected = Some(RecordId::new("gone"));
    book.cooldowns = [(RecordId::new("kept"), 130), (RecordId::new("over"), 90)].into();
    let save = EffectsSave::capture(&set, &book, 100);

    // A mod list without `drain_str` and without the `gone` spell.
    let fewer = EffectTable::new(vec![defs.get(&"fortify_str".into()).unwrap().clone()]).unwrap();
    let sources =
        SourceTable::new(vec![SourceDef::new("kept", SourceClass::Cast, smallvec![])]).unwrap();
    let mut fresh = EffectIdAlloc::default();
    let (active, book, report) = save.clone().restore(&fewer, &sources, 5, &mut fresh);
    assert_eq!(report.dropped_effects, vec![EffectKey::new("drain_str")]);
    assert_eq!(report.dropped_sources, vec![RecordId::new("gone")]);
    assert_eq!(active.sources()[0].effects.len(), 1);
    assert_eq!(book.known(), &[RecordId::new("kept")]);
    assert_eq!(book.selected(), None, "the selection went with its source");
    assert_eq!(
        book.ready_at(&RecordId::new("kept")),
        Some(35),
        "30 ticks left"
    );
    assert_eq!(
        book.ready_at(&RecordId::new("over")),
        None,
        "an expired cooldown is dropped"
    );
    assert_eq!(fresh.next, 41);
    assert!(EffectsSave::default().is_empty() && !save.is_empty());
}
