//! Detour's vector and geometry helpers (`DetourCommon`), in upstream's exact
//! operation order so results are bit-identical.

use crate::common::sqrtf;

pub(crate) type V3 = [f32; 3];

#[inline]
pub(crate) fn v3(s: &[f32]) -> V3 {
    [s[0], s[1], s[2]]
}
#[inline]
pub(crate) fn fmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
#[inline]
pub(crate) fn fmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
#[inline]
pub(crate) fn fclamp(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}
#[inline]
pub(crate) fn sqr(a: f32) -> f32 {
    a * a
}
#[inline]
pub(crate) fn vmad(a: &[f32], b: &[f32], s: f32) -> V3 {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
#[inline]
pub(crate) fn vlerp(a: &[f32], b: &[f32], t: f32) -> V3 {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}
#[inline]
pub(crate) fn vadd(a: &[f32], b: &[f32]) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
#[inline]
pub(crate) fn vsub(a: &[f32], b: &[f32]) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline]
pub(crate) fn vscale(v: &[f32], t: f32) -> V3 {
    [v[0] * t, v[1] * t, v[2] * t]
}
#[inline]
pub(crate) fn vmin(mn: &mut V3, v: &[f32]) {
    mn[0] = fmin(mn[0], v[0]);
    mn[1] = fmin(mn[1], v[1]);
    mn[2] = fmin(mn[2], v[2]);
}
#[inline]
pub(crate) fn vmax(mx: &mut V3, v: &[f32]) {
    mx[0] = fmax(mx[0], v[0]);
    mx[1] = fmax(mx[1], v[1]);
    mx[2] = fmax(mx[2], v[2]);
}
#[inline]
pub(crate) fn vdist(a: &[f32], b: &[f32]) -> f32 {
    sqrtf(vdist_sqr(a, b))
}
#[inline]
pub(crate) fn vdist_sqr(a: &[f32], b: &[f32]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let dz = b[2] - a[2];
    dx * dx + dy * dy + dz * dz
}
#[inline]
pub(crate) fn vlen_sqr(v: &[f32]) -> f32 {
    v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
}
#[inline]
pub(crate) fn vnormalize(v: &mut V3) {
    let d = 1.0 / sqrtf(sqr(v[0]) + sqr(v[1]) + sqr(v[2]));
    v[0] *= d;
    v[1] *= d;
    v[2] *= d;
}
#[inline]
pub(crate) fn vequal(a: &[f32], b: &[f32]) -> bool {
    let thr = sqr(1.0 / 16384.0);
    vdist_sqr(a, b) < thr
}
#[inline]
pub(crate) fn visfinite(v: &[f32]) -> bool {
    v[0].is_finite() && v[1].is_finite() && v[2].is_finite()
}
#[inline]
pub(crate) fn visfinite_2d(v: &[f32]) -> bool {
    v[0].is_finite() && v[2].is_finite()
}
#[inline]
pub(crate) fn vdot_2d(u: &[f32], v: &[f32]) -> f32 {
    u[0] * v[0] + u[2] * v[2]
}
#[inline]
pub(crate) fn vperp_2d(u: &[f32], v: &[f32]) -> f32 {
    u[2] * v[0] - u[0] * v[2]
}
#[inline]
pub(crate) fn tri_area_2d(a: &[f32], b: &[f32], c: &[f32]) -> f32 {
    let abx = b[0] - a[0];
    let abz = b[2] - a[2];
    let acx = c[0] - a[0];
    let acz = c[2] - a[2];
    acx * abz - abx * acz
}
#[inline]
pub(crate) fn overlap_quant_bounds(
    amin: &[u16; 3],
    amax: &[u16; 3],
    bmin: &[u16; 3],
    bmax: &[u16; 3],
) -> bool {
    !(amin[0] > bmax[0]
        || amax[0] < bmin[0]
        || amin[1] > bmax[1]
        || amax[1] < bmin[1]
        || amin[2] > bmax[2]
        || amax[2] < bmin[2])
}
#[inline]
pub(crate) fn overlap_bounds(amin: &[f32], amax: &[f32], bmin: &[f32], bmax: &[f32]) -> bool {
    !(amin[0] > bmax[0]
        || amax[0] < bmin[0]
        || amin[1] > bmax[1]
        || amax[1] < bmin[1]
        || amin[2] > bmax[2]
        || amax[2] < bmin[2])
}
#[inline]
pub(crate) fn next_pow2(mut v: u32) -> u32 {
    v = v.wrapping_sub(1);
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v.wrapping_add(1)
}
#[inline]
pub(crate) fn ilog2(mut v: u32) -> u32 {
    let mut r = ((v > 0xffff) as u32) << 4;
    v >>= r;
    let mut shift = ((v > 0xff) as u32) << 3;
    v >>= shift;
    r |= shift;
    shift = ((v > 0xf) as u32) << 2;
    v >>= shift;
    r |= shift;
    shift = ((v > 0x3) as u32) << 1;
    v >>= shift;
    r |= shift;
    r | (v >> 1)
}
#[inline]
pub(crate) fn align4(x: usize) -> usize {
    (x + 3) & !3
}
#[inline]
pub(crate) fn opposite_tile(side: i32) -> i32 {
    (side + 4) & 0x7
}

#[inline]
fn vdot(a: &[f32], b: &[f32]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[allow(dead_code)] // dtClosestPtPointTriangle: used by the crowd and tile cache ports
pub(crate) fn closest_pt_point_triangle(p: &[f32], a: &[f32], b: &[f32], c: &[f32]) -> V3 {
    let ab = vsub(b, a);
    let ac = vsub(c, a);
    let ap = vsub(p, a);
    let d1 = vdot(&ab, &ap);
    let d2 = vdot(&ac, &ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return v3(a);
    }
    let bp = vsub(p, b);
    let d3 = vdot(&ab, &bp);
    let d4 = vdot(&ac, &bp);
    if d3 >= 0.0 && d4 <= d3 {
        return v3(b);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return [a[0] + v * ab[0], a[1] + v * ab[1], a[2] + v * ab[2]];
    }
    let cp = vsub(p, c);
    let d5 = vdot(&ab, &cp);
    let d6 = vdot(&ac, &cp);
    if d6 >= 0.0 && d5 <= d6 {
        return v3(c);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return [a[0] + w * ac[0], a[1] + w * ac[1], a[2] + w * ac[2]];
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return [
            b[0] + w * (c[0] - b[0]),
            b[1] + w * (c[1] - b[1]),
            b[2] + w * (c[2] - b[2]),
        ];
    }
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    [
        a[0] + ab[0] * v + ac[0] * w,
        a[1] + ab[1] * v + ac[1] * w,
        a[2] + ab[2] * v + ac[2] * w,
    ]
}

/// Segment against convex polygon in xz: `(tmin, tmax, seg_min, seg_max)`.
pub(crate) fn intersect_segment_poly_2d(
    p0: &[f32],
    p1: &[f32],
    verts: &[f32],
    nverts: usize,
) -> Option<(f32, f32, i32, i32)> {
    const EPS: f32 = 0.000001;
    let mut tmin = 0.0f32;
    let mut tmax = 1.0f32;
    let mut seg_min = -1;
    let mut seg_max = -1;
    let dir = vsub(p1, p0);
    let mut j = nverts.wrapping_sub(1);
    for i in 0..nverts {
        let edge = vsub(&verts[i * 3..], &verts[j * 3..]);
        let diff = vsub(p0, &verts[j * 3..]);
        let n = vperp_2d(&edge, &diff);
        let d = vperp_2d(&dir, &edge);
        if d.abs() < EPS {
            if n < 0.0 {
                return None;
            }
            j = i;
            continue;
        }
        let t = n / d;
        if d < 0.0 {
            if t > tmin {
                tmin = t;
                seg_min = j as i32;
                if tmin > tmax {
                    return None;
                }
            }
        } else if t < tmax {
            tmax = t;
            seg_max = j as i32;
            if tmax < tmin {
                return None;
            }
        }
        j = i;
    }
    Some((tmin, tmax, seg_min, seg_max))
}

/// Squared xz distance from `pt` to segment `p`..`q`, and the parameter.
pub(crate) fn distance_pt_seg_sqr_2d(pt: &[f32], p: &[f32], q: &[f32]) -> (f32, f32) {
    let pqx = q[0] - p[0];
    let pqz = q[2] - p[2];
    let mut dx = pt[0] - p[0];
    let mut dz = pt[2] - p[2];
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
    dx = p[0] + t * pqx - pt[0];
    dz = p[2] + t * pqz - pt[2];
    (dx * dx + dz * dz, t)
}

pub(crate) fn closest_height_point_triangle(
    p: &[f32],
    a: &[f32],
    b: &[f32],
    c: &[f32],
) -> Option<f32> {
    const EPS: f32 = 1e-6;
    let v0 = vsub(c, a);
    let v1 = vsub(b, a);
    let v2 = vsub(p, a);
    let mut denom = v0[0] * v1[2] - v0[2] * v1[0];
    if denom.abs() < EPS {
        return None;
    }
    let mut u = v1[2] * v2[0] - v1[0] * v2[2];
    let mut v = v0[0] * v2[2] - v0[2] * v2[0];
    if denom < 0.0 {
        denom = -denom;
        u = -u;
        v = -v;
    }
    if u >= 0.0 && v >= 0.0 && (u + v) <= denom {
        return Some(a[1] + (v0[1] * u + v1[1] * v) / denom);
    }
    None
}

#[inline]
fn crosses(pt: &[f32], vi: &[f32], vj: &[f32]) -> bool {
    ((vi[2] > pt[2]) != (vj[2] > pt[2]))
        && (pt[0] < (vj[0] - vi[0]) * (pt[2] - vi[2]) / (vj[2] - vi[2]) + vi[0])
}

pub(crate) fn point_in_polygon(pt: &[f32], verts: &[f32], nverts: usize) -> bool {
    let mut c = false;
    let mut j = nverts.wrapping_sub(1);
    for i in 0..nverts {
        if crosses(pt, &verts[i * 3..], &verts[j * 3..]) {
            c = !c;
        }
        j = i;
    }
    c
}

pub(crate) fn distance_pt_poly_edges_sqr(
    pt: &[f32],
    verts: &[f32],
    nverts: usize,
    ed: &mut [f32],
    et: &mut [f32],
) -> bool {
    let mut c = false;
    let mut j = nverts.wrapping_sub(1);
    for i in 0..nverts {
        let vi = &verts[i * 3..];
        let vj = &verts[j * 3..];
        if crosses(pt, vi, vj) {
            c = !c;
        }
        let (d, t) = distance_pt_seg_sqr_2d(pt, vj, vi);
        ed[j] = d;
        et[j] = t;
        j = i;
    }
    c
}

fn project_poly(axis: &[f32], poly: &[f32], npoly: usize) -> (f32, f32) {
    let mut rmin = vdot_2d(axis, &poly[0..]);
    let mut rmax = rmin;
    for i in 1..npoly {
        let d = vdot_2d(axis, &poly[i * 3..]);
        rmin = fmin(rmin, d);
        rmax = fmax(rmax, d);
    }
    (rmin, rmax)
}

pub(crate) fn overlap_poly_poly_2d(
    polya: &[f32],
    npolya: usize,
    polyb: &[f32],
    npolyb: usize,
) -> bool {
    const EPS: f32 = 1e-4;
    for (poly, n) in [(polya, npolya), (polyb, npolyb)] {
        let mut j = n.wrapping_sub(1);
        for i in 0..n {
            let va = &poly[j * 3..];
            let vb = &poly[i * 3..];
            let nrm = [vb[2] - va[2], 0.0, -(vb[0] - va[0])];
            let (amin, amax) = project_poly(&nrm, polya, npolya);
            let (bmin, bmax) = project_poly(&nrm, polyb, npolyb);
            if (amin + EPS) > bmax || (amax - EPS) < bmin {
                return false;
            }
            j = i;
        }
    }
    true
}

pub(crate) fn random_point_in_convex_poly(
    pts: &[f32],
    npts: usize,
    areas: &mut [f32],
    s: f32,
    t: f32,
) -> V3 {
    let mut areasum = 0.0f32;
    for i in 2..npts {
        areas[i] = tri_area_2d(&pts[0..], &pts[(i - 1) * 3..], &pts[i * 3..]);
        areasum += fmax(0.001, areas[i]);
    }
    let thr = s * areasum;
    let mut acc = 0.0f32;
    let mut u = 1.0f32;
    let mut tri = npts - 1;
    for (i, &dacc) in areas.iter().enumerate().take(npts).skip(2) {
        if thr >= acc && thr < (acc + dacc) {
            u = (thr - acc) / dacc;
            tri = i;
            break;
        }
        acc += dacc;
    }
    let v = sqrtf(t);
    let a = 1.0 - v;
    let b = (1.0 - u) * v;
    let c = u * v;
    let pa = &pts[0..];
    let pb = &pts[(tri - 1) * 3..];
    let pc = &pts[tri * 3..];
    [
        a * pa[0] + b * pb[0] + c * pc[0],
        a * pa[1] + b * pb[1] + c * pc[1],
        a * pa[2] + b * pb[2] + c * pc[2],
    ]
}

#[inline]
fn vperp_xz(a: &[f32], b: &[f32]) -> f32 {
    a[0] * b[2] - a[2] * b[0]
}

pub(crate) fn intersect_seg_seg_2d(
    ap: &[f32],
    aq: &[f32],
    bp: &[f32],
    bq: &[f32],
) -> Option<(f32, f32)> {
    let u = vsub(aq, ap);
    let v = vsub(bq, bp);
    let w = vsub(ap, bp);
    let d = vperp_xz(&u, &v);
    if d.abs() < 1e-6 {
        return None;
    }
    Some((vperp_xz(&v, &w) / d, vperp_xz(&u, &w) / d))
}
