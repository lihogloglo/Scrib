//! Actor placements of `sf_world_bevy` streaming as characters, and the
//! worldspace render layer of every character ([`follow_worldspace_layers`]).

use bevy_camera::visibility::{RenderLayers, Visibility, VisibilityClass};
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh3d;
use sf_character::CharacterState;
use sf_world::{IndexedPlacement, Level, PlacementFlags, PlacementId};
use sf_world_bevy::{PlacementSpawner, SpawnCx, WorldspaceRoot};

use crate::{Character, Characters, HeldLightSlot};

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

/// What a character's render layer reaches: meshes, lights (anything Bevy's
/// visibility checks: `VisibilityClass`), held-light slots, and whatever
/// already carries layers.
type Layered = Or<(With<Mesh3d>, With<VisibilityClass>, With<HeldLightSlot>, With<RenderLayers>)>;

/// Keeps every [`Character`] on the render layer of its worldspace: the
/// root's `RenderLayers` become `layer(N)` of its nearest
/// [`WorldspaceRoot`] ancestor, and a change is pushed to every existing
/// descendant that renders or lights ([`Layered`]). Parts built later copy
/// the root's layers at build time (`finish_builds`, `held::sync_held`).
///
/// Why: Bevy pairs a light with shadow casters and views by `RenderLayers`
/// (a mesh without the component is layer 0), and a worldspace's lamps sit
/// on its layer only (a `[0, N]` light would light resident neighbouring
/// worldspaces), so a character left on layer 0 never shadows under them,
/// and a torch it carries lights nothing of its worldspace.
///
/// Runs every frame (`PostUpdate`, before `CheckVisibility`): an ancestor
/// walk per character, stopping at the first worldspace root (two or three
/// hops), so it catches every way a character changes worldspace: its own
/// reparenting, an ancestor's (a player avatar under a body that follows
/// the camera through a door after `TravelArrived`), and its first spawn.
/// After `Update`, so parts built this frame with the old layers exist and
/// get the new ones. Characters outside any worldspace keep their layers.
///
/// Bevy's `HierarchyPropagatePlugin<RenderLayers>` would do the push, but it
/// observes every `ChildOf` insert in the world (all streamed content) and
/// puts two components on every rig joint; the layer changes only on travel.
pub(crate) fn follow_worldspace_layers(
    mut commands: Commands,
    characters: Query<(Entity, Option<&RenderLayers>), With<Character>>,
    parents: Query<&ChildOf>,
    roots: Query<&WorldspaceRoot>,
    children: Query<&Children>,
    layered: Query<(), Layered>,
) {
    for (character, current) in &characters {
        let Some(layer) = parents.iter_ancestors(character).find_map(|a| roots.get(a).ok().map(|r| r.layer)) else { continue };
        let layers = RenderLayers::from_layers(&[layer]);
        if current == Some(&layers) {
            continue;
        }
        for part in children.iter_descendants(character) {
            if layered.contains(part) {
                commands.entity(part).try_insert(layers.clone());
            }
        }
        commands.entity(character).try_insert(layers);
    }
}
