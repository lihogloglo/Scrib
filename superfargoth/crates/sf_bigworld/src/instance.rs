//! The 16-byte tile instance format (spec `superfargoth/docs/specs/big-world-rendering.md` "Data flow"): a tile's
//! instances, quantised against the tile, packed into four little-endian `u32` words each and uploaded as one GPU
//! pool block in the order given.
//!
//! This is a format, like a file format: the shaders decode the same words. Changing a constant, a bit position or a
//! rounding rule changes every packed tile. Quantisation per field:
//!
//! - x, z: `u16`, tile-local in `[0, tile_m]`, step `tile_m / 65535` (≈ 3.9 mm at 256 m).
//! - y: `u16` over the tile's height range `[y_min, y_max]` (in the [`TileHeader`]); a flat tile stores 0.
//! - yaw: `u8`, 1.4° steps over `[0, 2π)`.
//! - scale: `u8`, logarithmic over `[0.25, 4]` (≈ 1.1 % steps); inputs outside are clamped.
//! - up: two `u8`, octahedral encoding with `+y` as the pole (Cigolle et al. 2014, "A Survey of Efficient
//!   Representations for Independent Unit Vectors", JCGT 3(2)); `+y` is exact.
//! - species, seed, flags: verbatim `u16`.
//!
//! Precision note: the decoded height is an `f32`, so the y step is only meaningful while it is above the `f32`
//! spacing at the tile's heights (always the case for terrain ranges of metres and more).

use std::f64::consts::TAU;

use crate::tile::TileKey;

// tunables: const (u16 quantisation maximum of x, z and y, a format constant)
const Q16_MAX: f64 = 65_535.0;
// tunables: const (yaw steps per turn of the u8 yaw, a format constant)
const YAW_STEPS: f64 = 256.0;
// tunables: const (smallest encodable scale, low end of the u8 log scale, a format constant)
const SCALE_MIN: f64 = 0.25;
// tunables: const (largest encodable scale, high end of the u8 log scale, a format constant)
const SCALE_MAX: f64 = 4.0;
// tunables: const (octaves covered by the u8 log scale: log2(SCALE_MAX / SCALE_MIN), a format constant)
const SCALE_OCTAVES: f64 = 4.0;
// tunables: const (u8 scale quantisation maximum, a format constant)
const SCALE_Q_MAX: f64 = 255.0;
// tunables: const (octahedral coordinate scale and zero offset: v in [-1, 1] stored as round(v·127) + 127, a format constant)
const OCT_HALF: f64 = 127.0;

/// One unpacked instance of a tile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instance {
    /// x and z tile-local in `[0, tile_m]`; y = absolute height in metres.
    pub pos: [f32; 3],
    /// Radians about `+y`, `[0, 2π)`.
    pub yaw: f32,
    /// Uniform scale.
    pub scale: f32,
    /// Unit vector, the object's up axis (`+y` = upright).
    pub up: [f32; 3],
    /// Species index (meaning is the source's).
    pub species: u16,
    /// Per-instance random bits for the shaders (tint, wind phase).
    pub seed: u16,
    /// Per-instance flags (meaning is the source's).
    pub flags: u16,
}

/// The 16-byte GPU instance: four little-endian u32 words.
///   w0 = `x_q | z_q << 16`
///   w1 = `y_q | yaw_q << 16 | scale_q << 24`
///   w2 = `species | seed << 16`
///   w3 = `flags | up_u << 16 | up_v << 24`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PackedInstance(pub [u32; 4]);

impl PackedInstance {
    /// Size of one packed instance in bytes.
    // tunables: const (size of one packed instance in bytes, a format constant)
    pub const BYTES: usize = 16;

    /// The four words as little-endian bytes, w0 first: the GPU layout.
    #[must_use]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut out = [0_u8; 16];
        for (chunk, word) in out.as_chunks_mut::<4>().0.iter_mut().zip(self.0) {
            *chunk = word.to_le_bytes();
        }
        out
    }
}

/// Per-tile data the shaders need to decode its instances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TileHeader {
    /// The tile.
    pub key: TileKey,
    /// Lowest instance height in metres (0 for an empty tile).
    pub y_min: f32,
    /// Highest instance height in metres (0 for an empty tile).
    pub y_max: f32,
    /// Number of instances.
    pub count: u32,
}

/// A packed tile: its header and its instances in upload order.
#[derive(Clone, Debug, PartialEq)]
pub struct PackedTile {
    /// Decode data.
    pub header: TileHeader,
    /// The instances, in upload order.
    pub instances: Vec<PackedInstance>,
}

impl PackedTile {
    /// Packs `instances` of tile `key`. `y_min` / `y_max` are the range of the input heights (0, 0 when empty).
    ///
    /// # Panics
    /// If a tile holds more than `u32::MAX` instances.
    #[must_use]
    pub fn pack(key: TileKey, tile_m: f32, instances: &[Instance]) -> Self {
        let (y_min, y_max) = if instances.is_empty() {
            (0.0, 0.0)
        } else {
            instances
                .iter()
                .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), i| (lo.min(i.pos[1]), hi.max(i.pos[1])))
        };
        let count = u32::try_from(instances.len()).expect("a tile holds at most u32::MAX instances");
        let header = TileHeader { key, y_min, y_max, count };
        let instances = instances.iter().map(|i| pack_one(i, tile_m, y_min, y_max)).collect();
        Self { header, instances }
    }

    /// Decodes instance `index` (x, z tile-local, y absolute).
    ///
    /// # Panics
    /// If `index` is out of range.
    #[must_use]
    pub fn unpack(&self, index: usize, tile_m: f32) -> Instance {
        let [w0, w1, w2, w3] = self.instances[index].0;
        let tile = f64::from(tile_m);
        let x = f64::from(lo16(w0)) / Q16_MAX * tile;
        let z = f64::from(hi16(w0)) / Q16_MAX * tile;
        let y = decode_y(lo16(w1), self.header.y_min, self.header.y_max);
        let yaw = f64::from(byte(w1, 2)) / YAW_STEPS * TAU;
        let scale = SCALE_MIN * (f64::from(byte(w1, 3)) / SCALE_Q_MAX * SCALE_OCTAVES).exp2();
        let up = decode_oct(byte(w3, 2), byte(w3, 3));
        Instance {
            pos: [to_f32(x), y, to_f32(z)],
            yaw: to_f32(yaw),
            scale: to_f32(scale),
            up,
            species: lo16(w2),
            seed: hi16(w2),
            flags: lo16(w3),
        }
    }

    /// World position of instance `index`: the tile's min corner plus its decoded local position, in `f64`.
    ///
    /// # Panics
    /// If `index` is out of range.
    #[must_use]
    pub fn world_position(&self, index: usize, tile_m: f32) -> [f64; 3] {
        let p = self.unpack(index, tile_m).pos;
        let [cx, cz] = self.header.key.min_corner(tile_m);
        [cx + f64::from(p[0]), f64::from(p[1]), cz + f64::from(p[2])]
    }

    /// Appends the instances only, 16 B each, in upload order.
    pub fn write_le_bytes(&self, out: &mut Vec<u8>) {
        out.reserve(self.instances.len() * PackedInstance::BYTES);
        for inst in &self.instances {
            out.extend_from_slice(&inst.to_le_bytes());
        }
    }
}

fn pack_one(i: &Instance, tile_m: f32, y_min: f32, y_max: f32) -> PackedInstance {
    let tile = f64::from(tile_m);
    let x_q = quant16(f64::from(i.pos[0]) / tile);
    let z_q = quant16(f64::from(i.pos[2]) / tile);
    let range = f64::from(y_max) - f64::from(y_min);
    let y_q = if range > 0.0 { quant16((f64::from(i.pos[1]) - f64::from(y_min)) / range) } else { 0 };
    let yaw_q = quant_yaw(f64::from(i.yaw));
    let scale_q = quant_scale(f64::from(i.scale));
    let (up_u, up_v) = encode_oct(i.up);
    PackedInstance([
        x_q | z_q << 16,
        y_q | yaw_q << 16 | scale_q << 24,
        u32::from(i.species) | u32::from(i.seed) << 16,
        u32::from(i.flags) | up_u << 16 | up_v << 24,
    ])
}

/// `round(t × 65535)` clamped to `0..=65535`; NaN gives 0.
fn quant16(t: f64) -> u32 {
    // Clamped to 0..=65535 just above (`as` maps NaN to 0).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let q = (t * Q16_MAX).round().clamp(0.0, Q16_MAX) as u32;
    q
}

/// `round(yaw / 2π × 256) mod 256`.
fn quant_yaw(yaw: f64) -> u32 {
    // Far beyond any yaw in radians; `as` saturates and maps NaN to 0.
    #[allow(clippy::cast_possible_truncation)]
    let steps = (yaw / TAU * YAW_STEPS).round() as i64;
    // rem_euclid(256) is in 0..256.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let q = steps.rem_euclid(256) as u32;
    q
}

/// `round(log2(s / 0.25) / 4 × 255)`, `s` clamped to `[0.25, 4]`.
fn quant_scale(s: f64) -> u32 {
    let s = s.clamp(SCALE_MIN, SCALE_MAX);
    // In 0..=255 by the clamp above (`as` maps NaN to 0).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let q = ((s / SCALE_MIN).log2() / SCALE_OCTAVES * SCALE_Q_MAX).round().clamp(0.0, SCALE_Q_MAX) as u32;
    q
}

/// Height from its `u16`: a lerp that is exact at both ends, clamped to the range so a re-pack keeps the header.
fn decode_y(q: u16, y_min: f32, y_max: f32) -> f32 {
    let t = f64::from(q) / Q16_MAX;
    let y = (1.0 - t) * f64::from(y_min) + t * f64::from(y_max);
    to_f32(y).clamp(y_min, y_max)
}

/// `+1` for zero and positive values (the octahedral "sign not zero").
fn sign_nz(v: f64) -> f64 {
    if v >= 0.0 { 1.0 } else { -1.0 }
}

/// Octahedral encoding with `+y` as the pole: project on the L1 sphere, fold the lower half over the diagonals,
/// store each of (x, z) as `round(v × 127) + 127`. A zero vector encodes as `+y`.
fn encode_oct(up: [f32; 3]) -> (u32, u32) {
    let [x, y, z] = up.map(f64::from);
    let l1 = x.abs() + y.abs() + z.abs();
    if l1.is_nan() || l1 <= 0.0 {
        return (127, 127);
    }
    let (mut px, mut pz) = (x / l1, z / l1);
    if y < 0.0 {
        (px, pz) = ((1.0 - pz.abs()) * sign_nz(px), (1.0 - px.abs()) * sign_nz(pz));
    }
    let q = |v: f64| {
        // In 0..=254 by the clamp (`as` maps NaN to 0).
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let q = ((v * OCT_HALF).round() + OCT_HALF).clamp(0.0, 2.0 * OCT_HALF) as u32;
        q
    };
    (q(px), q(pz))
}

/// Inverse of [`encode_oct`], renormalised. `(127, 127)` decodes exactly to `[0, 1, 0]`.
fn decode_oct(u: u8, v: u8) -> [f32; 3] {
    let mut px = (f64::from(u) - OCT_HALF) / OCT_HALF;
    let mut pz = (f64::from(v) - OCT_HALF) / OCT_HALF;
    let y = 1.0 - px.abs() - pz.abs();
    if y < 0.0 {
        (px, pz) = ((1.0 - pz.abs()) * sign_nz(px), (1.0 - px.abs()) * sign_nz(pz));
    }
    let len = (px * px + y * y + pz * pz).sqrt();
    [to_f32(px / len), to_f32(y / len), to_f32(pz / len)]
}

fn to_f32(v: f64) -> f32 {
    // Decoded values are tile-local, angles, scales or heights: well inside f32.
    #[allow(clippy::cast_possible_truncation)]
    let f = v as f32;
    f
}

fn lo16(w: u32) -> u16 {
    // Masked to 16 bits.
    #[allow(clippy::cast_possible_truncation)]
    let h = (w & 0xffff) as u16;
    h
}

fn hi16(w: u32) -> u16 {
    // Shifted to 16 bits.
    #[allow(clippy::cast_possible_truncation)]
    let h = (w >> 16) as u16;
    h
}

fn byte(w: u32, i: u32) -> u8 {
    // Masked to 8 bits.
    #[allow(clippy::cast_possible_truncation)]
    let b = ((w >> (8 * i)) & 0xff) as u8;
    b
}
