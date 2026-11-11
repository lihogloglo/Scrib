//! The voxel heightfield: creation, triangle rasterization and the three span
//! filters (upstream `Recast.cpp`, `RecastRasterization.cpp`, `RecastFilter.cpp`).

use alloc::vec::Vec;

use super::{NULL_AREA, SPAN_MAX_HEIGHT, dir_offset_x, dir_offset_y, validate_mesh};
use crate::common::{ceil_i32, clamp, floor_i32, fmax, fmin, v3, vmax, vmin};
use crate::{Error, Workspace};

const NONE: u32 = u32::MAX;
const MAX_HEIGHTFIELD_HEIGHT: i32 = 0xffff;

/// One solid span: a vertical run of voxels in a column (`rcSpan`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Lower limit, in cells (13 bits upstream).
    pub smin: u16,
    /// Upper limit, in cells (13 bits upstream).
    pub smax: u16,
    /// Area id (6 bits upstream).
    pub area: u8,
    next: u32,
}

/// A dynamic heightfield of solid spans (`rcHeightfield`). Spans of a column are
/// kept bottom to top in a linked list over one span pool, as upstream does.
#[derive(Clone, Debug, Default)]
pub struct Heightfield {
    /// Width in cells (x).
    pub width: i32,
    /// Height in cells (z).
    pub height: i32,
    /// Minimum bounds in world space.
    pub bmin: [f32; 3],
    /// Maximum bounds in world space.
    pub bmax: [f32; 3],
    /// Cell size (xz).
    pub cs: f32,
    /// Cell height (y).
    pub ch: f32,
    heads: Vec<u32>,
    pool: Vec<Span>,
    free: u32,
}

/// Spans of one column, bottom to top.
pub struct ColumnIter<'a> {
    hf: &'a Heightfield,
    cur: u32,
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = &'a Span;
    fn next(&mut self) -> Option<&'a Span> {
        let s = self.hf.pool.get(self.cur as usize)?;
        self.cur = s.next;
        Some(s)
    }
}

impl Heightfield {
    /// An empty heightfield over the given grid (`rcCreateHeightfield`).
    /// Both dimensions must be positive or both zero; a grid with exactly one
    /// zero dimension returns [`Error::InvalidParameter`].
    pub fn new(
        ws: &mut Workspace,
        width: i32,
        height: i32,
        bmin: [f32; 3],
        bmax: [f32; 3],
        cs: f32,
        ch: f32,
    ) -> Result<Heightfield, Error> {
        ws.begin();
        if width < 0 || height < 0 || (width == 0) != (height == 0) {
            return Err(Error::InvalidParameter("heightfield size"));
        }
        if !(cs > 0.0 && ch > 0.0 && cs.is_finite() && ch.is_finite()) {
            return Err(Error::InvalidParameter("cell size"));
        }
        if bmin.iter().chain(&bmax).any(|v| !v.is_finite()) {
            return Err(Error::InvalidParameter("bounds"));
        }
        let cells = (width as usize)
            .checked_mul(height as usize)
            .ok_or(Error::SizeOverflow)?;
        if cells >= u32::MAX as usize {
            return Err(Error::SizeOverflow);
        }
        let heads = ws.vec(cells, NONE)?;
        Ok(Heightfield {
            width,
            height,
            bmin,
            bmax,
            cs,
            ch,
            heads,
            pool: Vec::new(),
            free: NONE,
        })
    }

    /// Spans of the column at `x + z * width`, bottom to top.
    pub fn column(&self, index: usize) -> ColumnIter<'_> {
        ColumnIter {
            hf: self,
            cur: self.heads.get(index).copied().unwrap_or(NONE),
        }
    }

    pub(crate) fn span_count_for_report(&self) -> usize {
        self.pool.len()
    }

    /// Number of spans whose area is not [`NULL_AREA`] (`rcGetHeightFieldSpanCount`).
    pub fn walkable_span_count(&self) -> usize {
        (0..self.heads.len())
            .map(|i| self.column(i).filter(|s| s.area != NULL_AREA).count())
            .sum()
    }

    /// Checks that the public grid fields still match the span storage.
    pub(crate) fn check(&self) -> Result<(), Error> {
        let cells = (self.width.max(0) as usize).checked_mul(self.height.max(0) as usize);
        if self.width < 0
            || self.height < 0
            || (self.width == 0) != (self.height == 0)
            || cells != Some(self.heads.len())
        {
            return Err(Error::InvalidInput(
                "heightfield size changed after creation",
            ));
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn head(&self, i: usize) -> u32 {
        self.heads[i]
    }

    #[inline(always)]
    pub(crate) fn span(&self, s: u32) -> &Span {
        &self.pool[s as usize]
    }

    #[inline(always)]
    pub(crate) fn next_of(&self, s: u32) -> u32 {
        self.pool[s as usize].next
    }

    fn alloc_span(&mut self, ws: &mut Workspace) -> Result<u32, Error> {
        if self.free != NONE {
            let s = self.free;
            self.free = self.pool[s as usize].next;
            return Ok(s);
        }
        if self.pool.len() >= NONE as usize - 1 {
            return Err(Error::SizeOverflow);
        }
        ws.push(
            &mut self.pool,
            Span {
                smin: 0,
                smax: 0,
                area: 0,
                next: NONE,
            },
        )?;
        Ok((self.pool.len() - 1) as u32)
    }

    fn insert_span(
        &mut self,
        ws: &mut Workspace,
        x: i32,
        z: i32,
        smin: u16,
        smax: u16,
        area: u8,
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        let new = self.alloc_span(ws)?;
        let mut nsmin = smin;
        let mut nsmax = smax;
        let mut narea = area;
        let col = (x + z * self.width) as usize;
        let mut prev = NONE;
        let mut cur = self.heads[col];
        while cur != NONE {
            let c = self.pool[cur as usize];
            if c.smin > nsmax {
                break;
            }
            if c.smax < nsmin {
                prev = cur;
                cur = c.next;
            } else {
                if c.smin < nsmin {
                    nsmin = c.smin;
                }
                if c.smax > nsmax {
                    nsmax = c.smax;
                }
                if (i32::from(nsmax) - i32::from(c.smax)).abs() <= flag_merge_thr {
                    narea = narea.max(c.area);
                }
                let next = c.next;
                self.pool[cur as usize].next = self.free;
                self.free = cur;
                if prev != NONE {
                    self.pool[prev as usize].next = next;
                } else {
                    self.heads[col] = next;
                }
                cur = next;
            }
        }
        let link = if prev != NONE {
            let l = self.pool[prev as usize].next;
            self.pool[prev as usize].next = new;
            l
        } else {
            let l = self.heads[col];
            self.heads[col] = new;
            l
        };
        self.pool[new as usize] = Span {
            smin: nsmin,
            smax: nsmax,
            area: narea,
            next: link,
        };
        Ok(())
    }

    /// Adds a span to a column, merging with overlapping spans (`rcAddSpan`). A span
    /// with `smin >= smax` is ignored, as upstream does.
    #[allow(clippy::too_many_arguments)]
    pub fn add_span(
        &mut self,
        ws: &mut Workspace,
        x: i32,
        z: i32,
        smin: u16,
        smax: u16,
        area: u8,
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        if x < 0 || z < 0 || x >= self.width || z >= self.height {
            return Err(Error::InvalidParameter("span column"));
        }
        if i32::from(smax) > SPAN_MAX_HEIGHT || area > 63 {
            return Err(Error::InvalidParameter("span height or area"));
        }
        if smin >= smax {
            return Ok(());
        }
        self.insert_span(ws, x, z, smin, smax, area, flag_merge_thr)
    }

    /// Rasterizes indexed triangles into the heightfield (`rcRasterizeTriangles`).
    /// `verts` holds x, y, z per vertex, `tris` three vertex indices per triangle and
    /// `areas` one area id per triangle.
    pub fn rasterize_triangles(
        &mut self,
        ws: &mut Workspace,
        verts: &[f32],
        tris: &[i32],
        areas: &[u8],
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        validate_mesh(verts, tris, areas.len())?;
        if areas.iter().any(|&a| a > 63) {
            return Err(Error::InvalidInput("area id above 63"));
        }
        let ics = 1.0 / self.cs;
        let ich = 1.0 / self.ch;
        for (tri, &area) in tris.chunks_exact(3).zip(areas) {
            let v0 = v3(&verts[tri[0] as usize * 3..]);
            let v1 = v3(&verts[tri[1] as usize * 3..]);
            let v2 = v3(&verts[tri[2] as usize * 3..]);
            self.rasterize_tri(ws, &v0, &v1, &v2, area, ics, ich, flag_merge_thr)?;
        }
        Ok(())
    }

    /// [`Heightfield::rasterize_triangles`] with 16-bit indices (the
    /// `unsigned short` overload of `rcRasterizeTriangles`).
    pub fn rasterize_triangles_u16(
        &mut self,
        ws: &mut Workspace,
        verts: &[f32],
        tris: &[u16],
        areas: &[u8],
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        ws.begin();
        let mut wide = ws.vec_with_capacity::<i32>(tris.len())?;
        wide.extend(tris.iter().map(|&i| i32::from(i)));
        self.rasterize_triangles(ws, verts, &wide, areas, flag_merge_thr)
    }

    /// Rasterizes a triangle list without indices: nine floats per triangle, one
    /// area id per triangle (the overload of `rcRasterizeTriangles` that takes
    /// `numTris` and no index array).
    pub fn rasterize_triangle_soup(
        &mut self,
        ws: &mut Workspace,
        verts: &[f32],
        areas: &[u8],
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        if !verts.len().is_multiple_of(9) || verts.len() / 9 != areas.len() {
            return Err(Error::InvalidInput(
                "vertex array is not nine floats per triangle area",
            ));
        }
        if verts.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput("non-finite vertex coordinate"));
        }
        if areas.iter().any(|&a| a > 63) {
            return Err(Error::InvalidInput("area id above 63"));
        }
        let ics = 1.0 / self.cs;
        let ich = 1.0 / self.ch;
        for (t, &area) in verts.chunks_exact(9).zip(areas) {
            let (v0, v1, v2) = (v3(&t[0..]), v3(&t[3..]), v3(&t[6..]));
            self.rasterize_tri(ws, &v0, &v1, &v2, area, ics, ich, flag_merge_thr)?;
        }
        Ok(())
    }

    /// Rasterizes one triangle (`rcRasterizeTriangle`).
    pub fn rasterize_triangle(
        &mut self,
        ws: &mut Workspace,
        v0: [f32; 3],
        v1: [f32; 3],
        v2: [f32; 3],
        area: u8,
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        if v0.iter().chain(&v1).chain(&v2).any(|v| !v.is_finite()) {
            return Err(Error::InvalidInput("non-finite vertex coordinate"));
        }
        if area > 63 {
            return Err(Error::InvalidInput("area id above 63"));
        }
        let ics = 1.0 / self.cs;
        let ich = 1.0 / self.ch;
        self.rasterize_tri(ws, &v0, &v1, &v2, area, ics, ich, flag_merge_thr)
    }

    #[allow(clippy::too_many_arguments)]
    fn rasterize_tri(
        &mut self,
        ws: &mut Workspace,
        v0: &[f32; 3],
        v1: &[f32; 3],
        v2: &[f32; 3],
        area: u8,
        ics: f32,
        ich: f32,
        flag_merge_thr: i32,
    ) -> Result<(), Error> {
        let bmin = self.bmin;
        let bmax = self.bmax;
        let cs = self.cs;
        let mut tmin = *v0;
        vmin(&mut tmin, v1);
        vmin(&mut tmin, v2);
        let mut tmax = *v0;
        vmax(&mut tmax, v1);
        vmax(&mut tmax, v2);
        let overlap = tmin[0] <= bmax[0]
            && tmax[0] >= bmin[0]
            && tmin[1] <= bmax[1]
            && tmax[1] >= bmin[1]
            && tmin[2] <= bmax[2]
            && tmax[2] >= bmin[2];
        if !overlap {
            return Ok(());
        }
        let w = self.width;
        let h = self.height;
        let by = bmax[1] - bmin[1];
        let z0 = clamp(((tmin[2] - bmin[2]) * ics) as i32, -1, h - 1);
        let z1 = clamp(((tmax[2] - bmin[2]) * ics) as i32, 0, h - 1);
        ws.work(((z1 - z0 + 1).max(0)) as u64)?;

        // Four polygon buffers whose roles (in, inRow, p1, p2) are permuted by
        // reference swaps, exactly like upstream's pointer swaps.
        let mut a = [0.0f32; MAX_POLY_VERTS * 3];
        let mut b = [0.0f32; MAX_POLY_VERTS * 3];
        let mut c = [0.0f32; MAX_POLY_VERTS * 3];
        let mut d = [0.0f32; MAX_POLY_VERTS * 3];
        let (mut inp, mut row, mut p1, mut p2) = (&mut a, &mut b, &mut c, &mut d);
        inp[0..3].copy_from_slice(v0);
        inp[3..6].copy_from_slice(v1);
        inp[6..9].copy_from_slice(v2);
        let mut nv_in = 3usize;
        for z in z0..=z1 {
            let cell_z = bmin[2] + z as f32 * cs;
            let (nv_row, nv_rest) = divide_poly(&inp[..], nv_in, row, p1, cell_z + cs, 2)?;
            core::mem::swap(&mut inp, &mut p1);
            nv_in = nv_rest;
            if nv_row < 3 || z < 0 {
                continue;
            }
            let mut min_x = row[0];
            let mut max_x = row[0];
            for v in 1..nv_row {
                if min_x > row[v * 3] {
                    min_x = row[v * 3];
                }
                if max_x < row[v * 3] {
                    max_x = row[v * 3];
                }
            }
            let x0 = ((min_x - bmin[0]) * ics) as i32;
            let x1 = ((max_x - bmin[0]) * ics) as i32;
            if x1 < 0 || x0 >= w {
                continue;
            }
            let x0 = clamp(x0, -1, w - 1);
            let x1 = clamp(x1, 0, w - 1);
            ws.work((x1 - x0 + 1) as u64)?;
            let mut nv2 = nv_row;
            for x in x0..=x1 {
                let cx = bmin[0] + x as f32 * cs;
                let (nv, rest) = divide_poly(&row[..], nv2, p1, p2, cx + cs, 0)?;
                core::mem::swap(&mut row, &mut p2);
                nv2 = rest;
                if nv < 3 || x < 0 {
                    continue;
                }
                let mut smin = p1[1];
                let mut smax = p1[1];
                for v in 1..nv {
                    smin = fmin(smin, p1[v * 3 + 1]);
                    smax = fmax(smax, p1[v * 3 + 1]);
                }
                smin -= bmin[1];
                smax -= bmin[1];
                if smax < 0.0 || smin > by {
                    continue;
                }
                if smin < 0.0 {
                    smin = 0.0;
                }
                if smax > by {
                    smax = by;
                }
                let ismin = clamp(floor_i32(smin * ich), 0, SPAN_MAX_HEIGHT) as u16;
                let ismax =
                    clamp(ceil_i32(smax * ich), i32::from(ismin) + 1, SPAN_MAX_HEIGHT) as u16;
                self.insert_span(ws, x, z, ismin, ismax, area, flag_merge_thr)?;
            }
        }
        Ok(())
    }

    /// Marks non-walkable spans walkable when their top is within `walkable_climb`
    /// of a walkable span right below (`rcFilterLowHangingWalkableObstacles`).
    pub fn filter_low_hanging_walkable_obstacles(
        &mut self,
        ws: &mut Workspace,
        walkable_climb: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        ws.work(self.heads.len() as u64)?;
        for col in 0..self.heads.len() {
            let mut prev = NONE;
            let mut prev_walkable = false;
            let mut prev_area = NULL_AREA;
            let mut s = self.heads[col];
            while s != NONE {
                let walkable = self.pool[s as usize].area != NULL_AREA;
                if !walkable
                    && prev_walkable
                    && i32::from(self.pool[s as usize].smax)
                        - i32::from(self.pool[prev as usize].smax)
                        <= walkable_climb
                {
                    self.pool[s as usize].area = prev_area;
                }
                prev_walkable = walkable;
                prev_area = self.pool[s as usize].area;
                prev = s;
                s = self.pool[s as usize].next;
            }
        }
        Ok(())
    }

    /// Removes walkable spans next to drops higher than `walkable_climb`, and spans
    /// on steep stepped ground (`rcFilterLedgeSpans`).
    pub fn filter_ledge_spans(
        &mut self,
        ws: &mut Workspace,
        walkable_height: i32,
        walkable_climb: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        ws.work(self.heads.len() as u64)?;
        let xs = self.width;
        let zs = self.height;
        for z in 0..zs {
            for x in 0..xs {
                let mut s = self.heads[(x + z * xs) as usize];
                while s != NONE {
                    let span = self.pool[s as usize];
                    if span.area == NULL_AREA {
                        s = span.next;
                        continue;
                    }
                    let floor = i32::from(span.smax);
                    let ceiling = if span.next != NONE {
                        i32::from(self.pool[span.next as usize].smin)
                    } else {
                        MAX_HEIGHTFIELD_HEIGHT
                    };
                    let mut lowest_diff = MAX_HEIGHTFIELD_HEIGHT;
                    let mut lowest_trav = i32::from(span.smax);
                    let mut highest_trav = i32::from(span.smax);
                    for dir in 0..4 {
                        let nx = x + dir_offset_x(dir);
                        let nz = z + dir_offset_y(dir);
                        if nx < 0 || nz < 0 || nx >= xs || nz >= zs {
                            lowest_diff = -walkable_climb - 1;
                            break;
                        }
                        let mut ns = self.heads[(nx + nz * xs) as usize];
                        let mut n_ceiling = if ns != NONE {
                            i32::from(self.pool[ns as usize].smin)
                        } else {
                            MAX_HEIGHTFIELD_HEIGHT
                        };
                        if ceiling.min(n_ceiling) - floor >= walkable_height {
                            lowest_diff = -walkable_climb - 1;
                            break;
                        }
                        while ns != NONE {
                            let n = self.pool[ns as usize];
                            let n_floor = i32::from(n.smax);
                            n_ceiling = if n.next != NONE {
                                i32::from(self.pool[n.next as usize].smin)
                            } else {
                                MAX_HEIGHTFIELD_HEIGHT
                            };
                            ns = n.next;
                            if ceiling.min(n_ceiling) - floor.max(n_floor) < walkable_height {
                                continue;
                            }
                            let d = n_floor - floor;
                            lowest_diff = lowest_diff.min(d);
                            if d.abs() <= walkable_climb {
                                lowest_trav = lowest_trav.min(n_floor);
                                highest_trav = highest_trav.max(n_floor);
                            } else if d < -walkable_climb {
                                break;
                            }
                        }
                    }
                    if lowest_diff < -walkable_climb || highest_trav - lowest_trav > walkable_climb
                    {
                        self.pool[s as usize].area = NULL_AREA;
                    }
                    s = span.next;
                }
            }
        }
        Ok(())
    }

    /// Removes walkable spans with less than `walkable_height` of free space above
    /// (`rcFilterWalkableLowHeightSpans`).
    pub fn filter_walkable_low_height_spans(
        &mut self,
        ws: &mut Workspace,
        walkable_height: i32,
    ) -> Result<(), Error> {
        ws.begin();
        self.check()?;
        ws.work(self.heads.len() as u64)?;
        for col in 0..self.heads.len() {
            let mut s = self.heads[col];
            while s != NONE {
                let span = self.pool[s as usize];
                let floor = i32::from(span.smax);
                let ceiling = if span.next != NONE {
                    i32::from(self.pool[span.next as usize].smin)
                } else {
                    MAX_HEIGHTFIELD_HEIGHT
                };
                if ceiling - floor < walkable_height {
                    self.pool[s as usize].area = NULL_AREA;
                }
                s = span.next;
            }
        }
        Ok(())
    }
}

/// Most vertices a polygon buffer of the rasterizer holds (upstream's `7`).
///
/// A triangle clipped by axis-aligned planes stays within this bound in exact
/// arithmetic. In `f32` the interpolated cut points of a triangle that is huge
/// next to the grid can fall on the wrong side of a later plane, so the clipped
/// polygon stops being convex, crosses a plane more than twice and grows past
/// the buffers (upstream writes out of bounds there).
const MAX_POLY_VERTS: usize = 7;

/// Appends vertex `src` to `out`, or fails when the buffer is full.
#[inline(always)]
fn push_vertex(
    out: &mut [f32; MAX_POLY_VERTS * 3],
    count: &mut usize,
    src: &[f32],
) -> Result<(), Error> {
    if *count >= MAX_POLY_VERTS {
        return Err(Error::InvalidInput(
            "triangle coordinates too large for single-precision clipping",
        ));
    }
    out[*count * 3..*count * 3 + 3].copy_from_slice(&src[..3]);
    *count += 1;
    Ok(())
}

/// Splits a convex polygon along `axis` at `offset` (`dividePoly`): returns the
/// vertex counts of the part below the line (`out1`) and above it (`out2`).
///
/// `n` is at most [`MAX_POLY_VERTS`]. Fails with [`Error::InvalidInput`] instead
/// of overflowing when either part would exceed that many vertices.
fn divide_poly(
    input: &[f32],
    n: usize,
    out1: &mut [f32; MAX_POLY_VERTS * 3],
    out2: &mut [f32; MAX_POLY_VERTS * 3],
    offset: f32,
    axis: usize,
) -> Result<(usize, usize), Error> {
    if n > MAX_POLY_VERTS || input.len() < n * 3 {
        return Err(Error::InvalidInput(
            "triangle coordinates too large for single-precision clipping",
        ));
    }
    let mut d = [0.0f32; MAX_POLY_VERTS];
    for i in 0..n {
        d[i] = offset - input[i * 3 + axis];
    }
    let mut m = 0usize;
    let mut k = 0usize;
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let same_side = (d[i] >= 0.0) == (d[j] >= 0.0);
        if !same_side {
            let s = d[j] / (d[j] - d[i]);
            let mut cut = [0.0f32; 3];
            for c in 0..3 {
                cut[c] = input[j * 3 + c] + (input[i * 3 + c] - input[j * 3 + c]) * s;
            }
            push_vertex(out1, &mut m, &cut)?;
            push_vertex(out2, &mut k, &cut)?;
            if d[i] > 0.0 {
                push_vertex(out1, &mut m, &input[i * 3..i * 3 + 3])?;
            } else if d[i] < 0.0 {
                push_vertex(out2, &mut k, &input[i * 3..i * 3 + 3])?;
            }
        } else {
            if d[i] >= 0.0 {
                push_vertex(out1, &mut m, &input[i * 3..i * 3 + 3])?;
                if d[i] != 0.0 {
                    j = i;
                    continue;
                }
            }
            push_vertex(out2, &mut k, &input[i * 3..i * 3 + 3])?;
        }
        j = i;
    }
    Ok((m, k))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A regular `n`-gon in the xz plane, centred on the origin.
    fn ngon(n: usize) -> [f32; MAX_POLY_VERTS * 3] {
        let mut p = [0.0f32; MAX_POLY_VERTS * 3];
        for i in 0..n {
            let a = i as f32 * core::f32::consts::TAU / n as f32;
            p[i * 3] = a.cos();
            p[i * 3 + 2] = a.sin();
        }
        p
    }

    #[test]
    fn divide_poly_fills_the_buffers_exactly_at_the_bound() {
        let p = ngon(7);
        let (mut o1, mut o2) = ([0.0; 21], [0.0; 21]);
        // Entirely on one side: a full 7-vertex part, copied unchanged.
        assert_eq!(divide_poly(&p, 7, &mut o1, &mut o2, 10.0, 0), Ok((7, 0)));
        assert_eq!(o1, p);
        assert_eq!(divide_poly(&p, 7, &mut o1, &mut o2, -10.0, 0), Ok((0, 7)));
        assert_eq!(o2, p);
        // One cut through the polygon: two cut points added to both parts.
        let (m, k) = divide_poly(&p, 7, &mut o1, &mut o2, 0.05, 2).expect("within bounds");
        assert_eq!((m, k), (6, 5));
    }

    #[test]
    fn divide_poly_reports_overflow_instead_of_writing_past_the_buffers() {
        // A zig-zag list crossing the plane at every edge cannot come from exact
        // clipping of a triangle, but rounding on huge coordinates produces such
        // lists. Six crossings put six cut points in each part, plus the vertices.
        let mut p = [0.0f32; MAX_POLY_VERTS * 3];
        for i in 0..MAX_POLY_VERTS {
            p[i * 3 + 2] = if i % 2 == 0 { -1.0 } else { 1.0 };
            p[i * 3] = i as f32;
        }
        let (mut o1, mut o2) = ([0.0; 21], [0.0; 21]);
        assert!(matches!(
            divide_poly(&p, 7, &mut o1, &mut o2, 0.0, 2),
            Err(Error::InvalidInput(_))
        ));
        // A list longer than the buffers is refused up front.
        assert!(matches!(
            divide_poly(&[0.0; 36], 12, &mut o1, &mut o2, 0.0, 2),
            Err(Error::InvalidInput(_))
        ));
    }
}
