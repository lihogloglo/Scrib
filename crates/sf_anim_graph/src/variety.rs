//! Idle and crowd variety: characters that share a skeleton and clips must
//! not move in lockstep (animation.md "Idle and crowd variety").
//!
//! Two small deterministic pieces, both driven by a per-character seed (a
//! stable id such as a placement id, never an entity index, so a character
//! breathes the same way after a reload):
//!
//! - [`Variety`]: a start phase offset in `[0, 1)` of the cycle and a
//!   playback rate jitter (default ±5 %). The graph instance applies the
//!   phase to every looping clip it starts and the rate to looping clips
//!   that do not drive root motion; locomotion keeps its foot-speed match
//!   (the speed-match op divides the jitter back out), so nothing slides.
//!   This is UE's random start position and Unity's cycle offset.
//! - [`FidgetPicker`]: when to play an idle fidget and which. Weighted by
//!   the given chances, a gap between `min_interval` and `max_interval`
//!   seconds, and never the same fidget twice in a row (unless it is the
//!   only candidate).

/// Default rate jitter: the rate is `1 ± 0.05`.
pub const DEFAULT_RATE_JITTER: f32 = 0.05;

/// The splitmix64 finalizer: a bijective scramble, so adjacent seeds (ids
/// 1, 2, 3) give unrelated outputs.
#[must_use]
pub fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Folds a 128-bit id (a placement id) into a seed.
#[must_use]
pub fn seed_from_u128(id: u128) -> u64 {
    #[allow(clippy::cast_possible_truncation)]
    let (lo, hi) = (id as u64, (id >> 64) as u64);
    mix64(lo ^ mix64(hi))
}

/// A uniform value in `[0, 1)` from the top 24 bits.
#[allow(clippy::cast_precision_loss)]
fn unit(bits: u64) -> f32 {
    (bits >> 40) as f32 / (1u32 << 24) as f32
}

/// One character's phase offset and rate scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Variety {
    /// Start phase offset, `[0, 1)` of a looping clip's cycle.
    pub phase: f32,
    /// Playback rate factor, `1 ± jitter`.
    pub rate: f32,
}

impl Default for Variety {
    /// No variety: every clip starts at its authored phase and rate.
    fn default() -> Self {
        Self { phase: 0.0, rate: 1.0 }
    }
}

impl Variety {
    /// The variety of the character with `seed`; `rate_jitter` is the
    /// largest relative rate change (0.05 = ±5 %, 0 = none).
    #[must_use]
    pub fn from_seed(seed: u64, rate_jitter: f32) -> Self {
        let h = mix64(seed);
        let r = unit(mix64(h ^ 0xA5A5_A5A5_5A5A_5A5A));
        Self { phase: unit(h), rate: 1.0 + rate_jitter.abs() * (2.0 * r - 1.0) }
    }
}

/// The smallest circular distance between two phases in `[0, 1)`.
#[must_use]
pub fn phase_distance(a: f32, b: f32) -> f32 {
    let d = (a - b).abs().rem_euclid(1.0);
    d.min(1.0 - d)
}

/// Picks fidgets for one character. Call [`FidgetPicker::update`] every
/// frame the character stands; it returns the fidget to play, if one is due.
#[derive(Clone, Debug)]
pub struct FidgetPicker {
    weights: Vec<f32>,
    min_interval: f32,
    max_interval: f32,
    rng: u64,
    last: Option<usize>,
    wait: f32,
}

impl FidgetPicker {
    /// `weights[i]` is the chance of fidget `i` (relative, 0 = never).
    /// Gaps between fidgets are uniform in `[min_interval, max_interval]`
    /// seconds; the first gap is too, so characters spawned together don't
    /// fidget together. `seed` makes the sequence deterministic.
    #[must_use]
    pub fn new(weights: &[f32], min_interval: f32, max_interval: f32, seed: u64) -> Self {
        let min_interval = min_interval.max(0.0);
        let mut picker = Self {
            weights: weights.iter().map(|w| w.max(0.0)).collect(),
            min_interval,
            max_interval: max_interval.max(min_interval),
            rng: mix64(seed ^ 0xF1D6_E7C0_FFEE_0001),
            last: None,
            wait: 0.0,
        };
        picker.wait = picker.gap();
        picker
    }

    fn next_unit(&mut self) -> f32 {
        self.rng = mix64(self.rng);
        unit(self.rng)
    }

    fn gap(&mut self) -> f32 {
        self.min_interval + (self.max_interval - self.min_interval) * self.next_unit()
    }

    /// The last fidget picked.
    #[must_use]
    pub fn last(&self) -> Option<usize> {
        self.last
    }

    /// Picks a fidget now: weighted, never the last one while another has a
    /// chance. `None` when no fidget has a positive weight.
    pub fn pick(&mut self) -> Option<usize> {
        let last = self.last;
        let excluded = |i: usize| Some(i) == last;
        let others: f32 = self.weights.iter().enumerate().filter(|&(i, _)| !excluded(i)).map(|(_, w)| w).sum();
        // The only candidate may repeat: standing still is the alternative.
        let skip_last = others > 0.0;
        let total = if skip_last { others } else { self.weights.iter().sum() };
        if total <= 0.0 {
            return None;
        }
        let mut x = self.next_unit() * total;
        let mut chosen = None;
        for (i, &w) in self.weights.iter().enumerate() {
            if w <= 0.0 || (skip_last && excluded(i)) {
                continue;
            }
            chosen = Some(i);
            x -= w;
            if x < 0.0 {
                break;
            }
        }
        self.last = chosen;
        chosen
    }

    /// Advances the gap by `dt` seconds; when it runs out, picks a fidget and
    /// starts the next gap.
    pub fn update(&mut self, dt: f32) -> Option<usize> {
        self.wait -= dt;
        if self.wait > 0.0 {
            return None;
        }
        self.wait = self.gap();
        self.pick()
    }

    /// Starts a fresh gap (the character moved), keeping the last pick so a
    /// fidget still never repeats back to back.
    pub fn restart(&mut self) {
        self.wait = self.gap();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::needless_range_loop)]

    use super::*;

    #[test]
    fn deterministic_per_seed() {
        for seed in [0, 1, 42, u64::MAX] {
            assert_eq!(Variety::from_seed(seed, 0.05), Variety::from_seed(seed, 0.05));
        }
        let run = |seed| {
            let mut p = FidgetPicker::new(&[1.0, 2.0, 3.0], 2.0, 6.0, seed);
            (0..50).map(|_| p.update(0.5)).collect::<Vec<_>>()
        };
        assert_eq!(run(7), run(7));
        assert_ne!(run(7), run(8));
    }

    #[test]
    fn variety_in_range_and_default_is_identity() {
        assert_eq!(Variety::default(), Variety { phase: 0.0, rate: 1.0 });
        for seed in 0..10_000 {
            let v = Variety::from_seed(seed, 0.05);
            assert!((0.0..1.0).contains(&v.phase), "phase {}", v.phase);
            assert!((0.95..=1.05).contains(&v.rate), "rate {}", v.rate);
            assert!((Variety::from_seed(seed, 0.0).rate - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn phases_are_uniform_and_rates_cover_the_band() {
        let n = 20_000u64;
        let mut buckets = [0u32; 10];
        let (mut lo, mut hi) = (f32::MAX, f32::MIN);
        for seed in 0..n {
            let v = Variety::from_seed(seed, 0.05);
            buckets[(v.phase * 10.0) as usize] += 1;
            lo = lo.min(v.rate);
            hi = hi.max(v.rate);
        }
        for (i, &c) in buckets.iter().enumerate() {
            let share = f64::from(c) / n as f64;
            assert!((share - 0.1).abs() < 0.01, "bucket {i}: {share}");
        }
        assert!(lo < 0.951 && hi > 1.049, "rates {lo}..{hi}");
    }

    #[test]
    fn neighbouring_seeds_do_not_start_in_phase() {
        // Adjacent ids (the placements of one street) are unrelated: close
        // pairs (< 0.05 of a cycle apart, the rate band's worth) occur at
        // about the chance rate of 10 %, never in a run.
        let n = 10_000u64;
        let close = (0..n).filter(|&s| phase_distance(Variety::from_seed(s, 0.05).phase, Variety::from_seed(s + 1, 0.05).phase) < 0.05).count();
        let share = close as f64 / n as f64;
        assert!((0.07..0.13).contains(&share), "close share {share}");
    }

    #[test]
    fn id_seeds_differ_for_adjacent_ids() {
        let a = seed_from_u128(1);
        let b = seed_from_u128(2);
        assert_ne!(a, b);
        assert_ne!(seed_from_u128(1 << 64), seed_from_u128(1));
    }

    #[test]
    fn fidget_never_repeats_back_to_back() {
        for seed in 0..5 {
            let mut p = FidgetPicker::new(&[5.0, 1.0, 1.0], 1.0, 2.0, seed);
            let mut prev = None;
            for i in 0..1000 {
                let f = p.pick().expect("candidates");
                assert_ne!(Some(f), prev, "seed {seed}, pick {i}");
                prev = Some(f);
            }
        }
    }

    #[test]
    fn fidget_weights_are_respected() {
        // Excluding the last pick shifts the shares, but a heavier weight
        // must still win more often, and a zero weight never.
        let mut p = FidgetPicker::new(&[6.0, 3.0, 1.0, 0.0], 1.0, 2.0, 99);
        let mut counts = [0u32; 4];
        for _ in 0..20_000 {
            counts[p.pick().expect("candidates")] += 1;
        }
        assert_eq!(counts[3], 0);
        assert!(counts[0] > counts[1] && counts[1] > counts[2], "{counts:?}");
        // Without the last pick: P(i | last = j) = w_i / (W - w_j). Solve the
        // stationary shares for [6, 3, 1] by iterating the chain.
        let w = [6.0f64, 3.0, 1.0];
        let mut pi = [1.0 / 3.0; 3];
        for _ in 0..200 {
            let mut next = [0.0; 3];
            for j in 0..3 {
                let rest: f64 = (0..3).filter(|&k| k != j).map(|k| w[k]).sum();
                for i in (0..3).filter(|&i| i != j) {
                    next[i] += pi[j] * w[i] / rest;
                }
            }
            pi = next;
        }
        for i in 0..3 {
            let share = f64::from(counts[i]) / 20_000.0;
            assert!((share - pi[i]).abs() < 0.02, "fidget {i}: {share} vs {}", pi[i]);
        }
    }

    #[test]
    fn single_candidate_may_repeat_and_empty_picks_nothing() {
        let mut p = FidgetPicker::new(&[0.0, 2.0], 1.0, 2.0, 1);
        assert_eq!(p.pick(), Some(1));
        assert_eq!(p.pick(), Some(1));
        assert_eq!(FidgetPicker::new(&[0.0, 0.0], 1.0, 2.0, 1).pick(), None);
        assert_eq!(FidgetPicker::new(&[], 1.0, 2.0, 1).pick(), None);
    }

    #[test]
    fn gaps_stay_within_the_interval() {
        let mut p = FidgetPicker::new(&[1.0, 1.0], 3.0, 8.0, 5);
        let dt = 0.05;
        let (mut since, mut fired) = (0.0f32, 0);
        for _ in 0..40_000 {
            since += dt;
            if p.update(dt).is_some() {
                // The first gap is also in [min, max] (plus a frame of slack).
                assert!(since >= 3.0 - 1e-3 && since <= 8.0 + dt + 1e-3, "gap {since}");
                since = 0.0;
                fired += 1;
            }
        }
        assert!(fired > 300, "fired {fired}");
        // Two characters with different seeds do not fidget on the same frame.
        let mut a = FidgetPicker::new(&[1.0, 1.0], 3.0, 8.0, 1);
        let mut b = FidgetPicker::new(&[1.0, 1.0], 3.0, 8.0, 2);
        let same = (0..2000).filter(|_| a.update(dt).is_some() && b.update(dt).is_some()).count();
        assert!(same < 5, "{same}");
    }
}
