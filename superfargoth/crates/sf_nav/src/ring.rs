//! Interim tile residency around one anchor (nav.md "Streaming", `NV4c`): which baked tiles of a
//! worldspace should be resident for an anchor [`Location`], with load/unload hysteresis.
//! Bevy-free and store-free: [`NavRing`] plans keys; the host reads each tile from its bake store
//! and reports what it found ([`NavRing::settle`]), and removes the tiles [`NavRing::update`]
//! drops from its backend. `sf_nav_bevy` runs it on the bake store.
//!
//! **Scaffold `NavRingScaffold`**: one anchor (the player), synchronous store reads, a per-frame
//! tile cap. Removal path: NV5 replaces it with per-anchor, per-portal, budgeted residency on the
//! streaming scheduler (nav.md "Streaming").
//!
//! - A tile is in the ring when the anchor's XZ distance to the tile's square (region-local
//!   meters, the class's [`BuildConfig::tile_size`] grid from the region origin, as the bake
//!   tiles it) is at most the load radius; it stays until that distance passes the unload radius
//!   ([`RingRadii`]). Height is ignored: tiles are columns.
//! - Plans are made when the anchor enters another tile of any (class, region), changes
//!   worldspace, or the radii change; between plans the ring only drains its queue, so a still
//!   anchor costs nothing. The ring edge is therefore exact to within one tile of anchor motion.
//! - A worldspace change drops every tile of the old worldspace.

use std::collections::BTreeMap;

use glam::Vec2;
use sf_coords::WorldPos;
use sf_world::{Location, WorldspaceId};

use crate::{AgentClassId, BuildConfig, TileKey};

/// A nav region's frame: its tiles are on a grid from `origin` (nav.md "Nav regions").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegionFrame {
    pub region: u16,
    pub origin: WorldPos,
}

/// One agent class's tile grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClassTiling {
    pub class: AgentClassId,
    /// Meters per tile side ([`BuildConfig::tile_size`]).
    pub tile_size: f32,
}

impl ClassTiling {
    pub fn of(cfg: &BuildConfig) -> Self {
        Self {
            class: cfg.class.id,
            tile_size: cfg.tile_size(),
        }
    }
}

/// Load and unload radii, meters. An unload radius under the load radius acts as the load radius
/// (no hysteresis, never thrashing).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingRadii {
    pub load_m: f32,
    pub unload_m: f32,
}

impl RingRadii {
    fn unload(self) -> f32 {
        self.unload_m.max(self.load_m)
    }
}

/// What the ring keeps: the classes, and the regions of each worldspace.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RingLayout {
    pub classes: Vec<ClassTiling>,
    /// Worldspaces with their own regions.
    pub regions: BTreeMap<WorldspaceId, Vec<RegionFrame>>,
    /// The regions of every other worldspace.
    pub default_regions: Vec<RegionFrame>,
}

impl RingLayout {
    pub fn regions_of(&self, ws: &WorldspaceId) -> &[RegionFrame] {
        self.regions.get(ws).unwrap_or(&self.default_regions)
    }

    fn tiling(&self, class: AgentClassId) -> Option<&ClassTiling> {
        self.classes.iter().find(|c| c.class == class)
    }

    /// Every tile within `radius` of `at`, nearest first (ties in key order).
    #[allow(
        clippy::cast_possible_truncation,
        reason = "tile indices of a region (<= 16 km) are small"
    )]
    pub fn tiles_within(&self, at: &Location, radius: f32) -> Vec<TileKey> {
        let mut found: Vec<(f32, TileKey)> = Vec::new();
        for r in self.regions_of(&at.worldspace) {
            let local = r.origin.delta_f32(at.pos);
            let p = Vec2::new(local.x, local.z);
            for c in &self.classes {
                let size = c.tile_size;
                if size <= 0.0 {
                    continue;
                }
                let (x0, x1) = (
                    ((p.x - radius) / size).floor() as i32,
                    ((p.x + radius) / size).floor() as i32,
                );
                let (z0, z1) = (
                    ((p.y - radius) / size).floor() as i32,
                    ((p.y + radius) / size).floor() as i32,
                );
                for x in x0..=x1 {
                    for z in z0..=z1 {
                        let d = tile_distance(p, x, z, size);
                        if d <= radius {
                            found.push((
                                d,
                                TileKey {
                                    ws: at.worldspace.clone(),
                                    class: c.class,
                                    region: r.region,
                                    x,
                                    z,
                                },
                            ));
                        }
                    }
                }
            }
        }
        found.sort_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        found.into_iter().map(|(_, k)| k).collect()
    }

    /// The anchor's XZ distance to `key`'s tile; `None` when the tile is in another worldspace or
    /// its class or region isn't in the layout.
    pub fn distance(&self, key: &TileKey, at: &Location) -> Option<f32> {
        if key.ws != at.worldspace {
            return None;
        }
        let r = self
            .regions_of(&key.ws)
            .iter()
            .find(|r| r.region == key.region)?;
        let c = self.tiling(key.class)?;
        let local = r.origin.delta_f32(at.pos);
        Some(tile_distance(
            Vec2::new(local.x, local.z),
            key.x,
            key.z,
            c.tile_size,
        ))
    }

    /// The tile the anchor stands in, per (region, class) in layout order: the plan stamp.
    #[allow(
        clippy::cast_possible_truncation,
        reason = "tile indices of a region (<= 16 km) are small"
    )]
    fn anchor_tiles(&self, at: &Location) -> Vec<(i32, i32)> {
        let mut out = Vec::new();
        for r in self.regions_of(&at.worldspace) {
            let local = r.origin.delta_f32(at.pos);
            for c in &self.classes {
                let s = c.tile_size.max(f32::MIN_POSITIVE);
                out.push(((local.x / s).floor() as i32, (local.z / s).floor() as i32));
            }
        }
        out
    }
}

/// XZ distance from `p` (region-local) to tile `(x, z)`'s square of side `size`; zero inside.
#[allow(clippy::cast_precision_loss, reason = "small tile indices")]
pub fn tile_distance(p: Vec2, x: i32, z: i32, size: f32) -> f32 {
    let min = Vec2::new(x as f32, z as f32) * size;
    let max = min + Vec2::splat(size);
    (p.clamp(min, max) - p).length()
}

/// What reading a planned tile found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileState {
    /// In the backend.
    Resident,
    /// Baked with no polygons (zero bytes): nothing to add.
    Empty,
    /// No product for it (outside the region's extent, or the bake didn't run).
    Unbaked,
    /// The read, decode or `add_tile` failed; not retried until the tile leaves the ring.
    Failed,
}

/// Per-state counts and the queue, for stats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RingCounts {
    pub resident: usize,
    pub empty: usize,
    pub unbaked: usize,
    pub failed: usize,
    pub pending: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Stamp {
    ws: WorldspaceId,
    tiles: Vec<(i32, i32)>,
    radii: (u32, u32),
}

/// The ring's state: the tiles it decided on and the queue of tiles to read.
#[derive(Clone, Debug, Default)]
pub struct NavRing {
    layout: RingLayout,
    known: BTreeMap<TileKey, TileState>,
    /// Farthest first: [`NavRing::next`] pops the nearest.
    queue: Vec<TileKey>,
    stamp: Option<Stamp>,
}

impl NavRing {
    pub fn new(layout: RingLayout) -> Self {
        Self {
            layout,
            ..Self::default()
        }
    }

    pub fn layout(&self) -> &RingLayout {
        &self.layout
    }

    /// Re-plans for `anchor` if it entered another tile, worldspace, or the radii changed (no
    /// anchor: nothing changes). Returns the [`TileState::Resident`] tiles past the unload radius
    /// (or in another worldspace): the host removes them from its backend. Other dropped tiles
    /// are forgotten.
    pub fn update(&mut self, anchor: Option<&Location>, radii: RingRadii) -> Vec<TileKey> {
        let Some(at) = anchor else {
            return Vec::new();
        };
        let stamp = Stamp {
            ws: at.worldspace.clone(),
            tiles: self.layout.anchor_tiles(at),
            radii: (radii.load_m.to_bits(), radii.unload_m.to_bits()),
        };
        if self.stamp.as_ref() == Some(&stamp) {
            return Vec::new();
        }
        self.stamp = Some(stamp);
        let unload = radii.unload();
        let mut dropped = Vec::new();
        let layout = &self.layout;
        self.known.retain(|key, state| {
            let keep = layout.distance(key, at).is_some_and(|d| d <= unload);
            if !keep && *state == TileState::Resident {
                dropped.push(key.clone());
            }
            keep
        });
        let mut queue: Vec<TileKey> = layout
            .tiles_within(at, radii.load_m)
            .into_iter()
            .filter(|k| !self.known.contains_key(k))
            .collect();
        queue.reverse();
        self.queue = queue;
        dropped
    }

    /// The next tiles to read, nearest first, at most `cap`; each must be [`NavRing::settle`]d.
    pub fn next(&mut self, cap: usize) -> Vec<TileKey> {
        let n = cap.min(self.queue.len());
        let at = self.queue.len() - n;
        let mut out = self.queue.split_off(at);
        out.reverse();
        out
    }

    /// Records what reading `key` found.
    pub fn settle(&mut self, key: TileKey, state: TileState) {
        self.known.insert(key, state);
    }

    pub fn state(&self, key: &TileKey) -> Option<TileState> {
        self.known.get(key).copied()
    }

    /// Resident tiles, in key order.
    pub fn resident(&self) -> impl Iterator<Item = &TileKey> {
        self.known
            .iter()
            .filter(|(_, s)| **s == TileState::Resident)
            .map(|(k, _)| k)
    }

    pub fn counts(&self) -> RingCounts {
        let mut c = RingCounts {
            pending: self.queue.len(),
            ..RingCounts::default()
        };
        for s in self.known.values() {
            match s {
                TileState::Resident => c.resident += 1,
                TileState::Empty => c.empty += 1,
                TileState::Unbaked => c.unbaked += 1,
                TileState::Failed => c.failed += 1,
            }
        }
        c
    }

    /// Forgets every decision and the plan (the host swapped its backend): the next
    /// [`NavRing::update`] plans from scratch.
    pub fn clear(&mut self) {
        self.known.clear();
        self.queue.clear();
        self.stamp = None;
    }
}
