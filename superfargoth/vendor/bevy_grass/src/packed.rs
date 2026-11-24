//! Density masks and heightmaps as the GPU reads them, kept per image.
//!
//! Reading a 4096 by 4096 mask texel by texel takes milliseconds, so it is
//! done once per image and again only when that image changes: moving a field
//! or changing a style, which happens far more often, must not pay for it.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bevy::asset::{AssetId, RenderAssetUsages};
use bevy::color::LinearRgba;
use bevy::ecs::prelude::*;
use bevy::image::Image;
use bevy::math::UVec2;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// The packed form of every image a field uses as a mask or a heightmap.
#[derive(Resource, Debug, Default)]
pub(crate) struct PackedImages {
    masks: HashMap<AssetId<Image>, Arc<Vec<u32>>>,
    heights: HashMap<AssetId<Image>, Arc<Vec<f32>>>,
    /// Images read texel by texel so far. A test can tell whether a change
    /// cost a read.
    pub reads: u64,
}

impl PackedImages {
    /// `image` as a mask: R8G8 texels, two per word, low half first. The
    /// words are padded so a mask starts on a whole word.
    pub(crate) fn mask(&mut self, id: AssetId<Image>, image: &Image) -> Arc<Vec<u32>> {
        if let Some(packed) = self.masks.get(&id) {
            return packed.clone();
        }
        self.reads += 1;
        let packed = Arc::new(pack_mask(image));
        self.masks.insert(id, packed.clone());
        packed
    }

    /// `image` as a heightmap: its red channel, one normalized height per
    /// texel.
    pub(crate) fn heights(&mut self, id: AssetId<Image>, image: &Image) -> Arc<Vec<f32>> {
        if let Some(packed) = self.heights.get(&id) {
            return packed.clone();
        }
        self.reads += 1;
        let packed = Arc::new(read_heights(image));
        self.heights.insert(id, packed.clone());
        packed
    }

    /// Drops what was read from images that changed, so the next use reads
    /// them again.
    pub(crate) fn forget(&mut self, changed: &HashSet<AssetId<Image>>) {
        self.masks.retain(|id, _| !changed.contains(id));
        self.heights.retain(|id, _| !changed.contains(id));
    }

    /// Keeps only what fields still use.
    pub(crate) fn retain(&mut self, masks: &[AssetId<Image>], heights: &[AssetId<Image>]) {
        self.masks.retain(|id, _| masks.contains(id));
        self.heights.retain(|id, _| heights.contains(id));
    }
}

/// Words in a mask of `texels` texels.
pub fn mask_words(texels: u32) -> u32 {
    texels.div_ceil(2)
}

/// A mask as R8G8 texels, two per word, low half first.
///
/// Whatever the image's colour space, the red and green are the values
/// stored. (An sRGB image read as a colour would be gamma-curved.)
pub fn pack_mask(image: &Image) -> Vec<u32> {
    // superfargoth patch (own: raw image reads): the 8-bit unorm formats read
    // their bytes straight, which is what `get_color_at` rounds back to (a
    // byte b is b / 255, and b / 255 * 255 rounds to b). Every other format,
    // or an image whose data is missing or short, reads texel by texel.
    let fast = match image.texture_descriptor.format {
        TextureFormat::Rg8Unorm => texel_bytes(image, 2).map(|bytes| {
            pack_texels(
                bytes.len() / 2,
                bytes
                    .chunks_exact(2)
                    .map(|t| u32::from(t[0]) | (u32::from(t[1]) << 8)),
            )
        }),
        // One channel is grey: red and green are both the byte.
        TextureFormat::R8Unorm => texel_bytes(image, 1).map(|bytes| {
            pack_texels(bytes.len(), bytes.iter().map(|&v| u32::from(v) * 0x0101))
        }),
        TextureFormat::Rgba8Unorm => texel_bytes(image, 4).map(|bytes| {
            pack_texels(
                bytes.len() / 4,
                bytes
                    .chunks_exact(4)
                    .map(|t| u32::from(t[0]) | (u32::from(t[1]) << 8)),
            )
        }),
        _ => None,
    };
    fast.unwrap_or_else(|| pack_mask_per_texel(image))
}

// superfargoth patch (own: raw image reads): the bytes of every texel of a 2D
// image, row by row, when its data holds them all; `None` sends the read down
// the per-texel path.
fn texel_bytes(image: &Image, bytes_per_texel: usize) -> Option<&[u8]> {
    if image.texture_descriptor.dimension != TextureDimension::D2 {
        return None;
    }
    let size = image.size();
    let len = size.x as usize * size.y as usize * bytes_per_texel;
    image.data.as_deref().and_then(|data| data.get(..len))
}

// superfargoth patch (own: raw image reads): `texels` mask texels as words,
// two per word, low half first, the last word padded.
fn pack_texels(texels: usize, texels_iter: impl Iterator<Item = u32>) -> Vec<u32> {
    let mut words = Vec::with_capacity(texels.div_ceil(2));
    let mut half = None;
    for texel in texels_iter {
        match half.take() {
            None => half = Some(texel),
            Some(low) => words.push(low | (texel << 16)),
        }
    }
    if let Some(low) = half {
        words.push(low);
    }
    words
}

// superfargoth patch (own: raw image reads): the original per-texel read, kept
// for the formats the fast path does not cover.
/// The per-texel read (`Image::get_color_at`), for formats the raw path does not cover.
pub fn pack_mask_per_texel(image: &Image) -> Vec<u32> {
    let size = image.size();
    let texels = (0..size.y).flat_map(|y| {
        (0..size.x).map(move |x| {
            image.get_color_at(x, y).map_or(0xffff, |color| {
                let c = LinearRgba::from(color);
                let r = (c.red.clamp(0.0, 1.0) * 255.0).round() as u32;
                let g = (c.green.clamp(0.0, 1.0) * 255.0).round() as u32;
                r | (g << 8)
            })
        })
    });
    pack_texels((size.x * size.y) as usize, texels)
}

/// `densities` as a density mask image, `texels` across: red the density,
/// green 1 (full height), kept on the CPU only, which is all a mask needs.
pub(crate) fn density_image(densities: &[f32], texels: UVec2) -> Image {
    let data = densities
        .iter()
        .flat_map(|&density| {
            // NaN is no grass, like a density of 0.
            let density = if density.is_nan() { 0.0 } else { density };
            [(density.clamp(0.0, 1.0) * 255.0).round() as u8, u8::MAX]
        })
        .collect();
    Image::new(
        Extent3d {
            width: texels.x,
            height: texels.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rg8Unorm,
        RenderAssetUsages::MAIN_WORLD,
    )
}

/// A heightmap's red channel, normalized.
pub fn read_heights(image: &Image) -> Vec<f32> {
    // superfargoth patch (own: raw image reads): R32Float heights are their
    // little-endian floats as stored, which is all `get_color_at` returns for
    // red. Every other format, or missing or short data, reads texel by texel.
    if image.texture_descriptor.format == TextureFormat::R32Float
        && let Some(bytes) = texel_bytes(image, 4)
    {
        return bytes
            .chunks_exact(4)
            .map(|t| f32::from_le_bytes([t[0], t[1], t[2], t[3]]))
            .collect();
    }
    read_heights_per_texel(image)
}

// superfargoth patch (own: raw image reads): the original per-texel read, kept
// for the formats the fast path does not cover.
/// The per-texel read (`Image::get_color_at`), for formats the raw path does not cover.
pub fn read_heights_per_texel(image: &Image) -> Vec<f32> {
    let size = image.size();
    let mut heights = Vec::with_capacity((size.x * size.y) as usize);
    for y in 0..size.y {
        for x in 0..size.x {
            heights.push(
                image
                    .get_color_at(x, y)
                    .map_or(0.0, |c| LinearRgba::from(c).red),
            );
        }
    }
    heights
}

// superfargoth patch (own: stable tables): the tables write each image in
// place now (`fields::rebuild`); only the tests still join.
#[cfg(test)]
/// Joins packed images end to end. One image needs no copy at all.
pub(crate) fn join<T: Copy>(parts: &[Arc<Vec<T>>]) -> Arc<Vec<T>> {
    match parts {
        [] => Arc::default(),
        [only] => only.clone(),
        many => Arc::new(many.iter().flat_map(|part| part.iter().copied()).collect()),
    }
}

#[cfg(test)]
mod tests;
