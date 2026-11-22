//! [`GroundcoverMaterial`]: the model's `StandardMaterial` (alpha mask, forward) extended by the groundcover vertex
//! stages (`shaders/groundcover.wesl`, `shaders/groundcover_prepass.wesl`) and a tinting fragment stage.
//!
//! - **Bend**: height-weighted (quadratic in height over the species' bend height), driven by `sf_wind_bevy`'s
//!   field at the instance root (`wind_sample`), the UE `SimpleGrassWind` / `OpenMW` groundcover pattern. The sway comes
//!   from the advected gust field itself; each instance samples it offset along the wind by its tag's wind phase.
//! - **Motion vectors**: the prepass evaluates the bend twice, with this frame's field and with last frame's
//!   (`wind_sample_prev`, re-expressed around the current origin by `sf_wind_bevy`), so `previous_world_position`
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
    /// x: wind scale, y: tip displacement per m/s (fraction of height), z: phase sample offset (m), w: unused.
    pub wind: Vec4,
    pub species: [GroundcoverSpeciesGpu; MAX_SPECIES_PER_BATCH],
}

impl Default for GroundcoverParams {
    fn default() -> Self {
        Self { fade: Vec4::ZERO, wind: Vec4::ZERO, species: [GroundcoverSpeciesGpu::default(); MAX_SPECIES_PER_BATCH] }
    }
}

/// The live wind values (`groundcover.render.wind_scale`, `bend_per_mps`, `phase_offset_m`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindSettings {
    pub scale: f32,
    pub bend_per_mps: f32,
    pub phase_offset_m: f32,
}

impl Default for WindSettings {
    fn default() -> Self {
        Self {
            scale: t::WIND_SCALE.default_value(),
            bend_per_mps: t::BEND_PER_MPS.default_value(),
            phase_offset_m: t::PHASE_OFFSET.default_value(),
        }
    }
}

impl WindSettings {
    pub fn to_gpu(self) -> Vec4 {
        Vec4::new(self.scale, self.bend_per_mps, self.phase_offset_m, 0.0)
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
