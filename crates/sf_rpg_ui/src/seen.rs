//! Per-player "new" markers (journal-ui.md "`JournalSeen`").

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sf_rpg::{RecordId, WorldRpg};

/// What one player has seen of the journal. Presentation state: each client keeps its own
/// with the player's profile or save; the host never needs it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalSeen {
    /// Every entry below this sequence number has been seen.
    entries_below: u32,
    /// Entries seen at or above the watermark (pages viewed out of order).
    entries_above: BTreeSet<u32>,
    /// Heard lines seen per opened topic.
    topics: BTreeMap<RecordId, usize>,
}

impl JournalSeen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entry_seen(&self, seq: u32) -> bool {
        seq < self.entries_below || self.entries_above.contains(&seq)
    }

    pub fn mark_entry(&mut self, seq: u32) {
        if self.entry_seen(seq) {
            return;
        }
        self.entries_above.insert(seq);
        while self.entries_below < u32::MAX && self.entries_above.remove(&self.entries_below) {
            self.entries_below += 1;
        }
    }

    /// `true` until the topic is opened, and again when a line is heard after that.
    pub fn topic_new(&self, topic: &RecordId, heard: usize) -> bool {
        self.topics.get(topic).is_none_or(|&seen| seen < heard)
    }

    /// Heard lines seen under `topic`; `None` if it was never opened.
    pub fn topic_heard_seen(&self, topic: &RecordId) -> Option<usize> {
        self.topics.get(topic).copied()
    }

    pub fn mark_topic(&mut self, topic: &RecordId, heard: usize) {
        let seen = self.topics.entry(topic.clone()).or_default();
        *seen = (*seen).max(heard);
    }

    /// Entries in `world` not seen yet.
    pub fn unseen_entries(&self, world: &WorldRpg) -> usize {
        world
            .journal
            .entries()
            .iter()
            .filter(|e| !self.entry_seen(e.seq))
            .count()
    }

    /// Known topics that are new.
    pub fn new_topics(&self, world: &WorldRpg) -> usize {
        world
            .topics
            .in_learned_order()
            .into_iter()
            .filter(|t| self.topic_new(t, world.topics.get(t).map_or(0, |k| k.heard.len())))
            .count()
    }

    /// "Mark all read".
    pub fn mark_all_seen(&mut self, world: &WorldRpg) {
        for e in world.journal.entries() {
            self.mark_entry(e.seq);
        }
        for t in world.topics.in_learned_order() {
            let heard = world.topics.get(t).map_or(0, |k| k.heard.len());
            self.mark_topic(t, heard);
        }
    }
}
