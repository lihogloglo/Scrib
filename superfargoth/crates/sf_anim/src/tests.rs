//! Unit tests (animation.md and character.md, "Verification").
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]

use std::collections::HashMap;
use std::f32::consts::TAU;

use glam::{Affine3A, EulerRot, Quat, Vec3};
use rayon::prelude::*;

use crate::cook::model_of;
use crate::ik::{Aim, AimJoint, TwoBone};
use crate::math::affine_rotation;
use crate::*;

// --- Fixtures ----------------------------------------------------------------

fn j(name: &str, parent: Option<usize>, t: [f32; 3]) -> SourceJoint {
    SourceJoint { name: name.to_owned(), parent, rest: Trs::from_translation(Vec3::from(t)) }
}

/// Rest rotation of the upper arm: an A-pose.
const A_POSE: f32 = -0.6;

/// A small biped, Y up, facing +Z, in meters, arms in an A-pose.
fn biped_joints() -> Vec<SourceJoint> {
    let mut v = vec![
        j("root", None, [0.0, 0.0, 0.0]),
        j("hips", Some(0), [0.0, 1.0, 0.0]),
        j("spine", Some(1), [0.0, 0.1, 0.0]),
        j("chest", Some(2), [0.0, 0.25, 0.0]),
        j("head", Some(3), [0.0, 0.3, 0.0]),
        j("thigh_l", Some(1), [0.1, -0.05, 0.0]),
        j("shin_l", Some(5), [0.0, -0.45, 0.0]),
        j("foot_l", Some(6), [0.0, -0.45, 0.0]),
        j("thigh_r", Some(1), [-0.1, -0.05, 0.0]),
        j("shin_r", Some(8), [0.0, -0.45, 0.0]),
        j("foot_r", Some(9), [0.0, -0.45, 0.0]),
        j("upperarm_l", Some(3), [0.15, 0.2, 0.0]),
        j("forearm_l", Some(11), [0.28, 0.0, 0.0]),
        j("hand_l", Some(12), [0.25, 0.0, 0.0]),
        j("finger_l", Some(13), [0.08, 0.0, 0.0]),
    ];
    v[11].rest.rotation = Quat::from_rotation_z(A_POSE);
    v
}

fn biped() -> Skeleton {
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&biped_joints(), &rules).unwrap().0
}

fn idx(s: &Skeleton, name: &str) -> usize {
    s.find(name).unwrap_or_else(|| panic!("no joint {name}"))
}

/// A curve keyed every 0.1 s over one second.
fn sampled<T: CurveValue>(f: impl Fn(f32) -> T) -> Curve<T> {
    let times: Vec<f32> = (0..=10).map(|i| i as f32 / 10.0).collect();
    let values = times.iter().map(|&t| f(t)).collect();
    Curve::linear(times, values)
}

fn rot(joint: &str, f: impl Fn(f32) -> Quat) -> SourceTrack {
    SourceTrack { joint: joint.into(), rotation: Some(sampled(f)), ..SourceTrack::default() }
}

fn trans(joint: &str, f: impl Fn(f32) -> Vec3) -> SourceTrack {
    SourceTrack { joint: joint.into(), translation: Some(sampled(f)), ..SourceTrack::default() }
}

fn wave(t: f32) -> f32 {
    (TAU * t).sin()
}

/// One walk cycle: the root travels +Z at 1.2 m/s, legs and arm swing.
fn walk_source() -> SourceClip {
    SourceClip {
        name: "walk".into(),
        duration: 1.0,
        tracks: vec![
            trans("root", |t| Vec3::new(0.0, 0.0, 1.2 * t)),
            trans("hips", |t| Vec3::new(0.0, 1.0 + 0.03 * (2.0 * TAU * t).cos(), 0.0)),
            rot("hips", |t| Quat::from_rotation_y(0.1 * wave(t))),
            rot("spine", |t| Quat::from_rotation_x(0.1 * wave(t))),
            rot("thigh_l", |t| Quat::from_rotation_x(0.5 * wave(t))),
            rot("shin_l", |t| Quat::from_rotation_x(0.4 * (1.0 - (TAU * t).cos()))),
            rot("thigh_r", |t| Quat::from_rotation_x(-0.5 * wave(t))),
            rot("shin_r", |t| Quat::from_rotation_x(0.4 * (1.0 + (TAU * t).cos()))),
            rot("upperarm_l", |t| Quat::from_rotation_z(A_POSE) * Quat::from_rotation_x(0.3 * wave(t))),
            rot("forearm_l", |t| Quat::from_rotation_y(0.2 + 0.1 * wave(t))),
        ],
        ..SourceClip::default()
    }
}

fn no_root_motion() -> ClipRules {
    ClipRules { root_motion: RootMotionRules { extract: false, ..RootMotionRules::default() }, ..ClipRules::default() }
}

fn cook(s: &Skeleton, src: &SourceClip, rules: &ClipRules) -> Clip {
    cook_clip(s, src, rules, &CookSettings::default()).unwrap()
}

fn model(s: &Skeleton, pose: &LocalPose) -> ModelPose {
    let mut m = ModelPose::new(s.len());
    local_to_model(s, pose, &mut m, &Affine3A::IDENTITY);
    m
}

fn model_trs(s: &Skeleton, pose: &[Trs]) -> Vec<Affine3A> {
    let mut m = Vec::new();
    model_of(s, pose, &mut m);
    m
}

/// Deterministic pseudo-random local poses.
fn random_pose(n: usize, seed: u64) -> Vec<Trs> {
    let mut x = seed;
    let mut rnd = move || {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((x >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    };
    (0..n)
        .map(|_| Trs {
            translation: Vec3::new(rnd(), rnd(), rnd()),
            rotation: Quat::from_euler(EulerRot::XYZ, rnd() * 3.0, rnd() * 3.0, rnd() * 3.0),
            scale: Vec3::new(1.0 + 0.3 * rnd(), 1.0 + 0.3 * rnd(), 1.0 + 0.3 * rnd()),
        })
        .collect()
}

fn assert_trs_close(got: Trs, want: Trs, eps: f32, what: &str) {
    assert!(got.translation.distance(want.translation) < eps, "{what}: translation {got:?} vs {want:?}");
    assert!(angle_between(got.rotation, want.rotation) < eps, "{what}: rotation {got:?} vs {want:?}");
    assert!(got.scale.distance(want.scale) < eps, "{what}: scale {got:?} vs {want:?}");
}

// --- Skeleton ------------------------------------------------------------------

#[test]
fn skeleton_is_depth_first_with_contiguous_subtrees() {
    // Children before parents in the source.
    let joints = [j("c", Some(2), [0.0; 3]), j("b", Some(2), [0.0; 3]), j("a", None, [0.0; 3]), j("d", Some(0), [0.0; 3])];
    let (s, remap) = Skeleton::new(&joints).unwrap();
    assert_eq!(s.names(), ["a", "c", "d", "b"]);
    for (old, &new) in remap.iter().enumerate() {
        assert_eq!(s.name(new), joints[old].name);
    }
    assert_eq!(s.subtree(idx(&s, "c")), 1..3);
    assert_eq!(s.subtree(0), 0..4);
    let b = biped();
    for jn in 0..b.len() {
        if let Some(p) = b.parent(jn) {
            assert!(p < jn, "parent precedes child");
            assert!(b.is_ancestor(p, jn));
        }
    }
}

#[test]
fn skeleton_rejects_bad_hierarchies() {
    assert_eq!(Skeleton::new(&[]).unwrap_err(), SkeletonError::Empty);
    let dup = [j("a", None, [0.0; 3]), j("a", Some(0), [0.0; 3])];
    assert_eq!(Skeleton::new(&dup).unwrap_err(), SkeletonError::DuplicateName("a".into()));
    let cycle = [j("r", None, [0.0; 3]), j("x", Some(2), [0.0; 3]), j("y", Some(1), [0.0; 3])];
    assert!(matches!(Skeleton::new(&cycle).unwrap_err(), SkeletonError::BadParent(_)));
    let own = [j("x", Some(0), [0.0; 3])];
    assert!(matches!(Skeleton::new(&own).unwrap_err(), SkeletonError::BadParent(_)));
    let rules = SkeletonRules { root_motion_joint: Some("nope".into()), ..SkeletonRules::default() };
    assert_eq!(Skeleton::with_rules(&biped_joints(), &rules).unwrap_err(), SkeletonError::UnknownJoint("nope".into()));
}

#[test]
fn skeleton_tags_and_lod_masks() {
    for (name, want) in [
        ("LeftFoot", Some(tag::FOOT)),
        ("hand.R", Some(tag::HAND)),
        ("Finger0Nub", Some(tag::END)),
        ("upperarm_twist_01", Some(tag::TWIST)),
        ("Bip01 Pelvis", Some(tag::HIPS)),
        ("mixamorig:Spine1", Some(tag::SPINE)),
        ("Bip01 L Toe0", Some(tag::TOE)),
        ("thigh_l", None),
    ] {
        assert_eq!(guess_tag(name), want, "{name}");
    }
    let s = biped();
    let feet: Vec<&str> = s.joints_with_tag(tag::FOOT).map(|j| s.name(j)).collect();
    assert_eq!(feet, ["foot_l", "foot_r"]);
    assert_eq!(s.root_motion_joint(), Some(idx(&s, "root")));
    let lod1 = s.lod_mask(1).unwrap();
    assert!(!lod1.contains(idx(&s, "finger_l")), "fingers drop at LOD1");
    assert!(lod1.contains(idx(&s, "hand_l")));
    assert_eq!(s.lod_mask(0).unwrap().count(), s.len());
    assert!(s.lod_mask(3).is_none());
    for level in 0..3 {
        let m = s.lod_mask(level).unwrap();
        for jn in m.iter() {
            assert!(s.parent(jn).is_none_or(|p| m.contains(p)), "LOD{level} mask closed under parent");
        }
    }
    let keep = SkeletonRules { lod: LodRules { keep: vec!["finger_l".into()], ..LodRules::default() }, ..SkeletonRules::default() };
    let kept = Skeleton::with_rules(&biped_joints(), &keep).unwrap().0;
    assert!(kept.lod_mask(2).unwrap().contains(idx(&kept, "finger_l")));
}

#[test]
fn skeleton_id_and_bytes() {
    let s = biped();
    let bytes = s.to_bytes();
    let back = Skeleton::from_bytes(&bytes).unwrap();
    assert_eq!(back, s);
    assert_eq!(back.to_bytes(), bytes);
    // The id follows topology, not the rest pose.
    let mut moved = biped_joints();
    moved[1].rest.translation.y = 0.9;
    assert_eq!(Skeleton::new(&moved).unwrap().0.id(), s.id());
    let mut renamed = biped_joints();
    renamed[2].name = "spine1".into();
    assert_ne!(Skeleton::new(&renamed).unwrap().0.id(), s.id());
    let mut bad = bytes.clone();
    bad[0] = b'X';
    assert_eq!(Skeleton::from_bytes(&bad).unwrap_err(), FormatError::Magic);
    let mut old = bytes;
    old[4] = old[4].wrapping_add(1);
    assert!(matches!(Skeleton::from_bytes(&old).unwrap_err(), FormatError::Version { .. }));
}

// --- Pose ops ------------------------------------------------------------------

#[test]
fn local_to_model_matches_reference_and_subtree_update() {
    let s = biped();
    let rest = LocalPose::rest(&s);
    let m = model(&s, &rest);
    for (a, b) in m.mats.iter().zip(s.rest_model()) {
        assert!(a.abs_diff_eq(b, 1e-6));
    }
    // Under a root transform.
    let root = Affine3A::from_translation(Vec3::new(5.0, 0.0, 0.0));
    let mut shifted = ModelPose::new(s.len());
    local_to_model(&s, &rest, &mut shifted, &root);
    assert!((shifted.translation(idx(&s, "head")) - m.translation(idx(&s, "head")) - Vec3::X * 5.0).length() < 1e-6);
    // A subtree update equals a full pass.
    let mut pose = LocalPose::from_trs(&random_pose(s.len(), 7));
    let mut full = model(&s, &pose);
    let thigh = idx(&s, "thigh_l");
    pose.set_rotation(thigh, Quat::from_rotation_x(0.7));
    update_subtree(&s, &pose, &mut full, &Affine3A::IDENTITY, thigh);
    let reference = model_trs(&s, &pose.to_vec());
    for (a, b) in full.mats.iter().zip(&reference) {
        assert!(a.abs_diff_eq(*b, 1e-5));
    }
}

#[test]
fn blend_matches_per_joint_lerp() {
    // Seven joints: the last SIMD group is partly padding.
    let (a, b) = (random_pose(7, 1), random_pose(7, 2));
    let (pa, pb) = (LocalPose::from_trs(&a), LocalPose::from_trs(&b));
    let mut out = LocalPose::new(0);
    blend(&mut out, &pa, &pb, 0.3);
    assert_eq!(out.len(), 7);
    for jn in 0..7 {
        let want = Trs {
            translation: a[jn].translation.lerp(b[jn].translation, 0.3),
            rotation: nlerp(a[jn].rotation, b[jn].rotation, 0.3),
            scale: a[jn].scale.lerp(b[jn].scale, 0.3),
        };
        assert_trs_close(out.get(jn), want, 1e-5, "blend");
    }
    // Per-joint weights: 0 keeps a, 1 takes b.
    let mut w = JointWeights::splat(7, 0.0);
    w.set(2, 1.0);
    w.set(3, 0.5);
    let mut x = pa.clone();
    blend_in_place(&mut x, &pb, 1.0, Some(&w));
    assert_trs_close(x.get(0), a[0], 1e-6, "weight 0");
    assert_trs_close(x.get(2), b[2], 1e-5, "weight 1");
    assert!(angle_between(x.get(3).rotation, nlerp(a[3].rotation, b[3].rotation, 0.5)) < 1e-5);
    let mask = JointWeights::from_mask(
        &{
            let mut m = JointMask::empty(7);
            m.insert(4);
            m
        },
        1.0,
        0.0,
    );
    assert_eq!((mask.get(4), mask.get(5)), (1.0, 0.0));
}

#[test]
fn blend_layers_normalizes_and_fills_rest() {
    let (a, b, r) = (random_pose(9, 3), random_pose(9, 4), random_pose(9, 5));
    let (pa, pb, rest) = (LocalPose::from_trs(&a), LocalPose::from_trs(&b), LocalPose::from_trs(&r));
    let mut wa = JointWeights::splat(9, 1.0);
    wa.set(5, 0.0);
    let mut wb = JointWeights::splat(9, 1.0);
    wb.set(5, 0.0);
    let layers = [BlendLayer { pose: &pa, weight: 1.0, joint_weights: Some(&wa) }, BlendLayer { pose: &pb, weight: 3.0, joint_weights: Some(&wb) }];
    let mut out = LocalPose::new(0);
    blend_layers(&mut out, &layers, &rest, 0.1);
    let mut two = LocalPose::new(0);
    blend(&mut two, &pa, &pb, 0.75);
    for jn in (0..9).filter(|&jn| jn != 5) {
        assert_trs_close(out.get(jn), two.get(jn), 1e-5, "weights 1:3");
    }
    assert_trs_close(out.get(5), r[5], 1e-5, "undriven joint keeps rest");
}

#[test]
fn additive_round_trips_and_cancels() {
    let (p, base) = (random_pose(6, 8), random_pose(6, 9));
    let (pp, pbase) = (LocalPose::from_trs(&p), LocalPose::from_trs(&base));
    let mut delta = LocalPose::new(0);
    make_additive(&mut delta, &pp, &pbase);
    let mut x = pbase.clone();
    apply_additive(&mut x, &delta, 1.0, None);
    for (jn, want) in p.iter().enumerate() {
        assert_trs_close(x.get(jn), *want, 1e-4, "base + (pose - base) = pose");
    }
    let mut y = pbase.clone();
    apply_additive(&mut y, &delta, 0.0, None);
    for (jn, want) in base.iter().enumerate() {
        assert_trs_close(y.get(jn), *want, 1e-6, "weight 0");
    }
    // A pose's delta over itself is the identity.
    let mut zero = LocalPose::new(0);
    make_additive(&mut zero, &pp, &pp);
    for jn in 0..6 {
        assert_trs_close(zero.get(jn), Trs::IDENTITY, 1e-5, "self delta");
    }
}

// --- Source curves ---------------------------------------------------------------

#[test]
fn source_curves_step_linear_and_cubic() {
    let step = Curve { times: vec![0.0, 1.0], values: vec![1.0f32, 3.0], interpolation: Interpolation::Step };
    assert_eq!((step.sample(-1.0), step.sample(0.99), step.sample(1.0)), (1.0, 1.0, 3.0));
    let lin = Curve::linear(vec![0.0, 2.0], vec![0.0f32, 4.0]);
    assert_eq!((lin.sample(0.5), lin.sample(3.0)), (1.0, 4.0));
    // Zero tangents: smoothstep.
    let ease = Curve { times: vec![0.0, 1.0], values: vec![0.0f32, 0.0, 0.0, 0.0, 1.0, 0.0], interpolation: Interpolation::CubicSpline };
    assert!(ease.is_valid());
    assert!((ease.sample(0.25) - 0.156_25).abs() < 1e-6);
    // Tangents are per second: slope-2 keys half a second apart make a line.
    let line = Curve { times: vec![0.0, 0.5], values: vec![2.0f32, 0.0, 2.0, 2.0, 1.0, 2.0], interpolation: Interpolation::CubicSpline };
    assert!((line.sample(0.25) - 0.5).abs() < 1e-6);
    assert!(!Curve::linear(vec![1.0, 1.0], vec![0.0f32, 1.0]).is_valid());
    let q = Curve::linear(vec![0.0, 1.0], vec![Quat::IDENTITY, Quat::from_rotation_y(2.0)]);
    assert!(angle_between(q.sample(0.5), Quat::from_rotation_y(1.0)) < 1e-5, "slerp");
    let flipped = Curve::linear(vec![0.0, 1.0], vec![Quat::IDENTITY, -Quat::from_rotation_y(0.4)]);
    assert!(angle_between(flipped.sample(0.5), Quat::from_rotation_y(0.2)) < 1e-5, "short arc");
}

// --- Cooking and sampling ---------------------------------------------------------

#[test]
fn cooked_clip_samples_the_source_within_the_bound() {
    let s = biped();
    let src = walk_source();
    let clip = cook(&s, &src, &no_root_motion());
    assert!(clip.meta.max_error <= CookSettings::default().error_bound, "error {}", clip.meta.max_error);
    assert_eq!(clip.rate, 30.0, "the 10 Hz source cooks at the lowest rate");
    assert_eq!(clip.frame_count, 31);
    assert!(clip.meta.unbound_joints.is_empty());
    let (rotations, translations, scales, raw) = clip.track_counts();
    assert_eq!((rotations, translations, scales, raw), (8, 2, 0, 0), "static channels are constants");
    let bound = BoundSource::new(&src, &s);
    let mut truth = vec![Trs::IDENTITY; s.len()];
    let mut pose = LocalPose::new(s.len());
    for i in 0..=37 {
        let t = i as f32 / 37.0;
        bound.sample(t, s.rest(), &mut truth);
        clip.sample(t, &mut pose, None);
        for (jn, want) in truth.iter().enumerate() {
            assert_trs_close(pose.get(jn), *want, 1e-3, s.name(jn));
        }
    }
    // A masked sample matches the full one inside the mask.
    let mask = s.lod_mask(1).unwrap();
    let mut masked = LocalPose::new(s.len());
    clip.sample(0.42, &mut masked, Some(mask));
    clip.sample(0.42, &mut pose, None);
    for jn in mask.iter() {
        assert_eq!(masked.get(jn), pose.get(jn));
    }
}

#[test]
fn cook_is_deterministic_and_round_trips() {
    let s = biped();
    let src = walk_source();
    let rules = ClipRules { looping: Looping::Whole, ..ClipRules::default() };
    let run = |threads: usize| {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| (0..6).into_par_iter().map(|_| cook(&s, &src, &rules).to_bytes()).collect::<Vec<_>>())
    };
    let one = run(1);
    let many = run(4);
    assert!(one.iter().chain(&many).all(|b| *b == one[0]), "bit-identical across runs and thread counts");
    let clip = Clip::from_bytes(&one[0]).unwrap();
    assert_eq!(clip.to_bytes(), one[0]);
    let mut bad = one[0].clone();
    bad[5] ^= 1;
    assert!(matches!(Clip::from_bytes(&bad).unwrap_err(), FormatError::Version { .. }));
    assert!(Clip::from_bytes(&one[0][..20]).is_err());
}

#[test]
fn root_motion_is_extracted() {
    let s = biped();
    let mut src = walk_source();
    // Turn while walking: 0.5 rad over the clip.
    src.tracks.push(rot("root", |t| Quat::from_rotation_y(0.5 * t)));
    // Extraction alone: the contact-consistent root (AN-19) re-derives a stance root on purpose.
    let contacts = ContactRules { root_blend: 0.0, ..ContactRules::default() };
    let rules = ClipRules { looping: Looping::Whole, contacts, ..ClipRules::default() };
    let clip = cook(&s, &src, &rules);
    assert!(clip.meta.max_error <= 1e-4, "error {}", clip.meta.max_error);
    let (p, yaw) = clip.root_at(1.0);
    assert!(p.distance(Vec3::new(0.0, 0.0, 1.2)) < 1e-4, "{p}");
    assert!((yaw - 0.5).abs() < 1e-4);
    assert!(clip.meta.average_velocity.distance(Vec3::new(0.0, 0.0, 1.2)) < 1e-3);
    assert!((clip.meta.turn_rate - 0.5).abs() < 1e-3);
    assert!((clip.meta.cycle_distance - 1.2).abs() < 1e-3);
    assert_eq!(clip.meta.root_motion_mode, RootMotionMode::Match);
    // The root joint stays where it started, facing where it started.
    let mut pose = LocalPose::new(s.len());
    for t in [0.0, 0.33, 0.5, 1.0] {
        clip.sample(t, &mut pose, None);
        let root = pose.get(idx(&s, "root"));
        assert!(root.translation.length() < 1e-4, "t={t}: {}", root.translation);
        assert!(angle_between(root.rotation, Quat::IDENTITY) < 1e-4);
    }
    // Two and a half loops: the deltas compose along the turning path.
    let adv = clip.advance(0.0, 2.5, true, true, None);
    assert_eq!(adv.wraps, 2);
    assert!((adv.time - 0.5).abs() < 1e-4);
    let mut expected = RootDelta::ZERO;
    for _ in 0..2 {
        expected = expected.then(clip.root_delta(0.0, 1.0));
    }
    expected = expected.then(clip.root_delta(0.0, 0.5));
    assert!(adv.root.translation.distance(expected.translation) < 1e-4);
    assert!((adv.root.yaw - 1.25).abs() < 1e-3);
    // In-place source, Ignore mode: no track.
    let ignore = ClipRules { root_motion: RootMotionRules { mode: RootMotionMode::Ignore, ..RootMotionRules::default() }, ..ClipRules::default() };
    let still = SourceClip { tracks: vec![rot("spine", Quat::from_rotation_x)], ..walk_source() };
    assert_eq!(cook(&s, &still, &ignore).meta.root_motion_mode, RootMotionMode::Ignore);
}

fn event_clip() -> Clip {
    let s = biped();
    let src = SourceClip { name: "e".into(), duration: 1.0, tracks: vec![rot("spine", |t| Quat::from_rotation_x(0.2 * t))], ..SourceClip::default() };
    let rules = ClipRules {
        looping: Looping::Whole,
        events: vec![Event::point(0.0, "start"), Event::point(0.25, "l"), Event::window(0.4, 0.6, "hit"), Event::point(0.75, "r")],
        ..no_root_motion()
    };
    cook(&s, &src, &rules)
}

/// Per event: (points, enters, exits).
fn tally(clip: &Clip, fired: &[FiredEvent]) -> HashMap<String, (u32, u32, u32)> {
    let mut out: HashMap<String, (u32, u32, u32)> = HashMap::new();
    for f in fired {
        let e = out.entry(clip.events[f.event as usize].name.clone()).or_default();
        match f.edge {
            EventEdge::Point => e.0 += 1,
            EventEdge::Enter => e.1 += 1,
            EventEdge::Exit => e.2 += 1,
        }
    }
    out
}

#[test]
fn events_fire_once_across_wraps_and_in_reverse() {
    let clip = event_clip();
    assert_eq!(clip.events.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["start", "l", "hit", "r"]);
    let play = |start: f32, step: f32, steps: usize| {
        let (mut t, mut fired, mut wraps) = (start, Vec::new(), 0);
        for i in 0..steps {
            let a = clip.advance(t, step, true, i == 0, Some(&mut fired));
            t = a.time;
            wraps += a.wraps;
        }
        (tally(&clip, &fired), wraps)
    };
    // Forward over [0, 2.1]: "start" at 0, 1, 2.
    let (f, wraps) = play(0.0, 0.07, 30);
    assert_eq!(wraps, 2);
    assert_eq!(f["start"], (3, 0, 0));
    assert_eq!(f["l"], (2, 0, 0));
    assert_eq!(f["r"], (2, 0, 0));
    assert_eq!(f["hit"], (0, 2, 2));
    // Reverse from the end over [-1.1, 1.0].
    let (b, wraps) = play(1.0, -0.07, 30);
    assert_eq!(wraps, 2);
    assert_eq!(b["start"], (2, 0, 0));
    assert_eq!(b["l"], (2, 0, 0));
    assert_eq!(b["r"], (2, 0, 0));
    assert_eq!(b["hit"], (0, 2, 2));
    let mut fired = Vec::new();
    clip.advance(0.7, -0.4, true, false, Some(&mut fired));
    assert_eq!(fired, [FiredEvent { event: 2, edge: EventEdge::Enter }, FiredEvent { event: 2, edge: EventEdge::Exit }], "reverse enters at the end");
    // A hitch over 3.55 s: whole loops fire nothing, partial loops once.
    let mut fired = Vec::new();
    let hitch = clip.advance(0.1, 3.55, true, false, Some(&mut fired));
    assert_eq!(hitch.wraps, 3);
    assert!((hitch.time - 0.65).abs() < 1e-4);
    let h = tally(&clip, &fired);
    assert_eq!((h["start"].0, h["l"].0, h["r"].0), (1, 2, 1));
    // Not looping: stops at the ends.
    let mut fired = Vec::new();
    let end = clip.advance(0.9, 0.5, false, false, Some(&mut fired));
    assert!(end.finished && end.time == clip.duration && fired.is_empty());
    let start = clip.advance(0.1, -0.5, false, false, Some(&mut fired));
    assert!(start.finished && start.time == 0.0);
    assert_eq!(tally(&clip, &fired)["start"], (1, 0, 0));
    assert_eq!(clip.active_windows(0.5).collect::<Vec<_>>(), [2]);
    assert_eq!(clip.active_windows(0.7).count(), 0);
}

#[test]
fn additive_clip_cancels_at_its_base() {
    let s = biped();
    let spine = idx(&s, "spine");
    let src = SourceClip {
        name: "lean".into(),
        duration: 1.0,
        tracks: vec![rot("spine", |t| Quat::from_rotation_x(0.4 * t)), trans("hips", |t| Vec3::new(0.0, 1.0 - 0.1 * t, 0.0))],
        ..SourceClip::default()
    };
    let rules = ClipRules { additive: Some(AdditiveBase::FirstFrame), ..ClipRules::default() };
    let clip = cook(&s, &src, &rules);
    assert!(clip.meta.additive && clip.root_motion.is_none());
    assert!(clip.meta.max_error <= 1e-4, "error {}", clip.meta.max_error);
    let mut delta = LocalPose::new(s.len());
    clip.sample(0.0, &mut delta, None);
    for jn in 0..s.len() {
        assert_trs_close(delta.get(jn), Trs::IDENTITY, 1e-4, "delta at the base frame");
    }
    clip.sample(1.0, &mut delta, None);
    let mut pose = LocalPose::rest(&s);
    apply_additive(&mut pose, &delta, 1.0, None);
    assert!(angle_between(pose.get(spine).rotation, Quat::from_rotation_x(0.4)) < 1e-4);
    assert!((pose.get(idx(&s, "hips")).translation.y - 0.9).abs() < 1e-4);
    // Rest as the base.
    let rest_based = cook(&s, &src, &ClipRules { additive: Some(AdditiveBase::Rest), ..ClipRules::default() });
    rest_based.sample(0.5, &mut delta, None);
    assert!(angle_between(delta.get(spine).rotation, Quat::from_rotation_x(0.2)) < 1e-4);
}

#[test]
fn range_shifts_events_curves_and_loop() {
    let s = biped();
    let src = SourceClip {
        name: "take".into(),
        duration: 2.0,
        tracks: vec![SourceTrack {
            joint: "spine".into(),
            rotation: Some(Curve::linear(vec![0.0, 2.0], vec![Quat::IDENTITY, Quat::from_rotation_x(0.4)])),
            ..SourceTrack::default()
        }],
        curves: vec![SourceCurve { name: "c".into(), curve: Curve::linear(vec![0.0, 2.0], vec![0.0, 2.0]) }],
        events: vec![Event::point(0.4, "early"), Event::point(0.7, "a")],
        markers: vec![SyncMarker { time: 1.0, name: "m".into() }],
    };
    let rules =
        ClipRules { range: Some((0.5, 1.5)), looping: Looping::Range(0.75, 1.25), events: vec![Event::window(1.0, 1.6, "w")], ..no_root_motion() };
    let clip = cook(&s, &src, &rules);
    assert!((clip.duration - 1.0).abs() < 1e-6);
    let names: Vec<(&str, f32, Option<f32>)> = clip.events.iter().map(|e| (e.name.as_str(), e.time, e.end)).collect();
    assert_eq!(names.len(), 2);
    assert_eq!(names[0].0, "a");
    assert!((names[0].1 - 0.2).abs() < 1e-5);
    assert_eq!(names[1].0, "w");
    assert!((names[1].1 - 0.5).abs() < 1e-5 && names[1].2 == Some(1.0));
    assert_eq!(clip.markers.len(), 1);
    assert!((clip.markers[0].time - 0.5).abs() < 1e-5);
    let (a, b) = clip.loop_range.unwrap();
    assert!((a - 0.25).abs() < 1e-5 && (b - 0.75).abs() < 1e-5);
    let c = clip.find_curve("c").unwrap();
    assert!((clip.curve(c, 0.0) - 0.5).abs() < 1e-5);
    assert!((clip.curve(c, 1.0) - 1.5).abs() < 1e-5);
    let mut pose = LocalPose::new(s.len());
    clip.sample(0.0, &mut pose, None);
    assert!(angle_between(pose.get(idx(&s, "spine")).rotation, Quat::from_rotation_x(0.1)) < 1e-4);
    // An intro plays into the loop range and then stays in it.
    let adv = clip.advance(0.0, 1.0, true, true, None);
    assert!(adv.wraps >= 1 && (0.25..=0.75).contains(&adv.time));
    assert!(cook_clip(&s, &src, &ClipRules { range: Some((1.0, 0.5)), ..ClipRules::default() }, &CookSettings::default()).is_err());
}

#[test]
fn foot_contacts_become_curves_and_markers() {
    let joints = [j("root", None, [0.0; 3]), j("foot", Some(0), [0.0; 3])];
    let s = Skeleton::with_rules(&joints, &SkeletonRules::default()).unwrap().0;
    let lift = Curve::linear(vec![0.0, 0.3, 0.5, 0.8, 1.0], vec![Vec3::ZERO, Vec3::ZERO, Vec3::new(0.0, 0.2, 0.0), Vec3::ZERO, Vec3::ZERO]);
    let src = SourceClip {
        name: "step".into(),
        duration: 1.0,
        tracks: vec![SourceTrack { joint: "foot".into(), translation: Some(lift), ..SourceTrack::default() }],
        ..SourceClip::default()
    };
    let clip = cook(&s, &src, &ClipRules { looping: Looping::Whole, ..ClipRules::default() });
    let c = clip.find_curve("contact.foot").expect("contact curve");
    let values = &clip.curves[c].values;
    assert_eq!(values[0], 1.0);
    assert_eq!(values[values.len() / 2], 0.0, "lifted mid-clip");
    assert_eq!(*values.last().unwrap(), 1.0);
    assert!(clip.find_curve("roll.foot").is_none(), "no toe, no heel roll");
    assert_eq!(clip.markers.len(), 1, "one touch-down; the loop start is not an onset");
    assert_eq!(clip.markers[0].name, "down.foot");
    assert!((0.72..0.76).contains(&clip.markers[0].time), "{}", clip.markers[0].time);
    let (prev, next, phase) = clip.marker_phase(0.2).unwrap();
    assert_eq!((prev, next), (0, 0));
    assert!((0.0..=1.0).contains(&phase));
}

/// The heel roll curve (`roll.<foot>`): 1 from a stance's heel-off (where
/// the contact curve drops) while the toe is still down, until the toe lifts.
#[test]
fn heel_roll_curve_spans_heel_off_to_toe_off() {
    let joints = [
        j("root", None, [0.0; 3]),
        j("thigh", Some(0), [0.0, 0.9, 0.0]),
        j("shin", Some(1), [0.0, -0.45, 0.0]),
        j("foot", Some(2), [0.0, -0.45, 0.0]),
        j("toe", Some(3), [0.0, 0.0, 0.15]),
    ];
    let s = Skeleton::with_rules(&joints, &SkeletonRules::default()).unwrap().0;
    // The ankle (model space, shin at y = 0.45): still to 0.40 s, rolling over
    // the toe (15 cm ahead) to 0.65 s, then swinging back to its start.
    let roll_end = |th: f32| Vec3::new(0.0, 0.15 * th.sin(), 0.15 * (1.0 - th.cos()));
    let pose = move |t: f32| -> (Vec3, f32) {
        if t < 0.4 {
            (Vec3::ZERO, 0.0)
        } else if t < 0.65 {
            let th = 0.6 * (t - 0.4) / 0.25;
            (roll_end(th), th)
        } else {
            let k = ((t - 0.65) / 0.25).min(1.0);
            (roll_end(0.6) * (1.0 - k) + Vec3::Y * 0.15 * (std::f32::consts::PI * k).sin(), 0.6 * (1.0 - k))
        }
    };
    let src = SourceClip {
        name: "step".into(),
        duration: 1.0,
        tracks: vec![SourceTrack {
            joint: "foot".into(),
            translation: Some(fixtures::sampled(1.0, move |t| pose(t).0 - Vec3::Y * 0.45)),
            rotation: Some(fixtures::sampled(1.0, move |t| Quat::from_rotation_x(pose(t).1))),
            ..SourceTrack::default()
        }],
        ..SourceClip::default()
    };
    let contacts = ContactRules { feet: vec!["foot".into()], ..ContactRules::default() };
    let clip = cook(&s, &src, &ClipRules { looping: Looping::Whole, contacts, ..no_root_motion() });
    let values = |name: &str| clip.curves[clip.find_curve(name).unwrap_or_else(|| panic!("{name}"))].values.clone();
    let (contact, roll) = (values("contact.foot"), values("roll.foot"));
    let time = |f: usize| f as f32 / clip.rate;
    let heel_off = (1..contact.len()).find(|&f| contact[f - 1] > 0.5 && contact[f] <= 0.5).expect("a stance ends");
    let rolling: Vec<usize> = (0..roll.len()).filter(|&f| roll[f] > 0.5).collect();
    assert!(rolling.len() >= 3, "{roll:?}");
    assert_eq!(rolling[0], heel_off, "the roll starts at heel-off: contact {contact:?} roll {roll:?}");
    assert!(rolling.windows(2).all(|w| w[1] == w[0] + 1), "one run: {rolling:?}");
    assert!(rolling.iter().all(|&f| contact[f] <= 0.5), "never while planted");
    let last = time(*rolling.last().unwrap());
    assert!((0.6..0.8).contains(&last), "the roll ends as the toe lifts, not before the heel is up nor in mid-swing: {last}");
    assert!(time(heel_off) < 0.45, "heel-off {}", time(heel_off));
}

#[test]
fn cook_frames_round_trips_decoded_clips() {
    let s = biped();
    let clip = cook(&s, &walk_source(), &ClipRules::default());
    let frames = FrameClip::from_clip(&clip);
    assert_eq!(frames.frames.len(), frames.frame_count * s.len());
    let again = cook_frames(&s, &frames, &CookSettings::default()).unwrap();
    assert!(again.meta.max_error <= 1e-4);
    assert_eq!(again.root_motion, clip.root_motion);
    let (mut a, mut b) = (LocalPose::new(s.len()), LocalPose::new(s.len()));
    clip.sample(0.61, &mut a, None);
    again.sample(0.61, &mut b, None);
    for jn in 0..s.len() {
        assert_trs_close(a.get(jn), b.get(jn), 1e-3, s.name(jn));
    }
    let short = FrameClip { frames: Vec::new(), ..frames };
    assert_eq!(cook_frames(&s, &short, &CookSettings::default()).unwrap_err(), CookError::BadFrames);
}

// --- IK and springs -----------------------------------------------------------------

fn leg() -> Skeleton {
    Skeleton::new(&[j("hip", None, [0.0, 1.0, 0.0]), j("knee", Some(0), [0.0, -0.45, 0.0]), j("ankle", Some(1), [0.0, -0.45, 0.0])]).unwrap().0
}

#[test]
fn two_bone_reaches_and_clamps_without_flipping() {
    let s = leg();
    let pole = Vec3::new(0.0, 0.55, 1.0);
    for i in 0..=60 {
        // Sweep from close in, through full reach, to far out of reach.
        let dist = 0.2 + 1.2 * i as f32 / 60.0;
        let dir = Vec3::new(0.3 * (i as f32 * 0.2).sin(), -1.0, 0.4).normalize();
        let a = Vec3::new(0.0, 1.0, 0.0);
        let target = a + dir * dist;
        let mut local = LocalPose::rest(&s);
        let mut m = model(&s, &local);
        let left = ik::two_bone_ik(&s, &mut local, &mut m, &Affine3A::IDENTITY, [0, 1, 2], target, pole, TwoBone::default());
        let (b, c) = (m.translation(1), m.translation(2));
        assert!((b.distance(a) - 0.45).abs() < 1e-4 && (c.distance(b) - 0.45).abs() < 1e-4, "bone lengths kept");
        if dist < 0.89 {
            assert!(left < 1e-4, "reachable target at {dist}: {left}");
        } else {
            assert!((c.distance(a) - 0.9 * TwoBone::default().max_extension).abs() < 1e-3, "clamped to reach");
            assert!((c - a).normalize().dot(dir) > 0.9999, "pointing at the target");
        }
        // The knee stays on the pole's side of the reach line.
        let off = |p: Vec3| {
            let d = p - a;
            d - dir * d.dot(dir)
        };
        if dist < 0.89 {
            assert!(off(b).dot(off(pole)) > 0.0, "knee flipped at {dist}");
        }
    }
    // Weight 0 leaves the pose alone.
    let mut local = LocalPose::rest(&s);
    let mut m = model(&s, &local);
    ik::two_bone_ik(
        &s,
        &mut local,
        &mut m,
        &Affine3A::IDENTITY,
        [0, 1, 2],
        Vec3::new(0.0, 0.5, 0.5),
        pole,
        TwoBone { weight: 0.0, ..TwoBone::default() },
    );
    for (jn, rest) in s.rest().iter().enumerate() {
        assert_trs_close(local.get(jn), *rest, 1e-6, "weight 0");
    }
}

fn arm_chain() -> Skeleton {
    Skeleton::new(&[
        j("a", None, [0.0; 3]),
        j("b", Some(0), [0.25, 0.0, 0.0]),
        j("c", Some(1), [0.25, 0.0, 0.0]),
        j("d", Some(2), [0.25, 0.0, 0.0]),
        j("e", Some(3), [0.25, 0.0, 0.0]),
    ])
    .unwrap()
    .0
}

#[test]
fn fabrik_and_ccd_converge() {
    let target = Vec3::new(0.4, 0.5, 0.3);
    let mut points: Vec<Vec3> = (0..5).map(|i| Vec3::X * 0.25 * i as f32).collect();
    let left = ik::fabrik(&mut points, target, 1e-4, 50);
    assert!(left < 1e-3, "fabrik {left}");
    assert_eq!(points[0], Vec3::ZERO);
    for w in points.windows(2) {
        assert!((w[0].distance(w[1]) - 0.25).abs() < 1e-4);
    }
    // Out of reach: straight at the target.
    let mut far: Vec<Vec3> = (0..5).map(|i| Vec3::X * 0.25 * i as f32).collect();
    let left = ik::fabrik(&mut far, Vec3::new(0.0, 3.0, 0.0), 1e-4, 50);
    assert!((left - 2.0).abs() < 1e-4 && (far[4] - Vec3::Y).length() < 1e-4);

    let s = arm_chain();
    let chain = [0, 1, 2, 3, 4];
    let mut local = LocalPose::rest(&s);
    let mut m = model(&s, &local);
    let left = ik::fabrik_chain(&s, &mut local, &mut m, &Affine3A::IDENTITY, &chain, target, 1e-4, 50);
    assert!(left < 1e-3, "fabrik_chain {left}");
    assert!(m.mats.iter().zip(&model(&s, &local).mats).all(|(a, b)| a.abs_diff_eq(*b, 1e-5)), "model pose kept in sync");

    let mut local = LocalPose::rest(&s);
    let mut m = model(&s, &local);
    let left = ik::ccd_chain(&s, &mut local, &mut m, &Affine3A::IDENTITY, &chain, target, 1e-3, 100, 0.5);
    assert!(left < 5e-3, "ccd {left}");
}

#[test]
fn aim_chain_turns_toward_the_target_within_limits() {
    let s = Skeleton::new(&[j("spine", None, [0.0, 1.0, 0.0]), j("neck", Some(0), [0.0, 0.5, 0.0]), j("head", Some(1), [0.0, 0.15, 0.0])]).unwrap().0;
    let chain = [
        AimJoint { joint: 0, weight: 0.3, limit: 1.0 },
        AimJoint { joint: 1, weight: 0.5, limit: 1.0 },
        AimJoint { joint: 2, weight: 1.0, limit: 1.0 },
    ];
    let target = Vec3::new(1.0, 1.65, 1.0);
    let aim = Aim { forward: Vec3::Z, total_limit: std::f32::consts::PI, weight: 1.0 };
    let mut local = LocalPose::rest(&s);
    let mut m = model(&s, &local);
    let left = ik::aim_chain(&s, &mut local, &mut m, &Affine3A::IDENTITY, &chain, 2, target, aim);
    assert!(left < 0.02, "aim error {left}");
    // The spine took part of the turn.
    assert!(angle_between(local.get(0).rotation, Quat::IDENTITY) > 0.05);
    // A total limit caps the turn.
    let limited = Aim { total_limit: 0.3, ..aim };
    let mut local = LocalPose::rest(&s);
    let mut m = model(&s, &local);
    ik::aim_chain(&s, &mut local, &mut m, &Affine3A::IDENTITY, &chain[2..], 2, target, limited);
    let forward = m.rotation(2) * Vec3::Z;
    assert!(forward.angle_between(Vec3::Z) <= 0.3 + 1e-4);
    assert!(ik::clamp_angle(Quat::from_rotation_y(2.0), 0.5).angle_between(Quat::from_rotation_y(0.5)) < 1e-5);
}

#[test]
fn springs_are_frame_rate_independent_and_converge() {
    let damp = |dt: f32, steps: usize| (0..steps).fold(0.0f32, |x, _| spring::damper(x, 1.0, 0.2, dt));
    assert!((damp(1.0 / 60.0, 60) - damp(1.0 / 30.0, 30)).abs() < 1e-5);
    assert!((spring::damper(0.0f32, 1.0, 0.2, 0.2) - 0.5).abs() < 1e-3, "half the gap per half-life");
    let run = |dt: f32, steps: usize| {
        let (mut x, mut v) = (0.0f32, 0.0f32);
        for _ in 0..steps {
            spring::critical_spring(&mut x, &mut v, 1.0, 0.1, dt);
        }
        x
    };
    assert!((run(1.0 / 60.0, 60) - run(1.0 / 30.0, 30)).abs() < 1e-4);
    assert!((run(1.0 / 60.0, 60) - 1.0).abs() < 1e-3);
    let mut q = spring::QuatSpring::new(Quat::IDENTITY, 0.1);
    let goal = Quat::from_rotation_y(2.0);
    for _ in 0..120 {
        q.update(goal, 1.0 / 60.0);
    }
    assert!(angle_between(q.value, goal) < 1e-3);
    // A moving goal is followed with no lag beyond the step the goal makes
    // inside one update (it is held still for the update); a spring that
    // ignores the goal's velocity trails far behind.
    let dt = 1.0 / 60.0;
    let (mut x, mut v, mut y, mut w) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for i in 0..600 {
        let goal = 2.0 * i as f32 * dt;
        spring::critical_spring_moving(&mut x, &mut v, goal, 2.0, 0.1, dt);
        spring::critical_spring(&mut y, &mut w, goal, 0.1, dt);
    }
    let next_goal = 2.0 * 600.0 * dt;
    assert!((next_goal - x).abs() < 2.0 * dt && (v - 2.0).abs() < 0.02, "{x} {v}");
    assert!(next_goal - y > 10.0 * (next_goal - x).abs(), "{y}");
    let mut s = spring::Spring::new(Vec3::ZERO, Vec3::ZERO, 0.05);
    for _ in 0..60 {
        s.update(Vec3::ONE, 1.0 / 60.0);
    }
    assert!(s.value.distance(Vec3::ONE) < 1e-3);
}

// --- Retargeting --------------------------------------------------------------------

fn biped_rig(prefix: &str, a_pose: bool, ik: bool) -> RetargetRig {
    let c = |name: &str, joints: &[&str], ik: bool| RetargetChain {
        name: name.into(),
        joints: joints.iter().map(|j| format!("{prefix}{j}")).collect(),
        ik,
        solver: None,
    };
    RetargetRig {
        chains: vec![
            c("hips", &["hips"], false),
            c("spine", &["spine", "chest"], false),
            c("head", &["head"], false),
            c("leg_l", &["thigh_l", "shin_l", "foot_l"], ik),
            c("leg_r", &["thigh_r", "shin_r", "foot_r"], ik),
            c("arm_l", &["upperarm_l", "forearm_l", "hand_l"], false),
        ],
        retarget_pose: if a_pose { vec![(format!("{prefix}upperarm_l"), Quat::IDENTITY)] } else { Vec::new() },
        hips: Some(format!("{prefix}hips")),
    }
}

/// Joints standing in `s`'s retarget pose with positions times `scale`,
/// every joint frame turned by `frame(joint)` (other local axis
/// conventions), names prefixed, plus an unmapped `tail` on the hips.
fn reframed(s: &Skeleton, rig: &RetargetRig, prefix: &str, scale: f32, frame: impl Fn(usize) -> Quat) -> Vec<SourceJoint> {
    let mut pose = s.rest().to_vec();
    for (name, q) in &rig.retarget_pose {
        pose[idx(s, name)].rotation = *q;
    }
    let m = model_trs(s, &pose);
    let rot: Vec<Quat> = (0..s.len()).map(|jn| (affine_rotation(&m[jn]) * frame(jn)).normalize()).collect();
    let pos: Vec<Vec3> = m.iter().map(|a| Vec3::from(a.translation) * scale).collect();
    let mut joints: Vec<SourceJoint> = (0..s.len())
        .map(|jn| {
            let (pr, pp) = s.parent(jn).map_or((Quat::IDENTITY, Vec3::ZERO), |p| (rot[p], pos[p]));
            SourceJoint {
                name: format!("{prefix}{}", s.name(jn)),
                parent: s.parent(jn),
                rest: Trs::new(pr.inverse() * (pos[jn] - pp), (pr.inverse() * rot[jn]).normalize(), Vec3::ONE),
            }
        })
        .collect();
    let hips = idx(s, "hips");
    joints.push(SourceJoint {
        name: format!("{prefix}tail"),
        parent: Some(hips),
        rest: Trs::from_translation(rot[hips].inverse() * Vec3::new(0.0, 0.0, -0.3)),
    });
    joints
}

fn other_axes(jn: usize) -> Quat {
    let x = jn as f32;
    Quat::from_euler(EulerRot::XYZ, (x * 1.3).sin(), (x * 0.7).cos(), (x * 2.1).sin())
}

#[test]
fn retarget_round_trip_stays_within_a_degree_and_a_centimeter() {
    let s = biped();
    let rig_s = biped_rig("", true, false);
    let t = Skeleton::new(&reframed(&s, &rig_s, "T_", 1.0, other_axes)).unwrap().0;
    let rig_t = biped_rig("T_", false, false);
    let clip = cook(&s, &walk_source(), &ClipRules { looping: Looping::Whole, ..ClipRules::default() });
    let on_t = retarget_clip(&s, &rig_s, &t, &rig_t, &clip, &CookSettings::default()).unwrap();
    assert_eq!(on_t.skeleton, t.id());
    let back = retarget_clip(&t, &rig_t, &s, &rig_s, &on_t, &CookSettings::default()).unwrap();
    assert_eq!(back.skeleton, s.id());
    assert_eq!(back.loop_range, clip.loop_range);

    let mapped: Vec<usize> = rig_s.chains.iter().flat_map(|c| c.joints.iter().map(|n| idx(&s, n))).collect();
    let effectors = ["head", "foot_l", "foot_r", "hand_l"];
    let (mut a, mut b, mut c) = (LocalPose::new(s.len()), LocalPose::new(s.len()), LocalPose::new(t.len()));
    for i in 0..=10 {
        let time = i as f32 / 10.0;
        clip.sample(time, &mut a, None);
        back.sample(time, &mut b, None);
        on_t.sample(time, &mut c, None);
        let (ma, mb, mc) = (model(&s, &a), model(&s, &b), model(&t, &c));
        for &jn in &mapped {
            let err = angle_between(ma.rotation(jn), mb.rotation(jn));
            assert!(err < 1f32.to_radians(), "{} off by {}° at {time}", s.name(jn), err.to_degrees());
        }
        for e in effectors {
            let (ps, pt) = (ma.translation(idx(&s, e)), mb.translation(idx(&s, e)));
            assert!(ps.distance(pt) < 0.01, "{e} round trip off by {} m", ps.distance(pt));
            // Same proportions: T's effectors follow S's.
            let on = mc.translation(idx(&t, &format!("T_{e}")));
            assert!(ps.distance(on) < 0.01, "{e} on T off by {} m", ps.distance(on));
        }
        let (r0, r1) = (clip.root_at(time), back.root_at(time));
        assert!(r0.0.distance(r1.0) < 1e-3 && (r0.1 - r1.1).abs() < 1e-3);
    }
    // The unmapped tail keeps its rest transform.
    let tail = idx(&t, "T_tail");
    on_t.sample(0.3, &mut c, None);
    assert_trs_close(c.get(tail), t.rest()[tail], 1e-3, "tail");
}

#[test]
fn retarget_scales_translation_root_motion_and_renames_contacts() {
    let s = biped();
    let rig_s = biped_rig("", true, false);
    let t = Skeleton::new(&reframed(&s, &rig_s, "T_", 1.25, |_| Quat::IDENTITY)).unwrap().0;
    let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, false)).unwrap();
    assert!((r.scale() - 1.25).abs() < 1e-5);
    let mut moved = s.rest().to_vec();
    moved[idx(&s, "hips")].translation += Vec3::new(0.0, -0.1, 0.2);
    let frames = FrameClip {
        name: "x".into(),
        rate: 1.0,
        duration: 1.0,
        frame_count: 2,
        frames: [s.rest().to_vec(), moved].concat(),
        root_motion: Some(RootMotionTrack { frames: vec![[0.0; 4], [0.0, 0.0, 1.0, 0.5]], attitude: Vec::new() }),
        curves: vec![
            CurveTrack { name: "contact.foot_l".into(), values: vec![1.0, 0.0] },
            CurveTrack { name: "speed".into(), values: vec![0.0, 0.0] },
        ],
        events: vec![Event::point(0.5, "step")],
        markers: vec![SyncMarker { time: 0.0, name: "down.foot_l".into() }],
        loop_range: Some((0.0, 1.0)),
        meta: ClipMeta { average_velocity: Vec3::Z, cycle_distance: 1.0, ..ClipMeta::default() },
    };
    let out = r.frames(&frames).unwrap();
    assert_eq!(out.root_motion.as_ref().unwrap().frames[1], [0.0, 0.0, 1.25, 0.5]);
    assert_eq!(out.curves[0].name, "contact.T_foot_l");
    assert_eq!(out.curves[1].name, "speed");
    assert_eq!(out.markers[0].name, "down.T_foot_l");
    assert_eq!(out.events, frames.events);
    assert!(out.meta.average_velocity.distance(Vec3::Z * 1.25) < 1e-6 && (out.meta.cycle_distance - 1.25).abs() < 1e-6);
    let m = model_trs(&t, &out.frames[t.len()..]);
    let hips = Vec3::from(m[idx(&t, "T_hips")].translation);
    assert!(hips.distance(Vec3::new(0.0, 1.125, 0.25)) < 1e-5, "{hips}");

    // Errors.
    let additive = FrameClip { meta: ClipMeta { additive: true, ..ClipMeta::default() }, ..frames.clone() };
    assert_eq!(r.frames(&additive).unwrap_err(), RetargetError::Additive);
    let short = FrameClip { frame_count: 3, ..frames };
    assert_eq!(r.frames(&short).unwrap_err(), RetargetError::BadFrames);
    let on_t = cook(&t, &SourceClip { tracks: vec![rot("T_spine", Quat::from_rotation_x)], ..walk_source() }, &no_root_motion());
    assert_eq!(r.clip(&on_t, &CookSettings::default()).unwrap_err(), RetargetError::SkeletonMismatch);
    let unknown = RetargetRig {
        chains: vec![RetargetChain { name: "x".into(), joints: vec!["nope".into()], ik: false, solver: None }],
        ..RetargetRig::default()
    };
    assert!(matches!(Retargeter::new(&s, &unknown, &t, &biped_rig("T_", false, false)), Err(RetargetError::UnknownJoint { .. })));
    let disjoint = RetargetRig {
        chains: vec![RetargetChain { name: "x".into(), joints: vec!["T_hips".into()], ik: false, solver: None }],
        ..RetargetRig::default()
    };
    assert_eq!(Retargeter::new(&s, &rig_s, &t, &disjoint).unwrap_err(), RetargetError::NoSharedChains);
    let short_ik = RetargetRig {
        chains: vec![RetargetChain { name: "hips".into(), joints: vec!["T_hips".into()], ik: true, solver: None }],
        ..RetargetRig::default()
    };
    assert_eq!(Retargeter::new(&s, &rig_s, &t, &short_ik).unwrap_err(), RetargetError::IkChain("hips".into()));
}

/// A mocap-like source for root motion from the target pelvis: the hips
/// are the take's root, resting off the origin and lying along +X (the
/// retarget pose stands them up), and each frame puts them at `at(f)`
/// facing `heading(f)` in the take's space.
fn take_source(frames: usize, at: impl Fn(usize) -> Vec3, heading: impl Fn(usize) -> f32) -> (Skeleton, RetargetRig, FrameClip) {
    let mut joints = biped_joints();
    joints[1].rest = Trs::new(Vec3::new(1.52, 1.0, -0.72), Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), Vec3::ONE);
    let s = Skeleton::new(&joints).unwrap().0;
    let mut rig = biped_rig("", true, false);
    rig.retarget_pose.push(("hips".into(), Quat::IDENTITY));
    let hips = idx(&s, "hips");
    let mut out = Vec::new();
    for f in 0..frames {
        let mut pose = s.rest().to_vec();
        pose[hips].translation = at(f);
        pose[hips].rotation = Quat::from_rotation_y(heading(f));
        out.extend(pose);
    }
    let clip = FrameClip {
        name: "take".into(),
        rate: 4.0,
        duration: (frames - 1) as f32 / 4.0,
        frame_count: frames,
        frames: out,
        root_motion: None,
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
        loop_range: None,
        meta: ClipMeta::default(),
    };
    (s, rig, clip)
}

/// Target hips (model) and their heading against the retarget pose.
fn hips_frame(r: &Retargeter, t: &Skeleton, pose: &[Trs]) -> (Vec3, f32) {
    let m = model_trs(t, pose);
    let rest = model_trs(t, r.target_retarget_pose());
    let h = idx(t, "T_hips");
    (Vec3::from(m[h].translation), twist_angle(affine_rotation(&m[h]) * affine_rotation(&rest[h]).inverse(), Vec3::Y))
}

#[test]
fn root_from_target_pelvis_starts_at_the_origin_facing_forward() {
    use std::f32::consts::FRAC_PI_2;
    // Facing +X at frame 0 (90° off), walking forward 0.3 m and up 5 cm a frame.
    let (s, rig_s, take) = take_source(5, |f| Vec3::new(3.0 + 0.3 * f as f32, 1.0 + 0.05 * f as f32, 2.0), |_| FRAC_PI_2);
    let t = Skeleton::new(&reframed(&biped(), &biped_rig("", true, false), "T_", 1.25, other_axes)).unwrap().0;
    let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, false)).unwrap();
    assert!((r.scale() - 1.25).abs() < 1e-5);
    let rules = RootMotionRules { mode: RootMotionMode::Progress, vertical: true, ..RootMotionRules::default() };
    let out = r.frames_with_root(&take, &rules).unwrap();
    assert_eq!(out.meta.root_motion_mode, RootMotionMode::Progress);
    let n = t.len();
    let (p0, yaw0) = hips_frame(&r, &t, &out.frames[..n]);
    assert!(Vec3::new(p0.x, 0.0, p0.z).length() < 1e-3, "frame-0 hips off the origin: {p0}");
    assert!(yaw0.abs() < 0.5f32.to_radians(), "frame-0 heading {}°", yaw0.to_degrees());
    assert!((p0.y - 1.25).abs() < 1e-3, "hips height {}", p0.y);
    let track = &out.root_motion.as_ref().unwrap().frames;
    assert!(track[0].iter().all(|v| v.abs() < 1e-5));
    // Forward is +Z now, scaled by 1.25: 1.2 m forward, 0.2 m up.
    let last = track[4];
    assert!(Vec3::new(last[0], last[1], last[2]).distance(Vec3::new(0.0, 0.25, 1.5)) < 1e-3, "{last:?}");
    assert!(last[3].abs() < 1e-4);
    for f in 0..5 {
        let (p, _) = hips_frame(&r, &t, &out.frames[f * n..(f + 1) * n]);
        assert!(p.distance(p0) < 1e-3, "hips leave the root at frame {f}: {p}");
    }

    // Cooked: the same, through the source and target cooks.
    let source = cook_frames(&s, &take, &CookSettings::default()).unwrap();
    let on_t = r.clip_with_root(&source, &rules, &CookSettings::default()).unwrap();
    let (c, yaw) = on_t.root_at(on_t.duration);
    assert!(c.distance(Vec3::new(0.0, 0.25, 1.5)) < 1e-3 && yaw.abs() < 1e-3, "{c} {yaw}");
    assert!(on_t.meta.average_velocity.distance(Vec3::new(0.0, 0.25, 1.5)) < 1e-3, "{}", on_t.meta.average_velocity);
    let mut pose = LocalPose::new(t.len());
    on_t.sample(0.0, &mut pose, None);
    let (p, yaw) = hips_frame(&r, &t, &pose.to_vec());
    assert!(Vec3::new(p.x, 0.0, p.z).length() < 1e-3 && yaw.abs() < 0.5f32.to_radians(), "{p} {yaw}");

    // Errors.
    let with_root = FrameClip { root_motion: Some(RootMotionTrack { frames: vec![[0.0; 4]; 5], attitude: Vec::new() }), ..take.clone() };
    assert_eq!(r.frames_with_root(&with_root, &rules).unwrap_err(), RetargetError::SourceRootMotion);
    let no_hips = RetargetRig { hips: None, ..biped_rig("T_", false, false) };
    let r = Retargeter::new(&s, &rig_s, &t, &no_hips).unwrap();
    assert_eq!(r.frames_with_root(&take, &rules).unwrap_err(), RetargetError::NoHips);
}

#[test]
fn root_from_target_pelvis_keeps_a_turning_body_over_its_root() {
    use std::f32::consts::FRAC_PI_2;
    let at = |f: usize| Vec3::new(3.0 + (f as f32 * 0.5).sin(), 1.0, 2.0 + 0.4 * f as f32);
    let heading = |f: usize| FRAC_PI_2 + f as f32 * 30f32.to_radians();
    let (s, rig_s, take) = take_source(7, at, heading);
    let t = Skeleton::new(&reframed(&biped(), &biped_rig("", true, false), "T_", 1.25, other_axes)).unwrap().0;
    let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, false)).unwrap();
    let out = r.frames_with_root(&take, &RootMotionRules::default()).unwrap();
    let track = &out.root_motion.as_ref().unwrap().frames;
    let n = t.len();
    let (p0, _) = hips_frame(&r, &t, &out.frames[..n]);
    // The take re-based: frame 0's heading (90°) turned back to +Z.
    let rebase = Quat::from_rotation_y(-FRAC_PI_2);
    for (f, &[x, y, z, yaw]) in track.iter().enumerate() {
        let (p, _) = hips_frame(&r, &t, &out.frames[f * n..(f + 1) * n]);
        assert!(Vec3::new(p.x - p0.x, 0.0, p.z - p0.z).length() < 1e-3, "body leaves its root at frame {f}: {p}");
        assert!(wrap_yaw(yaw - (heading(f) - heading(0))).abs() < 1e-3, "yaw at {f}: {yaw}");
        let world = Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw), Vec3::new(x, y, z)).transform_point3(p);
        let want = rebase * (at(f) - at(0)) * 1.25;
        let got = world - p0;
        assert!(Vec3::new(got.x - want.x, 0.0, got.z - want.z).length() < 1e-3, "world hips at {f}: {got} vs {want}");
    }
}

fn wrap_yaw(a: f32) -> f32 {
    (a + std::f32::consts::PI).rem_euclid(TAU) - std::f32::consts::PI
}

#[test]
fn retarget_matches_chains_of_different_lengths_by_arc_length() {
    let s = biped();
    let rig_s = biped_rig("", true, false);
    let t = Skeleton::new(&[
        j("root", None, [0.0; 3]),
        j("hips", Some(0), [0.0, 1.0, 0.0]),
        j("spine", Some(1), [0.0, 0.1, 0.0]),
        j("chest", Some(2), [0.0, 0.1, 0.0]),
        j("upperchest", Some(3), [0.0, 0.15, 0.0]),
        j("head", Some(4), [0.0, 0.3, 0.0]),
    ])
    .unwrap()
    .0;
    let chain = |name: &str, joints: &[&str]| RetargetChain {
        name: name.into(),
        joints: joints.iter().map(|&n| n.to_owned()).collect(),
        ik: false,
        solver: None,
    };
    let rig_t = RetargetRig {
        chains: vec![chain("hips", &["hips"]), chain("spine", &["spine", "chest", "upperchest"]), chain("head", &["head"])],
        retarget_pose: Vec::new(),
        hips: Some("hips".into()),
    };
    let r = Retargeter::new(&s, &rig_s, &t, &rig_t).unwrap();
    assert_eq!(r.pairs().len(), 5);
    let mut pose = s.rest().to_vec();
    pose[idx(&s, "spine")].rotation = Quat::from_rotation_x(0.3);
    pose[idx(&s, "chest")].rotation = Quat::from_rotation_x(0.3);
    let mut out = vec![Trs::IDENTITY; t.len()];
    r.pose(&pose, &[], &mut out);
    let m = model_trs(&t, &out);
    let rot = |name: &str| affine_rotation(&m[idx(&t, name)]);
    assert!(angle_between(rot("spine"), Quat::from_rotation_x(0.3)) < 1e-4);
    // The chest sits 40% along the target spine: between the source deltas.
    assert!(angle_between(rot("chest"), Quat::from_rotation_x(0.3 + 0.3 * 0.4)) < 2e-3);
    assert!(angle_between(rot("upperchest"), Quat::from_rotation_x(0.6)) < 1e-4);
    assert!(angle_between(rot("head"), Quat::from_rotation_x(0.6)) < 1e-4);
}

#[test]
fn retarget_ik_pins_feet_on_other_proportions() {
    let s = biped();
    let rig_s = biped_rig("", true, false);
    let mut joints = reframed(&s, &rig_s, "T_", 1.0, |_| Quat::IDENTITY);
    // Longer thighs, shorter shins: same leg length, other knee height.
    for (name, y) in [("T_shin_l", -0.5), ("T_foot_l", -0.4)] {
        joints.iter_mut().find(|j| j.name == name).unwrap().rest.translation = Vec3::new(0.0, y, 0.0);
    }
    let t = Skeleton::new(&joints).unwrap().0;
    let mut pose = s.rest().to_vec();
    pose[idx(&s, "thigh_l")].rotation = Quat::from_rotation_x(-0.8);
    pose[idx(&s, "shin_l")].rotation = Quat::from_rotation_x(1.2);
    let ms = model_trs(&s, &pose);
    let goal = Vec3::from(ms[idx(&s, "foot_l")].translation);
    let foot = idx(&t, "T_foot_l");
    let solve = |ik: bool| {
        let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, ik)).unwrap();
        let mut out = vec![Trs::IDENTITY; t.len()];
        r.pose(&pose, &[1.0, 1.0], &mut out);
        model_trs(&t, &out)
    };
    let fk = solve(false);
    assert!(Vec3::from(fk[foot].translation).distance(goal) > 0.02, "FK alone misses on other proportions");
    let pinned = solve(true);
    let got = Vec3::from(pinned[foot].translation);
    assert!(got.distance(goal) < 1e-3, "IK pins the foot: off by {}", got.distance(goal));
    // The knee bends the same way as the source's.
    let (ks, kt) = (ms[idx(&s, "shin_l")].translation, pinned[idx(&t, "T_shin_l")].translation);
    assert!(ks.z * kt.z > 0.0);
}

#[test]
fn subjects_parse() {
    let s = steps::anim_subject("chars/a.glb", "anims/x.glb", "odd|name");
    assert_eq!(steps::parse_anim_subject(&s), Some(("chars/a.glb", "anims/x.glb", "odd|name")));
    let r = steps::retarget_subject("chars/b.glb", &s);
    assert_eq!(steps::parse_retarget_subject(&r), Some(("chars/b.glb", s.as_str())));
    assert_eq!(steps::parse_anim_subject("a|b"), None);
    assert_eq!(steps::parse_retarget_subject("a|b|c"), None);
}

// --- glTF and bake steps (need the `gltf` feature) ------------------------------

#[cfg(feature = "gltf")]
mod gltf {
    use std::f32::consts::FRAC_PI_2;
    use std::sync::{Arc, RwLock};

    use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};

    use super::*;
    use crate::steps::{AnimBake, AnimImporters, AnimRules, ClipSpec};

    /// A GLB under construction: float accessors in one buffer.
    #[derive(Default)]
    struct Glb {
        bin: Vec<u8>,
        views: Vec<String>,
        accessors: Vec<String>,
    }

    impl Glb {
        fn floats(&mut self, data: &[f32], ty: &str, bounds: bool) -> usize {
            let comps = match ty {
                "SCALAR" => 1,
                "VEC3" => 3,
                _ => 4,
            };
            let offset = self.bin.len();
            for v in data {
                self.bin.extend(v.to_le_bytes());
            }
            self.views.push(format!(r#"{{"buffer":0,"byteOffset":{offset},"byteLength":{}}}"#, data.len() * 4));
            let bounds = if bounds {
                let col = |c: usize, f: fn(f32, f32) -> f32, init: f32| data.iter().skip(c).step_by(comps).copied().fold(init, f).to_string();
                let min: Vec<String> = (0..comps).map(|c| col(c, f32::min, f32::INFINITY)).collect();
                let max: Vec<String> = (0..comps).map(|c| col(c, f32::max, f32::NEG_INFINITY)).collect();
                format!(r#","min":[{}],"max":[{}]"#, min.join(","), max.join(","))
            } else {
                String::new()
            };
            self.accessors.push(format!(
                r#"{{"bufferView":{},"componentType":5126,"count":{},"type":"{ty}"{bounds}}}"#,
                self.views.len() - 1,
                data.len() / comps
            ));
            self.accessors.len() - 1
        }

        fn finish(self, body: &str) -> Vec<u8> {
            let json = format!(
                r#"{{"asset":{{"version":"2.0"}},{body},"buffers":[{{"byteLength":{}}}],"bufferViews":[{}],"accessors":[{}]}}"#,
                self.bin.len(),
                self.views.join(","),
                self.accessors.join(",")
            );
            let mut json = json.into_bytes();
            while !json.len().is_multiple_of(4) {
                json.push(b' ');
            }
            let total = 12 + 8 + json.len() + 8 + self.bin.len();
            let mut out = Vec::new();
            out.extend(b"glTF");
            out.extend(2u32.to_le_bytes());
            out.extend(u32::try_from(total).unwrap().to_le_bytes());
            out.extend(u32::try_from(json.len()).unwrap().to_le_bytes());
            out.extend(b"JSON");
            out.extend(json);
            out.extend(u32::try_from(self.bin.len()).unwrap().to_le_bytes());
            out.extend(b"BIN\0");
            out.extend(self.bin);
            out
        }
    }

    /// A skinned character under a turned `Armature` node, with a mesh node
    /// that is not a joint, a "walk" (linear, step and cubic channels) and an
    /// unnamed idle.
    fn character_glb(p: &str) -> Vec<u8> {
        let q = |q: Quat| q.to_array();
        let zero = [0.0f32; 4];
        let mut g = Glb::default();
        let t3 = g.floats(&[0.0, 0.5, 1.0], "SCALAR", true);
        let hips_t = g.floats(&[0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0, 1.0], "VEC3", false);
        let spine_r = g.floats(&[q(Quat::IDENTITY), q(Quat::from_rotation_x(0.2)), q(Quat::IDENTITY)].concat(), "VEC4", false);
        let t2 = g.floats(&[0.0, 0.5], "SCALAR", true);
        let head_r = g.floats(&[q(Quat::IDENTITY), q(Quat::from_rotation_y(0.3))].concat(), "VEC4", false);
        let t1 = g.floats(&[0.0, 1.0], "SCALAR", true);
        let thigh_r = g.floats(&[zero, q(Quat::IDENTITY), zero, zero, q(Quat::from_rotation_x(0.5)), zero].concat(), "VEC4", false);
        let t_idle = g.floats(&[0.0, 2.0], "SCALAR", true);
        let hips_s = g.floats(&[1.0, 1.0, 1.0, 1.1, 1.1, 1.1], "VEC3", false);
        let r = std::f32::consts::FRAC_1_SQRT_2;
        let body = format!(
            r#""scene":0,"scenes":[{{"nodes":[0,7]}}],
    "nodes":[
    {{"name":"{p}Armature","rotation":[0,{r},0,{r}],"children":[1]}},
    {{"name":"{p}hips","translation":[0,1,0],"children":[2,4]}},
    {{"name":"{p}spine","translation":[0,0.3,0],"children":[3]}},
    {{"name":"{p}head","translation":[0,0.4,0]}},
    {{"name":"{p}thigh_l","translation":[0.1,-0.05,0],"children":[5]}},
    {{"name":"{p}shin_l","translation":[0,-0.45,0],"children":[6]}},
    {{"name":"{p}foot_l","translation":[0,-0.45,0]}},
    {{"name":"{p}body"}}],
    "skins":[{{"joints":[1,2,3,4,5,6]}}],
    "animations":[
    {{"name":"walk","channels":[
    {{"sampler":0,"target":{{"node":1,"path":"translation"}}}},
    {{"sampler":1,"target":{{"node":2,"path":"rotation"}}}},
    {{"sampler":2,"target":{{"node":3,"path":"rotation"}}}},
    {{"sampler":3,"target":{{"node":4,"path":"rotation"}}}}],
    "samplers":[
    {{"input":{t3},"output":{hips_t}}},
    {{"input":{t3},"output":{spine_r},"interpolation":"LINEAR"}},
    {{"input":{t2},"output":{head_r},"interpolation":"STEP"}},
    {{"input":{t1},"output":{thigh_r},"interpolation":"CUBICSPLINE"}}]}},
    {{"channels":[{{"sampler":0,"target":{{"node":1,"path":"scale"}}}}],"samplers":[{{"input":{t_idle},"output":{hips_s}}}]}}]"#
        );
        g.finish(&body)
    }

    #[test]
    fn gltf_imports_skeleton_and_clips() {
        let bytes = character_glb("");
        let src = GltfAnimImporter.import("chars/hero.glb", &bytes).unwrap();
        let joints = src.skeleton.as_ref().expect("skinned file has a skeleton");
        let names: Vec<&str> = joints.iter().map(|j| j.name.as_str()).collect();
        assert_eq!(names, ["Armature", "hips", "spine", "head", "thigh_l", "shin_l", "foot_l"], "joints and their ancestors, no mesh node");
        let s = Skeleton::new(joints).unwrap().0;
        assert_eq!(s.parent(idx(&s, "hips")), Some(idx(&s, "Armature")));
        assert!(angle_between(s.rest()[idx(&s, "Armature")].rotation, Quat::from_rotation_y(FRAC_PI_2)) < 1e-6);
        assert_eq!(s.rest()[idx(&s, "shin_l")].translation, Vec3::new(0.0, -0.45, 0.0));

        let clip_names: Vec<&str> = src.clips.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(clip_names, ["walk", "anim1"]);
        assert_eq!(list_gltf_clips("chars/hero.glb", &bytes).unwrap(), ["walk", "anim1"]);
        let walk = &src.clips[0];
        assert_eq!(walk.duration, 1.0);
        assert_eq!(walk.tracks.len(), 4);
        let track = |name: &str| walk.tracks.iter().find(|t| t.joint == name).unwrap();
        let head = track("head").rotation.as_ref().unwrap();
        assert_eq!(head.interpolation, Interpolation::Step);
        assert_eq!(head.sample(0.49), Quat::IDENTITY);
        assert!(angle_between(head.sample(0.5), Quat::from_rotation_y(0.3)) < 1e-6);
        let thigh = track("thigh_l").rotation.as_ref().unwrap();
        assert_eq!(thigh.interpolation, Interpolation::CubicSpline);
        assert_eq!(thigh.values.len(), 6);
        assert!(angle_between(thigh.sample(0.5), Quat::from_rotation_x(0.25)) < 1e-5);
        let hips = track("hips").translation.as_ref().unwrap();
        assert!(hips.sample(0.25).distance(Vec3::new(0.0, 1.0, 0.25)) < 1e-6);
        let idle = &src.clips[1];
        assert_eq!(idle.duration, 2.0);
        assert!(idle.tracks[0].scale.is_some());

        // The import cooks; root motion follows the turned armature (+Z → +X).
        let rules = SkeletonRules { root_motion_joint: Some("hips".into()), ..SkeletonRules::default() };
        let s = Skeleton::with_rules(joints, &rules).unwrap().0;
        let clip = cook(&s, walk, &ClipRules::default());
        assert!(clip.meta.average_velocity.distance(Vec3::X) < 1e-3, "{}", clip.meta.average_velocity);

        let external = br#"{"asset":{"version":"2.0"},"buffers":[{"byteLength":4,"uri":"data.bin"}]}"#;
        assert!(GltfAnimImporter.import("a.gltf", external).unwrap_err().message.contains("external buffer"));
        assert!(GltfAnimImporter.import("a.glb", b"not a glb").is_err());
        let mut rock = Glb::default();
        rock.floats(&[0.0], "SCALAR", false);
        let static_glb = rock.finish(r#""scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"name":"rock"}]"#);
        assert_eq!(GltfAnimImporter.import("rock.glb", &static_glb).unwrap(), AnimSource::default());
    }

    // --- Bake steps ---------------------------------------------------------------------

    #[derive(Default)]
    struct Files(RwLock<HashMap<String, Vec<u8>>>);

    impl Inputs for Files {
        fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
            Ok(self.0.read().unwrap().get(path).map(|b| Digest::of(b)))
        }
        fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
            self.0.read().unwrap().get(path).cloned().ok_or_else(|| BakeError::MissingInput(Input::File(path.into())))
        }
        fn record_fingerprint(&self, _kind: &str, _id: &str) -> Result<Option<Digest>, BakeError> {
            Ok(None)
        }
    }

    /// Rig data for `character_glb`: `chars/a.glb` has joints `a_*`, `b.glb` `b_*`.
    struct TestRules;

    fn prefix(path: &str) -> &'static str {
        if path.ends_with("b.glb") { "b_" } else { "a_" }
    }

    impl AnimRules for TestRules {
        fn skeleton(&self, path: &str) -> SkeletonRules {
            SkeletonRules { root_motion_joint: Some(format!("{}hips", prefix(path))), ..SkeletonRules::default() }
        }
        fn clip(&self, _file: &str, clip: &str, _skeleton: &str) -> ClipSpec {
            match clip {
                "walk_loop" => ClipSpec { source: "walk".into(), rules: ClipRules { looping: Looping::Whole, ..ClipRules::default() } },
                "walk_pelvis" => ClipSpec::named("walk"),
                _ => ClipSpec::named(clip),
            }
        }
        fn retarget_root(&self, _file: &str, clip: &str, _skeleton: &str) -> Option<RootMotionRules> {
            (clip == "walk_pelvis").then(RootMotionRules::default)
        }
        fn rig(&self, path: &str) -> Option<RetargetRig> {
            let p = prefix(path);
            let chain = |name: &str, joints: &[&str]| RetargetChain {
                name: name.into(),
                joints: joints.iter().map(|j| format!("{p}{j}")).collect(),
                ik: false,
                solver: None,
            };
            Some(RetargetRig {
                chains: vec![
                    chain("hips", &["hips"]),
                    chain("spine", &["spine"]),
                    chain("head", &["head"]),
                    chain("leg_l", &["thigh_l", "shin_l", "foot_l"]),
                ],
                retarget_pose: Vec::new(),
                hips: Some(format!("{p}hips")),
            })
        }
    }

    #[test]
    fn bake_steps_cook_retarget_and_cache() {
        let files = Arc::new(Files::default());
        files.0.write().unwrap().insert("chars/a.glb".into(), character_glb("a_"));
        files.0.write().unwrap().insert("chars/b.glb".into(), character_glb("b_"));
        let dir = tempfile::tempdir().unwrap();
        let bake = AnimBake::new(AnimImporters::framework(), Arc::new(TestRules), CookSettings::default());
        let session = || {
            let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files.clone());
            bake.register(&mut baker);
            baker
        };
        let (a, b) = ("chars/a.glb", "chars/b.glb");
        let walk = steps::anim_subject(a, a, "walk");
        let keys = [
            ProductKey::new(steps::SKELETON_COOK, a),
            ProductKey::new(steps::ANIM_COOK, walk.clone()),
            ProductKey::new(steps::ANIM_COOK, steps::anim_subject(a, a, "walk_loop")),
            ProductKey::new(steps::ANIM_COOK, steps::anim_subject(a, a, "anim1")),
            ProductKey::new(steps::ANIM_RETARGET, steps::retarget_subject(b, &walk)),
        ];
        let baker = session();
        let report = baker.bake(&keys);
        assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
        let get = |baker: &Baker, k: &ProductKey| baker.store().get(&baker.product(k).unwrap()).unwrap();

        let skel = Skeleton::from_bytes(&get(&baker, &keys[0])).unwrap();
        assert_eq!(skel.len(), 7);
        assert_eq!(skel.root_motion_joint(), skel.find("a_hips"));
        let walk_clip = Clip::from_bytes(&get(&baker, &keys[1])).unwrap();
        assert_eq!((walk_clip.name.as_str(), walk_clip.skeleton), ("walk", skel.id()));
        assert!(walk_clip.loop_range.is_none());
        assert!((walk_clip.meta.average_velocity.length() - 1.0).abs() < 1e-3);
        let looped = Clip::from_bytes(&get(&baker, &keys[2])).unwrap();
        assert_eq!(looped.name, "walk_loop");
        assert_eq!(looped.loop_range, Some((0.0, 1.0)));
        assert_eq!(Clip::from_bytes(&get(&baker, &keys[3])).unwrap().duration, 2.0);
        let b_skel = Skeleton::from_bytes(&get(&baker, &ProductKey::new(steps::SKELETON_COOK, b))).unwrap();
        let moved = Clip::from_bytes(&get(&baker, &keys[4])).unwrap();
        assert_eq!(moved.skeleton, b_skel.id());
        // Same-shaped rigs: the retargeted clip plays the same pose.
        let (mut pa, mut pb) = (LocalPose::new(skel.len()), LocalPose::new(b_skel.len()));
        for time in [0.25, 0.5, 0.8] {
            walk_clip.sample(time, &mut pa, None);
            moved.sample(time, &mut pb, None);
            for n in ["spine", "head", "thigh_l"] {
                let (x, y) = (pa.get(idx(&skel, &format!("a_{n}"))), pb.get(idx(&b_skel, &format!("b_{n}"))));
                assert!(angle_between(x.rotation, y.rotation) < 1e-3, "{n} at {time}");
            }
        }
        let stats = baker.step_stats();
        assert_eq!(stats[steps::SKELETON_COOK].built, 2);
        assert_eq!(stats[steps::ANIM_COOK].built, 3);
        assert_eq!(stats[steps::ANIM_RETARGET].built, 1);

        // A new session over the same store builds nothing, and the products reproduce.
        let again = session();
        assert_eq!(again.bake(&keys).failures().count(), 0);
        assert!(again.step_stats().values().all(|s| s.built == 0), "{:?}", again.step_stats());
        for (key, same) in again.verify(&keys) {
            assert!(same.unwrap(), "{key} is not reproducible");
        }
        // Errors are per product.
        assert!(again.product(&ProductKey::new(steps::ANIM_COOK, steps::anim_subject(a, a, "nope"))).is_err());
        assert!(again.product(&ProductKey::new(steps::ANIM_COOK, "no separators")).is_err());
        assert!(again.product(&ProductKey::new(steps::SKELETON_COOK, "chars/missing.glb")).is_err());
        assert!(again.product(&ProductKey::new(steps::SKELETON_COOK, "chars/a.fbx")).is_err());
    }

    /// `AnimRules::retarget_root`: the source cooks with its motion in the
    /// pose and the retarget extracts it on the target hips, matching the
    /// root track of the usual path (root motion on the hips, same shapes).
    #[test]
    fn bake_steps_generate_root_motion_from_the_target_pelvis() {
        let files = Arc::new(Files::default());
        files.0.write().unwrap().insert("chars/a.glb".into(), character_glb("a_"));
        files.0.write().unwrap().insert("chars/b.glb".into(), character_glb("b_"));
        let dir = tempfile::tempdir().unwrap();
        let bake = AnimBake::new(AnimImporters::framework(), Arc::new(TestRules), CookSettings::default());
        let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files);
        bake.register(&mut baker);
        let (a, b) = ("chars/a.glb", "chars/b.glb");
        let (walk, pelvis) = (steps::anim_subject(a, a, "walk"), steps::anim_subject(a, a, "walk_pelvis"));
        let keys = [
            ProductKey::new(steps::ANIM_COOK, pelvis.clone()),
            ProductKey::new(steps::ANIM_RETARGET, steps::retarget_subject(b, &pelvis)),
            ProductKey::new(steps::ANIM_RETARGET, steps::retarget_subject(b, &walk)),
        ];
        let report = baker.bake(&keys);
        assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
        let get = |k: &ProductKey| Clip::from_bytes(&baker.store().get(&baker.product(k).unwrap()).unwrap()).unwrap();
        assert!(get(&keys[0]).root_motion.is_none(), "the source keeps its motion in the pose");
        let (generated, usual) = (get(&keys[1]), get(&keys[2]));
        assert!(generated.root_motion.is_some());
        for time in [0.0, 0.5, 1.0] {
            let (g, u) = (generated.root_at(time), usual.root_at(time));
            assert!(g.0.distance(u.0) < 1e-3 && (g.1 - u.1).abs() < 1e-3, "root at {time}: {g:?} vs {u:?}");
        }
        assert!(generated.meta.average_velocity.distance(usual.meta.average_velocity) < 1e-3);
    }
}

/// A keyframed sidestep that skates (walkers3): the stance foot drifts at
/// 0.5 m/s while the body travels 1.4 m/s, with a 2 cm wobble. Measured
/// against the ground alone it never plants (the old cook: 0 of the
/// cycle); measured against the slowest low foot each foot plants for its
/// half of the cycle, once per cycle.
#[test]
fn skating_keyframed_steps_still_plant_each_foot() {
    let joints = [j("root", None, [0.0; 3]), j("foot_l", Some(0), [0.1, 0.0, 0.0]), j("foot_r", Some(0), [-0.1, 0.0, 0.0])];
    let s = Skeleton::with_rules(&joints, &SkeletonRules::default()).unwrap().0;
    let (travel, drift) = (1.4f32, 0.5f32);
    // World z and height of a foot at cycle phase `u` (0..1): stance then swing.
    let foot = move |u: f32| {
        let u = u.rem_euclid(1.0);
        if u < 0.5 {
            (drift * u, 0.02 * (std::f32::consts::PI * u / 0.5).sin())
        } else {
            let k = (u - 0.5) / 0.5;
            (drift * 0.5 + (travel - drift * 0.5) * k, 0.12 * (std::f32::consts::PI * k).sin())
        }
    };
    let samples = 61;
    let times: Vec<f32> = (0..samples).map(|i| i as f32 / (samples - 1) as f32).collect();
    let track = |phase: f32, x: f32| {
        let values = times
            .iter()
            .map(|&t| {
                let (z, y) = foot(t + phase);
                // The cycle start of the shifted foot is `phase` cycles behind.
                let z = z + travel * ((t + phase).floor()) - travel * t;
                Vec3::new(x, y, z)
            })
            .collect();
        Curve::linear(times.clone(), values)
    };
    let src = SourceClip {
        name: "sidestep".into(),
        duration: 1.0,
        tracks: vec![
            SourceTrack { joint: "root".into(), translation: Some(Curve::linear(vec![0.0, 1.0], vec![Vec3::ZERO, Vec3::Z * travel])), ..SourceTrack::default() },
            SourceTrack { joint: "foot_l".into(), translation: Some(track(0.0, 0.1)), ..SourceTrack::default() },
            SourceTrack { joint: "foot_r".into(), translation: Some(track(0.5, -0.1)), ..SourceTrack::default() },
        ],
        ..SourceClip::default()
    };
    // The feet hang 0.1 m off the root: thresholds for a 0.9 m leg in those units.
    let contacts = ContactRules { height: 0.06 * 9.0, speed: 0.35 * 9.0, ..ContactRules::default() };
    let clip = cook(&s, &src, &ClipRules { looping: Looping::Whole, contacts, ..ClipRules::default() });
    for name in ["contact.foot_l", "contact.foot_r"] {
        let v = &clip.curves[clip.find_curve(name).unwrap_or_else(|| panic!("{name}"))].values;
        let planted = v.iter().filter(|&&x| x > 0.5).count() as f32 / v.len() as f32;
        assert!((0.35..0.6).contains(&planted), "{name}: planted {planted:.2} of the cycle: {v:?}");
    }
    for side in ["down.foot_l", "down.foot_r"] {
        assert_eq!(clip.markers.iter().filter(|m| m.name == side).count(), 1, "{side}: {:?}", clip.markers);
    }
}

#[test]
fn stride_speed_follows_the_feet_where_measured() {
    let meta = |root: f32, feet: f32| ClipMeta { average_velocity: Vec3::Z * root, stance_velocity: Vec3::Z * feet, ..ClipMeta::default() };
    // Unmeasured: the root's; measured, the feet's, however close (a lock drifts by any gap).
    assert_eq!(meta(1.7, 0.0).stride_velocity(), Vec3::Z * 1.7);
    assert_eq!(meta(1.41, 1.45).stride_velocity(), Vec3::Z * 1.45);
    // A forward walk whose keyframed feet skate 10% (AN-16: the lock drifted 30 cm): the feet's.
    assert_eq!(meta(2.2, 1.97).stride_velocity(), Vec3::Z * 1.97);
    // A sidestep whose root outruns its planted feet: the feet's.
    assert_eq!(meta(1.72, 0.92).stride_velocity(), Vec3::Z * 0.92);
}
