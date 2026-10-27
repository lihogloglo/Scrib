//! Hit-stop (combat.md "The per-tick pipeline"): the action clock's rate is 0 for `hold`, then
//! eases back as `1 - (1 - u)^2` over `ease`, so windows, root motion, slot progress and sweeps
//! freeze together while input still latches. Integer ticks: identical on every peer.

use serde::{Deserialize, Serialize};
use sf_action::{RATE_ONE, TICK_HZ};

/// A running hit-stop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitStop {
    /// Ticks since it started (0 on the landing tick).
    pub t: u16,
    pub hold: u16,
    pub ease: u16,
}

/// Seconds to whole ticks (rounded, clamped to `u16`).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn ticks(secs: f32) -> u16 {
    (secs.max(0.0) * TICK_HZ as f32).round().min(f32::from(u16::MAX)) as u16
}

impl HitStop {
    pub fn new(hold_s: f32, ease_s: f32) -> Self {
        Self { t: 0, hold: ticks(hold_s), ease: ticks(ease_s) }
    }

    /// The clock rate for the tick after `t` ticks (thousandths).
    pub fn rate(self) -> u16 {
        if self.t < self.hold {
            return 0;
        }
        let e = self.t - self.hold;
        if e >= self.ease {
            return RATE_ONE;
        }
        // Ease tick `e` of `ease` sits at `(e + 1) / (ease + 1)`: none of them is a second hold
        // tick (u = 0) or already full rate (u = 1), so the clock runs slow for exactly `len()`.
        let u = f32::from(e + 1) / (f32::from(self.ease) + 1.0);
        let r = 1.0 - (1.0 - u) * (1.0 - u);
        // In 0..=1000 by construction.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let rate = (r * f32::from(RATE_ONE)).round() as u16;
        rate
    }

    /// Over once the clock is back at full rate.
    pub fn done(self) -> bool {
        self.t >= self.hold.saturating_add(self.ease)
    }

    /// Total ticks the clock runs slow.
    pub fn len(self) -> u16 {
        self.hold.saturating_add(self.ease)
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_then_eases_monotonically() {
        let mut s = HitStop::new(0.05, 0.10);
        assert_eq!((s.hold, s.ease), (3, 6));
        let mut rates = Vec::new();
        while !s.done() {
            rates.push(s.rate());
            s.t += 1;
        }
        assert_eq!(rates.len(), usize::from(s.len()));
        assert_eq!(&rates[..3], &[0, 0, 0]);
        assert!(rates[3] > 0, "{rates:?}");
        assert!(rates.windows(2).all(|w| w[0] <= w[1]), "{rates:?}");
        assert!(rates[3..].iter().all(|&r| r < RATE_ONE));
        assert_eq!(s.rate(), RATE_ONE);
    }
}
