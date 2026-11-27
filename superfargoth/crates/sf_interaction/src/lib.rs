//! Bevy-free interaction core (`superfargoth/docs/specs/interaction.md` "The shared core").
//!
//! - [`def`]: smart-object definitions (`sf.interact.def` records): slots and their kinds, verbs,
//!   entrances, carry and destruction data.
//! - [`claims`]: the host's claim table: priority with preemption of Reserved claims only, queue
//!   tickets, leases, exclusion groups, one body hold per user plus hand claims, typed refusals,
//!   invalidation reasons and the sorted same-tick batch.
//! - [`index`]: the point index of resident slots (grid buckets) and `find_slots`.
//! - [`verbs`]: `rank_verbs` (the uses an object offers now) and the per-player `FocusStabilizer`.
//! - [`destroy`]: tier 1 destruction maths: `apply_damage` (stages crossed, overkill), the debris
//!   distance gate, the per-tick break queue, the health + stage rest state.
//! - [`door`]: the swing-door state machine (toggle, step, leaf pose, close-behind schedule).
//! - [`rest`]: the rest hook's slotless verb `VERB_SLEEP`.
//! - [`light`]: light as state: `LightDef`, the `Lit` toggle (snuff / relight, rule sets), `LightChanged`,
//!   the one-byte rest state.
//! - [`entrance`]: `pick_entrance` / `pick_exit` (side order, the shell's validity checks).
//! - [`spot`]: `pick_spot` / `spot_candidates`, where a carried prop is put down (surface slots, then a
//!   floor spiral), the same answer in both AI tiers.
//! - [`cook`]: `interact.cook`, the one-time def bake (stub).
//! - [`params`]: the core's tunables as plain fields (`interact.*`, bound by `sf_interaction_bevy`).
//!
//! Never names a game, `sf_ai` or `sf_crime`.

pub mod claims;
pub mod cook;
pub mod def;
pub mod destroy;
pub mod door;
pub mod entrance;
pub mod index;
pub mod light;
pub mod params;
pub mod rest;
pub mod spot;
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
pub use destroy::{
    BreakQueue, DESTRUCT_REST_VERSION, DESTRUCT_STATE_OWNER, DestructState, Hit, apply_damage, debris_lod,
};
pub use door::{DOOR_OPEN, DOOR_REST_VERSION, DOOR_STATE_OWNER, DoorChanged, DoorState, VERB_CLOSE, VERB_OPEN, swing_sign};
pub use entrance::{pick_entrance, pick_exit, to_local};
pub use index::{IndexedSlot, KindTag, PointIndex, SlotUser, compose};
pub use light::{
    LIGHT_REST_LIT, LIGHT_REST_UNLIT, LIGHT_STATE_OWNER, LIT, LightChanged, LightDef, LightRefusal, LightState,
    VERB_RELIGHT, VERB_SNUFF,
};
pub use params::InteractParams;
pub use rest::VERB_SLEEP;
pub use spot::{Spot, SpotAnchor, SpotBlockers, SpotQuery, pick_spot, spiral_point, spot_candidates};
pub use verbs::{FocusStabilizer, VerbChoice, VerbTarget, rank_verbs};

#[cfg(test)]
mod tests_claims;
#[cfg(test)]
mod tests_destroy;
#[cfg(test)]
mod tests_door;
#[cfg(test)]
mod tests_light;
#[cfg(test)]
mod tests_queries;
#[cfg(test)]
mod tests_spot;
