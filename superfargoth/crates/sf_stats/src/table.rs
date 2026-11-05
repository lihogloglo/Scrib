//! Stat definitions (stats.md "Definitions"): which stats exist, their kinds, and the dense
//! runtime indices. Content data: the framework names no stat.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// A stat's save and wire key: a case-folded (ASCII) string such as `"strength"` or `"health"`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StatKey(Box<str>);

impl StatKey {
    pub fn new(key: &str) -> Self {
        Self(key.to_ascii_lowercase().into_boxed_str())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for StatKey {
    fn from(key: &str) -> Self {
        Self::new(key)
    }
}

impl fmt::Debug for StatKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl fmt::Display for StatKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Serialized as its folded string; folded again on load.
impl Serialize for StatKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for StatKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let key = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        Ok(Self::new(&key))
    }
}

/// A stat's dense runtime index in its [`StatTable`], assigned at load in definition order.
/// Runtime only: saves and the wire use [`StatKey`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StatIdx(pub u16);

/// What a pool's current does when its maximum moves (stats.md "Definitions").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rebase {
    /// current / max is kept (a max of 0 before leaves current 0).
    KeepRatio,
    /// current is kept, lowered to the new max if above it.
    KeepCurrent,
    /// max - current is kept.
    KeepDeficit,
}

/// The kind of a stat (stats.md "Definitions" table).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatKind {
    /// Base and damage, modified = `clamp((eff_base - damage + temp) * (1 + Σmul), min, cap)`.
    Value { min: f32, cap: Option<f32> },
    /// Base (the max before modifiers) and current; `Depleted` fires when current crosses below
    /// `depleted_below`.
    Pool { depleted_below: f32, rebase: Rebase },
    /// One number, no modifiers, clamped to `min..=max`.
    Counter { min: f32, max: f32 },
    /// Computed by `DerivedModel::derived`; cached, never saved.
    Derived,
}

/// One stat definition (content data; a `stat` record or synthesised by an adapter).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatDef {
    pub key: StatKey,
    pub kind: StatKind,
    /// The content's grouping for UI and queries (e.g. a game's attributes, skills, counters).
    pub group: String,
    /// Replicated to every client (health ratio, level), not only the owner's.
    pub public: bool,
    /// Localisation key for the display name.
    pub label: String,
}

/// The few stats framework consumers find without knowing names (HUD bars, combat glue,
/// stats screen). Everything else is read by key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Role {
    Health,
    Mana,
    Stamina,
    Level,
    CarryCapacity,
    Encumbrance,
    Bounty,
}

impl Role {
    pub const ALL: [Self; 7] =
        [Self::Health, Self::Mana, Self::Stamina, Self::Level, Self::CarryCapacity, Self::Encumbrance, Self::Bounty];
}

/// Content-supplied role assignments, by key.
pub type RoleMap = Vec<(Role, StatKey)>;

/// A table that can't be built.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TableError {
    #[error("stat `{0}` is defined twice")]
    Duplicate(StatKey),
    #[error("more than {} stats", u16::MAX)]
    TooMany,
    #[error("role {role:?} names unknown stat `{key}`")]
    UnknownRoleStat { role: Role, key: StatKey },
    #[error("role {role:?} needs a {expected} stat, `{key}` is not one")]
    RoleKind { role: Role, key: StatKey, expected: &'static str },
    #[error("stat `{0}` has an empty key")]
    EmptyKey(StatKey),
}

/// Where a stat lives in a sheet: the kind's array and the slot in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Slot {
    Value(u16),
    Pool(u16),
    Counter(u16),
    Derived(u16),
}

/// The stat definitions of one game (or one mod list), with dense indices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatTable {
    defs: Vec<StatDef>,
    slots: Vec<Slot>,
    #[serde(skip)]
    by_key: HashMap<StatKey, StatIdx>,
    roles: Vec<(Role, StatIdx)>,
    /// The stat indices of each kind array in slot order (0 value, 1 pool, 2 counter, 3 derived).
    by_kind: [Vec<StatIdx>; 4],
}

/// `i` as an index: tables hold at most `u16::MAX` stats (checked in [`StatTable::new`]).
#[allow(clippy::cast_possible_truncation)]
const fn at(i: usize) -> StatIdx {
    StatIdx(i as u16)
}

impl StatTable {
    /// Builds the table: indices follow `defs` order; role kinds are checked (health, mana and
    /// stamina are pools; level and bounty counters; carry capacity and encumbrance any kind).
    pub fn new(defs: Vec<StatDef>, roles: &[(Role, StatKey)]) -> Result<Self, TableError> {
        if defs.len() > usize::from(u16::MAX) {
            return Err(TableError::TooMany);
        }
        let mut by_key = HashMap::with_capacity(defs.len());
        let mut slots = Vec::with_capacity(defs.len());
        let mut by_kind: [Vec<StatIdx>; 4] = Default::default();
        for (i, def) in defs.iter().enumerate() {
            if def.key.as_str().is_empty() {
                return Err(TableError::EmptyKey(def.key.clone()));
            }
            let idx = at(i);
            if by_key.insert(def.key.clone(), idx).is_some() {
                return Err(TableError::Duplicate(def.key.clone()));
            }
            let k = match def.kind {
                StatKind::Value { .. } => 0,
                StatKind::Pool { .. } => 1,
                StatKind::Counter { .. } => 2,
                StatKind::Derived => 3,
            };
            let n = at(by_kind[k].len()).0;
            by_kind[k].push(idx);
            slots.push(match k {
                0 => Slot::Value(n),
                1 => Slot::Pool(n),
                2 => Slot::Counter(n),
                _ => Slot::Derived(n),
            });
        }
        let mut table = Self { defs, slots, by_key, roles: Vec::new(), by_kind };
        for (role, key) in roles {
            let idx = table.idx(key).ok_or_else(|| TableError::UnknownRoleStat { role: *role, key: key.clone() })?;
            let ok = match role {
                Role::Health | Role::Mana | Role::Stamina => table.is_pool(idx),
                Role::Level | Role::Bounty => matches!(table.slot(idx), Slot::Counter(_)),
                Role::CarryCapacity | Role::Encumbrance => true,
            };
            if !ok {
                let expected = if matches!(role, Role::Level | Role::Bounty) { "counter" } else { "pool" };
                return Err(TableError::RoleKind { role: *role, key: key.clone(), expected });
            }
            table.roles.retain(|(r, _)| r != role);
            table.roles.push((*role, idx));
        }
        Ok(table)
    }

    /// Rebuilds the key index after deserialization (it isn't serialized).
    pub fn reindex(&mut self) {
        self.by_key = self.defs.iter().enumerate().map(|(i, d)| (d.key.clone(), at(i))).collect();
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }

    pub fn idx(&self, key: &StatKey) -> Option<StatIdx> {
        self.by_key.get(key).copied()
    }

    /// Looks a key up from any spelling (folds it first).
    pub fn find(&self, key: &str) -> Option<StatIdx> {
        self.idx(&StatKey::new(key))
    }

    /// # Panics
    /// When `idx` is not from this table.
    pub fn def(&self, idx: StatIdx) -> &StatDef {
        &self.defs[usize::from(idx.0)]
    }

    pub fn get(&self, idx: StatIdx) -> Option<&StatDef> {
        self.defs.get(usize::from(idx.0))
    }

    pub fn defs(&self) -> impl Iterator<Item = (StatIdx, &StatDef)> {
        self.defs.iter().enumerate().map(|(i, d)| (at(i), d))
    }

    pub fn role(&self, role: Role) -> Option<StatIdx> {
        self.roles.iter().find(|(r, _)| *r == role).map(|(_, i)| *i)
    }

    pub fn is_pool(&self, idx: StatIdx) -> bool {
        matches!(self.slots.get(usize::from(idx.0)), Some(Slot::Pool(_)))
    }

    pub(crate) fn slot(&self, idx: StatIdx) -> Slot {
        self.slots[usize::from(idx.0)]
    }

    pub(crate) fn try_slot(&self, idx: StatIdx) -> Option<Slot> {
        self.slots.get(usize::from(idx.0)).copied()
    }

    pub(crate) fn of_kind(&self, k: usize) -> &[StatIdx] {
        &self.by_kind[k]
    }

    pub(crate) fn count(&self, k: usize) -> usize {
        self.by_kind[k].len()
    }
}
