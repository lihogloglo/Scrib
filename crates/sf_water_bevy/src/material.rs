//! The water draw (water.md "Water pass, refraction, reflections" step 3): an
//! `ExtendedMaterial<StandardMaterial, WaterExt>` in the transparent phase, sorted with the
//! blended surfaces but **depth-writing** (precipitation and particles sort against it), so it
//! keeps Bevy's view bindings (directional and clustered lights, shadows, the environment map)
//! with the least code (water.md open question 1, decided at W2). Its own vertex shader displaces
//! the clipmap; its fragment shader reads the colour + depth snapshot taken after the sky, clouds
//! and fog (`render.rs`) for refraction and depth colour, then fogs itself (aerial perspective
//! from a copy of Bevy's aerial-view LUT, then the froxels).

use bevy_asset::{Asset, Handle};
use bevy_image::Image;
use bevy_math::Vec4;
use bevy_mesh::MeshVertexBufferLayoutRef;
use bevy_pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy_shader::ShaderRef;

use crate::WaterImages;

const SHADER: &str = "embedded://sf_water_bevy/shaders/water_material.wesl";

/// Everything the water shaders need besides textures (`water_common.wesl` `WaterParams`).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct WaterParams {
    /// Origin phase (m) of cascades 0 (xy) and 1 (zw): `sf_water::origin_phase`.
    pub phase01: Vec4,
    /// Origin phase of cascades 2 and 3.
    pub phase23: Vec4,
    /// Cascade tile sizes, metres (0 = unused).
    pub sizes: Vec4,
    /// Shortest wavelength each cascade carries, metres (vertex LOD fade).
    pub lambda_min: Vec4,
    /// Absorption per metre (rgb).
    pub absorption: Vec4,
    /// Scattering per metre (rgb).
    pub scattering: Vec4,
    /// Air extinction per metre (rgb) for the aerial transmittance, w = the aerial-view LUT's
    /// range in metres (0: no LUT, no aerial perspective).
    pub aerial: Vec4,
    /// Froxel volume (near, far, slices, enabled): `sf_atmos_bevy::FroxelVolume::params`.
    pub froxel: Vec4,
    /// Clipmap (base spacing m, quads per ring side, rings, rest level: render-space Y).
    pub clip: Vec4,
    /// (whitecap intensity, contact-foam depth m, refraction strength m, roughness).
    pub foam: Vec4,
    /// (cascades, skirt radius m, displacement fade end m, snapshot valid 0/1).
    pub misc: Vec4,
}

/// The water half of [`WaterMaterial`] (bindings 100-111 of the material group).
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct WaterExt {
    #[uniform(100)]
    pub params: WaterParams,
    /// Per cascade (array layer): (Dx, h, Dz, foam).
    #[texture(101, dimension = "2d_array", visibility(vertex, fragment))]
    #[sampler(102, visibility(vertex, fragment))]
    pub displacement: Handle<Image>,
    /// Per cascade: (∂h/∂x, ∂h/∂z, ∂Dx/∂x, ∂Dz/∂z).
    #[texture(103, dimension = "2d_array")]
    pub slopes: Handle<Image>,
    /// Scene colour after the sky and fog (HDR, pre-exposed).
    #[texture(104, sample_type = "float", filterable = false)]
    pub snapshot_color: Handle<Image>,
    /// Scene depth (reverse Z, R32F).
    #[texture(105, sample_type = "float", filterable = false)]
    pub snapshot_depth: Handle<Image>,
    #[texture(106, dimension = "3d")]
    #[sampler(107)]
    pub froxels: Handle<Image>,
    /// Copy of Bevy's aerial-view LUT (log in-scatter).
    #[texture(108, dimension = "3d")]
    #[sampler(109)]
    pub aerial: Handle<Image>,
}

impl WaterExt {
    #[must_use]
    pub fn new(images: &WaterImages, params: WaterParams) -> Self {
        Self {
            params,
            displacement: images.displacement.clone(),
            slopes: images.slopes.clone(),
            snapshot_color: images.snapshot_color.clone(),
            snapshot_depth: images.snapshot_depth.clone(),
            froxels: images.froxels.clone(),
            aerial: images.aerial.clone(),
        }
    }
}

impl MaterialExtension for WaterExt {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Waves show their back faces at grazing angles (and from below, W5).
        descriptor.primitive.cull_mode = None;
        // Depth-writing in the transparent phase: later blended surfaces sort against the water,
        // and DLSS/TAA read the water's depth.
        if let Some(depth) = descriptor.depth_stencil.as_mut() {
            depth.depth_write_enabled = Some(true);
        }
        Ok(())
    }
}

/// The water draw's material.
pub type WaterMaterial = ExtendedMaterial<StandardMaterial, WaterExt>;
