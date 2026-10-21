//! Stable ids and the fixed-tick clock units (net-ready rules 1 and 7).

use serde::{Deserialize, Serialize};

/// Fixed simulation rate, ticks per second (net.md "Tick").
pub const TICK_HZ: u32 = 60;

/// The fixed-tick counter. Wraps after ~2.3 years at 60 Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tick(pub u32);

impl Tick {
    #[must_use]
    pub fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
    }

    /// Ticks from `earlier` to `self`.
    pub fn since(self, earlier: Self) -> u32 {
        self.0.wrapping_sub(earlier.0)
    }
}

/// A person in the session: 0-3, assigned by the host at join (net.md "Players").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerId(pub u8);

/// A character's stable id (spawn id, placement id or player character), never a Bevy `Entity`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterId(pub u64);

/// A stable id of something a character acts on: an interaction point, an item, a door.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TargetId(pub u64);

/// 32-bit FNV-1a: stable across builds and platforms, so keys hashed from content ids match on
/// every peer (the join handshake guarantees identical content). Collisions are load errors.
pub const fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u32;
        h = h.wrapping_mul(0x0100_0193);
        i += 1;
    }
    h
}

macro_rules! hashed_key {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(pub u32);

        impl $name {
            pub const fn of(id: &str) -> Self {
                Self(fnv1a32(id.as_bytes()))
            }
        }
    };
}

hashed_key!(
    /// A move rule's id (`"air.jump"`) as a small stable key: what state, snapshots and events carry.
    RuleKey
);
hashed_key!(
    /// A clip slot name (`"loco"`, `"land.hard"`) as a key.
    SlotKey
);
hashed_key!(
    /// An interaction tag (`"door"`, `"item"`) as a key.
    TagId
);

/// Action-clock time in thousandths of a tick (1 s = 60 000). Integer, so clocks scaled by
/// hit-stop advance identically on every peer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClockTime(pub u32);

impl ClockTime {
    pub const ZERO: Self = Self(0);
    /// One tick at clock rate 1.
    pub const TICK: Self = Self(1000);

    /// Seconds, rounded to the nearest unit; negative clamps to zero.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn from_secs(s: f32) -> Self {
        Self((f64::from(s.max(0.0)) * f64::from(TICK_HZ) * 1000.0).round() as u32)
    }

    pub const fn from_ticks(ticks: u32) -> Self {
        Self(ticks * 1000)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn secs(self) -> f32 {
        (f64::from(self.0) / (f64::from(TICK_HZ) * 1000.0)) as f32
    }
}
