//! Walkability repair ramps (LF-2, `landforms.md` "Pipeline" step 5, run last so it sees the final field).
//!
//! Per region (components cut at region borders, as `measure::land_rows`), walkable = slope <= `max_walk_deg`
//! (central differences, [`crate::measure::walk_slope_deg`]), 4-connected. While the region's largest walkable
//! component is under `min_largest` of its land: a 0-1 BFS from that component over the region's land (entering an
//! unwalkable sample costs 1) gives every other component its cheapest crossing; components of at least
//! `min_component_m2` are joined cheapest first (a Prim-style greedy, ties by sample index) until the joined land
//! reaches the target, each unwalkable run on the crossing path replaced by a **ramp**: a straight graded corridor
//! at most `ramp_deg` steep, `ramp_width_m` wide (at least 3 samples, the central-difference stencil), lengthened
//! on both ends until its end heights allow the grade, blended into the ground over `shoulder_m`. Crossings longer
//! than `max_cross_m` are left alone (no roads up whole mountainsides), and so are ramps whose centre line would cut
//! or fill more than `max_cut_m` (no causeways over ravines or canyons, no trenches through ridges). Rounds repeat with fresh labels. Sea and
//! frozen (anchor) samples are never written; ramps never make dry land wet. Deterministic (fixed orders).

use std::collections::VecDeque;

use crate::Grid;
use crate::measure::walk_slope_deg;
use crate::placed::scarp::ramp01;

/// Lowest height a written dry sample may take above sea level, metres.
// tunables: const (invariant: keeps written dry samples strictly dry so the wet/dry audit stays clean)
const LAND_FLOOR_M: f64 = 0.05;

/// Checked route values (`landforms.md` "Tunables" `route.*`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteParams {
    pub max_walk_deg: f64,
    pub min_largest: f64,
    pub ramp_deg: f64,
    pub ramp_width_m: f64,
    /// Per region.
    pub max_ramps: usize,
    pub min_component_m2: f64,
    pub max_cross_m: f64,
    pub max_ramp_m: f64,
    pub shoulder_m: f64,
    /// Largest cut or fill of the ramp's centre line below or above the ground, metres (no causeways over
    /// ravines, no trenches through ridges).
    pub max_cut_m: f64,
    pub rounds: usize,
}

/// One cut ramp (local metres `x = col s`, `y = row s`).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct Ramp {
    pub region: u8,
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub len_m: f64,
    pub grade_deg: f64,
}

/// Per-region outcome.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct RegionRoute {
    pub label: u8,
    pub land: usize,
    pub largest_before: f64,
    pub largest_after: f64,
    pub ramps: usize,
    /// Crossings found but not cut (no grade within `max_ramp_m`, off the grid, or sea at an end).
    pub failed: usize,
}

/// Walkable components per region: component id per sample (`u32::MAX` off), sizes, region of each component.
fn label(walk: &[bool], labels: &[u8], w: usize, h: usize) -> (Vec<u32>, Vec<usize>, Vec<u8>) {
    let n = w * h;
    let mut comp = vec![u32::MAX; n];
    let (mut sizes, mut region) = (Vec::new(), Vec::new());
    let mut stack = Vec::new();
    for s in 0..n {
        if !walk[s] || comp[s] != u32::MAX {
            continue;
        }
        let id = sizes.len() as u32;
        comp[s] = id;
        stack.push(s);
        let mut size = 0usize;
        while let Some(c) = stack.pop() {
            size += 1;
            for j in n4(c, w, h).into_iter().flatten() {
                if walk[j] && comp[j] == u32::MAX && labels[j] == labels[c] {
                    comp[j] = id;
                    stack.push(j);
                }
            }
        }
        sizes.push(size);
        region.push(labels[s]);
    }
    (comp, sizes, region)
}

#[inline]
fn n4(i: usize, w: usize, h: usize) -> [Option<usize>; 4] {
    let (x, y) = (i % w, i / w);
    [
        (x + 1 < w).then_some(i + 1),
        (x > 0).then(|| i - 1),
        (y + 1 < h).then_some(i + w),
        (y > 0).then(|| i - w),
    ]
}

/// Largest component share per region label and the land count (`land` = finite slope).
fn largest_per_region(land: &[bool], labels: &[u8], sizes: &[usize], region: &[u8]) -> ([usize; 256], [usize; 256], [Option<u32>; 256]) {
    let mut land_n = [0usize; 256];
    for (i, &l) in land.iter().enumerate() {
        if l {
            land_n[usize::from(labels[i])] += 1;
        }
    }
    let mut best = [0usize; 256];
    let mut best_id = [None; 256];
    for (c, (&s, &r)) in sizes.iter().zip(region).enumerate() {
        let r = usize::from(r);
        if s > best[r] {
            best[r] = s;
            best_id[r] = Some(c as u32);
        }
    }
    (land_n, best, best_id)
}

/// Bilinear height at local metres `p`, `None` off the grid.
fn height_at(g: &Grid, p: [f64; 2]) -> Option<f64> {
    let (c, r) = (p[0] / g.spacing, p[1] / g.spacing);
    if c < 0.0 || r < 0.0 || c > (g.w - 1) as f64 || r > (g.h - 1) as f64 {
        return None;
    }
    let (c0, r0) = (c.floor() as usize, r.floor() as usize);
    let (c1, r1) = ((c0 + 1).min(g.w - 1), (r0 + 1).min(g.h - 1));
    let (tx, ty) = (c - c0 as f64, r - r0 as f64);
    let top = g.at(c0, r0) + (g.at(c1, r0) - g.at(c0, r0)) * tx;
    let bot = g.at(c0, r1) + (g.at(c1, r1) - g.at(c0, r1)) * tx;
    Some(top + (bot - top) * ty)
}

/// Cuts one ramp between local points `pa` and `pb` (module docs). `None` when no grade fits.
#[allow(clippy::too_many_arguments)]
pub fn cut_ramp(
    g: &mut Grid,
    sea: f64,
    frozen: &[bool],
    pa: [f64; 2],
    pb: [f64; 2],
    region: u8,
    p: &RouteParams,
) -> Option<Ramp> {
    let s = g.spacing;
    let mid = [f64::midpoint(pa[0], pb[0]), f64::midpoint(pa[1], pb[1])];
    let dl = (pb[0] - pa[0]).hypot(pb[1] - pa[1]);
    if dl <= 0.0 {
        return None;
    }
    let u = [(pb[0] - pa[0]) / dl, (pb[1] - pa[1]) / dl];
    let nrm = [-u[1], u[0]];
    let tan_max = p.ramp_deg.to_radians().tan();
    let mut l = dl + 2.0 * s;
    let (z0, z1) = loop {
        let e0 = [mid[0] - u[0] * 0.5 * l, mid[1] - u[1] * 0.5 * l];
        let e1 = [mid[0] + u[0] * 0.5 * l, mid[1] + u[1] * 0.5 * l];
        let (z0, z1) = (height_at(g, e0)?, height_at(g, e1)?);
        if z0 < sea || z1 < sea {
            return None;
        }
        if (z1 - z0).abs() <= tan_max * l {
            break (z0, z1);
        }
        // Search step: lengthen by a quarter until the grade fits.
        l *= 1.25;
        if l > p.max_ramp_m {
            return None;
        }
    };
    // Centre-line cut / fill check before writing anything.
    let steps = (l / s).ceil().max(1.0) as usize;
    for k in 0..=steps {
        let t = -0.5 * l + l * k as f64 / steps as f64;
        let q = [mid[0] + u[0] * t, mid[1] + u[1] * t];
        let ground = height_at(g, q)?;
        let zr = z0 + (t + 0.5 * l) / l * (z1 - z0);
        if (zr - ground).abs() > p.max_cut_m || ground < sea {
            return None;
        }
    }
    let hw = 0.5 * p.ramp_width_m.max(3.0 * s);
    let sh = p.shoulder_m;
    let reach_t = 0.5 * l + sh;
    let reach_q = hw + sh;
    let ext = reach_t.abs().max(reach_q);
    let c0 = ((mid[0] - ext) / s).floor().max(0.0) as usize;
    let r0 = ((mid[1] - ext) / s).floor().max(0.0) as usize;
    let c1 = (((mid[0] + ext) / s).ceil() as usize).min(g.w - 1);
    let r1 = (((mid[1] + ext) / s).ceil() as usize).min(g.h - 1);
    for r in r0..=r1 {
        for c in c0..=c1 {
            let i = r * g.w + c;
            let old = g.data[i];
            if old < sea || frozen[i] {
                continue;
            }
            let (dx, dy) = (c as f64 * s - mid[0], r as f64 * s - mid[1]);
            let t = dx * u[0] + dy * u[1];
            let q = dx * nrm[0] + dy * nrm[1];
            if t.abs() > reach_t || q.abs() > reach_q {
                continue;
            }
            let wt = (1.0 - ramp01(q.abs() - hw, sh)) * (1.0 - ramp01(t.abs() - 0.5 * l, sh));
            if wt <= 0.0 {
                continue;
            }
            let tc = t.clamp(-0.5 * l, 0.5 * l);
            let zr = z0 + (tc + 0.5 * l) / l * (z1 - z0);
            g.data[i] = (old + (zr - old) * wt).max(sea + LAND_FLOOR_M);
        }
    }
    Some(Ramp {
        region,
        from: [mid[0] - u[0] * 0.5 * l, mid[1] - u[1] * 0.5 * l],
        to: [mid[0] + u[0] * 0.5 * l, mid[1] + u[1] * 0.5 * l],
        len_m: l,
        grade_deg: ((z1 - z0).abs() / l).atan().to_degrees(),
    })
}

/// Runs the repair in place (module docs). `labels` = region label per sample; `frozen` samples are never written.
#[allow(clippy::too_many_lines)]
pub fn repair(g: &mut Grid, sea: f64, labels: &[u8], frozen: &[bool], p: &RouteParams) -> (Vec<Ramp>, Vec<RegionRoute>) {
    let (w, h, n) = (g.w, g.h, g.len());
    let s = g.spacing;
    let min_comp = (p.min_component_m2 / (s * s)).ceil() as usize;
    let max_cost = (p.max_cross_m / s).ceil() as u32;
    let mut ramps = Vec::new();
    let mut report: Vec<RegionRoute> = Vec::new();
    let mut ramps_in = [0usize; 256];
    let mut failed_in = [0usize; 256];
    let mut dist = vec![u32::MAX; n];
    let mut parent = vec![u32::MAX; n];
    let mut cut = vec![false; n];
    for round in 0..=p.rounds {
        let slope = walk_slope_deg(g, sea);
        let land: Vec<bool> = slope.iter().map(|v| v.is_finite()).collect();
        let walk: Vec<bool> = slope.iter().map(|&v| v.is_finite() && v <= p.max_walk_deg).collect();
        let (comp, sizes, region) = label(&walk, labels, w, h);
        let (land_n, best, best_id) = largest_per_region(&land, labels, &sizes, &region);
        if round == 0 {
            for v in 0..256usize {
                if land_n[v] > 0 {
                    report.push(RegionRoute {
                        label: v as u8,
                        land: land_n[v],
                        largest_before: best[v] as f64 / land_n[v] as f64,
                        ..RegionRoute::default()
                    });
                }
            }
        }
        let needs = |v: usize, ramps_in: &[usize; 256]| {
            land_n[v] > 0 && (best[v] as f64) < p.min_largest * land_n[v] as f64 && ramps_in[v] < p.max_ramps
        };
        if round == p.rounds || !(0..256).any(|v| needs(v, &ramps_in)) {
            for r in &mut report {
                let v = usize::from(r.label);
                r.largest_after = best[v] as f64 / land_n[v].max(1) as f64;
                r.ramps = ramps_in[v];
                r.failed = failed_in[v];
            }
            break;
        }
        // 0-1 BFS from each repairing region's largest component.
        dist.fill(u32::MAX);
        parent.fill(u32::MAX);
        let mut dq = VecDeque::new();
        for i in 0..n {
            let v = usize::from(labels[i]);
            if comp[i] != u32::MAX && Some(comp[i]) == best_id[v] && needs(v, &ramps_in) {
                dist[i] = 0;
                dq.push_back(i);
            }
        }
        while let Some(c) = dq.pop_front() {
            let dc = dist[c];
            for j in n4(c, w, h).into_iter().flatten() {
                if !land[j] || labels[j] != labels[c] {
                    continue;
                }
                let cost = u32::from(!walk[j]);
                let nd = dc + cost;
                if nd < dist[j] && nd <= max_cost {
                    dist[j] = nd;
                    parent[j] = c as u32;
                    if cost == 0 {
                        dq.push_front(j);
                    } else {
                        dq.push_back(j);
                    }
                }
            }
        }
        // Cheapest entry of every other component.
        let mut entry: Vec<(u32, usize)> = vec![(u32::MAX, usize::MAX); sizes.len()];
        for i in 0..n {
            let c = comp[i];
            if c != u32::MAX && dist[i] != u32::MAX && (dist[i], i) < entry[c as usize] {
                entry[c as usize] = (dist[i], i);
            }
        }
        let mut cand: Vec<(u8, u32, usize, usize)> = (0..sizes.len())
            .filter(|&c| {
                let v = usize::from(region[c]);
                Some(c as u32) != best_id[v] && sizes[c] >= min_comp && entry[c].0 != u32::MAX && entry[c].0 > 0
            })
            .map(|c| (region[c], entry[c].0, entry[c].1, c))
            .collect();
        cand.sort_unstable();
        cut.fill(false);
        let mut gained = [0usize; 256];
        let mut any = false;
        for (v8, _, e, c) in cand {
            let v = usize::from(v8);
            if !needs(v, &ramps_in) || (best[v] + gained[v]) as f64 >= p.min_largest * land_n[v] as f64 {
                continue;
            }
            // Walk the chain entry -> source, cutting each unwalkable run.
            let mut chain = vec![e];
            let mut k = e;
            while parent[k] != u32::MAX {
                k = parent[k] as usize;
                chain.push(k);
            }
            let mut i = 0;
            let mut joined = true;
            while i < chain.len() {
                if walk[chain[i]] {
                    i += 1;
                    continue;
                }
                let start = i;
                while i < chain.len() && !walk[chain[i]] {
                    i += 1;
                }
                if start == 0 || i >= chain.len() || chain[start..i].iter().any(|&q| cut[q]) {
                    continue;
                }
                for &q in &chain[start..i] {
                    cut[q] = true;
                }
                let at = |q: usize| [(q % w) as f64 * s, (q / w) as f64 * s];
                if let Some(r) = cut_ramp(g, sea, frozen, at(chain[i]), at(chain[start - 1]), v8, p) {
                    ramps.push(r);
                    ramps_in[v] += 1;
                    any = true;
                } else {
                    failed_in[v] += 1;
                    joined = false;
                }
                if ramps_in[v] >= p.max_ramps {
                    break;
                }
            }
            if joined {
                gained[v] += sizes[c];
            }
        }
        if !any {
            // Nothing cut: the next round would see the same field.
            let slope = walk_slope_deg(g, sea);
            let walk: Vec<bool> = slope.iter().map(|&v| v.is_finite() && v <= p.max_walk_deg).collect();
            let (_, sizes, region) = label(&walk, labels, w, h);
            let (land_n, best, _) = largest_per_region(&land, labels, &sizes, &region);
            for r in &mut report {
                let v = usize::from(r.label);
                r.largest_after = best[v] as f64 / land_n[v].max(1) as f64;
                r.ramps = ramps_in[v];
                r.failed = failed_in[v];
            }
            break;
        }
    }
    (ramps, report)
}
