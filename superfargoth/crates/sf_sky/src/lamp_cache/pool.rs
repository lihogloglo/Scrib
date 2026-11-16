//! The store pool: persistent copies of clean faces, pooled per face and given back LRU.
//! Spec: `docs/specs/lamp-shadow-cache.md` §1 "Memory", "Eviction".

use super::LampKey;

/// A layer of the store texture (S2 owns the texture; the permanently cleared layer is not
/// one of these, see [`super::CopyOp::Clear`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreLayer(pub u32);

#[derive(Clone, Copy, Debug, Default)]
struct Slot {
    /// The face whose static depth the layer holds.
    owner: Option<(LampKey, u8)>,
    /// Its face's live layer has movers drawn over it: the layer is the only static copy.
    pinned: bool,
    /// The frame the layer was last saved or restored from.
    last_used: u64,
}

/// Fixed-size pool of store layers. A layer is free, held (its face is clean: reclaimable,
/// least recently used first) or pinned (its face has movers drawn over its live layer).
#[derive(Clone, Debug, Default)]
pub struct StorePool {
    slots: Vec<Slot>,
}

impl StorePool {
    #[must_use]
    pub fn new(layers: u32) -> Self {
        Self {
            slots: vec![Slot::default(); layers as usize],
        }
    }

    /// Layers in the pool.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Layers holding a face's static depth (held or pinned).
    #[must_use]
    pub fn in_use(&self) -> usize {
        self.slots.iter().filter(|s| s.owner.is_some()).count()
    }

    /// Layers whose face has movers drawn over its live layer.
    #[must_use]
    pub fn pinned(&self) -> usize {
        self.slots.iter().filter(|s| s.pinned).count()
    }

    /// The face a layer holds.
    #[must_use]
    pub fn owner(&self, layer: StoreLayer) -> Option<(LampKey, u8)> {
        self.slot(layer).and_then(|s| s.owner)
    }

    fn slot(&self, layer: StoreLayer) -> Option<&Slot> {
        self.slots.get(layer.0 as usize)
    }

    fn slot_mut(&mut self, layer: StoreLayer) -> Option<&mut Slot> {
        self.slots.get_mut(layer.0 as usize)
    }

    /// Layers [`Self::take`] can hand out without touching `exclude`'s own layers.
    pub(super) fn available(&self, exclude: LampKey) -> usize {
        self.slots
            .iter()
            .filter(|s| !s.pinned && s.owner.is_none_or(|(lamp, _)| lamp != exclude))
            .count()
    }

    /// A layer for `owner`: a free one (lowest index), else the least recently used unpinned
    /// layer of another lamp, whose face is returned so the caller drops its reference.
    pub(super) fn take(
        &mut self,
        owner: (LampKey, u8),
        frame: u64,
    ) -> Option<(StoreLayer, Option<(LampKey, u8)>)> {
        let index = self
            .slots
            .iter()
            .position(|s| s.owner.is_none())
            .or_else(|| {
                self.slots
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| !s.pinned && s.owner.is_some_and(|(lamp, _)| lamp != owner.0))
                    .min_by_key(|(i, s)| (s.last_used, *i))
                    .map(|(i, _)| i)
            })?;
        let slot = &mut self.slots[index];
        let victim = slot.owner.replace(owner);
        slot.pinned = false;
        slot.last_used = frame;
        let layer = StoreLayer(u32::try_from(index).expect("a pool is sized by a u32"));
        Some((layer, victim))
    }

    /// The layer is free again.
    pub(super) fn release(&mut self, layer: StoreLayer) {
        if let Some(slot) = self.slot_mut(layer) {
            *slot = Slot::default();
        }
    }

    pub(super) fn set_pinned(&mut self, layer: StoreLayer, pinned: bool) {
        if let Some(slot) = self.slot_mut(layer) {
            slot.pinned = pinned;
        }
    }

    pub(super) fn touch(&mut self, layer: StoreLayer, frame: u64) {
        if let Some(slot) = self.slot_mut(layer) {
            slot.last_used = frame;
        }
    }
}
