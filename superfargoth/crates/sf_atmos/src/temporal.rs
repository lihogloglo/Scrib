//! Temporal accumulation of the half-resolution cloud history (weather.md "Clouds", WE-10 fix 1),
//! the CPU twin of `cloud_resolve.wesl`: an exponential history whose fresh samples weigh
//! [`CloudTemporal::fresh_weight`], clipped to this frame's neighbourhood in `YCoCg` by variance
//! (mean ± γσ intersected with the min/max box, Salvi 2016 "An Excursion in Temporal
//! Supersampling"; the clip toward the box centre of Playdead's INSIDE TAA, Pedersen 2016).

/// How the cloud resolve accumulates its history.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloudTemporal {
    /// Weight of a fresh sample against the history (each half-resolution pixel is fresh one
    /// frame in four under the checkerboard): about 1 / samples accumulated.
    pub fresh_weight: f32,
    /// Variance-clip width γ in standard deviations; 0 clamps to the neighbourhood's min/max.
    pub clip_gamma: f32,
    /// March jitter from spatio-temporal blue noise (STBN) instead of interleaved gradient noise.
    pub blue_noise: bool,
}

impl CloudTemporal {
    /// Accumulate about 16 fresh samples (≈ 64 frames with the checkerboard), clip at 1.5σ, STBN.
    /// Tuned on `scene we3-ridge` (WE-10): a far-weather step is often opaque, so one sample is a
    /// coin toss and the frame-to-frame change scales with the weight; the clip keeps it responsive.
    pub const ACCUMULATE: Self = Self { fresh_weight: 0.06, clip_gamma: 1.5, blue_noise: true };
    /// The resolve before WE-10: half history, min/max clamp, gradient noise (the A/B "off").
    pub const LEGACY: Self = Self { fresh_weight: 0.5, clip_gamma: 0.0, blue_noise: false };

    /// The share of one sample's noise (standard deviation) left in the converged history:
    /// an exponential average of independent samples has variance `w / (2 − w)` of one sample.
    #[must_use]
    pub fn residual_noise(self) -> f32 {
        let w = self.fresh_weight.clamp(0.0, 1.0);
        (w / (2.0 - w)).sqrt()
    }
}

impl Default for CloudTemporal {
    fn default() -> Self {
        Self::ACCUMULATE
    }
}

/// Linear RGB to `YCoCg` (the colour space the clip box is built in).
#[must_use]
pub fn rgb_to_ycocg(c: [f32; 3]) -> [f32; 3] {
    [0.25 * c[0] + 0.5 * c[1] + 0.25 * c[2], 0.5 * c[0] - 0.5 * c[2], -0.25 * c[0] + 0.5 * c[1] - 0.25 * c[2]]
}

/// `YCoCg` back to linear RGB.
#[must_use]
pub fn ycocg_to_rgb(y: [f32; 3]) -> [f32; 3] {
    [y[0] + y[1] - y[2], y[0] + y[2], y[0] - y[1] - y[2]]
}

/// The clip box of a neighbourhood (per channel): `mean ± gamma·σ`, inside `[min, max]`.
/// `sum` and `sum_sq` are the sums of the `n` samples and of their squares.
#[must_use]
pub fn clip_box(sum: [f32; 4], sum_sq: [f32; 4], n: f32, lo: [f32; 4], hi: [f32; 4], gamma: f32) -> ([f32; 4], [f32; 4]) {
    let mean: [f32; 4] = std::array::from_fn(|i| sum[i] / n);
    let sigma: [f32; 4] = std::array::from_fn(|i| (sum_sq[i] / n - mean[i] * mean[i]).max(0.0).sqrt());
    (std::array::from_fn(|i| (mean[i] - gamma * sigma[i]).max(lo[i])), std::array::from_fn(|i| (mean[i] + gamma * sigma[i]).min(hi[i])))
}

/// Clips `h` toward the centre of the box `[lo, hi]` until it lies inside (unchanged inside).
#[must_use]
pub fn clip_toward_center(h: [f32; 4], lo: [f32; 4], hi: [f32; 4]) -> [f32; 4] {
    let c: [f32; 4] = std::array::from_fn(|i| f32::midpoint(lo[i], hi[i]));
    let v: [f32; 4] = std::array::from_fn(|i| h[i] - c[i]);
    let reach = (0..4).map(|i| v[i].abs() / (0.5 * (hi[i] - lo[i])).max(1e-6)).fold(0.0, f32::max);
    if reach <= 1.0 {
        return h;
    }
    std::array::from_fn(|i| c[i] + v[i] / reach)
}

/// The resolved value: the clipped history moved toward the fresh sample by `weight`.
#[must_use]
pub fn accumulate(history: f32, current: f32, weight: f32) -> f32 {
    history + (current - history) * weight
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact values by construction
mod tests {
    use super::*;

    #[test]
    fn ycocg_round_trips_exactly() {
        for c in [[1.0, 0.5, 0.25], [0.0, 0.0, 0.0], [4.0, 2.0, 8.0], [0.125, 0.75, 0.5]] {
            assert_eq!(ycocg_to_rgb(rgb_to_ycocg(c)), c);
        }
        assert_eq!(rgb_to_ycocg([1.0, 1.0, 1.0]), [1.0, 0.0, 0.0], "grey has no chroma");
    }

    #[test]
    fn variance_box_sits_inside_the_min_max_box() {
        // Samples 0, 0, 0, 4 on channel 0: mean 1, σ = √3; min/max 0..4.
        let (lo, hi) = clip_box([4.0, 2.0, 0.0, 0.0], [16.0, 1.0, 0.0, 0.0], 4.0, [0.0, 0.5, 0.0, 0.0], [4.0, 0.5, 0.0, 0.0], 1.0);
        assert_eq!(lo[0], 0.0, "mean − σ is below min: min wins");
        assert!((hi[0] - (1.0 + 3f32.sqrt())).abs() < 1e-6);
        assert_eq!((lo[1], hi[1]), (0.5, 0.5), "constant channel: σ = 0");
        // γ = 0 collapses to the mean (clamped into min/max).
        let (lo, hi) = clip_box([4.0, 0.0, 0.0, 0.0], [16.0, 0.0, 0.0, 0.0], 4.0, [0.0; 4], [4.0, 0.0, 0.0, 0.0], 0.0);
        assert_eq!((lo[0], hi[0]), (1.0, 1.0));
    }

    #[test]
    fn clip_moves_outliers_to_the_box_face_toward_the_centre() {
        let lo = [0.0, 0.0, 0.0, 0.0];
        let hi = [2.0, 2.0, 2.0, 2.0];
        assert_eq!(clip_toward_center([1.5, 0.5, 1.0, 1.0], lo, hi), [1.5, 0.5, 1.0, 1.0], "inside: unchanged");
        // 3 units out on channel 0 (extent 1): pulled 1/3 of the way, keeping the direction.
        assert_eq!(clip_toward_center([4.0, 1.0, 2.5, 1.0], lo, hi), [2.0, 1.0, 1.5, 1.0]);
    }

    #[test]
    fn fresh_weight_sets_the_residual_noise() {
        // The diagnosis (cloud-weather-edges-2026-11-06 (b) 3): weight 0.5 keeps √(1/3) ≈ 58 %.
        assert!((CloudTemporal::LEGACY.residual_noise() - (1.0f32 / 3.0).sqrt()).abs() < 1e-6);
        assert!((CloudTemporal::ACCUMULATE.residual_noise() - (0.06f32 / 1.94).sqrt()).abs() < 1e-6);
        assert_eq!(accumulate(1.0, 3.0, 0.25), 1.5);
        assert_eq!(accumulate(1.0, 3.0, 1.0), 3.0);
    }
}
