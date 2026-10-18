//! The uploaded form of the wind (`WindParams` uniform + `WindSource`
//! storage array in `shaders/sf_wind.wesl`) and its CPU sampler.

use glam::{Vec2, Vec3, Vec4};

use crate::field::{self, GustSample};

/// WGSL `WindParams` (std140/std430-compatible: six `vec4<f32>`, 96 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindParams {
    /// Prevailing direction (unit xz), mean speed, gustiness.
    pub axis_speed: Vec4,
    /// Fractional lattice offset (xz, cells in `[0, 1)`), 1 / cell size,
    /// cells per period.
    pub octave0: Vec4,
    pub octave1: Vec4,
    /// Whole lattice offsets (cells): octave 0 xz, octave 1 xz.
    pub cells: Vec4,
    /// Swirl, gust bias, gust band, source count.
    pub shape: Vec4,
    /// z0, 1 / ln(reference / z0 + 1), max profile, reference height.
    pub profile: Vec4,
}

impl WindParams {
    /// The bytes to upload, as plain floats (`bytemuck`-castable).
    pub fn to_array(&self) -> [[f32; 4]; 6] {
        [
            self.axis_speed.to_array(),
            self.octave0.to_array(),
            self.octave1.to_array(),
            self.cells.to_array(),
            self.shape.to_array(),
            self.profile.to_array(),
        ]
    }
}

/// WGSL `WindSource` (three `vec4<f32>`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindSourceGpu {
    /// Origin-relative center, kind (0 directional, 1 radial, 2 vortex).
    pub a: Vec4,
    /// Direction or axis, strength (envelope applied).
    pub b: Vec4,
    /// Inner radius, radius, unused, unused.
    pub c: Vec4,
}

impl WindSourceGpu {
    pub fn to_array(&self) -> [[f32; 4]; 3] {
        [self.a.to_array(), self.b.to_array(), self.c.to_array()]
    }
}

/// The wind for one frame around one floating origin: the GPU upload and
/// the CPU twin of `sf_wind.wesl` (cloth, particles, audio, tests).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindFrame {
    pub params: WindParams,
    /// At most [`crate::MAX_SOURCES`].
    pub sources: Vec<WindSourceGpu>,
}

impl WindFrame {
    /// Total velocity (m/s) at an origin-relative position, `above_ground`
    /// meters over the ground: the global wind plus every source.
    pub fn sample(&self, rel: Vec3, above_ground: f32) -> Vec3 {
        self.sources
            .iter()
            .fold(field::global(&self.params, rel, above_ground), |v, s| v + field::source(s, rel))
    }

    /// The weather's wind alone (WGSL `wind_global`).
    pub fn global(&self, rel: Vec3, above_ground: f32) -> Vec3 {
        field::global(&self.params, rel, above_ground)
    }

    /// The gust field (WGSL `wind_gust`).
    pub fn gust(&self, rel_xz: Vec2) -> GustSample {
        field::gust(&self.params, rel_xz)
    }

    /// The log-profile factor (WGSL `wind_profile`).
    pub fn profile(&self, above_ground: f32) -> f32 {
        field::profile(&self.params, above_ground)
    }

    /// Height (m) where the profile is 1: pass it when the height above
    /// ground is unknown.
    pub fn reference_height(&self) -> f32 {
        self.params.profile.w
    }
}
