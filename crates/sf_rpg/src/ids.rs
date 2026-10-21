//! Stable ids for players and actors (rpg.md "Players and actors", net.md "Players").

use serde::{Deserialize, Serialize};

/// The player's session id and the player cap live in `sf_ids`, shared with `sf_action`.
pub use sf_ids::{MAX_PLAYERS, PlayerId};

/// A player's persistent profile key (random, created once); saves find a returning
/// player's character by it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerKey(pub u128);

/// A stable actor id: the world's placement id or spawn id (`sf_rpg` does not depend on
/// `sf_world`). Never a Bevy `Entity`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActorKey(pub u128);

/// Whose inventory a command targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Holder {
    Player(PlayerId),
    Actor(ActorKey),
}
