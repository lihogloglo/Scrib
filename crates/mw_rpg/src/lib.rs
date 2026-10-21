//! Morrowind RPG adapter (`docs/specs/rpg-adapter.md`): builds `sf_rpg`'s dialogue
//! database and the journal table from DIAL/INFO records, evaluates INFO filters and text
//! macros ([`MwDialogueRules`] over a host [`MwView`]), defines the result-script seam
//! ([`ResultRunner`]) and Morrowind's equipment slots ([`MwEquipRules`]).

mod db;
mod equip;
mod filter;
mod result;
mod state;
mod text;

pub use db::{
    INFO_REFUSAL, JournalStage, LINK_RULES, MwDialogue, MwJournalDb, MwQuest, response_id,
};
pub use equip::{MwEquipKind, MwEquipRules, MwSlot, SLOT_NAMES, slot_table};
pub use filter::{ActorFacts, MwDialogueRules, MwView, rank_name};
pub use result::{ResultCall, ResultEffect, ResultError, ResultRunner, Target};
pub use state::NpcStates;
pub use text::{GlobalNumber, Macro, MacroSource, expand, format_global};

#[cfg(test)]
mod tests;
