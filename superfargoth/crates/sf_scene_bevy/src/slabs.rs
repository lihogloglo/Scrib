//! Where each chain level's geometry is bound on the GPU, fed back from the
//! render world (B3 brief §1.1).
//!
//! A batch key is `(space, class, vertex slab, index slab, bind group)`, and
//! the slabs are known only to Bevy's render-world `MeshAllocator`. So a
//! render-world system in `ExtractSchedule` reads `MeshAllocator::mesh_slabs`
//! for every mesh the main world asks about and writes the answers back into
//! the main-world [`ChainSlabTable`] through `ResMut<MainWorld>`: the pattern
//! `sf_surface_bevy` and `sf_terrain_bevy` already use (`extract_writes`). It
//! is read-only on Bevy's render state and adds no ordering of its own
//! (CONTRIBUTING "Patch Bevy, don't inject into it").
//!
//! One frame of lag is safe: a mesh's slab id is recorded once per allocation
//! and never rewritten while it lives (growth moves offsets inside the slab,
//! not the slab), so a late snapshot is never *wrong*, only *absent* — which is
//! exactly what [`sf_scene::ChainSlabs`] models as `None`.
//!
//! The industry name: a cached mesh draw command keyed on the buffer binding,
//! invalidated when the allocation moves (UE's `FMeshDrawCommand`, Unity's
//! SRP batcher); the GS port re-validates every range against its batch's slabs
//! per frame, so a stale key shows up in the log instead of silently.

use std::collections::{HashMap, HashSet};

use bevy_asset::AssetId;
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh;
use bevy_render::MainWorld;
use bevy_render::mesh::allocator::MeshAllocator;
use sf_scene::{ChainKey, ChainSlabs, ChainSlot, DrawSlabs};

/// The index slab of a mesh without indices. Chain products are always
/// indexed; this only keeps such a mesh out of every indexed mesh's batch.
pub const NON_INDEXED: u32 = u32::MAX - 1;

/// Main world: the slabs of every chain level mesh the scene asked about.
#[derive(Resource, Default, Debug)]
pub struct ChainSlabTable {
    /// Filled by [`extract_chain_slabs`]; only ids in [`Self::wanted`].
    pub slabs: HashMap<AssetId<Mesh>, DrawSlabs>,
    /// The meshes to look up: every level of every resident chain and of every
    /// cell waiting for its slabs. Rebuilt by `poll_cell_loads` when
    /// [`Self::dirty`] is set.
    pub wanted: HashSet<AssetId<Mesh>>,
    /// Something changed what [`Self::wanted`] must hold.
    pub dirty: bool,
    /// A render world feeds this table. Without one (headless apps, tests)
    /// nothing ever resolves, so cells plan without waiting for slabs and
    /// every chain batches on its own (always correct, never cheap).
    pub tracking: bool,
}

impl ChainSlabTable {
    #[must_use]
    pub fn get(&self, mesh: AssetId<Mesh>) -> Option<DrawSlabs> {
        self.slabs.get(&mesh).copied()
    }

    /// Whether every mesh has its slabs.
    pub fn has_all(&self, mut meshes: impl Iterator<Item = AssetId<Mesh>>) -> bool {
        meshes.all(|m| self.slabs.contains_key(&m))
    }
}

/// Render world, `ExtractSchedule`: answers the main world's [`ChainSlabTable::wanted`].
pub fn extract_chain_slabs(mut main: ResMut<MainWorld>, allocator: Option<Res<MeshAllocator>>) {
    let Some(allocator) = allocator else { return };
    let Some(mut table) = main.get_resource_mut::<ChainSlabTable>() else { return };
    let table = &mut *table;
    let (slabs, wanted) = (&mut table.slabs, &table.wanted);
    slabs.retain(|id, _| wanted.contains(id));
    for id in wanted {
        match allocator.mesh_slabs(id) {
            Some(s) => {
                slabs.insert(
                    *id,
                    DrawSlabs {
                        vertex_slab: s.vertex_slab_id.id.get(),
                        index_slab: s.index_slab_id.map_or(NON_INDEXED, |i| i.id.get()),
                        // TODO(B3b): the material's bind group (`RenderMaterialBindings`), once
                        // materials exist. Until then every chain shares bind group 0, so the
                        // batch key splits on class and slabs only.
                        bind_group: 0,
                    },
                );
            }
            None => {
                slabs.remove(id);
            }
        }
    }
}

/// The resolver one cell's `plan_in` gets: chain level meshes by mesh key (the
/// slots of chains this cell references first do not exist until `plan_in`
/// assigns them, so a slot-keyed view could not answer for them), and the
/// render world's answers.
///
/// (The brief's `(&ChainRegistry, &[Option<ChainAssets>], &ChainSlabTable)`
/// borrow struct cannot be built: `plan_in` takes the space `&mut`, so the
/// registry cannot be borrowed beside it.)
pub struct CellSlabs<'a> {
    pub levels: &'a HashMap<ChainKey, Vec<AssetId<Mesh>>>,
    pub table: &'a ChainSlabTable,
}

impl ChainSlabs for CellSlabs<'_> {
    fn slabs(&self, _chain: ChainSlot, _level: u8) -> Option<DrawSlabs> {
        None
    }

    fn slabs_keyed(&self, key: ChainKey, _chain: ChainSlot, level: u8) -> Option<DrawSlabs> {
        let mesh = *self.levels.get(&key)?.get(usize::from(level))?;
        self.table.get(mesh)
    }
}
