//! The per-space GPU instance table's slot allocator (static-scene.md §3, "The GPU scene").
//!
//! Stream-in takes one contiguous range per cell; stream-out releases it, but the slots return
//! to the free list only once the frames in flight that may still read them are done
//! ([`SlotAllocator::release`] then [`SlotAllocator::advance`]). First fit over a coalesced free
//! list; when nothing fits the table grows at its end (the high-water mark is the buffer size the
//! Bevy side must provide). No compaction: [`SlotStats`] reports fragmentation instead.

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
    /// A fixed-capacity allocator ([`ClassAllocator`]) is full. Not an error of the caller's:
    /// the cell publishes without those instances and they spawn as entities instead.
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
            high_water: 0,
            free: Vec::new(),
            live: BTreeMap::new(),
            pending: Vec::new(),
        }
    }

    /// A contiguous range of `len` slots (first fit, else at the end of the table).
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
/// output slots) is far above any batch capacity we configure, so a request that fits no class
/// could not have fitted the batch either.
pub const CLASS_SIZES: [u32; CLASSES] = class_sizes();

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

/// A batch's output slots: segregated size classes over a fixed capacity, **bounded**.
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
/// with `peak(c)` the most simultaneously live class-`c` blocks over the session — monotone,
/// and it *converges* once the worst neighbourhood has been walked, instead of creeping the way
/// first fit over mixed sizes does (bounded only by Robson's `O(M·log(max/min))`, ≈ 11× the live
/// set at our size ratio). The price is bounded internal waste, `size(c) < 1.25·len + 64`.
///
/// `peak(c)` is per class, and two classes may peak at different times, so `bump` is **not**
/// bounded by `1.25 × the live set`: it is bounded by `Σ_c (1.25·len(c) + 64)·peak(c)`, and the
/// honest statement is that the sum over classes is what it costs. `bump_is_bounded_under_a_city_walk`
/// demonstrates the identity, the convergence and the ratio.
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
        let start = match self.free[usize::from(class)].pop() {
            Some(start) => start,
            None => {
                // A new block at the end, never a split of a bigger free one.
                let end = self.bump.checked_add(size).ok_or(AllocError::Overflow)?;
                if end > self.capacity {
                    return Err(exhausted());
                }
                self.bump = end;
                end - size
            }
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

    /// The claim the whole design rests on: the high-water mark is `Σ_c size(c)·peak(c)`, it
    /// stops growing once the worst neighbourhood has been walked, and nothing creeps.
    #[test]
    fn bump_is_bounded_under_a_city_walk() {
        const SIZES: usize = 40;
        const RESIDENT: usize = 25;
        const CYCLES: u64 = 10_000;
        // 40 cell footprints, 7..638 instances: the spread of a Vvardenfell ring.
        let sizes: [u32; SIZES] = std::array::from_fn(|i| {
            let i = i as u32;
            7 + i * 13 + (i % 5) * 31
        });
        /// The walk's own books, so the test can check the allocator against them. A class's
        /// blocks are *occupied* while they are live **and** while they wait for the frames in
        /// flight: that is what the allocator cannot hand out again, so that is the peak.
        struct Walk {
            resident: std::collections::VecDeque<(SlotRange, u8)>,
            /// (frame the block returns to its class, class).
            waiting: std::collections::VecDeque<(u64, u8)>,
            occupied_per_class: [u32; CLASSES],
            peak_per_class: [u32; CLASSES],
            longest_per_class: [u32; CLASSES],
        }
        /// Streams one cell in per frame and, past the ring's size, the oldest one out.
        fn walk(
            a: &mut ClassAllocator,
            w: &mut Walk,
            next: &mut dyn FnMut() -> u32,
            sizes: &[u32],
            frames: std::ops::Range<u64>,
        ) {
            for frame in frames {
                a.advance(frame);
                while w.waiting.front().is_some_and(|&(ready, _)| ready <= frame) {
                    let (_, class) = w.waiting.pop_front().expect("front");
                    w.occupied_per_class[usize::from(class)] -= 1;
                }
                let len = sizes[next() as usize % sizes.len()];
                let (class, _) = size_class(len).expect("a cell footprint has a class");
                let range = a.alloc(len, u64::from(class)).expect("capacity");
                for (other, _) in &w.resident {
                    assert!(!overlaps(range, *other), "frame {frame}: {range:?} overlaps {other:?}");
                }
                w.resident.push_back((range, class));
                let c = usize::from(class);
                w.occupied_per_class[c] += 1;
                w.peak_per_class[c] = w.peak_per_class[c].max(w.occupied_per_class[c]);
                w.longest_per_class[c] = w.longest_per_class[c].max(len);
                if w.resident.len() > RESIDENT {
                    let (old, old_class) = w.resident.pop_front().expect("resident");
                    a.release(old, frame).expect("a live block");
                    w.waiting.push_back((frame + 3, old_class));
                }
            }
        }
        let mut a = ClassAllocator::new(1 << 20, 3);
        let mut next = rng(0x9E37_79B9);
        let mut w = Walk {
            resident: std::collections::VecDeque::new(),
            live_per_class: [0; CLASSES],
            peak_per_class: [0; CLASSES],
            longest_per_class: [0; CLASSES],
        };
        walk(&mut a, &mut w, &mut next, &sizes, 0..CYCLES);
        let after_first_walk = a.stats();
        // The identity: blocks are never split and never merged, so the only growth is the
        // number of class-`c` blocks that were ever live at once.
        let identity: u32 = (0..CLASSES).map(|c| CLASS_SIZES[c] * w.peak_per_class[c]).sum();
        assert_eq!(after_first_walk.bump, identity, "bump is Σ_c size(c)·peak(c)");
        // The bound, honestly stated: `size(c) < 1.25·len + 64` per class, summed over classes.
        let classes_in_play = w.peak_per_class.iter().filter(|&&p| p > 0).count();
        let bound: f64 = (0..CLASSES)
            .map(|c| {
                (1.25 * f64::from(w.longest_per_class[c]) + f64::from(BLOCK_GRAIN)) * f64::from(w.peak_per_class[c])
            })
            .sum();
        assert!(f64::from(after_first_walk.bump) <= bound, "bump {} > bound {bound}", after_first_walk.bump);
        // And it converges: ten thousand more cycles of the same walk grow it by nothing.
        walk(&mut a, &mut w, &mut next, &sizes, CYCLES..2 * CYCLES);
        let after_second_walk = a.stats();
        assert_eq!(after_second_walk.bump, after_first_walk.bump, "the high-water mark creeps");
        assert_eq!(after_second_walk.live_blocks, RESIDENT as u32 + 1);
        // The measured price of the policy, for the record: bump / peak live slots. 25 cells of
        // 7..638 instances peak around 8 k slots; the classes do not peak together, which is
        // exactly why the bound is a sum over classes and not 1.25 × the live set.
        let ratio = f64::from(after_second_walk.bump) / f64::from(after_second_walk.peak_live_slots);
        assert!(
            (1.0..3.0).contains(&ratio),
            "bump {} peak_live {} ratio {ratio:.2} over {classes_in_play} classes",
            after_second_walk.bump,
            after_second_walk.peak_live_slots
        );
        assert!(after_second_walk.waste < after_second_walk.live_slots / 4 + BLOCK_GRAIN * RESIDENT as u32);
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
