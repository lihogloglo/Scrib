//! Skins and geometry morphs (`crate::skin`) on synthetic NIFs.
use super::*;
use tes3::nif::{
    BoneData, MorphTarget, NiFloatKey, NiGeomMorpherController, NiLinFloatKey, NiMorphData, NiMorpherController,
    NiSkinData, NiSkinInstance,
};

const QUAD: [Vec3; 4] = [Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0), Vec3::Y];

fn shape(stream: &mut NiStream, name: &str, translation: Vec3, vertices: Vec<Vec3>) -> NiLink<NiAVObject> {
    let data = stream.insert(NiTriShapeData {
        base: NiTriBasedGeomData {
            base: NiGeometryData { normals: vec![Vec3::Z; vertices.len()], vertices, ..NiGeometryData::default() },
        },
        triangles: vec![[0, 1, 2], [0, 2, 3]],
        ..NiTriShapeData::default()
    });
    stream
        .insert(NiTriShape {
            base: NiTriBasedGeom {
                base: NiGeometry { base: av(name, translation, vec![]), geometry_data: data.cast(), ..NiGeometry::default() },
            },
        })
        .cast()
}

/// A NIF-space transform as `tes3` stores skin transforms (rotation transposed).
fn stored(a: Affine3A) -> (Mat3, Vec3) {
    (Mat3::from(a.matrix3).transpose(), Vec3::from(a.translation))
}

/// Root (offset; the converter drops it) > Bip01 > b1 (turned 90°) > b2, and
/// a skinned quad at `shape_at` under Root. b1 rests at z = 10 but was bound
/// at `b1_bind`; `skin_offset` leaves `NiSkinData`'s overall transform at
/// identity instead of the shape's inverse placement.
fn skinned(shape_at: Vec3, b1_bind: Vec3, skin_offset: bool) -> Model {
    let mut stream = NiStream::new();
    let turn = Mat3::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let b2 = node(&mut stream, "b2", Vec3::new(0.0, 5.0, 0.0), vec![], vec![]);
    let b1 = node(&mut stream, "b1", Vec3::new(0.0, 0.0, 10.0), vec![b2], vec![]);
    if let Some(n) = stream.get_mut(b1.cast::<NiNode>()) {
        n.base.rotation = turn.transpose(); // stored transposed, like skin transforms
    }
    let bip = node(&mut stream, "Bip01", Vec3::ZERO, vec![b1], vec![]);
    let quad = shape(&mut stream, "skin", shape_at, QUAD.to_vec());
    let top = node(&mut stream, "Root", Vec3::new(100.0, 0.0, 0.0), vec![bip, quad], vec![]);
    root(&mut stream, top);

    // Bind poses in Bip01 space; skin space is the shape's.
    let b1_world = Affine3A::from_mat3_translation(turn, b1_bind);
    let b2_world = b1_world * Affine3A::from_translation(Vec3::new(0.0, 5.0, 0.0));
    let skin_to_bip = if skin_offset { Affine3A::IDENTITY } else { Affine3A::from_translation(shape_at) };
    let bone = |world: Affine3A, weights: Vec<(u16, f32)>| {
        let (rotation, translation) = stored(world.inverse() * skin_to_bip);
        BoneData { rotation, translation, vertex_weights: weights, ..BoneData::default() }
    };
    let (rotation, translation) = stored(skin_to_bip.inverse());
    let data = stream.insert(NiSkinData {
        rotation,
        translation,
        bone_data: vec![
            bone(b1_world, vec![(0, 1.0), (1, 1.0), (2, 0.5), (3, 0.7)]),
            bone(b2_world, vec![(2, 0.5), (3, 0.3)]),
        ],
        ..NiSkinData::default()
    });
    let instance = stream.insert(NiSkinInstance { data, root: bip, bones: vec![b1, b2], ..NiSkinInstance::default() });
    if let Some(g) = stream.get_mut(quad.cast::<NiTriShape>()) {
        g.base.base.skin_instance = instance;
    }
    load(&stream.save_bytes().unwrap()).unwrap()
}

fn close(a: Vec3, b: Vec3) -> bool {
    a.abs_diff_eq(b, 1e-4)
}

#[test]
fn skin_reproduces_the_baked_rest_pose() {
    let rest_bind = Vec3::new(0.0, 0.0, 10.0);
    for (shape_at, b1_bind, offset) in [
        (Vec3::ZERO, rest_bind, false),
        (Vec3::new(2.0, 3.0, 4.0), rest_bind, false),
        (Vec3::new(2.0, 3.0, 4.0), Vec3::new(0.0, 1.0, 8.0), false),
        (Vec3::new(2.0, 3.0, 4.0), Vec3::new(0.0, 1.0, 8.0), true),
    ] {
        let model = skinned(shape_at, b1_bind, offset);
        assert!(model.had_skin);
        let node = model.nodes.iter().position(|n| n.name == "skin").unwrap();
        let mesh = &model.meshes[model.nodes[node].mesh.unwrap()];
        let skin = mesh.skin.as_deref().expect("skin resolves");
        assert!(mesh.skinned);
        assert_eq!(skin.offset_bind, offset);
        let names: Vec<&str> = skin.bones.iter().map(|b| model.nodes[b.node].name.as_str()).collect();
        assert_eq!(names, ["b1", "b2"]);
        assert_eq!(skin.influences.iter().map(Vec::len).collect::<Vec<_>>(), [1, 1, 2, 2]);
        for influences in &skin.influences {
            assert!((influences.iter().map(|i| i.1).sum::<f32>() - 1.0).abs() < 1e-6);
        }
        // The skin's rest pose is what the converter baked.
        let world = world_transforms(&model);
        for (p, baked) in skin.rest_positions(&world).iter().zip(&mesh.positions) {
            let baked = world[node].transform_point3(Vec3::from_array(*baked));
            assert!(close(*p, baked), "{shape_at} {b1_bind} {offset}: {p} vs {baked}");
        }
        // Bind space is the shape's raw vertex data.
        for (p, v) in skin.positions.iter().zip(QUAD) {
            assert!(close(Vec3::from_array(*p), coords::point(v)));
        }
        // Bound in the rest pose with a consistent skin transform, the
        // inverse bind is the plain "shape space -> joint space".
        if b1_bind == rest_bind && !offset {
            for b in &skin.bones {
                assert!(b.inverse_bind.abs_diff_eq(world[b.node].inverse() * world[node], 1e-4));
            }
        }
    }
}

#[test]
fn morph_targets_are_deltas_from_target_0() {
    let mut stream = NiStream::new();
    let quad = shape(&mut stream, "head", Vec3::ZERO, QUAD.to_vec());
    let base: Vec<Vec3> = QUAD.iter().map(|v| *v + Vec3::X).collect();
    let keys = vec![NiLinFloatKey { time: 0.0, value: 0.25 }, NiLinFloatKey { time: 1.0, value: 1.0 }];
    let data = stream.insert(NiMorphData {
        relative_targets: true,
        targets: vec![
            MorphTarget { vertices: base, ..MorphTarget::default() },
            MorphTarget { keys: NiFloatKey::LinKey(keys), vertices: vec![Vec3::Z; 4] },
        ],
        ..NiMorphData::default()
    });
    let controller = stream.insert(NiGeomMorpherController {
        base: NiMorpherController {
            base: NiTimeController { stop_time: 1.0, target: quad.cast(), ..NiTimeController::default() },
            data,
        },
        ..NiGeomMorpherController::default()
    });
    if let Some(g) = stream.get_mut(quad.cast::<NiTriShape>()) {
        g.base.base.base.base.controller = controller.cast();
    }
    let top = node(&mut stream, "Root", Vec3::ZERO, vec![quad], vec![]);
    root(&mut stream, top);
    let model = load(&stream.save_bytes().unwrap()).unwrap();

    let mesh = &model.meshes[0];
    let morph = mesh.morph.as_ref().expect("morph");
    assert_eq!(morph.targets.len(), 1);
    assert!(close(Vec3::from_array(morph.targets[0].deltas[2]), coords::point(Vec3::Z)));
    assert_eq!(morph.rest_weights(), [0.25]);
    assert!((morph.timing.stop - 1.0).abs() < 1e-6);
    // Positions are target 0, not the shape's own vertices.
    assert!(close(Vec3::from_array(mesh.positions[0]), coords::point(Vec3::X)));
    assert!(!mesh.skinned && mesh.skin.is_none());
    assert_eq!(census::support("NiGeomMorpherController"), census::Support::Converted);
}
