//! P7: `ragdoll.cook` on the fixture rigs; limp drops, stairs and a corpse
//! dragged by the wrist against the 1 cm joint-stretch gate (physics.md
//! "Verification").

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use glam::{Affine3A, IVec3, Quat, Vec3};
use sf_anim::{RigHints, Skeleton, Trs, detect_profile, fixtures};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, CarryEnd, Carrier, GrabMode, GrabResult, HolderLimits, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

use crate::cook::segment_distance_for_tests;
use crate::{CookParams, RagJointKind, RagdollDef, RagdollInstance, RagdollSettings, SpawnPose, cook};

fn cooked(skeleton: &Skeleton, params: &CookParams) -> RagdollDef {
    let profile = detect_profile(skeleton, &RigHints::default()).expect("profile");
    cook(skeleton, &profile, params).expect("cook")
}

fn human() -> (Skeleton, RagdollDef) {
    let s = fixtures::biped();
    let d = cooked(&s, &CookParams { total_mass: Some(75.0), ..CookParams::default() });
    (s, d)
}

fn world() -> (PhysicsWorld, WorldPos) {
    let base = WorldPos::new(IVec3::new(2, 0, -3), Vec3::new(10.0, 40.0, -20.0));
    let mut w = PhysicsWorld::new(base, PhysicsParams::default());
    let slab = BodyDesc::fixed(WorldTransform::from_pos(base.offset(Vec3::new(0.0, -0.5, 0.0))), ShapeRef::Box(Vec3::new(20.0, 0.5, 20.0)), Layer::STATIC);
    w.insert(&slab);
    (w, base)
}

fn rest_spawn(w: &mut PhysicsWorld, s: &Skeleton, d: &RagdollDef, root: WorldTransform, settings: &RagdollSettings) -> RagdollInstance {
    let model = s.rest_model();
    RagdollInstance::spawn(w, d, &SpawnPose { root, scale: 1.0, model: &model, previous: None, root_velocity: Vec3::ZERO }, settings)
}

/// Lying on its back: the rest pose turned face up, `height` above `base`.
fn lying(base: WorldPos, height: f32) -> WorldTransform {
    WorldTransform { pos: base.offset(Vec3::new(0.0, height, 0.0)), rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), scale: Vec3::ONE }
}

#[test]
fn fixture_rigs_cook() {
    for (name, s) in [("biped", fixtures::biped()), ("quadruped", fixtures::quadruped(1.0)), ("hexapod", fixtures::hexapod()), ("flyer", fixtures::flyer())] {
        let d = cooked(&s, &CookParams::default());
        let hinges = d.joints.iter().filter(|j| matches!(j.kind, RagJointKind::Hinge { .. })).count();
        let names: Vec<&str> = d.bodies.iter().map(|b| s.name(usize::from(b.joint))).collect();
        println!("{name}: {} bodies, {} joints ({hinges} hinges), {:.1} kg, regions {:?}, no-collide {}: {names:?}", d.bodies.len(), d.joints.len(), d.total_mass, d.regions, d.no_collide.len());
        assert_eq!(d.joints.len(), d.bodies.len() - 1, "{name}: a tree");
        assert!(d.bodies.len() >= 5, "{name}");
        for (i, b) in d.bodies.iter().enumerate().skip(1) {
            assert!(usize::from(b.parent.unwrap()) < i, "{name}: parents first");
        }
        let heaviest = d.bodies.iter().map(|b| b.mass).fold(0.0, f32::max);
        for b in &d.bodies[1..] {
            assert!(b.mass * 4.0 >= d.bodies[usize::from(b.parent.unwrap())].mass * 0.999, "{name}: mass ratio");
            assert!(b.mass > 0.0 && b.mass <= heaviest);
        }
        assert_eq!(RagdollDef::from_bytes(&d.to_bytes()).unwrap(), d);
    }
    let (s, d) = human();
    assert!((d.total_mass - 75.0).abs() < 1e-3);
    let body = |n: &str| d.body_of_joint(u16::try_from(s.find(n).unwrap()).unwrap()).unwrap();
    for knee in ["shin_l", "shin_r", "forearm_l", "forearm_r"] {
        let j = d.joints.iter().find(|j| usize::from(j.child) == body(knee)).unwrap();
        assert!(matches!(j.kind, RagJointKind::Hinge { .. }), "{knee}: {:?}", j.kind);
    }
}

/// Bends a hinge by +angle in the rest pose: the shin must swing backward
/// (knees bend back), the forearm forward.
#[test]
fn hinges_bend_the_right_way() {
    let (s, d) = human();
    for (child, expect_z) in [("shin_l", -1.0f32), ("forearm_l", 1.0)] {
        let b = d.body_of_joint(u16::try_from(s.find(child).unwrap()).unwrap()).unwrap();
        let j = d.joints.iter().find(|j| usize::from(j.child) == b).unwrap();
        let body = &d.bodies[b];
        let rest = s.rest_model();
        let jr = rest[usize::from(body.joint)].to_scale_rotation_translation().1;
        // Frame in model space and the bone direction.
        let frame = jr * body.local.1 * j.frame_child.1;
        let axis = frame * Vec3::X;
        let bone = jr * body.local.1 * Vec3::Y;
        let bent = Quat::from_axis_angle(axis, 0.5) * bone;
        assert!((bent - bone).z * expect_z > 0.05, "{child}: bone {bone} bends to {bent}");
    }
}

/// The local pose read back right after spawning at rest is the rest pose.
#[test]
fn readback_at_rest_is_the_rest_pose() {
    let (s, d) = human();
    let (mut w, base) = world();
    let root = WorldTransform { pos: base.offset(Vec3::new(1.0, 0.0, 2.0)), rotation: Quat::from_rotation_y(0.7), scale: Vec3::ONE };
    let r = rest_spawn(&mut w, &s, &d, root, &RagdollSettings::default());
    let mut out = Vec::new();
    assert!(r.local_pose(&w, &d, &s, &root, 1.0, s.rest(), &mut out));
    for (j, (a, b)) in out.iter().zip(s.rest()).enumerate() {
        assert!(a.translation.distance(b.translation) < 1e-3, "{}: {} vs {}", s.name(j), a.translation, b.translation);
        assert!(a.rotation.angle_between(b.rotation) < 1e-3, "{}", s.name(j));
    }
    r.remove(&mut w);
    assert_eq!(w.len(), 1);
}

struct Run {
    stretch: f32,
    speed: f32,
    lowest: f32,
    asleep_at: Option<f32>,
}

fn simulate(w: &mut PhysicsWorld, r: &RagdollInstance, base: WorldPos, seconds: f32) -> Run {
    let mut run = Run { stretch: 0.0, speed: 0.0, lowest: f32::MAX, asleep_at: None };
    let steps = (seconds * 60.0) as usize;
    for i in 0..steps {
        w.step();
        run.stretch = run.stretch.max(r.max_stretch(w));
        for &b in &r.bodies {
            run.speed = run.speed.max(w.velocity(b).unwrap().0.length());
            run.lowest = run.lowest.min(base.delta_f32(w.pose(b).unwrap().pos).y);
        }
        if run.asleep_at.is_none() && r.asleep(w) {
            run.asleep_at = Some(i as f32 / 60.0);
        }
    }
    run
}

/// A 75 kg ragdoll dropped flat from 0.5 m: no joint stretches more than
/// 1 cm, nothing explodes, nothing sinks into the floor, it sleeps.
#[test]
fn limp_drop() {
    let (s, d) = human();
    let (mut w, base) = world();
    let r = rest_spawn(&mut w, &s, &d, lying(base, 0.65), &RagdollSettings::default());
    let run = simulate(&mut w, &r, base, 5.0);
    println!("limp drop 0.5 m: max stretch {:.2} cm, top speed {:.2} m/s, lowest body center {:.3} m, asleep at {:?} s", run.stretch * 100.0, run.speed, run.lowest, run.asleep_at);
    assert!(run.stretch <= 0.01, "stretch {}", run.stretch);
    assert!(run.speed < 6.0, "explosion: {} m/s", run.speed);
    assert!(run.lowest > 0.0, "sank: {}", run.lowest);
    assert!(run.asleep_at.is_some(), "never slept");
}

/// Standing upright at the top of a flight of stairs (0.18 m risers, 0.3 m
/// treads), pushed forward: it tumbles down, settles, and its joints hold.
#[test]
fn settles_on_stairs() {
    let (s, d) = human();
    let (mut w, base) = world();
    for i in 0..8 {
        let top = 0.18 * (8 - i) as f32;
        let z = -0.3 * i as f32;
        let step = BodyDesc::fixed(
            WorldTransform::from_pos(base.offset(Vec3::new(0.0, top * 0.5, z - 0.15))),
            ShapeRef::Box(Vec3::new(1.5, top * 0.5, 0.15)),
            Layer::STATIC,
        );
        w.insert(&step);
    }
    let root = WorldTransform { pos: base.offset(Vec3::new(0.0, 1.45, 0.3)), rotation: Quat::IDENTITY, scale: Vec3::ONE };
    let model = s.rest_model();
    let previous: Vec<Affine3A> = model.iter().map(|m| Affine3A::from_translation(Vec3::new(0.0, 0.0, 0.03)) * *m).collect();
    let pose = SpawnPose { root, scale: 1.0, model: &model, previous: Some((&previous, 1.0 / 60.0)), root_velocity: Vec3::ZERO };
    let r = RagdollInstance::spawn(&mut w, &d, &pose, &RagdollSettings::default());
    let run = simulate(&mut w, &r, base, 8.0);
    let pelvis = base.delta_f32(r.root_pose(&w, 1.0).unwrap().pos);
    println!("stairs: max stretch {:.2} cm, top speed {:.2} m/s, pelvis ends at {pelvis}, asleep at {:?} s", run.stretch * 100.0, run.speed, run.asleep_at);
    assert!(run.stretch <= 0.01, "stretch {}", run.stretch);
    assert!(run.speed < 8.0, "explosion: {} m/s", run.speed);
    assert!(pelvis.y < 1.2, "it fell: {pelvis}");
    assert!(run.asleep_at.is_some(), "never settled");
}

/// Grabs the left hand of a 75 kg corpse lying on the floor and walks off
/// at the drag speed for 3 s: the grab is a drag (the whole ragdoll's mass
/// decides), the body follows, no joint stretches more than 1 cm.
fn drag_by_wrist(settings: &RagdollSettings, d: &RagdollDef, s: &Skeleton) -> (f32, f32, Vec<CarryEnd>) {
    let (mut w, base) = world();
    let r = rest_spawn(&mut w, s, d, lying(base, 0.2), settings);
    for _ in 0..90 {
        w.step();
    }
    let hand = r.bodies[d.body_of_joint(u16::try_from(s.find("hand_l").unwrap()).unwrap()).unwrap()];
    let grip = w.pose(hand).unwrap().pos;
    let limits = HolderLimits { carry_force: 240.0, carry_mass: 15.0, drag_mass: 120.0, throw_impulse: 6.0 };
    let mut c = Carrier::new(limits);
    let eye = grip.offset(Vec3::new(1.0, 1.6, 0.0));
    let look = |at: WorldPos, to: WorldPos| {
        let dir = at.delta_f32(to).normalize();
        WorldTransform { pos: at, rotation: Quat::from_rotation_arc(-Vec3::Z, dir), scale: Vec3::ONE }
    };
    c.record_view(0.0, look(eye, grip));
    match c.grab_body(&mut w, hand, grip) {
        GrabResult::Grabbed { mode, mass, .. } => {
            assert_eq!(mode, GrabMode::Drag);
            assert!((mass - 75.0).abs() < 0.5, "mode mass {mass}");
        }
        other => panic!("{other:?}"),
    }
    let pelvis0 = r.root_pose(&w, 1.0).unwrap().pos;
    let speed = c.speed_scale();
    let mut ends = Vec::new();
    let mut stretch: f32 = 0.0;
    let dir = Vec3::X;
    let start_view = look(eye, grip);
    for i in 1..=180 {
        let t = f64::from(i) / 60.0;
        let view = WorldTransform { pos: eye.offset(dir * (speed * t as f32)), ..start_view };
        c.record_view(t, view);
        ends.extend(c.before_step(&mut w, t));
        w.step();
        ends.extend(c.after_step(&mut w));
        stretch = stretch.max(r.max_stretch(&w));
        if !ends.is_empty() {
            break;
        }
        assert!(!r.asleep(&w), "asleep while dragged at step {i}: hand {:?}, grab {:?}, ends {ends:?}", w.is_sleeping(hand), c.held());
    }
    let moved = pelvis0.delta_f32(r.root_pose(&w, 1.0).unwrap().pos).x;
    (stretch, moved, ends)
}

#[test]
fn corpse_dragged_by_the_wrist() {
    let (s, d) = human();
    let (stretch, moved, ends) = drag_by_wrist(&RagdollSettings::default(), &d, &s);
    println!("drag 75 kg by the wrist: max stretch {:.2} cm, pelvis moved {moved:.2} m", stretch * 100.0);
    assert!(ends.is_empty(), "{ends:?}");
    assert!(stretch <= 0.01, "stretch {stretch}");
    assert!(moved > 0.5, "the body followed {moved} m");
}

/// The P7 stretch table: iterations × mass balancing, for the drop and the drag.
#[test]
#[ignore = "measurement: cargo test -p sf_ragdoll --release -- --ignored --nocapture"]
fn stretch_table() {
    let s = fixtures::biped();
    for ratio in [1000.0, 4.0] {
        let d = cooked(&s, &CookParams { total_mass: Some(75.0), max_mass_ratio: ratio, ..CookParams::default() });
        let lightest = d.bodies.iter().map(|b| b.mass).fold(f32::MAX, f32::min);
        for iterations in [0, 4, 8, 12, 16] {
            let settings = RagdollSettings { extra_iterations: iterations, ..RagdollSettings::default() };
            let (mut w, base) = world();
            let r = rest_spawn(&mut w, &s, &d, lying(base, 0.65), &settings);
            let drop = simulate(&mut w, &r, base, 3.0);
            let (drag, moved, ends) = drag_by_wrist(&settings, &d, &s);
            println!(
                "mass ratio cap {ratio:>6} (lightest {lightest:.2} kg), +{iterations:>2} iterations: drop {:.2} cm, drag {:.2} cm (moved {moved:.2} m, ends {ends:?})",
                drop.stretch * 100.0,
                drag * 100.0
            );
        }
    }
}

#[test]
fn segment_distance_cases() {
    let f = segment_distance_for_tests;
    assert!((f(Vec3::ZERO, Vec3::X, Vec3::new(0.5, 1.0, 0.0), Vec3::new(0.5, 2.0, 0.0)) - 1.0).abs() < 1e-5);
    assert!(f(Vec3::ZERO, Vec3::X, Vec3::new(0.5, -1.0, 0.0), Vec3::new(0.5, 1.0, 0.0)) < 1e-5);
    assert!((f(Vec3::ZERO, Vec3::ZERO, Vec3::new(3.0, 4.0, 0.0), Vec3::new(3.0, 4.0, 0.0)) - 5.0).abs() < 1e-5);
}

#[test]
fn frozen_pose_is_kept_for_unbodied_joints() {
    let (s, d) = human();
    let (mut w, base) = world();
    let root = WorldTransform::from_pos(base.offset(Vec3::Y));
    let r = rest_spawn(&mut w, &s, &d, root, &RagdollSettings::default());
    let mut frozen: Vec<Trs> = s.rest().to_vec();
    let root_joint = s.find("root").unwrap();
    frozen[root_joint].rotation = Quat::from_rotation_y(0.3);
    let mut out = Vec::new();
    assert!(r.local_pose(&w, &d, &s, &root, 1.0, &frozen, &mut out));
    // The skeleton's root sits above the pelvis body: it follows rigidly, and the hips stay where the body is.
    let hips = s.find("hips").unwrap();
    let model_hips = out[root_joint].to_affine() * out[hips].to_affine();
    assert!(Vec3::from(model_hips.translation).distance(Vec3::from(s.rest_model()[hips].translation)) < 1e-3);
}
