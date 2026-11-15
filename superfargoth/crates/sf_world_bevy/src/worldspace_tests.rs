//! Worldspace render-layer pairs belong to a worldspace's loaded lifetime (ROADMAP 1.1): a
//! worldspace that streamed out gives its root and pair back, the pool reuses them, and a layer
//! past Bevy's 64 inline ones never panics.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::camera::visibility::RenderLayers;
use bevy::mesh::MeshPlugin;
use bevy::prelude::*;
use big_space::prelude::FloatingOrigin;
use glam::DVec3;
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{CookSettings, cook};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::{
    DoorLink, IndexParams, LinkEnd, MemoryIndex, ModelInfo, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef,
    WorldspaceId, WorldspaceState,
};

use super::*;

fn room(i: u32) -> WorldspaceId {
    WorldspaceId::new(&format!("room{i}"))
}

/// A registry alone: 100 worldspaces loaded and released one after another share one pair; 40
/// held at once run past layer 63 without a panic; a root that still holds content keeps its pair.
#[test]
fn released_worldspaces_recycle_their_pair_and_layers_past_63_never_panic() {
    let mut world = World::new();
    world.init_resource::<WorldspaceRegistry>();
    for i in 0..100 {
        let e = WorldspaceRegistry::ensure_in(&mut world, &room(i));
        assert_eq!(e.layers(), WorldspaceLayers { world: 1, shadow: 2 }, "worldspace {i} reuses the freed pair");
        assert!(WorldspaceRegistry::release_in(&mut world, &room(i)));
        assert!(world.get_entity(e.root).is_err() && world.get_entity(e.proxy).is_err(), "root and proxy despawned");
    }
    let registry = world.resource::<WorldspaceRegistry>();
    assert_eq!((registry.len(), registry.pairs_allocated()), (0, 1));

    let held: Vec<WorldspaceEntry> = (0..40).map(|i| WorldspaceRegistry::ensure_in(&mut world, &room(i))).collect();
    let last = held[39].layers();
    assert_eq!(last, WorldspaceLayers { world: 79, shadow: 80 });
    assert_eq!(dynamic_layer(79).iter().collect::<Vec<_>>(), vec![79]);
    let (content, shadow) = (last.content(), last.shadow_only());
    assert!(content.intersects(&last.camera()) && !content.intersects(&shadow) && last.light().intersects(&shadow));
    assert!(!last.light().intersects(&held[0].layers().light()), "pairs past 63 stay apart from the inline ones");

    let cell = world.spawn(ChildOf(held[5].root)).id();
    assert!(!WorldspaceRegistry::release_in(&mut world, &room(5)), "content under the root keeps the pair");
    assert!(world.resource::<WorldspaceRegistry>().get(&room(5)).is_some());
    world.entity_mut(cell).despawn();
    assert!(WorldspaceRegistry::release_in(&mut world, &room(5)));
    assert!(WorldspaceRegistry::release_in(&mut world, &room(6)));
    // First in, first out: room5's pair (11, 12) comes back before room6's (13, 14).
    assert_eq!(WorldspaceRegistry::ensure_in(&mut world, &room(100)).layer, 11);
    assert_eq!(WorldspaceRegistry::ensure_in(&mut world, &room(101)).layer, 13);
    assert_eq!(world.resource::<WorldspaceRegistry>().pairs_allocated(), 40);
}

const DOORS: u32 = 40;
const DOOR_SPACING: f64 = 100.0;

fn placement(ws: &str, key: &str, pos: DVec3) -> Placement {
    Placement {
        id: PlacementId::from_key(ws, key.as_bytes()),
        base: RecordRef::new("thing", key),
        model: Some(VfsPath::new("plate.mdl").unwrap()),
        transform: WorldTransform::from_pos(WorldPos::from_f64(pos)),
        class: PlacementClass::Static,
        flags: PlacementFlags::empty(),
    }
}

fn point(x: f64) -> WorldTransform {
    WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(x, 0.0, 0.0)))
}

/// An exterior strip with [`DOORS`] doors 100 m apart, each into its own one-cell interior.
fn door_street(dir: &std::path::Path) -> App {
    let store = Store::open(dir).unwrap();
    let cooked = cook(&tests::plate(), &CookSettings::default()).unwrap().to_bytes();
    let mut manifest = Manifest::default();
    manifest.products.insert(ProductKey::new("mesh.cook", "plate.mdl"), store.put(&cooked, true).unwrap());
    let models =
        |_: &VfsPath| Some(ModelInfo { min: Vec3::new(-4.0, -0.3, -4.0), max: Vec3::new(4.0, 0.3, 4.0), bytes: 50_000, lod_distance: 0.0 });

    let mut index = MemoryIndex::new();
    let mut add = |ws: WorldspaceId, params: &IndexParams, placements: Vec<Placement>| {
        let (pages, _) = sf_world::build_pages(params, placements, &models);
        let directory = sf_world::build_directory(ws, params, &pages);
        index.insert(&sf_world::WorldIndex { directory, pages, stats: sf_world::IndexStats::default() });
    };
    let street = WorldspaceId::new("street");
    let doors = (0..DOORS).map(|i| placement("street", &format!("{i}"), DVec3::new(f64::from(i) * DOOR_SPACING, 0.0, 6.0))).collect();
    add(street.clone(), &IndexParams::default(), doors);
    let single = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let mut links = Vec::new();
    for i in 0..DOORS {
        add(room(i), &single, vec![placement(room(i).as_str(), "table", DVec3::new(0.0, 0.0, 3.0))]);
        let door = point(f64::from(i) * DOOR_SPACING);
        links.push(DoorLink {
            a: LinkEnd { worldspace: street.clone(), frame: door, arrival: door, door: None, extent: Vec2::ZERO },
            b: LinkEnd { worldspace: room(i), frame: point(0.0), arrival: point(0.0), door: None, extent: Vec2::ZERO },
            two_way: true,
        });
    }

    let mut app = App::new();
    app.register_asset_source(
        BAKED_SOURCE,
        baked_source(store, BakedManifest::new(manifest), BTreeMap::from([("mesh.cook".to_owned(), "sfmesh".to_owned())])),
    );
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), MeshPlugin))
        .init_asset::<StandardMaterial>()
        .add_plugins(WorldStreamingPlugin { config: StreamingConfig::default() })
        .insert_resource(WorldIndexSource(Arc::new(index)))
        .insert_resource(DoorLinks(links))
        .insert_resource(ActiveWorldspace(Some(street)));
    app.finish();
    app.cleanup();
    app
}

/// Runs frames until streaming is idle, then two more (the release runs at the end of a drive).
fn settle(app: &mut App) {
    for frame in 0..5000 {
        app.update();
        if frame > 3 && app.world().resource::<Streaming>().is_idle() {
            app.update();
            app.update();
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("streaming never went idle");
}

/// The Balmora walk of ROADMAP 1.1, through the real streaming stages: a player passes 40 doors,
/// each interior is prefetched behind its door and streams out behind them. The interior left
/// behind gives its root and pair back, so the street and one interior hold pairs once settled and
/// at most three pairs ever exist (the leaving and the arriving interior overlap for a few frames); coming
/// back to the first door prefetches it again under a new root.
#[test]
fn walking_past_forty_doors_keeps_worldspace_layer_pairs_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = door_street(dir.path());
    let street = WorldspaceId::new("street");
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &street).root;
    let at = |x: f64| to_grid(WorldPos::from_f64(DVec3::new(x, 1.0, 0.0)));
    let (cell, transform) = at(0.0);
    let player = app.world_mut().spawn((cell, transform, StreamingAnchor::gameplay(), FloatingOrigin, ChildOf(root))).id();
    app.world_mut().resource_mut::<Streaming>().begin_burst();

    let mut first_root = None;
    for i in 0..DOORS {
        app.world_mut().entity_mut(player).insert(at(f64::from(i) * DOOR_SPACING));
        settle(&mut app);
        if i > 0 {
            // The release follows the unload's async completion, so under machine load it can land a few
            // frames after `is_idle`: wait for it (bounded), it must still happen.
            for _ in 0..2000 {
                if app.world().resource::<WorldspaceRegistry>().get(&room(i - 1)).is_none() {
                    break;
                }
                app.update();
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        let streaming = app.world().resource::<Streaming>();
        assert_eq!(streaming.scheduler().worldspace_state(&room(i)), WorldspaceState::Prefetched, "door {i}");
        let registry = app.world().resource::<WorldspaceRegistry>();
        let entry = registry.get(&room(i)).expect("the prefetched interior holds a pair");
        first_root.get_or_insert(entry.root);
        if i > 0 {
            assert!(registry.get(&room(i - 1)).is_none(), "room{} streamed out and gave its pair back", i - 1);
        }
        assert!(registry.len() <= 2, "street + the interior behind this door, not {}", registry.len());
        assert!(registry.pairs_allocated() <= 3, "pairs reused, not grown: {}", registry.pairs_allocated());
    }

    app.world_mut().entity_mut(player).insert(at(0.0));
    settle(&mut app);
    let entry = app.world().resource::<WorldspaceRegistry>().get(&room(0)).expect("prefetched again");
    assert_ne!(Some(entry.root), first_root, "a new root");
    let parents: Vec<Entity> = app
        .world_mut()
        .query::<(&StreamCell, &ChildOf)>()
        .iter(app.world())
        .filter(|(c, _)| c.worldspace == room(0))
        .map(|(_, p)| p.parent())
        .collect();
    assert_eq!(parents, vec![entry.root], "its cell lives under the new root");
    let content = entry.layers().content();
    let tagged = app.world_mut().query::<&RenderLayers>().iter(app.world()).filter(|l| l.intersects(&content)).count();
    assert!(tagged > 0, "the interior's content carries the recycled pair");
    assert!(app.world().resource::<WorldspaceRegistry>().pairs_allocated() <= 3);
}
