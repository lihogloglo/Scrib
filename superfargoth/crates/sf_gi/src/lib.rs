//! Bevy-free core of the dynamic probe GI (`docs/specs/gi.md`).
//!
//! A probe grid is traced a budget of probes per frame (round-robin), each with
//! [`MAX_RAYS`] or fewer rays along spherical Fibonacci directions rotated by a
//! per-frame random rotation, so successive updates of one probe see different
//! directions and the hysteresis blend integrates the whole sphere. Each probe stores
//! DDGI's octahedral irradiance and visibility tiles ([`AtlasLayout`], module `ddgi`).
//!
//! The compute shader (`sf_gi_bevy`, `probe_update.wesl`) mirrors [`GridDesc::coord`],
//! [`irradiance_texel`], [`hysteresis_blend`] and the atlas maths; the directions and
//! rotation come from here.

use glam::{Mat3, Quat, UVec3, Vec3};

mod ddgi;
mod fit;
mod lights;

pub use ddgi::{
    border_source, chebyshev, hysteresis_blend, irradiance_texel, max_ray_distance, oct_decode, oct_encode, oct_texel_dir,
    self_shadow_bias, texel_weights, AtlasLayout, DEPTH_SHARPNESS, IRRADIANCE_TEXELS, SELF_SHADOW_BIAS, VISIBILITY_TEXELS,
};
pub use fit::{fit_volume, VolumeBudget, VolumeFit};

pub use lights::{
    box_distance, distance_attenuation, probe_hysteresis, range_falloff, select_nearest, spot_attenuation,
    spot_scale_offset, GiLight, LightTracker, Spot, FLAG_CHANGED, FLAG_SPOT, GPU_LIGHT_WORDS, MOVED_DISTANCE,
    POWER_CHANGE,
};

/// Rays per probe update at most: the shader's workgroup size (one ray per thread).
pub const MAX_RAYS: u32 = 64;

/// Per-volume settings (spec "Spike": 16x8x16 probes at 1 m, 64 rays, 512 per frame, 0.95;
/// slice 2: 0.8 for probes near a changed light, 64 lights).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GiSettings {
    pub dims: UVec3,
    /// Metres between probes.
    pub spacing: f32,
    /// Rays per probe update (clamped to [`MAX_RAYS`]).
    pub rays: u32,
    /// Probes re-traced per frame.
    pub probes_per_frame: u32,
    /// Weight of the previous value in the blend (0: no history, 1: frozen).
    pub hysteresis: f32,
    /// Hysteresis of probes within a changed light's range that frame (a torch lags frames).
    pub changed_hysteresis: f32,
    /// Lights in the list at most (nearest to the volume first).
    pub max_lights: u32,
}

impl Default for GiSettings {
    fn default() -> Self {
        Self { dims: UVec3::new(16, 8, 16), spacing: 1.0, rays: 64, probes_per_frame: 512, hysteresis: 0.95, changed_hysteresis: 0.8, max_lights: 64 }
    }
}

impl GiSettings {
    pub fn grid(&self) -> GridDesc {
        GridDesc { dims: self.dims, spacing: self.spacing }
    }

    /// Rays actually traced per probe: 1..=[`MAX_RAYS`].
    pub fn rays(&self) -> u32 {
        self.rays.clamp(1, MAX_RAYS)
    }
}

/// A regular probe grid, axis-aligned, probes at cell centres (as Bevy's voxels).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridDesc {
    pub dims: UVec3,
    pub spacing: f32,
}

impl GridDesc {
    pub fn probe_count(&self) -> u32 {
        self.dims.x * self.dims.y * self.dims.z
    }

    /// Size of the grid's box in metres (the light probe's scale).
    pub fn extent(&self) -> Vec3 {
        self.dims.as_vec3() * self.spacing
    }

    /// Probe coordinate of a linear index (x fastest, then y, then z).
    pub fn coord(&self, index: u32) -> UVec3 {
        let d = self.dims;
        UVec3::new(index % d.x, (index / d.x) % d.y, index / (d.x * d.y))
    }

    /// Linear index of a probe coordinate.
    pub fn index(&self, coord: UVec3) -> u32 {
        coord.x + self.dims.x * (coord.y + self.dims.y * coord.z)
    }

    /// Probe position relative to the grid's centre.
    pub fn probe_offset(&self, coord: UVec3) -> Vec3 {
        (coord.as_vec3() + 0.5) * self.spacing - self.extent() * 0.5
    }
}

/// `n` spherical Fibonacci directions (unit vectors, near-uniform on the sphere).
pub fn fibonacci_sphere(n: u32) -> Vec<Vec3> {
    let golden = std::f32::consts::PI * (3.0 - 5f32.sqrt());
    #[allow(clippy::cast_precision_loss)]
    let count = n as f32;
    (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let i = i as f32;
            let y = 1.0 - (2.0 * i + 1.0) / count;
            let r = (1.0 - y * y).max(0.0).sqrt();
            let phi = golden * i;
            Vec3::new(r * phi.cos(), y, r * phi.sin())
        })
        .collect()
}

/// A deterministic, uniformly distributed random rotation for frame `frame` (Shoemake's
/// method on a hashed seed): the directions' per-frame rotation.
pub fn frame_rotation(frame: u32) -> Mat3 {
    let seed = frame.wrapping_mul(3);
    let (u1, u2, u3) = (unit(hash(seed.wrapping_add(1))), unit(hash(seed.wrapping_add(2))), unit(hash(seed.wrapping_add(3))));
    let tau = std::f32::consts::TAU;
    let (s1, s2) = ((1.0 - u1).sqrt(), u1.sqrt());
    let q = Quat::from_xyzw(s1 * (tau * u2).sin(), s1 * (tau * u2).cos(), s2 * (tau * u3).sin(), s2 * (tau * u3).cos());
    Mat3::from_quat(q.normalize())
}

/// PCG-style integer hash (Jarzynski and Olano 2020, "pcg").
fn hash(x: u32) -> u32 {
    let state = x.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277_803_737);
    (word >> 22) ^ word
}

/// The top 24 bits of `x` as a float in [0, 1).
fn unit(x: u32) -> f32 {
    #[allow(clippy::cast_precision_loss)]
    let v = (x >> 8) as f32 / (1u32 << 24) as f32;
    v
}

/// This frame's probes: `(first, count)`, round-robin over the grid; probe `i` of the
/// window is `(first + i) % total`.
pub fn frame_window(frame: u32, budget: u32, total: u32) -> (u32, u32) {
    if total == 0 {
        return (0, 0);
    }
    let count = budget.min(total);
    let first = u32::try_from(u64::from(frame) * u64::from(count) % u64::from(total)).unwrap_or(0);
    (first, count)
}

#[cfg(test)]
mod tests;
