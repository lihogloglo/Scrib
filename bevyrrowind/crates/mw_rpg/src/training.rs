//! Morrowind training on `sf_rpg::service`'s list path (services-adapter.md "Training", slice
//! A3): the trainer's three best skills as priced rows ([`training_entries`]), the purchase
//! re-checked now ([`training_purchase`]) and its effects, and the game clock advance the 2
//! hours of training make ([`advance_calendar`]). [`crate::MwServiceRules`] calls these for
//! `mw.training`; the host applies the effects (`mw_rpg_bevy` `dialogue/services.rs`).
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, paths
//! under `apps/openmw/`:
//! - `mwgui/trainingwindow.cpp` 75-104 `setPtr`: the trainer's best 3 skills, walking the skill
//!   store in order, a skill replacing the lowest kept one only when strictly greater, kept
//!   sorted by a stable sort (ties keep the earlier skill) ([`trainer_skills`]); 230-235
//!   `getSkillForTraining`: the trainer's modified skill, or its base under `trainers training
//!   skills based on base skill` (our default, owner 2026-11-08) ([`TrainingOptions`]); 116-121
//!   the list price ([`crate::training_price`], A1).
//! - `mwgui/trainingwindow.cpp` 163-210 `onTrainingSelected`: gold, then the trainer's skill
//!   must exceed the player's base (`sServiceTrainingWords`), then the base must be below the
//!   modified governing attribute (`sNotifyMessage17`); then `skillLevelUp(player, skill,
//!   "trainer")` (a level-up source that counts toward the level, `mw_stats`' `raise` with
//!   `IncreaseSource::Training`), the gold moves to the trainer's pool, and 2 hours pass
//!   (`onTrainingFinished` 217-228: `rest(2, false)` then `advanceTime(2)`) ([`training_purchase`]
//!   effects: `Pay`, `RaiseStat`, `PassTime { hours: 2, sleep: false }`). No per-level limit
//!   (that is Skyrim's rule).
//! - `mwworld/datetimemanager.cpp` 17-48, 66-106, 146-160 `advanceTime`, `setHour`, `setDay`:
//!   `GameHour` wraps at 24, `Day` rolls over the month lengths into `Month` (0-11) and `Year`,
//!   `DaysPassed` counts the whole days added ([`advance_calendar`]).
//!
//! Differences (modern, services-adapter.md "Options"): refused rows stay listed with their
//! reason (`OpenMW` greys only unaffordable rows and says why after the click); the paid price
//! is the listed one (`OpenMW`'s purchase path drops the list's `max(1, ..)`, a skill-0 edge);
//! a refused row reports its reason even when it is also unaffordable (`OpenMW` checks the gold
//! first and says nothing).

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::service::{
    EntryDetail, EntryId, EntryRefusal, EntryState, LabelRef, ServiceEffect, ServiceEntry,
};
use sf_stats::Sheet;
use smallvec::SmallVec;

use mw_stats::{MwGmst, MwTable, SKILLS};

use crate::services::{BarterTerms, TrainingIn, TrainingRefusal, training_price};

/// Hours a training session takes (`mTimeAdvancer.run(2)`, `rest(2, false)`, `advanceTime(2)`).
pub const TRAINING_HOURS: u16 = 2;

/// How many skills a trainer offers (`maxSkills`).
pub const TRAINER_SKILLS: usize = 3;

/// Training switches (services-adapter.md "Options").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingOptions {
    /// Trainers offer and teach by their base skill (`OpenMW`'s setting; no fortify exploit).
    /// Off: the modified skill, as vanilla.
    pub base_skill: bool,
}

impl Default for TrainingOptions {
    fn default() -> Self {
        Self { base_skill: true }
    }
}

/// The sheets training reads: the stat table, the player's and the trainer's.
#[derive(Clone, Copy)]
pub struct TrainingSheets<'a> {
    pub table: &'a MwTable,
    pub player: &'a Sheet,
    pub trainer: &'a Sheet,
}

/// The trainer's best [`TRAINER_SKILLS`] skills from its skill values in record order: (skill
/// index, value), best first (`setPtr` 75-104: ties keep the earlier skill).
pub fn trainer_skills(values: impl IntoIterator<Item = f32>) -> SmallVec<[(usize, f32); 3]> {
    let mut best: SmallVec<[(usize, f32); 3]> = SmallVec::new();
    let by_value = |a: &(usize, f32), b: &(usize, f32)| b.1.total_cmp(&a.1);
    for (skill, value) in values.into_iter().enumerate() {
        if best.len() < TRAINER_SKILLS {
            best.push((skill, value));
        } else if best[TRAINER_SKILLS - 1].1 < value {
            best[TRAINER_SKILLS - 1] = (skill, value);
        } else {
            continue;
        }
        // `slice::sort_by` is stable, as `std::stable_sort`.
        best.sort_by(by_value);
    }
    best
}

/// The display-name GMST of skill `n` (`sSkill<Name>`, the stat table's label).
fn skill_label(t: &MwTable, n: usize) -> LabelRef {
    LabelRef(RecordId::new(t.table.def(t.idx.skills[n]).label.as_str()))
}

/// The row id of skill `n`: its stat key (`alchemy`).
pub fn skill_entry(n: usize) -> EntryId {
    EntryId(RecordId::new(SKILLS[n]))
}

/// One offered skill, priced and checked.
struct Offer {
    skill: usize,
    pc_base: f32,
    price: i32,
    refusal: Option<TrainingRefusal>,
}

/// The trainer's offers for this player (`setPtr` with the purchase's checks).
#[allow(
    clippy::cast_precision_loss,
    reason = "iTrainingMod is a small integer GMST"
)]
fn offers(
    s: &TrainingSheets<'_>,
    terms: Option<BarterTerms>,
    options: TrainingOptions,
    g: &MwGmst,
) -> Vec<Offer> {
    let t = s.table;
    let trainer = s.trainer.view(&t.table);
    let player = s.player.view(&t.table);
    let value = |n: usize| {
        let stat = t.idx.skills[n];
        if options.base_skill {
            trainer.base(stat)
        } else {
            trainer.modified(stat)
        }
    };
    // Without terms (no disposition or sheets for the barter offer) the price is the base, as
    // `MwServiceRules::quote` prices at the base value.
    let terms = terms.unwrap_or(BarterTerms {
        merchant_is_creature: true,
        ..BarterTerms::default()
    });
    trainer_skills((0..SKILLS.len()).map(value))
        .into_iter()
        .map(|(skill, trainer_skill)| {
            let pc_base = player.base(t.idx.skills[skill]);
            let attribute = player.modified(t.idx.attributes[t.skills[skill].governing]);
            let out = training_price(&TrainingIn::new(
                terms,
                pc_base,
                trainer_skill,
                attribute,
                g,
            ));
            Offer {
                skill,
                pc_base,
                price: out.price,
                refusal: out.refusal,
            }
        })
        .collect()
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "skills are whole numbers in 0..=100 (and modded beyond, saturating)"
)]
fn entry(t: &MwTable, o: &Offer) -> ServiceEntry {
    let current = o.pc_base as i32;
    ServiceEntry {
        id: skill_entry(o.skill),
        label: skill_label(t, o.skill),
        detail: EntryDetail::Stat {
            stat: RecordId::new(SKILLS[o.skill]),
            current,
            next: current + 1,
        },
        price: i64::from(o.price),
        state: match o.refusal {
            Some(why) => EntryState::Refused(refusal_label(why)),
            None => EntryState::Available,
        },
    }
}

/// The label of a refusal: the GMST of `OpenMW`'s message.
pub fn refusal_label(why: TrainingRefusal) -> LabelRef {
    LabelRef(RecordId::new(why.message_gmst()))
}

/// The trainer's rows for this player, best skill first. Affordability is the screen's (it
/// knows the gold); refused rows carry the reason.
pub fn training_entries(
    s: &TrainingSheets<'_>,
    terms: Option<BarterTerms>,
    options: TrainingOptions,
    g: &MwGmst,
) -> Vec<ServiceEntry> {
    offers(s, terms, options, g)
        .iter()
        .map(|o| entry(s.table, o))
        .collect()
}

/// Buying row `entry` now: `Pay`, `RaiseStat` (the skill key; the host raises it with
/// `IncreaseSource::Training`), `PassTime` 2 hours awake. `Unknown` when the skill is not one
/// of the trainer's three, `Refused` with the reason's label.
pub fn training_purchase(
    s: &TrainingSheets<'_>,
    terms: Option<BarterTerms>,
    options: TrainingOptions,
    g: &MwGmst,
    entry: &EntryId,
) -> Result<Vec<ServiceEffect>, EntryRefusal> {
    let offer = offers(s, terms, options, g)
        .into_iter()
        .find(|o| skill_entry(o.skill) == *entry)
        .ok_or(EntryRefusal::Unknown)?;
    if let Some(why) = offer.refusal {
        return Err(EntryRefusal::Refused(refusal_label(why)));
    }
    Ok(vec![
        ServiceEffect::Pay {
            amount: i64::from(offer.price),
        },
        ServiceEffect::RaiseStat {
            stat: RecordId::new(SKILLS[offer.skill]),
        },
        ServiceEffect::PassTime {
            hours: TRAINING_HOURS,
            sleep: false,
        },
    ])
}

/// The `MWScript` calendar globals (`GameHour`, `DaysPassed`, `Day`, `Month`, `Year`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Calendar {
    /// 0 <= hour < 24.
    pub hour: f32,
    pub days_passed: i32,
    /// Day of the month, from 1.
    pub day: i32,
    /// 0 (Morning Star) to 11 (Evening Star).
    pub month: i32,
    pub year: i32,
}

/// Days in month `month` (0-11; `getDaysPerMonth`, February 28).
pub fn days_per_month(month: i32) -> i32 {
    match month {
        1 => 28,
        3 | 5 | 8 | 10 => 30,
        _ => 31,
    }
}

/// `DateTimeManager::advanceTime(hours)`: the hour wraps at 24, each whole day added rolls
/// `Day` over the month lengths into `Month` and `Year`, and counts in `DaysPassed`. Negative
/// hours clamp the hour at 0, as `setHour` does.
#[allow(
    clippy::cast_possible_truncation,
    reason = "OpenMW's static_cast<int> of whole days"
)]
pub fn advance_calendar(c: Calendar, hours: f32) -> Calendar {
    let total = (c.hour + hours).max(0.0);
    let days = (total / 24.0).floor();
    let mut out = Calendar {
        hour: total - days * 24.0,
        ..c
    };
    let days = days as i32;
    if days > 0 {
        // setDay(days + mDay)
        let mut day = (days + c.day).max(1);
        let mut month = c.month;
        while day > days_per_month(month) {
            day -= days_per_month(month);
            if month < 11 {
                month += 1;
            } else {
                month = 0;
                out.year += 1;
            }
        }
        out.day = day;
        out.month = month;
        out.days_passed += days;
    }
    out
}
