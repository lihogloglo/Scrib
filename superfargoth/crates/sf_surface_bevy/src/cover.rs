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
//! W5b: a look with no height (alpha all opaque) gets one from its albedo's luminance
//! ([`with_height`]) and a look with no normal map one from that height ([`normal_from_height`]),
//! the bitmap-to-material fallback, so plain land textures still blend by height and catch light.
//! The shader samples top-down at `render XZ / tiling + `[`cover_uv`].
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

use crate::{SurfaceGpu, SurfaceOrigin, SurfaceStats, SurfaceUniform};

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

/// Slope of the derived normal per unit of derived height across one texel (W5b; the bitmap-to-
/// material fallback for looks without a normal map).
pub const DERIVED_NORMAL_STRENGTH: f32 = 3.0;
/// Luminance percentiles the derived height spans 0..1 (robust auto-levels).
const DERIVED_HEIGHT_PERCENTILES: (f32, f32) = (0.02, 0.98);

fn srgb_to_linear(c: u8) -> f32 {
    let c = f32::from(c) / 255.0;
    if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// A look's height: its alpha when the source has one, else (all alpha opaque: a plain albedo,
/// e.g. a land texture) the albedo's linear luminance spread over 0..1 between its 2nd and 98th
/// percentiles, bright = thick (the bitmap-to-material fallback; a modded look with a real
/// height in alpha keeps it).
pub fn with_height(albedo: &LayerImage) -> LayerImage {
    let px = albedo.rgba.as_chunks::<4>().0;
    if px.iter().any(|p| p[3] != 255) || px.is_empty() {
        return albedo.clone();
    }
    let luma: Vec<f32> = px
        .iter()
        .map(|p| 0.2126 * srgb_to_linear(p[0]) + 0.7152 * srgb_to_linear(p[1]) + 0.0722 * srgb_to_linear(p[2]))
        .collect();
    let mut sorted = luma.clone();
    sorted.sort_by(f32::total_cmp);
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let at = |q: f32| sorted[((sorted.len() - 1) as f32 * q).round() as usize];
    let (lo, hi) = (at(DERIVED_HEIGHT_PERCENTILES.0), at(DERIVED_HEIGHT_PERCENTILES.1));
    let span = (hi - lo).max(1e-4);
    let mut out = albedo.clone();
    for (p, l) in out.rgba.as_chunks_mut::<4>().0.iter_mut().zip(luma) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let h = (((l - lo) / span).clamp(0.0, 1.0) * 255.0).round() as u8;
        p[3] = h;
    }
    out
}

/// A tangent-space normal map (+Y toward the image's top, the array's convention) from the
/// height in `albedo_height`'s alpha: central differences, wrapping (looks tile).
pub fn normal_from_height(albedo_height: &LayerImage) -> LayerImage {
    let (w, h) = (albedo_height.width.max(1) as usize, albedo_height.height.max(1) as usize);
    let height = |x: usize, y: usize| f32::from(albedo_height.rgba[((y % h) * w + (x % w)) * 4 + 3]) / 255.0;
    let mut rgba = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            // Height rises toward +X and toward the image's top (row - 1).
            let dx = 0.5 * (height(x + 1, y) - height(x + w - 1, y)) * DERIVED_NORMAL_STRENGTH;
            let dy = 0.5 * (height(x, y + h - 1) - height(x, y + 1)) * DERIVED_NORMAL_STRENGTH;
            let n = bevy_math::Vec3::new(-dx, -dy, 1.0).normalize();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let b = |v: f32| ((v * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
            rgba.extend_from_slice(&[b(n.x), b(n.y), b(n.z), 255]);
        }
    }
    #[allow(clippy::cast_possible_truncation)]
    LayerImage { width: w as u32, height: h as u32, rgba }
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
        let albedo_height = with_height(&s.albedo_height);
        data.extend(mip_chain(&albedo_height, size, MapKind::Albedo));
        let derived;
        let normal_source = if let Some(n) = &s.normal {
            n
        } else {
            derived = normal_from_height(&albedo_height);
            &derived
        };
        let normal = mip_chain(normal_source, size, MapKind::Normal);
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

/// The cover UV at the render origin (`SurfaceUniform::cover_uv`: xy snow, zw ash): the origin's
/// X, Z over each channel's tiling, in f64 and wrapped, so the shader's `render XZ / tiling +
/// this` is the world-aligned UV with no f32 precision loss far from the world origin. `slots`
/// are [`cover_slots`] (x = 1 / tiling as the shader reads it).
pub fn cover_uv(origin: bevy_math::DVec3, slots: &[Vec4; 2]) -> Vec4 {
    #[allow(clippy::cast_possible_truncation)]
    let uv = |s: Vec4, v: f64| (v * f64::from(s.x)).rem_euclid(1.0) as f32;
    Vec4::new(uv(slots[0], origin.x), uv(slots[0], origin.z), uv(slots[1], origin.x), uv(slots[1], origin.z))
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

/// Repacks the array and the uniform slots when the host's layers change (or first appear), and
/// keeps the cover UV on the floating origin.
pub(crate) fn update_cover(
    layers: Option<Res<CoverLayers>>,
    gpu: Option<Res<SurfaceGpu>>,
    origin: Res<SurfaceOrigin>,
    mut images: ResMut<Assets<Image>>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
) {
    let (Some(layers), Some(gpu)) = (layers, gpu) else { return };
    if layers.is_changed() {
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
    let uv = cover_uv(origin.0.to_f64(), &uniform.cover);
    if uniform.cover_uv != uv {
        uniform.cover_uv = uv;
    }
}
