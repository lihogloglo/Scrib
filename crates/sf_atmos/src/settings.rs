//! What drives the clouds: weather scalars, look constants, quality.

use glam::Vec2;

/// The weather-driven part of a cloud layer. Every field blends linearly
/// ([`CloudWeather::lerp`]), so weather transitions are smooth: only scalars
/// change, never the noise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloudWeather {
    /// 0 clear sky, 1 fully covered. A threshold on the weather map, so it
    /// opens and closes holes instead of thinning the whole deck.
    pub coverage: f32,
    /// Cloud type: 0 flat stratus, 1 piled cumulus (the height profile).
    pub cloud_type: f32,
    /// Multiplier on the cloud's extinction (optical thickness).
    pub density: f32,
    /// Rain-laden darkening, 0..1 (Nubis "wetness"): darker bases.
    pub wetness: f32,
    /// Storm cells, 0..1: the weather map's cellular channel raises
    /// coverage, type and wetness where it is high.
    pub storm: f32,
    /// How hard the detail noise erodes edges, 0..1.
    pub erosion: f32,
    /// Layer base, meters above sea level.
    pub base: f32,
    /// Layer thickness, meters.
    pub thickness: f32,
    /// Wind at cloud altitude, meters per second (world X, Z).
    pub wind: Vec2,
}

/// Named presets. The weather state machine (`sf_weather`, later) blends
/// between records instead; these are its seeds and the app's `--clouds`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CloudPreset {
    Clear,
    Fair,
    Overcast,
    Storm,
}

impl CloudPreset {
    pub const ALL: [(&'static str, CloudPreset); 5] = [
        ("clear", CloudPreset::Clear),
        ("fair", CloudPreset::Fair),
        ("cumulus", CloudPreset::Fair),
        ("overcast", CloudPreset::Overcast),
        ("storm", CloudPreset::Storm),
    ];

    /// Parses a preset name (`clear`, `fair`/`cumulus`, `overcast`, `storm`).
    pub fn parse(name: &str) -> Option<Self> {
        let name = name.trim().to_ascii_lowercase();
        Self::ALL.iter().find(|(n, _)| *n == name).map(|(_, p)| *p)
    }

    pub fn name(self) -> &'static str {
        Self::ALL.iter().find(|(_, p)| *p == self).map_or("?", |(n, _)| n)
    }
}

impl CloudWeather {
    /// A preset at real-world altitudes. Games with a compressed world
    /// rescale the heights ([`CloudWeather::scaled_heights`]).
    pub fn preset(preset: CloudPreset) -> Self {
        let wind = Vec2::new(9.0, 3.0);
        match preset {
            CloudPreset::Clear => Self {
                coverage: 0.12,
                cloud_type: 0.7,
                density: 0.8,
                wetness: 0.0,
                storm: 0.0,
                erosion: 0.45,
                base: 1_600.0,
                thickness: 900.0,
                wind,
            },
            CloudPreset::Fair => Self {
                coverage: 0.45,
                cloud_type: 0.85,
                density: 1.0,
                wetness: 0.05,
                storm: 0.0,
                erosion: 0.4,
                base: 1_500.0,
                thickness: 1_400.0,
                wind,
            },
            CloudPreset::Overcast => Self {
                coverage: 0.92,
                cloud_type: 0.2,
                density: 1.2,
                wetness: 0.35,
                storm: 0.0,
                erosion: 0.25,
                base: 1_100.0,
                thickness: 900.0,
                wind: wind * 1.5,
            },
            CloudPreset::Storm => Self {
                coverage: 0.85,
                cloud_type: 1.0,
                density: 1.6,
                wetness: 0.8,
                storm: 0.9,
                erosion: 0.3,
                base: 900.0,
                thickness: 3_200.0,
                wind: wind * 2.5,
            },
        }
    }

    /// The same weather with base and thickness scaled (a compressed world).
    #[must_use]
    pub fn scaled_heights(mut self, scale: f32) -> Self {
        self.base *= scale;
        self.thickness *= scale;
        self
    }

    /// Linear blend, `t` in 0..1 (clamped).
    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let l = |a: f32, b: f32| a + (b - a) * t;
        Self {
            coverage: l(self.coverage, other.coverage),
            cloud_type: l(self.cloud_type, other.cloud_type),
            density: l(self.density, other.density),
            wetness: l(self.wetness, other.wetness),
            storm: l(self.storm, other.storm),
            erosion: l(self.erosion, other.erosion),
            base: l(self.base, other.base),
            thickness: l(self.thickness, other.thickness),
            wind: self.wind.lerp(other.wind, t),
        }
    }

    /// Mean sky cover this weather produces (0..1), estimated from the
    /// coverage threshold. The cloud shadow map reads it past its range
    /// (Megalophobia's "out of range is lit", generalized: average, never black).
    pub fn mean_cover(&self) -> f32 {
        let c = self.coverage.max(self.storm * 0.6);
        (c * c * (3.0 - 2.0 * c)).clamp(0.0, 1.0)
    }
}

impl Default for CloudWeather {
    fn default() -> Self {
        Self::preset(CloudPreset::Fair)
    }
}

/// Lighting and scale constants: art direction, not weather.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloudLook {
    /// Extinction per meter at density 1 (a reciprocal length).
    pub extinction: f32,
    /// Sky fill multiplier (on the atmosphere's sky radiance).
    pub ambient: f32,
    /// Ground bounce multiplier (ground albedo × sun irradiance).
    pub ground_bounce: f32,
    /// Beer-powder strength, 0..1.
    pub powder: f32,
    /// First-octave Henyey-Greenstein g.
    pub phase_g: f32,
    /// Per-octave decay of extinction, contribution and eccentricity.
    pub ms_falloff: f32,
    /// Tile sizes in meters. Each must divide 16 384 m (the floating
    /// origin's phase period), so origin moves never shift the noise.
    pub weather_tile: f32,
    pub shape_tile: f32,
    pub detail_tile: f32,
    /// Ray length cap, meters. The deck fades over the last quarter.
    pub far: f32,
    /// How dark full cloud makes the ground (cloud shadow map), 0..1.
    pub shadow_strength: f32,
}

impl Default for CloudLook {
    fn default() -> Self {
        Self {
            extinction: 0.015,
            ambient: 1.0,
            ground_bounce: 1.0,
            powder: 0.7,
            phase_g: 0.6,
            ms_falloff: 0.5,
            weather_tile: 16_384.0,
            shape_tile: 4_096.0,
            detail_tile: 256.0,
            far: 50_000.0,
            shadow_strength: 0.75,
        }
    }
}

/// Cost knobs (weather.md budgets: Low/Medium/High/Ultra).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloudQuality {
    /// Primary samples per traced pixel: the adaptive count stays in this range.
    pub min_steps: u32,
    pub max_steps: u32,
    /// Samples toward the light per lit primary sample.
    pub light_steps: u32,
    /// Multiple-scattering octaves (1 = single scattering, black clouds).
    pub ms_octaves: u32,
    /// Trace one pixel of each 2×2 block per frame at half resolution
    /// (1/16 of the screen traced per frame, temporally accumulated);
    /// `false` traces every half-resolution pixel each frame (Ultra).
    pub checkerboard: bool,
    /// Cloud shadow map size (texels a side) and samples per texel.
    pub shadow_size: u32,
    pub shadow_steps: u32,
}

impl Default for CloudQuality {
    fn default() -> Self {
        Self::HIGH
    }
}

impl CloudQuality {
    pub const HIGH: Self = Self {
        min_steps: 32,
        max_steps: 80,
        light_steps: 5,
        ms_octaves: 3,
        checkerboard: true,
        shadow_size: 512,
        shadow_steps: 8,
    };
    pub const ULTRA: Self = Self { max_steps: 96, light_steps: 6, checkerboard: false, ..Self::HIGH };
}

/// Everything the cloud renderer reads, per frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CloudSettings {
    pub weather: CloudWeather,
    pub look: CloudLook,
    pub quality: CloudQuality,
}

impl CloudSettings {
    pub fn preset(preset: CloudPreset) -> Self {
        Self { weather: CloudWeather::preset(preset), ..Self::default() }
    }
}
