//! [`GroundcoverMaterial`]: the model's `StandardMaterial` (alpha mask, forward) extended by the groundcover vertex
//! stages (`shaders/groundcover.wesl`, `shaders/groundcover_prepass.wesl`) and a tinting fragment stage.
//!
//! - **Bend**: height-weighted (quadratic in height over the species' bend height), driven by `sf_wind_bevy`'s
//!   field at the instance root (`wind_sample_gusty`, its gustiness raised to `groundcover.render.gust_contrast`), the
//!   UE `SimpleGrassWind` / `OpenMW` groundcover pattern; each instance samples it offset along the wind by its tag's
//!   wind phase. On top, detail bending per instance (Crysis vegetation, GPU Gems 3 ch. 16; Ghost of Tsushima's
//!   global noise + per-blade sine): an along-wind sway of the speed, a slower cross-wind sway and a fast cross-wind
//!   flutter with a per-vertex phase, all scaled by the wind speed, phased by the tag, timed by `globals.time`.
//!   Frequencies are whole cycles per hour ([`quantise_hz`]), so the hourly wrap of `globals.time` is seamless.
//! - **Motion vectors**: the prepass evaluates the bend twice, with this frame's field and with last frame's
//!   (`wind_sample_prev_gusty`, re-expressed around the current origin by `sf_wind_bevy`) at last frame's time
//!   (`globals.time - globals.delta_time`), so `previous_world_position`
//!   carries the plant's own motion (the `bevy_feronia` pattern; the previous wind state lives in the wind buffer).
//! - **Tag** (`sf_groundcover::Tag`): wind phase, tint position between the species' two tints, fade seed, batch
//!   slot (the species' row in this material's `species` table).
//! - **Fade**: each instance shrinks out at its own seeded distance in the band before the range end, by the
//!   camera view's position. The casting batch turns it off (its tiles are inside the shadow range, short of the
//!   band), so shadow views never fade a caster by the light's position.
//!
//! Non-bindless (one bind group per material, as `sf_rockgen_bevy`): a groundcover set has a handful of models.

use bevy_asset::{Asset, Handle};
use bevy_math::Vec4;
use bevy_mesh::MeshVertexBufferLayoutRef;
use bevy_pbr::{ExtendedMaterial, MATERIAL_BIND_GROUP_INDEX, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy_render::storage::ShaderBuffer;
use bevy_shader::ShaderRef;

use crate::residency::MAX_SPECIES_PER_BATCH;
use crate::tunables as t;

/// The main-pass stages (vertex + tinting fragment).
pub const MAIN_SHADER: &str = "embedded://sf_groundcover_bevy/shaders/groundcover.wesl";
/// The prepass / shadow vertex stage.
pub const PREPASS_SHADER: &str = "embedded://sf_groundcover_bevy/shaders/groundcover_prepass.wesl";

/// Material binding of [`GroundcoverWind::params`].
pub const PARAMS_BINDING: u32 = 100;
/// Material binding of [`GroundcoverWind::wind`] (`SF_WIND_BINDING`).
pub const WIND_BINDING: u32 = 101;

/// One species row of [`GroundcoverParams::species`] (`groundcover_common.wesl` `GroundcoverSpecies`).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct GroundcoverSpeciesGpu {
    /// Tint at tag position 0 (rgb), wind response factor (w).
    pub tint_a: Vec4,
    /// Tint at tag position 255 (rgb), bend height in metres (w: the model's height × the mean scale).
    pub tint_b: Vec4,
}

/// The extension's uniform (`groundcover_common.wesl` `GroundcoverParams`: same fields, same order).
#[derive(Clone, Copy, Debug, PartialEq, ShaderType)]
pub struct GroundcoverParams {
    /// x: range end (m), y: fade band width (m), z: shrink distance (m), w: 1 = fade on.
    pub fade: Vec4,
    /// x: wind scale, y: tip displacement per m/s (fraction of height), z: phase sample offset (m), w: minimum
    /// gustiness (gust contrast).
    pub wind: Vec4,
    /// x: sway amount, y: sway Hz, z: flutter amount, w: flutter Hz (Hz quantised by [`quantise_hz`]).
    pub sway: Vec4,
    /// x: cross-wind sway Hz (quantised), yzw: unused.
    pub sway_cross: Vec4,
    /// Trample (GC-M3, [`crate::trample`]): x the largest tilt under a full press (rad), y the scale on the press
    /// (0 = off), zw unused.
    pub trample: Vec4,
    pub species: [GroundcoverSpeciesGpu; MAX_SPECIES_PER_BATCH],
}

impl Default for GroundcoverParams {
    fn default() -> Self {
        Self {
            fade: Vec4::ZERO,
            wind: Vec4::ZERO,
            sway: Vec4::ZERO,
            sway_cross: Vec4::ZERO,
            trample: Vec4::ZERO,
            species: [GroundcoverSpeciesGpu::default(); MAX_SPECIES_PER_BATCH],
        }
    }
}

/// The live wind values (`groundcover.render.wind_scale`, `bend_per_mps`, `phase_offset_m`, `gust_contrast`,
/// `sway_amount`, `sway_hz`, `flutter_amount`, `flutter_hz`) and the trample's (`trample_max_deg`, `trample_amount`;
/// GC-M3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindSettings {
    pub scale: f32,
    pub bend_per_mps: f32,
    pub phase_offset_m: f32,
    pub gust_contrast: f32,
    pub sway_amount: f32,
    pub sway_hz: f32,
    pub flutter_amount: f32,
    pub flutter_hz: f32,
    pub trample_max_deg: f32,
    pub trample_amount: f32,
}

/// Seconds after which Bevy's `globals.time` wraps to 0 (`bevy_render` `globals.wesl`: "Wraps to 0 after 1 hour").
// tunables: const (Bevy's fixed wrap period of `globals.time`, a fact of the engine)
pub const TIME_WRAP_S: f32 = 3600.0;

/// The cross-wind sway's frequency as a fraction of the along-wind sway's: slower, and incommensurate with it so the
/// two never line up into a fixed figure.
// tunables: const (decorrelation ratio; the cross sway's speed is tuned through `groundcover.render.sway_hz`)
pub const CROSS_SWAY_RATIO: f32 = 0.63;

/// `hz` rounded to a whole number of cycles per [`TIME_WRAP_S`], so a sine of `globals.time` has no jump at the wrap
/// and `time - delta_time` stays right across it. Within `1 / (2 × TIME_WRAP_S)` Hz of the request.
pub fn quantise_hz(hz: f32) -> f32 {
    (hz * TIME_WRAP_S).round() / TIME_WRAP_S
}

impl Default for WindSettings {
    fn default() -> Self {
        Self {
            scale: t::WIND_SCALE.default_value(),
            bend_per_mps: t::BEND_PER_MPS.default_value(),
            phase_offset_m: t::PHASE_OFFSET.default_value(),
            gust_contrast: t::GUST_CONTRAST.default_value(),
            sway_amount: t::SWAY_AMOUNT.default_value(),
            sway_hz: t::SWAY_HZ.default_value(),
            flutter_amount: t::FLUTTER_AMOUNT.default_value(),
            flutter_hz: t::FLUTTER_HZ.default_value(),
            trample_max_deg: t::TRAMPLE_MAX_DEG.default_value(),
            trample_amount: t::TRAMPLE_AMOUNT.default_value(),
        }
    }
}

impl WindSettings {
    /// Writes the wind, sway, cross-sway and trample fields of `params` (frequencies quantised by [`quantise_hz`]).
    pub fn apply(self, params: &mut GroundcoverParams) {
        params.wind = Vec4::new(self.scale, self.bend_per_mps, self.phase_offset_m, self.gust_contrast);
        params.sway = Vec4::new(self.sway_amount, quantise_hz(self.sway_hz), self.flutter_amount, quantise_hz(self.flutter_hz));
        params.sway_cross = Vec4::new(quantise_hz(CROSS_SWAY_RATIO * self.sway_hz), 0.0, 0.0, 0.0);
        params.trample = Vec4::new(self.trample_max_deg.to_radians(), self.trample_amount, 0.0, 0.0);
    }
}

/// The groundcover half of [`GroundcoverMaterial`].
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
pub struct GroundcoverWind {
    #[uniform(100)]
    pub params: GroundcoverParams,
    /// `sf_wind_bevy::WindBuffer` (or a zeroed stand-in without a wind plugin: no bend).
    #[storage(101, read_only)]
    pub wind: Handle<ShaderBuffer>,
}

impl MaterialExtension for GroundcoverWind {
    fn vertex_shader() -> ShaderRef {
        MAIN_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        MAIN_SHADER.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        PREPASS_SHADER.into()
    }

    /// Never used while the base is forward (the plugin forces `OpaqueRendererMethod::Forward`); kept anyway.
    fn deferred_vertex_shader() -> ShaderRef {
        PREPASS_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // The material group index is 3.
        #[allow(clippy::cast_possible_truncation)]
        let defs = sf_wind_bevy::wind_shader_defs(MATERIAL_BIND_GROUP_INDEX as u32, WIND_BINDING);
        descriptor.vertex.shader_defs.extend(defs.iter().cloned());
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.extend(defs);
        }
        Ok(())
    }
}

/// A groundcover batch's material.
pub type GroundcoverMaterial = ExtendedMaterial<StandardMaterial, GroundcoverWind>;
