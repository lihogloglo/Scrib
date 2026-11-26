//! Distance matching curves (animation.md open question 5; UE5's Distance
//! Curve modifier in the Animation Locomotion Library, as Lyra uses it).
//!
//! `anim.cook` adds a per-frame float curve named [`DISTANCE_CURVE`] to a clip
//! whose [`crate::ClipRules::distance`] is set. Its value is the path length of
//! the root's ground (model X, Z) travel, signed by the clip's kind:
//!
//! - [`DistanceCurve::Start`]: distance travelled since frame 0 (0, then
//!   increasing).
//! - [`DistanceCurve::Stop`]: minus the distance left to the stop point (the
//!   end of the last moving step), reaching 0 there and staying 0 after.
//! - [`DistanceCurve::Pivot`]: distance from the pivot frame (the interior frame
//!   of lowest root speed, where the travel direction reverses), negative
//!   before it, 0 at it, positive after.
//!
//! A frame step slower than [`DistanceRules::rest_speed`] counts as standing
//! still and adds no distance, so idle drift before a start, after a stop or
//! around a pivot leaves the curve flat (UE's `StopSpeedThreshold`).

use serde::{Deserialize, Serialize};
use sf_tunables::{Def, Lifetime, Net, Scope, Source, def};

use crate::cook::frame_time;
use crate::{Clip, CurveTrack, RootMotionTrack};

/// Name of the curve `anim.cook` writes.
pub const DISTANCE_CURVE: &str = "distance";

/// The default [`DistanceRules::rest_speed`]: root ground speed below which a
/// frame step counts as standing still.
pub const REST_SPEED_MPS: Def<f32> = def!(float "anim.cook.distance.rest_speed_mps" = 0.05)
    .range(0.0, 1.0)
    .help("Root ground speed, m/s, below which a cooked frame step adds no distance to a start/stop/pivot clip's `distance` curve (UE DistanceCurveModifier StopSpeedThreshold)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Presentation)
    .lifetime(Lifetime::Bake);

/// Which distance a clip's curve measures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceCurve {
    /// Distance travelled from the clip's start.
    Start,
    /// Minus the distance left to where the root comes to rest.
    Stop,
    /// Signed distance from the pivot (lowest-speed) frame.
    Pivot,
}

/// [`crate::ClipRules::distance`]: the curve to cook and its rest threshold.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistanceRules {
    pub curve: DistanceCurve,
    /// Root ground speed below which a frame step adds no distance, m/s
    /// ([`REST_SPEED_MPS`]).
    #[serde(default = "default_rest_speed")]
    pub rest_speed: f32,
}

impl DistanceRules {
    pub fn new(curve: DistanceCurve) -> Self {
        Self { curve, rest_speed: default_rest_speed() }
    }
}

fn default_rest_speed() -> f32 {
    REST_SPEED_MPS.default_value()
}

/// The curve's value at each frame of `track`, whose frames are `dt` seconds
/// apart.
pub fn distance_curve(track: &RootMotionTrack, dt: f32, rules: &DistanceRules) -> Vec<f32> {
    let n = track.frames.len();
    if n == 0 {
        return Vec::new();
    }
    let ground = |f: usize| {
        let [x, _, z, _] = track.frames[f];
        (x, z)
    };
    let steps: Vec<f32> = (1..n)
        .map(|f| {
            let ((x0, z0), (x1, z1)) = (ground(f - 1), ground(f));
            (x1 - x0).hypot(z1 - z0)
        })
        .collect();
    let rest = rules.rest_speed * dt;
    let mut travelled = Vec::with_capacity(n);
    let mut d = 0.0_f32;
    travelled.push(d);
    for &s in &steps {
        if s > rest {
            d += s;
        }
        travelled.push(d);
    }
    let zero = match rules.curve {
        DistanceCurve::Start => 0.0,
        DistanceCurve::Stop => d,
        DistanceCurve::Pivot => travelled[pivot_frame(&steps)],
    };
    travelled.into_iter().map(|t| t - zero).collect()
}

/// Replaces (or adds) `clip`'s [`DISTANCE_CURVE`] by the one `rules` ask for,
/// measured on `clip`'s own root track; nothing for a clip without root
/// motion. `anim.cook` calls it last, and `anim.retarget` (or any retarget
/// path) again on the target's clip: a retargeted clip carries the source's
/// curve, measured on the source body's path, or none when its root motion
/// was generated on the target (`Retargeter::clip_with_root`).
pub fn apply_distance(clip: &mut Clip, rules: &DistanceRules) {
    let Some(track) = &clip.root_motion else {
        return;
    };
    let values = distance_curve(track, frame_time(1, clip.frame_count as usize, clip.duration), rules);
    clip.curves.retain(|c| c.name != DISTANCE_CURVE);
    clip.curves.push(CurveTrack { name: DISTANCE_CURVE.to_owned(), values });
}

/// The interior frame whose mean adjacent step is shortest (the first on
/// ties); frame 0 for clips under three frames.
fn pivot_frame(steps: &[f32]) -> usize {
    (1..steps.len()).min_by(|&a, &b| (steps[a - 1] + steps[a]).total_cmp(&(steps[b - 1] + steps[b]))).unwrap_or(0)
}
