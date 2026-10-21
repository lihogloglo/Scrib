//! A tiny deterministic generator for presentation draws (variants, intervals, shuffles).
//! Audio is local presentation (net.md), so peers need not agree; determinism is for tests.

/// xorshift64* (Vigna): fast, small state, good enough for picking sounds.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    /// A float in [0, 1) from the top 24 bits.
    pub fn unit(&mut self) -> f32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let v = (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 40) as u32;
        #[expect(clippy::cast_precision_loss, reason = "24 bits")]
        let f = v as f32 / 16_777_216.0;
        f
    }

    /// Uniform in `[min(a, b), max(a, b))`; `a` when the range is empty.
    pub fn range(&mut self, a: f32, b: f32) -> f32 {
        let (lo, hi) = (a.min(b), a.max(b));
        lo + (hi - lo) * self.unit()
    }

    /// An index in `0..n` (0 when `n` is 0).
    pub fn index(&mut self, n: usize) -> usize {
        #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "small n")]
        let i = (self.unit() * n as f32) as usize;
        i.min(n.saturating_sub(1))
    }
}
