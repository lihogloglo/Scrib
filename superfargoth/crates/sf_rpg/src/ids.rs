//! Stable ids for players and actors (rpg.md "Players and actors", net.md "Players").

use serde::{Deserialize, Serialize};

/// The player's session id, the player cap and the persistent player and actor keys live in
/// `sf_ids`, shared with `sf_action` and `sf_stats` (stats.md slice S0).
pub use sf_ids::{ActorKey, MAX_PLAYERS, PlayerId, PlayerKey};

/// Whose inventory a command targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Holder {
    Player(PlayerId),
    Actor(ActorKey),
}
