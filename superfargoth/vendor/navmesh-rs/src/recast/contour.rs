//! Contours (upstream `RecastContour.cpp`).

use alloc::vec::Vec;

use super::detail::check_surface_chf;
use super::{
    AREA_BORDER, BORDER_REG, BORDER_VERTEX, CONTOUR_REG_MASK, CompactHeightfield, NOT_CONNECTED,
    dir_offset_x, dir_offset_y,
};
use crate::{Error, Workspace};

/// Tessellate solid (impassable) edges during simplification (`RC_CONTOUR_TESS_WALL_EDGES`).
pub const CONTOUR_TESS_WALL_EDGES: i32 = 0x01;
/// Tessellate edges between areas during simplification (`RC_CONTOUR_TESS_AREA_EDGES`).
pub const CONTOUR_TESS_AREA_EDGES: i32 = 0x02;

/// A simplified region outline (`rcContour`).
#[derive(Clone, Debug, Default)]
pub struct Contour {
    /// Simplified vertices: x, y, z, flags/region per vertex.
    pub verts: Vec<i32>,
    /// Raw vertices: x, y, z, flags/region per vertex.
    pub rverts: Vec<i32>,
    /// Region id.
    pub reg: u16,
    /// Area id.
    pub area: u8,
}

/// The contours of a compact heightfield (`rcContourSet`).
#[derive(Clone, Debug, Default)]
pub struct ContourSet {
    /// Contours.
    pub conts: Vec<Contour>,
    /// Minimum bounds.
    pub bmin: [f32; 3],
    /// Maximum bounds.
    pub bmax: [f32; 3],
    /// Cell size.
    pub cs: f32,
    /// Cell height.
    pub ch: f32,
    /// Width in cells.
    pub width: i32,
    /// Height in cells.
    pub height: i32,
    /// Border size in cells.
    pub border_size: i32,
    /// Simplification error used.
    pub max_error: f32,
}

impl ContourSet {
    /// Traces and simplifies the region outlines of `chf` (`rcBuildContours`).
    /// `build_flags` is a combination of [`CONTOUR_TESS_WALL_EDGES`] and
    /// [`CONTOUR_TESS_AREA_EDGES`].
    pub fn build(
        ws: &mut Workspace,
        chf: &CompactHeightfield,
        max_error: f32,
        max_edge_len: i32,
        build_flags: i32,
    ) -> Result<ContourSet, Error> {
        ws.begin();
        check_surface_chf(ws, chf)?;
        let w = chf.width;
        let h = chf.height;
        let border = chf.border_size;
        let mut cset = ContourSet {
            conts: Vec::new(),
            bmin: chf.bmin,
            bmax: chf.bmax,
            cs: chf.cs,
            ch: chf.ch,
            width: chf.width - border * 2,
            height: chf.height - border * 2,
            border_size: border,
            max_error,
        };
        if border > 0 {
            let pad = border as f32 * chf.cs;
            cset.bmin[0] += pad;
            cset.bmin[2] += pad;
            cset.bmax[0] -= pad;
            cset.bmax[2] -= pad;
        }
        let n = chf.spans.len();
        ws.work(n as u64 * 2)?;
        let mut flags = ws.vec(n, 0u8)?;
        for y in 0..h {
            ws.work(1)?;
            for x in 0..w {
                let c = chf.cells[(x + y * w) as usize];
                for i in c.index as usize..c.index as usize + c.count as usize {
                    let s = chf.spans[i];
                    if s.reg == 0 || (s.reg & BORDER_REG) != 0 {
                        flags[i] = 0;
                        continue;
                    }
                    let mut res = 0u8;
                    for dir in 0..4 {
                        let r = nei_index(chf, x, y, i, dir).map_or(0, |ai| chf.spans[ai].reg);
                        if r == s.reg {
                            res |= 1 << dir;
                        }
                    }
                    flags[i] = res ^ 0xf;
                }
            }
        }
        let mut verts: Vec<i32> = Vec::new();
        let mut simplified: Vec<i32> = Vec::new();
        for y in 0..h {
            ws.work(1)?;
            for x in 0..w {
                let c = chf.cells[(x + y * w) as usize];
                for i in c.index as usize..c.index as usize + c.count as usize {
                    if flags[i] == 0 || flags[i] == 0xf {
                        flags[i] = 0;
                        continue;
                    }
                    let reg = chf.spans[i].reg;
                    if reg == 0 || (reg & BORDER_REG) != 0 {
                        continue;
                    }
                    let area = chf.areas[i];
                    verts.clear();
                    simplified.clear();
                    walk_contour(ws, x, y, i, chf, &mut flags, &mut verts)?;
                    simplify_contour(
                        ws,
                        &verts,
                        &mut simplified,
                        max_error,
                        max_edge_len,
                        build_flags,
                    )?;
                    remove_degenerate_segments(&mut simplified);
                    if simplified.len() / 4 >= 3 {
                        let mut cv = ws.vec_with_capacity(simplified.len())?;
                        cv.extend_from_slice(&simplified);
                        let mut rv = ws.vec_with_capacity(verts.len())?;
                        rv.extend_from_slice(&verts);
                        if border > 0 {
                            for v in cv.chunks_exact_mut(4).chain(rv.chunks_exact_mut(4)) {
                                v[0] -= border;
                                v[2] -= border;
                            }
                        }
                        ws.push(
                            &mut cset.conts,
                            Contour {
                                verts: cv,
                                rverts: rv,
                                reg,
                                area,
                            },
                        )?;
                    }
                }
            }
        }

        // Merge holes into their outlines.
        if !cset.conts.is_empty() {
            let winding = contour_winding(ws, &cset.conts)?;
            let nholes = winding.iter().filter(|&&w| w < 0).count();
            if nholes > 0 {
                let nregions = usize::from(chf.max_regions) + 1;
                let mut outline: Vec<Option<usize>> = ws.vec(nregions, None)?;
                let mut holes: Vec<Vec<usize>> = ws.vec(nregions, Vec::new())?;
                for (i, c) in cset.conts.iter().enumerate() {
                    let r = usize::from(c.reg);
                    if r >= nregions {
                        return Err(Error::InvalidInput("region id above max_regions"));
                    }
                    if winding[i] > 0 {
                        outline[r] = Some(i);
                    } else {
                        ws.push(&mut holes[r], i)?;
                    }
                }
                for r in 0..nregions {
                    if holes[r].is_empty() {
                        continue;
                    }
                    if let Some(o) = outline[r] {
                        merge_region_holes(ws, &mut cset.conts, o, &holes[r])?;
                    }
                }
            }
        }
        Ok(cset)
    }
}

#[inline(always)]
fn nei_index(chf: &CompactHeightfield, x: i32, y: i32, i: usize, dir: i32) -> Option<usize> {
    let c = chf.spans[i].con(dir);
    if c == NOT_CONNECTED {
        return None;
    }
    let ax = x + dir_offset_x(dir);
    let ay = y + dir_offset_y(dir);
    Some(chf.cells[(ax + ay * chf.width) as usize].index as usize + c as usize)
}

fn get_corner_height(chf: &CompactHeightfield, x: i32, y: i32, i: usize, dir: i32) -> (i32, bool) {
    let s = chf.spans[i];
    let mut ch = i32::from(s.y);
    let dirp = (dir + 1) & 3;
    let key = |k: usize| u32::from(chf.spans[k].reg) | (u32::from(chf.areas[k]) << 16);
    let mut regs = [key(i), 0, 0, 0];
    if let Some(ai) = nei_index(chf, x, y, i, dir) {
        let ax = x + dir_offset_x(dir);
        let ay = y + dir_offset_y(dir);
        ch = ch.max(i32::from(chf.spans[ai].y));
        regs[1] = key(ai);
        if let Some(ai2) = nei_index(chf, ax, ay, ai, dirp) {
            ch = ch.max(i32::from(chf.spans[ai2].y));
            regs[2] = key(ai2);
        }
    }
    if let Some(ai) = nei_index(chf, x, y, i, dirp) {
        let ax = x + dir_offset_x(dirp);
        let ay = y + dir_offset_y(dirp);
        ch = ch.max(i32::from(chf.spans[ai].y));
        regs[3] = key(ai);
        if let Some(ai2) = nei_index(chf, ax, ay, ai, dir) {
            ch = ch.max(i32::from(chf.spans[ai2].y));
            regs[2] = key(ai2);
        }
    }
    let border = u32::from(BORDER_REG);
    let mut is_border = false;
    for j in 0..4 {
        let a = regs[j];
        let b = regs[(j + 1) & 3];
        let c = regs[(j + 2) & 3];
        let d = regs[(j + 3) & 3];
        let two_same_exts = (a & b & border) != 0 && a == b;
        let two_ints = ((c | d) & border) == 0;
        let ints_same_area = (c >> 16) == (d >> 16);
        let no_zeros = a != 0 && b != 0 && c != 0 && d != 0;
        if two_same_exts && two_ints && ints_same_area && no_zeros {
            is_border = true;
            break;
        }
    }
    (ch, is_border)
}

#[allow(clippy::too_many_arguments)]
fn walk_contour(
    ws: &mut Workspace,
    mut x: i32,
    mut y: i32,
    mut i: usize,
    chf: &CompactHeightfield,
    flags: &mut [u8],
    points: &mut Vec<i32>,
) -> Result<(), Error> {
    let mut dir: i32 = 0;
    while dir < 4 && (flags[i] & (1 << dir)) == 0 {
        dir += 1;
    }
    let start_dir = dir;
    let start_i = i;
    let area = chf.areas[i];
    let mut iter = 0;
    loop {
        iter += 1;
        if iter >= 40000 {
            break;
        }
        if flags[i] & (1 << dir) != 0 {
            let (py, is_border) = get_corner_height(chf, x, y, i, dir);
            let mut px = x;
            let mut pz = y;
            match dir {
                0 => pz += 1,
                1 => {
                    px += 1;
                    pz += 1
                }
                2 => px += 1,
                _ => {}
            }
            let mut r = 0i32;
            let mut is_area_border = false;
            if let Some(ai) = nei_index(chf, x, y, i, dir) {
                r = i32::from(chf.spans[ai].reg);
                if area != chf.areas[ai] {
                    is_area_border = true;
                }
            }
            if is_border {
                r |= BORDER_VERTEX;
            }
            if is_area_border {
                r |= AREA_BORDER;
            }
            ws.reserve(points, 4)?;
            points.extend_from_slice(&[px, py, pz, r]);
            flags[i] &= !(1 << dir);
            dir = (dir + 1) & 3;
        } else {
            let Some(ni) = nei_index(chf, x, y, i, dir) else {
                return Ok(());
            };
            x += dir_offset_x(dir);
            y += dir_offset_y(dir);
            i = ni;
            dir = (dir + 3) & 3;
        }
        if start_i == i && start_dir == dir {
            break;
        }
    }
    ws.work(iter as u64)
}

fn distance_pt_seg(x: i32, z: i32, px: i32, pz: i32, qx: i32, qz: i32) -> f32 {
    let pqx = (qx - px) as f32;
    let pqz = (qz - pz) as f32;
    let mut dx = (x - px) as f32;
    let mut dz = (z - pz) as f32;
    let d = pqx * pqx + pqz * pqz;
    let mut t = pqx * dx + pqz * dz;
    if d > 0.0 {
        t /= d;
    }
    if t < 0.0 {
        t = 0.0;
    } else if t > 1.0 {
        t = 1.0;
    }
    dx = px as f32 + t * pqx - x as f32;
    dz = pz as f32 + t * pqz - z as f32;
    dx * dx + dz * dz
}

fn insert_point(
    ws: &mut Workspace,
    simplified: &mut Vec<i32>,
    at: usize,
    points: &[i32],
    maxi: usize,
) -> Result<(), Error> {
    let p = [
        points[maxi * 4],
        points[maxi * 4 + 1],
        points[maxi * 4 + 2],
        maxi as i32,
    ];
    let pos = (at + 1) * 4;
    ws.reserve(simplified, 4)?;
    simplified.splice(pos..pos, p);
    Ok(())
}

fn simplify_contour(
    ws: &mut Workspace,
    points: &[i32],
    simplified: &mut Vec<i32>,
    max_error: f32,
    max_edge_len: i32,
    build_flags: i32,
) -> Result<(), Error> {
    let pn = points.len() / 4;
    if pn == 0 {
        return Ok(());
    }
    let has_connections = points
        .chunks_exact(4)
        .any(|p| (p[3] & CONTOUR_REG_MASK) != 0);
    if has_connections {
        for i in 0..pn {
            let ii = (i + 1) % pn;
            let different_regs =
                (points[i * 4 + 3] & CONTOUR_REG_MASK) != (points[ii * 4 + 3] & CONTOUR_REG_MASK);
            let area_borders =
                (points[i * 4 + 3] & AREA_BORDER) != (points[ii * 4 + 3] & AREA_BORDER);
            if different_regs || area_borders {
                ws.reserve(simplified, 4)?;
                simplified.extend_from_slice(&[
                    points[i * 4],
                    points[i * 4 + 1],
                    points[i * 4 + 2],
                    i as i32,
                ]);
            }
        }
    }
    if simplified.is_empty() {
        let (mut llx, mut lly, mut llz, mut lli) = (points[0], points[1], points[2], 0);
        let (mut urx, mut ury, mut urz, mut uri) = (points[0], points[1], points[2], 0);
        for (k, p) in points.chunks_exact(4).enumerate() {
            let (x, y, z) = (p[0], p[1], p[2]);
            if x < llx || (x == llx && z < llz) {
                (llx, lly, llz, lli) = (x, y, z, k as i32);
            }
            if x > urx || (x == urx && z > urz) {
                (urx, ury, urz, uri) = (x, y, z, k as i32);
            }
        }
        ws.reserve(simplified, 8)?;
        simplified.extend_from_slice(&[llx, lly, llz, lli, urx, ury, urz, uri]);
    }
    let mut i = 0;
    while i < simplified.len() / 4 {
        let ii = (i + 1) % (simplified.len() / 4);
        let mut ax = simplified[i * 4];
        let mut az = simplified[i * 4 + 2];
        let ai = simplified[i * 4 + 3] as usize;
        let mut bx = simplified[ii * 4];
        let mut bz = simplified[ii * 4 + 2];
        let bi = simplified[ii * 4 + 3] as usize;
        let mut maxd = 0.0f32;
        let mut maxi: Option<usize> = None;
        let (mut ci, cinc, endi);
        if bx > ax || (bx == ax && bz > az) {
            cinc = 1;
            ci = (ai + cinc) % pn;
            endi = bi;
        } else {
            cinc = pn - 1;
            ci = (bi + cinc) % pn;
            endi = ai;
            core::mem::swap(&mut ax, &mut bx);
            core::mem::swap(&mut az, &mut bz);
        }
        if (points[ci * 4 + 3] & CONTOUR_REG_MASK) == 0 || (points[ci * 4 + 3] & AREA_BORDER) != 0 {
            while ci != endi {
                let d = distance_pt_seg(points[ci * 4], points[ci * 4 + 2], ax, az, bx, bz);
                if d > maxd {
                    maxd = d;
                    maxi = Some(ci);
                }
                ci = (ci + cinc) % pn;
            }
        }
        match maxi {
            Some(m) if maxd > max_error * max_error => insert_point(ws, simplified, i, points, m)?,
            _ => i += 1,
        }
    }
    if max_edge_len > 0 && (build_flags & (CONTOUR_TESS_WALL_EDGES | CONTOUR_TESS_AREA_EDGES)) != 0
    {
        let mut i = 0;
        while i < simplified.len() / 4 {
            let ii = (i + 1) % (simplified.len() / 4);
            let ax = simplified[i * 4];
            let az = simplified[i * 4 + 2];
            let ai = simplified[i * 4 + 3] as usize;
            let bx = simplified[ii * 4];
            let bz = simplified[ii * 4 + 2];
            let bi = simplified[ii * 4 + 3] as usize;
            let mut maxi: Option<usize> = None;
            let ci = (ai + 1) % pn;
            let mut tess = false;
            if (build_flags & CONTOUR_TESS_WALL_EDGES) != 0
                && (points[ci * 4 + 3] & CONTOUR_REG_MASK) == 0
            {
                tess = true;
            }
            if (build_flags & CONTOUR_TESS_AREA_EDGES) != 0
                && (points[ci * 4 + 3] & AREA_BORDER) != 0
            {
                tess = true;
            }
            if tess {
                let dx = bx - ax;
                let dz = bz - az;
                if dx.wrapping_mul(dx).wrapping_add(dz.wrapping_mul(dz))
                    > max_edge_len.wrapping_mul(max_edge_len)
                {
                    let n = if bi < ai { bi + pn - ai } else { bi - ai };
                    if n > 1 {
                        maxi = Some(if bx > ax || (bx == ax && bz > az) {
                            (ai + n / 2) % pn
                        } else {
                            (ai + n.div_ceil(2)) % pn
                        });
                    }
                }
            }
            match maxi {
                Some(m) => insert_point(ws, simplified, i, points, m)?,
                None => i += 1,
            }
        }
    }
    for i in 0..simplified.len() / 4 {
        let ai = (simplified[i * 4 + 3] as usize + 1) % pn;
        let bi = simplified[i * 4 + 3] as usize;
        simplified[i * 4 + 3] = (points[ai * 4 + 3] & (CONTOUR_REG_MASK | AREA_BORDER))
            | (points[bi * 4 + 3] & BORDER_VERTEX);
    }
    Ok(())
}

fn calc_area_of_polygon_2d(verts: &[i32]) -> i32 {
    let n = verts.len() / 4;
    let mut area = 0i32;
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let vi = &verts[i * 4..];
        let vj = &verts[j * 4..];
        area = area.wrapping_add(
            vi[0]
                .wrapping_mul(vj[2])
                .wrapping_sub(vj[0].wrapping_mul(vi[2])),
        );
        j = i;
    }
    (area.wrapping_add(1)) / 2
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

fn intersect_seg_contour(d0: &[i32], d1: &[i32], i: i64, verts: &[i32]) -> bool {
    let n = verts.len() / 4;
    for k in 0..n {
        let k1 = next(k, n);
        if i == k as i64 || i == k1 as i64 {
            continue;
        }
        let p0 = &verts[k * 4..k * 4 + 4];
        let p1 = &verts[k1 * 4..k1 * 4 + 4];
        if vequal(d0, p0) || vequal(d1, p0) || vequal(d0, p1) || vequal(d1, p1) {
            continue;
        }
        if intersect(d0, d1, p0, p1) {
            return true;
        }
    }
    false
}

fn in_cone(i: usize, verts: &[i32], pj: &[i32]) -> bool {
    let n = verts.len() / 4;
    let pi = &verts[i * 4..];
    let pi1 = &verts[next(i, n) * 4..];
    let pin1 = &verts[prev(i, n) * 4..];
    if left_on(pin1, pi, pi1) {
        return left(pi, pj, pin1) && left(pj, pi, pi1);
    }
    !(left_on(pi, pj, pi1) && left_on(pj, pi, pin1))
}

fn remove_degenerate_segments(simplified: &mut Vec<i32>) {
    let mut npts = simplified.len() / 4;
    let mut i = 0;
    while i < npts {
        let ni = next(i, npts);
        if vequal(&simplified[i * 4..], &simplified[ni * 4..]) {
            simplified.drain(i * 4..i * 4 + 4);
            npts -= 1;
        }
        i += 1;
    }
}

/// Merges hole contour `b` into outline `a` at vertices `ia` and `ib`.
fn merge_contours(
    ws: &mut Workspace,
    conts: &mut [Contour],
    a: usize,
    b: usize,
    ia: usize,
    ib: usize,
) -> Result<(), Error> {
    let na = conts[a].verts.len() / 4;
    let nb = conts[b].verts.len() / 4;
    let count = na
        .checked_add(nb)
        .and_then(|n| n.checked_add(2))
        .and_then(|n| n.checked_mul(4))
        .ok_or(Error::SizeOverflow)?;
    let mut verts = ws.vec_with_capacity(count)?;
    for i in 0..=na {
        let s = ((ia + i) % na) * 4;
        verts.extend_from_slice(&conts[a].verts[s..s + 4]);
    }
    for i in 0..=nb {
        let s = ((ib + i) % nb) * 4;
        verts.extend_from_slice(&conts[b].verts[s..s + 4]);
    }
    conts[a].verts = verts;
    conts[b].verts = Vec::new();
    Ok(())
}

#[derive(Clone, Copy, Default)]
struct Diag {
    vert: usize,
    dist: i32,
}

fn merge_region_holes(
    ws: &mut Workspace,
    conts: &mut [Contour],
    outline: usize,
    hole_ids: &[usize],
) -> Result<(), Error> {
    // (contour, minx, minz, leftmost), sorted stably like glibc's merge-sort qsort.
    let mut holes = ws.vec_with_capacity(hole_ids.len())?;
    for &h in hole_ids {
        let v = &conts[h].verts;
        let (mut minx, mut minz, mut leftmost) = (v[0], v[2], 0);
        for k in 1..v.len() / 4 {
            let (x, z) = (v[k * 4], v[k * 4 + 2]);
            if x < minx || (x == minx && z < minz) {
                (minx, minz, leftmost) = (x, z, k);
            }
        }
        holes.push((h, minx, minz, leftmost));
    }
    let mut hole_scratch = ws.vec(holes.len(), (0, 0, 0, 0))?;
    crate::common::stable_sort(&mut holes, &mut hole_scratch, |a, b| {
        a.1.cmp(&b.1).then(a.2.cmp(&b.2))
    });
    let max_verts = conts[outline].verts.len() / 4
        + holes
            .iter()
            .map(|h| conts[h.0].verts.len() / 4)
            .sum::<usize>();
    // Upstream reads diags[i] (the hole index) where diags[j] is meant; the buffer
    // persists across holes, so stale entries are part of the behaviour. Zeroed
    // here where upstream's buffer is uninitialised.
    let mut diags: Vec<Diag> = ws.vec(max_verts.max(holes.len()), Diag::default())?;
    let mut diag_scratch = ws.vec(diags.len(), Diag::default())?;
    for i in 0..holes.len() {
        let hole = holes[i].0;
        let hn = conts[hole].verts.len() / 4;
        let mut index: Option<usize> = None;
        let mut best = holes[i].3;
        ws.work(hn as u64 * (conts[outline].verts.len() / 4) as u64)?;
        for _ in 0..hn {
            let mut nd = 0;
            let corner = [
                conts[hole].verts[best * 4],
                conts[hole].verts[best * 4 + 1],
                conts[hole].verts[best * 4 + 2],
                conts[hole].verts[best * 4 + 3],
            ];
            let ov = &conts[outline].verts;
            for j in 0..ov.len() / 4 {
                if in_cone(j, ov, &corner) {
                    let dx = ov[j * 4] - corner[0];
                    let dz = ov[j * 4 + 2] - corner[2];
                    if nd >= diags.len() {
                        ws.push(&mut diags, Diag::default())?;
                    }
                    diags[nd] = Diag {
                        vert: j,
                        dist: dx.wrapping_mul(dx).wrapping_add(dz.wrapping_mul(dz)),
                    };
                    nd += 1;
                }
            }
            if diag_scratch.len() < nd {
                let additional = nd - diag_scratch.len();
                ws.reserve(&mut diag_scratch, additional)?;
                diag_scratch.resize(nd, Diag::default());
            }
            crate::common::stable_sort(&mut diags[..nd], &mut diag_scratch[..nd], |a, b| {
                a.dist.cmp(&b.dist)
            });
            index = None;
            for j in 0..nd {
                let pt = &ov[diags[j].vert * 4..diags[j].vert * 4 + 4];
                let skip = diags.get(i).map_or(-1, |d| d.vert as i64);
                let mut hit = intersect_seg_contour(pt, &corner, skip, ov);
                for h in &holes[i..] {
                    if hit {
                        break;
                    }
                    hit |= intersect_seg_contour(pt, &corner, -1, &conts[h.0].verts);
                }
                if !hit {
                    index = Some(diags[j].vert);
                    break;
                }
            }
            if index.is_some() {
                break;
            }
            best = (best + 1) % hn;
        }
        if let Some(index) = index {
            merge_contours(ws, conts, outline, hole, index, best)?;
        }
    }
    Ok(())
}

fn contour_winding(ws: &mut Workspace, conts: &[Contour]) -> Result<Vec<i8>, Error> {
    let mut winding = ws.vec_with_capacity(conts.len())?;
    for c in conts {
        winding.push(if calc_area_of_polygon_2d(&c.verts) < 0 {
            -1
        } else {
            1
        });
    }
    Ok(winding)
}

#[cfg(test)]
mod allocation_tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn winding_and_hole_merge_charge_before_allocation() {
        let contour = Contour {
            verts: alloc::vec![0, 0, 0, 0, 0, 0, 4, 0, 4, 0, 4, 0, 4, 0, 0, 0],
            ..Contour::default()
        };
        let mut conts = [contour.clone(), contour];
        let mut ws = Workspace::with_limits(Limits {
            max_bytes: 0,
            ..Limits::default()
        });
        assert_eq!(contour_winding(&mut ws, &conts), Err(Error::LimitExceeded));
        assert_eq!(ws.usage().bytes, 2);
        ws.begin();
        let original = conts[0].verts.clone();
        assert_eq!(
            merge_contours(&mut ws, &mut conts, 0, 1, 0, 0),
            Err(Error::LimitExceeded)
        );
        assert_eq!(ws.usage().bytes, 10 * 4 * 4);
        assert_eq!(conts[0].verts, original);
        ws.set_limits(Limits::default());
        ws.begin();
        merge_contours(&mut ws, &mut conts, 0, 1, 0, 0).unwrap();
        assert_eq!(conts[0].verts.len(), 40);
        assert!(conts[1].verts.is_empty());
    }

    #[test]
    fn hole_sort_storage_is_charged() {
        let c = Contour {
            verts: alloc::vec![0, 0, 0, 0, 0, 0, 4, 0, 4, 0, 4, 0],
            ..Contour::default()
        };
        let mut conts = [c.clone(), c];
        let mut ws = Workspace::with_limits(Limits {
            max_bytes: 0,
            ..Limits::default()
        });
        assert_eq!(
            merge_region_holes(&mut ws, &mut conts, 0, &[1]),
            Err(Error::LimitExceeded)
        );
        assert!(ws.usage().bytes > 0);
    }
}
