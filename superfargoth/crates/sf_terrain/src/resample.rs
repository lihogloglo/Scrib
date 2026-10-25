//! `upsample`: source heights to a level-0 tile by clamped Catmull-Rom.
//!
//! Catmull-Rom (Keys' cubic convolution with a = −1/2) interpolates: at a
//! source sample its weights are exactly (0, 1, 0, 0), so samples pass
//! through unchanged. Its overshoot is clamped to the range of the
//! neighborhood it read (the 4×4 for upsampling), as TAA resolves clamp to
//! the neighborhood, so it adds no ridges or pits.
//!
//! The filter is separable, and each output column (row) uses the same taps
//! and weights in every row (column), so it runs as a horizontal pass over
//! the source rows, then a vertical pass: 8 multiply-adds per sample
//! instead of 16. The neighborhood min/max is separable the same way.
//!
//! Sources finer than 0.5 m are downsampled with the same kernel stretched
//! by the ratio and normalized (the usual resize prefilter), so they don't
//! alias. Passing through samples then no longer applies.

use crate::source::{Lattice, SourceMosaic, floor_i64, to_i64};
use crate::tile::{BASE_SPACING, HeightTile, TILE_SAMPLES, TileCoord};

/// Resamples the mosaic into level-0 tile `coord`. Pure: the result depends
/// only on `coord` and the mosaic's samples near it.
pub fn upsample(coord: TileCoord, src: &SourceMosaic) -> HeightTile {
    assert_eq!(coord.level, 0, "upsample builds level-0 tiles; coarser levels come from `downsample`");
    let lattice = src.heights.lattice;
    let (gx0, gz0) = coord.first_sample();
    let px = AxisPlan::new(&lattice, gx0, 0);
    let pz = AxisPlan::new(&lattice, gz0, 1);
    let window = src.heights.gather([px.first, pz.first], [px.span, pz.span], src.fill_height);

    // Horizontal pass: every source row of the window, filtered at the
    // tile's 513 columns, with the per-tap min/max.
    let cols = TILE_SAMPLES;
    let rows = pz.span;
    let mut sum = vec![0.0f32; rows * cols];
    let mut lo = vec![0.0f32; rows * cols];
    let mut hi = vec![0.0f32; rows * cols];
    for r in 0..rows {
        let row = &window.data[r * px.span..(r + 1) * px.span];
        let out = r * cols..(r + 1) * cols;
        px.filter_row(row, &mut sum[out.clone()], &mut lo[out.clone()], &mut hi[out]);
    }

    // Vertical pass over the filtered rows, accumulating whole rows so the
    // inner loop vectorizes. Each sample still sums its taps in tap order.
    let mut heights = vec![0.0f32; cols * cols];
    let mut row_lo = vec![0.0f32; cols];
    let mut row_hi = vec![0.0f32; cols];
    for (m, out) in heights.chunks_exact_mut(cols).enumerate() {
        let start = pz.starts[m];
        let weights = &pz.weights[m * pz.taps..(m + 1) * pz.taps];
        row_lo.fill(f32::INFINITY);
        row_hi.fill(f32::NEG_INFINITY);
        for (k, &w) in weights.iter().enumerate() {
            let r = (start + k) * cols;
            let (s, l, h) = (&sum[r..r + cols], &lo[r..r + cols], &hi[r..r + cols]);
            for n in 0..cols {
                out[n] += w * s[n];
                row_lo[n] = row_lo[n].min(l[n]);
                row_hi[n] = row_hi[n].max(h[n]);
            }
        }
        for n in 0..cols {
            out[n] = out[n].max(row_lo[n]).min(row_hi[n]);
        }
    }
    HeightTile { coord, heights }
}

/// The taps and weights of one axis: output sample `n` reads window
/// indices `starts[n] .. starts[n] + taps` with `weights[n·taps ..]`.
struct AxisPlan {
    /// Lattice index of window index 0.
    first: i64,
    /// Window length.
    span: usize,
    taps: usize,
    starts: Vec<usize>,
    weights: Vec<f32>,
}

impl AxisPlan {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn new(lattice: &Lattice, g0: i64, axis: usize) -> Self {
        // Kernel stretch: 1 when upsampling, out/in spacing when downsampling.
        let scale = (f64::from(BASE_SPACING) / lattice.spacing).max(1.0);
        let radius = 2.0 * scale;
        let taps = 2 * radius.ceil() as usize;
        let mut abs_starts = Vec::with_capacity(TILE_SAMPLES);
        let mut weights = Vec::with_capacity(TILE_SAMPLES * taps);
        for n in 0..to_i64(TILE_SAMPLES) {
            let u = lattice.coord(g0 + n, axis);
            let start = floor_i64(u - radius) + 1;
            let w0 = weights.len();
            let mut total = 0.0;
            for k in 0..to_i64(taps) {
                let w = catmull_rom(((start + k) as f64 - u) / scale);
                total += w;
                weights.push(w);
            }
            if scale > 1.0 {
                for w in &mut weights[w0..] {
                    *w /= total;
                }
            }
            abs_starts.push(start);
        }
        // `u` grows with `n`, so starts are sorted.
        let first = abs_starts[0];
        let span = (abs_starts[TILE_SAMPLES - 1] - first) as usize + taps;
        Self {
            first,
            span,
            taps,
            starts: abs_starts.iter().map(|&s| (s - first) as usize).collect(),
            weights: weights.into_iter().map(|w| w as f32).collect(),
        }
    }

    /// Filters one window row at every output sample, with the min/max of
    /// the taps each read.
    fn filter_row(&self, row: &[f32], sum: &mut [f32], lo: &mut [f32], hi: &mut [f32]) {
        for n in 0..TILE_SAMPLES {
            let s = self.starts[n];
            let taps = &row[s..s + self.taps];
            let weights = &self.weights[n * self.taps..(n + 1) * self.taps];
            let (mut acc, mut mn, mut mx) = (0.0f32, f32::INFINITY, f32::NEG_INFINITY);
            for (&h, &w) in taps.iter().zip(weights) {
                acc += w * h;
                mn = mn.min(h);
                mx = mx.max(h);
            }
            sum[n] = acc;
            lo[n] = mn;
            hi[n] = mx;
        }
    }
}

/// Keys' cubic convolution kernel with a = −1/2 (Catmull-Rom).
fn catmull_rom(x: f64) -> f64 {
    let x = x.abs();
    if x < 1.0 {
        (1.5 * x - 2.5) * x * x + 1.0
    } else if x < 2.0 {
        ((-0.5 * x + 2.5) * x - 4.0) * x + 2.0
    } else {
        0.0
    }
}
