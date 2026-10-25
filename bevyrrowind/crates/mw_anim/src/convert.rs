//! `mw_nif` curves → `sf_anim` source curves, cut to a time range.
//!
//! - Hermite curves (translation, scale) stay cubic: `mw_nif` keeps NIF
//!   tangents per segment, `sf_anim` wants units per second, so each tangent
//!   is divided by its own segment's duration. A cut at `a` or `b` adds a key
//!   with the curve's value and derivative there; per-second tangents don't
//!   change when a segment is split, so the cut curve is the same cubic.
//! - Translation snaps (a key jumping meters in one frame: root-motion wraps
//!   at loop and group ends, the dreugh's and slaughterfish's `Root Bone`)
//!   are curve breaks ([`snap_breaks`]): the snap segment is linear and the
//!   keys on either side keep one-sided tangents, so neither the authored
//!   handles nor TCB tangents carry the jump into the smooth motion around it.
//! - Quaternion keys stay linear (slerp in both crates); cuts add a sampled key.
//! - Euler tracks compose three curves, which slerp can't reproduce, so
//!   they are sampled at their key times and at least [`EULER_RATE`] Hz.

use glam::{Quat, Vec3};
use mw_nif::anim::{Curve as NifCurve, Interpolate, RotationTrack, TransformTrack};
use mw_nif::glam as nif_glam;
use sf_anim::{Curve, CurveValue, Interpolation, SourceCurve, SourceTrack};

/// Minimum sampling rate of Euler rotation tracks (Hz).
pub const EULER_RATE: f32 = 60.0;
/// Keys closer than this to a cut or to each other merge (seconds).
const EPSILON: f32 = 1e-4;
/// A translation snap: a segment at most [`SNAP_DURATION`] long (seconds)
/// that moves at least [`SNAP_CHORD`] (meters) and [`SNAP_SPEED_RATIO`]
/// times as fast as each neighboring segment.
pub const SNAP_DURATION: f32 = 0.1;
pub const SNAP_CHORD: f32 = 0.05;
pub const SNAP_SPEED_RATIO: f32 = 4.0;
/// A key next to a snap gets a one-sided tangent on its other side when
/// that tangent differs from its segment's chord by more than this fraction
/// of the jump (TCB and auto-smooth handles carry about half to all of it).
const TAINT: f32 = 0.25;

/// The track's channels within `[a, b]`, as times from `a`.
pub fn track(joint: &str, track: &TransformTrack, a: f32, b: f32) -> SourceTrack {
    SourceTrack {
        joint: joint.to_owned(),
        translation: track.translation.as_ref().map(|c| hermite(&snap_breaks(c), a, b, |v| Vec3::from_array(v.to_array()))),
        rotation: track.rotation.as_ref().map(|r| rotation(r, a, b)),
        scale: track.scale.as_ref().map(|c| hermite(c, a, b, Vec3::splat)),
    }
}

/// Step keys (`(time, value)`, time order) within `[a, b]`, as times from
/// `a`: the value holding at `a`, then each change inside the range.
pub fn step(name: &str, keys: &[(f32, f32)], a: f32, b: f32) -> SourceCurve {
    let at = |t: f32| keys[..keys.partition_point(|k| k.0 <= t)].last().or(keys.first()).map_or(0.0, |k| k.1);
    let mut times = vec![0.0];
    let mut values = vec![at(a)];
    for &(t, v) in keys {
        if t > a + EPSILON && t < b - EPSILON && values.last() != Some(&v) {
            times.push(t - a);
            values.push(v);
        }
    }
    SourceCurve { name: name.to_owned(), curve: Curve { times, values, interpolation: Interpolation::Step } }
}

/// A float curve (a geometry-morph weight) within `[a, b]`, as times from `a`.
pub fn float(name: &str, curve: &NifCurve<f32>, a: f32, b: f32) -> SourceCurve {
    SourceCurve { name: name.to_owned(), curve: hermite(curve, a, b, |v| v) }
}

/// A translation curve with its snaps made curve breaks. A snap segment
/// (see [`SNAP_DURATION`]) becomes linear (both its tangents are its
/// chord), and the keys bounding it take one-sided tangents on their other
/// side (their other segment's chord, as a curve's end keys do) when the
/// snap tainted them ([`TAINT`]): the curve as if each side were exported
/// on its own. Without this, the
/// authored Bezier handles (exported with the snap key as a neighbor) and
/// TCB tangents throw the motion next to a snap far past its keys: 3.8 m on
/// the dreugh's `Walkforward`, 0.9 m on the slaughterfish's.
pub fn snap_breaks(curve: &NifCurve<nif_glam::Vec3>) -> NifCurve<nif_glam::Vec3> {
    let mut keys: Vec<_> = curve.keys().collect();
    let n = keys.len();
    if n < 2 {
        return curve.clone();
    }
    let chord = |k: &[(f32, nif_glam::Vec3, nif_glam::Vec3, nif_glam::Vec3)], i: usize| k[i + 1].1 - k[i].1;
    let speed = |i: usize| {
        let dt = keys[i + 1].0 - keys[i].0;
        if dt > EPSILON { chord(&keys, i).length() / dt } else { 0.0 }
    };
    let snaps: Vec<usize> = (0..n - 1)
        .filter(|&i| {
            let dt = keys[i + 1].0 - keys[i].0;
            let neighbors = if i > 0 { speed(i - 1) } else { 0.0 }.max(if i + 2 < n { speed(i + 1) } else { 0.0 });
            dt > EPSILON && dt <= SNAP_DURATION && chord(&keys, i).length() >= SNAP_CHORD && speed(i) >= SNAP_SPEED_RATIO * neighbors
        })
        .collect();
    if snaps.is_empty() {
        return curve.clone();
    }
    let original = keys.clone();
    for &i in &snaps {
        let jump = chord(&original, i);
        keys[i].3 = jump;
        keys[i + 1].2 = jump;
        // A tangent that carries a good part of the jump saw the snap key as
        // its neighbor; authored tangents that didn't keep their shape.
        let tainted = |tangent: nif_glam::Vec3, side: nif_glam::Vec3| (tangent - side).length() > TAINT * jump.length();
        if i > 0 && !snaps.contains(&(i - 1)) && tainted(original[i].2, chord(&original, i - 1)) {
            keys[i].2 = chord(&original, i - 1);
        }
        if i + 2 < n && !snaps.contains(&(i + 1)) && tainted(original[i + 1].3, chord(&original, i + 1)) {
            keys[i + 1].3 = chord(&original, i + 1);
        }
    }
    NifCurve::hermite(keys)
}

/// Value and derivative (per second) of a NIF Hermite curve at `t`.
fn eval<T: Interpolate>(keys: &[(f32, T, T, T)], t: f32) -> (T, T) {
    let (first, last) = (keys[0], keys[keys.len() - 1]);
    if t <= first.0 {
        return (first.1, first.1 * 0.0);
    }
    if t >= last.0 {
        return (last.1, last.1 * 0.0);
    }
    let i = keys.partition_point(|k| k.0 <= t);
    let ((t0, p0, _, m0), (t1, p1, m1, _)) = (keys[i - 1], keys[i]);
    let span = t1 - t0;
    if span <= 0.0 {
        return (p1, p1 * 0.0);
    }
    let x = (t - t0) / span;
    let (x2, x3) = (x * x, x * x * x);
    let value = p0 * (2.0 * x3 - 3.0 * x2 + 1.0) + p1 * (3.0 * x2 - 2.0 * x3) + m0 * (x3 - 2.0 * x2 + x) + m1 * (x3 - x2);
    let slope = p0 * (6.0 * x2 - 6.0 * x) + p1 * (6.0 * x - 6.0 * x2) + m0 * (3.0 * x2 - 4.0 * x + 1.0) + m1 * (3.0 * x2 - 2.0 * x);
    (value, slope * (1.0 / span))
}

/// A NIF Hermite curve as a `CubicSpline` curve on `[a, b]`, mapped by the
/// linear `f` (unit or type change).
fn hermite<T: Interpolate, U: CurveValue>(curve: &NifCurve<T>, a: f32, b: f32, f: impl Fn(T) -> U) -> Curve<U> {
    // Drop zero-length segments: sf_anim needs strictly increasing times.
    let mut keys: Vec<(f32, T, T, T)> = Vec::with_capacity(curve.keys().len());
    for key in curve.keys() {
        match keys.last_mut() {
            Some(last) if key.0 - last.0 <= EPSILON => *last = key,
            _ => keys.push(key),
        }
    }
    let mut times = Vec::new();
    let mut values = Vec::new();
    let mut push = |t: f32, value: T, in_tan: T, out_tan: T| {
        times.push(t - a);
        values.extend([f(in_tan), f(value), f(out_tan)]);
    };
    let (start, start_slope) = eval(&keys, a);
    push(a, start, start_slope, start_slope);
    for (i, &(t, value, in_tan, out_tan)) in keys.iter().enumerate() {
        if t <= a + EPSILON || t >= b - EPSILON {
            continue;
        }
        // Outside its keys the curve holds, so the end keys are flat there.
        let in_tan = if i == 0 { value * 0.0 } else { in_tan * (1.0 / (t - keys[i - 1].0)) };
        let out_tan = keys.get(i + 1).map_or(value * 0.0, |next| out_tan * (1.0 / (next.0 - t)));
        push(t, value, in_tan, out_tan);
    }
    if b - a > EPSILON {
        // The derivative arriving at `b`: evaluate just inside the range.
        let (end, _) = eval(&keys, b);
        let (_, end_slope) = eval(&keys, b - EPSILON * 0.5);
        push(b, end, end_slope, end_slope);
    }
    Curve { times, values, interpolation: Interpolation::CubicSpline }
}

fn quat(q: nif_glam::Quat) -> Quat {
    Quat::from_array(q.to_array())
}

fn rotation(track: &RotationTrack, a: f32, b: f32) -> Curve<Quat> {
    let mut times: Vec<f32> = vec![a];
    match track {
        RotationTrack::Quats(keys) => times.extend(keys.iter().map(|k| k.0).filter(|&t| t > a + EPSILON && t < b - EPSILON)),
        RotationTrack::Euler { axes, .. } => {
            let mut keyed: Vec<f32> = axes.iter().flat_map(|c| c.keys().map(|k| k.0)).filter(|&t| t > a + EPSILON && t < b - EPSILON).collect();
            keyed.push(b);
            keyed.sort_by(f32::total_cmp);
            let mut last = a;
            for t in keyed {
                // Fill gaps so the composed rotation is sampled densely enough.
                let steps = ((t - last) * EULER_RATE).ceil().max(1.0);
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // small positive count
                for k in 1..steps as u32 {
                    #[allow(clippy::cast_precision_loss)]
                    times.push(last + (t - last) * k as f32 / steps);
                }
                if t < b - EPSILON {
                    times.push(t);
                }
                last = t;
            }
        }
    }
    if b - a > EPSILON {
        times.push(b);
    }
    times.dedup_by(|x, y| *x - *y <= EPSILON);
    let values = times.iter().map(|&t| track.sample(t).map_or(Quat::IDENTITY, quat)).collect();
    Curve { times: times.iter().map(|t| t - a).collect(), values, interpolation: Interpolation::Linear }
}
