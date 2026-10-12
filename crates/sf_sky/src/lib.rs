//! Sky core (Bevy-free). Spec: `docs/specs/sky-lighting.md`.
//!
//! - [`SkyClock`] and [`Calendar`]: game time as days since an epoch.
//! - [`Orbit`]: sun and moon positions from time of day, season, latitude
//!   and axial tilt.
//! - [`AtmosphereModel`]: the scattering medium (fed to the GPU sky) and
//!   CPU sun transmittance (for render paths whose shaders don't apply it).
//! - [`SkySettings::evaluate`]: everything the renderer applies each frame
//!   ([`SkyState`]): light directions and intensities, shadow caster,
//!   exposure, ambient floor.
//! - [`Moon`], [`StarField`], [`SunDiskLook`]: the drawn sky bodies (any
//!   number of moons with their own paths and phases, stars, the sun's
//!   artistic disk and glare), evaluated into [`NightSky`].
//! - [`Weather`] and [`Wind`]: the M6 hook, and the shared wind.
//! - [`RenderPath`] and [`sphere_light_luminance`]: the raster/ray-traced
//!   switch and the emissive-proxy conversion for punctual lights.
//!
//! The Bevy glue lives in `sf_sky_bevy`.

mod atmosphere;
mod celestial;
mod clock;
mod curve;
mod night;
mod sky;
mod weather;

#[cfg(test)]
mod tests;

/// The math types used in this API.
pub use glam;

pub use atmosphere::{AtmosphereModel, Falloff, MediumScales, MediumTerm, Phase, WeatherScale, visible_disk_fraction};
pub use celestial::{Celestial, Orbit, SkyPosition};
pub use clock::{Calendar, SkyClock};
pub use curve::KeyCurve;
pub use night::{MAX_MOONS, Moon, MoonPath, MoonState, NightSky, StarField, SunDiskLook, phase_light};
pub use sky::{ExposureRange, SkyLight, SkySettings, SkyState, luminance};
pub use weather::{Weather, Wind};

use serde::{Deserialize, Serialize};

/// How the scene is lit. A runtime setting, like a graphics option.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RenderPath {
    /// Rasterized lighting: clustered forward, shadow maps, screen-space
    /// effects. Runs everywhere; the performance baseline.
    #[default]
    Raster,
    /// Ray-traced direct and indirect lighting (Bevy Solari). Needs
    /// hardware ray queries; falls back to [`RenderPath::Raster`] without.
    RayTraced,
}

impl RenderPath {
    /// The path that actually runs: the requested one if supported.
    #[must_use]
    pub fn resolve(self, ray_tracing_supported: bool) -> Self {
        match self {
            Self::RayTraced if ray_tracing_supported => Self::RayTraced,
            _ => Self::Raster,
        }
    }
}

/// Temporal upscaling and anti-aliasing, a graphics setting next to
/// [`RenderPath`]. On the ray-traced path the same quality drives the
/// denoiser's upscaling (DLSS Ray Reconstruction).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Upscaling {
    /// Native resolution, the renderer's own anti-aliasing (MSAA).
    #[default]
    Off,
    /// NVIDIA DLSS Super Resolution (RTX GPUs; falls back to `Off`).
    Dlss(UpscaleQuality),
}

/// Render resolution of a temporal upscaler, as a fraction of the output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpscaleQuality {
    /// The upscaler picks from the output resolution.
    #[default]
    Auto,
    /// Native resolution, temporal anti-aliasing only (DLAA).
    Native,
    /// 67 % per axis.
    Quality,
    /// 58 % per axis.
    Balanced,
    /// 50 % per axis.
    Performance,
    /// 33 % per axis.
    UltraPerformance,
}

impl UpscaleQuality {
    const NAMES: [(&'static str, Self); 7] = [
        ("auto", Self::Auto),
        ("native", Self::Native),
        ("dlaa", Self::Native),
        ("quality", Self::Quality),
        ("balanced", Self::Balanced),
        ("performance", Self::Performance),
        ("ultra-performance", Self::UltraPerformance),
    ];

    /// Setting name (`quality`, `ultra-performance`, ...).
    #[must_use]
    pub fn name(self) -> &'static str {
        Self::NAMES
            .iter()
            .find(|(_, q)| *q == self)
            .map_or("auto", |(n, _)| n)
    }
}

impl Upscaling {
    /// Parses a setting: `off`, or a quality name (`auto`, `native`/`dlaa`,
    /// `quality`, `balanced`, `performance`, `ultra-performance`) for DLSS.
    ///
    /// # Errors
    /// Unknown names; the message lists the known ones.
    pub fn parse(raw: &str) -> Result<Self, String> {
        let raw = raw.trim();
        if raw.eq_ignore_ascii_case("off") || raw.eq_ignore_ascii_case("none") {
            return Ok(Self::Off);
        }
        UpscaleQuality::NAMES
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(raw) || n.replace('-', "_").eq_ignore_ascii_case(raw))
            .map(|(_, q)| Self::Dlss(*q))
            .ok_or_else(|| {
                let names: Vec<&str> = UpscaleQuality::NAMES.iter().map(|(n, _)| *n).collect();
                format!("unknown upscaling {raw:?}; known: off, {}", names.join(", "))
            })
    }

    /// The setting that actually runs: DLSS only where it is supported.
    #[must_use]
    pub fn resolve(self, dlss_supported: bool) -> Self {
        match self {
            Self::Dlss(_) if dlss_supported => self,
            _ => Self::Off,
        }
    }

    /// `off` or the quality name.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Dlss(q) => q.name(),
        }
    }
}

/// Luminance (nits) of a uniformly emitting sphere of `radius` meters whose
/// total output is `lumens`: `Φ / (4π² r²)`. Seen from afar it has the
/// intensity `Φ / 4π` candela of a point light with the same flux, so an
/// emissive proxy of this luminance replaces a punctual light one-to-one.
pub fn sphere_light_luminance(lumens: f32, radius: f32) -> f32 {
    let r = radius.max(1e-4);
    lumens / (4.0 * std::f32::consts::PI * std::f32::consts::PI * r * r)
}
