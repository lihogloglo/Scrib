//! Polygon mesh (upstream `RecastMesh.cpp`).

use alloc::vec::Vec;

use super::{BORDER_VERTEX, ContourSet, MESH_NULL_IDX, MULTIPLE_REGS};
use crate::{Error, Workspace};

/// Partial ear-clipping counts (not in recastnavigation).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TriangulationReport {
    /// Contours producing fewer than n-2 triangles.
    pub failed_contours: usize,
    /// Expected triangles that were not produced.
    pub missing_triangles: usize,
}

/// A polygon mesh (`rcPolyMesh`).
#[derive(Clone, Debug, Default)]
pub struct PolyMesh {
    /// Vertices, x, y, z per vertex in cells.
    pub verts: Vec<u16>,
    /// Polygons: `nvp` vertex indices then `nvp` neighbour slots each.
    pub polys: Vec<u16>,
    /// Region id per polygon.
    pub regs: Vec<u16>,
    /// Flags per polygon.
    pub flags: Vec<u16>,
    /// Area id per polygon.
    pub areas: Vec<u8>,
    /// Vertex count.
    pub nverts: i32,
    /// Polygon count.
    pub npolys: i32,
    /// Allocated polygon count.
    pub maxpolys: i32,
    /// Largest vertex count per polygon.
    pub nvp: i32,
    /// Minimum bounds.
    pub bmin: [f32; 3],
    /// Maximum bounds.
    pub bmax: [f32; 3],
    /// Cell size.
    pub cs: f32,
    /// Cell height.
    pub ch: f32,
    /// Border size.
    pub border_size: i32,
    /// Simplification error of the source contours.
    pub max_edge_error: f32,
}

impl PolyMesh {
    /// Builds a convex polygon mesh from contours (`rcBuildPolyMesh`); `nvp` is the
    /// largest number of vertices per polygon.
    pub fn build(ws: &mut Workspace, cset: &ContourSet, nvp: i32) -> Result<PolyMesh, Error> {
        Self::build_with_report(ws, cset, nvp, false).map(|(mesh, _)| mesh)
    }

    /// Builds with partial-triangulation diagnostics (not in recastnavigation).
    /// Default-compatible mode retains upstream's partial output. Strict mode
    /// returns a typed error at the first incomplete contour.
    pub fn build_with_report(
        ws: &mut Workspace,
        cset: &ContourSet,
        nvp: i32,
        strict: bool,
    ) -> Result<(PolyMesh, TriangulationReport), Error> {
        let mut report = TriangulationReport::default();
        ws.begin();
        if !(3..=0xffff).contains(&nvp) {
            return Err(Error::InvalidParameter("vertices per polygon"));
        }
        let nvp_u = nvp as usize;
        let mut max_vertices = 0usize;
        let mut max_tris = 0usize;
        let mut max_verts_per_cont = 0usize;
        for c in &cset.conts {
            let n = c.verts.len() / 4;
            if n < 3 {
                continue;
            }
            if c.verts
                .chunks_exact(4)
                .any(|v| v[..3].iter().any(|x| !(0..=0xffff).contains(x)))
            {
                return Err(Error::InvalidInput("contour vertex outside 0..=65535"));
            }
            max_vertices += n;
            max_tris += n - 2;
            max_verts_per_cont = max_verts_per_cont.max(n);
        }
        if max_vertices >= 0xfffe {
            return Err(Error::FormatLimit("too many vertices for a polygon mesh"));
        }
        let mut mesh = PolyMesh {
            verts: ws.vec(max_vertices * 3, 0u16)?,
            polys: ws.vec(max_tris * nvp_u * 2, MESH_NULL_IDX)?,
            regs: ws.vec(max_tris, 0u16)?,
            flags: Vec::new(),
            areas: ws.vec(max_tris, 0u8)?,
            nverts: 0,
            npolys: 0,
            maxpolys: max_tris as i32,
            nvp,
            bmin: cset.bmin,
            bmax: cset.bmax,
            cs: cset.cs,
            ch: cset.ch,
            border_size: cset.border_size,
            max_edge_error: cset.max_error,
        };
        let mut vflags = ws.vec(max_vertices + 1, 0u8)?;
        let mut next_vert = ws.vec(max_vertices, 0i32)?;
        let mut first_vert = ws.vec(VERTEX_BUCKET_COUNT, -1i32)?;
        let mut indices = ws.vec(max_verts_per_cont, 0i32)?;
        let mut tris = ws.vec(max_verts_per_cont * 3, 0i32)?;
        let mut polys = ws.vec((max_verts_per_cont + 1) * nvp_u, MESH_NULL_IDX)?;
        let mut nverts = 0usize;
        let mut npolys_total = 0usize;
        for cont in &cset.conts {
            let cn = cont.verts.len() / 4;
            if cn < 3 {
                continue;
            }
            ws.work((cn * cn) as u64)?;
            for (j, idx) in indices[..cn].iter_mut().enumerate() {
                *idx = j as i32;
            }
            let mut ntris = triangulate(cn, &cont.verts, &mut indices[..cn], &mut tris);
            let produced =
                usize::try_from(ntris.unsigned_abs()).map_err(|_| Error::SizeOverflow)?;
            if produced < cn - 2 {
                report.failed_contours = report
                    .failed_contours
                    .checked_add(1)
                    .ok_or(Error::SizeOverflow)?;
                report.missing_triangles = report
                    .missing_triangles
                    .checked_add(cn - 2 - produced)
                    .ok_or(Error::SizeOverflow)?;
                if strict {
                    return Err(Error::PartialTriangulation {
                        failed_contours: report.failed_contours,
                        missing_triangles: report.missing_triangles,
                    });
                }
            }
            if ntris <= 0 {
                ntris = -ntris;
            }
            for j in 0..cn {
                let v = &cont.verts[j * 4..j * 4 + 4];
                let k = add_vertex(
                    v[0] as u16,
                    v[1] as u16,
                    v[2] as u16,
                    &mut mesh.verts,
                    &mut first_vert,
                    &mut next_vert,
                    &mut nverts,
                );
                indices[j] = i32::from(k);
                if v[3] & BORDER_VERTEX != 0 {
                    vflags[usize::from(k)] = 1;
                }
            }
            let mut npolys = 0usize;
            polys[..max_verts_per_cont * nvp_u].fill(MESH_NULL_IDX);
            for t in tris[..ntris as usize * 3].chunks_exact(3) {
                if t[0] != t[1] && t[0] != t[2] && t[1] != t[2] {
                    polys[npolys * nvp_u] = indices[t[0] as usize] as u16;
                    polys[npolys * nvp_u + 1] = indices[t[1] as usize] as u16;
                    polys[npolys * nvp_u + 2] = indices[t[2] as usize] as u16;
                    npolys += 1;
                }
            }
            if npolys == 0 {
                continue;
            }
            if nvp > 3 {
                merge_polys(ws, &mut polys, &mut npolys, nvp_u, &mesh.verts, None)?;
            }
            for j in 0..npolys {
                let base = npolys_total * nvp_u * 2;
                mesh.polys[base..base + nvp_u].copy_from_slice(&polys[j * nvp_u..(j + 1) * nvp_u]);
                mesh.regs[npolys_total] = cont.reg;
                mesh.areas[npolys_total] = cont.area;
                npolys_total += 1;
                if npolys_total > max_tris {
                    return Err(Error::FormatLimit("too many polygons"));
                }
            }
        }
        mesh.nverts = nverts as i32;
        mesh.npolys = npolys_total as i32;

        // Remove edge vertices.
        let mut i = 0usize;
        while i < mesh.nverts as usize {
            if vflags[i] != 0 {
                if !can_remove_vertex(ws, &mesh, i as u16)? {
                    i += 1;
                    continue;
                }
                ws.work(mesh.npolys as u64)?;
                remove_vertex(ws, &mut mesh, i as u16, max_tris)?;
                for j in i..mesh.nverts as usize {
                    vflags[j] = vflags[j + 1];
                }
                continue;
            }
            i += 1;
        }

        build_mesh_adjacency(
            ws,
            &mut mesh.polys,
            mesh.npolys as usize,
            mesh.nverts as usize,
            nvp_u,
        )?;

        if mesh.border_size > 0 {
            let w = cset.width;
            let h = cset.height;
            for i in 0..mesh.npolys as usize {
                let p = &mut mesh.polys[i * 2 * nvp_u..(i + 1) * 2 * nvp_u];
                for j in 0..nvp_u {
                    if p[j] == MESH_NULL_IDX {
                        break;
                    }
                    if p[nvp_u + j] != MESH_NULL_IDX {
                        continue;
                    }
                    let mut nj = j + 1;
                    if nj >= nvp_u || p[nj] == MESH_NULL_IDX {
                        nj = 0;
                    }
                    let va = &mesh.verts[usize::from(p[j]) * 3..];
                    let vb = &mesh.verts[usize::from(p[nj]) * 3..];
                    let (va0, va2, vb0, vb2) = (
                        i32::from(va[0]),
                        i32::from(va[2]),
                        i32::from(vb[0]),
                        i32::from(vb[2]),
                    );
                    if va0 == 0 && vb0 == 0 {
                        p[nvp_u + j] = 0x8000;
                    } else if va2 == h && vb2 == h {
                        p[nvp_u + j] = 0x8001;
                    } else if va0 == w && vb0 == w {
                        p[nvp_u + j] = 0x8002;
                    } else if va2 == 0 && vb2 == 0 {
                        p[nvp_u + j] = 0x8003;
                    }
                }
            }
        }
        mesh.flags = ws.vec(mesh.npolys as usize, 0u16)?;
        Ok((mesh, report))
    }

    /// Checks that the counts are non-negative and the arrays hold at least what
    /// the counts say; returns `(nverts, npolys, nvp)`.
    fn checked_layout(&self) -> Result<(usize, usize, usize), Error> {
        let nverts = usize::try_from(self.nverts)
            .map_err(|_| Error::InvalidInput("negative vertex count"))?;
        let npolys = usize::try_from(self.npolys)
            .map_err(|_| Error::InvalidInput("negative polygon count"))?;
        let nvp = usize::try_from(self.nvp)
            .ok()
            .filter(|n| (1..=0xffff).contains(n))
            .ok_or(Error::InvalidInput("vertices per polygon"))?;
        let vert_len = nverts.checked_mul(3).ok_or(Error::SizeOverflow)?;
        let poly_len = npolys
            .checked_mul(nvp)
            .and_then(|n| n.checked_mul(2))
            .ok_or(Error::SizeOverflow)?;
        if self.verts.len() < vert_len
            || self.polys.len() < poly_len
            || self.regs.len() < npolys
            || self.areas.len() < npolys
            || self.flags.len() < npolys
        {
            return Err(Error::InvalidInput(
                "polygon mesh arrays shorter than counts",
            ));
        }
        Ok((nverts, npolys, nvp))
    }

    /// Copies the mesh into new, exactly sized arrays (`rcCopyPolyMesh`).
    ///
    /// `maxpolys` of the copy is the polygon count, as upstream sets it. Arrays
    /// that are longer than the counts say (the over-allocation a build leaves)
    /// are trimmed; arrays that are shorter are rejected as
    /// [`Error::InvalidInput`], where upstream would read out of bounds.
    pub fn copy(&self, ws: &mut Workspace) -> Result<PolyMesh, Error> {
        ws.begin();
        let (nverts, npolys, nvp) = self.checked_layout()?;
        let vert_len = nverts * 3;
        let poly_len = npolys * nvp * 2;
        ws.work(u64::try_from(vert_len + poly_len + npolys * 3).map_err(|_| Error::SizeOverflow)?)?;
        let mut verts = ws.vec_with_capacity::<u16>(vert_len)?;
        verts.extend_from_slice(&self.verts[..vert_len]);
        let mut polys = ws.vec_with_capacity::<u16>(poly_len)?;
        polys.extend_from_slice(&self.polys[..poly_len]);
        let mut regs = ws.vec_with_capacity::<u16>(npolys)?;
        regs.extend_from_slice(&self.regs[..npolys]);
        let mut areas = ws.vec_with_capacity::<u8>(npolys)?;
        areas.extend_from_slice(&self.areas[..npolys]);
        let mut flags = ws.vec_with_capacity::<u16>(npolys)?;
        flags.extend_from_slice(&self.flags[..npolys]);
        Ok(PolyMesh {
            verts,
            polys,
            regs,
            flags,
            areas,
            nverts: self.nverts,
            npolys: self.npolys,
            maxpolys: self.npolys,
            nvp: self.nvp,
            bmin: self.bmin,
            bmax: self.bmax,
            cs: self.cs,
            ch: self.ch,
            border_size: self.border_size,
            max_edge_error: self.max_edge_error,
        })
    }

    /// Merges polygon meshes that share one grid into one mesh
    /// (`rcMergePolyMeshes`), for example the per-tile meshes of a tiled build.
    ///
    /// The meshes must agree on `nvp`; cell size, cell height and the `nvp` of the
    /// result come from the first mesh. Vertices within the same cell column and
    /// two cells of height are welded, neighbour links are rebuilt across the
    /// seams, and a tile's portal flags survive only on the sides that are also
    /// the outer boundary of the merged bounds. The result's `border_size` and
    /// `max_edge_error` are 0 and `maxpolys` is 0, as upstream leaves them.
    ///
    /// An empty list gives an empty mesh. Differences from upstream, which would
    /// read out of bounds or wrap: a polygon vertex index at or past its mesh's
    /// vertex count, `nvp` that differs between meshes, a cell offset outside
    /// `0..=65535`, or a result with more than 65,535 vertices or polygons is an
    /// error ([`Error::InvalidInput`], [`Error::FormatLimit`]).
    pub fn merge(ws: &mut Workspace, meshes: &[&PolyMesh]) -> Result<PolyMesh, Error> {
        ws.begin();
        let mut mesh = PolyMesh::default();
        let Some(first) = meshes.first() else {
            return Ok(mesh);
        };
        mesh.nvp = first.nvp;
        mesh.cs = first.cs;
        mesh.ch = first.ch;
        mesh.bmin = first.bmin;
        mesh.bmax = first.bmax;
        if !(mesh.cs > 0.0 && mesh.cs.is_finite()) {
            return Err(Error::InvalidInput("cell size"));
        }
        let mut max_verts = 0usize;
        let mut max_polys = 0usize;
        let mut max_verts_per_mesh = 0usize;
        let mut nvp_u = 0usize;
        for m in meshes {
            let (nv, np, nvp) = m.checked_layout()?;
            if m.nvp != first.nvp {
                return Err(Error::InvalidInput("meshes differ in vertices per polygon"));
            }
            nvp_u = nvp;
            if !m.bmin.iter().chain(&m.bmax).all(|x| x.is_finite()) {
                return Err(Error::InvalidInput("non-finite mesh bounds"));
            }
            for a in 0..3 {
                if m.bmin[a] < mesh.bmin[a] {
                    mesh.bmin[a] = m.bmin[a];
                }
                if m.bmax[a] > mesh.bmax[a] {
                    mesh.bmax[a] = m.bmax[a];
                }
            }
            max_verts_per_mesh = max_verts_per_mesh.max(nv);
            max_verts = max_verts.checked_add(nv).ok_or(Error::SizeOverflow)?;
            max_polys = max_polys.checked_add(np).ok_or(Error::SizeOverflow)?;
        }
        let slots = max_polys
            .checked_mul(2)
            .and_then(|n| n.checked_mul(nvp_u))
            .ok_or(Error::SizeOverflow)?;
        ws.work(u64::try_from(slots.saturating_add(max_verts)).map_err(|_| Error::SizeOverflow)?)?;
        mesh.verts = ws.vec(max_verts.checked_mul(3).ok_or(Error::SizeOverflow)?, 0u16)?;
        mesh.polys = ws.vec(slots, MESH_NULL_IDX)?;
        mesh.regs = ws.vec(max_polys, 0u16)?;
        mesh.areas = ws.vec(max_polys, 0u8)?;
        mesh.flags = ws.vec(max_polys, 0u16)?;
        let mut next_vert = ws.vec(max_verts, 0i32)?;
        let mut first_vert = ws.vec(VERTEX_BUCKET_COUNT, -1i32)?;
        let mut vremap = ws.vec(max_verts_per_mesh, 0u16)?;
        let mut nverts = 0usize;
        let mut npolys = 0usize;

        let cell_offset = |d: f32, cs: f32| -> Result<u16, Error> {
            let c = crate::common::floorf(d / cs + 0.5);
            if (0.0..=65535.0).contains(&c) {
                Ok(c as u16)
            } else {
                Err(Error::InvalidInput("mesh offset outside the merged bounds"))
            }
        };
        for pmesh in meshes {
            let ox = cell_offset(pmesh.bmin[0] - mesh.bmin[0], mesh.cs)?;
            let oz = cell_offset(pmesh.bmin[2] - mesh.bmin[2], mesh.cs)?;
            let is_min_x = ox == 0;
            let is_min_z = oz == 0;
            let is_max_x = cell_offset(mesh.bmax[0] - pmesh.bmax[0], mesh.cs).is_ok_and(|c| c == 0);
            let is_max_z = cell_offset(mesh.bmax[2] - pmesh.bmax[2], mesh.cs).is_ok_and(|c| c == 0);
            let is_on_border = is_min_x || is_min_z || is_max_x || is_max_z;

            let pn = pmesh.nverts as usize;
            for (j, remap) in vremap.iter_mut().enumerate().take(pn) {
                let v = &pmesh.verts[j * 3..j * 3 + 3];
                let x = u32::from(v[0]) + u32::from(ox);
                let z = u32::from(v[2]) + u32::from(oz);
                if x > 0xffff || z > 0xffff {
                    return Err(Error::FormatLimit("merged vertex outside the 16-bit grid"));
                }
                *remap = add_vertex(
                    x as u16,
                    v[1],
                    z as u16,
                    &mut mesh.verts,
                    &mut first_vert,
                    &mut next_vert,
                    &mut nverts,
                );
            }
            if nverts > 0xffff {
                return Err(Error::FormatLimit(
                    "merged mesh has more than 65535 vertices",
                ));
            }
            for j in 0..pmesh.npolys as usize {
                let tgt_base = npolys * 2 * nvp_u;
                let src = &pmesh.polys[j * 2 * nvp_u..(j + 1) * 2 * nvp_u];
                mesh.regs[npolys] = pmesh.regs[j];
                mesh.areas[npolys] = pmesh.areas[j];
                mesh.flags[npolys] = pmesh.flags[j];
                npolys += 1;
                let tgt = &mut mesh.polys[tgt_base..tgt_base + 2 * nvp_u];
                for k in 0..nvp_u {
                    if src[k] == MESH_NULL_IDX {
                        break;
                    }
                    let s = usize::from(src[k]);
                    if s >= pn {
                        return Err(Error::InvalidInput("polygon vertex index out of range"));
                    }
                    tgt[k] = vremap[s];
                }
                if is_on_border {
                    for k in nvp_u..nvp_u * 2 {
                        if src[k] & 0x8000 != 0 && src[k] != 0xffff {
                            let keep = match src[k] & 0xf {
                                0 => is_min_x,
                                1 => is_max_z,
                                2 => is_max_x,
                                3 => is_min_z,
                                _ => false,
                            };
                            if keep {
                                tgt[k] = src[k];
                            }
                        }
                    }
                }
            }
            if npolys > 0xffff {
                return Err(Error::FormatLimit(
                    "merged mesh has more than 65535 polygons",
                ));
            }
        }
        mesh.nverts = nverts as i32;
        mesh.npolys = npolys as i32;
        build_mesh_adjacency(ws, &mut mesh.polys, npolys, nverts, nvp_u)?;
        // Welding leaves unused tail room; keep exactly what the counts say.
        mesh.verts.truncate(nverts * 3);
        mesh.polys.truncate(npolys * 2 * nvp_u);
        mesh.regs.truncate(npolys);
        mesh.areas.truncate(npolys);
        mesh.flags.truncate(npolys);
        Ok(mesh)
    }
}

const VERTEX_BUCKET_COUNT: usize = 1 << 12;

#[inline(always)]
fn compute_vertex_hash(x: i32, y: i32, z: i32) -> usize {
    let h1: u32 = 0x8da6b343;
    let h2: u32 = 0xd8163841;
    let h3: u32 = 0xcb1ab31f;
    let n = h1
        .wrapping_mul(x as u32)
        .wrapping_add(h2.wrapping_mul(y as u32))
        .wrapping_add(h3.wrapping_mul(z as u32));
    (n as usize) & (VERTEX_BUCKET_COUNT - 1)
}

fn add_vertex(
    x: u16,
    y: u16,
    z: u16,
    verts: &mut [u16],
    first_vert: &mut [i32],
    next_vert: &mut [i32],
    nv: &mut usize,
) -> u16 {
    let bucket = compute_vertex_hash(i32::from(x), 0, i32::from(z));
    let mut i = first_vert[bucket];
    while i != -1 {
        let v = &verts[i as usize * 3..];
        if v[0] == x && (i32::from(v[1]) - i32::from(y)).abs() <= 2 && v[2] == z {
            return i as u16;
        }
        i = next_vert[i as usize];
    }
    let i = *nv;
    *nv += 1;
    verts[i * 3] = x;
    verts[i * 3 + 1] = y;
    verts[i * 3 + 2] = z;
    next_vert[i] = first_vert[bucket];
    first_vert[bucket] = i as i32;
    i as u16
}

#[inline(always)]
fn prev(i: usize, n: usize) -> usize {
    if i >= 1 { i - 1 } else { n - 1 }
}

#[inline(always)]
fn next(i: usize, n: usize) -> usize {
    if i + 1 < n { i + 1 } else { 0 }
}

#[inline(always)]
fn area2(a: &[i32], b: &[i32], c: &[i32]) -> i32 {
    // Wrapping like the C++ int arithmetic (only reachable on grids wider than ~46k cells).
    (b[0].wrapping_sub(a[0]))
        .wrapping_mul(c[2].wrapping_sub(a[2]))
        .wrapping_sub((c[0].wrapping_sub(a[0])).wrapping_mul(b[2].wrapping_sub(a[2])))
}

#[inline(always)]
fn left(a: &[i32], b: &[i32], c: &[i32]) -> bool {
    area2(a, b, c) < 0
}

#[inline(always)]
fn left_on(a: &[i32], b: &[i32], c: &[i32]) -> bool {
    area2(a, b, c) <= 0
}

#[inline(always)]
fn collinear(a: &[i32], b: &[i32], c: &[i32]) -> bool {
    area2(a, b, c) == 0
}

fn intersect_prop(a: &[i32], b: &[i32], c: &[i32], d: &[i32]) -> bool {
    if collinear(a, b, c) || collinear(a, b, d) || collinear(c, d, a) || collinear(c, d, b) {
        return false;
    }
    (left(a, b, c) ^ left(a, b, d)) && (left(c, d, a) ^ left(c, d, b))
}

fn between(a: &[i32], b: &[i32], c: &[i32]) -> bool {
    if !collinear(a, b, c) {
        return false;
    }
    if a[0] != b[0] {
        (a[0] <= c[0] && c[0] <= b[0]) || (a[0] >= c[0] && c[0] >= b[0])
    } else {
        (a[2] <= c[2] && c[2] <= b[2]) || (a[2] >= c[2] && c[2] >= b[2])
    }
}

fn intersect(a: &[i32], b: &[i32], c: &[i32], d: &[i32]) -> bool {
    intersect_prop(a, b, c, d)
        || between(a, b, c)
        || between(a, b, d)
        || between(c, d, a)
        || between(c, d, b)
}

#[inline(always)]
fn vequal(a: &[i32], b: &[i32]) -> bool {
    a[0] == b[0] && a[2] == b[2]
}

#[inline(always)]
fn vtx<'a>(verts: &'a [i32], indices: &[i32], i: usize) -> &'a [i32] {
    let k = (indices[i] & 0x0fff_ffff) as usize * 4;
    &verts[k..k + 4]
}

fn diagonalie(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32], strict: bool) -> bool {
    let d0 = vtx(verts, indices, i);
    let d1 = vtx(verts, indices, j);
    for k in 0..n {
        let k1 = next(k, n);
        if !(k == i || k1 == i || k == j || k1 == j) {
            let p0 = vtx(verts, indices, k);
            let p1 = vtx(verts, indices, k1);
            if vequal(d0, p0) || vequal(d1, p0) || vequal(d0, p1) || vequal(d1, p1) {
                continue;
            }
            let hit = if strict {
                intersect(d0, d1, p0, p1)
            } else {
                intersect_prop(d0, d1, p0, p1)
            };
            if hit {
                return false;
            }
        }
    }
    true
}

fn in_cone(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32]) -> bool {
    let pi = vtx(verts, indices, i);
    let pj = vtx(verts, indices, j);
    let pi1 = vtx(verts, indices, next(i, n));
    let pin1 = vtx(verts, indices, prev(i, n));
    if left_on(pin1, pi, pi1) {
        return left(pi, pj, pin1) && left(pj, pi, pi1);
    }
    !(left_on(pi, pj, pi1) && left_on(pj, pi, pin1))
}

fn in_cone_loose(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32]) -> bool {
    let pi = vtx(verts, indices, i);
    let pj = vtx(verts, indices, j);
    let pi1 = vtx(verts, indices, next(i, n));
    let pin1 = vtx(verts, indices, prev(i, n));
    if left_on(pin1, pi, pi1) {
        return left_on(pi, pj, pin1) && left_on(pj, pi, pi1);
    }
    !(left_on(pi, pj, pi1) && left_on(pj, pi, pin1))
}

fn diagonal(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32]) -> bool {
    in_cone(i, j, n, verts, indices) && diagonalie(i, j, n, verts, indices, true)
}

fn diagonal_loose(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32]) -> bool {
    in_cone_loose(i, j, n, verts, indices) && diagonalie(i, j, n, verts, indices, false)
}

const FLAG: i32 = 0x8000_0000_u32 as i32;

/// Ear-clipping triangulation (`triangulate`); returns the triangle count, negated
/// when the polygon could not be fully triangulated.
pub(crate) fn triangulate(
    mut n: usize,
    verts: &[i32],
    indices: &mut [i32],
    tris: &mut [i32],
) -> i32 {
    if n < 3 {
        // Upstream reads past the index array here.
        return 0;
    }
    let mut ntris = 0i32;
    let mut dst = 0usize;
    for i in 0..n {
        let i1 = next(i, n);
        let i2 = next(i1, n);
        if diagonal(i, i2, n, verts, indices) {
            indices[i1] |= FLAG;
        }
    }
    while n > 3 {
        let mut min_len = -1i32;
        let mut mini: Option<usize> = None;
        for i in 0..n {
            let i1 = next(i, n);
            if indices[i1] & FLAG != 0 {
                let p0 = vtx(verts, indices, i);
                let p2 = vtx(verts, indices, next(i1, n));
                let dx = p2[0] - p0[0];
                let dy = p2[2] - p0[2];
                let len = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
                if min_len < 0 || len < min_len {
                    min_len = len;
                    mini = Some(i);
                }
            }
        }
        if mini.is_none() {
            min_len = -1;
            for i in 0..n {
                let i1 = next(i, n);
                let i2 = next(i1, n);
                if diagonal_loose(i, i2, n, verts, indices) {
                    let p0 = vtx(verts, indices, i);
                    let p2 = vtx(verts, indices, next(i2, n));
                    let dx = p2[0] - p0[0];
                    let dy = p2[2] - p0[2];
                    let len = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
                    if min_len < 0 || len < min_len {
                        min_len = len;
                        mini = Some(i);
                    }
                }
            }
            if mini.is_none() {
                return -ntris;
            }
        }
        let i = mini.unwrap_or(0);
        let mut i1 = next(i, n);
        let i2 = next(i1, n);
        tris[dst] = indices[i] & 0x0fff_ffff;
        tris[dst + 1] = indices[i1] & 0x0fff_ffff;
        tris[dst + 2] = indices[i2] & 0x0fff_ffff;
        dst += 3;
        ntris += 1;
        n -= 1;
        for k in i1..n {
            indices[k] = indices[k + 1];
        }
        if i1 >= n {
            i1 = 0;
        }
        let i = prev(i1, n);
        if diagonal(prev(i, n), i1, n, verts, indices) {
            indices[i] |= FLAG;
        } else {
            indices[i] &= 0x0fff_ffff;
        }
        if diagonal(i, next(i1, n), n, verts, indices) {
            indices[i1] |= FLAG;
        } else {
            indices[i1] &= 0x0fff_ffff;
        }
    }
    tris[dst] = indices[0] & 0x0fff_ffff;
    tris[dst + 1] = indices[1] & 0x0fff_ffff;
    tris[dst + 2] = indices[2] & 0x0fff_ffff;
    ntris + 1
}

#[inline(always)]
fn count_poly_verts(p: &[u16], nvp: usize) -> usize {
    p[..nvp]
        .iter()
        .position(|&v| v == MESH_NULL_IDX)
        .unwrap_or(nvp)
}

#[inline(always)]
fn uleft(a: &[u16], b: &[u16], c: &[u16]) -> bool {
    let d = |p: &[u16], q: &[u16], k: usize| i32::from(p[k]) - i32::from(q[k]);
    d(b, a, 0)
        .wrapping_mul(d(c, a, 2))
        .wrapping_sub(d(c, a, 0).wrapping_mul(d(b, a, 2)))
        < 0
}

fn get_poly_merge_value(pa: &[u16], pb: &[u16], verts: &[u16], nvp: usize) -> (i32, usize, usize) {
    let na = count_poly_verts(pa, nvp);
    let nb = count_poly_verts(pb, nvp);
    if na + nb < 2 || na + nb - 2 > nvp {
        return (-1, 0, 0);
    }
    let mut ea: Option<usize> = None;
    let mut eb: Option<usize> = None;
    for i in 0..na {
        let mut va0 = pa[i];
        let mut va1 = pa[(i + 1) % na];
        if va0 > va1 {
            core::mem::swap(&mut va0, &mut va1);
        }
        for j in 0..nb {
            let mut vb0 = pb[j];
            let mut vb1 = pb[(j + 1) % nb];
            if vb0 > vb1 {
                core::mem::swap(&mut vb0, &mut vb1);
            }
            if va0 == vb0 && va1 == vb1 {
                ea = Some(i);
                eb = Some(j);
                break;
            }
        }
    }
    let (Some(ea), Some(eb)) = (ea, eb) else {
        return (-1, 0, 0);
    };
    let v = |k: u16| &verts[usize::from(k) * 3..usize::from(k) * 3 + 3];
    if !uleft(v(pa[(ea + na - 1) % na]), v(pa[ea]), v(pb[(eb + 2) % nb])) {
        return (-1, 0, 0);
    }
    if !uleft(v(pb[(eb + nb - 1) % nb]), v(pb[eb]), v(pa[(ea + 2) % na])) {
        return (-1, 0, 0);
    }
    let va = v(pa[ea]);
    let vb = v(pa[(ea + 1) % na]);
    let dx = i32::from(va[0]) - i32::from(vb[0]);
    let dy = i32::from(va[2]) - i32::from(vb[2]);
    (
        dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)),
        ea,
        eb,
    )
}

/// Merges polygon `b` into polygon `a` (`mergePolyVerts`); both are `nvp`-wide rows of `polys`.
fn merge_poly_verts(
    ws: &mut Workspace,
    polys: &mut [u16],
    a: usize,
    b: usize,
    ea: usize,
    eb: usize,
    nvp: usize,
) -> Result<(), Error> {
    let pa = &polys[a * nvp..(a + 1) * nvp];
    let pb = &polys[b * nvp..(b + 1) * nvp];
    let na = count_poly_verts(pa, nvp);
    let nb = count_poly_verts(pb, nvp);
    let mut tmp = [MESH_NULL_IDX; 64];
    let mut tmpv;
    let tmp: &mut [u16] = if nvp <= 64 {
        &mut tmp[..nvp]
    } else {
        tmpv = ws.vec(nvp, MESH_NULL_IDX)?;
        &mut tmpv[..]
    };
    let mut n = 0;
    for i in 0..na.saturating_sub(1) {
        tmp[n] = pa[(ea + 1 + i) % na];
        n += 1;
    }
    for i in 0..nb.saturating_sub(1) {
        tmp[n] = pb[(eb + 1 + i) % nb];
        n += 1;
    }
    polys[a * nvp..(a + 1) * nvp].copy_from_slice(tmp);
    Ok(())
}

/// The greedy merge loop shared by `rcBuildPolyMesh` and `removeVertex`.
/// `regs_areas`, when given, are moved and combined along with the polygons.
/// Greedy merge of `polys` (upstream's merge loop in `rcBuildPolyMesh` and
/// `removeVertex`). Each pass compares every pair, so a large contour costs
/// O(n^3); every pass is charged to `ws`.
fn merge_polys(
    ws: &mut Workspace,
    polys: &mut [u16],
    npolys: &mut usize,
    nvp: usize,
    verts: &[u16],
    mut regs_areas: Option<(&mut [u16], &mut [u8])>,
) -> Result<(), Error> {
    loop {
        let n = *npolys as u64;
        ws.work(n * n.saturating_sub(1) / 2 * nvp as u64)?;
        let mut best = 0i32;
        let (mut bpa, mut bpb, mut bea, mut beb) = (0, 0, 0, 0);
        for j in 0..npolys.saturating_sub(1) {
            for k in j + 1..*npolys {
                let (v, ea, eb) = get_poly_merge_value(
                    &polys[j * nvp..(j + 1) * nvp],
                    &polys[k * nvp..(k + 1) * nvp],
                    verts,
                    nvp,
                );
                if v > best {
                    best = v;
                    bpa = j;
                    bpb = k;
                    bea = ea;
                    beb = eb;
                }
            }
        }
        if best <= 0 {
            break;
        }
        merge_poly_verts(ws, polys, bpa, bpb, bea, beb, nvp)?;
        let last = *npolys - 1;
        if let Some((regs, areas)) = regs_areas.as_mut() {
            if regs[bpa] != regs[bpb] {
                regs[bpa] = MULTIPLE_REGS;
            }
            regs[bpb] = regs[last];
            areas[bpb] = areas[last];
        }
        if bpb != last {
            polys.copy_within(last * nvp..(last + 1) * nvp, bpb * nvp);
        }
        *npolys -= 1;
    }
    Ok(())
}

fn can_remove_vertex(ws: &mut Workspace, mesh: &PolyMesh, rem: u16) -> Result<bool, Error> {
    let nvp = mesh.nvp as usize;
    let mut num_touched = 0usize;
    let mut num_remaining_edges = 0i32;
    for i in 0..mesh.npolys as usize {
        let p = &mesh.polys[i * nvp * 2..];
        let nv = count_poly_verts(p, nvp);
        let removed = p[..nv].iter().filter(|&&v| v == rem).count();
        num_touched += removed;
        if removed > 0 {
            num_remaining_edges += nv as i32 - (removed as i32 + 1);
        }
    }
    if num_remaining_edges <= 2 {
        return Ok(false);
    }
    let mut edges: Vec<[i32; 3]> =
        ws.vec_with_capacity(num_touched.checked_mul(2).ok_or(Error::SizeOverflow)?)?;
    for i in 0..mesh.npolys as usize {
        let p = &mesh.polys[i * nvp * 2..];
        let nv = count_poly_verts(p, nvp);
        let mut k = nv.wrapping_sub(1);
        for j in 0..nv {
            if p[j] == rem || p[k] == rem {
                let (mut a, mut b) = (i32::from(p[j]), i32::from(p[k]));
                if b == i32::from(rem) {
                    core::mem::swap(&mut a, &mut b);
                }
                let mut exists = false;
                for e in edges.iter_mut() {
                    if e[1] == b {
                        e[2] += 1;
                        exists = true;
                    }
                }
                if !exists {
                    edges.push([a, b, 1]);
                }
            }
            k = j;
        }
    }
    Ok(edges.iter().filter(|e| e[2] < 2).count() <= 2)
}

fn remove_vertex(
    ws: &mut Workspace,
    mesh: &mut PolyMesh,
    rem: u16,
    max_tris: usize,
) -> Result<(), Error> {
    let nvp = mesh.nvp as usize;
    let mut edges: Vec<[i32; 4]> = Vec::new();
    let mut i = 0usize;
    while i < mesh.npolys as usize {
        let base = i * nvp * 2;
        let nv = count_poly_verts(&mesh.polys[base..], nvp);
        if !mesh.polys[base..base + nv].contains(&rem) {
            i += 1;
            continue;
        }
        let mut k = nv.wrapping_sub(1);
        for j in 0..nv {
            let (pj, pk) = (mesh.polys[base + j], mesh.polys[base + k]);
            if pj != rem && pk != rem {
                ws.push(
                    &mut edges,
                    [
                        i32::from(pk),
                        i32::from(pj),
                        i32::from(mesh.regs[i]),
                        i32::from(mesh.areas[i]),
                    ],
                )?;
            }
            k = j;
        }
        let last = mesh.npolys as usize - 1;
        if i != last {
            mesh.polys
                .copy_within(last * nvp * 2..last * nvp * 2 + nvp, base);
        }
        mesh.polys[base + nvp..base + 2 * nvp].fill(MESH_NULL_IDX);
        mesh.regs[i] = mesh.regs[last];
        mesh.areas[i] = mesh.areas[last];
        mesh.npolys -= 1;
    }
    let nverts = mesh.nverts as usize;
    let r = usize::from(rem);
    mesh.verts.copy_within((r + 1) * 3..nverts * 3, r * 3);
    mesh.nverts -= 1;
    for i in 0..mesh.npolys as usize {
        let base = i * nvp * 2;
        let nv = count_poly_verts(&mesh.polys[base..], nvp);
        for v in &mut mesh.polys[base..base + nv] {
            if *v > rem {
                *v -= 1;
            }
        }
    }
    for e in edges.iter_mut() {
        if e[0] > i32::from(rem) {
            e[0] -= 1;
        }
        if e[1] > i32::from(rem) {
            e[1] -= 1;
        }
    }
    if edges.is_empty() {
        return Ok(());
    }
    let capacity = edges.len().checked_add(1).ok_or(Error::SizeOverflow)?;
    let mut hole = ws.vec_with_capacity(capacity)?;
    let mut hreg = ws.vec_with_capacity(capacity)?;
    let mut harea = ws.vec_with_capacity(capacity)?;
    hole.push(edges[0][0]);
    hreg.push(edges[0][2]);
    harea.push(edges[0][3]);
    while !edges.is_empty() {
        let mut matched = false;
        let mut i = 0usize;
        while i < edges.len() {
            let [ea, eb, r, a] = edges[i];
            let mut add = false;
            if hole[0] == eb {
                hole.insert(0, ea);
                hreg.insert(0, r);
                harea.insert(0, a);
                add = true;
            } else if hole[hole.len() - 1] == ea {
                hole.push(eb);
                hreg.push(r);
                harea.push(a);
                add = true;
            }
            if add {
                edges.swap_remove(i);
                matched = true;
            } else {
                i += 1;
            }
        }
        if !matched {
            break;
        }
    }
    let nhole = hole.len();
    let mut tverts = ws.vec(nhole * 4, 0i32)?;
    let mut thole: Vec<i32> = ws.vec(nhole, 0i32)?;
    for (i, &pi) in hole.iter().enumerate() {
        thole[i] = i as i32;
        let v = &mesh.verts[pi as usize * 3..pi as usize * 3 + 3];
        tverts[i * 4] = i32::from(v[0]);
        tverts[i * 4 + 1] = i32::from(v[1]);
        tverts[i * 4 + 2] = i32::from(v[2]);
    }
    let mut tris = ws.vec(nhole * 3, 0i32)?;
    let mut ntris = triangulate(nhole, &tverts, &mut thole, &mut tris);
    if ntris < 0 {
        ntris = -ntris;
    }
    let ntris = ntris as usize;
    let mut polys = ws.vec((ntris + 1) * nvp, MESH_NULL_IDX)?;
    let mut pregs = ws.vec(ntris, 0u16)?;
    let mut pareas = ws.vec(ntris, 0u8)?;
    let mut npolys = 0usize;
    for t in tris[..ntris * 3].chunks_exact(3) {
        if t[0] != t[1] && t[0] != t[2] && t[1] != t[2] {
            let (a, b, c) = (t[0] as usize, t[1] as usize, t[2] as usize);
            polys[npolys * nvp] = hole[a] as u16;
            polys[npolys * nvp + 1] = hole[b] as u16;
            polys[npolys * nvp + 2] = hole[c] as u16;
            pregs[npolys] = if hreg[a] != hreg[b] || hreg[b] != hreg[c] {
                MULTIPLE_REGS
            } else {
                hreg[a] as u16
            };
            pareas[npolys] = harea[a] as u8;
            npolys += 1;
        }
    }
    if npolys == 0 {
        return Ok(());
    }
    if nvp > 3 {
        merge_polys(
            ws,
            &mut polys,
            &mut npolys,
            nvp,
            &mesh.verts,
            Some((&mut pregs, &mut pareas)),
        )?;
    }
    for i in 0..npolys {
        let n = mesh.npolys as usize;
        if n >= max_tris {
            break;
        }
        let base = n * nvp * 2;
        mesh.polys[base..base + nvp * 2].fill(MESH_NULL_IDX);
        mesh.polys[base..base + nvp].copy_from_slice(&polys[i * nvp..(i + 1) * nvp]);
        mesh.regs[n] = pregs[i];
        mesh.areas[n] = pareas[i];
        mesh.npolys += 1;
    }
    Ok(())
}

fn build_mesh_adjacency(
    ws: &mut Workspace,
    polys: &mut [u16],
    npolys: usize,
    nverts: usize,
    nvp: usize,
) -> Result<(), Error> {
    #[derive(Clone, Copy, Default)]
    struct Edge {
        vert: [u16; 2],
        poly_edge: [u16; 2],
        poly: [u16; 2],
    }
    let max_edges = npolys.checked_mul(nvp).ok_or(Error::SizeOverflow)?;
    ws.work(u64::try_from(max_edges).map_err(|_| Error::SizeOverflow)?)?;
    let mut first_edge = ws.vec(nverts, MESH_NULL_IDX)?;
    let mut next_edge = ws.vec(max_edges, 0u16)?;
    let mut edges: Vec<Edge> = ws.vec_with_capacity(max_edges)?;
    for i in 0..npolys {
        let t = &polys[i * nvp * 2..];
        for j in 0..nvp {
            if t[j] == MESH_NULL_IDX {
                break;
            }
            let v0 = t[j];
            let v1 = if j + 1 >= nvp || t[j + 1] == MESH_NULL_IDX {
                t[0]
            } else {
                t[j + 1]
            };
            if v0 < v1 {
                let e = edges.len();
                // 0xffff is the chain terminator, so it cannot name an edge.
                if e >= usize::from(MESH_NULL_IDX) {
                    return Err(Error::LimitExceeded);
                }
                edges.push(Edge {
                    vert: [v0, v1],
                    poly: [i as u16, i as u16],
                    poly_edge: [j as u16, 0],
                });
                next_edge[e] = first_edge[usize::from(v0)];
                first_edge[usize::from(v0)] = e as u16;
            }
        }
    }
    for i in 0..npolys {
        let t = &polys[i * nvp * 2..];
        for j in 0..nvp {
            if t[j] == MESH_NULL_IDX {
                break;
            }
            let v0 = t[j];
            let v1 = if j + 1 >= nvp || t[j + 1] == MESH_NULL_IDX {
                t[0]
            } else {
                t[j + 1]
            };
            if v0 > v1 {
                let mut e = first_edge[usize::from(v1)];
                while e != MESH_NULL_IDX {
                    let edge = &mut edges[usize::from(e)];
                    if edge.vert[1] == v0 && edge.poly[0] == edge.poly[1] {
                        edge.poly[1] = i as u16;
                        edge.poly_edge[1] = j as u16;
                        break;
                    }
                    e = next_edge[usize::from(e)];
                }
            }
        }
    }
    for e in &edges {
        if e.poly[0] != e.poly[1] {
            let p0 = usize::from(e.poly[0]) * nvp * 2;
            let p1 = usize::from(e.poly[1]) * nvp * 2;
            polys[p0 + nvp + usize::from(e.poly_edge[0])] = e.poly[1];
            polys[p1 + nvp + usize::from(e.poly_edge[1])] = e.poly[0];
        }
    }
    Ok(())
}

#[cfg(test)]
mod adjacency_tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn adjacency_charges_work_before_any_allocation() {
        let mut polys = [0, 1, 2, MESH_NULL_IDX, MESH_NULL_IDX, MESH_NULL_IDX];
        let mut ws = Workspace::with_limits(Limits {
            max_work: 0,
            ..Limits::default()
        });
        assert_eq!(
            build_mesh_adjacency(&mut ws, &mut polys, 1, 3, 3),
            Err(Error::LimitExceeded)
        );
        assert_eq!(ws.usage().bytes, 0);
    }

    #[test]
    fn adjacency_rejects_edge_sentinel_and_wrap() {
        // Ascending sides contribute two edges per triangle; the final reverse
        // triangle contributes one. All chains stay short at the exact boundary.
        let mut polys = Vec::new();
        for i in 0..32767u16 {
            polys.extend_from_slice(&[
                i,
                i + 1,
                i + 2,
                MESH_NULL_IDX,
                MESH_NULL_IDX,
                MESH_NULL_IDX,
            ]);
        }
        polys.extend_from_slice(&[
            32768,
            32767,
            32766,
            MESH_NULL_IDX,
            MESH_NULL_IDX,
            MESH_NULL_IDX,
        ]);
        build_mesh_adjacency(&mut Workspace::new(), &mut polys, 32768, 32769, 3).unwrap();
        // Reversing the final triangle adds edge 65,535, the null sentinel.
        // There are fewer than 65,534 vertices: a vertex guard cannot protect it.
        let base = 32767 * 6;
        polys[base..base + 3].copy_from_slice(&[32766, 32767, 32768]);
        assert_eq!(
            build_mesh_adjacency(&mut Workspace::new(), &mut polys, 32768, 32769, 3),
            Err(Error::LimitExceeded)
        );
    }
}
