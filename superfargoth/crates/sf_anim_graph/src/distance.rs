//! Distance matching (UE5's): a player's playhead follows its clip's
//! `sf_anim::DISTANCE_CURVE` to a distance parameter instead of the clock
//! ([`crate::asset::DistanceMatchSpec`]; spec `docs/specs/animation.md` open
//! question 5).

/// The first time at which `values` (one per frame, frame `i` at `i / fps`,
/// non-decreasing) reach `v`, linear between frames: 0 at or below the first
/// value, `duration` past the last one.
pub(crate) fn time_at(values: &[f32], fps: f32, duration: f32, v: f32) -> f32 {
    let i = values.partition_point(|&x| x < v);
    if i == 0 || fps <= 0.0 {
        return 0.0;
    }
    if i == values.len() {
        return duration;
    }
    // values[i - 1] < v <= values[i], so the span is positive.
    let (a, b) = (values[i - 1], values[i]);
    // A frame index (far below 2^24) is exact in f32.
    #[allow(clippy::cast_precision_loss)]
    let f = (i - 1) as f32 + (v - a) / (b - a);
    (f / fps).min(duration)
}

/// Clip seconds to advance a playhead at `time` toward the distance `value`:
/// none when the target is not ahead (never backwards), else the way to it
/// held to `rate_clamp` times `normal` (the clock's own advance), and never
/// past the curve's end of motion (the first time it reaches its last value),
/// so a stop comes to rest on its rest frame.
pub(crate) fn matched_delta(values: &[f32], fps: f32, duration: f32, time: f32, value: f32, normal: f32, (lo, hi): (f32, f32)) -> f32 {
    let target = time_at(values, fps, duration, value);
    let want = target - time;
    if want <= 0.0 {
        return 0.0;
    }
    let normal = normal.max(0.0);
    let rest = values.last().map_or(duration, |&last| time_at(values, fps, duration, last));
    let step = want.clamp(normal * lo, normal * hi);
    if time < rest { step.min(rest - time) } else { step }
}
