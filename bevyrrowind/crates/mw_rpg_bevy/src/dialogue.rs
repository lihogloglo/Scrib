//! Dialogue and the journal in the game (ROADMAP 2.4 / 2.2): the stock `sf_ui_screens` dialogue
//! window and journal, driven by `mw_rpg` sessions and the result runner.
//!
//! - [`Rpg`] (Bevy-free core, a resource): the game's `MwView` + `MwWorld` over the live state
//!   (actors met, player facts, globals, locals, journal), conversations ([`Talk`]) and the
//!   effects of result scripts, applied as `OpenMW` does (see [`Rpg::apply`]). It is also the
//!   journal's [`JournalSource`] (`Content` gives the labels).
//! - [`add`]: the app glue (after `DefaultPlugins`). Unthemed (the screens' fallback look); the
//!   HUD and Morrowind theme are later slices.
//!
//! Opening a conversation: the player activates an NPC (E on the focused placement,
//! `scripts.rs` writes `Activated`); an activation no script handled opens the window
//! ([`talk_on_activate`]). `J` opens and closes the journal (Back closes it too). While a menu is
//! open the player's controls are paused (`PlayerMode::paused`). Dev: `Shift+U` gives the player
//! Fargoth's ring (scaffold [`DevGiveRing`]).

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_camera_controller::free_camera::FreeCameraState;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_log::prelude::*;
use bevy_enhanced_input::prelude::EnhancedInputPlugin;
use mw_rpg::{
    ActorFacts, AiNudge, BoundItems, DispositionExtras, DispositionInput, GlobalNumber, MwDialogue, MwDialogueRules, MwResults, MwSheets, MwSlot,
    MwView, MwWorld, NoSheets, NpcStates, PersuasionSession, ResultCall, ResultEffect, ResultRunner, ResultState, ScriptVar, Target, VarType, clothing_value, rank_name, record_clothing_value,
};
use serde::{Deserialize, Serialize};
use sf_character_bevy::{Character, CharacterPlacement};
use sf_records::{RecordDb, RecordFormat, RecordId, apply_record_file};
use sf_rpg::{
    ActorKey, Attrs, CharacterRpg, ConversationCtx, DialogueOutput, DialogueSession,
    HeardLine, Holder, JournalEntry, PlayerId, RpgCommand, RpgEvent, SessionState, TopicKind, WorldRpg,
};
use sf_rpg::service::{ServiceEvent, ServiceSessions, WorldServices};
use sf_rpg_ui::{Alphabet, DialogueRequest, DialogueView, JournalLabels, JournalScreen, JournalSeen};
use sf_script_bevy::{ActivationOutcome, PLAYER, ScriptPlacements};
use sf_ui::{Layer, UiRegistry, register_kinds};
use sf_ui_bevy::{
    MessageBoxConfig, MessageBoxes, ScreenStackRes, SfUiPlugin, Toasts, UiRegistryRes,
    UiScreenRoot, UiTextPlugin,
};
use sf_ui_screens::{
    DialogueRequestEvent, DialogueScreen, JournalSeenChanged, JournalSource, JournalUi,
    JournalUiPlugin, ScreenSystems, SfScreensPlugin,
};
use tes3::esp::{FilterComparison, FilterFunction, FilterType, FilterValue, GlobalValue, GlobalVariable, Npc, Script};
use sf_world_bevy::StreamCell;
use bevy_ecs::system::SystemParam;
use mw_game_core::player::HOST_KEY;
use mw_stats::{MwGmst, MwTable};
use sf_rpg::PlayerKey;
use sf_stats_bevy::sf_stats::{CauseKind, Rules, Sheet, StatOwner};
use sf_stats_bevy::{StatCommand, StatIndex, StatRules, StatTick, Stats};

use crate::stats::MwStats;

mod persuasion;
mod services;
pub mod service_list_screen;
pub mod trade_screen;
pub use persuasion::{PERSUASION_ACTION, PERSUASION_OPTIONS, PersuadeRefused, Persuaded, persuasion_kind};
pub use services::{ServiceEventMessage, TradeQuotes, seed_store, trade_config};

/// Fargoth's ring (scaffold [`DevGiveRing`]).
const RING: &str = "ring_keley";
/// The journal key (modern default; Morrowind's is also J).
const JOURNAL_KEY: KeyCode = KeyCode::KeyJ;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

// ============================================================ the Bevy-free core

/// What a [`DispositionModel`] reads: one speaker's disposition towards one player (co-op: the
/// player by id, never a singleton).
pub struct DispositionCtx<'a> {
    pub sheets: &'a dyn MwSheets,
    pub records: &'a RecordDb,
    pub speaker: ActorKey,
    pub facts: &'a ActorFacts,
    pub player: PlayerId,
    /// The player's race id (`None`: the raceless placeholder player, until chargen).
    pub player_race: Option<&'a RecordId>,
    /// The NPC record's disposition.
    pub base: i32,
    /// The sum of permanent changes: `ModDisposition` / `SetDisposition` from result scripts and
    /// persuasion ([`Rpg::persuade`]).
    pub modifier: i32,
}

/// The speaker's disposition towards the player, as the filters and persuasion read it. A seam
/// (rule-hooks.md; future rule id `mw.disposition.derived`): the game's model is
/// [`DerivedDisposition`] (stats-adapter.md open question 2, owner 2026-11-08: `OpenMW`'s
/// formula ported); [`ContentDisposition`] is the content-only value.
pub trait DispositionModel: Send + Sync {
    /// The disposition, 0..=100.
    fn disposition(&self, cx: &DispositionCtx<'_>) -> i32;

    /// The unclamped value with a zero base and modifier and without Charm (`goodbyeSelected`'s
    /// `zero`: persuasion's goodbye keeps the derived value inside 0..=100 with it).
    fn without_base(&self, cx: &DispositionCtx<'_>) -> i32 {
        let _ = cx;
        0
    }
}

/// Content data only: record base plus permanent changes, clamped to 0..=100 as `OpenMW`
/// clamps the value the filters see.
pub struct ContentDisposition;

impl DispositionModel for ContentDisposition {
    fn disposition(&self, cx: &DispositionCtx<'_>) -> i32 {
        (cx.base + cx.modifier).clamp(0, 100)
    }
}

/// `OpenMW`'s `getDerivedDisposition` ([`mw_rpg::derived_disposition`]: race, the player's
/// Personality, faction standing, bounty), over the GMSTs the records give. Falls back to
/// [`ContentDisposition`] where the formula has no input: a creature, or no stats loaded.
pub struct DerivedDisposition {
    pub gmst: MwGmst,
}

impl DerivedDisposition {
    /// The formula's input with this base and modifier. Not ours yet, so 0 / false:
    /// TODO(crime system, ROADMAP crime): `crime_modifier` (`getCrimeDispositionModifier`);
    /// TODO(effect runtime, `sf_effects`): `diseased` (common or blight) and `charm` (Charm
    /// magnitude on the speaker); TODO(combat stance, `mw_player_bevy::combat`): `weapon_drawn`.
    fn input(cx: &DispositionCtx<'_>, base: i32, modifier: i32) -> Option<DispositionInput> {
        let extras = DispositionExtras { base, modifier, ..DispositionExtras::default() };
        mw_rpg::disposition_input(cx.sheets, cx.records, cx.player, cx.player_race, cx.facts, extras)
    }
}

impl DispositionModel for DerivedDisposition {
    fn disposition(&self, cx: &DispositionCtx<'_>) -> i32 {
        match Self::input(cx, cx.base, cx.modifier) {
            Some(i) => mw_rpg::derived_disposition(&i, &self.gmst),
            None => ContentDisposition.disposition(cx),
        }
    }

    /// Charm is 0 until the effect runtime (see [`DerivedDisposition::input`]), so this is the
    /// unclamped formula over a zero base.
    fn without_base(&self, cx: &DispositionCtx<'_>) -> i32 {
        Self::input(cx, 0, 0).map_or(0, |i| mw_rpg::derived_disposition_unclamped(&i, &self.gmst))
    }
}

/// An actor the player can talk to: its record facts and script locals' declarations.
#[derive(Clone, Debug)]
struct Actor {
    facts: ActorFacts,
    base_disposition: i32,
    /// The record's base Fight and Flee (`AIDT`), what persuasion's Intimidate and Taunt nudge.
    ai: AiNudge,
    declared: HashMap<RecordId, VarType>,
}

/// The saved part of the RPG state (the `saves` lane serializes it; ROADMAP 2.5).
/// Actor keys are entity bits for now: persistent reference ids arrive with saves.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RpgState {
    pub world: WorldRpg,
    pub pc: CharacterRpg,
    pub npcs: NpcStates,
    pub inventories: BTreeMap<ActorKey, CharacterRpg>,
    pub locals: BTreeMap<(ActorKey, RecordId), f32>,
    pub globals: BTreeMap<RecordId, f32>,
}

/// Read-only content and per-session facts.
struct Content {
    records: Arc<RecordDb>,
    dialogue: MwDialogue,
    /// GLOB records: type and initial value (current values are in [`RpgState::globals`]).
    global_types: HashMap<RecordId, GlobalNumber>,
    /// Exterior cell names by grid (the `Cell` filter reads the speaker's cell name).
    exterior_names: HashMap<(i32, i32), String>,
    actors: HashMap<ActorKey, Actor>,
    player: ActorFacts,
    /// Scaffold `StartingClothesFallback`: `PcClothingModifier` while the player's loadout
    /// wears nothing, the value of the `player` record's armor and clothing. The body wears
    /// them (`mw_actors` dresses the avatar from the record) but the RPG loadout starts empty,
    /// so without this every NPC greets a naked player. Removal path: the new game puts the
    /// record's starting inventory, equipped, in `RpgState::pc`.
    starting_clothes: u64,
    /// Script locals the skipped chargen left on NPCs, by base id (scaffold `SkipChargen`):
    /// the initial value [`Rpg::add_actor`] gives a local the actor has no value for yet.
    chargen_locals: HashMap<RecordId, Vec<(RecordId, f32)>>,
    disposition: Box<dyn DispositionModel>,
    /// The records' GMSTs (persuasion; vanilla values for missing ones).
    gmst: MwGmst,
    /// Bound item ids (`sMagicBound*`), never traded.
    bound: BoundItems,
}

/// One conversation (host side).
pub struct Talk {
    pub session: DialogueSession,
    pub speaker: ActorKey,
    /// Persuasion's bookkeeping for this conversation ([`Rpg::persuade`], settled at goodbye).
    pub persuasion: PersuasionSession,
}

/// The game's RPG host: content, state, the result runner.
#[derive(Resource)]
pub struct Rpg {
    content: Content,
    pub state: RpgState,
    results: MwResults,
    /// Message boxes and sounds result scripts asked for (UI and audio events, not saved).
    events: Vec<ResultEffect>,
    /// Sheet changes result scripts asked for (stats.md S7), sent as `StatCommand`s by
    /// `send_stat_commands`; the next fixed tick applies them.
    stat_commands: Vec<StatCommand>,
    /// Base Fight and Flee persuasion changed (Intimidate, Taunt), by actor; the record's
    /// otherwise. TODO(AI settings, ROADMAP Phase 3 AI): no per-actor AI settings exist yet; move
    /// these into them (saved, read by the AI) when they land. Unsaved until then.
    ai_settings: HashMap<ActorKey, AiNudge>,
    /// Merchant purses on the game clock, saved as `sf/services` (`mw_saves_bevy`).
    pub services: WorldServices,
    /// Open trade sessions, by id and player (not saved: staged items never moved).
    sessions: ServiceSessions,
    /// Service events for the screens since the last [`Rpg::take_service_events`].
    service_events: Vec<ServiceEvent>,
    /// Open list services (training...), by player and provider, with the rows last listed
    /// (not saved, like sessions).
    lists: Vec<services::OpenList>,
    /// Game hours passed by services since the game clock last took them
    /// ([`services::advance_sky`]).
    sky_hours: f32,
    /// The game clock's instant at the last [`services::sync_clock`] (clock.md): what journal
    /// stamps, purses and [`Rpg::game_time`] read. `None` without a clock (headless): the
    /// `MWScript` calendar globals until C3 derives them from the clock.
    clock: Option<sf_sky_bevy::GameInstant>,
}

impl Content {
    /// The disposition model's context for `speaker` (registered as `actor`) towards `player`.
    /// TODO(co-op): `player`'s race comes from the host's placeholder facts (`Content::player`).
    fn disposition_ctx<'a>(
        &'a self,
        state: &RpgState,
        sheets: &'a dyn MwSheets,
        player: PlayerId,
        speaker: ActorKey,
        actor: &'a Actor,
    ) -> DispositionCtx<'a> {
        DispositionCtx {
            sheets,
            records: &self.records,
            speaker,
            facts: &actor.facts,
            player,
            player_race: self.player.race.as_ref(),
            base: actor.base_disposition,
            modifier: state.npcs.modifier(speaker),
        }
    }

    /// The actor a result-script target names (`"arrille"->ModDisposition`): the first actor
    /// met with that base record.
    fn by_base(&self, base: &RecordId) -> Option<ActorKey> {
        self.actors.iter().find(|(_, a)| a.facts.id == *base).map(|(k, _)| *k)
    }

    fn target(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        match target {
            Target::Speaker => Some(ctx.speaker),
            Target::Actor(base) => self.by_base(base),
            Target::Player => None,
        }
    }
}

/// [`MwView`] + [`MwWorld`] for one speaker. `sheets`: the character sheets the PC and
/// speaker filter functions read ([`DialogueSheets`] in the app).
struct View<'a> {
    c: &'a Content,
    s: &'a RpgState,
    sheets: &'a dyn MwSheets,
}

impl View<'_> {
    fn global_value(&self, name: &RecordId) -> Option<GlobalNumber> {
        let ty = *self.c.global_types.get(name)?;
        Some(match (ty, self.s.globals.get(name)) {
            #[allow(clippy::cast_possible_truncation)]
            (GlobalNumber::Int(_), Some(v)) => GlobalNumber::Int(*v as i32),
            (GlobalNumber::Float(_), Some(v)) => GlobalNumber::Float(*v),
            (initial, None) => initial,
        })
    }

    /// A declared local of `actor`'s script (`OpenMW`: undeclared reads fail the filter).
    fn actor_local(&self, actor: ActorKey, name: &RecordId) -> Option<(VarType, f32)> {
        let ty = *self.c.actors.get(&actor)?.declared.get(name)?;
        Some((ty, self.s.locals.get(&(actor, name.clone())).copied().unwrap_or(0.0)))
    }

    fn target(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        self.c.target(ctx, target)
    }
}

impl MwView for View<'_> {
    fn speaker(&self, ctx: &ConversationCtx) -> Option<&ActorFacts> {
        self.c.actors.get(&ctx.speaker).map(|a| &a.facts)
    }

    fn player(&self, _player: PlayerId) -> Option<&ActorFacts> {
        Some(&self.c.player)
    }

    /// From the player's sheet (factions joined through `MWScript`, stats S7).
    fn player_rank(&self, player: PlayerId, faction: &RecordId) -> Option<i32> {
        mw_rpg::player_rank(self.sheets, player, faction)
    }

    fn global(&self, id: &RecordId) -> Option<GlobalNumber> {
        self.global_value(id)
    }

    fn local(&self, speaker: ActorKey, name: &RecordId) -> Option<f32> {
        self.actor_local(speaker, name).map(|(_, v)| v)
    }

    fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.s.world.quests.stage(quest)
    }

    fn item_count(&self, _player: PlayerId, item: &RecordId) -> u64 {
        self.s.pc.loadout.count(item)
    }

    /// No deaths tracked yet (Phase 3 combat).
    fn dead_count(&self, _base: &RecordId) -> i32 {
        0
    }

    fn disposition(&self, ctx: &ConversationCtx) -> i32 {
        let Some(actor) = self.c.actors.get(&ctx.speaker) else { return 0 };
        let cx = self.c.disposition_ctx(self.s, self.sheets, ctx.player, ctx.speaker, actor);
        self.c.disposition.disposition(&cx)
    }

    fn rank_name(&self, faction: &RecordId, rank: i32) -> Option<String> {
        rank_name(&self.c.records, faction, rank)
    }

    /// The sheets' functions (`mw_rpg::sheet_function`) and `PcClothingModifier` from what the
    /// player wears. TODO(co-op): `RpgState::pc` is the host's loadout; a guest's needs its own.
    #[allow(clippy::cast_precision_loss)] // item values are far below 2^24
    fn function(&self, ctx: &ConversationCtx, function: FilterFunction) -> Option<f32> {
        if function == FilterFunction::PcClothingModifier {
            let loadout = &self.s.pc.loadout;
            let wears = loadout.equipped().any(|(slot, _)| slot.0 <= MwSlot::Belt as u8);
            let value = if wears { clothing_value(&self.c.records, loadout) } else { self.c.starting_clothes };
            return (ctx.player == PlayerId::HOST).then_some(value as f32);
        }
        mw_rpg::sheet_function(self.sheets, &self.c.records, ctx, self.speaker(ctx)?, function)
    }
}

impl MwWorld for View<'_> {
    fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.global_value(name)
    }

    fn var(&self, ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        let (ty, value) = self.actor_local(self.target(ctx, target)?, name)?;
        Some(ScriptVar { ty, value })
    }

    fn item_count(&self, ctx: &ConversationCtx, target: &Target, item: &RecordId) -> u64 {
        match target {
            Target::Player => self.s.pc.loadout.count(item),
            other => self
                .target(ctx, other)
                .and_then(|k| self.s.inventories.get(&k))
                .map_or(0, |inv| inv.loadout.count(item)),
        }
    }

    fn sheets(&self) -> &dyn MwSheets {
        self.sheets
    }

    fn actor(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        self.target(ctx, target)
    }

    fn facts(&self, ctx: &ConversationCtx, target: &Target) -> Option<&ActorFacts> {
        let key = self.target(ctx, target)?;
        self.c.actors.get(&key).map(|a| &a.facts)
    }
}

/// The journal screen reads the host's world state and the content's labels.
impl JournalSource for Rpg {
    fn world(&self) -> &WorldRpg {
        &self.state.world
    }

    fn labels(&self) -> &dyn JournalLabels {
        &self.content
    }
}

/// Journal labels: quest names from the journal's name INFO, topic names from DIAL.
impl JournalLabels for Content {
    fn quest_name(&self, quest: &RecordId) -> Option<String> {
        self.dialogue.journal.quest(quest).and_then(|q| q.name.clone())
    }

    fn topic_name(&self, topic: &RecordId) -> Option<String> {
        self.dialogue.db.topic(topic).map(|t| t.name.clone())
    }

    /// Placeholder until the Morrowind calendar lands with the clock (saves, ROADMAP 2.5).
    fn day_heading(&self, day: u32) -> String {
        format!("Day {}", day + 1)
    }

    fn entry_text(&self, _entry: &JournalEntry) -> Option<String> {
        None
    }
}

/// The script locals a script declares (`short state` ...), from its source text.
fn declared_locals(text: &str) -> HashMap<RecordId, VarType> {
    let parsed = mw_script::parse(text, mw_script::SourceKind::Script, mw_script::FunctionTable::vanilla());
    parsed.script.locals.iter().map(|l| (id(&l.name.key), l.ty)).collect()
}

impl Rpg {
    /// Over the loaded records (DIAL/INFO, GLOB, CELL names).
    pub fn new(records: Arc<RecordDb>) -> Result<Self, String> {
        let dialogue = MwDialogue::from_records(&records).map_err(|e| e.to_string())?;
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
        let exterior_names = records
            .iter::<tes3::esp::Cell>()
            .filter(|(_, c)| !c.is_interior() && !c.name.is_empty())
            .map(|(_, c)| (c.data.grid, c.name.clone()))
            .collect();
        let starting_clothes = records.get::<Npc>(&id("player")).map_or(0, |p| record_clothing_value(&records, p));
        info!("dialogue: PcClothingModifier is {starting_clothes} (the player record's clothes) while nothing is worn (scaffold StartingClothesFallback)");
        // Scaffold `PlayerFactsPlaceholder` (no character generation yet): a nameless,
        // raceless player, so race/sex filters stay neutral. Removal path: chargen (M7).
        let player = ActorFacts {
            id: id("player"),
            name: "Outlander".into(),
            npc: true,
            rank: -1,
            ..ActorFacts::default()
        };
        // Missing GMSTs take vanilla values (the stats glue logs them).
        let (gmst, _) = MwGmst::from_db(&records);
        let bound = BoundItems::from_records(&records);
        Ok(Self {
            content: Content {
                records,
                dialogue,
                global_types,
                exterior_names,
                actors: HashMap::new(),
                player,
                starting_clothes,
                chargen_locals: HashMap::new(),
                disposition: Box::new(DerivedDisposition { gmst: gmst.clone() }),
                gmst,
                bound,
            },
            state: RpgState::default(),
            results: MwResults::new(),
            events: Vec::new(),
            stat_commands: Vec::new(),
            ai_settings: HashMap::new(),
            services: WorldServices::default(),
            sessions: ServiceSessions::new(),
            service_events: Vec::new(),
            lists: Vec::new(),
            sky_hours: 0.0,
            clock: None,
        })
    }

    /// New game without character generation (scaffold `SkipChargen`; removal path: chargen,
    /// M7): the state the end of chargen leaves that dialogue reads. As `OpenMW`'s bypassed new
    /// game (`apps/openmw/mwworld/worldimp.cpp` `World::startNewGame`, 0.49.0), `chargenstate`
    /// is -1 (chargen over: chargen-only lines stop matching). Beyond `OpenMW` (whose bypass
    /// leaves Sellus Gravius asking for papers), the chargen captain's hand-off lines run in the
    /// result runner as if chargen had reached them, and two kinds of effect are kept, so what
    /// is granted is the data's and a mod's edit of those lines counts:
    /// - `AddTopic`s of his "duties" result (the package for Caius; "latest rumors", "little
    ///   advice", ...): the topics are learned.
    /// - his own `set <local>`s from the greeting that takes the release papers
    ///   (`chargen statssheet`; `set state to -1`): his post-chargen greeting. Applied when he is
    ///   registered ([`Rpg::add_actor`]), since his actor key exists only once his cell loads.
    ///
    /// Their other effects (journal, package, gold, the papers' removal) start the main quest
    /// and stay out. Returns the topics learned.
    pub fn skip_chargen(&mut self) -> Vec<RecordId> {
        const HANDOFF_TOPIC: &str = "duties";
        const HANDOFF_SPEAKER: &str = "chargen captain";
        const PAPERS: &str = "chargen statssheet";
        const SPEAKER: ActorKey = ActorKey(u128::MAX);
        self.state.globals.insert(id("chargenstate"), -1.0);
        // The papers hand-in: a greeting that needs the player to hold the papers.
        let takes_papers = |f: &tes3::esp::Filter| {
            f.filter_type == FilterType::Item
                && f.id.eq_ignore_ascii_case(PAPERS)
                && f.comparison == FilterComparison::GreaterEqual
                && f.value == FilterValue::Integer(1)
        };
        let handoff: Vec<(RecordId, RecordId, String)> = self
            .content
            .records
            .iter::<tes3::esp::DialogueInfo>()
            .filter(|(_, info)| info.speaker_id.eq_ignore_ascii_case(HANDOFF_SPEAKER))
            .filter_map(|(key, info)| {
                let (topic, _) = mw_records::keys::split_info(key)?;
                let handoff = topic == HANDOFF_TOPIC
                    || (topic.starts_with("greeting") && info.filters.iter().any(takes_papers));
                handoff.then(|| (id(topic), key.clone(), info.script_text.clone()))
            })
            .collect();
        if handoff.is_empty() || !self.add_actor(SPEAKER, &id(HANDOFF_SPEAKER), "") {
            return Vec::new();
        }
        let ctx = ConversationCtx { player: PlayerId::HOST, speaker: SPEAKER, speaker_base: id(HANDOFF_SPEAKER), choice: None };
        // Before the world runs: no sheets yet (the hand-off results read none).
        let sheets = &NoSheets;
        let mut topics = Vec::new();
        let mut locals = Vec::new();
        {
            let v = View { c: &self.content, s: &self.state, sheets };
            let state = ResultState { world: &self.state.world, player: &self.state.pc, env: &v };
            let mut runner = self.results.runner(state);
            for (topic, response, script) in &handoff {
                match runner.run(ResultCall { ctx: &ctx, topic, response, script }) {
                    Ok(effects) => {
                        for e in effects {
                            match e {
                                ResultEffect::AddTopic(t) if topic.as_str() == HANDOFF_TOPIC => topics.push(t),
                                ResultEffect::SetLocal { target: Target::Speaker, name, value } => locals.push((name, value)),
                                _ => {}
                            }
                        }
                    }
                    Err(e) => warn!("new game: chargen hand-off {response}: {e}"),
                }
            }
        }
        for report in self.results.drain_reports() {
            debug!("new game: {report}");
        }
        self.content.actors.remove(&SPEAKER);
        if !locals.is_empty() {
            self.content.chargen_locals.insert(id(HANDOFF_SPEAKER), locals);
        }
        for t in &topics {
            let _ = self.state.world.apply(RpgCommand::AddTopic(t.clone()));
        }
        topics
    }

    /// The loaded records (the console's `CenterOnCell` reads CELL, REGN, LAND and GMST).
    pub fn records(&self) -> &RecordDb {
        &self.content.records
    }

    /// The exterior cell name at a Morrowind grid (`None`: wilderness).
    pub fn exterior_name(&self, grid: (i32, i32)) -> Option<&str> {
        self.content.exterior_names.get(&grid).map(String::as_str)
    }

    /// Registers an NPC the player may talk to (`false`: not an NPC record). Locals the skipped
    /// chargen left on its base record start at their value unless the actor already has one.
    pub fn add_actor(&mut self, key: ActorKey, base: &RecordId, cell: &str) -> bool {
        let db = &self.content.records;
        let Some(npc) = db.get::<Npc>(base) else { return false };
        let declared = db.get::<Script>(&id(&npc.script)).map(|s| declared_locals(&s.text)).unwrap_or_default();
        let actor = Actor {
            facts: ActorFacts::from_npc(db, npc, cell),
            base_disposition: i32::from(npc.data.disposition),
            ai: AiNudge { flee: i32::from(npc.ai_data.flee), fight: i32::from(npc.ai_data.fight) },
            declared,
        };
        self.content.actors.insert(key, actor);
        for (name, value) in self.content.chargen_locals.get(base).into_iter().flatten() {
            self.state.locals.entry((key, name.clone())).or_insert(*value);
        }
        true
    }

    pub fn actor_name(&self, key: ActorKey) -> Option<&str> {
        self.content.actors.get(&key).map(|a| a.facts.name.as_str())
    }

    /// `base` is an NPC record (activating one talks). Talking creatures are a later slice.
    pub fn is_npc(&self, base: &RecordId) -> bool {
        self.content.records.get::<Npc>(base).is_some()
    }

    /// Is talking to `key` possible (an NPC registered by [`Rpg::add_actor`])?
    pub fn knows_actor(&self, key: ActorKey) -> bool {
        self.content.actors.contains_key(&key)
    }

    /// Result effects are waiting for [`Rpg::take_events`] (check before taking, so a
    /// `ResMut<Rpg>` isn't marked changed every frame: the journal re-lays out on changes).
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Message boxes and sounds since the last call.
    pub fn take_events(&mut self) -> Vec<ResultEffect> {
        std::mem::take(&mut self.events)
    }

    /// One dev-console `MWScript` line (vanilla's console, ROADMAP 1.13), run by the result
    /// runner: `selected` (the console's selection, a registered actor) is the implicit target,
    /// else the player. A line that doesn't compile, or calls a function the interpreter doesn't
    /// run yet, fails whole before any effect applies. Returns notes and what it did.
    pub fn run_console(&mut self, text: &str, selected: Option<ActorKey>, sheets: &dyn MwSheets) -> Result<Vec<String>, String> {
        use mw_script::Severity;
        /// The implicit target without a selection (never a registered actor).
        const NO_SELECTION: ActorKey = ActorKey(u128::MAX - 1);
        let speaker_base = match selected {
            Some(key) => self.content.actors.get(&key).map(|a| a.facts.id.clone()).ok_or(
                "the selection is no actor the scripts know yet (NPCs only); deselect to target the player",
            )?,
            None => id("player"),
        };
        let mut out = Vec::new();
        let parsed = mw_script::parse(text, mw_script::SourceKind::DialogueResult, mw_script::FunctionTable::vanilla());
        for d in parsed.diagnostics.iter() {
            match d.severity() {
                Severity::Error => return Err(d.to_string()),
                Severity::Repaired | Severity::Warning => out.push(format!("note: {d}")),
                Severity::Accepted => {}
            }
        }
        let ctx = ConversationCtx { player: PlayerId::HOST, speaker: selected.unwrap_or(NO_SELECTION), speaker_base, choice: None };
        // Its own runtime: the dialogue's parse cache and report-once log stay the dialogue's.
        let mut results = MwResults::new();
        let response = id("console");
        let effects = {
            let v = View { c: &self.content, s: &self.state, sheets };
            let state = ResultState { world: &self.state.world, player: &self.state.pc, env: &v };
            let call = ResultCall { ctx: &ctx, topic: &response, response: &response, script: text };
            results.runner(state).run(call).map_err(|e| e.to_string())?
        };
        for report in results.drain_reports() {
            match report {
                mw_rpg::ResultReport::Unhandled(name) => return Err(format!("{name}: not runnable from the console yet")),
                mw_rpg::ResultReport::Inert { reason, .. } => return Err(reason),
                mw_rpg::ResultReport::UnknownVariable(name) => out.push(format!("note: unknown variable {name}")),
            }
        }
        for mut effect in effects {
            if selected.is_none()
                && let ResultEffect::AddItem { target, .. }
                | ResultEffect::RemoveItem { target, .. }
                | ResultEffect::ModDisposition { target, .. }
                | ResultEffect::SetDisposition { target, .. }
                | ResultEffect::SetLocal { target, .. }
                | ResultEffect::Stats { target, .. } = &mut effect
                && *target == Target::Speaker
            {
                *target = Target::Player;
            }
            out.push(match &effect {
                ResultEffect::AddItem { target, item, count } => format!("added {count} {item} to {target:?}"),
                ResultEffect::RemoveItem { target, item, count } => format!("removed up to {count} {item} from {target:?}"),
                e => format!("{e:?}"),
            });
            // Journal/topic events reach the journal on its next change check.
            let _ = self.apply(None, &ctx, effect, sheets);
        }
        Ok(out)
    }

    /// Opens a conversation with a registered actor: the greeting, fed to a new view.
    pub fn open(&mut self, speaker: ActorKey, sheets: &dyn MwSheets) -> Option<(Talk, DialogueView)> {
        let actor = self.content.actors.get(&speaker)?;
        let name = actor.facts.name.clone();
        let ctx = ConversationCtx { player: PlayerId::HOST, speaker, speaker_base: actor.facts.id.clone(), choice: None };
        let (session, out) = {
            let view = View { c: &self.content, s: &self.state, sheets };
            let rules = MwDialogueRules::new(&self.content.dialogue, &view);
            DialogueSession::open(&self.content.dialogue.db, &rules, ctx)
        };
        let mut talk = Talk { session, speaker, persuasion: PersuasionSession::default() };
        let mut view = DialogueView::new(name);
        self.after(&mut talk, &mut view, &out, sheets);
        Some((talk, view))
    }

    /// Applies a request from the dialogue window and feeds the view: [`Rpg::request_at`] at
    /// tick 0 (tests, tools; the game passes its tick, which seeds persuasion rolls).
    pub fn request(&mut self, talk: &mut Talk, view: &mut DialogueView, request: &DialogueRequest, sheets: &dyn MwSheets) {
        self.request_at(talk, view, request, sheets, 0);
    }

    /// Applies a request from the dialogue window at simulation tick `tick` and feeds the view.
    /// Topics, choices and goodbye are session steps; [`DialogueRequest::Action`] is routed by
    /// [`Rpg::route_action`] (Persuasion options and service buttons).
    pub fn request_at(&mut self, talk: &mut Talk, view: &mut DialogueView, request: &DialogueRequest, sheets: &dyn MwSheets, tick: u64) {
        if let DialogueRequest::Action { action, option } = request {
            let _ = self.route_action(talk, view, action, option.as_ref(), sheets, tick);
            return;
        }
        let out = {
            let v = View { c: &self.content, s: &self.state, sheets };
            let rules = MwDialogueRules::new(&self.content.dialogue, &v);
            request.apply(&mut talk.session, &self.content.dialogue.db, &rules)
        };
        match out {
            Ok(out) => self.after(talk, view, &out, sheets),
            Err(e) => warn!("dialogue: {request:?} refused: {e}"),
        }
        if *talk.session.state() == SessionState::Ended {
            self.end_talk(talk, sheets);
        }
    }

    /// After a session call (`OpenMW` `DialogueManager::executeTopic` / `addTopicsFromText`):
    /// learn the linked topics the speaker answers, record heard topic lines for the journal's
    /// topic index (`Journal::addTopic`), run results in the game's runner and apply their
    /// effects in call order; then refresh the view.
    fn after(&mut self, talk: &mut Talk, view: &mut DialogueView, out: &[DialogueOutput], sheets: &dyn MwSheets) {
        let ctx = talk.session.ctx().clone();
        let dlg = &self.content.dialogue;
        view.apply_outputs(&dlg.db, out);
        let mut commands = Vec::new();
        let mut effects = Vec::new();
        {
            let v = View { c: &self.content, s: &self.state, sheets };
            let rules = MwDialogueRules::new(dlg, &v);
            let state = ResultState { world: &self.state.world, player: &self.state.pc, env: &v };
            let mut runner = self.results.runner(state);
            for o in out {
                match o {
                    DialogueOutput::Learned(t) if rules.speaker_answers(&ctx, t) => {
                        commands.push(RpgCommand::AddTopic(t.clone()));
                    }
                    DialogueOutput::Line { topic, response, text }
                        if dlg.db.topic(topic).is_some_and(|t| t.kind == TopicKind::Conversation) =>
                    {
                        let line = HeardLine { source: response.clone(), speaker: v.c.actors[&ctx.speaker].facts.name.clone(), text: text.clone() };
                        commands.push(RpgCommand::HearLine { topic: topic.clone(), line });
                    }
                    DialogueOutput::RunResult { topic, response } => {
                        let Some(info) = dlg.info(response) else { continue };
                        let call = ResultCall { ctx: &ctx, topic, response, script: &info.script_text };
                        match runner.run(call) {
                            Ok(e) => effects.extend(e),
                            Err(e) => warn!("dialogue: result of {response}: {e}"),
                        }
                    }
                    _ => {}
                }
            }
        }
        for report in self.results.drain_reports() {
            debug!("dialogue: {report}");
        }
        let mut events = Vec::new();
        for cmd in commands {
            events.extend(self.state.world.apply(cmd).unwrap_or_default());
        }
        for effect in effects {
            events.extend(self.apply(Some(&mut *talk), &ctx, effect, sheets));
        }
        view.apply_events(&events);
        // The action buttons follow the state the step left (a bribe's gold, ...).
        self.refresh_actions(talk, view);
        let v = View { c: &self.content, s: &self.state, sheets };
        let rules = MwDialogueRules::new(&self.content.dialogue, &v);
        view.refresh_from(&talk.session, &self.content.dialogue.db, &rules, &self.state.world.topics);
    }

    fn character(&mut self, ctx: &ConversationCtx, target: &Target) -> Option<(Holder, &mut CharacterRpg)> {
        if *target == Target::Player {
            return Some((Holder::Player(PlayerId::HOST), &mut self.state.pc));
        }
        let key = self.content.target(ctx, target)?;
        Some((Holder::Actor(key), self.npc_store(key)))
    }

    /// One result-script effect, with `OpenMW`'s semantics (`apps/openmw/mwscript/
    /// dialogueextensions.cpp`, `containerextensions.cpp`, `statsextensions.cpp` at
    /// openmw-0.48.0; the `mw_rpg` runner decodes the calls, this applies them to the world).
    /// `talk`: the conversation (`None`: a console line, where `Choice` and `Goodbye` do nothing).
    fn apply(&mut self, talk: Option<&mut Talk>, ctx: &ConversationCtx, effect: ResultEffect, sheets: &dyn MwSheets) -> Vec<RpgEvent> {
        let mut events = Vec::new();
        let target_key = |rpg: &Self, t: &Target| rpg.content.target(ctx, t);
        match effect {
            // `Journal`: adds the stage's entry (text with macros expanded now) and sets the index.
            ResultEffect::Journal { quest, index } => {
                let cmd = {
                    let v = View { c: &self.content, s: &self.state, sheets };
                    let rules = MwDialogueRules::new(&self.content.dialogue, &v);
                    self.content.dialogue.journal_command(&self.state.world, &quest, index, self.game_time(), |t| {
                        rules.expand(ctx, t)
                    })
                };
                if let Some(cmd) = cmd {
                    events.extend(self.state.world.apply(cmd).unwrap_or_default());
                }
            }
            ResultEffect::SetJournalIndex { quest, index } => {
                events.extend(self.state.world.apply(MwDialogue::set_journal_index(&quest, index)).unwrap_or_default());
            }
            ResultEffect::AddTopic(t) => events.extend(self.state.world.apply(RpgCommand::AddTopic(t)).unwrap_or_default()),
            ResultEffect::AddItem { target, item, count } => {
                if let Some((holder, inv)) = self.character(ctx, &target) {
                    let _ = inv.apply(RpgCommand::AddItem { holder, item, count, attrs: Attrs::new() });
                }
            }
            // `RemoveItem` removes what there is (no error past zero).
            ResultEffect::RemoveItem { target, item, count } => {
                if let Some((holder, inv)) = self.character(ctx, &target) {
                    let count = count.min(u32::try_from(inv.loadout.count(&item)).unwrap_or(u32::MAX));
                    if count > 0 {
                        let _ = inv.apply(RpgCommand::RemoveItem { holder, item, count });
                    }
                }
            }
            ResultEffect::Choices(choices) => {
                let Some(talk) = talk else {
                    warn!("console: Choice needs a conversation");
                    return events;
                };
                if let Err(e) = talk.session.offer_choices(choices) {
                    warn!("dialogue: Choice refused: {e}");
                }
            }
            ResultEffect::Goodbye => {
                if let Some(talk) = talk {
                    talk.session.force_goodbye();
                }
            }
            ResultEffect::ModDisposition { target, delta } => {
                if let Some(k) = target_key(self, &target) {
                    self.state.npcs.modify(k, delta);
                }
            }
            ResultEffect::SetDisposition { target, value } => {
                if let Some(k) = target_key(self, &target) {
                    let base = self.content.actors.get(&k).map_or(0, |a| a.base_disposition);
                    self.state.npcs.set(k, base, value);
                }
            }
            ResultEffect::SetLocal { target, name, value } => {
                if let Some(k) = target_key(self, &target) {
                    self.state.locals.insert((k, name), value);
                }
            }
            ResultEffect::SetGlobal { name, value } => {
                self.state.globals.insert(name, value);
            }
            e @ (ResultEffect::MessageBox { .. } | ResultEffect::PlaySound(_)) => self.events.push(e),
            // The context player's sheet, or the actor's by its key (net rule 6: never a singleton).
            ResultEffect::Stats { target, ops } => {
                let owner = match &target {
                    Target::Player => player_key(ctx.player).map(StatOwner::Player),
                    other => target_key(self, other).map(StatOwner::Actor),
                };
                if let Some(owner) = owner {
                    self.stat_commands.push(StatCommand::new(owner, CauseKind::Script, ops));
                } else {
                    debug!("result scripts: stats of {target:?} have no sheet owner");
                }
            }
        }
        events
    }
}

// ============================================================ the app glue

/// Screens of the game so far: the dialogue window, the journal, the service screens over the
/// dialogue (trade: [`trade_screen`], service list: [`service_list_screen`]) and message boxes.
/// TODO(HUD slice): these records belong in the builtin mod's data (everything is a mod) once
/// more screens land.
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "dialogue"
value = { layer = "Menu", implementation = { Template = "dialogue" } }

[[define]]
kind = "ui.screen"
id = "journal"
value = { layer = "Menu", implementation = { Template = "journal" } }

[[define]]
kind = "ui.screen"
id = "trade"
value = { layer = "Menu", implementation = { Template = "trade" } }

[[define]]
kind = "ui.screen"
id = "service"
value = { layer = "Menu", implementation = { Template = "service" } }

[[define]]
kind = "ui.screen"
id = "sf.message_box"
value = { layer = "Modal", input = "modal", implementation = { Template = "message_box" } }
"#;

fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    for (records, what) in [(SCREENS, "game screens"), (mw_boot_bevy::ui_theme::THEME_RECORD, "UI theme")] {
        let outcome = apply_record_file(&mut db, base, records, RecordFormat::Toml).expect(what);
        assert!(outcome.errors.is_empty(), "{what}: {:?}", outcome.errors);
    }
    UiRegistry::build(&db).0
}

/// An actor the player can talk to (registered in [`Rpg`]).
#[derive(Component)]
pub struct Talkable;

/// The open conversation and its screen.
#[derive(Resource)]
struct ActiveTalk {
    talk: Talk,
    screen: Entity,
}

/// The player's journal "new" markers for this session. TODO(saves, ROADMAP 2.5): keep it with
/// the player's profile (`JournalSeenChanged` is the hook), not in the world save.
#[derive(Resource, Default)]
struct JournalSeenStore(JournalSeen);

/// Scaffold `DevGiveRing`: `Shift+U` puts Fargoth's ring in the player's inventory, so the
/// flow can be played before containers and pickup exist. Removal path: 2.2 U5 inventory
/// wiring plus item pickup (the ring is in the Census office in the real flow).
struct DevGiveRing;

/// The dialogue window, the journal and their hosting, after `DefaultPlugins`. `db`: the
/// loaded records (`None`: no actors, no dialogue). `open_ui`: `--open-ui`, a screen opened at
/// start for dev screenshots ([`OpenAtStart`]).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>, open_ui: Option<String>) {
    let Some(db) = db else { return };
    let mut rpg = match Rpg::new(db) {
        Ok(r) => r,
        Err(e) => {
            warn!("dialogue off: {e}");
            return;
        }
    };
    let granted = rpg.skip_chargen();
    info!("new game without chargen: {} topics from the chargen hand-off", granted.len());
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    app.add_plugins((SfUiPlugin, UiTextPlugin, SfScreensPlugin, JournalUiPlugin::<Rpg>::default()))
        .add_message::<ActivationOutcome>()
        .add_message::<ServiceEventMessage>()
        .insert_resource(UiRegistryRes(registry()))
        .insert_resource(rpg)
        .init_resource::<JournalSeenStore>()
        .insert_resource(OpenAtStart(open_ui))
        .add_systems(
            Update,
            (track_actors, dev_give_ring, talk_on_activate, toggle_journal, open_at_start).chain().before(ScreenSystems::Spawn),
        )
        .add_systems(
            Update,
            (
                services::sync_clock,
                host_dialogue,
                close_screens,
                keep_journal_seen,
                show_events,
                send_stat_commands,
                services::relist_services,
                services::send_service_events,
                services::advance_sky,
                free_camera_lock,
                pause_player,
            )
                .chain()
                .after(ScreenSystems::Input),
        );
    service_list_screen::add(app);
    // The Morrowind theme over the stock screens (content: the builtin mod's sheet).
    mw_boot_bevy::ui_theme::add(app);
}

/// An actor's key: its placement id (stable across cell loads, what saves address), else the
/// entity (characters not spawned from a placement).
pub fn key(e: Entity, placement: Option<&CharacterPlacement>) -> ActorKey {
    placement.map_or(ActorKey(u128::from(e.to_bits())), |p| ActorKey(p.0.0))
}

/// A player id's character key. TODO(co-op): the session roster maps a joined player's id to
/// its profile key; until then only the host has a character.
fn player_key(player: PlayerId) -> Option<PlayerKey> {
    (player == PlayerId::HOST).then_some(HOST_KEY)
}

/// The live character sheets dialogue reads ([`MwSheets`] over `sf_stats_bevy`): a player's
/// by its key, an actor's by its placement key ([`key`]), through `StatIndex`. Without stats
/// (no records) every sheet function is neutral.
#[derive(SystemParam)]
pub struct DialogueSheets<'w, 's> {
    stats: Option<Res<'w, MwStats>>,
    rules: Option<Res<'w, StatRules>>,
    index: Option<Res<'w, StatIndex>>,
    sheets: Query<'w, 's, &'static Stats>,
}

impl DialogueSheets<'_, '_> {
    fn sheet(&self, owner: StatOwner) -> Option<&Sheet> {
        let e = self.index.as_ref()?.get(owner)?;
        self.sheets.get(e).ok().map(|s| &s.sheet)
    }
}

impl MwSheets for DialogueSheets<'_, '_> {
    fn table(&self) -> Option<&MwTable> {
        self.stats.as_deref().map(|s| &*s.table)
    }

    fn player(&self, player: PlayerId) -> Option<&Sheet> {
        self.sheet(StatOwner::Player(player_key(player)?))
    }

    fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
        self.sheet(StatOwner::Actor(actor))
    }

    fn rules(&self) -> Option<&Rules> {
        self.rules.as_deref().map(|r| &*r.0)
    }
}

/// [`DialogueSheets`] over a `&World` (the console runs with the world; take [`Rpg`] out with
/// `World::resource_scope` first).
pub struct WorldSheets<'w>(pub &'w World);

impl WorldSheets<'_> {
    fn sheet(&self, owner: StatOwner) -> Option<&Sheet> {
        let e = self.0.get_resource::<StatIndex>()?.get(owner)?;
        self.0.get::<Stats>(e).map(|s| &s.sheet)
    }
}

impl MwSheets for WorldSheets<'_> {
    fn table(&self) -> Option<&MwTable> {
        self.0.get_resource::<MwStats>().map(|s| &*s.table)
    }

    fn player(&self, player: PlayerId) -> Option<&Sheet> {
        self.sheet(StatOwner::Player(player_key(player)?))
    }

    fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
        self.sheet(StatOwner::Actor(actor))
    }

    fn rules(&self) -> Option<&Rules> {
        self.0.get_resource::<StatRules>().map(|r| &*r.0)
    }
}

/// The speaker's cell name for the `Cell` filter: the interior's name, or the exterior cell's
/// name at the streaming cell's grid (empty in the wilderness).
fn cell_name(rpg: &Rpg, e: Entity, parents: &Query<&ChildOf>, cells: &Query<&StreamCell>) -> String {
    let mut current = e;
    loop {
        if let Ok(cell) = cells.get(current) {
            if cell.worldspace == mw_world::ids::exterior() {
                return rpg.exterior_name((cell.cell.x, cell.cell.y)).unwrap_or_default().to_owned();
            }
            return cell.worldspace.as_str().to_owned();
        }
        let Ok(parent) = parents.get(current) else { return String::new() };
        current = parent.parent();
    }
}

/// NPCs that appear become talkable.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn track_actors(
    mut commands: Commands,
    mut rpg: ResMut<Rpg>,
    fresh: Query<(Entity, &Character, Option<&CharacterPlacement>), (Added<Character>, Without<mw_game_core::player::PlayerAvatar>)>,
    parents: Query<&ChildOf>,
    cells: Query<&StreamCell>,
) {
    for (e, character, placement) in &fresh {
        let Character::Record { id, .. } = character else { continue };
        let cell = cell_name(&rpg, e, &parents, &cells);
        if rpg.add_actor(key(e, placement), id, &cell) {
            commands.entity(e).try_insert(Talkable);
        }
    }
}

/// A menu (dialogue, journal) is open.
fn menu_open(stack: &ScreenStackRes) -> bool {
    stack.0.top(Layer::Menu).is_some()
}

fn shift(keys: &ButtonInput<KeyCode>) -> bool {
    keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
}

/// Dev: `Shift+U` gives the player Fargoth's ring (scaffold [`DevGiveRing`]).
#[allow(clippy::needless_pass_by_value)]
fn dev_give_ring(keys: Res<ButtonInput<KeyCode>>, mut rpg: ResMut<Rpg>) {
    if !keys.just_pressed(KeyCode::KeyU) || !shift(&keys) {
        return;
    }
    let _ = DevGiveRing;
    let ring = id(RING);
    let _ = rpg.state.pc.apply(RpgCommand::AddItem {
        holder: Holder::Player(PlayerId::HOST),
        item: ring.clone(),
        count: 1,
        attrs: Attrs::new(),
    });
    info!("dialogue: dev gave {RING} (now {})", rpg.state.pc.loadout.count(&ring));
}

/// The default action of activating an NPC: talk. Reads the player's activations no script
/// handled (`ActivationOutcome`, as doors and sounds do) whose target placement is an NPC
/// record. `OpenMW` does the same in `MWClass::Npc::activate` (apps/openmw/mwclass/npc.cpp,
/// 0.49.0: `ActionTalk` for a living, non-hostile NPC); death, combat, sneak-pickpocket and
/// hostility are not modelled yet, so every registered NPC talks.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // Bevy system params
fn talk_on_activate(
    mut commands: Commands,
    mut outcomes: MessageReader<ActivationOutcome>,
    placements: Option<Res<ScriptPlacements>>,
    mut rpg: ResMut<Rpg>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    active: Option<Res<ActiveTalk>>,
    sheets: DialogueSheets,
) {
    let Some(placements) = placements else {
        outcomes.clear();
        return;
    };
    let talk_to = outcomes
        .read()
        .filter(|o| !o.handled && o.activation.actor == PLAYER)
        .filter(|o| placements.get(o.activation.target).is_some_and(|p| rpg.is_npc(&id(&p.record))))
        .map(|o| ActorKey(o.activation.target.0))
        .last();
    let Some(speaker) = talk_to else { return };
    if active.is_some() || menu_open(&stack) {
        return;
    }
    if rpg.knows_actor(speaker) {
        open_dialogue(&mut commands, &mut rpg, &mut stack, &registry, speaker, &sheets);
    } else {
        warn!("dialogue: activated NPC {:032x} was never registered (no character spawned?)", speaker.0);
    }
}

/// Opens the dialogue window on `speaker`.
fn open_dialogue(commands: &mut Commands, rpg: &mut Rpg, stack: &mut ScreenStackRes, registry: &UiRegistryRes, speaker: ActorKey, sheets: &dyn MwSheets) {
    let Some((talk, view)) = rpg.open(speaker, sheets) else { return };
    info!("dialogue: talking to {}", rpg.actor_name(talk.speaker).unwrap_or("?"));
    let screen = commands.spawn((UiScreenRoot("dialogue".into()), DialogueScreen::new(view))).id();
    commands.insert_resource(ActiveTalk { talk, screen });
    if let Err(e) = stack.0.push(&registry.0, "dialogue") {
        warn!("dialogue: {e:?}");
    }
}

/// `J` opens the journal when no other menu is open and closes it when it is (Back closes it
/// too, through the screen stack). The journal opens at its newest spread.
#[allow(clippy::needless_pass_by_value)]
fn toggle_journal(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    seen: Res<JournalSeenStore>,
) {
    if !keys.just_pressed(JOURNAL_KEY) || shift(&keys) {
        return;
    }
    if stack.0.is_open("journal") {
        stack.0.close("journal");
        return;
    }
    if menu_open(&stack) {
        return;
    }
    let screen = JournalScreen::new(Alphabet::default(), 2, seen.0.clone());
    commands.spawn((UiScreenRoot("journal".into()), JournalUi::new(screen)));
    if let Err(e) = stack.0.push(&registry.0, "journal") {
        warn!("journal: {e:?}");
    }
}

/// Dev scaffold `--open-ui <screen>`: opens `journal`, `dialogue` (with the first NPC tracked, once
/// the cell's characters are in) or a sample `message` box, once at start,
/// so `--screenshot` runs show the theme without keyboard input. Removal path: the UI test
/// scenes (`scene ui-*`) once screens get their own synthetic scene.
#[derive(Resource)]
struct OpenAtStart(Option<String>);

#[allow(clippy::too_many_arguments)] // Bevy system params
fn open_at_start(
    mut commands: Commands,
    mut open: ResMut<OpenAtStart>,
    mut stack: ResMut<ScreenStackRes>,
    mut boxes: ResMut<MessageBoxes>,
    registry: Res<UiRegistryRes>,
    seen: Res<JournalSeenStore>,
    config: Option<Res<MessageBoxConfig>>,
    mut rpg: ResMut<Rpg>,
    sheets: DialogueSheets,
) {
    let Some(screen) = open.0.take() else { return };
    match screen.as_str() {
        "journal" => {
            let journal = JournalScreen::new(Alphabet::default(), 2, seen.0.clone());
            commands.spawn((UiScreenRoot("journal".into()), JournalUi::new(journal)));
            if let Err(e) = stack.0.push(&registry.0, "journal") {
                warn!("--open-ui journal: {e:?}");
            }
        }
        "message" => {
            let config = config.as_deref().cloned().unwrap_or_default();
            let text = "You have found Fargoth's ring. Return it to him?".to_owned();
            boxes.show(text, vec!["Yes".to_owned(), "No".to_owned()], &config);
        }
        "dialogue" => {
            let Some(speaker) = rpg.content.actors.keys().min().copied() else {
                open.0 = Some("dialogue".to_owned());
                return;
            };
            open_dialogue(&mut commands, &mut rpg, &mut stack, &registry, speaker, &sheets);
        }
        other => warn!("--open-ui: unknown screen `{other}` (journal, dialogue, message)"),
    }
}

/// The journal's seen state outlives its window (the next opening keeps the "new" markers
/// cleared).
fn keep_journal_seen(mut changes: MessageReader<JournalSeenChanged>, mut store: ResMut<JournalSeenStore>) {
    if let Some(change) = changes.read().last() {
        store.0 = change.seen.clone();
    }
}

/// Window requests → the session (action buttons → [`Rpg::route_action`]); a closed
/// conversation closes its window. The fixed tick (`StatTick`, 0 without the stats plugin)
/// seeds persuasion rolls.
fn host_dialogue(
    mut requests: MessageReader<DialogueRequestEvent>,
    mut rpg: ResMut<Rpg>,
    active: Option<ResMut<ActiveTalk>>,
    mut screens: Query<&mut DialogueScreen>,
    sheets: DialogueSheets,
    mut stack: ResMut<ScreenStackRes>,
    tick: Option<Res<StatTick>>,
) {
    let Some(mut active) = active else {
        requests.clear();
        return;
    };
    let tick = tick.map_or(0, |t| u64::from(t.0));
    let screen = active.screen;
    let Ok(mut window) = screens.get_mut(screen) else { return };
    for event in requests.read() {
        if event.screen == screen {
            rpg.request_at(&mut active.talk, &mut window.view, &event.request, &sheets, tick);
        }
    }
    if window.view.is_closed() {
        stack.0.close("dialogue");
    }
}

/// Menu screens taken off the stack (goodbye, Back, `J`) despawn; the conversation ends with
/// its window, settled as goodbye settles it ([`Rpg::end_talk`]; Back and `J` close it without
/// one).
fn close_screens(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    roots: Query<(Entity, &UiScreenRoot)>,
    mut active: Option<ResMut<ActiveTalk>>,
    mut rpg: ResMut<Rpg>,
    sheets: DialogueSheets,
) {
    for (e, root) in &roots {
        let id = root.0.as_str();
        if matches!(id, "dialogue" | "journal") && !stack.0.is_open(id) {
            commands.entity(e).despawn();
            if let Some(a) = active.as_deref_mut()
                && a.screen == e
            {
                rpg.end_talk(&mut a.talk, &sheets);
                commands.remove_resource::<ActiveTalk>();
            }
        }
    }
}

/// Result scripts' sheet changes go to `sf_stats_bevy` as commands, applied on its next fixed
/// tick (stats.md S7). Checked before taking, so `Rpg` isn't marked changed every frame.
fn send_stat_commands(mut rpg: ResMut<Rpg>, mut commands: Commands) {
    if rpg.stat_commands.is_empty() {
        return;
    }
    for command in std::mem::take(&mut rpg.stat_commands) {
        commands.write_message(command);
    }
}

/// Result-script message boxes: with buttons a modal box, without a timed toast (Morrowind's
/// button-less boxes time out). Sounds: TODO(audio) route `PlaySound` to `sf_audio_bevy`.
#[allow(clippy::needless_pass_by_value)]
fn show_events(
    mut rpg: ResMut<Rpg>,
    mut toasts: ResMut<Toasts>,
    mut boxes: ResMut<MessageBoxes>,
    config: Option<Res<MessageBoxConfig>>,
) {
    if !rpg.has_events() {
        return;
    }
    for event in rpg.take_events() {
        match event {
            ResultEffect::MessageBox { text, buttons } if buttons.is_empty() => {
                toasts.push(text);
            }
            ResultEffect::MessageBox { text, buttons } => {
                let config = config.as_deref().cloned().unwrap_or_default();
                boxes.show(text, buttons, &config);
            }
            ResultEffect::PlaySound(sound) => debug!("dialogue: sound {sound} (not routed yet)"),
            _ => {}
        }
    }
}

/// The free camera doesn't look around while a menu is open (the mouse is the pointer), nor while the player's body is
/// driven (`mw_player_bevy::apply_mode` turns it off then; turned back on here, it flew the camera on WASD and its right
/// button grab freed and showed the cursor on the release, owner 2026-10-27).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn free_camera_lock(
    stack: Res<ScreenStackRes>,
    mode: Option<Res<mw_game_core::player::PlayerMode>>,
    mut cameras: Query<&mut FreeCameraState>,
) {
    let driven = mode.is_some_and(|m| m.body.is_some());
    let enabled = !menu_open(&stack) && !driven;
    for mut state in &mut cameras {
        if state.enabled != enabled {
            state.enabled = enabled;
        }
    }
}

/// The player's controls pause while a menu is open (`PlayerMode::paused`, player.rs's seam).
/// Only on our own open/close edges, so another pauser isn't overridden every frame.
#[allow(clippy::needless_pass_by_value)]
fn pause_player(stack: Res<ScreenStackRes>, mode: Option<ResMut<mw_game_core::player::PlayerMode>>, mut was_open: Local<bool>) {
    let open = menu_open(&stack);
    if open == *was_open {
        return;
    }
    *was_open = open;
    if let Some(mut mode) = mode {
        mode.paused = open;
    }
}

// ============================================================ tests

#[cfg(test)]
mod tests {
    //! The Fargoth's ring flow through the game host and the dialogue view-model (real data;
    //! skips when no Morrowind install is detected), as `mw_rpg/tests/fargoth.rs` checks it.
    use super::*;
    use crate::stats::NewGameSheets;
    use mw_install::MorrowindInstall;
    use sf_rpg_ui::DialogueMode;

    const FARGOTH: ActorKey = ActorKey(1);
    const ARRILLE: ActorKey = ActorKey(2);

    fn last_text(view: &DialogueView) -> String {
        view.entries().rev().find_map(|e| match &e.kind { sf_rpg_ui::EntryKind::Line(l) => Some(l.text()), _ => None }).unwrap_or_default()
    }

    /// The journal screen's view-model over the game's [`JournalSource`] after the ring flow:
    /// the quest under its name, finished, both entries in the book, the ring topic indexed.
    fn journal_after_the_ring(rpg: &Rpg, quest: &RecordId, ring: &RecordId) {
        use sf_rpg_ui::{BookContent, FixedWidthMeasure, JournalCtx, LinePaginator, PageMetrics};
        let paginator = LinePaginator::new(FixedWidthMeasure { columns: 40 }, PageMetrics::default());
        let ctx = JournalCtx { world: JournalSource::world(rpg), labels: JournalSource::labels(rpg), paginator: &paginator };
        let mut screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::default());
        screen.open(&ctx, BookContent::Journal);
        let texts: Vec<_> = screen.blocks().iter().map(|b| b.text.clone()).collect();
        for entry in rpg.state.world.journal.entries() {
            assert!(texts.contains(&entry.text), "entry {:?} in the book: {texts:?}", entry.text);
        }
        assert!(screen.page_count() >= 1);
        assert!(screen.quest_list().is_empty(), "the active list hides the finished quest");
        screen.set_quest_filter(sf_rpg_ui::QuestFilter::Finished);
        assert_eq!(screen.quest_list(), std::slice::from_ref(quest));
        let row = screen.quest_model().get(quest).cloned().unwrap();
        assert_eq!(row.name, "Fargoth's Ring");
        assert!(row.finished, "{row:?}");
        assert_eq!((row.stage, row.entries), (100, 2));
        assert!(row.new, "unseen entries mark the quest new");
        assert!(screen.topic_list().contains(ring), "ring in the topic index");
        screen.open(&ctx, BookContent::Quest(quest.clone()));
        assert!(screen.blocks().iter().any(|b| b.text == "Fargoth's Ring"), "{:?}", screen.blocks());
        screen.open(&ctx, BookContent::Topic(ring.clone()));
        eprintln!("journal topic 'ring': {:?}", screen.blocks().iter().map(|b| &b.text).collect::<Vec<_>>());
        assert!(screen.blocks().len() > 1, "heard lines under the ring topic");
    }

    #[test]
    fn fargoths_ring_through_the_window() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let content = mw_records::load(&vfs, &install.game_files()).unwrap();
        let db = Arc::new(content.db);
        let sheets = NewGameSheets::from_db(db.clone()).unwrap();
        let mut rpg = Rpg::new(db).unwrap();
        assert_eq!(rpg.exterior_name((-2, -9)), Some("Seyda Neen"));
        assert!(rpg.add_actor(FARGOTH, &id("fargoth"), "Seyda Neen"));
        assert!(rpg.add_actor(ARRILLE, &id("arrille"), "Seyda Neen"));
        let (mut talk, mut view) = rpg.open(FARGOTH, &sheets).unwrap();
        assert_eq!(view.speaker(), "Fargoth");
        eprintln!("greeting: {}", last_text(&view));
        let ring = id("ring");
        assert!(rpg.state.world.topics.knows(&ring));
        assert!(view.topics().iter().any(|(k, _)| *k == ring), "ring in the topic list");

        let quest = id("ms_fargothring");
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(ring.clone()), &sheets);
        assert_eq!(rpg.state.world.quests.stage(&quest), 10);
        assert_eq!(view.choices().len(), 2, "{:?}", view.choices());
        assert_eq!(view.mode(), DialogueMode::Choosing);

        // "Yes, here" without the ring.
        let yes = view.choices()[1].value;
        rpg.request(&mut talk, &mut view, &DialogueRequest::Choose(yes), &sheets);
        assert_eq!(rpg.state.npcs.modifier(FARGOTH), -5);

        rpg.state.pc.loadout.add(id(RING), 1, Attrs::new()).unwrap();
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(ring.clone()), &sheets);
        let yes = view.choices()[1].value;
        rpg.request(&mut talk, &mut view, &DialogueRequest::Choose(yes), &sheets);
        assert_eq!(rpg.state.world.quests.stage(&quest), 100);
        assert_eq!(rpg.state.pc.loadout.count(&id(RING)), 0);
        assert_eq!(rpg.state.inventories[&FARGOTH].loadout.count(&id(RING)), 1);
        assert_eq!(rpg.state.npcs.modifier(FARGOTH), 45);
        assert_eq!(rpg.state.npcs.modifier(ARRILLE), 40);
        let entries = rpg.state.world.journal.entries();
        assert_eq!(entries.iter().map(|e| e.stage).collect::<Vec<_>>(), [Some(10), Some(100)]);
        assert!(entries.iter().all(|e| !e.text.contains('%')), "{entries:?}");
        assert_eq!(rpg.content.quest_name(&quest).as_deref(), Some("Fargoth's Ring"));
        // The journal's topic index has what Fargoth said about his ring.
        assert!(rpg.state.world.topics.knows(&ring));
        journal_after_the_ring(&rpg, &quest, &ring);

        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(ring), &sheets);
        eprintln!("afterwards: {}", last_text(&view));
        rpg.request(&mut talk, &mut view, &DialogueRequest::Goodbye, &sheets);
        assert!(view.is_closed());
        let bytes = postcard::to_stdvec(&rpg.state).unwrap();
        let back: RpgState = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(back, rpg.state);
    }

    /// Owner report 2026-10-25: the Census and Excise Office NPCs showed no topics. The basic
    /// topics come from the end of chargen (Sellus Gravius's "duties" result), which a new game
    /// without chargen never ran; `skip_chargen` grants them from that data.
    #[test]
    fn census_topics_after_skipped_chargen() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let content = mw_records::load(&vfs, &install.game_files()).unwrap();
        let db = Arc::new(content.db);
        let sheets = NewGameSheets::from_db(db.clone()).unwrap();
        let cell = "Seyda Neen, Census and Excise Office";
        let topics_of = |rpg: &mut Rpg, base: &str| -> Vec<RecordId> {
            let key = ActorKey(7);
            assert!(rpg.add_actor(key, &id(base), cell));
            let (_, view) = rpg.open(key, &sheets).unwrap();
            let topics: Vec<RecordId> = view.topics().iter().map(|(k, _)| k.clone()).collect();
            eprintln!("{base}: {:?} {topics:?}", last_text(&view));
            topics
        };
        let has = |topics: &[RecordId], names: &[&str]| names.iter().all(|n| topics.contains(&id(n)));
        let basics = ["latest rumors", "little advice", "little secret"];

        // The report: before the grant the door guard lists only what his greeting named.
        let mut before = Rpg::new(db.clone()).unwrap();
        assert!(!has(&topics_of(&mut before, "chargen door guard"), &basics[..1]));

        let mut rpg = Rpg::new(db).unwrap();
        let granted = rpg.skip_chargen();
        eprintln!("granted: {granted:?}");
        assert_eq!(rpg.state.globals.get(&id("chargenstate")), Some(&-1.0));
        assert!(!rpg.content.actors.contains_key(&ActorKey(u128::MAX)), "the hand-off speaker is not left registered");
        for t in basics.iter().chain(&["my trade", "specific place", "someone in particular", "caius cosades"]) {
            assert!(granted.contains(&id(t)) && rpg.state.world.topics.knows(&id(t)), "{t} granted");
        }
        // The guard (greeting names "Imperial Legion"), Sellus Gravius, then Socucius Ergalla,
        // whose only topics in the data (imperial legion, law in morrowind, morrowind,
        // solstheim) aren't chargen grants: he lists the one the guard taught.
        assert!(has(&topics_of(&mut rpg, "chargen door guard"), &basics), "guard");
        assert!(has(&topics_of(&mut rpg, "chargen captain"), &["my trade", "caius cosades"]), "Sellus Gravius");
        assert!(has(&topics_of(&mut rpg, "chargen class"), &["imperial legion"]), "Socucius Ergalla");
    }

    /// Owner report 2026-10-25: after a skipped chargen Sellus Gravius greeted "Who are you?
    /// Where are your papers?". That greeting needs no `chargen statssheet` on the player and
    /// his local `state` != -1; chargen's papers hand-in sets it to -1. `skip_chargen` leaves
    /// that local, so he gives his post-chargen greeting.
    #[test]
    fn sellus_greets_after_skipped_chargen() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let content = mw_records::load(&vfs, &install.game_files()).unwrap();
        let db = Arc::new(content.db);
        let sheets = NewGameSheets::from_db(db.clone()).unwrap();
        let cell = "Seyda Neen, Census and Excise Office";
        let (sellus, state) = (ActorKey(7), id("state"));
        let greeting = |rpg: &mut Rpg| {
            assert!(rpg.add_actor(sellus, &id("chargen captain"), cell));
            let (_, view) = rpg.open(sellus, &sheets).unwrap();
            last_text(&view)
        };
        let papers = "Where are your papers?";

        // The report, reproduced on a game that never ran the hand-off.
        let mut before = Rpg::new(db.clone()).unwrap();
        assert!(greeting(&mut before).contains(papers));

        let mut rpg = Rpg::new(db).unwrap();
        rpg.skip_chargen();
        let text = greeting(&mut rpg);
        eprintln!("Sellus Gravius: {text}");
        assert!(!text.contains(papers), "{text}");
        assert!(text.contains("Sellus Gravius"), "the post-chargen greeting names him: {text}");
        assert_eq!(rpg.state.locals.get(&(sellus, state.clone())), Some(&-1.0));
        assert_eq!(rpg.state.pc.loadout.count(&id("chargen statssheet")), 0, "no papers granted");
        // A value he already has (a later `set state`, a loaded save) is kept on re-registering.
        rpg.state.locals.insert((sellus, state.clone()), 3.0);
        assert!(rpg.add_actor(sellus, &id("chargen captain"), cell));
        assert_eq!(rpg.state.locals.get(&(sellus, state)), Some(&3.0));
    }
}
