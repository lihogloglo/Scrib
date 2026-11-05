//! The terrain material: a custom `Material` whose vertex stage is CDLOD
//! over the height clipmap and whose fragment stage builds Bevy's
//! `PbrInput`, so every Bevy light, shadow, fog and atmosphere path applies.

use bevy_asset::{Asset, Handle};
use bevy_image::Image;
use bevy_mesh::MeshVertexBufferLayoutRef;
use bevy_pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy_render::storage::ShaderBuffer;
use bevy_shader::ShaderRef;

pub const SHADER_PATH: &str = "embedded://sf_terrain_bevy/shaders/terrain.wesl";
pub const PREPASS_SHADER_PATH: &str = "embedded://sf_terrain_bevy/shaders/terrain_prepass.wesl";

/// Bindings: see `shaders/common.wesl`. One instance per terrain; every
/// patch entity shares it, so all patches batch into one indirect draw per
/// view and mesh (full patch / quadrant).
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
#[bind_group_data(TerrainMaterialKey)]
pub struct TerrainMaterial {
    /// Per-frame parameters (`params.rs`), rewritten in place each frame.
    #[storage(100, read_only)]
    pub params: Handle<ShaderBuffer>,
    /// 10 × 512² `R32Float`, toroidal.
    #[texture(101, dimension = "2d_array", sample_type = "float", filterable = false)]
    pub heights: Handle<Image>,
    /// 3 × 512² `Rgba8Uint`: four layer slices per sample.
    #[texture(102, dimension = "2d_array", sample_type = "u_int")]
    pub layer_ids: Handle<Image>,
    /// 3 × 512² `Rgba8Unorm`: their weights.
    #[texture(103, dimension = "2d_array")]
    pub layer_weights: Handle<Image>,
    /// 10 × 512² sRGB macro albedo.
    #[texture(104, dimension = "2d_array")]
    #[sampler(105)]
    pub macro_albedo: Handle<Image>,
    /// Layer texture arrays (bake `terrain.layers`).
    #[texture(106, dimension = "2d_array")]
    #[sampler(107)]
    pub albedo_height: Handle<Image>,
    /// Normal X/Y, AO, roughness per layer.
    #[texture(108, dimension = "2d_array")]
    pub normal_ao_roughness: Handle<Image>,
    /// 10 × 512² `Rg8Snorm` normal X and Z, beside the heights (filtered).
    #[texture(109, dimension = "2d_array")]
    #[sampler(110)]
    pub normals: Handle<Image>,
    /// The far shadow map ([`crate::FarShadowMap`]): the march's stand-in
    /// for distant terrain.
    #[texture(111)]
    #[sampler(112)]
    pub far_map: Handle<Image>,
    /// Surface weather (surface-weather.md W2; scaffold until W3 moves these to mesh-view
    /// bindings through the vendored `bevy_pbr` patch "SW", then they go): the
    /// `SurfaceUniform` slots, the cell window (filtered, repeat), the shelter depth map
    /// (`Depth32Float`, loaded) and the noise atlas. Placeholders until `sf_surface_bevy` hooks
    /// the material and sets [`Self::surface`], which turns on the `SURFACE_WEATHER` shader def.
    #[storage(113, read_only)]
    pub surface_uniform: Handle<ShaderBuffer>,
    #[texture(114)]
    #[sampler(115)]
    pub surface_cells: Handle<Image>,
    #[texture(116, sample_type = "float", filterable = false)]
    pub surface_shelter: Handle<Image>,
    #[texture(117)]
    #[sampler(118)]
    pub surface_noise: Handle<Image>,
    pub surface: bool,
}

/// The pipeline key: whether the surface weather hook is compiled in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TerrainMaterialKey {
    pub surface: bool,
}

impl From<&TerrainMaterial> for TerrainMaterialKey {
    fn from(m: &TerrainMaterial) -> Self {
        Self { surface: m.surface }
    }
}

impl Material for TerrainMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        PREPASS_SHADER_PATH.into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        PREPASS_SHADER_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // The main pass only: the prepass doesn't import the layer.
        if key.bind_group_data.surface
            && let Some(fragment) = descriptor.fragment.as_mut()
        {
            fragment.shader_defs.push("SURFACE_WEATHER".into());
        }
        Ok(())
    }
}
