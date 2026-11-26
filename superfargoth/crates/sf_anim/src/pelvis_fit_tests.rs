//! Pelvis fit tests (`pelvis_fit.rs`): the fixture biped sitting down,
//! standing up and idling seated with its feet planted, fitted to a seat.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use glam::{Affine3A, Quat, Vec3};
use serde::Serialize;

use crate::fixtures::{biped, sampled};
use crate::ik::{LimbGoal, LimbSolver, solve_limb};
use crate::*;

const LEGS: [(&str, f32); 2] = [("l", 0.1), ("r", -0.1)];
/// Where the source keeps each foot (model space, x from [`LEGS`]).
const FOOT: Vec3 = Vec3::new(0.0, 0.06, 0.25);

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// A one-second take whose hips (local to the static root) sit at `h(t)`,
/// the legs solved each sample so the feet stay planted at [`FOOT`] with
/// their rest (identity) rotation, knees forward. Keyed every 25 ms.
fn seated_source(h: impl Fn(f32) -> f32 + Copy + 'static) -> SourceClip {
    let pose = move |t: f32| -> Vec<Trs> {
        let s = biped();
        let mut lp = LocalPose::rest(&s);
        lp.set_translation(s.find("hips").unwrap(), Vec3::new(0.0, h(t), 0.0));
        let mut mp = ModelPose::new(s.len());
        local_to_model(&s, &lp, &mut mp, &Affine3A::IDENTITY);
        for (side, x) in LEGS {
            let j: Vec<usize> = ["thigh", "shin", "foot"].iter().map(|n| s.find(&format!("{n}_{side}")).unwrap()).collect();
            let hip = mp.translation(j[0]);
            let goal = LimbGoal { position: FOOT + Vec3::X * x, rotation: Some(Quat::IDENTITY), pole: hip + Vec3::new(0.0, -0.45, 1.0), weight: 1.0 };
            solve_limb(&s, &mut lp, &mut mp, &Affine3A::IDENTITY, &j, LimbSolver::TwoBone, &goal);
        }
        lp.to_vec()
    };
    let s = biped();
    let mut tracks = vec![SourceTrack { joint: "hips".into(), translation: Some(sampled(1.0, move |t| Vec3::new(0.0, h(t), 0.0))), ..SourceTrack::default() }];
    for n in ["thigh_l", "shin_l", "foot_l", "thigh_r", "shin_r", "foot_r"] {
        let j = s.find(n).unwrap();
        tracks.push(SourceTrack { joint: n.into(), rotation: Some(sampled(1.0, move |t| pose(t)[j].rotation)), ..SourceTrack::default() });
    }
    SourceClip { name: "seat".into(), duration: 1.0, tracks, curves: Vec::new(), events: Vec::new(), markers: Vec::new() }
}

fn sit_down() -> SourceClip {
    seated_source(|t| 0.9 - 0.4 * smooth(t))
}

fn stand_up() -> SourceClip {
    seated_source(|t| 0.5 + 0.4 * smooth(t))
}

fn cook(source: &SourceClip, fit: Option<PelvisFit>) -> (Clip, CookReport) {
    let s = biped();
    let profile = detect_profile(&s, &RigHints::default()).unwrap();
    let rules = ClipRules { pelvis_fit: fit, ..ClipRules::default() };
    cook_clip_report(&s, Some(&profile), source, &rules, &CookSettings::default()).unwrap()
}

fn model_at(clip: &Clip, t: f32) -> ModelPose {
    let s = biped();
    let mut lp = LocalPose::new(s.len());
    clip.sample(t, &mut lp, None);
    let mut mp = ModelPose::new(s.len());
    local_to_model(&s, &lp, &mut mp, &Affine3A::IDENTITY);
    mp
}

fn joint(name: &str) -> usize {
    biped().find(name).unwrap()
}

fn frame_times(clip: &Clip) -> Vec<f32> {
    let n = clip.frame_count as usize;
    (0..n).map(|f| clip.duration * f as f32 / (n - 1) as f32).collect()
}

/// Every frame of `fitted`: the feet within 1 mm and 0.1° of `plain`'s.
fn assert_feet_kept(fitted: &Clip, plain: &Clip) {
    for t in frame_times(fitted) {
        let (a, b) = (model_at(fitted, t), model_at(plain, t));
        for (side, _) in LEGS {
            let foot = joint(&format!("foot_{side}"));
            let d = a.translation(foot).distance(b.translation(foot));
            assert!(d < 1e-3, "foot_{side} moved {d} m at {t}");
            let angle = angle_between(a.rotation(foot), b.rotation(foot)).to_degrees();
            assert!(angle < 0.1, "foot_{side} turned {angle} deg at {t}");
        }
    }
}

fn stats(report: &CookReport) -> PelvisFitStats {
    report.pelvis_fit.clone().expect("the rule was given").expect("fitted")
}

/// The frames of `plain` fitted on the target-rig path ([`fit_frame_clip`]).
fn fitted_frames(plain: &Clip, fit: &PelvisFit) -> (FrameClip, FrameClip, PelvisFitStats) {
    let s = biped();
    let profile = detect_profile(&s, &RigHints::default()).unwrap();
    let before = FrameClip::from_clip(plain);
    let mut after = before.clone();
    let stats = fit_frame_clip(&s, &profile, &mut after, fit).unwrap();
    (before, after, stats)
}

fn hips_height(frames: &FrameClip, f: usize) -> f32 {
    let s = biped();
    let j = s.len();
    let pose = &frames.frames[f * j..(f + 1) * j];
    (pose[joint("root")].to_affine() * pose[joint("hips")].to_affine()).translation.y
}

#[test]
fn sit_down_lands_on_the_seat_with_the_feet_planted() {
    let fit = PelvisFit::new(0.6, HubEnd::Last);
    let (plain, _) = cook(&sit_down(), None);
    let (fitted, report) = cook(&sit_down(), Some(fit.clone()));
    let s = stats(&report);
    println!("{s:?}, cook error {} / {}", fitted.meta.max_error, plain.meta.max_error);
    assert!((s.lift_m - 0.1).abs() < 1e-4, "{s:?}");
    assert!(s.max_foot_err_m < 1e-3, "{s:?}");
    assert!(s.frames_lifted > 0 && s.frames_lifted < fitted.frame_count, "standing frames untouched: {s:?}");
    let hips = joint("hips");
    let y = model_at(&fitted, fitted.duration).translation(hips).y;
    assert!((y - 0.6).abs() <= 1e-4 + fitted.meta.max_error, "hub pelvis at {y}");
    assert_feet_kept(&fitted, &plain);
    // The same on decoded frames (the target-rig path): exact at the hub,
    // the standing first frame bit for bit.
    let (before, after, fs) = fitted_frames(&plain, &fit);
    let n = after.frame_count;
    assert!((hips_height(&after, n - 1) - 0.6).abs() < 1e-4, "{}", hips_height(&after, n - 1));
    let j = biped().len();
    assert_eq!(after.frames[..j], before.frames[..j]);
    assert!(fs.max_foot_err_m < 1e-3, "{fs:?}");
}

#[test]
fn stand_up_leaves_the_seat_with_the_feet_planted() {
    let fit = PelvisFit::new(0.6, HubEnd::First);
    let (plain, _) = cook(&stand_up(), None);
    let (fitted, report) = cook(&stand_up(), Some(fit.clone()));
    let s = stats(&report);
    assert!((s.lift_m - 0.1).abs() < 1e-4, "{s:?}");
    assert!(s.max_foot_err_m < 1e-3, "{s:?}");
    let y = model_at(&fitted, 0.0).translation(joint("hips")).y;
    assert!((y - 0.6).abs() <= 1e-4 + fitted.meta.max_error, "hub pelvis at {y}");
    assert_feet_kept(&fitted, &plain);
    let (before, after, _) = fitted_frames(&plain, &fit);
    assert!((hips_height(&after, 0) - 0.6).abs() < 1e-4, "{}", hips_height(&after, 0));
    let (j, n) = (biped().len(), after.frame_count);
    assert_eq!(after.frames[(n - 1) * j..], before.frames[(n - 1) * j..]);
}

#[test]
fn a_seated_loop_is_lifted_whole() {
    let idle = seated_source(|t| 0.5 + 0.02 * (std::f32::consts::PI * t).sin().powi(2));
    let (plain, _) = cook(&idle, None);
    let (fitted, report) = cook(&idle, Some(PelvisFit::new(0.6, HubEnd::First)));
    let s = stats(&report);
    assert!((s.lift_m - 0.1).abs() < 1e-4, "{s:?}");
    assert_eq!(s.frames_lifted, fitted.frame_count);
    let hips = joint("hips");
    let tolerance = 1e-4 + fitted.meta.max_error + plain.meta.max_error;
    for t in frame_times(&fitted) {
        let lift = model_at(&fitted, t).translation(hips).y - model_at(&plain, t).translation(hips).y;
        assert!((lift - 0.1).abs() <= tolerance, "lift {lift} at {t}");
    }
    assert_feet_kept(&fitted, &plain);
}

#[test]
fn a_seat_out_of_reach_straightens_the_legs() {
    let (fitted, report) = cook(&sit_down(), Some(PelvisFit::new(1.2, HubEnd::Last)));
    let s = stats(&report);
    println!("{s:?}");
    assert!((s.lift_m - 0.7).abs() < 1e-4, "{s:?}");
    assert!(s.max_foot_err_m > 0.0, "{s:?}");
    for t in frame_times(&fitted) {
        let m = model_at(&fitted, t);
        assert!(m.mats.iter().all(Affine3A::is_finite), "NaN at {t}");
    }
    let sk = biped();
    let m = model_at(&fitted, fitted.duration);
    let rest = |j: usize| sk.rest()[j].translation.length();
    for (side, _) in LEGS {
        let (thigh, shin, foot) = (joint(&format!("thigh_{side}")), joint(&format!("shin_{side}")), joint(&format!("foot_{side}")));
        let reach = (rest(shin) + rest(foot)) * 0.9999;
        let d = m.translation(thigh).distance(m.translation(foot));
        assert!((d - reach).abs() < 1e-3, "leg_{side} at {d} of {reach}");
    }
}

/// [`ClipRules`] as it was before `pelvis_fit` (its bake-key bytes).
#[derive(Serialize)]
struct ClipRulesBefore {
    range: Option<(f32, f32)>,
    looping: Looping,
    root_motion: RootMotionRules,
    additive: Option<AdditiveBase>,
    events: Vec<Event>,
    markers: Vec<SyncMarker>,
    contacts: ContactRules,
    loop_cut: Option<LoopCutRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<DistanceRules>,
}

impl From<&ClipRules> for ClipRulesBefore {
    fn from(r: &ClipRules) -> Self {
        Self {
            range: r.range,
            looping: r.looping,
            root_motion: r.root_motion.clone(),
            additive: r.additive,
            events: r.events.clone(),
            markers: r.markers.clone(),
            contacts: r.contacts.clone(),
            loop_cut: r.loop_cut.clone(),
            distance: r.distance,
        }
    }
}

#[test]
fn without_the_rule_keys_and_clips_do_not_change() {
    let plain = ClipRules::default();
    let distance = ClipRules { distance: Some(DistanceRules::new(DistanceCurve::Stop)), loop_cut: Some(LoopCutRules::default()), ..ClipRules::default() };
    for r in [&plain, &distance] {
        assert_eq!(r.key_bytes(), postcard::to_stdvec(&ClipRulesBefore::from(r)).unwrap());
        let fitted = ClipRules { pelvis_fit: Some(PelvisFit::new(0.6, HubEnd::Last)), ..r.clone() };
        assert_ne!(fitted.key_bytes(), r.key_bytes());
    }
    // A rule that can't apply (no rig profile) cooks the clip as without it.
    let s = biped();
    let source = sit_down();
    let none = cook_clip_report(&s, None, &source, &plain, &CookSettings::default()).unwrap();
    let rules = ClipRules { pelvis_fit: Some(PelvisFit::new(0.6, HubEnd::Last)), ..plain };
    let refused = cook_clip_report(&s, None, &source, &rules, &CookSettings::default()).unwrap();
    assert_eq!(none.1.pelvis_fit, None);
    assert_eq!(refused.1.pelvis_fit, Some(Err(PelvisFitError::NoLegs)));
    assert_eq!(none.0.to_bytes(), refused.0.to_bytes());
}
