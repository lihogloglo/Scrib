//! Effect definitions (effects.md "Effect definitions"): content data, the framework names no
//! effect. A def says what an effect does to stats ([`Archetype`]); magnitudes, durations and
//! targets come with each landing ([`crate::EffectEntry`]).

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_stats::StatKey;
use smallvec::SmallVec;

use crate::active::SourceClass;

/// An effect's save and wire key: a case-folded (ASCII) string, as [`StatKey`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EffectKey(Box<str>);

impl EffectKey {
    pub fn new(key: &str) -> Self {
        Self(key.to_ascii_lowercase().into_boxed_str())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EffectKey {
    fn from(key: &str) -> Self {
        Self::new(key)
    }
}

impl fmt::Debug for EffectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl fmt::Display for EffectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Serialized as its folded string; folded again on load.
impl Serialize for EffectKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EffectKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let key = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        Ok(Self::new(&key))
    }
}

/// A content tag: a source's cure group (`disease`, `curse`), a [`Archetype::Request`] kind
/// (`open`, `teleport`). Case-folded (ASCII), serialized as its string, like [`EffectKey`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag(Box<str>);

impl Tag {
    /// The folded tag.
    pub fn new(tag: &str) -> Self {
        Self(tag.to_ascii_lowercase().into_boxed_str())
    }

    /// The folded string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Tag {
    fn from(tag: &str) -> Self {
        Self::new(tag)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Serialized as its folded string; folded again on load.
impl Serialize for Tag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tag = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        Ok(Self::new(&tag))
    }
}

/// A def's dense runtime index in its [`EffectTable`] (definition order). Runtime only: saves
/// and the wire use [`EffectKey`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EffectIdx(pub u16);

/// What an effect entry's argument names.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgKind {
    None,
    /// A stat of this content group (`StatDef::group`, e.g. a game's attributes or skills).
    Stat(String),
    /// A record of this kind (a creature to summon, an item to bind).
    Record(String),
}

/// An effect entry's argument: the stat or record it acts on.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArgRef {
    Stat(StatKey),
    Record(RecordId),
}

/// Which stat an archetype writes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatTarget {
    Fixed(StatKey),
    /// The entry's [`ArgRef::Stat`] (fortify *an* attribute).
    Arg,
}

/// Which `sf_stats` layer a modifier goes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerRule {
    /// `Permanent` for an innate source (`OpenMW` `Flag_AffectsBaseValues`), else `Temporary`.
    BySource,
}

/// What a [`Archetype::Pool`] effect does to a pool's current.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolMode {
    /// Lowers the current, not below 0.
    Hurt,
    /// Raises the current, not above the max.
    Heal,
    /// Lowers the current, below 0 allowed; reverted at the end.
    Drain,
    /// Raises max and current by the magnitude (a modifier, `Permanent` for an innate source,
    /// that shifts the current instead of rebasing it); reverted at the end.
    Fortify,
    /// Raises the current over the max, which stays; the same amount comes off at the end,
    /// below 0 allowed. Innate sources raise max and current together, as `Fortify` (Morrowind's
    /// Fortify Health / Magicka / Fatigue: `adjustDynamicStat`, `modDynamicStat` for abilities).
    Boost,
}

impl PoolMode {
    /// The caster's side of an absorb.
    #[must_use]
    pub fn inverse(self) -> Self {
        match self {
            Self::Hurt => Self::Heal,
            Self::Heal => Self::Hurt,
            Self::Drain => Self::Fortify,
            Self::Fortify | Self::Boost => Self::Drain,
        }
    }
}

/// What an effect does to stats (effects.md archetype table).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Archetype {
    /// An `Add` modifier of the magnitude while active.
    Modify { stat: StatTarget, layer: LayerRule },
    /// An `Add` modifier on a base-0 value stat other systems read (reflect, chameleon, light).
    MagnitudeStat { stat: StatTarget },
    /// Damage of the magnitude per second (or once with `applied_once`); never reverted.
    Damage { stat: StatTarget, cap_at_modified: bool },
    /// Restore of the magnitude per second (or once); never reverted.
    Restore { stat: StatTarget },
    /// Damage of the magnitude at the start, the same amount restored at the end.
    Drain { stat: StatTarget },
    /// A pool's current (and for `Fortify` and innate `Boost` its max), per [`PoolMode`].
    Pool { pool: StatTarget, mode: PoolMode },
    /// The target gets `inner`, the caster its inverse, under one cause.
    Absorb { inner: Box<Archetype> },
    /// Ends the target's matching sources when it lands (instant; E3).
    Cure { filter: CureFilter },
    /// Not a sheet change: the step emits `EffectRequest::Effect` at start and end for the
    /// game's handler (open, teleport, summon, levitate), which may answer
    /// `EffectOp::Invalidate` (E3).
    Request { kind: Tag },
}

/// What a cure ends (effects.md `CureFilter`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CureFilter {
    /// Running effects of this key in temporary sources (cure poison, cure paralysis); the
    /// spec's `Archetype(EffectKey)`.
    Effect(EffectKey),
    /// Whole instances whose `SourceDef` carries the tag (cure disease, remove curse); a cured
    /// innate or affliction source also leaves the book.
    SourceTag(Tag),
    /// Temporary spell instances (`Cast`, `Power`), each ended with chance `magnitude / 100`
    /// when `chance_from_magnitude` (else always); `harmful_only` spares sources without a
    /// harmful effect.
    Dispel { harmful_only: bool, chance_from_magnitude: bool },
}

/// What [`Archetype::stat`] returns for archetypes that write no stat.
static NO_STAT: StatTarget = StatTarget::Arg;

impl Archetype {
    /// The archetype an absorb wraps (itself when not an absorb).
    pub fn innermost(&self) -> &Self {
        match self {
            Self::Absorb { inner } => inner.innermost(),
            a => a,
        }
    }

    /// The stat the innermost archetype writes. `Cure` and `Request` write none and return
    /// `StatTarget::Arg`; [`Archetype::stat_target`] tells them apart.
    pub fn stat(&self) -> &StatTarget {
        self.stat_target().unwrap_or(&NO_STAT)
    }

    /// The stat the innermost archetype writes; `None` for `Cure` and `Request`.
    pub fn stat_target(&self) -> Option<&StatTarget> {
        match self.innermost() {
            Self::Modify { stat, .. }
            | Self::MagnitudeStat { stat }
            | Self::Damage { stat, .. }
            | Self::Restore { stat }
            | Self::Drain { stat }
            | Self::Pool { pool: stat, .. } => Some(stat),
            Self::Cure { .. } | Self::Request { .. } => None,
            Self::Absorb { .. } => unreachable!("innermost is never an absorb"),
        }
    }

    /// Delivers its magnitude per second (unless the def is `applied_once`).
    pub fn is_rate(&self) -> bool {
        matches!(
            self.innermost(),
            Self::Damage { .. } | Self::Restore { .. } | Self::Pool { mode: PoolMode::Hurt | PoolMode::Heal, .. }
        )
    }
}

/// Per-def switches (effects.md "Effect definitions").
#[allow(clippy::struct_excessive_bools, reason = "independent content flags")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectFlags {
    pub no_magnitude: bool,
    /// Instant in a timed source.
    pub no_duration: bool,
    /// Lands once (start ops), kept until it ends; else a rate per second for rate archetypes.
    pub applied_once: bool,
    pub harmful: bool,
    pub unreflectable: bool,
    /// Dropped when there is no caster.
    pub caster_linked: bool,
    /// Refuses a new landing while the same source is active on the target.
    pub non_recastable: bool,
}

/// What a second landing of the same effect from the same source does (effects.md
/// "Effect lists and sources"; matched by [`crate::StackKey`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StackPolicy {
    /// The old effect ends, the new one starts.
    Replace,
    /// The old effect's duration restarts; the new one is dropped.
    Refresh,
    /// Both run, up to `limit` at once; past it the new one is dropped.
    Stack { limit: u16 },
    /// The new one is dropped.
    Ignore,
}

/// One effect definition (content data; a record of kind `effect` or synthesised by an adapter).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectDef {
    pub key: EffectKey,
    pub archetype: Archetype,
    pub arg: ArgKind,
    pub flags: EffectFlags,
    /// Per unit of magnitude and duration; read by the cost rules (E2+).
    pub base_cost: f32,
    /// Stats the resist rule reads (E2+).
    pub resisted_by: SmallVec<[StatKey; 2]>,
    pub weakened_by: SmallVec<[StatKey; 1]>,
    /// `None`: the source class's default ([`SourceClass::default_stack`]).
    pub stack: Option<StackPolicy>,
    /// Localisation key for the display name.
    pub label: String,
}

impl EffectDef {
    /// A def with no argument, no flags, no cost and the class's stacking.
    pub fn new(key: &str, archetype: Archetype) -> Self {
        Self {
            key: EffectKey::new(key),
            archetype,
            arg: ArgKind::None,
            flags: EffectFlags::default(),
            base_cost: 0.0,
            resisted_by: SmallVec::new(),
            weakened_by: SmallVec::new(),
            stack: None,
            label: format!("effect.{}", key.to_ascii_lowercase()),
        }
    }

    /// The stacking policy for a landing from `class`.
    pub fn stack_policy(&self, class: SourceClass) -> StackPolicy {
        self.stack.unwrap_or(class.default_stack())
    }

    /// Delivers per tick: a rate archetype that is not `applied_once`.
    pub fn is_rate(&self) -> bool {
        self.archetype.is_rate() && !self.flags.applied_once
    }
}

/// A table that can't be built.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EffectTableError {
    #[error("effect `{0}` is defined twice")]
    Duplicate(EffectKey),
    #[error("more than {} effects", u16::MAX)]
    TooMany,
}

/// The content's effects, by key and dense index.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EffectTable {
    defs: Vec<EffectDef>,
    index: BTreeMap<EffectKey, EffectIdx>,
}

impl EffectTable {
    pub fn new(defs: Vec<EffectDef>) -> Result<Self, EffectTableError> {
        let mut index = BTreeMap::new();
        for (i, def) in defs.iter().enumerate() {
            let idx = EffectIdx(u16::try_from(i).map_err(|_| EffectTableError::TooMany)?);
            if index.insert(def.key.clone(), idx).is_some() {
                return Err(EffectTableError::Duplicate(def.key.clone()));
            }
        }
        Ok(Self { defs, index })
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }

    pub fn idx(&self, key: &EffectKey) -> Option<EffectIdx> {
        self.index.get(key).copied()
    }

    pub fn get(&self, key: &EffectKey) -> Option<&EffectDef> {
        self.idx(key).map(|i| &self.defs[usize::from(i.0)])
    }

    /// Panics on an index from another table.
    pub fn def(&self, idx: EffectIdx) -> &EffectDef {
        &self.defs[usize::from(idx.0)]
    }

    pub fn defs(&self) -> impl Iterator<Item = (EffectIdx, &EffectDef)> {
        self.defs.iter().zip(0u16..).map(|(d, i)| (EffectIdx(i), d))
    }
}
