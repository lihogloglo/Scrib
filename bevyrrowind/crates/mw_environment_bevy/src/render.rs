//! Render settings shared by interiors and exteriors: ray-traced lighting
//! (`--rt`, Bevy Solari) and temporal upscaling (`--dlss`, Bevy's DLSS).
//! The framework side is `sf_sky_bevy` (`RenderPathSetting`,
//! `UpscalingSetting`); spec `superfargoth/docs/specs/sky-lighting.md`.

use bevy_app::{App, Last, Update};
use bevy_camera::visibility::InheritedVisibility;
use bevy_camera::{Camera, Camera3d, Exposure};
use bevy_ecs::prelude::*;
use bevy_light::PointLight;
use bevy_log::info;
use bevy_post_process::auto_exposure::AutoExposure;
use bevy_render::view::screenshot::Screenshot;
use bevy_transform::components::GlobalTransform;
use sf_sky_bevy::sf_sky::{RenderPath, Upscaling};
use sf_sky_bevy::{PointShadowBudget, SkyRenderOptions, UpscalingSetting};

/// Default for `--point-shadows` (perf A/B: `--env BEVYRROWIND_POINT_SHADOWS=0`).
const ENV_POINT_SHADOWS: &str = "BEVYRROWIND_POINT_SHADOWS";

/// Default for `--sun-cascades` (perf A/B: `--env BEVYRROWIND_SUN_CASCADES=3:150`).
const ENV_SUN_CASCADES: &str = "BEVYRROWIND_SUN_CASCADES";

/// Point-light shadow budget of the quality presets without an override
/// (`(count, cube face size)`): 8 cached lamps at 512² using Bevy's native
/// per-face light-relative shadow cache (`bevy_pbr::skip_unchanged_shadow_views`)
/// with forward-hemisphere priority weighting in `sf_sky_bevy`.
pub const POINT_SHADOWS_DEFAULT: (usize, usize) = (8, 512);

/// Lamp shadows fade out between these viewer distances, m (`PointLightShadowFade`
/// in the PBR shader; 35–45 m covers Balmora street blocks without nearby pop-in).
const LAMP_SHADOW_FADE: (f32, f32) = (35.0, 45.0);

/// Legacy static/mover cubemap cache (`sf_shadow_cache_bevy`): off now that
/// vendored `bevy_pbr` caches shadow maps per cubemap face in light-relative
/// coordinates and skips `Prepare` / `Batch` / `Preprocess` for unchanged faces.
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
    /// Lamp shadow faces redraw the same frame their casters move or deform (SH-20;
    /// `--off movershadows`: the B3 grid + 2-faces-per-frame rule).
    pub mover_shadows: bool,
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
        Ok(Self { path, upscaling, denoiser, point_shadows, shadow_cache: true, mover_shadows: true, sun_cascades })
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
        app.insert_resource(bevy_anti_alias::dlss::DlssProjectId(bevy_asset::uuid::Uuid::from_u128(
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

    /// `--off shadowcache`: the patched Bevy path (superfargoth/vendor, PRs #26149/#26161): a
    /// shadowed light redraws all six faces whenever a caster it sees moves or deforms, and
    /// skips them otherwise (shadow reset B1, perf.md: slower than the cache with walkers).
    #[must_use]
    pub fn with_shadow_cache(mut self, on: bool) -> Self {
        self.shadow_cache = on;
        self
    }

    /// `--off movershadows` (SH-20 A/B): `bevy_pbr::ShadowMoverRefresh`.
    #[must_use]
    pub fn with_mover_shadows(mut self, on: bool) -> Self {
        self.mover_shadows = on;
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
        app.insert_resource(bevy_pbr::ShadowMoverRefresh { per_frame: self.mover_shadows });
        // Shadows F6 slice 2: static casters cached per light, movers drawn over the copy.
        if POINT_SHADOW_CACHE && self.shadow_cache {
            app.add_plugins(sf_shadow_cache_bevy::ShadowCachePlugin)
                .insert_resource(sf_shadow_cache_bevy::ShadowCacheSettings { enabled: true, ..Default::default() })
                .add_observer(movable_characters)
                .add_systems(Update, shadow_captures_at_black.before(sf_world_bevy::StreamingSystems::Transition));
        }

        app.add_systems(Last, log_lighting_at_screenshot);
    }

    /// After the sky plugin (it inits `SkyRenderOptions`): the denoiser switch.
    pub fn apply_options(self, options: &mut SkyRenderOptions) {
        options.ray_tracing_denoiser = self.denoiser;
    }
}

/// Shadow reset A2: lamp maps are captured during the door fade. While the
/// screen is black the cache warms up (unlimited captures, short settle), and
/// the fade stays black until every shadowed lamp in view has its final cached
/// faces (capped by `FadeTransition::gate_max_secs`), so the first visible
/// frames have no capture flicker. Removal path: slice B's patched `bevy_pbr`
/// replaces the cache.
/// The budget must have caught up first: the arrival's lamps take a slot over
/// two frames (twin, then the map), so until then the cache has nothing pending.
#[allow(clippy::type_complexity)]
fn shadow_captures_at_black(
    mut fade: ResMut<sf_world_bevy::FadeTransition>,
    mut settings: ResMut<sf_shadow_cache_bevy::ShadowCacheSettings>,
    stats: Res<sf_shadow_cache_bevy::ShadowCacheStats>,
    budget: Res<PointShadowBudget>,
    places: Query<&GlobalTransform>,
    lights: Query<(&GlobalTransform, &PointLight, &InheritedVisibility), With<sf_sky_bevy::BudgetedShadow>>,
) {
    let eye = match fade.phase() {
        sf_world_bevy::FadePhase::Hold { traveler, .. } => places.get(*traveler).ok().map(GlobalTransform::translation),
        _ => None,
    };
    let black = eye.is_some();
    if settings.warmup != black {
        settings.warmup = black;
    }
    let Some(eye) = eye else { return };
    let (mut candidates, mut shadowed) = (0, 0);
    for (at, light, shown) in &lights {
        if shown.get() && at.translation().distance(eye) < budget.fade_end {
            candidates += 1;
            shadowed += usize::from(light.shadow_maps_enabled);
        }
    }
    if !stats.settled() || shadowed < candidates.min(budget.max) {
        fade.hold();
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
    shots: Query<(), Added<Screenshot>>,
    cameras: Query<(&Camera, &Exposure, Has<AutoExposure>), With<Camera3d>>,
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
