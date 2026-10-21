//! Result-script hand-off (rpg-adapter.md "Result scripts").
//!
//! The game runs INFO result scripts in the one script runtime (DECISIONS 2026-10-13): the
//! W1 Luau translation in `sf_script`, with an `mw` library that turns calls into
//! [`ResultEffect`]s (W3). This module is only the seam: [`ResultRunner`] and the effect
//! data the host applies. No interpreter lives here.

use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_rpg::{Choice, ConversationCtx};
use thiserror::Error;

/// Whose state an effect targets (`Player->`, `"arrille"->`, or implicit = the speaker).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
/// [`NpcStates`](crate::NpcStates), locals in the script runtime.
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
    SetLocal {
        target: Target,
        name: RecordId,
        value: f32,
    },
    /// A known function with no effect yet (`ModPCFacRep`, `PCRaiseRank`, ...: M7).
    Stub(String),
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

/// Runs result scripts. The game's implementation is the Luau runtime (W3); tests use
/// doubles.
pub trait ResultRunner {
    fn run(&mut self, call: ResultCall<'_>) -> Result<Vec<ResultEffect>, ResultError>;
}
