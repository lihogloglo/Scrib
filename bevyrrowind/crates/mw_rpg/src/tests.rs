//! Synthetic filter, refusal and journal tests (the real-data flow is `tests/fargoth.rs`).

use std::collections::HashMap;

use sf_records::RecordId;
use sf_rpg::{
    ActorKey, Choice, ConversationCtx, DialogueOutput, DialogueSession, GameTime, PlayerId,
    RpgCommand, Topics, WorldRpg,
};
use tes3::esp::{
    Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison, FilterFunction, FilterType,
    FilterValue, QuestState,
};

use crate::db::info;
use crate::{ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwView};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn dial(name: &str, kind: DialogueType2) -> Dialogue {
    Dialogue {
        id: name.into(),
        dialogue_type: kind,
        ..Dialogue::default()
    }
}

fn filter(kind: FilterType, name: &str, op: FilterComparison, value: i32) -> Filter {
    Filter {
        filter_type: kind,
        id: name.into(),
        comparison: op,
        value: FilterValue::Integer(value),
        ..Filter::default()
    }
}

fn said_by(mut i: DialogueInfo, speaker: &str, filters: Vec<Filter>) -> DialogueInfo {
    i.speaker_id = speaker.into();
    i.filters = filters;
    i
}

fn data() -> MwDialogue {
    use FilterComparison::{Equal, Greater};
    use FilterType::{Journal, Local, NotId, NotLocal};
    let mut choice = filter(FilterType::Function, "", Equal, 1);
    choice.function = FilterFunction::Choice;
    let mut health = filter(FilterType::Function, "", Equal, 0);
    health.function = FilterFunction::PcHealthPercent;
    let mut secret = info("s1", "", "The secret.");
    secret.data.disposition = 50;
    let mut cell = info("c1", "", "Welcome to Seyda Neen.");
    cell.speaker_cell = "seyda".into();
    let mut name = info("q0", "", "The Ring Quest");
    name.quest_state = Some(QuestState::Name);
    let mut stage10 = info("q10", "q0", "%Name lost a ring.");
    stage10.data.disposition = 10;
    let mut stage100 = info("q100", "q10", "Returned it.");
    stage100.data.disposition = 100;
    stage100.quest_state = Some(QuestState::Finished);
    MwDialogue::from_topics([
        (
            dial("Greeting 1", DialogueType2::Greeting),
            vec![said_by(
                info("g1", "", "Go away."),
                "",
                vec![filter(NotId, "fargoth", Equal, 0)],
            )],
        ),
        (
            dial("Greeting 5", DialogueType2::Greeting),
            vec![said_by(
                info("g5", "", "Hello %PCName, they took my ring!"),
                "fargoth",
                vec![filter(Journal, "q", Equal, 0)],
            )],
        ),
        (
            dial("ring", DialogueType2::Topic),
            vec![
                said_by(
                    info("r1", "", "Thank you!"),
                    "fargoth",
                    vec![filter(Local, "state", Equal, -1)],
                ),
                said_by(info("r2", "r1", "Darn."), "fargoth", vec![choice]),
                said_by(
                    info("r3", "r2", "You found it!"),
                    "fargoth",
                    vec![filter(FilterType::Item, "ring_keley", Greater, 0)],
                ),
                said_by(
                    info("r4", "r3", "Never heard of it."),
                    "fargoth",
                    vec![filter(NotLocal, "missing", Equal, 0), health],
                ),
            ],
        ),
        (dial("secret", DialogueType2::Topic), vec![secret]),
        (dial("seyda neen", DialogueType2::Topic), vec![cell]),
        (
            dial("Info Refusal", DialogueType2::Persuasion),
            vec![info("x1", "", "I don't trust you, %PCName.")],
        ),
        (
            dial("q", DialogueType2::Journal),
            vec![name, stage10, stage100],
        ),
    ])
    .unwrap()
}

#[derive(Default)]
struct View {
    speaker: ActorFacts,
    player: ActorFacts,
    locals: HashMap<RecordId, f32>,
    items: HashMap<RecordId, u64>,
    disposition: i32,
    world: WorldRpg,
}

impl View {
    fn new(speaker: &str) -> Self {
        let actor = |name: &str, cell: &str| ActorFacts {
            id: id(name),
            name: name.to_owned(),
            npc: true,
            rank: -1,
            cell: cell.to_owned(),
            ..ActorFacts::default()
        };
        Self {
            speaker: actor(speaker, "Seyda Neen"),
            player: actor("Nerevar", "Seyda Neen"),
            disposition: 40,
            ..Self::default()
        }
    }
}

impl MwView for View {
    fn speaker(&self, _ctx: &ConversationCtx) -> Option<&ActorFacts> {
        Some(&self.speaker)
    }

    fn player(&self, _player: PlayerId) -> Option<&ActorFacts> {
        Some(&self.player)
    }

    fn player_rank(&self, _player: PlayerId, _faction: &RecordId) -> Option<i32> {
        None
    }

    fn global(&self, _id: &RecordId) -> Option<GlobalNumber> {
        None
    }

    fn local(&self, _speaker: ActorKey, name: &RecordId) -> Option<f32> {
        self.locals.get(name).copied()
    }

    fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.world.quests.stage(quest)
    }

    fn item_count(&self, _player: PlayerId, item: &RecordId) -> u64 {
        self.items.get(item).copied().unwrap_or(0)
    }

    fn dead_count(&self, _base: &RecordId) -> i32 {
        0
    }

    fn disposition(&self, _ctx: &ConversationCtx) -> i32 {
        self.disposition
    }

    fn rank_name(&self, _faction: &RecordId, _rank: i32) -> Option<String> {
        None
    }
}

fn ctx(speaker: &str) -> ConversationCtx {
    ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(7),
        speaker_base: id(speaker),
        choice: None,
    }
}

fn line(out: &[DialogueOutput]) -> (String, String) {
    out.iter()
        .find_map(|o| match o {
            DialogueOutput::Line { response, text, .. } => Some((
                response
                    .as_str()
                    .rsplit('\u{1f}')
                    .next()
                    .unwrap()
                    .to_owned(),
                text.clone(),
            )),
            _ => None,
        })
        .expect("a line")
}

#[test]
fn greetings_in_order_with_macros_and_links() {
    let dlg = data();
    assert_eq!(dlg.db.greetings(), [id("greeting 1"), id("greeting 5")]);
    let view = View::new("fargoth");
    let rules = MwDialogueRules::new(&dlg, &view);
    let (_, out) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
    assert_eq!(
        line(&out),
        ("g5".into(), "Hello Nerevar, they took my ring!".into())
    );
    assert!(out.contains(&DialogueOutput::Learned(id("ring"))));
    assert!(rules.speaker_answers(&ctx("fargoth"), &id("ring")));

    let guard = View::new("guard");
    let rules = MwDialogueRules::new(&dlg, &guard);
    let (_, out) = DialogueSession::open(&dlg.db, &rules, ctx("guard"));
    assert_eq!(line(&out).0, "g1");
    // The guard has no `ring` response: a host must not learn it from his text.
    assert!(!rules.speaker_answers(&ctx("guard"), &id("ring")));
}

#[test]
fn locals_choice_items_and_neutral_functions() {
    let dlg = data();
    let mut view = View::new("fargoth");
    let ring = id("ring");
    {
        let rules = MwDialogueRules::new(&dlg, &view);
        let (mut s, _) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
        // No `state` local: `Local` fails; `NotLocal missing` passes; no choice is -1.
        assert_eq!(line(&s.ask(&dlg.db, &rules, &ring).unwrap()).0, "r4");
        assert_eq!(rules.neutral_functions(), [FilterFunction::PcHealthPercent]);
        s.offer_choices(vec![Choice {
            label: "No".into(),
            value: 1,
        }])
        .unwrap();
        assert_eq!(line(&s.choose(&dlg.db, &rules, 1).unwrap()).0, "r2");
    }
    view.items.insert(id("ring_keley"), 1);
    {
        let rules = MwDialogueRules::new(&dlg, &view);
        let (mut s, _) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
        assert_eq!(line(&s.ask(&dlg.db, &rules, &ring).unwrap()).0, "r3");
    }
    view.locals.insert(id("state"), -1.0);
    let rules = MwDialogueRules::new(&dlg, &view);
    let (mut s, _) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
    assert_eq!(line(&s.ask(&dlg.db, &rules, &ring).unwrap()).0, "r1");
}

#[test]
fn disposition_falls_back_to_info_refusal() {
    let dlg = data();
    let mut view = View::new("fargoth");
    let secret = id("secret");
    let mut known = Topics::default();
    known.learn(secret.clone());
    {
        let rules = MwDialogueRules::new(&dlg, &view);
        let (mut s, _) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
        assert_eq!(
            s.available_topics(&dlg.db, &rules, &known),
            std::slice::from_ref(&secret)
        );
        let out = s.ask(&dlg.db, &rules, &secret).unwrap();
        assert_eq!(
            line(&out),
            ("x1".into(), "I don't trust you, Nerevar.".into())
        );
    }
    view.disposition = 60;
    let rules = MwDialogueRules::new(&dlg, &view);
    let (mut s, _) = DialogueSession::open(&dlg.db, &rules, ctx("fargoth"));
    assert_eq!(line(&s.ask(&dlg.db, &rules, &secret).unwrap()).0, "s1");
}

#[test]
fn cell_filter_is_a_player_cell_prefix() {
    let dlg = data();
    let mut view = View::new("fargoth");
    let topic = id("seyda neen");
    {
        let rules = MwDialogueRules::new(&dlg, &view);
        assert!(dlg.db.select(&rules, &ctx("fargoth"), &topic).is_some());
    }
    view.player.cell = "Balmora".into();
    let rules = MwDialogueRules::new(&dlg, &view);
    assert!(dlg.db.select(&rules, &ctx("fargoth"), &topic).is_none());
}

#[test]
fn journal_entries_once_per_index_and_stage_only_rises() {
    let dlg = data();
    let view = View::new("fargoth");
    let rules = MwDialogueRules::new(&dlg, &view);
    let q = id("q");
    assert_eq!(
        dlg.journal.quest(&q).unwrap().name.as_deref(),
        Some("The Ring Quest")
    );
    let mut world = WorldRpg::default();
    let journal = |world: &mut WorldRpg, index| {
        let cmd = dlg.journal_command(world, &q, index, GameTime::default(), |t| {
            rules.expand(&ctx("fargoth"), t)
        });
        if let Some(cmd) = cmd.clone() {
            world.apply(cmd).unwrap();
        }
        cmd
    };
    let Some(RpgCommand::SetStage {
        entry: Some(entry), ..
    }) = journal(&mut world, 10)
    else {
        panic!("entry")
    };
    assert_eq!(entry.text, "fargoth lost a ring.");
    assert_eq!(journal(&mut world, 10), None);
    assert!(journal(&mut world, 100).is_some());
    assert!(world.quests.get(&q).finished);
    assert_eq!(journal(&mut world, 10), None);
    assert_eq!(world.quests.stage(&q), 100);
    assert_eq!(world.journal.entries().len(), 2);
    // An index with no INFO raises the stage without an entry.
    let Some(RpgCommand::SetStage {
        stage: 150,
        entry: None,
        ..
    }) = journal(&mut world, 150)
    else {
        panic!("raise")
    };
}
