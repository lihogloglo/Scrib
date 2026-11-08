//! Active state per target (effects.md "Active state per target"): sources in insertion order,
//! each a copy of its rolled effects, keyed by host-allocated [`InstanceId`]s (net rule 7).

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_stats::{Layer, SourceId, SourceKind, StatOwner};
use smallvec::SmallVec;

use crate::def::{ArgRef, EffectKey, StackPolicy};

/// Who cast or carries an effect (a player or an actor; never an `Entity`, net rule 6).
pub type ActorRef = StatOwner;

/// Most effects in one source: the effect index fills the low 6 bits of its [`SourceId`].
pub const MAX_EFFECTS_PER_SOURCE: usize = 64;

/// One landed source on one target: host-allocated, saved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InstanceId(pub u64);

impl InstanceId {
    /// The stats source of effect `index` of this instance: `(instance << 6) | index`, so one
    /// effect can end before its siblings and `RemoveSource` is exact.
    pub fn source_id(self, index: u8) -> SourceId {
        SourceId { kind: SourceKind::Effect, id: (self.0 << 6) | u64::from(index & 63) }
    }
}

/// The host's instance counter (world save meta).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectIdAlloc {
    pub next: u64,
}

impl Default for EffectIdAlloc {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl EffectIdAlloc {
    pub fn alloc(&mut self) -> InstanceId {
        let id = InstanceId(self.next);
        self.next += 1;
        id
    }
}

/// A stable item instance id (host-allocated). Placeholder until items land (effects E5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ItemKey(pub u64);

/// What a landing came from: a source record, and the item that carried it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRef {
    pub record: RecordId,
    pub item: Option<ItemKey>,
}

/// The kind of source (effects.md source-class table): lifetime, layer and default stacking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceClass {
    Cast,
    Power,
    /// Abilities, birthsigns, racial bonuses: while in the book, on the permanent layer.
    Innate,
    /// Diseases, blights, curses: while in the book.
    Affliction,
    Consumable,
    ItemCast,
    /// While the item is equipped.
    ItemConstant,
}

/// Which earlier landings count as "the same source" for stacking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StackKey {
    BySourceCaster,
    BySourceCasterItem,
    BySource,
}

impl SourceClass {
    /// Effects live as long as the source (no duration).
    pub fn lives_with_source(self) -> bool {
        matches!(self, Self::Innate | Self::Affliction | Self::ItemConstant)
    }

    /// The layer of `LayerRule::BySource` modifiers.
    pub fn layer(self) -> Layer {
        if self == Self::Innate { Layer::Permanent } else { Layer::Temporary }
    }

    /// Stacking when the def names none.
    pub fn default_stack(self) -> StackPolicy {
        match self {
            Self::Cast | Self::Power | Self::ItemCast => StackPolicy::Replace,
            Self::Consumable => StackPolicy::Stack { limit: u16::MAX },
            Self::Innate | Self::Affliction | Self::ItemConstant => StackPolicy::Ignore,
        }
    }

    pub fn stack_key(self) -> StackKey {
        match self {
            Self::Cast | Self::Power | Self::Consumable => StackKey::BySourceCaster,
            Self::ItemCast | Self::ItemConstant => StackKey::BySourceCasterItem,
            Self::Innate | Self::Affliction => StackKey::BySource,
        }
    }
}

/// Per-landing switches.
#[allow(clippy::struct_excessive_bools, reason = "independent landing flags")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceFlags {
    pub ignore_resist: bool,
    pub ignore_reflect: bool,
    pub ignore_absorb: bool,
    pub from_reflect: bool,
}

/// An effect's state. E1 uses `Running` and `Ended`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Pending,
    Running,
    Invalid,
    Ended,
}

impl Phase {
    /// Still holds stats or waits to.
    pub fn is_live(self) -> bool {
        matches!(self, Self::Pending | Self::Running)
    }
}

/// One rolled effect of a source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveEffect {
    /// The entry's index in its source (the low bits of its [`SourceId`]).
    pub index: u8,
    pub effect: EffectKey,
    pub arg: Option<ArgRef>,
    /// Rolled once at the landing, after resist; never re-rolled.
    pub magnitude: f32,
    /// Ticks; `None` lives as long as the source.
    pub duration: Option<u32>,
    pub elapsed: u32,
    /// Amount applied: what the end reverts (drain, fortify), the total delivered for rates.
    pub applied: f32,
    pub phase: Phase,
}

/// One landed source on a target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSource {
    pub id: InstanceId,
    pub source: SourceRef,
    pub class: SourceClass,
    pub caster: Option<ActorRef>,
    /// The fixed tick it landed on.
    pub started: u32,
    pub flags: InstanceFlags,
    /// A copy of the rolled entries: a removed record still ends cleanly.
    pub effects: SmallVec<[ActiveEffect; 4]>,
}

impl ActiveSource {
    pub fn is_live(&self) -> bool {
        self.effects.iter().any(|e| e.phase.is_live())
    }
}

/// Everything active on one target. Changed only by [`crate::apply`] and [`crate::tick`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ActiveSet {
    pub(crate) sources: Vec<ActiveSource>,
    /// Some live effect has a duration or a rate: [`crate::tick`] skips sets without.
    pub(crate) timed: bool,
}

impl ActiveSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Live sources in landing order.
    pub fn sources(&self) -> &[ActiveSource] {
        &self.sources
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn get(&self, id: InstanceId) -> Option<&ActiveSource> {
        self.sources.iter().find(|s| s.id == id)
    }

    pub fn is_timed(&self) -> bool {
        self.timed
    }

    /// The summed magnitude of running effects of `effect` with `arg` (`OpenMW` `MagicEffects`,
    /// script `GetEffect`).
    pub fn magnitude(&self, effect: &EffectKey, arg: Option<&ArgRef>) -> f32 {
        self.sources
            .iter()
            .flat_map(|s| &s.effects)
            .filter(|e| e.phase == Phase::Running && e.effect == *effect && e.arg.as_ref() == arg)
            .map(|e| e.magnitude)
            .sum()
    }
}
