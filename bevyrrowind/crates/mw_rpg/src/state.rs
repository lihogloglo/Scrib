//! Morrowind-specific saved RPG state.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_rpg::ActorKey;

/// Per-NPC dialogue state (world state, host-owned, saved): the disposition modifier that
/// `ModDisposition` / `SetDisposition` change on top of the base disposition.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcStates {
    disposition: BTreeMap<ActorKey, i32>,
}

impl NpcStates {
    pub fn modifier(&self, npc: ActorKey) -> i32 {
        self.disposition.get(&npc).copied().unwrap_or(0)
    }

    pub fn modify(&mut self, npc: ActorKey, delta: i32) {
        *self.disposition.entry(npc).or_default() += delta;
    }

    /// `SetDisposition`: makes `base + modifier` equal `value`.
    pub fn set(&mut self, npc: ActorKey, base: i32, value: i32) {
        self.disposition.insert(npc, value - base);
    }

    /// Base plus modifier, clamped to 0..=100.
    pub fn disposition(&self, npc: ActorKey, base: i32) -> i32 {
        (base + self.modifier(npc)).clamp(0, 100)
    }
}
