//! Cloud shadow map amortization (weather.md "Cloud shadows"): which rows of
//! the map to re-render this frame.
//!
//! The map is **toroidal**: its texels are anchored to the light plane (the
//! light texture repeats, and its transform sits at a world anchor that moves
//! only in big steps), so a texel keeps meaning the same column of sky while
//! the camera moves. That lets the map refresh one band of rows per frame
//! (time slicing, as for reflection probes and far shadow cascades): the
//! clouds move a few meters per frame against texels of tens of meters, so a
//! band that is a few frames old is indistinguishable from a fresh one.
//!
//! A full refresh happens when the slices would disagree visibly: the anchor
//! moved, the light turned more than [`MAX_DRIFT_M`] at the camera since the
//! oldest slice was written (a time-of-day jump), or the weather jumped (a
//! preset switch, not a gradual blend).

use glam::Vec3;

use crate::CloudWeather;

/// How far (meters, at the camera) the shadow pattern may drift between the
/// oldest and the newest slice before the whole map is redrawn.
pub const MAX_DRIFT_M: f32 = 4.0;

/// Rows of the shadow map to render this frame: `start..start + count`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowRows {
    pub start: u32,
    pub count: u32,
}

/// Time-slices the cloud shadow map. One per shadow map.
#[derive(Clone, Debug, Default)]
pub struct ShadowSlicer {
    anchor: Option<[i64; 2]>,
    weather: Option<CloudWeather>,
    /// Light direction each slice was last rendered with.
    slices: Vec<Vec3>,
    next: usize,
}

impl ShadowSlicer {
    /// Forget the map's content (shadows were off, the map was recreated).
    pub fn invalidate(&mut self) {
        *self = Self::default();
    }

    /// The rows to render this frame for a `size`² map cut into `slices`
    /// bands. `anchor` is the light texture's world anchor cell, `light_dir`
    /// the unit direction toward the light, `lever` the distance (meters)
    /// from the anchor to the camera plus a near-field margin: a turn of the
    /// light by θ moves the pattern at the camera by about `lever · θ`.
    pub fn rows(&mut self, size: u32, slices: u32, anchor: [i64; 2], light_dir: Vec3, weather: &CloudWeather, lever: f32) -> ShadowRows {
        let n = slices.clamp(1, size.max(1)) as usize;
        let full = ShadowRows { start: 0, count: size };
        let jumped = self.weather.is_some_and(|w| weather_jumped(&w, weather));
        self.weather = Some(*weather);
        let drifted = self.slices.iter().any(|d| d.angle_between(light_dir) * lever > MAX_DRIFT_M);
        if self.anchor != Some(anchor) || jumped || drifted || self.slices.len() != n {
            self.anchor = Some(anchor);
            self.slices = vec![light_dir; n];
            self.next = 0;
            return full;
        }
        let k = self.next;
        self.slices[k] = light_dir;
        self.next = (k + 1) % n;
        // Bands of near-equal height that cover every row exactly once per cycle.
        #[allow(clippy::cast_possible_truncation)]
        let edge = |i: usize| (u64::from(size) * i as u64 / n as u64) as u32;
        ShadowRows { start: edge(k), count: edge(k + 1) - edge(k) }
    }
}

/// A change of the weather too big for one frame of a gradual blend (a
/// preset switch): the slices would show two different skies.
fn weather_jumped(a: &CloudWeather, b: &CloudWeather) -> bool {
    let rel = |x: f32, y: f32| (x - y).abs() > 0.05 * x.abs().max(y.abs()).max(1e-3);
    (a.coverage - b.coverage).abs() > 0.02
        || (a.cloud_type - b.cloud_type).abs() > 0.05
        || (a.erosion - b.erosion).abs() > 0.05
        || (a.cirrus - b.cirrus).abs() > 0.05
        || (a.storm - b.storm).abs() > 0.05
        || rel(a.density, b.density)
        || rel(a.base, b.base)
        || rel(a.thickness, b.thickness)
}
