//! `sf/character/<player key>`: one player's character (save.md "`CharacterSave`").
//!
//! One section per player who ever played in the world, so a guest's character file can carry
//! the exact same bytes (S4). Positions are `Location`s (net rule 7); the body is the gameplay
//! state `sf_action` and `sf_controller` call "saved".

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_action::ActionState;
use sf_controller::Locomotion;
use sf_rpg::{CharacterRpg, PlayerKey};
use sf_save::SectionKey;
use sf_stats::SheetSave;
use sf_world::Location;

use super::{CHARACTER, OWNER};

/// Owner-versioned bytes: per-character data of the adapter or a mod (appearance choices, a
/// mod's per-character state). Each owner migrates its own blob.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedBlob {
    pub version: u32,
    pub bytes: Vec<u8>,
}

/// The gameplay body (action.md: "saved"): the action state machine and the mover state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterBody {
    pub action: ActionState,
    pub locomotion: Locomotion,
}

/// One player's character in this world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterSave {
    pub player: PlayerKey,
    pub name: String,
    /// Where the character stands (feet).
    pub location: Location,
    /// Heading, radians about +y (0 = -z).
    pub facing: f32,
    /// `None` until the body exists (a character between creation and first spawn).
    pub body: Option<CharacterBody>,
    /// Inventory and equipment.
    pub rpg: CharacterRpg,
    /// The stats sheet (stats.md "Saves": values, pools, counters, groups by `StatKey`; no
    /// modifiers, their sources re-add them). `None`: built by the content on first spawn.
    pub stats: Option<SheetSave>,
    /// Extension blobs by owner (`mw`, a mod id).
    pub ext: BTreeMap<String, VersionedBlob>,
}

/// `sf/character/<32 hex digits of the player key>`.
pub fn character_key(player: PlayerKey) -> SectionKey {
    SectionKey::new(OWNER, format!("{CHARACTER}/{}", sf_save::id_hex(player.0))).expect("valid key")
}

/// The player key a character section is named for.
pub fn player_of(key: &SectionKey) -> Option<PlayerKey> {
    let hex = key.instance()?;
    (hex.len() == 32).then(|| u128::from_str_radix(hex, 16).ok().map(PlayerKey)).flatten()
}

/// Frozen older shapes of `sf/character` and their typed steps (save.md "Versioning and
/// migration"). The types keep their original names so their serde shapes match the old goldens
/// (`tests/shapes/sf.character.v<n>.json`, checked in `tests_formats.rs`).
pub mod legacy {
    /// `sf.character.v1` (S2, until combat C1a): `ActionState` without `forced` and `lock`,
    /// `ModeState` without `moveset`. The unchanged leaves (`ActionClock`, `Timers`,
    /// `Locomotion`, ...) are still the live types; the v1 shape golden catches a change to them.
    pub mod v1 {
        use std::collections::BTreeMap;

        use serde::{Deserialize, Serialize};
        use sf_action::{ActionClock, BodyMemory, InputBuffer, Mode, RuleKey, Tick, Timers};
        use sf_controller::Locomotion;
        use sf_rpg::{CharacterRpg, PlayerKey};
        use sf_world::Location;

        use super::super::VersionedBlob;

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ModeState {
            pub mode: Mode,
            pub combat: bool,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ActionState {
            pub mov: RuleKey,
            pub started: Tick,
            pub clock: ActionClock,
            pub mode: ModeState,
            pub buffer: InputBuffer,
            pub timers: Timers,
            pub body: BodyMemory,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct CharacterBody {
            pub action: ActionState,
            pub locomotion: Locomotion,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct CharacterSave {
            pub player: PlayerKey,
            pub name: String,
            pub location: Location,
            pub facing: f32,
            pub body: Option<CharacterBody>,
            pub rpg: CharacterRpg,
            pub ext: BTreeMap<String, VersionedBlob>,
        }

        impl CharacterSave {
            /// v1 → v2: no moveset (the shell sets it from equipment on the next tick), no forced
            /// reaction, no lock-on target.
            pub fn migrate(self) -> super::v2::CharacterSave {
                let body = self.body.map(|b| {
                    let a = b.action;
                    super::super::CharacterBody {
                        action: sf_action::ActionState {
                            mov: a.mov,
                            started: a.started,
                            clock: a.clock,
                            mode: sf_action::ModeState { mode: a.mode.mode, combat: a.mode.combat, moveset: None },
                            buffer: a.buffer,
                            timers: a.timers,
                            body: a.body,
                            warp: None,
                            forced: None,
                            lock: None,
                        },
                        locomotion: b.locomotion,
                    }
                });
                super::v2::CharacterSave {
                    player: self.player,
                    name: self.name,
                    location: self.location,
                    facing: self.facing,
                    body,
                    rpg: self.rpg,
                    ext: self.ext,
                }
            }
        }
    }

    /// `sf.character.v2` (combat C1a until stats S4): no stats sheet. The leaves are the live
    /// types; the v2 shape golden catches a change to them.
    pub mod v2 {
        use std::collections::BTreeMap;

        use serde::{Deserialize, Serialize};
        use sf_rpg::{CharacterRpg, PlayerKey};
        use sf_world::Location;

        use super::super::{CharacterBody, VersionedBlob};

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct CharacterSave {
            pub player: PlayerKey,
            pub name: String,
            pub location: Location,
            pub facing: f32,
            pub body: Option<CharacterBody>,
            pub rpg: CharacterRpg,
            pub ext: BTreeMap<String, VersionedBlob>,
        }

        impl CharacterSave {
            /// v2 → v3: no sheet; the content builds it on first spawn (the new-game path,
            /// stats.md "Saves").
            pub fn migrate(self) -> super::super::CharacterSave {
                super::super::CharacterSave {
                    player: self.player,
                    name: self.name,
                    location: self.location,
                    facing: self.facing,
                    body: self.body,
                    rpg: self.rpg,
                    stats: None,
                    ext: self.ext,
                }
            }
        }
    }
}
