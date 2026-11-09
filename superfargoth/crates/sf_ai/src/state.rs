//! The HTN world state (ai.md "Data model"): typed slots, not strings.
//!
//! Every name a method record uses (`enemy_visible`, `health`, `threat`) is a key declared once
//! at load in [`StateKeys`] by the systems that write it (perception, facts, the adapter), and
//! resolves to a typed slot index: a [`BitKey`], an [`IntKey`] or a [`RefKey`]. A [`WorldState`]
//! is then three flat arrays (bits packed in `u64` words, `i32`s, optional [`Target`]s); nothing in
//! a tick compares strings. Unknown names fail when the library loads (`htn::LoadReport`).
//!
//! Reuse: std only. The spec's illustrative `SmallVec`/`BitSet` buy nothing here: a state is
//! cloned once per decomposition (the planner undoes effects through a journal, not by cloning
//! per choice point), so plain `Vec`s keep `sf_ai`'s dependency list short.
//!
//! The world state is rebuilt from perception and facts, never saved (ai.md "Saves"); serde is
//! for dumps (BRP, tests).

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_ids::ActorKey;

/// A world reference held in a [`RefKey`] slot: a stable world id (an [`ActorKey`] or a
/// placement id; same `u128` space). Planning only tests presence and equality; what it points
/// at is the shell's business (executors resolve it).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Target(pub u128);

impl From<ActorKey> for Target {
    fn from(actor: ActorKey) -> Self {
        Self(actor.0)
    }
}

macro_rules! key {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub u16);

        impl $name {
            fn at(self) -> usize {
                usize::from(self.0)
            }
        }
    };
}

key!(
    /// A boolean slot.
    BitKey
);
key!(
    /// An `i32` slot (counts, ratios in percent, enum-like codes).
    IntKey
);
key!(
    /// An optional [`Target`] slot.
    RefKey
);

/// Which array a key lives in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyKind {
    Bit,
    Int,
    Ref,
}

impl fmt::Display for KeyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Bit => "bit",
            Self::Int => "int",
            Self::Ref => "ref",
        })
    }
}

/// Any key, typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateKey {
    Bit(BitKey),
    Int(IntKey),
    Ref(RefKey),
}

impl StateKey {
    pub fn kind(self) -> KeyKind {
        match self {
            Self::Bit(_) => KeyKind::Bit,
            Self::Int(_) => KeyKind::Int,
            Self::Ref(_) => KeyKind::Ref,
        }
    }
}

/// A key name declared twice with different kinds, or too many keys.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StateKeyError {
    #[error("state key `{name}` is already a {existing} key, not a {wanted} key")]
    KindClash { name: String, existing: KeyKind, wanted: KeyKind },
    #[error("more than 65535 {0} keys")]
    TooMany(KeyKind),
}

/// The load-time key registry: name (ASCII case-folded) → typed slot, in declaration order per
/// kind. Declaring a name again with the same kind returns the same key, so two systems can share
/// a key; a different kind is an error.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StateKeys {
    names: [Vec<String>; 3],
    by_name: HashMap<String, StateKey>,
}

impl StateKeys {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares (or finds) a boolean key.
    pub fn add_bit(&mut self, name: &str) -> Result<BitKey, StateKeyError> {
        match self.add(name, KeyKind::Bit)? {
            StateKey::Bit(k) => Ok(k),
            _ => unreachable!("add returns the asked kind"),
        }
    }

    /// Declares (or finds) an integer key.
    pub fn add_int(&mut self, name: &str) -> Result<IntKey, StateKeyError> {
        match self.add(name, KeyKind::Int)? {
            StateKey::Int(k) => Ok(k),
            _ => unreachable!("add returns the asked kind"),
        }
    }

    /// Declares (or finds) a reference key.
    pub fn add_ref(&mut self, name: &str) -> Result<RefKey, StateKeyError> {
        match self.add(name, KeyKind::Ref)? {
            StateKey::Ref(k) => Ok(k),
            _ => unreachable!("add returns the asked kind"),
        }
    }

    fn add(&mut self, name: &str, kind: KeyKind) -> Result<StateKey, StateKeyError> {
        let name = name.to_ascii_lowercase();
        if let Some(&key) = self.by_name.get(&name) {
            return if key.kind() == kind {
                Ok(key)
            } else {
                Err(StateKeyError::KindClash { name, existing: key.kind(), wanted: kind })
            };
        }
        let list = &mut self.names[kind as usize];
        let slot = u16::try_from(list.len()).map_err(|_| StateKeyError::TooMany(kind))?;
        let key = match kind {
            KeyKind::Bit => StateKey::Bit(BitKey(slot)),
            KeyKind::Int => StateKey::Int(IntKey(slot)),
            KeyKind::Ref => StateKey::Ref(RefKey(slot)),
        };
        list.push(name.clone());
        self.by_name.insert(name, key);
        Ok(key)
    }

    /// The key named `name` (any spelling).
    pub fn get(&self, name: &str) -> Option<StateKey> {
        self.by_name.get(&name.to_ascii_lowercase()).copied()
    }

    /// The key's declared (folded) name; `"?"` for a key from another registry.
    pub fn name(&self, key: StateKey) -> &str {
        let (kind, slot) = match key {
            StateKey::Bit(k) => (0, k.at()),
            StateKey::Int(k) => (1, k.at()),
            StateKey::Ref(k) => (2, k.at()),
        };
        self.names[kind].get(slot).map_or("?", String::as_str)
    }

    /// Number of keys of `kind`.
    pub fn count(&self, kind: KeyKind) -> usize {
        self.names[kind as usize].len()
    }
}

/// A small dense bitset over slot indices (`u64` words).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
struct Bits(Vec<u64>);

impl Bits {
    fn zeros(n: usize) -> Self {
        Self(vec![0; n.div_ceil(64)])
    }

    fn get(&self, i: usize) -> bool {
        self.0.get(i / 64).is_some_and(|w| w >> (i % 64) & 1 == 1)
    }

    fn set(&mut self, i: usize, on: bool) {
        if self.0.len() <= i / 64 {
            self.0.resize(i / 64 + 1, 0);
        }
        let mask = 1u64 << (i % 64);
        if on {
            self.0[i / 64] |= mask;
        } else {
            self.0[i / 64] &= !mask;
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.0.iter().zip(&other.0).any(|(a, b)| a & b != 0)
    }

    fn union(&mut self, other: &Self) {
        if self.0.len() < other.0.len() {
            self.0.resize(other.0.len(), 0);
        }
        for (a, b) in self.0.iter_mut().zip(&other.0) {
            *a |= b;
        }
    }

    fn is_empty(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }
}

/// A set of keys: what a plan read (ai.md "Data model": "plans record which keys they read, so
/// a change marks them dirty"), or which keys changed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeySet {
    bits: Bits,
    ints: Bits,
    refs: Bits,
}

impl KeySet {
    pub fn new() -> Self {
        Self::default()
    }

    fn part(&self, key: StateKey) -> (&Bits, usize) {
        match key {
            StateKey::Bit(k) => (&self.bits, k.at()),
            StateKey::Int(k) => (&self.ints, k.at()),
            StateKey::Ref(k) => (&self.refs, k.at()),
        }
    }

    pub fn insert(&mut self, key: StateKey) {
        let (part, i) = match key {
            StateKey::Bit(k) => (&mut self.bits, k.at()),
            StateKey::Int(k) => (&mut self.ints, k.at()),
            StateKey::Ref(k) => (&mut self.refs, k.at()),
        };
        part.set(i, true);
    }

    pub fn contains(&self, key: StateKey) -> bool {
        let (part, i) = self.part(key);
        part.get(i)
    }

    /// Whether any key is in both sets (a changed key the plan read: re-validate it).
    pub fn intersects(&self, other: &Self) -> bool {
        self.bits.intersects(&other.bits) || self.ints.intersects(&other.ints) || self.refs.intersects(&other.refs)
    }

    /// Adds every key of `other`.
    pub fn union(&mut self, other: &Self) {
        self.bits.union(&other.bits);
        self.ints.union(&other.ints);
        self.refs.union(&other.refs);
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty() && self.ints.is_empty() && self.refs.is_empty()
    }

    pub fn clear(&mut self) {
        self.bits.0.clear();
        self.ints.0.clear();
        self.refs.0.clear();
    }
}

/// One actor's planning state: every key of a [`StateKeys`] registry, default `false` / `0` /
/// `None`.
///
/// # Panics
/// The accessors panic on a key from a bigger registry than the one the state was made for (a
/// programming error: keys and states come from the same registry).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldState {
    bits: Bits,
    nbits: u16,
    ints: Vec<i32>,
    refs: Vec<Option<Target>>,
}

impl WorldState {
    /// A default state with a slot for every key in `keys`.
    pub fn new(keys: &StateKeys) -> Self {
        let n = keys.count(KeyKind::Bit);
        Self {
            bits: Bits::zeros(n),
            nbits: u16::try_from(n).unwrap_or(u16::MAX),
            ints: vec![0; keys.count(KeyKind::Int)],
            refs: vec![None; keys.count(KeyKind::Ref)],
        }
    }

    #[inline]
    pub fn bit(&self, key: BitKey) -> bool {
        assert!(key.0 < self.nbits, "bit key {} out of range", key.0);
        self.bits.get(key.at())
    }

    #[inline]
    pub fn set_bit(&mut self, key: BitKey, on: bool) {
        assert!(key.0 < self.nbits, "bit key {} out of range", key.0);
        self.bits.set(key.at(), on);
    }

    #[inline]
    pub fn int(&self, key: IntKey) -> i32 {
        self.ints[key.at()]
    }

    #[inline]
    pub fn set_int(&mut self, key: IntKey, value: i32) {
        self.ints[key.at()] = value;
    }

    #[inline]
    pub fn target(&self, key: RefKey) -> Option<Target> {
        self.refs[key.at()]
    }

    #[inline]
    pub fn set_target(&mut self, key: RefKey, target: Option<Target>) {
        self.refs[key.at()] = target;
    }

    /// The keys whose values differ between `self` and `other` (same registry): what a
    /// perception update dirtied, to intersect with a plan's reads.
    pub fn changed(&self, other: &Self) -> KeySet {
        let mut set = KeySet::new();
        for i in 0..self.nbits {
            if self.bits.get(usize::from(i)) != other.bits.get(usize::from(i)) {
                set.insert(StateKey::Bit(BitKey(i)));
            }
        }
        for (i, (a, b)) in (0u16..).zip(self.ints.iter().zip(&other.ints)) {
            if a != b {
                set.insert(StateKey::Int(IntKey(i)));
            }
        }
        for (i, (a, b)) in (0u16..).zip(self.refs.iter().zip(&other.refs)) {
            if a != b {
                set.insert(StateKey::Ref(RefKey(i)));
            }
        }
        set
    }
}
