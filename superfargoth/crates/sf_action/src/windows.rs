//! Typed windows on the action clock and the slot timing the chooser reads (action.md
//! "Windows"). B1's `animgraph.compile` fills a [`SlotSource`] from clip events; A1 callers
//! (tests, the shell's scaffold) fill a [`SlotTable`] by hand.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sf_anim::RootPath;

use crate::command::ClassSet;
use crate::ids::{ClockTime, SlotKey};
use crate::rule::{CompiledRule, WindowPolicy};

/// Which hit kinds a defence window covers (content flags; bit 0 = every plain hit). C1 uses
/// [`HitMask::ALL`]; typed i-frames (C3) name bits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HitMask(pub u8);

impl HitMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(u8::MAX);

    /// Covers any of `flags` (a hit row with no flags is covered by any non-empty mask).
    pub const fn covers(self, flags: Self) -> bool {
        if flags.0 == 0 { self.0 != 0 } else { self.0 & flags.0 != 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowKind {
    /// A press of these classes is latched from here (earlier presses are dropped).
    Accept(ClassSet),
    /// A latched request of these classes may start its move here.
    Cancel(ClassSet),
    /// Traversal: the last contact ended; cancels `Move`, `Jump`, `Dodge`, `Down`.
    Tail,
    /// Clip owner: the gameplay yaw turns toward the lock target, else the stick, at this rate
    /// (deg/s on the action clock).
    Turn(u16),
    /// Clip owner: no turning (wins over an overlapping `Turn`).
    NoTurn,
    /// Hit `n` of the move's combat table is active (the combat layer sweeps it).
    Hit(u8),
    /// Hits of these kinds don't land (they stay unspent: action.md "Windows").
    IFrames(HitMask),
}

impl WindowKind {
    /// Accept, cancel and tail windows: the ones the chooser gates on (the restricted rule
    /// counts only these).
    pub const fn gates(self) -> bool {
        matches!(self, Self::Accept(_) | Self::Cancel(_) | Self::Tail)
    }
}

/// A window `[start, end)` on the slot's local clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Window {
    pub kind: WindowKind,
    pub start: ClockTime,
    pub end: ClockTime,
}

impl Window {
    pub fn secs(kind: WindowKind, start: f32, end: f32) -> Self {
        Self { kind, start: ClockTime::from_secs(start), end: ClockTime::from_secs(end) }
    }
}

/// Timing of one clip slot.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotInfo {
    /// `None` loops (never ends).
    pub length: Option<ClockTime>,
    /// Authored or inferred windows; none = the rule's window policy (the restricted rule: a
    /// clip with no accept/cancel windows accepts and executes every class).
    pub windows: Vec<Window>,
}

impl SlotInfo {
    pub fn looping() -> Self {
        Self::default()
    }

    pub fn once(secs: f32) -> Self {
        Self { length: Some(ClockTime::from_secs(secs)), windows: Vec::new() }
    }

    #[must_use]
    pub fn with(mut self, w: Window) -> Self {
        self.windows.push(w);
        self
    }
}

/// Where slot timing comes from (compiled graph, or a hand table).
pub trait SlotSource {
    fn slot(&self, key: SlotKey) -> Option<&SlotInfo>;

    /// The slot clip's baked root track (Warp owners plan over it). `None`: the slot has no root
    /// motion, so a Warp rule using it is refused.
    fn root(&self, _key: SlotKey) -> Option<&dyn RootPath> {
        None
    }
}

/// A plain map of slots, with optional root paths.
#[derive(Clone, Default)]
pub struct SlotTable {
    pub slots: HashMap<SlotKey, SlotInfo>,
    pub roots: HashMap<SlotKey, Arc<dyn RootPath + Send + Sync>>,
}

impl std::fmt::Debug for SlotTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SlotTable").field("slots", &self.slots).field("roots", &self.roots.keys().collect::<Vec<_>>()).finish()
    }
}

impl SlotTable {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with(mut self, name: &str, info: SlotInfo) -> Self {
        self.slots.insert(SlotKey::of(name), info);
        self
    }

    /// Gives slot `name` a root path (its clip's root track).
    #[must_use]
    pub fn with_root(mut self, name: &str, root: Arc<dyn RootPath + Send + Sync>) -> Self {
        self.roots.insert(SlotKey::of(name), root);
        self
    }
}

impl SlotSource for SlotTable {
    fn slot(&self, key: SlotKey) -> Option<&SlotInfo> {
        self.slots.get(&key)
    }

    fn root(&self, key: SlotKey) -> Option<&dyn RootPath> {
        self.roots.get(&key).map(|r| &**r as &dyn RootPath)
    }
}

/// Where a move's clock is in its slot chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Located<'a> {
    pub chain: u8,
    pub slot: Option<SlotKey>,
    pub info: Option<&'a SlotInfo>,
    /// Time inside the current slot.
    pub local: ClockTime,
    /// Past the end of a finite chain (an empty chain ends at once).
    pub ended: bool,
}

/// Finds the current slot: each finite slot plays for its length, then the next; a looping slot
/// holds the chain. A slot the source doesn't have (dropped: no clip candidate present) is
/// skipped, unless the source has none of the chain's slots: then the first holds the chain as
/// before (a rule set checked with [`crate::RuleSet::check_slots`] never gets there).
pub fn locate<'a>(rule: &CompiledRule, slots: &'a dyn SlotSource, t: ClockTime) -> Located<'a> {
    let any = rule.slots.iter().any(|&k| slots.slot(k).is_some());
    let mut start = 0u32;
    let mut last = None;
    for (i, &key) in rule.slots.iter().enumerate() {
        let info = slots.slot(key);
        if info.is_none() && any {
            continue;
        }
        let chain = u8::try_from(i).unwrap_or(u8::MAX);
        last = Some((chain, key, info));
        match info.and_then(|s| s.length) {
            Some(len) if t.0 >= start + len.0 => start += len.0,
            _ => return Located { chain, slot: Some(key), info, local: ClockTime(t.0 - start), ended: false },
        }
    }
    let (chain, slot, info) = last.map_or((0, None, None), |(c, k, i)| (c, Some(k), i));
    Located { chain, slot, info, local: ClockTime(t.0 - start), ended: true }
}

/// The windows of `rule`'s current slot open at clock `t` (`start <= local < end`), every kind.
pub fn open_windows<'a>(rule: &CompiledRule, slots: &'a dyn SlotSource, t: ClockTime) -> impl Iterator<Item = &'a Window> + 'a {
    let at = locate(rule, slots, t);
    let windows = if at.ended { &[][..] } else { at.info.map(|s| s.windows.as_slice()).unwrap_or_default() };
    windows.iter().filter(move |w| w.start <= at.local && at.local < w.end)
}

/// What the current move allows now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gates {
    pub accept: ClassSet,
    pub cancel: ClassSet,
    /// No restriction at all (loops): automatic rules may take over.
    pub unrestricted: bool,
    pub ended: bool,
}

impl Gates {
    pub const OPEN: Self = Self { accept: ClassSet::ALL, cancel: ClassSet::ALL, unrestricted: true, ended: false };
}

/// The gates of `rule` at clock `t`.
pub fn gates(rule: &CompiledRule, slots: &dyn SlotSource, t: ClockTime) -> Gates {
    let at = locate(rule, slots, t);
    if at.ended {
        return Gates { ended: true, ..Gates::OPEN };
    }
    let authored = at.info.map(|s| s.windows.as_slice()).unwrap_or_default();
    if !authored.iter().any(|w| w.kind.gates()) {
        return match rule.policy {
            WindowPolicy::Committed => Gates { accept: ClassSet::ALL, cancel: ClassSet::EMPTY, unrestricted: false, ended: false },
            WindowPolicy::Unrestricted | WindowPolicy::Category => Gates::OPEN,
        };
    }
    let mut g = Gates { accept: ClassSet::EMPTY, cancel: ClassSet::EMPTY, unrestricted: false, ended: false };
    for w in authored.iter().filter(|w| w.start <= at.local && at.local < w.end) {
        match w.kind {
            WindowKind::Accept(c) => g.accept = g.accept.union(c),
            WindowKind::Cancel(c) => g.cancel = g.cancel.union(c),
            WindowKind::Tail => g.cancel = g.cancel.union(ClassSet::TAIL),
            WindowKind::Turn(_) | WindowKind::NoTurn | WindowKind::Hit(_) | WindowKind::IFrames(_) => {}
        }
    }
    g
}
