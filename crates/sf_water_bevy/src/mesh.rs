//! The ocean's camera-centred geometry clipmap (water.md "Mesh"): nested square rings of
//! `quads × quads` cells, each ring double the previous spacing, plus a horizon skirt. Vertices
//! carry grid coordinates only (`x`, ring, `z`); the vertex shader (`water_common.wesl`
//! `clipmap_xz`) snaps each ring to twice its spacing around the camera and morphs its outer
//! edge onto the next ring's grid (CDLOD), so one static mesh serves every camera position.

use bevy_asset::RenderAssetUsages;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};

use crate::ClipmapSettings;

/// Half-size, in its own grid cells, of the hole a ring leaves for the finer ring inside it.
/// The finer ring covers `quads/4` of this ring's cells around its own centre, which is at most
/// one cell off this ring's centre; two cells of overlap absorb that, and the fragment shader
/// discards this ring where the finer one is (no T-junctions: the finer ring's edge is morphed
/// onto this ring's grid).
pub fn hole_half(quads: u32) -> i32 {
    i32::try_from(quads / 4).unwrap_or(0) - 2
}

/// The clipmap mesh for `settings`. `ATTRIBUTE_POSITION` = (grid x, ring, grid z); the skirt's
/// inner vertices use ring `rings`, its outer ones `rings + 1`.
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::many_single_char_names)]
pub fn clipmap(settings: &ClipmapSettings) -> Mesh {
    let quads = i32::try_from(settings.quads.max(8)).unwrap_or(128) & !3;
    let half = quads / 2;
    let hole = hole_half(quads as u32);
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let side = (quads + 1) as u32;
    for ring in 0..settings.rings {
        let base = u32::try_from(positions.len()).unwrap_or(u32::MAX);
        for z in -half..=half {
            for x in -half..=half {
                positions.push([x as f32, ring as f32, z as f32]);
            }
        }
        for z in -half..half {
            for x in -half..half {
                if ring > 0 && x >= -hole && x < hole && z >= -hole && z < hole {
                    continue;
                }
                let i = base + (z + half) as u32 * side + (x + half) as u32;
                indices.extend_from_slice(&[i, i + side, i + 1, i + 1, i + side, i + side + 1]);
            }
        }
    }
    // Horizon skirt: the last ring's edge (every second vertex, as morphed) out to `skirt`.
    let mut edge = Vec::new();
    let mut k = -half;
    while k < half {
        edge.push((k, -half));
        k += 2;
    }
    k = -half;
    while k < half {
        edge.push((half, k));
        k += 2;
    }
    k = half;
    while k > -half {
        edge.push((k, half));
        k -= 2;
    }
    k = half;
    while k > -half {
        edge.push((-half, k));
        k -= 2;
    }
    let base = u32::try_from(positions.len()).unwrap_or(u32::MAX);
    let rings = settings.rings as f32;
    for &(x, z) in &edge {
        positions.push([x as f32, rings, z as f32]);
        positions.push([x as f32, rings + 1.0, z as f32]);
    }
    let count = u32::try_from(edge.len()).unwrap_or(0);
    for i in 0..count {
        let j = (i + 1) % count;
        let (a, b, c, d) = (base + 2 * i, base + 2 * j, base + 2 * j + 1, base + 2 * i + 1);
        indices.extend_from_slice(&[a, b, c, a, c, d]);
    }
    let normals = vec![[0.0, 1.0, 0.0]; positions.len()];
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
