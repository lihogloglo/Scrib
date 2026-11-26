//! Ring planner (spec `superfargoth/docs/specs/big-world-rendering.md` "Data flow", step 1): keeps the tiles around
//! the viewer resident, with hysteresis so a viewer pacing over a tile edge does not reload it.
//!
//! A tile enters when the ground distance from the viewer to its square ([`TileKey::distance_to`]) is at most
//! `radius_m`, and leaves once that distance is above `radius_m + hysteresis_tiles × tile_m`. The renderer owns two
//! rings: one at the impostor radius (tiles whose instances sit in the GPU pool) and one at the mesh radius (tiles
//! that also publish their instances to physics); [`ring_radii`] gives both from the species' bands.
//!
//! A pure state machine: the same sequence of calls gives the same deltas on every machine (f64 maths, correctly
//! rounded `sqrt`, total orders for sorting). With `hysteresis_tiles = 0` the resident set is a pure function of the
//! last viewer position. Each update scans only the square of tiles that can be within the enter distance, plus the
//! residents, never the world.

use std::collections::BTreeSet;

use crate::bands::Bands;
use crate::tile::TileKey;

/// What one ring plans with. `radius_m` and `hysteresis_tiles` may change between any two updates (live tunables).
/// A changed `tile_m` or `bounds` re-keys the world: the caller calls [`Ring::clear`] first (not detected here).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingSettings {
    /// Tile edge in metres (`bigworld.tile_m`).
    pub tile_m: f32,
    /// A tile enters when the distance from the viewer to its square is ≤ `radius_m`.
    pub radius_m: f32,
    /// It leaves when that distance is > `radius_m + hysteresis_tiles × tile_m` (`bigworld.ring.hysteresis_tiles`).
    pub hysteresis_tiles: u32,
    /// Inclusive `[min, max]` corner keys of the world; tiles outside never enter. `None`: unbounded.
    pub bounds: Option<[TileKey; 2]>,
}

/// What one update changed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RingDelta {
    /// Tiles that became resident, nearest first, ties by key (the upload order).
    pub enter: Vec<TileKey>,
    /// Tiles that stopped being resident, in key order.
    pub leave: Vec<TileKey>,
}

impl RingDelta {
    /// Nothing entered and nothing left.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.enter.is_empty() && self.leave.is_empty()
    }
}

/// The resident tile set of one ring.
#[derive(Clone, Debug, Default)]
pub struct Ring {
    resident: BTreeSet<TileKey>,
}

impl Ring {
    /// An empty ring.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Re-plans around `viewer` (world x, z): residents farther than the leave distance go out, then every tile within
    /// the enter distance (and inside `bounds`) that is not resident comes in. `enter` is sorted nearest tile first,
    /// ties by key (the upload order); `leave` is sorted by key; the two are disjoint.
    ///
    /// Degenerate input changes nothing and returns an empty delta: a non-finite viewer coordinate or radius, or a
    /// `tile_m` that is not a positive finite number. A negative radius admits no tile (residents still leave by the
    /// rule). The scan covers the square of keys around the viewer that can be within `radius_m`: the radius is a
    /// bounded tunable, and `bounds` clips the square as well.
    pub fn update(&mut self, viewer: [f64; 2], settings: &RingSettings) -> RingDelta {
        let [vx, vz] = viewer;
        let tile_m = settings.tile_m;
        let radius = f64::from(settings.radius_m);
        let usable = vx.is_finite() && vz.is_finite() && radius.is_finite() && tile_m.is_finite() && tile_m > 0.0;
        if !usable {
            return RingDelta::default();
        }
        let leave_m = radius + f64::from(settings.hysteresis_tiles) * f64::from(tile_m);

        let leave: Vec<TileKey> =
            self.resident.iter().copied().filter(|key| key.distance_to(vx, vz, tile_m) > leave_m).collect();
        for key in &leave {
            self.resident.remove(key);
        }

        let mut enter: Vec<(f64, TileKey)> = Vec::new();
        if let Some((xs, zs)) = scan_square(vx, vz, radius, tile_m, settings.bounds) {
            for z in zs[0]..=zs[1] {
                for x in xs[0]..=xs[1] {
                    let key = TileKey::new(x, z);
                    let dist = key.distance_to(vx, vz, tile_m);
                    if dist <= radius && !self.resident.contains(&key) {
                        enter.push((dist, key));
                    }
                }
            }
        }
        enter.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let enter: Vec<TileKey> = enter.into_iter().map(|(_, key)| key).collect();
        self.resident.extend(enter.iter().copied());
        RingDelta { enter, leave }
    }

    /// The resident tiles, in key order.
    pub fn resident(&self) -> impl Iterator<Item = TileKey> + '_ {
        self.resident.iter().copied()
    }

    /// Number of resident tiles.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resident.len()
    }

    /// No tile is resident.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resident.is_empty()
    }

    /// Whether `key` is resident.
    #[must_use]
    pub fn contains(&self, key: TileKey) -> bool {
        self.resident.contains(&key)
    }

    /// Everything leaves (teleport, feature switched off, `tile_m` or `bounds` changed); returned in key order.
    pub fn clear(&mut self) -> Vec<TileKey> {
        std::mem::take(&mut self.resident).into_iter().collect()
    }
}

/// Inclusive key ranges `([x_min, x_max], [z_min, z_max])` of the tiles that can be within `radius` of the viewer,
/// clipped to `bounds`; `None` when empty. A tile `k` can reach the viewer when `k·s ≤ v + r` and `(k+1)·s ≥ v − r`,
/// so `k ∈ [floor((v − r)/s) − 1, floor((v + r)/s)]` covers it with one key to spare.
fn scan_square(
    vx: f64,
    vz: f64,
    radius: f64,
    tile_m: f32,
    bounds: Option<[TileKey; 2]>,
) -> Option<([i32; 2], [i32; 2])> {
    if radius < 0.0 {
        return None;
    }
    let s = f64::from(tile_m);
    let span = |v: f64| [key_index((v - radius) / s - 1.0), key_index((v + radius) / s)];
    let (mut xs, mut zs) = (span(vx), span(vz));
    if let Some([lo, hi]) = bounds {
        xs = [xs[0].max(lo.x), xs[1].min(hi.x)];
        zs = [zs[0].max(lo.z), zs[1].min(hi.z)];
    }
    (xs[0] <= xs[1] && zs[0] <= zs[1]).then_some((xs, zs))
}

/// Floor of a tile coordinate as a key index, saturating at the `i32` range.
fn key_index(t: f64) -> i32 {
    // `as` saturates; finite inputs only (checked by `Ring::update`).
    #[allow(clippy::cast_possible_truncation)]
    let k = t.floor() as i32;
    k
}

/// `(mesh radius, impostor radius)` that cover every species at up to `max_scale`: the largest
/// `mesh_end_m × max_scale` and `impostor_end_m × max_scale`. An empty slice, or a scale that is negative or NaN,
/// gives `(0, 0)`.
#[must_use]
pub fn ring_radii(bands: &[Bands], max_scale: f32) -> (f32, f32) {
    // `f32::max` drops a NaN operand, so the fold never yields NaN.
    bands.iter().fold((0.0_f32, 0.0_f32), |(mesh, impostor), b| {
        (mesh.max(b.mesh_end_m * max_scale), impostor.max(b.impostor_end_m * max_scale))
    })
}
