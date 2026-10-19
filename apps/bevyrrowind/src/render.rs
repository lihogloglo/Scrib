//! Render settings shared by interiors and exteriors: ray-traced lighting
//! (`--rt`, Bevy Solari) and temporal upscaling (`--dlss`, Bevy's DLSS).
//! The framework side is `sf_sky_bevy` (`RenderPathSetting`,
//! `UpscalingSetting`); spec `superfargoth/docs/specs/sky-lighting.md`.

use bevy::prelude::*;
use sf_sky_bevy::sf_sky::{RenderPath, Upscaling};
use sf_sky_bevy::{PointShadowBudget, SkyRenderOptions, UpscalingSetting};

/// Default for `--point-shadows` (perf A/B: `--env BEVYRROWIND_POINT_SHADOWS=0`).
const ENV_POINT_SHADOWS: &str = "BEVYRROWIND_POINT_SHADOWS";

/// Point-light shadow budget of the quality presets without an override
/// (`(count, cube face size)`; lighting job 2026-10-13, sky-lighting.md
/// "Local-light shadows": GPU cost at `hi-balmora-night`).
pub const POINT_SHADOWS_DEFAULT: (usize, usize) = (1, 1024);

/// Point-light shadow cube cache (shadows F6 slice 2, `sf_shadow_cache_bevy`):
/// parked by owner decision 3 (2026-10-19, simpler shadows first); off, its
/// plugin isn't even added (its caster tracking cost ~0.5 ms a frame in
/// Balmora, `docs/perf.md` "Frame budget"). Resume needs the owner's go.
const POINT_SHADOW_CACHE: bool = false;

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
        Ok(Self { path, upscaling, denoiser, point_shadows, shadow_cache: true })
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

    /// `--off shadowcache`: Bevy redraws every shadowed light every frame.
    #[must_use]
    pub fn with_shadow_cache(mut self, on: bool) -> Self {
        self.shadow_cache = on;
        self
    }

    /// The point-light shadow budget: the override's count, else the
    /// preset's `(count, face size)`.
    pub fn point_shadow_budget(self, preset: (usize, usize)) -> PointShadowBudget {
        PointShadowBudget { max: self.point_shadows.unwrap_or(preset.0), map_size: preset.1, ..PointShadowBudget::default() }
    }

    /// Before the sky plugin: its settings resources.
    pub fn insert(self, app: &mut App) {
        app.insert_resource(UpscalingSetting(self.upscaling));
        app.insert_resource(self.point_shadow_budget(POINT_SHADOWS_DEFAULT));
        // Shadows F6 slice 2: static casters cached per light, movers drawn over the copy.
        if POINT_SHADOW_CACHE && self.shadow_cache {
            app.add_plugins(sf_shadow_cache_bevy::ShadowCachePlugin)
                .insert_resource(sf_shadow_cache_bevy::ShadowCacheSettings { enabled: true, ..Default::default() });
        }

        app.add_systems(Last, log_lighting_at_screenshot);
    }

    /// After the sky plugin (it inits `SkyRenderOptions`): the denoiser switch.
    pub fn apply_options(self, options: &mut SkyRenderOptions) {
        options.ray_tracing_denoiser = self.denoiser;
    }
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
