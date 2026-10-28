//! CPU texture decoding for bake steps that resample textures (terrain
//! layer arrays): DDS (BC1/BC2/BC3 and the uncompressed formats Morrowind
//! content uses), TGA, BMP, PNG → RGBA8.
//!
//! Reuse: Bevy decodes DDS only for upload (`BCn` stays compressed), so the
//! bake can't use it. `ddsfile` (already in the tree through `bevy_image`)
//! parses the container, `texpresso` decodes `BCn` blocks, and `image` (also
//! in the tree) reads TGA/BMP/PNG.

use ddsfile::{D3DFormat, Dds, DxgiFormat};

/// A decoded image, RGBA8, rows top to bottom.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rgba8 {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Decodes the first mip of `bytes`, by the extension of `path`.
pub fn decode_rgba(path: &str, bytes: &[u8]) -> Result<Rgba8, String> {
    let ext = path.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).unwrap_or_default();
    if ext == "dds" {
        return decode_dds(bytes);
    }
    let format = match ext.as_str() {
        "tga" => image::ImageFormat::Tga,
        "bmp" => image::ImageFormat::Bmp,
        "png" => image::ImageFormat::Png,
        other => return Err(format!("unsupported texture format {other:?}")),
    };
    let img = image::load_from_memory_with_format(bytes, format).map_err(|e| e.to_string())?.into_rgba8();
    Ok(Rgba8 { width: img.width(), height: img.height(), rgba: img.into_raw() })
}

#[allow(clippy::many_single_char_names)] // width, height, channel offsets
fn decode_dds(bytes: &[u8]) -> Result<Rgba8, String> {
    let dds = Dds::read(bytes).map_err(|e| e.to_string())?;
    let (w, h) = (dds.get_width(), dds.get_height());
    let data = dds.get_data(0).map_err(|e| e.to_string())?;
    let (wu, hu) = (w as usize, h as usize);
    let mut rgba = vec![0u8; wu * hu * 4];
    let bc = match (dds.get_d3d_format(), dds.get_dxgi_format()) {
        (Some(D3DFormat::DXT1), _) | (_, Some(DxgiFormat::BC1_UNorm | DxgiFormat::BC1_UNorm_sRGB)) => {
            Some(texpresso::Format::Bc1)
        }
        (Some(D3DFormat::DXT2 | D3DFormat::DXT3), _) | (_, Some(DxgiFormat::BC2_UNorm | DxgiFormat::BC2_UNorm_sRGB)) => {
            Some(texpresso::Format::Bc2)
        }
        (Some(D3DFormat::DXT4 | D3DFormat::DXT5), _) | (_, Some(DxgiFormat::BC3_UNorm | DxgiFormat::BC3_UNorm_sRGB)) => {
            Some(texpresso::Format::Bc3)
        }
        _ => None,
    };
    if let Some(format) = bc {
        let need = format.compressed_size(wu, hu);
        if data.len() < need {
            return Err(format!("truncated DDS: {} of {need} bytes", data.len()));
        }
        format.decompress(&data[..need], wu, hu, &mut rgba);
        return Ok(Rgba8 { width: w, height: h, rgba });
    }
    // 16-bit uncompressed DDS formats used by Morrowind cutout banners/nets/clouds.
    #[allow(clippy::cast_possible_truncation, clippy::chunks_exact_to_as_chunks)]
    if let Some(d3d) = dds.get_d3d_format() {
        let expand5 = |v: u16| ((v * 527 + 23) >> 6) as u8;
        let expand6 = |v: u16| ((v * 259 + 33) >> 6) as u8;
        let expand4 = |v: u16| (v * 17) as u8;
        match d3d {
            D3DFormat::A1R5G5B5 | D3DFormat::X1R5G5B5 => {
                if data.len() < wu * hu * 2 {
                    return Err("truncated DDS".into());
                }
                let has_alpha = d3d == D3DFormat::A1R5G5B5;
                for (px, out) in data.chunks_exact(2).zip(rgba.as_chunks_mut::<4>().0) {
                    let w = u16::from_le_bytes([px[0], px[1]]);
                    let a = if !has_alpha || (w & 0x8000) != 0 { 255 } else { 0 };
                    *out = [expand5((w >> 10) & 0x1f), expand5((w >> 5) & 0x1f), expand5(w & 0x1f), a];
                }
                return Ok(Rgba8 { width: w, height: h, rgba });
            }
            D3DFormat::A4R4G4B4 => {
                if data.len() < wu * hu * 2 {
                    return Err("truncated DDS".into());
                }
                for (px, out) in data.chunks_exact(2).zip(rgba.as_chunks_mut::<4>().0) {
                    let w = u16::from_le_bytes([px[0], px[1]]);
                    *out = [expand4((w >> 8) & 0xf), expand4((w >> 4) & 0xf), expand4(w & 0xf), expand4((w >> 12) & 0xf)];
                }
                return Ok(Rgba8 { width: w, height: h, rgba });
            }
            D3DFormat::R5G6B5 => {
                if data.len() < wu * hu * 2 {
                    return Err("truncated DDS".into());
                }
                for (px, out) in data.chunks_exact(2).zip(rgba.as_chunks_mut::<4>().0) {
                    let w = u16::from_le_bytes([px[0], px[1]]);
                    *out = [expand5((w >> 11) & 0x1f), expand6((w >> 5) & 0x3f), expand5(w & 0x1f), 255];
                }
                return Ok(Rgba8 { width: w, height: h, rgba });
            }
            _ => {}
        }
    }
    // Uncompressed 24/32-bit: (bytes per pixel, byte offsets of r, g, b, a).
    let layout: (usize, [usize; 3], Option<usize>) = match dds.get_d3d_format() {
        Some(D3DFormat::A8R8G8B8) => (4, [2, 1, 0], Some(3)),
        Some(D3DFormat::X8R8G8B8) => (4, [2, 1, 0], None),
        Some(D3DFormat::A8B8G8R8) => (4, [0, 1, 2], Some(3)),
        Some(D3DFormat::X8B8G8R8) => (4, [0, 1, 2], None),
        Some(D3DFormat::R8G8B8) => (3, [2, 1, 0], None),
        other => return Err(format!("unsupported DDS format {other:?} / {:?}", dds.get_dxgi_format())),
    };
    let (bpp, [r, g, b], a) = layout;
    if data.len() < wu * hu * bpp {
        return Err("truncated DDS".into());
    }
    for (px, out) in data.chunks_exact(bpp).zip(rgba.as_chunks_mut::<4>().0) {
        out.copy_from_slice(&[px[r], px[g], px[b], a.map_or(255, |a| px[a])]);
    }
    Ok(Rgba8 { width: w, height: h, rgba })
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
mod tests {
    use super::*;
    use ddsfile::NewD3dParams;

    fn dds(format: D3DFormat, data: &[u8], w: u32, h: u32) -> Vec<u8> {
        let mut dds = Dds::new_d3d(NewD3dParams {
            height: h,
            width: w,
            depth: None,
            format,
            mipmap_levels: None,
            caps2: None,
        })
        .unwrap();
        dds.data[..data.len()].copy_from_slice(data);
        let mut out = Vec::new();
        dds.write(&mut out).unwrap();
        out
    }

    #[test]
    fn decodes_uncompressed_bgra() {
        let bytes = dds(D3DFormat::A8R8G8B8, &[10, 20, 30, 40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 2, 2);
        let img = decode_rgba("textures/a.dds", &bytes).unwrap();
        assert_eq!((img.width, img.height), (2, 2));
        assert_eq!(&img.rgba[..4], &[30, 20, 10, 40]);
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

    #[test]
    fn decodes_bc1_solid_block() {
        // One 4×4 block: color0 = color1 = pure red (RGB565 0xF800), all indices 0.
        let block = [0x00, 0xF8, 0x00, 0xF8, 0, 0, 0, 0];
        let bytes = dds(D3DFormat::DXT1, &block, 4, 4);
        let img = decode_rgba("x.DDS", &bytes).unwrap();
        assert_eq!(img.rgba.len(), 64);
        assert!(img.rgba.chunks(4).all(|p| p[0] == 255 && p[1] == 0 && p[2] == 0 && p[3] == 255), "{:?}", &img.rgba[..4]);
    }

    #[test]
    fn rejects_unknown_extensions() {
        assert!(decode_rgba("a.xyz", &[]).is_err());
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
}
