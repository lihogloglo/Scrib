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

// Storm cloud shape (S3, `cbshape`; clouds.md "Storm cloud shape"). Every
// term is gated by the local storm scalar `cb` (0: the old functions,
// bit for bit), so fair, overcast and other decks never change.

/// Lowest storm top (height fraction) at vanishing coverage: rim towers.
pub const CB_TOP_MIN: f32 = 0.35;
/// Top height `mix(CB_TOP_MIN, 1, coverage^CB_TOP_EXP)`: > 1 keeps the
/// towers narrow (the top falls fast off the core).
pub const CB_TOP_EXP: f32 = 1.5;
/// Storm-cell thresholds slide up by this × (1 − storm): toward the region
/// edge cells shrink to their cores instead of fading whole.
pub const CB_CELL_SHIFT: f32 = 0.8;
/// The anvil's cell term thresholds this much lower than the cell's: the
/// overhang beyond the tower (cell noise units).
pub const CB_ANVIL_WIDEN: f32 = 0.15;
/// The anvil layer rises from here over `CB_ANVIL_RISE` (height fraction).
pub const CB_ANVIL_LO: f32 = 0.72;
pub const CB_ANVIL_RISE: f32 = 0.08;
/// Coverage the anvil layer lends (× anvil term) where the tower has none.
pub const CB_ANVIL_COVER: f32 = 0.6;
/// The ragged base: below this height fraction erosion turns wispy ...
pub const CB_RAG_H: f32 = 0.1;
/// ... and erodes up to (1 + this) × harder.
pub const CB_RAG_BOOST: f32 = 1.0;

/// The shape volume's vertical coordinate (before the `rise` phase): the
/// height fraction, or with `cb` world altitude in shape tiles, so a tall
/// deck's features stay as tall as they are wide (UE5/Frostbite practice).
pub fn shape_height(h: f32, thickness: f32, shape_tile: f32, cb: f32) -> f32 {
    if cb <= 0.0 {
        return h;
    }
    h * (1.0 + cb * (thickness / shape_tile.max(1.0) - 1.0))
}

/// The storm-cloud profile (`cb` > 0): a tower whose top follows coverage
/// (domed cells, tapering at the rims) plus an anvil layer near the top
/// where the wide cell term `anvil` reaches. `cb` 0: [`height_gradient`].
pub fn storm_gradient(h: f32, kind: f32, coverage: f32, cb: f32, anvil: f32) -> f32 {
    if cb <= 0.0 {
        return height_gradient(h, kind);
    }
    let top_h = 1.0 - cb * (1.0 - (CB_TOP_MIN + (1.0 - CB_TOP_MIN) * coverage.clamp(0.0, 1.0).powf(CB_TOP_EXP)));
    let bottom = ramp(0.0, 0.07 + (0.22 - 0.07) * kind, h);
    let top = ramp(top_h, top_h * (0.5 + (0.85 - 0.5) * kind), h);
    (bottom * top).max(anvil_lift(h, cb, anvil))
}

/// The anvil layer's weight at `h`: rises above `CB_ANVIL_LO`, fades into
/// the deck top.
fn anvil_lift(h: f32, cb: f32, anvil: f32) -> f32 {
    cb * anvil * ramp(CB_ANVIL_LO, CB_ANVIL_LO + CB_ANVIL_RISE, h) * ramp(1.0, 0.92, h)
}

/// [`combine_density`] with the storm shape: the storm profile, the anvil's
/// lent coverage and the Nubis anvil bias (Schneider 2017:
/// `pow(cov, remap(h, 0.7, 0.8, 1, mix(1, 0.5, anvil)))`).
pub fn combine_density_shaped(shape_r: f32, shape_g: f32, h: f32, kind: f32, coverage: f32, cb: f32, anvil: f32) -> f32 {
    if cb <= 0.0 {
        return combine_density(shape_r, shape_g, h, kind, coverage);
    }
    let d = remap(shape_r, shape_g - 1.0, 1.0, 0.0, 1.0) * storm_gradient(h, kind, coverage, cb, anvil);
    let bias = remap(h.clamp(0.7, 0.8), 0.7, 0.8, 1.0, 1.0 - 0.5 * anvil * cb);
    let cov = coverage.max(anvil_lift(h, cb, anvil) * CB_ANVIL_COVER).clamp(0.0, 1.0).powf(bias);
    remap(d, 1.0 - cov, 1.0, 0.0, 1.0) * cov
}

/// [`erode_density`] with the ragged storm base: below `CB_RAG_H` the
/// inverted (wispy) detail erodes up to `1 + CB_RAG_BOOST` × harder.
pub fn erode_density_shaped(d: f32, detail: f32, h: f32, erosion: f32, cb: f32) -> f32 {
    if cb <= 0.0 {
        return erode_density(d, detail, h, erosion);
    }
    let t = (h * 4.0).clamp(0.0, 1.0);
    let rag = cb * ramp(CB_RAG_H, 0.0, h);
    let billow = detail + (1.0 - detail - detail) * t;
    let erode = billow + (1.0 - detail - billow) * rag;
    remap(d, erode * erosion * (1.0 + CB_RAG_BOOST * rag), 1.0, 0.0, 1.0)
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
    /// The storm-shape gate `cb` (S3): the local storm scalar, 0 without the
    /// storm shape.
    pub storm: f32,
    /// The anvil's wide cell term, 0..1 (0 without the storm shape).
    pub anvil: f32,
}

/// Half-width of the coverage threshold's soft edge (in weather-noise units).
pub const COVERAGE_EDGE: f32 = 0.2;

/// Turns the weather-noise map texel (R coverage field, G type field,
/// B storm cells) and the weather scalars into local coverage, type and
/// wetness. The map is fixed noise; only the scalars change with weather,
/// so transitions are smooth and clouds build up where the field is high.
pub fn weather_at(noise: [f32; 4], w: &CloudWeather) -> WeatherSample {
    weather_at_shaped(noise, w, false)
}

/// [`weather_at`], with the storm shape (S3) when `shaped`: storm cells
/// shrink by threshold toward a weaker storm (equal to the old cells at
/// storm 0 and 1), and the sample carries the gate and the anvil term.
pub fn weather_at_shaped(noise: [f32; 4], w: &CloudWeather, shaped: bool) -> WeatherSample {
    let [field, type_field, cells, _] = noise;
    // Coverage 0 puts the threshold above the field's range (no cloud),
    // coverage 1 below it (all cloud).
    let threshold = 1.25 - 1.5 * w.coverage.clamp(0.0, 1.0);
    let base = ramp(threshold - COVERAGE_EDGE, threshold + COVERAGE_EDGE, field);
    let s = w.storm.clamp(0.0, 1.0);
    let (storm, cb, anvil) = if shaped {
        let lo = 0.45 + CB_CELL_SHIFT * (1.0 - s);
        (ramp(lo, lo + 0.4, cells), s, ramp(lo - CB_ANVIL_WIDEN, lo + 0.4 - CB_ANVIL_WIDEN, cells))
    } else {
        (s * ramp(0.45, 0.85, cells), 0.0, 0.0)
    };
    let coverage = base.max(storm);
    let kind = (w.cloud_type + (type_field - 0.5) * 0.4 + storm * 0.5).clamp(0.0, 1.0);
    let wetness = (w.wetness + storm * 0.5).clamp(0.0, 1.0);
    WeatherSample { coverage, kind, wetness, storm: cb, anvil }
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
