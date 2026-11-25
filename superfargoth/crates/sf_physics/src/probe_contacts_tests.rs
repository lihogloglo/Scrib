//! `IN11c2a`: `PhysicsProbe::contacts` reports moving bodies against upright capsules as the
//! `GroundProbe::contacts` contract says (`sf_controller` `probe.rs`): the face the body came
//! in by, the signed separation along it, the body and its velocity; never fixed bodies,
//! bodies at rest, bodies moving away or dynamic bodies (kinematic ones only).

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::time::Instant;

use glam::Vec3;
use sf_controller::{GroundBody, GroundHit, GroundProbe, ProbeQuery};
use sf_coords::{WorldPos, WorldTransform};

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

const RADIUS: f32 = 0.3;
const HALF: f32 = 0.45;
const MARGIN: f32 = 0.01;
/// A thin wall: 0.2 m thick, 2 m tall, 6 m wide.
const WALL: Vec3 = Vec3::new(0.1, 1.0, 3.0);

fn base() -> WorldPos {
    WorldPos::ORIGIN.offset(Vec3::new(5.0, 0.0, -3.0))
}

/// The capsule's centre (feet at `base`, raised over the ground).
fn capsule() -> WorldPos {
    base().offset(Vec3::Y * 1.2)
}

fn query() -> ProbeQuery {
    ProbeQuery::contact(capsule(), MARGIN, RADIUS, HALF)
}

/// A world at `hz` with ground and a wall of `kind` whose +x face starts `behind` m behind
/// the capsule's surface (−x).
fn world(hz: f32, kind: BodyKind, behind: f32) -> (PhysicsWorld, BodyId, WorldPos) {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams { dt: 1.0 / hz, ..PhysicsParams::default() });
    ground(&mut w, base());
    let start = capsule().offset(Vec3::new(-RADIUS - behind - WALL.x, 0.0, 0.0));
    let wall = w.insert(&BodyDesc::new(kind, at(start), ShapeRef::Box(WALL), Layer::STATIC));
    w.refresh_queries();
    (w, wall, start)
}

fn contact(w: &PhysicsWorld) -> Option<GroundHit> {
    let mut out = [None];
    w.probe(Layer::STATIC | Layer::PROP).contacts(&[query()], &mut out);
    out[0]
}

/// One step with the kinematic `wall` moved by `d`.
fn shove(w: &mut PhysicsWorld, wall: BodyId, d: Vec3) {
    let pos = w.pose(wall).expect("wall").pos.offset(d);
    assert!(w.set_kinematic_target(wall, WorldTransform::from_pos(pos)));
    w.step();
}

/// A wall stepping 0.05 m toward a capsule 0.03 m from it ends 0.02 m inside it: the face
/// toward the capsule, at −0.02 m, with the wall's velocity.
#[test]
fn an_approaching_wall_reports_its_face_and_the_depth() {
    let (mut w, wall, _) = world(60.0, BodyKind::Kinematic, 0.03);
    shove(&mut w, wall, Vec3::X * 0.05);
    let hit = contact(&w).expect("a contact");
    println!("approach: normal {}, distance {:.5} m, velocity {}", hit.normal, hit.distance, hit.velocity);
    assert!(hit.normal.dot(Vec3::X) > 0.999, "normal {}", hit.normal);
    assert!((hit.distance + 0.02).abs() < 1e-4, "distance {}", hit.distance);
    assert_eq!(hit.body, Some(GroundBody::from(wall)));
    assert!((hit.velocity - Vec3::X * 3.0).length() < 1e-2, "velocity {}", hit.velocity);
}

/// A 0.2 m wall stepping 0.5 m (30 m/s at 60 Hz) from 0.005 m behind the capsule ends past
/// its centre: still the face it came in by (+x), at −0.495 m, not the shallow way out.
#[test]
fn a_fast_thin_wall_reports_the_face_it_came_in_by() {
    let (mut w, wall, _) = world(60.0, BodyKind::Kinematic, 0.005);
    shove(&mut w, wall, Vec3::X * 0.5);
    let hit = contact(&w).expect("a contact");
    println!("fast: normal {}, distance {:.5} m", hit.normal, hit.distance);
    assert!(hit.normal.dot(Vec3::X) > 0.999, "normal {}", hit.normal);
    assert!((hit.distance + 0.495).abs() < 1e-3, "distance {}", hit.distance);
}

/// Inside the margin: a fixed wall, a kinematic wall at rest, one moving away and a dynamic
/// one moving in report nothing.
#[test]
fn fixed_resting_receding_and_dynamic_bodies_never_push() {
    let (w, _, _) = world(60.0, BodyKind::Fixed, 0.005);
    assert_eq!(contact(&w), None, "fixed");
    let (mut w, wall, _) = world(60.0, BodyKind::Kinematic, 0.005);
    shove(&mut w, wall, Vec3::ZERO);
    assert_eq!(contact(&w), None, "at rest");
    let (mut w, wall, _) = world(60.0, BodyKind::Kinematic, 0.002);
    shove(&mut w, wall, Vec3::NEG_X * 0.05);
    assert_eq!(contact(&w), None, "receding");
    // Dynamic: the solver and the impact path (`PhysicsEvent::Hit`) handle it, not a push.
    let (mut w, wall, _) = world(60.0, BodyKind::Dynamic { asleep: false }, 0.03);
    w.set_velocity(wall, Vec3::X * 3.0, Vec3::ZERO);
    w.step();
    assert_eq!(contact(&w), None, "dynamic");
}

/// Cost: one batched `contacts` call for 50 capsules among 20 moving kinematic boxes (each
/// within a metre of several capsules), µs per capsule.
#[test]
#[ignore = "measurement: contacts query cost per character"]
fn contacts_cost_per_character() {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    ground(&mut w, base());
    let boxes: Vec<BodyId> = (0..20)
        .map(|i| {
            let p = base().offset(Vec3::new((i % 5) as f32 * 2.0, 1.0, (i / 5) as f32 * 2.0));
            w.insert(&BodyDesc::new(BodyKind::Kinematic, at(p), ShapeRef::Box(Vec3::new(0.5, 1.0, 0.5)), Layer::STATIC))
        })
        .collect();
    w.refresh_queries();
    for b in &boxes {
        let pos = w.pose(*b).expect("box").pos.offset(Vec3::new(0.02, 0.0, 0.01));
        assert!(w.set_kinematic_target(*b, WorldTransform::from_pos(pos)));
    }
    w.step();
    let queries: Vec<ProbeQuery> = (0..50)
        .map(|i| {
            let p = base().offset(Vec3::new((i % 10) as f32 * 0.9 + 1.0, 1.2, (i / 10) as f32 * 1.6 + 1.0));
            ProbeQuery::contact(p, MARGIN, RADIUS, HALF)
        })
        .collect();
    let mut out = vec![None; queries.len()];
    let probe = w.probe(Layer::STATIC | Layer::PROP);
    probe.contacts(&queries, &mut out);
    let runs = 200;
    let started = Instant::now();
    for _ in 0..runs {
        probe.contacts(&queries, &mut out);
    }
    let us = started.elapsed().as_secs_f64() * 1e6 / f64::from(runs) / queries.len() as f64;
    let hits = out.iter().filter(|h| h.is_some()).count();
    println!("contacts: {us:.2} µs per character (50 characters, 20 moving boxes, {hits} in contact)");
}
