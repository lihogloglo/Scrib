//! Loop cutter ([`ClipRules::loop_cut`]): a seamless cycle out of a take that
//! is not periodic (a diffusion-generated walk ramps up, holds a pace and
//! slows down; a mocap take starts and ends standing).
//!
//! The technique is the transition-point search of motion graphs (Kovar,
//! Gleicher & Pighin 2002, "Motion Graphs"; Lee et al. 2002), applied to one
//! clip against itself, as Unity's "Loop Pose" match and UE's loop-pose
//! cleanup do for authored cycles:
//!
//! 1. **Steady part.** The take is sampled at its key rate. The character
//!    root (the root-motion joint, else the skeleton root, projected on the
//!    ground, with its heading) moves at a speed averaged over one stride;
//!    the steady part is the longest run of frames within
//!    `1 - `[`LoopCutRules::plateau`] of the take's median speed (the pace it
//!    holds most of the time; ramps and spikes skipped). An
//!    in-place take (under 0.1 m/s) is steady throughout.
//! 2. **Window ends.** Candidates are pairs of touchdowns of the same foot
//!    (the cook's own `down.<foot>` markers, from the clip cooked unlooped)
//!    [`LoopCutRules::min_strides`] to [`LoopCutRules::max_strides`] strides
//!    apart, each end moved up to [`LoopCutRules::refine`] s. The distance of
//!    two frames is Kovar's point-cloud metric in character space: the mean
//!    squared distance of every joint's position (relative to the
//!    stride-smoothed character root, so the hips' own yaw swing and sway
//!    count) plus their velocities weighted by (0.1 s)^2. The window with the
//!    lowest distance wins; ties (within 10%) go to fewer strides, then to
//!    the window nearest the middle of the steady part.
//! 3. **Cross-fade.** The clip is trimmed to `[a, b]`. Its last
//!    [`LoopCutRules::blend`] s fade into the frames just before `a`, moved
//!    by one cycle (`Δ`: the character joint's ground displacement from `a` to
//!    `b` and the stride-smoothed heading's turn): the last frame is then exactly `Δ` times the
//!    first, so pose, feet and root velocity carry across the seam and every
//!    loop travels the same `Δ`. The root-motion joint fades in model space
//!    (its ancestors' frame), the rest in local space.
//!
//! The result is a new source keyed at the take's rate that the cook loops
//! whole ([`Looping::Whole`]); [`LoopCut`] reports the window and the seam
//! errors before and after the fade. A take without two same-foot
//! touchdowns in a long enough steady part is a [`LoopCutError`], and the
//! cook keeps the clip unlooped.

use std::collections::BTreeMap;

use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::cook::{ClipRules, Looping, ancestors, model_of, shift_events, shift_markers};
use crate::source::BoundSource;
use crate::{Clip, Curve, Skeleton, SourceClip, SourceCurve, SourceTrack, Trs, math};

/// Opt-in loop extraction of one clip ([`ClipRules::loop_cut`]).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LoopCutRules {
    /// Fewest strides (same-foot touchdown to the next) in the loop.
    pub min_strides: u32,
    /// Most strides in the loop.
    pub max_strides: u32,
    /// The steady part: frames whose stride-averaged speed is within
    /// `1 - plateau` of the take's median speed (0.9: within 10%).
    pub plateau: f32,
    /// Cross-fade at the seam, seconds (0.12 s: 4 frames at 30 Hz).
    pub blend: f32,
    /// How far each window end may move from its touchdown, seconds.
    pub refine: f32,
    /// Source seconds the window must lie in (within the clip's `range`).
    pub search: Option<(f32, f32)>,
}

impl Default for LoopCutRules {
    fn default() -> Self {
        Self { min_strides: 1, max_strides: 2, plateau: 0.9, blend: 0.12, refine: 0.1, search: None }
    }
}

/// What the loop cutter chose, and how well the seam closes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoopCut {
    /// The window, source seconds.
    pub start: f32,
    pub end: f32,
    /// Strides in the loop.
    pub strides: u32,
    /// The foot whose touchdowns bound the window.
    pub foot: String,
    /// Frames cross-faded at the seam.
    pub blend_frames: u32,
    /// Before the fade (a hard cut): root-relative joint distance between
    /// the window's ends, RMS over joints, meters.
    pub cut_pose_error: f32,
    /// Before the fade: the largest contact-foot jump at the seam, meters.
    pub cut_foot_jump: f32,
    /// Before the fade: the root's ground speed change across the seam, m/s.
    pub cut_speed_jump: f32,
    /// After the fade: the last frame against the first moved by one cycle,
    /// RMS over joints, meters (0 up to rounding).
    pub seam_pose_error: f32,
    /// After the fade: the largest difference, over the contact feet, between
    /// the last step into the seam and the take's own step into the window's
    /// start (the fade's residue), meters per frame.
    pub seam_foot_jump: f32,
    /// After the fade: the root's ground speed into the seam against the
    /// take's own speed into the window's start, m/s.
    pub seam_speed_jump: f32,
    /// Ground speed of the loop (its travel per cycle over its duration), m/s.
    pub speed: f32,
    /// Mean stride-averaged ground speed of the steady part, m/s.
    pub plateau_speed: f32,
    /// Heading change per loop, radians.
    pub turn: f32,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum LoopCutError {
    #[error("no foot contacts to cut a loop at (contacts off, an additive clip, or no contact feet)")]
    NoContacts,
    #[error("no foot touches down twice in the take")]
    NoRepeatedContact,
    #[error("the steady part ({steady:.2} s) is shorter than {strides} stride(s) of {stride:.2} s plus the {blend:.2} s fade")]
    TooShort { steady: f32, strides: u32, stride: f32, blend: f32 },
    #[error("no window of {0} to {1} strides between same-foot touchdowns fits the steady part")]
    NoWindow(u32, u32),
}

/// A cut take: the looping source, the rules that cook it, the report.
pub(crate) struct Cut {
    pub source: SourceClip,
    pub rules: ClipRules,
    pub stats: LoopCut,
}

/// Weight of joint velocities against positions in the frame distance, s².
const VELOCITY_WEIGHT: f32 = 0.01;
/// Below this stride-averaged speed (m/s) a take is in place.
const IN_PLACE_SPEED: f32 = 0.1;

fn smoothstep(u: f32) -> f32 {
    let u = u.clamp(0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}

fn blend(a: Trs, b: Trs, w: f32) -> Trs {
    Trs { translation: a.translation.lerp(b.translation, w), rotation: math::nlerp(a.rotation, b.rotation, w), scale: a.scale.lerp(b.scale, w) }
}

fn planar(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}

fn character(c: Vec3, yaw: f32) -> Affine3A {
    Affine3A::from_rotation_translation(Quat::from_rotation_y(yaw), c)
}

/// The take sampled at its key rate, in model space, with its character root.
struct Sampled {
    rate: f32,
    /// Absolute frame number of local frame 0 (source time `first / rate`).
    first: usize,
    local: Vec<Vec<Trs>>,
    model: Vec<Vec<Affine3A>>,
    /// The character joint's raw ground position and unwrapped heading.
    ground: Vec<Vec3>,
    yaw: Vec<f32>,
}

impl Sampled {
    #[allow(clippy::cast_precision_loss)]
    fn time(&self, f: usize) -> f32 {
        (self.first + f) as f32 / self.rate
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
fn sample(skeleton: &Skeleton, source: &SourceClip, joint: usize, (start, end): (f32, f32)) -> Sampled {
    let bound = BoundSource::new(source, skeleton);
    let rate = bound.native_rate().filter(|r| (10.0..=240.0).contains(r)).unwrap_or(30.0);
    let first = (start * rate - 1e-3).ceil().max(0.0) as usize;
    let last = ((end * rate + 1e-3).floor().max(0.0) as usize).max(first);
    let rest_rot = math::affine_rotation(&skeleton.rest_model()[joint]);
    let mut s = Sampled { rate, first, local: Vec::new(), model: Vec::new(), ground: Vec::new(), yaw: Vec::new() };
    let (mut prev, mut acc) = (0.0f32, 0.0f32);
    for f in first..=last {
        let mut pose = skeleton.rest().to_vec();
        bound.sample(f as f32 / rate, skeleton.rest(), &mut pose);
        let mut model = Vec::new();
        model_of(skeleton, &pose, &mut model);
        let raw = math::twist_angle(math::affine_rotation(&model[joint]) * rest_rot.inverse(), Vec3::Y);
        acc = if f == first { raw } else { acc + math::wrap_angle(raw - prev) };
        prev = raw;
        s.ground.push(planar(Vec3::from(model[joint].translation)));
        s.yaw.push(acc);
        s.local.push(pose);
        s.model.push(model);
    }
    s
}

/// Cuts a loop out of `source` (rules `rules`, cooked unlooped as `plain`).
#[allow(clippy::too_many_lines, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::cast_possible_wrap)]
pub(crate) fn cut(skeleton: &Skeleton, source: &SourceClip, rules: &ClipRules, plain: &Clip, cut: &LoopCutRules) -> Result<Cut, LoopCutError> {
    let feet: Vec<usize> = plain.curves.iter().filter_map(|c| c.name.strip_prefix("contact.")).filter_map(|n| skeleton.find(n)).collect();
    if feet.is_empty() {
        return Err(LoopCutError::NoContacts);
    }
    let (range_start, range_end) = rules.range.unwrap_or((0.0, source.duration));
    let (lo, hi) = cut.search.map_or((range_start, range_end), |(a, b)| (a.max(range_start), b.min(range_end)));
    let joint = skeleton.root_motion_joint().unwrap_or(0);
    let s = sample(skeleton, source, joint, (range_start, range_end));
    let n = s.local.len();
    let rate = s.rate;
    // Touchdowns per foot, local frames (the plain clip's times start at the range).
    let mut downs: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for m in &plain.markers {
        let Some(name) = m.name.strip_prefix("down.") else { continue };
        let f = ((m.time + range_start) * rate).round() as isize - s.first as isize;
        if (0..n as isize).contains(&f) {
            downs.entry(name).or_default().push(f as usize);
        }
    }
    let mut intervals: Vec<usize> = downs.values().flat_map(|d| d.windows(2).map(|w| w[1] - w[0])).collect();
    if intervals.is_empty() {
        return Err(LoopCutError::NoRepeatedContact);
    }
    intervals.sort_unstable();
    let stride = intervals[intervals.len() / 2].max(2);
    let half = stride / 2;
    // The character root, smoothed over one stride.
    let window = |f: usize| f.saturating_sub(half)..=(f + half).min(n - 1);
    let centre: Vec<Vec3> = (0..n).map(|f| window(f).map(|g| s.ground[g]).sum::<Vec3>() / window(f).count() as f32).collect();
    let heading: Vec<f32> = (0..n).map(|f| window(f).map(|g| s.yaw[g]).sum::<f32>() / window(f).count() as f32).collect();
    let frame = |f: usize| character(centre[f], heading[f]);
    // Stride-averaged ground speed (frames with a whole stride around them).
    let speed: Vec<Option<f32>> =
        (0..n).map(|f| (f >= half && f + half < n).then(|| s.ground[f + half].distance(s.ground[f - half]) * rate / (2 * half) as f32)).collect();
    let mut sorted: Vec<f32> = speed.iter().flatten().copied().collect();
    sorted.sort_by(f32::total_cmp);
    let pace = sorted.get(sorted.len() / 2).copied().unwrap_or(0.0);
    let first_allowed = ((lo * rate - 1e-3).ceil() as isize - s.first as isize).max(0) as usize;
    let last_allowed = (((hi * rate + 1e-3).floor() as isize - s.first as isize).max(0) as usize).min(n - 1);
    let (steady_lo, steady_hi) = if pace < IN_PLACE_SPEED {
        (first_allowed, last_allowed)
    } else {
        // The longest run within `1 - plateau` of the median pace.
        let ok = |f: usize| (first_allowed..=last_allowed).contains(&f) && speed[f].is_some_and(|v| (v - pace).abs() <= (1.0 - cut.plateau) * pace);
        let (mut best, mut run) = ((0, 0), None::<usize>);
        for f in 0..=n {
            match (f < n && ok(f), run) {
                (true, None) => run = Some(f),
                (false, Some(r)) => {
                    if f - r > best.1 - best.0 {
                        best = (r, f);
                    }
                    run = None;
                }
                _ => {}
            }
        }
        (best.0, best.1.saturating_sub(1))
    };
    let fade = ((cut.blend * rate).round() as usize).max(1);
    let min_strides = cut.min_strides.max(1);
    let max_strides = cut.max_strides.max(min_strides);
    let steady = steady_hi.saturating_sub(steady_lo);
    if steady < min_strides as usize * stride {
        return Err(LoopCutError::TooShort { steady: steady as f32 / rate, strides: min_strides, stride: stride as f32 / rate, blend: fade as f32 / rate });
    }
    // Per frame and joint of the character joint's subtree (a static
    // wrapper above it stays behind): position and velocity in the
    // character frame.
    let subtree: Vec<usize> = (0..skeleton.len())
        .filter(|&j| std::iter::successors(Some(j), |&x| skeleton.parent(x)).any(|x| x == joint))
        .collect();
    let joints = subtree.len();
    let pos = |f: usize, j: usize| Vec3::from(s.model[f][j].translation);
    let (mut p, mut v) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for f in 0..n {
        let inv = frame(f).inverse();
        let (f0, f1) = (f.saturating_sub(1), (f + 1).min(n - 1));
        let dt = (f1 - f0).max(1) as f32 / rate;
        p.push(subtree.iter().map(|&j| inv.transform_point3(pos(f, j))).collect::<Vec<_>>());
        v.push(subtree.iter().map(|&j| inv.transform_vector3((pos(f1, j) - pos(f0, j)) / dt)).collect::<Vec<_>>());
    }
    let distance = |a: usize, b: usize| {
        let (mut dp, mut dv) = (0.0, 0.0);
        for j in 0..joints {
            dp += p[a][j].distance_squared(p[b][j]);
            dv += v[a][j].distance_squared(v[b][j]);
        }
        (dp + VELOCITY_WEIGHT * dv) / joints as f32
    };
    // Candidate windows: same-foot touchdowns, each end refined.
    let refine = (cut.refine * rate).round() as isize;
    let fits = |a: isize, b: isize| a >= steady_lo as isize && a >= fade as isize && b <= steady_hi as isize && b - a >= (2 * fade).max(2) as isize;
    let mut candidates: Vec<(f32, u32, usize, usize, &str)> = Vec::new();
    for (&foot, d) in &downs {
        for x in 0..d.len() {
            for k in min_strides..=max_strides {
                let Some(&y) = d.get(x + k as usize) else { break };
                let mut best: Option<(f32, usize, usize)> = None;
                for a in d[x] as isize - refine..=d[x] as isize + refine {
                    for b in y as isize - refine..=y as isize + refine {
                        if !fits(a, b) {
                            continue;
                        }
                        let c = distance(a as usize, b as usize);
                        if best.is_none_or(|(bc, _, _)| c < bc) {
                            best = Some((c, a as usize, b as usize));
                        }
                    }
                }
                if let Some((c, a, b)) = best {
                    candidates.push((c, k, a, b, foot));
                }
            }
        }
    }
    let lowest = candidates.iter().map(|c| c.0).fold(f32::INFINITY, f32::min);
    let middle = (steady_lo + steady_hi) as f32 / 2.0;
    let off_middle = |a: usize, b: usize| ((a + b) as f32 / 2.0 - middle).abs();
    let chosen = candidates
        .iter()
        .filter(|c| c.0 <= lowest * 1.1 + 1e-8)
        .min_by(|x, y| x.1.cmp(&y.1).then(off_middle(x.2, x.3).total_cmp(&off_middle(y.2, y.3))).then(x.0.total_cmp(&y.0)))
        .ok_or(LoopCutError::NoWindow(min_strides, max_strides))?;
    let (_, strides, a, b, foot) = *chosen;
    let len = b - a;
    // One cycle: the smoothed heading's turn, about the character joint's
    // own ground position (at the same phase its sway cancels, where the
    // stride-smoothed centre would lag near a ramp).
    let turn = heading[b] - heading[a];
    let delta = Affine3A::from_translation(s.ground[b]) * Affine3A::from_rotation_y(turn) * Affine3A::from_translation(-s.ground[a]);
    // The trimmed, cross-faded frames: the last `fade` blend into the
    // frames before `a`, moved by one cycle.
    let up = ancestors(skeleton, joint);
    let chain = |pose: &[Trs]| up.iter().fold(Affine3A::IDENTITY, |m, &j| m * pose[j].to_affine());
    let weight = |k: usize| if k + fade > len { smoothstep((k + fade - len) as f32 / fade as f32) } else { 0.0 };
    let out: Vec<Vec<Trs>> = (0..=len)
        .map(|k| {
            let base = &s.local[a + k];
            let w = weight(k);
            if w <= 0.0 {
                return base.clone();
            }
            let next = &s.local[a + k - len];
            let mut pose: Vec<Trs> = base.iter().zip(next).map(|(&x, &y)| blend(x, y, w)).collect();
            let moved = chain(&pose).inverse() * delta * chain(next) * next[joint].to_affine();
            pose[joint] = blend(base[joint], Trs::from_affine(&moved), w);
            pose
        })
        .collect();
    // Seam report.
    let rms = |sum: f32| (sum / joints as f32).sqrt();
    let cut_pose_error = rms((0..joints).map(|j| p[a][j].distance_squared(p[b][j])).sum());
    let cut_foot_jump = feet.iter().map(|&j| pos(b, j).distance(delta.transform_point3(pos(a, j)))).fold(0.0, f32::max);
    // A hard cut steps from `b - 1` to `a` moved by one cycle.
    let cut_speed_jump = (planar(delta.transform_point3(pos(a, joint)) - pos(b - 1, joint)).length() - planar(pos(a, joint) - pos(a - 1, joint)).length()).abs() * rate;
    let model: Vec<Vec<Affine3A>> = [0, len - 1, len]
        .iter()
        .map(|&k| {
            let mut m = Vec::new();
            model_of(skeleton, &out[k], &mut m);
            m
        })
        .collect();
    let at = |i: usize, j: usize| Vec3::from(model[i][j].translation);
    let seam_pose_error = rms(subtree.iter().map(|&j| at(2, j).distance_squared(delta.transform_point3(at(0, j)))).sum());
    // The last step into the seam against the take's own step into frame `a`
    // (what follows the seam is the step out of `a`): the fade's residue,
    // not the motion's own acceleration at a touchdown.
    let into_a = |j: usize| delta.transform_vector3(pos(a, j) - pos(a - 1, j));
    let seam_foot_jump = feet.iter().map(|&j| ((at(2, j) - at(1, j)) - into_a(j)).length()).fold(0.0, f32::max);
    let seam_speed_jump = (planar(at(2, joint) - at(1, joint)).length() - planar(into_a(joint)).length()).abs() * rate;
    let steady_speeds: Vec<f32> = (steady_lo..=steady_hi).filter_map(|f| speed[f]).collect();
    let plateau_speed = if steady_speeds.is_empty() { 0.0 } else { steady_speeds.iter().sum::<f32>() / steady_speeds.len() as f32 };
    let (start, end) = (s.time(a), s.time(b));
    let stats = LoopCut {
        start,
        end,
        strides,
        foot: foot.to_owned(),
        blend_frames: fade as u32,
        cut_pose_error,
        cut_foot_jump,
        cut_speed_jump,
        seam_pose_error,
        seam_foot_jump,
        seam_speed_jump,
        speed: s.ground[b].distance(s.ground[a]) * rate / len as f32,
        plateau_speed,
        turn,
    };
    // The looping source: tracks keyed per frame, curves cross-faded alike.
    let times: Vec<f32> = (0..=len).map(|k| k as f32 / rate).collect();
    let tracks = source
        .tracks
        .iter()
        .map(|t| {
            let Some(j) = skeleton.find(&t.joint) else {
                // Unbound: kept for the meta, resampled as is.
                let shift = |c: &Curve<Vec3>| Curve::linear(times.clone(), times.iter().map(|&x| c.sample(start + x)).collect());
                return SourceTrack {
                    joint: t.joint.clone(),
                    translation: t.translation.as_ref().map(shift),
                    rotation: t.rotation.as_ref().map(|c| Curve::linear(times.clone(), times.iter().map(|&x| c.sample(start + x)).collect())),
                    scale: t.scale.as_ref().map(shift),
                };
            };
            let mut rotations: Vec<Quat> = Vec::with_capacity(out.len());
            for pose in &out {
                let q = pose[j].rotation;
                rotations.push(rotations.last().map_or(q, |&r| math::align_hemisphere(q, r)));
            }
            SourceTrack {
                joint: t.joint.clone(),
                translation: t.translation.as_ref().map(|_| Curve::linear(times.clone(), out.iter().map(|p| p[j].translation).collect())),
                rotation: t.rotation.as_ref().map(|_| Curve::linear(times.clone(), rotations)),
                scale: t.scale.as_ref().map(|_| Curve::linear(times.clone(), out.iter().map(|p| p[j].scale).collect())),
            }
        })
        .collect();
    let curves = source
        .curves
        .iter()
        .map(|c| {
            let values = (0..=len)
                .map(|k| {
                    let x = c.curve.sample(s.time(a + k));
                    let w = weight(k);
                    if w > 0.0 { x + (c.curve.sample(s.time(a + k - len)) - x) * w } else { x }
                })
                .collect();
            SourceCurve { name: c.name.clone(), curve: Curve::linear(times.clone(), values) }
        })
        .collect();
    let duration = len as f32 / rate;
    let cut_source = SourceClip {
        name: source.name.clone(),
        duration,
        tracks,
        curves,
        events: shift_events(source.events.iter().chain(&rules.events), start, duration),
        markers: shift_markers(source.markers.iter().chain(&rules.markers), start, duration),
    };
    let cut_rules = ClipRules { range: None, looping: Looping::Whole, events: Vec::new(), markers: Vec::new(), loop_cut: None, ..rules.clone() };
    Ok(Cut { source: cut_source, rules: cut_rules, stats })
}
