//! IN20: the ground probe reports the body it hit and the ground's velocity at
//! the contact (travel.md decision 4, based movement inputs).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use glam::{Quat, Vec3};
use sf_controller::{GroundBody, GroundHit, GroundProbe, ProbeQuery};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

/// Float tolerance on velocities, m/s: f32 poses ~10 m from the anchor over
/// a 1/144 s pose delta.
const TOL: f32 = 2e-3;

fn assert_close(got: Vec3, want: Vec3, what: &str) {
    assert!((got - want).length() < TOL, "{what}: got {got}, want {want}");
}

/// The hit of a ray straight down from 2 m above `p`.
fn probe_down(w: &PhysicsWorld, p: WorldPos) -> GroundHit {
    w.probe(Layer::STATIC | Layer::PROP).probe_one(ProbeQuery::down(p.offset(Vec3::Y * 2.0), 4.0)).expect("ground below")
}

/// A deck-sized kinematic box translating at `v` and yawing at `yaw_rate`,
/// stepped at `hz` for half a second through per-tick kinematic targets.
fn turning_deck(hz: f32) {
    let params = PhysicsParams { hz: f64::from(hz), ..PhysicsParams::default() };
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, params);
    let start = WorldPos::ORIGIN.offset(Vec3::new(10.0, 1.0, 5.0));
    let deck = w.insert(&BodyDesc::new(BodyKind::Kinematic, at(start), ShapeRef::Box(Vec3::new(4.0, 0.25, 2.0)), Layer::STATIC));
    let v = Vec3::new(3.0, 0.0, 1.5);
    let yaw_rate = 0.6;
    let steps = (hz * 0.5).round() as u32;
    let pose = |k: u32| {
        let t = k as f32 / hz;
        WorldTransform { pos: start.offset(v * t), rotation: Quat::from_rotation_y(yaw_rate * t), scale: Vec3::ONE }
    };
    for k in 1..=steps {
        assert!(w.set_kinematic_target(deck, pose(k)));
        w.step();
    }
    let now = pose(steps);
    let body = w.pose(deck).expect("deck");
    assert!(body.pos.delta_f32(now.pos).length() < 1e-4, "deck reached its target");
    // Off-centre: 2.5 m forward-right of the centre in deck axes, on the top face.
    let over = now.pos.offset(now.rotation * Vec3::new(2.5, 0.0, -1.2));
    let hit = probe_down(&w, over);
    assert_eq!(hit.body, Some(GroundBody::from(deck)), "{hz} Hz: the deck");
    assert_eq!(BodyId::from(hit.body.expect("body")), deck);
    let omega = Vec3::Y * yaw_rate;
    let r = now.pos.delta_f32(hit.point);
    assert!((r - now.rotation * Vec3::new(2.5, 0.25, -1.2)).length() < 1e-3, "{hz} Hz: hit on the top face, r = {r}");
    assert_close(hit.angular_velocity, omega, &format!("{hz} Hz angular"));
    assert_close(hit.velocity, v + omega.cross(r), &format!("{hz} Hz point velocity"));

    // Targets stop: the next tick's pose delta is zero, so is the velocity.
    w.step();
    let still = probe_down(&w, over);
    assert_eq!(still.body, Some(GroundBody::from(deck)));
    assert_close(still.velocity, Vec3::ZERO, &format!("{hz} Hz stopped"));
    assert_close(still.angular_velocity, Vec3::ZERO, &format!("{hz} Hz stopped angular"));
}

#[test]
fn kinematic_deck_point_velocity_at_30_hz() {
    turning_deck(30.0);
}

#[test]
fn kinematic_deck_point_velocity_at_144_hz() {
    turning_deck(144.0);
}

#[test]
fn fixed_ground_has_no_body_and_no_velocity() {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let floor = ground(&mut w, WorldPos::ORIGIN);
    w.step();
    let over = WorldPos::ORIGIN.offset(Vec3::new(3.0, 0.0, -2.0));
    let hit = probe_down(&w, over);
    assert_eq!(hit.body, None);
    assert_eq!((hit.velocity, hit.angular_velocity), (Vec3::ZERO, Vec3::ZERO));
    // The pick path still names the body.
    let (picked, _) = w.probe(Layer::STATIC).cast(&ProbeQuery::down(over.offset(Vec3::Y * 2.0), 4.0)).expect("floor");
    assert_eq!(picked, floor);
}

/// Dynamic bodies report the solver's velocities: a raft drifting and
/// spinning in zero gravity (on the ground, rolling resistance stops the spin).
#[test]
fn drifting_dynamic_body_reports_its_point_velocity() {
    let params = PhysicsParams { gravity: Vec3::ZERO, ..PhysicsParams::default() };
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, params);
    let pose = at(WorldPos::ORIGIN.offset(Vec3::new(0.0, 0.25, 0.0)));
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Box(Vec3::new(1.0, 0.25, 1.0)), Layer::PROP);
    let slab = w.insert(&BodyDesc { mass: Some(20.0), ..desc });
    w.set_velocity(slab, Vec3::new(4.0, 0.0, -1.0), Vec3::new(0.0, 1.5, 0.0));
    for _ in 0..5 {
        w.step();
    }
    let (linear, angular) = w.velocity(slab).expect("slab");
    assert!(linear.length() > 1.0 && angular.length() > 0.2, "still drifting and spinning: {linear} {angular}");
    let now = w.pose(slab).expect("slab");
    let (com, _) = w.mass_center(slab).expect("slab");
    let com = now.pos.offset(now.rotation * com);
    let over = now.pos.offset(now.rotation * Vec3::new(0.6, 0.0, 0.4));
    let hit = probe_down(&w, over);
    assert_eq!(hit.body, Some(GroundBody::from(slab)));
    assert_close(hit.angular_velocity, angular, "angular");
    assert_close(hit.velocity, linear + angular.cross(com.delta_f32(hit.point)), "point velocity");
}
