//! Bevy-free view-models for the RPG screens (ui.md "Dialogue window view-model").
//!
//! Game state lives in `sf_rpg`; screens never read it. This crate turns that state into
//! what a screen binds to (MVVM, ui.md "Canonical technique") and turns player input back
//! into requests:
//!
//! - [`DialogueView`]: the dialogue window over a `sf_rpg::DialogueSession`: scrollback of
//!   lines split into hyperlinked runs, the sorted topic list (new vs known), choices,
//!   goodbye, journal-updated notices, scroll requests and keyboard/gamepad focus. Input
//!   ([`DialogueNav`], pointer activation) yields [`DialogueRequest`]s the host applies to
//!   the session; the view never mutates game state (net-ready: the same requests travel
//!   from a client).
//!
//! No rendering, no Bevy, no clock: the Bevy screen (`sf_ui_screens`) binds these values
//! to nodes, maps its `UiAction`s to [`DialogueNav`] and times notices itself.

mod dialogue;

#[cfg(test)]
mod tests;

pub use dialogue::{
    DEFAULT_MAX_ENTRIES, DialogueFocus, DialogueMode, DialogueNav, DialogueRequest, DialogueView,
    Entry, EntryId, EntryKind, LinkState, TextRun, TopicLink, TopicRow, ViewLine,
};
