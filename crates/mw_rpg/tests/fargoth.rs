//! Phase 2 acceptance flow on the real data (rpg-adapter.md "Acceptance flow"): talk to
//! Fargoth, his ring, journal entries, then a postcard round trip of the RPG state.
//! Skips (passes) when no Morrowind install is detected.

use std::collections::{BTreeMap, HashMap};

use mw_install::MorrowindInstall;
use mw_rpg::{
    ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwView, NpcStates, ResultCall,
    ResultEffect, ResultError, ResultRunner, Target, rank_name, response_id,
};
use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, Choice, ConversationCtx, DialogueOutput, DialogueSession,
    GameTime, Holder, PlayerId, RpgCommand, SessionState, WorldRpg,
};
use sf_vfs::Vfs;
use tes3::esp::{FilterFunction, GlobalValue, GlobalVariable, Npc};

const FARGOTH: ActorKey = ActorKey(1);
const ARRILLE: ActorKey = ActorKey(2);
const QUEST: &str = "ms_fargothring";

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Test double for the result runtime: parses the handful of `MWScript` calls Fargoth's
/// results use. Not a scaffold: the game runs results in the Luau runtime (W3).
struct FargothResultsTestDouble;

impl FargothResultsTestDouble {
    fn tokens(line: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut quoted = false;
        for c in line.chars() {
            match c {
                '"' => {
                    if quoted {
                        out.push(std::mem::take(&mut cur));
                    }
                    quoted = !quoted;
                }
                c if !quoted && (c.is_whitespace() || c == ',') => {
                    if !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        out
    }

    fn effect(line: &str) -> Option<ResultEffect> {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            return None;
        }
        let (target, call) = match line.split_once("->") {
            Some((who, call)) => {
                let who = who.trim().trim_matches('"');
                (
                    if who.eq_ignore_ascii_case("player") {
                        Target::Player
                    } else {
                        Target::Actor(id(who))
                    },
                    call,
                )
            }
            None => (Target::Speaker, line),
        };
        let t = Self::tokens(call);
        let int = |i: usize| {
            t[i].parse::<i32>()
                .unwrap_or_else(|_| panic!("number in {line:?}"))
        };
        let count = |i: usize| u32::try_from(int(i)).unwrap();
        Some(match t[0].to_ascii_lowercase().as_str() {
            "journal" => ResultEffect::Journal {
                quest: id(&t[1]),
                index: int(2),
            },
            "addtopic" => ResultEffect::AddTopic(id(&t[1])),
            "additem" => ResultEffect::AddItem {
                target,
                item: id(&t[1]),
                count: count(2),
            },
            "removeitem" => ResultEffect::RemoveItem {
                target,
                item: id(&t[1]),
                count: count(2),
            },
            "goodbye" => ResultEffect::Goodbye,
            "moddisposition" => ResultEffect::ModDisposition {
                target,
                delta: int(1),
            },
            "set" => ResultEffect::SetLocal {
                target,
                name: id(&t[1]),
                value: t[3].parse().unwrap(),
            },
            "choice" => ResultEffect::Choices(
                t[1..]
                    .chunks(2)
                    .map(|p| Choice {
                        label: p[0].clone(),
                        value: p[1].parse().unwrap(),
                    })
                    .collect(),
            ),
            other => ResultEffect::Stub(other.to_owned()),
        })
    }
}

impl ResultRunner for FargothResultsTestDouble {
    fn run(&mut self, call: ResultCall<'_>) -> Result<Vec<ResultEffect>, ResultError> {
        Ok(call.script.lines().filter_map(Self::effect).collect())
    }
}

/// Saved RPG state (what the round trip covers).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Saved {
    world: WorldRpg,
    pc: CharacterRpg,
    fargoth: CharacterRpg,
    npcs: NpcStates,
    locals: BTreeMap<(ActorKey, RecordId), f32>,
}

struct Host<'a> {
    records: &'a RecordDb,
    speaker: ActorFacts,
    player: ActorFacts,
    base_disposition: i32,
    globals: HashMap<RecordId, GlobalNumber>,
    s: Saved,
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
        self.globals.get(id).copied()
    }

    fn local(&self, speaker: ActorKey, name: &RecordId) -> Option<f32> {
        self.s.locals.get(&(speaker, name.clone())).copied()
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

impl Host<'_> {
    fn holder(target: &Target) -> Holder {
        match target {
            Target::Speaker => Holder::Actor(FARGOTH),
            Target::Player => Holder::Player(PlayerId::HOST),
            Target::Actor(a) => panic!("item command on {a}"),
        }
    }

    fn actor(target: &Target) -> ActorKey {
        match target {
            Target::Speaker => FARGOTH,
            Target::Actor(a) if a.as_str() == "arrille" => ARRILLE,
            other => panic!("unexpected target {other:?}"),
        }
    }

    fn apply(
        &mut self,
        session: &mut DialogueSession,
        commands: Vec<RpgCommand>,
        effects: Vec<ResultEffect>,
    ) {
        for cmd in commands {
            self.s.world.apply(cmd).unwrap();
        }
        for effect in effects {
            match effect {
                ResultEffect::AddTopic(t) => {
                    self.s.world.apply(RpgCommand::AddTopic(t)).unwrap();
                }
                ResultEffect::AddItem {
                    target,
                    item,
                    count,
                } => {
                    let holder = Self::holder(&target);
                    let inv = if holder == Holder::Actor(FARGOTH) {
                        &mut self.s.fargoth
                    } else {
                        &mut self.s.pc
                    };
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
                    let holder = Self::holder(&target);
                    let inv = if holder == Holder::Actor(FARGOTH) {
                        &mut self.s.fargoth
                    } else {
                        &mut self.s.pc
                    };
                    inv.apply(RpgCommand::RemoveItem {
                        holder,
                        item,
                        count,
                    })
                    .unwrap();
                }
                ResultEffect::Choices(choices) => session.offer_choices(choices).unwrap(),
                ResultEffect::Goodbye => session.force_goodbye(),
                ResultEffect::ModDisposition { target, delta } => {
                    self.s.npcs.modify(Self::actor(&target), delta);
                }
                ResultEffect::SetLocal {
                    target,
                    name,
                    value,
                } => {
                    self.s.locals.insert((Self::actor(&target), name), value);
                }
                other => panic!("unexpected effect {other:?}"),
            }
        }
    }
}

/// Host step after a session call: learn linked topics the speaker answers, run results.
fn after(
    dlg: &MwDialogue,
    host: &mut Host<'_>,
    session: &mut DialogueSession,
    out: &[DialogueOutput],
) -> RecordId {
    let mut commands = Vec::new();
    let mut effects = Vec::new();
    let mut shown = None;
    {
        let rules = MwDialogueRules::new(dlg, &*host);
        for o in out {
            match o {
                DialogueOutput::Line { response, .. } => shown = Some(response.clone()),
                DialogueOutput::Learned(t) if rules.speaker_answers(session.ctx(), t) => {
                    commands.push(RpgCommand::AddTopic(t.clone()));
                }
                DialogueOutput::RunResult { topic, response } => {
                    let script = &dlg.info(response).unwrap().script_text;
                    let call = ResultCall {
                        ctx: session.ctx(),
                        topic,
                        response,
                        script,
                    };
                    for effect in FargothResultsTestDouble.run(call).unwrap() {
                        if let ResultEffect::Journal { quest, index } = effect {
                            let render = |t: &str| rules.expand(session.ctx(), t);
                            commands.extend(dlg.journal_command(
                                &host.s.world,
                                &quest,
                                index,
                                GameTime::default(),
                                render,
                            ));
                        } else {
                            effects.push(effect);
                        }
                    }
                }
                _ => {}
            }
        }
        eprintln!(
            "neutral filter functions so far: {:?}",
            rules.neutral_functions()
        );
    }
    host.apply(session, commands, effects);
    shown.unwrap_or_else(|| panic!("no line in {out:?}"))
}

#[test]
#[allow(clippy::too_many_lines)]
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

    let npc: &Npc = records.get(&id("fargoth")).expect("fargoth");
    let speaker = ActorFacts::from_npc(records, npc, "Seyda Neen");
    let player = ActorFacts {
        id: id("player"),
        name: "Nerevar".into(),
        npc: true,
        rank: -1,
        cell: "Seyda Neen".into(),
        ..ActorFacts::default()
    };
    let globals = records
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
    let mut locals = BTreeMap::new();
    locals.insert((FARGOTH, id("state")), 0.0); // `LocalState` declares `short state`
    let saved = Saved {
        world: WorldRpg::default(),
        pc: CharacterRpg::default(),
        fargoth: CharacterRpg::default(),
        npcs: NpcStates::default(),
        locals,
    };
    let mut host = Host {
        records,
        speaker,
        player,
        base_disposition: i32::from(npc.data.disposition),
        globals,
        s: saved,
    };

    let ring = id("ring");
    let quest = id(QUEST);
    let ring_item = id("ring_keley");
    let resp = |topic: &str, info: &str| response_id(topic, info);
    let ctx = ConversationCtx {
        player: PlayerId::HOST,
        speaker: FARGOTH,
        speaker_base: id("fargoth"),
        choice: None,
    };

    // Greeting 5, which links and teaches `ring`.
    let (mut session, out) = {
        let rules = MwDialogueRules::new(&dlg, &host);
        let opened = DialogueSession::open(&dlg.db, &rules, ctx);
        eprintln!(
            "neutral filter functions in the greeting search: {:?}",
            rules.neutral_functions()
        );
        opened
    };
    let greeting = after(&dlg, &mut host, &mut session, &out);
    assert_eq!(
        greeting,
        resp("greeting 5", "31191105114705102"),
        "{:#?}",
        dlg.info(&greeting)
    );
    assert!(host.s.world.topics.knows(&ring));

    // Ask `ring`: response 5, stage 10, first entry, choices.
    let ask = |host: &mut Host<'_>, session: &mut DialogueSession| {
        let out = session
            .ask(&dlg.db, &MwDialogueRules::new(&dlg, &*host), &ring)
            .unwrap();
        after(&dlg, host, session, &out)
    };
    let choose = |host: &mut Host<'_>, session: &mut DialogueSession, value| {
        let out = session
            .choose(&dlg.db, &MwDialogueRules::new(&dlg, &*host), value)
            .unwrap();
        after(&dlg, host, session, &out)
    };
    assert_eq!(
        ask(&mut host, &mut session),
        resp("ring", "2507536232473510507")
    );
    assert_eq!(host.s.world.quests.stage(&quest), 10);
    assert_eq!(host.s.world.journal.entries().len(), 1);
    assert!(matches!(session.state(), SessionState::Choosing(c) if c.len() == 2));

    // "Yes, here" without the ring.
    assert_eq!(
        choose(&mut host, &mut session, 2),
        resp("ring", "158791326429969956")
    );
    assert_eq!(host.s.npcs.modifier(FARGOTH), -5);

    // With the ring: response 5 again (no second stage-10 entry), then response 4.
    host.s
        .pc
        .loadout
        .add(ring_item.clone(), 1, Attrs::new())
        .unwrap();
    assert_eq!(
        ask(&mut host, &mut session),
        resp("ring", "2507536232473510507")
    );
    assert_eq!(host.s.world.journal.entries().len(), 1);
    assert_eq!(
        choose(&mut host, &mut session, 2),
        resp("ring", "3245423813243151497")
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
    assert_eq!(host.s.pc.loadout.count(&ring_item), 0);
    assert_eq!(host.s.fargoth.loadout.count(&ring_item), 1);
    assert!((host.s.locals[&(FARGOTH, id("state"))] + 1.0).abs() < f32::EPSILON);
    assert_eq!(host.s.npcs.modifier(FARGOTH), 45);
    assert_eq!(host.s.npcs.modifier(ARRILLE), 40);
    let entries = host.s.world.journal.entries();
    assert_eq!(
        entries.iter().map(|e| e.stage).collect::<Vec<_>>(),
        [Some(10), Some(100)]
    );
    assert!(entries.iter().all(|e| !e.text.contains('%')), "{entries:?}");

    // Afterwards: "Yes, thank you for finding it!"
    assert_eq!(
        ask(&mut host, &mut session),
        resp("ring", "12942106023022017709")
    );

    // Save/load round trip.
    let bytes = postcard::to_stdvec(&host.s).unwrap();
    let back: Saved = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back, host.s);
    eprintln!("saved RPG state: {} bytes", bytes.len());
}
