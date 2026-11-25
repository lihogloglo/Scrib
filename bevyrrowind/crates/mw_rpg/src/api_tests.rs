//! The game API: its pure Morrowind rules, the run overlay and op serde (no interpreter).

use std::collections::HashMap;

use mw_ui::message::MessageArg;
use sf_records::RecordId;
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, Choice, ConversationCtx, PlayerId, RpgCommand, StageFlags,
    WorldRpg,
};
use sf_stats::StatOp;

use crate::api::{
    GameOp, GameRun, GameView, MwWorld, ScriptVar, Target, add_count, disposition, journal_stage,
    remove_count, stored_value,
};
use crate::{GlobalNumber, NpcStates, VarType};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

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

fn ctx() -> ConversationCtx {
    ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(1),
        speaker_base: id("npc"),
        choice: None,
    }
}

#[test]
fn journal_raises_and_never_lowers() {
    assert_eq!(journal_stage(0, 10), 10);
    assert_eq!(journal_stage(20, 10), 20);
    assert_eq!(journal_stage(20, 20), 20);
    assert_eq!(journal_stage(5, -1), 5);
}

#[test]
fn item_counts_clamp() {
    assert_eq!(add_count(3), Some(3));
    assert_eq!(add_count(0), None);
    assert_eq!(add_count(-2), None);
    assert_eq!(remove_count(5, 3), Some(3));
    assert_eq!(remove_count(2, 3), Some(2));
    assert_eq!(remove_count(0, 3), None);
    assert_eq!(remove_count(4, 0), None);
    assert_eq!(remove_count(4, -1), None);
    assert_eq!(remove_count(-3, 2), None);
}

#[test]
fn disposition_clamps_to_0_100() {
    assert_eq!(disposition(40, 25), 65);
    assert_eq!(disposition(90, 30), 100);
    assert_eq!(disposition(10, -30), 0);
    assert_eq!(disposition(i32::MAX, 1), 100);
    let mut npcs = NpcStates::default();
    npcs.modify(ActorKey(7), 80);
    assert_eq!(npcs.disposition(ActorKey(7), 50), 100);
}

#[test]
#[allow(
    clippy::float_cmp,
    reason = "exact stored values (integers and a dyadic fraction)"
)]
fn stored_values_take_the_variable_type() {
    assert_eq!(stored_value(VarType::Short, 2.9), 2.0);
    assert_eq!(stored_value(VarType::Short, 40000.0), -25536.0);
    assert_eq!(stored_value(VarType::Float, 0.25), 0.25);
}

#[test]
fn overlay_reads_see_the_runs_writes() {
    let mut world = WorldRpg::default();
    world
        .apply(RpgCommand::SetStage {
            quest: id("q"),
            stage: 20,
            flags: StageFlags::default(),
            entry: None,
        })
        .unwrap();
    let mut pc = CharacterRpg::default();
    pc.loadout.add(id("gem"), 2, Attrs::new()).unwrap();
    let mut env = TestWorld::default();
    env.globals.insert(id("g"), GlobalNumber::Int(1));
    env.vars.insert(
        (Target::Speaker, id("state")),
        ScriptVar {
            ty: VarType::Short,
            value: 0.0,
        },
    );
    env.items.insert((Target::Speaker, id("gem")), 1);
    let ctx = ctx();
    let view = GameView {
        ctx: &ctx,
        world: &world,
        player: &pc,
        env: &env,
    };
    let mut run = GameRun::new(view);

    run.journal(&id("q"), 10);
    assert_eq!(run.quest_stage(&id("q")), 20, "Journal never lowers");
    run.set_journal_index(&id("q"), 5);
    assert_eq!(run.quest_stage(&id("q")), 5, "SetJournalIndex lowers");
    assert_eq!(view.quest_stage(&id("q")), 20, "the view is not written");

    assert!(run.set_global(&id("g"), 7.6));
    assert_eq!(run.global(&id("g")), Some(7.0), "long truncates");
    assert!(!run.set_global(&id("nope"), 1.0));
    assert_eq!(run.global(&id("nope")), None);
    assert!(run.set_var(&Target::Speaker, &id("state"), 3.0));
    assert_eq!(run.var(&Target::Speaker, &id("state")), Some(3.0));
    assert!(!run.set_var(&Target::Player, &id("state"), 1.0));

    run.remove_item(Target::Player, id("gem"), 5);
    assert_eq!(run.item_count(&Target::Player, &id("gem")), 0);
    run.remove_item(Target::Player, id("gem"), 1);
    run.add_item(Target::Speaker, id("gem"), 2);
    run.add_item(Target::Speaker, id(""), 2);
    assert_eq!(run.item_count(&Target::Speaker, &id("gem")), 3);

    assert_eq!(
        run.into_ops(),
        vec![
            GameOp::Journal {
                quest: id("q"),
                index: 10
            },
            GameOp::SetJournalIndex {
                quest: id("q"),
                index: 5
            },
            GameOp::SetGlobal {
                name: id("g"),
                value: 7.0
            },
            GameOp::SetLocal {
                target: Target::Speaker,
                name: id("state"),
                value: 3.0
            },
            GameOp::RemoveItem {
                target: Target::Player,
                item: id("gem"),
                count: 2
            },
            GameOp::AddItem {
                target: Target::Speaker,
                item: id("gem"),
                count: 2
            },
        ]
    );
}

#[test]
fn message_box_formats_in_the_api() {
    let (world, pc, env, ctx) = (
        WorldRpg::default(),
        CharacterRpg::default(),
        TestWorld::default(),
        ctx(),
    );
    let mut run = GameRun::new(GameView {
        ctx: &ctx,
        world: &world,
        player: &pc,
        env: &env,
    });
    run.message_box(
        "%.1f gold",
        &[MessageArg::Num(5.0), MessageArg::Str("Ok".into())],
    );
    assert_eq!(
        run.ops(),
        [GameOp::MessageBox {
            text: "5.0 gold".into(),
            buttons: vec!["Ok".into()]
        }]
    );
}

#[test]
fn ops_round_trip_through_serde() {
    let ops = vec![
        GameOp::Journal {
            quest: id("q"),
            index: 10,
        },
        GameOp::SetJournalIndex {
            quest: id("q"),
            index: 0,
        },
        GameOp::AddTopic(id("t")),
        GameOp::AddItem {
            target: Target::Player,
            item: id("gold_001"),
            count: 5,
        },
        GameOp::RemoveItem {
            target: Target::Actor(id("arrille")),
            item: id("gem"),
            count: 1,
        },
        GameOp::Choices(vec![Choice {
            label: "Yes".into(),
            value: 1,
        }]),
        GameOp::Goodbye,
        GameOp::ModDisposition {
            target: Target::Speaker,
            delta: -5,
        },
        GameOp::SetDisposition {
            target: Target::Speaker,
            value: 60,
        },
        GameOp::SetLocal {
            target: Target::Speaker,
            name: id("state"),
            value: 2.0,
        },
        GameOp::SetGlobal {
            name: id("g"),
            value: 0.5,
        },
        GameOp::MessageBox {
            text: "hi".into(),
            buttons: vec![],
        },
        GameOp::PlaySound(id("snd")),
        GameOp::Stats {
            target: Target::Player,
            ops: Vec::<StatOp>::new(),
        },
    ];
    let bytes = postcard::to_allocvec(&ops).unwrap();
    let back: Vec<GameOp> = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, ops);
}
