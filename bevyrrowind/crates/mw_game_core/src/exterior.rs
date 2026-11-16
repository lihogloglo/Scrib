//! The exterior's shared pieces (`apps/bevyrrowind/src/exterior.rs` holds the systems and the
//! quality presets' values, `quality_presets`).

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
    mw_weather::visual::mw_clouds(genus)
}

/// Quality presets (industry-audit.md J1, ROADMAP Next up 4): the level the run starts at. Every value
/// a level decides (view distance, shadows, lights, post, sky, water, surface, textures, GI) is a
/// tunable written at the `Preset` layer from one table (`mw_exterior_bevy::quality_presets`,
/// tunables slices T4 and T5a).
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
}
