//! Hillshade previews: one directional light, true vertical scale, 8-bit gray PNG, and contact sheets.
//!
//! Gradient by Horn's 3 × 3 method (Horn, "Hill shading and the reflectance map", Proc. IEEE 69(1), 1981), the
//! one GIS hillshade tools use; shade = max(0, n · l) with l from an azimuth (clockwise from north) and an
//! altitude. Wet samples are drawn at one flat gray so every arm shows the same coastline treatment.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use rayon::prelude::*;

use crate::{Error, Grid, Result};

/// An 8-bit gray image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gray {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

/// Light and sea treatment of a hillshade.
#[derive(Debug, Clone, Copy)]
pub struct Light {
    /// Degrees clockwise from north.
    pub azimuth_deg: f64,
    /// Degrees above the horizon.
    pub altitude_deg: f64,
    /// Gray level of wet samples.
    pub sea_gray: u8,
}

/// Hillshade of `g` (heights in metres at `g.spacing`, true vertical scale).
pub fn hillshade(g: &Grid, sea_level: f64, light: Light) -> Gray {
    let (az, alt) = (
        light.azimuth_deg.to_radians(),
        light.altitude_deg.to_radians(),
    );
    let l = [az.sin() * alt.cos(), az.cos() * alt.cos(), alt.sin()];
    let mut px = vec![0u8; g.w * g.h];
    px.par_chunks_mut(g.w).enumerate().for_each(|(y, row)| {
        let y = y as isize;
        for (x, p) in row.iter_mut().enumerate() {
            let xi = x as isize;
            if g.at(x, y.cast_unsigned()) < sea_level {
                *p = light.sea_gray;
                continue;
            }
            let s = |dx: isize, dy: isize| g.at_clamped(xi + dx, y + dy);
            // Rows grow southward: north row is dy = -1.
            let de = ((s(1, -1) + 2.0 * s(1, 0) + s(1, 1))
                - (s(-1, -1) + 2.0 * s(-1, 0) + s(-1, 1)))
                / (8.0 * g.spacing);
            let dn = ((s(-1, -1) + 2.0 * s(0, -1) + s(1, -1))
                - (s(-1, 1) + 2.0 * s(0, 1) + s(1, 1)))
                / (8.0 * g.spacing);
            let len = (de * de + dn * dn + 1.0).sqrt();
            let shade = ((-de * l[0] - dn * l[1] + l[2]) / len).max(0.0);
            *p = (shade * 255.0).round().clamp(0.0, 255.0) as u8;
        }
    });
    Gray { w: g.w, h: g.h, px }
}

/// Box-filter downsample by an integer factor (partial blocks at the right and bottom edges averaged too).
pub fn downsample(img: &Gray, factor: usize) -> Gray {
    if factor <= 1 {
        return img.clone();
    }
    let (w, h) = (img.w.div_ceil(factor), img.h.div_ceil(factor));
    let mut px = vec![0u8; w * h];
    px.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, p) in row.iter_mut().enumerate() {
            let (mut sum, mut cnt) = (0u32, 0u32);
            for sy in y * factor..((y + 1) * factor).min(img.h) {
                for sx in x * factor..((x + 1) * factor).min(img.w) {
                    sum += u32::from(img.px[sy * img.w + sx]);
                    cnt += 1;
                }
            }
            *p = ((sum + cnt / 2) / cnt) as u8;
        }
    });
    Gray { w, h, px }
}

/// Images side by side, left to right, `gap` px of `gap_gray` between them; heights padded to the tallest.
pub fn contact_sheet(imgs: &[&Gray], gap: usize, gap_gray: u8) -> Gray {
    let h = imgs.iter().map(|i| i.h).max().unwrap_or(0);
    let w = imgs.iter().map(|i| i.w).sum::<usize>() + gap * imgs.len().saturating_sub(1);
    let mut px = vec![gap_gray; w * h];
    let mut x0 = 0;
    for img in imgs {
        for y in 0..img.h {
            px[y * w + x0..y * w + x0 + img.w].copy_from_slice(&img.px[y * img.w..(y + 1) * img.w]);
        }
        x0 += img.w + gap;
    }
    Gray { w, h, px }
}

/// Writes an 8-bit gray PNG.
pub fn write_png(path: &Path, img: &Gray) -> Result<()> {
    let w = u32::try_from(img.w).map_err(|_| Error::Invalid("image too wide".into()))?;
    let h = u32::try_from(img.h).map_err(|_| Error::Invalid("image too tall".into()))?;
    let mut enc = png::Encoder::new(BufWriter::new(File::create(path)?), w, h);
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(&img.px)?;
    Ok(())
}
