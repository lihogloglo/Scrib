//! The slot pool of one GPU batch (Bevy-free): a fixed number of equal **tile blocks** of `block_slots` slots
//! (spec "Facts that shape the build": no fragmentation, one write run per tile, freeing = one zero-fill).
//!
//! A block holds one (tile, species) scatter: its instances first, then inactive (zeroed) slots. The block size is
//! chosen by the plan (`residency::Plan`: the expected count plus headroom, or the hard ceiling with
//! `reserve_worst_case`); a tile with more instances than its block writes only the block's worth. A pool grows by
//! appending blocks ([`SlotPool::grow`]); blocks never move. Allocation takes the lowest free block, so the layout
//! is a pure function of the allocate/free/grow sequence.

use std::ops::Range;

/// A batch's blocks.
#[derive(Clone, Debug)]
pub struct SlotPool {
    block_slots: u32,
    /// `live[b]`: `Some(instances)` while block `b` is allocated.
    live: Vec<Option<u32>>,
    allocated: u32,
}

impl SlotPool {
    /// `blocks` blocks of `block_slots` slots. Their product must fit a `u32` (the batch capacity).
    pub fn new(blocks: u32, block_slots: u32) -> Self {
        assert!(block_slots > 0, "a tile block holds at least one slot");
        assert!(blocks.checked_mul(block_slots).is_some(), "batch capacity {blocks} x {block_slots} overflows u32");
        Self { block_slots, live: vec![None; blocks as usize], allocated: 0 }
    }

    pub const fn block_slots(&self) -> u32 {
        self.block_slots
    }

    /// Number of blocks.
    pub fn blocks(&self) -> u32 {
        // Constructed from a u32.
        #[allow(clippy::cast_possible_truncation)]
        let n = self.live.len() as u32;
        n
    }

    /// Slots of the whole batch (`blocks × block_slots`).
    pub fn capacity(&self) -> u32 {
        self.blocks() * self.block_slots
    }

    pub const fn allocated(&self) -> u32 {
        self.allocated
    }

    /// Live (active) slots over every allocated block.
    pub fn live_slots(&self) -> u64 {
        self.live.iter().flatten().map(|&n| u64::from(n)).sum()
    }

    /// Grows the pool to `blocks` blocks (never shrinks): every existing block keeps its slots, the new ones are
    /// free and come after them (the batch's new tail). Returns the new tail's slot range (to zero-fill).
    pub fn grow(&mut self, blocks: u32) -> Range<u32> {
        let old = self.capacity();
        if blocks > self.blocks() {
            assert!(blocks.checked_mul(self.block_slots).is_some(), "batch capacity {blocks} x {} overflows u32", self.block_slots);
            self.live.resize(blocks as usize, None);
        }
        old..self.capacity()
    }

    /// The lowest free block, now allocated with no live slot; `None` when the pool is full.
    pub fn allocate(&mut self) -> Option<u32> {
        let b = self.live.iter().position(Option::is_none)?;
        self.live[b] = Some(0);
        self.allocated += 1;
        // Index of a Vec built from a u32 length.
        #[allow(clippy::cast_possible_truncation)]
        let b = b as u32;
        Some(b)
    }

    /// Frees `block`, returning the slot range that held live instances (to zero-fill).
    pub fn free(&mut self, block: u32) -> Range<u32> {
        let live = self.live.get_mut(block as usize).and_then(Option::take);
        let Some(live) = live else {
            debug_assert!(false, "block {block} freed twice");
            return 0..0;
        };
        self.allocated -= 1;
        let first = self.first_slot(block);
        first..first + live
    }

    /// Records `count` live instances in `block` (clamped to the block), returning the slot range the write must
    /// cover: the instances, then zeros up to the block's previous live count.
    pub fn set_live(&mut self, block: u32, count: u32) -> Range<u32> {
        let count = count.min(self.block_slots);
        let first = self.first_slot(block);
        let Some(Some(live)) = self.live.get_mut(block as usize) else {
            debug_assert!(false, "block {block} written while free");
            return first..first;
        };
        let end = (*live).max(count);
        *live = count;
        first..first + end
    }

    /// Live instances in `block` (0 when free).
    pub fn live(&self, block: u32) -> u32 {
        self.live.get(block as usize).copied().flatten().unwrap_or(0)
    }

    /// First slot of `block`.
    pub const fn first_slot(&self, block: u32) -> u32 {
        block * self.block_slots
    }
}
