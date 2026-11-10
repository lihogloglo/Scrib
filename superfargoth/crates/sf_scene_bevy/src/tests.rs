//! Headless loader tests: a baked cell table off the `baked://` source, through
//! the extensions map and the extension-keyed loader, into the owned asset and
//! back out as a borrowing view.
//!
//! Harness template: `sf_world_bevy/src/tests.rs`'s `streaming_app` (a `Store`,
//! a `Manifest` and `baked_source` over them, `MinimalPlugins` + `AssetPlugin`,
//! no render app).

use std::collections::BTreeMap;
use std::time::Duration;

use bevy::asset::{AssetPlugin, AssetServer, Assets, Handle, LoadState};
use bevy::prelude::*;
use glam::{Affine3A, IVec3, Vec3};
use sf_bake::{Manifest, ProductKey, Store};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_scene::{
    Aabb, CELLS_EXT, CELLS_STEP, InstanceFlags, LodChain, LodLevel, MaterialClass, Sphere, StaticCellTable,
    StaticInstance, cell_subject,
};

use super::*;

/// `n` instances of one chain (one model, one LOD level), placement ids `0..n`.
fn table(n: u32) -> StaticCellTable {
    let mut instances = Vec::new();
    for (placement, x) in (0..n).zip((0u16..).step_by(5)) {
        let center = Vec3::new(f32::from(x), 1.0, 2.0);
        instances.push(StaticInstance {
            placement,
            transform: StaticInstance::rows(&Affine3A::from_translation(center)),
            sphere: Sphere { center, radius: 3.0 },
            chain: 0,
            material_override: 0,
            flags: InstanceFlags::CASTS_SHADOW,
        });
    }
    StaticCellTable {
        origin_cell: IVec3::new(2, 0, -5),
        aabb: Aabb { min: Vec3::splat(-128.0), max: Vec3::splat(128.0) },
        land_min: -1.5,
        layers: 1,
        instances,
        chains: vec![LodChain {
            mesh: 0x55,
            material: 0xA5A5,
            material_class: MaterialClass::Opaque,
            model: 0,
            part: 0,
            levels: vec![LodLevel { first_index: 0, index_count: 600, error: 0.0 }],
        }],
        models: vec!["meshes/a.mdl".into()],
    }
}

/// An app serving `bytes` as the `scene.cells` product of `ws/0,0,0`, with the
/// scene plugin's asset and loader registered. Returns it and the asset path.
fn loader_app(dir: &std::path::Path, bytes: &[u8]) -> (App, String) {
    let store = Store::open(dir).unwrap();
    let subject = cell_subject("ws", IVec3::ZERO);
    let mut manifest = Manifest::default();
    manifest.products.insert(ProductKey::new(CELLS_STEP, subject.clone()), store.put(bytes, true).unwrap());

    let mut app = App::new();
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(store, BakedManifest::new(manifest), BTreeMap::from_iter(baked_extensions())),
    );
    app.add_plugins((MinimalPlugins, AssetPlugin::default())).add_plugins(ScenePlugin::default());
    app.finish();
    app.cleanup();
    (app, sf_model_bevy::baked_path(CELLS_STEP, &subject, Some(CELLS_EXT)))
}

/// Pumps frames until the load resolved, and returns how it resolved.
fn settle(app: &mut App, handle: &Handle<SceneTableAsset>) -> LoadState {
    for _ in 0..2000 {
        app.update();
        match app.world().resource::<AssetServer>().get_load_state(handle.id()) {
            Some(LoadState::NotLoaded | LoadState::Loading) | None => {}
            Some(state) => return state,
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("the load never resolved");
}

/// The whole path: the extensions map strips `.sfscene` for `BakedReader`, the
/// extension picks `SceneTableLoader`, the asset owns the bytes verbatim and
/// its view borrows them back.
#[test]
fn the_cells_loader_parses_a_baked_table() {
    let dir = tempfile::tempdir().unwrap();
    let bytes = table(7).write().unwrap();
    let (mut app, path) = loader_app(dir.path(), &bytes);
    let handle: Handle<SceneTableAsset> = app.world().resource::<AssetServer>().load(path);
    assert!(matches!(settle(&mut app, &handle), LoadState::Loaded), "load failed");

    let assets = app.world().resource::<Assets<SceneTableAsset>>();
    let asset = assets.get(&handle).expect("loaded");
    assert_eq!(asset.0, bytes, "the bytes are kept verbatim");
    assert_eq!(asset.bytes(), bytes.len() as u64);
    let view = asset.view().expect("a table the loader parsed parses again");
    assert_eq!(view.instances().len(), 7);
    assert_eq!(view.instances().map(|i| i.placement).collect::<Vec<_>>(), (0..7).collect::<Vec<_>>());
    assert_eq!(view.origin_cell(), IVec3::new(2, 0, -5));
}

/// Validation in the loader (static-scene.md §5.3, first two rows): a corrupt
/// table is Bevy's own `LoadState::Failed`, decided on the loader task, so the
/// runtime needs no second error path.
#[test]
fn a_corrupt_table_fails_the_load() {
    let dir = tempfile::tempdir().unwrap();
    let mut bytes = table(3).write().unwrap();
    // Truncating the last instance record leaves the header's counts lying.
    bytes.truncate(bytes.len() - 8);
    let (mut app, path) = loader_app(dir.path(), &bytes);
    let handle: Handle<SceneTableAsset> = app.world().resource::<AssetServer>().load(path);
    assert!(matches!(settle(&mut app, &handle), LoadState::Failed(_)), "a short table must not load");
    assert!(app.world().resource::<Assets<SceneTableAsset>>().get(&handle).is_none());
}
