//! Sync: aligning a follower's playhead to a leader's (UE sync groups).
//!
//! **Markers** first: the leader's position is "fraction `f` of the way
//! from marker `P` to marker `N`" ([`Clip::marker_phase`]); the follower is
//! placed the same fraction between its own `P`→`N` pair, choosing the
//! occurrence nearest its current playhead. That keeps feet in step even
//! when clips have different rhythms (a limp, a two-step run). **Phase**
//! otherwise: the same normalized time in each clip's loop.

use sf_anim::Clip;

/// The section a playhead loops in.
pub(crate) fn loop_span(clip: &Clip, looping: bool) -> (f32, f32) {
    match (looping, clip.loop_range) {
        (true, Some((a, b))) if b > a => (a, b),
        _ => (0.0, clip.duration),
    }
}

/// Normalized phase of `time` in its loop.
pub(crate) fn phase(clip: &Clip, looping: bool, time: f32) -> f32 {
    let (a, b) = loop_span(clip, looping);
    if b - a <= 1e-6 { 0.0 } else { ((time - a) / (b - a)).clamp(0.0, 1.0) }
}

/// Signed playhead distance from `from` to `to`, the short way round when
/// looping.
pub(crate) fn wrapped_delta(from: f32, to: f32, span: (f32, f32), looping: bool) -> f32 {
    let d = to - from;
    let len = span.1 - span.0;
    if !looping || len <= 1e-6 {
        return d;
    }
    let w = (d + len * 0.5).rem_euclid(len) - len * 0.5;
    if w.abs() < 1e-7 { 0.0 } else { w }
}

/// The follower playhead matching a leader at `leader_time`.
/// `follower_time` (its current playhead) picks among repeated marker pairs.
pub(crate) fn sync_time(follower: &Clip, follower_looping: bool, follower_time: f32, leader: &Clip, leader_looping: bool, leader_time: f32) -> f32 {
    if let Some(t) = marker_time(follower, follower_looping, follower_time, leader, leader_time) {
        return t;
    }
    let (a, b) = loop_span(follower, follower_looping);
    a + phase(leader, leader_looping, leader_time) * (b - a)
}

fn marker_time(follower: &Clip, looping: bool, current: f32, leader: &Clip, leader_time: f32) -> Option<f32> {
    let (prev, next, frac) = leader.marker_phase(leader_time)?;
    let (pn, nn) = (&leader.markers[prev].name, &leader.markers[next].name);
    let m = &follower.markers;
    let n = m.len();
    if n == 0 {
        return None;
    }
    let duration = follower.duration.max(1e-6);
    let span = loop_span(follower, looping);
    let mut best: Option<(f32, f32)> = None;
    for i in 0..n {
        let j = (i + 1) % n;
        if &m[i].name != pn || &m[j].name != nn {
            continue;
        }
        let (t0, t1) = (m[i].time, m[j].time);
        let wraps = t1 <= t0;
        if wraps && !looping {
            continue;
        }
        let len = if wraps { t1 - t0 + duration } else { t1 - t0 };
        let mut t = t0 + frac * len;
        if t > duration {
            t -= duration;
        }
        let dist = wrapped_delta(current, t, span, looping).abs();
        if best.is_none_or(|(d, _)| dist < d) {
            best = Some((dist, t));
        }
    }
    best.map(|(_, t)| t)
}
