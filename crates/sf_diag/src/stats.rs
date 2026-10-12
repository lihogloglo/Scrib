//! Order statistics over samples.

use serde::{Deserialize, Serialize};

/// Which statistic of a sample set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stat {
    Mean,
    Min,
    Median,
    P95,
    P99,
    Max,
}

/// Summary of a sample set. Percentiles interpolate linearly between
/// order statistics (Hyndman & Fan type 7, the common default).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub count: usize,
    pub mean: f64,
    pub min: f64,
    pub median: f64,
    pub p95: f64,
    pub p99: f64,
    pub max: f64,
}

impl Stats {
    /// Stats over the non-NaN samples, or `None` if there are none.
    pub fn from_samples(samples: &[f64]) -> Option<Self> {
        let mut sorted: Vec<f64> = samples.iter().copied().filter(|v| !v.is_nan()).collect();
        if sorted.is_empty() {
            return None;
        }
        sorted.sort_by(f64::total_cmp);
        #[allow(clippy::cast_precision_loss)] // sample counts are far below 2^52
        let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
        Some(Self {
            count: sorted.len(),
            mean,
            min: sorted[0],
            median: percentile_sorted(&sorted, 50.0),
            p95: percentile_sorted(&sorted, 95.0),
            p99: percentile_sorted(&sorted, 99.0),
            max: sorted[sorted.len() - 1],
        })
    }

    pub fn get(&self, stat: Stat) -> f64 {
        match stat {
            Stat::Mean => self.mean,
            Stat::Min => self.min,
            Stat::Median => self.median,
            Stat::P95 => self.p95,
            Stat::P99 => self.p99,
            Stat::Max => self.max,
        }
    }
}

/// The `p`th percentile (0–100) of ascending, non-empty, NaN-free samples,
/// interpolating linearly between neighbors.
pub fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    assert!(!sorted.is_empty(), "percentile of no samples");
    #[allow(clippy::cast_precision_loss)] // sample counts are far below 2^52
    let rank = (sorted.len() - 1) as f64 * (p.clamp(0.0, 100.0) / 100.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // rank >= 0, < len
    let lo = rank.floor() as usize;
    let hi = (lo + 1).min(sorted.len() - 1);
    #[allow(clippy::cast_precision_loss)]
    let frac = rank - lo as f64;
    sorted[lo] + frac * (sorted[hi] - sorted[lo])
}
