//! Headless mantle (action.md B2a): the shell's context probe senses a box ledge on a Jump press
//! and the Warp move places the body on top; too high a ledge refuses into a plain jump.

use std::sync::Arc;
use std::time::Duration;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Vec2, Vec3};
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use bevy_transform::components::Transform;
use sf_action::{
    ActionClass, ActionState, BASE_RULES, CharacterId, CharacterParams, Command, Origin, RuleKey, RuleSetBuilder, SlotInfo, SlotTable,
    TRAVERSAL_RULES,
};
use sf_anim::{RootPath, RootPose};
use sf_controller::{
    CollisionBuilder, CollisionMesh, GroundHit, GroundProbe, Locomotion, MoverParams, Nearest, ProbeQuery, StaticShape, StaticWorld, SurfaceId,
    TerrainProbe,
};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::Plane;
use crate::*;

/// The plane plus one box ledge across the path.
#[derive(Resource)]
struct Course {
    terrain: TerrainProbe<Plane>,
    statics: StaticWorld,
}

impl GroundProbe for Course {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        Nearest(&[&self.terrain, &self.statics]).probe(queries, out);
    }
}

/// A box ledge `height` m tall whose face is 1 m ahead of the start (+Z), 4 m deep.
fn course(height: f32) -> Course {
    let (min, max) = (Vec3::new(-3.0, 0.0, 2.0), Vec3::new(3.0, height, 6.0));
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
    let mesh: CollisionMesh = b.finish();
    let mut statics = StaticWorld::new(WorldPos::ORIGIN);
    statics.insert(&Arc::new(StaticShape::new(&mesh).expect("box shape")), WorldTransform::from_pos(WorldPos::ORIGIN.offset((min + max) * 0.5)));
    statics.rebuild();
    Course { terrain: TerrainProbe(Plane), statics }
}

/// The clip's root track stand-in: up 1.2 m and forward 2 m in 0.667 s (the retargeted UAL2
/// `ClimbUp_1m_RM` measures about that).
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

/// Sneak held from this tick on, if any.
#[derive(Resource)]
struct SneakFrom(Option<u32>);

/// Forward from the start; Jump pressed on tick 10.
fn forward_and_jump(tick: Res<ActionTick>, sneak: Res<SneakFrom>, mut characters: Query<&mut ActionCharacter>) {
    for mut c in &mut characters {
        let mut cmd = Command::empty(tick.0);
        cmd.stick = if tick.0.0 < 60 { Vec2::Y } else { Vec2::ZERO };
        if tick.0.0 == 10 {
            cmd.buttons.pressed.insert(ActionClass::Jump);
            cmd.buttons.held.insert(ActionClass::Jump);
        }
        if sneak.0.is_some_and(|t| tick.0.0 >= t) {
            cmd.buttons.held.insert(ActionClass::Sneak);
        }
        c.command = cmd;
    }
}

/// The last feet position and the moves in order, over `ticks` ticks at a ledge `height` m tall.
fn run(height: f32, ticks: usize) -> (Vec3, Vec<RuleKey>) {
    run_with(height, ticks, None)
}

fn run_with(height: f32, ticks: usize, sneak_from: Option<u32>) -> (Vec3, Vec<RuleKey>) {
    let mut app = App::new();
    app.add_plugins((TimePlugin, SfActionPlugin::<Course>::default()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(course(height))
        .insert_resource(rules())
        .insert_resource(SneakFrom(sneak_from))
        .add_systems(FixedUpdate, forward_and_jump.in_set(ActionSystems::Command));
    let character = ActionCharacter::new(CharacterId(1), ActionState::new(RuleKey::of("loco")), CharacterParams::default());
    let start = WorldPos::ORIGIN.offset(Vec3::Z * 1.0);
    let body = ActionBody::new(Locomotion::new(start, 0.0), MoverParams::default(), WorldPos::ORIGIN);
    let e = app.world_mut().spawn((character, body, Transform::default())).id();
    let mut rules_seen = Vec::new();
    for _ in 0..ticks {
        app.update();
        let c = app.world().get::<ActionCharacter>(e).unwrap();
        if rules_seen.last() != Some(&c.state.mov) {
            rules_seen.push(c.state.mov);
        }
        let p = WorldPos::ORIGIN.delta_f32(app.world().get::<ActionBody>(e).unwrap().loco.position);
        assert!(p.is_finite() && p.y > -0.05, "never through the floor: {p}");
    }
    let body = app.world().get::<ActionBody>(e).unwrap();
    (WorldPos::ORIGIN.delta_f32(body.loco.position), rules_seen)
}

#[test]
fn a_waist_high_ledge_is_mantled_and_the_body_walks_on_from_its_top() {
    let mantle = RuleKey::of("traverse.mantle.waist");
    let (feet, seen) = run(1.0, 150);
    println!("1 m ledge: feet {feet}, moves {seen:?}");
    assert!(seen.contains(&mantle), "the mantle fired: {seen:?}");
    assert!((feet.y - 1.0).abs() < 0.05 && feet.z > 2.0, "ends on top: {feet}");
    assert_eq!(seen.last(), Some(&RuleKey::of("loco")), "hands back to walking");
}

#[test]
fn a_ledge_above_the_band_refuses_into_a_plain_jump() {
    let (feet, seen) = run(2.0, 150);
    println!("2 m ledge: feet {feet}, moves {seen:?}");
    assert!(!seen.contains(&RuleKey::of("traverse.mantle.waist")), "no mantle: {seen:?}");
    assert!(feet.y.abs() < 0.05 && feet.z < 2.0, "stays below, against the face: {feet}");
}

#[test]
fn sneaking_never_mantles() {
    let (feet, seen) = run_with(1.0, 150, Some(0));
    println!("sneak at a 1 m ledge: feet {feet}, moves {seen:?}");
    assert!(!seen.contains(&RuleKey::of("traverse.mantle.waist")), "no mantle while sneaking: {seen:?}");
    assert!(feet.y.abs() < 0.05 && feet.z < 2.0, "stays below: {feet}");
}

#[test]
fn sneak_pressed_during_a_mantle_does_not_leave_the_body_in_the_box() {
    let (feet, seen) = run_with(1.0, 150, Some(14));
    println!("sneak mid-mantle: feet {feet}, moves {seen:?}");
    let inside = feet.y < 0.95 && feet.z > 2.0 - 0.29;
    assert!(!inside, "the capsule is not left inside the ledge: {feet}");
}
