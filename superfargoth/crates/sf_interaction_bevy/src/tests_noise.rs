//! `IN18b` headless tests: the tick's door toggles and breaks become [`InteractionNoise`] through
//! the core's merge. One door toggle is one revealed noise at the doorway in the world with the door
//! reach x the material ratio (the reference material when the table lacks it); two stages of one
//! break in a tick are one noise at the object with the material's reach x `speed_mul_max`; an
//! unregistered placement makes none. With `physics`: one object's rattle within `merge_s` is one
//! noise (a quieter contact is not heard, a louder one is), heard again once the window ends; a
//! thrown body's landing names the thrower and does not reveal it; debris-shell pieces and held
//! bodies are silent. `DoorChanged`, `Destroyed` and the contacts are written directly: their
//! producers are tested in `tests_door`, `tests_destroy` and `tests_impact`.

use glam::{Quat, Vec3};
use sf_ids::PlayerKey;
use sf_interaction::{
    BreakAction, ClipFamily, DOOR_OPEN, DestructibleDef, DoorDef, IkTargets, MaterialDef, NoiseCase, Occupancy,
    SlotDef, SlotKind, Stage, TagFilter, TagId, Tick, UserKey, VERB_OPEN, VerbDef,
};
use sf_records::RecordId;
use sf_tunables_bevy::TunablesPlugin;
use smallvec::{SmallVec, smallvec};

use super::*;

const DOOR: PlacementId = PlacementId(10);
const JAR: PlacementId = PlacementId(11);
const WOOD: TagId = TagId::of("wood");
const GLASS: TagId = TagId::of("glass");

pub(super) fn player() -> UserKey {
    UserKey::Player(PlayerKey(1))
}

fn material(id: TagId, reach: f32) -> MaterialDef {
    MaterialDef { id, flammable: None, noise: TagId::of("impact.test"), impact_reach_m: reach }
}

/// A swing door of `material` whose leaf centre is 1 m above the object's origin.
pub(super) fn door(material: Option<TagId>) -> Arc<InteractableDef> {
    let leaf = DoorDef { hinge: Vec3::new(0.5, 1.0, 0.0), axis: Vec3::Y, open_deg: 90.0, both_ways: false, close_behind: false };
    let verb = VerbDef { verb: VERB_OPEN, priority: 0, requires: TagFilter { none: smallvec![DOOR_OPEN], ..TagFilter::default() } };
    let slot = SlotDef {
        kind: SlotKind::Door(leaf),
        local: Frame { translation: Vec3::new(0.0, 1.0, 0.0), rotation: Quat::IDENTITY },
        band: 0.0,
        verb,
        entrances: smallvec![],
        clips: ClipFamily("door".into()),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    Arc::new(InteractableDef {
        id: RecordId::new("door"),
        slots: vec![slot],
        verbs: SmallVec::new(),
        material,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}

/// A jar of `material` with `health` hp and one removing stage at 0.
pub(super) fn jar(material: Option<TagId>, health: f32) -> Arc<InteractableDef> {
    let stage =
        Stage { at: 0.0, model: None, debris: smallvec![], chunks: None, effect: None, on_break: BreakAction::None };
    Arc::new(InteractableDef {
        id: RecordId::new("jar"),
        slots: Vec::new(),
        verbs: SmallVec::new(),
        material,
        destructible: Some(DestructibleDef {
            health,
            stages: smallvec![stage],
            ignore_external: false,
            loot: None,
            salvage: smallvec![],
        }),
        carry: None,
        adverts: Vec::new(),
    })
}

#[derive(Resource, Default)]
struct Heard(Vec<InteractionNoise>);

fn hear(mut noises: MessageReader<InteractionNoise>, mut heard: ResMut<Heard>) {
    heard.0.extend(noises.read().cloned());
}

/// A host with wood (15 m) and glass (6 m) in the material table.
pub(super) fn base() -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_resource::<Heard>()
        .add_systems(FixedUpdate, hear.after(NoiseSystems::Emit));
    // Binds the tunables; no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    let mut materials = app.world_mut().resource_mut::<InteractionMaterials>();
    materials.insert(material(WOOD, 15.0));
    materials.insert(material(GLASS, 6.0));
    app
}

pub(super) fn insert(app: &mut App, placement: PlacementId, def: Arc<InteractableDef>, object: Frame) {
    let owner = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(IndexChange::Insert { placement, def, object, owner, disabled: SmallVec::new() });
    app.world_mut().run_schedule(FixedUpdate);
}

/// One fixed tick; the noises heard on it.
fn tick(app: &mut App) -> Vec<InteractionNoise> {
    app.world_mut().resource_mut::<Heard>().0.clear();
    app.world_mut().run_schedule(FixedUpdate);
    std::mem::take(&mut app.world_mut().resource_mut::<Heard>().0)
}

pub(super) fn now(app: &App) -> Tick {
    app.world().resource::<InteractionTick>().0
}

fn toggle(app: &mut App, placement: PlacementId, open: bool, user: Option<UserKey>) {
    let t = now(app);
    app.world_mut().write_message(DoorChanged(sf_interaction::DoorChanged { placement, open, user, tick: t }));
}

#[test]
fn a_door_toggle_is_one_revealed_noise_at_the_doorway_scaled_by_its_material() {
    let mut app = base();
    let object = Frame { translation: Vec3::new(4.0, 0.0, -2.0), rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2) };
    insert(&mut app, DOOR, door(Some(WOOD)), object);
    let p = app.world().resource::<InteractionSettings>().0;

    toggle(&mut app, DOOR, true, Some(player()));
    let heard = tick(&mut app);
    let doorway = object.translation + object.rotation * Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(heard.len(), 1, "{heard:?}");
    let n = &heard[0];
    assert_eq!(n.tick, now(&app));
    assert_eq!(n.at, doorway);
    assert_eq!(n.space, None);
    assert_eq!(n.kind, NOISE_DOOR);
    assert_eq!(n.case, NoiseCase::Door);
    assert_eq!(n.maker, Some(player()));
    assert!(n.reveals_maker);
    assert_eq!(n.loudness_m.to_bits(), (p.noise_door_reach_m * (15.0 / p.noise_material_ref_m)).to_bits());
    assert_eq!(n.loudness_m.to_bits(), 12.0_f32.to_bits(), "8 m x 15 / 10");

    // A door whose material the table lacks is the reference material; a close-behind has no maker.
    let mut app = base();
    insert(&mut app, DOOR, door(Some(TagId::of("unknown"))), Frame::default());
    toggle(&mut app, DOOR, false, None);
    let heard = tick(&mut app);
    assert_eq!(heard.len(), 1, "{heard:?}");
    assert_eq!(heard[0].loudness_m.to_bits(), p.noise_door_reach_m.to_bits());
    assert_eq!(heard[0].maker, None);
    assert!(heard[0].reveals_maker);

    // Nothing more without a toggle; a toggle of no registered door makes none.
    assert!(tick(&mut app).is_empty());
    toggle(&mut app, PlacementId(99), true, Some(player()));
    assert!(tick(&mut app).is_empty());
}

#[test]
fn a_break_is_one_noise_at_the_object_with_its_material_reach_times_the_speed_cap() {
    let mut app = base();
    let object = Frame { translation: Vec3::new(1.0, 0.5, 3.0), rotation: Quat::IDENTITY };
    insert(&mut app, JAR, jar(Some(GLASS), 1.0), object);
    let p = app.world().resource::<InteractionSettings>().0;
    let t = now(&app);
    let destroyed = |stage: u8| {
        Destroyed(sf_interaction::Destroyed {
            tick: t,
            placement: JAR,
            stage,
            point: Vec3::new(1.2, 0.6, 3.0),
            dir: Vec3::NEG_Y,
            seed: 7,
            culprit: Some(player()),
            overkill: 0.0,
            removes: stage == 1,
            kind: sf_interaction::DAMAGE_IMPACT,
        })
    };
    // Two stages crossed by one hit: one noise.
    app.world_mut().write_message(destroyed(0));
    app.world_mut().write_message(destroyed(1));
    let heard = tick(&mut app);
    assert_eq!(heard.len(), 1, "{heard:?}");
    let n = &heard[0];
    assert_eq!(n.at, object.translation);
    assert_eq!(n.kind, NOISE_BREAK);
    assert_eq!(n.case, NoiseCase::Break);
    assert_eq!(n.maker, Some(player()));
    assert!(!n.reveals_maker);
    assert_eq!(n.loudness_m.to_bits(), (6.0 * p.noise_speed_mul_max).to_bits());

    // An unregistered placement's break makes none.
    let mut other = destroyed(0);
    other.0.placement = PlacementId(99);
    app.world_mut().write_message(other);
    assert!(tick(&mut app).is_empty());
}

#[cfg(feature = "physics")]
mod physics {
    use bevy_app::First;
    use sf_coords::{WorldPos, WorldTransform};
    use sf_physics_bevy::sf_physics::{BodyDesc, BodyId, BodyKind, Layer, PhysicsEvent, ShapeRef, SurfaceId};
    use sf_physics_bevy::{CarryFeedback, CarryOutcome, Physics, PhysicsBody, PhysicsEvents, PhysicsStreaming};
    use sf_script_bevy::PLAYER;
    use sf_world::WorldspaceId;

    use super::*;

    fn ws() -> WorldspaceId {
        WorldspaceId::new("test")
    }

    /// [`base`] with an active physics world and the wood jar [`JAR`] as a dynamic body, mapped.
    fn physics_app() -> (App, BodyId) {
        let mut app = base();
        app.init_resource::<Physics>().init_resource::<PhysicsEvents>().init_resource::<PhysicsStreaming>();
        insert(&mut app, JAR, jar(Some(WOOD), 1000.0), Frame::default());
        let body = dynamic(&mut app, Layer::DEBRIS);
        app.world_mut().resource_mut::<PhysicsStreaming>().insert_placed(&ws(), JAR, body);
        (app, body)
    }

    fn dynamic(app: &mut App, layer: Layer) -> BodyId {
        let pose = WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(0.0, 1.0, 0.0)), ..WorldTransform::IDENTITY };
        let desc = BodyDesc {
            mass: Some(1.0),
            ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Box(Vec3::splat(0.1)), layer)
        };
        let mut physics = app.world_mut().resource_mut::<Physics>();
        physics.worlds.set_active(Some(ws()));
        physics.insert(&ws(), &desc)
    }

    /// A fixed floor body (no placement).
    fn floor(app: &mut App) -> BodyId {
        let pose = WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(0.0, -0.5, 0.0)), ..WorldTransform::IDENTITY };
        let desc = BodyDesc::fixed(pose, ShapeRef::Box(Vec3::new(5.0, 0.5, 5.0)), Layer::STATIC);
        app.world_mut().resource_mut::<Physics>().insert(&ws(), &desc)
    }

    fn contact(a: BodyId, b: BodyId, speed: f32) -> PhysicsEvent {
        PhysicsEvent::Contact {
            a,
            b,
            impulse: 1.0,
            speed,
            slide: 0.0,
            point: WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.1, 0.0)),
            normal: Vec3::Y,
            surfaces: [SurfaceId::DEFAULT; 2],
        }
    }

    /// One frame (events cleared, cursors reset) whose one fixed tick reads `events`.
    fn frame(app: &mut App, events: Vec<PhysicsEvent>) -> Vec<InteractionNoise> {
        app.world_mut().resource_mut::<PhysicsEvents>().0 = events;
        app.world_mut().run_schedule(First);
        tick(app)
    }

    #[test]
    fn one_rattle_within_the_merge_window_is_one_noise_and_a_louder_hit_is_heard() {
        let (mut app, jar) = physics_app();
        let ground = floor(&mut app);
        let p = app.world().resource::<InteractionSettings>().0;
        let window = p.ticks(p.noise_merge_s);
        assert_eq!(window, 15, "0.25 s at 60 Hz");

        // A landing and its bounce in one step, then decaying bounces: one noise, the loudest.
        let heard = frame(&mut app, vec![contact(jar, ground, 5.0), contact(ground, jar, 3.0)]);
        assert_eq!(heard.len(), 1, "{heard:?}");
        let n = &heard[0];
        assert_eq!(n.case, NoiseCase::Impact);
        assert_eq!(n.kind, TagId::of("impact.test"));
        assert_eq!(n.at, Vec3::new(0.0, 0.1, 0.0));
        assert_eq!(n.maker, None);
        assert!(!n.reveals_maker);
        assert_eq!(n.loudness_m.to_bits(), sf_interaction::loudness_m(15.0, 5.0, &p).unwrap().to_bits());
        for speed in [2.0, 1.5] {
            assert!(frame(&mut app, vec![contact(jar, ground, speed)]).is_empty(), "a quieter bounce at {speed} m/s");
        }
        // Still inside the window: a louder hit is heard.
        let louder = frame(&mut app, vec![contact(jar, ground, 8.0)]);
        assert_eq!(louder.len(), 1);
        assert!(louder[0].loudness_m > n.loudness_m);
        // Under the noise floor: nothing at any time.
        assert!(frame(&mut app, vec![contact(jar, ground, 0.5)]).is_empty());

        // The window opened at the landing, 4 ticks ago; past its 15 ticks the same tap is heard again.
        for _ in 0..window {
            frame(&mut app, Vec::new());
        }
        assert_eq!(frame(&mut app, vec![contact(jar, ground, 2.0)]).len(), 1);
    }

    #[test]
    fn a_thrown_landing_names_the_thrower_and_debris_and_held_bodies_are_silent() {
        let (mut app, jar) = physics_app();
        let ground = floor(&mut app);
        let holder = app
            .world_mut()
            .spawn(InteractionUser { user: player(), actor: PLAYER, priority: 30, tags: SmallVec::new() })
            .id();
        app.world_mut().write_message(CarryFeedback { holder, outcome: CarryOutcome::Thrown { body: jar, speed: 8.0 } });
        let heard = frame(&mut app, vec![contact(ground, jar, 6.0)]);
        assert_eq!(heard.len(), 1, "{heard:?}");
        assert_eq!(heard[0].case, NoiseCase::ThrownLanding);
        assert_eq!(heard[0].maker, Some(player()));
        assert!(!heard[0].reveals_maker);

        // A debris-shell piece, a held body and a body of no indexed placement: silent.
        let (mut app, jar) = physics_app();
        let ground = floor(&mut app);
        let piece = dynamic(&mut app, Layer::DEBRIS);
        let shard = DebrisPiece { set: RecordId::new("debris.shards"), piece: 0, placement: JAR, order: 0, age_s: 0.0 };
        app.world_mut().spawn((shard, PhysicsBody { worldspace: ws(), id: piece }));
        let held = dynamic(&mut app, Layer::HELD);
        let stray = dynamic(&mut app, Layer::PROP);
        let heard = frame(
            &mut app,
            vec![contact(jar, piece, 6.0), contact(held, jar, 6.0), contact(stray, ground, 6.0)],
        );
        assert!(heard.is_empty(), "{heard:?}");
    }
}
