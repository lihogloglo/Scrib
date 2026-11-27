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
//! Rules evaluation (`step_rules`), the save codec and replication are later slices.

use std::collections::{BTreeMap, BTreeSet};

use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::claims::Tick;
use crate::def::TagId;
use crate::params::InteractParams;

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
}
