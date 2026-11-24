//! Parked worldspaces (CW-9, `park.rs`): through the real streaming stages and the fade, the street
//! the player left goes out of every plain query while it stays resident, comes back with the same
//! entities and state, and the switch off is today's behaviour.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::entity_disabling::Disabled;
use bevy::mesh::MeshPlugin;
use bevy::prelude::*;
use big_space::prelude::{CellCoord, FloatingOrigin};
use glam::DVec3;
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{CookSettings, cook};
use sf_model_bevy::{BAKED_SOURCE, BakedManifest, baked_source};
use sf_vfs::VfsPath;
use sf_world::{
    DoorLink, IndexParams, LinkEnd, Location, MemoryIndex, ModelInfo, Partition, Placement, PlacementClass, PlacementFlags, PlacementId,
    RecordRef, WorldspaceId,
};

use super::*;

fn street() -> WorldspaceId {
    WorldspaceId::new("street")
}

fn room() -> WorldspaceId {
    WorldspaceId::new("room")
}

/// An outdoor character's state (stands in for AI, stats, a mover).
#[derive(Component, Clone, Debug, PartialEq)]
struct Npc {
    goal: u32,
}

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

/// A street of plates over a few cells around a door at the origin, into a one-cell room.
fn door_app(dir: &std::path::Path) -> App {
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
    let mut plates = Vec::new();
    for x in -3..=3 {
        for z in -3..=3 {
            let pos = DVec3::new(f64::from(x) * 40.0, 0.0, f64::from(z) * 40.0 + 6.0);
            plates.push(placement("street", &format!("{x},{z}"), pos));
        }
    }
    add(street(), &IndexParams::default(), plates);
    let single = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    add(room(), &single, vec![placement("room", "table", DVec3::new(0.0, 0.0, 3.0))]);
    let links = vec![DoorLink {
        a: LinkEnd { worldspace: street(), frame: point(0.0), arrival: point(0.0), door: None, extent: Vec2::ZERO },
        b: LinkEnd { worldspace: room(), frame: point(0.0), arrival: point(0.0), door: None, extent: Vec2::ZERO },
        two_way: true,
    }];
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
        .insert_resource(ActiveWorldspace(Some(street())));
    app.finish();
    app.cleanup();
    {
        let mut fade = app.world_mut().resource_mut::<FadeTransition>();
        fade.fade_secs = 0.0;
        fade.max_hold_secs = 0.05;
    }
    app.world_mut().resource_mut::<WorldspaceParking>().settle_s = 0.0;
    app
}

/// Runs frames until streaming is idle, then a few more (parking takes two frames).
fn settle(app: &mut App) {
    for frame in 0..5000 {
        app.update();
        if frame > 3 && app.world().resource::<Streaming>().is_idle() && !app.world().resource::<FadeTransition>().is_active() {
            for _ in 0..4 {
                app.update();
            }
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("streaming never went idle");
}

fn travel(app: &mut App, traveler: Entity, to: &WorldspaceId) {
    let to = Location::new(to.clone(), WorldPos::from_f64(DVec3::new(0.0, 1.0, 0.0)));
    app.world_mut().write_message(StartTravel { traveler, to: to.clone(), rotation: Quat::IDENTITY, reload: false });
    for _ in 0..2000 {
        app.update();
        if app.world_mut().resource_mut::<Messages<TravelArrived>>().drain().any(|a| a.to == to) {
            settle(app);
            return;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("never arrived");
}

/// Every entity under the street's root (root included), seen with `Allow<Disabled>`.
fn street_entities(app: &mut App) -> BTreeSet<Entity> {
    let root = app.world().resource::<WorldspaceRegistry>().get(&street()).expect("street loaded").root;
    let world = app.world_mut();
    let mut out = BTreeSet::new();
    let mut stack = vec![root];
    while let Some(e) = stack.pop() {
        out.insert(e);
        if let Some(children) = world.get::<Children>(e) {
            stack.extend(children.iter());
        }
    }
    out
}

/// Street cells and street-layer meshes a plain query sees.
fn plain_street(app: &mut App) -> (usize, usize) {
    let content = app.world().resource::<WorldspaceRegistry>().get(&street()).unwrap().layers().content();
    let world = app.world_mut();
    let cells = world.query::<&StreamCell>().iter(world).filter(|c| c.worldspace == street()).count();
    let meshes = world.query_filtered::<&RenderLayers, With<Mesh3d>>().iter(world).filter(|l| l.intersects(&content)).count();
    (cells, meshes)
}

/// The street with its player and an outdoor character, settled.
fn outdoors(app: &mut App) -> (Entity, Entity) {
    let root = WorldspaceRegistry::ensure_in(app.world_mut(), &street()).root;
    let (cell, transform) = to_grid(WorldPos::from_f64(DVec3::new(0.0, 1.0, 0.0)));
    let player = app.world_mut().spawn((cell, transform, StreamingAnchor::gameplay(), FloatingOrigin, ChildOf(root))).id();
    let npc = app.world_mut().spawn((Npc { goal: 7 }, CellCoord::default(), Transform::from_xyz(5.0, 0.0, 2.0), Visibility::Inherited, ChildOf(root))).id();
    app.world_mut().resource_mut::<Streaming>().begin_burst();
    settle(app);
    (player, npc)
}

/// Switch on: inside the room the street is resident but out of every plain query; walking back out
/// spawns nothing and finds the same entities with the same state, visible again.
#[test]
fn the_street_parks_while_inside_and_comes_back_whole() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = door_app(dir.path());
    app.world_mut().resource_mut::<WorldspaceParking>().enabled = true;
    let (player, npc) = outdoors(&mut app);
    let (cells, meshes) = plain_street(&mut app);
    assert!(cells > 1 && meshes > 1, "the street streamed in: {cells} cells, {meshes} meshes");
    assert!(!app.world().resource::<WorldspaceParking>().is_parked(&street()), "never parks the active worldspace");
    let before = street_entities(&mut app);

    travel(&mut app, player, &room());
    let parking = app.world().resource::<WorldspaceParking>();
    assert!(parking.is_disabled(&street()), "the street parked once the player left it");
    assert!(!parking.is_parked(&room()), "the active room is live");
    let parked = parking.parked_entities();
    assert_eq!(plain_street(&mut app), (0, 0), "no plain query sees the street");
    assert!(app.world_mut().query::<&Npc>().iter(app.world()).next().is_none(), "the outdoor character is out of queries");
    let world = app.world_mut();
    let kept = world.query_filtered::<&StreamCell, Allow<Disabled>>().iter(world).filter(|c| c.worldspace == street()).count();
    assert_eq!(kept, cells, "every street cell is still resident");
    let mut street_now = street_entities(&mut app);
    street_now.remove(&player);
    let mut expected = before.clone();
    expected.remove(&player);
    assert_eq!(street_now, expected, "nothing of the street despawned while parked");
    assert_eq!(parked, expected.len(), "the whole subtree (root included) is disabled");
    assert!(expected.iter().all(|e| app.world().get::<Disabled>(*e).is_some()));
    assert_eq!(app.world().get::<Npc>(npc), Some(&Npc { goal: 7 }), "state kept");
    let spawned = app.world().resource::<Streaming>().stats().entities_spawned;

    travel(&mut app, player, &street());
    let parking = app.world().resource::<WorldspaceParking>();
    assert!(!parking.is_parked(&street()), "back out: the street is live");
    assert_eq!(app.world().resource::<Streaming>().stats().entities_spawned, spawned, "the door back out spawned nothing");
    assert_eq!(plain_street(&mut app), (cells, meshes), "plain queries see the street again");
    let after = street_entities(&mut app);
    assert_eq!(after, before, "the same entity ids");
    assert!(after.iter().all(|e| app.world().get::<Disabled>(*e).is_none()));
    assert_eq!(app.world_mut().query::<&Npc>().single(app.world()).ok(), Some(&Npc { goal: 7 }));
    assert_eq!(app.world().get::<Transform>(npc).map(|t| t.translation), Some(Vec3::new(5.0, 0.0, 2.0)));
    let root = app.world().resource::<WorldspaceRegistry>().get(&street()).unwrap().root;
    assert_eq!(app.world().get::<Visibility>(root), Some(&Visibility::Visible), "the root's visibility restored");
}

/// Switch off (on by default since 2026-11-17): nothing parks, plain queries keep seeing the street.
#[test]
fn switch_off_parks_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = door_app(dir.path());
    app.world_mut().resource_mut::<WorldspaceParking>().enabled = false;
    let (player, _) = outdoors(&mut app);
    let outside = plain_street(&mut app);
    travel(&mut app, player, &room());
    assert_eq!(app.world().resource::<WorldspaceParking>().parked_count(), 0);
    assert_eq!(plain_street(&mut app), outside, "the left street stays in plain queries, as before");
    let world = app.world_mut();
    assert_eq!(world.query_filtered::<(), With<Disabled>>().iter(world).count(), 0, "nothing disabled");
}

/// Flipping the switch live indoors applies both ways, and a spawn under the parked street unparks it.
#[test]
fn flipping_live_indoors_and_a_late_spawn_unpark() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = door_app(dir.path());
    app.world_mut().resource_mut::<WorldspaceParking>().enabled = false;
    let (player, npc) = outdoors(&mut app);
    let outside = plain_street(&mut app);
    let before = street_entities(&mut app);
    travel(&mut app, player, &room());
    assert_eq!(plain_street(&mut app), outside);

    app.world_mut().resource_mut::<WorldspaceParking>().enabled = true;
    for _ in 0..3 {
        app.update();
    }
    assert!(app.world().resource::<WorldspaceParking>().is_disabled(&street()), "on: parks indoors");
    assert_eq!(plain_street(&mut app), (0, 0));

    app.world_mut().resource_mut::<WorldspaceParking>().enabled = false;
    app.update();
    assert_eq!(app.world().resource::<WorldspaceParking>().parked_count(), 0, "off: unparked");
    assert_eq!(plain_street(&mut app), outside, "back in plain queries");
    let mut now = street_entities(&mut app);
    now.remove(&player);
    let mut expected = before;
    expected.remove(&player);
    assert_eq!(now, expected, "same entities");
    assert_eq!(app.world().get::<Npc>(npc), Some(&Npc { goal: 7 }));

    // On again, then something spawns under a parked street entity (a late model, a reparent).
    app.world_mut().resource_mut::<WorldspaceParking>().enabled = true;
    for _ in 0..3 {
        app.update();
    }
    assert!(app.world().resource::<WorldspaceParking>().is_disabled(&street()));
    let late = app.world_mut().spawn((Npc { goal: 1 }, Transform::default(), ChildOf(npc))).id();
    app.world_mut().flush();
    assert!(!app.world().resource::<WorldspaceParking>().is_parked(&street()), "a spawn under the parked street unparks it");
    assert!(app.world().get::<Disabled>(npc).is_none() && app.world().get::<Disabled>(late).is_none(), "nothing runs under a disabled parent");
    assert_eq!(app.world_mut().query::<&Npc>().iter(app.world()).count(), 2);
    // Quiet again: it parks again (settle 0), the late child with it.
    for _ in 0..3 {
        app.update();
    }
    assert!(app.world().resource::<WorldspaceParking>().is_disabled(&street()), "parks again once quiet");
    assert!(app.world().get::<Disabled>(late).is_some());
}
