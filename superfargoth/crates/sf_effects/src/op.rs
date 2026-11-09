//! Operations, events and the step's context and output (effects.md "Operations", "Events").

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_stats::{Cause, StatOp, StatOwner};
use smallvec::SmallVec;

use crate::active::{ActorRef, EffectIdAlloc, InstanceFlags, InstanceId, SourceClass, SourceRef};
use crate::book::{BookRejection, GameMinutes};
use crate::def::{ArgRef, CureFilter, EffectKey, Tag};
use crate::rules::CastFailReason;

/// How an entry is delivered. E1 lands every entry on the op's target; delivery filters come
/// with casting (E3) and area landing (E5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Range {
    OnSelf,
    Touch,
    Target,
}

/// One effect of a source as content lists it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: EffectKey,
    pub arg: Option<ArgRef>,
    pub range: Range,
    /// Metres.
    pub area: f32,
    /// Seconds; 0 = instant.
    pub duration: f32,
    /// `(min, max)`.
    pub magnitude: (f32, f32),
}

pub type EffectList = SmallVec<[EffectEntry; 4]>;

/// The host's seed for one landing's rolls (a hash of what the host chooses: tick, caster,
/// salt). Rolls are drawn from it with [`crate::roll`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RollSeed(pub u64);

/// Why an effect ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EndReason {
    Expired,
    /// A new landing replaced it (`StackPolicy::Replace`).
    Replaced,
    Dispelled,
    Cured,
    SourceRemoved,
    Unequipped,
    CasterGone,
    Invalid,
    Cleared,
}

/// One change to an [`crate::ActiveSet`] (and, from E3, its owner's [`crate::SpellBook`]).
/// E1: landing, ending and skipping time; E3: casting, the book, cures and request answers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant, reason = "short-lived per-tick op lists; boxing would change the public op shape")]
pub enum EffectOp {
    /// Lands `entries` from `source` on the target (scripts, potions, and later casts).
    Inflict {
        source: SourceRef,
        class: SourceClass,
        caster: Option<ActorRef>,
        entries: EffectList,
        flags: InstanceFlags,
        rolls: RollSeed,
    },
    /// Ends an instance; an unknown instance is a no-op.
    End { instance: InstanceId, reason: EndReason },
    /// Ends every instance of a source record (`RemoveSpell`, unequip).
    EndBySource { record: RecordId },
    /// Rest, wait, travel: timed effects advance in one closed-form step.
    Skip { seconds: f32 },
    /// The target casts `source` from its book (E3, effects.md "Casting"): book check, the cast
    /// model's plan, the cost, the chance roll, then delivery. Self entries land on the caster;
    /// touch entries go to `target` and target entries to the projectile owner, both as
    /// [`EffectRequest`]s. Scripts use `Inflict` instead (no book, chance or cost).
    Cast { source: RecordId, target: Option<ActorRef>, rolls: RollSeed },
    /// Adds a source to the target's book; an innate or affliction source starts its instance
    /// (resistances ignored). Known already: a no-op.
    Learn { source: RecordId },
    /// Removes a source from the book; an innate or affliction source ends its instance
    /// (`SourceRemoved`). Unknown: a no-op.
    Forget { source: RecordId },
    /// The book's selected spell or power; `None` clears it.
    Select { source: Option<RecordId> },
    /// Ends what `filter` matches on the target (script cures; a `Cure` effect does the same
    /// when it lands). The op cures with magnitude 100 (a dispel always succeeds).
    Cure { filter: CureFilter, rolls: RollSeed },
    /// A request handler's answer (effects.md "Request"): the effect cannot act, so it ends as
    /// `Invalid` and no `End` request follows. Unknown instance or effect: a no-op.
    Invalidate { instance: InstanceId, index: u8 },
}

/// The request phase of a [`crate::Archetype::Request`] effect. `Tick` comes with periodic
/// effects (the def's `period`, not yet built).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequestPhase {
    Start,
    End,
}

/// Entries of a cast that land on another actor (touch) or on whatever a projectile hits
/// (target). The receiver runs [`Delivery::inflict`] on the actor it lands on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    pub source: SourceRef,
    pub class: SourceClass,
    pub caster: ActorRef,
    pub entries: EffectList,
    /// The cast's seed: the same cast lands the same rolls.
    pub rolls: RollSeed,
}

impl Delivery {
    /// The op that lands these entries on one target.
    pub fn inflict(&self) -> EffectOp {
        EffectOp::Inflict {
            source: self.source.clone(),
            class: self.class,
            caster: Some(self.caster),
            entries: self.entries.clone(),
            flags: InstanceFlags::default(),
            rolls: self.rolls,
        }
    }
}

/// Work the effect step hands to game code (plain data). Answers come back as ops next tick;
/// handlers never write the active set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EffectRequest {
    /// A `Request` effect started or ended on `target`.
    Effect {
        kind: Tag,
        phase: RequestPhase,
        target: ActorRef,
        caster: Option<ActorRef>,
        instance: InstanceId,
        /// The effect's index in its instance (for `Invalidate`).
        index: u8,
        effect: EffectKey,
        magnitude: f32,
        arg: Option<ArgRef>,
        tick: u32,
    },
    /// Touch entries of a cast for the caster's contact (`Cast { target }`).
    Touch { target: ActorRef, delivery: Delivery },
    /// Target entries of a cast for the projectile owner, which lands them on impact.
    Launch(Delivery),
}

/// Why an entry was not landed (reported, never silent).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rejection {
    UnknownEffect,
    /// The archetype's stat or the entry's argument is not in the stat table (or not of the
    /// def's argument group).
    UnknownStat,
    /// A `caster_linked` effect without a caster.
    NoCaster,
}

/// What the effect step tells the world (plain data, net rule 8).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EffectEvent {
    Applied { target: ActorRef, instance: InstanceId, effect: EffectKey, magnitude: f32, caster: Option<ActorRef>, tick: u32 },
    Resisted { target: ActorRef, effect: EffectKey, caster: Option<ActorRef>, tick: u32 },
    Ended { target: ActorRef, instance: InstanceId, effect: EffectKey, reason: EndReason, tick: u32 },
    /// A `non_recastable` landing while its source is active.
    CastRefused { target: ActorRef, source: RecordId, caster: Option<ActorRef>, tick: u32 },
    Rejected { target: ActorRef, effect: EffectKey, reason: Rejection, tick: u32 },
    /// A `Cast` went off (cost spent, entries delivered).
    CastSucceeded { caster: ActorRef, source: RecordId, tick: u32 },
    /// A `Cast` did not: refused by the book or the cast model (nothing spent) or `Fizzle`
    /// (cost spent).
    CastFailed { caster: ActorRef, source: RecordId, reason: CastFailReason, tick: u32 },
    /// A power with a recharge was used; its recharge runs from `time`.
    PowerUsed { caster: ActorRef, source: RecordId, time: GameMinutes, tick: u32 },
    /// A source entered (`added`) or left the book.
    BookChanged { owner: ActorRef, source: RecordId, added: bool, tick: u32 },
    /// A book op that could not run.
    BookRejected { owner: ActorRef, source: RecordId, reason: BookRejection, tick: u32 },
}

/// What one step produced: stat ops for the host to run through `sf_stats::apply` in the same
/// tick, in order, each on its owner's sheet with its cause; and events.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EffectOut {
    pub stat_ops: Vec<(StatOwner, StatOp, Cause)>,
    pub events: Vec<EffectEvent>,
    /// Work for game code: `Request` effects, touch and target deliveries (E3).
    pub requests: Vec<EffectRequest>,
}

impl EffectOut {
    pub fn clear(&mut self) {
        self.stat_ops.clear();
        self.events.clear();
        self.requests.clear();
    }
}

/// The context of one step on one target.
#[derive(Debug)]
pub struct EffectCtx<'a> {
    pub target: ActorRef,
    /// The fixed tick.
    pub tick: u32,
    /// Tick length in seconds (durations become `round(seconds / dt)` ticks).
    pub dt: f32,
    pub ids: &'a mut EffectIdAlloc,
}
