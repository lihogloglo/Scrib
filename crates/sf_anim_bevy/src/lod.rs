//! Animation LOD tiers A0–A3 and the budget allocator (animation.md "LOD
//! and budget"). Pure functions; the Bevy systems in `lib.rs` feed them.
//!
//! Tiers come from projected height in pixels, normalised to the spec's
//! reference view (1440 px tall, so the thresholds hold at any resolution),
//! times a significance factor. The allocator then demotes the least
//! significant characters one tier at a time until the predicted cost (a
//! measured per-tier EMA) fits the budget, the way UE's Animation Budget
//! Allocator throttles its least significant components first.

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

/// An animation LOD tier. The graph runtime's `lod` argument is the tier
/// index: 0–2 sample the skeleton's LOD joint masks, 3 advances clocks,
/// events and root motion only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnimTier {
    /// Every frame, full graph, all procedural ops, all joints.
    #[default]
    A0,
    /// Every frame, full graph, speed match/stride/feet, LOD1 joint mask.
    A1,
    /// Every `A2_INTERVAL` frames (staggered), full graph, LOD2 joint mask.
    A2,
    /// Clocks, events and root motion only; the pose is frozen.
    A3,
}

impl AnimTier {
    pub const ALL: [Self; 4] = [Self::A0, Self::A1, Self::A2, Self::A3];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i.min(3)]
    }

    /// One tier cheaper (A3 stays A3).
    #[must_use]
    pub fn demoted(self) -> Self {
        Self::from_index(self.index() + 1)
    }

    /// Frames between pose updates. A3 still ticks every frame (clocks and
    /// events are cheap), it just never poses.
    pub fn interval(self) -> u32 {
        match self {
            Self::A2 => A2_INTERVAL,
            _ => 1,
        }
    }
}

/// Frames between A2 pose updates (the spec allows 2–4).
pub const A2_INTERVAL: u32 = 2;

/// The tier thresholds and allocator settings.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct AnimLodSettings {
    /// Minimum reference pixels for A0, A1, A2 (below: A3).
    pub pixels: [f32; 3],
    /// Promotion needs `pixels × (1 + hysteresis)` (no tier flicker at a boundary).
    pub hysteresis: f32,
    /// Animation budget per frame, µs of evaluation summed over threads.
    /// `None`: tiers come from distance alone.
    pub budget_us: Option<f32>,
    /// Off-screen characters drop to A3 (shadow casters should count as A2:
    /// the host sets `AnimLod::significance` or forces a tier for those).
    pub cull_offscreen: bool,
    /// Reference view height in pixels the thresholds are given for.
    pub reference_height: f32,
}

impl Default for AnimLodSettings {
    fn default() -> Self {
        Self {
            pixels: [120.0, 50.0, 20.0],
            hysteresis: 0.15,
            budget_us: None,
            cull_offscreen: true,
            reference_height: 1440.0,
        }
    }
}

impl AnimLodSettings {
    /// The tier for `pixels` (already times significance), with hysteresis
    /// against `current`: a character moves to a richer tier only once it
    /// clears that tier's threshold by the hysteresis margin.
    pub fn tier(&self, pixels: f32, current: AnimTier) -> AnimTier {
        let plain = self
            .pixels
            .iter()
            .position(|&t| pixels >= t)
            .map_or(AnimTier::A3, AnimTier::from_index);
        if plain >= current {
            return plain;
        }
        let promoted = self
            .pixels
            .iter()
            .position(|&t| pixels >= t * (1.0 + self.hysteresis))
            .map_or(AnimTier::A3, AnimTier::from_index);
        promoted.min(current)
    }
}

/// Per character: the tier in use and what chose it.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct AnimLod {
    pub tier: AnimTier,
    /// Pins the tier (dialogue partner, the player, tests).
    pub forced: Option<AnimTier>,
    /// Multiplies projected pixels: 2 in combat, for the player's target…
    pub significance: f32,
    /// Bounding height of the character in metres (projected-size input).
    pub height: f32,
    /// Last projected height, reference pixels, before significance.
    pub pixels: f32,
    /// Seconds not yet given to the graph (A2 frames between updates).
    pub(crate) pending: f32,
    /// Frame offset for staggered A2 updates.
    pub(crate) phase: u32,
    /// Whether this frame updates the graph (and how).
    pub(crate) due: bool,
}

impl AnimLod {
    pub fn new(height: f32) -> Self {
        Self {
            significance: 1.0,
            height,
            ..Self::default()
        }
    }

    pub fn forced(tier: AnimTier) -> Self {
        Self {
            forced: Some(tier),
            significance: 1.0,
            height: 1.8,
            ..Self::default()
        }
    }
}

/// Projected height of `height` metres at `distance`, in reference pixels,
/// for a vertical field of view `fov_y` (radians).
pub fn projected_pixels(height: f32, distance: f32, fov_y: f32, reference_height: f32) -> f32 {
    let span = 2.0 * distance.max(0.01) * (fov_y * 0.5).tan();
    height / span * reference_height
}

/// Measured cost per character per tier (µs, EMA) and the last allocation.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct AnimBudget {
    /// µs per character per evaluated update, by tier.
    pub cost_us: [f32; 4],
    /// Predicted cost of the last allocation.
    pub predicted_us: f32,
    /// Characters the allocator demoted below their distance tier.
    pub demoted: usize,
}

impl Default for AnimBudget {
    fn default() -> Self {
        // Seeds until the first measurements (spec targets: A0 ≤ 25 µs).
        Self {
            cost_us: [25.0, 15.0, 8.0, 1.0],
            predicted_us: 0.0,
            demoted: 0,
        }
    }
}

impl AnimBudget {
    /// Folds one frame's mean cost per tier into the EMA.
    pub fn observe(&mut self, tier: AnimTier, mean_us: f32) {
        let c = &mut self.cost_us[tier.index()];
        *c += (mean_us - *c) * 0.1;
    }

    /// Cost of a tier per frame: A2 updates every `interval` frames.
    pub fn frame_cost(&self, tier: AnimTier) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let interval = tier.interval() as f32;
        self.cost_us[tier.index()] / interval
    }
}

/// Demotes the least significant characters one tier per round until the
/// predicted cost fits `budget_us` (or everyone is at A3). `chars` holds
/// `(score, tier)`; tiers are updated in place. Forced characters should be
/// left out by the caller. Returns the predicted cost and the demotions.
pub fn allocate(
    chars: &mut [(f32, AnimTier)],
    budget: &AnimBudget,
    budget_us: f32,
) -> (f32, usize) {
    let mut total: f32 = chars.iter().map(|&(_, t)| budget.frame_cost(t)).sum();
    let mut order: Vec<usize> = (0..chars.len()).collect();
    order.sort_by(|&a, &b| chars[a].0.total_cmp(&chars[b].0));
    let mut demoted = 0;
    while total > budget_us {
        let mut changed = false;
        for &i in &order {
            if total <= budget_us {
                break;
            }
            let t = chars[i].1;
            if t == AnimTier::A3 {
                continue;
            }
            let d = t.demoted();
            total += budget.frame_cost(d) - budget.frame_cost(t);
            chars[i].1 = d;
            demoted += 1;
            changed = true;
        }
        if !changed {
            break;
        }
    }
    (total, demoted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiers_follow_pixels_with_hysteresis() {
        let s = AnimLodSettings::default();
        assert_eq!(s.tier(200.0, AnimTier::A0), AnimTier::A0);
        assert_eq!(s.tier(100.0, AnimTier::A0), AnimTier::A1);
        assert_eq!(s.tier(10.0, AnimTier::A0), AnimTier::A3);
        // From A1, 125 px is above 120 but inside the margin: stays A1.
        assert_eq!(s.tier(125.0, AnimTier::A1), AnimTier::A1);
        assert_eq!(s.tier(140.0, AnimTier::A1), AnimTier::A0);
        // A 1.8 m character at 15 m, 70° fov: about the A0 threshold.
        let px = projected_pixels(1.8, 15.0, 70f32.to_radians(), 1440.0);
        assert!((px - 123.0).abs() < 3.0, "{px}");
    }

    #[test]
    fn allocator_demotes_least_significant_first() {
        let budget = AnimBudget {
            cost_us: [20.0, 10.0, 6.0, 1.0],
            ..AnimBudget::default()
        };
        let mut chars = vec![
            (500.0, AnimTier::A0),
            (100.0, AnimTier::A0),
            (50.0, AnimTier::A0),
        ];
        let (total, demoted) = allocate(&mut chars, &budget, 40.0);
        assert!(total <= 40.0, "{total}");
        assert_eq!(chars[0].1, AnimTier::A0, "the most significant keeps A0");
        assert_eq!(
            (chars[1].1, chars[2].1, demoted),
            (AnimTier::A1, AnimTier::A1, 2),
            "one tier per round, least significant first"
        );
        // Nothing fits: everyone ends at A3, no endless loop.
        let mut chars = vec![(1.0, AnimTier::A0); 4];
        let (total, _) = allocate(&mut chars, &budget, 0.5);
        assert!(chars.iter().all(|c| c.1 == AnimTier::A3));
        assert!((total - 4.0).abs() < 1e-4);
    }
}
