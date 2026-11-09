//! Merchant purses on the world clock (services.md "Merchant purses on the world clock") and
//! the `sf/services` save section.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::ActorKey;
use crate::quest::GameTime;

/// Save section key of [`WorldServices`] (owner `sf_rpg`).
pub const SERVICES_SECTION: &str = "sf/services";
/// Schema version of [`SERVICES_SECTION`].
pub const SERVICES_VERSION: u32 = 1;

/// A provider's money, apart from its inventory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Purse {
    pub gold: i64,
    pub last_restock: GameTime,
}

impl Purse {
    /// Lazy restock: at or after `last_restock + reset_hours`, the purse resets to `base`.
    /// Returns whether it restocked.
    pub fn restock(&mut self, base: i64, reset_hours: u32, now: GameTime) -> bool {
        let due = self.last_restock.total_minutes().saturating_add(u64::from(reset_hours) * 60);
        if now.total_minutes() < due {
            return false;
        }
        self.gold = base;
        self.last_restock = now;
        true
    }
}

/// How a provider's money works (the adapter's `purse_base`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurseRule {
    /// A pool reset to `base` every `reset_hours` of game time.
    Pool { base: i64, reset_hours: u32 },
    /// No limit (fixed-price games).
    Unlimited,
    /// The provider's own currency stacks are its money.
    Inventory,
}

/// Host-owned service world state: the `sf/services` save section.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldServices {
    pub purses: BTreeMap<ActorKey, Purse>,
}

impl WorldServices {
    /// Run when a service opens: creates the purse at `base` on first open, else restocks it
    /// lazily. Returns the gold after a restock (for `PurseRestocked`), `None` if nothing
    /// changed or the rule has no pool.
    pub fn open(&mut self, provider: ActorKey, rule: PurseRule, now: GameTime) -> Option<i64> {
        let PurseRule::Pool { base, reset_hours } = rule else {
            return None;
        };
        if let Some(purse) = self.purses.get_mut(&provider) {
            purse.restock(base, reset_hours, now).then_some(purse.gold)
        } else {
            self.purses.insert(provider, Purse { gold: base, last_restock: now });
            Some(base)
        }
    }

    /// The pool's gold, if this provider has one.
    pub fn gold(&self, provider: ActorKey) -> Option<i64> {
        self.purses.get(&provider).map(|p| p.gold)
    }

    /// Adds `delta` to a pool (a trade's `purse_delta`, a list service's `Pay`). The purse may
    /// rise above its base until the next restock. No-op for providers without a pool.
    pub fn apply_delta(&mut self, provider: ActorKey, delta: i64) {
        if let Some(purse) = self.purses.get_mut(&provider) {
            purse.gold = purse.gold.saturating_add(delta);
        }
    }

    /// On load: drops purses of actors that no longer exist. Returns them, for the warning.
    pub fn drop_missing(&mut self, exists: impl Fn(ActorKey) -> bool) -> Vec<ActorKey> {
        let gone: Vec<ActorKey> = self.purses.keys().copied().filter(|k| !exists(*k)).collect();
        for key in &gone {
            self.purses.remove(key);
        }
        gone
    }
}
