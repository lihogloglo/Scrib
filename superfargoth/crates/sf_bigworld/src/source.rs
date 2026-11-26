//! Deterministic instance sources (spec `superfargoth/docs/specs/big-world-rendering.md` "Data flow"): pure
//! functions that fill a tile with its [`Instance`]s on demand, so a tile is never stored, only regenerated.
//!
//! [`SyntheticForest`] is the built-in source for tests, benches and example scenes: one tree per cell of a jittered
//! grid, seeded per (world seed, tile, cell) through the crate's `SplitMix64` hash, on an optional analytic relief.

use std::f64::consts::TAU;

use crate::hash::{Stream, cell_seed};
use crate::instance::Instance;
use crate::tile::TileKey;

// tunables: const (square metres in a hectare, a unit)
const M2_PER_HA: f64 = 10_000.0;

/// Fills a tile with its instances. Deterministic and pure: the same `(key, tile_m)` gives the same instances in the
/// same order on every machine and in any call order; tiles are independent. Called on worker threads.
pub trait InstanceSource: Send + Sync {
    /// Appends the instances of tile `key` (x, z tile-local, y absolute) to `out`.
    fn tile(&self, key: TileKey, tile_m: f32, out: &mut Vec<Instance>);
}

/// A synthetic forest for tests, benches and example scenes: one tree per cell of a jittered grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyntheticForest {
    /// World seed.
    pub seed: u64,
    /// Trees per hectare (10 000 m²).
    pub density_per_ha: f32,
    /// Species drawn uniformly from `0..species_count`.
    pub species_count: u16,
    /// Uniform scale in `[min, max]`.
    pub scale: [f32; 2],
    /// Ground = `relief_m × sin(2π·wx/λ) × cos(2π·wz/λ)` on world coords; 0 = flat.
    pub relief_m: f32,
    /// The relief's wavelength λ in metres.
    pub relief_wavelength_m: f32,
}

impl SyntheticForest {
    /// Cells per tile side: `round(tile_m / sqrt(10 000 / density))`, 0 when the density is not positive.
    fn cells_per_side(&self, tile_m: f32) -> u32 {
        let density = f64::from(self.density_per_ha);
        if density.is_nan() || density <= 0.0 || tile_m.is_nan() || tile_m <= 0.0 {
            return 0;
        }
        // Positive and finite above; `as` saturates for absurd densities.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = (f64::from(tile_m) / (M2_PER_HA / density).sqrt()).round() as u32;
        n
    }

    /// Ground height at world `(wx, wz)`.
    fn ground(&self, wx: f64, wz: f64) -> f32 {
        let relief = f64::from(self.relief_m);
        let lambda = f64::from(self.relief_wavelength_m);
        if relief == 0.0 || lambda.is_nan() || lambda <= 0.0 {
            return 0.0;
        }
        let y = relief * (TAU * wx / lambda).sin() * (TAU * wz / lambda).cos();
        // At most |relief_m|, an f32.
        #[allow(clippy::cast_possible_truncation)]
        let y = y as f32;
        y
    }
}

/// `v` if it is below `max`, else the largest `f32` below `max` (guards f32 rounding up to an open bound).
fn below(v: f32, max: f32) -> f32 {
    if v < max { v } else { max.next_down() }
}

impl InstanceSource for SyntheticForest {
    fn tile(&self, key: TileKey, tile_m: f32, out: &mut Vec<Instance>) {
        let n = self.cells_per_side(tile_m);
        if n == 0 {
            return;
        }
        let cell_m = f64::from(tile_m) / f64::from(n);
        let [cx, cz] = key.min_corner(tile_m);
        let [s_min, s_max] = self.scale;
        // `TAU as f32` rounds below 2π.
        #[allow(clippy::cast_possible_truncation)]
        let tau = TAU as f32;
        out.reserve(usize::try_from(u64::from(n) * u64::from(n)).unwrap_or(0));
        for iz in 0..n {
            for ix in 0..n {
                let cell = u64::from(iz) * u64::from(n) + u64::from(ix);
                let mut rng = Stream::new(cell_seed(self.seed, key, cell));
                let jx = f64::from(rng.unit());
                let jz = f64::from(rng.unit());
                // Tile-local, in [0, tile_m].
                #[allow(clippy::cast_possible_truncation)]
                let (x, z) = (((f64::from(ix) + jx) * cell_m) as f32, ((f64::from(iz) + jz) * cell_m) as f32);
                let (x, z) = (below(x, tile_m), below(z, tile_m));
                let yaw = below(rng.unit() * tau, tau);
                let scale = s_min + rng.unit() * (s_max - s_min);
                // Below species_count, a u16.
                #[allow(clippy::cast_possible_truncation)]
                let species = rng.below(u32::from(self.species_count)) as u16;
                // The top 16 bits of a u32.
                #[allow(clippy::cast_possible_truncation)]
                let seed = (rng.next_u32() >> 16) as u16;
                let y = self.ground(cx + f64::from(x), cz + f64::from(z));
                out.push(Instance { pos: [x, y, z], yaw, scale, up: [0.0, 1.0, 0.0], species, seed, flags: 0 });
            }
        }
    }
}
