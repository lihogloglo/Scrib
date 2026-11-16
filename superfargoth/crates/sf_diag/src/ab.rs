//! Interleaved A/B runs: counterbalanced run order, outlier guard, Mann-Whitney U and the decision rule.
//!
//! Industry practice for perf A/Bs (Chromium Pinpoint, Mozilla Perfherder, Abedi & Brecht ICPE 2017 "randomized
//! multiple interleaved trials", Georges et al. OOPSLA 2007): arms interleaved in one chain, never in blocks; one
//! discarded warm-up per arm; a rank test over run medians and "inconclusive" as a verdict
//! (`docs/reviews/perf-verdict-audit-2026-11-16.md` §4-5).

use std::fmt;

use crate::stats::percentile_sorted;

/// Deterministic generator for the plan shuffle (splitmix64): the same seed gives the same plan.
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..n` (`n > 0`; the modulo bias is irrelevant for a handful of rows).
    #[allow(clippy::cast_possible_truncation)] // the result is < n, a usize
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The Latin squares of a Williams design for `k` arms: one square for even `k`, the square and its mirror for
/// odd `k`. Every square has each arm once per position; together they balance first-order carry-over (which
/// arm runs right before which).
fn williams_squares(k: usize) -> Vec<Vec<Vec<usize>>> {
    // First row 0, 1, k-1, 2, k-2, ...; row i adds i (mod k).
    let mut first = Vec::with_capacity(k);
    let (mut lo, mut hi) = (0, k);
    for j in 0..k {
        if j % 2 == 0 {
            first.push(lo);
            lo += 1;
        } else {
            hi -= 1;
            first.push(hi);
        }
    }
    let square: Vec<Vec<usize>> = (0..k).map(|i| first.iter().map(|a| (a + i) % k).collect()).collect();
    if k.is_multiple_of(2) {
        vec![square]
    } else {
        let mirror = square.iter().map(|row| row.iter().rev().copied().collect()).collect();
        vec![square, mirror]
    }
}

/// The arm order of each repeat (`orders[repeat][position] = arm`), counterbalanced: repeats come in blocks of
/// `arms` rows, each block a Latin square of a Williams design with its rows shuffled by `seed`. With two arms
/// that is ABBA or BAAB per pair of repeats. When `repeats` is a multiple of `arms`, every arm runs equally
/// often at every position.
pub fn interleaved_orders(arms: usize, repeats: usize, seed: u64) -> Vec<Vec<usize>> {
    if arms == 0 {
        return Vec::new();
    }
    let squares = williams_squares(arms);
    let mut rng = SplitMix(seed);
    let mut orders = Vec::with_capacity(repeats);
    let mut block = 0;
    while orders.len() < repeats {
        let mut rows = squares[block % squares.len()].clone();
        for i in (1..rows.len()).rev() {
            rows.swap(i, rng.below(i + 1));
        }
        orders.extend(rows.into_iter().take(repeats - orders.len()));
        block += 1;
    }
    orders
}

/// A run's numbers for the outlier guard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunPoint {
    /// The run's median frame time, ms.
    pub median_ms: f64,
    /// Hitches in the measured window.
    pub hitches: usize,
}

/// A run is an outlier when its median exceeds this times the median of its arm's other runs.
pub const OUTLIER_RATIO: f64 = 1.3;
/// ... or its measured hitches exceed this times the arm's median count plus [`HITCH_SLACK`].
pub const HITCH_RATIO: f64 = 3.0;
pub const HITCH_SLACK: f64 = 10.0;

fn median_of(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    Some(percentile_sorted(&v, 50.0))
}

/// Why `run` is an outlier against `peers` (the same arm's other kept runs), or `None`. Only slow runs are flagged
/// (garbage runs are slow: another load, a stall); no peers, no verdict.
pub fn outlier_reason(run: RunPoint, peers: &[RunPoint]) -> Option<String> {
    let peer_median = median_of(peers.iter().map(|p| p.median_ms).collect())?;
    if run.median_ms > OUTLIER_RATIO * peer_median {
        return Some(format!("median {:.2} ms > {OUTLIER_RATIO} x the arm's {peer_median:.2} ms", run.median_ms));
    }
    #[allow(clippy::cast_precision_loss)] // hitch counts
    let peer_hitches = median_of(peers.iter().map(|p| p.hitches as f64).collect())?;
    #[allow(clippy::cast_precision_loss)]
    let hitches = run.hitches as f64;
    if hitches > HITCH_RATIO * peer_hitches + HITCH_SLACK {
        return Some(format!("{} hitches vs the arm's {peer_hitches:.0}", run.hitches));
    }
    None
}

/// Leave-one-out [`outlier_reason`] over an arm's runs (at least 3, so "the others" means something): the
/// index and reason of each outlier.
pub fn outliers(points: &[RunPoint]) -> Vec<(usize, String)> {
    if points.len() < 3 {
        return Vec::new();
    }
    (0..points.len())
        .filter_map(|i| {
            let peers: Vec<RunPoint> = points.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, p)| *p).collect();
            outlier_reason(points[i], &peers).map(|why| (i, why))
        })
        .collect()
}

/// A two-sided Mann-Whitney U test of `a` against `b`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MannWhitney {
    /// U of `a`: pairs (x in a, y in b) with x > y, ties counting one half.
    pub u: f64,
    /// Two-sided p-value.
    pub p: f64,
    /// Exact distribution (no ties, both samples ≤ [`EXACT_MAX`]) or the normal approximation (tie and
    /// continuity corrected).
    pub exact: bool,
}

/// Largest sample size for the exact U distribution.
pub const EXACT_MAX: usize = 20;

/// Mann-Whitney U of `a` vs `b` (`None` when either is empty). Midranks for ties; exact p without ties when
/// both samples are small, else the normal approximation with tie correction and continuity correction.
pub fn mann_whitney(a: &[f64], b: &[f64]) -> Option<MannWhitney> {
    let (n1, n2) = (a.len(), b.len());
    if n1 == 0 || n2 == 0 {
        return None;
    }
    let mut all: Vec<(f64, bool)> = a.iter().map(|&v| (v, true)).chain(b.iter().map(|&v| (v, false))).collect();
    all.sort_by(|x, y| x.0.total_cmp(&y.0));
    let total_n = all.len();
    let mut rank_a = 0.0;
    let mut tie_term = 0.0;
    let mut has_ties = false;
    let mut start = 0;
    while start < total_n {
        let mut end = start;
        while end + 1 < total_n && all[end + 1].0.total_cmp(&all[start].0).is_eq() {
            end += 1;
        }
        let in_a = all[start..=end].iter().filter(|e| e.1).count();
        has_ties |= end > start;
        #[allow(clippy::cast_precision_loss)] // ranks and counts
        let (midrank, group, in_a) = ((start + end + 2) as f64 / 2.0, (end - start + 1) as f64, in_a as f64);
        tie_term += group * group * group - group;
        rank_a += midrank * in_a;
        start = end + 1;
    }
    #[allow(clippy::cast_precision_loss)]
    let (f1, f2, fnn) = (n1 as f64, n2 as f64, total_n as f64);
    let u = rank_a - f1 * (f1 + 1.0) / 2.0;
    let mean = f1 * f2 / 2.0;
    if !has_ties && n1 <= EXACT_MAX && n2 <= EXACT_MAX {
        // P(U <= u_low) from the count of arrangements per U value.
        let u_low = u.min(f1 * f2 - u);
        let counts = u_counts(n1, n2);
        let total: f64 = counts.iter().sum();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // u_low is a whole number >= 0 here
        let tail: f64 = counts.iter().take(u_low as usize + 1).sum();
        return Some(MannWhitney { u, p: (2.0 * tail / total).min(1.0), exact: true });
    }
    let var = f1 * f2 / 12.0 * ((fnn + 1.0) - tie_term / (fnn * (fnn - 1.0)));
    if var <= 0.0 {
        return Some(MannWhitney { u, p: 1.0, exact: false });
    }
    let z = ((u - mean).abs() - 0.5).max(0.0) / var.sqrt();
    Some(MannWhitney { u, p: erfc(z / std::f64::consts::SQRT_2).min(1.0), exact: false })
}

/// `counts[u]`: arrangements of `n1` + `n2` distinct values giving U = u (the recursion
/// f(m, n, u) = f(m - 1, n, u - n) + f(m, n - 1, u)).
fn u_counts(n1: usize, n2: usize) -> Vec<f64> {
    // table[m][n] = distribution for sizes m, n
    let max_u = n1 * n2;
    let mut table: Vec<Vec<Vec<f64>>> = vec![vec![Vec::new(); n2 + 1]; n1 + 1];
    for m in 0..=n1 {
        for n in 0..=n2 {
            let mut d = vec![0.0; m * n + 1];
            if m == 0 || n == 0 {
                d[0] = 1.0;
            } else {
                for (u, slot) in d.iter_mut().enumerate() {
                    let from_a = if u >= n { table[m - 1][n].get(u - n).copied().unwrap_or(0.0) } else { 0.0 };
                    let from_b = table[m][n - 1].get(u).copied().unwrap_or(0.0);
                    *slot = from_a + from_b;
                }
            }
            table[m][n] = d;
        }
    }
    let mut out = std::mem::take(&mut table[n1][n2]);
    out.resize(max_u + 1, 0.0);
    out
}

/// Complementary error function (Numerical Recipes `erfcc`, fractional error < 1.2e-7).
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let poly = -z * z - 1.265_512_23
        + t * (1.000_023_68
            + t * (0.374_091_96
                + t * (0.096_784_18
                    + t * (-0.186_288_06
                        + t * (0.278_868_07 + t * (-1.135_203_98 + t * (1.488_515_87 + t * (-0.822_152_23 + t * 0.170_872_77))))))));
    let r = t * poly.exp();
    if x >= 0.0 { r } else { 2.0 - r }
}

/// The outcome of an arm against the base arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Faster,
    Slower,
    Inconclusive,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Faster => "faster",
            Self::Slower => "slower",
            Self::Inconclusive => "inconclusive",
        })
    }
}

/// An arm's run medians against the base arm's.
#[derive(Clone, Debug, PartialEq)]
pub struct Verdict {
    /// Median of the arm's run medians minus the base's, ms (negative = faster).
    pub delta_ms: f64,
    pub base_range: (f64, f64),
    pub arm_range: (f64, f64),
    pub test: MannWhitney,
    pub outcome: Outcome,
}

/// Significance level of the decision rule.
pub const ALPHA: f64 = 0.05;

/// The decision rule (audit §6, written before the runs): an arm differs from the base when the median delta
/// exceeds the larger arm's min-max range, or the Mann-Whitney p < [`ALPHA`]; anything else is inconclusive.
/// `None` when either side has no runs.
pub fn verdict(base: &[f64], arm: &[f64]) -> Option<Verdict> {
    let test = mann_whitney(arm, base)?;
    let range = |v: &[f64]| v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| (lo.min(x), hi.max(x)));
    let (base_range, arm_range) = (range(base), range(arm));
    let delta_ms = median_of(arm.to_vec())? - median_of(base.to_vec())?;
    let spread = (base_range.1 - base_range.0).max(arm_range.1 - arm_range.0);
    let outcome = if delta_ms.abs() > spread || test.p < ALPHA {
        if delta_ms < 0.0 { Outcome::Faster } else { Outcome::Slower }
    } else {
        Outcome::Inconclusive
    };
    Some(Verdict { delta_ms, base_range, arm_range, test, outcome })
}

/// Median of `values` (`None` when empty): the summary statistic of an arm's runs.
pub fn median(values: &[f64]) -> Option<f64> {
    median_of(values.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    /// Each arm runs equally often at each position, every block is a permutation, the plan is deterministic.
    fn check_balanced(arms: usize, repeats: usize) {
        let orders = interleaved_orders(arms, repeats, 42);
        assert_eq!(orders.len(), repeats);
        for row in &orders {
            let mut sorted = row.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, (0..arms).collect::<Vec<_>>(), "each repeat runs every arm once: {row:?}");
        }
        for pos in 0..arms {
            for arm in 0..arms {
                let n = orders.iter().filter(|r| r[pos] == arm).count();
                assert_eq!(n, repeats / arms, "arm {arm} at position {pos} in {orders:?}");
            }
        }
        assert_eq!(orders, interleaved_orders(arms, repeats, 42), "same seed, same plan");
    }

    #[test]
    fn plans_are_counterbalanced() {
        check_balanced(2, 4);
        check_balanced(2, 6);
        check_balanced(3, 6);
        check_balanced(3, 3);
        check_balanced(5, 10);
        check_balanced(5, 5);
        // Two arms: every pair of repeats is ABBA or BAAB.
        for seed in 0..20 {
            let o = interleaved_orders(2, 4, seed);
            for pair in o.chunks(2) {
                assert!(pair == [vec![0, 1], vec![1, 0]] || pair == [vec![1, 0], vec![0, 1]], "{o:?}");
            }
        }
        // The seed changes the order.
        assert!((0..20).map(|s| interleaved_orders(3, 6, s)).any(|o| o != interleaved_orders(3, 6, 0)));
        assert!(interleaved_orders(0, 3, 1).is_empty());
        assert_eq!(interleaved_orders(1, 2, 1), [vec![0], vec![0]]);
    }

    #[test]
    fn odd_arm_counts_balance_carry_over_over_both_squares() {
        // 3 arms, 6 repeats: each ordered pair (a then b, a != b) follows once per square.
        let orders = interleaved_orders(3, 6, 7);
        for a in 0..3 {
            for b in (0..3).filter(|&b| b != a) {
                let n = orders.iter().filter(|r| r.windows(2).any(|w| w == [a, b])).count();
                assert_eq!(n, 2, "{a}->{b} in {orders:?}");
            }
        }
    }

    #[test]
    fn outliers_slow_runs_and_hitch_storms() {
        let p = |median_ms, hitches| RunPoint { median_ms, hitches };
        let arm = [p(10.0, 2), p(10.4, 3), p(10.2, 1)];
        assert_eq!(outlier_reason(p(10.6, 4), &arm), None);
        // 51.7 ms among ~10 ms runs (the `slab2_off` #3 of the audit).
        assert!(outlier_reason(p(51.7, 2), &arm).unwrap().starts_with("median 51.70 ms > 1.3 x the arm's 10.20 ms"));
        // 13.3 = 1.304 x 10.2: flagged; 13.2 is not.
        assert!(outlier_reason(p(13.3, 2), &arm).is_some());
        assert!(outlier_reason(p(13.2, 2), &arm).is_none());
        // Hitches: 3 x 2 + 10 = 16 allowed, 17 explode.
        assert!(outlier_reason(p(10.0, 16), &arm).is_none());
        assert_eq!(outlier_reason(p(10.0, 17), &arm).as_deref(), Some("17 hitches vs the arm's 2"));
        // Fast runs are never flagged; no peers, no verdict.
        assert!(outlier_reason(p(5.0, 0), &arm).is_none());
        assert!(outlier_reason(p(99.0, 0), &[]).is_none());
        // Leave-one-out: the garbage first run is found too.
        assert_eq!(outliers(&[p(67.9, 2), p(10.0, 2), p(10.2, 2)]).iter().map(|o| o.0).collect::<Vec<_>>(), [0]);
        assert!(outliers(&[p(67.9, 2), p(10.0, 2)]).is_empty(), "two runs: no leave-one-out");
    }

    #[test]
    fn mann_whitney_exact_matches_hand_computed() {
        // Complete separation, 3 vs 3: U = 0, P(U <= 0) = 1/C(6,3) = 1/20, two-sided 0.1.
        let t = mann_whitney(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap();
        assert!(t.exact && close(t.u, 0.0, 1e-12) && close(t.p, 0.1, 1e-12), "{t:?}");
        // 4 vs 4: 2/70.
        let t = mann_whitney(&[5.0, 6.0, 7.0, 8.0], &[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert!(close(t.u, 16.0, 1e-12) && close(t.p, 2.0 / 70.0, 1e-12), "{t:?}");
        // a = [1, 4], b = [2, 3]: U = 2 (4 beats both), U distribution over C(4,2) = 6 arrangements
        // is [1, 1, 2, 1, 1]; P(U <= 2) = 4/6, two-sided capped at 1.
        let t = mann_whitney(&[1.0, 4.0], &[2.0, 3.0]).unwrap();
        assert!(close(t.u, 2.0, 1e-12) && close(t.p, 1.0, 1e-12), "{t:?}");
        // a = [1, 2, 5], b = [3, 4, 6]: U_a = 2 (5 beats 3 and 4);
        // distribution for 3 vs 3: [1, 1, 2, 3, 3, 3, 3, 2, 1, 1]; P(U <= 2) = 4/20, two-sided 0.4.
        let t = mann_whitney(&[1.0, 2.0, 5.0], &[3.0, 4.0, 6.0]).unwrap();
        assert!(close(t.u, 2.0, 1e-12) && close(t.p, 0.4, 1e-12), "{t:?}");
        assert!(mann_whitney(&[], &[1.0]).is_none());
    }

    #[test]
    fn mann_whitney_ties_use_the_corrected_normal_approximation() {
        // a = [1, 2, 2], b = [2, 3, 4]: midranks 1, 3, 3 | 3, 5, 6; R_a = 7, U = 7 - 6 = 1.
        // Ties: one group of 3, sum(t^3 - t) = 24. var = 9/12 * (7 - 24/30) = 4.65, z = (|1 - 4.5| - 0.5)/2.1564
        // = 1.3912, p = 2 * (1 - Phi(1.3912)) = 0.1642.
        let t = mann_whitney(&[1.0, 2.0, 2.0], &[2.0, 3.0, 4.0]).unwrap();
        assert!(!t.exact && close(t.u, 1.0, 1e-12), "{t:?}");
        assert!(close(t.p, 0.1642, 1e-3), "{t:?}");
        // All equal: no evidence at all.
        let t = mann_whitney(&[1.0, 1.0], &[1.0, 1.0]).unwrap();
        assert!(close(t.p, 1.0, 1e-12), "{t:?}");
        // Large separated samples: normal approximation, tiny p.
        let a: Vec<f64> = (0..25).map(f64::from).collect();
        let b: Vec<f64> = (100..125).map(f64::from).collect();
        let t = mann_whitney(&a, &b).unwrap();
        assert!(!t.exact && t.p < 1e-6, "{t:?}");
        // erfc reference points.
        assert!(close(erfc(0.0), 1.0, 1e-7) && close(erfc(1.0), 0.157_299_2, 1e-6) && close(erfc(-1.0), 1.842_700_8, 1e-6));
    }

    #[test]
    fn verdict_rule() {
        // Clear win: delta -1.0 beyond both ranges (0.3); 4 vs 4 separated, p = 0.029.
        let v = verdict(&[10.0, 10.1, 10.2, 10.3], &[9.0, 9.1, 9.2, 9.3]).unwrap();
        assert_eq!(v.outcome, Outcome::Faster);
        assert!(close(v.delta_ms, -1.0, 1e-9) && close(v.test.p, 2.0 / 70.0, 1e-12));
        assert_eq!((v.base_range, v.arm_range), ((10.0, 10.3), (9.0, 9.3)));
        // 3 vs 3 separated: p = 0.1 can't pass, but delta 0.5 > ranges 0.2: slower.
        let v = verdict(&[10.0, 10.1, 10.2], &[10.5, 10.6, 10.7]).unwrap();
        assert_eq!(v.outcome, Outcome::Slower);
        // 3 vs 3 separated with delta 0.3 inside a 0.4 range: inconclusive (n = 3 has no power).
        let v = verdict(&[10.0, 10.2, 10.4], &[10.5, 10.5001, 10.6]).unwrap();
        assert_eq!(v.outcome, Outcome::Inconclusive, "{v:?}");
        // Overlapping: inconclusive.
        let v = verdict(&[10.0, 10.4, 10.2, 10.6], &[10.1, 10.5, 10.3, 10.2]).unwrap();
        assert_eq!(v.outcome, Outcome::Inconclusive);
        // 6 vs 6, delta 0.325 inside the base range 0.9: U_low = 5, P(U <= 5) = 19/924, p = 0.041 < 0.05 wins.
        let v = verdict(&[10.0, 10.1, 10.2, 10.3, 10.4, 10.9], &[10.45, 10.5, 10.55, 10.6, 10.65, 10.95]).unwrap();
        assert!(v.test.p < 0.05 && v.outcome == Outcome::Slower, "{v:?}");
        assert!(verdict(&[], &[1.0]).is_none());
    }
}
