//! Flow routing of the LEM port: steepest receivers, closed basins resolved by Cordonnier et al. (2019) with the
//! "fill" re-routing, the water surface over lakes, multiple receivers with slope-power weights, and drainage area.
//!
//! Ported from Veloren `world/src/sim/erosion.rs` (GPL-3.0-or-later; commit `6c99b292`, see [`super`]):
//! `get_lakes` → [`route`] steps 3-7, `get_multi_rec` → steps 8-10, `get_multi_drainage` → step 11. What changed:
//!
//! - Receivers by **steepest slope** (height drop / distance, fastscapelib's `single_flow_router`), not the lowest
//!   neighbour of Veloren's `downhill` (which favours diagonals on smooth slopes).
//! - Basin graph: Veloren keeps 8 pass slots per node and searches them; here the cross-basin neighbour pairs are
//!   collected once, sorted by (basin pair, pass height, nodes) and reduced to the lowest pass per pair, a CSR
//!   adjacency per basin. All boundary roots (sea, grid edge) form one outside basin (id 0), so the sea's millions
//!   of nodes add no edges. The tree is grown from the outside basin by the lowest pass first (Prim's algorithm, as
//!   Veloren's binary heap; equal to fastscapelib's Kruskal MST with a virtual root); ties break on node indices.
//! - Fill re-routing: breadth-first from the lake side of the pass (Veloren pops a stack, depth-first), each flooded
//!   node (height ≤ pass) drains to the already routed neighbour best aligned with the outflow; same cost function.
//! - Water surface: `next_up` (1 ulp, fastscapelib `fill_sinks_sloped`) instead of `2 ε |h|`, which is 0 at h = 0.
//! - Single-threaded where order matters (basins, Prim, stacks, accumulation); only pure per-node maps (steepest
//!   receiver, multi-receiver weights) run in parallel, so results never depend on the thread count.

// Grid loops index several parallel arrays by node and neighbour slot; iterators would hide that.
#![allow(clippy::needless_range_loop)]

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};

use rayon::prelude::*;

use crate::grid::{NEIGHBOUR_DIST, NEIGHBOURS, neighbour};
use crate::{Error, Result};

/// Receiver of a boundary node (sea, grid edge): flow leaves the domain.
pub const BOUNDARY: i32 = -2;
/// Receiver of a local minimum before lake resolution.
const PIT: i32 = -1;

/// Multiple-receiver exponent law `p = base + slope_coef × mean slope` (Veloren `get_multi_rec`, from
/// fastscapelib-fortran `find_mult_rec`): gentle slopes spread flow, steep ones concentrate it.
#[derive(Debug, Clone, Copy)]
pub struct MfdLaw {
    pub base: f64,
    pub slope_coef: f64,
}

/// One routing of a height field.
#[derive(Debug, Clone, Default)]
pub struct Routing {
    /// Lake-resolved single receiver per node ([`BOUNDARY`] on boundary nodes).
    pub receiver: Vec<i32>,
    /// Multiple-receiver bitmask per node, bit k = neighbour k of [`NEIGHBOURS`] (water surface strictly lower).
    pub mrec: Vec<u8>,
    /// Normalised receiver weights, per neighbour slot.
    pub mwrec: Vec<[f32; 8]>,
    /// Multi-receiver stack, from nodes without donors down to the boundary.
    pub mstack: Vec<u32>,
    /// Drainage area, m² (each node contributes its cell area).
    pub area: Vec<f64>,
    /// Closed basins (lakes) resolved this routing.
    pub lakes: usize,
    /// Nodes re-routed by the fill strategy (flooded nodes).
    pub lake_nodes: usize,
}

/// f64 with a total order (heap keys).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Key(f64);
impl Eq for Key {}
impl PartialOrd for Key {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Key {
    fn cmp(&self, o: &Self) -> Ordering {
        self.0.total_cmp(&o.0)
    }
}

/// Donor lists of a receiver array (CSR, donors in increasing index order).
fn donors(receiver: &[i32]) -> (Vec<u32>, Vec<u32>) {
    let n = receiver.len();
    let mut start = vec![0u32; n + 1];
    for &r in receiver {
        if r >= 0 {
            start[r as usize + 1] += 1;
        }
    }
    for i in 0..n {
        start[i + 1] += start[i];
    }
    let mut fill = start.clone();
    let mut list = vec![0u32; start[n] as usize];
    for (i, &r) in receiver.iter().enumerate() {
        if r >= 0 {
            let s = &mut fill[r as usize];
            list[*s as usize] = i as u32;
            *s += 1;
        }
    }
    (start, list)
}

/// Grid offset (dx, dy) from node `i` to node `j`.
#[inline]
fn offset(w: usize, i: usize, j: usize) -> (f64, f64) {
    (
        (j % w) as f64 - (i % w) as f64,
        (j / w) as f64 - (i / w) as f64,
    )
}

/// Routes `z` (`w` × `h` grid at `spacing` m). `boundary` nodes are outlets (base level). Fails only on an
/// internal inconsistency (a lake without outlet, a cycle), which the construction rules out when at least one
/// boundary node exists.
#[allow(clippy::too_many_lines)]
pub fn route(
    w: usize,
    hh: usize,
    spacing: f64,
    z: &[f64],
    boundary: &[bool],
    law: MfdLaw,
) -> Result<Routing> {
    let n = w * hh;
    // 1. Steepest receivers (pure per-node map).
    let mut rec: Vec<i32> = (0..n)
        .into_par_iter()
        .map(|i| {
            if boundary[i] {
                return BOUNDARY;
            }
            let zi = z[i];
            let (mut best, mut best_s) = (PIT, 0.0);
            for k in 0..8 {
                if let Some(j) = neighbour(w, hh, i, k) {
                    let s = (zi - z[j]) / NEIGHBOUR_DIST[k];
                    if s > best_s {
                        best_s = s;
                        best = j as i32;
                    }
                }
            }
            best
        })
        .collect();

    // 2-3. Basins: the outside basin 0 (every boundary tree), one id per pit.
    let (ds, dl) = donors(&rec);
    let mut basin = vec![u32::MAX; n];
    let mut roots: Vec<u32> = vec![u32::MAX];
    let mut queue: Vec<u32> = Vec::new();
    for i in 0..n {
        if rec[i] >= 0 {
            continue;
        }
        let id = if rec[i] == BOUNDARY {
            0
        } else {
            roots.push(i as u32);
            (roots.len() - 1) as u32
        };
        queue.clear();
        queue.push(i as u32);
        let mut head = 0;
        while head < queue.len() {
            let v = queue[head] as usize;
            head += 1;
            basin[v] = id;
            queue.extend_from_slice(&dl[ds[v] as usize..ds[v + 1] as usize]);
        }
    }
    let lakes = roots.len() - 1;
    let mut lake_nodes = 0;
    if lakes > 0 {
        // 4. Lowest pass per basin pair: (lo, hi, pass height, node in lo, node in hi).
        let mut edges: Vec<(u32, u32, f64, u32, u32)> = Vec::new();
        for i in 0..n {
            let bi = basin[i];
            // E, SE, S, SW: every unordered neighbour pair once.
            for k in 2..6 {
                if let Some(j) = neighbour(w, hh, i, k) {
                    let bj = basin[j];
                    if bi != bj {
                        let pass = z[i].max(z[j]);
                        if bi < bj {
                            edges.push((bi, bj, pass, i as u32, j as u32));
                        } else {
                            edges.push((bj, bi, pass, j as u32, i as u32));
                        }
                    }
                }
            }
        }
        edges.sort_unstable_by(|a, b| {
            (a.0, a.1)
                .cmp(&(b.0, b.1))
                .then(a.2.total_cmp(&b.2))
                .then((a.3, a.4).cmp(&(b.3, b.4)))
        });
        edges.dedup_by(|b, a| a.0 == b.0 && a.1 == b.1);
        // 5. Adjacency per basin (CSR of edge indices).
        let nb = lakes + 1;
        let mut astart = vec![0u32; nb + 1];
        for e in &edges {
            astart[e.0 as usize + 1] += 1;
            astart[e.1 as usize + 1] += 1;
        }
        for b in 0..nb {
            astart[b + 1] += astart[b];
        }
        let mut afill = astart.clone();
        let mut alist = vec![0u32; astart[nb] as usize];
        for (ei, e) in edges.iter().enumerate() {
            for b in [e.0, e.1] {
                let s = &mut afill[b as usize];
                alist[*s as usize] = ei as u32;
                *s += 1;
            }
        }
        // 6. Prim from the outside basin, lowest pass first. Candidate: (pass, lake-side node, outflow node, basin).
        let mut connected = vec![false; nb];
        let mut pass_side = vec![u32::MAX; nb];
        let mut order: Vec<u32> = Vec::with_capacity(lakes);
        let mut heap: BinaryHeap<Reverse<(Key, u32, u32, u32)>> = BinaryHeap::new();
        let push_from = |b: u32, connected: &[bool], heap: &mut BinaryHeap<Reverse<(Key, u32, u32, u32)>>| {
            for &ei in &alist[astart[b as usize] as usize..astart[b as usize + 1] as usize] {
                let e = edges[ei as usize];
                let (other, from, to) = if e.0 == b { (e.1, e.4, e.3) } else { (e.0, e.3, e.4) };
                if !connected[other as usize] {
                    heap.push(Reverse((Key(e.2), from, to, other)));
                }
            }
        };
        connected[0] = true;
        push_from(0, &connected, &mut heap);
        while let Some(Reverse((_, from, to, b))) = heap.pop() {
            if connected[b as usize] {
                continue;
            }
            connected[b as usize] = true;
            rec[roots[b as usize] as usize] = to as i32;
            pass_side[b as usize] = from;
            order.push(b);
            push_from(b, &connected, &mut heap);
        }
        if order.len() != lakes {
            return Err(Error::Invalid(format!(
                "{} closed basins have no path to a boundary node",
                lakes - order.len()
            )));
        }
        // 7. Fill strategy: flooded nodes (height ≤ pass) drain toward the outflow.
        let mut tag = vec![0u8; n]; // 0 unparsed, 1 queued, 2 routed
        let mut fifo: VecDeque<u32> = VecDeque::new();
        for &b in &order {
            let pass = pass_side[b as usize] as usize;
            let out = rec[roots[b as usize] as usize] as usize;
            let elev = z[out].max(z[pass]);
            tag[out] = 2;
            tag[pass] = 1;
            fifo.clear();
            fifo.push_back(pass as u32);
            while let Some(v) = fifo.pop_front() {
                let v = v as usize;
                let (ox, oy) = offset(w, v, out);
                let (mut best, mut best_cost) = (PIT, f64::NEG_INFINITY);
                for k in 0..8 {
                    let Some(j) = neighbour(w, hh, v, k) else {
                        continue;
                    };
                    if (basin[j] != b && j != out) || z[j] > elev {
                        continue;
                    }
                    match tag[j] {
                        2 => {
                            let (dx, dy) = (NEIGHBOURS[k].0 as f64, NEIGHBOURS[k].1 as f64);
                            let cost = (ox * dx + oy * dy) / NEIGHBOUR_DIST[k];
                            if cost > best_cost {
                                best_cost = cost;
                                best = j as i32;
                            }
                        }
                        0 => {
                            tag[j] = 1;
                            fifo.push_back(j as u32);
                        }
                        _ => {}
                    }
                }
                if best < 0 {
                    return Err(Error::Invalid("fill routing found no routed neighbour".into()));
                }
                rec[v] = best;
                tag[v] = 2;
                lake_nodes += 1;
            }
        }
    }

    // 8. Single-receiver stack from the boundary up, and the water surface (lakes lifted 1 ulp per step).
    let (ds, dl) = donors(&rec);
    let mut stack: Vec<u32> = Vec::with_capacity(n);
    for i in 0..n {
        if rec[i] != BOUNDARY {
            continue;
        }
        let s0 = stack.len();
        stack.push(i as u32);
        let mut head = s0;
        while head < stack.len() {
            let v = stack[head] as usize;
            head += 1;
            stack.extend_from_slice(&dl[ds[v] as usize..ds[v + 1] as usize]);
        }
    }
    if stack.len() != n {
        return Err(Error::Invalid(format!(
            "{} nodes not reached from the boundary (cycle)",
            n - stack.len()
        )));
    }
    let mut ws = vec![0.0; n];
    for &i in &stack {
        let i = i as usize;
        ws[i] = if rec[i] < 0 {
            z[i]
        } else {
            let wr = ws[rec[i] as usize];
            if wr >= z[i] { wr.next_up() } else { z[i] }
        };
    }

    // 9. Multiple receivers on the water surface, slope-power weights (pure per-node map).
    let inv = 1.0 / spacing;
    let (mrec, weights): (Vec<u8>, Vec<[f32; 8]>) = (0..n)
        .into_par_iter()
        .map(|i| {
            let mut bits = 0u8;
            let mut wk = [0.0f64; 8];
            if boundary[i] {
                return (0, [0.0; 8]);
            }
            let (mut sum, mut nrec) = (0.0, 0usize);
            for k in 0..8 {
                if let Some(j) = neighbour(w, hh, i, k)
                    && ws[j] < ws[i]
                {
                    bits |= 1 << k;
                    let s = (ws[i] - ws[j]) * inv / NEIGHBOUR_DIST[k];
                    wk[k] = s;
                    sum += s;
                    nrec += 1;
                }
            }
            let p = law.base + law.slope_coef * sum / nrec.max(1) as f64;
            let mut tot = 0.0;
            for v in &mut wk {
                if *v > 0.0 {
                    *v = v.powf(p);
                    tot += *v;
                }
            }
            let mut out = [0.0f32; 8];
            if tot > 0.0 {
                for (o, v) in out.iter_mut().zip(wk) {
                    *o = (v / tot) as f32;
                }
            } else if bits != 0 {
                // Every lower neighbour underflowed: share equally.
                let c = f64::from(bits.count_ones());
                for (k, o) in out.iter_mut().enumerate() {
                    if bits >> k & 1 == 1 {
                        *o = (1.0 / c) as f32;
                    }
                }
            }
            (bits, out)
        })
        .unzip();

    // 10. Multi-receiver stack: peel from nodes without donors (Veloren `get_multi_rec`).
    let mut ndon = vec![0u8; n];
    for i in 0..n {
        let b = mrec[i];
        for k in 0..8 {
            if b >> k & 1 == 1
                && let Some(j) = neighbour(w, hh, i, k)
            {
                ndon[j] += 1;
            }
        }
    }
    let summit: Vec<bool> = ndon.iter().map(|&d| d == 0).collect();
    let mut mstack: Vec<u32> = Vec::with_capacity(n);
    let mut parse: Vec<u32> = Vec::new();
    for i in 0..n {
        if summit[i] {
            parse.push(i as u32);
        }
        while let Some(v) = parse.pop() {
            mstack.push(v);
            let v = v as usize;
            let b = mrec[v];
            for k in 0..8 {
                if b >> k & 1 == 1
                    && let Some(j) = neighbour(w, hh, v, k)
                {
                    ndon[j] -= 1;
                    if ndon[j] == 0 {
                        parse.push(j as u32);
                    }
                }
            }
        }
    }
    if mstack.len() != n {
        return Err(Error::Invalid(format!(
            "multi-receiver graph has a cycle ({} of {n} nodes stacked)",
            mstack.len()
        )));
    }

    // 11. Drainage area along the multi stack (fixed order, deterministic sums).
    let mut area = vec![spacing * spacing; n];
    for &i in &mstack {
        let i = i as usize;
        let (b, a) = (mrec[i], area[i]);
        for k in 0..8 {
            if b >> k & 1 == 1
                && let Some(j) = neighbour(w, hh, i, k)
            {
                area[j] += a * f64::from(weights[i][k]);
            }
        }
    }
    Ok(Routing {
        receiver: rec,
        mrec,
        mwrec: weights,
        mstack,
        area,
        lakes,
        lake_nodes,
    })
}
