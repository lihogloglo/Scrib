//! `sf/meta`: the always-present, never-compressed first section (save.md "`sf/meta`").

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::codec::{DecodeError, decode, encode};

/// Schema version of [`SaveMeta`] (stored in the TOC like every section's).
pub const META_VERSION: u32 = 1;

/// Why a save was written. Drives slot rotation ([`crate::RotationPolicy`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SaveReason {
    Manual,
    Quick,
    Auto,
    /// Written when the game exits.
    Exit,
    /// The host autosaves when a guest leaves (net.md "Saves").
    GuestLeft,
}

impl SaveReason {
    pub const ALL: [Self; 5] = [Self::Manual, Self::Quick, Self::Auto, Self::Exit, Self::GuestLeft];

    /// The file-name token (`000042-quick.sfsave`).
    pub fn token(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Quick => "quick",
            Self::Auto => "auto",
            Self::Exit => "exit",
            Self::GuestLeft => "guestleft",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.token() == token)
    }
}

impl fmt::Display for SaveReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}

/// One mod of the load order, as the save recorded it (mods.md "Profile fingerprint").
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModRef {
    pub id: String,
    pub version: String,
    /// blake3 of the mod's manifest.
    pub manifest_hash: [u8; 32],
}

/// A player of the world (`sf/party` carries the full state).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartyMember {
    /// The persistent profile key (`PlayerKey`).
    pub player_key: u128,
    pub name: String,
}

/// What a save list shows and what load compares against the running profile. Display fields
/// are for the load screen only; nothing in them feeds gameplay (net rule 4).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveMeta {
    /// Random at new game; every save of a playthrough shares it.
    pub world_id: u128,
    /// Random per save.
    pub save_id: u128,
    /// The save this world state was loaded from or last saved as (`None` for the first).
    pub parent: Option<u128>,
    /// Save number within the world, increasing.
    pub sequence: u64,
    pub reason: SaveReason,
    /// Wall clock at saving, Unix seconds (display only).
    pub wall_clock: i64,
    /// Game time in the clock's days (display only; `sf/clock` is the state).
    pub game_days: f64,
    /// Accumulated play time, milliseconds.
    pub play_time_ms: u64,
    /// Fixed simulation ticks since the world began.
    pub sim_tick: u64,
    /// `sf_save`'s crate version that wrote the file.
    pub framework_version: String,
    /// The content pack (Bevyrrowind...) and its version.
    pub content_id: String,
    pub content_version: String,
    /// Free-form build identifier (git hash, store build number).
    pub build_id: String,
    /// Mod profile name.
    pub profile: String,
    /// The load order at saving.
    pub load_order: Vec<ModRef>,
    /// The profile fingerprint (mods.md): equal fingerprints load without a mod diff.
    pub profile_fingerprint: [u8; 32],
    /// `GRID_CELL` of the build (world-coordinates.md): positions in the file use it.
    pub grid_cell: f32,
    pub party: Vec<PartyMember>,
    /// Load-screen labels from the adapter.
    pub character_name: String,
    pub character_level: Option<u32>,
    pub location_label: String,
}

impl Default for SaveMeta {
    fn default() -> Self {
        Self {
            world_id: 0,
            save_id: 0,
            parent: None,
            sequence: 0,
            reason: SaveReason::Manual,
            wall_clock: 0,
            game_days: 0.0,
            play_time_ms: 0,
            sim_tick: 0,
            framework_version: env!("CARGO_PKG_VERSION").to_owned(),
            content_id: String::new(),
            content_version: String::new(),
            build_id: String::new(),
            profile: String::new(),
            load_order: Vec::new(),
            profile_fingerprint: [0; 32],
            grid_cell: 0.0,
            party: Vec::new(),
            character_name: String::new(),
            character_level: None,
            location_label: String::new(),
        }
    }
}

/// Why `sf/meta` can't be read.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MetaError {
    #[error("sf/meta version {found} is newer than this build's {META_VERSION}")]
    TooNew { found: u32 },
    #[error("sf/meta: {0}")]
    Decode(#[from] DecodeError),
}

impl SaveMeta {
    /// Postcard bytes at [`META_VERSION`].
    pub fn encode(&self) -> Vec<u8> {
        encode(self)
    }

    /// Decodes meta stored at `version`.
    pub fn decode(version: u32, bytes: &[u8]) -> Result<Self, MetaError> {
        match version {
            META_VERSION => Ok(decode(bytes)?),
            found => Err(MetaError::TooNew { found }),
        }
    }
}
