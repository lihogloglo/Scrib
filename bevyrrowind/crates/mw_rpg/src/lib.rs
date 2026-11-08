//! Morrowind RPG adapter (`docs/specs/rpg-adapter.md`): builds `sf_rpg`'s dialogue
//! database and the journal table from DIAL/INFO records, evaluates INFO filters and text
//! macros ([`MwDialogueRules`] over a host [`MwView`]), runs result scripts ([`ResultRunner`],
//! the game's [`MwResults`] over a host [`MwWorld`]) and defines Morrowind's equipment slots
//! ([`MwEquipRules`]). The PC stat filter functions read the host's sheets ([`MwSheets`]).

mod barter;
mod db;
mod disposition;
mod equip;
mod filter;
mod persuasion;
mod result;
mod runner;
mod script_stats;
mod services;
mod state;
mod stats;
mod text;

pub use barter::{
    ALL_ITEMS, ATTR_HEALTH, ATTR_RESTOCK, BarterOptions, BoundItems, GOLD_IDS, MwItemClass, MwService, MwServiceRules,
    MwTradeHost, ProviderFacts, SERVICE_REFUSAL_TOPIC, TradeItem, can_sell, is_gold, is_restocking, item_health,
    provider_facts, record_entry_stack, trade_item,
};
pub use db::{
    INFO_REFUSAL, JournalStage, LINK_RULES, MwDialogue, MwJournalDb, MwQuest, response_id,
};
pub use disposition::{
    DispositionExtras, DispositionInput, PlayerFaction, derived_disposition, derived_disposition_unclamped,
    disposition_input, faction_reaction, faction_standing,
};
pub use equip::{MwEquipKind, MwEquipRules, MwSlot, SLOT_NAMES, clothing_value, record_clothing_value, slot_table};
pub use filter::{ActorFacts, MwDialogueRules, MwView, rank_name};
pub use persuasion::{
    AiNudge, PersuaderStats, PersuasionInput, PersuasionKind, PersuasionOutcome, PersuasionRatings, PersuasionSession,
    persuader_stats, persuasion_change, persuasion_input, persuasion_ratings, persuasion_roll, session_perm_change,
    speechcraft_use,
};
pub use result::{ResultCall, ResultEffect, ResultError, ResultRunner, Target};
pub use runner::{MwResultRunner, MwResults, MwWorld, ResultReport, ResultState, ScriptVar};
pub use services::{
    BarterPriceIn, BarterTerms, EffectiveValueIn, FollowerPricing, HaggleIn, HaggleOut, PartyTerms, RULE_BARTER_HAGGLE,
    RULE_BARTER_PRICE, RULE_IDS, RULE_REPAIR_PRICE, RULE_SPELL_PRICE, RULE_TRAINING, RULE_TRAVEL, RepairPriceIn,
    SpellPriceIn, TrainingIn, TrainingOut, TrainingRefusal, TravelIn, TravelOut, barter_haggle, barter_offer,
    barter_price, effective_value, repair_price, spell_price, training_price, travel_price,
};
pub use state::NpcStates;
pub use stats::{MwSheets, NoSheets, player_rank, sheet_function};
pub use text::{GlobalNumber, Macro, MacroSource, expand, format_global};
/// Script variable types ([`ScriptVar`]), from `mw_script`.
pub use mw_script::ast::VarType;

#[cfg(test)]
mod runner_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_barter;
#[cfg(test)]
mod tests_disposition;
#[cfg(test)]
mod tests_script_stats;
#[cfg(test)]
mod tests_services;
#[cfg(test)]
mod tests_stats;
