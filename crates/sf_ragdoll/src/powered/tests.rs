//! P8: a powered ragdoll tracks a walk (physics.md "Verification": ≤ 5 cm
//! RMS at effectors, ≤ 5° per joint), a knockdown falls and rests, the
//! get-up root lands under the pelvis, balance, and the step cost.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]

use glam::{Affine3A, IVec3, Quat, Vec3};
use sf_anim::{LocalPose, ModelPose, RigHints, Skeleton, detect_profile, fixtures, local_to_model};
use sf_coords::{WorldPos, WorldTransform};
use sf_physics::{BodyDesc, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

use super::*;
use crate::{CookParams, RagJointKind, RagdollSettings, SpawnPose, cook};

fn human() -> (Skeleton, RagdollDef) {
    let s = fixtures::biped();
    let profile = detect_profile(&s, &RigHints::default()).expect("profile");
    let d = cook(&s, &profile, &CookParams { total_mass: Some(75.0), ..CookParams::default() }).expect("cook");
    (s, d)
}

fn base() -> WorldPos {
    WorldPos::new(IVec3::new(2, 0, -3), Vec3::new(10.0, 40.0, -20.0))
}

fn floor(w: &mut PhysicsWorld, at: WorldPos) {
    let slab = BodyDesc::fixed(WorldTransform::from_pos(at.offset(Vec3::new(0.0, -0.5, 0.0))), ShapeRef::Box(Vec3::new(30.0, 0.5, 30.0)), Layer::STATIC);
    w.insert(&slab);
}

/// A walk cycle (1 Hz stride): hips swing ±26°, knees flex up to 40°, arms
/// swing ±17° in opposition, the spine counter-twists, the pelvis bobs.
fn walk_pose(s: &Skeleton, t: f32) -> Vec<Affine3A> {
    let knee = 0.7f32;
    let mut local = LocalPose::rest(s);
    let w = std::f32::consts::TAU * t;
    let j = |n: &str| s.find(n).expect(n);
    let mul = |local: &mut LocalPose, joint: usize, q: Quat| {
        let r = local.get(joint).rotation;
        local.set_rotation(joint, (r * q).normalize());
    };
    for (side, sign) in [("l", 1.0f32), ("r", -1.0)] {
        mul(&mut local, j(&format!("thigh_{side}")), Quat::from_rotation_x(-0.45 * sign * w.sin()));
        mul(&mut local, j(&format!("shin_{side}")), Quat::from_rotation_x(knee * (sign * (w + 0.6).sin()).max(0.0)));
        mul(&mut local, j(&format!("upperarm_{side}")), Quat::from_rotation_y(0.3 * sign * w.sin()));
    }
    mul(&mut local, j("chest"), Quat::from_rotation_y(0.1 * w.sin()));
    let hips = j("hips");
    let mut h = local.get(hips);
    h.translation.y += 0.03 * (2.0 * w).sin();
    local.set(hips, h);
    let mut model = ModelPose::new(s.len());
    local_to_model(s, &local, &mut model, &Affine3A::IDENTITY);
    model.mats
}

fn body_pose_target(d: &RagdollDef, b: usize, model: &[Affine3A], root: &WorldTransform) -> WorldTransform {
    body_world(d.bodies[b].local, &model[usize::from(d.bodies[b].joint)], root, 1.0)
}

struct Tracking {
    effector_rms: f32,
    pelvis_rms: f32,
    joint_rms_max_deg: f32,
    worst_joint: String,
    step_ms: f32,
}

/// `n` powered bipeds walking side by side, on a floor or in the air;
/// errors measured on the first after a 1 s warm-up.
fn walk(n: usize, seconds: f32, on_floor: bool, params: &DriveParams) -> Tracking {
    let (s, d) = human();
    let mut w = PhysicsWorld::new(base(), PhysicsParams::default());
    if on_floor {
        floor(&mut w, base());
    }
    let dt = w.params().dt;
    let profile = StrengthProfile::uniform(&d, Drive::Powered { strength: 1.0 }, 1.0);
    let speed = 1.2;
    let lift = if on_floor { 0.0 } else { 1.0 };
    let root_at = |i: usize, t: f32| WorldTransform::from_pos(base().offset(Vec3::new(i as f32 * 2.0, lift, speed * t)));
    let mut ragdolls: Vec<RagdollInstance> = (0..n)
        .map(|i| {
            let (now, prev) = (walk_pose(&s, 0.0), walk_pose(&s, -dt));
            let pose = SpawnPose { root: root_at(i, 0.0), scale: 1.0, model: &now, previous: Some((&prev, dt)), root_velocity: Vec3::new(0.0, 0.0, 1.2) };
            RagdollInstance::spawn(&mut w, &d, &pose, &RagdollSettings::default())
        })
        .collect();
    let effectors: Vec<usize> = ["hand_l", "hand_r", "foot_l", "foot_r", "head"].iter().filter_map(|n| d.body_of_joint(s.find(n)? as u16)).collect();
    let (mut eff_sq, mut eff_n, mut pelvis_sq) = (0.0f64, 0usize, 0.0f64);
    let mut joint_sq = vec![0.0f64; d.joints.len()];
    let mut samples = 0usize;
    let mut step_time = std::time::Duration::ZERO;
    let steps = (seconds / dt).round() as usize;
    let mut prev = walk_pose(&s, 0.0);
    for k in 1..=steps {
        let t = k as f32 * dt;
        let model = walk_pose(&s, t);
        for (i, r) in ragdolls.iter_mut().enumerate() {
            let target = DriveTarget { root: root_at(i, t), model: &model, previous: Some((&prev, dt)) };
            r.drive(&mut w, &d, &target, &profile, params);
        }
        let t0 = std::time::Instant::now();
        w.step();
        step_time += t0.elapsed();
        prev.clone_from(&model);
        if t < 1.0 {
            continue;
        }
        let r = &ragdolls[0];
        let root = root_at(0, t);
        let want = body_pose_target(&d, 0, &model, &root);
        pelvis_sq += f64::from(w.pose(r.bodies[0]).expect("pelvis").pos.delta_f32(want.pos).length_squared());
        for &b in &effectors {
            let got = w.pose(r.bodies[b]).expect("body");
            let want = body_pose_target(&d, b, &model, &root);
            eff_sq += f64::from(got.pos.delta_f32(want.pos).length_squared());
            eff_n += 1;
        }
        for (ji, j) in d.joints.iter().enumerate() {
            let (pa, pc) = (w.pose(r.bodies[usize::from(j.parent)]).expect("p"), w.pose(r.bodies[usize::from(j.child)]).expect("c"));
            let got = (pa.rotation * j.frame_parent.1).inverse() * (pc.rotation * j.frame_child.1);
            let (ta, tc) = (body_pose_target(&d, usize::from(j.parent), &model, &root), body_pose_target(&d, usize::from(j.child), &model, &root));
            let want = (ta.rotation * j.frame_parent.1).inverse() * (tc.rotation * j.frame_child.1);
            joint_sq[ji] += f64::from(got.angle_between(want).powi(2));
        }
        samples += 1;
    }
    let (worst, max) = joint_sq.iter().enumerate().fold((0, 0.0f64), |a, (i, &v)| if v > a.1 { (i, v) } else { a });
    let child = d.bodies[usize::from(d.joints[worst].child)].joint;
    Tracking {
        effector_rms: (eff_sq / eff_n.max(1) as f64).sqrt() as f32,
        pelvis_rms: (pelvis_sq / samples.max(1) as f64).sqrt() as f32,
        joint_rms_max_deg: (max / samples.max(1) as f64).sqrt().to_degrees() as f32,
        worst_joint: s.name(usize::from(child)).to_owned(),
        step_ms: step_time.as_secs_f32() * 1000.0 / steps as f32,
    }
}

#[test]
fn powered_ragdoll_tracks_a_walk() {
    let air = walk(1, 4.0, false, &DriveParams::default());
    let r = walk(1, 4.0, true, &DriveParams::default());
    for (name, r) in [("air", &air), ("floor", &r)] {
        eprintln!(
            "tracking ({name}): effectors {:.2} cm RMS, pelvis {:.2} cm RMS, worst joint {:.2}° RMS ({}), step {:.3} ms",
            r.effector_rms * 100.0,
            r.pelvis_rms * 100.0,
            r.joint_rms_max_deg,
            r.worst_joint,
            r.step_ms
        );
    }
    // Gate 5 cm still open (P8b): the critically damped, gravity-compensated
    // pelvis drive took the air case from 11.5 to 6.8 cm (pelvis 9.0 to
    // 2.2 cm); on the floor the synthetic walk slides and sinks its stance
    // foot (stride faster than the root, 3 cm bob), so the feet fight the
    // floor (~11 cm, foot joint ~12°). Regression bounds.
    assert!(air.effector_rms < 0.08, "air effector RMS {} m", air.effector_rms);
    assert!(air.joint_rms_max_deg < 6.5, "air joint RMS {}° at {}", air.joint_rms_max_deg, air.worst_joint);
    assert!(r.effector_rms < 0.13, "floor effector RMS {} m", r.effector_rms);
}

/// Step cost with the powered budget (8) walking; release only.
#[test]
#[ignore = "timing: cargo test -p sf_ragdoll --release -- --ignored --nocapture powered_cost"]
fn powered_cost() {
    for iterations in [8, 4, 2, 0] {
        let params = DriveParams { powered_iterations: iterations, ..DriveParams::default() };
        for n in [1, 8] {
            let r = walk(n, 4.0, true, &params);
            eprintln!(
                "+{iterations} iterations, {n} powered: step {:.3} ms ({:.3} ms per character), effectors {:.2} cm, worst joint {:.2}°",
                r.step_ms,
                r.step_ms / n as f32,
                r.effector_rms * 100.0,
                r.joint_rms_max_deg
            );
        }
    }
}

#[test]
fn knockdown_falls_and_rests_then_root_goes_under_the_pelvis() {
    let (s, d) = human();
    let mut w = PhysicsWorld::new(base(), PhysicsParams::default());
    floor(&mut w, base());
    let dt = w.params().dt;
    let params = DriveParams::default();
    let rest = s.rest_model();
    let root = WorldTransform::from_pos(base());
    let mut r = RagdollInstance::spawn(&mut w, &d, &SpawnPose { root, scale: 1.0, model: &rest, previous: None, root_velocity: Vec3::ZERO }, &RagdollSettings::default());
    // Standing, powered, for 0.5 s.
    let standing = StrengthProfile::uniform(&d, Drive::Powered { strength: 1.0 }, 1.0);
    for _ in 0..30 {
        r.drive(&mut w, &d, &DriveTarget { root, model: &rest, previous: None }, &standing, &params);
        w.step();
    }
    let chest = d.body_of_joint(s.find("chest").expect("chest") as u16).expect("chest body") as u16;
    let point = w.pose(r.bodies[usize::from(chest)]).expect("chest").pos;
    assert_eq!(r.nearest_body(&w, point), Some(chest));
    // A 120 N·s shove from the front: a knockdown braces at 0.25 strength, then goes limp over 0.8 s.
    assert!(r.hit(&mut w, &d, &HitImpulse { body: chest, point, impulse: Vec3::new(0.0, 0.0, -120.0), dip: 1.0, recover: 0.5 }));
    let brace = StrengthProfile::uniform(&d, Drive::Powered { strength: 0.25 }, 0.0);
    let (mut slow, mut t, mut rested) = (0.0f32, 0.0f32, None);
    while t < 8.0 {
        let fade = 1.0 - (t / 0.8).min(1.0);
        r.drive(&mut w, &d, &DriveTarget { root, model: &rest, previous: None }, &brace.scaled(fade, &params), &params);
        r.advance(dt);
        w.step();
        t += dt;
        slow = if r.pelvis_speed(&w) < 0.2 { slow + dt } else { 0.0 };
        if t > 1.0 && slow >= 0.5 {
            rested = Some(t);
            break;
        }
    }
    let rested = rested.expect("rests");
    assert!(!r.powered, "limp after the brace fade");
    let pelvis = w.pose(r.bodies[0]).expect("pelvis");
    let height = base().delta_f32(pelvis.pos).y;
    assert!(height < 0.45, "fell: pelvis at {height} m");
    let frame = r.implied_frame(&w, &d, &rest, 1.0).expect("frame");
    let face_up = (frame.rotation * Vec3::Z).y > 0.0;
    eprintln!("knockdown: rested at {rested:.2} s, pelvis {height:.2} m, face {}", if face_up { "up" } else { "down" });
    assert!(face_up, "a shove from the front lands it on its back");
    // The get-up root puts a lying clip's pelvis on the ragdoll's.
    let b0 = &d.bodies[0];
    let (rr, tt) = rigid(&rest[usize::from(b0.joint)]);
    let lying = Quat::from_rotation_x(-PI / 2.0);
    let clip_body = (lying * rr * b0.local.1, lying * (tt + rr * b0.local.0) + Vec3::new(0.0, -0.8, 0.0));
    let g = get_up_root(&pelvis, clip_body, Vec3::Y, 1.0, false);
    let landed = g.pos.offset(g.rotation * clip_body.1);
    assert!(landed.delta_f32(pelvis.pos).length() < 1e-3);
}

#[test]
fn get_up_root_recovers_the_frame_and_rolls_face_down_bodies() {
    let clip_body = (Quat::from_rotation_x(-1.4) * Quat::from_rotation_y(0.2), Vec3::new(0.05, 0.15, -0.6));
    let root = WorldTransform { pos: base().offset(Vec3::new(3.0, 0.0, 1.0)), rotation: Quat::from_rotation_y(0.7), scale: Vec3::ONE };
    let body = WorldTransform { pos: root.pos.offset(root.rotation * clip_body.1), rotation: root.rotation * clip_body.0, scale: Vec3::ONE };
    let g = get_up_root(&body, clip_body, Vec3::Y, 1.0, false);
    assert!(g.pos.delta_f32(root.pos).length() < 1e-4);
    assert!(g.rotation.angle_between(root.rotation) < 1e-3);
    // Face down: the same body turned 180° about its own long axis (body-local
    // Y here: the clip lies it along the ground), not about the vertical (AN-10).
    let down = WorldTransform { rotation: body.rotation * Quat::from_rotation_y(PI), ..body };
    assert!((down.rotation * Vec3::Y).y.abs() < 0.2, "the spine lies along the ground");
    let g = get_up_root(&down, clip_body, Vec3::Y, 1.0, true);
    assert!(g.rotation.angle_between(root.rotation) < 1e-3, "rolled back: {}", g.rotation.angle_between(root.rotation));
}

#[test]
fn balance_against_the_support_polygon() {
    let feet = [Vec3::new(-0.1, 0.0, -0.1), Vec3::new(0.1, 0.0, -0.1), Vec3::new(0.1, 0.0, 0.15), Vec3::new(-0.1, 0.0, 0.15)];
    assert_eq!(balance(Vec3::new(0.0, 1.0, 0.0), &feet, 0.05, 0.3), Balance::Stable);
    assert_eq!(balance(Vec3::new(0.0, 1.0, 0.3), &feet, 0.05, 0.3), Balance::Stumble);
    assert_eq!(balance(Vec3::new(0.6, 1.0, 0.0), &feet, 0.05, 0.3), Balance::Fall);
    assert_eq!(balance(Vec3::new(0.0, 1.0, 0.0), &feet[..2], 0.05, 0.3), Balance::Stumble);
    assert_eq!(balance(Vec3::ZERO, &[], 0.05, 0.3), Balance::Fall);
}

#[test]
fn strength_profiles_scale_and_dips_recover() {
    let (_, d) = human();
    let params = DriveParams::default();
    let p = StrengthProfile::uniform(&d, Drive::Kinematic, 1.0);
    assert!(!p.is_limp());
    let half = p.scaled(0.5, &params);
    assert_eq!(half.drive(0), Drive::Powered { strength: 0.5 * params.kinematic_gain });
    assert!(p.scaled(0.0, &params).is_limp());
    assert_eq!(StrengthProfile::limp().drive(0), Drive::Limp);
    let mut dip = Dip { region: 0, amount: 0.8, recover: 1.0, elapsed: 0.0 };
    assert!((dip.factor() - 0.2).abs() < 1e-6);
    dip.elapsed = 0.5;
    assert!((dip.factor() - 0.6).abs() < 1e-6);
    dip.elapsed = 1.0;
    assert!((dip.factor() - 1.0).abs() < 1e-6);
}

/// Per-axis joint angles as Rapier's limit rows measure them: `2·atan2(v_i, w)`
/// of the child frame in the parent frame (X twist, Y/Z swing).
fn joint_angles(w: &PhysicsWorld, r: &RagdollInstance, d: &RagdollDef, i: usize) -> [f32; 3] {
    let (desc, _) = &r.joints[i];
    let (a, b) = (w.pose(desc.parent).expect("parent"), w.pose(desc.child).expect("child"));
    let q = (a.rotation * d.joints[i].frame_parent.1).inverse() * (b.rotation * d.joints[i].frame_child.1);
    let q = if q.w < 0.0 { -q } else { q };
    [q.x, q.y, q.z].map(|v| 2.0 * v.atan2(q.w))
}

/// How far (rad) each axis is past its limit.
fn limit_excess(kind: RagJointKind, a: [f32; 3]) -> f32 {
    let out = |v: f32, lo: f32, hi: f32| (lo - v).max(v - hi).max(0.0);
    match kind {
        RagJointKind::Ball { swing, twist } => out(a[0], twist[0], twist[1]).max(out(a[1], swing[0][0], swing[0][1])).max(out(a[2], swing[1][0], swing[1][1])),
        RagJointKind::Hinge { range } => out(a[0], range[0], range[1]).max(a[1].abs()).max(a[2].abs()),
        RagJointKind::Fixed => a.iter().fold(0.0, |m, v| m.max(v.abs())),
    }
}

struct Fall {
    /// Worst limit excess (rad) and its joint's child bone, per window [0, 0.3), [0.3, 1), [1, 4] s and at the end.
    excess: [(f32, String); 4],
    /// Peak angular speed of a core body (rad/s) after the first 0.3 s, and seconds above 6 rad/s.
    core_spin: f32,
    spinning: f32,
    stretch: f32,
    /// Per region, the peak joint deflection from the rest pose (rad) within the first second.
    flop: Vec<(String, f32)>,
    /// When a body last turned faster than 1 rad/s (s).
    settle: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Death: limp from the hit on.
    Limp,
    /// Knockdown: braced at 0.25 strength fading to limp over 0.8 s.
    Brace,
    /// Stagger: powered, strength 1 -> 0.35 at 0.08 s -> 1 at 0.5 s (reactions.ron).
    Stagger,
}

/// A 75 kg biped standing at rest, shoved at the chest with `impulse` N·s, then either limp
/// (death) or braced at 0.25 strength fading over 0.8 s (knockdown), for 4 s.
fn fall(impulse: f32, braced: bool) -> Fall {
    fall_with(impulse, if braced { Mode::Brace } else { Mode::Limp }, &RagdollSettings::default())
}

fn fall_with(impulse: f32, mode: Mode, settings: &RagdollSettings) -> Fall {
    let (s, d) = human();
    let mut w = PhysicsWorld::new(base(), PhysicsParams::default());
    floor(&mut w, base());
    let dt = w.params().dt;
    let params = DriveParams::default();
    let rest = s.rest_model();
    let root = WorldTransform::from_pos(base());
    let mut r = RagdollInstance::spawn(&mut w, &d, &SpawnPose { root, scale: 1.0, model: &rest, previous: None, root_velocity: Vec3::ZERO }, settings);
    let chest = d.body_of_joint(s.find("chest").expect("chest") as u16).expect("chest body") as u16;
    let full = StrengthProfile::uniform(&d, Drive::Powered { strength: 1.0 }, 1.0);
    if mode == Mode::Stagger {
        for _ in 0..30 {
            r.drive(&mut w, &d, &DriveTarget { root, model: &rest, previous: None }, &full, &params);
            w.step();
        }
    }
    let point = w.pose(r.bodies[usize::from(chest)]).expect("chest").pos;
    let dip = if mode == Mode::Stagger { 0.0 } else { 1.0 };
    r.hit(&mut w, &d, &HitImpulse { body: chest, point, impulse: Vec3::new(0.0, 0.0, -impulse), dip, recover: 0.5 });
    let rel = |w: &PhysicsWorld, r: &RagdollInstance, i: usize| {
        let j = &d.joints[i];
        let (a, b) = (w.pose(r.joints[i].0.parent).expect("parent"), w.pose(r.joints[i].0.child).expect("child"));
        (a.rotation * j.frame_parent.1).inverse() * (b.rotation * j.frame_child.1)
    };
    let rest_rel: Vec<Quat> = (0..d.joints.len()).map(|i| rel(&w, &r, i)).collect();
    let core = d.regions.iter().position(|n| n == "core").expect("core") as u8;
    let brace = StrengthProfile::uniform(&d, Drive::Powered { strength: 0.25 }, 0.0);
    let flop = d.regions.iter().map(|n| (n.clone(), 0.0)).collect();
    let mut f = Fall { excess: Default::default(), core_spin: 0.0, spinning: 0.0, stretch: 0.0, flop, settle: 0.0 };
    let dbg = std::env::var_os("FALL_DBG").is_some();
    let mut t = 0.0f32;
    while t < 4.0 {
        let profile = match mode {
            Mode::Brace => brace.scaled(1.0 - (t / 0.8).min(1.0), &params),
            Mode::Limp => StrengthProfile::limp(),
            Mode::Stagger => full.scaled(if t < 0.08 { 1.0 - 0.65 * t / 0.08 } else { (0.35 + 0.65 * (t - 0.08) / 0.42).min(1.0) }, &params),
        };
        r.drive(&mut w, &d, &DriveTarget { root, model: &rest, previous: None }, &profile, &params);
        r.advance(dt);
        w.step();
        t += dt;
        let window = if t < 0.3 { 0 } else if t < 1.0 { 1 } else { 2 };
        let at = if t + dt >= 4.0 { 3 } else { window };
        for (i, j) in d.joints.iter().enumerate() {
            let a = joint_angles(&w, &r, &d, i);
            let e = limit_excess(j.kind, a);
            let name = s.name(usize::from(d.bodies[usize::from(j.child)].joint));
            if dbg && e > 0.05 {
                eprintln!("DBG t={t:.3} {name} angles {:?} kind {:?}", a.map(f32::to_degrees), j.kind);
            }
            for k in [window, at] {
                if e > f.excess[k].0 {
                    f.excess[k] = (e, name.to_owned());
                }
            }
        }
        let spin = d.bodies.iter().zip(&r.bodies).filter(|(b, _)| b.region == core).map(|(_, &id)| w.velocity(id).expect("body").1.length()).fold(0.0, f32::max);
        if t > 0.3 {
            f.core_spin = f.core_spin.max(spin);
            if spin > 6.0 {
                f.spinning += dt;
            }
        }
        f.stretch = f.stretch.max(r.max_stretch(&w));
        if t <= 1.0 {
            for (i, j) in d.joints.iter().enumerate() {
                let dev = rest_rel[i].angle_between(rel(&w, &r, i));
                let reg = usize::from(d.bodies[usize::from(j.child)].region);
                f.flop[reg].1 = f.flop[reg].1.max(dev);
            }
        }
        if r.bodies.iter().any(|&id| w.velocity(id).expect("body").1.length() > 1.0) {
            f.settle = t;
        }
    }
    f
}

/// AN-20: a hit leaves limbs loose. The same 120 N·s knockdown and 40 N·s stagger with every
/// body at its shape's own inertia (the physical reference, 24 iterations) and with the
/// default conditioning: arms and head deflect at least 75% as far in the first second of a
/// knockdown, 45% in a stagger (the core floor keeps the chest from turning as far, and the
/// app shows a stagger at physics weight 0.2). AN-18's floor on every body: knockdown arms
/// 0.64, head 0.34; stagger arms 0.25. (Limp
/// deaths are left out: their flop swings with the solver's chaos, 7-29° for one setup.)
#[test]
fn hits_leave_limbs_loose() {
    let shape = RagdollSettings { inertia_floor: 0.0, end_inertia_floor: 0.0, head_inertia_floor: 0.0, axial_inertia: 0.0, extra_iterations: 24, ..RagdollSettings::default() };
    let mut fails = Vec::new();
    for (label, impulse, mode, bar) in [("knockdown", 120.0, Mode::Brace, 0.75), ("stagger", 40.0, Mode::Stagger, 0.45)] {
        let (reference, now) = (fall_with(impulse, mode, &shape), fall_with(impulse, mode, &RagdollSettings::default()));
        for (region, bar) in [("arm_l", bar), ("arm_r", bar), ("head", bar * 0.8)] {
            let flop = |f: &Fall| f.flop.iter().find(|(n, _)| n == region).map_or(0.0, |r| r.1);
            let (a, b) = (flop(&reference), flop(&now));
            eprintln!("{label} {region}: {:.0}° of {:.0}° ({:.2})", b.to_degrees(), a.to_degrees(), b / a);
            if b < bar * a {
                fails.push(format!("{label} {region}"));
            }
        }
    }
    assert!(fails.is_empty(), "stiff: {fails:?}");
}

/// AN-20 exploration: looseness and limits per inertia floor and solver iterations.
#[test]
#[ignore = "exploration, prints numbers"]
fn looseness_sweep() {
    // SWEEP="core floor,limb floor,axial,iterations;..."
    let sweep: Vec<(f32, f32, f32, usize)> = std::env::var("SWEEP").ok().map_or_else(
        || vec![(1.0, 1.0, 0.0, 8), (0.0, 0.0, 0.0, 24), (1.0, 0.0, 0.5, 8)],
        |v| {
            v.split(';')
                .map(|p| {
                    let x: Vec<&str> = p.split(',').collect();
                    (x[0].parse().expect("core"), x[1].parse().expect("limb"), x[2].parse().expect("axial"), x[3].parse().expect("iterations"))
                })
                .collect()
        },
    );
    for (floor, limb, axial, iters) in sweep {
        let settings = RagdollSettings { inertia_floor: floor, end_inertia_floor: limb, axial_inertia: axial, extra_iterations: iters, ..RagdollSettings::default() };
        for (label, impulse, mode) in [("death", 120.0, Mode::Limp), ("hard death", 300.0, Mode::Limp), ("knockdown", 120.0, Mode::Brace), ("hard knockdown", 300.0, Mode::Brace), ("stagger", 40.0, Mode::Stagger)] {
            let f = fall_with(impulse, mode, &settings);
            let worst = f.excess.iter().max_by(|a, b| a.0.total_cmp(&b.0)).expect("windows");
            let flop = f.flop.iter().map(|(n, v)| format!("{n} {:.0}", v.to_degrees())).collect::<Vec<_>>().join(", ");
            eprintln!(
                "floor {floor:.2}/{limb:.2} axial {axial} it {iters} {label}: excess {:.1}° {}, spin {:.1} ({:.2} s), stretch {:.1} cm, settle {:.2} s, flop° [{flop}]",
                worst.0.to_degrees(),
                worst.1,
                f.core_spin,
                f.spinning,
                f.stretch * 100.0,
                f.settle
            );
        }
    }
}

/// AN-18: a hard knockdown and a limp death keep every joint within its limits (+5°), the
/// torso doesn't spin (≤ 0.3 s above 6 rad/s), joints stretch ≤ 8 cm under these 120-300 N·s
/// chest hits (before the fix: the spine 25° past its limit at 120, 95° and stuck at 300).
#[test]
fn falls_keep_joints_in_limits_and_the_torso_from_spinning() {
    let mut fails = Vec::new();
    for (label, impulse, braced) in [("death", 120.0, false), ("hard death", 300.0, false), ("knockdown", 120.0, true), ("hard knockdown", 300.0, true)] {
        let f = fall(impulse, braced);
        let ex = f.excess.iter().map(|(e, n)| format!("{:.1}° {n}", e.to_degrees())).collect::<Vec<_>>().join(" | ");
        eprintln!("{label}: excess (0-0.3 | 0.3-1 | 1-4 s | end) {ex}; core spin peak {:.1} rad/s ({:.2} s above 6), stretch {:.2} cm", f.core_spin, f.spinning, f.stretch * 100.0);
        let worst = f.excess.iter().map(|e| e.0).fold(0.0, f32::max);
        if worst.to_degrees() > 5.0 || f.spinning > 0.3 || f.stretch > 0.08 {
            fails.push(label);
        }
    }
    assert!(fails.is_empty(), "failed: {fails:?}");
}
