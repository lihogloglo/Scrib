//! Shared stable ids (net.md "Players", net-ready rules 6 and 7): plain serde data, never a
//! Bevy `Entity`. `sf_action` and `sf_rpg` both name players with [`PlayerId`] from here.

use serde::{Deserialize, Serialize};

/// Most players in one world (net.md: 2-4 player co-op).
pub const MAX_PLAYERS: u8 = 4;

/// A player's session id: a small index (0..[`MAX_PLAYERS`]) the host assigns at join, used
/// on the wire, in commands, events and script contexts. Never a singleton: every gameplay
/// context names one. Out-of-range indices are rejected on construction and on deserialize.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct PlayerId(u8);

impl PlayerId {
    /// The host's player (single-player is a host with no clients).
    pub const HOST: Self = Self(0);

    /// `None` when `index >= MAX_PLAYERS`.
    pub const fn new(index: u8) -> Option<Self> {
        if index < MAX_PLAYERS { Some(Self(index)) } else { None }
    }

    pub const fn index(self) -> u8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_ids_are_validated_and_round_trip() {
        assert_eq!(PlayerId::new(3).map(PlayerId::index), Some(3));
        assert!(PlayerId::new(MAX_PLAYERS).is_none());
        assert!(PlayerId::try_from(MAX_PLAYERS).is_err());
        for i in 0..MAX_PLAYERS {
            let id = PlayerId::new(i).unwrap();
            let bytes = postcard::to_allocvec(&id).unwrap();
            assert_eq!(bytes, [i]);
            assert_eq!(postcard::from_bytes::<PlayerId>(&bytes).unwrap(), id);
        }
        assert!(postcard::from_bytes::<PlayerId>(&[MAX_PLAYERS]).is_err());
    }
}
