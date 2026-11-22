//! Deterministic tile scatter and its density CPU twin (spec "Scatter"; UE "use grid", Horizon footprint +
//! dithered density, Tsushima).
//!
//! For one world-integer tile and one species, the tile is cut into an `n × n` grid of footprint cells
//! (`n = round(tile / footprint)`, so the cells tile it exactly and nothing straddles a tile edge). Each cell is one
//! candidate: a jittered point inside the cell whose acceptance is a Bernoulli draw with probability
//! `density_at(point) × cell area`, compared as integers (`hash < p × 2³²`). Everything is a pure function of (tile
//! key, species id, cell index) and the ground at the point: the result does not depend on tile order, thread,
//! camera or floating origin, so every peer scatters the same field (presentation only; gameplay asks
//! [`Scatterer::density_at`]).
//!
//! **Positions** are tile-relative: an [`Instance`] stores `(x, z)` from its tile's min corner in f32 and the world
//! height as `y`; the caller keeps the [`TileKey`]. A tile-local f32 is exact to ~1 µm anywhere in any world, and a
//! floating-origin shift only moves the per-tile translation (computed in f64 by
//! [`Instance::translation_from`]), never the instances. World f32 positions would round to 1 m at 16 000 km and
//! would have to be rewritten on every shift.

use glam::{DVec3, Quat, Vec3};

use crate::ground::{GroundSample, GroundSampler};
use crate::hash::{Stream, candidate_seed};
use crate::overrides::{Effect, OverrideView, Overrides, Rect};
use crate::rules::{Groundcover, SpeciesIndex};
use crate::tunables as t;

// tunables: const (format: cells per axis capped so `n²` cell indices fit a u32)
const MAX_CELLS_PER_AXIS: u32 = 65_535;
// tunables: const (maths: 2³², the integer scale of the acceptance threshold)
const ACCEPT_SCALE: f64 = 4_294_967_296.0;

/// A world-anchored scatter tile: `[x, x + 1) × [z, z + 1)` in tile units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TileKey {
    pub x: i64,
    pub z: i64,
}

impl TileKey {
    pub const fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }

    /// The tile holding world point (x, z).
    pub fn containing(x: f64, z: f64, tile_size_m: f32) -> Self {
        let s = f64::from(tile_size_m);
        // Tile indices of any reachable world fit an i64 by far.
        #[allow(clippy::cast_possible_truncation)]
        let k = Self { x: (x / s).floor() as i64, z: (z / s).floor() as i64 };
        k
    }

    /// World (x, z) of the min corner. Exact while `|key| × size < 2⁵³`.
    pub fn min_corner(self, tile_size_m: f32) -> [f64; 2] {
        let s = f64::from(tile_size_m);
        #[allow(clippy::cast_precision_loss)]
        let c = [self.x as f64 * s, self.z as f64 * s];
        c
    }

    /// The tile's world-XZ rectangle.
    pub fn rect(self, tile_size_m: f32) -> Rect {
        let min = self.min_corner(tile_size_m);
        let s = f64::from(tile_size_m);
        Rect { min, max: [min[0] + s, min[1] + s] }
    }
}

/// The one `u32` the GPU instance carries. Bits 0-7 wind phase, 8-15 tint position (between the species' two
/// tints), 16-23 fade seed (dithered distance fade), 24-31 species slot within its GPU batch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Tag {
    pub wind_phase: u8,
    pub tint: u8,
    pub fade_seed: u8,
    pub batch_slot: u8,
}

impl Tag {
    pub const fn pack(self) -> u32 {
        u32::from_le_bytes([self.wind_phase, self.tint, self.fade_seed, self.batch_slot])
    }

    pub const fn unpack(v: u32) -> Self {
        let [wind_phase, tint, fade_seed, batch_slot] = v.to_le_bytes();
        Self { wind_phase, tint, fade_seed, batch_slot }
    }
}

/// One scattered instance, relative to its tile (see the module docs for why).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instance {
    /// `x`, `z`: metres from the tile's min corner, in `[0, tile]`. `y`: world height (ground at the point).
    pub local: [f32; 3],
    /// Yaw about the instance's up axis, radians.
    pub yaw: f32,
    /// Uniform scale.
    pub scale: f32,
    /// Unit up axis: world up blended toward the ground normal by the species' `align_to_surface`.
    pub up: [f32; 3],
    /// Packed [`Tag`].
    pub tag: u32,
}

impl Instance {
    /// Size of [`Self::to_le_bytes`].
    pub const BYTES: usize = 36;

    /// Little-endian bytes of every field in declaration order (a stable encoding for hashing and tests).
    pub fn to_le_bytes(&self) -> [u8; Self::BYTES] {
        let words = [
            self.local[0].to_bits(),
            self.local[1].to_bits(),
            self.local[2].to_bits(),
            self.yaw.to_bits(),
            self.scale.to_bits(),
            self.up[0].to_bits(),
            self.up[1].to_bits(),
            self.up[2].to_bits(),
            self.tag,
        ];
        let mut out = [0; Self::BYTES];
        for (chunk, w) in out.as_chunks_mut::<4>().0.iter_mut().zip(words) {
            *chunk = w.to_le_bytes();
        }
        out
    }

    pub const fn tag(&self) -> Tag {
        Tag::unpack(self.tag)
    }

    /// World position (f64), from its tile.
    pub fn world_position(&self, tile: TileKey, tile_size_m: f32) -> DVec3 {
        let c = tile.min_corner(tile_size_m);
        DVec3::new(c[0] + f64::from(self.local[0]), f64::from(self.local[1]), c[1] + f64::from(self.local[2]))
    }

    /// Render-space translation relative to a floating origin: the tile offset is taken in f64, so the f32 result
    /// is as exact as the distance from the origin allows.
    pub fn translation_from(&self, tile: TileKey, tile_size_m: f32, origin: DVec3) -> Vec3 {
        (self.world_position(tile, tile_size_m) - origin).as_vec3()
    }

    /// Rotation: yaw about world up, then world up tilted onto [`Self::up`].
    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_arc(Vec3::Y, Vec3::from(self.up)) * Quat::from_rotation_y(self.yaw)
    }
}

/// Scatter settings, from the `groundcover.*` tunables.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterSettings {
    pub tile_size_m: f32,
    pub max_instances_per_tile: u32,
    pub density_scale: f32,
}

impl Default for ScatterSettings {
    fn default() -> Self {
        Self {
            tile_size_m: t::TILE_SIZE.default_value(),
            max_instances_per_tile: t::MAX_INSTANCES_PER_TILE.default_value(),
            density_scale: t::DENSITY_SCALE.default_value(),
        }
    }
}

/// What one tile scatter did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TileStats {
    /// Grid cells tested.
    pub candidates: u32,
    /// Instances written.
    pub accepted: u32,
    /// The scatter stopped at `max_instances_per_tile` (cells after the cap, in row-major order, were not tested).
    pub capped: bool,
}

/// The footprint grid of one species in one tile: `n` cells per axis of `cell_m` metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grid {
    pub n: u32,
    pub cell_m: f32,
}

/// Groundcover, its overrides and the settings: everything the scatter and the density twin read.
#[derive(Clone, Copy, Debug)]
pub struct Scatterer<'a> {
    pub cover: &'a Groundcover,
    pub overrides: &'a Overrides,
    pub settings: ScatterSettings,
}

impl<'a> Scatterer<'a> {
    pub const fn new(cover: &'a Groundcover, overrides: &'a Overrides, settings: ScatterSettings) -> Self {
        Self { cover, overrides, settings }
    }

    /// The species' footprint grid: `n = round(tile / footprint)` (at least 1), so the effective footprint is the
    /// species' footprint snapped to divide the tile.
    pub fn grid(&self, s: SpeciesIndex) -> Grid {
        let tile = self.settings.tile_size_m;
        let ratio = (tile / self.cover.species(s).footprint_m).round();
        // Clamped into [1, MAX_CELLS_PER_AXIS] first.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
        let n = ratio.clamp(1.0, MAX_CELLS_PER_AXIS as f32) as u32;
        #[allow(clippy::cast_precision_loss)]
        let cell_m = tile / n as f32;
        Grid { n, cell_m }
    }

    /// The density CPU twin: instances per m² of species `s` at world (x, z), before footprint saturation. The scatter
    /// evaluates exactly this at every candidate.
    pub fn density_at<G: GroundSampler + ?Sized>(&self, ground: &G, x: f64, z: f64, s: SpeciesIndex) -> f32 {
        let view = self.overrides.view(Rect { min: [x, z], max: [x, z] });
        let effect = self.effect(&view, x, z, s);
        if effect == Effect::Zero {
            return 0.0;
        }
        let mut sample = GroundSample::blank();
        ground.sample(x, z, &mut sample);
        self.density_from(effect, &sample, s)
    }

    fn effect(&self, view: &OverrideView, x: f64, z: f64, s: SpeciesIndex) -> Effect {
        self.overrides.effect(view, x, z, s, self.cover.species(s).range)
    }

    /// Species density × rule (or "only" region) weight × paint × constraint tests × global scale.
    fn density_from(&self, effect: Effect, g: &GroundSample, s: SpeciesIndex) -> f32 {
        let Effect::Scale { only, paint } = effect else { return 0.0 };
        let c = &self.cover.species[s.get()];
        let sp = &c.species;
        if !g.valid
            || c.cos_slope.is_some_and(|(lo, hi)| !(g.normal.y >= lo && g.normal.y <= hi))
            || sp.altitude_m.is_some_and(|r| !r.contains(g.height))
            || sp.water_m.is_some_and(|r| !r.contains(g.water_distance))
        {
            return 0.0;
        }
        let weight = only.unwrap_or_else(|| self.cover.rule_weight(s, &g.channels));
        (sp.density * self.settings.density_scale * weight * paint).max(0.0)
    }

    /// Scatter species `s` over `tile`, appending its instances to `out` (cleared first) with `batch_slot` in every
    /// tag. Allocation-free once `out` and the ground sample have grown.
    pub fn scatter_tile<G: GroundSampler + ?Sized>(
        &self,
        ground: &G,
        tile: TileKey,
        s: SpeciesIndex,
        batch_slot: u8,
        out: &mut Vec<Instance>,
    ) -> TileStats {
        out.clear();
        let mut stats = TileStats::default();
        let c = &self.cover.species[s.get()];
        let sp = &c.species;
        if sp.density * self.settings.density_scale <= 0.0 {
            return stats;
        }
        let Grid { n, cell_m } = self.grid(s);
        let cell_area = cell_m * cell_m;
        let size = self.settings.tile_size_m;
        let corner = tile.min_corner(size);
        let view = self.overrides.view(tile.rect(size));
        let cap = self.settings.max_instances_per_tile as usize;
        let mut g = GroundSample::blank();

        'grid: for cz in 0..n {
            for cx in 0..n {
                stats.candidates += 1;
                // Draw order is part of the format: jitter x, jitter z, accept, yaw, scale, align, wind, tint, fade.
                let mut rng = Stream::new(candidate_seed(tile, c.id_hash, cz * n + cx));
                // Cell indices are < 2¹⁶: exact in f32.
                #[allow(clippy::cast_precision_loss)]
                let (lx, lz) = ((cx as f32 + rng.next_unit()) * cell_m, (cz as f32 + rng.next_unit()) * cell_m);
                let accept = rng.next_u32();
                let (wx, wz) = (corner[0] + f64::from(lx), corner[1] + f64::from(lz));

                let effect = self.effect(&view, wx, wz, s);
                if effect == Effect::Zero {
                    continue;
                }
                ground.sample(wx, wz, &mut g);
                let p = self.density_from(effect, &g, s) * cell_area;
                if !accepts(accept, p) {
                    continue;
                }
                if out.len() >= cap {
                    stats.capped = true;
                    break 'grid;
                }

                let yaw = sp.yaw_deg.lerp(rng.next_unit()).to_radians();
                let scale = sp.scale.lerp(rng.next_unit());
                let align = sp.align_to_surface.lerp(rng.next_unit());
                let up = Vec3::Y.lerp(g.normal, align).normalize_or(Vec3::Y);
                let tag = Tag { wind_phase: rng.next_u8(), tint: rng.next_u8(), fade_seed: rng.next_u8(), batch_slot };
                out.push(Instance { local: [lx, g.height, lz], yaw, scale, up: up.to_array(), tag: tag.pack() });
            }
        }
        // `out.len() <= cap <= u32::MAX`.
        #[allow(clippy::cast_possible_truncation)]
        let accepted = out.len() as u32;
        stats.accepted = accepted;
        stats
    }
}

/// The Bernoulli test, in integers: accept when `hash < p × 2³²` (`p` clamped to `[0, 1]`; `p = 1` always accepts).
#[inline]
pub fn accepts(hash: u32, p: f32) -> bool {
    if p.is_nan() || p <= 0.0 {
        return false;
    }
    // In [0, 2³²] after the clamp.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let threshold = (f64::from(p.min(1.0)) * ACCEPT_SCALE) as u64;
    u64::from(hash) < threshold
}
