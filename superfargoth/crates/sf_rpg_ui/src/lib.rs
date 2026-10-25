//! Bevy-free view-models for the RPG screens over `sf_rpg` state. Logic only: the widgets,
//! styling and input live in the UI crates.
//!
//! Game state lives in `sf_rpg`; screens never read it. This crate turns that state into
//! what a screen binds to (MVVM, ui.md "Canonical technique") and turns player input back
//! into requests.
//!
//! Dialogue window (ui.md "Dialogue window view-model"):
//!
//! - [`DialogueView`]: the dialogue window over a `sf_rpg::DialogueSession`: scrollback of
//!   lines split into hyperlinked runs, the sorted topic list (new vs known), choices,
//!   goodbye, journal-updated notices, scroll requests and keyboard/gamepad focus. Input
//!   ([`DialogueNav`], pointer activation) yields [`DialogueRequest`]s the host applies to
//!   the session; the view never mutates game state (net-ready: the same requests travel
//!   from a client).
//!
//! Journal screen (`docs/specs/journal-ui.md`), over `sf_rpg`'s journal, quest log and known
//! topics:
//!
//! - [`book`]: blocks flowed into pages behind the [`Paginator`] trait; the stock
//!   [`LinePaginator`] takes line breaks from a [`TextMeasure`] and applies orphans, widows and
//!   keep-with-next.
//! - [`QuestRow`] / [`TopicIndexRow`]: `sf_ui` view-models for the quest list (active /
//!   finished) and the topic index with [`Alphabet`] letter tabs.
//! - [`SearchQuery`]: case-insensitive AND search with highlight ranges.
//! - [`JournalSeen`]: per-player "new" markers.
//! - [`JournalScreen`]: the state machine tying them together.
//!
//! No rendering, no Bevy, no clock: the Bevy screens (`sf_ui_screens`) bind these values to
//! nodes, map their `UiAction`s to navigation and time notices themselves.

pub mod book;
mod dialogue;
mod labels;
mod rows;
mod screen;
mod search;
mod seen;

#[cfg(test)]
mod tests_dialogue;
#[cfg(test)]
mod tests_journal;

pub use book::{
    Block, BlockKey, BlockStyle, FixedWidthMeasure, LinePaginator, Page, PageMetrics, PageSlice,
    Paginator, TextMeasure,
};
pub use dialogue::{
    DEFAULT_MAX_ENTRIES, DialogueFocus, DialogueMode, DialogueNav, DialogueRequest, DialogueView,
    Entry, EntryId, EntryKind, LinkState, TextRun, TopicLink, TopicRow, ViewLine,
};
pub use labels::JournalLabels;
pub use rows::{
    Alphabet, LetterTab, QuestRow, TopicIndexRow, letter_tabs, quest_rows, topic_rows,
};
pub use screen::{
    BookContent, EntryHit, JournalCtx, JournalScreen, QuestFilter, QuestSort, SearchResults,
};
pub use search::{SearchQuery, highlights_in};
pub use seen::JournalSeen;
