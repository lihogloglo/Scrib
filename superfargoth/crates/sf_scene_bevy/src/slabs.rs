//! Where each chain level's geometry and material are bound on the GPU, fed
//! back from the render world (B3 brief §1.1).
//!
//! A batch key is `(space, class, vertex slab, index slab, bind group)`, and
//! the slabs are known only to Bevy's render-world `MeshAllocator`, the bind
//! group only to its `RenderMaterialBindings`. So a render-world system in
//! `ExtractSchedule` reads both for every mesh and material the main world
//! asks about and writes the answers back into the main-world
//! [`ChainSlabTable`] through `ResMut<MainWorld>`: the pattern
//! `sf_surface_bevy` and `sf_terrain_bevy` already use (`extract_writes`). It
//! is read-only on Bevy's render state and adds no ordering of its own
//! (CONTRIBUTING "Patch Bevy, don't inject into it").
//!
//! One frame of lag is safe: a mesh's slab id is recorded once per allocation
//! and never rewritten while it lives (growth moves offsets inside the slab,
//! not the slab), and a material's binding is fixed once it is prepared
//! (chain materials are never modified; a re-prepared one, e.g. a hot-reloaded
//! texture, may move, which the port's per-frame check logs). So a late
//! snapshot is never *wrong*, only *absent*, which is exactly what
//! [`sf_scene::ChainSlabs`] models as `None`. A material whose textures are
//! still loading has no binding yet: Bevy retries its preparation until they
//! arrive.
//!
//! The industry name: a cached mesh draw command keyed on the buffer and
//! material binding, invalidated when the allocation moves (UE's
//! `FMeshDrawCommand`, Unity's SRP batcher); the GS port re-validates every
//! range against its batch's slabs and bind group per frame, so a stale key
//! shows up in the log instead of silently.

use std::collections::{HashMap, HashSet};

use bevy_asset::AssetId;
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

/// A mesh's vertex and index slabs in the render world's `MeshAllocator`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshSlabIds {
    pub vertex_slab: u32,
    /// [`NON_INDEXED`] for a mesh without indices.
    pub index_slab: u32,
}

/// Main world: the slabs of every chain level mesh and the bind group of every
/// chain material the scene asked about.
#[derive(Resource, Default, Debug)]
pub struct ChainSlabTable {
    /// Filled by [`extract_chain_slabs`]; only ids in [`Self::wanted`].
    pub slabs: HashMap<AssetId<Mesh>, MeshSlabIds>,
    /// Filled by [`extract_chain_slabs`]: each prepared material's
    /// `MaterialBindingId::group`; only ids in [`Self::wanted_materials`].
    pub groups: HashMap<AssetId<StandardMaterial>, u32>,
    /// The meshes to look up: every level of every resident chain and of every
    /// cell waiting for its slabs. Rebuilt by `poll_cell_loads` when
    /// [`Self::dirty`] is set.
    pub wanted: HashSet<AssetId<Mesh>>,
    /// The materials to look up, of the same chains as [`Self::wanted`].
    pub wanted_materials: HashSet<AssetId<StandardMaterial>>,
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
}

/// Render world, `ExtractSchedule`: answers the main world's
/// [`ChainSlabTable::wanted`] and [`ChainSlabTable::wanted_materials`].
pub fn extract_chain_slabs(
    mut main: ResMut<MainWorld>,
    allocator: Option<Res<MeshAllocator>>,
    bindings: Option<Res<RenderMaterialBindings>>,
) {
    let Some(mut table) = main.get_resource_mut::<ChainSlabTable>() else { return };
    let table = &mut *table;
    if let Some(allocator) = allocator {
        let (slabs, wanted) = (&mut table.slabs, &table.wanted);
        slabs.retain(|id, _| wanted.contains(id));
        for id in wanted {
            match allocator.mesh_slabs(id) {
                Some(s) => {
                    let ids = MeshSlabIds {
                        vertex_slab: s.vertex_slab_id.id.get(),
                        index_slab: s.index_slab_id.map_or(NON_INDEXED, |i| i.id.get()),
                    };
                    slabs.insert(*id, ids);
                }
                None => {
                    slabs.remove(id);
                }
            }
        }
    }
    if let Some(bindings) = bindings {
        let (groups, wanted) = (&mut table.groups, &table.wanted_materials);
        groups.retain(|id, _| wanted.contains(id));
        for id in wanted {
            match bindings.get(&id.untyped()) {
                Some(binding) => {
                    groups.insert(*id, binding.group.0);
                }
                None => {
                    groups.remove(id);
                }
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
