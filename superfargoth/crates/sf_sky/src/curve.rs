//! Piecewise-linear curves for data-driven tuning (exposure, ambient).

use serde::{Deserialize, Serialize};

/// `(x, y)` keys sorted by `x`, linearly interpolated, clamped at both ends.
/// Serialized as a list of pairs, e.g. `[[-18, 0], [0, 10], [30, 14]]`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KeyCurve(Vec<(f32, f32)>);

impl KeyCurve {
    /// Builds a curve; keys are sorted by `x`.
    pub fn new(mut keys: Vec<(f32, f32)>) -> Self {
        keys.sort_by(|a, b| a.0.total_cmp(&b.0));
        Self(keys)
    }

    /// A constant curve.
    pub fn constant(y: f32) -> Self {
        Self(vec![(0.0, y)])
    }

    pub fn keys(&self) -> &[(f32, f32)] {
        &self.0
    }

    /// Value at `x`. An empty curve is 0.
    pub fn sample(&self, x: f32) -> f32 {
        let keys = &self.0;
        let (Some(first), Some(last)) = (keys.first(), keys.last()) else {
            return 0.0;
        };
        if x <= first.0 {
            return first.1;
        }
        if x >= last.0 {
            return last.1;
        }
        let i = keys.partition_point(|k| k.0 <= x);
        let (x0, y0) = keys[i - 1];
        let (x1, y1) = keys[i];
        if x1 <= x0 {
            return y1;
        }
        y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    }

    /// True when `y` never decreases as `x` grows.
    pub fn is_monotonic(&self) -> bool {
        self.0.windows(2).all(|w| w[1].1 >= w[0].1)
    }
}
