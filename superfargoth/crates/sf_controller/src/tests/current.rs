//! A water current carries a character (FL-5b): `Locomotion::current` drifts the body by its
//! horizontal part every step, swept like any other motion; the intent stays relative to the water.

use super::*;

/// The water's height in the swim tests, m (the swimmers start at 1 m, well under it).
const WATER: f32 = 5.0;

/// Steps `loco` `frames` times, refreshing the swim surface each step as the adapter does.
fn swim(world: &World, loco: &mut Locomotion, intent: LocomotionIntent, params: &MoverParams, frames: usize, mut each: impl FnMut(&Locomotion)) {
    let mut mover = KinematicMover::new();
    for _ in 0..frames {
        loco.mode = MoverMode::Swim { surface: WATER - xyz(loco.position).y };
        mover.step(MoveInput { loco: &mut *loco, intent, params, root: RootMotion::default() }, world, DT);
        each(loco);
    }
}

#[test]
fn a_current_carries_a_swimmer() {
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.0, 1.0, 0.0), 0.0);
    loco.current = Vec3::X;
    swim(&world, &mut loco, LocomotionIntent::IDLE, &params, 60, |_| {});
    let at = xyz(loco.position);
    assert!((at.x - 1.0).abs() < 1e-3 && at.z.abs() < 1e-6, "drifted to {at}");
    assert_eq!(loco.current, Vec3::X, "the mover never clears the current");
}

#[test]
fn a_wall_stops_the_drift() {
    let mut world = World::new();
    world.add_box(Vec3::new(1.0, 0.0, -10.0), Vec3::new(2.0, 8.0, 10.0));
    let params = MoverParams::default();
    let contact = 1.0 - params.radius - params.skin;
    let mut loco = Locomotion::new(p(0.0, 1.0, 0.0), 0.0);
    loco.current = Vec3::X;
    let mut xs = Vec::new();
    swim(&world, &mut loco, LocomotionIntent::IDLE, &params, 120, |l| xs.push(xyz(l.position).x));
    let at = xyz(loco.position);
    assert!(at.x > contact - 1e-3 && at.x <= contact + params.skin, "stopped at {at}, wall contact {contact}");
    assert!(xs.iter().all(|&x| x <= contact + params.skin), "passed the wall: {xs:?}");
    // It stays there: the last half second does not move.
    let rest = xs[xs.len() - 30];
    assert!(xs[xs.len() - 30..].iter().all(|&x| (x - rest).abs() < 1e-5), "{:?}", &xs[xs.len() - 30..]);
}

#[test]
fn intent_adds_to_the_current() {
    let world = World::new();
    let params = MoverParams::default();
    let intent = toward(Vec3::new(0.0, 0.0, 2.0));
    let frames = 120;
    let t = frames as f32 * DT;
    let run = |current: Vec3| {
        let mut loco = Locomotion::new(p(0.0, 1.0, 0.0), 0.0);
        loco.current = current;
        let mut tail = Vec::new();
        swim(&world, &mut loco, intent, &params, frames, |l| tail.push(xyz(l.position)));
        tail
    };
    let still = run(Vec3::ZERO);
    let carried = run(Vec3::new(0.8, 0.0, 0.0));
    let (a, b) = (still[frames - 1], carried[frames - 1]);
    assert!((b.x - a.x - 0.8 * t).abs() < 1e-3, "x {a} vs {b}");
    assert!((b.z - a.z).abs() < 1e-4, "z {a} vs {b}");
    // Settled: the last tick moves by (current.x, intent speed) x dt.
    let step = carried[frames - 1] - carried[frames - 2];
    assert!((step.x - 0.8 * DT).abs() < 1e-3 && (step.z - 2.0 * DT).abs() < 1e-3, "last tick {step}");
}

#[test]
fn a_current_carries_a_walker_too() {
    let world = World::new();
    let params = MoverParams::default();
    let mut loco = Locomotion::new(p(0.0, 0.0, 0.0), 0.0);
    loco.current = Vec3::new(0.5, 0.0, 0.0);
    walk(&world, &mut loco, LocomotionIntent::IDLE, &params, 60, |_| {});
    let at = xyz(loco.position);
    assert!((at.x - 0.5).abs() < 1e-3 && at.z.abs() < 1e-6, "walked to {at}");
    assert!(loco.grounded.is_some());
}

#[test]
fn the_current_is_not_saved() {
    let mut loco = Locomotion::new(p(0.5, 0.0, -2.0), 0.3);
    loco.mode = MoverMode::Swim { surface: 1.25 };
    loco.slide = Vec3::new(0.0, 0.0, 0.5);
    loco.current = Vec3::X;
    let bytes = postcard::to_allocvec(&loco).unwrap();
    let back = postcard::from_bytes::<Locomotion>(&bytes).unwrap();
    assert_eq!(back.current, Vec3::ZERO);
    assert_eq!(back, Locomotion { current: Vec3::ZERO, ..loco });
}
