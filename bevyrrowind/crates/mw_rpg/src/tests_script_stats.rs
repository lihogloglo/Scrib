//! Stat, faction and crime functions in result scripts against test sheets (stats.md S7): the
//! ported `OpenMW` semantics per op (`script_stats.rs` module docs), reads that see the run's own
//! writes, and the ops handed to the host in call order. Synthetic scripts and sheets.

use std::collections::HashMap;

use mw_stats::{MwGmst, MwTable, SkillInfo, openmw_rules};
use sf_records::RecordId;
use sf_rpg::{ActorKey, CharacterRpg, ConversationCtx, PlayerId, WorldRpg};
use sf_stats::{ApplyCtx, Cause, CauseKind, Inputs, PlayerKey, Rules, Sheet, StatOp, StatOwner};

use crate::script_stats::mod_stat;
use crate::{
    ActorFacts, GlobalNumber, MwResults, MwSheets, MwWorld, ResultCall, ResultEffect, ResultRunner,
    ResultState, ScriptVar, Target, VarType,
};

const FIGHTERS: &str = "fighters guild";
const SPEAKER: ActorKey = ActorKey(1);
const RAT: ActorKey = ActorKey(9);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn table() -> MwTable {
    MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
        governing: 0,
        specialization: 0,
        use_values: [1.0, 0.0, 0.0, 0.0],
    }))
    .unwrap()
}

fn sheet(t: &MwTable, rules: &Rules, owner: StatOwner, is_player: bool, ops: &[StatOp]) -> Sheet {
    let inputs = Inputs { is_player, ..Inputs::default() };
    let (sheet, rejected) = mw_stats::new_sheet(t, rules, owner, &inputs, ops);
    assert_eq!(rejected, 0);
    sheet
}

/// Player: Strength 50, Long Blade 40, health 60 / 80, Fighters Guild rank 3, bounty 0.
/// Speaker (an NPC of the Fighters Guild at record rank 1, no rank on its sheet): Strength 40,
/// level 5. Rat (a creature): Strength 30.
struct Sheets {
    table: MwTable,
    rules: Rules,
    player: Sheet,
    speaker: Sheet,
    rat: Sheet,
}

impl Sheets {
    fn new() -> Self {
        let t = table();
        let rules = openmw_rules(&t, &MwGmst::vanilla());
        let i = &t.idx;
        let owner = StatOwner::Player(PlayerKey(1));
        let mut player = sheet(&t, &rules, owner, true, &[
            StatOp::SetBase { stat: i.attributes[0], value: 50.0 },
            StatOp::SetBase { stat: i.skills[5], value: 40.0 },
            StatOp::SetBase { stat: i.health, value: 80.0 },
            StatOp::SetRank { group: id(FIGHTERS), rank: 3 },
        ]);
        let hurt = [StatOp::PoolAdjust { pool: i.health, delta: -20.0, below_zero: false, above_max: false }];
        let inputs = Inputs { is_player: true, ..Inputs::default() };
        let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Combat, source: None, tick: 0 }, inputs: &inputs };
        assert_eq!(sf_stats::apply(&t.table, &rules, &mut player, &hurt, &ctx, &mut Vec::new()), 0);
        let speaker = sheet(&t, &rules, StatOwner::Actor(SPEAKER), false, &[
            StatOp::SetBase { stat: i.attributes[0], value: 40.0 },
            StatOp::CounterSet { stat: i.level, value: 5.0 },
        ]);
        let rat = sheet(&t, &rules, StatOwner::Actor(RAT), false, &[
            StatOp::CounterSet { stat: i.creature, value: 1.0 },
            StatOp::SetBase { stat: i.attributes[0], value: 30.0 },
        ]);
        Self { table: t, rules, player, speaker, rat }
    }
}

impl MwSheets for Sheets {
    fn table(&self) -> Option<&MwTable> {
        Some(&self.table)
    }

    fn player(&self, player: PlayerId) -> Option<&Sheet> {
        (player == PlayerId::HOST).then_some(&self.player)
    }

    fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
        match actor {
            SPEAKER => Some(&self.speaker),
            RAT => Some(&self.rat),
            _ => None,
        }
    }

    fn rules(&self) -> Option<&Rules> {
        Some(&self.rules)
    }
}

/// The world: the sheets, the speaker's and the rat's facts, float speaker locals `a`-`f`
/// that capture reads.
struct World {
    sheets: Sheets,
    speaker: ActorFacts,
    rat: ActorFacts,
}

impl World {
    fn new() -> Self {
        let speaker = ActorFacts {
            id: id("guildmaster"),
            npc: true,
            faction: Some(id(FIGHTERS)),
            rank: 1,
            ..ActorFacts::default()
        };
        let rat = ActorFacts { id: id("rat"), npc: false, ..ActorFacts::default() };
        Self { sheets: Sheets::new(), speaker, rat }
    }
}

impl MwWorld for World {
    fn global(&self, _name: &RecordId) -> Option<GlobalNumber> {
        None
    }

    fn var(&self, _ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        let local = *target == Target::Speaker && ["a", "b", "c", "d", "e", "f"].contains(&name.as_str());
        local.then_some(ScriptVar { ty: VarType::Float, value: 0.0 })
    }

    fn item_count(&self, _ctx: &ConversationCtx, _target: &Target, _item: &RecordId) -> u64 {
        0
    }

    fn sheets(&self) -> &dyn MwSheets {
        &self.sheets
    }

    fn actor(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        match target {
            Target::Speaker => Some(ctx.speaker),
            Target::Actor(a) if a.as_str() == "rat" => Some(RAT),
            _ => None,
        }
    }

    fn facts(&self, _ctx: &ConversationCtx, target: &Target) -> Option<&ActorFacts> {
        match target {
            Target::Speaker => Some(&self.speaker),
            Target::Actor(a) if a.as_str() == "rat" => Some(&self.rat),
            _ => None,
        }
    }
}

/// What one run produced: the captured locals and the stat effects, in order.
struct Run {
    locals: HashMap<String, f32>,
    stats: Vec<(Target, Vec<StatOp>)>,
}

fn run(world: &World, script: &str) -> Run {
    let (rpg, pc) = (WorldRpg::default(), CharacterRpg::default());
    let mut results = MwResults::new();
    let ctx = ConversationCtx { player: PlayerId::HOST, speaker: SPEAKER, speaker_base: id("guildmaster"), choice: None };
    let (topic, response) = (id("topic"), id("r"));
    let effects = results
        .runner(ResultState { world: &rpg, player: &pc, env: world })
        .run(ResultCall { ctx: &ctx, topic: &topic, response: &response, script })
        .unwrap();
    assert!(results.drain_reports().is_empty(), "stat functions are never unhandled");
    let mut out = Run { locals: HashMap::new(), stats: Vec::new() };
    for e in effects {
        match e {
            ResultEffect::SetLocal { name, value, .. } => {
                out.locals.insert(name.as_str().to_owned(), value);
            }
            ResultEffect::Stats { target, ops } => out.stats.push((target, ops)),
            other => panic!("unexpected effect {other:?}"),
        }
    }
    out
}

impl Run {
    fn local(&self, name: &str) -> f32 {
        self.locals[name]
    }
}

#[test]
fn mod_stat_clamps_like_openmw() {
    assert!((mod_stat(50.0, 50.0, 60.0) - 100.0).abs() < f32::EPSILON, "gain capped at a modified 100");
    assert!((mod_stat(95.0, 95.0, 10.0) - 100.0).abs() < f32::EPSILON);
    assert!((mod_stat(50.0, 110.0, 30.0) - 80.0).abs() < f32::EPSILON, "a fortified base under 100 isn't capped");
    assert!((mod_stat(100.0, 110.0, 5.0) - 100.0).abs() < f32::EPSILON, "base 100: no gain");
    assert!(mod_stat(50.0, 30.0, -70.0).abs() < f32::EPSILON, "base floors at 0");
    assert!((mod_stat(50.0, 50.0, f32::NAN) - 50.0).abs() < f32::EPSILON);
}

#[test]
fn attributes_and_skills_get_set_mod() {
    let w = World::new();
    let i = &w.sheets.table.idx;
    let (str_, blade) = (i.attributes[0], i.skills[5]);
    let r = run(
        &w,
        "set a to player->GetStrength\nplayer->ModStrength 60\nset b to player->GetStrength\n\
         player->SetLongBlade 75\nset c to player->GetLongBlade\nplayer->ModLongBlade -80\n\
         set d to player->GetLongBlade\nset e to GetStrength",
    );
    assert_eq!(
        (r.local("a"), r.local("b"), r.local("c"), r.local("d"), r.local("e")),
        (50.0, 100.0, 75.0, 0.0, 40.0),
        "reads see the run's writes; implicit = the speaker"
    );
    assert_eq!(
        r.stats,
        [
            (Target::Player, vec![StatOp::SetBase { stat: str_, value: 100.0 }]),
            (Target::Player, vec![
                StatOp::SetBase { stat: blade, value: 75.0 },
                StatOp::Restore { stat: blade, amount: f32::MAX },
            ]),
            (Target::Player, vec![StatOp::SetBase { stat: blade, value: 0.0 }]),
        ]
    );
}

#[test]
fn dynamics_follow_openmw_flags() {
    let w = World::new();
    let health = w.sheets.table.idx.health;
    let r = run(
        &w,
        "set a to player->GetHealth\nplayer->ModHealth 10\nset b to player->GetHealth\n\
         set c to player->GetHealthGetRatio\nplayer->SetHealth 50\nset d to player->GetHealth\n\
         player->ModHealth -70\nset e to player->GetHealth\nset f to player->GetMagickaGetRatio",
    );
    assert_eq!((r.local("a"), r.local("b"), r.local("d"), r.local("e")), (60.0, 70.0, 50.0, -20.0));
    assert_eq!(r.local("c").to_bits(), (70.0_f32 / 90.0).to_bits(), "70 / 90");
    assert_eq!(r.local("f").to_bits(), 1.0_f32.to_bits(), "a 0 max reads ratio 1 (getRatio(false))");
    assert_eq!(
        r.stats,
        [
            (Target::Player, vec![StatOp::PoolModMax { pool: health, delta: 10.0 }]),
            (Target::Player, vec![
                StatOp::SetBase { stat: health, value: 50.0 },
                StatOp::PoolSetCurrent { pool: health, value: 50.0 },
            ]),
            (Target::Player, vec![
                StatOp::PoolModMax { pool: health, delta: -50.0 },
                StatOp::PoolSetCurrent { pool: health, value: -20.0 },
            ]),
        ],
        "ModHealth: the base stops at 0, the current moves by the whole diff"
    );
}

#[test]
fn mod_current_lets_only_fatigue_go_below_zero() {
    let w = World::new();
    let i = &w.sheets.table.idx;
    let fatigue = w.sheets.player.view(&w.sheets.table.table).current(i.fatigue);
    let r = run(
        &w,
        "player->ModCurrentFatigue -1000\nset a to player->GetFatigue\n\
         player->ModCurrentHealth -1000\nset b to player->GetHealth\nset c to player->GetMagicka",
    );
    assert_eq!((r.local("a"), r.local("b"), r.local("c")), (fatigue - 1000.0, 0.0, 0.0));
    assert_eq!(
        r.stats,
        [
            (Target::Player, vec![StatOp::PoolAdjust {
                pool: i.fatigue,
                delta: -1000.0,
                below_zero: true,
                above_max: false
            }]),
            (Target::Player, vec![StatOp::PoolAdjust {
                pool: i.health,
                delta: -1000.0,
                below_zero: false,
                above_max: false
            }]),
        ]
    );
}

#[test]
fn player_factions_rank_rep_expel() {
    let w = World::new();
    let (fg, mg, temple) = (id(FIGHTERS), id("mages guild"), id("temple"));
    let r = run(
        &w,
        "set a to GetPCRank\nPCRaiseRank\nset b to GetPCRank \"Fighters Guild\"\n\
         PCJoinFaction \"Mages Guild\"\nset c to GetPCRank \"mages guild\"\nPCLowerRank \"mages guild\"\n\
         set d to GetPCRank \"mages guild\"\nPCRaiseRank \"Temple\"\nModPCFacRep 5\nset e to GetPCFacRep\n\
         PCExpell\nset f to PCExpelled",
    );
    assert_eq!(
        (r.local("a"), r.local("b"), r.local("c"), r.local("d"), r.local("e"), r.local("f")),
        (3.0, 4.0, 0.0, -1.0, 5.0, 1.0),
        "no faction argument: the speaker's faction"
    );
    let p = Target::Player;
    assert_eq!(
        r.stats,
        [
            (p.clone(), vec![StatOp::SetRank { group: fg.clone(), rank: 4 }]),
            (p.clone(), vec![StatOp::Join { group: mg.clone() }]),
            (p.clone(), vec![StatOp::Leave { group: mg }]),
            (p.clone(), vec![StatOp::Join { group: temple }]),
            (p.clone(), vec![StatOp::GroupRep { group: fg.clone(), delta: 5, set: false }]),
            (p, vec![StatOp::Expel { group: fg }]),
        ],
        "lowering rank 0 leaves; raising a non-member joins"
    );
}

#[test]
fn rank_stops_at_the_last_slot() {
    let w = World::new();
    let r = run(&w, &"PCRaiseRank\n".repeat(7));
    let ranks: Vec<i32> = r
        .stats
        .iter()
        .map(|(_, ops)| match ops.as_slice() {
            [StatOp::SetRank { rank, .. }] => *rank,
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(ranks, [4, 5, 6, 7, 8, 9], "3 -> 9, then a rank past the 10 slots is ignored");
}

#[test]
fn npc_rank_starts_from_the_record_and_skips_the_player() {
    let w = World::new();
    let fg = id(FIGHTERS);
    let r = run(&w, "RaiseRank\nRaiseRank\nplayer->RaiseRank");
    assert_eq!(
        r.stats,
        [
            (Target::Speaker, vec![StatOp::Join { group: fg.clone() }, StatOp::SetRank { group: fg.clone(), rank: 2 }]),
            (Target::Speaker, vec![StatOp::SetRank { group: fg.clone(), rank: 3 }]),
        ]
    );
    let r = run(&w, "LowerRank\nLowerRank");
    assert_eq!(
        r.stats,
        [(Target::Speaker, vec![StatOp::Join { group: fg.clone() }, StatOp::SetRank { group: fg, rank: 0 }])],
        "record rank 1 -> 0, then rank 0 stays"
    );
}

#[test]
fn crime_reputation_level() {
    let w = World::new();
    let i = &w.sheets.table.idx;
    let r = run(
        &w,
        "set a to GetPCCrimeLevel\nModPCCrimeLevel 40\nModPCCrimeLevel -100\nSetPCCrimeLevel 25.9\n\
         set b to GetPCCrimeLevel\nModReputation 300\nset c to GetReputation\nSetLevel 12\nset d to GetLevel",
    );
    assert_eq!((r.local("a"), r.local("b"), r.local("c"), r.local("d")), (0.0, 25.0, 255.0, 12.0));
    let p = Target::Player;
    assert_eq!(
        r.stats,
        [
            (p.clone(), vec![StatOp::CounterSet { stat: i.bounty, value: 40.0 }]),
            (p.clone(), vec![StatOp::CounterSet { stat: i.bounty, value: 0.0 }]),
            (p, vec![StatOp::CounterSet { stat: i.bounty, value: 25.0 }]),
            (Target::Speaker, vec![StatOp::CounterSet { stat: i.reputation, value: 255.0 }]),
            (Target::Speaker, vec![StatOp::CounterSet { stat: i.level, value: 12.0 }]),
        ],
        "the bounty floors at 0 on Mod only; reputation clamps to 0..=255"
    );
}

#[test]
fn creatures_and_missing_sheets() {
    let w = World::new();
    let r = run(
        &w,
        "\"rat\"->SetLongBlade 50\n\"rat\"->ModReputation 5\nset a to \"rat\"->GetStrength\n\
         set b to \"rat\"->GetReputation\nset c to \"nobody\"->GetStrength",
    );
    assert!(r.stats.is_empty(), "no skill or reputation writes on a creature: {:?}", r.stats);
    assert_eq!((r.local("a"), r.local("b"), r.local("c")), (30.0, 0.0, 0.0));

    // No sheets at all (the default `MwWorld::sheets`): neutral reads, GetPCRank -1, no writes.
    struct Bare;
    impl MwWorld for Bare {
        fn global(&self, _: &RecordId) -> Option<GlobalNumber> {
            None
        }
        fn var(&self, _: &ConversationCtx, t: &Target, n: &RecordId) -> Option<ScriptVar> {
            (*t == Target::Speaker && n.as_str() == "a").then_some(ScriptVar { ty: VarType::Float, value: 0.0 })
        }
        fn item_count(&self, _: &ConversationCtx, _: &Target, _: &RecordId) -> u64 {
            0
        }
    }
    let (rpg, pc) = (WorldRpg::default(), CharacterRpg::default());
    let ctx = ConversationCtx { player: PlayerId::HOST, speaker: SPEAKER, speaker_base: id("x"), choice: None };
    let (topic, response) = (id("topic"), id("r"));
    let mut results = MwResults::new();
    let effects = results
        .runner(ResultState { world: &rpg, player: &pc, env: &Bare })
        .run(ResultCall { ctx: &ctx, topic: &topic, response: &response, script: "PCRaiseRank\nset a to GetPCRank" })
        .unwrap();
    assert_eq!(effects, [ResultEffect::SetLocal { target: Target::Speaker, name: id("a"), value: -1.0 }]);
}
