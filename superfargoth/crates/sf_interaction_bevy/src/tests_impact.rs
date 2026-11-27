//! `IN4b` headless tests (`physics` feature): physics contacts into impact [`PropDamage`]. A
//! registered prop dropped 3 m gets one hit per gated contact with the core's damage of that
//! contact's own impulse and speed (two steps a frame, each event once) and breaks at 1 hp; a
//! 0.2 m drop gives none; a body falling on a registered fixed placement damages that placement
//! only; a light `DEBRIS`-layer item counts; unregistered pairs, debris-shell pieces, projectiles
//! and held bodies give none.

use bevy_app::First;
use glam::Vec3;
use sf_coords::{WorldPos, WorldTransform};
use sf_interaction::{BreakAction, DAMAGE_IMPACT, DestructibleDef, Stage, impact_damage};
use sf_physics_bevy::sf_physics::{BodyDesc, BodyId, BodyKind, Layer, PhysicsEvent, ShapeRef};
use sf_physics_bevy::{Physics, PhysicsBody, PhysicsEvents, PhysicsStreaming, PhysicsSystems};
use sf_records::RecordId;
use sf_tunables_bevy::TunablesPlugin;
use sf_world::WorldspaceId;
use smallvec::{SmallVec, smallvec};

use super::*;

/// The game's step: the active world, its events into the frame's [`PhysicsEvents`].
fn step_world(mut physics: ResMut<Physics>, mut events: ResMut<PhysicsEvents>) {
    let Some(w) = physics.worlds.active_world_mut() else { return };
    w.step();
    events.0.extend(w.drain_events());
}

/// Every impact hit, as the damage step sees it.
#[derive(Resource, Default)]
struct Seen(Vec<PropDamage>);

fn record(mut hits: MessageReader<PropDamage>, mut seen: ResMut<Seen>) {
    seen.0.extend(hits.read().copied());
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

/// A host with one active world: a floor whose top is at y = 0. No `PhysicsFocus`: no debris.
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Physics>()
        .init_resource::<PhysicsEvents>()
        .init_resource::<PhysicsStreaming>()
        .init_resource::<Seen>()
        .add_systems(FixedUpdate, step_world.in_set(PhysicsSystems::Step))
        .add_systems(FixedUpdate, record.after(ImpactSystems).before(DestroySystems::Damage));
    // Binds the tunables (PreUpdate); no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    let mut physics = app.world_mut().resource_mut::<Physics>();
    let floor = WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(0.0, -0.5, 0.0)), ..WorldTransform::IDENTITY };
    physics.insert(&ws(), &BodyDesc::fixed(floor, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));
    physics.worlds.set_active(Some(ws()));
    app
}

/// One frame: the events cleared and the cursor reset (`First`), then `steps` fixed ticks.
fn frame(app: &mut App, steps: usize) {
    app.world_mut().resource_mut::<PhysicsEvents>().0.clear();
    app.world_mut().run_schedule(First);
    for _ in 0..steps {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

/// Registers `placement` with `health` hp and one removing stage at 0.
fn register(app: &mut App, placement: PlacementId, health: f32) {
    let stage =
        Stage { at: 0.0, model: None, debris: smallvec![], chunks: None, effect: None, on_break: BreakAction::None };
    let def = Arc::new(InteractableDef {
        id: RecordId::new("jar"),
        slots: Vec::new(),
        verbs: SmallVec::new(),
        material: None,
        destructible: Some(DestructibleDef {
            health,
            stages: smallvec![stage],
            ignore_external: false,
            loot: None,
            salvage: smallvec![],
        }),
        carry: None,
        adverts: Vec::new(),
    });
    app.world_mut().write_message(IndexChange::Insert {
        placement,
        def,
        object: Frame::default(),
        owner: Entity::PLACEHOLDER,
        disabled: SmallVec::new(),
    });
    frame(app, 1);
}

/// A body at `pos` (y up from the floor top).
fn body(app: &mut App, desc: BodyDesc) -> BodyId {
    app.world_mut().resource_mut::<Physics>().insert(&ws(), &desc)
}

fn at(pos: Vec3) -> WorldTransform {
    WorldTransform { pos: WorldPos::ORIGIN.offset(pos), ..WorldTransform::IDENTITY }
}

fn dynamic(pos: Vec3, shape: ShapeRef, layer: Layer, mass: f32) -> BodyDesc {
    BodyDesc { mass: Some(mass), ccd: true, ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(pos), shape, layer) }
}

fn place(app: &mut App, placement: PlacementId, id: BodyId) {
    app.world_mut().resource_mut::<PhysicsStreaming>().insert_placed(&ws(), placement, id);
}

fn live(app: &App, placement: PlacementId) -> bool {
    app.world().resource::<Destructibles>().get(placement).is_some_and(|d| !d.gone)
}

/// A hit's exact bits, for comparing sets.
type Key = (PlacementId, u32, [u32; 3], [u32; 3]);

fn key(d: &PropDamage) -> Key {
    let bits = |v: Vec3| v.to_array().map(f32::to_bits);
    (d.placement, d.amount.to_bits(), bits(d.point), bits(d.dir))
}

/// The frame's contacts with a nonzero core damage: (a, b, damage, point, normal).
fn gated(app: &App) -> Vec<(BodyId, BodyId, f32, Vec3, Vec3)> {
    let p = app.world().resource::<InteractionSettings>().0;
    app.world()
        .resource::<PhysicsEvents>()
        .0
        .iter()
        .filter_map(|e| match *e {
            PhysicsEvent::Contact { a, b, impulse, speed, point, normal, .. } => {
                let amount = impact_damage(impulse, speed, &p);
                (amount > 0.0).then(|| (a, b, amount, point.to_f64().as_vec3(), normal))
            }
            _ => None,
        })
        .collect()
}

/// The spec's hits for this frame's contacts, for the `sides` (body -> live placement).
fn expected(app: &App, sides: &[(BodyId, PlacementId)]) -> Vec<Key> {
    let mut out = Vec::new();
    for (a, b, amount, point, normal) in gated(app) {
        for (id, dir) in [(a, -normal), (b, normal)] {
            for &(_, placement) in sides.iter().filter(|s| s.0 == id) {
                out.push(key(&PropDamage { placement, amount, kind: DAMAGE_IMPACT, point, dir, culprit: None }));
            }
        }
    }
    out.sort_unstable();
    out
}

/// Runs `frames` frames of `steps` ticks; each frame's hits must be exactly [`expected`] for the
/// sides still live at its start. Returns every hit.
fn run_checked(app: &mut App, frames: usize, steps: usize, sides: &[(BodyId, PlacementId)]) -> Vec<PropDamage> {
    let mut all = Vec::new();
    for f in 0..frames {
        let alive: Vec<(BodyId, PlacementId)> = sides.iter().copied().filter(|s| live(app, s.1)).collect();
        app.world_mut().resource_mut::<Seen>().0.clear();
        frame(app, steps);
        let seen = std::mem::take(&mut app.world_mut().resource_mut::<Seen>().0);
        let mut got: Vec<Key> = seen.iter().map(key).collect();
        got.sort_unstable();
        assert_eq!(got, expected(app, &alive), "frame {f}");
        for h in &seen {
            assert_eq!((h.kind, h.culprit), (DAMAGE_IMPACT, None));
            assert!(h.amount.is_finite() && h.amount > 0.0 && h.point.is_finite() && h.dir.is_finite());
        }
        all.extend(seen);
    }
    all
}

fn drop_jar(app: &mut App, health: f32) -> BodyId {
    register(app, PlacementId(1), health);
    let jar = body(app, dynamic(Vec3::new(0.0, 3.1, 0.0), ShapeRef::Box(Vec3::splat(0.1)), Layer::PROP, 1.0));
    place(app, PlacementId(1), jar);
    jar
}

#[test]
fn a_dropped_prop_takes_each_contacts_own_damage_and_breaks() {
    let mut app = app();
    let jar = drop_jar(&mut app, 1.0);
    let hits = run_checked(&mut app, 90, 1, &[(jar, PlacementId(1))]);
    let first = hits.first().expect("the landing hits");
    assert!(first.amount > 1.0, "a 3 m drop breaks a 1 hp jar: {}", first.amount);
    assert!(!live(&app, PlacementId(1)), "broken");
}

#[test]
fn two_steps_in_one_frame_read_each_event_once() {
    let mut app = app();
    // Too tough to break: every frame's contacts stay on a live side.
    let jar = drop_jar(&mut app, 1000.0);
    let hits = run_checked(&mut app, 45, 2, &[(jar, PlacementId(1))]);
    assert!(!hits.is_empty(), "the landing hits");
}

#[test]
fn a_short_drop_gives_no_damage() {
    let mut app = app();
    register(&mut app, PlacementId(1), 1.0);
    let jar = body(&mut app, dynamic(Vec3::new(0.0, 0.3, 0.0), ShapeRef::Box(Vec3::splat(0.1)), Layer::PROP, 1.0));
    place(&mut app, PlacementId(1), jar);
    let hits = run_checked(&mut app, 60, 1, &[(jar, PlacementId(1))]);
    assert!(hits.is_empty(), "{hits:?}");
    let health = app.world().resource::<Destructibles>().get(PlacementId(1)).map(|d| d.state.health);
    assert_eq!(health, Some(1.0));
}

#[test]
fn a_body_falling_on_a_registered_fixed_placement_damages_only_it() {
    let mut app = app();
    register(&mut app, PlacementId(2), 15.0);
    let crate_ = body(&mut app, BodyDesc::fixed(at(Vec3::new(0.0, 0.25, 0.0)), ShapeRef::Box(Vec3::splat(0.25)), Layer::STATIC));
    place(&mut app, PlacementId(2), crate_);
    body(&mut app, dynamic(Vec3::new(0.0, 3.6, 0.0), ShapeRef::Sphere(0.1), Layer::PROP, 1.0));
    let hits = run_checked(&mut app, 90, 1, &[(crate_, PlacementId(2))]);
    assert!(!hits.is_empty(), "the pot lands on the crate");
    assert!(hits.iter().all(|h| h.placement == PlacementId(2)), "the unregistered pot takes nothing");
    let health = app.world().resource::<Destructibles>().get(PlacementId(2)).map(|d| d.state.health);
    assert!(health.is_some_and(|h| h < 15.0), "chipped: {health:?}");
}

#[test]
fn a_light_item_on_the_debris_layer_takes_impact_damage() {
    let mut app = app();
    register(&mut app, PlacementId(3), 1.0);
    let bottle = body(&mut app, dynamic(Vec3::new(0.0, 3.05, 0.0), ShapeRef::Sphere(0.05), Layer::DEBRIS, 0.4));
    place(&mut app, PlacementId(3), bottle);
    let hits = run_checked(&mut app, 90, 1, &[(bottle, PlacementId(3))]);
    assert!(!hits.is_empty(), "a DEBRIS-layer item is not filtered");
    assert!(!live(&app, PlacementId(3)), "a 3 m drop breaks a 1 hp bottle");
}

#[test]
fn no_damage_from_unregistered_pairs_debris_pieces_projectiles_or_held_bodies() {
    let mut app = app();
    register(&mut app, PlacementId(2), 15.0);
    let slab = body(&mut app, BodyDesc::fixed(at(Vec3::new(0.0, 0.25, 0.0)), ShapeRef::Box(Vec3::new(3.0, 0.25, 0.5)), Layer::STATIC));
    place(&mut app, PlacementId(2), slab);
    let drop = |x: f32, layer: Layer| dynamic(Vec3::new(x, 3.6, 0.0), ShapeRef::Sphere(0.1), layer, 1.0);
    let piece = body(&mut app, drop(-2.0, Layer::DEBRIS));
    let ws = ws();
    app.world_mut().spawn((
        DebrisPiece { set: RecordId::new("debris.shards"), piece: 0, placement: PlacementId(9), order: 0, age_s: 0.0 },
        PhysicsBody { worldspace: ws, id: piece },
    ));
    let projectile = body(&mut app, drop(0.0, Layer::PROJECTILE));
    let held = body(&mut app, drop(2.0, Layer::HELD));
    // Two unregistered bodies landing on the unregistered floor.
    let loose = body(&mut app, drop(6.0, Layer::PROP));
    let mut touched = Vec::new();
    for _ in 0..90 {
        app.world_mut().resource_mut::<Seen>().0.clear();
        frame(&mut app, 1);
        assert!(app.world().resource::<Seen>().0.is_empty(), "{:?}", app.world().resource::<Seen>().0);
        touched.extend(gated(&app).into_iter().flat_map(|(a, b, ..)| [a, b]));
    }
    for id in [piece, projectile, held, loose] {
        assert!(touched.contains(&id), "{id:?} landed hard (the test is not vacuous)");
    }
    let health = app.world().resource::<Destructibles>().get(PlacementId(2)).map(|d| d.state.health);
    assert_eq!(health, Some(15.0));
}
