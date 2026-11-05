//! Albedo de-lighting: an estimate of the light already painted into a texture, divided back out.
//!
//! Textures drawn for renderers without bounce light carry baked shading (a wall darker towards
//! its foot, a glow round a lamp). A renderer with GI lights them again and the painted light shows
//! twice. The estimate is the texture's low-frequency luminance on a coarse grid, normalised to
//! mean one, so dividing by it moves light from where the texture had it to where it did not and
//! leaves the total alone (Land and `McCann`'s Retinex smoothness assumption: it cannot tell a slow
//! albedo change from light, and it lets through occlusion or rims a few texels wide).
//!
//! 1. [`shading_grid`]: per texel, linear Rec. 709 luminance weighted by alpha (a hole in a cutout
//!    weighs nothing), summed into an [`EXTENT`]² grid by the cell the texel's own position falls
//!    in; each cell is its weighted mean; empty cells (a texture smaller than the grid) take the
//!    mean of the sampled ones; no sampled cell gives the neutral grid. [`BLUR_PASSES`] 3x3 box
//!    blurs, wrapping on an axis the texture repeats and clamping on one it clamps. Normalised to
//!    mean one (a black texture stays neutral), then clamped to [`FLOOR`]..[`CEILING`].
//! 2. [`delight`]: per texel, the grid's bilinear factor at the texel centre, mixed towards one by
//!    the strength, divides the linear colour; the result is scaled down as a whole (hue kept) so
//!    its brightest channel is at most one, then re-encoded sRGB. Alpha is untouched.
//!
//! # Provenance
//!
//! Ported from openmw-rtx (<https://github.com/xorza/openmw-rtx>, GPL-3.0, as upstream `OpenMW`),
//! commit `f2f8b2d5b2cb142d1b454b3bb1815f4552980562`:
//! `components/rtx/image/shadingmap.hpp` / `.cpp` (host `ShadingMap`, `paintedLight`),
//! `components/rtxvulkan/shaders/texture/shadingsum.comp` and `shadingmap.comp` (the device
//! version the game runs, authoritative here), `components/rtx/shaders/shadingmap.h`
//! (`SHADING_BLUR_PASSES`), `components/rtx/shaders/look.h` (`SHADING_FLOOR` / `SHADING_CEILING`),
//! `components/rtx/shaders/scene.h` (`SHADING_EXTENT`), `components/rtx/shaders/colour.h`
//! (`LUMINANCE_WEIGHTS`, `brightest`), `components/rtxvulkan/shaders/lib/ground.glsl`
//! (`delitTexel`). Tests port `apps/components_tests/rtx/image/shadingmap.cpp`.
//! Notice: `superfargoth/THIRD_PARTY_NOTICES.md`.
//!
//! Changes from upstream:
//! - Input is decoded RGBA8, so every texel goes to its own cell, as the device's `shadingsum.comp`
//!   does (the host sums BC1 by block, which differs only below 128 px).
//! - Cell sums accumulate in `f64` (the device: `f32` fixed-tree reductions); the grid stays `f32`
//!   and is not quantised to the device's R16 unorm, because nothing stores it: the bake writes
//!   de-lit texels instead of dividing in the shader.
//! - The lookup clamps on an axis the texture clamps (upstream's sampler always wraps), matching
//!   the clamped blur, whose reason upstream gives: a clamped picture's border must not take the
//!   opposite edge's light.

use std::sync::OnceLock;

/// Cells along each edge of the grid (`SHADING_EXTENT`). Coarse on purpose: painted lighting
/// varies slowly, painted detail does not. Three blurs reach about 11 texels at 256 px.
pub const EXTENT: usize = 32;
/// Cells in the grid, row by row.
pub const CELLS: usize = EXTENT * EXTENT;
/// Lowest factor the grid may hold (`SHADING_FLOOR`): a cell can be brightened at most twofold.
pub const FLOOR: f32 = 0.5;
/// Highest factor the grid may hold (`SHADING_CEILING`): a cell can be darkened at most twofold.
pub const CEILING: f32 = 2.0;
/// 3x3 box-blur passes over the grid (`SHADING_BLUR_PASSES`), a close enough Gaussian.
pub const BLUR_PASSES: usize = 3;
/// Rec. 709 luminance weights, applied in linear light.
pub const LUMINANCE_WEIGHTS: [f32; 3] = [0.2126, 0.7152, 0.0722];
/// Default de-lighting strength (owner decision 2026-11-05, `DECISIONS.md`); a bake profile setting.
pub const DEFAULT_STRENGTH: f32 = 0.5;

/// A texture's painted-light estimate: [`CELLS`] factors averaging one, row by row (`v` down).
#[derive(Clone, Debug, PartialEq)]
pub struct ShadingGrid {
    values: Box<[f32; CELLS]>,
    wrap_u: bool,
    wrap_v: bool,
}

impl ShadingGrid {
    /// The grid that changes nothing: every factor one.
    pub fn neutral(wrap_u: bool, wrap_v: bool) -> Self {
        Self {
            values: Box::new([1.0; CELLS]),
            wrap_u,
            wrap_v,
        }
    }

    /// The factors, row by row.
    pub fn values(&self) -> &[f32; CELLS] {
        &self.values
    }

    /// Whether the texture repeats along `u` (else it clamps).
    pub fn wrap_u(&self) -> bool {
        self.wrap_u
    }

    /// Whether the texture repeats along `v` (else it clamps).
    pub fn wrap_v(&self) -> bool {
        self.wrap_v
    }

    /// Whether every factor is exactly one, so [`delight`] would leave the texture as it is.
    #[allow(
        clippy::float_cmp,
        reason = "neutral means exactly one, the value `neutral` writes"
    )]
    pub fn is_neutral(&self) -> bool {
        self.values.iter().all(|&value| value == 1.0)
    }

    /// The factor at `(u, v)` in texture space, bilinear between cell centres, wrapping on an axis
    /// the texture repeats and held at the edge on one it clamps (upstream `paintedLight`).
    pub fn factor_at(&self, u: f32, v: f32) -> f32 {
        let (left, right, across) = axis_taps(u, self.wrap_u);
        let (top, bottom, down) = axis_taps(v, self.wrap_v);
        self.bilinear(
            Taps {
                low: left,
                high: right,
                t: across,
            },
            Taps {
                low: top,
                high: bottom,
                t: down,
            },
        )
    }

    fn bilinear(&self, x: Taps, y: Taps) -> f32 {
        let cell = |column: usize, row: usize| self.values[row * EXTENT + column];
        let upper = lerp(cell(x.low, y.low), cell(x.high, y.low), x.t);
        let lower = lerp(cell(x.low, y.high), cell(x.high, y.high), x.t);
        lerp(upper, lower, y.t)
    }
}

/// The two cells along one axis a lookup blends, and how far between them it is.
#[derive(Clone, Copy)]
struct Taps {
    low: usize,
    high: usize,
    t: f32,
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "grid coordinates are small: floor of a value in -0.5..EXTENT"
)]
fn axis_taps(coordinate: f32, wrap: bool) -> (usize, usize, f32) {
    let place = if wrap {
        coordinate - coordinate.floor()
    } else {
        coordinate.clamp(0.0, 1.0)
    };
    // Cell centres sit at (i + 0.5) / EXTENT; half a cell back puts the first one at zero.
    let at = place * EXTENT as f32 - 0.5;
    let low = at.floor();
    let t = at - low;
    let low = low as i64;
    let extent = EXTENT as i64;
    let index = |cell: i64| {
        if wrap {
            cell.rem_euclid(extent) as usize
        } else {
            cell.clamp(0, extent - 1) as usize
        }
    };
    (index(low), index(low + 1), t)
}

/// Lookup tables for the sRGB transfer function on bytes.
pub(crate) struct Srgb {
    /// Byte to linear.
    pub(crate) decode: [f32; 256],
    /// `decode` of each half step `(k + 0.5) / 255`: a linear value encodes to the number of
    /// thresholds at or below it, which is rounding in encoded space and maps `decode[k]` back to
    /// `k` exactly.
    thresholds: [f32; 255],
}

impl Srgb {
    pub(crate) fn get() -> &'static Self {
        static TABLES: OnceLock<Srgb> = OnceLock::new();
        TABLES.get_or_init(|| {
            let curve = |encoded: f64| {
                if encoded <= 0.04045 {
                    encoded / 12.92
                } else {
                    ((encoded + 0.055) / 1.055).powf(2.4)
                }
            };
            let mut decode = [0.0; 256];
            for (byte, linear) in decode.iter_mut().enumerate() {
                *linear = to_f32(curve(
                    f64::from(u8::try_from(byte).unwrap_or(u8::MAX)) / 255.0,
                ));
            }
            let mut thresholds = [0.0; 255];
            for (step, threshold) in thresholds.iter_mut().enumerate() {
                let half = f64::from(u8::try_from(step).unwrap_or(u8::MAX)) + 0.5;
                *threshold = to_f32(curve(half / 255.0));
            }
            Srgb { decode, thresholds }
        })
    }

    pub(crate) fn encode(&self, linear: f32) -> u8 {
        u8::try_from(
            self.thresholds
                .partition_point(|&threshold| threshold <= linear),
        )
        .unwrap_or(u8::MAX)
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "f64 to f32 by design: tables and grid are f32"
)]
fn to_f32(value: f64) -> f32 {
    value as f32
}

#[allow(
    clippy::cast_precision_loss,
    reason = "texture sizes are far below 2^24"
)]
fn usize_to_f32(value: usize) -> f32 {
    value as f32
}

pub(crate) fn check_len(rgba: &[u8], width: u32, height: u32) -> (usize, usize) {
    let width = width as usize;
    let height = height as usize;
    assert_eq!(
        rgba.len(),
        width * height * 4,
        "rgba holds {width}x{height} RGBA8 texels"
    );
    (width, height)
}

/// Estimates the light painted into a texture.
///
/// `rgba` is the texture's largest level, decoded RGBA8 with sRGB-encoded colour, row by row.
/// `wrap_u` / `wrap_v`: whether the texture repeats along that axis (else it clamps). A texture
/// with no area, or with every texel transparent, or black everywhere gets the neutral grid.
///
/// # Panics
/// If `rgba.len() != width * height * 4`.
pub fn shading_grid(
    rgba: &[u8],
    width: u32,
    height: u32,
    wrap_u: bool,
    wrap_v: bool,
) -> ShadingGrid {
    let (width, height) = check_len(rgba, width, height);
    let mut grid = ShadingGrid::neutral(wrap_u, wrap_v);
    if width == 0 || height == 0 {
        return grid;
    }
    let srgb = Srgb::get();

    // Each texel goes to the cell its own position lands in (`shadingsum.comp`'s `cellBegin`).
    let mut sums = vec![0.0_f64; CELLS];
    let mut weights = vec![0.0_f64; CELLS];
    let columns: Vec<usize> = (0..width)
        .map(|x| (x * EXTENT / width).min(EXTENT - 1))
        .collect();
    for (y, line) in rgba.chunks_exact(width * 4).enumerate() {
        let row = (y * EXTENT / height).min(EXTENT - 1) * EXTENT;
        for (texel, &column) in line.as_chunks::<4>().0.iter().zip(&columns) {
            if texel[3] == 0 {
                continue;
            }
            let alpha = f64::from(texel[3]) / 255.0;
            let luminance = LUMINANCE_WEIGHTS[0] * srgb.decode[usize::from(texel[0])]
                + LUMINANCE_WEIGHTS[1] * srgb.decode[usize::from(texel[1])]
                + LUMINANCE_WEIGHTS[2] * srgb.decode[usize::from(texel[2])];
            sums[row + column] += f64::from(luminance) * alpha;
            weights[row + column] += alpha;
        }
    }

    // Empty cells read as black would make the estimate a spike and drive the correction into its
    // clamps: they take the mean of the sampled cells instead.
    let mut total = 0.0_f32;
    let mut sampled = 0_usize;
    let values = &mut grid.values;
    for cell in 0..CELLS {
        if weights[cell] > 0.0 {
            values[cell] = to_f32(sums[cell] / weights[cell]);
            total += values[cell];
            sampled += 1;
        }
    }
    if sampled == 0 {
        return grid;
    }
    let average = total / usize_to_f32(sampled);
    for cell in 0..CELLS {
        if weights[cell] <= 0.0 {
            values[cell] = average;
        }
    }

    blur(values, wrap_u, wrap_v);

    // Normalising makes this a redistribution rather than a dimmer.
    let mean = to_f32(
        values.iter().map(|&value| f64::from(value)).sum::<f64>() / f64::from(usize_to_f32(CELLS)),
    );
    if mean.is_nan() || mean <= 0.0 {
        return ShadingGrid::neutral(wrap_u, wrap_v);
    }
    for value in values.iter_mut() {
        *value = (*value / mean).clamp(FLOOR, CEILING);
    }
    grid
}

/// [`BLUR_PASSES`] separable 3x3 box blurs, wrapping on a repeating axis and clamping on the other.
fn blur(values: &mut [f32; CELLS], wrap_u: bool, wrap_v: bool) {
    let last = EXTENT - 1;
    let before = |i: usize, wrap: bool| {
        if wrap {
            (i + last) % EXTENT
        } else {
            i.saturating_sub(1)
        }
    };
    let after = |i: usize, wrap: bool| {
        if wrap {
            (i + 1) % EXTENT
        } else {
            (i + 1).min(last)
        }
    };
    let mut across = [0.0_f32; CELLS];
    for _ in 0..BLUR_PASSES {
        for y in 0..EXTENT {
            let row = y * EXTENT;
            for x in 0..EXTENT {
                across[row + x] = (values[row + before(x, wrap_u)]
                    + values[row + x]
                    + values[row + after(x, wrap_u)])
                    / 3.0;
            }
        }
        for y in 0..EXTENT {
            let above = before(y, wrap_v) * EXTENT;
            let here = y * EXTENT;
            let below = after(y, wrap_v) * EXTENT;
            for x in 0..EXTENT {
                values[here + x] = (across[above + x] + across[here + x] + across[below + x]) / 3.0;
            }
        }
    }
}

/// Divides `grid`'s painted light out of `rgba` in place, at `strength` (0 = unchanged, 1 = the
/// full estimate; clamped to that range, NaN reads as 0).
///
/// Per texel: `delit = linear / mix(1, factor, strength)`, then `delit / max(brightest(delit), 1)`
/// so reflectance stays at most one with the colour's hue kept (upstream `delitTexel`). Colour is
/// decoded from and re-encoded to sRGB; alpha is untouched. `width` / `height` must match the
/// level the grid was estimated from in aspect; the lookup is in texture space.
///
/// # Panics
/// If `rgba.len() != width * height * 4`.
pub fn delight(rgba: &mut [u8], width: u32, height: u32, grid: &ShadingGrid, strength: f32) {
    let (width, height) = check_len(rgba, width, height);
    let strength = if strength.is_nan() {
        0.0
    } else {
        strength.clamp(0.0, 1.0)
    };
    if strength == 0.0 || width == 0 || height == 0 || grid.is_neutral() {
        return;
    }
    let srgb = Srgb::get();

    // The lookup is separable: precompute each column's and row's taps at the texel centre.
    let taps = |count: usize, wrap: bool| -> Vec<Taps> {
        (0..count)
            .map(|i| {
                let (low, high, t) = axis_taps((usize_to_f32(i) + 0.5) / usize_to_f32(count), wrap);
                Taps { low, high, t }
            })
            .collect()
    };
    let columns = taps(width, grid.wrap_u);
    let rows = taps(height, grid.wrap_v);

    for (line, &row) in rgba.chunks_exact_mut(width * 4).zip(&rows) {
        for (texel, &column) in line.as_chunks_mut::<4>().0.iter_mut().zip(&columns) {
            let factor = 1.0 + strength * (grid.bilinear(column, row) - 1.0);
            let mut colour = [0.0_f32; 3];
            for (out, &byte) in colour.iter_mut().zip(&texel[..3]) {
                *out = srgb.decode[usize::from(byte)] / factor;
            }
            let brightest = colour[0].max(colour[1]).max(colour[2]);
            let scale = if brightest > 1.0 {
                brightest.recip()
            } else {
                1.0
            };
            for (byte, &linear) in texel[..3].iter_mut().zip(&colour) {
                *byte = srgb.encode(linear * scale);
            }
        }
    }
}

#[cfg(test)]
mod tests;
