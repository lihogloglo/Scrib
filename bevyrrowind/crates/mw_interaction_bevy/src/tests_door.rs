//! Headless tests of [`DoorHook`]: the pivot at spawn, the index entry, the kinematic listing and
//! the restored rest state in R0, each toggle saved to the overlay, travel doors left alone, the
//! pivot map following despawns, the pivot at the rest state outside R0.

use std::sync::{Arc, Mutex, PoisonError};

use bevy_app::App;
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use bevy_transform::components::Transform;
use glam::{DVec3, Quat, Vec2, Vec3};
use mw_interaction::door_def;
use sf_coords::WorldTransform;
use sf_interaction::{DOOR_OPEN, DOOR_STATE_OWNER, DoorState, InvalidReason, VERB_CLOSE};
use sf_interaction_bevy::{DoorRest, Doors, IndexChange, InteractionStateTags, UseApproved};
use sf_physics_bevy::KinematicPlacements;
use sf_records::RecordId;
use sf_script_bevy::sf_script::Handle;
use sf_script_bevy::{Activated, PLAYER};
use sf_world::{DoorLink, IndexedPlacement, Level, LinkEnd, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::{
    ActivateCx, DoorLinks, EndInputs, LoadedModel, ModelTemplate, PlacementSpawner, SpawnCx, StreamingConfig, WorldOverlay,
};

use crate::door::{DoorHook, DoorPivot, DoorPivots};
use crate::index::frame_of;
use crate::tests::{Nothing, app, cell, placement, tick};
use crate::tests_door_body::approve_open;

pub(crate) const DOOR: PlacementId = PlacementId(40);
const BASE: &str = "in_door_01";

/// Records the `parent` and `local` of every spawn; one entity each.
#[derive(Default)]
struct Recorder(Mutex<Vec<(Entity, Transform)>>);

impl Recorder {
    fn calls(&self) -> Vec<(Entity, Transform)> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

impl PlacementSpawner for Recorder {
    fn spawn(&self, _: &mut World, cx: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).push((cx.parent, cx.local));
        1
    }
    fn prepare(&self, _: &AssetServer, _: &IndexedPlacement, _: Option<&LoadedModel>) -> Option<UntypedHandle> {
        None
    }
}

pub(crate) fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

pub(crate) fn door() -> IndexedPlacement {
    let mut p = placement(DOOR, BASE, DVec3::new(10.0, 1.5, -4.0), Quat::from_rotation_y(0.7));
    p.placement.class = PlacementClass::Door;
    p
}

/// A cooked model whose bounds' centre is (0.5, 1, 0).
fn model() -> LoadedModel {
    let aabb = Aabb::from_min_max(Vec3::new(0.0, 0.0, -0.05), Vec3::new(1.0, 2.0, 0.05));
    LoadedModel::Cooked(Arc::new(ModelTemplate { lods: Vec::new(), aabb, ray_tracing: None }))
}

/// The local transform the stream hands the spawner: scale 2, a rotation.
fn local() -> Transform {
    Transform { translation: Vec3::new(3.0, 1.5, -2.0), rotation: Quat::from_rotation_y(0.7), scale: Vec3::splat(2.0) }
}

/// `hook.spawn` under a new parent; returns (count, parent).
pub(crate) fn spawn(app: &mut App, hook: &DoorHook, p: &IndexedPlacement) -> (u32, Entity) {
    let model = model();
    let lod = StreamingConfig::default().lod;
    let layers = RenderLayers::default();
    let world = app.world_mut();
    let parent = world.spawn(Transform::IDENTITY).id();
    let worldspace = ws();
    let cx = SpawnCx {
        worldspace: &worldspace,
        cell: cell(),
        level: Level::Near,
        parent,
        layers: &layers,
        local: local(),
        model: Some(&model),
        max_distance: 200.0,
        end_band: 0.0,
        end: EndInputs::fixed(200.0, 0.0),
        lod: &lod,
    };
    (hook.spawn(world, &cx, p), parent)
}

pub(crate) fn residency(app: &mut App, hook: &DoorHook, p: &IndexedPlacement, on: bool) {
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let worldspace = ws();
    let cx = ActivateCx { worldspace: &worldspace, cell: cell(), cell_entity, discard: false };
    if on {
        hook.activate(world, &cx, p);
    } else {
        hook.deactivate(world, &cx, p);
    }
}

/// Every message of type `M` still in the buffers.
fn sent<M: Message + Clone>(app: &App) -> Vec<M> {
    let messages = app.world().resource::<Messages<M>>();
    messages.get_cursor().read(messages).cloned().collect()
}

fn door_changes(app: &App) -> Vec<IndexChange> {
    sent::<IndexChange>(app).into_iter().filter(|c| c.placement() == DOOR).collect()
}

fn kinematic(app: &App) -> bool {
    app.world().resource::<KinematicPlacements>().contains(&ws(), DOOR)
}

/// An app with the physics listing and an empty overlay.
fn door_app() -> App {
    let mut app = app();
    app.init_resource::<KinematicPlacements>().init_resource::<WorldOverlay>();
    app
}

fn pivots(app: &mut App) -> Vec<(Entity, DoorPivot)> {
    let mut q = app.world_mut().query::<(Entity, &DoorPivot)>();
    q.iter(app.world()).map(|(e, d)| (e, d.clone())).collect()
}

#[test]
fn a_swing_door_spawns_its_visuals_under_a_pivot() {
    let mut app = door_app();
    let recorder = Arc::new(Recorder::default());
    let hook = DoorHook(recorder.clone());
    let p = door();
    let (count, parent) = spawn(&mut app, &hook, &p);
    assert_eq!(count, 2);
    let pivots = pivots(&mut app);
    assert_eq!(pivots.len(), 1);
    let (pivot, d) = &pivots[0];
    let world = app.world();
    assert_eq!(world.get::<ChildOf>(*pivot).map(ChildOf::parent), Some(parent));
    let l = local();
    assert_eq!(world.get::<Transform>(*pivot), Some(&Transform { translation: l.translation, rotation: l.rotation, scale: Vec3::ONE }));
    assert_eq!(d.id, DOOR);
    assert_eq!(d.worldspace, ws());
    assert_eq!(d.origin, p.placement.transform.pos.offset(-l.translation));
    assert_eq!(d.doorway, Vec3::new(1.0, 2.0, 0.0));
    assert_eq!(world.resource::<DoorPivots>().get(DOOR), Some(*pivot));
    assert_eq!(recorder.calls(), vec![(*pivot, Transform::from_scale(l.scale))]);
}

#[test]
fn a_swing_door_is_indexed_and_kinematic_while_resident() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    let p = door();
    spawn(&mut app, &hook, &p);
    residency(&mut app, &hook, &p, true);
    let changes = door_changes(&app);
    assert_eq!(changes.len(), 1);
    let IndexChange::Insert { def, object, disabled, .. } = &changes[0] else { panic!("an insert: {changes:?}") };
    assert_eq!(**def, door_def(&RecordId::new(BASE), Vec3::new(1.0, 2.0, 0.0)));
    assert_eq!(*object, frame_of(&p.placement.transform));
    assert!(disabled.is_empty());
    assert!(kinematic(&app));
    assert!(sent::<DoorRest>(&app).is_empty());
    let swing = app.world().resource::<crate::SwingDoors>();
    assert_eq!(swing.len(), 1);
    assert_eq!(swing.get(DOOR).map(|d| d.transform), Some(p.placement.transform));
    tick(&mut app, 1);
    assert!(app.world().resource::<Doors>().get(DOOR).is_some(), "the shell registered the door");

    residency(&mut app, &hook, &p, false);
    let changes = door_changes(&app);
    assert_eq!(changes.len(), 2);
    assert!(matches!(changes[1], IndexChange::Remove { reason: InvalidReason::OwnerUnloaded, .. }), "{changes:?}");
    assert!(!kinematic(&app));
    assert!(app.world().resource::<crate::SwingDoors>().is_empty());

    residency(&mut app, &hook, &p, true);
    let changes = door_changes(&app);
    assert_eq!(changes.len(), 3);
    assert!(matches!(changes[2], IndexChange::Insert { .. }), "{changes:?}");
    assert!(kinematic(&app));
}

#[test]
fn a_saved_rest_state_is_restored_on_activation() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    let p = door();
    let bytes = DoorState::resting(true, -1).rest_bytes().to_vec();
    app.world().resource::<WorldOverlay>().0.write().unwrap_or_else(PoisonError::into_inner).set_state(DOOR, DOOR_STATE_OWNER, bytes);
    residency(&mut app, &hook, &p, true);
    assert_eq!(sent::<DoorRest>(&app), vec![DoorRest { placement: DOOR, open: true, sign: -1 }]);
    tick(&mut app, 1);
    let door = app.world().resource::<Doors>().get(DOOR).expect("registered");
    assert!(door.state.open);
    assert!((door.state.t - 1.0).abs() < f32::EPSILON, "t = {}", door.state.t);
    assert_eq!(door.state.sign, -1);
    assert!(app.world().resource::<InteractionStateTags>().tags(DOOR).contains(&DOOR_OPEN));
}

#[test]
fn a_rest_blob_that_does_not_decode_restores_nothing() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    app.world().resource::<WorldOverlay>().0.write().unwrap_or_else(PoisonError::into_inner).set_state(DOOR, DOOR_STATE_OWNER, vec![9]);
    residency(&mut app, &hook, &door(), true);
    assert_eq!(door_changes(&app).len(), 1);
    assert!(sent::<DoorRest>(&app).is_empty());
}

#[test]
fn a_travel_door_is_left_to_the_inner_spawner() {
    let mut app = door_app();
    let end = |door: Option<PlacementId>| LinkEnd {
        worldspace: ws(),
        frame: WorldTransform::from_pos(sf_coords::WorldPos::ORIGIN),
        arrival: WorldTransform::from_pos(sf_coords::WorldPos::ORIGIN),
        door,
        extent: Vec2::ZERO,
    };
    app.insert_resource(DoorLinks(vec![DoorLink { a: end(None), b: end(Some(DOOR)), two_way: true }]));
    let recorder = Arc::new(Recorder::default());
    let hook = DoorHook(recorder.clone());
    let p = door();
    let (count, parent) = spawn(&mut app, &hook, &p);
    assert_eq!(count, 1);
    assert!(pivots(&mut app).is_empty());
    assert_eq!(recorder.calls(), vec![(parent, local())]);
    residency(&mut app, &hook, &p, true);
    assert!(door_changes(&app).is_empty());
    assert!(!kinematic(&app));
}

#[test]
fn a_despawned_pivot_leaves_the_pivot_map() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    spawn(&mut app, &hook, &door());
    let pivot = app.world().resource::<DoorPivots>().get(DOOR).expect("spawned");
    app.world_mut().despawn(pivot);
    assert_eq!(app.world().resource::<DoorPivots>().get(DOOR), None);
}

fn approve_close(app: &mut App) {
    let activation = Activated { target: Handle(DOOR.0), actor: PLAYER, verb: Some(VERB_CLOSE.0) };
    app.world_mut().write_message(UseApproved { activation, verb: Some(VERB_CLOSE), slot: None });
}

/// The door's overlay blob, raw.
fn blob(app: &App) -> Option<Vec<u8>> {
    let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.state(DOOR, DOOR_STATE_OWNER).map(<[u8]>::to_vec)
}

/// The rest state the door's blob decodes to.
fn saved(app: &App) -> Option<DoorState> {
    blob(app).and_then(|b| DoorState::from_rest_bytes(&b))
}

fn shell_state(app: &App) -> DoorState {
    app.world().resource::<Doors>().get(DOOR).expect("registered").state
}

/// A resident door the shell registered.
fn resident(app: &mut App) -> DoorHook {
    let hook = DoorHook(Arc::new(Nothing));
    residency(app, &hook, &door(), true);
    tick(app, 1);
    hook
}

#[test]
fn an_opened_door_writes_an_open_rest() {
    let mut app = door_app();
    resident(&mut app);
    assert_eq!(blob(&app), None);
    approve_open(&mut app);
    app.update();
    let shell = shell_state(&app);
    assert!(shell.open && shell.is_moving(), "mid-swing: {shell:?}");
    assert_eq!(saved(&app), Some(DoorState::resting(true, shell.sign)));
}

#[test]
fn a_closed_door_writes_a_closed_rest() {
    let mut app = door_app();
    resident(&mut app);
    approve_open(&mut app);
    app.update();
    tick(&mut app, 200);
    let shell = shell_state(&app);
    assert!(shell.open && !shell.is_moving(), "at rest open: {shell:?}");
    approve_close(&mut app);
    app.update();
    assert!(shell_state(&app).is_moving(), "closing");
    assert_eq!(saved(&app), Some(DoorState::resting(false, shell.sign)));

    approve_open(&mut app);
    app.update();
    approve_close(&mut app);
    app.update();
    assert_eq!(saved(&app).map(|s| s.open), Some(false), "a mid-swing close");
}

#[test]
fn an_open_door_is_open_again_after_its_cell_reloads() {
    let mut app = door_app();
    let hook = resident(&mut app);
    approve_open(&mut app);
    app.update();
    let sign = shell_state(&app).sign;
    residency(&mut app, &hook, &door(), false);
    tick(&mut app, 1);
    assert!(app.world().resource::<Doors>().get(DOOR).is_none(), "the shell dropped the door");
    for _ in 0..3 {
        app.update();
    }
    residency(&mut app, &hook, &door(), true);
    tick(&mut app, 1);
    assert_eq!(shell_state(&app), DoorState::resting(true, sign));
    assert!(app.world().resource::<InteractionStateTags>().tags(DOOR).contains(&DOOR_OPEN));
}

fn set_blob(app: &App, bytes: Vec<u8>) {
    app.world().resource::<WorldOverlay>().0.write().unwrap_or_else(PoisonError::into_inner).set_state(DOOR, DOOR_STATE_OWNER, bytes);
}

fn pivot_transform(app: &App) -> Transform {
    let e = app.world().resource::<DoorPivots>().get(DOOR).expect("a pivot");
    *app.world().get::<Transform>(e).expect("a pivot transform")
}

/// The closed pivot `spawn` gives: the stream's translation and rotation, unit scale.
fn closed_pivot() -> Transform {
    let l = local();
    Transform { translation: l.translation, rotation: l.rotation, scale: Vec3::ONE }
}

/// The pivot at the open rest of `sign`: the closed translation (hinge at the origin), the closed
/// rotation turned by the def's `open_deg` about the object's up axis.
fn open_pivot(sign: i8) -> Transform {
    let def = door_def(&RecordId::new(BASE), Vec3::ZERO);
    let open_deg = def.door().expect("a door slot").2.open_deg;
    let closed = closed_pivot();
    Transform { rotation: closed.rotation * Quat::from_rotation_y(f32::from(sign) * open_deg.to_radians()), ..closed }
}

fn assert_near(got: &Transform, want: &Transform) {
    assert!(got.translation.distance(want.translation) < 1e-4, "translation {got:?} != {want:?}");
    assert!(got.rotation.angle_between(want.rotation) < 1e-3, "rotation {got:?} != {want:?}");
    assert_eq!(got.scale, want.scale);
}

#[test]
fn a_door_saved_open_spawns_open() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    let p = door();
    set_blob(&app, DoorState::resting(true, 1).rest_bytes().to_vec());
    spawn(&mut app, &hook, &p);
    let spawned = pivot_transform(&app);
    assert_near(&spawned, &open_pivot(1));
    assert!(spawned.rotation.angle_between(closed_pivot().rotation) > 1.0, "turned: {spawned:?}");

    residency(&mut app, &hook, &p, true);
    tick(&mut app, 1);
    app.update();
    assert_eq!(app.world().resource::<Doors>().get(DOOR).map(|d| d.state), Some(DoorState::resting(true, 1)));
    assert_near(&pivot_transform(&app), &spawned);
}

#[test]
fn a_door_with_no_open_rest_spawns_closed() {
    let blobs = [None, Some(DoorState::resting(false, 1).rest_bytes().to_vec()), Some(vec![9])];
    for blob in blobs {
        let mut app = door_app();
        let hook = DoorHook(Arc::new(Nothing));
        if let Some(bytes) = blob.clone() {
            set_blob(&app, bytes);
        }
        spawn(&mut app, &hook, &door());
        assert_eq!(pivot_transform(&app), closed_pivot(), "blob {blob:?}");
    }
}

#[test]
fn a_door_leaving_r0_mid_swing_rests_at_its_target() {
    let mut app = door_app();
    let hook = DoorHook(Arc::new(Nothing));
    let p = door();
    spawn(&mut app, &hook, &p);
    residency(&mut app, &hook, &p, true);
    tick(&mut app, 1);
    approve_open(&mut app);
    app.update();
    tick(&mut app, 3);
    app.update();
    let shell = shell_state(&app);
    assert!(shell.open && shell.is_moving(), "mid-swing: {shell:?}");
    let open = open_pivot(shell.sign);
    let swinging = pivot_transform(&app);
    assert!(swinging.rotation.angle_between(open.rotation) > 1e-2, "not yet open: {swinging:?}");
    assert!(swinging.rotation.angle_between(closed_pivot().rotation) > 1e-3, "moving: {swinging:?}");

    residency(&mut app, &hook, &p, false);
    assert_near(&pivot_transform(&app), &open);
    app.update();
    assert_near(&pivot_transform(&app), &open);
}

#[test]
fn a_door_that_never_toggled_has_no_blob() {
    let mut app = door_app();
    let hook = resident(&mut app);
    for _ in 0..3 {
        app.update();
    }
    residency(&mut app, &hook, &door(), false);
    tick(&mut app, 1);
    app.update();
    assert_eq!(blob(&app), None);
}
