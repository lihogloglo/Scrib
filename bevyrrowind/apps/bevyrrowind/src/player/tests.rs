//! A2 verification (action.md build plan): the player's bindings and rules on `level-flat-slopes`'
//! colliders, headless at 30/60/144 fps (manual frame times): walking draws even frame steps at
//! the same speed at every rate, and a jump has the same apex and lands.

use std::sync::Arc;
use std::time::Duration;

use bevy::time::TimeUpdateStrategy;
use sf_controller::{CollisionMesh, GroundHit, GroundProbe, ProbeQuery, StaticShape, StaticWorld, SurfaceId};
use sf_coords::WorldTransform;
use sf_level::Level;

use super::*;

const LEVEL: &str = include_str!("../../../../levels/flat-slopes.ron");

#[derive(Resource)]
struct Statics(StaticWorld);

impl GroundProbe for Statics {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

fn pos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(v.as_dvec3())
}

fn statics(level: &Level) -> Statics {
    let mut world = StaticWorld::new(WorldPos::ORIGIN);
    for o in level.objects.iter().filter(|o| o.collide && o.dynamic.is_none()) {
        let data = o.mesh();
        let mesh = CollisionMesh {
            positions: data.positions.clone(),
            triangles: data.triangles(),
            surfaces: vec![SurfaceId::DEFAULT; data.indices.len() / 3],
        };
        let shape = Arc::new(StaticShape::new(&mesh).expect("non-empty mesh"));
        world.insert(&shape, WorldTransform { pos: pos(o.at), rotation: o.rotation(), scale: Vec3::ONE });
    }
    world.rebuild();
    Statics(world)
}

/// The game's player (bindings, rules) at the level's `start` spawn, no camera (the stick is
/// world XZ).
fn app(fps: u32) -> (App, Entity) {
    app_with(fps, &crate::controls::keymap())
}

/// [`app`] with the bindings of `keymap`.
fn app_with(fps: u32, keymap: &sf_ui::Keymap) -> (App, Entity) {
    let level = Level::parse(LEVEL).expect("flat-slopes parses");
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), bevy::time::TimePlugin, bevy::input::InputPlugin, EnhancedInputPlugin))
        .add_plugins((SfActionPlugin::<Statics>::default(), ActionInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / f64::from(fps))))
        .insert_resource(statics(&level))
        .insert_resource(rules());
    // `EnhancedInputPlugin` creates its context registry in `finish` (`App::run` calls it).
    app.finish();
    app.cleanup();
    let feet = level.spawn("start").expect("start spawn").at;
    let character = ActionCharacter::new(PLAYER_CHARACTER, ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(pos(feet), 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let player = app
        .world_mut()
        .spawn((character, body, PlayerControls, PlayerInput::new(PlayerId::new(0).unwrap()), Transform::from_translation(feet)))
        .id();
    spawn_controls(&mut app.world_mut().commands(), keymap, player);
    app.world_mut().flush();
    (app, player)
}

fn action(app: &mut App, tag: PlayerAction) -> Entity {
    let mut q = app.world_mut().query::<(Entity, &PlayerAction)>();
    q.iter(app.world()).find(|(_, t)| **t == tag).expect("player action").0
}

fn mock<T: Into<ActionValue>>(app: &mut App, e: Entity, value: T) {
    app.world_mut().entity_mut(e).insert(ActionMock::new(TriggerState::Fired, value, MockSpan::Manual));
}

fn unmock(app: &mut App, e: Entity) {
    app.world_mut().entity_mut(e).remove::<ActionMock>();
}

/// The drawn root.
fn root(app: &App, player: Entity) -> Vec3 {
    let body = app.world().get::<ActionBody>(player).unwrap();
    let t = app.world().get::<Transform>(player).unwrap();
    WorldPos::ORIGIN.delta_f32(body.frame_origin.offset(t.translation))
}

fn frames(fps: u32, secs: f32) -> usize {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "a few hundred frames")]
    let n = (secs * fps as f32).round() as usize;
    n
}

struct Run {
    /// Mean drawn speed over the steady part of the walk, m/s.
    speed: f32,
    /// Coefficient of variation of the per-frame drawn speed.
    cv: f32,
    /// Slowest frame step / mean (monotonic: > 0).
    min_ratio: f32,
    apex: f32,
    landed: f32,
    /// Frames where the height reversed direction during the jump (smooth: exactly one).
    reversals: usize,
}

fn run(fps: u32) -> Run {
    let (mut app, player) = app(fps);
    #[allow(clippy::cast_precision_loss, reason = "small")]
    let dt = 1.0 / fps as f32;
    for _ in 0..frames(fps, 0.5) {
        app.update();
    }
    let ground = root(&app, player).y;
    let stick = action(&mut app, PlayerAction::Stick);
    mock(&mut app, stick, Vec2::Y);
    // Accelerate, then measure 1.5 s of steady walking.
    for _ in 0..frames(fps, 1.0) {
        app.update();
    }
    let mut speeds = Vec::new();
    let mut last = root(&app, player);
    for _ in 0..frames(fps, 1.5) {
        app.update();
        let now = root(&app, player);
        speeds.push(Vec2::new(now.x - last.x, now.z - last.z).length() / dt);
        last = now;
    }
    unmock(&mut app, stick);
    for _ in 0..frames(fps, 1.0) {
        app.update();
    }
    let jump = action(&mut app, PlayerAction::Button(ActionClass::Jump));
    mock(&mut app, jump, true);
    let (mut apex, mut prev, mut rising, mut reversals) = (f32::NEG_INFINITY, root(&app, player).y, true, 0);
    for i in 0..frames(fps, 2.0) {
        if i == frames(fps, 0.1) {
            unmock(&mut app, jump);
        }
        app.update();
        let y = root(&app, player).y;
        apex = apex.max(y - ground);
        if (y > prev + 1e-5) != rising && (y - prev).abs() > 1e-5 {
            rising = !rising;
            reversals += 1;
        }
        prev = y;
    }
    #[allow(clippy::cast_precision_loss, reason = "small")]
    let n = speeds.len() as f32;
    let mean = speeds.iter().sum::<f32>() / n;
    let var = speeds.iter().map(|s| (s - mean).powi(2)).sum::<f32>() / n;
    let min = speeds.iter().copied().fold(f32::INFINITY, f32::min);
    Run { speed: mean, cv: var.sqrt() / mean, min_ratio: min / mean, apex, landed: root(&app, player).y - ground, reversals }
}

#[test]
fn walk_and_jump_match_at_30_60_144_fps() {
    let runs: Vec<(u32, Run)> = [30, 60, 144].into_iter().map(|fps| (fps, run(fps))).collect();
    let (_, base) = &runs[1];
    for (fps, r) in &runs {
        eprintln!(
            "{fps} fps: speed {:.3} m/s, cv {:.4}, min step {:.3}, apex {:.3} m, landed {:.3} m, reversals {}",
            r.speed, r.cv, r.min_ratio, r.apex, r.landed, r.reversals
        );
        assert!(r.speed > 1.0, "{fps} fps: walks ({:.2} m/s)", r.speed);
        assert!(r.cv < 0.02, "{fps} fps: even frame steps (cv {:.4})", r.cv);
        assert!(r.min_ratio > 0.9, "{fps} fps: no stalled frame ({:.3})", r.min_ratio);
        assert!((r.speed - base.speed).abs() < 0.02 * base.speed, "{fps} fps: speed {:.3} vs {:.3} at 60", r.speed, base.speed);
        assert!(r.apex > 0.3, "{fps} fps: jumps ({:.3} m)", r.apex);
        assert!((r.apex - base.apex).abs() < 0.02, "{fps} fps: apex {:.3} vs {:.3} at 60", r.apex, base.apex);
        assert!(r.landed.abs() < 0.05, "{fps} fps: lands ({:.3} m)", r.landed);
        assert_eq!(r.reversals, 1, "{fps} fps: one smooth arc");
    }
}

/// A physics world of fixed boxes (center, half extents, layer) and their bodies.
fn boxes(list: &[(Vec3, Vec3, Layer)]) -> (Physics, Vec<BodyId>) {
    let ws = WorldspaceId::new("test");
    let mut physics = Physics::default();
    let ids = list
        .iter()
        .map(|(c, h, layer)| physics.insert(&ws, &sf_physics::BodyDesc::fixed(WorldTransform::from_pos(pos(*c)), sf_physics::ShapeRef::Box(*h), *layer)))
        .collect();
    physics.world_mut(&ws).expect("world").refresh_queries();
    (physics, ids)
}

#[test]
fn focus_ray_takes_the_nearest_hit_and_an_actor_in_front_wins() {
    let ws = WorldspaceId::new("test");
    let small = Vec3::splat(0.1);
    let (physics, ids) = boxes(&[(Vec3::new(0.0, 0.0, -1.5), small, Layer::PROP), (Vec3::new(0.0, 0.0, -2.5), small, Layer::PROP)]);
    let world = physics.world(&ws);
    let owner = |b: BodyId| ids.iter().position(|i| *i == b).map(|i| ScriptHandle(i as u128 + 1));
    let ray = |actors: &[ActorShape]| focus_ray(world, WorldPos::ORIGIN, Vec3::NEG_Z, 5.0, owner, actors);
    let (h, t) = ray(&[]).expect("hits the boxes");
    assert_eq!(h, ScriptHandle(1), "the nearer of two items");
    assert!((t - 1.4).abs() < 1e-3, "at its face ({t})");
    // An NPC (feet 1 m down, 1.8 m tall) standing in front of the item.
    let npc = ActorShape { handle: ScriptHandle(9), feet: pos(Vec3::new(0.0, -1.0, -1.0)), radius: 0.3, height: 1.8 };
    let (h, t) = ray(&[npc]).expect("hits the NPC");
    assert_eq!(h, ScriptHandle(9), "the NPC in front wins");
    assert!((t - 0.7).abs() < 1e-3, "at its chest ({t})");
    // Behind the item, the NPC loses; off the ray, it is missed.
    let behind = ActorShape { feet: pos(Vec3::new(0.0, -1.0, -2.0)), ..npc };
    assert_eq!(ray(&[behind]).map(|(h, _)| h), Some(ScriptHandle(1)));
    let aside = ActorShape { feet: pos(Vec3::new(1.0, -1.0, -1.0)), ..npc };
    assert_eq!(ray(&[aside]).map(|(h, _)| h), Some(ScriptHandle(1)));
    // A wall (a body with no placement) in front blocks everything behind it.
    let (physics, _) = boxes(&[(Vec3::new(0.0, 0.0, -0.5), small, Layer::STATIC), (Vec3::new(0.0, 0.0, -1.5), small, Layer::PROP)]);
    assert_eq!(focus_ray(physics.world(&ws), WorldPos::ORIGIN, Vec3::NEG_Z, 5.0, |_| None, &[npc]), None);
}

#[test]
fn interior_floor_is_under_the_roof() {
    let ws = WorldspaceId::new("test");
    let slab = Vec3::new(5.0, 0.15, 5.0);
    let (physics, _) =
        boxes(&[(Vec3::new(0.0, -0.15, 0.0), slab, Layer::STATIC), (Vec3::new(0.0, 3.15, 0.0), slab, Layer::STATIC)]);
    let world = physics.world(&ws).expect("world");
    let height = MoverParams::default().height;
    for eye in [10.0, 1.5] {
        let floor = floor_below(world, pos(Vec3::new(0.5, eye, 0.5)), height).expect("finds the floor");
        assert!(WorldPos::ORIGIN.delta_f32(floor).y.abs() < 1e-3, "eye {eye}: floor at 0, got {:?}", floor.to_f64());
    }
    // No ceiling: not an interior floor.
    let (open, _) = boxes(&[(Vec3::new(0.0, -0.15, 0.0), slab, Layer::STATIC)]);
    assert_eq!(floor_below(open.world(&ws).expect("world"), pos(Vec3::new(0.0, 10.0, 0.0)), height), None);
    // An arrival marker wins over the floor and gives the heading.
    let marker = WorldTransform::from_pos(pos(Vec3::new(2.0, 1.2, 0.0)));
    let (at, yaw) = interior_spawn(&[marker], Some(world), pos(Vec3::new(0.0, 10.0, 0.0))).expect("spawn");
    assert_eq!(at, marker.pos);
    assert!(yaw.is_some());
}

// ---- keymap (ROADMAP 2.1b, controls.rs) -----------------------------------------------------

/// A keymap with a profile's `[bindings.gameplay]` entries applied (what a restart loads).
fn keymap_with(gameplay: &str) -> sf_ui::Keymap {
    let mut keymap = crate::controls::keymap();
    let profile = sf_mods::Profile::from_toml("test", &format!("[bindings.gameplay]\n{gameplay}\n")).expect("profile parses");
    assert!(keymap.load_from(&profile).is_empty());
    keymap
}

fn hold_key(app: &mut App, key: KeyCode, down: bool) {
    let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    if down { keys.press(key) } else { keys.release(key) }
}

/// Highest rise above the start over `secs`, holding `key` for the first 0.1 s.
fn rise_on(app: &mut App, player: Entity, key: KeyCode) -> f32 {
    for _ in 0..frames(60, 0.5) {
        app.update();
    }
    let ground = root(app, player).y;
    let mut apex = 0.0_f32;
    for i in 0..frames(60, 1.0) {
        hold_key(app, key, i < frames(60, 0.1));
        app.update();
        apex = apex.max(root(app, player).y - ground);
    }
    apex
}

#[test]
fn rebound_jump_in_the_keymap_drives_the_jump() {
    let keymap = keymap_with(r#"jump = { kbm = ["key:KeyJ"] }"#);
    assert_eq!(keymap.bound(&crate::controls::Control::Jump.id(), sf_ui::Device::Kbm), ["key:KeyJ"]);
    let (mut app, player) = app_with(60, &keymap);
    assert!(rise_on(&mut app, player, KeyCode::Space) < 0.05, "Space no longer jumps");
    let apex = rise_on(&mut app, player, KeyCode::KeyJ);
    assert!(apex > 0.5, "J jumps: apex {apex}");
}

fn mode(app: &App, player: Entity) -> sf_action::Mode {
    app.world().get::<ActionCharacter>(player).unwrap().state.mode.mode
}

/// Sneak's mode after each step of `presses` (key down for 3 frames, then up for 3).
fn sneak_modes(keymap: &sf_ui::Keymap, presses: usize) -> Vec<(sf_action::Mode, sf_action::Mode)> {
    let (mut app, player) = app_with(60, keymap);
    for _ in 0..10 {
        app.update();
    }
    (0..presses)
        .map(|_| {
            hold_key(&mut app, KeyCode::ControlLeft, true);
            for _ in 0..3 {
                app.update();
            }
            let down = mode(&app, player);
            hold_key(&mut app, KeyCode::ControlLeft, false);
            for _ in 0..3 {
                app.update();
            }
            (down, mode(&app, player))
        })
        .collect()
}

#[test]
fn sneak_holds_by_default_and_toggles_when_set() {
    use sf_action::Mode::{Normal, Sneak};
    assert_eq!(sneak_modes(&crate::controls::keymap(), 1), [(Sneak, Normal)], "hold: sneak while Ctrl is down");
    let toggle = keymap_with(r#"sneak = { press = "toggle" }"#);
    assert_eq!(toggle.press_mode(&crate::controls::Control::Sneak.id()), Some(sf_ui::PressMode::Toggle));
    assert_eq!(sneak_modes(&toggle, 2), [(Sneak, Sneak), (Normal, Normal)], "toggle: a press on, the next off");
}

#[test]
fn rebinds_and_press_modes_round_trip_through_the_profile_file() {
    use crate::controls::Control;
    let mut keymap = crate::controls::keymap();
    let jump = sf_ui::SlotRef { action: Control::Jump.id(), device: sf_ui::Device::Kbm, slot: 0 };
    let proposal = keymap.propose(&jump, "key:KeyJ").unwrap();
    keymap.apply(&proposal, sf_ui::Resolution::Swap).unwrap();
    assert_eq!(keymap.set_press_mode(&Control::Sneak.id(), sf_ui::PressMode::Toggle), Ok(true));
    assert!(keymap.set_press_mode(&Control::Jump.id(), sf_ui::PressMode::Toggle).is_err(), "jump offers no choice");
    let mut profile = sf_mods::Profile { name: "keymap-test".into(), ..Default::default() };
    assert!(keymap.save_to(&mut profile));
    let dir = std::env::temp_dir().join(format!("bw-keymap-{}", std::process::id()));
    profile.save(&dir).unwrap();
    // A restart: a fresh keymap from the file.
    let mut restarted = crate::controls::keymap();
    assert!(restarted.load_from(&crate::controls::reload(&dir, "keymap-test")).is_empty());
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(restarted.bound(&Control::Jump.id(), sf_ui::Device::Kbm), ["key:KeyJ"]);
    assert_eq!(restarted.bound(&Control::Jump.id(), sf_ui::Device::Gamepad), ["pad:South"], "pad untouched");
    assert_eq!(restarted.press_mode(&Control::Sneak.id()), Some(sf_ui::PressMode::Toggle));
    assert_eq!(restarted.press_mode(&Control::Sprint.id()), Some(sf_ui::PressMode::Hold));
    assert!(restarted.conflicts().is_empty(), "defaults conflict: {:?}", restarted.conflicts());
}

/// S4-a (physics-fix-plan.md slice 2): a body restored 1.5 m above a floor whose cell activates
/// frames later holds still while gated (a witness without the gate falls through the missing
/// floor), then lands on it. Sanity only: no NaN, not below the floor top.
#[test]
fn arrival_waits_for_the_floor_cell_then_lands() {
    use sf_world::{
        Anchor, AnchorId, AnchorRole, IndexParams, Location, ModelInfo, Placement, PlacementClass, PlacementFlags, RecordRef, Scheduler,
        Stage, StageStep, TickInput, build_directory, build_pages,
    };
    use sf_world_bevy::{Streaming, StreamingConfig};

    let ws = WorldspaceId::new("test");
    let level = Level::parse(LEVEL).expect("flat-slopes parses");
    let floor_y = 0.0;
    let (mut app, player) = app(60);
    let above = pos(level.spawn("start").unwrap().at + Vec3::Y * 1.5);
    // The floor's cell: a scheduler that knows it, not yet activated; no collision yet.
    let floor = Placement {
        id: PlacementId(1),
        base: RecordRef::new("TEST", "floor"),
        model: Some(sf_vfs::VfsPath::new("floor.nif").unwrap()),
        transform: WorldTransform { pos: above, ..WorldTransform::IDENTITY },
        class: PlacementClass::Static,
        flags: PlacementFlags::empty(),
    };
    let params = IndexParams::default();
    let models = |_: &sf_vfs::VfsPath| Some(ModelInfo { min: Vec3::splat(-0.5), max: Vec3::splat(0.5), bytes: 1000, lod_distance: 100.0 });
    let (pages, _) = build_pages(&params, [floor], &models);
    let mut streaming = Streaming::new(StreamingConfig::default());
    let mut scheduler = Scheduler::new(StreamingConfig::default().scheduler);
    assert!(scheduler.add_worldspace(build_directory(ws.clone(), &params, &pages), None));
    *streaming.scheduler_mut() = scheduler;
    app.insert_resource(Statics(StaticWorld::new(WorldPos::ORIGIN)))
        .insert_resource(streaming)
        .insert_resource(PhysicsStreaming::default())
        .insert_resource(ActiveWorldspace(Some(ws.clone())))
        .init_resource::<FadeTransition>()
        .insert_resource(PlayerMode { on: true, paused: false, body: Some(player), last_ground: None })
        .init_resource::<ArrivalSpot>()
        .add_systems(Update, await_ground::<Statics>);
    let witness = app.world_mut().spawn(ActionBody::new(Locomotion::new(above, 0.0), MoverParams::default(), WorldPos::ORIGIN)).id();
    let character = ActionCharacter::new(CharacterId(2), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    app.world_mut().entity_mut(witness).insert(character);
    app.world_mut().get_mut::<ActionBody>(player).unwrap().teleport(above, 0.0);
    app.world_mut().entity_mut(player).insert((PlayerBody, AwaitingGround));
    let y = |app: &App, e: Entity| WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(e).unwrap().loco.position).y;

    for _ in 0..60 {
        app.update();
        assert!((y(&app, player) - 1.5).abs() < 1e-4, "the gated body moved: {}", y(&app, player));
    }
    assert!(app.world().get::<AwaitingGround>(player).is_some(), "released before the floor's cell activated");
    assert!(y(&app, witness) < floor_y - 0.5, "witness: an ungated body falls through the missing floor: {}", y(&app, witness));

    // The floor's cell activates and its collision is in.
    let mut now = 0.0;
    let mut frames = 0;
    loop {
        let mut streaming = app.world_mut().resource_mut::<Streaming>();
        let s = streaming.scheduler_mut();
        let cell = s.directory(&ws).unwrap().partition.cell_of(above);
        if s.level(&ws, cell) >= sf_world::Level::Active {
            break;
        }
        let anchors: Vec<Anchor> =
            [AnchorRole::Gameplay, AnchorRole::Visual].into_iter().zip(0..).map(|(role, i)| Anchor::new(AnchorId(i), role, Location::new(ws.clone(), above))).collect();
        let report = s.update(&TickInput { now, anchors: &anchors });
        for id in report.start {
            match s.request(id).map(|r| r.stage) {
                Some(Stage::Fetch) => s.fetched(id, now, 1, 1),
                Some(Stage::Assets) => s.assets_loaded(id, now),
                _ => false,
            };
        }
        for stage in [Stage::Deactivate, Stage::Despawn, Stage::Spawn, Stage::Upload, Stage::Activate] {
            s.run_stage(stage, &mut |_| StageStep { cost: 0.01, done: true });
        }
        now += 1.0 / 60.0;
        frames += 1;
        assert!(frames < 2000, "the floor's cell never activated");
    }
    app.insert_resource(statics(&level));
    for _ in 0..120 {
        app.update();
    }
    assert!(app.world().get::<AwaitingGround>(player).is_none(), "the body was never released");
    let landed = y(&app, player);
    assert!(landed.is_finite() && landed > floor_y - 0.05 && landed < 0.5, "the released body lands on the floor: {landed}");
}

/// The arrival gate of [`app`]'s player on `flat-slopes` (`Statics` as its ground), with no
/// collision streaming (the release waits only for the fade) and `physics` for the fall guards.
fn gated(physics: Physics) -> (App, Entity) {
    let (mut app, player) = app(60);
    app.insert_resource(physics)
        .insert_resource(ActiveWorldspace(Some(WorldspaceId::new("test"))))
        .init_resource::<FadeTransition>()
        .insert_resource(PlayerMode { on: true, paused: false, body: Some(player), last_ground: None })
        .init_resource::<ArrivalSpot>()
        .add_systems(Update, (guard_fall, await_ground::<Statics>).chain());
    app.world_mut().entity_mut(player).insert(PlayerBody);
    (app, player)
}

fn feet_y(app: &App, player: Entity) -> f32 {
    WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(player).unwrap().loco.position).y
}

/// S4-b (physics-fix-plan.md slice 3): a body released above the floor, or sunk into it, is
/// put on its top (two-stage ground find) and seeds `last_ground`. Sanity only.
#[test]
fn release_places_the_feet_on_the_floor_and_seeds_last_ground() {
    let start = pos(Level::parse(LEVEL).unwrap().spawn("start").unwrap().at);
    for dy in [0.4, -0.3] {
        let (mut app, player) = gated(Physics::default());
        app.world_mut().get_mut::<ActionBody>(player).unwrap().teleport(start.offset(Vec3::Y * dy), 0.0);
        app.world_mut().entity_mut(player).insert(AwaitingGround);
        app.update();
        assert!(app.world().get::<AwaitingGround>(player).is_none(), "{dy}: released");
        let y = feet_y(&app, player);
        assert!(y.is_finite() && (0.0..0.06).contains(&y), "{dy}: feet on the floor top: {y}");
        let ground = app.world().resource::<PlayerMode>().last_ground.expect("last_ground seeded");
        assert!(WorldPos::ORIGIN.delta_f32(ground).y.abs() < 0.06, "{dy}: last_ground on the floor");
        assert!(app.world().resource::<ArrivalSpot>().0.is_some(), "{dy}: arrival spot refined");
    }
}

/// S4-b: in an interior, a body over a hole falls; past the cell's lowest fixed body (the
/// floor's bottom, from the physics world) it goes back to the arrival spot and lands there.
/// Sanity only: returned, no NaN, on the floor.
#[test]
fn fall_into_the_interior_void_returns_to_the_arrival_spot() {
    let start = pos(Level::parse(LEVEL).unwrap().spawn("start").unwrap().at);
    // The 80 m floor's bounds: the lowest fixed body is its bottom, y -0.5.
    let (physics, _) = boxes(&[(Vec3::new(0.0, -0.25, 0.0), Vec3::new(40.0, 0.25, 40.0), Layer::STATIC)]);
    let (mut app, player) = gated(physics);
    app.world_mut().resource_mut::<ArrivalSpot>().0 = Some((start, 0.0));
    // Off the floor's edge: nothing below.
    app.world_mut().get_mut::<ActionBody>(player).unwrap().teleport(pos(Vec3::new(60.0, 0.5, 0.0)), 0.0);
    let mut lowest = f32::MAX;
    let mut returned = false;
    for _ in 0..frames(60, 4.0) {
        app.update();
        let at = WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(player).unwrap().loco.position);
        assert!(at.is_finite(), "NaN: {at}");
        lowest = lowest.min(at.y);
        returned |= at.x.abs() < 1.0;
    }
    assert!(lowest < -0.5, "the body over the hole fell: {lowest}");
    assert!(returned, "returned to the arrival spot");
    let at = WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(player).unwrap().loco.position);
    assert!(start.delta_f32(pos(at)).xz().length() < 0.5 && (-0.05..0.5).contains(&at.y), "lands at the arrival spot: {at}");
}
