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
    mw_weather::visual::mw_clouds(genus)
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

    /// Load-time texture size cap, largest side of level 0 in texels (`sf_texture_bevy::TextureCapPlugin`,
    /// textures.md §6 "Pool per preset"): Low 512, Medium 1024, High 2048, Ultra none. Absolute, like UE's
    /// texture-group `MaxLODSize`; vanilla textures (256-512) are never capped, HD packs' 2K are from Medium down.
    pub const fn texture_cap(self) -> Option<u32> {
        match self {
            Self::Low => Some(512),
            Self::Medium => Some(1024),
            Self::High => Some(2048),
            Self::Ultra => None,
        }
    }

    /// Parallax occlusion on materials with a depth companion (textures.md §9 "T4d", Bevy's
    /// `ParallaxMappingMethod`): Low none (the depth map isn't bound), Medium steep parallax with a
    /// one-sample interpolation (Bevy's `Occlusion`, its default 16 layers), High the same 16 layers
    /// refined by 5 binary-search steps (`Relief`, Bevy's `DEFAULT_RELIEF_MAPPING`), Ultra 32 layers
    /// and 8 steps (UE's POM node reference count at glancing angles). Layers scale with the view:
    /// 1 looking straight on, the maximum at grazing angles (`parallax_mapping.wesl`).
    pub const fn parallax(self) -> Option<Parallax> {
        match self {
            Self::Low => None,
            Self::Medium => Some(Parallax { max_layers: 16, relief_steps: None }),
            Self::High => Some(Parallax { max_layers: 16, relief_steps: Some(5) }),
            Self::Ultra => Some(Parallax { max_layers: 32, relief_steps: Some(8) }),
        }
    }
}

/// A preset's parallax occlusion settings ([`Quality::parallax`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parallax {
    /// Most depth layers the ray march steps through (at grazing angles).
    pub max_layers: u8,
    /// Binary-search refinement steps after the march (relief mapping); `None`: one interpolation
    /// between the last two layers (parallax occlusion mapping).
    pub relief_steps: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// tx-pom: Low binds no depth map, Medium marches with one interpolation, High and Ultra
    /// refine by binary search; layers never shrink up the presets.
    #[test]
    fn preset_picks_the_parallax_method() {
        assert_eq!(Quality::Low.parallax(), None);
        let medium = Quality::Medium.parallax().expect("medium");
        assert_eq!(medium, Parallax { max_layers: 16, relief_steps: None });
        assert_eq!(Quality::High.parallax(), Some(Parallax { max_layers: 16, relief_steps: Some(5) }));
        assert_eq!(Quality::Ultra.parallax(), Some(Parallax { max_layers: 32, relief_steps: Some(8) }));
        assert_eq!(Quality::default().parallax(), Quality::High.parallax());
    }
}
