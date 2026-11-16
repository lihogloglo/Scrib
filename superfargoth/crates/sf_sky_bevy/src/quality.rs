//! Shadow and light quality as live tunables (tunables slice T4, spec `docs/specs/tunables.md`
//! "Migration"). [`ShadowQualityPlugin`] binds every shadow and light tunable to what consumes it,
//! so a console `set <name> <value>`, BRP or a preset level applies in the running game:
//!
//! - the lamp budget ([`PointShadowBudget`]: count, map size, fade, contact, day gate) and lamp
//!   face culling (`bevy_pbr::PointShadowFaceCull::enabled`);
//! - the sun/moon shadow gate ([`SunShadowGating`]);
//! - the sun's cascades and PCSS ([`SunShadowQuality`], when the app inserts it) into
//!   [`SkyRenderOptions`]; the sun and moon lights follow the options' cascades, bias, contact
//!   shadows and PCSS size when they change ([`sync_sky_lights`]);
//! - the temporal resolve (`render.upscaling` into [`UpscalingSetting`]);
//! - per-view contact shadows, GTAO and the shadow filter on cameras with
//!   [`ViewQualityCamera`] (components inserted and removed live);
//! - the views' prepasses and MSAA follow what needs them ([`prepasses_follow_consumers`]): contact
//!   shadows, GTAO and a temporal resolve each need the depth and motion-vector prepass (GTAO also
//!   normals and MSAA off, a temporal resolve MSAA off); the view's own needs are its
//!   [`ViewPrepassBase`]. An effect turned off live costs what a start without it costs (T5a).
//!
//! Opt-in: an app that doesn't add the plugin keeps these resources as it sets them. A game sets its
//! own defaults above the framework's at the `Mod` or `Preset` layer, never by redeclaring a def.

use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_core_pipeline::prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass};
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use bevy_ecs::system::{EntityCommands, SystemParam};
use bevy_light::{CascadeShadowConfig, DirectionalLight, ShadowFilteringMethod};
use bevy_pbr::{ContactShadows, ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy_render::camera::TemporalJitter;
use bevy_render::view::Msaa;
use sf_sky::sun_gate as sun;
use sf_sky::{RenderPath, Upscaling};
use sf_tunables::{AnyDef, Def, Group, Id, Lifetime, Source, Type, Unit, def};
use sf_tunables_bevy::{TunableSystems, Tunables, TunablesAppExt};

use crate::shadow_budget::{
    DAY_GATE, DAY_GATE_CONTRAST, DAY_GATE_RECEIVER, FACE_CULL, LAMP_CONTACT, LAMP_COUNT, LAMP_FADE,
    LAMP_FADE_TIME, LAMP_HYSTERESIS, LAMP_MAP_SIZE,
};
use crate::{
    ActiveRenderPath, CascadeSplits, MoonLight, PointShadowBudget, SkyRenderOptions, SkySystems, SunLight, SunShadowGating,
    UpscalingSetting,
};

/// `shadow.sun.cascades`: [`CascadeSplits::num_cascades`] of the sun. A change reallocates the
/// directional shadow array (Bevy's cascade count is per-light uniform data, no shader def).
pub const SUN_CASCADES: Def<usize> = def!("shadow.sun.cascades" = 4_usize)
    .range(1, 4)
    .unit(Unit::Count)
    .help("Sun shadow cascades (1-4)")
    .source(Source::BevyDefault("bevy_light::CascadeShadowConfigBuilder::num_cascades"))
    .group(Group::Shadows);

/// `shadow.sun.distance_m`: [`CascadeSplits::maximum_distance`] (past it the terrain's far shadow).
pub const SUN_DISTANCE: Def<f32> = def!(float "shadow.sun.distance_m" = 250.0)
    .range(10.0, 2000.0)
    .unit(Unit::Meters)
    .help("Far bound of the last sun cascade")
    .source(Source::Measured("docs/ledger/shadows.md 2026-10-19: PSSM practical split to 250 m"))
    .group(Group::Shadows);

/// `shadow.sun.split_lambda`: [`CascadeSplits::lambda`] (PSSM practical split, GPU Gems 3 ch. 10).
pub const SUN_LAMBDA: Def<f32> = def!(float "shadow.sun.split_lambda" = 0.9)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Sun cascade split: 1 = logarithmic, 0 = uniform")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.sun.overlap`: [`CascadeSplits::overlap_proportion`].
pub const SUN_OVERLAP: Def<f32> = def!(float "shadow.sun.overlap" = 0.2)
    .range(0.0, 0.9)
    .unit(Unit::Ratio)
    .help("Overlap of each sun cascade with the previous one, blended")
    .source(Source::BevyDefault("bevy_light::CascadeShadowConfigBuilder::overlap_proportion"))
    .group(Group::Shadows);

/// `shadow.sun.soft`: PCSS on the sun (`DirectionalLight::soft_shadow_size`; needs the `pcss`
/// feature). A per-light uniform, so live.
pub const SUN_SOFT: Def<bool> = def!(bool "shadow.sun.soft" = false)
    .help("Soft contact-hardening sun shadows (PCSS)")
    .source(Source::BevyDefault("bevy_light::DirectionalLight::soft_shadow_size (None)"))
    .group(Group::Shadows);

/// `shadow.sun.soft_size_texels`: the PCSS light size in cascade texels (penumbra = blocker gap x
/// size: crisp at contact, wider with distance). Tuned by eye (shadows F5).
pub const SUN_SOFT_SIZE: Def<f32> = def!(float "shadow.sun.soft_size_texels" = 10.0)
    .range(0.5, 64.0)
    .unit(Unit::Texels)
    .help("PCSS light size of the sun in cascade shadow-map texels")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.contact`: Bevy's `ContactShadows` on the managed views and the sun's
/// `contact_shadows_enabled`. The camera component is a mesh pipeline key: pipelines re-specialise.
pub const CONTACT: Def<bool> = def!(bool "shadow.contact" = false)
    .help("Screen-space contact shadows (depth-prepass ray march toward the sun and shadowed lamps)")
    .source(Source::BevyDefault("bevy_pbr::ContactShadows (opt-in per camera)"))
    .group(Group::Shadows)
    .lifetime(Lifetime::Recompile);

/// `shadow.contact.steps`: `ContactShadows::linear_steps` (uniform data).
pub const CONTACT_STEPS: Def<u32> = def!(count "shadow.contact.steps" = 16)
    .range(1, 128)
    .unit(Unit::Count)
    .help("Contact-shadow ray-march steps")
    .source(Source::BevyDefault("bevy_pbr::ContactShadows::linear_steps"))
    .group(Group::Shadows);

/// `shadow.contact.thickness_m`: `ContactShadows::thickness`.
pub const CONTACT_THICKNESS: Def<f32> = def!(float "shadow.contact.thickness_m" = 0.1)
    .range(0.001, 10.0)
    .unit(Unit::Meters)
    .help("Contact-shadow surface thickness")
    .source(Source::BevyDefault("bevy_pbr::ContactShadows::thickness"))
    .group(Group::Shadows);

/// `shadow.contact.length_m`: `ContactShadows::length` (was 0.5 m: long view-dependent bands, SH-14).
pub const CONTACT_LENGTH: Def<f32> = def!(float "shadow.contact.length_m" = 0.3)
    .range(0.01, 20.0)
    .unit(Unit::Meters)
    .help("Contact-shadow ray length")
    .source(Source::BevyDefault("bevy_pbr::ContactShadows::length"))
    .group(Group::Shadows);

/// `render.ssao.quality`: Bevy GTAO on the managed views (`off` or its quality level). The level is
/// a GTAO shader def and the component a mesh pipeline key: pipelines re-specialise.
pub const SSAO: Def<&'static str> = def!(choice "render.ssao.quality" = "off")
    .choices(&["off", "low", "medium", "high", "ultra"])
    .help("Screen-space ambient occlusion (Bevy GTAO): off or its quality level")
    .source(Source::BevyDefault("bevy_pbr::ScreenSpaceAmbientOcclusion (opt-in per camera)"))
    .group(Group::Lighting)
    .lifetime(Lifetime::Recompile);

/// `shadow.filter`: `ShadowFilteringMethod` of the managed views. `auto`: temporal when a temporal
/// resolve (TAA, DLSS) runs, else Gaussian (the temporal kernel is fixed-pattern grain without one,
/// Bevy's note in `shadow_sampling.wesl`; shadow audit 2026-11-01). A mesh pipeline key.
pub const FILTER: Def<&'static str> = def!(choice "shadow.filter" = "auto")
    .choices(&["auto", "hardware", "gaussian", "temporal"])
    .help("Shadow-map filtering: auto (temporal under TAA/DLSS, else gaussian), hardware (2x2), gaussian, temporal")
    .source(Source::Industry("Bevy shadow_sampling.wesl: the temporal kernel needs a temporal resolve"))
    .group(Group::Shadows)
    .lifetime(Lifetime::Recompile);

/// `render.upscaling`: the temporal resolve ([`UpscalingSetting`]): `off` keeps MSAA, `taa` is Bevy's
/// TAA, the rest are DLSS modes (TAA where DLSS is missing). A temporal resolve adds jitter and the
/// prepasses and drops MSAA: pipelines re-specialise.
pub const UPSCALING: Def<&'static str> = def!(choice "render.upscaling" = "off")
    .choices(&["off", "taa", "auto", "native", "dlaa", "quality", "balanced", "performance", "ultra-performance"])
    .help("Temporal resolve: off (MSAA), taa, or a DLSS mode: auto, native (dlaa), quality, balanced, performance, ultra-performance (TAA without DLSS)")
    .source(Source::BevyDefault("bevy_anti_alias: no temporal resolve unless the camera opts in"))
    .group(Group::Post)
    .lifetime(Lifetime::Recompile);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &UPSCALING,
    &SUN_CASCADES,
    &SUN_DISTANCE,
    &SUN_LAMBDA,
    &SUN_OVERLAP,
    &SUN_SOFT,
    &SUN_SOFT_SIZE,
    &CONTACT,
    &CONTACT_STEPS,
    &CONTACT_THICKNESS,
    &CONTACT_LENGTH,
    &SSAO,
    &FILTER,
];

/// Binds the shadow and light tunables (module docs). Add it after the game registered its own
/// copies of these defs (env aliases), before or after `SkyPlugin`.
pub struct ShadowQualityPlugin;

impl Plugin for ShadowQualityPlugin {
    fn build(&self, app: &mut App) {
        app.register_tunables(crate::shadow_budget::TUNABLES).register_tunables(sun::TUNABLES).register_tunables(TUNABLES);
        // The lamp budget.
        bind(app, &LAMP_COUNT, |b: &mut PointShadowBudget, v| b.max = v);
        bind(app, &LAMP_MAP_SIZE, |b: &mut PointShadowBudget, v| b.map_size = v);
        bind(app, &LAMP_FADE, |b: &mut PointShadowBudget, [start, end]| (b.fade_start, b.fade_end) = (start, end));
        bind(app, &LAMP_FADE_TIME, |b: &mut PointShadowBudget, v| b.fade_time = v);
        bind(app, &LAMP_HYSTERESIS, |b: &mut PointShadowBudget, v| b.hysteresis = v);
        bind(app, &LAMP_CONTACT, |b: &mut PointShadowBudget, v| b.contact_shadows = v);
        bind(app, &DAY_GATE, |b: &mut PointShadowBudget, v| b.contrast_gate.enabled = v);
        bind(app, &DAY_GATE_RECEIVER, |b: &mut PointShadowBudget, v| b.contrast_gate.receiver_distance = v);
        bind(app, &DAY_GATE_CONTRAST, |b: &mut PointShadowBudget, [off, full]| {
            (b.contrast_gate.contrast_off, b.contrast_gate.contrast_full) = (off, full);
        });
        bind(app, &FACE_CULL, |c: &mut bevy_pbr::PointShadowFaceCull, v| c.enabled = v);
        // The sun/moon shadow gate.
        bind(app, &sun::GATE, |g: &mut SunShadowGating, v| g.0.enabled = v);
        bind(app, &sun::CONTRAST, |g: &mut SunShadowGating, [off, full]| (g.0.contrast_off, g.0.contrast_full) = (off, full));
        bind(app, &sun::HYSTERESIS, |g: &mut SunShadowGating, v| g.0.hysteresis = v);
        bind(app, &sun::FADE, |g: &mut SunShadowGating, v| g.0.fade_seconds = v);
        // The sun's cascades and PCSS.
        bind(app, &SUN_CASCADES, |q: &mut SunShadowQuality, v| q.splits.num_cascades = v);
        bind(app, &SUN_DISTANCE, |q: &mut SunShadowQuality, v| q.splits.maximum_distance = v);
        bind(app, &SUN_LAMBDA, |q: &mut SunShadowQuality, v| q.splits.lambda = v);
        bind(app, &SUN_OVERLAP, |q: &mut SunShadowQuality, v| q.splits.overlap_proportion = v);
        bind(app, &SUN_SOFT, |q: &mut SunShadowQuality, v| q.soft = v);
        bind(app, &SUN_SOFT_SIZE, |q: &mut SunShadowQuality, v| q.soft_size = v);
        // The temporal resolve (`upscale.rs` puts it on the cameras).
        bind(app, &UPSCALING, |u: &mut UpscalingSetting, v| {
            if let Ok(setting) = Upscaling::parse(v) {
                u.0 = setting;
            }
        });
        let ids = ViewQualityIds::register(app);
        app.insert_resource(ids)
            .add_systems(PreUpdate, (apply_sun_quality, sync_sky_lights).chain().after(TunableSystems::Bind))
            .add_systems(Update, sync_view_quality.before(SkySystems::RenderPath))
            .add_systems(Update, prepasses_follow_consumers.after(SkySystems::RenderPath));
    }
}

/// Registers `def` and binds it into resource `R` (written the first frame `R` exists, then on a change).
fn bind<T: Type + Sync + Send, R: Resource<Mutability = Mutable>>(
    app: &mut App,
    def: &Def<T>,
    apply: impl for<'v> Fn(&mut R, T::Read<'v>) + Send + Sync + 'static,
) {
    let _bound = app.add_tunable(def).bind_resource(apply);
}

/// The sun's cascade layout and PCSS, bound to the `shadow.sun.*` tunables; while present,
/// [`SkyRenderOptions::cascades`] and its `sun_shadow.soft_size` follow it. An app opts in by
/// inserting it ([`SunShadowQuality::read`] gives the starting values).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SunShadowQuality {
    /// `minimum_distance` and `split_near` are the app's; the rest are the tunables'.
    pub splits: CascadeSplits,
    pub soft: bool,
    pub soft_size: f32,
}

impl Default for SunShadowQuality {
    fn default() -> Self {
        Self {
            splits: CascadeSplits {
                num_cascades: SUN_CASCADES.default_value(),
                minimum_distance: 0.1,
                split_near: 1.0,
                maximum_distance: SUN_DISTANCE.default_value(),
                lambda: SUN_LAMBDA.default_value(),
                overlap_proportion: SUN_OVERLAP.default_value(),
            },
            soft: SUN_SOFT.default_value(),
            soft_size: SUN_SOFT_SIZE.default_value(),
        }
    }
}

impl SunShadowQuality {
    /// The tunables' effective values (registered ones; defaults otherwise).
    pub fn read(tunables: &sf_tunables::Tunables) -> Self {
        let mut q = Self::default();
        let get = |def: &Def<f32>, fallback: f32| tunables.id_of(def).map_or(fallback, |id| tunables.read(id));
        q.splits.num_cascades = tunables.id_of(&SUN_CASCADES).map_or(q.splits.num_cascades, |id| tunables.read(id));
        q.splits.maximum_distance = get(&SUN_DISTANCE, q.splits.maximum_distance);
        q.splits.lambda = get(&SUN_LAMBDA, q.splits.lambda);
        q.splits.overlap_proportion = get(&SUN_OVERLAP, q.splits.overlap_proportion);
        q.soft = tunables.id_of(&SUN_SOFT).map_or(q.soft, |id| tunables.read(id));
        q.soft_size = get(&SUN_SOFT_SIZE, q.soft_size);
        q
    }

    /// `DirectionalLight::soft_shadow_size`.
    pub fn soft_size(&self) -> Option<f32> {
        self.soft.then_some(self.soft_size)
    }
}

#[allow(clippy::float_cmp)] // exact: change detection, not maths
fn same_cascades(a: &CascadeShadowConfig, b: &CascadeShadowConfig) -> bool {
    a.bounds == b.bounds && a.overlap_proportion == b.overlap_proportion && a.minimum_distance == b.minimum_distance
}

/// [`SunShadowQuality`] into [`SkyRenderOptions`] when it changed.
fn apply_sun_quality(quality: Option<Res<SunShadowQuality>>, options: Option<ResMut<SkyRenderOptions>>) {
    let (Some(quality), Some(mut options)) = (quality, options) else { return };
    if !quality.is_changed() {
        return;
    }
    let cascades = quality.splits.build();
    if !same_cascades(&options.cascades, &cascades) {
        options.cascades = cascades;
    }
    let soft = quality.soft_size();
    if options.sun_shadow.soft_size != soft {
        options.sun_shadow.soft_size = soft;
    }
}

/// The sun and moon lights follow [`SkyRenderOptions`] (read at their spawn) when it changes: the
/// sun's cascades, bias, contact shadows and PCSS size, the moon's bias. Fields already equal are
/// not written (no spurious change of the lights).
#[allow(clippy::float_cmp, clippy::needless_pass_by_value, clippy::type_complexity)] // exact: change detection
pub fn sync_sky_lights(
    options: Option<Res<SkyRenderOptions>>,
    mut suns: Query<(&mut DirectionalLight, &mut CascadeShadowConfig), (With<SunLight>, Without<MoonLight>)>,
    mut moons: Query<&mut DirectionalLight, (With<MoonLight>, Without<SunLight>)>,
) {
    let Some(options) = options else { return };
    if !options.is_changed() {
        return;
    }
    let s = options.sun_shadow;
    for (mut light, mut cascades) in &mut suns {
        if !same_cascades(&cascades, &options.cascades) {
            *cascades = options.cascades.clone();
        }
        let bias = (light.shadow_depth_bias, light.shadow_normal_bias, light.contact_shadows_enabled);
        if bias != (s.depth_bias, s.normal_bias, s.contact_shadows) {
            light.shadow_depth_bias = s.depth_bias;
            light.shadow_normal_bias = s.normal_bias;
            light.contact_shadows_enabled = s.contact_shadows;
        }
        #[cfg(feature = "pcss")]
        if light.soft_shadow_size != s.soft_size {
            light.soft_shadow_size = s.soft_size;
        }
    }
    for mut light in &mut moons {
        if (light.shadow_depth_bias, light.shadow_normal_bias) != (s.depth_bias, s.normal_bias) {
            light.shadow_depth_bias = s.depth_bias;
            light.shadow_normal_bias = s.normal_bias;
        }
    }
}

/// On a camera whose contact shadows, GTAO and shadow filter follow the tunables, and whose
/// prepasses and MSAA follow what needs them ([`prepasses_follow_consumers`]).
#[derive(Component, Clone, Copy, Debug, Default)]
#[require(ViewPrepassBase)]
pub struct ViewQualityCamera;

/// What a [`ViewQualityCamera`] needs for itself, whatever its effects: the prepasses the host wants
/// (e.g. an occlusion-culling A/B) and its MSAA when nothing needs MSAA off. Default: no prepass, the
/// camera's default MSAA (Bevy's 4x).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ViewPrepassBase {
    pub depth: bool,
    pub normal: bool,
    pub motion_vectors: bool,
    pub msaa: Msaa,
}

/// The prepasses and MSAA a raster view needs: its base, plus the depth and motion-vector prepass for
/// contact shadows and GTAO (they march the depth; the motion-vector pass gives every opaque prepass
/// draw a fragment stage, so the LOD dither applies) and for a temporal resolve (TAA/DLSS require
/// them), normals for GTAO, MSAA off under GTAO or a temporal resolve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeededPrepasses {
    pub depth: bool,
    pub normal: bool,
    pub motion_vectors: bool,
    pub msaa: Msaa,
}

impl NeededPrepasses {
    pub fn of(base: ViewPrepassBase, contact: bool, ssao: bool, temporal: bool) -> Self {
        let marched = contact || ssao || temporal;
        Self {
            depth: base.depth || marched,
            normal: base.normal || ssao,
            motion_vectors: base.motion_vectors || marched,
            msaa: if ssao || temporal { Msaa::Off } else { base.msaa },
        }
    }
}

type PrepassViews<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static ViewPrepassBase,
        Option<&'static Msaa>,
        (Has<DepthPrepass>, Has<NormalPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>),
        (Has<ContactShadows>, Has<ScreenSpaceAmbientOcclusion>, Has<TemporalJitter>),
    ),
    With<ViewQualityCamera>,
>;

/// Inserts and removes the views' depth, normal and motion-vector prepasses and sets their MSAA from
/// what currently needs them ([`NeededPrepasses`]), after the effects (`sync_view_quality`) and the
/// temporal resolve (`upscale.rs`) changed this frame: turning contact shadows, GTAO and TAA/DLSS
/// off live leaves the view as a start without them (Bevy only *adds* required components). Raster
/// only: on the ray-traced path, and the frame it switches back, Solari's deferred camera is
/// `path.rs`'s (its snapshot restores MSAA one frame later).
#[allow(clippy::needless_pass_by_value)]
pub fn prepasses_follow_consumers(mut commands: Commands, path: Option<Res<ActiveRenderPath>>, views: PrepassViews) {
    if let Some(path) = &path
        && (path.0 != RenderPath::Raster || (path.is_changed() && !path.is_added()))
    {
        return;
    }
    for (entity, base, msaa, (depth, normal, motion_vectors, deferred), (contact, ssao, temporal)) in &views {
        if deferred {
            continue;
        }
        let want = NeededPrepasses::of(*base, contact, ssao, temporal);
        let mut e = commands.entity(entity);
        match (want.depth, depth) {
            (true, false) => {
                e.insert(DepthPrepass);
            }
            (false, true) => {
                e.remove::<DepthPrepass>();
            }
            _ => {}
        }
        match (want.normal, normal) {
            (true, false) => {
                e.insert(NormalPrepass);
            }
            (false, true) => {
                e.remove::<NormalPrepass>();
            }
            _ => {}
        }
        match (want.motion_vectors, motion_vectors) {
            (true, false) => {
                e.insert(MotionVectorPrepass);
            }
            (false, true) => {
                e.remove::<MotionVectorPrepass>();
            }
            _ => {}
        }
        if msaa.copied() != Some(want.msaa) {
            e.insert(want.msaa);
        }
    }
}

/// The typed ids of the per-view tunables.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ViewQualityIds {
    contact: Id<bool>,
    steps: Id<u32>,
    thickness: Id<f32>,
    length: Id<f32>,
    ssao: Id<&'static str>,
    filter: Id<&'static str>,
}

impl ViewQualityIds {
    fn register(app: &mut App) -> Self {
        Self {
            contact: app.add_tunable(&CONTACT).id(),
            steps: app.add_tunable(&CONTACT_STEPS).id(),
            thickness: app.add_tunable(&CONTACT_THICKNESS).id(),
            length: app.add_tunable(&CONTACT_LENGTH).id(),
            ssao: app.add_tunable(&SSAO).id(),
            filter: app.add_tunable(&FILTER).id(),
        }
    }

    fn generations(&self, tunables: &Tunables) -> [u64; 6] {
        [
            self.contact.untyped(),
            self.steps.untyped(),
            self.thickness.untyped(),
            self.length.untyped(),
            self.ssao.untyped(),
            self.filter.untyped(),
        ]
        .map(|id| tunables.var(id).generation())
    }
}

/// What a managed view should carry.
#[derive(Clone, Copy)]
pub struct ViewQuality {
    pub contact: Option<ContactShadows>,
    pub ssao: Option<ScreenSpaceAmbientOcclusionQualityLevel>,
    pub filter: ShadowFilteringMethod,
}

/// `render.ssao.quality` text to Bevy's level (`off`: none).
pub fn ssao_level(name: &str) -> Option<ScreenSpaceAmbientOcclusionQualityLevel> {
    match name {
        "low" => Some(ScreenSpaceAmbientOcclusionQualityLevel::Low),
        "medium" => Some(ScreenSpaceAmbientOcclusionQualityLevel::Medium),
        "high" => Some(ScreenSpaceAmbientOcclusionQualityLevel::High),
        "ultra" => Some(ScreenSpaceAmbientOcclusionQualityLevel::Ultra),
        _ => None,
    }
}

/// `shadow.filter` text to Bevy's method; `auto` is temporal under a temporal resolve.
pub fn shadow_filter(name: &str, temporal: bool) -> ShadowFilteringMethod {
    match name {
        "hardware" => ShadowFilteringMethod::Hardware2x2,
        "gaussian" => ShadowFilteringMethod::Gaussian,
        "temporal" => ShadowFilteringMethod::Temporal,
        _ if temporal => ShadowFilteringMethod::Temporal,
        _ => ShadowFilteringMethod::Gaussian,
    }
}

#[allow(clippy::float_cmp)] // exact: change detection, not maths
fn same_contact(a: &ContactShadows, b: &ContactShadows) -> bool {
    (a.linear_steps, a.thickness, a.length) == (b.linear_steps, b.thickness, b.length)
}

impl ViewQuality {
    /// The tunables' effective values; `temporal`: a temporal resolve is set (for `auto`).
    pub fn read(tunables: &Tunables, ids: &ViewQualityIds, temporal: bool) -> Self {
        let contact = tunables.read(ids.contact).then(|| ContactShadows {
            linear_steps: tunables.read(ids.steps),
            thickness: tunables.read(ids.thickness),
            length: tunables.read(ids.length),
        });
        Self { contact, ssao: ssao_level(tunables.read(ids.ssao)), filter: shadow_filter(tunables.read(ids.filter), temporal) }
    }

    /// Inserts what a new view needs: contact shadows and GTAO (their prepasses and MSAA follow,
    /// [`prepasses_follow_consumers`]) and the shadow filter.
    pub fn insert_into(&self, e: &mut EntityCommands) {
        e.insert(self.filter);
        if let Some(contact) = self.contact {
            e.insert(contact);
        }
        if let Some(quality_level) = self.ssao {
            e.insert(ScreenSpaceAmbientOcclusion { quality_level, ..Default::default() });
        }
    }
}

/// What a system needs to compute [`ViewQuality`] (a camera spawn reads it). Without
/// [`ShadowQualityPlugin`] it has nothing to say (`None`).
#[derive(SystemParam)]
pub struct ViewQualityParam<'w> {
    tunables: Option<Res<'w, Tunables>>,
    ids: Option<Res<'w, ViewQualityIds>>,
    upscaling: Option<Res<'w, UpscalingSetting>>,
}

impl ViewQualityParam<'_> {
    pub fn wanted(&self) -> Option<ViewQuality> {
        Some(ViewQuality::read(self.tunables.as_ref()?, self.ids.as_ref()?, self.temporal()))
    }

    fn key(&self) -> Option<([u64; 6], bool)> {
        Some((self.ids.as_ref()?.generations(self.tunables.as_ref()?), self.temporal()))
    }

    fn temporal(&self) -> bool {
        self.upscaling.as_ref().is_some_and(|u| u.0.is_temporal())
    }
}

type ManagedViews<'w, 's> = Query<
    'w,
    's,
    (Entity, Option<&'static ContactShadows>, Option<&'static ScreenSpaceAmbientOcclusion>, Option<&'static ShadowFilteringMethod>, Has<crate::upscale::TemporalShadowsParked>),
    With<ViewQualityCamera>,
>;

/// Inserts, retunes or removes the views' contact shadows and GTAO and sets their shadow filter when
/// a per-view tunable or the upscaling setting changed (or a managed view appeared); the sun's
/// contact flag follows `shadow.contact`.
#[allow(clippy::needless_pass_by_value)]
fn sync_view_quality(
    mut commands: Commands,
    view: ViewQualityParam,
    options: Option<ResMut<SkyRenderOptions>>,
    cameras: ManagedViews,
    added: Query<(), Added<ViewQualityCamera>>,
    mut seen: Local<Option<([u64; 6], bool)>>,
) {
    let (Some(key), Some(want)) = (view.key(), view.wanted()) else { return };
    if *seen == Some(key) && added.is_empty() {
        return;
    }
    *seen = Some(key);
    for (camera, contact, ssao, filter, parked) in &cameras {
        let mut e = commands.entity(camera);
        match (want.contact, contact) {
            (Some(c), Some(have)) if !same_contact(&c, have) => {
                e.insert(c);
            }
            (Some(c), None) => {
                e.insert(c);
            }
            (None, Some(_)) => {
                e.remove::<ContactShadows>();
            }
            _ => {}
        }
        match (want.ssao, ssao) {
            (Some(level), Some(have)) if have.quality_level != level => {
                e.insert(ScreenSpaceAmbientOcclusion { quality_level: level, ..have.clone() });
            }
            (Some(quality_level), None) => {
                e.insert(ScreenSpaceAmbientOcclusion { quality_level, ..Default::default() });
            }
            (None, Some(_)) => {
                e.remove::<ScreenSpaceAmbientOcclusion>();
            }
            _ => {}
        }
        // A temporal filter parked to Gaussian while the view has no jitter counts as temporal
        // (`upscale::shadow_filter_follows_jitter` gives it back).
        let have = if parked { ShadowFilteringMethod::Temporal } else { filter.copied().unwrap_or_default() };
        if filter.is_none() || have != want.filter {
            e.insert(want.filter).remove::<crate::upscale::TemporalShadowsParked>();
        }
    }
    if let Some(mut options) = options {
        let on = want.contact.is_some();
        if options.sun_shadow.contact_shadows != on {
            options.sun_shadow.contact_shadows = on;
        }
    }
}
