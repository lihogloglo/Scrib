//! Quest store, journal and known topics (rpg.md "Quests", "Journal").

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_records::RecordId;

/// A quest's progress. An unknown quest is `QuestState::default()`: stage 0, not finished.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestState {
    pub stage: i32,
    pub finished: bool,
}

/// Flags a stage carries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageFlags {
    /// Reaching this stage finishes the quest.
    pub finish: bool,
    /// Reaching this stage reopens a finished quest.
    pub restart: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestChange {
    pub quest: RecordId,
    pub old: QuestState,
    pub new: QuestState,
}

/// Quest id → progress. Stages may go down as well as up.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestLog {
    quests: BTreeMap<RecordId, QuestState>,
}

impl QuestLog {
    pub fn get(&self, quest: &RecordId) -> QuestState {
        self.quests.get(quest).copied().unwrap_or_default()
    }

    pub fn stage(&self, quest: &RecordId) -> i32 {
        self.get(quest).stage
    }

    pub fn set_stage(&mut self, quest: RecordId, stage: i32, flags: StageFlags) -> QuestChange {
        let progress = self.quests.entry(quest.clone()).or_default();
        let old = *progress;
        progress.stage = stage;
        if flags.finish {
            progress.finished = true;
        } else if flags.restart {
            progress.finished = false;
        }
        QuestChange { quest, old, new: *progress }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RecordId, &QuestState)> {
        self.quests.iter()
    }
}

/// Game time of an entry: days since the game started and minute of the day. The calendar
/// (month names, year) is the adapter's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GameTime {
    pub day: u32,
    pub minute: u16,
}

/// Text for a journal entry or heard line, before it gets its sequence number.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryDraft {
    /// The adapter's entry id (for re-reading localised text).
    pub source: Option<RecordId>,
    /// Snapshot with macros expanded at write time.
    pub text: String,
    pub time: GameTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub seq: u32,
    pub quest: Option<RecordId>,
    pub stage: Option<i32>,
    pub source: Option<RecordId>,
    pub text: String,
    pub time: GameTime,
}

/// The append-only, dated journal.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Journal {
    entries: Vec<JournalEntry>,
    next_seq: u32,
}

impl Journal {
    /// Appends an entry; returns its sequence number.
    pub fn add(&mut self, quest: Option<RecordId>, stage: Option<i32>, draft: EntryDraft) -> u32 {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        self.entries.push(JournalEntry { seq, quest, stage, source: draft.source, text: draft.text, time: draft.time });
        seq
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    pub fn for_quest<'a>(&'a self, quest: &'a RecordId) -> impl Iterator<Item = &'a JournalEntry> + 'a {
        self.entries.iter().filter(move |e| e.quest.as_ref() == Some(quest))
    }

    pub fn last_for_quest(&self, quest: &RecordId) -> Option<&JournalEntry> {
        self.entries.iter().rev().find(|e| e.quest.as_ref() == Some(quest))
    }

    /// Quests with entries, in order of their first entry.
    pub fn quests(&self) -> Vec<RecordId> {
        let mut out: Vec<RecordId> = Vec::new();
        for q in self.entries.iter().filter_map(|e| e.quest.as_ref()) {
            if !out.contains(q) {
                out.push(q.clone());
            }
        }
        out
    }
}

/// A response the player heard under a topic (the journal's topics tab).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeardLine {
    pub source: RecordId,
    pub speaker: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownTopic {
    /// Learning order (a counter; stable across saves).
    pub learned: u32,
    pub heard: Vec<HeardLine>,
}

/// Topics the party knows (world state).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Topics {
    known: BTreeMap<RecordId, KnownTopic>,
    clock: u32,
}

impl Topics {
    /// Learns a topic; `true` the first time.
    pub fn learn(&mut self, topic: RecordId) -> bool {
        if self.known.contains_key(&topic) {
            return false;
        }
        let learned = self.clock;
        self.clock = self.clock.wrapping_add(1);
        self.known.insert(topic, KnownTopic { learned, heard: Vec::new() });
        true
    }

    pub fn knows(&self, topic: &RecordId) -> bool {
        self.known.contains_key(topic)
    }

    /// Records a heard line, learning the topic if needed. A line already heard under the
    /// topic (same source) isn't repeated.
    pub fn hear(&mut self, topic: RecordId, line: HeardLine) {
        self.learn(topic.clone());
        let entry = self.known.get_mut(&topic).expect("learned above");
        if !entry.heard.iter().any(|h| h.source == line.source) {
            entry.heard.push(line);
        }
    }

    pub fn get(&self, topic: &RecordId) -> Option<&KnownTopic> {
        self.known.get(topic)
    }

    /// Known topics in learning order.
    pub fn in_learned_order(&self) -> Vec<&RecordId> {
        let mut v: Vec<(&RecordId, u32)> = self.known.iter().map(|(k, t)| (k, t.learned)).collect();
        v.sort_by_key(|(_, l)| *l);
        v.into_iter().map(|(k, _)| k).collect()
    }

    pub fn len(&self) -> usize {
        self.known.len()
    }

    pub fn is_empty(&self) -> bool {
        self.known.is_empty()
    }
}
