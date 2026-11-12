//! Temporal accumulation of the half-resolution cloud history (weather.md "Clouds", WE-10 fix 1),
//! the CPU twin of `cloud_resolve.wesl`: an exponential history whose fresh samples weigh
//! [`CloudTemporal::fresh_weight`], clipped to this frame's neighbourhood in `YCoCg` by variance
//! (mean ± γσ intersected with the min/max box, Salvi 2016 "An Excursion in Temporal
//! Supersampling"; the clip toward the box centre of Playdead's INSIDE TAA, Pedersen 2016).
//! Stable silhouettes (S1, clouds.md "Stable silhouette history"): the trace's phase-independent
//! depth window ([`depth_window`]), the resolve's same-population fallback ([`mismatch_fallback`])
//! and accumulated cloud distance ([`accumulate_distance`]).

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

/// The checkerboard sub-pixel a block traces on `frame` (`cloud_common.wesl` `sub_pixel`):
/// (0,0), (1,1), (1,0), (0,1), one of the block's 2x2 half-resolution texels per frame.
#[must_use]
pub fn sub_pixel(frame: u32) -> [u32; 2] {
    let i = frame % 4;
    [u32::from(i == 1 || i == 2), u32::from(i == 1 || i == 3)]
}

/// The full-resolution depth window a trace texel takes its furthest and nearest scene depth
/// over (`cloud_trace.wesl` `trace`), and the uv of the ray its open/cut class is taken along.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepthWindow {
    /// First full-resolution pixel (inclusive; the shader clamps each tap to the target).
    pub min: [i32; 2],
    /// Last full-resolution pixel (inclusive): always 6x6 pixels.
    pub max: [i32; 2],
    /// The class ray's uv.
    pub class_uv: [f32; 2],
}

/// The depth window of trace texel `block` on `frame` (S1, WE-14). The checkerboard's trace
/// target is a quarter of the half-resolution one: each block's 2x2 half texels are traced one
/// per frame. `stable` (S1, `CloudEdgeStable`): the window covers the whole block, its 4x4
/// full-resolution pixels plus the one pixel every side through which the composite's bilinear
/// upsample reaches the block's texels (the low-res furthest-depth footprint of Nubis' and UE's
/// cloud passes), and the class is taken along the block's centre ray, so neither depends on the
/// phase. Not `stable` (the old window): centred on this frame's traced half texel, so it moves
/// two full pixels with the phase and a block on a silhouette flips between open and cut, which
/// throws its history away (STORM-DIAG 2026-11-12). Without the checkerboard both are the same.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss)]
pub fn depth_window(block: [u32; 2], frame: u32, half_size: [u32; 2], full_size: [u32; 2], checkerboard: bool, stable: bool) -> DepthWindow {
    let sub = sub_pixel(frame);
    let traced: [f32; 2] = std::array::from_fn(|i| if checkerboard { (block[i] * 2 + sub[i]) as f32 + 0.5 } else { block[i] as f32 + 0.5 });
    let uv: [f32; 2] = std::array::from_fn(|i| {
        (if stable && checkerboard { (block[i] * 2) as f32 + 1.0 } else { traced[i] }) / half_size[i] as f32
    });
    let px: [i32; 2] = std::array::from_fn(|i| ((uv[i] * full_size[i] as f32) as i32).clamp(0, full_size[i] as i32 - 1));
    DepthWindow { min: px.map(|p| p - 3), max: px.map(|p| p + 2), class_uv: uv }
}

/// The furthest scene depth (reverse-Z: the smallest, 0 = sky) in `window` of a `width` x
/// `height` depth image, taps clamped to the image (the trace's `depth`): 0 means the texel is
/// open (it saw sky), else it is cut at that depth.
#[must_use]
#[allow(clippy::cast_sign_loss)] // clamped to 0 first
pub fn window_furthest_depth(depth: &[f32], width: usize, height: usize, window: DepthWindow) -> f32 {
    let mut furthest = 1.0f32;
    for y in window.min[1]..=window.max[1] {
        for x in window.min[0]..=window.max[0] {
            let (cx, cy) = (x.max(0) as usize, y.max(0) as usize);
            furthest = furthest.min(depth[cy.min(height - 1) * width + cx.min(width - 1)]);
        }
    }
    furthest
}

/// Whether a tap cut at `b` belongs with a texel cut at `cut` (`cloud_resolve.wesl`
/// `same_population`): both open (at `far` or beyond), or both cut within 10 % + 5 m.
#[must_use]
pub fn same_population(b: f32, cut: f32, far: f32) -> bool {
    let (open_tap, open_here) = (b >= far, cut >= far);
    (open_tap && open_here) || (!open_tap && !open_here && (b - cut).abs() <= 0.1 * cut + 5.0)
}

/// The value a texel shows when its history held only the other population (S1): the mean of
/// this frame's 3x3 taps `(colour, cut)` of its own population (the centre, cut at `cut`, always
/// counts), instead of its one raw jittered sample. `None` if no tap matches (no centre given).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn mismatch_fallback(cut: f32, taps: &[([f32; 4], f32)], far: f32) -> Option<[f32; 4]> {
    let same: Vec<[f32; 4]> = taps.iter().filter(|(_, b)| same_population(*b, cut, far)).map(|(c, _)| *c).collect();
    (!same.is_empty()).then(|| std::array::from_fn(|i| same.iter().map(|c| c[i]).sum::<f32>() / same.len() as f32))
}

/// The stored cloud distance with S1: the history clamped to the neighbourhood's range of this
/// frame's distances `[lo, hi]`, then moved toward the fresh one by `weight` like the colour, so
/// the composite's occlusion test reads a settled distance rather than the last jittered march.
#[must_use]
pub fn accumulate_distance(history: f32, current: f32, lo: f32, hi: f32, weight: f32) -> f32 {
    accumulate(history.clamp(lo, hi), current, weight)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact values by construction
mod tests {
    use super::*;

    const HALF: [u32; 2] = [8, 8];
    const FULL: [u32; 2] = [16, 16];

    /// The full-resolution pixels whose bilinear composite tap reaches half texel `t` (one axis).
    fn bilinear_reach(t: i32) -> Vec<i32> {
        // The composite's `floor(uv * half_size - 0.5)` at pixel centre f + 0.5: floor((2f − 1) / 4).
        (0..16).filter(|&f: &i32| {
            let base = (2 * f - 1).div_euclid(4);
            base == t || base + 1 == t
        }).collect()
    }

    #[test]
    fn stable_window_footprint_is_independent_of_the_checkerboard_phase() {
        for by in 0..4i32 {
            for bx in 0..4i32 {
                let block = [bx.unsigned_abs(), by.unsigned_abs()];
                let w: Vec<DepthWindow> = (0..4).map(|f| depth_window(block, f, HALF, FULL, true, true)).collect();
                assert!(w.iter().all(|x| *x == w[0]), "block {bx},{by}: one window and class ray for all 4 phases");
                let b = [bx, by];
                assert_eq!(w[0].min, b.map(|v| 4 * v - 1));
                assert_eq!(w[0].max, b.map(|v| 4 * v + 4));
                // It covers the block's 4x4 pixels and every pixel the composite upsamples its texels into.
                for axis in 0..2 {
                    for f in bilinear_reach(2 * b[axis]).into_iter().chain(bilinear_reach(2 * b[axis] + 1)) {
                        assert!((w[0].min[axis]..=w[0].max[axis]).contains(&f), "pixel {f} reaches block {b:?}");
                    }
                }
            }
        }
        // The old window moves two pixels with the phase (the bug); without the checkerboard
        // there is no phase and both windows agree.
        let old: Vec<DepthWindow> = (0..4).map(|f| depth_window([1, 1], f, HALF, FULL, true, false)).collect();
        assert_eq!(old[0].min, [2, 2]);
        assert_eq!(old[1].min, [4, 4]);
        assert_eq!(depth_window([2, 3], 1, FULL, [32, 32], false, true), depth_window([2, 3], 1, FULL, [32, 32], false, false));
    }

    /// A depth image with a vertical silhouette at x = 8: `left` for x < 8, `right` beyond.
    fn silhouette(left: f32, right: f32) -> Vec<f32> {
        (0..16 * 16).map(|i| if i % 16 < 8 { left } else { right }).collect()
    }

    #[test]
    fn a_block_straddling_a_silhouette_keeps_one_class_across_the_four_phases() {
        // Ridge against sky (open vs cut) and ridge against far terrain (cut at the ridge vs
        // cut at the far land): block 1 ends at pixel 7, the silhouette is at 8.
        for (left, right) in [(0.5, 0.0), (0.5, 0.01)] {
            let depth = silhouette(left, right);
            let class = |block: [u32; 2], f: u32, stable: bool| window_furthest_depth(&depth, 16, 16, depth_window(block, f, HALF, FULL, true, stable));
            for by in 0..4 {
                for bx in 0..4 {
                    let c: Vec<f32> = (0..4).map(|f| class([bx, by], f, true)).collect();
                    assert!(c.iter().all(|x| *x == c[0]), "block {bx},{by} ({left} | {right}): {c:?}");
                }
            }
            assert_eq!(class([1, 1], 0, true), right, "the block's bilinear reach touches the far side");
            // The old window: the same block flips with the phase.
            assert_eq!(class([1, 1], 0, false), left);
            assert_eq!(class([1, 1], 1, false), right);
        }
    }

    #[test]
    fn the_mismatch_fallback_averages_only_same_population_neighbours() {
        let far = 1000.0;
        let open = |v: f32| ([v, v, v, 1.0], far);
        let cut_at = |v: f32, t: f32| ([v, v, v, 0.5], t);
        // An open texel: the open taps 1, 2, 3; the cut taps (bright tower) never leak in.
        let taps = [open(1.0), open(2.0), cut_at(50.0, 100.0), open(3.0), cut_at(60.0, 104.0)];
        assert_eq!(mismatch_fallback(far, &taps, far), Some([2.0, 2.0, 2.0, 1.0]));
        // A texel cut at 100 m: the taps cut within 10 % + 5 m (100, 104), not 200 m nor sky.
        let taps = [cut_at(4.0, 100.0), cut_at(8.0, 104.0), cut_at(90.0, 200.0), open(70.0)];
        assert_eq!(mismatch_fallback(100.0, &taps, far), Some([6.0, 6.0, 6.0, 0.5]));
        // The centre alone: its own sample.
        assert_eq!(mismatch_fallback(100.0, &[cut_at(4.0, 100.0)], far), Some([4.0, 4.0, 4.0, 0.5]));
        assert_eq!(mismatch_fallback(100.0, &[open(1.0)], far), None);
    }

    #[test]
    fn the_stored_distance_accumulates_inside_the_neighbourhood_range() {
        // A settled 3000 m deck: a jittered 2000 m march moves it by the fresh weight only.
        assert_eq!(accumulate_distance(3000.0, 2000.0, 1000.0, 5000.0, 0.25), 2750.0);
        // A history outside this frame's range (a nearer cloud moved in) is clamped first.
        assert_eq!(accumulate_distance(9000.0, 1000.0, 1000.0, 2000.0, 0.5), 1500.0);
    }

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
