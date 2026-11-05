//! The cover layer array (surface-weather.md "GPU resources", slice W5a): the snow and ash looks
//! of the surface camera's worldspace, packed into one `Rgba8Unorm` 2D array (repeat, mips,
//! trilinear + anisotropic sampler) at mesh-view binding 50 (sampler 51) of the vendored
//! `bevy_pbr` patch "SW".
//!
//! Slices, in [`CoverChannel::ALL`] order, two per channel: `2c` albedo (sRGB-encoded bytes:
//! the shader decodes after sampling, one format for the whole array) with height in alpha,
//! `2c + 1` normal X, Y (tangent space, +Y up; Z reconstructed), roughness, AO. 512² with the full
//! mip chain (`sf_terrain::bake::mip_chain`, the terrain layer arrays' filter). With no layer
//! at all the array is a 1×1, one-slice neutral placeholder; a channel without a layer has
//! neutral slices and `1 / tiling` 0 in its uniform slot ([`cover_slots`]), which the layer
//! reads as "no cover look".
//!
//! The host (the game adapter) decodes the textures it picked ([`CoverLayers`]: the record and
//! RGBA8 images; it knows its formats); this crate never reads files.

use bevy_asset::{Assets, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_math::Vec4;
use bevy_render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
pub use sf_surface::cover::{CoverChannel, SurfaceLayer};
pub use sf_terrain::bake::LayerImage;
use sf_terrain::bake::{MapKind, mip_chain};

use crate::{SurfaceGpu, SurfaceStats, SurfaceUniform};

/// Edge of a cover slice's top mip.
pub const COVER_SIZE: u32 = 512;
/// Slices per channel (albedo + height; normal XY + roughness + AO).
pub const SLICES_PER_CHANNEL: u32 = 2;

/// Neutral texels: mid grey with a flat height, a flat normal with roughness 0.8 and AO 1.
pub const NEUTRAL_ALBEDO: [u8; 4] = [128, 128, 128, 255];
pub const NEUTRAL_NRAO: [u8; 4] = [128, 128, 204, 255];

/// One channel's look as the host decoded it: the record and its RGBA8 images (rows top to
/// bottom; `albedo_height` sRGB). `normal` and `arm` absent: flat, and the record's roughness
/// with AO 1.
#[derive(Clone, Debug, PartialEq)]
pub struct CoverSource {
    pub layer: SurfaceLayer,
    pub albedo_height: LayerImage,
    pub normal: Option<LayerImage>,
    pub arm: Option<LayerImage>,
}

/// The active cover looks of the surface camera's worldspace, in [`CoverChannel::ALL`] order
/// (the host inserts it; the array is repacked when it changes).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct CoverLayers(pub [Option<CoverSource>; 2]);

/// A packed array: `slices` × the full mip chain of `size`², slice-major (each slice's mips
/// in a row, Bevy's default data order).
#[derive(Clone, Debug, PartialEq)]
pub struct CoverArray {
    pub size: u32,
    pub mips: u32,
    pub slices: u32,
    pub data: Vec<u8>,
}

fn one(texel: [u8; 4]) -> LayerImage {
    LayerImage { width: 1, height: 1, rgba: texel.to_vec() }
}

/// Packs the layers into the array (CPU, once per change: ~4 × 512² mip chains).
pub fn pack(layers: &[Option<CoverSource>; 2]) -> CoverArray {
    if layers.iter().all(Option::is_none) {
        return CoverArray { size: 1, mips: 1, slices: 1, data: NEUTRAL_ALBEDO.to_vec() };
    }
    let size = COVER_SIZE;
    let mut data = Vec::new();
    for source in layers {
        let Some(s) = source else {
            data.extend(mip_chain(&one(NEUTRAL_ALBEDO), size, MapKind::Linear));
            data.extend(mip_chain(&one(NEUTRAL_NRAO), size, MapKind::Linear));
            continue;
        };
        data.extend(mip_chain(&s.albedo_height, size, MapKind::Albedo));
        let normal = mip_chain(s.normal.as_ref().unwrap_or(&one([128, 128, 255, 255])), size, MapKind::Normal);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let rough = (s.layer.roughness.clamp(0.0, 1.0) * 255.0).round() as u8;
        let arm = mip_chain(s.arm.as_ref().unwrap_or(&one([255, rough, 0, 255])), size, MapKind::Linear);
        for (n, a) in normal.as_chunks::<4>().0.iter().zip(arm.as_chunks::<4>().0) {
            data.extend_from_slice(&[n[0], n[1], a[1], a[0]]);
        }
    }
    CoverArray { size, mips: size.trailing_zeros() + 1, slices: SLICES_PER_CHANNEL * 2, data }
}

/// The uniform's cover slots, per channel: x `1 / tiling_m` (0: no layer), y depth (m), z
/// sparkle, w contrast.
pub fn cover_slots(layers: &[Option<CoverSource>; 2]) -> [Vec4; 2] {
    layers.each_ref().map(|s| match s {
        Some(s) if s.layer.tiling_m > 0.0 => {
            Vec4::new(1.0 / s.layer.tiling_m, s.layer.depth_m(), s.layer.sparkle, s.layer.contrast)
        }
        _ => Vec4::ZERO,
    })
}

/// The array as a Bevy image (a D2 array view even with one slice).
pub fn cover_image(array: &CoverArray) -> Image {
    let extent = Extent3d { width: array.size, height: array.size, depth_or_array_layers: array.slices };
    let mut image = Image::new_uninit(extent, TextureDimension::D2, TextureFormat::Rgba8Unorm, RenderAssetUsages::RENDER_WORLD);
    image.texture_descriptor.mip_level_count = array.mips;
    image.data = Some(array.data.clone());
    image.texture_view_descriptor =
        Some(TextureViewDescriptor { dimension: Some(TextureViewDimension::D2Array), ..TextureViewDescriptor::default() });
    // The render world binds its own sampler (51); this one documents the intent.
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 16,
        ..ImageSamplerDescriptor::default()
    });
    image
}

/// Repacks the array and the uniform slots when the host's layers change (or first appear).
pub(crate) fn update_cover(
    layers: Option<Res<CoverLayers>>,
    gpu: Option<Res<SurfaceGpu>>,
    mut images: ResMut<Assets<Image>>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
) {
    let (Some(layers), Some(gpu)) = (layers, gpu) else { return };
    if !layers.is_changed() {
        return;
    }
    let array = pack(&layers.0);
    stats.cover_slices = array.slices;
    if images.insert(&gpu.cover, cover_image(&array)).is_err() {
        return;
    }
    let slots = cover_slots(&layers.0);
    if uniform.cover != slots {
        uniform.cover = slots;
    }
}
