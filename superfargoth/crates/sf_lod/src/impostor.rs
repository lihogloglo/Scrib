//! Octahedral impostor capture: a small orthographic CPU rasterizer renders
//! the model from every frame direction into an atlas of albedo + alpha and
//! normal + depth (far-field.md, "Impostor atlas").
//!
//! Why our own rasterizer: the capture is orthographic, needs a depth
//! buffer and affine attribute interpolation only, and must be bit-exact
//! across machines and thread counts; `euc` (the maintained-looking crate)
//! was last released in 2021 and is a general programmable pipeline. This
//! is ~150 lines and deterministic (fixed iteration order, no SIMD paths).

use glam::{Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

use crate::octa::{OctaMode, frame_basis, frame_dir};

/// Geometry to capture, in model space (meters, Y up).
#[derive(Clone, Debug, Default)]
pub struct CaptureMesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    /// Linear RGBA (vertex color × material base color, alpha already
    /// including any texture alpha the caller sampled per vertex).
    pub colors: Vec<Vec4>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
    /// Per triangle: an index into the caller's materials (for `shade`).
    pub triangle_material: Vec<u32>,
}

/// Per-pixel shading during capture: material, uv and interpolated vertex
/// color → linear RGBA. Alpha below the material's cutoff is discarded by
/// returning `None`.
pub type Shade<'a> = &'a dyn Fn(u32, Vec2, Vec4) -> Option<Vec4>;

/// Capture settings (part of the `lod.impostor` key).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImpostorSettings {
    pub mode: OctaMode,
    /// Frames per side (N×N frames).
    pub frames: u32,
    /// Texels per frame side.
    pub frame_px: u32,
    /// Supersampling per axis.
    pub supersample: u32,
}

impl Default for ImpostorSettings {
    fn default() -> Self {
        Self { mode: OctaMode::Hemi, frames: 8, frame_px: 32, supersample: 2 }
    }
}

impl ImpostorSettings {
    pub fn atlas_px(&self) -> u32 {
        self.frames * self.frame_px
    }
}

/// A captured atlas: two RGBA8 images of `atlas_px`² with a full mip chain
/// each (level 0 first, levels concatenated).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpostorAtlas {
    pub settings: ImpostorSettings,
    /// Sphere the frames were fit to, in model space.
    pub center: [f32; 3],
    pub radius: f32,
    /// sRGB-encoded albedo, alpha = coverage.
    pub albedo: Vec<u8>,
    /// Model-space normal (×0.5 + 0.5) and depth toward the viewer
    /// (0 = `radius` behind the center, 1 = `radius` in front).
    pub normal_depth: Vec<u8>,
    pub mip_levels: u32,
}

#[derive(Clone, Copy, Default)]
struct Sample {
    depth: f32,
    covered: bool,
    color: Vec4,
    normal: Vec3,
}

/// Renders the atlas. `center` and `radius` bound the model.
pub fn capture(mesh: &CaptureMesh, center: Vec3, radius: f32, settings: &ImpostorSettings, shade: Shade<'_>) -> ImpostorAtlas {
    let n = settings.frames;
    let f = settings.frame_px;
    let ss = settings.supersample.max(1);
    let s = f * ss;
    let atlas = n * f;
    let mut albedo = vec![0u8; (atlas * atlas * 4) as usize];
    let mut normal_depth = vec![0u8; (atlas * atlas * 4) as usize];
    let radius = radius.max(1e-3);
    let mut samples = vec![Sample::default(); (s * s) as usize];
    for i in 0..n {
        for j in 0..n {
            let dir = frame_dir(settings.mode, n, i, j);
            samples.fill(Sample::default());
            raster_frame(mesh, center, radius, dir, s, shade, &mut samples);
            resolve_frame(&samples, s, ss, radius, [i * f, j * f], atlas, &mut albedo, &mut normal_depth);
        }
    }
    dilate(&mut albedo, &mut normal_depth, atlas, 4);
    let (albedo, levels) = mip_chain(albedo, atlas, true);
    let (normal_depth, _) = mip_chain(normal_depth, atlas, false);
    ImpostorAtlas { settings: *settings, center: center.to_array(), radius, albedo, normal_depth, mip_levels: levels }
}

/// Orthographic raster of one frame, two-sided, nearest-to-viewer wins.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]
fn raster_frame(mesh: &CaptureMesh, center: Vec3, radius: f32, dir: Vec3, s: u32, shade: Shade<'_>, out: &mut [Sample]) {
    let (right, up, toward) = frame_basis(dir);
    let size = s as f32;
    let project = |p: Vec3| {
        let r = p - center;
        Vec3::new((r.dot(right) / radius * 0.5 + 0.5) * size, (0.5 - r.dot(up) / radius * 0.5) * size, r.dot(toward))
    };
    for (t, tri) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
        let [a, b, c] = tri.map(|k| k as usize);
        let (pa, pb, pc) = (project(mesh.positions[a]), project(mesh.positions[b]), project(mesh.positions[c]));
        let area = (pb.x - pa.x) * (pc.y - pa.y) - (pb.y - pa.y) * (pc.x - pa.x);
        if area.abs() < 1e-12 {
            continue;
        }
        let material = mesh.triangle_material.get(t).copied().unwrap_or(0);
        let x0 = pa.x.min(pb.x).min(pc.x).floor().max(0.0) as u32;
        let x1 = (pa.x.max(pb.x).max(pc.x).ceil().min(size)) as u32;
        let y0 = pa.y.min(pb.y).min(pc.y).floor().max(0.0) as u32;
        let y1 = (pa.y.max(pb.y).max(pc.y).ceil().min(size)) as u32;
        for y in y0..y1 {
            for x in x0..x1 {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                let w0 = ((pb.x - p.x) * (pc.y - p.y) - (pb.y - p.y) * (pc.x - p.x)) / area;
                let w1 = ((pc.x - p.x) * (pa.y - p.y) - (pc.y - p.y) * (pa.x - p.x)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let depth = w0 * pa.z + w1 * pb.z + w2 * pc.z;
                let sample = &mut out[(y * s + x) as usize];
                if sample.covered && depth <= sample.depth {
                    continue;
                }
                let lerp4 = |v: &[Vec4]| v[a] * w0 + v[b] * w1 + v[c] * w2;
                let color = if mesh.colors.is_empty() { Vec4::ONE } else { lerp4(&mesh.colors) };
                let uv = if mesh.uvs.is_empty() { Vec2::ZERO } else { mesh.uvs[a] * w0 + mesh.uvs[b] * w1 + mesh.uvs[c] * w2 };
                let Some(color) = shade(material, uv, color) else { continue };
                let mut normal = if mesh.normals.is_empty() {
                    (mesh.positions[b] - mesh.positions[a]).cross(mesh.positions[c] - mesh.positions[a])
                } else {
                    mesh.normals[a] * w0 + mesh.normals[b] * w1 + mesh.normals[c] * w2
                }
                .normalize_or(toward);
                // Two-sided: a back face shows its flipped normal.
                if normal.dot(toward) < 0.0 {
                    normal = -normal;
                }
                *sample = Sample { depth, covered: true, color, normal };
            }
        }
    }
}

/// Box-filters the supersampled frame into the atlas.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::many_single_char_names)]
fn resolve_frame(
    samples: &[Sample],
    s: u32,
    ss: u32,
    radius: f32,
    corner: [u32; 2],
    atlas: u32,
    albedo: &mut [u8],
    normal_depth: &mut [u8],
) {
    let f = s / ss;
    for y in 0..f {
        for x in 0..f {
            let (mut n, mut color, mut normal, mut depth) = (0u32, Vec3::ZERO, Vec3::ZERO, f32::NEG_INFINITY);
            for sy in 0..ss {
                for sx in 0..ss {
                    let smp = samples[((y * ss + sy) * s + x * ss + sx) as usize];
                    if smp.covered {
                        n += 1;
                        color += smp.color.truncate();
                        normal += smp.normal;
                        depth = depth.max(smp.depth);
                    }
                }
            }
            let at = (((corner[1] + y) * atlas + corner[0] + x) * 4) as usize;
            if n == 0 {
                continue;
            }
            let k = n as f32;
            let coverage = k / (ss * ss) as f32;
            let c = color / k;
            albedo[at..at + 4].copy_from_slice(&[srgb8(c.x), srgb8(c.y), srgb8(c.z), unorm8(coverage)]);
            let nn = normal.normalize_or(Vec3::Z) * 0.5 + 0.5;
            let d = (depth / radius * 0.5 + 0.5).clamp(0.0, 1.0);
            normal_depth[at..at + 4].copy_from_slice(&[unorm8(nn.x), unorm8(nn.y), unorm8(nn.z), unorm8(d)]);
        }
    }
}

/// Spreads color and normal into uncovered texels (alpha stays 0), so
/// bilinear filtering and mips don't pull in black at silhouettes.
#[allow(clippy::many_single_char_names, clippy::similar_names)]
fn dilate(albedo: &mut [u8], normal_depth: &mut [u8], size: u32, passes: u32) {
    let idx = |x: u32, y: u32| ((y * size + x) * 4) as usize;
    let mut filled: Vec<bool> = (0..size * size).map(|i| albedo[(i * 4 + 3) as usize] > 0).collect();
    for _ in 0..passes {
        let mut next = filled.clone();
        for y in 0..size {
            for x in 0..size {
                if filled[(y * size + x) as usize] {
                    continue;
                }
                let mut acc = [0u32; 6];
                let mut k = 0u32;
                for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x.cast_signed() + dx, y.cast_signed() + dy);
                    if nx < 0 || ny < 0 || nx >= size.cast_signed() || ny >= size.cast_signed() {
                        continue;
                    }
                    let (nx, ny) = (nx.cast_unsigned(), ny.cast_unsigned());
                    if !filled[(ny * size + nx) as usize] {
                        continue;
                    }
                    let i = idx(nx, ny);
                    for c in 0..3 {
                        acc[c] += u32::from(albedo[i + c]);
                        acc[3 + c] += u32::from(normal_depth[i + c]);
                    }
                    k += 1;
                }
                if k > 0 {
                    let i = idx(x, y);
                    let avg = |sum: u32| u8::try_from(sum.checked_div(k).unwrap_or(0)).unwrap_or(255);
                    for c in 0..3 {
                        albedo[i + c] = avg(acc[c]);
                        normal_depth[i + c] = avg(acc[3 + c]);
                    }
                    next[(y * size + x) as usize] = true;
                }
            }
        }
        filled = next;
    }
}

/// Appends box-filtered mips down to 1×1. Albedo mips weight color by
/// alpha (no dark fringes) and average alpha (coverage).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names, clippy::similar_names)]
fn mip_chain(level0: Vec<u8>, size: u32, alpha_weighted: bool) -> (Vec<u8>, u32) {
    let mut out = level0;
    let mut levels = 1;
    let mut prev_start = 0usize;
    let mut s = size;
    while s > 1 {
        let h = s / 2;
        let mut next = vec![0u8; (h * h * 4) as usize];
        for y in 0..h {
            for x in 0..h {
                let mut acc = [0f32; 4];
                let mut wsum = 0f32;
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let i = prev_start + (((y * 2 + dy) * s + x * 2 + dx) * 4) as usize;
                    let a = f32::from(out[i + 3]) / 255.0;
                    let w = if alpha_weighted { a.max(1e-3) } else { 1.0 };
                    for c in 0..3 {
                        acc[c] += f32::from(out[i + c]) * w;
                    }
                    acc[3] += f32::from(out[i + 3]);
                    wsum += w;
                }
                let o = ((y * h + x) * 4) as usize;
                for c in 0..3 {
                    next[o + c] = (acc[c] / wsum).round().clamp(0.0, 255.0) as u8;
                }
                next[o + 3] = (acc[3] / 4.0).round() as u8;
            }
        }
        prev_start = out.len();
        out.extend(next);
        s = h;
        levels += 1;
    }
    (out, levels)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn unorm8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Linear → sRGB 8-bit.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn srgb8(linear: f32) -> u8 {
    let l = linear.clamp(0.0, 1.0);
    let s = if l <= 0.003_130_8 { l * 12.92 } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 };
    (s * 255.0).round() as u8
}

impl ImpostorAtlas {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = ATLAS_MAGIC.to_vec();
        out.extend(postcard::to_stdvec(self).expect("atlases serialize"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::FormatError> {
        let body = bytes.strip_prefix(&ATLAS_MAGIC).ok_or(crate::FormatError::BadMagic("impostor atlas"))?;
        Ok(postcard::from_bytes(body)?)
    }

    /// Fraction of level-0 texels with any coverage (tests, reports).
    #[allow(clippy::cast_precision_loss)]
    pub fn coverage(&self) -> f32 {
        let px = self.settings.atlas_px() as usize;
        let n = self.albedo[..px * px * 4].as_chunks::<4>().0.iter().filter(|t| t[3] > 0).count();
        n as f32 / (px * px) as f32
    }
}

const ATLAS_MAGIC: [u8; 8] = *b"SFIMPO1\0";

#[cfg(test)]
mod tests {
    use super::*;

    /// A unit cube, red on top, blue on sides.
    fn cube() -> CaptureMesh {
        let mut m = CaptureMesh::default();
        let faces = [
            (Vec3::Y, Vec3::X, Vec3::Z),
            (Vec3::NEG_Y, Vec3::Z, Vec3::X),
            (Vec3::X, Vec3::Z, Vec3::Y),
            (Vec3::NEG_X, Vec3::Y, Vec3::Z),
            (Vec3::Z, Vec3::Y, Vec3::X),
            (Vec3::NEG_Z, Vec3::X, Vec3::Y),
        ];
        for (n, a, b) in faces {
            let base = u32::try_from(m.positions.len()).expect("small");
            let color = if n == Vec3::Y { Vec4::new(1.0, 0.0, 0.0, 1.0) } else { Vec4::new(0.0, 0.0, 1.0, 1.0) };
            for (u, v) in [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)] {
                m.positions.push(n * 0.5 + a * u + b * v + Vec3::Y * 0.5);
                m.normals.push(n);
                m.colors.push(color);
            }
            m.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
            m.triangle_material.extend([0, 0]);
        }
        m
    }

    #[allow(clippy::unnecessary_wraps)] // the `Shade` signature
    fn flat(_: u32, _: Vec2, c: Vec4) -> Option<Vec4> {
        Some(c)
    }

    #[test]
    fn captures_silhouette_and_top_color() {
        let s = ImpostorSettings { frames: 4, frame_px: 16, ..ImpostorSettings::default() };
        let center = Vec3::Y * 0.5;
        let radius = 3f32.sqrt() * 0.5;
        let atlas = capture(&cube(), center, radius, &s, &flat);
        assert_eq!(atlas.mip_levels, 7);
        let px = s.atlas_px();
        // The frame looking straight down (hemi center: uv 0.5,0.5 falls
        // between frames on a 4-grid; use frame (2,2)'s neighborhood) — check
        // instead the top-down direction's frame by decoding.
        let cov = atlas.coverage();
        assert!(cov > 0.25 && cov < 0.9, "coverage {cov}");
        // Every frame has its center texel covered (the cube fills the middle).
        for i in 0..4 {
            for j in 0..4 {
                let (x, y) = (i * 16 + 8, j * 16 + 8);
                assert!(atlas.albedo[((y * px + x) * 4 + 3) as usize] > 0, "frame {i},{j}");
            }
        }
        // A frame looking from almost straight above sees red.
        let top = crate::octa::encode(OctaMode::Hemi, Vec3::Y) * 3.0;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (i, j) = (top.x.round() as u32, top.y.round() as u32);
        let at = (((j * 16 + 8) * px + i * 16 + 8) * 4) as usize;
        let rgba = &atlas.albedo[at..at + 4];
        assert!(rgba[0] > 200 && rgba[2] < 60, "top view is red: {rgba:?}");
    }

    #[test]
    fn capture_is_deterministic() {
        let s = ImpostorSettings { frames: 4, frame_px: 8, ..ImpostorSettings::default() };
        let a = capture(&cube(), Vec3::Y * 0.5, 0.9, &s, &flat);
        let b = capture(&cube(), Vec3::Y * 0.5, 0.9, &s, &flat);
        assert_eq!(a.to_bytes(), b.to_bytes());
        assert_eq!(ImpostorAtlas::from_bytes(&a.to_bytes()).expect("decodes"), a);
    }
}
