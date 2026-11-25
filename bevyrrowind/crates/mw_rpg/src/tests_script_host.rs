//! The global-script host over the game API (DQ-4b1b): synthetic scripts, a world test double.

use std::collections::HashMap;

use mw_script::ast::VarType;
use mw_script::{
    Env, FunctionTable, GlobalInstances, Interpreter, MemberLookup, Report, ScriptSet,
};
use sf_records::RecordId;
use sf_rpg::{ActorKey, CharacterRpg, ConversationCtx, PlayerId, WorldRpg};

use crate::{
    GameOp, GameView, GlobalNumber, MwWorld, ScriptHost, ScriptReport, ScriptReports, ScriptVar,
    Target,
};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Globals `g_count` and `g_seen` (long); no script variables, no world-held items.
struct TestWorld {
    globals: HashMap<RecordId, GlobalNumber>,
}

impl TestWorld {
    fn new() -> Self {
        let globals = [("g_count", 0), ("g_seen", 0)]
            .into_iter()
            .map(|(k, v)| (id(k), GlobalNumber::Int(v)))
            .collect();
        Self { globals }
    }
}

impl MwWorld for TestWorld {
    fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.globals.get(name).copied()
    }
    fn var(&self, _: &ConversationCtx, _: &Target, _: &RecordId) -> Option<ScriptVar> {
        None
    }
    fn item_count(&self, _: &ConversationCtx, _: &Target, _: &RecordId) -> u64 {
        0
    }
}

/// Script-compile names: the test globals, every object exists.
struct TestEnv;

impl Env for TestEnv {
    fn global(&self, key: &str) -> Option<VarType> {
        matches!(key, "g_count" | "g_seen").then_some(VarType::Long)
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        Some(true)
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Unknown
    }
}

fn set_of(scripts: &[(&str, &str)]) -> ScriptSet {
    let mut set = ScriptSet::default();
    for (name, text) in scripts {
        assert!(
            set.parse_insert(name, text, FunctionTable::vanilla(), &TestEnv),
            "{name} inert"
        );
    }
    set
}

/// The context of a global tick: the host player; the speaker is unused.
fn ctx() -> ConversationCtx {
    ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(0),
        speaker_base: id(""),
        choice: None,
    }
}

/// One tick of `instances` over a fresh view: the ops and the new reports.
fn tick(
    set: &ScriptSet,
    interp: &Interpreter,
    instances: &mut GlobalInstances,
    reports: &mut ScriptReports,
) -> Vec<GameOp> {
    let (world, player, env, ctx) = (
        WorldRpg::default(),
        CharacterRpg::default(),
        TestWorld::new(),
        ctx(),
    );
    let view = GameView {
        ctx: &ctx,
        world: &world,
        player: &player,
        env: &env,
    };
    let mut host = ScriptHost::new(view, reports);
    instances.tick(set, interp, &mut host);
    host.into_ops()
}

#[test]
fn a_global_tick_yields_its_ops_in_call_order() {
    let quest = "Begin dq_quest\nSetJournalIndex \"dq_q\" 20\nPlayer->AddItem \"dq_gem\" 2\nEnd\n";
    let set = set_of(&[("dq_quest", quest)]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut instances = GlobalInstances::default();
    instances.start(&set, "dq_quest", None);
    let mut reports = ScriptReports::default();
    let ops = tick(&set, &interp, &mut instances, &mut reports);
    assert_eq!(
        ops,
        [
            GameOp::SetJournalIndex {
                quest: id("dq_q"),
                index: 20,
            },
            GameOp::AddItem {
                target: Target::Player,
                item: id("dq_gem"),
                count: 2,
            },
        ]
    );
    assert_eq!(reports.drain(), []);
}

#[test]
fn a_step_limit_run_leaves_no_ops_and_later_runs_see_only_committed_writes() {
    let first = "Begin dq_a\nset g_count to 1\nJournal \"dq_q\" 10\nEnd\n";
    let runaway = "Begin dq_loop\nset g_count to 7\nPlayer->AddItem \"dq_gem\" 1\nwhile ( 1 )\nset g_count to g_count + 1\nendwhile\nEnd\n";
    let reader = "Begin dq_c\nset g_seen to g_count\nEnd\n";
    let set = set_of(&[("dq_a", first), ("dq_loop", runaway), ("dq_c", reader)]);
    let mut interp = Interpreter::new(FunctionTable::vanilla());
    interp.step_limit = 1000;
    let mut instances = GlobalInstances::default();
    for script in ["dq_a", "dq_loop", "dq_c"] {
        instances.start(&set, script, None);
    }
    let mut reports = ScriptReports::default();
    let ops = tick(&set, &interp, &mut instances, &mut reports);
    assert_eq!(
        ops,
        [
            GameOp::SetGlobal {
                name: id("g_count"),
                value: 1.0,
            },
            GameOp::Journal {
                quest: id("dq_q"),
                index: 10,
            },
            // dq_c reads dq_a's write, not the dropped run's.
            GameOp::SetGlobal {
                name: id("g_seen"),
                value: 1.0,
            },
        ]
    );
    assert_eq!(
        instances.take_reports(),
        [Report::StepLimit {
            script: "dq_loop".into(),
            line: 4,
        }]
    );
    assert!(!instances.is_running("dq_loop"));
}

#[test]
fn ref_start_script_targets_resolve_as_self() {
    let starter = "Begin dq_start\nshort done\nif ( done == 0 )\n\"dq_fargoth\"->StartScript dq_self\nPlayer->StartScript dq_pc\nStartScript dq_plain\nset done to 1\nendif\nEnd\n";
    let on_self = "Begin dq_self\nAddItem \"dq_gem\" 1\nif ( ( GetItemCount \"dq_gem\" ) == 1 )\nRemoveItem \"dq_gem\" 1\nendif\nEnd\n";
    let on_pc = "Begin dq_pc\nAddItem \"dq_coin\" 3\nEnd\n";
    let plain = "Begin dq_plain\nAddItem \"dq_gem\" 1\nJournal \"dq_q\" 5\nEnd\n";
    let set = set_of(&[
        ("dq_start", starter),
        ("dq_self", on_self),
        ("dq_pc", on_pc),
        ("dq_plain", plain),
    ]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut instances = GlobalInstances::default();
    instances.start(&set, "dq_start", None);
    let mut reports = ScriptReports::default();
    assert_eq!(tick(&set, &interp, &mut instances, &mut reports), []);
    assert_eq!(
        instances.get("dq_self").unwrap().target.as_deref(),
        Some("dq_fargoth")
    );
    // The started scripts first run on the next tick.
    let ops = tick(&set, &interp, &mut instances, &mut reports);
    let fargoth = Target::Actor(id("dq_fargoth"));
    assert_eq!(
        ops,
        [
            GameOp::AddItem {
                target: fargoth.clone(),
                item: id("dq_gem"),
                count: 1,
            },
            // GetItemCount on self reads the run's own AddItem.
            GameOp::RemoveItem {
                target: fargoth,
                item: id("dq_gem"),
                count: 1,
            },
            GameOp::AddItem {
                target: Target::Player,
                item: id("dq_coin"),
                count: 3,
            },
            // A plain global has no self: its AddItem is reported, its Journal still runs.
            GameOp::Journal {
                quest: id("dq_q"),
                index: 5,
            },
        ]
    );
    assert_eq!(
        reports.drain(),
        [ScriptReport::NoSelf {
            script: "dq_plain".into(),
            function: "AddItem".into(),
        }]
    );
    tick(&set, &interp, &mut instances, &mut reports);
    assert_eq!(reports.drain(), [], "reported once");
}
