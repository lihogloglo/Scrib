//! The tile rings around the viewer (Bevy-free): which world tiles are resident, which cast shadows, in what order
//! their scatter jobs start, and the worst-case tile count a batch is sized for.
//!
//! Distances are horizontal (XZ), from the viewer to a tile's rectangle. A tile **enters** a ring when its
//! *nearest* point is within the ring's range and **leaves** only past range + hysteresis, so a viewer standing
//! on a boundary never thrashes. The shadow ring uses the tile's *farthest* point instead, so every instance of a
//! casting tile is within the shadow range (+ hysteresis) and never reaches the fade band.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use sf_groundcover::TileKey;

/// One ring's geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingSettings {
    pub tile_size_m: f32,
    pub range_m: f32,
    pub hysteresis_m: f32,
}

/// Horizontal distance from `p` to the nearest point of `tile` (0 inside it).
pub fn nearest_distance(tile: TileKey, tile_size_m: f32, p: [f64; 2]) -> f64 {
    let r = tile.rect(tile_size_m);
    let dx = (r.min[0] - p[0]).max(p[0] - r.max[0]).max(0.0);
    let dz = (r.min[1] - p[1]).max(p[1] - r.max[1]).max(0.0);
    dx.hypot(dz)
}

/// Horizontal distance from `p` to the farthest corner of `tile`.
pub fn farthest_distance(tile: TileKey, tile_size_m: f32, p: [f64; 2]) -> f64 {
    let r = tile.rect(tile_size_m);
    let dx = (p[0] - r.min[0]).abs().max((r.max[0] - p[0]).abs());
    let dz = (p[1] - r.min[1]).abs().max((r.max[1] - p[1]).abs());
    dx.hypot(dz)
}

/// Every tile whose nearest point is within `range_m` of `p`, in (distance, key) order: the job order.
pub fn tiles_within(p: [f64; 2], tile_size_m: f32, range_m: f64) -> Vec<TileKey> {
    let s = f64::from(tile_size_m);
    // Tile indices of any reachable world fit an i64.
    #[allow(clippy::cast_possible_truncation)]
    let span = |c: f64| ((c - range_m) / s).floor() as i64..=((c + range_m) / s).floor() as i64;
    let mut out: Vec<(f64, TileKey)> = Vec::new();
    for z in span(p[1]) {
        for x in span(p[0]) {
            let k = TileKey::new(x, z);
            let d = nearest_distance(k, tile_size_m, p);
            if d <= range_m {
                out.push((d, k));
            }
        }
    }
    out.sort_by(by_distance);
    out.into_iter().map(|(_, k)| k).collect()
}

/// (distance, key) order: nearest first, ties by key (deterministic).
pub fn by_distance(a: &(f64, TileKey), b: &(f64, TileKey)) -> Ordering {
    a.0.total_cmp(&b.0).then(a.1.cmp(&b.1))
}

/// What one [`TileRing::update`] changed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RingDelta {
    /// New members, nearest first.
    pub entered: Vec<TileKey>,
    /// Members that left, in key order.
    pub left: Vec<TileKey>,
}

/// A ring of tiles with hysteresis.
#[derive(Clone, Debug, Default)]
pub struct TileRing {
    members: BTreeSet<TileKey>,
}

/// Which distance a ring measures a tile by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Measure {
    /// The tile's nearest point (the residency ring).
    Nearest,
    /// The tile's farthest corner (the shadow ring: the whole tile is inside).
    Farthest,
}

impl TileRing {
    pub fn members(&self) -> &BTreeSet<TileKey> {
        &self.members
    }

    pub fn contains(&self, tile: TileKey) -> bool {
        self.members.contains(&tile)
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn clear(&mut self) {
        self.members.clear();
    }

    /// Moves the ring to viewer `p`: tiles within `range` enter, members past `range + hysteresis` leave.
    pub fn update(&mut self, p: [f64; 2], s: &RingSettings, measure: Measure) -> RingDelta {
        self.update_among(p, s, measure, None)
    }

    /// [`Self::update`] restricted to `candidates` (the shadow ring only holds resident tiles).
    pub fn update_among(
        &mut self,
        p: [f64; 2],
        s: &RingSettings,
        measure: Measure,
        candidates: Option<&BTreeSet<TileKey>>,
    ) -> RingDelta {
        let range = f64::from(s.range_m.max(0.0));
        let leave = range + f64::from(s.hysteresis_m.max(0.0));
        let dist = |k: TileKey| match measure {
            Measure::Nearest => nearest_distance(k, s.tile_size_m, p),
            Measure::Farthest => farthest_distance(k, s.tile_size_m, p),
        };
        let allowed = |k: &TileKey| candidates.is_none_or(|c| c.contains(k));
        let mut delta = RingDelta::default();
        self.members.retain(|&k| {
            let keep = allowed(&k) && dist(k) <= leave;
            if !keep {
                delta.left.push(k);
            }
            keep
        });
        if s.range_m > 0.0 {
            for k in tiles_within(p, s.tile_size_m, range) {
                if allowed(&k) && dist(k) <= range && self.members.insert(k) {
                    delta.entered.push(k);
                }
            }
        }
        delta
    }
}

/// The most tiles a [`Measure::Nearest`] ring of `range_m` + `hysteresis_m` can ever hold, for any viewer
/// position: the tiles within that distance of the viewer's whole tile (a bound, exact for the worst position up
/// to the tile corners). Batches are sized from it.
pub fn max_ring_tiles(tile_size_m: f32, range_m: f32, hysteresis_m: f32) -> u32 {
    let s = f64::from(tile_size_m);
    let r = f64::from(range_m.max(0.0) + hysteresis_m.max(0.0));
    if range_m <= 0.0 {
        return 0;
    }
    // A count of tiles, far below 2^31.
    #[allow(clippy::cast_possible_truncation)]
    let k = (r / s).ceil() as i64 + 1;
    let mut n = 0u32;
    for dz in -k..=k {
        for dx in -k..=k {
            // Rect-to-rect gap between the viewer's tile and tile (dx, dz), in tiles.
            #[allow(clippy::cast_precision_loss)]
            let (gx, gz) = (((dx.abs() - 1).max(0)) as f64, ((dz.abs() - 1).max(0)) as f64);
            if s * gx.hypot(gz) <= r {
                n += 1;
            }
        }
    }
    n
}
