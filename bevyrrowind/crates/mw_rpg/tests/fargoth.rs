//! Phase 2 acceptance flow (rpg-adapter.md "Acceptance flow"): talk to Fargoth, his ring,
//! journal entries, then a postcard round trip of the RPG state. Result scripts run in the
//! game's runner ([`MwResults`], `mw_script`'s interpreter) over the host below.
//!
//! - `fargoths_ring`: the real data; skips (passes) when no Morrowind install is detected.
//! - `synthetic_ring_quest`: the same flow on a DIAL/INFO fixture of our own (same filter
//!   and result shapes, our text and ids), so it runs everywhere.

use std::collections::{BTreeMap, HashMap};

use mw_install::MorrowindInstall;
use mw_rpg::{
    ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwResults, MwView, MwWorld, NpcStates,
    ResultCall, ResultEffect, ResultRunner, ResultState, ScriptVar, Target, VarType, rank_name,
    response_id,
};
use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, ConversationCtx, DialogueOutput, DialogueSession, GameTime,
    Holder, PlayerId, RpgCommand, SessionState, WorldRpg,
};
use sf_vfs::Vfs;
use tes3::esp::{
    Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison, FilterFunction, FilterType,
    FilterValue, GlobalValue, GlobalVariable, Npc, QuestState, Script,
};

const SPEAKER: ActorKey = ActorKey(1);
const FRIEND: ActorKey = ActorKey(2);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// The ids one run of the flow checks.
struct Flow {
    speaker: &'static str,
    /// The other NPC whose disposition the final result changes.
    friend: &'static str,
    topic: &'static str,
    quest: &'static str,
    item: &'static str,
    greeting: (&'static str, &'static str),
    /// INFO ids under `topic`: thanks (afterwards), wrong item (choice 2 without it),
    /// returned (choice 2 with it), first ask.
    thanks: &'static str,
    wrong: &'static str,
    returned: &'static str,
    first: &'static str,
}

const FARGOTH: Flow = Flow {
    speaker: "fargoth",
    friend: "arrille",
    topic: "ring",
    quest: "ms_fargothring",
    item: "ring_keley",
    greeting: ("greeting 5", "31191105114705102"),
    thanks: "12942106023022017709",
    wrong: "158791326429969956",
    returned: "3245423813243151497",
    first: "2507536232473510507",
};

/// Saved RPG state (what the round trip covers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Saved {
    world: WorldRpg,
    pc: CharacterRpg,
    speaker: CharacterRpg,
    npcs: NpcStates,
    locals: BTreeMap<(ActorKey, RecordId), f32>,
    globals: BTreeMap<RecordId, f32>,
}

struct Host<'a> {
    records: &'a RecordDb,
    friend: RecordId,
    speaker: ActorFacts,
    player: ActorFacts,
    base_disposition: i32,
    /// GLOB records (type and initial value); current values are in `Saved::globals`.
    global_types: HashMap<RecordId, GlobalNumber>,
    /// The speaker script's declared locals (default 0).
    declared: HashMap<RecordId, VarType>,
    /// Message boxes and sounds the results asked for (UI and audio events, not saved).
    events: Vec<ResultEffect>,
    s: Saved,
}

impl Host<'_> {
    fn global_value(&self, id: &RecordId) -> Option<GlobalNumber> {
        let ty = *self.global_types.get(id)?;
        Some(match (ty, self.s.globals.get(id)) {
            #[allow(clippy::cast_possible_truncation)]
            (GlobalNumber::Int(_), Some(v)) => GlobalNumber::Int(*v as i32),
            (GlobalNumber::Float(_), Some(v)) => GlobalNumber::Float(*v),
            (initial, None) => initial,
        })
    }

    fn speaker_local(&self, name: &RecordId) -> Option<f32> {
        self.declared.contains_key(name).then(|| {
            self.s
                .locals
                .get(&(SPEAKER, name.clone()))
                .copied()
                .unwrap_or(0.0)
        })
    }
}

impl MwView for Host<'_> {
    fn speaker(&self, _ctx: &ConversationCtx) -> Option<&ActorFacts> {
        Some(&self.speaker)
    }

    fn player(&self, _player: PlayerId) -> Option<&ActorFacts> {
        Some(&self.player)
    }

    fn player_rank(&self, _player: PlayerId, _faction: &RecordId) -> Option<i32> {
        None
    }

    fn global(&self, id: &RecordId) -> Option<GlobalNumber> {
        self.global_value(id)
    }

    fn local(&self, speaker: ActorKey, name: &RecordId) -> Option<f32> {
        (speaker == SPEAKER)
            .then(|| self.speaker_local(name))
            .flatten()
    }

    fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.s.world.quests.stage(quest)
    }

    fn item_count(&self, _player: PlayerId, item: &RecordId) -> u64 {
        self.s.pc.loadout.count(item)
    }

    fn dead_count(&self, _base: &RecordId) -> i32 {
        0
    }

    fn disposition(&self, ctx: &ConversationCtx) -> i32 {
        self.s.npcs.disposition(ctx.speaker, self.base_disposition)
    }

    fn rank_name(&self, faction: &RecordId, rank: i32) -> Option<String> {
        rank_name(self.records, faction, rank)
    }

    /// A fresh character's values for the functions the greetings check (stats arrive in
    /// M7); everything else is the rules' neutral 0.
    fn function(&self, _ctx: &ConversationCtx, function: FilterFunction) -> Option<f32> {
        match function {
            // Starting clothes (common shirt, pants, shoes) are worth a few drakes.
            FilterFunction::PcClothingModifier => Some(10.0),
            FilterFunction::PcHealthPercent => Some(100.0),
            FilterFunction::PcLevel => Some(1.0),
            _ => None,
        }
    }
}

/// The world test double: globals, the speaker's declared locals, the speaker's items.
impl MwWorld for Host<'_> {
    fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.global_value(name)
    }

    fn var(&self, _ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        match target {
            Target::Speaker => Some(ScriptVar {
                ty: *self.declared.get(name)?,
                value: self.speaker_local(name)?,
            }),
            _ => None,
        }
    }

    fn item_count(&self, _ctx: &ConversationCtx, target: &Target, item: &RecordId) -> u64 {
        match target {
            Target::Speaker => self.s.speaker.loadout.count(item),
            _ => 0,
        }
    }
}

impl Host<'_> {
    fn character(&mut self, target: &Target) -> (Holder, &mut CharacterRpg) {
        match target {
            Target::Speaker => (Holder::Actor(SPEAKER), &mut self.s.speaker),
            Target::Player => (Holder::Player(PlayerId::HOST), &mut self.s.pc),
            Target::Actor(a) => panic!("item command on {a}"),
        }
    }

    fn actor(&self, target: &Target) -> ActorKey {
        match target {
            Target::Speaker => SPEAKER,
            Target::Actor(a) if *a == self.friend => FRIEND,
            other => panic!("unexpected target {other:?}"),
        }
    }

    /// Applies one effect (`Journal` is turned into a command by the caller).
    fn apply(&mut self, session: &mut DialogueSession, effect: ResultEffect) {
        match effect {
            ResultEffect::AddTopic(t) => {
                self.s.world.apply(RpgCommand::AddTopic(t)).unwrap();
            }
            ResultEffect::AddItem {
                target,
                item,
                count,
            } => {
                let (holder, inv) = self.character(&target);
                inv.apply(RpgCommand::AddItem {
                    holder,
                    item,
                    count,
                    attrs: Attrs::new(),
                })
                .unwrap();
            }
            ResultEffect::RemoveItem {
                target,
                item,
                count,
            } => {
                let (holder, inv) = self.character(&target);
                inv.apply(RpgCommand::RemoveItem {
                    holder,
                    item,
                    count,
                })
                .unwrap();
            }
            ResultEffect::SetJournalIndex { quest, index } => {
                self.s
                    .world
                    .apply(MwDialogue::set_journal_index(&quest, index))
                    .unwrap();
            }
            ResultEffect::Choices(choices) => session.offer_choices(choices).unwrap(),
            ResultEffect::Goodbye => session.force_goodbye(),
            ResultEffect::ModDisposition { target, delta } => {
                self.s.npcs.modify(self.actor(&target), delta);
            }
            ResultEffect::SetDisposition { target, value } => {
                let base = self.base_disposition;
                self.s.npcs.set(self.actor(&target), base, value);
            }
            ResultEffect::SetLocal {
                target,
                name,
                value,
            } => {
                self.s.locals.insert((self.actor(&target), name), value);
            }
            ResultEffect::SetGlobal { name, value } => {
                self.s.globals.insert(name, value);
            }
            e @ (ResultEffect::MessageBox { .. } | ResultEffect::PlaySound(_)) => {
                self.events.push(e);
            }
            ResultEffect::Journal { .. } => unreachable!("handled by the caller"),
        }
    }
}

/// Host step after a session call: learn linked topics the speaker answers, run results
/// in the game's runner, apply their effects in call order.
fn after(
    dlg: &MwDialogue,
    results: &mut MwResults,
    host: &mut Host<'_>,
    session: &mut DialogueSession,
    out: &[DialogueOutput],
) -> RecordId {
    let ctx = session.ctx().clone();
    let mut learned = Vec::new();
    let mut effects = Vec::new();
    let mut shown = None;
    {
        let rules = MwDialogueRules::new(dlg, &*host);
        let state = ResultState {
            world: &host.s.world,
            player: &host.s.pc,
            env: &*host,
        };
        let mut runner = results.runner(state);
        for o in out {
            match o {
                DialogueOutput::Line { response, .. } => shown = Some(response.clone()),
                DialogueOutput::Learned(t) if rules.speaker_answers(&ctx, t) => {
                    learned.push(RpgCommand::AddTopic(t.clone()));
                }
                DialogueOutput::RunResult { topic, response } => {
                    let script = &dlg.info(response).unwrap().script_text;
                    let call = ResultCall {
                        ctx: &ctx,
                        topic,
                        response,
                        script,
                    };
                    effects.extend(runner.run(call).unwrap());
                }
                _ => {}
            }
        }
        eprintln!(
            "neutral filter functions so far: {:?}",
            rules.neutral_functions()
        );
    }
    for report in results.drain_reports() {
        eprintln!("{report}");
    }
    for cmd in learned {
        host.s.world.apply(cmd).unwrap();
    }
    for effect in effects {
        if let ResultEffect::Journal { quest, index } = &effect {
            let cmd = {
                let rules = MwDialogueRules::new(dlg, &*host);
                dlg.journal_command(&host.s.world, quest, *index, GameTime::default(), |t| {
                    rules.expand(&ctx, t)
                })
            };
            if let Some(cmd) = cmd {
                host.s.world.apply(cmd).unwrap();
            }
        } else {
            host.apply(session, effect);
        }
    }
    shown.unwrap_or_else(|| panic!("no line in {out:?}"))
}

/// The speaker script's declared locals (`short state` ...), from its source text.
fn declared_locals(text: &str) -> HashMap<RecordId, VarType> {
    let parsed = mw_script::parse(
        text,
        mw_script::SourceKind::Script,
        mw_script::FunctionTable::vanilla(),
    );
    parsed
        .script
        .locals
        .iter()
        .map(|l| (id(&l.name.key), l.ty))
        .collect()
}

/// Greeting → ask → wrong choice → ask with the item → returned → thanks → round trip.
#[allow(clippy::too_many_lines)]
fn run_flow<'a>(flow: &Flow, dlg: &MwDialogue, mut host: Host<'a>) -> Host<'a> {
    let mut results = MwResults::new();
    let topic = id(flow.topic);
    let quest = id(flow.quest);
    let item = id(flow.item);
    let resp = |info: &str| response_id(flow.topic, info);
    let ctx = ConversationCtx {
        player: PlayerId::HOST,
        speaker: SPEAKER,
        speaker_base: id(flow.speaker),
        choice: None,
    };

    // The greeting, which links and teaches the topic.
    let (mut session, out) = {
        let rules = MwDialogueRules::new(dlg, &host);
        let opened = DialogueSession::open(&dlg.db, &rules, ctx);
        eprintln!(
            "neutral filter functions in the greeting search: {:?}",
            rules.neutral_functions()
        );
        opened
    };
    let greeting = after(dlg, &mut results, &mut host, &mut session, &out);
    assert_eq!(
        greeting,
        response_id(flow.greeting.0, flow.greeting.1),
        "{:#?}",
        dlg.info(&greeting)
    );
    assert!(host.s.world.topics.knows(&topic));

    let ask = |results: &mut MwResults, host: &mut Host<'_>, session: &mut DialogueSession| {
        let out = session
            .ask(&dlg.db, &MwDialogueRules::new(dlg, &*host), &topic)
            .unwrap();
        after(dlg, results, host, session, &out)
    };
    let choose =
        |results: &mut MwResults, host: &mut Host<'_>, session: &mut DialogueSession, value| {
            let out = session
                .choose(&dlg.db, &MwDialogueRules::new(dlg, &*host), value)
                .unwrap();
            after(dlg, results, host, session, &out)
        };

    // First ask: stage 10, first entry, choices.
    assert_eq!(ask(&mut results, &mut host, &mut session), resp(flow.first));
    assert_eq!(host.s.world.quests.stage(&quest), 10);
    assert_eq!(host.s.world.journal.entries().len(), 1);
    assert!(matches!(session.state(), SessionState::Choosing(c) if c.len() == 2));

    // "Yes, here" without the item.
    assert_eq!(
        choose(&mut results, &mut host, &mut session, 2),
        resp(flow.wrong)
    );
    assert_eq!(host.s.npcs.modifier(SPEAKER), -5);

    // With the item: the first response again (no second stage-10 entry), then returned.
    host.s
        .pc
        .loadout
        .add(item.clone(), 1, Attrs::new())
        .unwrap();
    assert_eq!(ask(&mut results, &mut host, &mut session), resp(flow.first));
    assert_eq!(host.s.world.journal.entries().len(), 1);
    assert_eq!(
        choose(&mut results, &mut host, &mut session, 2),
        resp(flow.returned)
    );
    let state = host.s.world.quests.get(&quest);
    assert_eq!(state.stage, 100);
    let finishes = dlg
        .journal
        .stage(&quest, 100)
        .is_some_and(|s| s.flags.finish);
    assert_eq!(
        state.finished, finishes,
        "finished flag comes from QSTF (expansions)"
    );
    eprintln!(
        "quest name: {:?}, finished: {}",
        dlg.journal.quest(&quest).and_then(|q| q.name.clone()),
        state.finished
    );
    assert_eq!(host.s.pc.loadout.count(&item), 0);
    assert_eq!(host.s.speaker.loadout.count(&item), 1);
    assert!((host.s.locals[&(SPEAKER, id("state"))] + 1.0).abs() < f32::EPSILON);
    assert_eq!(host.s.npcs.modifier(SPEAKER), 45);
    assert_eq!(host.s.npcs.modifier(FRIEND), 40);
    let entries = host.s.world.journal.entries();
    assert_eq!(
        entries.iter().map(|e| e.stage).collect::<Vec<_>>(),
        [Some(10), Some(100)]
    );
    assert!(entries.iter().all(|e| !e.text.contains('%')), "{entries:?}");

    // Afterwards: thanks.
    assert_eq!(
        ask(&mut results, &mut host, &mut session),
        resp(flow.thanks)
    );

    // Save/load round trip.
    let bytes = postcard::to_stdvec(&host.s).unwrap();
    let back: Saved = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, host.s);
    eprintln!("saved RPG state: {} bytes", bytes.len());
    host
}

fn saved() -> Saved {
    Saved {
        world: WorldRpg::default(),
        pc: CharacterRpg::default(),
        speaker: CharacterRpg::default(),
        npcs: NpcStates::default(),
        locals: BTreeMap::new(),
        globals: BTreeMap::new(),
    }
}

fn player() -> ActorFacts {
    ActorFacts {
        id: id("player"),
        name: "Nerevar".into(),
        npc: true,
        rank: -1,
        cell: "Seyda Neen".into(),
        ..ActorFacts::default()
    }
}

#[test]
fn fargoths_ring() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let records = &content.db;
    let dlg = MwDialogue::from_records(records).unwrap();
    eprintln!(
        "{} topics, {} quests",
        dlg.db.topics().count(),
        dlg.journal.len()
    );

    let npc: &Npc = records.get(&id(FARGOTH.speaker)).expect("fargoth");
    let script: &Script = records.get(&id(&npc.script)).expect("fargoth's script");
    let declared = declared_locals(&script.text);
    assert_eq!(declared.get(&id("state")), Some(&VarType::Short));
    let global_types = records
        .iter::<GlobalVariable>()
        .map(|(gid, g)| {
            let v = match g.value {
                GlobalValue::Float(f) => GlobalNumber::Float(f),
                GlobalValue::Long(l) => GlobalNumber::Int(l),
                GlobalValue::Short(s) => GlobalNumber::Int(i32::from(s)),
            };
            (gid.clone(), v)
        })
        .collect();
    let host = Host {
        records,
        friend: id(FARGOTH.friend),
        speaker: ActorFacts::from_npc(records, npc, "Seyda Neen"),
        player: player(),
        base_disposition: i32::from(npc.data.disposition),
        global_types,
        declared,
        events: Vec::new(),
        s: saved(),
    };
    let host = run_flow(&FARGOTH, &dlg, host);
    eprintln!("result events: {:?}", host.events);
}

// --- synthetic fixture ------------------------------------------------------------------

const SYNTHETIC: Flow = Flow {
    speaker: "test_bosmer",
    friend: "test_friend",
    topic: "lost ring",
    quest: "q_lost_ring",
    item: "ring_test",
    greeting: ("greeting 5", "g5"),
    thanks: "r1",
    wrong: "r3",
    returned: "r4",
    first: "r5",
};

fn dial(name: &str, kind: DialogueType2) -> Dialogue {
    Dialogue {
        id: name.into(),
        dialogue_type: kind,
        ..Dialogue::default()
    }
}

fn info(id: &str, prev: &str, text: &str, filters: Vec<Filter>, script: &str) -> DialogueInfo {
    let mut info = DialogueInfo {
        id: id.into(),
        prev_id: prev.into(),
        text: text.into(),
        filters,
        script_text: script.into(),
        ..DialogueInfo::default()
    };
    // -1 is "no rank filter" in the data (the type defaults to 0).
    info.data.speaker_rank = -1;
    info.data.player_rank = -1;
    info
}

fn said(mut info: DialogueInfo) -> DialogueInfo {
    info.speaker_id = SYNTHETIC.speaker.into();
    info
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

fn choice_is(value: i32) -> Filter {
    let mut f = filter(FilterType::Function, "", FilterComparison::Equal, value);
    f.function = FilterFunction::Choice;
    f
}

fn stage(id: &str, prev: &str, index: i32, text: &str, state: Option<QuestState>) -> DialogueInfo {
    let mut i = info(id, prev, text, Vec::new(), "");
    i.data.disposition = index;
    i.quest_state = state;
    i
}

/// Fargoth's flow re-authored: same filters and result calls, our own text and ids.
fn synthetic_dialogue() -> MwDialogue {
    use FilterComparison::{Equal, Greater};
    use FilterType::{Item, Journal, Local};
    let (q, ring) = (SYNTHETIC.quest, SYNTHETIC.item);
    let returned = format!(
        "Journal {q} 100\nAddItem \"{ring}\" 1\nplayer->RemoveItem \"{ring}\" 1\n\
         set state to -1\nModDisposition 50\n\"{}\"->ModDisposition 40\n\
         set test_reward_given to test_reward_given + 1\n\
         if ( GetJournalIndex {q} >= 100 )\nPlaySound \"Item Ring Up\"\nendif\n\
         MessageBox \"Disposition +%g\" 50",
        SYNTHETIC.friend
    );
    let first = format!(
        "Journal {q} 10\nChoice \"No, sorry.\" 1 \"Yes, here it is.\" 2 ; asks\n\
         ModPCFacRep 1 \"test_guild\""
    );
    MwDialogue::from_topics([
        (
            dial("Greeting 5", DialogueType2::Greeting),
            vec![said(info(
                "g5",
                "",
                "Someone took my lost ring, %PCName!",
                vec![filter(Journal, q, Equal, 0)],
                "",
            ))],
        ),
        (
            dial(SYNTHETIC.topic, DialogueType2::Topic),
            vec![
                said(info(
                    "r1",
                    "",
                    "You found it, thank you again.",
                    vec![filter(Local, "state", Equal, -1)],
                    "",
                )),
                said(info(
                    "r2",
                    "r1",
                    "Tell me if you see it.",
                    vec![choice_is(1)],
                    "",
                )),
                said(info(
                    "r3",
                    "r2",
                    "That is not my ring.",
                    vec![choice_is(2), filter(Item, ring, Equal, 0)],
                    "ModDisposition -5",
                )),
                said(info(
                    "r4",
                    "r3",
                    "My ring! Thank you.",
                    vec![choice_is(2), filter(Item, ring, Greater, 0)],
                    &returned,
                )),
                said(info(
                    "r5",
                    "r4",
                    "A guard has it, I am sure. Have you seen it?",
                    Vec::new(),
                    &first,
                )),
            ],
        ),
        (
            dial(q, DialogueType2::Journal),
            vec![
                stage("j0", "", 0, "The Lost Ring", Some(QuestState::Name)),
                stage("j10", "j0", 10, "%Name has lost a ring.", None),
                stage(
                    "j100",
                    "j10",
                    100,
                    "I returned the ring.",
                    Some(QuestState::Finished),
                ),
            ],
        ),
    ])
    .unwrap()
}

#[test]
fn synthetic_ring_quest() {
    let records = RecordDb::new();
    let dlg = synthetic_dialogue();
    let host = Host {
        records: &records,
        friend: id(SYNTHETIC.friend),
        speaker: ActorFacts {
            id: id(SYNTHETIC.speaker),
            name: "Test Bosmer".into(),
            npc: true,
            cell: "Test Town".into(),
            ..ActorFacts::default()
        },
        player: player(),
        base_disposition: 40,
        global_types: HashMap::from([(id("test_reward_given"), GlobalNumber::Int(0))]),
        declared: declared_locals("Begin test_bosmer_script\nshort state\nEnd"),
        events: Vec::new(),
        s: saved(),
    };
    let host = run_flow(&SYNTHETIC, &dlg, host);
    assert_eq!(host.s.globals.get(&id("test_reward_given")), Some(&1.0));
    assert!(host.s.world.quests.get(&id(SYNTHETIC.quest)).finished);
    assert_eq!(
        host.events,
        [
            ResultEffect::PlaySound(id("item ring up")),
            ResultEffect::MessageBox {
                text: "Disposition +50".into(),
                buttons: vec![]
            }
        ]
    );
    let entries = host.s.world.journal.entries();
    assert_eq!(entries[0].text, "Test Bosmer has lost a ring.");
}
