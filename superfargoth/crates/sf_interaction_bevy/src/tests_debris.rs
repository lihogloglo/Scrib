//! `IN1c` headless tests (`physics` feature): break debris on the fixed tick. The same break spawns
//! the same pieces with the same launch on two clients; the cap holds, the oldest fading first;
//! beyond `interact.debris.lod_m` (or without a focus) no bodies; every piece is gone after
//! `fade_s + shrink_s` with its body; sanity: no NaN, nothing through the floor.

use glam::Vec3;
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::PlayerKey;
use sf_interaction::{BreakAction, DestructibleDef, Stage, TagId, UserKey};
use sf_physics_bevy::sf_physics::{BodyDesc, Layer, ShapeRef};
use sf_physics_bevy::{Physics, PhysicsBody, PhysicsFocus, PhysicsSystems};
use sf_records::RecordId;
use sf_tunables_bevy::TunablesPlugin;
use sf_world::WorldspaceId;
use smallvec::{SmallVec, smallvec};

use super::*;

const SHARDS: &str = "debris.shards";
const PIECES: usize = 4;

fn step_world(mut physics: ResMut<Physics>) {
    physics.worlds.step();
}

/// A client with one active world (a floor whose top is at y = 0), its focus at `focus`.
fn app(focus: Option<WorldPos>) -> App {
    let mut app = App::new();
    app.add_plugins((TunablesPlugin, InteractionPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PhysicsFocus(focus))
        .init_resource::<Physics>()
        .add_systems(FixedUpdate, step_world.in_set(PhysicsSystems::Step));
    // Binds the tunables (PreUpdate); no `TimePlugin`, so the fixed loop runs only when a test asks.
    app.update();
    let ws = WorldspaceId::new("test");
    let mut physics = app.world_mut().resource_mut::<Physics>();
    let floor = WorldTransform { pos: WorldPos::ORIGIN.offset(Vec3::new(0.0, -0.5, 0.0)), ..WorldTransform::IDENTITY };
    physics.insert(&ws, &BodyDesc::fixed(floor, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC));
    physics.worlds.set_active(Some(ws));
    let pieces: Vec<DebrisPieceDef> = (0..PIECES)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let x = i as f32 * 0.12 - 0.18;
            DebrisPieceDef {
                shape: ShapeRef::Box(Vec3::splat(0.04)),
                local: Frame { translation: Vec3::new(x, 0.3, 0.0), ..Frame::default() },
                mass: 0.2,
            }
        })
        .collect();
    app.world_mut().resource_mut::<DebrisSets>().insert(RecordId::new(SHARDS), pieces);
    app
}

fn fixed(app: &mut App, runs: usize) {
    for _ in 0..runs {
        app.world_mut().run_schedule(FixedUpdate);
    }
}

/// A pot at `x` that breaks into one shard set at zero health.
fn insert_pot(app: &mut App, placement: PlacementId, x: f32) {
    let stage = Stage {
        at: 0.0,
        model: None,
        debris: smallvec![RecordId::new(SHARDS)],
        chunks: None,
        effect: None,
        on_break: BreakAction::None,
    };
    let def = Arc::new(InteractableDef {
        id: RecordId::new("pot"),
        slots: Vec::new(),
        verbs: SmallVec::new(),
        material: Some(TagId::of("clay")),
        destructible: Some(DestructibleDef {
            health: 10.0,
            stages: smallvec![stage],
            ignore_external: false,
            loot: None,
            salvage: smallvec![],
        }),
        carry: None,
        adverts: Vec::new(),
    });
    let object = Frame { translation: Vec3::new(x, 0.0, 0.0), ..Frame::default() };
    app.world_mut().write_message(IndexChange::Insert {
        placement,
        def,
        object,
        owner: Entity::PLACEHOLDER,
        disabled: SmallVec::new(),
    });
    fixed(app, 1);
}

fn smash(app: &mut App, placement: PlacementId, x: f32) {
    app.world_mut().write_message(PropDamage {
        placement,
        amount: 10.0,
        kind: TagId::of("blunt"),
        point: Vec3::new(x, 0.4, 0.2),
        dir: Vec3::NEG_Z,
        culprit: Some(UserKey::Player(PlayerKey(1))),
    });
    fixed(app, 1);
}

/// Live pieces by spawn rank: (set, piece, order, fading, linear velocity, position).
fn pieces(app: &mut App) -> Vec<(RecordId, u16, u64, bool, Vec3, WorldPos)> {
    let mut q = app.world_mut().query::<(&DebrisPiece, &PhysicsBody, Option<&DebrisFading>)>();
    let physics = app.world().resource::<Physics>();
    let mut out: Vec<_> = q
        .iter(app.world())
        .map(|(d, b, f)| {
            let w = physics.world(&b.worldspace).expect("world");
            let v = w.velocity(b.id).expect("body").0;
            let pos = w.pose(b.id).expect("body").pos;
            (d.set.clone(), d.piece, d.order, f.is_some(), v, pos)
        })
        .collect();
    out.sort_by_key(|p| p.2);
    out
}

fn bodies(app: &App) -> usize {
    app.world().resource::<Physics>().worlds.body_count()
}

#[test]
fn the_same_break_spawns_the_same_pieces_and_launch_on_two_clients() {
    let run = || {
        let mut app = app(Some(WorldPos::ORIGIN));
        insert_pot(&mut app, PlacementId(7), 0.0);
        smash(&mut app, PlacementId(7), 0.0);
        pieces(&mut app)
    };
    let (a, b) = (run(), run());
    assert_eq!(a.len(), PIECES);
    let launch = |p: &[(RecordId, u16, u64, bool, Vec3, WorldPos)]| {
        p.iter().map(|x| (x.0.clone(), x.1, x.4.to_array().map(f32::to_bits))).collect::<Vec<_>>()
    };
    assert_eq!(launch(&a), launch(&b), "same seed, same pieces and velocities");
    for (_, _, _, _, v, _) in &a {
        assert!(v.is_finite() && v.length() > 0.1, "launched: {v}");
    }
    assert!(a.windows(2).all(|w| w[0].4 != w[1].4), "each piece has its own jitter");
}

#[test]
fn the_cap_holds_and_the_oldest_fade_first() {
    let mut app = app(Some(WorldPos::ORIGIN));
    app.world_mut().resource_mut::<InteractionSettings>().0.debris_awake_max = 6;
    insert_pot(&mut app, PlacementId(1), -1.0);
    insert_pot(&mut app, PlacementId(2), 1.0);
    smash(&mut app, PlacementId(1), -1.0);
    smash(&mut app, PlacementId(2), 1.0);
    fixed(&mut app, 1); // the fade inserts land
    let p = pieces(&mut app);
    assert_eq!(p.len(), 2 * PIECES, "fading pieces stay until they shrink");
    let fading: Vec<u64> = p.iter().filter(|x| x.3).map(|x| x.2).collect();
    assert_eq!(fading, vec![0, 1], "the two oldest fade");
    for _ in 0..120 {
        fixed(&mut app, 1);
        let live = pieces(&mut app).iter().filter(|x| !x.3).count();
        assert!(live <= 6, "cap holds: {live}");
    }
}

#[test]
fn beyond_the_lod_or_without_a_focus_no_bodies() {
    for focus in [Some(WorldPos::ORIGIN.offset(Vec3::new(100.0, 0.0, 0.0))), None] {
        let mut app = app(focus);
        insert_pot(&mut app, PlacementId(3), 0.0);
        smash(&mut app, PlacementId(3), 0.0);
        assert!(pieces(&mut app).is_empty(), "{focus:?}");
        assert_eq!(bodies(&app), 1, "the floor only");
    }
}

#[test]
fn every_piece_rests_above_the_floor_and_is_gone_after_fade_and_shrink() {
    let mut app = app(Some(WorldPos::ORIGIN));
    insert_pot(&mut app, PlacementId(4), 0.0);
    smash(&mut app, PlacementId(4), 0.0);
    let s = app.world().resource::<InteractionSettings>().0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let limit = ((s.debris_fade_s + s.debris_shrink_s) * 60.0).ceil() as usize + 3;
    let mut ticks = 0;
    while !pieces(&mut app).is_empty() {
        assert!(ticks < limit, "still alive after {ticks} ticks");
        for (_, _, _, _, v, pos) in pieces(&mut app) {
            let y = pos.to_f64().y;
            assert!(v.is_finite() && y.is_finite(), "no NaN");
            assert!(y > -0.06, "through the floor: {y}");
        }
        fixed(&mut app, 1);
        ticks += 1;
    }
    assert_eq!(bodies(&app), 1, "bodies despawned with the pieces");
}
