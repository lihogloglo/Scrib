//! The scatter's integer hash: stable across platforms, compilers and releases, so every peer and every run
//! places the same instances (net-ready, spec "Scatter").
//!
//! - Species ids hash with 64-bit FNV-1a over their UTF-8 bytes ([`species_hash`]).
//! - A candidate's seed folds (tile x, tile z, species hash, cell index) through the `SplitMix64` finaliser
//!   (Stafford's "Mix13", the `splitmix64` output function) one word at a time ([`candidate_seed`]).
//! - Its random draws are a `SplitMix64` stream from that seed ([`Stream`]): state += golden gamma, output =
//!   finaliser(state), top 32 bits kept.
//!
//! Only wrapping integer arithmetic: no `std` hasher (`RandomState` is seeded per process), no floats.
//! Changing any constant or the fold order moves every instance in every world: treat it as a format.

use crate::scatter::TileKey;

// tunables: const (FNV-1a 64 offset basis, a hash format constant)
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
// tunables: const (FNV-1a 64 prime, a hash format constant)
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
// tunables: const (SplitMix64 golden-ratio increment, a hash format constant)
const GOLDEN_GAMMA: u64 = 0x9e37_79b9_7f4a_7c15;
// tunables: const (SplitMix64 / Mix13 multiplier, a hash format constant)
const MIX_A: u64 = 0xbf58_476d_1ce4_e5b9;
// tunables: const (SplitMix64 / Mix13 multiplier, a hash format constant)
const MIX_B: u64 = 0x94d0_49bb_1331_11eb;
// tunables: const (domain separator so tile (0, 0) does not start from a zero state, a hash format constant)
const DOMAIN: u64 = 0x6772_6f75_6e64_6376; // "groundcv"

/// `SplitMix64` finaliser: a bijective 64-bit avalanche mix.
#[inline]
pub const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(MIX_A);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_B);
    z ^ (z >> 31)
}

/// 64-bit FNV-1a of a species id. Stable: the same id gives the same hash on every machine.
pub fn species_hash(id: &str) -> u64 {
    id.bytes().fold(FNV_OFFSET, |h, b| (h ^ u64::from(b)).wrapping_mul(FNV_PRIME))
}

/// The seed of one scatter candidate: (world tile, species, cell index), folded through [`mix64`].
#[inline]
pub fn candidate_seed(tile: TileKey, species_hash: u64, cell: u32) -> u64 {
    // `as u64` on i64 is a two's-complement reinterpretation: exact and the same everywhere.
    #[allow(clippy::cast_sign_loss)]
    let (x, z) = (tile.x as u64, tile.z as u64);
    let mut h = mix64(DOMAIN ^ x);
    h = mix64(h ^ z);
    h = mix64(h ^ species_hash);
    mix64(h ^ u64::from(cell))
}

/// A `SplitMix64` stream of draws from one candidate seed.
#[derive(Clone, Copy, Debug)]
pub struct Stream(u64);

impl Stream {
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// The next 32 random bits.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(GOLDEN_GAMMA);
        // The top half of the mix: its best-mixed bits.
        #[allow(clippy::cast_possible_truncation)]
        let v = (mix64(self.0) >> 32) as u32;
        v
    }

    /// A uniform value in `[0, 1)` with 24 bits of resolution (exactly representable in f32, so the
    /// result is the same everywhere).
    #[inline]
    pub fn next_unit(&mut self) -> f32 {
        // 24-bit integer → f32 is exact; the scale is a power of two, also exact.
        #[allow(clippy::cast_precision_loss)]
        let v = (self.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0);
        v
    }

    /// The next 8 random bits.
    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        self.next_u32().to_le_bytes()[3]
    }
}
