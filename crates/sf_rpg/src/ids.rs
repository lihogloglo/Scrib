//! Stable ids for players and actors (rpg.md "Players and actors", net.md "Players").

use serde::{Deserialize, Serialize};

/// Most players in one world (net.md: 2-4 player co-op).
pub const MAX_PLAYERS: u8 = 4;

/// A player's session id: a small index the host assigns at join, used on the wire, in
/// events and in script contexts. Never a singleton: every gameplay context names one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct PlayerId(u8);

impl PlayerId {
    /// The host's player (single-player is a host with no clients).
    pub const HOST: Self = Self(0);

    /// `None` when `index >= MAX_PLAYERS`.
    pub fn new(index: u8) -> Option<Self> {
        (index < MAX_PLAYERS).then_some(Self(index))
    }

    pub fn index(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for PlayerId {
    type Error = String;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        Self::new(index).ok_or_else(|| format!("player index {index} out of range (max {MAX_PLAYERS})"))
    }
}

impl From<PlayerId> for u8 {
    fn from(id: PlayerId) -> Self {
        id.0
    }
}

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
