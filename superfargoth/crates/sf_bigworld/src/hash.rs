//! The instance sources' integer hash (spec `superfargoth/docs/specs/big-world-rendering.md` "Data flow"): stable
//! across platforms, compilers and releases, so every peer, every run and the GPU producer's CPU twin place the same
//! instances.
//!
//! - A cell's seed folds (world seed, tile x, tile z, cell index) through the `SplitMix64` finaliser one word at a
//!   time ([`cell_seed`]; Steele, Lea, Flood 2014, "Fast Splittable Pseudorandom Number Generators", OOPSLA).
//! - Its random draws are a `SplitMix64` stream from that seed ([`Stream`]): state += golden gamma, output =
//!   finaliser(state).
//!
//! Only wrapping integer arithmetic in the fold: no `std` hasher (`RandomState` is seeded per process), no floats.
//! Changing any constant or the fold order moves every instance in every world: treat it as a format.

use crate::tile::TileKey;

// tunables: const (SplitMix64 golden-ratio increment, a hash format constant)
const GOLDEN_GAMMA: u64 = 0x9e37_79b9_7f4a_7c15;
// tunables: const (SplitMix64 / Mix13 multiplier, a hash format constant)
const MIX_A: u64 = 0xbf58_476d_1ce4_e5b9;
// tunables: const (SplitMix64 / Mix13 multiplier, a hash format constant)
const MIX_B: u64 = 0x94d0_49bb_1331_11eb;
// tunables: const (domain separator so seed 0 at tile (0, 0) does not start from a zero state, a hash format constant)
const DOMAIN: u64 = 0x6269_6777_6f72_6c64; // "bigworld"
// tunables: const (2^-24, maps the top 24 bits of a draw to [0, 1), a format constant)
const UNIT_24: f32 = 1.0 / 16_777_216.0;

/// `SplitMix64` finaliser: a bijective 64-bit avalanche mix.
#[inline]
pub(crate) const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(MIX_A);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_B);
    z ^ (z >> 31)
}

/// Seed of cell `cell` of tile `key` in world `world_seed`: fold order (world seed, key.x, key.z, cell index).
pub(crate) const fn cell_seed(world_seed: u64, key: TileKey, cell: u64) -> u64 {
    let mut h = mix64(DOMAIN ^ world_seed);
    h = mix64(h ^ key.x.cast_unsigned() as u64);
    h = mix64(h ^ key.z.cast_unsigned() as u64);
    mix64(h ^ cell)
}

/// A `SplitMix64` stream.
pub(crate) struct Stream {
    state: u64,
}

impl Stream {
    pub(crate) const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next 64 random bits.
    pub(crate) const fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);
        mix64(self.state)
    }

    /// The next 32 random bits (the top half of a draw).
    pub(crate) const fn next_u32(&mut self) -> u32 {
        // The top 32 bits of a u64.
        #[allow(clippy::cast_possible_truncation)]
        let v = (self.next_u64() >> 32) as u32;
        v
    }

    /// Uniform in `[0, 1)`, 24-bit resolution.
    pub(crate) fn unit(&mut self) -> f32 {
        // Below 2^24: exact in an f32.
        #[allow(clippy::cast_precision_loss)]
        let v = (self.next_u32() >> 8) as f32 * UNIT_24;
        v
    }

    /// Uniform integer in `0..n` (0 when `n` is 0), Lemire's multiply-shift.
    pub(crate) const fn below(&mut self, n: u32) -> u32 {
        // (u32 × u32) >> 32 is below n.
        #[allow(clippy::cast_possible_truncation)]
        let v = ((self.next_u32() as u64 * n as u64) >> 32) as u32;
        v
    }
}
