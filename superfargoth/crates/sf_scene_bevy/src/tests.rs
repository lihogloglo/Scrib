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

/// `extract_chain_slabs` answers only what is pending (`slabs` module docs): the render world's
/// lookups, faked, with a count of the calls.
mod slab_pending {
    use std::cell::Cell;
    use std::collections::HashSet;

    use bevy::asset::{AssetId, AssetPlugin, Assets};
    use bevy::mesh::{Mesh, MeshPlugin};
    use bevy::pbr::StandardMaterial;
    use bevy::prelude::*;

    use crate::slabs::{ChainSlabTable, MATERIAL_RECHECK_EXTRACTS, MeshSlabIds, Pending, invalidate_chain_slabs};

    const SLABS: MeshSlabIds = MeshSlabIds { vertex_slab: 3, index_slab: 4 };

    fn mesh(n: u128) -> AssetId<Mesh> {
        AssetId::Uuid { uuid: bevy::asset::uuid::Uuid::from_u128(n) }
    }

    fn material(n: u128) -> AssetId<StandardMaterial> {
        AssetId::Uuid { uuid: bevy::asset::uuid::Uuid::from_u128(n) }
    }

    /// One extract with a render world that knows every mesh in `known` (and every material).
    fn extract(table: &mut ChainSlabTable, known: &HashSet<AssetId<Mesh>>, calls: &Cell<u32>) {
        let slabs = |id| {
            calls.set(calls.get() + 1);
            known.contains(&id).then_some(SLABS)
        };
        let group = |_| {
            calls.set(calls.get() + 1);
            Some(7)
        };
        table.answer_pending(Some(&slabs), Some(&group));
    }

    #[test]
    fn answered_ids_are_not_looked_up_again() {
        let mut table = ChainSlabTable { tracking: true, ..default() };
        table.set_wanted(HashSet::from([mesh(1), mesh(2)]), HashSet::from([material(10)]));
        assert!(table.has_pending(), "new wanted ids are pending");
        let (known, calls) = (HashSet::from([mesh(1), mesh(2)]), Cell::new(0));
        extract(&mut table, &known, &calls);
        assert_eq!(calls.get(), 3, "two meshes and one material looked up");
        assert_eq!(table.lookups, 3);
        assert!(!table.has_pending(), "every wanted id answered");
        assert_eq!(table.draw(mesh(1), Some(material(10))).map(|d| d.bind_group), Some(7));
        for _ in 0..5 {
            extract(&mut table, &known, &calls);
        }
        assert_eq!(calls.get(), 3, "steady extracts make no lookup");
        assert_eq!(table.lookups, 3);
        assert_eq!(table.slabs.get(&mesh(2)), Some(&SLABS), "answers stay");
    }

    #[test]
    fn an_unknown_mesh_stays_pending_and_is_retried() {
        let mut table = ChainSlabTable::default();
        table.set_wanted(HashSet::from([mesh(1)]), HashSet::new());
        let calls = Cell::new(0);
        for _ in 0..3 {
            extract(&mut table, &HashSet::new(), &calls);
        }
        assert_eq!(calls.get(), 3, "retried every extract");
        assert!(table.pending.contains_key(&mesh(1)) && !table.slabs.contains_key(&mesh(1)));
        extract(&mut table, &HashSet::from([mesh(1)]), &calls);
        assert_eq!(table.slabs.get(&mesh(1)), Some(&SLABS), "answered once allocated");
        assert!(!table.has_pending());
    }

    #[test]
    fn rewanting_marks_new_ids_pending_and_drops_old_answers() {
        let mut table = ChainSlabTable::default();
        let (known, calls) = (HashSet::from([mesh(1), mesh(2), mesh(3)]), Cell::new(0));
        table.set_wanted(HashSet::from([mesh(1), mesh(2)]), HashSet::from([material(10)]));
        extract(&mut table, &known, &calls);
        table.dirty = true;
        table.set_wanted(HashSet::from([mesh(2), mesh(3)]), HashSet::new());
        assert!(!table.dirty);
        assert!(!table.slabs.contains_key(&mesh(1)), "a mesh no longer wanted loses its answer");
        assert!(table.groups.is_empty(), "a material no longer wanted loses its answer");
        assert_eq!(table.slabs.get(&mesh(2)), Some(&SLABS), "a still-wanted answer stays");
        assert_eq!(table.pending.keys().copied().collect::<Vec<_>>(), vec![mesh(3)], "only the new id is pending");
        let before = calls.get();
        extract(&mut table, &known, &calls);
        assert_eq!(calls.get() - before, 1, "only the new id looked up");
        assert_eq!(table.slabs.get(&mesh(3)), Some(&SLABS));
        assert!(!table.has_pending());
    }

    /// The main-world reader: a `Modified` wanted material (and mesh) goes back to pending,
    /// held over the extract of the frame it changed, then looked up again.
    #[test]
    fn a_modified_material_or_mesh_is_looked_up_again() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
            .init_asset::<StandardMaterial>()
            .init_resource::<ChainSlabTable>()
            .add_systems(Last, invalidate_chain_slabs);
        let mat = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
        let m = app.world_mut().resource_mut::<Assets<Mesh>>().add(Mesh::from(Cuboid::default()));
        app.update(); // the `Added` events pass before the ids are wanted
        let (known, calls) = (HashSet::from([m.id()]), Cell::new(0));
        {
            let mut table = app.world_mut().resource_mut::<ChainSlabTable>();
            table.set_wanted(HashSet::from([m.id()]), HashSet::from([mat.id()]));
            extract(&mut table, &known, &calls);
            assert!(!table.has_pending());
        }
        app.update();
        assert!(!app.world().resource::<ChainSlabTable>().has_pending(), "no event, nothing pending");

        // `AssetMut` emits `Modified` on a mutable borrow only.
        app.world_mut().resource_mut::<Assets<StandardMaterial>>().get_mut(mat.id()).unwrap().perceptual_roughness = 0.4;
        app.world_mut().resource_mut::<Assets<Mesh>>().get_mut(m.id()).unwrap().translate_by(Vec3::ZERO);
        app.update();
        let mut table = app.world_mut().resource_mut::<ChainSlabTable>();
        assert!(!table.groups.contains_key(&mat.id()) && !table.slabs.contains_key(&m.id()), "answers dropped");
        assert_eq!(table.pending_materials.get(&mat.id()), Some(&Pending { hold: 1, recheck: MATERIAL_RECHECK_EXTRACTS }));
        assert_eq!(table.pending.get(&m.id()), Some(&Pending { hold: 1, recheck: 0 }));
        let before = calls.get();
        extract(&mut table, &known, &calls);
        assert_eq!(calls.get(), before, "this frame's extract still sees the old allocation: held");
        extract(&mut table, &known, &calls);
        assert_eq!(calls.get(), before + 2, "looked up again on the next extract");
        assert_eq!(table.slabs.get(&m.id()), Some(&SLABS));
        assert_eq!(table.groups.get(&mat.id()), Some(&7));
        assert!(table.pending.is_empty(), "the mesh is settled");
        assert!(table.pending_materials.contains_key(&mat.id()), "the material is rechecked while it may move");
        for _ in 0..MATERIAL_RECHECK_EXTRACTS {
            extract(&mut table, &known, &calls);
        }
        assert!(!table.has_pending(), "the recheck window ends");
        assert_eq!(table.groups.get(&mat.id()), Some(&7));
    }

    /// An event for an id no chain wants changes nothing.
    #[test]
    fn events_for_unwanted_ids_are_ignored() {
        let mut table = ChainSlabTable::default();
        table.invalidate_mesh(mesh(1));
        table.invalidate_material(material(10));
        assert!(!table.has_pending());
    }
}
