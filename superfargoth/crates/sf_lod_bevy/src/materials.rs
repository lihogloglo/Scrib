//! The custom materials and the meshes they draw. Proxies are Bevy's
//! `StandardMaterial` (vertex colors) extended only by the far shadow map
//! ([`ProxyMaterial`]); impostors and light sprites exist where Bevy has
//! nothing.

use bevy_asset::{Asset, Handle, RenderAssetUsages};
use bevy_image::Image;
use bevy_material::{AlphaMode, OpaqueRendererMethod};
use bevy_math::{UVec4, Vec3, Vec4};
use bevy_mesh::{Indices, Mesh, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat};
use bevy_pbr::{ExtendedMaterial, Material, MaterialExtension, MaterialPipeline, MaterialPipelineKey, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy_shader::ShaderRef;
use sf_lod::{ImpostorInstance, LightPoint};

pub const IMPOSTOR_SHADER: &str = "embedded://sf_lod_bevy/shaders/impostor.wesl";
pub const IMPOSTOR_PREPASS_SHADER: &str = "embedded://sf_lod_bevy/shaders/impostor_prepass.wesl";
pub const LIGHT_SPRITE_SHADER: &str = "embedded://sf_lod_bevy/shaders/light_sprite.wesl";
pub const PROXY_SHADER: &str = "embedded://sf_lod_bevy/shaders/proxy.wesl";

/// Per impostor vertex: (radius × scale, yaw, atlas layer, 0).
pub const ATTRIBUTE_IMPOSTOR: MeshVertexAttribute = MeshVertexAttribute::new("Impostor_Data", 988_540_917, VertexFormat::Float32x4);
/// Per light-sprite vertex: (radius m, 0, 0, 0).
pub const ATTRIBUTE_SPRITE: MeshVertexAttribute = MeshVertexAttribute::new("Sprite_Data", 988_540_918, VertexFormat::Float32x4);

/// `impostor_common.wesl`'s `ImpostorParams`.
#[derive(Clone, Copy, Debug, ShaderType)]
pub struct ImpostorParams {
    /// Frames per side, mode (0 hemi, 1 full), frame texels, unused.
    pub grid: UVec4,
    /// Debug tint (rgb) and alpha cutoff (a).
    pub tint: Vec4,
    /// The far shadow map: (min x, min z, 1 / extent, base height), render
    /// space; extent 0 = none ([`crate::FarShadowInput`]).
    pub far: Vec4,
}

/// Octahedral impostors of one worldspace: every atlas in two texture
/// arrays (one layer per model).
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct ImpostorMaterial {
    #[uniform(0)]
    pub params: ImpostorParams,
    /// sRGB albedo + coverage.
    #[texture(1, dimension = "2d_array")]
    #[sampler(2)]
    pub albedo: Handle<Image>,
    /// Model-space normal + depth.
    #[texture(3, dimension = "2d_array")]
    pub normal_depth: Handle<Image>,
    /// The far shadow map (`Rg16Float`; the default image until there is one).
    #[texture(4)]
    #[sampler(5)]
    pub far_map: Handle<Image>,
}

impl Material for ImpostorMaterial {
    fn vertex_shader() -> ShaderRef {
        IMPOSTOR_SHADER.into()
    }
    fn fragment_shader() -> ShaderRef {
        IMPOSTOR_SHADER.into()
    }
    fn prepass_vertex_shader() -> ShaderRef {
        IMPOSTOR_PREPASS_SHADER.into()
    }
    fn prepass_fragment_shader() -> ShaderRef {
        IMPOSTOR_PREPASS_SHADER.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Mask(self.params.tint.w)
    }
    fn opaque_render_method(&self) -> OpaqueRendererMethod {
        // The deferred path would need a G-buffer writer; impostors are
        // shaded in the forward pass on every render path.
        OpaqueRendererMethod::Forward
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            ATTRIBUTE_IMPOSTOR.at_shader_location(2),
        ])?];
        Ok(())
    }
}

/// Proxies: Bevy's `StandardMaterial` (vertex colors carry albedo; forward
/// path) plus the terrain's far shadow (`proxy.wesl`). One extension type,
/// so proxies still batch with each other.
pub type ProxyMaterial = ExtendedMaterial<StandardMaterial, FarShadowExt>;

/// The far shadow map binding of [`ProxyMaterial`].
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct FarShadowExt {
    /// (min x, min z, 1 / extent, base height), render space; extent 0 = none.
    #[uniform(100)]
    pub far: Vec4,
    #[texture(101)]
    #[sampler(102)]
    pub map: Handle<Image>,
}

impl MaterialExtension for FarShadowExt {
    fn fragment_shader() -> ShaderRef {
        PROXY_SHADER.into()
    }
}

/// `light_sprite.wesl`'s `SpriteParams`.
#[derive(Clone, Copy, Debug, ShaderType)]
pub struct SpriteParams {
    /// Night factor, minimum size (px), world size per radius meter,
    /// brightness scale.
    pub p: Vec4,
    /// Debug tint (rgb).
    pub tint: Vec4,
}

/// Additive camera-facing light glows.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct LightSpriteMaterial {
    #[uniform(0)]
    pub params: SpriteParams,
    /// [`sf_render_order_bevy::TranslucentFog::params`]: the glows fade with the
    /// fog's transmittance (render-order.md rule 1, RO-2; additive: no in-scatter).
    #[uniform(1)]
    pub fog_params: Vec4,
    /// [`sf_render_order_bevy::TranslucentFog::volume`].
    #[texture(2, dimension = "3d")]
    #[sampler(3)]
    pub fog_volume: Handle<Image>,
}

impl Material for LightSpriteMaterial {
    fn vertex_shader() -> ShaderRef {
        LIGHT_SPRITE_SHADER.into()
    }
    fn fragment_shader() -> ShaderRef {
        LIGHT_SPRITE_SHADER.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
    fn enable_prepass() -> bool {
        false
    }
    fn enable_shadows() -> bool {
        false
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
            ATTRIBUTE_SPRITE.at_shader_location(3),
        ])?];
        Ok(())
    }
}

const CORNERS: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];

fn quads(n: usize) -> Indices {
    let mut indices = Vec::with_capacity(n * 6);
    for i in 0..n {
        let b = u32::try_from(i * 4).expect("quad count fits u32");
        indices.extend([b, b + 1, b + 2, b, b + 2, b + 3]);
    }
    Indices::U32(indices)
}

/// Bounds of an instanced mesh: `(min, max)`.
pub type Bounds = (Vec3, Vec3);

/// The impostor batch of a node: four vertices per instance. `layer` maps
/// a node-local model index to its atlas layer and capture radius.
pub fn impostor_mesh(instances: &[ImpostorInstance], layer: &dyn Fn(u16) -> Option<(u32, f32)>) -> Option<(Mesh, Bounds)> {
    let mut positions = Vec::with_capacity(instances.len() * 4);
    let mut corners = Vec::with_capacity(instances.len() * 4);
    let mut data = Vec::with_capacity(instances.len() * 4);
    let (mut min, mut max) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    for inst in instances {
        let Some((layer, radius)) = layer(inst.model) else { continue };
        let r = radius * inst.scale;
        let c = Vec3::from_array(inst.pos);
        min = min.min(c - r);
        max = max.max(c + r);
        #[allow(clippy::cast_precision_loss)]
        let d = [r, inst.yaw, layer as f32, 0.0];
        for corner in CORNERS {
            positions.push(inst.pos);
            corners.push(corner);
            data.push(d);
        }
    }
    if positions.is_empty() {
        return None;
    }
    let n = positions.len() / 4;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, corners);
    mesh.insert_attribute(ATTRIBUTE_IMPOSTOR, data);
    mesh.insert_indices(quads(n));
    Some((mesh, (min, max)))
}

/// The light-sprite batch of a node.
pub fn light_mesh(lights: &[LightPoint]) -> Option<(Mesh, Bounds)> {
    if lights.is_empty() {
        return None;
    }
    let mut positions = Vec::with_capacity(lights.len() * 4);
    let mut corners = Vec::with_capacity(lights.len() * 4);
    let mut colors = Vec::with_capacity(lights.len() * 4);
    let mut data = Vec::with_capacity(lights.len() * 4);
    let (mut min, mut max) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
    for l in lights {
        let c = Vec3::from_array(l.pos);
        min = min.min(c - l.radius);
        max = max.max(c + l.radius);
        for corner in CORNERS {
            positions.push(l.pos);
            corners.push(corner);
            colors.push([l.color[0], l.color[1], l.color[2], l.intensity]);
            data.push([l.radius, 0.0, 0.0, 0.0]);
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, corners);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(ATTRIBUTE_SPRITE, data);
    mesh.insert_indices(quads(lights.len()));
    Some((mesh, (min, max)))
}
