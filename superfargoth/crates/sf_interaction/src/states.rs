//! Object states (interaction-world-rules.md "Rules layer", IN17a): `TagId` states with a time to live
//! on placements (`Burning`, `Wet`, `Charred`), Gameplay Tags with duration.
//!
//! - [`ObjectStates`]: the host's table by `PlacementId`, a few `(tag, expires)` per object, kept sorted
//!   by tag so the same operations give the same table whatever order they ran in.
//! - [`Ttl`]: `Secs` (whole ticks at `interact.tick_hz`, at least one) or `UntilRemoved`.
//! - Adding a tag the object already has refreshes it: the later expiry wins, `UntilRemoved` beats any
//!   `Secs`, so a refresh never shortens a state.
//! - [`ObjectStates::expire`] drops every state whose expiry tick has come (`expires <= now`), in
//!   (tick, placement, tag) order. The host runs it before the tick's rules, so `has` is current.
//!
//! - [`ObjectStates::rest_bytes`] / [`ObjectStates::from_rest_bytes`] (IN39d): one placement's states
//!   at rest as `[STATES_REST_VERSION]` then per state `tag u32 LE, remaining ticks u32 LE`
//!   (`u32::MAX`: until removed). Remaining time, never absolute ticks, so a save loads exact at any
//!   tick count.
//!
//! Rules evaluation is `rules::step_rules`; replication is a later slice.

use std::collections::{BTreeMap, BTreeSet};

use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::claims::Tick;
use crate::def::TagId;
use crate::params::InteractParams;

/// The format version of [`ObjectStates::rest_bytes`].
// tunables: const (format: save blob version)
pub const STATES_REST_VERSION: u8 = 1;
/// The remaining-ticks value of a state that lasts until removed.
// tunables: const (format: save blob sentinel)
const UNTIL_REMOVED: u32 = u32::MAX;

/// How long a state lasts once added.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Ttl {
    /// Seconds, rounded to whole ticks at `interact.tick_hz` (at least one).
    Secs(f32),
    /// Until a rule or the host removes it.
    UntilRemoved,
}

/// One state on one placement: its tag and the tick it expires at (`None`: until removed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateEntry {
    pub tag: TagId,
    pub expires: Option<Tick>,
}

/// The host's object states by placement.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObjectStates {
    /// Each placement's states, sorted by tag; no placement maps to an empty list.
    by_placement: BTreeMap<PlacementId, SmallVec<[StateEntry; 4]>>,
    /// Timed states by expiry, the sorted list `expire` walks.
    deadlines: BTreeSet<(Tick, PlacementId, TagId)>,
}

impl ObjectStates {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `tag` to `placement` at tick `now`, or refreshes it (the later expiry wins, `UntilRemoved`
    /// beats any `Secs`). Returns true when the placement did not have the tag before (a state change).
    pub fn add(&mut self, placement: PlacementId, tag: TagId, ttl: Ttl, now: Tick, params: &InteractParams) -> bool {
        let expires = match ttl {
            Ttl::Secs(s) => Some(now.plus(params.ticks(s))),
            Ttl::UntilRemoved => None,
        };
        self.add_until(placement, tag, expires)
    }

    /// [`Self::add`] with the expiry tick given (`None`: until removed).
    fn add_until(&mut self, placement: PlacementId, tag: TagId, expires: Option<Tick>) -> bool {
        let list = self.by_placement.entry(placement).or_default();
        match list.binary_search_by_key(&tag, |e| e.tag) {
            Ok(i) => {
                let old = list[i].expires;
                let new = match (old, expires) {
                    (None, _) | (_, None) => None,
                    (Some(a), Some(b)) => Some(a.max(b)),
                };
                if new != old {
                    if let Some(t) = old {
                        self.deadlines.remove(&(t, placement, tag));
                    }
                    if let Some(t) = new {
                        self.deadlines.insert((t, placement, tag));
                    }
                    list[i].expires = new;
                }
                false
            }
            Err(i) => {
                list.insert(i, StateEntry { tag, expires });
                if let Some(t) = expires {
                    self.deadlines.insert((t, placement, tag));
                }
                true
            }
        }
    }

    /// Removes `tag` from `placement`. Returns true when it was there.
    pub fn remove(&mut self, placement: PlacementId, tag: TagId) -> bool {
        let Some(list) = self.by_placement.get_mut(&placement) else {
            return false;
        };
        let Ok(i) = list.binary_search_by_key(&tag, |e| e.tag) else {
            return false;
        };
        if let Some(t) = list.remove(i).expires {
            self.deadlines.remove(&(t, placement, tag));
        }
        if list.is_empty() {
            self.by_placement.remove(&placement);
        }
        true
    }

    #[must_use]
    pub fn has(&self, placement: PlacementId, tag: TagId) -> bool {
        self.by_placement.get(&placement).is_some_and(|l| l.binary_search_by_key(&tag, |e| e.tag).is_ok())
    }

    /// The placement's states, sorted by tag (empty when it has none).
    #[must_use]
    pub fn states(&self, placement: PlacementId) -> &[StateEntry] {
        self.by_placement.get(&placement).map_or(&[], |l| l.as_slice())
    }

    /// The placement's state tags, sorted, for `TagFilter::matches` (verb `requires`, rule targets).
    #[must_use]
    pub fn tags(&self, placement: PlacementId) -> SmallVec<[TagId; 4]> {
        self.states(placement).iter().map(|e| e.tag).collect()
    }

    /// Drops every timed state whose expiry tick has come (`expires <= now`) and returns them in
    /// (expiry tick, placement, tag) order.
    pub fn expire(&mut self, now: Tick) -> Vec<(PlacementId, TagId)> {
        let mut out = Vec::new();
        while let Some(&(t, placement, tag)) = self.deadlines.first() {
            if t > now {
                break;
            }
            self.deadlines.pop_first();
            if let Some(list) = self.by_placement.get_mut(&placement) {
                if let Ok(i) = list.binary_search_by_key(&tag, |e| e.tag) {
                    list.remove(i);
                }
                if list.is_empty() {
                    self.by_placement.remove(&placement);
                }
            }
            out.push((placement, tag));
        }
        out
    }

    /// Placements with at least one state.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_placement.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_placement.is_empty()
    }

    /// Drops every state of `placement` (an unload, or a restore's fresh start). Returns true when it
    /// had any.
    pub fn forget(&mut self, placement: PlacementId) -> bool {
        let Some(list) = self.by_placement.remove(&placement) else { return false };
        for e in list {
            if let Some(t) = e.expires {
                self.deadlines.remove(&(t, placement, e.tag));
            }
        }
        true
    }

    /// `placement`'s states at rest at tick `now` (module docs): the remaining ticks of each timed
    /// state (0 for one due this tick, at most `u32::MAX - 1`), `u32::MAX` for one until removed.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn rest_bytes(&self, placement: PlacementId, now: Tick) -> Vec<u8> {
        let list = self.states(placement);
        let mut out = Vec::with_capacity(1 + 8 * list.len());
        out.push(STATES_REST_VERSION);
        for e in list {
            let remaining = e
                .expires
                .map_or(UNTIL_REMOVED, |t| t.0.saturating_sub(now.0).min(u64::from(UNTIL_REMOVED - 1)) as u32);
            out.extend_from_slice(&e.tag.0.to_le_bytes());
            out.extend_from_slice(&remaining.to_le_bytes());
        }
        out
    }

    /// Replaces `placement`'s states with the ones [`Self::rest_bytes`] wrote, each timed one
    /// expiring its remaining ticks after `now`. Any other version or length leaves the placement
    /// fresh (no states) and returns false.
    pub fn from_rest_bytes(&mut self, placement: PlacementId, bytes: &[u8], now: Tick) -> bool {
        self.forget(placement);
        let Some((&STATES_REST_VERSION, rest)) = bytes.split_first() else { return false };
        if rest.len() % 8 != 0 {
            return false;
        }
        for e in rest.as_chunks::<8>().0 {
            let tag = TagId(u32::from_le_bytes([e[0], e[1], e[2], e[3]]));
            let remaining = u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
            let expires = (remaining != UNTIL_REMOVED).then(|| now.plus(u64::from(remaining)));
            self.add_until(placement, tag, expires);
        }
        true
    }
}
