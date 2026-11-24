//! Lake and river mesh builders (`body_mesh`): exact geometry checks.

use glam::DVec2;

use crate::body_mesh::{
    BodyMesh, LakeMeshParams, RiverMeshParams, lake_mesh, river_mesh, river_samples,
};
use crate::records::RiverPoint;

fn d(x: f64, z: f64) -> DVec2 {
    DVec2::new(x, z)
}

fn tri_xz(m: &BodyMesh, t: usize) -> [DVec2; 3] {
    std::array::from_fn(|k| {
        let p = m.positions[m.indices[t * 3 + k] as usize];
        d(f64::from(p[0]), f64::from(p[2]))
    })
}

/// Signed area seen from above (positive = faces +Y), m².
fn up_area(p: [DVec2; 3]) -> f64 {
    let (u, v) = (p[1] - p[0], p[2] - p[0]);
    0.5 * (u.y * v.x - u.x * v.y)
}

fn total_area(m: &BodyMesh) -> f64 {
    (0..m.triangle_count()).map(|t| up_area(tri_xz(m, t))).sum()
}

fn ring_area(r: &[DVec2]) -> f64 {
    ((0..r.len())
        .map(|k| r[k].perp_dot(r[(k + 1) % r.len()]))
        .sum::<f64>()
        * 0.5)
        .abs()
}

fn assert_valid(m: &BodyMesh) {
    let n = m.positions.len();
    assert_eq!(m.normals.len(), n);
    assert_eq!(m.uvs.len(), n);
    assert_eq!(m.flow.len(), n);
    assert_eq!(m.indices.len() % 3, 0);
    assert!(m.indices.iter().all(|&i| (i as usize) < n));
    for t in 0..m.triangle_count() {
        assert!(
            up_area(tri_xz(m, t)) > 0.0,
            "triangle {t} faces down or is degenerate"
        );
    }
    for p in &m.positions {
        assert!(p.iter().all(|v| v.is_finite()));
        let (lo, hi) = (m.aabb.min.to_array(), m.aabb.max.to_array());
        assert!((0..3).all(|k| lo[k] <= p[k] && p[k] <= hi[k]));
    }
}

fn square(c: DVec2, half: f64) -> Vec<DVec2> {
    vec![
        c + d(-half, -half),
        c + d(half, -half),
        c + d(half, half),
        c + d(-half, half),
    ]
}

#[test]
fn lake_square_with_hole() {
    let origin = d(1000.0, -2000.0);
    let outline = square(origin + d(50.0, 50.0), 50.0);
    for hole in [
        square(origin + d(50.0, 50.0), 10.0),
        square(origin + d(50.0, 50.0), 10.0)
            .into_iter()
            .rev()
            .collect(),
    ] {
        let level = 12.5;
        let m = lake_mesh(&outline, &[hole], level, origin, &LakeMeshParams::default());
        assert_valid(&m);
        assert!(
            (total_area(&m) - 9600.0).abs() < 1e-6 * 9600.0,
            "area {}",
            total_area(&m)
        );
        for t in 0..m.triangle_count() {
            let p = tri_xz(&m, t);
            let c = (p[0] + p[1] + p[2]) / 3.0;
            assert!(
                !(c.x > 40.0 && c.x < 60.0 && c.y > 40.0 && c.y < 60.0),
                "triangle {t} in the hole at {c}"
            );
            assert!(c.x > 0.0 && c.x < 100.0 && c.y > 0.0 && c.y < 100.0);
        }
        #[allow(clippy::cast_possible_truncation)]
        let y = level as f32;
        assert!(m.positions.iter().all(|p| (p[1] - y).abs() < f32::EPSILON));
        assert!(
            m.normals.iter().all(
                |n| n[0].abs() + n[2].abs() < f32::EPSILON && (n[1] - 1.0).abs() < f32::EPSILON
            )
        );
        assert!(
            m.flow
                .iter()
                .all(|f| f[0].abs() + f[1].abs() < f32::EPSILON)
        );
        // Refinement bounded the triangle size.
        assert!((0..m.triangle_count()).all(|t| up_area(tri_xz(&m, t)) <= 1024.0 + 1e-6));
    }
}

#[test]
fn lake_concave_u_shape_area() {
    // A U: 100 × 80 with a 60 × 60 notch from the top, clockwise, with a closing duplicate.
    let u = vec![
        d(0.0, 0.0),
        d(0.0, 80.0),
        d(20.0, 80.0),
        d(20.0, 20.0),
        d(80.0, 20.0),
        d(80.0, 80.0),
        d(100.0, 80.0),
        d(100.0, 0.0),
        d(0.0, 0.0),
    ];
    let want = ring_area(&u[..8]);
    assert!((want - 4400.0).abs() < 1e-9);
    for params in [
        LakeMeshParams::default(),
        LakeMeshParams {
            max_steiner_vertices: 0,
            ..LakeMeshParams::default()
        },
    ] {
        let m = lake_mesh(&u, &[], 3.0, DVec2::ZERO, &params);
        assert_valid(&m);
        assert!(
            (total_area(&m) - want).abs() <= 1e-3 * want,
            "area {} vs {want}",
            total_area(&m)
        );
        for t in 0..m.triangle_count() {
            let p = tri_xz(&m, t);
            let c = (p[0] + p[1] + p[2]) / 3.0;
            assert!(
                !(c.x > 20.0 && c.x < 80.0 && c.y > 20.0),
                "triangle {t} in the notch at {c}"
            );
        }
    }
    // The bare CDT keeps exactly the outline's vertices: n − 2 triangles.
    let bare = lake_mesh(
        &u,
        &[],
        3.0,
        DVec2::ZERO,
        &LakeMeshParams {
            max_steiner_vertices: 0,
            ..LakeMeshParams::default()
        },
    );
    assert_eq!(bare.triangle_count(), 6);
}

#[test]
fn lake_many_vertices_with_island() {
    // A marching-squares-like outline: 4000 vertices, a wobbly concave circle with a round island.
    let ring = |n: usize, r: f64, wobble: f64| -> Vec<DVec2> {
        (0..n)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let a = std::f64::consts::TAU * k as f64 / n as f64;
                let rr = r * (1.0 + wobble * (7.0 * a).sin());
                d(rr * a.cos(), rr * a.sin())
            })
            .collect()
    };
    let outline = ring(4000, 800.0, 0.2);
    let island = ring(400, 100.0, 0.0);
    let m = lake_mesh(
        &outline,
        std::slice::from_ref(&island),
        0.0,
        DVec2::ZERO,
        &LakeMeshParams::default(),
    );
    assert_valid(&m);
    let want = ring_area(&outline) - ring_area(&island);
    assert!((total_area(&m) - want).abs() <= 1e-3 * want);
}

#[test]
fn lake_degenerate_inputs_are_empty() {
    let p = LakeMeshParams::default();
    let nan = d(f64::NAN, 0.0);
    for outline in [
        vec![],
        vec![d(0.0, 0.0), d(1.0, 0.0)],
        vec![d(0.0, 0.0), d(1.0, 0.0), d(2.0, 0.0)],
        vec![d(5.0, 5.0); 10],
        vec![nan, nan, nan, nan],
        vec![nan, d(0.0, 0.0), d(f64::INFINITY, 1.0), d(1.0, 0.0)],
    ] {
        let m = lake_mesh(&outline, &[], 0.0, DVec2::ZERO, &p);
        assert!(m.is_empty() && m.positions.is_empty(), "{outline:?}");
        assert_eq!(m, BodyMesh::default());
    }
    // A degenerate hole is skipped; a bow tie (self-crossing) neither panics nor faces down.
    let m = lake_mesh(
        &square(DVec2::ZERO, 10.0),
        &[vec![d(0.0, 0.0)], vec![]],
        0.0,
        DVec2::ZERO,
        &p,
    );
    assert!((total_area(&m) - 400.0).abs() < 1e-6);
    let bow = vec![d(0.0, 0.0), d(10.0, 10.0), d(10.0, 0.0), d(0.0, 10.0)];
    assert_valid(&lake_mesh(&bow, &[], 0.0, DVec2::ZERO, &p));
}

/// A river with a 90° bend: east 40 m, a quarter circle of radius 60 m, then north 40 m.
fn bend() -> Vec<RiverPoint> {
    let mut pts = vec![d(0.0, 0.0), d(40.0, 0.0)];
    for deg in [30.0_f64, 60.0, 90.0] {
        let a = deg.to_radians();
        pts.push(d(40.0 + 60.0 * a.sin(), 60.0 - 60.0 * a.cos()));
    }
    pts.push(d(100.0, 100.0));
    let widths = [10.0, 12.0, 14.0, 16.0, 12.0, 8.0];
    let heights = [10.0, 9.5, 9.6, 8.0, 7.9, 7.5]; // one rise: must come out flat, never up
    let speeds = [0.5, 1.0, 1.5, 1.0, 0.9, 0.8];
    (0..6)
        .map(|i| RiverPoint {
            center: pts[i] + d(5000.0, 300.0),
            width: widths[i],
            height: heights[i],
            flow_speed: speeds[i],
        })
        .collect()
}

#[test]
fn river_bend_faces_up_and_follows_input() {
    let spline = bend();
    let origin = d(5000.0, 300.0);
    let params = RiverMeshParams::default();
    let samples = river_samples(&spline, &params, origin);
    let m = river_mesh(&spline, &params, origin);
    assert_valid(&m);
    let cols = params.cross_samples as usize;
    assert_eq!(m.positions.len(), samples.len() * cols);
    // No triangle dropped: nothing folded or degenerate at this bend.
    assert_eq!(m.triangle_count(), (samples.len() - 1) * (cols - 1) * 2);

    // Every control point is a sample, with its input width.
    let controls: Vec<_> = samples
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.control.map(|c| (i, c)))
        .collect();
    assert_eq!(
        controls.iter().map(|c| c.1).collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 4, 5]
    );
    for &(row, c) in &controls {
        let s = &samples[row];
        assert!((s.center - (spline[c].center - origin)).length() < 1e-9);
        assert!((s.width - f64::from(spline[c].width)).abs() < 1e-9);
        let first = m.positions[row * cols];
        let last = m.positions[row * cols + cols - 1];
        let span = d(f64::from(last[0] - first[0]), f64::from(last[2] - first[2])).length();
        let want = f64::from(spline[c].width) + 2.0 * params.bank_extend_m;
        assert!(
            (span - want).abs() < 1e-3,
            "row {row}: span {span} vs {want}"
        );
    }
    // Spacing: even along the arc, no step longer than the spacing (chords of a 32-step table).
    for w in samples.windows(2) {
        let step = w[0].center.distance(w[1].center);
        assert!(
            step <= params.spacing_m * 1.01 && step > params.spacing_m * 0.5,
            "step {step}"
        );
    }

    // Heights non-increasing along the strip, flat across; the rise at point 2 is flattened.
    for w in samples.windows(2) {
        assert!(w[1].height <= w[0].height);
    }
    assert!((samples[controls[2].0].height - 9.5).abs() < 1e-12);
    for (i, s) in samples.iter().enumerate() {
        for j in 0..cols {
            #[allow(clippy::cast_possible_truncation)]
            let y = s.height as f32;
            assert!((m.positions[i * cols + j][1] - y).abs() < f32::EPSILON);
        }
    }
    // Flow points downstream (towards the next sample), with the sample's speed.
    for i in 0..samples.len() - 1 {
        let down = samples[i + 1].center - samples[i].center;
        for j in 0..cols {
            let f = m.flow[i * cols + j];
            let f = d(f64::from(f[0]), f64::from(f[1]));
            assert!(f.dot(down) > 0.0, "sample {i} col {j}");
            assert!((f.length() - samples[i].flow_speed).abs() < 1e-4);
        }
    }
    // Normals point up.
    assert!(m.normals.iter().all(|n| n[1] > 0.9));
}

#[test]
fn river_hairpin_never_folds() {
    // A U-turn tighter than the river's half width: the inner side must shorten, not fold.
    let pts = [
        (0.0, 0.0),
        (30.0, 0.0),
        (34.0, 3.0),
        (30.0, 6.0),
        (0.0, 6.0),
    ];
    let spline: Vec<_> = pts
        .iter()
        .enumerate()
        .map(|(i, &(x, z))| RiverPoint {
            center: d(x, z),
            width: 20.0,
            #[allow(clippy::cast_precision_loss)]
            height: 5.0 - i as f64,
            flow_speed: 1.0,
        })
        .collect();
    for cross in [2, 3, 4, 9] {
        let params = RiverMeshParams {
            cross_samples: cross,
            spacing_m: 1.0,
            ..RiverMeshParams::default()
        };
        let m = river_mesh(&spline, &params, DVec2::ZERO);
        assert!(!m.is_empty());
        assert_valid(&m);
    }
}

#[test]
fn river_degenerate_inputs_are_empty() {
    let p = |x: f64, w: f32| RiverPoint {
        center: d(x, 0.0),
        width: w,
        height: 1.0,
        flow_speed: 1.0,
    };
    let params = RiverMeshParams::default();
    for spline in [
        vec![],
        vec![p(0.0, 5.0)],
        vec![p(3.0, 5.0); 4],
        vec![p(f64::NAN, 5.0), p(1.0, 5.0)],
        vec![p(0.0, f32::NAN), p(1.0, f32::INFINITY)],
    ] {
        assert_eq!(
            river_mesh(&spline, &params, DVec2::ZERO),
            BodyMesh::default(),
            "{spline:?}"
        );
        assert!(river_samples(&spline, &params, DVec2::ZERO).is_empty());
    }
    // Zero spacing, one cross sample, negative widths: clamped, valid, finite.
    let odd = RiverMeshParams {
        spacing_m: 0.0,
        cross_samples: 0,
        bank_extend_m: -1.0,
    };
    let m = river_mesh(&[p(0.0, -4.0), p(2.0, 3.0)], &odd, DVec2::ZERO);
    assert_valid(&m);
    // Zero width and no bank: no area, so no triangles, but no panic.
    let flat = RiverMeshParams {
        bank_extend_m: 0.0,
        ..RiverMeshParams::default()
    };
    assert!(river_mesh(&[p(0.0, 0.0), p(10.0, 0.0)], &flat, DVec2::ZERO).is_empty());
}

#[test]
fn builders_are_deterministic() {
    let u = vec![
        d(0.0, 0.0),
        d(0.0, 80.0),
        d(20.0, 80.0),
        d(20.0, 20.0),
        d(80.0, 20.0),
        d(80.0, 80.0),
        d(100.0, 80.0),
        d(100.0, 0.0),
    ];
    let hole = square(d(50.0, 10.0), 4.0);
    let params = LakeMeshParams {
        max_triangle_area_m2: 50.0,
        ..LakeMeshParams::default()
    };
    let a = lake_mesh(&u, std::slice::from_ref(&hole), 1.0, d(3.0, 4.0), &params);
    let b = lake_mesh(&u, std::slice::from_ref(&hole), 1.0, d(3.0, 4.0), &params);
    assert!(a.triangle_count() > 100);
    assert_eq!(a, b);
    let r = RiverMeshParams::default();
    assert_eq!(
        river_mesh(&bend(), &r, DVec2::ZERO),
        river_mesh(&bend(), &r, DVec2::ZERO)
    );
}
