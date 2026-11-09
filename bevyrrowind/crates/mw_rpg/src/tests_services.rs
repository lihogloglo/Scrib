//! Service prices (services-adapter.md "Tests", slice A1) against values hand-computed from the
//! `OpenMW` formulas (commit ca5e2576, see `services.rs`) with vanilla GMSTs. Every `f32` step
//! below was also replayed with `Math.fround` (IEEE single precision, as `OpenMW` on x86-64 SSE)
//! to make sure no expected value sits on a rounding edge.
//!
//! Fixtures:
//! - `even()`: both sides Mercantile 30, Luck 40, Personality 40, fatigue term 1.25 (full
//!   fatigue), disposition 50. `a = 30, b = 0.1 x 40 = 4, c = 0.2 x 40 = 8`, so
//!   `pcTerm = npcTerm = 42 x 1.25 = 52.5`: `buyTerm = 0.01 x 100 = 1.0`,
//!   `sellTerm = 0.01 x 50 = 0.5` (both round to exactly 1.0 and 0.5 in `f32`).
//! - `skilled()`: player Mercantile 70, Luck 50, Personality 60 (`c = min(12, 10) = 10`),
//!   merchant Mercantile 20, Luck 30, Personality 40, both fatigue terms 1.0, disposition 60.
//!   `pcTerm = (60 - 50 + 70 + 5 + 10) x 1 = 95`, `npcTerm = 20 + 3 + 8 = 31`:
//!   `buyTerm = 0.01 x (100 - 0.5 x 64) = 0.68` (`f32` 0.680000007),
//!   `sellTerm = 0.01 x (50 + 32) = 0.82` (`f32` 0.819999993).

use sf_rpg::service::Side;

use mw_stats::MwGmst;

use crate::{
    BarterPriceIn, BarterTerms, EffectiveValueIn, FollowerPricing, HaggleIn, HaggleOut, PartyTerms, RULE_IDS,
    RepairPriceIn, SpellPriceIn, TrainingIn, TrainingOut, TrainingRefusal, TravelIn, TravelOut, barter_haggle,
    barter_offer, barter_price, effective_value, repair_price, spell_price, training_price, travel_price,
};

fn even() -> BarterTerms {
    let side = PartyTerms { mercantile: 30.0, luck: 40.0, personality: 40.0, fatigue_term: 1.25 };
    BarterTerms { merchant_is_creature: false, disposition: 50, pc: side, npc: side }
}

fn skilled() -> BarterTerms {
    BarterTerms {
        merchant_is_creature: false,
        disposition: 60,
        pc: PartyTerms { mercantile: 70.0, luck: 50.0, personality: 60.0, fatigue_term: 1.0 },
        npc: PartyTerms { mercantile: 20.0, luck: 30.0, personality: 40.0, fatigue_term: 1.0 },
    }
}

fn creature() -> BarterTerms {
    BarterTerms { merchant_is_creature: true, ..skilled() }
}

/// `even()` with the player's Mercantile replaced.
fn with_pc_mercantile(mercantile: f32) -> BarterTerms {
    let mut t = even();
    t.pc.mercantile = mercantile;
    t
}

// ------------------------------------------------------------------ barter offer

#[test]
fn barter_offer_zero_base_and_creature_keep_the_base() {
    assert_eq!(barter_offer(&skilled(), 0, true), 0);
    assert_eq!(barter_offer(&skilled(), 0, false), 0);
    assert_eq!(barter_offer(&creature(), 251, true), 251);
    assert_eq!(barter_offer(&creature(), 251, false), 251);
}

#[test]
fn barter_offer_buy_and_sell_terms() {
    // even: int(100 x 1.0) = 100, int(100 x 0.5) = 50.
    assert_eq!(barter_offer(&even(), 100, true), 100);
    assert_eq!(barter_offer(&even(), 100, false), 50);
    // skilled: int(251 x 0.68) = int(170.68) = 170; int(251 x 0.82) = int(205.82) = 205.
    assert_eq!(barter_offer(&skilled(), 251, true), 170);
    assert_eq!(barter_offer(&skilled(), 251, false), 205);
    // Low term: int(1 x 0.5) = 0, raised to 1.
    assert_eq!(barter_offer(&even(), 1, false), 1);
}

#[test]
fn barter_offer_clamps_mercantile_to_100() {
    // Mercantile 120 and 100 both give pcTerm = (100 + 4 + 8) x 1.25 = 140, npcTerm 52.5:
    // buyTerm = 0.01 x (100 - 43.75) = 0.5625, int(1000 x 0.5625) = 562.
    assert_eq!(barter_offer(&with_pc_mercantile(120.0), 1000, true), 562);
    assert_eq!(barter_offer(&with_pc_mercantile(100.0), 1000, true), 562);
}

#[test]
fn effective_value_scales_by_health_and_count() {
    // int(100 x 50 / 200 x 3) = int(25 x 3) = 75.
    assert_eq!(effective_value(&EffectiveValueIn { value: 100, health: Some((50, 200)), count: 3 }), 75);
    assert_eq!(effective_value(&EffectiveValueIn { value: 100, health: None, count: 3 }), 300);
    // A zero maximum normalises to 0.
    assert_eq!(effective_value(&EffectiveValueIn { value: 100, health: Some((5, 0)), count: 3 }), 0);
    // 7 x (1 / 3 = 0.33333334) = 2.3333335, x 2 = 4.666667, int 4.
    assert_eq!(effective_value(&EffectiveValueIn { value: 7, health: Some((1, 3)), count: 2 }), 4);
}

// ------------------------------------------------------------------ mw.barter.price

fn price(terms: BarterTerms, side: Side, cap_75: bool) -> i32 {
    barter_price(&BarterPriceIn { terms, base: 251, side, cap_75 })
}

#[test]
fn barter_price_caps_at_75_percent() {
    // cap = int(max(1, 0.75 x 251 = 188.25)) = 188.
    // Buy: the offer 170 is below the cap, so the player pays 188.
    assert_eq!(price(skilled(), Side::Buy, true), 188);
    // Sell: the offer 205 is above the cap, so the merchant pays 188.
    assert_eq!(price(skilled(), Side::Sell, true), 188);
    // A creature's sell price (the base, 251) is not capped; its buy price is above the cap.
    assert_eq!(price(creature(), Side::Sell, true), 251);
    assert_eq!(price(creature(), Side::Buy, true), 251);
}

#[test]
fn barter_price_without_cap_is_the_offer() {
    assert_eq!(price(skilled(), Side::Buy, false), barter_offer(&skilled(), 251, true));
    assert_eq!(price(skilled(), Side::Sell, false), barter_offer(&skilled(), 251, false));
    assert_eq!(price(skilled(), Side::Buy, false), 170);
    assert_eq!(price(skilled(), Side::Sell, false), 205);
}

#[test]
fn barter_price_zero_base_buys_for_one_under_the_cap() {
    // OpenMW: cap = int(max(1, 0)) = 1, offer 0, max(1, 0) = 1.
    let i = BarterPriceIn { terms: even(), base: 0, side: Side::Buy, cap_75: true };
    assert_eq!(barter_price(&i), 1);
    assert_eq!(barter_price(&BarterPriceIn { cap_75: false, ..i }), 0);
}

// ------------------------------------------------------------------ mw.barter.haggle

fn haggle(terms: BarterTerms, player_offer: i32, merchant_offer: i32, roll: i32) -> HaggleOut {
    barter_haggle(&HaggleIn::new(terms, player_offer, merchant_offer, roll, &MwGmst::vanilla()))
}

fn out(accepted: bool, mercantile_scale: f32, disposition_delta: i32) -> HaggleOut {
    HaggleOut { accepted, mercantile_scale, disposition_delta }
}

#[test]
fn haggle_accepts_the_offered_price_without_skill_use() {
    assert_eq!(haggle(even(), -100, -100, 100), out(true, 0.0, 1));
    assert_eq!(haggle(even(), 40, 50, 100), out(true, 0.0, 1));
    // Creatures too, with no disposition change.
    assert_eq!(haggle(creature(), -100, -100, 100), out(true, 0.0, 0));
}

#[test]
fn haggle_creature_rejects_a_better_ask() {
    assert_eq!(haggle(creature(), -90, -100, 1), out(false, 0.0, 0));
}

#[test]
fn haggle_integer_d_and_roll_edge() {
    // Buying: a = 101, b = 90, d = int(100 x 11 / 101) = int(10.89) = 10. even: pcTerm = npcTerm
    // = 52.5, so x = -4 x 10 + 50 + int(0) = 10. Roll 10 accepts, 11 rejects.
    // Scale: floor(100 x 11 / 101) = floor(10.89) = 10.
    assert_eq!(haggle(even(), -90, -101, 10), out(true, 10.0, 1));
    assert_eq!(haggle(even(), -90, -101, 11), out(false, 0.0, -1));
}

#[test]
fn haggle_selling_scale_floors() {
    // Selling: a = 100, b = 107, d = int(700 / 107) = 6, x = -24 + 50 + 0 = 26.
    // Scale: floor(100 x 7 / 107) = floor(6.54) = 6.
    assert_eq!(haggle(even(), 107, 100, 26), out(true, 6.0, 1));
    assert_eq!(haggle(even(), 107, 100, 27), out(false, 0.0, -1));
    // A tiny discount floors to 0: m = -1000, p = -999, floor(100 x 1 / 1000) = 0 (Mercantile
    // 400 so the roll passes: x = -4 x 0 + 50 + int((400 + 12) x 1.25 - 52.5 = 462.5) = 512).
    assert_eq!(haggle(with_pc_mercantile(400.0), -999, -1000, 1), out(true, 0.0, 1));
}

#[test]
fn haggle_terms_are_not_clamped() {
    // Mercantile 120 unclamped: pcTerm = (0 + 120 + 4 + 8) x 1.25 = 165, int(165 - 52.5) = 112,
    // x = -40 + 50 + 112 = 122: roll 100 accepts. Clamped to 100 it would be int(87.5) = 87,
    // x = 97: roll 100 rejects, as Mercantile 100 does.
    assert_eq!(haggle(with_pc_mercantile(120.0), -90, -101, 100), out(true, 10.0, 1));
    assert_eq!(haggle(with_pc_mercantile(100.0), -90, -101, 100), out(false, 0.0, -1));
}

#[test]
fn haggle_buying_player_cannot_ask_to_be_paid() {
    // m = -50. Mercantile 400: int((412 x 1.25 = 515) - 52.5) = 462. Both asks give d = 80
    // (|-10| and |10| are 10: int(100 x 40 / 50)), x = -320 + 50 + 462 = 192, so roll 1 passes.
    // Asking +10 (to be paid while buying) is still refused; paying 10 is accepted with
    // scale floor(100 x 40 / 50) = 80.
    assert_eq!(haggle(with_pc_mercantile(400.0), 10, -50, 1), out(false, 0.0, -1));
    assert_eq!(haggle(with_pc_mercantile(400.0), -10, -50, 1), out(true, 80.0, 1));
}

#[test]
fn haggle_reads_disposition_mod_and_deltas_from_gmsts() {
    let g = MwGmst {
        disposition_mod: 0.5,
        barter_success_disposition: 3.0,
        barter_fail_disposition: -2.0,
        ..MwGmst::vanilla()
    };
    // Disposition 70: term 0.5 x 20 = 10, pcTerm = (10 + 42) x 1.25 = 65, int(65 - 52.5) = 12.
    // Selling 107 for 100: x = -24 + 50 + 12 = 38.
    let terms = BarterTerms { disposition: 70, ..even() };
    let run = |roll| barter_haggle(&HaggleIn::new(terms, 107, 100, roll, &g));
    assert_eq!(run(38), out(true, 6.0, 3));
    assert_eq!(run(39), out(false, 0.0, -2));
}

#[test]
fn haggle_input_reads_vanilla_gmsts() {
    let i = HaggleIn::new(even(), 0, 0, 1, &MwGmst::vanilla());
    assert_eq!((i.success_disposition, i.fail_disposition), (1, -1));
    assert!((i.bargain_offer_base - 50.0).abs() < f32::EPSILON);
    assert!((i.bargain_offer_multi + 4.0).abs() < f32::EPSILON);
    assert!((i.disposition_mod - 1.0).abs() < f32::EPSILON);
}

// ------------------------------------------------------------------ mw.services.training

fn train(terms: BarterTerms, pc: f32, trainer: f32, attr: f32) -> TrainingOut {
    training_price(&TrainingIn::new(terms, pc, trainer, attr, &MwGmst::vanilla()))
}

#[test]
fn training_price_and_refusals() {
    // 40 x iTrainingMod 10 = 400; even buys at 1.0.
    assert_eq!(train(even(), 40.0, 41.0, 60.0), TrainingOut { price: 400, refusal: None });
    // skilled: int(400 x 0.680000007 = 272.0000029) = 272.
    assert_eq!(train(skilled(), 40.0, 41.0, 60.0).price, 272);
    // Skill 0: max(1, 0) = 1.
    assert_eq!(train(even(), 0.0, 41.0, 60.0).price, 1);
    // The trainer must be strictly better.
    assert_eq!(train(even(), 40.0, 40.0, 60.0).refusal, Some(TrainingRefusal::TrainerNotBetter));
    // The governing attribute caps the base skill.
    assert_eq!(train(even(), 40.0, 41.0, 40.0).refusal, Some(TrainingRefusal::AttributeCap));
    // Both: the trainer check comes first.
    assert_eq!(train(even(), 40.0, 40.0, 40.0).refusal, Some(TrainingRefusal::TrainerNotBetter));
    assert_eq!(TrainingRefusal::TrainerNotBetter.message_gmst(), "sServiceTrainingWords");
    assert_eq!(TrainingRefusal::AttributeCap.message_gmst(), "sNotifyMessage17");
}

// ------------------------------------------------------------------ mw.services.travel

fn travel(terms: BarterTerms, interior: bool, dest: [f32; 3], followers: u32, rule: FollowerPricing) -> TravelOut {
    travel_price(&TravelIn::new(terms, interior, [0.0; 3], dest, followers, rule, &MwGmst::vanilla()))
}

const FAR: [f32; 3] = [12000.0, 16000.0, 0.0];

#[test]
fn travel_exterior_price_from_3d_hours_from_2d() {
    // 3D distance 20000: int(20000 / 4000) = 5; 2D 20000: int(20000 / 16000 = 1.25) = 1.
    assert_eq!(travel(even(), false, FAR, 0, FollowerPricing::OpenMw), TravelOut { price: 5, hours: 1 });
    // Straight up 8000: int(2) = 2 gold, 2D distance 0: no time.
    assert_eq!(travel(even(), false, [0.0, 0.0, 8000.0], 0, FollowerPricing::OpenMw), TravelOut { price: 2, hours: 0 });
    // Distance 100: int(0.025) = 0, raised to 1.
    assert_eq!(travel(even(), false, [60.0, 80.0, 0.0], 0, FollowerPricing::OpenMw), TravelOut { price: 1, hours: 0 });
}

#[test]
fn travel_follower_pricing_both_ways() {
    // OpenMW: 5 x (1 + followers); vanilla: 5 x max(1, followers).
    assert_eq!(travel(even(), false, FAR, 1, FollowerPricing::OpenMw).price, 10);
    assert_eq!(travel(even(), false, FAR, 2, FollowerPricing::OpenMw).price, 15);
    assert_eq!(travel(even(), false, FAR, 1, FollowerPricing::Vanilla).price, 5);
    assert_eq!(travel(even(), false, FAR, 2, FollowerPricing::Vanilla).price, 10);
}

#[test]
fn travel_interior_provider_flat_price_no_time() {
    // fMagesGuildTravel 10 x (1 + 3) = 40, whatever the distance; 0 hours.
    assert_eq!(travel(even(), true, FAR, 3, FollowerPricing::OpenMw), TravelOut { price: 40, hours: 0 });
    // skilled: int(10 x 0.68) = 6.
    assert_eq!(travel(skilled(), true, [1.0; 3], 0, FollowerPricing::OpenMw), TravelOut { price: 6, hours: 0 });
}

#[test]
fn travel_zero_mult_uses_the_distance() {
    // Player at (100, 200, 0), destination 20000 away: fTravelMult 0 means int(d) = 20000.
    let i = TravelIn {
        player_pos: [100.0, 200.0, 0.0],
        dest_pos: [12100.0, 16200.0, 0.0],
        travel_mult: 0.0,
        ..TravelIn::new(even(), false, [0.0; 3], [0.0; 3], 0, FollowerPricing::OpenMw, &MwGmst::vanilla())
    };
    assert_eq!(travel_price(&i), TravelOut { price: 20000, hours: 1 });
}

// ------------------------------------------------------------------ mw.services.spell_price

#[test]
fn spell_price_from_cost() {
    let g = MwGmst::vanilla();
    // 7 x fSpellValueMult 10 = 70; skilled int(70 x 0.68 = 47.6) = 47; cost 0 gives max(1, 0) = 1.
    assert_eq!(spell_price(&SpellPriceIn::new(even(), 7, &g)), 70);
    assert_eq!(spell_price(&SpellPriceIn::new(skilled(), 7, &g)), 47);
    assert_eq!(spell_price(&SpellPriceIn::new(even(), 0, &g)), 1);
}

// ------------------------------------------------------------------ mw.services.repair_price

#[test]
fn repair_price_hand_computed() {
    let g = MwGmst::vanilla();
    let repair = |terms, value, health, max| repair_price(&RepairPriceIn::new(terms, value, health, max, &g));
    // p = 100, r = max(1, int(400 / 100)) = 4, x = int(350 / 4 = 87.5) = 87, x fRepairMult 1.
    assert_eq!(repair(even(), 100, 50, 400), 87);
    // skilled: int(87 x 0.68 = 59.16) = 59.
    assert_eq!(repair(skilled(), 100, 50, 400), 59);
    // Value above max: r = max(1, int(0.4)) = 1, x = 350.
    assert_eq!(repair(even(), 1000, 50, 400), 350);
    // Value 0: p = 1, r = 400, x = int(350 / 400) = 0, raised to 1.
    assert_eq!(repair(even(), 0, 50, 400), 1);
    // fRepairMult 2: int(2 x 87) = 174.
    let doubled = RepairPriceIn { repair_mult: 2.0, ..RepairPriceIn::new(even(), 100, 50, 400, &g) };
    assert_eq!(repair_price(&doubled), 174);
}

// ------------------------------------------------------------------ GMSTs and ids

#[test]
fn service_gmsts_vanilla_values() {
    let g = MwGmst::vanilla();
    let expected = [
        ("fBargainOfferBase", 50.0),
        ("fBargainOfferMulti", -4.0),
        ("fDispositionMod", 1.0),
        ("iBarterSuccessDisposition", 1.0),
        ("iBarterFailDisposition", -1.0),
        ("fBarterGoldResetDelay", 24.0),
        ("iTrainingMod", 10.0),
        ("fTravelMult", 4000.0),
        ("fTravelTimeMult", 16000.0),
        ("fMagesGuildTravel", 10.0),
        ("fSpellValueMult", 10.0),
        ("fRepairMult", 1.0),
        ("fFatigueBase", 1.25),
        ("fFatigueMult", 0.5),
    ];
    let entries = g.entries();
    for (name, value) in expected {
        let got = entries.iter().find(|(n, _)| n == name).map(|(_, v)| *v);
        assert_eq!(got.map(f32::to_bits), Some(f32::to_bits(value)), "{name}");
    }
}

#[test]
fn party_terms_read_the_fatigue_term() {
    // fFatigueBase 1.25 - fFatigueMult 0.5 x (1 - 50 / 200) = 0.875.
    let p = PartyTerms::new(30.0, 40.0, 40.0, 50.0, 200.0, &MwGmst::vanilla());
    assert_eq!(p.fatigue_term.to_bits(), 0.875_f32.to_bits());
}

#[test]
fn rule_ids_are_unique() {
    let mut ids = RULE_IDS.to_vec();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 6);
}
