//! The water draw (water.md "Water pass, refraction, reflections" step 3): an
//! `ExtendedMaterial<StandardMaterial, WaterExt>` in Bevy's transmissive phase (W-ORDER: the
//! base's `specular_transmission` > 0 routes it there; the shader never reads it), drawn after
//! the opaque pass and **depth-writing**, so the sky, aerial perspective, clouds and fog that
//! follow cover it by its depth like land, and later translucents sort against it. It keeps
//! Bevy's view bindings (directional and clustered lights, shadows, the environment map) with
//! the least code (water.md open question 1, decided at W2). Its own vertex shader displaces the
//! clipmap; its fragment shader reads the colour + depth snapshot of the opaque scene
//! (`render.rs`) for refraction and depth colour, and its SSPR image for local reflections. It does not fog itself (render-order.md
//! "Frame graph").

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
    /// Clipmap (base spacing m, quads per ring side, rings, rest level: render-space Y).
    pub clip: Vec4,
    /// (whitecap intensity, contact-foam depth m, refraction strength m, roughness).
    pub foam: Vec4,
    /// (cascades, skirt radius m, displacement fade end m, snapshot valid 0/1).
    pub misc: Vec4,
    /// (swell scale, chop scale, bounded 0/1, shore field on 0/1).
    pub waves: Vec4,
    /// Render-space XZ rectangle (min x, min z, max x, max z) of a bounded body.
    pub bounds: Vec4,
    /// Shore field: render-space XZ of its corner, 1 / its extent (m).
    pub shore: Vec4,
    /// (range m, swell ramp m, chop ramp m, chop floor): `sf_water::ShoreSettings`.
    pub shore2: Vec4,
    /// (time s, shore foam band m, shore normal floor (`sf_water::ShoreSettings::normal_floor`),
    /// shore foam of the body's own look).
    pub misc2: Vec4,
    /// Optics map: render-space XZ of cell (0, 0)'s centre, 1 / cell size, on 0/1.
    pub regions: Vec4,
    /// SSPR (W-SSPR): (wave distortion; texel scale of the reflection image, 1 or 2, 0 = this
    /// surface does not use it; mirrored plane level, render-space Y; 0).
    pub reflection: Vec4,
    /// Shore waves (W-GERSTNER, `sf_water::ShoreWaves`): (ω of the three trains rad/s, the previous
    /// frame's time s, for the motion vectors).
    pub swash: Vec4,
    /// (offshore amplitude m, swash amplitude m, beach slope, band m); amplitude and swash 0 = off.
    pub swash2: Vec4,
    /// (land margin m, variation 0-1, noise origin x, z m: the floating origin modulo
    /// `sf_water::shore_waves::NOISE_PERIOD`).
    pub swash3: Vec4,
    /// A sea-state transition (W-WIND, `ShoreWaves::blend`): (ω of the trains crossfaded from ×3,
    /// weight of `swash.xyz` against them; 1 = none).
    pub swash4: Vec4,
    /// Optics table of the map, 3 per entry: (absorption, whitecaps), (scattering, contact depth),
    /// (roughness, shore foam, 0, 0).
    pub looks: [Vec4; 24],
}

/// The water half of [`WaterMaterial`] (bindings 100-107 and 110-113 of the material group).
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
    /// The opaque scene colour before the water (HDR, pre-exposed; no sky, clouds or fog yet).
    #[texture(104, sample_type = "float", filterable = false)]
    pub snapshot_color: Handle<Image>,
    /// Scene depth (reverse Z, R32F).
    #[texture(105, sample_type = "float", filterable = false)]
    pub snapshot_depth: Handle<Image>,
    /// SSPR of the snapshot (premultiplied colour, confidence in alpha; `render.rs`), linear clamp.
    #[texture(106)]
    #[sampler(107)]
    pub reflection: Handle<Image>,
    /// `sf_water::ShoreField::encode_rgba16f` (`Rgba16Float`), or a 1x1 open-sea texel.
    #[texture(110, visibility(vertex, fragment))]
    #[sampler(111, visibility(vertex, fragment))]
    pub shore: Handle<Image>,
    /// Optics index per map cell (`R8Uint`), or a 1x1 zero.
    #[texture(112, sample_type = "u_int")]
    pub regions: Handle<Image>,
    /// The bed slope (`sf_water::ShoreField::encode_slope_r16f`, `R16Float`; W-GERSTNER-DEPTH) over
    /// the shore field's extent, or a 1x1 beach slope; sampled with the shore sampler.
    #[texture(113, visibility(vertex, fragment))]
    pub shore_bed: Handle<Image>,
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
            reflection: images.reflection.clone(),
            shore: images.shore.clone(),
            shore_bed: images.shore_bed.clone(),
            regions: images.regions.clone(),
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
        // Depth-writing in the transmissive phase: the sky, clouds and fog after it read the
        // water's depth (W-ORDER), later blended surfaces sort against it,
        // and DLSS/TAA read the water's depth.
        if let Some(depth) = descriptor.depth_stencil.as_mut() {
            depth.depth_write_enabled = Some(true);
        }
        Ok(())
    }
}

/// The water draw's material.
pub type WaterMaterial = ExtendedMaterial<StandardMaterial, WaterExt>;
