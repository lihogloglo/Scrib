#![allow(clippy::float_cmp)] // exact values written by the test itself
use super::*;

use tes3::nif::glam::{Mat3, Vec2, Vec4};
use tes3::nif::{
    Map, NiAlphaProperty, NiExtraData, NiMaterialProperty, NiObject, NiObjectNET, NiSourceTexture, NiTexture,
    NiTexturingProperty, NiTriBasedGeom, NiTriBasedGeomData, NiTriShape, TextureMap, TextureSource,
};

fn named(name: &str) -> NiObjectNET {
    NiObjectNET { name: name.into(), ..NiObjectNET::default() }
}

fn av(name: &str, translation: Vec3, properties: Vec<NiLink<NiProperty>>) -> NiAVObject {
    NiAVObject { base: named(name), translation, rotation: Mat3::IDENTITY, properties, ..NiAVObject::default() }
}

/// A unit quad (two triangles) at z = 0 in MW space.
fn quad(stream: &mut NiStream, name: &str, properties: Vec<NiLink<NiProperty>>) -> NiLink<NiAVObject> {
    let data = NiTriShapeData {
        base: NiTriBasedGeomData {
            base: NiGeometryData {
                vertices: vec![Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0), Vec3::Y],
                normals: vec![Vec3::Z; 4],
                uv_sets: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
                vertex_colors: vec![Vec4::ONE; 4],
                ..NiGeometryData::default()
            },
        },
        triangles: vec![[0, 1, 2], [0, 2, 3]],
        ..NiTriShapeData::default()
    };
    let data = stream.insert(data);
    let shape = NiTriShape {
        base: NiTriBasedGeom {
            base: NiGeometry { base: av(name, Vec3::ZERO, properties), geometry_data: data.cast(), ..NiGeometry::default() },
        },
    };
    stream.insert(shape).cast()
}

fn texturing(stream: &mut NiStream, file: &str) -> NiLink<NiProperty> {
    let source = stream.insert(NiSourceTexture {
        base: NiTexture { base: named(file) },
        source: TextureSource::External(file.into()),
        ..NiSourceTexture::default()
    });
    let map = Map { texture: source, ..Map::default() };
    stream
        .insert(NiTexturingProperty { texture_maps: vec![Some(TextureMap::Map(map))], ..NiTexturingProperty::default() })
        .cast()
}

fn node(stream: &mut NiStream, name: &str, translation: Vec3, children: Vec<NiLink<NiAVObject>>, properties: Vec<NiLink<NiProperty>>) -> NiLink<NiAVObject> {
    stream.insert(NiNode { base: av(name, translation, properties), children, ..NiNode::default() }).cast()
}

fn root(stream: &mut NiStream, link: NiLink<NiAVObject>) {
    stream.roots.push(link.cast::<NiObject>());
}

#[test]
fn properties_inherit_and_materials_dedupe() {
    let mut stream = NiStream::new();
    let wood = texturing(&mut stream, "Tx_Wood.tga");
    let a = quad(&mut stream, "a", vec![]);
    let b = quad(&mut stream, "b", vec![]);
    let stone = texturing(&mut stream, "tx_stone.dds");
    let c = quad(&mut stream, "c", vec![stone]);
    let top = node(&mut stream, "Root", Vec3::ZERO, vec![a, b, c], vec![wood]);
    root(&mut stream, top);

    let model = convert(stream).unwrap();
    assert_eq!(model.meshes.len(), 3);
    assert_eq!(model.materials.len(), 2, "a and b share the inherited wood material");
    assert_eq!(model.meshes[0].material, model.meshes[1].material);
    assert_eq!(model.materials[model.meshes[0].material].base_texture.as_deref(), Some("textures/tx_wood.tga"));
    assert_eq!(model.materials[model.meshes[2].material].base_texture.as_deref(), Some("textures/tx_stone.dds"));
}

#[test]
fn geometry_is_in_engine_space() {
    let mut stream = NiStream::new();
    let shape = quad(&mut stream, "q", vec![]);
    // Node 64 units north and 64 up.
    let top = node(&mut stream, "Root", Vec3::new(0.0, 64.0, 64.0), vec![shape], vec![]);
    root(&mut stream, top);
    let model = convert(stream).unwrap();

    let mesh = &model.meshes[0];
    // MW +Z normal is engine +Y.
    assert!(mesh.normals.iter().all(|n| (n[1] - 1.0).abs() < 1e-6));
    // MW (0, 1, 0) is engine (0, 0, -unit).
    let unit = coords::UNIT_METERS;
    assert!((mesh.positions[3][2] + unit).abs() < 1e-6);
    let t = model.nodes[0].transform.translation;
    assert!((t[1] - 0.9144).abs() < 1e-5 && (t[2] + 0.9144).abs() < 1e-5, "{t:?}");
    let (lo, hi) = visible_bounds(&model).unwrap();
    assert!((hi[1] - lo[1]).abs() < 1e-6, "flat quad lies on a horizontal plane");
}

#[test]
fn visibility_rules() {
    let mut stream = NiStream::new();
    let shown = quad(&mut stream, "shown", vec![]);
    let marker = quad(&mut stream, "Tri EditorMarker 01", vec![]);
    let hidden = quad(&mut stream, "hidden", vec![]);
    if let Some(av) = stream.get_as_mut::<_, NiAVObject>(hidden) {
        av.flags |= 1; // app culled
    }
    let collider = quad(&mut stream, "collider", vec![]);
    let collision: NiLink<NiAVObject> = stream
        .insert(RootCollisionNode { base: NiNode { base: av("RootCollisionNode", Vec3::ZERO, vec![]), children: vec![collider], ..NiNode::default() } })
        .cast();
    let off = quad(&mut stream, "off", vec![]);
    let on = quad(&mut stream, "on", vec![]);
    let switch: NiLink<NiAVObject> = stream
        .insert(NiSwitchNode { base: NiNode { base: av("switch", Vec3::ZERO, vec![]), children: vec![off, on], ..NiNode::default() }, active_index: 1 })
        .cast();
    let top = node(&mut stream, "Root", Vec3::ZERO, vec![shown, marker, hidden, collision, switch], vec![]);
    root(&mut stream, top);

    let model = convert(stream).unwrap();
    let visible: Vec<&str> = model.nodes.iter().filter(|n| n.visible && n.mesh.is_some()).map(|n| n.name.as_str()).collect();
    assert_eq!(visible, ["shown", "on"]);
    let collider = model.nodes.iter().find(|n| n.name == "collider").unwrap();
    assert!(model.meshes[collider.mesh.unwrap()].collision);
    assert!(model.nodes.iter().any(|n| n.kind == NodeKind::Collision));
    assert!(model.nodes.iter().any(|n| n.kind == NodeKind::Switch { active: 1 }));
}

#[test]
fn alpha_and_material_properties() {
    let mut stream = NiStream::new();
    let mut alpha = NiAlphaProperty::default();
    alpha.base.flags = 0x0001 | 0x0200; // blending + testing
    alpha.test_ref = 128;
    let alpha: NiLink<NiProperty> = stream.insert(alpha).cast();
    let material: NiLink<NiProperty> = stream
        .insert(NiMaterialProperty { emissive_color: Vec3::new(1.0, 0.5, 0.0), alpha: 0.5, ..NiMaterialProperty::default() })
        .cast();
    let shape = quad(&mut stream, "leaf", vec![alpha, material]);
    root(&mut stream, shape);

    let model = convert(stream).unwrap();
    let m = &model.materials[0];
    assert_eq!(m.alpha_mode, AlphaMode::Mask { cutoff: 128.0 / 255.0 });
    assert_eq!(m.emissive, [1.0, 0.5, 0.0]);
    assert!((m.alpha - 0.5).abs() < f32::EPSILON);
}

#[test]
fn marker_extra_data_is_detected() {
    let mut stream = NiStream::new();
    let extra = stream.insert(NiStringExtraData { base: NiExtraData::default(), value: "MRK".into() });
    let shape = quad(&mut stream, "q", vec![]);
    let top = node(&mut stream, "Root", Vec3::ZERO, vec![shape], vec![]);
    if let Some(av) = stream.get_as_mut::<_, NiAVObject>(top) {
        av.base.extra_data = extra.cast();
    }
    root(&mut stream, top);
    let model = convert(stream).unwrap();
    assert!(model.marker);
    assert!(!model.no_collision);
}

#[test]
fn survives_a_save_load_round_trip() {
    let mut stream = NiStream::new();
    let wood = texturing(&mut stream, "tx_wood.tga");
    let shape = quad(&mut stream, "q", vec![wood]);
    let top = node(&mut stream, "Root", Vec3::ZERO, vec![shape], vec![]);
    root(&mut stream, top);
    let bytes = stream.save_bytes().unwrap();

    let model = load(&bytes).unwrap();
    assert_eq!(model.meshes.len(), 1);
    assert_eq!(model.meshes[0].indices.len(), 6);
    assert_eq!(model.materials[0].base_texture.as_deref(), Some("textures/tx_wood.tga"));
}

#[test]
fn strips_become_triangles() {
    // 0-1-2-3 strip: (0,1,2), (1,3,2); a degenerate repeat is dropped.
    assert_eq!(strip_triangles(&[4], &[0, 1, 2, 3]), [0, 1, 2, 1, 3, 2]);
    assert_eq!(strip_triangles(&[3, 3], &[0, 0, 1, 2, 3, 4]), [2, 3, 4]);
}
