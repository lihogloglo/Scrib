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
use sf_world::Location;

use super::{CHARACTER, OWNER};

/// Owner-versioned bytes: per-character data of the adapter or a mod (stats until M7,
/// appearance choices, a mod's per-character state). Each owner migrates its own blob.
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
