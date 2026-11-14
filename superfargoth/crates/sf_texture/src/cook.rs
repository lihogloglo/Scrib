//! Cooking a decoded texture to a block-compressed KTX2 file by its role (textures.md §5, slice T1).
//!
//! A texture's role decides its format, colour space and mip filter (the industry standard, as in
//! Unreal's texture groups and compression settings, Unity's texture types and NVIDIA Texture Tools):
//!
//! | Role | Maps | Format | Mips |
//! |---|---|---|---|
//! | [`Role::Color`] | albedo, glow, detail | BC7 sRGB | [`mip_chain`] (linear light, alpha-weighted), [`preserve_coverage`] for alpha-tested |
//! | [`Role::Normal`] | tangent-space normals | BC5 (R = X, G = Y) | [`normal_mip_chain`]: box filter of vectors, renormalised per level |
//! | [`Role::Mask`] | roughness, metalness, AO, specular intensity, height | BC4 | [`mask_mip_chain`]: plain box of one channel |
//! | [`Role::Packed`] | channel-packed linear maps (ORM: R = AO, G = roughness, B = metalness) | BC7 linear | [`linear_mip_chain`]: plain box of every channel |
//!
//! - **Normals** come out in the engine's convention, +Y (OpenGL, glTF, Bevy): a [`SourceY::Down`]
//!   (DirectX) source gets its green channel inverted here, so the runtime never flips. BC5 keeps
//!   only X and Y; the shader rebuilds Z = sqrt(1 - X² - Y²), so the filter works on that same
//!   vector: each texel decodes to (X, Y, rebuilt Z) (the source's blue is ignored, an XY outside the
//!   unit disc is normalised), levels are box-filtered from the previous level in `f32` and
//!   renormalised (a footprint whose vectors cancel out becomes +Z). The levels this module returns
//!   carry the rebuilt Z in blue for viewers; the encoder drops it.
//! - **Masks** keep one channel of the source; `invert` stores `1 - value`, which turns a height map
//!   (white = high) into the depth map Bevy's parallax samples (white = deep), and a gloss map
//!   into roughness.
//! - **Packed** maps are data, not colour: no sRGB, and alpha is a channel like the others (no
//!   alpha weighting).
//! - Encoders are Intel's ISPC Texture Compressor kernels through `intel_tex_2`: [`encode_bc7`],
//!   [`encode_bc4`], [`encode_bc5`]. A level whose size isn't a multiple of 4 is padded by repeating
//!   its edge, as [`encode_bc7`] does.
//!
//! [`cook_rgba`] runs the role's chain, encoder and [`write_ktx2`]. Like [`crate::bc7_ktx2`], it
//! encodes any size; wgpu needs level 0 to be a whole number of blocks, so the bake step checks
//! [`crate::ktx2_out::is_block_aligned`] first.

use std::borrow::Cow;

use intel_tex_2::{RSurface, RgSurface, bc4, bc5};

use crate::delight::check_len;
use crate::ktx2_out::{
    Bc7Quality, BlockFormat, MipLevel, downsample, encode_bc7, level_len, mip_chain, mip_count,
    pad_to, preserve_coverage, unit_to_byte, write_ktx2,
};

/// What a colour texture's alpha channel is used for.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AlphaUse {
    /// Unused: alpha is cooked as 255, so the encoder spends no bits on it.
    #[default]
    None,
    /// Alpha-tested at this cutoff (`0.0..=1.0`, a `>=` test): mips preserve coverage.
    Test(f32),
    /// Blended (translucent): alpha kept as filtered.
    Blend,
}

/// Which way a normal map's green channel points (its Y axis).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum SourceY {
    /// +Y up: OpenGL, glTF, Blender, Bevy. Cooked as is.
    #[default]
    Up,
    /// +Y down: DirectX, Unreal, most DDS packs. Green is inverted at cook.
    Down,
}

/// One channel of an RGBA8 source.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Channel {
    /// Red, also a greyscale source's value.
    #[default]
    R,
    G,
    B,
    A,
}

impl Channel {
    const fn index(self) -> usize {
        match self {
            Self::R => 0,
            Self::G => 1,
            Self::B => 2,
            Self::A => 3,
        }
    }
}

/// How a texture is used, which decides its format and mip filter (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    /// sRGB colour: albedo, glow (emissive), detail.
    Color { alpha: AlphaUse },
    /// Tangent-space normal map.
    Normal { source_y: SourceY },
    /// One linear channel: roughness, metalness, ambient occlusion, specular intensity, or
    /// (`invert: true`) height cooked as a depth map / gloss cooked as roughness.
    Mask { channel: Channel, invert: bool },
    /// Linear channel-packed data, every channel kept: ORM (R = occlusion, G = roughness,
    /// B = metalness, the glTF and Bevy layout) and similar masks.
    Packed,
}

impl Role {
    /// A height map (white = high) as Bevy's parallax depth map (white = deep).
    pub const fn height(channel: Channel) -> Self {
        Self::Mask {
            channel,
            invert: true,
        }
    }
}

/// The block format a role cooks to (module docs).
pub const fn format_for(role: Role) -> BlockFormat {
    match role {
        Role::Color { .. } => BlockFormat::Bc7Srgb,
        Role::Normal { .. } => BlockFormat::Bc5,
        Role::Mask { .. } => BlockFormat::Bc4,
        Role::Packed => BlockFormat::Bc7,
    }
}

/// One level of a single-channel chain: one byte per texel, row by row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaneLevel {
    pub width: u32,
    pub height: u32,
    pub values: Vec<u8>,
}

/// Box-filtered levels of an `f32` texel grid down to 1x1, each from the previous one in `f32`,
/// `fix` applied to every texel of every level below the first (renormalising, for normals).
/// `emit` receives each level, the first included, to quantise.
fn box_levels(
    mut current: Vec<[f32; 4]>,
    (width, height): (u32, u32),
    fix: impl Fn(&mut [f32; 4]),
    mut emit: impl FnMut(u32, u32, &[[f32; 4]]),
) {
    emit(width, height, &current);
    let (mut w, mut h) = (width as usize, height as usize);
    while w > 1 || h > 1 {
        let (next_w, next_h) = ((w / 2).max(1), (h / 2).max(1));
        current = downsample(&current, (w, h), (next_w, next_h));
        current.iter_mut().for_each(&fix);
        (w, h) = (next_w, next_h);
        emit(
            u32::try_from(w).unwrap_or(u32::MAX),
            u32::try_from(h).unwrap_or(u32::MAX),
            &current,
        );
    }
}

fn check_area(rgba: &[u8], width: u32, height: u32) {
    check_len(rgba, width, height);
    assert!(
        width > 0 && height > 0,
        "a mip chain needs a texture with area"
    );
}

/// A normal map's mip chain in the engine's +Y convention (module docs): level 0 keeps the
/// source's X and Y bytes (green inverted for [`SourceY::Down`]) with the rebuilt Z in blue and
/// alpha 255; every level below is box-filtered and renormalised.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the texture has no area.
pub fn normal_mip_chain(rgba: &[u8], width: u32, height: u32, source_y: SourceY) -> Vec<MipLevel> {
    check_area(rgba, width, height);
    let decode = |byte: u8| f32::from(byte) / 255.0 * 2.0 - 1.0;
    let encode = |value: f32| unit_to_byte(value * 0.5 + 0.5);
    let top: Vec<[f32; 4]> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| {
            let x = decode(texel[0]);
            let g = match source_y {
                SourceY::Up => texel[1],
                SourceY::Down => u8::MAX - texel[1],
            };
            let y = decode(g);
            let xy = x.mul_add(x, y * y);
            if xy > 1.0 {
                let scale = xy.sqrt().recip();
                [x * scale, y * scale, 0.0, 0.0]
            } else {
                [x, y, (1.0 - xy).sqrt(), 0.0]
            }
        })
        .collect();
    let mut levels = Vec::with_capacity(mip_count(width, height) as usize);
    box_levels(
        top,
        (width, height),
        |texel| {
            let [x, y, z, _] = *texel;
            let length = x.mul_add(x, y.mul_add(y, z * z)).sqrt();
            *texel = if length > 1e-6 {
                [x / length, y / length, z / length, 0.0]
            } else {
                [0.0, 0.0, 1.0, 0.0]
            };
        },
        |w, h, texels| {
            let rgba = texels
                .iter()
                .flat_map(|&[x, y, z, _]| [encode(x), encode(y), encode(z), u8::MAX])
                .collect();
            levels.push(MipLevel {
                width: w,
                height: h,
                rgba,
            });
        },
    );
    levels
}

/// One channel's mip chain, plain box filter (linear data); `invert` stores `255 - value`.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the texture has no area.
pub fn mask_mip_chain(
    rgba: &[u8],
    width: u32,
    height: u32,
    channel: Channel,
    invert: bool,
) -> Vec<PlaneLevel> {
    check_area(rgba, width, height);
    let index = channel.index();
    let top: Vec<[f32; 4]> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| {
            let byte = if invert {
                u8::MAX - texel[index]
            } else {
                texel[index]
            };
            [f32::from(byte) / 255.0, 0.0, 0.0, 0.0]
        })
        .collect();
    let mut levels = Vec::with_capacity(mip_count(width, height) as usize);
    box_levels(
        top,
        (width, height),
        |_| {},
        |w, h, texels| {
            levels.push(PlaneLevel {
                width: w,
                height: h,
                values: texels.iter().map(|texel| unit_to_byte(texel[0])).collect(),
            });
        },
    );
    levels
}

/// A mip chain of linear data with every channel filtered alike, alpha included (no alpha
/// weighting, no sRGB): channel-packed masks.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the texture has no area.
pub fn linear_mip_chain(rgba: &[u8], width: u32, height: u32) -> Vec<MipLevel> {
    check_area(rgba, width, height);
    let top: Vec<[f32; 4]> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| texel.map(|byte| f32::from(byte) / 255.0))
        .collect();
    let mut levels = Vec::with_capacity(mip_count(width, height) as usize);
    box_levels(
        top,
        (width, height),
        |_| {},
        |w, h, texels| {
            levels.push(MipLevel {
                width: w,
                height: h,
                rgba: texels
                    .iter()
                    .flat_map(|texel| texel.map(unit_to_byte))
                    .collect(),
            });
        },
    );
    levels
}

/// `texels` (`texel_bytes` each, `width` x `height`) on the 4x4-block grid the encoders read, and
/// that grid's size.
pub(crate) fn block_grid(
    texels: &[u8],
    texel_bytes: usize,
    width: u32,
    height: u32,
) -> (Cow<'_, [u8]>, u32, u32) {
    let (w, h) = (width as usize, height as usize);
    assert_eq!(
        texels.len(),
        w * h * texel_bytes,
        "{width}x{height} texels of {texel_bytes} bytes"
    );
    assert!(w > 0 && h > 0, "a level needs a texture with area");
    let (padded_w, padded_h) = (width.div_ceil(4) * 4, height.div_ceil(4) * 4);
    let data = if (padded_w, padded_h) == (width, height) {
        Cow::Borrowed(texels)
    } else {
        Cow::Owned(pad_to(
            texels,
            texel_bytes,
            (w, h),
            (padded_w as usize, padded_h as usize),
        ))
    };
    (data, padded_w, padded_h)
}

/// BC4 blocks of one level (one byte per texel, row by row), [`level_len`] bytes.
///
/// # Panics
/// If `values.len() != width * height` or the level has no area.
pub fn encode_bc4(values: &[u8], width: u32, height: u32) -> Vec<u8> {
    let (data, padded_w, padded_h) = block_grid(values, 1, width, height);
    let surface = RSurface {
        data: &data,
        width: padded_w,
        height: padded_h,
        stride: padded_w,
    };
    let mut blocks = vec![0; level_len(BlockFormat::Bc4, width, height)];
    bc4::compress_blocks_into(&surface, &mut blocks);
    blocks
}

/// BC5 blocks of one level (R, G byte pairs per texel, row by row), [`level_len`] bytes.
///
/// # Panics
/// If `rg.len() != width * height * 2` or the level has no area.
pub fn encode_bc5(rg: &[u8], width: u32, height: u32) -> Vec<u8> {
    let (data, padded_w, padded_h) = block_grid(rg, 2, width, height);
    let surface = RgSurface {
        data: &data,
        width: padded_w,
        height: padded_h,
        stride: padded_w * 2,
    };
    let mut blocks = vec![0; level_len(BlockFormat::Bc5, width, height)];
    bc5::compress_blocks_into(&surface, &mut blocks);
    blocks
}

/// The encoded levels of a texture in its role's format ([`format_for`]), level 0 first: the
/// payload [`cook_rgba`] wraps. `quality` applies to the BC7 roles; BC4 and BC5 have one profile.
///
/// # Panics
/// If `rgba.len() != width * height * 4`, the texture has no area, or an alpha-test cutoff is
/// outside `0.0..=1.0`.
pub fn cook_levels(
    role: Role,
    rgba: &[u8],
    width: u32,
    height: u32,
    quality: Bc7Quality,
) -> Vec<Vec<u8>> {
    let bc7 = |chain: Vec<MipLevel>| -> Vec<Vec<u8>> {
        chain
            .iter()
            .map(|level| encode_bc7(&level.rgba, level.width, level.height, quality))
            .collect()
    };
    match role {
        Role::Color { alpha } => {
            let source = if alpha == AlphaUse::None {
                check_len(rgba, width, height);
                let mut opaque = rgba.to_vec();
                for texel in opaque.as_chunks_mut::<4>().0 {
                    texel[3] = u8::MAX;
                }
                Cow::Owned(opaque)
            } else {
                Cow::Borrowed(rgba)
            };
            let mut chain = mip_chain(&source, width, height, true);
            if let AlphaUse::Test(cutoff) = alpha {
                preserve_coverage(&mut chain, cutoff);
            }
            bc7(chain)
        }
        Role::Normal { source_y } => normal_mip_chain(rgba, width, height, source_y)
            .iter()
            .map(|level| {
                let rg: Vec<u8> = level
                    .rgba
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .flat_map(|texel| [texel[0], texel[1]])
                    .collect();
                encode_bc5(&rg, level.width, level.height)
            })
            .collect(),
        Role::Mask { channel, invert } => mask_mip_chain(rgba, width, height, channel, invert)
            .iter()
            .map(|level| encode_bc4(&level.values, level.width, level.height))
            .collect(),
        Role::Packed => bc7(linear_mip_chain(rgba, width, height)),
    }
}

/// A texture's KTX2 file for its role: [`cook_levels`] wrapped by [`write_ktx2`] in
/// [`format_for`]`(role)`.
///
/// # Panics
/// As [`cook_levels`].
pub fn cook_rgba(role: Role, rgba: &[u8], width: u32, height: u32, quality: Bc7Quality) -> Vec<u8> {
    let levels = cook_levels(role, rgba, width, height, quality);
    write_ktx2(format_for(role), width, height, &levels)
}

/// The perceptual roughness (Bevy's `perceptual_roughness`, `α = r²`) matching a Blinn-Phong
/// specular exponent: `α = √(2 / (n + 2))` (Walter et al. 2007, "Microfacet Models for
/// Refraction", the Phong ↔ Beckmann equivalence; GGX's α is used alike, as in Karis's
/// "Specular BRDF Reference"), so `r = (2 / (n + 2))^¼`. `n = 0` is fully rough (1.0); `n = 255`
/// is 0.297.
pub fn blinn_phong_roughness(exponent: f32) -> f32 {
    (2.0 / (exponent.max(0.0) + 2.0)).powf(0.25)
}

/// A legacy specular map's texels (RGB specular colour, A the Blinn-Phong exponent ÷ 255) as
/// canonical ORM texels, in place: R 255 (no occlusion), G the perceptual roughness of the
/// exponent ([`blinn_phong_roughness`]), B 0 (dielectric), A 255. The specular colour has no ORM
/// channel and is dropped (a per-texel specular tint needs another material input).
///
/// # Panics
/// If `rgba.len()` isn't a multiple of 4.
pub fn specular_to_orm(rgba: &mut [u8]) {
    let (texels, rest) = rgba.as_chunks_mut::<4>();
    assert!(rest.is_empty(), "RGBA8 texels");
    let mut table = [0u8; 256];
    for (a, out) in table.iter_mut().enumerate() {
        #[allow(clippy::cast_precision_loss)] // 0..=255
        let exponent = a as f32;
        *out = unit_to_byte(blinn_phong_roughness(exponent));
    }
    for texel in texels {
        *texel = [u8::MAX, table[usize::from(texel[3])], 0, u8::MAX];
    }
}

#[cfg(test)]
mod tests;
