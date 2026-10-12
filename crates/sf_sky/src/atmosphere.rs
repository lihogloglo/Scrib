//! The atmosphere's participating medium, and CPU sun transmittance.
//!
//! One description feeds both the GPU sky (the plugin turns it into Bevy's
//! `ScatteringMedium`) and the CPU transmittance used to color sunlight on
//! render paths whose shaders don't apply it (Solari). Coefficients are the
//! standard Earth values (Bruneton 2017; Hillaire, *A Scalable and
//! Production Ready Sky and Atmosphere Rendering Technique*, 2020), and the
//! density profiles use the same normalized-altitude falloff functions as
//! Bevy's atmosphere, so both paths agree.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Density as a function of normalized height `p`: 1 at the ground, 0 at
/// the top of the atmosphere, linear in altitude.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Falloff {
    /// `f(p) = p`.
    Linear,
    /// Exponential decay with `scale` as a fraction of the atmosphere's
    /// height, normalized so `f(1) = 1`, `f(0) = 0`.
    Exponential { scale: f32 },
    /// A triangle peaking at `center`, zero at `center ± width / 2`.
    Tent { center: f32, width: f32 },
}

impl Falloff {
    pub fn sample(&self, p: f32) -> f32 {
        match *self {
            Self::Linear | Self::Exponential { scale: 0.0 } => p,
            Self::Exponential { scale } => {
                let s = -1.0 / scale;
                (((1.0 - p) * s).exp() - s.exp()) / (1.0 - s.exp())
            }
            Self::Tent { center, width } => (1.0 - (p - center).abs() / (0.5 * width)).max(0.0),
        }
    }
}

/// Angular distribution of scattered light.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Isotropic,
    /// Molecules (small particles).
    Rayleigh,
    /// Aerosols: Cornette-Shanks / Henyey-Greenstein with this asymmetry.
    Mie {
        asymmetry: f32,
    },
}

/// One constituent of the medium (gas, aerosol, ozone).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediumTerm {
    /// Scattering coefficient per meter at sea-level density (RGB).
    pub scattering: Vec3,
    /// Absorption coefficient per meter at sea-level density (RGB).
    pub absorption: Vec3,
    pub falloff: Falloff,
    pub phase: Phase,
    /// Scaled by [`crate::Weather::haze`] (aerosols).
    pub weather_scaled: bool,
}

impl MediumTerm {
    pub fn extinction(&self) -> Vec3 {
        self.scattering + self.absorption
    }
}

/// A planet's atmosphere.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtmosphereModel {
    /// Planet radius, meters.
    pub inner_radius: f32,
    /// Radius of the top of the atmosphere, meters.
    pub outer_radius: f32,
    /// Average ground albedo, used for multiple scattering.
    pub ground_albedo: Vec3,
    pub terms: Vec<MediumTerm>,
}

impl Default for AtmosphereModel {
    fn default() -> Self {
        Self::earth()
    }
}

impl AtmosphereModel {
    /// Earth: Rayleigh, Mie and ozone terms, 100 km thick.
    ///
    /// Mie uses the literature split (scattering 3.996e-6, absorption
    /// 0.444e-6, single-scattering albedo 0.9). Bevy 0.20's
    /// `ScatteringMedium::earth` has the two swapped; extinction, and so
    /// transmittance, is the same either way, but its sun halo is ~9× dimmer.
    pub fn earth() -> Self {
        Self {
            inner_radius: 6_360_000.0,
            outer_radius: 6_460_000.0,
            ground_albedo: Vec3::splat(0.3),
            terms: vec![
                MediumTerm {
                    scattering: Vec3::new(5.802e-6, 13.558e-6, 33.1e-6),
                    absorption: Vec3::ZERO,
                    falloff: Falloff::Exponential { scale: 8.0 / 60.0 },
                    phase: Phase::Rayleigh,
                    weather_scaled: false,
                },
                MediumTerm {
                    scattering: Vec3::splat(3.996e-6),
                    absorption: Vec3::splat(0.444e-6),
                    falloff: Falloff::Exponential { scale: 1.2 / 60.0 },
                    phase: Phase::Mie { asymmetry: 0.8 },
                    weather_scaled: true,
                },
                MediumTerm {
                    scattering: Vec3::ZERO,
                    absorption: Vec3::new(0.650e-6, 1.881e-6, 0.085e-6),
                    falloff: Falloff::Tent {
                        center: 0.75,
                        width: 0.3,
                    },
                    phase: Phase::Isotropic,
                    weather_scaled: false,
                },
            ],
        }
    }

    /// Total extinction per meter at `altitude` meters above the ground.
    pub fn extinction_at(&self, altitude: f32, haze: f32) -> Vec3 {
        let p = 1.0 - (altitude / (self.outer_radius - self.inner_radius)).clamp(0.0, 1.0);
        self.terms
            .iter()
            .map(|t| {
                let scale = if t.weather_scaled { haze } else { 1.0 };
                t.extinction() * t.falloff.sample(p) * scale
            })
            .sum()
    }

    /// Transmittance from a point `altitude` meters up toward a direction
    /// `elevation` radians above the horizon, out to space. Zero when the
    /// ray hits the planet. Midpoint quadrature with `samples` steps.
    pub fn transmittance(&self, altitude: f32, elevation: f32, haze: f32, samples: u32) -> Vec3 {
        let r = self.inner_radius + altitude.max(0.0);
        let mu = elevation.sin();
        // Ray-sphere: distance to the ground and to the top.
        let disc_ground = r * r * (mu * mu - 1.0) + self.inner_radius * self.inner_radius;
        if mu < 0.0 && disc_ground >= 0.0 {
            return Vec3::ZERO;
        }
        let disc_top = r * r * (mu * mu - 1.0) + self.outer_radius * self.outer_radius;
        let length = -r * mu + disc_top.max(0.0).sqrt();
        let samples = samples.max(1);
        #[expect(clippy::cast_precision_loss, reason = "small sample count")]
        let dt = length / samples as f32;
        let mut depth = Vec3::ZERO;
        for i in 0..samples {
            #[expect(clippy::cast_precision_loss, reason = "small sample count")]
            let t = (i as f32 + 0.5) * dt;
            // Radius at distance t along the ray (law of cosines).
            let ri = (r * r + t * t + 2.0 * r * t * mu).sqrt();
            depth += self.extinction_at(ri - self.inner_radius, haze) * dt;
        }
        (-depth).exp()
    }

    /// Elevation of the true horizon (radians, ≤ 0) seen from `altitude`.
    pub fn horizon_elevation(&self, altitude: f32) -> f32 {
        let r = self.inner_radius + altitude.max(0.0);
        -(self.inner_radius / r).clamp(-1.0, 1.0).acos()
    }
}

/// Fraction of a disk of angular diameter `angular_size` (radians) whose
/// center is `elevation` radians above the horizon that is visible.
pub fn visible_disk_fraction(elevation: f32, horizon: f32, angular_size: f32) -> f32 {
    let radius = 0.5 * angular_size;
    if radius <= 0.0 {
        return if elevation >= horizon { 1.0 } else { 0.0 };
    }
    let d = ((elevation - horizon) / radius).clamp(-1.0, 1.0);
    // Area of the circular segment above the chord at signed distance d.
    (std::f32::consts::PI - d.acos() + d * (1.0 - d * d).sqrt()) / std::f32::consts::PI
}
