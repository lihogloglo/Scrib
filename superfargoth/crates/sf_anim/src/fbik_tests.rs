//! Full-body IK (animation.md "Layer 2", verification): reaches beyond a
//! limb pull the spine and pelvis, feet stay pinned, hinges never flip,
//! limits hold, the solve is stable frame to frame and identical on any
//! thread count, on a biped, a digitigrade quadruped and a real humanoid.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use glam::{Affine3A, Quat, Vec3};
use rayon::prelude::*;

use crate::fixtures::{biped, quadruped};
use crate::ik::{self, Effector, FullBody, FullBodyScratch};
use crate::*;

struct Fixture {
    s: Skeleton,
    body: FullBody,
    local: LocalPose,
    model: ModelPose,
}

impl Fixture {
    fn new(s: Skeleton) -> Self {
        let p = detect_profile(&s, &RigHints::default()).unwrap();
        let body = FullBody::new(&s, &p).unwrap();
        let local = LocalPose::rest(&s);
        let mut model = ModelPose::new(s.len());
        local_to_model(&s, &local, &mut model, &Affine3A::IDENTITY);
        Self {
            s,
            body,
            local,
            model,
        }
    }

    fn j(&self, name: &str) -> usize {
        self.s.find(name).unwrap()
    }

    fn pos(&self, name: &str) -> Vec3 {
        self.model.translation(self.j(name))
    }

    /// Pins `pinned` joints where they are, adds `goals`, solves.
    fn solve(
        &self,
        pinned: &[&str],
        goals: &[(&str, Vec3)],
        iterations: usize,
    ) -> (LocalPose, ModelPose, f32) {
        let mut effectors: Vec<Effector> = pinned
            .iter()
            .map(|n| {
                let j = self.j(n);
                Effector {
                    joint: j,
                    position: self.model.translation(j),
                    rotation: Some(self.model.rotation(j)),
                    weight: 1.0,
                    pull: 0.0,
                }
            })
            .collect();
        effectors.extend(goals.iter().map(|&(n, p)| Effector {
            joint: self.j(n),
            position: p,
            rotation: None,
            weight: 1.0,
            pull: 1.0,
        }));
        let (mut local, mut model) = (self.local.clone(), self.model.clone());
        let mut scratch = FullBodyScratch::default();
        let e = self.body.solve(
            &self.s,
            &mut local,
            &mut model,
            &Affine3A::IDENTITY,
            &effectors,
            iterations,
            &mut scratch,
        );
        (local, model, e)
    }

    /// Every body joint's local rotation change is within its limit, and
    /// the model pose agrees with the local pose.
    fn assert_limits(&self, local: &LocalPose, model: &ModelPose) {
        for b in &self.body.joints {
            let j = usize::from(b.joint);
            let change = angle_between(local.get(j).rotation, self.local.get(j).rotation);
            assert!(
                change <= b.limit + 1e-3,
                "{} turned {:.1}° > {:.1}°",
                self.s.name(j),
                change.to_degrees(),
                b.limit.to_degrees()
            );
        }
        let mut check = ModelPose::new(self.s.len());
        local_to_model(&self.s, local, &mut check, &Affine3A::IDENTITY);
        for j in 0..self.s.len() {
            assert!(
                check.translation(j).distance(model.translation(j)) < 1e-4,
                "{}: model pose stale",
                self.s.name(j)
            );
            assert!(model.translation(j).is_finite());
        }
    }

    /// Side of `mid` relative to the line `a`–`c`, along the animated side.
    fn bend_side(&self, model: &ModelPose, [a, m, c]: [&str; 3]) -> f32 {
        let side = |model: &ModelPose| {
            let (pa, pm, pc) = (
                model.translation(self.j(a)),
                model.translation(self.j(m)),
                model.translation(self.j(c)),
            );
            let line = (pc - pa).normalize();
            let off = pm - pa;
            off - line * off.dot(line)
        };
        let rest = side(&self.model).normalize();
        // The animated side turned with the limb's line.
        let (r0, r1) = (
            self.pos(c) - self.pos(a),
            model.translation(self.j(c)) - model.translation(self.j(a)),
        );
        let rest = Quat::from_rotation_arc(r0.normalize(), r1.normalize()) * rest;
        side(model).dot(rest)
    }
}

const FEET: [&str; 2] = ["foot_l", "foot_r"];

#[test]
fn biped_reach_beyond_the_arm_pulls_the_spine_and_pelvis() {
    let b = Fixture::new(biped());
    let shoulder = b.pos("upperarm_l");
    let target = Vec3::new(0.3, 1.2, 0.75);
    assert!(shoulder.distance(target) > 0.53 * 1.5, "beyond the arm");
    let (local, model, err) = b.solve(&FEET, &[("hand_l", target)], 10);
    assert!(err < 0.02, "hand misses by {err}");
    for f in FEET {
        assert!(
            model.translation(b.j(f)).distance(b.pos(f)) < 5e-3,
            "{f} left its pin"
        );
    }
    // The torso leans toward the target: the chest comes forward, the spine bends.
    assert!(
        model.translation(b.j("chest")).z - b.pos("chest").z > 0.1,
        "chest did not follow"
    );
    let spine_turn = angle_between(
        local.get(b.j("spine")).rotation,
        b.local.get(b.j("spine")).rotation,
    );
    assert!(
        spine_turn > 5f32.to_radians(),
        "spine did not participate ({:.1}°)",
        spine_turn.to_degrees()
    );
    // No knee flips.
    for leg in [
        ["thigh_l", "shin_l", "foot_l"],
        ["thigh_r", "shin_r", "foot_r"],
    ] {
        assert!(b.bend_side(&model, leg) >= -1e-4, "{leg:?} flipped");
    }
    b.assert_limits(&local, &model);
}

#[test]
fn reachable_goals_land_within_a_centimeter_and_no_goal_changes_nothing() {
    let b = Fixture::new(biped());
    let target = b.pos("hand_l") + Vec3::new(-0.1, -0.15, 0.2);
    let (local, model, err) = b.solve(&FEET, &[("hand_l", target)], 10);
    assert!(err < 0.01, "miss {err}");
    b.assert_limits(&local, &model);
    // Only pins (already satisfied): the pose is untouched.
    let (local, _, err) = b.solve(&FEET, &[], 10);
    assert!(err < 1e-5);
    for j in 0..b.s.len() {
        assert!(
            angle_between(local.get(j).rotation, b.local.get(j).rotation) < 1e-4,
            "{}",
            b.s.name(j)
        );
        assert!(
            local
                .get(j)
                .translation
                .distance(b.local.get(j).translation)
                < 1e-5,
            "{}",
            b.s.name(j)
        );
    }
}

#[test]
fn unreachable_goals_clamp_without_flips_and_hold_limits() {
    let b = Fixture::new(biped());
    let target = Vec3::new(3.0, 1.5, 1.0);
    let (local, model, err) = b.solve(&FEET, &[("hand_l", target)], 10);
    assert!(err.is_finite() && err > 1.0);
    // The arm points at the target.
    let arm = model.translation(b.j("hand_l")) - model.translation(b.j("upperarm_l"));
    let to = target - model.translation(b.j("upperarm_l"));
    assert!(
        arm.angle_between(to) < 30f32.to_radians(),
        "arm off by {:.1}°",
        arm.angle_between(to).to_degrees()
    );
    for f in FEET {
        assert!(
            model.translation(b.j(f)).distance(b.pos(f)) < 0.01,
            "{f} left its pin"
        );
    }
    for leg in [
        ["thigh_l", "shin_l", "foot_l"],
        ["thigh_r", "shin_r", "foot_r"],
    ] {
        assert!(b.bend_side(&model, leg) >= -1e-4, "{leg:?} flipped");
    }
    b.assert_limits(&local, &model);
}

#[test]
fn moving_goals_give_a_smooth_stable_pose() {
    let b = Fixture::new(biped());
    let (mut last, mut last_target): (Option<ModelPose>, Vec3) = (None, Vec3::ZERO);
    for k in 0..120 {
        let t = k as f32 / 60.0;
        let target = Vec3::new(
            0.4 + 0.15 * (t * 3.0).sin(),
            1.3 + 0.1 * (t * 2.0).cos(),
            0.55 + 0.15 * (t * 1.5).sin(),
        );
        let (_, model, _) = b.solve(&FEET, &[("hand_l", target)], 10);
        // Same input, same output, bit for bit.
        let (_, again, _) = b.solve(&FEET, &[("hand_l", target)], 10);
        assert_eq!(model, again);
        if let Some(prev) = &last {
            let step = target.distance(last_target);
            for j in 0..b.s.len() {
                let moved = model.translation(j).distance(prev.translation(j));
                assert!(
                    moved <= 3.0 * step + 1e-3,
                    "frame {k}: {} jumped {moved} (target moved {step})",
                    b.s.name(j)
                );
            }
        }
        last = Some(model);
        last_target = target;
    }
}

#[test]
fn one_and_many_threads_give_identical_results() {
    let b = Fixture::new(biped());
    let targets: Vec<Vec3> = (0..64)
        .map(|k| {
            Vec3::new(
                0.2 + 0.01 * k as f32,
                1.0 + 0.008 * k as f32,
                0.3 + 0.01 * (k % 7) as f32,
            )
        })
        .collect();
    let run = |t: &Vec3| {
        b.solve(
            &FEET,
            &[("hand_l", *t), ("hand_r", *t * Vec3::new(-1.0, 1.0, 1.0))],
            10,
        )
        .1
    };
    let serial: Vec<ModelPose> = targets.iter().map(run).collect();
    let parallel: Vec<ModelPose> = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap()
        .install(|| targets.par_iter().map(run).collect());
    assert_eq!(serial, parallel);
}

#[test]
fn quadruped_head_reach_bends_the_neck_spine_and_keeps_four_feet_planted() {
    let b = Fixture::new(quadruped(1.0));
    let feet = ["toe_l", "toe_r", "finger_l", "finger_r"];
    let feet: Vec<&str> = feet
        .into_iter()
        .filter(|f| b.body.particle(b.j(f)).is_some())
        .collect();
    // Effectors are the profile's limb ends (paw or toe, wrist or finger).
    let ends: Vec<&str> = [
        "toe_l", "toe_r", "finger_l", "finger_r", "paw_l", "paw_r", "wrist_l", "wrist_r",
    ]
    .into_iter()
    .filter(|f| {
        b.body.particle(b.j(f)).is_some()
            && b.body.joints[b.body.particle(b.j(f)).unwrap()].primary == ik::NO_JOINT
    })
    .collect();
    assert_eq!(
        ends.len(),
        4,
        "four limb ends in the body: {ends:?} ({feet:?})"
    );
    // Grazing: the head goes down and forward, beyond the neck's reach.
    let target = b.pos("head") + Vec3::new(0.0, -0.25, 0.1);
    let (local, model, err) = b.solve(&ends, &[("head", target)], 10);
    // A non-limb effector is placed exactly by the particles; the root shift
    // and a CCD pass remove what the rotation recovery loses: within 1 cm
    // at 10 and at 20 iterations.
    assert!(err < 0.01, "head misses by {err}");
    let (_, _, err20) = b.solve(&ends, &[("head", target)], 20);
    assert!(err20 < 0.01, "head misses by {err20} at 20 iterations");
    for f in &ends {
        assert!(
            model.translation(b.j(f)).distance(b.pos(f)) < 5e-3,
            "{f} left its pin"
        );
    }
    assert!(
        model.translation(b.j("chest")).y < b.pos("chest").y - 0.02,
        "the chest did not lower"
    );
    for leg in [
        ["thigh_l", "shin_l", "hock_l"],
        ["shin_l", "hock_l", "paw_l"],
        ["upper_l", "fore_l", "wrist_l"],
    ] {
        assert!(b.bend_side(&model, leg) >= -1e-4, "{leg:?} flipped");
    }
    b.assert_limits(&local, &model);
    // The tail (not in the body) follows the pelvis rigidly.
    let tail = |m: &ModelPose| m.translation(b.j("tail3")) - m.translation(b.j("pelvis"));
    let pelvis_turn = model.rotation(b.j("pelvis")) * b.model.rotation(b.j("pelvis")).inverse();
    assert!((pelvis_turn * tail(&b.model)).distance(tail(&model)) < 1e-4);
}

#[test]
#[ignore = "timing; run in release"]
fn full_body_timing() {
    let n = 20_000u32;
    let time = |b: &Fixture, pinned: &[&str], goals: &[(&str, Vec3)]| {
        let mut effectors: Vec<Effector> = pinned
            .iter()
            .map(|f| {
                let j = b.j(f);
                Effector {
                    joint: j,
                    position: b.model.translation(j),
                    rotation: Some(b.model.rotation(j)),
                    weight: 1.0,
                    pull: 0.0,
                }
            })
            .collect();
        effectors.extend(goals.iter().map(|&(f, p)| Effector {
            joint: b.j(f),
            position: p,
            rotation: None,
            weight: 1.0,
            pull: 1.0,
        }));
        let (mut local, mut model) = (b.local.clone(), b.model.clone());
        let mut scratch = FullBodyScratch::default();
        let start = std::time::Instant::now();
        for _ in 0..n {
            local.copy_from(&b.local);
            model.mats.copy_from_slice(&b.model.mats);
            std::hint::black_box(b.body.solve(
                &b.s,
                &mut local,
                &mut model,
                &Affine3A::IDENTITY,
                &effectors,
                10,
                &mut scratch,
            ));
        }
        start.elapsed().as_secs_f64() * 1e6 / f64::from(n)
    };
    let biped = Fixture::new(biped());
    let t = time(&biped, &FEET, &[("hand_l", Vec3::new(0.3, 1.2, 0.75))]);
    println!("biped: {} body joints, {t:.2} µs", biped.body.joints.len());
    let quad = Fixture::new(quadruped(1.0));
    let ends: Vec<&str> = [
        "toe_l", "toe_r", "finger_l", "finger_r", "paw_l", "paw_r", "wrist_l", "wrist_r",
    ]
    .into_iter()
    .filter(|f| {
        quad.body
            .particle(quad.j(f))
            .is_some_and(|p| quad.body.joints[p].primary == ik::NO_JOINT)
    })
    .collect();
    let t = time(
        &quad,
        &ends,
        &[("head", quad.pos("head") + Vec3::new(0.0, -0.55, 0.25))],
    );
    println!(
        "quadruped: {} body joints, {t:.2} µs",
        quad.body.joints.len()
    );
    let file = "quaternius_universal_rig.gltf";
    let path = format!(
        "{}/../sf_character/tests/data/{file}",
        env!("CARGO_MANIFEST_DIR")
    );
    let src = GltfAnimImporter
        .import(file, &std::fs::read(path).unwrap())
        .unwrap();
    let (s, _) = Skeleton::with_rules(&src.skeleton.unwrap(), &SkeletonRules::default()).unwrap();
    let human = Fixture::new(s);
    let p = detect_profile(&human.s, &RigHints::default()).unwrap();
    let ends: Vec<String> = p
        .limbs
        .iter()
        .map(|l| human.s.name(usize::from(l.effector)).to_owned())
        .collect();
    let (feet, hands): (Vec<&Limb>, Vec<&Limb>) = p.limbs.iter().partition(|l| l.support);
    let feet: Vec<&str> = feet
        .iter()
        .map(|l| human.s.name(usize::from(l.effector)))
        .collect();
    let hand = human.s.name(usize::from(hands[0].effector)).to_owned();
    let target = human.pos(&hand) + Vec3::new(0.0, -0.3, 0.6) * p.measures.height;
    let t = time(&human, &feet, &[(hand.as_str(), target)]);
    println!(
        "quaternius humanoid: {} joints, {} body joints ({ends:?}), {t:.2} µs",
        human.s.len(),
        human.body.joints.len()
    );
}
