//! The owner's overlay: a hillshade of the analysed field with every feature drawn in its kind's colour
//! (outline or polyline, a cross for points) and its number, as an RGB PNG.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::shade::{Light, downsample, hillshade};
use crate::{Error, Result};

use super::{Feature, Field};

/// Colour per kind (also the README legend).
pub fn colour(kind: &str) -> [u8; 3] {
    match kind {
        "crater" => [255, 40, 40],
        "basin" => [255, 140, 0],
        "pool" => [255, 210, 120],
        "lake" => [0, 120, 255],
        "lagoon" => [0, 200, 255],
        "trench" => [200, 0, 255],
        "gorge" => [255, 0, 150],
        "valley" => [40, 200, 40],
        "peak" => [255, 255, 0],
        "plateau" => [160, 100, 40],
        "bay" => [0, 255, 200],
        "headland" => [255, 255, 255],
        _ => [120, 255, 120],
    }
}

struct Img {
    w: usize,
    h: usize,
    px: Vec<u8>,
}

impl Img {
    fn put(&mut self, x: i64, y: i64, c: [u8; 3]) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            let o = (y as usize * self.w + x as usize) * 3;
            self.px[o..o + 3].copy_from_slice(&c);
        }
    }

    fn line(&mut self, a: (f64, f64), b: (f64, f64), c: [u8; 3]) {
        let n = (b.0 - a.0).abs().max((b.1 - a.1).abs()).ceil().max(1.0) as usize;
        for k in 0..=n {
            let t = k as f64 / n as f64;
            self.put(
                (a.0 + (b.0 - a.0) * t).round() as i64,
                (a.1 + (b.1 - a.1) * t).round() as i64,
                c,
            );
        }
    }

    // 3 x 5 digits, scale 2, with a black shadow.
    fn number(&mut self, x: i64, y: i64, v: usize, c: [u8; 3]) {
        // tunables: const (format: a 3 x 5 bitmap font)
        const DIGITS: [u16; 10] = [
            0b111_101_101_101_111,
            0b010_110_010_010_111,
            0b111_001_111_100_111,
            0b111_001_111_001_111,
            0b101_101_111_001_001,
            0b111_100_111_001_111,
            0b111_100_111_101_111,
            0b111_001_010_010_010,
            0b111_101_111_101_111,
            0b111_101_111_001_111,
        ];
        let s = v.to_string();
        for (ci, ch) in s.bytes().enumerate() {
            let g = DIGITS[(ch - b'0') as usize];
            for row in 0..5 {
                for col in 0..3 {
                    if g >> (14 - (row * 3 + col)) & 1 == 1 {
                        let (px, py) = (x + (ci as i64 * 4 + col) * 2, y + row * 2);
                        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                            self.put(px + dx + 1, py + dy + 1, [0, 0, 0]);
                            self.put(px + dx, py + dy, c);
                        }
                    }
                }
            }
        }
    }
}

/// Writes the overlay of `features` on `f` to `path`, at most `max_px` pixels on the long side.
pub fn write(path: &Path, f: &Field, features: &[Feature], max_px: usize) -> Result<()> {
    let light = Light {
        azimuth_deg: 315.0,
        altitude_deg: 45.0,
        sea_gray: 40,
    };
    let shade = hillshade(&f.g, f.sea, light);
    let factor = f.g.w.max(f.g.h).div_ceil(max_px.max(1)).max(1);
    let gray = downsample(&shade, factor);
    let mut img = Img {
        w: gray.w,
        h: gray.h,
        px: gray.px.iter().flat_map(|&g| [g, g, g]).collect(),
    };
    let to_px = |p: [f64; 2]| {
        let (c, r) = f.frame.to_sample(p);
        (c / factor as f64, r / factor as f64)
    };
    for ft in features {
        let c = colour(ft.kind);
        let pts = ft.outline_m.as_ref().or(ft.polyline_m.as_ref());
        if let Some(pts) = pts {
            for s in pts.windows(2) {
                img.line(to_px(s[0]), to_px(s[1]), c);
            }
            if ft.outline_m.is_some() && pts.len() > 2 {
                img.line(to_px(pts[pts.len() - 1]), to_px(pts[0]), c);
            }
        } else {
            let (x, y) = to_px(ft.centroid_m);
            for d in -3..=3 {
                img.put(x.round() as i64 + d, y.round() as i64, c);
                img.put(x.round() as i64, y.round() as i64 + d, c);
            }
        }
        let (x, y) = to_px(ft.centroid_m);
        let num = ft
            .id
            .rsplit('-')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        img.number(x.round() as i64 + 4, y.round() as i64 - 5, num, c);
    }
    let w = u32::try_from(img.w).map_err(|_| Error::Invalid("image too wide".into()))?;
    let h = u32::try_from(img.h).map_err(|_| Error::Invalid("image too tall".into()))?;
    let mut enc = png::Encoder::new(BufWriter::new(File::create(path)?), w, h);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(&img.px)?;
    Ok(())
}
