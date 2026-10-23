//! Morrowind RPG adapter (`docs/specs/rpg-adapter.md`): builds `sf_rpg`'s dialogue
//! database and the journal table from DIAL/INFO records, evaluates INFO filters and text
//! macros ([`MwDialogueRules`] over a host [`MwView`]), runs result scripts ([`ResultRunner`],
//! the game's [`MwResults`] over a host [`MwWorld`]) and defines Morrowind's equipment slots
//! ([`MwEquipRules`]).

mod db;
mod equip;
mod filter;
mod result;
mod runner;
mod state;
mod text;

pub use db::{
    INFO_REFUSAL, JournalStage, LINK_RULES, MwDialogue, MwJournalDb, MwQuest, response_id,
};
pub use equip::{MwEquipKind, MwEquipRules, MwSlot, SLOT_NAMES, slot_table};
pub use filter::{ActorFacts, MwDialogueRules, MwView, rank_name};
pub use result::{ResultCall, ResultEffect, ResultError, ResultRunner, Target};
pub use runner::{MwResultRunner, MwResults, MwWorld, ResultReport, ResultState, ScriptVar};
pub use state::NpcStates;
pub use text::{GlobalNumber, Macro, MacroSource, expand, format_global};
/// Script variable types ([`ScriptVar`]), from `mw_script`.
pub use mw_script::ast::VarType;

#[cfg(test)]
mod runner_tests;
#[cfg(test)]
mod tests;
