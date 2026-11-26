//! Shared stable ids (net.md "Players", net-ready rules 6 and 7): plain serde data, never a
//! Bevy `Entity`. `sf_action` and `sf_rpg` both name players with [`PlayerId`] from here;
//! [`PlayerKey`] and [`ActorKey`] live here so `sf_rpg` and `sf_stats` share them (stats.md S0).
//! The world clock ([`GameClock`], [`GameInstant`]; clock.md) lives here too: one plain value
//! every core reads.

use serde::{Deserialize, Serialize};

pub mod clock;
#[cfg(test)]
mod tests_clock;

pub use clock::{GameClock, GameInstant};

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

/// A player's persistent profile key (random, created once); saves find a returning
/// player's character by it. Fits in 64 bits (a profile draws 64 random bits), so it has an
/// [`ActorKey`] of its own ([`ActorKey::player`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerKey(pub u128);

/// A stable actor id: the world's placement id or spawn id (this crate does not depend on
/// `sf_world`), or a player's character ([`ActorKey::player`]). Never a Bevy `Entity`.
///
/// **Players** (ai.md "Data model": every player, local or remote, is one target): a player's
/// key is [`ActorKey::PLAYERS`] in the high 64 bits and the [`PlayerKey`] in the low 64. No
/// other key source writes that block: placement ids are 128-bit hash prefixes (one lands in it
/// with probability 2^-64), entity- and hash-derived spawn keys fit in 64 bits (high half zero),
/// and sentinels sit at `u128::MAX - n` (high half all ones).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActorKey(pub u128);

impl ActorKey {
    /// The high half of every player's key: ASCII `sfplayer`.
    pub const PLAYERS: u128 = 0x7366_706c_6179_6572_0000_0000_0000_0000;
    /// The low half: the [`PlayerKey`].
    const PLAYER_BITS: u128 = 0xffff_ffff_ffff_ffff;

    /// The character of player `key` (its low 64 bits: [`PlayerKey`] fits in them).
    pub const fn player(key: PlayerKey) -> Self {
        debug_assert!(key.0 <= Self::PLAYER_BITS, "a PlayerKey fits in 64 bits");
        Self(Self::PLAYERS | (key.0 & Self::PLAYER_BITS))
    }

    /// The player this key is the character of, if it is one.
    pub const fn as_player(self) -> Option<PlayerKey> {
        if self.0 & !Self::PLAYER_BITS == Self::PLAYERS { Some(PlayerKey(self.0 & Self::PLAYER_BITS)) } else { None }
    }

    pub const fn is_player(self) -> bool {
        self.as_player().is_some()
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

    /// `MA2d3`: a player's [`ActorKey`] round-trips to its [`PlayerKey`], has the exact layout,
    /// and no placement-, entity- or sentinel-style key reads as a player.
    #[test]
    fn player_actor_keys_round_trip_and_stay_out_of_other_key_ranges() {
        for raw in [0, 1, 7, 0x00c0_ffee, u128::from(u64::MAX)] {
            let key = ActorKey::player(PlayerKey(raw));
            assert_eq!(key.as_player(), Some(PlayerKey(raw)));
            assert!(key.is_player());
            assert_eq!(key.0 >> 64, 0x7366_706c_6179_6572, "the high half is the tag");
            assert_eq!(key.0 & u128::from(u64::MAX), raw);
        }
        assert_ne!(ActorKey::player(PlayerKey(1)), ActorKey::player(PlayerKey(2)));
        // Entity- and hash-derived spawn keys (64 bits), test sentinels, small placement ids.
        let others = [0, 1, 42, u128::from(u64::MAX), u128::MAX, u128::MAX - 2, u128::MAX - 7, ActorKey::PLAYERS - 1];
        for raw in others {
            assert_eq!(ActorKey(raw).as_player(), None, "{raw:#x}");
        }
        // Next to the block, on both sides.
        assert_eq!(ActorKey(ActorKey::PLAYERS + (1 << 64)).as_player(), None);
        assert_eq!(ActorKey(ActorKey::PLAYERS).as_player(), Some(PlayerKey(0)));
    }
}
