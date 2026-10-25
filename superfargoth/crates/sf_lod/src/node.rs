//! Node keys of the far-field tree over the streaming grid.
//!
//! L1 nodes are streaming cells; an L2 node is 8×8×8 cells (one index page,
//! 1 km with 128 m cells); an L3 node is 4×4×4 L2 nodes (4 km). Parents are
//! integer shifts of the cell index, so the tree needs no stored links.

use std::fmt;

use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::{WorldPos, partition_extent, partition_origin};
use sf_world::Partition;

/// The finest proxy level (L0 is the objects themselves).
pub const FIRST_LEVEL: u8 = 1;
/// The coarsest level.
pub const LAST_LEVEL: u8 = 3;

/// Cell-index shift of each level relative to the streaming cells.
pub const fn level_shift(level: u8) -> i32 {
    match level {
        0 | 1 => 0,
        2 => 3,
        _ => 5,
    }
}

/// A node of the far-field tree: a level and an index on that level's grid.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeKey {
    pub level: u8,
    pub index: [i32; 3],
}

impl NodeKey {
    pub fn new(level: u8, index: IVec3) -> Self {
        Self { level, index: index.to_array() }
    }

    /// The L1 node of a streaming cell.
    pub fn of_cell(cell: IVec3) -> Self {
        Self::new(FIRST_LEVEL, cell)
    }

    /// The node of `level` containing a streaming cell.
    pub fn containing(cell: IVec3, level: u8) -> Self {
        let s = level_shift(level);
        Self::new(level, IVec3::new(cell.x >> s, cell.y >> s, cell.z >> s))
    }

    pub fn index(&self) -> IVec3 {
        IVec3::from_array(self.index)
    }

    pub fn parent(&self) -> Option<Self> {
        (self.level < LAST_LEVEL).then(|| {
            let s = level_shift(self.level + 1) - level_shift(self.level);
            let i = self.index();
            Self::new(self.level + 1, IVec3::new(i.x >> s, i.y >> s, i.z >> s))
        })
    }

    /// Whether `cell` (a streaming cell) lies in this node.
    pub fn contains_cell(&self, cell: IVec3) -> bool {
        Self::containing(cell, self.level) == *self
    }

    /// Partition shift of this node's grid, given the streaming partition.
    pub fn shift(&self, partition: Partition) -> i32 {
        streaming_shift(partition) + level_shift(self.level)
    }

    /// Edge length in meters.
    pub fn extent(&self, partition: Partition) -> f32 {
        partition_extent(self.shift(partition))
    }

    /// The minimum corner.
    pub fn origin(&self, partition: Partition) -> WorldPos {
        partition_origin(self.index(), self.shift(partition))
    }

    /// The center of the node's box (its entity origin, from which
    /// handoff distances are measured).
    pub fn center(&self, partition: Partition) -> WorldPos {
        let half = self.extent(partition) * 0.5;
        self.origin(partition).offset(Vec3::splat(half))
    }

    /// Bake subject tail: `x,y,z`.
    pub fn subject_tail(&self) -> String {
        format!("{},{},{}", self.index[0], self.index[1], self.index[2])
    }

    pub fn parse_tail(level: u8, tail: &str) -> Option<Self> {
        let mut it = tail.split(',').map(str::parse::<i32>);
        let index = IVec3::new(it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?);
        it.next().is_none().then_some(Self::new(level, index))
    }
}

impl fmt::Debug for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}({},{},{})", self.level, self.index[0], self.index[1], self.index[2])
    }
}

/// The grid shift of streaming cells (a `Single` partition has one cell:
/// treated as one 128 m-grid cell at the origin).
pub fn streaming_shift(partition: Partition) -> i32 {
    match partition {
        Partition::Grid { shift } => shift,
        Partition::Single => -1,
    }
}
