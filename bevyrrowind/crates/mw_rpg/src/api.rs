//! The game API (scripting-adapter.md "The game API (`mw_rpg::api`)"): what an `MWScript` run may
//! read and ask for, independent of who runs it. Script hosts (the results host, `runner.rs`; the
//! script host, DQ-4b1b) only bind `mw_script::Host` calls onto it.
//!
//! - [`GameView`] answers reads: `sf_rpg` state (quests, the context player's inventory) and,
//!   through [`MwWorld`], what `mw_rpg` cannot own (globals, script variables, items held by world
//!   objects, sheets).
//! - A run ([`GameRun`]) mutates nothing: it produces [`GameOp`]s in call order, serde data the
//!   authoritative host applies on its tick (net rules 2, 5), so a run is replayable.
//! - Its [`Overlay`] makes later reads in the same run see the run's own writes, as in the
//!   original engine where every call takes effect at once.
//! - Morrowind's rules are pure functions ([`journal_stage`], [`add_count`], [`remove_count`],
//!   [`disposition`], [`stored_value`]); stat, faction and crime rules stay in `script_stats`.
//!
//! Families covered (the spec table's "built for results"): variables, journal and topics, items,
//! disposition, `Choice`/`Goodbye`, `MessageBox`/`PlaySound` events, stats.

use std::collections::HashMap;

use mw_script::StatCall;
use mw_script::ast::{Ident, Target as AstTarget, VarRef, VarType};
use mw_script::interp::store;
use mw_ui::message::{MessageArg, message_box_from_args};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::{ActorKey, CharacterRpg, Choice, ConversationCtx, WorldRpg};
use sf_stats::StatOp;

use crate::filter::ActorFacts;
use crate::script_stats::StatOverlay;
use crate::stats::{MwSheets, NoSheets};
use crate::text::GlobalNumber;

/// Whose state an op targets: `Speaker` is the spec's `SelfRef` (the running instance: the
/// speaker for results, the placement for local scripts), `Player` the context player
/// (`Player->`), `Actor` the spec's `Id(record)` (`"arrille"->`: an object by base id, which the
/// host resolves to a live reference).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Target {
    Speaker,
    /// The context player (`ctx.player`).
    Player,
    /// An object by base id; the host resolves it (to an `ActorKey` for actors).
    Actor(RecordId),
    /// One exact reference by its `PlacementId` (`sf_world::PlacementId.0`): a local script's
    /// self (DQ-5b), never resolved by base id, so other references of the same base are not
    /// touched.
    Ref(u128),
    /// One stack in an inventory (`mw_script::LocalKey::Stack`: the holder's id and the
    /// `StackId`): the self of a carried item's local script (DQ-5b3). Only its locals are
    /// addressable; it holds no items, sheet or disposition, so those ops do nothing.
    Stack { holder: u128, stack: u64 },
}

/// What a run asked for, in call order (also exported as `ResultEffect`). The host applies
/// these on its tick: journal through
/// [`MwDialogue::journal_command`](crate::MwDialogue::journal_command), items as `sf_rpg`
/// commands, choices and goodbye on the session, dispositions in
/// [`NpcStates`](crate::NpcStates), locals and globals in the script runtime, message boxes to
/// the UI, sounds to audio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameOp {
    Journal {
        quest: RecordId,
        index: i32,
    },
    SetJournalIndex {
        quest: RecordId,
        index: i32,
    },
    AddTopic(RecordId),
    AddItem {
        target: Target,
        item: RecordId,
        count: u32,
    },
    RemoveItem {
        target: Target,
        item: RecordId,
        count: u32,
    },
    Choices(Vec<Choice>),
    Goodbye,
    ModDisposition {
        target: Target,
        delta: i32,
    },
    SetDisposition {
        target: Target,
        value: i32,
    },
    /// A script local of the speaker or another object, already converted to its type.
    SetLocal {
        target: Target,
        name: RecordId,
        value: f32,
    },
    /// A global (GLOB), already converted to its type.
    SetGlobal {
        name: RecordId,
        value: f32,
    },
    /// `MessageBox`: formatted text; no buttons = a notification, else a modal.
    MessageBox {
        text: String,
        buttons: Vec<String>,
    },
    /// `PlaySound`: a sound record id (2D, at the listener).
    PlaySound(RecordId),
    /// A stat, faction or crime function's change to `target`'s sheet (`Target::Player`: the
    /// context player), as `sf_stats` ops in order; the host sends them as one stats command
    /// (stats.md S7, `script_stats.rs`).
    Stats {
        target: Target,
        ops: Vec<StatOp>,
    },
}

/// A script variable and its declared type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptVar {
    pub ty: VarType,
    pub value: f32,
}

/// World-side reads the API needs and `mw_rpg` does not own. The host implements it over its
/// world (ECS, placement overlay); tests use doubles.
pub trait MwWorld {
    /// A global (GLOB); `None`: no such global (reads 0, writes are dropped).
    fn global(&self, name: &RecordId) -> Option<GlobalNumber>;
    /// A script variable of `target` (the speaker's locals, `id.var`, `id->` objects);
    /// `None`: no script or no such variable (reads 0, writes are dropped).
    fn var(&self, ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar>;
    /// Items held by `target` when it is not the context player (the speaker, `id->`
    /// actors and containers); the player's come from its `CharacterRpg`.
    fn item_count(&self, ctx: &ConversationCtx, target: &Target, item: &RecordId) -> u64;

    /// The character sheets the stat functions read and write (stats.md S7). Default: none,
    /// so every stat read is neutral and writes are dropped.
    fn sheets(&self) -> &dyn MwSheets {
        &NoSheets
    }

    /// The actor `target` names (`None`: the player, or no such actor). Default: the speaker.
    fn actor(&self, ctx: &ConversationCtx, target: &Target) -> Option<ActorKey> {
        (*target == Target::Speaker).then_some(ctx.speaker)
    }

    /// What the records say about the actor `target` names (faction and record rank, NPC or
    /// creature); `None` for the player. Default: nothing known.
    fn facts(&self, ctx: &ConversationCtx, target: &Target) -> Option<&ActorFacts> {
        let _ = (ctx, target);
        None
    }
}

/// The game as a run sees it at its start: the run's context (`ctx.speaker` is
/// [`Target::Speaker`], `ctx.player` the context player), `sf_rpg` world state, the context
/// player's character and the host's world.
#[derive(Clone, Copy)]
pub struct GameView<'a> {
    pub ctx: &'a ConversationCtx,
    pub world: &'a WorldRpg,
    /// The character of `ctx.player`.
    pub player: &'a CharacterRpg,
    pub env: &'a dyn MwWorld,
}

impl GameView<'_> {
    /// A quest's current stage (0: not started).
    pub fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.world.quests.stage(quest)
    }

    /// A global's value and width; `None`: no such global.
    pub fn global(&self, name: &RecordId) -> Option<GlobalNumber> {
        self.env.global(name)
    }

    /// A script variable of `target`; `None`: no script or no such variable.
    pub fn var(&self, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        self.env.var(self.ctx, target, name)
    }

    /// How many `item`s `target` holds: the context player's from its loadout, others' from
    /// the world.
    pub fn item_count(&self, target: &Target, item: &RecordId) -> u64 {
        match target {
            Target::Player => self.player.loadout.count(item),
            other => self.env.item_count(self.ctx, other, item),
        }
    }
}

/// A run's writes, read back by its later reads (dropped with the run; the ops carry them).
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    stages: HashMap<RecordId, i32>,
    /// Item deltas per holder.
    items: HashMap<(Target, RecordId), i64>,
    vars: HashMap<(Target, RecordId), f32>,
    globals: HashMap<RecordId, f32>,
    /// The run's copies of the sheets it changed.
    stats: StatOverlay,
}

/// A point a [`GameRun`] can return to ([`GameRun::rollback`]): the script host drops a faulted
/// script's writes while keeping the earlier scripts' of the same tick.
#[derive(Debug)]
pub struct Checkpoint {
    overlay: Overlay,
    ops: usize,
}

/// One run: reads through the [`Overlay`] of its own writes over a [`GameView`], ops in call
/// order. Methods that write and can fail say so (`false`: unknown variable, dropped).
pub struct GameRun<'a> {
    view: GameView<'a>,
    overlay: Overlay,
    ops: Vec<GameOp>,
}

impl<'a> GameRun<'a> {
    pub fn new(view: GameView<'a>) -> Self {
        Self {
            view,
            overlay: Overlay::default(),
            ops: Vec::new(),
        }
    }

    /// The view the run started from.
    pub fn view(&self) -> GameView<'a> {
        self.view
    }

    /// The ops so far, in call order.
    pub fn ops(&self) -> &[GameOp] {
        &self.ops
    }

    /// Ends the run: its ops, in call order.
    pub fn into_ops(self) -> Vec<GameOp> {
        self.ops
    }

    /// The run as it is now (copies the overlay: it holds only this run's writes).
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            overlay: self.overlay.clone(),
            ops: self.ops.len(),
        }
    }

    /// Back to `checkpoint`: the writes and ops since are dropped.
    pub fn rollback(&mut self, checkpoint: Checkpoint) {
        self.overlay = checkpoint.overlay;
        self.ops.truncate(checkpoint.ops);
    }

    // Variables.

    /// A global as this run sees it; `None`: no such global.
    pub fn global(&self, name: &RecordId) -> Option<f32> {
        self.overlay
            .globals
            .get(name)
            .copied()
            .or_else(|| self.view.global(name).map(GlobalNumber::as_f32))
    }

    /// Writes a global, converted to its width ([`stored_value`]); `false`: no such global.
    pub fn set_global(&mut self, name: &RecordId, value: f64) -> bool {
        let Some(global) = self.view.global(name) else {
            return false;
        };
        let value = stored_value(global_type(global), value);
        self.overlay.globals.insert(name.clone(), value);
        self.ops.push(GameOp::SetGlobal {
            name: name.clone(),
            value,
        });
        true
    }

    /// A script variable of `target` as this run sees it; `None`: unknown.
    pub fn var(&self, target: &Target, name: &RecordId) -> Option<f32> {
        self.overlay
            .vars
            .get(&(target.clone(), name.clone()))
            .copied()
            .or_else(|| self.view.var(target, name).map(|v| v.value))
    }

    /// Writes a script variable of `target`, converted to its type; `false`: unknown.
    pub fn set_var(&mut self, target: &Target, name: &RecordId, value: f64) -> bool {
        let Some(var) = self.view.var(target, name) else {
            return false;
        };
        let value = stored_value(var.ty, value);
        self.overlay
            .vars
            .insert((target.clone(), name.clone()), value);
        self.ops.push(GameOp::SetLocal {
            target: target.clone(),
            name: name.clone(),
            value,
        });
        true
    }

    // Journal and topics.

    /// The quest stage this run sees.
    pub fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.overlay
            .stages
            .get(quest)
            .copied()
            .unwrap_or_else(|| self.view.quest_stage(quest))
    }

    /// `Journal`: raises the stage, never lowers it ([`journal_stage`]).
    pub fn journal(&mut self, quest: &RecordId, index: i32) {
        let stage = journal_stage(self.quest_stage(quest), index);
        self.overlay.stages.insert(quest.clone(), stage);
        self.ops.push(GameOp::Journal {
            quest: quest.clone(),
            index,
        });
    }

    /// `SetJournalIndex`: sets the stage, lower or higher.
    pub fn set_journal_index(&mut self, quest: &RecordId, index: i32) {
        self.overlay.stages.insert(quest.clone(), index);
        self.ops.push(GameOp::SetJournalIndex {
            quest: quest.clone(),
            index,
        });
    }

    pub fn add_topic(&mut self, topic: RecordId) {
        self.ops.push(GameOp::AddTopic(topic));
    }

    // Items.

    /// How many `item`s `target` holds as this run sees it.
    pub fn item_count(&self, target: &Target, item: &RecordId) -> i64 {
        let delta = self
            .overlay
            .items
            .get(&(target.clone(), item.clone()))
            .copied()
            .unwrap_or(0);
        i64::try_from(self.view.item_count(target, item))
            .unwrap_or(i64::MAX)
            .saturating_add(delta)
    }

    /// `AddItem`: nothing for a count below 1 or an empty id ([`add_count`]).
    pub fn add_item(&mut self, target: Target, item: RecordId, count: i32) {
        let Some(count) = add_count(count) else {
            return;
        };
        if item.as_str().is_empty() {
            return;
        }
        self.shift_items(&target, &item, i64::from(count));
        self.ops.push(GameOp::AddItem {
            target,
            item,
            count,
        });
    }

    /// `RemoveItem`: at most what `target` holds ([`remove_count`]); nothing when that is 0.
    pub fn remove_item(&mut self, target: Target, item: RecordId, count: i32) {
        let Some(count) = remove_count(self.item_count(&target, &item), count) else {
            return;
        };
        self.shift_items(&target, &item, -i64::from(count));
        self.ops.push(GameOp::RemoveItem {
            target,
            item,
            count,
        });
    }

    fn shift_items(&mut self, target: &Target, item: &RecordId, delta: i64) {
        *self
            .overlay
            .items
            .entry((target.clone(), item.clone()))
            .or_default() += delta;
    }

    // Disposition (applied and clamped by the host, `NpcStates`, [`disposition`]).

    pub fn set_disposition(&mut self, target: Target, value: i32) {
        self.ops.push(GameOp::SetDisposition { target, value });
    }

    pub fn mod_disposition(&mut self, target: Target, delta: i32) {
        self.ops.push(GameOp::ModDisposition { target, delta });
    }

    // Dialogue session.

    pub fn choice(&mut self, choices: Vec<Choice>) {
        self.ops.push(GameOp::Choices(choices));
    }

    pub fn goodbye(&mut self) {
        self.ops.push(GameOp::Goodbye);
    }

    // Events.

    /// Formats as `OpenMW` (`mw_ui::message::message_box_from_args`: values first, then as many
    /// buttons as there are quoted strings among the rest, at most 10). Text defines
    /// (`^PCName`) are left for the UI, as `OpenMW` expands them after formatting.
    pub fn message_box(&mut self, format: &str, args: &[MessageArg]) {
        let message = message_box_from_args(format, args);
        self.ops.push(GameOp::MessageBox {
            text: message.text,
            buttons: message.buttons,
        });
    }

    pub fn play_sound(&mut self, sound: RecordId) {
        self.ops.push(GameOp::PlaySound(sound));
    }

    // Stats, factions, crime (`script_stats`).

    /// Runs a stat function on `target`: what it reads (0 for writes); a write appends its
    /// [`GameOp::Stats`] and later reads in the run see it.
    pub fn stat(&mut self, target: &Target, call: &StatCall) -> f64 {
        self.overlay
            .stats
            .call(self.view.ctx, self.view.env, target, call, &mut self.ops)
    }
}

// Morrowind rules (pure).

/// `Journal`'s stage after writing `index` at `current`: it raises, never lowers.
pub fn journal_stage(current: i32, index: i32) -> i32 {
    current.max(index)
}

/// `AddItem`'s count: `None` (nothing happens) below 1.
pub fn add_count(asked: i32) -> Option<u32> {
    u32::try_from(asked).ok().filter(|&n| n >= 1)
}

/// `RemoveItem`'s count: at most what is `held`; `None` (nothing happens) when that is below 1.
pub fn remove_count(held: i64, asked: i32) -> Option<u32> {
    u32::try_from(i64::from(asked).min(held))
        .ok()
        .filter(|&n| n >= 1)
}

/// The disposition filters and scripts see: base plus the scripted modifier, clamped to 0..=100.
pub fn disposition(base: i32, modifier: i32) -> i32 {
    base.saturating_add(modifier).clamp(0, 100)
}

/// A value written to a variable of type `ty`, as the variable keeps it (`mw_script`'s `store`:
/// integer widths wrap, floats narrow).
#[allow(clippy::cast_possible_truncation)]
pub fn stored_value(ty: VarType, value: f64) -> f32 {
    store(ty, value) as f32
}

/// The variable type a global resolves as: integers as `long` (`GlobalNumber` keeps no width).
pub fn global_type(global: GlobalNumber) -> VarType {
    match global {
        GlobalNumber::Int(_) => VarType::Long,
        GlobalNumber::Float(_) => VarType::Float,
    }
}

// Bindings from `mw_script`'s AST (shared by the hosts).

pub(crate) fn rid(id: &Ident) -> RecordId {
    RecordId::new(&id.key)
}

/// The target a call names (implicit = the running instance).
pub(crate) fn target(t: &AstTarget) -> Target {
    match t {
        AstTarget::Implicit => Target::Speaker,
        AstTarget::Explicit(id) => object(id),
    }
}

/// The target an object name names (`player` = the context player).
pub(crate) fn object(id: &Ident) -> Target {
    if id.key == "player" {
        Target::Player
    } else {
        Target::Actor(rid(id))
    }
}

/// Where a variable lives: `(None, name)` a global, `(Some(target), name)` a script variable of
/// `target`; `None` for what the interpreter keeps itself.
pub(crate) fn owner(var: &VarRef) -> Option<(Option<Target>, &Ident)> {
    match var {
        VarRef::Global(name) => Some((None, name)),
        VarRef::Speaker(name) | VarRef::Name(name) => Some((Some(Target::Speaker), name)),
        VarRef::Member { object: o, var } => Some((Some(object(o)), var)),
        VarRef::Local(_) | VarRef::Undeclared(_) => None,
    }
}

/// How an unknown variable is reported (`global x`, `speaker.x`, `id.x`).
pub(crate) fn unknown_name(owner: Option<&Target>, name: &Ident) -> String {
    match owner {
        None => format!("global {}", name.key),
        Some(Target::Speaker) => format!("speaker.{}", name.key),
        Some(Target::Player) => format!("player.{}", name.key),
        Some(Target::Actor(a)) => format!("{a}.{}", name.key),
        Some(Target::Ref(p)) => format!("ref {p:032x}.{}", name.key),
        Some(Target::Stack { holder, stack }) => format!("stack {holder:032x}/{stack}.{}", name.key),
    }
}
