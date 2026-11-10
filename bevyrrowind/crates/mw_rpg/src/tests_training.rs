//! Training (A3), exact: the trainer's best three skills, rows, prices, refusals, the purchase's
//! effects, and the calendar advance. Synthetic sheets; vanilla GMSTs (`iTrainingMod` 10).

use mw_stats::{MwGmst, MwTable, SKILLS, SkillInfo, openmw_rules};
use sf_records::RecordId;
use sf_rpg::service::{EntryDetail, EntryId, EntryRefusal, EntryState, LabelRef, ServiceEffect};
use sf_stats::{Inputs, Sheet, StatOp, StatOwner};

use crate::services::{BarterTerms, PartyTerms};
use crate::training::{
    Calendar, TrainingOptions, TrainingSheets, advance_calendar, skill_entry, trainer_skills,
    training_entries, training_purchase,
};

const LONG_BLADE: usize = 5;
const DESTRUCTION: usize = 10;
const ACROBATICS: usize = 20;
const SPEECHCRAFT: usize = 25;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Every skill governed by Strength (attribute 0).
fn table() -> MwTable {
    MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
        governing: 0,
        specialization: 0,
        use_values: [1.0, 0.0, 0.0, 0.0],
    }))
    .unwrap()
}

fn sheet(t: &MwTable, player: bool, ops: &[StatOp]) -> Sheet {
    let rules = openmw_rules(t, &MwGmst::vanilla());
    let owner = if player {
        StatOwner::Player(sf_stats::PlayerKey(1))
    } else {
        StatOwner::Actor(sf_rpg::ActorKey(7))
    };
    let inputs = Inputs {
        is_player: player,
        ..Inputs::default()
    };
    let (sheet, rejected) = mw_stats::new_sheet(t, &rules, owner, &inputs, ops);
    assert_eq!(rejected, 0);
    sheet
}

fn skill(t: &MwTable, n: usize, value: f32) -> StatOp {
    StatOp::SetBase {
        stat: t.idx.skills[n],
        value,
    }
}

/// Trainer: Long Blade, Destruction and Speechcraft 60, Acrobatics 55, the rest below.
/// Player: Strength 30, Long Blade 20, Destruction 30 (= Strength), Speechcraft 60.
fn sheets(t: &MwTable) -> (Sheet, Sheet) {
    let trainer = sheet(
        t,
        false,
        &[
            skill(t, 0, 40.0),
            skill(t, 2, 50.0),
            skill(t, LONG_BLADE, 60.0),
            skill(t, DESTRUCTION, 60.0),
            skill(t, ACROBATICS, 55.0),
            skill(t, SPEECHCRAFT, 60.0),
            skill(t, 26, 60.0),
        ],
    );
    let player = sheet(
        t,
        true,
        &[
            StatOp::SetBase {
                stat: t.idx.attributes[0],
                value: 30.0,
            },
            skill(t, LONG_BLADE, 20.0),
            skill(t, DESTRUCTION, 30.0),
            skill(t, SPEECHCRAFT, 60.0),
        ],
    );
    (player, trainer)
}

/// Equal terms: the buy term is exactly 1.0, the price is `base x iTrainingMod`.
fn even() -> BarterTerms {
    let side = PartyTerms {
        mercantile: 30.0,
        luck: 40.0,
        personality: 40.0,
        fatigue_term: 1.25,
    };
    BarterTerms {
        merchant_is_creature: false,
        disposition: 50,
        pc: side,
        npc: side,
    }
}

#[test]
fn the_best_three_keep_record_order_on_ties() {
    // 0:40, 2:50, then 5:60 and 10:60 replace the lowest, 20:55 replaces 50, 25:60 replaces 55,
    // 26:60 is not strictly greater than the lowest 60 and is not taken.
    let mut values = [0.0; 27];
    for (n, v) in [
        (0, 40.0),
        (2, 50.0),
        (5, 60.0),
        (10, 60.0),
        (20, 55.0),
        (25, 60.0),
        (26, 60.0),
    ] {
        values[n] = v;
    }
    assert_eq!(
        trainer_skills(values).as_slice(),
        [(5, 60.0), (10, 60.0), (25, 60.0)]
    );
    // Fewer than three candidates above zero: zeros in record order fill the list.
    let mut values = [0.0; 27];
    values[7] = 10.0;
    assert_eq!(
        trainer_skills(values).as_slice(),
        [(7, 10.0), (0, 0.0), (1, 0.0)]
    );
}

#[test]
fn rows_are_the_trainers_best_three_with_prices_and_reasons() {
    let t = table();
    let (player, trainer) = sheets(&t);
    let s = TrainingSheets {
        table: &t,
        player: &player,
        trainer: &trainer,
    };
    let rows = training_entries(
        &s,
        Some(even()),
        TrainingOptions::default(),
        &MwGmst::vanilla(),
    );
    let summary: Vec<_> = rows
        .iter()
        .map(|e| (e.id.clone(), e.price, e.state.clone()))
        .collect();
    assert_eq!(
        summary,
        [
            (skill_entry(LONG_BLADE), 200, EntryState::Available),
            (
                skill_entry(DESTRUCTION),
                300,
                EntryState::Refused(LabelRef(id("sNotifyMessage17")))
            ),
            (
                skill_entry(SPEECHCRAFT),
                600,
                EntryState::Refused(LabelRef(id("sServiceTrainingWords")))
            ),
        ]
    );
    assert_eq!(rows[0].id, EntryId(id("longblade")));
    assert_eq!(rows[0].label, LabelRef(id("sSkillLongblade")));
    assert_eq!(
        rows[0].detail,
        EntryDetail::Stat {
            stat: id(SKILLS[LONG_BLADE]),
            current: 20,
            next: 21
        }
    );
}

#[test]
fn a_mercantile_difference_lowers_the_price_and_no_terms_price_at_the_base() {
    let t = table();
    let (player, trainer) = sheets(&t);
    let s = TrainingSheets {
        table: &t,
        player: &player,
        trainer: &trainer,
    };
    let g = MwGmst::vanilla();
    // pcTerm (50 + 4 + 8) x 1.25 = 77.5, npcTerm (30 + 4 + 8) x 1.25 = 52.5:
    // 0.01 x (100 - 0.5 x 25) = 0.875; int(200 x 0.875) = 175.
    let mut terms = even();
    terms.pc.mercantile = 50.0;
    assert_eq!(
        training_entries(&s, Some(terms), TrainingOptions::default(), &g)[0].price,
        175
    );
    assert_eq!(
        training_entries(&s, None, TrainingOptions::default(), &g)[0].price,
        200
    );
}

#[test]
fn a_skill_at_zero_costs_one_before_the_barter_offer() {
    let t = table();
    let trainer = sheet(&t, false, &[skill(&t, 0, 30.0)]);
    let player = sheet(
        &t,
        true,
        &[StatOp::SetBase {
            stat: t.idx.attributes[0],
            value: 30.0,
        }],
    );
    let s = TrainingSheets {
        table: &t,
        player: &player,
        trainer: &trainer,
    };
    let rows = training_entries(
        &s,
        Some(even()),
        TrainingOptions::default(),
        &MwGmst::vanilla(),
    );
    assert_eq!((rows[0].id.clone(), rows[0].price), (skill_entry(0), 1));
}

#[test]
fn trainers_use_their_base_skill_unless_the_option_is_off() {
    let t = table();
    let (player, mut trainer) = sheets(&t);
    // A fortified Acrobatics 80 (55 base): a Temporary-layer modifier, as a Fortify Skill gives.
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let inputs = Inputs::default();
    let owner = StatOwner::Actor(sf_rpg::ActorKey(7));
    let ctx = sf_stats::ApplyCtx {
        owner,
        cause: sf_stats::Cause {
            kind: sf_stats::CauseKind::Effect,
            source: None,
            tick: 0,
        },
        inputs: &inputs,
    };
    let fortify = StatOp::AddModifier(sf_stats::Modifier {
        stat: t.idx.skills[ACROBATICS],
        layer: sf_stats::Layer::Temporary,
        op: sf_stats::ModOp::Add,
        value: 25.0,
        source: sf_stats::SourceId {
            kind: sf_stats::SourceKind::Effect,
            id: 1,
        },
    });
    assert_eq!(
        sf_stats::apply(
            &t.table,
            &rules,
            &mut trainer,
            &[fortify],
            &ctx,
            &mut Vec::new()
        ),
        0
    );
    let s = TrainingSheets {
        table: &t,
        player: &player,
        trainer: &trainer,
    };
    let g = MwGmst::vanilla();
    let ids = |o: TrainingOptions| -> Vec<EntryId> {
        training_entries(&s, Some(even()), o, &g)
            .into_iter()
            .map(|e| e.id)
            .collect()
    };
    let base = ids(TrainingOptions { base_skill: true });
    assert_eq!(
        base,
        [
            skill_entry(LONG_BLADE),
            skill_entry(DESTRUCTION),
            skill_entry(SPEECHCRAFT)
        ]
    );
    let modified = ids(TrainingOptions { base_skill: false });
    assert_eq!(
        modified,
        [
            skill_entry(ACROBATICS),
            skill_entry(LONG_BLADE),
            skill_entry(DESTRUCTION)
        ]
    );
}

#[test]
fn a_purchase_pays_raises_and_passes_two_hours_or_says_why_not() {
    let t = table();
    let (player, trainer) = sheets(&t);
    let s = TrainingSheets {
        table: &t,
        player: &player,
        trainer: &trainer,
    };
    let g = MwGmst::vanilla();
    let buy = |entry: usize| {
        training_purchase(
            &s,
            Some(even()),
            TrainingOptions::default(),
            &g,
            &skill_entry(entry),
        )
    };
    assert_eq!(
        buy(LONG_BLADE),
        Ok(vec![
            ServiceEffect::Pay { amount: 200 },
            ServiceEffect::RaiseStat {
                stat: id("longblade")
            },
            ServiceEffect::PassTime {
                hours: 2,
                sleep: false
            },
        ])
    );
    assert_eq!(
        buy(DESTRUCTION),
        Err(EntryRefusal::Refused(LabelRef(id("sNotifyMessage17"))))
    );
    assert_eq!(
        buy(SPEECHCRAFT),
        Err(EntryRefusal::Refused(LabelRef(id("sServiceTrainingWords"))))
    );
    // Acrobatics 55 is the trainer's fourth best: not offered.
    assert_eq!(buy(ACROBATICS), Err(EntryRefusal::Unknown));
}

fn cal(hour: f32, days_passed: i32, day: i32, month: i32, year: i32) -> Calendar {
    Calendar {
        hour,
        days_passed,
        day,
        month,
        year,
    }
}

#[test]
fn the_calendar_wraps_hours_days_months_and_years() {
    assert_eq!(
        advance_calendar(cal(9.0, 0, 16, 7, 427), 2.0),
        cal(11.0, 0, 16, 7, 427)
    );
    assert_eq!(
        advance_calendar(cal(23.0, 4, 16, 7, 427), 2.0),
        cal(1.0, 5, 17, 7, 427)
    );
    // 31 Last Seed (month 7) to 1 Hearthfire.
    assert_eq!(
        advance_calendar(cal(23.5, 4, 31, 7, 427), 2.0),
        cal(1.5, 5, 1, 8, 427)
    );
    // 28 Sun's Dawn (February) to 1 First Seed; 31 Evening Star to the next year.
    assert_eq!(
        advance_calendar(cal(22.0, 0, 28, 1, 427), 3.0),
        cal(1.0, 1, 1, 2, 427)
    );
    assert_eq!(
        advance_calendar(cal(22.0, 0, 31, 11, 427), 3.0),
        cal(1.0, 1, 1, 0, 428)
    );
    // Several days at once (travel): 50 hours from 20:00 on day 30 of a 30-day month.
    assert_eq!(
        advance_calendar(cal(20.0, 0, 30, 3, 427), 50.0),
        cal(22.0, 2, 2, 4, 427)
    );
}
