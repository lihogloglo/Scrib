//! Distance-matching curves (`distance.rs`; animation.md open question 5).
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use glam::{Quat, Vec3};

use crate::*;

/// A root track sampled `n` frames `dt` apart from a ground position `p(t)`
/// (`[x, y, z]`); yaw stays 0.
fn track(n: usize, dt: f32, p: impl Fn(f32) -> [f32; 3]) -> RootMotionTrack {
    let frames = (0..n)
        .map(|f| {
            let [x, y, z] = p(f as f32 * dt);
            [x, y, z, 0.0]
        })
        .collect();
    RootMotionTrack { frames, attitude: Vec::new() }
}

fn exact(curve: DistanceCurve) -> DistanceRules {
    DistanceRules { curve, rest_speed: 0.0 }
}

fn assert_close(got: &[f32], want: impl Fn(usize) -> f32, eps: f32) {
    for (f, &g) in got.iter().enumerate() {
        let w = want(f);
        assert!((g - w).abs() <= eps, "frame {f}: {g} vs {w}");
    }
}

#[test]
fn start_at_constant_speed_is_speed_times_time() {
    // 1.5 m/s along (0.6, 0.8) on the ground; the vertical bob adds nothing.
    let dt = 1.0 / 30.0;
    let t = track(31, dt, |t| [0.9 * t + 0.2, 0.05 * (9.0 * t).sin(), 1.2 * t - 0.1]);
    let d = distance_curve(&t, dt, &DistanceRules::new(DistanceCurve::Start));
    assert_eq!(d.len(), 31);
    assert_eq!(d[0], 0.0);
    assert_close(&d, |f| 1.5 * f as f32 * dt, 1e-5);
}

#[test]
fn stop_at_constant_deceleration_ends_at_zero() {
    // From 2 m/s to rest over 1 s, then 10 frames standing: the curve is
    // minus the distance left, -a(T - t)^2 / 2, and 0 once at rest.
    let (v0, stop, dt) = (2.0_f32, 1.0_f32, 1.0 / 30.0);
    let a = v0 / stop;
    let z = move |t: f32| {
        let t = t.min(stop);
        v0 * t - 0.5 * a * t * t
    };
    let t = track(41, dt, |t| [0.0, 0.0, z(t)]);
    let rules = DistanceRules { curve: DistanceCurve::Stop, rest_speed: 0.01 };
    let d = distance_curve(&t, dt, &rules);
    assert_close(
        &d,
        |f| {
            let left = (stop - f as f32 * dt).max(0.0);
            -0.5 * a * left * left
        },
        1e-5,
    );
    assert!((d[0] + 0.5 * a * stop * stop).abs() < 1e-5, "starts a(T^2)/2 = 1 m out: {}", d[0]);
    assert!(d[30..].iter().all(|&v| v == 0.0), "0 from the stop on: {:?}", &d[30..]);
    assert!(d.windows(2).all(|w| w[1] >= w[0]));
}

#[test]
fn stop_drift_below_rest_speed_adds_nothing() {
    // A 1 m/s run stops at frame 20, then the root sways 2 mm per frame
    // (6 cm/s at 30 Hz, under a 0.1 m/s rest speed): the curve stays 0.
    let dt = 1.0 / 30.0;
    let t = track(40, dt, |t| {
        let f = (t / dt).round() as usize;
        let sway = if f > 20 && f % 2 == 1 { 0.002 } else { 0.0 };
        [sway, 0.0, t.min(20.0 * dt)]
    });
    let d = distance_curve(&t, dt, &DistanceRules { curve: DistanceCurve::Stop, rest_speed: 0.1 });
    assert_close(&d, |f| -(20.0 - f.min(20) as f32) * dt, 1e-5);
    assert!(d[20..].iter().all(|&v| v == 0.0));
}

#[test]
fn pivot_is_negative_then_zero_at_the_reversal_then_positive() {
    // z = c - k (t - tp)^2: forward, reversing at tp (frame 15), back.
    let (k, dt) = (3.0_f32, 1.0 / 30.0);
    let tp = 15.0 * dt;
    let t = track(31, dt, |t| [0.4, 0.0, 1.0 - k * (t - tp) * (t - tp)]);
    let d = distance_curve(&t, dt, &exact(DistanceCurve::Pivot));
    assert_eq!(d[15], 0.0);
    assert_close(
        &d,
        |f| {
            let u = f as f32 * dt - tp;
            u.signum() * k * u * u
        },
        1e-5,
    );
    assert!(d[..15].iter().all(|&v| v < 0.0) && d[16..].iter().all(|&v| v > 0.0));
}

#[test]
fn curved_path_measures_path_length_not_chord() {
    // A quarter circle of radius 2 m at 1 rad/s, sampled at 60 Hz: each step
    // is the chord 2r sin(w dt / 2), and the total beats the end-to-end chord.
    let (r, w, dt) = (2.0_f32, 1.0_f32, 1.0 / 60.0);
    let n = 95;
    let t = track(n, dt, |t| [r * (w * t).sin(), 0.0, r * (1.0 - (w * t).cos())]);
    let d = distance_curve(&t, dt, &exact(DistanceCurve::Start));
    let step = 2.0 * r * (0.5 * w * dt).sin();
    assert_close(&d, |f| f as f32 * step, 1e-4);
    let total = (n - 1) as f32 * dt;
    let chord = 2.0 * r * (0.5 * w * total).sin();
    assert!(d[n - 1] > chord + 0.05, "path {} vs chord {chord}", d[n - 1]);
    assert!((d[n - 1] - r * w * total).abs() < 1e-3, "≈ arc length r·w·T");
}

#[test]
fn short_tracks() {
    let one = track(1, 0.1, |_| [0.0; 3]);
    for c in [DistanceCurve::Start, DistanceCurve::Stop, DistanceCurve::Pivot] {
        assert!(distance_curve(&RootMotionTrack { frames: Vec::new(), attitude: Vec::new() }, 0.1, &exact(c)).is_empty());
        assert_eq!(distance_curve(&one, 0.1, &exact(c)), vec![0.0]);
    }
    let two = track(2, 0.5, |t| [0.0, 0.0, t]);
    assert_eq!(distance_curve(&two, 0.5, &exact(DistanceCurve::Pivot)), vec![0.0, 0.5]);
    assert_eq!(distance_curve(&two, 0.5, &exact(DistanceCurve::Stop)), vec![-0.5, 0.0]);
}

// --- At the cook ---------------------------------------------------------------

/// A clip whose root travels +Z at 1.2 m/s for 1 s, hips above it.
fn travelling() -> SourceClip {
    let lin = |f: fn(f32) -> Vec3| {
        let times: Vec<f32> = (0..=10).map(|i| i as f32 / 10.0).collect();
        let values = times.iter().map(|&t| f(t)).collect();
        Curve::linear(times, values)
    };
    SourceClip {
        name: "start".into(),
        duration: 1.0,
        tracks: vec![SourceTrack { joint: "root".into(), translation: Some(lin(|t| Vec3::new(0.0, 0.0, 1.2 * t))), ..SourceTrack::default() }],
        ..SourceClip::default()
    }
}

fn rules(distance: Option<DistanceRules>) -> ClipRules {
    ClipRules { contacts: ContactRules { enabled: false, ..ContactRules::default() }, distance, ..ClipRules::default() }
}

#[test]
fn cook_adds_the_distance_curve_and_nothing_else() {
    let s = fixtures::biped();
    let src = travelling();
    let settings = CookSettings::default();
    let plain = cook_clip(&s, &src, &rules(None), &settings).unwrap();
    assert!(plain.find_curve(DISTANCE_CURVE).is_none());
    let mut with = cook_clip(&s, &src, &rules(Some(DistanceRules::new(DistanceCurve::Start))), &settings).unwrap();
    let i = with.find_curve(DISTANCE_CURVE).expect("the rule was set");
    let values = &with.curves[i].values;
    assert_eq!(values.len(), with.frame_count as usize);
    assert_eq!(values[0], 0.0);
    assert!((values[values.len() - 1] - 1.2).abs() < 1e-4, "{}", values[values.len() - 1]);
    assert!((with.curve(i, 0.5) - 0.6).abs() < 1e-3);
    // Without the curve the clip is the plain cook, byte for byte.
    with.curves.remove(i);
    assert_eq!(with.to_bytes(), plain.to_bytes());
}

#[test]
fn cook_skips_the_curve_without_root_motion() {
    let s = fixtures::biped();
    let mut r = rules(Some(DistanceRules::new(DistanceCurve::Stop)));
    r.root_motion.extract = false;
    let clip = cook_clip(&s, &travelling(), &r, &CookSettings::default()).unwrap();
    assert!(clip.root_motion.is_none());
    assert!(clip.find_curve(DISTANCE_CURVE).is_none());
}

#[test]
fn rules_key_and_ron_unchanged_without_the_rule() {
    // The bake key of rules without `distance` is the field-by-field
    // encoding of the fields before it (postcard writes structs as their
    // fields in order), so existing clips keep their cache keys.
    let r = rules(None);
    let before = postcard::to_stdvec(&(r.range, r.looping, &r.root_motion, r.additive, &r.events, &r.markers, &r.contacts, &r.loop_cut)).unwrap();
    assert_eq!(r.key_bytes(), before);
    assert_ne!(rules(Some(DistanceRules::new(DistanceCurve::Start))).key_bytes(), before);
    assert_ne!(
        rules(Some(DistanceRules::new(DistanceCurve::Start))).key_bytes(),
        rules(Some(DistanceRules::new(DistanceCurve::Stop))).key_bytes()
    );
    // RON without the field loads as None; with it, the rest speed defaults.
    let text = ron::to_string(&r).unwrap();
    assert!(!text.contains("distance"), "{text}");
    assert_eq!(ron::from_str::<ClipRules>(&text).unwrap(), r);
    let set = format!("{},distance: Some((curve: Pivot)))", text.strip_suffix(')').unwrap().trim_end_matches(','));
    let parsed: ClipRules = ron::from_str(&set).unwrap();
    assert_eq!(parsed.distance, Some(DistanceRules { curve: DistanceCurve::Pivot, rest_speed: REST_SPEED_MPS.default_value() }));
}

// --- Retargeted clips (`apply_distance` after `Retargeter`) --------------------------

/// The cooked clip's [`DISTANCE_CURVE`] values.
fn curve_of(clip: &Clip) -> &[f32] {
    &clip.curves[clip.find_curve(DISTANCE_CURVE).expect("a distance curve")].values
}

/// What `apply_distance` must write: [`distance_curve`] of the clip's own root track.
fn measured(clip: &Clip, rules: DistanceRules) -> Vec<f32> {
    let dt = cook::frame_time(1, clip.frame_count as usize, clip.duration);
    distance_curve(clip.root_motion.as_ref().unwrap(), dt, &rules)
}

#[test]
fn retargeted_clip_curve_is_measured_on_the_target_root_track() {
    use crate::tests::{biped, biped_rig, reframed};
    // A target 1.25 times the source (longer legs): its root travels 1.25 times as far.
    let s = biped();
    let rig_s = biped_rig("", true, false);
    let t = Skeleton::new(&reframed(&s, &rig_s, "T_", 1.25, |_| Quat::IDENTITY)).unwrap().0;
    let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, false)).unwrap();
    assert!((r.scale() - 1.25).abs() < 1e-5);
    // Five frames standing, then a start accelerating at 2 m/s^2 for 1 s.
    let (n, rate) = (36, 30.0_f32);
    let dt = 1.0 / rate;
    let root = track(n, dt, |t| [0.0, 0.0, (t - 5.0 * dt).max(0.0).powi(2)]);
    let take = FrameClip {
        name: "start".into(),
        rate,
        duration: (n - 1) as f32 * dt,
        frame_count: n,
        frames: (0..n).flat_map(|_| s.rest().to_vec()).collect(),
        root_motion: Some(root),
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
        loop_range: None,
        meta: ClipMeta::default(),
    };
    let rules = DistanceRules::new(DistanceCurve::Start);
    let settings = CookSettings::default();
    let mut source = cook_frames(&s, &take, &settings).unwrap();
    apply_distance(&mut source, &rules);
    assert_eq!(curve_of(&source), measured(&source, rules));

    let mut on_t = r.clip(&source, &settings).unwrap();
    // The retarget alone carries the source's curve over.
    assert_eq!(curve_of(&on_t), curve_of(&source));
    apply_distance(&mut on_t, &rules);
    let (got, src) = (curve_of(&on_t), curve_of(&source));
    assert_eq!(got, measured(&on_t, rules));
    assert_eq!(on_t.curves.iter().filter(|c| c.name == DISTANCE_CURVE).count(), 1, "replaced, not added");
    let (last, src_last) = (got[got.len() - 1], src[src.len() - 1]);
    // Less the first moving step, under the rest speed on both bodies.
    assert!((src_last - (1.0 - dt * dt)).abs() < 1e-3, "source travels 1 m: {src_last}");
    assert!((last - 1.25 * (1.0 - dt * dt)).abs() < 1e-3, "target travels 1.25 m: {last}");
    assert!(got[..6].iter().all(|&d| d == 0.0), "standing frames stay flat: {:?}", &got[..6]);
}

#[test]
fn root_generated_on_the_target_gets_the_curve() {
    use crate::tests::{biped, biped_rig, reframed, take_source};
    // The motion is in the source pose (`retarget_root`: cooked without
    // extraction, so no root track and no curve); the root is generated
    // from the target pelvis, 1.25 times the source: 0.3 m a frame forward
    // becomes 0.375 m, over four steps 1.5 m.
    let (s, rig_s, take) = take_source(5, |f| Vec3::new(3.0 + 0.3 * f as f32, 1.0 + 0.05 * f as f32, 2.0), |_| std::f32::consts::FRAC_PI_2);
    let t = Skeleton::new(&reframed(&biped(), &biped_rig("", true, false), "T_", 1.25, |_| Quat::IDENTITY)).unwrap().0;
    let r = Retargeter::new(&s, &rig_s, &t, &biped_rig("T_", false, false)).unwrap();
    let rules = DistanceRules::new(DistanceCurve::Start);
    let settings = CookSettings::default();
    let mut source = cook_frames(&s, &take, &settings).unwrap();
    apply_distance(&mut source, &rules);
    assert!(source.root_motion.is_none() && source.find_curve(DISTANCE_CURVE).is_none());

    let root = RootMotionRules { vertical: true, ..RootMotionRules::default() };
    let mut on_t = r.clip_with_root(&source, &root, &settings).unwrap();
    assert!(on_t.find_curve(DISTANCE_CURVE).is_none(), "the retarget alone has no curve to carry");
    apply_distance(&mut on_t, &rules);
    let got = curve_of(&on_t);
    assert_eq!(got, measured(&on_t, rules));
    assert!((got[got.len() - 1] - 1.5).abs() < 1e-3, "{got:?}");
}
