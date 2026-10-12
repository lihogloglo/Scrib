//! Source textures in the far bake: per-material average albedo (proxies
//! carry albedo in vertex colors) and texel sampling for impostor capture.
//!
//! Decoding is the adapter's job ([`TextureDecoder`]): source formats are
//! content knowledge. The bake reads texture files through its tracked
//! inputs, so editing a texture rebuilds what used it.

use glam::{Vec2, Vec4};

/// A decoded texture: sRGB-encoded RGBA8, rows top to bottom.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextureImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Adapter hook: turns texture files into pixels.
pub trait TextureDecoder: Send + Sync {
    /// Bump when decoding changes (part of every key that reads textures).
    fn version(&self) -> u32;
    /// Files to try for a material's texture path, in order (adapters map
    /// legacy extensions, e.g. a `.tga` reference shipped as `.dds`).
    fn candidates(&self, path: &str) -> Vec<String> {
        vec![path.to_owned()]
    }
    fn decode(&self, path: &str, bytes: &[u8]) -> Option<TextureImage>;
}

/// sRGB 8-bit → linear.
pub fn srgb_to_linear(c: u8) -> f32 {
    let s = f32::from(c) / 255.0;
    if s <= 0.040_45 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
}

impl TextureImage {
    /// Average linear color of the texels that pass `cutoff` (all texels for
    /// 0), and the fraction that pass (coverage) in `w`.
    #[allow(clippy::cast_precision_loss)]
    pub fn average(&self, cutoff: f32) -> Vec4 {
        let mut sum = [0f64; 3];
        let mut n = 0u64;
        let total = self.rgba.len() / 4;
        // Large textures: every k-th texel is plenty for a mean.
        let step = (total / 65_536).max(1);
        let mut seen = 0u64;
        for px in self.rgba.as_chunks::<4>().0.iter().step_by(step) {
            seen += 1;
            if f32::from(px[3]) / 255.0 < cutoff {
                continue;
            }
            for c in 0..3 {
                sum[c] += f64::from(srgb_to_linear(px[c]));
            }
            n += 1;
        }
        if n == 0 {
            return Vec4::new(0.5, 0.5, 0.5, 0.0);
        }
        #[allow(clippy::cast_possible_truncation)]
        let avg = |s: f64| (s / n as f64) as f32;
        #[allow(clippy::cast_possible_truncation)]
        let coverage = (n as f64 / seen.max(1) as f64) as f32;
        Vec4::new(avg(sum[0]), avg(sum[1]), avg(sum[2]), coverage)
    }

    /// Bilinear sample with wrapping, linear RGB and alpha.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::many_single_char_names)]
    pub fn sample(&self, uv: Vec2) -> Vec4 {
        if self.width == 0 || self.height == 0 {
            return Vec4::ONE;
        }
        let (w, h) = (self.width as f32, self.height as f32);
        let x = uv.x * w - 0.5;
        let y = uv.y * h - 0.5;
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let texel = |xi: f32, yi: f32| -> Vec4 {
            let xi = (xi as i64).rem_euclid(i64::from(self.width)) as usize;
            let yi = (yi as i64).rem_euclid(i64::from(self.height)) as usize;
            let i = (yi * self.width as usize + xi) * 4;
            let p = &self.rgba[i..i + 4];
            Vec4::new(srgb_to_linear(p[0]), srgb_to_linear(p[1]), srgb_to_linear(p[2]), f32::from(p[3]) / 255.0)
        };
        let a = texel(x0, y0).lerp(texel(x0 + 1.0, y0), fx);
        let b = texel(x0, y0 + 1.0).lerp(texel(x0 + 1.0, y0 + 1.0), fx);
        a.lerp(b, fy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_respects_cutoff_and_reports_coverage() {
        // Two opaque white texels, two transparent black ones.
        let img = TextureImage { width: 2, height: 2, rgba: vec![255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0] };
        let all = img.average(0.0);
        assert!((all.x - 0.5).abs() < 1e-3 && (all.w - 1.0).abs() < 1e-6);
        let masked = img.average(0.5);
        assert!((masked.x - 1.0).abs() < 1e-3 && (masked.w - 0.5).abs() < 1e-6);
        let s = img.sample(Vec2::new(0.25, 0.25));
        assert!((s.x - 1.0).abs() < 1e-3 && (s.w - 1.0).abs() < 1e-3);
    }
}
