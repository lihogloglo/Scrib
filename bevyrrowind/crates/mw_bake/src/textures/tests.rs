use ddsfile::{AlphaMode, Caps2, D3D10ResourceDimension, D3DFormat, Dds, DxgiFormat, NewD3dParams, NewDxgiParams};
use image::ExtendedColorType;
use sf_texture::{Bc7Quality, BlockFormat, decode_level, encode_level, write_ktx2};

use super::*;

fn write(dds: &Dds) -> Vec<u8> {
    let mut out = Vec::new();
    dds.write(&mut out).unwrap();
    out
}

fn dds_caps(format: D3DFormat, data: &[u8], w: u32, h: u32, caps2: Option<Caps2>) -> Vec<u8> {
    let mut dds =
        Dds::new_d3d(NewD3dParams { height: h, width: w, depth: None, format, mipmap_levels: None, caps2 }).unwrap();
    dds.data[..data.len()].copy_from_slice(data);
    write(&dds)
}

fn dds(format: D3DFormat, data: &[u8], w: u32, h: u32) -> Vec<u8> {
    dds_caps(format, data, w, h, None)
}

fn dx10(format: DxgiFormat, data: &[u8], w: u32, h: u32) -> Vec<u8> {
    let mut dds = Dds::new_dxgi(NewDxgiParams {
        height: h,
        width: w,
        depth: None,
        format,
        mipmap_levels: None,
        array_layers: None,
        caps2: None,
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Unknown,
    })
    .unwrap();
    dds.data[..data.len()].copy_from_slice(data);
    write(&dds)
}

/// A one-level, uncompressed KTX2 with an empty DFD (just its 4-byte total size; the reader checks
/// only its bounds).
fn ktx2(vk_format: u32, w: u32, h: u32, data: &[u8]) -> Vec<u8> {
    let mut out = KTX2_MAGIC.to_vec();
    // vkFormat, typeSize, width, height, depth, layers, faces, levels, supercompression.
    for v in [vk_format, 1, w, h, 0, 0, 1, 1, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    // DFD at 104 (4 bytes), no key/value data (u32 offset, length), no SGD (u64 offset, length).
    for v in [104u32, 4, 0, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&[0; 16]);
    let len = data.len() as u64;
    for v in [108, len, len] {
        out.extend_from_slice(&u64::to_le_bytes(v));
    }
    out.extend_from_slice(&4u32.to_le_bytes());
    out.extend_from_slice(data);
    out
}

fn texel(path: &str, bytes: &[u8]) -> [u8; 4] {
    let img = decode_rgba(path, bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
    assert_eq!((img.width, img.height), (1, 1), "{path}");
    img.rgba[..4].try_into().unwrap()
}

#[test]
fn decodes_uncompressed_bgra() {
    let bytes = dds(D3DFormat::A8R8G8B8, &[10, 20, 30, 40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 2, 2);
    let img = decode_rgba("textures/a.dds", &bytes).unwrap();
    assert_eq!((img.width, img.height), (2, 2));
    assert_eq!(&img.rgba[..8], &[30, 20, 10, 40, 3, 2, 1, 4]);
}

#[test]
fn decodes_16bit_cutout_dds_formats() {
    // A1R5G5B5: pixel 0 = opaque red (0xFC00), pixel 1 = transparent hole (0x0000).
    let a1r5g5b5 = [0x00, 0xFC, 0x00, 0x00];
    let img = decode_rgba("tx_banner_temple_04.dds", &dds(D3DFormat::A1R5G5B5, &a1r5g5b5, 2, 1)).unwrap();
    assert_eq!(&img.rgba[..8], &[255, 0, 0, 255, 0, 0, 0, 0]);
    assert!(alpha_is_cutout(&img));

    // A4R4G4B4: pixel 0 = opaque green (0xF0F0), pixel 1 = transparent hole (0x0000).
    let a4r4g4b4 = [0xF0, 0xF0, 0x00, 0x00];
    let img4 = decode_rgba("tx_rope_net_01.dds", &dds(D3DFormat::A4R4G4B4, &a4r4g4b4, 2, 1)).unwrap();
    assert_eq!(&img4.rgba[..8], &[0, 255, 0, 255, 0, 0, 0, 0]);
    assert!(alpha_is_cutout(&img4));
}

/// Every legacy (pre-DX10) uncompressed layout, by its masks (D3D9 names: high bits first).
#[test]
fn decodes_every_legacy_uncompressed_dds_layout() {
    let cases: [(D3DFormat, &[u8], [u8; 4]); 21] = [
        (D3DFormat::A8R8G8B8, &[30, 20, 10, 40], [10, 20, 30, 40]),
        (D3DFormat::X8R8G8B8, &[30, 20, 10, 99], [10, 20, 30, 255]),
        (D3DFormat::A8B8G8R8, &[10, 20, 30, 40], [10, 20, 30, 40]),
        (D3DFormat::X8B8G8R8, &[10, 20, 30, 99], [10, 20, 30, 255]),
        (D3DFormat::R8G8B8, &[30, 20, 10], [10, 20, 30, 255]),
        (D3DFormat::R5G6B5, &0xF800u16.to_le_bytes(), [255, 0, 0, 255]),
        (D3DFormat::R5G6B5, &0x0410u16.to_le_bytes(), [0, 130, 132, 255]),
        (D3DFormat::X1R5G5B5, &0x83E0u16.to_le_bytes(), [0, 255, 0, 255]),
        (D3DFormat::A1R5G5B5, &0x801Fu16.to_le_bytes(), [0, 0, 255, 255]),
        (D3DFormat::A4R4G4B4, &0x8F00u16.to_le_bytes(), [255, 0, 0, 136]),
        (D3DFormat::X4R4G4B4, &0x80F0u16.to_le_bytes(), [0, 255, 0, 255]),
        (D3DFormat::A8R3G3B2, &0x80E0u16.to_le_bytes(), [255, 0, 0, 128]),
        (D3DFormat::L8, &[77], [77, 77, 77, 255]),
        (D3DFormat::A8L8, &[77, 200], [77, 77, 77, 200]),
        (D3DFormat::A4L4, &[0x8F], [255, 255, 255, 136]),
        (D3DFormat::A8, &[200], [0, 0, 0, 200]),
        (D3DFormat::L16, &0x8080u16.to_le_bytes(), [128, 128, 128, 255]),
        (D3DFormat::G16R16, &[0xFF, 0xFF, 0x00, 0x80], [255, 128, 0, 255]),
        (D3DFormat::A2B10G10R10, &0xC000_03FFu32.to_le_bytes(), [255, 0, 0, 255]),
        (D3DFormat::A2R10G10B10, &0x4000_03FFu32.to_le_bytes(), [0, 0, 255, 85]),
        (D3DFormat::A16B16G16R16, &[0xFF, 0xFF, 0, 0, 0x80, 0x80, 0xFF, 0xFF], [255, 0, 128, 255]),
    ];
    for (format, data, expected) in cases {
        let mut bytes = dds(format, data, 1, 1);
        // `ddsfile` writes every bit-mask format as `DDPF_RGB`; real writers (D3DX, `DirectXTex`,
        // NVTT) flag luminance and alpha-only layouts as such.
        let flags = match format {
            D3DFormat::L8 | D3DFormat::L16 => Some(DDPF_LUMINANCE),
            D3DFormat::A8L8 | D3DFormat::A4L4 => Some(DDPF_LUMINANCE | DDPF_ALPHAPIXELS),
            D3DFormat::A8 => Some(DDPF_ALPHA),
            _ => None,
        };
        if let Some(flags) = flags {
            bytes[80..84].copy_from_slice(&flags.to_le_bytes());
        }
        // ... and 64-bit `A16B16G16R16` as its D3D9 code in `FourCC` (`DirectXTex` `DDSPF_A16B16G16R16`).
        if format == D3DFormat::A16B16G16R16 {
            bytes[80..84].copy_from_slice(&DDPF_FOURCC.to_le_bytes());
            bytes[84..88].copy_from_slice(&D3DFMT_A16B16G16R16.to_le_bytes());
        }
        assert_eq!(texel("t.dds", &bytes), expected, "{format:?}");
    }
}

#[test]
fn decodes_palettized_p8_dds() {
    let mut bytes = dds(D3DFormat::L8, &[], 2, 1);
    bytes.truncate(DDS_DATA);
    bytes[80..84].copy_from_slice(&DDPF_PALETTEINDEXED8.to_le_bytes());
    bytes[92..108].fill(0);
    let mut palette = [0u8; PALETTE_LEN];
    palette[4..8].copy_from_slice(&[10, 20, 30, 255]);
    palette[1020..].copy_from_slice(&[1, 2, 3, 0]);
    bytes.extend_from_slice(&palette);
    bytes.extend_from_slice(&[255, 1]);
    let img = decode_rgba("p8.dds", &bytes).unwrap();
    assert_eq!(img.rgba, [1, 2, 3, 0, 10, 20, 30, 255]);
}

#[test]
fn decodes_dx10_uncompressed_formats() {
    let cases: [(DxgiFormat, &[u8], [u8; 4]); 13] = [
        (DxgiFormat::R8G8B8A8_UNorm, &[10, 20, 30, 40], [10, 20, 30, 40]),
        (DxgiFormat::R8G8B8A8_UNorm_sRGB, &[10, 20, 30, 40], [10, 20, 30, 40]),
        (DxgiFormat::B8G8R8A8_UNorm, &[30, 20, 10, 40], [10, 20, 30, 40]),
        (DxgiFormat::B8G8R8X8_UNorm, &[30, 20, 10, 40], [10, 20, 30, 255]),
        (DxgiFormat::R8G8_UNorm, &[10, 20], [10, 20, 0, 255]),
        (DxgiFormat::R8_UNorm, &[10], [10, 0, 0, 255]),
        (DxgiFormat::A8_UNorm, &[10], [0, 0, 0, 10]),
        (DxgiFormat::R16G16B16A16_UNorm, &[0xFF, 0xFF, 0, 0, 0x80, 0x80, 0, 0], [255, 0, 128, 0]),
        (DxgiFormat::R16G16_UNorm, &[0, 0, 0xFF, 0xFF], [0, 255, 0, 255]),
        (DxgiFormat::R16_UNorm, &[0x80, 0x80], [128, 0, 0, 255]),
        (DxgiFormat::R10G10B10A2_UNorm, &0xC000_03FFu32.to_le_bytes(), [255, 0, 0, 255]),
        (DxgiFormat::B5G6R5_UNorm, &0x07E0u16.to_le_bytes(), [0, 255, 0, 255]),
        (DxgiFormat::B5G5R5A1_UNorm, &0x7C00u16.to_le_bytes(), [255, 0, 0, 0]),
    ];
    for (format, data, expected) in cases {
        assert_eq!(texel("t.dds", &dx10(format, data, 1, 1)), expected, "{format:?}");
    }
}

#[test]
fn decodes_ktx2_uncompressed_formats() {
    let cases: [(u32, &[u8], [u8; 4]); 7] = [
        (37, &[10, 20, 30, 40], [10, 20, 30, 40]),                  // R8G8B8A8_UNORM
        (50, &[30, 20, 10, 40], [10, 20, 30, 40]),                  // B8G8R8A8_SRGB
        (23, &[10, 20, 30], [10, 20, 30, 255]),                     // R8G8B8_UNORM
        (16, &[10, 20], [10, 20, 0, 255]),                          // R8G8_UNORM
        (9, &[10], [10, 0, 0, 255]),                                // R8_UNORM
        (4, &0xF800u16.to_le_bytes(), [255, 0, 0, 255]),            // R5G6B5_UNORM_PACK16
        (3, &0xF00Fu16.to_le_bytes(), [0, 0, 255, 255]),            // B4G4R4A4_UNORM_PACK16
    ];
    for (vk, data, expected) in cases {
        assert_eq!(texel("t.ktx2", &ktx2(vk, 1, 1, data)), expected, "vkFormat {vk}");
    }
}

/// An 8x8 gradient with a varying alpha.
fn gradient() -> Vec<u8> {
    (0..64u8).flat_map(|i| [i * 4, 255 - i * 4, (i % 8) * 32, 128 + i]).collect()
}

/// BC1-BC5 and BC7 decode the same from a legacy DDS, a DX10 DDS and a KTX2, and equal the
/// framework's (and, for BC1-BC3, the old `texpresso`) decode of the blocks.
#[test]
fn decodes_block_compressed_in_every_container() {
    let rgba = gradient();
    for (format, dxgi, legacy) in [
        (BlockFormat::Bc1, DxgiFormat::BC1_UNorm, Some((D3DFormat::DXT1, None))),
        (BlockFormat::Bc2, DxgiFormat::BC2_UNorm, Some((D3DFormat::DXT3, None))),
        (BlockFormat::Bc3, DxgiFormat::BC3_UNorm_sRGB, Some((D3DFormat::DXT5, None))),
        (BlockFormat::Bc4, DxgiFormat::BC4_UNorm, Some((D3DFormat::DXT1, Some(*b"ATI1")))),
        (BlockFormat::Bc5, DxgiFormat::BC5_UNorm, Some((D3DFormat::DXT5, Some(*b"ATI2")))),
        (BlockFormat::Bc7, DxgiFormat::BC7_UNorm_sRGB, None),
    ] {
        let blocks = encode_level(format, &rgba, 8, 8, Bc7Quality::Fast);
        let expected = decode_level(format, &blocks, 8, 8);
        let squish = match format {
            BlockFormat::Bc1 => Some(texpresso::Format::Bc1),
            BlockFormat::Bc2 => Some(texpresso::Format::Bc2),
            BlockFormat::Bc3 => Some(texpresso::Format::Bc3),
            _ => None,
        };
        if let Some(squish) = squish {
            let mut old = vec![0; 256];
            squish.decompress(&blocks, 8, 8, &mut old);
            assert_eq!(old, expected, "{format:?}");
        }
        let mut files = vec![("dx10.dds", dx10(dxgi, &blocks, 8, 8)), ("t.ktx2", write_ktx2(format, 8, 8, &[&blocks]))];
        if let Some((d3d, fourcc)) = legacy {
            let mut bytes = dds(d3d, &blocks, 8, 8);
            if let Some(code) = fourcc {
                bytes[84..88].copy_from_slice(&code);
            }
            files.push(("legacy.dds", bytes));
        }
        for (name, bytes) in files {
            let img = decode_rgba(name, &bytes).unwrap_or_else(|e| panic!("{format:?} {name}: {e}"));
            assert_eq!((img.width, img.height), (8, 8), "{format:?} {name}");
            assert_eq!(img.rgba, expected, "{format:?} {name}");
        }
    }
}

#[test]
fn decodes_bc1_solid_block() {
    // One 4×4 block: color0 = color1 = pure red (RGB565 0xF800), all indices 0.
    let block = [0x00, 0xF8, 0x00, 0xF8, 0, 0, 0, 0];
    let bytes = dds(D3DFormat::DXT1, &block, 4, 4);
    let img = decode_rgba("x.DDS", &bytes).unwrap();
    assert_eq!(img.rgba.len(), 64);
    assert!(img.rgba.chunks(4).all(|p| p == [255, 0, 0, 255]), "{:?}", &img.rgba[..4]);
}

#[test]
fn decodes_unaligned_block_compressed_sizes() {
    // 6x6 BC1: 2x2 blocks, all solid red; the padding is cropped.
    let blocks = [0x00, 0xF8, 0x00, 0xF8, 0, 0, 0, 0].repeat(4);
    let img = decode_rgba("odd.dds", &dx10(DxgiFormat::BC1_UNorm, &blocks, 6, 6)).unwrap();
    assert_eq!((img.width, img.height, img.rgba.len()), (6, 6, 6 * 6 * 4));
    assert!(img.rgba.chunks(4).all(|p| p == [255, 0, 0, 255]));
}

#[test]
fn the_bytes_name_the_container_not_the_extension() {
    assert_eq!(texel("renamed.dds", &ktx2(37, 1, 1, &[1, 2, 3, 4])), [1, 2, 3, 4]);
    assert_eq!(texel("renamed.tga", &dds(D3DFormat::A8B8G8R8, &[1, 2, 3, 4], 1, 1)), [1, 2, 3, 4]);
    let mut png = Vec::new();
    image::RgbaImage::from_pixel(1, 1, image::Rgba([5, 6, 7, 8]))
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .unwrap();
    assert_eq!(texel("renamed.tga", &png), [5, 6, 7, 8]);
}

#[test]
fn decodes_tga_rle_greyscale_and_8bit_bmp() {
    // RLE TGA (`TgaEncoder` compresses by default): runs and literals.
    let rgba: Vec<u8> = [[9, 8, 7, 255]; 5].into_iter().chain([[1, 2, 3, 4], [5, 6, 7, 8], [9, 9, 9, 9]]).flatten().collect();
    let mut tga = Vec::new();
    image::codecs::tga::TgaEncoder::new(&mut tga).encode(&rgba, 4, 2, ExtendedColorType::Rgba8).unwrap();
    assert_ne!(tga[2], 2, "an RLE image type");
    let img = decode_rgba("t.tga", &tga).unwrap();
    assert_eq!((img.width, img.height, img.rgba), (4, 2, rgba));
    // Greyscale TGA.
    let mut grey = Vec::new();
    image::codecs::tga::TgaEncoder::new(&mut grey).encode(&[50, 150], 2, 1, ExtendedColorType::L8).unwrap();
    assert_eq!(decode_rgba("g.tga", &grey).unwrap().rgba, [50, 50, 50, 255, 150, 150, 150, 255]);
    // 8-bit palettized BMP.
    let palette: Vec<[u8; 3]> = (0..=255u8).map(|i| [i, 255 - i, 7]).collect();
    let mut bmp = Vec::new();
    image::codecs::bmp::BmpEncoder::new(&mut bmp)
        .encode_with_palette(&[3, 200], 2, 1, ExtendedColorType::L8, Some(&palette))
        .unwrap();
    assert_eq!(bmp[28], 8, "8 bits a pixel");
    assert_eq!(decode_rgba("p.bmp", &bmp).unwrap().rgba, [3, 252, 7, 255, 200, 55, 7, 255]);
}

#[test]
fn rejects_what_it_does_not_read() {
    assert!(decode_rgba("a.xyz", &[]).is_err());
    assert!(decode_rgba("hdr.dds", &dx10(DxgiFormat::BC6H_UF16, &[0; 16], 4, 4)).is_err());
    assert!(decode_rgba("float.dds", &dx10(DxgiFormat::R16_Float, &[0; 2], 1, 1)).is_err());
    assert!(decode_rgba("sky.dds", &dds_caps(D3DFormat::A8R8G8B8, &[], 1, 1, Some(Caps2::CUBEMAP))).is_err());
    assert!(decode_rgba("photo.jpg", &[0xFF, 0xD8, 0xFF, 0xE0, 0, 0x10, b'J', b'F', b'I', b'F', 0]).is_err());
    assert!(decode_rgba("short.dds", &dds(D3DFormat::A8R8G8B8, &[], 4, 4)[..140]).is_err(), "truncated");
}

fn alphas(values: &[u8]) -> Rgba8 {
    let rgba = values.iter().flat_map(|&a| [0, 0, 0, a]).collect();
    Rgba8 { width: u32::try_from(values.len()).unwrap(), height: 1, rgba }
}

#[test]
fn leaf_alpha_is_a_cutout_glass_is_not() {
    // Leaf card: holes, solid texels, a thin filtered rim.
    let leaf: Vec<u8> = [0; 40].into_iter().chain([255; 50]).chain([128; 10]).collect();
    assert!(alpha_is_cutout(&alphas(&leaf)));
    // Glass / potion: broad partial alpha.
    let glass: Vec<u8> = [0; 10].into_iter().chain([255; 20]).chain([100; 70]).collect();
    assert!(!alpha_is_cutout(&alphas(&glass)));
    // No holes: nothing to cut.
    assert!(!alpha_is_cutout(&alphas(&[255; 16])));
}

/// Histograms (holes, solid, soft %) measured on the vanilla textures
/// with `examples/foliage_report` (LM-1).
fn histogram(holes: usize, solid: usize, soft: usize) -> Rgba8 {
    let v: Vec<u8> = [0].repeat(holes).into_iter().chain([255].repeat(solid)).chain([128].repeat(soft)).collect();
    alphas(&v)
}

#[test]
fn twig_atlases_are_cutouts_decals_and_crusts_are_not() {
    for (name, h, s, f) in [("bc_leaves_02", 45, 32, 23), ("branches_01", 85, 10, 5), ("bc_moss_01", 53, 23, 24)] {
        assert!(alpha_is_cutout(&histogram(h, s, f)), "{name}");
    }
    for (name, h, s, f) in [
        ("dagoth_chalk00", 71, 2, 27),
        ("bc_moss_03", 35, 5, 61),
        ("lava_crust", 5, 48, 46),
        ("bc_podplant_03", 5, 40, 54),
        ("waterfall_02", 13, 0, 87),
    ] {
        assert!(!alpha_is_cutout(&histogram(h, s, f)), "{name}");
    }
}
