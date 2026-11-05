//! Mip chain, BC7 encode and KTX2 container for a baked colour texture.
//!
//! 1. [`mip_chain`]: box-filtered levels down to 1x1 (KTX2/Vulkan sizes: each axis halves, rounded
//!    down, never below 1). Each output texel averages the source area it covers, so an odd axis
//!    (75 to 37) blends partial texels instead of dropping a column. The average is taken in linear
//!    light for sRGB data, with colour weighted by alpha (premultiplied), so a cutout's transparent
//!    texels don't bleed their colour into its edge; alpha itself is the plain average. Levels are
//!    filtered from the previous level kept in `f32`, quantised once.
//!
//!    For alpha-tested (cutout) textures, [`preserve_coverage`]: averaging alpha shrinks the part
//!    of a level that passes the alpha test, so foliage, fences and hair thin out and vanish with
//!    distance. Each level below the top gets its alpha scaled so that the fraction of texels at
//!    or above the caller's cutoff matches the top level's: Ignacio Castaño, "Computing Alpha
//!    Mipmaps" (2010), as in NVIDIA Texture Tools (`scaleAlphaToCoverage`), Unity's "Mip Maps
//!    Preserve Coverage" and Unreal. The scale is found by bisection on the quantised output.
//! 2. [`encode_bc7`]: BC7 through Intel's ISPC Texture Compressor kernels (`intel_tex_2`),
//!    the "basic" or "fast" profile, its alpha variant only when the level has a texel with
//!    alpha below 255. A level whose size isn't a multiple of 4 is padded by repeating its edge
//!    (the padding lies outside the texture and is never sampled).
//! 3. [`write_ktx2_bc7`]: a KTX 2.0 file, `VK_FORMAT_BC7_SRGB_BLOCK` (146) or `_UNORM_BLOCK` (145),
//!    no supercompression. Layout per the Khronos KTX 2.0 specification
//!    (<https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html>): header and index (§3.1-3.7),
//!    level index with level 0 first (§3.8), the Data Format Descriptor (§3.10) from the `ktx2`
//!    crate's table for the format (one basic block: colour model BC7, BT.709 primaries, sRGB or
//!    linear transfer, straight alpha, 4x4 blocks of 16 bytes), one key/value entry `KTXwriter`
//!    (§3.11, §5.13), then the level data smallest level first (§3.9), each level at an offset that
//!    is a multiple of 16 (`lcm(texel block size, 4)`, §3.9.7).
//!
//! [`bc7_ktx2`] runs all three; [`bc7_ktx2_cutout`] also preserves coverage when given a cutoff. wgpu needs a block-compressed texture's level 0 to be a whole number
//! of blocks: check [`is_block_aligned`] first.

use intel_tex_2::{RgbaSurface, bc7};
use ktx2::{Format, Header, Index, LevelIndex, dfd};

use crate::delight::{Srgb, check_len};

/// The `KTXwriter` value written into every file.
pub const KTX_WRITER: &str = concat!("sf_texture ", env!("CARGO_PKG_VERSION"));

/// Bytes in one BC7 block (4x4 texels).
pub const BC7_BLOCK_BYTES: usize = 16;

/// Colour weight of a fully transparent texel, relative to an opaque one's 1: enough that a
/// footprint with no visible texel still averages its colours, too little to tint a visible one.
const CLEAR_WEIGHT: f32 = 1.0 / 4096.0;

/// One level of a mip chain: RGBA8 texels, row by row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MipLevel {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// BC7 encoder effort: the ISPC compressor's profiles of the same name.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Bc7Quality {
    /// About 2.5x the speed of [`Bc7Quality::Basic`], a little lower quality.
    Fast,
    /// The default.
    #[default]
    Basic,
}

/// Number of levels down to 1x1: `floor(log2(max(width, height))) + 1` (KTX2 §3.8).
pub fn mip_count(width: u32, height: u32) -> u32 {
    32 - width.max(height).max(1).leading_zeros()
}

/// Whether a texture of this size is a whole number of 4x4 blocks, as wgpu requires of a
/// block-compressed texture's level 0.
pub fn is_block_aligned(width: u32, height: u32) -> bool {
    width > 0 && height > 0 && width.is_multiple_of(4) && height.is_multiple_of(4)
}

/// Bytes of one BC7 level of this size.
pub fn bc7_level_len(width: u32, height: u32) -> usize {
    width.div_ceil(4) as usize * height.div_ceil(4) as usize * BC7_BLOCK_BYTES
}

/// The full mip chain of a texture: level 0 is `rgba` itself, then each level box-filters the
/// previous one (module docs). `srgb`: the colour channels are sRGB-encoded (filtered in linear
/// light); else they are filtered as stored.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the texture has no area.
pub fn mip_chain(rgba: &[u8], width: u32, height: u32, srgb: bool) -> Vec<MipLevel> {
    check_len(rgba, width, height);
    assert!(
        width > 0 && height > 0,
        "a mip chain needs a texture with area"
    );
    let decode = |byte: u8| {
        if srgb {
            Srgb::get().decode[usize::from(byte)]
        } else {
            f32::from(byte) / 255.0
        }
    };
    let mut current: Vec<[f32; 4]> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| {
            let weight = f32::from(texel[3]) / 255.0 + CLEAR_WEIGHT;
            [
                decode(texel[0]) * weight,
                decode(texel[1]) * weight,
                decode(texel[2]) * weight,
                weight,
            ]
        })
        .collect();
    let mut levels = Vec::with_capacity(mip_count(width, height) as usize);
    levels.push(MipLevel {
        width,
        height,
        rgba: rgba.to_vec(),
    });
    let (mut w, mut h) = (width as usize, height as usize);
    while w > 1 || h > 1 {
        let (next_w, next_h) = ((w / 2).max(1), (h / 2).max(1));
        current = downsample(&current, (w, h), (next_w, next_h));
        (w, h) = (next_w, next_h);
        let rgba = current
            .iter()
            .flat_map(|texel| quantise(texel, srgb))
            .collect();
        levels.push(MipLevel {
            width: u32::try_from(w).unwrap_or(u32::MAX),
            height: u32::try_from(h).unwrap_or(u32::MAX),
            rgba,
        });
    }
    levels
}

/// Bisection steps of [`preserve_coverage`]'s scale search: the bracket shrinks to 2^-32 of its
/// width, well below the step where one more alpha byte crosses the cutoff.
const COVERAGE_STEPS: u32 = 32;

/// Scales the alpha of every level after the first so that its alpha-test coverage (the fraction
/// of texels with `alpha / 255 >= cutoff`, the test of a `>=` mask such as Bevy's
/// `AlphaMode::Mask`) matches the first level's as closely as its texel count allows (module
/// docs). A level whose coverage already matches keeps its bytes; colour is never touched.
/// `cutoff` is in `0.0..=1.0` and comes from the content (a model's alpha-test threshold).
///
/// # Panics
/// If `cutoff` is not within `0.0..=1.0`.
pub fn preserve_coverage(levels: &mut [MipLevel], cutoff: f32) {
    assert!(
        (0.0..=1.0).contains(&cutoff),
        "alpha cutoff {cutoff} outside 0..=1"
    );
    let Some((top, rest)) = levels.split_first_mut() else {
        return;
    };
    let target = coverage(&top.rgba, cutoff, 1.0);
    let top_texels = texel_count(top);
    for level in rest {
        let texels = texel_count(level);
        // The pass count closest to the top level's fraction, in exact integer arithmetic.
        let wanted = (2 * target * texels + top_texels) / (2 * top_texels);
        let passing = |scale: f32| coverage(&level.rgba, cutoff, scale);
        if passing(1.0) == wanted {
            continue;
        }
        // Coverage only grows with the scale: bracket the smallest scale reaching `wanted`, then
        // keep it or the largest scale below it, whichever count lands closer.
        let (mut low, mut high) = (0.0_f32, 256.0_f32);
        for _ in 0..COVERAGE_STEPS {
            let mid = f32::midpoint(low, high);
            if passing(mid) >= wanted {
                high = mid;
            } else {
                low = mid;
            }
        }
        let scale = if wanted.abs_diff(passing(low)) < wanted.abs_diff(passing(high)) {
            low
        } else {
            high
        };
        for texel in level.rgba.as_chunks_mut::<4>().0 {
            texel[3] = scaled_alpha(texel[3], scale);
        }
    }
}

fn texel_count(level: &MipLevel) -> usize {
    level.width as usize * level.height as usize
}

/// Texels of `rgba` passing the alpha test at `cutoff` once alpha is scaled by `scale`.
fn coverage(rgba: &[u8], cutoff: f32, scale: f32) -> usize {
    rgba.as_chunks::<4>()
        .0
        .iter()
        .filter(|texel| f32::from(scaled_alpha(texel[3], scale)) / 255.0 >= cutoff)
        .count()
}

/// `alpha` scaled and requantised; a scale of exactly 1 returns it unchanged.
#[allow(clippy::float_cmp, reason = "exactly 1 is the identity scale")]
fn scaled_alpha(alpha: u8, scale: f32) -> u8 {
    if scale == 1.0 {
        alpha
    } else {
        unit_to_byte(f32::from(alpha) / 255.0 * scale)
    }
}

/// A weighted linear texel back to RGBA8.
fn quantise(&[r, g, b, weight]: &[f32; 4], srgb: bool) -> [u8; 4] {
    let colour = |premultiplied: f32| {
        let linear = (premultiplied / weight).clamp(0.0, 1.0);
        if srgb {
            Srgb::get().encode(linear)
        } else {
            unit_to_byte(linear)
        }
    };
    [
        colour(r),
        colour(g),
        colour(b),
        unit_to_byte(weight - CLEAR_WEIGHT),
    ]
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "clamped to 0..=255 first"
)]
fn unit_to_byte(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// One axis's box filter from `src` texels to `dst`: output texel `i` covers the source span
/// `[i * src / dst, (i + 1) * src / dst)`, and each source texel weighs its overlap with it.
/// Exact integer overlaps (in units of `1 / dst` texel), divided once.
fn taps(src: usize, dst: usize) -> Vec<Vec<(usize, f32)>> {
    (0..dst)
        .map(|i| {
            let (start, end) = (i * src, (i + 1) * src);
            (start / dst..end.div_ceil(dst))
                .map(|j| {
                    let overlap = end.min((j + 1) * dst) - start.max(j * dst);
                    (j, ratio(overlap, src))
                })
                .collect()
        })
        .collect()
}

#[allow(
    clippy::cast_precision_loss,
    reason = "texture sizes are far below 2^24"
)]
fn ratio(numerator: usize, denominator: usize) -> f32 {
    numerator as f32 / denominator as f32
}

/// Separable box filter of a weighted texel grid: rows, then columns.
fn downsample(
    src: &[[f32; 4]],
    (src_w, src_h): (usize, usize),
    (dst_w, dst_h): (usize, usize),
) -> Vec<[f32; 4]> {
    let across = taps(src_w, dst_w);
    let mut rows = Vec::with_capacity(dst_w * src_h);
    for row in src.chunks_exact(src_w) {
        rows.extend(
            across
                .iter()
                .map(|taps| weighted_sum(taps.iter().map(|&(x, weight)| (&row[x], weight)))),
        );
    }
    let down = taps(src_h, dst_h);
    let mut out = Vec::with_capacity(dst_w * dst_h);
    for taps in &down {
        out.extend((0..dst_w).map(|x| {
            weighted_sum(
                taps.iter()
                    .map(|&(y, weight)| (&rows[y * dst_w + x], weight)),
            )
        }));
    }
    out
}

fn weighted_sum<'a>(texels: impl Iterator<Item = (&'a [f32; 4], f32)>) -> [f32; 4] {
    texels.fold([0.0; 4], |mut sum, (texel, weight)| {
        for (total, value) in sum.iter_mut().zip(texel) {
            *total += value * weight;
        }
        sum
    })
}

/// BC7 blocks of one level, row by row ([`bc7_level_len`] bytes). The alpha-aware profile is used
/// only when a texel's alpha is below 255; the opaque one decodes to alpha 255.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the level has no area.
pub fn encode_bc7(rgba: &[u8], width: u32, height: u32, quality: Bc7Quality) -> Vec<u8> {
    let (w, h) = check_len(rgba, width, height);
    assert!(w > 0 && h > 0, "a BC7 level needs a texture with area");
    let alpha = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .any(|texel| texel[3] != u8::MAX);
    let settings = match (alpha, quality) {
        (false, Bc7Quality::Fast) => bc7::opaque_fast_settings(),
        (false, Bc7Quality::Basic) => bc7::opaque_basic_settings(),
        (true, Bc7Quality::Fast) => bc7::alpha_fast_settings(),
        (true, Bc7Quality::Basic) => bc7::alpha_basic_settings(),
    };
    let (padded_w, padded_h) = (width.div_ceil(4) * 4, height.div_ceil(4) * 4);
    let padded;
    let data = if (padded_w, padded_h) == (width, height) {
        rgba
    } else {
        padded = pad_to(rgba, (w, h), (padded_w as usize, padded_h as usize));
        &padded
    };
    let surface = RgbaSurface {
        data,
        width: padded_w,
        height: padded_h,
        stride: padded_w * 4,
    };
    let mut blocks = vec![0; bc7_level_len(width, height)];
    bc7::compress_blocks_into(&settings, &surface, &mut blocks);
    blocks
}

/// The texels on a larger grid, the last column and row repeated into the margin.
fn pad_to(rgba: &[u8], (w, h): (usize, usize), (padded_w, padded_h): (usize, usize)) -> Vec<u8> {
    let mut out = Vec::with_capacity(padded_w * padded_h * 4);
    for y in 0..padded_h {
        let row = &rgba[y.min(h - 1) * w * 4..][..w * 4];
        out.extend_from_slice(row);
        for _ in w..padded_w {
            out.extend_from_slice(&row[(w - 1) * 4..]);
        }
    }
    out
}

/// A KTX2 file holding BC7 levels (level 0 first, as [`encode_bc7`] returns them) of a
/// `width` x `height` texture. `srgb` picks `BC7_SRGB_BLOCK` over `BC7_UNORM_BLOCK`.
///
/// # Panics
/// If `levels` is empty, longer than [`mip_count`], or a level's length isn't
/// [`bc7_level_len`] of its size.
pub fn write_ktx2_bc7(width: u32, height: u32, srgb: bool, levels: &[Vec<u8>]) -> Vec<u8> {
    let level_count = u32::try_from(levels.len()).unwrap_or(u32::MAX);
    assert!(
        (1..=mip_count(width, height)).contains(&level_count),
        "{level_count} levels for a {width}x{height} texture"
    );
    for (level, data) in levels.iter().enumerate() {
        let (w, h) = ((width >> level).max(1), (height >> level).max(1));
        assert_eq!(
            data.len(),
            bc7_level_len(w, h),
            "level {level} ({w}x{h}) holds whole BC7 blocks"
        );
    }
    let format = if srgb {
        Format::BC7_SRGB_BLOCK
    } else {
        Format::BC7_UNORM_BLOCK
    };
    let (basic, type_size) = dfd::Basic::from_format(format)
        .unwrap_or_else(|error| panic!("ktx2 describes {format:?}: {error:?}"));
    let block = dfd::Block::Basic(basic).to_vec();

    // Sections in file order (§3.1): header + index, level index, DFD, key/value data, levels.
    let level_index_len = LevelIndex::LENGTH * levels.len();
    let dfd_offset = Header::LENGTH + level_index_len;
    let dfd_len = 4 + block.len();
    let kvd_offset = dfd_offset + dfd_len;
    let kvd = key_value_data(&[("KTXwriter", KTX_WRITER)]);
    let mut level_index = vec![
        LevelIndex {
            byte_offset: 0,
            byte_length: 0,
            uncompressed_byte_length: 0
        };
        levels.len()
    ];
    let mut end = kvd_offset + kvd.len();
    for (level, data) in levels.iter().enumerate().rev() {
        end = end.next_multiple_of(BC7_BLOCK_BYTES);
        let length = data.len() as u64;
        level_index[level] = LevelIndex {
            byte_offset: end as u64,
            byte_length: length,
            uncompressed_byte_length: length,
        };
        end += data.len();
    }

    let header = Header {
        format: Some(format),
        type_size,
        pixel_width: width,
        pixel_height: height,
        pixel_depth: 0,
        layer_count: 0,
        face_count: 1,
        level_count,
        supercompression_scheme: None,
        index: Index {
            dfd_byte_offset: offset_u32(dfd_offset),
            dfd_byte_length: offset_u32(dfd_len),
            kvd_byte_offset: offset_u32(kvd_offset),
            kvd_byte_length: offset_u32(kvd.len()),
            sgd_byte_offset: 0,
            sgd_byte_length: 0,
        },
    };
    let mut out = Vec::with_capacity(end);
    out.extend_from_slice(&header.as_bytes());
    for entry in &level_index {
        out.extend_from_slice(&entry.as_bytes());
    }
    out.extend_from_slice(&offset_u32(dfd_len).to_le_bytes());
    out.extend_from_slice(&block);
    out.extend_from_slice(&kvd);
    for (level, data) in levels.iter().enumerate().rev() {
        out.resize(
            usize::try_from(level_index[level].byte_offset).unwrap_or(usize::MAX),
            0,
        );
        out.extend_from_slice(data);
    }
    out
}

/// Key/value entries (§3.11): length, `key\0value\0`, padded to 4 bytes; keys must be sorted.
fn key_value_data(entries: &[(&str, &str)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (key, value) in entries {
        out.extend_from_slice(&offset_u32(key.len() + value.len() + 2).to_le_bytes());
        out.extend_from_slice(key.as_bytes());
        out.push(0);
        out.extend_from_slice(value.as_bytes());
        out.push(0);
        out.resize(out.len().next_multiple_of(4), 0);
    }
    out
}

fn offset_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or_else(|_| panic!("KTX2 section offset {value} exceeds u32"))
}

/// [`mip_chain`], [`encode_bc7`] of every level, [`write_ktx2_bc7`]: a texture's KTX2 file.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the texture has no area.
pub fn bc7_ktx2(rgba: &[u8], width: u32, height: u32, srgb: bool, quality: Bc7Quality) -> Vec<u8> {
    bc7_ktx2_cutout(rgba, width, height, srgb, quality, None)
}

/// [`bc7_ktx2`] for a texture that may be alpha-tested: with `Some(cutoff)` the mips keep the top
/// level's coverage at that cutoff ([`preserve_coverage`]); `None` is [`bc7_ktx2`] exactly.
///
/// # Panics
/// If `rgba.len() != width * height * 4`, the texture has no area, or the cutoff is outside
/// `0.0..=1.0`.
pub fn bc7_ktx2_cutout(
    rgba: &[u8],
    width: u32,
    height: u32,
    srgb: bool,
    quality: Bc7Quality,
    alpha_cutoff: Option<f32>,
) -> Vec<u8> {
    let mut chain = mip_chain(rgba, width, height, srgb);
    if let Some(cutoff) = alpha_cutoff {
        preserve_coverage(&mut chain, cutoff);
    }
    let levels: Vec<Vec<u8>> = chain
        .iter()
        .map(|level| encode_bc7(&level.rgba, level.width, level.height, quality))
        .collect();
    write_ktx2_bc7(width, height, srgb, &levels)
}

#[cfg(test)]
mod tests;
