//! The guide towns as a scatter exclusion (WG-X5): flora and rocks stay out of a guide's own towns (`<guide>/mods/*`,
//! `sf.placement_group`s streamed as `town:<id>`) as they stay out of the stamped anchors. A guide town has no stamp
//! (`worldgen-derive` drops the vanilla settlement's), so its area comes from its placements.
//!
//! **Area:** the standard exclusion mask from placed buildings (UE PCG's "Difference" against building bounds,
//! Horizon's placement exclusion), closed into a built-up area the way urban-footprint delineation does it
//! (morphological closing of the built cells, then the enclosed holes filled):
//! 1. **Footprints:** each placement's model bounds (scaled, rotated) as a ground rectangle along its yaw; a placement
//!    without bounds is a point. Rasterised on a [`TownParams::cell_m`] grid per town, conservatively (every cell the
//!    rectangle touches).
//! 2. **Closing** by [`TownParams::close_m`]: streets, yards and plazas narrower than twice it between footprints
//!    join the town (the owner's look 2026-11-24: rocks and logs in Balmora 2.0's streets), without spreading past
//!    its outer edge.
//! 3. **Holes filled:** what the closed area encloses (a walled town's inside, whatever its plaza sizes).
//! 4. **Apron** of [`TownParams::margin_m`] around the result: nothing grows against a wall.
//!
//! Exact squared Euclidean distance transforms (Felzenszwalb & Huttenlocher 2012) do steps 2 and 4, so the mask is
//! one bit per cell, built in linear time; a point tests its cell. The apron is rounded up by a cell diagonal (the
//! rasterisation's slack), so no candidate stands within `margin_m` of a footprint.
//!
//! Frames: placements in the engine frame (Y up, −Z north), the mask in the guide's (x east, y north), as
//! [`super::placements`] converts.

// Grid maths: cell counts stay far below 2^31 (bounded by `flora.town_cells_max`).
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

use glam::Vec3;
use sf_world::Placement;
use sf_worldgen::filter::Declared;
use sf_worldgen::scatter::{Site, Terrain};

use super::Bounds;

/// The town exclusion values (spec `tunables.md` "Definition").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TownParams {
    /// Clear apron around the town area, metres.
    pub margin_m: f64,
    /// Closing radius: gaps up to twice this between footprints are town, metres.
    pub close_m: f64,
    /// Mask cell edge, metres.
    pub cell_m: f64,
    /// Upper bound of one town's mask cells (the cell grows to fit).
    pub cells_max: f64,
}

/// The declared town exclusion values.
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, help| Declared { name, v, unit, min, max, source: "Guess".into(), help };
    vec![
        d("flora.town_margin_m", 3.0, "m", 0.0, 50.0, "clear apron around a guide town's area: no flora or rock grows \
           against a wall or a house"),
        d("flora.town_close_m", 20.0, "m", 0.0, 200.0, "closing radius of a guide town's area: streets, yards and \
           plazas up to twice this wide between its footprints are town (no scatter)"),
        d("flora.town_cell_m", 1.0, "m", 0.1, 10.0, "cell edge of a guide town's exclusion mask (the area's precision)"),
        d("flora.town_cells_max", 1.6e7, "count", 1.0e4, 1.0e9, "upper bound of one town mask's cells (16 Mbit); a \
           larger town gets coarser cells"),
    ]
}

impl Default for TownParams {
    fn default() -> Self {
        let v = |name: &str| declared().into_iter().find(|d| d.name == name).map_or(f64::NAN, |d| d.v);
        Self {
            margin_m: v("flora.town_margin_m"),
            close_m: v("flora.town_close_m"),
            cell_m: v("flora.town_cell_m"),
            cells_max: v("flora.town_cells_max"),
        }
    }
}

/// A placement's ground footprint: a rectangle, guide frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Footprint {
    pub centre: [f64; 2],
    /// Unit axis along the placement's yaw; the other axis is its left normal.
    pub axis: [f64; 2],
    pub half: [f64; 2],
}

impl Footprint {
    /// Distance from `(x, y)` to the rectangle (0 inside).
    #[must_use]
    pub fn distance(&self, x: f64, y: f64) -> f64 {
        let (dx, dy) = (x - self.centre[0], y - self.centre[1]);
        let [ux, uy] = self.axis;
        let du = (dx * ux + dy * uy).abs() - self.half[0];
        let dv = (-dx * uy + dy * ux).abs() - self.half[1];
        du.max(0.0).hypot(dv.max(0.0))
    }
}

/// The footprint of `p` from its model bounds, and whether it had any (without, a point at its position).
#[must_use]
pub fn footprint(p: &Placement, bounds: Bounds<'_>) -> (Footprint, bool) {
    let t = &p.transform;
    let e = t.pos.to_f64();
    let pos = [e.x, -e.z];
    // The yaw axis: the model's X on the ground (its Z when X stands vertical).
    let ground = |v: Vec3| [f64::from(v.x), -f64::from(v.z)];
    let [ax, ay] = ground(t.rotation * Vec3::X);
    let [ax, ay] = if ax.hypot(ay) > 1e-3 { [ax, ay] } else { ground(t.rotation * Vec3::Z) };
    let n = ax.hypot(ay).max(1e-12);
    let axis = [ax / n, ay / n];
    let point = Footprint { centre: pos, axis, half: [0.0, 0.0] };
    let Some((min, max)) = p.model.as_ref().and_then(|m| bounds(m.as_str())) else { return (point, false) };
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for k in 0..8 {
        let c = Vec3::new(
            if k & 1 == 0 { min[0] } else { max[0] },
            if k & 2 == 0 { min[1] } else { max[1] },
            if k & 4 == 0 { min[2] } else { max[2] },
        );
        let [gx, gy] = ground(t.rotation * (c * t.scale));
        let (u, v) = (gx * axis[0] + gy * axis[1], -gx * axis[1] + gy * axis[0]);
        lo = [lo[0].min(u), lo[1].min(v)];
        hi = [hi[0].max(u), hi[1].max(v)];
    }
    let (mu, mv) = (f64::midpoint(lo[0], hi[0]), f64::midpoint(lo[1], hi[1]));
    let centre = [pos[0] + mu * axis[0] - mv * axis[1], pos[1] + mu * axis[1] + mv * axis[0]];
    (Footprint { centre, axis, half: [(hi[0] - lo[0]) * 0.5, (hi[1] - lo[1]) * 0.5] }, true)
}

/// One town's mask: a bit per cell, row-major from the min corner.
#[derive(Clone, Debug)]
struct Raster {
    min: [f64; 2],
    cell: f64,
    w: usize,
    h: usize,
    bits: Vec<bool>,
}

impl Raster {
    fn contains(&self, x: f64, y: f64) -> bool {
        let (c, r) = ((x - self.min[0]) / self.cell, (y - self.min[1]) / self.cell);
        if c < 0.0 || r < 0.0 {
            return false;
        }
        let (c, r) = (c as usize, r as usize);
        c < self.w && r < self.h && self.bits[r * self.w + c]
    }
}

/// The census of a town exclusion.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TownCensus {
    pub towns: usize,
    pub footprints: usize,
    /// Placements whose model has no bounds (points).
    pub unbounded: usize,
    /// Area of the masks, m².
    pub area_m2: f64,
    /// Scatter candidates the towns excluded (outside every stamp; set by the pass).
    pub excluded: usize,
}

/// Every guide town of a pass as one exclusion ([module docs](self)).
#[derive(Clone, Debug, Default)]
pub struct TownMask {
    rasters: Vec<Raster>,
    pub census: TownCensus,
}

impl TownMask {
    /// The mask of `towns` (one placement list per town; an empty list adds nothing).
    #[must_use]
    pub fn new(towns: &[&[Placement]], bounds: Bounds<'_>, prm: &TownParams) -> Self {
        let mut mask = Self::default();
        for town in towns.iter().filter(|t| !t.is_empty()) {
            let fps: Vec<Footprint> = town
                .iter()
                .map(|p| {
                    let (f, bounded) = footprint(p, bounds);
                    mask.census.unbounded += usize::from(!bounded);
                    f
                })
                .collect();
            mask.census.footprints += fps.len();
            let r = raster(&fps, prm);
            mask.census.area_m2 += r.bits.iter().filter(|&&b| b).count() as f64 * r.cell * r.cell;
            mask.rasters.push(r);
            mask.census.towns += 1;
        }
        mask
    }

    /// Inside a town's area (apron included).
    #[must_use]
    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.rasters.iter().any(|r| r.contains(x, y))
    }
}

/// A terrain with the towns excluded too, counting the points only the towns exclude.
pub struct WithTowns<'a> {
    pub terrain: &'a dyn Terrain,
    pub towns: &'a TownMask,
    pub hits: AtomicUsize,
}

impl<'a> WithTowns<'a> {
    #[must_use]
    pub fn new(terrain: &'a dyn Terrain, towns: &'a TownMask) -> Self {
        Self { terrain, towns, hits: AtomicUsize::new(0) }
    }

    /// The count so far, reset to 0.
    pub fn take_hits(&self) -> usize {
        self.hits.swap(0, Ordering::Relaxed)
    }
}

impl Terrain for WithTowns<'_> {
    fn regions_in(&self, rect: [f64; 4], out: &mut Vec<u16>) {
        self.terrain.regions_in(rect, out);
    }

    fn region(&self, x: f64, y: f64) -> Option<u16> {
        self.terrain.region(x, y)
    }

    fn site(&self, x: f64, y: f64) -> Option<Site> {
        self.terrain.site(x, y)
    }

    fn excluded(&self, x: f64, y: f64) -> bool {
        if self.terrain.excluded(x, y) {
            return true;
        }
        let hit = self.towns.contains(x, y);
        if hit {
            self.hits.fetch_add(1, Ordering::Relaxed);
        }
        hit
    }

    fn height(&self, x: f64, y: f64) -> f64 {
        self.terrain.height(x, y)
    }
}

/// Squared distance stand-in for "no seed" (finite: the transform's intersections must not see ∞ − ∞).
// tunables: const (maths: a finite infinity far above any squared cell distance)
const FAR: f64 = 1e20;

/// The town's mask from its footprints ([module docs](self), steps 1-4).
fn raster(fps: &[Footprint], prm: &TownParams) -> Raster {
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for f in fps {
        let ext = [f.half[0] * f.axis[0].abs() + f.half[1] * f.axis[1].abs(), f.half[0] * f.axis[1].abs() + f.half[1] * f.axis[0].abs()];
        for a in 0..2 {
            lo[a] = lo[a].min(f.centre[a] - ext[a]);
            hi[a] = hi[a].max(f.centre[a] + ext[a]);
        }
    }
    // The cell, coarser when the town would pass the cell budget; the padding keeps the outside (flood seed) clear of
    // the closing and the apron.
    let reach = prm.close_m + prm.margin_m;
    let span = [hi[0] - lo[0] + 2.0 * reach, hi[1] - lo[1] + 2.0 * reach];
    let cell = prm.cell_m.max((span[0] * span[1] / prm.cells_max).sqrt());
    let pad = (reach / cell).ceil() + 3.0;
    let min = [lo[0] - pad * cell, lo[1] - pad * cell];
    let (w, h) = (((hi[0] - lo[0]) / cell + 2.0 * pad).ceil() as usize + 1, ((hi[1] - lo[1]) / cell + 2.0 * pad).ceil() as usize + 1);
    let centre = |c: usize, r: usize| [min[0] + (c as f64 + 0.5) * cell, min[1] + (r as f64 + 0.5) * cell];
    // 1. Every cell a footprint touches: its centre within half a diagonal of the rectangle.
    let slack = cell * std::f64::consts::FRAC_1_SQRT_2;
    let mut built = vec![false; w * h];
    for f in fps {
        let ext = [f.half[0] * f.axis[0].abs() + f.half[1] * f.axis[1].abs(), f.half[0] * f.axis[1].abs() + f.half[1] * f.axis[0].abs()];
        let span = |a: usize, n: usize| {
            let a0 = ((f.centre[a] - ext[a] - slack - min[a]) / cell).floor().max(0.0) as usize;
            let a1 = (((f.centre[a] + ext[a] + slack - min[a]) / cell).ceil() as usize).min(n - 1);
            a0..=a1
        };
        for r in span(1, h) {
            for c in span(0, w) {
                let [x, y] = centre(c, r);
                if f.distance(x, y) <= slack {
                    built[r * w + c] = true;
                }
            }
        }
    }
    // 2. Closing: dilate by close_m, erode the result by close_m.
    let close = (prm.close_m / cell).powi(2);
    let dilated: Vec<bool> = edt_sq(&built, w, h).into_iter().map(|d| d <= close).collect();
    let outside: Vec<bool> = dilated.iter().map(|&b| !b).collect();
    let mut closed: Vec<bool> = edt_sq(&outside, w, h).into_iter().map(|d| d > close).collect();
    // 3. Holes: whatever the border's flood can't reach.
    let mut reached = vec![false; w * h];
    let mut queue = VecDeque::new();
    for r in 0..h {
        for c in 0..w {
            if (r == 0 || c == 0 || r == h - 1 || c == w - 1) && !closed[r * w + c] {
                reached[r * w + c] = true;
                queue.push_back((c, r));
            }
        }
    }
    while let Some((c, r)) = queue.pop_front() {
        let mut visit = |c: usize, r: usize| {
            let k = r * w + c;
            if !closed[k] && !reached[k] {
                reached[k] = true;
                queue.push_back((c, r));
            }
        };
        if c > 0 {
            visit(c - 1, r);
        }
        if c + 1 < w {
            visit(c + 1, r);
        }
        if r > 0 {
            visit(c, r - 1);
        }
        if r + 1 < h {
            visit(c, r + 1);
        }
    }
    for (k, b) in closed.iter_mut().enumerate() {
        *b |= !reached[k];
    }
    // 4. Apron: margin_m plus the rasterisation's slack (a point and a footprint point each up to half a diagonal
    // from their cell centres).
    let apron = (prm.margin_m / cell + std::f64::consts::SQRT_2).powi(2);
    let bits = edt_sq(&closed, w, h).into_iter().map(|d| d <= apron).collect();
    Raster { min, cell, w, h, bits }
}

/// Squared Euclidean distance, in cells, from each cell to the nearest `seed` cell ([`FAR`] when none): the separable
/// exact transform of Felzenszwalb & Huttenlocher ("Distance Transforms of Sampled Functions", 2012), columns then
/// rows.
fn edt_sq(seed: &[bool], w: usize, h: usize) -> Vec<f64> {
    let mut d: Vec<f64> = seed.iter().map(|&s| if s { 0.0 } else { FAR }).collect();
    let n = w.max(h);
    let (mut f, mut out, mut v, mut z) = (vec![0.0; n], vec![0.0; n], vec![0usize; n], vec![0.0; n + 1]);
    for c in 0..w {
        for r in 0..h {
            f[r] = d[r * w + c];
        }
        dt1(&f[..h], &mut out[..h], &mut v, &mut z);
        for r in 0..h {
            d[r * w + c] = out[r];
        }
    }
    for r in 0..h {
        f[..w].copy_from_slice(&d[r * w..(r + 1) * w]);
        dt1(&f[..w], &mut out[..w], &mut v, &mut z);
        d[r * w..(r + 1) * w].copy_from_slice(&out[..w]);
    }
    d
}

/// The 1D transform: `out[q] = min_p (q − p)² + f[p]` (lower envelope of parabolas).
fn dt1(f: &[f64], out: &mut [f64], v: &mut [usize], z: &mut [f64]) {
    let n = f.len();
    let sq = |q: usize| (q * q) as f64;
    let cross = |q: usize, p: usize| ((f[q] + sq(q)) - (f[p] + sq(p))) / (2.0 * (q as f64 - p as f64));
    let mut k = 0;
    v[0] = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let mut s = cross(q, v[k]);
        while s <= z[k] {
            k -= 1;
            s = cross(q, v[k]);
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, o) in out.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k];
        *o = (q as f64 - p as f64).powi(2) + f[p];
    }
}
