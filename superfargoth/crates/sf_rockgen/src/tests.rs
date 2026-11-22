//! RG-1 / RG-4 tests (rockgen.md "Build plan"): exact assertions on determinism and mesh validity for every class;
//! the look is the owner's (RG-3 / RG-4 visual checks). Measurements print from the ignored test.

use glam::{Quat, Vec3};

use crate::boulder::tunables;
use crate::mesher::{MeshSettings, mesh_solid};
use crate::sdf::{Piece, Plane, Solid, Weathering};
use crate::{BoulderParams, CLASSES, Family, RockLods, RockMesh, generate_boulder};

const SEEDS: std::ops::Range<u64> = 0..24;

/// Every preset at a test size (the chipped class at 12 m, as in RG-1, so size-scaled resolution is covered).
fn classes() -> Vec<(&'static str, Family, BoulderParams)> {
    let d = BoulderParams::default();
    CLASSES
        .iter()
        .map(|c| {
            let size_m = if c.name == "chipped" || c.name == "columnar" { 12.0 } else { d.size_m };
            (c.name, c.family, c.params(BoulderParams { size_m, ..d }))
        })
        .collect()
}

/// Every class × seed, generated once for all the validity tests (in parallel: one thread per class).
pub(crate) fn rocks() -> &'static [(&'static str, Family, u64, RockLods)] {
    static ROCKS: std::sync::OnceLock<Vec<(&'static str, Family, u64, RockLods)>> = std::sync::OnceLock::new();
    ROCKS.get_or_init(|| {
        std::thread::scope(|scope| {
            let jobs: Vec<_> = classes()
                .into_iter()
                .map(|(name, family, params)| {
                    scope.spawn(move || SEEDS.map(|seed| (name, family, seed, generate_boulder(&params, seed))).collect::<Vec<_>>())
                })
                .collect();
            jobs.into_iter().flat_map(|j| j.join().unwrap_or_default()).collect()
        })
    })
}

fn each(mut f: impl FnMut(&str, Family, u64, &RockLods)) {
    for (name, family, seed, rock) in rocks() {
        f(name, *family, *seed, rock);
    }
}

#[test]
fn same_seed_same_mesh() {
    for (_, _, params) in classes() {
        for seed in [0, 7, u64::MAX] {
            let a = generate_boulder(&params, seed);
            let b = generate_boulder(&params, seed);
            assert_eq!(a.hash, b.hash);
            assert_eq!(a, b);
        }
    }
    let d = BoulderParams::default();
    assert_ne!(generate_boulder(&d, 1).hash, generate_boulder(&d, 2).hash, "seeds must matter");
}

#[test]
fn every_lod_is_closed() {
    each(|name, _, seed, rock| {
        for (level, lod) in rock.lods.iter().enumerate() {
            assert!(lod.is_closed(), "{name} seed {seed} LOD{level} is not closed");
            assert!(lod.signed_volume() > 0.0, "{name} seed {seed} LOD{level} faces inward");
        }
    });
}

#[test]
fn no_nan_and_unit_normals() {
    each(|name, _, seed, rock| {
        for lod in &rock.lods {
            assert_eq!(lod.positions.len(), lod.normals.len());
            assert!(lod.indices.iter().all(|&i| (i as usize) < lod.positions.len()));
            for (p, n) in lod.positions.iter().zip(&lod.normals) {
                assert!(p.iter().chain(n).all(|x| x.is_finite()), "{name} seed {seed}: non-finite vertex");
                let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                assert!((len - 1.0).abs() < 1e-3, "{name} seed {seed}: normal length {len}");
            }
        }
    });
}

#[test]
fn bounds_within_declared_size() {
    let sizes = classes();
    each(|name, _, seed, rock| {
        let size = sizes.iter().find(|c| c.0 == name).map_or(0.0, |c| c.2.size_m);
        let e = rock.bounds.extent();
        let longest = e[0].max(e[1]).max(e[2]);
        assert!((longest - size).abs() <= size * 1e-4, "{name} seed {seed}: longest side {longest}, size {size}");
        for k in 0..3 {
            assert!(rock.bounds.min[k] >= -0.5 * size * (1.0 + 1e-4), "{name} seed {seed}: outside the size box");
            assert!(rock.bounds.max[k] <= 0.5 * size * (1.0 + 1e-4), "{name} seed {seed}: outside the size box");
        }
        for lod in &rock.lods[1..] {
            let b = lod.bounds();
            assert!((0..3).all(|k| b.min[k] >= rock.bounds.min[k] && b.max[k] <= rock.bounds.max[k]));
        }
    });
}

/// Triangles strictly decrease along every chain; eroded rocks always get 3+ levels. A hard-edged rock's LOD0 may
/// already be a handful of flat faces, so its chain may stop early (rockgen.md "LOD chain").
#[test]
fn lod_triangles_strictly_decrease() {
    let levels = tunables::LOD_LEVELS.default_value() as usize;
    each(|name, family, seed, rock| {
        assert!(!rock.lods.is_empty(), "{name} seed {seed}: no LOD");
        if family == Family::Eroded {
            assert!(rock.lods.len() >= levels.min(3), "{name} seed {seed}: only {} LODs", rock.lods.len());
        }
        for pair in rock.lods.windows(2) {
            assert!(
                pair[1].triangle_count() < pair[0].triangle_count(),
                "{name} seed {seed}: {} then {} triangles",
                pair[0].triangle_count(),
                pair[1].triangle_count()
            );
        }
    });
}

#[test]
#[allow(clippy::float_cmp)] // clamping must give the declared bounds exactly
fn out_of_range_params_are_clamped() {
    let wild = BoulderParams {
        size_m: f32::NAN,
        aspect: [7.0, -1.0],
        joint_sets: 9,
        resolution: 100_000,
        lod_levels: 0,
        columns: 1000,
        crease_deg: 400.0,
        ..BoulderParams::default()
    };
    let c = wild.clamped();
    assert_eq!(c.size_m, tunables::SIZE.default_value());
    assert_eq!(c.aspect, [1.0, 0.05]);
    assert_eq!(c.joint_sets, 3);
    assert_eq!(c.resolution, 128);
    assert_eq!(c.lod_levels, 1);
    assert_eq!(c.columns, 40);
    assert_eq!(c.crease_deg, 180.0);
    let one_level = BoulderParams { lod_levels: 0, ..BoulderParams::default() };
    assert_eq!(generate_boulder(&one_level, 3).lods.len(), 1);
}

#[test]
fn resolution_grows_with_size() {
    let d = BoulderParams::default();
    assert_eq!(BoulderParams { size_m: 0.5, ..d }.effective_resolution(), d.resolution);
    assert_eq!(BoulderParams { size_m: d.resolution_ref_m, ..d }.effective_resolution(), d.resolution);
    let big = BoulderParams { size_m: 8.0, ..d }.effective_resolution();
    assert_eq!(big, d.resolution * 2, "(8 / 2)^0.5 = 2");
    assert_eq!(BoulderParams { size_m: 40.0, resolution_size_exp: 1.0, ..d }.effective_resolution(), d.resolution_max);
}

/// Crease split only splits normals: positions bit-equal, and welding the copies gives back the source indices.
#[test]
fn crease_split_keeps_the_surface_watertight() {
    for (name, _, params) in classes() {
        let smooth = generate_boulder(&BoulderParams { crease_deg: 180.0, ..params }, 5);
        for (level, lod) in smooth.lods.iter().enumerate() {
            let split = lod.split_creases(35.0);
            assert!(split.positions.len() >= lod.positions.len());
            assert_eq!(split.indices.len(), lod.indices.len());
            // Each split index points at a vertex at the source vertex's exact position.
            for (&a, &b) in split.indices.iter().zip(&lod.indices) {
                let (pa, pb) = (split.positions[a as usize], lod.positions[b as usize]);
                assert_eq!(pa.map(f32::to_bits), pb.map(f32::to_bits), "{name} LOD{level}: a split moved a vertex");
            }
            assert_eq!(split.welded_indices(), lod.welded_indices(), "{name} LOD{level}: welded topology changed");
            assert!(split.is_closed(), "{name} LOD{level}: split mesh not watertight");
            assert_eq!(lod.split_creases(180.0), *lod, "180 deg leaves the mesh as it is");
        }
    }
}

/// A hard-edged solid split at the crease shades flat: every LOD0 vertex normal of a cube is one of its faces'.
fn cube_solid(rotation: Quat, half: f32, voxel: f32) -> Solid {
    let planes = [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z]
        .map(|n| Plane { n: rotation * n, d: half })
        .to_vec();
    let reach = Vec3::splat(half * 3f32.sqrt());
    Solid::new(vec![Piece { planes, ..Piece::default() }], 0.0, Weathering::default(), reach, voxel, true, 0)
}

/// Plane-snap fixture: a cube SDF (axis aligned and tilted off the grid) meshes to its exact corners and faces.
#[test]
fn snapped_cube_has_exact_corners_and_faces() {
    let half = 0.4123;
    let voxel = 1.0 / 40.0;
    for rotation in [Quat::IDENTITY, Quat::from_euler(glam::EulerRot::XYZ, 0.31, -0.57, 0.83)] {
        let settings = MeshSettings { voxel, lod0_error: 0.2 * voxel, lod_levels: 1, lod_ratio: 0.5, crease_deg: 35.0 };
        let lod0 = &mesh_solid(&cube_solid(rotation, half, voxel), &settings)[0];
        assert!(lod0.is_closed() && lod0.signed_volume() > 0.0);
        let local = |p: [f32; 3]| rotation.inverse() * Vec3::from_array(p);
        for &p in &lod0.positions {
            let q = local(p).abs();
            assert!((q.max_element() - half).abs() < 1e-4, "vertex {q} is off the cube's faces");
        }
        for corner in 0..8 {
            let c = Vec3::new(
                if corner & 1 == 0 { -half } else { half },
                if corner & 2 == 0 { -half } else { half },
                if corner & 4 == 0 { -half } else { half },
            );
            let nearest = lod0.positions.iter().map(|&p| (local(p) - c).length()).fold(f32::INFINITY, f32::min);
            assert!(nearest < 1e-4, "corner {c}: nearest vertex {nearest} away");
        }
        let faces: Vec<Vec3> = [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z].map(|n| rotation * n).to_vec();
        let off_normals = lod0.normals.iter().filter(|n| !faces.iter().any(|f| f.dot(Vec3::from_array(**n)) > 0.999)).count();
        let tri_normal = |t: &[u32; 3]| {
            let p = |i: u32| Vec3::from_array(lod0.positions[i as usize]);
            (p(t[1]) - p(t[0])).cross(p(t[2]) - p(t[0]))
        };
        let tris = lod0.indices.as_chunks::<3>().0;
        let off_tris: Vec<f32> = tris
            .iter()
            .map(tri_normal)
            .filter(|n| n.length() > 1e-9 && !faces.iter().any(|f| f.dot(n.normalize()) > 0.999))
            .map(Vec3::length)
            .collect();
        println!("cube: {} verts, {} off normals, {} tris, off tris areas {off_tris:?}", lod0.positions.len(), off_normals, tris.len());
        assert_eq!(off_normals, 0, "every normal is a face normal (hard edges must split)");
        // Flat faces collapse: a cube needs 12 triangles; allow the simplifier some slack.
        assert!(lod0.triangle_count() <= 48, "cube LOD0 kept {} triangles", lod0.triangle_count());
    }
}

/// Without the snap (or with rounding over a cell) the same cube stays bevelled: the A/B the rock lab shows.
#[test]
fn unsnapped_cube_is_bevelled() {
    // Off the grid (0.4 m would put every face on grid points, where surface nets is exact).
    let half = 0.4123;
    let voxel = 1.0 / 40.0;
    let planes = [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z].map(|n| Plane { n, d: half }).to_vec();
    let solid = Solid::new(vec![Piece { planes, ..Piece::default() }], 0.0, Weathering::default(), Vec3::splat(0.7), voxel, false, 0);
    let settings = MeshSettings { voxel, lod0_error: 0.2 * voxel, lod_levels: 1, lod_ratio: 0.5, crease_deg: 180.0 };
    let lod0 = &mesh_solid(&solid, &settings)[0];
    let corner = Vec3::splat(half);
    let nearest = lod0.positions.iter().map(|&p| (Vec3::from_array(p) - corner).length()).fold(f32::INFINITY, f32::min);
    assert!(nearest > 0.1 * voxel, "naive surface nets should bevel the corner (nearest {nearest})");
}

fn mean(v: impl Iterator<Item = usize>) -> usize {
    let (sum, n) = v.fold((0, 0), |(s, n), x| (s + x, n + 1));
    sum / n.max(1)
}

#[test]
#[ignore = "measurement: ms per rock and triangles / vertices per LOD for each class, at 2 m and 12 m"]
fn measure_boulders() {
    let mut arms = classes();
    // The edge-snap A/B arm (default off) on the class it costs most.
    if let Some(&(_, family, params)) = arms.iter().find(|c| c.0 == "columnar") {
        arms.push(("columnar+snap", family, BoulderParams { edge_snap: true, ..params }));
    }
    for (name, _, params) in arms {
        for size_m in [2.0, 12.0] {
            let p = BoulderParams { size_m, ..params };
            let start = std::time::Instant::now();
            let rocks: Vec<RockLods> = SEEDS.map(|s| generate_boulder(&p, s)).collect();
            let ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(u32::try_from(rocks.len()).unwrap_or(1));
            let levels = rocks.iter().map(|r| r.lods.len()).max().unwrap_or(0);
            let per_lod: Vec<String> = (0..levels)
                .map(|l| {
                    let lods: Vec<&RockMesh> = rocks.iter().filter_map(|r| r.lods.get(l)).collect();
                    let tris = mean(lods.iter().map(|m| m.triangle_count()));
                    let verts = mean(lods.iter().map(|m| m.positions.len()));
                    format!("LOD{l} {tris} t/{verts} v")
                })
                .collect();
            // Triangles facing against their own vertex normals (a snap that folded a triangle over).
            let (flipped, total) = rocks.iter().filter_map(|r| r.lods.first()).fold((0usize, 0usize), |(f, t), m| {
                let v = |i: u32| Vec3::from_array(m.positions[i as usize]);
                let n = |i: u32| Vec3::from_array(m.normals[i as usize]);
                let bad = m
                    .indices
                    .as_chunks::<3>()
                    .0
                    .iter()
                    .filter(|tri| (v(tri[1]) - v(tri[0])).cross(v(tri[2]) - v(tri[0])).dot(n(tri[0]) + n(tri[1]) + n(tri[2])) < 0.0)
                    .count();
                (f + bad, t + m.triangle_count())
            });
            println!(
                "rockgen {name} {size_m} m: {ms:.1} ms/rock (res {}), mean {}; LOD0 flipped {flipped}/{total}",
                p.clamped().effective_resolution(),
                per_lod.join(", ")
            );
        }
    }
}
