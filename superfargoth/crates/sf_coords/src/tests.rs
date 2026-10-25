// Exact float equality is what these tests check: canonical forms are bit-identical.
#![allow(clippy::float_cmp)]

use super::*;

fn canonical(p: WorldPos) -> bool {
    p.local.to_array().iter().all(|v| (-HALF_CELL..HALF_CELL).contains(v))
}

#[test]
fn round_trip_f64_is_exact_far_away() {
    for x in [0.0, 1e-3, -1e-3, 127.999, 128.0, -128.0, 1e6 + 0.001, -7.5e7, 1e8 + 0.25, 4.2e11] {
        let p = DVec3::new(x, -x / 3.0, x * 0.5);
        let w = WorldPos::from_f64(p);
        assert!(canonical(w), "{w:?}");
        let back = w.to_f64();
        // f32 local keeps ~15 µm at worst inside a 256 m cell.
        assert!((back - p).abs().max_element() < 2e-5, "{p:?} -> {w:?} -> {back:?}");
    }
}

#[test]
fn boundaries_are_canonical() {
    let w = WorldPos::from_f64(DVec3::new(128.0, -256.0, 383.999_999_999));
    assert_eq!(w.cell, IVec3::new(1, -1, 2));
    assert_eq!(w.local.x, -128.0);
    assert_eq!(w.local.y, 0.0);
    assert!(canonical(w));
    // Values that round up to +128 in f32 roll over into the next cell.
    let edge = WorldPos::new(IVec3::ZERO, Vec3::new(127.999_999, 0.0, 0.0));
    assert!(canonical(edge));
}

#[test]
fn negative_zero_is_normalized() {
    let a = WorldPos::new(IVec3::ZERO, Vec3::new(-0.0, 0.0, -0.0));
    assert_eq!(a.to_bytes(), WorldPos::ORIGIN.to_bytes());
}

#[test]
fn offset_crosses_cells() {
    let p = WorldPos::ORIGIN.offset(Vec3::new(300.0, -1000.0, 0.5));
    assert_eq!(p.cell, IVec3::new(1, -4, 0));
    assert!((p.to_f64() - DVec3::new(300.0, -1000.0, 0.5)).abs().max_element() < 1e-9);
}

#[test]
fn delta_is_exact_across_extremes() {
    let a = WorldPos::new(IVec3::splat(i32::MIN + 1), Vec3::new(0.5, 0.0, 0.0));
    let b = WorldPos::new(IVec3::splat(i32::MAX - 1), Vec3::new(0.25, 0.0, 0.0));
    let d = a.delta(b);
    let cells = (f64::from(i32::MAX - 1) - f64::from(i32::MIN + 1)) * 256.0;
    assert_eq!(d.x, cells - 0.25);
    assert_eq!(d.y, cells);
}

#[test]
fn near_positions_one_millimeter_apart_far_away() {
    let a = WorldPos::from_f64(DVec3::new(1e6, 0.0, 0.0));
    let b = WorldPos::from_f64(DVec3::new(1e6 + 0.001, 0.0, 0.0));
    assert!((a.delta_f32(b).x - 0.001).abs() < 1e-5);
}

#[test]
fn bytes_golden_and_round_trip() {
    let p = WorldPos::new(IVec3::new(1, -2, 3), Vec3::new(0.5, -1.0, 2.0));
    let bytes = p.to_bytes();
    assert_eq!(&bytes[0..4], &1i32.to_le_bytes());
    assert_eq!(&bytes[4..8], &(-2i32).to_le_bytes());
    assert_eq!(&bytes[12..16], &0.5f32.to_le_bytes());
    assert_eq!(WorldPos::from_bytes(bytes), p);
}

#[test]
fn partitions_nest() {
    // Points just inside each corner of grid cell 0.
    for x in [-128.0, -0.001, 0.0, 127.99] {
        let p = WorldPos::from_f64(DVec3::new(x, 0.0, x));
        assert_eq!(p.partition(0), IVec3::ZERO, "x = {x}");
        let half = p.partition(-1);
        let expected = i32::from(x >= 0.0);
        assert_eq!(half.x, expected, "x = {x}");
        // A finer cell always lies inside the coarser one.
        assert_eq!(half.x >> 1, p.partition(0).x);
    }
    let far = WorldPos::from_f64(DVec3::new(-1000.0, 0.0, 5000.0));
    for shift in -3..6 {
        let idx = far.partition(shift);
        let origin = partition_origin(idx, shift);
        let rel = origin.delta(far);
        let edge = f64::from(partition_extent(shift));
        assert!(rel.x >= 0.0 && rel.x < edge, "shift {shift}: {rel:?}");
        assert!(rel.z >= 0.0 && rel.z < edge, "shift {shift}: {rel:?}");
        if shift > -3 {
            let finer = far.partition(shift - 1);
            assert_eq!(finer.x >> 1, idx.x);
            assert_eq!(finer.z >> 1, idx.z);
        }
    }
}

#[test]
fn serde_accepts_grid_and_absolute_forms() {
    let p: WorldPos = serde_json::from_str(r#"{"cell":[1,0,-1],"local":[1.5,2.0,-3.0]}"#).unwrap();
    assert_eq!(p, WorldPos::new(IVec3::new(1, 0, -1), Vec3::new(1.5, 2.0, -3.0)));
    let q: WorldPos = serde_json::from_str("[257.5, 2.0, -259.0]").unwrap();
    assert_eq!(q, p);
    let text = serde_json::to_string(&p).unwrap();
    assert_eq!(serde_json::from_str::<WorldPos>(&text).unwrap(), p);

    let t: WorldTransform = serde_json::from_str(r#"{"pos":[10.0,0.0,0.0]}"#).unwrap();
    assert_eq!(t.rotation, Quat::IDENTITY);
    assert_eq!(t.scale, Vec3::ONE);
}

#[test]
fn binary_serde_round_trips() {
    let t = WorldTransform {
        pos: WorldPos::new(IVec3::new(-3, 0, 9), Vec3::new(1.25, 2.0, -0.5)),
        rotation: Quat::from_rotation_y(0.5),
        scale: Vec3::splat(2.0),
    };
    let bytes = postcard::to_stdvec(&t).unwrap();
    let back: WorldTransform = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(back.pos, t.pos);
    assert!(back.rotation.abs_diff_eq(t.rotation, 1e-6));
}
