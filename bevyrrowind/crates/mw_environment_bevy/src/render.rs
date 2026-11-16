//! Render settings shared by interiors and exteriors: ray-traced lighting
//! (`--rt`, Bevy Solari) and temporal upscaling (`--dlss`, Bevy's DLSS).
//! The framework side is `sf_sky_bevy` (`RenderPathSetting`,
//! `UpscalingSetting`); spec `superfargoth/docs/specs/sky-lighting.md`.

use bevy_app::{App, Last, PostUpdate, Update};
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

/// Default for `--sun-cascades` (perf A/B: `--env BEVYRROWIND_SUN_CASCADES=3:150`).
const ENV_SUN_CASCADES: &str = "BEVYRROWIND_SUN_CASCADES";

/// Point-light shadow budget of the quality presets without an override
/// (`(count, cube face size)`): 8 cached lamps at 512² using Bevy's native
/// per-face light-relative shadow cache (`bevy_pbr::skip_unchanged_shadow_views`)
/// and the 360° importance pick with off-screen holders in `sf_sky_bevy`.
pub const POINT_SHADOWS_DEFAULT: (usize, usize) = (8, 512);

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
#[expect(clippy::struct_excessive_bools, reason = "independent --off A/B switches")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderSettings {
    pub path: RenderPath,
    pub upscaling: Upscaling,
    /// DLSS Ray Reconstruction on the ray-traced path (`--off denoiser` turns it off).
    pub denoiser: bool,
    /// Point lights with shadow maps (`--point-shadows`): the `CommandLine` layer of
    /// `shadow.lamp.count` (alias `BEVYRROWIND_POINT_SHADOWS`, set at registration).
    pub point_shadows: Option<usize>,
    /// Cached point-light shadow maps (`--off shadowcache` turns them off).
    pub shadow_cache: bool,
    /// Lamp shadow faces redraw the same frame their casters move or deform (SH-20;
    /// `--off movershadows`: the B3 grid + 2-faces-per-frame rule).
    pub mover_shadows: bool,
    /// One command encoder per sun cascade (ROADMAP 1.22 F1; `--off cascadeencoders`: one for all,
    /// stock Bevy).
    pub cascade_encoders: bool,
    /// Far sun cascades keep their fit and map between redraws (ROADMAP 1.22 F3,
    /// `sf_sky_bevy::SunCascadeHold`; `--off cascadehold`: every cascade drawn every frame).
    pub cascade_hold: bool,
    /// Sun cascades `(count, last bound m)` (`--sun-cascades`, else `BEVYRROWIND_SUN_CASCADES`): the
    /// `CommandLine` layer of `shadow.sun.cascades` and `shadow.sun.distance_m`.
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
        let sun_cascades = std::env::var(ENV_SUN_CASCADES).ok().and_then(|v| parse_sun_cascades(&v).ok());
        Ok(Self {
            path,
            upscaling,
            denoiser,
            point_shadows: None,
            shadow_cache: true,
            mover_shadows: true,
            cascade_encoders: true,
            cascade_hold: true,
            sun_cascades,
        })
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

    /// `--off cascadeencoders` (ROADMAP 1.22 F1 A/B): `bevy_pbr::ShadowCascadeEncoders`.
    #[must_use]
    pub fn with_cascade_encoders(mut self, on: bool) -> Self {
        self.cascade_encoders = on;
        self
    }

    /// `--off cascadehold` (ROADMAP 1.22 F3 A/B): `sf_sky_bevy::SunCascadeHold`.
    #[must_use]
    pub fn with_cascade_hold(mut self, on: bool) -> Self {
        self.cascade_hold = on;
        self
    }

    /// The point-light shadow budget at its starting values: the override's count, else the
    /// preset's `(count, face size)`, Bevyrrowind's fade band, the framework's other defaults.
    /// In apps with [`Self::insert`] the tunables then drive it (`sf_sky_bevy::ShadowQualityPlugin`).
    pub fn point_shadow_budget(self, preset: (usize, usize)) -> PointShadowBudget {
        let [fade_start, fade_end] = crate::shadow_tunables::LAMP_FADE_M;
        PointShadowBudget { max: self.point_shadows.unwrap_or(preset.0), map_size: preset.1, fade_start, fade_end, ..PointShadowBudget::default() }
    }

    /// Before the sky plugin: its settings resources, and the shadow and light tunables
    /// (`shadow_tunables`: alias copies first, then `sf_sky_bevy::ShadowQualityPlugin` binds them;
    /// the lamp budget, face culling and the sun gate follow them live).
    pub fn insert(self, app: &mut App) {
        crate::shadow_tunables::register(app, POINT_SHADOWS_DEFAULT, self.point_shadows, self.sun_cascades);
        if !app.is_plugin_added::<sf_sky_bevy::ShadowQualityPlugin>() {
            app.add_plugins(sf_sky_bevy::ShadowQualityPlugin);
        }
        app.insert_resource(UpscalingSetting(self.upscaling));
        app.insert_resource(self.point_shadow_budget(POINT_SHADOWS_DEFAULT));
        app.insert_resource(bevy_pbr::ShadowMoverRefresh { per_frame: self.mover_shadows });
        // CW-3b: `shadow.lamp.face_cull` and its margin (tunables).
        app.insert_resource(bevy_pbr::PointShadowFaceCull::default());
        // CW-8: `shadow.sun.gate` and its thresholds (tunables, `sf_sky::sun_gate`).
        app.insert_resource(sf_sky_bevy::SunShadowGating::default());
        app.insert_resource(bevy_pbr::ShadowCascadeEncoders(self.cascade_encoders));
        app.insert_resource(sf_sky_bevy::SunCascadeHold(sf_sky_bevy::sf_sky::cascade_hold::CascadeHoldConfig {
            enabled: self.cascade_hold,
            ..Default::default()
        }));
        // N1 (night sheen): the flat ambient (the exterior night floor, the interior cell
        // ambient) lights diffuse only, as `OpenMW`'s; env maps and lamps keep their specular.
        // Live A/B: `flip ambient_specular`.
        app.insert_resource(bevy_pbr::AmbientLightSpecular(false));
        // Shadows F6 slice 2: static casters cached per light, movers drawn over the copy.
        if POINT_SHADOW_CACHE && self.shadow_cache {
            app.add_plugins(sf_shadow_cache_bevy::ShadowCachePlugin)
                .insert_resource(sf_shadow_cache_bevy::ShadowCacheSettings { enabled: true, ..Default::default() })
                .add_observer(movable_characters)
                .add_systems(Update, shadow_captures_at_black.before(sf_world_bevy::StreamingSystems::Transition));
        }

        app.add_systems(Last, log_lighting_at_screenshot);
        app.add_systems(PostUpdate, publish_sun_occlusion.after(sf_atmos_bevy::CloudSystems::Drive));
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

/// CW-8: the cloud shadow map's mean on the sun (`sf_atmos_bevy::mean_cloud_shadow`; 1 while the
/// sun carries no light texture) for the sun/moon shadow gate (`sf_sky_bevy::SunLightOcclusion`):
/// the map dims the sun's direct light but not the sky's fill. Glue here: the two framework crates
/// don't depend on each other.
#[allow(clippy::needless_pass_by_value)]
fn publish_sun_occlusion(
    clouds: Option<Res<sf_atmos_bevy::Clouds>>,
    suns: Query<Has<bevy_light::DirectionalLightTexture>, With<sf_sky_bevy::SunLight>>,
    occlusion: Option<ResMut<sf_sky_bevy::SunLightOcclusion>>,
) {
    let (Some(clouds), Some(mut occlusion)) = (clouds, occlusion) else { return };
    let textured = suns.iter().any(|t| t);
    let s = &clouds.settings;
    let mean = if textured { sf_atmos_bevy::mean_cloud_shadow(s.weather.mean_cover(), s.look.shadow_strength) } else { 1.0 };
    occlusion.set_if_neq(sf_sky_bevy::SunLightOcclusion(mean));
}

/// With every screenshot: the exposure and local-light state it was taken
/// at (lighting job logs: EV100 per camera, the sky's, shadowed lights).
fn log_lighting_at_screenshot(
    shots: Query<(), Added<Screenshot>>,
    cameras: Query<(&Camera, &Exposure, Has<AutoExposure>), With<Camera3d>>,
    sky: Option<Res<sf_sky_bevy::CurrentSky>>,
    lights: Query<(&PointLight, &InheritedVisibility)>,
    budget: Option<Res<PointShadowBudget>>,
    gate: Option<Res<sf_sky_bevy::SunShadowGateStats>>,
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
    // CW-8: the sun/moon shadow gate (zero while off).
    let sky = match gate.filter(|g| **g != sf_sky_bevy::SunShadowGateStats::default()) {
        Some(g) => format!("{sky}, sun shadow contrast {:.3} strength {:.2}, moon {:.3} / {:.2}", g.sun_contrast, g.sun_strength, g.moon_contrast, g.moon_strength),
        None => sky,
    };
    info!(
        "lighting at screenshot: cameras [{}]{sky}; point lights {lit} shown, {shadowed} shadowed (budget {})",
        cams.join("; "),
        budget.map_or(0, |b| b.max)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// N1: the game's render settings turn the flat ambient's specular off (the vendored
    /// `bevy_pbr` view flag `AMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT` follows this resource).
    #[test]
    fn render_settings_make_the_flat_ambient_diffuse_only() {
        let mut app = App::new();
        let settings = RenderSettings::new(false, Some(Upscaling::Off), true, true).expect("settings").with_shadow_cache(false);
        settings.insert(&mut app);
        assert_eq!(app.world().get_resource::<bevy_pbr::AmbientLightSpecular>().map(|a| a.0), Some(false));
    }

    /// F1 (ROADMAP 1.22): each sun cascade records into its own encoder unless
    /// `--off cascadeencoders`; the vendored `bevy_pbr` per-view shadow pass systems split by
    /// `per_view_shadow_encoder_slot`.
    #[test]
    fn cascade_encoders_switch_and_split() {
        for on in [true, false] {
            let mut app = App::new();
            let settings = RenderSettings::new(false, Some(Upscaling::Off), true, true)
                .expect("settings")
                .with_shadow_cache(false)
                .with_cascade_encoders(on);
            settings.insert(&mut app);
            assert_eq!(app.world().get_resource::<bevy_pbr::ShadowCascadeEncoders>().map(|e| e.0), Some(on));
        }
        assert!(bevy_pbr::ShadowCascadeEncoders::default().0, "stock default splits");
        // Split: the sun's 3 cascades take systems 0, 1, 2; a second light's wrap around.
        let split: Vec<usize> = (0..6).map(|i| bevy_pbr::per_view_shadow_encoder_slot(i, true)).collect();
        assert_eq!(split, [0, 1, 2, 3, 0, 1]);
        assert!(split.iter().all(|&s| s < bevy_pbr::PER_VIEW_SHADOW_ENCODERS));
        // Off: every cascade in system 0's one encoder, as stock Bevy.
        assert!((0..8).all(|i| bevy_pbr::per_view_shadow_encoder_slot(i, false) == 0));
    }

    /// A render-settings app with the tunables plugin and a fixed environment (the aliases).
    fn tunable_app(env: &[(&str, &str)]) -> App {
        let mut app = App::new();
        app.add_plugins(sf_tunables_bevy::TunablesPlugin).insert_resource(sf_tunables_bevy::TunableEnv::from_pairs(env));
        RenderSettings::new(false, Some(Upscaling::Off), true, true).expect("settings").with_shadow_cache(false).insert(&mut app);
        app.update();
        app
    }

    fn console_set(app: &mut App, name: &str, value: &str) {
        app.world_mut().write_message(sf_tunables_bevy::SetTunable::new(name, value, sf_tunables_bevy::SetBy::Console));
        app.update();
    }

    /// CW gates (ROADMAP 1.23b, 1.25 CW-3b, CW-8) are live tunables: off by default, on through the old
    /// env switch (alias), and a console set flips each one both ways, reaching its consumer next frame.
    #[test]
    fn cw_shadow_gates_are_live_tunables() {
        use sf_sky_bevy::sf_sky::sun_gate::SunShadowGate;
        let read = |app: &App| {
            let w = app.world();
            (
                w.resource::<PointShadowBudget>().contrast_gate.enabled,
                w.resource::<bevy_pbr::PointShadowFaceCull>().enabled,
                w.resource::<sf_sky_bevy::SunShadowGating>().0.enabled,
            )
        };
        let mut app = tunable_app(&[]);
        assert_eq!(read(&app), (false, false, false), "off by default");
        let env = [("BEVYRROWIND_LAMP_DAY_GATE", "1"), ("BEVYRROWIND_POINT_FACE_CULL", "1"), ("BEVYRROWIND_SUN_SHADOW_GATE", "1")];
        let mut aliased = tunable_app(&env);
        assert_eq!(read(&aliased), (true, true, true), "env aliases");
        for (name, value) in [("shadow.lamp.day_gate", "off"), ("shadow.lamp.face_cull", "off"), ("shadow.sun.gate", "off")] {
            console_set(&mut aliased, name, value);
        }
        assert_eq!(read(&aliased), (false, false, false), "console above the alias");
        for (name, value) in [("shadow.lamp.day_gate", "on"), ("shadow.lamp.face_cull", "on"), ("shadow.sun.gate", "on")] {
            console_set(&mut app, name, value);
        }
        assert_eq!(read(&app), (true, true, true), "live on");
        // The thresholds stay the framework's.
        let w = app.world();
        assert_eq!(w.resource::<sf_sky_bevy::SunShadowGating>().0, SunShadowGate { enabled: true, ..SunShadowGate::default() });
        assert_eq!(w.resource::<PointShadowBudget>().contrast_gate, sf_sky_bevy::ShadowContrastGate { enabled: true, ..Default::default() });
        assert_eq!(*w.resource::<bevy_pbr::PointShadowFaceCull>(), bevy_pbr::PointShadowFaceCull { enabled: true, ..Default::default() });
    }

    /// The lamp budget follows its tunables: Bevyrrowind's fade at the `Mod` layer (two truths), the
    /// base preset's count and face size, `--point-shadows` and its env alias at `CommandLine`, a
    /// console set live.
    #[test]
    fn lamp_budget_follows_its_tunables() {
        let app = tunable_app(&[]);
        let b = *app.world().resource::<PointShadowBudget>();
        assert_eq!((b.max, b.map_size, b.fade_start, b.fade_end), (8, 512, 35.0, 45.0));
        let tunables = app.world().resource::<sf_tunables_bevy::Tunables>();
        let fade = tunables.var(tunables.id("shadow.lamp.fade_m").expect("fade"));
        assert_eq!((fade.set_by(), fade.default()), (sf_tunables_bevy::SetBy::Mod, &sf_tunables_bevy::Value::Float2([15.0, 25.0])));
        let mut app = tunable_app(&[("BEVYRROWIND_POINT_SHADOWS", "2")]);
        assert_eq!(app.world().resource::<PointShadowBudget>().max, 2, "env alias");
        console_set(&mut app, "shadow.lamp.count", "5");
        console_set(&mut app, "shadow.lamp.fade_m", "10,20");
        let b = *app.world().resource::<PointShadowBudget>();
        assert_eq!((b.max, b.fade_start, b.fade_end), (5, 10.0, 20.0));
        let mut cli = App::new();
        cli.add_plugins(sf_tunables_bevy::TunablesPlugin).insert_resource(sf_tunables_bevy::TunableEnv::from_pairs(&[("BEVYRROWIND_POINT_SHADOWS", "2")]));
        RenderSettings::new(false, Some(Upscaling::Off), true, true).expect("settings").with_point_shadows(Some(3)).insert(&mut cli);
        cli.update();
        assert_eq!(cli.world().resource::<PointShadowBudget>().max, 3, "--point-shadows wins over the env");
    }

    /// CW-8: the sun's and moons' shadow contrast on a receiver facing them per weather and hour, as the
    /// gate computes it (`sf_sky::sun_gate::contrasts`) under the game's sky: `LightCalibration`'s
    /// night floor and moonlight (copied: `mw_assets` is not a dependency), Masser and Secunda,
    /// each weather's haze, dust and cloud genus (`mw_weather::visual::Look`), the cloud shadow
    /// map's mean (`sf_atmos_bevy::mean_cloud_shadow`, default look). Not counted: the clouds'
    /// own glow, multiple scattering (the fill errs low), the ambient tint (luminance 1).
    #[test]
    #[ignore = "measurement: CW-8 sun/moon shadow contrast per weather"]
    fn print_sun_shadow_contrast_per_weather() {
        use mw_weather::visual::{Look, mw_clouds};
        use sf_sky_bevy::sf_sky::sun_gate::{SunShadowGate, contrasts, facing_direct};
        use sf_sky_bevy::sf_sky::{KeyCurve, Moonlight, SkyClock, SkySettings, Weather};
        let night_nits = 4.0_f32;
        let mut settings = SkySettings {
            ambient: KeyCurve::new(vec![(-12.0, night_nits), (-4.0, 0.0)]),
            moonlight: Moonlight { full_lux: 2.5 * std::f32::consts::PI * night_nits, artistic: true },
            ..SkySettings::default()
        };
        crate::night_sky::apply(&mut settings);
        let gate = SunShadowGate::default();
        let strength = sf_atmos_bevy::sf_atmos::CloudLook::default().shadow_strength;
        println!("band: off <= {} .. full >= {} (holder: x{})", gate.contrast_off, gate.contrast_full, 1.0 / gate.hysteresis);
        println!("weather        cover  cookie | hour  sun_deg  direct_lx  fill_lx   c_sun  w_sun | moon_lx  c_moon  w_moon");
        // Day 80 (equinox, as the bench's default sky); moon phase from the game's moons.
        for ty in mw_weather::WeatherType::ALL {
            let look = Look::of(ty);
            let cover = mw_clouds(look.genus).mean_cover();
            let weather = Weather { haze: look.haze, dust: look.dust, dust_tint: look.dust_tint, cloud_cover: cover, ..Weather::default() };
            let cookie = sf_atmos_bevy::mean_cloud_shadow(cover, strength);
            for hour in [8.0, 10.0, 12.0, 16.0, 17.5, 0.0] {
                let k = settings.evaluate(&SkyClock::at(80.0, hour, &settings.calendar), &weather);
                let ambient = std::f32::consts::PI * k.ambient;
                let (sun, moon) = contrasts(&k, cookie, ambient);
                let fill = k.sun.sky_fill + k.moon.sky_fill + ambient;
                println!(
                    "{:13} {cover:5.2}  {cookie:5.2}  | {hour:4.1} {:7.1} {:10.0} {:8.0}  {sun:6.3} {:6.2} | {:7.3} {moon:7.3} {:6.2}",
                    ty.name(),
                    k.sun.elevation.to_degrees(),
                    facing_direct(&k.sun, cookie),
                    fill,
                    if k.sun.casts_shadows { gate.weight(sun, false) } else { f32::NAN },
                    facing_direct(&k.moon, 1.0),
                    if k.moon.casts_shadows { gate.weight(moon, false) } else { f32::NAN },
                );
            }
        }
        // Clear: the sun elevation where the band starts (evening).
        let look = Look::of(mw_weather::WeatherType::Clear);
        let cover = mw_clouds(look.genus).mean_cover();
        let clear = Weather { haze: look.haze, cloud_cover: cover, ..Weather::default() };
        let cookie = sf_atmos_bevy::mean_cloud_shadow(cover, strength);
        for minutes in (15 * 60..19 * 60).step_by(10) {
            let k = settings.evaluate(&SkyClock::at(80.0, f64::from(minutes) / 60.0, &settings.calendar), &clear);
            let (sun, _) = contrasts(&k, cookie, std::f32::consts::PI * k.ambient);
            if k.sun.casts_shadows && sun < gate.contrast_full {
                println!("clear: below full strength from {:02}:{:02} (sun {:.1} deg, c {sun:.3})", minutes / 60, minutes % 60, k.sun.elevation.to_degrees());
                break;
            }
        }
    }

    /// F3 (ROADMAP 1.22): far sun cascades hold unless `--off cascadehold`, framework defaults otherwise.
    #[test]
    fn cascade_hold_switch() {
        for on in [true, false] {
            let mut app = App::new();
            let settings = RenderSettings::new(false, Some(Upscaling::Off), true, true)
                .expect("settings")
                .with_shadow_cache(false)
                .with_cascade_hold(on);
            settings.insert(&mut app);
            let hold = app.world().get_resource::<sf_sky_bevy::SunCascadeHold>().map(|h| h.0).expect("hold settings");
            let defaults = sf_sky_bevy::sf_sky::cascade_hold::CascadeHoldConfig::default();
            assert_eq!(hold, sf_sky_bevy::sf_sky::cascade_hold::CascadeHoldConfig { enabled: on, ..defaults });
        }
    }
}
