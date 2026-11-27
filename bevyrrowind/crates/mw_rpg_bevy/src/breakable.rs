//! The host's stage hook for breakables (`IN38b2`; superfargoth `interaction-destruction.md`
//! "Host stage hook"): a released break ([`Destroyed`], `sf_interaction_bevy` `destroy.rs`) whose
//! stage removes the object takes its placement out of the world.
//!
//! - **Live** (the placement is in R0, [`ActivePlacements`]: crates, barrels, sacks and urns of the
//!   `CONTAINER` class, loose `MISC` pottery): [`ActivePlacements::remove_live`] runs its
//!   deactivation hooks now (static collider, script handle, furniture index unload, sounds),
//!   removes it from the overlay and despawns its visuals (the container's `LiveRoot`); an item's
//!   physics `ItemPivot` goes too, as a take does (`inventory::take_placement`).
//! - **Unloaded** (a queued break released after its cell left R0, the shell's "Release"): the
//!   overlay removal alone, so the cell never brings it back; any visual still drawn (a cell
//!   kept at `Near`) despawns with it.
//! - Ordered `.after(DestroySystems::Release)`: the shell's `IndexChange::Remove { OwnerDestroyed }`
//!   is written before the deactivation hook's `unload`, so claims end for the break's reason.
//!
//! Not yet (own rows): spilling a container's contents as placed items (`IN38b3`: a live item
//! spawn into a loaded cell needs its model loaded, which `Streaming::spawn_into` does not start;
//! until then the store stays keyed by the broken placement, nothing is lost), loot, salvage and
//! the stage effect; a model swap stage (no adapter def has one: `IN38a` gives one removing stage),
//! debris (`IN1c`), decorative `STAT` urns (`IN38c`). Classic rules: no destructible defs, no
//! `Destroyed`, nothing runs.

use std::sync::PoisonError;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_log::info;
use mw_scripts_bevy::{ActivePlacements, LiveRoot};
use sf_interaction_bevy::{DestroySystems, Destroyed};
use sf_physics_bevy::ItemPivot;
use sf_world::PlacementId;
use sf_world_bevy::WorldOverlay;

/// What a removing break did to its placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Removal {
    /// It was live in R0: hooks, overlay, visuals.
    Live,
    /// Its cell had left R0: the overlay removal (and any visual still drawn).
    Unloaded,
    /// Already removed (a take or an earlier break): nothing changed.
    Gone,
}

/// Registers the hook (host; the app calls it with the inventory).
pub fn add(app: &mut App) {
    app.add_message::<Destroyed>().add_systems(FixedUpdate, apply_breaks.after(DestroySystems::Release));
}

/// The tick's released breaks: every removing one takes its placement out ([`remove_broken`]).
fn apply_breaks(world: &mut World, reader: &mut SystemState<MessageReader<'static, 'static, Destroyed>>) {
    let breaks: Vec<Destroyed> = reader.get_mut(world).map(|mut r| r.read().copied().collect()).unwrap_or_default();
    for Destroyed(b) in breaks.into_iter().filter(|b| b.0.removes) {
        let how = remove_broken(world, b.placement);
        info!("break {:032x} stage {}: {how:?}", b.placement.0, b.stage);
    }
}

/// Takes broken placement `id` out of the world (module docs); a second call finds it [`Removal::Gone`].
pub fn remove_broken(world: &mut World, id: PlacementId) -> Removal {
    let how = if ActivePlacements::remove_live(world, id).is_some() {
        Removal::Live
    } else {
        let Some(overlay) = world.get_resource::<WorldOverlay>() else { return Removal::Gone };
        let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
        if overlay.is_removed(id) {
            return Removal::Gone;
        }
        overlay.remove(id);
        Removal::Unloaded
    };
    let visuals: Vec<Entity> = world
        .query::<(Entity, Option<&LiveRoot>, Option<&ItemPivot>)>()
        .iter(world)
        .filter(|(_, root, pivot)| root.is_some_and(|r| r.0 == id) || pivot.is_some_and(|p| p.id == id))
        .map(|(e, ..)| e)
        .collect();
    for e in visuals {
        world.despawn(e);
    }
    how
}
