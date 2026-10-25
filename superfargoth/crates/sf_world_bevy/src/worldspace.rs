//! One `big_space` `BigSpace` root per loaded worldspace, each with its own
//! render layer (streaming.md, "Worldspaces at runtime").
//!
//! `big_space` needs exactly one `FloatingOrigin` in every `BigSpace`. A
//! worldspace the camera is in uses the camera; every other root gets an
//! [`OriginProxy`] child that holds the origin instead (portal cameras
//! replace it in M8).

use std::collections::BTreeMap;

use bevy_camera::visibility::{RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_transform::components::{GlobalTransform, Transform};
use big_space::prelude::{BigSpace, CellCoord, FloatingOrigin};
use sf_world::WorldspaceId;

use crate::coords::world_grid;

/// Marks a worldspace's `BigSpace` root.
#[derive(Component, Clone, Debug)]
pub struct WorldspaceRoot {
    pub id: WorldspaceId,
    /// The render layer of everything in this worldspace.
    pub layer: usize,
}

/// Holds a root's `FloatingOrigin` while no camera of that worldspace does.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OriginProxy;

/// A loaded worldspace's entities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldspaceEntry {
    pub root: Entity,
    pub proxy: Entity,
    pub layer: usize,
}

/// Every worldspace root, by id. Layer 0 stays shared (sky, debug, UI
/// cameras see it); worldspaces get layers 1, 2, …
#[derive(Resource, Debug, Default)]
pub struct WorldspaceRegistry {
    entries: BTreeMap<WorldspaceId, WorldspaceEntry>,
    next_layer: usize,
}

impl WorldspaceRegistry {
    pub fn get(&self, id: &WorldspaceId) -> Option<WorldspaceEntry> {
        self.entries.get(id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WorldspaceId, &WorldspaceEntry)> {
        self.entries.iter()
    }

    /// The render layers a camera in `id` should see: the worldspace's
    /// layer plus the shared layer 0.
    pub fn camera_layers(&self, id: &WorldspaceId) -> RenderLayers {
        self.get(id).map_or_else(RenderLayers::default, |e| RenderLayers::from_layers(&[0, e.layer]))
    }

    /// The root of `id`, spawning it (and its origin proxy) on first use.
    pub fn ensure(&mut self, commands: &mut Commands, id: &WorldspaceId) -> WorldspaceEntry {
        if let Some(e) = self.get(id) {
            return e;
        }
        let layer = self.allocate_layer();
        let root = commands.spawn(root_bundle(id, layer)).id();
        let proxy = commands.spawn(proxy_bundle(root)).id();
        self.insert(id, WorldspaceEntry { root, proxy, layer })
    }

    /// [`WorldspaceRegistry::ensure`] for exclusive systems.
    pub fn ensure_in(world: &mut World, id: &WorldspaceId) -> WorldspaceEntry {
        if let Some(e) = world.resource::<Self>().get(id) {
            return e;
        }
        let layer = world.resource_mut::<Self>().allocate_layer();
        let root = world.spawn(root_bundle(id, layer)).id();
        let proxy = world.spawn(proxy_bundle(root)).id();
        world.resource_mut::<Self>().insert(id, WorldspaceEntry { root, proxy, layer })
    }

    fn allocate_layer(&mut self) -> usize {
        self.next_layer += 1;
        self.next_layer
    }

    fn insert(&mut self, id: &WorldspaceId, entry: WorldspaceEntry) -> WorldspaceEntry {
        self.entries.insert(id.clone(), entry);
        entry
    }
}

fn root_bundle(id: &WorldspaceId, layer: usize) -> impl Bundle {
    (
        BigSpace::default(),
        world_grid(),
        GlobalTransform::default(),
        Visibility::Visible,
        WorldspaceRoot { id: id.clone(), layer },
    )
}

/// Spawned without the origin: `assign_origins` gives it one if no camera
/// of the worldspace has it (never two at once).
fn proxy_bundle(root: Entity) -> impl Bundle {
    (OriginProxy, CellCoord::default(), Transform::IDENTITY, ChildOf(root))
}

/// Gives each root's proxy the floating origin exactly when no other
/// entity of that root holds one.
pub(crate) fn assign_origins(
    mut commands: Commands,
    real: Query<&ChildOf, (With<FloatingOrigin>, Without<OriginProxy>)>,
    proxies: Query<(Entity, &ChildOf, Has<FloatingOrigin>), With<OriginProxy>>,
) {
    for (proxy, parent, has_origin) in &proxies {
        let taken = real.iter().any(|c| c.parent() == parent.parent());
        if taken && has_origin {
            commands.entity(proxy).remove::<FloatingOrigin>();
        } else if !taken && !has_origin {
            commands.entity(proxy).insert(FloatingOrigin);
        }
    }
}
