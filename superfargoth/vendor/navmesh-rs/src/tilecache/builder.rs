//! Layer meshing, translated from DetourTileCacheBuilder.cpp.
use super::Layer;
use crate::{Error, Workspace};
use alloc::vec::Vec;
const NULL: u16 = 0xffff;
fn byte(v: usize) -> Result<u8, Error> {
    u8::try_from(v).map_err(|_| Error::FormatLimit("layer byte"))
}
fn short(v: usize) -> Result<u16, Error> {
    u16::try_from(v).map_err(|_| Error::FormatLimit("layer index"))
}
fn charge(ws: &mut Workspace, n: usize) -> Result<(), Error> {
    ws.work(u64::try_from(n).map_err(|_| Error::SizeOverflow)?)
}
#[derive(Clone, Default)]
struct Sweep {
    ns: u16,
    id: u8,
    nei: u8,
}
#[derive(Clone, Default)]
struct Region {
    area: usize,
    neis: Vec<usize>,
    id: usize,
    area_id: u8,
}
fn connected(l: &Layer, a: usize, b: usize, climb: i32) -> bool {
    l.areas[a] == l.areas[b] && (i32::from(l.heights[a]) - i32::from(l.heights[b])).abs() <= climb
}
/// Partitions and merges layer regions in upstream order.
pub fn build_regions(ws: &mut Workspace, l: &mut Layer, climb: i32) -> Result<(), Error> {
    ws.begin();
    regions(ws, l, climb)
}
pub(crate) fn regions(ws: &mut Workspace, l: &mut Layer, climb: i32) -> Result<(), Error> {
    if climb < 0 {
        return Err(Error::InvalidParameter("walkable climb"));
    }
    let w = usize::from(l.header.width);
    let h = usize::from(l.header.height);
    l.regions_built = false;
    l.regs.fill(255);
    l.reg_count = 0;
    let mut sweeps = ws.vec(w, Sweep::default())?;
    let mut rid = 0usize;
    for y in 0..h {
        charge(ws, w * 2)?;
        let mut prev = [0u16; 256];
        let mut sid = 0usize;
        for x in 0..w {
            let i = x + y * w;
            if l.areas[i] == 0 {
                continue;
            }
            let mut s = 255usize;
            if x > 0 && connected(l, i, i - 1, climb) {
                s = usize::from(l.regs[i - 1]);
            }
            if s == 255 {
                s = sid;
                sid += 1;
                sweeps[s].nei = 255;
                sweeps[s].ns = 0;
            }
            if y > 0 && connected(l, i, i - w, climb) {
                let nr = l.regs[i - w];
                if nr != 255 {
                    if sweeps[s].ns == 0 {
                        sweeps[s].nei = nr;
                    }
                    if sweeps[s].nei == nr {
                        sweeps[s].ns += 1;
                        prev[usize::from(nr)] += 1;
                    } else {
                        sweeps[s].nei = 255;
                    }
                }
            }
            l.regs[i] = byte(s)?;
        }
        for s in &mut sweeps[..sid] {
            if s.nei != 255 && prev[usize::from(s.nei)] == s.ns {
                s.id = s.nei;
            } else {
                if rid == 255 {
                    return Err(Error::FormatLimit("layer regions"));
                }
                s.id = byte(rid)?;
                rid += 1;
            }
        }
        for x in 0..w {
            let i = x + y * w;
            if l.regs[i] != 255 {
                l.regs[i] = sweeps[usize::from(l.regs[i])].id;
            }
        }
    }
    let mut regs = ws.vec(rid, Region::default())?;
    for (i, r) in regs.iter_mut().enumerate() {
        r.id = i;
    }
    for y in 0..h {
        for x in 0..w {
            charge(ws, 1)?;
            let i = x + y * w;
            let ri = usize::from(l.regs[i]);
            if ri == 255 {
                continue;
            }
            regs[ri].area += 1;
            regs[ri].area_id = l.areas[i];
            if y > 0 && connected(l, i, i - w, climb) {
                let rj = usize::from(l.regs[i - w]);
                if rj != 255 && rj != ri {
                    for (a, b) in [(ri, rj), (rj, ri)] {
                        if regs[a].neis.last() != Some(&b) {
                            if regs[a].neis.len() == 16 {
                                return Err(Error::FormatLimit("layer region neighbours"));
                            }
                            ws.push(&mut regs[a].neis, b)?;
                        }
                    }
                }
            }
        }
    }
    // Indexed directed boundary counts implement canMerge without rescanning
    // every region for each candidate. Group membership uses intrusive lists.
    charge(ws, rid * rid)?;
    let mut links = ws.vec(rid * rid, 0u16)?;
    let mut heads = [usize::MAX; 256];
    let mut tails = [usize::MAX; 256];
    let mut next = [usize::MAX; 256];
    for i in 0..rid {
        heads[i] = i;
        tails[i] = i;
        for &j in &regs[i].neis {
            charge(ws, 1)?;
            links[i * rid + j] += 1;
        }
    }
    for i in 0..rid {
        let mut merge = None;
        let mut area = 0;
        for j in 0..regs[i].neis.len() {
            charge(ws, 1)?;
            let nei = regs[i].neis[j];
            if regs[i].id == regs[nei].id
                || regs[i].area_id != regs[nei].area_id
                || regs[nei].area <= area
            {
                continue;
            }
            if links[regs[i].id * rid + regs[nei].id] == 1 {
                area = regs[nei].area;
                merge = Some(nei);
            }
        }
        if let Some(nei) = merge {
            let old = regs[i].id;
            let new = regs[nei].id;
            charge(ws, rid * 4)?;
            for k in 0..rid {
                if k == old || k == new {
                    continue;
                }
                let row = links[old * rid + k];
                let col = links[k * rid + old];
                links[new * rid + k] += row;
                links[k * rid + new] += col;
                links[old * rid + k] = 0;
                links[k * rid + old] = 0;
            }
            let internal = links[old * rid + old] + links[old * rid + new] + links[new * rid + old];
            links[new * rid + new] += internal;
            links[old * rid + old] = 0;
            links[old * rid + new] = 0;
            links[new * rid + old] = 0;
            let mut member = heads[old];
            while member != usize::MAX {
                charge(ws, 1)?;
                regs[member].id = new;
                member = next[member];
            }
            next[tails[new]] = heads[old];
            tails[new] = tails[old];
            heads[old] = usize::MAX;
            tails[old] = usize::MAX;
        }
    }
    let mut remap = [0u8; 256];
    for r in &regs {
        remap[r.id] = 1;
    }
    let mut count = 0usize;
    for r in &mut remap {
        if *r != 0 {
            *r = byte(count)?;
            count += 1;
        }
    }
    for r in &mut regs {
        r.id = usize::from(remap[r.id]);
    }
    l.reg_count = byte(count)?;
    charge(ws, w * h)?;
    for r in &mut l.regs {
        if *r != 255 {
            *r = byte(regs[usize::from(*r)].id)?;
        }
    }
    l.regions_built = true;
    Ok(())
}
/// A contour vertex carries x, height, z, and portal/removal flags.
///
/// ```compile_fail
/// use navmesh_rs::tilecache::LayerContour;
/// let LayerContour { reg, area, verts } = LayerContour::new();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct LayerContour {
    /// Region id.
    pub reg: u8,
    /// Area id.
    pub area: u8,
    /// Four bytes per vertex.
    pub verts: Vec<u8>,
}
impl LayerContour {
    /// Creates an empty contour; customize its public fields before building.
    pub fn new() -> Self {
        Self::default()
    }
}
/// Outlines, ordered by region id.
///
/// ```compile_fail
/// use navmesh_rs::tilecache::LayerContourSet;
/// let LayerContourSet { contours } = LayerContourSet::new();
/// ```
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct LayerContourSet {
    /// One contour slot per region.
    pub contours: Vec<LayerContour>,
}
impl LayerContourSet {
    /// Creates an empty set; append contours through its public field.
    pub fn new() -> Self {
        Self::default()
    }
}
fn neighbour(l: &Layer, x: usize, y: usize, dir: usize) -> u8 {
    let w = usize::from(l.header.width);
    let i = x + y * w;
    if l.cons[i] & (1 << dir) == 0 {
        return if (l.cons[i] >> 4) & (1 << dir) != 0 {
            0xf8 + u8::try_from(dir).unwrap()
        } else {
            255
        };
    }
    let n = match dir {
        0 => i - 1,
        1 => i + w,
        2 => i + 1,
        _ => i - w,
    };
    l.regs[n]
}
fn append(v: &mut Vec<[u8; 4]>, p: [u8; 4], cap: usize) -> Result<(), Error> {
    let n = v.len();
    if n > 1
        && v[n - 1][3] == p[3]
        && ((v[n - 2][0] == v[n - 1][0] && v[n - 1][0] == p[0])
            || (v[n - 2][2] == v[n - 1][2] && v[n - 1][2] == p[2]))
    {
        v[n - 1] = p;
        return Ok(());
    }
    if n == cap {
        return Err(Error::BufferTooSmall);
    }
    v.push(p);
    Ok(())
}
fn distance(p: [u8; 4], a: [u8; 4], b: [u8; 4]) -> f32 {
    let px = f32::from(a[0]);
    let pz = f32::from(a[2]);
    let x = f32::from(p[0]);
    let z = f32::from(p[2]);
    let qx = f32::from(b[0]) - px;
    let qz = f32::from(b[2]) - pz;
    let dx = x - px;
    let dz = z - pz;
    let d = qx * qx + qz * qz;
    let mut t = qx * dx + qz * dz;
    if d > 0.0 {
        t /= d;
    }
    t = t.clamp(0.0, 1.0);
    let dx = px + t * qx - x;
    let dz = pz + t * qz - z;
    dx * dx + dz * dz
}
/// Traces and simplifies region outlines.
pub fn build_contours(
    ws: &mut Workspace,
    l: &Layer,
    climb: i32,
    max_error: f32,
) -> Result<LayerContourSet, Error> {
    ws.begin();
    contours(ws, l, climb, max_error)
}
pub(crate) fn contours(
    ws: &mut Workspace,
    l: &Layer,
    climb: i32,
    max_error: f32,
) -> Result<LayerContourSet, Error> {
    if climb < 0 || !max_error.is_finite() || max_error < 0.0 {
        return Err(Error::InvalidParameter("contour settings"));
    }
    if !l.regions_built {
        return Err(Error::InvalidInput("layer regions not built"));
    }
    let w = usize::from(l.header.width);
    let h = usize::from(l.header.height);
    let cap = (w + h) * 4;
    let mut out = LayerContourSet {
        contours: ws.vec(usize::from(l.reg_count), LayerContour::default())?,
    };
    let mut raw = ws.vec_with_capacity::<[u8; 4]>(cap)?;
    let mut poly = ws.vec_with_capacity::<usize>(cap)?;
    for y0 in 0..h {
        for x0 in 0..w {
            charge(ws, 1)?;
            let i = x0 + y0 * w;
            let ri = l.regs[i];
            if ri == 255 {
                continue;
            }
            let cont = &mut out.contours[usize::from(ri)];
            if !cont.verts.is_empty() {
                continue;
            }
            cont.reg = ri;
            cont.area = l.areas[i];
            raw.clear();
            poly.clear();
            let Some(sd) = (0..4)
                .map(|d| (d + 3) & 3)
                .find(|&d| neighbour(l, x0, y0, d) != ri)
            else {
                continue;
            };
            let (mut x, mut y, mut dir) = (x0, y0, sd);
            for iter in 0..w * h {
                charge(ws, 1)?;
                let rn = neighbour(l, x, y, dir);
                let (mut nx, mut ny) = (x, y);
                let nd = if rn != l.regs[x + y * w] {
                    let (px, pz) = match dir {
                        0 => (x, y + 1),
                        1 => (x + 1, y + 1),
                        _ => (x + usize::from(dir == 2), y),
                    };
                    append(
                        &mut raw,
                        [byte(px)?, l.heights[x + y * w], byte(pz)?, rn],
                        cap,
                    )?;
                    (dir + 1) & 3
                } else {
                    match dir {
                        0 => nx -= 1,
                        1 => ny += 1,
                        2 => nx += 1,
                        _ => ny -= 1,
                    };
                    (dir + 3) & 3
                };
                if iter > 0 && x == x0 && y == y0 && dir == sd {
                    break;
                }
                x = nx;
                y = ny;
                dir = nd;
            }
            if raw.len() > 1
                && raw.last().map(|p| (p[0], p[2])) == raw.first().map(|p| (p[0], p[2]))
            {
                raw.pop();
            }
            if raw.is_empty() {
                continue;
            }
            let n = raw.len();
            for i in 0..n {
                if raw[(i + 1) % n][3] != raw[i][3] {
                    poly.push(i);
                }
            }
            if poly.len() < 2 {
                let (mut ll, mut ur) = (0, 0);
                for i in 1..n {
                    charge(ws, 1)?;
                    if (raw[i][0], raw[i][2]) < (raw[ll][0], raw[ll][2]) {
                        ll = i;
                    }
                    if (raw[i][0], raw[i][2]) > (raw[ur][0], raw[ur][2]) {
                        ur = i;
                    }
                }
                poly.clear();
                poly.push(ll);
                poly.push(ur);
            }
            let mut i = 0;
            while i < poly.len() {
                let a = poly[i];
                let b = poly[(i + 1) % poly.len()];
                let mut maxd = 0.0;
                let mut maxi = None;
                let (mut ci, inc, end) = if (raw[b][0], raw[b][2]) > (raw[a][0], raw[a][2]) {
                    ((a + 1) % n, 1, b)
                } else {
                    ((b + n - 1) % n, n - 1, a)
                };
                while ci != end {
                    charge(ws, 1)?;
                    let d = distance(raw[ci], raw[a], raw[b]);
                    if d > maxd {
                        maxd = d;
                        maxi = Some(ci);
                    }
                    ci = (ci + inc) % n;
                }
                if let Some(k) = maxi.filter(|_| maxd > max_error * max_error) {
                    if poly.len() == cap {
                        return Err(Error::BufferTooSmall);
                    }
                    charge(ws, poly.len() - i)?;
                    poly.insert(i + 1, k);
                } else {
                    i += 1;
                }
            }
            let start = poly
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| *v)
                .map(|(i, _)| i)
                .unwrap_or(0);
            cont.verts = ws.vec(poly.len() * 4, 0)?;
            for k in 0..poly.len() {
                charge(ws, 4)?;
                let v = raw[poly[(start + k) % poly.len()]];
                let nei = raw[poly[(start + k + 1) % poly.len()]][3];
                let (mut count, mut portal, mut height, mut preg, mut same) =
                    (0, 15u8, 0, 255, true);
                for dz in -1..=0 {
                    for dx in -1..=0 {
                        let px = i32::from(v[0]) + dx;
                        let pz = i32::from(v[2]) + dz;
                        if px < 0
                            || pz < 0
                            || px >= i32::from(l.header.width)
                            || pz >= i32::from(l.header.height)
                        {
                            continue;
                        }
                        let idx = usize::try_from(px).unwrap() + usize::try_from(pz).unwrap() * w;
                        if (i32::from(l.heights[idx]) - i32::from(v[1])).abs() <= climb
                            && l.areas[idx] != 0
                        {
                            height = height.max(l.heights[idx]);
                            portal &= l.cons[idx] >> 4;
                            if preg != 255 && preg != l.regs[idx] {
                                same = false;
                            }
                            preg = l.regs[idx];
                            count += 1;
                        }
                    }
                }
                let mut flags = if nei != 255 && nei >= 0xf8 {
                    nei - 0xf8
                } else {
                    15
                };
                if count > 1 && portal.count_ones() == 1 && same {
                    flags |= 0x80;
                }
                cont.verts[k * 4..k * 4 + 4].copy_from_slice(&[v[0], height, v[2], flags]);
            }
        }
    }
    Ok(out)
}
/// Layer polygon mesh, with six vertices and six neighbour indices per polygon.
///
/// ```compile_fail
/// use navmesh_rs::tilecache::LayerPolyMesh;
/// let LayerPolyMesh { verts, polys, flags, areas } = LayerPolyMesh::new();
/// ```
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct LayerPolyMesh {
    /// Vertices (three quantized coordinates each).
    pub verts: Vec<u16>,
    /// Polygon rows, twelve indices each.
    pub polys: Vec<u16>,
    /// User flags per polygon, initially zero.
    pub flags: Vec<u16>,
    /// Area ids per polygon.
    pub areas: Vec<u8>,
}
impl LayerPolyMesh {
    /// Creates an empty mesh; customize its public fields as needed.
    pub fn new() -> Self {
        Self::default()
    }
}

const FLAG: i32 = 0x1000_0000;
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
    let k = usize::try_from(indices[i] & 0x0fff_ffff).unwrap() * 4;
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

fn diagonal(i: usize, j: usize, n: usize, verts: &[i32], indices: &[i32]) -> bool {
    in_cone(i, j, n, verts, indices) && diagonalie(i, j, n, verts, indices, true)
}

/// Ear-clipping triangulation (`triangulate`); returns the triangle count, negated
/// when the polygon could not be fully triangulated.
fn triangulate(mut n: usize, verts: &[i32], indices: &mut [i32], tris: &mut [i32]) -> i32 {
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
            return -ntris;
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
    p[..nvp].iter().position(|&v| v == NULL).unwrap_or(nvp)
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
fn merge_poly_verts(polys: &mut [u16], a: usize, b: usize, ea: usize, eb: usize, nvp: usize) {
    let pa = &polys[a * nvp..(a + 1) * nvp];
    let pb = &polys[b * nvp..(b + 1) * nvp];
    let na = count_poly_verts(pa, nvp);
    let nb = count_poly_verts(pb, nvp);
    // This builder always uses Detour's six-vertex polygons.
    let mut tmp = [NULL; 6];
    let tmp = &mut tmp[..nvp];
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
        let n = u64::try_from(*npolys).map_err(|_| Error::SizeOverflow)?;
        let width = u64::try_from(nvp).map_err(|_| Error::SizeOverflow)?;
        // Each pair compares up to nvp² edges; include the pass and row copies.
        let pairs = n
            .checked_mul(n.saturating_sub(1))
            .ok_or(Error::SizeOverflow)?
            / 2;
        let cost = pairs
            .checked_mul(width * width)
            .and_then(|v| v.checked_add(width * 4 + 1))
            .ok_or(Error::SizeOverflow)?;
        ws.work(cost)?;
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
        merge_poly_verts(polys, bpa, bpb, bea, beb, nvp);
        let last = *npolys - 1;
        if let Some((regs, areas)) = regs_areas.as_mut() {
            if regs[bpa] != regs[bpb] {
                regs[bpa] = 0;
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

/// Triangulates, greedily merges, removes portal vertices and builds adjacency.
///
/// The pinned ear-clipping and greedy merge algorithms retain their original
/// order for bit-exact output and have O(n³) worst-case work for n contour
/// vertices. [`Limits::default`](crate::Limits::default) is unlimited: callers
/// handling untrusted or large contours should set [`Limits::max_work`](crate::Limits::max_work).
/// A conservative cubic triangulation charge is reserved before mesh allocation,
/// and each greedy merge pass is charged before comparing polygon pairs.
pub fn build_poly_mesh(ws: &mut Workspace, cset: &LayerContourSet) -> Result<LayerPolyMesh, Error> {
    ws.begin();
    poly_mesh(ws, cset)
}
pub(crate) fn poly_mesh(
    ws: &mut Workspace,
    cset: &LayerContourSet,
) -> Result<LayerPolyMesh, Error> {
    let mut maxv = 0usize;
    let mut maxt = 0usize;
    charge(ws, cset.contours.len())?;
    for c in &cset.contours {
        charge(ws, c.verts.len())?;
        if c.verts.len() % 4 != 0
            || c.area > 63
            || c.verts
                .as_chunks::<4>()
                .0
                .iter()
                .any(|v| v[3] & 0x7f > 3 && v[3] & 0x7f != 15)
        {
            return Err(Error::InvalidInput("layer contour"));
        }
        let n = c.verts.len() / 4;
        if n >= 3 {
            maxv = maxv.checked_add(n).ok_or(Error::SizeOverflow)?;
            maxt = maxt.checked_add(n - 2).ok_or(Error::SizeOverflow)?;
            // Reject expensive contours before any mesh/scratch allocation.
            charge(
                ws,
                n.checked_mul(n)
                    .and_then(|v| v.checked_mul(n))
                    .ok_or(Error::SizeOverflow)?,
            )?;
        }
    }
    if maxv >= 0xfffe {
        return Err(Error::FormatLimit("layer mesh vertices"));
    }
    let mut m = LayerPolyMesh {
        verts: ws.vec_with_capacity(maxv * 3)?,
        polys: ws.vec_with_capacity(maxt * 12)?,
        areas: ws.vec_with_capacity(maxt)?,
        flags: Vec::new(),
    };
    let mut vflags = ws.vec(maxv, false)?;
    // Exact (x,z) heads avoid unrelated hash chains; insertion order matches upstream.
    charge(ws, 256 * 256)?;
    let mut heads = ws.vec(256 * 256, usize::MAX)?;
    let mut nextv = ws.vec(maxv, usize::MAX)?;
    charge(ws, cset.contours.len())?;
    for c in &cset.contours {
        let n = c.verts.len() / 4;
        if n < 3 {
            continue;
        }
        let mut v = ws.vec(n * 4, 0i32)?;
        for (a, &b) in v.iter_mut().zip(&c.verts) {
            *a = i32::from(b);
        }
        let mut idx = ws.vec(n, 0i32)?;
        for (i, a) in idx.iter_mut().enumerate() {
            *a = i32::try_from(i).map_err(|_| Error::SizeOverflow)?;
        }
        let mut tris = ws.vec(n * 3, 0i32)?;
        // Diagonal tests were charged conservatively during validation.
        let nt = usize::try_from(triangulate(n, &v, &mut idx, &mut tris).unsigned_abs())
            .map_err(|_| Error::SizeOverflow)?;
        let mut indices = ws.vec(n, 0u16)?;
        for (j, p) in c.verts.as_chunks::<4>().0.iter().enumerate() {
            let key = usize::from(p[0]) + usize::from(p[2]) * 256;
            let mut k = heads[key];
            let mut found = None;
            while k != usize::MAX {
                charge(ws, 1)?;
                if (i32::from(m.verts[k * 3 + 1]) - i32::from(p[1])).abs() <= 2 {
                    found = Some(k);
                    break;
                }
                k = nextv[k];
            }
            let k = if let Some(k) = found {
                k
            } else {
                let k = m.verts.len() / 3;
                m.verts
                    .extend_from_slice(&[u16::from(p[0]), u16::from(p[1]), u16::from(p[2])]);
                nextv[k] = heads[key];
                heads[key] = k;
                k
            };
            indices[j] = short(k)?;
            if p[3] & 0x80 != 0 {
                vflags[k] = true;
            }
        }
        let mut polys = ws.vec((nt + 1) * 6, NULL)?;
        let mut np = 0;
        for t in tris[..nt * 3].as_chunks::<3>().0.iter() {
            let a = usize::try_from(t[0]).map_err(|_| Error::InvalidInput("triangle"))?;
            let b = usize::try_from(t[1]).map_err(|_| Error::InvalidInput("triangle"))?;
            let cc = usize::try_from(t[2]).map_err(|_| Error::InvalidInput("triangle"))?;
            if a != b && a != cc && b != cc {
                polys[np * 6..np * 6 + 3].copy_from_slice(&[indices[a], indices[b], indices[cc]]);
                np += 1;
            }
        }
        merge_polys(ws, &mut polys, &mut np, 6, &m.verts, None)?;
        for p in polys[..np * 6].as_chunks::<6>().0.iter() {
            m.polys.extend_from_slice(p);
            m.polys.extend_from_slice(&[NULL; 6]);
            m.areas.push(c.area);
        }
    }
    let mut i = 0;
    while i < m.verts.len() / 3 {
        if vflags[i] && can_remove(ws, &m, short(i)?)? {
            remove(ws, &mut m, short(i)?, maxt)?;
            let nv = m.verts.len() / 3;
            charge(ws, nv - i)?;
            vflags.copy_within(i + 1..nv + 1, i);
        } else {
            i += 1;
        }
    }
    adjacency(ws, &mut m, cset)?;
    m.flags = ws.vec(m.areas.len(), 0)?;
    Ok(m)
}
fn can_remove(ws: &mut Workspace, m: &LayerPolyMesh, rem: u16) -> Result<bool, Error> {
    charge(ws, m.polys.len())?;
    let mut touched = 0;
    let mut remaining = 0;
    for p in m.polys.as_chunks::<12>().0.iter() {
        let n = count_poly_verts(p, 6);
        let r = p[..n].iter().filter(|&&v| v == rem).count();
        touched += r;
        if r > 0 {
            remaining += n - (r + 1);
        }
    }
    if remaining <= 2 || touched * 2 > 48 {
        return Ok(false);
    }
    // Upstream's scalar array holds 16 triples despite its 48-entry check.
    // Keep room for all 48 triples; never reproduce its out-of-bounds writes.
    let mut edges = [(0u16, 0usize); 48];
    let mut ne = 0;
    for p in m.polys.as_chunks::<12>().0.iter() {
        let n = count_poly_verts(p, 6);
        for j in 0..n {
            let a = p[j];
            let b = p[(j + n - 1) % n];
            if a == rem || b == rem {
                let other = if a == rem { b } else { a };
                charge(ws, ne)?;
                if let Some(e) = edges[..ne].iter_mut().find(|e| e.0 == other) {
                    e.1 += 1;
                } else {
                    if ne == 48 {
                        return Err(Error::BufferTooSmall);
                    }
                    edges[ne] = (other, 1);
                    ne += 1;
                }
            }
        }
    }
    Ok(edges[..ne].iter().filter(|e| e.1 < 2).count() <= 2)
}
fn remove(
    ws: &mut Workspace,
    m: &mut LayerPolyMesh,
    rem: u16,
    maxtris: usize,
) -> Result<(), Error> {
    let mut edges = ws.vec_with_capacity::<[u16; 3]>(48)?;
    charge(ws, m.polys.len())?;
    let mut i = 0;
    while i < m.areas.len() {
        let b = i * 12;
        let n = count_poly_verts(&m.polys[b..], 6);
        if !m.polys[b..b + n].contains(&rem) {
            i += 1;
            continue;
        }
        for j in 0..n {
            let a = m.polys[b + (j + n - 1) % n];
            let c = m.polys[b + j];
            if a != rem && c != rem {
                if edges.len() == 48 {
                    return Err(Error::BufferTooSmall);
                }
                edges.push([a, c, u16::from(m.areas[i])]);
            }
        }
        let last = m.areas.len() - 1;
        m.polys.copy_within(last * 12..last * 12 + 6, b);
        m.polys[b + 6..b + 12].fill(NULL);
        m.polys.truncate(last * 12);
        m.areas.swap_remove(i);
    }
    charge(ws, m.verts.len() + m.polys.len() + edges.len() * 2)?;
    let r = usize::from(rem);
    m.verts.drain(r * 3..r * 3 + 3);
    for p in m.polys.as_chunks_mut::<12>().0.iter_mut() {
        for v in &mut p[..6] {
            if *v != NULL && *v > rem {
                *v -= 1;
            }
        }
    }
    for e in &mut edges {
        for v in &mut e[..2] {
            if *v > rem {
                *v -= 1;
            }
        }
    }
    if edges.is_empty() {
        return Ok(());
    }
    let mut hole = ws.vec_with_capacity::<u16>(48)?;
    let mut areas = ws.vec_with_capacity::<u8>(48)?;
    hole.push(edges[0][0]);
    areas.push(u8::try_from(edges[0][2]).map_err(|_| Error::SizeOverflow)?);
    while !edges.is_empty() {
        let mut matched = false;
        let mut j = 0;
        while j < edges.len() {
            charge(ws, 1)?;
            let [a, b, ar] = edges[j];
            if hole[0] == b {
                if hole.len() == 48 {
                    return Err(Error::BufferTooSmall);
                }
                charge(ws, hole.len())?;
                hole.insert(0, a);
                areas.insert(0, u8::try_from(ar).unwrap());
            } else if hole[hole.len() - 1] == a {
                if hole.len() == 48 {
                    return Err(Error::BufferTooSmall);
                }
                hole.push(b);
                areas.push(u8::try_from(ar).unwrap());
            } else {
                j += 1;
                continue;
            }
            edges.swap_remove(j);
            matched = true;
        }
        if !matched {
            break;
        }
    }
    let n = hole.len();
    charge(ws, n * n * n)?;
    let mut verts = ws.vec(n * 4, 0i32)?;
    let mut idx = ws.vec(n, 0i32)?;
    let mut tris = ws.vec(n * 3, 0i32)?;
    for (j, &k) in hole.iter().enumerate() {
        for c in 0..3 {
            verts[j * 4 + c] = i32::from(m.verts[usize::from(k) * 3 + c]);
        }
        idx[j] = i32::try_from(j).unwrap();
    }
    let nt = usize::try_from(triangulate(n, &verts, &mut idx, &mut tris).unsigned_abs())
        .map_err(|_| Error::SizeOverflow)?;
    let mut polys = ws.vec((nt + 1) * 6, NULL)?;
    let mut pa = ws.vec(nt, 0u8)?;
    let mut pr = ws.vec(nt, 0u16)?;
    let mut np = 0;
    for t in tris[..nt * 3].as_chunks::<3>().0.iter() {
        let a = usize::try_from(t[0]).unwrap();
        let b = usize::try_from(t[1]).unwrap();
        let c = usize::try_from(t[2]).unwrap();
        if a != b && a != c && b != c {
            polys[np * 6..np * 6 + 3].copy_from_slice(&[hole[a], hole[b], hole[c]]);
            pa[np] = areas[a];
            np += 1;
        }
    }
    merge_polys(
        ws,
        &mut polys,
        &mut np,
        6,
        &m.verts,
        Some((&mut pr, &mut pa)),
    )?;
    for (j, p) in polys[..np * 6].as_chunks::<6>().0.iter().enumerate() {
        if m.areas.len() >= maxtris {
            return Err(Error::BufferTooSmall);
        }
        m.polys.extend_from_slice(p);
        m.polys.extend_from_slice(&[NULL; 6]);
        m.areas.push(pa[j]);
    }
    Ok(())
}
fn adjacency(
    ws: &mut Workspace,
    m: &mut LayerPolyMesh,
    cset: &LayerContourSet,
) -> Result<(), Error> {
    // Sorted edge index: no all-polygons scan for an edge.
    let mut edges = ws.vec_with_capacity::<(u16, u16, usize, usize)>(m.areas.len() * 6)?;
    for (i, p) in m.polys.as_chunks::<12>().0.iter().enumerate() {
        let n = count_poly_verts(p, 6);
        for j in 0..n {
            edges.push((p[j].min(p[(j + 1) % n]), p[j].max(p[(j + 1) % n]), i, j));
        }
    }
    let n = edges.len();
    charge(ws, n * (usize::try_from(n.max(1).ilog2()).unwrap() + 1))?;
    edges.sort_unstable();
    // Portal lookup by unit edge, retaining the last contour segment just as C++.
    charge(ws, 2 * 256 * 256)?;
    let mut portals = ws.vec(2 * 256 * 256, None::<(usize, u8)>)?;
    let mut order = 0;
    charge(ws, cset.contours.len())?;
    for c in &cset.contours {
        let n = c.verts.len() / 4;
        if n < 3 {
            continue;
        }
        charge(ws, n)?;
        for j in 0..n {
            let a = &c.verts[j * 4..];
            let b = &c.verts[((j + 1) % n) * 4..];
            let d = a[3] & 15;
            order += 1;
            if d == 15 {
                continue;
            }
            let (axis, fixed, lo, hi) = if d == 0 || d == 2 {
                (0, usize::from(a[0]), a[2].min(b[2]), a[2].max(b[2]))
            } else {
                (1, usize::from(a[2]), a[0].min(b[0]), a[0].max(b[0]))
            };
            charge(ws, usize::from(hi - lo))?;
            for t in lo..hi {
                portals[axis * 65536 + fixed * 256 + usize::from(t)] = Some((order, d));
            }
        }
    }
    let mut i = 0;
    while i < edges.len() {
        charge(ws, 1)?;
        let e = edges[i];
        let mut end = i + 1;
        while end < edges.len() && edges[end].0 == e.0 && edges[end].1 == e.1 {
            end += 1;
        }
        if end - i == 2 {
            let f = edges[i + 1];
            m.polys[e.2 * 12 + 6 + e.3] = short(f.2)?;
            m.polys[f.2 * 12 + 6 + f.3] = short(e.2)?;
        } else if end - i > 2 {
            return Err(Error::InvalidInput("nonmanifold layer edge"));
        } else {
            let a = &m.verts[usize::from(e.0) * 3..];
            let b = &m.verts[usize::from(e.1) * 3..];
            let entry = if a[0] == b[0] {
                Some((0, usize::from(a[0]), a[2].min(b[2]), a[2].max(b[2])))
            } else if a[2] == b[2] {
                Some((1, usize::from(a[2]), a[0].min(b[0]), a[0].max(b[0])))
            } else {
                None
            };
            if let Some((axis, fixed, lo, hi)) = entry {
                let mut best = None;
                charge(ws, usize::from(hi - lo))?;
                for t in lo..hi {
                    let p = portals[axis * 65536 + fixed * 256 + usize::from(t)];
                    if p > best {
                        best = p;
                    }
                }
                if let Some((_, d)) = best {
                    m.polys[e.2 * 12 + 6 + e.3] = 0x8000 | u16::from(d);
                }
            }
        }
        i = end;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn every_greedy_merge_pass_is_charged_before_it_runs() {
        let verts = [0, 0, 0, 0, 0, 8, 8, 0, 8, 8, 0, 0];
        let original = [0, 1, 2, NULL, NULL, NULL, 0, 2, 3, NULL, NULL, NULL];
        for (limit, expected_count, expected_work) in [(60, 2, 61), (85, 1, 86), (86, 1, 86)] {
            let mut ws = Workspace::with_limits(Limits {
                max_work: limit,
                ..Limits::default()
            });
            let mut polys = original;
            let mut count = 2;
            let result = merge_polys(&mut ws, &mut polys, &mut count, 6, &verts, None);
            assert_eq!(
                result,
                if limit < 86 {
                    Err(Error::LimitExceeded)
                } else {
                    Ok(())
                }
            );
            assert_eq!(count, expected_count);
            assert_eq!(ws.usage().work, expected_work);
            if limit == 60 {
                assert_eq!(polys, original);
            }
        }
    }
}
