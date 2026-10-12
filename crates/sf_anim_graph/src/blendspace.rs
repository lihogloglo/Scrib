//! Blend spaces: sample weights at an input point.
//!
//! 2D spaces use a Delaunay triangulation of the samples (UE's approach):
//! inside a triangle the weights are barycentric (at most three clips
//! play); outside the hull the input is projected onto the nearest hull
//! edge. 1D spaces (and collinear 2D ones) are a chain of segments. The
//! triangulation is done once at compile ([`triangulate`], `bake` feature)
//! over coordinates normalized by each axis' range, so axes in different
//! units (m/s and degrees) don't skew the triangles.

use glam::Vec2;

use crate::compiled::BlendSpace;

/// Up to three `(sample, weight)` pairs, weights summing to 1.
pub type Weights = [(u16, f32); 3];

impl BlendSpace {
    /// Normalizes an input point into the space's `[0, 1]` coordinates.
    pub fn normalize(&self, p: Vec2) -> Vec2 {
        let span = (self.max - self.min).max(Vec2::splat(1e-6));
        let n = (p - self.min) / span;
        if self.y.is_some() { n } else { Vec2::new(n.x, 0.0) }
    }

    /// Sample weights at normalized point `p`. Returns how many entries of
    /// the result are used.
    pub fn weights(&self, p: Vec2) -> (Weights, usize) {
        let mut out = [(0, 0.0); 3];
        match self.points.len() {
            0 => return (out, 0),
            1 => {
                out[0] = (0, 1.0);
                return (out, 1);
            }
            _ => {}
        }
        for t in &self.triangles {
            let [a, b, c] = t.map(|i| self.points[usize::from(i)]);
            if let Some([wa, wb, wc]) = barycentric(p, a, b, c) {
                let mut n = 0;
                for (i, w) in [(t[0], wa), (t[1], wb), (t[2], wc)] {
                    if w > 1e-6 {
                        out[n] = (i, w);
                        n += 1;
                    }
                }
                let sum: f32 = out[..n].iter().map(|x| x.1).sum();
                for x in &mut out[..n] {
                    x.1 /= sum;
                }
                return (out, n);
            }
        }
        // Outside: the nearest point on any edge.
        let mut best = (f32::INFINITY, 0u16, 0u16, 0.0f32);
        for e in &self.edges {
            let (a, b) = (self.points[usize::from(e[0])], self.points[usize::from(e[1])]);
            let ab = b - a;
            let u = if ab.length_squared() > 0.0 { ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
            let d = p.distance_squared(a + ab * u);
            if d < best.0 {
                best = (d, e[0], e[1], u);
            }
        }
        let (_, i, j, u) = best;
        if u <= 1e-6 {
            out[0] = (i, 1.0);
            (out, 1)
        } else if u >= 1.0 - 1e-6 {
            out[0] = (j, 1.0);
            (out, 1)
        } else {
            out[0] = (i, 1.0 - u);
            out[1] = (j, u);
            (out, 2)
        }
    }
}

/// Barycentric weights of `p` in triangle `abc`, if inside (with a small
/// tolerance so shared edges never leave gaps).
fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Option<[f32; 3]> {
    const EPS: f32 = -1e-5;
    let (v0, v1, v2) = (b - a, c - a, p - a);
    let den = v0.perp_dot(v1);
    if den.abs() < 1e-12 {
        return None;
    }
    let wb = v2.perp_dot(v1) / den;
    let wc = v0.perp_dot(v2) / den;
    let wa = 1.0 - wb - wc;
    (wa >= EPS && wb >= EPS && wc >= EPS).then(|| [wa.max(0.0), wb.max(0.0), wc.max(0.0)])
}

/// Triangles and boundary edges (see [`BlendSpace`]).
#[cfg(feature = "bake")]
pub type Triangulation = (Vec<[u16; 3]>, Vec<[u16; 2]>);

/// Triangles and boundary edges of normalized sample points: a Delaunay
/// triangulation (`spade`) and its convex hull, or, for collinear points,
/// the segments between neighbours along the line.
#[cfg(feature = "bake")]
pub fn triangulate(points: &[Vec2]) -> Result<Triangulation, String> {
    use spade::{DelaunayTriangulation, Point2, Triangulation};

    for (i, a) in points.iter().enumerate() {
        if points[..i].iter().any(|b| a.distance(*b) < 1e-5) {
            return Err(format!("two samples at the same position ({}, {})", a.x, a.y));
        }
    }
    let idx = |i: usize| u16::try_from(i).expect("sample count fits u16");
    if points.len() < 2 {
        return Ok((Vec::new(), Vec::new()));
    }
    let mut t: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
    let mut handle_to_sample = vec![0usize; points.len()];
    for (i, p) in points.iter().enumerate() {
        let h = t.insert(Point2::new(f64::from(p.x), f64::from(p.y))).map_err(|e| format!("{e:?}"))?;
        handle_to_sample[h.index()] = i;
    }
    let mut triangles: Vec<[u16; 3]> = t
        .inner_faces()
        .map(|f| {
            let [a, b, c] = f.vertices();
            [a, b, c].map(|v| idx(handle_to_sample[v.fix().index()]))
        })
        .collect();
    if triangles.is_empty() {
        // Collinear: order along the line.
        let dir = (points[1] - points[0]).normalize_or(Vec2::X);
        let mut order: Vec<usize> = (0..points.len()).collect();
        order.sort_by(|&a, &b| points[a].dot(dir).total_cmp(&points[b].dot(dir)).then(a.cmp(&b)));
        let edges = order.windows(2).map(|w| [idx(w[0]), idx(w[1])]).collect();
        return Ok((Vec::new(), edges));
    }
    // Canonical order, independent of the triangulator's internals.
    for tri in &mut triangles {
        let m = (0..3).min_by_key(|&k| tri[k]).expect("three vertices");
        tri.rotate_left(m);
    }
    triangles.sort_unstable();
    let mut edges: Vec<[u16; 2]> =
        t.convex_hull().map(|e| [idx(handle_to_sample[e.from().fix().index()]), idx(handle_to_sample[e.to().fix().index()])]).collect();
    for e in &mut edges {
        e.sort_unstable();
    }
    edges.sort_unstable();
    Ok((triangles, edges))
}
