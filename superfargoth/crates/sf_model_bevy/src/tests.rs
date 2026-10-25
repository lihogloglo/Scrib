use std::collections::BTreeMap;
use std::path::Path;

use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::io::{AssetReader, Reader};
use bevy_asset::{AssetApp, AssetPlugin, AssetServer, Assets, RenderAssetUsages};
use bevy_mesh::{Indices, Mesh, MeshPlugin};
use futures_lite::future::block_on;
use sf_bake::{Manifest, ProductKey, Store};
use sf_model::{CookSettings, Cooked, Material, MeshData, Model, Node, cook};

use super::*;

/// A 32×32-quad wavy grid: enough triangles for a LOD chain.
fn grid_model() -> Model {
    let n = 32u32;
    let mut mesh = MeshData {
        normals: Some(Vec::new()),
        uvs: Some(Vec::new()),
        ..MeshData::default()
    };
    for z in 0..=n {
        for x in 0..=n {
            #[allow(clippy::cast_precision_loss)]
            let (fx, fz) = (x as f32 / n as f32, z as f32 / n as f32);
            mesh.positions.push([
                fx * 4.0,
                0.3 * (fx * 8.0).sin() * (fz * 6.0).cos(),
                fz * 4.0,
            ]);
            mesh.normals.as_mut().unwrap().push([0.0, 1.0, 0.0]);
            mesh.uvs.as_mut().unwrap().push([fx, fz]);
        }
    }
    let row = n + 1;
    for z in 0..n {
        for x in 0..n {
            let i = z * row + x;
            mesh.indices
                .extend([i, i + row, i + 1, i + 1, i + row, i + row + 1]);
        }
    }
    let mut node = Node::new("grid");
    node.mesh = Some(0);
    Model {
        nodes: vec![node],
        meshes: vec![mesh],
        materials: vec![Material::default()],
    }
}

fn cooked_bytes() -> Vec<u8> {
    cook(&grid_model(), &CookSettings::default())
        .unwrap()
        .to_bytes()
}

#[test]
fn submeshes_become_compressed_bevy_meshes() {
    let Cooked::Static(model) = Cooked::from_bytes(&cooked_bytes()).unwrap() else {
        panic!("skipped")
    };
    let sub = &model.lods[0].submeshes[0];
    let mesh = submesh_to_mesh(&model, sub, RenderAssetUsages::all(), true).unwrap();
    assert_eq!(mesh.count_vertices(), sub.vertex_count as usize);
    // Few vertices: Bevy's compression narrows indices to 16 bits.
    assert!(matches!(mesh.indices(), Some(Indices::U16(i)) if i.len() == sub.index_count as usize));
    let plain = submesh_to_mesh(&model, sub, RenderAssetUsages::all(), false).unwrap();
    assert!(matches!(plain.indices(), Some(Indices::U32(_))));
    assert!(plain.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
}

#[test]
fn ray_tracing_meshes_have_exactly_the_solari_layout() {
    let Cooked::Static(model) = Cooked::from_bytes(&cooked_bytes()).unwrap() else {
        panic!("skipped")
    };
    let sub = &model.lods[0].submeshes[0];
    let mesh = submesh_to_ray_tracing_mesh(&model, sub).unwrap();
    // Bevy Solari 0.20 `is_mesh_raytracing_compatible`: these four, in this
    // order, at their default formats; u32 indices; triangle list.
    let layout: Vec<_> = mesh.attributes().map(|(a, _)| (a.id, a.format)).collect();
    let expected: Vec<_> = [
        Mesh::ATTRIBUTE_POSITION,
        Mesh::ATTRIBUTE_NORMAL,
        Mesh::ATTRIBUTE_UV_0,
        Mesh::ATTRIBUTE_TANGENT,
    ]
    .iter()
    .map(|a| (a.id, a.format))
    .collect();
    assert_eq!(layout, expected);
    assert!(matches!(mesh.indices(), Some(Indices::U32(i)) if i.len() == sub.index_count as usize));
    assert!(mesh.enable_raytracing);
}

/// A store holding one cooked product, plus its manifest.
fn store_with_product(dir: &Path) -> (Store, BakedManifest) {
    let store = Store::open(dir).unwrap();
    let digest = store.put(&cooked_bytes(), true).unwrap();
    let mut manifest = Manifest::default();
    manifest
        .products
        .insert(ProductKey::new("mesh.cook", "meshes/grid.nif"), digest);
    (store, BakedManifest::new(manifest))
}

fn extensions() -> BTreeMap<String, String> {
    BTreeMap::from([("mesh.cook".to_owned(), "sfmesh".to_owned())])
}

#[test]
fn reader_resolves_products_through_the_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let (store, manifest) = store_with_product(dir.path());
    let reader = BakedReader::new(store, manifest.clone(), extensions());
    block_on(async {
        let mut bytes = Vec::new();
        let mut r = reader
            .read(Path::new("mesh.cook/meshes/grid.nif.sfmesh"))
            .await
            .unwrap();
        r.read_to_end(&mut bytes).await.unwrap();
        assert_eq!(bytes, cooked_bytes());
        assert!(
            reader
                .read(Path::new("mesh.cook/meshes/other.nif.sfmesh"))
                .await
                .is_err()
        );
        assert!(reader.read(Path::new("no-step")).await.is_err());
        // Swapping the manifest takes effect immediately.
        manifest.replace(Manifest::default());
        assert!(
            reader
                .read(Path::new("mesh.cook/meshes/grid.nif.sfmesh"))
                .await
                .is_err()
        );
    });
    assert_eq!(
        baked_path("mesh.cook", "a/b.nif", Some("sfmesh")),
        "baked://mesh.cook/a/b.nif.sfmesh"
    );
}

#[test]
fn app_loads_a_cooked_model_from_baked_source() {
    let dir = tempfile::tempdir().unwrap();
    let (store, manifest) = store_with_product(dir.path());
    let mut app = App::new();
    app.register_asset_source(BAKED_SOURCE, baked_source(store, manifest, extensions()));
    app.add_plugins((
        TaskPoolPlugin::default(),
        AssetPlugin::default(),
        MeshPlugin,
        SfModelPlugin,
    ));

    let handle = app
        .world()
        .resource::<AssetServer>()
        .load::<CookedModelAsset>(baked_path("mesh.cook", "meshes/grid.nif", Some("sfmesh")));
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
    let models = app.world().resource::<Assets<CookedModelAsset>>();
    let model = models.get(&handle).expect("cooked model loaded");
    assert!(model.skipped.is_none());
    assert!(model.lods.len() >= 2, "{} levels", model.lods.len());
    let meshes = app.world().resource::<Assets<Mesh>>();
    for lod in &model.lods {
        for (mesh, _) in &lod.parts {
            assert!(meshes.get(&model.meshes[*mesh]).is_some());
        }
    }
}

#[test]
fn gpu_only_meshes_keep_their_bounds() {
    let Cooked::Static(model) = Cooked::from_bytes(&cooked_bytes()).unwrap() else {
        panic!("skipped")
    };
    let sub = &model.lods[0].submeshes[0];
    // What extraction does to GPU-only meshes: drop the CPU data and keep
    // only the AABB computed here. (Meshes with a CPU copy are never taken.)
    let mut mesh = submesh_to_mesh(&model, sub, RenderAssetUsages::RENDER_WORLD, true).unwrap();
    let gpu = mesh.take_gpu_data().unwrap();
    assert!(
        gpu.final_aabb.is_some(),
        "bounds lost: the mesh would render collapsed"
    );
}

mod skinned;
