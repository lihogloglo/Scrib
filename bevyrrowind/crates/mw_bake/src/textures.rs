//! CPU texture decoding for bake steps that resample or re-encode textures (the texture cook's
//! decode path, de-light, terrain layer arrays, far textures): level 0 of any texture file a
//! Morrowind or `OpenMW` pack ships, to RGBA8 (textures-adapter.md §9 "Source formats").
//!
//! - **The bytes decide, then the extension.** A `DDS ` or KTX2 signature is read as that
//!   container whatever the file is named, PNG and BMP by their signature (`image::guess_format`),
//!   TGA (it has none) by its extension.
//! - **Block-compressed** DDS (legacy `FourCC` and DX10 header) and KTX2, BC1-BC5 and BC7: parsed
//!   and decoded by the framework ([`sf_texture::read_source`], [`sf_texture::decode_level`]), the
//!   reader the texture cook's rewrap uses, so whatever rewraps also decodes when it has to be
//!   cooked instead (a BC7 normal that needs its Y flipped, a BC4 used as colour). Level 0 need not
//!   be whole blocks (the cook decides what to do with an unaligned size).
//! - **Uncompressed legacy DDS**: any `DDPF_RGB` / `DDPF_LUMINANCE` / `DDPF_ALPHA` layout of 8, 16,
//!   24 or 32 bits a texel, read by its bit masks (as `DirectXTex`'s legacy table and OSG's DDS
//!   plugin match them), `DDPF_PALETTEINDEXED8` (P8: a 256-entry RGBA palette after the header)
//!   and the D3D9 code 36 (`A16B16G16R16`) in `FourCC`.
//! - **Uncompressed DX10 DDS and KTX2**: the 8- and 16-bit UNORM formats (sRGB variants alike),
//!   5:6:5, 5:5:5:1, 4:4:4:4 and 10:10:10:2.
//! - **Not read** (an error, so the cook serves the file as shipped): float and HDR (BC6H, `R16F`,
//!   ...), signed and integer formats, video (YUY2), ETC/ASTC, Basis Universal, supercompressed
//!   KTX2, cubemaps, volumes and arrays, JPEG (no decoder in the tree).
//!
//! Channels follow D3D sampling: a missing colour channel reads 0 and a missing alpha 1, luminance
//! fills R, G and B, `A8` is (0, 0, 0, a). Rows are tightly packed, as the DDS and KTX2 references
//! store them: the DDS pitch field is ignored (old writers fill it wrong).
//!
//! Reuse: Bevy's DDS/KTX2 loaders only upload (block formats stay compressed; `bevy_image`
//! `dds.rs` rejects most legacy uncompressed layouts and maps `L8`/`A8L8` to integer formats), so
//! the bake can't use them. `ddsfile` and `ktx2` (in the tree through `bevy_image`) parse the
//! containers, `sf_texture` decodes blocks (`texpresso`, `block_compression`), `image` reads
//! TGA/BMP/PNG (RLE and palettized included).

use ddsfile::{Dds, DxgiFormat};
use ktx2::Format;
use sf_texture::{decode_level, read_source};

/// Extensions that may replace a texture reference, in preference order
/// (`sf_vfs::Vfs::resolve_preferring`). `dds` first is the original engine's
/// behavior; `ktx2`/`png` let modern texture mods (and the de-lit bake layer,
/// [`crate::delight`]) take the slot.
pub const TEXTURE_ALTERNATES: &[&str] = &["ktx2", "dds", "png"];

/// A decoded image, RGBA8, rows top to bottom.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rgba8 {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

const DDS_MAGIC: &[u8; 4] = b"DDS ";
const KTX2_MAGIC: &[u8; 12] = b"\xABKTX 20\xBB\r\n\x1A\n";
/// Magic (4) + `DDS_HEADER` (124): level 0 of a legacy DDS (or a P8 palette).
const DDS_DATA: usize = 128;
/// After the `DDS_HEADER_DXT10` extension (20).
const DX10_DATA: usize = 148;
/// A P8 palette: 256 entries of R, G, B, A (`PALETTEENTRY`, its flags byte read as alpha).
const PALETTE_LEN: usize = 1024;

/// Decodes the first mip of `bytes`; `path`'s extension names the format only when the bytes don't
/// (TGA).
pub fn decode_rgba(path: &str, bytes: &[u8]) -> Result<Rgba8, String> {
    if bytes.starts_with(DDS_MAGIC) || bytes.starts_with(KTX2_MAGIC) {
        return decode_gpu_container(bytes);
    }
    let ext = path.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).unwrap_or_default();
    let format = match image::guess_format(bytes) {
        Ok(format @ (image::ImageFormat::Png | image::ImageFormat::Bmp)) => format,
        Ok(image::ImageFormat::Jpeg) => return Err("JPEG: no decoder in the bake".into()),
        _ => match ext.as_str() {
            "tga" => image::ImageFormat::Tga,
            other => return Err(format!("unsupported texture format {other:?}")),
        },
    };
    let img = image::load_from_memory_with_format(bytes, format).map_err(|e| e.to_string())?.into_rgba8();
    Ok(Rgba8 { width: img.width(), height: img.height(), rgba: img.into_raw() })
}

/// A DDS or KTX2 file: block-compressed through the framework's reader, else uncompressed.
fn decode_gpu_container(bytes: &[u8]) -> Result<Rgba8, String> {
    if let Some(source) = read_source(bytes).map_err(|reason| format!("{reason:?}"))? {
        let (width, height) = (source.width, source.height);
        let rgba = decode_level(source.format, source.levels[0], width, height);
        return Ok(Rgba8 { width, height, rgba });
    }
    // `read_source` turned away cubemaps, volumes, arrays and supercompression already.
    if bytes.starts_with(DDS_MAGIC) { decode_dds(bytes) } else { decode_ktx2(bytes) }
}

/// An uncompressed texel layout: bytes a texel (little-endian) and the bit mask of R, G, B, A
/// (0: the channel is absent). `luminance` reads R's mask as grey for R, G and B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Layout {
    bytes: usize,
    masks: [u64; 4],
    luminance: bool,
}

impl Layout {
    const fn rgba(bytes: usize, r: u64, g: u64, b: u64, a: u64) -> Self {
        Self { bytes, masks: [r, g, b, a], luminance: false }
    }

    fn texel(self, px: &[u8]) -> [u8; 4] {
        let mut word = [0; 8];
        word[..self.bytes].copy_from_slice(px);
        let word = u64::from_le_bytes(word);
        let [r, g, b, a] = self.masks.map(|mask| channel(word, mask));
        let (r, a) = (r.unwrap_or(0), a.unwrap_or(u8::MAX));
        if self.luminance { [r, r, r, a] } else { [r, g.unwrap_or(0), b.unwrap_or(0), a] }
    }
}

/// The value under `mask` scaled to 8 bits, rounded (5 bits: `v * 255 / 31`); `None` for no mask.
fn channel(v: u64, mask: u64) -> Option<u8> {
    if mask == 0 {
        return None;
    }
    let shift = mask.trailing_zeros();
    let max = mask >> shift;
    let x = (v & mask) >> shift;
    Some(u8::try_from((x * 255 + max / 2) / max).unwrap_or(u8::MAX))
}

const RGBA8: Layout = Layout::rgba(4, 0xff, 0xff00, 0xff_0000, 0xff00_0000);
const BGRA8: Layout = Layout::rgba(4, 0xff_0000, 0xff00, 0xff, 0xff00_0000);
const BGRX8: Layout = Layout::rgba(4, 0xff_0000, 0xff00, 0xff, 0);
const RGB8: Layout = Layout::rgba(3, 0xff, 0xff00, 0xff_0000, 0);
const BGR8: Layout = Layout::rgba(3, 0xff_0000, 0xff00, 0xff, 0);
const RG8: Layout = Layout::rgba(2, 0xff, 0xff00, 0, 0);
const R8: Layout = Layout::rgba(1, 0xff, 0, 0, 0);
const A8: Layout = Layout::rgba(1, 0, 0, 0, 0xff);
const RGBA16: Layout = Layout::rgba(8, 0xffff, 0xffff << 16, 0xffff << 32, 0xffff << 48);
const RG16: Layout = Layout::rgba(4, 0xffff, 0xffff_0000, 0, 0);
const R16: Layout = Layout::rgba(2, 0xffff, 0, 0, 0);
const RGB10A2: Layout = Layout::rgba(4, 0x3ff, 0x3ff << 10, 0x3ff << 20, 0b11 << 30);
const R5G6B5: Layout = Layout::rgba(2, 0xf800, 0x07e0, 0x1f, 0);
const A1R5G5B5: Layout = Layout::rgba(2, 0x7c00, 0x03e0, 0x1f, 0x8000);
const A4R4G4B4: Layout = Layout::rgba(2, 0x0f00, 0xf0, 0xf, 0xf000);

/// `DDS_PIXELFORMAT.dwFlags` bits.
const DDPF_ALPHAPIXELS: u32 = 0x1;
const DDPF_ALPHA: u32 = 0x2;
const DDPF_FOURCC: u32 = 0x4;
const DDPF_PALETTEINDEXED8: u32 = 0x20;
const DDPF_RGB: u32 = 0x40;
const DDPF_LUMINANCE: u32 = 0x2_0000;
/// D3D9's `D3DFMT_A16B16G16R16`, written as a number in `FourCC`.
const D3DFMT_A16B16G16R16: u32 = 36;

/// The legacy header's layout, from the raw `DDS_PIXELFORMAT` (`ddsfile` drops the palette flag and
/// matches masks to a fixed list; masks read directly take any 8-32-bit layout).
fn legacy_layout(bytes: &[u8]) -> Result<Layout, String> {
    let at = |offset: usize| bytes.get(offset..offset + 4).map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let (flags, fourcc, bits) = (at(80), at(84), at(88));
    let [r, g, b, a] = [92, 96, 100, 104].map(|offset| u64::from(at(offset)));
    if flags & DDPF_FOURCC != 0 {
        return match fourcc {
            D3DFMT_A16B16G16R16 => Ok(RGBA16),
            code => Err(format!("unsupported DDS FourCC {:?}", String::from_utf8_lossy(&code.to_le_bytes()))),
        };
    }
    let bytes = match bits {
        8 | 16 | 24 | 32 => bits as usize / 8,
        _ => return Err(format!("unsupported {bits}-bit DDS")),
    };
    let alpha = if flags & (DDPF_ALPHAPIXELS | DDPF_ALPHA) == 0 { 0 } else { a };
    if flags & DDPF_LUMINANCE != 0 {
        Ok(Layout { bytes, masks: [r, 0, 0, alpha], luminance: true })
    } else if flags & DDPF_RGB != 0 {
        Ok(Layout::rgba(bytes, r, g, b, alpha))
    } else if flags & DDPF_ALPHA != 0 {
        Ok(Layout::rgba(bytes, 0, 0, 0, a))
    } else {
        Err(format!("unsupported DDS pixel format (flags {flags:#x})"))
    }
}

/// The DX10 header's uncompressed formats read.
fn dxgi_layout(format: DxgiFormat) -> Option<Layout> {
    use DxgiFormat as D;
    Some(match format {
        D::R8G8B8A8_Typeless | D::R8G8B8A8_UNorm | D::R8G8B8A8_UNorm_sRGB => RGBA8,
        D::B8G8R8A8_Typeless | D::B8G8R8A8_UNorm | D::B8G8R8A8_UNorm_sRGB => BGRA8,
        D::B8G8R8X8_Typeless | D::B8G8R8X8_UNorm | D::B8G8R8X8_UNorm_sRGB => BGRX8,
        D::R8G8_Typeless | D::R8G8_UNorm => RG8,
        D::R8_Typeless | D::R8_UNorm => R8,
        D::A8_UNorm => A8,
        D::R16G16B16A16_UNorm => RGBA16,
        D::R16G16_UNorm => RG16,
        D::R16_UNorm => R16,
        D::R10G10B10A2_Typeless | D::R10G10B10A2_UNorm => RGB10A2,
        D::B5G6R5_UNorm => R5G6B5,
        D::B5G5R5A1_UNorm => A1R5G5B5,
        D::B4G4R4A4_UNorm => A4R4G4B4,
        _ => return None,
    })
}

/// The KTX2 `vkFormat`s read uncompressed (Vulkan names packed formats high bit first).
const KTX2_LAYOUTS: [(Format, Layout); 21] = [
    (Format::R8G8B8A8_UNORM, RGBA8),
    (Format::R8G8B8A8_SRGB, RGBA8),
    (Format::A8B8G8R8_UNORM_PACK32, RGBA8),
    (Format::A8B8G8R8_SRGB_PACK32, RGBA8),
    (Format::B8G8R8A8_UNORM, BGRA8),
    (Format::B8G8R8A8_SRGB, BGRA8),
    (Format::R8G8B8_UNORM, RGB8),
    (Format::R8G8B8_SRGB, RGB8),
    (Format::B8G8R8_UNORM, BGR8),
    (Format::B8G8R8_SRGB, BGR8),
    (Format::R8G8_UNORM, RG8),
    (Format::R8G8_SRGB, RG8),
    (Format::R8_UNORM, R8),
    (Format::R8_SRGB, R8),
    (Format::R16G16B16A16_UNORM, RGBA16),
    (Format::R16G16_UNORM, RG16),
    (Format::R16_UNORM, R16),
    (Format::A2B10G10R10_UNORM_PACK32, RGB10A2),
    (Format::R5G6B5_UNORM_PACK16, R5G6B5),
    (Format::A1R5G5B5_UNORM_PACK16, A1R5G5B5),
    (Format::B4G4R4A4_UNORM_PACK16, Layout::rgba(2, 0xf0, 0x0f00, 0xf000, 0xf)),
];

fn decode_dds(bytes: &[u8]) -> Result<Rgba8, String> {
    let dds = Dds::read(bytes).map_err(|e| e.to_string())?;
    let (width, height) = (dds.get_width(), dds.get_height());
    if let Some(header10) = &dds.header10 {
        let format = header10.dxgi_format;
        let layout = dxgi_layout(format).ok_or_else(|| format!("unsupported DDS format {format:?}"))?;
        return unpack(layout, bytes.get(DX10_DATA..).unwrap_or_default(), width, height);
    }
    let flags = bytes.get(80..84).map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if flags & DDPF_PALETTEINDEXED8 != 0 {
        return decode_p8(bytes.get(DDS_DATA..).unwrap_or_default(), width, height);
    }
    unpack(legacy_layout(bytes)?, bytes.get(DDS_DATA..).unwrap_or_default(), width, height)
}

/// A P8 DDS's payload: the palette, then one index a texel.
fn decode_p8(data: &[u8], width: u32, height: u32) -> Result<Rgba8, String> {
    let texels = texel_count(width, height)?;
    let (Some(palette), Some(indices)) = (data.get(..PALETTE_LEN), data.get(PALETTE_LEN..PALETTE_LEN + texels)) else {
        return Err("truncated P8 DDS".into());
    };
    let rgba = indices.iter().flat_map(|&i| palette[usize::from(i) * 4..][..4].iter().copied()).collect();
    Ok(Rgba8 { width, height, rgba })
}

fn decode_ktx2(bytes: &[u8]) -> Result<Rgba8, String> {
    let reader = ktx2::Reader::new(bytes).map_err(|e| format!("KTX2: {e:?}"))?;
    let header = reader.header();
    let format = header.format.ok_or("KTX2 without a vkFormat")?;
    let layout = KTX2_LAYOUTS
        .iter()
        .find(|(vk, _)| *vk == format)
        .map(|(_, layout)| *layout)
        .ok_or_else(|| format!("unsupported KTX2 format {format:?}"))?;
    let level = reader.levels().next().ok_or("KTX2 without levels")?;
    unpack(layout, level.data, header.pixel_width, header.pixel_height.max(1))
}

fn texel_count(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 {
        return Err(format!("{width}x{height} texture"));
    }
    (width as usize).checked_mul(height as usize).ok_or_else(|| format!("{width}x{height} texture"))
}

/// Level 0 of `layout` texels, rows tightly packed.
fn unpack(layout: Layout, data: &[u8], width: u32, height: u32) -> Result<Rgba8, String> {
    let need = texel_count(width, height)?.checked_mul(layout.bytes).ok_or("texture too large")?;
    let data = data.get(..need).ok_or_else(|| format!("truncated texture: {} of {need} bytes", data.len()))?;
    let rgba = data.chunks_exact(layout.bytes).flat_map(|px| layout.texel(px)).collect();
    Ok(Rgba8 { width, height, rgba })
}

/// Whether a blended texture's alpha is a cutout (leaf cards: holes plus
/// solid texels, a thin soft rim) rather than translucency (glass, potions:
/// broad partial alpha). Cutouts draw alpha-tested ([`mw_nif::Cutout`]).
/// Thresholds: alpha <= 0.1 is a hole, >= 0.9 solid. Two shapes count:
/// - mostly solid (banners, hair): soft texels at most a quarter of the
///   covered ones;
/// - sparse atlases (leaves, twigs, ferns: >= 10% holes): soft texels at
///   most 1.25x the solid ones. Thin twigs are mostly filtered rim
///   (`tx_bc_leaves_02`: 45% holes, 32% solid, 23% soft), which the first
///   rule alone rejected, so those trees stayed blended and cast no
///   shadow (LM-1). Translucents keep failing both: glass, waterfalls and
///   crystals have no solid texels to speak of, decals and glows (chalk,
///   flares, webbing) are almost all soft, lava/oil crusts have few holes.
pub fn alpha_is_cutout(image: &Rgba8) -> bool {
    let (mut holes, mut solid, mut soft) = (0usize, 0usize, 0usize);
    for a in image.rgba.as_chunks::<4>().0.iter().map(|p| p[3]) {
        match a {
            0..=25 => holes += 1,
            230..=255 => solid += 1,
            _ => soft += 1,
        }
    }
    let total = holes + solid + soft;
    let mostly_solid = soft * 4 <= solid + soft;
    let sparse_atlas = holes * 10 >= total && soft * 4 <= solid * 5;
    holes > 0 && solid > 0 && (mostly_solid || sparse_atlas)
}

#[cfg(test)]
mod tests;
