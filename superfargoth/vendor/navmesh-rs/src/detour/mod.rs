//! Navmesh runtime and queries (upstream Detour).
//!
//! - [`create_nav_mesh_data`] turns a Recast poly mesh and detail mesh (plus off-mesh
//!   connections) into tile bytes in upstream's `DNAV` v7 layout.
//! - [`NavMesh`] owns tiles: it validates tile bytes before adding them (upstream
//!   trusts them), links polygons inside and across tiles and hands out [`PolyRef`]s
//!   with upstream's salt/tile/poly bit layout.
//! - [`NavMeshQuery`] runs every upstream query over a borrowed navmesh, with a
//!   [`QueryFilter`] and its own node pools.
//!
//! Poly refs are 64-bit with the default `polyref64` feature (upstream's
//! `DT_POLYREF64`), 32-bit without it. The width changes the link size in tile
//! bytes, so 32-bit and 64-bit tiles are distinct formats.
//!
//! Upstream's `dtStatus` maps to `Result`: failures become [`Error`]
//! ([`status_of`] gives the upstream code back), and successes that carry details
//! (partial result, out of nodes, buffer too small) return a [`Status`] next to
//! their output.

// The port keeps upstream's control flow and comparisons so it can be reviewed
// line by line against the C++; these lints would rewrite exactly that (and
// `clamp` differs from upstream's comparisons on NaN).
#![allow(
    clippy::collapsible_if,
    clippy::needless_range_loop,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::manual_clamp,
    clippy::needless_late_init,
    clippy::chunks_exact_to_as_chunks,
    // `PolyRef` is u32 or u64 by feature; its casts are no-ops at one width.
    clippy::unnecessary_cast
)]

mod builder;
mod endian;
pub(crate) mod math;
mod navmesh;
mod node;
pub(crate) mod query;
mod query_local;
mod resumable;
mod tile;

pub use builder::{NavMeshCreateParams, create_nav_mesh_data, create_nav_mesh_data_f32};
pub use endian::{
    nav_mesh_data_swap_endian, nav_mesh_header_swap_endian, nav_mesh_swap_off_mesh_user_ids,
};
pub use navmesh::{
    Committed, EdgeKind, NavMesh, NavMeshParams, PreparedTile, REMOVAL_LOG_CAPACITY, RefState,
    ResidencyRecord, SlotPolicy, TileKey, UnloadedTile, content_hash, prepare_tile,
};
// superfargoth patch (own: public detached query): `DetachedQuery` exported.
pub use query::{
    DefaultFilter, DetachedQuery, NavMeshQuery, QueryFilter, RaycastHit, SegmentWall,
    StraightPathPoint,
};
pub use resumable::{
    DistanceToWallSearch, LocalNeighbourhoodSearch, PolysAroundSearch, RaycastSearch,
    SurfaceMoveSearch,
};
pub use tile::{BvNode, Link, MeshHeader, MeshTile, OffMeshConnection, Poly, PolyDetail};

use crate::{Error, TileDataError, TileStateError};

/// A polygon reference: salt, tile index and polygon index packed as upstream does.
#[cfg(feature = "polyref64")]
pub type PolyRef = u64;
/// A polygon reference: salt, tile index and polygon index packed as upstream does.
#[cfg(not(feature = "polyref64"))]
pub type PolyRef = u32;
/// A tile reference (a poly ref with polygon index 0).
pub type TileRef = PolyRef;

/// Width of [`PolyRef`] in bits.
pub const POLYREF_BITS: u32 = PolyRef::BITS;

/// Maximum vertices per polygon.
pub const VERTS_PER_POLYGON: usize = 6;
/// Tile data magic (`'DNAV'`).
pub const NAVMESH_MAGIC: i32 =
    (b'D' as i32) << 24 | (b'N' as i32) << 16 | (b'A' as i32) << 8 | b'V' as i32;
/// Tile data version.
pub const NAVMESH_VERSION: i32 = 7;
/// Tile state magic (`'DNMS'`).
pub const NAVMESH_STATE_MAGIC: i32 =
    (b'D' as i32) << 24 | (b'N' as i32) << 16 | (b'M' as i32) << 8 | b'S' as i32;
/// Tile state version.
pub const NAVMESH_STATE_VERSION: i32 = 1;
/// Neighbour flag for an edge on a tile border.
pub const EXT_LINK: u16 = 0x8000;
/// End of a link list.
pub const NULL_LINK: u32 = 0xffff_ffff;
/// Off-mesh connection flag: traversable both ways.
pub const OFFMESH_CON_BIDIR: u8 = 1;
/// Number of area ids.
pub const MAX_AREAS: usize = 64;
/// Ground polygon type.
pub const POLYTYPE_GROUND: u8 = 0;
/// Off-mesh connection polygon type.
pub const POLYTYPE_OFFMESH_CONNECTION: u8 = 1;
/// Straight path vertex flag: the start point.
pub const STRAIGHTPATH_START: u8 = 0x01;
/// Straight path vertex flag: the end point.
pub const STRAIGHTPATH_END: u8 = 0x02;
/// Straight path vertex flag: the start of an off-mesh connection.
pub const STRAIGHTPATH_OFFMESH_CONNECTION: u8 = 0x04;
/// Straight path option: add a vertex at every area change.
pub const STRAIGHTPATH_AREA_CROSSINGS: i32 = 0x01;
/// Straight path option: add a vertex at every polygon edge crossing.
pub const STRAIGHTPATH_ALL_CROSSINGS: i32 = 0x02;
/// Find path option: shortcut with raycasts (sliced search only).
pub const FINDPATH_ANY_ANGLE: u32 = 0x02;
/// Raycast option: compute the path cost.
pub const RAYCAST_USE_COSTS: u32 = 0x01;
/// Detail triangle edge flag: the edge lies on the polygon boundary.
pub const DETAIL_EDGE_BOUNDARY: i32 = 0x01;
/// Flags of one edge of a detail triangle (`dtGetDetailTriEdgeFlags`): the two bits
/// for the edge that starts at vertex `edge_index` (0 for AB, 1 for BC, 2 for CA)
/// of the triangle whose fourth byte is `tri_flags`. Bit 0 of the result is
/// [`DETAIL_EDGE_BOUNDARY`]. An `edge_index` of 4 or more gives 0 (upstream
/// computes the same for 4 to 15 and shifts out of range beyond).
pub const fn detail_tri_edge_flags(tri_flags: u8, edge_index: usize) -> u8 {
    if edge_index >= 4 {
        return 0;
    }
    (tri_flags >> (edge_index * 2)) & 0x3
}
/// Any-angle raycast length limit, as a multiple of the agent radius.
pub const RAY_CAST_LIMIT_PROPORTIONS: f32 = 50.0;

/// Upstream's `dtStatus` bits: success or in-progress plus detail flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Status(pub u32);

impl Status {
    /// Operation failed.
    pub const FAILURE: u32 = 1 << 31;
    /// Operation succeeded.
    pub const SUCCESS: u32 = 1 << 30;
    /// Operation still in progress.
    pub const IN_PROGRESS: u32 = 1 << 29;
    /// Input data is not recognized.
    pub const WRONG_MAGIC: u32 = 1 << 0;
    /// Input data is the wrong version.
    pub const WRONG_VERSION: u32 = 1 << 1;
    /// Out of memory or out of tile slots.
    pub const OUT_OF_MEMORY: u32 = 1 << 2;
    /// An input parameter was invalid.
    pub const INVALID_PARAM: u32 = 1 << 3;
    /// The result buffer was too small for the whole result.
    pub const BUFFER_TOO_SMALL: u32 = 1 << 4;
    /// The query ran out of nodes during the search.
    pub const OUT_OF_NODES: u32 = 1 << 5;
    /// The query did not reach the end location; the result is the best guess.
    pub const PARTIAL_RESULT: u32 = 1 << 6;
    /// A tile already occupies the location.
    pub const ALREADY_OCCUPIED: u32 = 1 << 7;
    /// The goal was not reached and the search touched an edge facing a known,
    /// unloaded tile column (not in recastnavigation).
    pub const UNLOADED_BORDER: u32 = 1 << 8;
    /// A search was continued after a tile it had touched was unloaded or removed
    /// (not in recastnavigation).
    pub const MESH_CHANGED: u32 = 1 << 9;

    /// Plain success.
    pub const OK: Status = Status(Self::SUCCESS);

    /// Whether the success bit is set.
    pub fn is_success(self) -> bool {
        self.0 & Self::SUCCESS != 0
    }
    /// Whether the in-progress bit is set.
    pub fn is_in_progress(self) -> bool {
        self.0 & Self::IN_PROGRESS != 0
    }
    /// Whether the failure bit is set.
    pub fn is_failure(self) -> bool {
        self.0 & Self::FAILURE != 0
    }
    /// Whether a detail bit is set.
    pub fn has_detail(self, detail: u32) -> bool {
        self.0 & detail != 0
    }
}

/// The upstream `dtStatus` for an error returned by this module.
pub fn status_of(e: &Error) -> Status {
    Status(
        Status::FAILURE
            | match e {
                Error::Failed => 0,
                Error::InvalidTileData(TileDataError::WrongMagic) => Status::WRONG_MAGIC,
                Error::InvalidTileData(TileDataError::WrongVersion) => Status::WRONG_VERSION,
                Error::InvalidTileState(TileStateError::WrongMagic) => Status::WRONG_MAGIC,
                Error::InvalidTileState(TileStateError::WrongVersion) => Status::WRONG_VERSION,
                Error::TileSlotOccupied => Status::ALREADY_OCCUPIED,
                Error::NoFreeTile | Error::AllocationFailed | Error::LimitExceeded => {
                    Status::OUT_OF_MEMORY
                }
                Error::BufferTooSmall => Status::BUFFER_TOO_SMALL,
                Error::MeshChanged => Status::MESH_CHANGED,
                _ => Status::INVALID_PARAM,
            },
    )
}

/// The raw `dtStatus` of a result: its success status or its error's code.
pub fn raw_status<T>(r: &Result<T, Error>, ok: impl FnOnce(&T) -> Status) -> u32 {
    match r {
        Ok(v) => ok(v).0,
        Err(e) => status_of(e).0,
    }
}

#[cfg(feature = "recast")]
pub(crate) use tile::check_bake_tile_limits;

#[cfg(feature = "recast")]
pub(crate) use builder::create_with_limits;
