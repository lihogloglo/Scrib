//! The bench's two meshes and its one texture, all procedural and deterministic: a 12 m tree (a tapered 8-sided
//! trunk plus about 700 alpha-masked leaf quads in an ellipsoid crown), a single upright card standing in for the
//! impostor tier, and the RGBA mask both sample.
//!
//! Mask layout: texel columns `0..OPAQUE_COLUMNS` are a fully opaque bark patch (the trunk samples it), the rest is
//! a leaf pattern of opaque blobs on transparent texels at about half coverage (leaves and the card sample it).

use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Fixture: tree height in metres.
pub const TREE_HEIGHT_M: f32 = 12.0;
/// Fixture: crown width in metres (the crown spans x and z in `[-4, 4]`).
pub const CROWN_WIDTH_M: f32 = 8.0;
/// Fixture: height of the crown's base in metres.
const CROWN_BASE_M: f32 = 4.0;
/// Fixture: sides of the trunk.
const TRUNK_SIDES: u32 = 8;
/// Fixture: trunk segments along its height.
const TRUNK_SEGMENTS: u32 = 4;
/// Fixture: trunk top in metres (inside the crown).
const TRUNK_TOP_M: f32 = 10.5;
/// Fixture: trunk radius at the base and at the top, metres.
const TRUNK_RADIUS_M: [f32; 2] = [0.35, 0.08];
/// Fixture: leaf quads in the crown.
const LEAF_COUNT: u32 = 700;
/// Fixture: leaf quad edge in metres.
const LEAF_SIZE_M: f32 = 1.2;
/// Fixture: seed of the leaf placement stream.
const LEAF_SEED: u64 = 0x5eed_1eaf;
/// Fixture: card width and height, metres (the tree's crown width and height).
const CARD_SIZE_M: [f32; 2] = [CROWN_WIDTH_M, TREE_HEIGHT_M];

/// Fixture: mask edge in texels.
pub const MASK_SIZE: u32 = 256;
/// Fixture: texel columns of the opaque bark patch, from the left edge.
pub const OPAQUE_COLUMNS: u32 = 32;
/// Fixture: seed of the leaf blob stream.
const MASK_SEED: u64 = 0xb10b_5eed;
/// Fixture: leaf blob radius range, texels.
const BLOB_RADIUS: [f32; 2] = [5.0, 12.0];
/// Fixture: the leaf patch stops adding blobs once this share of it is opaque.
const MASK_COVERAGE: f32 = 0.5;
/// Fixture: bark and leaf colours (sRGB).
const BARK_RGB: [u8; 3] = [92, 66, 46];
const LEAF_RGB: [u8; 3] = [58, 104, 40];

/// UV `u` range of the opaque patch, inset by 0.01 (2.5 texels) so filtering never reaches the leaf texels.
fn opaque_u() -> [f32; 2] {
    let edge = OPAQUE_COLUMNS as f32 / MASK_SIZE as f32;
    [0.01, edge - 0.01]
}

/// UV `u` range of the leaf patch.
fn leaf_u() -> [f32; 2] {
    let edge = OPAQUE_COLUMNS as f32 / MASK_SIZE as f32;
    [edge + 0.01, 1.0]
}

/// A tiny `SplitMix64` stream: deterministic, no `rand`.
struct Hash(u64);

impl Hash {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)` from the top 24 bits.
    fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u32 << 24) as f32
    }

    /// Uniform in `[lo, hi)`.
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.unit() * (hi - lo)
    }
}

/// A mesh from triangle-list attributes.
fn build(positions: Vec<[f32; 3]>, normals: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>, indices: Vec<u32>) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

/// The tree: base at the origin, 12 m tall, crown 8 m wide from 4 m to 12 m, about 1,460 triangles.
pub fn tree_mesh() -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    // Trunk: rings of TRUNK_SIDES + 1 vertices (the last repeats the first for the UV seam).
    let [u0, u1] = opaque_u();
    for k in 0..=TRUNK_SEGMENTS {
        let t = k as f32 / TRUNK_SEGMENTS as f32;
        let y = t * TRUNK_TOP_M;
        let r = TRUNK_RADIUS_M[0] + t * (TRUNK_RADIUS_M[1] - TRUNK_RADIUS_M[0]);
        for j in 0..=TRUNK_SIDES {
            let s = j as f32 / TRUNK_SIDES as f32;
            let (sin, cos) = (s * TAU).sin_cos();
            positions.push([r * cos, y, r * sin]);
            normals.push([cos, 0.0, sin]);
            uvs.push([u0 + s * (u1 - u0), 1.0 - t]);
        }
    }
    let ring_len = TRUNK_SIDES + 1;
    for k in 0..TRUNK_SEGMENTS {
        for j in 0..TRUNK_SIDES {
            let a = k * ring_len + j;
            let (b, c) = (a + 1, a + ring_len);
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    // Leaves: quad centres inside the crown ellipsoid shrunk by a margin, random orientation and UV window.
    let half = LEAF_SIZE_M * 0.5;
    let radius = CROWN_WIDTH_M * 0.5 - half;
    let half_height = (TREE_HEIGHT_M - CROWN_BASE_M) * 0.5 - half;
    let centre_y = f32::midpoint(TREE_HEIGHT_M, CROWN_BASE_M);
    let [lu0, lu1] = leaf_u();
    let window = 0.25;
    let mut rng = Hash(LEAF_SEED);
    let leaf_start = positions.len();
    let mut leaf_quads = Vec::with_capacity(LEAF_COUNT as usize);
    for _ in 0..LEAF_COUNT {
        // Rejection-sample the unit ball.
        let p = loop {
            let p = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            if p.length_squared() <= 1.0 {
                break p;
            }
        };
        let centre = Vec3::new(p.x * radius, centre_y + p.y * half_height, p.z * radius);
        let rot = Quat::from_euler(EulerRot::YXZ, rng.range(0.0, TAU), rng.range(-1.2, 1.2), rng.range(0.0, TAU));
        let su = rng.range(lu0, lu1 - window);
        let sv = rng.range(0.0, 1.0 - window);
        let corners = [[-half, -half], [half, -half], [half, half], [-half, half]];
        let corner_uv = [[su, sv + window], [su + window, sv + window], [su + window, sv], [su, sv]];
        let base = u32::try_from(positions.len()).unwrap_or(u32::MAX);
        for (c, uv) in corners.iter().zip(corner_uv) {
            positions.push((centre + rot * Vec3::new(c[0], c[1], 0.0)).to_array());
            normals.push([0.0, 0.0, 1.0]);
            uvs.push(uv);
        }
        leaf_quads.push(base);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    // Stretch the leaves so the crown spans exactly [-4, 4] in x and z and [CROWN_BASE_M, TREE_HEIGHT_M] in y, then
    // take each quad's normal from its stretched corners.
    let leaves = &mut positions[leaf_start..];
    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in leaves.iter() {
        lo = lo.min(Vec3::from_array(*p));
        hi = hi.max(Vec3::from_array(*p));
    }
    let target_lo = Vec3::new(-CROWN_WIDTH_M * 0.5, CROWN_BASE_M, -CROWN_WIDTH_M * 0.5);
    let target_hi = Vec3::new(CROWN_WIDTH_M * 0.5, TREE_HEIGHT_M, CROWN_WIDTH_M * 0.5);
    let scale = (target_hi - target_lo) / (hi - lo);
    for p in leaves.iter_mut() {
        *p = (target_lo + (Vec3::from_array(*p) - lo) * scale).to_array();
    }
    for base in leaf_quads {
        let i = base as usize;
        let [p0, p1, p2] = [positions[i], positions[i + 1], positions[i + 2]].map(Vec3::from_array);
        let n = (p1 - p0).cross(p2 - p0).normalize_or(Vec3::Y).to_array();
        normals[i..i + 4].fill(n);
    }

    build(positions, normals, uvs, indices)
}

/// The card: one upright quad, 8 m wide and 12 m tall, base centred at the origin, facing +Z, 2 triangles.
pub fn card_mesh() -> Mesh {
    let [w, h] = CARD_SIZE_M;
    let positions = vec![[-w * 0.5, 0.0, 0.0], [w * 0.5, 0.0, 0.0], [w * 0.5, h, 0.0], [-w * 0.5, h, 0.0]];
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let [u0, u1] = leaf_u();
    let uvs = vec![[u0, 1.0], [u1, 1.0], [u1, 0.0], [u0, 0.0]];
    build(positions, normals, uvs, vec![0, 1, 2, 0, 2, 3])
}

/// The RGBA8 sRGB mask: an opaque bark patch on the left, a blob leaf pattern at about 50 % coverage elsewhere.
pub fn leaf_mask_image() -> Image {
    let size = MASK_SIZE as usize;
    let opaque = OPAQUE_COLUMNS as usize;
    let mut data = vec![0u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let px = &mut data[(y * size + x) * 4..][..4];
            if x < opaque {
                px.copy_from_slice(&[BARK_RGB[0], BARK_RGB[1], BARK_RGB[2], 255]);
            } else {
                // Transparent texels keep the leaf colour, so filtering never darkens the edges.
                px.copy_from_slice(&[LEAF_RGB[0], LEAF_RGB[1], LEAF_RGB[2], 0]);
            }
        }
    }
    // Blobs: discs at random centres in the leaf patch until it is MASK_COVERAGE opaque.
    let patch_texels = (size - opaque) * size;
    let mut covered = 0usize;
    let mut rng = Hash(MASK_SEED);
    while (covered as f32) < MASK_COVERAGE * patch_texels as f32 {
        let cx = rng.range(opaque as f32, size as f32);
        let cy = rng.range(0.0, size as f32);
        let r = rng.range(BLOB_RADIUS[0], BLOB_RADIUS[1]);
        let x0 = ((cx - r).floor().max(opaque as f32)) as usize;
        let x1 = ((cx + r).ceil().min(size as f32 - 1.0)) as usize;
        let y0 = ((cy - r).floor().max(0.0)) as usize;
        let y1 = ((cy + r).ceil().min(size as f32 - 1.0)) as usize;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
                let alpha = &mut data[(y * size + x) * 4 + 3];
                if dx * dx + dy * dy <= r * r && *alpha == 0 {
                    *alpha = 255;
                    covered += 1;
                }
            }
        }
    }
    Image::new(
        Extent3d { width: MASK_SIZE, height: MASK_SIZE, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangles(mesh: &Mesh) -> usize {
        mesh.indices().map_or(0, |i| i.len() / 3)
    }

    fn positions(mesh: &Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|a| a.as_float3()).map(<[_]>::to_vec).unwrap_or_default()
    }

    fn aabb(mesh: &Mesh) -> (Vec3, Vec3) {
        positions(mesh).iter().fold((Vec3::splat(f32::MAX), Vec3::splat(f32::MIN)), |(lo, hi), p| {
            (lo.min(Vec3::from_array(*p)), hi.max(Vec3::from_array(*p)))
        })
    }

    fn indices_in_range(mesh: &Mesh) -> bool {
        let n = positions(mesh).len();
        mesh.indices().is_some_and(|i| i.iter().all(|i| i < n))
    }

    #[test]
    fn tree_counts_and_bounds() {
        let tree = tree_mesh();
        let tris = triangles(&tree);
        assert!((1_400..=1_600).contains(&tris), "tree has {tris} triangles");
        assert!(indices_in_range(&tree));
        let (lo, hi) = aabb(&tree);
        assert!((hi.y - lo.y - TREE_HEIGHT_M).abs() < 0.03, "height {}", hi.y - lo.y);
        assert!(lo.y.abs() < 0.03, "base {}", lo.y);
        assert!((hi.x - lo.x - CROWN_WIDTH_M).abs() < 0.03, "width x {}", hi.x - lo.x);
        assert!((hi.z - lo.z - CROWN_WIDTH_M).abs() < 0.03, "width z {}", hi.z - lo.z);
    }

    #[test]
    fn tree_is_deterministic() {
        assert_eq!(positions(&tree_mesh()), positions(&tree_mesh()));
    }

    #[test]
    fn card_counts_and_bounds() {
        let card = card_mesh();
        assert_eq!(triangles(&card), 2);
        assert!(indices_in_range(&card));
        let (lo, hi) = aabb(&card);
        assert_eq!((lo, hi), (Vec3::new(-4.0, 0.0, 0.0), Vec3::new(4.0, 12.0, 0.0)));
    }

    #[test]
    fn mask_patches() {
        let image = leaf_mask_image();
        let data = image.data.as_deref().unwrap_or_default();
        let size = MASK_SIZE as usize;
        assert_eq!(data.len(), size * size * 4);
        let opaque = OPAQUE_COLUMNS as usize;
        let alpha = |x: usize, y: usize| data[(y * size + x) * 4 + 3];
        assert!((0..size).all(|y| (0..opaque).all(|x| alpha(x, y) == 255)), "the bark patch is not opaque");
        let mut on = 0usize;
        for y in 0..size {
            for x in opaque..size {
                let a = alpha(x, y);
                assert!(a == 0 || a == 255, "alpha {a} is neither 0 nor 255");
                on += usize::from(a == 255);
            }
        }
        let coverage = on as f32 / ((size - opaque) * size) as f32;
        assert!((0.35..=0.65).contains(&coverage), "leaf coverage {coverage}");
        assert_eq!(leaf_mask_image().data, image.data, "the mask is deterministic");
    }
}
