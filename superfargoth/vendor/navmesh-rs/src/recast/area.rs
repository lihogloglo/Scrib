//! Area erosion, median filter and area marking (upstream `RecastArea.cpp`).

use super::{CompactHeightfield, NOT_CONNECTED, NULL_AREA, dir_offset_x, dir_offset_y};
use crate::common::{sqr, sqrtf, v3, vmax, vmin};
use crate::{Error, Workspace};

pub(crate) fn check_chf(ws: &mut Workspace, chf: &CompactHeightfield) -> Result<(), Error> {
    let cells = (chf.width.max(0) as usize).checked_mul(chf.height.max(0) as usize);
    if chf.width < 0
        || chf.height < 0
        || (chf.width == 0) != (chf.height == 0)
        || cells != Some(chf.cells.len())
        || chf.spans.len() != chf.span_count.max(0) as usize
        || chf.areas.len() != chf.spans.len()
    {
        return Err(Error::InvalidInput("inconsistent compact heightfield"));
    }
    // Validation itself visits every cell/span; charge before traversing.
    ws.work(
        (chf.cells.len() as u64)
            .checked_add(
                (chf.spans.len() as u64)
                    .checked_mul(4)
                    .ok_or(Error::SizeOverflow)?,
            )
            .ok_or(Error::SizeOverflow)?,
    )?;
    if ws.compact_trusted() {
        // Built by this crate's pipeline and unreachable by callers since: skip
        // the scans but charge exactly what the generic path charges.
        for _ in 0..chf.height {
            ws.work(1)?;
        }
        return Ok(());
    }
    if chf
        .cells
        .iter()
        .any(|c| c.index as usize + c.count as usize > chf.spans.len())
    {
        return Err(Error::InvalidInput("inconsistent compact heightfield"));
    }
    // A dense single-layer grid has no per-link layer lookup: every link is
    // either layer zero or disconnected. Verify the layout before taking this
    // path; public fields may have been edited by callers. Keep finite-budget
    // traversal on the generic path to retain its exact charging order.
    if chf.width > 0
        && ws.limits().max_work == u64::MAX
        && chf.spans.len() == chf.cells.len()
        && chf.cells.iter().enumerate().fold(true, |valid, (i, c)| {
            valid & (c.count == 1) & (c.index as usize == i)
        })
    {
        ws.work(chf.height as u64)?;
        // Within each six-bit field, all adjacent bits must agree. Thus the
        // only permitted layer values are zero and NOT_CONNECTED (63).
        let invalid = chf
            .spans
            .iter()
            .fold(0, |bits, s| bits | ((s.con ^ (s.con >> 1)) & 0x007d_f7df));
        let w = chf.width as usize;
        let h = chf.height as usize;
        let mut boundary_valid = true;
        for row in chf.spans.chunks_exact(w) {
            boundary_valid &= row[0].con(0) == NOT_CONNECTED;
            boundary_valid &= row[w - 1].con(2) == NOT_CONNECTED;
        }
        for s in &chf.spans[..w] {
            boundary_valid &= s.con(3) == NOT_CONNECTED;
        }
        for s in &chf.spans[(h - 1) * w..] {
            boundary_valid &= s.con(1) == NOT_CONNECTED;
        }
        return if invalid == 0 && boundary_valid {
            Ok(())
        } else {
            Err(Error::InvalidInput("compact span connection out of range"))
        };
    }
    // Sparse single-layer fields (terrain with holes): one row pass over the
    // packed connection masks instead of four neighbour lookups per span.
    if chf.width > 0
        && ws.limits().max_work == u64::MAX
        && super::detail::single_layer_connections_valid(chf)
    {
        return ws.work(chf.height as u64);
    }
    for z in 0..chf.height {
        ws.work(1)?;
        for x in 0..chf.width {
            let c = chf.cells[(x + z * chf.width) as usize];
            for s in &chf.spans[c.index as usize..c.index as usize + c.count as usize] {
                for dir in 0..4 {
                    let con = s.con(dir);
                    if con == NOT_CONNECTED {
                        continue;
                    }
                    let nx = x + dir_offset_x(dir);
                    let nz = z + dir_offset_y(dir);
                    if nx < 0
                        || nz < 0
                        || nx >= chf.width
                        || nz >= chf.height
                        || con >= i32::from(chf.cells[(nx + nz * chf.width) as usize].count)
                    {
                        return Err(Error::InvalidInput("compact span connection out of range"));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Index of the neighbour of span `i` (in column `x`, `z`) in direction `dir`, or
/// `None` when not connected.
#[inline(always)]
fn nei(chf: &CompactHeightfield, x: i32, z: i32, i: usize, dir: i32) -> Option<(i32, i32, usize)> {
    let c = chf.spans[i].con(dir);
    if c == NOT_CONNECTED {
        return None;
    }
    let ax = x + dir_offset_x(dir);
    let az = z + dir_offset_y(dir);
    Some((
        ax,
        az,
        chf.cells[(ax + az * chf.width) as usize].index as usize + c as usize,
    ))
}

/// Removes walkable area within `radius` cells of a boundary (`rcErodeWalkableArea`).
pub fn erode_walkable_area(
    ws: &mut Workspace,
    radius: i32,
    chf: &mut CompactHeightfield,
) -> Result<(), Error> {
    ws.begin();
    check_chf(ws, chf)?;
    let w = chf.width;
    let h = chf.height;
    let n = chf.spans.len();
    ws.work(n as u64 * 3 + chf.cells.len() as u64)?;
    let mut dist = ws.vec(n, 0xffu8)?;
    for z in 0..h {
        ws.work(1)?;
        for x in 0..w {
            let c = chf.cells[(x + z * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA {
                    dist[i] = 0;
                    continue;
                }
                let mut count = 0;
                for dir in 0..4 {
                    match nei(chf, x, z, i, dir) {
                        Some((_, _, ai)) if chf.areas[ai] != NULL_AREA => count += 1,
                        _ => break,
                    }
                }
                if count != 4 {
                    dist[i] = 0;
                }
            }
        }
    }
    let relax = |dist: &mut [u8], i: usize, j: usize, add: i32| {
        let nd = (i32::from(dist[j]) + add).min(255) as u8;
        if nd < dist[i] {
            dist[i] = nd;
        }
    };
    for z in 0..h {
        ws.work(1)?;
        for x in 0..w {
            let c = chf.cells[(x + z * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if let Some((ax, az, ai)) = nei(chf, x, z, i, 0) {
                    relax(&mut dist, i, ai, 2);
                    if let Some((_, _, bi)) = nei(chf, ax, az, ai, 3) {
                        relax(&mut dist, i, bi, 3);
                    }
                }
                if let Some((ax, az, ai)) = nei(chf, x, z, i, 3) {
                    relax(&mut dist, i, ai, 2);
                    if let Some((_, _, bi)) = nei(chf, ax, az, ai, 2) {
                        relax(&mut dist, i, bi, 3);
                    }
                }
            }
        }
    }
    for z in (0..h).rev() {
        ws.work(1)?;
        for x in (0..w).rev() {
            let c = chf.cells[(x + z * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if let Some((ax, az, ai)) = nei(chf, x, z, i, 2) {
                    relax(&mut dist, i, ai, 2);
                    if let Some((_, _, bi)) = nei(chf, ax, az, ai, 1) {
                        relax(&mut dist, i, bi, 3);
                    }
                }
                if let Some((ax, az, ai)) = nei(chf, x, z, i, 1) {
                    relax(&mut dist, i, ai, 2);
                    if let Some((_, _, bi)) = nei(chf, ax, az, ai, 0) {
                        relax(&mut dist, i, bi, 3);
                    }
                }
            }
        }
    }
    let thr = (radius.wrapping_mul(2)) as u8;
    for (d, a) in dist.iter().zip(chf.areas.iter_mut()) {
        if *d < thr {
            *a = NULL_AREA;
        }
    }
    Ok(())
}

/// Replaces each walkable span's area with the median of its 3x3 neighbourhood
/// (`rcMedianFilterWalkableArea`).
pub fn median_filter_walkable_area(
    ws: &mut Workspace,
    chf: &mut CompactHeightfield,
) -> Result<(), Error> {
    ws.begin();
    check_chf(ws, chf)?;
    let w = chf.width;
    let h = chf.height;
    ws.work(chf.spans.len() as u64)?;
    let mut areas = ws.vec(chf.spans.len(), 0xffu8)?;
    for z in 0..h {
        ws.work(1)?;
        for x in 0..w {
            let c = chf.cells[(x + z * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA {
                    areas[i] = chf.areas[i];
                    continue;
                }
                let mut nb = [chf.areas[i]; 9];
                for dir in 0..4 {
                    let Some((ax, az, ai)) = nei(chf, x, z, i, dir) else {
                        continue;
                    };
                    if chf.areas[ai] != NULL_AREA {
                        nb[(dir * 2) as usize] = chf.areas[ai];
                    }
                    let dir2 = (dir + 1) & 3;
                    if let Some((_, _, bi)) = nei(chf, ax, az, ai, dir2)
                        && chf.areas[bi] != NULL_AREA
                    {
                        nb[(dir * 2 + 1) as usize] = chf.areas[bi];
                    }
                }
                // Insertion sort, as upstream (the median does not depend on it).
                nb.sort_unstable();
                areas[i] = nb[4];
            }
        }
    }
    chf.areas.copy_from_slice(&areas);
    Ok(())
}

fn cell_range(
    chf: &CompactHeightfield,
    bmin: &[f32; 3],
    bmax: &[f32; 3],
) -> Option<(i32, i32, i32, i32, i32, i32)> {
    let minx = ((bmin[0] - chf.bmin[0]) / chf.cs) as i32;
    let miny = ((bmin[1] - chf.bmin[1]) / chf.ch) as i32;
    let minz = ((bmin[2] - chf.bmin[2]) / chf.cs) as i32;
    let maxx = ((bmax[0] - chf.bmin[0]) / chf.cs) as i32;
    let maxy = ((bmax[1] - chf.bmin[1]) / chf.ch) as i32;
    let maxz = ((bmax[2] - chf.bmin[2]) / chf.cs) as i32;
    if maxx < 0 || minx >= chf.width || maxz < 0 || minz >= chf.height {
        return None;
    }
    Some((
        minx.max(0),
        miny,
        minz.max(0),
        maxx.min(chf.width - 1),
        maxy,
        maxz.min(chf.height - 1),
    ))
}

/// Sets the area of walkable spans inside an axis-aligned box (`rcMarkBoxArea`).
pub fn mark_box_area(
    ws: &mut Workspace,
    bmin: [f32; 3],
    bmax: [f32; 3],
    area: u8,
    chf: &mut CompactHeightfield,
) -> Result<(), Error> {
    ws.begin();
    check_chf(ws, chf)?;
    if bmin.iter().chain(&bmax).any(|v| !v.is_finite()) {
        return Err(Error::InvalidInput("non-finite box bounds"));
    }
    let Some((minx, miny, minz, maxx, maxy, maxz)) = cell_range(chf, &bmin, &bmax) else {
        return Ok(());
    };
    ws.work(((maxx - minx + 1) as u64) * ((maxz - minz + 1) as u64))?;
    for z in minz..=maxz {
        ws.work(1)?;
        for x in minx..=maxx {
            let c = chf.cells[(x + z * chf.width) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                let y = i32::from(chf.spans[i].y);
                if y < miny || y > maxy || chf.areas[i] == NULL_AREA {
                    continue;
                }
                chf.areas[i] = area;
            }
        }
    }
    Ok(())
}

fn point_in_poly(verts: &[f32], p: &[f32; 3]) -> bool {
    let n = verts.len() / 3;
    let mut inside = false;
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let vi = &verts[i * 3..];
        let vj = &verts[j * 3..];
        j = i;
        if (vi[2] > p[2]) == (vj[2] > p[2]) {
            continue;
        }
        if p[0] >= (vj[0] - vi[0]) * (p[2] - vi[2]) / (vj[2] - vi[2]) + vi[0] {
            continue;
        }
        inside = !inside;
    }
    inside
}

/// Sets the area of walkable spans inside a convex polygon extruded between
/// `hmin` and `hmax` (`rcMarkConvexPolyArea`). `verts` holds x, y, z per point.
pub fn mark_convex_poly_area(
    ws: &mut Workspace,
    verts: &[f32],
    hmin: f32,
    hmax: f32,
    area: u8,
    chf: &mut CompactHeightfield,
) -> Result<(), Error> {
    ws.begin();
    check_chf(ws, chf)?;
    ws.work(verts.len() as u64)?;
    if verts.len() < 3 || !verts.len().is_multiple_of(3) {
        return Err(Error::InvalidInput("polygon vertex array"));
    }
    if verts.iter().any(|v| !v.is_finite()) || !hmin.is_finite() || !hmax.is_finite() {
        return Err(Error::InvalidInput("non-finite polygon coordinate"));
    }
    let mut bmin = v3(verts);
    let mut bmax = v3(verts);
    for v in verts.chunks_exact(3).skip(1) {
        vmin(&mut bmin, v);
        vmax(&mut bmax, v);
    }
    bmin[1] = hmin;
    bmax[1] = hmax;
    let Some((minx, miny, minz, maxx, maxy, maxz)) = cell_range(chf, &bmin, &bmax) else {
        return Ok(());
    };
    ws.work(((maxx - minx + 1) as u64) * ((maxz - minz + 1) as u64))?;
    for z in minz..=maxz {
        ws.work(1)?;
        for x in minx..=maxx {
            let c = chf.cells[(x + z * chf.width) as usize];
            ws.work(
                u64::from(c.count)
                    .checked_mul((verts.len() / 3) as u64)
                    .ok_or(Error::SizeOverflow)?,
            )?;
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA {
                    continue;
                }
                let y = i32::from(chf.spans[i].y);
                if y < miny || y > maxy {
                    continue;
                }
                let p = [
                    chf.bmin[0] + (x as f32 + 0.5) * chf.cs,
                    0.0,
                    chf.bmin[2] + (z as f32 + 0.5) * chf.cs,
                ];
                if point_in_poly(verts, &p) {
                    chf.areas[i] = area;
                }
            }
        }
    }
    Ok(())
}

/// Sets the area of walkable spans inside a vertical cylinder (`rcMarkCylinderArea`).
pub fn mark_cylinder_area(
    ws: &mut Workspace,
    pos: [f32; 3],
    radius: f32,
    height: f32,
    area: u8,
    chf: &mut CompactHeightfield,
) -> Result<(), Error> {
    ws.begin();
    check_chf(ws, chf)?;
    if pos.iter().any(|v| !v.is_finite()) || !radius.is_finite() || !height.is_finite() {
        return Err(Error::InvalidInput("non-finite cylinder"));
    }
    let bmin = [pos[0] - radius, pos[1], pos[2] - radius];
    let bmax = [pos[0] + radius, pos[1] + height, pos[2] + radius];
    let Some((minx, miny, minz, maxx, maxy, maxz)) = cell_range(chf, &bmin, &bmax) else {
        return Ok(());
    };
    ws.work(((maxx - minx + 1) as u64) * ((maxz - minz + 1) as u64))?;
    let r2 = radius * radius;
    for z in minz..=maxz {
        ws.work(1)?;
        for x in minx..=maxx {
            let c = chf.cells[(x + z * chf.width) as usize];
            let cx = chf.bmin[0] + (x as f32 + 0.5) * chf.cs;
            let cz = chf.bmin[2] + (z as f32 + 0.5) * chf.cs;
            let dx = cx - pos[0];
            let dz = cz - pos[2];
            if sqr(dx) + sqr(dz) >= r2 {
                continue;
            }
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA {
                    continue;
                }
                let y = i32::from(chf.spans[i].y);
                if y >= miny && y <= maxy {
                    chf.areas[i] = area;
                }
            }
        }
    }
    Ok(())
}

fn safe_normalize(v: &mut [f32; 3]) {
    let sq = sqr(v[0]) + sqr(v[1]) + sqr(v[2]);
    if sq > 1e-6 {
        let inv = 1.0 / sqrtf(sq);
        v[0] *= inv;
        v[1] *= inv;
        v[2] *= inv;
    }
}

/// Expands a convex polygon by `offset`, beveling sharp corners (`rcOffsetPoly`).
/// Appends to `out` (x, y, z per point) and returns the number of points written,
/// or 0 when more than `max_out` points would be needed.
pub fn offset_poly(
    verts: &[f32],
    offset: f32,
    out: &mut alloc::vec::Vec<f32>,
    max_out: usize,
) -> usize {
    const MITER_LIMIT: f32 = 1.20;
    let n = verts.len() / 3;
    let start = out.len();
    let mut count = 0usize;
    for b in 0..n {
        let a = (b + n - 1) % n;
        let c = (b + 1) % n;
        let va = &verts[a * 3..a * 3 + 3];
        let vb = &verts[b * 3..b * 3 + 3];
        let vc = &verts[c * 3..c * 3 + 3];
        let mut prev = [vb[0] - va[0], 0.0, vb[2] - va[2]];
        safe_normalize(&mut prev);
        let mut cur = [vc[0] - vb[0], 0.0, vc[2] - vb[2]];
        safe_normalize(&mut cur);
        let cross = cur[0] * prev[2] - prev[0] * cur[2];
        let pnx = -prev[2];
        let pnz = prev[0];
        let cnx = -cur[2];
        let cnz = cur[0];
        let mut mx = (pnx + cnx) * 0.5;
        let mut mz = (pnz + cnz) * 0.5;
        let msq = sqr(mx) + sqr(mz);
        let bevel = msq * MITER_LIMIT * MITER_LIMIT < 1.0;
        if msq > 1e-6 {
            let scale = 1.0 / msq;
            mx *= scale;
            mz *= scale;
        }
        if bevel && cross < 0.0 {
            if count + 2 > max_out {
                out.truncate(start);
                return 0;
            }
            let d = (1.0 - (prev[0] * cur[0] + prev[2] * cur[2])) * 0.5;
            out.extend_from_slice(&[
                vb[0] + (-pnx + prev[0] * d) * offset,
                vb[1],
                vb[2] + (-pnz + prev[2] * d) * offset,
                vb[0] + (-cnx - cur[0] * d) * offset,
                vb[1],
                vb[2] + (-cnz - cur[2] * d) * offset,
            ]);
            count += 2;
        } else {
            if count + 1 > max_out {
                out.truncate(start);
                return 0;
            }
            out.extend_from_slice(&[vb[0] - mx * offset, vb[1], vb[2] - mz * offset]);
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use crate::recast::{CompactCell, CompactSpan};

    #[test]
    fn dense_connections_match_generic_validation() {
        let mut chf = CompactHeightfield {
            width: 3,
            height: 3,
            span_count: 9,
            cells: (0..9).map(|i| CompactCell { index: i, count: 1 }).collect(),
            spans: alloc::vec![CompactSpan { con: 0xffffff, ..CompactSpan::default() }; 9],
            areas: alloc::vec![63; 9],
            ..CompactHeightfield::default()
        };
        for i in 0..9 {
            for dir in 0..4 {
                for layer in 0..64 {
                    chf.spans[i].set_con(dir, layer);
                    let mut fast = Workspace::new();
                    let mut generic = Workspace::with_limits(crate::Limits {
                        max_work: 1000,
                        ..crate::Limits::default()
                    });
                    assert_eq!(check_chf(&mut fast, &chf), check_chf(&mut generic, &chf));
                    chf.spans[i].set_con(dir, NOT_CONNECTED);
                }
            }
        }
    }

    fn sparse_field() -> CompactHeightfield {
        // 3x3 grid, centre column empty: eight single-layer spans.
        let mut index = 0;
        let cells = (0..9)
            .map(|i| {
                let count = u8::from(i != 4);
                let cell = CompactCell { index, count };
                index += u32::from(count);
                cell
            })
            .collect();
        CompactHeightfield {
            width: 3,
            height: 3,
            span_count: 8,
            cells,
            spans: alloc::vec![CompactSpan { con: 0xffffff, ..CompactSpan::default() }; 8],
            areas: alloc::vec![63; 8],
            ..CompactHeightfield::default()
        }
    }

    #[test]
    fn sparse_single_layer_matches_generic_validation() {
        let mut chf = sparse_field();
        for i in 0..8 {
            for dir in 0..4 {
                for layer in 0..64 {
                    chf.spans[i].set_con(dir, layer);
                    let mut fast = Workspace::new();
                    let mut generic = Workspace::with_limits(crate::Limits {
                        max_work: 1000,
                        ..crate::Limits::default()
                    });
                    assert_eq!(check_chf(&mut fast, &chf), check_chf(&mut generic, &chf));
                    chf.spans[i].set_con(dir, NOT_CONNECTED);
                }
            }
        }
    }

    #[test]
    fn trusted_compact_field_charges_like_validation() {
        let chf = sparse_field();
        for max_work in [u64::MAX, 1000] {
            let limits = crate::Limits {
                max_work,
                ..crate::Limits::default()
            };
            let mut checked = Workspace::with_limits(limits);
            let mut trusted = Workspace::with_limits(limits);
            trusted.set_compact_trusted(true);
            checked.begin();
            trusted.begin();
            assert_eq!(check_chf(&mut checked, &chf), Ok(()));
            assert_eq!(check_chf(&mut trusted, &chf), Ok(()));
            assert_eq!(checked.usage(), trusted.usage());
        }
        // A budget that runs out inside the row charges fails the same way.
        let tight = crate::Limits {
            max_work: 8 + 8 * 4 + 1,
            ..crate::Limits::default()
        };
        let mut checked = Workspace::with_limits(tight);
        let mut trusted = Workspace::with_limits(tight);
        trusted.set_compact_trusted(true);
        assert_eq!(check_chf(&mut checked, &chf), Err(Error::LimitExceeded));
        assert_eq!(check_chf(&mut trusted, &chf), Err(Error::LimitExceeded));
        assert_eq!(checked.usage(), trusted.usage());
    }

    #[test]
    fn checks_every_direction_and_layer_without_rejecting_disconnected_edges() {
        let mut chf = CompactHeightfield {
            width: 3,
            height: 3,
            span_count: 18,
            cells: (0..9)
                .map(|i| CompactCell {
                    index: i * 2,
                    count: 2,
                })
                .collect(),
            spans: alloc::vec![CompactSpan { con: 0xffffff, ..CompactSpan::default() }; 18],
            areas: alloc::vec![63; 18],
            ..CompactHeightfield::default()
        };
        let mut ws = Workspace::new();
        assert_eq!(check_chf(&mut ws, &chf), Ok(()));
        for z in 0..3 {
            for x in 0..3 {
                let i = (z * 3 + x) as usize * 2;
                for dir in 0..4 {
                    let nx = x + dir_offset_x(dir);
                    let nz = z + dir_offset_y(dir);
                    let interior = (0..3).contains(&nx) && (0..3).contains(&nz);
                    for layer in [0, 1, 2, 62, 63] {
                        chf.spans[i + 1].set_con(dir, layer);
                        assert_eq!(
                            check_chf(&mut ws, &chf).is_ok(),
                            layer == 63 || (interior && layer < 2)
                        );
                    }
                    chf.spans[i + 1].set_con(dir, NOT_CONNECTED);
                }
            }
        }
        chf.cells[4].index = 18;
        assert!(check_chf(&mut ws, &chf).is_err());
    }
}
