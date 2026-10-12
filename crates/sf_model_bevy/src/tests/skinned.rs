//! The `Cooked::Skinned` → Bevy mapping (bake.md "Bevy mapping").

use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::{AssetPlugin, AssetServer, Assets, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_ecs::world::CommandQueue;
use bevy_math::{Mat4, Vec3};
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::{Mesh, MeshPlugin, VertexAttributeValues};
use sf_model::{
    CookSettings, Cooked, CookedJoint, CookedSkinned, Material, MeshData, MeshSkin, Model,
    MorphKind, MorphTarget, Node, SkinJoint, cook,
};

use super::super::*;

/// A 4×16-quad strip along +Y skinned to two joints ("root" at 0, "tip" at
/// 1 m) with a linear blend, plus an expression target ("blink", moves the
/// top row) and a shape target ("fat", default weight 0.5, moves every
/// vertex along +X).
fn skinned_model() -> Model {
    let (nx, ny) = (4u32, 16u32);
    let mut mesh = MeshData {
        normals: Some(Vec::new()),
        uvs: Some(Vec::new()),
        skinned: true,
        ..MeshData::default()
    };
    let mut influences = Vec::new();
    for y in 0..=ny {
        for x in 0..=nx {
            #[allow(clippy::cast_precision_loss)]
            let (fx, fy) = (x as f32 / nx as f32, y as f32 / ny as f32);
            mesh.positions
                .push([fx * 0.25, fy * 2.0, 0.05 * (fx * 9.0).sin()]);
            mesh.normals.as_mut().unwrap().push([0.0, 0.0, 1.0]);
            mesh.uvs.as_mut().unwrap().push([fx, fy]);
            influences.push(vec![(0, 1.0 - fy), (1, fy)]);
        }
    }
    let row = nx + 1;
    for y in 0..ny {
        for x in 0..nx {
            let i = y * row + x;
            mesh.indices
                .extend([i, i + 1, i + row, i + 1, i + row + 1, i + row]);
        }
    }
    let n = mesh.positions.len();
    let top = (n - row as usize)..n;
    let blink = (0..n)
        .map(|v| {
            if top.contains(&v) {
                [0.0, 0.0, 0.1]
            } else {
                [0.0; 3]
            }
        })
        .collect();
    mesh.morph_targets = vec![
        MorphTarget {
            name: "blink".into(),
            positions: blink,
            normals: None,
            default_weight: 0.0,
            kind: MorphKind::Expression,
        },
        MorphTarget {
            name: "fat".into(),
            positions: vec![[0.02, 0.0, 0.0]; n],
            normals: None,
            default_weight: 0.5,
            kind: MorphKind::Shape,
        },
    ];
    let ib = |y: f32| Mat4::from_translation(Vec3::new(0.0, -y, 0.0)).to_cols_array();
    mesh.skin = Some(MeshSkin {
        joints: vec![
            SkinJoint {
                node: 1,
                inverse_bind: ib(0.0),
            },
            SkinJoint {
                node: 2,
                inverse_bind: ib(1.0),
            },
        ],
        influences,
    });
    let mut strip = Node::new("strip");
    strip.mesh = Some(0);
    let tip = Node {
        parent: Some(1),
        translation: [0.0, 1.0, 0.0],
        ..Node::new("tip")
    };
    Model {
        nodes: vec![strip, Node::new("root"), tip],
        meshes: vec![mesh],
        materials: vec![Material::default()],
    }
}

fn cooked() -> CookedSkinned {
    match cook(&skinned_model(), &CookSettings::default()).unwrap() {
        Cooked::Skinned(s) => s,
        other => panic!("not skinned: {other:?}"),
    }
}

#[test]
fn skinned_submeshes_map_to_bevy_skin_attributes() {
    let skinned = cooked();
    assert_eq!(
        skinned
            .joints
            .iter()
            .map(|j| j.name.as_str())
            .collect::<Vec<_>>(),
        ["root", "tip"]
    );
    let (mesh, info) =
        skinned_submesh_to_mesh(&skinned, 0, 0, RenderAssetUsages::all(), false).unwrap();
    let n = mesh.count_vertices();
    let Some(VertexAttributeValues::Uint16x4(joints)) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
    else {
        panic!("joint indices not Uint16x4")
    };
    assert_eq!(joints.len(), n);
    assert!(
        joints
            .iter()
            .flatten()
            .all(|&j| usize::from(j) < skinned.joints.len())
    );
    let Some(VertexAttributeValues::Float32x4(weights)) =
        mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
    else {
        panic!("joint weights not Float32x4")
    };
    for w in weights {
        assert!(
            (w.iter().sum::<f32>() - 1.0).abs() < 1e-5,
            "weights {w:?} not normalized"
        );
        assert!(w.iter().all(|&x| x >= 0.0));
    }
    assert!(
        mesh.skinned_mesh_bounds().is_some(),
        "DynamicSkinnedMeshBounds needs SkinnedMeshBounds"
    );
    // The shape target is baked; only the expression target reaches Bevy.
    let blink = skinned
        .morph_targets
        .iter()
        .position(|t| t.name == "blink")
        .unwrap();
    assert_eq!(info.targets, [u16::try_from(blink).unwrap()]);
    assert_eq!(
        mesh.morph_target_names().map(<[String]>::to_vec),
        Some(vec!["blink".to_owned()])
    );
    assert!(mesh.has_morph_targets());
    // Baked at the default weight: x moved by 0.5 × 0.02.
    let raw = skinned.decode_submesh(0, 0).unwrap().vertices;
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("positions not Float32x3")
    };
    for (p, raw) in positions.iter().zip(&raw) {
        assert!((p[0] - raw.position[0] - 0.01).abs() < 1e-5);
    }
    // Compressed (Bevy quantizes the weights): still skinned, bounds kept.
    let (compressed, _) =
        skinned_submesh_to_mesh(&skinned, 0, 0, RenderAssetUsages::all(), true).unwrap();
    assert!(compressed.attribute(Mesh::ATTRIBUTE_JOINT_INDEX).is_some());
    assert!(compressed.skinned_mesh_bounds().is_some());
}

#[test]
#[allow(clippy::float_cmp)] // exact copies, not computed values
fn morph_blocks_are_target_major() {
    let skinned = cooked();
    let mut sub = skinned.decode_submesh(0, 0).unwrap();
    sub.bake_shapes(&skinned.morph_targets, |_| 0.5);
    let attributes = morph_attributes(&sub);
    assert_eq!(attributes.len(), sub.targets.len() * sub.vertices.len());
    for (a, d) in attributes.iter().zip(&sub.morphs) {
        assert_eq!(a.position.to_array(), d.position);
        assert_eq!(a.normal.to_array(), d.normal);
        assert_eq!(a.tangent, Vec3::ZERO);
    }
    assert!(
        attributes.iter().any(|a| (a.position.z - 0.1).abs() < 1e-6),
        "blink moves the top row"
    );
    assert_eq!(gather_weights(&[1, 0, 7], &[0.25, 0.75]), [0.75, 0.25, 0.0]);
}

#[test]
fn joint_palettes_over_the_bevy_limit_fail_loudly() {
    let mut skinned = cooked();
    assert_eq!(inverse_bindposes(&skinned).unwrap().len(), 2);
    let joint = CookedJoint {
        name: "extra".into(),
        inverse_bind: Mat4::IDENTITY.to_cols_array(),
    };
    skinned.joints.resize(MAX_JOINTS + 1, joint);
    assert!(matches!(
        inverse_bindposes(&skinned),
        Err(SfMeshLoadError::Limit(_))
    ));
}

#[test]
fn loaded_skinned_models_spawn_bound_by_joint_name() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("strip.sfmesh"),
        Cooked::Skinned(cooked()).to_bytes(),
    )
    .unwrap();
    let mut app = App::new();
    let file_path = dir.path().to_string_lossy().into_owned();
    app.add_plugins((
        TaskPoolPlugin::default(),
        AssetPlugin {
            file_path,
            ..Default::default()
        },
        MeshPlugin,
        SfModelPlugin,
    ));
    let handle = app
        .world()
        .resource::<AssetServer>()
        .load::<CookedModelAsset>("strip.sfmesh");
    for _ in 0..10_000 {
        app.update();
        if app
            .world()
            .resource::<AssetServer>()
            .is_loaded_with_dependencies(&handle)
        {
            break;
        }
        std::thread::yield_now();
    }
    let asset = app
        .world_mut()
        .resource_mut::<Assets<CookedModelAsset>>()
        .remove(&handle)
        .expect("loaded");
    let skin = asset.skin.clone().expect("skinned asset");
    assert!(asset.skipped.is_none() && asset.ray_tracing.is_none());
    assert_eq!(skin.meshes.len(), asset.meshes.len());

    let model = skinned_model();
    let world = app.world_mut();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    let root = commands.spawn_empty().id();
    let skeleton = spawn_rest_skeleton(&mut commands, root, &model);
    let missing =
        resolve_joints(&skin, |name| (name != "tip").then(|| skeleton[name])).unwrap_err();
    assert_eq!(missing.0, ["tip"]);
    let joints = resolve_joints(&skin, |name| skeleton.get(name).copied()).unwrap();
    let meshes = spawn_skinned(&mut commands, root, &asset, 0, &joints, |_, _| {});
    queue.apply(world);
    assert!(!meshes.is_empty());
    for &e in &meshes {
        assert_eq!(world.get::<SkinnedMesh>(e).unwrap().joints, joints);
    }
    // Live weights: the root's model-wide vector reaches the morphing mesh.
    let blink = skin
        .morph_targets
        .iter()
        .position(|t| t.name == "blink")
        .unwrap();
    world.get_mut::<ModelMorphWeights>(root).unwrap().0[blink] = 1.0;
    app.update();
    let world = app.world();
    let weights: Vec<Vec<f32>> = meshes
        .iter()
        .filter_map(|&e| match world.get::<MeshMorphWeights>(e) {
            Some(MeshMorphWeights::Value { weights }) => Some(weights.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(weights, [vec![1.0]]);
}
