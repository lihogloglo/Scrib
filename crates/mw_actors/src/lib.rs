//! Morrowind actors as framework characters, Bevy-free: the
//! `CharacterSource` for `NPC_` and `CREA` records ([`MwActors`]).
//!
//! - **Parts** (`sf.body_part`, built on first use through [`sf_character::PartLookup`]):
//!   one per (`BODY`, part slot), id `"<body id>#<slot>"`. Skinned models carry
//!   the slot's node filter (a skin file holds both sides); rigid ones are
//!   bolted to the slot's bone, mirrored on left-side bones.
//! - **Wearables** (`sf.wearable`): one per `ARMO`/`CLOT` and body sex, id
//!   `"<item id>#m"`/`"#f"`, from the item's part list (female parts fall back
//!   to male), with the original part priorities as layers and hidden slots.
//! - **Recipes**: race skin parts, head, hair, auto-equipped inventory
//!   ([`equip`]), race height/weight scale, skeleton by sex and beast race,
//!   the first-person view model, held items ([`held`]: weapon, shield, torch
//!   at night). Creatures: their model as skeleton and
//!   body, a rig profile instead of a humanoid map, CREA scale.
//!
//! Rules follow the original engine as `OpenMW` documents it (GPL reference;
//! this crate is GPL). Spec: `docs/specs/actors-adapter.md` ("`CharacterSource` as built").

pub mod equip;
pub mod held;
pub mod probes;
pub mod slots;
mod source;

pub use source::{
    ActorConfig, CREATURE_KIND, LEVELED_CREATURE_KIND, ModelProbe, MwActors, NPC_KIND, PartModel, creature_part_id, layer, leveled_pick, part_id, skeleton,
    wearable_id,
};

#[cfg(test)]
mod tests;
