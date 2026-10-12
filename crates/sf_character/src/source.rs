//! The adapter interface: game records in, character recipes and rig maps out.

use sf_records::RecordId;
use sf_vfs::VfsPath;

use crate::records::{CharacterRecipe, HeldItem, WearableRef};
use crate::rig::RigMap;

/// Runtime state layered over a base record: what the character wears and
/// holds now. Owned by gameplay (`sf_rpg` later; a component in M3).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharacterState {
    /// Equipped items in equip order. `None` keeps the base record's own
    /// equipment; `Some` replaces it.
    pub worn: Option<Vec<WearableRef>>,
    /// Held items (weapon, shield, torch). `None` keeps the adapter's
    /// default choice for the base record; `Some` replaces it.
    pub held: Option<Vec<HeldItem>>,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum CharacterError {
    #[error("no character record {kind}:{id}")]
    UnknownBase { kind: String, id: RecordId },
    #[error("missing {kind} record {id}")]
    MissingRecord { kind: &'static str, id: RecordId },
    #[error("{0}")]
    Adapter(String),
}

/// Implemented by adapters (Bevy-free), like `WorldSource`.
pub trait CharacterSource: Send + Sync {
    /// Resolves a base record (an NPC, a creature: its record kind and id)
    /// plus its runtime state into a recipe. Part and wearable records the
    /// recipe names are native kinds the adapter defined at load.
    fn recipe(&self, kind: &str, id: &RecordId, state: &CharacterState) -> Result<CharacterRecipe, CharacterError>;

    /// One rig map per skeleton the adapter's recipes use, keyed by
    /// [`crate::RigMapRef`] (usually the skeleton path). Skeletons without
    /// an entry are auto-mapped.
    fn rig_maps(&self) -> Vec<(VfsPath, RigMap)>;
}
