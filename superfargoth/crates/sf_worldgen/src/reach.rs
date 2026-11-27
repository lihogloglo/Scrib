//! **Reachability flood core** (spec `sites.md` "Design" WG-V1, slice WG-V1.1): a cost-distance flood over a walking
//! grid, the GIS cost-distance technique with Tobler's (1993) hiking function. Bevy-free and game-agnostic: the
//! caller samples its final ground into a [`ReachGround`] and names the source cells ([`sources`]: towns, hamlets,
//! road ribbons); [`verdicts`] then gives every site its status (WG-V1.2, "Site verdicts" below).
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
//!
//! **Site verdicts** ([`verdicts`]). A site's *cells* are the cells whose centre lies inside its footprint ring (or
//! within `radius_m` of its centre when it has no ring; the nearest cell when neither covers one). Its *disc* is
//! `radius_m`, else the farthest ring point from the centre. Per site, in file order (each site alone, so the result
//! is independent of thread count):
//! - **kept** sites never move: `placed`, travel from their best cell if reached;
//! - otherwise the site *passes where it stands* when its `reach` demand is met on one of its cells ([`ReachMap::best`];
//!   `any` is always met) and its centre is within the row's road distance `[min, max]` (the post-road rule): `placed`,
//!   `travel_s` and `via` from its cheapest cell;
//! - else it is **re-seated**: candidate cells are those whose centre lies within `reach.reseat_radius_m` of the
//!   site's centre and that meet the demand, tried by (travel time, distance moved, cell index); the first whose
//!   shifted site meets its row (steepest ground over the shifted cells, no water cell centre within disc +
//!   `water_gap_m`, every anchor footprint at least disc + `anchor_gap_m` away, road distance) wins. The site moves
//!   rigidly by the offset from its centre to that cell's centre (centre and ring; `access` stays where the road
//!   ends, `ground_m` is cleared as stale, `buildings[]` and `installations[]` pass through: placer serde, opaque here);
//! - else **dropped**, with the reason. A site with `access: none` is judged like any other (no climbing model).
//!
//! Sources ([`sources`]): every town and hamlet cell, and every cell within half a road's width of its smoothed
//! centreline ([`crate::roads::Centreline`], sampled every half cell, the nearest cell of each sample included, so a
//! trail narrower than a cell still marks its line). Deck crossings are sources too.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use rayon::prelude::*;

use crate::anchors::Footprint;
use crate::filter::Declared;
use crate::grid::{NEIGHBOURS, NEIGHBOUR_DIST, neighbour};
use crate::roads::{Centreline, RoadNetwork};
use crate::sites::{Reach, Site, SiteKind, SitesFile, Status, Via};
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

/// The verdict product's file name in the guide folder (spec "Design", step 3).
// tunables: const (file name)
pub const VALID_FILE: &str = "sites.valid.json";

/// A site row's rules V1 checks itself (spec "Site table"): the post-road road distance, and the placement rules a
/// re-seated site must still meet. Table data (per row), not tunables of this module.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SiteRule {
    /// Steepest ground over the site's cells, deg (`slope_max_deg`).
    pub slope_max_deg: f64,
    /// Minimum gap from the site's disc to a water cell centre, m (`distance_m.water` min).
    pub water_gap_m: f64,
    /// Minimum gap from the site's disc to an anchor footprint, m (`distance_m.anchor`).
    pub anchor_gap_m: f64,
    /// Distance from the site's centre to the nearest road centreline, m (`distance_m.road` `[min, max]`).
    pub road_m: [f64; 2],
}

impl SiteRule {
    /// No rule at all: any slope, no gap, any road distance.
    // tunables: const (the absence of a rule, not a value)
    pub const NONE: Self =
        Self { slope_max_deg: f64::INFINITY, water_gap_m: 0.0, anchor_gap_m: 0.0, road_m: [0.0, f64::INFINITY] };

    /// True when the road rule asks nothing.
    fn no_road_rule(&self) -> bool {
        self.road_m[0] <= 0.0 && self.road_m[1] == f64::INFINITY
    }

    /// The distance beyond which the road rule's answer no longer changes (its finite bound).
    fn road_cap(&self) -> f64 {
        if self.road_m[1].is_finite() { self.road_m[1] } else { self.road_m[0] }
    }
}

/// One road's smoothed centreline, its half width and a bounding box of its knots (padded by a cell).
#[derive(Debug, Clone)]
pub struct RoadLine {
    pub line: Centreline,
    pub half_width_m: f64,
    bounds: [f64; 4],
}

impl RoadLine {
    /// Distance from `p` to the centreline, or `None` when it is surely farther than `cap` (bounding box test).
    fn distance_within(&self, p: [f64; 2], cap: f64) -> Option<f64> {
        let b = self.bounds;
        let (dx, dy) = ((b[0] - p[0]).max(p[0] - b[2]).max(0.0), (b[1] - p[1]).max(p[1] - b[3]).max(0.0));
        if dx.hypot(dy) > cap {
            return None;
        }
        let q = self.line.eval(self.line.project(p)).0;
        Some((p[0] - q[0]).hypot(p[1] - q[1]))
    }
}

/// The centrelines of a road network, arc tables at `cell_m` density; a segment with fewer than two distinct points
/// has no line and is skipped.
#[must_use]
pub fn road_lines(net: &RoadNetwork, cell_m: f64) -> Vec<RoadLine> {
    net.segments
        .iter()
        .filter_map(|seg| {
            let line = Centreline::new(&seg.points, cell_m)?;
            let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
            for p in line.knots() {
                b = [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])];
            }
            // A centripetal Catmull-Rom bulges a little past its knots: pad by a cell.
            let b = [b[0] - cell_m, b[1] - cell_m, b[2] + cell_m, b[3] + cell_m];
            Some(RoadLine { line, half_width_m: seg.width_m / 2.0, bounds: b })
        })
        .collect()
}

/// Whether the nearest-road distance at `p` lies in `rule`'s `[min, max]`.
fn road_ok(roads: &[RoadLine], rule: &SiteRule, p: [f64; 2]) -> bool {
    if rule.no_road_rule() {
        return true;
    }
    let cap = rule.road_cap();
    let d = roads.iter().filter_map(|r| r.distance_within(p, cap)).fold(f64::INFINITY, f64::min);
    d >= rule.road_m[0] && d <= rule.road_m[1]
}

impl ReachGround {
    /// Cells whose centre lies within `r` of `p` (box scan), in index order.
    fn cells_within(&self, p: [f64; 2], r: f64) -> Vec<usize> {
        let c = self.cell_m;
        let (fx, fy) = ((p[0] - self.origin_m[0]) / c, (self.origin_m[1] - p[1]) / c);
        let span = r / c;
        if fx + span < 0.0 || fy + span < 0.0 || fx - span > (self.w - 1) as f64 || fy - span > (self.h - 1) as f64 {
            return Vec::new();
        }
        let clamp = |v: f64, n: usize| v.clamp(0.0, (n - 1) as f64) as usize;
        let (x0, x1) = (clamp((fx - span).ceil(), self.w), clamp((fx + span).floor(), self.w));
        let (y0, y1) = (clamp((fy - span).ceil(), self.h), clamp((fy + span).floor(), self.h));
        let mut out = Vec::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = y * self.w + x;
                let q = self.centre(i);
                if (q[0] - p[0]).hypot(q[1] - p[1]) <= r {
                    out.push(i);
                }
            }
        }
        out
    }

    /// Tan of the ground slope at cell `i` (central differences).
    fn cell_tan(&self, i: usize) -> f64 {
        let (x, y) = (i % self.w, i / self.w);
        self.grad_x(x, y).hypot(self.grad_y(x, y))
    }
}

/// Even-odd point in polygon (a ring is closed implicitly; a point on an edge falls either way).
fn in_ring(ring: &[[f64; 2]], p: [f64; 2]) -> bool {
    let mut inside = false;
    let mut j = ring.len() - 1;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// A site's disc radius: `radius_m`, else the farthest ring point from the centre.
fn disc_m(s: &Site) -> f64 {
    if s.radius_m > 0.0 {
        return s.radius_m;
    }
    s.footprint.iter().map(|q| (q[0] - s.centre_m[0]).hypot(q[1] - s.centre_m[1])).fold(0.0, f64::max)
}

/// The cells of site `s` shifted by `shift` (module docs "Site verdicts"), in index order; empty off the grid.
fn site_cells(g: &ReachGround, s: &Site, shift: [f64; 2]) -> Vec<usize> {
    let c = [s.centre_m[0] + shift[0], s.centre_m[1] + shift[1]];
    let mut cells = g.cells_within(c, disc_m(s));
    if s.footprint.len() >= 3 {
        let ring: Vec<[f64; 2]> = s.footprint.iter().map(|q| [q[0] + shift[0], q[1] + shift[1]]).collect();
        cells.retain(|&i| in_ring(&ring, g.centre(i)));
    }
    if cells.is_empty() {
        cells.extend(g.cell_at(c[0], c[1]));
    }
    cells
}

/// The source cells (module docs, "Sources"): sorted, no duplicates.
#[must_use]
pub fn sources(g: &ReachGround, sites: &[Site], roads: &[RoadLine]) -> Vec<usize> {
    let mut out: Vec<usize> = sites
        .iter()
        .filter(|s| matches!(s.kind, SiteKind::Town | SiteKind::Hamlet))
        .flat_map(|s| site_cells(g, s, [0.0, 0.0]))
        .collect();
    let step = g.cell_m / 2.0;
    for r in roads {
        let len = r.line.length();
        let n = (len / step).ceil() as usize;
        for k in 0..=n {
            let p = r.line.eval((k as f64 * step).min(len)).0;
            out.extend(g.cell_at(p[0], p[1]));
            out.extend(g.cells_within(p, r.half_width_m));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// What [`verdicts`] judges against.
#[derive(Debug, Clone, Copy)]
pub struct VerdictInput<'a> {
    pub ground: &'a ReachGround,
    pub map: &'a ReachMap,
    /// [`road_lines`] of the network the sources came from.
    pub roads: &'a [RoadLine],
    /// Anchor footprints in the guide frame.
    pub anchors: &'a [Footprint],
    /// Rules per site row; every site that is not kept must name a row here.
    pub rules: &'a BTreeMap<String, SiteRule>,
}

/// The cheapest of `cells` meeting `demand`: (travel µs, via), lowest cost, then lowest index.
fn best_of(map: &ReachMap, cells: &[usize], demand: Reach) -> Option<(u64, Via)> {
    cells.iter().filter_map(|&i| map.best(i, demand)).min_by_key(|b| b.0)
}

/// Whether `demand` is met by `best` (an `any` demand always is).
fn demand_met(demand: Reach, best: Option<(u64, Via)>) -> bool {
    demand == Reach::Any || best.is_some()
}

/// Whether site `s` shifted by `shift` meets `rule` (slope, water gap, anchor gap, road distance).
fn meets_rule(inp: &VerdictInput, s: &Site, rule: &SiteRule, shift: [f64; 2]) -> bool {
    let g = inp.ground;
    let c = [s.centre_m[0] + shift[0], s.centre_m[1] + shift[1]];
    let r = disc_m(s);
    let cells = site_cells(g, s, shift);
    if cells.is_empty() {
        return false;
    }
    let steepest = cells.iter().map(|&i| g.cell_tan(i)).fold(0.0, f64::max).atan().to_degrees();
    if steepest > rule.slope_max_deg {
        return false;
    }
    if g.cells_within(c, r + rule.water_gap_m).iter().any(|&i| g.water[i]) {
        return false;
    }
    if inp.anchors.iter().any(|a| a.distance(c) < r + rule.anchor_gap_m) {
        return false;
    }
    road_ok(inp.roads, rule, c)
}

fn set_reached(s: &mut Site, best: Option<(u64, Via)>) {
    s.travel_s = best.map(|b| b.0 as f64 / US_PER_S);
    s.via = best.map(|b| b.1);
}

/// One site's verdict (module docs, "Site verdicts").
fn verdict(inp: &VerdictInput, v: &ReachValues, s: &Site) -> Result<Site> {
    let g = inp.ground;
    let mut out = s.clone();
    out.reason.clear();
    out.moved_m = None;
    let here = best_of(inp.map, &site_cells(g, s, [0.0, 0.0]), s.reach);
    if s.kept {
        out.status = Some(Status::Placed);
        set_reached(&mut out, here);
        return Ok(out);
    }
    let rule = inp
        .rules
        .get(&s.row)
        .ok_or_else(|| Error::Invalid(format!("reach: site {} names row {:?}, which has no rule", s.key, s.row)))?;
    let reached_here = demand_met(s.reach, here);
    if reached_here && road_ok(inp.roads, rule, s.centre_m) {
        out.status = Some(Status::Placed);
        set_reached(&mut out, here);
        return Ok(out);
    }
    let mut cands: Vec<(u64, f64, usize)> = g
        .cells_within(s.centre_m, v.reseat_radius_m)
        .into_iter()
        .filter_map(|i| {
            let b = inp.map.best(i, s.reach);
            if !demand_met(s.reach, b) {
                return None;
            }
            let q = g.centre(i);
            Some((b.map_or(UNREACHED, |b| b.0), (q[0] - s.centre_m[0]).hypot(q[1] - s.centre_m[1]), i))
        })
        .collect();
    cands.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(a.2.cmp(&b.2)));
    for &(_, moved, i) in &cands {
        let q = g.centre(i);
        let shift = [q[0] - s.centre_m[0], q[1] - s.centre_m[1]];
        if !meets_rule(inp, s, rule, shift) {
            continue;
        }
        out.status = Some(Status::Reseated);
        out.moved_m = Some(moved);
        out.centre_m = q;
        for p in &mut out.footprint {
            *p = [p[0] + shift[0], p[1] + shift[1]];
        }
        out.ground_m = None;
        set_reached(&mut out, best_of(inp.map, &site_cells(g, s, shift), s.reach));
        return Ok(out);
    }
    out.status = Some(Status::Dropped);
    set_reached(&mut out, None);
    let why = if reached_here {
        "outside its road distance".to_owned()
    } else {
        format!("unreached ({})", reach_name(s.reach))
    };
    out.reason = if cands.is_empty() {
        format!("{why}; no cell within {} m meets its reach demand", v.reseat_radius_m)
    } else {
        format!("{why}; none of {} reachable cells within {} m meets its row", cands.len(), v.reseat_radius_m)
    };
    Ok(out)
}

fn reach_name(r: Reach) -> &'static str {
    match r {
        Reach::Foot => "foot",
        Reach::Swim => "swim",
        Reach::Any => "any",
    }
}

/// Every site's verdict (module docs, "Site verdicts"): the same file with `status`, `moved_m`, `travel_s`, `via`
/// and `reason` set, re-seated sites moved. Errors when a site that is not kept names a row with no rule.
pub fn verdicts(inp: &VerdictInput, v: &ReachValues, file: &SitesFile) -> Result<SitesFile> {
    let sites = file.sites.par_iter().map(|s| verdict(inp, v, s)).collect::<Result<Vec<_>>>()?;
    Ok(SitesFile { sites, ..file.clone() })
}
