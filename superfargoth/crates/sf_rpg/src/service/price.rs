//! The framework's one price rule (services.md "Price seam"): fixed prices, no haggle.

use serde::{Deserialize, Serialize};

use super::Side;

/// Input of [`FixedPrice::price`]: everything the rule reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixedPriceIn {
    pub side: Side,
    /// The adapter's per-piece value (condition-scaled), `>= 0`.
    pub unit_value: i64,
    pub count: u32,
}

/// Buy at base value, sell at `floor(base * sell_permille / 1000)` per piece. Integer
/// arithmetic, so every host and client computes the same price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixedPrice {
    /// The sell ratio in thousandths (500 = half price).
    pub sell_permille: u32,
}

impl FixedPrice {
    /// The price of a whole line, `>= 0`.
    pub fn price(self, input: &FixedPriceIn) -> i64 {
        let unit = input.unit_value.max(0);
        let per_piece = match input.side {
            Side::Buy => unit,
            Side::Sell => unit.saturating_mul(i64::from(self.sell_permille)) / 1000,
        };
        per_piece.saturating_mul(i64::from(input.count))
    }
}
