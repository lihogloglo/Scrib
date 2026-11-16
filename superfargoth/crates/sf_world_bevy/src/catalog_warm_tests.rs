//! Pipeline warm-up W4b1 (`pipeline-warmup.md` §5.3, §5.4): the catalog warm-up requests exactly
//! the catalog's representatives (with the entry flags, through streaming's material path) under
//! [`CATALOG_TAG`], a background tag that never holds the reveal (W4b3); the reveal's shown set
//! ([`RESIDENT_TAG`]) does.
//!
//! GPU-free and exact: cooked models are added to `Assets<CookedModelAsset>` directly and handed
//! out by a stand-in [`CatalogWarmup::loader`]; the render world's status report is played by
//! [`Precache::report`] (as `PrecachePlugin`'s `report_status` does).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::asset::AssetPlugin;
use bevy::mesh::{MeshPlugin, PrimitiveTopology};
use bevy::camera::visibility::VisibilityRange;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use sf_model::catalog::{Catalog, CatalogEntry, EntityFlags, MaterialSignature, Representative, VertexLayout};
use sf_model_bevy::{CookedLodMeshes, CookedModelAsset, CookedSkin};

use crate::{
    BuiltinOutcome, BuiltinSpecimens, CATALOG_TAG, CatalogWarmup, ModelMaterials, Precache, PrecacheStatus, RESIDENT_TAG, ResidentWarmup, RevealAsk, RevealCheck,
    RevealGate, RevealStatus, SETTLE_FRAMES, Specimen, SpecimenFlags, Streaming, StreamingConfig, WarmSpecimen, standard_material,
};

fn desc(name: &str) -> sf_model::Material {
    sf_model::Material { name: name.to_owned(), ..sf_model::Material::default() }
}

fn entry(model: &str, layout: VertexLayout, flags: EntityFlags, submesh: u32, material: u32) -> CatalogEntry {
    CatalogEntry {
        layout,
        material: MaterialSignature::new("standard"),
        flags,
        representative: Representative { step: "mesh.cook".to_owned(), model: model.to_owned(), lod: 0, submesh, material },
        uses: 1,
    }
}

/// Three representatives (a static shadow caster's second submesh, a crossfading non-caster, a
/// skinned morphed caster) and one with no asset path.
fn catalog() -> Catalog {
    let skinned = VertexLayout { uvs: true, skinned: true, morphs: true, ..VertexLayout::default() };
    Catalog {
        entries: vec![
            entry("rock.nif", VertexLayout { uvs: true, ..VertexLayout::default() }, EntityFlags { shadow_caster: true, crossfade: false }, 1, 1),
            entry("tree.nif", VertexLayout::default(), EntityFlags { shadow_caster: false, crossfade: true }, 0, 0),
            entry("body.nif", skinned, EntityFlags { shadow_caster: true, crossfade: false }, 0, 0),
            entry("missing.nif", VertexLayout::default(), EntityFlags::default(), 0, 0),
        ],
        ..Catalog::default()
    }
}

fn mesh() -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, bevy::asset::RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0f32; 3]; 3])
}

/// A cooked model: one mesh per part, `materials` named `<model>/<i>`.
fn model(meshes: &mut Assets<Mesh>, name: &str, parts: usize, skinned: bool) -> (CookedModelAsset, Vec<Handle<Mesh>>) {
    let handles: Vec<Handle<Mesh>> = (0..parts).map(|_| meshes.add(mesh())).collect();
    let skin = skinned.then(|| CookedSkin {
        inverse_bindposes: Handle::default(),
        joints: Vec::new(),
        morph_targets: Vec::new(),
        parts: Vec::new(),
        meshes: Vec::new(),
    });
    let asset = CookedModelAsset {
        meshes: handles.clone(),
        lods: vec![CookedLodMeshes { error: 0.0, parts: (0..parts).map(|i| (i, u32::try_from(i).expect("small"))).collect() }],
        materials: (0..parts).map(|i| desc(&format!("{name}/{i}"))).collect(),
        bounds: sf_model::Aabb { min: [0.0; 3], max: [1.0; 3] },
        skipped: None,
        ray_tracing: None,
        skin,
    };
    (asset, handles)
}

#[derive(Resource, Default)]
struct Statuses(Vec<RevealStatus>);

fn client(mut reveal: RevealCheck, mut seen: ResMut<Statuses>) {
    let status = reveal.step(RevealAsk { kind: "loading", place: "census", focus: None, cells: true, gated: false, give_up: false });
    seen.0.push(status);
}

struct Fixture {
    app: App,
    /// Per representative model: its meshes.
    meshes: HashMap<&'static str, Vec<Handle<Mesh>>>,
}

/// An app with streaming's material cache, the precache's main-world side, the warm-up (started)
/// and a reveal client stepping every frame.
fn fixture(builtins: &[&str]) -> Fixture {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .init_asset::<Image>()
        .init_asset::<CookedModelAsset>()
        .init_resource::<Precache>()
        .init_resource::<RevealGate>()
        .init_resource::<BuiltinSpecimens>()
        .init_resource::<Statuses>()
        .init_resource::<ModelMaterials>()
        .insert_resource(Streaming::new(StreamingConfig::default()))
        .add_systems(Update, (crate::catalog_warm::drive_catalog_warmup, client).chain());
    let mut models = HashMap::new();
    let mut meshes = HashMap::new();
    {
        let world = app.world_mut();
        let made: Vec<(&'static str, CookedModelAsset, Vec<Handle<Mesh>>)> = world.resource_scope(|_, mut m: Mut<Assets<Mesh>>| {
            [("rock.nif", 2, false), ("tree.nif", 1, false), ("body.nif", 1, true)]
                .into_iter()
                .map(|(name, parts, skinned)| {
                    let (asset, handles) = model(&mut m, name, parts, skinned);
                    (name, asset, handles)
                })
                .collect()
        });
        let mut cooked = world.resource_mut::<Assets<CookedModelAsset>>();
        for (name, asset, handles) in made {
            models.insert(name.to_owned(), cooked.add(asset));
            meshes.insert(name, handles);
        }
    }
    let mut warmup = CatalogWarmup::default();
    // The character path's mapping (unlit here, so the test can tell it apart).
    warmup.skinned_materials = Some(ModelMaterials(Arc::new(|m, s| StandardMaterial { unlit: true, ..standard_material(m, s) })));
    warmup.loader = Some(Arc::new(move |rep, _| models.get(&rep.model).cloned()));
    let mut catalog = catalog();
    catalog.builtins = builtins.iter().map(|&b| b.to_owned()).collect();
    assert!(warmup.set_catalog(catalog));
    warmup.start();
    app.insert_resource(warmup);
    Fixture { app, meshes }
}

/// The specimens sent so far (as `PrecachePlugin`'s extract takes them), by tag name.
fn sent(app: &App) -> Vec<Specimen> {
    let (inbox, releases) = app.world().resource::<Precache>().take();
    assert!(releases.is_empty());
    assert!(inbox.iter().all(|(tag, _)| *tag == 0), "one tag: {CATALOG_TAG}");
    inbox.into_iter().map(|(_, s)| s).collect()
}

/// The warm-up requests exactly the loadable representatives, with the entry flags; statics bind
/// streaming's own template material (a later streamed placement is a cache hit, same handle),
/// skinned ones the character path's; the one without an asset path is counted, not waited for.
#[test]
fn requests_each_representative_with_its_flags_under_the_catalog_tag() {
    let Fixture { mut app, meshes } = fixture(&[]);
    app.update();
    let status = app.world().resource::<Precache>().status(CATALOG_TAG);
    assert_eq!(status, PrecacheStatus { queued: 3, ..PrecacheStatus::default() }, "requested, not yet in the render world; failures {:?}", app.world().resource::<CatalogWarmup>().failures());
    let got = sent(&app);
    assert_eq!(app.world().resource::<Precache>().status(CATALOG_TAG).queued, 3, "in flight until the render world reports");

    let world = app.world_mut();
    let mapper = world.resource::<ModelMaterials>().clone();
    let server = world.resource::<AssetServer>().clone();
    let (rock, tree) = world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
        let mut streaming = world.resource_mut::<Streaming>();
        let rock = streaming.template_material(&desc("rock.nif/1"), &mut materials, &mapper, &server);
        let tree = streaming.template_material(&desc("tree.nif/0"), &mut materials, &mapper, &server);
        (rock, tree)
    });
    let warmup = world.resource::<CatalogWarmup>();
    let body = warmup.specimens().iter().find(|s| s.mesh == meshes["body.nif"][0]).expect("the skinned specimen");
    let body_material = body.material.clone().typed::<StandardMaterial>();
    assert!(world.resource::<Assets<StandardMaterial>>().get(&body_material).expect("made").unlit, "skinned: the character mapping");

    let expected = vec![
        Specimen { mesh: meshes["rock.nif"][1].id(), material: rock.id().untyped(), flags: SpecimenFlags::SHADOW_CASTER },
        Specimen { mesh: meshes["tree.nif"][0].id(), material: tree.id().untyped(), flags: SpecimenFlags::VISIBILITY_RANGE_DITHER },
        Specimen {
            mesh: meshes["body.nif"][0].id(),
            material: body_material.id().untyped(),
            flags: SpecimenFlags::SHADOW_CASTER | SpecimenFlags::HAS_PREVIOUS_SKIN | SpecimenFlags::HAS_PREVIOUS_MORPH,
        },
    ];
    let key = |s: &Specimen| format!("{:?}", s.mesh);
    let (mut got, mut expected) = (got, expected);
    got.sort_by_key(key);
    expected.sort_by_key(key);
    assert_eq!(format!("{got:?}"), format!("{expected:?}"));
    assert_eq!(warmup.specimens().len(), 3, "handles kept alive");
    assert_eq!(warmup.failures(), ["missing.nif: no asset path"]);

    // Nothing more is requested on later frames.
    app.update();
    assert!(sent(&app).is_empty());
}

/// W4b3: the catalog never holds the reveal (UE5: the bundled precache compiles in the
/// background): with the whole catalog pending the gate opens after `SETTLE_FRAMES`; the tag is a
/// background one; the warm-up resolves after the reveal, when the render world reports it.
#[test]
fn the_gate_does_not_wait_for_the_catalog() {
    let Fixture { mut app, .. } = fixture(&[]);
    {
        let world = app.world();
        assert_eq!(world.resource::<CatalogWarmup>().pending(world.get_resource::<Precache>()), 4, "not started: every entry");
    }
    for _ in 0..SETTLE_FRAMES {
        app.update();
    }
    let _ = sent(&app);
    let world = app.world();
    assert_eq!(world.resource::<CatalogWarmup>().pending(world.get_resource::<Precache>()), 3, "still pending");
    let opened: Vec<bool> = world.resource::<Statuses>().0.iter().map(|s| s.open).collect();
    assert_eq!(opened, [false, false, true], "opens after SETTLE_FRAMES clear frames, catalog or not");
    assert!(world.resource::<Precache>().is_background(CATALOG_TAG));
    assert!(!world.resource::<CatalogWarmup>().is_resolved());

    app.world().resource::<Precache>().report(vec![PrecacheStatus { ready: 2, failed: 1, ..PrecacheStatus::default() }]);
    app.update();
    let world = app.world();
    assert_eq!(world.resource::<CatalogWarmup>().pending(world.get_resource::<Precache>()), 0);
    assert!(world.resource::<CatalogWarmup>().is_resolved());
}

#[derive(Resource)]
struct Focus(Entity);

fn focus_client(mut reveal: RevealCheck, focus: Res<Focus>, mut seen: ResMut<Statuses>) {
    let status = reveal.step(RevealAsk { kind: "door", place: "census", focus: Some(focus.0), cells: true, gated: false, give_up: false });
    seen.0.push(status);
}

/// W4b3: the reveal's shown set. While a hold runs, each resident mesh within the radius of the
/// focus (hidden or far ones not) is requested once per distinct (mesh, material, flags) under
/// [`RESIDENT_TAG`], with its own entity flags; the gate holds while the tag is pending and opens
/// once the render world reports it ready; the next hold releases the last one's tag and
/// requests its shown set again.
#[test]
fn the_shown_set_is_requested_and_holds_the_reveal() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .init_asset::<Image>()
        .init_resource::<Precache>()
        .init_resource::<RevealGate>()
        .init_resource::<ResidentWarmup>()
        .init_resource::<Statuses>()
        .add_systems(Update, focus_client)
        .add_systems(
            PostUpdate,
            (crate::resident_warm::begin_resident_frame, crate::resident_warm::scan_resident::<StandardMaterial>).chain(),
        );
    let world = app.world_mut();
    let focus = world.spawn(GlobalTransform::from_xyz(100.0, 0.0, 0.0)).id();
    world.insert_resource(Focus(focus));
    let (rock, tree) = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        (meshes.add(mesh()), meshes.add(mesh()))
    };
    let material = world.resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    let range = VisibilityRange { start_margin: 0.0..0.0, end_margin: 0.0..1.0, use_aabb: false };
    let at = |x: f32| (GlobalTransform::from_xyz(100.0 + x, 0.0, 0.0), InheritedVisibility::VISIBLE);
    world.spawn((Mesh3d(rock.clone()), MeshMaterial3d(material.clone()), at(5.0)));
    world.spawn((Mesh3d(rock.clone()), MeshMaterial3d(material.clone()), at(-7.0))); // same specimen
    world.spawn((Mesh3d(tree.clone()), MeshMaterial3d(material.clone()), at(10.0), NotShadowCaster, range.clone()));
    world.spawn((Mesh3d(tree.clone()), MeshMaterial3d(material.clone()), at(60.0))); // beyond 48 m
    world.spawn((Mesh3d(tree.clone()), MeshMaterial3d(material.clone()), GlobalTransform::from_xyz(101.0, 0.0, 0.0), InheritedVisibility::HIDDEN));

    for _ in 0..10 {
        app.update();
    }
    let precache = app.world().resource::<Precache>().clone();
    assert_eq!(precache.status(RESIDENT_TAG), PrecacheStatus { queued: 2, ..PrecacheStatus::default() });
    assert!(app.world().resource::<Statuses>().0.iter().all(|s| s.cpu_ready && !s.open), "held on the shown set");
    assert_eq!(app.world().resource::<ResidentWarmup>().requested(), 2);
    let (inbox, releases) = precache.take();
    assert!(releases.is_empty(), "nothing to release on the first hold");
    let mut got: Vec<Specimen> = inbox.into_iter().map(|(_, s)| s).collect();
    got.sort_by_key(|s| s.flags.bits());
    let m = material.id().untyped();
    assert_eq!(
        format!("{got:?}"),
        format!(
            "{:?}",
            [
                Specimen { mesh: rock.id(), material: m, flags: SpecimenFlags::SHADOW_CASTER },
                Specimen { mesh: tree.id(), material: m, flags: SpecimenFlags::VISIBILITY_RANGE_DITHER },
            ]
        )
    );

    // Compiling still holds; ready clears.
    precache.report(vec![PrecacheStatus { compiling: 1, ready: 1, ..PrecacheStatus::default() }]);
    app.update();
    assert!(app.world().resource::<Statuses>().0.iter().all(|s| !s.open));
    precache.report(vec![PrecacheStatus { ready: 2, ..PrecacheStatus::default() }]);
    app.world_mut().resource_mut::<Statuses>().0.clear();
    for _ in 0..SETTLE_FRAMES {
        app.update();
    }
    assert_eq!(app.world().resource::<Statuses>().0.iter().filter(|s| s.open).count(), 1, "opens after SETTLE_FRAMES");

    // The client steps again: a new hold releases the last tag and requests its set again.
    app.update();
    app.update();
    let (inbox, releases) = precache.take();
    assert_eq!((releases.len(), inbox.len()), (1, 2), "released, then requested again");
}

/// Builtins by name: a registered one gives its specimens, an unknown one is skipped (logged, not
/// fatal), and one not available yet is retried while a reveal holds.
#[test]
fn builtins_by_name_unknown_skipped_not_yet_retried() {
    let mut registry = BuiltinSpecimens::default();
    assert!(matches!(registry.build("water", &mut World::new()), BuiltinOutcome::Skip(r) if r.contains("no builder registered")));

    let Fixture { mut app, .. } = fixture(&["nope", "terrain", "water"]);
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh());
    let material = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
    let water = WarmSpecimen::new(mesh.clone(), material, SpecimenFlags::empty());
    let expected = water.specimen();
    registry.register("water", move |_| BuiltinOutcome::Ready(vec![water.clone()]));
    let tries = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let counted = tries.clone();
    registry.register("terrain", move |world: &mut World| {
        counted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if world.contains_resource::<TerrainLive>() {
            BuiltinOutcome::Ready(Vec::new())
        } else {
            BuiltinOutcome::NotYet("no terrain yet".to_owned())
        }
    });
    assert!(registry.contains("water") && !registry.contains("nope"));
    app.insert_resource(registry);

    app.update();
    let got = sent(&app);
    assert_eq!(got.len(), 4, "3 representatives + water");
    assert!(format!("{got:?}").contains(&format!("{expected:?}")));
    app.update();
    assert!(app.world().resource::<RevealGate>().is_holding());
    app.world_mut().insert_resource(TerrainLive);
    app.update();
    assert_eq!(tries.load(std::sync::atomic::Ordering::Relaxed), 3, "tried at the start, retried each held frame until live");
    app.update();
    assert_eq!(tries.load(std::sync::atomic::Ordering::Relaxed), 3, "built: not retried");
    let world = app.world();
    assert_eq!(world.resource::<CatalogWarmup>().specimens().len(), 4);
    assert!(world.resource::<CatalogWarmup>().failures().iter().all(|f| !f.contains("nope")), "a skipped builtin is not a failure");
}

#[derive(Resource)]
struct TerrainLive;
