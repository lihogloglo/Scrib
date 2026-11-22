//! Mesh conversion and LOD range checks (exact: they are data, not look).

use bevy_mesh::{Indices, Mesh, VertexAttributeValues};
use sf_rockgen::{BoulderParams, generate_boulder};

use crate::{CLASSES, RockLodSettings, class_params, lod_ranges, params_from_tunables, rock_mesh};

fn attribute(mesh: &Mesh, id: bevy_mesh::MeshVertexAttribute) -> Vec<[f32; 3]> {
    match mesh.attribute(id) {
        Some(VertexAttributeValues::Float32x3(v)) => v.clone(),
        other => panic!("attribute {} is {other:?}", id.name),
    }
}

/// Every LOD of every class converts with matching counts, unit normals, no NaN, the short axis up, and
/// the same triangles (indices unchanged, positions only rotated).
#[test]
#[allow(clippy::float_cmp)] // the rotation only moves components and flips a sign: exact
fn rock_mesh_conversion_is_sane() {
    for (class, name) in CLASSES.iter().map(|c| c.name).enumerate() {
        let params = class_params(class, BoulderParams { size_m: 3.0, ..BoulderParams::default() });
        let lods = generate_boulder(&params, 11 + class as u64);
        assert!(!lods.lods.is_empty(), "{name}: no LODs");
        for (k, src) in lods.lods.iter().enumerate() {
            let mesh = rock_mesh(src);
            let positions = attribute(&mesh, Mesh::ATTRIBUTE_POSITION);
            let normals = attribute(&mesh, Mesh::ATTRIBUTE_NORMAL);
            assert_eq!(positions.len(), src.positions.len(), "{name} LOD{k}: vertex count");
            assert_eq!(normals.len(), positions.len(), "{name} LOD{k}: normal count");
            let Some(Indices::U32(indices)) = mesh.indices() else { panic!("{name} LOD{k}: no u32 indices") };
            assert_eq!(indices, &src.indices, "{name} LOD{k}: indices");
            assert_eq!(indices.len() % 3, 0);
            assert!(indices.iter().all(|&i| (i as usize) < positions.len()), "{name} LOD{k}: index out of range");
            for (p, n) in positions.iter().zip(&normals) {
                assert!(p.iter().chain(n).all(|v| v.is_finite()), "{name} LOD{k}: NaN or infinity");
                let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                assert!((len - 1.0).abs() < 1e-3, "{name} LOD{k}: normal length {len}");
            }
            for (p, q) in positions.iter().zip(&src.positions) {
                assert_eq!(*p, [q[0], q[2], -q[1]], "{name} LOD{k}: (x, y, z) -> (x, z, -y)");
            }
        }
        // sf_rockgen's +Z is the short axis: here the vertical extent is the smallest.
        let positions = attribute(&rock_mesh(&lods.lods[0]), Mesh::ATTRIBUTE_POSITION);
        let extent = |axis: usize| {
            let (lo, hi) = positions.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(p[axis]), hi.max(p[axis])));
            hi - lo
        };
        assert!(extent(1) <= extent(0) + 1e-3 && extent(1) <= extent(2) + 1e-3, "{name}: y is not the short axis");
    }
}

/// LOD ranges: LOD0 from the camera, each switch farther than the last, neighbours overlap exactly by the
/// crossfade, the last one open-ended.
#[test]
fn lod_ranges_chain() {
    let s = RockLodSettings::default();
    let ranges = lod_ranges(2.0, 4, &s);
    assert_eq!(ranges.len(), 4);
    assert_eq!(ranges[0].start_margin, 0.0..0.0);
    for k in 1..4 {
        assert_eq!(ranges[k].start_margin, ranges[k - 1].end_margin, "LOD{k} fades in where LOD{} fades out", k - 1);
        assert!(ranges[k].start_margin.start > ranges[k - 1].start_margin.end, "LOD{k} switches farther out");
    }
    let switch = s.switch_m(2.0, 1);
    assert!((switch - 2.0 * s.lod1_distance).abs() < 1e-4);
    assert!(ranges[3].end_margin.start > 1.0e6, "the last LOD never ends");
    assert_eq!(lod_ranges(2.0, 1, &s)[0].end_margin, ranges[3].end_margin, "a single LOD is open-ended");
}

/// With no store values set, the store's parameters are the declared defaults.
#[test]
fn tunable_defaults_are_boulder_defaults() {
    let mut store = sf_tunables::Tunables::default();
    store.register_all(sf_rockgen::boulder::tunables::TUNABLES).expect("register");
    assert_eq!(params_from_tunables(&store), BoulderParams::default().clamped());
    assert_eq!(params_from_tunables(&sf_tunables::Tunables::default()), BoulderParams::default().clamped(), "unregistered: defaults");
}
