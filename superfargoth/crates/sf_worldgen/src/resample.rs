//! Bicubic sampling (Catmull-Rom spline, Keys 1981 with a = -0.5) and the cubic B-spline used to blend
//! per-region parameters.
//!
//! Keys, R. "Cubic convolution interpolation for digital image processing", IEEE TASSP 29(6), 1981. Catmull-Rom
//! interpolates (passes through the samples) and reproduces linear ramps; it may overshoot by a few percent near
//! steps, which is why region *parameters* use the non-negative cubic B-spline instead (de Boor), which never
//! overshoots and is C2, so a blended parameter leaves no crease at region-cell edges.
//!
//! Exactness: the sum is taken as `h0 + Σ wᵢ (hᵢ - h0)` around the nearest sample h0, so a constant field
//! resamples to exactly that constant (the weights' rounding error multiplies zero).

use crate::Grid;

/// Catmull-Rom weights for the 4 taps at offsets -1, 0, 1, 2 from the floor sample, fractional position `t`.
#[inline]
pub fn catmull_rom_weights(t: f64) -> [f64; 4] {
    let t2 = t * t;
    let t3 = t2 * t;
    [
        -0.5 * t3 + t2 - 0.5 * t,
        1.5 * t3 - 2.5 * t2 + 1.0,
        -1.5 * t3 + 2.0 * t2 + 0.5 * t,
        0.5 * t3 - 0.5 * t2,
    ]
}

/// Uniform cubic B-spline weights for the 4 taps at offsets -1, 0, 1, 2 from the floor, fractional `t`.
#[inline]
pub fn bspline_weights(t: f64) -> [f64; 4] {
    let u = 1.0 - t;
    let t2 = t * t;
    let t3 = t2 * t;
    [
        u * u * u / 6.0,
        (3.0 * t3 - 6.0 * t2 + 4.0) / 6.0,
        (-3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0) / 6.0,
        t3 / 6.0,
    ]
}

/// Catmull-Rom sample of `get` (clamped edges handled by the caller) at fractional sample position (x, y).
#[inline]
pub fn bicubic_with(x: f64, y: f64, get: impl Fn(isize, isize) -> f64) -> f64 {
    let xf = x.floor();
    let yf = y.floor();
    let (ix, iy) = (xf as isize, yf as isize);
    let wx = catmull_rom_weights(x - xf);
    let wy = catmull_rom_weights(y - yf);
    let h0 = get((x.round()) as isize, (y.round()) as isize);
    let mut acc = 0.0;
    for (j, wyj) in wy.iter().enumerate() {
        let mut row = 0.0;
        for (i, wxi) in wx.iter().enumerate() {
            row += wxi * (get(ix + i as isize - 1, iy + j as isize - 1) - h0);
        }
        acc += wyj * row;
    }
    h0 + acc
}

/// Catmull-Rom sample of a grid at fractional sample position (x, y), edges extended.
#[inline]
pub fn bicubic(g: &Grid, x: f64, y: f64) -> f64 {
    bicubic_with(x, y, |i, j| g.at_clamped(i, j))
}

/// Resamples `src` to a new spacing over its whole extent: output sample (i, j) sits at source position
/// (i, j) × `out_spacing / src.spacing`. Pure per sample, so the parallel map is deterministic.
pub fn resample(src: &Grid, out_spacing: f64) -> Grid {
    use rayon::prelude::*;
    let r = out_spacing / src.spacing;
    let w = (((src.w - 1) as f64) / r).floor() as usize + 1;
    let h = (((src.h - 1) as f64) / r).floor() as usize + 1;
    let mut data = vec![0.0; w * h];
    data.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        for (i, v) in row.iter_mut().enumerate() {
            *v = bicubic(src, i as f64 * r, j as f64 * r);
        }
    });
    Grid {
        w,
        h,
        spacing: out_spacing,
        data,
    }
}
