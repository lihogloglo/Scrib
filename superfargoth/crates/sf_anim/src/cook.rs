//! `anim.cook`: source clip → cooked [`Clip`].
//!
//! 1. Bind the source tracks to the skeleton by joint name.
//! 2. For each candidate rate, coarsest first: resample uniformly (the last
//!    frame lands on the end), extract root motion, convert to additive if
//!    asked, and measure the resampling error (unquantized frames against
//!    the source at every midpoint). A rate whose resampling error alone
//!    exceeds 1.5x the bound can't pass and is skipped; otherwise quantize
//!    every channel against its error budget and measure the virtual-vertex
//!    error of the cooked clip against the source at every frame and every
//!    midpoint (stopping at the first sample over the bound). The first
//!    rate within the bound wins; if none is, the rate with the lowest
//!    resampling error is cooked and its full error recorded.
//! 3. Resample curves, shift events and markers, detect foot contacts
//!    (contact curves and sync markers), and compute speed meta.
//!
//! **Error budget.** Virtual vertices sit `shell_distance` from each joint
//! along its three axes (ACL's metric). Half the bound goes to
//! quantization, split over channels so that the errors along any
//! root-to-leaf chain sum to at most that half: a channel's budget is the
//! half-bound divided by the number of moving channels on the longest
//! chain through it, and its tolerance is that budget over its lever arm
//! (shell distance plus the reach of its subtree). The other half covers
//! resampling, which the measurement checks directly.

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::clip::{RAW, RotTrack, VecChannel, VecTrack, rebuild};
use crate::source::BoundSource;
use crate::{
    Clip, CurveTrack, Event, LocalPose, ModelPose, RigProfile, RootMotionMode, RootMotionTrack, Skeleton, SourceClip, SyncMarker, Trs,
    apply_additive, local_to_model, math, tag,
};

/// Quality settings of `anim.cook` (bake parameters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookSettings {
    /// Largest allowed virtual-vertex error, meters.
    pub error_bound: f32,
    /// Distance of the virtual vertices from their joint, meters.
    pub shell_distance: f32,
    /// Lowest sample rate, Hz.
    pub min_rate: f32,
    /// Highest sample rate tried, Hz. A clip that misses the bound even
    /// here is cooked at this rate and its error recorded in the meta.
    pub max_rate: f32,
}

impl Default for CookSettings {
    fn default() -> Self {
        Self { error_bound: 1e-4, shell_distance: 1.0, min_rate: 30.0, max_rate: 120.0 }
    }
}

impl CookSettings {
    /// Stable bytes for the bake key.
    pub fn key_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("in-memory serialization")
    }
}

/// How a clip loops by default (times in source seconds).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Looping {
    #[default]
    Once,
    /// The whole clip loops.
    Whole,
    /// Plays into `[start, end]` and loops there.
    Range(f32, f32),
}

/// Root-motion extraction rules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // Independent switches, as in the spec.
pub struct RootMotionRules {
    pub mode: RootMotionMode,
    /// Extract when the skeleton names a root-motion joint.
    pub extract: bool,
    /// Ground-plane (X, Z) translation.
    pub horizontal: bool,
    /// Vertical (Y) translation: climbs, jumps that change level.
    pub vertical: bool,
    /// Heading about +Y.
    pub yaw: bool,
    /// Pitch and roll in the heading frame (flyers, swimmers): the root's
    /// attitude leaves the pose and becomes an attitude track.
    pub attitude: bool,
    /// In-place clips only (`extract` off): re-base the clip so the
    /// root-motion joint's ground position and heading at the **last**
    /// frame are the model origin and +Z (heading against its rest
    /// rotation). For in-place parts cut from a longer take that end standing
    /// away from the take's origin (a get-up tail): the character then ends
    /// over its frame, so a capsule placed at the frame needs no slide.
    /// Root motion from the target pelvis (`Retargeter::frames_with_root`)
    /// anchors the target hips the same way.
    #[serde(default)]
    pub anchor_end: bool,
}

impl Default for RootMotionRules {
    fn default() -> Self {
        Self { mode: RootMotionMode::Match, extract: true, horizontal: true, vertical: false, yaw: true, attitude: false, anchor_end: false }
    }
}

/// What an additive clip is a delta from.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AdditiveBase {
    FirstFrame,
    Rest,
    /// The clip's own pose at this source time.
    Time(f32),
}

/// Foot-contact detection. Thresholds are fractions of each foot's leg
/// length (rig-profile.md "Measures"), so a 0.2 m scrib and a 6 m giant
/// use the same rules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactRules {
    pub enabled: bool,
    /// Joints to test. Empty: the support-limb effectors of the rig profile
    /// given to [`cook_clip_for`], else every joint tagged `foot`, `claw`,
    /// `hoof` or `contact` ([`tag::CONTACTS`]).
    pub feet: Vec<String>,
    /// Height above the joint's lowest point in the clip below which a
    /// foot may plant, in leg lengths.
    pub height: f32,
    /// Speed over the ground below which a low foot plants, leg lengths
    /// per second. In-place clips measure it against their estimated
    /// ground velocity (the planted feet's slide).
    pub speed: f32,
    /// Hysteresis: a planted foot stays planted until it rises above
    /// `height × release` or moves faster than `speed × release`.
    pub release: f32,
    /// Heel-off, leg lengths per second: a stance ends where its foot's
    /// last rise faster than this begins (in the stance's second half).
    pub lift: f32,
    /// Debounce, seconds: a contact gap this short at a loop's seam, between
    /// two planted runs, is closed (the seam's velocity spike doesn't release
    /// the foot mid stance).
    #[serde(default = "default_contact_gap")]
    pub gap: f32,
    /// Contact-consistent root motion (footskate cleanup), seconds: a
    /// travelling loop's planar root track is re-derived so its planted
    /// feet move against the body at one constant speed (what speed
    /// matching plays it by), each stance's weight ramping in and out over
    /// this long. 0 keeps the extracted track.
    #[serde(default = "default_root_blend")]
    pub root_blend: f32,
}

impl Default for ContactRules {
    /// 6% and 35% of the leg to plant, twice that to lift: about 5 cm and
    /// 0.3 m/s (11 cm, 0.63 m/s) on a 0.9 m human leg; heel-off from 5 cm/s up.
    fn default() -> Self {
        Self {
            enabled: true,
            feet: Vec::new(),
            height: 0.06,
            speed: 0.35,
            release: 2.0,
            lift: 0.06,
            gap: default_contact_gap(),
            root_blend: default_root_blend(),
        }
    }
}

fn default_contact_gap() -> f32 {
    0.05
}

fn default_root_blend() -> f32 {
    0.1
}

/// Leg length assumed for a contact joint with no leg above it (a bare
/// foot joint): the defaults then mean about 5 cm and 0.3 m/s.
const FALLBACK_LEG_LENGTH: f32 = 0.85;

/// Rest-pose leg length of a contact joint without a profile: the chain
/// from it up two joints (foot, shin, thigh); [`FALLBACK_LEG_LENGTH`] when
/// that is under 5 cm.
fn estimated_leg_length(skeleton: &Skeleton, foot: usize) -> f32 {
    let model = skeleton.rest_model();
    let p = |j: usize| Vec3::from(model[j].translation);
    let mut len = 0.0;
    let mut j = foot;
    for _ in 0..2 {
        let Some(parent) = skeleton.parent(j) else {
            break;
        };
        len += p(j).distance(p(parent));
        j = parent;
    }
    if len < 0.05 { FALLBACK_LEG_LENGTH } else { len }
}

/// The joints [`ContactRules`] tests, with their leg lengths.
fn contact_feet(skeleton: &Skeleton, profile: Option<&RigProfile>, rules: &ContactRules) -> Vec<(usize, f32)> {
    let length_of = |j: usize| {
        profile
            .and_then(|p| {
                p.limbs.iter().find(|l| usize::from(l.effector) == j || l.foot.as_ref().and_then(|f| f.toe).is_some_and(|t| usize::from(t) == j))
            })
            .map_or_else(|| estimated_leg_length(skeleton, j), |l| l.length.max(0.05))
    };
    if !rules.feet.is_empty() {
        return rules.feet.iter().filter_map(|f| skeleton.find(f)).map(|j| (j, length_of(j))).collect();
    }
    if let Some(p) = profile {
        let feet: Vec<(usize, f32)> = p.support_limbs().map(|l| (usize::from(l.effector), l.length.max(0.05))).collect();
        if !feet.is_empty() {
            return feet;
        }
    }
    let mut feet: Vec<usize> = tag::CONTACTS.iter().flat_map(|t| skeleton.joints_with_tag(t)).collect();
    feet.sort_unstable();
    feet.dedup();
    feet.into_iter().map(|j| (j, length_of(j))).collect()
}

/// Adapter rules for one clip. All times are in the source's seconds.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClipRules {
    /// The part of the source this clip is (splitting one long take).
    pub range: Option<(f32, f32)>,
    pub looping: Looping,
    pub root_motion: RootMotionRules,
    pub additive: Option<AdditiveBase>,
    /// Added to the source's own events.
    pub events: Vec<Event>,
    pub markers: Vec<SyncMarker>,
    pub contacts: ContactRules,
}

impl ClipRules {
    pub fn key_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("in-memory serialization")
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CookError {
    #[error("bad clip range {0}..{1}")]
    BadRange(f32, f32),
    #[error("curve of {0:?} has inconsistent keys")]
    BadCurve(String),
    #[error("frame data size mismatch")]
    BadFrames,
}

/// A clip as uniformly sampled local poses: the cook's intermediate, and
/// the input of [`cook_frames`] (retargeting produces these).
#[derive(Clone, Debug, PartialEq)]
pub struct FrameClip {
    pub name: String,
    /// Samples per second; `frames` holds `frame_count × joints` poses.
    pub rate: f32,
    pub duration: f32,
    pub frame_count: usize,
    pub frames: Vec<Trs>,
    pub root_motion: Option<RootMotionTrack>,
    pub curves: Vec<CurveTrack>,
    pub events: Vec<Event>,
    pub markers: Vec<SyncMarker>,
    pub loop_range: Option<(f32, f32)>,
    pub meta: crate::ClipMeta,
}

impl FrameClip {
    /// Decodes a cooked clip back into frames (for retargeting).
    pub fn from_clip(clip: &Clip) -> Self {
        let j = clip.joint_count();
        let n = clip.frame_count as usize;
        let mut pose = LocalPose::new(j);
        let mut frames = Vec::with_capacity(n * j);
        for f in 0..n {
            let t = frame_time(f, n, clip.duration);
            clip.sample(t, &mut pose, None);
            frames.extend(pose.to_vec());
        }
        Self {
            name: clip.name.clone(),
            rate: clip.rate,
            duration: clip.duration,
            frame_count: n,
            frames,
            root_motion: clip.root_motion.clone(),
            curves: clip.curves.clone(),
            events: clip.events.clone(),
            markers: clip.markers.clone(),
            loop_range: clip.loop_range,
            meta: clip.meta.clone(),
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn frame_time(f: usize, n: usize, duration: f32) -> f32 {
    if n < 2 { 0.0 } else { duration * f as f32 / (n - 1) as f32 }
}

/// Cooks a source clip for `skeleton`.
pub fn cook_clip(skeleton: &Skeleton, source: &SourceClip, rules: &ClipRules, settings: &CookSettings) -> Result<Clip, CookError> {
    cook_clip_for(skeleton, None, source, rules, settings)
}

/// [`cook_clip`] with the skeleton's rig profile: contacts default to its
/// support-limb effectors, thresholds scaled by each limb's length.
pub fn cook_clip_for(
    skeleton: &Skeleton,
    profile: Option<&RigProfile>,
    source: &SourceClip,
    rules: &ClipRules,
    settings: &CookSettings,
) -> Result<Clip, CookError> {
    for t in &source.tracks {
        let ok = t.translation.as_ref().is_none_or(crate::Curve::is_valid)
            && t.rotation.as_ref().is_none_or(crate::Curve::is_valid)
            && t.scale.as_ref().is_none_or(crate::Curve::is_valid);
        if !ok {
            return Err(CookError::BadCurve(t.joint.clone()));
        }
    }
    if let Some(c) = source.curves.iter().find(|c| !c.curve.is_valid()) {
        return Err(CookError::BadCurve(c.name.clone()));
    }
    let bound = BoundSource::new(source, skeleton);
    let (start, end) = rules.range.unwrap_or((0.0, source.duration));
    if !(start.is_finite() && end.is_finite() && start >= 0.0 && end >= start) {
        return Err(CookError::BadRange(start, end));
    }
    let duration = end - start;
    let rm_joint = skeleton.root_motion_joint().filter(|_| {
        let r = &rules.root_motion;
        r.extract && (r.horizontal || r.vertical || r.yaw || r.attitude) && rules.additive.is_none()
    });

    let rates = candidate_rates(bound.native_rate(), settings);
    let anchor = skeleton
        .root_motion_joint()
        .filter(|_| rules.root_motion.anchor_end && rm_joint.is_none() && rules.additive.is_none())
        .map(|rm| (rm, end_anchor(skeleton, &bound, start + duration, rm)))
        .or_else(|| rm_joint.filter(|_| rules.root_motion.horizontal).map(|rm| (rm, start_anchor(skeleton, &bound, start, rm))));
    let truth = Truth { skeleton, bound: &bound, start, duration, anchor };
    // Rates go coarsest first and the first that passes wins. A pass whose
    // resampling error alone exceeds 1.5x the bound can't pass
    // (quantization adds at most half the bound, its budget), so it is
    // neither quantized nor measured; a measurement stops at the first
    // sample over the bound. If no rate passes (a source with kinks,
    // discontinuities or very fast motion), the rate with the lowest
    // resampling error is kept, preferring the coarser on ties (10%).
    let mut best: Option<Pass> = None;
    for &rate in &rates {
        let n = frame_count(duration, rate);
        let mut pass = Pass::new(skeleton, &truth, n, rm_joint, rules);
        pass.resampling = pass.resampling_error(skeleton, &truth, settings.shell_distance);
        let bound = settings.error_bound;
        if pass.resampling <= bound * 1.5 && pass.cook(skeleton, &source.name, settings, bound) <= bound {
            best = Some(pass);
            break;
        }
        if best.as_ref().is_none_or(|b| pass.resampling < b.resampling * 0.9) {
            best = Some(pass);
        }
        if n < 2 {
            break;
        }
    }
    let mut pass = best.expect("at least one candidate rate");
    if !pass.exact {
        pass.cook(skeleton, &source.name, settings, f32::INFINITY);
    }
    let mut clip = pass.clip.take().expect("cooked");
    let n = clip.frame_count as usize;

    // Curves, events, markers, loop range: shift from source time.
    clip.curves = source
        .curves
        .iter()
        .map(|c| CurveTrack { name: c.name.clone(), values: (0..n).map(|f| c.curve.sample(start + frame_time(f, n, duration))).collect() })
        .collect();
    clip.events = shift_events(source.events.iter().chain(&rules.events), start, duration);
    clip.markers = shift_markers(source.markers.iter().chain(&rules.markers), start, duration);
    clip.loop_range = match rules.looping {
        Looping::Once => None,
        Looping::Whole => Some((0.0, duration)),
        Looping::Range(a, b) => Some(((a - start).clamp(0.0, duration), (b - start).clamp(0.0, duration))),
    };
    clip.meta.root_motion_mode = rules.root_motion.mode;
    clip.meta.unbound_joints.clone_from(&bound.unbound);
    if clip.root_motion.is_some() {
        root_meta(&mut clip);
    }
    if rules.contacts.enabled && rules.additive.is_none() && n >= 2 {
        contacts(skeleton, profile, &pass.source_frames, &mut clip, &rules.contacts);
        let fix = rm_joint.zip(contact_root(skeleton, profile, &pass.source_frames, &clip, &rules.contacts));
        if let Some((rm, (track, stance))) = fix {
            // Re-root the cooked frames on the contact-consistent track (the
            // model poses don't change) and quantize them again.
            let old = clip.root_motion.as_ref().expect("contact_root needs a track");
            reroot(skeleton, &mut pass.frames, rm, old, &track);
            pass.root_motion = Some(RootMotionTrack { frames: track, attitude: old.attitude.clone() });
            pass.cook(skeleton, &source.name, settings, f32::INFINITY);
            let mut fresh = pass.clip.take().expect("cooked");
            fresh.curves = std::mem::take(&mut clip.curves);
            fresh.events = std::mem::take(&mut clip.events);
            fresh.markers = std::mem::take(&mut clip.markers);
            fresh.loop_range = clip.loop_range;
            let max_error = fresh.meta.max_error;
            fresh.meta = std::mem::take(&mut clip.meta);
            fresh.meta.max_error = max_error;
            if let Some(u) = stance {
                fresh.meta.stance_velocity = u;
            }
            clip = fresh;
            root_meta(&mut clip);
        }
    }
    clip.markers.sort_by(|a, b| a.time.total_cmp(&b.time));
    Ok(clip)
}

/// Cooks already-resampled frames (no root-motion extraction: a
/// `FrameClip` carries its own). The error is measured against the frames.
pub fn cook_frames(skeleton: &Skeleton, frames: &FrameClip, settings: &CookSettings) -> Result<Clip, CookError> {
    let j = skeleton.len();
    if frames.frames.len() != frames.frame_count * j {
        return Err(CookError::BadFrames);
    }
    let n = frames.frame_count;
    let mut clip = quantize(skeleton, &frames.name, &frames.frames, n, frames.rate, frames.duration, settings);
    clip.root_motion.clone_from(&frames.root_motion);
    clip.curves.clone_from(&frames.curves);
    clip.events.clone_from(&frames.events);
    clip.markers.clone_from(&frames.markers);
    clip.loop_range = frames.loop_range;
    clip.meta = frames.meta.clone();
    let rest = LocalPose::rest(skeleton);
    let truth = |t: f32, out: &mut [Trs]| {
        let (f0, f1, a) = clip_frame_pair(n, frames.duration, t);
        for (k, o) in out.iter_mut().enumerate() {
            let (p, q) = (frames.frames[f0 * j + k], frames.frames[f1 * j + k]);
            *o = Trs {
                translation: p.translation.lerp(q.translation, a),
                rotation: math::nlerp(p.rotation, q.rotation, a),
                scale: p.scale.lerp(q.scale, a),
            };
        }
    };
    // Additive frames are deltas: measure them applied to the rest pose.
    let base = frames.meta.additive.then_some(&rest);
    let mut local = vec![Trs::IDENTITY; j];
    let mut applied = |_: usize, t: f32, out: &mut Vec<Affine3A>| {
        truth(t, &mut local);
        if base.is_some() {
            for (k, o) in local.iter_mut().enumerate() {
                *o = apply_delta(skeleton.rest()[k], *o);
            }
        }
        model_of(skeleton, &local, out);
    };
    clip.meta.max_error = measure(skeleton, &clip, base, false, &mut applied, settings.shell_distance, f32::INFINITY);
    Ok(clip)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn clip_frame_pair(n: usize, duration: f32, t: f32) -> (usize, usize, f32) {
    if n < 2 || duration <= 0.0 {
        return (0, 0, 0.0);
    }
    let x = (t / duration).clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (x as usize).min(n - 2);
    (i, i + 1, (x - i as f32).clamp(0.0, 1.0))
}

fn candidate_rates(native: Option<f32>, s: &CookSettings) -> Vec<f32> {
    let mut rates = Vec::new();
    if let Some(native) = native {
        let multiple = (s.min_rate / native).ceil().max(1.0);
        let r = native * multiple;
        if r <= s.max_rate * 1.001 {
            rates.push(r);
        }
    }
    let mut r = s.min_rate;
    while r <= s.max_rate * 1.001 {
        rates.push(r);
        r *= 2.0;
    }
    if rates.is_empty() {
        rates.push(s.min_rate);
    }
    rates.sort_by(f32::total_cmp);
    rates.dedup_by(|a, b| (*a - *b).abs() < 0.01 * *b);
    rates
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn frame_count(duration: f32, rate: f32) -> usize {
    if duration <= 1e-6 { 1 } else { ((duration * rate - 1e-3).ceil().max(1.0) as usize) + 1 }
}

fn additive_delta(t: Trs, base: Trs) -> Trs {
    let safe = |a: f32, b: f32| if b.abs() > 1e-12 { a / b } else { 1.0 };
    Trs {
        translation: t.translation - base.translation,
        rotation: (t.rotation * base.rotation.inverse()).normalize(),
        scale: Vec3::new(safe(t.scale.x, base.scale.x), safe(t.scale.y, base.scale.y), safe(t.scale.z, base.scale.z)),
    }
}

fn apply_delta(base: Trs, d: Trs) -> Trs {
    Trs { translation: base.translation + d.translation, rotation: (d.rotation * base.rotation).normalize(), scale: base.scale * d.scale }
}

/// Model transforms of a local pose given as `Trs` (cook-time helper).
pub(crate) fn model_of(skeleton: &Skeleton, local: &[Trs], out: &mut Vec<Affine3A>) {
    out.clear();
    for (j, t) in local.iter().enumerate() {
        let m = t.to_affine();
        let m = match skeleton.parent(j) {
            Some(p) => out[p] * m,
            None => m,
        };
        out.push(m);
    }
}

/// Removes the root motion from `frames` (in place) and returns the track.
///
/// The ancestors of `joint`, root first.
pub(crate) fn ancestors(skeleton: &Skeleton, joint: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut a = skeleton.parent(joint);
    while let Some(p) = a {
        out.push(p);
        a = skeleton.parent(p);
    }
    out.reverse();
    out
}

/// [`RootMotionRules::anchor_end`]: the inverse of the root-motion joint's
/// ground frame at source time `end`: its ground position (less its rest
/// ground offset turned by the heading) and its heading against its rest
/// rotation, about +Y.
fn end_anchor(skeleton: &Skeleton, bound: &BoundSource<'_>, end: f32, rm: usize) -> Affine3A {
    let mut pose = skeleton.rest().to_vec();
    bound.sample(end, skeleton.rest(), &mut pose);
    let mut model = Vec::new();
    model_of(skeleton, &pose, &mut model);
    let rest = skeleton.rest_model();
    let ground = |m: &Affine3A| Vec3::new(m.translation.x, 0.0, m.translation.z);
    let yaw = math::twist_angle(math::affine_rotation(&model[rm]) * math::affine_rotation(&rest[rm]).inverse(), Vec3::Y);
    let heading = Quat::from_rotation_y(yaw);
    let origin = ground(&model[rm]) - heading * ground(&rest[rm]);
    Affine3A::from_rotation_translation(heading, origin).inverse()
}

/// Horizontal extraction: the shift that puts the root-motion joint's ground position at source
/// time `start` over its rest ground position (its heading is kept). A clip cut from a longer
/// take (a combo's recovery or next swing, a move group in a long timeline) can start with the
/// joint far from the take's origin; extraction keeps the frame-0 offset in the pose, so the
/// drawn body would stand that far off the capsule its root motion moves (UE's root bone, Unity's
/// "Root Transform Position (XZ)": the in-place pose stands over the character's root).
fn start_anchor(skeleton: &Skeleton, bound: &BoundSource<'_>, start: f32, rm: usize) -> Affine3A {
    let mut pose = skeleton.rest().to_vec();
    bound.sample(start, skeleton.rest(), &mut pose);
    let mut model = Vec::new();
    model_of(skeleton, &pose, &mut model);
    let rest = skeleton.rest_model();
    let ground = |m: &Affine3A| Vec3::new(m.translation.x, 0.0, m.translation.z);
    Affine3A::from_translation(ground(&rest[rm]) - ground(&model[rm]))
}

/// The character root at frame `f` is `C_f = (c_f, yaw_f)` with
/// `c_f = h(p_f) − R_y(yaw_f)·h(p_0)`, where `p` is the root-motion joint's
/// model position and `h` keeps the extracted axes. In the root's frame the
/// joint then keeps its frame-0 ground position and heading.
pub(crate) fn extract_root_motion(skeleton: &Skeleton, frames: &mut [Trs], n: usize, rm: usize, rules: &RootMotionRules) -> RootMotionTrack {
    let joints = skeleton.len();
    let mut ancestors = Vec::new();
    let mut a = skeleton.parent(rm);
    while let Some(p) = a {
        ancestors.push(p);
        a = skeleton.parent(p);
    }
    ancestors.reverse();
    let mask = |v: Vec3| {
        Vec3::new(if rules.horizontal { v.x } else { 0.0 }, if rules.vertical { v.y } else { 0.0 }, if rules.horizontal { v.z } else { 0.0 })
    };
    let mut out = Vec::with_capacity(n);
    let mut attitude = Vec::new();
    let (mut p0, mut q0) = (Vec3::ZERO, Quat::IDENTITY);
    let (mut yaw, mut prev_raw) = (0.0f32, 0.0f32);
    for f in 0..n {
        let pose = &mut frames[f * joints..(f + 1) * joints];
        let parent = ancestors.iter().fold(Affine3A::IDENTITY, |m, &j| m * pose[j].to_affine());
        let m = parent * pose[rm].to_affine();
        let p = Vec3::from(m.translation);
        let q = math::affine_rotation(&m);
        if f == 0 {
            p0 = p;
            q0 = q;
        }
        // With attitude, yaw is the Y angle of the same YXZ decomposition, so
        // heading × attitude rebuilds the root rotation exactly.
        let raw = match (rules.yaw, rules.attitude) {
            (false, _) => 0.0,
            (true, false) => math::twist_angle(q * q0.inverse(), Vec3::Y),
            (true, true) => (q * q0.inverse()).normalize().to_euler(glam::EulerRot::YXZ).0,
        };
        if f > 0 {
            yaw += math::wrap_angle(raw - prev_raw);
        }
        prev_raw = raw;
        let ry = Quat::from_rotation_y(yaw);
        let c = mask(p) - ry * mask(p0);
        out.push([c.x, c.y, c.z, yaw]);
        let mut frame_rot = ry;
        if rules.attitude {
            // Residual rotation in the heading frame, as YXZ Euler: X is pitch, Z roll.
            let (_, pitch, roll) = (ry.inverse() * q * q0.inverse()).normalize().to_euler(glam::EulerRot::YXZ);
            attitude.push([pitch, roll]);
            frame_rot = ry * crate::clip::attitude_rotation(glam::Vec2::new(pitch, roll));
        }
        let local = parent.inverse() * Affine3A::from_rotation_translation(frame_rot, c).inverse() * m;
        let t = Trs::from_affine(&local);
        pose[rm].translation = t.translation;
        pose[rm].rotation = t.rotation;
    }
    RootMotionTrack { frames: out, attitude }
}

/// Per joint: number of moving channels on the longest root-to-leaf chain
/// through it.
fn chain_counts(skeleton: &Skeleton, moving: &[u32]) -> Vec<u32> {
    let n = skeleton.len();
    let mut up = vec![0u32; n];
    for j in 0..n {
        up[j] = moving[j] + skeleton.parent(j).map_or(0, |p| up[p]);
    }
    let mut down = moving.to_vec();
    for j in (0..n).rev() {
        if let Some(p) = skeleton.parent(j) {
            down[p] = down[p].max(moving[p] + down[j]);
        }
    }
    (0..n).map(|j| (up[j] + down[j] - moving[j]).max(1)).collect()
}

enum RotEnc {
    Constant(Quat),
    Quantized { drop: u8, bits: u8, min: [f32; 3], step: [f32; 3], ints: Vec<[u32; 3]> },
    Raw(Vec<Quat>),
}

enum VecEnc {
    Constant(Vec3),
    Quantized { bits: u8, min: [f32; 3], step: [f32; 3], ints: Vec<[u32; 3]> },
    Raw(Vec<Vec3>),
}

fn components(q: Quat) -> [f32; 4] {
    [q.x, q.y, q.z, q.w]
}

fn kept(c: [f32; 4], drop: usize) -> [f32; 3] {
    let mut k = [0.0; 3];
    let mut i = 0;
    for (n, v) in c.iter().enumerate() {
        if n != drop {
            k[i] = *v;
            i += 1;
        }
    }
    k
}

fn max_int(bits: u8) -> f32 {
    #[allow(clippy::cast_precision_loss)]
    let m = ((1u64 << bits) - 1) as f32;
    m
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn quantize_value(v: f32, min: f32, range: f32, bits: u8) -> u32 {
    if range <= 0.0 {
        return 0;
    }
    let m = max_int(bits);
    ((v - min) / range * m).round().clamp(0.0, m) as u32
}

#[allow(clippy::cast_precision_loss)]
fn ranges(values: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for v in values {
        for i in 0..3 {
            min[i] = min[i].min(v[i]);
            max[i] = max[i].max(v[i]);
        }
    }
    (min, [max[0] - min[0], max[1] - min[1], max[2] - min[2]])
}

fn quantize3(values: &[[f32; 3]], min: [f32; 3], range: [f32; 3], bits: u8) -> (Vec<[u32; 3]>, [f32; 3]) {
    let ints = values.iter().map(|v| [0, 1, 2].map(|i| quantize_value(v[i], min[i], range[i], bits))).collect();
    let m = max_int(bits);
    (ints, range.map(|r| r / m))
}

#[allow(clippy::cast_precision_loss)]
fn dequantize3(ints: [u32; 3], min: [f32; 3], step: [f32; 3]) -> Vec3 {
    Vec3::new(min[0] + step[0] * ints[0] as f32, min[1] + step[1] * ints[1] as f32, min[2] + step[2] * ints[2] as f32)
}

/// Smallest bit count in 1..=24 for which `ok` holds (assumed monotonic),
/// searched from the estimate `guess`: the rounding bound usually lands
/// on the answer or next to it, so this takes about two tests where a
/// bisection of 1..=24 takes six.
fn min_bits(guess: u8, mut ok: impl FnMut(u8) -> bool) -> Option<u8> {
    let mut b = guess.clamp(1, 24);
    if ok(b) {
        while b > 1 && ok(b - 1) {
            b -= 1;
        }
        return Some(b);
    }
    while b < 24 {
        b += 1;
        if ok(b) {
            return Some(b);
        }
    }
    None
}

/// Bits for which rounding each of three components over `range` stays
/// within `tolerance` (the step's half-diagonal), times `scale` (2 for
/// quaternion components: angle ≈ 2 × component error).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn bits_guess(range: [f32; 3], tolerance: f32, scale: f32) -> u8 {
    let diagonal = Vec3::from(range).length() * 0.5 * scale;
    if tolerance <= 0.0 || !diagonal.is_finite() {
        return 24;
    }
    (diagonal / tolerance + 1.0).log2().ceil().clamp(1.0, 24.0) as u8
}

fn encode_rotation(values: &[Quat], tolerance: f32) -> RotEnc {
    let first = values[0];
    if values.iter().all(|q| math::angle_between(*q, first) <= tolerance) {
        return RotEnc::Constant(first.normalize());
    }
    // Sign-continuous track.
    let mut aligned = Vec::with_capacity(values.len());
    let mut prev = first;
    for q in values {
        let q = math::align_hemisphere(q.normalize(), prev);
        aligned.push(q);
        prev = q;
    }
    // Drop the component that stays farthest from zero with a fixed sign.
    let mut best: Option<(usize, f32)> = None;
    for c in 0..4 {
        let comp: Vec<f32> = aligned.iter().map(|q| components(*q)[c]).collect();
        let same_sign = comp.iter().all(|v| *v > 0.0) || comp.iter().all(|v| *v < 0.0);
        let min_abs = comp.iter().fold(f32::INFINITY, |m, v| m.min(v.abs()));
        if same_sign && best.is_none_or(|(_, m)| min_abs > m) {
            best = Some((c, min_abs));
        }
    }
    let Some((drop, min_abs)) = best.filter(|(_, m)| *m > 0.05) else {
        return RotEnc::Raw(aligned);
    };
    let _ = min_abs;
    if components(aligned[0])[drop] < 0.0 {
        for q in &mut aligned {
            *q = -*q;
        }
    }
    let k: Vec<[f32; 3]> = aligned.iter().map(|q| kept(components(*q), drop)).collect();
    let (min, range) = ranges(&k);
    #[allow(clippy::cast_possible_truncation)]
    let drop8 = drop as u8;
    let error = |bits: u8| -> (f32, Vec<[u32; 3]>, [f32; 3]) {
        let (ints, step) = quantize3(&k, min, range, bits);
        let err = ints
            .iter()
            .zip(&aligned)
            .map(|(i, q)| {
                let v = dequantize3(*i, min, step);
                let d = (1.0 - v.length_squared()).max(0.0).sqrt();
                math::angle_between(rebuild(v, d, drop8).normalize(), *q)
            })
            .fold(0.0, f32::max);
        (err, ints, step)
    };
    match min_bits(bits_guess(range, tolerance, 2.0), |b| error(b).0 <= tolerance) {
        Some(bits) => {
            let (_, ints, step) = error(bits);
            RotEnc::Quantized { drop: drop8, bits, min, step, ints }
        }
        None => RotEnc::Raw(aligned),
    }
}

fn encode_vector(values: &[Vec3], tolerance: f32) -> VecEnc {
    let first = values[0];
    if values.iter().all(|v| v.distance(first) <= tolerance) {
        return VecEnc::Constant(first);
    }
    let k: Vec<[f32; 3]> = values.iter().map(Vec3::to_array).collect();
    let (min, range) = ranges(&k);
    let error = |bits: u8| {
        let (ints, step) = quantize3(&k, min, range, bits);
        let err = ints.iter().zip(values).map(|(i, v)| dequantize3(*i, min, step).distance(*v)).fold(0.0, f32::max);
        (err, ints, step)
    };
    match min_bits(bits_guess(range, tolerance, 1.0), |b| error(b).0 <= tolerance) {
        Some(bits) => {
            let (_, ints, step) = error(bits);
            VecEnc::Quantized { bits, min, step, ints }
        }
        None => VecEnc::Raw(values.to_vec()),
    }
}

fn write_bits(data: &mut [u8], bit: usize, bits: u8, value: u32) {
    let byte = bit >> 3;
    let mut word = u64::from_le_bytes(data[byte..byte + 8].try_into().expect("8 bytes"));
    word |= (u64::from(value) & ((1u64 << bits) - 1)) << (bit & 7);
    data[byte..byte + 8].copy_from_slice(&word.to_le_bytes());
}

/// Classifies and quantizes every channel of `frames` (`n × joints`).
#[allow(clippy::too_many_lines)]
fn quantize(skeleton: &Skeleton, name: &str, frames: &[Trs], n: usize, rate: f32, duration: f32, s: &CookSettings) -> Clip {
    let joints = skeleton.len();
    let channel = |j: usize| -> (Vec<Quat>, Vec<Vec3>, Vec<Vec3>) {
        let it = (0..n).map(|f| frames[f * joints + j]);
        (it.clone().map(|t| t.rotation).collect(), it.clone().map(|t| t.translation).collect(), it.map(|t| t.scale).collect())
    };
    // Pass 1: which channels move at all (beyond float noise).
    let mut moving = vec![0u32; joints];
    for (j, m) in moving.iter_mut().enumerate() {
        let (r, t, sc) = channel(j);
        *m = u32::from(r.iter().any(|q| math::angle_between(*q, r[0]) > 1e-6))
            + u32::from(t.iter().any(|v| v.distance(t[0]) > 1e-7))
            + u32::from(sc.iter().any(|v| v.distance(sc[0]) > 1e-7));
    }
    let chains = chain_counts(skeleton, &moving);
    let reach = skeleton.subtree_reach();
    let q_bound = s.error_bound * 0.5;

    let mut constant = vec![Trs::IDENTITY; joints];
    let mut rot_enc = Vec::new();
    let mut vec_enc = Vec::new();
    for j in 0..joints {
        #[allow(clippy::cast_precision_loss)]
        let budget = q_bound / chains[j] as f32;
        let lever = s.shell_distance + reach[j];
        let (r, t, sc) = channel(j);
        match encode_rotation(&r, budget / lever) {
            RotEnc::Constant(q) => constant[j].rotation = q,
            e => rot_enc.push((j, e)),
        }
        match encode_vector(&t, budget) {
            VecEnc::Constant(v) => constant[j].translation = v,
            e => vec_enc.push((j, VecChannel::Translation, e)),
        }
        match encode_vector(&sc, budget / lever) {
            VecEnc::Constant(v) => constant[j].scale = v,
            e => vec_enc.push((j, VecChannel::Scale, e)),
        }
    }

    // Lay out the frame record.
    let mut offset = 0usize;
    let to_u16 = |j: usize| u16::try_from(j).expect("joint index fits u16");
    let to_u32 = |o: usize| u32::try_from(o).expect("frame record fits u32 bits");
    let mut rotations = Vec::new();
    for (j, e) in &rot_enc {
        let (bits, drop, min, step, size) = match e {
            RotEnc::Quantized { drop, bits, min, step, .. } => (*bits, *drop, *min, *step, 3 * usize::from(*bits)),
            RotEnc::Raw(_) => (RAW, 3, [0.0; 3], [0.0; 3], 128),
            RotEnc::Constant(_) => unreachable!("constants are not tracks"),
        };
        rotations.push(RotTrack { joint: to_u16(*j), bits, drop, offset: to_u32(offset), min, step });
        offset += size;
    }
    let mut vectors = Vec::new();
    for (j, channel, e) in &vec_enc {
        let (bits, min, step, size) = match e {
            VecEnc::Quantized { bits, min, step, .. } => (*bits, *min, *step, 3 * usize::from(*bits)),
            VecEnc::Raw(_) => (RAW, [0.0; 3], [0.0; 3], 96),
            VecEnc::Constant(_) => unreachable!("constants are not tracks"),
        };
        vectors.push(VecTrack { joint: to_u16(*j), channel: *channel, bits, offset: to_u32(offset), min, step });
        offset += size;
    }
    let frame_bytes = offset.div_ceil(8);
    let mut data = vec![0u8; n * frame_bytes + 8];
    for f in 0..n {
        let base = f * frame_bytes * 8;
        for (track, (_, e)) in rotations.iter().zip(&rot_enc) {
            let bit = base + track.offset as usize;
            match e {
                RotEnc::Quantized { bits, ints, .. } => {
                    for (i, v) in ints[f].iter().enumerate() {
                        write_bits(&mut data, bit + i * usize::from(*bits), *bits, *v);
                    }
                }
                RotEnc::Raw(q) => {
                    for (i, v) in components(q[f]).iter().enumerate() {
                        write_bits(&mut data, bit + i * 32, RAW, v.to_bits());
                    }
                }
                RotEnc::Constant(_) => {}
            }
        }
        for (track, (_, _, e)) in vectors.iter().zip(&vec_enc) {
            let bit = base + track.offset as usize;
            match e {
                VecEnc::Quantized { bits, ints, .. } => {
                    for (i, v) in ints[f].iter().enumerate() {
                        write_bits(&mut data, bit + i * usize::from(*bits), *bits, *v);
                    }
                }
                VecEnc::Raw(v) => {
                    for (i, x) in v[f].to_array().iter().enumerate() {
                        write_bits(&mut data, bit + i * 32, RAW, x.to_bits());
                    }
                }
                VecEnc::Constant(_) => {}
            }
        }
    }
    Clip::from_parts(
        name.to_owned(),
        skeleton.id(),
        constant,
        duration,
        rate,
        u32::try_from(n).expect("frame count fits u32"),
        rotations,
        vectors,
        u32::try_from(frame_bytes).expect("frame record fits u32"),
        data,
    )
}

/// The source, sampled on demand: local poses at frame times, model poses
/// at any time.
struct Truth<'a> {
    skeleton: &'a Skeleton,
    bound: &'a BoundSource<'a>,
    start: f32,
    duration: f32,
    /// The root-motion joint and a model-space shift applied to every sample:
    /// [`RootMotionRules::anchor_end`]'s inverse of its ground frame at the
    /// last frame, or horizontal extraction's [`start_anchor`].
    anchor: Option<(usize, Affine3A)>,
}

impl Truth<'_> {
    /// Local pose at frame `f` of `n` (clip time `frame_time(f, n)`).
    fn frame(&self, f: usize, n: usize, out: &mut [Trs]) {
        self.bound.sample(self.start + frame_time(f, n, self.duration), self.skeleton.rest(), out);
        if let Some((rm, inverse)) = self.anchor {
            let parent = ancestors(self.skeleton, rm).iter().fold(Affine3A::IDENTITY, |m, &j| m * out[j].to_affine());
            let local = parent.inverse() * inverse * parent * out[rm].to_affine();
            let t = Trs::from_affine(&local);
            out[rm].translation = t.translation;
            out[rm].rotation = t.rotation;
        }
    }

    /// Model pose at grid point `s` of `points` (`local` is scratch).
    fn model(&self, s: usize, points: usize, local: &mut [Trs], out: &mut Vec<Affine3A>) {
        self.frame(s, points, local);
        model_of(self.skeleton, local, out);
    }
}

/// One candidate rate: its frames after root-motion extraction and the
/// additive conversion, and the cooked clip once quantized.
struct Pass {
    n: usize,
    rate: f32,
    duration: f32,
    frames: Vec<Trs>,
    /// The frames as sampled (for contacts).
    source_frames: Vec<Trs>,
    root_motion: Option<RootMotionTrack>,
    base: Option<LocalPose>,
    clip: Option<Clip>,
    /// Source model poses at the midpoints between frames, from
    /// [`Self::resampling_error`] (reused by the measurement).
    midpoints: Vec<Affine3A>,
    /// Resampling error ([`Self::resampling_error`]).
    resampling: f32,
    /// Whether `clip`'s `max_error` is complete (not a stopped measurement).
    exact: bool,
}

impl Pass {
    fn new(skeleton: &Skeleton, truth: &Truth, n: usize, rm_joint: Option<usize>, rules: &ClipRules) -> Self {
        let joints = skeleton.len();
        let (start, duration) = (truth.start, truth.duration);
        let mut frames = vec![Trs::IDENTITY; n * joints];
        for f in 0..n {
            truth.frame(f, n, &mut frames[f * joints..(f + 1) * joints]);
        }
        let source_frames = frames.clone();
        let root_motion = rm_joint.map(|rm| extract_root_motion(skeleton, &mut frames, n, rm, &rules.root_motion));
        let base = rules.additive.map(|b| {
            let base: Vec<Trs> = match b {
                AdditiveBase::FirstFrame => frames[..joints].to_vec(),
                AdditiveBase::Rest => skeleton.rest().to_vec(),
                AdditiveBase::Time(t) => {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
                    let f = (((t - start) / duration.max(f32::EPSILON)) * (n - 1) as f32).round().clamp(0.0, (n - 1) as f32) as usize;
                    frames[f * joints..(f + 1) * joints].to_vec()
                }
            };
            for f in 0..n {
                for (j, b) in base.iter().enumerate() {
                    let t = &mut frames[f * joints + j];
                    *t = additive_delta(*t, *b);
                }
            }
            LocalPose::from_trs(&base)
        });
        #[allow(clippy::cast_precision_loss)]
        let rate = if n < 2 { 0.0 } else { (n - 1) as f32 / duration };
        Self { n, rate, duration, frames, source_frames, root_motion, base, clip: None, midpoints: Vec::new(), resampling: 0.0, exact: false }
    }

    /// Virtual-vertex error of the unquantized frames, interpolated as the
    /// runtime interpolates, against the source at every midpoint (the
    /// frames are the source). The cooked error is this give or take the
    /// quantization error, which its budget keeps under half the bound.
    fn resampling_error(&mut self, skeleton: &Skeleton, truth: &Truth, shell: f32) -> f32 {
        let joints = skeleton.len();
        let rm = skeleton.root_motion_joint().filter(|_| self.root_motion.is_some());
        let (mut local, mut reference) = (vec![Trs::IDENTITY; joints], Vec::with_capacity(joints));
        let mut ev = Evaluator::new(skeleton, shell);
        let mut worst = 0.0f32;
        for f in 0..self.n.saturating_sub(1) {
            let (a, b) = (&self.frames[f * joints..(f + 1) * joints], &self.frames[(f + 1) * joints..(f + 2) * joints]);
            for (j, (p, q)) in a.iter().zip(b).enumerate() {
                ev.pose.set_rotation(j, math::nlerp(p.rotation, q.rotation, 0.5));
                ev.pose.set_translation(j, p.translation.lerp(q.translation, 0.5));
                ev.pose.set_scale(j, p.scale.lerp(q.scale, 0.5));
            }
            let root = rm.zip(self.root_motion.as_ref()).map(|(rm, track)| {
                let (p, q) = (track.frames[f], track.frames[f + 1]);
                let lerp = |i: usize| p[i] + (q[i] - p[i]) * 0.5;
                let attitude = if track.attitude.len() == track.frames.len() {
                    glam::Vec2::from(track.attitude[f]).lerp(glam::Vec2::from(track.attitude[f + 1]), 0.5)
                } else {
                    glam::Vec2::ZERO
                };
                (rm, Vec3::new(lerp(0), lerp(1), lerp(2)), lerp(3), attitude)
            });
            truth.model(2 * f + 1, 2 * self.n - 1, &mut local, &mut reference);
            self.midpoints.extend_from_slice(&reference);
            worst = worst.max(ev.error(self.base.as_ref(), root, &reference));
        }
        worst
    }

    /// Quantizes the frames (once) and measures the cooked clip against
    /// the source at every frame and midpoint, stopping at the first
    /// sample over `stop`. Returns the error found.
    fn cook(&mut self, skeleton: &Skeleton, name: &str, settings: &CookSettings, stop: f32) -> f32 {
        let mut clip = self.clip.take().unwrap_or_else(|| {
            let mut clip = quantize(skeleton, name, &self.frames, self.n, self.rate, self.duration, settings);
            clip.root_motion.clone_from(&self.root_motion);
            clip.meta.additive = self.base.is_some();
            clip
        });
        let joints = skeleton.len();
        let mut reference = |s: usize, _t: f32, out: &mut Vec<Affine3A>| {
            if s.is_multiple_of(2) {
                // Frame times: the frames as sampled are the source there.
                model_of(skeleton, &self.source_frames[s / 2 * joints..(s / 2 + 1) * joints], out);
            } else {
                out.clear();
                out.extend_from_slice(&self.midpoints[s / 2 * joints..(s / 2 + 1) * joints]);
            }
        };
        let error = measure(skeleton, &clip, self.base.as_ref(), true, &mut reference, settings.shell_distance, stop);
        clip.meta.max_error = error;
        self.exact = error <= stop;
        self.clip = Some(clip);
        error
    }
}

/// Model poses of candidate local poses and their virtual-vertex distance
/// to a reference (ACL's error metric).
struct Evaluator<'a> {
    skeleton: &'a Skeleton,
    shell: f32,
    pose: LocalPose,
    applied: LocalPose,
    model: ModelPose,
}

impl<'a> Evaluator<'a> {
    fn new(skeleton: &'a Skeleton, shell: f32) -> Self {
        let j = skeleton.len();
        Self { skeleton, shell, pose: LocalPose::new(j), applied: LocalPose::new(j), model: ModelPose::new(j) }
    }

    /// Error of `self.pose` (a delta on `base` when given; with `root`,
    /// the root-motion joint's subtree moved by `(joint, position, yaw,
    /// attitude)`) against the model poses `reference`.
    fn error(&mut self, base: Option<&LocalPose>, root: Option<(usize, Vec3, f32, glam::Vec2)>, reference: &[Affine3A]) -> f32 {
        let local = match base {
            Some(b) => {
                self.applied.copy_from(b);
                apply_additive(&mut self.applied, &self.pose, 1.0, None);
                &self.applied
            }
            None => &self.pose,
        };
        local_to_model(self.skeleton, local, &mut self.model, &Affine3A::IDENTITY);
        if let Some((rm, p, yaw, attitude)) = root {
            let heading = Quat::from_rotation_y(yaw) * crate::clip::attitude_rotation(attitude);
            let c = Affine3A::from_rotation_translation(heading, p);
            for j in self.skeleton.subtree(rm) {
                self.model.mats[j] = c * self.model.mats[j];
            }
        }
        // The vertex `shell · axis_i` maps to `t + shell · column_i`, so
        // its distance is that of the column and translation differences.
        let mut worst = 0.0f32;
        for (r, m) in reference.iter().zip(&self.model.mats) {
            let dt = Vec3::from(r.translation - m.translation);
            let d = r.matrix3 - m.matrix3;
            for col in [d.x_axis, d.y_axis, d.z_axis] {
                worst = worst.max((dt + Vec3::from(col) * self.shell).length_squared());
            }
        }
        worst.sqrt()
    }
}

/// Largest virtual-vertex distance between the cooked clip and the
/// reference model poses at every frame and midpoint (`reference(sample,
/// time, out)`), or the first one over `stop`. With `with_root`, the root
/// motion is re-applied to the root-motion joint's subtree (the reference
/// is the source).
fn measure(
    skeleton: &Skeleton,
    clip: &Clip,
    base: Option<&LocalPose>,
    with_root: bool,
    reference: &mut dyn FnMut(usize, f32, &mut Vec<Affine3A>),
    shell: f32,
    stop: f32,
) -> f32 {
    let n = clip.frame_count as usize;
    let samples = if n < 2 { 1 } else { 2 * n - 1 };
    let mut ref_model = Vec::new();
    let mut ev = Evaluator::new(skeleton, shell);
    let rm = if with_root { skeleton.root_motion_joint().filter(|_| clip.root_motion.is_some()) } else { None };
    let mut worst = 0.0f32;
    for s in 0..samples {
        #[allow(clippy::cast_precision_loss)]
        let t = if samples == 1 { 0.0 } else { clip.duration * s as f32 / (samples - 1) as f32 };
        reference(s, t, &mut ref_model);
        clip.sample(t, &mut ev.pose, None);
        let root = rm.map(|rm| {
            let (p, yaw) = clip.root_at(t);
            (rm, p, yaw, clip.attitude_at(t))
        });
        worst = worst.max(ev.error(base, root, &ref_model));
        if worst > stop {
            break;
        }
    }
    worst
}

fn shift_events<'a>(events: impl Iterator<Item = &'a Event>, start: f32, duration: f32) -> Vec<Event> {
    let mut out: Vec<Event> = events
        .filter(|e| e.time >= start - 1e-5 && e.time <= start + duration + 1e-5)
        .map(|e| Event {
            time: (e.time - start).clamp(0.0, duration),
            end: e.end.map(|x| (x - start).clamp(0.0, duration)),
            name: e.name.clone(),
            payload: e.payload.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.time.total_cmp(&b.time));
    out
}

fn shift_markers<'a>(markers: impl Iterator<Item = &'a SyncMarker>, start: f32, duration: f32) -> Vec<SyncMarker> {
    markers
        .filter(|m| m.time >= start - 1e-5 && m.time <= start + duration + 1e-5)
        .map(|m| SyncMarker { time: (m.time - start).clamp(0.0, duration), name: m.name.clone() })
        .collect()
}

pub(crate) fn root_meta(clip: &mut Clip) {
    let (a, b) = clip.loop_range.unwrap_or((0.0, clip.duration));
    let span = b - a;
    if span <= 0.0 {
        return;
    }
    let (p0, y0) = clip.root_at(a);
    let (p1, y1) = clip.root_at(b);
    clip.meta.average_velocity = (p1 - p0) / span;
    clip.meta.turn_rate = (y1 - y0) / span;
    clip.meta.cycle_distance = (p1 - p0).length();
}

/// Contact curves (`contact.<joint>`) and touch-down markers
/// (`down.<joint>`) from the source motion, the way motion-matching
/// pipelines label contacts: a foot is planted while it is low (height
/// over its lowest point in the clip) and still relative to the ground
/// (its velocity in the clip's space plus the ground velocity), with
/// hysteresis (a Schmitt trigger: stricter thresholds to plant than to
/// stay planted) so a rolling heel or a slow lift doesn't flicker.
///
/// Travelling clips carry the ground in their root motion (ground velocity
/// zero). In-place clips slide their planted feet back at the ground
/// speed: it is estimated from the low feet (component median, then the
/// mean of those that move with it) and written to the clip's meta.
#[allow(clippy::too_many_lines)]
fn contacts(skeleton: &Skeleton, profile: Option<&RigProfile>, source_frames: &[Trs], clip: &mut Clip, rules: &ContactRules) {
    let joints = skeleton.len();
    let n = clip.frame_count as usize;
    let (feet, leg_lengths): (Vec<usize>, Vec<f32>) = contact_feet(skeleton, profile, rules).into_iter().unzip();
    if feet.is_empty() {
        return;
    }
    let mut model = Vec::new();
    let mut positions = vec![Vec::with_capacity(n); feet.len()];
    // The toe beside each foot (stance velocity: a rolling heel isn't slide).
    let toes: Vec<Option<usize>> = feet.iter().map(|&foot| toe_joint(skeleton, profile, foot)).collect();
    let mut toe_positions = vec![Vec::with_capacity(n); feet.len()];
    for f in 0..n {
        model_of(skeleton, &source_frames[f * joints..(f + 1) * joints], &mut model);
        for (i, &foot) in feet.iter().enumerate() {
            positions[i].push(Vec3::from(model[foot].translation));
            if let Some(toe) = toes[i] {
                toe_positions[i].push(Vec3::from(model[toe].translation));
            }
        }
    }
    // A clip travels if its root (or the skeleton root) moves over the ground.
    let travel_joint = skeleton.root_motion_joint().unwrap_or(0);
    let travel = {
        model_of(skeleton, &source_frames[..joints], &mut model);
        let first = Vec3::from(model[travel_joint].translation);
        model_of(skeleton, &source_frames[(n - 1) * joints..], &mut model);
        let last = Vec3::from(model[travel_joint].translation);
        let d = last - first;
        Vec3::new(d.x, 0.0, d.z).length()
    };
    let in_place = travel < 0.1 * clip.duration.max(f32::EPSILON);
    let dt = if clip.rate > 0.0 { 1.0 / clip.rate } else { 1.0 };
    // Per foot: height over its floor and horizontal velocity, per frame.
    let loop_frames = clip.loop_range.map(|(a, b)| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let at = |t: f32| ((t / dt).round() as usize).min(n - 1);
        (at(a), at(b))
    });
    let cycle = loop_frames.filter(|(a, b)| a < b).map_or(0..n, |(a, b)| a..b + 1);
    let heights: Vec<Vec<f32>> = positions
        .iter()
        .map(|p| {
            // The floor of the frames that repeat: a looping group's intro and
            // outro may stand lower than its cycle ever does.
            let floor = p[cycle.clone()].iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
            p.iter().map(|v| v.y - floor).collect()
        })
        .collect();
    // Per foot and frame: horizontal velocity, and vertical (a planted foot
    // rising is lifting: heel-off).
    let (velocities, rises): (Vec<Vec<Vec3>>, Vec<Vec<f32>>) = positions.iter().map(|p| foot_velocities(p, dt)).unzip();
    let release = rules.release.max(1.0);
    let ground = if in_place { ground_velocity(&heights, &velocities, &leg_lengths, rules, release) } else { Vec3::ZERO };
    // Per frame, the stance reference: the slowest low foot over the ground.
    // Keyframed sources skate (a planted foot drifts at a fraction of the
    // travel speed); measuring every foot against the one that moves least
    // keeps that foot planted and the swinging one lifted, while clean
    // motion (a still reference) is measured against the ground itself.
    let reference: Vec<Vec3> = (0..n)
        .map(|f| {
            (0..feet.len())
                .filter(|&i| heights[i][f] < rules.height * release * leg_lengths[i])
                .map(|i| velocities[i][f] + ground)
                .min_by(|a, b| a.length_squared().total_cmp(&b.length_squared()))
                .unwrap_or(Vec3::ZERO)
        })
        .collect();
    let mut planted_velocity = Vec3::ZERO;
    let mut planted_count = 0u32;
    // Travelling loops: the planted feet's slide in the source's own space.
    let measure_stance = !in_place && clip.root_motion.is_some() && loop_frames.is_some_and(|(a, b)| a < b);
    let (mut stance_slide, mut stance_count) = (Vec3::ZERO, 0u32);
    for (i, &foot) in feet.iter().enumerate() {
        let (h, v, rise) = (&heights[i], &velocities[i], &rises[i]);
        let (height, speed, lift) = (rules.height * leg_lengths[i], rules.speed * leg_lengths[i], rules.lift * leg_lengths[i]);
        let world_speed = |f: usize| (v[f] + ground - reference[f]).length();
        let enter = |f: usize| h[f] < height && world_speed(f) < speed;
        let stay = |f: usize| h[f] < height * release && world_speed(f) < speed * release;
        let mut planted = vec![false; n];
        let mut state = enter(0);
        for (f, p) in planted.iter_mut().enumerate() {
            state = if state { stay(f) } else { enter(f) };
            *p = state;
        }
        // A loop's first frames follow its last: run the trigger round once more.
        if let Some((a, b)) = loop_frames.filter(|(a, b)| a < b) {
            let mut state = planted[b];
            for (f, p) in planted.iter_mut().enumerate().take(b + 1).skip(a) {
                state = if state { stay(f) } else { enter(f) };
                *p = state;
            }
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        fill_seam_gaps(&mut planted, loop_frames, (rules.gap / dt).round() as usize);
        trim_heel_off(&mut planted, rise, lift);
        let toe_v = (toe_positions[i].len() == n).then(|| foot_velocities(&toe_positions[i], dt).0);
        // The heel roll: after heel-off the toe is still down (same trigger
        // thresholds as the foot's "stay", over its own floor in the cycle).
        let roll = toe_v.as_ref().map(|tv| {
            let tp = &toe_positions[i];
            let floor = tp[cycle.clone()].iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
            let toe_down = |f: usize| tp[f].y - floor < height * release && (tv[f] + ground - reference[f]).length() < speed * release;
            roll_frames(&planted, toe_down, clip.loop_range.is_some())
        });
        if in_place {
            for f in (0..n).filter(|&f| planted[f]) {
                planted_velocity += v[f];
                planted_count += 1;
            }
        }
        if measure_stance {
            let (slide, count) = stance_slide_of(&planted, v, toe_v.as_deref(), &cycle);
            stance_slide += slide;
            stance_count += count;
        }
        let values: Vec<f32> = planted.iter().map(|&p| if p { 1.0 } else { 0.0 }).collect();
        let name = skeleton.name(foot);
        let looping = clip.loop_range.is_some();
        for f in 0..n {
            let prev = if f == 0 { if looping { values[n - 1] } else { 0.0 } } else { values[f - 1] };
            if values[f] > 0.5 && prev <= 0.5 {
                #[allow(clippy::cast_precision_loss)]
                let time = (f as f32 * dt).min(clip.duration);
                clip.markers.push(SyncMarker { time, name: format!("down.{name}") });
            }
        }
        clip.curves.push(CurveTrack { name: format!("contact.{name}"), values });
        if let Some(values) = roll {
            clip.curves.push(CurveTrack { name: format!("roll.{name}"), values });
        }
    }
    if measure_stance {
        clip.meta.stance_velocity = stance_velocity(clip.meta.average_velocity, stance_slide, stance_count);
    }
    if in_place && clip.root_motion.is_none() && planted_count > 0 {
        // Planted feet slide backward at the ground speed.
        #[allow(clippy::cast_precision_loss)]
        let v = -planted_velocity / planted_count as f32;
        clip.meta.average_velocity = v;
        clip.meta.cycle_distance = v.length() * clip.loop_range.map_or(clip.duration, |(a, b)| b - a);
    }
}

/// One foot's slide over the source's ground, summed over its planted
/// frames of the loop `cycle` (not its closing duplicate), with their
/// count: the slower of heel and toe per frame, the point that stays down.
fn stance_slide_of(planted: &[bool], heel: &[Vec3], toe: Option<&[Vec3]>, cycle: &std::ops::Range<usize>) -> (Vec3, u32) {
    let (mut sum, mut count) = (Vec3::ZERO, 0u32);
    for f in cycle.start..cycle.end.saturating_sub(1) {
        if planted[f] {
            sum += toe.map_or(heel[f], |t| if t[f].length_squared() < heel[f].length_squared() { t[f] } else { heel[f] });
            count += 1;
        }
    }
    (sum, count)
}

/// How fast the planted feet move against the body: the root's velocity
/// minus their mean slide over the source's ground (`slide` summed over
/// `count` planted frames). Zero (unmeasured) unless it still travels the
/// root's way at a fair fraction of its speed.
fn stance_velocity(root: Vec3, slide: Vec3, count: u32) -> Vec3 {
    if count == 0 {
        return Vec3::ZERO;
    }
    #[allow(clippy::cast_precision_loss)]
    let stance = root - slide / count as f32;
    if stance.dot(root) > 0.0 && stance.length() > 0.25 * root.length() { stance } else { Vec3::ZERO }
}

/// Contact-consistent root motion ([`ContactRules::root_blend`]): the
/// root half of footskate cleanup (Kovar, Schreiner & Gleicher 2002), as
/// motion-matching data prep does it. Keyframed walks skate: their planted
/// feet move over the source's ground, unevenly within a stance, so a body
/// moving at a constant speed (the capsule) leaves a foot lock up to 20 cm
/// off its animation by heel-off, and the release whips the foot (AN-19).
/// Here the planar root velocity during stance becomes the pinned point's
/// own velocity (the heel while `contact` is up, as the foot lock holds it;
/// the toe while `roll` is) plus one constant `u`: in the root's frame the
/// planted foot then moves back at exactly `u`, and the body travels over
/// it as authored. Double support hands over between the feet by
/// trapezoid weights ramping over `root_blend`; unpinned frames (a run's
/// flight) keep the extracted motion, faded in over the same time. `u` closes the loop (the cycle's displacement is
/// kept) and is returned as the clip's stance velocity (only with stance
/// over at least a quarter of the cycle). The heading is low-passed too:
/// see the end. Travelling loops with a root track only.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::too_many_lines)]
fn contact_root(skeleton: &Skeleton, profile: Option<&RigProfile>, source_frames: &[Trs], clip: &Clip, rules: &ContactRules) -> Option<(Vec<[f32; 4]>, Option<Vec3>)> {
    let track = clip.root_motion.as_ref()?;
    let n = clip.frame_count as usize;
    let (ta, tb) = clip.loop_range?;
    if rules.root_blend <= 0.0 || clip.rate <= 0.0 || n < 3 || track.frames.len() != n {
        return None;
    }
    let dt = 1.0 / clip.rate;
    let at = |t: f32| ((t / dt).round() as usize).min(n - 1);
    let (a, b) = (at(ta), at(tb));
    if b < a + 2 {
        return None;
    }
    let len = b - a;
    let joints = skeleton.len();
    let feet: Vec<usize> = contact_feet(skeleton, profile, rules).into_iter().map(|(f, _)| f).collect();
    let toes: Vec<usize> = feet.iter().map(|&f| toe_joint(skeleton, profile, f).unwrap_or(f)).collect();
    let (mut heels, mut tips) = (vec![Vec::with_capacity(n); feet.len()], vec![Vec::with_capacity(n); feet.len()]);
    let mut model = Vec::new();
    for f in 0..n {
        model_of(skeleton, &source_frames[f * joints..(f + 1) * joints], &mut model);
        for i in 0..feet.len() {
            heels[i].push(Vec3::from(model[feet[i]].translation));
            tips[i].push(Vec3::from(model[toes[i]].translation));
        }
    }
    // Per cycle frame: the stance weight and the weighted pinned velocity.
    let ramp = (rules.root_blend / dt).max(1.0);
    let (mut weight, mut slide) = (vec![0.0f32; len], vec![Vec3::ZERO; len]);
    for (i, &foot) in feet.iter().enumerate() {
        let name = skeleton.name(foot);
        let curve = |prefix: &str| clip.curves.iter().find(|c| c.name == format!("{prefix}.{name}")).map(|c| &c.values);
        let Some(contact) = curve("contact") else { continue };
        let roll = curve("roll");
        let (hv, tv) = (foot_velocities(&heels[i], dt).0, foot_velocities(&tips[i], dt).0);
        let pin: Vec<Option<Vec3>> = (a..b)
            .map(|f| {
                if contact[f] > 0.5 {
                    Some(hv[f])
                } else {
                    roll.filter(|r| r[f] > 0.5).map(|_| tv[f])
                }
            })
            .collect();
        for k in 0..len {
            let Some(v) = pin[k] else { continue };
            // Frames to the stance's nearer edge (the cycle wraps): a trapezoid.
            let mut d = 1;
            while d < len && pin[(k + d) % len].is_some() && pin[(k + len - d) % len].is_some() {
                d += 1;
            }
            let w = (d as f32 / ramp).min(1.0);
            weight[k] += w;
            slide[k] += v * w;
        }
    }
    // Held wherever a foot is pinned (the trapezoids only hand over between
    // feet); fading out over the ramp into unpinned frames (a run's flight).
    let hold: Vec<f32> = (0..len)
        .map(|k| {
            let d = (0..len).find(|&d| weight[(k + d) % len] > 0.0 || weight[(k + len - d) % len] > 0.0).unwrap_or(len);
            (1.0 - d as f32 / ramp).max(0.0)
        })
        .collect();
    // Per step k (frame a+k to a+k+1): blend weight and pinned velocity.
    let step = |k: usize| {
        let mean = |j: usize| {
            let s = if weight[j] > 0.0 { slide[j] / weight[j] } else { Vec3::ZERO };
            (hold[j], s)
        };
        let ((w0, s0), (w1, s1)) = (mean(k), mean((k + 1) % len));
        (f32::midpoint(w0, w1), 0.5 * (w0 * s0 + w1 * s1))
    };
    let planar = |f: usize| Vec3::new(track.frames[f][0], 0.0, track.frames[f][2]);
    let distance = planar(b) - planar(a);
    let (mut stance_time, mut fixed) = (0.0f32, Vec3::ZERO);
    for k in 0..len {
        let (w, ws) = step(k);
        stance_time += w * dt;
        fixed += ws * dt + (1.0 - w) * (planar(a + k + 1) - planar(a + k));
    }
    let span = len as f32 * dt;
    let root = distance / span;
    // A travelling loop (as `contacts` tells one from an in-place clip).
    if distance.length() < 0.1 * span {
        return None;
    }
    let u = (distance - fixed) / stance_time.max(f32::EPSILON);
    let u = (stance_time >= 0.25 * span && u.dot(root) > 0.0 && u.length() >= 0.25 * root.length()).then_some(u);
    let mut frames = track.frames.clone();
    if let Some(u) = u {
        let mut c = planar(a);
        for k in 0..len {
            let (w, ws) = step(k);
            c += ws * dt + w * u * dt + (1.0 - w) * (planar(a + k + 1) - planar(a + k));
            frames[a + k + 1][0] = c.x;
            frames[a + k + 1][2] = c.z;
        }
    }
    // The heading: the hips' yaw swings with every step (±17° on a
    // keyframed male walk), and a root that carries it turns the whole body
    // back and forth about the capsule, swinging the planted feet. A loop's
    // heading is its travel heading: the yaw averaged over one cycle around
    // each frame (the periodic extension keeps a turning loop's turn rate).
    let turn = track.frames[b][3] - track.frames[a][3];
    #[allow(clippy::cast_possible_wrap)]
    let yaw = |i: isize| track.frames[a + i.rem_euclid(len as isize) as usize][3] + turn * i.div_euclid(len as isize) as f32;
    #[allow(clippy::cast_possible_wrap)]
    let heading: Vec<f32> = (0..=len as isize).map(|k| (k - len as isize / 2..k - len as isize / 2 + len as isize).map(yaw).sum::<f32>() / len as f32).collect();
    for (f, frame) in frames.iter_mut().enumerate() {
        // Outside the cycle (a lead-in, a tail): shifted as the cycle's nearer end.
        let k = f.clamp(a, b) - a;
        frame[3] += heading[k] - track.frames[a + k][3];
    }
    Some((frames, u))
}

/// Moves the root-motion joint `rm` of every frame from root track `old`
/// to `new` so its model pose (root × local) stays the same.
fn reroot(skeleton: &Skeleton, frames: &mut [Trs], rm: usize, old: &RootMotionTrack, new: &[[f32; 4]]) {
    let joints = skeleton.len();
    let ancestors: Vec<usize> = ancestors(skeleton, rm);
    let root = |c: &[f32; 4], f: usize| {
        let mut q = Quat::from_rotation_y(c[3]);
        if let Some(att) = old.attitude.get(f) {
            q *= crate::clip::attitude_rotation(glam::Vec2::from(*att));
        }
        Affine3A::from_rotation_translation(q, Vec3::new(c[0], c[1], c[2]))
    };
    for (f, (o, c)) in old.frames.iter().zip(new).enumerate() {
        let pose = &mut frames[f * joints..(f + 1) * joints];
        let parent = ancestors.iter().fold(Affine3A::IDENTITY, |m, &j| m * pose[j].to_affine());
        let local = parent.inverse() * root(c, f).inverse() * root(o, f) * parent * pose[rm].to_affine();
        let t = Trs::from_affine(&local);
        pose[rm].translation = t.translation;
        pose[rm].rotation = t.rotation;
    }
}

/// The toe beside a contact foot: the rig profile's, else the foot's first
/// child joint (None for a foot with neither).
fn toe_joint(skeleton: &Skeleton, profile: Option<&RigProfile>, foot: usize) -> Option<usize> {
    let mapped = profile.and_then(|p| p.limbs.iter().find(|l| usize::from(l.effector) == foot)).and_then(|l| l.foot.as_ref()).and_then(|f| f.toe);
    mapped.map(usize::from).or_else(|| (0..skeleton.len()).find(|&j| skeleton.parent(j) == Some(foot)))
}

/// One foot's per-frame velocity by central differences: horizontal, and
/// vertical.
fn foot_velocities(p: &[Vec3], dt: f32) -> (Vec<Vec3>, Vec<f32>) {
    let n = p.len();
    (0..n)
        .map(|f| {
            let (a, b) = (f.saturating_sub(1), (f + 1).min(n - 1));
            #[allow(clippy::cast_precision_loss)]
            let d = (p[b] - p[a]) / ((b - a).max(1) as f32 * dt);
            (Vec3::new(d.x, 0.0, d.z), d.y)
        })
        .unzip()
}

/// Debounce at a loop's seam: closes a contact gap of at most `max` frames
/// at the end or start of the cycle `a..b` (frame `b` repeats `a`) between
/// two planted runs. A velocity spike where the source's last key meets its
/// first would otherwise release and re-plant the foot mid stance, every
/// cycle (AN-17). Gaps inside the cycle are the source's own and stay.
fn fill_seam_gaps(planted: &mut [bool], loop_frames: Option<(usize, usize)>, max: usize) {
    let n = planted.len();
    let Some((a, b)) = loop_frames.filter(|&(a, b)| a < b && b < n) else { return };
    let len = b - a;
    // Scan from a planted frame, so a run across the seam is one run.
    let Some(first) = (0..len).find(|&i| planted[a + i]) else { return };
    let at = |k: usize| a + (first + k) % len;
    let mut k = 0;
    while k < len {
        if planted[at(k)] {
            k += 1;
            continue;
        }
        let start = k;
        while k < len && !planted[at(k)] {
            k += 1;
        }
        let seam = (start..k).any(|j| at(j) == a || at(j) == b - 1);
        if seam && k - start <= max {
            for j in start..k {
                planted[at(j)] = true;
            }
        }
    }
    planted[b] = planted[a];
}

/// Heel-off: a stance ends where the foot's last rise before lifting
/// begins (offline labelling sees the whole stance), so the lock doesn't
/// hold the heel down while the authored foot peels off (a heel held into
/// heel-off overreaches uphill). Rise jitter mid-stance doesn't split the
/// stance: only the rise faster than `lift` that runs into the lift is
/// trimmed, and only in the stance's second half, where heel-off falls.
fn trim_heel_off(planted: &mut [bool], rise: &[f32], lift: f32) {
    for f in 1..planted.len() {
        if planted[f - 1] && !planted[f] {
            let start = (0..f).rev().find(|&g| !planted[g]).map_or(0, |g| g + 1);
            let keep = start + (f - start).div_ceil(2);
            let mut g = f - 1;
            while g >= keep && g > 0 && rise[g] >= lift {
                planted[g] = false;
                g -= 1;
            }
        }
    }
}

/// The heel roll (`roll.<foot>`, 1 = rolling): from each stance's end (the
/// foot's heel-off, where `planted` drops) while the toe is still down
/// (`toe_down`), until the toe lifts or the foot plants again. The toe is
/// the pivot of the roll: `FootPlacement` pins it there instead of
/// releasing the whole foot at heel-off. Loops wrap.
fn roll_frames(planted: &[bool], toe_down: impl Fn(usize) -> bool, looping: bool) -> Vec<f32> {
    let n = planted.len();
    let mut roll = vec![0.0; n];
    for f in 0..n {
        let prev = if f > 0 { planted[f - 1] } else { looping && planted[n - 1] };
        if !prev || planted[f] {
            continue;
        }
        for k in 0..n {
            let g = if f + k < n { f + k } else if looping { f + k - n } else { break };
            if planted[g] || !toe_down(g) {
                break;
            }
            roll[g] = 1.0;
        }
    }
    roll
}

/// The ground velocity of an in-place clip, in its space: its planted feet
/// slide back at it. The component median of the low feet's velocities
/// (robust to the swing frames among them), then the mean of the low feet
/// moving with it.
fn ground_velocity(heights: &[Vec<f32>], velocities: &[Vec<Vec3>], leg_lengths: &[f32], rules: &ContactRules, release: f32) -> Vec3 {
    let low = |i: usize, f: usize| heights[i][f] < rules.height * release * leg_lengths[i];
    let mut xs = Vec::new();
    let mut zs = Vec::new();
    for (i, v) in velocities.iter().enumerate() {
        for (f, v) in v.iter().enumerate() {
            if low(i, f) {
                xs.push(v.x);
                zs.push(v.z);
            }
        }
    }
    if xs.is_empty() {
        return Vec3::ZERO;
    }
    let median = |s: &mut Vec<f32>| {
        s.sort_by(f32::total_cmp);
        s[s.len() / 2]
    };
    let guess = Vec3::new(-median(&mut xs), 0.0, -median(&mut zs));
    let (mut sum, mut count) = (Vec3::ZERO, 0u32);
    for (i, v) in velocities.iter().enumerate() {
        for (f, v) in v.iter().enumerate() {
            if low(i, f) && (*v + guess).length() < rules.speed * release * leg_lengths[i] {
                sum += *v;
                count += 1;
            }
        }
    }
    #[allow(clippy::cast_precision_loss)]
    if count == 0 { guess } else { -sum / count as f32 }
}
