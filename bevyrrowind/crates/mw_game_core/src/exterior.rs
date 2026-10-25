//! The exterior's shared pieces (`apps/bevyrrowind/src/exterior.rs` holds the systems and the
//! quality presets' render settings, `QualityRender`).

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use sf_atmos::{CloudPreset, CloudWeather};

/// Consecutive frames the capture gate must see CPU settled and no pipeline compiling.
pub const CAPTURE_STABLE_FRAMES: u32 = 10;
/// Seconds after start before a capture gate gives up.
pub const CAPTURE_TIMEOUT_SECS: f32 = 120.0;

/// Inserted once the start has fully streamed in (`--use-door` and
/// `BEVYRROWIND_AUTO` wait for it; `--use-door` removes it for each trip).
#[derive(Resource)]
pub struct Settled;

/// The streaming camera (visual anchor, floating origin).
#[derive(Component)]
pub struct ExteriorCamera;

/// Morrowind's world is compressed, so cloud altitudes **and sizes** are
/// halved together (weather.md open question 8; `mw_weather::visual`).
pub const CLOUD_WORLD_SCALE: f32 = mw_weather::visual::CLOUD_WORLD_SCALE;

/// A genus pinned over the weather's clouds (`--clouds`, BRP `clouds`), at
/// Morrowind's halved altitudes; its wind stays the weather's.
pub fn pinned_clouds(genus: CloudPreset) -> CloudWeather {
    CloudWeather::preset(genus).scaled_heights(CLOUD_WORLD_SCALE)
}

/// Quality presets (industry-audit.md J1, ROADMAP Next up 4): one view-distance
/// `bias` scales LOD selection, size culling and the far-field handoff `d1`
/// (the baked 400 m × bias, so the rings follow), plus the size-culling cutoff
/// in reference pixels and its floor. High matches the small-feature cutoff of `OpenMW`
/// (2 px), Ultra our 1 px LOD budget. The render settings per preset live with the
/// renderer (the app's `exterior::QualityRender`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    Low,
    Medium,
    #[default]
    High,
    Ultra,
}

impl Quality {
    pub const ENV: &'static str = "BEVYRROWIND_QUALITY";

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "ultra" => Some(Self::Ultra),
            _ => None,
        }
    }

    /// `--quality`, else `BEVYRROWIND_QUALITY`, else High.
    pub fn resolve(flag: Option<Self>) -> Self {
        flag.or_else(|| std::env::var(Self::ENV).ok().and_then(|v| Self::parse(&v))).unwrap_or_default()
    }

    /// Whether the clouds read the world weather map's layer per density
    /// sample (High/Ultra) or per ray, 3 fetches lerped (Medium/Low;
    /// `sf_atmos::CloudQuality::map_per_sample`, weather.md, distant weather as built).
    pub const fn cloud_map_per_sample(self) -> bool {
        matches!(self, Self::High | Self::Ultra)
    }
}
