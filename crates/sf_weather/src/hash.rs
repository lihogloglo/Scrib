//! Counter-based hashing: every random choice is a pure function of
//! `(seed, key, salt, counter)`, so the forecast needs no stored RNG state
//! (saves keep the seed only) and any slot can be queried in O(1).

/// `SplitMix64`'s finalizer (Steele, Lea and Flood 2014): a bijective
/// 64-bit mixer with full avalanche.
pub const fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash of a seed, a record key, a salt (which decision) and a counter.
pub const fn hash(seed: u64, key: u64, salt: u64, counter: u64) -> u64 {
    mix(seed ^ mix(key ^ mix(salt ^ mix(counter))))
}

/// Uniform in `[0, 1)` from the top 53 bits.
#[allow(clippy::cast_precision_loss)]
pub fn unit(h: u64) -> f64 {
    (h >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

/// Stable 64-bit key of a record id (FNV-1a over its folded bytes):
/// independent of load order, so adding a mod never reshuffles forecasts.
pub fn key(id: &str) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for b in id.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01B3);
    }
    h
}

/// Smooth value noise in `[0, 1]` over `t` with lattice spacing `period`:
/// continuous in time, so a value derived from it never pops.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::many_single_char_names)]
pub fn value_noise(seed: u64, key: u64, salt: u64, t: f64, period: f64) -> f32 {
    let x = t / period;
    let i = x.floor();
    let f = (x - i) as f32;
    // Two's-complement reinterpretation of the cell index: any bijection works.
    #[allow(clippy::cast_sign_loss)]
    let i = i as i64 as u64;
    let a = unit(hash(seed, key, salt, i)) as f32;
    let b = unit(hash(seed, key, salt, i.wrapping_add(1))) as f32;
    let s = f * f * (3.0 - 2.0 * f);
    a + (b - a) * s
}

/// Salts: one per kind of decision.
pub mod salt {
    pub const PHASE: u64 = 1;
    pub const JITTER: u64 = 2;
    pub const PICK: u64 = 3;
    pub const WIND_SPEED: u64 = 4;
    pub const WIND_DIR: u64 = 5;
    pub const STRIKE_COUNT: u64 = 6;
    pub const STRIKE: u64 = 7;
}
