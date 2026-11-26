//! Headless tests of the placement hook: which classes become script handles at R0.

use std::sync::Arc;

use bevy_app::{App, PostStartup};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_script::Handle;
use sf_script_bevy::ScriptPlacements;
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx};

use crate::{HANDLE_CLASSES, hook_placements};

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
