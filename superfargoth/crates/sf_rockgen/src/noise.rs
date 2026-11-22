//! Counter-based randomness and value noise: every number is a hash of (seed, counter) or (seed, lattice point),
//! so results never depend on call order across threads (splitmix64, Steele et al. 2014, the finaliser
//! `sf_worldgen::scatter` uses too).

use glam::Vec3;

/// splitmix64's finaliser: a bijective 64-bit mix.
pub(crate) const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

const GOLDEN: u64 = 0x9e37_79b9_7f4a_7c15;

/// Maps the top 24 bits of a hash to `[0, 1)`.
fn unit(h: u64) -> f32 {
    // 24 bits fit an f32 mantissa exactly.
    let bits = u32::try_from(h >> 40).unwrap_or(0);
    #[allow(clippy::cast_precision_loss)]
    let x = bits as f32;
    x * (1.0 / 16_777_216.0)
}

/// A seeded stream: the n-th draw is `mix64(seed + n·golden)`, the splitmix64 sequence.
pub(crate) struct Stream {
    state: u64,
}

impl Stream {
    pub(crate) fn new(seed: u64, purpose: u64) -> Self {
        Self { state: mix64(seed ^ mix64(purpose.wrapping_add(GOLDEN))) }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN);
        mix64(self.state)
    }

    /// Uniform in `[0, 1)`.
    pub(crate) fn unit(&mut self) -> f32 {
        unit(self.next_u64())
    }

    /// Uniform in `[-1, 1)`.
    pub(crate) fn signed(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }

    /// Uniform direction on the unit sphere (Marsaglia's method without rejection: z uniform, angle uniform).
    pub(crate) fn direction(&mut self) -> Vec3 {
        let z = self.signed();
        let phi = self.unit() * std::f32::consts::TAU;
        let r = (1.0 - z * z).max(0.0).sqrt();
        Vec3::new(r * phi.cos(), r * phi.sin(), z)
    }
}

/// Hash of a lattice point to `[-1, 1)`.
fn lattice(seed: u64, x: i32, y: i32, z: i32) -> f32 {
    let k = seed
        ^ u64::from(x.cast_unsigned()).wrapping_mul(0x8cb9_2ba7_2f3d_8dd7)
        ^ u64::from(y.cast_unsigned()).wrapping_mul(0xd6e8_feb8_6659_fd93)
        ^ u64::from(z.cast_unsigned()).wrapping_mul(0xa076_1d64_78bd_642f);
    unit(mix64(k)) * 2.0 - 1.0
}

/// Quintic fade (Perlin 2002): C2-continuous interpolation weights.
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Trilinear value noise in `[-1, 1]`, unit lattice.
pub(crate) fn value3(seed: u64, point: Vec3) -> f32 {
    let cell = point.floor();
    let frac = point - cell;
    #[allow(clippy::cast_possible_truncation)]
    let [ix, iy, iz] = [cell.x as i32, cell.y as i32, cell.z as i32];
    let [fx, fy, fz] = [fade(frac.x), fade(frac.y), fade(frac.z)];
    let corner = |dx, dy, dz| lattice(seed, ix + dx, iy + dy, iz + dz);
    let lerp = |from: f32, to: f32, s: f32| from + (to - from) * s;
    let x00 = lerp(corner(0, 0, 0), corner(1, 0, 0), fx);
    let x10 = lerp(corner(0, 1, 0), corner(1, 1, 0), fx);
    let x01 = lerp(corner(0, 0, 1), corner(1, 0, 1), fx);
    let x11 = lerp(corner(0, 1, 1), corner(1, 1, 1), fx);
    lerp(lerp(x00, x10, fy), lerp(x01, x11, fy), fz)
}

/// Fractal sum of `octaves` value-noise octaves (lacunarity 2, gain 0.5), normalised to `[-1, 1]`.
pub(crate) fn fbm(seed: u64, p: Vec3, octaves: u32) -> f32 {
    let (mut sum, mut norm, mut amp, mut freq, mut offset) = (0.0, 0.0, 1.0, 1.0, 0.0);
    for o in 0..octaves {
        // A per-octave seed and offset so octaves don't share lattice points at the origin.
        sum += amp * value3(mix64(seed ^ u64::from(o)), p * freq + Vec3::splat(offset));
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
        offset += 17.31;
    }
    if norm > 0.0 { sum / norm } else { 0.0 }
}
