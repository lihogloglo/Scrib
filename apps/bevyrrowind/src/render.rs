//! Render settings shared by interiors and exteriors: ray-traced lighting
//! (`--rt`, Bevy Solari) and temporal upscaling (`--dlss`, Bevy's DLSS).
//! The framework side is `sf_sky_bevy` (`RenderPathSetting`,
//! `UpscalingSetting`); spec `superfargoth/docs/specs/sky-lighting.md`.

use bevy::prelude::*;
use sf_sky_bevy::sf_sky::{RenderPath, Upscaling};
use sf_sky_bevy::{PointShadowBudget, SkyRenderOptions, UpscalingSetting};

/// Default for `--point-shadows` (perf A/B: `--env BEVYRROWIND_POINT_SHADOWS=0`).
const ENV_POINT_SHADOWS: &str = "BEVYRROWIND_POINT_SHADOWS";

/// Default for `--sun-cascades` (perf A/B: `--env BEVYRROWIND_SUN_CASCADES=3:150`).
const ENV_SUN_CASCADES: &str = "BEVYRROWIND_SUN_CASCADES";

/// Point-light shadow budget of the quality presets without an override
/// (`(count, cube face size)`): 8 cached lamps at 512² since shadows v2 V1
/// (shadows2 2026-10-20; superfargoth `docs/specs/shadows.md`, perf.md "Shadows v2").
pub const POINT_SHADOWS_DEFAULT: (usize, usize) = (8, 512);

/// Lamp shadows fade out between these viewer distances, m (was 15-25 with one
/// uncached lamp: shadows "only when super close", SH-13).
const LAMP_SHADOW_FADE: (f32, f32) = (30.0, 40.0);

/// Point-light shadow cube cache (`sf_shadow_cache_bevy`, shadows.md v2 V1):
/// on since shadows2 (2026-10-20); `--off shadowcache` is the A/B (Bevy redraws
/// every face every frame). Removal path: delete the switch once V1 is signed off.
const POINT_SHADOW_CACHE: bool = true;

/// Default for `--dlss` when the flag isn't given (perf suite: `--env BEVYRROWIND_DLSS=quality`).
const ENV_DLSS: &str = "BEVYRROWIND_DLSS";

/// Upscaling without `--dlss` or `BEVYRROWIND_DLSS`: a temporal resolve on
/// Medium and up (ROADMAP 1.3): DLSS `auto` (Quality at our resolutions) in
/// builds with DLSS, which falls back to TAA on GPUs without it; TAA in
/// builds without. The Low preset keeps MSAA. `--dlss taa|off` picks.
pub fn default_upscaling(low: bool) -> Upscaling {
    match (low, cfg!(feature = "dlss")) {
        (true, _) => Upscaling::Off,
        (false, true) => Upscaling::parse("auto").unwrap_or(Upscaling::Taa),
        (false, false) => Upscaling::Taa,
    }
}

/// The render options of a view.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderSettings {
    pub path: RenderPath,
    pub upscaling: Upscaling,
    /// DLSS Ray Reconstruction on the ray-traced path (`--off denoiser` turns it off).
    pub denoiser: bool,
    /// Point lights with shadow maps (`--point-shadows`, else
    /// `BEVYRROWIND_POINT_SHADOWS`, else the quality preset's).
    pub point_shadows: Option<usize>,
    /// Cached point-light shadow maps (`--off shadowcache` turns them off).
    pub shadow_cache: bool,
    /// Sun cascades `(count, last bound m)` instead of the preset's
    /// (`--sun-cascades`, else `BEVYRROWIND_SUN_CASCADES`).
    pub sun_cascades: Option<(usize, f32)>,
}

/// `N:M`: `N` sun cascades out to `M` meters.
pub fn parse_sun_cascades(s: &str) -> Result<(usize, f32), String> {
    let (n, m) = s.trim().split_once(':').ok_or_else(|| format!("sun cascades {s:?}: expected COUNT:METERS, e.g. 3:150"))?;
    let n: usize = n.parse().map_err(|_| format!("sun cascades {s:?}: bad count"))?;
    let m: f32 = m.parse().map_err(|_| format!("sun cascades {s:?}: bad distance"))?;
    if !(1..=4).contains(&n) || m <= 1.0 {
        return Err(format!("sun cascades {s:?}: 1-4 cascades, distance > 1 m"));
    }
    Ok((n, m))
}

impl RenderSettings {
    /// From the flags; `--dlss` falls back to `BEVYRROWIND_DLSS`, then to
    /// [`default_upscaling`] (`low`: the Low quality preset).
    pub fn new(ray_traced: bool, dlss: Option<Upscaling>, denoiser: bool, low: bool) -> Result<Self, String> {
        let upscaling = match dlss {
            Some(u) => u,
            None => match std::env::var(ENV_DLSS) {
                Ok(raw) if !raw.trim().is_empty() => Upscaling::parse(&raw)?,
                _ => default_upscaling(low),
            },
        };
        if ray_traced && !cfg!(feature = "solari") {
            eprintln!("--rt needs a build with `--features solari`; rendering raster");
        }
        if matches!(upscaling, Upscaling::Dlss(_)) && !cfg!(feature = "dlss") {
            eprintln!("--dlss needs a build with `--features dlss`; TAA instead");
        }
        let path = if ray_traced { RenderPath::RayTraced } else { RenderPath::Raster };
        let point_shadows = std::env::var(ENV_POINT_SHADOWS).ok().and_then(|v| v.trim().parse().ok());
        let sun_cascades = std::env::var(ENV_SUN_CASCADES).ok().and_then(|v| parse_sun_cascades(&v).ok());
        Ok(Self { path, upscaling, denoiser, point_shadows, shadow_cache: true, sun_cascades })
    }

    /// Whether the view needs the sky plugin's render-path management.
    pub fn managed(self) -> bool {
        self.path == RenderPath::RayTraced || self.upscaling != Upscaling::Off
    }

    /// For reports: `raster`, `rt`, plus the upscaling (`raster dlss=quality`).
    pub fn label(self) -> String {
        let path = match self.path {
            RenderPath::Raster => "raster",
            RenderPath::RayTraced if self.denoiser => "rt",
            RenderPath::RayTraced => "rt nodenoiser",
        };
        match self.upscaling {
            Upscaling::Off => path.to_owned(),
            Upscaling::Taa => format!("{path} taa"),
            u @ Upscaling::Dlss(_) => format!("{path} dlss={}", u.label()),
        }
    }

    /// Before `DefaultPlugins`: DLSS identifies the application to NVIDIA's
    /// runtime (Bevy's DLSS init plugin reads it while creating the device).
    pub fn init(app: &mut App) {
        #[cfg(feature = "dlss")]
        app.insert_resource(bevy::anti_alias::dlss::DlssProjectId(bevy::asset::uuid::Uuid::from_u128(
            0x6b65_7272_6f77_696e_642d_6276_7972_726f,
        )));
        #[cfg(not(feature = "dlss"))]
        let _ = app;
    }

    /// `--point-shadows` from the command line (wins over the environment).
    #[must_use]
    pub fn with_point_shadows(mut self, count: Option<usize>) -> Self {
        self.point_shadows = count.or(self.point_shadows);
        self
    }

    /// `--sun-cascades` from the command line (wins over the environment).
    #[must_use]
    pub fn with_sun_cascades(mut self, cascades: Option<(usize, f32)>) -> Self {
        self.sun_cascades = cascades.or(self.sun_cascades);
        self
    }

    /// `--off shadowcache`: Bevy redraws every shadowed light every frame.
    #[must_use]
    pub fn with_shadow_cache(mut self, on: bool) -> Self {
        self.shadow_cache = on;
        self
    }

    /// The point-light shadow budget: the override's count, else the
    /// preset's `(count, face size)`.
    pub fn point_shadow_budget(self, preset: (usize, usize)) -> PointShadowBudget {
        let (fade_start, fade_end) = LAMP_SHADOW_FADE;
        PointShadowBudget { max: self.point_shadows.unwrap_or(preset.0), map_size: preset.1, fade_start, fade_end, ..PointShadowBudget::default() }
    }

    /// Before the sky plugin: its settings resources.
    pub fn insert(self, app: &mut App) {
        app.insert_resource(UpscalingSetting(self.upscaling));
        app.insert_resource(self.point_shadow_budget(POINT_SHADOWS_DEFAULT));
        // Shadows F6 slice 2: static casters cached per light, movers drawn over the copy.
        if POINT_SHADOW_CACHE && self.shadow_cache {
            app.add_plugins(sf_shadow_cache_bevy::ShadowCachePlugin)
                .insert_resource(sf_shadow_cache_bevy::ShadowCacheSettings { enabled: true, ..Default::default() })
                .add_observer(movable_characters);
        }

        app.add_systems(Last, log_lighting_at_screenshot);
    }

    /// After the sky plugin (it inits `SkyRenderOptions`): the denoiser switch.
    pub fn apply_options(self, options: &mut SkyRenderOptions) {
        options.ray_tracing_denoiser = self.denoiser;
    }
}

/// Characters (and what hangs off them: held items, attachments) are movers
/// of the lamp shadow cache: never baked, so an NPC that stops and starts
/// doesn't recapture the lights around it (a 1-frame loss of actor shadows).
fn movable_characters(add: On<Add<sf_character_bevy::Character>>, mut commands: Commands) {
    commands.entity(add.entity).try_insert(sf_shadow_cache_bevy::ShadowMobility::Movable);
}

/// With every screenshot: the exposure and local-light state it was taken
/// at (lighting job logs: EV100 per camera, the sky's, shadowed lights).
fn log_lighting_at_screenshot(
    shots: Query<(), Added<bevy::render::view::screenshot::Screenshot>>,
    cameras: Query<(&Camera, &bevy::camera::Exposure, Has<bevy::post_process::auto_exposure::AutoExposure>), With<Camera3d>>,
    sky: Option<Res<sf_sky_bevy::CurrentSky>>,
    lights: Query<(&PointLight, &InheritedVisibility)>,
    budget: Option<Res<PointShadowBudget>>,
) {
    if shots.is_empty() {
        return;
    }
    let cams: Vec<String> = cameras
        .iter()
        .filter(|(c, _, _)| c.is_active)
        .map(|(c, e, auto)| format!("order {} EV100 {:.2}{}", c.order, e.ev100, if auto { " + auto exposure" } else { "" }))
        .collect();
    let lit = lights.iter().filter(|(_, v)| v.get()).count();
    let shadowed = lights.iter().filter(|(l, v)| v.get() && l.shadow_maps_enabled).count();
    let sky = sky.map_or_else(String::new, |s| format!(", sky EV100 {:.2} (sun {:.1} deg)", s.0.ev100, s.0.celestial.sun.elevation_deg()));
    info!(
        "lighting at screenshot: cameras [{}]{sky}; point lights {lit} shown, {shadowed} shadowed (budget {})",
        cams.join("; "),
        budget.map_or(0, |b| b.max)
    );
}
