//! The Nubis density model (Schneider 2015/2017) as ported from
//! Megalophobia's `cloud_density.wgsl`, plus the weather-map lookup.
//!
//! This is the CPU twin of `sf_atmos_bevy/src/shaders/cloud_common.wesl`:
//! change both together. Tests pin the shapes; gameplay can ask "is this
//! point in cloud" with the same answer the renderer draws.

use crate::settings::CloudWeather;

/// Linear remap of `v` from `lo..hi` to `nlo..nhi` (unclamped).
pub fn remap(v: f32, lo: f32, hi: f32, nlo: f32, nhi: f32) -> f32 {
    nlo + (v - lo) * (nhi - nlo) / (hi - lo).max(1e-5)
}

/// `smoothstep` whose edges may run backwards (WGSL leaves that undefined).
pub fn ramp(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Vertical profile: both faces taper, and the shoulder rides higher as the
/// type runs from stratus (0) to cumulus (1): flat bottoms, lumpy tops.
pub fn height_gradient(h: f32, kind: f32) -> f32 {
    let bottom = ramp(0.0, 0.07 + (0.22 - 0.07) * kind, h);
    let top = ramp(1.0, 0.5 + (0.85 - 0.5) * kind, h);
    bottom * top
}

/// Base density from the shape volume (R Perlin-Worley, G Worley FBM),
/// profiled by height, then thresholded by coverage: below it there is no
/// cloud at all, which makes holes instead of soup.
pub fn combine_density(shape_r: f32, shape_g: f32, h: f32, kind: f32, coverage: f32) -> f32 {
    let d = remap(shape_r, shape_g - 1.0, 1.0, 0.0, 1.0) * height_gradient(h, kind);
    remap(d, 1.0 - coverage, 1.0, 0.0, 1.0) * coverage
}

/// The erosion octave: billows at the base, wisps at the top (the same
/// noise, inverted with height).
pub fn erode_density(d: f32, detail: f32, h: f32, erosion: f32) -> f32 {
    let t = (h * 4.0).clamp(0.0, 1.0);
    let erode = detail + (1.0 - detail - detail) * t;
    remap(d, erode * erosion, 1.0, 0.0, 1.0)
}

/// What the weather map says at a point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherSample {
    /// Local coverage, 0..1.
    pub coverage: f32,
    /// Local cloud type, 0..1.
    pub kind: f32,
    /// Local wetness (darkening), 0..1.
    pub wetness: f32,
}

/// Half-width of the coverage threshold's soft edge (in weather-noise units).
pub const COVERAGE_EDGE: f32 = 0.2;

/// Turns the weather-noise map texel (R coverage field, G type field,
/// B storm cells) and the weather scalars into local coverage, type and
/// wetness. The map is fixed noise; only the scalars change with weather,
/// so transitions are smooth and clouds build up where the field is high.
pub fn weather_at(noise: [f32; 4], w: &CloudWeather) -> WeatherSample {
    let [field, type_field, cells, _] = noise;
    // Coverage 0 puts the threshold above the field's range (no cloud),
    // coverage 1 below it (all cloud).
    let threshold = 1.25 - 1.5 * w.coverage.clamp(0.0, 1.0);
    let base = ramp(threshold - COVERAGE_EDGE, threshold + COVERAGE_EDGE, field);
    let storm = w.storm.clamp(0.0, 1.0) * ramp(0.45, 0.85, cells);
    let coverage = base.max(storm);
    let kind = (w.cloud_type + (type_field - 0.5) * 0.4 + storm * 0.5).clamp(0.0, 1.0);
    let wetness = (w.wetness + storm * 0.5).clamp(0.0, 1.0);
    WeatherSample { coverage, kind, wetness }
}

/// Weight of the weather map's second (evolution) octave in the cloud field.
pub const EVOLVE_MIX: f32 = 0.35;

/// The cloud field from the weather map's two drifting octaves (R of each):
/// a blend, contrast-restored so the coverage threshold keeps its meaning
/// (two independent fields blended 0.65/0.35 have ~0.74 of the spread).
pub fn evolve_field(a: f32, b: f32) -> f32 {
    let m = a + (b - a) * EVOLVE_MIX;
    ((m - 0.5) * 1.35 + 0.5).clamp(0.0, 1.0)
}

/// High (cirrus) layer density, 0..1, from its streak field and the
/// weather's `cirrus` cover: a soft threshold that thins to nothing at 0.
pub fn cirrus_at(field: f32, cover: f32) -> f32 {
    let cover = cover.clamp(0.0, 1.0);
    let t = 1.1 - 1.2 * cover;
    ramp(t - 0.25, t + 0.25, field) * (cover * 4.0).min(1.0)
}
