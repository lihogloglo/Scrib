//! World and character state, commands and events (rpg.md "Commands and events").

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

use crate::equip::Loadout;
use crate::ids::Holder;
use crate::item::{AttrValue, Attrs, InventoryError, StackId};
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
    /// Take `count` pieces of a stack and add them to another holder, merging into an
    /// identical stack there (trade settlement, the container screen). Spans two characters:
    /// apply with [`CharacterRpg::apply_move`].
    MoveStack { from: Holder, to: Holder, stack: StackId, count: u32 },
    /// Set (`Some`) or clear (`None`) one attribute of a stack, then re-merge it.
    SetStackAttr { holder: Holder, stack: StackId, key: RecordId, value: Option<AttrValue> },
}

impl RpgCommand {
    /// The inventory an item command targets (`None` for world commands). For a
    /// [`RpgCommand::MoveStack`], the source; see [`RpgCommand::target`].
    pub fn holder(&self) -> Option<Holder> {
        match self {
            Self::AddItem { holder, .. }
            | Self::RemoveItem { holder, .. }
            | Self::SplitStack { holder, .. }
            | Self::MergeStacks { holder, .. }
            | Self::SetStackAttr { holder, .. }
            | Self::MoveStack { from: holder, .. } => Some(*holder),
            _ => None,
        }
    }

    /// The second inventory of a [`RpgCommand::MoveStack`] (`None` for every other command).
    pub fn target(&self) -> Option<Holder> {
        match self {
            Self::MoveStack { to, .. } => Some(*to),
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
    /// A stack's attributes changed; `stack` now holds its pieces (when it re-merged, a
    /// `StacksMerged` from the old id comes first).
    StackChanged { holder: Holder, stack: StackId },
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RpgError {
    #[error("item commands apply to a character, not the world")]
    NotAWorldCommand,
    #[error("world commands apply to the world, not a character")]
    NotAnItemCommand,
    #[error("a stack move spans two characters: use CharacterRpg::apply_move")]
    TwoHolders,
    #[error("CharacterRpg::apply_move takes only MoveStack")]
    NotAMove,
    #[error("a stack move needs two different holders")]
    SameHolder,
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
            | RpgCommand::MergeStacks { .. }
            | RpgCommand::MoveStack { .. }
            | RpgCommand::SetStackAttr { .. } => return Err(RpgError::NotAWorldCommand),
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
            RpgCommand::SetStackAttr { holder, stack, key, value } => {
                let now = self.loadout.set_attr(stack, key, value)?;
                let mut events = Vec::with_capacity(2);
                if now != stack {
                    events.push(RpgEvent::StacksMerged { holder, from: stack, into: now });
                }
                events.push(RpgEvent::StackChanged { holder, stack: now });
                Ok(events)
            }
            RpgCommand::MoveStack { .. } => Err(RpgError::TwoHolders),
            _ => Err(RpgError::NotAnItemCommand),
        }
    }

    /// Applies a [`RpgCommand::MoveStack`] from `source` to `target` (the caller resolved both
    /// holders). Atomic: the target is checked before the source changes.
    pub fn apply_move(source: &mut Self, target: &mut Self, cmd: RpgCommand) -> Result<Vec<RpgEvent>, RpgError> {
        let RpgCommand::MoveStack { from, to, stack, count } = cmd else {
            return Err(RpgError::NotAMove);
        };
        if from == to {
            return Err(RpgError::SameHolder);
        }
        let s = source.loadout.inventory().get(stack).ok_or(InventoryError::NoStack(stack))?;
        if count == 0 {
            return Err(InventoryError::ZeroCount.into());
        }
        if count > s.count {
            return Err(InventoryError::NotEnough { item: s.item.clone(), have: u64::from(s.count), want: count }.into());
        }
        target.loadout.can_add(&s.item, count, &s.attrs)?;
        let taken = source.loadout.take(stack, count)?;
        let item = taken.item.clone();
        target.loadout.add(taken.item, taken.count, taken.attrs)?;
        Ok(vec![
            RpgEvent::ItemsChanged { holder: from, item: item.clone(), delta: -i64::from(count) },
            RpgEvent::ItemsChanged { holder: to, item, delta: i64::from(count) },
        ])
    }
}
