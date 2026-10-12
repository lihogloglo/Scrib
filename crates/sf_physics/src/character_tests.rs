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
    for _ in 0..60 {
        walker.frame(&mut w, Vec3::ZERO);
        hit |= w.drain_events().any(|e| matches!(e, PhysicsEvent::ContactForce { a, b, .. } if a == walker.capsule || b == walker.capsule));
    }
    assert!(hit, "contact force on the capsule");
    let z = base.delta_f32(w.pose(ball).unwrap().pos).z;
    assert!(z < 0.0, "the ball bounced off, not through: z {z}");
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
