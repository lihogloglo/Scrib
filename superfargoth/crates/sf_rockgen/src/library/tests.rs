//! RG-2 tests (rockgen.md "Build plan" RG-2): the library is deterministic and follows its tunables (exact), every
//! hull is a valid convex hull containing every LOD0 vertex within the stated tolerance (exact geometry checks).
//! Build times and hull sizes print from the ignored measurement.

use crate::boulder::tunables as t;
use crate::library::{HullSettings, Library, LibrarySettings, build_library, library_key, variant_seed};
use crate::{BoulderParams, CLASSES, generate_boulder};

fn talus() -> BoulderParams {
    CLASSES.iter().find(|c| c.name == "talus").map(|c| c.params(BoulderParams::default())).expect("talus class")
}

fn small() -> LibrarySettings {
    LibrarySettings { variants: 8, ..LibrarySettings::default() }
}

fn talus_library() -> &'static Library {
    static LIB: std::sync::OnceLock<Library> = std::sync::OnceLock::new();
    LIB.get_or_init(|| build_library(&talus(), &small(), 7))
}

#[test]
fn library_is_deterministic() {
    let a = talus_library();
    let b = build_library(&talus(), &small(), 7);
    assert_eq!(a.key, b.key);
    assert_eq!(a.content_hash(), b.content_hash());
    assert_eq!(*a, b);
    // Parallel build == each variant generated alone (thread count can't matter).
    for (i, v) in (0u32..).zip(&a.variants) {
        assert_eq!(v.seed, variant_seed(7, i));
        assert_eq!(v.lods, generate_boulder(&talus(), v.seed), "variant {i}");
    }
    let other = build_library(&talus(), &small(), 8);
    assert_ne!(a.key, other.key);
    assert_ne!(a.content_hash(), other.content_hash());
}

#[test]
fn variant_seeds_are_distinct_and_count_independent() {
    let seeds: Vec<u64> = (0..32).map(|i| variant_seed(7, i)).collect();
    let mut sorted = seeds.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), seeds.len());
    let hashes: Vec<u64> = talus_library().variants.iter().map(|v| v.lods.hash).collect();
    let mut unique = hashes.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), hashes.len(), "every variant is a different rock");
}

#[test]
fn variant_count_follows_the_tunable() {
    assert_eq!(LibrarySettings::default().variants, t::LIBRARY_VARIANTS.default_value());
    assert_eq!(t::LIBRARY_VARIANTS.default_value(), 16);
    assert_eq!(t::LIBRARY_VARIANTS.bounds(), Some((8, 32)));
    assert_eq!(talus_library().variants.len(), 8);
    assert_eq!(LibrarySettings { variants: 3, ..small() }.clamped().variants, 8);
    assert_eq!(LibrarySettings { variants: 100, ..small() }.clamped().variants, 32);
    let nan = HullSettings { tolerance: f32::NAN, ..HullSettings::default() }.clamped();
    assert!((nan.tolerance - t::HULL_TOLERANCE.default_value()).abs() < f32::EPSILON);
}

#[test]
fn key_covers_every_input() {
    let p = talus();
    let s = small();
    let k = library_key(&p, &s, 7);
    assert_eq!(k, talus_library().key);
    assert_ne!(k, library_key(&p, &LibrarySettings { variants: 9, ..s }, 7));
    assert_ne!(k, library_key(&BoulderParams { size_m: 3.0, ..p }, &s, 7));
    assert_ne!(k, library_key(&BoulderParams { edge_snap: !p.edge_snap, ..p }, &s, 7));
    let hull = HullSettings { max_vertices: 32, ..s.hull };
    assert_ne!(k, library_key(&p, &LibrarySettings { hull, ..s }, 7));
    // Out-of-range input keys like its clamped value (same rocks, same key).
    let floor = LibrarySettings { variants: 8, ..s };
    assert_eq!(library_key(&p, &LibrarySettings { variants: 3, ..s }, 7), library_key(&p, &floor, 7));
}

#[test]
fn pick_is_uniform_and_scale_passes_through() {
    let lib = talus_library();
    let mut hits = vec![0u32; lib.variants.len()];
    for seed in 0..4000u64 {
        let i = lib.pick(seed);
        assert_eq!(i, lib.pick(seed));
        hits[i] += 1;
    }
    // 500 expected per variant; a fair pick lands well inside ±30 %.
    assert!(hits.iter().all(|&h| (350..650).contains(&h)), "{hits:?}");
    assert!((lib.scale_for(lib.size_m * 2.5) - 2.5).abs() < 1e-6);
}

#[cfg(not(feature = "collision"))]
#[test]
fn no_hulls_without_the_collision_feature() {
    assert!(talus_library().variants.iter().all(|v| v.hull.is_none()));
}

#[cfg(feature = "collision")]
mod hulls {
    use glam::Vec3;
    use parry3d::query::PointQuery;
    use parry3d::transformation::check_convex_hull;

    use super::talus_library;
    use crate::collision::build_hull;
    use crate::library::{Hull, HullSettings};
    use crate::{RockLods, RockMesh};

    /// Float slack on top of the tolerance (GJK projection precision at rock scale).
    const SLACK_M: f32 = 1e-4;

    fn longest(lods: &RockLods) -> f32 {
        let e = lods.bounds.extent();
        e[0].max(e[1]).max(e[2])
    }

    /// Every class × seed of the RG-1/RG-4 tests with its default-settings hull (one thread per class).
    fn hulls() -> &'static [(&'static str, u64, &'static RockLods, Hull)] {
        static HULLS: std::sync::OnceLock<Vec<(&'static str, u64, &'static RockLods, Hull)>> = std::sync::OnceLock::new();
        HULLS.get_or_init(|| {
            let rocks = crate::tests::rocks();
            std::thread::scope(|scope| {
                let jobs: Vec<_> = rocks
                    .chunk_by(|a, b| a.0 == b.0)
                    .map(|class| {
                        scope.spawn(move || {
                            class
                                .iter()
                                .map(|(name, _, seed, lods)| {
                                    let hull = build_hull(&lods.lods[0], &HullSettings::default(), longest(lods))
                                        .unwrap_or_else(|| panic!("{name} seed {seed}: no hull"));
                                    (*name, *seed, lods, hull)
                                })
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect();
                jobs.into_iter().flat_map(|j| j.join().expect("hull worker")).collect()
            })
        })
    }

    #[test]
    fn library_variants_carry_hulls() {
        let lib = talus_library();
        for v in &lib.variants {
            let hull = v.hull.as_ref().expect("collision feature builds a hull per variant");
            assert!(hull.to_parry().is_some());
        }
    }

    #[test]
    fn hull_contains_every_lod0_vertex_within_tolerance() {
        let settings = HullSettings::default();
        for (name, seed, lods, hull) in hulls() {
            let tol = settings.tolerance_m(longest(lods));
            assert!(hull.error_m <= tol, "{name} seed {seed}: hull error {} m > tolerance {tol} m", hull.error_m);
            let shape = hull.to_parry().expect("valid hull");
            let mut worst = 0.0f32;
            for p in &lods.lods[0].positions {
                worst = worst.max(shape.distance_to_local_point(Vec3::from_array(*p), true));
            }
            assert!(worst <= tol + SLACK_M, "{name} seed {seed}: LOD0 vertex {worst} m outside (tolerance {tol} m)");
            assert!(worst <= hull.error_m + SLACK_M, "{name} seed {seed}: error_m {} under-reports {worst}", hull.error_m);
        }
    }

    #[test]
    fn hull_is_a_valid_convex_subset_of_lod0() {
        let cap = usize::try_from(HullSettings::default().max_vertices).unwrap_or(usize::MAX);
        for (name, seed, lods, hull) in hulls() {
            assert!(hull.points.iter().flatten().all(|x| x.is_finite()), "{name} seed {seed}: NaN");
            assert!(hull.error_m.is_finite() && hull.error_m >= 0.0);
            assert!((4..=cap).contains(&hull.points.len()), "{name} seed {seed}: {} hull vertices", hull.points.len());
            let pts: Vec<Vec3> = hull.points.iter().map(|p| Vec3::from_array(*p)).collect();
            check_convex_hull(&pts, &hull.triangles).unwrap_or_else(|e| panic!("{name} seed {seed}: {e:?}"));
            // A subset of LOD0's vertices (bitwise), so it never sticks out of the visual mesh.
            let lod0: std::collections::HashSet<[u32; 3]> =
                lods.lods[0].positions.iter().map(|p| p.map(f32::to_bits)).collect();
            assert!(hull.points.iter().all(|p| lod0.contains(&p.map(f32::to_bits))), "{name} seed {seed}");
            // Closed, outward (positive volume), and every triangle convex: no vertex in front of any face.
            let mesh = RockMesh {
                positions: hull.points.clone(),
                normals: Vec::new(),
                indices: hull.triangles.iter().flatten().copied().collect(),
            };
            assert!(mesh.is_closed(), "{name} seed {seed}: hull not closed");
            assert!(mesh.signed_volume() > 0.0, "{name} seed {seed}: hull inside out");
        }
    }
}

#[test]
#[ignore = "measurement: library build ms per class (16 variants, default size), hull vertex counts and build ms"]
fn measure_libraries() {
    let settings = LibrarySettings::default();
    for class in CLASSES {
        let params = class.params(BoulderParams::default());
        let start = std::time::Instant::now();
        let lib = build_library(&params, &settings, 1);
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        let tris: usize = lib.variants.iter().map(|v| v.lods.lods[0].triangle_count()).sum();
        #[cfg(feature = "collision")]
        let hull = hull_stats(&lib, &settings.hull);
        #[cfg(not(feature = "collision"))]
        let hull = String::from("(no collision feature)");
        println!(
            "rockgen library {} {} m x{}: {ms:.0} ms wall, LOD0 mean {} t; {hull}",
            class.name,
            lib.size_m,
            lib.variants.len(),
            tris / lib.variants.len().max(1)
        );
    }
}

#[cfg(feature = "collision")]
fn hull_stats(lib: &Library, settings: &HullSettings) -> String {
    let mut verts = Vec::new();
    let mut exact = Vec::new();
    let mut err: f32 = 0.0;
    let start = std::time::Instant::now();
    for v in &lib.variants {
        let lod0 = &v.lods.lods[0];
        let hull = crate::collision::build_hull(lod0, settings, lib.size_m).expect("hull");
        verts.push(hull.points.len());
        err = err.max(hull.error_m);
        let pts: Vec<glam::Vec3> = lod0.positions.iter().map(|p| glam::Vec3::from_array(*p)).collect();
        exact.push(parry3d::transformation::try_convex_hull(&pts).map_or(0, |h| h.0.len()));
    }
    let hull_ms = start.elapsed().as_secs_f64() * 1000.0 / f64::from(u32::try_from(verts.len()).unwrap_or(1));
    let mean = |v: &[usize]| v.iter().sum::<usize>() / v.len().max(1);
    format!(
        "hull verts {}-{} mean {} (exact hull mean {}), max error {:.1} mm, {hull_ms:.2} ms/hull (incl. exact-hull rerun)",
        verts.iter().min().unwrap_or(&0),
        verts.iter().max().unwrap_or(&0),
        mean(&verts),
        mean(&exact),
        err * 1000.0
    )
}
