//! Sun cascade splits (shadows F3, `docs/research/shadows/README.md`).
//!
//! Bevy 0.20's `CascadeShadowConfigBuilder` fixes the first bound and spaces
//! the rest geometrically. [`CascadeSplits`] is the industry's *practical
//! split* instead (Zhang et al., PSSM, GPU Gems 3 ch. 10): each bound is a
//! `lambda` blend of the logarithmic split (constant texel density per depth)
//! and the uniform one, which keeps the last cascades from growing too fast.
//! The result is a plain Bevy [`CascadeShadowConfig`], so Bevy still fits
//! each cascade to a bounding sphere of its frustum slice and snaps it to
//! whole texels (stable cascades, no shimmer when the camera moves) and
//! blends neighbours over `overlap_proportion`.

use bevy_light::CascadeShadowConfig;

/// A practical-split cascade layout (PSSM).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CascadeSplits {
    /// Number of cascades (Bevy caps it at 4).
    pub num_cascades: usize,
    /// Near bound of the first cascade, meters.
    pub minimum_distance: f32,
    /// Near plane the split formula uses, meters. The camera's 0.1 m near
    /// plane would make the log split degenerate (a 0.7 m first cascade),
    /// so engines use a larger split near.
    pub split_near: f32,
    /// Far bound of the last cascade, meters.
    pub maximum_distance: f32,
    /// Log weight: 1 = logarithmic, 0 = uniform.
    pub lambda: f32,
    /// Overlap with the previous cascade, blended by Bevy.
    pub overlap_proportion: f32,
}

impl CascadeSplits {
    /// The far bounds of the cascades, meters.
    #[must_use]
    pub fn bounds(&self) -> Vec<f32> {
        let n = self.split_near.max(1e-3);
        let f = self.maximum_distance.max(n);
        #[allow(clippy::cast_precision_loss)]
        let count = self.num_cascades.max(1) as f32;
        (1..=self.num_cascades.max(1))
            .map(|i| {
                #[allow(clippy::cast_precision_loss)]
                let t = i as f32 / count;
                let log = n * (f / n).powf(t);
                let uniform = n + (f - n) * t;
                self.lambda * log + (1.0 - self.lambda) * uniform
            })
            .collect()
    }

    /// The Bevy component.
    #[must_use]
    pub fn build(&self) -> CascadeShadowConfig {
        CascadeShadowConfig {
            bounds: self.bounds(),
            overlap_proportion: self.overlap_proportion,
            minimum_distance: self.minimum_distance,
        }
    }
}

/// World size of one shadow-map texel per cascade, meters, as Bevy 0.20
/// sizes them (`bevy_light::cascade::calculate_cascade`: the larger of the
/// slice's body and far-plane diagonals, rounded up, over the map size).
/// For tuning and tests; Bevy computes the real ones each frame.
#[must_use]
pub fn cascade_texel_sizes(config: &CascadeShadowConfig, vertical_fov: f32, aspect: f32, map_size: u32) -> Vec<f32> {
    let t = (vertical_fov * 0.5).tan();
    #[allow(clippy::cast_precision_loss)]
    let size = map_size as f32;
    config
        .bounds
        .iter()
        .enumerate()
        .map(|(i, &far)| {
            let near = if i == 0 { config.minimum_distance } else { (1.0 - config.overlap_proportion) * config.bounds[i - 1] };
            let (hn, wn, hf, wf) = (near * t, near * t * aspect, far * t, far * t * aspect);
            let body = ((wf + wn).powi(2) + (hf + hn).powi(2) + (far - near).powi(2)).sqrt();
            let far_plane = (2.0 * wf).hypot(2.0 * hf);
            body.max(far_plane).ceil() / size
        })
        .collect()
}
