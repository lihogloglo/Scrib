//! Distance field and the three region partitions: watershed, monotone and layers
//! (upstream `RecastRegion.cpp`).

use alloc::vec::Vec;

use super::area::check_chf as validate_chf;
use super::{BORDER_REG, CompactHeightfield, NOT_CONNECTED, NULL_AREA, dir_offset_x, dir_offset_y};
use crate::{Error, Workspace};

#[derive(Clone, Copy)]
struct Entry {
    x: i32,
    y: i32,
    index: i32,
}

#[inline(always)]
fn nei(chf: &CompactHeightfield, x: i32, y: i32, i: usize, dir: i32) -> Option<(i32, i32, usize)> {
    let c = chf.spans[i].con(dir);
    if c == NOT_CONNECTED {
        return None;
    }
    let ax = x + dir_offset_x(dir);
    let ay = y + dir_offset_y(dir);
    Some((
        ax,
        ay,
        chf.cells[(ax + ay * chf.width) as usize].index as usize + c as usize,
    ))
}

macro_rules! for_spans {
    ($chf:expr, $x:ident, $y:ident, $c:ident, $i:ident, $body:block) => {
        for $y in 0..$chf.height {
            for $x in 0..$chf.width {
                let $c = $chf.cells[($x + $y * $chf.width) as usize];
                for $i in $c.index as usize..$c.index as usize + $c.count as usize $body
            }
        }
    };
}

fn calculate_distance_field(chf: &CompactHeightfield, src: &mut [u16]) -> u16 {
    src.fill(0xffff);
    for_spans!(chf, x, y, c, i, {
        let area = chf.areas[i];
        let mut nc = 0;
        for dir in 0..4 {
            if let Some((_, _, ai)) = nei(chf, x, y, i, dir)
                && chf.areas[ai] == area
            {
                nc += 1;
            }
        }
        if nc != 4 {
            src[i] = 0;
        }
    });
    let relax = |src: &mut [u16], i: usize, j: usize, add: i32| {
        if i32::from(src[j]) + add < i32::from(src[i]) {
            src[i] = (i32::from(src[j]) + add) as u16;
        }
    };
    for_spans!(chf, x, y, c, i, {
        if let Some((ax, ay, ai)) = nei(chf, x, y, i, 0) {
            relax(src, i, ai, 2);
            if let Some((_, _, aai)) = nei(chf, ax, ay, ai, 3) {
                relax(src, i, aai, 3);
            }
        }
        if let Some((ax, ay, ai)) = nei(chf, x, y, i, 3) {
            relax(src, i, ai, 2);
            if let Some((_, _, aai)) = nei(chf, ax, ay, ai, 2) {
                relax(src, i, aai, 3);
            }
        }
    });
    for y in (0..chf.height).rev() {
        for x in (0..chf.width).rev() {
            let c = chf.cells[(x + y * chf.width) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if let Some((ax, ay, ai)) = nei(chf, x, y, i, 2) {
                    relax(src, i, ai, 2);
                    if let Some((_, _, aai)) = nei(chf, ax, ay, ai, 1) {
                        relax(src, i, aai, 3);
                    }
                }
                if let Some((ax, ay, ai)) = nei(chf, x, y, i, 1) {
                    relax(src, i, ai, 2);
                    if let Some((_, _, aai)) = nei(chf, ax, ay, ai, 0) {
                        relax(src, i, aai, 3);
                    }
                }
            }
        }
    }
    src.iter().copied().max().unwrap_or(0)
}

fn box_blur(chf: &CompactHeightfield, thr: i32, src: &[u16], dst: &mut [u16]) {
    let thr = thr * 2;
    for_spans!(chf, x, y, c, i, {
        let cd = src[i];
        if i32::from(cd) <= thr {
            dst[i] = cd;
            continue;
        }
        let mut d = i32::from(cd);
        for dir in 0..4 {
            if let Some((ax, ay, ai)) = nei(chf, x, y, i, dir) {
                d += i32::from(src[ai]);
                let dir2 = (dir + 1) & 3;
                if let Some((_, _, ai2)) = nei(chf, ax, ay, ai, dir2) {
                    d += i32::from(src[ai2]);
                } else {
                    d += i32::from(cd);
                }
            } else {
                d += i32::from(cd) * 2;
            }
        }
        dst[i] = ((d + 5) / 9) as u16;
    });
}

/// Computes the distance from each span to the nearest boundary, blurred once
/// (`rcBuildDistanceField`); fills `chf.dist` and `chf.max_distance`.
pub fn build_distance_field(ws: &mut Workspace, chf: &mut CompactHeightfield) -> Result<(), Error> {
    ws.begin();
    validate_chf(ws, chf)?;
    let n = chf.spans.len();
    // Four grid passes also visit columns with no spans.
    ws.work(n as u64 * 4 + (chf.cells.len() as u64 + chf.height as u64) * 4)?;
    chf.dist = Vec::new();
    let mut src = ws.vec(n, 0u16)?;
    let mut dst = ws.vec(n, 0u16)?;
    chf.max_distance = calculate_distance_field(chf, &mut src);
    box_blur(chf, 1, &src, &mut dst);
    chf.dist = dst;
    Ok(())
}

fn paint_rect_region(
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    reg: u16,
    chf: &CompactHeightfield,
    src_reg: &mut [u16],
) {
    for y in miny..maxy {
        for x in minx..maxx {
            let c = chf.cells[(x + y * chf.width) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] != NULL_AREA {
                    src_reg[i] = reg;
                }
            }
        }
    }
}

fn paint_border(chf: &CompactHeightfield, border: i32, id: &mut u16, src_reg: &mut [u16]) {
    if border > 0 {
        let w = chf.width;
        let h = chf.height;
        let bw = w.min(border);
        let bh = h.min(border);
        paint_rect_region(0, bw, 0, h, *id | BORDER_REG, chf, src_reg);
        *id = id.wrapping_add(1);
        paint_rect_region(w - bw, w, 0, h, *id | BORDER_REG, chf, src_reg);
        *id = id.wrapping_add(1);
        paint_rect_region(0, w, 0, bh, *id | BORDER_REG, chf, src_reg);
        *id = id.wrapping_add(1);
        paint_rect_region(0, w, h - bh, h, *id | BORDER_REG, chf, src_reg);
        *id = id.wrapping_add(1);
    }
}

#[allow(clippy::too_many_arguments)]
fn flood_region(
    ws: &mut Workspace,
    x: i32,
    y: i32,
    i: usize,
    level: u16,
    r: u16,
    chf: &CompactHeightfield,
    src_reg: &mut [u16],
    src_dist: &mut [u16],
    stack: &mut Vec<Entry>,
) -> Result<bool, Error> {
    let area = chf.areas[i];
    stack.clear();
    ws.push(
        stack,
        Entry {
            x,
            y,
            index: i as i32,
        },
    )?;
    src_reg[i] = r;
    src_dist[i] = 0;
    let lev = level.saturating_sub(2);
    let mut count = 0;
    while let Some(back) = stack.pop() {
        let (cx, cy, ci) = (back.x, back.y, back.index as usize);
        let mut ar = 0u16;
        for dir in 0..4 {
            let Some((ax, ay, ai)) = nei(chf, cx, cy, ci, dir) else {
                continue;
            };
            if chf.areas[ai] != area {
                continue;
            }
            let nr = src_reg[ai];
            if nr & BORDER_REG != 0 {
                continue;
            }
            if nr != 0 && nr != r {
                ar = nr;
                break;
            }
            let dir2 = (dir + 1) & 3;
            if let Some((_, _, ai2)) = nei(chf, ax, ay, ai, dir2) {
                if chf.areas[ai2] != area {
                    continue;
                }
                let nr2 = src_reg[ai2];
                if nr2 != 0 && nr2 != r {
                    ar = nr2;
                    break;
                }
            }
        }
        if ar != 0 {
            src_reg[ci] = 0;
            continue;
        }
        count += 1;
        for dir in 0..4 {
            let Some((ax, ay, ai)) = nei(chf, cx, cy, ci, dir) else {
                continue;
            };
            if chf.areas[ai] != area {
                continue;
            }
            if chf.dist[ai] >= lev && src_reg[ai] == 0 {
                src_reg[ai] = r;
                src_dist[ai] = 0;
                ws.push(
                    stack,
                    Entry {
                        x: ax,
                        y: ay,
                        index: ai as i32,
                    },
                )?;
            }
        }
    }
    ws.work(count)?;
    Ok(count > 0)
}

#[allow(clippy::too_many_arguments)]
fn expand_regions(
    ws: &mut Workspace,
    max_iter: i32,
    level: u16,
    chf: &CompactHeightfield,
    src_reg: &mut [u16],
    src_dist: &mut [u16],
    stack: &mut Vec<Entry>,
    fill_stack: bool,
    dirty: &mut Vec<(usize, u16, u16)>,
) -> Result<(), Error> {
    if fill_stack {
        stack.clear();
        for y in 0..chf.height {
            ws.work(1)?;
            for x in 0..chf.width {
                let c = chf.cells[(x + y * chf.width) as usize];
                for i in c.index as usize..c.index as usize + c.count as usize {
                    if chf.dist[i] >= level && src_reg[i] == 0 && chf.areas[i] != NULL_AREA {
                        ws.push(
                            stack,
                            Entry {
                                x,
                                y,
                                index: i as i32,
                            },
                        )?;
                    }
                }
            }
        }
    } else {
        for e in stack.iter_mut() {
            if e.index >= 0 && src_reg[e.index as usize] != 0 {
                e.index = -1;
            }
        }
    }
    let mut iter = 0;
    while !stack.is_empty() {
        let mut failed = 0usize;
        dirty.clear();
        ws.work(stack.len() as u64)?;
        for j in 0..stack.len() {
            let Entry { x, y, index } = stack[j];
            if index < 0 {
                failed += 1;
                continue;
            }
            let i = index as usize;
            let mut r = src_reg[i];
            let mut d2 = 0xffffu16;
            let area = chf.areas[i];
            for dir in 0..4 {
                let Some((_, _, ai)) = nei(chf, x, y, i, dir) else {
                    continue;
                };
                if chf.areas[ai] != area {
                    continue;
                }
                if src_reg[ai] > 0
                    && (src_reg[ai] & BORDER_REG) == 0
                    && i32::from(src_dist[ai]) + 2 < i32::from(d2)
                {
                    r = src_reg[ai];
                    d2 = src_dist[ai].wrapping_add(2);
                }
            }
            if r != 0 {
                stack[j].index = -1;
                ws.push(dirty, (i, r, d2))?;
            } else {
                failed += 1;
            }
        }
        for &(idx, r, d) in dirty.iter() {
            src_reg[idx] = r;
            src_dist[idx] = d;
        }
        if failed == stack.len() {
            break;
        }
        if level > 0 {
            iter += 1;
            if iter >= max_iter {
                break;
            }
        }
    }
    Ok(())
}

fn sort_cells_by_level(
    ws: &mut Workspace,
    start_level: u16,
    chf: &CompactHeightfield,
    src_reg: &[u16],
    stacks: &mut [Vec<Entry>],
    log_levels: u32,
) -> Result<(), Error> {
    let start = i32::from(start_level >> log_levels);
    for s in stacks.iter_mut() {
        s.clear();
    }
    for y in 0..chf.height {
        ws.work(1)?;
        for x in 0..chf.width {
            let c = chf.cells[(x + y * chf.width) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA || src_reg[i] != 0 {
                    continue;
                }
                let level = i32::from(chf.dist[i] >> log_levels);
                let mut sid = start - level;
                if sid >= stacks.len() as i32 {
                    continue;
                }
                if sid < 0 {
                    sid = 0;
                }
                ws.push(
                    &mut stacks[sid as usize],
                    Entry {
                        x,
                        y,
                        index: i as i32,
                    },
                )?;
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct Region {
    span_count: i32,
    id: u16,
    area_type: u8,
    remap: bool,
    visited: bool,
    overlap: bool,
    connects_to_border: bool,
    ymin: u16,
    ymax: u16,
    connections: Vec<i32>,
    floors: Vec<i32>,
}

impl Region {
    fn new(id: u16) -> Region {
        Region {
            span_count: 0,
            id,
            area_type: 0,
            remap: false,
            visited: false,
            overlap: false,
            connects_to_border: false,
            ymin: 0xffff,
            ymax: 0,
            connections: Vec::new(),
            floors: Vec::new(),
        }
    }
}

fn remove_adjacent_neighbours(conns: &mut Vec<i32>) {
    let mut i = 0;
    while i < conns.len() && conns.len() > 1 {
        let ni = (i + 1) % conns.len();
        if conns[i] == conns[ni] {
            conns.remove(i);
        } else {
            i += 1;
        }
    }
}

fn replace_neighbour(reg: &mut Region, old: u16, new: u16) {
    let mut changed = false;
    for c in reg.connections.iter_mut() {
        if *c == i32::from(old) {
            *c = i32::from(new);
            changed = true;
        }
    }
    for f in reg.floors.iter_mut() {
        if *f == i32::from(old) {
            *f = i32::from(new);
        }
    }
    if changed {
        remove_adjacent_neighbours(&mut reg.connections);
    }
}

fn can_merge_with_region(a: &Region, b: &Region) -> bool {
    if a.area_type != b.area_type {
        return false;
    }
    let n = a
        .connections
        .iter()
        .filter(|&&c| c == i32::from(b.id))
        .count();
    if n > 1 {
        return false;
    }
    !a.floors.contains(&i32::from(b.id))
}

fn add_unique_floor_region(ws: &mut Workspace, reg: &mut Region, n: i32) -> Result<(), Error> {
    if !reg.floors.contains(&n) {
        ws.push(&mut reg.floors, n)?;
    }
    Ok(())
}

fn add_unique_connection(ws: &mut Workspace, reg: &mut Region, n: i32) -> Result<(), Error> {
    if !reg.connections.contains(&n) {
        ws.push(&mut reg.connections, n)?;
    }
    Ok(())
}

/// `mergeRegions(regions[a], regions[b])`; `a != b`.
fn merge_regions(
    ws: &mut Workspace,
    regions: &mut [Region],
    a: usize,
    b: usize,
) -> Result<bool, Error> {
    let aid = i32::from(regions[a].id);
    let bid = i32::from(regions[b].id);
    let acon = core::mem::take(&mut regions[a].connections);
    let Some(insa) = acon.iter().position(|&c| c == bid) else {
        regions[a].connections = acon;
        return Ok(false);
    };
    let Some(insb) = regions[b].connections.iter().position(|&c| c == aid) else {
        regions[a].connections = acon;
        return Ok(false);
    };
    let bcon = core::mem::take(&mut regions[b].connections);
    let mut out = ws.vec_with_capacity(
        acon.len()
            .checked_add(bcon.len())
            .ok_or(Error::SizeOverflow)?,
    )?;
    let ni = acon.len();
    for i in 0..ni.saturating_sub(1) {
        out.push(acon[(insa + 1 + i) % ni]);
    }
    let ni = bcon.len();
    for i in 0..ni.saturating_sub(1) {
        out.push(bcon[(insb + 1 + i) % ni]);
    }
    remove_adjacent_neighbours(&mut out);
    regions[a].connections = out;
    let bfloors = core::mem::take(&mut regions[b].floors);
    for &f in &bfloors {
        add_unique_floor_region(ws, &mut regions[a], f)?;
    }
    regions[b].floors = bfloors;
    regions[a].span_count += regions[b].span_count;
    regions[b].span_count = 0;
    Ok(true)
}

fn is_solid_edge(
    chf: &CompactHeightfield,
    src_reg: &[u16],
    x: i32,
    y: i32,
    i: usize,
    dir: i32,
) -> bool {
    let r = match nei(chf, x, y, i, dir) {
        Some((_, _, ai)) => src_reg[ai],
        None => 0,
    };
    r != src_reg[i]
}

#[allow(clippy::too_many_arguments)]
fn walk_contour(
    ws: &mut Workspace,
    mut x: i32,
    mut y: i32,
    mut i: usize,
    mut dir: i32,
    chf: &CompactHeightfield,
    src_reg: &[u16],
    cont: &mut Vec<i32>,
) -> Result<(), Error> {
    let start_dir = dir;
    let start_i = i;
    let mut cur_reg = match nei(chf, x, y, i, dir) {
        Some((_, _, ai)) => src_reg[ai],
        None => 0,
    };
    ws.push(cont, i32::from(cur_reg))?;
    let mut iter = 0;
    loop {
        iter += 1;
        if iter >= 40000 {
            break;
        }
        if is_solid_edge(chf, src_reg, x, y, i, dir) {
            let r = match nei(chf, x, y, i, dir) {
                Some((_, _, ai)) => src_reg[ai],
                None => 0,
            };
            if r != cur_reg {
                cur_reg = r;
                ws.push(cont, i32::from(cur_reg))?;
            }
            dir = (dir + 1) & 3;
        } else {
            let Some((nx, ny, ni)) = nei(chf, x, y, i, dir) else {
                return Ok(());
            };
            x = nx;
            y = ny;
            i = ni;
            dir = (dir + 3) & 3;
        }
        if start_i == i && start_dir == dir {
            break;
        }
    }
    if cont.len() > 1 {
        let mut j = 0;
        while j < cont.len() {
            let nj = (j + 1) % cont.len();
            if cont[j] == cont[nj] {
                cont.remove(j);
            } else {
                j += 1;
            }
        }
    }
    Ok(())
}

fn merge_and_filter_regions(
    ws: &mut Workspace,
    min_region_area: i32,
    merge_region_size: i32,
    max_region_id: &mut u16,
    chf: &CompactHeightfield,
    src_reg: &mut [u16],
) -> Result<(), Error> {
    let nreg = usize::from(*max_region_id) + 1;
    let mut regions: Vec<Region> = ws.vec_with_capacity(nreg)?;
    for i in 0..nreg {
        regions.push(Region::new(i as u16));
    }
    ws.work(chf.height as u64)?;
    for_spans!(chf, x, y, c, i, {
        let r = src_reg[i];
        if r == 0 || usize::from(r) >= nreg {
            continue;
        }
        let reg = &mut regions[usize::from(r)];
        reg.span_count += 1;
        for j in c.index as usize..c.index as usize + c.count as usize {
            if i == j {
                continue;
            }
            let floor = src_reg[j];
            if floor == 0 || usize::from(floor) >= nreg {
                continue;
            }
            if floor == r {
                reg.overlap = true;
            }
            add_unique_floor_region(ws, reg, i32::from(floor))?;
        }
        if !reg.connections.is_empty() {
            continue;
        }
        reg.area_type = chf.areas[i];
        if let Some(ndir) = (0..4).find(|&d| is_solid_edge(chf, src_reg, x, y, i, d)) {
            walk_contour(ws, x, y, i, ndir, chf, src_reg, &mut reg.connections)?;
        }
    });
    ws.work(nreg as u64)?;

    // Remove small isolated regions.
    let mut stack: Vec<usize> = Vec::new();
    let mut trace: Vec<usize> = Vec::new();
    for i in 0..nreg {
        let reg = &regions[i];
        if reg.id == 0 || (reg.id & BORDER_REG) != 0 || reg.span_count == 0 || reg.visited {
            continue;
        }
        let mut connects_to_border = false;
        let mut span_count = 0;
        stack.clear();
        trace.clear();
        regions[i].visited = true;
        ws.push(&mut stack, i)?;
        while let Some(ri) = stack.pop() {
            span_count += regions[ri].span_count;
            ws.push(&mut trace, ri)?;
            for j in 0..regions[ri].connections.len() {
                let c = regions[ri].connections[j];
                if c & i32::from(BORDER_REG) != 0 {
                    connects_to_border = true;
                    continue;
                }
                let Some(nr) = regions.get_mut(c as usize) else {
                    continue;
                };
                if nr.visited || nr.id == 0 || (nr.id & BORDER_REG) != 0 {
                    continue;
                }
                ws.push(&mut stack, usize::from(nr.id))?;
                nr.visited = true;
            }
        }
        if span_count < min_region_area && !connects_to_border {
            ws.regions_removed_small = ws.regions_removed_small.saturating_add(trace.len() as u32);
            for &t in &trace {
                regions[t].span_count = 0;
                regions[t].id = 0;
            }
        }
    }

    // Merge too small regions into neighbours.
    loop {
        let mut merge_count = 0;
        for i in 0..nreg {
            let reg = &regions[i];
            if reg.id == 0 || (reg.id & BORDER_REG) != 0 || reg.overlap || reg.span_count == 0 {
                continue;
            }
            if reg.span_count > merge_region_size && reg.connections.contains(&0) {
                continue;
            }
            let mut smallest = 0xfffffff;
            let mut merge_id = reg.id;
            for &c in &reg.connections {
                if c & i32::from(BORDER_REG) != 0 {
                    continue;
                }
                let Some(m) = regions.get(c as usize) else {
                    continue;
                };
                if m.id == 0 || (m.id & BORDER_REG) != 0 || m.overlap {
                    continue;
                }
                if m.span_count < smallest
                    && can_merge_with_region(reg, m)
                    && can_merge_with_region(m, reg)
                {
                    smallest = m.span_count;
                    merge_id = m.id;
                }
            }
            if merge_id != reg.id {
                let old = reg.id;
                let target = usize::from(merge_id);
                if target != i && target < nreg && merge_regions(ws, &mut regions, target, i)? {
                    for r in regions.iter_mut() {
                        if r.id == 0 || (r.id & BORDER_REG) != 0 {
                            continue;
                        }
                        if r.id == old {
                            r.id = merge_id;
                        }
                        replace_neighbour(r, old, merge_id);
                    }
                    merge_count += 1;
                }
            }
        }
        ws.work(nreg as u64)?;
        if merge_count == 0 {
            break;
        }
    }

    compress_region_ids(&mut regions, max_region_id, src_reg);
    Ok(())
}

fn compress_region_ids(regions: &mut [Region], max_region_id: &mut u16, src_reg: &mut [u16]) {
    let nreg = regions.len();
    for r in regions.iter_mut() {
        r.remap = r.id != 0 && (r.id & BORDER_REG) == 0;
    }
    let mut next_id = 0u16;
    for i in 0..nreg {
        if !regions[i].remap {
            continue;
        }
        let old = regions[i].id;
        next_id = next_id.wrapping_add(1);
        for r in regions[i..].iter_mut() {
            if r.id == old {
                r.id = next_id;
                r.remap = false;
            }
        }
    }
    *max_region_id = next_id;
    for r in src_reg.iter_mut() {
        if (*r & BORDER_REG) == 0 {
            *r = regions.get(usize::from(*r)).map_or(0, |g| g.id);
        }
    }
}

fn check_partition(ws: &mut Workspace, chf: &CompactHeightfield, border: i32) -> Result<(), Error> {
    validate_chf(ws, chf)?;
    if border < 0 {
        return Err(Error::InvalidParameter("border size"));
    }
    Ok(())
}

/// Watershed partition (`rcBuildRegions`). Needs [`build_distance_field`] first.
pub fn build_regions(
    ws: &mut Workspace,
    chf: &mut CompactHeightfield,
    border_size: i32,
    min_region_area: i32,
    merge_region_area: i32,
) -> Result<(), Error> {
    ws.begin();
    check_partition(ws, chf, border_size)?;
    let n = chf.spans.len();
    if chf.dist.len() != n {
        return Err(Error::InvalidInput(
            "distance field missing; call build_distance_field first",
        ));
    }
    let mut src_reg = ws.vec(n, 0u16)?;
    let mut src_dist = ws.vec(n, 0u16)?;
    const LOG_NB_STACKS: u32 = 3;
    const NB_STACKS: usize = 1 << LOG_NB_STACKS;
    let mut lvl_stacks: [Vec<Entry>; NB_STACKS] = Default::default();
    let mut stack: Vec<Entry> = Vec::new();
    let mut dirty = Vec::new();
    let mut region_id: u16 = 1;
    let mut level: u16 = (chf.max_distance.wrapping_add(1)) & !1;
    const EXPAND_ITERS: i32 = 8;
    if border_size > 0 {
        // Bound the four border rectangles, including empty rows.
        ws.work((chf.cells.len() as u64 + chf.height as u64) * 4)?;
    }
    paint_border(chf, border_size, &mut region_id, &mut src_reg);
    chf.border_size = border_size;
    let chf_ref: &CompactHeightfield = chf;
    let mut sid: i32 = -1;
    while level > 0 {
        level = level.saturating_sub(2);
        sid = (sid + 1) & (NB_STACKS as i32 - 1);
        let s = sid as usize;
        if sid == 0 {
            sort_cells_by_level(ws, level, chf_ref, &src_reg, &mut lvl_stacks, 1)?;
        } else {
            let (lo, hi) = lvl_stacks.split_at_mut(s);
            for e in &lo[s - 1] {
                if e.index < 0 || src_reg[e.index as usize] != 0 {
                    continue;
                }
                ws.push(&mut hi[0], *e)?;
            }
        }
        expand_regions(
            ws,
            EXPAND_ITERS,
            level,
            chf_ref,
            &mut src_reg,
            &mut src_dist,
            &mut lvl_stacks[s],
            false,
            &mut dirty,
        )?;
        for j in 0..lvl_stacks[s].len() {
            let cur = lvl_stacks[s][j];
            if cur.index >= 0
                && src_reg[cur.index as usize] == 0
                && flood_region(
                    ws,
                    cur.x,
                    cur.y,
                    cur.index as usize,
                    level,
                    region_id,
                    chf_ref,
                    &mut src_reg,
                    &mut src_dist,
                    &mut stack,
                )?
            {
                if region_id == 0xffff {
                    return Err(Error::FormatLimit("more than 65,535 regions"));
                }
                region_id += 1;
            }
        }
    }
    expand_regions(
        ws,
        EXPAND_ITERS * 8,
        0,
        chf_ref,
        &mut src_reg,
        &mut src_dist,
        &mut stack,
        true,
        &mut dirty,
    )?;
    let mut max_regions = region_id;
    merge_and_filter_regions(
        ws,
        min_region_area,
        merge_region_area,
        &mut max_regions,
        chf_ref,
        &mut src_reg,
    )?;
    chf.max_regions = max_regions;
    for (s, r) in chf.spans.iter_mut().zip(&src_reg) {
        s.reg = *r;
    }
    Ok(())
}

#[derive(Clone, Copy, Default)]
struct Sweep {
    id: u16,
    ns: u16,
    nei: u16,
}

const NULL_NEI: u16 = 0xffff;

/// The row sweep shared by the monotone and layer partitions; returns the next
/// free region id.
fn sweep_regions(
    ws: &mut Workspace,
    chf: &CompactHeightfield,
    border: i32,
    src_reg: &mut [u16],
) -> Result<u16, Error> {
    let w = chf.width;
    let h = chf.height;
    let mut id: u16 = 1;
    if border > 0 {
        // Bound the four border rectangles, including empty rows.
        ws.work((chf.cells.len() as u64 + chf.height as u64) * 4)?;
    }
    paint_border(chf, border, &mut id, src_reg);
    let mut sweeps: Vec<Sweep> = ws.vec(w.max(h).max(1) as usize, Sweep::default())?;
    let mut prev: Vec<i32> = ws.vec(256, 0)?;
    for y in border..h - border {
        ws.work(1)?;
        let need = usize::from(id) + 1;
        if prev.len() < need {
            let grow = need - prev.len();
            ws.reserve(&mut prev, grow)?;
        }
        prev.resize(need, 0);
        prev[..usize::from(id)].fill(0);
        let mut rid: u16 = 1;
        ws.work(w as u64)?;
        for x in border..w - border {
            let c = chf.cells[(x + y * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if chf.areas[i] == NULL_AREA {
                    continue;
                }
                let mut previd = 0u16;
                if let Some((_, _, ai)) = nei(chf, x, y, i, 0)
                    && (src_reg[ai] & BORDER_REG) == 0
                    && chf.areas[i] == chf.areas[ai]
                {
                    previd = src_reg[ai];
                }
                if previd == 0 {
                    previd = rid;
                    rid = rid.wrapping_add(1);
                    if usize::from(previd) >= sweeps.len() {
                        let grow = usize::from(previd) + 1 - sweeps.len();
                        ws.reserve(&mut sweeps, grow)?;
                        sweeps.resize(usize::from(previd) + 1, Sweep::default());
                    }
                    sweeps[usize::from(previd)].ns = 0;
                    sweeps[usize::from(previd)].nei = 0;
                }
                if let Some((_, _, ai)) = nei(chf, x, y, i, 3)
                    && src_reg[ai] != 0
                    && (src_reg[ai] & BORDER_REG) == 0
                    && chf.areas[i] == chf.areas[ai]
                {
                    let nr = src_reg[ai];
                    let sw = &mut sweeps[usize::from(previd)];
                    if sw.nei == 0 || sw.nei == nr {
                        sw.nei = nr;
                        sw.ns = sw.ns.wrapping_add(1);
                        if let Some(p) = prev.get_mut(usize::from(nr)) {
                            *p += 1;
                        }
                    } else {
                        sw.nei = NULL_NEI;
                    }
                }
                src_reg[i] = previd;
            }
        }
        for i in 1..usize::from(rid) {
            let sw = sweeps[i];
            if sw.nei != NULL_NEI
                && sw.nei != 0
                && prev.get(usize::from(sw.nei)).copied() == Some(i32::from(sw.ns))
            {
                sweeps[i].id = sw.nei;
            } else {
                sweeps[i].id = id;
                id = id.wrapping_add(1);
            }
        }
        for x in border..w - border {
            let c = chf.cells[(x + y * w) as usize];
            for i in c.index as usize..c.index as usize + c.count as usize {
                if src_reg[i] > 0 && src_reg[i] < rid {
                    src_reg[i] = sweeps[usize::from(src_reg[i])].id;
                }
            }
        }
    }
    Ok(id)
}

/// Monotone partition (`rcBuildRegionsMonotone`).
pub fn build_regions_monotone(
    ws: &mut Workspace,
    chf: &mut CompactHeightfield,
    border_size: i32,
    min_region_area: i32,
    merge_region_area: i32,
) -> Result<(), Error> {
    ws.begin();
    check_partition(ws, chf, border_size)?;
    let mut src_reg = ws.vec(chf.spans.len(), 0u16)?;
    let mut max_regions = sweep_regions(ws, chf, border_size, &mut src_reg)?;
    chf.border_size = border_size;
    merge_and_filter_regions(
        ws,
        min_region_area,
        merge_region_area,
        &mut max_regions,
        chf,
        &mut src_reg,
    )?;
    chf.max_regions = max_regions;
    for (s, r) in chf.spans.iter_mut().zip(&src_reg) {
        s.reg = *r;
    }
    Ok(())
}

fn merge_and_filter_layer_regions(
    ws: &mut Workspace,
    min_region_area: i32,
    max_region_id: &mut u16,
    chf: &CompactHeightfield,
    src_reg: &mut [u16],
) -> Result<(), Error> {
    let nreg = usize::from(*max_region_id) + 1;
    let mut regions: Vec<Region> = ws.vec_with_capacity(nreg)?;
    for i in 0..nreg {
        regions.push(Region::new(i as u16));
    }
    let mut lregs: Vec<i32> = Vec::new();
    for y in 0..chf.height {
        ws.work(1)?;
        for x in 0..chf.width {
            let c = chf.cells[(x + y * chf.width) as usize];
            lregs.clear();
            for i in c.index as usize..c.index as usize + c.count as usize {
                let s = chf.spans[i];
                let area = chf.areas[i];
                let ri = src_reg[i];
                if ri == 0 || usize::from(ri) >= nreg {
                    continue;
                }
                let reg = &mut regions[usize::from(ri)];
                reg.span_count += 1;
                reg.area_type = area;
                reg.ymin = reg.ymin.min(s.y);
                reg.ymax = reg.ymax.max(s.y);
                ws.push(&mut lregs, i32::from(ri))?;
                for dir in 0..4 {
                    if let Some((_, _, ai)) = nei(chf, x, y, i, dir) {
                        let rai = src_reg[ai];
                        if rai > 0 && usize::from(rai) < nreg && rai != ri {
                            add_unique_connection(ws, reg, i32::from(rai))?;
                        }
                        if rai & BORDER_REG != 0 {
                            reg.connects_to_border = true;
                        }
                    }
                }
            }
            for i in 0..lregs.len().saturating_sub(1) {
                for j in i + 1..lregs.len() {
                    if lregs[i] != lregs[j] {
                        add_unique_floor_region(ws, &mut regions[lregs[i] as usize], lregs[j])?;
                        add_unique_floor_region(ws, &mut regions[lregs[j] as usize], lregs[i])?;
                    }
                }
            }
        }
    }
    ws.work(nreg as u64)?;

    let mut layer_id: u16 = 1;
    for r in regions.iter_mut() {
        r.id = 0;
    }
    // Each region is enqueued once. An indexed Vec keeps FIFO order without
    // VecDeque's power-of-two capacity rounding escaping the byte charge.
    let mut stack = ws.vec_with_capacity(nreg)?;
    for i in 1..nreg {
        if regions[i].id != 0 {
            continue;
        }
        regions[i].id = layer_id;
        stack.clear();
        stack.push(i);
        let mut head = 0;
        while head < stack.len() {
            let ri = stack[head];
            head += 1;
            let ncons = regions[ri].connections.len();
            for j in 0..ncons {
                let n = regions[ri].connections[j] as usize;
                if regions[n].id != 0 || regions[ri].area_type != regions[n].area_type {
                    continue;
                }
                if regions[i].floors.contains(&(n as i32)) {
                    continue;
                }
                stack.push(n);
                regions[n].id = layer_id;
                let floors = core::mem::take(&mut regions[n].floors);
                for &f in &floors {
                    add_unique_floor_region(ws, &mut regions[i], f)?;
                }
                regions[n].floors = floors;
                let (nymin, nymax, nspans, nborder) = (
                    regions[n].ymin,
                    regions[n].ymax,
                    regions[n].span_count,
                    regions[n].connects_to_border,
                );
                let root = &mut regions[i];
                root.ymin = root.ymin.min(nymin);
                root.ymax = root.ymax.max(nymax);
                root.span_count += nspans;
                regions[n].span_count = 0;
                let root = &mut regions[i];
                root.connects_to_border = root.connects_to_border || nborder;
            }
        }
        layer_id = layer_id.wrapping_add(1);
    }
    for i in 0..nreg {
        if regions[i].span_count > 0
            && regions[i].span_count < min_region_area
            && !regions[i].connects_to_border
        {
            let reg = regions[i].id;
            for r in regions.iter_mut() {
                if r.id == reg {
                    if reg != 0 {
                        ws.regions_removed_small = ws.regions_removed_small.saturating_add(1);
                    }
                    r.id = 0;
                }
            }
        }
    }
    compress_region_ids(&mut regions, max_region_id, src_reg);
    Ok(())
}

/// Layer partition (`rcBuildLayerRegions`): monotone regions merged into
/// non-overlapping layers.
pub fn build_layer_regions(
    ws: &mut Workspace,
    chf: &mut CompactHeightfield,
    border_size: i32,
    min_region_area: i32,
) -> Result<(), Error> {
    ws.begin();
    check_partition(ws, chf, border_size)?;
    let mut src_reg = ws.vec(chf.spans.len(), 0u16)?;
    let mut max_regions = sweep_regions(ws, chf, border_size, &mut src_reg)?;
    chf.border_size = border_size;
    merge_and_filter_layer_regions(ws, min_region_area, &mut max_regions, chf, &mut src_reg)?;
    chf.max_regions = max_regions;
    for (s, r) in chf.spans.iter_mut().zip(&src_reg) {
        s.reg = *r;
    }
    Ok(())
}

#[cfg(test)]
mod allocation_tests {
    use super::*;
    use crate::Limits;

    #[test]
    fn region_merge_and_floor_growth_obey_byte_limits() {
        let mut regions = [Region::new(1), Region::new(2)];
        regions[0].connections = alloc::vec![2, 0];
        regions[1].connections = alloc::vec![1, 0];
        let mut ws = Workspace::with_limits(Limits {
            max_bytes: 0,
            ..Limits::default()
        });
        assert_eq!(
            merge_regions(&mut ws, &mut regions, 0, 1),
            Err(Error::LimitExceeded)
        );
        assert_eq!(ws.usage().bytes, 16);
        ws.begin();
        assert_eq!(
            add_unique_floor_region(&mut ws, &mut regions[0], 3),
            Err(Error::LimitExceeded)
        );
        assert!(regions[0].floors.is_empty());
        ws.set_limits(Limits::default());
        ws.begin();
        regions[0].connections = alloc::vec![2, 0];
        regions[1].connections = alloc::vec![1, 0];
        assert_eq!(merge_regions(&mut ws, &mut regions, 0, 1), Ok(true));
        assert_eq!(regions[0].connections, [0]);
    }
}
