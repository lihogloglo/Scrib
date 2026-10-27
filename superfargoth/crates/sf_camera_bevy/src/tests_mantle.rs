//! The camera through a waist mantle (AL-8): a box ledge on a floor, the action shell's Warp
//! move (a straight-line climb root, like the retargeted UAL2 `ClimbUp_1m_RM`), and the player
//! camera behind. Records the arm, the pivot lag and the push-in per frame.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Vec2, Vec3};
use bevy_time::TimeUpdateStrategy;
use bevy_transform::components::Transform;
use sf_action::{
    ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, Origin, RootPath, RuleKey, RuleSetBuilder, SlotInfo,
    SlotTable, TRAVERSAL_RULES,
};
use sf_action_bevy::{ActionRules, ActionTick, SfActionPlugin};
use sf_anim::RootPose;
use sf_controller::{CollisionBuilder, Locomotion, MoverParams, StaticShape, StaticWorld, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};

use crate::*;

/// The floor and one box ledge (2 m wide, 1.5 m deep, its face at z = 0, the body comes from -z).
#[derive(Resource)]
struct Course(StaticWorld);

impl GroundProbe for Course {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.0.probe(queries, out);
    }
}

fn add_box(statics: &mut StaticWorld, min: Vec3, max: Vec3) {
    let h = (max - min) / 2.0;
    let c = |x: f32, y: f32, z: f32| [x * h.x, y * h.y, z * h.z];
    let corners = [
        c(-1.0, -1.0, -1.0),
        c(1.0, -1.0, -1.0),
        c(1.0, 1.0, -1.0),
        c(-1.0, 1.0, -1.0),
        c(-1.0, -1.0, 1.0),
        c(1.0, -1.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(-1.0, 1.0, 1.0),
    ];
    let idx = [0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5];
    let mut b = CollisionBuilder::new(1e-3);
    b.add(&corners, &idx, Affine3A::IDENTITY, SurfaceId(3));
    let shape = Arc::new(StaticShape::new(&b.finish()).expect("box shape"));
    statics.insert(&shape, WorldTransform::from_pos(WorldPos::ORIGIN.offset((min + max) * 0.5)));
}

const FACE_Z: f32 = 0.0;

fn course(height: f32) -> Course {
    let mut statics = StaticWorld::new(WorldPos::ORIGIN);
    add_box(&mut statics, Vec3::new(-20.0, -1.0, -20.0), Vec3::new(20.0, 0.0, 20.0));
    add_box(&mut statics, Vec3::new(-1.0, 0.0, FACE_Z), Vec3::new(1.0, height, FACE_Z + 1.5));
    statics.rebuild();
    Course(statics)
}

/// The retargeted `ClimbUp_1m_RM` root (`sf_action_bevy` `tests_mantle`): up 1.2 m and forward
/// 2 m on a straight line in 0.667 s.
struct Climb;

impl RootPath for Climb {
    fn duration(&self) -> f32 {
        0.667
    }
    fn root_at(&self, t: f32) -> RootPose {
        RootPose::new(Vec3::new(0.0, 1.2, 2.0) * (t / 0.667).clamp(0.0, 1.0), 0.0)
    }
}

fn rules() -> ActionRules {
    let slots = SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
        .with("traverse.mantle.waist", SlotInfo::once(Climb.duration()))
        .with_root("traverse.mantle.waist", Arc::new(Climb));
    let mut b = RuleSetBuilder::new();
    b.add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Framework, TRAVERSAL_RULES);
    ActionRules { rules: b.build().expect("base + traversal rules"), slots: Box::new(slots) }
}

/// The tick Jump is pressed on (forward held until tick 150).
#[derive(Resource)]
struct JumpAt(u32);

fn forward_and_jump(tick: Res<ActionTick>, jump: Res<JumpAt>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        let mut cmd = Command::empty(tick.0);
        cmd.stick = if tick.0.0 < 150 { Vec2::Y } else { Vec2::ZERO };
        if tick.0.0 == jump.0 {
            cmd.buttons.pressed.insert(ActionClass::Jump);
            cmd.buttons.held.insert(ActionClass::Jump);
        }
        c.command = cmd;
    }
}

/// One rendered frame of the run.
#[derive(Clone, Copy, Debug)]
struct Frame {
    mov: RuleKey,
    root: Vec3,
    pivot_lag: Vec3,
    eye: Vec3,
    arm: f32,
    blocked: bool,
}

/// Walk from `start_z` toward a `height` m box at `fps`, Jump on tick `jump_at`, look `pitch`
/// (radians, up positive) along +z; `frames` frames.
fn mantle_run(height: f32, start_z: f32, jump_at: u32, pitch: f32, fps: u32, frames: usize) -> Vec<Frame> {
    let mut app = App::new();
    app.add_plugins((bevy_app::TaskPoolPlugin::default(), bevy_time::TimePlugin))
        .add_plugins((SfActionPlugin::<Course>::default(), SfCameraPlugin::<Course>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / f64::from(fps))))
        .insert_resource(course(height))
        .insert_resource(rules())
        .insert_resource(JumpAt(jump_at))
        .add_systems(FixedUpdate, forward_and_jump.in_set(ActionSystems::Command));
    let feet = Vec3::new(0.0, 0.0, start_z);
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let body = ActionBody::new(Locomotion::new(WorldPos::ORIGIN.offset(feet), 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let player = app.world_mut().spawn((character, body, Transform::from_translation(feet))).id();
    let mut cam = PlayerCamera::new(player);
    cam.set_look(Look::new(0.0, pitch));
    let camera = app.world_mut().spawn((cam, Transform::default())).id();
    let mut out = Vec::with_capacity(frames);
    for _ in 0..frames {
        app.update();
        let w = app.world();
        let body = w.get::<ActionBody>(player).unwrap();
        let t = w.get::<Transform>(player).unwrap();
        let cam = w.get::<PlayerCamera>(camera).unwrap();
        let Some(pose) = cam.pose() else { continue };
        out.push(Frame {
            mov: w.get::<ActionCharacter>(player).unwrap().state.mov,
            root: WorldPos::ORIGIN.delta_f32(body.frame_origin.offset(t.translation)),
            pivot_lag: cam.orbit.lag(),
            eye: WorldPos::ORIGIN.delta_f32(pose.eye),
            arm: pose.arm_length,
            blocked: pose.blocked,
        });
    }
    out
}

fn mantled(frames: &[Frame]) -> bool {
    frames.iter().any(|f| f.mov == RuleKey::of("traverse.mantle.waist"))
}

#[test]
#[ignore = "measurement: per-frame camera arm through the 1 m and 1.5 m mantles"]
fn camera_through_the_mantle() {
    let cases = [1.0f32, 1.5].into_iter().flat_map(|h| [33u32, 49].into_iter().flat_map(move |j| [-0.35f32, 0.0, 0.3].map(|p| (h, j, p))));
    for (height, jump, pitch) in cases {
        println!("== {height} m box, jump tick {jump}, pitch {pitch}");
        println!("  frame move                      root(y,z)        lag(y,z)        arm   blocked eye(y,z)");
        let frames = mantle_run(height, -2.5, jump, pitch, 60, 140);
        for (i, f) in frames.iter().enumerate().skip(jump as usize).take(70) {
            println!(
                "  {i:3} {:<28} {:5.2},{:5.2}   {:6.3},{:6.3}   {:4.2}  {:5}  {:5.2},{:5.2}",
                format!("{:?}", f.mov),
                f.root.y,
                f.root.z,
                f.pivot_lag.y,
                f.pivot_lag.z,
                f.arm,
                f.blocked,
                f.eye.y,
                f.eye.z
            );
        }
    }
}

#[test]
#[ignore = "measurement: arm and pivot lag summary over jump distances and pitches"]
fn camera_mantle_sweep() {
    for height in [1.0f32, 1.5] {
        for jump in (1..60).step_by(4) {
            for pitch in [-0.6f32, -0.35, 0.0, 0.3, 0.6] {
                let frames = mantle_run(height, -2.5, jump, pitch, 60, 150);
                if !mantled(&frames) {
                    continue;
                }
                let start = frames.iter().position(|f| f.mov == RuleKey::of("traverse.mantle.waist")).unwrap();
                // From just before the climb to 0.25 s after it (not the walk off the back).
                let end = frames.iter().rposition(|f| f.mov == RuleKey::of("traverse.mantle.waist")).unwrap();
                let w = &frames[start.saturating_sub(5)..(end + 15).min(frames.len())];
                let min_arm = w.iter().map(|f| f.arm).fold(f32::MAX, f32::min);
                let blocked = w.iter().filter(|f| f.blocked).count();
                let lag = w.iter().map(|f| f.pivot_lag.length()).fold(0.0, f32::max);
                println!(
                    "{height} m jump {jump:2} (from z {:5.2}) pitch {pitch:5.2}: min arm {min_arm:4.2}, blocked {blocked:2} frames, max lag {lag:4.2}",
                    frames[start].root.z
                );
            }
        }
    }
}

/// AL-8: jumped from close to the 1.5 m box, the pivot used to ride the climb root through the
/// lip and the arm collapsed into the chest (0 m). With nothing between the body and the camera
/// (level or downward look), the arm stays out through the whole mantle.
#[test]
fn the_arm_stays_out_through_a_mantle() {
    for height in [1.0f32, 1.5] {
        for jump in [33u32, 45, 49, 53] {
            for pitch in [-0.35f32, 0.0] {
                let frames = mantle_run(height, -2.5, jump, pitch, 60, 140);
                assert!(mantled(&frames), "{height} m, jump {jump}: no mantle");
                let start = frames.iter().position(|f| f.mov == RuleKey::of("traverse.mantle.waist")).unwrap();
                let end = frames.iter().rposition(|f| f.mov == RuleKey::of("traverse.mantle.waist")).unwrap();
                for (i, f) in frames.iter().enumerate().take(end + 15).skip(start.saturating_sub(5)) {
                    assert!(f.eye.is_finite() && f.arm > 1.0, "{height} m, jump {jump}, pitch {pitch}, frame {i}: arm {}", f.arm);
                }
            }
        }
    }
}
