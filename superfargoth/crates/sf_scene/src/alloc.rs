//! The per-space GPU instance table's slot allocator (static-scene.md §3, "The GPU scene").
//!
//! Stream-in takes one contiguous range per cell; stream-out releases it, but the slots return
//! to the free list only once the frames in flight that may still read them are done
//! ([`SlotAllocator::release`] then [`SlotAllocator::advance`]). First fit over a coalesced free
//! list; when nothing fits the table grows at its end (the high-water mark is the buffer size the
//! Bevy side must provide). No compaction: [`SlotStats`] reports fragmentation instead.
//!
//! [`SlotAllocator::with_capacity`] is the same allocator over a **fixed** ceiling, for a batch's
//! output slots, where there is nothing to grow into: a request that would pass the capacity is
//! [`AllocError::Exhausted`] and the caller leaves those instances to the entity path. First fit
//! is what the scene buffer, the batch output slots and the chain table all use, by the
//! coordinator's 2026-11-10 decision — [`ClassAllocator`]'s docs carry the measurement that
//! decided it.

use std::collections::BTreeMap;

/// A contiguous slot range `[start, start + len)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotRange {
    pub start: u32,
    pub len: u32,
}

impl SlotRange {
    pub fn end(self) -> u32 {
        self.start + self.len
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum AllocError {
    #[error("a slot range needs at least one slot")]
    ZeroLength,
    #[error("the slot table would exceed u32 slots")]
    Overflow,
    #[error("slot range {0:?} is not allocated")]
    NotAllocated(SlotRange),
    /// A fixed-capacity allocator ([`SlotAllocator::with_capacity`] or [`ClassAllocator`]) is
    /// full. Not an error of the caller's: the cell publishes without those instances and they
    /// spawn as entities instead.
    #[error("no room for {want} more slots in a batch of {capacity}")]
    Exhausted { want: u32, capacity: u32 },
}

/// Occupancy and fragmentation (for the perf report and the debug view).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlotStats {
    /// Slots the GPU buffer must hold.
    pub high_water: u32,
    pub live_slots: u32,
    pub live_ranges: u32,
    /// Free and reusable now.
    pub free_slots: u32,
    pub free_ranges: u32,
    pub largest_free: u32,
    /// Released, waiting for the frames in flight.
    pub pending_slots: u32,
    /// `1 − largest_free / free_slots` (0 with no free slot): 0 means all free space is one hole.
    pub fragmentation: f32,
}

#[derive(Clone, Debug)]
pub struct SlotAllocator {
    frames_in_flight: u64,
    /// The ceiling the table may not pass: `None` for a table that grows (the per-space scene
    /// buffer and the chain table), `Some` for a batch's fixed output slots.
    capacity: Option<u32>,
    high_water: u32,
    /// Sorted by start, coalesced, below `high_water`.
    free: Vec<SlotRange>,
    /// start → len.
    live: BTreeMap<u32, u32>,
    /// (release frame, range).
    pending: Vec<(u64, SlotRange)>,
}

impl SlotAllocator {
    /// An empty table whose released ranges wait `frames_in_flight` frames before reuse.
    pub fn new(frames_in_flight: u32) -> Self {
        Self {
            frames_in_flight: u64::from(frames_in_flight),
            capacity: None,
            high_water: 0,
            free: Vec::new(),
            live: BTreeMap::new(),
            pending: Vec::new(),
        }
    }

    /// The same allocator over a **fixed** `capacity`: a batch's output slots, whose buffer and
    /// materialize bind group are sized from it and are never rebuilt at stream-in (§4.4). An
    /// `alloc` that would pass `capacity` is [`AllocError::Exhausted`], not a growth.
    ///
    /// First fit rather than [`ClassAllocator`] by the coordinator's decision of 2026-11-10: A4
    /// measured 3.33× the peak live slots for segregated size classes against **1.17×** for first
    /// fit on the same walk ([`ClassAllocator`]'s docs). First fit has no a-priori ceiling in
    /// exchange, so `Exhausted` must stay a counted, tested path.
    #[must_use]
    pub fn with_capacity(capacity: u32, frames_in_flight: u32) -> Self {
        Self { capacity: Some(capacity), ..Self::new(frames_in_flight) }
    }

    /// The fixed ceiling, if this allocator has one.
    #[must_use]
    pub fn capacity(&self) -> Option<u32> {
        self.capacity
    }

    /// A contiguous range of `len` slots (first fit, else at the end of the table); with a fixed
    /// [`Self::capacity`], [`AllocError::Exhausted`] rather than a growth past it.
    pub fn alloc(&mut self, len: u32) -> Result<SlotRange, AllocError> {
        if len == 0 {
            return Err(AllocError::ZeroLength);
        }
        let range = if let Some(k) = self.free.iter().position(|f| f.len >= len) {
            let hole = &mut self.free[k];
            let range = SlotRange { start: hole.start, len };
            hole.start += len;
            hole.len -= len;
            if hole.len == 0 {
                self.free.remove(k);
            }
            range
        } else {
            // A hole touching the end is extended rather than skipped.
            let start = match self.free.last() {
                Some(last) if last.end() == self.high_water => last.start,
                _ => self.high_water,
            };
            let end = start.checked_add(len).ok_or(AllocError::Overflow)?;
            // A fixed-capacity table has nothing to grow into. Checked before anything is
            // mutated: an `Exhausted` allocator is exactly as it was.
            if let Some(capacity) = self.capacity.filter(|&capacity| end > capacity) {
                return Err(AllocError::Exhausted { want: len, capacity });
            }
            if start != self.high_water {
                self.free.pop();
            }
            self.high_water = end;
            SlotRange { start, len }
        };
        self.live.insert(range.start, range.len);
        Ok(range)
    }

    /// Releases `range` at frame `frame`; it becomes free at `frame + frames_in_flight`.
    pub fn release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError> {
        if self.live.get(&range.start) != Some(&range.len) {
            return Err(AllocError::NotAllocated(range));
        }
        self.live.remove(&range.start);
        self.pending.push((frame, range));
        Ok(())
    }

    /// Frees every pending range whose frames in flight are done by `frame`.
    pub fn advance(&mut self, frame: u64) {
        let window = self.frames_in_flight;
        let mut done = Vec::new();
        self.pending.retain(|&(at, range)| {
            let ready = at.saturating_add(window) <= frame;
            if ready {
                done.push(range);
            }
            !ready
        });
        for range in done {
            self.insert_free(range);
        }
    }

    fn insert_free(&mut self, range: SlotRange) {
        let k = self.free.partition_point(|f| f.start < range.start);
        let mut merged = range;
        if k < self.free.len() && self.free[k].start == merged.end() {
            merged.len += self.free.remove(k).len;
        }
        if k > 0 && self.free[k - 1].end() == merged.start {
            self.free[k - 1].len += merged.len;
        } else {
            self.free.insert(k, merged);
        }
    }

    /// The live range starting at `start`, if any.
    pub fn live_range(&self, start: u32) -> Option<SlotRange> {
        self.live.get(&start).map(|&len| SlotRange { start, len })
    }

    pub fn stats(&self) -> SlotStats {
        let free_slots: u32 = self.free.iter().map(|f| f.len).sum();
        let largest_free = self.free.iter().map(|f| f.len).max().unwrap_or(0);
        #[allow(clippy::cast_precision_loss)] // a ratio
        let fragmentation = if free_slots == 0 { 0.0 } else { 1.0 - largest_free as f32 / free_slots as f32 };
        SlotStats {
            high_water: self.high_water,
            live_slots: self.live.values().sum(),
            live_ranges: u32::try_from(self.live.len()).unwrap_or(u32::MAX),
            free_slots,
            free_ranges: u32::try_from(self.free.len()).unwrap_or(u32::MAX),
            largest_free,
            pending_slots: self.pending.iter().map(|(_, r)| r.len).sum(),
            fragmentation,
        }
    }
}

/// Slot granularity of a [`ClassAllocator`] block: the materialize dispatch is
/// `capacity / 64` workgroups, so a block boundary is a workgroup boundary.
pub const BLOCK_GRAIN: u32 = 64;

/// Size classes of a [`ClassAllocator`].
pub const CLASSES: usize = 32;

/// Block size per class, ascending: multiples of [`BLOCK_GRAIN`] up to 512 slots, then ×1.25
/// rounded up to the grain. The largest ([`CLASS_SIZES`]`[31]`, 129 088 slots ≈ 8 MB of 64 B
/// output slots) is far above any batch capacity we configure (`SpaceConfig::class_capacity`,
/// at most 16 384 by default), so a request that fits no class could not have fitted the batch either.
pub const CLASS_SIZES: [u32; CLASSES] = class_sizes();

#[allow(clippy::cast_possible_truncation)] // CLASSES is 32; `u32::try_from` is not const
const fn class_sizes() -> [u32; CLASSES] {
    let mut out = [0u32; CLASSES];
    let mut c = 0;
    while c < CLASSES {
        out[c] = if c < 8 {
            BLOCK_GRAIN * (c as u32 + 1)
        } else {
            let want = out[c - 1] + out[c - 1] / 4;
            let grains = want / BLOCK_GRAIN + if want % BLOCK_GRAIN == 0 { 0 } else { 1 };
            grains * BLOCK_GRAIN
        };
        c += 1;
    }
    out
}

/// The class a request of `len` slots takes, and that class's block size (always `>= len`);
/// `None` for `len == 0` or a `len` above the largest class.
///
/// A block wastes under `max(63 slots, 25 %)`: `block < 1.25 * len + 64` for every `len`.
#[must_use]
pub fn size_class(len: u32) -> Option<(u8, u32)> {
    if len == 0 {
        return None;
    }
    let class = CLASS_SIZES.iter().position(|&size| size >= len)?;
    Some((u8::try_from(class).ok()?, CLASS_SIZES[class]))
}

/// One live block of a [`ClassAllocator`].
#[derive(Clone, Copy, Debug)]
struct Block {
    class: u8,
    /// Slots the caller asked for (`<=` the block's size: the rest is this design's only waste).
    len: u32,
    /// The caller's label for what the block holds (`sf_scene_bevy`: the chain and level whose
    /// geometry this span draws).
    tag: u64,
}

/// Occupancy of one batch's output slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassStats {
    /// The batch's fixed output size.
    pub capacity: u32,
    /// Slots ever handed out as blocks: the allocator's own high-water mark (never shrinks).
    pub bump: u32,
    pub live_slots: u32,
    /// The most [`Self::live_slots`] ever live at once (what the buffer would need with perfect
    /// packing: `bump / peak_live_slots` is the price of this policy).
    pub peak_live_slots: u32,
    pub live_blocks: u32,
    /// Released and reusable now, in their own classes.
    pub free_blocks: u32,
    /// Released, waiting for the frames in flight.
    pub pending_blocks: u32,
    /// Slots inside live blocks nothing uses (`block size − len`), summed: the internal waste
    /// bought in exchange for zero external fragmentation.
    pub waste: u32,
}

/// A batch's output slots: segregated size classes over a fixed capacity.
///
/// One per batch (static-scene.md §3, "The GPU scene"; `scene-runtime-plan.md` §4.4). A request
/// takes a whole block of its [`size_class`], and a block is **never split and never merged**, so
/// a freed class-`c` block satisfies *any* later class-`c` request: external fragmentation inside
/// a class is zero, and the only growth is
///
/// ```text
/// bump = Σ_c size(c) · peak(c)
/// ```
///
/// with `peak(c)` the most class-`c` blocks ever occupied at once — live, or still waiting for the
/// frames in flight. That identity is exact (`bump_is_bounded_under_a_city_walk` asserts it at
/// twenty stages of a 200 000-cycle walk), and with it comes the one bound that holds a priori:
/// `peak(c) ≤ resident cells + frames in flight`, so
///
/// ```text
/// bump ≤ (resident + frames in flight) · Σ_{c reachable} size(c)
/// ```
///
/// — computable from the ring configuration before a cell streams in, and **independent of session
/// length**. That is what "bounded" means here, and all it means.
///
/// **What the plan claims and the measurement denies** (`scene-runtime-plan.md` §4.4 "Verdict:
/// bounded", to be corrected): `bump` is *not* `≤ 1.25 × the live set + 64·CLASSES`, and it does
/// *not* converge. The classes do not peak together, and `peak(c)` is a record process whose
/// records keep being beaten, ever more rarely. On a synthetic 5×5 ring over 40 cell footprints
/// (7..=638 instances, nine classes) `bump / peak_live` measures **3.0× after 200 000 stream-in
/// cycles and 3.33× after 2 M**, still creeping, against an a-priori ceiling of 7.0×. The same
/// sequence through first-fit [`SlotAllocator`] settles at **1.2×**
/// (`first_fit_high_water_on_the_same_walk`): Robson's ≈ 11× is adversarial, and a streaming ring
/// with a content-bounded size set is not an adversary.
///
/// So the case for segregated classes over a batch's output slots is the a-priori ceiling (no
/// appeal to the sequence), [`BLOCK_GRAIN`]-aligned block starts for the materialize dispatch, and
/// O(1) allocation with no free-list scan — **not** a smaller high-water mark. Budget a batch's
/// `capacity` from the ceiling, never from `1.25 × the live set`. The internal waste the plan
/// predicts does hold: `size(c) < 1.25·len + 64`, measured under 1 000 slots across 25 live cells.
///
/// `capacity` is **fixed** (the batch's output buffer and its materialize bind group are sized
/// from it, and growing them at stream-in would rebuild both): a request that does not fit is
/// [`AllocError::Exhausted`], and the caller leaves those instances to the entity path.
/// `capacity` should be a multiple of [`BLOCK_GRAIN`]; the last `capacity % 64` slots are
/// otherwise unreachable.
///
/// Release is deferred by the frames in flight, exactly as [`SlotAllocator`]'s is
/// ([`Self::release`] then [`Self::advance`]), so a reused block can never be read by a frame
/// still holding the old contents. Within a class the free list is LIFO, so the whole allocator
/// is deterministic: the same sequence of calls gives the same slots.
#[derive(Clone, Debug)]
pub struct ClassAllocator {
    capacity: u32,
    bump: u32,
    /// Free block starts per class, LIFO.
    free: [Vec<u32>; CLASSES],
    /// start → block.
    live: BTreeMap<u32, Block>,
    /// (release frame, start, class).
    pending: Vec<(u64, u32, u8)>,
    frames_in_flight: u64,
    live_slots: u32,
    peak_live_slots: u32,
}

impl ClassAllocator {
    /// An empty allocator over `capacity` slots whose released blocks wait `frames_in_flight`
    /// frames before reuse.
    #[must_use]
    pub fn new(capacity: u32, frames_in_flight: u32) -> Self {
        Self {
            capacity,
            bump: 0,
            free: std::array::from_fn(|_| Vec::new()),
            live: BTreeMap::new(),
            pending: Vec::new(),
            frames_in_flight: u64::from(frames_in_flight),
            live_slots: 0,
            peak_live_slots: 0,
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// `len` slots labelled `tag`, from a free block of their class or from the end.
    ///
    /// The range is the `len` slots the caller asked for; the rest of the block is its own and is
    /// returned to its class on [`Self::release`].
    pub fn alloc(&mut self, len: u32, tag: u64) -> Result<SlotRange, AllocError> {
        if len == 0 {
            return Err(AllocError::ZeroLength);
        }
        let exhausted = || AllocError::Exhausted { want: len, capacity: self.capacity };
        let (class, size) = size_class(len).ok_or_else(exhausted)?;
        let start = if let Some(start) = self.free[usize::from(class)].pop() {
            start
        } else {
            // A new block at the end, never a split of a bigger free one.
            let end = self.bump.checked_add(size).ok_or(AllocError::Overflow)?;
            if end > self.capacity {
                return Err(exhausted());
            }
            self.bump = end;
            end - size
        };
        self.live.insert(start, Block { class, len, tag });
        self.live_slots += len;
        self.peak_live_slots = self.peak_live_slots.max(self.live_slots);
        Ok(SlotRange { start, len })
    }

    /// Releases the block `range` came from at frame `frame`; its class gets it back at
    /// `frame + frames_in_flight`.
    pub fn release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError> {
        match self.live.get(&range.start) {
            Some(block) if block.len == range.len => {
                let class = block.class;
                self.live.remove(&range.start);
                self.live_slots -= range.len;
                self.pending.push((frame, range.start, class));
                Ok(())
            }
            _ => Err(AllocError::NotAllocated(range)),
        }
    }

    /// Returns every pending block whose frames in flight are done by `frame` to its class.
    pub fn advance(&mut self, frame: u64) {
        let window = self.frames_in_flight;
        let mut done = Vec::new();
        self.pending.retain(|&(at, start, class)| {
            let ready = at.saturating_add(window) <= frame;
            if ready {
                done.push((start, class));
            }
            !ready
        });
        for (start, class) in done {
            self.free[usize::from(class)].push(start);
        }
    }

    /// The live blocks in address order, each with the caller's tag: the range list of the
    /// batch is built from this.
    pub fn blocks(&self) -> impl Iterator<Item = (SlotRange, u64)> + '_ {
        self.live.iter().map(|(&start, b)| (SlotRange { start, len: b.len }, b.tag))
    }

    /// The live block starting at `start`, with its tag.
    pub fn block(&self, start: u32) -> Option<(SlotRange, u64)> {
        self.live.get(&start).map(|b| (SlotRange { start, len: b.len }, b.tag))
    }

    /// Free blocks of `class` (for the fragmentation report and the bound's test).
    pub fn free_blocks_in(&self, class: u8) -> usize {
        self.free.get(usize::from(class)).map_or(0, Vec::len)
    }

    pub fn stats(&self) -> ClassStats {
        ClassStats {
            capacity: self.capacity,
            bump: self.bump,
            live_slots: self.live_slots,
            peak_live_slots: self.peak_live_slots,
            live_blocks: u32::try_from(self.live.len()).unwrap_or(u32::MAX),
            free_blocks: u32::try_from(self.free.iter().map(Vec::len).sum::<usize>()).unwrap_or(u32::MAX),
            pending_blocks: u32::try_from(self.pending.len()).unwrap_or(u32::MAX),
            waste: self.live.values().map(|b| CLASS_SIZES[usize::from(b.class)] - b.len).sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fixed_capacity_table_is_exhausted_not_grown() {
        let mut a = SlotAllocator::with_capacity(10, 2);
        assert_eq!(a.capacity(), Some(10));
        assert_eq!(a.alloc(6), Ok(SlotRange { start: 0, len: 6 }));
        // Past the ceiling: an error, and the allocator is exactly as it was.
        let before = a.stats();
        assert_eq!(a.alloc(5), Err(AllocError::Exhausted { want: 5, capacity: 10 }));
        assert_eq!(a.stats(), before);
        // What fits still fits, and the capacity is reachable to its last slot.
        assert_eq!(a.alloc(4), Ok(SlotRange { start: 6, len: 4 }));
        assert_eq!(a.stats().high_water, 10);
        assert_eq!(a.alloc(1), Err(AllocError::Exhausted { want: 1, capacity: 10 }));
        // A released range comes back after the frames in flight, and first fit takes it.
        a.release(SlotRange { start: 0, len: 6 }, 7).unwrap();
        a.advance(9);
        assert_eq!(a.alloc(6), Ok(SlotRange { start: 0, len: 6 }));
        // A hole touching the high-water mark is extended, not skipped — and still capped.
        let mut b = SlotAllocator::with_capacity(10, 0);
        assert_eq!(b.alloc(8), Ok(SlotRange { start: 0, len: 8 }));
        b.release(SlotRange { start: 0, len: 8 }, 0).unwrap();
        b.advance(0);
        assert_eq!(b.alloc(10), Ok(SlotRange { start: 0, len: 10 }));
        // The unbounded allocator is untouched by all this.
        let mut c = SlotAllocator::new(0);
        assert_eq!(c.capacity(), None);
        assert_eq!(c.alloc(1_000_000), Ok(SlotRange { start: 0, len: 1_000_000 }));
    }

    fn overlaps(a: SlotRange, b: SlotRange) -> bool {
        a.start < b.end() && b.start < a.end()
    }

    /// Deterministic xorshift32, so every test below is the same run on every machine.
    fn rng(seed: u32) -> impl FnMut() -> u32 {
        let mut s = seed;
        move || {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            s
        }
    }

    #[test]
    fn size_classes_ascend_and_waste_under_a_quarter() {
        assert_eq!(CLASS_SIZES[0], 64);
        assert_eq!(CLASS_SIZES[7], 512);
        assert_eq!(CLASS_SIZES[8], 640);
        assert!(CLASS_SIZES.windows(2).all(|w| w[0] < w[1]), "{CLASS_SIZES:?}");
        assert!(CLASS_SIZES.iter().all(|s| s.is_multiple_of(BLOCK_GRAIN)));
        // Every class is far enough apart to be a class, and no class is a no-op.
        assert_eq!(size_class(0), None);
        assert_eq!(size_class(1), Some((0, 64)));
        assert_eq!(size_class(64), Some((0, 64)));
        assert_eq!(size_class(65), Some((1, 128)));
        assert_eq!(size_class(512), Some((7, 512)));
        assert_eq!(size_class(513), Some((8, 640)));
        let top = CLASS_SIZES[CLASSES - 1];
        assert_eq!(top, 129_088, "the largest class, quoted in CLASS_SIZES' docs");
        assert_eq!(size_class(top), Some((31, top)));
        assert_eq!(size_class(top + 1), None, "above the largest class");
        // The waste bound, over every length a class can be asked for.
        for len in 1..=top {
            let (class, block) = size_class(len).unwrap();
            assert!(block >= len, "{len}: block {block}");
            assert_eq!(block, CLASS_SIZES[usize::from(class)]);
            assert!(
                f64::from(block) < 1.25 * f64::from(len) + f64::from(BLOCK_GRAIN),
                "{len}: block {block} wastes too much"
            );
        }
    }

    #[test]
    fn blocks_are_never_split_and_a_freed_block_satisfies_its_class() {
        let mut a = ClassAllocator::new(4096, 0);
        // 100 slots take a 128-slot block; 300 take 320.
        let big = a.alloc(100, 7).unwrap();
        assert_eq!(big, SlotRange { start: 0, len: 100 });
        let next = a.alloc(300, 8).unwrap();
        assert_eq!(next, SlotRange { start: 128, len: 300 }, "the next block starts after the whole first one");
        assert_eq!(a.stats().bump, 128 + 320);
        assert_eq!(a.stats().waste, 28 + 20);
        a.release(big, 0).unwrap();
        a.advance(0);
        // A smaller request is a different class, so it never splits the freed block.
        let small = a.alloc(10, 9).unwrap();
        assert_eq!(small, SlotRange { start: 448, len: 10 });
        assert_eq!(a.free_blocks_in(1), 1, "the 128-slot block is still waiting for its own class");
        // Any later class-1 request takes it, whatever its length inside the class.
        let reuse = a.alloc(65, 10).unwrap();
        assert_eq!(reuse, SlotRange { start: 0, len: 65 });
        assert_eq!(a.free_blocks_in(1), 0);
        assert_eq!(a.stats().bump, 128 + 320 + 64, "reuse grew nothing");
        // Blocks come back in address order with their tags.
        let blocks: Vec<_> = a.blocks().collect();
        assert_eq!(
            blocks,
            [(SlotRange { start: 0, len: 65 }, 10), (SlotRange { start: 128, len: 300 }, 8), (SlotRange { start: 448, len: 10 }, 9)]
        );
        assert_eq!(a.block(128), Some((SlotRange { start: 128, len: 300 }, 8)));
        assert_eq!(a.block(1), None);
    }

    #[test]
    fn exhaustion_is_an_error_not_a_panic() {
        // Capacity 256: four 64-slot blocks, and then nothing.
        let mut a = ClassAllocator::new(256, 0);
        let r: Vec<_> = (0..4).map(|k| a.alloc(1, k).unwrap()).collect();
        assert_eq!(r[3], SlotRange { start: 192, len: 1 });
        assert_eq!(a.alloc(1, 4), Err(AllocError::Exhausted { want: 1, capacity: 256 }));
        assert_eq!(a.stats().bump, 256, "a failed alloc changes nothing");
        // A request no class could hold, and a zero-length one.
        assert_eq!(a.alloc(0, 0), Err(AllocError::ZeroLength));
        assert_eq!(
            a.alloc(CLASS_SIZES[CLASSES - 1] + 1, 0),
            Err(AllocError::Exhausted { want: CLASS_SIZES[CLASSES - 1] + 1, capacity: 256 })
        );
        // Freeing one makes exactly one more fit, in its own class.
        a.release(r[0], 0).unwrap();
        a.advance(0);
        assert_eq!(a.alloc(64, 5).unwrap(), SlotRange { start: 0, len: 64 });
        assert_eq!(a.alloc(65, 6), Err(AllocError::Exhausted { want: 65, capacity: 256 }));
        // A range that was never live, and a double release.
        assert_eq!(a.release(SlotRange { start: 3, len: 1 }, 1), Err(AllocError::NotAllocated(SlotRange { start: 3, len: 1 })));
        a.release(r[1], 1).unwrap();
        assert_eq!(a.release(r[1], 1), Err(AllocError::NotAllocated(r[1])));
    }

    #[test]
    fn a_released_block_waits_for_the_frames_in_flight() {
        let mut a = ClassAllocator::new(4096, 3);
        let first = a.alloc(64, 1).unwrap();
        a.release(first, 100).unwrap();
        assert_eq!(a.stats().pending_blocks, 1);
        for frame in 101..=102 {
            a.advance(frame);
            // Still readable by a frame in flight: the next request must not land on it.
            let other = a.alloc(64, 2).unwrap();
            assert!(!overlaps(first, other), "frame {frame}");
        }
        a.advance(103);
        assert_eq!((a.stats().pending_blocks, a.free_blocks_in(0)), (0, 1));
        assert_eq!(a.alloc(64, 3).unwrap(), first);
    }

    /// The walk's own books, so the test can check the allocator against them. A class's blocks
    /// are *occupied* while they are live **and** while they wait for the frames in flight: that
    /// is what the allocator cannot hand out again, so that is the peak.
    struct Walk {
        resident: std::collections::VecDeque<(SlotRange, u8)>,
        /// (frame the block returns to its class, class).
        waiting: std::collections::VecDeque<(u64, u8)>,
        occupied_per_class: [u32; CLASSES],
        peak_per_class: [u32; CLASSES],
        longest_per_class: [u32; CLASSES],
    }

    impl Walk {
        fn new() -> Self {
            Self {
                resident: std::collections::VecDeque::new(),
                waiting: std::collections::VecDeque::new(),
                occupied_per_class: [0; CLASSES],
                peak_per_class: [0; CLASSES],
                longest_per_class: [0; CLASSES],
            }
        }

        /// Streams one cell in per frame and, past `resident`'s size, the oldest one out.
        fn walk(
            &mut self,
            a: &mut ClassAllocator,
            next: &mut dyn FnMut() -> u32,
            sizes: &[u32],
            resident: usize,
            flight: u64,
            frames: std::ops::Range<u64>,
        ) {
            for frame in frames {
                a.advance(frame);
                while self.waiting.front().is_some_and(|&(ready, _)| ready <= frame) {
                    let (_, class) = self.waiting.pop_front().expect("front");
                    self.occupied_per_class[usize::from(class)] -= 1;
                }
                let pick = usize::try_from(next()).expect("u32 fits usize") % sizes.len();
                let len = sizes[pick];
                let (class, _) = size_class(len).expect("a cell footprint has a class");
                let range = a.alloc(len, u64::from(class)).expect("capacity");
                for (other, _) in &self.resident {
                    assert!(!overlaps(range, *other), "frame {frame}: {range:?} overlaps {other:?}");
                }
                self.resident.push_back((range, class));
                let c = usize::from(class);
                self.occupied_per_class[c] += 1;
                self.peak_per_class[c] = self.peak_per_class[c].max(self.occupied_per_class[c]);
                self.longest_per_class[c] = self.longest_per_class[c].max(len);
                if self.resident.len() > resident {
                    let (old, old_class) = self.resident.pop_front().expect("resident");
                    a.release(old, frame).expect("a live block");
                    self.waiting.push_back((frame + flight, old_class));
                }
            }
        }
    }

    /// Cell footprints of the synthetic walk: 40 of them, 7..=638 instances, the spread of a
    /// Vvardenfell ring. They fall in classes 0..=8, so the walk exercises nine free lists.
    const WALK_SIZES: usize = 40;
    /// Cells resident at once (a 5×5 ring).
    const WALK_RESIDENT: usize = 25;
    const WALK_FLIGHT: u64 = 3;

    fn walk_sizes() -> [u32; WALK_SIZES] {
        std::array::from_fn(|i| {
            let i = u32::try_from(i).expect("WALK_SIZES fits u32");
            7 + i * 13 + (i % 5) * 31
        })
    }

    /// The only bound of a [`ClassAllocator`] that holds a priori: every class the size set can
    /// ask for, each holding as many blocks as the ring can have occupied at once (resident plus
    /// the ones still waiting for the frames in flight). Computable from the ring configuration
    /// before a single cell streams in, and independent of session length — which is what
    /// "bounded" means here, and all it means.
    fn occupancy_ceiling(sizes: &[u32], resident: usize, flight: u64) -> u32 {
        let mut per_class = [0u32; CLASSES];
        for &len in sizes {
            let (class, _) = size_class(len).expect("a footprint has a class");
            per_class[usize::from(class)] = 1;
        }
        let blocks = u32::try_from(resident).expect("small") + u32::try_from(flight).expect("small");
        (0..CLASSES).map(|c| CLASS_SIZES[c] * per_class[c] * blocks).sum()
    }

    /// The design's load-bearing claim, measured rather than argued — and **the plan's version of
    /// it is false** (`scene-runtime-plan.md` §4.4 "Verdict: bounded").
    ///
    /// What holds:
    /// - the identity `bump = Σ_c size(c)·peak(c)`, exactly, with `peak(c)` the most class-`c`
    ///   blocks ever occupied at once (live, or waiting for the frames in flight). Blocks are
    ///   never split and never merged, so nothing else can make the allocator grow: **external
    ///   fragmentation inside a class is zero**;
    /// - `bump ≤` [`occupancy_ceiling`], an a-priori ceiling from the ring configuration alone,
    ///   independent of how long the session runs.
    ///
    /// What does **not** hold: `bump ≤ 1.25·peak_live + 64·CLASSES`. The classes do not peak
    /// together, so `bump` is measured at **3.0 × the peak live slots** after 200 000 cycles (and
    /// still creeping: 3.33× at 2 M, `class_allocator_high_water_curve`). `peak(c)` is a record
    /// process — each class's record occupancy keeps being beaten, ever more rarely — so the
    /// high-water mark does not converge within any session length we can test; it only flattens
    /// logarithmically under its ceiling. `first_fit_high_water_on_the_same_walk` is the contrast
    /// the plan expected to lose.
    #[test]
    fn bump_is_bounded_under_a_city_walk() {
        const CYCLES: u64 = 10_000;
        let sizes = walk_sizes();
        let ceiling = occupancy_ceiling(&sizes, WALK_RESIDENT, WALK_FLIGHT);
        assert_eq!(ceiling, 82_432, "nine classes × 28 occupiable blocks");
        let mut a = ClassAllocator::new(1 << 20, u32::try_from(WALK_FLIGHT).expect("small"));
        let mut next = rng(0x9E37_79B9);
        let mut w = Walk::new();
        let mut curve = Vec::new();
        for seg in 0..20 {
            w.walk(&mut a, &mut next, &sizes, WALK_RESIDENT, WALK_FLIGHT, seg * CYCLES..(seg + 1) * CYCLES);
            let s = a.stats();
            // The identity, re-checked at every stage of the walk.
            let identity: u32 = (0..CLASSES).map(|c| CLASS_SIZES[c] * w.peak_per_class[c]).sum();
            assert_eq!(s.bump, identity, "after {} cycles: bump is Σ_c size(c)·peak(c)", (seg + 1) * CYCLES);
            // The a-priori ceiling.
            assert!(s.bump <= ceiling, "after {} cycles: bump {} > ceiling {ceiling}", (seg + 1) * CYCLES, s.bump);
            // Monotone, and the ring is the only thing live.
            assert!(s.bump >= curve.last().copied().unwrap_or(0));
            assert_eq!(s.live_blocks, u32::try_from(WALK_RESIDENT).expect("small"));
            curve.push(s.bump);
        }
        let end = a.stats();
        let classes_in_play = w.peak_per_class.iter().filter(|&&p| p > 0).count();
        assert_eq!(classes_in_play, 9);
        let ratio = f64::from(end.bump) / f64::from(end.peak_live_slots);
        println!(
            "city walk, 200 000 cycles: bump {} peak_live {} ratio {ratio:.2} ceiling {ceiling} \
             waste {} over {classes_in_play} classes; curve {curve:?}",
            end.bump, end.peak_live_slots, end.waste
        );
        // The disproof, pinned: the plan's bound is violated by more than a factor of two. If a
        // later change makes this fail *low*, the plan was right after all and §4.4 can stand.
        let plan_bound = 1.25 * f64::from(end.peak_live_slots)
            + f64::from(BLOCK_GRAIN) * f64::from(u32::try_from(CLASSES).expect("small"));
        assert!(f64::from(end.bump) > plan_bound, "bump {} <= the plan's bound {plan_bound:.0}", end.bump);
        assert!((2.5..4.0).contains(&ratio), "measured price of the policy moved: {ratio:.2}");
        // Still creeping at the end of the walk: the last increase is in the second half.
        let half = curve[curve.len() / 2 - 1];
        assert!(end.bump > half, "bump {} did not grow after 100 000 cycles ({half})", end.bump);
        // Internal waste is the part the plan gets right: under a quarter of the live slots plus
        // one grain per live block (`size(c) < 1.25·len + 64`, summed over the resident cells).
        let waste_bound = end.live_slots / 4 + BLOCK_GRAIN * u32::try_from(WALK_RESIDENT).expect("small");
        assert!(end.waste < waste_bound, "waste {} >= {waste_bound}", end.waste);
    }

    /// The contrast the plan claims is the bad one: the **same** walk through the first-fit
    /// [`SlotAllocator`] over mixed sizes. Its measured high-water mark is **1.2 × the peak live
    /// slots** — not the ≈ 11× of Robson's worst case, and 2.7× tighter than the segregated
    /// classes on the identical sequence. Robson's bound is adversarial; a streaming ring with a
    /// content-bounded size set is nowhere near adversarial, and coalescing keeps the holes usable.
    ///
    /// So the reason to prefer [`ClassAllocator`] for a batch's output slots is **not** the
    /// high-water mark: it is the a-priori ceiling (`occupancy_ceiling`, no appeal to the
    /// sequence), [`BLOCK_GRAIN`]-aligned block starts for the materialize dispatch, and O(1)
    /// allocation. Where neither is needed, first fit is the cheaper policy.
    #[test]
    fn first_fit_high_water_on_the_same_walk() {
        const CYCLES: u64 = 200_000;
        let sizes = walk_sizes();
        let mut a = SlotAllocator::new(u32::try_from(WALK_FLIGHT).expect("small"));
        let mut next = rng(0x9E37_79B9);
        let mut resident: std::collections::VecDeque<SlotRange> = std::collections::VecDeque::new();
        let mut peak_live = 0;
        for frame in 0..CYCLES {
            a.advance(frame);
            let len = sizes[usize::try_from(next()).expect("u32 fits usize") % sizes.len()];
            let range = a.alloc(len).expect("first fit never refuses");
            for other in &resident {
                assert!(!overlaps(range, *other), "frame {frame}");
            }
            resident.push_back(range);
            peak_live = peak_live.max(a.stats().live_slots);
            if resident.len() > WALK_RESIDENT {
                a.release(resident.pop_front().expect("resident"), frame).expect("a live range");
            }
        }
        let s = a.stats();
        let ratio = f64::from(s.high_water) / f64::from(peak_live);
        println!(
            "first fit, same walk: high_water {} peak_live {peak_live} ratio {ratio:.2} \
             free_ranges {} fragmentation {:.2}",
            s.high_water, s.free_ranges, s.fragmentation
        );
        assert!(ratio < 1.5, "first fit's high-water mark moved: {ratio:.2}");
        // And the free space stays a handful of coalesced holes, not a sieve.
        assert!(s.free_ranges < 16, "free ranges {}", s.free_ranges);
    }

    /// Where the high-water mark goes over a session far longer than any test should run: it is
    /// still creeping at 2 M stream-in cycles (≈ 9 h at 60 fps), under its ceiling.
    #[test]
    #[ignore = "measurement: the ClassAllocator high-water curve over 2 M stream-in cycles"]
    fn class_allocator_high_water_curve() {
        const SEG: u64 = 10_000;
        let sizes = walk_sizes();
        let mut a = ClassAllocator::new(1 << 22, u32::try_from(WALK_FLIGHT).expect("small"));
        let mut next = rng(0x9E37_79B9);
        let mut w = Walk::new();
        let mut last = 0;
        for seg in 0..200 {
            w.walk(&mut a, &mut next, &sizes, WALK_RESIDENT, WALK_FLIGHT, seg * SEG..(seg + 1) * SEG);
            let s = a.stats();
            if s.bump != last {
                last = s.bump;
                println!(
                    "cycles {:>8} bump {:>6} peak_live {:>5} ratio {:.2} waste {:>5}",
                    (seg + 1) * SEG,
                    s.bump,
                    s.peak_live_slots,
                    f64::from(s.bump) / f64::from(s.peak_live_slots),
                    s.waste
                );
            }
        }
        println!(
            "peaks per class {:?}, longest request per class {:?}, ceiling {}",
            &w.peak_per_class[..9],
            &w.longest_per_class[..9],
            occupancy_ceiling(&sizes, WALK_RESIDENT, WALK_FLIGHT)
        );
    }

    #[test]
    fn allocates_contiguous_disjoint_ranges() {
        let mut a = SlotAllocator::new(2);
        let r: Vec<_> = [10, 1, 300, 7].map(|n| a.alloc(n).unwrap()).into();
        assert_eq!(r[0], SlotRange { start: 0, len: 10 });
        assert_eq!(r[1], SlotRange { start: 10, len: 1 });
        assert_eq!(r[3], SlotRange { start: 311, len: 7 });
        for (i, x) in r.iter().enumerate() {
            for y in &r[i + 1..] {
                assert!(!overlaps(*x, *y));
            }
        }
        let s = a.stats();
        assert_eq!((s.high_water, s.live_slots, s.live_ranges, s.free_slots), (318, 318, 4, 0));
        assert_eq!(a.alloc(0), Err(AllocError::ZeroLength));
    }

    #[test]
    fn release_waits_for_the_frames_in_flight() {
        let mut a = SlotAllocator::new(2);
        let r0 = a.alloc(10).unwrap();
        let _r1 = a.alloc(10).unwrap();
        a.release(r0, 100).unwrap();
        assert_eq!(a.stats().pending_slots, 10);
        a.advance(101);
        // Still read by a frame in flight: a new range must not reuse it.
        let r2 = a.alloc(10).unwrap();
        assert_eq!(r2.start, 20);
        assert!(!overlaps(r0, r2));
        a.advance(102);
        let s = a.stats();
        assert_eq!((s.pending_slots, s.free_slots, s.largest_free), (0, 10, 10));
        // After the window the hole is reused.
        assert_eq!(a.alloc(4).unwrap(), SlotRange { start: 0, len: 4 });
        assert_eq!(a.alloc(6).unwrap(), SlotRange { start: 4, len: 6 });
        assert_eq!(a.stats().high_water, 30);
    }

    #[test]
    fn rejects_unknown_and_double_release() {
        let mut a = SlotAllocator::new(1);
        let r = a.alloc(5).unwrap();
        assert_eq!(a.release(SlotRange { start: 0, len: 4 }, 0), Err(AllocError::NotAllocated(SlotRange { start: 0, len: 4 })));
        a.release(r, 0).unwrap();
        assert_eq!(a.release(r, 0), Err(AllocError::NotAllocated(r)));
        assert_eq!(a.live_range(0), None);
    }

    #[test]
    fn coalesces_and_reports_fragmentation() {
        let mut a = SlotAllocator::new(0);
        let r: Vec<_> = [4, 4, 4, 4].map(|n| a.alloc(n).unwrap()).into();
        a.release(r[0], 0).unwrap();
        a.release(r[2], 0).unwrap();
        a.advance(0);
        let s = a.stats();
        assert_eq!((s.free_ranges, s.free_slots, s.largest_free), (2, 8, 4));
        assert!((s.fragmentation - 0.5).abs() < 1e-6);
        // A range of 6 fits no hole: it goes at the end.
        assert_eq!(a.alloc(6).unwrap().start, 16);
        a.release(r[1], 1).unwrap();
        a.advance(1);
        let s = a.stats();
        assert_eq!((s.free_ranges, s.free_slots, s.largest_free, s.fragmentation), (1, 12, 12, 0.0));
        assert_eq!(a.alloc(12).unwrap(), SlotRange { start: 0, len: 12 });
    }

    #[test]
    fn a_hole_at_the_end_is_extended() {
        let mut a = SlotAllocator::new(0);
        let _r0 = a.alloc(4).unwrap();
        let r1 = a.alloc(4).unwrap();
        a.release(r1, 0).unwrap();
        a.advance(0);
        assert_eq!(a.alloc(10).unwrap(), SlotRange { start: 4, len: 10 });
        let s = a.stats();
        assert_eq!((s.high_water, s.free_slots), (14, 0));
    }

    #[test]
    fn churn_never_overlaps_live_or_pending_ranges() {
        // Deterministic stream-in/out over a few hundred frames.
        let mut a = SlotAllocator::new(3);
        let mut live: Vec<SlotRange> = Vec::new();
        let mut pending: Vec<(u64, SlotRange)> = Vec::new();
        let mut seed = 0x9E37_79B9u32;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed
        };
        for frame in 0..400u64 {
            a.advance(frame);
            pending.retain(|&(at, _)| at + 3 > frame);
            if next().is_multiple_of(3) && !live.is_empty() {
                let r = live.swap_remove(next() as usize % live.len());
                a.release(r, frame).unwrap();
                pending.push((frame, r));
            } else {
                let r = a.alloc(next() % 64 + 1).unwrap();
                for other in live.iter().chain(pending.iter().map(|(_, p)| p)) {
                    assert!(!overlaps(r, *other), "frame {frame}: {r:?} overlaps {other:?}");
                }
                live.push(r);
            }
        }
        let s = a.stats();
        assert_eq!(s.live_slots, live.iter().map(|r| r.len).sum::<u32>());
        assert_eq!(s.live_slots + s.free_slots + s.pending_slots, s.high_water);
    }
}
