//! Rewrapping an already block-compressed texture (DDS or KTX2, BC1-BC5, BC7) into our KTX2
//! container without re-encoding it (textures.md §5 "Rewrap rule", slice T2).
//!
//! A second lossy encode of block-compressed data only loses quality and costs time, so the
//! industry standard for shipped blocks is to keep them: [`rewrap`] copies every shipped level's
//! bytes unchanged into [`write_ktx2`] and only **fills missing levels** (a single-level DDS, or a chain that stops
//! early): the smallest shipped level is decoded ([`decode_level`]), filtered down with the role's
//! mip filter (the same as [`crate::cook`]'s) and the new levels are encoded **in the source's
//! format** ([`encode_level`]), because a KTX2 file has one `vkFormat` for all its levels.
//!
//! **Formats read.**
//! - DDS (Microsoft `DDS_HEADER` / `DDS_HEADER_DXT10` reference), parsed with `ddsfile`: legacy
//!   `FourCC` `DXT1` (BC1), `DXT2`/`DXT3` (BC2), `DXT4`/`DXT5` (BC3), `ATI1`/`BC4U` (BC4),
//!   `ATI2`/`BC5U` (BC5), and the DX10 header's DXGI `BC1`-`BC5`, `BC7` (`TYPELESS`, `UNORM`,
//!   `UNORM_SRGB`). Level byte sizes come from the block size (`ceil(w/4) * ceil(h/4) * 8|16`),
//!   never from the header's pitch fields (old tools write them wrong); the level count is the
//!   header's `dwMipMapCount` even when `DDSD_MIPMAPCOUNT` is unset, 0 meaning 1 (as `DirectXTex`
//!   `DDSTextureLoader` reads it), capped at the full chain; levels missing from a truncated file
//!   count as missing.
//! - KTX2 (Khronos KTX 2.0) BC1-BC5/BC7 without supercompression, parsed with the `ktx2` crate.
//! - Not block-compressed (uncompressed DDS, KTX2 of another format, or any other container):
//!   [`Rewrap::Cook`], decode and run the T1 cook path. Cubemaps, volumes, arrays, BC6H, signed
//!   BC4/BC5, Basis Universal and supercompressed KTX2: [`Rewrap::Skip`], served as shipped.
//!
//! **Role fit** ([`target_format`]): colour takes BC1/BC2/BC3/BC7; normals with the engine's +Y
//! convention take BC5 or three-channel BC1/BC3/BC7; a single-channel mask of red takes BC4;
//! packed data takes BC1/BC3/BC7. **Layout changes are block-exact remaps** ([`crate::remap`],
//! slice T2b): the role's canonical layout ([`canonical`]: a `SourceY::Down` normal's green
//! complemented, a normal's alpha dropped, a mask's channel moved to R and complemented if
//! asked) composed with a pack's [`ChannelMap`] ([`rewrap_mapped`]) runs on the blocks, so a
//! DirectX BC1 normal stays BC1, a DXT5 `_nh` splits into a BC1 normal and a BC4 depth. What has
//! no exact block operation is cooked ([`CookReason`]). **Colour space follows the role**, not
//! the file's flag: colour is sRGB, everything else UNORM. That is what Bevy's own DDS loader does
//! (its `is_srgb` setting picks the variant, `bevy_image` `dds.rs`), what the game's model textures load as today, and what
//! UE/Unity do (sRGB is a texture setting); many DDS writers label sRGB albedo `UNORM`.
//!
//! **Encoders for filled levels**: BC7, BC4, BC5, BC3 and opaque BC1 through Intel's ISPC kernels
//! (`intel_tex_2`, as the cook path); BC1 with punch-through alpha and BC2 through `texpresso`
//! (a libsquish port, cluster fit), because ISPC has no BC2 and its BC1 is opaque only. So every
//! filled level stays in the source format and the shipped levels are never touched (the
//! alternative, re-encoding the whole file to BC3/BC7 when no same-format encoder exists, is not
//! needed). Decoding: `texpresso` for BC1-BC5, `block_compression` for BC7.
//!
//! wgpu needs level 0 of a block-compressed texture to be a whole number of blocks: a source whose
//! level 0 isn't is a [`SkipReason::Unaligned`].

use block_compression::{BC7Settings, CompressionVariant, decode::decompress_blocks_as_rgba8};
use ddsfile::{Caps2, D3D10ResourceDimension, DxgiFormat, Header, Header10, MiscFlag};
use intel_tex_2::{RgbaSurface, bc1, bc3};
use ktx2::{Format, Header as Ktx2Header, LevelIndex, Reader};

use crate::cook::{
    AlphaUse, Channel, Role, SourceY, block_grid, encode_bc4, encode_bc5, linear_mip_chain,
    normal_mip_chain,
};
use crate::delight::check_len;
use crate::ktx2_out::{
    Bc7Quality, BlockFormat, MipLevel, encode_bc7, is_block_aligned, level_len, mip_chain,
    mip_count, preserve_coverage, write_ktx2,
};
use crate::remap::{ChannelMap, RemapError, Src, plan};

/// The file a block-compressed source came in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Container {
    Dds,
    Ktx2,
}

/// A parsed block-compressed source: its blocks as shipped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BcSource<'a> {
    pub container: Container,
    /// The block format, its linear (UNORM) variant; the file's colour space is `declared_srgb`.
    pub format: BlockFormat,
    /// The colour space the file declares: DXGI `_SRGB` / `_UNORM` or the KTX2 `vkFormat`; `None`
    /// for legacy `FourCC` DDS and DXGI `TYPELESS`, which declare none.
    pub declared_srgb: Option<bool>,
    pub width: u32,
    pub height: u32,
    /// The shipped levels, level 0 first, each exactly [`level_len`] bytes. A truncated chain
    /// stops at its last complete level; never empty.
    pub levels: Vec<&'a [u8]>,
}

/// Why a texture can't be rewrapped and goes through the cook path instead (decode, then
/// [`crate::cook_levels`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CookReason {
    /// Not block-compressed: uncompressed DDS, KTX2 of another format, another container.
    NotBlockCompressed,
    /// Block-compressed, in a format that doesn't suit the role (`target_format`) or can't hold
    /// its layout without re-encoding ([`RemapError::Unrepresentable`]).
    FormatForRole(BlockFormat),
    /// The layout change has no block-exact operation for this source (BC7 complement, BC1
    /// punch-through complement; never `Unrepresentable`, which is `FormatForRole`): decode,
    /// [`ChannelMap::apply_rgba`], cook.
    NotBlockExact(RemapError),
}

/// Why a texture is served as shipped, neither rewrapped nor cooked (counted by the bake).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkipReason {
    /// Level 0 isn't a whole number of 4x4 blocks (wgpu rejects it as block-compressed).
    Unaligned { width: u32, height: u32 },
    /// A kind of texture this crate doesn't cook: cubemap, volume, array, BC6H, signed BC4/BC5,
    /// Basis Universal.
    Unsupported(&'static str),
    /// A supercompressed KTX2 (zstd, zlib, BasisLZ): already a runtime file; served as shipped.
    Supercompressed,
    /// The file doesn't parse, or level 0 is incomplete.
    Malformed(String),
}

/// A rewrapped texture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rewrapped {
    /// The KTX2 file: the shipped levels' bytes unchanged, then the filled levels.
    pub ktx2: Vec<u8>,
    pub format: BlockFormat,
    pub width: u32,
    pub height: u32,
    /// Levels in the file: the full chain down to 1x1.
    pub levels: u32,
    /// Of those, the levels copied from the source (the rest were filled).
    pub shipped: u32,
    /// The source is a KTX2 already in `format` with the full chain: it can be indexed as-is
    /// (textures.md §5 "Authored beats derived").
    pub as_shipped: bool,
    /// The blocks went through a block-exact remap ([`crate::remap`]): Y flip, channel swap,
    /// dropped alpha or a split-out channel.
    pub remapped: bool,
}

/// The outcome of [`rewrap`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rewrap {
    Rewrapped(Rewrapped),
    Cook(CookReason),
    Skip(SkipReason),
}

const DDS_MAGIC: &[u8; 4] = b"DDS ";
const KTX2_MAGIC: [u8; 12] = [
    0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, b'\r', b'\n', 0x1A, b'\n',
];
/// Magic (4) + `DDS_HEADER` (124).
const DDS_DATA: usize = 128;
/// `DDS_HEADER_DXT10`.
const DX10_LEN: usize = 20;

const fn fourcc(code: [u8; 4]) -> u32 {
    u32::from_le_bytes(code)
}

/// `width` x `height` at `level` (each axis halves, rounded down, never below 1).
fn level_size(width: u32, height: u32, level: usize) -> (u32, u32) {
    let shift = u32::try_from(level).unwrap_or(u32::MAX).min(31);
    ((width >> shift).max(1), (height >> shift).max(1))
}

/// `format` in the given colour space (BC4 and BC5 have only a linear variant).
pub const fn with_srgb(format: BlockFormat, srgb: bool) -> BlockFormat {
    use BlockFormat as F;
    match (format, srgb) {
        (F::Bc1 | F::Bc1Srgb, false) => F::Bc1,
        (F::Bc1 | F::Bc1Srgb, true) => F::Bc1Srgb,
        (F::Bc2 | F::Bc2Srgb, false) => F::Bc2,
        (F::Bc2 | F::Bc2Srgb, true) => F::Bc2Srgb,
        (F::Bc3 | F::Bc3Srgb, false) => F::Bc3,
        (F::Bc3 | F::Bc3Srgb, true) => F::Bc3Srgb,
        (F::Bc7 | F::Bc7Srgb, false) => F::Bc7,
        (F::Bc7 | F::Bc7Srgb, true) => F::Bc7Srgb,
        (F::Bc4, _) => F::Bc4,
        (F::Bc5, _) => F::Bc5,
    }
}

/// Parses a DDS or KTX2 file. `Ok(None)`: not block-compressed (cook it); `Err`: served as shipped.
///
/// # Errors
/// A [`SkipReason`] for files this crate doesn't cook or can't parse.
pub fn read_source(bytes: &[u8]) -> Result<Option<BcSource<'_>>, SkipReason> {
    if bytes.starts_with(DDS_MAGIC) {
        read_dds(bytes)
    } else if bytes.starts_with(&KTX2_MAGIC) {
        read_ktx2(bytes)
    } else {
        Ok(None)
    }
}

fn read_dds(bytes: &[u8]) -> Result<Option<BcSource<'_>>, SkipReason> {
    let malformed = |error: ddsfile::Error| SkipReason::Malformed(format!("DDS header: {error}"));
    let header = Header::read(&bytes[DDS_MAGIC.len()..]).map_err(malformed)?;
    if header.caps2.contains(Caps2::CUBEMAP) {
        return Err(SkipReason::Unsupported("cubemap"));
    }
    if header.caps2.contains(Caps2::VOLUME) || header.depth.is_some_and(|depth| depth > 1) {
        return Err(SkipReason::Unsupported("volume texture"));
    }
    let mut data = DDS_DATA;
    let (format, declared_srgb) = match header.spf.fourcc.as_ref().map(|code| code.0) {
        None => return Ok(None),
        Some(code) if code == fourcc(*b"DX10") => {
            let header10 =
                Header10::read(bytes.get(DDS_DATA..).unwrap_or_default()).map_err(malformed)?;
            data += DX10_LEN;
            if header10.misc_flag.contains(MiscFlag::TEXTURECUBE) {
                return Err(SkipReason::Unsupported("cubemap"));
            }
            if header10.resource_dimension != D3D10ResourceDimension::Texture2D {
                return Err(SkipReason::Unsupported("not a 2D texture"));
            }
            if header10.array_size > 1 {
                return Err(SkipReason::Unsupported("texture array"));
            }
            match dxgi_format(header10.dxgi_format)? {
                Some(format) => format,
                None => return Ok(None),
            }
        }
        Some(code) => match legacy_format(code)? {
            Some(format) => (format, None),
            None => return Ok(None),
        },
    };
    let (width, height) = (header.width, header.height);
    if width == 0 || height == 0 {
        return Err(SkipReason::Malformed(format!("{width}x{height} DDS")));
    }
    // `dwMipMapCount` at file offset 28, whatever `DDSD_MIPMAPCOUNT` says (DDSTextureLoader).
    let declared = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
    let count = declared.clamp(1, mip_count(width, height)) as usize;
    let mut levels = Vec::with_capacity(count);
    for level in 0..count {
        let (w, h) = level_size(width, height, level);
        let Some(blocks) = bytes.get(data..data + level_len(format, w, h)) else {
            break;
        };
        levels.push(blocks);
        data += blocks.len();
    }
    if levels.is_empty() {
        return Err(SkipReason::Malformed("DDS level 0 truncated".into()));
    }
    Ok(Some(BcSource {
        container: Container::Dds,
        format,
        declared_srgb,
        width,
        height,
        levels,
    }))
}

/// A legacy (pre-DX10) `FourCC`'s block format; `None` for the other codes (float and packed
/// uncompressed formats).
fn legacy_format(code: u32) -> Result<Option<BlockFormat>, SkipReason> {
    Ok(Some(match code {
        c if c == fourcc(*b"DXT1") => BlockFormat::Bc1,
        // DXT2 is DXT3 with premultiplied colour: the same blocks.
        c if c == fourcc(*b"DXT2") || c == fourcc(*b"DXT3") => BlockFormat::Bc2,
        c if c == fourcc(*b"DXT4") || c == fourcc(*b"DXT5") => BlockFormat::Bc3,
        c if c == fourcc(*b"ATI1") || c == fourcc(*b"BC4U") => BlockFormat::Bc4,
        c if c == fourcc(*b"ATI2") || c == fourcc(*b"BC5U") => BlockFormat::Bc5,
        c if c == fourcc(*b"BC4S") || c == fourcc(*b"BC5S") => {
            return Err(SkipReason::Unsupported("signed BC4/BC5"));
        }
        _ => return Ok(None),
    }))
}

/// A DXGI format's block format and declared colour space; `None` for formats that aren't
/// block-compressed.
fn dxgi_format(format: DxgiFormat) -> Result<Option<(BlockFormat, Option<bool>)>, SkipReason> {
    use BlockFormat as F;
    use DxgiFormat as D;
    Ok(Some(match format {
        D::BC1_Typeless => (F::Bc1, None),
        D::BC1_UNorm => (F::Bc1, Some(false)),
        D::BC1_UNorm_sRGB => (F::Bc1, Some(true)),
        D::BC2_Typeless => (F::Bc2, None),
        D::BC2_UNorm => (F::Bc2, Some(false)),
        D::BC2_UNorm_sRGB => (F::Bc2, Some(true)),
        D::BC3_Typeless => (F::Bc3, None),
        D::BC3_UNorm => (F::Bc3, Some(false)),
        D::BC3_UNorm_sRGB => (F::Bc3, Some(true)),
        D::BC4_Typeless => (F::Bc4, None),
        D::BC4_UNorm => (F::Bc4, Some(false)),
        D::BC5_Typeless => (F::Bc5, None),
        D::BC5_UNorm => (F::Bc5, Some(false)),
        D::BC7_Typeless => (F::Bc7, None),
        D::BC7_UNorm => (F::Bc7, Some(false)),
        D::BC7_UNorm_sRGB => (F::Bc7, Some(true)),
        D::BC4_SNorm | D::BC5_SNorm => return Err(SkipReason::Unsupported("signed BC4/BC5")),
        D::BC6H_Typeless | D::BC6H_UF16 | D::BC6H_SF16 => {
            return Err(SkipReason::Unsupported("BC6H (HDR)"));
        }
        _ => return Ok(None),
    }))
}

/// KTX2 `vkFormat`s read, with their block format and colour space (BC1 RGB and RGBA have the
/// same blocks; we write RGBA).
const KTX2_FORMATS: [(Format, BlockFormat, bool); 12] = [
    (Format::BC1_RGB_UNORM_BLOCK, BlockFormat::Bc1, false),
    (Format::BC1_RGB_SRGB_BLOCK, BlockFormat::Bc1, true),
    (Format::BC1_RGBA_UNORM_BLOCK, BlockFormat::Bc1, false),
    (Format::BC1_RGBA_SRGB_BLOCK, BlockFormat::Bc1, true),
    (Format::BC2_UNORM_BLOCK, BlockFormat::Bc2, false),
    (Format::BC2_SRGB_BLOCK, BlockFormat::Bc2, true),
    (Format::BC3_UNORM_BLOCK, BlockFormat::Bc3, false),
    (Format::BC3_SRGB_BLOCK, BlockFormat::Bc3, true),
    (Format::BC4_UNORM_BLOCK, BlockFormat::Bc4, false),
    (Format::BC5_UNORM_BLOCK, BlockFormat::Bc5, false),
    (Format::BC7_UNORM_BLOCK, BlockFormat::Bc7, false),
    (Format::BC7_SRGB_BLOCK, BlockFormat::Bc7, true),
];

fn read_ktx2(bytes: &[u8]) -> Result<Option<BcSource<'_>>, SkipReason> {
    let reader =
        Reader::new(bytes).map_err(|error| SkipReason::Malformed(format!("KTX2: {error:?}")))?;
    let header = reader.header();
    let Some(vk_format) = header.format else {
        return Err(SkipReason::Unsupported(
            "KTX2 without a vkFormat (Basis Universal)",
        ));
    };
    if header.supercompression_scheme.is_some() {
        return Err(SkipReason::Supercompressed);
    }
    if header.face_count > 1 {
        return Err(SkipReason::Unsupported("cubemap"));
    }
    if header.pixel_depth > 1 {
        return Err(SkipReason::Unsupported("volume texture"));
    }
    if header.layer_count > 1 {
        return Err(SkipReason::Unsupported("texture array"));
    }
    if [Format::BC4_SNORM_BLOCK, Format::BC5_SNORM_BLOCK].contains(&vk_format) {
        return Err(SkipReason::Unsupported("signed BC4/BC5"));
    }
    if [Format::BC6H_UFLOAT_BLOCK, Format::BC6H_SFLOAT_BLOCK].contains(&vk_format) {
        return Err(SkipReason::Unsupported("BC6H (HDR)"));
    }
    let Some(&(_, format, srgb)) = KTX2_FORMATS.iter().find(|(vk, ..)| *vk == vk_format) else {
        return Ok(None);
    };
    let (width, height) = (header.pixel_width, header.pixel_height.max(1));
    if width == 0 {
        return Err(SkipReason::Malformed("KTX2 of width 0".into()));
    }
    let count = mip_count(width, height) as usize;
    let mut levels = Vec::with_capacity(count);
    // The level index follows the header (KTX2 §3.8); `Reader::new` checked its bounds, but its
    // `levels()` borrow the reader, so the slices are taken from `bytes` here.
    let shipped = (header.level_count.max(1) as usize).min(count);
    for level in 0..shipped {
        let entry = bytes
            .get(Ktx2Header::LENGTH + level * LevelIndex::LENGTH..)
            .and_then(|rest| rest.first_chunk::<{ LevelIndex::LENGTH }>())
            .map(LevelIndex::from_bytes);
        let data = entry.and_then(|entry| {
            let start = usize::try_from(entry.byte_offset).ok()?;
            let len = usize::try_from(entry.byte_length).ok()?;
            bytes.get(start..start.checked_add(len)?)
        });
        let (w, h) = level_size(width, height, level);
        let expected = level_len(format, w, h);
        match data {
            Some(data) if data.len() == expected => levels.push(data),
            _ => {
                return Err(SkipReason::Malformed(format!(
                    "KTX2 level {level}: {w}x{h} {format:?} needs {expected} bytes"
                )));
            }
        }
    }
    if levels.is_empty() {
        return Err(SkipReason::Malformed("KTX2 without levels".into()));
    }
    Ok(Some(BcSource {
        container: Container::Ktx2,
        format,
        declared_srgb: Some(srgb),
        width,
        height,
        levels,
    }))
}

/// The format a block-compressed source keeps for `role` (module docs, "Role fit"): the source's
/// blocks in the role's colour space. Layout is not this function's concern: a normal's Y
/// convention and a mask's channel are remapped first ([`canonical`]), so it is asked about the
/// canonical role (`Normal` either way, `Mask { channel: R, invert: false }`).
///
/// # Errors
/// The [`CookReason`] when the source must be decoded and cooked instead.
pub fn target_format(source: BlockFormat, role: Role) -> Result<BlockFormat, CookReason> {
    use BlockFormat as F;
    let linear = with_srgb(source, false);
    let fits = match role {
        Role::Color { .. } => matches!(linear, F::Bc1 | F::Bc2 | F::Bc3 | F::Bc7),
        Role::Normal { .. } => matches!(linear, F::Bc5 | F::Bc1 | F::Bc3 | F::Bc7),
        Role::Mask { channel, invert } => linear == F::Bc4 && channel == Channel::R && !invert,
        Role::Packed => matches!(linear, F::Bc1 | F::Bc3 | F::Bc7),
    };
    if fits {
        Ok(with_srgb(linear, matches!(role, Role::Color { .. })))
    } else {
        Err(CookReason::FormatForRole(linear))
    }
}

/// A role's canonical layout as a map from the as-shipped channels, and the role the output
/// then has: a `SourceY::Down` normal → green complemented, `Up`; a normal's B (Z, rebuilt from
/// XY for two-channel formats) and A are unused, so BC2/BC3 normals drop to BC1; a mask of any
/// channel → that channel (complemented if `invert`) in R, `Mask { R, false }`. Colour and
/// packed data keep every channel.
pub const fn canonical(role: Role) -> (ChannelMap, Role) {
    match role {
        Role::Color { .. } | Role::Packed => (ChannelMap::IDENTITY, role),
        Role::Normal { source_y } => {
            let y = match source_y {
                SourceY::Up => Src::Channel(Channel::G),
                SourceY::Down => Src::Inverted(Channel::G),
            };
            (
                ChannelMap([Src::Channel(Channel::R), y, Src::Unused, Src::Unused]),
                Role::Normal {
                    source_y: SourceY::Up,
                },
            )
        }
        Role::Mask { channel, invert } => {
            let src = if invert {
                Src::Inverted(channel)
            } else {
                Src::Channel(channel)
            };
            (
                ChannelMap([src, Src::Unused, Src::Unused, Src::Unused]),
                Role::Mask {
                    channel: Channel::R,
                    invert: false,
                },
            )
        }
    }
}

/// Rewraps a DDS or KTX2 block-compressed texture into KTX2 for `role`, filling missing levels
/// (module docs): [`rewrap_mapped`] with the source taken as laid out like the role.
pub fn rewrap(bytes: &[u8], role: Role) -> Rewrap {
    rewrap_mapped(bytes, &ChannelMap::IDENTITY, role)
}

/// Rewraps a texture whose channels are laid out as `layout` says (output channel ← shipped
/// channel; a pack profile's data, [`crate::profile`]) into `role`'s canonical layout: `layout`
/// then [`canonical`]`(role)`, run on the blocks when exact ([`crate::remap`]), then the
/// shipped levels rewrapped and missing ones filled as [`rewrap`] does.
pub fn rewrap_mapped(bytes: &[u8], layout: &ChannelMap, role: Role) -> Rewrap {
    let source = match read_source(bytes) {
        Ok(Some(source)) => source,
        Ok(None) => return Rewrap::Cook(CookReason::NotBlockCompressed),
        Err(reason) => return Rewrap::Skip(reason),
    };
    let (to_canonical, role) = canonical(role);
    let remap = match plan(source.format, &layout.then(to_canonical)) {
        Ok(remap) => remap,
        Err(RemapError::Unrepresentable) => {
            return Rewrap::Cook(CookReason::FormatForRole(with_srgb(source.format, false)));
        }
        Err(error) => return Rewrap::Cook(CookReason::NotBlockExact(error)),
    };
    let Ok(format) = target_format(remap.output, role) else {
        let linear = with_srgb(source.format, false);
        return Rewrap::Cook(CookReason::FormatForRole(linear));
    };
    let (width, height) = (source.width, source.height);
    if !is_block_aligned(width, height) {
        return Rewrap::Skip(SkipReason::Unaligned { width, height });
    }
    let remapped: Vec<Vec<u8>> = if remap.is_identity() {
        Vec::new()
    } else {
        match source
            .levels
            .iter()
            .map(|level| remap.level(level))
            .collect()
        {
            Ok(levels) => levels,
            Err(error) => return Rewrap::Cook(CookReason::NotBlockExact(error)),
        }
    };
    let shipped_levels: Vec<&[u8]> = if remap.is_identity() {
        source.levels.clone()
    } else {
        remapped.iter().map(Vec::as_slice).collect()
    };
    let full = mip_count(width, height) as usize;
    let shipped = shipped_levels.len();
    let filled = if shipped < full {
        let (w, h) = level_size(width, height, shipped - 1);
        fill_levels(format, role, shipped_levels[shipped - 1], w, h)
    } else {
        Vec::new()
    };
    debug_assert_eq!(shipped + filled.len(), full, "filled down to 1x1");
    let levels: Vec<&[u8]> = shipped_levels
        .iter()
        .copied()
        .chain(filled.iter().map(Vec::as_slice))
        .collect();
    let declared = with_srgb(source.format, source.declared_srgb == Some(true));
    Rewrap::Rewrapped(Rewrapped {
        ktx2: write_ktx2(format, width, height, &levels),
        format,
        width,
        height,
        levels: u32::try_from(full).unwrap_or(u32::MAX),
        shipped: u32::try_from(shipped).unwrap_or(u32::MAX),
        as_shipped: source.container == Container::Ktx2
            && filled.is_empty()
            && remap.is_identity()
            && declared == format,
        remapped: !remap.is_identity(),
    })
}

/// The levels below `last` (a `width` x `height` level in `format`) down to 1x1, encoded in
/// `format`: `last` decoded, filtered with the role's mip filter, every level after it encoded.
fn fill_levels(
    format: BlockFormat,
    role: Role,
    last: &[u8],
    width: u32,
    height: u32,
) -> Vec<Vec<u8>> {
    let rgba = decode_level(format, last, width, height);
    let chain: Vec<MipLevel> = match role {
        Role::Color { alpha } => {
            let mut chain = mip_chain(&rgba, width, height, format.is_srgb());
            if let AlphaUse::Test(cutoff) = alpha {
                preserve_coverage(&mut chain, cutoff);
            }
            chain
        }
        Role::Normal { source_y } => normal_mip_chain(&rgba, width, height, source_y),
        Role::Mask { .. } | Role::Packed => linear_mip_chain(&rgba, width, height),
    };
    chain
        .iter()
        .skip(1)
        .map(|level| {
            encode_level(
                format,
                &level.rgba,
                level.width,
                level.height,
                Bc7Quality::Basic,
            )
        })
        .collect()
}

/// Decodes one level of `format` blocks to RGBA8, row by row. BC4 decodes to grey (R in R, G, B),
/// BC5 to (R, G, 0, 255), BC1's punch-through texels to (0, 0, 0, 0).
///
/// # Panics
/// If `blocks.len()` isn't [`level_len`]`(format, width, height)` or the level has no area.
pub fn decode_level(format: BlockFormat, blocks: &[u8], width: u32, height: u32) -> Vec<u8> {
    use BlockFormat as F;
    assert!(width > 0 && height > 0, "a level needs a texture with area");
    assert_eq!(
        blocks.len(),
        level_len(format, width, height),
        "{width}x{height} {format:?} blocks"
    );
    let (w, h) = (width as usize, height as usize);
    let squish = match format {
        F::Bc1 | F::Bc1Srgb => Some(texpresso::Format::Bc1),
        F::Bc2 | F::Bc2Srgb => Some(texpresso::Format::Bc2),
        F::Bc3 | F::Bc3Srgb => Some(texpresso::Format::Bc3),
        F::Bc4 => Some(texpresso::Format::Bc4),
        F::Bc5 => Some(texpresso::Format::Bc5),
        F::Bc7 | F::Bc7Srgb => None,
    };
    let mut rgba = vec![0; w * h * 4];
    if let Some(squish) = squish {
        squish.decompress(blocks, w, h, &mut rgba);
        return rgba;
    }
    // BC7: decode whole blocks, then crop the padding.
    let (padded_w, padded_h) = (width.div_ceil(4) * 4, height.div_ceil(4) * 4);
    let mut padded = vec![0; padded_w as usize * padded_h as usize * 4];
    decompress_blocks_as_rgba8(
        CompressionVariant::BC7(BC7Settings::opaque_basic()),
        padded_w,
        padded_h,
        blocks,
        &mut padded,
    );
    for (row, out) in rgba.chunks_exact_mut(w * 4).enumerate() {
        out.copy_from_slice(&padded[row * padded_w as usize * 4..][..w * 4]);
    }
    rgba
}

/// Encodes one RGBA8 level in `format` ([`level_len`] bytes): BC7 ([`encode_bc7`] at `quality`),
/// BC5 from R and G, BC4 from R, BC3 and opaque BC1 through ISPC; BC1 with alpha below 255
/// (punch-through, a texel below 128 becomes transparent) and BC2 through `texpresso` cluster fit,
/// colour error weighted perceptually for sRGB formats, uniformly for linear data.
///
/// # Panics
/// If `rgba.len() != width * height * 4` or the level has no area.
pub fn encode_level(
    format: BlockFormat,
    rgba: &[u8],
    width: u32,
    height: u32,
    quality: Bc7Quality,
) -> Vec<u8> {
    use BlockFormat as F;
    let (w, h) = check_len(rgba, width, height);
    assert!(w > 0 && h > 0, "a level needs a texture with area");
    let texels = rgba.as_chunks::<4>().0;
    match format {
        F::Bc7 | F::Bc7Srgb => encode_bc7(rgba, width, height, quality),
        F::Bc5 => {
            let rg: Vec<u8> = texels
                .iter()
                .flat_map(|texel| [texel[0], texel[1]])
                .collect();
            encode_bc5(&rg, width, height)
        }
        F::Bc4 => {
            let r: Vec<u8> = texels.iter().map(|texel| texel[0]).collect();
            encode_bc4(&r, width, height)
        }
        F::Bc3 | F::Bc3Srgb => ispc(format, rgba, width, height, bc3::compress_blocks_into),
        F::Bc1 | F::Bc1Srgb if texels.iter().all(|texel| texel[3] == u8::MAX) => {
            ispc(format, rgba, width, height, bc1::compress_blocks_into)
        }
        F::Bc1 | F::Bc1Srgb | F::Bc2 | F::Bc2Srgb => {
            let squish = if matches!(format, F::Bc1 | F::Bc1Srgb) {
                texpresso::Format::Bc1
            } else {
                texpresso::Format::Bc2
            };
            let params = texpresso::Params {
                algorithm: texpresso::Algorithm::ClusterFit,
                weights: if format.is_srgb() {
                    texpresso::COLOUR_WEIGHTS_PERCEPTUAL
                } else {
                    texpresso::COLOUR_WEIGHTS_UNIFORM
                },
                weigh_colour_by_alpha: false,
            };
            let mut blocks = vec![0; level_len(format, width, height)];
            squish.compress(rgba, w, h, params, &mut blocks);
            blocks
        }
    }
}

/// An ISPC RGBA encoder over the level padded to whole blocks.
fn ispc(
    format: BlockFormat,
    rgba: &[u8],
    width: u32,
    height: u32,
    compress: fn(&RgbaSurface<'_>, &mut [u8]),
) -> Vec<u8> {
    let (data, padded_w, padded_h) = block_grid(rgba, 4, width, height);
    let surface = RgbaSurface {
        data: &data,
        width: padded_w,
        height: padded_h,
        stride: padded_w * 4,
    };
    let mut blocks = vec![0; level_len(format, width, height)];
    compress(&surface, &mut blocks);
    blocks
}

#[cfg(test)]
pub(crate) mod tests;
