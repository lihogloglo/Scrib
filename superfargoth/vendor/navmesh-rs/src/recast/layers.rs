//! Heightfield layers (upstream `RecastLayers.cpp`).

use alloc::vec::Vec;

use super::area::check_chf;
use super::{CompactHeightfield, NOT_CONNECTED, NULL_AREA, dir_offset_x, dir_offset_y};
use crate::{Error, Workspace};

/// One layer of a heightfield layer set (`rcHeightfieldLayer`).
#[derive(Clone, Debug, Default)]
pub struct HeightfieldLayer {
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
    /// Minimum x of the used area.
    pub minx: i32,
    /// Maximum x of the used area.
    pub maxx: i32,
    /// Minimum y of the used area.
    pub miny: i32,
    /// Maximum y of the used area.
    pub maxy: i32,
    /// Lowest height.
    pub hmin: i32,
    /// Highest height.
    pub hmax: i32,
    /// Height per cell.
    pub heights: Vec<u8>,
    /// Area per cell.
    pub areas: Vec<u8>,
    /// Packed neighbour connectivity per cell.
    pub cons: Vec<u8>,
}

/// Heightfield layers (`rcHeightfieldLayerSet`).
#[derive(Clone, Debug, Default)]
pub struct HeightfieldLayerSet {
    /// Layers.
    pub layers: Vec<HeightfieldLayer>,
}

const MAX_LAYERS: usize = 63;
const MAX_NEIS: usize = 16;

#[derive(Clone, Copy)]
struct LayerRegion {
    layers: [u8; MAX_LAYERS],
    neis: [u8; MAX_NEIS],
    ymin: u16,
    ymax: u16,
    layer_id: u8,
    nlayers: u8,
    nneis: u8,
    base: u8,
}

fn contains(a: &[u8], n: u8, v: u8) -> bool {
    a[..usize::from(n)].contains(&v)
}

fn add_unique(a: &mut [u8], n: &mut u8, max: usize, v: u8) -> bool {
    if contains(a, *n, v) {
        return true;
    }
    if usize::from(*n) >= max {
        return false;
    }
    a[usize::from(*n)] = v;
    *n += 1;
    true
}

#[inline(always)]
fn overlap_range(amin: u16, amax: u16, bmin: u16, bmax: u16) -> bool {
    !(amin > bmax || amax < bmin)
}

#[derive(Clone, Copy, Default)]
struct Sweep {
    ns: u16,
    id: u8,
    nei: u8,
}

impl HeightfieldLayerSet {
    /// Splits the walkable spans of a tile into 2D layers of at most 255 height
    /// units each (`rcBuildHeightfieldLayers`), the tile cache's input.
    /// Fails with [`Error::FormatLimit`] where upstream gives up (more than 255
    /// regions, or too many overlapping layers).
    pub fn build(
        ws: &mut Workspace,
        chf: &CompactHeightfield,
        border_size: i32,
        walkable_height: i32,
    ) -> Result<HeightfieldLayerSet, Error> {
        ws.begin();
        check_chf(ws, chf)?;
        if border_size < 0 {
            return Err(Error::InvalidParameter("border size"));
        }
        let w = chf.width;
        let h = chf.height;
        let nei = |x: i32, y: i32, i: usize, dir: i32| -> Option<(i32, i32, usize)> {
            let c = chf.spans[i].con(dir);
            if c == NOT_CONNECTED {
                return None;
            }
            let ax = x + dir_offset_x(dir);
            let ay = y + dir_offset_y(dir);
            Some((
                ax,
                ay,
                chf.cells[(ax + ay * w) as usize].index as usize + c as usize,
            ))
        };
        ws.work(chf.spans.len() as u64 * 2)?;
        let mut src_reg = ws.vec(chf.spans.len(), 0xffu8)?;
        let mut sweeps = ws.vec(256, Sweep::default())?;
        let mut prev_count = [0i32; 256];
        let mut reg_id: u8 = 0;
        for y in border_size..h - border_size {
            ws.work(1)?;
            prev_count[..usize::from(reg_id)].fill(0);
            let mut sweep_id: u8 = 0;
            for x in border_size..w - border_size {
                let c = chf.cells[(x + y * w) as usize];
                for i in c.index as usize..c.index as usize + c.count as usize {
                    if chf.areas[i] == NULL_AREA {
                        continue;
                    }
                    let mut sid = 0xffu8;
                    if let Some((_, _, ai)) = nei(x, y, i, 0)
                        && chf.areas[ai] != NULL_AREA
                        && src_reg[ai] != 0xff
                    {
                        sid = src_reg[ai];
                    }
                    if sid == 0xff {
                        sid = sweep_id;
                        sweep_id = sweep_id.wrapping_add(1);
                        sweeps[usize::from(sid)].nei = 0xff;
                        sweeps[usize::from(sid)].ns = 0;
                    }
                    if let Some((_, _, ai)) = nei(x, y, i, 3) {
                        let nr = src_reg[ai];
                        if nr != 0xff {
                            let sw = &mut sweeps[usize::from(sid)];
                            if sw.ns == 0 {
                                sw.nei = nr;
                            }
                            if sw.nei == nr {
                                sw.ns = sw.ns.wrapping_add(1);
                                prev_count[usize::from(nr)] += 1;
                            } else {
                                sw.nei = 0xff;
                            }
                        }
                    }
                    src_reg[i] = sid;
                }
            }
            for i in 0..usize::from(sweep_id) {
                let sw = sweeps[i];
                if sw.nei != 0xff && prev_count[usize::from(sw.nei)] == i32::from(sw.ns) {
                    sweeps[i].id = sw.nei;
                } else {
                    if reg_id == 255 {
                        return Err(Error::FormatLimit("more than 255 layer regions in a tile"));
                    }
                    sweeps[i].id = reg_id;
                    reg_id += 1;
                }
            }
            for x in border_size..w - border_size {
                let c = chf.cells[(x + y * w) as usize];
                for i in c.index as usize..c.index as usize + c.count as usize {
                    if src_reg[i] != 0xff {
                        src_reg[i] = sweeps[usize::from(src_reg[i])].id;
                    }
                }
            }
        }

        let nregs = usize::from(reg_id);
        let mut regs = ws.vec(
            nregs,
            LayerRegion {
                layers: [0; MAX_LAYERS],
                neis: [0; MAX_NEIS],
                ymin: 0xffff,
                ymax: 0,
                layer_id: 0xff,
                nlayers: 0,
                nneis: 0,
                base: 0,
            },
        )?;
        let fail = || Error::FormatLimit("too many overlapping layers");
        for y in 0..h {
            ws.work(1)?;
            for x in 0..w {
                let c = chf.cells[(x + y * w) as usize];
                let mut lregs = [0u8; MAX_LAYERS];
                let mut nlregs = 0usize;
                for i in c.index as usize..c.index as usize + c.count as usize {
                    let s = chf.spans[i];
                    let ri = src_reg[i];
                    if ri == 0xff {
                        continue;
                    }
                    let r = &mut regs[usize::from(ri)];
                    r.ymin = r.ymin.min(s.y);
                    r.ymax = r.ymax.max(s.y);
                    if nlregs < MAX_LAYERS {
                        lregs[nlregs] = ri;
                        nlregs += 1;
                    }
                    for dir in 0..4 {
                        if let Some((_, _, ai)) = nei(x, y, i, dir) {
                            let rai = src_reg[ai];
                            if rai != 0xff && rai != ri {
                                let r = &mut regs[usize::from(ri)];
                                add_unique(&mut r.neis, &mut r.nneis, MAX_NEIS, rai);
                            }
                        }
                    }
                }
                for i in 0..nlregs.saturating_sub(1) {
                    for j in i + 1..nlregs {
                        if lregs[i] != lregs[j] {
                            let (a, b) = (usize::from(lregs[i]), usize::from(lregs[j]));
                            let ra = &mut regs[a];
                            if !add_unique(&mut ra.layers, &mut ra.nlayers, MAX_LAYERS, lregs[j]) {
                                return Err(fail());
                            }
                            let rb = &mut regs[b];
                            if !add_unique(&mut rb.layers, &mut rb.nlayers, MAX_LAYERS, lregs[i]) {
                                return Err(fail());
                            }
                        }
                    }
                }
            }
        }

        // Create 2D layers from regions.
        let mut layer_id: u8 = 0;
        const MAX_STACK: usize = 64;
        let mut stack = [0u8; MAX_STACK];
        for i in 0..nregs {
            if regs[i].layer_id != 0xff {
                continue;
            }
            regs[i].layer_id = layer_id;
            regs[i].base = 1;
            let mut nstack = 1usize;
            stack[0] = i as u8;
            while nstack > 0 {
                let ri = usize::from(stack[0]);
                nstack -= 1;
                stack.copy_within(1..=nstack, 0);
                let reg = regs[ri];
                for j in 0..usize::from(reg.nneis) {
                    let n = reg.neis[j];
                    let rn = regs[usize::from(n)];
                    if rn.layer_id != 0xff {
                        continue;
                    }
                    let root = regs[i];
                    if contains(&root.layers, root.nlayers, n) {
                        continue;
                    }
                    let ymin = i32::from(root.ymin.min(rn.ymin));
                    let ymax = i32::from(root.ymax.max(rn.ymax));
                    if ymax - ymin >= 255 {
                        continue;
                    }
                    if nstack < MAX_STACK {
                        stack[nstack] = n;
                        nstack += 1;
                        regs[usize::from(n)].layer_id = layer_id;
                        let root = &mut regs[i];
                        for k in 0..usize::from(rn.nlayers) {
                            if !add_unique(
                                &mut root.layers,
                                &mut root.nlayers,
                                MAX_LAYERS,
                                rn.layers[k],
                            ) {
                                return Err(fail());
                            }
                        }
                        root.ymin = root.ymin.min(rn.ymin);
                        root.ymax = root.ymax.max(rn.ymax);
                    }
                }
            }
            layer_id = layer_id.wrapping_add(1);
        }

        // Merge non-overlapping regions that are close in height.
        let merge_height = (walkable_height as u16 as i32 * 4) as u16;
        for i in 0..nregs {
            if regs[i].base == 0 {
                continue;
            }
            let new_id = regs[i].layer_id;
            loop {
                let mut old_id = 0xffu8;
                let ri = regs[i];
                for j in 0..nregs {
                    if i == j {
                        continue;
                    }
                    let rj = regs[j];
                    if rj.base == 0 {
                        continue;
                    }
                    if !overlap_range(
                        ri.ymin,
                        ri.ymax.wrapping_add(merge_height),
                        rj.ymin,
                        rj.ymax.wrapping_add(merge_height),
                    ) {
                        continue;
                    }
                    let ymin = i32::from(ri.ymin.min(rj.ymin));
                    let ymax = i32::from(ri.ymax.max(rj.ymax));
                    if ymax - ymin >= 255 {
                        continue;
                    }
                    let overlap = (0..nregs).any(|k| {
                        regs[k].layer_id == rj.layer_id && contains(&ri.layers, ri.nlayers, k as u8)
                    });
                    if overlap {
                        continue;
                    }
                    old_id = rj.layer_id;
                    break;
                }
                if old_id == 0xff {
                    break;
                }
                for j in 0..nregs {
                    if regs[j].layer_id == old_id {
                        let rj = regs[j];
                        regs[j].base = 0;
                        regs[j].layer_id = new_id;
                        let ri = &mut regs[i];
                        for k in 0..usize::from(rj.nlayers) {
                            if !add_unique(
                                &mut ri.layers,
                                &mut ri.nlayers,
                                MAX_LAYERS,
                                rj.layers[k],
                            ) {
                                return Err(fail());
                            }
                        }
                        ri.ymin = ri.ymin.min(rj.ymin);
                        ri.ymax = ri.ymax.max(rj.ymax);
                    }
                }
            }
        }

        // Compact layer ids.
        let mut remap = [0u8; 256];
        for r in &regs {
            remap[usize::from(r.layer_id)] = 1;
        }
        let mut nlayers: u8 = 0;
        for m in remap.iter_mut() {
            if *m != 0 {
                *m = nlayers;
                nlayers = nlayers.wrapping_add(1);
            } else {
                *m = 0xff;
            }
        }
        for r in regs.iter_mut() {
            r.layer_id = remap[usize::from(r.layer_id)];
        }
        let mut lset = HeightfieldLayerSet::default();
        if nlayers == 0 {
            return Ok(lset);
        }
        let lw = w - border_size * 2;
        let lh = h - border_size * 2;
        let mut bmin = chf.bmin;
        let mut bmax = chf.bmax;
        let b = border_size as f32 * chf.cs;
        bmin[0] += b;
        bmin[2] += b;
        bmax[0] -= b;
        bmax[2] -= b;
        let grid = (lw.max(0) as usize) * (lh.max(0) as usize);
        for cur in 0..nlayers {
            let mut hmin = 0i32;
            let mut hmax = 0i32;
            for r in &regs {
                if r.base != 0 && r.layer_id == cur {
                    hmin = i32::from(r.ymin);
                    hmax = i32::from(r.ymax);
                }
            }
            let mut layer = HeightfieldLayer {
                bmin,
                bmax,
                cs: chf.cs,
                ch: chf.ch,
                width: lw,
                height: lh,
                minx: lw,
                maxx: 0,
                miny: lh,
                maxy: 0,
                hmin,
                hmax,
                heights: ws.vec(grid, 0xffu8)?,
                areas: ws.vec(grid, 0u8)?,
                cons: ws.vec(grid, 0u8)?,
            };
            layer.bmin[1] = bmin[1] + hmin as f32 * chf.ch;
            layer.bmax[1] = bmin[1] + hmax as f32 * chf.ch;
            for y in 0..lh {
                ws.work(1)?;
                for x in 0..lw {
                    let cx = border_size + x;
                    let cy = border_size + y;
                    let c = chf.cells[(cx + cy * w) as usize];
                    for j in c.index as usize..c.index as usize + c.count as usize {
                        let s = chf.spans[j];
                        if src_reg[j] == 0xff {
                            continue;
                        }
                        let lid = regs[usize::from(src_reg[j])].layer_id;
                        if lid != cur {
                            continue;
                        }
                        layer.minx = layer.minx.min(x);
                        layer.maxx = layer.maxx.max(x);
                        layer.miny = layer.miny.min(y);
                        layer.maxy = layer.maxy.max(y);
                        let idx = (x + y * lw) as usize;
                        layer.heights[idx] = (i32::from(s.y) - hmin) as u8;
                        layer.areas[idx] = chf.areas[j];
                        let mut portal = 0u8;
                        let mut con = 0u8;
                        for dir in 0..4 {
                            let Some((ax, ay, ai)) = nei(cx, cy, j, dir) else {
                                continue;
                            };
                            let alid = if src_reg[ai] != 0xff {
                                regs[usize::from(src_reg[ai])].layer_id
                            } else {
                                0xff
                            };
                            if chf.areas[ai] != NULL_AREA && lid != alid {
                                portal |= 1 << dir;
                                let ay_ = i32::from(chf.spans[ai].y);
                                if ay_ > hmin {
                                    layer.heights[idx] = layer.heights[idx].max((ay_ - hmin) as u8);
                                }
                            }
                            if chf.areas[ai] != NULL_AREA && lid == alid {
                                let nx = ax - border_size;
                                let ny = ay - border_size;
                                if nx >= 0 && ny >= 0 && nx < lw && ny < lh {
                                    con |= 1 << dir;
                                }
                            }
                        }
                        layer.cons[idx] = (portal << 4) | con;
                    }
                }
            }
            if layer.minx > layer.maxx {
                layer.minx = 0;
                layer.maxx = 0;
            }
            if layer.miny > layer.maxy {
                layer.miny = 0;
                layer.maxy = 0;
            }
            ws.push(&mut lset.layers, layer)?;
        }
        Ok(lset)
    }
}
