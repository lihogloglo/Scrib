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
    // Uncompressed: (bytes per pixel, byte offsets of r, g, b, a).
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
/// Thresholds: alpha <= 0.1 is a hole, >= 0.9 solid; soft texels may be at
/// most a quarter of the covered ones (a 256² leaf atlas with filtered edges
/// sits well below that).
pub fn alpha_is_cutout(image: &Rgba8) -> bool {
    let (mut holes, mut solid, mut soft) = (0usize, 0usize, 0usize);
    for a in image.rgba.as_chunks::<4>().0.iter().map(|p| p[3]) {
        match a {
            0..=25 => holes += 1,
            230..=255 => solid += 1,
            _ => soft += 1,
        }
    }
    holes > 0 && solid > 0 && soft * 4 <= solid + soft
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
}
