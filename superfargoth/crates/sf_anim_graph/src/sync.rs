//! Sync: aligning a follower's playhead to a leader's (UE sync groups).
//!
//! **Markers** first: the leader's position is "fraction `f` of the way
//! from marker `P` to marker `N`"; the follower is placed the same fraction
//! between its own `P`→`N` pair, choosing the occurrence nearest its
//! current playhead. That keeps feet in step even when clips have
//! different rhythms (a limp, a two-step run). **Phase** otherwise: the
//! same normalized time in each clip's loop.
//!
//! A looping clip with a loop range inside it (an intro before, an outro
//! after) cycles through its loop only: marker pairs wrap at the loop's
//! end, and markers outside the loop are never sync targets. A follower
//! placed past its loop's end would play out to the clip's end and park
//! there, frozen (walkers5: torch-bearers' frozen walk lanes).

use sf_anim::{Clip, SyncMarker};

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

/// Brings a looping playhead past its loop's end back into the loop.
pub(crate) fn into_loop(clip: &Clip, looping: bool, time: f32) -> f32 {
    let (a, b) = loop_span(clip, looping);
    if looping && b - a > 1e-6 && time >= b { a + (time - a).rem_euclid(b - a) } else { time }
}

/// The markers a playhead keeps passing: those inside the loop when
/// looping, else all.
fn cycle_markers(clip: &Clip, looping: bool) -> impl Iterator<Item = &SyncMarker> + Clone {
    let (a, b) = loop_span(clip, looping);
    clip.markers.iter().filter(move |m| !looping || (m.time >= a && m.time < b))
}

/// The leader's markers around `time` and the fraction between them:
/// `(previous name, next name, fraction)`.
fn leader_phase(clip: &Clip, looping: bool, time: f32) -> Option<(&str, &str, f32)> {
    if !looping {
        let (prev, next, frac) = clip.marker_phase(time)?;
        return Some((&clip.markers[prev].name, &clip.markers[next].name, frac));
    }
    let (a, b) = loop_span(clip, looping);
    let len = (b - a).max(f32::EPSILON);
    let t = into_loop(clip, looping, time);
    let markers = cycle_markers(clip, looping);
    let first = markers.clone().next()?;
    let last = markers.clone().last()?;
    let prev = markers.clone().take_while(|m| m.time <= t).last().unwrap_or(last);
    let next = markers.clone().find(|m| m.time > t).unwrap_or(first);
    let gap = if next.time > prev.time { next.time - prev.time } else { next.time - prev.time + len };
    let since = (t - prev.time).rem_euclid(len);
    Some((&prev.name, &next.name, if gap > 0.0 { (since / gap).clamp(0.0, 1.0) } else { 0.0 }))
}

/// The follower playhead matching a leader at `leader_time`.
/// `follower_time` (its current playhead) picks among repeated marker pairs.
/// A looping follower always lands inside its loop (or its intro).
pub(crate) fn sync_time(follower: &Clip, follower_looping: bool, follower_time: f32, leader: &Clip, leader_looping: bool, leader_time: f32) -> f32 {
    if let Some(t) = marker_time(follower, follower_looping, follower_time, leader, leader_looping, leader_time) {
        return t;
    }
    let (a, b) = loop_span(follower, follower_looping);
    a + phase(leader, leader_looping, leader_time) * (b - a)
}

fn marker_time(follower: &Clip, looping: bool, current: f32, leader: &Clip, leader_looping: bool, leader_time: f32) -> Option<f32> {
    let (pn, nn, frac) = leader_phase(leader, leader_looping, leader_time)?;
    let span = loop_span(follower, looping);
    let cycle = if looping { span.1 - span.0 } else { follower.duration.max(1e-6) };
    let markers = cycle_markers(follower, looping);
    let first = markers.clone().next()?;
    let mut best: Option<(f32, f32)> = None;
    let mut it = markers.peekable();
    while let Some(m) = it.next() {
        let next = it.peek().copied().unwrap_or(first);
        if m.name != pn || next.name != nn {
            continue;
        }
        let (t0, t1) = (m.time, next.time);
        let wraps = t1 <= t0;
        if wraps && !looping {
            continue;
        }
        let len = if wraps { t1 - t0 + cycle } else { t1 - t0 };
        let t = into_loop(follower, looping, (t0 + frac * len).min(if looping { f32::MAX } else { follower.duration }));
        let dist = wrapped_delta(current, t, span, looping).abs();
        if best.is_none_or(|(d, _)| dist < d) {
            best = Some((dist, t));
        }
    }
    best.map(|(_, t)| t)
}
