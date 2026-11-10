//! `sf/effects` and the character's `effects` (effects.md "Saves"), through the real section
//! codec: the counter and a character's active set and book save and load byte-identically; a
//! save from before effects loads with the default counter and no effects. The shape goldens are
//! `tests/shapes/sf.effects.v1.json` and `sf.character.v4.json` (`tests_formats.rs`).

use sf_effects::{
    ActiveSet, Archetype, EffectCtx, EffectDef, EffectEntry, EffectIdAlloc, EffectOp, EffectOut,
    EffectTable, EffectsSave, InstanceFlags, LayerRule, Range, RollSeed, SourceClass, SourceRef,
    SpellBook, StatTarget, TestRules,
};
use sf_records::RecordId;
use sf_save::SaveFile;
use sf_stats::{ActorKey, StatDef, StatKind, StatOwner, StatTable};

use crate::sections::{self, EffectsWorldSave};
use crate::tests::{FxWorld, Party, app, load, new_game, play, save, state, wire};
use crate::{LoadRequest, SaveRequest};

/// A fortify Str for 2 s landed on tick 9 and aged 3 ticks, saved on tick 12.
fn effects() -> EffectsSave {
    let stats = StatTable::new(
        vec![StatDef {
            key: "str".into(),
            kind: StatKind::Value {
                min: 0.0,
                cap: None,
            },
            group: "g".into(),
            public: false,
            label: String::new(),
        }],
        &[],
    )
    .unwrap();
    let defs = EffectTable::new(vec![EffectDef::new(
        "fortify",
        Archetype::Modify {
            stat: StatTarget::Fixed("str".into()),
            layer: LayerRule::BySource,
        },
    )])
    .unwrap();
    let (mut set, mut ids, mut out) = (
        ActiveSet::new(),
        EffectIdAlloc { next: 41 },
        EffectOut::default(),
    );
    let entry = EffectEntry {
        effect: "fortify".into(),
        arg: None,
        range: Range::OnSelf,
        area: 0.0,
        duration: 2.0,
        magnitude: (10.0, 10.0),
    };
    let op = EffectOp::Inflict {
        source: SourceRef {
            record: RecordId::new("potion"),
            item: None,
        },
        class: SourceClass::Consumable,
        caster: Some(StatOwner::Actor(ActorKey(3))),
        entries: [entry].into_iter().collect(),
        flags: InstanceFlags::default(),
        rolls: RollSeed(1),
    };
    let target = StatOwner::Actor(ActorKey(1));
    sf_effects::apply(
        &defs,
        &stats,
        &TestRules::default(),
        &mut set,
        &[op],
        &mut EffectCtx {
            target,
            tick: 9,
            dt: 0.25,
            ids: &mut ids,
        },
        &mut out,
    );
    for tick in 10..13 {
        sf_effects::tick(
            &defs,
            &stats,
            &mut set,
            &EffectCtx {
                target,
                tick,
                dt: 0.25,
                ids: &mut ids,
            },
            &mut out,
        );
    }
    assert_eq!(set.sources().len(), 1);
    EffectsSave::capture(&set, &SpellBook::new(), 12)
}

#[test]
fn the_counter_and_a_characters_effects_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, y) = new_game(dir.path());
    play(&mut a, &y);
    let world = EffectsWorldSave {
        ids: EffectIdAlloc { next: 42 },
    };
    a.world_mut().resource_mut::<FxWorld>().0 = world;
    a.world_mut().resource_mut::<Party>().0[0].effects = Some(effects());
    let first = save(&mut a, SaveRequest::quick());
    let f = SaveFile::open(std::fs::read(&first.path).unwrap()).unwrap();
    assert_eq!(
        f.entry(&sections::key(sections::EFFECTS)).unwrap().version,
        sections::EFFECTS_VERSION
    );

    let (mut b, _) = new_game(dir.path());
    load(&mut b, LoadRequest::path(&first.path)).unwrap();
    assert_eq!(b.world().resource::<FxWorld>().0, world);
    assert_eq!(b.world().resource::<Party>().0[0].effects, Some(effects()));
    assert_eq!(state(&b), state(&a));
    let again = save(&mut b, SaveRequest::quick());
    let raw = |p: &std::path::Path| {
        let f = SaveFile::open(std::fs::read(p).unwrap()).unwrap();
        [
            sections::key(sections::EFFECTS),
            sections::character_key(crate::tests::HERO),
        ]
        .map(|k| f.raw(&k).unwrap())
    };
    assert_eq!(raw(&again.path), raw(&first.path));
}

#[test]
fn a_save_from_before_effects_loads_with_none() {
    let dir = tempfile::tempdir().unwrap();
    let old =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/character-v3.sfsave");
    let f = SaveFile::open(std::fs::read(&old).unwrap()).unwrap();
    assert!(f.entry(&sections::key(sections::EFFECTS)).is_none());
    let mut b = app(dir.path(), false);
    wire(&mut b);
    b.world_mut().resource_mut::<FxWorld>().0 = EffectsWorldSave {
        ids: EffectIdAlloc { next: 9 },
    };
    load(&mut b, LoadRequest::path(&old)).unwrap();
    assert_eq!(
        b.world().resource::<FxWorld>().0,
        EffectsWorldSave::default(),
        "full replace: the default counter"
    );
    assert!(
        b.world()
            .resource::<Party>()
            .0
            .iter()
            .all(|c| c.effects.is_none())
    );
    let again = save(&mut b, SaveRequest::quick());
    assert!(
        SaveFile::open(std::fs::read(&again.path).unwrap())
            .unwrap()
            .entry(&sections::key(sections::EFFECTS))
            .is_none()
    );
}
