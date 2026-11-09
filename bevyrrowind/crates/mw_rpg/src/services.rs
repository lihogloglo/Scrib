//! Morrowind's service prices (services-adapter.md "Formulas", slice A1): the barter offer every
//! price passes through ([`barter_offer`] over a [`BarterTerms`]) and the six rules, each a pure
//! `fn(&In) -> Out` over one input struct holding every number it reads, GMSTs included
//! (rule-hooks.md "Declaring a rule"; ids in [`RULE_IDS`]):
//!
//! | Rule | Function | Input |
//! |---|---|---|
//! | `mw.barter.price` | [`barter_price`] | [`BarterPriceIn`] |
//! | `mw.barter.haggle` | [`barter_haggle`] | [`HaggleIn`] |
//! | `mw.services.training` | [`training_price`] | [`TrainingIn`] |
//! | `mw.services.travel` | [`travel_price`] | [`TravelIn`] |
//! | `mw.services.spell_price` | [`spell_price`] | [`SpellPriceIn`] |
//! | `mw.services.repair_price` | [`repair_price`] | [`RepairPriceIn`] |
//!
//! Bevy-free and not wired: trade sessions, offers and effects are slice A2+.
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`, paths
//! under `apps/openmw/`:
//! - `mwmechanics/mechanicsmanagerimp.cpp` 563-590 `getBarterOffer` ([`barter_offer`]): zero
//!   base and creature merchants keep the base; skills clamped to 100, `0.1 x Luck` and
//!   `0.2 x Personality` clamped to 10, times each side's fatigue term; buy and sell terms.
//!   Re-verified line for line at `c0790fc` (2026-11-09): the sell branch is `sellTerm` alone,
//!   with no `min(buyTerm, sellTerm)`, so at `pcTerm - npcTerm > 50` the raw sell offer does
//!   exceed the raw buy offer. `OpenMW` closes that round trip in `updateOffer`'s 75% cap
//!   (`tradewindow.cpp` 616-639, "to avoid exploits connected to buying and selling the same
//!   item"), which is [`BarterPriceIn::cap_75`] here, not in `getBarterOffer`.
//! - `mwgui/tradewindow.cpp` 38-46 `getEffectiveValue` ([`effective_value`]), 48-114 `haggle`
//!   ([`barter_haggle`]; unclamped terms, integer `d`, `rollDice(100) + 1`, the Mercantile
//!   skill-use scale), 447-457 the disposition change for NPC merchants after every haggle,
//!   608-645 `updateOffer` ([`barter_price`]; the 75% cap, sell cap for NPCs only).
//! - `mwgui/trainingwindow.cpp` 118-121 (list price with `max(1, ..)`), 172-191 (purchase:
//!   refusals `sServiceTrainingWords`, `sNotifyMessage17`) ([`training_price`]).
//! - `mwgui/travelwindow.cpp` 50-83 (price: `fMagesGuildTravel` for interior providers, else the
//!   3D distance in `double` over `fTravelMult`, followers, `max(1, ..)`), 203-213 (hours from
//!   the 2D distance over `fTravelTimeMult`, exterior providers only) ([`travel_price`]).
//! - `mwgui/spellbuyingwindow.cpp` 54-59 ([`spell_price`]).
//! - `mwgui/merchantrepair.cpp` 64-83 ([`repair_price`]).
//!
//! All arithmetic is `f32` in `OpenMW`'s order (the travel distance `f64`, as `std::pow`
//! promotes), and `int(..)` truncates toward zero, as Rust's `as` does (which also saturates
//! where C++ would be undefined). Integer intermediates that could overflow `i32` in C++ are
//! widened to `i64`; same results in range.
//!
//! **Randomness.** The haggle's only roll, `Misc::Rng::rollDice(100, prng) + 1`, is
//! [`HaggleIn::roll`] (1..=100), drawn by the caller from the world's seeded stream.
//!
//! Status: written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10).

use serde::{Deserialize, Serialize};
use sf_rpg::service::Side;

use mw_stats::{MwGmst, fatigue_term};

/// `mw.barter.price`.
pub const RULE_BARTER_PRICE: &str = "mw.barter.price";
/// `mw.barter.haggle`.
pub const RULE_BARTER_HAGGLE: &str = "mw.barter.haggle";
/// `mw.services.training`.
pub const RULE_TRAINING: &str = "mw.services.training";
/// `mw.services.travel`.
pub const RULE_TRAVEL: &str = "mw.services.travel";
/// `mw.services.spell_price`.
pub const RULE_SPELL_PRICE: &str = "mw.services.spell_price";
/// `mw.services.repair_price`.
pub const RULE_REPAIR_PRICE: &str = "mw.services.repair_price";
/// The six rule ids, in the module docs' order.
pub const RULE_IDS: [&str; 6] =
    [RULE_BARTER_PRICE, RULE_BARTER_HAGGLE, RULE_TRAINING, RULE_TRAVEL, RULE_SPELL_PRICE, RULE_REPAIR_PRICE];

/// One side's numbers in a barter (all modified values).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PartyTerms {
    /// Modified Mercantile (`getSkill`).
    pub mercantile: f32,
    /// Modified Luck.
    pub luck: f32,
    /// Modified Personality.
    pub personality: f32,
    /// `CreatureStats::getFatigueTerm` ([`fatigue_term`]).
    pub fatigue_term: f32,
}

impl PartyTerms {
    /// The terms from modified stats and current/maximum fatigue (`fFatigueBase`,
    /// `fFatigueMult` from `g`).
    #[must_use]
    pub fn new(
        mercantile: f32,
        luck: f32,
        personality: f32,
        fatigue_current: f32,
        fatigue_max: f32,
        g: &MwGmst,
    ) -> Self {
        Self { mercantile, luck, personality, fatigue_term: fatigue_term(g, fatigue_current, fatigue_max) }
    }
}

/// The block every price reads (services-adapter.md "Shared input: `BarterTerms`").
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BarterTerms {
    /// The merchant is a creature (`ptr.getType() == ESM::Creature`): prices are the base.
    pub merchant_is_creature: bool,
    /// The merchant's derived disposition, clamped 0..=100 (`getDerivedDisposition`, the
    /// conversation's temporary change included).
    pub disposition: i32,
    /// The player.
    pub pc: PartyTerms,
    /// The merchant.
    pub npc: PartyTerms,
}

/// `MechanicsManager::getBarterOffer(merchant, base, buying)`: the price of `base` gold of value,
/// `buying` when the player buys. Never below 1 unless `base` is 0.
#[allow(clippy::cast_precision_loss, reason = "OpenMW multiplies the int base and disposition as float")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's int(...) truncates toward zero, as `as` does")]
#[allow(clippy::many_single_char_names, clippy::similar_names, reason = "OpenMW's names, for side-by-side review")]
pub fn barter_offer(t: &BarterTerms, base: i32, buying: bool) -> i32 {
    if base == 0 || t.merchant_is_creature {
        return base;
    }
    let a = t.pc.mercantile.min(100.0);
    let b = (0.1 * t.pc.luck).min(10.0);
    let c = (0.2 * t.pc.personality).min(10.0);
    let d = t.npc.mercantile.min(100.0);
    let e = (0.1 * t.npc.luck).min(10.0);
    let f = (0.2 * t.npc.personality).min(10.0);
    let pc_term = ((t.disposition - 50) as f32 + a + b + c) * t.pc.fatigue_term;
    let npc_term = (d + e + f) * t.npc.fatigue_term;
    let term = if buying {
        0.01 * (100.0 - 0.5 * (pc_term - npc_term))
    } else {
        0.01 * (50.0 - 0.5 * (npc_term - pc_term))
    };
    ((base as f32 * term) as i32).max(1)
}

/// Input of [`effective_value`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveValueIn {
    /// The record's per-piece value (`getValue`).
    pub value: i32,
    /// `(health, max_health)` for classes with item health (weapons, armor, lockpicks, probes,
    /// repair tools), `None` otherwise.
    pub health: Option<(i32, i32)>,
    /// Pieces in the line.
    pub count: u32,
}

/// `getEffectiveValue` (`tradewindow.cpp` 38-46): `int(value x health / max x count)`; a zero
/// maximum gives 0 (`getItemNormalizedHealth`, `mwworld/class.cpp` 94-104).
#[allow(clippy::cast_precision_loss, reason = "OpenMW converts value, health and count to float")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn effective_value(i: &EffectiveValueIn) -> i32 {
    let mut price = i.value as f32;
    if let Some((health, max)) = i.health {
        price *= if max == 0 { 0.0 } else { health as f32 / max as f32 };
    }
    (price * i.count as f32) as i32
}

/// Input of `mw.barter.price` (one trade line).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BarterPriceIn {
    pub terms: BarterTerms,
    /// The line's [`effective_value`].
    pub base: i32,
    /// [`Side::Buy`]: the player buys from the merchant.
    pub side: Side,
    /// `OpenMW`'s 75% cap (option "75% barter cap"; off is pure vanilla).
    pub cap_75: bool,
}

/// `mw.barter.price` (`tradewindow.cpp` `updateOffer` 608-645): the line's price, `>= 0`.
/// Buying pays at least 75% of the base; selling to an NPC earns at most 75%.
#[allow(clippy::cast_precision_loss, reason = "OpenMW multiplies the int base as float")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn barter_price(i: &BarterPriceIn) -> i32 {
    let cap = (0.75 * i.base as f32).max(1.0) as i32;
    match i.side {
        Side::Buy => {
            let offer = barter_offer(&i.terms, i.base, true);
            if i.cap_75 { cap.max(offer) } else { offer }
        }
        Side::Sell => {
            let offer = barter_offer(&i.terms, i.base, false);
            if i.cap_75 && !i.terms.merchant_is_creature { cap.min(offer) } else { offer }
        }
    }
}

/// Input of `mw.barter.haggle`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HaggleIn {
    pub terms: BarterTerms,
    /// The balance the player asks (`mCurrentBalance`: positive = the merchant pays).
    pub player_offer: i32,
    /// The merchant's offer (`mCurrentMerchantOffer`: Σ sell − Σ buy).
    pub merchant_offer: i32,
    /// `rollDice(100) + 1`, drawn by the caller (1..=100).
    pub roll: i32,
    /// `fDispositionMod`.
    pub disposition_mod: f32,
    /// `fBargainOfferMulti`.
    pub bargain_offer_multi: f32,
    /// `fBargainOfferBase`.
    pub bargain_offer_base: f32,
    /// `iBarterSuccessDisposition`.
    pub success_disposition: i32,
    /// `iBarterFailDisposition`.
    pub fail_disposition: i32,
}

impl HaggleIn {
    /// The input with its GMSTs read from `g`.
    #[allow(clippy::cast_possible_truncation, reason = "integer GMSTs are stored as f32; getInteger is whole")]
    #[must_use]
    pub fn new(terms: BarterTerms, player_offer: i32, merchant_offer: i32, roll: i32, g: &MwGmst) -> Self {
        Self {
            terms,
            player_offer,
            merchant_offer,
            roll,
            disposition_mod: g.disposition_mod,
            bargain_offer_multi: g.bargain_offer_multi,
            bargain_offer_base: g.bargain_offer_base,
            success_disposition: g.barter_success_disposition as i32,
            fail_disposition: g.barter_fail_disposition as i32,
        }
    }
}

/// What `mw.barter.haggle` decided.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HaggleOut {
    pub accepted: bool,
    /// The Mercantile `Mercantile_Success` use scale (`skillUsageSucceeded`'s `skillGain`):
    /// non-zero only when accepted at a better price than offered.
    pub mercantile_scale: f32,
    /// `iBarterSuccessDisposition` or `iBarterFailDisposition` for NPC merchants, 0 for creatures.
    pub disposition_delta: i32,
}

/// `mw.barter.haggle` (`tradewindow.cpp` 48-114, disposition 447-457). Accepts at once when the
/// player asks no more than offered (creatures too, no skill use); otherwise creatures refuse and
/// NPCs roll against `fBargainOfferMulti x d + fBargainOfferBase + int(pcTerm - npcTerm)` with
/// unclamped terms. A buying player asking to be paid is refused.
#[allow(clippy::cast_precision_loss, reason = "OpenMW mixes these ints into float terms")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's int(...) truncates toward zero, as `as` does")]
#[allow(clippy::many_single_char_names, clippy::similar_names, reason = "OpenMW's names, for side-by-side review")]
pub fn barter_haggle(i: &HaggleIn) -> HaggleOut {
    let t = &i.terms;
    let delta = |accepted: bool| match (t.merchant_is_creature, accepted) {
        (true, _) => 0,
        (false, true) => i.success_disposition,
        (false, false) => i.fail_disposition,
    };
    if i.player_offer <= i.merchant_offer {
        return HaggleOut { accepted: true, mercantile_scale: 0.0, disposition_delta: delta(true) };
    }
    let rejected = HaggleOut { accepted: false, mercantile_scale: 0.0, disposition_delta: delta(false) };
    if t.merchant_is_creature {
        return rejected;
    }
    let buying = i.merchant_offer < 0;
    let a = i64::from(i.merchant_offer).abs();
    let b = i64::from(i.player_offer).abs();
    // Neither divisor is 0: buying means a > 0; selling means player > merchant >= 0, so b > 0.
    let d = if buying { 100 * (a - b) / a } else { 100 * (b - a) / b };

    let a1 = t.pc.mercantile;
    let b1 = 0.1 * t.pc.luck;
    let c1 = 0.2 * t.pc.personality;
    let d1 = t.npc.mercantile;
    let e1 = 0.1 * t.npc.luck;
    let f1 = 0.2 * t.npc.personality;
    let disposition_term = i.disposition_mod * (t.disposition - 50) as f32;
    let pc_term = (disposition_term + a1 + b1 + c1) * t.pc.fatigue_term;
    let npc_term = (d1 + e1 + f1) * t.npc.fatigue_term;
    let x = i.bargain_offer_multi * d as f32 + i.bargain_offer_base + ((pc_term - npc_term) as i32) as f32;

    if i.roll as f32 > x || (i.merchant_offer < 0 && 0 < i.player_offer) {
        return rejected;
    }
    let scale = if !buying && b > a {
        (100.0 * (b - a) as f32 / b as f32).floor()
    } else if buying && b < a {
        (100.0 * (a - b) as f32 / a as f32).floor()
    } else {
        0.0
    };
    HaggleOut { accepted: true, mercantile_scale: scale, disposition_delta: delta(true) }
}

/// Why a trainer refuses (`trainingwindow.cpp` 179-191, checked in this order after the gold).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrainingRefusal {
    /// The trainer's skill is not above the player's base skill.
    TrainerNotBetter,
    /// The player's base skill has reached the governing attribute.
    AttributeCap,
}

impl TrainingRefusal {
    /// The GMST holding the message `OpenMW` shows.
    pub fn message_gmst(self) -> &'static str {
        match self {
            Self::TrainerNotBetter => "sServiceTrainingWords",
            Self::AttributeCap => "sNotifyMessage17",
        }
    }
}

/// Input of `mw.services.training` (one skill).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrainingIn {
    pub terms: BarterTerms,
    /// The player's base skill (`getSkill(..).getBase()`, a float in `OpenMW`).
    pub pc_skill_base: f32,
    /// The trainer's skill (`getSkillForTraining`: modified, or base under the option).
    pub trainer_skill: f32,
    /// The player's modified governing attribute.
    pub governing_attr_modified: f32,
    /// `iTrainingMod`.
    pub training_mod: i32,
}

impl TrainingIn {
    /// The input with `iTrainingMod` read from `g`.
    #[allow(clippy::cast_possible_truncation, reason = "integer GMSTs are stored as f32; getInteger is whole")]
    #[must_use]
    pub fn new(
        terms: BarterTerms,
        pc_skill_base: f32,
        trainer_skill: f32,
        governing_attr_modified: f32,
        g: &MwGmst,
    ) -> Self {
        Self { terms, pc_skill_base, trainer_skill, governing_attr_modified, training_mod: g.training_mod as i32 }
    }
}

/// What `mw.services.training` gives for one skill.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingOut {
    pub price: i32,
    pub refusal: Option<TrainingRefusal>,
}

/// `mw.services.training`: `barter_offer(max(1, int(base x iTrainingMod)), buying)`. `OpenMW`'s
/// purchase path omits the list's `max(1, ..)`; the list's value is used for both, so the shown
/// price is the paid price (differs only at skill 0).
#[allow(clippy::cast_precision_loss, reason = "OpenMW multiplies the float skill by the int GMST")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn training_price(i: &TrainingIn) -> TrainingOut {
    let base = ((i.pc_skill_base * i.training_mod as f32) as i32).max(1);
    let price = barter_offer(&i.terms, base, true);
    let refusal = if i.trainer_skill <= i.pc_skill_base {
        Some(TrainingRefusal::TrainerNotBetter)
    } else if i.pc_skill_base >= i.governing_attr_modified {
        Some(TrainingRefusal::AttributeCap)
    } else {
        None
    };
    TrainingOut { price, refusal }
}

/// How travelling followers multiply the fare (option "Travel follower pricing").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FollowerPricing {
    /// `OpenMW`: `1 + followers` ("unlike vanilla the first follower doesn't travel for free").
    #[default]
    OpenMw,
    /// Morrowind: `max(1, followers)`.
    Vanilla,
}

/// Input of `mw.services.travel` (one destination).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TravelIn {
    pub terms: BarterTerms,
    /// The provider stands in an interior cell (`!getCell()->isExterior()`: guild guides).
    pub provider_interior: bool,
    /// The player's position (Morrowind units).
    pub player_pos: [f32; 3],
    /// The destination's `DODT` position.
    pub dest_pos: [f32; 3],
    /// Followers that travel along (`ActionTeleport::getFollowers`).
    pub followers: u32,
    pub follower_pricing: FollowerPricing,
    /// `fTravelMult`.
    pub travel_mult: f32,
    /// `fTravelTimeMult`.
    pub travel_time_mult: f32,
    /// `fMagesGuildTravel` (`getInteger()`).
    pub mages_guild_travel: i32,
}

impl TravelIn {
    /// The input with its GMSTs read from `g`.
    #[allow(clippy::cast_possible_truncation, reason = "getInteger() of the float GMST truncates, as `as` does")]
    #[must_use]
    pub fn new(
        terms: BarterTerms,
        provider_interior: bool,
        player_pos: [f32; 3],
        dest_pos: [f32; 3],
        followers: u32,
        follower_pricing: FollowerPricing,
        g: &MwGmst,
    ) -> Self {
        Self {
            terms,
            provider_interior,
            player_pos,
            dest_pos,
            followers,
            follower_pricing,
            travel_mult: g.travel_mult,
            travel_time_mult: g.travel_time_mult,
            mages_guild_travel: g.mages_guild_travel as i32,
        }
    }
}

/// What `mw.services.travel` gives for one destination.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TravelOut {
    pub price: i32,
    /// Hours passed (sleeping); 0 for interior providers.
    pub hours: i32,
}

/// `mw.services.travel`: the fare from the 3D distance (`double`), the hours from the 2D one
/// (`f32`, `osg::Vec2f::length`), so a destination straight up costs gold and takes no time.
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn travel_price(i: &TravelIn) -> TravelOut {
    let delta = |k: usize| i.dest_pos[k] - i.player_pos[k];
    let mut base = if i.provider_interior {
        i.mages_guild_travel
    } else {
        let d = (0..3).map(|k| f64::from(delta(k)).powi(2)).sum::<f64>().sqrt();
        if i.travel_mult == 0.0 { d as i32 } else { (d / f64::from(i.travel_mult)) as i32 }
    };
    let followers = i32::try_from(i.followers).unwrap_or(i32::MAX);
    let mult = match i.follower_pricing {
        FollowerPricing::OpenMw => followers.saturating_add(1),
        FollowerPricing::Vanilla => followers.max(1),
    };
    base = base.saturating_mul(mult);
    let price = barter_offer(&i.terms, base.max(1), true);
    let hours = if i.provider_interior {
        0
    } else {
        let (dx, dy) = (delta(0), delta(1));
        ((dx * dx + dy * dy).sqrt() / i.travel_time_mult) as i32
    };
    TravelOut { price, hours }
}

/// Input of `mw.services.spell_price`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellPriceIn {
    pub terms: BarterTerms,
    /// `calcSpellCost` (`mwmechanics/spellutil.cpp`): the record's cost, or the autocalc cost.
    pub spell_cost: i32,
    /// `fSpellValueMult`.
    pub spell_value_mult: f32,
}

impl SpellPriceIn {
    /// The input with `fSpellValueMult` read from `g`.
    #[must_use]
    pub fn new(terms: BarterTerms, spell_cost: i32, g: &MwGmst) -> Self {
        Self { terms, spell_cost, spell_value_mult: g.spell_value_mult }
    }
}

/// `mw.services.spell_price`: `barter_offer(max(1, int(cost x fSpellValueMult)), buying)`.
#[allow(clippy::cast_precision_loss, reason = "OpenMW multiplies the int cost as float")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn spell_price(i: &SpellPriceIn) -> i32 {
    let base = ((i.spell_cost as f32 * i.spell_value_mult) as i32).max(1);
    barter_offer(&i.terms, base, true)
}

/// Input of `mw.services.repair_price` (one damaged item).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairPriceIn {
    pub terms: BarterTerms,
    /// The record's value (`getValue`).
    pub value: i32,
    /// Current condition (`getItemHealth`).
    pub health: i32,
    /// Maximum condition (`getItemMaxHealth`), non-zero for listed items.
    pub max_health: i32,
    /// `fRepairMult`.
    pub repair_mult: f32,
}

impl RepairPriceIn {
    /// The input with `fRepairMult` read from `g`.
    #[must_use]
    pub fn new(terms: BarterTerms, value: i32, health: i32, max_health: i32, g: &MwGmst) -> Self {
        Self { terms, value, health, max_health, repair_mult: g.repair_mult }
    }
}

/// `mw.services.repair_price` (`merchantrepair.cpp` 75-83): `p = max(1, value)`,
/// `r = max(1, int(max / p))`, `x = max(1, int(fRepairMult x int((max - health) / r)))`, then
/// the buying offer.
#[allow(clippy::cast_precision_loss, reason = "OpenMW divides the int conditions as float")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn repair_price(i: &RepairPriceIn) -> i32 {
    let p = i.value.max(1) as f32;
    let r = ((i.max_health as f32 / p) as i32).max(1) as f32;
    let x = ((i64::from(i.max_health) - i64::from(i.health)) as f32 / r) as i32;
    let x = ((i.repair_mult * x as f32) as i32).max(1);
    barter_offer(&i.terms, x, true)
}
