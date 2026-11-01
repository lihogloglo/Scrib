//! superfargoth patch (own, GI): DDGI probe-volume bindings in the mesh view bind group.
//!
//! A view with a [`GiProbeVolumeView`] component gets [`MeshPipelineViewLayoutKey::GI_PROBES`]
//! (and the `GI_PROBES` shader def): bindings 39-42 of the mesh view bind group hold the
//! volume's octahedral irradiance and visibility atlases, a linear clamp sampler and a
//! [`GiProbeVolumeUniform`]. `gi_probes.wesl` samples them in `apply_pbr_lighting` (eight probes,
//! trilinear x back-face x Chebyshev visibility, Majercik et al. 2019) in place of the
//! environment map's diffuse inside the volume, and scale the flat ambient light there by the
//! uniform's ambient scale. The probe system (superfargoth `sf_gi_bevy`)
//! owns the textures and inserts the component on its cameras' render-world view entities; it
//! is the only coupling. The bindings travel with the component, so a view's layout key and its
//! bind group can never disagree.
//!
//! [`MeshPipelineViewLayoutKey::GI_PROBES`]: crate::MeshPipelineViewLayoutKey::GI_PROBES

use bevy_ecs::component::Component;
use bevy_math::{UVec4, Vec4};
use bevy_render::render_resource::{Buffer, Sampler, ShaderType, TextureView};

/// Binding numbers in the mesh view bind group (group 0), after the last stock one (38).
pub const GI_PROBE_IRRADIANCE_BINDING: u32 = 39;
pub const GI_PROBE_VISIBILITY_BINDING: u32 = 40;
pub const GI_PROBE_SAMPLER_BINDING: u32 = 41;
pub const GI_PROBE_UNIFORM_BINDING: u32 = 42;

/// The probe volume's constants, as `GiProbeVolume` in `mesh_view_types.wesl`.
#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub struct GiProbeVolumeUniform {
    /// xyz: the grid's min corner (probe (0,0,0) sits half a spacing inside); w: spacing (m).
    pub origin_spacing: Vec4,
    /// xyz: probes per axis; w: interior texels per irradiance tile side (6).
    pub dims: UVec4,
    /// x: self-shadow bias length (m), y: intensity multiplier, z: interior texels per
    /// visibility tile side (14), w: multiplier on the flat ambient light inside the volume
    /// (0: the probes replace it; `apply_pbr_lighting`).
    pub bias_intensity: Vec4,
}

/// What a view samples: `Rgba16Float` atlases (irradiance rgb; visibility r = mean distance,
/// g = mean squared distance), each probe a bordered octahedral tile at column
/// `x + dims.x * y`, row `z`; a filtering clamp-to-edge sampler; the uniform buffer.
#[derive(Clone)]
pub struct GiProbeBindings {
    pub irradiance: TextureView,
    pub visibility: TextureView,
    pub sampler: Sampler,
    pub uniform: Buffer,
}

/// On a render-world view entity: the view samples this probe volume.
#[derive(Component, Clone)]
pub struct GiProbeVolumeView(pub GiProbeBindings);
