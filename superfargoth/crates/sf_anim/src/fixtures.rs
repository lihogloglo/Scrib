//! Synthetic test skeletons and gait clips (feature `fixtures`, tests
//! only): a biped, a guar-like quadruped with digitigrade hind legs, an
//! insect-like hexapod and a cliff-racer-like flyer. Y up, facing +Z, +X
//! left, meters at `scale` 1.

use std::f32::consts::PI;

use glam::{Quat, Vec3};

use crate::{Curve, Event, Skeleton, SkeletonRules, SourceClip, SourceCurve, SourceJoint, SourceTrack, Trs};

fn joint(name: &str, parent: Option<usize>, t: [f32; 3], scale: f32) -> SourceJoint {
    SourceJoint { name: name.to_owned(), parent, rest: Trs::from_translation(Vec3::from(t) * scale) }
}

type J = (String, Option<String>, [f32; 3]);

fn e(name: &str, parent: Option<&str>, t: [f32; 3]) -> J {
    (name.to_owned(), parent.map(str::to_owned), t)
}

fn build(joints: &[J], scale: f32) -> Skeleton {
    let index = |n: &str| joints.iter().position(|j| j.0 == n).expect("parent listed first");
    let v: Vec<SourceJoint> = joints.iter().map(|(n, p, t)| joint(n, p.as_deref().map(index), *t, scale)).collect();
    let rules = SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() };
    Skeleton::with_rules(&v, &rules).expect("valid fixture").0
}

/// A small biped (hips at 1 m).
pub fn biped() -> Skeleton {
    build(
        &[
            e("root", None, [0.0, 0.0, 0.0]),
            e("hips", Some("root"), [0.0, 1.0, 0.0]),
            e("spine", Some("hips"), [0.0, 0.1, 0.0]),
            e("chest", Some("spine"), [0.0, 0.25, 0.0]),
            e("head", Some("chest"), [0.0, 0.3, 0.0]),
            e("thigh_l", Some("hips"), [0.1, -0.05, 0.0]),
            e("shin_l", Some("thigh_l"), [0.0, -0.45, 0.02]),
            e("foot_l", Some("shin_l"), [0.0, -0.45, -0.02]),
            e("thigh_r", Some("hips"), [-0.1, -0.05, 0.0]),
            e("shin_r", Some("thigh_r"), [0.0, -0.45, 0.02]),
            e("foot_r", Some("shin_r"), [0.0, -0.45, -0.02]),
            e("upperarm_l", Some("chest"), [0.15, 0.2, 0.0]),
            e("forearm_l", Some("upperarm_l"), [0.28, 0.0, 0.0]),
            e("hand_l", Some("forearm_l"), [0.25, 0.0, 0.0]),
            e("upperarm_r", Some("chest"), [-0.15, 0.2, 0.0]),
            e("forearm_r", Some("upperarm_r"), [-0.28, 0.0, 0.0]),
            e("hand_r", Some("forearm_r"), [-0.25, 0.0, 0.0]),
        ],
        1.0,
    )
}

/// A guar-like quadruped: digitigrade hind legs (thigh, shin, hock, paw,
/// toe; knee forward, hock backward), plantigrade front legs behind a short
/// scapula (elbow backward), a tail and a jaw. Hips at 0.9 m × `scale`.
pub fn quadruped(scale: f32) -> Skeleton {
    let mut j: Vec<J> = vec![
        e("root", None, [0.0, 0.0, 0.0]),
        e("pelvis", Some("root"), [0.0, 0.9, -0.45]),
        e("spine1", Some("pelvis"), [0.0, 0.02, 0.3]),
        e("spine2", Some("spine1"), [0.0, 0.02, 0.3]),
        e("chest", Some("spine2"), [0.0, 0.0, 0.3]),
        e("neck", Some("chest"), [0.0, 0.15, 0.2]),
        e("head", Some("neck"), [0.0, 0.1, 0.25]),
        e("jaw", Some("head"), [0.0, -0.08, 0.1]),
        e("tail1", Some("pelvis"), [0.0, 0.0, -0.3]),
        e("tail2", Some("tail1"), [0.0, -0.05, -0.3]),
        e("tail3", Some("tail2"), [0.0, -0.05, -0.3]),
    ];
    for (s, x) in [("l", 1.0f32), ("r", -1.0)] {
        let n = |base: &str| format!("{base}_{s}");
        j.extend([
            e(&n("thigh"), Some("pelvis"), [0.15 * x, -0.05, 0.0]),
            e(&n("shin"), Some(&n("thigh")), [0.0, -0.35, 0.15]),
            e(&n("hock"), Some(&n("shin")), [0.0, -0.3, -0.15]),
            e(&n("paw"), Some(&n("hock")), [0.0, -0.17, 0.03]),
            e(&n("toe"), Some(&n("paw")), [0.0, -0.03, 0.1]),
            e(&n("scapula"), Some("chest"), [0.12 * x, -0.04, 0.0]),
            e(&n("upper"), Some(&n("scapula")), [0.05 * x, -0.06, 0.0]),
            e(&n("fore"), Some(&n("upper")), [0.0, -0.38, -0.12]),
            e(&n("wrist"), Some(&n("fore")), [0.0, -0.42, 0.12]),
            e(&n("finger"), Some(&n("wrist")), [0.0, -0.04, 0.1]),
        ]);
    }
    build(&j, scale)
}

/// An insect-like hexapod: three leg pairs on one thorax (coxa, femur,
/// tibia, tarsus; knees up and out), a head and an abdomen.
pub fn hexapod() -> Skeleton {
    let mut j: Vec<J> = vec![
        e("root", None, [0.0, 0.0, 0.0]),
        e("thorax", Some("root"), [0.0, 0.3, 0.0]),
        e("head", Some("thorax"), [0.0, 0.02, 0.25]),
        e("abdomen1", Some("thorax"), [0.0, 0.0, -0.2]),
        e("abdomen2", Some("abdomen1"), [0.0, 0.0, -0.2]),
    ];
    for (row, z) in [(0, 0.12f32), (1, 0.0), (2, -0.12)] {
        for (s, x) in [("l", 1.0f32), ("r", -1.0)] {
            let n = |base: &str| format!("{base}{row}_{s}");
            j.extend([
                e(&n("coxa"), Some("thorax"), [0.08 * x, 0.0, z]),
                e(&n("femur"), Some(&n("coxa")), [0.12 * x, 0.1, 0.0]),
                e(&n("tibia"), Some(&n("femur")), [0.1 * x, -0.25, 0.0]),
                e(&n("tarsus"), Some(&n("tibia")), [0.05 * x, -0.15, 0.0]),
            ]);
        }
    }
    build(&j, 1.0)
}

/// A cliff-racer-like flyer in its flight pose: body 1 m up, neck and
/// head, a tail, two wings (upper, fore, hand, tip) and small legs hanging.
pub fn flyer() -> Skeleton {
    let mut j: Vec<J> = vec![
        e("root", None, [0.0, 0.0, 0.0]),
        e("body", Some("root"), [0.0, 1.0, 0.0]),
        e("neck", Some("body"), [0.0, 0.05, 0.25]),
        e("head", Some("neck"), [0.0, 0.05, 0.2]),
        e("tail1", Some("body"), [0.0, 0.0, -0.3]),
        e("tail2", Some("tail1"), [0.0, 0.0, -0.3]),
        e("tail3", Some("tail2"), [0.0, 0.0, -0.3]),
    ];
    for (s, x) in [("l", 1.0f32), ("r", -1.0)] {
        let n = |base: &str| format!("{base}_{s}");
        j.extend([
            e(&n("wing_upper"), Some("body"), [0.1 * x, 0.05, 0.0]),
            e(&n("wing_fore"), Some(&n("wing_upper")), [0.4 * x, 0.0, 0.0]),
            e(&n("wing_hand"), Some(&n("wing_fore")), [0.4 * x, 0.0, 0.05]),
            e(&n("wing_tip"), Some(&n("wing_hand")), [0.5 * x, 0.0, -0.1]),
            e(&n("thigh"), Some("body"), [0.08 * x, -0.1, 0.0]),
            e(&n("shin"), Some(&n("thigh")), [0.0, -0.25, 0.05]),
            e(&n("foot"), Some(&n("shin")), [0.0, -0.25, -0.03]),
        ]);
    }
    build(&j, 1.0)
}

/// A curve sampled every 25 ms.
pub fn sampled<T: crate::CurveValue>(duration: f32, f: impl Fn(f32) -> T) -> Curve<T> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (duration / 0.025).round().max(1.0) as usize;
    #[allow(clippy::cast_precision_loss)]
    let times: Vec<f32> = (0..=n).map(|i| duration * i as f32 / n as f32).collect();
    let values = times.iter().map(|&t| f(t)).collect();
    Curve::linear(times, values)
}

/// A gait for any skeleton: the root moves forward at `speed`; each leg
/// `(root joint, effector, phase)` swings its root joint about its local X
/// with amplitude `swing`, in stance for the first `duty` of its cycle
/// (an authored `contact.<effector>` curve says so). Loops over `duration`.
/// Planted feet keep still in the world when `speed` is the stance sweep,
/// `2·L·sin(swing) / (duty·duration)` for a leg hanging `L` below its root.
pub fn gait(skeleton: &Skeleton, name: &str, duration: f32, speed: f32, swing: f32, duty: f32, legs: &[(&str, &str, f32)]) -> SourceClip {
    let d = duration;
    let mut tracks =
        vec![SourceTrack { joint: "root".into(), translation: Some(sampled(d, move |t| Vec3::new(0.0, 0.0, speed * t))), ..SourceTrack::default() }];
    let mut curves = Vec::new();
    for &(root, effector, phase) in legs {
        let rest = skeleton.rest()[skeleton.find(root).expect("leg root")].rotation;
        tracks.push(SourceTrack {
            joint: root.into(),
            // Stance (the first `duty` of the cycle) sweeps the foot from
            // forward (-swing) to back (+swing); the swing phase returns it.
            rotation: Some(sampled(d, move |t| {
                let u = (t / d + phase).rem_euclid(1.0);
                let a = if u < duty { -swing * (PI * u / duty).cos() } else { swing * (PI * (u - duty) / (1.0 - duty)).cos() };
                rest * Quat::from_rotation_x(a)
            })),
            ..SourceTrack::default()
        });
        curves.push(SourceCurve {
            name: format!("contact.{effector}"),
            curve: sampled(d, move |t| if (t / d + phase).rem_euclid(1.0) < duty { 1.0 } else { 0.0 }),
        });
    }
    SourceClip { name: name.into(), duration: d, tracks, curves, events: vec![Event::point(0.0, "step")], markers: Vec::new() }
}
