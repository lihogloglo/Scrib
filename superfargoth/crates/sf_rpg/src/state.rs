//! World and character state, commands and events (rpg.md "Commands and events").

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

use crate::equip::Loadout;
use crate::ids::Holder;
use crate::item::{Attrs, InventoryError, StackId};
use crate::quest::{EntryDraft, HeardLine, Journal, QuestLog, QuestState, StageFlags, Topics};

/// Shared, host-owned RPG world state (net.md "Quest model").
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRpg {
    pub quests: QuestLog,
    pub journal: Journal,
    pub topics: Topics,
}

/// Per-character RPG state (inventory and equipment; stats in M7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterRpg {
    pub loadout: Loadout,
}

/// A requested change. The host applies commands on its tick; scripts and UI only produce them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RpgCommand {
    SetStage { quest: RecordId, stage: i32, flags: StageFlags, entry: Option<EntryDraft> },
    AddTopic(RecordId),
    HearLine { topic: RecordId, line: HeardLine },
    AddItem { holder: Holder, item: RecordId, count: u32, attrs: Attrs },
    RemoveItem { holder: Holder, item: RecordId, count: u32 },
    /// Split `count` pieces off a stack into a new stack (UI "split").
    SplitStack { holder: Holder, stack: StackId, count: u32 },
    /// Merge stack `from` into the identical stack `into` (UI "merge", drop onto a stack).
    MergeStacks { holder: Holder, from: StackId, into: StackId },
}

impl RpgCommand {
    /// The inventory an item command targets (`None` for world commands).
    pub fn holder(&self) -> Option<Holder> {
        match self {
            Self::AddItem { holder, .. }
            | Self::RemoveItem { holder, .. }
            | Self::SplitStack { holder, .. }
            | Self::MergeStacks { holder, .. } => Some(*holder),
            _ => None,
        }
    }
}

/// What changed, as data (UI, replication, script events).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RpgEvent {
    StageChanged { quest: RecordId, old: QuestState, new: QuestState },
    JournalAdded { seq: u32, quest: Option<RecordId> },
    TopicLearned(RecordId),
    ItemsChanged { holder: Holder, item: RecordId, delta: i64 },
    /// A stack was split; counts per item are unchanged.
    StackSplit { holder: Holder, source: StackId, new: StackId },
    /// Stack `from` merged into `into`; counts per item are unchanged.
    StacksMerged { holder: Holder, from: StackId, into: StackId },
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RpgError {
    #[error("item commands apply to a character, not the world")]
    NotAWorldCommand,
    #[error("world commands apply to the world, not a character")]
    NotAnItemCommand,
    #[error(transparent)]
    Inventory(#[from] InventoryError),
}

impl WorldRpg {
    /// Applies a world command (quests, journal, topics).
    pub fn apply(&mut self, cmd: RpgCommand) -> Result<Vec<RpgEvent>, RpgError> {
        let mut events = Vec::new();
        match cmd {
            RpgCommand::SetStage { quest, stage, flags, entry } => {
                let change = self.quests.set_stage(quest.clone(), stage, flags);
                events.push(RpgEvent::StageChanged { quest: change.quest, old: change.old, new: change.new });
                if let Some(draft) = entry {
                    let seq = self.journal.add(Some(quest.clone()), Some(stage), draft);
                    events.push(RpgEvent::JournalAdded { seq, quest: Some(quest) });
                }
            }
            RpgCommand::AddTopic(topic) => {
                if self.topics.learn(topic.clone()) {
                    events.push(RpgEvent::TopicLearned(topic));
                }
            }
            RpgCommand::HearLine { topic, line } => {
                let new = !self.topics.knows(&topic);
                self.topics.hear(topic.clone(), line);
                if new {
                    events.push(RpgEvent::TopicLearned(topic));
                }
            }
            RpgCommand::AddItem { .. }
            | RpgCommand::RemoveItem { .. }
            | RpgCommand::SplitStack { .. }
            | RpgCommand::MergeStacks { .. } => return Err(RpgError::NotAWorldCommand),
        }
        Ok(events)
    }
}

impl CharacterRpg {
    /// Applies an item command to this character (the caller resolved its `Holder`).
    /// Atomic: on error nothing changed.
    pub fn apply(&mut self, cmd: RpgCommand) -> Result<Vec<RpgEvent>, RpgError> {
        match cmd {
            RpgCommand::AddItem { holder, item, count, attrs } => {
                self.loadout.add(item.clone(), count, attrs)?;
                Ok(vec![RpgEvent::ItemsChanged { holder, item, delta: i64::from(count) }])
            }
            RpgCommand::RemoveItem { holder, item, count } => {
                self.loadout.remove(&item, count)?;
                Ok(vec![RpgEvent::ItemsChanged { holder, item, delta: -i64::from(count) }])
            }
            RpgCommand::SplitStack { holder, stack, count } => {
                let new = self.loadout.split(stack, count)?;
                Ok(vec![RpgEvent::StackSplit { holder, source: stack, new }])
            }
            RpgCommand::MergeStacks { holder, from, into } => {
                self.loadout.merge(from, into)?;
                Ok(vec![RpgEvent::StacksMerged { holder, from, into }])
            }
            _ => Err(RpgError::NotAnItemCommand),
        }
    }
}
