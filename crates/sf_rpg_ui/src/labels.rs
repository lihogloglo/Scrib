//! Display text the content pack supplies (journal-ui.md "Data model").

use sf_rpg::{JournalEntry, RecordId};

/// Adapter hook: names, dates and localised text for the journal screen.
pub trait JournalLabels {
    /// A quest's display name; `None` shows the id.
    fn quest_name(&self, quest: &RecordId) -> Option<String>;

    /// A topic's display name; `None` shows the id.
    fn topic_name(&self, topic: &RecordId) -> Option<String>;

    /// Heading for a day (days since the game start; the calendar is the adapter's).
    fn day_heading(&self, day: u32) -> String;

    /// The entry's text re-rendered in the current language; `None` (the default) shows the
    /// snapshot taken when the entry was written (rpg.md open question 2).
    fn entry_text(&self, entry: &JournalEntry) -> Option<String> {
        let _ = entry;
        None
    }
}

pub(crate) fn quest_name(labels: &dyn JournalLabels, quest: &RecordId) -> String {
    labels
        .quest_name(quest)
        .unwrap_or_else(|| quest.as_str().to_owned())
}

pub(crate) fn topic_name(labels: &dyn JournalLabels, topic: &RecordId) -> String {
    labels
        .topic_name(topic)
        .unwrap_or_else(|| topic.as_str().to_owned())
}

pub(crate) fn entry_text(labels: &dyn JournalLabels, entry: &JournalEntry) -> String {
    labels
        .entry_text(entry)
        .unwrap_or_else(|| entry.text.clone())
}
