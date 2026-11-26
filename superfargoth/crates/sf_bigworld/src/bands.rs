//! Tier bands by screen size (spec `superfargoth/docs/specs/big-world-rendering.md` "Tiers"): which representation an
//! instance gets from its size on screen at the internal render resolution.
//!
//! - T0 mesh while the tree's height is at least `bigworld.mesh.switch_px` pixels tall.
//! - T1 impostor while its crown is at least `bigworld.aggregate.switch_px` pixels wide.
//! - T2 tile aggregate beyond: per tile, not per instance, so only its lead distance is computed here.
//!
//! Neighbouring tiers overlap in a crossfade band (`bigworld.fade_band`, a share of the switch distance): inside it an
//! instance is written to both tiers with complementary fades, and the renderer dithers them, so there is never a
//! hole. Past `aggregate_lead_m` the T2 tile is drawn fully underneath the fading impostors.
//!
//! [`Bands::fades`] is the CPU twin of the GPU producer's compute shader: plain f32 maths in saturate / min / max
//! style that ports to WGSL one-to-one (`select` for the hard-switch case), no f64, no tables.
//!
//! Degenerate inputs never give NaN (they are clamped the way `max(x, 0)` / `min(x, f32::MAX)` do):
//! - a distance that is negative or NaN counts as at the viewer (0: full mesh); `+inf` is beyond every tier;
//! - a scale that is negative or NaN is 0 (nothing drawn); `+inf` clamps to `f32::MAX`;
//! - a size (height, crown) that is negative or NaN is 0 (that tier ends at the viewer);
//! - a switch size of 0 px, negative or NaN never switches (the tier reaches `f32::MAX`);
//! - a fade band outside [0, 1] is clamped into it, NaN is 0 (hard switch).

/// Pixels per radian of a view at its internal render resolution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewScale {
    pub px_per_rad: f32,
}

impl ViewScale {
    /// `ppr = H / (2 tan(fov_y / 2))`. A field of view outside (0, π) gives a NaN-free but meaningless scale.
    #[must_use]
    pub fn new(internal_height_px: f32, fov_y_rad: f32) -> Self {
        Self { px_per_rad: over(internal_height_px, 2.0 * (0.5 * fov_y_rad).tan()) }
    }

    /// Size on screen in pixels of `size_m` metres seen at `dist_m`: `size · ppr / d` (at or behind the viewer: inf).
    #[must_use]
    pub fn px(self, size_m: f32, dist_m: f32) -> f32 {
        over(size_m * self.px_per_rad, dist_m)
    }

    /// Distance at which `size_m` metres are `px` pixels on screen: `size · ppr / px` (0 px or less: inf).
    #[must_use]
    pub fn distance(self, size_m: f32, px: f32) -> f32 {
        over(size_m * self.px_per_rad, px)
    }
}

/// What the bands need to know of a species, at scale 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpeciesDims {
    pub height_m: f32,
    pub crown_m: f32,
}

/// The tunables the bands read (`crate::tunables`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BandSettings {
    pub mesh_switch_px: f32,
    pub aggregate_switch_px: f32,
    pub fade_band: f32,
}

impl Default for BandSettings {
    fn default() -> Self {
        Self {
            mesh_switch_px: crate::tunables::MESH_SWITCH_PX.default_value(),
            aggregate_switch_px: crate::tunables::AGGREGATE_SWITCH_PX.default_value(),
            fade_band: crate::tunables::FADE_BAND.default_value(),
        }
    }
}

/// Switch distances of one species at scale 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bands {
    /// Where the mesh has faded out completely (its height is `mesh_switch_px` tall here).
    pub mesh_end_m: f32,
    /// Where the impostor has faded out completely (its crown is `aggregate_switch_px` wide here); never below
    /// `mesh_end_m`, so a species whose crown rule ends first goes straight from mesh to aggregate.
    pub impostor_end_m: f32,
    /// Share of each end distance over which the tiers crossfade, in [0, 1].
    pub fade_band: f32,
}

/// How much of each per-instance tier an instance shows: 0 = not written, 1 = fully drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TierFades {
    pub mesh: f32,
    pub impostor: f32,
}

impl Bands {
    /// `mesh_end_m = height · ppr / mesh_switch_px`, `impostor_end_m = max(crown · ppr / aggregate_switch_px,
    /// mesh_end_m)`, both clamped to [0, `f32::MAX`].
    #[must_use]
    pub fn new(dims: SpeciesDims, view: ViewScale, settings: BandSettings) -> Self {
        let mesh_end_m = finite_size(view.distance(dims.height_m, settings.mesh_switch_px));
        let impostor_end_m = finite_size(view.distance(dims.crown_m, settings.aggregate_switch_px)).max(mesh_end_m);
        Self { mesh_end_m, impostor_end_m, fade_band: unit(settings.fade_band) }
    }

    /// Fades of an instance of `scale` at `dist_m` from the viewer. With `m = mesh_end_m · scale`,
    /// `i = impostor_end_m · scale`, `b = fade_band`:
    /// - mesh is 1 up to `m(1 − b)`, falls linearly to 0 at `m`, 0 beyond;
    /// - impostor is `1 − mesh` up to the far band (so mesh + impostor = 1 in the first band), 1 until `i(1 − b)`,
    ///   falls linearly to 0 at `i`, 0 beyond: `impostor = min(1 − mesh, far ramp)`;
    /// - `b = 0` is a hard switch: `d < m` mesh, `m ≤ d < i` impostor, `d ≥ i` neither.
    ///
    /// When the bands touch (`i = m`, or the far band starts inside the first one) the impostor is capped by its own
    /// fade-out, so mesh + impostor drops below 1 past `i(1 − b)`; that is where the T2 aggregate is already fully
    /// drawn underneath ([`Self::aggregate_lead_m`]), so there is still no hole. Before `i(1 − b)` the sum is 1.
    #[must_use]
    pub fn fades(&self, dist_m: f32, scale: f32) -> TierFades {
        let dist = dist_m.max(0.0);
        let scale = finite_size(scale);
        let band = unit(self.fade_band);
        let mesh_end = finite_size(self.mesh_end_m * scale);
        let impostor_end = finite_size(self.impostor_end_m * scale);
        let mesh = ramp_down(dist, mesh_end, mesh_end * band);
        let impostor = (1.0 - mesh).min(ramp_down(dist, impostor_end, impostor_end * band));
        TierFades { mesh, impostor }
    }

    /// Distance from which the T2 tile aggregate must already be fully drawn for an instance of `scale`: `i(1 − b)`,
    /// where its impostor starts fading out.
    #[must_use]
    pub fn aggregate_lead_m(&self, scale: f32) -> f32 {
        finite_size(self.impostor_end_m * finite_size(scale)) * (1.0 - unit(self.fade_band))
    }
}

/// Clamp into [0, `f32::MAX`]; NaN becomes 0. `f32::clamp` would pass NaN through, `max` returns the non-NaN operand.
#[allow(clippy::manual_clamp, reason = "clamp keeps NaN; max/min maps it to 0, as WGSL max/min do")]
fn finite_size(x: f32) -> f32 {
    x.max(0.0).min(f32::MAX)
}

/// `num / den` for sizes: a negative or NaN numerator is 0 (and gives 0), a denominator that is not positive gives
/// inf; never NaN (the numerator is capped at `f32::MAX` so `inf / inf` cannot happen).
fn over(num: f32, den: f32) -> f32 {
    let num = finite_size(num);
    if num <= 0.0 {
        0.0
    } else if den > 0.0 {
        num / den
    } else {
        f32::INFINITY
    }
}

/// Clamp into [0, 1]; NaN becomes 0 (`f32::max` returns the non-NaN operand).
#[allow(clippy::manual_clamp, reason = "clamp keeps NaN; max/min maps it to 0, as WGSL saturate does")]
fn unit(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

/// 1 before `end − width`, linear down to 0 at `end`, 0 beyond. A zero width is a step (`d < end`), the WGSL
/// `select(f32(d < end), saturate((end - d) / width), width > 0)`.
fn ramp_down(d: f32, end: f32, width: f32) -> f32 {
    if width > 0.0 {
        unit((end - d) / width)
    } else if d < end {
        1.0
    } else {
        0.0
    }
}
