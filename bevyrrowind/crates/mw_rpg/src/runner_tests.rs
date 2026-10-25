//! Result runner over `sf_rpg` state with a world test double (synthetic scripts).

use std::collections::HashMap;

use sf_records::RecordId;
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, Choice, ConversationCtx, PlayerId, RpgCommand, StageFlags,
    WorldRpg,
};

use crate::{
    GlobalNumber, MwResults, MwWorld, ResultCall, ResultEffect, ResultError, ResultReport,
    ResultRunner, ResultState, ScriptVar, Target, VarType,
};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// World test double: globals, script variables and world-held items in maps.
#[derive(Default)]
struct TestWorld {
    globals: HashMap<RecordId, GlobalNumber>,
    vars: HashMap<(Target, RecordId), ScriptVar>,
    items: HashMap<(Target, RecordId), u64>,
}

impl MwWorld for TestWorld {
    fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.globals.get(name).copied()
    }
    fn var(&self, _: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        self.vars.get(&(target.clone(), name.clone())).copied()
    }
    fn item_count(&self, _: &ConversationCtx, target: &Target, item: &RecordId) -> u64 {
        self.items
            .get(&(target.clone(), item.clone()))
            .copied()
            .unwrap_or(0)
    }
}

fn world() -> TestWorld {
    let mut w = TestWorld::default();
    w.globals.insert(id("g_count"), GlobalNumber::Int(0));
    w.globals.insert(id("g_ratio"), GlobalNumber::Float(0.5));
    w.vars.insert(
        (Target::Speaker, id("state")),
        ScriptVar {
            ty: VarType::Short,
            value: 0.0,
        },
    );
    w.vars.insert(
        (Target::Actor(id("other_npc")), id("met")),
        ScriptVar {
            ty: VarType::Long,
            value: 3.0,
        },
    );
    w.items.insert((Target::Speaker, id("gem")), 4);
    w
}

fn ctx() -> ConversationCtx {
    ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(1),
        speaker_base: id("test_npc"),
        choice: None,
    }
}

struct Fixture {
    world: WorldRpg,
    pc: CharacterRpg,
    env: TestWorld,
    results: MwResults,
}

impl Fixture {
    fn new() -> Self {
        Self {
            world: WorldRpg::default(),
            pc: CharacterRpg::default(),
            env: world(),
            results: MwResults::new(),
        }
    }

    fn run_as(&mut self, response: &str, script: &str) -> Result<Vec<ResultEffect>, ResultError> {
        let state = ResultState {
            world: &self.world,
            player: &self.pc,
            env: &self.env,
        };
        let (ctx, topic, response) = (ctx(), id("topic"), id(response));
        self.results.runner(state).run(ResultCall {
            ctx: &ctx,
            topic: &topic,
            response: &response,
            script,
        })
    }

    fn run(&mut self, script: &str) -> Vec<ResultEffect> {
        self.run_as("r", script).unwrap()
    }
}

#[test]
fn quest_result_becomes_effects_in_order() {
    let mut f = Fixture::new();
    f.pc.loadout.add(id("lost_ring"), 1, Attrs::new()).unwrap();
    let effects = f.run(
        "Journal \"q_ring\" 100\nAddItem \"lost_ring\" 1\nplayer->RemoveItem \"lost_ring\" 1\n\
         set state to -1\nModDisposition 50\n\"other_npc\"->ModDisposition 40",
    );
    assert_eq!(
        effects,
        [
            ResultEffect::Journal {
                quest: id("q_ring"),
                index: 100
            },
            ResultEffect::AddItem {
                target: Target::Speaker,
                item: id("lost_ring"),
                count: 1
            },
            ResultEffect::RemoveItem {
                target: Target::Player,
                item: id("lost_ring"),
                count: 1
            },
            ResultEffect::SetLocal {
                target: Target::Speaker,
                name: id("state"),
                value: -1.0
            },
            ResultEffect::ModDisposition {
                target: Target::Speaker,
                delta: 50
            },
            ResultEffect::ModDisposition {
                target: Target::Actor(id("other_npc")),
                delta: 40
            },
        ]
    );
    assert!(f.results.drain_reports().is_empty());
}

#[test]
fn journal_reads_see_this_runs_writes() {
    let mut f = Fixture::new();
    f.world
        .apply(RpgCommand::SetStage {
            quest: id("q"),
            stage: 20,
            flags: StageFlags::default(),
            entry: None,
        })
        .unwrap();
    let effects = f.run(
        "Journal q 10\nif ( GetJournalIndex q == 20 )\nAddTopic \"kept\"\nendif\n\
         SetJournalIndex q 5\nif ( GetJournalIndex q == 5 )\nAddTopic \"lowered\"\nendif",
    );
    let topics: Vec<_> = effects
        .iter()
        .filter_map(|e| match e {
            ResultEffect::AddTopic(t) => Some(t.as_str().to_owned()),
            _ => None,
        })
        .collect();
    assert_eq!(
        topics,
        ["kept", "lowered"],
        "Journal only raises; SetJournalIndex sets"
    );
}

#[test]
fn item_counts_read_player_world_and_overlay() {
    let mut f = Fixture::new();
    f.pc.loadout.add(id("gem"), 2, Attrs::new()).unwrap();
    let effects = f.run(
        "set state to player->GetItemCount gem\nset met_total to 0\n\
         AddItem gem 1\nif ( GetItemCount gem == 5 )\nset state to state + 10\nendif\n\
         player->RemoveItem gem 9\nif ( player->GetItemCount gem == 0 )\nset state to state + 100\nendif",
    );
    let removed = effects.iter().find_map(|e| match e {
        ResultEffect::RemoveItem { count, .. } => Some(*count),
        _ => None,
    });
    assert_eq!(removed, Some(2), "RemoveItem takes what the holder has");
    assert_eq!(
        effects.last(),
        Some(&ResultEffect::SetLocal {
            target: Target::Speaker,
            name: id("state"),
            value: 112.0
        })
    );
    assert_eq!(
        f.results.drain_reports(),
        [ResultReport::UnknownVariable("speaker.met_total".into())]
    );
}

#[test]
fn removing_nothing_and_adding_nothing_are_no_ops() {
    let mut f = Fixture::new();
    assert!(
        f.run("player->RemoveItem gem 1\nAddItem gem 0\nAddItem gem -3")
            .is_empty()
    );
}

#[test]
fn globals_and_members_convert_and_overlay() {
    let mut f = Fixture::new();
    let effects = f.run(
        "set g_count to 2.9\nset g_ratio to g_ratio + 0.25\nset other_npc.met to other_npc.met * 2\n\
         if ( g_count == 2 )\nset state to g_ratio * 4\nendif",
    );
    assert_eq!(
        effects,
        [
            ResultEffect::SetGlobal {
                name: id("g_count"),
                value: 2.0
            },
            ResultEffect::SetGlobal {
                name: id("g_ratio"),
                value: 0.75
            },
            ResultEffect::SetLocal {
                target: Target::Actor(id("other_npc")),
                name: id("met"),
                value: 6.0
            },
            ResultEffect::SetLocal {
                target: Target::Speaker,
                name: id("state"),
                value: 3.0
            },
        ]
    );
}

#[test]
fn ui_session_and_sound_effects() {
    let mut f = Fixture::new();
    let effects = f.run(
        "Choice \"No.\" 1 \"Yes.\" 2\nMessageBox \"%g drakes\" 25\nPlaySound \"Item Gold Up\"\n\
         SetDisposition 60\nGoodbye",
    );
    assert_eq!(
        effects,
        [
            ResultEffect::Choices(vec![
                Choice {
                    label: "No.".into(),
                    value: 1
                },
                Choice {
                    label: "Yes.".into(),
                    value: 2
                },
            ]),
            ResultEffect::MessageBox {
                text: "25 drakes".into(),
                buttons: vec![]
            },
            ResultEffect::PlaySound(id("item gold up")),
            ResultEffect::SetDisposition {
                target: Target::Speaker,
                value: 60
            },
            ResultEffect::Goodbye,
        ]
    );
}

#[test]
fn unhandled_functions_are_reported_once() {
    let mut f = Fixture::new();
    let effects = f.run("ModPCFacRep 5 \"temple\"\nif ( GetPCRank temple < 0 )\nGoodbye\nendif");
    assert_eq!(
        effects,
        [ResultEffect::Goodbye],
        "GetPCRank reads its stub -1"
    );
    f.run_as("other", "ModPCFacRep 1 \"temple\"").unwrap();
    assert_eq!(
        f.results.drain_reports(),
        [
            ResultReport::Unhandled("ModPCFacRep".into()),
            ResultReport::Unhandled("GetPCRank".into())
        ]
    );
    f.run("ModPCFacRep 2 \"temple\"");
    assert!(f.results.drain_reports().is_empty());
}

#[test]
fn inert_results_never_run_and_are_reported_once() {
    let mut f = Fixture::new();
    let text = "AddTopic \"a\"\nNoSuchFunction 3";
    assert!(f.run(text).is_empty());
    assert!(f.run(text).is_empty());
    let reports = f.results.drain_reports();
    assert!(
        matches!(&reports[..], [ResultReport::Inert { response, .. }] if response.as_str() == "r"),
        "{reports:?}"
    );
}

#[test]
fn edited_text_is_parsed_again() {
    let mut f = Fixture::new();
    assert_eq!(f.run("AddTopic a"), [ResultEffect::AddTopic(id("a"))]);
    assert_eq!(f.run("AddTopic b"), [ResultEffect::AddTopic(id("b"))]);
}

#[test]
fn runaway_loops_fail_the_run() {
    let mut f = Fixture::new();
    f.results.set_step_limit(50);
    let err = f
        .run_as("loop", "while ( 1 )\nAddTopic a\nendwhile")
        .unwrap_err();
    assert!(matches!(err, ResultError::Failed { response, .. } if response.as_str() == "loop"));
}
