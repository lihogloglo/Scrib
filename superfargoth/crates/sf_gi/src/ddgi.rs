//! DDGI probe atlases (Majercik et al. 2019, "Dynamic Diffuse Global Illumination with
//! Ray-Traced Irradiance Fields", JCGT 8(2), <https://jcgt.org/published/0008/02/01/>), written
//! from the paper (not from RTXGI's SDK).
//!
//! Each probe owns one square tile per atlas: an octahedral irradiance tile
//! ([`IRRADIANCE_TEXELS`] interior texels per side) and an octahedral visibility tile
//! ([`VISIBILITY_TEXELS`]: mean hit distance and mean squared distance), each with a one-texel
//! border that repeats the octahedral map's wrap so bilinear filtering never bleeds between
//! probes. Tiles sit in a 2D grid: probe `(x, y, z)` at tile column `x + dims.x * y`, row `z`.
//!
//! The update shader (`sf_gi_bevy` `probe_update.wesl`) and the sampling shader (vendored
//! `bevy_pbr` `render/gi_probes.wesl`) mirror [`oct_encode`], [`oct_decode`], [`AtlasLayout`],
//! [`border_source`], [`chebyshev`] and the bias and distance constants here.

use glam::{UVec2, UVec3, Vec2, Vec3};

/// Interior texels per side of a probe's irradiance tile (6x6, 8x8 with its border).
pub const IRRADIANCE_TEXELS: u32 = 6;
/// Interior texels per side of a probe's visibility tile (14x14, 16x16 with its border).
pub const VISIBILITY_TEXELS: u32 = 14;
/// Exponent of the visibility texels' cosine lobe (the paper's "depth sharpness").
pub const DEPTH_SHARPNESS: f32 = 50.0;
/// The paper's self-shadow bias factor (scaled by 0.75 x the probe spacing).
pub const SELF_SHADOW_BIAS: f32 = 0.3;

/// `±1` with the sign of `v`, `+1` for zero (the paper's `signNotZero`).
fn sign_not_zero(v: f32) -> f32 {
    if v >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// Octahedral encoding of a unit direction to `[-1, 1]^2` (Cigolle et al. 2014, as DDGI):
/// the upper (`+z`) hemisphere fills the inner diamond, the lower folds into the corners.
pub fn oct_encode(n: Vec3) -> Vec2 {
    let p = Vec2::new(n.x, n.y) / (n.x.abs() + n.y.abs() + n.z.abs());
    if n.z < 0.0 {
        Vec2::new((1.0 - p.y.abs()) * sign_not_zero(p.x), (1.0 - p.x.abs()) * sign_not_zero(p.y))
    } else {
        p
    }
}

/// Inverse of [`oct_encode`]: a point of `[-1, 1]^2` to a unit direction.
pub fn oct_decode(p: Vec2) -> Vec3 {
    let mut n = Vec3::new(p.x, p.y, 1.0 - p.x.abs() - p.y.abs());
    if n.z < 0.0 {
        let (x, y) = (n.x, n.y);
        n.x = (1.0 - y.abs()) * sign_not_zero(x);
        n.y = (1.0 - x.abs()) * sign_not_zero(y);
    }
    n.normalize()
}

/// The direction at the centre of interior texel `texel` (0-based) of an `inner` x `inner` tile.
pub fn oct_texel_dir(texel: UVec2, inner: u32) -> Vec3 {
    #[allow(clippy::cast_precision_loss)]
    let p = (texel.as_vec2() + 0.5) / inner as f32 * 2.0 - 1.0;
    oct_decode(p)
}

/// For texel `texel` of a bordered tile (`inner + 2` per side, 0-based), the interior texel
/// (same coordinates) whose value it repeats, or `None` for an interior texel. Edges mirror
/// along the edge (the octahedral map folds each edge onto itself); each corner takes the
/// opposite interior corner (all four corners are the `-z` pole).
pub fn border_source(texel: UVec2, inner: u32) -> Option<UVec2> {
    let last = inner + 1;
    let (x, y) = (texel.x, texel.y);
    let (on_x, on_y) = (x == 0 || x == last, y == 0 || y == last);
    let mirror = |v: u32| last - v;
    match (on_x, on_y) {
        (false, false) => None,
        (true, true) => Some(UVec2::new(if x == 0 { inner } else { 1 }, if y == 0 { inner } else { 1 })),
        // Top or bottom edge: the row inside, mirrored column.
        (false, true) => Some(UVec2::new(mirror(x), if y == 0 { 1 } else { inner })),
        // Left or right edge: the column inside, mirrored row.
        (true, false) => Some(UVec2::new(if x == 0 { 1 } else { inner }, mirror(y))),
    }
}

/// A probe atlas: one bordered `inner + 2` square tile per probe of a `dims` grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtlasLayout {
    pub dims: UVec3,
    /// Interior texels per tile side.
    pub inner: u32,
}

impl AtlasLayout {
    pub fn irradiance(dims: UVec3) -> Self {
        Self { dims, inner: IRRADIANCE_TEXELS }
    }

    pub fn visibility(dims: UVec3) -> Self {
        Self { dims, inner: VISIBILITY_TEXELS }
    }

    /// Texels per tile side, border included.
    pub fn tile_size(&self) -> u32 {
        self.inner + 2
    }

    /// Tiles per row and column: `(dims.x * dims.y, dims.z)`.
    pub fn tiles(&self) -> UVec2 {
        UVec2::new(self.dims.x * self.dims.y, self.dims.z)
    }

    /// The atlas texture's size in texels.
    pub fn size(&self) -> UVec2 {
        self.tiles() * self.tile_size()
    }

    /// Tile of probe `coord`.
    pub fn tile(&self, coord: UVec3) -> UVec2 {
        UVec2::new(coord.x + self.dims.x * coord.y, coord.z)
    }

    /// Probe of tile `tile` (inverse of [`Self::tile`]).
    pub fn probe(&self, tile: UVec2) -> UVec3 {
        UVec3::new(tile.x % self.dims.x, tile.x / self.dims.x, tile.y)
    }

    /// Atlas texel of bordered-tile texel `local` (0..`tile_size`) of probe `coord`.
    pub fn texel(&self, coord: UVec3, local: UVec2) -> UVec2 {
        self.tile(coord) * self.tile_size() + local
    }

    /// Normalised atlas UV of octahedral point `oct` (`[-1, 1]^2`) in probe `coord`'s tile:
    /// the interior spans texels `1..=inner`, so bilinear taps at its edge read the border.
    pub fn uv(&self, coord: UVec3, oct: Vec2) -> Vec2 {
        #[allow(clippy::cast_precision_loss)]
        let inner = self.inner as f32;
        let texel = self.tile(coord).as_vec2() * (inner + 2.0) + 1.0 + (oct * 0.5 + 0.5) * inner;
        texel / self.size().as_vec2()
    }
}

/// Visibility rays and misses are clamped to this distance: 1.5 x the probe cell's diagonal.
pub fn max_ray_distance(spacing: f32) -> f32 {
    1.5 * 3f32.sqrt() * spacing
}

/// Length of the paper's self-shadow bias (`0.75 x spacing x` [`SELF_SHADOW_BIAS`]), applied
/// along `0.2 N + 0.8 V` (normal and view direction) before the probe lookup.
pub fn self_shadow_bias(spacing: f32) -> f32 {
    0.75 * spacing * SELF_SHADOW_BIAS
}

/// The paper's Chebyshev visibility weight of a point `distance` from the probe, given the
/// probe's mean distance and mean squared distance in that direction: 1 when nearer than the
/// mean, else `(variance / (variance + (d - mean)^2))^3`.
pub fn chebyshev(mean: f32, mean_sq: f32, distance: f32) -> f32 {
    if distance <= mean {
        return 1.0;
    }
    let variance = (mean * mean - mean_sq).abs();
    let d = distance - mean;
    let w = variance / (variance + d * d);
    (w * w * w).max(0.0)
}

/// Weight of a ray along `ray` in the irradiance texel along `texel` (cosine) and in the
/// visibility texel (cosine to the power [`DEPTH_SHARPNESS`]).
pub fn texel_weights(texel: Vec3, ray: Vec3) -> (f32, f32) {
    let c = texel.dot(ray).max(0.0);
    (c, c.powf(DEPTH_SHARPNESS))
}

/// Cosine-weighted mean radiance about `texel` from `(unit direction, radiance)` samples:
/// irradiance / pi, the value Bevy multiplies by the diffuse colour. 0 where no sample faces it.
pub fn irradiance_texel(texel: Vec3, samples: &[(Vec3, Vec3)]) -> Vec3 {
    let (mut sum, mut weight) = (Vec3::ZERO, 0.0);
    for &(dir, radiance) in samples {
        let w = texel_weights(texel, dir).0;
        sum += radiance * w;
        weight += w;
    }
    if weight > 0.0 {
        sum / weight
    } else {
        Vec3::ZERO
    }
}

/// Exponential history: `hysteresis * previous + (1 - hysteresis) * new`.
pub fn hysteresis_blend(new: Vec3, previous: Vec3, hysteresis: f32) -> Vec3 {
    previous.lerp(new, 1.0 - hysteresis)
}
