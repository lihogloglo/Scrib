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
//! - **Spill** (`IN38b3c`; interaction-destruction.md "Spill"): a removing stage whose `on_break` is
//!   `SpillContents` (the registry's def; a container record when the registry already let go)
//!   lands the container's store on the floor ([`spill_stacks`]): seeded first if never opened,
//!   taken out of `RpgState::inventories`, each stack thinned by the hit's overkill and its
//!   condition worn (the `mw_interaction` maths off the break's seed), one spawned placement per
//!   kept stack in the overlay (`physics` blob: awake, so it falls and settles; `mw.loose` blob:
//!   count, owned flag, condition), scattered in the former footprint. Live: queued into its cell
//!   (`Streaming::spawn_when_ready`, the model loads first); Unloaded: the overlay alone, the
//!   cell's next load spawns them. A placement with no known transform (an unloaded index
//!   reference: only live, moved and spawned placements carry one here) keeps its store under the
//!   dead id (warn): nothing is lost.
//!
//! - **Static** (`IN38c2`; interaction-destruction.md "Static removal"): a decorative `STAT` urn
//!   is live like a container (`mw_scripts_bevy::LiveStatics`): `remove_live` drops its `Fixed`
//!   collider, removes it from the overlay and despawns its `LiveRoot` when it was entity-drawn;
//!   then `Streaming::hide_covered` stops the static scene drawing it (its instances inactive,
//!   one way). It spills nothing (not a `CONT`).
//!
//! Not yet (own rows): loot, salvage and the stage effect; a model swap stage (no adapter def has
//! one: `IN38a` gives one removing stage), debris (`IN1c`), urns kept out of the precombine (`IN38c3`).
//! Classic rules: no destructible defs, no `Destroyed`, nothing runs.

use std::sync::PoisonError;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_log::{debug, info, warn};
use mw_interaction::breakable::draw;
use mw_interaction::{LOOSE_STATE_OWNER, LooseItem, SpillParams, health_loss, kept_count, lost_fraction, scatter, spill_draw};
use mw_rpg::{ATTR_HEALTH, MwItemClass, item_health, trade_item};
use mw_scripts_bevy::{ActivePlacement, ActivePlacements, LiveRoot};
use sf_coords::WorldTransform;
use sf_interaction_bevy::sf_interaction::{self, BreakAction};
use sf_interaction_bevy::{DestroySystems, Destroyed, Destructibles};
use sf_physics_bevy::{ItemMotion, ItemPivot};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ItemStack};
use sf_vfs::VfsPath;
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{PlacementSpawners, SpawnWhenReady, Streaming, WorldOverlay};
use tes3::esp::Container;

use crate::dialogue::{Rpg, WorldSheets};
use crate::inventory::{ItemRecords, RefOwners, container_store, is_gold, player_level};

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

/// The spill's tunables, read when a break releases: the host passes the adapter's live values
/// (`mw_interaction_bevy` `BreakSettings`); this crate does not depend on that shell.
#[derive(Resource, Clone, Copy)]
pub struct SpillSource(pub fn(&World) -> SpillParams);

/// Registers the hook (host; the app calls it with the inventory), reading the spill's tunables
/// through `spill`.
pub fn add(app: &mut App, spill: fn(&World) -> SpillParams) {
    app.insert_resource(SpillSource(spill))
        .add_message::<Destroyed>()
        .add_systems(FixedUpdate, apply_breaks.after(DestroySystems::Release));
}

/// The tick's released breaks: every removing one takes its placement out ([`remove_broken`]) and
/// a spilling one lands its contents (module docs, "Spill").
fn apply_breaks(world: &mut World, reader: &mut SystemState<MessageReader<'static, 'static, Destroyed>>) {
    let breaks: Vec<Destroyed> = reader.get_mut(world).map(|mut r| r.read().copied().collect()).unwrap_or_default();
    for Destroyed(b) in breaks.into_iter().filter(|b| b.0.removes) {
        let spills = stage_spills(world, &b);
        let (how, spot) = take_out(world, b.placement);
        info!("break {:032x} stage {}: {how:?}", b.placement.0, b.stage);
        if spills != Some(false) && how != Removal::Gone {
            spill(world, &b, how, spot);
        }
    }
}

/// Whether the break's stage spills (the registry's def); `None` once the registry let go of the
/// placement (its cell unloaded before the release): then a container record spills.
fn stage_spills(world: &World, b: &sf_interaction::Destroyed) -> Option<bool> {
    let d = world.get_resource::<Destructibles>()?.get(b.placement)?;
    let stage = d.destructible()?.stages.get(usize::from(b.stage))?;
    Some(stage.on_break == BreakAction::SpillContents)
}

/// Where a broken placement was, captured before its removal.
struct Spot {
    worldspace: WorldspaceId,
    placement: Placement,
}

/// Takes broken placement `id` out of the world (module docs); a second call finds it [`Removal::Gone`].
pub fn remove_broken(world: &mut World, id: PlacementId) -> Removal {
    take_out(world, id).0
}

/// [`remove_broken`], with where the placement was (live, or moved / spawned in the overlay).
fn take_out(world: &mut World, id: PlacementId) -> (Removal, Option<Spot>) {
    let (how, spot) = if let Some(a) = ActivePlacements::remove_live(world, id) {
        if a.placement.placement.class == PlacementClass::Static {
            hide_static(world, &a);
        }
        (Removal::Live, Some(Spot { worldspace: a.worldspace, placement: a.placement.placement }))
    } else {
        let Some(overlay) = world.get_resource::<WorldOverlay>() else { return (Removal::Gone, None) };
        let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
        if overlay.is_removed(id) {
            return (Removal::Gone, None);
        }
        let spot = overlay.placed(id).map(|(ws, p)| Spot { worldspace: ws.clone(), placement: p.clone() });
        overlay.remove(id);
        (Removal::Unloaded, spot)
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
    (how, spot)
}

/// A live breakable static (`IN38c2`, module docs "Static"): the scene stops drawing it now (an
/// entity-drawn one went with its `LiveRoot`: the hide finds nothing).
fn hide_static(world: &mut World, a: &ActivePlacement) {
    if !world.contains_resource::<Streaming>() {
        return;
    }
    let id = a.placement.placement.id;
    let hidden = world.resource_scope(|world, mut streaming: Mut<Streaming>| streaming.hide_covered(world, &a.worldspace, a.cell, id));
    debug!("break {:032x}: static {}", id.0, if hidden { "hidden in the scene" } else { "entity-drawn" });
}

/// Lands the store of broken container `b.placement` on the floor (module docs, "Spill").
fn spill(world: &mut World, b: &sf_interaction::Destroyed, how: Removal, spot: Option<Spot>) {
    let Some(spot) = spot else {
        warn!("spill {:032x}: no transform known, its contents stay keyed by it", b.placement.0);
        return;
    };
    let Some(db) = world.get_resource::<ItemRecords>().map(|r| r.0.clone()) else { return };
    let record = spot.placement.base.id.clone();
    if db.get::<Container>(&record).is_none() || !world.contains_resource::<Rpg>() {
        return;
    }
    let params = world.get_resource::<SpillSource>().map_or_else(SpillParams::default, |s| (s.0)(world));
    let level = player_level(&WorldSheets(world));
    let owned = world.get_resource::<RefOwners>().is_some_and(|o| o.0.contains(&b.placement));
    let key = ActorKey(b.placement.0);
    let stacks = {
        let mut rpg = world.resource_mut::<Rpg>();
        container_store(&mut rpg.state, &db, key, &record, level);
        rpg.state.inventories.remove(&key).map(|c| c.loadout.inventory().stacks().to_vec()).unwrap_or_default()
    };
    let loose = spill_stacks(&db, &stacks, b.seed, b.overkill, owned, &params);
    let mut placed = Vec::with_capacity(loose.len());
    if let Some(overlay) = world.get_resource::<WorldOverlay>() {
        let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
        for (index, (item, loose)) in (0u64..).zip(&loose) {
            let Some(base) = mw_world::source::resolve_base(&db, item.as_str()) else {
                warn!("spill {:032x}: {item} has no placeable record, lost", b.placement.0);
                continue;
            };
            let at = scatter(b.seed, index, &params);
            let pos = spot.placement.transform.pos.offset(at.offset);
            let mut placement = Placement {
                id: PlacementId(0),
                base: RecordRef::new(base.tag, item.clone()),
                model: base.model.and_then(|m| VfsPath::new(&m).ok()),
                transform: WorldTransform { pos, rotation: at.rotation, ..WorldTransform::IDENTITY },
                class: base.class,
                flags: PlacementFlags::empty(),
            };
            placement.id = overlay.spawn(spot.worldspace.clone(), placement.clone());
            overlay.set_state(placement.id, sf_physics_bevy::STATE_OWNER, ItemMotion::DROPPED.encode());
            overlay.set_state(placement.id, LOOSE_STATE_OWNER, loose.encode());
            placed.push(placement);
        }
    }
    info!("spill {:032x}: {} of {} stacks", b.placement.0, placed.len(), stacks.len());
    if how == Removal::Live {
        spawn_live(world, &spot.worldspace, placed);
    }
}

/// Queues spilled `placed` into their loaded cells; a refused one (its cell not resident) spawns
/// from the overlay at that cell's next load.
fn spawn_live(world: &mut World, ws: &WorldspaceId, placed: Vec<Placement>) {
    if !world.contains_resource::<Streaming>() || !world.contains_resource::<PlacementSpawners>() {
        return;
    }
    world.resource_scope(|world, mut streaming: Mut<Streaming>| {
        let partition = streaming.partition(ws);
        let spawners = world.resource::<PlacementSpawners>();
        for placement in placed {
            let cell = partition.cell_of(placement.transform.pos);
            let id = placement.id;
            let queued = streaming.spawn_when_ready(world, spawners, ws, cell, IndexedPlacement { placement, radius: 0.0 });
            if queued == SpawnWhenReady::Refused {
                debug!("spill: {:032x} waits for cell {cell} to load", id.0);
            }
        }
    });
}

/// What each of a broken container's `stacks` leaves on the floor, in store order (a stack that
/// loses every piece is left out): the pieces kept ([`kept_count`] of [`lost_fraction`]`(overkill)`,
/// gold whole) and, for weapons and armour with a condition, the condition left after
/// [`health_loss`] (`None` = full). A restocking stack spills its count once: the loose item
/// carries no restock mark.
#[must_use]
pub fn spill_stacks(db: &RecordDb, stacks: &[ItemStack], seed: u64, overkill: f32, owned: bool, p: &SpillParams) -> Vec<(RecordId, LooseItem)> {
    let lost = lost_fraction(overkill, p);
    let health_key = RecordId::new(ATTR_HEALTH);
    (0u64..)
        .zip(stacks)
        .filter_map(|(index, s)| {
            let count = kept_count(s.count, lost, spill_draw(seed, index, draw::COUNT), is_gold(&s.item));
            if count == 0 {
                return None;
            }
            let max = trade_item(db, &s.item)
                .filter(|t| matches!(t.class, MwItemClass::Weapon | MwItemClass::Armor))
                .and_then(|t| t.max_health);
            let health = max.and_then(|max| {
                let wear = health_loss(overkill, max, p);
                let recorded = s.attrs.get(&health_key).is_some();
                (recorded || wear > 0).then(|| (item_health(&s.attrs, max) - wear).max(0))
            });
            Some((s.item.clone(), LooseItem { count, owned, health }))
        })
        .collect()
}
