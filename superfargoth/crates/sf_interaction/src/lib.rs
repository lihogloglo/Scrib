//! Bevy-free interaction core (`superfargoth/docs/specs/interaction.md` "The shared core").
//!
//! - [`def`]: smart-object definitions (`sf.interact.def` records): slots and their kinds, verbs,
//!   entrances, carry and destruction data.
//! - [`claims`]: the host's claim table: priority with preemption of Reserved claims only, queue
//!   tickets, leases, exclusion groups, one body hold per user plus hand claims, typed refusals,
//!   invalidation reasons and the sorted same-tick batch.
//! - [`index`]: the point index of resident slots (grid buckets) and `find_slots`.
//! - [`verbs`]: `rank_verbs` (the uses an object offers now) and the per-player `FocusStabilizer`.
//! - [`door`]: the swing-door state machine (toggle, step, leaf pose, close-behind schedule).
//! - [`entrance`]: `pick_entrance` / `pick_exit` (side order, the shell's validity checks).
//! - [`cook`]: `interact.cook`, the one-time def bake (stub).
//! - [`params`]: the core's tunables as plain fields (`interact.*`, bound by `sf_interaction_bevy`).
//!
//! Never names a game, `sf_ai` or `sf_crime`.

pub mod claims;
pub mod cook;
pub mod def;
pub mod door;
pub mod entrance;
pub mod index;
pub mod params;
pub mod verbs;

pub use claims::{
    Claim, Claims, EventKind, Granted, Hold, InteractionEvent, InvalidReason, Op, Phase, Refusal, Request, SlotRef,
    SlotTerms, Subject, Ticket, Tick, UserKey,
};
pub use def::{
    BreakAction, CarryClass, CarryDef, ClipFamily, Control, DestructibleDef, DoorDef, Entrance, Frame, Grip,
    IkTargets, InteractableDef, LadderDef, LeverDef, LeverLink, Occupancy, Side, SlotDef, SlotKind, Stage, TagFilter,
    TagId, VerbDef, WorkstationDef, fnv1a32,
};
pub use cook::cook;
pub use door::{DOOR_OPEN, DoorChanged, DoorState, swing_sign};
pub use entrance::{pick_entrance, pick_exit, to_local};
pub use index::{IndexedSlot, KindTag, PointIndex, SlotUser, compose};
pub use params::InteractParams;
pub use verbs::{FocusStabilizer, VerbChoice, VerbTarget, rank_verbs};

#[cfg(test)]
mod tests_claims;
#[cfg(test)]
mod tests_door;
#[cfg(test)]
mod tests_queries;
