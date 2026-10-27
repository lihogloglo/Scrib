//! Reaction tiers and sides, and the sheet's (tier, side) -> forced move table (combat.md
//! "Reactions, ragdoll and Delirium").

use std::collections::BTreeMap;

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_action::RuleKey;

/// How hard a hit staggers, lowest first (ER's four tiers plus SK's additive minimum).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tier {
    /// No state change: a cosmetic flinch.
    #[default]
    Additive,
    Small,
    Medium,
    Large,
    Knockdown,
    Death,
}

impl Tier {
    pub const ALL: [Self; 6] = [Self::Additive, Self::Small, Self::Medium, Self::Large, Self::Knockdown, Self::Death];

    /// The next lower tier (`None` below `Additive`).
    pub fn lower(self) -> Option<Self> {
        let i = Self::ALL.iter().position(|&t| t == self)?;
        i.checked_sub(1).map(|j| Self::ALL[j])
    }
}

/// Where a hit came from, in the defender's frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Side {
    #[default]
    Front,
    Back,
    Left,
    Right,
}

impl Side {
    /// The side of a defender facing `yaw` (0 faces +Z) that `toward` (defender to attacker,
    /// world axes) points at: front and back within +-45 deg.
    pub fn of(yaw: f32, toward: Vec3) -> Self {
        let (fwd, right) = (Vec3::new(yaw.sin(), 0.0, yaw.cos()), Vec3::new(-yaw.cos(), 0.0, yaw.sin()));
        let (f, r) = (toward.dot(fwd), toward.dot(right));
        if f.abs() >= r.abs() {
            if f >= 0.0 { Self::Front } else { Self::Back }
        } else if r >= 0.0 {
            Self::Right
        } else {
            Self::Left
        }
    }

    /// Stable index for [`sf_action::Forced::side`].
    pub const fn index(self) -> u8 {
        self as u8
    }
}

/// The compiled reaction table of one moveset.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReactionTable {
    pub moves: BTreeMap<(Tier, Side), RuleKey>,
}

impl ReactionTable {
    /// The forced move for `(tier, side)`: a missing side falls back to `Front`, a missing tier to
    /// the next lower. `None`: no reaction (an `Additive` flinch, or nothing authored).
    pub fn lookup(&self, tier: Tier, side: Side) -> Option<RuleKey> {
        let mut t = Some(tier);
        while let Some(tier) = t {
            if let Some(&k) = self.moves.get(&(tier, side)).or_else(|| self.moves.get(&(tier, Side::Front))) {
                return Some(k);
            }
            t = tier.lower();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sides_and_fallbacks() {
        assert_eq!(Side::of(0.0, Vec3::Z), Side::Front);
        assert_eq!(Side::of(0.0, -Vec3::Z), Side::Back);
        assert_eq!(Side::of(0.0, -Vec3::X), Side::Right);
        assert_eq!(Side::of(std::f32::consts::FRAC_PI_2, Vec3::X), Side::Front);
        let mut t = ReactionTable::default();
        t.moves.insert((Tier::Small, Side::Front), RuleKey(1));
        t.moves.insert((Tier::Small, Side::Back), RuleKey(2));
        t.moves.insert((Tier::Large, Side::Front), RuleKey(3));
        assert_eq!(t.lookup(Tier::Small, Side::Left), Some(RuleKey(1)));
        assert_eq!(t.lookup(Tier::Small, Side::Back), Some(RuleKey(2)));
        assert_eq!(t.lookup(Tier::Medium, Side::Back), Some(RuleKey(2)));
        assert_eq!(t.lookup(Tier::Death, Side::Right), Some(RuleKey(3)));
        assert_eq!(t.lookup(Tier::Additive, Side::Front), None);
    }
}
