//! Order statistics for the bench's per-frame series: the median, nearest-rank percentiles, and a [`Series`] that
//! collects one value per frame (or per GPU sample) and summarises it at the end of an arm.

/// The median: the middle value, or the mean of the two middle values for an even count. Sorts `values` in place.
/// Empty input gives 0.
pub fn median(values: &mut [f64]) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    values.sort_unstable_by(f64::total_cmp);
    if n % 2 == 1 { values[n / 2] } else { f64::midpoint(values[n / 2 - 1], values[n / 2]) }
}

/// The `p`th percentile (`p` in 0..=100, clamped) by nearest rank: the value at rank `ceil(p x n / 100)` (at least
/// 1) of the sorted values, so p99 of 100 values is the 99th smallest. Sorts `values` in place. Empty input gives 0.
pub fn percentile(values: &mut [f64], p: f64) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    values.sort_unstable_by(f64::total_cmp);
    // `p x n` first, then `/ 100`: exact for whole `p` (0.95 x 20 would round up to rank 20).
    let rank = (p.clamp(0.0, 100.0) * n as f64 / 100.0).ceil() as usize;
    values[rank.clamp(1, n) - 1]
}

/// One value per frame (or per sample), summarised at the end of an arm.
#[derive(Clone, Debug, Default)]
pub struct Series {
    values: Vec<f64>,
}

impl Series {
    pub fn push(&mut self, value: f64) {
        self.values.push(value);
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn median(&self) -> f64 {
        median(&mut self.values.clone())
    }

    pub fn p95(&self) -> f64 {
        percentile(&mut self.values.clone(), 95.0)
    }

    pub fn p99(&self) -> f64 {
        percentile(&mut self.values.clone(), 99.0)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // every expected value is one of the inputs or an exact midpoint
mod tests {
    use super::*;

    #[test]
    fn median_odd_even_and_empty() {
        assert_eq!(median(&mut [3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&mut [4.0, 1.0, 3.0, 2.0]), 2.5);
        assert_eq!(median(&mut [7.0]), 7.0);
        assert_eq!(median(&mut []), 0.0);
    }

    #[test]
    fn percentile_is_nearest_rank() {
        let ramp = || (1..=100).map(f64::from).rev().collect::<Vec<_>>();
        assert_eq!(percentile(&mut ramp(), 99.0), 99.0);
        assert_eq!(percentile(&mut ramp(), 95.0), 95.0);
        assert_eq!(percentile(&mut ramp(), 50.0), 50.0);
        assert_eq!(percentile(&mut ramp(), 100.0), 100.0);
        assert_eq!(percentile(&mut ramp(), 0.0), 1.0);
        // 20 values: p95 is rank 19, p99 is rank ceil(19.8) = 20.
        let twenty = || (1..=20).map(f64::from).collect::<Vec<_>>();
        assert_eq!(percentile(&mut twenty(), 95.0), 19.0);
        assert_eq!(percentile(&mut twenty(), 99.0), 20.0);
        // 3 values: p50 is rank ceil(1.5) = 2.
        assert_eq!(percentile(&mut [30.0, 10.0, 20.0], 50.0), 20.0);
        assert_eq!(percentile(&mut [], 99.0), 0.0);
        assert_eq!(percentile(&mut [5.0, 6.0], 150.0), 6.0);
        assert_eq!(percentile(&mut [5.0, 6.0], -3.0), 5.0);
    }

    #[test]
    fn series_summarises_without_reordering() {
        let mut s = Series::default();
        assert_eq!((s.count(), s.median(), s.p99()), (0, 0.0, 0.0));
        for v in [10.0, 1.0, 9.0, 2.0, 8.0, 3.0, 7.0, 4.0, 6.0, 5.0] {
            s.push(v);
        }
        assert_eq!(s.count(), 10);
        assert_eq!(s.median(), 5.5);
        assert_eq!(s.p95(), 10.0);
        assert_eq!(s.p99(), 10.0);
        s.push(100.0);
        assert_eq!((s.count(), s.median(), s.p99()), (11, 6.0, 100.0));
    }
}
