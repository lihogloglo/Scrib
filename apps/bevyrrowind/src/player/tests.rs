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
    spawn_controls(&mut app.world_mut().commands(), player);
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
