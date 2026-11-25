//! DQ-3 main-quest trace (`docs/roadmaps/dialogue-quests.md` §3 decision 3): a greedy
//! walkthrough bot over the real DIAL/INFO data, through the game's filters
//! ([`MwDialogueRules`]) and result runner ([`MwResults`]), from the new-game state. It prints
//! the furthest stage per main-quest quest and why each one stalled. Measurement only (the
//! lane gauge, rerun after every slice); skips (passes) when no Morrowind install is detected.
//!
//! The bot is honest: no item, global or stage it did not get through dialogue. It may walk to
//! any placed NPC (the player then stands in the speaker's cell). Each step it tries every
//! (speaker, topic) pair whose topic holds a response that would raise a main-quest index,
//! with every `Choice` branch on a cloned state (bounded depth), and keeps the one that
//! advances furthest. With nothing advancing it asks the relevant NPCs every known topic once
//! to learn new topics (only the topics are kept), and stops when that teaches nothing.
//! Object, global and start scripts do not run yet, so a stage only a script sets stalls with
//! "needs a script". Output: counts, record ids and function names only, never game text.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;

use mw_install::MorrowindInstall;
use mw_rpg::{
    ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwResults, MwView, MwWorld, NpcStates,
    ResultCall, ResultEffect, ResultRunner, ResultState, ScriptVar, Target, VarType, rank_name,
};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, ConversationCtx, DialogueOutput, DialogueRules, DialogueSession,
    GameTime, Holder, PlayerId, RpgCommand, SessionState, TopicKind, WorldRpg,
};
use sf_vfs::Vfs;
use tes3::esp::{
    Cell, Creature, Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison,
    FilterFunction, FilterType, FilterValue, GlobalValue, GlobalVariable, Npc, Region, Script, Sex,
};

/// Steps (committed advances plus topic-learning rounds) before the bot gives up.
const MAX_STEPS: usize = 300;
/// Nested `Choice` answers tried after one ask.
const MAX_CHOICE_DEPTH: usize = 3;
/// NPCs tried as the speaker of a response that names no speaker.
const MAX_SPEAKERLESS: usize = 3;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// Main-quest progress: (stages reached, sum of quest indexes); higher is further.
type Progress = (usize, i64);

/// A trial's rank: how little it jumps ahead of the story (`World::start_rank`, lower first),
/// then its progress.
type Rank = (Reverse<u8>, Progress);

/// A trial: walk to this speaker and ask this topic (`None`: the greeting only).
type Action = (usize, Option<RecordId>);

fn key(ix: usize) -> ActorKey {
    ActorKey(ix as u128 + 1)
}

/// Main-quest quest ids: `A1_*`, `A2_*` (Blades and the Sixth House), `B<n>_*` (the
/// Nerevarine prophecies) and `C<n>_*` (the end). Faction and side quests use other prefixes.
fn is_main(quest: &str) -> bool {
    let b = quest.as_bytes();
    b.len() > 3
        && matches!(b[0], b'a' | b'b' | b'c')
        && b[1].is_ascii_digit()
        && (b[2] == b'_' || (b[2].is_ascii_digit() && b.get(3) == Some(&b'_')))
}

/// `Journal <quest> <index>` and `SetJournalIndex <quest> <index>` calls in a script's text
/// (any `x->` prefix, quotes and commas allowed; comments skipped).
fn journal_calls(text: &str) -> Vec<(RecordId, i32)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let code = line.split(';').next().unwrap_or("");
        let toks: Vec<String> = code
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|t| !t.is_empty())
            .map(|t| t.trim_matches('"').to_ascii_lowercase())
            .collect();
        for (i, t) in toks.iter().enumerate() {
            let cmd = t.rsplit("->").next().unwrap_or(t);
            if cmd != "journal" && cmd != "setjournalindex" {
                continue;
            }
            if let (Some(q), Some(ix)) = (toks.get(i + 1), toks.get(i + 2))
                && let Ok(ix) = ix.parse::<i32>()
            {
                out.push((id(q), ix));
            }
        }
    }
    out
}

struct NpcEntry {
    base: RecordId,
    facts: ActorFacts,
    disposition: i32,
    declared: HashMap<RecordId, VarType>,
}

/// A response under a conversation or greeting topic whose result raises a main-quest index.
#[derive(Clone)]
struct Setter {
    topic: RecordId,
    response: RecordId,
    greeting: bool,
    quest: RecordId,
    index: i32,
}

/// What does not change while the bot plays.
struct World<'a> {
    records: &'a RecordDb,
    dlg: &'a MwDialogue,
    /// Placed NPCs, sorted by base id; index `i` is `key(i)`.
    npcs: Vec<NpcEntry>,
    by_base: HashMap<RecordId, usize>,
    global_types: HashMap<RecordId, GlobalNumber>,
    player: ActorFacts,
    /// Main-quest quests (sorted) and their stage indexes (> 0) in the data.
    main: BTreeMap<RecordId, Vec<i32>>,
    setters: Vec<Setter>,
    /// (quest, index) → scripts (sorted ids) whose text sets it.
    script_setters: BTreeMap<(RecordId, i32), Vec<RecordId>>,
    /// NPCs with a speaker-named response that reads or writes a main quest.
    relevant: BTreeSet<usize>,
}

impl World<'_> {
    fn npc(&self, key: ActorKey) -> Option<&NpcEntry> {
        usize::try_from(key.0.checked_sub(1)?)
            .ok()
            .and_then(|i| self.npcs.get(i))
    }

    fn player_at(&self, ix: usize) -> ActorFacts {
        ActorFacts {
            cell: self.npcs[ix].facts.cell.clone(),
            ..self.player.clone()
        }
    }

    fn ctx(&self, ix: usize) -> ConversationCtx {
        ConversationCtx {
            player: PlayerId::HOST,
            speaker: key(ix),
            speaker_base: self.npcs[ix].base.clone(),
            choice: None,
        }
    }

    /// How far a step from `s` to `st` jumps ahead of the story: 0 when it only advances
    /// quests already in the journal, 1 when it starts a quest from a response gated on
    /// another main quest's journal (the story sends the player there), 2 when it starts one
    /// ungated (a sequence break, such as Dagoth Gares' "lord's friendship" before Caius sends
    /// the player to Ilunibi, whose `A2_2_6thHouse 49` then shadows Caius' `A1_1` orders).
    /// The bot finishes what it started first, as a player following the journal does.
    fn start_rank(&self, s: &State, st: &State) -> u8 {
        self.main
            .keys()
            .filter(|q| s.world.quests.stage(q) == 0 && st.world.quests.stage(q) > 0)
            .map(|q| {
                let at = st.world.quests.stage(q);
                let gated = self.setters.iter().any(|t| {
                    &t.quest == q
                        && t.index == at
                        && self.dlg.info(&t.response).is_some_and(|i| {
                            i.filters.iter().any(|f| {
                                f.filter_type == FilterType::Journal
                                    && id(&f.id) != *q
                                    && self.main.contains_key(&id(&f.id))
                            })
                        })
                });
                if gated { 1 } else { 2 }
            })
            .max()
            .unwrap_or(0)
    }

    /// (stages reached, sum of indexes) over the main quests.
    fn progress(&self, s: &State) -> Progress {
        self.main.keys().fold((0, 0), |(n, sum), q| {
            (n + reached(s, q), sum + i64::from(s.world.quests.stage(q)))
        })
    }
}

/// Stages of `quest` the bot reached: distinct journal entries (a stage skipped by a later
/// index does not count, nor an index without an entry).
fn reached(s: &State, quest: &RecordId) -> usize {
    s.world
        .journal
        .for_quest(quest)
        .filter_map(|e| e.stage)
        .collect::<BTreeSet<i32>>()
        .len()
}

/// What the bot's actions change (cloned per trial).
#[derive(Clone, Default)]
struct State {
    world: WorldRpg,
    pc: CharacterRpg,
    /// Items given to NPCs (their own inventories are not loaded).
    held: BTreeMap<ActorKey, CharacterRpg>,
    npcs: NpcStates,
    locals: BTreeMap<(ActorKey, RecordId), f32>,
    globals: BTreeMap<RecordId, f32>,
}

/// The host's read side for one conversation (the player stands at the speaker).
struct View<'v> {
    w: &'v World<'v>,
    s: &'v State,
    player: ActorFacts,
}

impl View<'_> {
    fn global_value(&self, gid: &RecordId) -> Option<GlobalNumber> {
        let ty = *self.w.global_types.get(gid)?;
        Some(match (ty, self.s.globals.get(gid)) {
            #[allow(clippy::cast_possible_truncation)]
            (GlobalNumber::Int(_), Some(v)) => GlobalNumber::Int(*v as i32),
            (GlobalNumber::Float(_), Some(v)) => GlobalNumber::Float(*v),
            (initial, None) => initial,
        })
    }

    fn local_value(&self, actor: ActorKey, name: &RecordId) -> Option<f32> {
        let npc = self.w.npc(actor)?;
        npc.declared.contains_key(name).then(|| {
            self.s
                .locals
                .get(&(actor, name.clone()))
                .copied()
                .unwrap_or(0.0)
        })
    }

    fn target_key(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        match target {
            Target::Speaker => Some(ctx.speaker),
            Target::Actor(a) => self.w.by_base.get(a).map(|&i| key(i)),
            Target::Player | Target::Ref(_) | Target::Stack { .. } => None,
        }
    }
}

impl MwView for View<'_> {
    fn speaker(&self, ctx: &ConversationCtx) -> Option<&ActorFacts> {
        self.w.npc(ctx.speaker).map(|n| &n.facts)
    }

    fn player(&self, _player: PlayerId) -> Option<&ActorFacts> {
        Some(&self.player)
    }

    /// A new character belongs to no faction (joining is a stats effect: no sheets here).
    fn player_rank(&self, _player: PlayerId, _faction: &RecordId) -> Option<i32> {
        None
    }

    fn global(&self, gid: &RecordId) -> Option<GlobalNumber> {
        self.global_value(gid)
    }

    fn local(&self, speaker: ActorKey, name: &RecordId) -> Option<f32> {
        self.local_value(speaker, name)
    }

    fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.s.world.quests.stage(quest)
    }

    fn item_count(&self, _player: PlayerId, item: &RecordId) -> u64 {
        self.s.pc.loadout.count(item)
    }

    /// The bot never kills.
    fn dead_count(&self, _base: &RecordId) -> i32 {
        0
    }

    fn disposition(&self, ctx: &ConversationCtx) -> i32 {
        let base = self.w.npc(ctx.speaker).map_or(0, |n| n.disposition);
        self.s.npcs.disposition(ctx.speaker, base)
    }

    fn rank_name(&self, faction: &RecordId, rank: i32) -> Option<String> {
        rank_name(self.w.records, faction, rank)
    }

    /// A fresh level-1 character (as `fargoths_ring`); everything else is the neutral 0.
    fn function(&self, _ctx: &ConversationCtx, function: FilterFunction) -> Option<f32> {
        match function {
            FilterFunction::PcClothingModifier => Some(10.0),
            FilterFunction::PcHealthPercent => Some(100.0),
            FilterFunction::PcLevel => Some(1.0),
            _ => None,
        }
    }
}

impl MwWorld for View<'_> {
    fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.global_value(name)
    }

    fn var(&self, ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        let k = self.target_key(ctx, target)?;
        Some(ScriptVar {
            ty: *self.w.npc(k)?.declared.get(name)?,
            value: self.local_value(k, name)?,
        })
    }

    fn item_count(&self, ctx: &ConversationCtx, target: &Target, item: &RecordId) -> u64 {
        self.target_key(ctx, target)
            .and_then(|k| self.s.held.get(&k))
            .map_or(0, |c| c.loadout.count(item))
    }

    fn actor(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        self.target_key(ctx, target)
    }

    fn facts(&self, ctx: &ConversationCtx, target: &Target) -> Option<&ActorFacts> {
        let k = self.target_key(ctx, target)?;
        self.w.npc(k).map(|n| &n.facts)
    }
}

/// Counters for the log line.
#[derive(Default)]
struct Stats {
    trials: usize,
    result_errors: usize,
    stat_effects: usize,
    reports: BTreeSet<String>,
}

struct Bot<'w> {
    w: &'w World<'w>,
    results: MwResults,
    stats: Stats,
}

impl<'w> Bot<'w> {
    fn view<'v>(&self, s: &'v State, ix: usize) -> View<'v>
    where
        'w: 'v,
    {
        View {
            w: self.w,
            s,
            player: self.w.player_at(ix),
        }
    }

    /// The host step after a session call (fargoth.rs `after`): learn linked topics the
    /// speaker answers, run results in the game's runner, apply their effects in call order.
    fn after(
        &mut self,
        s: &mut State,
        ix: usize,
        session: &mut DialogueSession,
        out: &[DialogueOutput],
    ) {
        let w = self.w;
        let ctx = session.ctx().clone();
        let mut learned = Vec::new();
        let mut effects = Vec::new();
        {
            let view = View {
                w,
                s: &*s,
                player: w.player_at(ix),
            };
            let rules = MwDialogueRules::new(w.dlg, &view);
            let state = ResultState {
                world: &s.world,
                player: &s.pc,
                env: &view,
            };
            let mut runner = self.results.runner(state);
            for o in out {
                match o {
                    DialogueOutput::Learned(t) if rules.speaker_answers(&ctx, t) => {
                        learned.push(t.clone());
                    }
                    DialogueOutput::RunResult { topic, response } => {
                        let script = &w.dlg.info(response).expect("shown INFO").script_text;
                        let call = ResultCall {
                            ctx: &ctx,
                            topic,
                            response,
                            script,
                        };
                        match runner.run(call) {
                            Ok(e) => effects.extend(e),
                            Err(_) => self.stats.result_errors += 1,
                        }
                    }
                    _ => {}
                }
            }
        }
        for r in self.results.drain_reports() {
            self.stats.reports.insert(r.to_string());
        }
        for t in learned {
            let _ = s.world.apply(RpgCommand::AddTopic(t));
        }
        for effect in effects {
            if let ResultEffect::Journal { quest, index } = &effect {
                let cmd = {
                    let view = self.view(s, ix);
                    let rules = MwDialogueRules::new(w.dlg, &view);
                    w.dlg
                        .journal_command(&s.world, quest, *index, GameTime::default(), |t| {
                            rules.expand(&ctx, t)
                        })
                };
                if let Some(cmd) = cmd {
                    let _ = s.world.apply(cmd);
                }
            } else {
                self.apply(s, session, &ctx, effect);
            }
        }
    }

    fn apply(
        &mut self,
        s: &mut State,
        session: &mut DialogueSession,
        ctx: &ConversationCtx,
        effect: ResultEffect,
    ) {
        let target_key = |t: &Target| match t {
            Target::Speaker => Some(ctx.speaker),
            Target::Actor(a) => self.w.by_base.get(a).map(|&i| key(i)),
            Target::Player | Target::Ref(_) | Target::Stack { .. } => None,
        };
        match effect {
            ResultEffect::AddTopic(t) => {
                let _ = s.world.apply(RpgCommand::AddTopic(t));
            }
            ResultEffect::AddItem {
                target,
                item,
                count,
            } => {
                let cmd = |holder| RpgCommand::AddItem {
                    holder,
                    item,
                    count,
                    attrs: Attrs::new(),
                };
                if target == Target::Player {
                    let _ = s.pc.apply(cmd(Holder::Player(PlayerId::HOST)));
                } else if let Some(k) = target_key(&target) {
                    let _ = s.held.entry(k).or_default().apply(cmd(Holder::Actor(k)));
                }
            }
            ResultEffect::RemoveItem {
                target,
                item,
                count,
            } => {
                let cmd = |holder| RpgCommand::RemoveItem {
                    holder,
                    item,
                    count,
                };
                if target == Target::Player {
                    let _ = s.pc.apply(cmd(Holder::Player(PlayerId::HOST)));
                } else if let Some(k) = target_key(&target) {
                    let _ = s.held.entry(k).or_default().apply(cmd(Holder::Actor(k)));
                }
            }
            ResultEffect::SetJournalIndex { quest, index } => {
                let _ = s.world.apply(MwDialogue::set_journal_index(&quest, index));
            }
            ResultEffect::Choices(choices) => {
                let _ = session.offer_choices(choices);
            }
            ResultEffect::Goodbye => session.force_goodbye(),
            ResultEffect::ModDisposition { target, delta } => {
                if let Some(k) = target_key(&target) {
                    s.npcs.modify(k, delta);
                }
            }
            ResultEffect::SetDisposition { target, value } => {
                if let Some(k) = target_key(&target) {
                    let base = self.w.npc(k).map_or(0, |n| n.disposition);
                    s.npcs.set(k, base, value);
                }
            }
            ResultEffect::SetLocal {
                target,
                name,
                value,
            } => {
                if let Some(k) = target_key(&target) {
                    s.locals.insert((k, name), value);
                }
            }
            ResultEffect::SetGlobal { name, value } => {
                s.globals.insert(name, value);
            }
            ResultEffect::Stats { .. } => self.stats.stat_effects += 1,
            ResultEffect::Journal { .. }
            | ResultEffect::MessageBox { .. }
            | ResultEffect::PlaySound(_)
            | ResultEffect::Ai { .. } => {}
        }
    }

    /// Walks to NPC `ix` and opens a conversation (its greeting runs).
    fn open(&mut self, s: &mut State, ix: usize) -> DialogueSession {
        let (mut session, out) = {
            let view = self.view(s, ix);
            let rules = MwDialogueRules::new(self.w.dlg, &view);
            DialogueSession::open(&self.w.dlg.db, &rules, self.w.ctx(ix))
        };
        self.after(s, ix, &mut session, &out);
        session
    }

    fn ask(&mut self, s: &mut State, ix: usize, session: &mut DialogueSession, topic: &RecordId) {
        let out = {
            let view = self.view(s, ix);
            let rules = MwDialogueRules::new(self.w.dlg, &view);
            session.ask(&self.w.dlg.db, &rules, topic)
        };
        if let Ok(out) = out {
            self.after(s, ix, session, &out);
        }
    }

    fn choose(&mut self, s: &mut State, ix: usize, session: &mut DialogueSession, value: i32) {
        let out = {
            let view = self.view(s, ix);
            let rules = MwDialogueRules::new(self.w.dlg, &view);
            session.choose(&self.w.dlg.db, &rules, value)
        };
        if let Ok(out) = out {
            self.after(s, ix, session, &out);
        }
    }

    /// The best state reachable by answering pending choices (depth-bounded); ties keep the
    /// earlier (fewer answers, lower values).
    fn best_choices(
        &mut self,
        s: State,
        ix: usize,
        session: &DialogueSession,
        depth: usize,
    ) -> (State, Vec<i32>) {
        let SessionState::Choosing(choices) = session.state() else {
            return (s, Vec::new());
        };
        if depth == 0 {
            return (s, Vec::new());
        }
        let mut values: Vec<i32> = choices.iter().map(|c| c.value).collect();
        values.sort_unstable();
        let mut best_p = self.w.progress(&s);
        let mut best: Option<(State, Vec<i32>)> = None;
        for v in values {
            let mut st = s.clone();
            let mut sess = session.clone();
            self.choose(&mut st, ix, &mut sess, v);
            let (st, mut path) = self.best_choices(st, ix, &sess, depth - 1);
            let p = self.w.progress(&st);
            if p > best_p {
                best_p = p;
                path.insert(0, v);
                best = Some((st, path));
            }
        }
        best.unwrap_or((s, Vec::new()))
    }

    /// One trial: walk to `ix`, greet, ask `topic` (greeting setters: none), answer choices.
    fn trial(&mut self, s: &State, ix: usize, topic: Option<&RecordId>) -> (State, Vec<i32>) {
        self.stats.trials += 1;
        let mut st = s.clone();
        let mut session = self.open(&mut st, ix);
        if let Some(t) = topic
            && *session.state() == SessionState::Talking
        {
            self.ask(&mut st, ix, &mut session, t);
        }
        self.best_choices(st, ix, &session, MAX_CHOICE_DEPTH)
    }

    /// NPCs the rules accept as the speaker of `response` now (first few by base id).
    fn speakers(&self, s: &State, setter: &Setter) -> Vec<usize> {
        let info = self.w.dlg.info(&setter.response).expect("setter INFO");
        if !info.speaker_id.is_empty() {
            return self
                .w
                .by_base
                .get(&id(&info.speaker_id))
                .copied()
                .into_iter()
                .collect();
        }
        (0..self.w.npcs.len())
            .filter(|&ix| {
                let view = self.view(s, ix);
                let rules = MwDialogueRules::new(self.w.dlg, &view);
                rules.accepts(&self.w.ctx(ix), &setter.topic, &setter.response)
            })
            .take(MAX_SPEAKERLESS)
            .collect()
    }

    /// (speaker, topic) pairs worth a trial: a setter that would raise its quest, its topic
    /// known (greetings: always heard).
    fn actions(&self, s: &State) -> BTreeSet<Action> {
        let mut out = BTreeSet::new();
        for setter in &self.w.setters {
            if setter.index <= s.world.quests.stage(&setter.quest) {
                continue;
            }
            if !setter.greeting && !s.world.topics.knows(&setter.topic) {
                continue;
            }
            let topic = (!setter.greeting).then(|| setter.topic.clone());
            for ix in self.speakers(s, setter) {
                out.insert((ix, topic.clone()));
            }
        }
        out
    }

    /// Asks every relevant NPC every known conversation topic once; keeps only the topics
    /// learned. Returns how many were new.
    fn learn_topics(&mut self, s: &mut State, extra: &BTreeSet<usize>) -> usize {
        let before = s.world.topics.len();
        let known: Vec<RecordId> = {
            let mut v: Vec<RecordId> = self
                .w
                .dlg
                .db
                .topics()
                .filter(|(t, e)| e.kind == TopicKind::Conversation && s.world.topics.knows(t))
                .map(|(t, _)| t.clone())
                .collect();
            v.sort();
            v
        };
        let npcs: BTreeSet<usize> = self.w.relevant.union(extra).copied().collect();
        let mut learned = BTreeSet::new();
        for ix in npcs {
            let mut st = s.clone();
            let mut session = self.open(&mut st, ix);
            for t in &known {
                if *session.state() != SessionState::Talking {
                    session = self.open(&mut st, ix);
                    if *session.state() != SessionState::Talking {
                        break;
                    }
                }
                self.ask(&mut st, ix, &mut session, t);
            }
            for t in st.world.topics.in_learned_order() {
                if !s.world.topics.knows(t) {
                    learned.insert(t.clone());
                }
            }
        }
        for t in learned {
            let _ = s.world.apply(RpgCommand::AddTopic(t));
        }
        s.world.topics.len() - before
    }
}

// --- stall diagnosis -----------------------------------------------------------------------

fn literal(v: FilterValue) -> f64 {
    match v {
        FilterValue::Integer(i) => f64::from(i),
        FilterValue::Float(f) => f64::from(f),
    }
}

fn compare(value: f64, op: FilterComparison, lit: f64) -> bool {
    match op {
        FilterComparison::Equal => (value - lit).abs() < f64::EPSILON,
        FilterComparison::NotEqual => (value - lit).abs() >= f64::EPSILON,
        FilterComparison::Greater => value > lit,
        FilterComparison::GreaterEqual => value >= lit,
        FilterComparison::Less => value < lit,
        FilterComparison::LessEqual => value <= lit,
    }
}

fn op(c: FilterComparison) -> &'static str {
    match c {
        FilterComparison::Equal => "==",
        FilterComparison::NotEqual => "!=",
        FilterComparison::Greater => ">",
        FilterComparison::GreaterEqual => ">=",
        FilterComparison::Less => "<",
        FilterComparison::LessEqual => "<=",
    }
}

fn starts_with_ci(text: &str, prefix: &str) -> bool {
    text.get(..prefix.len())
        .is_some_and(|p| p.eq_ignore_ascii_case(prefix))
}

/// The speaker-field checks (`OpenMW` `Filter::testActor`), for picking diagnosis speakers.
fn actor_matches(info: &DialogueInfo, f: &ActorFacts) -> bool {
    let eq = |field: &str, v: Option<&RecordId>| {
        field.is_empty() || v.is_some_and(|v| v.as_str().eq_ignore_ascii_case(field))
    };
    eq(&info.speaker_race, f.race.as_ref())
        && eq(&info.speaker_class, f.class.as_ref())
        && eq(&info.speaker_faction, f.faction.as_ref())
        && (info.data.speaker_rank == -1
            || (f.faction.is_some() && f.rank >= i32::from(info.data.speaker_rank)))
        && match info.data.speaker_sex {
            Sex::Any => true,
            Sex::Male => !f.female,
            Sex::Female => f.female,
        }
}

/// Every condition `setter` fails at speaker `ix` (filter order), as short labels.
fn failures(view: &View<'_>, ctx: &ConversationCtx, setter: &Setter) -> Vec<String> {
    let info = view.w.dlg.info(&setter.response).expect("setter INFO");
    let speaker = &view.w.npc(ctx.speaker).expect("placed speaker").facts;
    let mut out = Vec::new();
    if !actor_matches(info, speaker) {
        out.push("speaker fields".to_owned());
    }
    if !info.player_faction.is_empty() {
        out.push(format!(
            "PlayerFaction {} (not a member)",
            info.player_faction.to_ascii_lowercase()
        ));
    } else if info.data.player_rank != -1 {
        out.push("PlayerRank in speaker faction (not a member)".to_owned());
    }
    if !info.speaker_cell.is_empty() && !starts_with_ci(&view.player.cell, &info.speaker_cell) {
        out.push(format!("Cell {}", info.speaker_cell.to_ascii_lowercase()));
    }
    for f in &info.filters {
        if let Some(fail) = filter_failure(view, ctx, f, speaker) {
            out.push(fail);
        }
    }
    let disposition = view.disposition(ctx);
    if speaker.npc && disposition < info.data.disposition {
        out.push(format!(
            "Disposition >= {} (actual {disposition})",
            info.data.disposition
        ));
    }
    if !setter.greeting && !view.s.world.topics.knows(&setter.topic) {
        out.push(format!("topic unknown: {}", setter.topic));
    }
    out
}

/// `filter`'s failure label, `None` when it passes (as `MwDialogueRules`' select test).
fn filter_failure(
    view: &View<'_>,
    ctx: &ConversationCtx,
    f: &Filter,
    speaker: &ActorFacts,
) -> Option<String> {
    let fid = id(&f.id);
    let is = |v: Option<&RecordId>| v == Some(&fid);
    let not = |pass: bool, kind: &str| (!pass).then(|| format!("{kind} {fid}"));
    let (label, value) = match f.filter_type {
        FilterType::None => return None,
        FilterType::NotId => return not(!is(Some(&speaker.id)), "NotId"),
        FilterType::NotFaction => return not(!is(speaker.faction.as_ref()), "NotFaction"),
        FilterType::NotClass => return not(!is(speaker.class.as_ref()), "NotClass"),
        FilterType::NotRace => return not(!is(speaker.race.as_ref()), "NotRace"),
        FilterType::NotCell => return not(!starts_with_ci(&speaker.cell, &f.id), "NotCell"),
        FilterType::NotLocal => {
            return not(view.local(ctx.speaker, &fid).is_none(), "NotLocal");
        }
        FilterType::Local => match view.local(ctx.speaker, &fid) {
            Some(v) => (format!("Local {fid}"), f64::from(v)),
            None => return Some(format!("Local {fid} (undeclared)")),
        },
        FilterType::Global => (
            format!("Global {fid}"),
            f64::from(view.global_value(&fid)?.as_f32()),
        ),
        FilterType::Journal => (format!("Journal {fid}"), f64::from(view.quest_stage(&fid))),
        #[allow(clippy::cast_precision_loss)]
        FilterType::Item => (
            format!("Item {fid}"),
            MwView::item_count(view, ctx.player, &fid) as f64,
        ),
        FilterType::Dead => (format!("Dead {fid}"), f64::from(view.dead_count(&fid))),
        FilterType::Function if f.function == FilterFunction::Choice => {
            // Reached by answering; the trial tries every answer.
            return None;
        }
        FilterType::Function => (
            format!("Function {:?}", f.function),
            view.function(ctx, f.function).map_or(0.0, f64::from),
        ),
    };
    let lit = literal(f.value);
    (!compare(value, f.comparison, lit))
        .then(|| format!("{label} {} {lit} (actual {value})", op(f.comparison)))
}

/// Why a setter whose conditions all pass at `ctx`'s speaker was not reached: another
/// response wins its topic (greetings: across the greeting topics), or it needs an answer.
fn unreached(view: &View<'_>, ctx: &ConversationCtx, setter: &Setter) -> String {
    let rules = MwDialogueRules::new(view.w.dlg, view);
    let db = &view.w.dlg.db;
    let speaker = &view.w.npc(ctx.speaker).expect("placed speaker").base;
    let selected = if setter.greeting {
        db.select_greeting(&rules, ctx).map(|(_, r)| r)
    } else {
        db.select(&rules, ctx, &setter.topic)
    };
    if selected.as_ref() == Some(&setter.response) {
        return format!("selected at {speaker} but its result does not raise the stage");
    }
    let info = view.w.dlg.info(&setter.response).expect("setter INFO");
    if let Some(choice) = info
        .filters
        .iter()
        .find(|f| f.filter_type == FilterType::Function && f.function == FilterFunction::Choice)
    {
        let n = literal(choice.value);
        let offers = |r: &RecordId| {
            view.w.dlg.info(r).is_some_and(|i| {
                (i.speaker_id.is_empty() || i.speaker_id.eq_ignore_ascii_case(speaker.as_str()))
                    && i.script_text.to_ascii_lowercase().contains("choice")
            })
        };
        if selected.as_ref().is_some_and(&offers) {
            return format!("Choice {n} at {speaker}: answers deeper than the bot tried");
        }
        // The responses here that offer choices (their result calls `Choice`): the one that
        // passes is shadowed; else the fewest failures (a later stage of the quest last).
        let own = format!("Journal {} ", setter.quest);
        let offer = db
            .topic(&setter.topic)
            .into_iter()
            .flat_map(|e| &e.responses)
            .filter(|r| *r != &setter.response && offers(r))
            .map(|r| {
                let probe = Setter {
                    response: r.clone(),
                    ..setter.clone()
                };
                (r, failures(view, ctx, &probe))
            })
            .min_by_key(|(_, f)| (f.first().is_some_and(|x| x.contains(&own)), f.len()));
        let topic = &setter.topic;
        return match offer {
            Some((_, f)) if !f.is_empty() => {
                format!("Choice {n}: the offer in {topic} fails {}", f[0])
            }
            Some((r, _)) => format!(
                "Choice {n}: the offer in {topic} is {}",
                shadowed(view, selected.as_ref(), r)
            ),
            None => format!("Choice {n} at {speaker}: no offer in {topic}"),
        };
    }
    format!(
        "{} at {speaker}",
        shadowed(view, selected.as_ref(), &setter.response)
    )
}

/// "shadowed by INFO <id> (holds: <its first filter `wanted` lacks>)": the earlier response
/// the topic selects instead of `wanted`.
fn shadowed(view: &View<'_>, selected: Option<&RecordId>, wanted: &RecordId) -> String {
    let Some(sel) = selected else {
        return "not selected (nothing is)".to_owned();
    };
    let by = mw_records::keys::split_info(sel).map_or("", |(_, info)| info);
    let (Some(shadow), Some(want)) = (view.w.dlg.info(sel), view.w.dlg.info(wanted)) else {
        return format!("shadowed by INFO {by}");
    };
    let same = |a: &Filter, b: &Filter| {
        a.filter_type == b.filter_type
            && a.id.eq_ignore_ascii_case(&b.id)
            && a.function == b.function
            && a.comparison == b.comparison
            && a.value == b.value
    };
    let extra = shadow
        .filters
        .iter()
        .find(|f| f.filter_type != FilterType::None && !want.filters.iter().any(|w| same(f, w)));
    match extra {
        Some(f) => {
            let what = if f.filter_type == FilterType::Function {
                format!("{:?}", f.function)
            } else {
                f.id.to_ascii_lowercase()
            };
            format!(
                "shadowed by INFO {by} (holds: {:?} {what} {} {})",
                f.filter_type,
                op(f.comparison),
                literal(f.value)
            )
        }
        None => format!("shadowed by INFO {by}"),
    }
}

/// Why `quest` stops at its current stage.
fn stall_reason(w: &World<'_>, s: &State, quest: &RecordId) -> String {
    let at = s.world.quests.stage(quest);
    // Dialogue setters raising the quest: not waiting on a later stage of the same quest,
    // then the next index, then the fewest failures.
    // (blocked by a later stage of the same quest, index, failure count, first failure).
    let mut best: Option<(bool, i32, usize, String)> = None;
    let own = format!("Journal {quest} ");
    // `DQ3_QUEST=<quest id>` lists every setter's failures for that quest (ids only).
    let verbose = std::env::var("DQ3_QUEST").is_ok_and(|q| id(&q) == *quest);
    for setter in w
        .setters
        .iter()
        .filter(|t| &t.quest == quest && t.index > at)
    {
        let info = w.dlg.info(&setter.response).expect("setter INFO");
        let speakers: Vec<usize> = if info.speaker_id.is_empty() {
            (0..w.npcs.len())
                .filter(|&ix| actor_matches(info, &w.npcs[ix].facts))
                .take(MAX_SPEAKERLESS)
                .collect()
        } else {
            w.by_base
                .get(&id(&info.speaker_id))
                .copied()
                .into_iter()
                .collect()
        };
        if speakers.is_empty() {
            let why = if info.speaker_id.is_empty() {
                "no placed NPC matches the speaker fields".to_owned()
            } else {
                format!(
                    "speaker {} not placed in any cell",
                    info.speaker_id.to_ascii_lowercase()
                )
            };
            let cand = (false, setter.index, usize::MAX, why);
            if best
                .as_ref()
                .is_none_or(|b| (cand.0, cand.1, cand.2) < (b.0, b.1, b.2))
            {
                best = Some(cand);
            }
            continue;
        }
        for ix in speakers {
            let view = View {
                w,
                s,
                player: w.player_at(ix),
            };
            let fails = failures(&view, &w.ctx(ix), setter);
            let why = fails
                .first()
                .cloned()
                .unwrap_or_else(|| unreached(&view, &w.ctx(ix), setter));
            if verbose {
                eprintln!(
                    "  {} -> {} at {} / {}: {fails:?} | {why}",
                    setter.quest, setter.index, w.npcs[ix].base, setter.response
                );
            }
            let cand = (why.contains(&own), setter.index, fails.len(), why);
            if best
                .as_ref()
                .is_none_or(|b| (cand.0, cand.1, cand.2) < (b.0, b.1, b.2))
            {
                best = Some(cand);
            }
        }
    }
    // Scripts that set the next index the dialogue does not.
    let script = w
        .script_setters
        .range((quest.clone(), at + 1)..(quest.clone(), i32::MAX))
        .next();
    match (best, script) {
        (Some((_, ix, _, why)), Some(((_, six), ids))) if *six < ix => {
            format!(
                "needs a script: {} (index {six}; dialogue first at {ix}: {why})",
                ids[0]
            )
        }
        (Some((_, ix, _, why)), _) => format!("index {ix}: {why}"),
        (None, Some(((_, six), ids))) => format!(
            "needs a script: {} (index {six}{})",
            ids[0],
            if ids.len() > 1 {
                format!(", +{} more", ids.len() - 1)
            } else {
                String::new()
            }
        ),
        (None, None) => "no setter in dialogue or scripts".to_owned(),
    }
}

/// The reason's kind, for the histogram (`Journal a1_...`, `Function PcLevel`, ...).
fn reason_kind(reason: &str) -> String {
    let r = reason
        .split_once(": ")
        .filter(|(head, _)| head.starts_with("index "))
        .map_or(reason, |(_, tail)| tail);
    if r.starts_with("needs a script") {
        return "needs a script".to_owned();
    }
    let mut words = r.split_whitespace();
    let first = words.next().unwrap_or("");
    match first {
        "Function" | "Global" | "Local" | "Item" | "Dead" | "Journal" | "PlayerFaction" => {
            format!("{first} {}", words.next().unwrap_or(""))
        }
        _ => r.split(['(', ':']).next().unwrap_or(r).trim().to_owned(),
    }
}

// --- setup ---------------------------------------------------------------------------------

/// The declared locals of a script, from its source text.
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

/// Each placed NPC's (and talking creature's) cell name (exteriors: the cell's or its
/// region's name); the first by name when placed more than once.
fn placements(records: &RecordDb, talkers: &BTreeSet<RecordId>) -> BTreeMap<RecordId, String> {
    let mut out: BTreeMap<RecordId, String> = BTreeMap::new();
    for (_, cell) in records.iter::<Cell>() {
        let name = if cell.is_interior() || !cell.name.is_empty() {
            cell.name.clone()
        } else {
            cell.region
                .as_deref()
                .and_then(|r| records.get::<Region>(&id(r)))
                .map(|r| r.name.clone())
                .unwrap_or_default()
        };
        for r in cell.references.values() {
            let base = id(&r.id);
            if records.get::<Npc>(&base).is_none() && !talkers.contains(&base) {
                continue;
            }
            out.entry(base)
                .and_modify(|c| {
                    if name < *c {
                        c.clone_from(&name);
                    }
                })
                .or_insert_with(|| name.clone());
        }
    }
    out
}

#[allow(clippy::too_many_lines)]
fn build_world<'a>(records: &'a RecordDb, dlg: &'a MwDialogue) -> World<'a> {
    let mut npcs = Vec::new();
    let declared = |script: &str| {
        records
            .get::<Script>(&id(script))
            .map(|s| declared_locals(&s.text))
            .unwrap_or_default()
    };
    // Creatures a response names by id (ash vampires, Dagoth Ur, ...).
    let talkers: BTreeSet<RecordId> = records
        .iter::<DialogueInfo>()
        .filter(|(_, i)| !i.speaker_id.is_empty())
        .map(|(_, i)| id(&i.speaker_id))
        .filter(|s| records.get::<Creature>(s).is_some())
        .collect();
    for (base, cell) in placements(records, &talkers) {
        if let Some(npc) = records.get::<Npc>(&base) {
            npcs.push(NpcEntry {
                facts: ActorFacts::from_npc(records, npc, &cell),
                disposition: i32::from(npc.data.disposition),
                declared: declared(&npc.script),
                base,
            });
        } else if let Some(c) = records.get::<Creature>(&base) {
            npcs.push(NpcEntry {
                facts: ActorFacts {
                    id: base.clone(),
                    name: c.name.clone(),
                    cell,
                    ..ActorFacts::default()
                },
                disposition: 0,
                declared: declared(&c.script),
                base,
            });
        }
    }
    let by_base = npcs
        .iter()
        .enumerate()
        .map(|(i, n)| (n.base.clone(), i))
        .collect();
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
    let main: BTreeMap<RecordId, Vec<i32>> = records
        .iter::<Dialogue>()
        .filter(|(q, d)| d.dialogue_type == DialogueType2::Journal && is_main(q.as_str()))
        .map(|(q, _)| {
            let stages = dlg
                .journal
                .quest(q)
                .map(|mq| mq.stages.keys().copied().filter(|&i| i > 0).collect())
                .unwrap_or_default();
            (q.clone(), stages)
        })
        .collect();
    let greetings: BTreeSet<&RecordId> = dlg.db.greetings().iter().collect();
    let mut setters = Vec::new();
    let mut world = World {
        records,
        dlg,
        npcs,
        by_base,
        global_types,
        player: ActorFacts {
            id: id("player"),
            name: "Nerevar".into(),
            npc: true,
            rank: -1,
            ..ActorFacts::default()
        },
        main,
        setters: Vec::new(),
        script_setters: BTreeMap::new(),
        relevant: BTreeSet::new(),
    };
    let mut relevant = BTreeSet::new();
    let mut topics: Vec<(&RecordId, _)> = dlg
        .db
        .topics()
        .filter(|(t, e)| e.kind == TopicKind::Conversation || greetings.contains(t))
        .collect();
    topics.sort_by(|a, b| a.0.cmp(b.0));
    for (topic, entry) in topics {
        for response in &entry.responses {
            let Some(info) = dlg.info(response) else {
                continue;
            };
            let calls: Vec<(RecordId, i32)> = journal_calls(&info.script_text)
                .into_iter()
                .filter(|(q, _)| world.main.contains_key(q))
                .collect();
            let reads = info.filters.iter().any(|f| {
                f.filter_type == FilterType::Journal && world.main.contains_key(&id(&f.id))
            });
            if (reads || !calls.is_empty())
                && let Some(&ix) = world.by_base.get(&id(&info.speaker_id))
            {
                relevant.insert(ix);
            }
            for (quest, index) in calls {
                setters.push(Setter {
                    topic: topic.clone(),
                    response: response.clone(),
                    greeting: greetings.contains(topic),
                    quest,
                    index,
                });
            }
        }
    }
    let mut script_setters: BTreeMap<(RecordId, i32), Vec<RecordId>> = BTreeMap::new();
    for (sid, script) in records.iter::<Script>() {
        for (quest, index) in journal_calls(&script.text) {
            if world.main.contains_key(&quest) {
                script_setters
                    .entry((quest, index))
                    .or_default()
                    .push(sid.clone());
            }
        }
    }
    for ids in script_setters.values_mut() {
        ids.sort();
        ids.dedup();
    }
    world.setters = setters;
    world.script_setters = script_setters;
    world.relevant = relevant;
    world
}

/// The game's new game without chargen (`mw_rpg_bevy` `Rpg::skip_chargen`, DQ-3b): `chargenstate`
/// -1, and the chargen captain's "duties" result applied as in the game: its topics, journal and
/// the player's items (package, directions, gold). His locals don't matter to the bot.
fn new_game(bot: &mut Bot<'_>) -> State {
    let w = bot.w;
    let mut s = State::default();
    s.globals.insert(id("chargenstate"), -1.0);
    let Some(&ix) = w.by_base.get(&id("chargen captain")) else {
        return s;
    };
    let topic = id("duties");
    let Some(entry) = w.dlg.db.topic(&topic) else {
        return s;
    };
    let ctx = w.ctx(ix);
    let mut topics = Vec::new();
    let mut kept = Vec::new();
    {
        let view = bot.view(&s, ix);
        let state = ResultState {
            world: &s.world,
            player: &s.pc,
            env: &view,
        };
        let mut runner = bot.results.runner(state);
        for response in &entry.responses {
            let Some(info) = w.dlg.info(response) else {
                continue;
            };
            if !info.speaker_id.eq_ignore_ascii_case("chargen captain") {
                continue;
            }
            let call = ResultCall {
                ctx: &ctx,
                topic: &topic,
                response,
                script: &info.script_text,
            };
            for e in runner.run(call).unwrap_or_default() {
                match e {
                    ResultEffect::AddTopic(t) => topics.push(t),
                    ResultEffect::Journal { .. }
                    | ResultEffect::SetJournalIndex { .. }
                    | ResultEffect::AddItem {
                        target: Target::Player,
                        ..
                    }
                    | ResultEffect::RemoveItem {
                        target: Target::Player,
                        ..
                    } => kept.push(e),
                    _ => {}
                }
            }
        }
    }
    for t in topics {
        let _ = s.world.apply(RpgCommand::AddTopic(t));
    }
    for e in kept {
        match e {
            ResultEffect::Journal { quest, index } => {
                let cmd = {
                    let view = bot.view(&s, ix);
                    let rules = MwDialogueRules::new(w.dlg, &view);
                    w.dlg
                        .journal_command(&s.world, &quest, index, GameTime::default(), |t| {
                            rules.expand(&ctx, t)
                        })
                };
                if let Some(cmd) = cmd {
                    let _ = s.world.apply(cmd);
                }
            }
            ResultEffect::SetJournalIndex { quest, index } => {
                let _ = s.world.apply(MwDialogue::set_journal_index(&quest, index));
            }
            ResultEffect::AddItem { item, count, .. } => {
                let _ = s.pc.apply(RpgCommand::AddItem {
                    holder: Holder::Player(PlayerId::HOST),
                    item,
                    count,
                    attrs: Attrs::new(),
                });
            }
            ResultEffect::RemoveItem { item, count, .. } => {
                let _ = s.pc.apply(RpgCommand::RemoveItem {
                    holder: Holder::Player(PlayerId::HOST),
                    item,
                    count,
                });
            }
            _ => {}
        }
    }
    s
}

#[test]
#[ignore = "measurement: DQ-3 main-quest trace"]
#[allow(clippy::too_many_lines)]
fn main_quest_trace() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let started = Instant::now();
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let records = &content.db;
    let dlg = MwDialogue::from_records(records).unwrap();
    let w = build_world(records, &dlg);
    eprintln!(
        "world: {} placed NPCs and talking creatures ({} relevant), {} main quests, {} dialogue setters, {} script setters; bounds: {MAX_STEPS} steps, choice depth {MAX_CHOICE_DEPTH}, {MAX_SPEAKERLESS} speakers per speakerless response",
        w.npcs.len(),
        w.relevant.len(),
        w.main.len(),
        w.setters.len(),
        w.script_setters.len()
    );
    let mut bot = Bot {
        w: &w,
        results: MwResults::new(),
        stats: Stats::default(),
    };
    let mut s = new_game(&mut bot);
    eprintln!("new game: {} topics known", s.world.topics.len());

    let mut steps = 0;
    for step in 0..MAX_STEPS {
        steps = step + 1;
        let base = w.progress(&s);
        let actions = bot.actions(&s);
        // (rank, state, (speaker, topic), choices)
        let mut best: Option<(Rank, State, &Action, Vec<i32>)> = None;
        for action in &actions {
            let (st, path) = bot.trial(&s, action.0, action.1.as_ref());
            let p = w.progress(&st);
            let k = (Reverse(w.start_rank(&s, &st)), p);
            if p > base && best.as_ref().is_none_or(|b| k > b.0) {
                best = Some((k, st, action, path));
            }
        }
        if let Some(((_, p), st, (ix, topic), path)) = best {
            let raised: Vec<String> = w
                .main
                .keys()
                .filter(|q| st.world.quests.stage(q) > s.world.quests.stage(q))
                .map(|q| format!("{q} {}", st.world.quests.stage(q)))
                .collect();
            eprintln!(
                "step {step}: {} / {} choices {path:?}: {} ({} stages)",
                w.npcs[*ix].base,
                topic.as_ref().map_or("(greeting)", RecordId::as_str),
                raised.join(", "),
                p.0
            );
            s = st;
            continue;
        }
        let extra: BTreeSet<usize> = actions.iter().map(|(ix, _)| *ix).collect();
        let learned = bot.learn_topics(&mut s, &extra);
        eprintln!(
            "step {step}: nothing advances ({} actions); asked around: {learned} new topics ({} known)",
            actions.len(),
            s.world.topics.len()
        );
        if learned == 0 {
            break;
        }
    }

    // Report.
    let mut reached_total = 0;
    let mut stages_total = 0;
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    for (quest, stages) in &w.main {
        let at = s.world.quests.stage(quest);
        let reached = reached(&s, quest);
        let max = stages.iter().max().copied().unwrap_or(0);
        reached_total += reached;
        stages_total += stages.len();
        let reason = if at >= max {
            "done".to_owned()
        } else {
            let r = stall_reason(&w, &s, quest);
            *kinds.entry(reason_kind(&r)).or_default() += 1;
            r
        };
        eprintln!(
            "{quest}: {at} / {max}, stages {reached} of {}: {reason}",
            stages.len()
        );
    }
    let mut kinds: Vec<(String, usize)> = kinds.into_iter().collect();
    kinds.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    eprintln!("stall reasons (most common first):");
    for (k, n) in &kinds {
        eprintln!("  {n:3}  {k}");
    }
    eprintln!(
        "bot: {steps} steps, {} trials, {} result errors, {} stat effects dropped (no sheets), {} unhandled-function/variable reports, {:.1} s",
        bot.stats.trials,
        bot.stats.result_errors,
        bot.stats.stat_effects,
        bot.stats.reports.len(),
        started.elapsed().as_secs_f32()
    );
    eprintln!(
        "runner reports: {}",
        bot.stats
            .reports
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join("; ")
    );
    eprintln!("main quest: {reached_total} of {stages_total} stages reached");
}
