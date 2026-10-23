//! Headless app tests on the camera test level's statics (`MinimalPlugins`, the action shell's
//! fixed tick and mover, the level's colliders as the ground probe): push-in against a wall and
//! under a low ceiling, recovery, the view toggle, zoom, lock-on framing, input through
//! `bevy_enhanced_input`, frame-rate independence, and a `big_space` grid far from the origin.

use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, FixedUpdate, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};
use bevy_time::TimeUpdateStrategy;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_action::{ActionState, BASE_RULES, CharacterId, CharacterParams, Command, RuleKey, RuleSet, SlotInfo, SlotTable};
use sf_action_bevy::{ActionRules, ActionTick, PlayerId, SfActionPlugin};
use sf_controller::{CollisionMesh, Locomotion, MoverParams, StaticShape, StaticWorld, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};
use sf_level::Level;

use crate::*;

const LEVEL: &str = include_str!("../../sf_level/tests/levels/camera.ron");

/// The level's collide objects as the ground probe (the same triangles as the render meshes).
#[derive(Resource)]
struct Statics(StaticWorld);

impl GroundProbe for Statics {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

fn level() -> Level {
    Level::parse(LEVEL).expect("camera level parses")
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

fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5));
    ActionRules { rules: RuleSet::from_ron(BASE_RULES).expect("base rules"), slots: Box::new(slots) }
}

fn app(fps: u32) -> App {
    let mut app = App::new();
    app.add_plugins((bevy_app::TaskPoolPlugin::default(), bevy_time::TimePlugin))
        .add_plugins((SfActionPlugin::<Statics>::default(), SfCameraPlugin::<Statics>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / f64::from(fps))))
        .insert_resource(statics(&level()))
        .insert_resource(rules());
    app
}

/// A character at a level spawn (spawn yaw: degrees, 0 = -z) and a player camera on it, looking
/// the way the character faces.
fn spawn_at(app: &mut App, spawn: &str) -> (Entity, Entity) {
    let level = level();
    let s = level.spawn(spawn).unwrap_or_else(|| panic!("no spawn {spawn}"));
    let yaw = sf_camera::wrap_angle(s.yaw.to_radians() + PI);
    spawn_body(app, s.at, yaw)
}

fn spawn_body(app: &mut App, feet: Vec3, yaw: f32) -> (Entity, Entity) {
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(pos(feet), yaw), MoverParams::default(), WorldPos::ORIGIN);
    let player = app.world_mut().spawn((character, body, Transform::from_translation(feet))).id();
    let mut cam = PlayerCamera::new(player);
    cam.set_look(Look::new(yaw, 0.0));
    let camera = app.world_mut().spawn((cam, Transform::default())).id();
    (player, camera)
}

fn camera(app: &App, e: Entity) -> &PlayerCamera {
    app.world().get::<PlayerCamera>(e).expect("player camera")
}

fn camera_mut(app: &mut App, e: Entity) -> Mut<'_, PlayerCamera> {
    app.world_mut().get_mut::<PlayerCamera>(e).expect("player camera")
}

fn pose(app: &App, e: Entity) -> CameraPose {
    *camera(app, e).pose().expect("a pose after the first frame")
}

/// The drawn body's root.
fn root(app: &App, player: Entity) -> Vec3 {
    let body = app.world().get::<ActionBody>(player).unwrap();
    let t = app.world().get::<Transform>(player).unwrap();
    WorldPos::ORIGIN.delta_f32(body.frame_origin.offset(t.translation))
}

fn at(p: WorldPos) -> Vec3 {
    WorldPos::ORIGIN.delta_f32(p)
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

/// The unobstructed arm at zoom 1 in `Normal` (shoulder offset and distance).
fn full_arm() -> f32 {
    let o = OrbitParams::default();
    o.distance.normal.hypot(o.shoulder_offset)
}

fn eye_radius() -> f32 {
    CollisionParams::default().radius
}

#[test]
fn the_level_parses_and_names_its_spawns() {
    let level = level();
    assert!(level.problems().is_empty(), "{:?}", level.problems());
    for s in ["corridor", "ceiling", "pillar", "open"] {
        assert!(level.spawn(s).is_some(), "spawn {s}");
    }
}

#[test]
fn push_in_against_a_corridor_wall_keeps_the_eye_out_of_it() {
    let mut app = app(60);
    // Facing the east wall: the arm runs west into the west wall (inner face x = -1).
    let (player, cam) = spawn_at(&mut app, "corridor");
    camera_mut(&mut app, cam).set_look(Look::new(FRAC_PI_2, 0.0));
    for frame in 0..60 {
        app.update();
        let p = pose(&app, cam);
        let eye = at(p.eye);
        // Pushed in from the very first frame, and never inside the wall.
        assert!(p.blocked, "frame {frame}: not blocked");
        assert!(eye.x - eye_radius() >= -1.0 - 1e-3, "frame {frame}: eye {eye} inside the west wall");
        assert!(p.arm_length < 1.3, "frame {frame}: arm {}", p.arm_length);
    }
    let p = pose(&app, cam);
    assert_eq!(p.view, ViewChoice::ThirdPerson);
    assert!(p.fade > 0.0, "the body should start fading this close (arm {})", p.arm_length);
    // The camera transform is the pose (frame origin at the world origin here).
    let t = app.world().get::<Transform>(cam).unwrap();
    assert!((t.translation - at(p.eye)).length() < 1e-5);
    // The body stood still.
    assert!(root(&app, player).length() < 0.05, "{}", root(&app, player));
}

#[test]
fn the_eye_stays_under_a_low_ceiling() {
    let mut app = app(60);
    let (_, cam) = spawn_at(&mut app, "ceiling");
    // Looking down 35°: the arm climbs behind, into the 2.1 m ceiling.
    camera_mut(&mut app, cam).set_look(Look::new(PI, -35f32.to_radians()));
    for frame in 0..60 {
        app.update();
        let p = pose(&app, cam);
        let eye = at(p.eye);
        assert!(p.blocked, "frame {frame}: not blocked");
        assert!(eye.y + eye_radius() <= 2.1 + 1e-3, "frame {frame}: eye {eye} in the ceiling");
    }
    assert!(pose(&app, cam).arm_length < full_arm() * 0.5);
}

#[test]
fn recovery_eases_out_slower_than_push_in() {
    let mut app = app(60);
    let (_, cam) = spawn_at(&mut app, "corridor");
    camera_mut(&mut app, cam).set_look(Look::new(FRAC_PI_2, 0.0));
    run(&mut app, 30);
    let pushed = pose(&app, cam).arm_length;
    // Turn to face north along the corridor: the arm runs south, out of the open end.
    camera_mut(&mut app, cam).set_look(Look::new(PI, 0.0));
    let mut lengths = Vec::new();
    for _ in 0..180 {
        app.update();
        lengths.push(pose(&app, cam).arm_length);
    }
    assert!(!pose(&app, cam).blocked);
    // Eased, not snapped: the first frame covers a small part of the way out.
    assert!(lengths[0] < pushed + 0.25 * (full_arm() - pushed), "snapped out: {} -> {}", pushed, lengths[0]);
    assert!(lengths.windows(2).all(|w| w[1] >= w[0] - 1e-5), "not monotonic: {lengths:?}");
    let back = lengths.iter().position(|&l| l > 0.98 * full_arm()).expect("never recovered");
    assert!(back < 120, "recovered only after {back} frames");
    // Turning back into the wall pushes in within the frame.
    camera_mut(&mut app, cam).set_look(Look::new(FRAC_PI_2, 0.0));
    app.update();
    assert!(pose(&app, cam).arm_length < pushed + 0.05, "{} vs {pushed}", pose(&app, cam).arm_length);
}

#[test]
fn a_pillar_behind_the_character_pushes_the_eye_in_front_of_it() {
    let mut app = app(60);
    let (player, cam) = spawn_at(&mut app, "pillar");
    run(&mut app, 30);
    let p = pose(&app, cam);
    let eye = at(p.eye);
    assert!(p.blocked);
    // The eye sphere is clear of the pillar (axis x = 8, z = 0, radius 0.4) and between it and
    // the character.
    let axis = Vec2::new(eye.x - 8.0, eye.z).length();
    assert!(axis >= 0.4 + eye_radius() - 1e-3, "eye {eye} touches the pillar");
    assert!(eye.z < 0.0 && eye.z > root(&app, player).z, "eye {eye}");
}

#[test]
fn the_view_toggle_cuts_between_third_and_first_person() {
    let mut app = app(60);
    let (player, cam) = spawn_at(&mut app, "open");
    run(&mut app, 30);
    let third = pose(&app, cam);
    assert!((third.arm_length - full_arm()).abs() < 1e-3, "{}", third.arm_length);
    assert!(third.fade.abs() < 1e-6, "{}", third.fade);
    let look = camera(&app, cam).look();

    camera_mut(&mut app, cam).toggle_view();
    app.update();
    let first = pose(&app, cam);
    let fp = FirstPersonParams::default();
    let r = root(&app, player);
    let expected = r + Vec3::Y * fp.eye_height + Vec3::new(look.yaw.sin(), 0.0, look.yaw.cos()) * fp.eye_forward;
    assert_eq!(first.view, ViewChoice::FirstPerson);
    assert!((at(first.eye) - expected).length() < 1e-4, "{} vs {expected}", at(first.eye));
    assert!(first.arm_length.abs() < 1e-6 && (first.fade - 1.0).abs() < 1e-6);
    // The aim is untouched by the switch.
    assert_eq!(camera(&app, cam).look(), look);

    camera_mut(&mut app, cam).toggle_view();
    app.update();
    let back = pose(&app, cam);
    assert_eq!(back.view, ViewChoice::ThirdPerson);
    assert!((back.arm_length - full_arm()).abs() < 1e-3, "{}", back.arm_length);
}

#[test]
fn zoom_eases_the_arm_within_its_limits() {
    let mut app = app(60);
    let (_, cam) = spawn_at(&mut app, "open");
    run(&mut app, 10);
    camera_mut(&mut app, cam).zoom_by(100.0);
    assert!((camera(&app, cam).zoom() - ZoomParams::default().min).abs() < 1e-6);
    app.update();
    assert!(pose(&app, cam).arm_length > 0.8 * full_arm(), "zoom snapped");
    run(&mut app, 120);
    let o = OrbitParams::default();
    let near = (o.distance.normal * ZoomParams::default().min).hypot(o.shoulder_offset);
    assert!((pose(&app, cam).arm_length - near).abs() < 0.01, "{} vs {near}", pose(&app, cam).arm_length);
}

/// A lockable target entity at `feet`.
fn spawn_target(app: &mut App, feet: Vec3) -> Entity {
    app.world_mut().spawn((Lockable::default(), Transform::from_translation(feet))).id()
}

fn forward(p: &CameraPose) -> Vec3 {
    p.rotation * Vec3::NEG_Z
}

#[test]
fn lock_on_frames_the_target_past_the_character() {
    let mut app = app(60);
    let (player, cam) = spawn_at(&mut app, "open");
    // Looking south, away from the target 10 m north.
    camera_mut(&mut app, cam).set_look(Look::new(0.0, 0.0));
    let target = spawn_target(&mut app, Vec3::new(-10.0, 0.0, 0.0));
    run(&mut app, 10);
    app.world_mut().entity_mut(player).insert(LockOn(target));
    run(&mut app, 120);
    let p = pose(&app, cam);
    assert_eq!((p.mode, p.locked), (RigMode::Locked, Some(target)));
    let aim = Vec3::new(-10.0, Lockable::default().aim_height, 0.0);
    let to_target = (aim - at(p.eye)).normalize();
    // The target is near the screen centre (the shoulder offset and the pitch bias move it a little).
    let off = forward(&p).dot(to_target).clamp(-1.0, 1.0).acos().to_degrees();
    assert!(off < 12.0, "target {off}° off the view axis");
    // The camera is behind the character: character between eye and target.
    let r = root(&app, player);
    assert!(at(p.eye).distance(aim) > r.distance(aim) + 1.0, "eye {} root {r}", at(p.eye));
    // The look (the aim the commands carry) turned to the target.
    assert!(sf_camera::wrap_angle(camera(&app, cam).look().yaw - PI).abs() < 0.05);
    // The locked arm length (eased).
    let o = OrbitParams::default();
    assert!((p.arm_length - o.distance.locked.hypot(o.shoulder_offset)).abs() < 0.02, "{}", p.arm_length);

    // Released: the look stays where the lock left it.
    app.world_mut().entity_mut(player).remove::<LockOn>();
    let look = camera(&app, cam).look();
    run(&mut app, 30);
    assert_eq!(pose(&app, cam).mode, RigMode::Normal);
    assert_eq!(camera(&app, cam).look(), look);

    // First person never hard-locks.
    app.world_mut().entity_mut(player).insert(LockOn(target));
    camera_mut(&mut app, cam).set_view(ViewChoice::FirstPerson);
    camera_mut(&mut app, cam).set_look(Look::new(0.0, 0.0));
    run(&mut app, 30);
    assert_eq!(pose(&app, cam).locked, None);
    assert_eq!(camera(&app, cam).look(), Look::new(0.0, 0.0));
}

#[test]
fn shake_is_rotation_only_and_dies_out() {
    let mut app = app(60);
    let (_, cam) = spawn_at(&mut app, "open");
    run(&mut app, 10);
    let still = pose(&app, cam);
    app.world_mut().write_message(CameraShake { trauma: 1.0 });
    app.update();
    let shaken = pose(&app, cam);
    assert_eq!(shaken.eye, still.eye);
    assert_eq!(shaken.look, still.look);
    assert!(shaken.rotation.angle_between(still.rotation) > 1e-3, "no shake");
    run(&mut app, 60);
    assert!(pose(&app, cam).rotation.angle_between(still.rotation) < 1e-6, "still shaking");
    // Off in the comfort settings: nothing.
    camera_mut(&mut app, cam).settings.shake = false;
    app.world_mut().write_message(CameraShake { trauma: 1.0 });
    app.update();
    assert!(pose(&app, cam).rotation.angle_between(still.rotation) < 1e-6);
}

/// Walks a character east past the pillar with the camera behind it (through the pillar), locks
/// on a target at 1.5 s, shakes at 2.5 s: the camera pose at 1, 2, 3 and 4 s.
fn walk_past_the_pillar(fps: u32) -> (Vec<(Vec3, Quat)>, bool, bool) {
    fn walk_east(tick: Res<ActionTick>, mut characters: Query<&mut ActionCharacter>) {
        for mut c in &mut characters {
            c.command = Command { stick: Vec2::new(0.35, 0.0), ..Command::empty(tick.0) };
        }
    }
    let mut app = app(fps);
    app.add_systems(FixedUpdate, walk_east.in_set(ActionSystems::Command));
    let (player, cam) = spawn_body(&mut app, Vec3::new(5.5, 0.0, -2.5), PI);
    let target = spawn_target(&mut app, Vec3::new(14.0, 0.0, -10.0));
    let mut out = Vec::new();
    let (mut blocked, mut locked) = (false, false);
    let fps = fps as usize;
    // Bevy's first update has a zero delta: frame `n` ends at `(n - 1) / fps` seconds. An event
    // sent before frame `frame_at(t) + 1` takes effect from `t` at every frame rate.
    let frame_at = |halves: usize| halves * fps / 2 + 1;
    for frame in 1..=frame_at(8) {
        if frame == frame_at(3) + 1 {
            app.world_mut().entity_mut(player).insert(LockOn(target));
        }
        if frame == frame_at(5) + 1 {
            app.world_mut().write_message(CameraShake { trauma: 0.8 });
        }
        app.update();
        let p = pose(&app, cam);
        blocked |= p.blocked;
        locked |= p.mode == RigMode::Locked;
        if (1..=4).any(|s| frame == frame_at(2 * s)) {
            out.push((at(p.eye), p.rotation));
        }
    }
    (out, blocked, locked)
}

#[test]
fn the_pose_is_the_same_at_30_60_and_144_fps() {
    let (reference, blocked, locked) = walk_past_the_pillar(60);
    // The walk did pass behind the pillar and lock on.
    assert!(blocked && locked);
    assert!(reference.windows(2).all(|w| w[0].0.distance(w[1].0) > 0.1), "{reference:?}");
    for fps in [30, 144] {
        let (other, ..) = walk_past_the_pillar(fps);
        for (i, (a, b)) in reference.iter().zip(&other).enumerate() {
            let d = a.0.distance(b.0);
            let angle = a.1.angle_between(b.1).to_degrees();
            assert!(d < 0.005, "{fps} fps, {} s: eye off by {d} m ({} vs {})", i + 1, b.0, a.0);
            assert!(angle < 0.1, "{fps} fps, {} s: rotation off by {angle}°", i + 1);
        }
    }
}

#[test]
fn enhanced_input_turns_the_look_toggles_the_view_and_steers_the_stick() {
    use bevy_enhanced_input::prelude::*;
    use sf_action_bevy::{ActionInputPlugin, PlayerAction, PlayerControls, PlayerInput, spawn_player_actions};

    let mut app = app(60);
    app.add_plugins((bevy_input::InputPlugin, EnhancedInputPlugin, ActionInputPlugin));
    // `EnhancedInputPlugin` creates its context registry in `finish` (`App::run` calls it).
    app.finish();
    app.cleanup();
    let (player, cam) = spawn_at(&mut app, "open");
    app.world_mut().entity_mut(player).insert((PlayerControls, PlayerInput::new(PlayerId::new(0).unwrap())));
    let (moves, looks) = {
        let mut commands = app.world_mut().commands();
        (spawn_player_actions(&mut commands, player), spawn_camera_actions(&mut commands, player))
    };
    app.world_mut().flush();
    run(&mut app, 5);
    let start = camera(&app, cam).look();
    let find = |tag: CameraAction| looks.iter().find(|(t, _)| *t == tag).unwrap().1;
    let stick = moves.iter().find(|(t, _)| *t == PlayerAction::Stick).unwrap().1;

    // Look right 0.01 rad and up 0.005 rad per frame for 20 frames.
    app.world_mut().entity_mut(find(CameraAction::Look)).insert(ActionMock::new(
        TriggerState::Fired,
        Vec2::new(0.01, 0.005),
        MockSpan::Manual,
    ));
    run(&mut app, 20);
    app.world_mut().entity_mut(find(CameraAction::Look)).insert(ActionMock::once(TriggerState::None, Vec2::ZERO));
    run(&mut app, 2);
    let look = camera(&app, cam).look();
    // Mocks apply from the next evaluation: 19-20 frames of input.
    let turned = sf_camera::wrap_angle(start.yaw - look.yaw);
    assert!((0.185..=0.205).contains(&turned), "turned {turned}");
    assert!((look.pitch - start.pitch - turned / 2.0).abs() < 1e-4, "{look:?}");
    // The look reaches the commands as the view.
    let c = app.world().get::<ActionCharacter>(player).unwrap();
    assert!((c.command.view.yaw - look.yaw).abs() < 1e-6 && (c.command.view.pitch - look.pitch).abs() < 1e-6);

    // The forward stick is resolved against the look's yaw (camera-relative movement).
    app.world_mut().entity_mut(stick).insert(ActionMock::new(TriggerState::Fired, Vec2::Y, MockSpan::Manual));
    run(&mut app, 3);
    let c = app.world().get::<ActionCharacter>(player).unwrap();
    let want = Vec2::new(look.yaw.sin(), look.yaw.cos());
    assert!(c.command.stick.distance(want) < 1e-4, "{} vs {want}", c.command.stick);
    app.world_mut().entity_mut(stick).insert(ActionMock::once(TriggerState::None, Vec2::ZERO));

    // Zoom in two notches, toggle the view, swap shoulders.
    app.world_mut().entity_mut(find(CameraAction::Zoom)).insert(ActionMock::once(TriggerState::Fired, 2.0));
    app.world_mut().entity_mut(find(CameraAction::SwapShoulder)).insert(ActionMock::once(TriggerState::Fired, true));
    run(&mut app, 3);
    assert!((camera(&app, cam).zoom() - 0.8).abs() < 1e-5, "{}", camera(&app, cam).zoom());
    assert_eq!(camera(&app, cam).shoulder(), Shoulder::Left);
    app.world_mut().entity_mut(find(CameraAction::ToggleView)).insert(ActionMock::once(TriggerState::Fired, true));
    run(&mut app, 3);
    assert_eq!(camera(&app, cam).view(), ViewChoice::FirstPerson);
}

#[test]
fn on_a_big_space_grid_far_out_the_camera_cell_is_exact() {
    use big_space::plugin::BigSpaceMinimalPlugins;
    use big_space::prelude::{BigSpace, FloatingOrigin, Grid};
    use sf_coords::GRID_CELL;

    let mut app = App::new();
    // No ground resource: the body stays put and the arm is free.
    app.add_plugins((bevy_app::TaskPoolPlugin::default(), bevy_time::TimePlugin, BigSpaceMinimalPlugins))
        .add_plugins((SfActionPlugin::<Statics>::default(), SfCameraPlugin::<Statics>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)));
    let root_e = app.world_mut().spawn((BigSpace::default(), Grid::new(GRID_CELL, 32.0), GlobalTransform::default())).id();
    // About 1000 km out, 0.1 m short of the cell's +x edge.
    let feet = WorldPos::new(IVec3::new(3907, 0, -7813), Vec3::new(127.9, 5.0, 0.0));
    let cell = WorldPos { cell: feet.cell, local: Vec3::ZERO };
    let body = ActionBody::new(Locomotion::new(feet, 0.0), MoverParams::default(), cell);
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let player = app
        .world_mut()
        .spawn((
            character,
            body,
            CellCoord::new(feet.cell.x, feet.cell.y, feet.cell.z),
            Transform::from_translation(feet.local),
            ChildOf(root_e),
        ))
        .id();
    // Looking -x (yaw -π/2): the arm runs +x, across the cell edge 0.1 m away.
    let mut pc = PlayerCamera::new(player);
    pc.set_look(Look::new(-FRAC_PI_2, 0.0));
    let cam = app.world_mut().spawn((pc, CellCoord::default(), Transform::default(), FloatingOrigin, ChildOf(root_e))).id();
    run(&mut app, 5);

    let p = pose(&app, cam);
    let c = app.world().get::<CellCoord>(cam).unwrap();
    let t = app.world().get::<Transform>(cam).unwrap();
    assert_eq!(IVec3::new(c.x, c.y, c.z), p.eye.cell);
    assert_eq!(p.eye.cell.x, feet.cell.x + 1, "the eye should be in the next cell");
    assert_eq!(t.translation, p.eye.local);
    // Rendered relative to the floating origin (the camera): small numbers, exact arm.
    let g_cam = app.world().get::<GlobalTransform>(cam).unwrap().translation();
    let g_body = app.world().get::<GlobalTransform>(player).unwrap().translation();
    assert!(g_cam.length() < 300.0 && g_body.length() < 300.0, "{g_cam} {g_body}");
    let arm = feet.delta_f32(p.eye);
    assert!(((g_cam - g_body) - arm).length() < 1e-4, "{} vs {arm}", g_cam - g_body);
    assert!((arm - Vec3::Y * OrbitParams::default().pivot_height).length() > 3.0);
}

/// The rig runs before propagation: the transform written this frame is the one drawn.
#[test]
fn the_rig_runs_in_post_update_before_propagation() {
    let mut app = app(60);
    let (_, cam) = spawn_at(&mut app, "open");
    app.add_systems(
        PostUpdate,
        (move |cams: Query<(&PlayerCamera, &Transform)>| {
            let (c, t) = cams.get(cam).unwrap();
            assert_eq!(c.pose().map(|p| at(p.eye)), Some(t.translation));
        })
        .after(CameraSystems::Rig),
    );
    run(&mut app, 3);
}
