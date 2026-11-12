//! Loop cutter tests (`loop_cut.rs`): synthetic walks that ramp up, hold a
//! steady pace and slow to a stop, as a diffusion take does.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::f32::consts::PI;

use glam::{Quat, Vec3};

use crate::fixtures::biped;
use crate::*;

const RATE: f32 = 30.0;
/// Leg length of the fixture biped (thigh to foot).
const LEG: f32 = 0.9;

/// Distance walked by time `t` of a take of `d` s that ramps up over `ramp`
/// s (smoothstep speed), holds `speed` and ramps down over the last `ramp`.
fn walked(t: f32, d: f32, ramp: f32, speed: f32) -> f32 {
    // Integral of the smoothstep speed ramp over `tau` seconds.
    let f = |tau: f32| {
        let u = (tau / ramp).clamp(0.0, 1.0);
        speed * ramp * (u * u * u - 0.5 * u * u * u * u)
    };
    if t < ramp {
        f(t)
    } else if t <= d - ramp {
        speed * (0.5 * ramp + t - ramp)
    } else {
        speed * (d - ramp) - f(d - t)
    }
}

/// A biped walk take: the root walks `walked`, each leg's phase follows the
/// distance (so planted feet stay still through the ramps), stance sweeps
/// the foot back in a straight line (the hips bob to keep it on the floor),
/// swing bends the knee to lift it; arms counter-swing. One stride is
/// `period` s at the steady `speed`. Keyed at [`RATE`].
fn take(d: f32, ramp: f32, speed: f32, period: f32) -> SourceClip {
    drifting_take(d, ramp, speed, period, 0.0)
}

/// [`take`] whose arm swing grows by `drift` rad per second: no two strides match.
fn drifting_take(d: f32, ramp: f32, speed: f32, period: f32, drift: f32) -> SourceClip {
    let skeleton = biped();
    let duty = 0.6;
    let stride = speed * period;
    let s = speed * duty * period / (2.0 * LEG);
    let phase = move |t: f32| walked(t, d, ramp, speed) / stride;
    // Thigh angle about X (positive swings the foot back) and knee bend.
    let leg = move |u: f32| -> (f32, f32, bool) {
        let u = u.rem_euclid(1.0);
        if u < duty {
            ((s * (2.0 * u / duty - 1.0)).asin(), 0.0, true)
        } else {
            let v = (u - duty) / (1.0 - duty);
            (s.asin() * (PI * v).cos(), 0.7 * (PI * v).sin(), false)
        }
    };
    let n = (d * RATE).round() as usize;
    let times: Vec<f32> = (0..=n).map(|k| k as f32 / RATE).collect();
    let curve_q = |f: &dyn Fn(f32) -> Quat| Curve::linear(times.clone(), times.iter().map(|&t| f(t)).collect());
    let curve_v = |f: &dyn Fn(f32) -> Vec3| Curve::linear(times.clone(), times.iter().map(|&t| f(t)).collect());
    let rest = |name: &str| skeleton.rest()[skeleton.find(name).unwrap()];
    let mut tracks = vec![SourceTrack {
        joint: "root".into(),
        translation: Some(curve_v(&|t| Vec3::new(0.0, 0.0, walked(t, d, ramp, speed)))),
        ..SourceTrack::default()
    }];
    let hips = rest("hips").translation;
    tracks.push(SourceTrack {
        joint: "hips".into(),
        translation: Some(curve_v(&|t| {
            let p = phase(t);
            let stance = [0.0, 0.5].iter().map(|o| leg(p + o)).filter(|l| l.2).map(|l| l.0.cos()).fold(0.0f32, f32::max);
            hips - Vec3::Y * LEG * (1.0 - stance)
        })),
        ..SourceTrack::default()
    });
    for (side, offset) in [("l", 0.0), ("r", 0.5)] {
        let thigh = rest(&format!("thigh_{side}")).rotation;
        let shin = rest(&format!("shin_{side}")).rotation;
        let arm = rest(&format!("upperarm_{side}")).rotation;
        tracks.push(SourceTrack {
            joint: format!("thigh_{side}"),
            rotation: Some(curve_q(&|t| thigh * Quat::from_rotation_x(leg(phase(t) + offset).0))),
            ..SourceTrack::default()
        });
        tracks.push(SourceTrack {
            joint: format!("shin_{side}"),
            rotation: Some(curve_q(&|t| shin * Quat::from_rotation_x(leg(phase(t) + offset).1))),
            ..SourceTrack::default()
        });
        tracks.push(SourceTrack {
            joint: format!("upperarm_{side}"),
            rotation: Some(curve_q(&|t| arm * Quat::from_rotation_y(-(0.3 + drift * t) * (2.0 * PI * (phase(t) + offset)).cos()))),
            ..SourceTrack::default()
        });
    }
    SourceClip { name: "take".into(), duration: n as f32 / RATE, tracks, curves: Vec::new(), events: Vec::new(), markers: Vec::new() }
}

fn cut_rules(cut: LoopCutRules) -> ClipRules {
    ClipRules { looping: Looping::Whole, loop_cut: Some(cut), ..ClipRules::default() }
}

fn cook_cut(source: &SourceClip, cut: LoopCutRules) -> (Clip, Result<LoopCut, LoopCutError>) {
    let (clip, report) = cook_clip_report(&biped(), None, source, &cut_rules(cut), &CookSettings::default()).unwrap();
    (clip, report.loop_cut.expect("the rule was given"))
}

#[test]
fn cuts_whole_strides_inside_the_steady_part() {
    // 1 s strides at 1.2 m/s; ramps over the first and last second.
    let (clip, cut) = cook_cut(&take(6.0, 1.0, 1.2, 1.0), LoopCutRules::default());
    let cut = cut.unwrap();
    println!("{cut:?}");
    assert_eq!(cut.strides, 1, "fewest strides among equal matches");
    assert!((cut.end - cut.start - 1.0).abs() < 1e-4, "one whole stride: {}..{}", cut.start, cut.end);
    assert!(cut.start >= 1.0 && cut.end <= 5.0, "inside the steady part: {}..{}", cut.start, cut.end);
    assert!(cut.cut_pose_error < 1e-3, "a periodic plateau matches itself: {}", cut.cut_pose_error);
    assert!((cut.speed - 1.2).abs() < 1e-3 && (cut.plateau_speed - 1.2).abs() < 1e-2, "{} {}", cut.speed, cut.plateau_speed);
    assert!(cut.turn.abs() < 1e-4);
    assert!((clip.duration - 1.0).abs() < 1e-4);
    assert_eq!(clip.loop_range, Some((0.0, clip.duration)));
    assert!((clip.meta.average_velocity - Vec3::Z * 1.2).length() < 1e-2, "{}", clip.meta.average_velocity);

    // Two strides asked for: two whole periods.
    let (_, two) = cook_cut(&take(6.0, 1.0, 1.2, 1.0), LoopCutRules { min_strides: 2, ..LoopCutRules::default() });
    let two = two.unwrap();
    assert_eq!(two.strides, 2);
    assert!((two.end - two.start - 2.0).abs() < 1e-4 && two.start >= 1.0 && two.end <= 5.0, "{}..{}", two.start, two.end);
}

#[test]
fn the_fade_closes_the_seam() {
    // The arm swing grows: no two strides match, the fade hides the rest.
    let (clip, cut) = cook_cut(&drifting_take(6.0, 1.0, 1.2, 1.0, 0.1), LoopCutRules::default());
    let cut = cut.unwrap();
    println!("{cut:?}");
    assert!(cut.cut_pose_error > 1e-3, "the hard cut would pop: {}", cut.cut_pose_error);
    assert!(cut.seam_pose_error < 1e-4, "seam pose {}", cut.seam_pose_error);
    assert!(cut.seam_speed_jump < 1e-3, "seam speed {}", cut.seam_speed_jump);
    assert!(cut.seam_foot_jump < 2e-3, "seam foot {}", cut.seam_foot_jump);
    assert_eq!(cut.blend_frames, 4);
    // The cooked loop's last pose is its first (root motion carries the cycle).
    let skeleton = biped();
    let (mut first, mut last) = (LocalPose::new(skeleton.len()), LocalPose::new(skeleton.len()));
    clip.sample(0.0, &mut first, None);
    clip.sample(clip.duration, &mut last, None);
    for (j, (a, b)) in first.to_vec().iter().zip(last.to_vec()).enumerate() {
        assert!(a.translation.distance(b.translation) < 1e-3 && angle_between(a.rotation, b.rotation) < 1e-3, "joint {j}: {a:?} vs {b:?}");
    }
}

#[test]
fn contacts_stay_continuous_at_the_seam() {
    for strides in [1, 2] {
        let (clip, cut) = cook_cut(&take(6.0, 1.0, 1.2, 1.0), LoopCutRules { min_strides: strides, max_strides: strides, ..LoopCutRules::default() });
        assert_eq!(cut.unwrap().strides, strides);
        for foot in ["foot_l", "foot_r"] {
            let contact = &clip.curves.iter().find(|c| c.name == format!("contact.{foot}")).expect("contact curve").values;
            assert_eq!(contact.first(), contact.last(), "{foot}: the closing frame repeats the first");
            let downs = clip.markers.iter().filter(|m| m.name == format!("down.{foot}")).count();
            assert_eq!(downs, strides as usize, "{foot}: one touchdown per stride, none added at the seam");
            // Stances: as many planted runs as strides, counted round the loop.
            let runs = (0..contact.len() - 1).filter(|&f| contact[f] > 0.5 && contact[(f + 1) % (contact.len() - 1)] <= 0.5).count();
            assert_eq!(runs, strides as usize, "{foot}: {contact:?}");
        }
    }
}

#[test]
fn takes_without_a_loop_stay_unlooped() {
    let skeleton = biped();
    // Too short for three strides.
    let (clip, cut) = cook_cut(&take(3.0, 0.5, 1.2, 1.0), LoopCutRules { min_strides: 3, max_strides: 3, ..LoopCutRules::default() });
    let err = cut.unwrap_err();
    assert!(matches!(err, LoopCutError::TooShort { strides: 3, .. }), "{err}");
    assert!(err.to_string().contains("shorter than 3 stride"), "{err}");
    assert_eq!(clip.loop_range, None);
    // Standing still: each foot plants once and never again.
    let stand = SourceClip {
        name: "stand".into(),
        duration: 3.0,
        tracks: vec![SourceTrack { joint: "hips".into(), translation: Some(Curve::constant(skeleton.rest()[1].translation)), ..SourceTrack::default() }],
        ..SourceClip::default()
    };
    let (clip, cut) = cook_cut(&stand, LoopCutRules::default());
    assert_eq!(cut.unwrap_err(), LoopCutError::NoRepeatedContact);
    assert_eq!(clip.loop_range, None);
    // No contacts at all.
    let rules = ClipRules { contacts: ContactRules { enabled: false, ..ContactRules::default() }, ..cut_rules(LoopCutRules::default()) };
    let (clip, report) = cook_clip_report(&skeleton, None, &take(6.0, 1.0, 1.2, 1.0), &rules, &CookSettings::default()).unwrap();
    assert_eq!(report.loop_cut, Some(Err(LoopCutError::NoContacts)));
    assert_eq!(clip.loop_range, None);
}

#[test]
fn opt_out_cooks_as_before() {
    let skeleton = biped();
    let source = take(6.0, 1.0, 1.2, 1.0);
    let settings = CookSettings::default();
    for rules in [ClipRules::default(), ClipRules { looping: Looping::Whole, ..ClipRules::default() }] {
        let (clip, report) = cook_clip_report(&skeleton, None, &source, &rules, &settings).unwrap();
        assert_eq!(report, CookReport::default());
        assert_eq!(clip.to_bytes(), cook_clip(&skeleton, &source, &rules, &settings).unwrap().to_bytes());
    }
}

#[test]
fn rules_read_from_ron() {
    let cut: LoopCutRules = ron::from_str("(max_strides: 3)").unwrap();
    assert_eq!(cut, LoopCutRules { max_strides: 3, ..LoopCutRules::default() });
    let rules: ClipRules = ron::from_str("(looping: Whole)").unwrap_or_default();
    assert_eq!(rules.loop_cut, None);
}
