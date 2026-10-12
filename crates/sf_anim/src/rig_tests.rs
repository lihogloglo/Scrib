//! Rig profiles, limb solvers, profile-driven contacts, root attitude and
//! limb retargeting on synthetic creatures and the CC0 Quaternius rigs
//! (rig-profile.md "Verification").
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::too_many_lines)]

use std::f32::consts::TAU;

use glam::{Affine3A, Quat, Vec3};

use crate::fixtures::{biped, flyer, gait, hexapod, quadruped, sampled};
use crate::ik::{LimbGoal, LimbSolver, limb_pole, solve_limb};
use crate::*;

fn names<'a>(s: &'a Skeleton, joints: &[u16]) -> Vec<&'a str> {
    joints.iter().map(|&j| s.name(usize::from(j))).collect()
}

fn limb<'a>(p: &'a RigProfile, name: &str) -> &'a Limb {
    p.limb(name).unwrap_or_else(|| panic!("no limb {name}: {:?}", p.limbs.iter().map(|l| &l.name).collect::<Vec<_>>()))
}

#[test]
fn detects_a_biped() {
    let s = biped();
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Biped);
    let l = limb(&p, "leg_l");
    assert_eq!(names(&s, &l.joints), ["thigh_l", "shin_l", "foot_l"]);
    assert_eq!((l.solver, l.kind, l.side, l.support), (LimbSolver::TwoBone, LimbKind::Leg(LegKind::Plantigrade), Side::Left, true));
    assert_eq!(limb(&p, "leg_r").side, Side::Right);
    let a = limb(&p, "arm_l");
    assert_eq!(names(&s, &a.joints), ["upperarm_l", "forearm_l", "hand_l"]);
    assert!(!a.support);
    assert_eq!(names(&s, &p.spine.as_ref().unwrap().joints), ["hips", "spine", "chest"]);
    assert_eq!(s.name(usize::from(p.neck_head.as_ref().unwrap().head)), "head");
    assert_eq!(p.girdles.len(), 2);
    assert_eq!(s.name(usize::from(p.girdles[0].joint)), "hips");
    assert!(p.mirror.contains(&(u16::try_from(s.find("shin_l").unwrap()).unwrap(), u16::try_from(s.find("shin_r").unwrap()).unwrap())));
    assert!((p.measures.leg_length - 0.9).abs() < 0.01, "{:?}", p.measures);
    assert_eq!(l.foot.as_ref().unwrap().contact, "contact.foot_l");
}

#[test]
fn detects_a_digitigrade_quadruped() {
    let s = quadruped(1.0);
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Quadruped);
    let hind = limb(&p, "leg_bl");
    assert_eq!(names(&s, &hind.joints), ["thigh_l", "shin_l", "hock_l", "paw_l"]);
    assert_eq!((hind.solver, hind.kind), (LimbSolver::TwoBoneAnkle, LimbKind::Leg(LegKind::Digitigrade)));
    assert_eq!(s.name(usize::from(hind.foot.as_ref().unwrap().toe.unwrap())), "toe_l");
    let front = limb(&p, "leg_fr");
    assert_eq!(names(&s, &front.joints), ["scapula_r", "upper_r", "fore_r", "wrist_r"]);
    assert_eq!(front.solver, LimbSolver::TwoBone, "the short scapula stays passive");
    assert_eq!((front.order, hind.order), (0, 1));
    assert_eq!(names(&s, &p.girdles.iter().map(|g| g.joint).collect::<Vec<_>>()), ["pelvis", "chest"]);
    assert_eq!(p.girdle_joint(front), u16::try_from(s.find("chest").unwrap()).unwrap());
    assert_eq!(names(&s, &p.spine.as_ref().unwrap().joints), ["pelvis", "spine1", "spine2", "chest"]);
    assert_eq!(names(&s, &p.neck_head.as_ref().unwrap().neck.joints), ["neck"]);
    assert_eq!(names(&s, &p.tails[0].joints), ["tail1", "tail2", "tail3"]);
    // Poles in girdle space (girdles are unrotated here): hind knee forward,
    // front elbow backward.
    let axis = |l: &Limb| match l.pole {
        PoleHint::Axis(a) => a,
        PoleHint::Animated => panic!("no axis"),
    };
    assert!(axis(hind).z > 0.8, "{}", axis(hind));
    assert!(axis(front).z < -0.8, "{}", axis(front));
    assert!(p.measures.body_length > 1.5 && p.measures.hip_height > 0.85, "{:?}", p.measures);
}

#[test]
fn detects_a_hexapod_and_a_flyer() {
    let s = hexapod();
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Multiped);
    assert_eq!(p.support_limbs().count(), 6);
    assert_eq!(p.girdles.len(), 1);
    for (row, name) in [(0, "leg_l0"), (1, "leg_l1"), (2, "leg_l2")] {
        let l = limb(&p, name);
        assert_eq!(l.order, row);
        assert_eq!(names(&s, &l.joints)[0], format!("coxa{row}_l"));
        assert_eq!(l.solver, LimbSolver::TwoBone);
    }
    assert_eq!(names(&s, &p.tails[0].joints), ["abdomen1", "abdomen2"]);

    let s = flyer();
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Flyer);
    let w = limb(&p, "wing_l");
    assert_eq!((w.kind, w.support), (LimbKind::Wing, false));
    assert_eq!(names(&s, &w.joints), ["wing_upper_l", "wing_fore_l", "wing_hand_l"]);
    let leg = limb(&p, "leg_r");
    assert_eq!((leg.kind, leg.support), (LimbKind::Leg(LegKind::Plantigrade), false));
    assert_eq!(p.tails.len(), 1);
    assert!(p.mirror.len() >= 6);
}

fn skeleton_of(joints: &[(&str, Option<&str>, [f32; 3])]) -> Skeleton {
    let index = |name: &str| joints.iter().position(|j| j.0 == name).expect("parent listed");
    let src: Vec<SourceJoint> = joints
        .iter()
        .map(|&(name, parent, t)| SourceJoint { name: name.to_owned(), parent: parent.map(index), rest: Trs::from_translation(Vec3::from(t)) })
        .collect();
    Skeleton::new(&src).unwrap().0
}

/// Regression: a body joint that carries only legs (nothing else below it,
/// with a non-support leaf beside it under the root, as a mesh node) was
/// walked past, so all six legs became one limb rooted at the body. Same for
/// a pelvis that carries only two legs. Splayed toes of one foot must not
/// split the leg.
#[test]
fn a_joint_carrying_legs_on_both_sides_is_their_girdle() {
    let mut j: Vec<(String, Option<String>, [f32; 3])> =
        vec![("root".into(), None, [0.0; 3]), ("body".into(), Some("root".into()), [0.0, 0.3, 0.0]), ("shell".into(), Some("root".into()), [0.0, 0.35, 0.0])];
    for (row, z) in [(0, 0.15f32), (1, 0.0), (2, -0.15)] {
        for (s, x) in [("l", 1.0f32), ("r", -1.0)] {
            let n = |base: &str| format!("{base}{row}_{s}");
            j.extend([
                (n("coxa"), Some("body".to_owned()), [0.1 * x, -0.02, z]),
                (n("femur"), Some(n("coxa")), [0.4 * x, 0.25, z]),
                (n("tip"), Some(n("femur")), [0.1 * x, -0.53, 0.1 * z]),
            ]);
        }
    }
    let refs: Vec<(&str, Option<&str>, [f32; 3])> = j.iter().map(|(n, p, t)| (n.as_str(), p.as_deref(), *t)).collect();
    let s = skeleton_of(&refs);
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Multiped);
    assert_eq!(p.support_limbs().count(), 6);
    assert_eq!(names(&s, &p.girdles.iter().map(|g| g.joint).collect::<Vec<_>>()), ["body"]);
    for (row, name) in [(0, "leg_l0"), (1, "leg_l1"), (2, "leg_l2")] {
        let l = limb(&p, name);
        assert_eq!(names(&s, &l.joints), [format!("coxa{row}_l"), format!("femur{row}_l"), format!("tip{row}_l")]);
        assert_eq!((l.solver, l.side, l.order), (LimbSolver::TwoBone, Side::Left, row));
    }

    // Legs only under the hips; three toes splayed on each foot.
    let mut j: Vec<(String, Option<String>, [f32; 3])> = vec![("root".into(), None, [0.0; 3]), ("hips".into(), Some("root".into()), [0.0, 0.9, 0.0])];
    for (s, x) in [("l", 1.0f32), ("r", -1.0)] {
        let n = |base: &str| format!("{base}_{s}");
        j.extend([
            (n("thigh"), Some("hips".to_owned()), [0.1 * x, 0.0, 0.0]),
            (n("shin"), Some(n("thigh")), [0.0, -0.45, 0.02]),
            (n("foot"), Some(n("shin")), [0.0, -0.4, -0.02]),
            (n("toe_a"), Some(n("foot")), [0.03, -0.05, 0.12]),
            (n("toe_b"), Some(n("foot")), [0.0, -0.05, 0.13]),
            (n("toe_c"), Some(n("foot")), [-0.03, -0.05, 0.12]),
        ]);
    }
    let refs: Vec<(&str, Option<&str>, [f32; 3])> = j.iter().map(|(n, p, t)| (n.as_str(), p.as_deref(), *t)).collect();
    let s = skeleton_of(&refs);
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    assert_eq!(p.template, RigTemplate::Biped);
    assert_eq!(names(&s, &limb(&p, "leg_l").joints), ["thigh_l", "shin_l", "foot_l"]);
    assert_eq!(names(&s, &limb(&p, "leg_r").joints), ["thigh_r", "shin_r", "foot_r"]);
    assert_eq!(names(&s, &p.girdles.iter().map(|g| g.joint).collect::<Vec<_>>()), ["hips"]);
}

#[test]
fn detects_the_quaternius_humanoids_and_round_trips_through_ron() {
    for (file, thigh, toe) in [("quaternius_universal_rig.gltf", "thigh_l", "ball_l"), ("quaternius_ual1_rig.gltf", "DEF-thigh.L", "DEF-toe.L")] {
        let path = format!("{}/../sf_character/tests/data/{file}", env!("CARGO_MANIFEST_DIR"));
        let src = GltfAnimImporter.import(file, &std::fs::read(path).unwrap()).unwrap();
        let (s, _) = Skeleton::with_rules(&src.skeleton.unwrap(), &SkeletonRules::default()).unwrap();
        let p = detect_profile(&s, &RigHints::default()).unwrap();
        assert_eq!(p.template, RigTemplate::Biped, "{file}");
        let legs: Vec<&Limb> = p.support_limbs().collect();
        assert_eq!(legs.len(), 2, "{file}");
        let l = legs.iter().find(|l| l.side == Side::Left).unwrap();
        assert_eq!(s.name(usize::from(l.joints[0])), thigh, "{file}");
        assert_eq!((l.joints.len(), l.solver), (3, LimbSolver::TwoBone), "{file}");
        assert_eq!(s.name(usize::from(l.foot.as_ref().unwrap().toe.unwrap())), toe, "{file}");
        let arms: Vec<&Limb> = p.limbs.iter().filter(|l| l.kind == LimbKind::Arm).collect();
        assert_eq!(arms.len(), 2, "{file}: {:?}", p.limbs.iter().map(|l| (&l.name, l.kind)).collect::<Vec<_>>());
        assert_eq!(arms[0].joints.len(), 4, "clavicle, upper, lower, hand");
        assert!(p.neck_head.is_some());

        let text = ron::ser::to_string_pretty(&p, ron::ser::PrettyConfig::default()).unwrap();
        let back: RigProfile = ron::from_str(&text).unwrap();
        assert_eq!(back, p);
        assert_eq!(RigProfile::from_bytes(&p.to_bytes()).unwrap(), p);
        p.validate(&s).unwrap();
    }
}

#[test]
fn hints_override_detection() {
    let s = quadruped(1.0);
    let hints = RigHints {
        template: Some(RigTemplate::Custom),
        limbs: vec![LimbHint {
            name: "tail_limb".into(),
            joints: vec!["tail1".into(), "tail2".into(), "tail3".into()],
            kind: Some(LimbKind::Tentacle),
            solver: Some(LimbSolver::Fabrik),
            support: Some(false),
            pole: None,
        }],
        ..RigHints::default()
    };
    let p = detect_profile(&s, &hints).unwrap();
    assert_eq!(p.template, RigTemplate::Custom);
    assert_eq!(limb(&p, "tail_limb").solver, LimbSolver::Fabrik);
    assert!(p.tails.is_empty(), "the hinted joints are not detected again");
    assert_eq!(p.support_limbs().count(), 4);
    // A full profile override is used as is, and checked.
    let full = RigHints { profile: Some(p.clone()), ..RigHints::default() };
    assert_eq!(detect_profile(&s, &full).unwrap(), p);
    assert_eq!(detect_profile(&biped(), &full), Err(RigError::Mismatch));
    let bad = RigHints { limbs: vec![LimbHint { name: "x".into(), joints: vec!["nope".into()], ..LimbHint::default() }], ..RigHints::default() };
    assert_eq!(detect_profile(&s, &bad), Err(RigError::UnknownJoint("nope".into())));
}

#[test]
fn two_bone_ankle_reaches_without_flips_and_keeps_the_metatarsal() {
    let s = quadruped(1.0);
    let p = detect_profile(&s, &RigHints::default()).unwrap();
    let hind = limb(&p, "leg_bl");
    let joints: Vec<usize> = hind.joints.iter().map(|&j| usize::from(j)).collect();
    let [thigh, shin, hock, paw] = [joints[0], joints[1], joints[2], joints[3]];
    let rest = LocalPose::rest(&s);
    let mut base = ModelPose::new(s.len());
    local_to_model(&s, &rest, &mut base, &Affine3A::IDENTITY);
    let animated_angle = (base.translation(hock) - base.translation(shin)).angle_between(base.translation(paw) - base.translation(hock));
    let hip = base.translation(thigh);
    let foot = base.translation(paw);
    let mut worst = 0.0f32;
    for k in 0..200 {
        let a = k as f32 * 0.37;
        // Targets around the animated foot, from deep compression to near full reach.
        let target = foot + Vec3::new(0.08 * a.sin(), 0.35 * (a * 0.7).sin().abs() - 0.05, 0.15 * (a * 1.3).cos());
        let (mut local, mut model) = (rest.clone(), base.clone());
        let pole = limb_pole(&model, &joints, Some(Vec3::Z));
        let d = solve_limb(
            &s,
            &mut local,
            &mut model,
            &Affine3A::IDENTITY,
            &joints,
            hind.solver,
            &LimbGoal { position: target, rotation: None, pole, weight: 1.0 },
        );
        let reach = hip.distance(target) < 0.95 * hind.length;
        if reach {
            worst = worst.max(d);
        }
        // No flip: the knee stays in front of the hip-to-paw line.
        let (h, kn, f) = (model.translation(thigh), model.translation(shin), model.translation(paw));
        let line = (f - h).normalize();
        let off = (kn - h) - line * (kn - h).dot(line);
        assert!(off.z > 0.0, "target {target}: knee flipped ({off})");
        // At or beyond the animated extension the metatarsal keeps its angle to the shin.
        if hip.distance(target) >= hip.distance(foot) {
            let angle = (model.translation(hock) - model.translation(shin)).angle_between(model.translation(paw) - model.translation(hock));
            assert!((angle - animated_angle).abs() < 5f32.to_radians(), "metatarsal {angle} vs {animated_angle}");
        }
    }
    assert!(worst < 1e-3, "worst reachable miss {worst}");
}

#[test]
fn hand_ik_matches_rotation_and_chains_use_fabrik_and_ccd() {
    let s = biped();
    let rest = LocalPose::rest(&s);
    let mut base = ModelPose::new(s.len());
    local_to_model(&s, &rest, &mut base, &Affine3A::IDENTITY);
    let arm: Vec<usize> = ["upperarm_l", "forearm_l", "hand_l"].iter().map(|n| s.find(n).unwrap()).collect();
    let target = Vec3::new(0.45, 1.25, 0.3);
    let rot = Quat::from_rotation_y(0.7) * Quat::from_rotation_x(-0.4);
    for solver in [LimbSolver::TwoBone, LimbSolver::Fabrik, LimbSolver::Ccd] {
        let (mut local, mut model) = (rest.clone(), base.clone());
        let pole = limb_pole(&model, &arm, Some(Vec3::NEG_Z));
        let d = solve_limb(
            &s,
            &mut local,
            &mut model,
            &Affine3A::IDENTITY,
            &arm,
            solver,
            &LimbGoal { position: target, rotation: Some(rot), pole, weight: 1.0 },
        );
        assert!(d < 2e-3, "{solver:?}: miss {d}");
        assert!(angle_between(model.rotation(arm[2]), rot) < 1e-3, "{solver:?}: rotation");
        // The local pose agrees with the model pose.
        let mut check = ModelPose::new(s.len());
        local_to_model(&s, &local, &mut check, &Affine3A::IDENTITY);
        assert!(check.translation(arm[2]).distance(model.translation(arm[2])) < 1e-4);
    }
}

#[test]
fn contacts_come_from_support_limbs_scaled_by_leg_length() {
    let legs = [("thigh_l", "paw_l", 0.0), ("thigh_r", "paw_r", 0.5), ("scapula_l", "wrist_l", 0.25), ("scapula_r", "wrist_r", 0.75)];
    let rules = ClipRules {
        looping: Looping::Whole,
        contacts: ContactRules::default(),
        root_motion: RootMotionRules { mode: RootMotionMode::Match, ..RootMotionRules::default() },
        ..ClipRules::default()
    };
    let mut patterns = Vec::new();
    for scale in [1.0f32, 0.25] {
        let s = quadruped(scale);
        let p = detect_profile(&s, &RigHints::default()).unwrap();
        let mut src = gait(&s, "walk", 1.0, 1.0 * scale, 0.35, 0.6, &legs);
        src.curves.clear(); // detected, not authored
        let clip = cook_clip_for(&s, Some(&p), &src, &rules, &CookSettings::default()).unwrap();
        let names: Vec<&str> = clip.curves.iter().map(|c| c.name.as_str()).collect();
        for l in p.support_limbs() {
            assert!(names.contains(&l.foot.as_ref().unwrap().contact.as_str()), "{names:?}");
        }
        assert_eq!(names.len(), 4);
        patterns.push(clip.curves.iter().map(|c| c.values.clone()).collect::<Vec<_>>());
        // Without a profile, contact-tagged joints are used (none tagged here
        // but the front `wrist`s are hands): nothing for the hind paws.
        let plain = cook_clip(&s, &src, &rules, &CookSettings::default()).unwrap();
        assert!(plain.curves.iter().all(|c| !c.name.contains("paw")));
    }
    // A quarter-size creature with a quarter-speed gait plants on the same frames.
    assert_eq!(patterns[0], patterns[1]);
    // Contact-tagged joints count without a profile.
    let s = quadruped(1.0);
    let tagged: Vec<(String, String)> = ["paw_l", "paw_r"].iter().map(|j| ((*j).to_owned(), tag::CONTACT.to_owned())).collect();
    let mut joints: Vec<SourceJoint> =
        (0..s.len()).map(|j| SourceJoint { name: s.name(j).to_owned(), parent: s.parent(j), rest: s.rest()[j] }).collect();
    joints.truncate(s.len());
    let (s2, _) =
        Skeleton::with_rules(&joints, &SkeletonRules { tags: tagged, root_motion_joint: Some("root".into()), ..SkeletonRules::default() }).unwrap();
    let src = gait(&s2, "walk", 1.0, 1.0, 0.35, 0.6, &legs);
    let clip = cook_clip(&s2, &src, &rules, &CookSettings::default()).unwrap();
    assert!(clip.find_curve("contact.paw_l").is_some() && clip.find_curve("contact.paw_r").is_some());
}

#[test]
fn flyer_root_motion_keeps_pitch_and_roll() {
    let s = flyer();
    let d = 2.0;
    let pitch = |t: f32| 0.4 * (TAU * t / d).sin();
    let roll = |t: f32| -0.3 * (TAU * t / d).sin();
    let src = SourceClip {
        name: "dive".into(),
        duration: d,
        tracks: vec![SourceTrack {
            joint: "root".into(),
            translation: Some(sampled(d, |t| Vec3::new(0.0, -0.5 * t, 4.0 * t))),
            rotation: Some(sampled(d, move |t| Quat::from_euler(glam::EulerRot::YXZ, 0.2 * t, pitch(t), roll(t)))),
            ..SourceTrack::default()
        }],
        ..SourceClip::default()
    };
    let rules = ClipRules {
        root_motion: RootMotionRules { mode: RootMotionMode::Drive, vertical: true, attitude: true, ..RootMotionRules::default() },
        contacts: ContactRules { enabled: false, ..ContactRules::default() },
        ..ClipRules::default()
    };
    let clip = cook_clip(&s, &src, &rules, &CookSettings::default()).unwrap();
    assert!(clip.meta.max_error <= CookSettings::default().error_bound, "error {}", clip.meta.max_error);
    for t in [0.3f32, 0.9, 1.6] {
        let a = clip.attitude_at(t);
        assert!((a.x - pitch(t)).abs() < 2e-3 && (a.y - roll(t)).abs() < 2e-3, "t {t}: {a}");
    }
    let delta = clip.root_delta(0.3, 0.9);
    assert!((delta.tilt.x - (pitch(0.9) - pitch(0.3))).abs() < 3e-3);
    assert!((delta.yaw - 0.2 * 0.6).abs() < 2e-3);
    // The root joint carries no attitude left: it stays level in the root's frame.
    let mut pose = LocalPose::new(s.len());
    clip.sample(0.5, &mut pose, None);
    assert!(angle_between(pose.get(s.find("root").unwrap()).rotation, Quat::IDENTITY) < 2e-3);
    // Ground creatures are unchanged: no attitude track.
    let ground = cook_clip(
        &s,
        &src,
        &ClipRules { root_motion: RootMotionRules { attitude: false, ..rules.root_motion.clone() }, ..rules },
        &CookSettings::default(),
    )
    .unwrap();
    assert!(ground.root_motion.unwrap().attitude.is_empty());
}

#[test]
fn retarget_ik_pins_four_joint_legs_by_limb_scale() {
    let (src_s, dst_s) = (quadruped(1.0), quadruped(1.3));
    let rig = |s: &Skeleton| {
        let p = detect_profile(s, &RigHints::default()).unwrap();
        RetargetRig {
            chains: p
                .limbs
                .iter()
                .map(|l| RetargetChain {
                    name: l.name.clone(),
                    joints: l.joints.iter().map(|&j| s.name(usize::from(j)).to_owned()).collect(),
                    ik: l.support,
                    solver: Some(l.solver),
                })
                .chain(std::iter::once(RetargetChain { name: "body".into(), joints: vec!["pelvis".into()], ik: false, solver: None }))
                .collect(),
            retarget_pose: Vec::new(),
            hips: Some("pelvis".into()),
        }
    };
    let legs = [("thigh_l", "paw_l", 0.0), ("thigh_r", "paw_r", 0.5), ("scapula_l", "wrist_l", 0.25), ("scapula_r", "wrist_r", 0.75)];
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let p = detect_profile(&src_s, &RigHints::default()).unwrap();
    let clip = cook_clip_for(&src_s, Some(&p), &gait(&src_s, "walk", 1.0, 1.0, 0.35, 0.6, &legs), &rules, &CookSettings::default()).unwrap();
    let r = Retargeter::new(&src_s, &rig(&src_s), &dst_s, &rig(&dst_s)).unwrap();
    assert!((r.scale() - 1.3).abs() < 1e-4);
    let out = r.clip(&clip, &CookSettings::default()).unwrap();
    // During contact the target paws sit at the source paws × 1.3.
    let (mut sp, mut tp) = (LocalPose::new(src_s.len()), LocalPose::new(dst_s.len()));
    let (mut sm, mut tm) = (ModelPose::new(src_s.len()), ModelPose::new(dst_s.len()));
    let contact = clip.find_curve("contact.paw_l").unwrap();
    let (sj, tj) = (src_s.find("paw_l").unwrap(), dst_s.find("paw_l").unwrap());
    let mut checked = 0;
    for k in 0..20 {
        let t = k as f32 * 0.05;
        if clip.curve(contact, t) < 0.99 {
            continue;
        }
        clip.sample(t, &mut sp, None);
        out.sample(t, &mut tp, None);
        local_to_model(&src_s, &sp, &mut sm, &Affine3A::IDENTITY);
        local_to_model(&dst_s, &tp, &mut tm, &Affine3A::IDENTITY);
        let want = sm.translation(sj) * 1.3;
        assert!(tm.translation(tj).distance(want) < 0.01, "t {t}: {} vs {want}", tm.translation(tj));
        checked += 1;
    }
    assert!(checked > 5);
}
