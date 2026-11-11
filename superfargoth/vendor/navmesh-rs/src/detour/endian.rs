//! Byte-order conversion of tile bytes (`dtNavMeshHeaderSwapEndian`,
//! `dtNavMeshDataSwapEndian`).
//!
//! The crate reads and writes tile bytes little-endian on every platform. Upstream's
//! "native" byte order is the host's, so on a little-endian host these functions
//! convert between the crate's tiles and big-endian tiles; they never depend on the
//! host of the crate itself. Both functions swap in place and are their own inverse
//! on a valid tile, so the same call converts either way.

use super::tile::{
    BV_NODE_SIZE, HEADER_SIZE, OFF_MESH_CON_SIZE, POLY_DETAIL_SIZE, POLY_SIZE, Sections,
    peek_header,
};
use super::{NAVMESH_MAGIC, NAVMESH_VERSION};
use crate::{Error, TileDataError};

fn le_i32(data: &[u8], at: usize) -> i32 {
    i32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]])
}

fn swap_u16s(data: &mut [u8], at: usize, count: usize) {
    for i in 0..count {
        data.swap(at + i * 2, at + i * 2 + 1);
    }
}

fn swap_u32s(data: &mut [u8], at: usize, count: usize) {
    for i in 0..count {
        let b = at + i * 4;
        data.swap(b, b + 3);
        data.swap(b + 1, b + 2);
    }
}

/// Swaps the byte order of the tile header in place (`dtNavMeshHeaderSwapEndian`).
///
/// Accepts a header whose magic and version are both correct in one byte order or
/// both correct in the other, then reverses the bytes of all 25 four-byte header
/// fields. [`Error::InvalidTileData`] with `WrongMagic` or `WrongVersion` when
/// neither holds, and `BadSize` when `data` is shorter than the 100-byte header.
/// The body is not touched: use [`nav_mesh_data_swap_endian`] for it.
///
/// To read a tile of the other byte order, call this first and the data swap second;
/// to write one, call the data swap first and this second.
pub fn nav_mesh_header_swap_endian(data: &mut [u8]) -> Result<(), Error> {
    if data.len() < HEADER_SIZE {
        return Err(Error::InvalidTileData(TileDataError::BadSize));
    }
    let (magic, version) = (le_i32(data, 0), le_i32(data, 4));
    let native = magic == NAVMESH_MAGIC && version == NAVMESH_VERSION;
    let swapped = magic == NAVMESH_MAGIC.swap_bytes() && version == NAVMESH_VERSION.swap_bytes();
    if !native && !swapped {
        let magic_ok = magic == NAVMESH_MAGIC || magic == NAVMESH_MAGIC.swap_bytes();
        return Err(Error::InvalidTileData(if magic_ok {
            TileDataError::WrongVersion
        } else {
            TileDataError::WrongMagic
        }));
    }
    swap_u32s(data, 0, HEADER_SIZE / 4);
    Ok(())
}

/// Finds the section offsets of a tile whose header is in the crate's byte order,
/// checking that every section lies inside `data`.
fn sections(data: &[u8]) -> Result<[usize; 8], Error> {
    let h = peek_header(data)?;
    let s = Sections::of(&h).ok_or(Error::InvalidTileData(TileDataError::BadSize))?;
    let total = s
        .total()
        .ok_or(Error::InvalidTileData(TileDataError::BadSize))?;
    if total > data.len() {
        return Err(Error::InvalidTileData(TileDataError::BadSize));
    }
    // Section start offsets in layout order: vertices, polygons, links, detail
    // meshes, detail vertices, detail triangles, BV tree, off-mesh connections.
    let sizes = [
        s.verts,
        s.polys,
        s.links,
        s.detail_meshes,
        s.detail_verts,
        s.detail_tris,
        s.bv_tree,
        s.off_mesh_cons,
    ];
    let mut at = HEADER_SIZE;
    let mut out = [0usize; 8];
    for (o, size) in out.iter_mut().zip(sizes) {
        *o = at;
        at += size;
    }
    Ok(out)
}

/// Swaps the byte order of the tile body in place (`dtNavMeshDataSwapEndian`):
/// vertices, polygon vertex and neighbour indices and flags, detail sub-mesh
/// bases, detail vertices, BV-tree nodes, and the positions, radii and polygon
/// indices of off-mesh connections.
///
/// The header must already be in the crate's byte order (little-endian), as
/// upstream requires of the host order: [`Error::InvalidTileData`] with
/// `WrongMagic` or `WrongVersion` otherwise. Unlike upstream, which trusts the
/// counts, `BadSize` is returned when a count is negative or a section does not fit
/// `data`, before anything is changed.
///
/// Like upstream this leaves the link pool, polygon `first_link` fields, detail
/// triangles (single bytes) and off-mesh `user_id` untouched. Links and `first_link`
/// are rebuilt when a tile is added. The `user_id` omission is an upstream defect: a
/// tile converted from another byte order keeps its off-mesh `user_id` in the old
/// order; [`nav_mesh_swap_off_mesh_user_ids`] converts those.
pub fn nav_mesh_data_swap_endian(data: &mut [u8]) -> Result<(), Error> {
    let [
        verts,
        polys,
        _links,
        detail_meshes,
        detail_verts,
        _tris,
        bv_tree,
        off_mesh,
    ] = sections(data)?;
    let h = peek_header(data)?;
    // `sections` accepted the counts, so they are non-negative and in bounds.
    let n = |c: i32| c as usize;
    swap_u32s(data, verts, n(h.vert_count) * 3);
    for i in 0..n(h.poly_count) {
        let p = polys + i * POLY_SIZE;
        // `first_link` (4 bytes) is rebuilt on load; verts[6], neis[6], flags follow.
        swap_u16s(data, p + 4, 6 + 6 + 1);
    }
    for i in 0..n(h.detail_mesh_count) {
        swap_u32s(data, detail_meshes + i * POLY_DETAIL_SIZE, 2);
    }
    swap_u32s(data, detail_verts, n(h.detail_vert_count) * 3);
    for i in 0..n(h.bv_node_count) {
        let b = bv_tree + i * BV_NODE_SIZE;
        swap_u16s(data, b, 6);
        swap_u32s(data, b + 12, 1);
    }
    for i in 0..n(h.off_mesh_con_count) {
        let c = off_mesh + i * OFF_MESH_CON_SIZE;
        swap_u32s(data, c, 7);
        swap_u16s(data, c + 28, 1);
    }
    Ok(())
}

/// Swaps the byte order of every off-mesh connection's `user_id` in place (not in
/// recastnavigation).
///
/// [`nav_mesh_data_swap_endian`] follows upstream and skips this field. Call this
/// next to it, with the same header precondition and the same errors, so that a
/// converted tile keeps its off-mesh user ids.
pub fn nav_mesh_swap_off_mesh_user_ids(data: &mut [u8]) -> Result<(), Error> {
    let off_mesh = sections(data)?[7];
    let count = peek_header(data)?.off_mesh_con_count as usize;
    for i in 0..count {
        swap_u32s(data, off_mesh + i * OFF_MESH_CON_SIZE + 32, 1);
    }
    Ok(())
}
