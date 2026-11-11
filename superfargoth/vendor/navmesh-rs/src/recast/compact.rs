//! The compact heightfield: open space above walkable spans with neighbour links
//! (upstream `rcBuildCompactHeightfield`).

use alloc::vec::Vec;

use super::{Heightfield, NOT_CONNECTED, NULL_AREA, dir_offset_x, dir_offset_y};
use crate::{Error, Workspace};

/// One grid column of the compact heightfield (`rcCompactCell`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompactCell {
    /// Index of the column's first span (24 bits upstream).
    pub index: u32,
    /// Number of spans in the column.
    pub count: u8,
}

/// An open span (`rcCompactSpan`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompactSpan {
    /// Floor height, in cells.
    pub y: u16,
    /// Region id (0 before partitioning).
    pub reg: u16,
    /// Neighbour layer index per direction, 6 bits each (24 bits).
    pub con: u32,
    /// Free height above the floor, in cells (clamped to 255).
    pub h: u8,
}

impl CompactSpan {
    /// The neighbour layer index in direction `dir`, or [`NOT_CONNECTED`] (`rcGetCon`).
    #[inline(always)]
    pub fn con(&self, dir: i32) -> i32 {
        ((self.con >> (dir as u32 * 6)) & 0x3f) as i32
    }

    /// Sets the neighbour layer index in direction `dir` (`rcSetCon`).
    #[inline(always)]
    pub fn set_con(&mut self, dir: i32, i: i32) {
        let shift = dir as u32 * 6;
        self.con = (self.con & !(0x3f << shift)) | (((i as u32) & 0x3f) << shift);
    }
}

/// The compact heightfield (`rcCompactHeightfield`).
#[derive(Clone, Debug, Default)]
pub struct CompactHeightfield {
    /// Width in cells.
    pub width: i32,
    /// Height in cells.
    pub height: i32,
    /// Number of spans.
    pub span_count: i32,
    /// Walkable height used to build it, in cells.
    pub walkable_height: i32,
    /// Walkable climb used to build it, in cells.
    pub walkable_climb: i32,
    /// Border size, in cells (set by the region builders).
    pub border_size: i32,
    /// Largest value in [`dist`](Self::dist).
    pub max_distance: u16,
    /// Number of regions plus one (set by the region builders).
    pub max_regions: u16,
    /// Minimum bounds.
    pub bmin: [f32; 3],
    /// Maximum bounds.
    pub bmax: [f32; 3],
    /// Cell size.
    pub cs: f32,
    /// Cell height.
    pub ch: f32,
    /// Columns, `width * height`.
    pub cells: Vec<CompactCell>,
    /// Spans, `span_count`.
    pub spans: Vec<CompactSpan>,
    /// Distance to the border per span; empty until [`build_distance_field`](super::build_distance_field).
    pub dist: Vec<u16>,
    /// Area id per span.
    pub areas: Vec<u8>,
}

impl CompactHeightfield {
    /// Builds the compact heightfield from the walkable spans of `hf`
    /// (`rcBuildCompactHeightfield`). Fails with [`Error::FormatLimit`] where
    /// upstream's bit fields would overflow (more than 2^24 spans, or more than 255
    /// walkable spans in one column).
    pub fn build(
        ws: &mut Workspace,
        walkable_height: i32,
        walkable_climb: i32,
        hf: &Heightfield,
    ) -> Result<CompactHeightfield, Error> {
        ws.begin();
        hf.check()?;
        let xs = hf.width;
        let zs = hf.height;
        let ncols = (xs as usize) * (zs as usize);
        ws.work(ncols as u64)?;
        let span_count = hf.walkable_span_count();
        if span_count > 0xff_ffff {
            return Err(Error::FormatLimit("more than 2^24 compact spans"));
        }
        let mut chf = CompactHeightfield {
            width: xs,
            height: zs,
            span_count: span_count as i32,
            walkable_height,
            walkable_climb,
            border_size: 0,
            max_distance: 0,
            max_regions: 0,
            bmin: hf.bmin,
            bmax: hf.bmax,
            cs: hf.cs,
            ch: hf.ch,
            cells: ws.vec(ncols, CompactCell::default())?,
            spans: ws.vec(span_count, CompactSpan::default())?,
            dist: Vec::new(),
            areas: ws.vec(span_count, NULL_AREA)?,
        };
        chf.bmax[1] += walkable_height as f32 * hf.ch;
        const MAX_HEIGHT: i32 = 0xffff;
        let mut idx = 0usize;
        for col in 0..ncols {
            let mut s = hf.head(col);
            if s == u32::MAX {
                continue;
            }
            let mut count = 0u32;
            chf.cells[col].index = idx as u32;
            while s != u32::MAX {
                let span = hf.span(s);
                let next = hf.next_of(s);
                if span.area != NULL_AREA {
                    let bot = i32::from(span.smax);
                    let top = if next != u32::MAX {
                        i32::from(hf.span(next).smin)
                    } else {
                        MAX_HEIGHT
                    };
                    chf.spans[idx].y = bot.clamp(0, 0xffff) as u16;
                    chf.spans[idx].h = (top - bot).clamp(0, 0xff) as u8;
                    chf.areas[idx] = span.area;
                    idx += 1;
                    count += 1;
                }
                s = next;
            }
            if count > 255 {
                return Err(Error::FormatLimit(
                    "more than 255 walkable spans in a column",
                ));
            }
            chf.cells[col].count = count as u8;
        }

        ws.work(span_count as u64)?;
        const MAX_LAYERS: i32 = NOT_CONNECTED - 1;
        for z in 0..zs {
            for x in 0..xs {
                let c = chf.cells[(x + z * xs) as usize];
                for i in c.index as usize..(c.index as usize + c.count as usize) {
                    let mut span = chf.spans[i];
                    for dir in 0..4 {
                        span.set_con(dir, NOT_CONNECTED);
                        let nx = x + dir_offset_x(dir);
                        let nz = z + dir_offset_y(dir);
                        if nx < 0 || nz < 0 || nx >= xs || nz >= zs {
                            continue;
                        }
                        let nc = chf.cells[(nx + nz * xs) as usize];
                        for k in nc.index as usize..(nc.index as usize + nc.count as usize) {
                            let ns = &chf.spans[k];
                            let bot = i32::from(span.y.max(ns.y));
                            let top = (i32::from(span.y) + i32::from(span.h))
                                .min(i32::from(ns.y) + i32::from(ns.h));
                            if top - bot >= walkable_height
                                && (i32::from(ns.y) - i32::from(span.y)).abs() <= walkable_climb
                            {
                                let layer = (k - nc.index as usize) as i32;
                                if layer > MAX_LAYERS {
                                    continue;
                                }
                                span.set_con(dir, layer);
                                break;
                            }
                        }
                    }
                    chf.spans[i] = span;
                }
            }
        }
        Ok(chf)
    }
}
