//! effects.md "Tests" for E3: book sync, cast refused / fizzled / succeeded, delivery by range,
//! power recharge on game time, cooldowns, cures, `Request` effects. Exact assertions on a
//! small stat table, synthetic sources and [`TestRules`] (or the standard cast model for
//! cooldowns); the host is a loop over `sf_stats::apply` per emitted op.

// Exact float assertions are the point (CONTRIBUTING "Tests": logic and data are exact).
#![allow(clippy::float_cmp)]

use std::collections::BTreeMap;

use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Inputs, Rebase, Role, Rules, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp, StatOwner,
    StatTable, TestModel,
};
use smallvec::smallvec;

use crate::*;

const STR: StatIdx = StatIdx(0);
const MANA: StatIdx = StatIdx(1);

const ME: StatOwner = StatOwner::Actor(ActorKey(1));
const FOE: StatOwner = StatOwner::Actor(ActorKey(2));
/// Four ticks a second.
const DT: f32 = 0.25;

fn stats() -> StatTable {
    let value = StatKind::Value { min: 0.0, cap: None };
    let pool = StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepCurrent };
    let stat = |key: &str, group: &str, kind| StatDef {
        key: key.into(),
        kind,
        group: group.into(),
        public: false,
        label: format!("stat.{key}"),
    };
    let defs = vec![stat("str", "attribute", value), stat("mana", "pool", pool), stat("health", "pool", pool)];
    StatTable::new(defs, &[(Role::Health, "health".into()), (Role::Mana, "mana".into())]).unwrap()
}

fn defs() -> EffectTable {
    let str_ = || StatTarget::Fixed("str".into());
    let mut poison = EffectDef::new("poison", Archetype::Drain { stat: str_() });
    poison.flags.harmful = true;
    EffectTable::new(vec![
        EffectDef::new("fortify_str", Archetype::Modify { stat: str_(), layer: LayerRule::BySource }),
        EffectDef::new("drain_str", Archetype::Drain { stat: str_() }),
        poison,
        EffectDef::new("open", Archetype::Request { kind: Tag::new("open") }),
        EffectDef::new("cure_poison", Archetype::Cure { filter: CureFilter::Effect("poison".into()) }),
        EffectDef::new("cure_disease", Archetype::Cure { filter: CureFilter::SourceTag(Tag::new("disease")) }),
        EffectDef::new(
            "dispel",
            Archetype::Cure { filter: CureFilter::Dispel { harmful_only: false, chance_from_magnitude: true } },
        ),
    ])
    .unwrap()
}

fn entry(effect: &str, range: Range, magnitude: f32, seconds: f32) -> EffectEntry {
    EffectEntry { effect: effect.into(), arg: None, range, area: 0.0, duration: seconds, magnitude: (magnitude, magnitude) }
}

fn on_self(effect: &str, magnitude: f32, seconds: f32) -> EffectEntry {
    entry(effect, Range::OnSelf, magnitude, seconds)
}

fn sources() -> SourceTable {
    let mut spark = SourceDef::new(
        "spark",
        SourceClass::Cast,
        smallvec![
            on_self("fortify_str", 10.0, 1.0),
            entry("fortify_str", Range::Touch, 5.0, 1.0),
            entry("fortify_str", Range::Target, 7.0, 1.0),
        ],
    );
    spark.cost = Some(20.0);
    let mut might = SourceDef::new("might", SourceClass::Power, smallvec![on_self("fortify_str", 5.0, 1.0)]);
    might.recharge = Some(GameMinutes::DAY);
    let mut quick = SourceDef::new("quick", SourceClass::Cast, smallvec![on_self("fortify_str", 1.0, 1.0)]);
    quick.cost = Some(10.0);
    quick.cooldown = 8;
    let mut rot = SourceDef::new("rot", SourceClass::Affliction, smallvec![on_self("drain_str", 2.0, 0.0)]);
    rot.tags.push(Tag::new("disease"));
    SourceTable::new(vec![
        spark,
        might,
        quick,
        rot,
        SourceDef::new("strong", SourceClass::Innate, smallvec![on_self("fortify_str", 3.0, 0.0)]),
        SourceDef::new("opener", SourceClass::Cast, smallvec![on_self("open", 20.0, 1.0)]),
        SourceDef::new("purge", SourceClass::Cast, smallvec![on_self("cure_disease", 0.0, 0.0)]),
        SourceDef::new("dispeller", SourceClass::Cast, smallvec![on_self("dispel", 100.0, 0.0)]),
        SourceDef::new("potion", SourceClass::Consumable, smallvec![on_self("fortify_str", 1.0, 1.0)]),
    ])
    .unwrap()
}

fn rid(id: &str) -> RecordId {
    RecordId::new(id)
}

fn learn(id: &str) -> EffectOp {
    EffectOp::Learn { source: rid(id) }
}

fn cast_at(id: &str, target: Option<StatOwner>) -> EffectOp {
    EffectOp::Cast { source: rid(id), target, rolls: RollSeed(7) }
}

fn cast(id: &str) -> EffectOp {
    cast_at(id, None)
}

/// One caster (`ME`) with a book, str 50 and mana 100 (full), and a foe sheet.
struct World {
    stats: StatTable,
    rules: Rules,
    defs: EffectTable,
    sources: SourceTable,
    land: TestRules,
    set: ActiveSet,
    book: SpellBook,
    ids: EffectIdAlloc,
    sheets: BTreeMap<StatOwner, Sheet>,
    tick: u32,
    time: GameMinutes,
    events: Vec<EffectEvent>,
    requests: Vec<EffectRequest>,
    out: EffectOut,
}

impl World {
    fn new() -> Self {
        let stats = stats();
        let rules = TestModel::default().rules();
        let mut sheets = BTreeMap::new();
        for owner in [ME, FOE] {
            let mut s = Sheet::new(&stats);
            let init = [
                StatOp::SetBase { stat: STR, value: 50.0 },
                StatOp::SetBase { stat: MANA, value: 100.0 },
                StatOp::SetBase { stat: StatIdx(2), value: 100.0 },
            ];
            let inputs = Inputs::default();
            let cause = sf_stats::Cause { kind: sf_stats::CauseKind::Debug, source: None, tick: 0 };
            let ctx = ApplyCtx { owner, cause, inputs: &inputs };
            assert_eq!(sf_stats::apply(&stats, &rules, &mut s, &init, &ctx, &mut Vec::new()), 0);
            s.settle(&stats, &rules, &inputs, Settle::NewCharacter);
            sheets.insert(owner, s);
        }
        Self {
            stats,
            rules,
            defs: defs(),
            sources: sources(),
            land: TestRules::default(),
            set: ActiveSet::new(),
            book: SpellBook::new(),
            ids: EffectIdAlloc::default(),
            sheets,
            tick: 0,
            time: GameMinutes::default(),
            events: Vec::new(),
            requests: Vec::new(),
            out: EffectOut::default(),
        }
    }

    fn host(&mut self, out: EffectOut) {
        let inputs = Inputs::default();
        for (owner, op, cause) in &out.stat_ops {
            let sheet = self.sheets.get_mut(owner).unwrap();
            let ctx = ApplyCtx { owner: *owner, cause: *cause, inputs: &inputs };
            assert_eq!(sf_stats::apply(&self.stats, &self.rules, sheet, std::slice::from_ref(op), &ctx, &mut Vec::new()), 0);
        }
        self.events.extend(out.events.iter().cloned());
        self.requests.extend(out.requests.iter().cloned());
        self.out = out;
    }

    /// One `apply_book` on `ME` with `cast` as the cast model.
    fn run_with(&mut self, cast: &dyn CastModel, ops: &[EffectOp]) {
        let mut out = EffectOut::default();
        {
            let sheet = self.sheets[&ME].view(&self.stats);
            let env = EffectEnv {
                defs: &self.defs,
                stats: &self.stats,
                sources: &self.sources,
                land: &self.land,
                cast,
                sheet: Some(sheet),
                time: self.time,
            };
            let mut ctx = EffectCtx { target: ME, tick: self.tick, dt: DT, ids: &mut self.ids };
            apply_book(&env, &mut self.set, &mut self.book, ops, &mut ctx, &mut out);
        }
        self.host(out);
    }

    fn run(&mut self, ops: &[EffectOp]) {
        self.run_with(&TestRules::default(), ops);
    }

    fn steps(&mut self, n: u32) {
        for _ in 0..n {
            let mut out = EffectOut::default();
            let ctx = EffectCtx { target: ME, tick: self.tick, dt: DT, ids: &mut self.ids };
            tick(&self.defs, &self.stats, &mut self.set, &ctx, &mut out);
            self.host(out);
            self.tick += 1;
        }
    }

    fn str_(&self) -> f32 {
        self.sheets[&ME].view(&self.stats).modified(STR)
    }

    fn mana(&self) -> f32 {
        self.sheets[&ME].view(&self.stats).current(MANA)
    }

    /// The `CastFailed` reasons so far.
    fn failed(&self) -> Vec<CastFailReason> {
        self.events
            .iter()
            .filter_map(|e| if let EffectEvent::CastFailed { reason, .. } = e { Some(*reason) } else { None })
            .collect()
    }

    fn succeeded(&self) -> usize {
        self.events.iter().filter(|e| matches!(e, EffectEvent::CastSucceeded { .. })).count()
    }

    fn ended(&self) -> Vec<EndReason> {
        self.events.iter().filter_map(|e| if let EffectEvent::Ended { reason, .. } = e { Some(*reason) } else { None }).collect()
    }

    fn book_events(&self) -> Vec<(String, bool)> {
        self.events
            .iter()
            .filter_map(|e| match e {
                EffectEvent::BookChanged { source, added, .. } => Some((source.as_str().to_owned(), *added)),
                _ => None,
            })
            .collect()
    }

    fn rejected(&self) -> Vec<BookRejection> {
        self.events
            .iter()
            .filter_map(|e| if let EffectEvent::BookRejected { reason, .. } = e { Some(*reason) } else { None })
            .collect()
    }
}

#[test]
fn learn_and_forget_keep_the_book_and_the_active_set_in_sync() {
    let mut w = World::new();
    w.run(&[learn("strong")]);
    assert!(w.book.knows(&rid("strong")));
    assert_eq!(w.set.sources().len(), 1);
    assert_eq!(w.set.sources()[0].class, SourceClass::Innate);
    assert!(w.set.sources()[0].flags.ignore_resist, "innate sources ignore resistances");
    assert_eq!(w.set.sources()[0].caster, Some(ME));
    assert_eq!(w.str_(), 53.0);

    w.run(&[learn("strong")]);
    assert_eq!(w.set.sources().len(), 1, "learning twice is a no-op");
    assert_eq!(w.book.known(), [rid("strong")]);

    w.run(&[learn("spark")]);
    assert!(w.book.knows(&rid("spark")));
    assert_eq!(w.set.sources().len(), 1, "a spell is only castable");

    w.run(&[EffectOp::Forget { source: rid("strong") }]);
    assert!(!w.book.knows(&rid("strong")));
    assert!(w.set.is_empty());
    assert_eq!(w.str_(), 50.0);
    assert_eq!(w.ended(), [EndReason::SourceRemoved]);
    let changes = [("strong".to_owned(), true), ("spark".to_owned(), true), ("strong".to_owned(), false)];
    assert_eq!(w.book_events(), changes);

    w.run(&[learn("potion"), learn("nope"), EffectOp::Forget { source: rid("nope") }]);
    assert_eq!(w.rejected(), [BookRejection::NotABookSource, BookRejection::UnknownSource]);
    assert_eq!(w.book.known(), [rid("spark")]);
}

#[test]
fn select_takes_only_known_spells_and_powers() {
    let mut w = World::new();
    w.run(&[EffectOp::Select { source: Some(rid("spark")) }]);
    assert_eq!(w.rejected(), [BookRejection::NotKnown]);
    w.run(&[learn("spark"), learn("strong"), EffectOp::Select { source: Some(rid("strong")) }]);
    assert_eq!(w.rejected(), [BookRejection::NotKnown, BookRejection::NotABookSource]);
    w.run(&[EffectOp::Select { source: Some(rid("spark")) }]);
    assert_eq!(w.book.selected(), Some(&rid("spark")));
    w.run(&[EffectOp::Forget { source: rid("spark") }]);
    assert_eq!(w.book.selected(), None, "forgetting the selected spell clears the selection");
}

#[test]
fn cast_success_spends_the_cost_and_delivers_by_range() {
    let mut w = World::new();
    w.run(&[learn("spark"), cast_at("spark", Some(FOE))]);
    assert_eq!(w.succeeded(), 1);
    assert_eq!(w.mana(), 80.0);
    assert_eq!(w.str_(), 60.0, "the self entry lands on the caster now");
    let [EffectRequest::Touch { target, delivery: touch }, EffectRequest::Launch(far)] = w.requests.as_slice() else {
        panic!("expected touch then launch: {:?}", w.requests);
    };
    assert_eq!(*target, FOE);
    assert_eq!((touch.entries.len(), touch.entries[0].magnitude), (1, (5.0, 5.0)));
    assert_eq!((far.entries.len(), far.entries[0].magnitude), (1, (7.0, 7.0)));
    assert_eq!((touch.caster, touch.source.record.clone(), touch.rolls), (ME, rid("spark"), RollSeed(7)));
    let EffectOp::Inflict { caster, class, .. } = touch.inflict() else { panic!("inflict") };
    assert_eq!((caster, class), (Some(ME), SourceClass::Cast));

    // Without a contact the touch entries hit nothing.
    let mut w = World::new();
    w.run(&[learn("spark"), cast("spark")]);
    assert!(matches!(w.requests.as_slice(), [EffectRequest::Launch(_)]));
}

#[test]
fn refused_casts_spend_nothing_and_fizzles_spend_the_cost() {
    let mut w = World::new();
    w.run(&[cast("spark"), cast("nope"), learn("strong"), cast("strong")]);
    assert_eq!(w.failed(), [CastFailReason::NotKnown, CastFailReason::UnknownSource, CastFailReason::NotCastable]);
    assert_eq!(w.mana(), 100.0);

    w.run(&[learn("spark")]);
    let silenced = TestRules { cast: Some(CastPlan::Refused(CastFailReason::Silenced)), ..TestRules::default() };
    w.run_with(&silenced, &[cast("spark")]);
    assert_eq!(w.failed().last(), Some(&CastFailReason::Silenced));
    assert!(w.out.stat_ops.is_empty(), "a refusal spends nothing");
    assert_eq!(w.mana(), 100.0);

    let clumsy = TestRules { cast: Some(CastPlan::Ready { cost: 20.0, chance: 0.0, cooldown: 0 }), ..TestRules::default() };
    w.run_with(&clumsy, &[cast("spark")]);
    assert_eq!(w.failed().last(), Some(&CastFailReason::Fizzle));
    assert_eq!(w.mana(), 80.0, "a fizzle spends the cost");
    assert_eq!(w.str_(), 53.0, "only the ability; nothing delivered");
    assert!(w.requests.is_empty());
    assert_eq!(w.succeeded(), 0);
}

#[test]
fn power_recharges_on_game_time() {
    let mut w = World::new();
    w.time = GameMinutes(600);
    w.run(&[learn("might"), cast("might")]);
    assert_eq!(w.succeeded(), 1);
    assert_eq!(w.book.power_used(&rid("might")), Some(GameMinutes(600)));
    assert!(w.events.iter().any(|e| matches!(e, EffectEvent::PowerUsed { time: GameMinutes(600), .. })));
    assert_eq!(w.str_(), 55.0);

    w.time = GameMinutes(600 + 1439);
    w.run(&[cast("might")]);
    assert_eq!(w.failed(), [CastFailReason::Recharging]);

    w.time = GameMinutes(600 + 1440);
    w.run(&[cast("might")]);
    assert_eq!(w.succeeded(), 2);
    assert_eq!(w.book.power_used(&rid("might")), Some(GameMinutes(2040)));
    assert_eq!(w.str_(), 55.0, "the recast replaces the first landing");
    assert_eq!(w.mana(), 100.0, "TestRules casts sources without a cost for free");
}

#[test]
fn cooldown_blocks_until_its_tick() {
    let standard = StandardEffectRules::default();
    let mut w = World::new();
    w.run_with(&standard, &[learn("quick"), cast("quick")]);
    assert_eq!(w.succeeded(), 1);
    assert_eq!(w.book.ready_at(&rid("quick")), Some(8));
    assert_eq!(w.mana(), 90.0, "fixed cost, no skill discount");

    w.tick = 7;
    w.run_with(&standard, &[cast("quick")]);
    assert_eq!(w.failed(), [CastFailReason::Recharging]);
    assert_eq!(w.mana(), 90.0);

    w.tick = 8;
    w.run_with(&standard, &[cast("quick")]);
    assert_eq!(w.succeeded(), 2);
    assert_eq!(w.book.ready_at(&rid("quick")), Some(16));
    assert_eq!(w.mana(), 80.0);
}

#[test]
fn request_effects_start_end_and_can_be_invalidated() {
    let mut w = World::new();
    w.run(&[learn("opener"), cast("opener")]);
    let id = w.set.sources()[0].id;
    let [EffectRequest::Effect { kind, phase: RequestPhase::Start, instance, index: 0, magnitude, .. }] = w.requests.as_slice()
    else {
        panic!("expected one start: {:?}", w.requests);
    };
    assert_eq!((kind.as_str(), *instance, *magnitude), ("open", id, 20.0));
    assert!(w.out.stat_ops.is_empty(), "a request writes no stat");
    w.steps(4);
    assert!(matches!(w.requests.last(), Some(EffectRequest::Effect { phase: RequestPhase::End, .. })));
    assert_eq!(w.ended(), [EndReason::Expired]);
    assert!(w.set.is_empty());

    w.run(&[cast("opener")]);
    let id = w.set.sources()[0].id;
    let before = w.requests.len();
    w.run(&[EffectOp::Invalidate { instance: id, index: 0 }]);
    assert_eq!(w.ended(), [EndReason::Expired, EndReason::Invalid]);
    assert_eq!(w.requests.len(), before, "an invalid request gets no end request");
    assert!(w.set.is_empty());
    w.run(&[EffectOp::Invalidate { instance: id, index: 0 }]);
    assert_eq!(w.ended().len(), 2, "unknown instance: a no-op");
}

#[test]
fn cures_end_by_effect_by_tag_and_by_dispel() {
    let mut w = World::new();
    // Tag: the affliction's instance ends and it leaves the book.
    w.run(&[learn("rot")]);
    assert_eq!(w.str_(), 48.0);
    w.run(&[EffectOp::Cure { filter: CureFilter::SourceTag(Tag::new("disease")), rolls: RollSeed(1) }]);
    assert_eq!(w.str_(), 50.0);
    assert!(!w.book.knows(&rid("rot")) && w.set.is_empty());
    assert_eq!(w.ended(), [EndReason::Cured]);
    assert_eq!(w.book_events().last(), Some(&("rot".to_owned(), false)));

    // The same through a landing `Cure` effect.
    w.run(&[learn("rot"), learn("purge"), cast("purge")]);
    assert_eq!(w.str_(), 50.0);
    assert!(!w.book.knows(&rid("rot")));

    // Effect: only running effects of that key in temporary sources.
    let poison = EffectOp::Inflict {
        source: SourceRef { record: rid("venom"), item: None },
        class: SourceClass::Cast,
        caster: Some(FOE),
        entries: smallvec![on_self("poison", 3.0, 1.0), on_self("fortify_str", 1.0, 1.0)],
        flags: InstanceFlags::default(),
        rolls: RollSeed(2),
    };
    w.run(&[poison]);
    assert_eq!(w.str_(), 48.0);
    w.run(&[EffectOp::Cure { filter: CureFilter::Effect("poison".into()), rolls: RollSeed(3) }]);
    assert_eq!(w.str_(), 51.0, "the poison ends, its sibling stays");
    assert_eq!(w.set.sources().len(), 1);

    // Dispel at magnitude 100: every temporary spell, never the book's ability.
    w.run(&[learn("strong"), learn("dispeller"), cast("dispeller")]);
    assert_eq!(w.str_(), 53.0);
    assert!(w.ended().contains(&EndReason::Dispelled));
    assert!(w.book.knows(&rid("strong")));
    assert_eq!(w.set.sources().len(), 1);
    assert_eq!(w.set.sources()[0].class, SourceClass::Innate);
}

#[test]
fn book_round_trips_and_same_inputs_give_same_outputs() {
    let run = || {
        let mut w = World::new();
        w.time = GameMinutes(5);
        w.run_with(&StandardEffectRules::default(), &[learn("might"), learn("quick"), cast("might"), cast("quick")]);
        w
    };
    let (a, b) = (run(), run());
    assert_eq!(a.events, b.events);
    assert_eq!(a.set, b.set);
    let bytes = postcard::to_allocvec(&a.book).unwrap();
    assert_eq!(bytes, postcard::to_allocvec(&b.book).unwrap());
    let back: SpellBook = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, a.book);
    assert_eq!(back.ready_at(&rid("quick")), Some(8));
    assert_eq!(back.power_used(&rid("might")), Some(GameMinutes(5)));
}

/// clock.md: power recharges count the world instant's whole minutes.
#[test]
fn game_minutes_are_the_instant_rounded_down() {
    use sf_ids::GameInstant;
    assert_eq!(GameMinutes::from(GameInstant(59_999)), GameMinutes(0));
    assert_eq!(GameMinutes::from(GameInstant(86_400_000)), GameMinutes::DAY);
    assert_eq!(GameMinutes::from(GameInstant::from_minutes(1441)), GameMinutes(1441));
}
