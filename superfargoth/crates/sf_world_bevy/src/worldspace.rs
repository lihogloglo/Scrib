//! One `big_space` `BigSpace` root per loaded worldspace, each with its own
//! pair of render layers, [`WorldspaceLayers`] (streaming.md, "Worldspaces at
//! runtime"; first-person.md "Layer plan").
//!
//! `big_space` needs exactly one `FloatingOrigin` in every `BigSpace`. A
//! worldspace the camera is in uses the camera; every other root gets an
//! [`OriginProxy`] child that holds the origin instead (portal cameras
//! replace it in M8).
//!
//! A pair belongs to a worldspace's *loaded* lifetime, not to its id: streaming releases the root
//! and its pair once nothing of the worldspace is resident ([`WorldspaceRegistry::release_in`]), and
//! the next worldspace reuses the pair. Without that, every interior ever prefetched behind a door
//! kept a pair, and the 32nd one ran past Bevy's 64 inline layers (ROADMAP 1.1).

use std::collections::{BTreeMap, VecDeque};

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
    /// The render layer of everything in this worldspace ([`WorldspaceLayers::world`]).
    pub layer: usize,
}

impl WorldspaceRoot {
    pub fn layers(&self) -> WorldspaceLayers {
        WorldspaceLayers::of(self.layer)
    }
}

/// A worldspace's two render layers, allocated together by the [`WorldspaceRegistry`], and the
/// sets each kind of entity carries. Layer 0 stays shared (sky, debug, UI cameras).
///
/// - `world`: its content (meshes, terrain, water), drawn by its cameras, lit and shadowed by its lights.
/// - `shadow`, the next layer: **shadow-only**. Its lights carry it and no camera does, so a mesh
///   on it alone casts shadows (raster: Bevy pairs a light with casters by `RenderLayers`; ray
///   traced: `sf_sky_bevy` traces what the lights' layers meet) without being drawn. Bevy 0.20
///   has no shadow-only flag (UE `bCastHiddenShadow`, Unity `ShadowsOnly`); this is that, per
///   worldspace so another worldspace's lights never pick it up. First user: the first-person
///   body's head (`sf_character_bevy::RoleLayer::SHADOW`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldspaceLayers {
    pub world: usize,
    pub shadow: usize,
}

impl WorldspaceLayers {
    /// The pair whose world layer is `world` (the shadow layer is the next one).
    pub const fn of(world: usize) -> Self {
        Self { world, shadow: world + 1 }
    }

    /// Content: `[world]`.
    pub fn content(self) -> RenderLayers {
        dynamic_layer(self.world)
    }

    /// A camera in the worldspace: `[0, world]`, never the shadow layer.
    pub fn camera(self) -> RenderLayers {
        RenderLayers::from_layers(&[0, self.world])
    }

    /// A light of the worldspace (lamps, carried torches): `[world, shadow]`.
    pub fn light(self) -> RenderLayers {
        RenderLayers::from_layers(&[self.world, self.shadow])
    }

    /// A sky light shining into the worldspace (sun, moon): `[0, world, shadow]`, like its camera
    /// plus the shadow layer.
    pub fn sky_light(self) -> RenderLayers {
        RenderLayers::from_layers(&[0, self.world, self.shadow])
    }

    /// Shadow-only: `[shadow]`.
    pub fn shadow_only(self) -> RenderLayers {
        dynamic_layer(self.shadow)
    }
}

/// `[n]` for a layer computed at runtime. Bevy's const `RenderLayers::layer` asserts `n < 64` (its one
/// inline block); this grows the set instead, so more loaded worldspaces than inline pairs cost a heap
/// block, never a panic.
pub fn dynamic_layer(n: usize) -> RenderLayers {
    RenderLayers::none().with(n)
}

/// Pairs that fit Bevy's inline `RenderLayers` block (layers 1..=62; 0 is shared).
const INLINE_PAIRS: usize = 31;

/// Holds a root's `FloatingOrigin` while no camera of that worldspace does.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OriginProxy;

/// A loaded worldspace's entities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldspaceEntry {
    pub root: Entity,
    pub proxy: Entity,
    /// [`WorldspaceLayers::world`].
    pub layer: usize,
}

impl WorldspaceEntry {
    pub fn layers(&self) -> WorldspaceLayers {
        WorldspaceLayers::of(self.layer)
    }
}

/// Every loaded worldspace root, by id. Layer 0 stays shared (sky, debug, UI
/// cameras see it); worldspaces get [`WorldspaceLayers`] pairs (1, 2), (3, 4), …,
/// and a released worldspace's pair goes back to the pool.
#[derive(Resource, Debug, Default)]
pub struct WorldspaceRegistry {
    entries: BTreeMap<WorldspaceId, WorldspaceEntry>,
    next_layer: usize,
    /// Released pairs' world layers, oldest first: reused first-in first-out, so a pair stays
    /// unused as long as possible before another worldspace takes it.
    free: VecDeque<usize>,
    warned_spill: bool,
}

impl WorldspaceRegistry {
    pub fn get(&self, id: &WorldspaceId) -> Option<WorldspaceEntry> {
        self.entries.get(id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WorldspaceId, &WorldspaceEntry)> {
        self.entries.iter()
    }

    /// Loaded worldspaces (each holds one pair).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Pairs ever allocated (in use or free): the highest layer in use is at most `2 * this`.
    pub fn pairs_allocated(&self) -> usize {
        self.next_layer / 2
    }

    /// The render layers a camera in `id` should see: the worldspace's
    /// layer plus the shared layer 0 ([`WorldspaceLayers::camera`]).
    pub fn camera_layers(&self, id: &WorldspaceId) -> RenderLayers {
        self.get(id).map_or_else(RenderLayers::default, |e| e.layers().camera())
    }

    /// The render layers a light in `id` carries ([`WorldspaceLayers::light`]); layer 0 for a
    /// worldspace not loaded.
    pub fn light_layers(&self, id: &WorldspaceId) -> RenderLayers {
        self.get(id).map_or_else(RenderLayers::default, |e| e.layers().light())
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

    /// Forgets `id` and frees its pair: despawns its root and origin proxy. Refused (false) while
    /// the root holds anything but its proxy (cells, a traveler, far-field nodes, a game's own
    /// entities): those carry the pair's layers, and a recycled pair would leak them into another
    /// worldspace. True when `id` is not loaded.
    pub fn release_in(world: &mut World, id: &WorldspaceId) -> bool {
        let Some(entry) = world.resource::<Self>().get(id) else { return true };
        if world.get::<Children>(entry.root).is_some_and(|c| c.iter().any(|e| e != entry.proxy)) {
            return false;
        }
        if let Ok(root) = world.get_entity_mut(entry.root) {
            root.despawn();
        }
        let mut registry = world.resource_mut::<Self>();
        registry.entries.remove(id);
        registry.free.push_back(entry.layer);
        true
    }

    /// A free pair's world layer, else a new pair's ([`WorldspaceLayers::of`] adds the shadow
    /// layer after it).
    fn allocate_layer(&mut self) -> usize {
        if let Some(world) = self.free.pop_front() {
            return world;
        }
        let layers = WorldspaceLayers::of(self.next_layer + 1);
        self.next_layer = layers.shadow;
        if self.pairs_allocated() > INLINE_PAIRS && !std::mem::replace(&mut self.warned_spill, true) {
            log::warn!("sf_world_bevy: more than {INLINE_PAIRS} worldspaces loaded at once; render layers past 63 spill to the heap");
        }
        layers.world
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
