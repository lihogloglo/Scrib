//! Spill motion (interaction-destruction.md "Spill", row `IN38b3b`): the `physics` overlay blob
//! ([`ItemMotion`]) is read on item activation. An awake blob puts the item
//! in awake with its velocities and never held; no blob, or an asleep one,
//! leaves it as before; the deactivation writer's blob decodes to the
//! velocities the body had.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::cook::{FitSettings, fit_render};
use sf_physics::{BodyDesc, BodyId, BodyKind, Layer, ShapeRef};
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, ActiveWorldspace, PlacementSpawner, SpawnCx, WorldOverlay};

use crate::{ItemBody, ItemContent, ItemMotion, Physics, PhysicsBodySource, PhysicsHook, PhysicsPlugin, PhysicsShapeSource, PhysicsStreaming, STATE_OWNER};

const ID: PlacementId = PlacementId(9);

struct Nothing;
impl PlacementSpawner for Nothing {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
}

/// Every item is held by the adapter's rule (`hold`), so a body that goes in
/// dynamic shows the blob overrode it.
struct Items {
    hold: bool,
}
impl ItemContent for Items {
    fn is_item(&self, p: &Placement) -> bool {
        p.class == PlacementClass::Dynamic
    }
    fn item_body(&self, _: &WorldspaceId, _: &Placement, _: f32) -> ItemBody {
        ItemBody { mass: Some(1.0), hold: self.hold }
    }
}

fn cube() -> Arc<sf_physics::BodyShape> {
    let h = 0.1;
    let p: Vec<Vec3> = (0..8)
        .map(|i| Vec3::new(if i & 1 == 0 { -h } else { h }, if i & 2 == 0 { -h } else { h }, if i & 4 == 0 { -h } else { h }))
        .collect();
    let t = vec![[0, 2, 1], [1, 2, 3], [4, 5, 6], [5, 7, 6], [0, 1, 4], [1, 5, 4], [2, 6, 3], [3, 6, 7], [0, 4, 2], [2, 4, 6], [1, 3, 5], [3, 7, 5]];
    Arc::new(fit_render(&p, &t, &FitSettings::default()).unwrap())
}

fn base() -> WorldPos {
    WorldPos::new(IVec3::new(1, 0, 1), Vec3::new(5.0, 0.0, 5.0))
}

/// A world with a ground box and an item 1 m above it.
fn app(ws: &WorldspaceId, hold: bool) -> App {
    bevy_tasks::AsyncComputeTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let mut app = App::new();
    app.add_plugins((TimePlugin, PhysicsPlugin::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(ActiveWorldspace(Some(ws.clone())));
    let shape = cube();
    app.insert_resource(PhysicsBodySource { loader: Arc::new(move |_: &str| Some(shape.clone())), items: Arc::new(Items { hold }), supports: None })
        .insert_resource(PhysicsShapeSource(Arc::new(|_: &str| None)))
        .init_resource::<WorldOverlay>();
    let ground = WorldTransform { pos: base().offset(Vec3::new(0.0, -0.5, 0.0)), ..WorldTransform::IDENTITY };
    app.world_mut().resource_mut::<Physics>().insert(ws, &BodyDesc::fixed(ground, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));
    app
}

fn item() -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: ID,
            base: RecordRef::new("MISC", "cube"),
            model: Some(sf_vfs::VfsPath::new("cube.nif").unwrap()),
            transform: WorldTransform { pos: base().offset(Vec3::new(0.0, 1.0, 0.0)), ..WorldTransform::IDENTITY },
            class: PlacementClass::Dynamic,
            flags: PlacementFlags::empty(),
        },
        radius: 0.2,
    }
}

fn set_blob(app: &App, motion: ItemMotion) {
    app.world().resource::<WorldOverlay>().0.write().unwrap().set_state(ID, STATE_OWNER, motion.encode());
}

/// Activates the item and updates until its body is in (the frame it goes
/// in has no physics step after the insert: `FixedUpdate` runs before `Update`).
fn activate(app: &mut App, ws: &WorldspaceId, hook: &PhysicsHook, cell_entity: Entity) -> BodyId {
    hook.activate(app.world_mut(), &ActivateCx { worldspace: ws, cell: IVec3::ZERO, cell_entity, discard: false }, &item());
    for _ in 0..500 {
        app.update();
        if let Some(b) = app.world().resource::<PhysicsStreaming>().body(ws, ID) {
            return b;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("item not inserted");
}

fn hook() -> PhysicsHook {
    PhysicsHook { inner: Arc::new(Nothing), statics: true }
}

#[test]
fn motion_blob_round_trips_and_rejects_others() {
    let m = ItemMotion { asleep: false, linear: Vec3::new(1.5, -2.0, 0.25), angular: Vec3::new(0.0, 3.0, -1.0) };
    let blob = m.encode();
    assert_eq!(blob.len(), 26);
    assert_eq!(blob[0], crate::STATE_VERSION);
    assert_eq!(ItemMotion::decode(&blob), Some(m));
    assert_eq!(ItemMotion::decode(&ItemMotion::DROPPED.encode()), Some(ItemMotion::DROPPED));
    let mut other = blob.clone();
    other[0] = 2;
    assert_eq!(ItemMotion::decode(&other), None, "another version");
    assert_eq!(ItemMotion::decode(&blob[..25]), None, "short");
    let nan = ItemMotion { linear: Vec3::NAN, ..m }.encode();
    assert_eq!(ItemMotion::decode(&nan), None, "non-finite");
}

#[test]
fn awake_blob_goes_in_awake_with_its_velocity_never_held() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws, true);
    let motion = ItemMotion { asleep: false, linear: Vec3::new(0.0, -2.0, 0.0), angular: Vec3::new(0.0, 1.0, 0.0) };
    set_blob(&app, motion);
    let cell = app.world_mut().spawn_empty().id();
    let b = activate(&mut app, &ws, &hook(), cell);
    let w = app.world().resource::<Physics>().world(&ws).unwrap();
    assert_eq!(w.kind(b), Some(BodyKind::Dynamic { asleep: false }), "awake dynamic, not held");
    assert_eq!(w.is_sleeping(b), Some(false));
    assert_eq!(w.velocity(b), Some((motion.linear, motion.angular)));
    // It falls: the next steps take it down toward the ground.
    let start = w.pose(b).unwrap().pos;
    for _ in 0..10 {
        app.update();
    }
    let now = app.world().resource::<Physics>().world(&ws).unwrap().pose(b).unwrap().pos;
    assert!(start.delta(now).y < -0.1, "falling, moved {:?}", start.delta(now));
}

#[test]
fn no_blob_or_asleep_blob_goes_in_as_before() {
    let ws = WorldspaceId::new("test");
    // No blob: the adapter's rule (not held -> asleep dynamic; held -> held).
    for (hold, expect) in [(false, BodyKind::Dynamic { asleep: true }), (true, BodyKind::Held)] {
        let mut app = app(&ws, hold);
        let cell = app.world_mut().spawn_empty().id();
        let b = activate(&mut app, &ws, &hook(), cell);
        let w = app.world().resource::<Physics>().world(&ws).unwrap();
        assert_eq!(w.kind(b), Some(expect), "no blob, hold {hold}");
        assert_eq!(w.velocity(b), Some((Vec3::ZERO, Vec3::ZERO)));
    }
    // An asleep blob: as no blob.
    let mut app = app(&ws, true);
    set_blob(&app, ItemMotion { asleep: true, linear: Vec3::new(0.0, -2.0, 0.0), angular: Vec3::ZERO });
    let cell = app.world_mut().spawn_empty().id();
    let b = activate(&mut app, &ws, &hook(), cell);
    let w = app.world().resource::<Physics>().world(&ws).unwrap();
    assert_eq!(w.kind(b), Some(BodyKind::Held), "asleep blob keeps the adapter's hold");
    assert_eq!(w.velocity(b), Some((Vec3::ZERO, Vec3::ZERO)));
}

#[test]
fn item_leaving_mid_flight_comes_back_with_its_velocity() {
    let ws = WorldspaceId::new("test");
    let mut app = app(&ws, false);
    let hook = hook();
    let cell = app.world_mut().spawn_empty().id();
    let b = activate(&mut app, &ws, &hook, cell);
    app.world_mut().resource_mut::<Physics>().world_mut(&ws).unwrap().apply_impulse(b, Vec3::new(2.0, 1.0, 0.0), None);
    for _ in 0..3 {
        app.update();
    }
    let (linear, angular) = app.world().resource::<Physics>().world(&ws).unwrap().velocity(b).unwrap();
    assert!(linear.length() > 0.5, "moving at deactivation");
    let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity: cell, discard: false };
    hook.deactivate(app.world_mut(), &cx, &item());
    let written = {
        let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap();
        ItemMotion::decode(overlay.state(ID, STATE_OWNER).expect("blob written")).expect("v1 blob")
    };
    assert_eq!(written, ItemMotion { asleep: false, linear, angular }, "the writer's blob decodes to its values");
    let b = activate(&mut app, &ws, &hook, cell);
    let w = app.world().resource::<Physics>().world(&ws).unwrap();
    assert_eq!(w.kind(b), Some(BodyKind::Dynamic { asleep: false }));
    assert_eq!(w.velocity(b), Some((linear, angular)), "comes back with the velocity it left with");
}
