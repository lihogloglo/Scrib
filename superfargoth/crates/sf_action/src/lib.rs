//! Bevy-free action layer (`docs/specs/action.md`): one system picks what every character does
//! next, player and NPC alike, from data move rules on the fixed tick.
//!
//! Slice A1 (backbone):
//! - [`Command`]: the only input, one per character per tick; [`InputSampler`] turns frame-rate
//!   device edges into commands, [`CommandFeed`] yields a remote player's commands on the host.
//! - [`MoveRule`] records (RON) compiled into a priority-ordered [`RuleSet`] with load errors
//!   naming the mod; [`BASE_RULES`] holds the framework's locomotion, air, landing and
//!   activation rules.
//! - [`step`]: the chooser. Sekiro's two-stage buffer (latch in `accept`, fire in `cancel`, no
//!   timer), jump buffer and coyote time in ticks, forced landings by impact band, a bounded
//!   settle loop, the Sim owner's [`sf_controller::LocomotionIntent`], and the "press to use"
//!   [`Prompt`] / [`EventKind::Activate`] hook.
//! - [`ActionState`]: plain serde, integer clocks, stable ids, no singleton player
//!   (net.md rules 1-8).

mod command;
mod ids;
mod rule;
mod state;
mod step;
mod windows;

#[cfg(test)]
mod tests;

pub use command::{
    ActionClass, ButtonEdges, CLASSES, ClassSet, Command, CommandFeed, InputSampler, LockRequest, LockSide, REPEAT_MISSING_TICKS,
    ViewDir,
};
pub use ids::{CharacterId, ClockTime, PlayerId, RuleKey, SlotKey, TICK_HZ, TagId, TargetId, Tick, fnv1a32};
pub use rule::{
    Category, CompiledRule, Entry, Exit, ExitOn, ExitTo, Launch, LoadError, Mode, MoveKind, MoveRule, Origin, Owner, Phase, RuleSet,
    RuleSetBuilder, WindowPolicy,
};
pub use state::{
    ActionClock, ActionState, BodyMemory, CharacterParams, GaitTable, InputBuffer, ModeProfile, ModeState, RATE_ONE, Timers,
};
pub use step::{ActionEvent, ActionOutput, EventKind, Focus, MAX_SETTLE, Prompt, Refusal, Refused, Sense, StepCtx, step};
pub use windows::{Gates, Located, SlotInfo, SlotSource, SlotTable, Window, WindowKind, gates, locate};

/// The framework's base move rules (A1: locomotion, air, landing bands, activation), loaded as
/// [`Origin::Framework`] before content packs.
pub const BASE_RULES: &str = include_str!("base_rules.ron");
