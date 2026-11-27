//! **Site and road clearings** (spec `sites.md` "WG-V1b clearings", flow step 5, slice WG-V1b.1): a pure point
//! query, [`Clearings::contains`], that scatter (flora and rocks) asks through its terrain's `excluded`. The standard
//! exclusion mask against site footprints and road ribbons (UE PCG "Difference", Horizon's placement exclusion; spec
//! "Industry comparison"). Bevy-free, game-agnostic.
//!
//! **What is cleared.** A point is in a clearing when it lies
//!
//! - inside the footprint of a site that is not dropped, grown by the site's `clearing_m`: its ring (three points or
//!   more; inside the ring or within `clearing_m` of an edge), else its disc ([`crate::reach`]'s `disc_m`, then
//!   `disc + clearing_m` from the centre). Kept hand-made towns (`kept = true`) are left out: their ground is already
//!   excluded, more tightly, by their anchor stamps and their placements' built-up mask, and their format-1
//!   `radius_m` is a settlement extent (100-450 m), not a footprint;
//! - or within `clearing.road_m` of a road ribbon's edge, the ribbon included: distance to the smoothed centreline
//!   ([`Centreline`], the ribbon's own spline) at most `width_m / 2 + clearing.road_m`. The centreline is a polyline
//!   of chords every `clearing.road_step_m` along its arc (the chord error of a step `h` on a bend of radius `R` is
//!   `h² / 8R`: 2.5 cm at the 2 m default on a 20 m bend). Bridge decks and fords are cleared like the rest.
//!
//! **Speed.** Every shape (a site, or one chord of a road, each with its margin) goes into the buckets of a sparse
//! grid that its margin-grown box touches; a point tests only its bucket's shapes. A shape's test needs nothing but
//! the point, so a point's answer is the same whatever else is asked, in any order, on any thread, and a tile
//! evaluated alone equals the whole run.

use std::collections::HashMap;

use super::{Site, Status};
use crate::filter::Declared;
use crate::reach::{disc_m, in_ring};
use crate::roads::{Centreline, RoadNetwork};
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_site_clearings.rs"]
mod tests;

/// Edge of a bucket of the shape grid, m. Changes only the speed, never an answer.
// tunables: const (layout: bucket size of an acceleration grid, no effect on results)
const BUCKET_M: f64 = 64.0;

/// The clearing values (spec "Tunables", `clearing.*`; per-site margins are the site table's `clearing_m`).
#[must_use]
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("clearing.road_m", 6.0, "m", 0.0, 30.0, "Guess(owner Q1)", "flora-free margin beyond a road ribbon's edge"),
        d("clearing.road_step_m", 2.0, "m", 0.5, 20.0, "Guess(chord error h^2/8R: 2.5 cm on a 20 m bend; quality against cost)",
          "chord step along a road centreline for the clearing test"),
    ]
}

/// The clearing values, checked against their ranges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearingValues {
    pub road_m: f64,
    pub road_step_m: f64,
}

impl ClearingValues {
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("clearings: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self { road_m: get("clearing.road_m")?, road_step_m: get("clearing.road_step_m")? })
    }
}

impl Default for ClearingValues {
    fn default() -> Self {
        Self::from_declared(&declared()).expect("defaults are valid")
    }
}

/// One cleared shape.
#[derive(Debug, Clone)]
enum Shape {
    /// A footprint ring grown by `margin`.
    Ring { ring: Vec<[f64; 2]>, margin: f64 },
    /// A disc of radius `r` (footprint disc + margin).
    Disc { c: [f64; 2], r: f64 },
    /// A capsule: points within `r` of the chord `a`-`b`.
    Chord { a: [f64; 2], b: [f64; 2], r: f64 },
}

impl Shape {
    fn bounds(&self) -> [f64; 4] {
        match self {
            Self::Ring { ring, margin } => {
                let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
                for p in ring {
                    b = [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])];
                }
                [b[0] - margin, b[1] - margin, b[2] + margin, b[3] + margin]
            }
            Self::Disc { c, r } => [c[0] - r, c[1] - r, c[0] + r, c[1] + r],
            Self::Chord { a, b, r } => [a[0].min(b[0]) - r, a[1].min(b[1]) - r, a[0].max(b[0]) + r, a[1].max(b[1]) + r],
        }
    }

    fn contains(&self, p: [f64; 2]) -> bool {
        match self {
            Self::Ring { ring, margin } => {
                if in_ring(ring, p) {
                    return true;
                }
                let n = ring.len();
                (0..n).any(|i| seg_dist(p, ring[i], ring[(i + 1) % n]) <= *margin)
            }
            Self::Disc { c, r } => (p[0] - c[0]).hypot(p[1] - c[1]) <= *r,
            Self::Chord { a, b, r } => seg_dist(p, *a, *b) <= *r,
        }
    }
}

/// Distance from `p` to the segment `a`-`b`.
fn seg_dist(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let l2 = ab[0] * ab[0] + ab[1] * ab[1];
    let f = if l2 > 0.0 { (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1]) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (p[0] - (a[0] + ab[0] * f)).hypot(p[1] - (a[1] + ab[1] * f))
}

/// The clearings of a guide: site footprints and road margins, a pure point query (module docs).
#[derive(Debug, Clone)]
pub struct Clearings {
    shapes: Vec<Shape>,
    buckets: HashMap<(i64, i64), Vec<u32>>,
    bucket_m: f64,
    /// Sites that gave a clearing.
    pub sites: usize,
    /// Road segments that gave a clearing.
    pub roads: usize,
}

impl Clearings {
    /// The clearings of `sites` (dropped sites and kept towns left out) and of `roads`, when there are roads.
    #[must_use]
    pub fn new(sites: &[Site], roads: Option<&RoadNetwork>, v: &ClearingValues) -> Self {
        Self::with_bucket(sites, roads, v, BUCKET_M)
    }

    pub(crate) fn with_bucket(sites: &[Site], roads: Option<&RoadNetwork>, v: &ClearingValues, bucket_m: f64) -> Self {
        let mut shapes = Vec::new();
        let mut n_sites = 0;
        for s in sites.iter().filter(|s| !s.kept && s.status != Some(Status::Dropped)) {
            n_sites += 1;
            if s.footprint.len() >= 3 {
                shapes.push(Shape::Ring { ring: s.footprint.clone(), margin: s.clearing_m });
            } else {
                shapes.push(Shape::Disc { c: s.centre_m, r: disc_m(s) + s.clearing_m });
            }
        }
        let mut n_roads = 0;
        for seg in roads.map_or(&[][..], |n| &n.segments[..]) {
            let Some(line) = Centreline::new(&seg.points, v.road_step_m) else { continue };
            n_roads += 1;
            let r = seg.width_m / 2.0 + v.road_m;
            let len = line.length();
            let steps = ((len / v.road_step_m).ceil() as usize).max(1);
            let mut a = line.eval(0.0).0;
            for k in 1..=steps {
                let b = line.eval(len * k as f64 / steps as f64).0;
                shapes.push(Shape::Chord { a, b, r });
                a = b;
            }
        }
        let mut buckets: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        for (i, sh) in shapes.iter().enumerate() {
            let b = sh.bounds();
            let (x0, y0) = key(b[0], b[1], bucket_m);
            let (x1, y1) = key(b[2], b[3], bucket_m);
            for y in y0..=y1 {
                for x in x0..=x1 {
                    buckets.entry((x, y)).or_default().push(u32::try_from(i).expect("fewer than 2^32 shapes"));
                }
            }
        }
        Self { shapes, buckets, bucket_m, sites: n_sites, roads: n_roads }
    }

    /// Whether `(x, y)` (guide m) lies in a clearing.
    #[must_use]
    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.buckets
            .get(&key(x, y, self.bucket_m))
            .is_some_and(|ids| ids.iter().any(|&i| self.shapes[i as usize].contains([x, y])))
    }

    /// No clearing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
}

/// Bucket of a point.
fn key(x: f64, y: f64, bucket_m: f64) -> (i64, i64) {
    ((x / bucket_m).floor() as i64, (y / bucket_m).floor() as i64)
}
