//! Columnar jointing (rockgen.md RG-4 "columnar"): a cluster of prism columns, the Voronoi cells of a jittered
//! hex lattice inside an elliptic footprint (cooling basalt cracks into mostly 5-6 sided columns, Goehring &
//! Morris 2009), each topped by its own tilted cross-joint plane, separated by open joints down to a fused base.
//! Columns stand along +Z (the rock's short axis, which the viewer and the scatter put up).
//!
//! Every column is two convex [`Piece`]s: the column above the fused level, its sides inset by half the joint
//! width toward its neighbours (never on the cluster's outside), and its share of the base below. Neighbours'
//! shared faces are the same bisector with exactly negated values and are never rounded ([`Piece::hard`]), and the
//! planes all columns share (the floor; the boulder's chips) cut the union once ([`crate::sdf::Solid::with_cap`]),
//! so the union has no seam, no notch and no sliver.

use glam::{Vec2, Vec3};

use crate::noise::Stream;
use crate::sdf::{Piece, Plane};

/// Neighbour seeds farther than this many spacings never bound a jittered hex cell (Voronoi neighbours of a
/// lattice jittered by < 0.5 spacing are within 2 spacings).
// tunables: const (geometry: neighbour search radius of a jittered lattice)
const NEIGHBOUR_SPACINGS: f32 = 2.5;
/// A joint narrower than this many cells can't be resolved by the grid: the columns are fused instead.
// tunables: const (sampling limit of the mesher)
const MIN_GROOVE_CELLS: f32 = 2.5;

/// What the cluster looks like (unit frame; the boulder's tunables in [`crate::BoulderParams`]).
pub(crate) struct ColumnSpec {
    pub count: u32,
    pub jitter: f32,
    pub groove: f32,
    pub step: f32,
    pub tilt_deg: f32,
    pub fuse: f32,
}

/// The cluster's pieces, the planes that cut the whole cluster (its floor: a [`crate::sdf::Solid`] cap, rounded
/// once across the columns), and the half-extent of the box that holds them. `half` = footprint semi-axes (x, y)
/// and half height (z).
pub(crate) fn column_cluster(spec: &ColumnSpec, half: Vec3, voxel: f32, rng: &mut Stream) -> (Vec<Piece>, Vec<Plane>, Vec3) {
    let count = spec.count.max(1);
    // Spacing so ~count hex cells fill the footprint ellipse: area π·a·b = count·(√3/2)·s².
    #[allow(clippy::cast_precision_loss)]
    let spacing = (std::f32::consts::PI * half.x * half.y / (count as f32 * 0.5 * 3f32.sqrt())).sqrt();
    let row = 0.5 * 3f32.sqrt() * spacing;
    let phase = Vec2::new(rng.unit(), rng.unit()) * spacing;
    let margin = NEIGHBOUR_SPACINGS * spacing;
    #[allow(clippy::cast_possible_truncation)]
    let (nx, ny) = (((half.x + margin) / spacing).ceil() as i32, ((half.y + margin) / row).ceil() as i32);
    let mut seeds: Vec<(Vec2, bool)> = Vec::new();
    for j in -ny..=ny {
        for i in -nx..=nx {
            #[allow(clippy::cast_precision_loss)]
            let lattice = Vec2::new((i as f32 + 0.5 * (j.rem_euclid(2) as f32)) * spacing, j as f32 * row) + phase;
            let angle = rng.unit() * std::f32::consts::TAU;
            let r = spec.jitter * spacing * rng.unit().sqrt();
            let seed = lattice + Vec2::new(angle.cos(), angle.sin()) * r;
            let inside = (seed / half.truncate()).length_squared() <= 1.0;
            seeds.push((seed, inside));
        }
    }
    if !seeds.iter().any(|s| s.1) {
        // A footprint too small for a lattice point: the nearest seed is the column.
        if let Some(k) = (0..seeds.len()).min_by(|&a, &b| seeds[a].0.length().total_cmp(&seeds[b].0.length())) {
            seeds[k].1 = true;
        }
    }

    let groove = spec.groove * spacing;
    let grooved = groove >= MIN_GROOVE_CELLS * voxel;
    let height = 2.0 * half.z;
    let columns: Vec<usize> = (0..seeds.len()).filter(|&k| seeds[k].1).collect();
    let tops: Vec<f32> = columns.iter().map(|_| half.z - spec.step * height * rng.unit()).collect();
    let lowest = tops.iter().copied().fold(half.z, f32::min);
    let fuse_z = -half.z + spec.fuse * (lowest + half.z);
    let bottom = Plane { n: Vec3::NEG_Z, d: half.z };
    let max_tilt = spec.tilt_deg.to_radians();

    let mut pieces = Vec::new();
    let mut reach = Vec3::new(0.0, 0.0, half.z);
    for (&k, &top) in columns.iter().zip(&tops) {
        let me = seeds[k].0;
        // Voronoi cell: clip a square by every near seed's bisector; keep the bisectors that made an edge.
        let mut poly = vec![me + Vec2::new(-margin, -margin), me + Vec2::new(margin, -margin), me + Vec2::new(margin, margin), me + Vec2::new(-margin, margin)];
        let mut bisectors = Vec::new();
        for (j, &(other, is_column)) in seeds.iter().enumerate() {
            if j == k || (other - me).length() > margin {
                continue;
            }
            let n = (other - me).normalize();
            let d = n.dot((me + other) * 0.5);
            poly = clip(&poly, n, d);
            bisectors.push((n, d, is_column));
        }
        let mut sides = Vec::new();
        for &(n, d, is_column) in &bisectors {
            if poly.iter().any(|q| (n.dot(*q) - d).abs() < 1e-5) {
                sides.push((n, d, is_column));
            }
        }
        for q in &poly {
            reach.x = reach.x.max(q.x.abs());
            reach.y = reach.y.max(q.y.abs());
        }
        #[allow(clippy::cast_precision_loss)] // a handful of corners
        let centroid = poly.iter().copied().sum::<Vec2>() / poly.len().max(1) as f32;
        // Cross-joint top: tilted by up to max_tilt toward a random direction, through the column's centre.
        let tilt = max_tilt * rng.unit();
        let azimuth = rng.unit() * std::f32::consts::TAU;
        let top_n = Vec3::new(tilt.sin() * azimuth.cos(), tilt.sin() * azimuth.sin(), tilt.cos());
        let top_plane = Plane { n: top_n, d: top_n.dot(centroid.extend(top)) };
        let radius = poly.iter().map(|q| (*q - centroid).length()).fold(0.0, f32::max);
        reach.z = reach.z.max(top + tilt.tan() * radius);
        let side = |n: Vec2, d: f32, offset: f32| Plane { n: n.extend(0.0), d: d + offset };
        let mut column = Piece::default();
        let mut base = Piece::default();
        for &(n, d, is_column) in &sides {
            match (is_column, grooved) {
                // The cluster's outside: a real face of both the column and its base.
                (false, _) => {
                    column.planes.push(side(n, d, 0.0));
                    base.planes.push(side(n, d, 0.0));
                }
                // An open joint: the column's face is inset; the base below shares the bisector (hard).
                (true, true) => {
                    column.planes.push(side(n, d, -0.5 * groove));
                    base.hard.push(side(n, d, 0.0));
                }
                // Fused neighbours: the shared bisector is internal (hard).
                (true, false) => column.hard.push(side(n, d, 0.0)),
            }
        }
        column.planes.push(top_plane);
        pieces.push(column);
        if grooved {
            // The joints' floor is one plane shared by every base: hard (rounded per base, it would leave a slit
            // at each joint's mouth).
            base.hard.push(Plane { n: Vec3::Z, d: fuse_z.min(top) });
            pieces.push(base);
        }
    }
    (pieces, vec![bottom], reach)
}

/// Sutherland-Hodgman: the part of convex polygon `poly` with `n·q ≤ d`.
fn clip(poly: &[Vec2], n: Vec2, d: f32) -> Vec<Vec2> {
    let mut out = Vec::with_capacity(poly.len() + 1);
    for (i, &a) in poly.iter().enumerate() {
        let b = poly[(i + 1) % poly.len()];
        let (da, db) = (n.dot(a) - d, n.dot(b) - d);
        if da <= 0.0 {
            out.push(a);
        }
        if (da < 0.0) != (db < 0.0) && (da - db).abs() > 0.0 {
            out.push(a + (b - a) * (da / (da - db)));
        }
    }
    out
}
