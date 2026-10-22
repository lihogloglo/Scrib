//! Bevy-free view-models for RPG screens (`docs/specs/journal-ui.md`). Logic only: the
//! widgets, styling and input live in the UI crates.
//!
//! The journal screen over `sf_rpg`'s journal, quest log and known topics:
//!
//! - [`book`]: blocks flowed into pages behind the [`Paginator`] trait; the stock
//!   [`LinePaginator`] takes line breaks from a [`TextMeasure`] and applies orphans, widows and
//!   keep-with-next.
//! - [`QuestRow`] / [`TopicRow`]: `sf_ui` view-models for the quest list (active / finished)
//!   and the topic index with [`Alphabet`] letter tabs.
//! - [`SearchQuery`]: case-insensitive AND search with highlight ranges.
//! - [`JournalSeen`]: per-player "new" markers.
//! - [`JournalScreen`]: the state machine tying them together.

pub mod book;
mod labels;
mod rows;
mod screen;
mod search;
mod seen;

pub use book::{
    Block, BlockKey, BlockStyle, FixedWidthMeasure, LinePaginator, Page, PageMetrics, PageSlice,
    Paginator, TextMeasure,
};
pub use labels::JournalLabels;
pub use rows::{Alphabet, LetterTab, QuestRow, TopicRow, letter_tabs, quest_rows, topic_rows};
pub use screen::{
    BookContent, EntryHit, JournalCtx, JournalScreen, QuestFilter, QuestSort, SearchResults,
};
pub use search::{SearchQuery, highlights_in};
pub use seen::JournalSeen;

#[cfg(test)]
mod tests;
