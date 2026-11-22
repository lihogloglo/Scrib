//! Convex collision hulls for library rocks (rockgen.md "Collision (RG-2)", feature `collision`): the standard
//! simple collision for a convex-ish prop (UE "auto convex", `PhysX` convex cooking), built with `parry3d`'s quickhull.
//!
//! Points are LOD0's vertices (every coarser LOD's vertices are a subset of LOD0's, so a hull of a low LOD could
//! leave LOD0 vertices outside). The exact hull of LOD0 has hundreds of vertices; it is reduced the way quickhull
//! grows a hull, with a stop tolerance (qhull's `-C`/`-W` precision options, `PhysX`'s vertex limit): start from an
//! extreme simplex, repeatedly add the exact hull's vertex farthest outside the current hull, stop once every vertex
//! is within `tolerance_m` (Euclidean, by `parry3d`'s point projection) or the vertex cap is reached. The result is
//! a subset of LOD0's vertices, so it never sticks out of the visual mesh; it can sit up to `error_m` inside it.
//! Checking only the exact hull's vertices is enough: distance outside a convex set is a convex function, so its
//! maximum over LOD0 (inside the exact hull) is reached at one of the exact hull's vertices.

use parry3d::math::Vector;
use parry3d::query::PointQuery;
use parry3d::shape::ConvexPolyhedron;
use parry3d::transformation::try_convex_hull;

use crate::RockMesh;
use crate::library::{Hull, HullSettings};

/// A simplex this much thinner than the rock (relative to its exact hull's extent) is degenerate.
// tunables: const (numerical epsilon)
const DEGENERATE: f32 = 1e-6;

impl Hull {
    /// The `parry3d` shape (coplanar triangles merged into polygonal faces); `None` if the stored mesh is degenerate.
    pub fn to_parry(&self) -> Option<ConvexPolyhedron> {
        let points = self.points.iter().map(|p| Vector::from_array(*p)).collect();
        ConvexPolyhedron::from_convex_mesh(points, &self.triangles)
    }
}

/// The reduced convex hull of `mesh`'s vertices (see the module docs); `None` for a flat or empty mesh.
pub(crate) fn build_hull(mesh: &RockMesh, settings: &HullSettings, size_m: f32) -> Option<Hull> {
    let points: Vec<Vector> = mesh.positions.iter().map(|p| Vector::from_array(*p)).collect();
    let (exact, exact_tris) = try_convex_hull(&points).ok()?;
    let tolerance = settings.tolerance_m(size_m);
    let cap = usize::try_from(settings.max_vertices).unwrap_or(usize::MAX).max(4);
    let Some(simplex) = initial_simplex(&exact) else {
        return Some(as_hull(&exact, exact_tris, 0.0));
    };
    let mut chosen: Vec<usize> = simplex.to_vec();
    let mut taken = vec![false; exact.len()];
    for &i in &chosen {
        taken[i] = true;
    }
    loop {
        let subset: Vec<Vector> = chosen.iter().map(|&i| exact[i]).collect();
        let Ok((hull, tris)) = try_convex_hull(&subset) else {
            // Quickhull gave up on a near-degenerate subset: the exact hull is always valid.
            return Some(as_hull(&exact, exact_tris, 0.0));
        };
        // Cheap pass: farthest outside a face plane (a lower bound of the Euclidean distance).
        let planes = face_planes(&hull, &tris);
        let farthest = argmax(&exact, &taken, |p| planes.iter().map(|(n, d)| n.dot(p) - d).fold(f32::MIN, f32::max));
        if let Some((i, d)) = farthest
            && d > tolerance
            && chosen.len() < cap
        {
            chosen.push(i);
            taken[i] = true;
            continue;
        }
        // Exact pass: Euclidean distance outside (0 inside).
        let shape = ConvexPolyhedron::from_convex_mesh(hull.clone(), &tris)?;
        let worst = argmax(&exact, &taken, |p| shape.distance_to_local_point(p, true));
        match worst {
            Some((i, d)) if d > tolerance && chosen.len() < cap => {
                chosen.push(i);
                taken[i] = true;
            }
            _ => return Some(as_hull(&hull, tris, worst.map_or(0.0, |(_, d)| d.max(0.0)))),
        }
    }
}

fn as_hull(points: &[Vector], triangles: Vec<[u32; 3]>, error_m: f32) -> Hull {
    Hull { points: points.iter().map(Vector::to_array).collect(), triangles, error_m }
}

/// Index and value of the largest `f` over the points not yet taken (first on ties: deterministic).
fn argmax(points: &[Vector], taken: &[bool], f: impl Fn(Vector) -> f32) -> Option<(usize, f32)> {
    let mut best: Option<(usize, f32)> = None;
    for (i, &p) in points.iter().enumerate() {
        if taken[i] {
            continue;
        }
        let v = f(p);
        if best.is_none_or(|(_, b)| v > b) {
            best = Some((i, v));
        }
    }
    best
}

/// Outward unit normal and offset of every triangle (`n·p − d` > 0 outside).
fn face_planes(points: &[Vector], tris: &[[u32; 3]]) -> Vec<(Vector, f32)> {
    #[allow(clippy::cast_precision_loss)] // a hull has at most a few hundred points
    let centroid = points.iter().copied().sum::<Vector>() / points.len().max(1) as f32;
    tris.iter()
        .filter_map(|t| {
            let [a, b, c] = t.map(|i| points[i as usize]);
            let n = (b - a).cross(c - a).try_normalize()?;
            let n = if n.dot(centroid - a) > 0.0 { -n } else { n };
            Some((n, n.dot(a)))
        })
        .collect()
}

/// Quickhull's starting tetrahedron: the lowest-x point, the point farthest from it, the one farthest from their
/// line, the one farthest from their plane. `None` if the points are (near) flat.
fn initial_simplex(p: &[Vector]) -> Option<[usize; 4]> {
    let none = vec![false; p.len()];
    let (i0, _) = argmax(p, &none, |q| -q.x)?;
    let a = p[i0];
    let (i1, len) = argmax(p, &none, |q| q.distance(a))?;
    let eps = DEGENERATE * len.max(f32::MIN_POSITIVE);
    if len <= eps {
        return None;
    }
    let axis = (p[i1] - a) / len;
    let (i2, off) = argmax(p, &none, |q| (q - a).cross(axis).length())?;
    if off <= eps {
        return None;
    }
    let n = (p[i1] - a).cross(p[i2] - a).try_normalize()?;
    let (i3, h) = argmax(p, &none, |q| (q - a).dot(n).abs())?;
    if h <= eps {
        return None;
    }
    Some([i0, i1, i2, i3])
}
