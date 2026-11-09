use glam::{Affine3A, IVec3, Quat, Vec3};

use super::*;

fn chain(mesh: u64, class: MaterialClass, errors: &[f32]) -> LodChain {
    LodChain {
        mesh,
        material: mesh.rotate_left(17) ^ 0xA5A5,
        material_class: class,
        levels: errors
            .iter()
            .zip(0u32..)
            .map(|(&error, k)| LodLevel { first_index: k * 300, index_count: 300 >> k, error })
            .collect(),
    }
}

fn instance(placement: u32, chain: u32, center: Vec3, radius: f32) -> StaticInstance {
    let affine = Affine3A::from_scale_rotation_translation(Vec3::splat(1.5), Quat::from_rotation_y(0.7), center);
    StaticInstance {
        placement,
        transform: StaticInstance::rows(&affine),
        sphere: Sphere { center, radius },
        chain,
        material_override: 0,
        flags: InstanceFlags::CASTS_SHADOW | InstanceFlags::WIND,
    }
}

/// Chain 0 (mesh key 0x10, three levels) and chain 1 (mesh key 0x20, one level).
fn sample() -> StaticCellTable {
    StaticCellTable {
        origin_cell: IVec3::new(-3, 0, 7),
        aabb: Aabb { min: Vec3::new(-128.0, -5.0, -128.0), max: Vec3::new(128.0, 40.0, 128.0) },
        land_min: -2.5,
        layers: 0x8000_0001,
        instances: vec![
            instance(4, 0, Vec3::new(1.0, 2.0, 3.0), 2.0),
            instance(9, 1, Vec3::new(-50.0, 0.5, 60.0), 8.0),
            StaticInstance { flags: InstanceFlags::ALPHA_MASKED, ..instance(77, 0, Vec3::ZERO, 0.5) },
        ],
        chains: vec![
            chain(0x10, MaterialClass::OpaqueWind, &[0.0, 0.02, 0.1]),
            chain(0xFFFF_FFFF_0000_0020, MaterialClass::MaskedWindTwoSided, &[0.0]),
        ],
    }
}

#[test]
fn v2_round_trip_is_exact() {
    let table = sample();
    let bytes = table.write().unwrap();
    assert_eq!(bytes.len(), 80 + 3 * 80 + 2 * 32 + 4 * 12);
    assert_eq!(&bytes[..8], b"SFSCENE\0");
    assert_eq!(FORMAT_VERSION, 2);
    assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 2);
    // Layers live in the header (offset 64), once per table.
    assert_eq!(u32::from_le_bytes(bytes[64..68].try_into().unwrap()), 0x8000_0001);
    assert_eq!(&bytes[68..80], &[0; 12]);
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert_eq!(view.to_table(), table);
    assert_eq!(view.len(), 3);
    assert_eq!(view.layers(), 0x8000_0001);
    assert_eq!(view.origin_cell(), IVec3::new(-3, 0, 7));
    assert_eq!(view.land_min().to_bits(), (-2.5f32).to_bits());
    assert_eq!(view.instance(1), Some(table.instances[1]));
    assert_eq!(view.instance(3), None);
    assert_eq!(view.find(77), Some(2));
    assert_eq!(view.find(4), Some(0));
    assert_eq!(view.find(5), None);
    assert_eq!(view.chain_count(), 2);
    assert_eq!(view.chain(0), Some(table.chains[0].clone()));
    assert_eq!(view.chain(2), None);
    assert_eq!(view.levels(1).unwrap().len(), 1);
    assert_eq!(view.material_class(1), Some(MaterialClass::MaskedWindTwoSided));
    // The instance section starts 16-aligned; its records are 80 B with the chain at 68, the
    // override at 72 and the flags at 76.
    assert_eq!(view.instance_bytes(), &bytes[80..80 + 240]);
    let second = &view.instance_bytes()[80..160];
    assert_eq!(&second[64..68], &9u32.to_le_bytes());
    assert_eq!(&second[68..72], &1u32.to_le_bytes());
    assert_eq!(&second[72..76], &0u32.to_le_bytes());
    assert_eq!(&second[76..80], &(InstanceFlags::CASTS_SHADOW | InstanceFlags::WIND).bits().to_le_bytes());
    // Chain records: class at 8, mesh key at 16, material key at 24.
    let chain1 = &bytes[80 + 240 + 32..80 + 240 + 64];
    assert_eq!(&chain1[8..10], &(MaterialClass::MaskedWindTwoSided as u16).to_le_bytes());
    assert_eq!(&chain1[16..24], &0xFFFF_FFFF_0000_0020u64.to_le_bytes());
    assert_eq!(&chain1[24..32], &table.chains[1].material.to_le_bytes());
    // Writing the read table again gives the same bytes.
    assert_eq!(view.to_table().write().unwrap(), bytes);
}

#[test]
fn v1_and_other_versions_are_rejected() {
    let bytes = sample().write().unwrap();
    for v in [0u32, 1, 3, u32::MAX] {
        let mut b = bytes.clone();
        b[8..12].copy_from_slice(&v.to_le_bytes());
        assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::Version(v));
    }
    // A real v1 file (64 B header, no instance): rejected on its version, before its size.
    let mut v1 = b"SFSCENE\0".to_vec();
    v1.extend_from_slice(&1u32.to_le_bytes());
    v1.resize(64, 0);
    assert_eq!(StaticCellTableView::parse(&v1).unwrap_err(), TableError::Version(1));
}

#[test]
fn chain_indices_are_dense_and_remap_by_mesh_key() {
    let table = sample();
    let bytes = table.write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    // Remapping data: the mesh key finds the cell-local index.
    assert_eq!(view.chain_index(0x10), Some(0));
    assert_eq!(view.chain_index(0xFFFF_FFFF_0000_0020), Some(1));
    assert_eq!(view.chain_index(0x11), None);
    // Stream-in: cell-local 0 → global 40, 1 → 7; everything else is copied as is.
    let mut out = vec![0xEE];
    view.write_remapped(&[40, 7], &mut out);
    assert_eq!(out.len(), 1 + view.instance_bytes().len());
    let records = out[1..].as_chunks::<80>().0;
    let chains: Vec<u32> = records.iter().map(|r| u32::from_le_bytes(r[68..72].try_into().unwrap())).collect();
    assert_eq!(chains, [40, 7, 40]);
    for (copied, stored) in records.iter().zip(view.instance_bytes().as_chunks::<80>().0) {
        assert_eq!(copied[..68], stored[..68]);
        assert_eq!(copied[72..], stored[72..]);
    }
    // Indices past the chains, unused chains and unsorted or repeated mesh keys are rejected.
    let mut t = sample();
    t.instances[1].chain = 2;
    assert_eq!(t.write().unwrap_err(), TableError::MissingChain { placement: 9, chain: 2 });
    let mut t = sample();
    t.instances[1].chain = 0;
    assert_eq!(t.write().unwrap_err(), TableError::UnusedChain(1));
    let mut t = sample();
    t.chains.swap(0, 1);
    assert_eq!(t.write().unwrap_err(), TableError::ChainOrder(0x10));
    let mut t = sample();
    t.chains[1].mesh = 0x10;
    assert_eq!(t.write().unwrap_err(), TableError::ChainOrder(0x10));
}

#[test]
#[should_panic(expected = "one global slot per chain")]
fn remap_needs_one_slot_per_chain() {
    let bytes = sample().write().unwrap();
    StaticCellTableView::parse(&bytes).unwrap().write_remapped(&[1], &mut Vec::new());
}

#[test]
fn material_class_and_key_live_in_the_chain() {
    let bytes = sample().write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    let classes: Vec<_> = view.chains().map(|c| (c.material_class, c.material)).collect();
    let t = sample();
    assert_eq!(classes, [(MaterialClass::OpaqueWind, t.chains[0].material), (MaterialClass::MaskedWindTwoSided, t.chains[1].material)]);
    // An unknown class is rejected; so are non-zero reserved chain bytes.
    let chain0 = 80 + 3 * 80;
    let mut b = bytes.clone();
    b[chain0 + 8..chain0 + 10].copy_from_slice(&8u16.to_le_bytes());
    assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::MaterialClass { chain: 0, class: 8 });
    for at in [10, 12, 15] {
        let mut b = bytes.clone();
        b[chain0 + at] = 1;
        assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::Reserved(chain0 + at));
    }
    let mut b = bytes.clone();
    b[70] = 1;
    assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::Reserved(70));
}

#[test]
fn material_override_defaults_to_the_chain() {
    let t = sample();
    assert!(t.instances.iter().all(|i| i.material_override == 0));
    let bytes = t.write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert!(view.instances().all(|i| i.material_override == 0));
    // No override table exists in v2: a baked override is rejected.
    let mut t = sample();
    t.instances[2].material_override = 3;
    assert_eq!(t.write().unwrap_err(), TableError::MaterialOverride(77));
}

#[test]
fn placements_with_several_parts_share_an_id() {
    // Placement 9 has three parts (chains 0, 1, 0 again: two parts may draw the same mesh
    // from different placements, and a placement's parts are adjacent).
    let mut t = sample();
    t.instances = vec![
        instance(4, 0, Vec3::ZERO, 1.0),
        instance(9, 0, Vec3::ZERO, 1.0),
        instance(9, 1, Vec3::ZERO, 1.0),
        instance(9, 0, Vec3::X, 1.0),
        instance(12, 1, Vec3::ZERO, 1.0),
    ];
    let bytes = t.write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert_eq!(view.to_table(), t);
    assert_eq!(view.find(4), Some(0));
    assert_eq!(view.find(9), Some(1));
    assert_eq!(view.find(12), Some(4));
    assert_eq!(view.find(10), None);
    assert_eq!(view.find(u32::MAX), None);
    assert_eq!(view.find_all(9), 1..4);
    assert_eq!(view.find_all(12), 4..5);
    assert_eq!(view.find_all(5), 0..0);
    // Decreasing ids are still rejected.
    t.instances.swap(3, 4);
    assert_eq!(t.write().unwrap_err(), TableError::PlacementOrder(9));
}

#[test]
fn affine_rows_round_trip_and_max_scale() {
    let a = Affine3A::from_scale_rotation_translation(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_x(0.3), Vec3::new(4.0, 5.0, 6.0));
    let i = StaticInstance { transform: StaticInstance::rows(&a), ..instance(1, 0, Vec3::ZERO, 1.0) };
    assert_eq!(i.affine(), a);
    assert_eq!(i.transform[0][3].to_bits(), 4f32.to_bits());
    assert_eq!(i.transform[2][3].to_bits(), 6f32.to_bits());
    let p = Vec3::new(1.0, -1.0, 0.5);
    let [r0, r1, r2] = i.transform;
    let by_rows = Vec3::new(
        r0[0] * p.x + r0[1] * p.y + r0[2] * p.z + r0[3],
        r1[0] * p.x + r1[1] * p.y + r1[2] * p.z + r1[3],
        r2[0] * p.x + r2[1] * p.y + r2[2] * p.z + r2[3],
    );
    assert!(by_rows.abs_diff_eq(a.transform_point3(p), 1e-6));
    // The largest axis scale, whatever the rotation.
    assert!((i.max_scale() - 3.0).abs() < 1e-6);
    assert!((instance(1, 0, Vec3::ZERO, 1.0).max_scale() - 1.5).abs() < 1e-6);
}

#[test]
fn empty_table_round_trips() {
    let table = StaticCellTable {
        origin_cell: IVec3::ZERO,
        aabb: Aabb { min: Vec3::ZERO, max: Vec3::ZERO },
        land_min: f32::NEG_INFINITY,
        layers: 1,
        instances: vec![],
        chains: vec![],
    };
    let bytes = table.write().unwrap();
    assert_eq!(bytes.len(), 80);
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert!(view.is_empty());
    assert_eq!(view.to_table(), table);
}

#[test]
fn rejects_bad_bytes() {
    let bytes = sample().write().unwrap();
    let mut b = bytes.clone();
    b[0] = b'X';
    assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::BadMagic);
    assert_eq!(StaticCellTableView::parse(&[]).unwrap_err(), TableError::BadMagic);
    // Truncated anywhere, or with trailing bytes: a size error, never a panic.
    for n in 12..bytes.len() {
        assert!(matches!(StaticCellTableView::parse(&bytes[..n]), Err(TableError::Size { .. })), "{n}");
    }
    let mut b = bytes.clone();
    b.push(0);
    assert!(matches!(StaticCellTableView::parse(&b), Err(TableError::Size { .. })));
    // A huge count can't overflow the size check.
    let mut b = bytes.clone();
    b[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(StaticCellTableView::parse(&b), Err(TableError::Size { .. })));
}

#[test]
fn rejects_inconsistent_tables() {
    let write = |t: StaticCellTable| t.write().unwrap_err();
    let mut t = sample();
    t.instances.swap(0, 1);
    assert_eq!(write(t), TableError::PlacementOrder(4));
    let mut t = sample();
    t.instances[2].placement = 4;
    assert_eq!(write(t), TableError::PlacementOrder(4));
    let mut t = sample();
    t.chains[1].levels.clear();
    assert_eq!(write(t), TableError::ChainLevels(1));
    let mut t = sample();
    t.chains[1] = chain(t.chains[1].mesh, MaterialClass::Opaque, &[0.0; MAX_LOD_LEVELS + 1]);
    assert_eq!(write(t), TableError::ChainLevels(1));
    let mut t = sample();
    t.chains[0].levels[1].error = f32::NAN;
    assert_eq!(write(t), TableError::LevelError(0));
    let mut t = sample();
    t.instances[2].flags |= InstanceFlags::INACTIVE;
    assert_eq!(write(t), TableError::Flags(77));
    // Unknown flag bits (low and high half) and a level range past the section, by hand.
    for bit in [9, 16, 31] {
        let mut b = sample().write().unwrap();
        b[80 + 76..80 + 80].copy_from_slice(&(1u32 << bit).to_le_bytes());
        assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::Flags(4), "bit {bit}");
    }
    let mut b = sample().write().unwrap();
    let chain1 = 80 + 3 * 80 + 32;
    b[chain1..chain1 + 4].copy_from_slice(&4u32.to_le_bytes());
    assert_eq!(StaticCellTableView::parse(&b).unwrap_err(), TableError::ChainLevels(1));
}

#[test]
fn rejects_header_values_that_would_break_a_cull() {
    let write = |t: StaticCellTable| t.write().unwrap_err();
    // A table in no layer is a cell nothing would ever draw.
    assert_eq!(write(StaticCellTable { layers: 0, ..sample() }), TableError::NoLayers);
    // A non-finite or inverted box pre-culls every cell, or none.
    for aabb in [
        Aabb { min: Vec3::new(f32::NAN, 0.0, 0.0), max: Vec3::splat(1.0) },
        Aabb { min: Vec3::splat(f32::NEG_INFINITY), max: Vec3::splat(f32::INFINITY) },
        Aabb { min: Vec3::splat(1.0), max: Vec3::splat(-1.0) },
        Aabb { min: Vec3::new(0.0, 2.0, 0.0), max: Vec3::new(1.0, 1.0, 1.0) },
    ] {
        assert_eq!(write(StaticCellTable { aabb, ..sample() }), TableError::Aabb, "{aabb:?}");
    }
    // A degenerate (point) box is fine: a cell with one tiny instance, or none.
    StaticCellTable { aabb: Aabb { min: Vec3::ZERO, max: Vec3::ZERO }, ..sample() }.write().unwrap();
    // NaN makes every horizon comparison false, +inf makes them all true; only −inf means
    // "no land" (it hides nothing).
    for land_min in [f32::NAN, f32::INFINITY] {
        assert_eq!(write(StaticCellTable { land_min, ..sample() }), TableError::LandMin, "{land_min}");
    }
    StaticCellTable { land_min: f32::NEG_INFINITY, ..sample() }.write().unwrap();
}

#[test]
fn rejects_instances_the_cull_could_not_test() {
    let write = |t: StaticCellTable| t.write().unwrap_err();
    // A non-finite transform: `max_scale` would be NaN and the LOD pick meaningless.
    let mut t = sample();
    t.instances[1].transform[1][2] = f32::NAN;
    assert_eq!(write(t), TableError::Transform(9));
    let mut t = sample();
    t.instances[0].transform[0][3] = f32::INFINITY;
    assert_eq!(write(t), TableError::Transform(4));
    // A non-finite or negative sphere breaks the frustum, sub-pixel and LOD tests.
    for sphere in [
        Sphere { center: Vec3::new(0.0, f32::NAN, 0.0), radius: 1.0 },
        Sphere { center: Vec3::ZERO, radius: f32::NAN },
        Sphere { center: Vec3::ZERO, radius: f32::INFINITY },
        Sphere { center: Vec3::ZERO, radius: -1.0 },
    ] {
        let mut t = sample();
        t.instances[1].sphere = sphere;
        assert_eq!(write(t), TableError::Sphere(9), "{sphere:?}");
    }
    // Radius 0 is a legal point instance.
    let mut t = sample();
    t.instances[1].sphere = Sphere { center: Vec3::ZERO, radius: 0.0 };
    t.write().unwrap();
}

#[test]
fn rejects_flags_no_slice_implements_yet() {
    // FAR_ONLY (S7) and OCCLUDER (S6) parse as flags but nothing writes or reads them: a table
    // setting one would be stored, ignored, and silently wrong the day the slice lands.
    assert_eq!(
        InstanceFlags::NOT_BAKED,
        InstanceFlags::INACTIVE | InstanceFlags::FAR_ONLY | InstanceFlags::OCCLUDER
    );
    for flag in [InstanceFlags::FAR_ONLY, InstanceFlags::OCCLUDER, InstanceFlags::INACTIVE] {
        let mut t = sample();
        t.instances[1].flags |= flag;
        assert_eq!(t.write().unwrap_err(), TableError::Flags(9), "{flag:?}");
    }
    // The bits the bake does write are accepted.
    let mut t = sample();
    t.instances[1].flags = InstanceFlags::CASTS_SHADOW | InstanceFlags::ALPHA_MASKED | InstanceFlags::WIND;
    t.write().unwrap();
}
