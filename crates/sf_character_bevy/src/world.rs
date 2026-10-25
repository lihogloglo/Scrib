//! Actor placements of `sf_world_bevy` streaming as characters.

use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use sf_character::CharacterState;
use sf_world::{IndexedPlacement, Level, PlacementFlags, PlacementId};
use sf_world_bevy::{PlacementSpawner, SpawnCx};

use crate::{Character, Characters};

/// On a character spawned from a placement: the placement it stands for, the stable id that
/// activation, scripts and saves address it by (the entity is per session).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CharacterPlacement(pub PlacementId);

/// The `PlacementSpawner` for actor placements (register it for
/// `PlacementClass::Actor`): each placement becomes a [`Character`] of its
/// base record (`RecordRef` kind and id, as the adapter's
/// `CharacterSource` takes them) at the placement, turned by
/// [`crate::CharacterContent::placement_rotation`]. Actors are near-ring
/// content (streaming.md): the far level spawns none, disabled
/// placements neither. The character dresses itself over the next frames
/// (off-thread build); the level doesn't wait for it.
pub struct CharacterSpawner;

impl PlacementSpawner for CharacterSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        if cx.level == Level::Far || p.flags.contains(PlacementFlags::DISABLED) {
            return 0;
        }
        let Some(turn) = world.get_resource::<Characters>().map(|c| c.content().placement_rotation()) else { return 0 };
        let mut local = cx.local;
        local.rotation *= turn;
        let character = Character::Record { kind: p.base.kind.clone(), id: p.base.id.clone(), state: CharacterState::default() };
        world.spawn((
            character,
            CharacterPlacement(p.id),
            local,
            Visibility::Inherited,
            ChildOf(cx.parent),
            cx.layers.clone(),
            crate::AnimSeed::from_id(p.id.0),
        ));
        1
    }

    /// The character loads its own parts (the placement's model is the
    /// skeleton or a creature's body, not a static to instance).
    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        false
    }
}
