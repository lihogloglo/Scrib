//! Baked hand and foot contacts (`annotate.rs`) on synthetic tracks and a cooked synthetic climb.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::f32::consts::{PI, TAU};

use glam::{Quat, Vec3};

use crate::annotate::planted_spans;
use crate::fixtures::{biped, sampled};
use crate::*;

const RATE: f32 = 30.0;
const DT: f32 = 1.0 / RATE;
/// The fixture biped's arm and leg lengths, about.
const ARM: f32 = 0.53;
const LEG: f32 = 0.9;

fn track(duration: f32, f: impl Fn(f32) -> Vec3) -> Vec<Vec3> {
    let n = (duration * RATE).round() as usize + 1;
    (0..n).map(|i| f(i as f32 * DT)).collect()
}

fn times(spans: &[(usize, usize)]) -> Vec<(f32, f32)> {
    spans.iter().map(|&(a, b)| (a as f32 * DT, b as f32 * DT)).collect()
}

fn near(t: f32, want: f32) -> bool {
    (t - want).abs() <= DT + 1e-4
}

#[test]
fn a_hand_that_holds_at_a_lip_gets_one_contact() {
    // Reaches up and decelerates onto the lip by 0.4 s, holds while the body climbs, lets go at 0.9 s.
    let lip = Vec3::new(0.2, 1.1, 0.4);
    let hand = track(1.5, |t| {
        if t < 0.4 {
            lip - Vec3::new(0.0, 1.0, 0.5).normalize() * 6.0 * (0.4 - t) * (0.4 - t)
        } else if t <= 0.9 {
            lip
        } else {
            lip + Vec3::Y * 1.5 * (t - 0.9)
        }
    });
    let shoulder = track(1.5, |t| Vec3::new(0.2, 1.2 + 0.5 * t, 0.0));
    let spans = times(&planted_spans(&hand, Some(&shoulder), ARM, DT));
    assert_eq!(spans.len(), 1, "{spans:?}");
    let (start, end) = spans[0];
    assert!(near(start, 0.4) && near(end, 0.9), "{spans:?}");
}

#[test]
fn a_swinging_hand_has_no_contact() {
    // Slow only at its turning points, for a frame or two; the shoulder climbs, so bracing passes.
    let hand = track(2.0, |t| Vec3::new(0.3 * (TAU * t).sin(), 1.0 + 0.1 * t, 0.2));
    let shoulder = track(2.0, |t| Vec3::new(0.0, 1.4 + 0.3 * t, 0.0));
    assert_eq!(planted_spans(&hand, Some(&shoulder), ARM, DT), []);
}

#[test]
fn a_resting_hand_is_not_a_contact_but_a_still_foot_is() {
    // Nothing moves (an idle): the hand isn't braced, the foot carries weight.
    let hand = track(1.0, |_| Vec3::new(0.6, 1.0, 0.0));
    let shoulder = track(1.0, |_| Vec3::new(0.15, 1.55, 0.0));
    assert_eq!(planted_spans(&hand, Some(&shoulder), ARM, DT), []);
    let foot = track(1.0, |_| Vec3::new(0.1, 0.05, 0.0));
    assert_eq!(planted_spans(&foot, None, LEG, DT), [(0, 30)]);
}

#[test]
fn a_step_plants_its_foot_at_both_ends() {
    // Planted to 0.3 s, a 0.6 m eased swing to 0.7 s, planted to the end.
    let foot = track(1.2, |t| {
        let u = ((t - 0.3) / 0.4).clamp(0.0, 1.0);
        Vec3::new(0.1, 0.05 + 0.1 * (PI * u).sin(), 0.3 * (1.0 - (PI * u).cos()))
    });
    let spans = times(&planted_spans(&foot, None, LEG, DT));
    assert_eq!(spans.len(), 2, "{spans:?}");
    assert!(near(spans[0].0, 0.0) && near(spans[0].1, 0.3), "{spans:?}");
    assert!(near(spans[1].0, 0.7) && near(spans[1].1, 1.2), "{spans:?}");
}

fn rise(t: f32) -> Vec3 {
    Vec3::new(0.0, 0.5 * t, 0.3 * t)
}

/// Where the fixture biped's left hand rests in the rest pose: the lip it reaches for.
const LIP: Vec3 = Vec3::new(0.68, 1.55, 0.0);

/// The fixture biped climbing 0.5 m up and 0.3 m forward over 1 s. Its left hand reaches up,
/// decelerating onto the lip by 0.3 s, and holds there (the upper arm counter-moves in the
/// still chest); its right arm swings.
fn climb() -> SourceClip {
    let s = biped();
    let rest = |j: &str| s.rest()[s.find(j).unwrap()];
    let (upper_l, upper_r) = (rest("upperarm_l").translation, rest("upperarm_r").rotation);
    let reach = |t: f32| if t < 0.3 { -Vec3::Y * 6.0 * (0.3 - t) * (0.3 - t) } else { Vec3::ZERO };
    SourceClip {
        name: "climb".into(),
        duration: 1.0,
        tracks: vec![
            SourceTrack { joint: "root".into(), translation: Some(sampled(1.0, rise)), ..SourceTrack::default() },
            SourceTrack {
                joint: "upperarm_l".into(),
                translation: Some(sampled(1.0, move |t| upper_l - rise(t) + reach(t))),
                ..SourceTrack::default()
            },
            SourceTrack {
                joint: "upperarm_r".into(),
                rotation: Some(sampled(1.0, move |t| upper_r * Quat::from_rotation_z(0.8 * (TAU * t).sin()))),
                ..SourceTrack::default()
            },
        ],
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
    }
}

fn climb_rules() -> ClipRules {
    ClipRules { root_motion: RootMotionRules { vertical: true, ..RootMotionRules::default() }, ..ClipRules::default() }
}

#[test]
fn a_cooked_climb_annotates_the_hand_on_the_lip() {
    let s = biped();
    let profile = detect_profile(&s, &RigHints::default()).unwrap();
    let clip = cook_clip_for(&s, Some(&profile), &climb(), &climb_rules(), &CookSettings::default()).unwrap();
    assert!(clip.root_motion.is_some());
    let dt = 1.0 / clip.rate;
    // The feet rise with the body (0.58 m/s, over the plant speed); the right hand swings.
    let contacts = &clip.meta.contacts;
    assert_eq!(contacts.len(), 1, "{contacts:?}");
    let c = &contacts[0];
    assert_eq!((c.kind, c.side, c.limb.as_str(), c.joint.as_str()), (ContactKind::Hand, Side::Left, "arm_l", "hand_l"));
    assert!((c.start - 0.3).abs() <= dt + 1e-4 && c.end >= clip.duration - dt - 1e-4, "{c:?}");
    // Clip space: on the lip. Root space: the root has risen and moved on since the clip began.
    assert!(c.position.distance(LIP) < 0.02, "{c:?}");
    assert!(c.local.distance(LIP - rise(c.start)) < 0.02, "{c:?}");
    assert_eq!(clip.meta.contact_span(ContactKind::Hand), Some((c.start, c.end)));
    assert_eq!(clip.meta.contact_span(ContactKind::Foot), None);
    let back = Clip::from_bytes(&clip.to_bytes()).unwrap();
    assert_eq!(back.meta.contacts, clip.meta.contacts);
}

#[test]
fn annotation_needs_a_profile_and_enabled_contacts() {
    let s = biped();
    let profile = detect_profile(&s, &RigHints::default()).unwrap();
    let settings = CookSettings::default();
    let with = cook_clip_for(&s, Some(&profile), &climb(), &climb_rules(), &settings).unwrap();
    let mut clip = cook_clip_for(&s, None, &climb(), &climb_rules(), &settings).unwrap();
    assert!(clip.meta.contacts.is_empty(), "no profile, no annotation");
    annotate_contacts(&s, &profile, &mut clip);
    assert_eq!(clip.meta.contacts, with.meta.contacts, "deterministic, and re-annotating replaces");
    annotate_contacts(&s, &profile, &mut clip);
    assert_eq!(clip.meta.contacts, with.meta.contacts);
    let off = ClipRules { contacts: ContactRules { enabled: false, ..ContactRules::default() }, ..climb_rules() };
    assert!(cook_clip_for(&s, Some(&profile), &climb(), &off, &settings).unwrap().meta.contacts.is_empty());
}

#[test]
fn a_hand_sliding_up_the_face_as_the_body_climbs_past_it_gets_one_contact() {
    // A waist mantle as keyframed: the hand lands at 0.2 s, its palm slides up the face at
    // 0.7 m/s (over the still speed) while the shoulder rises past it at 3 m/s, and leaves at 0.4 s.
    let lip = Vec3::new(0.2, 1.0, 0.8);
    let hand = track(0.8, |t| {
        if t < 0.2 {
            lip - Vec3::new(0.0, 0.3, 1.0).normalize() * 4.0 * (0.2 - t)
        } else if t <= 0.4 {
            lip + Vec3::Y * 0.7 * (t - 0.2)
        } else {
            lip + Vec3::Y * (0.14 + 4.0 * (t - 0.4))
        }
    });
    let shoulder = track(0.8, |t| Vec3::new(0.2, 0.6 + 3.0 * t, 0.5));
    let spans = times(&planted_spans(&hand, Some(&shoulder), ARM, DT));
    assert_eq!(spans.len(), 1, "{spans:?}");
    assert!(near(spans[0].0, 0.2) && near(spans[0].1, 0.4), "{spans:?}");
}

#[test]
fn a_hand_carried_by_the_climbing_body_has_no_contact() {
    // The arm locked to the body: slow or fast, the shoulder never moves past the hand.
    for rate in [0.15, 0.7] {
        let shoulder = track(1.0, |t| Vec3::new(0.15, 1.4 + rate * t, 0.3 * t));
        let hand: Vec<Vec3> = shoulder.iter().map(|s| *s + Vec3::new(0.3, -0.4, 0.2)).collect();
        assert_eq!(planted_spans(&hand, Some(&shoulder), ARM, DT), [], "rising {rate} m/s");
    }
}

#[test]
fn a_retargeted_clip_inherits_its_source_contacts() {
    let s = biped();
    let profile = detect_profile(&s, &RigHints::default()).unwrap();
    let source = cook_clip_for(&s, Some(&profile), &climb(), &climb_rules(), &CookSettings::default()).unwrap();
    let mut clip = source.clone();
    clip.meta.contacts.clear();
    transfer_contacts(&source.meta.contacts, &s, &profile, &mut clip);
    assert_eq!(clip.meta.contacts.len(), source.meta.contacts.len());
    for (got, want) in clip.meta.contacts.iter().zip(&source.meta.contacts) {
        assert_eq!((got.kind, got.side, &got.limb, &got.joint, got.start, got.end), (want.kind, want.side, &want.limb, &want.joint, want.start, want.end));
        assert!(got.position.distance(want.position) < 1e-4 && got.local.distance(want.local) < 1e-4, "{got:?} {want:?}");
    }
    // A target without arms keeps no hand contact.
    let legs_only = RigProfile { limbs: profile.limbs.iter().filter(|l| l.kind != LimbKind::Arm).cloned().collect(), ..profile.clone() };
    transfer_contacts(&source.meta.contacts, &s, &legs_only, &mut clip);
    assert_eq!(clip.meta.contacts_of(ContactKind::Hand).count(), 0);
}
