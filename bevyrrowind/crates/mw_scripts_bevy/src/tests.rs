//! Headless tests of the placement hook: which classes become script handles at R0.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_app::{App, PostStartup};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Quat, Vec3};
use bevy_transform::components::Transform;
use sf_coords::{WorldPos, WorldTransform};
use sf_script::Handle;
use sf_script_bevy::ScriptPlacements;
use sf_world::{IndexedPlacement, Level, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, EndInputs, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, StreamingConfig, WorldOverlay};

use crate::{ActivePlacements, HANDLE_CLASSES, LiveRoot, hook_placements};

/// The class's own spawner: does nothing.
struct Nothing;

impl PlacementSpawner for Nothing {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
    fn prepare(&self, _: &AssetServer, _: &IndexedPlacement, _: Option<&LoadedModel>) -> Option<UntypedHandle> {
        None
    }
}

fn placed(id: u128, tag: &'static str, record: &str, class: PlacementClass) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: PlacementId(id),
            base: RecordRef::new(tag, record),
            model: None,
            transform: WorldTransform { pos: WorldPos::from_f64([1.0, 2.0, 3.0].into()), rotation: Quat::IDENTITY, scale: Vec3::ONE },
            class,
            flags: PlacementFlags::empty(),
        },
        radius: 0.5,
    }
}

/// The spawners after the hook ran, over a default spawner only (what a custom class such as
/// the activators' falls back to).
fn hooked() -> App {
    let mut app = App::new();
    let mut spawners = PlacementSpawners::default();
    spawners.set_default(Some(Arc::new(Nothing)));
    app.insert_resource(spawners).init_resource::<ScriptPlacements>().add_systems(PostStartup, hook_placements);
    app.update();
    app
}

/// `placement` enters R0 (`on`) or leaves it through its class's spawner chain.
fn residency(app: &mut App, placement: &IndexedPlacement, on: bool) {
    let spawner = app.world().resource::<PlacementSpawners>().for_class(placement.placement.class).cloned().expect("a spawner");
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let ws = WorldspaceId::new("test");
    let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false };
    if on {
        spawner.activate(world, &cx, placement);
    } else {
        spawner.deactivate(world, &cx, placement);
    }
}

#[test]
fn an_activator_entering_r0_becomes_a_script_handle_and_leaves_with_its_cell() {
    let mut app = hooked();
    let bed = placed(41, "ACTI", "active_de_p_bed_28", mw_world::source::ACTIVATOR);
    residency(&mut app, &bed, true);
    let loaded = &app.world().resource::<ScriptPlacements>().loaded;
    let entry = loaded.get(&Handle(41)).expect("the activator has a script handle");
    assert_eq!((entry.record.as_str(), entry.worldspace.as_str(), entry.position, entry.radius), ("active_de_p_bed_28", "test", [1.0, 2.0, 3.0], 0.5));
    residency(&mut app, &bed, false);
    assert!(app.world().resource::<ScriptPlacements>().loaded.is_empty());
}

#[test]
fn every_handle_class_is_registered_and_a_marker_is_not() {
    let mut app = hooked();
    for (i, class) in HANDLE_CLASSES.into_iter().enumerate() {
        residency(&mut app, &placed(100 + i as u128, "STAT", "thing", class), true);
    }
    assert_eq!(app.world().resource::<ScriptPlacements>().loaded.len(), HANDLE_CLASSES.len());
    // A class outside the list keeps the plain default spawner: no handle.
    residency(&mut app, &placed(7, "STAT", "other", PlacementClass::Custom(200)), true);
    assert!(!app.world().resource::<ScriptPlacements>().loaded.contains_key(&Handle(7)));
}

/// A class spawner that spawns one visual under its parent and counts deactivations.
struct Probe(Arc<AtomicU32>);

impl PlacementSpawner for Probe {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        world.spawn((cx.local, ChildOf(cx.parent)));
        1
    }
    fn deactivate(&self, _: &mut World, _: &ActivateCx<'_>, _: &IndexedPlacement) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

/// `placement` spawned through its class's chain under a new parent at a scaled, turned local
/// transform; returns (count, parent).
fn spawn_visuals(app: &mut App, placement: &IndexedPlacement) -> (u32, Entity) {
    let spawner = app.world().resource::<PlacementSpawners>().for_class(placement.placement.class).cloned().expect("a spawner");
    let lod = StreamingConfig::default().lod;
    let layers = RenderLayers::default();
    let world = app.world_mut();
    let parent = world.spawn(Transform::IDENTITY).id();
    let ws = WorldspaceId::new("test");
    let cx = SpawnCx {
        worldspace: &ws,
        cell: IVec3::ZERO,
        level: Level::Near,
        parent,
        layers: &layers,
        local: Transform { translation: Vec3::new(3.0, 1.5, -2.0), rotation: Quat::from_rotation_y(0.7), scale: Vec3::splat(2.0) },
        model: None,
        max_distance: 200.0,
        end_band: 0.0,
        end: EndInputs::fixed(200.0, 0.0),
        lod: &lod,
    };
    (spawner.spawn(world, &cx, placement), parent)
}

#[test]
fn a_container_is_live_under_its_root_and_a_live_removal_takes_it_out_once() {
    let deactivated = Arc::new(AtomicU32::new(0));
    let mut app = App::new();
    let mut spawners = PlacementSpawners::default();
    spawners.set_default(Some(Arc::new(Nothing))).set(mw_world::source::CONTAINER, Arc::new(Probe(deactivated.clone())));
    app.insert_resource(spawners)
        .init_resource::<ScriptPlacements>()
        .init_resource::<ActivePlacements>()
        .init_resource::<WorldOverlay>()
        .add_systems(PostStartup, hook_placements);
    app.update();
    let crate_ = placed(9, "CONT", "com_crate_01", mw_world::source::CONTAINER);

    // The visuals spawn under one root carrying the pose; the visual keeps the scale.
    let (count, parent) = spawn_visuals(&mut app, &crate_);
    assert_eq!(count, 2);
    let world = app.world_mut();
    let roots: Vec<(Entity, LiveRoot, Transform)> = world.query::<(Entity, &LiveRoot, &Transform)>().iter(world).map(|(e, r, t)| (e, *r, *t)).collect();
    assert_eq!(roots.len(), 1);
    let (root, live, pose) = roots[0];
    assert_eq!(live, LiveRoot(PlacementId(9)));
    assert_eq!((pose.translation, pose.rotation, pose.scale), (Vec3::new(3.0, 1.5, -2.0), Quat::from_rotation_y(0.7), Vec3::ONE));
    assert_eq!(world.get::<ChildOf>(root).map(ChildOf::parent), Some(parent));
    let visual: Vec<(Entity, Transform)> =
        world.query::<(Entity, &Transform, &ChildOf)>().iter(world).filter(|(_, _, c)| c.parent() == root).map(|(e, t, _)| (e, *t)).collect();
    assert_eq!(visual.len(), 1);
    assert_eq!(visual[0].1, Transform::from_scale(Vec3::splat(2.0)));

    // R0: the container is live; the removal runs its hooks once, removes it from the overlay
    // and despawns its visuals; a second one finds nothing.
    residency(&mut app, &crate_, true);
    assert!(app.world().resource::<ActivePlacements>().0.contains_key(&PlacementId(9)));
    let removed = ActivePlacements::remove_live(app.world_mut(), PlacementId(9));
    assert_eq!(removed.map(|a| a.placement.placement.id), Some(PlacementId(9)));
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);
    let world = app.world_mut();
    assert!(world.get_entity(root).is_err() && world.get_entity(visual[0].0).is_err());
    assert!(world.get_entity(parent).is_ok());
    assert!(world.resource::<WorldOverlay>().0.read().unwrap().is_removed(PlacementId(9)));
    assert!(world.resource::<ScriptPlacements>().loaded.is_empty());
    assert!(world.resource::<ActivePlacements>().0.is_empty());
    assert!(ActivePlacements::remove_live(world, PlacementId(9)).is_none());
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);

    // A static spawns as before: no root.
    let (count, _) = spawn_visuals(&mut app, &placed(10, "STAT", "ex_wall", PlacementClass::Static));
    assert_eq!(count, 0);
    let world = app.world_mut();
    assert_eq!(world.query::<&LiveRoot>().iter(world).count(), 0);
}
