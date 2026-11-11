//! Where each chain level's geometry and material are bound on the GPU, fed
//! back from the render world (B3 brief §1.1).
//!
//! A batch key is `(space, class, vertex slab, index slab, bind group)`, and
//! the slabs are known only to Bevy's render-world `MeshAllocator`, the bind
//! group only to its `RenderMaterialBindings`. So a render-world system in
//! `ExtractSchedule` reads both for the meshes and materials the main world
//! asks about and writes the answers back into the main-world
//! [`ChainSlabTable`] through `ResMut<MainWorld>`: the pattern
//! `sf_surface_bevy` and `sf_terrain_bevy` already use (`extract_writes`). It
//! is read-only on Bevy's render state and adds no ordering of its own
//! (CONTRIBUTING "Patch Bevy, don't inject into it").
//!
//! **Answered once, not per frame.** A mesh's slab id is recorded once per
//! allocation and never rewritten while it lives (slab growth,
//! `MeshAllocator::meshes_displaced_by_slab_growth`, replaces the buffer under
//! the slab and moves offsets inside it, never the slab id), and a material's
//! binding is fixed once it is prepared. So the table keeps a *pending* set,
//! the wanted ids not answered yet: [`extract_chain_slabs`] looks up only
//! those, an answered id leaves it, and with nothing pending the extract does
//! not run at all (its run condition [`chain_slabs_pending`] reads the main
//! world shared, so a steady frame takes no exclusive `MainWorld` borrow).
//! Rebuilding the wanted sets ([`ChainSlabTable::set_wanted`]) drops the
//! answers no longer wanted and marks new ids pending. A mesh the allocator
//! does not know yet stays pending and is retried every extract (the
//! residency Slabs gate bounds how long a cell waits for it).
//!
//! **Invalidation.** An asset Bevy re-allocates or re-prepares may move: a
//! mesh `Added`/`Modified`/`Removed` (freed and allocated again) or a material
//! `Modified` (e.g. a hot-reloaded texture; chain materials are otherwise
//! never modified). [`invalidate_chain_slabs`] reads those `AssetEvent`s in
//! the main world's `Last`, after `AssetEventSystems` (`PostUpdate`) emitted
//! them and before this frame's extract, drops the wanted id's answer and
//! marks it pending with a one-extract hold: Bevy re-allocates (re-prepares)
//! it in the render frame that follows this extract, so this extract would
//! still see the old slab. A re-prepared material is also looked up again for
//! [`MATERIAL_RECHECK_EXTRACTS`] extracts after its new answer, because Bevy
//! keeps the old binding while the material's textures load and moves it only
//! once a later preparation succeeds. The events are read in the main world
//! because they live there, and a render-world `Extract<>` reader cannot share
//! a system with `ResMut<MainWorld>`.
//!
//! A late answer is never *wrong*, only *absent*, which is exactly what
//! [`sf_scene::ChainSlabs`] models as `None`. A material whose textures are
//! still loading has no binding yet: Bevy retries its preparation until they
//! arrive, and the id stays pending until then.
//!
//! The industry name: a cached mesh draw command keyed on the buffer and
//! material binding, invalidated when the allocation moves (UE's
//! `FMeshDrawCommand` caching, Unity's SRP batcher); the GS port re-validates
//! every range against its batch's slabs and bind group per frame, so a stale
//! key shows up in the log instead of silently.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy_asset::{AssetEvent, AssetId};
use bevy_ecs::message::{MessageCursor, Messages};
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use bevy_render::MainWorld;
use bevy_render::material_bind_groups::RenderMaterialBindings;
use bevy_render::mesh::allocator::MeshAllocator;
use sf_scene::{ChainKey, ChainSlabs, ChainSlot, DrawSlabs};

use crate::chains::ChainAssets;

/// The index slab of a mesh without indices. Chain products are always
/// indexed; this only keeps such a mesh out of every indexed mesh's batch.
pub const NON_INDEXED: u32 = u32::MAX - 1;

/// Extracts after a re-prepared material's new answer during which it is
/// looked up again (about 2 s at 60 fps): Bevy keeps a changed material's old
/// binding while its textures load and moves it only when a later preparation
/// succeeds.
pub const MATERIAL_RECHECK_EXTRACTS: u16 = 120;

/// A mesh's vertex and index slabs in the render world's `MeshAllocator`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshSlabIds {
    pub vertex_slab: u32,
    /// [`NON_INDEXED`] for a mesh without indices.
    pub index_slab: u32,
}

/// A wanted id the render world has still to answer for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pending {
    /// Extracts to let pass before looking it up: 1 for an asset changed this
    /// frame (Bevy re-allocates it only after this frame's extract), else 0.
    pub hold: u8,
    /// Extracts after an answer during which it is looked up again, the
    /// answer replaced each time: [`MATERIAL_RECHECK_EXTRACTS`] for a
    /// re-prepared material, else 0 (it leaves pending once answered).
    pub recheck: u16,
}

/// Main world: the slabs of every chain level mesh and the bind group of every
/// chain material the scene asked about.
#[derive(Resource, Default, Debug)]
pub struct ChainSlabTable {
    /// Filled by [`extract_chain_slabs`]; only ids in [`Self::wanted`]. An
    /// answer stays while its mesh is wanted and not re-allocated.
    pub slabs: HashMap<AssetId<Mesh>, MeshSlabIds>,
    /// Filled by [`extract_chain_slabs`]: each prepared material's
    /// `MaterialBindingId::group`; only ids in [`Self::wanted_materials`]. An
    /// answer stays while its material is wanted and not re-prepared.
    pub groups: HashMap<AssetId<StandardMaterial>, u32>,
    /// The meshes to look up: every level of every resident chain and of every
    /// cell waiting for its slabs. Set by `poll_cell_loads` through
    /// [`Self::set_wanted`] when [`Self::dirty`] is set.
    pub wanted: HashSet<AssetId<Mesh>>,
    /// The materials to look up, of the same chains as [`Self::wanted`].
    pub wanted_materials: HashSet<AssetId<StandardMaterial>>,
    /// The wanted meshes [`extract_chain_slabs`] still has to look up: not
    /// answered yet, or invalidated by [`invalidate_chain_slabs`].
    pub pending: HashMap<AssetId<Mesh>, Pending>,
    /// The wanted materials still to look up, as [`Self::pending`].
    pub pending_materials: HashMap<AssetId<StandardMaterial>, Pending>,
    /// Allocator and binding lookups [`extract_chain_slabs`] has made, in
    /// total: flat on frames where nothing is pending.
    pub lookups: u64,
    /// Something changed what [`Self::wanted`] must hold.
    pub dirty: bool,
    /// A render world feeds this table. Without one (headless apps, tests)
    /// nothing ever resolves, so cells plan without waiting for slabs and
    /// every chain batches on its own (always correct, never cheap).
    pub tracking: bool,
}

impl ChainSlabTable {
    /// The draw slabs of `mesh` drawn with `material`: `None` until both are
    /// known. A chain without a material (an app without
    /// `Assets<StandardMaterial>`) is in bind group 0.
    #[must_use]
    pub fn draw(&self, mesh: AssetId<Mesh>, material: Option<AssetId<StandardMaterial>>) -> Option<DrawSlabs> {
        let s = self.slabs.get(&mesh)?;
        let bind_group = match material {
            Some(m) => *self.groups.get(&m)?,
            None => 0,
        };
        Some(DrawSlabs { vertex_slab: s.vertex_slab, index_slab: s.index_slab, bind_group })
    }

    /// Whether every mesh has its slabs.
    pub fn has_all(&self, mut meshes: impl Iterator<Item = AssetId<Mesh>>) -> bool {
        meshes.all(|m| self.slabs.contains_key(&m))
    }

    /// Whether every level mesh of `chain` has its slabs and its material its
    /// bind group: what the Slabs gate waits for.
    #[must_use]
    pub fn resolves(&self, chain: &ChainAssets) -> bool {
        self.has_all(chain.level_ids()) && chain.material_id().is_none_or(|m| self.groups.contains_key(&m))
    }

    /// Whether [`extract_chain_slabs`] has anything to look up.
    #[must_use]
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty() || !self.pending_materials.is_empty()
    }

    /// Replaces the wanted sets: answers and pending entries no longer wanted
    /// are dropped, wanted ids without an answer become pending (an id already
    /// pending keeps its hold). Clears [`Self::dirty`].
    pub fn set_wanted(&mut self, wanted: HashSet<AssetId<Mesh>>, wanted_materials: HashSet<AssetId<StandardMaterial>>) {
        rewant(&wanted, &mut self.slabs, &mut self.pending);
        rewant(&wanted_materials, &mut self.groups, &mut self.pending_materials);
        self.wanted = wanted;
        self.wanted_materials = wanted_materials;
        self.dirty = false;
    }

    /// `mesh` is re-allocated this frame: if it is wanted, drop its answer
    /// and look it up again after this frame's extract.
    pub fn invalidate_mesh(&mut self, mesh: AssetId<Mesh>) {
        if self.wanted.contains(&mesh) {
            self.slabs.remove(&mesh);
            self.pending.insert(mesh, Pending { hold: 1, recheck: 0 });
        }
    }

    /// `material` is re-prepared this frame: if it is wanted, drop its answer
    /// and look it up again after this frame's extract, then for
    /// [`MATERIAL_RECHECK_EXTRACTS`] more.
    pub fn invalidate_material(&mut self, material: AssetId<StandardMaterial>) {
        if self.wanted_materials.contains(&material) {
            self.groups.remove(&material);
            self.pending_materials.insert(material, Pending { hold: 1, recheck: MATERIAL_RECHECK_EXTRACTS });
        }
    }

    /// One extract's answers: looks up every pending mesh with `mesh_slabs`
    /// (when the render world has an allocator) and every pending material
    /// with `group` (when it has material bindings). The lookups are
    /// parameters so tests answer without a GPU.
    pub fn answer_pending(
        &mut self,
        mesh_slabs: Option<&dyn Fn(AssetId<Mesh>) -> Option<MeshSlabIds>>,
        group: Option<&dyn Fn(AssetId<StandardMaterial>) -> Option<u32>>,
    ) {
        if let Some(look) = mesh_slabs {
            answer(&mut self.pending, &mut self.slabs, &mut self.lookups, look);
        }
        if let Some(look) = group {
            answer(&mut self.pending_materials, &mut self.groups, &mut self.lookups, look);
        }
    }
}

/// [`ChainSlabTable::set_wanted`] for one id kind.
fn rewant<K: Copy + Eq + Hash, V>(wanted: &HashSet<K>, answers: &mut HashMap<K, V>, pending: &mut HashMap<K, Pending>) {
    answers.retain(|id, _| wanted.contains(id));
    pending.retain(|id, _| wanted.contains(id));
    for id in wanted {
        if !answers.contains_key(id) {
            pending.entry(*id).or_default();
        }
    }
}

/// [`ChainSlabTable::answer_pending`] for one id kind: an id answered with no
/// recheck left leaves `pending`; one the render world does not know stays.
fn answer<K: Copy + Eq + Hash, V>(
    pending: &mut HashMap<K, Pending>,
    answers: &mut HashMap<K, V>,
    lookups: &mut u64,
    look: &dyn Fn(K) -> Option<V>,
) {
    pending.retain(|&id, p| {
        if p.hold > 0 {
            p.hold -= 1;
            return true;
        }
        *lookups += 1;
        if let Some(v) = look(id) {
            answers.insert(id, v);
            if p.recheck == 0 {
                return false;
            }
            p.recheck -= 1;
        } else {
            answers.remove(&id);
        }
        true
    });
}

/// Render world, `ExtractSchedule`, under [`chain_slabs_pending`]: answers the
/// main world's [`ChainSlabTable::pending`] and
/// [`ChainSlabTable::pending_materials`].
pub fn extract_chain_slabs(
    mut main: ResMut<MainWorld>,
    allocator: Option<Res<MeshAllocator>>,
    bindings: Option<Res<RenderMaterialBindings>>,
) {
    let Some(mut table) = main.get_resource_mut::<ChainSlabTable>() else { return };
    let mesh_slabs = |id: AssetId<Mesh>| {
        allocator.as_deref()?.mesh_slabs(&id).map(|s| MeshSlabIds {
            vertex_slab: s.vertex_slab_id.id.get(),
            index_slab: s.index_slab_id.map_or(NON_INDEXED, |i| i.id.get()),
        })
    };
    let group = |id: AssetId<StandardMaterial>| bindings.as_deref()?.get(&id.untyped()).map(|b| b.group.0);
    let mesh_slabs: &dyn Fn(AssetId<Mesh>) -> Option<MeshSlabIds> = &mesh_slabs;
    let group: &dyn Fn(AssetId<StandardMaterial>) -> Option<u32> = &group;
    table.answer_pending(allocator.is_some().then_some(mesh_slabs), bindings.is_some().then_some(group));
}

/// Render-world run condition of [`extract_chain_slabs`]: the main world's
/// table has something pending. It reads `MainWorld` shared, so a steady frame
/// costs one resource lookup and no exclusive borrow of the main world.
#[must_use]
pub fn chain_slabs_pending(main: Res<MainWorld>) -> bool {
    main.get_resource::<ChainSlabTable>().is_some_and(ChainSlabTable::has_pending)
}

/// Main world, `Last`: marks the wanted meshes and materials Bevy re-allocates
/// or re-prepares this frame pending again (module docs, "Invalidation").
/// An app without `Assets<Mesh>` or `Assets<StandardMaterial>` has no such
/// events to read.
pub fn invalidate_chain_slabs(
    mut table: ResMut<ChainSlabTable>,
    meshes: Option<Res<Messages<AssetEvent<Mesh>>>>,
    materials: Option<Res<Messages<AssetEvent<StandardMaterial>>>>,
    mut mesh_cursor: Local<MessageCursor<AssetEvent<Mesh>>>,
    mut material_cursor: Local<MessageCursor<AssetEvent<StandardMaterial>>>,
) {
    if let Some(meshes) = meshes {
        for event in mesh_cursor.read(&meshes) {
            if let AssetEvent::Added { id } | AssetEvent::Modified { id } | AssetEvent::Removed { id } = *event {
                table.invalidate_mesh(id);
            }
        }
    }
    if let Some(materials) = materials {
        for event in material_cursor.read(&materials) {
            if let AssetEvent::Modified { id } = *event {
                table.invalidate_material(id);
            }
        }
    }
}

/// One chain's asset ids, as a cell's resolver needs them.
#[derive(Clone, Debug)]
pub struct ChainIds {
    pub levels: Vec<AssetId<Mesh>>,
    pub material: Option<AssetId<StandardMaterial>>,
}

impl From<&ChainAssets> for ChainIds {
    fn from(assets: &ChainAssets) -> Self {
        Self { levels: assets.level_ids().collect(), material: assets.material_id() }
    }
}

/// The resolver one cell's `plan_in` gets: chain level meshes and materials by
/// mesh key (the slots of chains this cell references first do not exist until
/// `plan_in` assigns them, so a slot-keyed view could not answer for them),
/// and the render world's answers.
///
/// (The brief's `(&ChainRegistry, &[Option<ChainAssets>], &ChainSlabTable)`
/// borrow struct cannot be built: `plan_in` takes the space `&mut`, so the
/// registry cannot be borrowed beside it.)
pub struct CellSlabs<'a> {
    pub chains: &'a HashMap<ChainKey, ChainIds>,
    pub table: &'a ChainSlabTable,
}

impl ChainSlabs for CellSlabs<'_> {
    fn slabs(&self, _chain: ChainSlot, _level: u8) -> Option<DrawSlabs> {
        None
    }

    fn slabs_keyed(&self, key: ChainKey, _chain: ChainSlot, level: u8) -> Option<DrawSlabs> {
        let chain = self.chains.get(&key)?;
        self.table.draw(*chain.levels.get(usize::from(level))?, chain.material)
    }
}
