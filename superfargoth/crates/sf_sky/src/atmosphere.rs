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
    /// Which weather input scales this term.
    pub weather: WeatherScale,
}

/// The weather input that scales a [`MediumTerm`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherScale {
    /// Fixed (gases, ozone).
    #[default]
    None,
    /// [`crate::Weather::haze`]: the background aerosols.
    Haze,
    /// [`crate::Weather::dust`], absorption tinted by
    /// [`crate::Weather::dust_tint`]: dust, ash, smoke.
    Dust,
}

/// The weather's multipliers on the medium ([`crate::Weather::medium`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediumScales {
    pub haze: f32,
    pub dust: f32,
    pub dust_tint: Vec3,
}

impl Default for MediumScales {
    /// A clear day: haze 1, no dust.
    fn default() -> Self {
        Self {
            haze: 1.0,
            dust: 0.0,
            dust_tint: Vec3::ONE,
        }
    }
}

impl MediumScales {
    /// A clear medium with this haze.
    pub fn haze(haze: f32) -> Self {
        Self {
            haze,
            ..Self::default()
        }
    }

    /// The largest difference from `other` in any input, in units of
    /// that input's `steps` (haze, dust, tint): rebuilding a medium's LUTs
    /// is worth it at ≥ 1.
    pub fn change(&self, other: &Self, steps: Vec3) -> f32 {
        let d = Vec3::new(
            (self.haze - other.haze).abs(),
            (self.dust - other.dust).abs(),
            (self.dust_tint - other.dust_tint).abs().max_element(),
        );
        (d / steps.max(Vec3::splat(f32::EPSILON))).max_element()
    }
}

impl MediumTerm {
    pub fn extinction(&self) -> Vec3 {
        self.scattering + self.absorption
    }

    /// Scattering and absorption under the weather `scales`.
    pub fn scaled(&self, scales: &MediumScales) -> (Vec3, Vec3) {
        match self.weather {
            WeatherScale::None => (self.scattering, self.absorption),
            WeatherScale::Haze => (self.scattering * scales.haze, self.absorption * scales.haze),
            WeatherScale::Dust => {
                let d = scales.dust.max(0.0);
                (self.scattering * d, self.absorption * scales.dust_tint.max(Vec3::ZERO) * d)
            }
        }
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
    /// Earth: Rayleigh, Mie and ozone terms, 100 km thick, plus a dust term
    /// that is empty until the weather sets [`crate::Weather::dust`].
    ///
    /// Mie uses the literature split (scattering 3.996e-6, absorption
    /// 0.444e-6, single-scattering albedo 0.9). Bevy 0.20's
    /// `ScatteringMedium::earth` has the two swapped; extinction, and so
    /// transmittance, is the same either way, but its sun halo is ~9× dimmer.
    ///
    /// Dust (weather.md N2), per unit of `dust`: extinction ~9.5e-5 /m at
    /// the ground (Koschmieder visibility ~40 km alone, ~9 km at 4), scale
    /// height 2 km. Single-scattering albedo 0.87 / 0.81 / 0.71 (R/G/B):
    /// mineral dust and volcanic ash absorb blue most (AERONET dust
    /// climatologies; Bevy's `ScatteringMedium::mars` dust has the same
    /// trend). Ours is darker than measured desert dust (~0.93-0.97) for a
    /// grey-brown ash look: the scattered light turns brown and the sun dims
    /// and warms. Asymmetry 0.7, less forward than background haze (larger,
    /// rougher particles).
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
                    weather: WeatherScale::None,
                },
                MediumTerm {
                    scattering: Vec3::splat(3.996e-6),
                    absorption: Vec3::splat(0.444e-6),
                    falloff: Falloff::Exponential { scale: 1.2 / 60.0 },
                    phase: Phase::Mie { asymmetry: 0.8 },
                    weather: WeatherScale::Haze,
                },
                MediumTerm {
                    scattering: Vec3::ZERO,
                    absorption: Vec3::new(0.650e-6, 1.881e-6, 0.085e-6),
                    falloff: Falloff::Tent {
                        center: 0.75,
                        width: 0.3,
                    },
                    phase: Phase::Isotropic,
                    weather: WeatherScale::None,
                },
                // Dust and ash (weather.md N2): empty until `Weather::dust` > 0.
                MediumTerm {
                    scattering: Vec3::new(8.0e-5, 7.6e-5, 7.0e-5),
                    absorption: Vec3::new(1.2e-5, 1.8e-5, 2.8e-5),
                    falloff: Falloff::Exponential { scale: 0.02 },
                    phase: Phase::Mie { asymmetry: 0.7 },
                    weather: WeatherScale::Dust,
                },
            ],
        }
    }

    /// Total extinction per meter at `altitude` meters above the ground.
    pub fn extinction_at(&self, altitude: f32, scales: &MediumScales) -> Vec3 {
        let p = 1.0 - (altitude / (self.outer_radius - self.inner_radius)).clamp(0.0, 1.0);
        self.terms
            .iter()
            .map(|t| {
                let (scattering, absorption) = t.scaled(scales);
                (scattering + absorption) * t.falloff.sample(p)
            })
            .sum()
    }

    /// Transmittance from a point `altitude` meters up toward a direction
    /// `elevation` radians above the horizon, out to space. Zero when the
    /// ray hits the planet. Midpoint quadrature with `samples` steps.
    pub fn transmittance(&self, altitude: f32, elevation: f32, scales: &MediumScales, samples: u32) -> Vec3 {
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
            depth += self.extinction_at(ri - self.inner_radius, scales) * dt;
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
