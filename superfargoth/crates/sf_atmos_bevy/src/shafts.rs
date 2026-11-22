//! Screen-space light shaft occlusion (`docs/specs/aerial-shadow.md` "S3", ROADMAP 1.27 S3a,
//! FG-12): a sun hidden behind terrain stops lighting the haze in front of it. The classic
//! post-process (Mitchell, GPU Gems 3 ch. 13; UE "Light Shaft Occlusion"):
//!
//! 1. `shafts_mask` (half the view's internal resolution, after the water so its depth counts):
//!    m = 1 where the depth buffer shows sky or geometry farther than
//!    [`SHAFTS_OCCLUSION_DEPTH`], else 0, averaged over each 2×2 block.
//! 2. `shafts_blur`, [`SHAFTS_PASSES`] times: [`SHAFTS_TAPS`] taps toward the sun's screen
//!    position, each pass stepping `taps` times further than the last
//!    ([`sf_atmos::shafts::pass_step`]): together one long march of `taps^passes` samples, no noise,
//!    reaching [`SHAFTS_LENGTH`] of the way to the sun. Output o = how much of the way to the sun
//!    is open sky.
//! 3. The occlusion `1 − darkness · (1 − o) · lobe` (lobe: the haze's forward lobe toward the sun,
//!    normalised to 1 at it) dims Bevy's aerial perspective on geometry pixels (vendored
//!    `render_sky`, `bevy_pbr::AtmosphereLightShafts`, patch "own, LSO") and the analytic fog's sun
//!    term in the cloud composite. The froxels keep their own shadow-mapped sun.
//!
//! 4. **Bloom (S3b, UE "Light Shaft Bloom": the visible rays)**, after the clouds composite (so
//!    cloud gaps make rays and clouds block them), before the transparents and Bevy's
//!    post-processing: `shafts_bloom` takes the pre-exposed scene colour of the open-sky texels
//!    (the mask's depth test) at half resolution, rescaled down to [`SHAFTS_BLOOM_MAX`] and
//!    thresholded ([`SHAFTS_BLOOM_THRESHOLD`], [`sf_atmos::shafts::bloom_color`]), weighted by the
//!    falloff around the sun ([`SHAFTS_BLOOM_RADIUS`]); the same `shafts_blur` passes blur it toward
//!    the sun; `shafts_apply` adds it over the scene × [`SHAFTS_BLOOM_SCALE`] ×
//!    [`SHAFTS_BLOOM_TINT`]. Off at night: gated by the planet's shadow on the sun as seen from the
//!    camera ([`sf_atmos::shafts::sun_seen`], the twin of the composite's key light).
//!
//! Off with the sun behind the camera; the strength fades as the sun's screen position leaves the
//! screen ([`SHAFTS_OFFSCREEN_FADE`]). The sun is the [`CloudShadowLight`] (the light the far
//! terrain and cloud shadows are made for). GPU timers: `shafts` (mask + blurs), `shafts_bloom`
//! (bloom source + blurs + apply).

use bevy_app::{App, PostUpdate};
use bevy_asset::{load_embedded_asset, Handle};
use bevy_camera::{MainPassResolutionOverride, Viewport};
use bevy_core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy_core_pipeline::schedule::Core3d;
use bevy_core_pipeline::FullscreenShader;
use bevy_ecs::prelude::*;
use bevy_light::DirectionalLight;
use bevy_math::{UVec2, Vec2, Vec3};
use bevy_pbr::{main_transmissive_pass_3d, AtmosphereLightShafts, AtmosphereSettings, ExtractedAtmosphere, GpuAtmosphereLightShafts, RenderSkyInputs};
use bevy_render::camera::ExtractedCamera;
use bevy_render::diagnostic::RecordDiagnostics;
use bevy_render::render_resource::binding_types::{sampler, texture_2d, texture_2d_multisampled, uniform_buffer};
use bevy_render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation, BlendState,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, DynamicUniformBuffer, FilterMode, FragmentState, LoadOp, MultisampleState, Operations,
    PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, ShaderType, StoreOp, TextureFormat, TextureSampleType, TextureView, UniformBuffer,
};
use bevy_render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::{CachedTexture, TextureCache};
use bevy_render::view::{ExtractedView, Msaa, ViewDepthStencilTexture, ViewTarget, ViewUniformOffset, ViewUniforms};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems};
use bevy_shader::Shader;
use bevy_transform::components::GlobalTransform;
use bevy_transform::TransformSystems;
use sf_atmos::shafts as core;
use sf_tunables::{Def, Group, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::CloudShadowLight;

/// `sky.shafts.enabled`: the screen-space light shaft occlusion (S3a). Off by default since A2
/// (aerial-shadow.md "Architecture review": the sun-visibility pass, `sky.sun_vis.enabled`, replaces
/// it); the opt-in A/B arm. With both on, the sun-visibility pass owns the aerial and fog hooks.
/// `--off shafts` sets it false at the command-line layer.
pub const SHAFTS_ENABLED: Def<bool> = def!(bool "sky.shafts.enabled" = false)
    .help("Screen-space light shaft occlusion: the haze toward a sun hidden by the scene stops glowing")
    .source(Source::Industry("UE directional light 'Light Shaft Occlusion'; Mitchell, GPU Gems 3 ch. 13"))
    .group(Group::Sky);

/// `sky.shafts.occlusion`: how dark the haze's forward lobe gets where the way to the sun is
/// closed (0 = stock, 1 = none of the sun's in-scattering left at the sun).
pub const SHAFTS_OCCLUSION: Def<f32> = def!(float "sky.shafts.occlusion" = 0.95)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Darkness of the haze toward the sun where the scene hides it (0 = stock haze, 1 = fully masked)")
    .source(Source::Industry("UE OcclusionMaskDarkness default 0.05 (the occluded value): darkness 1 - 0.05"))
    .group(Group::Sky);

/// `sky.shafts.occlusion_depth_m`: geometry farther than this counts as open sky in the mask.
pub const SHAFTS_OCCLUSION_DEPTH: Def<f32> = def!(float "sky.shafts.occlusion_depth_m" = 100_000.0)
    .range(0.0, 1_000_000.0)
    .unit(Unit::Meters)
    .help("Geometry nearer than this hides the sun from the haze; farther counts as open sky")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.passes`: radial blur passes (each steps `taps` times further than the last).
pub const SHAFTS_PASSES: Def<u32> = def!(count "sky.shafts.passes" = 3)
    .range(0, 6)
    .unit(Unit::Count)
    .help("Radial blur passes of the light shaft mask toward the sun (each reaches taps times further)")
    .source(Source::Industry("UE r.LightShaftBlurPasses default 3"))
    .group(Group::Sky);

/// `sky.shafts.taps`: taps per radial blur pass.
pub const SHAFTS_TAPS: Def<u32> = def!(count "sky.shafts.taps" = 8)
    .range(1, 32)
    .unit(Unit::Count)
    .help("Taps per radial blur pass of the light shaft mask")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.length`: how far toward the sun the passes reach together.
pub const SHAFTS_LENGTH: Def<f32> = def!(float "sky.shafts.length" = 1.0)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("How far toward the sun's screen position the light shaft blur reaches (fraction of the way)")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.offscreen_fade`: the strength fades to 0 as the sun's screen position gets this far
/// outside the screen (UV units).
pub const SHAFTS_OFFSCREEN_FADE: Def<f32> = def!(float "sky.shafts.offscreen_fade" = 0.3)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Distance outside the screen (fraction of the screen) over which the light shaft occlusion fades out")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.lobe_g`: the masked lobe's Henyey-Greenstein asymmetry.
pub const SHAFTS_LOBE_G: Def<f32> = def!(float "sky.shafts.lobe_g" = 0.8)
    .range(0.0, 0.99)
    .unit(Unit::Ratio)
    .help("Forward-lobe asymmetry of the haze the light shaft occlusion masks (Henyey-Greenstein g)")
    .source(Source::Reference("sf_sky AtmosphereModel::earth haze Mie asymmetry 0.8 (Bevy's earth medium)"))
    .group(Group::Sky);

/// `sky.shafts.bloom`: the light shaft bloom, the visible rays (S3b). Needs `sky.shafts.enabled`.
/// Off by default (owner 2026-11-22, FG-12: very blocky, shimmers even with the sun still, redundant
/// with the occlusion's shafts; parked). On: `set sky.shafts.bloom true`.
pub const SHAFTS_BLOOM: Def<bool> = def!(bool "sky.shafts.bloom" = false)
    .help("Light shaft bloom: the bright sky around the sun blurred toward it and added back (visible god rays)")
    .source(Source::Industry("UE directional light 'Light Shaft Bloom'; Mitchell, GPU Gems 3 ch. 13"))
    .group(Group::Sky);

/// `sky.shafts.bloom_scale`: how much of the blurred bloom is added over the scene. UE's light
/// default is 0.2 as recalled (UE's docs give no default): `Guess` until checked or tuned.
pub const SHAFTS_BLOOM_SCALE: Def<f32> = def!(float "sky.shafts.bloom_scale" = 0.2)
    .range(0.0, 10.0)
    .unit(Unit::Ratio)
    .help("Strength of the light shaft bloom added over the scene (UE 'Bloom Scale')")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.bloom_threshold`: the pre-exposed luminance a sky texel must exceed to bloom. Our
/// scene colour is Bevy's pre-exposed HDR (the view's exposure applied: 1 ≈ white before
/// tonemapping), the space of UE's scene colour after its pre-exposure, so UE's number is taken
/// as is: 8 = three stops over white, the sun's disc and inner aureole. UE's default 8 as
/// recalled (its docs give none): `Guess`.
pub const SHAFTS_BLOOM_THRESHOLD: Def<f32> = def!(float "sky.shafts.bloom_threshold" = 8.0)
    .range(0.0, 1000.0)
    .unit(Unit::None)
    .help("Pre-exposed sky luminance (1 = white before tonemapping) that must be exceeded to make light shafts (UE 'Bloom Threshold')")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.bloom_max`: a sky texel's pre-exposed luminance is rescaled down to this before the
/// threshold (UE docs: "after exposure is applied, scene color brightness larger than Bloom Max
/// Brightness will be rescaled down to Bloom Max Brightness"). UE's default 100 as recalled:
/// `Guess`.
pub const SHAFTS_BLOOM_MAX: Def<f32> = def!(float "sky.shafts.bloom_max" = 100.0)
    .range(0.0, 100_000.0)
    .unit(Unit::None)
    .help("Pre-exposed luminance a bright sky texel (the sun's disc) is clamped to before it makes light shafts (UE 'Bloom Max Brightness')")
    .source(Source::Guess)
    .group(Group::Sky);

/// `sky.shafts.bloom_tint`: linear RGB multiplier of the added bloom (white = the sky's own colour).
pub const SHAFTS_BLOOM_TINT: Def<[f32; 3]> = def!(float3 "sky.shafts.bloom_tint" = [1.0, 1.0, 1.0])
    .range([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])
    .help("Linear RGB tint of the light shaft bloom (white = the sky's own colour; UE 'Bloom Tint')")
    .source(Source::Industry("UE light 'Bloom Tint' multiplies the bloom colour; white leaves it as the scene's"))
    .group(Group::Sky);

/// `sky.shafts.bloom_radius`: the bloom source's falloff around the sun's screen position, in
/// screen heights (`(1 − d / radius)²`, [`sf_atmos::shafts::bloom_falloff`]).
pub const SHAFTS_BLOOM_RADIUS: Def<f32> = def!(float "sky.shafts.bloom_radius" = 0.5)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Radius around the sun (fraction of the screen height) whose bright sky makes light shafts")
    .source(Source::Guess)
    .group(Group::Sky);

/// The light shaft occlusion's settings (bound from the `sky.shafts.*` tunables).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LightShafts {
    pub enabled: bool,
    /// Darkness of a fully closed pixel looking at the sun, `0..1`.
    pub occlusion: f32,
    /// Geometry farther than this is open sky, meters.
    pub occlusion_depth_m: f32,
    pub passes: u32,
    pub taps: u32,
    /// Reach toward the sun, fraction of the way.
    pub length: f32,
    /// Fade distance outside the screen, UV units.
    pub offscreen_fade: f32,
    pub lobe_g: f32,
    /// The light shaft bloom (S3b).
    pub bloom: bool,
    /// Added fraction of the blurred bloom.
    pub bloom_scale: f32,
    /// Pre-exposed luminance to exceed.
    pub bloom_threshold: f32,
    /// Pre-exposed luminance a texel is rescaled down to first.
    pub bloom_max: f32,
    /// Linear RGB tint.
    pub bloom_tint: Vec3,
    /// Falloff radius around the sun, screen heights.
    pub bloom_radius: f32,
}

impl Default for LightShafts {
    fn default() -> Self {
        Self {
            enabled: SHAFTS_ENABLED.default_value(),
            occlusion: SHAFTS_OCCLUSION.default_value(),
            occlusion_depth_m: SHAFTS_OCCLUSION_DEPTH.default_value(),
            passes: SHAFTS_PASSES.default_value(),
            taps: SHAFTS_TAPS.default_value(),
            length: SHAFTS_LENGTH.default_value(),
            offscreen_fade: SHAFTS_OFFSCREEN_FADE.default_value(),
            lobe_g: SHAFTS_LOBE_G.default_value(),
            bloom: SHAFTS_BLOOM.default_value(),
            bloom_scale: SHAFTS_BLOOM_SCALE.default_value(),
            bloom_threshold: SHAFTS_BLOOM_THRESHOLD.default_value(),
            bloom_max: SHAFTS_BLOOM_MAX.default_value(),
            bloom_tint: Vec3::from_array(SHAFTS_BLOOM_TINT.default_value()),
            bloom_radius: SHAFTS_BLOOM_RADIUS.default_value(),
        }
    }
}

/// Toward the sun (world space, unit): the [`CloudShadowLight`]'s direction; `None` without one.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShaftsSun(pub Option<Vec3>);

/// Registers the tunables, binds them, tracks the sun and adds the render passes.
pub(crate) fn build(app: &mut App) {
    app.init_resource::<LightShafts>().init_resource::<ShaftsSun>();
    let _bound = app.add_tunable(&SHAFTS_ENABLED).bind_resource(|s: &mut LightShafts, v| s.enabled = v);
    let _bound = app.add_tunable(&SHAFTS_OCCLUSION).bind_resource(|s: &mut LightShafts, v| s.occlusion = v);
    let _bound = app.add_tunable(&SHAFTS_OCCLUSION_DEPTH).bind_resource(|s: &mut LightShafts, v| s.occlusion_depth_m = v);
    let _bound = app.add_tunable(&SHAFTS_PASSES).bind_resource(|s: &mut LightShafts, v| s.passes = v);
    let _bound = app.add_tunable(&SHAFTS_TAPS).bind_resource(|s: &mut LightShafts, v| s.taps = v);
    let _bound = app.add_tunable(&SHAFTS_LENGTH).bind_resource(|s: &mut LightShafts, v| s.length = v);
    let _bound = app.add_tunable(&SHAFTS_OFFSCREEN_FADE).bind_resource(|s: &mut LightShafts, v| s.offscreen_fade = v);
    let _bound = app.add_tunable(&SHAFTS_LOBE_G).bind_resource(|s: &mut LightShafts, v| s.lobe_g = v);
    let _bound = app.add_tunable(&SHAFTS_BLOOM).bind_resource(|s: &mut LightShafts, v| s.bloom = v);
    let _bound = app.add_tunable(&SHAFTS_BLOOM_SCALE).bind_resource(|s: &mut LightShafts, v| s.bloom_scale = v);
    let _bound = app.add_tunable(&SHAFTS_BLOOM_THRESHOLD).bind_resource(|s: &mut LightShafts, v| s.bloom_threshold = v);
    let _bound = app.add_tunable(&SHAFTS_BLOOM_MAX).bind_resource(|s: &mut LightShafts, v| s.bloom_max = v);
    let _bound = app.add_tunable(&SHAFTS_BLOOM_TINT).bind_resource(|s: &mut LightShafts, v| s.bloom_tint = Vec3::from_array(v));
    let _bound = app.add_tunable(&SHAFTS_BLOOM_RADIUS).bind_resource(|s: &mut LightShafts, v| s.bloom_radius = v);
    app.add_systems(PostUpdate, track_sun.after(TransformSystems::Propagate));

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app
        .init_resource::<ShaftsFrame>()
        .init_resource::<ShaftsPipelineIds>()
        .add_systems(ExtractSchedule, extract)
        .add_systems(RenderStartup, bevy_render::init_gpu_resource::<ShaftsPipelines>)
        .add_systems(
            Render,
            (
                queue_pipelines.in_set(RenderSystems::Queue),
                prepare_views.in_set(RenderSystems::PrepareResources),
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            ),
        )
        // After the water (its depth is in the mask, W-ORDER), before `render_sky` and the
        // composite that read the result.
        .add_systems(
            Core3d,
            shafts_pass
                .after(main_opaque_pass_3d)
                .after(main_transmissive_pass_3d)
                .in_set(RenderSkyInputs)
                .before(main_transparent_pass_3d),
        );
    // `bloom_pass` goes after the clouds composite: `render::finish`.
}

/// Copies the [`CloudShadowLight`]'s direction into [`ShaftsSun`] (written only when it changes).
pub fn track_sun(lights: Query<&GlobalTransform, (With<CloudShadowLight>, With<DirectionalLight>)>, mut sun: ResMut<ShaftsSun>) {
    let dir = lights.iter().next().and_then(|t| t.back().as_vec3().try_normalize());
    sun.set_if_neq(ShaftsSun(dir));
}

/// What a view's passes and the occlusion use this frame (CPU side, testable without a GPU).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShaftsViewParams {
    /// The sun's screen position (UV, y down); `None` behind the camera.
    pub sun_uv: Option<Vec2>,
    /// `occlusion` × the off-screen fade; 0 = the passes are skipped and nothing is masked.
    pub darkness: f32,
    /// Blur passes (`taps` < 2 or `length` 0: none).
    pub passes: u32,
    pub taps: u32,
    /// The bloom's added fraction: `bloom_scale` × the off-screen fade × the night gate; 0 = no
    /// bloom passes (also with `sky.shafts.bloom` off).
    pub bloom: f32,
}

impl ShaftsViewParams {
    /// From the settings, the view's projection and rotation (`view_from_world` of a direction),
    /// the direction toward the sun and how much of the sun the camera sees past the planet's
    /// shadow (`sun_seen`, [`sf_atmos::shafts::sun_seen`]: the bloom's night gate).
    #[must_use]
    pub fn new(settings: &LightShafts, clip_from_view: bevy_math::Mat4, toward_sun_view: Vec3, sun_seen: f32) -> Self {
        let sun_uv = core::sun_uv(clip_from_view, toward_sun_view);
        let strength = sun_uv.map_or(0.0, |uv| core::offscreen_strength(uv, settings.offscreen_fade));
        let blur = settings.taps >= 2 && settings.length > 0.0;
        Self {
            sun_uv,
            darkness: settings.occlusion.clamp(0.0, 1.0) * strength,
            passes: if blur { settings.passes.min(MAX_PASSES) } else { 0 },
            taps: settings.taps.clamp(1, MAX_TAPS),
            bloom: if settings.bloom { settings.bloom_scale.max(0.0) * strength * sun_seen.clamp(0.0, 1.0) } else { 0.0 },
        }
    }

    /// The occlusion passes run (and anything is masked).
    #[must_use]
    pub fn active(&self) -> bool {
        self.darkness > 0.0 && self.sun_uv.is_some()
    }

    /// The bloom passes run (and something is added).
    #[must_use]
    pub fn bloom_active(&self) -> bool {
        self.bloom > 0.0 && self.sun_uv.is_some()
    }
}

/// The most blur passes and taps the passes take (the tunables' ranges).
// tunables: const (capacity: the top of sky.shafts.passes' range)
const MAX_PASSES: u32 = 6;
// tunables: const (capacity: the top of sky.shafts.taps' range)
const MAX_TAPS: u32 = 32;
/// The mask and its blurs: one channel, filterable, smooth averages.
const MASK: TextureFormat = TextureFormat::R16Float;
/// The bloom and its blurs: pre-exposed HDR colour (≤ `bloom_max`, f16 holds it).
const BLOOM: TextureFormat = TextureFormat::Rgba16Float;

// -- render world --------------------------------------------------------------

/// The main world's [`LightShafts`] and [`ShaftsSun`], extracted.
#[derive(Resource, Clone, Copy, Debug, Default)]
struct ShaftsFrame {
    settings: Option<LightShafts>,
    sun: Option<Vec3>,
}

/// A render-world camera that gets the light shaft passes (while [`LightShafts::enabled`], with a
/// sun, on a camera with Bevy's `AtmosphereSettings`).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShaftsCamera;

/// A [`ShaftsCamera`] that also gets the bloom passes (while [`LightShafts::bloom`]).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShaftsBloomCamera;

/// GPU layout of `ShaftsPass` in `shafts_common.wesl` (field order is the layout).
#[derive(Clone, Copy, Default, ShaderType)]
pub(crate) struct ShaftsPassUniform {
    /// The sun's screen position (UV).
    pub sun_uv: Vec2,
    /// Blur: this pass's step, fraction of the way to the sun.
    pub step: f32,
    pub taps: u32,
    /// The view's internal resolution (the depth texels the mask reads).
    pub full_size: Vec2,
    /// Mask: geometry farther than this is open sky, meters.
    pub depth_m: f32,
}

/// GPU layout of `ShaftsBloom` in `shafts_common.wesl` (field order is the layout).
#[derive(Clone, Copy, Default, ShaderType)]
pub(crate) struct ShaftsBloomUniform {
    /// Added colour per unit of blurred bloom: tint × [`ShaftsViewParams::bloom`].
    pub tint: Vec3,
    /// Pre-exposed luminance to exceed.
    pub threshold: f32,
    /// Pre-exposed luminance a texel is rescaled down to first.
    pub max_luminance: f32,
    /// Falloff radius around the sun, screen heights.
    pub radius: f32,
    /// The view's width / height.
    pub aspect: f32,
}

/// A view's bloom textures and uniform.
struct BloomView {
    /// The source writes `a`; blur pass k reads `a` (k even) or `b` and writes the other.
    textures: (CachedTexture, CachedTexture),
    uniform: UniformBuffer<ShaftsBloomUniform>,
}

/// A view's light shaft textures and uniforms (prepared in `PrepareResources`).
#[derive(Component)]
pub(crate) struct ShaftsView {
    params: ShaftsViewParams,
    /// The mask writes `a`; blur pass k reads `a` (k even) or `b` and writes the other.
    textures: (CachedTexture, CachedTexture),
    passes: DynamicUniformBuffer<ShaftsPassUniform>,
    /// Dynamic offsets: the mask's (also the bloom source's), then one per blur pass (shared by
    /// the mask's and the bloom's blurs).
    offsets: Vec<u32>,
    samples: u32,
    /// The occlusion's uniform ([`GpuAtmosphereLightShafts`]), shared with the composite.
    pub occlusion: UniformBuffer<GpuAtmosphereLightShafts>,
    /// The bloom's, on a [`ShaftsBloomCamera`] whose bloom pipelines are ready.
    bloom: Option<BloomView>,
}

/// After `passes` blur passes ping-ponging from `a`: `a` after an even number, `b` after an odd one.
fn ping_pong_result(textures: &(CachedTexture, CachedTexture), passes: u32) -> &CachedTexture {
    if passes % 2 == 1 { &textures.1 } else { &textures.0 }
}

impl ShaftsView {
    /// The final occlusion texture.
    pub(crate) fn result(&self) -> &CachedTexture {
        ping_pong_result(&self.textures, self.params.passes)
    }
}

/// The view's bind groups: the mask's, then one per blur pass; the bloom's blurs and apply (the
/// bloom source's is made in the pass: it reads the view's current main texture).
#[derive(Component)]
pub(crate) struct ShaftsBindGroups {
    mask: BindGroup,
    blur: Vec<BindGroup>,
    bloom: Option<(Vec<BindGroup>, BindGroup)>,
}

/// The light shaft pipelines, by MSAA samples where they read depth or draw into the view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ShaftsPipe {
    /// The occlusion mask (depth, `samples`).
    Mask(u32),
    /// The radial blur into the mask format.
    Blur,
    /// The bloom source (depth, `samples`; the resolved scene colour).
    BloomSource(u32),
    /// The same radial blur shader into the bloom format.
    BloomBlur,
    /// The bloom added over the view (`samples`).
    Apply(u32),
}

#[derive(Resource)]
pub(crate) struct ShaftsPipelines {
    mask: BindGroupLayoutDescriptor,
    mask_msaa: BindGroupLayoutDescriptor,
    blur: BindGroupLayoutDescriptor,
    bloom_source: BindGroupLayoutDescriptor,
    bloom_source_msaa: BindGroupLayoutDescriptor,
    apply: BindGroupLayoutDescriptor,
    /// mask, blur, bloom source, apply.
    shaders: [Handle<Shader>; 4],
    fullscreen: FullscreenShader,
    sampler: Sampler,
}

impl FromWorld for ShaftsPipelines {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let linear = device.create_sampler(&SamplerDescriptor {
            label: Some("shafts"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..SamplerDescriptor::default()
        });
        let mask = |label, depth| {
            BindGroupLayoutDescriptor::new(
                label,
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (uniform_buffer::<ShaftsPassUniform>(true), uniform_buffer::<bevy_render::view::ViewUniform>(true), depth),
                ),
            )
        };
        let bloom_source = |label, depth| {
            BindGroupLayoutDescriptor::new(
                label,
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (
                        uniform_buffer::<ShaftsPassUniform>(true),
                        uniform_buffer::<bevy_render::view::ViewUniform>(true),
                        depth,
                        uniform_buffer::<ShaftsBloomUniform>(false),
                        texture_2d(TextureSampleType::default()),
                    ),
                ),
            )
        };
        let sampled = |label| {
            BindGroupLayoutDescriptor::new(
                label,
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (uniform_buffer::<ShaftsPassUniform>(true), texture_2d(TextureSampleType::default()), sampler(SamplerBindingType::Filtering)),
                ),
            )
        };
        Self {
            mask: mask("shafts_mask", texture_2d(TextureSampleType::Depth)),
            mask_msaa: mask("shafts_mask_msaa", texture_2d_multisampled(TextureSampleType::Depth)),
            blur: sampled("shafts_blur"),
            bloom_source: bloom_source("shafts_bloom_source", texture_2d(TextureSampleType::Depth)),
            bloom_source_msaa: bloom_source("shafts_bloom_source_msaa", texture_2d_multisampled(TextureSampleType::Depth)),
            apply: BindGroupLayoutDescriptor::new(
                "shafts_apply",
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::FRAGMENT,
                    (uniform_buffer::<ShaftsBloomUniform>(false), texture_2d(TextureSampleType::default()), sampler(SamplerBindingType::Filtering)),
                ),
            ),
            shaders: [
                load_embedded_asset!(world, "shaders/shafts_mask.wesl"),
                load_embedded_asset!(world, "shaders/shafts_blur.wesl"),
                load_embedded_asset!(world, "shaders/shafts_bloom.wesl"),
                load_embedded_asset!(world, "shaders/shafts_apply.wesl"),
            ],
            fullscreen: world.resource::<FullscreenShader>().clone(),
            sampler: linear,
        }
    }
}

/// Adds the source over the destination's colour, keeps its alpha.
const ADD: BlendState = BlendState {
    color: BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
    alpha: BlendComponent { src_factor: BlendFactor::Zero, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
};

impl ShaftsPipelines {
    fn descriptor(&self, pipe: ShaftsPipe) -> RenderPipelineDescriptor {
        // (layout, shader, entry, target format, blend, depth samples read, samples drawn)
        let (layout, shader, entry, format, blend, depth_samples, draw_samples) = match pipe {
            ShaftsPipe::Mask(n) => (if n > 1 { &self.mask_msaa } else { &self.mask }, 0, "mask", MASK, None, n, 1),
            ShaftsPipe::Blur => (&self.blur, 1, "blur", MASK, None, 1, 1),
            ShaftsPipe::BloomSource(n) => (if n > 1 { &self.bloom_source_msaa } else { &self.bloom_source }, 2, "source", BLOOM, None, n, 1),
            ShaftsPipe::BloomBlur => (&self.blur, 1, "blur", BLOOM, None, 1, 1),
            ShaftsPipe::Apply(n) => (&self.apply, 3, "apply", crate::render::HDR, Some(ADD), 1, n),
        };
        let shader_defs = if depth_samples > 1 { vec!["MULTISAMPLED".into()] } else { Vec::new() };
        RenderPipelineDescriptor {
            label: Some(format!("shafts_{pipe:?}").into()),
            layout: vec![layout.clone()],
            vertex: self.fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shaders[shader].clone(),
                shader_defs,
                entry_point: Some(entry.into()),
                targets: vec![Some(ColorTargetState { format, blend, write_mask: ColorWrites::ALL })],
                ..FragmentState::default()
            }),
            multisample: MultisampleState { count: draw_samples.max(1), ..MultisampleState::default() },
            ..RenderPipelineDescriptor::default()
        }
    }
}

/// The queued pipelines.
#[derive(Resource, Default)]
pub(crate) struct ShaftsPipelineIds(std::collections::HashMap<ShaftsPipe, CachedRenderPipelineId>);

impl ShaftsPipelineIds {
    fn get<'a>(&self, cache: &'a PipelineCache, pipe: ShaftsPipe) -> Option<&'a RenderPipeline> {
        self.0.get(&pipe).and_then(|id| cache.get_render_pipeline(*id))
    }
}

/// The occlusion's pipelines for `samples`.
fn occlusion_pipes(samples: u32) -> [ShaftsPipe; 2] {
    [ShaftsPipe::Mask(samples), ShaftsPipe::Blur]
}

/// The bloom's pipelines for `samples`.
fn bloom_pipes(samples: u32) -> [ShaftsPipe; 3] {
    [ShaftsPipe::BloomSource(samples), ShaftsPipe::BloomBlur, ShaftsPipe::Apply(samples)]
}

type CameraItem = (RenderEntity, Has<AtmosphereSettings>);

pub(crate) fn extract(
    mut commands: Commands,
    settings: Extract<Res<LightShafts>>,
    sun: Extract<Res<ShaftsSun>>,
    sun_vis: Extract<Option<Res<crate::SunVisibility>>>,
    cameras: Extract<Query<CameraItem, With<bevy_camera::Camera3d>>>,
) {
    let on = settings.enabled && sun.0.is_some();
    let sun_vis_on = sun_vis.as_ref().is_some_and(|s| s.enabled) && sun.0.is_some();
    commands.insert_resource(ShaftsFrame { settings: on.then_some(**settings), sun: sun.0 });
    for (entity, atmosphere) in &cameras {
        let mut e = commands.entity(entity);
        if on && atmosphere {
            e.insert(ShaftsCamera);
            if settings.bloom {
                e.insert(ShaftsBloomCamera);
            } else {
                e.remove::<ShaftsBloomCamera>();
            }
        } else {
            e.remove::<(ShaftsCamera, ShaftsBloomCamera, ShaftsView, ShaftsBindGroups)>();
            // The sun-visibility pass owns the vendored hook while it is on; else the vendored
            // `render_sky` falls back to its stock pipeline the same frame.
            if !(sun_vis_on && atmosphere) {
                e.remove::<AtmosphereLightShafts>();
            }
        }
    }
}

fn queue_pipelines(
    views: Query<(&Msaa, Has<ShaftsBloomCamera>), With<ShaftsCamera>>,
    pipelines: Option<Res<ShaftsPipelines>>,
    cache: Res<PipelineCache>,
    mut ids: ResMut<ShaftsPipelineIds>,
) {
    let Some(pipelines) = pipelines else { return };
    for (msaa, bloom) in &views {
        let samples = msaa.samples();
        let bloom = bloom.then(|| bloom_pipes(samples));
        for pipe in occlusion_pipes(samples).into_iter().chain(bloom.into_iter().flatten()) {
            ids.0.entry(pipe).or_insert_with(|| cache.queue_render_pipeline(pipelines.descriptor(pipe)));
        }
    }
}

type ViewItem = (
    Entity,
    &'static ExtractedView,
    &'static Msaa,
    Option<&'static MainPassResolutionOverride>,
    Option<&'static ExtractedAtmosphere>,
    Has<ShaftsBloomCamera>,
    Has<crate::SunVisCamera>,
);

#[allow(clippy::too_many_arguments)]
fn prepare_views(
    mut commands: Commands,
    views: Query<ViewItem, With<ShaftsCamera>>,
    frame: Res<ShaftsFrame>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut textures: ResMut<TextureCache>,
    (ids, cache): (Res<ShaftsPipelineIds>, Res<PipelineCache>),
) {
    let (Some(settings), Some(sun)) = (frame.settings, frame.sun) else { return };
    let ready = |pipes: &[ShaftsPipe]| pipes.iter().all(|p| ids.get(&cache, *p).is_some());
    for (entity, view, msaa, resolution, atmosphere, bloom_camera, sun_vis) in &views {
        let full = resolution.map_or(UVec2::new(view.viewport.z, view.viewport.w), |r| r.0).max(UVec2::ONE);
        let half = (full + 1) / 2;
        let toward_sun_view = view.world_from_view.rotation().inverse() * sun;
        // The bloom's night gate: the planet's shadow on the sun as seen from the camera, as the
        // composite's key light (`key_light.wesl`); no atmosphere: ungated.
        let sun_seen = atmosphere.map_or(1.0, |a| {
            let planet_center = a.world_to_atmosphere.inverse().transform_point3(Vec3::ZERO);
            core::sun_seen(view.world_from_view.translation() - planet_center, a.inner_radius, sun)
        });
        let mut params = ShaftsViewParams::new(&settings, view.clip_from_view, toward_sun_view, sun_seen);
        // Nothing is masked until the passes can write the occlusion (an unwritten target would
        // darken the haze for the frames the pipelines compile).
        if !ready(&occlusion_pipes(msaa.samples())) {
            params.darkness = 0.0;
        }
        if !(bloom_camera && ready(&bloom_pipes(msaa.samples()))) {
            params.bloom = 0.0;
        }
        let sun_uv = params.sun_uv.unwrap_or(Vec2::splat(0.5));
        let mut passes = DynamicUniformBuffer::<ShaftsPassUniform>::default();
        let base = ShaftsPassUniform { sun_uv, step: 0.0, taps: params.taps, full_size: full.as_vec2(), depth_m: settings.occlusion_depth_m };
        let mut offsets = vec![passes.push(&base)];
        for k in 0..params.passes {
            offsets.push(passes.push(&ShaftsPassUniform { step: core::pass_step(k, params.passes, params.taps, settings.length), ..base }));
        }
        passes.write_buffer(&device, &queue);
        let mut occlusion = UniformBuffer::from(GpuAtmosphereLightShafts { light_dir: sun, darkness: params.darkness, lobe_g: settings.lobe_g });
        occlusion.write_buffer(&device, &queue);
        let a = crate::render::cached(&mut textures, &device, "shafts_a", half, MASK);
        let b = crate::render::cached(&mut textures, &device, "shafts_b", half, MASK);
        let bloom = params.bloom_active().then(|| {
            #[allow(clippy::cast_precision_loss)]
            let mut uniform = UniformBuffer::from(ShaftsBloomUniform {
                tint: settings.bloom_tint.max(Vec3::ZERO) * params.bloom,
                threshold: settings.bloom_threshold,
                max_luminance: settings.bloom_max,
                radius: settings.bloom_radius,
                aspect: full.x as f32 / full.y as f32,
            });
            uniform.write_buffer(&device, &queue);
            let a = crate::render::cached(&mut textures, &device, "shafts_bloom_a", half, BLOOM);
            let b = crate::render::cached(&mut textures, &device, "shafts_bloom_b", half, BLOOM);
            BloomView { textures: (a, b), uniform }
        });
        let shafts = ShaftsView { params, textures: (a, b), passes, offsets, samples: msaa.samples(), occlusion, bloom };
        let Some(uniform) = shafts.occlusion.buffer().cloned() else { continue };
        let vendored = AtmosphereLightShafts { occlusion: shafts.result().default_view.clone(), uniform };
        // With the sun-visibility pass on, it owns the aerial hook (`sun_visibility.rs`).
        if sun_vis {
            commands.entity(entity).insert(shafts);
        } else {
            commands.entity(entity).insert((shafts, vendored));
        }
    }
}

fn prepare_bind_groups(
    mut commands: Commands,
    views: Query<(Entity, &ShaftsView, &ViewDepthStencilTexture)>,
    pipelines: Option<Res<ShaftsPipelines>>,
    cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    view_uniforms: Res<ViewUniforms>,
) {
    let Some(pipelines) = pipelines else { return };
    let Some(view_binding) = view_uniforms.uniforms.binding() else { return };
    for (entity, shafts, depth) in &views {
        let (Some(depth_view), Some(pass_binding)) = (depth.attachment.depth_stencil_views().depth_only_view(), shafts.passes.binding()) else { continue };
        let layout = if shafts.samples > 1 { &pipelines.mask_msaa } else { &pipelines.mask };
        let mask = device.create_bind_group("shafts_mask", &cache.get_bind_group_layout(layout), &BindGroupEntries::sequential((pass_binding.clone(), view_binding.clone(), depth_view)));
        // Blur pass k reads `a` (k even) or `b`.
        let blurs = |label, textures: &(CachedTexture, CachedTexture)| -> Vec<BindGroup> {
            (0..shafts.params.passes)
                .map(|k| {
                    let source = if k % 2 == 0 { &textures.0 } else { &textures.1 };
                    device.create_bind_group(
                        label,
                        &cache.get_bind_group_layout(&pipelines.blur),
                        &BindGroupEntries::sequential((pass_binding.clone(), &source.default_view, &pipelines.sampler)),
                    )
                })
                .collect()
        };
        let blur = blurs("shafts_blur", &shafts.textures);
        let bloom = shafts.bloom.as_ref().and_then(|bloom| {
            let uniform = bloom.uniform.binding()?;
            let result = ping_pong_result(&bloom.textures, shafts.params.passes);
            let apply = device.create_bind_group(
                "shafts_apply",
                &cache.get_bind_group_layout(&pipelines.apply),
                &BindGroupEntries::sequential((uniform, &result.default_view, &pipelines.sampler)),
            );
            Some((blurs("shafts_bloom_blur", &bloom.textures), apply))
        });
        commands.entity(entity).insert(ShaftsBindGroups { mask, blur, bloom });
    }
}

/// One fullscreen draw of `pipeline` into `target` (kept, then stored).
fn draw(ctx: &mut RenderContext, label: &'static str, target: &TextureView, pipeline: &RenderPipeline, group: &BindGroup, offsets: &[u32]) {
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: target,
            depth_slice: None,
            resolve_target: None,
            ops: Operations { load: LoadOp::Load, store: StoreOp::Store },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, group, offsets);
    pass.draw(0..3, 0..1);
}

/// The mask and the blur passes, under one GPU timer `shafts`. Skipped on a view under water
/// (`render_sky` and the composite skip it too) and while nothing is masked.
pub(crate) fn shafts_pass(
    view: ViewQuery<(&ShaftsView, &ShaftsBindGroups, &ViewUniformOffset), Without<bevy_pbr::SkyOccluded>>,
    ids: Res<ShaftsPipelineIds>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (shafts, groups, view_offset) = view.into_inner();
    if !shafts.params.active() {
        return;
    }
    let (Some(mask), Some(blur)) = (ids.get(&cache, ShaftsPipe::Mask(shafts.samples)), ids.get(&cache, ShaftsPipe::Blur)) else { return };
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let span = diagnostics.time_span(ctx.command_encoder(), "shafts");
    draw(&mut ctx, "shafts_mask", &shafts.textures.0.default_view, mask, &groups.mask, &[shafts.offsets[0], view_offset.offset]);
    for (k, group) in groups.blur.iter().enumerate() {
        let target = if k % 2 == 0 { &shafts.textures.1 } else { &shafts.textures.0 };
        draw(&mut ctx, "shafts_blur", &target.default_view, blur, group, &[shafts.offsets[k + 1]]);
    }
    span.end(ctx.command_encoder());
}

type BloomViewItem = (
    &'static ShaftsView,
    &'static ShaftsBindGroups,
    &'static ViewUniformOffset,
    &'static ViewTarget,
    &'static ViewDepthStencilTexture,
    &'static ExtractedCamera,
    Option<&'static MainPassResolutionOverride>,
);

/// The light shaft bloom (S3b), under one GPU timer `shafts_bloom`: the source from the view's
/// scene colour (after the clouds composite: `render::finish` orders it), the radial blurs, and
/// the additive apply into the view. Only on a [`ShaftsBloomCamera`] (`sky.shafts.enabled` and
/// `sky.shafts.bloom`); skipped under water and while nothing is added.
pub(crate) fn bloom_pass(
    view: ViewQuery<BloomViewItem, (With<ShaftsBloomCamera>, Without<bevy_pbr::SkyOccluded>)>,
    ids: Res<ShaftsPipelineIds>,
    pipelines: Option<Res<ShaftsPipelines>>,
    cache: Res<PipelineCache>,
    view_uniforms: Res<ViewUniforms>,
    mut ctx: RenderContext,
) {
    let (shafts, groups, view_offset, target, depth, camera, resolution) = view.into_inner();
    let (Some(pipelines), Some(bloom), Some((blur_groups, apply_group))) = (pipelines, shafts.bloom.as_ref(), groups.bloom.as_ref()) else { return };
    if !shafts.params.bloom_active() {
        return;
    }
    let samples = shafts.samples;
    let (Some(source), Some(blur), Some(apply)) =
        (ids.get(&cache, ShaftsPipe::BloomSource(samples)), ids.get(&cache, ShaftsPipe::BloomBlur), ids.get(&cache, ShaftsPipe::Apply(samples)))
    else {
        return;
    };
    let (Some(view_binding), Some(pass_binding), Some(bloom_binding), Some(depth_view)) =
        (view_uniforms.uniforms.binding(), shafts.passes.binding(), bloom.uniform.binding(), depth.attachment.depth_stencil_views().depth_only_view())
    else {
        return;
    };
    let layout = if samples > 1 { &pipelines.bloom_source_msaa } else { &pipelines.bloom_source };
    // Made here, not in `PrepareBindGroups`: the view's main texture is the one the main passes
    // drew into this frame only from the graph's point of view.
    let source_group = ctx.render_device().create_bind_group(
        "shafts_bloom_source",
        &cache.get_bind_group_layout(layout),
        &BindGroupEntries::sequential((pass_binding, view_binding, depth_view, bloom_binding, target.main_texture_view())),
    );
    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let span = diagnostics.time_span(ctx.command_encoder(), "shafts_bloom");
    draw(&mut ctx, "shafts_bloom_source", &bloom.textures.0.default_view, source, &source_group, &[shafts.offsets[0], view_offset.offset]);
    for (k, group) in blur_groups.iter().enumerate() {
        let to = if k % 2 == 0 { &bloom.textures.1 } else { &bloom.textures.0 };
        draw(&mut ctx, "shafts_bloom_blur", &to.default_view, blur, group, &[shafts.offsets[k + 1]]);
    }
    {
        let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
            label: Some("shafts_apply"),
            color_attachments: &[Some(target.get_color_attachment())],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        // The view's internal (DLSS) resolution, as the composite.
        if let Some(v) = Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution) {
            #[allow(clippy::cast_precision_loss)]
            pass.set_viewport(v.physical_position.x as f32, v.physical_position.y as f32, v.physical_size.x as f32, v.physical_size.y as f32, v.depth.start, v.depth.end);
        }
        pass.set_pipeline(apply);
        pass.set_bind_group(0, apply_group, &[]);
        pass.draw(0..3, 0..1);
    }
    span.end(ctx.command_encoder());
}
