//! Tile data: the `DNAV` v7 layout, its in-memory form and the validating reader.

use alloc::vec::Vec;

use super::math::align4;
use super::{
    EXT_LINK, NAVMESH_MAGIC, NAVMESH_VERSION, NULL_LINK, POLYTYPE_OFFMESH_CONNECTION, PolyRef,
    VERTS_PER_POLYGON,
};
use crate::{Error, Limits, TileDataError};

pub(crate) const HEADER_SIZE: usize = 100;
pub(crate) const POLY_SIZE: usize = 32;
pub(crate) const LINK_SIZE: usize = if PolyRef::BITS == 64 { 16 } else { 12 };
pub(crate) const POLY_DETAIL_SIZE: usize = 12;
pub(crate) const BV_NODE_SIZE: usize = 16;
pub(crate) const OFF_MESH_CON_SIZE: usize = 36;

/// Tile header (`dtMeshHeader`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
pub struct MeshHeader {
    pub magic: i32,
    pub version: i32,
    pub x: i32,
    pub y: i32,
    pub layer: i32,
    pub user_id: u32,
    pub poly_count: i32,
    pub vert_count: i32,
    pub max_link_count: i32,
    pub detail_mesh_count: i32,
    pub detail_vert_count: i32,
    pub detail_tri_count: i32,
    pub bv_node_count: i32,
    pub off_mesh_con_count: i32,
    pub off_mesh_base: i32,
    pub walkable_height: f32,
    pub walkable_radius: f32,
    pub walkable_climb: f32,
    pub bmin: [f32; 3],
    pub bmax: [f32; 3],
    pub bv_quant_factor: f32,
}

/// A polygon (`dtPoly`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Poly {
    /// First link in the tile's link list, or [`NULL_LINK`].
    pub first_link: u32,
    /// Vertex indices into the tile's vertices.
    pub verts: [u16; VERTS_PER_POLYGON],
    /// Neighbour per edge: 0 none, `index + 1` inside the tile, or
    /// [`EXT_LINK`]` | side` on a tile border.
    pub neis: [u16; VERTS_PER_POLYGON],
    /// User flags (filters test them).
    pub flags: u16,
    /// Number of vertices.
    pub vert_count: u8,
    /// Area id (low 6 bits) and type (high 2 bits).
    pub area_and_type: u8,
}

impl Poly {
    /// Sets the area id.
    pub fn set_area(&mut self, a: u8) {
        self.area_and_type = (self.area_and_type & 0xc0) | (a & 0x3f);
    }
    /// Sets the polygon type.
    pub fn set_type(&mut self, t: u8) {
        self.area_and_type = (self.area_and_type & 0x3f) | (t << 6);
    }
    /// Area id.
    pub fn area(&self) -> u8 {
        self.area_and_type & 0x3f
    }
    /// Polygon type ([`super::POLYTYPE_GROUND`] or [`POLYTYPE_OFFMESH_CONNECTION`]).
    pub fn poly_type(&self) -> u8 {
        self.area_and_type >> 6
    }
}

/// Detail sub-mesh of a polygon (`dtPolyDetail`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct PolyDetail {
    pub vert_base: u32,
    pub tri_base: u32,
    pub vert_count: u8,
    pub tri_count: u8,
}

/// A link between polygons (`dtLink`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Link {
    pub r#ref: PolyRef,
    pub next: u32,
    pub edge: u8,
    pub side: u8,
    pub bmin: u8,
    pub bmax: u8,
}

/// Bounding-volume tree node (`dtBVNode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct BvNode {
    pub bmin: [u16; 3],
    pub bmax: [u16; 3],
    pub i: i32,
}

/// Off-mesh connection (`dtOffMeshConnection`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
pub struct OffMeshConnection {
    pub pos: [f32; 6],
    pub rad: f32,
    pub poly: u16,
    pub flags: u8,
    pub side: u8,
    pub user_id: u32,
}

/// A tile resident in a navmesh (`dtMeshTile`), decoded from validated bytes.
#[derive(Clone, Debug, Default)]
pub struct MeshTile {
    /// Header.
    pub header: MeshHeader,
    /// Vertices, 3 floats each.
    pub verts: Vec<f32>,
    /// Polygons.
    pub polys: Vec<Poly>,
    /// Link pool.
    pub links: Vec<Link>,
    /// Detail sub-meshes.
    pub detail_meshes: Vec<PolyDetail>,
    /// Detail vertices, 3 floats each.
    pub detail_verts: Vec<f32>,
    /// Detail triangles, 4 bytes each (3 indices and edge flags).
    pub detail_tris: Vec<u8>,
    /// Bounding-volume tree (empty when the tile has none).
    pub bv_tree: Vec<BvNode>,
    /// Off-mesh connections.
    pub off_mesh_cons: Vec<OffMeshConnection>,
    /// The bytes the tile was created from.
    pub data: Vec<u8>,
    pub(crate) links_free_list: u32,
}

/// A polygon without vertices or links (stands in for an out-of-range index).
pub(crate) static EMPTY_POLY: Poly = Poly {
    first_link: NULL_LINK,
    verts: [0; VERTS_PER_POLYGON],
    neis: [0; VERTS_PER_POLYGON],
    flags: 0,
    vert_count: 0,
    area_and_type: 0,
};

pub(crate) const EMPTY_HEADER: MeshHeader = MeshHeader {
    magic: 0,
    version: 0,
    x: 0,
    y: 0,
    layer: 0,
    user_id: 0,
    poly_count: 0,
    vert_count: 0,
    max_link_count: 0,
    detail_mesh_count: 0,
    detail_vert_count: 0,
    detail_tri_count: 0,
    bv_node_count: 0,
    off_mesh_con_count: 0,
    off_mesh_base: 0,
    walkable_height: 0.0,
    walkable_radius: 0.0,
    walkable_climb: 0.0,
    bmin: [0.0; 3],
    bmax: [0.0; 3],
    bv_quant_factor: 0.0,
};

/// Section sizes of a tile, in bytes, in layout order.
pub(crate) struct Sections {
    pub verts: usize,
    pub polys: usize,
    pub links: usize,
    pub detail_meshes: usize,
    pub detail_verts: usize,
    pub detail_tris: usize,
    pub bv_tree: usize,
    pub off_mesh_cons: usize,
}

impl Sections {
    pub(crate) fn of(h: &MeshHeader) -> Option<Sections> {
        let c = |n: i32, size: usize| -> Option<usize> {
            let n = usize::try_from(n).ok()?;
            Some(align4(n.checked_mul(size)?))
        };
        Some(Sections {
            verts: c(h.vert_count, 12)?,
            polys: c(h.poly_count, POLY_SIZE)?,
            links: c(h.max_link_count, LINK_SIZE)?,
            detail_meshes: c(h.detail_mesh_count, POLY_DETAIL_SIZE)?,
            detail_verts: c(h.detail_vert_count, 12)?,
            detail_tris: c(h.detail_tri_count, 4)?,
            bv_tree: c(h.bv_node_count, BV_NODE_SIZE)?,
            off_mesh_cons: c(h.off_mesh_con_count, OFF_MESH_CON_SIZE)?,
        })
    }

    pub(crate) fn total(&self) -> Option<usize> {
        [
            self.verts,
            self.polys,
            self.links,
            self.detail_meshes,
            self.detail_verts,
            self.detail_tris,
            self.bv_tree,
            self.off_mesh_cons,
        ]
        .iter()
        .try_fold(HEADER_SIZE, |a, &b| a.checked_add(b))
    }
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u8(&mut self) -> u8 {
        let v = self.b[self.at];
        self.at += 1;
        v
    }
    fn u16(&mut self) -> u16 {
        let v = u16::from_le_bytes([self.b[self.at], self.b[self.at + 1]]);
        self.at += 2;
        v
    }
    fn u32(&mut self) -> u32 {
        let s = &self.b[self.at..self.at + 4];
        self.at += 4;
        u32::from_le_bytes([s[0], s[1], s[2], s[3]])
    }
    fn i32(&mut self) -> i32 {
        self.u32() as i32
    }
    fn f32(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }
    fn v3(&mut self) -> [f32; 3] {
        [self.f32(), self.f32(), self.f32()]
    }
}

/// Reads magic and version only (upstream checks these before anything else).
pub(crate) fn peek_header(data: &[u8]) -> Result<MeshHeader, Error> {
    if data.len() < HEADER_SIZE {
        // Upstream reads the header unconditionally; a short buffer cannot hold one.
        return Err(Error::InvalidTileData(TileDataError::BadSize));
    }
    let mut r = Reader { b: data, at: 0 };
    let h = MeshHeader {
        magic: r.i32(),
        version: r.i32(),
        x: r.i32(),
        y: r.i32(),
        layer: r.i32(),
        user_id: r.u32(),
        poly_count: r.i32(),
        vert_count: r.i32(),
        max_link_count: r.i32(),
        detail_mesh_count: r.i32(),
        detail_vert_count: r.i32(),
        detail_tri_count: r.i32(),
        bv_node_count: r.i32(),
        off_mesh_con_count: r.i32(),
        off_mesh_base: r.i32(),
        walkable_height: r.f32(),
        walkable_radius: r.f32(),
        walkable_climb: r.f32(),
        bmin: r.v3(),
        bmax: r.v3(),
        bv_quant_factor: r.f32(),
    };
    if h.magic != NAVMESH_MAGIC {
        return Err(Error::InvalidTileData(TileDataError::WrongMagic));
    }
    if h.version != NAVMESH_VERSION {
        return Err(Error::InvalidTileData(TileDataError::WrongVersion));
    }
    Ok(h)
}

fn bad(r: TileDataError) -> Error {
    Error::InvalidTileData(r)
}

/// A vector of `n` elements made by `f`, allocated fallibly up front.
fn try_vec<T>(n: usize, mut f: impl FnMut() -> T) -> Result<Vec<T>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    for _ in 0..n {
        v.push(f());
    }
    Ok(v)
}

/// Checks the header counts against `limits` before anything is allocated: the
/// per-tile counts, the tile size and the bytes the decoded tile will hold.
fn check_limits(h: &MeshHeader, len: usize, limits: &Limits) -> Result<(), Error> {
    // Counts are non-negative here: `Sections::of` rejected negative ones.
    let n = |c: i32| c as usize;
    let detail = n(h.detail_mesh_count)
        .max(n(h.detail_vert_count))
        .max(n(h.detail_tri_count));
    if len > limits.max_tile_bytes
        || n(h.poly_count) > limits.max_tile_polys
        || n(h.vert_count) > limits.max_tile_verts
        || n(h.max_link_count) > limits.max_tile_links
        || detail > limits.max_tile_detail
    {
        return Err(Error::LimitExceeded);
    }
    let decoded = [
        (n(h.vert_count), 3 * size_of::<f32>()),
        (n(h.poly_count), size_of::<Poly>()),
        (n(h.max_link_count), size_of::<Link>()),
        (n(h.detail_mesh_count), size_of::<PolyDetail>()),
        (n(h.detail_vert_count), 3 * size_of::<f32>()),
        (n(h.detail_tri_count), 4),
        (n(h.bv_node_count), size_of::<BvNode>()),
        (n(h.off_mesh_con_count), size_of::<OffMeshConnection>()),
    ]
    .iter()
    .try_fold(0usize, |a, &(c, s)| a.checked_add(c.checked_mul(s)?))
    .ok_or(Error::SizeOverflow)?;
    if decoded > limits.max_bytes {
        return Err(Error::LimitExceeded);
    }
    Ok(())
}

impl MeshTile {
    /// Decodes and validates tile bytes. Every index upstream follows without a
    /// check is checked here, so a tile that passes cannot make a query read out of
    /// bounds. Same as [`MeshTile::parse_with_limits`] with [`Limits::UNLIMITED`]:
    /// allocations are still bounded by the buffer length.
    pub fn parse(data: Vec<u8>) -> Result<MeshTile, Error> {
        Self::parse_with_limits(data, &Limits::UNLIMITED)
    }

    /// Decodes and validates tile bytes under `limits`. Every section must fit the
    /// buffer and every count must be within `limits` before anything is allocated;
    /// allocation is fallible ([`Error::AllocationFailed`]), a count over a limit is
    /// [`Error::LimitExceeded`] and malformed bytes are [`Error::InvalidTileData`].
    pub fn parse_with_limits(data: Vec<u8>, limits: &Limits) -> Result<MeshTile, Error> {
        let h = peek_header(&data)?;
        let s = Sections::of(&h).ok_or(bad(TileDataError::BadSize))?;
        let total = s.total().ok_or(bad(TileDataError::BadSize))?;
        if total > data.len() {
            return Err(bad(TileDataError::BadSize));
        }
        check_limits(&h, data.len(), limits)?;
        // Every read below stays inside the sections, which fit `data` (checked above).
        let mut r = Reader {
            b: &data,
            at: HEADER_SIZE,
        };
        let mut t = MeshTile {
            header: h,
            ..Default::default()
        };
        let mut next = HEADER_SIZE;
        let nv = h.vert_count as usize;
        t.verts = try_vec(nv * 3, || r.f32())?;
        next += s.verts;
        r.at = next;
        let np = h.poly_count as usize;
        t.polys = try_vec(np, || {
            let first_link = r.u32();
            let mut p = Poly {
                first_link,
                ..Default::default()
            };
            for v in &mut p.verts {
                *v = r.u16();
            }
            for v in &mut p.neis {
                *v = r.u16();
            }
            p.flags = r.u16();
            p.vert_count = r.u8();
            p.area_and_type = r.u8();
            p
        })?;
        next += s.polys;
        // Links are rebuilt when the tile is added; their bytes are not read.
        t.links = try_vec(h.max_link_count as usize, Link::default)?;
        next += s.links;
        r.at = next;
        t.detail_meshes = try_vec(h.detail_mesh_count as usize, || {
            let d = PolyDetail {
                vert_base: r.u32(),
                tri_base: r.u32(),
                vert_count: r.u8(),
                tri_count: r.u8(),
            };
            r.at += 2;
            d
        })?;
        next += s.detail_meshes;
        r.at = next;
        t.detail_verts = try_vec(h.detail_vert_count as usize * 3, || r.f32())?;
        next += s.detail_verts;
        r.at = next;
        t.detail_tris = try_vec(h.detail_tri_count as usize * 4, || r.u8())?;
        next += s.detail_tris;
        r.at = next;
        t.bv_tree = try_vec(h.bv_node_count as usize, || BvNode {
            bmin: [r.u16(), r.u16(), r.u16()],
            bmax: [r.u16(), r.u16(), r.u16()],
            i: r.i32(),
        })?;
        next += s.bv_tree;
        r.at = next;
        t.off_mesh_cons = try_vec(h.off_mesh_con_count as usize, || {
            let mut pos = [0.0; 6];
            for p in &mut pos {
                *p = r.f32();
            }
            OffMeshConnection {
                pos,
                rad: r.f32(),
                poly: r.u16(),
                flags: r.u8(),
                side: r.u8(),
                user_id: r.u32(),
            }
        })?;
        t.validate()?;
        t.data = data;
        Ok(t)
    }

    fn validate(&self) -> Result<(), Error> {
        let h = &self.header;
        let finite = |v: &[f32]| v.iter().all(|x| x.is_finite());
        if !finite(&self.verts)
            || !finite(&self.detail_verts)
            || !finite(&h.bmin)
            || !finite(&h.bmax)
            || !h.walkable_height.is_finite()
            || !h.walkable_radius.is_finite()
            || !h.walkable_climb.is_finite()
            || !h.bv_quant_factor.is_finite()
            || self
                .off_mesh_cons
                .iter()
                .any(|c| !finite(&c.pos) || !c.rad.is_finite())
        {
            return Err(bad(TileDataError::NonFinite));
        }
        let nv = h.vert_count as usize;
        let np = h.poly_count as usize;
        if h.off_mesh_base < 0 || h.off_mesh_base as usize > np {
            return Err(bad(TileDataError::BadOffMeshConnection));
        }
        for (i, p) in self.polys.iter().enumerate() {
            let n = p.vert_count as usize;
            let offmesh = p.poly_type() == POLYTYPE_OFFMESH_CONNECTION;
            if p.poly_type() > POLYTYPE_OFFMESH_CONNECTION
                || n > VERTS_PER_POLYGON
                || (offmesh && n != 2)
                || (!offmesh && n < 3)
                || p.verts[..n].iter().any(|&v| v as usize >= nv)
            {
                return Err(bad(TileDataError::BadPoly));
            }
            for &nei in &p.neis[..n] {
                let bad_nei = if nei & EXT_LINK != 0 {
                    (nei & 0xff) > 7
                } else {
                    nei != 0 && nei as usize > np
                };
                if bad_nei {
                    return Err(bad(TileDataError::BadPoly));
                }
            }
            if offmesh {
                let k = i as i64 - h.off_mesh_base as i64;
                if k < 0 || k >= self.off_mesh_cons.len() as i64 {
                    return Err(bad(TileDataError::BadOffMeshConnection));
                }
            } else {
                let d = self
                    .detail_meshes
                    .get(i)
                    .ok_or(bad(TileDataError::BadDetail))?;
                // u64: a u32 base plus a count cannot wrap, even with a 32-bit usize.
                let vend = d.vert_base as u64 + d.vert_count as u64;
                let tend = d.tri_base as u64 + d.tri_count as u64;
                if vend > h.detail_vert_count as u64 || tend > h.detail_tri_count as u64 {
                    return Err(bad(TileDataError::BadDetail));
                }
                let limit = n + d.vert_count as usize;
                for t in d.tri_base as usize..tend as usize {
                    if self.detail_tris[t * 4..t * 4 + 3]
                        .iter()
                        .any(|&k| k as usize >= limit)
                    {
                        return Err(bad(TileDataError::BadDetail));
                    }
                }
            }
        }
        // A leaf names a polygon; an internal node's escape index (`-i`) skips its
        // subtree and lands at most one past the last node; bounds are ordered.
        let nodes = self.bv_tree.len() as u64;
        for (k, node) in self.bv_tree.iter().enumerate() {
            let in_range = if node.i >= 0 {
                (node.i as usize) < np
            } else {
                k as u64 + node.i.unsigned_abs() as u64 <= nodes
            };
            if !in_range || (0..3).any(|a| node.bmin[a] > node.bmax[a]) {
                return Err(bad(TileDataError::BadBvTree));
            }
        }
        // Connection `j` owns polygon `off_mesh_base + j`, an off-mesh polygon.
        for (j, c) in self.off_mesh_cons.iter().enumerate() {
            let owner = h.off_mesh_base as usize + j;
            let p = self
                .polys
                .get(owner)
                .ok_or(bad(TileDataError::BadOffMeshConnection))?;
            if c.poly as usize != owner || p.poly_type() != POLYTYPE_OFFMESH_CONNECTION {
                return Err(bad(TileDataError::BadOffMeshConnection));
            }
        }
        Ok(())
    }

    /// Resets the link pool to one free list over every link (as `addTile` does).
    pub(crate) fn reset_links(&mut self) -> u32 {
        let n = self.links.len();
        for (i, l) in self.links.iter_mut().enumerate() {
            l.next = if i + 1 == n { NULL_LINK } else { i as u32 + 1 };
        }
        if n == 0 { NULL_LINK } else { 0 }
    }
}

// Bake output limits without allocating decoded runtime sections.
#[cfg(feature = "recast")]
pub(crate) fn check_bake_tile_limits(data: &[u8], limits: &Limits) -> Result<(), Error> {
    check_limits(&peek_header(data)?, data.len(), limits)
}
