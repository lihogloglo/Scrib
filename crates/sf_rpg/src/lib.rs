//! Bevy-free RPG primitives (`docs/specs/rpg.md`): item stacks and inventories, data-driven
//! equipment, a quest store, the journal and known topics, and the dialogue session state
//! machine. No rules and no content format: filters, text macros, formulas and scripts come
//! from the content pack through the adapter hooks ([`DialogueRules`], [`EquipRules`],
//! [`LinkRules`]). Everything saved or sent is plain serde data keyed by stable ids.

mod dialogue;
mod equip;
mod ids;
mod item;
mod link;
mod quest;
mod state;

pub use dialogue::{
    Choice, ConversationCtx, Conversations, DialogueDb, DialogueError, DialogueOutput, DialogueRules,
    DialogueSession, SessionState, ShownLine, TopicEntry, TopicKind,
};
pub use equip::{EquipChange, EquipPlacement, EquipRules, Loadout, MAX_SLOTS, SlotId, SlotTable, SlotTableError};
pub use ids::{ActorKey, Holder, MAX_PLAYERS, PlayerId, PlayerKey};
pub use item::{AttrValue, Attrs, Inventory, InventoryError, ItemStack, StackId};
pub use link::{LinkError, LinkRules, TopicLinker, TopicSpan};
pub use quest::{
    EntryDraft, GameTime, HeardLine, Journal, JournalEntry, KnownTopic, QuestChange, QuestLog, QuestState, StageFlags,
    Topics,
};
pub use sf_records::RecordId;
pub use state::{CharacterRpg, RpgCommand, RpgError, RpgEvent, WorldRpg};

#[cfg(test)]
mod tests;
