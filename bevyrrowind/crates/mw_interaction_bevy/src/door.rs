//! Swing doors into the slot index (crate docs, "Doors").
//!
//! [`DoorHook`] wraps the Door class's spawner. A swing door is a Door placement that is not a
//! travel-door exit (`DoorLinks`). Its visuals spawn under a [`DoorPivot`] that carries the
//! placement's translation and rotation, so the leaf can turn as one. On R0 activation it is
//! indexed with `mw_interaction::door_def`, listed in `KinematicPlacements` (its body is
//! kinematic) and, when the overlay holds a decodable door state, put back at that rest with a
//! `DoorRest`. Deactivation undoes the index entry and the kinematic listing; the shell
//! (`sf_interaction_bevy::Doors`) registers and swings the door from the index changes.

use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError};

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_math::Vec3;
use bevy_transform::components::Transform;
use mw_interaction::door_def;
use sf_coords::{WorldPos, WorldTransform};
use sf_interaction::{DOOR_STATE_OWNER, DoorState};
use sf_interaction_bevy::{DoorRest, IndexChange};
use sf_physics_bevy::KinematicPlacements;
use sf_world::{DoorLink, IndexedPlacement, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::{ActivateCx, DoorLinks, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, WorldOverlay};
use smallvec::SmallVec;

use crate::index::frame_of;

/// The pivot of a swing door's visuals: the placement's translation and rotation (the scale
/// stays on the visuals).
#[derive(Component, Clone, Debug)]
pub struct DoorPivot {
    pub worldspace: WorldspaceId,
    pub id: PlacementId,
    /// The world position of the pivot's parent.
    pub origin: WorldPos,
    /// The closed leaf's centre in the object's frame, metres, scale applied (the model bounds'
    /// centre; zero without a cooked model).
    pub doorway: Vec3,
}

/// The spawned pivot of each swing door, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct DoorPivots(BTreeMap<PlacementId, Entity>);

impl DoorPivots {
    #[must_use]
    pub fn get(&self, id: PlacementId) -> Option<Entity> {
        self.0.get(&id).copied()
    }
}

/// One resident swing door.
#[derive(Clone, Debug)]
pub struct SwingDoor {
    pub worldspace: WorldspaceId,
    /// The closed placement, full precision.
    pub transform: WorldTransform,
}

/// The swing doors in R0, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct SwingDoors(BTreeMap<PlacementId, SwingDoor>);

impl SwingDoors {
    #[must_use]
    pub fn get(&self, id: PlacementId) -> Option<&SwingDoor> {
        self.0.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &SwingDoor)> {
        self.0.iter().map(|(id, d)| (*id, d))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Indexed swing doors per activated cell since the last log line.
#[derive(Resource, Default)]
pub(crate) struct SwingDoorLog(BTreeMap<(String, [i32; 3]), u32>);

/// Whether `id` swings: it is not the exit of a travel door.
fn swing(world: &World, id: PlacementId) -> bool {
    world.get_resource::<DoorLinks>().is_none_or(|links| !links.0.iter().flat_map(DoorLink::exits).any(|(from, _)| from.door == Some(id)))
}

/// The rest state the overlay holds for `id`, if it decodes.
fn saved_rest(world: &World, id: PlacementId) -> Option<DoorState> {
    let overlay = world.get_resource::<WorldOverlay>()?;
    let overlay = overlay.0.read().unwrap_or_else(PoisonError::into_inner);
    DoorState::from_rest_bytes(overlay.state(id, DOOR_STATE_OWNER)?)
}

/// Wraps the Door class's spawner with [`DoorHook`].
pub(crate) fn hook_spawners(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    if let Some(inner) = spawners.for_class(PlacementClass::Door).cloned() {
        spawners.set(PlacementClass::Door, Arc::new(DoorHook(inner)));
    }
}

/// Drops a pivot's entry when it loses [`DoorPivot`] (its cell level despawned), unless a respawn
/// already replaced it.
pub(crate) fn forget_pivot(remove: On<Remove<DoorPivot>>, pivots: Query<&DoorPivot>, mut map: ResMut<DoorPivots>) {
    let Ok(pivot) = pivots.get(remove.entity) else { return };
    if map.0.get(&pivot.id) == Some(&remove.entity) {
        map.0.remove(&pivot.id);
    }
}

/// Spawns swing doors under a pivot and indexes them in R0 (module docs).
pub struct DoorHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for DoorHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        if !swing(world, p.id) {
            return self.0.spawn(world, cx, placement);
        }
        let doorway = match cx.model {
            Some(LoadedModel::Cooked(template)) => Vec3::from(template.aabb.center) * cx.local.scale,
            _ => Vec3::ZERO,
        };
        let origin = p.transform.pos.offset(-cx.local.translation);
        let pivot_local = Transform { translation: cx.local.translation, rotation: cx.local.rotation, scale: Vec3::ONE };
        let pivot = world
            .spawn((
                pivot_local,
                Visibility::Inherited,
                ChildOf(cx.parent),
                DoorPivot { worldspace: cx.worldspace.clone(), id: p.id, origin, doorway },
            ))
            .id();
        world.get_resource_or_init::<DoorPivots>().0.insert(p.id, pivot);
        let inner = SpawnCx { parent: pivot, local: Transform::from_scale(cx.local.scale), ..*cx };
        1 + self.0.spawn(world, &inner, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        if !swing(world, p.id) {
            return;
        }
        let pivot = world.get_resource::<DoorPivots>().and_then(|m| m.get(p.id));
        let doorway = pivot.and_then(|e| world.get::<DoorPivot>(e)).map_or(Vec3::ZERO, |d| d.doorway);
        world.write_message(IndexChange::Insert {
            placement: p.id,
            def: Arc::new(door_def(&p.base.id, doorway)),
            object: frame_of(&p.transform),
            owner: cx.cell_entity,
            disabled: SmallVec::new(),
        });
        if let Some(mut kinematic) = world.get_resource_mut::<KinematicPlacements>() {
            kinematic.insert(cx.worldspace, p.id);
        }
        if let Some(rest) = saved_rest(world, p.id) {
            world.write_message(DoorRest { placement: p.id, open: rest.open, sign: rest.sign });
        }
        let door = SwingDoor { worldspace: cx.worldspace.clone(), transform: p.transform };
        world.get_resource_or_init::<SwingDoors>().0.insert(p.id, door);
        let key = (cx.worldspace.as_str().to_owned(), cx.cell.to_array());
        *world.get_resource_or_init::<SwingDoorLog>().0.entry(key).or_default() += 1;
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        let id = placement.placement.id;
        let Some(door) = world.get_resource_mut::<SwingDoors>().and_then(|mut d| d.0.remove(&id)) else { return };
        world.write_message(IndexChange::unload(id));
        if let Some(mut kinematic) = world.get_resource_mut::<KinematicPlacements>() {
            kinematic.remove(&door.worldspace, id);
        }
    }
}

/// One line per cell that indexed swing doors this frame.
pub(crate) fn log_indexed(mut log: ResMut<SwingDoorLog>) {
    if log.0.is_empty() {
        return;
    }
    for ((worldspace, [x, y, z]), n) in std::mem::take(&mut log.0) {
        info!("interact: {n} swing doors indexed in {worldspace} cell ({x}, {y}, {z})");
    }
}
