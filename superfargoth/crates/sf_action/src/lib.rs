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
//!
//! Slice B2a: the Warp owner ([`Owner::Warp`]): a rule with a ledge probe ([`ProbeNeed`]) plans an
//! [`sf_anim::WarpPlan`] over its slot chain's root path ([`SlotSource::root`]) at start, keeps it
//! in [`ActionState::warp`] and writes the root pose and chain progress each tick
//! ([`ActionOutput::warp`]); [`TRAVERSAL_RULES`] holds `traverse.mantle.waist` and `traverse.mantle.high`.

mod command;
mod ids;
mod rule;
mod state;
mod step;
mod windows;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_lock;
#[cfg(test)]
mod tests_warp;

pub use command::{
    ActionClass, ButtonEdges, CLASSES, ClassSet, Command, CommandFeed, InputSampler, LockRequest, LockSide, REPEAT_MISSING_TICKS, TapHold,
    ViewDir,
};
pub use ids::{CharacterId, ClockTime, MovesetKey, PlayerId, RuleKey, SlotKey, TICK_HZ, TagId, TargetId, Tick, fnv1a32};
pub use rule::{
    Category, CompiledRule, Entry, Exit, ExitOn, ExitTo, Launch, LoadError, Mode, MoveKind, MoveRule, ObstacleClass, Origin, Owner, Phase,
    Prev, PrevKey, ProbeNeed, RuleSet, RuleSetBuilder, SquareUp, WarpTarget, WindowPolicy,
};
pub use state::{
    ActionClock, ActionState, BodyMemory, CharacterParams, Forced, GaitTable, InputBuffer, ModeProfile, ModeState, RATE_ONE, Timers, WarpState,
};
pub use step::{ActionEvent, ActionOutput, ClipOutput, EventKind, Focus, MAX_SETTLE, Obstacle, Prompt, Refusal, Refused, Sense, StepCtx, WarpOutput, step};
/// The root-path trait a [`SlotSource`] hands Warp moves (hosts implement or box it).
pub use sf_anim::RootPath;
pub use windows::{Gates, HitMask, Located, SlotInfo, SlotSource, SlotTable, Window, WindowKind, gates, locate, open_windows};

/// The framework's base move rules (A1: locomotion, air, landing bands, activation), loaded as
/// [`Origin::Framework`] before content packs.
pub const BASE_RULES: &str = include_str!("base_rules.ron");

/// The framework's traversal rules (B2: the waist and high mantles), added after [`BASE_RULES`] by hosts
/// whose slot source has the traversal slots and whose shell senses obstacles ([`Sense::obstacle`]).
pub const TRAVERSAL_RULES: &str = include_str!("traversal_rules.ron");
