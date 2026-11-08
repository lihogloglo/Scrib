//! Derived disposition and persuasion (ROADMAP 2.7) against hand-computed values from the
//! `OpenMW` formulas and vanilla GMSTs (`MwGmst::vanilla`). Synthetic data only.

use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, PlayerId};
use sf_stats::{Inputs, Sheet, StatOp, StatOwner};
use tes3::esp::{Faction, FactionReaction};

use mw_stats::{MwGmst, MwTable, SkillInfo, openmw_rules};

use crate::{
    ActorFacts, AiNudge, DispositionExtras, DispositionInput, MwSheets, PersuaderStats, PersuasionInput,
    PersuasionKind, PersuasionOutcome, PersuasionRatings, PlayerFaction, derived_disposition,
    derived_disposition_unclamped, disposition_input, faction_reaction, faction_standing, persuasion_change,
    persuasion_input, persuasion_ratings, session_perm_change,
};

const SPEAKER: ActorKey = ActorKey(7);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

// ------------------------------------------------------------------ derived disposition

/// Base 50, Personality 50, nothing else: exactly 50.
fn neutral() -> DispositionInput {
    DispositionInput { base: 50, personality: 50.0, ..DispositionInput::default() }
}

fn disp(i: &DispositionInput) -> i32 {
    derived_disposition(i, &MwGmst::vanilla())
}

#[test]
fn derived_disposition_terms() {
    assert_eq!(disp(&neutral()), 50);
    // Permanent changes and the crime modifier add to the base: 50 - 20 + 5.
    assert_eq!(disp(&DispositionInput { modifier: -20, crime_modifier: 5, ..neutral() }), 35);
    // fDispRaceMod 5.
    assert_eq!(disp(&DispositionInput { same_race: true, ..neutral() }), 55);
    // fDispPersonalityMult 0.5 x (Per - fDispPersonalityBase 50): +10, -9.5 (40.5), -10.5 (39.5).
    assert_eq!(disp(&DispositionInput { personality: 70.0, ..neutral() }), 60);
    assert_eq!(disp(&DispositionInput { personality: 31.0, ..neutral() }), 40);
    assert_eq!(disp(&DispositionInput { personality: 29.0, ..neutral() }), 39);
    // (fDispFactionRankMult 0.5 x rank + fDispFactionRankBase 1) x fDispFactionMod 3 x reaction.
    assert_eq!(disp(&DispositionInput { faction_rank: 2, faction_reaction: 3, ..neutral() }), 68);
    assert_eq!(disp(&DispositionInput { faction_rank: 0, faction_reaction: -2, ..neutral() }), 44);
    // fDispDiseaseMod -10, fDispWeaponDrawn -5, Charm magnitude.
    assert_eq!(disp(&DispositionInput { diseased: true, ..neutral() }), 40);
    assert_eq!(disp(&DispositionInput { weapon_drawn: true, ..neutral() }), 45);
    assert_eq!(disp(&DispositionInput { charm: 7.5, ..neutral() }), 57);
}

#[test]
fn derived_disposition_crime_term() {
    // Vanilla fDispCrimeMod is 0: a bounty changes nothing.
    assert_eq!(disp(&DispositionInput { bounty: 1000, ..neutral() }), 50);
    // A mod's fDispCrimeMod 0.25 x bounty 40 = -10.
    let g = MwGmst { disp_crime_mod: 0.25, ..MwGmst::vanilla() };
    assert_eq!(derived_disposition(&DispositionInput { bounty: 40, ..neutral() }, &g), 40);
}

#[test]
fn derived_disposition_clamps_and_truncates() {
    let g = MwGmst::vanilla();
    let high = DispositionInput { base: 120, ..neutral() };
    assert_eq!(derived_disposition_unclamped(&high, &g), 120);
    assert_eq!(derived_disposition(&high, &g), 100);
    let low = DispositionInput { base: 5, diseased: true, weapon_drawn: true, ..neutral() };
    assert_eq!(derived_disposition_unclamped(&low, &g), -10);
    assert_eq!(derived_disposition(&low, &g), 0);
    // -0.5 truncates toward zero (static_cast<int>), not down.
    let half = DispositionInput { base: 0, personality: 49.0, ..neutral() };
    assert_eq!(derived_disposition_unclamped(&half, &g), 0);
}

#[test]
fn faction_standing_follows_openmw() {
    let (a, b, c) = (id("a"), id("b"), id("c"));
    let reaction = |from: &RecordId, to: &RecordId| match (from.as_str(), to.as_str()) {
        ("a", "a") => 2,
        ("a", "b") => 3,
        ("a", "c") => -1,
        _ => 0,
    };
    let member = |id, rank| PlayerFaction { id, rank, expelled: false };
    let expelled = |id, rank| PlayerFaction { id, rank, expelled: true };
    // No speaker faction; a player without factions.
    assert_eq!(faction_standing(None, &[member(&b, 1)], reaction), (0, 0));
    assert_eq!(faction_standing(Some(&a), &[], reaction), (0, 0));
    // A member: the faction's reaction to itself and the player's rank; expelled: nothing.
    assert_eq!(faction_standing(Some(&a), &[member(&a, 4), member(&b, 1)], reaction), (4, 2));
    assert_eq!(faction_standing(Some(&a), &[expelled(&a, 4), member(&b, 1)], reaction), (0, 0));
    // Not a member: the lowest reaction among the player's factions, with that rank.
    assert_eq!(faction_standing(Some(&a), &[member(&b, 1), member(&c, 5)], reaction), (5, -1));
    assert_eq!(faction_standing(Some(&a), &[member(&b, 1)], reaction), (1, 3));
    // A tie keeps the first in id order.
    let even = |_: &RecordId, _: &RecordId| 2;
    assert_eq!(faction_standing(Some(&a), &[member(&b, 1), member(&c, 6)], even), (1, 2));
    // OpenMW quirk: the first faction expelled leaves the start value 0, so a positive reaction
    // is never taken, a negative one is.
    assert_eq!(faction_standing(Some(&a), &[expelled(&b, 0), member(&c, 3)], |_, _| 4), (0, 0));
    assert_eq!(faction_standing(Some(&a), &[expelled(&b, 0), member(&c, 3)], reaction), (3, -1));
}

fn faction_db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Faction>("FACT").unwrap();
    let src = db.add_source("test", &[]);
    let thieves = Faction {
        id: "Thieves Guild".into(),
        reactions: vec![
            FactionReaction { faction: "Fighters Guild".into(), reaction: -2 },
            FactionReaction { faction: "Mages Guild".into(), reaction: 5 },
        ],
        ..Default::default()
    };
    db.define(src, id("Thieves Guild"), thieves).unwrap();
    db
}

#[test]
fn faction_reaction_reads_the_record() {
    let db = faction_db();
    assert_eq!(faction_reaction(&db, &id("thieves guild"), &id("fighters guild")), -2);
    assert_eq!(faction_reaction(&db, &id("thieves guild"), &id("mages guild")), 5);
    assert_eq!(faction_reaction(&db, &id("thieves guild"), &id("thieves guild")), 0);
    assert_eq!(faction_reaction(&db, &id("no such faction"), &id("mages guild")), 0);
}

// ------------------------------------------------------------------ sheets

fn table() -> MwTable {
    MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
        governing: 0,
        specialization: 0,
        use_values: [1.0, 0.0, 0.0, 0.0],
    }))
    .unwrap()
}

fn sheet(t: &MwTable, owner: StatOwner, ops: &[StatOp]) -> Sheet {
    let rules = openmw_rules(t, &MwGmst::vanilla());
    let inputs = Inputs { is_player: matches!(owner, StatOwner::Player(_)), ..Inputs::default() };
    let (sheet, rejected) = mw_stats::new_sheet(t, &rules, owner, &inputs, ops);
    assert_eq!(rejected, 0);
    sheet
}

/// Personality 70, Luck 40, Mercantile 20, Speechcraft 30, level 2, reputation 6, bounty 40,
/// Fighters Guild rank 2, expelled from the Mages Guild.
fn player_sheet(t: &MwTable) -> Sheet {
    let idx = &t.idx;
    sheet(t, StatOwner::Player(sf_stats::PlayerKey(1)), &[
        StatOp::SetBase { stat: idx.attributes[6], value: 70.0 },
        StatOp::SetBase { stat: idx.attributes[7], value: 40.0 },
        StatOp::SetBase { stat: idx.skills[24], value: 20.0 },
        StatOp::SetBase { stat: idx.skills[25], value: 30.0 },
        StatOp::CounterSet { stat: idx.level, value: 2.0 },
        StatOp::CounterSet { stat: idx.reputation, value: 6.0 },
        StatOp::CounterSet { stat: idx.bounty, value: 40.0 },
        StatOp::SetRank { group: id("fighters guild"), rank: 2 },
        StatOp::SetRank { group: id("mages guild"), rank: 0 },
        StatOp::Expel { group: id("mages guild") },
    ])
}

/// Personality 40, Luck 50, Speechcraft 25, level 4.
fn npc_sheet(t: &MwTable) -> Sheet {
    let idx = &t.idx;
    sheet(t, StatOwner::Actor(SPEAKER), &[
        StatOp::SetBase { stat: idx.attributes[6], value: 40.0 },
        StatOp::SetBase { stat: idx.attributes[7], value: 50.0 },
        StatOp::SetBase { stat: idx.skills[25], value: 25.0 },
        StatOp::CounterSet { stat: idx.level, value: 4.0 },
    ])
}

struct Sheets {
    table: MwTable,
    host: Sheet,
    npc: Sheet,
}

impl Sheets {
    fn new() -> Self {
        let table = table();
        let host = player_sheet(&table);
        let npc = npc_sheet(&table);
        Self { table, host, npc }
    }
}

impl MwSheets for Sheets {
    fn table(&self) -> Option<&MwTable> {
        Some(&self.table)
    }

    fn player(&self, player: PlayerId) -> Option<&Sheet> {
        (player == PlayerId::HOST).then_some(&self.host)
    }

    fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
        (actor == SPEAKER).then_some(&self.npc)
    }
}

fn thief() -> ActorFacts {
    ActorFacts {
        id: id("thief"),
        npc: true,
        race: Some(id("dark elf")),
        faction: Some(id("thieves guild")),
        rank: 0,
        ..ActorFacts::default()
    }
}

#[test]
fn disposition_input_from_sheets_and_records() {
    let sheets = Sheets::new();
    let db = faction_db();
    let extras = DispositionExtras { base: 40, modifier: 5, ..DispositionExtras::default() };
    let race = id("dark elf");
    let i = disposition_input(&sheets, &db, PlayerId::HOST, Some(&race), &thief(), extras).unwrap();
    // Fighters Guild rank 2 and the Thieves Guild's -2 towards it; the expelled Mages Guild
    // (+5) is skipped.
    assert_eq!(
        i,
        DispositionInput {
            base: 40,
            modifier: 5,
            same_race: true,
            personality: 70.0,
            faction_rank: 2,
            faction_reaction: -2,
            bounty: 40,
            ..DispositionInput::default()
        }
    );
    // 45 + 5 (race) + 10 (personality) - 12 (faction) - 0 (vanilla crime mod).
    assert_eq!(disp(&i), 48);
    // Neutral without stats or for a creature.
    assert!(disposition_input(&crate::NoSheets, &db, PlayerId::HOST, Some(&race), &thief(), extras).is_none());
    let creature = ActorFacts { npc: false, ..thief() };
    assert!(disposition_input(&sheets, &db, PlayerId::HOST, Some(&race), &creature, extras).is_none());
}

#[test]
fn persuasion_input_from_sheets() {
    let sheets = Sheets::new();
    let ai = AiNudge { flee: 20, fight: 30 };
    let i = persuasion_input(&sheets, PlayerId::HOST, SPEAKER, PersuasionKind::Taunt, 50, ai, 17).unwrap();
    let t = &sheets.table;
    let (pv, nv) = (sheets.host.view(&t.table), sheets.npc.view(&t.table));
    assert_eq!(
        i.player,
        PersuaderStats {
            personality: 70.0,
            luck: 40.0,
            speechcraft: 30.0,
            mercantile: 20.0,
            reputation: 6,
            level: 2,
            fatigue_current: pv.current(t.idx.fatigue),
            fatigue_max: pv.max(t.idx.fatigue),
        }
    );
    assert_eq!((i.npc.personality, i.npc.luck, i.npc.speechcraft, i.npc.level), (40.0, 50.0, 25.0, 4));
    assert_eq!((i.npc.fatigue_current, i.npc.fatigue_max), (nv.current(t.idx.fatigue), nv.max(t.idx.fatigue)));
    assert_eq!((i.kind, i.disposition, i.npc_ai, i.roll), (PersuasionKind::Taunt, 50, ai, 17));
    assert!(persuasion_input(&sheets, PlayerId::HOST, ActorKey(8), PersuasionKind::Taunt, 50, ai, 17).is_none());
}

// ------------------------------------------------------------------ persuasion

/// Full fatigue (term 1.25). Ratings: speech (6 + 4 + 10 + 30) x 1.25 = 62.5, level
/// 62.5 + 10 = 72.5, mercantile (20 + 4 + 10) x 1.25 = 42.5.
fn player() -> PersuaderStats {
    PersuaderStats {
        personality: 50.0,
        luck: 40.0,
        speechcraft: 30.0,
        mercantile: 20.0,
        reputation: 6,
        level: 2,
        fatigue_current: 100.0,
        fatigue_max: 100.0,
    }
}

/// Ratings: speech (2 + 5 + 8 + 25) x 1.25 = 50, level (20 + 2 + 5 + 8 + 25) x 1.25 = 75,
/// mercantile (15 + 2 + 5 + 8) x 1.25 = 37.5.
fn npc() -> PersuaderStats {
    PersuaderStats {
        personality: 40.0,
        luck: 50.0,
        speechcraft: 25.0,
        mercantile: 15.0,
        reputation: 2,
        level: 4,
        fatigue_current: 100.0,
        fatigue_max: 100.0,
    }
}

const AI: AiNudge = AiNudge { flee: 20, fight: 30 };

/// At disposition 50 (d = 1) the targets are: Admire and Taunt 62.5, Intimidate 47.5, bribes
/// 55 + 35 / 75 / 150.
fn persuade(kind: PersuasionKind, disposition: i32, roll: u8) -> PersuasionOutcome {
    let i = PersuasionInput { kind, player: player(), npc: npc(), disposition, npc_ai: AI, roll };
    persuasion_change(&i, &MwGmst::vanilla())
}

fn outcome(success: bool, temp_change: i32, perm_change: i32, ai: Option<AiNudge>, gold: i32) -> PersuasionOutcome {
    PersuasionOutcome { success, temp_change, perm_change, ai, gold }
}

#[test]
fn persuasion_ratings_per_side() {
    let g = MwGmst::vanilla();
    assert_eq!(persuasion_ratings(&player(), true, &g), PersuasionRatings { speech: 62.5, level: 72.5, mercantile: 42.5 });
    assert_eq!(persuasion_ratings(&npc(), false, &g), PersuasionRatings { speech: 50.0, level: 75.0, mercantile: 37.5 });
    // Half fatigue: 1.25 - 0.5 x (1 - 0.5) = 1.
    let tired = PersuaderStats { fatigue_current: 50.0, ..player() };
    assert_eq!(persuasion_ratings(&tired, true, &g), PersuasionRatings { speech: 50.0, level: 60.0, mercantile: 34.0 });
}

#[test]
fn admire() {
    use PersuasionKind::Admire;
    // floor(0.3 x 22.5) = 6, raised to iPerMinChange 10.
    assert_eq!(persuade(Admire, 50, 40), outcome(true, 10, 10, None, 0));
    // floor(0.3 x 62.5) = 18.
    assert_eq!(persuade(Admire, 50, 0), outcome(true, 18, 18, None, 0));
    // Fail: floor(0.3 x -17.5) = -6.
    assert_eq!(persuade(Admire, 50, 80), outcome(false, -6, -6, None, 0));
    // The edge: 62 <= 62.5 succeeds, 63 fails with floor(0.3 x -0.5) = -1.
    assert!(persuade(Admire, 50, 62).success);
    assert_eq!(persuade(Admire, 50, 63), outcome(false, -1, -1, None, 0));
}

#[test]
fn intimidate() {
    use PersuasionKind::Intimidate;
    // r = floor(47.5 - 10) = 37, s = floor(37 x 0.3) = 11: Flee +11, Fight -11; c = -11.
    assert_eq!(persuade(Intimidate, 50, 10), outcome(true, 11, -11, Some(AiNudge { flee: 31, fight: 19 }), 0));
    // Marginal win (OpenMW's deviation): r = 7, |c| = 2 < 10, so +10 now, -10 at goodbye;
    // the AI nudge is at least iPerMinChange.
    assert_eq!(persuade(Intimidate, 50, 40), outcome(true, 10, -10, Some(AiNudge { flee: 30, fight: 20 }), 0));
    // Fail: r = floor(-12.5) = -13, c = -|floor(-3.9)| = -4.
    assert_eq!(persuade(Intimidate, 50, 60), outcome(false, -4, -4, None, 0));
    // The AI settings clamp to 0..=100.
    let i = PersuasionInput {
        kind: Intimidate,
        player: player(),
        npc: npc(),
        disposition: 50,
        npc_ai: AiNudge { flee: 95, fight: 5 },
        roll: 10,
    };
    assert_eq!(persuasion_change(&i, &MwGmst::vanilla()).ai, Some(AiNudge { flee: 100, fight: 0 }));
}

#[test]
fn intimidate_at_the_minimum_chance() {
    use PersuasionKind::Intimidate;
    // Disposition 0: d = 0, every target is iPerMinChance 5. A roll equal to it uses r = 1.
    assert_eq!(persuade(Intimidate, 0, 5), outcome(true, 10, -10, Some(AiNudge { flee: 30, fight: 20 }), 0));
    // Fail by 1: c = -1; the temporary change is kept at 0, the permanent one is y = -1.
    assert_eq!(persuade(Intimidate, 0, 6), outcome(false, 0, -1, None, 0));
}

#[test]
fn taunt() {
    use PersuasionKind::Taunt;
    // c = 22, x = floor(-6.6) = -7, raised to -10; s = 6.6: Flee min(-10, -6), Fight max(10, 6).
    assert_eq!(persuade(Taunt, 50, 40), outcome(true, -10, -10, Some(AiNudge { flee: 10, fight: 40 }), 0));
    // c = 62, x = floor(-18.6) = -19; s = 18.6: Flee -18, Fight +18.
    assert_eq!(persuade(Taunt, 50, 0), outcome(true, -19, -19, Some(AiNudge { flee: 2, fight: 48 }), 0));
    // Fail: c = |floor(-17.5)| = 18, x = floor(-5.4) = -6, no minimum, no nudge.
    assert_eq!(persuade(Taunt, 50, 80), outcome(false, -6, -6, None, 0));
}

#[test]
fn bribes() {
    use PersuasionKind::{Bribe10, Bribe100, Bribe1000};
    // Targets 90, 130, 205.
    assert_eq!(persuade(Bribe10, 50, 50), outcome(true, 12, 12, None, 10));
    assert_eq!(persuade(Bribe10, 50, 95), outcome(false, -2, -2, None, 0));
    // floor(31 x 0.3) = 9, raised to 10.
    assert_eq!(persuade(Bribe100, 50, 99), outcome(true, 10, 10, None, 100));
    // floor(106 x 0.3) = 31.
    assert_eq!(persuade(Bribe1000, 50, 99), outcome(true, 31, 31, None, 1000));
    assert_eq!([Bribe10, Bribe100, Bribe1000].map(PersuasionKind::gold), [10, 100, 1000]);
    assert_eq!(PersuasionKind::Admire.gold(), 0);
}

#[test]
fn changes_keep_the_disposition_in_range() {
    use PersuasionKind::{Admire, Bribe1000};
    // Disposition 100 (d = 0, target 5): +10 is cut to 0.
    assert_eq!(persuade(Admire, 100, 0), outcome(true, 0, 0, None, 0));
    // Disposition 0: floor(0.3 x (5 - 99)) = -29 is cut to 0.
    assert_eq!(persuade(Admire, 0, 99), outcome(false, 0, 0, None, 0));
    // Disposition 75 (d = 0.5, target 0.5 x 55 + 150 = 177.5): floor(53.25) = 53 is cut to 25.
    assert_eq!(persuade(Bribe1000, 75, 0), outcome(true, 25, 25, None, 1000));
}

#[test]
fn topics_and_session_correction() {
    use PersuasionKind as K;
    assert_eq!(K::Admire.topic(true), "Admire Success");
    assert_eq!(K::Intimidate.topic(false), "Intimidate Fail");
    assert_eq!(K::Taunt.topic(true), "Taunt Success");
    assert_eq!(K::Bribe100.topic(false), "Bribe Fail");
    // A positive attempt never takes the session's permanent total below zero.
    assert_eq!(session_perm_change(10, 10, -30, 5), 25);
    assert_eq!(session_perm_change(10, 10, 50, 0), 10);
    assert_eq!(session_perm_change(-5, 10, -30, 0), 10);
}
