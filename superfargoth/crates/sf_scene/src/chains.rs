//! The global chain table: mesh key → one dense chain slot, refcounted by resident cell
//! (static-scene.md §2 "Chains are cell-local", §3; `scene-runtime-plan.md` §4.2).
//!
//! A baked table names its chains **cell-locally** (a dense index into its own chain section), so
//! a cell loads on its own and no two cells can disagree on an id. The GPU chain table, though, is
//! one array per space, so stream-in maps each cell-local index to a **global** slot keyed by the
//! chain's mesh key ([`crate::table::StaticCellTableView::write_remapped`] copies the instance
//! section with the slots in place). [`ChainRegistry`] owns that mapping and the refcount that
//! says when the last resident cell has let a chain go.
//!
//! Three rules, each a test:
//!
//! - **Slots are dense and deterministic.** They come from a [`SlotAllocator`] with `alloc(1)`,
//!   whose first fit over a free list sorted by start is always the *lowest* free slot, so the
//!   same sequence of cells gives the same slots on every machine and in every session. The
//!   high-water mark is the length the Bevy side's per-slot side table must have.
//! - **Reuse is deferred by the frames in flight.** [`ChainRegistry::release`] hands the slot to
//!   the allocator's pending list and [`ChainRegistry::advance`] frees it `frames_in_flight`
//!   frames later, so a slot cannot be handed to new geometry while a frame in flight still reads
//!   the old. A chain re-acquired inside that window takes a *new* slot: the Bevy half drops the
//!   mesh handles when the last cell releases, so resurrecting the old slot would point at
//!   geometry that is already on its way out.
//! - **A disagreement is an error, not an assert.** Equal mesh key must mean equal geometry
//!   subject, material class, material key and level list, bit for bit. A mismatch means one of
//!   the two cells was baked against a different mod set and is stale in the product store: it
//!   fails *that cell* (`scene-runtime-plan.md` §5.3, which leaves its placements to the entity
//!   path) and the registry is left exactly as it was.
//!
//! No Bevy types live here. The Bevy half keeps its own `Vec<Option<ChainAssets>>` indexed by
//! slot, and drops the entry when [`ChainRegistry::release`] reports the last reference.

use std::collections::BTreeMap;

use crate::alloc::{AllocError, SlotAllocator, SlotRange};
use crate::material::MaterialClass;
use crate::product::chain_subject;
use crate::table::{LodLevel, StaticCellTableView};

/// A chain's identity across cells: the 64-bit mesh key of a baked chain record
/// ([`crate::LodChain::mesh`]), the cooked geometry of one (model, material part).
pub type ChainKey = u64;

/// A chain's global slot: what [`crate::table::StaticCellTableView::write_remapped`] writes into
/// the instances, and the index of the chain's row in the GPU chain table.
pub type ChainSlot = u32;

/// Where a chain's geometry comes from, read out of a v3 table
/// ([`crate::table::StaticCellTableView::chain_source`]): the `mesh.cook` subject of its model
/// and its part, the material index inside that model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainSource {
    pub model: String,
    pub part: u32,
}

impl ChainSource {
    /// The chain's geometry subject, `<model>/<part>` ([`chain_subject`]): the name of its
    /// `scene.chains` product.
    #[must_use]
    pub fn subject(&self) -> String {
        chain_subject(&self.model, self.part)
    }
}

/// One chain as the registry holds it: everything a cell's table says about it, which every other
/// cell naming the same mesh key must say too.
#[derive(Clone, Debug, PartialEq)]
pub struct ChainEntry {
    pub key: ChainKey,
    pub source: ChainSource,
    pub class: MaterialClass,
    /// Content hash of the chain's cooked material ([`crate::LodChain::material`]): equal keys,
    /// one bindless material slot.
    pub material_key: u64,
    /// The chain's LOD levels, finest first.
    pub levels: Vec<LodLevel>,
}

impl ChainEntry {
    /// Chain `index` of a validated v3 table; `None` past its chain section.
    #[must_use]
    pub fn from_table(view: &StaticCellTableView<'_>, index: u32) -> Option<Self> {
        let chain = view.chain(index)?;
        let (model, part) = view.chain_source(index)?;
        Some(Self {
            key: chain.mesh,
            source: ChainSource { model: model.to_owned(), part },
            class: chain.material_class,
            material_key: chain.material,
            levels: chain.levels,
        })
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ChainError {
    /// Two cells disagree about one mesh key: a stale `scene.cells` product in the store. Fails
    /// the cell that brought the second one; the registry is unchanged.
    #[error("two cells disagree about chain {key:#018x}: {field}")]
    Disagreement { key: ChainKey, field: &'static str },
    /// [`ChainRegistry::release`] was given a key no cell holds — a double release, or a cell
    /// releasing a chain it never acquired.
    #[error("chain {key:#018x} is not resident")]
    NotResident { key: ChainKey },
    /// The slot allocator refused (only [`AllocError::Overflow`] is reachable: the chain table
    /// would pass `u32::MAX` slots).
    #[error("no slot for chain: {0}")]
    Slot(#[from] AllocError),
}

/// What [`ChainRegistry::acquire`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acquired {
    /// First reference: the slot is new and the Bevy half must load the chain's geometry into it.
    Added(ChainSlot),
    /// Another cell already had it; only the refcount moved.
    Shared(ChainSlot),
}

impl Acquired {
    #[must_use]
    pub fn slot(self) -> ChainSlot {
        match self {
            Self::Added(slot) | Self::Shared(slot) => slot,
        }
    }

    #[must_use]
    pub fn is_new(self) -> bool {
        matches!(self, Self::Added(_))
    }
}

/// Occupancy of the chain table (`scene/chains/*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChainStats {
    /// Chains at least one resident cell holds.
    pub live: u32,
    /// Slots released, waiting for the frames in flight.
    pub pending: u32,
    /// Slots the GPU chain table (and the Bevy half's per-slot side table) must hold.
    pub high_water: u32,
}

#[derive(Clone, Debug)]
struct Resident {
    entry: ChainEntry,
    /// Resident cells holding this chain.
    refs: u32,
}

/// Mesh key → global chain slot, refcounted by resident cell.
///
/// See the module docs for the three rules. Clone is a deep copy of the whole table (for a debug
/// snapshot); nothing in the engine needs two live registries for one space.
#[derive(Clone, Debug)]
pub struct ChainRegistry {
    /// Indexed by slot, `None` where the slot is free or waiting. Grows to the allocator's
    /// high-water mark and never shrinks.
    slots: Vec<Option<Resident>>,
    by_key: BTreeMap<ChainKey, ChainSlot>,
    /// One slot per chain, so `alloc(1)` over a free list sorted by start is a lowest-free-first
    /// segregated list, and release is deferred for us.
    alloc: SlotAllocator,
}

impl ChainRegistry {
    /// An empty table whose released slots wait `frames_in_flight` frames before reuse.
    #[must_use]
    pub fn new(frames_in_flight: u32) -> Self {
        Self { slots: Vec::new(), by_key: BTreeMap::new(), alloc: SlotAllocator::new(frames_in_flight) }
    }

    /// One more reference to `entry`'s chain: a new slot ([`Acquired::Added`]) or the slot another
    /// cell already holds ([`Acquired::Shared`]).
    ///
    /// Fails with [`ChainError::Disagreement`] when a resident chain of the same key says anything
    /// else about the geometry, and changes nothing in that case.
    pub fn acquire(&mut self, entry: ChainEntry) -> Result<Acquired, ChainError> {
        let key = entry.key;
        if let Some(&slot) = self.by_key.get(&key) {
            let resident = self.resident_mut(slot);
            agree(&resident.entry, &entry)?;
            resident.refs = resident.refs.checked_add(1).expect("a chain in 2^32 cells at once");
            return Ok(Acquired::Shared(slot));
        }
        let slot = self.alloc.alloc(1)?.start;
        let k = slot as usize;
        if self.slots.len() <= k {
            self.slots.resize(k + 1, None);
        }
        self.slots[k] = Some(Resident { entry, refs: 1 });
        self.by_key.insert(key, slot);
        Ok(Acquired::Added(slot))
    }

    /// Drops one reference to `key`'s chain at frame `frame`; `Some(slot)` when it was the last,
    /// which is when the caller drops its handles for that slot. The slot itself returns to the
    /// free list only at [`Self::advance`]`(frame + frames_in_flight)`.
    pub fn release(&mut self, key: ChainKey, frame: u64) -> Result<Option<ChainSlot>, ChainError> {
        let slot = *self.by_key.get(&key).ok_or(ChainError::NotResident { key })?;
        let resident = self.resident_mut(slot);
        resident.refs -= 1;
        if resident.refs > 0 {
            return Ok(None);
        }
        self.slots[slot as usize] = None;
        self.by_key.remove(&key);
        self.alloc.release(SlotRange { start: slot, len: 1 }, frame)?;
        Ok(Some(slot))
    }

    /// Returns every slot whose frames in flight are done by `frame` to the free list.
    pub fn advance(&mut self, frame: u64) {
        self.alloc.advance(frame);
    }

    /// The slot of a resident chain.
    #[must_use]
    pub fn slot(&self, key: ChainKey) -> Option<ChainSlot> {
        self.by_key.get(&key).copied()
    }

    /// The chain in `slot`, if one is resident there.
    #[must_use]
    pub fn entry(&self, slot: ChainSlot) -> Option<&ChainEntry> {
        self.slots.get(slot as usize)?.as_ref().map(|r| &r.entry)
    }

    /// Resident cells holding `key`'s chain (for the debug view and the tests).
    #[must_use]
    pub fn refs(&self, key: ChainKey) -> Option<u32> {
        let slot = self.slot(key)?;
        self.slots[slot as usize].as_ref().map(|r| r.refs)
    }

    /// Every resident chain in slot order.
    pub fn resident(&self) -> impl Iterator<Item = (ChainSlot, &ChainEntry)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(k, r)| r.as_ref().map(|r| (u32::try_from(k).unwrap_or(u32::MAX), &r.entry)))
    }

    #[must_use]
    pub fn stats(&self) -> ChainStats {
        let slots = self.alloc.stats();
        ChainStats {
            live: u32::try_from(self.by_key.len()).unwrap_or(u32::MAX),
            pending: slots.pending_slots,
            high_water: slots.high_water,
        }
    }

    /// `by_key` and `slots` are written together, so a key's slot is always resident.
    fn resident_mut(&mut self, slot: ChainSlot) -> &mut Resident {
        self.slots[slot as usize].as_mut().expect("by_key names a resident slot")
    }
}

/// The first field `new` disagrees with `old` about, if any.
fn agree(old: &ChainEntry, new: &ChainEntry) -> Result<(), ChainError> {
    let field = if old.source.model != new.source.model {
        "model"
    } else if old.source.part != new.source.part {
        "part"
    } else if old.class != new.class {
        "material class"
    } else if old.material_key != new.material_key {
        "material key"
    } else if old.levels.len() != new.levels.len() {
        "level count"
    } else if !levels_agree(&old.levels, &new.levels) {
        "levels"
    } else {
        return Ok(());
    };
    Err(ChainError::Disagreement { key: old.key, field })
}

/// Level lists compared **bit for bit**, errors included: the same cooked geometry gives the same
/// bits, and comparing the errors as floats would call two NaN level errors equal (and a stale
/// product is exactly where a NaN would come from).
fn levels_agree(old: &[LodLevel], new: &[LodLevel]) -> bool {
    old.iter().zip(new).all(|(a, b)| {
        (a.first_index, a.index_count, a.error.to_bits()) == (b.first_index, b.index_count, b.error.to_bits())
    })
}

#[cfg(test)]
mod tests {
    use glam::{IVec3, Vec3};

    use super::*;
    use crate::table::{Aabb, InstanceFlags, LodChain, Sphere, StaticCellTable, StaticInstance};

    fn levels(errors: &[f32]) -> Vec<LodLevel> {
        errors
            .iter()
            .zip(0u32..)
            .map(|(&error, k)| LodLevel { first_index: k * 300, index_count: 300 >> k, error })
            .collect()
    }

    fn entry(key: ChainKey, model: &str, part: u32) -> ChainEntry {
        ChainEntry {
            key,
            source: ChainSource { model: model.to_owned(), part },
            class: MaterialClass::Opaque,
            material_key: key ^ 0xA5A5,
            levels: levels(&[0.0, 0.05]),
        }
    }

    #[test]
    fn a_second_cell_shares_the_slot_and_bumps_the_refcount() {
        let mut r = ChainRegistry::new(2);
        let first = r.acquire(entry(0x10, "meshes/a.mdl", 0)).unwrap();
        assert_eq!(first, Acquired::Added(0));
        assert!(first.is_new());
        // A second cell with the same chain: same slot, no new geometry to load.
        let second = r.acquire(entry(0x10, "meshes/a.mdl", 0)).unwrap();
        assert_eq!(second, Acquired::Shared(0));
        assert!(!second.is_new());
        assert_eq!((r.refs(0x10), r.slot(0x10)), (Some(2), Some(0)));
        assert_eq!(r.stats(), ChainStats { live: 1, pending: 0, high_water: 1 });
        // One unload keeps it: the handles stay alive for the other cell.
        assert_eq!(r.release(0x10, 5).unwrap(), None);
        assert_eq!(r.refs(0x10), Some(1));
        assert_eq!(r.entry(0).map(|e| e.source.subject()), Some("meshes/a.mdl/0".to_owned()));
        // The second unload is the last.
        assert_eq!(r.release(0x10, 6).unwrap(), Some(0));
        assert_eq!((r.refs(0x10), r.slot(0x10), r.entry(0)), (None, None, None));
        assert_eq!(r.stats(), ChainStats { live: 0, pending: 1, high_water: 1 });
        // A key no cell holds, and a double release.
        assert_eq!(r.release(0x10, 7), Err(ChainError::NotResident { key: 0x10 }));
        assert_eq!(r.release(0x99, 7), Err(ChainError::NotResident { key: 0x99 }));
    }

    #[test]
    fn slots_are_dense_and_lowest_first() {
        let mut r = ChainRegistry::new(0);
        for (k, key) in [0x30, 0x10, 0x20, 0x40].into_iter().enumerate() {
            // Slots are handed out in acquisition order, not key order: dense from 0.
            assert_eq!(r.acquire(entry(key, "m", 0)).unwrap(), Acquired::Added(u32::try_from(k).unwrap()));
        }
        assert_eq!(r.stats(), ChainStats { live: 4, pending: 0, high_water: 4 });
        // Free slots 1 and 2, then take two: the lowest free comes first, every time.
        assert_eq!(r.release(0x10, 0).unwrap(), Some(1));
        assert_eq!(r.release(0x20, 0).unwrap(), Some(2));
        r.advance(0);
        assert_eq!(r.acquire(entry(0x50, "m", 0)).unwrap(), Acquired::Added(1));
        assert_eq!(r.acquire(entry(0x60, "m", 0)).unwrap(), Acquired::Added(2));
        // And the table never grew: four chains, four slots.
        assert_eq!(r.stats(), ChainStats { live: 4, pending: 0, high_water: 4 });
        assert_eq!(
            r.resident().map(|(slot, e)| (slot, e.key)).collect::<Vec<_>>(),
            [(0, 0x30), (1, 0x50), (2, 0x60), (3, 0x40)]
        );
        // Released in the other order, the slots still come back lowest first.
        assert_eq!(r.release(0x60, 1).unwrap(), Some(2));
        assert_eq!(r.release(0x50, 1).unwrap(), Some(1));
        r.advance(1);
        assert_eq!(r.acquire(entry(0x70, "m", 0)).unwrap(), Acquired::Added(1));
    }

    #[test]
    fn the_last_release_frees_after_the_frames_in_flight() {
        let mut r = ChainRegistry::new(3);
        assert_eq!(r.acquire(entry(0x10, "m", 0)).unwrap().slot(), 0);
        assert_eq!(r.release(0x10, 100).unwrap(), Some(0));
        assert_eq!(r.stats().pending, 1);
        // The slot is free only once the frames that could still read it are done.
        for frame in 100..103 {
            r.advance(frame);
            assert_eq!(r.stats().pending, 1, "frame {frame}");
        }
        r.advance(103);
        assert_eq!(r.stats(), ChainStats { live: 0, pending: 0, high_water: 1 });
        assert_eq!(r.acquire(entry(0x20, "m", 0)).unwrap(), Acquired::Added(0));
    }

    #[test]
    fn a_released_slot_is_not_reused_inside_the_window() {
        let mut r = ChainRegistry::new(3);
        let old = r.acquire(entry(0x10, "meshes/a.mdl", 0)).unwrap().slot();
        assert_eq!(r.release(0x10, 10).unwrap(), Some(old));
        // Inside the window, every acquire must land somewhere else — the chain's own key
        // included: the Bevy half dropped the handles for `old` when the last cell let it go.
        for (frame, key) in (10..13).zip([0x20, 0x30, 0x10]) {
            r.advance(frame);
            let slot = r.acquire(entry(key, "meshes/a.mdl", 0)).unwrap().slot();
            assert_ne!(slot, old, "frame {frame}: slot {old} is still in flight");
            assert_eq!(r.release(key, frame).unwrap(), Some(slot));
        }
        // Past it, the slot comes back.
        r.advance(13);
        assert_eq!(r.acquire(entry(0x10, "meshes/a.mdl", 0)).unwrap(), Acquired::Added(old));
    }

    #[test]
    fn disagreeing_level_lists_are_an_error_and_mutate_nothing() {
        let mut r = ChainRegistry::new(2);
        let good = entry(0x10, "meshes/a.mdl", 0);
        r.acquire(good.clone()).unwrap();
        let before = r.stats();
        let cases: [(&str, ChainEntry); 6] = [
            ("model", ChainEntry { source: ChainSource { model: "meshes/b.mdl".into(), part: 0 }, ..good.clone() }),
            ("part", ChainEntry { source: ChainSource { model: "meshes/a.mdl".into(), part: 1 }, ..good.clone() }),
            ("material class", ChainEntry { class: MaterialClass::Masked, ..good.clone() }),
            ("material key", ChainEntry { material_key: 7, ..good.clone() }),
            ("level count", ChainEntry { levels: levels(&[0.0]), ..good.clone() }),
            ("levels", ChainEntry { levels: levels(&[0.0, 0.06]), ..good.clone() }),
        ];
        for (field, stale) in cases {
            assert_eq!(
                r.acquire(stale),
                Err(ChainError::Disagreement { key: 0x10, field }),
                "a stale product should fail on {field}"
            );
            // Nothing moved: not the refcount, not the slot, not the entry.
            assert_eq!((r.refs(0x10), r.stats()), (Some(1), before));
            assert_eq!(r.entry(0), Some(&good));
        }
        // A NaN level error is a disagreement, not an accidental equality.
        let mut nan = good.clone();
        nan.levels[1].error = f32::NAN;
        assert_eq!(r.acquire(nan.clone()), Err(ChainError::Disagreement { key: 0x10, field: "levels" }));
        // ...in both directions: NaN against NaN is the *same* bits, so it agrees with itself.
        let mut r2 = ChainRegistry::new(2);
        r2.acquire(nan.clone()).unwrap();
        assert_eq!(r2.acquire(nan).unwrap(), Acquired::Shared(0));
    }

    #[test]
    fn an_entry_comes_out_of_a_v3_table() {
        let table = StaticCellTable {
            models: vec!["meshes/a.mdl".into(), "meshes/b.mdl".into()],
            origin_cell: IVec3::new(1, 0, 2),
            aabb: Aabb { min: Vec3::splat(-10.0), max: Vec3::splat(10.0) },
            land_min: -1.0,
            layers: 1,
            instances: vec![
                StaticInstance {
                    placement: 0,
                    transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
                    sphere: Sphere { center: Vec3::ZERO, radius: 1.0 },
                    chain: 0,
                    material_override: 0,
                    flags: InstanceFlags::empty(),
                },
                StaticInstance {
                    placement: 1,
                    transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
                    sphere: Sphere { center: Vec3::ZERO, radius: 1.0 },
                    chain: 1,
                    material_override: 0,
                    flags: InstanceFlags::empty(),
                },
            ],
            chains: vec![
                LodChain {
                    mesh: 0x10,
                    material: 0x11,
                    material_class: MaterialClass::OpaqueWind,
                    model: 0,
                    part: 0,
                    levels: levels(&[0.0, 0.05]),
                },
                LodChain {
                    mesh: 0x20,
                    material: 0x21,
                    material_class: MaterialClass::MaskedTwoSided,
                    model: 1,
                    part: 3,
                    levels: levels(&[0.0]),
                },
            ],
        };
        let bytes = table.write().unwrap();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        let first = ChainEntry::from_table(&view, 0).unwrap();
        assert_eq!(first.key, 0x10);
        assert_eq!(first.source, ChainSource { model: "meshes/a.mdl".into(), part: 0 });
        assert_eq!((first.class, first.material_key), (MaterialClass::OpaqueWind, 0x11));
        assert_eq!(first.levels, levels(&[0.0, 0.05]));
        let second = ChainEntry::from_table(&view, 1).unwrap();
        assert_eq!(second.source.subject(), "meshes/b.mdl/3");
        assert_eq!(ChainEntry::from_table(&view, 2), None, "past the chain section");
        // And the registry takes them straight from the table, in table order.
        let mut r = ChainRegistry::new(2);
        assert_eq!(r.acquire(first).unwrap(), Acquired::Added(0));
        assert_eq!(r.acquire(second).unwrap(), Acquired::Added(1));
        assert_eq!(r.slot(0x20), Some(1));
    }
}
