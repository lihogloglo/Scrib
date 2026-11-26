//! Cook-time mirror tests (`mirror.rs`, [`ClipRules::mirror`]): the
//! fixture biped's left-side take played as the right-side one.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use glam::{Affine3A, Quat, Vec3};
use serde::Serialize;
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};

use crate::fixtures::{biped, sampled};
use crate::mirror::{Axis, mirror_side};
use crate::steps::{AnimBake, AnimImporters, AnimRules, ClipSpec};
use crate::*;

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn flip(v: Vec3) -> Vec3 {
    Vec3::new(-v.x, v.y, v.z)
}

fn joint(name: &str) -> usize {
    biped().find(name).unwrap()
}

fn profile() -> RigProfile {
    detect_profile(&biped(), &RigHints::default()).unwrap()
}

/// A one-second take: the left thigh swings out so the left foot steps
/// to +x, the right arm lifts forward, the spine twists.
fn left_step() -> SourceClip {
    let rot = |joint: &str, f: fn(f32) -> Quat| SourceTrack { joint: joint.into(), rotation: Some(sampled(1.0, f)), ..SourceTrack::default() };
    SourceClip {
        name: "seat.enter.left".into(),
        duration: 1.0,
        tracks: vec![
            rot("thigh_l", |t| Quat::from_rotation_z(0.35 * smooth(t)) * Quat::from_rotation_x(-0.2 * smooth(t))),
            rot("upperarm_r", |t| Quat::from_rotation_y(0.5 * t) * Quat::from_rotation_x(-0.4 * t)),
            rot("spine", |t| Quat::from_rotation_y(0.3 * smooth(t))),
        ],
        curves: Vec::new(),
        events: vec![Event::point(0.5, "grab.hand_r")],
        markers: Vec::new(),
    }
}

fn cooked(source: &SourceClip, rules: &ClipRules) -> Clip {
    cook_clip_report(&biped(), Some(&profile()), source, rules, &CookSettings::default()).unwrap().0
}

fn model(frames: &FrameClip, f: usize) -> ModelPose {
    let s = biped();
    let j = s.len();
    let mut mp = ModelPose::new(j);
    local_to_model(&s, &LocalPose::from_trs(&frames.frames[f * j..(f + 1) * j]), &mut mp, &Affine3A::IDENTITY);
    mp
}

fn mirrored(frames: &FrameClip) -> FrameClip {
    let mut out = frames.clone();
    mirror_frame_clip(&biped(), &profile(), &MirrorRules::default(), &mut out).unwrap();
    out
}

/// The identity retargeter of the fixture biped onto itself (every limb mapped).
fn identity() -> Retargeter {
    let c = |name: &str, joints: &[&str]| RetargetChain { name: name.into(), joints: joints.iter().map(|&j| j.to_owned()).collect(), ik: false, solver: None };
    let rig = RetargetRig {
        chains: vec![
            c("hips", &["hips"]),
            c("spine", &["spine", "chest"]),
            c("head", &["head"]),
            c("leg_l", &["thigh_l", "shin_l", "foot_l"]),
            c("leg_r", &["thigh_r", "shin_r", "foot_r"]),
            c("arm_l", &["upperarm_l", "forearm_l", "hand_l"]),
            c("arm_r", &["upperarm_r", "forearm_r", "hand_r"]),
        ],
        retarget_pose: Vec::new(),
        hips: Some("hips".into()),
    };
    let s = biped();
    Retargeter::new(&s, &rig, &s, &rig).unwrap()
}

fn mirror_rules() -> ClipRules {
    ClipRules { mirror: Some(MirrorRules::default()), ..ClipRules::default() }
}

#[test]
fn the_left_step_becomes_the_right_step() {
    let s = biped();
    let table = MirrorTable::from_pairs(&s, &profile().mirror, Axis::X);
    assert_eq!(table.pair(joint("foot_l")), joint("foot_r"));
    assert_eq!(table.pair(joint("hand_r")), joint("hand_l"));
    assert_eq!(table.pair(joint("spine")), joint("spine"));
    let before = FrameClip::from_clip(&cooked(&left_step(), &ClipRules::default()));
    let after = mirrored(&before);
    let n = before.frame_count;
    assert!(n > 2);
    for f in 0..n {
        let (a, b) = (model(&after, f), model(&before, f));
        for jn in 0..s.len() {
            let p = table.pair(jn);
            let d = a.translation(jn).distance(flip(b.translation(p)));
            assert!(d < 1e-4, "{} at frame {f}: {d} m off the reflected {}", s.name(jn), s.name(p));
            // The fixture's rest rotations are identity: the rotation reflects as `S R S`.
            let q = b.rotation(p);
            let reflected = Quat::from_xyzw(q.x, -q.y, -q.z, q.w);
            assert!(angle_between(a.rotation(jn), reflected) < 1e-4, "{} at frame {f}", s.name(jn));
        }
    }
    // The right foot steps to -x by as much as the left one stepped to +x.
    let (fl, fr) = (joint("foot_l"), joint("foot_r"));
    let step = model(&before, n - 1).translation(fl).x - model(&before, 0).translation(fl).x;
    let mirrored_step = model(&after, n - 1).translation(fr).x - model(&after, 0).translation(fr).x;
    assert!(step > 0.05, "the take steps out: {step}");
    assert!((mirrored_step + step).abs() < 1e-4, "{mirrored_step} vs {step}");
    // Side names follow: the event and the contact curves and markers.
    assert_eq!(after.events[0].name, "grab.hand_l");
    assert!(!before.curves.is_empty(), "the cook measured contacts");
    let swap = |name: &str| name.replace("_l", "_x").replace("_r", "_l").replace("_x", "_r");
    for (a, b) in after.curves.iter().zip(&before.curves) {
        assert_eq!((a.name.as_str(), &a.values), (swap(&b.name).as_str(), &b.values));
    }
    for (a, b) in after.markers.iter().zip(&before.markers) {
        assert_eq!(a.name, swap(&b.name));
    }
    assert_eq!(mirror_side(Axis::X, Side::Left), Side::Right);
    assert_eq!(mirror_side(Axis::Z, Side::Left), Side::Left);
}

/// A take on the rest pose whose root drifts +x and turns +30 degrees.
fn turning_take() -> FrameClip {
    let s = biped();
    let n = 31;
    let frames: Vec<[f32; 4]> =
        (0..n).map(|f| f as f32 / (n - 1) as f32).map(|t| [0.4 * t, 0.0, 1.0 * t, 30f32.to_radians() * t]).collect();
    FrameClip {
        name: "turn".into(),
        rate: 30.0,
        duration: 1.0,
        frame_count: n,
        frames: (0..n).flat_map(|_| s.rest().to_vec()).collect(),
        root_motion: Some(RootMotionTrack { frames, attitude: Vec::new() }),
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
        loop_range: None,
        meta: ClipMeta { average_velocity: Vec3::new(0.4, 0.0, 1.0), turn_rate: 30f32.to_radians(), ..ClipMeta::default() },
    }
}

#[test]
fn the_root_track_turns_and_drifts_the_other_way() {
    let after = mirrored(&turning_take());
    let last = *after.root_motion.as_ref().unwrap().frames.last().unwrap();
    assert!((last[0] + 0.4).abs() < 1e-6 && (last[2] - 1.0).abs() < 1e-6, "{last:?}");
    assert!((last[3] + 30f32.to_radians()).abs() < 1e-6, "{last:?}");
    assert_eq!(after.meta.average_velocity, Vec3::new(-0.4, 0.0, 1.0));
    assert_eq!(after.meta.turn_rate, -30f32.to_radians());

    // The same on the target-rig path: cooked, retargeted with the rule.
    let settings = CookSettings::default();
    let source = cook_frames(&biped(), &turning_take(), &settings).unwrap();
    let (out, report) = identity().clip_post(&source, None, Some(&profile()), &mirror_rules(), &settings).unwrap();
    assert_eq!(report.mirror, Some(Ok(())));
    let (at, yaw) = out.root_at(out.duration);
    assert!(at.distance(Vec3::new(-0.4, 0.0, 1.0)) < 1e-3 && (yaw + 30f32.to_radians()).abs() < 1e-3, "{at} {yaw}");
}

#[test]
fn mirroring_twice_gives_the_take_back() {
    for take in [FrameClip::from_clip(&cooked(&left_step(), &ClipRules::default())), turning_take()] {
        let back = mirrored(&mirrored(&take));
        for (a, b) in back.frames.iter().zip(&take.frames) {
            assert!(a.translation.distance(b.translation) < 1e-5 && angle_between(a.rotation, b.rotation) < 1e-5, "{a:?} vs {b:?}");
        }
        let (ra, rb) = (back.root_motion.as_ref(), take.root_motion.as_ref());
        assert_eq!(ra.map(|r| r.frames.len()), rb.map(|r| r.frames.len()));
        for (a, b) in ra.into_iter().flat_map(|r| &r.frames).zip(rb.into_iter().flat_map(|r| &r.frames)) {
            assert!(a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5), "{a:?} vs {b:?}");
        }
        assert_eq!((&back.curves, &back.markers, &back.events, &back.meta), (&take.curves, &take.markers, &take.events, &take.meta));
    }
}

/// [`ClipRules`] as it was before `mirror` (its bake-key bytes).
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pelvis_fit: Option<PelvisFit>,
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
            pelvis_fit: r.pelvis_fit.clone(),
        }
    }
}

#[test]
fn without_the_rule_keys_and_clips_do_not_change() {
    let plain = ClipRules::default();
    let fitted = ClipRules { pelvis_fit: Some(PelvisFit::new(0.6, HubEnd::Last)), distance: Some(DistanceRules::new(DistanceCurve::Stop)), ..plain.clone() };
    for r in [&plain, &fitted] {
        assert_eq!(r.key_bytes(), postcard::to_stdvec(&ClipRulesBefore::from(r)).unwrap());
        let m = ClipRules { mirror: Some(MirrorRules::default()), ..r.clone() };
        assert_ne!(m.key_bytes(), r.key_bytes());
    }
    assert_eq!(ron::from_str::<MirrorRules>("()").unwrap(), MirrorRules { axis: Axis::X });
    let m = mirror_rules();
    assert_eq!(ron::from_str::<ClipRules>(&ron::to_string(&m).unwrap()).unwrap(), m);
    assert!(!ron::to_string(&plain).unwrap().contains("mirror"));
    // The source cook ignores the rule.
    assert_eq!(cooked(&left_step(), &mirror_rules()).to_bytes(), cooked(&left_step(), &plain).to_bytes());
    // The target path: no rule, the plain retarget; a rule without a profile, reported and not applied.
    let source = cooked(&left_step(), &plain);
    let r = identity();
    let settings = CookSettings::default();
    let reference = r.clip(&source, &settings).unwrap().to_bytes();
    let (none, report) = r.clip_post(&source, None, Some(&profile()), &plain, &settings).unwrap();
    assert_eq!(report, PostReport::default());
    assert_eq!(none.to_bytes(), reference);
    let (refused, report) = r.clip_post(&source, None, None, &mirror_rules(), &settings).unwrap();
    assert_eq!(report.mirror, Some(Err(MirrorError::NoProfile)));
    assert_eq!(refused.to_bytes(), reference);
}

/// A one-second sit: the hips drop from 0.9 m to 0.5 m.
fn sit_down() -> SourceClip {
    SourceClip {
        name: "seat.enter.left".into(),
        duration: 1.0,
        tracks: vec![
            SourceTrack { joint: "hips".into(), translation: Some(sampled(1.0, |t| Vec3::new(0.0, 0.9 - 0.4 * smooth(t), 0.0))), ..SourceTrack::default() },
            SourceTrack { joint: "thigh_l".into(), rotation: Some(sampled(1.0, |t| Quat::from_rotation_z(0.3 * smooth(t)))), ..SourceTrack::default() },
        ],
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
    }
}

#[test]
fn the_mirror_keeps_the_fitted_seat_height() {
    let settings = CookSettings::default();
    let source = cooked(&sit_down(), &ClipRules::default());
    let fit = ClipRules { pelvis_fit: Some(PelvisFit::new(0.6, HubEnd::Last)), ..ClipRules::default() };
    let both = ClipRules { mirror: Some(MirrorRules::default()), ..fit.clone() };
    let r = identity();
    let (fitted, fr) = r.clip_post(&source, None, Some(&profile()), &fit, &settings).unwrap();
    let (mirrored, mr) = r.clip_post(&source, None, Some(&profile()), &both, &settings).unwrap();
    assert!(matches!(fr.pelvis_fit, Some(Ok(_))) && fr.mirror.is_none(), "{fr:?}");
    assert_eq!((mr.pelvis_fit, mr.mirror), (fr.pelvis_fit, Some(Ok(()))));
    let s = biped();
    let hips = joint("hips");
    let tolerance = 1e-4 + fitted.meta.max_error + mirrored.meta.max_error;
    let n = fitted.frame_count as usize;
    assert_eq!(mirrored.frame_count as usize, n);
    let (mut a, mut b) = (LocalPose::new(s.len()), LocalPose::new(s.len()));
    let (mut ma, mut mb) = (ModelPose::new(s.len()), ModelPose::new(s.len()));
    for f in 0..n {
        let t = fitted.duration * f as f32 / (n - 1) as f32;
        fitted.sample(t, &mut a, None);
        mirrored.sample(t, &mut b, None);
        local_to_model(&s, &a, &mut ma, &Affine3A::IDENTITY);
        local_to_model(&s, &b, &mut mb, &Affine3A::IDENTITY);
        let (ya, yb) = (ma.translation(hips).y, mb.translation(hips).y);
        assert!((ya - yb).abs() <= tolerance, "hips {yb} vs {ya} at {t}");
        // The fitted left foot is the mirrored right one.
        let (fl, fr) = (ma.translation(joint("foot_l")), mb.translation(joint("foot_r")));
        assert!(fr.distance(flip(fl)) <= 2.0 * tolerance, "{fr} vs {fl} at {t}");
    }
    let y = mb.translation(hips).y;
    assert!((y - 0.6).abs() <= tolerance, "hub pelvis at {y}");
}

// --- The bake path: `anim.cook` + `anim.retarget` -------------------------------------

/// Two-second take of the fixture biped: the left thigh swings out over the
/// first second (the left foot steps to +x), then holds; the hips stand.
fn stand_and_step() -> SourceClip {
    SourceClip {
        name: "step".into(),
        duration: 2.0,
        tracks: vec![SourceTrack {
            joint: "thigh_l".into(),
            rotation: Some(sampled(2.0, |t| Quat::from_rotation_z(0.3 * smooth(t.min(1.0))))),
            ..SourceTrack::default()
        }],
        curves: Vec::new(),
        events: Vec::new(),
        markers: Vec::new(),
    }
}

/// Imports any `.biped` file as the fixture biped with [`stand_and_step`].
struct BipedImporter;

impl AnimImporter for BipedImporter {
    fn extensions(&self) -> &[&str] {
        &["biped"]
    }
    fn version(&self) -> u32 {
        1
    }
    fn import(&self, _path: &str, _bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let s = biped();
        let joints = (0..s.len()).map(|i| SourceJoint { name: s.name(i).to_owned(), parent: s.parent(i), rest: s.rest()[i] }).collect();
        Ok(AnimSource { skeleton: Some(joints), clips: vec![stand_and_step()], root_motion: Some("root".into()) })
    }
}

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

const SEAT_M: f32 = 0.69;

/// Library-like rules: every clip retargets, `step_fit` fits the seat
/// height, `step_fit_mirror` also mirrors (both from the source take `step`).
struct SeatRules;

impl AnimRules for SeatRules {
    fn skeleton(&self, _path: &str) -> SkeletonRules {
        SkeletonRules { root_motion_joint: Some("root".into()), ..SkeletonRules::default() }
    }
    fn clip(&self, _file: &str, clip: &str, _skeleton: &str) -> ClipSpec {
        let fit = Some(PelvisFit::new(SEAT_M, HubEnd::Last));
        let rules = match clip {
            "step_fit" => ClipRules { pelvis_fit: fit, ..ClipRules::default() },
            "step_fit_mirror" => ClipRules { pelvis_fit: fit, mirror: Some(MirrorRules::default()), ..ClipRules::default() },
            _ => ClipRules::default(),
        };
        ClipSpec { source: "step".into(), rules }
    }
    fn rig(&self, _path: &str) -> Option<RetargetRig> {
        let c = |name: &str, joints: &[&str]| RetargetChain { name: name.into(), joints: joints.iter().map(|&j| j.to_owned()).collect(), ik: false, solver: None };
        Some(RetargetRig {
            chains: vec![
                c("hips", &["hips"]),
                c("spine", &["spine", "chest"]),
                c("head", &["head"]),
                c("leg_l", &["thigh_l", "shin_l", "foot_l"]),
                c("leg_r", &["thigh_r", "shin_r", "foot_r"]),
                c("arm_l", &["upperarm_l", "forearm_l", "hand_l"]),
                c("arm_r", &["upperarm_r", "forearm_r", "hand_r"]),
            ],
            retarget_pose: Vec::new(),
            hips: Some("hips".into()),
        })
    }
    fn retargeted(&self, _file: &str, _clip: &str, _skeleton: &str) -> bool {
        true
    }
}

/// `(kind, side, start, end)` of each contact, in time order.
fn contact_sides(clip: &Clip, side: impl Fn(Side) -> Side) -> Vec<(ContactKind, Side, u32, u32)> {
    let mut v: Vec<_> = clip.meta.contacts.iter().map(|c| (c.kind, side(c.side), c.start.to_bits(), c.end.to_bits())).collect();
    v.sort_by_key(|c| (c.2, c.1 == Side::Right));
    v
}

/// `ClipRules::pelvis_fit` and `mirror` through the bake: `anim.cook` cooks
/// the source without them (the same product as the plain clip's), and
/// `anim.retarget` fits the hub pelvis to the seat on the target and plays
/// the left step as the right one, its contacts on the other side.
#[test]
fn the_bake_fits_and_mirrors_on_the_target() {
    let files = Arc::new(Files::default());
    for p in ["chars/a.biped", "chars/b.biped"] {
        files.0.write().unwrap().insert(p.into(), b"biped".to_vec());
    }
    let dir = tempfile::tempdir().unwrap();
    let bake = AnimBake::new(AnimImporters::new().with(Arc::new(BipedImporter)), Arc::new(SeatRules), CookSettings::default());
    let mut baker = Baker::new(Store::open(dir.path()).unwrap(), files);
    bake.register(&mut baker);
    let (a, b) = ("chars/a.biped", "chars/b.biped");
    let cook = |clip: &str| ProductKey::new(steps::ANIM_COOK, steps::anim_subject(a, a, clip));
    let moved = |clip: &str| ProductKey::new(steps::ANIM_RETARGET, steps::retarget_subject(b, &steps::anim_subject(a, a, clip)));
    let keys: Vec<_> = ["step", "step_fit", "step_fit_mirror"].iter().flat_map(|n| [cook(n), moved(n)]).collect();
    let report = baker.bake(&keys);
    assert_eq!(report.failures().count(), 0, "{:?}", report.failures().collect::<Vec<_>>());
    let bytes = |k: &ProductKey| baker.store().get(&baker.product(k).unwrap()).unwrap();
    let get = |k: &ProductKey| Clip::from_bytes(&bytes(k)).unwrap();

    // The source cook leaves the target-rig rules out; the retarget keys them.
    let mut source = get(&cook("step_fit_mirror"));
    "step".clone_into(&mut source.name);
    assert_eq!(source.to_bytes(), bytes(&cook("step")));
    assert_ne!(baker.product(&moved("step_fit")).unwrap(), baker.product(&moved("step")).unwrap());

    let (plain, fitted, mirrored) = (get(&moved("step")), get(&moved("step_fit")), get(&moved("step_fit_mirror")));
    let s = biped();
    let at = |clip: &Clip, t: f32| {
        let (mut pose, mut model) = (LocalPose::new(s.len()), ModelPose::new(s.len()));
        clip.sample(t, &mut pose, None);
        local_to_model(&s, &pose, &mut model, &Affine3A::IDENTITY);
        model
    };
    let (end, hips) = (fitted.duration, joint("hips"));
    assert!((at(&plain, end).translation(hips).y - 1.0).abs() < 1e-3, "the plain clip stands");
    for clip in [&fitted, &mirrored] {
        let y = at(clip, end).translation(hips).y;
        assert!((y - SEAT_M).abs() < 1e-3, "hub pelvis at {y}, want {SEAT_M}");
    }
    // The left foot stepped to +x; mirrored, the right foot steps to -x.
    let (fl, fr) = (joint("foot_l"), joint("foot_r"));
    let step = at(&fitted, end).translation(fl).x - at(&fitted, 0.0).translation(fl).x;
    let mirrored_step = at(&mirrored, end).translation(fr).x - at(&mirrored, 0.0).translation(fr).x;
    assert!(step > 0.05, "the take steps out: {step}");
    assert!((mirrored_step + step).abs() < 1e-3, "{mirrored_step} vs {step}");
    // The contacts: the same kinds and times, on the other side.
    assert!(!fitted.meta.contacts.is_empty(), "the source cook measured contacts");
    assert_eq!(contact_sides(&mirrored, |s| s), contact_sides(&fitted, |s| mirror_side(Axis::X, s)));
}
