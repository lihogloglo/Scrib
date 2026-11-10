//! Actor placements of `sf_world_bevy` streaming as characters, and the
//! worldspace render layers of every character ([`follow_worldspace_layers`]).

use bevy_camera::visibility::{RenderLayers, Visibility, VisibilityClass};
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh3d;
use sf_character::CharacterState;
use sf_world::{IndexedPlacement, Level, PlacementFlags, PlacementId};
use sf_world_bevy::{PlacementSpawner, SpawnCx, WorldspaceRoot};

use crate::{Character, Characters, HeldLightSlot, ShadowLayers};

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

/// Keeps every [`Character`] on the render layers of its worldspace: the
/// root's `RenderLayers` become the content layer of its nearest
/// [`WorldspaceRoot`] ancestor and its [`ShadowLayers`] that worldspace's
/// shadow-only layer (`WorldspaceLayers`), and a change is pushed to every
/// existing descendant that renders or lights ([`Layered`]): content layers,
/// except for held light slots and what hangs under them (the character's
/// lights), which take the worldspace's light layers (content plus shadow-only,
/// so they cast the character's shadow-only parts too). Parts built later copy
/// the root's layers at build time (`finish_builds`, `held::sync_held`), role
/// parts are re-layered from them (`roles::sync_role_layers`).
///
/// Why: Bevy pairs a light with shadow casters and views by `RenderLayers`
/// (a mesh without the component is layer 0), and a worldspace's lamps sit
/// on its layers only (a `[0, N]` light would light resident neighbouring
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
#[allow(clippy::type_complexity)]
pub(crate) fn follow_worldspace_layers(
    mut commands: Commands,
    characters: Query<(Entity, Option<&RenderLayers>, Option<&ShadowLayers>), With<Character>>,
    parents: Query<&ChildOf>,
    roots: Query<&WorldspaceRoot>,
    children: Query<&Children>,
    layered: Query<(), Layered>,
    slots: Query<(), With<HeldLightSlot>>,
) {
    for (character, current, current_shadow) in &characters {
        let Some(ws) = parents.iter_ancestors(character).find_map(|a| roots.get(a).ok().map(WorldspaceRoot::layers)) else { continue };
        let (layers, shadow) = (ws.content(), ShadowLayers(ws.shadow_only()));
        if current == Some(&layers) && current_shadow == Some(&shadow) {
            continue;
        }
        for part in children.iter_descendants(character) {
            if slots.contains(part) {
                for light in std::iter::once(part).chain(children.iter_descendants(part)) {
                    if layered.contains(light) {
                        commands.entity(light).try_insert(ws.light());
                    }
                }
            } else if layered.contains(part) && !parents.iter_ancestors(part).take_while(|a| *a != character).any(|a| slots.contains(a)) {
                commands.entity(part).try_insert(layers.clone());
            }
        }
        commands.entity(character).try_insert((layers, shadow));
    }
}
