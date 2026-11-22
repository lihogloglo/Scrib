//! [`RockMaterial`]: Bevy's `StandardMaterial` (forward) extended by a world-axis triplanar projection of one
//! PBR texture set (`shaders/rock.wesl`), so rock meshes need no UVs (rockgen.md "Materials").
//!
//! Non-bindless (one bind group per material, as `sf_lod_bevy`'s proxies): a rock field uses a handful of
//! materials (texture sets × LOD levels), so batching across materials is not worth the bindless layout yet.

use bevy_asset::{Asset, Handle};
use bevy_color::Color;
use bevy_image::Image;
use bevy_material::OpaqueRendererMethod;
use bevy_pbr::{ExtendedMaterial, MaterialExtension, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, ShaderType};
use bevy_shader::ShaderRef;

use crate::tunables as t;

/// The embedded main-pass shader.
pub const ROCK_SHADER: &str = "embedded://sf_rockgen_bevy/shaders/rock.wesl";

// Flag bits of [`RockParams::flags`] (`rock.wesl` FLAG_*).
pub const FLAG_ALBEDO: u32 = 1;
pub const FLAG_NORMAL: u32 = 2;
pub const FLAG_ORM: u32 = 4;
/// The ORM map's R holds occlusion (a packed ORM; a converted specular map's R is 1 and is not read).
pub const FLAG_OCCLUSION: u32 = 8;
/// The normal map's green channel points down (DirectX convention).
pub const FLAG_FLIP_Y: u32 = 16;
pub const FLAG_HEX: u32 = 32;
pub const FLAG_LOD_TINT: u32 = 64;

/// The GPU uniform of [`RockExt`] (`rock.wesl` `RockParams`: same fields, same order).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct RockParams {
    /// 1 / texture tiling (m): texture repeats per meter.
    pub inv_tiling: f32,
    pub blend_sharpness: f32,
    pub hex_contrast: f32,
    pub hex_rotation: f32,
    pub hex_scale: f32,
    pub flags: u32,
    /// The LOD level this material draws (the LOD tint's colour).
    pub lod: u32,
    pub pad: u32,
}

/// The triplanar half of [`RockMaterial`]: the texture set and its parameters.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
pub struct RockExt {
    #[uniform(100)]
    pub params: RockParams,
    #[texture(101)]
    #[sampler(102)]
    pub albedo: Option<Handle<Image>>,
    #[texture(103)]
    pub normal: Option<Handle<Image>>,
    #[texture(104)]
    pub orm: Option<Handle<Image>>,
}

impl MaterialExtension for RockExt {
    fn fragment_shader() -> ShaderRef {
        ROCK_SHADER.into()
    }
}

/// A rock's material.
pub type RockMaterial = ExtendedMaterial<StandardMaterial, RockExt>;

/// One rock texture set, as the adapter resolved it (handles and the factors of the material it would
/// give a static with the same texture, so rock and static shading agree).
#[derive(Clone, Debug)]
pub struct RockTextures {
    pub albedo: Option<Handle<Image>>,
    pub normal: Option<Handle<Image>>,
    pub orm: Option<Handle<Image>>,
    /// The ORM map's R channel is occlusion.
    pub orm_occlusion: bool,
    pub flip_normal_y: bool,
    /// Meters per texture repeat (the terrain layer's tiling, so rocks and ground share texel density).
    pub tiling_m: f32,
    pub base_color: Color,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
}

impl RockTextures {
    /// The maps and factors of a `StandardMaterial` built for the same texture (its UV-space maps are
    /// sampled triplanar instead).
    pub fn from_standard(m: &StandardMaterial, tiling_m: f32) -> Self {
        Self {
            albedo: m.base_color_texture.clone(),
            normal: m.normal_map_texture.clone(),
            orm: m.metallic_roughness_texture.clone(),
            orm_occlusion: m.occlusion_texture.is_some(),
            flip_normal_y: m.flip_normal_map_y,
            tiling_m,
            base_color: m.base_color,
            perceptual_roughness: m.perceptual_roughness,
            metallic: m.metallic,
            reflectance: m.reflectance,
        }
    }
}

/// The live material settings (`rockgen.material.*`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RockMaterialSettings {
    pub blend_sharpness: f32,
    pub hex_tiling: bool,
    pub hex_rotation: f32,
    pub hex_contrast: f32,
    pub hex_scale: f32,
}

impl Default for RockMaterialSettings {
    fn default() -> Self {
        Self {
            blend_sharpness: t::BLEND_SHARPNESS.default_value(),
            hex_tiling: t::HEX_TILING.default_value(),
            hex_rotation: t::HEX_ROTATION.default_value(),
            hex_contrast: t::HEX_CONTRAST.default_value(),
            hex_scale: t::HEX_SCALE.default_value(),
        }
    }
}

impl RockMaterialSettings {
    /// The store's values (a def nobody registered keeps its default).
    pub fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        Self {
            blend_sharpness: crate::read(store, &t::BLEND_SHARPNESS),
            hex_tiling: crate::read(store, &t::HEX_TILING),
            hex_rotation: crate::read(store, &t::HEX_ROTATION),
            hex_contrast: crate::read(store, &t::HEX_CONTRAST),
            hex_scale: crate::read(store, &t::HEX_SCALE),
        }
    }

    /// Writes these settings and the debug tint into `params`, keeping its texture flags and LOD.
    pub fn apply(&self, params: &mut RockParams, lod_tint: bool) {
        params.blend_sharpness = self.blend_sharpness;
        params.hex_rotation = self.hex_rotation;
        params.hex_contrast = self.hex_contrast;
        params.hex_scale = self.hex_scale;
        let set = |flags: u32, bit: u32, on: bool| if on { flags | bit } else { flags & !bit };
        params.flags = set(params.flags, FLAG_HEX, self.hex_tiling);
        params.flags = set(params.flags, FLAG_LOD_TINT, lod_tint);
    }
}

/// The material for `textures` drawn as LOD `lod`.
pub fn rock_material(textures: &RockTextures, settings: &RockMaterialSettings, lod: u32, lod_tint: bool) -> RockMaterial {
    let mut flags = 0;
    for (on, bit) in [
        (textures.albedo.is_some(), FLAG_ALBEDO),
        (textures.normal.is_some(), FLAG_NORMAL),
        (textures.orm.is_some(), FLAG_ORM),
        (textures.orm.is_some() && textures.orm_occlusion, FLAG_OCCLUSION),
        (textures.flip_normal_y, FLAG_FLIP_Y),
    ] {
        if on {
            flags |= bit;
        }
    }
    let mut params = RockParams { inv_tiling: 1.0 / textures.tiling_m.max(1e-3), flags, lod, ..RockParams::default() };
    settings.apply(&mut params, lod_tint);
    ExtendedMaterial {
        base: StandardMaterial {
            base_color: textures.base_color,
            perceptual_roughness: textures.perceptual_roughness,
            metallic: textures.metallic,
            reflectance: textures.reflectance,
            // The deferred path would need a G-buffer writer for the triplanar maps.
            opaque_render_method: OpaqueRendererMethod::Forward,
            ..StandardMaterial::default()
        },
        extension: RockExt { params, albedo: textures.albedo.clone(), normal: textures.normal.clone(), orm: textures.orm.clone() },
    }
}
