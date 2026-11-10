//! effects-adapter.md "Tests" for A5a: the classic cast chance against hand-computed `OpenMW`
//! values (`spellutil.cpp` `calcSpellBaseSuccessChance` / `getSpellSuccessChance`, vanilla
//! GMSTs), the modern cast rule, and casts through `sf_effects::apply_book` on a Morrowind
//! sheet: cost spent, refusal without magicka, fizzle spends the cost, a power once a day,
//! abilities and diseases never cast. Exact assertions on synthetic records.

#![allow(clippy::float_cmp)]

use std::sync::Arc;

use sf_effects::{
    ActiveSet, CastFailReason, EffectCtx, EffectEnv, EffectEvent, EffectIdAlloc, EffectOp,
    EffectOut, EffectRequest, EffectRules, GameMinutes, RollSeed, SourceClass, SourceTable,
    SpellBook, apply_book, roll,
};
use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, Inputs, Rules, Settle, Sheet, StatOp, StatOwner,
};
use smallvec::smallvec;
use tes3::esp::{AttributeId2, EffectId2, EffectRange, SpellFlags, SpellType};

use crate::tests::{effects, entry, spell};
use crate::*;

const ME: StatOwner = StatOwner::Actor(ActorKey(7));
const FOE: StatOwner = StatOwner::Actor(ActorKey(8));

/// The two-school spell of the tests: Fire Damage 10-20 for 2 s in 10 ft on target (destruction,
/// base cost 5) and Restore Health 5 for 10 s on self (restoration, base cost 1), record cost 12.
fn two_schools(destruction: f32, restoration: f32) -> CastChanceIn {
    CastChanceIn {
        effects: smallvec![
            ChanceEffectIn {
                base_cost: 5.0,
                school_skill: destruction,
                duration: 2.0,
                magnitude: (10.0, 20.0),
                area: 10.0,
                on_target: true,
                applied_once: false,
            },
            ChanceEffectIn {
                base_cost: 1.0,
                school_skill: restoration,
                duration: 10.0,
                magnitude: (5.0, 5.0),
                area: 0.0,
                on_target: false,
                applied_once: false,
            },
        ],
        spell_cost: 12.0,
        willpower: 40.0,
        luck: 50.0,
        sound: 10.0,
        silence: 0.0,
        // fFatigueBase 1.25 - fFatigueMult 0.5 x (1 - 150 / 200).
        fatigue_term: 1.125,
        class: SourceClass::Cast,
        always_succeeds: false,
        effect_cost_mult: 0.5,
    }
}

#[test]
fn classic_chance_picks_the_weakest_school() {
    // Fire: x = 2 x 0.1 x 5 x 15 + 10 x 0.05 x 5 = 17.5, x 1.5 x 0.5 = 13.125; 2 x 50 - x = 86.875.
    // Restore: x = 10 x 0.1 x 1 x 5 x 0.5 = 2.5; 2 x 30 - x = 57.5: the lowest, restoration.
    let i = two_schools(50.0, 30.0);
    // 60 - 12 + 0.2 x 40 + 0.1 x 50 = 61.
    assert_eq!(
        base_success_chance(&i),
        BaseChance {
            chance: 61.0,
            school_effect: Some(1)
        }
    );
    // (61 - Sound 10) x 1.125 = 57.375; the integer roll succeeds on 0..=57: 58 %.
    assert_eq!(success_chance(&i), 57.375);
    assert_eq!(roll_chance(success_chance(&i)), 0.58);

    // Restoration 60: 120 - 2.5 = 117.5 > 86.875, destruction is now the weakest.
    let i = two_schools(50.0, 60.0);
    assert_eq!(
        base_success_chance(&i),
        BaseChance {
            chance: 101.0,
            school_effect: Some(0)
        }
    );
    assert_eq!(success_chance(&i), 100.0, "(101 - 10) x 1.125 capped");

    // A full fatigue bar: 1.25; no Sound.
    let i = CastChanceIn {
        sound: 0.0,
        fatigue_term: 1.25,
        ..two_schools(20.0, 30.0)
    };
    // Destruction 40 - 13.125 = 26.875 < 57.5: 40 - 12 + 13 = 41, x 1.25 = 51.25.
    assert_eq!(success_chance(&i), 51.25);
    assert_eq!(roll_chance(51.25), 0.52);
}

#[test]
fn classic_chance_branches() {
    let base = two_schools(50.0, 30.0);
    let power = CastChanceIn {
        class: SourceClass::Power,
        silence: 50.0,
        ..base.clone()
    };
    assert_eq!(
        success_chance(&power),
        100.0,
        "a usable power, silenced or not"
    );
    assert_eq!(
        success_chance(&CastChanceIn {
            silence: 1.0,
            ..base.clone()
        }),
        0.0,
        "silenced"
    );
    assert_eq!(
        success_chance(&CastChanceIn {
            always_succeeds: true,
            sound: 500.0,
            ..base.clone()
        }),
        100.0
    );
    assert_eq!(
        success_chance(&CastChanceIn {
            sound: 200.0,
            ..base.clone()
        }),
        0.0,
        "never below 0"
    );
    let empty = CastChanceIn {
        effects: smallvec![],
        ..base
    };
    assert_eq!(
        base_success_chance(&empty),
        BaseChance {
            chance: -12.0 + 8.0 + 5.0,
            school_effect: None
        }
    );
    assert_eq!(
        (
            roll_chance(0.0),
            roll_chance(0.2),
            roll_chance(99.5),
            roll_chance(100.0)
        ),
        (0.0, 0.01, 1.0, 1.0)
    );
}

#[test]
fn modern_cast_rule() {
    let spell = ModernCastIn {
        spell_cost: 12.0,
        sound: 0.0,
        silence: 0.0,
        class: SourceClass::Cast,
    };
    assert_eq!(modern_cast(&spell), ModernCast::Cost(12.0));
    assert_eq!(
        modern_cast(&ModernCastIn {
            sound: 50.0,
            ..spell
        }),
        ModernCast::Cost(18.0),
        "Sound 50: +50 %"
    );
    assert_eq!(
        modern_cast(&ModernCastIn {
            silence: 1.0,
            ..spell
        }),
        ModernCast::Silenced
    );
    let power = ModernCastIn {
        class: SourceClass::Power,
        silence: 1.0,
        sound: 50.0,
        ..spell
    };
    assert_eq!(
        modern_cast(&power),
        ModernCast::Cost(12.0),
        "powers ignore Silence and Sound"
    );
}

/// A caster on Morrowind's stat table and rules, and the spells of the tests as sources.
struct Caster {
    t: mw_stats::MwTable,
    rules: Rules,
    sheet: Sheet,
    fx: Arc<MwEffects>,
    sources: SourceTable,
    set: ActiveSet,
    book: SpellBook,
    ids: EffectIdAlloc,
    time: GameMinutes,
    tick: u32,
}

const HEAL: &str = "heal";
const TOUCH: &str = "touch_might";
const POWER: &str = "power";
const ABILITY: &str = "ability";
const DISEASE: &str = "disease";

impl Caster {
    /// Intelligence 50 (magicka 50), fatigue 150 / 200, Willpower 40, Luck 50, Restoration 30.
    fn new() -> Self {
        let skill = mw_stats::SkillInfo {
            governing: 0,
            specialization: 0,
            use_values: [0.0; 4],
        };
        let t = mw_stats::MwTable::with_skills([skill; 27]).unwrap();
        let rules = mw_stats::openmw_rules(&t, &mw_stats::MwGmst::vanilla());
        let fx = Arc::new(effects());
        let heal = entry(EffectId2::RestoreHealth, EffectRange::OnSelf, (5, 5), 10, 0);
        let mut might = entry(
            EffectId2::FortifyAttribute,
            EffectRange::OnTouch,
            (10, 10),
            30,
            0,
        );
        might.attribute = AttributeId2::Strength;
        let mut self_might = might.clone();
        self_might.range = EffectRange::OnSelf;
        let spells = [
            spell(
                HEAL,
                SpellType::Spell,
                SpellFlags::empty(),
                12,
                vec![heal.clone()],
            ),
            spell(TOUCH, SpellType::Spell, SpellFlags::empty(), 5, vec![might]),
            spell(
                POWER,
                SpellType::Power,
                SpellFlags::empty(),
                0,
                vec![self_might.clone()],
            ),
            spell(
                ABILITY,
                SpellType::Ability,
                SpellFlags::empty(),
                0,
                vec![self_might],
            ),
            spell(
                DISEASE,
                SpellType::Disease,
                SpellFlags::empty(),
                0,
                vec![heal],
            ),
        ];
        let (sources, _) = source_table(&fx, &MwEffectGmst::vanilla(), &spells);
        let mut c = Self {
            sheet: Sheet::new(&t.table),
            t,
            rules,
            fx,
            sources,
            set: ActiveSet::new(),
            book: SpellBook::new(),
            ids: EffectIdAlloc::default(),
            time: GameMinutes::default(),
            tick: 0,
        };
        let i = c.t.idx.clone();
        let a = |n: usize| i.attributes[n];
        let mut init = vec![StatOp::SetBase {
            stat: i.health,
            value: 100.0,
        }];
        for (n, v) in [50.0, 50.0, 40.0, 50.0, 50.0, 60.0, 50.0, 50.0]
            .into_iter()
            .enumerate()
        {
            init.push(StatOp::SetBase {
                stat: a(n),
                value: v,
            });
        }
        // Destruction 50, Restoration 30 (skill indices 10 and 15).
        init.push(StatOp::SetBase {
            stat: i.skills[10],
            value: 50.0,
        });
        init.push(StatOp::SetBase {
            stat: i.skills[15],
            value: 30.0,
        });
        c.stat(&init);
        c.sheet
            .settle(&c.t.table, &c.rules, &inputs(), Settle::NewCharacter);
        c.stat(&[StatOp::PoolSetCurrent {
            pool: i.fatigue,
            value: 150.0,
        }]);
        c
    }

    fn stat(&mut self, ops: &[StatOp]) {
        let inputs = inputs();
        let cause = Cause {
            kind: CauseKind::Debug,
            source: None,
            tick: self.tick,
        };
        let ctx = ApplyCtx {
            owner: ME,
            cause,
            inputs: &inputs,
        };
        sf_stats::apply(
            &self.t.table,
            &self.rules,
            &mut self.sheet,
            ops,
            &ctx,
            &mut Vec::new(),
        );
    }

    fn magicka(&self) -> f32 {
        self.sheet.view(&self.t.table).current(self.t.idx.magicka)
    }

    /// One `apply_book` with `rules`; the caster's own stat ops land on its sheet.
    fn run(&mut self, rules: &EffectRules, ops: &[EffectOp]) -> EffectOut {
        let mut out = EffectOut::default();
        {
            let sheet = self.sheet.view(&self.t.table);
            let env = EffectEnv::new(
                self.fx.table(),
                &self.t.table,
                &self.sources,
                rules,
                Some(sheet),
                self.time,
            );
            let mut ctx = EffectCtx {
                target: ME,
                tick: self.tick,
                dt: 1.0 / 60.0,
                ids: &mut self.ids,
            };
            apply_book(&env, &mut self.set, &mut self.book, ops, &mut ctx, &mut out);
        }
        let mine: Vec<StatOp> = out
            .stat_ops
            .iter()
            .filter(|(o, _, _)| *o == ME)
            .map(|(_, op, _)| op.clone())
            .collect();
        self.stat(&mine);
        out
    }

    fn classic(&self) -> EffectRules {
        EffectRules::from_model(&OpenMwEffectRules::new(
            self.fx.clone(),
            MwEffectGmst::vanilla(),
            mw_stats::MwGmst::vanilla(),
        ))
    }
}

/// The player's inputs (`fPCbaseMagickaMult`: magicka = Intelligence).
fn inputs() -> Inputs {
    Inputs {
        is_player: true,
        ..Inputs::default()
    }
}

fn cast(id: &str, seed: u64) -> EffectOp {
    EffectOp::Cast {
        source: RecordId::new(id),
        target: Some(FOE),
        rolls: RollSeed(seed),
    }
}

fn learn(id: &str) -> EffectOp {
    EffectOp::Learn {
        source: RecordId::new(id),
    }
}

fn failed(out: &EffectOut) -> Option<CastFailReason> {
    out.events.iter().find_map(|e| match e {
        EffectEvent::CastFailed { reason, .. } => Some(*reason),
        _ => None,
    })
}

fn succeeded(out: &EffectOut) -> bool {
    out.events
        .iter()
        .any(|e| matches!(e, EffectEvent::CastSucceeded { .. }))
}

/// Seeds whose chance roll (`roll(seed, 0, 2)`) is below and at or above `chance`.
fn seeds(chance: f32) -> (u64, u64) {
    let pass = (0..1000_u64)
        .find(|&s| roll(RollSeed(s), 0, 2) < chance)
        .unwrap();
    let fail = (0..1000_u64)
        .find(|&s| roll(RollSeed(s), 0, 2) >= chance)
        .unwrap();
    (pass, fail)
}

#[test]
fn classic_casts_spend_refuse_and_fizzle() {
    let mut c = Caster::new();
    let rules = c.classic();
    assert_eq!(c.magicka(), 50.0);
    // Restore Health 5 x 10 s, restoration only: x = 2.5, 60 - 12 + 8 + 5 = 61, x 1.125 = 68.625:
    // the roll succeeds on 0..=68.
    let (pass, fail) = seeds(0.69);
    assert!(roll(RollSeed(pass), 0, 2) < 0.69 && roll(RollSeed(fail), 0, 2) >= 0.69);
    c.run(&rules, &[learn(HEAL)]);

    let out = c.run(&rules, &[cast(HEAL, fail)]);
    assert_eq!(failed(&out), Some(CastFailReason::Fizzle));
    assert_eq!(
        c.magicka(),
        38.0,
        "a fizzle spends the cost (startSpellCast took it)"
    );
    assert!(c.set.sources().is_empty());

    let out = c.run(&rules, &[cast(HEAL, pass)]);
    assert!(succeeded(&out));
    assert_eq!(c.magicka(), 26.0);
    assert_eq!(c.set.sources().len(), 1, "the self entries landed");

    c.run(&rules, &[cast(HEAL, pass), cast(HEAL, pass)]);
    assert_eq!(c.magicka(), 2.0);
    let out = c.run(&rules, &[cast(HEAL, pass)]);
    assert_eq!(
        failed(&out),
        Some(CastFailReason::NoMana),
        "2 < 12: refused"
    );
    assert_eq!(c.magicka(), 2.0, "nothing spent");

    // Silenced: chance 0, the cost still spent (OpenMW checks magicka only at the start).
    let mut c = Caster::new();
    let silence = c.t.table.find(SILENCE_STAT).unwrap();
    c.stat(&[StatOp::SetBase {
        stat: silence,
        value: 1.0,
    }]);
    c.run(&rules, &[learn(HEAL)]);
    let out = c.run(&rules, &[cast(HEAL, pass)]);
    assert_eq!(
        (failed(&out), c.magicka()),
        (Some(CastFailReason::Fizzle), 38.0)
    );
}

#[test]
fn classic_sound_lowers_the_chance() {
    let mut c = Caster::new();
    let rules = c.classic();
    let sound = c.t.table.find(SOUND_STAT).unwrap();
    c.stat(&[StatOp::SetBase {
        stat: sound,
        value: 40.0,
    }]);
    // (61 - 40) x 1.125 = 23.625: succeeds on 0..=23.
    let (pass, fail) = seeds(0.24);
    c.run(&rules, &[learn(HEAL)]);
    assert_eq!(
        failed(&c.run(&rules, &[cast(HEAL, fail)])),
        Some(CastFailReason::Fizzle)
    );
    assert!(succeeded(&c.run(&rules, &[cast(HEAL, pass)])));
}

#[test]
fn powers_once_a_day_and_innate_sources_never_cast() {
    for rules in [
        Caster::new().classic(),
        EffectRules::from_model(&MwModernEffectRules::default()),
    ] {
        let mut c = Caster::new();
        c.run(&rules, &[learn(POWER), learn(ABILITY), learn(DISEASE)]);
        let out = c.run(&rules, &[cast(POWER, 1)]);
        assert!(succeeded(&out));
        assert!(
            out.events
                .iter()
                .any(|e| matches!(e, EffectEvent::PowerUsed { .. }))
        );
        assert_eq!(
            c.book.power_used(&RecordId::new(POWER)),
            Some(GameMinutes(0))
        );

        c.time = GameMinutes(GameMinutes::DAY.0 - 1);
        assert_eq!(
            failed(&c.run(&rules, &[cast(POWER, 1)])),
            Some(CastFailReason::Recharging)
        );
        c.time = GameMinutes::DAY;
        assert!(
            succeeded(&c.run(&rules, &[cast(POWER, 1)])),
            "usable 24 game hours on"
        );
        assert_eq!(
            c.book.power_used(&RecordId::new(POWER)),
            Some(GameMinutes::DAY)
        );

        for id in [ABILITY, DISEASE] {
            assert_eq!(
                failed(&c.run(&rules, &[cast(id, 1)])),
                Some(CastFailReason::NotCastable),
                "{id}"
            );
        }
        assert_eq!(c.magicka(), 50.0, "powers of cost 0 spend nothing");
    }
}

#[test]
fn modern_casts_never_fizzle() {
    let mut c = Caster::new();
    let rules = EffectRules::from_model(&MwModernEffectRules::default());
    c.run(&rules, &[learn(HEAL), learn(TOUCH)]);
    // Any roll: chance 1.
    let (_, fail) = seeds(0.69);
    assert!(succeeded(&c.run(&rules, &[cast(HEAL, fail)])));
    assert_eq!(c.magicka(), 38.0);

    let sound = c.t.table.find(SOUND_STAT).unwrap();
    c.stat(&[StatOp::SetBase {
        stat: sound,
        value: 50.0,
    }]);
    assert!(succeeded(&c.run(&rules, &[cast(HEAL, fail)])));
    assert_eq!(c.magicka(), 20.0, "Sound 50: 12 x 1.5 = 18");

    // The touch spell: no self entry lands, the touch entries leave as a request for the contact.
    let out = c.run(&rules, &[cast(TOUCH, fail)]);
    assert!(succeeded(&out));
    assert!(
        matches!(out.requests.as_slice(), [EffectRequest::Touch { target: FOE, delivery }] if delivery.caster == ME),
        "{:?}",
        out.requests
    );
    assert_eq!(c.magicka(), 12.5, "5 x 1.5 = 7.5 spent");

    let silence = c.t.table.find(SILENCE_STAT).unwrap();
    c.stat(&[StatOp::SetBase {
        stat: silence,
        value: 1.0,
    }]);
    assert_eq!(
        failed(&c.run(&rules, &[cast(HEAL, fail)])),
        Some(CastFailReason::Silenced)
    );
    assert_eq!(c.magicka(), 12.5, "refused: nothing spent");
}

#[test]
fn the_cast_school_is_the_weakest() {
    let c = Caster::new();
    let src = c.sources.get(&RecordId::new(HEAL)).unwrap();
    let sheet = c.sheet.view(&c.t.table);
    let q = sf_effects::CastQuery {
        defs: c.fx.table(),
        source: &src.id,
        class: src.class,
        entries: &src.effects,
        cost: src.cost,
        always_succeeds: false,
        cooldown: 0,
        ready_at: None,
        tick: 0,
        caster: sheet,
        skill: None,
    };
    assert_eq!(
        spell_school(&c.fx, MwEffectGmst::vanilla(), &q),
        Some(sf_stats::StatKey::new("restoration"))
    );
    let rules = OpenMwEffectRules::new(
        c.fx.clone(),
        MwEffectGmst::vanilla(),
        mw_stats::MwGmst::vanilla(),
    );
    assert_eq!(success_chance(&rules.chance_in(&q)), 68.625);
}
