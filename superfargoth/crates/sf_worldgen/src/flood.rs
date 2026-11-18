//! Depression routing by **Priority-Flood+ε** (Barnes, Lehman, Mulla, "Priority-flood: An optimal
//! depression-filling and watershed-labeling algorithm for digital elevation models", Computers & Geosciences 62,
//! 2014, Algorithms 2 and 3: the plain-queue "pit" shortcut with the ε raise).
//!
//! The result is a routing surface `z ≥ h` in which every dry sample has a strictly lower neighbour, so steepest
//! descent on `z` reaches a base-level sample from everywhere (no interior pits after routing). ε is one ulp
//! (`f64::next_up`), so the raise is invisible at metre scale. The heights themselves are not filled: lakes are
//! water, and only the hydrology stage creates water (worldgen.md "Stage rules").
//!
//! Base level: wet samples (below sea level) at the sea level itself (the water surface, not the sea floor) and
//! dry samples on the grid border. Single-threaded with a total order (height, then index): deterministic.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

use crate::grid::neighbour;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    z: f64,
    i: u32,
}

impl Eq for Node {}

impl Ord for Node {
    // Min-heap on (z, i).
    fn cmp(&self, o: &Self) -> Ordering {
        o.z.total_cmp(&self.z).then_with(|| o.i.cmp(&self.i))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

/// Fills `z` (resized to `h.len()`) with the Priority-Flood+ε routing surface of `h`. `wet[i]` marks base-level
/// water; wet samples get `z = sea_level`.
pub fn priority_flood_eps(
    w: usize,
    hgt: usize,
    h: &[f64],
    wet: &[bool],
    sea_level: f64,
    z: &mut Vec<f64>,
) {
    flood(w, hgt, h, wet, sea_level, z, true);
}

/// Fills `z` with the plain **depression-filled** surface of `h` (Barnes et al. 2014 Algorithm 2 with the pit
/// queue, no ε): every closed depression is raised exactly to its spill height, so `z − h` is the fill depth.
/// Same base level and order as [`priority_flood_eps`].
pub fn priority_fill(w: usize, hgt: usize, h: &[f64], wet: &[bool], sea_level: f64, z: &mut Vec<f64>) {
    flood(w, hgt, h, wet, sea_level, z, false);
}

/// The shared queue logic: with `eps`, a pit sample is raised one ulp above the sample it was reached from.
fn flood(w: usize, hgt: usize, h: &[f64], wet: &[bool], sea_level: f64, z: &mut Vec<f64>, eps: bool) {
    let n = w * hgt;
    z.clear();
    z.extend(
        h.iter()
            .zip(wet)
            .map(|(&v, &is_wet)| if is_wet { sea_level } else { v }),
    );
    let mut closed = wet.to_vec();
    let mut open = BinaryHeap::new();
    let mut pit = VecDeque::new();
    for i in 0..n {
        if wet[i] {
            // A wet sample seeds the flood only where it touches land.
            if (0..8).any(|k| neighbour(w, hgt, i, k).is_some_and(|j| !wet[j])) {
                open.push(Node {
                    z: z[i],
                    i: i as u32,
                });
            }
        } else {
            let (x, y) = (i % w, i / w);
            if x == 0 || y == 0 || x + 1 == w || y + 1 == hgt {
                closed[i] = true;
                open.push(Node {
                    z: z[i],
                    i: i as u32,
                });
            }
        }
    }
    loop {
        let c = if let Some(c) = pit.pop_front() {
            c
        } else if let Some(node) = open.pop() {
            node.i as usize
        } else {
            break;
        };
        let zc = z[c];
        for k in 0..8 {
            let Some(nb) = neighbour(w, hgt, c, k) else {
                continue;
            };
            if closed[nb] {
                continue;
            }
            closed[nb] = true;
            if z[nb] <= zc {
                z[nb] = if eps { zc.next_up() } else { zc };
                pit.push_back(nb);
            } else {
                open.push(Node {
                    z: z[nb],
                    i: nb as u32,
                });
            }
        }
    }
}
