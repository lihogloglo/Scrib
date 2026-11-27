//! `IN38b2` headless tests of the host stage hook: a removing break takes a live container out
//! once (hooks, overlay, visuals), leaves a standing stage alone, and an unloaded placement's
//! break removes it from the overlay alone.

#![allow(clippy::default_trait_access, reason = "the crate has no math dependency to name IVec3 / Vec3")]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use mw_scripts_bevy::{ActivePlacement, ActivePlacements, LiveRoot};
use sf_coords::WorldTransform;
use sf_interaction_bevy::Destroyed;
use sf_interaction_bevy::sf_interaction::{self, Tick};
use sf_world::{IndexedPlacement, Placement, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, PlacementSpawner, PlacementSpawners, SpawnCx, WorldOverlay};

use crate::breakable::{Removal, remove_broken};

const CRATE: PlacementId = PlacementId(80);
const FAR_CRATE: PlacementId = PlacementId(81);

/// The container class's spawner: counts deactivations.
struct Counting(Arc<AtomicU32>);

impl PlacementSpawner for Counting {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
    fn deactivate(&self, _: &mut World, _: &ActivateCx<'_>, _: &IndexedPlacement) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn placed(id: PlacementId) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id,
            base: RecordRef::new("CONT", "contain_crate_01"),
            model: None,
            transform: WorldTransform::IDENTITY,
            class: mw_world::source::CONTAINER,
            flags: PlacementFlags::empty(),
        },
        radius: 0.5,
    }
}

/// The hook over a live crate in R0 (its `ActivePlacements` entry and a root with one visual
/// under it); returns (app, deactivation count, root, visual).
fn app() -> (App, Arc<AtomicU32>, Entity, Entity) {
    let deactivated = Arc::new(AtomicU32::new(0));
    let mut app = App::new();
    let mut spawners = PlacementSpawners::default();
    spawners.set(mw_world::source::CONTAINER, Arc::new(Counting(deactivated.clone())));
    app.insert_resource(spawners).init_resource::<ActivePlacements>().init_resource::<WorldOverlay>();
    crate::breakable::add(&mut app);
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let root = world.spawn((LiveRoot(CRATE), ChildOf(cell_entity))).id();
    let visual = world.spawn(ChildOf(root)).id();
    let entry = ActivePlacement { placement: placed(CRATE), worldspace: WorldspaceId::new("test"), cell: Default::default(), cell_entity };
    world.resource_mut::<ActivePlacements>().0.insert(CRATE, entry);
    (app, deactivated, root, visual)
}

/// One released break of `placement`'s first stage.
fn broke(placement: PlacementId, removes: bool) -> Destroyed {
    Destroyed(sf_interaction::Destroyed {
        tick: Tick(3),
        placement,
        stage: 0,
        point: Default::default(),
        dir: Default::default(),
        seed: 7,
        culprit: None,
        overkill: 0.0,
        removes,
    })
}

fn removed(app: &App, id: PlacementId) -> bool {
    app.world().resource::<WorldOverlay>().0.read().unwrap().is_removed(id)
}

#[test]
fn a_removing_break_takes_a_live_container_out_once() {
    let (mut app, deactivated, root, visual) = app();
    // A standing stage changes nothing.
    app.world_mut().write_message(broke(CRATE, false));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(deactivated.load(Ordering::Relaxed), 0);
    assert!(!removed(&app, CRATE) && app.world().get_entity(root).is_ok());

    // The removing stage: hooks once, overlay, root and visual; the same break again finds it gone.
    app.world_mut().write_message(broke(CRATE, true));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);
    assert!(removed(&app, CRATE));
    assert!(app.world().get_entity(root).is_err() && app.world().get_entity(visual).is_err());
    assert!(app.world().resource::<ActivePlacements>().0.is_empty());
    assert_eq!(remove_broken(app.world_mut(), CRATE), Removal::Gone);
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);
}

#[test]
fn an_unloaded_placement_break_removes_it_from_the_overlay_only() {
    let (mut app, deactivated, root, _) = app();
    assert_eq!(remove_broken(app.world_mut(), FAR_CRATE), Removal::Unloaded);
    assert!(removed(&app, FAR_CRATE) && !removed(&app, CRATE));
    // No hook ran, the live crate kept its visuals.
    assert_eq!(deactivated.load(Ordering::Relaxed), 0);
    assert!(app.world().get_entity(root).is_ok());
    assert_eq!(remove_broken(app.world_mut(), FAR_CRATE), Removal::Gone);
    assert_eq!(remove_broken(app.world_mut(), CRATE), Removal::Live);
}
