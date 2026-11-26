//! The game's wiring of the interaction shell (`bevyrrowind/docs/specs/interaction-adapter.md`,
//! slice `IN32c1`; superfargoth `docs/specs/interaction.md` "Activation contract"). The logic is
//! in `mw_interaction` (defs, activation rows) and `sf_interaction(_bevy)` (index, claims, focus,
//! activation, seats); this crate connects them to the streamed world, the player and its input.
//!
//! - **Defs** ([`FurnitureDefs`]): the baked `mw.interact.defs` product (`mw_bake::interact`, `IN32b2`),
//!   base `RecordId` → def, read once at startup ([`load_defs`]). A missing or undecodable product
//!   logs one warning and gives no defs: the game runs as before (nothing indexed, every target
//!   def-less).
//! - **Index** ([`index`]): a placement-spawner hook on the census classes (statics, activators,
//!   containers) writes `IndexChange::Insert` (the def, the placement's world frame, the cell entity
//!   as owner) when a placement with a def enters R0, and `IndexChange::Remove` (`OwnerUnloaded`)
//!   when it leaves (also on a discard). `mw.rules.interaction.sit` off (Classic) indexes no
//!   furniture; flipping it live removes or re-inserts the resident furniture.
//! - **Doors** ([`door`]): a hook on the Door class spawns a swing door's (not a travel-door exit)
//!   visuals under a [`DoorPivot`] (at its rest state while outside R0) and, in R0, indexes it
//!   with `mw_interaction::door_def`, lists its body as kinematic and restores its saved rest
//!   state (`DoorRest`); the shell's `Doors`
//!   registers and swings it, and each toggle writes its rest state to the placement overlay
//!   (owner `door`). Log: `interact: N swing doors indexed in <worldspace> cell (x, y, z)`.
//! - **Door bodies** ([`door_body`]): each swing step of the shell (`DoorMoved`) moves the door's
//!   kinematic body to the leaf (a restored rest teleports it, also for a body inserted later),
//!   a step whose leaf would overlap a character's capsule is held (the door pauses in their path),
//!   and the [`DoorPivot`] follows the body's interpolated pose each frame (the shell's leaf
//!   without physics).
//! - **Player user** ([`input`]): the player body gets an `InteractionUser` (`PLAYER`, the live
//!   `interact.claim.priority_player`, no tags), so the seat default (`IN2b2`) seats it.
//! - **Focus**: the focus ray's raw first hit (`mw_player_bevy::Focus::hit`) is the player's one
//!   `FocusCandidates` per fixed tick (score from distance; none: empty). A target with a def
//!   prompts the verb its focus offers, from the furniture rows (`mw_interaction::activation::ROWS`:
//!   "Sit", "Lie down", "Lean"); every other target keeps `activatable`'s name and verb, and so
//!   does a def'd census container or activator whose best verb is its class's own (the baked
//!   slotless class verb, `IN32c0`: "Open", "Use"; its pose verbs follow in the hold list), with
//!   that verb's own row prompt when `activatable`'s is another (an open swing door's `close`:
//!   "Close", `IN33c2c`).
//! - **Activation**: each latched Use press (`mw_player_bevy::UsePresses`, written by
//!   `mw_scripts_bevy`) becomes one `ActivateCommand` ([`FocusState::command`], best verb and slot;
//!   a def-less target's primary use) between `InteractionSystems::Focus` and `::Activate`.
//!
//! Log lines: `interact: N furniture defs loaded (S slots)` at startup and
//! `interact: indexed M furniture placements in <worldspace> (x, y, z)` once per cell activation.
//!
//! [`FocusState::command`]: sf_interaction_bevy::FocusState::command

pub mod door;
pub mod door_body;
pub mod index;
pub mod input;
mod report;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_door;
#[cfg(test)]
mod tests_door_body;

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, PostStartup, Update};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use mw_bake::interact::{INTERACT_DEFS, SUBJECT, decode_interact_defs};
use mw_game_core::player::FocusUpdate;
use mw_player_bevy::{Focus, UsePresses};
use sf_bake::{Manifest, Store};
use sf_interaction::InteractableDef;
use sf_interaction_bevy::{DoorSystems, InteractionPlugin, InteractionSystems};
use sf_physics_bevy::PhysicsSystems;
use sf_records::RecordId;
use sf_tunables_bevy::{TunablesAppExt, tunable_changed};

pub use door::{DoorHook, DoorPivot, DoorPivots, SwingDoor, SwingDoors};
pub use index::{ResidentFurniture, SitSwitch};
pub use input::furniture_prompt;

/// The baked furniture defs, by base record.
#[derive(Resource, Clone, Debug, Default)]
pub struct FurnitureDefs(pub BTreeMap<RecordId, Arc<InteractableDef>>);

impl FurnitureDefs {
    #[must_use]
    pub fn get(&self, base: &RecordId) -> Option<&Arc<InteractableDef>> {
        self.0.get(base)
    }
}

/// The `mw.interact.defs` product from the bake cache; no defs (one warning) when it is missing
/// or does not decode (a bake older than the product's format).
#[must_use]
pub fn load_defs(store: &Store, manifest: &Manifest) -> FurnitureDefs {
    let Some(digest) = manifest.get(INTERACT_DEFS, SUBJECT) else {
        warn!("interact: no {INTERACT_DEFS} product (run `bevyrrowind bake`): furniture offers no verbs");
        return FurnitureDefs::default();
    };
    let decoded = store.get(&digest).map_err(|e| e.to_string()).and_then(|b| decode_interact_defs(&b).map_err(|e| e.to_string()));
    match decoded {
        Ok(product) => {
            let slots: usize = product.defs.iter().map(|d| d.slots.len()).sum();
            let defs: BTreeMap<RecordId, Arc<InteractableDef>> =
                product.defs.into_iter().map(|d| (d.id.clone(), Arc::new(d))).collect();
            info!("interact: {} furniture defs loaded ({slots} slots)", defs.len());
            FurnitureDefs(defs)
        }
        Err(e) => {
            warn!("interact: {INTERACT_DEFS} unreadable ({e}; re-run `bevyrrowind bake`): furniture offers no verbs");
            FurnitureDefs::default()
        }
    }
}

/// Adds the interaction shell to the game with the baked defs of `store` / `manifest`.
pub fn add(app: &mut App, store: &Store, manifest: &Manifest) {
    add_with_defs(app, load_defs(store, manifest));
}

/// Adds the interaction shell with `defs` (module docs). Needs `sf_script_bevy::ScriptsPlugin`
/// (the scripts answer every activation) and, for the player parts, `mw_player_bevy::add`.
pub fn add_with_defs(app: &mut App, defs: FurnitureDefs) {
    if !app.is_plugin_added::<InteractionPlugin>() {
        app.add_plugins(InteractionPlugin);
    }
    app.register_tunables(mw_interaction::TUNABLES);
    let sit = app.add_tunable(&mw_interaction::tunables::SIT).id();
    app.insert_resource(defs)
        .insert_resource(SitSwitch(sit))
        .init_resource::<ResidentFurniture>()
        .init_resource::<index::IndexLog>()
        .init_resource::<DoorPivots>()
        .init_resource::<SwingDoors>()
        .init_resource::<door::SwingDoorLog>()
        .init_resource::<Focus>()
        .init_resource::<UsePresses>()
        .add_observer(door::forget_pivot)
        .add_systems(PostStartup, (index::hook_spawners, door::hook_spawners))
        .add_systems(Update, (index::sync_sit.run_if(tunable_changed(sit)), index::log_indexed, door::log_indexed, input::player_user))
        .add_systems(Update, input::prompt.in_set(FocusUpdate).after(mw_player_bevy::FocusRay))
        .add_systems(FixedUpdate, input::candidates.after(InteractionSystems::Apply).before(InteractionSystems::Focus))
        .add_systems(FixedUpdate, input::commands.after(InteractionSystems::Focus).before(InteractionSystems::Activate))
        .add_systems(Update, door::save_rest.after(DoorSystems::Default))
        .add_systems(Update, report::log_reports.after(sf_interaction_bevy::SeatSystems::Request))
        .add_systems(FixedUpdate, door_body::block_sweeps.after(DoorSystems::Plan).before(DoorSystems::Commit))
        .add_systems(FixedUpdate, door_body::move_bodies.after(DoorSystems::Commit).before(PhysicsSystems::Step))
        .add_systems(Update, (door_body::pose_new_bodies, door_body::follow_pivots).chain().after(PhysicsSystems::Stream));
}
