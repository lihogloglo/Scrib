//! Power-of-two 2D quadtree / buddy allocator for square shadow-atlas tiles.

/// Maximum quadtree depth supported (`2^10 = 1024x` ratio between `atlas_size` and `min_tile`).
const MAX_TREE_DEPTH: usize = 10; // tunables: const (capacity: quadtree array bound)

/// A square tile allocated inside a 2D shadow atlas, in texels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AtlasRect {
    /// Left edge in texels.
    pub x: u16,
    /// Top edge in texels.
    pub y: u16,
    /// Side length in texels (power of two).
    pub size: u16,
}

impl AtlasRect {
    /// Normalized `([u_origin, v_origin], uv_scale)` inside a square `atlas_size × atlas_size` texture.
    #[allow(clippy::cast_precision_loss)]
    pub fn uv_rect(self, atlas_size: u32) -> ([f32; 2], f32) {
        let denom = atlas_size as f32;
        (
            [f32::from(self.x) / denom, f32::from(self.y) / denom],
            f32::from(self.size) / denom,
        )
    }

    /// Packed `[x, y, size]` representation in texels.
    pub fn to_array(self) -> [u16; 3] {
        [self.x, self.y, self.size]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeState {
    /// Covered by an ancestor that is `Free` or `Used`.
    Inactive,
    /// Active unallocated block at this node's level (parent, if any, is `Split`).
    Free,
    /// Subdivided into 4 active child nodes.
    Split,
    /// Allocated to a caller.
    Used,
}

/// 2D quadtree buddy allocator over a power-of-two square atlas.
///
/// - Allocations prefer an already-split branch with an exact-size free slot (then the smallest
///   larger free block) before splitting larger blocks, minimizing fragmentation.
/// - Frees coalesce automatically up the quadtree whenever all 4 siblings are free, guaranteeing
///   zero external fragmentation.
/// - Optional `reserved_texels` holds headroom that regular [`Self::alloc`] cannot consume, while
///   [`Self::alloc_handover`] may dip into it for simultaneous handover crossfades.
#[derive(Clone, Debug)]
pub struct BuddyAtlas {
    atlas_size: u16,
    min_tile: u16,
    max_tile: u16,
    max_depth: usize,
    total_texels: u64,
    used_texels: u64,
    reserved_texels: u64,
    alloc_count: usize,
    nodes: Vec<NodeState>,
    free_at_depth: Vec<u32>,
}

impl BuddyAtlas {
    /// Creates an empty buddy allocator covering `atlas_size × atlas_size` texels.
    ///
    /// # Preconditions
    /// - `atlas_size`, `min_tile`, and `max_tile` must all be powers of two.
    /// - `min_tile <= max_tile <= atlas_size`.
    /// - `(atlas_size / min_tile).trailing_zeros() <= 10`.
    pub fn new(atlas_size: u16, min_tile: u16, max_tile: u16) -> Self {
        assert!(
            atlas_size.is_power_of_two(),
            "atlas_size ({atlas_size}) must be a power of two"
        );
        assert!(
            min_tile.is_power_of_two(),
            "min_tile ({min_tile}) must be a power of two"
        );
        assert!(
            max_tile.is_power_of_two(),
            "max_tile ({max_tile}) must be a power of two"
        );
        assert!(
            min_tile <= max_tile && max_tile <= atlas_size,
            "expected min_tile ({min_tile}) <= max_tile ({max_tile}) <= atlas_size ({atlas_size})"
        );

        let max_depth = (atlas_size / min_tile).trailing_zeros() as usize;
        assert!(
            max_depth <= MAX_TREE_DEPTH,
            "atlas/min_tile depth ({max_depth}) exceeds maximum supported depth ({MAX_TREE_DEPTH})"
        );

        let total_nodes = level_start(max_depth + 1);
        let mut nodes = vec![NodeState::Inactive; total_nodes];
        nodes[0] = NodeState::Free;

        let mut free_at_depth = vec![0u32; max_depth + 1];
        free_at_depth[0] = 1;

        let side = u64::from(atlas_size);
        Self {
            atlas_size,
            min_tile,
            max_tile,
            max_depth,
            total_texels: side * side,
            used_texels: 0,
            reserved_texels: 0,
            alloc_count: 0,
            nodes,
            free_at_depth,
        }
    }

    /// Builder that sets the texel reserve protected from regular [`Self::alloc`] calls.
    #[must_use]
    pub fn with_reserve_texels(mut self, reserved_texels: u64) -> Self {
        self.reserved_texels = reserved_texels;
        self
    }

    /// Updates the texel reserve protected from regular [`Self::alloc`] calls.
    pub fn set_reserve_texels(&mut self, reserved_texels: u64) {
        self.reserved_texels = reserved_texels;
    }

    /// Texels reserved for handover allocations.
    pub fn reserve_texels(&self) -> u64 {
        self.reserved_texels
    }

    /// Allocates a `tile_size × tile_size` tile if remaining free texels stay `>= reserved_texels`.
    ///
    /// Prefers an already-split branch with an exact-size free slot before splitting a larger
    /// free block.
    pub fn alloc(&mut self, tile_size: u16) -> Option<AtlasRect> {
        self.alloc_inner(tile_size, false)
    }

    /// Allocates a `tile_size × tile_size` tile, permitted to dip into `reserved_texels` for
    /// simultaneous handover crossfades.
    pub fn alloc_handover(&mut self, tile_size: u16) -> Option<AtlasRect> {
        self.alloc_inner(tile_size, true)
    }

    /// Frees an active allocation and coalesces up the quadtree whenever all 4 siblings are free.
    ///
    /// Returns `false` if `rect` is invalid or not currently allocated.
    pub fn free(&mut self, rect: AtlasRect) -> bool {
        if !self.is_valid_tile_size(rect.size) {
            return false;
        }
        if rect.x >= self.atlas_size
            || rect.y >= self.atlas_size
            || !rect.x.is_multiple_of(rect.size)
            || !rect.y.is_multiple_of(rect.size)
        {
            return false;
        }

        let target_depth = self.depth_for_tile(rect.size);
        let idx = self.coords_to_index(rect.x, rect.y, target_depth);
        if self.nodes[idx] != NodeState::Used {
            return false;
        }

        let tile_texels = u64::from(rect.size) * u64::from(rect.size);
        self.nodes[idx] = NodeState::Free;
        self.free_at_depth[target_depth] += 1;
        self.used_texels -= tile_texels;
        self.alloc_count -= 1;

        let mut cur = idx;
        let mut cur_depth = target_depth;
        while cur > 0 {
            let parent = (cur - 1) / 4;
            let first_child = 4 * parent + 1;
            let all_free = self.nodes[first_child..first_child + 4]
                .iter()
                .all(|&state| state == NodeState::Free);
            if !all_free {
                break;
            }
            for child in &mut self.nodes[first_child..first_child + 4] {
                *child = NodeState::Inactive;
            }
            self.free_at_depth[cur_depth] -= 4;
            self.nodes[parent] = NodeState::Free;
            self.free_at_depth[cur_depth - 1] += 1;
            cur = parent;
            cur_depth -= 1;
        }

        true
    }

    /// Resets every node to free while keeping dimensions and `reserved_texels`.
    pub fn clear(&mut self) {
        self.nodes.fill(NodeState::Inactive);
        self.nodes[0] = NodeState::Free;
        self.free_at_depth.fill(0);
        self.free_at_depth[0] = 1;
        self.used_texels = 0;
        self.alloc_count = 0;
    }

    /// Square side length of the atlas in texels.
    pub fn atlas_size(&self) -> u16 {
        self.atlas_size
    }

    /// Minimum allocatable tile side length in texels.
    pub fn min_tile(&self) -> u16 {
        self.min_tile
    }

    /// Maximum allocatable tile side length in texels.
    pub fn max_tile(&self) -> u16 {
        self.max_tile
    }

    /// Total area of the atlas (`atlas_size²`).
    pub fn total_texels(&self) -> u64 {
        self.total_texels
    }

    /// Currently allocated area in texels.
    pub fn used_texels(&self) -> u64 {
        self.used_texels
    }

    /// Currently unallocated area in texels (`total_texels - used_texels`).
    pub fn free_texels(&self) -> u64 {
        self.total_texels - self.used_texels
    }

    /// Number of texels reserved for handover allocations.
    pub fn reserved_texels(&self) -> u64 {
        self.reserved_texels
    }

    /// Number of currently active tile allocations.
    pub fn alloc_count(&self) -> usize {
        self.alloc_count
    }

    /// Fraction of the atlas currently allocated (`0.0..=1.0`).
    #[allow(clippy::cast_precision_loss)]
    pub fn occupancy(&self) -> f32 {
        self.used_texels as f32 / self.total_texels as f32
    }

    fn alloc_inner(&mut self, tile_size: u16, allow_reserve: bool) -> Option<AtlasRect> {
        if !self.is_valid_tile_size(tile_size) {
            return None;
        }

        let tile_texels = u64::from(tile_size) * u64::from(tile_size);
        let remaining = self.free_texels().checked_sub(tile_texels)?;
        if !allow_reserve && remaining < self.reserved_texels {
            return None;
        }

        let target_depth = self.depth_for_tile(tile_size);

        // Search from deepest (exact-size free slot) up to root so we always prefer an
        // already-split branch over splitting a larger free block.
        let mut found: Option<(usize, usize)> = None;
        for depth in (0..=target_depth).rev() {
            if self.free_at_depth[depth] == 0 {
                continue;
            }
            let start = level_start(depth);
            let len = level_len(depth);
            if let Some(rel) = self.nodes[start..start + len]
                .iter()
                .position(|&state| state == NodeState::Free)
            {
                found = Some((depth, start + rel));
                break;
            }
        }

        let (mut depth, mut idx) = found?;
        while depth < target_depth {
            self.nodes[idx] = NodeState::Split;
            self.free_at_depth[depth] -= 1;
            let first_child = 4 * idx + 1;
            for child in &mut self.nodes[first_child..first_child + 4] {
                *child = NodeState::Free;
            }
            self.free_at_depth[depth + 1] += 4;
            idx = first_child;
            depth += 1;
        }

        self.nodes[idx] = NodeState::Used;
        self.free_at_depth[target_depth] -= 1;
        self.used_texels += tile_texels;
        self.alloc_count += 1;

        let (x, y) = self.index_to_coords(idx, tile_size);
        Some(AtlasRect {
            x,
            y,
            size: tile_size,
        })
    }

    fn is_valid_tile_size(&self, tile_size: u16) -> bool {
        tile_size.is_power_of_two() && tile_size >= self.min_tile && tile_size <= self.max_tile
    }

    fn depth_for_tile(&self, tile_size: u16) -> usize {
        let depth = (self.atlas_size / tile_size).trailing_zeros() as usize;
        debug_assert!(depth <= self.max_depth);
        depth
    }

    fn coords_to_index(&self, x: u16, y: u16, depth: usize) -> usize {
        let mut idx = 0usize;
        let mut bx = 0u16;
        let mut by = 0u16;
        let mut size = self.atlas_size;
        for _ in 0..depth {
            let half = size >> 1;
            let qx = usize::from(x >= bx + half);
            let qy = usize::from(y >= by + half);
            let quadrant = (qy << 1) | qx;
            idx = 4 * idx + 1 + quadrant;
            if qx == 1 {
                bx += half;
            }
            if qy == 1 {
                by += half;
            }
            size = half;
        }
        idx
    }

    fn index_to_coords(&self, idx: usize, tile_size: u16) -> (u16, u16) {
        let mut cur = idx;
        let mut size = tile_size;
        let mut x = 0u16;
        let mut y = 0u16;
        while cur > 0 {
            let rel = cur - 1;
            let quadrant = rel % 4;
            if (quadrant & 1) != 0 {
                x += size;
            }
            if (quadrant & 2) != 0 {
                y += size;
            }
            cur = rel / 4;
            size <<= 1;
        }
        debug_assert_eq!(size, self.atlas_size);
        (x, y)
    }
}

const fn level_len(depth: usize) -> usize {
    1usize << (2 * depth)
}

const fn level_start(depth: usize) -> usize {
    ((1usize << (2 * depth)) - 1) / 3
}
