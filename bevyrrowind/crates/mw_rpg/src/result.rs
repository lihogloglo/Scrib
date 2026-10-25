//! Result-script hand-off (rpg-adapter.md "Result scripts").
//!
//! The seam between dialogue and the script runtime: [`ResultRunner`] and the effect data
//! the host applies. The game's runner is [`MwResults`](crate::MwResults) (`runner.rs`):
//! `mw_script`'s native interpreter while Luau is parked (owner 2026-10-18).

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::{Choice, ConversationCtx};
use thiserror::Error;

/// Whose state an effect targets (`Player->`, `"arrille"->`, or implicit = the speaker).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Target {
    Speaker,
    /// The context player (`ctx.player`).
    Player,
    /// A unique actor by base id; the host resolves it to an `ActorKey`.
    Actor(RecordId),
}

/// What a result script asked for, in call order. The host applies these on its tick:
/// journal through [`MwDialogue::journal_command`](crate::MwDialogue::journal_command),
/// items as `sf_rpg` commands, choices and goodbye on the session, dispositions in
/// [`NpcStates`](crate::NpcStates), locals and globals in the script runtime, message boxes
/// to the UI, sounds to audio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResultEffect {
    Journal {
        quest: RecordId,
        index: i32,
    },
    SetJournalIndex {
        quest: RecordId,
        index: i32,
    },
    AddTopic(RecordId),
    AddItem {
        target: Target,
        item: RecordId,
        count: u32,
    },
    RemoveItem {
        target: Target,
        item: RecordId,
        count: u32,
    },
    Choices(Vec<Choice>),
    Goodbye,
    ModDisposition {
        target: Target,
        delta: i32,
    },
    SetDisposition {
        target: Target,
        value: i32,
    },
    /// A script local of the speaker or another object, already converted to its type.
    SetLocal {
        target: Target,
        name: RecordId,
        value: f32,
    },
    /// A global (GLOB), already converted to its type.
    SetGlobal {
        name: RecordId,
        value: f32,
    },
    /// `MessageBox`: formatted text; no buttons = a notification, else a modal.
    MessageBox {
        text: String,
        buttons: Vec<String>,
    },
    /// `PlaySound`: a sound record id (2D, at the listener).
    PlaySound(RecordId),
}

/// One result to run: the INFO's script text as the speaker, context player `ctx.player`.
#[derive(Clone, Copy, Debug)]
pub struct ResultCall<'a> {
    pub ctx: &'a ConversationCtx,
    pub topic: &'a RecordId,
    pub response: &'a RecordId,
    pub script: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResultError {
    #[error("result of {response}: {message}")]
    Failed { response: RecordId, message: String },
}

/// Runs result scripts. The game's implementation is [`MwResults`](crate::MwResults)'
/// runner; tests may use doubles.
pub trait ResultRunner {
    fn run(&mut self, call: ResultCall<'_>) -> Result<Vec<ResultEffect>, ResultError>;
}
