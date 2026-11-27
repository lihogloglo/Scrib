//! **Reachability flood core** (spec `sites.md` "Design" WG-V1, slice WG-V1.1): a cost-distance flood over a walking
//! grid, the GIS cost-distance technique with Tobler's (1993) hiking function. Bevy-free and game-agnostic: the
//! caller samples its final ground into a [`ReachGround`] and names the source cells (towns, road ribbons); site
//! verdicts are WG-V1.2.
//!
//! **Frame.** Guide frame metres, x east, y north; cell (0, 0) is centred on `origin_m`, row 0 is the north edge
//! ([`crate::guide`]). Cell `(cx, cy)` is centred on `(origin_x + cx * cell_m, origin_y - cy * cell_m)`.
//!
//! **Step rule** (8-neighbourhood, [`crate::grid::NEIGHBOURS`]): a step a -> b of length `d` (cell or diagonal)
//! - into a water cell is a swim step: `d / v_flat * swim_factor`, no slope rule, and the path counts as `swim`;
//! - otherwise a walk step, timed by Tobler's speed `v = 6 exp(-3.5 |tan + 0.05|)` km/h on the directional slope
//!   `tan = (h_b - h_a) / d`, and a **wall** when the ground the step crosses is steeper than `max_walk_deg`. That
//!   ground slope is the gradient of the surface under the step, not the step's own rise: walking diagonally or
//!   across a 47 deg face is no easier than straight up it for a character controller, whose limit is the
//!   ground normal (`MoverParams::max_slope_deg`). A diagonal step uses the gradient of the 2 x 2 cells it crosses;
//!   a straight step its own rise plus the cross gradient of its two cells (central differences, one-sided at the
//!   border).
//!
//! Over water the height is the water surface, so a step out of the water onto a steep bank is a wall like any other.
//!
//! **Two floods.** `foot`: water cells are never entered (a path on foot only); `swim`: swim steps allowed. A
//! source cell starts at 0 in both, water or not (a bridge deck's ribbon cells are sources). Multi-source Dijkstra,
//! costs in integer microseconds; a cell keeps the lowest `(cost, swam)` pair, so a tie prefers the dry path, and
//! the heap pops equal costs by cell index: the result depends on nothing but the inputs (no thread count, no
//! insertion order). Sampling the ground is a pure per-cell map (rayon), the flood itself is serial.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use rayon::prelude::*;

use crate::filter::Declared;
use crate::grid::{NEIGHBOURS, NEIGHBOUR_DIST, neighbour};
use crate::sites::{Reach, Via};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_reach.rs"]
mod tests;

/// Cost of a cell no path reaches.
// tunables: const (sentinel, not a value)
pub const UNREACHED: u64 = u64::MAX;

/// Tobler (1993) hiking function: top speed, km/h.
// tunables: const (Tobler 1993 hiking function, maths)
const TOBLER_KMH: f64 = 6.0;
/// Tobler: decay per unit of |tan + offset|.
// tunables: const (Tobler 1993 hiking function, maths)
const TOBLER_DECAY: f64 = 3.5;
/// Tobler: the slope offset (fastest walking is slightly downhill).
// tunables: const (Tobler 1993 hiking function, maths)
const TOBLER_OFFSET: f64 = 0.05;
/// km/h to m/s.
// tunables: const (unit conversion)
const KMH_TO_MS: f64 = 1.0 / 3.6;
/// Seconds to microseconds.
// tunables: const (unit conversion)
const US_PER_S: f64 = 1.0e6;

/// The reachability values (spec "Tunables", `reach.*`).
#[must_use]
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("reach.cell_m", 18.288, "m", 2.0, 60.0, "Guess(guide spacing; quality against cost)",
          "edge of a reachability grid cell"),
        d("reach.max_walk_deg", 46.0, "deg", 20.0, 70.0, "Reference(sf_controller MoverParams::max_slope_deg default 46)",
          "ground steeper than this is a wall (no climbing model)"),
        d("reach.swim_factor", 3.0, "x", 1.0, 10.0, "Guess", "a swim step costs this many times flat walking"),
        d("reach.reseat_radius_m", 60.0, "m", 0.0, 200.0, "Guess(one MU cell at 1024 over the 10x world)",
          "a site failing its reach demand may move this far (WG-V1.2)"),
    ]
}

/// The reachability values (see [`declared`] for units and meaning).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReachValues {
    pub cell_m: f64,
    pub max_walk_deg: f64,
    pub swim_factor: f64,
    pub reseat_radius_m: f64,
}

impl ReachValues {
    /// Builds the values from declared entries, ranges checked.
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("reach: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self {
            cell_m: get("reach.cell_m")?,
            max_walk_deg: get("reach.max_walk_deg")?,
            swim_factor: get("reach.swim_factor")?,
            reseat_radius_m: get("reach.reseat_radius_m")?,
        })
    }
}

impl Default for ReachValues {
    fn default() -> Self {
        Self::from_declared(&declared()).expect("defaults are valid")
    }
}

/// The walking surface on the reachability grid: height (the water surface over water) and a water flag per cell.
#[derive(Debug, Clone, PartialEq)]
pub struct ReachGround {
    pub w: usize,
    pub h: usize,
    pub cell_m: f64,
    /// Guide-frame metres of cell (0, 0)'s centre.
    pub origin_m: [f64; 2],
    /// Surface height per cell, m, row-major, row 0 = north.
    pub height: Vec<f64>,
    /// Water per cell.
    pub water: Vec<bool>,
}

impl ReachGround {
    /// Samples `f(x, y) -> (surface height m, water)` at every cell centre (a pure per-cell map).
    pub fn sample(
        origin_m: [f64; 2],
        w: usize,
        h: usize,
        cell_m: f64,
        f: impl Fn(f64, f64) -> (f64, bool) + Sync,
    ) -> Result<Self> {
        if w == 0 || h == 0 || !(cell_m.is_finite() && cell_m > 0.0) {
            return Err(Error::Invalid(format!("reach grid {w} x {h} at {cell_m} m")));
        }
        let cells: Vec<(f64, bool)> = (0..w * h)
            .into_par_iter()
            .map(|i| f(origin_m[0] + (i % w) as f64 * cell_m, origin_m[1] - (i / w) as f64 * cell_m))
            .collect();
        if let Some(i) = cells.iter().position(|c| !c.0.is_finite()) {
            return Err(Error::Invalid(format!("reach grid: height at cell {i} is not finite")));
        }
        let (height, water) = cells.into_iter().unzip();
        Ok(Self { w, h, cell_m, origin_m, height, water })
    }

    /// Centre of cell `i`, guide-frame metres.
    #[must_use]
    pub fn centre(&self, i: usize) -> [f64; 2] {
        [
            self.origin_m[0] + (i % self.w) as f64 * self.cell_m,
            self.origin_m[1] - (i / self.w) as f64 * self.cell_m,
        ]
    }

    /// The cell whose centre is nearest `(x, y)`, or `None` off the grid.
    #[must_use]
    pub fn cell_at(&self, x: f64, y: f64) -> Option<usize> {
        let cx = ((x - self.origin_m[0]) / self.cell_m).round();
        let cy = ((self.origin_m[1] - y) / self.cell_m).round();
        (cx >= 0.0 && cy >= 0.0 && cx < self.w as f64 && cy < self.h as f64)
            .then(|| cy as usize * self.w + cx as usize)
    }

    fn at(&self, x: usize, y: usize) -> f64 {
        self.height[y * self.w + x]
    }

    /// d height / d (cell column), per metre: central difference, one-sided at the border, 0 on a 1-wide grid.
    fn grad_x(&self, x: usize, y: usize) -> f64 {
        let (lo, hi) = (x.saturating_sub(1), (x + 1).min(self.w - 1));
        if hi == lo { 0.0 } else { (self.at(hi, y) - self.at(lo, y)) / ((hi - lo) as f64 * self.cell_m) }
    }

    /// d height / d (cell row), per metre (rows run south), as [`Self::grad_x`].
    fn grad_y(&self, x: usize, y: usize) -> f64 {
        let (lo, hi) = (y.saturating_sub(1), (y + 1).min(self.h - 1));
        if hi == lo { 0.0 } else { (self.at(x, hi) - self.at(x, lo)) / ((hi - lo) as f64 * self.cell_m) }
    }

    /// Gradient magnitude (tan of the ground slope) under the step from `a` along neighbour `k` to `b`.
    fn step_ground_tan(&self, a: usize, b: usize, k: usize) -> f64 {
        let (ax, ay, bx, by) = (a % self.w, a / self.w, b % self.w, b / self.w);
        let (dx, dy) = NEIGHBOURS[k];
        let c = self.cell_m;
        let (gx, gy) = if dx != 0 && dy != 0 {
            // The 2 x 2 cells a, (bx, ay), (ax, by), b: mean of the two differences along each axis.
            let (hab, hba) = (self.at(bx, ay), self.at(ax, by));
            let sx = dx as f64 * c;
            let sy = dy as f64 * c;
            (
                ((hab - self.height[a]) + (self.height[b] - hba)) / (2.0 * sx),
                ((hba - self.height[a]) + (self.height[b] - hab)) / (2.0 * sy),
            )
        } else if dx != 0 {
            let along = (self.height[b] - self.height[a]) / (dx as f64 * c);
            (along, f64::midpoint(self.grad_y(ax, ay), self.grad_y(bx, by)))
        } else {
            let along = (self.height[b] - self.height[a]) / (dy as f64 * c);
            (f64::midpoint(self.grad_x(ax, ay), self.grad_x(bx, by)), along)
        };
        gx.hypot(gy)
    }
}

/// Tobler's walking speed on a directional slope `tan` (rise over run), m/s.
#[must_use]
pub fn tobler_ms(tan: f64) -> f64 {
    TOBLER_KMH * (-TOBLER_DECAY * (tan + TOBLER_OFFSET).abs()).exp() * KMH_TO_MS
}

/// One flood: per cell the cheapest travel time from any source, µs ([`UNREACHED`] if none), and whether that
/// path swims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flood {
    pub cost_us: Vec<u64>,
    pub swam: Vec<bool>,
}

impl Flood {
    /// True when a path reaches cell `i`.
    #[must_use]
    pub fn reached(&self, i: usize) -> bool {
        self.cost_us[i] != UNREACHED
    }
}

/// Both floods over one ground (module docs, "Two floods").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachMap {
    pub foot: Flood,
    pub swim: Flood,
}

impl ReachMap {
    /// Floods `ground` from `sources` (cell indices; duplicates allowed).
    pub fn flood(ground: &ReachGround, sources: &[usize], v: &ReachValues) -> Result<Self> {
        if let Some(&s) = sources.iter().find(|&&s| s >= ground.height.len()) {
            return Err(Error::Invalid(format!("reach: source cell {s} off the {} x {} grid", ground.w, ground.h)));
        }
        Ok(Self { foot: flood(ground, sources, v, false), swim: flood(ground, sources, v, true) })
    }

    /// The best path to cell `i` that meets `demand`: (travel time µs, how it goes), or `None`. `Foot` asks the
    /// foot flood; `Swim` and `Any` the swim flood (`Any` sets no demand; the caller decides what unreached means).
    #[must_use]
    pub fn best(&self, i: usize, demand: Reach) -> Option<(u64, Via)> {
        match demand {
            Reach::Foot => self.foot.reached(i).then(|| (self.foot.cost_us[i], Via::Foot)),
            Reach::Swim | Reach::Any => self
                .swim
                .reached(i)
                .then(|| (self.swim.cost_us[i], if self.swim.swam[i] { Via::Swim } else { Via::Foot })),
        }
    }
}

fn flood(g: &ReachGround, sources: &[usize], v: &ReachValues, allow_swim: bool) -> Flood {
    let n = g.height.len();
    let mut cost = vec![UNREACHED; n];
    let mut swam = vec![false; n];
    let mut done = vec![false; n];
    let mut heap = BinaryHeap::new();
    for &s in sources {
        if cost[s] != 0 {
            cost[s] = 0;
            heap.push(Reverse((0u64, s)));
        }
    }
    let max_tan = v.max_walk_deg.to_radians().tan();
    let flat_ms = tobler_ms(0.0);
    while let Some(Reverse((c, a))) = heap.pop() {
        if done[a] {
            continue;
        }
        done[a] = true;
        for (k, &dist) in NEIGHBOUR_DIST.iter().enumerate() {
            let Some(b) = neighbour(g.w, g.h, a, k) else { continue };
            if done[b] {
                continue;
            }
            let d = dist * g.cell_m;
            let (step_s, step_swims) = if g.water[b] {
                if !allow_swim {
                    continue;
                }
                (d / flat_ms * v.swim_factor, true)
            } else {
                if g.step_ground_tan(a, b, k) > max_tan {
                    continue;
                }
                (d / tobler_ms((g.height[b] - g.height[a]) / d), false)
            };
            let nc = c.saturating_add((step_s * US_PER_S).round() as u64);
            let ns = swam[a] || step_swims;
            if (nc, ns) < (cost[b], swam[b]) {
                cost[b] = nc;
                swam[b] = ns;
                heap.push(Reverse((nc, b)));
            }
        }
    }
    Flood { cost_us: cost, swam }
}
