//! P5: the kinematic mover on physics (physics.md "Controller").

#![allow(clippy::cast_precision_loss)]

use glam::{IVec3, Vec3};
use sf_controller::{KinematicMover, Locomotion, LocomotionIntent, MoveInput, MoverParams, RootMotion};
use sf_coords::WorldPos;

use crate::tests::{at, ground};
use crate::{BodyDesc, BodyId, BodyKind, CharacterShape, Layer, PhysicsEvent, PhysicsParams, PhysicsWorld, PushParams, ShapeRef};

const DT: f32 = 1.0 / 60.0;

fn world() -> (PhysicsWorld, WorldPos) {
    let base = WorldPos::new(IVec3::new(3, 0, -2), Vec3::new(10.0, 4.0, 10.0));
    let mut w = PhysicsWorld::new(base, PhysicsParams::default());
    ground(&mut w, base);
    (w, base)
}

/// A walker on physics: the mover probes `STATIC | TERRAIN | PROP`, its
/// capsule follows, and it pushes what it touches. Returns the capsule.
struct Walker {
    loco: Locomotion,
    params: MoverParams,
    capsule: BodyId,
    shape: CharacterShape,
    push: PushParams,
    mover: KinematicMover,
}

impl Walker {
    fn new(w: &mut PhysicsWorld, feet: WorldPos) -> Self {
        let params = MoverParams::default();
        let shape = CharacterShape { radius: params.radius, height: params.height };
        let push = PushParams::default();
        let capsule = w.insert(&BodyDesc::character(feet, shape, &push));
        Self { loco: Locomotion::new(feet, 0.0), params, capsule, shape, push, mover: KinematicMover::new() }
    }

    /// One frame: mover, capsule target, pushes, one physics step.
    fn frame(&mut self, w: &mut PhysicsWorld, velocity: Vec3) -> crate::Pushed {
        w.refresh_queries();
        let intent = LocomotionIntent { desired_velocity: velocity, ..LocomotionIntent::IDLE };
        let probe = w.probe(Layer::STATIC | Layer::TERRAIN | Layer::PROP);
        let input = MoveInput { loco: &mut self.loco, intent, params: &self.params, root: RootMotion::default() };
        self.mover.step(input, &probe, DT);
        w.move_character(self.capsule, self.loco.position, self.shape);
        // The intended velocity: the mover removes the part into a wall it hit.
        let pushed = w.push(self.loco.position, self.shape, velocity, &self.push, DT);
        w.step();
        pushed
    }
}

/// A heavy crate blocks the walker, then yields to sustained pushing; no
/// single push is over the cap.
#[test]
fn capsule_pushes_a_crate() {
    let (mut w, base) = world();
    let crate_at = base.offset(Vec3::new(0.0, 0.4, 2.0));
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: true }, at(crate_at), ShapeRef::Box(Vec3::splat(0.4)), Layer::PROP);
    let crate_ = w.insert(&BodyDesc { mass: Some(60.0), ..desc });
    let mut walker = Walker::new(&mut w, base);
    let cap = walker.push.push_force * DT;
    let mut max = 0.0_f32;
    for _ in 0..60 {
        max = max.max(walker.frame(&mut w, Vec3::Z * 1.5).max_impulse);
    }
    // After a second the walker is against the crate, not through it.
    let feet_z = base.delta_f32(walker.loco.position).z;
    let crate_z = base.delta_f32(w.pose(crate_).unwrap().pos).z;
    assert!(feet_z < crate_z - 0.4 - walker.params.radius + 0.02, "walker {feet_z} vs crate {crate_z}");
    for _ in 0..180 {
        max = max.max(walker.frame(&mut w, Vec3::Z * 1.5).max_impulse);
    }
    let moved = base.delta_f32(w.pose(crate_).unwrap().pos).z - 2.0;
    assert!(moved > 0.5, "the crate yields: moved {moved}");
    assert!(max <= cap + 1e-4, "push {max} over the cap {cap}");
    let feet = base.delta_f32(walker.loco.position).z;
    assert!(feet > 0.5, "the walker follows the crate: {feet}");
}

/// A cup in the path never blocks the walker; it is pushed aside.
#[test]
fn debris_never_blocks() {
    let (mut w, base) = world();
    let cup_at = base.offset(Vec3::new(0.05, 0.05, 1.0));
    let cup = w.insert(&BodyDesc::item(at(cup_at), ShapeRef::Box(Vec3::splat(0.05)), Layer::DEBRIS, Some(0.3)));
    let mut walker = Walker::new(&mut w, base);
    for _ in 0..120 {
        walker.frame(&mut w, Vec3::Z * 1.5);
    }
    let feet = base.delta_f32(walker.loco.position).z;
    assert!(feet > 1.8, "walked past the cup: {feet}");
    let moved = cup_at.delta_f32(w.pose(cup).unwrap().pos).length();
    assert!(moved > 0.1, "the cup was pushed: {moved}");
}

/// A thrown ball hits a standing walker: a contact force on the capsule, and
/// the ball doesn't pass through it.
#[test]
fn ball_hits_a_walker() {
    let (mut w, base) = world();
    let mut walker = Walker::new(&mut w, base);
    walker.frame(&mut w, Vec3::ZERO);
    let start = base.offset(Vec3::new(0.0, 1.2, -3.0));
    let ball = w.insert(&BodyDesc {
        ccd: true,
        mass: Some(3.0),
        ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(start), ShapeRef::Sphere(0.1), Layer::PROP)
    });
    w.set_velocity(ball, Vec3::Z * 25.0, Vec3::ZERO);
    let mut hit = false;
    let mut hits = Vec::new();
    for _ in 0..60 {
        walker.frame(&mut w, Vec3::ZERO);
        for e in w.drain_events() {
            hit |= matches!(e, PhysicsEvent::ContactForce { a, b, .. } if a == walker.capsule || b == walker.capsule);
            hits.extend(capsule_hit(e, walker.capsule, ball));
        }
    }
    assert!(hit, "contact force on the capsule");
    // One hit, the ball's momentum (3 kg × 25 m/s, less a little gravity and drag).
    assert_eq!(hits.len(), 1, "one hit: {hits:?}");
    assert!((70.0..=76.0).contains(&hits[0].0), "impact {hits:?}");
    assert!(hits[0].1.z > 0.9, "pushes the walker along the throw: {hits:?}");
    let z = base.delta_f32(w.pose(ball).unwrap().pos).z;
    assert!(z < 0.0, "the ball bounced off, not through: z {z}");
}

/// The impact and direction of a [`PhysicsEvent::Hit`] on `capsule` by `by`.
fn capsule_hit(e: PhysicsEvent, capsule: BodyId, by: BodyId) -> Option<(f32, Vec3)> {
    match e {
        PhysicsEvent::Hit { target, by: b, impulse, direction, .. } if target == capsule && b == by => Some((impulse, direction)),
        _ => None,
    }
}

/// A soft 2 m/s throw (PH-10): exactly one hit with the ball's momentum,
/// about 6 N·s, even though the solver spreads the contact force over steps
/// (under the contact-force threshold in every one of them).
#[test]
fn soft_ball_is_one_soft_hit() {
    let (mut w, base) = world();
    let mut walker = Walker::new(&mut w, base);
    walker.frame(&mut w, Vec3::ZERO);
    let ball = w.insert(&BodyDesc {
        ccd: true,
        mass: Some(3.0),
        ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(base.offset(Vec3::new(0.0, 1.2, -0.6))), ShapeRef::Sphere(0.1), Layer::PROP)
    });
    // Ballistic onto the chest: 0.6 m in 0.3 s, gravity compensated.
    w.set_velocity(ball, Vec3::new(0.0, 9.81 * 0.15, 2.0), Vec3::ZERO);
    let (mut hits, mut forces) = (Vec::new(), Vec::new());
    for _ in 0..90 {
        walker.frame(&mut w, Vec3::ZERO);
        for e in w.drain_events() {
            if let PhysicsEvent::ContactForce { a, b, impulse, .. } = e
                && (a == walker.capsule || b == walker.capsule)
            {
                forces.push(impulse);
            }
            hits.extend(capsule_hit(e, walker.capsule, ball));
        }
    }
    // The contact-force measure the app used before (per-step solver impulses over 200 N).
    eprintln!("soft ball: contact-force impulses {forces:?}, hits {hits:?}");
    assert_eq!(hits.len(), 1, "one hit: {hits:?}");
    assert!((5.0..=6.5).contains(&hits[0].0), "about 3 kg × 2 m/s: {hits:?}");
}

/// A ball chasing a walker that walks away at 1.5 m/s, thrown 2 m/s faster
/// (the app's aimed throw with lead): the hit is the closing speed, about
/// 3 kg × 2 m/s, not the ball's 3.5 m/s ground speed.
#[test]
fn ball_chasing_a_walker_hits_with_the_closing_speed() {
    let (mut w, base) = world();
    let mut walker = Walker::new(&mut w, base);
    for _ in 0..30 {
        walker.frame(&mut w, Vec3::Z * 1.5);
    }
    let start = walker.loco.position.offset(Vec3::new(0.0, 1.2, -0.6));
    let ball = w.insert(&BodyDesc {
        ccd: true,
        mass: Some(3.0),
        ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(start), ShapeRef::Sphere(0.12), Layer::PROP)
    });
    w.set_velocity(ball, Vec3::new(0.0, 9.81 * 0.15, 3.5), Vec3::ZERO);
    let mut hits = Vec::new();
    for _ in 0..60 {
        walker.frame(&mut w, Vec3::Z * 1.5);
        hits.extend(w.drain_events().filter_map(|e| capsule_hit(e, walker.capsule, ball)));
    }
    // The walker is still accelerating (~1.2 m/s at the hit): closing ~2.3 m/s.
    assert_eq!(hits.len(), 1, "one hit: {hits:?}");
    assert!((5.5..=7.5).contains(&hits[0].0), "about 3 kg × 2.3 m/s closing, not 3.5 m/s: {hits:?}");
}

/// A soft ball thrown at a walker walking toward it: the push zone (wider
/// than the capsule) doesn't bat it away first; one hit with the ball's own
/// momentum, about 3 kg × 2 m/s (not the 3.5 m/s closing speed).
#[test]
fn ball_thrown_at_an_approaching_walker_is_one_soft_hit() {
    let (mut w, base) = world();
    let mut walker = Walker::new(&mut w, base);
    for _ in 0..60 {
        walker.frame(&mut w, Vec3::Z * 1.5);
    }
    let start = walker.loco.position.offset(Vec3::new(0.0, 1.2, 0.7));
    let ball = w.insert(&BodyDesc {
        ccd: true,
        mass: Some(3.0),
        ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(start), ShapeRef::Sphere(0.12), Layer::PROP)
    });
    w.set_velocity(ball, Vec3::new(0.0, 9.81 * 0.15, -2.0), Vec3::ZERO);
    let mut hits = Vec::new();
    for _ in 0..60 {
        walker.frame(&mut w, Vec3::Z * 1.5);
        hits.extend(w.drain_events().filter_map(|e| capsule_hit(e, walker.capsule, ball)));
    }
    assert!(!hits.is_empty(), "the ball hit");
    assert!((5.0..=6.5).contains(&hits[0].0), "about 3 kg × 2 m/s: {hits:?}");
}

/// A walker walking into a ball resting on the ground, and on until the ball
/// is wedged against a wall: never a hit, however hard the solver pushes
/// (the PH-10 72 N·s "knockdown" by a ball lying on the street).
#[test]
fn walking_into_a_resting_ball_is_no_hit() {
    let (mut w, base) = world();
    let wall = BodyDesc::new(BodyKind::Fixed, at(base.offset(Vec3::new(0.0, 1.0, 2.0))), ShapeRef::Box(Vec3::new(2.0, 1.0, 0.1)), Layer::STATIC);
    w.insert(&wall);
    // The app's `V` ball: 3 kg, 12 cm, CCD, a `PROP` (it blocks the mover, which pushes it).
    let ball = w.insert(&BodyDesc {
        ccd: true,
        mass: Some(3.0),
        ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, at(base.offset(Vec3::new(0.0, 0.12, 1.0))), ShapeRef::Sphere(0.12), Layer::PROP)
    });
    let mut walker = Walker::new(&mut w, base);
    let (mut hits, mut max_force) = (Vec::new(), 0.0_f32);
    for _ in 0..180 {
        walker.frame(&mut w, Vec3::Z * 1.5);
        for e in w.drain_events() {
            if let PhysicsEvent::ContactForce { a, b, impulse, .. } = e
                && (a == walker.capsule || b == walker.capsule)
            {
                max_force = max_force.max(impulse);
            }
            hits.extend(capsule_hit(e, walker.capsule, ball));
        }
    }
    // The contact-force measure the app used before (one step's solver impulse).
    eprintln!("walking into a resting ball: largest contact-force impulse {max_force:.1} N·s, hits {hits:?}");
    assert!(hits.iter().all(|h| h.0 < 1.0), "no hit from a ball the walker walks into: {hits:?}");
}

/// [`crate::impact_impulse`]: the incoming body's own approach speed, capped by the closing speed.
#[test]
fn impact_impulse_is_the_incoming_momentum() {
    let n = Vec3::Z;
    assert!((crate::impact_impulse(3.0, Vec3::Z * 2.0, Vec3::ZERO, n) - 6.0).abs() < 1e-6);
    // Walking into a resting or retreating body: nothing.
    assert!(crate::impact_impulse(3.0, Vec3::ZERO, -Vec3::Z * 1.5, n).abs() < 1e-6);
    assert!(crate::impact_impulse(3.0, -Vec3::Z * 0.5, -Vec3::Z * 1.5, n).abs() < 1e-6);
    // Running into a thrown ball: no harder than the throw; chased: the closing speed.
    assert!((crate::impact_impulse(3.0, Vec3::Z * 2.0, -Vec3::Z * 1.5, n) - 6.0).abs() < 1e-6);
    assert!((crate::impact_impulse(3.0, Vec3::Z * 2.0, Vec3::Z * 1.5, n) - 1.5).abs() < 1e-6);
    // Tangential motion doesn't count.
    assert!(crate::impact_impulse(3.0, Vec3::X * 9.0, Vec3::ZERO, n).abs() < 1e-6);
}

/// Running at 7 m/s into a 4 cm plank (a `PROP`, too heavy to push through
/// quickly): the walker never ends up past it.
#[test]
fn no_tunnelling_at_running_speed() {
    let (mut w, base) = world();
    let plank_at = base.offset(Vec3::new(0.0, 1.0, 1.5));
    let desc = BodyDesc::new(BodyKind::Dynamic { asleep: true }, at(plank_at), ShapeRef::Box(Vec3::new(1.5, 1.0, 0.02)), Layer::PROP);
    let plank = w.insert(&BodyDesc { mass: Some(60.0), ..desc });
    let mut walker = Walker::new(&mut w, base);
    for _ in 0..120 {
        walker.frame(&mut w, Vec3::Z * 7.0);
        let feet = base.delta_f32(walker.loco.position).z;
        let plank_z = base.delta_f32(w.pose(plank).unwrap().pos).z;
        assert!(feet < plank_z, "tunnelled: feet {feet}, plank {plank_z}");
    }
}
