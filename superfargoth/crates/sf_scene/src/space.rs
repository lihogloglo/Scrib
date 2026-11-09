//! The per-space scene lifecycle: a cell's table in, its slots and batches out
//! (`scene-runtime-plan.md` §4.5, static-scene.md §3).
//!
//! [`SceneSpace`] owns everything one space (a worldspace or an interior) retains on the GPU's
//! behalf, and nothing Bevy: the [`ChainRegistry`], the **scene buffer**'s [`SlotAllocator`] (one
//! contiguous range per cell, the table's 80 B records verbatim) and one allocator per **batch**
//! over that batch's fixed output slots. [`SceneSpace::plan_in`] does the whole stream-in
//! computation in one pass and returns a [`CellPlan`]; the Bevy half uploads it and calls
//! [`SceneSpace::commit_in`]. Nothing here reads the clock, the world or an asset.
//!
//! **Two buffers, not one** (plan §1.1): the scene buffer is the *input* of the per-view cull (one
//! 80 B record per instance of a resident cell), a batch's output slots are its *result* (one 64 B
//! `GpuMeshInstance` per slot, per view). Both are retained and both are allocated here, by two
//! different allocators over two different address spaces: a `SlotWrite`'s `slot` is a batch
//! output slot, `scene.start + index` is a scene-buffer slot.
//!
//! **Why a batch is more than a material class.** One indirect draw binds exactly one vertex
//! buffer, one index buffer and one material bind group
//! (`vendor/bevy_pbr/src/gpu_instance_batch/mod.rs`, `GpuBatchedMeshRange`), so a [`BatchKey`] is
//! `(space, material class, vertex slab, index slab, material bind group)` — the key the GS patch
//! batches on (plan §1 item 3). The slab and bind-group identities belong to the renderer's
//! allocators, which this crate must not name: the caller supplies them through [`ChainSlabs`],
//! and a chain level it cannot resolve yet (its geometry is still loading) gets a batch of its own
//! — exactly what [`BatchGranularity::PerChain`] does for every chain.
//!
//! **The range list is a strict partition.** [`SceneSpace::batch_ranges`] emits what
//! `validate_ranges` accepts: sorted by first slot, every span non-empty, each starting exactly
//! where the previous ended and the last ending exactly at the batch capacity. A rejected range
//! list makes the batch fall back to drawing one range for the whole capacity, which would draw
//! one chain's geometry for every instance in the batch, so the producer must never emit one. The
//! free slots between two live blocks are merged into the preceding range (the leading gap into
//! the following one) and are left `is_active = 0`, so they draw nothing.
//!
//! **First fit, measured.** The plan's §4.4 reasoned that a segregated size-class allocator
//! ([`crate::alloc::ClassAllocator`]) would hold the high-water mark down. A4 measured the
//! opposite: 3.33× the peak live slots and still creeping after 2 M stream-in cycles, against
//! 1.17× for first fit on the same walk ([`crate::alloc::ClassAllocator`]'s own docs carry the
//! numbers). **Coordinator decision 2026-11-10: the scene buffer, a batch's output slots and the
//! chain table all allocate with first fit** ([`SlotAllocator`]). What does *not* change is the
//! capacity policy: a batch's capacity is fixed at configuration and never grown at stream-in
//! (growing it would reallocate the output buffer and rebuild the materialize bind group), so
//! [`SlotAllocator::with_capacity`] answers [`AllocError::Exhausted`] and those instances are
//! left to the entity path — counted in [`SpaceStats`] and named once by
//! [`SceneSpace::take_overflow_warning`]. First fit has no a-priori ceiling and its 1.17× was
//! measured on one synthetic walk, so overflow is a path we must be able to *see*, not a corner.

use std::collections::BTreeMap;

use glam::IVec3;

use crate::PlacementId;
use crate::alloc::{AllocError, SlotAllocator, SlotRange, SlotStats};
use crate::chains::{ChainEntry, ChainError, ChainKey, ChainRegistry, ChainSlot, ChainSource, ChainStats};
use crate::eligibility::Promotions;
use crate::material::MaterialClass;
use crate::table::{StaticCellTableView, TableError};

/// Which space's buffers a batch's slots live in: spaces never share a batch (plan §1 item 3),
/// because they do not share a scene buffer or a chain table.
pub type SpaceId = u32;

/// A batch's index in its space ([`SceneSpace::batch_key`]): dense, and never reused, so the Bevy
/// half can key its batch entities on it.
pub type BatchId = u32;

/// One of the renderer's allocator identities (a `MeshAllocator` vertex or index slab, or a
/// material bind group), **opaque** to this crate: equal ids must mean the same binding, and two
/// unequal ids only ever cost an extra batch.
pub type SlabId = u32;

/// No slot: an instance whose batch was full ([`CellPlan::batch_slots`]).
pub const NO_SLOT: u32 = u32::MAX;

/// A slab identity the caller could not resolve yet (the chain's geometry is still loading), so
/// the chain gets a batch of its own ([`BatchKey::chain`]).
pub const UNRESOLVED: SlabId = u32::MAX;

/// The LOD level a resident instance draws under milestone M1 (plan §13 Q4: LOD0). M1 has no
/// per-view work items, so the CPU writes each slot once at stream-in and leaves it; M2 (the GPU
/// count/allocate/scatter pass) picks the level per view and per frame, and then a block's level
/// is the level of its *range*, which is why one lives in [`RangeSpec`] already.
pub const M1_LEVEL: u8 = 0;

/// Where one chain level's geometry is bound, in the caller's own id space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrawSlabs {
    pub vertex_slab: SlabId,
    pub index_slab: SlabId,
    pub bind_group: SlabId,
}

/// The renderer's answer to "what does this chain level bind?" — the one thing a batch key needs
/// and this crate cannot know (the `MeshAllocator` slabs and the material bind group live in
/// Bevy's render world).
///
/// `None` means "not resolvable yet": the chain's geometry is still loading, or the caller does
/// not track slabs at all. Such a chain gets its own batch, which is always correct (one chain is
/// one mesh allocation, hence one slab pair and one material) and is what
/// [`BatchGranularity::PerChain`] does for every chain.
pub trait ChainSlabs {
    fn slabs(&self, chain: ChainSlot, level: u8) -> Option<DrawSlabs>;
}

/// Resolves nothing: every chain level batches on its own.
impl ChainSlabs for () {
    fn slabs(&self, _chain: ChainSlot, _level: u8) -> Option<DrawSlabs> {
        None
    }
}

/// How instances are grouped into batches (plan §13 Q1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BatchGranularity {
    /// One batch per chain level: one range per batch, no slab resolution, the most draws. The
    /// fallback, and the oracle the per-class path is tested against.
    PerChain,
    /// One batch per `(class, slab pair, bind group)`, the default: the fewest draws the GS patch
    /// allows. A chain whose slabs [`ChainSlabs`] cannot resolve yet falls back to `PerChain` for
    /// that chain alone.
    #[default]
    PerClass,
}

/// What one indirect draw can hold: one vertex buffer, one index buffer, one material bind group,
/// in one space, for one material class (plan §1 item 3, §4.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BatchKey {
    pub space: SpaceId,
    pub class: MaterialClass,
    pub vertex_slab: SlabId,
    pub index_slab: SlabId,
    pub bind_group: SlabId,
    /// `Some((chain, level))` when this batch holds one chain level only: either
    /// [`BatchGranularity::PerChain`], or a chain whose slabs were [`UNRESOLVED`] at stream-in.
    /// The three slab fields are then [`UNRESOLVED`] and the batch binds whatever that chain
    /// level's mesh binds — guaranteed to be one of each, since a chain is one allocation.
    pub chain: Option<(ChainSlot, u8)>,
}

/// One batch output slot the Bevy half must write: instance `index` of the cell's table goes to
/// `slot` of `batch`, with `is_active = active`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotWrite {
    pub batch: BatchId,
    pub slot: u32,
    /// Index in the cell's table (its scene-buffer slot is the cell's range start plus this).
    pub index: u32,
    /// `false` leaves the slot resident but drawing nothing: a promoted placement (§7), or a
    /// placement the entity path took because another of its parts overflowed.
    pub active: bool,
}

/// One range of a batch's range list ([`SceneSpace::batch_ranges`]): the slots `first_slot
/// .. first_slot + slot_count` draw chain `chain` at level `level`.
///
/// The Bevy half turns it into a `GpuBatchedMeshRange` with
/// `{ mesh: meshes[level], first_index: 0, index_count: 0 }` (plan §3.2: a chain product is one
/// mesh per level, so a range is one whole mesh and needs no index arithmetic).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeSpec {
    pub first_slot: u32,
    pub slot_count: u32,
    pub chain: ChainSlot,
    pub level: u8,
}

/// Everything stream-in must do for one cell, computed in one pass by [`SceneSpace::plan_in`].
///
/// The allocations are already made when the plan is returned: publish it and
/// [`SceneSpace::commit_in`], or drop it and [`SceneSpace::cancel_in`].
#[derive(Clone, Debug, PartialEq)]
pub struct CellPlan {
    /// The cell's contiguous scene-buffer range (`len == 0` for an empty table).
    pub scene: SlotRange,
    /// Cell-local chain index → global chain slot, in index order: the argument of
    /// [`StaticCellTableView::write_remapped`], one slot for **every** chain of the table,
    /// including chains whose batch overflowed.
    pub chain_slots: Vec<ChainSlot>,
    /// Chains whose first reference this cell is: the Bevy half starts their asset loads.
    pub load: Vec<(ChainSlot, ChainSource)>,
    /// Batches this cell needs, with their keys: the Bevy half makes sure each exists.
    pub batches: Vec<(BatchId, BatchKey)>,
    /// The scene-buffer upload: the table's instance section with the global chain slots in
    /// place, 80 B per instance, cell-relative.
    pub scene_bytes: Vec<u8>,
    /// The batch output slots to write, in table order.
    pub writes: Vec<SlotWrite>,
    /// Table index → batch output slot, [`NO_SLOT`] where the batch was full. The promotion
    /// index: with [`CellResidency::blocks`] it names the `(batch, slot)` of any part of any
    /// placement without a search of the table.
    pub batch_slots: Vec<u32>,
    /// The blocks allocated for this cell, `(batch, slots, chain, level)`: what
    /// [`SceneSpace::plan_out`] releases.
    pub blocks: Vec<(BatchId, SlotRange, ChainSlot, u8)>,
    /// Batches whose range list changed: the Bevy half rebuilds them from
    /// [`SceneSpace::batch_ranges`] **after** `commit_in`.
    pub ranges: Vec<BatchId>,
    /// Placements the entity path must not spawn: every part of them is in the scene. A placement
    /// one of whose parts overflowed is **not** here — it spawns whole, and its other parts' slots
    /// are written inactive.
    pub covered: Vec<PlacementId>,
    /// The batches that were full, once each: `scene/overflow/*` and the one-time warning.
    pub overflowed: Vec<BatchKey>,
}

/// What [`SceneSpace::plan_out`] freed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellRelease {
    /// The cell's scene-buffer range, released (reusable after the frames in flight).
    pub scene: SlotRange,
    /// Chains whose last resident cell this was: the Bevy half drops their handles.
    pub freed_chains: Vec<ChainSlot>,
    /// Batches whose range list changed.
    pub ranges: Vec<BatchId>,
}

/// What a space keeps about one resident cell.
#[derive(Clone, Debug, PartialEq)]
pub struct CellResidency {
    /// Instances in the cell's table.
    pub table_len: u32,
    pub scene: SlotRange,
    /// Cell-local chain index → global chain slot.
    pub chain_slots: Vec<ChainSlot>,
    /// Table index → batch output slot, [`NO_SLOT`] where the batch was full.
    pub batch_slots: Vec<u32>,
    /// `(batch, slots, chain, level)` per bucket of this cell.
    pub blocks: Vec<(BatchId, SlotRange, ChainSlot, u8)>,
    /// Placements this cell has handed to the entity path, one way (§7).
    pub promotions: Promotions,
}

/// Occupancy of one space (`scene/*`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceStats {
    pub cells: u32,
    /// The scene buffer: one contiguous range per resident cell.
    pub scene: SlotStats,
    /// The chain table; its `high_water` is the length of the Bevy half's per-slot side table.
    pub chains: ChainStats,
    pub batches: u32,
    /// Output slots in live blocks, summed over the batches.
    pub batch_live_slots: u32,
    /// Output slots ever handed out, summed over the batches: `batch_live_slots` at its peak is
    /// what perfect packing would need, this is what first fit needs.
    pub batch_high_water: u32,
    /// Output slots the batches reserve: `batches × batch_capacity`, **per view**.
    pub batch_capacity: u32,
    /// Resident cells that overflowed at least one batch.
    pub overflow_cells: u32,
    /// Instances left to the entity path because their batch was full.
    pub overflow_instances: u32,
    /// Placements promoted to entities, summed over the resident cells (§7).
    pub promoted: u32,
}

/// Why a cell could not stream in or out.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SceneError {
    /// The table says something `plan_in` cannot act on. (`parse` has already rejected everything
    /// it can see; this is the rest.)
    #[error("the cell's table cannot be streamed in: {0}")]
    Table(#[from] TableError),
    /// Two cells disagree about one chain, or the chain table is full: fails *this cell*, whose
    /// placements then spawn as entities (plan §5.3).
    #[error(transparent)]
    Chain(#[from] ChainError),
    /// The scene buffer refused the cell's range.
    #[error("the space's instance table refused the cell: {0}")]
    Scene(AllocError),
    #[error("cell {0:?} is already resident")]
    Resident(IVec3),
    #[error("cell {0:?} is not resident")]
    NotResident(IVec3),
}

/// How a space allocates (plan §13 Q1, Q2, Q5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpaceConfig {
    pub space: SpaceId,
    pub granularity: BatchGranularity,
    /// Frames the GPU may still be reading a released slot: every reuse waits that long.
    pub frames_in_flight: u32,
    /// Output slots per batch, **fixed here and never grown at stream-in** (plan §4.4): growing
    /// it would reallocate the batch's output buffer and rebuild its materialize bind group.
    pub batch_capacity: u32,
}

impl Default for SpaceConfig {
    fn default() -> Self {
        Self { space: 0, granularity: BatchGranularity::PerClass, frames_in_flight: 3, batch_capacity: 4096 }
    }
}

/// One batch: its key, its fixed output slots, and what lives in them.
#[derive(Clone, Debug)]
struct Batch {
    key: BatchKey,
    /// First fit over the batch's fixed capacity (the module docs' measurement).
    slots: SlotAllocator,
    /// Live blocks: first slot → `(chain, level)`. Exactly the allocator's live ranges, so
    /// [`SceneSpace::batch_ranges`] needs no second source of truth for the slot layout.
    blocks: BTreeMap<u32, (ChainSlot, u8)>,
}

/// One space's retained scene: the chain table, the scene buffer, the batches and the resident
/// cells (module docs).
#[derive(Clone, Debug)]
pub struct SceneSpace {
    config: SpaceConfig,
    chains: ChainRegistry,
    scene: SlotAllocator,
    /// Indexed by [`BatchId`].
    batches: Vec<Batch>,
    by_key: BTreeMap<BatchKey, BatchId>,
    /// Keyed by the cell's array form: `IVec3` is not `Ord`, and iteration must be deterministic.
    cells: BTreeMap<[i32; 3], CellResidency>,
    frame: u64,
    overflow_cells: u32,
    overflow_instances: u32,
    first_overflow: Option<BatchKey>,
}

impl SceneSpace {
    #[must_use]
    pub fn new(config: SpaceConfig) -> Self {
        Self {
            chains: ChainRegistry::new(config.frames_in_flight),
            scene: SlotAllocator::new(config.frames_in_flight),
            batches: Vec::new(),
            by_key: BTreeMap::new(),
            cells: BTreeMap::new(),
            frame: 0,
            overflow_cells: 0,
            overflow_instances: 0,
            first_overflow: None,
            config,
        }
    }

    #[must_use]
    pub fn config(&self) -> SpaceConfig {
        self.config
    }

    /// The space's chain table (the Bevy half reads a slot's [`ChainEntry`] to load its geometry).
    #[must_use]
    pub fn chains(&self) -> &ChainRegistry {
        &self.chains
    }

    #[must_use]
    pub fn batch_key(&self, batch: BatchId) -> Option<BatchKey> {
        self.batches.get(batch as usize).map(|b| b.key)
    }

    /// Every batch, in id order.
    pub fn batch_keys(&self) -> impl Iterator<Item = (BatchId, BatchKey)> + '_ {
        (0u32..).zip(self.batches.iter().map(|b| b.key))
    }

    #[must_use]
    pub fn resident(&self, cell: IVec3) -> Option<&CellResidency> {
        self.cells.get(&cell.to_array())
    }

    /// Every resident cell, in cell order.
    pub fn cells(&self) -> impl Iterator<Item = (IVec3, &CellResidency)> {
        self.cells.iter().map(|(&c, r)| (IVec3::from_array(c), r))
    }

    /// Plans a cell's stream-in: acquires its chain slots, allocates its scene range and its
    /// batch blocks, and returns everything the Bevy half must upload ([`CellPlan`]).
    ///
    /// One pass over the table plus one per bucket; three vectors, nothing per instance beyond
    /// the plan's own. **On the one reachable error — two cells disagreeing about a chain — the
    /// space is left exactly as it was**: every chain of the table that was already resident is
    /// acquired first, before anything is allocated, so a disagreement unwinds to refcounts
    /// alone. (The remaining errors need a 2^32-slot table; those unwind too, but a rolled-back
    /// range then waits out the frames in flight like any other release.)
    ///
    /// `slabs` answers for the chain levels whose geometry the caller has already resolved; pass
    /// `&()` to batch every chain on its own.
    pub fn plan_in(
        &mut self,
        cell: IVec3,
        table: &StaticCellTableView<'_>,
        slabs: &impl ChainSlabs,
    ) -> Result<CellPlan, SceneError> {
        if self.cells.contains_key(&cell.to_array()) {
            return Err(SceneError::Resident(cell));
        }
        let table_len = u32::try_from(table.len()).map_err(|_| TableError::TooLarge)?;
        let chain_count = table.chain_count();
        // `parse` validated the chain section, so every index below names a chain.
        let entries: Vec<ChainEntry> = (0..u32::try_from(chain_count).unwrap_or(u32::MAX))
            .map(|c| ChainEntry::from_table(table, c).expect("a parsed table holds chain_count chains"))
            .collect();

        let (chain_slots, load, taken) = self.acquire_chains(&entries)?;
        let scene = if table_len == 0 {
            SlotRange { start: 0, len: 0 }
        } else {
            match self.scene.alloc(table_len) {
                Ok(range) => range,
                Err(e) => {
                    self.undo_chains(&taken);
                    return Err(SceneError::Scene(e));
                }
            }
        };
        let mut scene_bytes = Vec::with_capacity(table.instance_bytes().len());
        // One slot per chain, in index order: `write_remapped` panics otherwise.
        table.write_remapped(&chain_slots, &mut scene_bytes);

        // Bucket the instances by (batch, chain, level). The table is sorted by (placement, part),
        // not by chain, so a chain's instances are not contiguous and a bucket must be collected.
        let mut buckets: BTreeMap<(BatchKey, ChainSlot, u8), Vec<u32>> = BTreeMap::new();
        let mut placements: Vec<PlacementId> = Vec::with_capacity(table_len as usize);
        for (index, instance) in (0u32..).zip(table.instances()) {
            placements.push(instance.placement);
            // Validated: every instance's chain index is in range.
            let local = instance.chain as usize;
            let chain = chain_slots[local];
            let level = top_level(entries[local].levels.len());
            let key = self.key_of(chain, level, entries[local].class, slabs);
            buckets.entry((key, chain, level)).or_default().push(index);
        }

        let mut batch_slots = vec![NO_SLOT; table_len as usize];
        let mut batch_ids = vec![BatchId::MAX; table_len as usize];
        let mut blocks: Vec<(BatchId, SlotRange, ChainSlot, u8)> = Vec::new();
        let mut batches: BTreeMap<BatchId, BatchKey> = BTreeMap::new();
        let mut overflowed: Vec<BatchKey> = Vec::new();
        for ((key, chain, level), indices) in buckets {
            let id = self.batch_of(key);
            let len = u32::try_from(indices.len()).unwrap_or(u32::MAX);
            let batch = &mut self.batches[id as usize];
            match batch.slots.alloc(len) {
                Ok(range) => {
                    batch.blocks.insert(range.start, (chain, level));
                    blocks.push((id, range, chain, level));
                    batches.insert(id, key);
                    for (slot, &index) in (range.start..range.end()).zip(&indices) {
                        batch_slots[index as usize] = slot;
                        batch_ids[index as usize] = id;
                    }
                }
                // The batch is full: these instances spawn as entities (plan §5.3).
                Err(AllocError::Exhausted { .. }) => overflowed.push(key),
                Err(e) => {
                    // Unreachable: a bucket is never empty and a capped allocator cannot overflow
                    // `u32`. Unwound anyway, so no error path can leak a slot.
                    self.undo_blocks(&blocks);
                    if scene.len > 0 {
                        self.release_scene(scene);
                    }
                    self.undo_chains(&taken);
                    return Err(SceneError::Scene(e));
                }
            }
        }

        // A placement is covered only if *every* part of it got a slot: a partly resident
        // placement would be drawn twice, once by the scene and once by the entity it spawns.
        let mut covered: Vec<PlacementId> = Vec::new();
        let mut active = vec![true; table_len as usize];
        let mut first = 0usize;
        while first < placements.len() {
            let placement = placements[first];
            let end = placements[first..].iter().position(|&p| p != placement).map_or(placements.len(), |n| first + n);
            if (first..end).all(|k| batch_slots[k] != NO_SLOT) {
                covered.push(placement);
            } else {
                for flag in &mut active[first..end] {
                    *flag = false;
                }
            }
            first = end;
        }

        let mut writes: Vec<SlotWrite> = Vec::new();
        for index in 0..table_len {
            let slot = batch_slots[index as usize];
            if slot == NO_SLOT {
                continue;
            }
            writes.push(SlotWrite { batch: batch_ids[index as usize], slot, index, active: active[index as usize] });
        }

        let ranges: Vec<BatchId> = batches.keys().copied().collect();
        Ok(CellPlan {
            scene,
            chain_slots,
            load,
            batches: batches.into_iter().collect(),
            scene_bytes,
            writes,
            batch_slots,
            blocks,
            ranges,
            covered,
            overflowed,
        })
    }

    /// Records a published plan as resident. `plan` must be the last plan
    /// [`Self::plan_in`] returned for `cell`.
    pub fn commit_in(&mut self, cell: IVec3, plan: &CellPlan) {
        let overflowed = plan.batch_slots.iter().filter(|&&slot| slot == NO_SLOT).count();
        if !plan.overflowed.is_empty() {
            self.overflow_cells += 1;
            self.overflow_instances += u32::try_from(overflowed).unwrap_or(u32::MAX);
            if self.first_overflow.is_none() {
                self.first_overflow = plan.overflowed.first().copied();
            }
        }
        let residency = CellResidency {
            table_len: u32::try_from(plan.batch_slots.len()).unwrap_or(u32::MAX),
            scene: plan.scene,
            chain_slots: plan.chain_slots.clone(),
            batch_slots: plan.batch_slots.clone(),
            blocks: plan.blocks.clone(),
            promotions: Promotions::new(),
        };
        let previous = self.cells.insert(cell.to_array(), residency);
        debug_assert!(previous.is_none(), "committing a plan over resident cell {cell:?}");
    }

    /// Gives back everything a plan allocated, for a load that was cancelled between
    /// [`Self::plan_in`] and [`Self::commit_in`] (plan §5.3). The slots wait out the frames in
    /// flight like any other release, which costs nothing: they were never uploaded.
    pub fn cancel_in(&mut self, plan: &CellPlan) {
        self.undo_blocks(&plan.blocks);
        if plan.scene.len > 0 {
            self.release_scene(plan.scene);
        }
        for &slot in &plan.chain_slots {
            self.release_chain_slot(slot);
        }
    }

    /// Streams a cell out: releases its scene range, its blocks and its chain references.
    ///
    /// Everything released waits the frames in flight before it is reused, so a frame still
    /// reading the old contents can never see new geometry in them.
    pub fn plan_out(&mut self, cell: IVec3) -> Result<CellRelease, SceneError> {
        let residency = self.cells.remove(&cell.to_array()).ok_or(SceneError::NotResident(cell))?;
        let mut freed_chains = Vec::new();
        for &slot in &residency.chain_slots {
            if let Some(freed) = self.release_chain_slot(slot) {
                freed_chains.push(freed);
            }
        }
        let mut ranges: Vec<BatchId> = residency.blocks.iter().map(|&(id, ..)| id).collect();
        ranges.sort_unstable();
        ranges.dedup();
        self.undo_blocks(&residency.blocks);
        if residency.scene.len > 0 {
            self.release_scene(residency.scene);
        }
        Ok(CellRelease { scene: residency.scene, freed_chains, ranges })
    }

    /// Promotes one placement of a resident cell to the entity path (§7): its slots stay
    /// allocated and become inactive, so the GPU draws nothing for it from this frame on.
    ///
    /// One way and idempotent: a second call returns no write.
    pub fn promote(
        &mut self,
        cell: IVec3,
        placement: PlacementId,
        table: &StaticCellTableView<'_>,
    ) -> Result<Vec<SlotWrite>, SceneError> {
        let Some(residency) = self.cells.get_mut(&cell.to_array()) else {
            return Err(SceneError::NotResident(cell));
        };
        if !residency.promotions.promote(placement) {
            return Ok(Vec::new());
        }
        let mut writes = Vec::new();
        for index in table.find_all(placement) {
            let slot = residency.batch_slots.get(index).copied().unwrap_or(NO_SLOT);
            if slot == NO_SLOT {
                continue;
            }
            // The instance's chain and level name its block, and the block names its batch: a
            // slot number alone would not, since every batch numbers its slots from zero.
            let Some(instance) = table.instance(index) else { continue };
            let chain = residency.chain_slots.get(instance.chain as usize).copied().unwrap_or(NO_SLOT);
            let level = top_level(table.levels(instance.chain).map_or(1, |levels| levels.len()));
            let block = residency.blocks.iter().find(|&&(_, range, c, l)| {
                c == chain && l == level && range.start <= slot && slot < range.end()
            });
            if let Some(&(batch, ..)) = block {
                writes.push(SlotWrite { batch, slot, index: u32::try_from(index).unwrap_or(u32::MAX), active: false });
            } else {
                debug_assert!(false, "slot {slot} of cell {cell:?} is in no block of chain {chain}");
            }
        }
        Ok(writes)
    }

    /// The batch's range list: a **strict partition** of its `0..batch_capacity` output slots,
    /// sorted by first slot, every span non-empty, each starting exactly where the previous ended
    /// and the last ending at the capacity — the contract `validate_ranges` enforces
    /// (`vendor/bevy_pbr/src/gpu_instance_batch/mod.rs`).
    ///
    /// One range per live block, in slot order. The free slots between two blocks are **merged
    /// into the preceding range** (and the leading gap into the following one), rather than given
    /// filler ranges: there is no mesh to name in a hole, and those slots are never written
    /// active, so they draw nothing.
    ///
    /// Empty when the batch holds nothing: an empty range list is valid and means exactly
    /// upstream behaviour (the whole batch draws its own mesh), which for a batch with no active
    /// slot draws nothing.
    #[must_use]
    pub fn batch_ranges(&self, batch: BatchId) -> Vec<RangeSpec> {
        let Some(b) = self.batches.get(batch as usize) else { return Vec::new() };
        let capacity = self.config.batch_capacity;
        let mut out: Vec<RangeSpec> = Vec::with_capacity(b.blocks.len());
        for (&start, &(chain, level)) in &b.blocks {
            // The first range absorbs the leading gap; every other starts at its block.
            let first_slot = if out.is_empty() { 0 } else { start };
            if let Some(previous) = out.last_mut() {
                previous.slot_count = first_slot - previous.first_slot;
            }
            out.push(RangeSpec { first_slot, slot_count: 0, chain, level });
        }
        if let Some(last) = out.last_mut() {
            // A block never starts past the capacity, so this span is never empty.
            last.slot_count = capacity.saturating_sub(last.first_slot);
        }
        out
    }

    /// Frees everything whose frames in flight are done by `frame`, and makes `frame` the frame
    /// later releases are stamped with.
    pub fn advance(&mut self, frame: u64) {
        self.frame = frame;
        self.scene.advance(frame);
        self.chains.advance(frame);
        for batch in &mut self.batches {
            batch.slots.advance(frame);
        }
    }

    /// The batch key of the first batch that ever overflowed, once: the caller warns with it and
    /// never again (this crate has no logger). [`SpaceStats`] keeps counting.
    pub fn take_overflow_warning(&mut self) -> Option<BatchKey> {
        self.first_overflow.take()
    }

    #[must_use]
    pub fn stats(&self) -> SpaceStats {
        let mut batch_live_slots = 0u32;
        let mut batch_high_water = 0u32;
        for batch in &self.batches {
            let slots = batch.slots.stats();
            batch_live_slots += slots.live_slots;
            batch_high_water += slots.high_water;
        }
        let batches = u32::try_from(self.batches.len()).unwrap_or(u32::MAX);
        SpaceStats {
            cells: u32::try_from(self.cells.len()).unwrap_or(u32::MAX),
            scene: self.scene.stats(),
            chains: self.chains.stats(),
            batches,
            batch_live_slots,
            batch_high_water,
            batch_capacity: batches.saturating_mul(self.config.batch_capacity),
            overflow_cells: self.overflow_cells,
            overflow_instances: self.overflow_instances,
            promoted: self
                .cells
                .values()
                .map(|c| u32::try_from(c.promotions.len()).unwrap_or(u32::MAX))
                .fold(0, u32::saturating_add),
        }
    }

    /// Acquires a chain slot for every chain of the table, resident chains **first**: a
    /// disagreement is then found before anything has been allocated, and unwinds to refcounts
    /// alone. Slots are handed out in cell-local chain order, so the same cell always gets the
    /// same slots (a `Shared` acquire takes no slot, so the order is the same as a single pass's).
    fn acquire_chains(&mut self, entries: &[ChainEntry]) -> Result<AcquiredChains, SceneError> {
        let mut slots = vec![NO_SLOT; entries.len()];
        let mut load = Vec::new();
        let mut taken: Vec<ChainKey> = Vec::with_capacity(entries.len());
        for resident_first in [true, false] {
            for (index, entry) in entries.iter().enumerate() {
                if (slots[index] != NO_SLOT) || (self.chains.slot(entry.key).is_some() != resident_first) {
                    continue;
                }
                match self.chains.acquire(entry.clone()) {
                    Ok(acquired) => {
                        slots[index] = acquired.slot();
                        taken.push(entry.key);
                        if acquired.is_new() {
                            load.push((acquired.slot(), entry.source.clone()));
                        }
                    }
                    Err(e) => {
                        self.undo_chains(&taken);
                        return Err(SceneError::Chain(e));
                    }
                }
            }
        }
        Ok((slots, load, taken))
    }

    /// The batch key of one chain level: the renderer's slabs when the caller can resolve them,
    /// else the chain's own batch.
    fn key_of(&self, chain: ChainSlot, level: u8, class: MaterialClass, slabs: &impl ChainSlabs) -> BatchKey {
        let resolved = match self.config.granularity {
            BatchGranularity::PerChain => None,
            BatchGranularity::PerClass => slabs.slabs(chain, level),
        };
        match resolved {
            Some(d) => BatchKey {
                space: self.config.space,
                class,
                vertex_slab: d.vertex_slab,
                index_slab: d.index_slab,
                bind_group: d.bind_group,
                chain: None,
            },
            None => BatchKey {
                space: self.config.space,
                class,
                vertex_slab: UNRESOLVED,
                index_slab: UNRESOLVED,
                bind_group: UNRESOLVED,
                chain: Some((chain, level)),
            },
        }
    }

    /// The batch with this key, created empty if it is new.
    fn batch_of(&mut self, key: BatchKey) -> BatchId {
        if let Some(&id) = self.by_key.get(&key) {
            return id;
        }
        let id = u32::try_from(self.batches.len()).unwrap_or(u32::MAX);
        let slots = SlotAllocator::with_capacity(self.config.batch_capacity, self.config.frames_in_flight);
        self.batches.push(Batch { key, slots, blocks: BTreeMap::new() });
        self.by_key.insert(key, id);
        id
    }

    fn undo_blocks(&mut self, blocks: &[(BatchId, SlotRange, ChainSlot, u8)]) {
        for &(id, range, ..) in blocks {
            let frame = self.frame;
            let batch = &mut self.batches[id as usize];
            batch.blocks.remove(&range.start);
            if let Err(e) = batch.slots.release(range, frame) {
                debug_assert!(false, "releasing block {range:?} of batch {id}: {e}");
            }
        }
    }

    fn release_scene(&mut self, range: SlotRange) {
        if let Err(e) = self.scene.release(range, self.frame) {
            debug_assert!(false, "releasing scene range {range:?}: {e}");
        }
    }

    /// One reference less to whatever chain is in `slot`; `Some(slot)` when it was the last.
    fn release_chain_slot(&mut self, slot: ChainSlot) -> Option<ChainSlot> {
        let key = self.chains.entry(slot)?.key;
        match self.chains.release(key, self.frame) {
            Ok(freed) => freed,
            Err(e) => {
                // Our own acquires put the key there, so it is resident.
                debug_assert!(false, "releasing chain {key:#018x}: {e}");
                None
            }
        }
    }

    /// Unwinds the acquires of a failed [`Self::plan_in`], newest first.
    fn undo_chains(&mut self, taken: &[ChainKey]) {
        for &key in taken.iter().rev() {
            if let Err(e) = self.chains.release(key, self.frame) {
                debug_assert!(false, "unwinding chain {key:#018x}: {e}");
            }
        }
    }
}

/// What [`SceneSpace::acquire_chains`] took: a slot per cell-local chain, the chains to load, and
/// the keys to unwind if the rest of the plan fails.
type AcquiredChains = (Vec<ChainSlot>, Vec<(ChainSlot, ChainSource)>, Vec<ChainKey>);

/// The level a resident instance draws, clamped to a chain of `levels` levels ([`M1_LEVEL`]).
fn top_level(levels: usize) -> u8 {
    if usize::from(M1_LEVEL) < levels {
        M1_LEVEL
    } else {
        u8::try_from(levels.saturating_sub(1)).unwrap_or(u8::MAX)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;
    use crate::table::{Aabb, InstanceFlags, LodChain, LodLevel, Sphere, StaticCellTable, StaticInstance};

    /// `(mesh key, model index, part, class)`.
    type ChainSpec = (u64, u16, u32, MaterialClass);

    /// A caller that resolves every chain level to the same `(vertex slab, index slab, bind
    /// group)`, so only the material class can split a batch.
    struct Slabs(SlabId, SlabId, SlabId);

    impl ChainSlabs for Slabs {
        fn slabs(&self, _chain: ChainSlot, _level: u8) -> Option<DrawSlabs> {
            Some(DrawSlabs { vertex_slab: self.0, index_slab: self.1, bind_group: self.2 })
        }
    }

    fn levels() -> Vec<LodLevel> {
        vec![
            LodLevel { first_index: 0, index_count: 300, error: 0.0 },
            LodLevel { first_index: 300, index_count: 150, error: 0.05 },
        ]
    }

    /// A synthetic table: `models` sorted and unique, `chains` sorted by mesh key, `instances`
    /// sorted by placement, as the format requires.
    fn table_bytes(models: &[&str], chains: &[ChainSpec], instances: &[(PlacementId, u32)]) -> Vec<u8> {
        let chains = chains
            .iter()
            .map(|&(mesh, model, part, class)| LodChain {
                mesh,
                material: mesh ^ 0x5A5A,
                material_class: class,
                model,
                part,
                levels: levels(),
            })
            .collect();
        let instances = instances
            .iter()
            .map(|&(placement, chain)| StaticInstance {
                placement,
                transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
                sphere: Sphere { center: Vec3::ZERO, radius: 1.0 },
                chain,
                material_override: 0,
                flags: InstanceFlags::CASTS_SHADOW,
            })
            .collect();
        StaticCellTable {
            origin_cell: IVec3::ZERO,
            aabb: Aabb { min: Vec3::splat(-1.0), max: Vec3::splat(1.0) },
            land_min: 0.0,
            layers: 1,
            instances,
            chains,
            models: models.iter().map(|m| (*m).to_owned()).collect(),
        }
        .write()
        .expect("a valid synthetic table")
    }

    /// Two chains of one model, three placements: placement 1 has both parts.
    fn two_chain_table() -> Vec<u8> {
        table_bytes(
            &["meshes/a.mdl"],
            &[(0x10, 0, 0, MaterialClass::Opaque), (0x20, 0, 1, MaterialClass::Masked)],
            &[(1, 0), (1, 1), (2, 0), (3, 1)],
        )
    }

    fn space(granularity: BatchGranularity, capacity: u32) -> SceneSpace {
        SceneSpace::new(SpaceConfig { granularity, batch_capacity: capacity, ..SpaceConfig::default() })
    }

    /// The vendor's `validate_ranges` (`vendor/bevy_pbr/src/gpu_instance_batch/mod.rs`), which a
    /// Bevy-free crate cannot call: sorted by first slot, no empty span, no gap, no overlap, and
    /// the last span ends exactly at the capacity. Kept line for line with it.
    fn assert_partitions(ranges: &[RangeSpec], capacity: u32) {
        if ranges.is_empty() {
            return;
        }
        let mut expected = 0u32;
        for (index, range) in ranges.iter().enumerate() {
            assert_ne!(range.slot_count, 0, "range {index} is empty");
            assert_eq!(range.first_slot, expected, "range {index} is not contiguous");
            expected = range.first_slot + range.slot_count;
        }
        assert_eq!(expected, capacity, "the ranges do not cover the capacity");
    }

    /// The chain slot each instance's record carries in a remapped scene-buffer copy.
    fn remapped_slots(bytes: &[u8]) -> Vec<u32> {
        bytes
            .as_chunks::<80>()
            .0
            .iter()
            .map(|record| u32::from_le_bytes(record[68..72].try_into().expect("four bytes")))
            .collect()
    }

    #[test]
    fn plan_in_remaps_every_chain_index() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 64);
        let plan = s.plan_in(IVec3::new(1, 0, 2), &view, &()).unwrap();
        // One global slot per cell-local chain, in index order, both of them new.
        assert_eq!(plan.chain_slots, [0, 1]);
        assert_eq!(plan.load.iter().map(|(slot, _)| *slot).collect::<Vec<_>>(), [0, 1]);
        assert_eq!(plan.load[0].1.subject(), "meshes/a.mdl/0");
        assert_eq!(plan.load[1].1.subject(), "meshes/a.mdl/1");
        // The upload is the instance section with the global slots in place, 80 B each.
        assert_eq!(plan.scene_bytes.len(), 4 * 80);
        assert_eq!(remapped_slots(&plan.scene_bytes), [0, 1, 0, 1]);
        assert_eq!(plan.scene, SlotRange { start: 0, len: 4 });
        // Every placement is covered, and every instance has a batch slot.
        assert_eq!(plan.covered, [1, 2, 3]);
        assert!(plan.overflowed.is_empty());
        assert!(plan.batch_slots.iter().all(|&slot| slot != NO_SLOT));
        assert!(plan.writes.iter().all(|w| w.active));
        // Per chain: two batches, each with the two instances of its own chain.
        assert_eq!(plan.batches.len(), 2);
        assert_eq!(plan.blocks.len(), 2);
        s.commit_in(IVec3::new(1, 0, 2), &plan);
        assert_eq!(s.stats().cells, 1);
        assert_eq!(s.stats().chains, ChainStats { live: 2, pending: 0, high_water: 2 });
        assert_eq!(s.stats().batch_live_slots, 4);
        // A second plan for a resident cell is refused, and nothing moved.
        let before = s.stats();
        assert_eq!(s.plan_in(IVec3::new(1, 0, 2), &view, &()), Err(SceneError::Resident(IVec3::new(1, 0, 2))));
        assert_eq!(s.stats(), before);
    }

    #[test]
    fn plan_in_is_atomic_on_error() {
        // Two cells whose tables disagree about chain 0x10: the same mesh key, a different part.
        let first = table_bytes(&["meshes/a.mdl"], &[(0x10, 0, 0, MaterialClass::Opaque)], &[(1, 0)]);
        let second = table_bytes(
            &["meshes/a.mdl"],
            &[(0x10, 0, 7, MaterialClass::Opaque), (0x20, 0, 0, MaterialClass::Opaque)],
            &[(1, 0), (2, 1)],
        );
        let (first, second) = (
            StaticCellTableView::parse(&first).unwrap(),
            StaticCellTableView::parse(&second).unwrap(),
        );
        let mut s = space(BatchGranularity::PerClass, 128);
        let plan = s.plan_in(IVec3::ZERO, &first, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        let before = s.stats();
        let error = s.plan_in(IVec3::X, &second, &()).unwrap_err();
        assert_eq!(error, SceneError::Chain(ChainError::Disagreement { key: 0x10, field: "part" }));
        // Exactly as it was: no refcount, no slot, no scene range, no block, no new batch.
        assert_eq!(s.stats(), before);
        assert_eq!(s.chains().refs(0x10), Some(1));
        assert_eq!(s.chains().slot(0x20), None);
        assert_eq!(s.resident(IVec3::X), None);
        // And the cell can still stream in once its product is rebaked.
        let plan = s.plan_in(IVec3::X, &first, &()).unwrap();
        assert_eq!(plan.chain_slots, [0]);
        assert_eq!(s.chains().refs(0x10), Some(2));
    }

    #[test]
    fn batch_slots_index_promotion() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 64);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        // Table order is (placement, part): chain 0 holds instances 0 and 2, chain 1 holds 1 and 3.
        let residency = s.resident(IVec3::ZERO).unwrap();
        assert_eq!(residency.table_len, 4);
        assert_eq!(residency.batch_slots, [0, 0, 1, 1]);
        assert_eq!(residency.chain_slots, [0, 1]);
        // The slots are per batch, so two instances of different batches share slot 0; a write
        // names both. That is why promotion resolves the batch through the blocks.
        let mut writes = plan.writes.clone();
        writes.sort_unstable_by_key(|w| (w.batch, w.slot));
        assert_eq!(writes[0], SlotWrite { batch: 0, slot: 0, index: 0, active: true });
        assert_eq!(writes[1], SlotWrite { batch: 0, slot: 1, index: 2, active: true });
        assert_eq!(writes[2], SlotWrite { batch: 1, slot: 0, index: 1, active: true });
        assert_eq!(writes[3], SlotWrite { batch: 1, slot: 1, index: 3, active: true });
    }

    #[test]
    fn promote_deactivates_every_part_of_the_placement() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerClass, 64);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        // Placement 1 has both parts: one inactive write each, in table order.
        let writes = s.promote(IVec3::ZERO, 1, &view).unwrap();
        assert_eq!(writes.len(), 2);
        assert!(writes.iter().all(|w| !w.active));
        assert_eq!(writes.iter().map(|w| w.index).collect::<Vec<_>>(), [0, 1]);
        for write in &writes {
            let slot = s.resident(IVec3::ZERO).unwrap().batch_slots[write.index as usize];
            assert_eq!(write.slot, slot);
            assert_eq!(s.batch_key(write.batch).map(|k| k.chain.is_some()), Some(true));
        }
        // One way and idempotent: the second call writes nothing.
        assert!(s.promote(IVec3::ZERO, 1, &view).unwrap().is_empty());
        assert_eq!(s.stats().promoted, 1);
        // The slots stay allocated: a promoted placement costs its slots, not its draws.
        assert_eq!(s.stats().batch_live_slots, 4);
        // A placement of another cell, and a cell that is not resident.
        assert!(s.promote(IVec3::ZERO, 99, &view).unwrap().is_empty());
        assert_eq!(s.promote(IVec3::X, 1, &view), Err(SceneError::NotResident(IVec3::X)));
    }

    #[test]
    fn plan_out_releases_ranges_and_chains() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 64);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        let release = s.plan_out(IVec3::ZERO).unwrap();
        assert_eq!(release.scene, SlotRange { start: 0, len: 4 });
        assert_eq!(release.freed_chains, [0, 1]);
        assert_eq!(release.ranges, [0, 1]);
        assert_eq!(s.resident(IVec3::ZERO), None);
        // Everything released waits the frames in flight, and the batches now hold nothing.
        let stats = s.stats();
        assert_eq!((stats.cells, stats.batch_live_slots), (0, 0));
        assert_eq!(stats.scene.pending_slots, 4);
        assert_eq!(stats.chains, ChainStats { live: 0, pending: 2, high_water: 2 });
        assert!(s.batch_ranges(0).is_empty(), "a batch with no block has no range");
        // The same cell streams in again only after the window, and then into the same slots.
        s.advance(3);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        assert_eq!((plan.scene, plan.chain_slots.as_slice()), (SlotRange { start: 0, len: 4 }, [0, 1].as_slice()));
        assert_eq!(s.plan_out(IVec3::X), Err(SceneError::NotResident(IVec3::X)));
    }

    #[test]
    fn two_cells_sharing_a_chain_keep_it_through_one_unload() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerClass, 64);
        for cell in [IVec3::ZERO, IVec3::X] {
            let plan = s.plan_in(cell, &view, &()).unwrap();
            // The second cell shares both chains: nothing new to load, and no new slot.
            if cell == IVec3::X {
                assert!(plan.load.is_empty());
                assert_eq!(plan.chain_slots, [0, 1]);
                // Its blocks are its own, in the same batches as the first cell's.
                assert_eq!(plan.blocks.iter().map(|&(id, ..)| id).collect::<Vec<_>>(), [0, 1]);
            }
            s.commit_in(cell, &plan);
        }
        assert_eq!(s.chains().refs(0x10), Some(2));
        assert_eq!(s.stats().chains, ChainStats { live: 2, pending: 0, high_water: 2 });
        // One unload keeps the chains alive for the other cell.
        let release = s.plan_out(IVec3::ZERO).unwrap();
        assert!(release.freed_chains.is_empty());
        assert_eq!(s.chains().refs(0x10), Some(1));
        assert_eq!(s.stats().batch_live_slots, 4);
        // The last one frees them.
        let release = s.plan_out(IVec3::X).unwrap();
        assert_eq!(release.freed_chains, [0, 1]);
        assert_eq!(s.stats().chains, ChainStats { live: 0, pending: 2, high_water: 2 });
    }

    #[test]
    fn overflow_leaves_the_rest_of_the_cell() {
        // Three chains of one batch (one slab pair, one bind group) over three slots: the first
        // bucket takes two, the second does not fit, the third takes the last one.
        let bytes = table_bytes(
            &["meshes/a.mdl"],
            &[
                (0x10, 0, 0, MaterialClass::Opaque),
                (0x20, 0, 1, MaterialClass::Opaque),
                (0x30, 0, 2, MaterialClass::Opaque),
            ],
            &[(1, 0), (1, 1), (2, 0), (3, 1), (4, 2)],
        );
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let slabs = Slabs(0, 0, 0);
        let mut s = space(BatchGranularity::PerClass, 3);
        let plan = s.plan_in(IVec3::ZERO, &view, &slabs).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        assert_eq!(s.stats().batches, 1);
        assert_eq!(plan.overflowed.len(), 1, "chain 1's bucket is the one that did not fit");
        assert_eq!(plan.batch_slots, [0, NO_SLOT, 1, NO_SLOT, 2]);
        // Placement 1 has a part in the scene and a part that overflowed, so it spawns whole and
        // its resident part is written inactive. The rest of the cell is unaffected.
        assert_eq!(plan.covered, [2, 4]);
        let inactive: Vec<u32> = plan.writes.iter().filter(|w| !w.active).map(|w| w.index).collect();
        assert_eq!(inactive, [0]);
        assert_eq!(plan.writes.len(), 3);
        // Counted, and named once.
        let stats = s.stats();
        assert_eq!((stats.overflow_cells, stats.overflow_instances), (1, 2));
        assert_eq!(s.take_overflow_warning(), Some(plan.overflowed[0]));
        assert_eq!(s.take_overflow_warning(), None, "warned once");
        // Every chain still got a slot, overflowed or not: `write_remapped` demands one per chain.
        assert_eq!(plan.chain_slots, [0, 1, 2]);
        assert_eq!(remapped_slots(&plan.scene_bytes), [0, 1, 0, 1, 2]);
        // And the batch's range list still partitions its capacity, with the hole left by the
        // overflowed bucket merged into the range before it.
        assert_eq!(
            s.batch_ranges(0),
            [
                RangeSpec { first_slot: 0, slot_count: 2, chain: 0, level: 0 },
                RangeSpec { first_slot: 2, slot_count: 1, chain: 2, level: 0 },
            ]
        );
        assert_partitions(&s.batch_ranges(0), 3);
    }

    #[test]
    fn batch_ranges_partition_the_capacity_with_no_gaps() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        // One batch for both chains: the caller resolves every chain level to the same slabs.
        let slabs = Slabs(1, 2, 3);
        let mut s = space(BatchGranularity::PerClass, 16);
        let plan = s.plan_in(IVec3::ZERO, &view, &slabs).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        // Two material classes, so still two batches — but keyed on the slabs, not the chain.
        assert_eq!(s.stats().batches, 2);
        for (batch, key) in s.batch_keys() {
            assert_eq!(key.chain, None);
            assert_eq!((key.vertex_slab, key.index_slab, key.bind_group), (1, 2, 3));
            let ranges = s.batch_ranges(batch);
            assert_partitions(&ranges, 16);
            // One block, so one range: the two instances plus the whole tail of the capacity.
            assert_eq!(ranges, [RangeSpec { first_slot: 0, slot_count: 16, chain: batch, level: 0 }]);
        }
    }

    #[test]
    fn batch_ranges_merge_free_blocks_into_a_neighbour() {
        // Three cells of one chain each, in one batch: three blocks of two slots.
        let cells: Vec<Vec<u8>> = (0..3)
            .map(|k| {
                table_bytes(
                    &["meshes/a.mdl"],
                    &[(0x10 + u64::from(k), 0, k, MaterialClass::Opaque)],
                    &[(1, 0), (2, 0)],
                )
            })
            .collect();
        let slabs = Slabs(0, 0, 0);
        let mut s = space(BatchGranularity::PerClass, 16);
        for (k, bytes) in cells.iter().enumerate() {
            let view = StaticCellTableView::parse(bytes).unwrap();
            let cell = IVec3::new(i32::try_from(k).unwrap(), 0, 0);
            let plan = s.plan_in(cell, &view, &slabs).unwrap();
            s.commit_in(cell, &plan);
        }
        assert_eq!(s.stats().batches, 1);
        assert_eq!(
            s.batch_ranges(0),
            [
                RangeSpec { first_slot: 0, slot_count: 2, chain: 0, level: 0 },
                RangeSpec { first_slot: 2, slot_count: 2, chain: 1, level: 0 },
                RangeSpec { first_slot: 4, slot_count: 12, chain: 2, level: 0 },
            ]
        );
        // Unload the middle cell: its two slots are a hole, merged into the preceding range
        // rather than given a filler range of their own. They are never written active.
        let view = StaticCellTableView::parse(&cells[1]).unwrap();
        s.plan_out(IVec3::new(1, 0, 0)).unwrap();
        let ranges = s.batch_ranges(0);
        assert_partitions(&ranges, 16);
        assert_eq!(
            ranges,
            [
                RangeSpec { first_slot: 0, slot_count: 4, chain: 0, level: 0 },
                RangeSpec { first_slot: 4, slot_count: 12, chain: 2, level: 0 },
            ]
        );
        // And a hole at the start is absorbed by the range that follows it.
        s.plan_out(IVec3::ZERO).unwrap();
        let ranges = s.batch_ranges(0);
        assert_partitions(&ranges, 16);
        assert_eq!(ranges, [RangeSpec { first_slot: 0, slot_count: 16, chain: 2, level: 0 }]);
        // The hole comes back as slots once the frames in flight are done, and first fit takes
        // the lowest: the ranges partition the capacity at every step.
        s.advance(3);
        let plan = s.plan_in(IVec3::new(1, 0, 0), &view, &slabs).unwrap();
        s.commit_in(IVec3::new(1, 0, 0), &plan);
        assert_eq!(plan.blocks[0].1, SlotRange { start: 0, len: 2 });
        assert_partitions(&s.batch_ranges(0), 16);
    }

    #[test]
    fn per_chain_and_per_class_granularity_agree_on_slot_counts() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let slabs = Slabs(4, 5, 6);
        let mut per_chain = space(BatchGranularity::PerChain, 32);
        let mut per_class = space(BatchGranularity::PerClass, 32);
        let a = per_chain.plan_in(IVec3::ZERO, &view, &slabs).unwrap();
        let b = per_class.plan_in(IVec3::ZERO, &view, &slabs).unwrap();
        per_chain.commit_in(IVec3::ZERO, &a);
        per_class.commit_in(IVec3::ZERO, &b);
        // The same instances, the same scene buffer, the same chains, the same coverage: only the
        // batching differs, and `PerChain` ignores the resolver (a batch per chain level).
        assert_eq!(a.scene_bytes, b.scene_bytes);
        assert_eq!((a.chain_slots, a.covered, a.batch_slots), (b.chain_slots, b.covered, b.batch_slots));
        assert_eq!(a.writes.len(), b.writes.len());
        assert_eq!(per_chain.stats().batch_live_slots, per_class.stats().batch_live_slots);
        assert!(per_chain.batch_keys().all(|(_, key)| key.chain.is_some()));
        assert!(per_class.batch_keys().all(|(_, key)| key.chain.is_none()));
        // Two classes here, so both granularities give two batches; the classes are what
        // `PerClass` cannot merge.
        assert_eq!(per_chain.stats().batches, per_class.stats().batches);
        for (batch, _) in per_class.batch_keys() {
            assert_partitions(&per_class.batch_ranges(batch), 32);
        }
    }

    #[test]
    fn a_cancelled_load_gives_everything_back() {
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 64);
        let before = s.stats();
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.cancel_in(&plan);
        s.advance(u64::from(s.config().frames_in_flight));
        let after = s.stats();
        // Back to an empty space but for the high-water marks, which never shrink.
        assert_eq!(after.cells, before.cells);
        assert_eq!((after.batch_live_slots, after.scene.live_slots), (0, 0));
        assert_eq!(after.chains, ChainStats { live: 0, pending: 0, high_water: 2 });
        assert_eq!(s.resident(IVec3::ZERO), None);
        // And the cell can stream in again, into the same slots.
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        assert_eq!((plan.scene.start, plan.chain_slots.as_slice()), (0, [0, 1].as_slice()));
    }

    /// The walk `first_fit_high_water_on_the_same_walk` (`alloc.rs`) measured on the allocator
    /// alone, now through the whole space: a ring of cells streaming in and out, with the batch
    /// slots allocated by first fit as the coordinator decided on 2026-11-10.
    ///
    /// Asserts the property the decision rests on — the high-water mark stays near the peak live
    /// set — and that nothing overflows a 4096-slot batch at this footprint.
    fn ring_walk(cycles: u32, capacity: u32) -> (SpaceStats, u32) {
        // 40 cell footprints of 7..=638 instances over four chains, as the A4 walk used.
        let tables: Vec<Vec<u8>> = (0..40u32)
            .map(|k| {
                let instances: Vec<(PlacementId, u32)> = (0..7 + k * 16).map(|i| (i, i % 4)).collect();
                table_bytes(
                    &["meshes/a.mdl"],
                    &[
                        (0x10, 0, 0, MaterialClass::Opaque),
                        (0x20, 0, 1, MaterialClass::Opaque),
                        (0x30, 0, 2, MaterialClass::Opaque),
                        (0x40, 0, 3, MaterialClass::Opaque),
                    ],
                    &instances,
                )
            })
            .collect();
        let mut s = space(BatchGranularity::PerChain, capacity);
        let mut peak_live = 0u32;
        let mut resident: Vec<IVec3> = Vec::new();
        for cycle in 0..cycles {
            s.advance(u64::from(cycle));
            // 25 resident cells: one in, one out, as a 5x5 ring walks.
            if resident.len() >= 25 {
                let cell = resident.remove(0);
                s.plan_out(cell).expect("a resident cell");
            }
            let cell = IVec3::new(i32::try_from(cycle % 1000).unwrap(), 0, 0);
            if s.resident(cell).is_some() {
                continue;
            }
            let bytes = &tables[(cycle % 40) as usize];
            let view = StaticCellTableView::parse(bytes).expect("a valid table");
            let plan = s.plan_in(cell, &view, &()).expect("the walk never fails");
            s.commit_in(cell, &plan);
            resident.push(cell);
            peak_live = peak_live.max(s.stats().batch_live_slots);
        }
        (s.stats(), peak_live)
    }

    #[test]
    fn first_fit_batch_slots_settle_near_the_peak_live_set() {
        let (stats, peak_live) = ring_walk(4_000, 4096);
        assert_eq!(stats.overflow_cells, 0, "a 4096-slot batch holds this footprint");
        assert!(peak_live > 0);
        // First fit's price here, against the 3.33x the size-class allocator measured on the
        // same shape of walk (`ClassAllocator`'s docs): a small constant, not a creep.
        let ratio = f64::from(stats.batch_high_water) / f64::from(peak_live);
        assert!(ratio < 1.5, "high water {} is {ratio:.2}x the peak live {peak_live}", stats.batch_high_water);
    }

    #[test]
    #[ignore = "measurement: first-fit batch slots over a long walk (prints the curve)"]
    fn batch_slot_high_water_curve() {
        for cycles in [1_000u32, 10_000, 50_000, 200_000] {
            let (stats, peak_live) = ring_walk(cycles, 4096);
            let ratio = f64::from(stats.batch_high_water) / f64::from(peak_live);
            println!(
                "{cycles:>7} cycles: high_water {:>6}  peak_live {peak_live:>6}  ratio {ratio:.2}  \
                 free ranges {:>3}  overflow cells {}",
                stats.batch_high_water, stats.scene.free_ranges, stats.overflow_cells
            );
        }
    }

    #[test]
    fn an_empty_table_is_resident_without_a_slot() {
        // A cell with no instance and no chain: valid (a cell of nothing but terrain).
        let bytes = table_bytes(&[], &[], &[]);
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerClass, 64);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        assert_eq!(plan.scene, SlotRange { start: 0, len: 0 });
        assert!(plan.scene_bytes.is_empty() && plan.writes.is_empty() && plan.batches.is_empty());
        s.commit_in(IVec3::ZERO, &plan);
        assert_eq!(s.stats().scene.live_slots, 0);
        let release = s.plan_out(IVec3::ZERO).unwrap();
        assert_eq!(release, CellRelease { scene: SlotRange { start: 0, len: 0 }, freed_chains: vec![], ranges: vec![] });
    }

    #[test]
    fn a_batch_block_is_never_wider_than_its_bucket() {
        // First fit hands out exactly what was asked for: no block padding, so a batch's live
        // slots are its instances, and the only free space is between blocks.
        let bytes = two_chain_table();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 4096);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        let instances: u32 = plan.blocks.iter().map(|&(_, range, ..)| range.len).sum();
        assert_eq!(instances, 4);
        assert_eq!(s.stats().batch_live_slots, 4);
        // Which is what makes the capacity the only ceiling: `batch_capacity` slots per batch and
        // per view, no class rounding on top.
        assert_eq!(s.stats().batch_capacity, 2 * 4096);
    }

    #[test]
    fn exhaustion_is_an_error_not_a_panic() {
        // A batch of one slot against a chain with two instances: the whole bucket overflows.
        let bytes = table_bytes(&["m"], &[(0x10, 0, 0, MaterialClass::Opaque)], &[(1, 0), (2, 0)]);
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let mut s = space(BatchGranularity::PerChain, 1);
        let plan = s.plan_in(IVec3::ZERO, &view, &()).unwrap();
        s.commit_in(IVec3::ZERO, &plan);
        assert_eq!(plan.overflowed.len(), 1);
        assert!(plan.covered.is_empty(), "both placements spawn as entities");
        assert!(plan.writes.is_empty() && plan.blocks.is_empty() && plan.ranges.is_empty());
        // The chain was still acquired and the scene range still written: the cell is resident,
        // it just draws nothing. Stream-out gives it all back.
        assert_eq!(plan.chain_slots, [0]);
        assert_eq!(s.stats().overflow_instances, 2);
        let release = s.plan_out(IVec3::ZERO).unwrap();
        assert_eq!(release.freed_chains, [0]);
    }
}
