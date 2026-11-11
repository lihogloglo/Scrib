//! Tile creation (`dtCreateNavMeshData`).

use alloc::vec::Vec;

use super::math::{align4, vmax, vmin};
use super::tile::{
    BV_NODE_SIZE, HEADER_SIZE, LINK_SIZE, OFF_MESH_CON_SIZE, POLY_DETAIL_SIZE, POLY_SIZE,
};
use super::{
    EXT_LINK, NAVMESH_MAGIC, NAVMESH_VERSION, OFFMESH_CON_BIDIR, POLYTYPE_GROUND,
    POLYTYPE_OFFMESH_CONNECTION, VERTS_PER_POLYGON,
};
use crate::common::{ceilf, floorf};
use crate::{Error, Limits, Workspace};

const MESH_NULL_IDX: u16 = 0xffff;

/// Input of [`create_nav_mesh_data`] (`dtNavMeshCreateParams`).
#[derive(Clone, Copy, Debug, Default)]
pub struct NavMeshCreateParams<'a> {
    /// Poly mesh vertices in cells (3 per vertex).
    pub verts: &'a [u16],
    /// Number of poly mesh vertices.
    pub vert_count: i32,
    /// Polygons: `nvp` vertex indices then `nvp` neighbour slots each.
    pub polys: &'a [u16],
    /// Flags per polygon.
    pub poly_flags: &'a [u16],
    /// Area per polygon.
    pub poly_areas: &'a [u8],
    /// Number of polygons.
    pub poly_count: i32,
    /// Maximum vertices per polygon.
    pub nvp: i32,
    /// Detail sub-meshes (4 per polygon), or empty for none.
    pub detail_meshes: &'a [u32],
    /// Detail vertices (3 floats each).
    pub detail_verts: &'a [f32],
    /// Number of detail vertices.
    pub detail_verts_count: i32,
    /// Detail triangles (4 bytes each).
    pub detail_tris: &'a [u8],
    /// Number of detail triangles.
    pub detail_tri_count: i32,
    /// Off-mesh connection end points (6 floats each).
    pub off_mesh_con_verts: &'a [f32],
    /// Off-mesh connection radii.
    pub off_mesh_con_rad: &'a [f32],
    /// Off-mesh connection flags.
    pub off_mesh_con_flags: &'a [u16],
    /// Off-mesh connection areas.
    pub off_mesh_con_areas: &'a [u8],
    /// Off-mesh connection directions (non-zero: bidirectional).
    pub off_mesh_con_dir: &'a [u8],
    /// Off-mesh connection user ids (may be empty).
    pub off_mesh_con_user_id: &'a [u32],
    /// Number of off-mesh connections.
    pub off_mesh_con_count: i32,
    /// Tile user id.
    pub user_id: u32,
    /// Tile x.
    pub tile_x: i32,
    /// Tile y.
    pub tile_y: i32,
    /// Tile layer.
    pub tile_layer: i32,
    /// Tile bounds minimum.
    pub bmin: [f32; 3],
    /// Tile bounds maximum.
    pub bmax: [f32; 3],
    /// Agent height (world units).
    pub walkable_height: f32,
    /// Agent radius (world units).
    pub walkable_radius: f32,
    /// Agent climb (world units).
    pub walkable_climb: f32,
    /// Cell size.
    pub cs: f32,
    /// Cell height.
    pub ch: f32,
    /// Whether to build the bounding-volume tree.
    pub build_bv_tree: bool,
}

#[derive(Clone, Copy)]
struct BvItem {
    bmin: [u16; 3],
    bmax: [u16; 3],
    i: i32,
}

fn longest_axis(x: u16, y: u16, z: u16) -> usize {
    let mut axis = 0;
    let mut max = x;
    if y > max {
        axis = 1;
        max = y;
    }
    if z > max {
        axis = 2;
    }
    axis
}

fn subdivide(
    items: &mut [BvItem],
    scratch: &mut [BvItem],
    imin: usize,
    imax: usize,
    cur: &mut usize,
    nodes: &mut [BvItem],
) {
    let inum = imax - imin;
    let icur = *cur;
    *cur += 1;
    if inum == 1 {
        nodes[icur] = items[imin];
        return;
    }
    let mut bmin = items[imin].bmin;
    let mut bmax = items[imin].bmax;
    for it in &items[imin + 1..imax] {
        for k in 0..3 {
            if it.bmin[k] < bmin[k] {
                bmin[k] = it.bmin[k];
            }
            if it.bmax[k] > bmax[k] {
                bmax[k] = it.bmax[k];
            }
        }
    }
    let axis = longest_axis(
        bmax[0].wrapping_sub(bmin[0]),
        bmax[1].wrapping_sub(bmin[1]),
        bmax[2].wrapping_sub(bmin[2]),
    );
    // glibc qsort is a stable merge sort; a stable sort with the same comparator
    // reproduces it.
    crate::common::stable_sort(&mut items[imin..imax], scratch, |a, b| {
        a.bmin[axis].cmp(&b.bmin[axis])
    });
    let isplit = imin + inum / 2;
    subdivide(items, scratch, imin, isplit, cur, nodes);
    subdivide(items, scratch, isplit, imax, cur, nodes);
    let iescape = (*cur - icur) as i32;
    nodes[icur] = BvItem {
        bmin,
        bmax,
        i: -iescape,
    };
}

fn quant(v: f32, base: f32, f: f32) -> u16 {
    // (int) truncation, then clamp to [0, 0xffff]; Rust's saturating cast equals
    // upstream for every in-range value (upstream is undefined outside it).
    (((v - base) * f) as i32).clamp(0, 0xffff) as u16
}

fn create_bv_tree(
    ws: &mut Workspace,
    p: &NavMeshCreateParams,
    nodes: &mut [BvItem],
    world: Option<&[f32]>,
) -> Result<usize, Error> {
    let qf = 1.0 / p.cs;
    let nvp = p.nvp as usize;
    let mut items = ws.vec_with_capacity(p.poly_count as usize)?;
    for i in 0..p.poly_count as usize {
        items.push(if !p.detail_meshes.is_empty() {
            let vb = p.detail_meshes[i * 4] as usize;
            let ndv = p.detail_meshes[i * 4 + 1] as usize;
            let dv = &p.detail_verts[vb * 3..];
            let mut bmin = [dv[0], dv[1], dv[2]];
            let mut bmax = bmin;
            for j in 1..ndv {
                vmin(&mut bmin, &dv[j * 3..]);
                vmax(&mut bmax, &dv[j * 3..]);
            }
            BvItem {
                bmin: [0, 1, 2].map(|k| quant(bmin[k], p.bmin[k], qf)),
                bmax: [0, 1, 2].map(|k| quant(bmax[k], p.bmin[k], qf)),
                i: i as i32,
            }
        } else if let Some(world) = world {
            let pv = &p.polys[i * nvp * 2..];
            let v = usize::from(pv[0]) * 3;
            let mut lo = [world[v], world[v + 1], world[v + 2]];
            let mut hi = lo;
            for &idx in pv.iter().take(nvp).take_while(|&&v| v != MESH_NULL_IDX) {
                let v = usize::from(idx) * 3;
                vmin(&mut lo, &world[v..]);
                vmax(&mut hi, &world[v..]);
            }
            BvItem {
                bmin: [0, 1, 2].map(|k| quant(lo[k], p.bmin[k], qf)),
                bmax: [0, 1, 2].map(|k| quant(hi[k], p.bmin[k], qf)),
                i: i as i32,
            }
        } else {
            let pv = &p.polys[i * nvp * 2..];
            let v0 = pv[0] as usize * 3;
            let mut it = BvItem {
                bmin: [p.verts[v0], p.verts[v0 + 1], p.verts[v0 + 2]],
                bmax: [p.verts[v0], p.verts[v0 + 1], p.verts[v0 + 2]],
                i: i as i32,
            };
            for &vj in pv.iter().take(nvp).skip(1) {
                if vj == MESH_NULL_IDX {
                    break;
                }
                let v = &p.verts[vj as usize * 3..];
                for k in 0..3 {
                    if v[k] < it.bmin[k] {
                        it.bmin[k] = v[k];
                    }
                    if v[k] > it.bmax[k] {
                        it.bmax[k] = v[k];
                    }
                }
            }
            it.bmin[1] = floorf(it.bmin[1] as f32 * p.ch / p.cs) as u16;
            it.bmax[1] = ceilf(it.bmax[1] as f32 * p.ch / p.cs) as u16;
            it
        });
    }
    let mut cur = 0;
    let n = items.len();
    if n > 0 {
        let mut scratch = ws.vec(n, items[0])?;
        subdivide(&mut items, &mut scratch, 0, n, &mut cur, nodes);
    }
    Ok(cur)
}

fn classify_off_mesh_point(pt: &[f32], bmin: &[f32; 3], bmax: &[f32; 3]) -> u8 {
    const XP: u8 = 1;
    const ZP: u8 = 2;
    const XM: u8 = 4;
    const ZM: u8 = 8;
    let mut outcode = 0;
    if pt[0] >= bmax[0] {
        outcode |= XP;
    }
    if pt[2] >= bmax[2] {
        outcode |= ZP;
    }
    if pt[0] < bmin[0] {
        outcode |= XM;
    }
    if pt[2] < bmin[2] {
        outcode |= ZM;
    }
    match outcode {
        XP => 0,
        x if x == XP | ZP => 1,
        ZP => 2,
        x if x == XM | ZP => 3,
        XM => 4,
        x if x == XM | ZM => 5,
        ZM => 6,
        x if x == XP | ZM => 7,
        _ => 0xff,
    }
}

struct Writer {
    b: Vec<u8>,
}

impl Writer {
    fn at(&mut self, at: usize) -> Cursor<'_> {
        Cursor { b: &mut self.b, at }
    }
}

struct Cursor<'a> {
    b: &'a mut Vec<u8>,
    at: usize,
}

impl Cursor<'_> {
    fn put(&mut self, s: &[u8]) -> &mut Self {
        self.b[self.at..self.at + s.len()].copy_from_slice(s);
        self.at += s.len();
        self
    }
    fn u8(&mut self, v: u8) -> &mut Self {
        self.put(&[v])
    }
    fn u16(&mut self, v: u16) -> &mut Self {
        self.put(&v.to_le_bytes())
    }
    fn u32(&mut self, v: u32) -> &mut Self {
        self.put(&v.to_le_bytes())
    }
    fn i32(&mut self, v: i32) -> &mut Self {
        self.put(&v.to_le_bytes())
    }
    fn f32(&mut self, v: f32) -> &mut Self {
        self.put(&v.to_le_bytes())
    }
}

fn check_len<T>(s: &[T], n: usize) -> Result<(), Error> {
    if s.len() < n {
        Err(Error::InvalidInput("array shorter than its count"))
    } else {
        Ok(())
    }
}

/// Builds tile bytes from a poly mesh, an optional detail mesh and off-mesh
/// connections, byte-identical to upstream's `dtCreateNavMeshData`.
pub fn create_nav_mesh_data(p: &NavMeshCreateParams) -> Result<Vec<u8>, Error> {
    create_with_limits(p, &Limits::UNLIMITED)
}

/// Imports world-space float vertices without quantizing them (not in recastnavigation).
/// `verts` must contain exactly `vert_count * 3` finite coordinates within the
/// finite ordered tile bounds. `params.verts` is ignored; all other parameters
/// retain their usual meaning. DNAV stores vertex floats verbatim. BV bounds
/// alone are quantized using `cs`; choose cs so every bound fits 0..=65535.
/// This API does not weld, clip, or infer polygon adjacency.
pub fn create_nav_mesh_data_f32(p: &NavMeshCreateParams, verts: &[f32]) -> Result<Vec<u8>, Error> {
    if !(p.cs > 0.0)
        || !p.cs.is_finite()
        || !(1.0 / p.cs).is_finite()
        || !(p.ch > 0.0)
        || !p.ch.is_finite()
    {
        return Err(Error::InvalidParameter("float import cell size"));
    }
    if p.off_mesh_con_count < 0 || p.detail_verts_count < 0 || p.detail_tri_count < 0 {
        return Err(Error::InvalidParameter("negative import count"));
    }
    // Conservatively bound encoding/output arithmetic on narrow targets before
    // the shared encoder computes fixed-stride section offsets.
    [
        p.vert_count,
        p.poly_count,
        p.off_mesh_con_count,
        p.detail_verts_count,
        p.detail_tri_count,
    ]
    .into_iter()
    .try_fold(0usize, |sum, n| {
        let n = usize::try_from(n).map_err(|_| Error::InvalidParameter("import count"))?;
        sum.checked_add(n.checked_mul(1024).ok_or(Error::SizeOverflow)?)
            .ok_or(Error::SizeOverflow)
    })?;
    if !p.off_mesh_con_user_id.is_empty() {
        check_len(
            p.off_mesh_con_user_id,
            usize::try_from(p.off_mesh_con_count).map_err(|_| Error::SizeOverflow)?,
        )?;
    }
    if p.poly_areas
        .iter()
        .chain(p.off_mesh_con_areas)
        .any(|&a| a > 63)
        || p.detail_verts
            .iter()
            .chain(p.off_mesh_con_verts)
            .any(|v| !v.is_finite())
        || p.off_mesh_con_rad
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
    {
        return Err(Error::InvalidInput(
            "float import area, coordinate or radius",
        ));
    }
    let n = usize::try_from(p.vert_count)
        .map_err(|_| Error::InvalidParameter("vert_count"))?
        .checked_mul(3)
        .ok_or(Error::SizeOverflow)?;
    if verts.len() != n
        || p.bmin.iter().chain(&p.bmax).any(|v| !v.is_finite())
        || (0..3).any(|k| {
            p.bmin[k] > p.bmax[k]
                || !(p.bmax[k] - p.bmin[k]).is_finite()
                || (p.build_bv_tree && (p.bmax[k] - p.bmin[k]) * (1.0 / p.cs) > 65535.0)
        })
    {
        return Err(Error::InvalidInput("float vertices or bounds"));
    }
    for v in verts.as_chunks::<3>().0 {
        for k in 0..3 {
            if !v[k].is_finite()
                || v[k] < p.bmin[k]
                || v[k] > p.bmax[k]
                || (p.build_bv_tree && (v[k] - p.bmin[k]) / p.cs > 65535.0)
            {
                return Err(Error::InvalidInput("float vertex outside tile or BV range"));
            }
        }
    }
    let data =
        create_with_workspace_impl(p, &Limits::UNLIMITED, &mut Workspace::new(), Some(verts))?;
    // Validate all serialized indices, detail sections and connection fields too.
    super::MeshTile::parse(data).map(|tile| tile.data)
}

// Calculate exact output size/counts before allocating any builder storage.
pub(crate) fn create_with_limits(
    p: &NavMeshCreateParams,
    limits: &Limits,
) -> Result<Vec<u8>, Error> {
    create_with_workspace(p, limits, &mut Workspace::new())
}

fn create_with_workspace(
    p: &NavMeshCreateParams,
    limits: &Limits,
    ws: &mut Workspace,
) -> Result<Vec<u8>, Error> {
    create_with_workspace_impl(p, limits, ws, None)
}

fn create_with_workspace_impl(
    p: &NavMeshCreateParams,
    limits: &Limits,
    ws: &mut Workspace,
    world: Option<&[f32]>,
) -> Result<Vec<u8>, Error> {
    if p.nvp as usize > VERTS_PER_POLYGON || p.nvp < 3 {
        return Err(Error::InvalidParameter("nvp"));
    }
    if p.vert_count >= 0xffff || p.vert_count <= 0 {
        return Err(Error::InvalidParameter("vert_count"));
    }
    if p.poly_count <= 0 {
        return Err(Error::InvalidParameter("poly_count"));
    }
    if !(p.cs > 0.0) || !p.ch.is_finite() || !p.cs.is_finite() {
        return Err(Error::InvalidParameter("cs"));
    }
    let nvp = p.nvp as usize;
    let npolys = p.poly_count as usize;
    let nverts = p.vert_count as usize;
    let noff = p.off_mesh_con_count.max(0) as usize;
    if world.is_none() {
        check_len(p.verts, nverts * 3)?;
    }
    check_len(p.polys, npolys * nvp * 2)?;
    check_len(p.poly_flags, npolys)?;
    check_len(p.poly_areas, npolys)?;
    if p.polys[..npolys * nvp * 2].chunks(nvp * 2).any(|c| {
        c[..nvp].iter().take_while(|&&v| v != MESH_NULL_IDX).count() < 3
            || c[..nvp]
                .iter()
                .any(|&v| v != MESH_NULL_IDX && v as usize >= nverts)
    }) {
        return Err(Error::InvalidInput("polygon vertex index"));
    }
    let has_detail = !p.detail_meshes.is_empty();
    if has_detail {
        check_len(p.detail_meshes, npolys * 4)?;
        check_len(p.detail_verts, p.detail_verts_count.max(0) as usize * 3)?;
        check_len(p.detail_tris, p.detail_tri_count.max(0) as usize * 4)?;
        for m in p.detail_meshes[..npolys * 4].chunks(4) {
            if m[0] as usize + m[1] as usize > p.detail_verts_count.max(0) as usize || m[1] == 0 {
                return Err(Error::InvalidInput("detail mesh vertex range"));
            }
        }
    }
    check_len(p.off_mesh_con_verts, noff * 6)?;
    check_len(p.off_mesh_con_rad, noff)?;
    check_len(p.off_mesh_con_flags, noff)?;
    check_len(p.off_mesh_con_areas, noff)?;
    check_len(p.off_mesh_con_dir, noff)?;

    let mut off_bounds = None;
    let mut stored_off = 0usize;
    let mut off_link_count = 0usize;
    if noff > 0 {
        let mut hmin = f32::MAX;
        let mut hmax = -f32::MAX;
        if has_detail && p.detail_verts_count > 0 {
            for i in 0..p.detail_verts_count as usize {
                let h = p.detail_verts[i * 3 + 1];
                hmin = super::math::fmin(hmin, h);
                hmax = super::math::fmax(hmax, h);
            }
        } else {
            for i in 0..nverts {
                let h = world.map_or_else(
                    || p.bmin[1] + p.verts[i * 3 + 1] as f32 * p.ch,
                    |v| v[i * 3 + 1],
                );
                hmin = super::math::fmin(hmin, h);
                hmax = super::math::fmax(hmax, h);
            }
        }
        hmin -= p.walkable_climb;
        hmax += p.walkable_climb;
        let mut bmin = p.bmin;
        let mut bmax = p.bmax;
        bmin[1] = hmin;
        bmax[1] = hmax;
        off_bounds = Some((bmin, bmax));
        for i in 0..noff {
            let p0 = &p.off_mesh_con_verts[i * 6..];
            let p1 = &p.off_mesh_con_verts[i * 6 + 3..];
            let mut c0 = classify_off_mesh_point(p0, &bmin, &bmax);
            let c1 = classify_off_mesh_point(p1, &bmin, &bmax);
            if c0 == 0xff && (p0[1] < bmin[1] || p0[1] > bmax[1]) {
                c0 = 0;
            }
            if c0 == 0xff {
                off_link_count += 1;
            }
            if c1 == 0xff {
                off_link_count += 1;
            }
            if c0 == 0xff {
                stored_off += 1;
            }
        }
    }
    let tot_poly = npolys + stored_off;
    let tot_vert = nverts + stored_off * 2;
    let mut edge_count = 0usize;
    let mut portal_count = 0usize;
    let mut detail_tri_count = 0usize;
    let mut unique_detail = 0usize;
    for i in 0..npolys {
        let pp = &p.polys[i * 2 * nvp..];
        let mut nv = 0;
        for j in 0..nvp {
            if pp[j] == MESH_NULL_IDX {
                break;
            }
            nv += 1;
            edge_count += 1;
            if pp[nvp + j] & 0x8000 != 0 && pp[nvp + j] & 0xf != 0xf {
                portal_count += 1;
            }
        }
        if has_detail {
            // Upstream subtracts in int; a detail mesh with fewer vertices than its
            // polygon would underflow, which the range check above rules out.
            unique_detail += (p.detail_meshes[i * 4 + 1] as usize).saturating_sub(nv);
        } else {
            detail_tri_count += nv - 2;
        }
    }
    if has_detail {
        detail_tri_count = p.detail_tri_count.max(0) as usize;
    }
    let max_link_count = edge_count + portal_count * 2 + off_link_count * 2;
    if tot_vert > 0xffff {
        return Err(Error::FormatLimit("more than 65,535 tile vertices"));
    }

    let verts_size = align4(12 * tot_vert);
    let polys_size = align4(POLY_SIZE * tot_poly);
    let links_size = align4(LINK_SIZE * max_link_count);
    let dmeshes_size = align4(POLY_DETAIL_SIZE * npolys);
    let dverts_size = align4(12 * unique_detail);
    let dtris_size = align4(4 * detail_tri_count);
    let bv_size = if p.build_bv_tree {
        align4(BV_NODE_SIZE * npolys * 2)
    } else {
        0
    };
    let off_size = align4(OFF_MESH_CON_SIZE * stored_off);
    let data_size = HEADER_SIZE
        + verts_size
        + polys_size
        + links_size
        + dmeshes_size
        + dverts_size
        + dtris_size
        + bv_size
        + off_size;
    if data_size > limits.max_tile_bytes
        || tot_poly > limits.max_tile_polys
        || tot_vert > limits.max_tile_verts
        || max_link_count > limits.max_tile_links
        || npolys.max(unique_detail).max(detail_tri_count) > limits.max_tile_detail
    {
        return Err(Error::LimitExceeded);
    }
    let mut class = ws.vec(noff * 2, 0u8)?;
    if let Some((bmin, bmax)) = off_bounds {
        for i in 0..noff {
            let p0 = &p.off_mesh_con_verts[i * 6..];
            class[i * 2] = classify_off_mesh_point(p0, &bmin, &bmax);
            class[i * 2 + 1] =
                classify_off_mesh_point(&p.off_mesh_con_verts[i * 6 + 3..], &bmin, &bmax);
            if class[i * 2] == 0xff && (p0[1] < bmin[1] || p0[1] > bmax[1]) {
                class[i * 2] = 0;
            }
        }
    }
    let mut w = Writer { b: Vec::new() };
    w.b.try_reserve_exact(data_size)
        .map_err(|_| Error::AllocationFailed)?;
    w.b.resize(data_size, 0);
    let verts_at = HEADER_SIZE;
    let polys_at = verts_at + verts_size;
    let dmeshes_at = polys_at + polys_size + links_size;
    let dverts_at = dmeshes_at + dmeshes_size;
    let dtris_at = dverts_at + dverts_size;
    let bv_at = dtris_at + dtris_size;
    let off_at = bv_at + bv_size;

    {
        let mut c = w.at(0);
        c.i32(NAVMESH_MAGIC)
            .i32(NAVMESH_VERSION)
            .i32(p.tile_x)
            .i32(p.tile_y)
            .i32(p.tile_layer)
            .u32(p.user_id)
            .i32(tot_poly as i32)
            .i32(tot_vert as i32)
            .i32(max_link_count as i32)
            .i32(npolys as i32)
            .i32(unique_detail as i32)
            .i32(detail_tri_count as i32)
            .i32(if p.build_bv_tree {
                npolys as i32 * 2
            } else {
                0
            })
            .i32(stored_off as i32)
            .i32(npolys as i32)
            .f32(p.walkable_height)
            .f32(p.walkable_radius)
            .f32(p.walkable_climb);
        for v in p.bmin.iter().chain(p.bmax.iter()) {
            c.f32(*v);
        }
        c.f32(1.0 / p.cs);
    }
    {
        let mut c = w.at(verts_at);
        for i in 0..nverts {
            if let Some(world) = world {
                for &v in &world[i * 3..i * 3 + 3] {
                    c.f32(v);
                }
            } else {
                let iv = &p.verts[i * 3..];
                c.f32(p.bmin[0] + iv[0] as f32 * p.cs)
                    .f32(p.bmin[1] + iv[1] as f32 * p.ch)
                    .f32(p.bmin[2] + iv[2] as f32 * p.cs);
            }
        }
        for i in 0..noff {
            if class[i * 2] == 0xff {
                for &v in &p.off_mesh_con_verts[i * 6..i * 6 + 6] {
                    c.f32(v);
                }
            }
        }
    }
    let mut poly_vert_counts = ws.vec(npolys, 0usize)?;
    {
        let mut c = w.at(polys_at);
        for i in 0..npolys {
            let src = &p.polys[i * nvp * 2..];
            let mut verts = [0u16; VERTS_PER_POLYGON];
            let mut neis = [0u16; VERTS_PER_POLYGON];
            let mut nv = 0usize;
            for j in 0..nvp {
                if src[j] == MESH_NULL_IDX {
                    break;
                }
                verts[j] = src[j];
                let nei = src[nvp + j];
                neis[j] = if nei & 0x8000 != 0 {
                    match nei & 0xf {
                        0xf => 0,
                        0 => EXT_LINK | 4,
                        1 => EXT_LINK | 2,
                        2 => EXT_LINK,
                        3 => EXT_LINK | 6,
                        _ => neis[j],
                    }
                } else {
                    nei.wrapping_add(1)
                };
                nv += 1;
            }
            poly_vert_counts[i] = nv;
            c.u32(0);
            for v in verts {
                c.u16(v);
            }
            for v in neis {
                c.u16(v);
            }
            c.u16(p.poly_flags[i])
                .u8(nv as u8)
                .u8((p.poly_areas[i] & 0x3f) | (POLYTYPE_GROUND << 6));
        }
        let mut n = 0;
        for i in 0..noff {
            if class[i * 2] == 0xff {
                c.u32(0)
                    .u16((nverts + n * 2) as u16)
                    .u16((nverts + n * 2 + 1) as u16);
                for _ in 0..4 + 6 {
                    c.u16(0);
                }
                c.u16(p.off_mesh_con_flags[i])
                    .u8(2)
                    .u8((p.off_mesh_con_areas[i] & 0x3f) | (POLYTYPE_OFFMESH_CONNECTION << 6));
                n += 1;
            }
        }
    }
    if has_detail {
        let mut vbase = 0usize;
        let mut dverts = ws.vec_with_capacity(unique_detail * 3)?;
        {
            let mut c = w.at(dmeshes_at);
            for i in 0..npolys {
                let m = &p.detail_meshes[i * 4..];
                let (vb, ndv) = (m[0] as usize, m[1] as usize);
                let nv = poly_vert_counts[i];
                let extra = ndv.saturating_sub(nv);
                c.u32(vbase as u32)
                    .u32(m[2])
                    .u8(extra as u8)
                    .u8(m[3] as u8)
                    .u16(0);
                if extra > 0 {
                    dverts.extend_from_slice(&p.detail_verts[(vb + nv) * 3..(vb + ndv) * 3]);
                    vbase = (vbase + extra) & 0xffff;
                }
            }
        }
        let mut c = w.at(dverts_at);
        for v in dverts {
            c.f32(v);
        }
        let n = detail_tri_count * 4;
        w.at(dtris_at).put(&p.detail_tris[..n]);
    } else {
        let mut tbase = 0usize;
        let mut c = w.at(dmeshes_at);
        for &nv in poly_vert_counts.iter() {
            c.u32(0).u32(tbase as u32).u8(0).u8((nv - 2) as u8).u16(0);
            tbase += nv - 2;
        }
        let mut c = w.at(dtris_at);
        for &nv in poly_vert_counts.iter() {
            for j in 2..nv {
                let mut f = 1u8 << 2;
                if j == 2 {
                    f |= 1;
                }
                if j == nv - 1 {
                    f |= 1 << 4;
                }
                c.u8(0).u8((j - 1) as u8).u8(j as u8).u8(f);
            }
        }
    }
    if p.build_bv_tree {
        let mut nodes = ws.vec(
            npolys * 2,
            BvItem {
                bmin: [0; 3],
                bmax: [0; 3],
                i: 0,
            },
        )?;
        let n = create_bv_tree(ws, p, &mut nodes, world)?;
        let mut c = w.at(bv_at);
        for it in &nodes[..n] {
            for v in it.bmin.iter().chain(it.bmax.iter()) {
                c.u16(*v);
            }
            c.i32(it.i);
        }
    }
    {
        let mut c = w.at(off_at);
        let mut n = 0;
        for i in 0..noff {
            if class[i * 2] == 0xff {
                for &v in &p.off_mesh_con_verts[i * 6..i * 6 + 6] {
                    c.f32(v);
                }
                c.f32(p.off_mesh_con_rad[i])
                    .u16((npolys + n) as u16)
                    .u8(if p.off_mesh_con_dir[i] != 0 {
                        OFFMESH_CON_BIDIR
                    } else {
                        0
                    })
                    .u8(class[i * 2 + 1])
                    .u32(p.off_mesh_con_user_id.get(i).copied().unwrap_or(0));
                n += 1;
            }
        }
    }
    Ok(w.b)
}

#[cfg(test)]
mod limit_tests {
    use super::*;

    #[test]
    fn exact_tile_limits_precede_all_builder_allocations() {
        let p = NavMeshCreateParams {
            verts: &[0, 0, 0, 0, 0, 1, 1, 0, 1],
            vert_count: 3,
            polys: &[0, 1, 2, 0xffff, 0xffff, 0xffff],
            poly_flags: &[1],
            poly_areas: &[0],
            poly_count: 1,
            nvp: 3,
            cs: 1.,
            ch: 1.,
            ..NavMeshCreateParams::default()
        };
        let data = create_nav_mesh_data(&p).unwrap();
        let limits = Limits {
            max_tile_bytes: data.len(),
            max_tile_polys: 1,
            max_tile_verts: 3,
            max_tile_links: 3,
            max_tile_detail: 1,
            ..Limits::default()
        };
        assert_eq!(create_with_limits(&p, &limits).unwrap(), data);
        for field in 0..5 {
            let mut tight = limits;
            match field {
                0 => tight.max_tile_bytes -= 1,
                1 => tight.max_tile_polys -= 1,
                2 => tight.max_tile_verts -= 1,
                3 => tight.max_tile_links -= 1,
                _ => tight.max_tile_detail -= 1,
            }
            let mut ws = Workspace::new();
            assert_eq!(
                create_with_workspace(&p, &tight, &mut ws),
                Err(Error::LimitExceeded)
            );
            assert_eq!(ws.usage().bytes, 0);
        }
    }
}
