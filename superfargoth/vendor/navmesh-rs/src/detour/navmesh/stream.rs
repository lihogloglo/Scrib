//! Tile streaming (not in recastnavigation): keyed slots, tiles prepared off the
//! owner thread, unloading that keeps refs, ref states, a structural epoch and a
//! residency record.
//!
//! # Salts and content
//!
//! A slot's salt is part of every ref into it. Here it changes only when the
//! slot's content changes: [`NavMesh::unload_tile`] keeps the salt and reserves the
//! slot for the unloaded tile, recording the tile's [`content_hash`]. Committing
//! that tile again with the same hash keeps the salt, so every ref into the
//! unloaded copy is valid again and names the same polygon; any other content for
//! that tile increments the salt. [`NavMesh::remove_tile`] increments it as before.
//!
//! Salts only grow (exhaustion is refused, see [`NavMesh`]), and a ref whose salt is
//! older than its slot's is [`RefState::Stale`] for good: no sequence of commits,
//! unloads, removals or residency restores makes it valid again.
//! [`NavMesh::restore_tile_allow_stale`] is the one exception, by its contract.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ops::Range;

use super::{NavMesh, NavMeshParams, Placement, SlotTable, layout_bits};
use crate::detour::math::{ilog2, next_pow2};
use crate::detour::tile::{MeshTile, peek_header};
use crate::detour::{EXT_LINK, NAVMESH_STATE_MAGIC, NAVMESH_STATE_VERSION, PolyRef, TileRef};
use crate::{Budget, Error, Limits, TileDataError, TileStateError};

/// Removals the epoch log keeps (see [`NavMesh::epoch`]).
pub const REMOVAL_LOG_CAPACITY: usize = 256;

const STREAM_STATE_FOOTER: usize = 40;
const STREAM_STATE_MAGIC: [u8; 4] = *b"NVS1";

/// How a navmesh assigns tile slots (and so the tile field of refs).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SlotPolicy {
    /// Upstream's free list: a tile takes the next free slot. Refs depend on the
    /// order tiles were added in.
    #[default]
    Sequential,
    /// Tile `(x, y, layer)` always takes slot
    /// `((y - origin[1]) * width + (x - origin[0])) * max_layers + layer`, so its
    /// refs do not depend on load order or on the session. Tiles outside the grid
    /// are refused with [`Error::TileOutsideGrid`]. Pages of 1024 slots are
    /// allocated as tiles reach them; a slot that ever held a tile keeps its page
    /// (its salt must survive).
    Keyed {
        /// Tile coordinates of the grid's first column.
        origin: [i32; 2],
        /// Columns along x.
        width: u32,
        /// Columns along y (z in world space).
        height: u32,
        /// Layers per column.
        max_layers: u32,
    },
}

impl SlotPolicy {
    pub(super) fn validate(&self, params: &NavMeshParams) -> Result<(), Error> {
        if let SlotPolicy::Keyed {
            origin,
            width,
            height,
            max_layers,
        } = *self
        {
            if width == 0 || height == 0 || max_layers == 0 {
                return Err(Error::InvalidParameter("keyed grid size"));
            }
            let n = u64::from(width)
                .checked_mul(u64::from(height))
                .and_then(|n| n.checked_mul(u64::from(max_layers)));
            if n.is_none_or(|n| n > params.max_tiles as u64) {
                return Err(Error::InvalidParameter(
                    "keyed grid needs more slots than max_tiles",
                ));
            }
            if i64::from(origin[0]) + i64::from(width) > i64::from(i32::MAX) + 1
                || i64::from(origin[1]) + i64::from(height) > i64::from(i32::MAX) + 1
            {
                return Err(Error::InvalidParameter("keyed grid beyond the i32 range"));
            }
        }
        Ok(())
    }
}

/// A tile column: tile coordinates `x` and `y` (all layers).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TileKey {
    /// Tile x.
    pub x: i32,
    /// Tile y (z in world space).
    pub y: i32,
}

/// What a ref names now (see [`NavMesh::ref_state`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RefState {
    /// A polygon of a resident tile.
    Valid,
    /// A polygon of an unloaded tile in this column; it becomes `Valid` again when
    /// the same content is committed.
    Unloaded(TileKey),
    /// The ref was issued for content that has since been removed or replaced. It
    /// never becomes valid again.
    Stale,
    /// Not a ref this navmesh issued (null, bits outside the layout, a slot or
    /// salt never used, a polygon index past the tile).
    Invalid,
}

/// Why a raycast stopped at an edge (see [`RaycastHit::edge`](crate::detour::RaycastHit::edge)).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum EdgeKind {
    /// The ray reached its end (no edge was hit).
    #[default]
    None,
    /// A wall, or a tile side facing a resident tile with no link across.
    Wall,
    /// A tile side facing a column that is known but not (fully) resident.
    UnloadedBorder(TileKey),
    /// A tile side facing a column with no tile and not known.
    WorldEdge,
}

/// Outcome of [`NavMesh::commit_budgeted`].
#[derive(Debug)]
#[non_exhaustive]
pub enum Committed {
    /// The tile is resident.
    Done(TileRef),
    /// The budget could not cover the commit; nothing changed and the tile is
    /// handed back for a later frame.
    Refused(Box<PreparedTile>),
}

/// The reservation of a slot whose tile is unloaded.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Parked {
    pub key: TileKey,
    pub layer: i32,
    pub hash: [u8; 32],
    pub polys: u32,
}

/// A tile parsed and validated by [`prepare_tile`], ready for [`NavMesh::commit`].
#[derive(Clone, Debug)]
pub struct PreparedTile {
    tile: MeshTile,
    hash: [u8; 32],
    poly_bits: u32,
}

impl PreparedTile {
    /// The tile's column.
    pub fn key(&self) -> TileKey {
        TileKey {
            x: self.tile.header.x,
            y: self.tile.header.y,
        }
    }

    /// The tile's layer.
    pub fn layer(&self) -> i32 {
        self.tile.header.layer
    }

    /// [`content_hash`] of the tile bytes.
    pub fn content_hash(&self) -> [u8; 32] {
        self.hash
    }

    /// The parsed tile. Internal link refs are polygon offsets until commit
    /// assigns the slot and salt.
    pub fn tile(&self) -> &MeshTile {
        &self.tile
    }

    /// Bytes the tile holds once resident (tile bytes plus parsed arrays).
    pub fn resident_bytes(&self) -> usize {
        let t = &self.tile;
        use core::mem::size_of;
        size_of::<MeshTile>()
            + t.data.len()
            + t.verts.len() * 4
            + t.polys.len() * size_of::<crate::detour::Poly>()
            + t.links.len() * size_of::<crate::detour::Link>()
            + t.detail_meshes.len() * size_of::<crate::detour::PolyDetail>()
            + t.detail_verts.len() * 4
            + t.detail_tris.len()
            + t.bv_tree.len() * size_of::<crate::detour::BvNode>()
            + t.off_mesh_cons.len() * size_of::<crate::detour::OffMeshConnection>()
    }
}

/// An evicted tile (see [`NavMesh::unload_tile`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnloadedTile {
    /// The tile's column.
    pub key: TileKey,
    /// The tile's layer.
    pub layer: i32,
    /// The bytes the tile was committed from.
    pub bytes: Vec<u8>,
    /// Runtime flags and areas for [`NavMesh::commit_with_state`]: the
    /// [`NavMesh::store_tile_state`] prefix plus a versioned content-hash footer.
    pub state: Vec<u8>,
    /// [`content_hash`] of `bytes`.
    pub content_hash: [u8; 32],
}

/// SHA-256 digest of all input tile bytes, including accepted trailing bytes.
/// Platform independent and collision resistant. Uses the pure-Rust `sha2` crate
/// with default features disabled (MIT OR Apache-2.0); no standard library needed.
pub fn content_hash(bytes: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes).into()
}

/// Parses and validates tile bytes for a navmesh with `params`, on any thread
/// (the expensive, untrusted-bytes part of [`NavMesh::add_tile`]). It accepts and
/// rejects exactly the bytes `add_tile` accepts and rejects on an empty navmesh
/// with the same parameters and limits; slot checks (occupied location, free
/// slot, keyed grid) happen at [`NavMesh::commit`].
pub fn prepare_tile(
    params: &NavMeshParams,
    limits: Limits,
    bytes: Vec<u8>,
) -> Result<PreparedTile, Error> {
    let (_, _, poly_bits) = layout_bits(params)?;
    let h = peek_header(&bytes)?;
    if PolyRef::BITS == 32
        && (h.poly_count < 0 || poly_bits < ilog2(next_pow2(h.poly_count as u32)))
    {
        return Err(Error::InvalidTileData(TileDataError::TooManyPolys));
    }
    if h.poly_count < 0 || h.poly_count as u64 > (1u64 << poly_bits) {
        return Err(Error::InvalidTileData(TileDataError::TooManyPolys));
    }
    let mut tile = MeshTile::parse_with_limits(bytes, &limits)?;
    tile.links_free_list = tile.reset_links();
    NavMesh::prepare_internal_links(&mut tile, 0);
    let hash = content_hash(&tile.data);
    Ok(PreparedTile {
        tile,
        hash,
        poly_bits,
    })
}

/// Checks a tile state blob against the ref and polygon count it will be applied
/// to; returns the state's size.
pub(super) fn check_state(tile: TileRef, polys: usize, data: &[u8]) -> Result<usize, Error> {
    let size = polys
        .checked_mul(super::TILE_POLY_STATE_SIZE)
        .and_then(|n| n.checked_add(super::TILE_STATE_HEADER_SIZE))
        .ok_or(Error::SizeOverflow)?;
    if data.len() < size {
        return Err(Error::InvalidTileState(TileStateError::TooShort));
    }
    let le_i32 =
        |at: usize| i32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]]);
    if le_i32(0) != NAVMESH_STATE_MAGIC {
        return Err(Error::InvalidTileState(TileStateError::WrongMagic));
    }
    if le_i32(4) != NAVMESH_STATE_VERSION {
        return Err(Error::InvalidTileState(TileStateError::WrongVersion));
    }
    let mut stored = [0u8; (PolyRef::BITS / 8) as usize];
    stored.copy_from_slice(&data[8..super::TILE_STATE_HEADER_SIZE]);
    if PolyRef::from_le_bytes(stored) != tile {
        return Err(Error::InvalidTileState(TileStateError::WrongTile));
    }
    Ok(size)
}

/// Writes a checked state blob into a tile's polygons.
pub(super) fn apply_state(t: &mut MeshTile, data: &[u8], size: usize) {
    for (p, s) in t
        .polys
        .iter_mut()
        .zip(data[super::TILE_STATE_HEADER_SIZE..size].chunks_exact(super::TILE_POLY_STATE_SIZE))
    {
        p.flags = u16::from_le_bytes([s[0], s[1]]);
        p.set_area(s[2]);
    }
}

/// The streaming part of a navmesh's state.
#[derive(Clone, Debug)]
pub(crate) struct StreamState {
    policy: SlotPolicy,
    epoch: u64,
    /// Ring of `(epoch, slot)` for unloads and removals.
    log: Vec<(u64, u32)>,
    log_head: usize,
    /// Highest epoch of an entry the ring overwrote.
    lost_through: u64,
    /// Declared layers per known column ([`NavMesh::mark_known`]).
    known: BTreeMap<(i32, i32), u8>,
    /// Reserved slot per unloaded tile `(x, y, layer)` (sequential policy only;
    /// keyed slots are computed).
    parked: BTreeMap<(i32, i32, i32), u32>,
    parked_count: usize,
}

impl StreamState {
    pub(super) fn new(policy: SlotPolicy) -> Result<Self, Error> {
        let mut log = Vec::new();
        log.try_reserve_exact(REMOVAL_LOG_CAPACITY)
            .map_err(|_| Error::AllocationFailed)?;
        Ok(StreamState {
            policy,
            epoch: 0,
            log,
            log_head: 0,
            lost_through: 0,
            known: BTreeMap::new(),
            parked: BTreeMap::new(),
            parked_count: 0,
        })
    }

    pub(super) fn is_keyed(&self) -> bool {
        matches!(self.policy, SlotPolicy::Keyed { .. })
    }

    /// For a keyed navmesh, the slot of `(x, y, layer)` (`Some(None)` outside the
    /// grid); `None` for a sequential one.
    pub(super) fn keyed_slot(&self, x: i32, y: i32, layer: i32) -> Option<Option<usize>> {
        let SlotPolicy::Keyed { max_layers, .. } = self.policy else {
            return None;
        };
        Some(
            self.column_base(x, y).and_then(|b| {
                (layer >= 0 && (layer as u32) < max_layers).then(|| b + layer as usize)
            }),
        )
    }

    fn column_base(&self, x: i32, y: i32) -> Option<usize> {
        let SlotPolicy::Keyed {
            origin,
            width,
            height,
            max_layers,
        } = self.policy
        else {
            return None;
        };
        let dx = i64::from(x) - i64::from(origin[0]);
        let dy = i64::from(y) - i64::from(origin[1]);
        if dx < 0 || dy < 0 || dx >= i64::from(width) || dy >= i64::from(height) {
            return None;
        }
        usize::try_from((dy * i64::from(width) + dx) * i64::from(max_layers)).ok()
    }

    /// For a keyed navmesh, the slots of column `(x, y)` (empty outside the grid).
    pub(super) fn column_slots(&self, x: i32, y: i32) -> Option<Range<usize>> {
        let SlotPolicy::Keyed { max_layers, .. } = self.policy else {
            return None;
        };
        Some(match self.column_base(x, y) {
            Some(b) => b..b + max_layers as usize,
            None => 0..0,
        })
    }

    pub(super) fn parked_slot(
        &self,
        slots: &SlotTable,
        x: i32,
        y: i32,
        layer: i32,
    ) -> Option<usize> {
        match self.keyed_slot(x, y, layer) {
            Some(k) => k.filter(|&i| slots.get(i).is_some_and(|s| s.parked.is_some())),
            None => self.parked.get(&(x, y, layer)).map(|&i| i as usize),
        }
    }

    fn park(&mut self, x: i32, y: i32, layer: i32, slot: usize) {
        if !self.is_keyed() {
            self.parked.insert((x, y, layer), slot as u32);
        }
        self.parked_count += 1;
    }

    pub(super) fn unpark(&mut self, x: i32, y: i32, layer: i32) {
        if !self.is_keyed() {
            self.parked.remove(&(x, y, layer));
        }
        self.parked_count = self.parked_count.saturating_sub(1);
    }

    pub(super) fn bump_epoch(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
    }

    /// Records that `slot` lost its tile (unload, removal, forgotten reservation).
    pub(super) fn log_removal(&mut self, slot: u32) {
        self.bump_epoch();
        let e = (self.epoch, slot);
        if self.log.len() < REMOVAL_LOG_CAPACITY {
            self.log.push(e);
        } else {
            let old = core::mem::replace(&mut self.log[self.log_head], e);
            self.lost_through = self.lost_through.max(old.0);
            self.log_head = (self.log_head + 1) % REMOVAL_LOG_CAPACITY;
        }
    }
}

impl NavMesh {
    /// The slot policy.
    pub fn slot_policy(&self) -> SlotPolicy {
        self.stream.policy
    }

    /// Structural change counter: incremented by every commit, add, restore,
    /// unload, removal and forgotten reservation.
    pub fn epoch(&self) -> u64 {
        self.stream.epoch
    }

    /// Whether a slot for which `touched` returns true lost its tile after epoch
    /// `since`. Also true when the removal log no longer reaches back to `since`
    /// (more than [`REMOVAL_LOG_CAPACITY`] removals since), so callers restart.
    pub fn changed_since(&self, since: u64, touched: &mut dyn FnMut(u32) -> bool) -> bool {
        if self.stream.epoch == since {
            return false;
        }
        if self.stream.lost_through > since {
            return true;
        }
        self.stream
            .log
            .iter()
            .any(|&(e, slot)| e > since && touched(slot))
    }

    /// Whether any column is known but not fully resident (unloaded tiles or
    /// [`NavMesh::mark_known`] declarations exist).
    pub fn has_unloaded(&self) -> bool {
        self.stream.parked_count > 0 || !self.stream.known.is_empty()
    }

    /// Declares that column `key` has `layers` tiles on the caller's side (0
    /// forgets the declaration). A column with fewer resident tiles than declared,
    /// or with an unloaded tile, is reported as an unloaded border by queries.
    /// Declarations do not change refs or the epoch.
    pub fn mark_known(&mut self, key: TileKey, layers: u8) {
        if layers == 0 {
            self.stream.known.remove(&(key.x, key.y));
        } else {
            self.stream.known.insert((key.x, key.y), layers);
        }
    }

    /// Whether column `(x, y)` is known but not fully resident.
    pub fn is_column_unloaded(&self, x: i32, y: i32) -> bool {
        let parked = match self.stream.column_slots(x, y) {
            Some(r) => r.into_iter().any(|i| self.slots[i].parked.is_some()),
            None => self
                .stream
                .parked
                .range((x, y, i32::MIN)..=(x, y, i32::MAX))
                .next()
                .is_some(),
        };
        if parked {
            return true;
        }
        match self.stream.known.get(&(x, y)) {
            Some(&want) => self.tiles_at(x, y, usize::from(want)).len() < want as usize,
            None => false,
        }
    }

    /// The column holding world position `pos` when it is known but not fully
    /// resident: the diagnostic for a
    /// [`find_nearest_poly`](crate::detour::NavMeshQuery::find_nearest_poly) that
    /// found nothing because the tile there is not loaded.
    pub fn unloaded_column_at(&self, pos: &[f32; 3]) -> Option<TileKey> {
        let (x, y) = self.calc_tile_loc(pos);
        self.is_column_unloaded(x, y).then_some(TileKey { x, y })
    }

    /// What `r` names now. Exact under the rules in the module docs: `Valid` iff
    /// its slot holds a resident tile with the ref's salt and the polygon exists;
    /// `Unloaded` iff the slot is reserved for an unloaded tile with that salt;
    /// `Stale` iff the slot was used and the ref's salt is older than the slot's;
    /// otherwise `Invalid`.
    pub fn ref_state(&self, r: PolyRef) -> RefState {
        if r == 0 || !self.ref_in_layout(r) {
            return RefState::Invalid;
        }
        let (salt, it, ip) = self.decode_poly_id(r);
        let Some(s) = self.slots.get(it as usize) else {
            return RefState::Invalid;
        };
        if !s.used || salt == 0 {
            return RefState::Invalid;
        }
        if salt < s.salt {
            return RefState::Stale;
        }
        if salt != s.salt {
            return RefState::Invalid;
        }
        if let Some(t) = s.tile.as_deref() {
            if (ip as usize) < t.polys.len() {
                return RefState::Valid;
            }
        } else if let Some(p) = s.parked {
            if ip < p.polys {
                return RefState::Unloaded(p.key);
            }
        }
        RefState::Invalid
    }

    /// Work units [`NavMesh::commit_budgeted`] charges for `t`: its polygons, the
    /// polygons of the resident tiles in its column and the eight around it, and
    /// one per resident tile plus a full connection/candidate relink allowance
    /// when global off-mesh links
    /// are on (all connection counts times all polygon, link and tile counts).
    pub fn commit_cost(&self, t: &PreparedTile) -> u64 {
        let (x, y) = (t.tile.header.x, t.tile.header.y);
        let mut cost = 1 + t.tile.polys.len() as u64;
        let mut add = |j: usize| cost += self.tile(j).map_or(0, |t| t.polys.len() as u64);
        for j in self.tiles_at(x, y, 32) {
            add(j);
        }
        for side in 0..8 {
            for j in self.neighbour_tiles_at(x, y, side) {
                add(j);
            }
        }
        if self.global_off_mesh {
            cost = cost.saturating_add(self.resident as u64);
            let mut connections = t.tile.off_mesh_cons.len() as u64;
            let mut candidates = 1 + t.tile.polys.len() as u64 + t.tile.links.len() as u64;
            for i in self.resident_slots() {
                if let Some(tile) = self.tile(i) {
                    connections = connections.saturating_add(tile.off_mesh_cons.len() as u64);
                    candidates = candidates
                        .saturating_add(1 + tile.polys.len() as u64 + tile.links.len() as u64);
                }
            }
            cost = cost.saturating_add(connections.saturating_mul(candidates));
        }
        cost
    }

    /// Owner-thread half of [`NavMesh::add_tile`] for a tile from [`prepare_tile`]:
    /// slot choice, salt rule, internal, external and global off-mesh links. A tile
    /// whose column and layer were unloaded goes back into its reserved slot; with
    /// the same content hash its old refs are valid again. Errors:
    /// [`Error::TileSlotOccupied`], [`Error::NoFreeTile`],
    /// [`Error::TileOutsideGrid`], [`Error::InvalidParameter`] for a tile prepared
    /// for another ref layout. All or nothing.
    pub fn commit(&mut self, t: PreparedTile) -> Result<TileRef, Error> {
        let target = self.commit_target(&t)?;
        self.link_in(t.tile, target, Some(t.hash), true)
    }

    /// [`NavMesh::commit`] when `budget` covers [`NavMesh::commit_cost`]; otherwise
    /// nothing changes, the budget is marked exhausted and the tile comes back as
    /// [`Committed::Refused`].
    pub fn commit_budgeted(
        &mut self,
        t: PreparedTile,
        budget: &mut Budget,
    ) -> Result<Committed, Error> {
        let target = self.commit_target(&t)?;
        if !budget.spend(self.commit_cost(&t)) {
            return Ok(Committed::Refused(Box::new(t)));
        }
        Ok(Committed::Done(self.link_in(
            t.tile,
            target,
            Some(t.hash),
            true,
        )?))
    }

    /// [`NavMesh::commit`], then restores a tile state blob from
    /// [`NavMesh::unload_tile`] (runtime polygon flags and areas). The blob is
    /// checked against the ref the tile is about to get and the prepared content
    /// hash before anything changes. This also rejects other content in a fresh
    /// mesh whose initial ref happens to match. A bare `store_tile_state` blob
    /// lacks the identity footer and is rejected; use the state from `unload_tile`.
    /// Content mismatches fail with [`TileStateError::WrongTile`].
    pub fn commit_with_state(&mut self, t: PreparedTile, state: &[u8]) -> Result<TileRef, Error> {
        let target = self.commit_target(&t)?;
        let salt = match target.parked_hash {
            Some(old) if old != t.hash => self.next_salt(target.salt)?,
            _ => target.salt,
        };
        let would = self.encode_poly_id(salt, target.ti as u32, 0);
        let size = check_state(would, t.tile.polys.len(), state)?;
        let expected = size
            .checked_add(STREAM_STATE_FOOTER)
            .ok_or(Error::SizeOverflow)?;
        if state.len() < expected {
            return Err(Error::InvalidTileState(TileStateError::TooShort));
        }
        if state.len() != expected {
            return Err(Error::InvalidTileState(TileStateError::WrongTile));
        }
        let footer = &state[size..expected];
        if footer[..4] != STREAM_STATE_MAGIC {
            return Err(Error::InvalidTileState(TileStateError::WrongMagic));
        }
        if footer[4..8] != 2u32.to_le_bytes() {
            return Err(Error::InvalidTileState(TileStateError::WrongVersion));
        }
        if footer[8..40] != t.hash {
            return Err(Error::InvalidTileState(TileStateError::WrongTile));
        }
        let ti = target.ti;
        let r = self.link_in(t.tile, target, Some(t.hash), true)?;
        if let Some(tile) = self.slots[ti].tile.as_deref_mut() {
            apply_state(tile, state, size);
        }
        Ok(r)
    }

    fn commit_target(&self, t: &PreparedTile) -> Result<super::Target, Error> {
        if t.poly_bits != self.poly_bits {
            return Err(Error::InvalidParameter(
                "tile prepared for another ref layout",
            ));
        }
        let h = &t.tile.header;
        if self.tile_index_at(h.x, h.y, h.layer).is_some() {
            return Err(Error::TileSlotOccupied);
        }
        self.choose_slot(h.x, h.y, h.layer, Placement::Free)
    }

    /// Evicts a resident tile and keeps its refs: the slot stays reserved for the
    /// tile with its salt, refs into it read [`RefState::Unloaded`], and committing
    /// the same bytes again makes them valid. Returns the bytes, the tile state
    /// (runtime flags and areas) and the content hash. All or nothing.
    pub fn unload_tile(&mut self, r: TileRef) -> Result<UnloadedTile, Error> {
        let ti = self.resident_slot(r)?;
        let size = self.tile_state_size(r)?;
        let mut state = Vec::new();
        state
            .try_reserve_exact(
                size.checked_add(STREAM_STATE_FOOTER)
                    .ok_or(Error::SizeOverflow)?,
            )
            .map_err(|_| Error::AllocationFailed)?;
        state.resize(size, 0);
        self.store_tile_state(r, &mut state)?;
        let Some(tile) = self.detach(ti) else {
            return Err(Error::InvalidRef);
        };
        let h = tile.header;
        let key = TileKey { x: h.x, y: h.y };
        let hash = content_hash(&tile.data);
        // Capacity was reserved before detaching: appending cannot allocate.
        state.extend_from_slice(&STREAM_STATE_MAGIC);
        state.extend_from_slice(&2u32.to_le_bytes());
        state.extend_from_slice(&hash);
        let s = &mut self.slots[ti];
        s.next = -1;
        s.parked = Some(Parked {
            key,
            layer: h.layer,
            hash,
            polys: tile.polys.len() as u32,
        });
        self.stream.park(h.x, h.y, h.layer, ti);
        self.stream.log_removal(ti as u32);
        Ok(UnloadedTile {
            key,
            layer: h.layer,
            bytes: tile.data,
            state,
            content_hash: hash,
        })
    }

    /// Drops the reservation of an unloaded tile: its refs become
    /// [`RefState::Stale`] and the slot is free again (as after `remove_tile`).
    /// [`Error::InvalidParameter`] when no tile is unloaded at `key, layer`.
    pub fn forget_unloaded(&mut self, key: TileKey, layer: i32) -> Result<(), Error> {
        let ti = self
            .stream
            .parked_slot(&self.slots, key.x, key.y, layer)
            .ok_or(Error::InvalidParameter("no unloaded tile there"))?;
        let next = self.next_salt(self.slots[ti].salt)?;
        self.stream.unpark(key.x, key.y, layer);
        self.release_slot(ti, next);
        Ok(())
    }

    /// Why an edge of polygon `ip` of `tile` stops movement: a tile side facing a
    /// known, unloaded column, a side facing nothing, or a wall.
    pub(crate) fn edge_kind(&self, tile: &MeshTile, ip: usize, edge: usize) -> EdgeKind {
        let Some(p) = tile.polys.get(ip) else {
            return EdgeKind::Wall;
        };
        let nei = p.neis.get(edge).copied().unwrap_or(0);
        if nei & EXT_LINK == 0 {
            return EdgeKind::Wall;
        }
        let (dx, dy) = super::side_offset(i32::from(nei & 0xff));
        let (Some(x), Some(y)) = (tile.header.x.checked_add(dx), tile.header.y.checked_add(dy))
        else {
            return EdgeKind::WorldEdge;
        };
        if self.is_column_unloaded(x, y) {
            EdgeKind::UnloadedBorder(TileKey { x, y })
        } else if self.tiles_at(x, y, 1).is_empty() {
            EdgeKind::WorldEdge
        } else {
            EdgeKind::Wall
        }
    }

    /// Unloaded columns facing any edge of the polygons `refs` names, sorted and
    /// deduplicated.
    pub(crate) fn unloaded_frontier(&self, refs: impl Iterator<Item = PolyRef>) -> Vec<TileKey> {
        let mut out = Vec::new();
        if !self.has_unloaded() {
            return out;
        }
        for r in refs {
            let Ok((tile, poly)) = self.tile_and_poly_by_ref(r) else {
                continue;
            };
            let ip = self.decode_poly_id_poly(r) as usize;
            for e in 0..poly.vert_count as usize {
                if let EdgeKind::UnloadedBorder(k) = self.edge_kind(tile, ip, e) {
                    out.push(k);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// The navmesh's slot history: per slot that was ever used, its salt and, for
    /// a resident or unloaded tile, the tile's key, layer, content hash and
    /// polygon count. Restored into a new navmesh with
    /// [`NavMesh::restore_residency`], refs saved with it stay valid when the same
    /// tiles are committed again, and stale refs stay stale.
    pub fn residency(&self) -> ResidencyRecord {
        let mut entries = Vec::new();
        for i in self.slots.indices() {
            let s = &self.slots[i];
            if !s.used {
                continue;
            }
            let tile = match (&s.tile, s.parked) {
                (Some(t), _) => Some(Parked {
                    key: TileKey {
                        x: t.header.x,
                        y: t.header.y,
                    },
                    layer: t.header.layer,
                    hash: content_hash(&t.data),
                    polys: t.polys.len() as u32,
                }),
                (None, p) => p,
            };
            entries.push(ResidencyEntry {
                slot: i as u32,
                salt: s.salt,
                tile,
            });
        }
        ResidencyRecord {
            ref_bits: PolyRef::BITS,
            poly_bits: self.poly_bits,
            max_tiles: self.params.max_tiles as u32,
            policy: self.stream.policy,
            entries,
        }
    }

    /// Restores a [`NavMesh::residency`] record into a navmesh that has never held
    /// a tile (epoch 0). Every tile of the record becomes unloaded in its slot;
    /// committing the same bytes makes saved refs valid. The record must come
    /// from a navmesh with the same ref width, polygon-bit layout, `max_tiles`
    /// and slot policy. All or nothing.
    pub fn restore_residency(&mut self, rec: &ResidencyRecord) -> Result<(), Error> {
        if self.stream.epoch != 0 || self.resident != 0 {
            return Err(Error::InvalidParameter(
                "residency restored into a navmesh that held tiles",
            ));
        }
        if rec.ref_bits != PolyRef::BITS
            || rec.poly_bits != self.poly_bits
            || rec.max_tiles != self.params.max_tiles as u32
            || rec.policy != self.stream.policy
        {
            return Err(Error::InvalidParameter("residency record layout"));
        }
        let mask = ((1u64 << self.salt_bits) - 1) as u32;
        let mut keys = BTreeMap::new();
        let mut prev: Option<u32> = None;
        for e in &rec.entries {
            if prev.is_some_and(|p| e.slot <= p)
                || e.slot as usize >= self.slots.len()
                || e.salt == 0
                || e.salt > mask
            {
                return Err(Error::InvalidParameter("residency record entry"));
            }
            prev = Some(e.slot);
            if let Some(p) = e.tile {
                if u64::from(p.polys) > (1u64 << self.poly_bits) {
                    return Err(Error::InvalidParameter("residency record entry"));
                }
                if let Some(k) = self.stream.keyed_slot(p.key.x, p.key.y, p.layer) {
                    if k != Some(e.slot as usize) {
                        return Err(Error::InvalidParameter("residency record entry"));
                    }
                }
                if keys.insert((p.key.x, p.key.y, p.layer), e.slot).is_some() {
                    return Err(Error::InvalidParameter("residency record entry"));
                }
            }
        }
        for e in &rec.entries {
            self.slots.ensure(e.slot as usize)?;
        }
        // Apply: nothing below fails.
        for e in &rec.entries {
            let s = &mut self.slots[e.slot as usize];
            s.used = true;
            s.salt = e.salt;
            s.parked = e.tile;
            if let Some(p) = e.tile {
                self.stream.park(p.key.x, p.key.y, p.layer, e.slot as usize);
            }
        }
        if !self.stream.is_keyed() {
            self.next_free = -1;
            for i in (0..self.slots.len()).rev() {
                if self.slots[i].parked.is_none() {
                    self.slots[i].next = self.next_free;
                    self.next_free = i as i32;
                } else {
                    self.slots[i].next = -1;
                }
            }
        }
        self.stream.bump_epoch();
        Ok(())
    }
}

/// One slot of a [`ResidencyRecord`].
#[derive(Clone, Copy, Debug)]
struct ResidencyEntry {
    slot: u32,
    salt: u32,
    tile: Option<Parked>,
}

/// A navmesh's slot history (see [`NavMesh::residency`]), with a validated byte
/// form for saving alongside refs.
#[derive(Clone, Debug)]
pub struct ResidencyRecord {
    ref_bits: u32,
    poly_bits: u32,
    max_tiles: u32,
    policy: SlotPolicy,
    entries: Vec<ResidencyEntry>,
}

const RES_MAGIC: u32 = u32::from_le_bytes(*b"NVRS");
const RES_VERSION: u32 = 3;
const RES_HEADER: usize = 44;
const RES_ENTRY: usize = 64;

impl ResidencyRecord {
    /// Number of slots recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no slot was ever used.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Little-endian bytes: a 48-byte header (magic `NVRS`, version, ref bits,
    /// max tiles, policy, keyed origin and size, polygon bits, entry count) and 64 bytes per
    /// slot (slot, salt, tile flag, layer, x, y, polygon count, pad, content hash).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(RES_HEADER + self.entries.len() * RES_ENTRY);
        let (tag, o, w, h, l) = match self.policy {
            SlotPolicy::Keyed {
                origin,
                width,
                height,
                max_layers,
            } => (1u32, origin, width, height, max_layers),
            SlotPolicy::Sequential => (0, [0, 0], 0, 0, 0),
        };
        for v in [RES_MAGIC, RES_VERSION, self.ref_bits, self.max_tiles, tag] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&o[0].to_le_bytes());
        out.extend_from_slice(&o[1].to_le_bytes());
        for v in [w, h, l] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&self.poly_bits.to_le_bytes());
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            out.extend_from_slice(&e.slot.to_le_bytes());
            out.extend_from_slice(&e.salt.to_le_bytes());
            let p = e.tile.unwrap_or(Parked {
                key: TileKey::default(),
                layer: 0,
                hash: [0; 32],
                polys: 0,
            });
            out.extend_from_slice(&u32::from(e.tile.is_some()).to_le_bytes());
            out.extend_from_slice(&p.layer.to_le_bytes());
            out.extend_from_slice(&p.key.x.to_le_bytes());
            out.extend_from_slice(&p.key.y.to_le_bytes());
            out.extend_from_slice(&p.polys.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&p.hash);
        }
        out
    }

    /// Parses [`ResidencyRecord::to_bytes`] output. The bytes are untrusted:
    /// length, magic, version, ref width, policy, ascending slots, flag and pad
    /// values are checked, and an entry without a tile must have zero tile fields.
    /// Whether the record fits a navmesh is checked by
    /// [`NavMesh::restore_residency`].
    pub fn from_bytes(data: &[u8]) -> Result<ResidencyRecord, Error> {
        let bad = || Error::InvalidInput("residency record");
        let u = |at: usize| -> Result<u32, Error> {
            let b = data.get(at..at + 4).ok_or_else(bad)?;
            Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        let i = |at: usize| -> Result<i32, Error> { Ok(u(at)? as i32) };
        if u(0)? != RES_MAGIC || u(4)? != RES_VERSION {
            return Err(bad());
        }
        let ref_bits = u(8)?;
        if ref_bits != PolyRef::BITS {
            return Err(bad());
        }
        let max_tiles = u(12)?;
        let poly_bits = u(40)?;
        let policy = match u(16)? {
            0 => {
                if u(20)? != 0 || u(24)? != 0 || u(28)? != 0 || u(32)? != 0 || u(36)? != 0 {
                    return Err(bad());
                }
                SlotPolicy::Sequential
            }
            1 => SlotPolicy::Keyed {
                origin: [i(20)?, i(24)?],
                width: u(28)?,
                height: u(32)?,
                max_layers: u(36)?,
            },
            _ => return Err(bad()),
        };
        let count = u(RES_HEADER)? as usize;
        let body = RES_HEADER + 4;
        if data.len() < body
            || (data.len() - body) / RES_ENTRY != count
            || !(data.len() - body).is_multiple_of(RES_ENTRY)
        {
            return Err(bad());
        }
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        let mut prev: Option<u32> = None;
        for k in 0..count {
            let at = body + k * RES_ENTRY;
            let slot = u(at)?;
            if prev.is_some_and(|p| slot <= p) {
                return Err(bad());
            }
            prev = Some(slot);
            let salt = u(at + 4)?;
            let flag = u(at + 8)?;
            let (layer, x, y, polys, pad) = (
                i(at + 12)?,
                i(at + 16)?,
                i(at + 20)?,
                u(at + 24)?,
                u(at + 28)?,
            );
            let hb = data.get(at + 32..at + 64).ok_or_else(bad)?;
            let mut hash = [0u8; 32];
            hash.copy_from_slice(hb);
            if salt == 0 || pad != 0 {
                return Err(bad());
            }
            let tile = match flag {
                0 => {
                    if layer != 0 || x != 0 || y != 0 || polys != 0 || hash != [0; 32] {
                        return Err(bad());
                    }
                    None
                }
                1 => Some(Parked {
                    key: TileKey { x, y },
                    layer,
                    hash,
                    polys,
                }),
                _ => return Err(bad()),
            };
            entries.push(ResidencyEntry { slot, salt, tile });
        }
        Ok(ResidencyRecord {
            ref_bits,
            poly_bits,
            max_tiles,
            policy,
            entries,
        })
    }
}
