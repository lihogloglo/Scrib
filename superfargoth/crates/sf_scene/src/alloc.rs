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

#[cfg(test)]
mod tests {
    use super::*;

    fn overlaps(a: SlotRange, b: SlotRange) -> bool {
        a.start < b.end() && b.start < a.end()
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
