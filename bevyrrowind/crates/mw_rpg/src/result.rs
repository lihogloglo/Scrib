//! Result-script hand-off (rpg-adapter.md "Result scripts").
//!
//! The seam between dialogue and the script runtime: [`ResultRunner`] and the effect data
//! the host applies. The game's runner is [`MwResults`](crate::MwResults) (`runner.rs`):
//! `mw_script`'s native interpreter while Luau is parked (owner 2026-10-18).

use sf_records::RecordId;
use sf_rpg::ConversationCtx;
use thiserror::Error;

/// The op targets and the ops a run produces live in the game API ([`crate::api`]); a result
/// run's effects are its [`GameOp`](crate::api::GameOp)s.
pub use crate::api::{GameOp as ResultEffect, Target};

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
