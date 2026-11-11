//! The navmesh (`dtNavMesh`): tile slots, poly refs and links.

use alloc::sync::Arc;
use alloc::vec::Vec;

use super::math::*;
use super::tile::{MeshTile, Poly, peek_header};
use super::{
    DETAIL_EDGE_BOUNDARY, EXT_LINK, NAVMESH_STATE_MAGIC, NAVMESH_STATE_VERSION, NULL_LINK,
    OFFMESH_CON_BIDIR, POLYTYPE_OFFMESH_CONNECTION, PolyRef, TileRef, VERTS_PER_POLYGON,
};
use crate::common::floorf;
use crate::{Error, Limits, TileDataError};
use alloc::boxed::Box;

pub(crate) mod stream;
pub use stream::{
    Committed, EdgeKind, PreparedTile, REMOVAL_LOG_CAPACITY, RefState, ResidencyRecord, SlotPolicy,
    TileKey, UnloadedTile, content_hash, prepare_tile,
};

// An owner token survives moves. A cloned mesh gets a new token because the two
// owners can make different edits at the same epoch.
#[derive(Debug)]
struct MeshIdentity(Arc<()>);

impl Clone for MeshIdentity {
    fn clone(&self) -> Self {
        Self(Arc::new(()))
    }
}

/// Size of the tile state header: `int` magic, `int` version, tile ref.
const TILE_STATE_HEADER_SIZE: usize = 8 + (PolyRef::BITS / 8) as usize;
/// Size of one polygon's state: `u16` flags, `u8` area, one pad byte.
const TILE_POLY_STATE_SIZE: usize = 4;

/// Navmesh layout (`dtNavMeshParams`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NavMeshParams {
    /// World-space origin of the tile grid.
    pub orig: [f32; 3],
    /// Tile width (x).
    pub tile_width: f32,
    /// Tile height (z).
    pub tile_height: f32,
    /// Maximum number of tiles.
    pub max_tiles: i32,
    /// Maximum number of polygons per tile.
    pub max_polys: i32,
}

#[derive(Clone, Debug)]
struct Slot {
    salt: u32,
    next: i32,
    tile: Option<Box<MeshTile>>,
    /// Whether the slot ever held a tile (and so may have handed out refs).
    used: bool,
    /// Set while the slot's tile is unloaded: the slot is reserved for that tile and
    /// keeps its salt.
    parked: Option<stream::Parked>,
}

/// A never-allocated slot (read through [`SlotTable`] for pages not allocated yet).
static EMPTY_SLOT: Slot = Slot {
    salt: 1,
    next: -1,
    tile: None,
    used: false,
    parked: None,
};

/// Slots per page of a keyed navmesh (pages are allocated on first use, so a large
/// keyed grid costs memory only where tiles were committed).
const PAGE_SHIFT: u32 = 10;

/// The slot array, in pages. A sequential navmesh has a single page allocated up
/// front; a keyed one allocates pages as tiles reach them.
#[derive(Clone, Debug)]
struct SlotTable {
    pages: Vec<Option<Box<[Slot]>>>,
    shift: u32,
    len: usize,
    /// Write target for a slot of a page that is not allocated. Every mutating path
    /// allocates the page first, so nothing is ever written here.
    sink: Slot,
}

impl SlotTable {
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> Option<&Slot> {
        if i >= self.len {
            return None;
        }
        match self.pages.get(i >> self.shift) {
            Some(Some(p)) => p.get(i & ((1 << self.shift) - 1)),
            _ => Some(&EMPTY_SLOT),
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Slot> {
        if i >= self.len {
            return None;
        }
        let mask = (1usize << self.shift) - 1;
        match self.pages.get_mut(i >> self.shift) {
            Some(Some(p)) => p.get_mut(i & mask),
            _ => None,
        }
    }

    /// Allocates the page holding slot `i`.
    fn ensure(&mut self, i: usize) -> Result<(), Error> {
        if i >= self.len {
            return Err(Error::NoFreeTile);
        }
        let page = i >> self.shift;
        let Some(entry) = self.pages.get_mut(page) else {
            return Err(Error::NoFreeTile);
        };
        if entry.is_none() {
            let n = (1usize << self.shift).min(self.len - (page << self.shift));
            let mut v = Vec::new();
            v.try_reserve_exact(n)
                .map_err(|_| Error::AllocationFailed)?;
            v.resize(n, EMPTY_SLOT.clone());
            *entry = Some(v.into_boxed_slice());
        }
        Ok(())
    }

    /// Indices of the allocated slots, ascending.
    fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        let shift = self.shift;
        self.pages
            .iter()
            .enumerate()
            .filter_map(move |(p, pg)| pg.as_ref().map(|s| (p << shift)..(p << shift) + s.len()))
            .flatten()
    }
}

impl core::ops::Index<usize> for SlotTable {
    type Output = Slot;
    fn index(&self, i: usize) -> &Slot {
        self.get(i).unwrap_or(&EMPTY_SLOT)
    }
}

impl core::ops::IndexMut<usize> for SlotTable {
    fn index_mut(&mut self, i: usize) -> &mut Slot {
        let mask = (1usize << self.shift) - 1;
        if i < self.len {
            if let Some(Some(p)) = self.pages.get_mut(i >> self.shift) {
                if let Some(s) = p.get_mut(i & mask) {
                    return s;
                }
            }
        }
        &mut self.sink
    }
}

/// A navmesh: a grid of tiles with links between their polygons.
///
/// # Salts
///
/// Each tile slot carries a salt that is part of every ref into it; removing a
/// tile increments the salt, so refs to the removed tile stop validating. The salt
/// has `salt_bits` bits (16 with 64-bit refs, at least 10 with 32-bit refs) and
/// skips 0. At salt `2^salt_bits - 1`, a removal, forgotten reservation or
/// replacement requiring a new salt fails with [`Error::SaltExhausted`] before
/// mutation. Thus a slot permits at most `2^salt_bits - 2` salt advances in its
/// lifetime (65,534 with 64-bit refs). Identical unload/reload remains possible.
///
/// [`NavMesh::unload_tile`] evicts a tile without changing its salt, so refs
/// survive an identical reload; see the streaming section of the `detour` module
/// (`stream` items: [`SlotPolicy`], [`prepare_tile`], [`NavMesh::ref_state`]).
#[derive(Clone, Debug)]
pub struct NavMesh {
    identity: MeshIdentity,
    params: NavMeshParams,
    lut_mask: i32,
    pos_lookup: Vec<i32>,
    next_free: i32,
    slots: SlotTable,
    salt_bits: u32,
    tile_bits: u32,
    poly_bits: u32,
    limits: Limits,
    /// Number of resident tiles.
    resident: usize,
    /// Tile-grid bounds (min x, min y, max x, max y) covering every resident tile
    /// (grown on add, reset when the navmesh empties).
    occupied: Option<[i32; 4]>,
    /// Whether off-mesh connections resolve their far endpoint in any tile.
    global_off_mesh: bool,
    /// Streaming state: slot policy, unloaded tiles, known columns, epoch.
    stream: stream::StreamState,
}

/// A slot chosen for a tile, before anything changed.
#[derive(Clone, Copy)]
struct Target {
    ti: usize,
    /// Free-list predecessor (`Some(-1)` for the head) when the slot is taken off
    /// the free list.
    prev: Option<i32>,
    salt: u32,
    /// Content hash of the unloaded tile the slot is reserved for.
    parked_hash: Option<[u8; 32]>,
}

/// Where [`NavMesh::insert_tile`] puts a tile.
#[derive(Clone, Copy)]
enum Placement {
    /// The next free slot.
    Free,
    /// Slot `idx` with salt `salt`; with `stale` false, only into a never-used slot
    /// or with the slot's current salt.
    Restore { idx: u32, salt: u32, stale: bool },
}

/// Salt, tile and polygon bits of the ref layout of `params`, after checking the
/// parameters (`dtNavMesh::init`).
fn layout_bits(params: &NavMeshParams) -> Result<(u32, u32, u32), Error> {
    if params.max_tiles <= 0 || params.max_polys <= 0 {
        return Err(Error::InvalidParameter(
            "max_tiles and max_polys must be positive",
        ));
    }
    let size_ok = |s: f32| s.is_finite() && s > 0.0;
    if !visfinite(&params.orig) || !size_ok(params.tile_width) || !size_ok(params.tile_height) {
        return Err(Error::InvalidParameter("orig, tile size"));
    }
    if PolyRef::BITS == 64 {
        if params.max_tiles > 1 << 28 || params.max_polys > 1 << 20 {
            return Err(Error::InvalidParameter(
                "max_tiles or max_polys beyond the 64-bit ref layout",
            ));
        }
        Ok((16, 28, 20))
    } else {
        let tile_bits = ilog2(next_pow2(params.max_tiles as u32));
        let poly_bits = ilog2(next_pow2(params.max_polys as u32));
        if tile_bits + poly_bits > 31 {
            return Err(Error::InvalidParameter(
                "max_tiles and max_polys need more than 31 bits",
            ));
        }
        let salt_bits = 31u32.min(32 - tile_bits - poly_bits);
        if salt_bits < 10 {
            return Err(Error::InvalidParameter("fewer than 10 salt bits"));
        }
        Ok((salt_bits, tile_bits, poly_bits))
    }
}

/// Tile-grid offset of a portal side (`0` is +x, counter-clockwise).
fn side_offset(side: i32) -> (i32, i32) {
    match side {
        0 => (1, 0),
        1 => (1, 1),
        2 => (0, 1),
        3 => (-1, 1),
        4 => (-1, 0),
        5 => (-1, -1),
        6 => (0, -1),
        7 => (1, -1),
        _ => (0, 0),
    }
}

fn tile_hash(x: i32, y: i32, mask: i32) -> usize {
    let n = 0x8da6_b343u32
        .wrapping_mul(x as u32)
        .wrapping_add(0xd816_3841u32.wrapping_mul(y as u32));
    (n & mask as u32) as usize
}

fn round_unit(x: f32) -> u8 {
    // roundf for x in [0, 255]: x - floor(x) is exact there.
    let f = floorf(x);
    (if x - f >= 0.5 { f + 1.0 } else { f }) as u8
}

fn slab_coord(va: &[f32], side: i32) -> f32 {
    match side {
        0 | 4 => va[0],
        2 | 6 => va[2],
        _ => 0.0,
    }
}

fn slab_end_points(va: &[f32], vb: &[f32], side: i32) -> ([f32; 2], [f32; 2]) {
    let k = match side {
        0 | 4 => 2,
        2 | 6 => 0,
        _ => return ([0.0; 2], [0.0; 2]),
    };
    if va[k] < vb[k] {
        ([va[k], va[1]], [vb[k], vb[1]])
    } else {
        ([vb[k], vb[1]], [va[k], va[1]])
    }
}

fn overlap_slabs(
    amin: &[f32; 2],
    amax: &[f32; 2],
    bmin: &[f32; 2],
    bmax: &[f32; 2],
    px: f32,
    py: f32,
) -> bool {
    let minx = fmax(amin[0] + px, bmin[0] + px);
    let maxx = fmin(amax[0] - px, bmax[0] - px);
    if minx > maxx {
        return false;
    }
    let ad = (amax[1] - amin[1]) / (amax[0] - amin[0]);
    let ak = amin[1] - ad * amin[0];
    let bd = (bmax[1] - bmin[1]) / (bmax[0] - bmin[0]);
    let bk = bmin[1] - bd * bmin[0];
    let aminy = ad * minx + ak;
    let amaxy = ad * maxx + ak;
    let bminy = bd * minx + bk;
    let bmaxy = bd * maxx + bk;
    let dmin = bminy - aminy;
    let dmax = bmaxy - amaxy;
    if dmin * dmax < 0.0 {
        return true;
    }
    let thr = sqr(py * 2.0);
    dmin * dmin <= thr || dmax * dmax <= thr
}

fn alloc_link(tile: &mut MeshTile) -> u32 {
    let link = tile.links_free_list;
    match tile.links.get(link as usize) {
        Some(l) => {
            tile.links_free_list = l.next;
            link
        }
        None => NULL_LINK,
    }
}

fn free_link(tile: &mut MeshTile, link: u32) {
    if let Some(l) = tile.links.get_mut(link as usize) {
        l.next = tile.links_free_list;
        tile.links_free_list = link;
    }
}

/// The vertices of a detail triangle corner: polygon vertex or detail vertex.
#[inline]
pub(crate) fn detail_vert<'a>(tile: &'a MeshTile, poly: &Poly, vert_base: u32, k: u8) -> &'a [f32] {
    if k < poly.vert_count {
        let v = poly.verts[k as usize] as usize * 3;
        &tile.verts[v..v + 3]
    } else {
        let v = (vert_base as usize + (k - poly.vert_count) as usize) * 3;
        &tile.detail_verts[v..v + 3]
    }
}

pub(crate) fn closest_point_on_detail_edges(
    tile: &MeshTile,
    ip: usize,
    pos: &[f32],
    only_boundary: bool,
) -> V3 {
    let poly = &tile.polys[ip];
    let pd = &tile.detail_meshes[ip];
    let mut dmin = f32::MAX;
    let mut tmin = 0.0;
    let mut pmin: &[f32] = &[0.0; 3];
    let mut pmax: &[f32] = &[0.0; 3];
    const ANY_BOUNDARY_EDGE: u8 =
        (DETAIL_EDGE_BOUNDARY | DETAIL_EDGE_BOUNDARY << 2 | DETAIL_EDGE_BOUNDARY << 4) as u8;
    for i in 0..pd.tri_count as usize {
        let t = (pd.tri_base as usize + i) * 4;
        let tris = &tile.detail_tris[t..t + 4];
        if only_boundary && (tris[3] & ANY_BOUNDARY_EDGE) == 0 {
            continue;
        }
        let v = [
            detail_vert(tile, poly, pd.vert_base, tris[0]),
            detail_vert(tile, poly, pd.vert_base, tris[1]),
            detail_vert(tile, poly, pd.vert_base, tris[2]),
        ];
        let mut j = 2;
        for k in 0..3 {
            let flags = (tris[3] >> (j * 2)) & 0x3;
            if (flags as i32 & DETAIL_EDGE_BOUNDARY) == 0 && (only_boundary || tris[j] < tris[k]) {
                j = k;
                continue;
            }
            let (d, t) = distance_pt_seg_sqr_2d(pos, v[j], v[k]);
            if d < dmin {
                dmin = d;
                tmin = t;
                pmin = v[j];
                pmax = v[k];
            }
            j = k;
        }
    }
    vlerp(pmin, pmax, tmin)
}

/// Height of `pos` on polygon `ip` of `tile` (`dtNavMesh::getPolyHeight`).
pub(crate) fn poly_height(tile: &MeshTile, ip: usize, pos: &[f32]) -> Option<f32> {
    let poly = &tile.polys[ip];
    if poly.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
        return None;
    }
    let pd = &tile.detail_meshes[ip];
    let mut verts = [0.0f32; VERTS_PER_POLYGON * 3];
    let nv = poly.vert_count as usize;
    for i in 0..nv {
        let v = poly.verts[i] as usize * 3;
        verts[i * 3..i * 3 + 3].copy_from_slice(&tile.verts[v..v + 3]);
    }
    if !point_in_polygon(pos, &verts, nv) {
        return None;
    }
    for j in 0..pd.tri_count as usize {
        let t = (pd.tri_base as usize + j) * 4;
        let tr = &tile.detail_tris[t..t + 4];
        let v = [
            detail_vert(tile, poly, pd.vert_base, tr[0]),
            detail_vert(tile, poly, pd.vert_base, tr[1]),
            detail_vert(tile, poly, pd.vert_base, tr[2]),
        ];
        if let Some(h) = closest_height_point_triangle(pos, v[0], v[1], v[2]) {
            return Some(h);
        }
    }
    Some(closest_point_on_detail_edges(tile, ip, pos, false)[1])
}

/// `dtNavMesh::closestPointOnPoly` on polygon `ip` of `tile`.
pub(crate) fn closest_point_on_poly_in_tile(tile: &MeshTile, ip: usize, pos: &[f32]) -> (V3, bool) {
    let mut closest = v3(pos);
    if let Some(h) = poly_height(tile, ip, pos) {
        closest[1] = h;
        return (closest, true);
    }
    let poly = &tile.polys[ip];
    if poly.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
        let v0 = &tile.verts[poly.verts[0] as usize * 3..];
        let v1 = &tile.verts[poly.verts[1] as usize * 3..];
        let (_, t) = distance_pt_seg_sqr_2d(pos, v0, v1);
        return (vlerp(v0, v1, t), false);
    }
    (closest_point_on_detail_edges(tile, ip, pos, true), false)
}

/// Polygons of `tile` whose bounds overlap `qmin`..`qmax` (`queryPolygonsInTile`),
/// as polygon indices, through `f` until it returns false... collected in order.
pub(crate) fn query_polygons_in_tile(
    tile: &MeshTile,
    qmin: &[f32],
    qmax: &[f32],
    out: &mut Vec<u32>,
    max: usize,
) {
    out.clear();
    query_polygons_in_tile_with(tile, qmin, qmax, &mut |ip| {
        if out.len() < max {
            out.push(ip);
        }
        true
    });
}

pub(crate) fn query_polygons_in_tile_with(
    tile: &MeshTile,
    qmin: &[f32],
    qmax: &[f32],
    visit: &mut impl FnMut(u32) -> bool,
) -> bool {
    let h = &tile.header;
    if !tile.bv_tree.is_empty() {
        let tbmin = &h.bmin;
        let tbmax = &h.bmax;
        let qfac = h.bv_quant_factor;
        let q = |v: f32, k: usize| fclamp(v, tbmin[k], tbmax[k]) - tbmin[k];
        let bmin = [0, 1, 2].map(|k| ((qfac * q(qmin[k], k)) as u16) & 0xfffe);
        let bmax = [0, 1, 2].map(|k| ((qfac * q(qmax[k], k) + 1.0) as u16) | 1);
        let end = tile.bv_tree.len();
        let mut i = 0usize;
        while i < end {
            let node = &tile.bv_tree[i];
            let overlap = overlap_quant_bounds(&bmin, &bmax, &node.bmin, &node.bmax);
            let leaf = node.i >= 0;
            if leaf && overlap && !visit(node.i as u32) {
                return false;
            }
            if overlap || leaf {
                i += 1;
            } else {
                i += (-(node.i as i64)) as usize;
            }
        }
    } else {
        for (i, p) in tile.polys.iter().enumerate() {
            if p.poly_type() == POLYTYPE_OFFMESH_CONNECTION {
                continue;
            }
            let v = &tile.verts[p.verts[0] as usize * 3..];
            let mut bmin = v3(v);
            let mut bmax = bmin;
            for j in 1..p.vert_count as usize {
                let v = &tile.verts[p.verts[j] as usize * 3..];
                vmin(&mut bmin, v);
                vmax(&mut bmax, v);
            }
            if overlap_bounds(qmin, qmax, &bmin, &bmax) && !visit(i as u32) {
                return false;
            }
        }
    }
    true
}

/// `findNearestPolyInTile`: nearest polygon index and point.
pub(crate) fn find_nearest_poly_in_tile(
    tile: &MeshTile,
    center: &[f32],
    half: &[f32],
) -> Option<(u32, V3)> {
    let bmin = vsub(center, half);
    let bmax = vadd(center, half);
    let mut polys = Vec::new();
    query_polygons_in_tile(tile, &bmin, &bmax, &mut polys, 128);
    let mut nearest = None;
    let mut nearest_d = f32::MAX;
    for &ip in &polys {
        let (cp, over) = closest_point_on_poly_in_tile(tile, ip as usize, center);
        let diff = vsub(center, &cp);
        let d = if over {
            let d = diff[1].abs() - tile.header.walkable_climb;
            if d > 0.0 { d * d } else { 0.0 }
        } else {
            vlen_sqr(&diff)
        };
        if d < nearest_d {
            nearest_d = d;
            nearest = Some((ip, cp));
        }
    }
    nearest
}

impl NavMesh {
    /// Creates an empty navmesh (`dtNavMesh::init` with parameters). The origin must
    /// be finite and the tile sizes finite and positive.
    pub fn new(params: &NavMeshParams) -> Result<NavMesh, Error> {
        Self::with_limits(params, Limits::UNLIMITED)
    }

    /// Creates an empty navmesh whose tile reader applies `limits` (see
    /// [`MeshTile::parse_with_limits`]).
    pub fn with_limits(params: &NavMeshParams, limits: Limits) -> Result<NavMesh, Error> {
        Self::with_policy(params, limits, SlotPolicy::Sequential)
    }

    /// Creates an empty navmesh whose tiles take slots by `policy` (see
    /// [`SlotPolicy`]). [`SlotPolicy::Keyed`] needs `width * height * max_layers`
    /// slots, at most `params.max_tiles`; its slot pages are allocated as tiles
    /// reach them.
    pub fn with_policy(
        params: &NavMeshParams,
        limits: Limits,
        policy: SlotPolicy,
    ) -> Result<NavMesh, Error> {
        let (salt_bits, tile_bits, poly_bits) = layout_bits(params)?;
        policy.validate(params)?;
        let keyed = matches!(policy, SlotPolicy::Keyed { .. });
        let mut lut = if keyed {
            1
        } else {
            next_pow2((params.max_tiles / 4) as u32) as i32
        };
        if lut == 0 {
            lut = 1;
        }
        let n = params.max_tiles as usize;
        let shift = if keyed {
            PAGE_SHIFT
        } else {
            usize::BITS - n.leading_zeros()
        };
        let npages = if keyed {
            n.div_ceil(1 << PAGE_SHIFT)
        } else {
            1
        };
        let mut pages = Vec::new();
        pages
            .try_reserve_exact(npages)
            .map_err(|_| Error::AllocationFailed)?;
        pages.resize(npages, None);
        let mut slots = SlotTable {
            pages,
            shift,
            len: n,
            sink: EMPTY_SLOT.clone(),
        };
        if !keyed {
            // Upstream's free list: every slot, in index order.
            slots.ensure(0)?;
            for i in 0..n {
                slots[i].next = if i + 1 < n { i as i32 + 1 } else { -1 };
            }
        }
        let mut pos_lookup = Vec::new();
        pos_lookup
            .try_reserve_exact(lut as usize)
            .map_err(|_| Error::AllocationFailed)?;
        pos_lookup.resize(lut as usize, -1);
        Ok(NavMesh {
            identity: MeshIdentity(Arc::new(())),
            params: *params,
            lut_mask: lut - 1,
            pos_lookup,
            next_free: if keyed { -1 } else { 0 },
            slots,
            salt_bits,
            tile_bits,
            poly_bits,
            limits,
            resident: 0,
            occupied: None,
            global_off_mesh: false,
            stream: stream::StreamState::new(policy)?,
        })
    }

    pub(crate) fn report_identity(&self) -> &Arc<()> {
        &self.identity.0
    }

    /// The limits the tile reader applies.
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Changes the limits for tiles added later.
    pub fn set_limits(&mut self, limits: Limits) {
        self.limits = limits;
    }

    /// Creates a single-tile navmesh from tile bytes (`dtNavMesh::init` with data).
    pub fn new_single(data: Vec<u8>) -> Result<(NavMesh, TileRef), Error> {
        let h = peek_header(&data)?;
        let params = NavMeshParams {
            orig: h.bmin,
            tile_width: h.bmax[0] - h.bmin[0],
            tile_height: h.bmax[2] - h.bmin[2],
            max_tiles: 1,
            max_polys: h.poly_count,
        };
        let mut nav = NavMesh::new(&params)?;
        let r = nav.add_tile(data)?;
        Ok((nav, r))
    }

    /// Layout parameters.
    pub fn params(&self) -> &NavMeshParams {
        &self.params
    }

    /// Maximum number of tiles.
    pub fn max_tiles(&self) -> i32 {
        self.params.max_tiles
    }

    /// The tile in slot `i`, if any.
    pub fn tile(&self, i: usize) -> Option<&MeshTile> {
        self.slots.get(i).and_then(|s| s.tile.as_deref())
    }

    /// The tile in slot `i`, or an empty tile. Callers pass tile indices decoded
    /// from refs this navmesh created; any other index reads as an empty tile.
    #[inline]
    pub(crate) fn tile_unchecked(&self, i: u32) -> &MeshTile {
        self.slots
            .get(i as usize)
            .and_then(|s| s.tile.as_deref())
            .unwrap_or(&EMPTY_TILE)
    }

    /// Whether `r` has no bit set outside the salt, tile and polygon fields.
    #[inline]
    fn ref_in_layout(&self, r: PolyRef) -> bool {
        let used = self.salt_bits + self.tile_bits + self.poly_bits;
        used >= PolyRef::BITS || r >> used == 0
    }

    /// Packs salt, tile index and polygon index into a ref.
    #[inline]
    pub fn encode_poly_id(&self, salt: u32, it: u32, ip: u32) -> PolyRef {
        ((salt as PolyRef) << (self.poly_bits + self.tile_bits))
            | ((it as PolyRef) << self.poly_bits)
            | ip as PolyRef
    }

    /// Unpacks a ref into salt, tile index and polygon index.
    #[inline]
    pub fn decode_poly_id(&self, r: PolyRef) -> (u32, u32, u32) {
        let m = |bits: u32| ((1u64 << bits) - 1) as PolyRef;
        (
            ((r >> (self.poly_bits + self.tile_bits)) & m(self.salt_bits)) as u32,
            ((r >> self.poly_bits) & m(self.tile_bits)) as u32,
            (r & m(self.poly_bits)) as u32,
        )
    }

    /// The tile index of a ref.
    #[inline]
    pub fn decode_poly_id_tile(&self, r: PolyRef) -> u32 {
        self.decode_poly_id(r).1
    }

    /// The polygon index of a ref.
    #[inline]
    pub fn decode_poly_id_poly(&self, r: PolyRef) -> u32 {
        self.decode_poly_id(r).2
    }

    /// Ref of polygon 0 of slot `i` with the slot's current salt (`None` when `i`
    /// is not a slot).
    #[inline]
    pub fn poly_ref_base(&self, i: usize) -> Option<PolyRef> {
        let s = self.slots.get(i)?;
        Some(self.encode_poly_id(s.salt, i as u32, 0))
    }

    /// [`NavMesh::poly_ref_base`] for slot indices this navmesh produced (0 for any
    /// other index).
    #[inline]
    pub(crate) fn base_ref(&self, i: usize) -> PolyRef {
        self.poly_ref_base(i).unwrap_or(0)
    }

    /// Ref of the tile in slot `i` (0 when empty).
    pub fn tile_ref(&self, i: usize) -> TileRef {
        match self.slots.get(i) {
            Some(s) if s.tile.is_some() => self.encode_poly_id(s.salt, i as u32, 0),
            _ => 0,
        }
    }

    /// Tile grid location of a world position.
    pub fn calc_tile_loc(&self, pos: &[f32; 3]) -> (i32, i32) {
        (
            floorf((pos[0] - self.params.orig[0]) / self.params.tile_width) as i32,
            floorf((pos[2] - self.params.orig[2]) / self.params.tile_height) as i32,
        )
    }

    fn chain(&self, x: i32, y: i32) -> impl Iterator<Item = usize> + '_ {
        // A keyed navmesh has no lookup lists: the slots of a column are computed.
        let mut keyed = self.stream.column_slots(x, y);
        let mut cur = if keyed.is_some() {
            -1
        } else {
            self.pos_lookup
                .get(tile_hash(x, y, self.lut_mask))
                .copied()
                .unwrap_or(-1)
        };
        let mut left = self.slots.len();
        core::iter::from_fn(move || {
            if let Some(r) = keyed.as_mut() {
                // Canonical installation adds ascending layers; sequential
                // lookup lists therefore visit higher layers first.
                return r.next_back();
            }
            if cur < 0 || left == 0 {
                return None;
            }
            left -= 1;
            let i = cur as usize;
            cur = self.slots.get(i).map_or(-1, |s| s.next);
            Some(i)
        })
    }

    /// Slot index of the tile at `x, y, layer`.
    pub fn tile_index_at(&self, x: i32, y: i32, layer: i32) -> Option<usize> {
        self.chain(x, y).find(|&i| {
            self.tile(i)
                .is_some_and(|t| t.header.x == x && t.header.y == y && t.header.layer == layer)
        })
    }

    /// Slot indices of every tile at `x, y` (all layers), in lookup order.
    pub fn tiles_at(&self, x: i32, y: i32, max: usize) -> Vec<usize> {
        self.chain(x, y)
            .filter(|&i| {
                self.tile(i)
                    .is_some_and(|t| t.header.x == x && t.header.y == y)
            })
            .take(max)
            .collect()
    }

    /// Tile-grid bounds `[min x, min y, max x, max y]` that cover every resident
    /// tile (`None` when the navmesh is empty). The bounds grow as tiles are added
    /// and may stay wider than the resident tiles after removals.
    pub fn occupied_tile_bounds(&self) -> Option<[i32; 4]> {
        self.occupied
    }

    /// Calls `f(slot)` for every resident tile with `minx <= x <= maxx` and
    /// `miny <= y <= maxy`, in upstream's scan order: rows of increasing y, columns
    /// of increasing x, and lookup order within a location. Stops when `f` returns
    /// false. The work is bounded by the resident tiles, not by the range size.
    pub(crate) fn for_tiles_in(
        &self,
        (minx, miny): (i32, i32),
        (maxx, maxy): (i32, i32),
        f: &mut dyn FnMut(usize) -> bool,
    ) {
        let Some([ox0, oy0, ox1, oy1]) = self.occupied else {
            return;
        };
        let (x0, y0, x1, y1) = (minx.max(ox0), miny.max(oy0), maxx.min(ox1), maxy.min(oy1));
        if x0 > x1 || y0 > y1 {
            return;
        }
        // The full i32 grid has 2^64 cells. Saturation keeps that range on the
        // sparse path instead of overflowing (or wrapping to zero in release).
        let cells =
            ((x1 as i64 - x0 as i64 + 1) as u64).saturating_mul((y1 as i64 - y0 as i64 + 1) as u64);
        if cells <= self.slots.len() as u64 {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    for ti in self.tiles_at(x, y, 32) {
                        if !f(ti) {
                            return;
                        }
                    }
                }
            }
            return;
        }
        // A sparse range: visit the distinct resident locations in the same order.
        let mut locs: Vec<(i32, i32)> = self
            .slots
            .indices()
            .filter_map(|i| self.tile(i))
            .map(|t| (t.header.y, t.header.x))
            .filter(|&(y, x)| (x0..=x1).contains(&x) && (y0..=y1).contains(&y))
            .collect();
        locs.sort_unstable();
        locs.dedup();
        for (y, x) in locs {
            for ti in self.tiles_at(x, y, 32) {
                if !f(ti) {
                    return;
                }
            }
        }
    }

    fn neighbour_tiles_at(&self, x: i32, y: i32, side: i32) -> Vec<usize> {
        let (dx, dy) = side_offset(side);
        // A neighbour outside the i32 grid does not exist.
        match (x.checked_add(dx), y.checked_add(dy)) {
            (Some(nx), Some(ny)) => self.tiles_at(nx, ny, 32),
            _ => Vec::new(),
        }
    }

    /// Tile and polygon of a ref, validated (`getTileAndPolyByRef`).
    pub fn tile_and_poly_by_ref(&self, r: PolyRef) -> Result<(&MeshTile, &Poly), Error> {
        if r == 0 {
            return Err(Error::Failed);
        }
        if !self.ref_in_layout(r) {
            return Err(Error::InvalidRef);
        }
        let (salt, it, ip) = self.decode_poly_id(r);
        let s = self.slots.get(it as usize).ok_or(Error::InvalidRef)?;
        let t = s.tile.as_ref().ok_or(Error::InvalidRef)?;
        if s.salt != salt {
            return Err(Error::InvalidRef);
        }
        let p = t.polys.get(ip as usize).ok_or(Error::InvalidRef)?;
        Ok((t, p))
    }

    /// Tile and polygon of a ref known to be valid (refs read from links).
    #[inline]
    pub(crate) fn tile_and_poly_unsafe(&self, r: PolyRef) -> (&MeshTile, usize) {
        let (_, it, ip) = self.decode_poly_id(r);
        (self.tile_unchecked(it), ip as usize)
    }

    /// Whether a ref names a polygon of a resident tile.
    pub fn is_valid_poly_ref(&self, r: PolyRef) -> bool {
        self.tile_and_poly_by_ref(r).is_ok()
    }

    /// Tile of a tile ref.
    pub fn tile_by_ref(&self, r: TileRef) -> Option<&MeshTile> {
        if r == 0 || !self.ref_in_layout(r) {
            return None;
        }
        let (salt, it, _) = self.decode_poly_id(r);
        let s = self.slots.get(it as usize)?;
        if s.salt != salt {
            return None;
        }
        s.tile.as_deref()
    }

    /// End points of an off-mesh connection polygon, oriented away from `prev`.
    pub fn off_mesh_connection_poly_end_points(
        &self,
        prev: PolyRef,
        r: PolyRef,
    ) -> Result<(V3, V3), Error> {
        let (tile, poly) = self.tile_and_poly_by_ref(r)?;
        if poly.poly_type() != POLYTYPE_OFFMESH_CONNECTION {
            return Err(Error::Failed);
        }
        let (mut i0, mut i1) = (0, 1);
        let mut i = poly.first_link;
        while let Some(l) = tile.links.get(i as usize) {
            if l.edge == 0 {
                if l.r#ref != prev {
                    i0 = 1;
                    i1 = 0;
                }
                break;
            }
            i = l.next;
        }
        let v = |k: usize| v3(&tile.verts[poly.verts[k] as usize * 3..]);
        Ok((v(i0), v(i1)))
    }

    /// The off-mesh connection of an off-mesh polygon ref.
    pub fn off_mesh_connection_by_ref(&self, r: PolyRef) -> Option<&super::OffMeshConnection> {
        let (tile, poly) = self.tile_and_poly_by_ref(r).ok()?;
        if poly.poly_type() != POLYTYPE_OFFMESH_CONNECTION {
            return None;
        }
        let ip = self.decode_poly_id_poly(r) as i64 - tile.header.off_mesh_base as i64;
        tile.off_mesh_cons.get(usize::try_from(ip).ok()?)
    }

    fn poly_mut(&mut self, r: PolyRef) -> Result<&mut Poly, Error> {
        self.tile_and_poly_by_ref(r)?;
        let (_, it, ip) = self.decode_poly_id(r);
        self.slots
            .get_mut(it as usize)
            .and_then(|s| s.tile.as_deref_mut())
            .and_then(|t| t.polys.get_mut(ip as usize))
            .ok_or(Error::InvalidRef)
    }

    /// Sets a polygon's flags.
    pub fn set_poly_flags(&mut self, r: PolyRef, flags: u16) -> Result<(), Error> {
        self.poly_mut(r)?.flags = flags;
        Ok(())
    }

    /// A polygon's flags.
    pub fn poly_flags(&self, r: PolyRef) -> Result<u16, Error> {
        Ok(self.tile_and_poly_by_ref(r)?.1.flags)
    }

    /// Sets a polygon's area.
    pub fn set_poly_area(&mut self, r: PolyRef, area: u8) -> Result<(), Error> {
        self.poly_mut(r)?.set_area(area);
        Ok(())
    }

    /// A polygon's area.
    pub fn poly_area(&self, r: PolyRef) -> Result<u8, Error> {
        Ok(self.tile_and_poly_by_ref(r)?.1.area())
    }

    /// Bytes of the state blob of a resident tile (`dtNavMesh::getTileStateSize`):
    /// a header (magic, version, tile ref) then four bytes per polygon.
    /// [`Error::InvalidRef`] when `tile` is not a resident tile's ref.
    pub fn tile_state_size(&self, tile: TileRef) -> Result<usize, Error> {
        let t = self.tile_by_ref(tile).ok_or(Error::InvalidRef)?;
        let polys = usize::try_from(t.header.poly_count).map_err(|_| Error::InvalidRef)?;
        polys
            .checked_mul(TILE_POLY_STATE_SIZE)
            .and_then(|n| n.checked_add(TILE_STATE_HEADER_SIZE))
            .ok_or(Error::SizeOverflow)
    }

    /// Writes the state of a resident tile into `out` (`dtNavMesh::storeTileState`)
    /// and returns the bytes written, [`NavMesh::tile_state_size`].
    ///
    /// The state is the non-structural, caller-editable data: every polygon's flags
    /// and area. It is bound to the tile's current ref, so it can be restored only
    /// into the same tile incarnation, not after the tile was removed and added
    /// again. The blob is little-endian; polygon entries are `u16` flags, `u8` area
    /// and a zero pad byte (upstream leaves the pad uninitialised).
    /// [`Error::BufferTooSmall`] when `out` is shorter than the state.
    pub fn store_tile_state(&self, tile: TileRef, out: &mut [u8]) -> Result<usize, Error> {
        let size = self.tile_state_size(tile)?;
        if out.len() < size {
            return Err(Error::BufferTooSmall);
        }
        let t = self.tile_by_ref(tile).ok_or(Error::InvalidRef)?;
        out[0..4].copy_from_slice(&NAVMESH_STATE_MAGIC.to_le_bytes());
        out[4..8].copy_from_slice(&NAVMESH_STATE_VERSION.to_le_bytes());
        out[8..TILE_STATE_HEADER_SIZE].copy_from_slice(&tile.to_le_bytes());
        for (p, s) in t
            .polys
            .iter()
            .zip(out[TILE_STATE_HEADER_SIZE..size].chunks_exact_mut(TILE_POLY_STATE_SIZE))
        {
            s[0..2].copy_from_slice(&p.flags.to_le_bytes());
            s[2] = p.area();
            s[3] = 0;
        }
        Ok(size)
    }

    /// Restores polygon flags and areas stored by [`NavMesh::store_tile_state`]
    /// (`dtNavMesh::restoreTileState`). The tile's ref and polygon refs do not
    /// change.
    ///
    /// The blob is untrusted: it is checked completely before the tile is touched.
    /// Errors, in upstream's order: [`Error::InvalidRef`] for a ref that is not a
    /// resident tile; [`Error::InvalidTileState`] with
    /// [`TileStateError::TooShort`](crate::TileStateError::TooShort) when `data` is shorter than the tile's state,
    /// `WrongMagic`, `WrongVersion`, or `WrongTile` when the stored ref differs from
    /// `tile`. Bytes after the state and the pad bytes are ignored. Area ids are
    /// masked to 6 bits, as `setPolyArea` does.
    pub fn restore_tile_state(&mut self, tile: TileRef, data: &[u8]) -> Result<(), Error> {
        let polys = self.tile_by_ref(tile).ok_or(Error::InvalidRef)?.polys.len();
        let size = stream::check_state(tile, polys, data)?;
        let (_, it, _) = self.decode_poly_id(tile);
        let t = self
            .slots
            .get_mut(it as usize)
            .and_then(|s| s.tile.as_deref_mut())
            .ok_or(Error::InvalidRef)?;
        stream::apply_state(t, data, size);
        Ok(())
    }

    /// Adds a tile from its bytes into the next free slot (`addTile` without a last
    /// ref). The bytes are validated after upstream's own checks, so upstream's
    /// verdicts on magic, version, ref bits, occupied locations and free slots are
    /// kept. All or nothing: on error the navmesh is unchanged.
    pub fn add_tile(&mut self, data: Vec<u8>) -> Result<TileRef, Error> {
        self.insert_tile(data, Placement::Free)
    }

    /// Adds a tile into the slot and salt of `tile_ref`, for example to rebuild a
    /// saved navmesh with the same refs. The slot must be free, and either have
    /// never held a tile (any salt is accepted) or `tile_ref` must carry the slot's
    /// current salt, so a ref to a removed tile can never become valid again.
    /// Errors: [`Error::InvalidRef`] for a null ref, a ref with bits outside the
    /// layout, salt 0 or a stale salt; [`Error::NoFreeTile`] when the slot is out
    /// of range or in use; otherwise as [`NavMesh::add_tile`].
    pub fn restore_tile(&mut self, data: Vec<u8>, tile_ref: TileRef) -> Result<TileRef, Error> {
        self.restore(data, tile_ref, false)
    }

    /// Upstream's `addTile` with a last ref: the tile goes into the slot of
    /// `tile_ref` and takes its salt whatever the slot's history. Unlike upstream, a
    /// null ref, a ref with bits outside the layout and salt 0 (which would make the
    /// slot's first polygon ref null) are rejected with [`Error::InvalidRef`].
    ///
    /// **This can resurrect stale refs**: restoring with the ref of a tile that was
    /// removed makes every ref to that removed tile valid again (now naming polygons
    /// of the new tile). Use [`NavMesh::restore_tile`] unless you need upstream's
    /// exact behaviour.
    pub fn restore_tile_allow_stale(
        &mut self,
        data: Vec<u8>,
        tile_ref: TileRef,
    ) -> Result<TileRef, Error> {
        self.restore(data, tile_ref, true)
    }

    fn restore(&mut self, data: Vec<u8>, tile_ref: TileRef, stale: bool) -> Result<TileRef, Error> {
        if tile_ref == 0 || !self.ref_in_layout(tile_ref) {
            return Err(Error::InvalidRef);
        }
        let (salt, idx, _) = self.decode_poly_id(tile_ref);
        if salt == 0 {
            return Err(Error::InvalidRef);
        }
        self.insert_tile(data, Placement::Restore { idx, salt, stale })
    }

    fn insert_tile(&mut self, data: Vec<u8>, place: Placement) -> Result<TileRef, Error> {
        let h = peek_header(&data)?;
        if PolyRef::BITS == 32
            && (h.poly_count < 0 || self.poly_bits < ilog2(next_pow2(h.poly_count as u32)))
        {
            return Err(Error::InvalidTileData(TileDataError::TooManyPolys));
        }
        if self.tile_index_at(h.x, h.y, h.layer).is_some() {
            return Err(Error::TileSlotOccupied);
        }
        if h.poly_count < 0 || h.poly_count as u64 > (1u64 << self.poly_bits) {
            return Err(Error::InvalidTileData(TileDataError::TooManyPolys));
        }
        // Find the slot (and its free-list predecessor) without changing anything, so
        // every check below can still fail with the navmesh untouched.
        let target = self.choose_slot(h.x, h.y, h.layer, place)?;
        let mut tile = MeshTile::parse_with_limits(data, &self.limits)?;
        tile.links_free_list = tile.reset_links();
        let hash = target.parked_hash.map(|_| stream::content_hash(&tile.data));
        self.link_in(tile, target, hash, false)
    }

    /// The slot a tile at `x, y, layer` goes into, without changing anything.
    fn choose_slot(&self, x: i32, y: i32, layer: i32, place: Placement) -> Result<Target, Error> {
        if let Some(ti) = self.stream.parked_slot(&self.slots, x, y, layer) {
            // The tile was unloaded: it goes back into its reserved slot.
            let s = &self.slots[ti];
            return match place {
                Placement::Free => Ok(Target {
                    ti,
                    prev: None,
                    salt: s.salt,
                    parked_hash: s.parked.as_ref().map(|p| p.hash),
                }),
                Placement::Restore { .. } => Err(Error::TileSlotOccupied),
            };
        }
        if let Some(keyed) = self.stream.keyed_slot(x, y, layer) {
            let ti = keyed.ok_or(Error::TileOutsideGrid)?;
            let s = &self.slots[ti];
            let salt = match place {
                Placement::Free => s.salt,
                Placement::Restore { idx, salt, stale } => {
                    if idx as usize != ti {
                        return Err(Error::NoFreeTile);
                    }
                    if !stale && s.used && s.salt != salt {
                        return Err(Error::InvalidRef);
                    }
                    salt
                }
            };
            return Ok(Target {
                ti,
                prev: None,
                salt,
                parked_hash: None,
            });
        }
        match place {
            Placement::Free => {
                if self.next_free < 0 {
                    return Err(Error::NoFreeTile);
                }
                let ti = self.next_free as usize;
                Ok(Target {
                    ti,
                    prev: Some(-1),
                    salt: self.slots[ti].salt,
                    parked_hash: None,
                })
            }
            Placement::Restore { idx, salt, stale } => {
                let target = idx as usize;
                let slot = self.slots.get(target).ok_or(Error::NoFreeTile)?;
                if slot.tile.is_some() {
                    return Err(Error::NoFreeTile);
                }
                let mut prev = -1i32;
                let mut cur = self.next_free;
                let mut left = self.slots.len();
                while cur >= 0 && cur as usize != target && left > 0 {
                    prev = cur;
                    cur = self.slots.get(cur as usize).map_or(-1, |s| s.next);
                    left -= 1;
                }
                if cur < 0 || cur as usize != target {
                    return Err(Error::NoFreeTile);
                }
                if !stale && slot.used && slot.salt != salt {
                    return Err(Error::InvalidRef);
                }
                Ok(Target {
                    ti: target,
                    prev: Some(prev),
                    salt,
                    parked_hash: None,
                })
            }
        }
    }

    /// Links a parsed tile into the slot `target` chose. `hash` is the tile's
    /// content hash, needed only when the slot was reserved for an unloaded tile:
    /// equal content keeps the salt (refs into the unloaded copy become valid
    /// again), other content increments it.
    fn link_in(
        &mut self,
        tile: MeshTile,
        target: Target,
        hash: Option<[u8; 32]>,
        internally_linked: bool,
    ) -> Result<TileRef, Error> {
        let ti = target.ti;
        let salt = if target.parked_hash.is_some_and(|old| hash != Some(old)) {
            self.next_salt(target.salt)?
        } else {
            target.salt
        };
        // Finish fallible preflight before the slot and free list change.
        self.slots.ensure(ti)?;
        let h = &tile.header;
        let (hx, hy, hl) = (h.x, h.y, h.layer);

        // Commit: nothing below fails.
        if let Some(prev) = target.prev {
            let nxt = self.slots[ti].next;
            if prev < 0 {
                self.next_free = nxt;
            } else {
                self.slots[prev as usize].next = nxt;
            }
        }
        if target.parked_hash.is_some() {
            self.stream.unpark(hx, hy, hl);
        }
        if !self.stream.is_keyed() {
            let hsh = tile_hash(hx, hy, self.lut_mask);
            self.slots[ti].next = self.pos_lookup[hsh];
            self.pos_lookup[hsh] = ti as i32;
        }
        let s = &mut self.slots[ti];
        s.salt = salt;
        s.parked = None;
        s.tile = Some(Box::new(tile));
        s.used = true;
        self.stream.bump_epoch();
        self.resident += 1;
        self.occupied = Some(match self.occupied {
            None => [hx, hy, hx, hy],
            Some([x0, y0, x1, y1]) => [x0.min(hx), y0.min(hy), x1.max(hx), y1.max(hy)],
        });

        let base = self.base_ref(ti);
        let tile = self.tile_mut(ti);
        if internally_linked {
            for p in &tile.polys {
                let mut i = p.first_link;
                while i != NULL_LINK {
                    let l = &mut tile.links[i as usize];
                    l.r#ref |= base;
                    i = l.next;
                }
            }
        } else {
            Self::prepare_internal_links(tile, base);
        }
        self.base_off_mesh_links(ti);
        self.connect_ext_off_mesh_links(ti, ti, -1);
        for j in self.tiles_at(hx, hy, 32) {
            if j == ti {
                continue;
            }
            self.connect_ext_links(ti, j, -1);
            self.connect_ext_links(j, ti, -1);
            self.connect_ext_off_mesh_links(ti, j, -1);
            self.connect_ext_off_mesh_links(j, ti, -1);
        }
        for i in 0..8 {
            for j in self.neighbour_tiles_at(hx, hy, i) {
                self.connect_ext_links(ti, j, i);
                self.connect_ext_links(j, ti, opposite_tile(i));
                self.connect_ext_off_mesh_links(ti, j, i);
                self.connect_ext_off_mesh_links(j, ti, opposite_tile(i));
            }
        }
        if self.global_off_mesh {
            self.rebuild_global_links();
        }
        Ok(self.tile_ref(ti))
    }

    /// Removes a tile and returns the bytes it was added from (`removeTile`). The
    /// slot's salt is incremented, so refs to the removed tile no longer validate
    /// (see the salt exhaustion limit on [`NavMesh`]). All or nothing: on error the
    /// navmesh is unchanged.
    pub fn remove_tile(&mut self, r: TileRef) -> Result<Vec<u8>, Error> {
        let ti = self.resident_slot(r)?;
        let next = self.next_salt(self.slots[ti].salt)?;
        let tile = self.detach(ti);
        self.release_slot(ti, next);
        Ok(tile.map(|t| t.data).unwrap_or_default())
    }

    /// The slot of the resident tile `r` names.
    fn resident_slot(&self, r: TileRef) -> Result<usize, Error> {
        if r == 0 {
            return Err(Error::InvalidParameter("null tile ref"));
        }
        if !self.ref_in_layout(r) {
            return Err(Error::InvalidRef);
        }
        let (salt, it, _) = self.decode_poly_id(r);
        match self.slots.get(it as usize) {
            Some(Slot {
                salt: s,
                tile: Some(_),
                ..
            }) if *s == salt => Ok(it as usize),
            _ => Err(Error::InvalidRef),
        }
    }

    /// The next salt, refusing reuse before any mutation.
    fn next_salt(&self, salt: u32) -> Result<u32, Error> {
        let mask = ((1u64 << self.salt_bits) - 1) as u32;
        if salt >= mask {
            Err(Error::SaltExhausted)
        } else {
            Ok(salt + 1)
        }
    }

    /// Frees slot `ti` (no tile, not reserved) with salt `salt`: back on the free
    /// list of a sequential navmesh. Logs the removal.
    fn release_slot(&mut self, ti: usize, salt: u32) {
        let keyed = self.stream.is_keyed();
        let s = &mut self.slots[ti];
        s.salt = salt;
        s.parked = None;
        if keyed {
            s.next = -1;
        } else {
            s.next = self.next_free;
            self.next_free = ti as i32;
        }
        self.stream.log_removal(ti as u32);
    }

    /// Takes the tile out of slot `ti`: lookup list, links from its neighbours and
    /// global pairs. The slot's salt, free-list state and reservation are left to
    /// the caller.
    fn detach(&mut self, ti: usize) -> Option<Box<MeshTile>> {
        let (x, y) = self.tile(ti).map(|t| (t.header.x, t.header.y))?;
        if !self.stream.is_keyed() {
            self.unlink_lookup(ti, x, y);
        }
        for j in self.tiles_at(x, y, 32) {
            if j != ti {
                self.unconnect_links(j, ti);
            }
        }
        for i in 0..8 {
            for j in self.neighbour_tiles_at(x, y, i) {
                self.unconnect_links(j, ti);
            }
        }
        if self.global_off_mesh {
            self.disconnect_global_pairs(ti);
        }
        let tile = self.slots[ti].tile.take();
        self.resident = self.resident.saturating_sub(1);
        if self.global_off_mesh {
            self.rebuild_global_links();
        }
        if self.resident == 0 {
            self.occupied = None;
        }
        tile
    }

    fn unlink_lookup(&mut self, ti: usize, x: i32, y: i32) {
        let hsh = tile_hash(x, y, self.lut_mask);
        let mut prev = -1i32;
        let mut cur = self.pos_lookup[hsh];
        let mut left = self.slots.len();
        while cur >= 0 && left > 0 {
            if cur as usize == ti {
                let nxt = self.slots[ti].next;
                if prev < 0 {
                    self.pos_lookup[hsh] = nxt;
                } else {
                    self.slots[prev as usize].next = nxt;
                }
                break;
            }
            prev = cur;
            cur = self.slots.get(cur as usize).map_or(-1, |s| s.next);
            left -= 1;
        }
    }

    fn tile_mut(&mut self, i: usize) -> &mut MeshTile {
        self.slots[i].tile.get_or_insert_with(Default::default)
    }

    pub(super) fn prepare_internal_links(tile: &mut MeshTile, base: PolyRef) {
        for i in 0..tile.polys.len() {
            tile.polys[i].first_link = NULL_LINK;
            if tile.polys[i].poly_type() == POLYTYPE_OFFMESH_CONNECTION {
                continue;
            }
            for j in (0..tile.polys[i].vert_count as usize).rev() {
                let nei = tile.polys[i].neis[j];
                if nei == 0 || nei & EXT_LINK != 0 {
                    continue;
                }
                let idx = alloc_link(tile);
                if idx != NULL_LINK {
                    let first = tile.polys[i].first_link;
                    let l = &mut tile.links[idx as usize];
                    l.r#ref = base | (nei - 1) as PolyRef;
                    l.edge = j as u8;
                    l.side = 0xff;
                    l.bmin = 0;
                    l.bmax = 0;
                    l.next = first;
                    tile.polys[i].first_link = idx;
                }
            }
        }
    }

    fn base_off_mesh_links(&mut self, ti: usize) {
        let base = self.base_ref(ti);
        let tile = self.tile_mut(ti);
        for i in 0..tile.off_mesh_cons.len() {
            let con = tile.off_mesh_cons[i];
            let half = [con.rad, tile.header.walkable_climb, con.rad];
            let p = &con.pos[0..3];
            let Some((ip, nearest)) = find_nearest_poly_in_tile(tile, p, &half) else {
                continue;
            };
            if sqr(nearest[0] - p[0]) + sqr(nearest[2] - p[2]) > sqr(con.rad) {
                continue;
            }
            let cp = con.poly as usize;
            let v = tile.polys[cp].verts[0] as usize * 3;
            tile.verts[v..v + 3].copy_from_slice(&nearest);
            let idx = alloc_link(tile);
            if idx != NULL_LINK {
                let first = tile.polys[cp].first_link;
                let l = &mut tile.links[idx as usize];
                l.r#ref = base | ip as PolyRef;
                l.edge = 0;
                l.side = 0xff;
                l.bmin = 0;
                l.bmax = 0;
                l.next = first;
                tile.polys[cp].first_link = idx;
            }
            let tidx = alloc_link(tile);
            if tidx != NULL_LINK {
                let land = ip as u16 as usize;
                let first = tile.polys[land].first_link;
                let l = &mut tile.links[tidx as usize];
                l.r#ref = base | con.poly as PolyRef;
                l.edge = 0xff;
                l.side = 0xff;
                l.bmin = 0;
                l.bmax = 0;
                l.next = first;
                tile.polys[land].first_link = tidx;
            }
        }
    }

    fn find_connecting_polys(
        &self,
        va: &[f32],
        vb: &[f32],
        ti: usize,
        side: i32,
        out: &mut Vec<(PolyRef, f32, f32)>,
    ) {
        out.clear();
        let Some(tile) = self.slots[ti].tile.as_ref() else {
            return;
        };
        let (amin, amax) = slab_end_points(va, vb, side);
        let apos = slab_coord(va, side);
        let m = EXT_LINK | side as u16;
        let base = self.base_ref(ti);
        for (i, poly) in tile.polys.iter().enumerate() {
            let nv = poly.vert_count as usize;
            for j in 0..nv {
                if poly.neis[j] != m {
                    continue;
                }
                let vc = &tile.verts[poly.verts[j] as usize * 3..];
                let vd = &tile.verts[poly.verts[(j + 1) % nv] as usize * 3..];
                let bpos = slab_coord(vc, side);
                if (apos - bpos).abs() > 0.01 {
                    continue;
                }
                let (bmin, bmax) = slab_end_points(vc, vd, side);
                if !overlap_slabs(&amin, &amax, &bmin, &bmax, 0.01, tile.header.walkable_climb) {
                    continue;
                }
                if out.len() < 4 {
                    out.push((
                        base | i as PolyRef,
                        fmax(amin[0], bmin[0]),
                        fmin(amax[0], bmax[0]),
                    ));
                }
                break;
            }
        }
    }

    fn connect_ext_links(&mut self, ti: usize, target: usize, side: i32) {
        let mut nei = Vec::new();
        let npolys = self.slots[ti].tile.as_ref().map_or(0, |t| t.polys.len());
        for i in 0..npolys {
            let tile = self.tile_unchecked(ti as u32);
            let poly = tile.polys[i];
            let nv = poly.vert_count as usize;
            for j in 0..nv {
                if poly.neis[j] & EXT_LINK == 0 {
                    continue;
                }
                let dir = (poly.neis[j] & 0xff) as i32;
                if side != -1 && dir != side {
                    continue;
                }
                let tile = self.tile_unchecked(ti as u32);
                let va = v3(&tile.verts[poly.verts[j] as usize * 3..]);
                let vb = v3(&tile.verts[poly.verts[(j + 1) % nv] as usize * 3..]);
                self.find_connecting_polys(&va, &vb, target, opposite_tile(dir), &mut nei);
                let tile = self.tile_mut(ti);
                for &(r, cmin, cmax) in &nei {
                    let idx = alloc_link(tile);
                    if idx == NULL_LINK {
                        continue;
                    }
                    let first = tile.polys[i].first_link;
                    let l = &mut tile.links[idx as usize];
                    l.r#ref = r;
                    l.edge = j as u8;
                    l.side = dir as u8;
                    l.next = first;
                    let k = if dir == 0 || dir == 4 {
                        Some(2)
                    } else if dir == 2 || dir == 6 {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(k) = k {
                        let mut tmin = (cmin - va[k]) / (vb[k] - va[k]);
                        let mut tmax = (cmax - va[k]) / (vb[k] - va[k]);
                        if tmin > tmax {
                            core::mem::swap(&mut tmin, &mut tmax);
                        }
                        l.bmin = round_unit(fclamp(tmin, 0.0, 1.0) * 255.0);
                        l.bmax = round_unit(fclamp(tmax, 0.0, 1.0) * 255.0);
                    }
                    tile.polys[i].first_link = idx;
                }
            }
        }
    }

    fn connect_ext_off_mesh_links(&mut self, ti: usize, target: usize, side: i32) {
        self.link_off_mesh(ti, target, side, false);
    }

    /// Links the off-mesh connections of `target` whose far endpoint lies outside its
    /// bounds to the polygons of `ti`, wherever `ti` is in the grid (global links,
    /// not in recastnavigation). Pairs upstream already resolves are left to it.
    fn connect_global_off_mesh_links(&mut self, ti: usize, target: usize) {
        self.link_off_mesh(ti, target, -1, true);
    }

    /// Whether upstream resolves the far endpoint of a connection of `owner` with
    /// `con_side` in tile `landing`: the neighbour tile in direction `con_side`.
    fn upstream_resolves(&self, owner: usize, con_side: u8, landing: usize) -> bool {
        let Some(t) = self.slots[owner].tile.as_ref() else {
            return false;
        };
        self.neighbour_tiles_at(t.header.x, t.header.y, i32::from(con_side))
            .contains(&landing)
    }

    fn link_off_mesh(&mut self, ti: usize, target: usize, side: i32, global: bool) {
        let opposite = if side == -1 {
            0xff
        } else {
            opposite_tile(side) as u8
        };
        let ncons = self.slots[target]
            .tile
            .as_ref()
            .map_or(0, |t| t.off_mesh_cons.len());
        let target_base = self.base_ref(target);
        let tile_base = self.base_ref(ti);
        for i in 0..ncons {
            let tt = self.tile_unchecked(target as u32);
            let con = tt.off_mesh_cons[i];
            if global {
                if con.side == 0xff || ti == target || self.upstream_resolves(target, con.side, ti)
                {
                    continue;
                }
            } else if con.side != opposite {
                continue;
            }
            let cp = con.poly as usize;
            if tt.polys[cp].first_link == NULL_LINK {
                continue;
            }
            let half = [con.rad, tt.header.walkable_climb, con.rad];
            let p = &con.pos[3..6];
            let Some((ip, nearest)) =
                find_nearest_poly_in_tile(self.tile_unchecked(ti as u32), p, &half)
            else {
                continue;
            };
            if sqr(nearest[0] - p[0]) + sqr(nearest[2] - p[2]) > sqr(con.rad) {
                continue;
            }
            let tt = self.tile_mut(target);
            let v = tt.polys[cp].verts[1] as usize * 3;
            tt.verts[v..v + 3].copy_from_slice(&nearest);
            let idx = alloc_link(tt);
            if idx != NULL_LINK {
                let first = tt.polys[cp].first_link;
                let l = &mut tt.links[idx as usize];
                l.r#ref = tile_base | ip as PolyRef;
                l.edge = 1;
                l.side = if global { 0xff } else { opposite };
                l.bmin = 0;
                l.bmax = 0;
                l.next = first;
                tt.polys[cp].first_link = idx;
            }
            if con.flags & OFFMESH_CON_BIDIR != 0 {
                let tile = self.tile_mut(ti);
                let tidx = alloc_link(tile);
                if tidx != NULL_LINK {
                    let land = ip as u16 as usize;
                    let first = tile.polys[land].first_link;
                    let l = &mut tile.links[tidx as usize];
                    l.r#ref = target_base | con.poly as PolyRef;
                    l.edge = 0xff;
                    l.side = if global || side == -1 {
                        0xff
                    } else {
                        side as u8
                    };
                    l.bmin = 0;
                    l.bmax = 0;
                    l.next = first;
                    tile.polys[land].first_link = tidx;
                }
            }
        }
    }

    fn has_off_mesh_cons(&self, ti: usize) -> bool {
        self.slots[ti]
            .tile
            .as_ref()
            .is_some_and(|t| !t.off_mesh_cons.is_empty())
    }

    /// Rebuild global pairs in lexicographic tile-key order. Prepending links
    /// makes the greatest landing key the snapped endpoint and first link.
    fn rebuild_global_links(&mut self) {
        let mut resident = self.resident_slots();
        resident.sort_unstable_by_key(|&i| {
            let h = &self.tile_unchecked(i as u32).header;
            (h.x, h.y, h.layer)
        });
        for &owner in &resident {
            if self.has_off_mesh_cons(owner) {
                for &landing in &resident {
                    if landing != owner {
                        self.disconnect_global_pair(owner, landing);
                    }
                }
            }
        }
        self.reset_global_endpoints(&resident);
        for &owner in &resident {
            if self.has_off_mesh_cons(owner) {
                for &landing in &resident {
                    if landing != owner {
                        self.connect_global_off_mesh_links(landing, owner);
                    }
                }
            }
        }
    }

    /// Restore authored or upstream-only endpoints before global candidates are
    /// replayed, including when the final global landing has just disappeared.
    fn reset_global_endpoints(&mut self, resident: &[usize]) {
        for &owner in resident {
            let count = self.tile_unchecked(owner as u32).off_mesh_cons.len();
            for i in 0..count {
                let t = self.tile_unchecked(owner as u32);
                let con = t.off_mesh_cons[i];
                if con.side == 0xff {
                    continue;
                }
                let p = [con.pos[3], con.pos[4], con.pos[5]];
                let half = [con.rad, t.header.walkable_climb, con.rad];
                let mut endpoint = p;
                for &landing in resident {
                    if self.upstream_resolves(owner, con.side, landing)
                        && let Some((_, nearest)) = find_nearest_poly_in_tile(
                            self.tile_unchecked(landing as u32),
                            &p,
                            &half,
                        )
                        && sqr(nearest[0] - p[0]) + sqr(nearest[2] - p[2]) <= sqr(con.rad)
                    {
                        endpoint = nearest;
                    }
                }
                let t = self.tile_mut(owner);
                let v = t.polys[con.poly as usize].verts[1] as usize * 3;
                t.verts[v..v + 3].copy_from_slice(&endpoint);
            }
        }
    }

    /// Removes the global links between `ti` and every other tile, in both roles.
    fn disconnect_global_pairs(&mut self, ti: usize) {
        for j in self.resident_slots() {
            if j != ti {
                self.disconnect_global_pair(j, ti);
                self.disconnect_global_pair(ti, j);
            }
        }
    }

    /// Allocated slot candidates, including parked/free slots; sparse keyed grids
    /// visit allocated pages only. The observer charges each candidate itself.
    pub(crate) fn allocated_slots(&self) -> impl Iterator<Item = usize> + '_ {
        self.slots.indices()
    }

    /// Slots holding a tile, ascending.
    pub(crate) fn resident_slots(&self) -> Vec<usize> {
        self.slots
            .indices()
            .filter(|&i| self.slots[i].tile.is_some())
            .collect()
    }

    /// Removes the global links of `owner`'s connections into `landing`: the links
    /// from the connection polygons and the return links of bidirectional ones.
    fn disconnect_global_pair(&mut self, owner: usize, landing: usize) {
        let owner_base = self.base_ref(owner);
        let landing_num = landing as u32;
        let (pb, tb) = (self.poly_bits, self.tile_bits);
        let decode_tile = move |r: PolyRef| ((r >> pb) & (((1u64 << tb) - 1) as PolyRef)) as u32;
        let cons: Vec<(u8, u16)> = self.slots[owner].tile.as_ref().map_or(Vec::new(), |t| {
            t.off_mesh_cons.iter().map(|c| (c.side, c.poly)).collect()
        });
        for (con_side, poly) in cons {
            if con_side == 0xff || self.upstream_resolves(owner, con_side, landing) {
                continue;
            }
            let con_ref = owner_base | PolyRef::from(poly);
            let tile = self.tile_mut(owner);
            remove_links_where(tile, Some(poly as usize), |r| decode_tile(r) == landing_num);
            let tile = self.tile_mut(landing);
            remove_links_where(tile, None, |r| r == con_ref);
        }
    }

    /// Whether off-mesh connections resolve their far endpoint in any loaded tile
    /// (see [`NavMesh::set_global_off_mesh_links`]).
    pub fn global_off_mesh_links(&self) -> bool {
        self.global_off_mesh
    }

    /// Opt-in extension (not in recastnavigation): lets an off-mesh connection
    /// resolve its far endpoint in **any** loaded tile.
    ///
    /// Upstream links a connection whose far endpoint lies outside its tile only into
    /// the neighbour tile in the direction of that endpoint, so a connection spanning
    /// several tiles (a teleporter, an elevator) never gets a landing polygon. With
    /// this on, such a connection is also resolved in every other tile, in whatever
    /// order the tiles are loaded: adding a tile links it to the connections of all
    /// resident tiles and the connections it owns to all resident tiles, and removing
    /// a tile drops exactly those links, so removing and re-adding either tile
    /// restores the path. Connections whose far endpoint lies inside their own tile
    /// and the pairs upstream resolves are untouched, so the default (off) and every
    /// pair upstream handles are identical to upstream.
    ///
    /// Global pairs are rebuilt in ascending `(x, y, layer)` order after each
    /// commit or removal. Links are prepended, so the greatest eligible landing
    /// key supplies the snapped endpoint and first outgoing global link. This
    /// rule is independent of commit order, including multiple landing layers.
    ///
    /// Endpoint snapping is upstream's: the far endpoint is moved to the nearest point
    /// on the nearest polygon of the candidate tile, which is found within the
    /// connection radius horizontally (a circle test on the distance to the snapped
    /// point) and the tile's walkable climb vertically (the search box is
    /// `[radius, walkable_climb, radius]` around the authored endpoint), both measured
    /// from the endpoint as authored. Turning this on or off relinks the resident
    /// tiles; cost is proportional to the tiles with off-mesh connections times the
    /// resident tiles.
    pub fn set_global_off_mesh_links(&mut self, enabled: bool) {
        if enabled == self.global_off_mesh {
            return;
        }
        let resident = self.resident_slots();
        if enabled {
            self.global_off_mesh = true;
            self.rebuild_global_links();
        } else {
            for &owner in &resident {
                for &landing in &resident {
                    if landing != owner {
                        self.disconnect_global_pair(owner, landing);
                    }
                }
            }
            self.reset_global_endpoints(&resident);
            self.global_off_mesh = false;
        }
    }

    fn unconnect_links(&mut self, ti: usize, target: usize) {
        let target_num = target as u32;
        let (pb, tb) = (self.poly_bits, self.tile_bits);
        let tile = self.tile_mut(ti);
        let decode_tile = |r: PolyRef| ((r >> pb) & (((1u64 << tb) - 1) as PolyRef)) as u32;
        for i in 0..tile.polys.len() {
            let mut j = tile.polys[i].first_link;
            let mut pj = NULL_LINK;
            while j != NULL_LINK {
                if decode_tile(tile.links[j as usize].r#ref) == target_num {
                    let nj = tile.links[j as usize].next;
                    if pj == NULL_LINK {
                        tile.polys[i].first_link = nj;
                    } else {
                        tile.links[pj as usize].next = nj;
                    }
                    free_link(tile, j);
                    j = nj;
                } else {
                    pj = j;
                    j = tile.links[j as usize].next;
                }
            }
        }
    }
}

/// Frees the links of `tile`'s polygons (only `poly` when given) whose target
/// satisfies `drop`.
fn remove_links_where(tile: &mut MeshTile, poly: Option<usize>, drop: impl Fn(PolyRef) -> bool) {
    let range = match poly {
        Some(p) if p < tile.polys.len() => p..p + 1,
        Some(_) => return,
        None => 0..tile.polys.len(),
    };
    for i in range {
        let mut j = tile.polys[i].first_link;
        let mut pj = NULL_LINK;
        while j != NULL_LINK {
            if drop(tile.links[j as usize].r#ref) {
                let nj = tile.links[j as usize].next;
                if pj == NULL_LINK {
                    tile.polys[i].first_link = nj;
                } else {
                    tile.links[pj as usize].next = nj;
                }
                free_link(tile, j);
                j = nj;
            } else {
                pj = j;
                j = tile.links[j as usize].next;
            }
        }
    }
}

static EMPTY_TILE: MeshTile = MeshTile {
    header: super::tile::EMPTY_HEADER,
    verts: Vec::new(),
    polys: Vec::new(),
    links: Vec::new(),
    detail_meshes: Vec::new(),
    detail_verts: Vec::new(),
    detail_tris: Vec::new(),
    bv_tree: Vec::new(),
    off_mesh_cons: Vec::new(),
    data: Vec::new(),
    links_free_list: NULL_LINK,
};
