#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names, clippy::type_complexity)]

use std::f32::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

use glam::{Affine3A, IVec3, Quat, Vec3};
use sf_controller::{CollisionBuilder, CollisionMesh, GroundProbe, ProbeQuery, StaticWorld, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};

use crate::{
    BodyDesc, BodyId, BodyKind, Drive, HeightShape, JointDesc, JointKind, JointSolver, Layer, MeshShape, PhysicsEvent,
    PhysicsParams, PhysicsWorld, ShapeRef,
};

pub(crate) fn at(pos: WorldPos) -> WorldTransform {
    WorldTransform::from_pos(pos)
}

/// A 40 m ground slab whose top is at `base`'s height.
pub(crate) fn ground(w: &mut PhysicsWorld, base: WorldPos) -> BodyId {
    let pose = at(base.offset(Vec3::new(0.0, -0.5, 0.0)));
    w.insert(&BodyDesc::fixed(pose, ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC))
}

/// `n` 0.4 m cubes stacked on the ground at `base`, touching.
pub(crate) fn stack(w: &mut PhysicsWorld, base: WorldPos, n: usize) -> Vec<BodyId> {
    (0..n)
        .map(|i| {
            let pose = at(base.offset(Vec3::new(0.0, 0.2 + 0.4 * i as f32, 0.0)));
            let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Box(Vec3::splat(0.2)), Layer::PROP);
            w.insert(&BodyDesc { mass: Some(5.0), ..desc })
        })
        .collect()
}

/// A 15-body humanoid ragdoll standing at `base` (feet on the ground):
/// pelvis, 2 spine, head, and per side upper arm, forearm, hand, thigh,
/// calf, foot. Capsules along each segment; balls at the shoulders, hips,
/// spine and neck, hinges at the elbows and knees.
pub(crate) struct Ragdoll {
    pub bodies: Vec<BodyId>,
    pub joints: Vec<(JointDesc, crate::JointId)>,
}

/// Extra solver iterations for ragdoll islands (physics.md "Stepping").
pub(crate) const RAGDOLL_ITERATIONS: usize = 4;

pub(crate) fn ragdoll(w: &mut PhysicsWorld, base: WorldPos, solver: JointSolver, drive: Option<Drive>) -> Ragdoll {
    ragdoll_with(w, base, solver, drive, RAGDOLL_ITERATIONS, RAGDOLL_MIN_MASS)
}

/// Lightest ragdoll body, kg: hands and feet are raised toward their neighbors' mass (solver mass ratios).
pub(crate) const RAGDOLL_MIN_MASS: f32 = 2.0;

pub(crate) fn ragdoll_with(
    w: &mut PhysicsWorld,
    base: WorldPos,
    solver: JointSolver,
    drive: Option<Drive>,
    iterations: usize,
    min_mass: f32,
) -> Ragdoll {
    // (parent, start, end, radius); model space, feet at y 0, facing -Z.
    let segs: [(Option<usize>, [f32; 3], [f32; 3], f32); 15] = [
        (None, [0.0, 0.95, 0.0], [0.0, 1.1, 0.0], 0.12),            // 0 pelvis
        (Some(0), [0.0, 1.1, 0.0], [0.0, 1.3, 0.0], 0.12),          // 1 spine
        (Some(1), [0.0, 1.3, 0.0], [0.0, 1.5, 0.0], 0.13),          // 2 chest
        (Some(2), [0.0, 1.55, 0.0], [0.0, 1.75, 0.0], 0.1),         // 3 head
        (Some(2), [0.2, 1.45, 0.0], [0.48, 1.45, 0.0], 0.05),       // 4 upper arm L
        (Some(4), [0.48, 1.45, 0.0], [0.74, 1.45, 0.0], 0.045),     // 5 forearm L
        (Some(5), [0.74, 1.45, 0.0], [0.86, 1.45, 0.0], 0.04),      // 6 hand L
        (Some(2), [-0.2, 1.45, 0.0], [-0.48, 1.45, 0.0], 0.05),     // 7 upper arm R
        (Some(7), [-0.48, 1.45, 0.0], [-0.74, 1.45, 0.0], 0.045),   // 8 forearm R
        (Some(8), [-0.74, 1.45, 0.0], [-0.86, 1.45, 0.0], 0.04),    // 9 hand R
        (Some(0), [0.1, 0.92, 0.0], [0.1, 0.5, 0.0], 0.07),         // 10 thigh L
        (Some(10), [0.1, 0.5, 0.0], [0.1, 0.1, 0.0], 0.055),        // 11 calf L
        (Some(11), [0.1, 0.05, 0.05], [0.1, 0.05, -0.15], 0.045),   // 12 foot L
        (Some(0), [-0.1, 0.92, 0.0], [-0.1, 0.5, 0.0], 0.07),       // 13 thigh R
        (Some(13), [-0.1, 0.5, 0.0], [-0.1, 0.1, 0.0], 0.055),      // 14 calf R
    ];
    let mut bodies = Vec::new();
    let mut frames = Vec::new();
    for (_, s, e, r) in &segs {
        let (s, e) = (Vec3::from_array(*s), Vec3::from_array(*e));
        let axis = e - s;
        let rot = Quat::from_rotation_arc(Vec3::Y, axis.normalize());
        let center = (s + e) * 0.5;
        let half = (axis.length() * 0.5 - r).max(0.01);
        let pose = WorldTransform { rotation: rot, ..at(base.offset(center)) };
        let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Capsule { r: *r, half }, Layer::RAGDOLL);
        let volume = PI * r * r * (2.0 * half + 4.0 / 3.0 * r);
        let mass = Some((volume * 1000.0).max(min_mass));
        bodies.push(w.insert(&BodyDesc { extra_iterations: iterations, mass, ..desc }));
        frames.push((center, rot));
    }
    let mut joints = Vec::new();
    for (i, (parent, s, _, _)) in segs.iter().enumerate() {
        let Some(p) = *parent else { continue };
        let pivot = Vec3::from_array(*s);
        let local = |(c, r): (Vec3, Quat)| (r.inverse() * (pivot - c), r.inverse());
        let kind = match i {
            5 | 8 => JointKind::Hinge { range: [-150f32.to_radians(), 0.0] },
            11 | 14 => JointKind::Hinge { range: [0.0, 150f32.to_radians()] },
            6 | 9 | 12 => JointKind::Ball { swing: [0.6, 0.6], twist: [-0.3, 0.3] },
            4 | 7 | 10 | 13 => JointKind::Ball { swing: [80f32.to_radians(), 60f32.to_radians()], twist: [-0.7, 0.7] },
            _ => JointKind::Ball { swing: [0.5, 0.5], twist: [-0.5, 0.5] },
        };
        let desc = JointDesc {
            parent: bodies[p],
            child: bodies[i],
            frame_parent: local(frames[p]),
            frame_child: local(frames[i]),
            kind,
            contacts: false,
        };
        let id = w.insert_joint(&desc, solver).expect("tree joint");
        if drive.is_some() {
            w.set_joint_drive(id, [0.0; 3], drive);
        }
        joints.push((desc, id));
    }
    Ragdoll { bodies, joints }
}

fn positions(w: &PhysicsWorld, ids: &[BodyId]) -> Vec<WorldPos> {
    ids.iter().map(|&b| w.pose(b).unwrap().pos).collect()
}

fn count(events: &[PhysicsEvent], f: impl Fn(&PhysicsEvent) -> bool) -> usize {
    events.iter().filter(|e| f(e)).count()
}

#[test]
fn layers_are_symmetric() {
    for a in Layer::all().iter() {
        for b in Layer::all().iter() {
            assert_eq!(a.collides_with().contains(b), b.collides_with().contains(a), "{a:?} vs {b:?}");
        }
    }
    assert!(!Layer::STATIC.collides_with().contains(Layer::TERRAIN));
    assert!(Layer::DEBRIS.collides_with().contains(Layer::CHARACTER));
}

/// The rebase gate: a 30-body stack, a powered ragdoll hung from its pelvis
/// and sleeping items, stepped with and without a mid-run rebase: the step
/// across the rebase within 0.1 mm for every body, the stack within 0.1 mm
/// at the end, and the sleeping items never wake.
#[test]
fn rebase_matches_unrebased_run() {
    // Bodies two cells (≈ 512 m) from the initial anchor; the rebase moves the anchor onto them.
    let base = WorldPos::new(IVec3::new(5, 0, 3), Vec3::new(10.3, 0.0, -7.1));
    let far_anchor = WorldPos::new(IVec3::new(3, 0, 3), Vec3::ZERO);
    let run = |rebase: bool| {
        let mut w = PhysicsWorld::new(far_anchor, PhysicsParams::default());
        ground(&mut w, base);
        let mut ids = stack(&mut w, base, 30);
        let rd = ragdoll(&mut w, base.offset(Vec3::new(3.0, 0.3, 0.0)), JointSolver::Multibody, Some(Drive::critical(25.0)));
        // A falling ragdoll (or one in a limit cycle against its joint limits) is chaotic: float noise
        // grows to centimeters with or without a rebase. Hang it from the pelvis, kick it, and let the
        // drives bring it back to its pose.
        let pelvis = w.pose(rd.bodies[0]).unwrap();
        let hook = w.insert(&BodyDesc::fixed(pelvis, ShapeRef::Sphere(0.01), Layer::empty()));
        let weld = JointDesc { parent: hook, child: rd.bodies[0], frame_parent: (Vec3::ZERO, Quat::IDENTITY), frame_child: (Vec3::ZERO, Quat::IDENTITY), kind: JointKind::Fixed, contacts: false };
        w.insert_joint(&weld, JointSolver::Impulse).unwrap();
        w.set_velocity(rd.bodies[6], Vec3::new(0.0, 1.0, -2.0), Vec3::ZERO);
        w.set_velocity(rd.bodies[3], Vec3::new(1.0, 0.0, 0.5), Vec3::ZERO);
        w.set_velocity(rd.bodies[11], Vec3::new(0.0, 0.0, 1.5), Vec3::ZERO);
        ids.extend(&rd.bodies);
        let sleepers: Vec<BodyId> = (0..10)
            .map(|i| {
                let p = base.offset(Vec3::new(-3.0 - i as f32 * 0.5, 0.1, 2.0));
                w.insert(&BodyDesc::item(at(p), ShapeRef::Box(Vec3::splat(0.1)), Layer::DEBRIS, Some(1.0)))
            })
            .collect();
        let mut events = Vec::new();
        let mut after_rebase = Vec::new();
        for step in 0..240 {
            if rebase && step == 120 {
                w.rebase(base.cell);
                assert_eq!(w.anchor().cell, base.cell);
            }
            w.step();
            if step == 120 {
                after_rebase = positions(&w, &ids);
            }
            events.extend(w.drain_events());
        }
        let woke = count(&events, |e| matches!(e, PhysicsEvent::Woke(b) if sleepers.contains(b)));
        (positions(&w, &ids), after_rebase, woke)
    };
    let (a, a1, woke_a) = run(false);
    let (b, b1, woke_b) = run(true);
    assert_eq!((woke_a, woke_b), (0, 0), "sleeping items woke");
    let worst = |a: &[WorldPos], b: &[WorldPos]| a.iter().zip(b).map(|(p, q)| p.delta(*q).length()).fold(0.0, f64::max);
    // The step across the rebase moves nothing beyond float noise, for every body.
    let step = worst(&a1, &b1);
    assert!(step <= 1e-4, "the rebase step diverged by {step} m");
    // Contact-only content stays exact over the whole run (measured: bit-identical). The powered
    // ragdoll is excluded at the end: its drives don't settle yet (P8 owns drive targets), so it
    // amplifies any perturbation, including the engine's joint-solver reset on moved bodies.
    let stack = worst(&a[..30], &b[..30]);
    assert!(stack <= 1e-4, "rebased stack diverged by {stack} m");
}

#[test]
fn placed_items_stay_asleep_and_exact() {
    let base = WorldPos::new(IVec3::new(-2, 0, 7), Vec3::new(1.0, 12.0, 3.0));
    let mut w = PhysicsWorld::new(base, PhysicsParams::default());
    ground(&mut w, base);
    // Some sit 5 mm into the ground, some hover 2 mm, some stack on each other.
    let mut items = Vec::new();
    for i in 0..50 {
        let x = (i % 10) as f32 * 0.6 - 3.0;
        let z = (i / 10) as f32 * 0.6 - 1.5;
        let y = 0.1 + [-0.005, 0.0, 0.002][i % 3];
        let shape = if i % 2 == 0 { ShapeRef::Box(Vec3::splat(0.1)) } else { ShapeRef::Sphere(0.1) };
        items.push(w.insert(&BodyDesc::item(at(base.offset(Vec3::new(x, y, z))), shape, Layer::DEBRIS, Some(0.5))));
        if i % 7 == 0 {
            let top = base.offset(Vec3::new(x, y + 0.2, z));
            items.push(w.insert(&BodyDesc::item(at(top), ShapeRef::Box(Vec3::splat(0.1)), Layer::DEBRIS, Some(0.5))));
        }
    }
    let before = positions(&w, &items);
    let mut events = Vec::new();
    for _ in 0..600 {
        w.step();
        events.extend(w.drain_events());
    }
    assert_eq!(count(&events, |e| matches!(e, PhysicsEvent::Woke(_))), 0);
    assert_eq!(positions(&w, &items), before);
    assert_eq!(w.awake_count(), 0);
}

#[test]
fn knocked_item_wakes_rests_and_interpolates() {
    let base = WorldPos::ORIGIN;
    let mut w = PhysicsWorld::new(base, PhysicsParams::default());
    ground(&mut w, base);
    let item = w.insert(&BodyDesc::item(at(base.offset(Vec3::Y * 0.1)), ShapeRef::Box(Vec3::splat(0.1)), Layer::DEBRIS, Some(1.0)));
    assert_eq!(w.is_sleeping(item), Some(true));
    w.apply_impulse(item, Vec3::new(2.0, 1.0, 0.0), None);
    let mut events = Vec::new();
    let mut rested = None;
    for step in 0..180 {
        w.step();
        if step == 3 {
            let (p0, p1) = (w.pose_at(item, 0.0).unwrap().pos, w.pose_at(item, 1.0).unwrap().pos);
            assert_eq!(p1, w.pose(item).unwrap().pos);
            assert!(p0.delta(p1).length() > 1e-3, "interpolation endpoints differ while moving");
        }
        events.extend(w.drain_events());
        if rested.is_none() && events.contains(&PhysicsEvent::Slept(item)) {
            rested = Some(step);
        }
    }
    assert_eq!(events.first(), Some(&PhysicsEvent::Woke(item)));
    assert!(rested.is_some(), "knocked item still moving after 3 s");
    assert!(w.pose(item).unwrap().pos.local.x > 0.5);
    assert_eq!(w.pose_at(item, 0.3), w.pose(item), "resting bodies don't interpolate");
}

#[test]
fn ids_are_generational() {
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let a = w.insert(&BodyDesc::fixed(WorldTransform::IDENTITY, ShapeRef::Sphere(1.0), Layer::STATIC).with_user(7));
    assert_eq!(w.user(a), Some(7));
    assert!(w.remove(a));
    assert!(!w.remove(a));
    let b = w.insert(&BodyDesc::fixed(WorldTransform::IDENTITY, ShapeRef::Sphere(1.0), Layer::STATIC));
    assert_ne!(a, b);
    assert!(w.pose(a).is_none() && w.pose(b).is_some());
    assert_eq!(w.len(), 1);
}

#[test]
fn fallen_body_returns_to_rest() {
    let base = WorldPos::new(IVec3::new(0, 1, 0), Vec3::ZERO);
    let params = PhysicsParams { kill_height: Some(200.0), ..PhysicsParams::default() };
    let mut w = PhysicsWorld::new(base, params);
    let item = w.insert(&BodyDesc::item(at(base), ShapeRef::Sphere(0.1), Layer::DEBRIS, Some(1.0)));
    w.wake(item);
    let mut events = Vec::new();
    for _ in 0..600 {
        w.step();
        events.extend(w.drain_events());
    }
    assert!(events.contains(&PhysicsEvent::FellOut(item)));
    assert_eq!(w.pose(item).unwrap().pos, base);
    assert_eq!(w.is_sleeping(item), Some(true));
}

#[test]
fn heightfield_matches_its_samples() {
    let n = 17;
    let spacing = 0.5;
    let h = |x: usize, z: usize| x as f32 * 0.1 + z as f32 * 0.03 + 5.0;
    let heights: Vec<f32> = (0..n * n).map(|i| h(i % n, i / n)).collect();
    let shape = HeightShape::new(n, spacing, &heights).unwrap();
    let center = WorldPos::new(IVec3::new(2, 0, -1), Vec3::new(4.0, 0.0, 4.0));
    let mut w = PhysicsWorld::new(center, PhysicsParams::default());
    w.insert(&BodyDesc::fixed(at(center), ShapeRef::Heightfield(shape.clone()), Layer::TERRAIN));
    w.refresh_queries();
    let min = center.offset(Vec3::new(-shape.extent() / 2.0, 0.0, -shape.extent() / 2.0));
    for (x, z) in [(0, 0), (3, 11), (16, 2), (9, 9)] {
        let p = min.offset(Vec3::new(x as f32 * spacing, 50.0, z as f32 * spacing));
        let hit = w.probe(Layer::TERRAIN).probe_one(ProbeQuery::down(p, 100.0)).expect("hit terrain");
        let y = hit.point.to_f64().y as f32;
        assert!((y - h(x, z)).abs() < 1e-3, "({x}, {z}): {y} vs {}", h(x, z));
    }
}

fn room_mesh() -> CollisionMesh {
    // A 6 × 3 × 6 open-top room (floor surface 1, walls 2) and a ramp (surface 3).
    let mut b = CollisionBuilder::new(1e-4);
    let quad = |b: &mut CollisionBuilder, p: [[f32; 3]; 4], s: u16| b.add(&p, &[0, 1, 2, 0, 2, 3], Affine3A::IDENTITY, SurfaceId(s));
    quad(&mut b, [[-3.0, 0.0, -3.0], [-3.0, 0.0, 3.0], [3.0, 0.0, 3.0], [3.0, 0.0, -3.0]], 1);
    quad(&mut b, [[-3.0, 0.0, -3.0], [3.0, 0.0, -3.0], [3.0, 3.0, -3.0], [-3.0, 3.0, -3.0]], 2);
    quad(&mut b, [[3.0, 0.0, -3.0], [3.0, 0.0, 3.0], [3.0, 3.0, 3.0], [3.0, 3.0, -3.0]], 2);
    quad(&mut b, [[-2.0, 0.0, 2.0], [0.0, 1.2, 2.0], [0.0, 1.2, 0.0], [-2.0, 0.0, 0.0]], 3);
    b.finish()
}

/// `PhysicsProbe` answers like `StaticWorld` on the same placements.
#[test]
fn physics_probe_matches_static_world() {
    let mesh = room_mesh();
    let anchor = WorldPos::new(IVec3::new(1, 0, 1), Vec3::ZERO);
    let placements = [
        WorldTransform { pos: anchor.offset(Vec3::new(0.0, 0.0, 0.0)), rotation: Quat::IDENTITY, scale: Vec3::ONE },
        WorldTransform { pos: anchor.offset(Vec3::new(12.0, 0.5, 0.0)), rotation: Quat::from_rotation_y(0.7), scale: Vec3::splat(1.5) },
        WorldTransform {
            pos: anchor.offset(Vec3::new(0.0, 0.0, 12.0)),
            rotation: Quat::from_rotation_y(-FRAC_PI_2) * Quat::from_rotation_x(0.05),
            scale: Vec3::new(1.0, 0.8, 1.3),
        },
    ];
    let mut statics = StaticWorld::new(anchor);
    let shape = Arc::new(sf_controller::StaticShape::new(&mesh).unwrap());
    let mut w = PhysicsWorld::new(anchor, PhysicsParams::default());
    let ms = MeshShape::new(&mesh).unwrap();
    for t in placements {
        statics.insert(&shape, t);
        w.insert(&BodyDesc::fixed(t, ShapeRef::Mesh(ms.clone()), Layer::STATIC));
    }
    w.refresh_queries();
    let probe = w.probe(Layer::WORLD);
    let mut queries = Vec::new();
    for i in 0..15 {
        for j in 0..15 {
            let o = anchor.offset(Vec3::new(i as f32 * 1.1 - 4.0, 4.0, j as f32 * 1.1 - 4.0));
            queries.push(ProbeQuery::down(o, 10.0));
            queries.push(ProbeQuery::sphere(o, Vec3::NEG_Y, 10.0, 0.3));
            queries.push(ProbeQuery::capsule(o, Vec3::NEG_Y, 10.0, 0.3, 0.5));
            let side = o.offset(Vec3::new(0.0, -3.0, 0.0));
            let dir = Vec3::new((i as f32 * 0.7).cos(), 0.0, (j as f32 * 0.9).sin()).normalize();
            queries.push(ProbeQuery::ray(side, dir, 20.0));
            queries.push(ProbeQuery::sphere(side, dir, 20.0, 0.25));
        }
    }
    let mut a = vec![None; queries.len()];
    let mut b = vec![None; queries.len()];
    statics.probe(&queries, &mut a);
    probe.probe(&queries, &mut b);
    let mut hits = 0;
    for ((q, a), b) in queries.iter().zip(&a).zip(&b) {
        match (a, b) {
            (None, None) => {}
            (Some(a), Some(b)) => {
                hits += 1;
                // Rays agree to float noise; sweeps are two GJK/TOI paths (StaticWorld casts per mesh).
                let tol = if q.radius > 0.0 { 5e-3 } else { 1e-3 };
                assert!((a.distance - b.distance).abs() < tol, "{q:?}: {a:?} vs {b:?}");
                assert!(a.normal.dot(b.normal) > 0.999 || (q.radius > 0.0 && a.normal.dot(b.normal) > 0.95), "{q:?}: {a:?} vs {b:?}");
                // A cast that starts inside a surface has no well-defined witness point.
                assert!(a.distance < 1e-4 || a.point.delta(b.point).length() < 2e-2, "{q:?}: {a:?} vs {b:?}");
                if q.radius <= 0.0 {
                    assert_eq!(a.surface, b.surface, "{q:?}");
                }
            }
            _ => panic!("{q:?}: static {a:?} vs physics {b:?}"),
        }
    }
    assert!(hits > 500, "fixture too sparse: {hits} hits");
}

/// Limp ragdolls of both solvers fall and settle without joint stretch.
#[test]
fn limp_ragdoll_chains_hold_together() {
    let mut report = Vec::new();
    for (solver, iterations, min_mass) in [
        (JointSolver::Impulse, RAGDOLL_ITERATIONS, RAGDOLL_MIN_MASS),
        (JointSolver::Multibody, RAGDOLL_ITERATIONS, RAGDOLL_MIN_MASS),
        (JointSolver::Impulse, RAGDOLL_ITERATIONS, 0.0),
        (JointSolver::Impulse, 0, RAGDOLL_MIN_MASS),
        (JointSolver::Impulse, 12, RAGDOLL_MIN_MASS),
        (JointSolver::Multibody, 0, 0.0),
    ] {
        let base = WorldPos::ORIGIN;
        let mut w = PhysicsWorld::new(base, PhysicsParams::default());
        ground(&mut w, base);
        let rd = ragdoll_with(&mut w, base.offset(Vec3::Y * 0.5), solver, None, iterations, min_mass);
        let initial = rd.joints.iter().map(|(d, _)| w.joint_stretch(d).unwrap()).fold(0.0, f32::max);
        assert!(initial < 1e-5, "joint frames disagree at rest: {initial}");
        w.set_velocity(rd.bodies[2], Vec3::new(1.0, 0.0, 2.0), Vec3::new(0.0, PI, 0.0));
        let mut worst: f32 = 0.0;
        for _ in 0..240 {
            w.step();
            for (d, _) in &rd.joints {
                worst = worst.max(w.joint_stretch(d).unwrap());
            }
        }
        let pelvis = w.pose(rd.bodies[0]).unwrap().pos.to_f64().y;
        assert!(pelvis < 0.4, "{solver:?}: ragdoll didn't fall ({pelvis})");
        report.push((solver, iterations, min_mass, worst));
    }
    println!("joint stretch (solver, extra iterations, min mass, worst m): {report:?}");
    // Multibodies (reduced coordinates) meet the P7 gate (no stretch > 1 cm); impulse joints only hold
    // together (they stretch a few cm on impact at these mass ratios: physics.md open question 1).
    assert!(report.iter().all(|r| r.3 < if r.0 == JointSolver::Multibody { 0.01 } else { 0.06 }), "joint stretch (m): {report:?}");
}

/// The P1 spike: timings against the physics.md gates. Release only:
/// `cargo test -p sf_physics --release -- --ignored --nocapture spike`.
#[test]
#[ignore = "benchmark"]
fn spike() {
    crate::spike::run();
}
