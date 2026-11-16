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

/// Quality presets (industry-audit.md J1, ROADMAP Next up 4): the level the run starts at, a level of
/// the overall graphics preset `sf.graphics_preset` (tunables slice T5b). Every value a level decides
/// (view distance, shadows, lights, post, sky, water, surface, textures, GI) is a tunable its `sg.*`
/// group writes at the `Preset` layer from the builtin mod's records (`mw_mods::BASE_RECORDS`); the
/// level switches live (the options menu's Graphics page, console `set sf.graphics_preset <level>`, `preset`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    /// The floor for weak machines and CPU/GPU comparisons, below Low.
    ExtremelyLow,
    Low,
    Medium,
    #[default]
    High,
    Ultra,
}

impl Quality {
    pub const ENV: &'static str = "BEVYRROWIND_QUALITY";
    /// Every level, weakest first.
    pub const ALL: [Self; 5] = [Self::ExtremelyLow, Self::Low, Self::Medium, Self::High, Self::Ultra];

    /// `extremely_low` (also `extremely-low`, `extremelylow`), `low`, `medium`, `high`, `ultra`.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "extremely_low" | "extremelylow" => Some(Self::ExtremelyLow),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "ultra" => Some(Self::Ultra),
            _ => None,
        }
    }

    /// The level id of `sf.graphics_preset` (`extremely_low`, ..., `ultra`).
    pub const fn name(self) -> &'static str {
        match self {
            Self::ExtremelyLow => "extremely_low",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Ultra => "ultra",
        }
    }

    /// `--quality`, else `BEVYRROWIND_QUALITY`, else High.
    pub fn resolve(flag: Option<Self>) -> Self {
        flag.or_else(|| std::env::var(Self::ENV).ok().and_then(|v| Self::parse(&v))).unwrap_or_default()
    }
}

#[cfg(test)]
mod quality_tests {
    use super::Quality;

    /// `--quality` / `BEVYRROWIND_QUALITY` take the five levels, `extremely-low` spelled either way.
    #[test]
    fn quality_parses_every_level() {
        for spelling in ["extremely-low", "extremely_low", "Extremely-Low", " extremelylow "] {
            assert_eq!(Quality::parse(spelling), Some(Quality::ExtremelyLow), "{spelling}");
        }
        for q in Quality::ALL {
            assert_eq!(Quality::parse(q.name()), Some(q));
        }
        assert_eq!(Quality::parse("epic"), None);
    }
}
