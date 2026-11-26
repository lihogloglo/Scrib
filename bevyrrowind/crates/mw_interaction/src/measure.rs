//! A census furniture model's [`Measured`] shape from its triangles (`IN32b1`, interaction-adapter.md
//! "As built (`IN32b1`)"): pure code the bake (`IN32b2`) calls once per furniture base record with the
//! census triangle soup (collision first, render fallback) and the census top
//! (`Shape::top_below(kind.cap())`).
//!
//! Space: the model's own frame, engine axes, metres, Y up, the floor = the lowest point (as the
//! census). The footprint is the x/z bounds of every triangle, its centre their middle; `floor` = the
//! lowest point's y (slot roots stand there).
//!
//! - **Seat front** = the side opposite its backrest: the parts of triangles more than
//!   [`MeasureParams::backrest_above_m`] above the top, if their area is at least
//!   [`MeasureParams::upright_min_area`] of the footprint area and their area-weighted centre is at
//!   least [`MeasureParams::backrest_offset`] of the half footprint off-centre along x or z (the
//!   larger of the two). Arms are symmetric across the seat and cancel. **No backrest** (stools,
//!   backless benches): a seat at least [`MeasureParams::bench_aspect`] times longer than deep faces
//!   the + side of its short axis (long along x: +Z; along z: +X); any other faces +Z (a fixed model
//!   axis).
//! - **Bed head** = the end of the long axis with the head- or footboard: the parts of triangles more
//!   than [`MeasureParams::headboard_above_m`] above the top inside each end zone (the outer
//!   [`MeasureParams::bed_end_share`] of the length); the end with at least
//!   [`MeasureParams::head_share`] of the two zones' area is the head, if that area reaches
//!   [`MeasureParams::upright_min_area`] of the footprint. A taller board has more area above the
//!   line, so the taller end wins; equal posts and a canopy cancel. **None** (bedrolls, hammocks, a
//!   tie): the + end of the long axis. A bed's `front` = its head (`def_for` reads only `head`).
//! - **Table, counter, shelf front** = the + side of the short axis (long along x: +Z; along z: +X):
//!   a counter's stations stand on its two long sides, a table's on all four edges, so the sign is
//!   not measured (the served side, should a slice need it, is a later measurement). `head` = `front`
//!   for every kind but beds.
//!
//! Square footprints count as long along x.

use glam::Vec3;
use smallvec::SmallVec;

use crate::defs::{Basis, Facing, Measured};
use crate::furniture::Kind;
use crate::tunables::MeasureParams;

/// A triangle clipped by planes (each plane adds at most one vertex).
type Poly = SmallVec<[Vec3; 8]>;

/// A horizontal axis of the model frame.
#[derive(Clone, Copy)]
enum Axis {
    X,
    Z,
}

impl Axis {
    /// The coordinate of `v` along this axis.
    fn of(self, v: Vec3) -> f32 {
        match self {
            Self::X => v.x,
            Self::Z => v.z,
        }
    }

    fn unit(self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Z => Vec3::Z,
        }
    }

    /// The facing along this axis, + or - by `positive`.
    fn facing(self, positive: bool) -> Facing {
        match (self, positive) {
            (Self::X, true) => Facing::PosX,
            (Self::X, false) => Facing::NegX,
            (Self::Z, true) => Facing::PosZ,
            (Self::Z, false) => Facing::NegZ,
        }
    }
}

/// The measured shape of a `kind` model from its triangles and its census `top` (height of the
/// usable surface above the floor, m), or `None`: no triangles, a non-finite point, a footprint
/// with no extent along x or z, or a top that is not a positive finite height.
pub fn measure(kind: Kind, triangles: &[[Vec3; 3]], top: f32, p: &MeasureParams) -> Option<Measured> {
    if !(top.is_finite() && top > 0.0) {
        return None;
    }
    let (lo, hi) = bounds(triangles)?;
    let size = hi - lo;
    if !(size.x > 0.0 && size.z > 0.0) {
        return None;
    }
    let center = (lo + hi) / 2.0;
    let (long, short) = if size.x >= size.z { (Axis::X, Axis::Z) } else { (Axis::Z, Axis::X) };
    let long_side = short.facing(true);
    let footprint_area = size.x * size.z;
    let top_y = lo.y + top;
    let (front, basis) = match kind {
        Kind::Seat => match seat_front(triangles, center, size, top_y, footprint_area, p) {
            Some(front) => (front, Basis::Upright),
            None if long.of(size) / short.of(size) >= p.bench_aspect => (long_side, Basis::LongSide),
            None => (Facing::PosZ, Basis::PlusZ),
        },
        Kind::Bed => match bed_head(triangles, lo, hi, long, top_y, footprint_area, p) {
            Some(head) => (head, Basis::Upright),
            None => (long.facing(true), Basis::LongEnd),
        },
        Kind::Table | Kind::Counter | Kind::Shelf => (long_side, Basis::LongSide),
    };
    Some(Measured {
        center: [center.x, center.z],
        footprint: [size.x, size.z],
        floor: lo.y,
        top,
        front,
        head: front,
        basis,
    })
}

/// Lowest and highest corner, or `None` (empty, non-finite).
fn bounds(triangles: &[[Vec3; 3]]) -> Option<(Vec3, Vec3)> {
    // `Vec3::min` skips NaN, so check every point first.
    if !triangles.iter().flatten().all(|p| p.is_finite()) {
        return None;
    }
    let mut points = triangles.iter().flatten();
    let first = *points.next()?;
    Some(points.fold((first, first), |(lo, hi), p| (lo.min(*p), hi.max(*p))))
}

/// The seat's front from its backrest, or `None` (no backrest).
fn seat_front(
    triangles: &[[Vec3; 3]],
    center: Vec3,
    size: Vec3,
    top_y: f32,
    footprint_area: f32,
    p: &MeasureParams,
) -> Option<Facing> {
    let line = top_y + p.backrest_above_m;
    let (mut area, mut moment) = (0.0, Vec3::ZERO);
    for tri in triangles {
        let (part, centre) = area_centroid(&clip(tri, Vec3::Y, line));
        area += part;
        moment += centre * part;
    }
    if area < p.upright_min_area * footprint_area {
        return None;
    }
    let off = moment / area - center;
    let share = |axis: Axis| axis.of(off) / (axis.of(size) / 2.0);
    let (sx, sz) = (share(Axis::X), share(Axis::Z));
    let (axis, s) = if sx.abs() >= sz.abs() { (Axis::X, sx) } else { (Axis::Z, sz) };
    // The backrest is on the `s` side; the front is the other one.
    (s.abs() >= p.backrest_offset).then(|| axis.facing(s < 0.0))
}

/// The bed's head end from its boards, or `None` (no boards, or no end clearly taller).
fn bed_head(
    triangles: &[[Vec3; 3]],
    lo: Vec3,
    hi: Vec3,
    long: Axis,
    top_y: f32,
    footprint_area: f32,
    p: &MeasureParams,
) -> Option<Facing> {
    let line = top_y + p.headboard_above_m;
    let zone = (long.of(hi) - long.of(lo)) * p.bed_end_share;
    let n = long.unit();
    let (mut plus, mut minus) = (0.0, 0.0);
    for tri in triangles {
        let above = clip(tri, Vec3::Y, line);
        if above.is_empty() {
            continue;
        }
        plus += area_centroid(&clip(&above, n, long.of(hi) - zone)).0;
        minus += area_centroid(&clip(&above, -n, -(long.of(lo) + zone))).0;
    }
    let sum = plus + minus;
    if sum < p.upright_min_area * footprint_area {
        return None;
    }
    if plus / sum >= p.head_share {
        Some(long.facing(true))
    } else if minus / sum >= p.head_share {
        Some(long.facing(false))
    } else {
        None
    }
}

/// The part of a convex polygon where `p.dot(n) >= d` (Sutherland-Hodgman, one plane).
fn clip(poly: &[Vec3], n: Vec3, d: f32) -> Poly {
    let mut out = Poly::new();
    let Some(&last) = poly.last() else { return out };
    let mut prev = last;
    let mut prev_in = prev.dot(n) >= d;
    for &cur in poly {
        let cur_in = cur.dot(n) >= d;
        if cur_in != prev_in {
            let (from, to) = (prev.dot(n) - d, cur.dot(n) - d);
            out.push(prev + (cur - prev) * (from / (from - to)));
        }
        if cur_in {
            out.push(cur);
        }
        prev = cur;
        prev_in = cur_in;
    }
    out
}

/// Area and area-weighted centre of a planar convex polygon (a fan from its first vertex); zero
/// area for fewer than three vertices.
fn area_centroid(poly: &[Vec3]) -> (f32, Vec3) {
    let Some((&o, rest)) = poly.split_first() else { return (0.0, Vec3::ZERO) };
    let (mut area, mut moment) = (0.0, Vec3::ZERO);
    for w in rest.windows(2) {
        let a = (w[0] - o).cross(w[1] - o).length() / 2.0;
        area += a;
        moment += (o + w[0] + w[1]) / 3.0 * a;
    }
    if area > 0.0 { (area, moment / area) } else { (0.0, o) }
}
