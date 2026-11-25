//! Bevy-free interaction core (`superfargoth/docs/specs/interaction.md` "The shared core").
//!
//! - [`def`]: smart-object definitions (`sf.interact.def` records): slots and their kinds, verbs,
//!   entrances, carry and destruction data.
//! - [`claims`]: the host's claim table: priority with preemption of Reserved claims only, queue
//!   tickets, leases, exclusion groups, one body hold per user plus hand claims, typed refusals,
//!   invalidation reasons and the sorted same-tick batch.
//! - [`params`]: the core's tunables as plain fields (`interact.*`, bound by `sf_interaction_bevy`).
//!
//! Never names a game, `sf_ai` or `sf_crime`.

pub mod claims;
pub mod def;
pub mod params;

pub use claims::{
    Claim, Claims, EventKind, Granted, Hold, InteractionEvent, InvalidReason, Op, Phase, Refusal, Request, SlotRef,
    SlotTerms, Subject, Ticket, Tick, UserKey,
};
pub use def::{
    BreakAction, CarryClass, CarryDef, ClipFamily, Control, DestructibleDef, DoorDef, Entrance, Frame, Grip,
    IkTargets, InteractableDef, LadderDef, LeverDef, LeverLink, Occupancy, Side, SlotDef, SlotKind, Stage, TagFilter,
    TagId, VerbDef, WorkstationDef, fnv1a32,
};
pub use params::InteractParams;

#[cfg(test)]
mod tests_claims;
