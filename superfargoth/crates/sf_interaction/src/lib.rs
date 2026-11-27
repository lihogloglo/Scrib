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
//!   distance gate, the per-tick break queue, the health + stage rest state, the `Destroyed` event and
//!   its `break_seed`.
//! - [`door`]: the swing-door state machine (toggle, step, leaf pose, close-behind schedule).
//! - [`rest`]: the rest hook's slotless verb `VERB_SLEEP`.
//! - [`light`]: light as state: `LightDef`, the `Lit` toggle (snuff / relight, rule sets), `LightChanged`,
//!   the one-byte rest state.
//! - [`entrance`]: `pick_entrance` / `pick_exit` (side order, the shell's validity checks).
//! - [`spot`]: `pick_spot` / `spot_candidates`, where a carried prop is put down (surface slots, then a
//!   floor spiral), the same answer in both AI tiers.
//! - [`states`]: object states, `TagId`s with a TTL by placement (`ObjectStates`: add/refresh, remove,
//!   `has`, sorted expiry), the table world rules read and write.
//! - [`rules`]: the rules layer: `InteractionRule` records, the `RuleQueue` of causes and `step_rules`
//!   (first match in record order, budgeted, oldest first, the same result whatever the arrival order).
//! - [`fire`]: the fire heat core: `step_fire` (heat from burning neighbours with falloff, wind and seeded
//!   jitter, ignition at the material's `ignite_heat`, fuel then `Charred`, `Wet` takes no heat, rain douses).
//! - [`impact`]: `impact_damage`, the raw damage of one gated physics contact.
//! - [`noise`]: `loudness_m` (material reach x clamped impact speed) and `NoiseMerge` (an object's noise
//!   within its window heard only when louder, the per-tick cap, loudest first, any arrival order alike).
//! - [`world_change`]: `WorldChange`, the lasting changes reactions read (kinds as `TagId` rows, pure
//!   mappers from the door, light, destroy and fire events, `fold_tick`: one event per transition).
//! - [`bash`]: one bash blow on a lock or door: `bash` (threshold by material, kind factor, self damage or
//!   weapon wear, contents risk), the `Bashed` event and its `bash_seed`.
//! - [`cook`]: `interact.cook`, the one-time def bake (stub).
//! - [`params`]: the core's tunables as plain fields (`interact.*`, bound by `sf_interaction_bevy`).
//!
//! Never names a game, `sf_ai` or `sf_crime`.

pub mod bash;
pub mod claims;
pub mod cook;
pub mod def;
pub mod destroy;
pub mod door;
pub mod entrance;
pub mod fire;
pub mod impact;
pub mod index;
pub mod light;
pub mod noise;
pub mod params;
pub mod rest;
pub mod rules;
pub mod spot;
pub mod states;
pub mod verbs;
pub mod world_change;

pub use bash::{BashResult, BashRules, Bashed, Blow, VERB_BASH, bash, bash_seed};
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
    BreakQueue, DAMAGE_IMPACT, DAMAGE_SCRIPT, DESTRUCT_REST_VERSION, DESTRUCT_STATE_OWNER, DestructState, Destroyed, Hit,
    apply_damage, break_seed, debris_lod, splitmix64,
};
pub use door::{DOOR_OPEN, DOOR_REST_VERSION, DOOR_STATE_OWNER, DoorChanged, DoorState, VERB_CLOSE, VERB_OPEN, swing_sign};
pub use fire::{
    BURNING, CHARRED, Fire, FireBody, FireEnv, FireEvent, FireOutput, WET, fire_region, heat_jitter, step_fire,
    wind_factor,
};
pub use entrance::{pick_entrance, pick_exit, to_local};
pub use impact::impact_damage;
pub use index::{IndexedSlot, KindTag, PointIndex, SlotUser, compose};
pub use light::{
    LIGHT_REST_LIT, LIGHT_REST_UNLIT, LIGHT_STATE_OWNER, LIT, LightChanged, LightDef, LightRefusal, LightState,
    VERB_RELIGHT, VERB_SNUFF,
};
pub use noise::{Noise, NoiseCase, NoiseMerge, loudness_m};
pub use params::InteractParams;
pub use rest::VERB_SLEEP;
pub use rules::{
    Cause, Flammable, InteractionRule, MaterialDef, RuleEvent, RuleOutput, RuleQueue, RuleResult, Rules, step_rules,
};
pub use spot::{Spot, SpotAnchor, SpotBlockers, SpotQuery, pick_spot, spiral_point, spot_candidates};
pub use states::{ObjectStates, StateEntry, Ttl};
pub use verbs::{FocusStabilizer, VerbChoice, VerbTarget, rank_verbs};
pub use world_change::{
    CHANGE_BASHED, CHANGE_BURNT, CHANGE_CLOSED, CHANGE_DESTROYED, CHANGE_DISPLACED, CHANGE_DROPPED, CHANGE_IGNITED,
    CHANGE_KILLED, CHANGE_LANDED, CHANGE_LOCK_BROKEN, CHANGE_LOCKED, CHANGE_OCCUPIED, CHANGE_OPENED, CHANGE_RELIT,
    CHANGE_SNUFFED, CHANGE_SPILLED, CHANGE_SWITCHED, CHANGE_TAKEN, CHANGE_UNLOCKED, ChangeDetail, ChangeOwner,
    ChangeSite, SHIPPED_KINDS, WorldChange, fold_tick, from_bashed, from_destroyed, from_door, from_fire, from_light,
};

#[cfg(test)]
mod tests_bash;
#[cfg(test)]
mod tests_claims;
#[cfg(test)]
mod tests_destroy;
#[cfg(test)]
mod tests_door;
#[cfg(test)]
mod tests_fire;
#[cfg(test)]
mod tests_impact;
#[cfg(test)]
mod tests_light;
#[cfg(test)]
mod tests_noise;
#[cfg(test)]
mod tests_queries;
#[cfg(test)]
mod tests_rules;
#[cfg(test)]
mod tests_spot;
#[cfg(test)]
mod tests_states;
#[cfg(test)]
mod tests_world_change;
