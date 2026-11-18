//! Raster geometry shared by the detectors: connected-component labelling, outer-boundary tracing, perimeter,
//! polyline simplification, bilinear sampling and box means.
//!
//! - Labelling: 8-connected breadth-first flood in raster order (deterministic labels 1..=n).
//! - Outline: Moore-neighbour tracing of the outer boundary through sample centres (Gonzalez & Woods, "Digital
//!   Image Processing", §11.1.1), stopped when the first move repeats from the start (Jacob's criterion).
//!   Holes are not traced.
//! - Perimeter of the traced chain: the corner-count estimator of Vossepoel & Smeulders ("Vector code probability
//!   and metrication error in the representation of straight lines of finite length", CGIP 20, 1982):
//!   `L = 0.980 nₑ + 1.406 nₒ − 0.091 n_c` (even steps, odd steps, corners), unbiased for straight edges of any
//!   direction, so circularity `4πA/P²` is close to 1 for a disc.
//! - Simplification: Douglas & Peucker ("Algorithms for the reduction of the number of points required to
//!   represent a digitized line or its caricature", Cartographica 10(2), 1973), iterative, tolerance doubled until
//!   the point budget holds.

use std::collections::VecDeque;

use crate::Grid;
use crate::grid::neighbour;

/// 8-connected labels of `mask` (0 = outside, 1..=n in raster order of each component's first sample) and `n`.
pub fn label(mask: &[bool], w: usize, h: usize) -> (Vec<u32>, usize) {
    let mut lab = vec![0u32; w * h];
    let mut n = 0u32;
    let mut q = VecDeque::new();
    for s in 0..w * h {
        if !mask[s] || lab[s] != 0 {
            continue;
        }
        n += 1;
        lab[s] = n;
        q.push_back(s);
        while let Some(c) = q.pop_front() {
            for k in 0..8 {
                if let Some(j) = neighbour(w, h, c, k)
                    && mask[j]
                    && lab[j] == 0
                {
                    lab[j] = n;
                    q.push_back(j);
                }
            }
        }
    }
    (lab, n as usize)
}

/// Per-component sample lists (index order) of a labelling with `n` labels; entry `l − 1` is label `l`.
pub fn members(lab: &[u32], n: usize) -> Vec<Vec<u32>> {
    let mut out = vec![Vec::new(); n];
    for (i, &l) in lab.iter().enumerate() {
        if l != 0 {
            out[l as usize - 1].push(i as u32);
        }
    }
    out
}

/// True when any sample of `m` lies on the grid's outer ring.
pub fn touches_border(m: &[u32], w: usize, h: usize) -> bool {
    m.iter().any(|&i| {
        let (x, y) = (i as usize % w, i as usize / w);
        x == 0 || y == 0 || x + 1 == w || y + 1 == h
    })
}

/// Mean sample position `(col, row)` and bounding box `(c0, r0, c1, r1)` of a sample list.
pub fn centroid_bbox(m: &[u32], w: usize) -> ([f64; 2], [f64; 4]) {
    let (mut sx, mut sy) = (0.0, 0.0);
    let mut bb = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    for &i in m {
        let (x, y) = ((i as usize % w) as f64, (i as usize / w) as f64);
        sx += x;
        sy += y;
        bb = [bb[0].min(x), bb[1].min(y), bb[2].max(x), bb[3].max(y)];
    }
    let n = m.len().max(1) as f64;
    ([sx / n, sy / n], bb)
}

// Clockwise ring in image coordinates (x right, y down), starting west.
const RING: [(i64, i64); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn ring_index(d: (i64, i64)) -> usize {
    RING.iter().position(|&r| r == d).unwrap_or(0)
}

/// Outer boundary of the component `l` of `lab` whose first raster sample is `start`, as sample centres
/// `(col, row)` in clockwise order (image coordinates), closed implicitly (the start is not repeated).
pub fn trace_outline(lab: &[u32], w: usize, h: usize, l: u32, start: usize) -> Vec<(i64, i64)> {
    let inside = |p: (i64, i64)| {
        p.0 >= 0
            && p.1 >= 0
            && (p.0 as usize) < w
            && (p.1 as usize) < h
            && lab[p.1 as usize * w + p.0 as usize] == l
    };
    let s = ((start % w) as i64, (start / w) as i64);
    let mut pts = vec![s];
    let mut p = s;
    // The start is the first sample in raster order, so its west neighbour is outside.
    let mut back = 0usize;
    let mut first: Option<(i64, i64)> = None;
    // A boundary visits each sample at most 4 times; the cap only guards against a logic error.
    let cap = 4 * w * h + 8;
    for _ in 0..cap {
        let mut found = None;
        for k in 1..=8 {
            let d = (back + k) % 8;
            let q = (p.0 + RING[d].0, p.1 + RING[d].1);
            if inside(q) {
                found = Some((d, q));
                break;
            }
        }
        let Some((d, q)) = found else {
            break; // single sample
        };
        if p == s {
            match first {
                Some(f) if f == q => break,
                None => first = Some(q),
                _ => {}
            }
        }
        let prev = (p.0 + RING[(d + 7) % 8].0, p.1 + RING[(d + 7) % 8].1);
        back = ring_index((prev.0 - q.0, prev.1 - q.1));
        pts.push(q);
        p = q;
    }
    if pts.len() > 1 && pts.last() == Some(&s) {
        pts.pop();
    }
    pts
}

/// Perimeter (in samples) of a closed 8-connected chain by the Vossepoel & Smeulders estimator.
pub fn chain_perimeter(pts: &[(i64, i64)]) -> f64 {
    let n = pts.len();
    if n < 2 {
        return 0.0;
    }
    let (mut ne, mut no, mut nc) = (0usize, 0usize, 0usize);
    let mut last: Option<(i64, i64)> = None;
    for i in 0..n {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let d = (b.0 - a.0, b.1 - a.1);
        if d.0 == 0 || d.1 == 0 {
            ne += 1;
        } else {
            no += 1;
        }
        if last.is_some_and(|l| l != d) {
            nc += 1;
        }
        last = Some(d);
    }
    if pts.len() > 2 {
        let d0 = (pts[1].0 - pts[0].0, pts[1].1 - pts[0].1);
        if last.is_some_and(|l| l != d0) {
            nc += 1;
        }
    }
    0.980 * ne as f64 + 1.406 * no as f64 - 0.091 * nc as f64
}

fn seg_dist(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 {
        (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    ((p[0] - a[0] - t * dx).powi(2) + (p[1] - a[1] - t * dy).powi(2)).sqrt()
}

/// Distance from `p` to the polyline `line` (a point when it has one vertex).
pub fn polyline_dist(p: [f64; 2], line: &[[f64; 2]]) -> f64 {
    match line.len() {
        0 => f64::INFINITY,
        1 => ((p[0] - line[0][0]).powi(2) + (p[1] - line[0][1]).powi(2)).sqrt(),
        _ => line
            .windows(2)
            .map(|s| seg_dist(p, s[0], s[1]))
            .fold(f64::INFINITY, f64::min),
    }
}

/// True when `p` is inside the closed polygon `poly` (even-odd rule).
pub fn inside_polygon(p: [f64; 2], poly: &[[f64; 2]]) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + n - 1) % n]);
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            inside = !inside;
        }
    }
    inside
}

/// Douglas-Peucker on an open polyline: indices kept, first and last always.
fn dp_keep(pts: &[[f64; 2]], tol: f64) -> Vec<bool> {
    let n = pts.len();
    let mut keep = vec![false; n];
    if n == 0 {
        return keep;
    }
    keep[0] = true;
    keep[n - 1] = true;
    let mut stack = vec![(0usize, n - 1)];
    while let Some((a, b)) = stack.pop() {
        if b <= a + 1 {
            continue;
        }
        let (mut best, mut bi) = (-1.0, a);
        for (i, &p) in pts.iter().enumerate().take(b).skip(a + 1) {
            let d = seg_dist(p, pts[a], pts[b]);
            if d > best {
                best = d;
                bi = i;
            }
        }
        if best > tol {
            keep[bi] = true;
            stack.push((a, bi));
            stack.push((bi, b));
        }
    }
    keep
}

/// Simplified polyline (open, or closed when `closed`: split at the start and its farthest vertex), tolerance
/// `tol` doubled until at most `max_pts` vertices remain.
pub fn simplify(pts: &[[f64; 2]], closed: bool, tol: f64, max_pts: usize) -> Vec<[f64; 2]> {
    if pts.len() <= 2 {
        return pts.to_vec();
    }
    let mut tol = tol.max(1e-9);
    loop {
        let out: Vec<[f64; 2]> = if closed {
            let far = (1..pts.len())
                .max_by(|&a, &b| {
                    let da = (pts[a][0] - pts[0][0]).hypot(pts[a][1] - pts[0][1]);
                    let db = (pts[b][0] - pts[0][0]).hypot(pts[b][1] - pts[0][1]);
                    da.total_cmp(&db).then(b.cmp(&a))
                })
                .unwrap_or(1);
            let mut first: Vec<[f64; 2]> = pts[..=far].to_vec();
            let mut second: Vec<[f64; 2]> = pts[far..].to_vec();
            second.push(pts[0]);
            let k1 = dp_keep(&first, tol);
            let k2 = dp_keep(&second, tol);
            first = first
                .iter()
                .zip(&k1)
                .filter_map(|(p, &k)| k.then_some(*p))
                .collect();
            let tail: Vec<[f64; 2]> = second
                .iter()
                .zip(&k2)
                .filter_map(|(p, &k)| k.then_some(*p))
                .collect();
            // Drop the duplicated split vertex and the repeated start.
            first.extend_from_slice(&tail[1..tail.len() - 1]);
            first
        } else {
            let k = dp_keep(pts, tol);
            pts.iter()
                .zip(&k)
                .filter_map(|(p, &k)| k.then_some(*p))
                .collect()
        };
        if out.len() <= max_pts {
            return out;
        }
        tol *= 2.0;
    }
}

/// Bilinear height at the fractional sample position `(x, y)`, or `None` outside the grid.
pub fn bilinear(g: &Grid, x: f64, y: f64) -> Option<f64> {
    if x < 0.0 || y < 0.0 || x > (g.w - 1) as f64 || y > (g.h - 1) as f64 {
        return None;
    }
    let (x0, y0) = (x.floor() as usize, y.floor() as usize);
    let (x1, y1) = ((x0 + 1).min(g.w - 1), (y0 + 1).min(g.h - 1));
    let (fx, fy) = (x - x0 as f64, y - y0 as f64);
    let a = g.at(x0, y0) * (1.0 - fx) + g.at(x1, y0) * fx;
    let b = g.at(x0, y1) * (1.0 - fx) + g.at(x1, y1) * fx;
    Some(a * (1.0 - fy) + b * fy)
}

/// Mean of `v` over the square window of half-width `r` samples around every sample (clipped to the grid), by
/// a summed-area table (Crow, "Summed-area tables for texture mapping", SIGGRAPH 1984).
pub fn box_mean(v: &[f64], w: usize, h: usize, r: usize) -> Vec<f64> {
    let mut sat = vec![0.0f64; (w + 1) * (h + 1)];
    for y in 0..h {
        let mut row = 0.0;
        for x in 0..w {
            row += v[y * w + x];
            sat[(y + 1) * (w + 1) + x + 1] = sat[y * (w + 1) + x + 1] + row;
        }
    }
    let mut out = vec![0.0; w * h];
    for y in 0..h {
        let (y0, y1) = (y.saturating_sub(r), (y + r + 1).min(h));
        for x in 0..w {
            let (x0, x1) = (x.saturating_sub(r), (x + r + 1).min(w));
            let s = sat[y1 * (w + 1) + x1] - sat[y0 * (w + 1) + x1] - sat[y1 * (w + 1) + x0]
                + sat[y0 * (w + 1) + x0];
            out[y * w + x] = s / ((x1 - x0) * (y1 - y0)) as f64;
        }
    }
    out
}

/// Horn slope in degrees per sample (edge-clamped), wet samples 0.
pub fn slope_deg(g: &Grid, wet: &[bool]) -> Vec<f64> {
    use rayon::prelude::*;
    let mut out = vec![0.0; g.len()];
    out.par_chunks_mut(g.w).enumerate().for_each(|(y, row)| {
        let yi = y as isize;
        for (x, o) in row.iter_mut().enumerate() {
            if wet[y * g.w + x] {
                continue;
            }
            let xi = x as isize;
            let s = |dx: isize, dy: isize| g.at_clamped(xi + dx, yi + dy);
            let de = ((s(1, -1) + 2.0 * s(1, 0) + s(1, 1))
                - (s(-1, -1) + 2.0 * s(-1, 0) + s(-1, 1)))
                / (8.0 * g.spacing);
            let dn = ((s(-1, -1) + 2.0 * s(0, -1) + s(1, -1))
                - (s(-1, 1) + 2.0 * s(0, 1) + s(1, 1)))
                / (8.0 * g.spacing);
            *o = de.hypot(dn).atan().to_degrees();
        }
    });
    out
}
